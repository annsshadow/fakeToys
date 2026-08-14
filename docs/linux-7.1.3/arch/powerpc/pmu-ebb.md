## PMU Event Based Branches


Event Based Branches（基于事件的分支，EBB）是一项特性，它允许硬件在某些事件发生时，直接分支到指定的用户空间地址。

完整规范见 Power ISA v2.07：

  https://www.power.org/documentation/power-isa-version-2-07/

EBB 可以配置的一种事件类型是 PMU 异常。本文档描述了配置 Power PMU 以生成 EBB 的 API，使用的是 Linux perf_events API。


### Terminology


在本文档中，我们始终会提到"EBB event"或"EBB events"。这仅指在其 attr.config 中设置了 "EBB" 标志的 struct perf_event。硬件 PMU 上可以配置的所有事件都可能是"EBB events"。


### Background


当发生 PMU EBB 时，它被递交给当前正在运行的进程。因此，EBB 只能被程序用于自我监视，这样才有意义。

perf_events API 的一个特性是，事件可以在其他进程上创建，但需经过标准权限检查。EBB 事件也是如此，不过除非目标进程启用了 EBB（通过 mtspr(BESCR)），否则永远不会递送任何 EBB。

这使得一个进程能够为自己启用 EBB，但实际上并未配置任何事件。稍后另一个进程可以介入，并向该进程附加一个 EBB 事件，从而开始向第一个进程递送 EBB。目前尚不清楚这是否真的有用。


当 PMU 被配置为 EBB 时，所有 PMU 中断都被递交给用户进程。这意味着一旦在 PMU 上调度了一个 EBB 事件，就不能再配置任何非 EBB 事件。这意味着 EBB 事件无法与常规的 'perf' 命令或任何其他 perf 事件并发运行。

不过，在使用 EBB 的进程上运行 'perf' 命令是安全的。内核通常会调度 EBB 事件，而 perf 会收到通知说它的事件无法运行。

EBB 事件与常规事件之间的互斥是通过 perf_events 现有的 "pinned" 和 "exclusive" 属性实现的。这意味着 EBB 事件会优先于其他事件，除非它们也被 pinned。如果一个 EBB 事件和一个常规事件都被 pinned，那么先被启用的那个会被调度，另一个会进入错误状态。更多信息请参阅下面标题为"启用一个 EBB 事件"的小节。


### Creating an EBB event


要请求使用 EBB 来计数某个事件，事件码应当置位第 63 位。

EBB 事件必须使用一组特定且受限的属性来创建——这样它们才能与 perf_events 子系统的其余部分正确协作。

EBB 事件必须以 "pinned" 和 "exclusive" 属性被创建。请注意，如果你正在创建一组 EBB 事件，只有 leader 可以设置这些属性。

EBB 事件绝不能设置任何 "inherit"、"sample_period"、"freq" 或 "enable_on_exec" 属性。

EBB 事件必须附加到一个任务上。这是通过向 perf_event_open() 传递一个 pid 值来指定的，通常为 0，表示当前任务。

组内的所有事件必须在是否想要 EBB 上保持一致。也就是说，要么所有事件都请求 EBB，要么都不请求 EBB。

EBB 事件必须指定要在其上计数的 PMC。这确保用户空间能够可靠地确定事件被调度到了哪个 PMC。


### Enabling an EBB event


一旦 EBB 事件被成功打开，就必须用 perf_events API 启用它。这可以通过 ioctl() 接口或 prctl() 接口来完成。

不过，由于 perf_events API 的设计，启用一个事件并不能保证它已经被调度到 PMU 上。要确保 EBB 事件已经被调度到 PMU 上，你必须对该事件执行一次 read()。如果 read() 返回 EOF，则说明该事件尚未被调度，EBB 也未启用。

出现这种行为是因为 EBB 事件是 pinned 且 exclusive 的。当 EBB 事件被启用时，它会将 PMU 上所有其他非 pinned 的事件挤掉。在这种情况下，启用会成功。然而，如果 PMU 上已经有一个 pinned 的事件，那么启用就不会成功。


### Reading an EBB event


可以从 EBB 事件执行 read()。但结果毫无意义。由于中断是被递交给用户进程的，内核无法对该事件进行计数，因此会返回一个无意义的值。


### Closing an EBB event


当一个 EBB 事件使用完毕时，可以像任何常规事件一样用 close() 关闭它。如果这是最后一个 EBB 事件，PMU 将被去配置，不会再递送任何 PMU EBB。


### EBB Handler


EBB 处理程序只是常规的用户空间代码，但它必须以中断处理程序的风格编写。进入处理程序时，所有寄存器（可能）都是活动的，因此在处理程序能够调用其他代码之前，必须以某种方式保存它们。

程序如何处理这一点由自己决定。对于 C 程序，一个相对简单的选择是在栈上创建一个中断帧，并将寄存器保存在那里。


### Fork


EBB 事件不会跨 fork 继承。如果子进程希望使用 EBB，它应该为自己打开一个新的事件。类似地，BESCR/EBBHR/EBBRR 中的 EBB 状态会在 fork() 时被清除。
