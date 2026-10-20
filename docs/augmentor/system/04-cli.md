# 04 · 命令行端 / CLI（`cli.py` + `augmentor/cli/`）

> CLI 是核心引擎的**第三个薄适配层**（与 REST、Web 并列），面向脚本、CI、批处理
> 与离线环境。它不实现能力，只把能力组织成 **37 个子命令**，统一入口 `python cli.py <cmd> [选项]`。

---

## 1. 整体架构

```
python cli.py <subcommand> [args]
        │
        ▼
cli.py::main()
   ├─ harden_stdio()          把 stdout/stderr 编码处理器换成 replace（L56：emoji/编码事故不影响跑完）
   ├─ build_parser()          argparse（全部子命令与参数定义在 augmentor/cli/parser.py）
   ├─ COMMANDS[cmd]           查分发表（augmentor/cli/commands/__init__.py，37 项）
   └─ load_config(args.config) + handler(args, config)   统一 try/except → 「错误: …」+ 退出码 1
```

**薄壳原则**：`cli.py` 只做「兜 stdio 编码 → 解析 → 查分发表 → 调 handler → 统一异常处理」。
每个 handler 签名统一为 `(args, config) -> None`，按领域分 9 组住在 `augmentor/cli/commands/`。

---

## 2. 模块划分

```
cli.py                              入口薄壳（兜 stdio → 解析 → 分发表 → handler）
augmentor/cli/
├── parser.py                       build_parser()：全部 argparse 定义 + 格式/规则常量表
├── io.py                           4 个共用 I/O helper（含 harden_stdio）
├── verdict.py                      verdict_exit()：退出码的唯一出口（0/1/2）
└── commands/                      37 个 handler，按领域 9 组
    ├── profiling.py   profile / outliers / features / auto-config / aggregate
    ├── pipeline.py    augment / stream
    ├── quality.py     quality / quality-report / clean / annotate / rag / benchmark / health-gate
    ├── export.py      preview / export
    ├── analysis.py    analyze / stats / visualize / compare
    ├── data_ops.py    merge / sample / split / validate / convert / validate-config / search
    ├── version.py     version / backup
    ├── security.py    sanitize / check-leakage / audit
    └── ops.py         doctor / auto-test / monitor / dependency / migrate
```

### 2.1 37 个子命令速查（按领域）

| 组 | 命令 | 作用（部分含门禁判定） |
|----|------|------------------------|
| **profiling** | `profile` `outliers` `features` `auto-config` `aggregate` | 画像 / 长度离群 / 特征维度 / 推荐管线参数 / 多源聚合 |
| **pipeline** | `augment` `stream` | LLM 增强（含断点） / 流式处理 |
| **quality** | `quality` `quality-report` `clean` `annotate` `rag` `benchmark` `health-gate` | 评估筛数据 / 出报告 / 清洗 / 自动标注 / RAG 格式 / 基准 / 健康门禁 |
| **export** | `preview` `export` | 预览转换 / 多格式导出 |
| **analysis** | `analyze` `stats` `visualize` `compare` | 洞察 / 逐字段统计 / 可视化 / A/B 对比 |
| **data_ops** | `merge` `sample` `split` `validate` `convert` `validate-config` `search` | 合并 / 采样 / 分割 / 数据集校验 / 格式转换 / 配置校验 / 检索 |
| **version** | `version` `backup` | 版本快照/回滚/历史 / 备份 create·list·restore·delete·verify·cleanup |
| **security** | `sanitize` `check-leakage` `audit` | PII 脱敏 / 训测泄漏 / 就绪审计 |
| **ops** | `doctor` `auto-test` `monitor` `dependency` `migrate` | 依赖诊断 / 自动测试 / 质量监控 / 依赖登记 / 数据迁移 |

> 命令的 help 文案**权威出处是 `python cli.py <cmd> --help`**（`parser.py` 现提）；
> 各命令示例详见仓内 [`docs/augmentor/README.md § CLI`](../README.md)。

---

## 3. 退出码约定（`verdict.py` 是唯一出口）

退出码只有 **三种形状**，彼此不许互相顶替：

| 退出码 | 含义 | 来源 |
|:------:|------|------|
| `0` | 判决通过，或该命令压根不判负（报告形命令默认档） | 正常结束 |
| `1` | 判决未通过 **或** handler 抛异常被 `main()` 翻译成 `错误: …` | `verdict_exit()` / `main()` |
| `2` | argparse 用法错误（参数不认识 / 缺必填 / 传旧命令名） | 解析器 |

**关键区分**：同样是 `1`，「判决判负」与「崩溃」**靠 stderr 有无 `错误:` 来分**——
判负是命令正常完成但结论为负；崩溃是 handler 抛了异常。

### 3.1 校验形 vs 报告形（`--gate` 语义）

- **校验形**（`validate` / `validate-config` / `dependency --action validate` / `health-gate`）：判负即 `1`，无 `--gate` 开关。
- **报告形**（`quality-report` / `audit` / `check-leakage` / `doctor` / `auto-test` / `migrate`）：
  **默认恒 0**（审计报告的本分是有问题也照出报告），加 `--gate` 才把判决位翻译成退出码。

哪些命令接了这套口径由 `tests/integration/test_cli_verdict_wiring.py` 从
**parser 声明与 handler 源码双向推导**对账（声明 `--gate` 的命令 == handler 真传 `enforce=args.gate` 的），
**不抄清单**。命令面与全表相等由 `tests/unit/test_readme_cli_index_l204.py` 机器守门。

---

## 4. 输出契约（stdout 保持机器可读）

报告型命令的 **stdout 是机器可读**的，`--output` 只负责把同一份内容落盘，提示语写 stderr、不污染 stdout：

| 命令 | stdout 形状 |
|------|-------------|
| `clean` / `analyze` / `stats` | JSON |
| `compare` | Markdown 摘要 |
| `search` | 两行摘要 + 条目 JSON 列表 |
| 多数数据命令 | 人类可读摘要 / JSON（视命令而定） |

**编码不再参与退出码**（L56）：CLI 入口把 stdout/stderr 处理器从 `strict` 换成 `replace`，
产品侧文本读写显式带 `encoding=`。用户数据含 emoji 或当前 locale 编不出的字符时命令照常跑完，
该字符落成 `?`、JSON 输出仍可 `json.loads`。上面那句「`1` = 崩溃」从此只对应真异常。

---

## 5. 与后端的交互关系

CLI **不经过 REST**，直接 import 核心引擎（`from augmentor import ...`）读本地文件、调 LLM：

```
cli.py ──(import)──► augmentor/pipeline.py 等能力模块 ──► 本地 JSON / 模型后端 / 向量库
```

因此 CLI 与 API 是**平行的两个驱动入口**，共享同一套引擎行为。
差异点（运维须知）：

- **默认注册表/备份目录**：CLI 与 API 的默认依赖注册表目录**不是同一处**
  （API 跟数据白名单首根走，SDK/CLI 默认相对工作目录）。要共用请显式传 `--registry-path`。
- **退出码可作 CI 门禁**：报告形命令加 `--gate` 后可直接进 CI（判负即 `1`）。
- 详细的端间流程见 [`05-interactions-and-dataflow.md`](05-interactions-and-dataflow.md)。

---

## 6. 典型用法（最小示例）

```bash
# 增强（默认开质量检查 + 去重 + 断点）
python cli.py augment --input train_data.json --output train_data_augmented.json

# 多格式导出
python cli.py export --input train_data.json --output-dir exports --formats jsonl alpaca sharegpt

# 质量报告 + CI 门禁
python cli.py quality-report --input train_data.json --threshold 0.8 --gate

# 数据安全三连
python cli.py sanitize      --input raw.json --output masked.json --fields instruction output
python cli.py check-leakage --train train.json --test test.json --gate
python cli.py audit         --input data.json --gate

# 版本管理
python cli.py version --action create --input data.json --description "初始版本"
python cli.py version --action rollback --version v1.0.0
```
