## Net DIM 通用网络动态中断调

:Author: Tal Gilboa <talgi@mellanox.com>


## 前提假设


本文档假定读者具备网络驱动以及通用中断调节方面的基础知识

## 简

动态中断调节（DIM）（在网络领域）是指改变某个通道的中断调节配置，以优化数据包处理。该机制包含一个算法，用于决定是否需要以及如何改变某个通道的调节参数，通常通过对系统运行时采样数据进行分析来实现。Net DIM 就是这样一种机制。在算法的每次迭代中，它会分析给定的数据样本，将其与上一次的样本进行比较，并在需要时决定修改部分中断调节配置字段。数据样本由数据带宽、数据包数量以及事件数量组成。样本之间的时间间隔也会被测量。Net DIM 会比较当前与前一次的数据，并返回一个调整后的中断调节配置对象。在某些情况下，算法可能决定不做任何改变。配置字段包括事件之间允许的最小间隔（微秒）以及每个事件期望的最大数据包数量。Net DIM 算法更看重提升带宽，而非降低中断频率

## Net DIM 算法


Net DIM 算法的每次迭代都遵循以下步骤
#. 计算新的数据样本#. 将其与上一次的样本进行比较#. 做出决策 —建议中断调节配置字段#. 调用一个调度工作函数，由其应用所建议的配置
前两个步骤是直观的，新的和前一次的数据都由注册Net DIM 的驱动提供。前一次的数据即为提供给上一次迭代的新数据。比较步骤会检查新数据与前一次数据之间的差异，并决定最后一步的结果。如果带宽上升，该步骤结果为“更好”；如果带宽下降，则为“更差”。如果带宽没有变化，则以类似方式比较包速率 —上升即“更好”，下降即“更差”。如果包速率也没有变化，则比较中断频率。此处算法会优化以降低中断频率为目标，因此中断频率上升被视为“更差”，下降被视为“更好”。步#2 有一个避免误判的优化：只有当样本之间的差异大于某个百分比时，才将其视为有效。此外，由于 Net DIM 本身不进行任何测量，它假定驱动提供的数据是有效的
步骤 #3 根据步骤 #2 的结果以及算法的内部状态来决定建议的配置。状态反映了算法的“方向”：是向左（减小调节）、向右（增大调节）还是保持不动。另一个优化是：如果多次决定保持不动，则算法迭代之间的间隔会增大，以降低计算开销。此外，在“停靠”到最左或最右的某个决策后，算法可能会决定通过向相反方向跨一步来验证该决策。这样做是为了避免陷入“深度睡眠”场景。一旦做出决策，就会从预定义的配置文件中选中一个中断调节配置
最后一步是通知已注册的驱动，使其应用所建议的配置。这是通过调度一个由 Net DIM API 定义、由已注册驱动提供的工作函数来实现的
正如你所看到的，Net DIM 本身并不主动与系统交互。如果向它提供错误的数据，它将难以做出正确决策；而如果工作函数不应用所建议的配置，它也将毫无用处。不过，这确实给了已注册驱动一些回旋余地，因为它可以在某些条件下提供部分数据或忽略算法的建议

## 将网络设备进DIM 注册


Net DIM API 暴露了主函数 net_dim()。该函数是进Net DIM 算法的入口，每次驱动想要检查是否应该改变中断调节参数时都必须调用它。驱动需要提供两个数据结构：`struct dim <dim>` `struct dim_sample <dim_sample>`。`struct dim <dim>` 描述某个特定对象（RX 队列、TX 队列、其他队列等）的 DIM 状态。其中包括当前选中的配置文件、前一次的数据样本、驱动提供的回调函数等。`struct dim_sample <dim_sample>` 描述一个数据样本，该样本将与保存在 `struct dim <dim>` 中的数据样本进行比较，以决定算法的下一步。样本应包含由驱动测量的字节数、数据包数和中断次数
为了在网络驱动中使用 Net DIM，驱动需要调用主函数 net_dim()。推荐的方法是在每次中断时调net_dim()。由Net DIM 内置了调节机制，并且在某些情况下它可能决定跳过迭代，因此无需net_dim() 的调用再做调节。如上所述，驱动需要向 net_dim() 函数调用提供一`struct dim <dim>` 类型的对象。建议每个使Net DIM 的实体都在其数据结构中持有一`struct dim <dim>`，并将其作为 Net DIM API 的主对象。`struct dim_sample <dim_sample>` 应保存最新的字节数、数据包数和中断次数计数。无需进行任何计算，只需包含原始数据即可
net_dim() 调用本身不返回任何值。相反，Net DIM 依赖驱动提供一个回调函数，当算法决定改变中断调节参数时会调用该函数。该回调会被调度并在一个单独的线程中运行，以免给数据流增加开销。工作完成后，Net DIM 算法需要被置为合适的状态，以便进入下一次迭代

## 示例


以下代码演示了如何将驱动注册Net DIM。实际使用并不完整，但应当能使使用方式的轮廓清晰

  #include <linux/dim.h>

  /** Callback for net DIM to schedule on a decision to change moderation **/
  void my_driver_do_dim_work(struct work_struct *work)
  {
	/** Get struct dim from struct work_struct **/
	struct dim *dim = container_of(work, struct dim,
				       work);
	/** Do interrupt moderation related stuff **/
	...

	/** Signal net DIM work is done and it should move to next iteration **/
	dim->state = DIM_START_MEASURE;
  }

  /** My driver's interrupt handler **/
  int my_driver_handle_interrupt(struct my_driver_entity *my_entity, ...)
  {
	...
	/** A struct to hold current measured data **/
	struct dim_sample dim_sample;
	...
	/** Initiate data sample struct with current data **/
	dim_update_sample(my_entity->events,
		          my_entity->packets,
		          my_entity->bytes,
		          &dim_sample);
	/** Call net DIM **/
	net_dim(&my_entity->dim, &dim_sample);
	...
  }

  /** My entity's initialization function (my_entity was already allocated) **/
  int my_driver_init_my_entity(struct my_driver_entity *my_entity, ...)
  {
	...
	/** Initiate struct work_struct with my driver's callback function **/
	INIT_WORK(&my_entity->dim.work, my_driver_do_dim_work);
	...
  }


## 调优 DIM


Net DIM 服务于各类网络设备，并带来出色的加速收益。然而，已经观察DIM 的某些预设配置可能无法与网络设备各异的规格无缝契合，而这种不匹配已被确定为导致启DIM 的网络设备出现次优性能表现的一个因素，其根源在于配置文件不匹配
为解决此问题，Net DIM 引入了每设备控制，用于修改和访问设备`rx-profile` `tx-profile` 参数假设目标网络设备名为 ethx，且 ethx 仅声明支RX 配置文件设置，并支持修改 `usec` 字段`pkts` 字段（参见数据结构：`struct dim_cq_moder <dim_cq_moder>`）
你可以使ethtool 来修改当前的 RX DIM 配置文件，其中全

```
    $ ethtool -C ethx rx-profile 1,1,n_2,2,n_3,n,n_n,4,n_n,n,n
```

`n` 表示不修改该字段，`_` 用于分隔配置文件数组中的结构体元素

```
    $ ethtool -c ethx
    ...
    rx-profile:
    {.usec =   1, .pkts =   1, .comps = n/a,},
    {.usec =   2, .pkts =   2, .comps = n/a,},
    {.usec =   3, .pkts =  64, .comps = n/a,},
    {.usec =  64, .pkts =   4, .comps = n/a,},
    {.usec =  64, .pkts =  64, .comps = n/a,}
    tx-profile:   n/a
```

如果网络设备不支DIM 配置文件的某些特定字段，则会显示相应`n/a`。如果正在修`n/a` 字段，会报告错误消息

## 动态中断调节（DIM）库 API


    :internal:
