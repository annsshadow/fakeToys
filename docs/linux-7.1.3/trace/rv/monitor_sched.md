## 调度器监视器


- 名称：sched
- 类型：多个监视器的容- 作者：Gabriele Monaco <gmonaco@redhat.com>, Daniel Bristot de Oliveira <bristot@kernel.org>

### 描述


描述复杂系统（例如调度器）的监视器很容易变得难以理解，因为可能的状态转换太多通常可以将这类描述拆分为更小的监视器，共享部分或全部事件。事实上，并发启用这些更小的监视器，就相当于用一个单一的大监视器来测试系统将模型拆分为多个规格不仅更容易理解，而且在出错时能提供更多线索
sched 监视器是一组用于描述调度器行为的规格。它包含多个CPU 和按任务的监视器，各自独立工作以验证调度器应遵循的不同规格
为了让这一系统尽可能直观，sched 的规格是**嵌套**的监视器，sched 本身*容器**从接口角度看，sched 将其他监视器作为子目录包含在内，sched 启用/禁用或设置反应（reactor），会将变更传播到所有监视器，但单个监视器也可以独立使用
重要的是，未来的模块应在其容器（本例中为 sched）之后构建，否则链接器将不遵守顺序，嵌套也无法按预期工作为此，只需Makefile 中将它们加在 sched 之后
### 规格


sched 中包含的规格目前仍是进行中的工作，正在适配 Daniel Bristot [^1^] 中定义的那些
目前我们包含了以下内容：

#### 监视sco


调度上下文操作（sco）监视器确保任务状态的变化
```

                        |
                        |
                        v
    sched_set_state   +------------------+
  +------------------ |                  |
  |                   |  thread_context  |
  +-----------------> |                  | <+
                      +------------------+  |
                        |                   |
                        | schedule_entry    | schedule_exit
                        v                   |
                                            |
                       scheduling_context  -+

```
#### 监视snroc


在自身上下文设置不可运行（snroc）监视器确保任务状态的变化仅发生在相应任务的上下文中。这是一个按任务```

                        |
                        |
                        v
                      +------------------+
                      |  other_context   | <+
                      +------------------+  |
                        |                   |
                        | sched_switch_in   | sched_switch_out
                        v                   |
    sched_set_state                         |
  +------------------                       |
  |                       own_context       |
  +----------------->                      -+

```
#### 监视scpd


在禁用抢占的情况下调schedule（scpd）监视器确保 schedule ```

                       |
                       |
                       v
                     +------------------+
                     |    cant_sched    | <+
                     +------------------+  |
                       |                   |
                       | preempt_disable   | preempt_enable
                       v                   |
    schedule_entry                         |
    schedule_exit                          |
  +-----------------      can_sched        |
  |                                        |
  +---------------->                      -+

```
#### 监视snep


schedule 不启用抢占（snep）监视器确保 schedule 调用
```

                        |
                        |
                        v
    preempt_disable   +------------------------+
    preempt_enable    |                        |
  +------------------ | non_scheduling_context |
  |                   |                        |
  +-----------------> |                        | <+
                      +------------------------+  |
                        |                         |
                        | schedule_entry          | schedule_exit
                        v                         |
                                                  |
                          scheduling_contex      -+

```
#### 监视sts


schedule 意味着任务切换（sts）监视器确保任务切换仅发生在调度上下文中、且至多发生一次，同时调度在中断启用时发生，但在中断被禁用之前不会发生任务切换。当为执行挑选的下一个任务与之前
```

    irq_entry                      |
     +----+                        |
     v    |                        v
 +------------+ irq_enable    #===================#   irq_disable
 |            | ------------> H                   H   irq_entry
 | cant_sched | <------------ H                   H   irq_enable
 |            | irq_disable   H     can_sched     H --------------+
 +------------+               H                   H               |
                              H                   H               |
            +---------------> H                   H <-------------+
            |                 #===================#
            |                   |
      schedule_exit             | schedule_entry
            |                   v
            |   +-------------------+     irq_enable
            |   |    scheduling     | <---------------+
            |   +-------------------+                 |
            |     |                                   |
            |     | irq_disable                    +--------+  irq_entry
            |     v                                |        | --------+
            |   +-------------------+  irq_entry   | in_irq |         |
            |   |                   | -----------> |        | <-------+
            |   | disable_to_switch |              +--------+
            |   |                   | --+
            |   +-------------------+   |
            |     |                     |
            |     | sched_switch        |
            |     v                     |
            |   +-------------------+   |
            |   |     switching     |   | irq_enable
            |   +-------------------+   |
            |     |                     |
            |     | irq_enable          |
            |     v                     |
            |   +-------------------+   |
            +-- |  enable_to_exit   | <-+
                +-------------------+
                  ^               | irq_disable
                  |               | irq_entry
                  +---------------+ irq_enable

```
### 监视nrp


需要重调度抢占（nrp）监视器确保抢占需`need_resched`。只考虑内核抢占，因为对本监视器而言，返回用户空间时的抢占与 `sched_switch_yield`（在 sssw 监视器中描述）无法区分内核抢占是指 `__schedule` 以抢占标志设true 被调用时（例如来preempt_enable 或从中断退出）。这种抢占发生在设置了重调度需求之后这不适用于该标志*惰性（lazy*变体，它只会导致用户空间抢占`schedule_entry_preempt` 可能涉及也可能不涉及任务切换，在后一种情况下，任务从抢占上下文经过调度器，但被选为下一个要运行的任务。由于调度器运行，这会清除重调度需求。`any_thread_running` 状态并不意味着被监视的任务没有在运行，因为本监视器不跟踪调度的结果
理论上，抢占只能在设置了 `need_resched` 标志之后发生。但在实践中，有可能看到标志未设置时的抢```

  need_resched
                   preempt_schedule()
                                           preempt_schedule_irq()
                                                   __schedule()
  !need_resched
                           __schedule()

```
在上述情形中，标准抢占开始（例如在标志设置时来自 preempt_enable），调度前发生了中断，并且在它的退出路径上进行了调度，从而清除了 `need_resched` 标志当被抢占的任务再次运行时，早先开始的标准抢占会恢复，尽管标志已不再设置。监视器将此视为 `nested_preemption`（嵌套抢占），这允许在不重新设置标志的情况下再次抢占。此条件放宽了监视器的约束，可能捕获到假阴性（即没有真正的 `nested_preemptions`），但使监视器更加健壮，并能够验证其他场景为简单起见，监视器从 `preempt_irq` 开始，尽管没有中断
```

    schedule_entry
    irq_entry                 #===========================================#
  +-------------------------- H                                           H
  |                           H                                           H
  +-------------------------> H             any_thread_running            H
                              H                                           H
  +-------------------------> H                                           H
  |                           #===========================================#
  | schedule_entry              |                       ^
  | schedule_entry_preempt      | sched_need_resched    | schedule_entry
  |                             |                      schedule_entry_preempt
  |                             v                       |
  |                           +----------------------+  |
  |                      +--- |                      |  |
  |   sched_need_resched |    |     rescheduling     | -+
  |                      +--> |                      |
  |                           +----------------------+
  |                             | irq_entry
  |                             v
  |                           +----------------------+
  |                           |                      | ---+
  |                      ---> |                      |    | sched_need_resched
  |                           |      preempt_irq     |    | irq_entry
  |                           |                      | <--+
  |                           |                      | <--+
  |                           +----------------------+    |
  |                             | schedule_entry          | sched_need_resched
  |                             | schedule_entry_preempt  |
  |                             v                         |
  |                           +-----------------------+   |
  +-------------------------- |    nested_preempt     | --+
                              +-----------------------+
                                ^ irq_entry         |
                                +-------------------+

```
由于抢占计数上的 `need_resched` 标志arm64 上的工作方式，本监视器在该架构上不稳定，因为它经常在该标志未设置时记录抢占，即便存在上述变通方案也是如此目前，该监视器在 arm64 上默认禁用
### 监视sssw


设置睡眠状态与唤醒（sssw）监视器确保 `set_state` 为可睡眠（sleepable）会导致睡眠，且睡眠中的任务需要被唤醒。它包含以下类型的切换：

- `switch_suspend`  任务让自己进入睡眠，这只能在显式将任务设`sleepable` 之后发生。任务被挂起后，需要被唤醒（`waking` 状态）才能再次被切换进来  如果在切换之前任务被唤醒或设`runnable`，将任务状态设`sleepable` 可以被撤销- `switch_blocking`  `switch_suspend` 的一种特殊情况，任务正在等待一个睡眠中RT 锁（`PREEMPT_RT` 专用），常见的情况是唤醒与设置状态事件相互竞争，导致模型在任务未被设为可睡眠时感知到这种切换。这是模型在 SMP 系统上的局限，变通方案可能拖慢系统- `switch_preempt`  由于内核抢占（nrp 模型中的 `schedule_entry_preempt`）导致的任务切换- `switch_yield`  任务显式调用调度器，或在返回用户空间时被抢占。它可以`yield` 系统调用之后、从 idle 任务发出，或在设置了 `need_resched` 标志时发生。根据定义，任务`sleepable` 时不yield，因为那将是挂起。yield 的一种特殊情况发生在处于 `TASK_INTERRUPTIBLE` 的任务在有待处理信号时调用调度器。任务不会经过通常的阻唤醒，而是被设runnable，由此产生的切换（如果有的话）看起来像到 `signal_wakeup` 状态的 yield，随后是信号投递。从此状态起，监视器期望一个信号，即使它看到一个唤醒事件（尽管并非必须），以排除假阴性
本监视器不包running 状态，`sleepable` `runnable` 仅指任务的期望状态，任务可能被调度出去（例如由于抢占）。不过，它确实包含事`sched_switch_in` 来表示任务何时被允许变为运行。这也可能由抢占触发，但在任务进入之后不能再发生
```

   +--------------------------------------------------------------------------+
   |                                                                          |
   |                                                                          |
   | switch_suspend           |                                               |
   | switch_blocking          |                                               |
   v                          v                                               |
 +----------+              #==========================#   set_state_runnable  |
 |          |              H                          H   wakeup              |
 |          |              H                          H   switch_in           |
 |          |              H                          H   switch_yield        |
 | sleeping |              H                          H   switch_preempt      |
 |          |              H                          H   signal_deliver      |
 |          |  switch_     H                          H ------+               |
 |          |  _blocking   H         runnable         H       |               |
 |          | <----------- H                          H <-----+               |
 +----------+              H                          H                       |
   |   wakeup              H                          H                       |
   +---------------------> H                          H                       |
                           H                          H                       |
               +---------> H                          H                       |
               |           #==========================#                       |
               |             |                ^                               |
               |             |                | set_state_runnable            |
               |             |                | wakeup                        |
               |    set_state_sleepable       |      +------------------------+
               |             v                |      |
               |           +--------------------------+  set_state_sleepable
               |           |                          |  switch_in
               |           |                          |  switch_preempt
   signal_deliver          |        sleepable         |  signal_deliver
               |           |                          | ------+
               |           |                          |       |
               |           |                          | <-----+
               |           +--------------------------+
               |             |                ^
               |        switch_yield          | set_state_sleepable
               |             v                |
               |           +---------------+  |
               +---------- | signal_wakeup | -+
                           +---------------+
                             ^           | switch_in
                             |           | switch_preempt
                             |           | switch_yield
                             +-----------+ wakeup

```
### 监视opid


在禁用抢占与中断情况下进行操作（opid）监视器确保 `wakeup` `need_resched` 这类操作在中断与抢占都被禁用的情况下发生`need_resched` 可以由某RCU 内部函数设置，此时它不匹配任务唤醒，并且可能仅在中断被禁用时发生中断与抢占状态由混合自动机验```

   |
   |
   v
 #=========#   sched_need_resched;irq_off == 1
 H         H   sched_waking;irq_off == 1 && preempt_off == 1
 H   any   H ------------------------------------------------+
 H         H                                                 |
 H         H <-----------------------------------------------+
 #=========#

```
### 参考资

[^1^] - https://bristot.me/linux-task-model
