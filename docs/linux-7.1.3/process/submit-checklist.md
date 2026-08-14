
## Linux 内核补丁提交检查清单


如果开发者希望自己的内核补丁提交能够被更快地接受，
以下是他们应当做到的一些基本事项。

这些都超出了 `Documentation/process/submitting-patches.rst` <submittingpatches>
以及其他地方关于提交 Linux 内核补丁的文档所提供的要求。

## 审查你的代码


1) 如果你使用了某个功能（facility），那么请 `#include` 定义/声明
   该功能所在的头文件。不要依赖其他头文件为你
   拉入你所使用的头文件。

2) 按照 `Documentation/process/coding-style.rst` <codingstyle>
   中的详细说明检查补丁的总体风格。

3) 所有内存屏障（例如 `barrier()`、`rmb()`、`wmb()`）都需要在
   源代码中有注释，解释它们正在做什么以及为什么这样做的逻辑。

## 审查 Kconfig 改动


1) 任何新增或修改的 `CONFIG` 选项都不要弄乱配置菜单，并且
   除非满足 `Documentation/kbuild/kconfig-language.rst` 中
   “菜单属性：默认值”所记录的例外标准，否则默认应为关闭（off）。

2) 所有新增的 `Kconfig` 选项都应有帮助文本（help text）。

3) 已经针对相关的 `Kconfig` 组合进行了仔细审查。这一点很难通过
   测试做对——在这里，脑力思考（brainpower）是值得的。

## 提供文档


1) 包含 kernel-doc <kernel_doc> 以文档化全局内核 API。
   （静态函数不要求，但在那里也可以。）

2) 所有新增的 `/proc` 条目都在 `Documentation/` 下有文档说明。

3) 所有新增的内核启动参数都在
   `Documentation/admin-guide/kernel-parameters.rst` 中有文档说明。

4) 所有新增的模块参数都用 `MODULE_PARM_DESC()` 进行文档说明。

5) 所有新增的用户空间接口都在 `Documentation/ABI/` 中有文档说明。
   有关更多信息，请参见 Documentation/admin-guide/abi.rst（或 `Documentation/ABI/README`）。
   修改用户空间接口的补丁应当抄送（CC）到
   linux-api@vger.kernel.org。

6) 如果补丁新增了任何 ioctl，那么也要更新
   `Documentation/userspace-api/ioctl/ioctl-number.rst`。

## 用工具检查你的代码


1) 在提交之前用补丁风格检查器检查是否存在琐碎的违规
   （`scripts/checkpatch.pl`）。
   你应该能够为补丁中残留的所有违规给出合理解释。

2) 用 sparse 干净地通过检查。

3) 使用 `make checkstack` 并修复它发现的任何问题。
   注意 `checkstack` 并不会显式指出问题，
   但任何在栈上使用超过 512 字节的函数都是需要修改的候选对象。

## 构建你的代码


1) 干净地构建：

  a) 在适用的或已修改的 `CONFIG` 选项分别为 `=y`、`=m` 以及
     `=n` 时。没有 `gcc` 警告/错误，没有链接器警告/错误。

  b) 通过 `allnoconfig`、`allmodconfig`。

  c) 在使用 `O=builddir` 时构建成功。

  d) 任何 Documentation/ 下的改动都能成功构建，且不产生新的警告/错误。
     使用 `make htmldocs` 或 `make pdfdocs` 来检查构建并
     修复任何问题。

2) 通过使用本地交叉编译工具或某个其他构建集群，
   在多种 CPU 架构上构建。
   注意，针对不同字长（32 位和 64 位）以及不同字节序
   （大端和小端）的架构进行测试，能够有效发现由于对
   可表示数量范围、数据对齐或字节序等做出错误假设而导致的
   各种可移植性问题。

3) 新增的代码已经用 `gcc -W`（使用
   `make KCFLAGS=-W`）编译过。这会产生大量噪音，但有利于
   发现类似“warning: comparison between signed and unsigned”
   这样的 bug。

4) 如果你修改的源代码依赖或使用了以下 `Kconfig` 符号相关的
   任何内核 API 或特性，那么请用相关的 `Kconfig` 符号被禁用
   和/或设为 `=m`（如果该选项可用）来进行多次构建
   [不需要同时全部设置，只需它们各种随机的组合]：

   `CONFIG_SMP`、`CONFIG_SYSFS`、`CONFIG_PROC_FS`、`CONFIG_INPUT`、
   `CONFIG_PCI`、`CONFIG_BLOCK`、`CONFIG_PM`、`CONFIG_MAGIC_SYSRQ`、
   `CONFIG_NET`、`CONFIG_INET=n`（后者搭配 `CONFIG_NET=y`）。

## 测试你的代码


1) 已经用 `CONFIG_PREEMPT`、`CONFIG_DEBUG_PREEMPT`、
   `CONFIG_SLUB_DEBUG`、`CONFIG_DEBUG_PAGEALLOC`、`CONFIG_DEBUG_MUTEXES`、
   `CONFIG_DEBUG_SPINLOCK`、`CONFIG_DEBUG_ATOMIC_SLEEP`、
   `CONFIG_PROVE_RCU` 以及 `CONFIG_DEBUG_OBJECTS_RCU_HEAD`
   全部同时启用进行测试。

2) 已经用和不用 `CONFIG_SMP` 与
   `CONFIG_PREEMPT` 进行了构建与运行时测试。

3) 所有代码路径都已在启用所有 lockdep 特性的情况下被执行过。

4) 已经通过至少注入 slab 与页分配
   失败进行检查。参见 `Documentation/fault-injection/`。
   如果新代码量很大，添加子系统特定的故障注入可能是合适的。

5) 已用 linux-next 最新的标签进行测试，以确保它仍能
   与所有其他已排队的补丁以及 VM、
   VFS 和其他子系统中的各种变动协同工作。
