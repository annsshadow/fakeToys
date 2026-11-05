// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// ──────────────────────────────────────────────────────────────────────────────
// graceful — 优雅停机
//
// 修前 `axum::serve(...).await` 没有接任何关闭信号：进程收到 SIGTERM 时由
// 运行时直接终止，正在执行的请求（尤其是多语句事务）会被**腰斩**。
// 表现是：部署一次就产生一批「客户端 500 / 数据写了一半」的投诉，
// 而日志里没有任何痕迹 —— 因为进程根本没机会写。
//
// 现在的行为：
//   1. 收到 SIGINT / SIGTERM → 停止接受新连接；
//   2. 已在途的请求继续跑完；
//   3. 超过 grace period 仍未结束的，记录 warn 并强制退出。
// ──────────────────────────────────────────────────────────────────────────────

use std::future::{Future, IntoFuture};
use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use tokio::net::TcpListener;

/// 默认宽限期。比典型的部署探活间隔长，又短于编排系统的 SIGKILL 超时。
pub const DEFAULT_GRACE_PERIOD: Duration = Duration::from_secs(10);

/// 启动 HTTP 服务，并保证「先排空、后退出」，且排空有上界。
///
/// `shutdown` 是外部注入的关闭信号 future；生产传 [`shutdown_signal`]，
/// 测试传 `oneshot::Receiver`，避免测试进程给自己发 SIGTERM。
pub async fn serve_router_with_shutdown<F>(
    listener: TcpListener,
    app: Router,
    grace: Duration,
    shutdown: F,
) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    // 记录“关闭信号已到达”，用于只在**排空阶段**套宽限期。
    // 修前 `timeout(grace, server)` 套在整个服务生命周期上：无论有没有关机
    // 信号，进程启动 10s 后都会被强制退出（实测 listening → +10s 退出）。
    let (draining_tx, draining_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown = async move {
        shutdown.await;
        let _ = draining_tx.send(());
    };

    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown)
    .into_future();
    tokio::pin!(server);

    tokio::select! {
        result = &mut server => result,
        _ = draining_rx => {
            // 关闭信号之后才计宽限期：排空超时则记录并强制退出。
            match tokio::time::timeout(grace, &mut server).await {
                Ok(result) => result,
                Err(_) => {
                    tracing::warn!(
                        grace_ms = grace.as_millis() as u64,
                        "graceful shutdown timed out; forcing exit"
                    );
                    Ok(())
                }
            }
        }
    }
}

/// 等待 SIGINT（Ctrl+C）或 SIGTERM（容器/编排系统的默认停止信号）。
pub async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!(error = %err, "failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(err) => tracing::error!(error = %err, "failed to install SIGTERM handler"),
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received; draining in-flight requests");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::{oneshot, Notify};

    /// 向本机端口发出一个 GET，并读到 EOF（服务端带 Connection: close）。
    async fn request(addr: SocketAddr, path: &str) -> String {
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(
                format!(
                    "GET {} HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
                    path
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// 没有关机信号时，宽限期不能作用于服务本身。
    ///
    /// 修前 `timeout(grace, server)` 包住整个服务：宽限期一到就强制退出，
    /// 这个用例（等 3 倍宽限期后再请求）会因连接被拒而红。
    #[tokio::test]
    async fn server_keeps_running_before_shutdown_signal() {
        let app = Router::new().route("/ok", get(|| async { "ok" }));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel::<()>();
        let server = tokio::spawn(serve_router_with_shutdown(
            listener,
            app,
            Duration::from_millis(100),
            async {
                let _ = rx.await;
            },
        ));

        tokio::time::sleep(Duration::from_millis(300)).await;
        let body = request(addr, "/ok").await;
        assert!(
            body.contains("200 OK"),
            "关机信号到来前服务被宽限期误杀：{body}"
        );
        tx.send(()).unwrap();
        server.await.unwrap().unwrap();
    }

    /// 关闭信号到达时，在途请求必须**跑完**再退出。
    #[tokio::test]
    async fn drains_in_flight_request() {
        let entered = Arc::new(Notify::new());
        let app = Router::new().route(
            "/slow",
            get({
                let entered = entered.clone();
                move || {
                    let entered = entered.clone();
                    async move {
                        // 必须先确认 handler 真的开始跑了，再触发关闭；
                        // 否则连接可能还躺在 accept backlog 里，hyper 会直接丢弃。
                        entered.notify_one();
                        tokio::time::sleep(Duration::from_millis(200)).await;
                        "done"
                    }
                }
            }),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel::<()>();
        let server = tokio::spawn(serve_router_with_shutdown(
            listener,
            app,
            Duration::from_secs(5),
            async {
                let _ = rx.await;
            },
        ));

        let client = tokio::spawn(request(addr, "/slow"));
        entered.notified().await;
        tx.send(()).unwrap();

        let body = client.await.unwrap();
        assert!(body.contains("200 OK"), "in-flight 请求被中断：{body}");
        assert!(body.ends_with("done"), "响应体不完整：{body}");
        server.await.unwrap().unwrap();
    }

    /// 卡住的请求不能无限拖住宿主：超过宽限期必须强制返回。
    #[tokio::test]
    async fn forced_exit_after_grace_period() {
        let entered = Arc::new(Notify::new());
        let app = Router::new().route(
            "/hang",
            get({
                let entered = entered.clone();
                move || {
                    let entered = entered.clone();
                    async move {
                        entered.notify_one();
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        "never"
                    }
                }
            }),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel::<()>();
        tokio::spawn(request(addr, "/hang"));
        let server = tokio::spawn(serve_router_with_shutdown(
            listener,
            app,
            Duration::from_millis(100),
            async {
                let _ = rx.await;
            },
        ));

        entered.notified().await;
        tx.send(()).unwrap();
        let started = std::time::Instant::now();
        server.await.unwrap().unwrap();
        let elapsed = started.elapsed();
        assert!(
            elapsed >= Duration::from_millis(90),
            "未等待宽限期就退出：{elapsed:?}"
        );
        assert!(
            elapsed < Duration::from_secs(1),
            "宽限期未生效，被卡住：{elapsed:?}"
        );
    }
}
