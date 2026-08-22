## power allocator 璋冭妭鍣ㄥ彲璋冨弬鏁。

### 触发点（Trip points

该调节器在具有以下两个被动触发点时工作最佳：

1. “switch on”（开启）触发点：温度高于此值时，调节器的控制循环开始运行   这是热区（thermal zone）的第一个被动触发点
2. “desired temperature”（期望温度）触发点：它应当高于“switch on”触发点   这是调节器所控制的目标温度。这是热区的最后一个被动触发点

### PID 鎺у埗鍣。

power allocator 调节器实现了一个比积分-导数控制器（PID 控制器），以温度控制输入、以功率为受控输出：

    P_max = k_p ** e + k_i ** err_integral + k_d * diff_err + sustainable_power

其中
   - e = desired_temperature - current_temperature（期望温- 当前温度   - err_integral 是之前所有误差的累加   - diff_err = e - previous_error（当前误- 上一次误差）

```
				      k_d
				       |
  current_temp                         |
       |                               v
       |              +----------+   +---+
       |       +----->| diff_err |-->| X |------+
       |       |      +----------+   +---+      |
       |       |                                |      tdp        actor
       |       |                      k_i       |       |  get_requested_power()
       |       |                       |        |       |        |     |
       |       |                       |        |       |        |     | ...
       v       |                       v        v       v        v     v
     +---+     |      +-------+      +---+    +---+   +---+   +----------+
     | S |-----+----->| sum e |----->| X |--->| S |-->| S |-->|power     |
     +---+     |      +-------+      +---+    +---+   +---+   |allocation|
       ^       |                                ^             +----------+
       |       |                                |                |     |
       |       |        +---+                   |                |     |
       |       +------->| X |-------------------+                v     v
       |                +---+                               granted performance
  desired_temperature     ^
			  |
			  |
		      k_po/k_pu
```

### 可持续功率（Sustainable power

在注册热区时，应当提供一个可持续耗散功率（单mW）的估计值。它估计了在期望控制温度下可以耗散的持续功率。这是在期望的最高温度下可分配的最大持续功率。实持续功率可能会因为多种原因而变化。闭环控制器会处理诸如环境条件，以及与硅片速度
等级（speed-grade）相关的一些因素所带来的变化。因`sustainable_power` 仅仅是一
个估计值，并且可以被调优以影响热爬升（thermal ramp）的激进程度。作为参考，一4 英寸手机的可持续功率通常2000mW，而一10 英寸平板约为 4500mW（可能随屏幕
尺寸而变）。也可以用一个抽象的标度来表达功率值。持续功率应当与相关冷却设备所使用标度对齐
如果你使用的是设备树，请把它作为
```
	thermal-zones {
		soc_thermal {
			polling-delay = <1000>;
			polling-delay-passive = <100>;
			sustainable-power = <2500>;
			...
```
的属性来添加
相反，如果热区是从平台代码注册的，则传入一个带`sustainable_power` `thermal_zone_params`。如果原本没有传`thermal_zone_params`，那么类似下面这```
	static const struct thermal_zone_params tz_params = {
		.sustainable_power = 3500,
	};
```
然后，把 `tz_params` 作为5 个参数传`thermal_zone_device_register()`

### k_po 涓?k_pu


power allocator 热调节器PID 控制器的实现允许配置两个比例项常数：`k_po` `k_pu`。`k_po` 是温度超调期间（当前温度高于“desired temperature”触发点）的
比例项常数。反之，`k_pu` 是温度欠调期间（当前温度低于“desired temperature”触发点的比例项常数
这些控制项旨在作为配置系统允许的thermal “ramp”（热爬升）的主要机制。例如，较低
`k_pu` 值会提供更慢的爬升，代价是在低温下限制可用容量。另一方面，较高的
`k_pu` 值会导致调节器在温度较低时授予非常高的功率，并可能导致温度超调
```
    2 * sustainable_power / (desired_temperature - switch_on_temp)
```
这意味着`switch_on_temp` 处，控制器比例项的输出将2 * `sustainable_power`默认```
    sustainable_power / (desired_temperature - switch_on_temp)
```
关注 PID 的比例项和前馈```
    P_max = k_p * e + sustainable_power
```
比例项与期望温度和当前温度之差成正比。当当前温度就是期望温度时，比例分量为零`P_max` = `sustainable_power`。也就是说，在恒定负载下，系统应当运行在热平衡状态`sustainable_power` 仅仅是一个估计值，这正是需要此类闭环控制的原因
```
    P_max = 2 * sustainable_power * (T_set - T) / (T_set - T_on) +
	sustainable_power
```
其中
    - T_set 是期望温    - T 是当前温    - T_on 是开启温度（switch on temperature
当当前温度就switch_on 温度时，上式
```
    P_max = 2 * sustainable_power * (T_set - T_on) / (T_set - T_on) +
	sustainable_power = 2 * sustainable_power + sustainable_power =
	3 * sustainable_power
```
因此，仅比例项就会随着温度从开启温度升高到期望温度，将功率3 * `sustainable_power`
线性降低到 `sustainable_power`

### k_i 涓?integral_cutoff


`k_i` 配置 PID 循环的积分项常数。这一项使 PID 控制器能够补偿长期漂移，以及输出量化特性：冷却设备无法设置调节器所请求的精确功率。当温度误差低于 `integral_cutoff`
时，误差被累加进积分项。这一项随后乘`k_i`，其结果被加到控制器的输出中。通常
`k_i` 设得较低 2），`integral_cutoff` 0

### k_d


`k_d` 配置 PID 循环的导数项常数。建议保持默认值：0

## 冷却设备功率 API


由该调节器控制的冷却设备必须在其 `cooling_device_ops` 中提供额外的“powerAPI它由三个操作组成
```
    int get_requested_power(struct thermal_cooling_device *cdev,
			    struct thermal_zone_device *tz, u32 *power);
```
@cdev:
	`struct thermal_cooling_device` 指针
@tz:
	当前所处的热区
@power:
	用于存放计算所得功率的指针

`get_requested_power()` 计算设备所请求的功率（单位毫瓦）并存入 @power。成功时返回
0，失败时返回 -E*。目power allocator 调节器用它通过计算要给每个冷却设备分配多少
功率
```
	int state2power(struct thermal_cooling_device *cdev, struct
			thermal_zone_device *tz, unsigned long state,
			u32 *power);
```
@cdev:
	`struct thermal_cooling_device` 指针
@tz:
	当前所处的热区
@state:
	一个冷却设备状@power:
	用于存放等效功率的指
把冷却设备状@state 转换为功耗（毫瓦）并存入 @power。成功时返回 0，失败时返回
-E*。目thermal core 用它通过计算一个执行体（actor）能够消耗的最大功率
```
	int power2state(struct thermal_cooling_device *cdev, u32 power,
			unsigned long *state);
```
@cdev:
	`struct thermal_cooling_device` 指针
@power:
	功率（毫瓦）
@state:
	用于存放所得状态的指针

计算一个冷却设备状态，使该设备最多消@power 毫瓦，并存入 @state。成功时返回 0失败时返-E*。目thermal core 用它通过power allocator 调节器设定的某个功率
转换为冷却设备能够设置的状态。它是一个函数，因为这种转换可能依赖于可能发生变化的
外部因素，因此该函数应当在“当前情况”下给出最佳转换

### 冷却设备权重


权重是一种在冷却设备之间偏置分配的机制。它们表达了不同冷却设备的相对功率效率可以用较高的权重来表达较高的功率效率。权重是相对的，如果每个冷却设备的权重都1，则认为它们相等。这在异构系统中尤其有用，例如两个冷却设备可能执行同类计算，效率不同。例如一个拥有两种不同类型处理器的系统
如果热区是通过 `thermal_zone_device_register()`（即平台代码）注册的，那么权作为热区`thermal_bind_parameters` 的一部分传入。如果平台是通过设备树注册的那么它们作为 `cooling-maps` 节点中每map `contribution` 属性传入

## power allocator 调节器的局限

power allocator 调节器的 PID 控制器在存在周期tick 时工作最佳。如果你有一驱动反复调用 `thermal_zone_device_update()`（或任何最终会调用调节`throttle()`
函数的东西），调节器的响应就不会很好。注意，这并非该调节器特有——step-wise 调节也是如此，如果你比正常的 thermal 框架 tick 更频繁地调用它的 throttle()（例如由中断），它也会行为异常，因为它会反应过度

## Energy Model 要求


另一件重要的事情是冷却设备所提供的功率值标度要一致。单个热区中的所有冷却设备，功率值应当要么以毫瓦报告，要么缩放到相同的“抽象标度”