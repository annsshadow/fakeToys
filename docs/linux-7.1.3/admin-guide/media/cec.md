
## HDMI CEC


## 主线中受支持的硬件


HDMI 发送器：

- Exynos4
- Exynos5
- STIH4xx HDMI CEC
- V4L2 adv7511（相同的硬件，但是与 drm adv7511 不同的驱动）
- stm32
- Allwinner A10 (sun4i)
- Raspberry Pi
- dw-hdmi (Synopsis IP)
- amlogic (meson ao-cec 和 ao-cec-g12a)
- drm adv7511/adv7533
- omap4
- tegra
- rk3288, rk3399
- tda998x
- DisplayPort CEC-Tunneling-over-AUX on i915, nouveau and amdgpu
- ChromeOS EC CEC
- CEC for SECO boards (UDOO x86).
- Chrontel CH7322


HDMI 接收器：

- adv7604/11/12
- adv7842
- tc358743

USB 加密狗（有关如何使用这些加密狗的更多信息，请参见下文）：

- Pulse-Eight：pulse8-cec 驱动实现了以下模块选项：`persistent_config`：默认情况下它是
  关闭的，但当设为 1 时，驱动会将当前设置存储到设备内部 eeprom 中，并在下次设备连接到
  USB 端口时恢复。

- RainShadow Tech。注意：该驱动不支持 Pulse-Eight 驱动的 persistent_config 模块选项。
  硬件支持该功能，但我不打算添加此特性。不过我接受补丁 :-)

- Extron DA HD 4K PLUS HDMI 分配放大器。更多信息请参见 extron_da_hd_4k_plus。

杂项：

- vivid：模拟一个 CEC 接收器和一个 CEC 发送器。可用于在没有实际 CEC 硬件的情况下测试
  CEC 应用程序。

- cec-gpio。如果 CEC 引脚连接到 GPIO 引脚，则可以通过此驱动控制 CEC 线路。它还支持错误
  注入。

- cec-gpio 以及 Allwinner A10（或任何其他使用 CEC 引脚框架直接驱动 CEC 引脚的驱动）：
  CEC 引脚框架使用高分辨率定时器。这些定时器会受到 NTP 守护进程的影响，后者会加快或减慢
  时钟以与官方时间同步。chronyd 服务器默认会将时钟加快或减慢 1/12。这会导致 CEC 时序
  超出规范。要修复此问题，可在 chronyd.conf 中添加一行 'maxslewrate 40000'。这会将时钟
  频率变化限制在 1/25，从而使 CEC 时序保持在规范内。


## 工具


工具可在此处获取：https://git.linuxtv.org/v4l-utils.git

`utils/cec-ctl`：控制 CEC 设备

`utils/cec-compliance`：测试远程 CEC 设备的合规性

`utils/cec-follower`：模拟一个 CEC 从设备

注意，`cec-ctl` 支持某些酒店显示屏使用的 CEC Hospitality Profile。请参见
http://www.htng.org。

注意，libcec 库（https://github.com/Pulse-Eight/libcec）支持 linux CEC 框架。

如果你想获取 CEC 规范，请查看 HDMI 维基百科页面的参考文献：
https://en.wikipedia.org/wiki/HDMI。CEC 是 HDMI 规范的一部分。HDMI 1.3 可以免费获取
（在 CEC 方面与 HDMI 1.4 非常相似），对大多数用途应该足够。


## 支持 CEC 的 DisplayPort 转 HDMI 适配器


背景：大多数适配器不支持 CEC Tunneling 特性，而在支持该特性的适配器中，许多实际上并未
连接 CEC 引脚。不幸的是，这意味着虽然创建了 CEC 设备，但它实际上在世界上形单影只，永远
无法看到其他 CEC 设备。

这是一个已知的可用适配器列表，它们具有 CEC Tunneling 并且正确连接了 CEC 引脚。如果你发现
可用的适配器但不在本列表中，请给我留言。

测试方法：将 DP 转 HDMI 适配器连接到支持 CEC 的设备

```

	cec-ctl --playback	# Configure the PC as a CEC Playback device
	cec-ctl -S		# Show the CEC topology

```
`cec-ctl -S` 命令应至少显示两个 CEC 设备：我们自己和所连接的 CEC 设备（通常是电视）。

一般说明：我只在 Parade PS175、PS176 和 PS186 芯片组以及 MegaChips 2900 上见过它正常工作。
虽然 MegaChips 28x0 声称支持 CEC，但我从未见过它工作。

### USB-C 转 HDMI


Samsung Multiport Adapter EE-PW700: https://www.samsung.com/ie/support/model/EE-PW700BBEGWW/

Kramer ADC-U31C/HF: https://www.kramerav.com/product/ADC-U31C/HF

Club3D CAC-2504: https://www.club-3d.com/en/detail/2449/usb_3.1_type_c_to_hdmi_2.0_uhd_4k_60hz_active_adapter/

### DisplayPort 转 HDMI


Club3D CAC-1080: https://www.club-3d.com/en/detail/2442/displayport_1.4_to_hdmi_2.0b_hdr/

CableCreation (SKU: CD0712): https://www.cablecreation.com/products/active-displayport-to-hdmi-adapter-4k-hdr

HP DisplayPort to HDMI True 4k Adapter (P/N 2JA63AA): https://www.hp.com/us-en/shop/pdp/hp-displayport-to-hdmi-true-4k-adapter

### Mini-DisplayPort 转 HDMI


Club3D CAC-1180: https://www.club-3d.com/en/detail/2443/mini_displayport_1.4_to_hdmi_2.0b_hdr/

注意，无源适配器永远不会工作，你需要有源适配器。

本列表中的 Club3D 适配器都基于 MegaChips 2900。其他 Club3D 适配器基于 PS176 且没有连接
CEC 引脚，因此只有上述三款 Club3D 适配器已知可用。

我怀疑基于 MegaChips 2900 的设计通常可能可用，而 PS176 则更像是碰运气（大多不可用）。
PS186 很可能连接了 CEC 引脚，看起来他们更改了该芯片组的参考设计。


## USB CEC 加密狗


这些加密狗显示为 `/dev/ttyACMX` 设备，需要 `inputattach` 工具来创建 `/dev/cecX` 设备。
对 Pulse-Eight 的支持已添加到 `inputattach` 1.6.0。对 Rainshadow Tech 的支持已添加到
`inputattach` 1.6.1。

```

	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="2548", ATTRS{idProduct}=="1002", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="pulse8-cec-inputattach@%k.service"
	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="2548", ATTRS{idProduct}=="1001", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="pulse8-cec-inputattach@%k.service"
	SUBSYSTEM=="tty", KERNEL=="ttyACM[0-9]*", ATTRS{idVendor}=="04d8", ATTRS{idProduct}=="ff59", ACTION=="add", TAG+="systemd", ENV{SYSTEMD_WANTS}+="rainshadow-cec-inputattach@%k.service"

```
以及这些 systemd 服务：

```

	[Unit]
	Description=inputattach for pulse8-cec device on %I

	[Service]
	Type=simple
	ExecStart=/usr/bin/inputattach --pulse8-cec /dev/%I

```
```

	[Unit]
	Description=inputattach for rainshadow-cec device on %I

	[Service]
	Type=simple
	ExecStart=/usr/bin/inputattach --rainshadow-cec /dev/%I


```
```

	[Unit]
	Description=restart inputattach for cec devices
	After=suspend.target

	[Service]
	Type=forking
	ExecStart=/bin/bash -c 'for d in /dev/serial/by-id/usb-Pulse-Eight*; do /usr/bin/inputattach --daemon --pulse8-cec $d; done; for d in /dev/serial/by-id/usb-RainShadow_Tech*; do /usr/bin/inputattach --daemon --rainshadow-cec $d; done'

	[Install]
	WantedBy=suspend.target

```
并运行 `systemctl enable restart-cec-inputattach`。

要在 CEC 设备创建时自动设置其物理地址

```

	cec-ctl -E /sys/class/drm/card0-DP-1/edid

```
这假设加密狗连接到 card0-DP-1 输出（`xrandr` 会告诉你使用的是哪个输出），它会轮询 EDID
的变化并在发生变化时更新物理地址。

要自动运行此命令，可以使用 cron。用以下方式编辑 crontab：

```

	@reboot /usr/local/bin/cec-ctl -E /sys/class/drm/card0-DP-1/edid

```
这仅适用于在 `/sys/class/drm` 中暴露 EDID 的显示驱动，例如 i915 驱动。


## 无 HPD 的 CEC


某些显示器在待机模式下没有 HDMI 热插拔检测（Hotplug Detect）信号，但 CEC 仍然启用，因此
连接的设备可以发送 <Image View On> CEC 消息以唤醒此类显示器。不幸的是，并非所有 CEC 适配器
都支持这一点。例如 Odroid-U3 SBC，其电平转换器在 HPD 信号为低电平时断电，从而阻塞 CEC
引脚。即使 SoC 可以在无 HPD 的情况下使用 CEC，电平转换器也会阻止其正常工作。

有一个 CEC 能力标志来表示这一点：`CEC_CAP_NEEDS_HPD`。如果设置了该标志，则硬件无法以
这种方式唤醒显示器。

给 CEC 应用程序实现者的提示：<Image View On> 消息必须是你发送的第一条消息，在此之前不要
发送任何其他消息。某些非常糟糕但不幸地并不少见的 CEC 实现，如果收到除该消息以外的任何
消息，就会变得非常混乱，从而不会唤醒。

编写驱动时，测试这一点可能很棘手。有两种方法可以做到：

1) 获取一个 Pulse-Eight USB CEC 加密狗，用 HDMI 线缆将你的设备连接到 Pulse-Eight，但
   不要将 Pulse-Eight 连接到显示器。

```

	cec-ctl -p0.0.0.0 --tv

   and start monitoring::

	sudo cec-ctl -M

   On the device you are testing run::

	cec-ctl --playback

   It should report a physical address of f.f.f.f. Now run this
   command::

	cec-ctl -t0 --image-view-on

   The Pulse-Eight should see the <Image View On> message. If not,
   then something (hardware and/or software) is preventing the CEC
   message from going out.

   To make sure you have the wiring correct just connect the
   Pulse-Eight to a CEC-enabled display and run the same command
   on your device: now there is a HPD, so you should see the command
   arriving at the Pulse-Eight.

```
2) 如果你有另一台支持无 HPD CEC 的 linux 设备，则可以直接将你的设备连接到该设备。是的，
   你可以将两个 HDMI 输出连接在一起。你将没有 HPD（这正是我们在此测试中想要的），但第二台
   设备可以监控 CEC 引脚。否则使用与 1 相同的命令。

如果没有 HPD 时 CEC 消息无法通过，则需要找出原因。通常这要么是硬件限制，要么是软件在 HPD
变低时关闭了 CEC 核心。前者当然无法纠正，后者很可能需要修改驱动。


## 微控制器与 CEC


我们见过一些显示器中的 CEC 实现使用微控制器对总线进行采样。这不一定是个问题，但某些实现
存在时序问题。除非你能接上一个底层的 CEC 调试器（参见下一节），否则很难发现这一点。

你会看到 CEC 发送器将 CEC 线路拉高或拉低的时间超过允许值的情况。对于定向消息，这不是问题，
因为如果发生这种情况，消息不会被确认（Acked），并将被重传。对于广播消息则不存在这种机制。

目前尚不清楚该如何处理。明智的做法可能是将某些广播消息发送两次，以降低它们丢失的概率。
具体而言，<Standby> 和 <Active Source> 是这类消息的候选。


## 制作一个 CEC 调试器


通过使用 Raspberry Pi 4B 和一些廉价组件，你可以制作自己的底层 CEC 调试器。

关键组件是以下 HDMI 母对母直通连接器之一（全焊接型 1）：

https://elabbay.myshopify.com/collections/camera/products/hdmi-af-af-v1a-hdmi-type-a-female-to-hdmi-type-a-female-pass-through-adapter-breakout-board?variant=45533926147

视频质量不稳定，肯定不足以直通 4kp60（594 MHz）视频。你可能能够支持 4kp30，但更可能受限于
1080p60（148.5 MHz）。但对于 CEC 测试来说这已经足够。

你需要一个面包板和一些面包板线：

http://www.dx.com/p/diy-40p-male-to-female-male-to-male-female-to-female-dupont-line-wire-3pcs-356089#.WYLOOXWGN7I

如果你还想监控 HPD 和/或 5V 线路，那么你需要以下 5V 到 3.3V 电平转换器之一：

https://www.adafruit.com/product/757

（这只是我购买这些组件的地方，你还可以从许多其他地方买到类似的东西）。

当然，HDMI 连接器的地引脚需要连接到 Raspberry Pi 的地引脚。

HDMI 连接器的 CEC 引脚需要连接到以下引脚：GPIO 6 和 GPIO 7。HDMI 连接器可选的 HPD 引脚
应通过电平转换器连接到以下引脚：GPIO 23 和 GPIO 12。HDMI 连接器可选的 5V 引脚应通过电平
转换器连接到以下引脚：GPIO 25 和 GPIO 22。监控 HPD 和 5V 线路不是必需的，但很有帮助。

在 `arch/arm/boot/dts/bcm2711-rpi-4-b.dts` 中添加以下设备树：

```

	cec@6 {
		compatible = "cec-gpio";
		cec-gpios = <&gpio 6 (GPIO_ACTIVE_HIGH|GPIO_OPEN_DRAIN)>;
		hpd-gpios = <&gpio 23 GPIO_ACTIVE_HIGH>;
		v5-gpios = <&gpio 25 GPIO_ACTIVE_HIGH>;
	};

	cec@7 {
		compatible = "cec-gpio";
		cec-gpios = <&gpio 7 (GPIO_ACTIVE_HIGH|GPIO_OPEN_DRAIN)>;
		hpd-gpios = <&gpio 12 GPIO_ACTIVE_HIGH>;
		v5-gpios = <&gpio 22 GPIO_ACTIVE_HIGH>;
	};

```
如果你没有连接 HPD 和/或 5V 线路，则只需删除这些行。

这个 dts 改动将启用两个 cec GPIO 设备：我通常用一个来发送/接收 CEC 命令，另一个用于监控。
如果你使用未配置的 CEC 适配器进行监控，它将使用 GPIO 中断，从而使监控非常精确。

如果你只想监控流量，那么单个实例就足够了。最小配置是一个 HDMI 母对母直通连接器以及两根
母对母面包板线：一根用于将 HDMI 地引脚连接到 Raspberry Pi 上的地引脚，另一根用于将 HDMI
CEC 引脚连接到 Raspberry Pi 上的 GPIO 6。

有关如何使用错误注入的文档请参见：cec_pin_error_inj。

`cec-ctl --monitor-pin` 将执行底层的 CEC 总线嗅探和分析。你还可以使用 `--store-pin` 将
CEC 流量存储到文件，并使用 `--analyze-pin` 稍后分析。

你还可以将其配置为完整的 CEC 设备，使用 `cec-ctl --tv -p0.0.0.0` 或
`cec-ctl --playback -p1.0.0.0`。


## Extron DA HD 4K PLUS CEC 适配器驱动


此驱动用于 Extron DA HD 4K PLUS 系列 HDMI 分配放大器：
https://www.extron.com/product/dahd4kplusseries

支持 2、4 和 6 端口型号。

需要固件版本 1.02.0001 或更高。

注意，较旧的 Extron 硬件版本存在 CEC 电压问题，这可能意味着 CEC 无法工作。该问题在硬件
版本 E34814 及更高版本中得到修复。

CEC 支持有两种模式：第一种是手动模式，用户空间必须手动控制 HDMI 输入和所有 HDMI 输出的
CEC。虽然这提供了完全的控制权，但也比较复杂。

第二种是自动模式，当设置了模块选项 `vendor_id` 时选中。在这种情况下，驱动控制 CEC，并且
在输入中接收到的 CEC 消息将被分发到各个输出。仍然可以使用 /dev/cecX 设备直接与所连接的
设备通信，但所有配置以及热插拔检测变化等事务都由驱动处理。

驱动还负责 EDID：会创建 /dev/videoX 设备来读取 EDID 以及（针对 HDMI 输入端口）设置 EDID。

默认情况下，用户空间负责根据所连接显示器的 EDID 为 HDMI 输入设置 EDID。但如果设置了
`manufacturer_name` 模块选项，则驱动将根据所连接显示器支持的分辨率为 HDMI 输入设置 EDID。
目前驱动仅支持 1080p60 和 4kp60 分辨率：如果所有连接的显示器都支持 4kp60，则它会在 HDMI
输入上通告 4kp60，否则将回退到仅报告 1080p60 的 EDID。

Extron 的状态报告在 `/sys/kernel/debug/cec/cecX/status` 中。

extron-da-hd-4k-plus 驱动实现了以下模块选项：

### ``debug``

如果设为 1，则显示所有串口流量。

### ``vendor_id``

要报告给所连接显示器的 CEC 厂商 ID。

如果设置，则驱动负责将输入中接收到的 CEC 消息分发到 HDMI 输出。以下 CEC 消息会进行此处理：

- <Standby>
- <Image View On> 和 <Text View On>
- <Give Device Power Status>
- <Set System Audio Mode>
- <Request Current Latency>

如果未设置，则用户空间负责此事，并且必须手动为 HDMI 输入和 HDMI 输出配置 CEC 设备。

### ``manufacturer_name``

用于 HDMI 输入 EDID 的三字符厂商名称。如果未设置，则用户空间负责配置 EDID。如果设置，则
驱动将根据所连接显示器支持的分辨率自动更新 EDID，并且将无法再手动设置 HDMI 输入的 EDID。

### ``hpd_never_low``

如果设置，则 HDMI 输入的 Hotplug Detect 引脚将始终为高电平，即使没有任何东西连接到 HDMI
输出。如果未设置（默认），则当 HDMI 输出的所有检测到的 Hotplug Detect 引脚也为低电平时，
HDMI 输入的 Hotplug Detect 引脚将变低。

此选项可以动态更改。
