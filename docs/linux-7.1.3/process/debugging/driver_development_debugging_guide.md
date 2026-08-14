
## 驱动开发调试建议


本文档作为调试设备驱动的一般起点和查阅入口。虽然本指南侧重于需要重新编译
模块/内核的调试，但 :doc:`用户空间调试指南
</process/debugging/userspace_debugging_guide>` 将引导你了解诸如动态调试、ftrace
等用于调试问题和行为的实用工具。有关通用调试建议，请参阅 :doc:`通用建议文档
</process/debugging/index>`。

    :depth: 3

以下各节向你展示可用的工具。

### printk() 及其同类


这些是 printf() 的变体，具有不同的输出目标，并支持动态开启或关闭（或缺乏此支持）。

#### 简单的 printk()


经典的用法，可用于快速而粗糙地开发新模块，或提取用于排障的任意必要数据，效果极佳。

前置条件：`CONFIG_PRINTK`（默认通常启用）

**优点**：

- 无需学习任何东西，使用简单
- 易于根据你的需求精确修改（数据的格式化（参见：
  [/core-api/printk-formats](/core-api/printk-formats)）、在日志中的可见性）
- 会导致代码执行出现延迟（有利于确认时序是否是一个因素）

**缺点**：

- 需要重新构建内核/模块
- 会导致代码执行出现延迟（这可能导致问题无法复现）

完整文档请参见 [/core-api/printk-basics](/core-api/printk-basics)

#### Trace_printk


前置条件：`CONFIG_DYNAMIC_FTRACE` & `#include <linux/ftrace.h>`

它使用起来比 printk() 稍微不那么顺手，因为你需要从 trace 文件中读取消息（参见：
read_ftrace_log），而不是从内核日志中读取；但当 printk() 给代码执行带来不想要的
延迟，导致问题变得不稳定或隐藏时，它非常有用。

如果处理过程仍然导致时序问题，那么你可以尝试 trace_puts()。

完整文档请参见 trace_printk()

#### dev_dbg


打印语句，可被 process/debugging/userspace_debugging_guide:dynamic debug 所针对，
其中包含关于在上下文中使用的设备的额外信息。

**何时适合在代码中留下调试打印？**

永久性的调试语句必须对开发者排查驱动异常行为有用。判断这一点更多是一门艺术而非
科学，但一些指导原则在 :ref:`编码风格指南
<process/coding-style:13) printing kernel messages>` 中。在几乎所有情况下，调试
语句都不应被合入上游，因为一个正常工作的驱动应当是安静的。

#### 自定义 printk


```

  #define core_dbg(fmt, arg...) do { \
	  if (core_debug) \
		  printk(KERN_DEBUG pr_fmt("core: " fmt), ## arg); \
	  } while (0)

```
**何时应该这样做？**

最好直接使用 pr_debug()，它之后可以通过动态调试开启/关闭。此外，许多驱动通过类似
`core_debug` 这样的变量（由模块参数设置）来激活这些打印。然而，模块参数 `已不再
被推荐 <https://lore.kernel.org/all/2024032757-surcharge-grime-d3dd@gregkh>`_。

### Ftrace


#### 创建自定义 Ftrace 跟踪点


跟踪点向你的代码中添加一个钩子，当该跟踪点被启用时会被调用并记录。例如，这可用于
跟踪命中一个条件分支，或在调试会话期间代码流的特定点转储内部状态。

这里是关于 :ref:`如何实现新跟踪点 <trace/tracepoints:usage>` 的基本描述。

完整事件跟踪文档请参见 [/trace/events](/trace/events)

完整 Ftrace 文档请参见 [/trace/ftrace](/trace/ftrace)

### DebugFS


前置条件：``CONFIG_DEBUG_FS` & `#include <linux/debugfs.h>``

DebugFS 不同于其它调试方法，因为它不向内核日志写入消息，也不向代码添加跟踪。相反，
它允许开发者处理一组文件。借助这些文件，你可以存储变量的值，或进行寄存器/内存转储，
或者你可以使这些文件可写，以修改驱动中的值/设置。

可能的用例包括但不限于：

- 存储寄存器值
- 跟踪变量
- 存储错误
- 存储设置
- 切换某个设置，例如调试开/关
- 错误注入

当数据转储的大小难以作为通用内核日志的一部分消化时（例如转储原始比特流数据时），
或者当你并非一直对所有值都感兴趣、但希望能够检查它们时，这尤其有用。

一般思路是：

- 在 probe 期间创建一个目录（``struct dentry *parent =
  debugfs_create_dir("my_driver", NULL);``）
- 创建一个文件（`debugfs_create_u32("my_value", 444, parent, &my_variable);`）

  - 在此示例中，该文件位于
    `/sys/kernel/debug/my_driver/my_value`（对用户/组/所有人具有读权限）
  - 对该文件的任何读取都将返回变量 `my_variable` 的当前内容

- 在移除设备时清理该目录
  （`debugfs_remove(parent);`）

完整文档请参见 [/filesystems/debugfs](/filesystems/debugfs)。

### KASAN、UBSAN、lockdep 及其它错误检查器


#### KASAN（内核地址消毒剂）


前置条件：`CONFIG_KASAN`

KASAN 是一个动态内存错误检测器，有助于发现释放后使用（use-after-free）和越界
（out-of-bounds）错误。它使用编译期插桩来检查每次内存访问。

完整文档请参见 [/dev-tools/kasan](/dev-tools/kasan)。

#### UBSAN（未定义行为消毒剂）


前置条件：`CONFIG_UBSAN`

UBSAN 依赖编译器插桩和运行时检查来检测未定义行为。它旨在发现各种问题，包括有符号
整数溢出、数组下标越界等。

完整文档请参见 [/dev-tools/ubsan](/dev-tools/ubsan)

#### lockdep（锁依赖验证器）


前置条件：`CONFIG_DEBUG_LOCKDEP`

lockdep 是一个运行时锁依赖验证器，可检测潜在死锁以及内核中其它与锁相关的问题。
它跟踪锁的获取和释放，构建一个依赖图，并分析其中潜在的死锁。lockdep 对于验证内核
中锁顺序的正确性特别有用。

#### PSI（压力阻塞信息跟踪）


前置条件：`CONFIG_PSI`

PSI 是一个测量工具，用于识别硬件资源上过度的过量提交（overcommit），这可能会导致
性能中断甚至 OOM 杀死。

### 设备核心转储（device coredump）


前置条件：`CONFIG_DEV_COREDUMP` & `#include <linux/devcoredump.h>`

提供基础设施，使驱动能够向用户空间提供任意数据。它最常与 udev 或类似的用户空间
应用程序结合使用，以监听指示转储已就绪的内核 uevent。Udev 有规则将该文件复制到某处
进行长期存储和分析，因为默认情况下，转储数据会在默认 5 分钟后自动清理。该数据使用
驱动特定的工具或 GDB 进行分析。

设备核心转储可以用 vmalloc 区域创建（带有 read/free 方法），或者作为分散/聚集列表
创建。

你可以在以下位置找到一个示例实现：
`drivers/media/platform/qcom/venus/core.c
<https://elixir.bootlin.com/linux/v6.11.6/source/drivers/media/platform/qcom/venus/core.c#L30>`__，
在蓝牙 HCI 层、若干无线驱动以及若干 DRM 驱动中。

#### devcoredump 接口



**版权** ©2024 : Collabora
