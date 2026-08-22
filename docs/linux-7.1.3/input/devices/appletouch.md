
### Apple 触摸板驱动（appletouch

:Copyright: |copy| 2005 Stelian Pop <stelian@popies.net>

appletouch 是一Linux 内核驱动，用2005 2 月和 2005 10 月之后出产的 Apple Aluminium Powerbook 上所配备USB 触摸板
该驱动派生自 Johannes Berg appletrackpad 驱动 [#f1]_，但在一些方面进行了改进
 - appletouch 是一个完整的内核驱动，无需任何用户空间程序
 - appletouch 可以synaptics X11 驱动对接，从而获得触摸板加速、滚动等功能

感谢 Johannes Berg 对触摸板协议进行的逆向工程，Frank Arnold 的进一步完善，以及 Alex Harper 提供的关于触摸板传感器内部工作原理的额外信息。Michael Hanselmann 增加了对 2005 10 月型号的支持
### 用法


要在基本模式下使用触摸板，编译驱动并加载模块。系统将检测到一个新的输入设备，你可以从 /dev/input/mice 读取鼠标数据（使gpm X11）
X11 中，你可以将触摸板配置为使用 synaptics X11 驱动，从而获得额外的功能，如加速、滚动、双指点击模拟中键、三指点击模拟右键等。为此，请确保你使用的是较新版本synaptics 驱动（已0.14.2 上测试，可从 [#f2]_ 获取），并在你的 X11 配置文件中配置一个新的输入设备（配置示例见下

```

	Section "InputDevice"
		Identifier      "Synaptics Touchpad"
		Driver          "synaptics"
		Option          "SendCoreEvents"        "true"
		Option          "Device"                "/dev/input/mice"
		Option          "Protocol"              "auto-dev"
		Option		"LeftEdge"		"0"
		Option		"RightEdge"		"850"
		Option		"TopEdge"		"0"
		Option		"BottomEdge"		"645"
		Option		"MinSpeed"		"0.4"
		Option		"MaxSpeed"		"1"
		Option		"AccelFactor"		"0.02"
		Option		"FingerLow"		"0"
		Option		"FingerHigh"		"30"
		Option		"MaxTapMove"		"20"
		Option		"MaxTapTime"		"100"
		Option		"HorizScrollDelta"	"0"
		Option		"VertScrollDelta"	"30"
		Option		"SHMConfig"		"on"
	EndSection

	Section "ServerLayout"
		...
		InputDevice	"Mouse"
		InputDevice	"Synaptics Touchpad"
	...
	EndSection

```

### 抖动问题


触摸板传感器对热量非常敏感，当温度变化时会产生大量噪声。首次给笔记本电脑上电时尤其明显
appletouch 驱动会尝试处理此噪声并自动适应，但它并非完美。如果手指移动不再被识别，请尝试重新加载驱动
你可以使'debug' 模块参数开启调试。值为 0 关闭所有调试，1 开启对无效采样的跟踪，2 开
```

	modprobe appletouch debug=1

```

```

	echo "1" > /sys/module/appletouch/parameters/debug


```
