## E-MU 数字音频系统 mixer 与默认 DSP 代码

本文档介绍 E-MU 0404/1010/1212/1616/1820 的 PCI/PCI-e/CardBus 系列声卡。

这些声卡采用常规的 EMU10K2（SoundBlaster Audigy）芯片，但搭配了面向半专业录音室录音的替代前端（front-end）。

本文档基于 audigy-mixer.rst。


## 硬件兼容性

EMU10K2 芯片的采集 FIFO（capture FIFO）非常短，若声卡的 PCI 总线请求未被以适当的优先级处理，会导致录音不可靠。在较新的主板上尤其如此：PCI 总线往往只是次级外设，而非设备访问的实际仲裁者。具体而言，在 Intel DP55 主板（内存控制器位于 CPU）上同时进行播放与录音时，我遇到了录音故障；但在 Intel DP45 主板（内存控制器位于北桥）上曾获得成功。这些声卡的 PCI Express 版本（板载一个 PCI 桥，其余部分相同）问题通常较少。


## 驱动能力

该驱动仅支持 16 位、44.1/48 kHz 的操作。多声道设备（参见 emu10k1-jack.rst）还支持 24 位采集。

一个用于增强该驱动的补丁集可从 [一个 GitHub 仓库](https://github.com/ossilator/linux/tree/ossis-emu10k1) 获取。其多声道设备同时支持 24 位的播放与采集，并且还支持完整的 88.2/96/176.4/192 kHz 操作。该补丁集不会进入主线（upstream），原因是对于什么才构成良好的用户体验存在根本分歧。


## 数字 mixer 控制

注意：这些控制作为衰减器（attenuator）工作——最大值即中性位置，保持信号不变。另请注意：若多个控制指向同一目标，信号会被累加，并可能被削波（clip，即在未做溢出检查的情况下被设置为最大或最小值）。

所用缩写说明：

DAC
	数字到模拟转换器（Digital-to-Analog Converter）
ADC
	模拟到数字转换器（Analog-to-Digital Converter）
LFE
	低频效果（low frequency effects，用作低音炮信号）
IEC958
	S/PDIF
FX-bus
	EMU10K2 芯片具有一条效果总线（effect bus），包含 64 个累加器。每个合成器声部（voice）可将自己的输出送入这些累加器，DSP 微控制器可对所得的和进行运算。

### name='Clock Source',索引=0

该控制在内部生成的 44.1 或 48 kHz 字时钟（word clock）与若干外部时钟源之间切换。

注意：1616 CardBus 声卡可用的外部时钟源尚不明确，欢迎反馈你的发现。

### name='Clock Fallback',索引=0

该控制决定当选定的外部时钟源无效（或变为无效）时，声卡所回退到的内部时钟。

### name='DAC1 0202 14dB PAD',索引=0, 等.

输出衰减控制。在 0404 声卡上不可用。

### name='ADC1 14dB PAD 0202',索引=0, 等.

输入衰减控制。在 0404 声卡上不可用。

### name='Optical 输出 模式',索引=0

在 TOSLINK 输出端口的 S/PDIF 与 ADAT 之间切换。在 0404 声卡上不可用（固定为 S/PDIF）。

### name='Optical 输入 模式',索引=0

在 TOSLINK 输入端口的 S/PDIF 与 ADAT 之间切换。在 0404 声卡上不可用（固定为 S/PDIF）。

### name='PCM Front Playback Volume',索引=0

该控制用于衰减来自左、右前置 PCM FX-bus 累加器的采样。ALSA 将累加器 8 和 9 用于左、右前置 PCM 采样（对应 5.1 声道播放）。结果采样被送往 DSP 0 与 1 播放通道。

### name='PCM Surround Playback Volume',索引=0

该控制用于衰减来自左、右环绕 PCM FX-bus 累加器的采样。ALSA 将累加器 2 和 3 用于左、右环绕 PCM 采样（对应 5.1 声道播放）。结果采样被送往 DSP 2 与 3 播放通道。

### name='PCM Side Playback Volume',索引=0

该控制用于衰减来自左、右侧面 PCM FX-bus 累加器的采样。ALSA 将累加器 14 和 15 用于左、右侧面 PCM 采样（对应 7.1 声道播放）。结果采样被送往 DSP 6 与 7 播放通道。

### name='PCM Center Playback Volume',索引=0

该控制用于衰减来自中央 PCM FX-bus 累加器的采样。ALSA 将累加器 6 用于中央 PCM 采样（对应 5.1 声道播放）。结果采样被送往 DSP 4 播放通道。

### name='PCM LFE Playback Volume',索引=0

该控制用于衰减来自 LFE PCM FX-bus 累加器的采样。ALSA 将累加器 7 用于 LFE PCM 采样（对应 5.1 声道播放）。结果采样被送往 DSP 5 播放通道。

### name='PCM Playback Volume',索引=0

该控制用于衰减来自左、右 PCM FX-bus 累加器的采样。ALSA 将累加器 0 和 1 用于左、右 PCM 采样（对应立体声播放）。结果采样被送往虚拟立体声 mixer。

### name='PCM Capture Volume',索引=0

该控制用于衰减来自左、右 PCM FX-bus 累加器的采样。ALSA 将累加器 0 和 1 用于左、右 PCM。结果被送往标准采集 PCM 设备。

### name='Music Playback Volume',索引=0

该控制用于衰减来自左、右 MIDI FX-bus 累加器的采样。ALSA 将累加器 4 和 5 用于左、右 MIDI 采样。结果采样被送往虚拟立体声 mixer。

### name='Music Capture Volume',索引=0

这些控制用于衰减来自左、右 MIDI FX-bus 累加器的采样。ALSA 将累加器 4 和 5 用于左、右 MIDI 采样。结果被送往标准采集 PCM 设备。

### name='Front Playback Volume',索引=0

该控制用于衰减来自虚拟立体声 mixer 的采样。结果采样被送往 DSP 0 与 1 播放通道。

### name='Surround Playback Volume',索引=0

该控制用于衰减来自虚拟立体声 mixer 的采样。结果采样被送往 DSP 2 与 3 播放通道。

### name='Side Playback Volume',索引=0

该控制用于衰减来自虚拟立体声 mixer 的采样。结果采样被送往 DSP 6 与 7 播放通道。

### name='Center Playback Volume',索引=0

该控制用于衰减来自虚拟立体声 mixer 的采样。结果采样被送往 DSP 4 播放通道。

### name='LFE Playback Volume',索引=0

该控制用于衰减来自虚拟立体声 mixer 的采样。结果采样被送往 DSP 5 播放通道。

### name='Tone Control - Switch',索引=0

该控制用于打开或关闭音调控制（tone control）。受影响的采样会被送往 DSP 播放通道。

### name='Tone Control - Bass',索引=0

该控制设置低音（bass）强度。不存在中性值！启用音调控制代码后，采样始终会被修改。最接近纯净信号的值为 20。

### name='Tone Control - Treble',索引=0

该控制设置高音（treble）强度。不存在中性值！启用音调控制代码后，采样始终会被修改。最接近纯净信号的值为 20。

### name='Master Playback Volume',索引=0

该控制用于衰减所有 DSP 播放通道的采样。

### name='EMU Capture Volume',索引=0

该控制用于衰减来自 DSP 0 与 1 采集通道的采样。结果被送往标准采集 PCM 设备。

### name='DAC Left',索引=0, 等.

选择给定物理音频输出的来源。可以是物理输入、播放通道（DSP xx，以十进制数字表示），或静音（silence）。

### name='DSP x',索引=0

选择给定采集通道（以十六进制数字表示）的来源。选项与物理音频输出相同。


## PCM 流相关控制

这些控制在 audigy-mixer.rst 中有说明。


## 手册/专利

参见 sb-live-mixer.rst。
