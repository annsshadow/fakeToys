
## vidtv：虚拟数字电视驱


作者：Daniel W. S. Almeida <dwlsalmeida@gmail.com>020 6 月

### 背景


Vidtv 是一个虚DVB 驱动，旨在作为驱动开发者的参考模板。它还用于验证现有的媒体 DVB API，从而帮助上层应用程序的开发者

目前，它由以下部分组成：

- 一个虚拟调谐器（tuner）驱动，如果所选频率距离某个特定传输系统的有效频率表过远，它会报告较差的信号质量

- 一个虚拟解调器（demod）驱动，它会持续轮询调谐器返回的虚拟信号质量，模拟一个可以根CNR 水平丢失/重新获取信号锁定的设备

- 一个虚拟桥接（bridge）驱动，它负modprobe 加载虚拟调谐器和解调器模块，并实现解复用（demux）逻辑。该模块在初始化时接收参数，这些参数将决定模拟的行为

- 负责编码一个有MPEG 传输流（Transport Stream）的代码，该流随后被传递给桥接驱动。这个虚拟流包含一些硬编码内容。目前，我们有一个单独的、仅含音频的频道，其中包含一MPEG 基本流（Elementary Stream），它又包含一SMPTE 302m 编码的正弦波。请注意，选择这个特定的编码器是因为它是在 MPEG 传输流中编码 PCM 音频数据最简单的方式


### 构建 vidtv


vidtv 是一个测试驱动，因此在编译内核时**默认*启用

为了启用 vidtv 的编译：

- 启用 **DVB_TEST_DRIVERS**，然
- 启用 **DVB_VIDTV**

当编译为模块时，预期会生成以.ko 文件

- dvb_vidtv_tuner.ko

- dvb_vidtv_demod.ko

- dvb_vidtv_bridge.ko


### 运行 vidtv


```
	modprobe vidtv
```
就是这样！桥接驱动会在它自身的初始化过程中初始化调谐器和解调器驱动

默认情况下，它将接受以下频率

 - 474 MHz，对DVB-T/T2/C
 - 11,362 GHz，对DVB-S/S2

对于卫星系统，该驱动模拟一个通用的扩展型 LNBf，其频率位于 Ku 波段，范围从 10.7 GHz 12.75 GHz

你可以选择性地vidtv 定义一些命令行参数


### vidtv 的命令行参数


以下是可以提供给 vidtv 的所有参数列表：

drop_tslock_prob_on_low_snr
	当信号质量差时丢TS 锁定的概率
	这个概率会被虚拟解调器驱动使用，以便在信号质量不好时
	最终返回一0 状态

recover_tslock_prob_on_good_snr:
	当信号改善时恢复 TS 锁定的概率。这
	概率会被虚拟解调器驱动使用，以便在信号质量改善时/若改善时
	最终返回一0x1f 状态

mock_power_up_delay_msec
	模拟上电延迟。默认值：0

mock_tune_delay_msec
	模拟调谐延迟。默认0

vidtv_valid_dvb_t_freqs
	要模拟的有效 DVB-T 频率，单位为 Hz

vidtv_valid_dvb_c_freqs
	要模拟的有效 DVB-C 频率，单位为 Hz

vidtv_valid_dvb_s_freqs
	要模拟的位于 Ku 波段的有DVB-S/S2 频率，单位为 kHz

max_frequency_shift_hz,
	调谐到某个频道时允许的最大偏移量，单位为 Hz

si_period_msec
	发SI 包的频率。默认值：40ms

pcr_period_msec
	发PCR 包的频率。默认值：40ms

mux_rate_kbytes_sec
	如有必要，通过插入 TS 空包来维持该比特率。默认值：4096

pcr_pid,
	所有频道的 PCR PID。默认值：0x200

mux_buf_sz_pkts,
	复用缓冲区大小，188 字节为单位

### vidtv 内部结构


内核模块按以下方式拆分：

vidtv_tuner.[ch]
	实现一个虚拟调谐器 DVB 驱动

vidtv_demod.[ch]
	实现一个虚拟解调器 DVB 驱动

vidtv_bridge.[ch]
	实现一个桥接驱动

MPEG 相关的代码按以下方式拆分

vidtv_ts.[ch]
	处理 MPEG TS 包的代码，例TS 头、适配字段
	PCR 包和 NULL 包

vidtv_psi.[ch]
	这是 PSI 生成器。PSI 包包含关MPEG 传输流的
	一般信息。需要一PSI 生成器，这样上层应用才能
	获取关于传输流的信息，并最终调谐到一个（虚拟）频道

	由于该生成器实现在一个单独的文件中，它可以在媒体子系统的其他地方被复用

	目前 vidtv 支持处理 5 PSI 表：PAT、PMT
	SDT、NIT EIT

	PAT PMT 的规范可参见 *ISO 13818-1:
	Systems**，SDT、NIT、EIT 的规范可参见 **ETSI
	EN 300 468: Specification for Service Information (SI) in DVB
	systems*銆。

	这并非严格必要，但在调试 PSI 表时使用一个真实的 TS 文件会很有帮助。Vidtv 目前尝试复制此文件中PSI 结构：`TS1Globo.ts
	<https://tsduck.io/streams/brazil-isdb-tb/TS1globo.ts>`_銆。

	一种可视化流结构的好方法是使用
	`DVBInspector <https://sourceforge.net/projects/dvbinspector/>`_銆。

vidtv_pes.[ch]
	实现 PES 逻辑，将编码器数据转换为 MPEG TS 包
	这些包随后可以被送入 TS 复用器，并最终进入用户空间

vidtv_encoder.h
	vidtv 编码器的接口。可以通过实现此文件中的调用来向该驱动添加新的编码器

vidtv_s302m.[ch]
	实现一S302M 编码器，以便PCM 音频数据插入生成
	MPEG 传输流中。相关规范可在线获取，名*SMPTE 302M-2007:
	Television - Mapping of AES3 Data into MPEG-2 Transport Stream*銆。


	生成结果 MPEG 基本流通过附带一S302M 注册描述符的私有流传送

	这样就可以将音频信号传入用户空间，从而被媒体软件解码和播放。ffmpeg 中对应的解码器位'libavcodec/s302m.c'，目前仍是实验性的

vidtv_channel.[ch]
	实现一个“频道（channel）”抽象

	vidtv 启动时，它会创建一些硬编码的频道：

	#. 它们的服务会被拼接起来以填充 SDT

	#. 它们的节目会被拼接起来以填充 PAT

	#. 它们的事件会被拼接起来以填充 EIT

	#. 对于 PAT 中的每个节目，都会创建一PMT 段

	#. 某个频道PMT 段会被分配它的流

	#. 每个流都会在其对应的编码器上被循环轮询以产生 TS 包
	   这些包可能被复用器交错，然后传递给桥接驱动

vidtv_mux.[ch]
	实现一MPEG TS 复用器，大致基于 ffmpeg 
	"libavcodec/mpegtsenc.c" 中的实现

	复用器运行一个循环，负责

	#. 跟踪自上次迭代以来经过的时间量

	#. 轮询编码器以获取“elapsed_time”大小的数据

	#. 如有需要，插入 PSI PCR 包

	#. 如有必要，用 NULL 包填充结果流，以维持所选的比特率

	#. 将结TS 包传递给桥接驱动，以便它能将它们传给解复用器


### 使用 v4l-utils 测试 vidtv


使用 v4l-utils 中的工具是测试和检vidtv 输出的好方法。它托管在这里：`v4l-utils Documentation
<https://linuxtv.org/wiki/index.php/V4l-utils>`_銆。

```
	The v4l-utils are a series of packages for handling media devices.

	It is hosted at http://git.linuxtv.org/v4l-utils.git, and packaged
	on most distributions.

	It provides a series of libraries and utilities to be used to
	control several aspect of the media boards.
```
```
	modprobe dvb_vidtv_bridge
```
如果驱动正常，它应当会被加载，并且它的探测代码会运行。这会将调谐器和解调器驱动一并拉入


#### 使用 dvb-fe-tool


```
	$ dvb-fe-tool
	Device Dummy demod for DVB-T/T2/C/S/S2 (/dev/dvb/adapter0/frontend0) capabilities:
	    CAN_FEC_1_2
	    CAN_FEC_2_3
	    CAN_FEC_3_4
	    CAN_FEC_4_5
	    CAN_FEC_5_6
	    CAN_FEC_6_7
	    CAN_FEC_7_8
	    CAN_FEC_8_9
	    CAN_FEC_AUTO
	    CAN_GUARD_INTERVAL_AUTO
	    CAN_HIERARCHY_AUTO
	    CAN_INVERSION_AUTO
	    CAN_QAM_16
	    CAN_QAM_32
	    CAN_QAM_64
	    CAN_QAM_128
	    CAN_QAM_256
	    CAN_QAM_AUTO
	    CAN_QPSK
	    CAN_TRANSMISSION_MODE_AUTO
	DVB API Version 5.11, Current v5 delivery system: DVBC/ANNEX_A
	Supported delivery systems:
	    DVBT
	    DVBT2
	    [DVBC/ANNEX_A]
	    DVBS
	    DVBS2
	Frequency range for the current standard:
	From:            51.0 MHz
	To:              2.15 GHz
	Step:            62.5 kHz
	Tolerance:       29.5 MHz
	Symbol rate ranges for the current standard:
	From:            1.00 MBauds
	To:              45.0 MBauds
```
```
	static const struct dvb_frontend_ops vidtv_demod_ops = {
		.delsys = {
			SYS_DVBT,
			SYS_DVBT2,
			SYS_DVBC_ANNEX_A,
			SYS_DVBS,
			SYS_DVBS2,
		},

		.info = {
			.name                   = "Dummy demod for DVB-T/T2/C/S/S2",
			.frequency_min_hz       = 51 * MHz,
			.frequency_max_hz       = 2150 * MHz,
			.frequency_stepsize_hz  = 62500,
			.frequency_tolerance_hz = 29500 * kHz,
			.symbol_rate_min        = 1000000,
			.symbol_rate_max        = 45000000,

			.caps = FE_CAN_FEC_1_2 |
				FE_CAN_FEC_2_3 |
				FE_CAN_FEC_3_4 |
				FE_CAN_FEC_4_5 |
				FE_CAN_FEC_5_6 |
				FE_CAN_FEC_6_7 |
				FE_CAN_FEC_7_8 |
				FE_CAN_FEC_8_9 |
				FE_CAN_QAM_16 |
				FE_CAN_QAM_64 |
				FE_CAN_QAM_32 |
				FE_CAN_QAM_128 |
				FE_CAN_QAM_256 |
				FE_CAN_QAM_AUTO |
				FE_CAN_QPSK |
				FE_CAN_FEC_AUTO |
				FE_CAN_INVERSION_AUTO |
				FE_CAN_TRANSMISSION_MODE_AUTO |
				FE_CAN_GUARD_INTERVAL_AUTO |
				FE_CAN_HIERARCHY_AUTO,
		}

		....

```
有关 dvb-fe-tools 的更多信息，请查看其在线文档
`dvb-fe-tool Documentation
<https://www.linuxtv.org/wiki/index.php/Dvb-fe-tool>`_銆。


#### 使用 dvb-scan


为了调谐到某个频道并读取 PSI 表，我们可以使用 dvb-scan

为此，需要提供一份称为“扫描文件（scan file）”的配置文件
```
	[Channel]
	FREQUENCY = 474000000
	MODULATION = QAM/AUTO
	SYMBOL_RATE = 6940000
	INNER_FEC = AUTO
	DELIVERY_SYSTEM = DVBC/ANNEX_A
```
	参数取决于你所测试的电视标准

	Vidtv 是一个虚拟驱动，不会对扫描文件中的大部分信息进行验证。对DVB-T/DVB-T2，只需指定 'FREQUENCY' 'DELIVERY_SYSTEM' 就足够了。不过对DVB-S/DVB-C，你还应当提'SYMBOL_RATE'

你可以在线浏览扫描表：`dvb-scan-tables
<https://git.linuxtv.org/dtv-scan-tables.git>`_銆。

```
	$ dvbv5-scan channel.conf
	dvbv5-scan ~/vidtv.conf
	ERROR    command BANDWIDTH_HZ (5) not found during retrieve
	Cannot calc frequency shift. Either bandwidth/symbol-rate is unavailable (yet).
	Scanning frequency #1 330000000
	    (0x00) Signal= -68.00dBm
	Scanning frequency #2 474000000
	Lock   (0x1f) Signal= -34.45dBm C/N= 33.74dB UCB= 0
	Service Beethoven, provider LinuxTV.org: digital television
```
有关 dvb-scan 的更多信息，请查看其在线文档
`dvb-scan Documentation <https://www.linuxtv.org/wiki/index.php/Dvbscan>`_銆。


#### 使用 dvb-zap


dvbv5-zap 是一个命令行工具，可用于MPEG-TS 录制到磁盘。典型用法是调谐到某个频道并将其置于录制模式。示
```
	$ dvbv5-zap -c dvb_channel.conf "beethoven" -o music.ts -P -t 10
	using demux 'dvb0.demux0'
	reading channels from file 'dvb_channel.conf'
	tuning to 474000000 Hz
	pass all PID's to TS
	dvb_set_pesfilter 8192
	dvb_dev_set_bufsize: buffer set to 6160384
	Lock   (0x1f) Quality= Good Signal= -34.66dBm C/N= 33.41dB UCB= 0 postBER= 0 preBER= 1.05x10^-3 PER= 0
	Lock   (0x1f) Quality= Good Signal= -34.57dBm C/N= 33.46dB UCB= 0 postBER= 0 preBER= 1.05x10^-3 PER= 0
	Record to file 'music.ts' started
	received 24587768 bytes (2401 Kbytes/sec)
	Lock   (0x1f) Quality= Good Signal= -34.42dBm C/N= 33.89dB UCB= 0 postBER= 0 preBER= 2.44x10^-3 PER= 0
```
       针对 music.ts 文件


可以通过使用能识MPEG-TS 格式的播放器（例`mplayer` `vlc`）播放流的内容来观看该频道

通过播放流的内容，可以直观地检
```
	$ mplayer music.ts
```
```
	$ dvbv5-zap -c dvb_channel.conf "beethoven" -P -r &
```
```
	$ mplayer /dev/dvb/adapter0/dvr0
```
有关 dvb-zap 的更多信息，请查看其在线文档
`dvb-zap Documentation
<https://www.linuxtv.org/wiki/index.php/Dvbv5-zap>`_銆。
另见：`zap <https://www.linuxtv.org/wiki/index.php/Zap>`_


### vidtv 中仍可改进之


#### 添加 *debugfs* 集成


尽管前端驱动通过 .read_status 调用提供 DVBv5 统计信息，一个不错的补充是通过 debugfs 向上层空间提供额外的统计信息，debugfs 是一个简单易用、基RAM 的文件系统，专门用于调试目的

为了避免污染前端驱动，这部分逻辑应实现在一个单独的文件中。这些统计信息是驱动特定的，在测试期间可能很有用

Siano 驱动是使debugfs 向用户空间传递驱动特定统计信息的一个例子，可以作为参考

为了方便，这应当进一步通过一Kconfig 选项来启用和禁用


#### 添加测试视频的方


目前，vidtv 只能编码 PCM 音频。如果能实现一个最简版本MPEG-2 视频编码，我们就可以同时测试视频，那将非常好。首先应当查*ISO 13818-2：信息技术——运动图像及其伴音信息的通用编码——第 2 部分：视，其中涵盖了 MPEG 传输流中压缩视频的编码

这可以选择性地使用 Video4Linux2 测试图案生成v4l2-tpg
```
	drivers/media/common/v4l2-tpg/
```


#### 添加白噪声模


vidtv 调谐器已经有代码来识别所选频率是否距离有效频率表过远。目前，这意味着解调器最终可能会丢失信号锁定，因为调谐器会报告较差的信号质量

一个不错的补充是在信号质量差时模拟一些噪声：

- 随机丢弃一TS 包。如果连续性计数器被更新但包没有被传递给解复用器，这将触发一个连续性错误

- 相应地更新错误统计信息（例如 BER 等）

- 在编码数据中模拟一些噪声


### vidtv 中使用的函数和结构体

















