## 混合自动机（Hybrid Automata）

混合自动机是确定性自动机（deterministic automata）的一种扩展，文献中有几种混合自动机的定义。这里实现的适配形式被正式记作 G，并定义为一个 7 元组：

        **G** = { **X**, **E**, **V**, **f**, x\ `0`, X\ `m`, **i** }

- **X** 是状态集合；
- **E** 是事件的有限集合；
- **V** 是环境变量的有限集合；
- x\ `0` 是初始状态；
- X\ `m`（**X** 的子集）是标记（或最终）状态的集合。
- **f** : **X** x **E** x **C(V)** -> **X** 是转移函数。
  它定义了在状态 **X** 中发生来自 **E** 的事件时的状态转移。与确定性自动机不同，转移函数还包括来自所有可能约束集合（定义为 **C(V)**）的守卫（guard）。守卫在事件发生时根据 **V** 的取值可以为真或假，并且仅当约束为真时转移才可能。与确定性自动机类似，在 **X** 中的状态发生 **E** 中的事件时，如果守卫为真，则有一个确定性的下一状态来自 **X**。
- **i** : **X** -> **C'(V)** 是不变量赋值函数，这是分配给 **X** 中每个状态的约束，在 **X** 中的每个状态都必须在不变量为假之前离开。对于无论 **V** 取值如何都为真的那些不变量，我们可以省略其表示。

所有可能约束的集合 **C(V)** 根据以下语法定义：

        g = v < c | v > c | v <= c | v >= c | v == c | v != c | g && g | true

其中 v 是 **V** 中的变量，c 是数值。

我们将变量以均匀速率增长的混合自动机的特例定义为时间自动机（timed automata）。在这种情况下，变量被称为时钟（clock）。顾名思义，时间自动机可用于描述实时。此外，时钟支持另一种总是求值为真的守卫：

        reset(v)

reset 约束用于将时钟的值设为 0。

不变量约束集合 **C'(V)** 是 **C(V)** 的子集，只包含以下形式的约束：

        g = v < c | true

这简化了实现，因为时钟过期是不变量被违反的必要且充分条件，同时仍允许将更复杂的约束指定为守卫。

需要注意的是，任何混合自动机都是一个带有额外守卫和不变量的有效确定性自动机。这些只能进一步约束哪些转移是有效的，但不可能基于 **V** 的取值，定义从 **X** 中同一状态和 **E** 中同一事件开始却以 **X** 中不同状态结束的转移函数。

### 示例

#### 作为混合自动机的 Wip

作为确定性自动机引入的 ‘wip’（wakeup in preemptive，抢占中的唤醒）示例也可以描述为：

- **X** = { `any_thread_running` }
- **E** = { `sched_waking` }
- **V** = { `preemptive` }
- x\ `0` = `any_thread_running`
- X\ `m` = {`any_thread_running`}
- **f** =
   - **f**\ (`any_thread_running`, `sched_waking`, `preemptive==0`) = `any_thread_running`
- **i** =
   - **i**\ (`any_thread_running`) = `true`

```
     |
     |
     v
   #====================#   sched_waking;preemptive==0
   H                    H ------------------------------+
   H any_thread_running H                               |
   H                    H <-----------------------------+
   #====================#

```
在此示例中，通过将系统的抢占状态用作环境变量，我们可以在不要求抢占事件（正如我们在确定性自动机中所做的那样）的情况下，对 `sched_waking` 断言此约束，这在那些事件在系统上不可用或不可靠时很有用。

由于 **i** 中的所有不变量都为真，我们可以从表示中省略它们。

#### 带守卫的停滞模型（迭代 1）

作为时间自动机的示例，我们可以将 ‘stall’ 定义为：

- **X** = { `dequeued`, `enqueued`, `running`}
- **E** = { `enqueue`, `dequeue`, `switch_in`}
- **V** = { `clk` }
- x\ `0` = `dequeue`
- X\ `m` = {`dequeue`}
- **f** =
   - **f**\ (`enqueued`, `switch_in`, `clk < threshold`) = `running`
   - **f**\ (`running`, `dequeue`) = `dequeued`
   - **f**\ (`dequeued`, `enqueue`, `reset(clk)`) = `enqueued`
- **i** = **省略，因为全为真**

```
       |
       |
       v
     #============================#
     H          dequeued          H <+
     #============================#  |
       |                             |
       | enqueue; reset(clk)         |
       v                             |
     +----------------------------+  |
     |          enqueued          |  | dequeue
     +----------------------------+  |
       |                             |
       | switch_in; clk < threshold  |
       v                             |
     +----------------------------+  |
     |          running           | -+
     +----------------------------+

```
该模型规定，一个任务从入队（变为可运行）到真正运行之间的时间必须低于某个阈值。该模型中的失败意味着任务正在饥饿（starving）。
在这种情况下，在边上使用守卫的一个问题是，模型在 `switch_in` 事件发生之前不会报告失败。这意味着，根据该模型，任务永远不运行也是有效的。

#### 带不变量的停滞模型（迭代 2）

第一次迭代并不完全符合预期，我们可以将模型更改为：

- **X** = { `dequeued`, `enqueued`, `running`}
- **E** = { `enqueue`, `dequeue`, `switch_in`}
- **V** = { `clk` }
- x\ `0` = `dequeue`
- X\ `m` = {`dequeue`}
- **f** =
   - **f**\ (`enqueued`, `switch_in`) = `running`
   - **f**\ (`running`, `dequeue`) = `dequeued`
   - **f**\ (`dequeued`, `enqueue`, `reset(clk)`) = `enqueued`
- **i** =
   - **i**\ (`enqueued`) = `clk < threshold`

```
    |
    |
    v
  #=========================#
  H        dequeued         H <+
  #=========================#  |
    |                          |
    | enqueue; reset(clk)      |
    v                          |
  +-------------------------+  |
  |        enqueued         |  |
  |    clk < threshold      |  | dequeue
  +-------------------------+  |
    |                          |
    | switch_in                |
    v                          |
  +-------------------------+  |
  |         running         | -+
  +-------------------------+

```
在这种情况下，我们将守卫作为不变量移到了 `enqueued` 状态，这意味着我们不仅禁止在 `clk` 超过阈值后发生 `switch_in`，而且如果我们在阈值之后**仍然**处于 `enqueued` 状态，也会标记为无效。该模型在任务饥饿的那一刻就实际上处于无效状态，而不是在饥饿的任务最终运行时。

### C 语言中的混合自动机

C 语言中混合自动机的定义大量基于确定性自动机的定义。具体来说，我们添加环境变量的集合以及约束（转移上的守卫和状态上的不变量），如下所示。

```
  /* 用作索引的 X（状态集合）的枚举表示 */
  enum states {
	dequeued,
	enqueued,
	running,
	state_max,
  };

  #define INVALID_STATE state_max

  /* 用作索引的 E（事件集合）的枚举表示 */
  enum events {
	dequeue,
	enqueue,
	switch_in,
	event_max,
  };

  /* 用作索引的 V（环境变量集合）的枚举表示 */
  enum envs {
	clk,
	env_max,
	env_max_stored = env_max,
  };

  struct automaton {
	char *state_names[state_max];                  // X: 状态集合
	char *event_names[event_max];                  // E: 事件有限集合
	char *env_names[env_max];                      // V: 环境变量有限集合
	unsigned char function[state_max][event_max];  // f: 转移函数
	unsigned char initial_state;                   // x_0: 初始状态
	bool final_states[state_max];                  // X_m: 标记状态集合
  };

  struct automaton aut = {
	.state_names = {
		"dequeued",
		"enqueued",
		"running",
	},
	.event_names = {
		"dequeue",
		"enqueue",
		"switch_in",
	},
	.env_names = {
		"clk",
	},
	.function = {
		{ INVALID_STATE,      enqueued, INVALID_STATE },
		{ INVALID_STATE, INVALID_STATE,       running },
		{      dequeued, INVALID_STATE, INVALID_STATE },
	},
	.initial_state = dequeued,
	.final_states = { 1, 0, 0 },
  };

  static bool verify_constraint(enum states curr_state, enum events event,
                                enum states next_state)
  {
	bool res = true;

	/* 作为 f 的一部分验证守卫 */
	if (curr_state == enqueued && event == switch_in)
		res = get_env(clk) < threshold;
	else if (curr_state == dequeued && event == enqueue)
		reset_env(clk);

	/* 验证 i 中的不变量 */
	if (next_state == curr_state || !res)
		return res;
	if (next_state == enqueued)
		ha_start_timer_jiffy(ha_mon, clk, threshold_jiffies);
	else if (curr_state == enqueued)
		res = !ha_cancel_timer(ha_mon);
	return res;
  }

```
函数 `verify_constraint`（此处以简化形式给出）检查守卫、执行重置并启动定时器，以根据规范验证不变量，这些无法轻易地表示在 automaton 结构体中。由于环境变量的复杂性，用户需要提供获取和重置非常规时钟（例如具有 ns 或 jiffy 粒度的时钟）的环境变量的函数。
由于不变量仅定义为时钟过期（例如 *clk < threshold*），到达进入状态时武装的定时器过期实际上意味着模型中的失败并触发一个反应。离开该状态会停止定时器。

需要注意的是，使用 hrtimer 实现的定时器会引入开销，如果监视器有多个实例（例如所有任务），这可能成为问题。使用定时器轮（`HA_TIMER_TYPE` 设为 `HA_TIMER_WHEEL`）可以降低这种影响，这不会损害模型的准确性，因为在回调延迟的情况下，在禁用定时器之前会检查不变量条件。或者，如果保证监视器**最终**会离开该状态，且等待下一个事件所产生的延迟是可接受的，则可以使用守卫来代替不变量，如 stall 示例所示。

### Graphviz .dot 格式

同样地，混合自动机的 Graphviz 表示也是确定性自动机表示的扩展。具体来说，守卫可以在事件中提供

```
    "state_start" -> "state_dest" [ label = "sched_waking;preemptible==0;reset(clk)" ];

```
```
    "enqueued" [label = "enqueued\nclk < threshold_jiffies"];

```
约束可以指定为有效的 C 比较并允许空格，比较的第一个元素必须是时钟，而第二个是数值或参数化的值。守卫允许使用布尔运算（`&&` 和 `||`）组合比较，重置必须与其他约束分开。

```
  digraph state_automaton {
      {node [shape = circle] "enqueued"};
      {node [shape = plaintext, style=invis, label=""] "__init_dequeued"};
      {node [shape = doublecircle] "dequeued"};
      {node [shape = circle] "running"};
      "__init_dequeued" -> "dequeued";
      "enqueued" [label = "enqueued\nclk < threshold_jiffies"];
      "running" [label = "running"];
      "dequeued" [label = "dequeued"];
      "enqueued" -> "running" [ label = "switch_in" ];
      "running" -> "dequeued" [ label = "dequeue" ];
      "dequeued" -> "enqueued" [ label = "enqueue;reset(clk)" ];
      { rank = min ;
          "__init_dequeued";
          "dequeued";
      }
  }

```
### 参考文献

```
  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking,
  The MIT Press, 2008.

```
```
  Thomas Henzinger: The theory of hybrid automata,
  Proceedings 11th Annual IEEE Symposium on Logic in Computer Science, 1996.

```
