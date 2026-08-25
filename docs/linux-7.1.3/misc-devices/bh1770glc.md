
## 内核驱动 bh1770glc


支持的芯片：

- ROHM BH1770GLC
- OSRAM SFH7770

数据手册不可自由获取

作者：
Samu Onkalo <samu.p.onkalo@nokia.com>

### 描述


BH1770GLC SFH7770 是环境光（ambient light）与接近（proximity）二合一传感器。ALS 与接部分各自独立运行，但它们共享通用I2C 接口与中断逻辑。原则上它们可以独立运行，但 ALS 侧的
结果被用于估计接近传感器的可靠性
ALS 产生 16 lux 值。该芯片包含中断逻辑，可产生低阈值与高阈值中断
接近部分包含最3 IR-led 的驱动。该芯片测量反射 IR 光的量并产生接近结果。分辨率8 位驱动仅支持一个通道。驱动使ALS 结果来估计接近结果的可靠性。因此，在需要接近检测时，ALS
始终在运行
驱动使用阈值中断以避免轮询（polling）数值的需要。芯片中不存在接近低中断，这通过延迟工作
（delayed work）来模拟。只要存在高于阈值的接近中断，延迟工作就会被不断推后。因此，当接水平降到阈值以下时，没有中断产生，延迟工作最终会运行。这被当作“无接近”指示来处理
芯片状态在配置启用时通过运行时电源管理（runtime pm）框架控制
Calibscale 因子用于隐藏芯片之间的差异。默认情况下该值设为中性状态，即系数为 1.00。为获得
正确的值，需要以经过校准的光源作为参考。Calibscale 因子被设置为使测量产生大约预期的 lux 值
### SYSFS


chip_id
	RO - 显示检测到的芯片类型与版本

power_state
	RW - 启用 / 禁用芯片

	使用计数逻辑

      - 1 启用芯片
      - 0 禁用芯片

lux0_input
	RO - 测量得到lux 
	     sysfs_notify 在阈值中断发生时被调
lux0_sensor_range
	RO - lux0_input 的最大
lux0_rate
	RW - 测量速率（Hz
lux0_rate_avail
	RO - 支持的测量速率

lux0_thresh_above_value
	RW - 高（HI）电平阈
	     所有高于该值的结果都会触发中断5535（即 sensor_range）会禁用上述中断
lux0_thresh_below_value
	RW - 低（LO）电平阈
	     所有低于该值的结果都会触发中断 会禁用下方中断
lux0_calibscale
	RW - 鏍″噯鍊。
	     默认设为中性值	     输出结果乘以 calibscale / calibscale_default 值
lux0_calibscale_default
	RO - 中性校准
prox0_raw
	RO - 测量得到的接近
	     sysfs_notify 在阈值中断发生时被调
prox0_sensor_range
	RO - prox0_raw 的最大
prox0_raw_en
	RW - 启用 / 禁用接近

	     使用计数逻辑

      - 1 启用接近
      - 0 禁用接近

prox0_thresh_above_count
	RW - 触发事件前所需的接近中断次
prox0_rate_above
	RW - 电平高于阈值时的测量速率（Hz	即当报告了“接近开启”时
prox0_rate_below
	RW - 电平低于阈值时的测量速率（Hz	即当报告了“接近关闭”时
prox0_rate_avail
	RO - 支持的接近测量速率（Hz
prox0_thresh_above0_value
	RW - 触发接近事件的阈值电平
	     由持久化过滤器（prox0_thresh_above_count）过
prox0_thresh_above1_value
	RW - 立即触发事件的阈值电