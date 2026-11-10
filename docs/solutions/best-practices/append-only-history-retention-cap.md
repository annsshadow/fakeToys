---
title: "只追加的账也要有保留条数上限：写侧封顶 + 两份孪生一起治（A399 / L306）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: best-practices
module: augmentor/augmentor/version_control.py + augmentor/augmentor/versioning.py
problem_type: unbounded_append_only_log
component: history_jsonl_retention_cap
severity: medium
symptoms:
  - "history.jsonl 只以 'a' 追加、从不轮转封顶 ⇒ 行数线性放大磁盘"
  - "get_history 全量读进内存再切片 ⇒ limit 只省返回体积、不省读取"
  - "同一形状的账在两个模块里各有一份（CLI 走前者、pipeline 走后者）"
root_cause: append_only_jsonl_has_no_retention_cap
resolution_type: retention_cap_config_key_with_delete_old
tags: [retention, cap, jsonl, append-only, history, twin-implementations, delete-old, locking, bounded-read]
applies_when:
  - "有一份只追加的 JSONL 账（操作历史 / 审计 / 事件流），写侧从不封顶"
  - "读侧按 limit 切片，但 limit 只省返回体积、不省读取"
  - "同一形状的实现在两个模块里各有一份（一条走 CLI、一条走服务）"
related_components:
  - augmentor/augmentor/version_control.py
  - augmentor/augmentor/versioning.py
  - augmentor/augmentor/config.py
  - augmentor/augmentor/config_validator.py
  - augmentor/augmentor/pipeline.py
  - augmentor/tests/unit/test_history_cap_l306.py
---

# 只追加的账也要有保留条数上限：写侧封顶 + 两份孪生一起治

## 现象

A399（L290 立）量到 `augmentor/version_control.py` 的 `history.jsonl` **无界累积**：

- 写侧只 `'a'` 追加、从不轮转封顶 ⇒ 行数线性放大磁盘；
- 读侧 `get_history` 全量读进内存再 `reverse()` + 切片 ⇒ `limit` **只省返回体积、不省读取**；
- 实测：连写 5 次 ⇒ 5 行 / 665 字节，**没有一处封顶**。

它与 A409 是兄弟格不是同一格：A409 治的是**产物全量副本**的无界复制，这一格治的是
**账**的无界追加。

## 口径：照既有形状，不新造

- **保留条数**（不是删除条数），与 A409 的 `max_versions`、备份族的 `max_backups` 同款：
  超出时**删最旧的**；
- **不选「轮转」**：轮转要留备份文件（`.1` / `.2`），而这一份是**账**（读侧按
  `get_history` 取），留多份备份会让读侧口径分裂；
- **不选「只读尾部 N 行」**（见下）。

## 两份孪生一起治

`augmentor/version_control.py` 的 `DatasetVersionManager`（`self._history_file`）与
`augmentor/versioning.py` 的 `VersionManager`（`self._history_path`）各带一份**同形状**的
history。CLI 的 `version` 命令走前者、pipeline 的自动快照走后者 ⇒ **只治一半就留下另一半是
同一族**。本轮两份一起收。

## 封顶怎么落

- **写侧**：追加之后按保留条数封顶（删最旧的）。**整份重写**——JSONL 的「最旧」在文件
  **开头**，要删开头几行就得把后面的搬过去；行数封顶下文件本身不大（每行一条记录），
  整份重写的代价与规模同阶。
- **锁**：封顶在**写侧的同一把锁**（`path_lock`）里做，否则并发下两支都以为「自己那条是
  最旧的」而把彼此的账删掉。
- **判参在落盘之前**：`history_limit=0` 当场拒，且磁盘上不留任何东西（L44 立的口径）。

## 「只读尾部」为什么没做（真实取舍，不藏）

本轮实测过尾部 seek 方案（`seek` 到末尾往前读块）：它确实只读尾部那一段（108 KB 文件取 2 行
耗时 0.7 ms），但**倒着逐行找换行**那段实现既慢又易错（块边界会把行截断）。

更关键的是：封顶已经把这份账的规模钉在 `history_limit` 行 ⇒「`limit` 只省返回体积、不省读取」
那一族的代价**是有界的**。所以读侧保持朴素的全读 + 切片，把「只读尾部」留成另一族候选。

中途试过 `deque(f, maxlen=...)` 又撤回：**它只省内存不省读取**——实测迭代到末尾、文件位置仍在
全长（`maxlen` 约束的是容器长度，不是读取量）。这个形状看起来像「只读尾部」，实际不是。

## 新增配置键的固定代价

与 A409 同款：新增 `versioning.history_limit`（出厂 1000）要同时进四处——静态规格表、
`config.yaml` 的 `versioning` 节（写出 + 注释两态）、`docs/augmentor/README.md` 的配置表、
以及三把计数尺（l154 键数普查、l222 三侧计数、l154 数值规格键、l77 落地矩阵）。漏一处就有
尺子当场喊红。

## 守卫

`augmentor/tests/unit/test_history_cap_l306.py`（**9 支 / 零 parametrize**）：

- 封顶三支：两份孪生连写 5 次、封顶 3 条 ⇒ 盘上恒 3 行 / 删的是最旧的 / 没超上限一条不丢；
- 承重一支：把 `_prune_history` 换成恒等函数 ⇒ 盘上留 5 行；
- 读侧两支：`limit` 三档语义不变（`None` = 保留集、`0` = 一条都不要、`n` = 最新 n 条）；
  写多少条读侧看到的都不超过保留条数；
- 接线三支：配置键出厂 1000 / pipeline 把键交给 manager / `history_limit=0` 当场拒且磁盘上
  不留任何东西。

## 教训

- **「看起来只读尾部」不等于真的只读尾部**：`deque(f, maxlen=...)` 是这一族最容易自欺的
  形状——它约束的是容器长度。量法要用**文件位置**而不是容器长度。
- **两份孪生是最容易漏的一半**：同形状的实现在两个模块里各有一份时，「改了一处」等于
  「只治了一半」。找孪生的办法是搜**字段名**（`self._history_file` / `self._history_path`）
  而不是搜方法名。
- **封顶是写侧的事**：读侧再怎么优化，省下的也只是返回体积；先把写侧钉住，读侧的代价才有界。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L306 清单行与日志块、A399 行
- 守卫：`augmentor/tests/unit/test_snapshot_cap_l305.py`（A409 的产品侧封顶）、
  `augmentor/tests/unit/test_history_cap_l306.py`（A399 的账本侧封顶）
- 复用：`augmentor/backup.py` 的 `clean_old_backups`（同款「保留份数 / 超出删旧」形状）、
  `augmentor/atomic_write.py` 的 `path_lock`
- 同族：A409（产物全量副本的无界复制）、A402（请求侧列表长度那一维无约束）
