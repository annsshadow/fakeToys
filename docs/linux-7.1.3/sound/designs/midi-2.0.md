
## Linux 上的 MIDI 2.0


## 概述


MIDI 2.0 是一种扩展协议，用于提供比传统 MIDI 1.0 更高的分辨率以及更精细的控制。为支撑 MIDI 2.0 而引入的根本性变化包括：

- 支持通用 MIDI 数据包（Universal MIDI Packet，简称 UMP）
- 支持 MIDI 2.0 协议消息
- UMP 与传统 MIDI 1.0 字节流之间的透明转换
- 用于属性与配置文件配置的 MIDI-CI

UMP 是一种新的容器格式，用于承载所有 MIDI 协议 1.0 与 MIDI 2.0 协议消息。与以往的字节流不同，它是 32 位对齐的，并且每条消息都可以放入单个数据包中。UMP 最多可以发送 16 个“UMP 组（UMP Group）”的事件，其中每个 UMP 组最多包含 16 个 MIDI 通道。

MIDI 2.0 协议是一种扩展协议，用于实现比旧的 MIDI 1.0 协议更高的分辨率与更多的控制。

MIDI-CI 是一种高层协议，可以与 MIDI 设备进行灵活配置文件与配置的协商。它以特殊 SysEx 的形式表示。

对于 Linux 实现，内核支持 UMP 传输以及在 UMP 上对 MIDI 协议进行编解码，而 MIDI-CI 则在用户空间通过标准 SysEx 获得支持。

截至本文撰写时，只有 USB MIDI 设备原生支持 UMP 与 Linux 2.0。UMP 支持本身是相当通用的，因此它也可以被其他传输层使用，尽管它也可能以不同的方式实现（例如作为 ALSA 音序器客户端）。

对 UMP 设备的访问以两种方式提供：通过 rawmidi 设备的访问，以及通过 ALSA 音序器 API 的访问。

ALSA 音序器 API 已被扩展以允许 UMP 数据包的负载。允许在 MIDI 1.0 与 MIDI 2.0 音序器客户端之间自由连接，并且事件会被透明地转换。


## 内核配置


为支持 MIDI 2.0，新增了以下配置项：
`CONFIG_SND_UMP`、`CONFIG_SND_UMP_LEGACY_RAWMIDI`、
`CONFIG_SND_SEQ_UMP`、`CONFIG_SND_SEQ_UMP_CLIENT`，以及
`CONFIG_SND_USB_AUDIO_MIDI_V2`。第一个可见的是
`CONFIG_SND_USB_AUDIO_MIDI_V2`，当你选择它（设置为 `=y`）时，
UMP 的核心支持（`CONFIG_SND_UMP`）与音序器绑定
（`CONFIG_SND_SEQ_UMP_CLIENT`）会被自动选中。

此外，`CONFIG_SND_UMP_LEGACY_RAWMIDI=y` 将为 UMP 端点启用对
传统 raw MIDI 设备的支持。


## 使用 USB MIDI 2.0 的 Rawmidi 设备


当设备支持 MIDI 2.0 时，USB 音频驱动会探测并使用 MIDI 2.0 接口
（始终位于 altset 1）作为默认接口，而非 MIDI 1.0 接口（位于 altset 0）。
你也可以通过将 `midi2_enable=0` 选项传递给 snd-usb-audio 驱动模块，
切换回使用旧的 MIDI 1.0 接口的绑定。

USB 音频驱动会尝试查询自 UMP v1.1 起提供的 UMP Endpoint 与 UMP Function
Block 信息，并基于这些信息构建拓扑。当设备较旧、对新 UMP 查询无响应时，
驱动会回退并基于来自 USB 描述符的 Group Terminal Block（GTB）信息构建拓扑。
某些设备可能会被意外的 UMP 命令搞乱；在这种情况下，向 snd-usb-audio 驱动
传递 `midi2_ump_probe=0` 选项以跳过 UMP v1.1 查询。

当探测到 MIDI 2.0 设备时，内核会为该设备的每个 UMP Endpoint 创建一个
rawmidi 设备。其设备名为 `/dev/snd/umpC**D**`，不同于标准 rawmidi 设备名
`/dev/snd/midiC**D**`（对应 MIDI 1.0），以避免传统应用程序误访问 UMP 设备。

你可以直接对该 UMP rawmidi 设备进行 UMP 数据包的读取与写入。例如，像下面这样
通过 `hexdump` 读取，将以十六进制形式显示卡 0 设备 0 的传入 UMP 数据包

```
  % hexdump -C /dev/snd/umpC0D0
  00000000  01 07 b0 20 00 07 b0 20  64 3c 90 20 64 3c 80 20  |... ... d<. d<. |
```

与 MIDI 1.0 字节流不同，UMP 是一个 32 位数据包，并且读取或写入设备时的
大小也按 32 位（即 4 字节）对齐。

UMP 数据包负载中的 32 位字始终采用 CPU 本机字节序。传输驱动负责将 UMP 字
从/向系统字节序转换为所需的传输字节序/字节顺序。

当设置了 `CONFIG_SND_UMP_LEGACY_RAWMIDI` 时，驱动会额外创建一个标准 raw MIDI
设备 `/dev/snd/midiC**D**`。它包含 16 个子流，每个子流对应一个（从 0 开始计数的）
UMP 组。传统应用程序可以通过每个子流以 MIDI 1.0 字节流格式访问指定的组。
使用 ALSA rawmidi API 时，你可以打开任意子流，而仅打开 `/dev/snd/midiC**D**`
最终会打开第一个子流。

每个 UMP Endpoint 都可以提供附加信息，这些信息由通过 UMP 1.1 Stream 消息或
USB MIDI 2.0 描述符查询得到的信息构建而成。一个 UMP Endpoint 可以包含一个或多个
UMP Block，其中 UMP Block 是 ALSA UMP 实现中引入的一种抽象，用于表示 UMP 组之间的
关联。UMP Block 对应于 UMP 1.1 规范中的 Function Block。当 UMP 1.1 Function Block
信息不可用时，会部分地从 USB MIDI 2.0 规范中定义的 Group Terminal Block（GTB）
填充。

UMP Endpoint 与 UMP Block 的信息可以在 proc 文件中找到

```
  % cat /proc/asound/card1/midi0
  ProtoZOA MIDI

  Type: UMP
  EP Name: ProtoZOA
  EP Product ID: ABCD12345678
  UMP Version: 0x0000
  Protocol Caps: 0x00000100
  Protocol: 0x00000100
  Num Blocks: 3

  Block 0 (ProtoZOA Main)
    Direction: bidirection
    Active: Yes
    Groups: 1-1
    Is MIDI1: No

  Block 1 (ProtoZOA Ext IN)
    Direction: output
    Active: Yes
    Groups: 2-2
    Is MIDI1: Yes (Low Speed)
  ....

```

注意，上面 proc 文件中显示的 `Groups` 字段表示的是从 1 开始计数的 UMP 组编号
（从-到）。

这些附加的 UMP Endpoint 与 UMP Block 信息可以分别通过新的 ioctl
`SNDRV_UMP_IOCTL_ENDPOINT_INFO` 与 `SNDRV_UMP_IOCTL_BLOCK_INFO` 获取。

rawmidi 名称与 UMP Endpoint 名称通常相同，对于 USB MIDI，它取自相应 USB MIDI
接口描述符的 `iInterface`。如果未提供，则作为回退从 USB 设备描述符的 `iProduct`
复制。

Endpoint Product ID 是一个字符串字段，应当是唯一的。对于 USB MIDI，它从设备的
`iSerialNumber` 复制而来。

协议能力与实际协议位定义在 `asound.h` 中。


## 使用 USB MIDI 2.0 的 ALSA 音序器


除了 rawmidi 接口之外，ALSA 音序器接口也支持新的 UMP MIDI 2.0 设备。
现在，每个 ALSA 音序器客户端都可以设置其 MIDI 版本（0、1 或 2），以分别声明
自身为传统设备、UMP MIDI 1.0 设备或 UMP MIDI 2.0 设备。第一个即传统客户端，
按原样发送/接收旧式音序器事件。而 UMP MIDI 1.0 与 2.0 客户端则以用于 UMP 的
扩展事件记录发送和接收。MIDI 版本可以在 `snd_seq_client_info` 的新字段
`midi_version` 中看到。

通过在音序器事件中指定新的事件标志位 `SNDRV_SEQ_EVENT_UMP`，可以以嵌入方式
发送/接收 UMP 数据包。当设置此标志时，事件拥有 16 字节（128 位）的数据负载来
存放 UMP 数据包。如果不带 `SNDRV_SEQ_EVENT_UMP` 标志位，事件将像以前一样被视为
传统事件（最大 12 字节数据负载）。

设置 `SNDRV_SEQ_EVENT_UMP` 标志时，UMP 音序器事件的 type 字段会被忽略（但默认
应设为 0）。

每个客户端的类型可以在 `/proc/asound/seq/clients` 中看到。

```
  % cat /proc/asound/seq/clients
  Client info
    cur  clients : 3
  ....
  Client  14 : "Midi Through" [Kernel Legacy]
    Port   0 : "Midi Through Port-0" (RWe-)
  Client  20 : "ProtoZOA" [Kernel UMP MIDI1]
    UMP Endpoint: ProtoZOA
    UMP Block 0: ProtoZOA Main [Active]
      Groups: 1-1
    UMP Block 1: ProtoZOA Ext IN [Active]
      Groups: 2-2
    UMP Block 2: ProtoZOA Ext OUT [Active]
      Groups: 3-3
    Port   0 : "MIDI 2.0" (RWeX) [In/Out]
    Port   1 : "ProtoZOA Main" (RWeX) [In/Out]
    Port   2 : "ProtoZOA Ext IN" (-We-) [Out]
    Port   3 : "ProtoZOA Ext OUT" (R-e-) [In]

```

在这里你可以找到两类内核客户端，客户端 14 为 “Legacy”，客户端 20 为 “UMP MIDI1”，
它就是一个 USB MIDI 2.0 设备。
USB MIDI 2.0 客户端始终将端口 0 作为 “MIDI 2.0” 提供，其余端口从 1 开始对应每个
UMP 组（例如端口 1 对应组 1）。
在此示例中，设备有三个活动组（Main、Ext IN 与 Ext OUT），它们作为音序器端口从 1 到 3
暴露出来。
“MIDI 2.0” 端口用于 UMP Endpoint，它与其他 UMP 组端口的区别在于：UMP Endpoint 端口
发送来自设备上所有端口的事件（“捕获全部”，catch-all），而每个 UMP 组端口只发送
来自给定 UMP 组的事件。
此外，无组的 UMP 消息（例如 UMP 消息类型 0x0f）只会发送到 UMP Endpoint 端口。

注意，虽然每个 UMP 音序器客户端通常会创建 16 个端口，但那些不属于任何 UMP Block
（或属于非活动 UMP Block）的端口会被标记为不活动，并且不会出现在 proc 输出中。
在上面的示例中，从 4 到 16 的音序器端口是存在的，但没有显示在那里。

上面的 proc 文件也显示了 UMP Block 信息。同样的条目（但带有更详细的信息）可以在
rawmidi 的 proc 输出中找到。

当客户端在不同 MIDI 版本之间连接时，事件会根据客户端的版本自动转换，不仅是在
传统类型与 UMP MIDI 1.0/2.0 类型之间，也在 UMP MIDI 1.0 与 2.0 类型之间。例如，
在 ProtoZOA Main 端口上以传统模式运行 `aseqdump` 程序将

```
  % aseqdump -p 20:1
  Waiting for data. Press Ctrl+C to end.
  Source  Event                  Ch  Data
   20:1   Note on                 0, note 60, velocity 100
   20:1   Note off                0, note 60, velocity 100
   20:1   Control change          0, controller 11, value 4
```

当你以 MIDI 2.0 模式运行 `aseqdump` 时，它将接收到高

```
  % aseqdump -u 2 -p 20:1
  Waiting for data. Press Ctrl+C to end.
  Source  Event                  Ch  Data
   20:1   Note on                 0, note 60, velocity 0xc924, attr type = 0, data = 0x0
   20:1   Note off                0, note 60, velocity 0xc924, attr type = 0, data = 0x0
   20:1   Control change          0, controller 11, value 0x2000000
```

而数据由 ALSA 音序器核心自动转换。


## Rawmidi API 扩展


- 可以通过新的 ioctl `SNDRV_UMP_IOCTL_ENDPOINT_INFO` 获取附加的 UMP Endpoint
  信息。它包含关联的音卡与设备编号、位标志、协议、UMP Block 数量、端点的名称
  字符串等。

  协议由 protocol capabilities（协议能力）与 current protocol（当前协议）两个
  字段指定。二者都包含位标志，在上字节中指定 MIDI 协议版本
  （`SNDRV_UMP_EP_INFO_PROTO_MIDI1` 或 `SNDRV_UMP_EP_INFO_PROTO_MIDI2`），
  在下字节中指定抖动消除时间戳（`SNDRV_UMP_EP_INFO_PROTO_JRTS_TX` 与
  `SNDRV_UMP_EP_INFO_PROTO_JRTS_RX`）。

  一个 UMP Endpoint 最多可包含 32 个 UMP Block，当前已分配块的数量显示在 Endpoint
  信息中。

- 每个 UMP Block 的信息可以通过另一个新的 ioctl `SNDRV_UMP_IOCTL_BLOCK_INFO`
  获取。必须传入要查询的块的块 ID 号（从 0 开始）。接收到的数据包含该块的关联
  方向、第一个关联组 ID（从 0 开始）与组数量、块的名称字符串等。

  方向为 `SNDRV_UMP_DIR_INPUT`、`SNDRV_UMP_DIR_OUTPUT` 或
  `SNDRV_UMP_DIR_BIDIRECTION` 之一。

- 对于支持 UMP v1.1 的设备，可以通过 “Stream Configuration Request” 消息
  （UMP 类型 0x0f，状态码 0x05）切换 UMP MIDI 协议。当 UMP 核心收到这样的消息时，
  它会相应地更新 UMP EP 信息以及相应的音序器客户端。

- 传统 rawmidi 设备编号可以在 rawmidi 信息的新字段 `tied_device` 中找到。
  另一方面，UMP rawmidi 设备编号也可以在传统 rawmidi 信息的 `tied_device` 字段
  中找到。

- 传统 rawmidi 的每个子流可以根据 UMP FB 状态动态启用/禁用。
  当所选子流不活动时，会通过传统 rawmidi 信息 `flags` 字段中的位 0x10
  （`SNDRV_RAWMIDI_INFO_STREAM_INACTIVE`）来指示。


## Control API 扩展


- 引入了新的 ioctl `SNDRV_CTL_IOCTL_UMP_NEXT_DEVICE` 用于查询下一个 UMP rawmidi
  设备，而现有的 ioctl `SNDRV_CTL_IOCTL_RAWMIDI_NEXT_DEVICE` 只查询传统 rawmidi
  设备。

  要设置要打开的子设备（子流编号），请像普通 rawmidi 一样使用 ioctl
  `SNDRV_CTL_IOCTL_RAWMIDI_PREFER_SUBDEVICE`。

- 两个新的 ioctl `SNDRV_CTL_IOCTL_UMP_ENDPOINT_INFO` 与
  `SNDRV_CTL_IOCTL_UMP_BLOCK_INFO` 通过 ALSA control API 提供指定 UMP 设备的
  UMP Endpoint 与 UMP Block 信息，而无需打开实际的（UMP）rawmidi 设备。
  查询时忽略 `card` 字段，始终与 control 接口所在的音卡绑定。


## Sequencer API 扩展


- 向 `snd_seq_client_info` 添加了 `midi_version` 字段，用于指示每个客户端的
  当前 MIDI 版本（0、1 或 2）。当 `midi_version` 为 1 或 2 时，从 UMP 音序器
  客户端读取的对齐方式也从原来的 28 字节改为 32 字节，以适应扩展负载。写入的
  对齐大小未改变，但每个事件的大小可能因下面的新位标志而不同。

- 为每个音序器事件标志添加了 `SNDRV_SEQ_EVENT_UMP` 标志位。当设置该位标志时，
  音序器事件被扩展为拥有更大的 16 字节负载（取代传统的 12 字节），并且事件在负载中
  包含 UMP 数据包。

- 新的音序器端口类型位（`SNDRV_SEQ_PORT_TYPE_MIDI_UMP`）表示该端口支持 UMP。

- 音序器端口拥有新的能力位以指示不活动端口（`SNDRV_SEQ_PORT_CAP_INACTIVE`）与
  UMP Endpoint 端口（`SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT`）。

- 可以通过设置到客户端信息的新的过滤位 `SNDRV_SEQ_FILTER_NO_CONVERT` 来抑制
  ALSA 音序器客户端的事件转换。例如，内核透传客户端（`snd-seq-dummy`）会在内部
  设置此标志。

- 端口信息获得了新字段 `direction`，用于指示端口的方向（为
  `SNDRV_SEQ_PORT_DIR_INPUT`、`SNDRV_SEQ_PORT_DIR_OUTPUT` 或
  `SNDRV_SEQ_PORT_DIR_BIDIRECTION` 之一）。

- 端口信息的另一个附加字段是 `ump_group`，它指定关联的 UMP 组编号（从 1 开始）。
  当它非零时，UMP 数据包中的 UMP 组字段会在投递到指定组时更新（修正为从 0 开始）。
  每个音序器端口如果是一个特定于某个 UMP 组的端口，应当设置此字段。

- 每个客户端可以在 `group_filter` 位图中为 UMP 组设置附加的事件过滤器。该过滤器
  由从 1 开始计数的组编号组成的位图。例如，当设置位 1 时，来自组 1（即第一个组）
  的消息会被过滤而不被投递。位 0 用于过滤无组的 UMP 消息。

- 为支持 UMP 的客户端新增了两个 ioctl：
  `SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO` 与
  `SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO`。它们用于获取和设置与音序器客户端关联的
  `snd_ump_endpoint_info` 或 `snd_ump_block_info` 数据。USB MIDI 驱动从底层的 UMP
  rawmidi 提供这些信息，而用户空间客户端可以通过 `*_SET` ioctl 提供其自身的数据。
  对于 Endpoint 数据，向 `type` 字段传入 0；对于 Block 数据，向 `type` 字段传入
  块号 + 1。
  为内核客户端设置数据将导致错误。

- 在 UMP 1.1 下，Function Block 信息可能会动态改变。当从设备收到 Function Block
  的更新时，ALSA 音序器核心会相应地更改相应的音序器端口名称与属性，并像普通的端口
  变更通知一样，通过向 ALSA 音序器系统端口的公告来通知这些变更。

- 有两个扩展事件类型用于通过系统公告端口通知 UMP Endpoint 与 Function Block 的
  变更：类型 68（`SNDRV_SEQ_EVENT_UMP_EP_CHANGE`）与类型 69
  （`SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE`）。它们在负载中采用新类型
  `snd_seq_ev_ump_notify`，指示发生变更的客户端编号与 FB 编号。


## MIDI2 USB 复合设备功能驱动


最新的内核包含对 USB MIDI 2.0 复合设备功能驱动的支持，它可用于 MIDI 2.0 特性的
原型设计与调试。

需要启用 `CONFIG_USB_GADGET`、`CONFIG_USB_CONFIGFS` 与
`CONFIG_USB_CONFIGFS_F_MIDI2` 才能使用该 MIDI2 复合设备驱动。

此外，要使用复合设备驱动，你需要一个可用的 UDC 驱动。在下面的示例中，我们使用
`dummy_hcd` 驱动（通过 `CONFIG_USB_DUMMY_HCD` 启用），它在 PC 与 VM 上可用于
调试目的。根据平台不同还有其他 UDC 驱动，它们也可以用于真实设备。

```
  % modprobe libcomposite
```

然后你会在 configfs 空间下拥有 `usb_gadget` 子目录（在现代操作系统上通常为
`/sys/kernel/config`）。接着创建一个复合设备

```
  % cd /sys/kernel/config
  % mkdir usb_gadget/g1

  % cd usb_gadget/g1
  % mkdir configs/c.1
  % mkdir functions/midi2.usb0

  % echo 0x0004 > idProduct
  % echo 0x17b3 > idVendor
  % mkdir strings/0x409
  % echo "ACME Enterprises" > strings/0x409/manufacturer
  % echo "ACMESynth" > strings/0x409/product
  % echo "ABCD12345" > strings/0x409/serialnumber

  % mkdir configs/c.1/strings/0x409
  % echo "Monosynth" > configs/c.1/strings/0x409/configuration
  % echo 120 > configs/c.1/MaxPower
```

此时必须存在一个子目录 `ep.0`，它就是一个 UMP Endpoint 的配置。你可以填写该
Endpoint

```
  % echo "ACMESynth" > functions/midi2.usb0/iface_name
  % echo "ACMESynth" > functions/midi2.usb0/ep.0/ep_name
  % echo "ABCD12345" > functions/midi2.usb0/ep.0/product_id
  % echo 0x0123 > functions/midi2.usb0/ep.0/family
  % echo 0x4567 > functions/midi2.usb0/ep.0/model
  % echo 0x123456 > functions/midi2.usb0/ep.0/manufacturer
  % echo 0x12345678 > functions/midi2.usb0/ep.0/sw_revision
```

```
  % echo 2 > functions/midi2.usb0/ep.0/protocol
```

并且，你可以在此 Endpoint 下找到一个子目录 `block.0`

```
  % echo "Monosynth" > functions/midi2.usb0/ep.0/block.0/name
  % echo 0 > functions/midi2.usb0/ep.0/block.0/first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.0/num_groups
```

```
  % ln -s functions/midi2.usb0 configs/c.1
  % echo dummy_udc.0 > UDC
```

其中 `dummy_udc.0` 是一个示例情况，会因系统而异。你可以在 `/sys/class/udc` 中
找到 UDC 实例并传入

```
  % ls /sys/class/udc
  dummy_udc.0
```

现在，MIDI 2.0 复合设备已启用，复合设备主机会创建一个包含 UMP rawmidi 设备的新
声卡实例

```
  % cat /proc/asound/cards
  ....
  1 [Gadget         ]: f_midi2 - MIDI 2.0 Gadget
                       MIDI 2.0 Gadget
```

而在所连接的主机上，也应该会出现一张类似的卡，但带有

```
  % cat /proc/asound/cards
  ....
  2 [ACMESynth      ]: USB-Audio - ACMESynth
                       ACME Enterprises ACMESynth at usb-dummy_hcd.0-1, high speed
```

```
  % aplaymidi -p 20:1 to_host.mid
```

而这会出现在已连接主机上作为一个来自 MIDI 设备的输入

```
  % aseqdump -p 20:0 -u 2
```

反之亦然，在已连接主机上的回放也会作为复合设备上的输入工作。

每个 Function Block 可以有不同的方向与 UI 提示（UI-hint），通过 `direction` 与
`ui_hint` 属性指定。传入 `1` 表示仅输入，`2` 表示仅输出，`3` 表示

```
  % echo 2 > functions/midi2.usb0/ep.0/block.0/direction
  % echo 2 > functions/midi2.usb0/ep.0/block.0/ui_hint
```

当你需要多于一个 Function Block 时，可以动态创建子目录 `block.1`、`block.2` 等，
并在上面链接之前的配置步骤中配置它们。

```
  % mkdir functions/midi2.usb0/ep.0/block.1
  % echo "Keyboard" > functions/midi2.usb0/ep.0/block.1/name
  % echo 1 > functions/midi2.usb0/ep.0/block.1/first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.1/num_groups
  % echo 1 > functions/midi2.usb0/ep.0/block.1/direction
  % echo 1 > functions/midi2.usb0/ep.0/block.1/ui_hint
```

`block.*` 子目录也可以动态移除（除了持久存在的 `block.0`）。

要为 MIDI 1.0 I/O 分配一个 Function Block，请在 `is_midi1` 属性中设置。1 表示
MIDI 1.0，2 表示低速率的 MIDI 1.0

```
  % echo 2 > functions/midi2.usb0/ep.0/block.1/is_midi1
```

要禁用复合设备中对 UMP Stream 消息的处理

```
  % echo 0 > functions/midi2.usb0/process_ump
```

复合设备驱动也支持位于 altset 0 的 MIDI 1.0 接口。当已连接主机选择了 MIDI 1.0
接口时，复合设备上的 UMP I/O 会相应地与 USB MIDI 1.0 数据包相互转换，而复合设备
驱动仍通过 UMP rawmidi 与用户空间通信。

MIDI 1.0 端口由每个 Function Block 中的配置建立。

```
  % echo 0 > functions/midi2.usb0/ep.0/block.0/midi1_first_group
  % echo 1 > functions/midi2.usb0/ep.0/block.0/midi1_num_groups
```

上面的配置将为 MIDI 1.0 接口启用组 1（索引 0）。注意这些组必须位于为 Function
Block 本身定义的组之中。

复合设备驱动也支持多于一个 UMP Endpoint。与 Function Block 类似，你可以创建一个新的
子目录

```
  % mkdir functions/midi2.usb0/ep.1
```

并在其中创建一个新的 Function Block。例如，要创建 4 个

```
  % mkdir functions/midi2.usb0/ep.1/block.0
  % echo 4 > functions/midi2.usb0/ep.1/block.0/num_groups
```

现在，你总共会有 4 个 rawmidi 设备：前两个是 Endpoint 0 与 Endpoint 1 的 UMP
rawmidi 设备，另外两个是对应的 EP 0 与 EP 1 的传统 MIDI 1.0 rawmidi 设备。

复合设备上的当前 altsetting 可以通过一个带有 `RAWMIDI` iface 的名为 “Operation Mode”
（操作模式）的控制元素来告知。例如，你可以读取它

```
  % amixer -c1 cget iface=RAWMIDI,name='Operation Mode'
  ; type=INTEGER,access=r--v----,values=1,min=0,max=2,step=0
  : values=2
```

该值（在第二行返回内容中以 `: values=` 显示）表示：1 为 MIDI 1.0（altset 0），
2 为 MIDI 2.0（altset 1），0 为未设置。

截至目前，绑定之后无法更改配置。
