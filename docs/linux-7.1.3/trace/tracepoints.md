## 使用 Linux 内核跟踪点（Tracepoints


:Author: Mathieu Desnoyers


本文档介Linux 内核跟踪点及其使用方法。它提供了如何在内核中插入跟踪点、并将探测函数连接到这些跟踪点的示例，同时给出了一些探测函数的例子


### 跟踪点的用


放置在代码中的跟踪点提供了一个钩子，用于调用一个你可以在运行时提供的函数（探针）。一个跟踪点可以是“开启”（已连接探针）或“关闭”（未附加探针）状态。当跟踪点为“关闭”时，它除了带来微小的时间开销（检查分支条件）和空间开销（在被插桩的函数末尾添加用于函数调用的几个字节，并在一个独立的段中添加数据结构）外，没有其它影响。当跟踪点为“开启”时，你提供的函数会在每次跟踪点执行时被调用，且处于调用者的执行上下文中。当所提供函数执行结束时，它会返回到调用者（从跟踪点位置继续执行）

你可以在代码中的重要位置放置跟踪点。它们是轻量级的钩子，可以传递任意数量的参数，其原型在放在头文件中的跟踪点声明里描述

它们可用于追踪和性能统计


### 用法


使用跟踪点需要两个要素：

- 放置在头文件中的跟踪点定义
- 位于 C 代码中的跟踪点语句

为了使用跟踪点，你应该包linux/tracepoint.h

```

	#undef TRACE_SYSTEM
	#define TRACE_SYSTEM subsys

	#if !defined(_TRACE_SUBSYS_H) || defined(TRACE_HEADER_MULTI_READ)
	#define _TRACE_SUBSYS_H

	#include <linux/tracepoint.h>

	DECLARE_TRACE(subsys_eventname,
		TP_PROTO(int firstarg, struct task_struct *p),
		TP_ARGS(firstarg, p));

	#endif /* _TRACE_SUBSYS_H */

	/* This part must be outside protection */
	#include <trace/define_trace.h>

```
```

	#include <trace/events/subsys.h>

	#define CREATE_TRACE_POINTS
	DEFINE_TRACE(subsys_eventname);

	void somefct(void)
	{
		...
		trace_subsys_eventname_tp(arg, task);
		...
	}

```
其中
  - subsys_eventname 是你的事件中唯一的标识符

    - subsys 是你的子系统名称
    - eventname 是要追踪的事件名称

  - `TP_PROTO(int firstarg, struct task_struct *p)` 是该跟踪点所调用函数的原型

  - `TP_ARGS(firstarg, p)` 是参数名称，与原型中的相同

  - 如果你在多个源文件中使用该头文件，`#define CREATE_TRACE_POINTS` 应该只出现在一个源文件中

将一个函数（探针）连接到一个跟踪点，是通过为特定跟踪点提供一个探针（要调用的函数）来完成的，使用 register_trace_subsys_eventname()。移除探针则通过 unregister_trace_subsys_eventname() 完成；它会移除该探针

必须在模块退出函数结束之前调tracepoint_synchronize_unregister()，以确保没有调用者仍在使用该探针。这一点，加上在探针调用周围禁用了抢占，保证了探针移除和模块卸载的安全性

跟踪点机制支持插入同一个跟踪点的多个实例，但必须对整个内核中的给定跟踪点名称只做一次定义，以确保不会发生类型冲突。跟踪点的名称改写（name mangling）使用原型来完成，以确保类型正确。探测类型正确性的验证由编译器在注册处完成。跟踪点可以放在内联函数、内联静态函数、展开循环以及常规函数中

这里建议采用“subsys_event”命名方案作为一种约定，以限制名称冲突。跟踪点名称对整个内核是全局的：无论它们位于核心内核映像还是模块中，都被视为相同的

如果跟踪点要在内核模块中使用，可以使EXPORT_TRACEPOINT_SYMBOL_GPL() EXPORT_TRACEPOINT_SYMBOL() 来导出已定义的跟踪点

如果你需要为某个跟踪点参数做一点工作，而该工作仅用于该跟踪点，则可以将该工作封
```

	if (trace_foo_bar_enabled()) {
		int i;
		int tot = 0;

		for (i = 0; i < count; i++)
			tot += calculate_nuggets();

		trace_foo_bar_tp(tot);
	}

```
所trace_<tracepoint>_tp() 调用都有一个匹配的 trace_<tracepoint>_enabled() 函数，当跟踪点启用时返回 true，否则返false。trace_<tracepoint>_tp() 应始终位if (trace_<tracepoint>_enabled()) 块内部，以防止跟踪点被启用与检查被观察到之间发生竞态

使用 trace_<tracepoint>_enabled() 的优势在于，它利用跟踪点static_key if 语句可以通过跳转标签（jump labels）实现，从而避免条件分支

      定义跟踪点。注意，DECLARE_TRACE(foo) 会创建一个名"trace_foo_tp()" 的函数，TRACE_EVENT(foo) 会创建一个名"trace_foo()" 的函数，同时还会/sys/kernel/tracing/events 目录下将该跟踪点作为跟踪事件暴露出来。更多细节请参阅 http://lwn.net/Articles/379903、http://lwn.net/Articles/381064 http://lwn.net/Articles/383362 系列文章

如果你需要从头文件中调用跟踪点，不建议直接调用或使用 trace_<tracepoint>_enabled() 函数调用，因为当头文件被设置CREATE_TRACE_POINTS 的文件包含时，头文件中的跟踪点可能产生副作用，而且 trace_<tracepoint>() 内联函数并不算小，如果被其它内联函数使用会使内核膨胀。相反，应当包含 tracepoint-defs.h 并使tracepoint_enabled()

```

	void do_trace_foo_bar_wrapper(args)
	{
		trace_foo_bar_tp(args); // for tracepoints created via DECLARE_TRACE
					//   or
		trace_foo_bar(args);    // for tracepoints created via TRACE_EVENT
	}

```
```

	DECLARE_TRACEPOINT(foo_bar);

	static inline void some_inline_function()
	{
		[..]
		if (tracepoint_enabled(foo_bar))
			do_trace_foo_bar_wrapper(args);
		[..]
	}

```