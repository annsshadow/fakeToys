## 截止时间监视器（Deadline monitors

- 名称：deadline
- 类型：多个监视器的容- 作者：Gabriele Monaco <gmonaco@redhat.com>

### 描述


deadline 监视器是一组用于描述截止时间调度器（deadline scheduler）行为的
规范。它包含针对每个调度实体（截止时间任务与服务器）的监视器，这些监视器
独立工作，以验证截止时间调度器应当遵循的不同规范
### 规范


#### 监视nomiss


nomiss 监视器确dl 实体在其截止时间**之前**得以运行**并且**运行至完成，
尽管可延迟（deferrable）服务器可能不运行。如果一个实`throttled`
（无论是因为它主动让步还是用完了其运行时间），或者当它主动开`sleeping`
时，即被视为完成该监视器包含一个用户可配置的截止时间阈值。如果截止时间任务的总利用率
大于 1，则它们仅保证有界延迟（bounded tardiness）。更多细节请参见
Documentation/scheduler/sched-deadline.rst。可以将阈值（模块参数
`nomiss.deadline_thresh`）配置为避免监视器基于系统中可接受的延迟而失败由于 `dl_throttle` 是实体完成的合法结果，除`HRTICK_DL` 调度器特性处于活动状态，否则要考虑节流延迟，最小延迟需要为 1 tick
服务器还有一个中间的 `idle` 状态，在没有任何可运行任务（从 ready running且未施加时序约束时立即出现。服务器通过停止进入休眠，没有等效的唤醒因为服务器启动与补充的顺序未定义，因此一```

                                  |
  sched_wakeup                    v
  dl_replenish;reset(clk) -- #=========================#
               |             H                         H dl_replenish;reset(clk)
               +-----------> H                         H <--------------------+
                             H                         H                      |
      +- dl_server_stop ---- H          ready          H                      |
      |  +-----------------> H   clk < DEADLINE_NS()   H   dl_throttle;       |
      |  |                   H                         H     is_defer == 1    |
      |  | sched_switch_in - H                         H -----------------+   |
      |  |   |               #=========================#                  |   |
      |  |   |                       |            ^                       |   |
      |  |   |             dl_server_idle    dl_replenish;reset(clk)      |   |
      |  |   |                       v            |                       |   |
      |  |   |                      +--------------+                      |   |
      |  |   |              +------ |              |                      |   |
      |  |   |     dl_server_idle   |              | dl_throttle          |   |
      |  |   |              |       |     idle     | -----------------+   |   |
      |  |   |              +-----> |              |                  |   |   |
      |  |   |                      |              |                  |   |   |
      |  |   |                      |              |                  |   |   |
   +--+--+---+--- dl_server_stop -- +--------------+                  |   |   |
   |  |  |   |                       |           ^                    |   |   |
   |  |  |   |            sched_switch_in    dl_server_idle           |   |   |
   |  |  |   |                       v           |                    |   |   |
   |  |  |   |      +---------- +---------------------+               |   |   |
   |  |  |   | sched_switch_in  |                     |               |   |   |
   |  |  |   | sched_wakeup     |                     |               |   |   |
   |  |  |   | dl_replenish;    |      running        | -------+      |   |   |
   |  |  |   |      reset(clk)  | clk < DEADLINE_NS() |        |      |   |   |
   |  |  |   |      +---------> |                     | dl_throttle   |   |   |
   |  |  |   +----------------> |                     |        |      |   |   |
   |  |  |                      +---------------------+        |      |   |   |
   |  | sched_wakeup                ^   sched_switch_suspend   |      |   |   |
   v  v dl_replenish;reset(clk)     |   dl_server_stop         |      |   |   |
 +--------------+                   |   |                      v      v   v   |
 |              | - sched_switch_in +   |                     +---------------+
 |              | <---------------------+     dl_throttle +-- |               |
 |   sleeping   |                            sched_wakeup |   |   throttled   |
 |              | -- dl_server_stop        dl_server_idle +-> |               |
 |              |    dl_server_idle     sched_switch_suspend  +---------------+
 +--------------+ <---------+                                        ^
        |                                                            |
        +------ dl_throttle;is_constr_dl == 1 || is_defer == 1 ------+

```
