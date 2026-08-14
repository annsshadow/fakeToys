

## LG Gram 笔记本额外特性


By Matan Ziv-Av <matan@svgalib.org>


### 热键


以下 FN 键在没有此驱动时会被内核忽略：

- FN-F1（LG 控制面板）   - 产生 F15
- FN-F5（触摸板开关）    - 产生 F21
- FN-F6（飞行模式）      - 产生 RFKILL
- FN-F9（阅读模式）      - 产生 F14

其余 FN 键无需特殊驱动即可工作。


### 阅读模式


向 /sys/devices/platform/lg-laptop/reader_mode 写入 0/1 可禁用/启用阅读模式。在此模式下屏幕颜色会改变（蓝色减少），并且阅读模式指示灯 LED（位于 F9 键上）亮起。


### FN 锁定


向 /sys/devices/platform/lg-laptop/fn_lock 写入 0/1 可禁用/启用 FN 锁定。


### 电池保养上限


向 /sys/class/power_supply/CMB0/charge_control_end_threshold 写入 80/100 可设置电池充电的最大容量。限制充电可减少电池容量随时间损耗。

该值在 kernel 引导时重置为 100。


### 风扇模式


向 /sys/devices/platform/lg-laptop/fan_mode 写入 0/1/2 可分别将风扇模式设为 最优/静音/性能。


### USB 充电


向 /sys/devices/platform/lg-laptop/usb_charge 写入 0/1 可在设备关机时禁用/启用从 USB 端口为另一台设备充电。

该值在 kernel 引导时重置为 0。


#### LED


驱动支持两个 LED 设备：


### 键盘背光灯


一个名为 kbd_led 的 led 设备控制键盘背光灯。共有三个亮度级别：关闭（0）、低（127）和高（255）。

键盘背光灯也由按键组合 FN-F8 控制，该组合在这些级别间循环切换。


### 触摸板指示灯 LED


位于 F5 键上。由名为 tpad_led 的 led 设备控制。
