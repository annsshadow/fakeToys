---
title: "自动快照这一路要有保留份数上限：删旧不拒新，以及 current 那一版的保护（A409 / L305）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: best-practices
module: augmentor/augmentor/versioning.py + augmentor/augmentor/config.py
problem_type: unbounded_artifact_accumulation
component: auto_snapshot_retention_cap
severity: medium
symptoms:
  - "每次增强结束都在 data/versions 落一份全量副本，而这一族没有任何一侧封顶"
  - "N 次增强 ⇒ 盘上 N 份全量；产物量级与数据集同阶，不是日志那种小量级"
  - "自动打点不封顶 ⇒ 磁盘占用随使用次数线性放大，而调用方拿不到任何信号"
root_cause: snapshot_writer_has_no_retention_cap_and_no_one_owns_pruning
resolution_type: retention_cap_config_key_with_delete_old_keep_new
tags: [retention, cap, snapshot, auto-snapshot, versioning, config-key, delete-old-keep-new, current-pointer, orphan-pointer, three-side-reconciliation]
applies_when:
  - "有一个「每次操作都落一份产物」的写面，而产物量级与输入同阶"
  - "配置面已经有同款形状的键（保留份数 / 超出删旧），可以照抄而不新造口径"
  - "落盘产物之间有一条「当前版本」指针，删档时必须考虑它"
related_components:
  - augmentor/augmentor/versioning.py
  - augmentor/augmentor/config.py
  - augmentor/augmentor/config_validator.py
  - augmentor/augmentor/pipeline.py
  - augmentor/augmentor/backup.py
  - augmentor/tests/unit/test_snapshot_cap_l305.py
---

# 自动快照这一路要有保留份数上限：删旧不拒新，以及 current 那一版的保护

## 现象

A409（L294 立）量到 `versioning.enabled` × `versioning.auto_snapshot` 这一路的自动快照
**没有保留上限**：

- 每次增强结束都在 `data/versions` 落一份**全量副本**（`augmentor/pipeline.py` 的
  `augment_dataset` 在 `versioning.enabled and versioning.auto_snapshot` 时调
  `VersionManager.create_version`）；
- 仓内查无任何一侧封顶：`max_versions` / `keep_last` / `prune` / `retention`
  四个候选键名在 `augmentor/versioning.py` 与 `augmentor/config.py` 两文件现量 **0 命中**；
- 实测：连打 5 次自动快照 ⇒ 盘上 5 份完整副本 + `history.jsonl` 5 行，**没有一处封顶**。

A399 治的是同一族里 `history.jsonl` 的无界追加；这一格是**产物全量副本**的无界复制，
量级差两个数量级。

## 口径：照备份族，不新造形状

`augmentor/backup.py` 的 `clean_old_backups(backup_dir, max_backups=10)` 已经是
「**保留份数** / 超出删旧」的形状，本轮照抄：

- `VersioningConfig` 新增 `max_versions: int = 10`（**保留份数**，不是删除份数）；
- `VersionManager` 接住它，并在 `create_version` 之后自己收旧 ⇒ **谁建版本谁负责封顶**。
  为什么不放在 pipeline 侧另开一条路：那一族若只封「自动快照」，手工建的版本仍会无限堆，
  只治一半；
- 超出时**删旧不拒新**：刚建的那一份永远在保留集里，被删的永远是最旧的。

**为什么删旧不拒新**：这一族的用户动作是「增强结束自动打点」，**拒新会让打点静默失败**
（调用方拿不到任何信号），删旧则可逆（`history.jsonl` 留一笔 `prune`）。

## current 那一版的保护（本轮真实返工）

删掉 `current` 指向的那一版会让 `current` 指向不存在的目录，而读侧
`get_current_version` 仍报得出那个 id ⇒ **孤儿指针**。

第一版按「prune 时读 current」实现，被守卫当场抓住：`create_version` 建版过程**已经把
current 改指到新版本**了 ⇒ prune 时读到的是新版本，用户回滚指着的旧版本照样被删。

处置是 `prune_versions(protect=...)` 接一个「建版**之前**的 current」，由 `create_version`
在锁内取一次传进来；受保护的那一版自己**占掉一个保留名额**（否则保留集是 N+1 份、
封顶就没封住）。

配套把守卫那一支的期望也改口径：量的是「**current 此刻指向**的那一版永不被删」，而不是
「某一份被回滚指着的版本永不被删」—— 后者已被用户的新版本正常覆盖，按「删旧不拒新」的
语义删掉是合乎口径的。这一条容易读反：**护的是指针的指向，不是某一版的身份**。

## 新增配置键的固定代价

新增一个配置键要同时进四处，漏一处就有尺子当场喊红：

| 面 | 落点 |
| --- | --- |
| 运行时判据 | `VersioningConfig.__post_init__` 的 `require_count(..., minimum=1)` |
| 静态规格表 | `augmentor/config_validator.py` 的 `KNOWN_FIELDS` |
| 配置模板 | `config.yaml` 的 `versioning` 节（写出 + 注释两态都要有） |
| 文档表 | `docs/augmentor/README.md` 的配置表 |
| 键数普查 | l154 的逐节键数（`versioning` 3 → 4 只、全账 30 → 31 只） |
| 三侧计数 | l222 的三侧同数（70 → 71 键） |
| 数值规格键计数 | l154 的 `numeric` 现量（21 → 22） |
| 落地矩阵 | l77 的 `fields()` 推导矩阵（计数 70 → 71，落地档自动长出并判绿） |

判参的**先后次序**也算一处代价：`VersionManager.__init__` 里 `require_count` 必须排在
`mkdir` **之前**（L44 立的口径：坏参数不许在磁盘上留下任何东西）。

## 守卫

`augmentor/tests/unit/test_snapshot_cap_l305.py`（**10 支 / 零 parametrize**）：

- 封顶四支：连打 5 次封顶 3 份 ⇒ 恒 3 份 / 删的是最旧的 / 没超上限不删 /
  current 指向的那一版永不被删；
- 承重两支：把 `prune_versions` 换成恒等函数 ⇒ 盘上留 5 份（证明上面那几支量的
  确实是封顶的功劳）；封顶返回删除份数；
- 接线四支：配置键出厂 10 / pipeline 把键交给 manager / 被封顶删掉的那份在 history 留
  `prune` 一笔 / `max_versions=0` 当场拒且磁盘上不留任何东西。

**改后实测**：封顶 3 份时连打 5 次 ⇒ 盘上恒 3 份、`history.jsonl` 动作序列
`create×3 / prune / create / prune`、current 始终在场、总字节 54359 → 33213。

## 余面与教训

- **history.jsonl 仍是无界的**：A399 那一格（`get_history` 全量读进内存再切片）本轮**没动**，
  它是兄弟格不是同一格 —— 本轮治的是**产物全量副本**。
- **手工建的版本也归封顶管**：`prune_versions` 挂在 `create_version` 上而不是挂在
  `auto_snapshot` 分支上，所以 CLI / SDK 建的版本同样进保留集 ⇒ 「自动快照封了、手工建的
  版本无限堆」那一族只治一半的问题不存在。
- **封顶会删用户的档**：这是不可逆动作（`shutil.rmtree`）。缓解是三件：删的永远是最旧的、
  被删的那份在 `history.jsonl` 里留痕、`current` 指向的那一版永不被删。**要不要给用户一个
  「不封顶」的开关**（配置键为 0 / 负数即不限）本轮**没拍** —— 那是另一族对外语义
  （「关掉封顶」与「封顶 1 份」在后果上完全不同），待拍。
- **判据要先问「护的是谁」**：本轮的返工就是没先问清楚 —— 护的是「指针的指向」还是
  「某一版的身份」，两者写出来的代码形状不同。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L305 清单行与日志块、A409 行
- 守卫：`augmentor/tests/unit/test_snapshot_cap_l305.py`（10 支 / 零 parametrize）
- 复用：`augmentor/backup.py` 的 `clean_old_backups`（同款「保留份数 / 超出删旧」形状）、
  `augmentor/config.py` 的 `require_count` 判据口径、`augmentor/versioning.py` 的
  `path_lock` 同抢一把
- 同族：A399（`history.jsonl` 无界追加，兄弟格未动）、A401 / A402 / A403 / A408（同批待拍）
