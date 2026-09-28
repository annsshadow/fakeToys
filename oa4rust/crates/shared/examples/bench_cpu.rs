// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// 纯 CPU 密集路径微基准（无外部 bench 依赖，避免污染 Cargo.lock）。
// 用同一份源码分别以「thin-LTO 开 / 关」构建本 example 并各跑一遍，量化
// [profile.release] thin-LTO 对 CPU 路径的运行时收益：
//
//   # LTO 开（本仓 [profile.release]）
//   cargo run --release -p shared --example bench_cpu
//   # LTO 关（默认发布档）
//   cargo run --release -p shared --example bench_cpu \
//     --config profile.release.lto=false --config profile.release.codegen-units=16
//
// 选取两类真实热点：serde_json 响应序列化往返、SHA-256 会话/签名摘要。
use std::hint::black_box;
use std::time::Instant;

use sha2::{Digest, Sha256};

fn make_value(n: usize) -> serde_json::Value {
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        items.push(serde_json::json!({
            "id": format!("row-{i}"),
            "name": format!("流程节点 {i}"),
            "enabled": i % 2 == 0,
            "weight": (i as f64) * 1.5,
            "tags": ["a", "b", "c"],
            "meta": { "created": 1_700_000_000_i64 + i as i64, "score": i * 3 },
        }));
    }
    serde_json::json!({ "code": 0, "message": "ok", "data": items })
}

fn bench<F: FnMut()>(label: &str, iters: u32, mut f: F) {
    // 预热
    for _ in 0..(iters / 10).max(1) {
        f();
    }
    let t = Instant::now();
    for _ in 0..iters {
        f();
    }
    let ns = t.elapsed().as_nanos() as f64;
    println!("{label:<28} iters={iters}  total={:.1}ms  per_op={:.2}µs", ns / 1e6, ns / iters as f64 / 1e3);
}

fn main() {
    let value = make_value(200);
    let serialized = serde_json::to_string(&value).unwrap();
    println!("payload_bytes={}", serialized.len());

    bench("json_serialize", 20_000, || {
        let s = serde_json::to_string(black_box(&value)).unwrap();
        black_box(s);
    });
    bench("json_roundtrip", 20_000, || {
        let s = serde_json::to_string(black_box(&value)).unwrap();
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        black_box(v);
    });
    let data = serialized.as_bytes();
    bench("sha256_digest", 200_000, || {
        let mut h = Sha256::new();
        h.update(black_box(data));
        black_box(h.finalize());
    });
}
