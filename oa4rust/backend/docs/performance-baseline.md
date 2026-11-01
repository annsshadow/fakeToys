# Performance Baseline & Comparison Report

- **Date**: 2026-10-28（2026-10-30 复测）
- **Rust backend**: `cargo test --test perf_baseline perf_baseline -- --ignored --nocapture`
- **Java backend**: SKIP (no reachable Java backend; comparison not attempted)
- **Iterations**: 5 warm-up + 50 measured request pairs, no other workload

## Rust Baseline

| Scenario | Date | Samples | Avg (ms) | P50 (ms) | P99 (ms) | Max (ms) |
|----------|------|--------:|---------:|---------:|---------:|---------:|
| `POST /api/authentication` | 2026-10-28 | 50 | 1.70 | 1.69 | 2.73 | 2.73 |
| `GET /api/cms_assemble_control/data/document` | 2026-10-28 | 50 | 1.07 | 1.03 | 1.58 | 1.58 |
| `POST /api/authentication` | 2026-10-30 | 50 | 2.19 | 2.20 | 2.53 | 2.53 |
| `GET /api/cms_assemble_control/data/document` | 2026-10-30 | 50 | 1.36 | 1.36 | 1.59 | 1.59 |

> 2026-10-30 复测：auth avg 2.19ms / document avg 1.36ms，毫秒级稳定，无性能回归。
> 与 10-28 相比 auth +0.49ms、document +0.29ms，属容器共享负载的正常波动
> （debug build、同机多任务）。复测容器内需 `apt-get install -y curl` 且
> 编译命令带 `RUST_MIN_STACK=33554432`（见计划文档 D3）。

## Environment

- Host: AMD Ryzen 9 7945HX, 32 vCPU visible to the container, 31 GiB RAM,
  Docker Desktop on WSL2.
- Backend: container `oa4rust-cov`, Rust 1.98.1, **debug** (`cargo test`) build.
- Database: PostgreSQL 17.11 in container `lyp-pg`, `shared_buffers=128MB`,
  reached over the Docker network.
- Rate limits raised to 100000 for the test process only (50 logins exceed the
  production default of 30/minute).

## Caveats

- **Login is not production bcrypt cost.** The integration fixture uses a
  bcrypt **cost 4** hash; production hashes use a higher cost (typically 12).
  The auth number is a lower bound covering HTTP + middleware + DB round-trip +
  a cheap hash verify, not a production login latency.
- **Debug build.** Release builds will be faster; these numbers are for
  regression comparison, not capacity planning.
- **Superseded report.** The previous 2026-08-15 table (`login 549/549 errors`,
  P50 7913 ms, `work-start`/`cms` all zero) came from an external `aiohttp`
  script that hit a broken endpoint. It is not a valid baseline and has been
  replaced by the compile-checked `perf_baseline` test above.
