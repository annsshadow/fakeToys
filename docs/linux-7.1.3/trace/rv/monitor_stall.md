## Monitor stall（停滞任务监视器


- 名称：stall - 停滞任务监视
- 类型：每任务混合自动
- 作者：Gabriele Monaco <gmonaco@redhat.com>

### 描述


停滞任务（stall）监视器是一个示例性的每任务定时监视器，用于检
```

                        |
                        |
                        v
                      #==========================#
  +-----------------> H         dequeued         H
  |                   #==========================#
  |                     |
 sched_switch_wait      | sched_wakeup;reset(clk)
  |                     v
  |                   +--------------------------+ <+
  |                   |         enqueued         |  | sched_wakeup
  |                   | clk < threshold_jiffies  | -+
  |                   +--------------------------+
  |                     |                 ^
  |              sched_switch_in    sched_switch_preempt;reset(clk)
  |                     v                 |
  |                   +--------------------------+
  +------------------ |         running          |
                      +--------------------------+
                        ^ sched_switch_in      |
                        | sched_wakeup         |
                        +----------------------+

```
阈值可作为一个参数进行配置，既可以通过在内核启动时传入
`stall.threshold_jiffies=<新` 参数，也可以
`/sys/module/stall/parameters/threshold_jiffies` 写入新值

### 规格说明

Graphviz Dot 文件位于 tools/verification/models/stall.dot
