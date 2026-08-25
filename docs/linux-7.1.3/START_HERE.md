# Linux Kernel 学习路径

> 3934 篇内核文档中精选出的阅读路线图> 按此路径阅读，可在合理时间内建立对内核整体架构的心智模型
---

## 如何使用本指
- **必读** 标记了建立基础认知不可或缺的文- **选读** 可按兴趣和需求选择阅读
- 每阶段建议按顺序阅读，但阶段之间可以跳过不需要的领域
- 链接指向 `docs/` 下的 Markdown 版本（由 `tools/docs/rst-to-md.py` 转换生成
---

## 阶段 1：Orientation（必读，2 小时
目标：了解内核是什么、文档在哪里、社区如何运作
| # | 文档 | 说明 |
|---|------|------|
| 1 | `README` | 按角色（开发研究安全专家/系统管理员）给你的入|
| 2 | `Documentation/index.md` | 文档地图，了解整个文档树的组织结|
| 3 | `Documentation/admin-guide/README.md` | "内核是什——硬件支持、构建安装、快速概|
| 4 | `Documentation/process/howto.md` | 如何成为内核开发者——工具、邮件列表、社区规|
| 5 | `Documentation/process/development-process.md` | 内核社区如何运作——发布周期、合并窗口、补丁生命周|

---

## 阶段 2：架构基础（必读，4-6 小时
目标：建立对内核执行模型的核心直觉。这是最重要的阶段
| # | 文档 | 说明 |
|---|------|------|
| 6 | `Documentation/kernel-hacking/hacking.md` | **最重要的单篇文*：CPU 执行上下文（用户/软中硬中空闲）、`current` 指针、调度点、基础|
| 7 | `Documentation/core-api/index.md` | 内核 API 分类图谱——快速扫一遍，知道有什么接口可|
| 8 | `Documentation/mm/index.md` | 内存管理：从物理内存到页表、slab、vmalloc |
| 9 | `Documentation/scheduler/index.md` | 进程调度：CFS 设计 + 当前 EEVDF 调度|
| 10 | `Documentation/locking/index.md` | 同步原语：自旋锁、互斥锁、RCU 锁的分类与使用场|
| 11 | `Documentation/RCU/index.md` | RCU 机制：读-复制-更新的核心思想与实|
| 12 | `Documentation/core-api/irq/index.md` | 中断处理：IRQ 域affinitymanaged IRQ |

**阅读建议*
- 先读 `hacking.md`（第 6 篇），它会在 30 分钟内给内核是怎么跑起的整体感- 然后`core-api/index.md`（第 7 篇）建立 API 地图
- 其余 5 篇按你的兴趣顺序阅读

---

## 阶段 3：子系统选读（选读，按需深入
目标：按兴趣深入了解具体子系统
| # | 文档 | 说明 |
|---|------|------|
| 13 | `Documentation/filesystems/index.md` | 虚拟文件系统（VFS）：superblock/inode/dentry 模型、路径查找、挂载命名空|
| 14 | `Documentation/networking/index.md` | 网络栈：`sk_buff` 生命周期、NAPI、`netdevice` 模型、协议栈 |
| 15 | `Documentation/driver-api/index.md` | 驱动模型：kobject/device/driver/bus 层次结构 |
| 16 | `Documentation/power/index.md` | 电源管理：运行时 PM、系统挂恢复 |
| 17 | `Documentation/security/index.md` | 安全架构：LSM 框架、内核自防御机制、凭证管|
| 18 | `Documentation/trace/index.md` | 可观测性：tracepoint、ftrace、kprobes |

**阅读建议*
- 每个子系统通常只需要读 `index.md` + 1-2 篇核心设计文- 不需要按顺序读，根据你的兴趣或工作需求选择

---

## 阶段 4：开发实践（必读，约 2 小时
目标：了解如何向内核提交代码、遵守的规范、可用的工具
| # | 文档 | 说明 |
|---|------|------|
| 19 | `Documentation/process/coding-style.rst` | 内核编码规范——缩进、命名、注释、空格与制表|
| 20 | `Documentation/process/submitting-patches.rst` | 补丁提交流程：git 格式、`git send-email`、`Signed-off-by`、changelog |
| 21 | `Documentation/kbuild/index.md` | 构建系统：顶Makefile、Kconfig 语法、模块构|
| 22 | `Documentation/dev-tools/index.md` | 开发工具：checkpatch、KUnit 单元测试、调试工|

---

## 阶段 5：扩展资源（选读
目标：找到更深入的外部学习材料
| # | 文档 | 说明 |
|---|------|------|
| 23 | `Documentation/process/kernel-docs.md` | 外部书籍和论文推荐（Linux Device Drivers、LWN 文章等） |
| 24 | `Documentation/admin-guide/index.md` | 系统管理员接口：`/proc/sys` 可调参数、启动参|
| 25 | `Documentation/userspace-api/index.md` | 用户空间 API：系统调用、安全接口、`/dev` 设备 |

---

## 快速参考：按目标选择

| 你的目标 | 建议路径 |
|----------|----------|
| 快速了解内核全| 阶段 1 + 阶段 2（第 6 篇必读） |
| 写第一个内核模| 阶段 1 阶段 2 阶段 4 `Documentation/driver-api/index.md` |
| 理解内存管理 | 阶段 1 阶段 2 `Documentation/mm/index.md` `Documentation/vm/index.md` |
| 理解文件系统 | 阶段 1 阶段 2 `Documentation/filesystems/index.md` |
| 理解网络| 阶段 1 阶段 2 `Documentation/networking/index.md` |
| 准备提交补丁 | 阶段 1 阶段 4（第 19-20 篇） |

---

## 备注

- 所Markdown 文件`tools/docs/rst-to-md.py` Sphinx `.rst` 源文件自动转换生- 如果发现转换质量问题，可以查看原`.rst` 文件或修复转换脚- 文档内容随内核版本更新，本指南基v7.1.3Baby Opossum Posse"