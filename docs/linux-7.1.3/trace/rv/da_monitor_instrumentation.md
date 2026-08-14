## 确定性自动机插桩


dot2k 创建的 RV 监视器文件，名为 "$MODEL_NAME.c"，包含一个专门用于插桩（instrumentation）
的节。

```

  /*
   * This is the instrumentation part of the monitor.
   *
   * This is the section where manual work is required. Here the kernel events
   * are translated into model's event.
   *
   */
  static void handle_preempt_disable(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(preempt_disable_wip);
  }

  static void handle_preempt_enable(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(preempt_enable_wip);
  }

  static void handle_sched_waking(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(sched_waking_wip);
  }

  static int enable_wip(void)
  {
	int retval;

	retval = da_monitor_init_wip();
	if (retval)
		return retval;

	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_disable);
	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_enable);
	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_sched_waking);

	return 0;
  }

```
该节顶部的注释解释了总体思路：插桩节把**内核事件**翻译成*模型的事件*。

### 跟踪回调函数


前三个函数是来自 wip 模型的三个事件各自的回调*处理函数*的起点。开发者不一定需要使用它们：
它们只是起点。

```

 void handle_preempt_disable(void *data, /* XXX: fill header */)
 {
        da_handle_event_wip(preempt_disable_wip);
 }

```
来自模型的 preempt_disable 事件直接连接到 preemptirq:preempt_disable。preemptirq:preempt_disable
事件
```

  TP_PROTO(unsigned long ip, unsigned long parent_ip)

```
```

  void handle_preempt_disable(void *data, unsigned long ip, unsigned long parent_ip)

```
在这种情况下，内核事件与自动机事件一一对应，确实，该函数不需要做其它修改。

下一个处理函数 handle_preempt_enable() 具有与 handle_preempt_disable() 相同的参数列表。
区别在于 preempt_enable 事件将用于把系统同步到模型。

最初，**模型**被置于初始状态。然而，**系统**可能在也可能不在初始状态。监视器在知道系统已
到达初始状态之前不能开始处理事件。否则，监视器和系统可能失步。

查看自动机定义，可以看到系统和模型预期在 preempt_enable 执行后返回到初始状态。因此，它
可以在监视节的初始化时用于把系统和模型同步。

开始通过一个特殊的 handle 函数告知，
```

  da_handle_start_event_wip(preempt_enable_wip);

```
```

  void handle_preempt_enable(void *data, unsigned long ip, unsigned long parent_ip)
  {
        da_handle_start_event_wip(preempt_enable_wip);
  }

```
```

  void handle_sched_waking(void *data, struct task_struct *task)
  {
        da_handle_event_wip(sched_waking_wip);
  }

```
而解释则留给读者作为练习。

### enable 和 disable 函数


```

  enable_$(MONITOR_NAME)()
  disable_$(MONITOR_NAME)()

```
这些函数分别在监视器被启用和禁用时调用。

它们应当用于把插桩**附加（attach）**和**分离（detach）**到运行中的系统。开发者必须在相应的
函数中添加将其监视器**附加**和**分离**到系统所需的一切。

```

 enable_wip()
 disable_wip()

```
但不需要做修改，因为：默认情况下，这些函数**附加**和**分离** tracepoints_to_attach，这对于
此情况已经足够。

### 插桩辅助函数


为了完成插桩，在监视启用阶段，需要把**处理函数**附加到一个内核事件。

RV 接口也简化了这一步。例如，宏 "rv_attach_trace_probe()" 用于把 wip 模型事件连接到
相应的内核事件。dot2k 会自动为每个模型事件在启用阶段添加 "rv_attach_trace_probe()" 函数
调用，作为建议。

```

  static int enable_wip(void)
  {
        int retval;

        retval = da_monitor_init_wip();
        if (retval)
                return retval;

        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_enable);
        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_sched_waking);
        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_disable);

        return 0;
  }

```
然后这些探针需要在禁用阶段被分离。

[^1^] wip 模型在以下文档中给出：

  Documentation/trace/rv/deterministic_automata.rst

wip 监视器在以下文档中给出：

  Documentation/trace/rv/monitor_synthesis.rst
