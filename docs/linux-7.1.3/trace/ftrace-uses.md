## 使用 ftrace 钩挂到函
4.14 撰写

## 简
ftrace 基础设施最初被创建用来将回调附加到函数的开头，以记录和追踪内核的流程。但对函数开头的回调可以有其他用例。无论是用于内核热补丁（live kernel patching），还是用于安全监控。本文档描述如何使用 ftrace 实现你自己的函数回调
## ftrace 上下
  向内核中几乎任何函数添加回调的能力伴随着风险。回调可以从任何上下文（普通、softirq、irq NMI）调用。回调也可以在即将进入空闲、CPU 上线和下线期间，或即将进入用户空间时调用。这要求对回调内部可以做什么格外小心。回调可以在 RCU 的保护范围之外被调用
有辅助函数可以帮助防止递归，并确保 RCU 正在监视（watching）。这些将在下面解释
## ftrace_ops 结构

要注册一个函数回调，需要一ftrace_ops。此结构用于告诉 ftrace 哪个函数应作为回调被调用，以及该回调将执行哪些保护从而不需ftrace 来处理
在向 ftrace 注册 ftrace_ops 时，只需要设置一个字段：

 struct ftrace_ops ops = {
       .func			= my_callback_func,
       .flags			= MY_FTRACE_FLAGS
       .private			= any_private_data_structure,
 };

.flags .private 都是可选的。只.func 是必需的
```

    register_ftrace_function(&ops);

```
```

    unregister_ftrace_function(&ops);

```
```

    #include <linux/ftrace.h>

```
注册的回调将register_ftrace_function() 被调用之后、返回之前的某个时刻开始被调用。回调开始被调用的确切时间取决于架构和服务的调度。如果回调必须在精确时刻开始，则它自己必须处理任何同步
unregister_ftrace_function() 将保证在 unregister_ftrace_function() 返回之后，函数不再调用该回调。注意，为了执行这一保证，unregister_ftrace_function() 可能需要一些时间来完成
## 回调函数

回调函数的原型如下（v4.14 起）
   void callback_func(unsigned long ip, unsigned long parent_ip,
                      struct ftrace_ops **op, struct pt_regs **regs);

@ip
	 这是正在被追踪的函数的指令指针      	 （fentry mcount 在函数的位置
@parent_ip
	 这是调用了被追踪函数的函数的指令指针
	（函数调用发生的位置）
@op
	 这是指向用于注册该回调的 ftrace_ops 的指针	 这可用于通过 private 指针向回调传递数据
@regs
	 如果ftrace_ops 结构中设置了 FTRACE_OPS_FL_SAVE_REGS 	 FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED 标志，那么这将指	 pt_regs 结构，就像在 ftrace 所追踪的函数开头放置了一	 断点一样。否则它要么包含垃圾数据，要么为 NULL
## 保护你的回调

由于函数可以从任何地方调用，并且回调调用的函数也可能被追踪、并调用同一个回调，因此必须使用递归保护。在这方面有两个辅助函数可以提供帮助。如果你这样开始你的代码：

 int bit;

	bit = ftrace_test_recursion_trylock(ip, parent_ip);
	if (bit < 0)
		return;

并以这样结束
	ftrace_test_recursion_unlock(bit);

那么中间包含的代码将可安全使用，即使它最终调用了回调正在追踪的函数。注意，成功ftrace_test_recursion_trylock() 将禁用抢占，ftrace_test_recursion_unlock() 将再次启用（如果之前已启用）。指令指针（ip）及其父指针（parent_ip）被传递给 ftrace_test_recursion_trylock() 以记录递归发生的位置（如果设置CONFIG_FTRACE_RECORD_RECURSION）
或者，如果ftrace_ops 上设置了 FTRACE_OPS_FL_RECURSION 标志（如下所述），那么将使用一个辅trampoline 来为回调测试递归，无需进行递归测试。但这代价是来自额外函数调用的略微更多开销
如果你的回调访问任何需RCU 保护的数据或临界区，最好确RCU 正在“监视”，否则该数据或临界区将不会按预期受到保护。在这种情况下添加：

	if (!rcu_is_watching())
		return;

或者，如果ftrace_ops 上设置了 FTRACE_OPS_FL_RCU 标志（如下所述），那么将使用一个辅trampoline 来为回调测试 rcu_is_watching，无需进行其他测试。但这代价是来自额外函数调用的略微更多开销
## ftrace 标志

ftrace_ops 标志都在 include/linux/ftrace.h 中定义和记录。其中一些标志用ftrace 的内部基础设施，但用户应当了解的标志如下：

FTRACE_OPS_FL_SAVE_REGS
	如果回调需要读取或修改传递给回调pt_regs，则必须设置此标志。在不支持将 pt_regs 传递给回调的架构上，注册带有此标志ftrace_ops 将失败
FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED
	类似SAVE_REGS，但在不支持传regs 的架构上注册 ftrace_ops 不会因设置了此标志而失败。但回调必须检regs 是否NULL 以确定该架构是否支持
FTRACE_OPS_FL_RECURSION
	默认情况下，期望回调能够处理递归。但如果回调不太担心开销，那么设置此位将通过调用一个辅助函数来为回调添加递归保护，该辅助函数执行递归保护，并且仅在不递归时才调用回调
	注意，如果未设置此标志，且发生了递归，可能导致系统崩溃，并可能通过三重错误（triple fault）重启
	注意，如果设置了此标志，那么回调将始终在禁用抢占的情况下被调用。如果未设置，则回调有可能（但不保证）在可抢占上下文中被调用
FTRACE_OPS_FL_IPMODIFY
	需要设FTRACE_OPS_FL_SAVE_REGS。如果回调要“劫持”被追踪的函数（用另一个函数代替被追踪的函数调用），则需要设置此标志。这就是 live kernel patches（内核热补丁）所用的。没有此标志，pt_regs->ip 无法被修改
	注意，任何给定函数一次只能注册一个设置了 FTRACE_OPS_FL_IPMODIFY ftrace_ops
FTRACE_OPS_FL_RCU
	如果设置了此标志，那么回调将只被 RCU 正在“监视”的函数调用。如果回调函数执行任rcu_read_lock() 操作，则需要此标志
	RCU 在系统进入空闲、CPU 被取下和重新上线，以及从内核进入用户空间再回到内核空间时停止监视。在这些转换期间，回调可能被执行，RCU 同步不会保护它
FTRACE_OPS_FL_PERMANENT
        如果在任ftrace ops 上设置了此标志，那么通过proc sysctl ftrace_enabled 写入 0 无法禁用追踪。同样地，如ftrace_enabled 0，则无法注册设置了该标志的回调
        Livepatch 使用它以避免丢失函数重定向，从而系统保持受保护
## 过滤要追踪的函数

如果回调只从特定函数调用，则必须设置过滤器。过滤器按名称添加，如果已知也可ip 添加
   int ftrace_set_filter(struct ftrace_ops **ops, unsigned char **buf,
                         int len, int reset);

@ops
	 用于设置过滤器的 ops

@buf
	 持有函数过滤文本的字符串@len
	 字符串的长度
@reset
	 非零表示在应用此过滤器之前重置所有过滤器
过滤器表示在启用追踪时应启用哪些函数。如@buf NULL 且设置了 reset，则所有函数都将被启用以供追踪
@buf 也可以是 glob 表达式，以启用所有匹配特定模式的函数
请参Documentation/trace/ftrace.rst 中的 Filter Commands（过滤命令）
要仅追踪 schedule 函数
   ret = ftrace_set_filter(&ops, "schedule", strlen("schedule"), 0);

要添加更多函数，多次调用 ftrace_set_filter()，将 @reset 参数设为 0。要移除当前的过滤器集并@buf 定义的新函数替换它，@reset 设为非零
要移除所有被过滤的函数并追踪所有函数：

   ret = ftrace_set_filter(&ops, NULL, 0, 1);

有时多个函数具有相同的名称。要在这种情况下追踪特定函数，可以使ftrace_set_filter_ip()
   ret = ftrace_set_filter_ip(&ops, ip, 0, 0);

尽管 ip 必须是函数内调用 fentry mcount 的地址所在位置。此函数perf kprobes 使用，它们从用户（通常使用内核的调试信息）获取 ip 地址
如果使用 glob 设置过滤器，函数可以被添加到一个“notrace”列表，该列表将阻止这些函数调用回调“notrace”列表优先于“filter”列表。如果两个列表都非空且包含相同的函数，则任何函数都不会调用回调
空的“notrace”列表表示允许过滤器定义的所有函数被追踪
   int ftrace_set_notrace(struct ftrace_ops **ops, unsigned char **buf,
                          int len, int reset);

这接受与 ftrace_set_filter() 相同的参数，但会将它找到的函数添加到不被追踪的列表中。这是与过滤器列表分开的列表，并且此函数不会修改过滤器列表
非零@reset 将在把匹@buf 的函数添加到其中之前清除“notrace”列表
清除“notrace”列表与清除过滤器列表相
  ret = ftrace_set_notrace(&ops, NULL, 0, 1);

过滤器和 notrace 列表可以随时更改。如果只应有一组函数调用回调，最好在注册回调之前设置过滤器。但更改也可能在回调注册之后发生
如果过滤器已就位，且 @reset 非零，且 @buf 包含匹配函数glob，则切换将在 ftrace_set_filter() 调用期间发生。任何时刻都不会有所有函数都调用回调
   ftrace_set_filter(&ops, "schedule", strlen("schedule"), 1);

   register_ftrace_function(&ops);

   msleep(10);

   ftrace_set_filter(&ops, "try_to_wake_up", strlen("try_to_wake_up"), 1);

与以下不同：

   ftrace_set_filter(&ops, "schedule", strlen("schedule"), 1);

   register_ftrace_function(&ops);

   msleep(10);

   ftrace_set_filter(&ops, NULL, 0, 1);

   ftrace_set_filter(&ops, "try_to_wake_up", strlen("try_to_wake_up"), 0);

因为后者在重置时间和新过滤器设置时间之间会有一个短暂的时间段，所有函数都会调用回调