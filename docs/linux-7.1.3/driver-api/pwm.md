## 脉冲宽度调制（PWM）接口


本文提供关于 Linux PWM 接口的概述。

PWM 通常用于控制手机中的 LED、风扇或振动器。用途固定的 PWM 无需实现 Linux PWM
API（尽管它们可以）。然而，PWM 经常作为 SoC 上用途不固定的分立器件出现。由板卡
设计者将它们连接到 LED 或风扇。为了提供这种灵活性，通用的 PWM API 应运而生。

### 标识 PWM


传统 PWM API 的用户使用唯一 ID 来引用 PWM 设备。

板卡 setup 代码不应通过其唯一 ID 引用 PWM 设备，而应注册一个可用于匹配 PWM 的
静态映射
```

	static struct pwm_lookup board_pwm_lookup[] = {
		PWM_LOOKUP("tegra-pwm", 0, "pwm-backlight", NULL,
			   50000, PWM_POLARITY_NORMAL),
	};

	static void __init board_init(void)
	{
		...
		pwm_add_table(board_pwm_lookup, ARRAY_SIZE(board_pwm_lookup));
		...
	}

```
### 使用 PWM


消费者使用 pwm_get() 函数，并向其传入消费者设备或消费者名称。pwm_put() 用于
释放 PWM 设备。也存在托管（managed）变体：devm_pwm_get() 和
devm_fwnode_pwm_get()。

```

	int pwm_apply_might_sleep(struct pwm_device *pwm, struct pwm_state *state);

```
该 API 同时控制 PWM 周期/占空比配置以及使能/禁用状态。

如果 PWM 不睡眠，则可以从原子上下文使用 PWM 设备。你
```

        bool pwm_might_sleep(struct pwm_device *pwm);

```
```

	int pwm_apply_atomic(struct pwm_device *pwm, struct pwm_state *state);

```
作为消费者，不要依赖被禁用 PWM 的输出来判断其状态。如果容易做到，驱动应当发出
非激活状态，但有些驱动无法做到。如果你依赖获得非激活状态，请使用
.duty_cycle=0、.enabled=true。

还有一个 usage_power 设置：如果设置，PWM 驱动只需维持功率输出，但在信号形式上
有更大的自由度。如果驱动支持，信号可以被优化，例如通过相位偏移芯片的各个通道
来改善 EMI。

pwm_config()、pwm_enable() 和 pwm_disable() 函数只是 pwm_apply_might_sleep() 的
包装，如果用户想一次性更改多个参数，则不应使用它们。例如，如果你在同一函数中看到
pwm_config() 和 pwm_{enable,disable}() 调用，这可能意味着你应该切换到
pwm_apply_might_sleep()。

PWM 用户 API 还允许使用 pwm_get_state() 查询传递给上一次 pwm_apply_might_sleep()
调用的 PWM 状态。注意，如果请求无法被所用硬件精确满足，这与驱动实际实现的
有所不同。目前消费者无法获取实际实现的设置。

除了 PWM 状态外，PWM API 还暴露 PWM 参数（arguments），它是应当在该 PWM 上使用的
参考 PWM 配置。PWM 参数通常是平台相关的，并允许 PWM 用户只关心相对于整个周期
的占空比（例如，duty = 周期 的 50%）。struct pwm_args 包含两个字段（period 和
polarity），应当用于设置初始 PWM 配置（通常在 PWM 用户的 probe 函数中完成）。PWM
参数通过 pwm_get_args() 获取。

所有消费者都应当在恢复时适当地重新配置 PWM。这是确保一切以正确顺序恢复的唯一方法。

### 通过 sysfs 接口使用 PWM


如果你在内核配置中启用了 CONFIG_SYSFS，会提供一个简单的 sysfs 接口，以从用户空间
使用 PWM。它暴露在 /sys/class/pwm/。每个被探测到的 PWM 控制器/芯片会被导出为
pwmchipN，其中 N 是 PWM 芯片的基地址。在该目录中你会发现：

  npwm
    该芯片支持的 PWM 通道数量（只读）。

  export
    导出一个 PWM 通道供 sysfs 使用（只写）。

  unexport
    从 sysfs 取消导出一个 PWM 通道（只写）。

PWM 通道使用每个芯片从 0 到 npwm-1 的索引编号。

当导出一个 PWM 通道时，会在其关联的 pwmchipN 目录中创建一个 pwmX 目录，其中 X 是
被导出通道的编号。随后以下属性将可用：

  period
    PWM 信号的总周期（读/写）。
    值以纳秒为单位，是 PWM 激活和非激活时间之和。

  duty_cycle
    PWM 信号的激活时间（读/写）。
    值以纳秒为单位，必须小于或等于周期。

  polarity
    更改 PWM 信号的极性（读/写）。
    只有在 PWM 芯片支持更改极性时，对该属性的写入才有效。
    值为字符串 "normal" 或 "inversed"。

  enable
    启用/禁用 PWM 信号（读/写）。

 - 0 - 禁用
 - 1 - 启用

### 实现 PWM 驱动


目前有两种方式来实现 PWM 驱动。传统上只有裸骨（barebone）API，意味着每个驱动
必须自己实现 pwm_*() 函数。这意味着系统中不可能有多个 PWM 驱动。因此，新的驱动
必须使用通用 PWM 框架。

一个新的 PWM 控制器/芯片可以使用 pwmchip_alloc() 分配，然后使用 pwmchip_add()
注册，并使用 pwmchip_remove() 再次移除。要撤销 pwmchip_alloc() 可使用 pwmchip_put()。
pwmchip_add() 接受一个填充好的 struct pwm_chip 作为参数，它向框架提供 PWM 芯片的
描述、芯片提供的 PWM 设备数量，以及所支持的 PWM 操作的芯片特定实现。

在 PWM 驱动中实现极性支持时，请确保遵守 PWM 框架中的信号约定。根据定义，正常
极性表征一个信号在占空比持续期间为高电平，并在周期的剩余时间为低电平。相反，具有
反相极性的信号在占空比持续期间为低电平，并在周期的剩余时间为高电平。

鼓励驱动实现 ->apply() 而非传统的 ->enable()、->disable() 和 ->config() 方法。
这样做应在 PWM 配置工作流中提供原子性，这在 PWM 控制关键设备（如调节器）时是必需的。

实现 ->get_state()（一种用于获取初始 PWM 状态的方法）也出于同样的原因被鼓励：让
PWM 用户了解当前 PWM 状态可以避免故障（glitch）。

驱动不应实现任何电源管理。换句话说，消费者应按照“使用 PWM”一节所述来实现它。

### 锁


PWM 核心的列表操作受互斥体（mutex）保护，因此 pwm_get() 和 pwm_put() 不能从原子
上下文调用。PWM 消费者 API 中的大多数函数可能睡眠，因此不能从原子上下文调用。值得
注意的例外是 pwm_apply_atomic()，它与 pwm_apply_might_sleep() 语义相同，但可以从
原子上下文调用。（代价是它并非对所有 PWM 设备都有效，使用 pwm_might_sleep() 来检查
给定的 PWM 是否支持原子操作。）

PWM 核心中的锁确保与单个芯片相关的回调被串行化。

### 辅助函数


目前一个 PWM 只能以 period_ns 和 duty_ns 配置。对于若干用例，freq_hz 和
duty_percent 可能更好。请不要在你的驱动中自行计算，而是考虑向框架添加适当的
辅助函数。
