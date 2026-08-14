## 并行端口设备


   :internal:

   :export:

   :export:

   :internal:

## 16x50 UART 驱动


   :export:

相关 API 请参阅 serial/driver.rst。

## 脉冲宽度调制（PWM）


脉冲宽度调制（Pulse-width modulation）是一种主要用于控制供给电气设备功率的调制技术。

PWM 框架为 PWM 信号的提供者与使用者提供了抽象。提供一个或多个 PWM 信号的控制器会被注册为 `struct pwm_chip <pwm_chip>`。提供者应将此结构嵌入到驱动相关的结构中。该结构包含描述特定芯片的字段。

一个芯片会暴露一个或多个 PWM 信号源，每个信号源以 `struct pwm_device <pwm_device>` 的形式暴露。可对 PWM 设备执行操作，以控制信号周期、占空比、极性及有效状态。

注意，PWM 设备是独占资源：任意时刻只能被一个使用者占用。

   :internal:

   :export:
