## 集群级上下电竞争规避算法


本文档描述了用于协调 CPU 与集群的建立与拆除操作，并安全管理硬件一致控制的算法
“原理”一节解释了该算法的作用及其必要性。“基本模型”使用简化的系统视图
解释通用概念。其余各节则解释该算法实际使用的细节

### 原理


在包含多CPU 的系统中，希望能够在系统空闲时关闭单CPU，以降低功耗与
散热
在包含多CPU 集群的系统中，同样希望有能力关闭整个集群
关闭和开启整个集群是一件有风险的事，因为它涉及执行可能影响一组独立运CPU 的、具有潜在破坏性的操作，而此时操作系统仍在运行。这意味着我们需某种协调机制，以确保关键的集群级操作仅在真正安全时才执行
简单的加锁可能不足以解决此问题，因为像 Linux 自旋锁之类的机制可能依赖一致性机制，而集群上电时这些机制并不是立即可用的。由于启用或禁用这些机制
本身可能是非原子操作（例如写入某些硬件寄存器并使大容量缓存失效），因此需其他协调方法，以保证集群级的安全下电与上电
本文档提出的机制描述了一种用于执行所需协调的、基于一致性内存的协议。它
力求尽可能轻量，同时提供所需的安全属性

### 基本模型


每个集群CPU 都被赋予一个状态，如下
 - DOWN
 - COMING_UP
 - UP
 - GOING_DOWN

```

	    +---------> UP ----------+
	    |                        v

	COMING_UP                GOING_DOWN

	    ^                        |
	    +--------- DOWN <--------+


```
DOWN:
	CPU 或集群不具备一致性，且处于断电或挂起状态，或者已准备好被断电
	或挂起
COMING_UP:
	CPU 或集群已确定要进UP 状态。它可能正处于初始化并启用一致性的
	过程之中
UP:
	CPU 或集群在硬件层面处于活动且一致的状态。处于此状态的 CPU 不一	正被内核主动使用
GOING_DOWN:
	CPU 或集群已确定要进DOWN 状态。它可能正处于拆除与退出一致性的
	过程之中

每个 CPU 在任何时刻都被赋予上述状态之一。CPU 状态在下面的“CPU 状态”一节中
描述
每个集群也被赋予一个状态，但有必要将该状态值拆分为两部分（“cluster”状态与
“inbound”状态），并引入额外的状态，以避免集群中不同 CPU 同时修改状态而产竞争。集群级状态在“集群状态”一节中描述
为了在本讨论中区CPU 状态与集群状态，状态名CPU 状态加`CPU_` 前缀对集群状态加`CLUSTER_` `INBOUND_` 前缀

### CPU 状

在本算法中，多核处理器中的每个独立核心被称为一个“CPU”。CPU 被假定为
单线程：因此，一CPU 在任一时刻只能做一件事
这意味着 CPU 与基本模型高度契合
该算法为系统中的每个 CPU 定义了以下状态：

 - CPU_DOWN
 - CPU_COMING_UP
 - CPU_UP
 - CPU_GOING_DOWN

```

	 cluster setup and
	CPU setup complete          policy decision
	      +-----------> CPU_UP ------------+
	      |                                v

	CPU_COMING_UP                   CPU_GOING_DOWN

	      ^                                |
	      +----------- CPU_DOWN <----------+
	 policy decision           CPU teardown complete
	or hardware event


```
四种状态的定义与基本模型的状态紧密对应
状态之间的转换如下所述
触发事件（自发）意味着 CPU 仅通过本地推进即可转换到下一状态，而无需任何
外部事件发生

CPU_DOWN:
	CPU 准备好下电时，它达到 CPU_DOWN 状态。到达此状态后，CPU 通常	通过 WFI 指令或固件调用自行断电或挂起
	下一状态：
		CPU_COMING_UP
	条件		
	触发事件		a) 由另一 CPU 上的策略决策导致的显式硬件上电操作；

		b) 硬件事件，例如中断

CPU_COMING_UP:
	在集群建立并就绪一致之前，CPU 无法开始参与硬件一致性。如果集群尚	就绪，则 CPU 将停留在 CPU_COMING_UP 状态，直到集群已建立
	下一状态：
		CPU_UP
	条件		CPU 的父集群必须处于 CLUSTER_UP	触发事件		父集群转换到 CLUSTER_UP
	关于 CLUSTER_UP 状态的描述，请参阅“集群状态”一节

CPU_UP:
	CPU 达到 CPU_UP 状态时，CPU 开始参与本地一致性是安全的
	这是通过跳转到内核的 CPU 恢复代码来完成的
	注意，该状态的定义与基本模型定义略有不同：CPU_UP 并不意味着 CPU 已经
	具备一致性，但它确实意味着恢复内核是安全的。内核负责处理剩余的恢复
	流程，因此其余步骤不会作为竞争规避算法的一部分而可见
	CPU 保持在此状态，直到做出显式的策略决策来关闭或挂起该 CPU
	下一状态：
		CPU_GOING_DOWN
	条件			触发事件		显式策略决策


CPU_GOING_DOWN:
	在此状态下，CPU 退出一致性，包括为此所需的任何操作（例如清理数据
	缓存）
	下一状态：
		CPU_DOWN
	条件		本地 CPU 拆除完成
	触发事件		（自发）


### 集群状

集群是一组具有某些公共资源的相连 CPU。由于集群包含多CPU，它可以同时
做多件事。这带来一些影响。特别是，一CPU 可以在另一CPU 正在拆除集群启动起来
在本讨论中，“outbound 侧”是正在拆除集群CPU 所见到的集群状态视图“inbound 侧”是正在建立 CPU CPU 所见到的集群状态视图
为了在此类情况下实现安全协调，正在建立集群的 CPU 能够独立于正在拆除集群的
CPU 通告其状态，这一点非常重要。因此，集群状态被拆分为两部分
	“cluster”状态：集群的全局状态；outbound 侧的状态：

  - CLUSTER_DOWN
  - CLUSTER_UP
  - CLUSTER_GOING_DOWN

	“inbound”状态：集群inbound 侧的状态
  - INBOUND_NOT_COMING_UP
  - INBOUND_COMING_UP


	这些状态的不同组合导致了六种可能的
```

	                            CLUSTER_UP
	          +==========> INBOUND_NOT_COMING_UP -------------+
	          #                                               |
	                                                          |
	     CLUSTER_UP     <----+                                |
	  INBOUND_COMING_UP      |                                v

	          ^             CLUSTER_GOING_DOWN       CLUSTER_GOING_DOWN
	          #              INBOUND_COMING_UP <=== INBOUND_NOT_COMING_UP

	    CLUSTER_DOWN         |                                |
	  INBOUND_COMING_UP <----+                                |
	                                                          |
	          ^                                               |
	          +===========     CLUSTER_DOWN      <------------+
	                       INBOUND_NOT_COMING_UP

	转换 -----> 只能outbound CPU 进行，且只涉及“cluster”状态的变更
	转换 ===##> 只能inbound CPU 进行，且只涉及“inbound”状态的变更	除非outbound 侧已不可能有进一步转换（outbound CPU 已将集群置于
	CLUSTER_DOWN 状态）
	竞争规避算法没有提供方法来判定集群中具体是哪CPU 承担这些角色。这
	必须事先通过其他手段决定。更多说明请参阅“最后一人（last man）与
	第一人（first man）的选择”一节

	CLUSTER_DOWN/INBOUND_NOT_COMING_UP 是唯一集群可以真正下电的状态
	inbound outbound CPU 的并行性通过观察CLUSTER_GOING_DOWN/
	INBOUND_NOT_COMING_UP（对应基本模型中GOING_DOWN）到 CLUSTER_DOWN/
	INBOUND_COMING_UP（对应基本模型中COMING_UP）存在两条不同路径而得	体现。第二条路径完全避免了集群拆除
	CLUSTER_UP/INBOUND_COMING_UP 等价于基本模型中UP。最终到 CLUSTER_UP/
	INBOUND_NOT_COMING_UP 的转换是平凡的，仅仅是重置状态机以为下一轮循	做好准备
	有关允许的转换细节如下
	每种情况下的下一状态记
		<cluster 状/<inbound 状 (<转换)

	其中 <转换 是转换可以发生的一侧；要么inbound 侧，要么	outbound 侧

```
CLUSTER_DOWN/INBOUND_NOT_COMING_UP:
	下一状态：
		CLUSTER_DOWN/INBOUND_COMING_UP (inbound)
	条件		
	触发事件		a) 由另一 CPU 上的策略决策导致的显式硬件上电操作；

		b) 硬件事件，例如中断

CLUSTER_DOWN/INBOUND_COMING_UP:

	在此状态下，inbound CPU 建立集群，包括启用集群级的硬件一致性以	为此所需的任何其他操作（例如缓存失效）
	该状态的目的在于完成足够的集群级建立，以使集群中的其CPU 能够
	安全地进入一致性
	下一状态：
		CLUSTER_UP/INBOUND_COMING_UP (inbound)
	条件		集群级建立与硬件一致性完	触发事件		（自发）


CLUSTER_UP/INBOUND_COMING_UP:

	集群级建立已完成，且集群的硬件一致性已启用。集群中的其CPU 可以
	安全地进入一致性
	这是一个瞬态，会立即导CLUSTER_UP/INBOUND_NOT_COMING_UP。集群上	所有其CPU 应将这两种状态视为等价
	下一状态：
		CLUSTER_UP/INBOUND_NOT_COMING_UP (inbound)
	条件			触发事件		（自发）


CLUSTER_UP/INBOUND_NOT_COMING_UP:

	集群级建立已完成，且集群的硬件一致性已启用。集群中的其CPU 可以
	安全地进入一致性
	集群将保持在此状态，直到做出下电的策略决策
	下一状态：
		CLUSTER_GOING_DOWN/INBOUND_NOT_COMING_UP (outbound)
	条件			触发事件		下电集群的策略决

CLUSTER_GOING_DOWN/INBOUND_NOT_COMING_UP:

	一outbound CPU 正在拆除集群。被选中CPU 必须在此状态等待，直到
	集群中的所CPU 都处CPU_DOWN 状态
	当所CPU 都处CPU_DOWN 状态时，集群即可被拆除，例如通过清理数据
	缓存并退出集群级一致性
	为了避免不必要的浪费性拆除操作，outbound 侧应检inbound 集群状	是否异步转换到了 INBOUND_COMING_UP。或者，也可以检查单CPU 是否进入
	浜?CPU_COMING_UP 鎴?CPU_UP銆。

	下一状态：

	CLUSTER_DOWN/INBOUND_NOT_COMING_UP (outbound)
		条件			集群已拆除并准备好断		触发事件			（自发）

	CLUSTER_GOING_DOWN/INBOUND_COMING_UP (inbound)
		条件			
		触发事件			a) 由另一 CPU 上的策略决策导致的显式硬件上电操作；

			b) 硬件事件，例如中断

CLUSTER_GOING_DOWN/INBOUND_COMING_UP:

	集群正在（或曾经）被拆除，但与此同时另一CPU 已上线，并正试图
	重新建立集群
	如果 outbound CPU 观察到此状态，它有两个选择
		a) 退出拆除，将集群恢复到 CLUSTER_UP 状态；

		b) 完成集群拆除，并将集群置CLUSTER_DOWN 状态；inbound CPU
		   将从该状态重新建立集群
	选择 (a) 可以在集群实际上不会被断电的情况下，通过避免不必要的拆除		建立操作来消除一些延迟

	下一状态：

	CLUSTER_UP/INBOUND_COMING_UP (outbound)
		条件			集群级建立与硬件一致性完
		触发事件			（自发）

	CLUSTER_DOWN/INBOUND_COMING_UP (outbound)
		条件			集群已拆除并准备好断
		触发事件			（自发）


### 最后一人（last man）与第一人（first man）的选择


outbound 侧执行集群拆除操作的 CPU 通常被称为“last man”（最后一人）
inbound 侧执行集群建立的 CPU 通常被称为“first man”（第一人）
上文记录的竞争规避算法没有提供选择哪些 CPU 承担这些角色的机制

最后一人：

关闭集群时，所有相关的 CPU 最初都在执Linux，因此具备一致性。因此，CPU 变为非一致性之前，可以使用普通自旋锁安全地选定最后一人

第一人：

由于 CPU 可能响应外部唤醒事件而异步上电，需要一个动态机制来确保只有一CPU 尝试扮演第一人角色并执行集群级初始化：任何其CPU 都必须等待其完成
才能继续
集群级初始化可能涉及诸如在总线互连（bus fabric）中配置一致性控制等操作
当前 mcpm_head.S 中的实现使用一个独立的互斥机制来进行此仲裁。该机制vlocks.txt 中有详细记录

### 特性与限制


实现
	当前基于 ARM 的实现分布在 arch/arm/common/mcpm_head.S
	（底inbound CPU 操作）与 arch/arm/common/mcpm_entry.c（其余部分）
	__mcpm_cpu_going_down() 表示 CPU CPU_GOING_DOWN 状态的转换
	__mcpm_cpu_down() 表示 CPU CPU_DOWN 状态的转换
	一CPU 通过 mcpm_head.S 中的底层上电代码转换CPU_COMING_UP	继而转换到 CPU_UP。这可能涉及特定CPU 的建立代码，但在当前实现
	中并未涉及
	__mcpm_outbound_enter_critical() 涓?__mcpm_outbound_leave_critical()
	处理CLUSTER_UP CLUSTER_GOING_DOWN、并由此CLUSTER_DOWN 	回到 CLUSTER_UP（在集群下电被中止的情况下）的转换
	由于集群级安全转换所需的额CPU 间协调，这些函数__mcpm_cpu_*()
	系列函数更为复杂
	集群通过 mcpm_head.S 中的底层上电代码CLUSTER_DOWN 转换	CLUSTER_UP。这通常涉及平台特定的建立代码，由通过 mcpm_sync_init
	注册的、平台特定的 power_up_setup 函数提供
深层拓扑
	正如当前所描述与实现的，该算法不支持超过两级的 CPU 拓扑（即不支	集群的集群）。可以通过为额外的拓扑层级复制集群级状态，并修改中	（非最外层）集群层级的转换规则来扩展该算法

### 版本说明


本文最初由 Dave Martin Linaro Limited 创建并记录，Nicolas Pitre Achin Gupta 合作完成
Copyright (C) 2012-2013  Linaro Limited
根据 linux/COPYING 中定义的 GNU 通用公共许可证第 2 版条款分发