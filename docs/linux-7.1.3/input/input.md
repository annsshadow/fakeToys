
## 简

:Copyright: |copy| 1999-2001 Vojtech Pavlik <vojtech@ucw.cz> - Sponsored by SuSE

## 架构


Input 子系统是设计为支Linux 下所有输入设备的一组驱动的集合。大多数驱动位于 drivers/input，不过也有不少位drivers/hid drivers/platform
输入子系统的核心是位于最底层input 模块，它必须在任何其他输入模块之前加载——它作为两组模块之间的通信方式
### 设备驱动


这些模块与硬件对话（例如经由 USB），并向 input 模块提供事件（按键、鼠标移动）
### 事件处理程序


这些模块从输入核心获取事件，并通过各种接口将其传递到所需之处——按键送往内核，鼠标移动经由模拟的 PS/2 接口送往 GPM X，等等
## 简单用

对于最常见的配置，即一USB 鼠标和一USB 键盘，你需要加载以下模块（或将其编译进内核
```

	input
	mousedev
	usbcore
	uhci_hcd or ohci_hcd or ehci_hcd
	usbhid
	hid_generic

```
此后，USB 键盘将立即工作，USB 鼠标
```

	crw-r--r--   1 root     root      13,  63 Mar 28 22:45 mice

```
该设备通常由系统自动创建。命```

	cd /dev
	mkdir input
	mknod input/mice c 13 63

```
之后，你必须GPM（文本模式鼠标剪切粘贴工具）指向
```

	gpm -t ps2 -m /dev/input/mice

```
```

	Section "Pointer"
	    Protocol    "ImPS/2"
	    Device      "/dev/input/mice"
	    ZAxisMapping 4 5
	EndSection

```
当你完成上述所有操作后，就可以使用你的 USB 鼠标和键盘了
## 详细描述


### 事件处理程序


事件处理程序根据需要把来自设备的事件分发给用户空间和内核内消费者
#### evdev


`evdev` 是通用的输入事件接口。它把内核中产生的事件连同时间戳直接传递给程序。事件码在所有架构上都相同，且与硬件无关
这是用户空间消费用户输入的首选接口，鼓励所有客户端使用它
有关 API 的说明，请参event-interface
```

	crw-r--r--   1 root     root      13,  64 Apr  1 10:49 event0
	crw-r--r--   1 root     root      13,  65 Apr  1 10:50 event1
	crw-r--r--   1 root     root      13,  66 Apr  1 10:50 event2
	crw-r--r--   1 root     root      13,  67 Apr  1 10:50 event3
	...

```
有两组次设备号范围：64 95 是静态遗留范围。如果系统中输入设备超过 32 个，则会以从 256 开始的次设备号创建额外evdev 节点
#### keyboard


`keyboard` 是内核内的输入处理程序，VT 代码的一部分。它消费键盘按键并处VT 控制台的用户输入
#### mousedev


`mousedev` 是一个让使用鼠标输入的遗留程序得以工作的兼容层。它从鼠标或数字化仪/手写板获取事件，并向用户空间提供一PS/2 风格的（类似 /dev/psaux）鼠标设备
```

	crw-r--r--   1 root     root      13,  32 Mar 28 22:45 mouse0
	crw-r--r--   1 root     root      13,  33 Mar 29 00:41 mouse1
	crw-r--r--   1 root     root      13,  34 Mar 29 00:41 mouse2
	crw-r--r--   1 root     root      13,  35 Apr  1 10:50 mouse3
	...
	...
	crw-r--r--   1 root     root      13,  62 Apr  1 10:50 mouse30
	crw-r--r--   1 root     root      13,  63 Apr  1 10:50 mice

```
每个 `mouse` 设备都分配给单个鼠标或数字化仪，最后一`mice` 除外。这个单一字符设备被所有鼠标和数字化仪共享，即使没有连接任何设备，该设备也存在。这对于 USB 鼠标的热插拔很有用，使得不处理热插拔的旧程序即使在没有鼠标时也能打开该设备
内核配置中的 CONFIG_INPUT_MOUSEDEV_SCREEN_[XY] 是你XFree86 中屏幕的大小（像素）。如果你想在 X 中使用数字化仪，就需要它，因为其移动是通过一个虚PS/2 鼠标发送给 X 的，因此需要相应缩放。如果只使用鼠标，则不会用到这些值
Mousedev 会根据读取数据的程序的需求，生成 PS/2、ImPS/2（Microsoft IntelliMouse）或 ExplorerPS/2（IntelliMouse Explorer）协议。你可以GPM X 设为其中任意一种。如果想使用 USB 鼠标的滚轮，需ImPS/2；如果想使用额外的（最5 个）按钮，需ExplorerPS/2
#### joydev


`joydev` 实现v0.x v1.x Linux 游戏API。详情请参见 joystick-api
```

	crw-r--r--   1 root     root      13,   0 Apr  1 10:50 js0
	crw-r--r--   1 root     root      13,   1 Apr  1 10:50 js1
	crw-r--r--   1 root     root      13,   2 Apr  1 10:50 js2
	crw-r--r--   1 root     root      13,   3 Apr  1 10:50 js3
	...

```
以此类推直到遗留范围内的 js31，如果存在更多游戏杆设备，还会有次设备号大于 256 的额外节点
### 设备驱动


设备驱动是产生事件的模块
#### hid-generic


`hid-generic` 是整个套件中最大、最复杂的驱动之一。它处理所HID 设备，由于设备种类极其繁多，USB HID 规范并不简单，它需要如此庞大
目前，它处理 USB 鼠标、游戏杆、游戏手柄、方向盘、键盘、轨迹球和数字化仪
然而，USB 也把 HID 用于显示器控制、扬声器控制、UPS、LCD 以及许多其他用途
显示器和扬声器控制应当很容易加入 hid/input 接口，但对于 UPS LCD 来说意义不大。为此，设计hiddev 接口。更多信息请参见 Documentation/hid/hiddev.rst
usbhid 模块的使用非常简单，它不带任何参数，自动检测一切，当插HID 设备时，会恰当地检测到它
不过，由于设备差异极大，你可能会碰到一个工作不太好的设备。在这种情况下，请在 hid-core.c 开#define DEBUG 并把 syslog 跟踪信息发给我
#### usbmouse


对于嵌入式系统、带有损HID 描述符的鼠标，以及任何不适合使用庞大usbhid 的场合，usbmouse 驱动。它只处USB 鼠标，使用更简单的 HIDBP 协议。这也意味着鼠标必须支持这个更简单的协议，但并非所有鼠标都支持。如果没有什么强烈理由使用这个模块，请改usbhid
#### usbkbd


usbmouse 类似，这个模块用简化的 HIDBP 协议与键盘通信。它更小，但不支持任何额外的特殊按键。如果没有特殊理由使用它，请改用 usbhid
#### psmouse


这是适用于所有使PS/2 协议的指点设备的驱动，包Synaptics ALPS 触控板、Intellimouse Explorer 设备、Logitech PS/2 鼠标等等
#### atkbd


这是用于 PS/2（AT）键盘的驱动
#### iforce


用于 I-Force 游戏杆和方向盘的驱动，可通过 USB RS232。它现在包含力反馈（Force Feedback）支持，尽管 Immersion Corp. 将协议视为商业机密，一个字也不愿透露
## 验证是否工作


在键盘上敲几个键，就足以检查键盘是否工作并已正确连接到内核键盘驱动
执行 `cat /dev/input/mouse0`（c, 13, 32）可以验证鼠标也被模拟出来了；移动鼠标时应当出现字符
你可以用 `jstest` 工具测试游戏杆模拟，该工具在 joystick 软件包中可用（参joystick-doc）
你可以用 `evtest` 工具测试事件设备
## 事件接口


你可以使用阻塞和非阻塞读取，也可以在 /dev/input/eventX 设备上使select()，你总是会得到一个整数个的输```

    struct input_event {
	    struct timeval time;
	    unsigned short type;
	    unsigned short code;
	    int value;
    };

```
`time` 是时间戳，它返回事件发生的时间。`type` 例如相对移动EV_REL、按键按下或松开EV_KEY。更多类型定义于 include/uapi/linux/input-event-codes.h
`code` 是事件码，例REL_X KEY_BACKSPACE，完整列表同样在 include/uapi/linux/input-event-codes.h 中
`value` 是事件携带的值。对EV_REL 是相对变化量，对EV_ABS（游戏杆……）是绝对新值，对于 EV_KEY 则是：松开0，按下为 1，自动重复为 2
有关各种事件码的更多信息，请参见 input-event-codes