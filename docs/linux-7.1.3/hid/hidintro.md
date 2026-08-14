
## HID 报告描述符简介


本章旨在广泛概述 HID 报告描述符是什么，以及一个普通的（非内核）程序员如何处理在 Linux
下工作不正常的 HID 设备。

    :local:
    :depth: 2

- [hidreport-parsing](hidreport-parsing)

## 简介


HID 代表 Human Interface Device（人机接口设备），可以是你用来与计算机交互的任何设备，
无论是鼠标、触摸板、数位板还是麦克风。

许多 HID 设备开箱即用（out of the box），即使它们的硬件各不相同。例如，鼠标可以有任意
数量的按钮；它们可能有一个滚轮；不同型号之间的移动灵敏度不同，等等。尽管如此，大多数
时候一切都能正常工作，而无需为自 1970 年以来开发的每个鼠标型号在内核中编写专门的代码。

这是因为现代 HID 设备确实通过其 **HID 报告描述符**（HID report descriptor）——一组
固定的字节，精确描述设备与主机之间可以发送哪些 **HID 报告**（HID reports）以及这些
报告中每个单独位的含义——来声明其能力。例如，一个 HID 报告描述符可以指定 “在 ID 为 3
的报告中，第 8 到 15 位是鼠标的 X 增量坐标”。

HID 报告本身随后仅携带实际数据值，没有任何额外的元信息。请注意，HID 报告可以从设备
发出（"Input Reports"，即输入事件）、发往设备（"Output Reports"，例如用于改变 LED）或
用于设备配置（"Feature reports"）。一个设备可以支持一个或多个 HID 报告。

HID 子系统负责解析 HID 报告描述符，并将 HID 事件转换为正常的输入设备接口（参见
Documentation/hid/hid-transport.rst）。设备可能行为异常，原因包括设备提供的 HID 报告
描述符有误、需要以特殊方式处理，或默认代码未处理某些特殊设备或交互模式。

HID 报告描述符的格式由两个文档描述，可从 `USB Implementers Forum
<https://www.usb.org/>`_ `HID web page <https://www.usb.org/hid>`_ 地址获取：

 - the `HID USB Device Class Definition
   <https://www.usb.org/document-library/device-class-definition-hid-111>`_ (HID Spec from now on)
 - the `HID Usage Tables <https://usb.org/document-library/hid-usage-tables-14>`_ (HUT from now on)

HID 子系统可以处理不同的传输（transport）驱动（USB、I2C、Bluetooth 等）。参见
Documentation/hid/hid-transport.rst。

## 解析 HID 报告描述符


当前 HID 设备的列表可在 `/sys/bus/hid/devices/` 找到。对于每个设备，例如
`/sys/bus/hid/devices/0003\:093A\:2510.0002/`，
```
  $ hexdump -C /sys/bus/hid/devices/0003\:093A\:2510.0002/report_descriptor
  00000000  05 01 09 02 a1 01 09 01  a1 00 05 09 19 01 29 03  |..............).|
  00000010  15 00 25 01 75 01 95 03  81 02 75 05 95 01 81 01  |..%.u.....u.....|
  00000020  05 01 09 30 09 31 09 38  15 81 25 7f 75 08 95 03  |...0.1.8..%.u...|
  00000030  81 06 c0 c0                                       |....|
  00000034
```

可选：HID 报告描述符也可以通过直接访问 hidraw 驱动 [#hidraw]_ 来读取。

HID 报告描述符的基本结构在 HID 规范中定义，而 HUT “定义了一组常量，可供应用程序解释以
识别 HID 报告中数据字段的用途和含义”。每个条目至少由两字节定义，其中第一个字节定义
后面跟随的值的类型，并在 HID 规范中描述；第二个字节携带实际值，并在 HUT 中描述。

原则上，HID 报告描述符可以逐字节地、费力地手工解析。

关于如何做到这一点的简短介绍概述于 Documentation/hid/hidreport-parsing.rst；只有当你
需要修补（patch）HID 报告描述符时才需要理解它。

在实践中，你不应手工解析 HID 报告描述符；相反，你应当使用现有的解析器。在所有可用的
解析器中：

  - 在线的 `USB Descriptor and Request Parser
    <http://eleccelerator.com/usbdescreqparser/>`_；
  - `hidrdd <https://github.com/abend0c1/hidrdd>`_，
    它提供非常详细且有些冗长的描述（如果你不熟悉 HID 报告描述符，这种冗长可能很有用）；
  - `hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_，
    一套完整的实用工具集，除其它功能外，允许记录和回放原始的 HID 报告，以及调试和回放
    HID 设备。它正由 Linux HID 子系统维护者积极开发。

用 `hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_ 解析鼠标的 HID 报告
描述符得到：
```
    $ ./hid-decode /sys/bus/hid/devices/0003\:093A\:2510.0002/report_descriptor
    # device 0:0
    # 0x05, 0x01,		     // Usage Page (Generic Desktop)	    0
    # 0x09, 0x02,		     // Usage (Mouse)			    2
    # 0xa1, 0x01,		     // Collection (Application)	    4
    # 0x09, 0x01,		     // Usage (Pointer)		    	    6
    # 0xa1, 0x00,		     // Collection (Physical)  	    	    8
    # 0x05, 0x09, 		     //	Usage Page (Button)		   10
```
```
    # 0x19, 0x01, 		     //	Usage Minimum (1)		   12
    # 0x29, 0x03, 		     //	Usage Maximum (3)		   14
```
```
    # 0x15, 0x00, 		     //	Logical Minimum (0)		   16
    # 0x25, 0x01, 		     //	Logical Maximum (1)		   18
```
每个按钮可以发送从 0 到包括 1 的值
```
    # 0x75, 0x01, 		     //	Report Size (1) 		   20
```
```
    # 0x95, 0x03, 		     //	Report Count (3)		   22
```
```
    # 0x81, 0x02, 		     //	Input (Data,Var,Abs)		   24
```
它是实际的数据（Data，非常量填充），表示单个变量（Var），其值为绝对（Absolute，而非
相对）；
```
    # 0x75, 0x05, 		     //	Report Size (5) 		   26
```
```
    # 0x95, 0x01, 		     //	Report Count (1)		   28
```
```
    # 0x81, 0x01, 		     //	Input (Cnst,Arr,Abs)		   30
```
```
    # 0x05, 0x01,		     // Usage Page (Generic Desktop)       32
    # 0x09, 0x30,		     // Usage (X)			   34
    # 0x09, 0x31,		     // Usage (Y)			   36
    # 0x09, 0x38,		     // Usage (Wheel) 		    	   38
```
该鼠标还有两个物理位置（Usage (X)、Usage (Y)）
```
    # 0x15, 0x81, 		     //	Logical Minimum (-127)  	   40
    # 0x25, 0x7f, 		     //	Logical Maximum (127)		   42
```
```
    # 0x75, 0x08, 		     //	Report Size (8) 		   44
```
```
    # 0x95, 0x03, 		     //	Report Count (3)		   46
```
```
    # 0x81, 0x06,		     // Input (Data,Var,Rel)  	    	   48
```
这次数据值是相对的（Relative，Rel），即它们表示
```
    # 0xc0,			     // End Collection 		    	   50
    # 0xc0,			     // End Collection  		   51
    #
    R: 52 05 01 09 02 a1 01 09 01 a1 00 05 09 19 01 29 03 15 00 25 01 75 01 95 03 81 02 75 05 95 01 81 01 05 01 09 30 09 31 09 38 15 81 25 7f 75 08 95 03 81 06 c0 c0
    N: device 0:0
    I: 3 0001 0001
```
这个报告描述符告诉我们，鼠标输入将使用四个字节传输：第一个字节用于按钮（使用三位，五位
用于填充），最后三个字节分别用于鼠标的 X、Y 和滚轮变化。

实际上，对于任何事件，鼠标都会发送一个四字节的 **report**。我们可以通过例如借助来自
`hid-tools <https://gitlab.freedesktop.org/libevdev/hid-tools>`_ 的 `hid-recorder` 工具
来检查发送的值：
```
  $ sudo ./hid-recorder /dev/hidraw1

  ....
  output of hid-decode
  ....

  #  Button: 1  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000000.000000 4 01 00 00 00
  #  Button: 0  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000000.183949 4 00 00 00 00
  #  Button: 0  1  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000001.959698 4 02 00 00 00
  #  Button: 0  0  0 | # | X:	 0 | Y:    0 | Wheel:	 0
  E: 000002.103899 4 00 00 00 00
  #  Button: 0  0  1 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000004.855799 4 04 00 00 00
  #  Button: 0  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000005.103864 4 00 00 00 00
```
这个例子表明，当点击按钮 2 时，会发送字节 `02 00 00 00`，而紧随其后的事件
（`00 00 00 00`）是按钮 2 的释放（没有按钮被按下，请记住数据值是 **绝对**（absolute）
的）。

如果改为先点击并按住按钮 1，然后点击并按住按钮
```
  #  Button: 1  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000044.175830 4 01 00 00 00
  #  Button: 1  1  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000045.975997 4 03 00 00 00
  #  Button: 0  1  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000047.407930 4 02 00 00 00
  #  Button: 0  0  0 | # | X:    0 | Y:    0 | Wheel:    0
  E: 000049.199919 4 00 00 00 00
```
其中使用 `03 00 00 00` 表示两个按钮都被按下，而随后的 `02 00 00 00` 表示按钮 1 被释放
而按钮 2 仍处于激活状态。

### Output、Input 与 Feature 报告


HID 设备可以具有 Input 报告（如鼠标示例）、Output 报告和 Feature 报告。“Output” 意味着
信息被发往设备。例如，带有力反馈（force feedback）的操纵杆会有某些输出；键盘的 LED 也
需要输出。“Input” 意味着数据来自设备。

“Feature” 并非供最终用户消费，而是定义设备的配置选项。它们可以从主机查询；当声明为
**Volatile**（易变）时，它们应由主机更改。


## 集合（Collections）、报告 ID 与 Evdev 事件


单个设备可以在逻辑上将数据分组到不同的独立集合中，称为 **Collection**（集合）。集合可以
嵌套，并且存在不同类型的集合（详见 HID 规范 6.2.2.6 “Collection, End Collection
Items”）。

不同的报告通过不同的 **Report ID**（报告 ID）字段来标识，即一个用于标识紧随其后的报告
结构编号。每当需要 Report ID 时，它都作为任何报告的第一个字节传输。一个只支持单个 HID
报告的设备（如上面的鼠标示例）可以省略报告 ID。

```
  05 01 09 02 A1 01 85 01 05 09 19 01 29 05 15 00
  25 01 95 05 75 01 81 02 95 01 75 03 81 01 05 01
  09 30 09 31 16 00 F8 26 FF 07 75 0C 95 02 81 06
  09 38 15 80 25 7F 75 08 95 01 81 06 05 0C 0A 38
  02 15 80 25 7F 75 08 95 01 81 06 C0 05 01 09 02
  A1 01 85 02 05 09 19 01 29 05 15 00 25 01 95 05
  75 01 81 02 95 01 75 03 81 01 05 01 09 30 09 31
  16 00 F8 26 FF 07 75 0C 95 02 81 06 09 38 15 80
  25 7F 75 08 95 01 81 06 05 0C 0A 38 02 15 80 25
  7F 75 08 95 01 81 06 C0 05 01 09 07 A1 01 85 05
  05 07 15 00 25 01 09 29 09 3E 09 4B 09 4E 09 E3
  09 E8 09 E8 09 E8 75 01 95 08 81 02 95 00 81 01
  C0 05 0C 09 01 A1 01 85 06 15 00 25 01 75 01 95
  01 09 3F 81 06 09 3F 81 06 09 3F 81 06 09 3F 81
  06 09 3F 81 06 09 3F 81 06 09 3F 81 06 09 3F 81
  06 C0 05 0C 09 01 A1 01 85 03 09 05 15 00 26 FF
  00 75 08 95 02 B1 02 C0
```
在解析它之后（试着用建议的工具自己解析！）可以看到，该设备呈现了两个 `Mouse` 应用集合
（分别由报告 ID 1 和 2 标识）、一个 `Keypad` 应用集合（其报告由报告 ID 5 标识）以及两个
`Consumer Controls` 应用集合（分别由报告 ID 6 和 3 标识）。但请注意，一个设备可以针对
同一个应用集合使用不同的报告 ID。

发送的数据将以报告 ID 字节开头，随后是相应的信息。例如，为以下部分传输的数据：
```
  0x05, 0x0C,        // Usage Page (Consumer)
  0x09, 0x01,        // Usage (Consumer Control)
  0xA1, 0x01,        // Collection (Application)
  0x85, 0x03,        //   Report ID (3)
  0x09, 0x05,        //   Usage (Headphone)
  0x15, 0x00,        //   Logical Minimum (0)
  0x26, 0xFF, 0x00,  //   Logical Maximum (255)
  0x75, 0x08,        //   Report Size (8)
  0x95, 0x02,        //   Report Count (2)
  0xB1, 0x02,        //   Feature (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
  0xC0,              // End Collection
```
将是三个字节：第一个字节是报告 ID（3），接下来的两个字节用于耳机，各为两个
（`Report Count (2)`）字节（`Report Size (8)`），每个字节的范围从 0（`Logical Minimum
(0)`）到 255（`Logical Maximum (255)`）。

设备发送的所有 Input 数据都应被转换为相应的 Evdev 事件，以便协议栈的其余部分能够知道
发生了什么，例如第一个按钮的位转换为 `EV_KEY/BTN_LEFT` evdev 事件，相对的 X 移动转换为
`EV_REL/REL_X` evdev 事件。

## 事件


在 Linux 中，会为每个 ``Application Collection`` 创建一个 `/dev/input/event*`。回到鼠标的
例子，并重复先点击并按住按钮 1、然后点击并按住的序列：
```
  $ sudo libinput record /dev/input/event1
  # libinput record
  version: 1
  ndevices: 1
  libinput:
    version: "1.23.0"
    git: "unknown"
  system:
    os: "opensuse-tumbleweed:20230619"
    kernel: "6.3.7-1-default"
    dmi: "dmi:bvnHP:bvrU77Ver.01.05.00:bd03/24/2022:br5.0:efr20.29:svnHP:pnHPEliteBook64514inchG9NotebookPC:pvr:rvnHP:rn89D2:rvrKBCVersion14.1D.00:cvnHP:ct10:cvr:sku5Y3J1EA#ABZ:"
  devices:
  - node: /dev/input/event1
    evdev:
      # Name: PixArt HP USB Optical Mouse
      # ID: bus 0x3 vendor 0x3f0 product 0x94a version 0x111
      # Supported Events:
      # Event type 0 (EV_SYN)
      # Event type 1 (EV_KEY)
      #   Event code 272 (BTN_LEFT)
      #   Event code 273 (BTN_RIGHT)
      #   Event code 274 (BTN_MIDDLE)
      # Event type 2 (EV_REL)
      #   Event code 0 (REL_X)
      #   Event code 1 (REL_Y)
      #   Event code 8 (REL_WHEEL)
      #   Event code 11 (REL_WHEEL_HI_RES)
      # Event type 4 (EV_MSC)
      #   Event code 4 (MSC_SCAN)
      # Properties:
      name: "PixArt HP USB Optical Mouse"
      id: [3, 1008, 2378, 273]
      codes:
  	0: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] # EV_SYN
  	1: [272, 273, 274] # EV_KEY
  	2: [0, 1, 8, 11] # EV_REL
  	4: [4] # EV_MSC
      properties: []
    hid: [
      0x05, 0x01, 0x09, 0x02, 0xa1, 0x01, 0x09, 0x01, 0xa1, 0x00, 0x05, 0x09, 0x19, 0x01, 0x29, 0x03,
      0x15, 0x00, 0x25, 0x01, 0x95, 0x08, 0x75, 0x01, 0x81, 0x02, 0x05, 0x01, 0x09, 0x30, 0x09, 0x31,
      0x09, 0x38, 0x15, 0x81, 0x25, 0x7f, 0x75, 0x08, 0x95, 0x03, 0x81, 0x06, 0xc0, 0xc0
    ]
    udev:
      properties:
      - ID_INPUT=1
      - ID_INPUT_MOUSE=1
      - LIBINPUT_DEVICE_GROUP=3/3f0/94a:usb-0000:05:00.3-2
    quirks:
    events:
    # Current time is 12:31:56
    - evdev:
      - [  0,	   0,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  0,	   0,	1, 272,       1] # EV_KEY / BTN_LEFT		      1
      - [  0,	   0,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +0ms
    - evdev:
      - [  1, 207892,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  1, 207892,	1, 273,       1] # EV_KEY / BTN_RIGHT		      1
      - [  1, 207892,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +1207ms
    - evdev:
      - [  2, 367823,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  2, 367823,	1, 272,       0] # EV_KEY / BTN_LEFT		      0
      - [  2, 367823,	0,   0,       0] # ------------ SYN_REPORT (0) ---------- +1160ms
    # Current time is 12:32:00
    - evdev:
      - [  3, 247617,	4,   4,      30] # EV_MSC / MSC_SCAN		     30 (obfuscated)
      - [  3, 247617,	1, 273,       0] # EV_KEY / BTN_RIGHT		      0
      - [  3, 247617,   0,   0,       0] # ------------ SYN_REPORT (0) ---------- +880ms
```
注意：如果你的系统上没有 `libinput record`，请尝试使用 `evemu-record`。

## 当某些功能不工作时


设备行为不正确的原因可能有很多。例如：

- 设备提供的 HID 报告描述符可能是错误的，例如

  - 它不遵循标准，因此内核将无法理解该 HID 报告描述符；
  - HID 报告描述符 **与实际** 设备发送的内容不匹配（这可以通过读取原始 HID 数据来验证）；
- HID 报告描述符可能需要一些 “quirks”（怪癖，见后文）。

因此，可能不会为每个应用集合创建 `/dev/input/event*`，并且/或者其中的事件可能不符合
你的预期。


### Quirks（怪癖）


内核知道如何修复的 HID 设备有一些已知的特性——这些被称为 HID quirks，其列表可在
`include/linux/hid.h` 中找到。

如果是这种情况，对于手头的 HID 设备，只需在内核中添加所需的 quirk 即可。这可以在
`drivers/hid/hid-quirks.c` 文件中完成。在查看该文件后，如何做应该相对直观。

当前定义的 quirks 列表（来自 `include/linux/hid.h`）是

   :doc: HID quirks

USB 设备的 quirks 可以在加载 usbhid 模块时指定，参见 `modinfo usbhid`，但正确的修复
应当进入 hid-quirks.c 并 **提交到上游（be submitted upstream）**。关于如何提交补丁的指南，
请参见 Documentation/process/submitting-patches.rst。其它总线的 quirks 需要进入
hid-quirks.c。

### 修补 HID 报告描述符


如果你需要修补 HID 报告描述符，最简单的方法是求助于 eBPF，如 Documentation/hid/hid-bpf.rst
中所述。

基本上，你可以更改原始 HID 报告描述符的任何字节。samples/hid 中的示例应该是一个很好的
起点：
```
  SEC("fmod_ret/hid_bpf_rdesc_fixup")
  int BPF_PROG(hid_rdesc_fixup, struct hid_bpf_ctx *hctx)
  {
    ....
       data[39] = 0x31;
       data[41] = 0x30;
    return 0;
  }
```
当然，这也可以在内核源码中完成，例如参考 `drivers/hid/hid-aureal.c` 或
`drivers/hid/hid-samsung.c` 以获得稍微复杂一些的文件。

如果你在查阅 HID 手册和理解 HID 报告描述符十六进制数字的确切含义方面需要任何帮助，请
查阅 Documentation/hid/hidreport-parsing.rst。

无论你想出什么解决方案，请记住 **将修复提交给 HID 维护者**，以便它能直接整合进内核，使
那个特定的 HID 设备能对所有其他人正常工作。关于如何做到这一点的指南，请参见
Documentation/process/submitting-patches.rst。


### 动态修改传输的数据


使用 eBPF 还可以修改与设备交换的数据。再次参见 samples/hid 中的示例。

同样地，**请发布你的修复**，以便它能整合进内核！

### 编写专门的驱动


这真的应该是你的最后手段。

例如可参考 `samples/hidraw/hid-example.c` 文件。
```
    $ sudo ./hid-example
    Report Descriptor Size: 52
    Report Descriptor:
    5 1 9 2 a1 1 9 1 a1 0 5 9 19 1 29 3 15 0 25 1 75 1 95 3 81 2 75 5 95 1 81 1 5 1 9 30 9 31 9 38 15 81 25 7f 75 8 95 3 81 6 c0 c0

    Raw Name: PixArt USB Optical Mouse
    Raw Phys: usb-0000:05:00.4-2.3/input0
    Raw Info:
            bustype: 3 (USB)
            vendor: 0x093a
            product: 0x2510
    ...
```
