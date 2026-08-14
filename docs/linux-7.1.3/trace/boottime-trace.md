
## 启动期追踪（Boot-time tracing）


:Author: Masami Hiramatsu <mhiramat@kernel.org>

## 概述


启动期追踪允许用户在启动阶段（包括设备初始化）进行追踪，并可使用 ftrace 的全部功能，
包括按事件的过滤与动作、直方图、kprobe 事件（kprobe-events）与合成事件
（synthetic-events），以及追踪实例（trace instances）。
由于内核命令行不足以控制这些复杂的功能，这里使用 bootconfig 文件来描述追踪功能的
编程配置。

## Boot Config 中的选项


以下是启动期追踪在 boot config 文件 [^1^]_ 中可用的选项列表。所有选项都位于 "ftrace."
或 "kernel." 前缀之下。以 "kernel." 前缀开头的选项请参见内核参数 [^2^]_。

### Ftrace 全局选项


Ftrace 全局选项在 boot config 中使用 "kernel." 前缀，这意味着这些选项是作为内核
传统命令行的一部分传入的。

kernel.tp_printk
   同时将追踪事件数据输出到 printk 缓冲区。

kernel.dump_on_oops [= MODE]
   在 Oops 时转储 ftrace。如果 MODE = 1 或省略，则转储所有 CPU 上的追踪缓冲区。
   如果 MODE = 2，则只转储触发 Oops 的那个 CPU 上的缓冲区。

kernel.traceoff_on_warning
   如果发生 WARN_ON()，则停止追踪。

kernel.fgraph_max_depth = MAX_DEPTH
   将 fgraph tracer 的最大深度设为 MAX_DEPTH。

kernel.fgraph_filters = FILTER[, FILTER2...]
   添加 fgraph 追踪的函数过滤器。

kernel.fgraph_notraces = FILTER[, FILTER2...]
   添加 fgraph 非追踪的函数过滤器。

### Ftrace 每实例选项


这些选项可用于每个实例，包括全局 ftrace 节点。

ftrace.[instance.INSTANCE.]options = OPT1[, OPT2[...]]
   启用给定的 ftrace 选项。

ftrace.[instance.INSTANCE.]tracing_on = 0|1
   在启动期追踪开始时，启用/禁用该实例上的追踪。
   （你也可以通过 "traceon" 事件触发动作来启用它）

ftrace.[instance.INSTANCE.]trace_clock = CLOCK
   将 ftrace 的 trace_clock 设为给定的 CLOCK。

ftrace.[instance.INSTANCE.]buffer_size = SIZE
   将 ftrace 缓冲区大小配置为 SIZE。该 SIZE 可以使用 "KB" 或 "MB"。

ftrace.[instance.INSTANCE.]alloc_snapshot
   分配快照缓冲区。

ftrace.[instance.INSTANCE.]cpumask = CPUMASK
   将 CPUMASK 设为追踪的 CPU 掩码。

ftrace.[instance.INSTANCE.]events = EVENT[, EVENT2[...]]
   在启动时启用给定的事件。EVENT 中可以使用通配符。

ftrace.[instance.INSTANCE.]tracer = TRACER
   在启动时将当前 tracer 设为 TRACER。（例如 function）

ftrace.[instance.INSTANCE.]ftrace.filters
   接受一组追踪函数过滤规则。

ftrace.[instance.INSTANCE.]ftrace.notraces
   接受一组非追踪函数过滤规则。

### Ftrace 每事件选项


这些选项用于设置每事件的选项。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.enable
   启用 GROUP:EVENT 的追踪。

ftrace.[instance.INSTANCE.]event.GROUP.enable
   启用 GROUP 内的所有事件追踪。

ftrace.[instance.INSTANCE.]event.enable
   启用所有事件追踪。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.filter = FILTER
   将 FILTER 规则设置到 GROUP:EVENT。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.actions = ACTION[, ACTION2[...]]
   将 ACTION 设置到 GROUP:EVENT。

ftrace.[instance.INSTANCE.]event.kprobes.EVENT.probes = PROBE[, PROBE2[...]]
   基于 PROBEs 定义新的 kprobe 事件。可以在一个事件上定义多个探针，但这些探针
   必须具有相同类型的参数。该选项仅对组名为 "kprobes" 的事件可用。

ftrace.[instance.INSTANCE.]event.synthetic.EVENT.fields = FIELD[, FIELD2[...]]
   用 FIELDs 定义新的合成事件。每个字段应为 "type varname"。

注意，kprobe 与合成事件的定义可以写在实例节点之下，但它们在其他实例中也是可见的。
因此请注意事件名冲突的问题。

### Ftrace 直方图选项


由于将直方图动作作为每事件 action 选项的字符串来写会过长，这里提供了位于每事件
'hist' 子键下的树形选项，用于配置直方图动作。关于每个参数的详细信息，请阅读事件
直方图文档（Documentation/trace/histogram.rst）。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]keys = KEY1[, KEY2[...]]
  设置直方图键参数。（必填）
  'N' 是用于多个直方图的数值字符串。如果该事件上只有一个直方图，可以省略它。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]values = VAL1[, VAL2[...]]
  设置直方图值参数。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]sort = SORT1[, SORT2[...]]
  设置直方图排序参数选项。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]size = NR_ENTRIES
  设置直方图大小（条目数）。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]name = NAME
  设置直方图名称。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]var.VARIABLE = EXPR
  通过 EXPR 表达式定义一个新的 VARIABLE。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<pause|continue|clear>
  设置直方图控制参数。可以设置其中的一个。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmatch.[M.]event = GROUP.EVENT
  设置直方图 'onmatch' 处理器匹配的事件参数。
  'M' 是用于多个 'onmatch' 处理器的数值字符串。如果此直方图上只有一个 'onmatch'
  处理器，可以省略它。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmatch.[M.]trace = EVENT[, ARG1[...]]
  为 'onmatch' 设置直方图 'trace' 动作。
  EVENT 必须是合成事件名，而 ARG1... 是该事件的参数。如果设置了 'onmatch.event'
  选项则为必填。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmax.[M.]var = VAR
  设置直方图 'onmax' 处理器变量参数。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onchange.[M.]var = VAR
  设置直方图 'onchange' 处理器变量参数。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<onmax|onchange>.[M.]save = ARG1[, ARG2[...]]
  为 'onmax' 或 'onchange' 处理器设置直方图 'save' 动作参数。
  如果设置了 'onmax.var' 或 'onchange.var' 选项，则此选项或下面的 'snapshot' 选项为必填。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<onmax|onchange>.[M.]snapshot
  为 'onmax' 或 'onchange' 处理器设置直方图 'snapshot' 动作。
  如果设置了 'onmax.var' 或 'onchange.var' 选项，则此选项或上面的 'save' 选项为必填。

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.filter = FILTER_EXPR
  设置直方图过滤表达式。在 FILTER_EXPR 中不需要写 'if'。

注意，如果每事件的 'actions' 选项包含直方图动作，则该 'hist' 选项可能与其冲突。

## 何时启动


所有以 `ftrace` 开头的启动期追踪选项都会在 core_initcall 结束时启用。这意味着你可以
追踪从 postcore_initcall 开始的事件。大多数子系统和与架构相关的驱动会在那之后初始化
（arch_initcall 或 subsys_initcall）。因此，你可以用启动期追踪来追踪它们。
如果你希望在 core_initcall 之前追踪事件，可以使用以 `kernel` 开头的选项。其中部分
选项会比 initcall 处理更早启用（例如 `kernel.ftrace=function` 和 `kernel.trace_event`
会在 initcall 之前启动）。

## 示例


例如，要为每个事件添加过滤器和动作、定义 kprobe 事件以及带直方图的合成事件，可以编写
如下 boot config
```

  ftrace.event {
        task.task_newtask {
                filter = "pid < 128"
                enable
        }
        kprobes.vfs_read {
                probes = "vfs_read $arg1 $arg2"
                filter = "common_pid < 200"
                enable
        }
        synthetic.initcall_latency {
                fields = "unsigned long func", "u64 lat"
                hist {
                        keys = func.sym, lat
                        values = lat
                        sort = lat
                }
        }
        initcall.initcall_start.hist {
                keys = func
                var.ts0 = common_timestamp.usecs
        }
        initcall.initcall_finish.hist {
                keys = func
                var.lat = common_timestamp.usecs - $ts0
                onmatch {
                        event = initcall.initcall_start
                        trace = initcall_latency, func, $lat
                }
        }
  }

```
此外，启动期追踪支持 "instance" 节点，允许我们同时为不同目的运行多个 tracer。例如，
一个 tracer 用于追踪以 "user\_" 开头的函数，另一个追踪
```
  ftrace.instance {
        foo {
                tracer = "function"
                ftrace.filters = "user_*"
        }
        bar {
                tracer = "function"
                ftrace.filters = "kernel_*"
        }
  }

```
实例节点也接受事件节点，因此每个实例可以自定义其事件追踪。

借助触发动作与 kprobe，你可以在某个函数被调用时追踪其函数图（function-graph）。例如，
这将追踪如下代码中的全部函数调用
```
  ftrace {
        tracing_on = 0
        tracer = function_graph
        event.kprobes {
                start_event {
                        probes = "pci_proc_init"
                        actions = "traceon"
                }
                end_event {
                        probes = "pci_proc_init%return"
                        actions = "traceoff"
                }
        }
  }


```
此启动期追踪也通过 boot config 支持 ftrace 内核参数。
```
  trace_options=sym-addr trace_event=initcall:* tp_printk trace_buf_size=1M ftrace=function ftrace_filter="vfs*"

```
```
  kernel {
        trace_options = sym-addr
        trace_event = "initcall:*"
        tp_printk
        trace_buf_size = 1M
        ftrace = function
        ftrace_filter = "vfs*"
  }

```
注意，参数以 "kernel" 前缀而非 "ftrace" 前缀开头。
