
## 内核驱动 apds990x


支持的芯片：
Avago APDS990X

数据手册：
不公开提供

作者：
Samu Onkalo <samu.p.onkalo@nokia.com>

### 描述


APDS990x 是一个组合的环境光和接近传感器。ALS 和接近功能高度相关。在启用接近功能时，ALS 测量路径必须处于运行状态。

ALS 为两个通道产生原始测量值：Clear 通道（红外+可见光）和仅 IR 通道。然而，阈值比较仅使用 Clear 通道。Lux 值和硬件上的阈值电平可能会因光源光谱的不同而有很大差异。

驱动在双向都进行必要的转换，使用户只需处理 lux 值。Lux 值使用两个通道的信息计算。硬件阈值电平根据给定 lux 值计算，以匹配当前的光照类型。有时估计的不准确会导致误中断，但这并无妨碍。

ALS 包含 4 个不同的增益步进。驱动自动选择合适的增益步进。每次测量后，会估计结果的可靠性，并在必要时触发新的测量。

如果已知数值，平台数据可以为转换公式提供调优后的值。否则使用传感器默认值。

接近功能一侧稍微简单一些。不需要复杂的转换。它直接产生可用的值。

驱动使用 pm_runtime 框架控制芯片的运行状态。电压调节器根据芯片的运行状态进行控制。

### SYSFS



chip_id
	RO - 显示检测到的芯片类型和版本

power_state
	RW - 启用/禁用芯片。使用计数逻辑

	     1 启用芯片
	     0 禁用芯片
lux0_input
	RO - 测量的 lux 值

	     sysfs_notify 在阈值中断发生时被调用

lux0_sensor_range
	RO - lux0_input 最大值。

	     实际上永远不会达到，因为传感器往往在此之前就已饱和。真实最大值因光谱等因素而异。

lux0_rate
	RW - 测量速率（Hz）

lux0_rate_avail
	RO - 支持的测量速率

lux0_calibscale
	RW - 校准值。

	     默认设置为中性值。输出结果乘以 calibscale / calibscale_default 值。

lux0_calibscale_default
	RO - 中性校准值

lux0_thresh_above_value
	RW - 高电平阈值。

	     所有高于该值的结果都会触发中断。65535（即 sensor_range）会禁用该高电平中断。

lux0_thresh_below_value
	RW - 低电平阈值。

	     所有低于该值的结果都会触发中断。0 会禁用该低电平中断。

prox0_raw
	RO - 测量的接近值

	     sysfs_notify 在阈值中断发生时被调用

prox0_sensor_range
	RO - prox0_raw 最大值（1023）

prox0_raw_en
	RW - 启用/禁用接近 - 使用计数逻辑

      - 1 启用接近
      - 0 禁用接近

prox0_reporting_mode
	RW - 触发/周期。

	     在 "trigger" 模式下，驱动给出两个可能的值：0 或 prox0_sensor_range 值。0 表示无接近，1023 表示有接近。这导致最少的中断次数。在 "periodic" 模式下，驱动报告所有高于 prox0_thresh_above 的值。这会导致更多中断，但可以对距离给出_粗略_估计。

prox0_reporting_mode_avail
	RO - prox0_reporting_mode 的可接受值（trigger, periodic）

prox0_thresh_above_value
	RW - 触发接近事件的阈值电平。
