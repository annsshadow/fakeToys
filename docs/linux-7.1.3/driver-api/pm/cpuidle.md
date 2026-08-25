
## CPU 空闲时间管理


:Copyright: |copy| 2019 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


## CPU 绌洪棽鏃堕棿绠＄悊瀛愮郴缁。

系统中每当一个逻辑 CPU（即那些看起来会取指并执行指令的实体：若支持则为硬件线程否则为处理器核心）在中断或等效唤醒事件之后处于空闲状态，即除与之关联的特定“空闲任务外没有任务可在其上运行时，就存在为它所属的处理器节省能量的机会。这可以通过
让空闲的逻辑 CPU 停止从内存取指，并将其所依赖的部分处理器功能单元置于一种功更低的空闲状态来实现
然而，原则上在这种情况下可能有多个不同的空闲状态可用，因此可能需要找到最合适的
一个（从内核角度看），并请求处理器使用（或“进入”）该特定空闲状态。这正是内核称为 `CPUIdle` CPU 空闲时间管理子系统的职责
`CPUIdle` 的设计是模块化的，并基于避免代码重复的原则，因此原则上不必依赖其硬件或平台设计细节的通用代码，与和硬件交互的代码是分离的。它通常分为三类功能单元负责选择要请求处理器进入的空闲状态的**调节器（governor*、将调节器的决策传递给
硬件*驱动（driver*，以及为它们提供通用框架*核心（core*

## CPU 绌洪棽鏃堕棿璋冭妭鍣。
一CPU 空闲时间（`CPUIdle`）调节器是一束策略代码，在系统中某个逻辑 CPU 变为
空闲时被调用。它的作用是选择一个空闲状态，请求处理器进入以节省一些能量
`CPUIdle` 调节器是通用的，其中任何一个都可用Linux 内核可运行的任何硬件平台因此，它们所操作的数据结构也不能依赖任何硬件架构或平台设计细节
调节器本身由一struct cpuidle_governor 对象表示，其中包含四个回调指:c`enable`c`disable`c`select`c`reflect`，一个如下所述的
:c`rating` 字段，以及一个用于标识它的名称（字符串）
要使调节器可用，需要将该对象通过调用 `cpuidle_register_governor()` 并传入指它的指针作为参数，注册到 `CPUIdle` 核心。若成功，核心会将该调节器加入全局可用
调节器列表；并且，如果它是列表中的唯一一个（即此前列表为空），或者其 :c`rating`
字段的值大于当前所用调节器的该字段值，或者新调节器的名称作为 `cpuidle.governor=`
命令行参数的值传递给了内核，那么从该时刻起就将使用新调节器（同一时刻只能有一`CPUIdle` 调节器在使用）。此外，用户空间也可以通过 `sysfs` 在运行时选择要使用的
`CPUIdle` 调节器
一旦注册，`CPUIdle` 调节器便无法注销，因此将它们放入可加载内核模块并不现实
`CPUIdle` 调节器与核心之间的接口由四个回调组成
:c`enable`
```

	  int (*enable) (struct cpuidle_driver *drv, struct cpuidle_device *dev);

	此回调的作用是为处理``dev`` 参数所指向struct cpuidle_device 对象所表示
	的（逻辑）CPU 做好准备。由 ``drv`` 参数所指向struct cpuidle_driver 对象表示
	要与CPU 一起使用的 ``CPUIdle`` 驱动（除其他外，它应包含代表可请求处理器进入	空闲状态的 struct cpuidle_state 对象列表）
	它可能失败，此时应返回负的错误码，这会导致内核在CPU 上运行架构特定的默认
	空闲 CPU 代码，而不``CPUIdle``，直到针对该 CPU 再次调用 ``->enable()`` 调节	回调
```

:c`disable`
```

	  void (*disable) (struct cpuidle_driver *drv, struct cpuidle_device *dev);

	调用以使调节器停止处理由 ``dev`` 参数所指向struct cpuidle_device 对象所表示	（逻辑）CPU
	它应当撤销上次为目CPU 调用 ``->enable()`` 回调时所做的任何更改、释放该回调分配	所有内存等
```

:c`select`
```

	  int (*select) (struct cpuidle_driver *drv, struct cpuidle_device *dev,
	                 bool *stop_tick);

	调用以为持有``dev`` 参数所指向struct cpuidle_device 对象所表示的（逻辑）CPU 	处理器选择一个空闲状态
	需要考虑的空闲状态列表由 ``drv`` 参数所指向struct cpuidle_driver 对象所持有	struct cpuidle_state 对象:c:member:`states` 数组表示（该对象代表要与CPU 一	使用``CPUIdle`` 驱动）。此回调返回的值被解释为指向该数组的索引（除非它是负的错误码）
	``stop_tick`` 参数用于指示在请求处理器进入所选空闲状态之前，是否停止调度器节	（tick）。当它指向的 ``bool`` 变量（在调用此回调之前被设为 ``true``）被清为 ``false``
	时，处理器将被请求进入所选空闲状态而不停止CPU 上的调度器节拍（但是，如果该 CPU 	的节拍已经停止，则在请求处理器进入空闲状态之前不会重启它）
	此回调是强制性的（即 struct cpuidle_governor 中的 :c:member:`select` 回调指针不能	``NULL``，否则调节器注册将失败）
```

:c`reflect`
```

	  void (*reflect) (struct cpuidle_device *dev, int index);

	调用以允许调节器评估上次调用 ``->select()`` 回调时所做空闲状态选择的准确性，并可	利用该结果在未来提高空闲状态选择的准确性
```

此外，`CPUIdle` 调节器在选择空闲状态时必须考虑处理器唤醒延迟方面的电源管理服务质量
（PM QoS）约束。为了获取给CPU 当前的生PM QoS 唤醒延迟约束，`CPUIdle` 调节器应CPU 的编号传递给 `cpuidle_governor_latency_req()`。随后，调节器的 `->select()`
回调不得返回一个其 :c`exit_latency` 值大于该函数所返回数字的空闲状态的索引

## CPU 空闲时间管理驱动

CPU 空闲时间管理（`CPUIdle`）驱动在 `CPUIdle` 的其他部分与硬件之间提供接口
首先，`CPUIdle` 驱动必须填充代表它的 struct cpuidle_driver 对象中所包含struct cpuidle_state 对象:c`states` 数组。此后，该数组将代表由给定驱动所处理所有逻辑 CPU 共享、可请求处理器进入的可用空闲状态列表
:c`states` 数组中的各项预期struct cpuidle_state :c`target_residency` 字段的以升序排序（即索0 应对应于具有最:c`target_residency` 值的空闲状态）。[由于
:c`target_residency` 值预期反映持有它struct cpuidle_state 对象所代表的空闲状态的
“深度”，因此此排序顺序应与按空闲状态“深度”的升序排序相同。]

现有 `CPUIdle` 调节器使struct cpuidle_state 中的三个字段进行与空闲状态选择相关计算
:c`target_residency`
	在此空闲状态中停留的最短时间（包括进入它所需的时间，这部分可能相当可观），以	节省的能量多于在相同时间内留在较浅空闲状态所能节省的能量，单位为微秒
:c`exit_latency`
	请求处理器进入此空闲状态的 CPU，在从中唤醒后开始执行第一条指令所需的最长时间，
	单位为微秒
:c`flags`
	代表空闲状态属性的标志。目前调节器只使`CPUIDLE_FLAG_POLLING` 标志，当给定对象
	不代表真实的空闲状态、而只是一个可用于避免请求处理器进入任何空闲状态的软件“循环	接口时设置该标志。[`CPUIdle` 核心在特殊情况下还会使用其他标志。]

struct cpuidle_state 中不能为 `NULL` :c`enter` 回调指针，指向为请求处理器进入此
特定空闲状态而要执行的例程：

```

  void (*enter) (struct cpuidle_device *dev, struct cpuidle_driver *drv,
                 int index);

```

它的前两个参数分别指向代表运行此回调的逻辑 CPU struct cpuidle_device 对象，以代表驱动本身struct cpuidle_driver 对象，最后一个参数是驱动 :c`states` 数组中代要请求处理器进入的空闲状态的 struct cpuidle_state 项的索引
struct cpuidle_state 中类似的 `->enter_s2idle()` 回调仅用于实现挂起到空闲
（suspend-to-idle）的系统性电源管理特性。它`->enter()` 的区别在于：它不得在任何
时候（即便是临时）重新启用中断，或试图改变时钟事件设备的状态，`->enter()` 回调
有时可以这样做
一:c`states` 数组被填充，其中有效项的数量必须存入代表驱动struct cpuidle_driver
对象:c`state_count` 字段。此外，如果 :c`states` 数组中的任何项代表“耦合（coupled）空闲状态（即只有多个相关逻辑 CPU 都空闲时才能请求的空闲状态），struct cpuidle_driver
中的 :c`safe_state_index` 字段需要是一个非“耦合”空闲状态的索引（即仅当一个逻辑 CPU
空闲时也能请求的那个）
除此之外，如果给定的 `CPUIdle` 驱动只打算处理系统中逻辑 CPU 的一个子集，struct cpuidle_driver 对象中的 :c`cpumask` 字段必须指向将由它处理的 CPU 集合（掩码）
`CPUIdle` 驱动只有在注册之后才能使用。如果驱动的 :c`states` 数组中没有“耦合”空闲状项，可以通过将其 struct cpuidle_driver 对象传递给 `cpuidle_register_driver()` 来完注册；否则应使用 `cpuidle_register()` 来完成此目的
然而，还需要借助 `cpuidle_register_device()` 为给`CPUIdle` 驱动所处理的所有逻辑 CPU
注册 struct cpuidle_device 对象，该操作在驱动注册之后进行，`cpuidle_register_driver()`
`cpuidle_register()` 不同，不会自动完成此工作。因此，使用 `cpuidle_register_driver()`
注册自身的驱动还必须负责按需注册 struct cpuidle_device 对象，所以通常建议在所有情况下使用 `cpuidle_register()` 来注`CPUIdle` 驱动
注册 struct cpuidle_device 对象会导致创`CPUIdle` `sysfs` 接口，并为它所代表的逻辑
CPU 调用调节器的 `->enable()` 回调，因此该操作必须发生在注册将要处理该 CPU 的驱动之后
`CPUIdle` 驱动struct cpuidle_device 对象不再需要时，可以注销它们，从而释放与之关联的
部分资源。由于它们之间相互依赖，在调`cpuidle_unregister_driver()` 注销驱动之前，必须先
借助 `cpuidle_unregister_device()` 注销由给`CPUIdle` 驱动所代表的所CPU struct cpuidle_device 对象。或者，可以调用 `cpuidle_unregister()` 来注销一`CPUIdle`
驱动以及代表其所有受处理 CPU struct cpuidle_device 对象
`CPUIdle` 驱动可以响应导致可用处理器空闲状态列表更改的运行时系统配置变化（例如，当系统电源从交流（AC）切换到电池，或反之）。在收到此类变化的通知后，`CPUIdle` 驱动应调`cpuidle_pause_and_lock()` 暂时关闭 `CPUIdle`，然后针对所有受该变化影响的 struct cpuidle_device
对象调用 `cpuidle_disable_device()`。接下来，它可以根据系统的新配置更新:c`states` 数组针对所有相关的 struct cpuidle_device 对象调用 `cpuidle_enable_device()`，并调用
`cpuidle_resume_and_unlock()` 以允许再次使`CPUIdle`