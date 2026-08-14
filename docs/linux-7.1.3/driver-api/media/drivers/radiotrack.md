
## Radiotrack 收音机驱动


作者：Stephen M. Benoit <benoits@servicepro.com>

日期：1996 年 12 月 14 日

### 致谢


本文档基于 Gideon le Grange（legrang@active.co.za 或 legrang@cs.sun.ac.za）1994 年为 Linux 编写的“C”代码，以及 Frans Brinkman（brinkman@esd.nl）1996 年的补充而写成。此处报告的结果来自作者在自己环境中进行的实验，因此效果可能因人而异……对于本信息的适用性或有效性，我不作任何保证、声明或担保。作者未获得任何关于 AIMS Lab（http://www.aimslab.com/）RadioTrack 卡的其它文档。提供本文档是希望它能帮助那些想在 MS Windows 以外的环境中使用 RadioTrack 卡的用户。

### 为何撰写本文档？


我有一张在运行 MS-Windows 平台时买的 RadioTrack 卡。转到 Linux 后，我找到了 Gideon le Grange 用于运行该卡的命令行软件，发现它很好用！Frans Brinkman 制作了一个舒适的 X-windows 界面，并增加了扫描功能。出于折腾的目的，我想看看调谐器能否调谐到通常的 FM 广播频段之外，这样我就能接收到位于 87.0-109.0 MHz 范围上下方的北美广播电视频道的音频载波。我没有取得太大成功，但我了解了 Linux 下对 ioport 的编程，并对该卡所用的硬件设计有了一些认识。

那么，闲话少说，下面是细节。

### 物理描述


RadioTrack 卡是一张 ISA 8 位 FM 收音机卡。射频（RF）输入只是一根天线引线，输出则是通过微型耳机插孔提供的功率音频信号。其工作射频大致限制在 87.0 到 109.0 MHz（商业 FM 广播频段）。尽管寄存器可以被编程以请求超出这些限制的频率，但实验并未给出有希望的结果。对中间频率（IF）信号进行解调的压控振荡器（VFO）可能只有很小的有用频率范围，并且超出上述限制后会回绕或被截断。

### 用 IOPORT 控制该卡


RadioTrack（基址）ioport 可配置为 0x30c 或 0x20c。似乎只涉及一个 ioport。ioport 译码电路应该非常简单，因为各个 ioport 位被直接映射到收音机卡的特定功能（或模块）。这样，通过一次对 ioport 的写操作就可以并行改变许多功能。通过 ioport 可用的唯一反馈似乎是“立体声检测（Stereo Detect）”位。

ioport 的位排列如下：


	MSb                                                         LSb
	+------+------+------+--------+--------+-------+---------+--------+
	| VolA | VolB | ???? | Stereo | Radio  | TuneA | TuneB   | Tune   |
	|  (+) |  (-) |      | Detect | Audio  | (bit) | (latch) | Update |
	|      |      |      | Enable | Enable |       |         | Enable |
	+------+------+------+--------+--------+-------+---------+--------+


====  ====  =================================
VolA  VolB  Description
====  ====  =================================
0	 0  audio mute
0	 1  volume +    (some delay required)
1	 0  volume -    (some delay required)
1	 1  stay at present volume
====  ====  =================================

====================	===========
Stereo Detect Enable	Description
====================	===========
0			No Detect
1			Detect
====================	===========

=============================	=============================
Radio to Audio (path) Enable	Description
=============================	=============================
0				Disable path (silence)
1				Enable path  (audio produced)
=============================	=============================

=====  =====  ==================
TuneA  TuneB  Description
=====  =====  ==================
0	0     "zero" bit phase 1
0	1     "zero" bit phase 2
1	0     "one" bit phase 1
1	1     "one" bit phase 2
=====  =====  ==================


在最后一次端口写入后，等待超过 60 毫秒再读取 ioport 即可获得结果。

  0xff ==> 未检测到立体声，  0xfd ==> 检测到立体声。

=============================	=============================
Radio to Audio (path) Enable	Description
=============================	=============================
0				Disable path (silence)
1				Enable path  (audio produced)
=============================	=============================

24 位代码，其中 bits = (freq*40) + 10486188。
最高有效 11 位必须为 1010 xxxx 0x0 才有效。
位以 LSb 优先的方式移入。

==================	===========================
Tune Update Enable	Description
==================	===========================
0			Tuner held constant
1			Tuner updating in progress
==================	===========================


### 编程示例



	Default:        BASE <-- 0xc8  (current volume, no stereo detect,
					radio enable, tuner adjust disable)

	Card Off:	BASE <-- 0x00  (audio mute, no stereo detect,
					radio disable, tuner adjust disable)

	Card On:	BASE <-- 0x00  (see "Card Off", clears any unfinished business)
			BASE <-- 0xc8  (see "Default")

	Volume Down:    BASE <-- 0x48  (volume down, no stereo detect,
					radio enable, tuner adjust disable)
			wait 10 msec
			BASE <-- 0xc8  (see "Default")

	Volume Up:      BASE <-- 0x88  (volume up, no stereo detect,
					radio enable, tuner adjust disable)
			wait 10 msec
			BASE <-- 0xc8  (see "Default")

	Check Stereo:   BASE <-- 0xd8  (current volume, stereo detect,
					radio enable, tuner adjust disable)
			wait 100 msec
			x <-- BASE     (read ioport)
			BASE <-- 0xc8  (see "Default")

			x=0xff ==> "not stereo", x=0xfd ==> "stereo detected"

	Set Frequency:  code = (freq*40) + 10486188
			foreach of the 24 bits in code,
			(from Least to Most Significant):
			to write a "zero" bit,
			BASE <-- 0x01  (audio mute, no stereo detect, radio
					disable, "zero" bit phase 1, tuner adjust)
			BASE <-- 0x03  (audio mute, no stereo detect, radio
					disable, "zero" bit phase 2, tuner adjust)
			to write a "one" bit,
			BASE <-- 0x05  (audio mute, no stereo detect, radio
					disable, "one" bit phase 1, tuner adjust)
			BASE <-- 0x07  (audio mute, no stereo detect, radio
					disable, "one" bit phase 2, tuner adjust)
