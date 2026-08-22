## 运行时验证监视器合成


应用运行时验证（RV）技术的起点是，对受审查系统期望（或不期望）的行为进*规约
（specification**建模（modeling*
接下来，需要将形式化表*合成（synthesized*为一*监视器（monitor*，该监视随后可用于分析系统的 trace。监视器通过一*插桩（instrumentation*连接到系统，插桩将来*系统**的事件转换为**规约**的事件
Linux 的语境中，运行时验证监视器被封装**RV monitor** 抽象内部。RV monitor 包含
监视器的一组实例（CPU 监视器、每任务监视器等）、将监视器与系统参考模型粘合的辅助
函数，以及作为对事件解析和异常的反应trace 输出，如下图所```

 Linux   +---- RV Monitor ----------------------------------+ Formal
  Realm  |                                                  |  Realm
  +-------------------+     +----------------+     +-----------------+
  |   Linux kernel    |     |     Monitor    |     |     Reference   |
  |     Tracing       |  -> |   Instance(s)  | <-  |       Model     |
  | (instrumentation) |     | (verification) |     | (specification) |
  +-------------------+     +----------------+     +-----------------+
         |                          |                       |
         |                          V                       |
         |                     +----------+                 |
         |                     | Reaction |                 |
         |                     +--+--+--+-+                 |
         |                        |  |  |                   |
         |                        |  |  +-> trace output ?  |
         +------------------------|--|----------------------+
                                  |  +----> panic ?
                                  +-------> <user-specified>

```

### RV 监视器合

将规约合成为 Linux **RV monitor** 抽象，由 rvgen 工具和包含创建监视器公共代码的头文件
自动完成。这些头文件为：

  - rv/da_monitor.h，用于确定性自动机（deterministic automaton）监视器  - rv/ltl_monitor.h，用于线性时态逻辑（linear temporal logic）监视器  - rv/ha_monitor.h，用于混合自动机（hybrid automaton）监视器
### rvgen


rvgen 工具将规约转换为 C 表示，并生成 C 语言内核监视器的骨架。例如，可以转换存在```

  $ rvgen monitor -c da -s wip.dot -t per_cpu

```

中的 wip.dot 模型。这将创建一个名wip/ 的目录，包含以下文件
- wip.h：C 语言形式wip 模型
- wip.c：RV monitor

wip.c 文件包含监视器声明以及系统插桩的起始点
类似地，可以用以下命令生成线性时态逻辑监视```

  $ rvgen monitor -c ltl -s pagefault.ltl -t per_task

```

这将生成 pagefault/ 目录，包含：

- pagefault.h：Buchi 自动机（用于验证规约的非确定性状态机- pagefault.c：RV monitor 的骨
### 监视器头文件


头文件：

- `rv/da_monitor.h`，用于确定性自动机监视- `rv/ltl_monitor` 用于线性时态逻辑监视
包含用于实现*监视器实例（Monitor Instance(s)的公共宏和静态函数
将所有公共功能放在单个头文件中的好处有三
  - 减少代码重复  - 便于修复/改进  - 避免开发者为（比如说）以非标准方式操纵模型而改动监视器核心代码的情况
rv/da_monitor.h
+++++++++++++++

这个初始实现提供了三种不同类型的监视器实例：

- `#define RV_MON_TYPE RV_MON_GLOBAL`
- `#define RV_MON_TYPE RV_MON_PER_CPU`
- `#define RV_MON_TYPE RV_MON_PER_TASK`

第一种为全局确定性自动机监视器声明函数，第二种为CPU 实例的监视器，第三种为每任务
实例的监视器
在所有情况下，C 文件必须包含 `rvgen` 生成$(MODEL_NAME).h 文件（例如，要定义每 CPU “wip”监视器，`wip.c` 源文件应
```

  #define RV_MON_TYPE RV_MON_PER_CPU
  #include "wip.h"
  #include <rv/da_monitor.h>

```

监视器通过发送待处理的事件来执行，使用以下函```

  da_handle_event($(event from event enum));
  da_handle_start_event($(event from event enum));
  da_handle_start_run_event($(event from event enum));

```

函数 `da_handle_event()` 是常规情况，即当监视器正在处理事件时会处理该事件
当监视器被启用时，它被置于自动机的初始状态。然而，监视器并不知道系统是否处*初始
状*
`da_handle_start_event()` 函数用于通知监视器系统正在返回初始状态，从而监视器可以开监视下一个事件
`da_handle_start_run_event()` 函数用于通知监视器系统已知处于初始状态，从而监视器可以
开始监视并处理当前事件
wip 模型为例，事"preempt_disable" ```

  da_handle_event(preempt_disable_wip);
  da_handle_event(sched_waking_wip);

```

```

  da_handle_start_event(preempt_enable_wip);

```

用于通知监视器系统将返回初始状态，从而系统与监视器应当保持同步
rv/ltl_monitor.h
++++++++++++++++
该文件必须与 `rvgen` 生成$(MODEL_NAME).h 文件结合才完整。例如，对于 `pagefault`
监视器，`pagefault.c` ```

  #include "pagefault.h"
  #include <rv/ltl_monitor.h>

```

（`rvgen` 生成的骨架监视器文件已经这样做了）
`$(MODEL_NAME).h`（上例中`pagefault.h`）包Buchi 自动机的实现——一个验LTL 规约
的非确定性状态机。`rv/ltl_monitor.h` 包含Buchi 自动机交互并实现 RV 监视器的公共
辅助函数
```

  enum ltl_atom {
      LTL_$(FIRST_ATOMIC_PROPOSITION),
      LTL_$(SECOND_ATOMIC_PROPOSITION),
      ...
      LTL_NUM_ATOM
  };

```

这是 LTL 规约中存在的原子命题（atomic proposition）列表（带有“LTL\_”前缀以避免命名冲突）这个 `enum` 被传递给Buchi 自动机交互的函数
生成代码时，`rvgen` 无法理解原子命题的含义。因此该任务留给人工完成。推荐的做法是，原子命题发生变化的地方添tracepoints，并```

  void ltl_atom_update(struct task_struct *task, enum ltl_atom atom, bool value)

```

中告Buchi 自动机原子命`atom` 现在`value`。Buchi 自动机检LTL 规约是否仍被
满足，并在检测到违例时调用监视器的错tracepoint 和反应器（reactor）
应尽可能地使tracepoints `ltl_atom_update()`。然而，有时这并不是最方便的方式对于在内核多个位置发生变化的某些原子命题，追踪所有这些位置会很麻烦。此外，原子命题精确时刻被更新可能并不重要。例如，考虑以下线性时```

  RULE = always (RT imply not PAGEFAULT)

```

这个 LTL 表示实时任务不会引发页错误（page fault）。对于该规约，当 `PAGEFAULT` 为真时，
`RT` 具有正确的值即可，至于 `RT` 何时改变并不重要。受此情形启发，提供了另一```

  void ltl_atom_fetch(struct task_struct *task, struct ltl_monitor *mon)

```

该函数每Buchi 自动机被触发时调用。因此，
```

  void ltl_atom_fetch(struct task_struct *task, struct ltl_monitor *mon)
  {
      ltl_atom_set(mon, LTL_RT, rt_task(task));
  }

```

实际上，每当通过调用 `ltl_atom_update()` 更新 `PAGEFAULT` 时，`RT` 也会被获取。因此，LTL
规约可以在不追踪各处 `RT` 的情况下被验证
对于表现得像事件的原子命题，它们通常需要在设置（或清除）后立即清除（或设置）。一方便的函数是
```

  void ltl_atom_pulse(struct task_struct *task, enum ltl_atom atom, bool value)

```

```

  ltl_atom_update(task, atom, value);
  ltl_atom_update(task, atom, !value);

```

要初始化原子命题，必须使用以下函```

  ltl_atoms_init(struct task_struct *task, struct ltl_monitor *mon, bool task_creation)

```

当监视器被启用时，该函数为所有运行中的任务调用。它也会为启用监视器后创建的新任务调用它应
```

  void ltl_atom_init(struct task_struct *task, struct ltl_monitor *mon, bool task_creation)
  {
      ltl_atom_set(mon, LTL_RT, rt_task(task));
      if (task_creation)
          ltl_atom_set(mon, LTL_PAGEFAULT, false);
  }

```

未被 `ltl_atom_init()` 初始化的原子命题将停留在未知状态，直到命中相关tracepoints，这
可能需要一些时间。由于在任务的全部原子命题都已知之前无法对其执行监视，监视器可能需一些时间来开始验证在监视器启用之前就已运行的任务。因此，建议在启用监视器之后再启感兴趣的任务
rv/ha_monitor.h
+++++++++++++++

混合自动机监视器的实现直接派生自确定性自动机。尽管使用了不同的头文件（`ha_monitor.h`），
处理事件的函数是相同的（例如 `da_handle_event`）
此外，`rvgen` 工具会根据监视器源文件中的监视器规约，为 `ha_verify_constraint``ha_get_env` `ha_reset_env` 填充骨架
`ha_verify_constraint` 通常开箱即用，因为它由 `rvgen` 生成
```

    res = ha_get_env(ha_mon, ENV) < VALUE;

```

```

    ha_reset_env(ha_mon, ENV);

```

- 状态上的约束使用定时器实现

  - 在进入状态前武装（armed
  - 在进入任何其他状态时取消

  - 如果事件未导致状态改变则保持不变

  - 如果定时器到期但回调未运行则检
  - 可用的实现有 `HA_TIMER_HRTIMER` `HA_TIMER_WHEEL`

    - hrtimer 更精确但可能有更高开销

```

      #define HA_TIMER_TYPE HA_TIMER_HRTIMER

```

约束值可以用不同形式指定
```

    preemptive == 0
    clk < 100ns
    threshold <= 10j

```

```

    clk < MAX_NS

```

```

    clk <= threshold_jiffies

```

```

    clk < MAX_NS()

```

```

    clk <= threshold_jiffies()

```

在所有情况下，`rvgen` 会尝试从名称或单位判断环境变量的类型。例如，`_NS` `_jiffies`
结尾的常量或参数分别被当ns jiffy 粒度的时钟。带有度量单`j` 的字面量jiffies如果指定了时间单位（`ns` `s`），`rvgen` 会将值转换为 `ns`
常量需要由用户定义（但与名称不同，它们不一定需要定义为常量）。参数会被转换为模块参数用户需要提供默认值。函数和宏同样由用户定义，默认情况下它们`ha_monitor` 作为参数，常的用法是通过辅助函数 `ha_get_target(ha_mon)` 从目标（例如每任务监视器中的 task）获所需值
如果 `rvgen` 确定该变量是时钟，它会根据单位提getter resetter。否则，用户需要提适当的定义。通常非时钟的环境变量不会被重置。在这种情况下，`rvgen` 生成的文件中只会
存在 getter 骨架```

  static u64 ha_get_env(struct ha_monitor *ha_mon, enum envs env)
  {
      if (env == preemptible)
          return preempt_count() == 0;
      return ENV_INVALID_VALUE;
  }

```

该函数传`ha_mon` 参数，以备需要存储（对时钟而言就是这种情况），但无需重置的环境变不需要存储，可以忽略该参数。需要存储的环境变量数量`MAX_HA_ENV_LEN` 限制，但该限不适用于其他变量
最后，状态上的约束仅对时钟有效，且只有当约束形如 `clk < N` 时有效。这是因为此类约束是
通过定时器到期实现的。通常时钟变量在武装定时器之前被重置，但不一定非得如此，可用函数处理好这一点。确保任务退出时没有定时器仍在运行，是每任务监视器的责任
默认情况下生成器使用 hrtimer 实现定时器（`HA_TIMER_TYPE` 设为 `HA_TIMER_HRTIMER`），
这能带来更好的响应性但更高的开销。定时器轮（timer wheel，`HA_TIMER_WHEEL`）对于具有多实例（例如每任务）的监视器是一个不错的替代方案，它能在增加延迟的同时实现更低的开销，且
不牺牲精度
### 最后说

有了基于头文件和 rvgen 的监视器合成，开发者的工作应仅限于对系统插桩，从而提升整体方的可信度
[^1^] 关于确定性自动机格式及其转换的细节，请参```

  Documentation/trace/rv/deterministic_automata.rst

```

[^2^] rvgen 会将监视器名称后缀追加到事件枚举上，以避免在导出供 BPF 程序使用的全局
vmlinux.h 时出现变量冲突