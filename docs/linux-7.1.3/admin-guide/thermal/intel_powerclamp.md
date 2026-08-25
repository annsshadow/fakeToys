## Intel Powerclamp 驱动


By:
  - Arjan van de Ven <arjan@linux.intel.com>
  - Jacob Pan <jacob.jun.pan@linux.intel.com>


	(*) 简     - 目标与目
	(*) 工作原理
     - 空闲注入
     - 校准

	(*) 性能分析
     - 有效性与限制
     - 功耗与性能
     - 可扩展     - 校准
     - 与替代技术的比较

	(*) 使用与接     - 通用散热层（sysfs     - 内核 API（待定）

	(*) 模块参数

## 简

考虑这样一种情况：由于功率预算、热约束或噪声水平的限制，必须在运行时降低系统的功耗，并且不希使用主动冷却。必须执行软件管理的被动功耗降低，以防止那些为灾难性场景设计的硬件动作被触发
目前，P-states、T-states（时钟调制）CPU 下线（offlining）被用于 CPU 节流
Intel CPU 上，C-states 提供有效的功耗降低，但到目前为止它们只是基于工作负载被机会性地使用随着 intel_powerclamp 驱动的开发，引入了在所有在CPU 线程之间同步空闲注入的方法。目标是实现
强制的、可控的 C-state 驻留（residency）
已经在功耗、性能、可扩展性和用户体验方面进行了测分析。在许多情况下，与将 CPU 下线或调CPU
时钟相比，它显示出明显的优势

## 工作原理


### 空闲注入


在现Intel 处理器（Nehalem 或更高）上，封装级（package level）C-state 驻留可在 MSR 中获得，
因此也对内核可用
```

      #define MSR_PKG_C2_RESIDENCY      0x60D
      #define MSR_PKG_C3_RESIDENCY      0x3F8
      #define MSR_PKG_C6_RESIDENCY      0x3F9
      #define MSR_PKG_C7_RESIDENCY      0x3FA

```

如果内核还可以向系统注入空闲时间，那么就可以建立一个管理封装级 C-state 的闭环控制系统intel_powerclamp 驱动正是被构想为这样一个控制系统，其目标设定点（set point）是用户选择的空比率（基于功耗降低），而误差是实际封装C-state 驻留比率与目标空闲比率之间的差值
注入由为每个在线 CPU 产生的高优先级内核线程控制
这些内核线程SCHED_FIFO 类创建，用于执行受控占空比和持续时间的钳制（clamping）动作。每个每 CPU
线程基于 jiffies 的取整来同步其空闲时间和持续时间，从而可以防止累积误差，避免抖动效应。线程还绑定CPU，除CPU 下线，否则它们不能被迁移。在这种情况下，属于下线 CPU 的线程将立即终止
SCHED_FIFO 和相对较高的优先级运行，也使得此方案既适用于可抢占内核，也适用于不可抢占内核空闲时间围绕 jiffies 对齐，确保了 HZ 值的可扩展性。使Perf timechart 可以更好地可视化此效果下图显示了内核线kidle_inject/cpu 的行为。在空闲注入期间，它为给定的“duration”运monitor/mwait 空闲，然后将 CPU 让给其他任务，直到下一个时间间隔
在空闲期间禁用了 NOHZ 调度节拍（schedule tick），但中断未被屏蔽。测试表明，来自调度节拍的额外唤powerclamp 驱动在大型系统（80 个处理器Westmere 系统）上的有效性有显著影响
```

  CPU0
		    ____________          ____________
  kidle_inject/0   |   sleep    |  mwait |  sleep     |
	  _________|            |________|            |_______
				 duration
  CPU1
		    ____________          ____________
  kidle_inject/1   |   sleep    |  mwait |  sleep     |
	  _________|            |________|            |_______
				^
				|
				|
				roundup(jiffies, interval)

```

只有一CPU 被允许收集统计信息并更新全局控制参数。本文档将此 CPU 称为控制 CPU。控CPU 在运行时
选举产生，其策略偏向 BSP，同时考虑CPU 热插拔的可能性
就空闲控制系统的动态特性而言，封装级空闲时间在很大程度上被视为一个非因果（non-causal）系统，行为不能基于过去或当前的输入。因此，intel_powerclamp 驱动试图作为给定输入（目标空闲比率）即时强制
实施所需的空闲时间。注入之后，powerclamp 监视给定时间窗口内的实际空闲，并相应调整下一次注入，避免过度或不足校正
当用于因果控制系统（例如温度控制）时，由该驱动的使用者来实现将过去样本和输出包含在反馈中的算法例如，基PID 的热控制器可以使powerclamp 驱动来保持期望的目标温度，基于过去样本的积分和微分增益
### 校准


在可扩展性测试期间观察到，随着核心数量的增长，CPU 之间的同步动作变得具有挑战性。系统进入封装级
C-states 的能力也是如此
为了确保 intel_powerclamp 驱动具有良好的可扩展性，实现了在线校准。进行此类校准的目标是：

a) 确定空闲注入比率的有效范b) 确定每个目标比率所需的补偿量

对每个目标比率的补偿由两部分组成
	a) 稳态误差补
	   这是为了抵消当系统可以在没有额外唤醒（例如外部中断）的情况下进入空闲时发生的误差
	b) 动态误差补
	   当空闲期间发生过多唤醒时，可以通过减慢 CPU 活动来添加额外的空闲比率以平息中断
提供了一debugfs 文件供用户检查补
```

  [jacob@nex01 ~]$ cat
  /sys/kernel/debug/intel_powerclamp/powerclamp_calib
  controlling cpu: 0
  pct confidence steady dynamic (compensation)
  0       0       0       0
  1       1       0       0
  2       1       1       0
  3       3       1       0
  4       3       1       0
  5       3       1       0
  6       3       1       0
  7       3       1       0
  8       3       1       0
  ...
  30      3       2       0
  31      3       2       0
  32      3       1       0
  33      3       2       0
  34      3       1       0
  35      3       2       0
  36      3       1       0
  37      3       2       0
  38      3       1       0
  39      3       2       0
  40      3       3       0
  41      3       1       0
  42      3       2       0
  43      3       1       0
  44      3       1       0
  45      3       2       0
  46      3       3       0
  47      3       0       0
  48      3       2       0
  49      3       3       0

```

校准在运行时进行。没有可用的离线方法。稳态补偿仅在所有相邻比率的置信水平都达到满意水平时才使用置信水平基于运行时收集的干净数据累积。在没有额外中断的时期收集的数据被视为干净的
为了补偿空闲期间过多的唤醒，当检测到这种情况时会注入额外的空闲时间。目前，我们有一个简单的算法注入比率加倍。一个可能的增强是限制有问题IRQ，例如延迟电平触发中断的 EOI。但要做到对调度器或 IRQ
核心代码无侵入是具有挑战性的

### CPU 上线/下线


CPU 内核线程在收CPU 热插拔活动通知时启停止。intel_powerclamp 驱动跟踪钳制内核线程，即在它们被迁移到其CPU 之后，在 CPU 下线事件之后也是如此

## 性能分析

本节描述了在多个系统（包Westmere0P）和 Ivy BridgePP））上收集的一般性能数据
### 有效性与限制

空闲注入允许的最大范围上限为 50%。如前所述，由于在强制空闲期间允许中断，过多的中断可能导致有效降低。极端情况是flooded 网络中断执行 ping -f CPU 几乎不确认。在这种情况下，空闲注入线程几乎
无能为力。在大多数正常情况下，例scp 一个大文件，应用程序可以被 powerclamp 驱动节流，因为减CPU 也会减慢网络协议处理，从而减少中断
当控CPU 在运行时更改控制参数时，其余 CPU 可能需要额外一个周期才能赶上变化。在此期间，空闲注入
不同步，因此无法以期望的比率进入封装 C-states。但这种影响很小，因为在大多数情况下，对目标比率更改比空闲注入频率更新得少得多
### 可扩展
测试还显示了 4P/8P Ivy Bridge 系统80P Westmere 服务器在 50% 空闲比率下的微小但可测量差异。对于相同的目标空闲比率，Westmere 需要更多的补偿。补偿也随之随空闲比率增大而增大。上述原构成了需要校准代码
IVB 8P 系统上，与下线的 CPU 相比，powerclamp 可以实现高达 40% 的每瓦性能提升。（通过所有运CPU 产生的每 CPU 计数线程求和的自旋计数器测量。）

## 使用与接
powerclamp 驱动作为冷却设备注册到通用散热层，
```

  jacob@chromoly:/sys/class/thermal/cooling_device14$ grep . *
  cur_state:0
  max_state:50
  type:intel_powerclamp

```

cur_state 允许用户设置所需的空闲百分比。向 cur_state 写入 0 将停止空闲注入。写1 max_state
之间的值将启动空闲注入。读cur_state 返回实际和当前的空闲百分比。这可能不同于用户设置的值，因为
当前空闲百分比取决于工作负载并包含自然空闲。当空闲注入被禁用时，读cur_state 返回-1 而不0以避免将 100% 繁忙状态与禁用状态混淆
示例用法
```

	$ sudo sh -c "echo 25 > /sys/class/thermal/cooling_device80/cur_state

```

如果系统不繁忙且已有超过 25% 的空闲时间，那么 powerclamp 驱动将不会启动空闲注入。使Top 将不显示空闲注入内核线程
如果系统繁忙（下面的自旋测试）且自然空闲时间少于 25%，powerclamp 内核线程将进行空闲注入。强制空时间作为正常空闲计入，因为走的是与空闲任务相同的公共代码路径
在此示例中，显示24.1% 的空闲。这有助于系统管理员
```


  Tasks: 197 total,   1 running, 196 sleeping,   0 stopped,   0 zombie
  Cpu(s): 71.2%us,  4.7%sy,  0.0%ni, 24.1%id,  0.0%wa,  0.0%hi,  0.0%si,  0.0%st
  Mem:   3943228k total,  1689632k used,  2253596k free,    74960k buffers
  Swap:  4087804k total,        0k used,  4087804k free,   945336k cached

    PID USER      PR  NI  VIRT  RES  SHR S %CPU %MEM    TIME+  COMMAND
   3352 jacob     20   0  262m  644  428 S  286  0.0   0:17.16 spin
   3341 root     -51   0     0    0    0 D   25  0.0   0:01.62 kidle_inject/0
   3344 root     -51   0     0    0    0 D   25  0.0   0:01.60 kidle_inject/3
   3342 root     -51   0     0    0    0 D   25  0.0   0:01.61 kidle_inject/1
   3343 root     -51   0     0    0    0 D   25  0.0   0:01.60 kidle_inject/2
   2935 jacob     20   0  696m 125m  35m S    5  3.3   0:31.11 firefox
   1546 root      20   0  158m  20m 6640 S    3  0.5   0:26.97 Xorg
   2100 jacob     20   0 1223m  88m  30m S    3  2.3   0:23.68 compiz

```

测试表明，通过使用 powerclamp 驱动作为冷却设备，在没有添加其他热影响时，基PID 的用户空间热控制可以有效地管CPU 温度。例如，UltraBook 用户可以在某个温度（低于大多数主动跳变点）以下编译内核
## 模块参数


`cpumask` (RW)
	要注入空闲的 CPU 的位掩码。位掩码的格式与 /proc/irq/\*/smp_affinity 等其他子系统中使用的
	格式相同。该掩码是以逗号分隔32 位组。每CPU 是一位。例如，对于 256 CPU 的系统，完整
	掩码为：
	ffffffff,ffffffff,ffffffff,ffffffff,ffffffff,ffffffff,ffffffff,ffffffff

	最右边的掩码对CPU 0-32
`max_idle` (RW)
	注入的空闲时间与CPU 时间之比的最大百分比，范围从 1 100。即使冷却设备的 max_state 始终
	10000%），此参数也允许添加一个最大空闲百分比限制。默认值为 50，以匹配 powerclamp 驱动	当前实现。如cpumask 包含系统中存在的每个 CPU，也不允许值超75