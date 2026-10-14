# 2026-10-14 安全审计（依赖 + 代码级 复验 + 新增发现）

本审计在独立 worktree（分支 `oa4rust-vuln-audit`）上进行，避免污染主工作区。
范围：oa4rust 全栈（Rust 后端 96 crate / Vue 前端 / uni-app 移动端 / MCP 服务）。
方法：

- **依赖**：拉取 RustSec advisory-db（`@ 6aed8466`）对 `Cargo.lock`（517 个 registry 包）
  全量逐条版本匹配（1290 条通告）；前端/移动端 `pnpm audit`。
- **代码级**：对注入（format!/拼接 SQL、命令执行、文件穿越、SSRF、弱加密、不安全随机）
  各族 pattern 逐一人工复核关键落点，并与 2026-10-12 审计结论对账。

## 一、新增发现（本次已修复）

### V-A（high，供应链）quinn-proto 0.11.17 → 3 条 RustSec DoS 通告

2026-09-14 起 RustSec 收录 3 条 quinn-proto 远程 panic/DoS 通告，
**晚于 2026-10-12 审计所用的固定库与例外清单**，故上一轮未捕获：

| 通告 | 标题 | 修复版 |
| --- | --- | --- |
| RUSTSEC-2026-0338 | 零长 STREAM 帧在 unordered read 时 panic `RangeSet` | >= 0.11.18 |
| RUSTSEC-2026-0340 | 对端最小 ACK 延迟 panic | >= 0.11.18 |
| RUSTSEC-2026-0343 | 出站 datagram 缓冲记账 panic 发送方 | >= 0.11.18 |

- **依赖链**：`reqwest 0.12.28 (optional http3) → quinn 0.11.11 → quinn-proto 0.11.17`。
- **可达性**：当前特性集下 `http3` 未启用，`cargo tree -i quinn` 无构建反向依赖，
  故 quinn 系**不在运行时构建图**（供应链层风险，非现行可利用路径）；但 `Cargo.lock`
  中残留，`cargo audit`（每日浮动门）会持续命中。
- **修复**（`Cargo.lock`）：`cargo update -p quinn -p quinn-proto`
  → `quinn 0.11.11 → 0.11.12`、`quinn-proto 0.11.17 → 0.11.19`（已含 0.11.18 修复）。
- **配套**：`security/rustsec-db.rev` 由 `b50980aa` 抬到 `6aed8466`（含上述 3 通告），
  使固定门亦可感知并复现本次结论。

## 二、既有例外复验（仍有效，非新增）

| 项 | 通告 | 状态 |
| --- | --- | --- |
| `lopdf 0.26.0` | RUSTSEC-2026-0187（栈溢出 DoS，high） | 已有例外（`security/rustsec-exceptions.toml`，2026-10-28 到期）：genpdf 0.2.0 无补丁链且仅用于**生成** PDF（无 `load*` 入口），解析/签名走 crates/signature 的 lopdf 0.42（已修）。复验确认可达性结论不变。 |
| `rsa 0.9.10` | RUSTSEC-2023-0071（Marvin 时序侧信道，medium） | 已有例外（2026-11-08 到期）：仅 dev-dependency 的 MySQL 测试驱动可达，生产 OIDC/PDF 签名用 ring。复验确认。 |

## 三、信息级（unmaintained / deprecated，按政策不处置）

`audit.toml` 将 `unmaintained` 设为 `informational_warnings`。命中并记录在案：
`adler 1.0.2`（RUSTSEC-2025-0056）、`encoding`（2021-0153）、`lzw 0.10.0`（2020-0144）、
`proc-macro-error 1.0.4`（2024-0370）、`ring 0.17.14`（RUSTSEC-2025-0007）、
`rusttype 0.8.3`（2021-0140）、`stb_truetype 0.3.1`（2020-0020）、`stdweb 0.4.20`（2020-0056）、
`ttf-parser 0.25.1`（RUSTSEC-2026-0192）、`unsafe-libyaml 0.2.11`（2024-0449）。
均为纯信息级，随既有依赖链，不影响 `--deny unsound/vulnerable` 门。

## 四、前端 / 移动端（pnpm audit，复验既有决策）

3 条命中与 2026-10-12 审计"保留 3 条"完全一致（移动端 uni-app 构建工具链，非运行时）：

- `braces <=3.0.3`（无修复版；chokidar dev 工具，glob 输入非用户可控）
- `file-type`（>=13 <21.3.1；jimp 硬 pin ^16，构建期图片探测）
- `postcss-selector-parser <7.1.6`（uni-h5 硬 pin 6.x，已抬到 6.x 末版 6.1.3，7.x 破坏 API）

**无新增**前端漏洞。既有安全 override（source-map-js 1.2.2、vue 3.5.42 家族）仍生效。

## 五、代码级复核结论（未发现新的真实漏洞）

关键落点逐一人工复核，均已有防御或不在请求输入可达路径：

- **SQL**：动态表/列名均为 `&'static str` 编译期常量 + `$N` 参数绑定；
  `format!` 命中点无请求输入插值。
- **命令执行**：仅 `console`（白名单 `uname/df/free/ps/uptime` + 参数字符白名单 +
  `is_admin` 门禁 + 非 shell `Command::new`）与 `preview`（libreoffice 参数化、无 shell）。
- **SSRF**：出站 URL 来自 env（`AI_API_BASE`/`AI_API_KEY`、MCP 配置）；
  用户可控 URL 走 `shared::netguard`（IP 黑名单 + 重定向逐跳校验）。
- **路径穿越**：`shared/storage` 拒 `ParentDir`/绝对路径；`preview::safe_base_name`
  取末段并拒 `.`/`..`/NUL；SPA 回退拒 `..` 段。
- **MCP 服务**：默认绑定回环；`MCP_API_KEY` 设置后 `/mcp` 强制 `X-API-Key`
  （常量时间比较）；非回环且无 key 时启动告警。
- **会话/密码**：HttpOnly + Secure（生产强制）+ SameSite=Lax；reset 码 6 位 +
  5 次 + 5 分钟 TTL + 限流；新密码 bcrypt（cost 10），遗留 MD5/DES 走
  `AUTH_LEGACY_COMPAT` 且默认 fail-closed。

## 六、验证

- **改动纯手术式**：`git show -- Cargo.lock` 仅修改 `quinn`、`quinn-proto` 两个
  `[[package]]` 块的 `version`/`checksum` 及其依赖指针（重新指向 lock 中**已存在**的
  `socket2 0.5.10`、`getrandom 0.3.4`、`windows-sys 0.48.0` 块）。
  `[[package]]` 块总数 690→690，**无任何块增删** → 主构建图字节级不变。
- **依赖解析有效**：`cargo metadata` 通过（690 包，`quinn 0.11.12` / `quinn-proto 0.11.19` /
  `quinn-udp 0.5.15` 均在图谱中）；`cargo tree -i quinn`（含 `--target all`）无构建反向
  依赖 → 确认 quinn 系不在任何构建目标中（http3 特性未启用）。
- **无新增通告命中**：对 `6aed8466` 库复扫，quinn-proto 3 条已清零；剩余命中仅
  lopdf/rsa（既有例外）与 10 条 unmaintained（信息级）。
- **DB pin 有效**：`6aed8466` 为 RustSec advisory-db 真实 commit，含 3 条 quinn 通告，
  固定门可据此复现本次结论。

### 完整编译 / 单测验证（Docker Linux 容器，硬证据）

本机 host 为 `x86_64-pc-windows-gnu` 且缺 mingw 工具链（`dlltool`/
`x86_64-w64-mingw32-gcc` 不可用），原生 `cargo build/test` 在两个 C 组件
（`windows-sys`、`getrandom`，经 socket2/rand→captcha→auth 等主构建图、**与 quinn
无关**）上因 `dlltool: program not found` 中止。为补齐完整验证，改用 Docker 在 Linux
容器（`quay.io/fedora` + gcc + Rust **1.98.1**，即 CI 钉版）执行：

- **`cargo check --workspace`：通过**（exit 0，全 96 crate + 主二进制 `oa4rust` 编译成功，
  27m22s）。证明更新后的 `Cargo.lock`（quinn 0.11.12 / quinn-proto 0.11.19）下整树
  **无编译回归**。
- **`cargo test --lib -p mcp_server -p console -p shared`：全绿**（exit 0）——
  `console` 13、`mcp_server` 19、`shared` 151（+11 ignored），与 2026-10-12 审计读数一致。

（完整 DB 集成测试套件 `cargo test --workspace --lib` 依赖 Postgres，其 2026-10-12 已证
绿；本次 quinn 锁改动不在构建/测试图内、不触 DB，故以「全树编译 + 无 DB 单测」为验证
口径。）

## 七、处置

- 已修：quinn-proto 依赖升级 + DB pin 抬升（本分支 `oa4rust-vuln-audit` 的
  `Cargo.lock` / `security/rustsec-db.rev` 两处改动）。
- 留档：lopdf / rsa 例外在其到期日前（2026-10-28 / 2026-11-08）按既有 30 天重审流程处理。
- 无需处置：unmaintained 信息级、前端 3 条既有保留项。
