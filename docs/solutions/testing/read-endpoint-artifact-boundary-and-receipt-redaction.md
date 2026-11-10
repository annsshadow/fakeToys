---
title: "只读端点的副作用落点与回执脱敏：产物落进白名单闸内、校验回执不再回显服务器绝对落点（A407 / A398 / A363 / L304）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/api/deps.py + augmentor/api/routes/data.py + augmentor/api/routes/system_ops.py + augmentor/augmentor/pipeline.py
problem_type: read_endpoint_side_effect_and_path_leak
component: artifact_boundary_and_response_redaction
severity: medium
symptoms:
  - "GET /api/visualize/{filename} 名义上是读，实际在进程当前工作目录下建目录并写 PNG；落点随启动目录漂移，也在数据白名单之外"
  - "并发两次可视化请求会把共享单例上那只 visualizer 的落盘目录改掉，最后写的那一支赢"
  - "POST /api/system/validate-config 的 200 回执把 PyYAML 异常里的解析后绝对落点原样回显，客户端拿到的是服务器目录结构"
root_cause: read_endpoint_reuses_relative_output_dir_and_error_receipt_echoes_server_absolute_path
resolution_type: whitelist_anchor_output_dir_and_receipt_redaction
tags: [side-effect, path-leak, redaction, whitelist, output-dir, copy-semantics, shared-singleton, read-only-endpoint, load-bearing-test]
applies_when:
  - "名义上是读的端点却在写盘，且落点用相对路径（跟着进程 CWD 漂）"
  - "多个请求共享同一个带可变状态的对象（visualizer / scorer / 客户端）"
  - "第三方库的异常消息里带着服务端绝对路径，而这段消息要原样进响应体"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/data.py
  - augmentor/api/routes/system_ops.py
  - augmentor/augmentor/pipeline.py
  - augmentor/tests/unit/test_artifact_boundaries_l304.py
---

# 只读端点的副作用落点与回执脱敏

## 现象

A407（L294 立）与 A398（L290 立）量到两件形状不同、根子相近的事：

1. **只读端点留了写盘痕迹，且落点跟着 CWD 漂**。
   `GET /api/visualize/{filename}` 走 `AugmentorPipeline.visualize_dataset` 的默认参数
   `"visualizations"`，于是每次 GET 都在**进程当前工作目录**建目录并写 PNG。落点随启动目录
   漂移，也在数据白名单之外——同一个请求在不同的工作目录下落在不同地方，而那两处都不是
   闸内。

2. **共享单例上的可变落点被每次调用搬走**（A363）。
   `visualize_dataset` 直接写 `self.visualizer.output_dir`。并发两次调用时最后写的那一支
   赢，于是「这次请求的产物落点」会被别的请求改掉。

3. **回执把服务端绝对落点回显给客户端**（A398）。
   PyYAML 的 `str(exc)` 会把打开文件时用的路径原样写进 message
   （`in "C:\\...\\config.yaml", line 1, column 9`），于是 200 的校验回执成了
   「服务器目录长什么样」的读数。**文件内容不外泄**——外泄的只是路径。

## 判定与改法

用户 2026-11-10 拍板：A407 走「落闸内 + 收 A363」，A398 走「只脱敏、不改鉴权」。

### A407：落点跟白名单首根走

新增 `deps.default_visualization_dir()`，与 `default_registry_dir` / `default_backup_dir`
同构——白名单**首个根目录**下的 `visualizations`：

```python
def default_visualization_dir() -> Path:
    return _default_root_subdir("visualizations")
```

端点**显式**把该目录传给管道（不传就等于默认走相对路径），回执由 `_public_chart_paths()`
折回 `visualizations/<名>` 的相对形：闸内路径按「目录名 / 相对名」返回，万一有产物落到目录
外，只回文件名——**任何一支都不把服务器绝对路径写进响应**。

为什么不新增配置键：这条落点的形状已经由「白名单首根 + 固定子目录」决定，加一个可配键
等于把「产物该落哪」变成一件要人配的事，而闸内的约定本来就该由闸自己给。

### A363：改局部视图，不搬共享单例

```python
visualizer = copy.copy(self.visualizer)
visualizer.output_dir = Path(output_dir)
return visualizer.generate_all_visualizations(items)
```

直改共享单例会让并发请求把落点互相搬走（最后写的那一支赢），也会把调用方的默认值永久
改掉。浅拷贝保留单例上已加载的 wordcloud / matplotlib 引用与调用方注入的替身方法，代价只有
一次对象复制。

### A398：只在回执层脱敏

`system_ops._redact_path()` 只改 message 文本：把服务端绝对落点替换成**调用方给的原字符串**
（缺省时用文件名），行列号、`path` 逻辑键（`yaml` / `file`）与其余诊断一字不动。

鉴权档位**不变**：这一格挂在只读白名单上（无 `verify_api_key`），本轮不碰——用户拍板的是
「只脱敏，不改鉴权」，把鉴权一起改会让这一格变成两件事（对外契约变更），不在本轮范围。

## 守卫

`augmentor/tests/unit/test_artifact_boundaries_l304.py`（**14 支 / 零 parametrize**）：

- A407 六支：默认目录跟白名单首根 / 端点显式传目录 / 产物落根下且 CWD 零痕迹 /
  回执只给相对形 / 两种折回形状 / **折回是承重的**；
- A363 两支：两次不同目录的调用后单例仍是原值 / 调用时看到的目录是本次入参；
- A398 五支：保住行列号丢掉根目录 / 缺省 path 用裸文件名 / **脱敏是承重的** /
  端点不挂 `verify_api_key`（静态读依赖）/ 设了 `AUGMENTOR_API_KEY` 仍无头可过（行为双证）。

三条设计要点值得单独记：

1. **白名单根与 CWD 分开建**（`tmp/data` 与 `tmp/cwd`）。合在一起建的话，「产物落在根下」
   与「什么也没落到 CWD」两支会互相掩护——一个跑到另一个的目录里也照样绿。
2. **两支承重臂**：把 `_public_chart_paths` / `_redact_path` 换成恒等函数，绝对落点必须立刻
   出现在回执里。没有这两支，上面那几支可能只是空转。
3. **不用真出图**：当前环境无 matplotlib / wordcloud ⇒ 用「记录器可视化器」替身记录
   `output_dir`。`copy.copy` 与单例共享 `__dict__` 里的列表引用，因此局部视图在调用时看到的
   `output_dir` 能被记下来。

## 真实返工：JSON 反斜杠转义造成的假阴

「折回承重」那一支首跑红了。红格是断言写成

```python
assert str(root) in json.dumps(payload)
```

JSON 把 `\` 转义成 `\\`，于是子串比对**假阴**——而这一支正是「回执不出现绝对路径」的反证
臂，假阴等于该臂没在工作。改成直接读值（`response.json()["charts"]["wordcloud"]`）后
**14 passed**（9.68 s）。

## 行号位移：内容守恒重钉

产品码在四只文件**中部**插行（`deps.py` +12 / `data.py` +1 起算 / `system_ops.py` +1 与
+30 两段 / `pipeline.py` +1），把全仓在册定位引用顶到别的行 ⇒ a184 硬 0 档当场报出 **21 处
新增 `dead_line`**（卷一 1、卷二 4、`web/src/schemas/full-schemas.ts` 16）。

处置按 A127 / L180 / L193 既有口径逐格**按现量重钉**（不是降级名锚——这些引用指的是本轮改动
后仍然存在的代码）。映射用**内容守恒**：`difflib` 的 `equal` 块逐行对位（含空行），并逐行
断言「旧行内容 == 新行内容」，错配即当场炸。共重钉 **57 行 / 8 个文件**，零未解析。

**两处按语义特判**（内容守恒对它们不适用，因为旧值本就指向空行／注释行）：

- `api/deps.py` 那三处旧引用指向空行（死引用），散文说的是「分派表」⇒ 钉到
  `def to_http_error(` 的现量行；
- A368 那一格指的是 `augment_async` 的收尾落盘，旧值落在「保存输出」那段注释上 ⇒ 钉到
  `atomic_write_json` 的现量行。

重钉后 a184 `dead_line` 39 → **35**、`dead_anchor` 5 → **3**（A407 行那两处的内容维红格
随之消失），l79 的 `dead_line` 从 1 红回到 **0 红**；按「同一引用换行号」归一后逐格对照为
**新增 0 格、修好 5 格**。

## 余面与教训

- **本轮不动鉴权**：A398 的「只读白名单、无 `verify_api_key`」这一维原样保留，只脱敏。
  谁要把它改成写面档位，那是另一族对外契约变更。
- **回执折回不是「永远相对」**：闸外产物只回文件名（不写绝对路径），但也不假装它在闸内。
  折回函数对闸外情形有专门的分支，不要把它简化成「一律拼目录名」。
- **写档动作本身会造新账**：本轮日志块最初写了两处 `api/deps.py:511`，那三格旧值指向空行
  ⇒ 记下旧值这个动作自己把引用写成了新的死引用，被 a184 当场抓住。按仓库既有先例改成
  「不抄那个形状」，没有扩大任何排除面。
- **l177 的语料计数欠账**：L303 新增 `test_backlog_claim_roster_l303.py` 时没把 l177 的测试面
  计数（438）重钉 ⇒ HEAD 态实测已 439 ≠ 438（红）。本轮再新增一支守卫 ⇒ 现量 440，按现量
  重钉为 440（那一格只钉总数，不钉分解）。
- **全量基线**：本轮不跑全量；L301 的基线仍是 `37 failed / 10272 passed / 6 skipped /
  99.62%`，37 项红全部落在既有基线族，不声称全绿。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L304 清单行与日志块、A407 / A398 / A363 行
- 守卫：`augmentor/tests/unit/test_artifact_boundaries_l304.py`（14 支 / 零 parametrize）
- 复用：`api/deps.py` 的 `default_registry_dir` / `default_backup_dir` 同构形状；
  `augmentor/api/routes/data.py` 的 `_safe_data_path` 闸内解析口径
- 同族：A307（两支只读端点把注册表目录建出来）、A362 / A364（`augment_async` 的断点族）、
  A341（`api/deps.py` 的写面键位收口）
