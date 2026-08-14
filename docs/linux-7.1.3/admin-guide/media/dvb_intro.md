
## 使用数字电视框架


#### 简介


数字电视（Digital TV）与模拟电视（Analogue TV）之间一个重大区别——不谨慎的人（比如我自己）应当考虑到——是：尽管 DVB-T 卡的组成结构与模拟电视卡大体相似，但它们的运作方式却大不相同。

模拟电视的用途是接收并显示模拟电视信号。模拟电视信号（又称复合视频）是以交错（interlacing）技术光栅化的图像帧序列（欧洲为每秒 25 帧）的模拟编码。交错技术用两个场来表示一帧。因此，PC 用的模拟电视卡有如下用途：

- 调谐接收器以接收广播信号
- 解调广播信号
- 解复用模拟视频信号与模拟音频信号。

```
     some countries employ a digital audio signal
     embedded within the modulated composite analogue signal -
     using NICAM signaling.)
```

- 将模拟视频信号数字化，并使生成的数据流可供数据总线使用。

模拟电视卡产生的数字数据流由卡上的电路生成，通常以未压缩形式呈现。对于以 768x576 分辨率、24 位彩色像素、每秒 25 帧编码的 PAL 电视信号，会产生相当大量的数据，必须先由 PC 处理才能在视频监视器屏幕上显示。一些 PC 用的模拟电视卡带有板载 MPEG2 编码器，可使原始数字数据流以编码且压缩的形式呈现给 PC——类似于数字电视中使用的形式。

一块简单的入门级数字电视卡（DVB-T、C 或 S）的用途很简单：

- 调谐接收器以接收广播信号。* 从广播信号中提取编码后的数字数据流。
- 使编码后的数字数据流（MPEG2）可供数据总线使用。

两者的重大区别在于：模拟电视卡上的调谐器输出模拟信号，而数字电视卡上的调谐器输出压缩编码后的数字数据流。由于信号已经数字化，只需极少的额外处理即可将此数据流传给 PC 数据总线，然后提取数字视频与音频数据流，传递给相应的软件或硬件进行解码与观看。

#### 让网卡工作起来


Linux 下 DVB 的设备驱动 API 会通过 devfs 文件系统创建以下设备节点：

- /dev/dvb/adapter0/demux0
- /dev/dvb/adapter0/dvr0
- /dev/dvb/adapter0/frontend0

`/dev/dvb/adapter0/dvr0` 设备节点用于读取 MPEG2 数据流，`/dev/dvb/adapter0/frontend0` 设备节点用于调谐前端调谐器模块。`/dev/dvb/adapter0/demux0` 用于控制将接收哪些节目。

根据网卡的功能集，设备驱动 API 还可能暴露其他设备节点：

- /dev/dvb/adapter0/ca0
- /dev/dvb/adapter0/audio0
- /dev/dvb/adapter0/net0
- /dev/dvb/adapter0/osd0
- /dev/dvb/adapter0/video0

`/dev/dvb/adapter0/ca0` 用于解码加密频道。其他设备节点仅存在于使用 av7110 驱动的设备上，该驱动（及其所用额外 API）现已被废弃。

#### 接收数字电视频道


本节试图解释其工作原理，以及它如何影响数字电视卡的配置。

在此例中，我们考虑调谐到澳大利亚墨尔本地区（Melbourne）的 DVB-T 频道。

目前 Mount Dandenong 发射台广播的频率如下：

表 1. 发射机频率（Mount Dandenong，维多利亚州，澳大利亚）

===========	===========
广播机构	频率
===========	===========
Seven		177.500 Mhz
SBS		184.500 Mhz
Nine		191.625 Mhz
Ten		219.500 Mhz
ABC		226.500 Mhz
Channel 31	557.625 Mhz
===========	===========

数字电视扫描工具（如 dvbv5-scan）内置了一套针对不同国家与地区的编译默认配置。这些配置目前作为一个独立的软件包提供，名为 dtv-scan-tables。其 git 仓库位于 LinuxTV.org：

    https://git.linuxtv.org/dtv-scan-tables.git/

如果那里的表都不适合，你可以在命令行指定一个包含转发器频率的数据文件。以下是上述频道转发器的一个示例文件，采用旧的 "channel" 格式

```
	# Data file for DVB scan program
	#
	# C Frequency SymbolRate FEC QAM
	# S Frequency Polarisation SymbolRate FEC
	# T Frequency Bandwidth FEC FEC2 QAM Mode Guard Hier

	T 177500000 7MHz AUTO AUTO QAM64 8k 1/16 NONE
	T 184500000 7MHz AUTO AUTO QAM64 8k 1/8 NONE
	T 191625000 7MHz AUTO AUTO QAM64 8k 1/16 NONE
	T 219500000 7MHz AUTO AUTO QAM64 8k 1/16 NONE
	T 226500000 7MHz AUTO AUTO QAM64 8k 1/16 NONE
	T 557625000 7MHz AUTO AUTO QPSK 8k 1/16 NONE
```

如今我们更倾向于使用一种更新的格式，它更详尽也更易理解。采用新格式时，"Seven" 频道转发器的配置如下

```
	[Seven]
		DELIVERY_SYSTEM = DVBT
		FREQUENCY = 177500000
		BANDWIDTH_HZ = 7000000
		CODE_RATE_HP = AUTO
		CODE_RATE_LP = AUTO
		MODULATION = QAM/64
		TRANSMISSION_MODE = 8K
		GUARD_INTERVAL = 1/16
		HIERARCHY = NONE
		INVERSION = AUTO
```

有关完整表格的更新版本，请参见：

    https://git.linuxtv.org/dtv-scan-tables.git/tree/dvb-t/au-Melbourne

当数字电视扫描工具运行时，它会输出一个文件，其中包含网卡前端能够锁定的每个频道转发器上存在的所有音频与视频节目信息（即任何在天线处信号足够强的节目）。

以下是从一次频道扫描得到的 dvbv5 工具输出（取自

```
    [ABC HDTV]
	    SERVICE_ID = 560
	    VIDEO_PID = 2307
	    AUDIO_PID = 0
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [ABC TV Melbourne]
	    SERVICE_ID = 561
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [ABC TV 2]
	    SERVICE_ID = 562
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [ABC TV 3]
	    SERVICE_ID = 563
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [ABC TV 4]
	    SERVICE_ID = 564
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [ABC DiG Radio]
	    SERVICE_ID = 566
	    VIDEO_PID = 0
	    AUDIO_PID = 2311
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 226500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 3/4
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital]
	    SERVICE_ID = 1585
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital 1]
	    SERVICE_ID = 1586
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital 2]
	    SERVICE_ID = 1587
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital 3]
	    SERVICE_ID = 1588
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital]
	    SERVICE_ID = 1589
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital 4]
	    SERVICE_ID = 1590
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital]
	    SERVICE_ID = 1591
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN HD]
	    SERVICE_ID = 1592
	    VIDEO_PID = 514
	    AUDIO_PID = 0
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [TEN Digital]
	    SERVICE_ID = 1593
	    VIDEO_PID = 512
	    AUDIO_PID = 650
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 219500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [Nine Digital]
	    SERVICE_ID = 1072
	    VIDEO_PID = 513
	    AUDIO_PID = 660
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 191625000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [Nine Digital HD]
	    SERVICE_ID = 1073
	    VIDEO_PID = 512
	    AUDIO_PID = 0
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 191625000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [Nine Guide]
	    SERVICE_ID = 1074
	    VIDEO_PID = 514
	    AUDIO_PID = 670
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 191625000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 3/4
	    CODE_RATE_LP = 1/2
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/16
	    HIERARCHY = NONE

    [7 Digital]
	    SERVICE_ID = 1328
	    VIDEO_PID = 769
	    AUDIO_PID = 770
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [7 Digital 1]
	    SERVICE_ID = 1329
	    VIDEO_PID = 769
	    AUDIO_PID = 770
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [7 Digital 2]
	    SERVICE_ID = 1330
	    VIDEO_PID = 769
	    AUDIO_PID = 770
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [7 Digital 3]
	    SERVICE_ID = 1331
	    VIDEO_PID = 769
	    AUDIO_PID = 770
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [7 HD Digital]
	    SERVICE_ID = 1332
	    VIDEO_PID = 833
	    AUDIO_PID = 834
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [7 Program Guide]
	    SERVICE_ID = 1334
	    VIDEO_PID = 865
	    AUDIO_PID = 866
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 177500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS HD]
	    SERVICE_ID = 784
	    VIDEO_PID = 102
	    AUDIO_PID = 103
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS DIGITAL 1]
	    SERVICE_ID = 785
	    VIDEO_PID = 161
	    AUDIO_PID = 81
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS DIGITAL 2]
	    SERVICE_ID = 786
	    VIDEO_PID = 162
	    AUDIO_PID = 83
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS EPG]
	    SERVICE_ID = 787
	    VIDEO_PID = 163
	    AUDIO_PID = 85
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS RADIO 1]
	    SERVICE_ID = 798
	    VIDEO_PID = 0
	    AUDIO_PID = 201
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE

    [SBS RADIO 2]
	    SERVICE_ID = 799
	    VIDEO_PID = 0
	    AUDIO_PID = 202
	    DELIVERY_SYSTEM = DVBT
	    FREQUENCY = 536500000
	    INVERSION = OFF
	    BANDWIDTH_HZ = 7000000
	    CODE_RATE_HP = 2/3
	    CODE_RATE_LP = 2/3
	    MODULATION = QAM/64
	    TRANSMISSION_MODE = 8K
	    GUARD_INTERVAL = 1/8
	    HIERARCHY = NONE
```
