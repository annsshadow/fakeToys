# augmentor FAQ（3.0.0）

面向「用的时候卡住了」的五个高频问题。部署（Docker / 端口 / 卷 / 反向代理）见
`docs/augmentor/DEPLOYMENT.md`（§1 本地、§2 Docker、§3 服务器、§5 排查），本文件不重复运维文档。

**锚点约定**：`§x.y` 指 augmentor/OPTIMIZATION_PLAN.md 的对应小节；`文件:行号` 指产品代码的当前落点。
行号会随代码搬家 —— 本仓有一把守卫专盯文档与注释里的定位引用
（`augmentor/tests/unit/test_comment_refs_a184.py`），改了代码不更新引用会让它变红。

---

## 1. 五个模型后端怎么选？一家挂了会自动换另一家吗？

**不会自动换。** 一次请求只打向你配置里选中的那一个后端，失败只在该后端内部重试；
不存在「Claude 失败了自动切 OpenAI」这种跨厂商降级链。

- **五家是「类型」，不是模型名**：`baidu` / `openai` / `ollama` / `claude` / `gemini`
  （`augmentor/config.py:92`）。出厂默认模型叫 `ernie`（`augmentor/config.py:841`），
  但 `ernie` **不是**一个后端类型 —— 它是默认条目的名字，那条条目的 `type` 写的是 `baidu`；
  把 `type` 写成 `ernie` 会被工厂拒绝（`augmentor/models/factory.py:110`）。
- **选型只有两条路**：改配置里的 `models.default`，或调用方显式传 `model_type`。
  工厂是一张「类型 → 类」的表（`augmentor/models/factory.py:21`），查不到就抛
  `ConfigError`（`augmentor/models/factory.py:110`）——**不会静默回落到某一家**。
- **重试不是降级**：默认最多尝试 3 次（`augmentor/models/base.py:75`），
  配置项是 `augmentation.max_retries`（默认 3，`augmentor/config.py:254`）。
  它重试的是同一个后端；`Retry-After` 只控制等多久，不换目标。
- **换模型不会命中旧缓存**：缓存键覆盖 provider / model / 采样参数 / prompt
  （`augmentor/models/base.py:278`），不是「只拿 prompt 当键」。
- **响应缓存默认是关的**：`response_cache_dir=None` 即不启用，因为把 prompt 与响应
  写盘属于需要显式选择的行为（`augmentor/models/base.py:97`）。开启后是磁盘
  `DiskCache`（`augmentor/cache.py:212`），容量上限 256 MB
  （`augmentor/models/base.py:93`）—— 没有上限的磁盘缓存只是把内存泄漏换成磁盘泄漏。

> 计划文档里曾把这条写成「多模型降级顺序」，是勘察前的措辞；按实现订正为
> 「单后端 + 同后端内重试」，订正记录见 docs/plans 下 2026-10-26-r4 计划的 §8。

---

## 2. 13 种导出格式有什么区别？为什么 `--format alpaca` 写出来是 `.jsonl`？

因为**「文件容器」和「行内 schema」是两层**，13 种格式里只有 4 种决定文件怎么摆。

- **权威清单是 `ExportFormat` 枚举**（`augmentor/export_enhanced.py:29`），迭代产出 **13 种**
  （含 `raw`）。`SHARE_GPT` 与 `SHAREGPT` 是**同一个值**（`augmentor/export_enhanced.py:53`），
  它是历史拼写的别名，不是第 14 种。
- **容器只有四个**：`json` / `jsonl` / `csv` / `tsv`（`augmentor/converter.py:44`），
  它们决定物理布局（选哪个解析器）。`alpaca` / `sharegpt` / `chatml` / `llama_factory` /
  `vicuna` / `belle` 只是**行内 schema**：同一份 schema 既能落成 JSON 数组，也能一行一条。
  所以 `--format alpaca` 输出 `.jsonl` 时是一行一条，并且能原样读回
  （OPTIMIZATION_LOOP.md 的 L20 条目）。
- **反向边是后来补的**：`alpaca` / `belle` / `llama_factory` / `sharegpt` / `vicuna` / `chatml`
  → `json` 这 6 条边由 L16 补齐，CLI 的 `--input-format` 与 API 的 `source_format` 同批接线
  （OPTIMIZATION_LOOP.md 的 L16 条目）。此前转换图是单向的。
- **Excel 只进不出 `.xls`**：读边认 `excel` / `xlsx` / `xls` 三种拼法
  （`augmentor/converter.py:51`）；写边只认 `excel` / `xlsx`（`augmentor/converter.py:56`），
  `.xls` 是**点名拒收**的（`augmentor/converter.py:59`）—— 写真正的 BIFF 老格式要额外依赖，
  宁可不收，也不产出一份「扩展名是 xls、字节是 zip」的假容器。

---

## 3. 脱敏 / 泄漏检测 / 就绪审计，什么时候会自动跑？

**都不会自动跑。** 这三件是独立端点，由调用方（页面按钮或 SDK）显式触发；
上传、增强、导出主链路里没有任何一步会顺手做它们。

| 能力 | 触发面 | 锚点 |
|---|---|---|
| PII 脱敏 | `POST /api/privacy/sanitize` | `augmentor/api/routes/privacy.py:48` |
| PII 模式清单 | `GET /api/privacy/patterns` | `augmentor/api/routes/privacy.py:74` |
| 训练/测试集泄漏检测 | `POST /api/leakage/check` | `augmentor/api/routes/leakage.py:44` |
| 数据集就绪审计 | `POST /api/audit` | `augmentor/api/routes/audit.py:27` |

三点容易踩的细节：

- **就绪审计的参考集是可选的**：不传 `reference_file` 就只做单集审计，
  传了才做训练/测试对比（`augmentor/api/routes/audit.py:32`）。
- **审计内部会复用另外两件的判据**：它自己会跑一遍 PII 扫描与泄漏检测，把结果并进组合信号
  （`augmentor/audit.py:98`、`augmentor/audit.py:134`）——但触发点仍然只有「你调了审计」这一个，
  不是主链路顺手做的。
- **参数错是 400，不是 500**：SDK 抛 `DataValidationError` 时走边界转换
  （`augmentor/api/routes/leakage.py:70`、`augmentor/api/routes/audit.py:51`），
  把「请求方的错」如实回成 400；只有未分类异常才收敛成 500。
  这直接影响你读日志：看到 500 优先怀疑服务端，看到 400 先看自己传的参数。

---

## 4. 为什么裸文件名还能读，写却必须写 data/xxx.json？

因为读、写两侧**故意不对称**：读会在白名单各根目录里找，写不猜目录。

- **出厂白名单只有 `data`**：`web.data_roots = ["data"]`（`augmentor/config.py:723`），
  原默认是 `["."]`（放行整个工作目录），后按 F-07 收紧；代价与迁移处方写在
  OPTIMIZATION_PLAN.md §3.2。
- **读**：裸文件名会在白名单各根目录内查找，命中不了才按工作目录解释
  （`augmentor/api/deps.py:394`）；越界的符号链接仍会被拦下。
- **写**：必须显式给白名单内的路径，例如 data/out.json；服务端不替文件选目录
  （`augmentor/api/deps.py:442`）。
- **环境变量优先**：`AUGMENTOR_DATA_ROOTS`（`os.pathsep` 分隔）优先于配置文件
  （`augmentor/api/deps.py:166`）。

所以「列表里看得见、读得到，但一写就 403」通常不是 bug，而是写路径没带 `data/` 前缀。

---

## 5. 现在还慢在哪？文档里那些性能数字是多少？

下面三个是本仓实测过的**峰值 / 端到端**数字，不是目标值，也不是估算：

| 场景 | 实测 | 出处 |
|---|---|---|
| dedup 的 N×N 相似度矩阵内存 | 峰值 **9536 MB → 20 MB** | OPTIMIZATION_PLAN.md §2.2 第 1 条 |
| 伪流式（先全量读进内存再分块）改真流式 | 峰值 **22.70 MB → 1.66 MB** | OPTIMIZATION_PLAN.md §2.2 第 3 条 |
| 倒排索引按需构建（真实 6902 条默认 contains 查询） | **90.30 → 7.96 ms（11.34×）**，regex 19.41×，单次查询峰值 6.30 MB → 0.13 MB | OPTIMIZATION_LOOP.md 的 L13 条目 |

读这些数字要注意口径：它们是**当时那台机器、那份语料**上的读数，比较的是同一进程内
前后两种实现；换机器或换语料绝对值会变，但「数量级差」是判据要保的东西。

---

## 本文件不写什么

- **部署与运维**：镜像、端口、卷、反向代理 —— `docs/augmentor/DEPLOYMENT.md` 已有，
  重复写会漂移。
- **指不回锚点的数字**：宁可不写，也不写「约」。文档漂移在本仓按缺陷处理，
  FAQ 是新文档，出生就带锚点。

## 附：部署问题去哪查（只给索引，不复制内容）

| 你遇到的问题 | 看 `docs/augmentor/DEPLOYMENT.md` |
|---|---|
| 本机起服务 / 只起前端 / 后台常驻 | §1（本地部署） |
| Docker 构建、数据卷、健康检查、升级与停止 | §2（Docker 部署） |
| systemd、Nginx 反向代理、HTTPS、定时备份 | §3（服务器部署） |
| 环境变量与模型密钥从哪注入 | §4（环境变量清单，只有键名） |
| 起不来、日志在哪、备份恢复、安全基线 | §5（运维与排查） |

值一律由部署环境注入（`.env` 不入仓）；本表只指路，不写任何环境真值。
