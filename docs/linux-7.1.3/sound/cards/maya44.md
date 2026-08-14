## Maya44 USB 音频支持说明


   以下是 Rainer 补丁的原始文档，当前 maya44 代码基于该补丁。部分内容可能已经过时，但我作为参考保留在此 -- tiwai

2008 年 2 月 14 日

Rainer Zimmermann <mail@lightshed.de>

## 开发状态


该驱动由 Piotr Makowski（oponek@gmail.com）倡议开发，并由 Lars Bergmann 资助。
开发由 Rainer Zimmermann（mail@lightshed.de）进行。

ESI 为开发工作提供了一张 Maya44 样卡。

然而，遗憾的是，获取详细的编程信息十分困难，因此我（Rainer Zimmermann）不得不通过实验和推测来找出一些卡相关的信息。部分信息（特别是几个 GPIO 位）仍然缺失。

这是发布到 alsa-devel 邮件列表的 Maya44 驱动的第一个测试版本（2008 年 2 月 5 日）。


经 Rainer Zimmermann 和 Piotr Makowski 测试，以下功能可用：

- 所有采样率的播放和采集
- 输入/输出电平
- 交叉混音（crossmixing）
- 线路/麦克风切换
- 幻象电源开关
- 模拟监听（即旁路，bypass）


以下功能**应该**可用，但尚未完全测试：

- 通道 3+4 模拟 - S/PDIF 输入切换
- S/PDIF 输出
- M/IO/DIO 扩展卡上的所有输入/输出
- 内部/外部时钟选择


**特别是，我们非常感谢任何能访问 M/IO/DIO 扩展卡的人测试这些功能。**


似乎无法工作的内容：

- alsamixer 中的电平表（“multi track”）似乎不对输入信号作出反应（如果这是一个 bug，它很可能在现有的 ICE1724 代码中）。

- Ardour 2.1 似乎只能通过 JACK 工作，而不能直接使用 ALSA 或通过 OSS 使用。这仍需要追查。


## 驱动细节


添加了以下文件：

- pci/ice1724/maya44.c - Maya44 相关代码
- pci/ice1724/maya44.h
- pci/ice1724/ice1724.patch
- pci/ice1724/ice1724.h.patch - 对 ice1724.h 的建议补丁（见“采样率”）
- i2c/other/wm8776.c - Wolfson WM8776 编解码器的底层访问例程
- include/wm8776.h


注意，wm8776.c 代码的意图是与具体卡无关，并且实际上不会向 ALSA 基础设施注册该编解码器。
这由 maya44.c 完成，主要是因为某些 WM8776 控件以 Maya44 特有的方式使用，应当有恰当的名称。


在 pci/ice1724 中创建了以下文件，它们只是简单地 `#include` 了来自 alsa-kernel 树的对应文件：

- wtm.h
- vt1720_mobo.h
- revo.h
- prodigy192.h
- pontis.h
- phase.h
- maya44.h
- juli.h
- aureon.h
- amp.h
- envy24ht.h
- se.h
- prodigy_hifi.h


**我希望这是正确的做法。**


## 采样率


Maya44 卡（或者更准确地说，Wolfson WM8776 编解码器）允许播放最高 192 kHz、采集最高 92 kHz 的采样率。

由于 ICE1724 芯片只允许一个全局采样率，处理方式如下：

- 在 maya44 卡上任何已打开的 PCM 设备上设置采样率，将始终为所有播放和采集通道设置**全局**采样率。

- 在驱动的当前状态下，即使是采集设备也允许设置最高 192 kHz 的采样率。

**请避免以高于 96kHz 的采样率进行采集**，即使它看起来可以工作。编解码器实际上无法以这样的速率采集，意味着音质很差。


我建议增加一些代码，用于在采集 PCM 设备上设置时限制采样率。但由于全局采样率的存在，该逻辑会有些问题。

建议的代码（当前已停用）位于 ice1712.h.patch、ice1724.c 和 maya44.c（在 pci/ice1712 中）。


## 声音设备


PCM 设备与输入/输出对应关系如下（假设 Maya44 为第 0 号卡）：

- hw:0,0 input - 立体声，模拟输入 1+2
- hw:0,0 output - 立体声，模拟输出 1+2
- hw:0,1 input - 立体声，模拟输入 3+4 或 S/PDIF 输入
- hw:0,1 output - 立体声，模拟输出 3+4（以及 SPDIF 输出）


## 混音器控件的命名


（关于信号流的更多信息，请参阅 ESI Maya44 手册第 24 页的框图，或 ESI Windows 软件）。


PCM
    （数字）通道 1+2 的输出电平
PCM 1
    通道 3+4 的同样设置

Mic Phantom+48V
    为输入 1/2 上的静电麦克风提供 +48V 幻象电源的开关。

    确保当有任何其他音源连接到输入 1/2 时，不要打开此开关。
    它可能会损坏音源和/或 maya44 卡。

Mic/Line input
    如果开关打开，输入插孔 1/2 为麦克风输入（单声道），否则为线路输入（立体声）。

Bypass
    通道 1+2 从 ADC 输入到输出的模拟旁路。等同于 Windows 驱动中的“Monitor”。
Bypass 1
    通道 3+4 的同样设置。

Crossmix
    从通道 1+2 到通道 3+4 的交叉混音器
Crossmix 1
    从通道 3+4 到通道 1+2 的交叉混音器

IEC958 Output
    S/PDIF 输出开关。

    这不受 ESI Windows 驱动支持。
    S/PDIF 应输出与通道 3+4 相同的信号。[未测试！]


Digital output selectors
    这些开关允许从 ADC 到 DAC 的直接数字路由。
    每个开关决定送往某个 DAC 的数字输入数据来自何处。
    它们不受 ESI Windows 驱动支持。
    对于正常运行，它们都应设置为 “PCM out”。

H/W
    输出源通道 1
H/W 1
    输出源通道 2
H/W 2
    输出源通道 3
H/W 3
    输出源通道 4

H/W 4 ... H/W 9
    未知功能，保留以便进行测试。

    其中某些可能控制 S/PDIF 输出。
    如果这些被证明未使用，它们将在后续驱动版本中移除。

每个数字输出选择器的可选值为：

PCM out
	DAC 对应通道的输出（默认设置）
Input 1 ... Input 4
	来自所选输入通道 ADC 输出的直接路由
