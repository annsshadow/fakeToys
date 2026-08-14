## Leds BlinkM 驱动


leds-blinkm 驱动支持 BlinkM 系列的设备。

它们是 RGB-LED 模块，由 (AT)tiny 微控制器驱动，并通过 I2C 通信。这些模块的默认
地址是 0x09，但可以通过命令更改。这样你可以在一条 I2C 总线上以菊花链方式连接最多
127 个 BlinkM。

该设备通过独立的命令接受 RGB 与 HSB 颜色值。你也可以在控制器中把闪烁序列存储为
“脚本”（scripts）并运行它们。渐变（fading）也是一个可选项。

本驱动提供的接口有三层：

# a) 用于配合触发器使用的 LED 多色类接口


```

  blinkm-<i2c-bus-nr>-<i2c-device-nr>:rgb:indicator

  $ ls -h /sys/class/leds/blinkm-1-9:rgb:indicator
  brightness  device  max_brightness  multi_index  multi_intensity  power  subsystem  trigger  uevent

```
色相（Hue）由 multi_intensity 文件控制，亮度（lightness）由 brightness 文件控制。

写入强度值的顺序可以在 multi_index 中找到。必须向 multi_intensity 写入正好三个
介于 0 到 255 之间的值，以：

```

  $ echo 255 100 50 > multi_intensity

```
通过向 brightness 文件写入一个介于 0 到 255 之间的值，可以改变整体亮度。

# b) 用于配合触发器使用的 LED 类接口


```

  blinkm-<i2c-bus-nr>-<i2c-device-nr>-<color>

  $ ls -h /sys/class/leds/blinkm-6-*
  /sys/class/leds/blinkm-6-9-blue:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

  /sys/class/leds/blinkm-6-9-green:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

  /sys/class/leds/blinkm-6-9-red:
  brightness  device  max_brightness  power  subsystem  trigger  uevent

```
（/sys/bus/i2c/devices/6-0009/leds 中相同）

我们可以将颜色拆分为红、绿、蓝分别控制，并为每种颜色分配触发器。

```

  $ cat blinkm-6-9-blue/brightness
  05

  $ echo 200 > blinkm-6-9-blue/brightness
  $

  $ modprobe ledtrig-heartbeat
  $ echo heartbeat > blinkm-6-9-green/trigger
  $


```
# b) 用于控制 rgb、fade、hsb、scripts 的 Sysfs 组 ...


此扩展接口作为 blinkm 文件夹，位于 I2C 设备的 sysfs 文件夹中。例如位于
/sys/bus/i2c/devices/6-0009/blinkm 下

  $ ls -h /sys/bus/i2c/devices/6-0009/blinkm/
  blue  green  red  test

目前仅支持设置红、绿、蓝以及一个测试序列。

```

  $ cat *
  00
  00
  00
  #Write into test to start test sequence!#

  $ echo 1 > test
  $

  $ echo 255 > red
  $



```
截至 2024 年 07 月

dl9pf <at> gmx <dot> de
jstrauss <at> mailbox <dot> org
