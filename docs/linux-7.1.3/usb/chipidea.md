## ChipIdea 高速双角色控制器驱

### 1. 如何测试 OTG FSM（HNP SRP

展示如何通过 sys 输入文件，用 2 Freescale i.MX6Q sabre SD 板演OTG HNP SRP 功能
### 1.1 如何启用 OTG FSM


##### 1.1.1 menuconfig 中选择 CONFIG_USB_OTG_FSM，重新构建内

映像与模块。如果你想检otg fsm 的一些内部变量，挂载 debugfs，有以下 2 个文```

	cat /sys/kernel/debug/ci_hdrc.0/otg
	cat /sys/kernel/debug/ci_hdrc.0/registers

```
##### 1.1.2 在你dts 文件中为你的控制器节点添加以下条

```

	otg-rev = <0x0200>;
	adp-disable;

```
### 1.2 测试操作


1) 用已加载 gadget 类驱动（例如 g_mass_storage）的 2 Freescale i.MX6Q sabre SD 板上电
2) usb 线缆连接 2 块板：一端是 micro A 插头，另一端是 micro B 插头
   A 设备（插micro A 插头）应当枚B 设备
3) 角色切换

```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   B 设备应当担当主机角色并枚A 设备
```
4) A 设备切回主机
```

	echo 0 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   或者，通过引入 HNP 轮询，B-Host 可以知道 A-peripheral 何时希望处于主机角色，因此此角色切换也可   A-peripheral 端通过应答来自 B-Host 的轮询来触发。这可以A 设备上完:

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_req

   A 设备应当切回主机并枚B 设备
```
5) 移除 B 设备（拔micro B 插头）并10 秒内重新插入；A 设备应当再次枚举 B 设备
6) 移除 B 设备（拔micro B 插头）并10 秒后重新插入；A 设备应当***枚举 B 设备
   如果 A 设备想要使用总线
```

	echo 0 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_drop
	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_req

   如果 B 设备想要使用总线
   B 设备:

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

```
7) A 设备断电总线
```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_drop

   A 设备应当B 设备断开并断电总线
```
8) B 设备SRP 做数据脉冲
```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   A 设备应当恢复 usb 总线并枚B 设备
```
### 1.3 参考文

"On-The-Go and Embedded Host Supplement to the USB Revision 2.0 Specification
July 27, 2012 Revision 2.0 version 1.1a"

### 2. 如何USB 启用为系统唤醒源


以下是如何在 imx6 平台上将 USB 启用为系统唤醒源的示例
```

	echo enabled > /sys/bus/platform/devices/ci_hdrc.0/power/wakeup

```
```

	echo enabled > /sys/bus/platform/devices/2184000.usb/power/wakeup

```
```

	echo enabled > /sys/bus/platform/devices/20c9000.usbphy/power/wakeup

```
```

	echo enabled > /sys/bus/usb/devices/usb1/power/wakeup

```
```

	echo enabled > /sys/bus/usb/devices/1-1/power/wakeup

```
如果系统只有一usb 端口，并且你想在该端口启usb 唤醒，你
```

	for i in $(find /sys -name wakeup | grep usb);do echo enabled > $i;done;

```
