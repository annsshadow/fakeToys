
## Fprobe - 函数入口/出口探针

## 简介

Fprobe 是一种基于 ftrace 中 function-graph 追踪特性的函数入口/出口探针。
如果你不想追踪所有函数，而是想在特定函数的入口和出口附加回调（类似于 kprobes 和
kretprobes），可以使用 fprobe。与 kprobes 和 kretprobes 相比，fprobe 通过单个
处理函数为多个函数提供更快速的检测。本文档描述如何使用 fprobe。

## fprobe 的用法

fprobe 是 ftrace（加上类似 kretprobe 的返回回调）的一个封装，用于向多个函数的
入口和出口附加回调。用户需要设置好 `struct fprobe` 并将其传递给 `register_fprobe()`。

通常，`fprobe` 数据结构会像下面这样用 `entry_handler` 和/或 `exit_handler` 进行
初始化。

 struct fprobe fp = {
        .entry_handler  = my_entry_callback,
        .exit_handler   = my_exit_callback,
 };

要启用 fprobe，可以调用 register_fprobe()、register_fprobe_ips() 和
register_fprobe_syms() 中的一个。这些函数使用不同类型的参数来注册 fprobe。

register_fprobe() 通过函数名过滤器来启用 fprobe。
```

  register_fprobe(&fp, "func*", "func2");

```
register_fprobe_ips() 通过 ftrace 位置地址来启用 fprobe。例如：


  unsigned long ips[] = { 0x.... };

  register_fprobe_ips(&fp, ips, ARRAY_SIZE(ips));

而 register_fprobe_syms() 通过符号名来启用 fprobe。例如：


  char syms[] = {"func1", "func2", "func3"};

  register_fprobe_syms(&fp, syms, ARRAY_SIZE(syms));

```

  unregister_fprobe(&fp);

```
```

  disable_fprobe(&fp);

```
```

  enable_fprobe(&fp);

```
```

  #include <linux/fprobe.h>

```
与 ftrace 相同，已注册的回调会在 register_fprobe() 被调用之后、返回之前的某个时刻
开始被调用。参见 Documentation/trace/ftrace.rst。

此外，unregister_fprobe() 会保证，在它返回之后，enter 和 exit 处理函数都不再被
函数调用，这与 unregister_ftrace_function() 相同。

## fprobe 入口/出口处理函数

入口/出口回调函数的原型如下：

 int entry_callback(struct fprobe **fp, unsigned long entry_ip, unsigned long ret_ip, struct ftrace_regs **fregs, void *entry_data);

 void exit_callback(struct fprobe **fp, unsigned long entry_ip, unsigned long ret_ip, struct ftrace_regs **fregs, void *entry_data);

注意，@entry_ip 在函数入口处被保存，并传递给 exit 处理函数。
如果入口回调函数返回 !0，则相应的 exit 回调将被取消。

@fp
        这是与此处理函数相关的 `fprobe` 数据结构的地址。
        你可以将 `fprobe` 嵌入到自己的数据结构中，并通过 container_of() 宏从
        @fp 获取它。@fp 绝不能为 NULL。

@entry_ip
        这是被追踪函数的 ftrace 地址（入口和出口都是）。注意，这可能不是函数的
        实际入口地址，而是 ftrace 进行检测的位置地址。

@ret_ip
        这是被追踪函数将返回到的地址，位于调用者某处。它可以在入口和出口处都使用。

@fregs
        这是入口和出口处的 `ftrace_regs` 数据结构。它包含函数参数或返回值。因此
        用户可以通过适当的 `ftrace_regs_*` API 来访问这些值。

@entry_data
        这是一个用于在入口和出口处理函数之间共享数据的本地存储。默认情况下该存储
        为 NULL。如果用户在注册 fprobe 时指定了 `exit_handler` 字段和 `entry_data_size`
        字段，则会分配该存储，并传递给 `entry_handler` 和 `exit_handler`。

## 入口数据大小与同一函数上的出口处理函数

由于入口数据是通过每任务栈传递的，且大小有限，每个探针的入口数据大小被限制为
`15 * sizeof(long)`。你还需要注意，当不同的 fprobe 探测同一个函数时，这个限制会
变得更小。入口数据大小按 `sizeof(long)` 对齐，每个带有 exit 处理函数的 fprobe
会在栈上使用 `sizeof(long)` 大小的空间，因此你应让同一个函数上的 fprobe 数量
尽可能少。

## 与 kprobes 共享回调

由于 fprobe（和 ftrace）的递归安全性与 kprobes 略有不同，如果用户希望从 fprobe
和 kprobes 运行相同的代码，这可能会引发问题。

Kprobes 有一个每 CPU 的 'current_kprobe' 变量，它在所有情况下都保护 kprobe 处理
函数免受递归。另一方面，fprobe 只使用 ftrace_test_recursion_trylock()。这允许在
fprobe 用户处理函数运行时，中断上下文调用另一个（或同一个）fprobe。

如果公共回调代码自身具有递归检测，或者能够处理不同上下文（普通/中断/NMI）中的
递归，这就不是问题。但如果它依赖于 'current_kprobe' 递归锁，则必须检查
kprobe_running() 并使用 kprobe_busy_*() API。

Fprobe 提供了 FPROBE_FL_KPROBE_SHARED 标志来实现这一点。如果你的公共回调代码将
与 kprobes 共享，请在注册 fprobe **之前**设置 FPROBE_FL_KPROBE_SHARED，例如：


 fprobe.flags = FPROBE_FL_KPROBE_SHARED;

 register_fprobe(&fprobe, "func*", NULL);

这将保护你的公共回调免受嵌套调用。

## 未命中计数器

**`fprobe`** 数据结构拥有与 kprobes 相同的 `: nmissed` 计数器字段。
当以下情况发生时，该计数器会递增：

 - fprobe 未能获取 ftrace_recursion 锁。这通常意味着从 entry_handler 中调用了
   被其他 ftrace 用户追踪的函数。

 - fprobe 由于无法从每任务影子栈分配数据缓冲区，而未能设置函数出口。

**`fprobe`** 的 `: nmissed` 字段在上述两种情况下都会递增。因此，前者会跳过入口和
出口回调，后者会跳过出口回调，但在两种情况下计数器都会加 1。

注意，如果你在注册 fprobe 时将 FTRACE_OPS_FL_RECURSION 和/或 FTRACE_OPS_FL_RCU
设置到 **`fprobe`** 的 `ops::flags`（ftrace_ops::flags），该计数器可能无法正确
工作，因为 ftrace 会跳过用于递增该计数器的 fprobe 函数。

## 函数与结构体
