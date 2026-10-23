# 技术债台账

| ID | 登记日 | 模块 | 摘要 | 优先级 | 状态 |
|----|--------|------|------|--------|------|
| TD-1 | 2026-09-22 | augmentor | config 保存时重写 config.yaml 丢注释、清占位符 | 低 | 已修复（AU-Q1, 2026-10-24） |
| TD-2 | 2026-09-22 | oa4rust-web | file-type 16.5.4 传递依赖漏洞（Dependabot #127），补丁不兼容暂不处理 | 低 | 已接受 |
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

## TD-2 file-type 传递依赖漏洞（Dependabot #127）补丁不兼容，暂不处理

- **登记日**：2026-09-22
- **告警**：GitHub Dependabot #127，GHSA-5v7r-6r5c-r473，medium。
- **依赖链**：`@dcloudio/uni-mp-weixin`（uni-app 微信小程序构建工具链，dev-only）→ `jimp@0.22.12` → `@jimp/core` → `file-type@16.5.4`。
- **漏洞**：file-type ASF 解析器对畸形输入（零长度 sub-header）陷入无限循环（DoS）。影响范围 `>=13.0.0 <21.3.1`，补丁版 `21.3.1`。
- **为何暂不修**：
  1. 补丁版 `file-type@21.3.1` 为 **ESM-only + node>=20**，而 jimp 0.22.12 用 CJS `require` 加载它，override 到 21.x 会直接破坏构建；
  2. 无攻击面：file-type 仅在小程序**构建期**解析项目自有图片资源时被调用，不处理外部/用户输入，无法被远程触发；
  3. 属 dev 依赖，不进任何运行时产物。
- **正解（依赖上游）**：待 uni-app 工具链升级到基于 `jimp@1.6.x` 的版本（该版把图片解析拆为 `@jimp/js-*` 独立插件，不再依赖老 file-type），漏洞自然消失。当前 uni-app 为 `3.0.0-alpha` 预发布版，不宜为此单独魔改上游工具链。
- **决策记录**：2026-09-22 在 Dependabot 标记为 "risk is tolerable" / 登记为已接受技术债。对比项 #126（uuid）已于同日 override 修复。

---

## TD-3 chromadb 1.5.9 四条服务端漏洞，上游无补丁（嵌入式用法不可达）

- **登记日**：2026-10-24
- **告警**：`pip-audit -r augmentor/requirements.txt` 报 chromadb 1.5.9 四条：PYSEC-2026-311（预认证代码注入 RCE）、PYSEC-2026-3814（认证后代码注入）、PYSEC-2026-3815（SimpleRBAC 鉴权缺陷）、PYSEC-2026-3813（缺授权校验可任意读写）。
- **为何无法升级修复**：1.5.9 已是 PyPI 最新版（`pip-audit` 的 Fix Versions 为空，crates/PyPI 均无更高修补版）——属**上游无修复**，与 cargo 侧 rsa/lopdf/zip 同类。
- **为何判为不可达（补偿控制）**：augmentor 只用**嵌入式** `chromadb.PersistentClient(path=...)`（`augmentor/vector/chromadb.py:80`，`semantic.py` 注释「自管落盘」），**从不**起 chromadb HTTP server（无 `HttpClient` / `chromadb run` / 监听端口）。四条漏洞全是 chromadb **服务端** API 的预认证/越权/RCE，需要暴露的网络服务与鉴权边界才触达；单进程本地嵌入式用法下不可达。
- **处置**：登记为已接受技术债（不等同于「安全」）。待 chromadb 发布修补版即升；若将来改用 `HttpClient` 连远端/共享 chromadb server，必须重新评估（届时可达）。
- **同期已修（对照）**：本轮 dependabot 清理中，cargo 侧 rustls/webpki-roots/rustls-webpki/ctutils 已升到修补版（清 7 条），zip/rsa/lopdf 按既有 rustsec-exceptions.toml 登记例外；npm augmentor/web 的 @vitest/mocker 已升 5.0.5（清 2 条）；pnpm oa4rust 的 postcss-selector-parser 已覆盖到 7.1.6（清 1 条）。
