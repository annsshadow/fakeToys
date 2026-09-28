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

## 运行时（关键接口延迟）基准方法 —— 待带 DB 环境执行

服务启动即 `create_pool()` + `run_migrations()`，**强依赖 Postgres**；本沙箱无运行中的
Docker daemon / 本地 5432，故运行时延迟基准在此环境无法产出真实数字，仅记录可复现步骤：

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

- 脚本：`oa4rust/scripts/bench_release_profile.sh`
- 临时构建目录 `oa4rust/target-bench/{baseline,optimized}/`（各约数十 GB，测毕可删）。
