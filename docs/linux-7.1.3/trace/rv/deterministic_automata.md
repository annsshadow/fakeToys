## Deterministic Automata


形式上，一个确定性自动机（记作 G）被定义为一个五元组：

        **G** = { **X**, **E**, **f**, x\ `0`, X\ `m` }

其中：

- **X** 是状态的集合；
- **E** 是事件的有限集合；
- x\ `0` 是初始状态；
- X\ `m`（**X** 的子集）是标记（或最终）状态的集合。
- **f** : **X** x **E** -> **X** 是转移函数。它定义了在状态 **X** 中发生来自 **E** 的事件时的状态转移。在确定性自动机的特殊情况下，在 **X** 中的某个状态下发生 **E** 中的事件，会确定性地得到 **X** 中的下一个状态。

例如，一个称为 'wip'（wakeup in preemptive，抢占式唤醒）的给定自动机可以定义为：

- **X** = { `preemptive`, `non_preemptive`}
- **E** = { `preempt_enable`, `preempt_disable`, `sched_waking`}
- x\ `0` = `preemptive`
- X\ `m` = {`preemptive`}
- **f** =
   - **f**\ (`preemptive`, `preempt_disable`) = `non_preemptive`
   - **f**\ (`non_preemptive`, `sched_waking`) = `non_preemptive`
   - **f**\ (`non_preemptive`, `preempt_enable`) = `preemptive`

这种形式化定义的好处之一是它可以用多种格式呈现。例如，使用对**操作系统**从业者非常直观的、由顶点（节点）和边组成的**图形表示**，且没有任何信息损失。

```

                       preempt_enable
          +---------------------------------+
          v                                 |
        #============#  preempt_disable   +------------------+
    --> H preemptive H -----------------> |  non_preemptive  |
        #============#                    +------------------+
                                            ^              |
                                            | sched_waking |
                                            +--------------+

```
### Deterministic Automaton in C


在论文 "Efficient formal verification for the Linux kernel" 中，作者提出了一种在 C 中表示自动机的简单方法，该方法可作为 Linux 内核中的常规代码使用。

```

  /* enum representation of X (set of states) to be used as index */
  enum states {
	preemptive = 0,
	non_preemptive,
	state_max
  };

  #define INVALID_STATE state_max

  /* enum representation of E (set of events) to be used as index */
  enum events {
	preempt_disable = 0,
	preempt_enable,
	sched_waking,
	event_max
  };

  struct automaton {
	char *state_names[state_max];                   // X: the set of states
	char *event_names[event_max];                   // E: the finite set of events
	unsigned char function[state_max][event_max];   // f: transition function
	unsigned char initial_state;                    // x_0: the initial state
	bool final_states[state_max];                   // X_m: the set of marked states
  };

  struct automaton aut = {
	.state_names = {
		"preemptive",
		"non_preemptive"
	},
	.event_names = {
		"preempt_disable",
		"preempt_enable",
		"sched_waking"
	},
	.function = {
		{ non_preemptive,  INVALID_STATE,  INVALID_STATE },
		{  INVALID_STATE,     preemptive, non_preemptive },
	},
	.initial_state = preemptive,
	.final_states = { 1, 0 },
  };

```
**转移函数**表示为状态（行）和事件（列）的矩阵，因此函数 **f** : **X** x **E** -> **X** 可以通过以下方式求解
```

  next_state = automaton_wip.function[curr_state][event];

```
### Graphviz .dot format


Graphviz 开源工具可以使用（文本形式的）DOT 语言作为源来生成自动机的图形表示。DOT 格式被广泛使用，并且可以转换为许多其他格式。

```

  digraph state_automaton {
        {node [shape = circle] "non_preemptive"};
        {node [shape = plaintext, style=invis, label=""] "__init_preemptive"};
        {node [shape = doublecircle] "preemptive"};
        {node [shape = circle] "preemptive"};
        "__init_preemptive" -> "preemptive";
        "non_preemptive" [label = "non_preemptive"];
        "non_preemptive" -> "non_preemptive" [ label = "sched_waking" ];
        "non_preemptive" -> "preemptive" [ label = "preempt_enable" ];
        "preemptive" [label = "preemptive"];
        "preemptive" -> "non_preemptive" [ label = "preempt_disable" ];
        { rank = min ;
                "__init_preemptive";
                "preemptive";
        }
  }

```
这种 DOT 格式可以使用 dot 工具转换为位图或矢量图像，或使用 graph-easy 转换为 ASCII art。对于
```

  $ dot -Tsvg -o wip.svg wip.dot
  $ graph-easy wip.dot > wip.txt

```
### dot2c


dot2c 是一个工具，可以解析包含如上例所示自动机的 .dot 文件，并自动将其转换为 [^3^] 中介绍的 C 表示。

例如，将前面的 'wip' 模型放入名为 'wip.dot' 的文件中，以下命令将把 .dot 文件转换为 C
```

  $ dot2c wip.dot > wip.h

```
'wip.h' 的内容就是 'Deterministic Automaton in C' 一节中的代码示例。

### Remarks


自动机形式化允许以多种格式对离散事件系统（DES）建模，以适应不同的应用/用户。

例如，使用集合论的形式化描述更适合自动机运算，而图形格式更适合人工解读；计算机语言则适合机器执行。

### References


```

  O'Regan, Gerard. Concise guide to software engineering. Springer,
  Cham, 2017.

```
详细描述（包括运算以及在离散事件系统上的应用）可参见
```

  Cassandras, Christos G., and Stephane Lafortune, eds. Introduction to discrete
  event systems. Boston, MA: Springer US, 2008.

```

```

  De Oliveira, Daniel Bristot; Cucinotta, Tommaso; De Oliveira, Romulo
  Silva. Efficient formal verification for the Linux kernel. In:
  International Conference on Software Engineering and Formal Methods.
  Springer, Cham, 2019. p. 315-332.

```
