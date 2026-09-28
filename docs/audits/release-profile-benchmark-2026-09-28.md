<!--
Copyright (C) 2026 annsshadow
SPDX-License-Identifier: AGPL-3.0-or-later
-->
# oa4rust 发布构建（[profile.release]）性能基准 — 2026-09-28

量化提交 `854cb6b3e` 引入的 `[profile.release]`（thin-LTO + codegen-units=1 +
strip=symbols + opt-level=3）相对「无 profile.release 时 Cargo 默认发布档」
（opt-level=3 + codegen-units=16 + lto=false + strip=false）的收益。

## 方法（可复现）

一次运行、同一份当前源码、两次构建仅 profile 参数不同——受控 A/B，用 CLI `--config`
覆盖 profile 参数注入基线档，两个二进制落在独立 target 目录互不覆盖：

```bash
cd oa4rust
bash scripts/bench_release_profile.sh
```

脚本内部（节选）：

```bash
# baseline（默认发布档）
cargo build --release --bin oa4rust --target-dir target-bench/baseline \
  --config profile.release.lto=false \
  --config profile.release.codegen-units=16 \
  --config profile.release.strip=false
# optimized（本仓提交档）
cargo build --release --bin oa4rust --target-dir target-bench/optimized
```

- 主机：Windows 10 / x86_64-pc-windows-msvc；rustc/cargo 1.96.0。
- `CARGO_INCREMENTAL=0`（规避本机 Windows 增量编译 ICE，见项目记忆）。

## 结果

| 指标 | baseline（默认档） | optimized（本仓档） | Δ |
|---|---:|---:|---:|
| 二进制 `oa4rust.exe` | 143,748,096 B（137.1 MiB） | 133,523,456 B（127.3 MiB） | **−7.1%**（−10.2 MB） |
| 调试符号 `oa4rust.pdb` | 30,019,584 B（28.6 MiB） | 12,931,072 B（12.3 MiB） | **−56.9%**（−17.1 MB） |
| exe+pdb 合计落盘 | 173.77 MB | 146.45 MB | **−15.7%** |
| 全量构建墙钟 | 1626 s（27.1 min） | 1353 s（22.6 min） | −16.8%（单次，见下注） |

### 解读

- **二进制体积 −7.1%**：thin-LTO 的跨 crate 死代码消除/内联在 Windows/MSVC 的 `.exe`
  上即可兑现；codegen-units=1 消除跨单元重复代码，进一步压缩。
- **符号 −56.9%**：codegen-units=1 + LTO 显著减少调试信息体量。
- **Linux 生产目标收益更大**：MSVC 把符号表放在独立 `.pdb`，故 `strip=symbols` 在
  Windows `.exe` 上几乎不改体积；在 Linux ELF 上 `strip` 会额外移除二进制内嵌符号表，
  因此**部署实测（ubuntu，CI/生产目标）应在 −7.1% 之上再叠加 strip 的体积削减**。
- **构建墙钟 −16.8% 属单次顺序测量，有噪声**（两次构建先后串行、共享 OS 文件缓存与
  已下载依赖），不作为稳定结论；体积为确定性数字，可信。运行时（LTO 内联的执行加速）
  需下节的接口延迟基准，本沙箱无 DB 未能执行。

## 运行时（关键接口延迟）基准 —— 已执行

主机本地 PostgreSQL 15（`postgres://o2server:password@localhost:5432/oa4rust`，379 表，
已迁移）可用；两档二进制均取自当前 HEAD（optimized 用本地 `swagger-ui-v5.17.12.zip`
经 `SWAGGER_UI_DOWNLOAD_URL=file://…` 离线构建，规避 build.rs 网络下载）。

harness：`scripts/bench_endpoint_latency.sh`——起服务→等 200 就绪→预热 50→采样 N，
对无鉴权且 CPU 密集的验证码端点 `GET /api/authentication/captcha`（图像生成，LTO 敏感）
测 curl 往返耗时：

```bash
bash scripts/bench_endpoint_latency.sh \
  target-bench/baseline/release/oa4rust \
  target-bench/optimized/release/oa4rust 500
```

### 结果（N=500，localhost，同机顺序，单轮）

| 分位 | baseline（默认档） | optimized（thin-LTO） | Δ |
|---|---:|---:|---:|
| mean | 2.11 ms | 1.96 ms | **−7.1%** |
| p50 | 1.72 ms | 1.67 ms | −2.9% |
| p95 | 1.98 ms | 1.94 ms | −2.0% |
| p99 | 20.50 ms | 17.65 ms | **−13.9%** |

- thin-LTO 在各分位上一致小幅领先，尾延迟 p99 改善最明显（−13.9%）。
- **口径与噪声**：单轮、同机、localhost（无网络往返），p50/p95 的 2–3% 差异接近运行间
  抖动量级，方向一致但不宜过度解读；mean 与 p99 的改善更实在。这是 I/O/框架开销占主导的
  端点，LTO 的 CPU 内联收益本就有限——若要更强证据可多轮取中位或用 `hyperfine`/`wrk`
  提升并发与样本量。

## 运行时基准复现步骤（其他环境）


```bash
# 1) 起依赖栈（提供 postgres://o2server:password@localhost:5432/oa4rust）
docker compose -f oa4rust/docker-compose.yml up -d postgres

# 2) 分别以两档二进制起服务（同一 DB、同一 seed），各测一遍
target-bench/baseline/release/oa4rust  &   # 或 optimized/
#   等待就绪后，对无鉴权只读端点做延迟采样（示例：OpenAPI / 健康检查）：
for i in $(seq 1 500); do
  curl -s -o /dev/null -w '%{time_total}\n' http://localhost:3000/openapi.json
done | sort -n | awk '{a[NR]=$1} END{print "p50="a[int(NR*0.5)]" p95="a[int(NR*0.95)]}'
```

对比两档的 p50/p95 即得 LTO 的运行时收益。建议同时用 `hyperfine`/`wrk` 提升可信度，
并固定 seed 数据与并发度。

## 复现产物与清理

- 脚本：`oa4rust/scripts/bench_release_profile.sh`（体积/构建）、
  `oa4rust/scripts/bench_endpoint_latency.sh`（接口延迟）。
- 临时构建目录 `oa4rust/target-bench/{baseline,optimized}/` 及 `server-*.log`/`lat-*.txt`
  （已 gitignore；测毕可删）。

## 附：三端质量门禁现状（2026-09-28 核验）

后端与前端两端的质量门禁已成体系，本次核验其覆盖与绿态：

| 端 | 门禁 | 命令 | 结果 |
|---|---|---|---|
| 后端 Rust | fmt / clippy(-D warnings) / 守卫测试 | `cargo fmt --check`；`cargo clippy --workspace --all-targets -- -D warnings` | 0 告警（见 CI `oa4rust-ci.yml`） |
| 桌面+移动+SDK | typecheck（全 6 包） | `pnpm typecheck`（递归 tsc --noEmit） | exit 0，locales/sdk/apis/mobile/ui/desktop 全 Done |
| 前端全树（含 mobile/SDK） | biome lint | `pnpm lint`（`biome check --diagnostic-level=error .`） | 235 文件 0 error |
| 桌面/移动 | vitest / 构建 | `pnpm test`、`pnpm test:mobile`、desktop/mobile build | 见 `oa4rust-web.yml` |

结论：**移动端与 SDK 已与后端同级纳入 lint + typecheck 门禁**（`oa4rust-web.yml` 的
`pnpm typecheck` 递归覆盖全部 6 个包，`pnpm lint` 覆盖整树），无缺口需新建。

**已知强度差（展望）**：前端 biome 门禁按 `--diagnostic-level=error` 只拦 error；
warn 级尚有 6390 条积压（`--error-on-warnings` 未启用）。后端 clippy 则已 `-D warnings`
全拦。要达到与后端完全同强度，需先清理这批 biome warning 再收紧门禁——属独立的
增量清理任务，非本轮范围。

