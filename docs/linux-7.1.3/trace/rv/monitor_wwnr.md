## 监控WWR


- 名称：wwrn - 未运行时唤醒
- 类型：每任务确定性自动机
- 作者：丹尼尔·布里斯托·德·奥利维拉<bristot@kernel.org>

### 描述


这是一个每任务示例监视器，具有以下内容
```

               |
               |
               v
    wakeup   +-------------+
  +--------- |             |
  |          | not_running |
  +--------> |             | <+
             +-------------+  |
               |              |
               | switch_in    | switch_out
               v              |
             +-------------+  |
             |   running   | -+
             +-------------+

```
这个模型被打破了，原因是一个任务可以运
在处理器中，而不被设置为 RUNNABLE。想想一
```

  1:      set_current_state(TASK_UNINTERRUPTIBLE);
  2:      schedule();

```
然后想象一IRQ 发生在第一行和第二行之间，
唤醒任务BOOM，任务执行时会发生唤
跑步

- 那么为什么我们需要这个模型呢
- 测试反应堆

### 规格

工具/验证/模型/wwnr.dot 中的 Grapviz 点文
