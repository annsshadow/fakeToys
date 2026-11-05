// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! O28：微信模板消息投递结果可观测性（真实 worker + 本地假微信 API）。
//!
//! 改前缺陷：异步队列 worker 只把「传输/解码错误」记日志，微信返回的 `errcode≠0`
//! 被当作成功静默丢弃——投递失败在日志里完全不可见（既无 success 字段也无 errcode）。
//!
//! 本测试用 `MPWEXIN_API_BASE` 指向本地假 API，让真实 worker 走完整 HTTP 链路，
//! 断言「成功」与「业务拒绝」两条结果都留下结构化日志。

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use auth::mpweixin::{enqueue_template_message, spawn_template_queue_worker, TemplateMessageTask};

/// 记录到内存的日志 writer（`tracing_subscriber::fmt` 的 MakeWriter）。
struct CaptureWriter(Arc<Mutex<Vec<u8>>>);

impl Write for CaptureWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn install_log_capture(buf: Arc<Mutex<Vec<u8>>>) {
    let sink = buf.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .with_writer(move || CaptureWriter(sink.clone()))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("install tracing subscriber");
}

struct FakeWechat {
    base_url: String,
    requests: Arc<Mutex<Vec<String>>>,
}

/// 本地假微信 API：`/cgi-bin/token` 返回固定 token；模板发送按 body 里的
/// `tpl-reject` 区分为「业务拒绝（errcode=40001）」或「成功（msgid=msg-ok）」。
fn start_fake_wechat() -> FakeWechat {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind fake wechat");
    let port = listener.local_addr().unwrap().port();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let recorded = requests.clone();

    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let Some((head, body)) = read_request(&mut stream) else {
                continue;
            };
            let request_line = head.lines().next().unwrap_or("").to_string();
            recorded.lock().unwrap().push(request_line.clone());

            let response_body = if request_line.contains("/cgi-bin/token") {
                r#"{"access_token":"fake-access-token","expires_in":7200}"#.to_string()
            } else if body.contains("tpl-reject") {
                r#"{"errcode":40001,"errmsg":"invalid credential"}"#.to_string()
            } else {
                r#"{"errcode":0,"errmsg":"ok","msgid":"msg-ok"}"#.to_string()
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });

    FakeWechat {
        base_url: format!("http://127.0.0.1:{port}"),
        requests,
    }
}

fn read_request(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        let n = stream.read(&mut chunk).ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&chunk[..n]);
        let Some(head_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") else {
            continue;
        };
        let head = String::from_utf8_lossy(&buf[..head_end]).to_string();
        let body_start = head_end + 4;
        let content_length = head
            .lines()
            .find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .and_then(|v| v.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        while buf.len() < body_start + content_length {
            let n = stream.read(&mut chunk).ok()?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
        }
        let body = String::from_utf8_lossy(&buf[body_start..]).to_string();
        return Some((head, body));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delivery_outcomes_are_observable_for_success_and_rejection() {
    let fake = start_fake_wechat();
    std::env::set_var("MPWEXIN_API_BASE", &fake.base_url);
    std::env::set_var("MPWEXIN_APP_ID", "probe-app-id");
    std::env::set_var("MPWEXIN_APP_SECRET", "probe-app-secret");

    let logs = Arc::new(Mutex::new(Vec::new()));
    install_log_capture(logs.clone());

    spawn_template_queue_worker();
    assert!(enqueue_template_message(TemplateMessageTask {
        touser: "user-reject".into(),
        template_id: "tpl-reject".into(),
        content: r#"{"thing1":{"value":"hi"}}"#.into(),
    }));
    assert!(enqueue_template_message(TemplateMessageTask {
        touser: "user-ok".into(),
        template_id: "tpl-ok".into(),
        content: r#"{"thing1":{"value":"ok"}}"#.into(),
    }));

    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let captured = String::from_utf8_lossy(&logs.lock().unwrap()).to_string();
        if captured.contains("user-reject")
            && captured.contains("user-ok")
            && captured.contains("msgid=\"msg-ok\"")
        {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "5s 内没有等到两条投递日志；当前捕获内容：\n{captured}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let captured = String::from_utf8_lossy(&logs.lock().unwrap()).to_string();

    // 业务拒绝必须留下失败结果：success=false + errcode=40001。
    let reject_line = captured
        .lines()
        .find(|line| line.contains("user-reject"))
        .expect("缺少失败投递日志");
    assert!(
        reject_line.contains("success=false"),
        "拒绝未被标记为失败：{reject_line}"
    );
    assert!(
        reject_line.contains("errcode=40001"),
        "拒绝日志缺少 errcode：{reject_line}"
    );
    assert!(
        reject_line.contains("rejected"),
        "拒绝日志未说明投递失败：{reject_line}"
    );

    // 成功必须留下 msgid，便于与微信侧对账。
    let ok_line = captured
        .lines()
        .find(|line| line.contains("user-ok"))
        .expect("缺少成功投递日志");
    assert!(ok_line.contains("success=true"), "成功未被标记：{ok_line}");
    assert!(
        ok_line.contains("msgid=\"msg-ok\""),
        "成功日志缺少 msgid：{ok_line}"
    );

    // 规模下限：成功与失败各至少一条结构化投递日志。
    let delivery_lines = captured
        .lines()
        .filter(|line| line.contains("wechat template message"))
        .count();
    assert!(
        delivery_lines >= 2,
        "结构化投递日志只有 {delivery_lines} 条，少于成功/失败各一条的下限"
    );

    // 原功能没坏：真实发出了 token 请求和两条发送请求。
    let requests = fake.requests.lock().unwrap().clone();
    assert!(
        requests.iter().any(|r| r.starts_with("GET /cgi-bin/token")),
        "worker 未请求 access_token：{requests:?}"
    );
    assert!(
        requests
            .iter()
            .filter(|r| r.starts_with("POST /cgi-bin/message/wxopen/template/send"))
            .count()
            >= 2,
        "worker 发送请求数不足两条：{requests:?}"
    );
}
