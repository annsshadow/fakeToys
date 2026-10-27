# 技术债台账

| ID | 登记日 | 模块 | 摘要 | 优先级 | 状态 |
|----|--------|------|------|--------|------|
| TD-1 | 2026-09-22 | augmentor | config 保存时重写 config.yaml 丢注释、清占位符 | 低 | 已修复（AU-Q1, 2026-10-24） |
| TD-2 | 2026-09-22 | oa4rust-web | file-type 16.5.4 传递依赖漏洞（Dependabot #127） | 低 | 已修复（2026-10-27，依赖链已移除） |
| TD-3 | 2026-10-24 | augmentor | chromadb 1.5.9（已是最新版）4 条服务端漏洞（预认证 RCE / 越权 / RBAC），上游无补丁 | 中 | 已接受（嵌入式用法不可达） |

---

## TD-1 config.yaml 被运行时重写，注释与占位符丢失

- **登记日**：2026-09-22
- **位置**：`augmentor/augmentor/config.py:1283`（`save_config()` 内 `yaml.safe_dump(data, ...)`；2026-10-14 自 `:423` 复核更新为 `:1274`，2026-10-22 再核自 `:1274` 漂移至此，代码重构后行号持续变动）
- **现象**：任何触发"保存配置"的运行时路径都会把内存态配置 dump 回 `config.yaml`，导致：
  1. 全部中文结构注释（约 300 行）被删除；
  2. `api_key: ${BAIDU_API_KEY}` 等环境变量占位符被替换为空值（如 `base_url: ''`）。
- **证据**：2026-09-22 工作区曾出现被重写的 config.yaml，已用 `git checkout --` 恢复；重写行为与 `config.py:423` 逻辑吻合。
- **影响**：仅影响本地可读性与占位符语义，不影响运行（程序读不到占位符时走各自默认/环境变量）。但用户手工编辑的注释会静默丢失。
- **修复方向（候选）**：
  1. 保存时只更新已存在的键值，用 ruamel.yaml 保留注释（引入新依赖）；
  2. 或将"可变配置"与"手工配置"分离：程序只写 `config.local.yaml`，不碰带注释的模板；
  3. 最小方案：保存前检测 diff 仅涉及注释/占位符时拒绝写回。
- **决策记录**：2026-09-22 仅登记不修复（本次工作区清理时发现，影响面小）。

---

## TD-2 file-type 传递依赖漏洞（Dependabot #127）—— 已通过移除依赖链修复

- **登记日**：2026-09-22　**关闭日**：2026-10-27
- **告警**：GitHub Dependabot #227（原 #127），GHSA-5v7r-6r5c-r473，medium。
- **原依赖链**：`@dcloudio/uni-mp-weixin`（uni-app 微信小程序构建工具链，dev-only）→ `jimp@0.22.12` → `@jimp/core` → `file-type@16.5.4`。
- **漏洞**：file-type ASF 解析器对畸形输入（零长度 sub-header）陷入无限循环（DoS）。影响范围 `>=13.0.0 <21.3.1`，补丁版 `21.3.1`。
- **为何不能靠升 file-type 修**（复核于 2026-10-27，结论与登记日一致）：
  1. 补丁版 `file-type@21.3.1` 是 **ESM-only + node>=20**，且 API 从 `fromBuffer` 改名 `fileTypeFromBuffer`；`@jimp/core` 是 CJS `require("file-type").default.fromBuffer(...)`，强推直接 TypeError；
  2. 坏点是 `jimp.read(Buffer)` 这条**唯一**被用到的路径（automator 截图/二维码解码），type-check/单测/构建都发现不了，属静默失败；
  3. 还会额外带进 4 个传递依赖（strtok3 10 / token-types 6 / uint8array-extras / @tokenizer/inflate），供应面反而变大。
- **实际修法（2026-10-27）**：不再抬 jimp，**撤掉** 2026-09-22 加的 `jimp@0.10.3 → 0.22.12` override，让 jimp 回到 `@dcloudio/uni-mp-weixin` 精确 pin 的 `0.10.3`（其链上是 `file-type@9.0.0`，不在受影响区间），原先靠抬 jimp 绕开的 phin 2.9.3 改为直接 `'phin@<3.7.1': '3.7.1'` 覆盖。见 `oa4rust/pnpm-workspace.yaml` 的 overrides 注释。
- **安全性论证**：jimp 在 oa4rust 中只被 `@dcloudio/uni-mp-weixin/lib/uni.automator.js` 引用（二维码解码，传 Buffer），本仓未启用 automator；phin 3 是 promise-only，而 @jimp/core 的回调式调用只在 `jimp.read(远程 URL)` 时触发，该路径既不启用也不受影响。
- **验证**：`pnpm audit` 从 2 条（file-type + body-parser）降到 0；typecheck / lint / build(desktop) / build:h5 / build:mp-weixin 全绿且零告警，mobile 单测 121 条通过。

---

## TD-3 chromadb 1.5.9 四条服务端漏洞，上游无补丁（嵌入式用法不可达）

- **登记日**：2026-10-24
- **告警**：`pip-audit -r augmentor/requirements.txt` 报 chromadb 1.5.9 四条：PYSEC-2026-311（预认证代码注入 RCE）、PYSEC-2026-3814（认证后代码注入）、PYSEC-2026-3815（SimpleRBAC 鉴权缺陷）、PYSEC-2026-3813（缺授权校验可任意读写）。
- **为何无法升级修复**：1.5.9 已是 PyPI 最新版（`pip-audit` 的 Fix Versions 为空，crates/PyPI 均无更高修补版）——属**上游无修复**，与 cargo 侧 rsa/lopdf/zip 同类。
- **为何判为不可达（补偿控制）**：augmentor 只用**嵌入式** `chromadb.PersistentClient(path=...)`（`augmentor/vector/chromadb.py:80`，`semantic.py` 注释「自管落盘」），**从不**起 chromadb HTTP server（无 `HttpClient` / `chromadb run` / 监听端口）。四条漏洞全是 chromadb **服务端** API 的预认证/越权/RCE，需要暴露的网络服务与鉴权边界才触达；单进程本地嵌入式用法下不可达。
- **处置**：登记为已接受技术债（不等同于「安全」）。待 chromadb 发布修补版即升；若将来改用 `HttpClient` 连远端/共享 chromadb server，必须重新评估（届时可达）。
- **同期已修（对照）**：本轮 dependabot 清理中，cargo 侧 rustls/webpki-roots/rustls-webpki/ctutils 已升到修补版（清 7 条），zip/rsa/lopdf 按既有 rustsec-exceptions.toml 登记例外；npm augmentor/web 的 @vitest/mocker 已升 5.0.5（清 2 条）；pnpm oa4rust 的 postcss-selector-parser 已覆盖到 7.1.6（清 1 条）。
