
## video4linux 驱动中的红外遥控器支持


Authors: Gerd Hoffmann, Mauro Carvalho Chehab

## 基础


大多数模拟与数字 TV 板卡都支持遥控器。其中一些板卡带有微处理器，可接收 IR
载波，将其转换为脉冲/间隔序列，进而转换为扫描码（scancode），并返回给用户空间
（"scancode 模式"）。另一些板卡则只返回脉冲/间隔序列（"raw 模式"）。

scancode 模式下的遥控器支持由标准的 Linux input 层提供；raw 模式的支持则通过
LIRC 提供。

为了检查并测试该支持，建议下载 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_。
它提供了两个用于处理遥控器的工具：

- ir-keytable：提供查询遥控器、列出其支持的协议、在内核中启用 IR 解码器支持，
  或切换协议并测试扫描码接收的方法；

- ir-ctl：提供用于处理支持 raw 模式遥控器的工具，经由 LIRC 接口。

通常，遥控器模块会在检测到 TV 卡时自动加载。但对于少数设备，你需要手动加载
ir-kbd-i2c 模块。

## 工作原理


这些模块在 Linux input 层中将遥控器注册为键盘，也就是说，你会把遥控器上的
按键视为普通的按键（前提是启用了 CONFIG_INPUT_KEYBOARD）。

借助 event 设备（CONFIG_INPUT_EVDEV），应用程序可以通过 /dev/input/event<n>
设备访问遥控器。udev/systemd 会自动创建这些设备。若你安装了
`v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_，它可能还会自动加载一个
与默认不同的键表。详情请参阅 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_
的 ir-keytable.1 手册页。

ir-keytable 工具非常适合排查故障，例如确认 input 设备是否确实存在、它究竟是
哪个设备、按下遥控器按键时是否真的产生了事件等等。你也可以使用任何其他修改
键映射的 input 工具，例如 input kbd 工具。


### 与 lircd 配合使用


最新版本的 lircd 守护进程支持从 Linux input 层（经由 event 设备）读取事件，
同时也支持以 lirc 模式接收 IR 码。


### 不使用 lircd


Xorg 能识别若干数值小于 247 的 IR 键码。随着 Wayland 的出现，input 驱动也
得到了更新，现在应当能接受所有键码。不过，你可能仍希望将这些键码重新映射为你
常用的媒体应用程序所喜欢的键值。

这可以通过在运行时让 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_
加载你自己的键表来实现。详情请阅读 ir-keytable.1 手册页。
