## 使用 JACK 与 emu10k1/emu10k2 实现低延迟、多声道音频


本文档是一份指南，介绍如何将基于 emu10k1 的设备与 JACK 配合使用，以获得低延迟、多声道录音功能。
我近期所有让 Linux 用户使用其硬件全部能力的工作，都受到 kX Project 的启发。没有他们的成果，我
永远无法发现这款硬件的真正威力。

	http://www.kxproject.com
      - Lee Revell，2005.03.30

直到最近，Linux 上的 emu10k1 用户还无法使用其 Windows 驱动中“kX ASIO”特性所提供的相同低延迟、
多声道功能。自 ALSA 1.0.9 起，这已成为过去！

对于那些不熟悉 kX ASIO 的人，它由 16 个采集通道与 16 个回放通道组成。使用 2.6.9 之后的 Linux 内核，
低至 64（1.33 ms）甚至 32（0.66ms）帧的延迟应该都能良好工作。

配置比在 Windows 上稍微复杂一些，因为你必须选择正确的设备供 JACK 使用。实际上，对于 qjackctl 用户
来说这相当一目了然——选择 Duplex，然后为采集与回放选择多声道设备，将输入与输出通道设为 16，采样率
设为 48000Hz。命令行如下：
```

  /usr/local/bin/jackd -R -dalsa -r48000 -p64 -n2 -D -Chw:0,2 -Phw:0,3 -S

```
这将为你提供 16 个输入端口与 16 个输出端口。

16 个输出端口映射到 16 个 FX 总线（对于 Audigy 则是前 16 个，共 64 个）。从 FX 总线到物理输出的映射
在 sb-live-mixer.rst（或 audigy-mixer.rst）中描述。

16 个输入端口连接到 16 个物理输入。与普遍看法相反，所有 emu10k1 卡都是多声道卡。这些输入通道中哪些
连接有物理输入，取决于卡的型号。强烈建议通过试错来确定；一些富有进取心的 kX 用户已经逆向工程出该卡的
引脚图，并可在网上找到。Meterbridge 在这里很有帮助，kX 论坛中也充斥着有用的信息。

每个输入端口要么对应于一个数字（SPDIF）输入、一个模拟输入，要么什么也没有。唯一的例外是 SBLive! 5.1。
在这些设备上，第二个与第三个输入端口被接到 center/LFE 输出。你仍然会看到 16 个采集通道，但只有 14 个
可用于录音输入。

下表借用自 kxfxlib/da_asio51.cpp，描述了 JACK 端口到 FXBUS2（多轨录音输入）与 EXTOUT（物理输出）
通道的映射。

10k1 5.1 SBLive 卡上的 JACK（及 ASIO）映射：

==============  ========        ============
JACK		Epilog		FXBUS2(nr)
==============  ========        ============
capture_1	asio14		FXBUS2(0xe)
capture_2	asio15		FXBUS2(0xf)
capture_3	asio0		FXBUS2(0x0)	
~capture_4	Center		EXTOUT(0x11)	// 由 Center 映射而来
~capture_5	LFE		EXTOUT(0x12)	// 由 LFE 映射而来
capture_6	asio3		FXBUS2(0x3)
capture_7	asio4		FXBUS2(0x4)
capture_8	asio5		FXBUS2(0x5)
capture_9	asio6		FXBUS2(0x6)
capture_10	asio7		FXBUS2(0x7)
capture_11	asio8		FXBUS2(0x8)
capture_12	asio9		FXBUS2(0x9)
capture_13	asio10		FXBUS2(0xa)
capture_14	asio11		FXBUS2(0xb)
capture_15	asio12		FXBUS2(0xc)
capture_16	asio13		FXBUS2(0xd)
==============  ========        ============

待办：描述 ld10k1/qlo10k1 与 JACK 结合使用
