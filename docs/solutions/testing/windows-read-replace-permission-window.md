---
title: "Windows 读侧撞上 os.replace 的 PermissionError 窗口：先补定向探针，不急着改产品（A356① / L298）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: testing
module: augmentor/api/deps.py + augmentor/api/routes/data.py
problem_type: platform_specific_transient_failure
component: windows_read_replace_window
severity: medium
symptoms:
  - "并发写数据集时，GET /api/data/load 偶发 500，日志里是 read_items 的 open() 抛 PermissionError [Errno 13]"
  - "同一份代码在 POSIX 上不复现；Windows 的 os.replace 与打开着的读句柄互斥"
  - "响应体只有固定文案 INTERNAL_ERROR_DETAIL，原文只进日志；窗口过后同一请求转绿，失败没有被缓存成「文件坏了」"
root_cause: read_side_has_no_lock_or_retry_while_write_side_replaces_atomically
resolution_type: test_only_probe
tags: [windows, permissionerror, eacces, os-replace, movefileexw, atomic-write, read-write-race, probe, skipif, ast-guard]
applies_when:
  - "Windows 上并发读写同一份 JSON / 配置文件，写侧用 os.replace 原子替换"
  - "读侧是裸 open()，没有锁也没有重试"
  - "想把一个已知的瞬时失败窗口钉成判据，而不是马上改延迟 / 吞吐 / 状态码"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/data.py
  - augmentor/augmentor/atomic_write.py
  - augmentor/tests/integration/test_read_replace_collision_l298.py
---

# Windows 读侧撞上 os.replace 的 PermissionError 窗口

## 现象

L273 做「丢更新」那一轮时，改前读数里出现过 3 支 500：50 支并发 `PUT /api/data/update` 全部打到同一份数据集，失败的 3 支调用栈都停在 `api/deps.py::read_items` 的 `open()` 上，异常是 `PermissionError [Errno 13]`。当时的现场是**另一支请求正在 `os.replace` 同一份文件**。

这不是逻辑错误，而是 Windows 的平台语义：

- `os.replace` 走 `MoveFileExW(REPLACE_EXISTING)`，目标有未共享删除的打开句柄时，替换被拒绝，Python 抛 `PermissionError(EACCES)`。
- POSIX 的 `rename` 不因打开着的句柄失败，所以同一份代码在 Linux / macOS 上不复现。
- 窗口极短，但并发足够密时会出现；客户端看到的是 **500 + 固定文案** `INTERNAL_ERROR_DETAIL`，原文只进服务端日志。

L273 只把**写侧**的读-改-写整段罩进了 `path_lock`，读侧与替换之间的窗口没有关，也没有为它写探针。A356 立账时把这一格记成「已知形状、候选待拍」。

## 根因

**写侧原子，不等于读侧与写侧之间原子。**

- 写侧：`mutate_json_file` 把「读 → 改 → 写」整段放进 `path_lock`，落盘走 `atomic_write_json` 的 `os.replace`；替换失败时目标保持原内容、临时件被清掉。
- 读侧：`read_json_file` → `_read_items_cached` → `read_items`。`_read_items_cached` 先 `stat()` 取 `mtime_ns + size` 做缓存键，未命中就调 `read_items`；`read_items` 是**裸 `open()`**，不取 `path_lock`，也没有重试循环。
- 因此，GET 的读与写侧的 `os.replace` 之间仍有一个窗口：替换那一瞬读侧拿 `EACCES`，路由的兜底把它折成 500。

`path_lock` 还是**进程内**的 `threading.RLock`（A355），所以即使读侧将来取锁，也只覆盖同进程；跨进程那一维仍要单独拍。A356 的候选②（读侧有限重试）与候选③（瞬时 OS 失败算 500 还是 503 / 429）都会改延迟、吞吐或对外状态码契约，不能在「只补尺子」这一轮里顺手做。

## 处置

本轮按用户拍板的候选① 执行：**产品码零改动，只补定向探针**。

新增 `augmentor/tests/integration/test_read_replace_collision_l298.py`，6 支 / 零 `parametrize`，三面六格。

### 一、机制面：两格，Windows 专属

1. **真实机制臂**：读者持有目标文件的 `open()` 句柄时，`os.replace` 到同一目标必拿 `PermissionError(EACCES)`；句柄关掉后，同一份临时件照常换上。这一格是 Windows「读句柄挡住替换」的精确形状，确定性复现。
2. **同形替身臂**：用 `msvcrt.locking` 锁住目标文件的字节范围，另一支裸 `open()` 当场拿 `PermissionError(13)`。它把「这一瞬读不到」变成可持有的状态，与 L273 观测到的读侧异常**同一个 Python 形状**（同为 `EACCES`）。

docstring 明确写了：替身不是 `MoveFileExW` 本身，不能把它当成机制证明。两条机制臂都在 `os.name != "nt"` 时 skip，因为 POSIX 的 `rename` 不因打开着的句柄失败。

### 二、产品面：两格，真跑 HTTP

1. `GET /api/data/load/{filename}` 在拒绝窗口里回 **500 + `INTERNAL_ERROR_DETAIL`**；响应体不含 `Permission` / `Errno 13` / 临时路径；日志里确有对应的 `PermissionError`（`exc_info` 逐支验）；窗口解除后同一请求回 200 并拿回原数据。
2. `PUT /api/data/update/{filename}` 在读者持有句柄时回 **500 + 固定文案**；失败后盘上仍是进入时的内容，零 `.tmp-` 残留；关掉句柄后同一请求成功，改的是第 0 条、其余条目原样。

这两格钉住的是**失败语义**，不只是状态码：GET 的失败没有被缓存成「这份文件坏了」，PUT 的失败没有撕开原子写的「失败保持原样」。

### 三、结构面：两格，平台无关

1. AST 钉住 `read_items` 仍是裸 `open`（无 `path_lock`、无重试循环）、`_read_items_cached` 不取锁；对照写侧 `mutate_json_file._read_modify_write` 的读-改-写整段确实在 `path_lock` 内，且 GET 的异步入口走 `to_thread`。
2. 一支合成牙齿臂证明尺子认得出 `path_lock` 与重试循环，避免「查不到是因为不会查」。

**这一面是设计成会红的**：谁给读侧加锁或重试，结构臂会红，提醒把 A356 与对外语义（延迟 / 吞吐 / 状态码）一起更新。红是提醒，不是回归。

## 守卫与证据

- 守卫：`augmentor/tests/integration/test_read_replace_collision_l298.py`，6 支，零 `parametrize`。
- 定向复跑：`python -m pytest tests/integration/test_read_replace_collision_l298.py -q --no-cov` ⇒ **6 passed**（0.65 s）。
- 两臂注入取证（逐字节复原，`api/deps.py` sha256 `19BEDDAB0E7E3AA7…`、`api/routes/data.py` sha256 `6DDE85AF7FFAD58F…` 前后一致）：
  - 给 `read_items` 套上 `with path_lock(...)` ⇒ `1 failed, 5 passed`（点名结构臂）。
  - 把 `load_data` 的 500 兜底换成 `detail=str(e)` ⇒ `1 failed, 5 passed`（点名 GET 产品臂，回执逐字变成 `[Errno 13] Permission denied`）。
- 全量定标：`python -m pytest tests -q -p no:cacheprovider` ⇒ `37 failed, 10267 passed, 6 skipped, 6 warnings, 438.50s`，覆盖率 `99.62%`。37 项红全部落在 L297 批次后的既有基线族（l210 2、l224 8、l233 4、l252 1、a184 6、l251 1、l200 5、l265 2、memory monitor 1、l237 7），本轮未引入新红，也未扩大修复范围。

对外可观测面零变更：状态码、回执形状、产物字节、落盘路径一字未动，本轮只把 A356 的已知窗口记录成判据。

## 教训

1. **平台特定的瞬时失败要分三层量**：真实机制臂（证明 OS 语义）、同形替身臂（把窗口做成可持有状态）、产品臂（证明路由怎么折它）。只 mock 一个异常，量不到「读侧与替换真的互斥」。
2. **同形替身必须自报身份**：`msvcrt.locking` 不是 `MoveFileExW`，docstring 要写明，否则下一轮会把它当成机制证明。
3. **探针可以钉「当前行为」**：结构臂钉住「读侧没有锁也没有重试」，行为一变就红；这时红是提醒更新账本与对外语义，不是回归。把这句话写进测试 docstring，后来人才不会顺手把红改绿。
4. **失败不缓存是恢复语义的一部分**：GET 在窗口过后拿到原数据，PUT 失败后盘上仍是旧内容、无临时件残留——这两格比「回 500」更接近用户真正关心的事。
5. **先补尺子再改产品**：状态码与延迟契约未拍板时，先把已知窗口钉住；否则「修了一个窗口」很容易在下一轮引入另一个。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L298 清单行与日志块，A356 行
- 提交：`b2e55f440`（`test,docs(augmentor): L298 A356① 读侧撞 os.replace 定向探针`）
- 守卫：`augmentor/tests/integration/test_read_replace_collision_l298.py`
- 同族：L273 / A355（`path_lock` 只管同进程）、A356（本格）
- 产品：`augmentor/api/deps.py`、`augmentor/api/routes/data.py`、`augmentor/augmentor/atomic_write.py`
