## 一次LED 触发


这是一LED 触发器，适用于在没有明确的陷阱点来放置标led-on led-off 设置的事件中向用户发出信号。使用该触发器，应用程序只需要在事件发生时向触发器发出信号，然后触发器点LED，并在指定时间内保持熄灭

该触发器旨在既可用于偶发事件，也可用于密集事件。在前一种情况下，触发器为每个事件产生一次清晰、受控的闪烁；而在后一种情况下，它以恒定频率持续闪烁，以表明事件正在连续到达

一次LED 仅在没有事件时保持恒定状态。附加的 "invert" 属性指LED 在未重新触发时是保持熄灭（正常）还是点亮（反转）

可以从用户空间在 led class 设备上激活该触发器，如下所
```
  echo oneshot > trigger
```
这会向该 LED 添加 sysfs 属性，这些属性在以下文档中有说明
Documentation/ABI/testing/sysfs-class-led-trigger-oneshot

```
  echo oneshot > trigger # 设置led 的触发器
  echo 33 > delay_on     # 在连续流量下1 / (33 + 33) Hz 闪烁
  echo 33 > delay_off

```
```
  echo 1 > invert # led 设为常亮，点led

```
```
  echo 1 > shot # led 开始闪烁，若已在闪烁则忽略

```
```
  echo 0 > invert # led 设为常灭，熄led

```
