use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc};
use tracing::{debug, error, info};

use crate::redis::RedisPool;

// ──────────────────────────────────────────────────────────────────────────────
// messaging
//
// 消息总线：提供进程内（InMemoryBus）和跨进程（RedisPubSubBus）两种实现。
// 用于 auth_token_threshold 事件通知替代轮询、系统事件广播等场景。
// ──────────────────────────────────────────────────────────────────────────────

/// 消息总线错误
#[derive(thiserror::Error, Debug)]
pub enum MessagingError {
    #[error("publish error: {0}")]
    PublishError(String),
    #[error("subscribe error: {0}")]
    SubscribeError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("channel closed")]
    ChannelClosed,
    #[error("redis error: {0}")]
    RedisError(String),
}

pub type MessagingResult<T> = Result<T, MessagingError>;

/// 消息信封（主题 + 载荷）
#[derive(Clone, Serialize)]
pub struct Envelope<T: Serialize + Send + Sync + Clone + 'static> {
    pub topic: String,
    pub payload: T,
    pub timestamp_ms: u64,
}

impl<T: Serialize + Send + Sync + Clone + 'static> Envelope<T> {
    pub fn new(topic: impl Into<String>, payload: T) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            topic: topic.into(),
            payload,
            timestamp_ms,
        }
    }
}

impl<'de, T: Serialize + Send + Sync + Clone + 'static + serde::de::DeserializeOwned> serde::Deserialize<'de> for Envelope<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let envelope = serde_json::Value::deserialize(deserializer)?;
        let topic = envelope.get("topic").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let payload: T = serde_json::from_value(envelope.get("payload").cloned().unwrap_or_default()).map_err(serde::de::Error::custom)?;
        let timestamp_ms = envelope.get("timestamp_ms").and_then(|v| v.as_u64()).unwrap_or(0);
        Ok(Self { topic, payload, timestamp_ms })
    }
}

/// 消息总线 Trait（publish / subscribe）
#[async_trait]
pub trait MessageBus<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static>: Send + Sync {
    async fn publish(&self, topic: String, payload: M) -> MessagingResult<()>;

    async fn subscribe(
        &self,
        topic: String,
    ) -> MessagingResult<mpsc::Receiver<Envelope<M>>>;
}

// ──────────────────────────────────────────────────────────────────────────────
// InMemoryBus（单进程）
// ──────────────────────────────────────────────────────────────────────────────

/// 单进程内存消息总线
#[derive(Clone)]
pub struct InMemoryBus<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> {
    subscribers: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<Envelope<M>>>>>>,
}

impl<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> InMemoryBus<M> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> Default for InMemoryBus<M> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> MessageBus<M> for InMemoryBus<M> {
    async fn publish(&self, topic: String, payload: M) -> MessagingResult<()> {
        let envelope = Envelope::new(topic.clone(), payload);
        let subscribers = self.subscribers.read().await;
        if let Some(senders) = subscribers.get(&topic) {
            for sender in senders {
                if let Err(_) = sender.send(envelope.clone()).await {
                    debug!(topic = %topic, "failed to deliver in-memory message");
                }
            }
        }
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: String,
    ) -> MessagingResult<mpsc::Receiver<Envelope<M>>> {
        let (tx, rx) = mpsc::channel::<Envelope<M>>(1024);
        let mut subscribers = self.subscribers.write().await;
        subscribers.entry(topic).or_default().push(tx);
        Ok(rx)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// RedisPubSubBus（多进程）
// ──────────────────────────────────────────────────────────────────────────────

/// Redis Pub/Sub 消息总线（支持多进程/多实例）
#[derive(Clone)]
pub struct RedisPubSubBus<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> {
    redis_pool: Option<RedisPool>,
    _marker: std::marker::PhantomData<M>,
}

impl<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> RedisPubSubBus<M> {
    pub fn new(redis_pool: Option<RedisPool>) -> Self {
        Self {
            redis_pool,
            _marker: std::marker::PhantomData,
        }
    }

    /// Redis 是否可用
    pub fn is_available(&self) -> bool {
        self.redis_pool.is_some()
    }
}

#[async_trait]
impl<M: Serialize + Send + Sync + Clone + for<'de> Deserialize<'de> + 'static> MessageBus<M> for RedisPubSubBus<M> {
    async fn publish(&self, topic: String, payload: M) -> MessagingResult<()> {
        let pool = match &self.redis_pool {
            Some(p) => p,
            None => return Err(MessagingError::RedisError("Redis pool not initialized".into())),
        };

        let envelope = Envelope::new(topic.clone(), payload);
        let serialized = serde_json::to_string(&envelope)
            .map_err(|e| MessagingError::SerializationError(e.to_string()))?;

        let mut guard = pool.0.manager.lock().await;
        let conn = guard
            .as_mut()
            .ok_or_else(|| MessagingError::RedisError("Redis connection manager not initialized".into()))?;

        use redis::AsyncCommands;
        conn.publish::<_, _, ()>(&topic, serialized)
            .await
            .map_err(|e| MessagingError::RedisError(e.to_string()))?;

        debug!(topic = %topic, "published message via Redis Pub/Sub");
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: String,
    ) -> MessagingResult<mpsc::Receiver<Envelope<M>>> {
        let pool = match &self.redis_pool {
            Some(p) => p,
            None => return Err(MessagingError::RedisError("Redis pool not initialized".into())),
        };

        let (tx, rx) = mpsc::channel::<Envelope<M>>(1024);

        let topic_clone = topic.clone();
        let pool_clone = pool.clone();
        tokio::spawn(async move {
            let result: MessagingResult<()> = async {
                let client = redis::Client::open(pool_clone.url().to_string())
                    .map_err(|e| MessagingError::RedisError(e.to_string()))?;
                let mut pubsub_conn = client.get_async_pubsub().await
                    .map_err(|e| MessagingError::RedisError(e.to_string()))?;

                pubsub_conn
                    .subscribe(&topic_clone)
                    .await
                    .map_err(|e| MessagingError::RedisError(e.to_string()))?;

                info!(topic = %topic_clone, "subscribed to Redis Pub/Sub topic");

                let mut msg_stream = pubsub_conn.on_message();
                while let Some(msg) = msg_stream.next().await {
                    let payload: String = msg.get_payload().map_err(|e| MessagingError::RedisError(e.to_string()))?;
                    match serde_json::from_str::<serde_json::Value>(&payload) {
                        Ok(json) => {
                            let topic = json.get("topic").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                            let timestamp_ms = json.get("timestamp_ms").and_then(|v| v.as_u64()).unwrap_or(0);
                            let payload_value = json.get("payload").cloned().unwrap_or_default();
                            let payload: M = serde_json::from_value(payload_value)
                                .map_err(|e| MessagingError::SerializationError(e.to_string()))?;
                            let envelope = Envelope { topic, payload, timestamp_ms };
                            if let Err(_) = tx.send(envelope).await {
                                debug!(topic = %topic_clone, "subscriber channel closed");
                                break;
                            }
                        }
                        Err(e) => {
                            error!(error = %e, topic = %topic_clone, "failed to deserialize message");
                        }
                    }
                }
                Ok(())
            }
            .await;

            if let Err(e) = result {
                error!(topic = %topic, error = %e, "Redis Pub/Sub subscriber error");
            }
        });

        Ok(rx)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 测试辅助：TokenThresholdEvent 消息类型
// ──────────────────────────────────────────────────────────────────────────────

/// 用于替代 auth_token_threshold 轮询的事件消息
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenThresholdEvent {
    pub person_unique: String,
    pub threshold_time: String,
    pub expired_tokens: Vec<String>,
    pub triggered_at_ms: u64,
}

// ──────────────────────────────────────────────────────────────────────────────
// 单元测试
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod messaging_tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_bus_publish_subscribe() {
        let bus = InMemoryBus::<String>::new();
        let mut rx = bus.subscribe("test-topic".to_string()).await.unwrap();

        bus.publish("test-topic".to_string(), "hello".to_string())
            .await
            .unwrap();

        let msg = rx.recv().await.unwrap();
        assert_eq!(msg.payload, "hello");
        assert_eq!(msg.topic, "test-topic");
    }

    #[tokio::test]
    async fn test_in_memory_bus_multiple_subscribers() {
        let bus = InMemoryBus::<i32>::new();
        let mut rx1 = bus.subscribe("nums".to_string()).await.unwrap();
        let mut rx2 = bus.subscribe("nums".to_string()).await.unwrap();

        bus.publish("nums".to_string(), 42).await.unwrap();

        let m1 = rx1.recv().await.unwrap();
        let m2 = rx2.recv().await.unwrap();
        assert_eq!(m1.payload, 42);
        assert_eq!(m2.payload, 42);
    }

    #[tokio::test]
    async fn test_in_memory_bus_no_subscribers_no_panic() {
        let bus = InMemoryBus::<String>::new();
        let result = bus.publish("empty-topic".to_string(), "no-one-listening".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_envelope_timestamp() {
        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let env = Envelope::new("t", "data");
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        assert!(env.timestamp_ms >= before && env.timestamp_ms <= after);
    }

    #[tokio::test]
    async fn test_redis_pubsub_bus_is_available() {
        let bus = RedisPubSubBus::<String>::new(None);
        assert!(!bus.is_available());

        let pool = super::RedisPool::from_url("redis://127.0.0.1:6379").await;
        let bus2: RedisPubSubBus<String> = RedisPubSubBus::new(pool.ok());
        assert!(bus2.is_available());
    }
}
