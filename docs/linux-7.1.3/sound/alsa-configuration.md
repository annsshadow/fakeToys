## ALSA（Advanced Linux Sound Architecture）驱动配置指



## 内核配置


要启ALSA 支持，你至少需要在内核中构建主声卡支持（`CONFIG_SOUND`）。由ALSA 可以模拟 OSS，因此你无需选择任何 OSS 模块

如果你希望用 ALSA 运行 OSS 应用程序，请启用“OSS API 模拟”（`CONFIG_SND_OSSEMUL`）以OSS 混音器和 PCM 支持

如果你想支持 SB Live! 等声卡上的波表（WaveTable）功能，则需要启用“音序器支持”（`CONFIG_SND_SEQUENCER`）

若要ALSA 的调试信息更详细，请启用“Verbose printk”和“Debug”选项。若要检查内存泄漏，还要打开“Debug memory”。“Debug detection”会添加用于声卡检测的检查

请注意，所ALSA ISA 驱动都支Linux isapnp API（前提是声卡支持 ISA PnP）。你无需使用 isapnptools 来配置声卡


## 模块参数


用户可以带选项加载模块。如果某个模块支持多块声卡，而你又有多块同类型声卡，则可以用逗号分隔为选项指定多个值


### 模块 snd


ALSA 核心模块。它被所ALSA 声卡驱动所使用
它接受以下具有全局影响的选项

major
    声卡驱动major 号；
    默认值：116
cards_limit
    限制自动加载的声卡索引（1-8）；
    默认值：1
    若要自动加载多块声卡，请将此选项snd-card-X 别名一起指定
slots
    为给定驱动保留槽位索引；
    此选项接受多个字符串
    详见 `Module Autoloading Support`_ 小节
debug
    指定调试信息级别
     = 禁用调试打印 = 普通调试信息，
    2 = 详细调试信息）；
    此选项仅在 `CONFIG_SND_DEBUG=y` 时才会出现
    此选项可通过 sysfs 中的
    /sys/module/snd/parameters/debug 文件动态修改

### 模块 snd-pcm-oss


PCM OSS 模拟模块
该模块接受用于改变设备映射的选项

dsp_map
    分配给第 1 OSS 设备PCM 设备号；
    默认值：0
adsp_map
    分配给第 2 OSS 设备PCM 设备号；
    默认值：1
nonblock_open
    打开繁忙PCM 设备时不阻塞
    默认值：1

例如，当 `dsp_map=2` 时，/dev/dsp 将被映射
0 号声卡的2 PCM。类似地，当 `adsp_map=0` 时，
/dev/adsp 将被映射到第 0 号声卡的0 PCM
若要修改第二块或更后面的声卡，可用逗号指定选项
例如 `dsp_map=0,1`

`nonblock_open` 选项用于改变 PCM 在打开设备时的行为
当该选项非零时，打开一个繁忙的 OSS PCM 设备不会被阻塞，
而是立即EAGAIN 返回（就O_NONBLOCK 标志一样）

### 模块 snd-rawmidi


该模块接受用于改变设备映射的选项
snd-pcm-oss 模块类似

midi_map
    分配给第 1 OSS 设备MIDI 设备号；
    默认值：0
amidi_map
    分配给第 2 OSS 设备MIDI 设备号；
    默认值：1

### 模块 snd-soc-core


SoC 核心模块。它被所ALSA 声卡驱动所使用
它接受以下具有全局影响的选项

prealloc_buffer_size_kbytes
    kbytes 为单位指定预分配缓冲区大小（默认12）

### 顶层声卡模块的通用参数


每个顶层声卡模块都接受以下选项

index
    声卡的索引（槽位 #）；
    取值：0 31 或负数；
    若非负，则分配该索引号；
    若为负，则解释为允许索引的位掩码
    分配第一个空闲的允许索引
    默认值：-1
id
    声卡 ID（标识符或名称）
    最15 个字符；
    默认值：声卡类型
    /proc/asound/ 下会创建以此名称命名的目录，
    其中包含该声卡的相关信息
    在识别声卡时可以用此 ID 代替索引
enable
    启用声卡
    默认值：对于 PCI ISA PnP 声卡为启

这些选项用于指定实例的顺序，或在多个设备绑定到同一驱动
控制每个设备的启用与禁用。例如，许多机器有两HD-audio
控制器（一块用HDMI/DP 音频，另一块用于板载模拟音频）
在大多数情况下，第二块是主要用途，用户希望将其分配
最先出现的声卡。可以通过指定 "index=1,0" 模块参数来实现，
这会交换分配槽位

如今，在带有 PulseAudio PipeWire 等支持动态配置的声音
后端的情况下，这种用法已没什么价值，但在过去它对静态配
很有帮助

### 模块 snd-adlib


用于 AdLib FM 声卡的模块

port
    OPL 芯片的端口号 #

该模块支持多块声卡。它不支持自动探测，因此必须指定端口
对于实际AdLib FM 声卡，端口为 0x388
注意该声卡没PCM 支持和混音器，仅FM 合成

请确保你已准备好 alsa-tools 软件包中`sbiload`，并
加载模块后通过 `sbiload -l` 查明所分配ALSA 音序器端口号

示例输出
```

      Port     Client name                       Port name
      64:0     OPL2 FM synth                     OPL2 FM Port

```
加载同样`sbiload` 提供`std.sb` `drums.sb` 音色
```

      sbiload -p 64:0 std.sb drums.sb

```
如果你使用该驱动来驱OPL3，则可以改用 `std.o3` `drums.o3`
若要让声卡发出声音，可使alsa-utils 中的 `aplaymidi`
```

      aplaymidi -p 64:0 foo.mid

```
### 模块 snd-ad1816a


用于基于 Analog Devices AD1816A/AD1815 ISA 芯片的声卡的模块

clockfreq
    AD1816A 芯片的时钟频率（默认 = 03000Hz

该模块支持多块声卡、自动探测和 PnP

### 模块 snd-ad1848


用于基于 AD1848/AD1847/CS4248 ISA 芯片的声卡的模块

port
    AD1848 芯片的端口号 #
irq
    AD1848 芯片IRQ #
dma1
    AD1848 芯片DMA #,1,3

该模块支持多块声卡。它不支持自动探测，因此必须指定主端口！！！
其他端口为可选

支持电源管理

### 模块 snd-ad1889


用于 Analog Devices AD1889 芯片的模块

ac97_quirk
    针对异常硬件AC'97 规避方案
    详见 intel8x0 模块的描述

该模块支持多块声卡

### 模块 snd-ali5451


用于 ALi M5451 PCI 芯片的模块

pcm_channels
    PCM 分配的硬件通道
spdif
    支持 SPDIF I/O
    默认值：禁用

该模块支持单芯片和自动探测

支持电源管理

### 模块 snd-als100


用于基于 Avance Logic ALS100/ALS120 ISA 芯片的声卡的模块

该模块支持多块声卡、自动探测和 PnP

支持电源管理

### 模块 snd-als300


用于 Avance Logic ALS300 ALS300+ 的模块

该模块支持多块声卡

支持电源管理

### 模块 snd-als4000


用于基于 Avance Logic ALS4000 PCI 芯片的声卡的模块

joystick_port
    传统游戏杆支持的端口#
    0 = 禁用（默认） = 自动探测

该模块支持多块声卡、自动探测和 PnP

支持电源管理

### 模块 snd-asihpi


用于 AudioScience ASI 声卡的模块

enable_hpi_hwdep
    AudioScience 声卡启用 HPI hwdep

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-atiixp


用于 ATI IXP 150/200/250/400 AC97 控制器的模块

ac97_clock
    AC'97 时钟（默= 48000
ac97_quirk
    针对异常硬件AC'97 规避方案
    详见下面`AC97 Quirk Option`_ 小节
ac97_codec
    用于指定使用某个 AC'97 编解码器而非探测的规避方案
    如果这对你有效，请附上你`lspci -vn` 输出提交一bug
    2 = 强制探测1 = 默认行为-2 = 使用指定的编解码器。）
spdif_aclink
    通过 AC-link 传输 S/PDIF（默= 1

该模块支持单块声卡和自动探测

ATI IXP 有两种不同的方法来控SPDIF 输出。一种是通过
AC-link，另一种是通过“directSPDIF 输出。具体实现取决于
主板，你需要通过 spdif_aclink 模块选项选择正确的方式

支持电源管理

### 模块 snd-atiixp-modem


用于 ATI IXP 150/200/250 AC97 调制器控制器的模块

该模块支持单块声卡和自动探测

注意：该模块的默index 值为 -2，即第一个槽位被排除

支持电源管理

### 模块 snd-au8810、snd-au8820、snd-au8830


用于 Aureal Vortex、Vortex2 Advantage 设备的模块

pcifix
    控制 PCI 规避方案
    0 = 禁用所有规避方案，
    1 = Aureal 声卡PCI 延迟强制设为 0xff
    2 = 强制 Extend PCI#2 Internal Master，以VIA KT133 AGP
    桥上高效处理 Dummy Requests
    3 = 强制上述两种设置
    255 = 自动探测所需设置（默认）

该模块支持所ADB PCM 通道、ac97 混音器、SPDIF、硬EQ
mpu401、gameport。A3D 和波表支持仍在开发中
开发和逆向工程工作正在
https://savannah.nongnu.org/projects/openvortex/ 协调进行
SPDIF 输出AC97 编解码器输出的副本，除非你使
`spdif` pcm 设备，它允许原始数据透传
硬件 EQ SPDIF 仅存在于 Vortex2 Advantage 中

注意：某ALSA 混音器应用程序不能正确处SPDIF 采样率控制
如果你在这方面遇到问题，可以尝试另一个兼ALSA 的混音器
（alsamixer 可用）

### 模块 snd-azt1605


用于基于 Aztech AZT1605 芯片组的 Aztech Sound Galaxy 声卡的模块

port
    BASE 的端口号 #x220,0x240,0x260,0x280
wss_port
    WSS 的端口号 #x530,0x604,0xe80,0xf40
irq
    WSS 鐨?IRQ #锛?,9,10,11锛。
dma1
    WSS 播放DMA #,1,3
dma2
    WSS 采集DMA #,1），-1 = 禁用（默认）
mpu_port
    MPU-401 UART 的端口号 #x300,0x330），-1 = 禁用（默认）
mpu_irq
    MPU-401 UART IRQ #,5,7,9），-1 = 禁用（默认）
fm_port
    OPL3 的端口号 #x388），-1 = 禁用（默认）

该模块支持多块声卡。它不支持自动探测：`port`、`wss_port`
`irq` `dma1` 必须指定。其他值为可选

`port` 需要匹配声卡上 BASE ADDRESS 跳线x220 0x240
或声EEPROM 中存储的值（适用于带 EEPROM 且将“CONFIG MODE
跳线设为“EEPROM SETTING”的声卡）。其他值可以从上面列举
选项中自由选择

如果 `dma2` 被指定且`dma1` 不同，声卡将以全双工模式工作
`dma1=3` 时，只有 `dma2=0` 有效，并且由于只有通道 0 1
可用于采集，这也是启用采集的唯一方式

通用设置``port=0x220 wss_port=0x530 irq=10 dma1=1 dma2=0
mpu_port=0x330 mpu_irq=9 fm_port=0x388``銆。

无论你选择哪个 IRQ DMA 通道，请务必BIOS 中为传统 ISA
保留它们

### 模块 snd-azt2316


用于基于 Aztech AZT2316 芯片组的 Aztech Sound Galaxy 声卡的模块

port
    BASE 的端口号 #x220,0x240,0x260,0x280
wss_port
    WSS 的端口号 #x530,0x604,0xe80,0xf40
irq
    WSS 鐨?IRQ #锛?,9,10,11锛。
dma1
    WSS 播放DMA #,1,3
dma2
    WSS 采集DMA #,1），-1 = 禁用（默认）
mpu_port
    MPU-401 UART 的端口号 #x300,0x330），-1 = 禁用（默认）
mpu_irq
    MPU-401 UART IRQ #,7,9,10），-1 = 禁用（默认）
fm_port
    OPL3 的端口号 #x388），-1 = 禁用（默认）

该模块支持多块声卡。它不支持自动探测：`port`、`wss_port`
`irq` `dma1` 必须指定。其他值为可选

`port` 需要匹配声卡上 BASE ADDRESS 跳线x220 0x240
或声EEPROM 中存储的值（适用于带 EEPROM 且将“CONFIG MODE
跳线设为“EEPROM SETTING”的声卡）。其他值可以从上面列举
选项中自由选择

如果 `dma2` 被指定且`dma1` 不同，声卡将以全双工模式工作
`dma1=3` 时，只有 `dma2=0` 有效，并且由于只有通道 0 1
可用于采集，这也是启用采集的唯一方式

通用设置``port=0x220 wss_port=0x530 irq=10 dma1=1 dma2=0
mpu_port=0x330 mpu_irq=9 fm_port=0x388``銆。

无论你选择哪个 IRQ DMA 通道，请务必BIOS 中为传统 ISA
保留它们

### 模块 snd-aw2


用于 Audiowerk2 声卡的模块

该模块支持多块声卡

### 模块 snd-azt2320


用于基于 Aztech System AZT2320 ISA 芯片（仅 PnP）的声卡的模块

该模块支持多块声卡、PnP 和自动探测

支持电源管理

### 模块 snd-azt3328


用于基于 Aztech AZF3328 PCI 芯片的声卡的模块

joystick
    启用游戏杆（默认关闭

该模块支持多块声卡

### 模块 snd-bt87x


用于基于 Bt87x 芯片的视频卡的模块

digital_rate
    覆盖默认的数字速率（Hz
load_all
    即使不知道声卡型号也加载驱动

该模块支持多块声卡

注意：该模块的默index 值为 -2，即第一个槽位被排除

### 模块 snd-ca0106


用于 Creative Audigy LS SB Live 24bit 的模块

该模块支持多块声卡


### 模块 snd-cmi8330


用于基于 C-Media CMI8330 ISA 芯片的声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

wssport
    CMI8330 芯片（WSS）的端口#
wssirq
    CMI8330 芯片（WSS）的 IRQ #
wssdma
    CMI8330 芯片（WSS）的第一DMA #
sbport
    CMI8330 芯片（SB16）的端口#
sbirq
    CMI8330 芯片（SB16）的 IRQ #
sbdma8
    CMI8330 芯片（SB16）的 8 DMA #
sbdma16
    CMI8330 芯片（SB16）的 16 DMA #
fmport
    （可选）OPL3 I/O 端口
mpuport
    （可选）MPU401 I/O 端口
mpuirq
    （可选）MPU401 irq #

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-cmipci


用于 C-Media CMI8338/8738/8768/8770 PCI 声卡的模块

mpu_port
    MIDI 接口的端口地址（仅 8338）：
    0x300,0x310,0x320,0x330 = 传统端口
    1 = 集成 PCI 端口738 上的默认），
    0 = 禁用
fm_port
    OPL-3 FM 合成器的端口地址（仅 8x38）：
    0x388 = 传统端口
    1 = 集成 PCI 端口738 上的默认），
    0 = 禁用
soft_ac3
    软件转换原始 SPDIF 数据包（model 033）（默认 = 1
joystick_port
    游戏杆端口地址 = 禁用 = 自动探测

该模块支持自动探测和多块声卡

支持电源管理

### 模块 snd-cs4231


用于基于 CS4231 ISA 芯片的声卡的模块

port
    CS4231 芯片的端口号 #
mpu_port
    MPU-401 UART 的端口号 #（可选）1 = 禁用
irq
    CS4231 芯片IRQ #
mpu_irq
    MPU-401 UART 鐨?IRQ #
dma1
    CS4231 芯片的第一DMA #
dma2
    CS4231 芯片的第二个 DMA #

该模块支持多块声卡。该模块不支持自动探测，因此必须指定主端口！！！
其他端口为可选

支持电源管理

### 模块 snd-cs4236


用于基于 CS4232/CS4232A、CS4235/CS4236/CS4236B/CS4237B/CS4238B/
CS4239 ISA 芯片的声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    CS4236 芯片的端口号 #（PnP 设置 - 0x534
cport
    CS4236 芯片的控制端口号 #（PnP 设置 - 0x120,0x210,0xf00
mpu_port
    MPU-401 UART 的端口号 #（PnP 设置 - 0x300），-1 = 禁用
fm_port
    CS4236 芯片FM 端口#（PnP 设置 - 0x388），-1 = 禁用
irq
    CS4236 芯片IRQ #,7,9,11,12,15
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,11,12,15锛。
dma1
    CS4236 芯片的第一DMA #,1,3
dma2
    CS4236 芯片的第二个 DMA #,1,3），-1 = 禁用

该模块支持多块声卡。该模块不支持自动探测（若未使用 ISA PnP），
因此必须指定主端口和控制端口！！！其他端口为可选

支持电源管理

此模块也被别名为 snd-cs4232，因为它同时提供了旧
snd-cs4232 功能

### 模块 snd-cs4281


用于 Cirrus Logic CS4281 声芯片的模块

dual_codec
    第二编解码器 ID = 禁用，默认）

该模块支持多块声卡

支持电源管理

### 模块 snd-cs46xx


用于基于 CS4610/CS4612/CS4614/CS4615/CS4622/CS4624/CS4630/
CS4280 PCI 芯片PCI 声卡的模块

external_amp
    强制启用外部放大器
thinkpad
    强制启用 Thinkpad CLKRUN 控制
mmap_valid
    支持 OSS mmap 模式（默= 0）

该模块支持多块声卡和自动探测
通常外部放大器和 CLKRUN 控制会根PCI 子系统厂设备 ID 自动
探测。如果它们不工作，请显式给出上述选项

支持电源管理

### 模块 snd-cs5530


用于 Cyrix/NatSemi Geode 5530 芯片的模块

### 模块 snd-cs5535audio


用于多功CS5535 配套 PCI 设备的模块

支持电源管理

### 模块 snd-ctxfi


用于 Creative Sound Blaster X-Fi 板卡0k1 / 20k2 芯片）的模块

- Creative Sound Blaster X-Fi Titanium Fatal1ty Champion Series
- Creative Sound Blaster X-Fi Titanium Fatal1ty Professional Series
- Creative Sound Blaster X-Fi Titanium Professional Audio
- Creative Sound Blaster X-Fi Titanium
- Creative Sound Blaster X-Fi Elite Pro
- Creative Sound Blaster X-Fi Platinum
- Creative Sound Blaster X-Fi Fatal1ty
- Creative Sound Blaster X-Fi XtremeGamer
- Creative Sound Blaster X-Fi XtremeMusic


reference_rate
    参考采样率4100 48000（默认）
multiple
    参考采样率的倍数 2（默认）
subsystem
    覆盖用于探测PCI SSID
    该值由 SSVID << 16 | SSDID 组成
    默认值为零，表示不覆盖

该模块支持多块声卡


### 模块 snd-darla20


用于 Echoaudio Darla20 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-darla24


用于 Echoaudio Darla24 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-dt019x


用于 Diamond Technologies DT-019X / Avance Logic ALS-007（仅 PnP）的模块

该模块支持多块声卡。该模块仅在启用 ISA PnP 支持时才可用

支持电源管理

### 模块 snd-dummy


用于虚拟声卡的模块。这个“声卡”不进行任何输出或输入，但你可以
将它用于任何需要声卡的应用程序（如 RealPlayer）

pcm_devs
    分配给每块声卡的 PCM 设备数（默认 = 1，最4
pcm_substreams
    分配给每PCM PCM 子流数（默认 = 8，最128
hrtimer
    使用 hrtimer1，默认）或系统定时器0
fake_buffer
    虚假缓冲区分配（默认 = 1

当创建多PCM 设备时，snd-dummy 对每PCM 设备给出不同的行为：
- 0 = mmap 支持的交错模
- 1 = mmap 支持的非交错模式
- 2 = 不带 mmap 的交错模
- 3 = 不带 mmap 的非交错模式

默认情况下，snd-dummy 驱动不分配真实的缓冲区，而是忽略
写或将单个虚拟页 mmap 到所有缓冲区页，以节省资源
如果你的应用程序需要读/写缓冲区数据保持一致，请传
fake_buffer=0 选项

支持电源管理

### 模块 snd-echo3g


用于 Echoaudio 3G 声卡（Gina3G/Layla3G）的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-emu10k1


用于基于 EMU10K1/EMU10k2 PCI 声卡的模块

- Sound Blaster Live!
- Sound Blaster PCI 512
- Sound Blaster Audigy
- E-MU APS（部分支持）
- E-MU DAS

extin
    用于 FX8010 的可用外部输入位图（见下
extout
    用于 FX8010 的可用外部输出位图（见下
seq_ports
    分配的音序器端口（默4
max_synth_voices
    用于波表的语音数上限（默64
max_buffer_size
    MB 为单位指定波PCM 缓冲区的最大大小
    默认值为 128
enable_ir
    启用 IR

该模块支持多块声卡和自动探测

输入与输出配		[extin/extout]
- Creative Card wo/Digital out			[0x0003/0x1f03]
- Creative Card w/Digital out			[0x0003/0x1f0f]
- Creative Card w/Digital CD in			[0x000f/0x1f0f]
- Creative Card wo/Digital out + LiveDrive	[0x3fc3/0x1fc3]
- Creative Card w/Digital out + LiveDrive	[0x3fc3/0x1fcf]
- Creative Card w/Digital CD in + LiveDrive	[0x3fcf/0x1fcf]
- Creative Card wo/Digital out + Digital I/O 2  [0x0fc3/0x1f0f]
- Creative Card w/Digital out + Digital I/O 2	[0x0fc3/0x1f0f]
- Creative Card w/Digital CD in + Digital I/O 2	[0x0fcf/0x1f0f]
- Creative Card 5.1/w Digital out + LiveDrive	[0x3fc3/0x1fff]
- Creative Card 5.1 (c) 2003			[0x3fc3/0x7cff]
- Creative Card all ins and outs		[0x3fff/0x7fff]

支持电源管理

### 模块 snd-emu10k1x


用于 Creative Emu10k1X（SB Live Dell OEM 版本）的模块

该模块支持多块声卡

### 模块 snd-ens1370


用于 Ensoniq AudioPCI ES1370 PCI 声卡的模块

- SoundBlaster PCI 64
- SoundBlaster PCI 128

joystick
    启用游戏杆（默认关闭

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-ens1371


用于 Ensoniq AudioPCI ES1371 PCI 声卡的模块

- SoundBlaster PCI 64
- SoundBlaster PCI 128
- SoundBlaster Vibra PCI

joystick_port
    游戏杆的端口#x200,0x208,0x210,0x218），0 = 禁用
    （默认） = 自动探测

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-es1688


用于 ESS AudioDrive ES-1688 ES-688 声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）
mpu_port
    MPU-401 端口的端口号 #x300,0x310,0x320,0x330），-1 = 禁用（默认）
mpu_irq
    MPU-401 端口IRQ #,7,9,10
fm_port
    OPL3 的端口号 #（可选；默认MPU 端口共用

`isapnp=0` 时，可使用以下附加选项

port
    ES-1688 芯片的端口号 #x220,0x240,0x260
irq
    ES-1688 芯片IRQ #,7,9,10
dma8
    ES-1688 芯片DMA #,1,3

该模块支持多块声卡和自动探测（不MPU-401 端口
以及ES968 芯片PnP

### 模块 snd-es18xx


用于 ESS AudioDrive ES-18xx 声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    ES-18xx 芯片的端口号 #x220,0x240,0x260
mpu_port
    MPU-401 端口的端口号 #x300,0x310,0x320,0x330），-1 = 禁用（默认）
fm_port
    FM 的端口号 #（可选，未使用）
irq
    ES-18xx 芯片IRQ #,7,9,10
dma1
    ES-18xx 芯片的第一DMA #,1,3
dma2
    ES-18xx 芯片的第一DMA #,1,3

该模块支持多块声卡、ISA PnP 和自动探测（若未使用原生 ISA PnP
例程则不MPU-401 端口）。当 `dma2` `dma1` 相等时，驱动
半双工方式工作

支持电源管理

### 模块 snd-es1938


用于基于 ESS Solo-1（ES1938,ES1946）芯片的声卡的模块

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-es1968


用于基于 ESS Maestro-1/2/2E（ES1968/ES1978）芯片的声卡的模块

total_bufsize
    kB 为单位的总缓冲区大小-4096kB
pcm_substreams_p
    播放通道数（1-8，默2
pcm_substreams_c
    采集通道数（1-8，默0
clock
    时钟 = 自动探测
use_pm
    支持电源管理 = 关闭 = 开启，2 = 自动（默认）
enable_mpu
    启用 MPU401 = 关闭 = 开启，2 = 自动（默认）
joystick
    启用游戏杆（默认关闭

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-fm801


用于基于 ForteMedia FM801 PCI 声卡的模块

tea575x_tuner
    启用 TEA575x 调谐器；
    1 = MediaForte 256-PCS锛。
    2 = MediaForte 256-PCPR锛。
    3 = MediaForte 64-PCR
    16 位为视频（收音机）设备号 + 1
    例如x10002（MediaForte 256-PCPR，设1

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-gina20


用于 Echoaudio Gina20 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-gina24


用于 Echoaudio Gina24 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-gusclassic


用于 Gravis UltraSound Classic 声卡的模块

port
    GF1 芯片的端口号 #x220,0x230,0x240,0x250,0x260
irq
    GF1 芯片IRQ #,5,9,11,12,15
dma1
    GF1 芯片DMA #,3,5,6,7
dma2
    GF1 芯片DMA #,3,5,6,7,-1=禁用
joystick_dac
    0 31，（0.59V-4.52V 0.389V-2.98V
voices
    GF1 语音数上限（14-32
pcm_voices
    保留PCM 语音

该模块支持多块声卡和自动探测

### 模块 snd-gusextreme


用于 Gravis UltraSound Extreme（Synergy ViperMax）声卡的模块

port
    ES-1688 芯片的端口号 #x220,0x230,0x240,0x250,0x260
gf1_port
    GF1 芯片的端口号 #x210,0x220,0x230,0x240,0x250,0x260,0x270
mpu_port
    MPU-401 端口的端口号 #x300,0x310,0x320,0x330），-1 = 禁用
irq
    ES-1688 芯片IRQ #,7,9,10
gf1_irq
    GF1 芯片IRQ #,5,9,11,12,15
mpu_irq
    MPU-401 端口IRQ #,7,9,10
dma8
    ES-1688 芯片DMA #,1,3
dma1
    GF1 芯片DMA #,3,5,6,7
joystick_dac
    0 31，（0.59V-4.52V 0.389V-2.98V
voices
    GF1 语音数上限（14-32
pcm_voices
    保留PCM 语音

该模块支持多块声卡和自动探测（不MPU-401 端口）

### 模块 snd-gusmax


用于 Gravis UltraSound MAX 声卡的模块

port
    GF1 芯片的端口号 #x220,0x230,0x240,0x250,0x260
irq
    GF1 芯片IRQ #,5,9,11,12,15
dma1
    GF1 芯片DMA #,3,5,6,7
dma2
    GF1 芯片DMA #,3,5,6,7,-1=禁用
joystick_dac
    0 31，（0.59V-4.52V 0.389V-2.98V
voices
    GF1 语音数上限（14-32
pcm_voices
    保留PCM 语音

该模块支持多块声卡和自动探测

### 模块 snd-hda-intel


用于 Intel HD Audio（ICH6, ICH6M, ESB2, ICH7, ICH8, ICH9, ICH10,
PCH, SCH）、ATI SB450, SB600, R600, RS600, RS690, RS780, RV610, RV620,
RV630, RV635, RV670, RV770, VIA VT8251/VT8237A, SIS966, ULI M5461 的模块

[每个声卡实例的多个选项]

model
    强制指定型号名称
position_fix
    修正 DMA 指针
    -1 = 系统默认：根据控制器硬件选择合适的方案
    0 = 自动：当 POSBUF 不工作时回退LPIB
    1 = 使用 LPIB
    2 = POSBUF：使用位置缓冲区
    3 = VIACOMBO：针对采集的 VIA 特定规避方案
    4 = COMBO：播放使LPIB，采集流自动
    5 = SKL+：应用近Intel 芯片上可用的延迟计算
    6 = FIFO：用固定FIFO 大小修正位置，用于近AMD 芯片
probe_mask
    用于探测编解码器的位掩码（默= -1，即所有槽位）
    当第 8 位（0x100）置位时，低 8 位用作“固定”的编解码器
    槽位；即无论硬件报告什么，驱动都会探测这些槽位
probe_only
    仅探测而不初始化编解码器（默认=off）；
    用于检查编解码器的初始状态以调试
bdl_pos_adj
    以采样为单位指定 DMA IRQ 定时延迟
    传入 -1 将让驱动根据控制器芯片选择合适的取值
patch
    指定在初始化编解码器之前用于修改 HD-audio 设置的早
    “patch”文件
    此选项仅在设置`CONFIG_SND_HDA_PATCH_LOADER=y` 时可用
    详见 hd-audio/notes.rst
beep_mode
    选择蜂鸣注册模式=关闭=开启）
    默认值通过 `CONFIG_SND_HDA_INPUT_BEEP_MODE` kconfig 设置

[单一（全局）选项]

single_cmd
    使用单一立即命令与编解码器通信
    （仅用于调试
enable_msi
    启用消息信号中断（MSI）（默认 = 关闭
power_save
    自动省电超时（以秒为单位 = 禁用
power_save_controller
    在省电模式下复位 HD-audio 控制器（默认 = 开启）
pm_blacklist
    启用 / 禁用电源管理拒绝列表（默= 查询 PM
    拒绝列表 = 跳过 PM 拒绝列表 = 强制关闭运行PM
align_buffer_size
    强制将缓冲区/周期大小四舍五入128 字节的倍数
    这在内存访问方面更高效，HDA 规范并不要求
    且会阻止用户指定精确的周缓冲区大小。（默认 = 开启）
snoop
    启用/禁用窥探（默= 开启）

该模块支持多块声卡和自动探测

有关 HD-audio 驱动的更多细节，请参hd-audio/notes.rst

每个编解码器都可能有针对不同配置的型号表
如果你的机器未列在其中，则会设置默认（通常是最精简的）
配置。在这种情况下你可以传入 `model=<name>` 选项来指
某个型号。根据编解码器芯片的不同有不同型号。可用型号列
瑙?hd-audio/models.rst銆。

型号`generic` 被视为一种特殊情况。当给定此型号时，驱
使用通用的编解码器解析器，而不使用“codec-patch”。这有时
对测试和调试很有用

model 选项也可用于别名到另一PCI 或编解码SSID
当以 `model=XXXX:YYYY` 的形式传入时，其XXXX YYYY
分别是十六进制的子系统厂商和子系统设ID，驱动将把该
SSID 作为异常表的参考

如果默认配置不工作，而上述某一项与你的设备匹配，请将它
连同 alsa-info.sh 的输出（使用 `--no-upload` 选项）一
报告kernel bugzilla alsa-devel 邮件列表
（见 `Links and Addresses`_ 小节）

`power_save` `power_save_controller` 选项用于省电模式
详见 powersave.rst

注意 2：如果你的输出有咔嗒声，请尝试模块选项
`position_fix=1` `2`。`position_fix=1` 将使用未FIFO
大小修正SD_LPIB 寄存器值作为当DMA 指针。`position_fix=2`
将使驱动使用位置缓冲区而不是读SD_LPIB 寄存器
（通常 SD_LPIB 寄存器比位置缓冲区更精确。）

`position_fix=3` 专用VIA 设备。采集流的位置从 LPIB 
POSBUF 两个值中检查。`position_fix=4` 是组合模式，播放使用
LPIB，采集使POSBUF

注意：如果在加载时出现大`azx_get_response timeout` 消息
很可能是中断的问题（例如 ACPI irq 路由）。尝试用
`pci=noacpi` 之类的选项启动。此外，你可以尝`single_cmd=1`
模块选项。这会将 HDA 控制器与编解码器之间的通信方式切换
单一立即命令，而不CORB/RIRB。基本上，单一命令模式仅用
BIOS，你也不会收到未 solicited 事件。但至少，它独立irq
工作。请记住这是最后手段，应尽可能避免…

关于 `azx_get_response timeout` 问题的更多说明：
在某些硬件上，你可能需要添加合适的 probe_mask 选项来避
上述 `azx_get_response timeout` 问题。当访问不存在或不工
的编解码器槽位（很可能是调制器槽位）导致 HD-audio 总线上的
通信停滞时就会发生这种情况。你可以通过启用 `CONFIG_SND_DEBUG_VERBOSE`
来查看探测了哪些编解码器槽位，或者直接从编解码器 proc 文件
文件名看出。然后通过 probe_mask 选项限制要探测的槽位
例如，`probe_mask=1` 表示只探测第一个槽位，`probe_mask=4`
表示只探测第三个槽位

支持电源管理

### 模块 snd-hdsp


用于 RME Hammerfall DSP 音频接口的模块

该模块支持多块声卡

注意：当设置`CONFIG_FW_LOADER` 时，固件数据可以通过 hotplug
自动加载。否则，你需要通过 alsa-tools 软件包中包含hdsploader
工具加载固件。固件数据位alsa-firmware 软件包中

注意：snd-page-alloc 模块承担了以snd-hammerfall-mem 模块
工作。它会在发现任何 HDSP 声卡时预先分配缓冲区。为了确
缓冲区分配成功，请在启动序列的早期阶段加snd-page-alloc
模块。详`Early Buffer Allocation`_ 小节

### 模块 snd-hdspm


用于 RME HDSP MADI 板卡的模块

precise_ptr
    启用精确指针，或禁用
line_outs_monitor
    默认将播放流发送到模拟输出
enable_monitor
    默认在通道 63/64 上启用模拟输出

详见 hdspm.rst

### 模块 snd-ice1712


用于基于 Envy24（ICE1712）的 PCI 声卡的模块

- MidiMan M Audio Delta 1010
- MidiMan M Audio Delta 1010LT
- MidiMan M Audio Delta DiO 2496
- MidiMan M Audio Delta 66
- MidiMan M Audio Delta 44
- MidiMan M Audio Delta 410
- MidiMan M Audio Audiophile 2496
- TerraTec EWS 88MT
- TerraTec EWS 88D
- TerraTec EWX 24/96
- TerraTec DMX 6Fire
- TerraTec Phase 88
- Hoontech SoundTrack DSP 24
- Hoontech SoundTrack DSP 24 Value
- Hoontech SoundTrack DSP 24 Media 7.1
- Event Electronics, EZ8
- Digigram VX442
- Lionstracs, Mediastaton
- Terrasoniq TS 88

model
    使用给定的板卡型号，以下之一
    delta1010, dio2496, delta66, delta44, audiophile, delta410,
    delta1010lt, vx442, ewx2496, ews88mt, ews88mt_new, ews88d,
    dmx6fire, dsp24, dsp24_value, dsp24_71, ez8,
    phase88, mediastation
omni
    MidiMan M-Audio Delta44/66 Omni I/O 支持
cs8427_timeout
    CS8427 芯片（S/PDIF 收发器）的复位超时，msec
    为单位，默认值为 500.5 秒）

该模块支持多块声卡和自动探测
注意：消费部分并非在所有基Envy24 的声卡上都使
（例如在 MidiMan Delta 系列中）

注意：支持的板卡通过读取 EEPROM PCI SSID（若 EEPROM
不可用）来检测。如果驱动配置不正确，或你想尝试另一
类型进行测试，可以通过传入 `model` 模块选项来覆盖型号

### 模块 snd-ice1724


用于基于 Envy24HT（VT/ICE1724）、Envy24PT（VT1720）的 PCI 声卡的模块

- MidiMan M Audio Revolution 5.1
- MidiMan M Audio Revolution 7.1
- MidiMan M Audio Audiophile 192
- AMP Ltd AUDIO2000
- TerraTec Aureon 5.1 Sky
- TerraTec Aureon 7.1 Space
- TerraTec Aureon 7.1 Universe
- TerraTec Phase 22
- TerraTec Phase 28
- AudioTrak Prodigy 7.1
- AudioTrak Prodigy 7.1 LT
- AudioTrak Prodigy 7.1 XT
- AudioTrak Prodigy 7.1 HIFI
- AudioTrak Prodigy 7.1 HD2
- AudioTrak Prodigy 192
- Pontis MS300
- Albatron K8X800 Pro II
- Chaintech ZNF3-150
- Chaintech ZNF3-250
- Chaintech 9CJS
- Chaintech AV-710
- Shuttle SN25P
- Onkyo SE-90PCI
- Onkyo SE-200PCI
- ESI Juli@
- ESI Maya44
- Hercules Fortissimo IV
- EGO-SYS WaveTerminal 192M

model
    使用给定的板卡型号，以下之一
    revo51, revo71, amp2000, prodigy71, prodigy71lt,
    prodigy71xt, prodigy71hifi, prodigyhd2, prodigy192,
    juli, aureon51, aureon71, universe, ap192, k8x800,
    phase22, phase28, ms300, av710, se200pci, se90pci,
    fortissimo4, sn25p, WT192M, maya44

该模块支持多块声卡和自动探测

注意：支持的板卡通过读取 EEPROM PCI SSID（若 EEPROM
不可用）来检测。如果驱动配置不正确，或你想尝试另一
类型进行测试，可以通过传入 `model` 模块选项来覆盖型号

### 模块 snd-indigo


用于 Echoaudio Indigo 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-indigodj


用于 Echoaudio Indigo DJ 的模块


该模块支持多块声卡
该驱动需要内核提供固件加载器支持


### 模块 snd-indigoio


用于 Echoaudio Indigo IO 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-intel8x0


用于来自 Intel 及兼容厂商的 AC'97 主板的模块

- Intel i810/810E, i815, i820, i830, i84x, MX440 ICH5, ICH6, ICH7,
  6300ESB, ESB2
- SiS 7012 (SiS 735)
- NVidia NForce, NForce2, NForce3, MCP04, CK804 CK8, CK8S, MCP501
- AMD AMD768, AMD8111
- ALi m5455

ac97_clock
    AC'97 编解码器时钟基准 = 自动探测
ac97_quirk
    针对异常硬件AC'97 规避方案
    见下面的 `AC97 Quirk Option`_ 小节
buggy_irq
    启用某些主板上异常中断的规避方案
    （在 nForce 芯片上默认为开启，其他为关闭）
buggy_semaphore
    启用针对带有异常信号量的硬件的规避方案（例如某些
    ASUS 笔记本）（默认关闭）
spdif_aclink
    使用通过 AC-link S/PDIF，而不是来自控制器芯片
    直接连接 = 关闭 = 开启，-1 = 默认

该模块支持单芯片和自动探测

注意：最新的驱动支持芯片时钟的自动探测。如果你仍然遇到
播放过快的问题，请通过模块选项 `ac97_clock=41194` 显式
指定时钟

本驱动不支持游戏MIDI 端口。如果你的主板有这些设备，请
分别使用 ns558 snd-mpu401 模块

支持电源管理

### 模块 snd-intel8x0m


用于 Intel ICH（i8x0）芯片组 MC97 调制器的模块

- Intel i810/810E, i815, i820, i830, i84x, MX440 ICH5, ICH6, ICH7
- SiS 7013 (SiS 735)
- NVidia NForce, NForce2, NForce2s, NForce3
- AMD AMD8111
- ALi m5455

ac97_clock
    AC'97 编解码器时钟基准 = 自动探测

该模块支持单块声卡和自动探测

注意：该模块的默index 值为 -2，即第一个槽位被排除

支持电源管理

### 模块 snd-interwave


用于 Gravis UltraSound PnP、Dynasonic 3-D/Pro、STB Sound Rage 32
以及基于 AMD InterWave (tm) 芯片的其他声卡的模块

joystick_dac
    0 31，（0.59V-4.52V 0.389V-2.98V
midi
    1 = 启用 MIDI UART = 禁用 MIDI UART（默认）
pcm_voices
    为合成器保留PCM 语音数（默认 2
effect
    1 = 启用 InterWave 效果（默0）；需8 个语
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    InterWave 芯片的端口号 #x210,0x220,0x230,0x240,0x250,0x260
irq
    InterWave 芯片IRQ #,5,9,11,12,15
dma1
    InterWave 芯片DMA #,1,3,5,6,7
dma2
    InterWave 芯片DMA #,1,3,5,6,7,-1=禁用

该模块支持多块声卡、自动探测和 ISA PnP

### 模块 snd-interwave-stb


用于 UltraSound 32-Pro（Compaq 使用STB 声卡）以及基AMD
InterWave (tm) 芯片、并带有 TEA6330T 电路以扩展控制低音
高音和主音量的其他声卡的模块

joystick_dac
    0 31，（0.59V-4.52V 0.389V-2.98V
midi
    1 = 启用 MIDI UART = 禁用 MIDI UART（默认）
pcm_voices
    为合成器保留PCM 语音数（默认 2
effect
    1 = 启用 InterWave 效果（默0）；需8 个语
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    InterWave 芯片的端口号 #x210,0x220,0x230,0x240,0x250,0x260
port_tc
    TEA6330T 芯片（i2c 总线）的音调控制端口#x350,0x360,0x370,0x380
irq
    InterWave 芯片IRQ #,5,9,11,12,15
dma1
    InterWave 芯片DMA #,1,3,5,6,7
dma2
    InterWave 芯片DMA #,1,3,5,6,7,-1=禁用

该模块支持多块声卡、自动探测和 ISA PnP

### 模块 snd-jazz16


用于 Media Vision Jazz16 芯片组的模块。该芯片组由 3 个芯片组成：
MVD1216 + MVA416 + MVA514銆。

port
    SB DSP 芯片的端口号 #x210,0x220,0x230,0x240,0x250,0x260
irq
    SB DSP 芯片IRQ #,5,7,9,10,15
dma8
    SB DSP 芯片DMA #,3
dma16
    SB DSP 芯片DMA #,7
mpu_port
    MPU-401 端口#x300,0x310,0x320,0x330
mpu_irq
    MPU-401 鐨?irq #锛?,3,5,7锛。

该模块支持多块声卡

### 模块 snd-korg1212


用于 Korg 1212 IO PCI 声卡的模块

该模块支持多块声卡

### 模块 snd-layla20


用于 Echoaudio Layla20 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-layla24


用于 Echoaudio Layla24 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-lola


用于 Digigram Lola PCI-e 板卡的模块

该模块支持多块声卡

### 模块 snd-lx6464es


用于 Digigram LX6464ES 板卡的模块

该模块支持多块声卡

### 模块 snd-maestro3


用于 Allegro/Maestro3 芯片的模块

external_amp
    启用外部放大器（默认启用
amp_gpio
    外部放大器的 GPIO 引脚号（0-15）或 -1 表示默认引脚
    （allegro 8，其他为 1

该模块支持自动探测和多芯片

注意：放大器的绑定取决于硬件。如果所有通道都已解除静音
仍然没有声音，请尝试通过 amp_gpio 选项指定其他 gpio 连接
例如，某些松下笔记本可能需`amp_gpio=0x0d` 选项

支持电源管理

### 模块 snd-mia


用于 Echoaudio Mia 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-miro


用于 Miro 声卡：miroSOUND PCM 1 pro、miroSOUND PCM 12
miroSOUND PCM 20 Radio銆。

port
    端口#x530,0x604,0xe80,0xf40
irq
    IRQ #锛?,7,9,10,11锛。
dma1
    第一dma #,1,3
dma2
    第二dma #,1
mpu_port
    MPU-401 端口#x300,0x310,0x320,0x330
mpu_irq
    MPU-401 鐨?irq #锛?,7,9,10锛。
fm_port
    FM 端口#x388
wss
    启用 WSS 模式
ide
    启用板载 ide 支持

### 模块 snd-mixart


用于 Digigram miXart8 声卡的模块

该模块支持多块声卡
注意：一miXart8 板卡会被表示4 alsa 声卡
详见 Documentation/sound/cards/mixart.rst

当驱动编译为模块且支hotplug 固件时，固件数据会通过
hotplug 自动加载。请alsa-firmware 软件包中安装所需
固件文件。当没有可用hotplug 固件加载器时，你需要通过
alsa-tools 软件包中mixartloader 工具加载固件

### 模块 snd-mona


用于 Echoaudio Mona 的模块

该模块支持多块声卡
该驱动需要内核提供固件加载器支持

### 模块 snd-mpu401


用于 MPU-401 UART 设备的模块

port
    端口号或 -1（禁用）
irq
    IRQ 号或 -1（禁用）
pnp
    PnP 检- 0 = 禁用 = 启用（默认）

该模块支持多设备PnP

### 模块 snd-msnd-classic


用于 Turtle Beach MultiSound Classic、Tahiti Monterey 声卡的模块

io
    msnd-classic 声卡的端口号 #
irq
    msnd-classic 声卡IRQ #
mem
    内存地址xb0000, 0xc8000, 0xd0000, 0xd8000, 0xe0000 0xe8000
write_ndelay
    启用ndelay（默= 1
calibrate_signal
    校准信号（默= 0
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）
digital
    存在数字子板（默= 0
cfg
    配置端口x250, 0x260 0x270）默= PnP
reset
    复位所有设
mpu_io
    MPU401 I/O 端口
mpu_irq
    MPU401 irq#
ide_io0
    IDE 端口 #0
ide_io1
    IDE 端口 #1
ide_irq
    IDE irq#
joystick_io
    游戏I/O 端口

该驱动需要固件文`turtlebeach/msndinit.bin` 
`turtlebeach/msndperm.bin` 位于正确的固件目录中

关于该驱动的重要信息，请参见 Documentation/sound/cards/multisound.sh
注意它已被停止维护，Voyetra Turtle Beach 关于它的知识
条目仍可在以下地址获取
https://www.turtlebeach.com

### 模块 snd-msnd-pinnacle


用于 Turtle Beach MultiSound Pinnacle/Fiji 声卡的模块

io
    pinnacle/fiji 声卡的端口号 #
irq
    pinnalce/fiji 声卡IRQ #
mem
    内存地址xb0000, 0xc8000, 0xd0000, 0xd8000, 0xe0000 0xe8000
write_ndelay
    启用ndelay（默= 1
calibrate_signal
    校准信号（默= 0
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

该驱动需要固件文`turtlebeach/pndspini.bin` 
`turtlebeach/pndsperm.bin` 位于正确的固件目录中

### 模块 snd-mtpav


用于 MOTU MidiTimePiece AV 多端MIDI（并口）的模块

port
    MTPAV I/O 端口#x378,0x278，默0x378
irq
    MTPAV IRQ #,5，默7
hwports
    受支持的硬件端口数，默认=8

模块仅支1 块声卡。该模块没有 enable 选项

### 模块 snd-mts64


用于 Ego Systems（ESI）Miditerminal 4140 的模块

该模块支持多设备
需parport（`CONFIG_PARPORT`）

### 模块 snd-nm256


用于 NeoMagic NM256AV/ZX 芯片的模块

playback_bufsize
    最大播放帧大小，以 kB 为单位（4-128kB
capture_bufsize
    最大采集帧大小，以 kB 为单位（4-128kB
force_ac97
    0 1（默认禁用）
buffer_top
    指定缓冲区顶部地址
use_cache
    0 1（默认禁用）
vaio_hack
    别名 buffer_top=0x25a800
reset_workaround
    为某些笔记本启用 AC97 RESET 规避方案
reset_workaround2
    为某些其他笔记本启用扩展AC97 RESET 规避方案

该模块支持单芯片和自动探测

支持电源管理

注意：在某些笔记本上，缓冲区地址无法自动探测，或在初始化
期间导致挂起。在这种情况下，请通过 buffer_top 选项显式指定
缓冲区顶部地址。例如：
Sony F250：buffer_top=0x25a800
Sony F270：buffer_top=0x272800
该驱动仅支持 ac97 编解码器。即使未探测到，也可以强制初始化/
使用 ac97。在这种情况下，使用 `force_ac97=1` 选项——但能否
工作***作任何保证！

注意：NM256 芯片可以在内部与AC97 编解码器连接。本驱动
仅支AC97 编解码器，无法与带有其他（很可能CS423x 
OPL3SAx）芯片的机器工作，即使该设备lspci 中能被探测到
在这种情况下，请尝试其他驱动，例snd-cs4232 snd-opl3sa2
其中一些支ISA-PnP，一些不支持。在没有 ISA PnP 的情况下
你需要指`isapnp=0` 以及正确的硬件参数

注意：某些笔记本需要针AC97 RESET 的规避方案。对于已知的
硬件Dell Latitude LS Sony PCG-F305，此规避方案会自
启用。对于其他出现硬冻结的笔记本，你可以尝试 `reset_workaround=1`
选项

注意：Dell Latitude CSx 笔记本在 AC97 RESET 方面有另一个问题
在这些笔记本上，reset_workaround2 选项默认开启。如果之前的
reset_workaround 选项没有帮助，这个选项值得一试

注意：这个驱动真的很糟糕。它移植OSS 驱动，而后者是黑魔
般逆向工程的产物。如果驱动在 X-server 之后加载（如上所述）
编解码器的探测会失败。你可能能够强制加载该模块，但可能导
挂起。因此，如果遇到这类问题，请确保X 之前加载此模块

### 模块 snd-opl3sa2


用于 Yamaha OPL3-SA2/SA3 声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    OPL3-SA 芯片的控制端口号 #x370
sb_port
    OPL3-SA 芯片SB 端口#x220,0x240
wss_port
    OPL3-SA 芯片WSS 端口#x530,0xe80,0xf40,0x604
midi_port
    MPU-401 UART 的端口号 #x300,0x330），-1 = 禁用
fm_port
    OPL3-SA 芯片FM 端口#x388），-1 = 禁用
irq
    OPL3-SA 芯片IRQ #,7,9,10
dma1
    Yamaha OPL3-SA 芯片的第一DMA #,1,3
dma2
    Yamaha OPL3-SA 芯片的第二个 DMA #,1,3），-1 = 禁用

该模块支持多块声卡和 ISA PnP。它不支持自动探测（若未使用
ISA PnP），因此必须指定所有端口！！！

支持电源管理

### 模块 snd-opti92x-ad1848


用于基于 OPTi 82c92x Analog Devices AD1848 芯片的声卡的模块
该模块也适用OAK Mozart 声卡

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    WSS 芯片的端口号 #x530,0xe80,0xf40,0x604
mpu_port
    MPU-401 UART 的端口号 #x300,0x310,0x320,0x330
fm_port
    OPL3 设备的端口号 #x388
irq
    WSS 芯片IRQ #,7,9,10,11
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛。
dma1
    WSS 芯片的第一DMA #,1,3

该模块仅支持一块声卡、自动探测和 PnP

### 模块 snd-opti92x-cs4231


用于基于 OPTi 82c92x Crystal CS4231 芯片的声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    WSS 芯片的端口号 #x530,0xe80,0xf40,0x604
mpu_port
    MPU-401 UART 的端口号 #x300,0x310,0x320,0x330
fm_port
    OPL3 设备的端口号 #x388
irq
    WSS 芯片IRQ #,7,9,10,11
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛。
dma1
    WSS 芯片的第一DMA #,1,3
dma2
    WSS 芯片的第二个 DMA #,1,3

该模块仅支持一块声卡、自动探测和 PnP

### 模块 snd-opti93x


用于基于 OPTi 82c93x 芯片的声卡的模块

isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

`isapnp=0` 时，可使用以下选项

port
    WSS 芯片的端口号 #x530,0xe80,0xf40,0x604
mpu_port
    MPU-401 UART 的端口号 #x300,0x310,0x320,0x330
fm_port
    OPL3 设备的端口号 #x388
irq
    WSS 芯片IRQ #,7,9,10,11
mpu_irq
    MPU-401 UART 鐨?IRQ #锛?,7,9,10锛。
dma1
    WSS 芯片的第一DMA #,1,3
dma2
    WSS 芯片的第二个 DMA #,1,3

该模块仅支持一块声卡、自动探测和 PnP

### 模块 snd-oxygen


用于基于 C-Media CMI8786/8787/8788 芯片的声卡的模块

- Asound A-8788
- Asus Xonar DG/DGX
- AuzenTech X-Meridian
- AuzenTech X-Meridian 2G
- Bgears b-Enspirer
- Club3D Theatron DTS
- HT-Omega Claro (plus)
- HT-Omega Claro halo (XT)
- Kuroutoshikou CMI8787-HG2PCI
- Razer Barracuda AC-1
- Sondigo Inferno
- TempoTec HiFier Fantasia
- TempoTec HiFier Serenade

该模块支持自动探测和多块声卡

### 模块 snd-pcsp


用于内部 PC 扬声器（PC-Speaker）的模块

nopcm
    禁用 PC 扬声PCM 声音。仅保留蜂鸣声
nforce_wa
    启用 NForce 芯片组规避方案。预期声音质量较差

该模块支持系统蜂鸣、某PCM 播放，甚至几个混音器控制

### 模块 snd-pcxhr


用于 Digigram PCXHR 板卡的模块

该模块支持多块声卡

### 模块 snd-portman2x4


用于 Midiman Portman 2x4 并口 MIDI 接口的模块

该模块支持多块声卡

### 模块 snd-powermac（仅 ppc 上）


用于 PowerMac、iMac iBook 板载声芯片的模块

enable_beep
    启用使用 PCM 的蜂鸣声（默认启用）

模块支持自动探测芯片

注意：该驱动在字节序方面可能有问题

支持电源管理

### 模块 snd-pxa2xx-ac97（仅 arm 上）


用于 Intel PXA2xx 芯片AC97 驱动的模块

仅用ARM 架构

支持电源管理

### 模块 snd-riptide


用于 Conexant Riptide 芯片的模块

joystick_port
    游戏杆端口号 #（默认：0x200
mpu_port
    MPU401 端口#（默认：0x330
opl3_port
    OPL3 端口#（默认：0x388

该模块支持多块声卡
该驱动需要内核提供固件加载器支持
你需要将固件文件 `riptide.hex` 安装到标准固件路
（例/lib/firmware）

### 模块 snd-rme32


用于 RME Digi32、Digi32 Pro Digi32/8（Sek'd Prodif32
Prodif96 Prodif Gold）声卡的模块

该模块支持多块声卡

### 模块 snd-rme96


用于 RME Digi96、Digi96/8 Digi96/8 PRO/PAD/PST 声卡的模块

该模块支持多块声卡

### 模块 snd-rme9652


用于 RME Digi9652（Hammerfall、Hammerfall-Light）声卡的模块

precise_ptr
    启用精确指针（工作不可靠）。（默认 = 0

该模块支持多块声卡

注意：snd-page-alloc 模块承担了以snd-hammerfall-mem 模块
的工作。它会在发现任何 RME9652 声卡时预先分配缓冲区。为了确
缓冲区分配成功，请在启动序列的早期阶段加snd-page-alloc
模块。详`Early Buffer Allocation`_ 小节

### 模块 snd-sa11xx-uda1341（仅 arm 上）


用于 Compaq iPAQ H3600 声卡Philips UDA1341TS 的模块

模块仅支持一块声卡
模块没有 enable index 选项

支持电源管理

### 模块 snd-sb8


用于 8 SoundBlaster 声卡：SoundBlaster 1.0、SoundBlaster 2.0
SoundBlaster Pro 的模块

port
    SB DSP 芯片的端口号 #x220,0x240,0x260
irq
    SB DSP 芯片IRQ #,7,9,10
dma8
    SB DSP 芯片DMA #,3

该模块支持多块声卡和自动探测

支持电源管理

### 妯″潡 snd-sb16 鍜?snd-sbawe


用于 16 SoundBlaster 声卡：SoundBlaster 16（PnP）
SoundBlaster AWE 32（PnP）、SoundBlaster AWE 64 PnP 的模块

mic_agc
    麦克风自动增益控- 0 = 禁用 = 启用（默认）
csp
    ASP/CSP 芯片支持 - 0 = 禁用（默认） = 启用
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

isapnp=0 时，可使用以下选项

port
    SB DSP 4.x 芯片的端口号 #x220,0x240,0x260
mpu_port
    MPU-401 UART 的端口号 #x300,0x330），-1 = 禁用
awe_port
    EMU8000 合成器的基端口号 #x620,0x640,0x660）（snd-sbawe
    模块
irq
    SB DSP 4.x 芯片IRQ #,7,9,10
dma8
    SB DSP 4.x 芯片8 DMA #,1,3
dma16
    SB DSP 4.x 芯片16 DMA #,6,7

该模块支持多块声卡、自动探测和 ISA PnP


注意：若要在 16 位半双工模式下使Vibra16X 声卡，必
通过 dma16 = -1 模块参数禁用 16 DMA。此外，所Sound Blaster 16
类型声卡都可以通过禁用16 DMA 通道，改8 DMA 通道
16 位半双工模式工作

支持电源管理


### 模块 snd-sc6000


用于 Gallant SC-6000 声卡及后续型号：SC-6600 SC-7000 的模块

port
    端口#x220 0x240
mss_port
    MSS 端口#x530 0xe80
irq
    IRQ #锛?,7,9,10,11锛。
mpu_irq
    MPU-401 IRQ #,7,9,10），0 - MPU-401 irq
dma
    DMA #锛?,3,0锛。
joystick
    启用游戏端口 - 0 = 禁用（默认） = 启用

该模块支持多块声卡

该声卡也称为 Audio Excel DSP 16 Zoltrix AV302

### 模块 snd-sscape


用于 ENSONIQ SoundScape 声卡的模块

port
    端口#（PnP 设置
wss_port
    WSS 端口#（PnP 设置
irq
    IRQ #（PnP 设置
mpu_irq
    MPU-401 IRQ #（PnP 设置
dma
    DMA #（PnP 设置
dma2
    第二DMA #（PnP 设置1 表示禁用
joystick
    启用游戏端口 - 0 = 禁用（默认） = 启用

该模块支持多块声卡

该驱动需要内核提供固件加载器支持

### 模块 snd-sun-amd7930（仅 sparc 上）


用于 Sparc 上的 AMD7930 声芯片的模块

该模块支持多块声卡

### 模块 snd-sun-cs4231（仅 sparc 上）


用于 Sparc 上的 CS4231 声芯片的模块

该模块支持多块声卡

### 模块 snd-sun-dbri（仅 sparc 上）


用于 Sparc 上的 DBRI 声芯片的模块

该模块支持多块声卡

### 模块 snd-wavefront


用于 Turtle Beach Maui、Tropez Tropez+ 声卡的模块

use_cs4232_midi
    使用 CS4232 MPU-401 接口
    （位于你计算机内部无法访问的位置
isapnp
    ISA PnP 检- 0 = 禁用 = 启用（默认）

isapnp=0 时，可使用以下选项

cs4232_pcm_port
    CS4232 PCM 接口的端口号 #
cs4232_pcm_irq
    CS4232 PCM 接口IRQ #,7,9,11,12,15）
cs4232_mpu_port
    CS4232 MPU-401 接口的端口号 #
cs4232_mpu_irq
    CS4232 MPU-401 接口IRQ #,11,12,15）
ics2115_port
    ICS2115 的端口号 #
ics2115_irq
    ICS2115 鐨?IRQ #
fm_port
    FM OPL-3 端口#
dma1
    CS4232 PCM 接口DMA1 #
dma2
    CS4232 PCM 接口DMA2 #

以下wavefront_synth 功能的选项

wf_raw
    假定我们需要引导操作系统（默认：否）；
    若为是，则在驱动加载期间忽略板卡状态，无论如何
    我们都会复位板卡并加载固件
fx_raw
    假定 FX 处理需要帮助（默认：是）；
    若为否，在驱动加载时我们FX 处理器保留为任意状态
    默认会下载微程序及相关的系数，将其设置为“默认”操作，
    无论那意味着什么
debug_default
    用于声卡初始化的调试参数
wait_usecs
    在不睡眠的情况下等待多长时间，单位为微秒（默认：150）；
    基于我有限的实验，这个魔数似乎能给出相当优化的吞吐量
    如果你想尝试并找到更好的值，请随意。记住，要点是得到一
    让我们能尽可能多地忙等待 WaveFront 命令的数字，而不会大
    霸占整个 CPU
    具体来说，使用这个数字，在约 134,000 次状态等待中，只
    250 次导致睡眠
sleep_interval
    等待回复时睡眠多长时间（默认00
sleep_tries
    在一次等待期间尝试睡眠多少次（默认：50
ospath
    经处理的 ICS2115 OS 固件的路径名（默认：wavefront.os）；
    ISC2115 OS 固件的路径名。在最近的版本中，它通过固件加载
    框架处理，因此必须安装在正确的路径中，通常/lib/firmware
reset_time
    等待复位生效多长时间（默认：2
ramcheck_time
    等待 RAM 测试多少秒（默认0
osrun_time
    等待 ICS2115 OS 多少秒（默认0

该模块支持多块声卡和 ISA PnP

注意：早期的版本中固件文`wavefront.os` 位于 /etc。现在它通过
固件加载器加载，必须位于正确的固件路径中，例/lib/firmware
如果在升级内核后遇到有关固件下载的错误，请适当地复制（或建
符号链接）该文件

### 模块 snd-sonicvibes


用于 S3 SonicVibes PCI 声卡的模块
- PINE Schubert 32 PCI

reverb
    混响启用 - 1 = 启用 = 禁用（默认）
    声卡必须带有板载 SRAM 才能使用此功能
mge
    麦克风增益启- 1 = 启用 = 禁用（默认）

该模块支持多块声卡和自动探测

### 模块 snd-serial-u16550


用于 UART16550A 串行 MIDI 端口的模块

port
    UART16550A 芯片的端口号 #
irq
    UART16550A 芯片IRQ #1 = 轮询模式
speed
    速度，单位为波特600,19200,38400,57600,115200
    38400 = 默认
base
    波特率除数基准（57600,115200,230400,460800
    115200 = 默认
outs
    一个串行端口中MIDI 端口数（1-4
    1 = 默认
adaptor
    适配器类型
	0 = Soundcanvas锛? = MS-124T锛? = MS-124W S/A锛。
	3 = MS-124W M/B = 通用

该模块支持多块声卡。该模块不支持自动探测，因此必须指定主端口！！！
其他选项为可选

### 模块 snd-trident


用于 Trident 4DWave DX/NX 声卡的模块
- Best Union  Miss Melody 4DWave PCI
- HIS  4DWave PCI
- Warpspeed  ONSpeed 4DWave PCI
- AzTech  PCI 64-Q3D
- Addonics  SV 750
- CHIC  True Sound 4Dwave
- Shark  Predator4D-PCI
- Jaton  SonicWave 4D
- SiS SI7018 PCI Audio
- Hoontech SoundTrack Digital 4DWave NX

pcm_channels
    PCM 保留的最大通道数（语音数）
wavetable_size
    最大波表大小，kB 为单位（4-kb

该模块支持多块声卡和自动探测

支持电源管理

### 模块 snd-ua101


用于 Edirol UA-101/UA-1000 音频/MIDI 接口的模块

该模块支持多设备、自动探测和热插拔

### 模块 snd-usb-audio


用于 USB 音频USB MIDI 设备的模块

vid
    设备的厂ID（可选）
pid
    设备的产ID（可选）
nrpacks
    每个 URB 的最大包数（默认
device_setup
    设备特定的魔数（可选）
    影响取决于设
    默认值：0x0000
ignore_ctl_error
    忽略任何有关混音器接口的 USB 控制器错误（默认：否
    `ignore_ctl_error=1` 可能在你访问混音器元素（URB error -22
    时遇到错误有所帮助。这发生在某些有缺陷USB 设备或控制器上
    此规避方案也对应 `quirk_flags` 的第 14 位
autoclock
    UAC2 设备启用自动时钟选择（默认：是）
lowlatency
    启用低延迟播放模式（默认：是）
    如果遇到回归问题，可将其关闭以切回旧模式
quirk_alias
    异常别名列表，传入类`0123abcd:5678beef` 的字符串，将设备
    5678:beef 上已有的异常应用到一个新设备 0123:abcd
implicit_fb
    应用通用的隐式反馈同步模式。当此选项被设置且播放流同步模式为
    ASYNC 时，驱动会尝试将一个相邻的 ASYNC 采集流绑定为隐式反馈源
    这等价于 quirk_flags 的第 17 位
use_vmalloc
    使用 vmalloc() 分配 PCM 缓冲区（默认：是）
    对于ARM MIPS 这样具有非一致性内存的架构，mmap 访问使用
    vmalloc 分配的缓冲区可能产生不一致的结果。如果在此类架构上使
    mmap，请关闭此选项，这样会分配并使DMA 一致性缓冲区
delayed_register
    该选项用于具有在多USB 接口中定义的多个流的设备。驱动可
    会多次（每个接口一次）进行注册，这可能导致设备枚举不完整
    该选项接收一个字符串数组，你可以传入类似 `0123abcd:4` 
    ID:INTERFACE 来执行对该给定设备的延迟注册。在此例中，当探测到
    USB 设备 0123:abcd 时，驱动会等USB 接口 4 被探测后才注册
    对此类设备，驱动会打印类“Found post-registration device
    assignment: 1234abcd:04的消息，以便用户注意到这一需要
skip_validation
    跳过单元描述符校验（默认：否）
    该选项用于忽略单元描述符的校验错误（以单元描述符的十六进制转储
    形式），而不是产生驱动探测错误，以便我们检查其细节
quirk_flags
    该选项提供了用于应用异常标志的精细且灵活的控制。它允许为每
    设备指定异常标志，并且可以通过 sysfs 动态修改
    旧的用法接受一个整数数组，其中每个整数按照探测顺序对设备应
    异常标志。例如，`quirk_flags=0x01,0x02` 对第一个设备应
    get_sample_rate，对第二个设备应share_media_device
    新的用法接受格式`VID1:PID1:FLAGS1;VID2:PID2:FLAGS2;...` 
    字符串，其中 `VIDx` `PIDx` 指定设备，`FLAGSx` 指定要应用的
    标志。`VIDx` `PIDx` 4 位十六进制数，可以指定为 `*` 以匹
    任意值。`FLAGSx` 可以是一组以 `|` 分隔的、按名称给出的标志，
    表示位标志的十六进制数。可用的标志名称如下。可以在标志名前
    叹号以对该标志取反
    例如，`1234:abcd:mixer_playback_min_mute|!ignore_ctl_error;**:**:0x01;`
    对设1234:abcd 应用 `mixer_playback_min_mute` 标志并清
    `ignore_ctl_error` 标志，并对所有设备应`skip_sample_rate` 标志

        - 0 位：`get_sample_rate`
          跳过读取设备的采样率
        - 1 位：`share_media_device`
          创建 Media Controller API 条目
        - 2 位：`align_transfer`
          允许在传输时对音频子时隙进行对齐
        - 3 位：`tx_length`
          在传输中添加长度说明
        - 4 位：`playback_first`
          在隐式反馈模式下首先启动播放
        - 5 位：`skip_clock_selector`
          跳过时钟选择器设
        - 6 位：`ignore_clock_source`
          忽略时钟源搜索的错误
        - 7 位：`itf_usb_dsd_dac`
          表示基于 ITF-USB DSD DAC
        - 8 位：`ctl_msg_delay`
          在每个控制消息处理时添加 20ms 延迟
        - 9 位：`ctl_msg_delay_1m`
          在每个控制消息处理时添加 1-2ms 延迟
        - 10 位：`ctl_msg_delay_5m`
          在每个控制消息处理时添加 5-6ms 延迟
        - 11 位：`iface_delay`
          在每个接口设置时添加 50ms 延迟
        - 12 位：`validate_rates`
          在探测时执行采样率校
        - 13 位：`disable_autosuspend`
          禁用运行PM 自动挂起
        - 14 位：`ignore_ctl_error`
          忽略混音器访问的错误
        - 15 位：`dsd_raw`
          支持通用DSD 原始 U32_BE 格式
        - 16 位：`set_iface_first`
          UAC1 一样首先设置接
        - 17 位：`generic_implicit_fb`
          应用通用的隐式反馈同步模
        - 18 位：`skip_implicit_fb`
          不应用隐式反馈同步模
        - 19 位：`iface_skip_close`
          在设置采样率期间不关闭接
        - 20 位：`force_iface_reset`
          在每次停止和重启流时强制复位接口
        - 21 位：`fixed_rate`
          当给定端点只有一个可用速率时，不设PCM 速率（频率）
        - 22 位：`mic_res_16`
          Mic Capture Volume 设置固定分辨16
        - 23 位：`mic_res_384`
          Mic Capture Volume 设置固定分辨384
        - 24 位：`mixer_playback_min_mute`
          将最小音量控制值设为静音，适用于最低播放值表示静音状
          而非最小可听音量的设备
        - 25 位：`mixer_capture_min_mute`
          类似于第 24 位，但用于采集流
        - 26 位：`skip_iface_setup`
          跳过探测时的接口设置（usb_set_interface、init_pitch
          init_sample_rate）；与流打开时的 snd_usb_endpoint_prepare()
          重复
        - 27 位：`mixer_playback_linear_vol`
          为播放音量控制值线性映射到电压（而非 dB）水平的设备设置
          线性音量映射。简而言之：`x(raw) = (raw - raw_min) / (raw_max - raw_min)`
          `V(x) = k ** x`；`dB(x) = 20 ** log10(x)`。覆盖第 24 
        - 28 位：`mixer_capture_linear_vol`
          类似于第 27 位，但用于采集流。覆盖第 25 

该模块支持多设备、自动探测和热插拔

注意：`nrpacks` 参数可以通过 sysfs 动态修改。不要将该值设得超20
通过 sysfs 修改不进行健全性检查

注意：`ignore_ctl_error=1` 只是提供了一种快速绕过问题的方法。如果你
有需要这些异常的有缺陷设备，请向上游报告

注意：`quirk_alias` 选项仅用于测开发。如果你希望获得适当的支持，
请联系上游，在驱动代码中静态添加匹配的异常。`quirk_flags` 同理。如
某设备已知需要特定的规避方案，请向上游报告

### 模块 snd-usb-caiaq


用于 caiaq USB 音频接口的模块：

- Native Instruments RigKontrol2
- Native Instruments Kore Controller
- Native Instruments Audio Kontrol 1
- Native Instruments Audio 8 DJ

该模块支持多设备、自动探测和热插拔

### 模块 snd-usb-usx2y


用于 Tascam USB US-122、US-224 US-428 设备的模块

该模块支持多设备、自动探测和热插拔

注意：你需要通过 alsa-tools alsa-firmware 软件包中包含
`usx2yloader` 工具加载固件

### 模块 snd-via82xx


用于基于 VIA 82C686A/686B233233A233C235237
（南桥）AC'97 主板的模块

mpu_port
    0x300,0x310,0x320,0x330，否则从 BIOS 设置获取
    [浠?VIA686A/686B]
joystick
    启用游戏杆（默认关闭）[VIA686A/686B]
ac97_clock
    AC'97 编解码器时钟基准（默48000Hz
dxs_support
    支持 DXS 通道 = 自动（默认） = 启用 = 禁用
    3 = 48k = VRA = 启用任意采样率且不同通道使用
    不同的采样率 [VIA8233/C235237]
ac97_quirk
    针对异常硬件AC'97 规避方案
    见下面的 `AC97 Quirk Option`_ 小节

该模块支持单芯片和自动探测

注意：在某些 SMP 主板（如 MSI 694D）上，中断可能无法正确生成
在这种情况下，请尝试BIOS 上的 SMP（或 MPS）版本设1.1 而不
默认1.4。这样中断号将被分配15 以下。你也可以升级你BIOS

注意：VIA8233/5/7（非 VIA8233A）可以将 DXS（direct sound）通道支持
第一PCM。在这些通道上，最多可同时播放 4 个流，且控制器可以对每个
通道以独立的速率执行采样率转换
默认情况下（`dxs_support = 0`），除已知设备外，选择固定48k 速率
因为在某些主板上，由BIOS 缺陷，除 48k 外输出往往有杂音
请先尝试一`dxs_support=5`，如果它在其他采样率（例mp3 播放
44.1kHz）下工作，请PCI 子系统厂设备 ID（即 `lspci -nv` 的输出）
告诉我们
如果 `dxs_support=5` 不工作，尝试 `dxs_support=4`；如果还不工作，尝试
dxs_support=1。（dxs_support=1 通常用于旧主板。正确实现的板卡应该
能在 4 5 下工作。）如果仍然不工作，而默认设置可以，`dxs_support=3`
是正确选择。如果默认设置根本不工作，尝`dxs_support=2` 来禁DXS 通道
在任何情况下，请将结果和子系统厂设备 ID 告诉我们。见下面
`Links and Addresses`_銆。

注意：对VIA823x 上的 MPU401，请另外使用 snd-mpu401 驱动。mpu_port
选项仅用VIA686 芯片

支持电源管理

### 模块 snd-via82xx-modem


用于 VIA82xx AC97 调制器的模块

ac97_clock
    AC'97 编解码器时钟基准（默48000Hz

该模块支持单块声卡和自动探测

注意：该模块的默index 值为 -2，即第一个槽位被排除

支持电源管理

### 模块 snd-virmidi


用于虚拟 rawmidi 设备的模块
该模块创建与相应 ALSA 音序器端口通信的虚rawmidi 设备

midi_devs
    MIDI 设备#-4，默4

该模块支持多块声卡

### 模块 snd-virtuoso


用于基于 Asus AV66/AV100/AV200 芯片的声卡的模块，即 Xonar D1、DX
D2、D2X、DS、DSX、Essence ST（Deluxe）、Essence STX（II）、HDAV1.3
（Deluxe）和 HDAV1.3 Slim

该模块支持自动探测和多块声卡

### 模块 snd-vx222


用于 Digigram VX-Pocket VX222、V222 v2 Mic 声卡的模块

mic
    V222 Mic 上启用麦克风（NYI
ibl
    采集 IBL 大小。（默认 = 0，最小大小）

该模块支持多块声卡

当驱动编译为模块且支hotplug 固件时，固件数据会通过 hotplug 自动
加载。请alsa-firmware 软件包中安装所需的固件文件。当没有可用
hotplug 固件加载器时，你需要通过 alsa-tools 软件包中vxloader 工具
加载固件。要自动调用 vxloader，请将以下内容添加到
/etc/modprobe.d/alsa.conf锛。

```

  install snd-vx222 /sbin/modprobe --first-time -i snd-vx222\
    && /usr/bin/vxloader


```
（对2.2/2.4 内核，改为将 `post-install /usr/bin/vxloader` 添加
/etc/modules.conf。）
IBL 大小定义PCM 的中断周期。更小的大小带来更低的延迟，但也会导
更多CPU 消耗。该大小通常对齐126。默认（=0）时选择最小的大小
可能IBL 值可以在 /proc/asound/cardX/vx-status proc 文件中找到

支持电源管理


### 模块 snd-vxpocket


用于 Digigram VX-Pocket VX2 440 PCMCIA 声卡的模块

ibl
    采集 IBL 大小。（默认 = 0，最小大小）

该模块支持多块声卡。该模块仅在设置PCMCIA 支持的内核中才被编译

在较旧的 2.6.x 内核上，要通过卡管理器激活驱动，你需要设
/etc/pcmcia/vxpocket.conf。参sound/pcmcia/vx/vxpocket.c.6.13 
更新的内核不再需要配置文件

当驱动编译为模块且支hotplug 固件时，固件数据会通过 hotplug 自动
加载。请alsa-firmware 软件包中安装所需的固件文件。当没有可用
hotplug 固件加载器时，你需要通过 alsa-tools 软件包中vxloader 工具
加载固件

关于采集 IBL，请参见 snd-vx222 模块的描述

注意：自 ALSA 1.0.10 起，snd-vxp440 驱动已合并到 snd-vxpocket 驱动中

支持电源管理

### 模块 snd-ymfpci


用于 Yamaha PCI 芯片（YMF72x、YMF74x YMF75x）的模块

mpu_port
    0x300,0x330,0x332,0x334，默0（禁用）
    1（仅 YMF744/754 自动探测
fm_port
    0x388,0x398,0x3a0,0x3a8，默0（禁用）
    1（仅 YMF744/754 自动探测
joystick_port
    0x201,0x202,0x204,0x205，默0（禁用）
    1（自动探测）
rear_switch
    启用共享的后线路输入开关（bool

该模块支持自动探测和多芯片

支持电源管理

### 模块 snd-pdaudiocf


用于 Sound Core PDAudioCF 声卡的模块

支持电源管理


## AC97 硬件异常规避选项


ac97_quirk 选项用于为板AC'97 控制器（snd-intel8x0）驱动上
特定设备启用/覆盖规避方案。某些硬件把 Master Headphone Surround
之间的输出引脚接反了（这要归功于 AC'97 规范在各个版本之间的混乱 :-

驱动提供了对已知问题设备的自动探测，但有些可能未知或被错误探测
在这种情况下，请通过此选项传入正确的值

接受以下字符串：

default
    不覆盖默认设
none
    禁用异常规避
hp_only
    Master Headphone 控制绑定为单一控制
swap_hp
    交换耳机和主控制
swap_surround
    交换主和环绕控制
ad_sharing
    对于 AD1985，开OMS 位并使用耳机
alc_jack
    对于 ALC65x，开启插孔检测模
inv_eapd
    反转EAPD 实现
mute_led
    绑定 EAPD 位以开关闭静音 LED

为了向后兼容，相应的整数-1 等也被接受

例如，如`Master` 音量控制对你的设备无效，而只`Headphone` 有效
请传ac97_quirk=hp_only 模块选项


## 閰嶇疆闈?ISAPNP 澹板崱


当内核配置了 ISA-PnP 支持时，支持 isapnp 声卡的模块会`isapnp`
模块选项。如果设置了此选项，将***探测 ISA-PnP 设备。要探测
ISA-PnP 声卡，你必须传入 `isapnp=0` 选项以及正确I/O irq 配置

当内核未配置 ISA-PnP 支持时，isapnp 选项将不会被编译进去


## 模块自动加载支持


ALSA 驱动可以通过定义模块别名按需自动加载。对ALSA 原生设备，会请求
字符`snd-card-%1`，其`%i` 是从 0 7 的声卡号

要为 OSS 服务自动加载 ALSA 驱动，请定义字符`sound-slot-%i`，其
`%i` 表示 OSS 的槽位号，它对应 ALSA 的声卡索引。通常，将其定义为
同一声卡模块

单块 emu10k1 声卡的示例配置如下：
```

    ----- /etc/modprobe.d/alsa.conf
    alias snd-card-0 snd-emu10k1
    alias sound-slot-0 snd-emu10k1
    ----- /etc/modprobe.d/alsa.conf

```
可自动加载的声卡数量取决snd 模块`cards_limit` 模块选项。默
设为 1。要启用多块声卡的自动加载，请在该选项中指定声卡数量

当有多块声卡可用时，最好也通过模块选项为每块声卡指index 号，以便
声卡的顺序保持一致

两块声卡的示例配置如下：
```

    ----- /etc/modprobe.d/alsa.conf
    # ALSA 部分
    options snd cards_limit=2
    alias snd-card-0 snd-interwave
    alias snd-card-1 snd-ens1371
    options snd-interwave index=0
    options snd-ens1371 index=1
    # OSS/Free 部分
    alias sound-slot-0 snd-interwave
    alias sound-slot-1 snd-ens1371
    ----- /etc/modprobe.d/alsa.conf

```
在此例中，interwave 声卡始终作为第一块声卡（index 0）加载，ens1371
作为第二块（index 1）

另一种（较新的）固定槽位分配的方法是使用 snd 模块`slots` 选项
对于上面的例子，按如下方式指定：
```

    options snd slots=snd-interwave,snd-ens1371

```
这样，第一个槽位（#0）保留给 snd-interwave 驱动，第二个1）保留给
snd-ens1371。如果使slots 选项，可以省略每个驱动中index 选项
（不过只要不冲突，也可以同时保留）

slots 选项对于避免可能的热插拔及其导致的槽位冲突特别有用。例如，再次
考虑上面的例子，前两个槽位已被保留。如果有任何其他驱动（例
snd-usb-audio）在 snd-interwave snd-ens1371 之前加载，它将被分配
第三个或更后的槽位

当模块名'!' 给出时，该槽位将保留给除该名称外的任何模块。例如，
`slots=!snd-pcsp` 将把第一个槽位保留给snd-pcsp 外的任何模块


## ALSA PCM 设备OSS 设备的映


```

    /dev/snd/pcmC0D0[c|p]  -> /dev/audio0 (/dev/audio) -> minor 4
    /dev/snd/pcmC0D0[c|p]  -> /dev/dsp0 (/dev/dsp)     -> minor 3
    /dev/snd/pcmC0D1[c|p]  -> /dev/adsp0 (/dev/adsp)   -> minor 12
    /dev/snd/pcmC1D0[c|p]  -> /dev/audio1              -> minor 4+16 = 20
    /dev/snd/pcmC1D0[c|p]  -> /dev/dsp1                -> minor 3+16 = 19
    /dev/snd/pcmC1D1[c|p]  -> /dev/adsp1               -> minor 12+16 = 28
    /dev/snd/pcmC2D0[c|p]  -> /dev/audio2              -> minor 4+32 = 36
    /dev/snd/pcmC2D0[c|p]  -> /dev/dsp2                -> minor 3+32 = 39
    /dev/snd/pcmC2D1[c|p]  -> /dev/adsp2               -> minor 12+32 = 44

```
`/dev/snd/pcmC{X}D{Y}[c|p]` 表达式中的第一个数字表示声卡号，第二个
表示设备号。ALSA 设备带有 `c` `p` 后缀，分别表示方向：采集和播放

请注意，上述设备映射可能会通过 snd-pcm-oss 模块的模块选项而改变


## Proc 接口proc/asound


### /proc/asound/card#/pcm#[cp]/oss


erase
    擦除关于 OSS 应用程序的所有附加信

<app_name> <fragments> <fragment_size> [<options>]
    <app_name>
	带（较高优先级）或不带路径的应用程序名称
    <fragments>
	 分片数，自动则为 0
    <fragment_size>
	 分片大小（字节），自动则0
    <options>
	可选参

	disable
	    应用程序尝试为此通道打开一pcm 设备，但不想使用它
	    （因 bug 或需mmap
	    这对 Quake 等程序很有用…
	direct
	    不使用插
	block
	    强制块模式（rvplayer
	non-block
	    强制非块模式
	whole-frag
	    只写入整个分片（仅影响播放的优化
	no-silence
	    不预先填充静音以避免咔嗒
	buggy-ptr
	    GETOPTR ioctl 中返回空白块而不是已填充的块

示例
```

    echo "x11amp 128 16384" > /proc/asound/card0/pcm0p/oss
    echo "squake 0 0 disable" > /proc/asound/card0/pcm0c/oss
    echo "rvplayer 0 0 block" > /proc/asound/card0/pcm0p/oss


```
## 早期缓冲区分


某些驱动（例hdsp）需要大的连续缓冲区，而由于内存碎片，有时
驱动模块实际加载时再寻找这样的空间已经太迟。你可以通过提前加载
snd-page-alloc 模块并向proc 文件写入命令来预分配 PCM 缓冲区，
例如在早期启动阶段（`/etc/init.d/*.local` 脚本）进行

读取 proc 文件 /proc/drivers/snd-page-alloc 会显示当前页面分配的
使用情况。在写入时，你可以向 snd-page-alloc 驱动发送以下命令：

- add VENDOR DEVICE MASK SIZE BUFFERS

VENDOR DEVICE PCI 厂商和设ID。它们取整数（十六进制需
0x 前缀）。MASK PCI DMA 掩码。如果不限制则传 0。SIZE 是要分配
每个缓冲区的大小。你可以KB MB 使用 k m 后缀。最大数量为 16MB
BUFFERS 是要分配的缓冲区数量。它必须大于 0。最大数量为 4

- erase

这将擦除所有未在使用中的预分配缓冲区


## 链接与地址


ALSA 项目主页
    http://www.alsa-project.org
Kernel Bugzilla
    http://bugzilla.kernel.org/
ALSA 开发者邮件列
    mailto:alsa-devel@alsa-project.org
alsa-info.sh 脚本
    https://www.alsa-project.org/alsa-info.sh
