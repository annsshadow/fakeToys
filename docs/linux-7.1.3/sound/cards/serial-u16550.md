## 串口 UART 16450/16550 MIDI 驱动


adaptor 模块参数允许你选择以下之一

- 0 - Roland Soundcanvas 支持（默认）
- 1 - Midiator MS-124T 支持
- 2 - Midiator MS-124W S/A 模式
- 3 - MS-124W M/B 模式支持
- 4 - 支持多输入的通用设备

对于 Midiator MS-124W，你必须Midiator 上的物理 M-S A-B 开关设置得与你所选择的驱动模式相匹配

Roland Soundcanvas 模式下，支持多个 ALSA raw MIDI 子流（midiCnD0-midiCnD15）。每当你写入一个不同的子流时，驱动会发送非标准MIDI 命令序列 F5 NN，其NN 为子流编号加 1。Roland 模块使用此命令在不同“声部”（part）之间切换，因此该特性让你可以将每个声部当作一个独立的 raw MIDI 子流对待。驱动没有提供发F5 00（不选择）或不发F5 NN 命令序列的方式；或许应当提供

简单串口转换器的使用示例：
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 speed=115200

```
4 MIDI 端口Roland SoundCanvas 使用示例
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 outs=4

```
MS-124T 模式下，支持一raw MIDI 子流（midiCnD0）；outs 模块参数会自动设1。驱动将相同的数据发送到全部四个 MIDI Out 接口。将 A-B 开关和 speed 模块参数设置为匹配（A=19200，B=9600）

A-B 开关处A 位的 MS-124T 使用示例
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=1 \
			speed=19200

```
MS-124W S/A 模式下，支持一raw MIDI 子流（midiCnD0）；outs 模块参数会自动设1。驱动以全MIDI 速率将相同的数据发送到全部四个 MIDI Out 接口

S/A 模式使用示例
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=2

```
MS-124W M/B 模式下，驱动支持 16 ALSA raw MIDI 子流；outs 模块参数会自动设16。子流编号给出数据应发送到MIDI Out 接口的位掩码，其midiCnD1 发送到 Out 1，midiCnD2 Out 2，midiCnD4 Out 3，midiCnD8 Out 4。因midiCnD15 将数据发送到全部 4 个端口。作为一种特殊情况，midiCnD0 也会发送到所有端口，因为向无端口发送数据并无用处。M/B 模式有额外开销来为每个字节选择 MIDI Out，因此四MIDI Out 上的总数据速率最多为每个字节 520 微秒一次，而全MIDI 数据速率为每端口每字320 微秒一次

M/B 模式使用示例
```

	/sbin/setserial /dev/ttyS0 uart none
	/sbin/modprobe snd-serial-u16550 port=0x3f8 irq=4 adaptor=3

```
MS-124W 硬件M/A 模式目前不受支持。该模式允许 MIDI Out M/B 两倍的总吞吐独立工作，但不允许将同一字节同时发送到多个 MIDI Out。M/A 协议要求驱动在时序约束下拨动调制解调器控制线，因此实现起来比其他模式稍复杂

MS-124W MS-124T 之外Midiator 型号目前不受支持。请注意后缀字母是有意义的；MS-124 MS-124B 不兼容，其他已知型号 MS-101、MS-101B、MS-103 MS-114 之间也不兼容。我手头有（tim.mann@compaq.com）部分涵盖这些型号的文档，但没有可供试验的实物。MS-124W 支持已用真实设备测试过。MS-124T 支持未经测试，但应当可用

通用驱动通过单个串口支持多个输入和输出子流。与 Roland Soundcanvas 模式类似，使F5 NN 来选择适当的输入或输出流（取决于数据方向）。此外，CTS 信号用于调节数据流。输入的数量ins 参数指定
