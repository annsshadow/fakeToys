## Sound Blaster Audigy 混音器 / 默认 DSP 代码

本文基于 sb-live-mixer.rst。EMU10K2 芯片包含一个 DSP 部分，它可以被编程以支持多种采样处理方式，本文对此进行描述。（本文不涉及 EMU10K2 芯片的整体功能，详见 manuals 一节。）ALSA 驱动默认会对芯片的这一部分进行编程（之后可以修改），从而提供以下功能：

## 数字混音器控制

这些控制由 DSP 指令构建而成，提供了扩展功能。本文仅描述 ALSA 驱动中默认内置的代码。请注意，这些控制用作衰减器（attenuator）：最大值即为不改变信号的中性位置。另请注意，如果多个控制引用了相同的目标（destination），信号会被累加，并可能被削波（clip，即在未做溢出检查的情况下被设为最大或最小值）。

所用缩写说明：

DAC
	数字到模拟转换器
ADC
	模拟到数字转换器
I2S
	飞利浦半导体公司定义的单向三线串行总线，用于数字音频（该标准用于连接独立的 D/A 与 A/D 转换器）
LFE
	低频效果（用作低音炮信号）
AC97
	包含模拟混音器、D/A 与 A/D 转换器的芯片
IEC958
	S/PDIF
FX-bus
	EMU10K2 芯片拥有一条效果总线（FX-bus），包含 64 个累加器（accumulator）。每个合成器声部（voice）都可以将自身输出送入这些累加器，DSP 微控制器则可以对它们的和进行操作。

### name='PCM Front Playback Volume',索引=0

该控制用于衰减来自左、右前置 PCM FX-bus 累加器的采样。在 5.1 声道回放中，ALSA 使用累加器 8 和 9 来处理左、右前置 PCM 采样。处理后的采样被送往前置扬声器。

### name='PCM Surround Playback Volume',索引=0

该控制用于衰减来自左、右环绕（surround）PCM FX-bus 累加器的采样。在 5.1 声道回放中，ALSA 使用累加器 2 和 3 来处理左、右环绕 PCM 采样。处理后的采样被送往环绕（后置）扬声器。

### name='PCM Side Playback Volume',索引=0

该控制用于衰减来自左、右侧面（side）PCM FX-bus 累加器的采样。在 7.1 声道回放中，ALSA 使用累加器 14 和 15 来处理左、右侧面 PCM 采样。处理后的采样被送往侧面扬声器。

### name='PCM Center Playback Volume',索引=0

该控制用于衰减来自中央（center）PCM FX-bus 累加器的采样。在 5.1 声道回放中，ALSA 使用累加器 6 来处理中央 PCM 采样。处理后的采样被送往中置扬声器。

### name='PCM LFE Playback Volume',索引=0

该控制用于衰减 LFE PCM FX-bus 累加器的采样。在 5.1 声道回放中，ALSA 使用累加器 7 来处理 LFE PCM 采样。处理后的采样被送往低音炮。

### name='PCM Playback Volume',索引=0

该控制用于衰减来自左、右 PCM FX-bus 累加器的采样。在立体声回放中，ALSA 使用累加器 0 和 1 来处理左、右 PCM 采样。处理后的采样被送往前置扬声器。

### name='PCM Capture Volume',索引=0

该控制用于衰减来自左、右 PCM FX-bus 累加器的采样。在立体声回放中，ALSA 使用累加器 0 和 1 来处理左、右 PCM 采样。处理后的结果被送往标准 capture PCM 设备。

### name='Music Playback Volume',索引=0

该控制用于衰减来自左、右 MIDI FX-bus 累加器的采样。ALSA 使用累加器 4 和 5 来处理左、右 MIDI 采样。处理后的采样被送往虚拟立体声混音器。

### name='Music Capture Volume',索引=0

这些控制用于衰减来自左、右 MIDI FX-bus 累加器的采样。ALSA 使用累加器 4 和 5 来处理左、右 MIDI 采样。处理后的结果被送往标准 capture PCM 设备。

### name='Mic Playback Volume',索引=0

该控制用于衰减来自 AC97 编解码器中左、右 Mic 输入的采样。处理后的采样被送往虚拟立体声混音器。

### name='Mic Capture Volume',索引=0

该控制用于衰减来自 AC97 编解码器中左、右 Mic 输入的采样。处理后的结果被送往标准 capture PCM 设备。原始采样同时被送往 Mic capture PCM 设备（设备 1；16 位 / 8 KHz 单声道），且不受音量控制。

### name='Audigy CD Playback Volume',索引=0

该控制用于衰减来自左、右 IEC958 TTL 数字输入的采样（通常由 CDROM 驱动器提供）。处理后的采样被送往虚拟立体声混音器。

### name='Audigy CD Capture Volume',索引=0

该控制用于衰减来自左、右 IEC958 TTL 数字输入的采样（通常由 CDROM 驱动器提供）。处理后的结果被送往标准 capture PCM 设备。

### name='IEC958 Optical Playback Volume',索引=0

该控制用于衰减来自左、右 IEC958 光纤数字输入的采样。处理后的采样被送往虚拟立体声混音器。

### name='IEC958 Optical Capture Volume',索引=0

该控制用于衰减来自左、右 IEC958 光纤数字输入的采样。处理后的结果被送往标准 capture PCM 设备。

### name='Line2 Playback Volume',索引=0

该控制用于衰减来自左、右 I2S ADC 输入的采样（位于 AudigyDrive 上）。处理后的采样被送往虚拟立体声混音器。

### name='Line2 Capture Volume',索引=1

该控制用于衰减来自左、右 I2S ADC 输入的采样（位于 AudigyDrive 上）。处理后的结果被送往标准 capture PCM 设备。

### name='Analog Mix Playback Volume',索引=0

该控制用于衰减来自 Philips ADC 的左、右 I2S ADC 输入的采样。处理后的采样被送往虚拟立体声混音器。其中包含来自 CD、Line In、Aux 等模拟音源的混音。

### name='Analog Mix Capture Volume',索引=1

该控制用于衰减来自 Philips ADC 的左、右 I2S ADC 输入的采样。处理后的结果被送往标准 capture PCM 设备。

### name='Aux2 Playback Volume',索引=0

该控制用于衰减来自左、右 I2S ADC 输入的采样（位于 AudigyDrive 上）。处理后的采样被送往虚拟立体声混音器。

### name='Aux2 Capture Volume',索引=1

该控制用于衰减来自左、右 I2S ADC 输入的采样（位于 AudigyDrive 上）。处理后的结果被送往标准 capture PCM 设备。

### name='Front Playback Volume',索引=0

该控制用于衰减来自虚拟立体声混音器的采样。处理后的采样被送往前置扬声器。

### name='Surround Playback Volume',索引=0

该控制用于衰减来自虚拟立体声混音器的采样。处理后的采样被送往环绕（后置）扬声器。

### name='Side Playback Volume',索引=0

该控制用于衰减来自虚拟立体声混音器的采样。处理后的采样被送往侧面扬声器。

### name='Center Playback Volume',索引=0

该控制用于衰减来自虚拟立体声混音器的采样。处理后的采样被送往中置扬声器。

### name='LFE Playback Volume',索引=0

该控制用于衰减来自虚拟立体声混音器的采样。处理后的采样被送往低音炮。

### name='Tone Control - Switch',索引=0

该控制用于开启或关闭音调控制。送往扬声器输出的采样会受影响。

### name='Tone Control - Bass',索引=0

该控制用于设置低音（bass）强度。不存在中性值！一旦音调控制代码被激活，采样总是会被修改。最接近纯净信号的值是 20。

### name='Tone Control - Treble',索引=0

该控制用于设置高音（treble）强度。不存在中性值！一旦音调控制代码被激活，采样总是会被修改。最接近纯净信号的值是 20。

### name='Master Playback Volume',索引=0

该控制用于衰减送往扬声器输出的采样。

### name='IEC958 Optical Raw Playback Switch',索引=0

若开启此开关，则用于 IEC958（S/PDIF）数字输出的采样仅取自 raw iec958 ALSA PCM 设备（默认情况下它使用累加器 20 和 21 处理左、右 PCM）。


## 与 PCM 流相关的控制

### name='EMU10K1 PCM Volume',索引 0-31

通道音量衰减，范围 0–0x1fffd。中间值（即无衰减）为默认值。三个值的通道映射如下：

- 0 - mono，默认 0xffff（无衰减）
- 1 - left，默认 0xffff（无衰减）
- 2 - right，默认 0xffff（无衰减）

### name='EMU10K1 PCM Send Routing',索引 0-31

该控制指定目标——即 FX-bus 累加器。此映射中共有 24 个值：

- 0 -  mono，A 目标（FX-bus 0-63），默认 0
- 1 -  mono，B 目标（FX-bus 0-63），默认 1
- 2 -  mono，C 目标（FX-bus 0-63），默认 2
- 3 -  mono，D 目标（FX-bus 0-63），默认 3
- 4 -  mono，E 目标（FX-bus 0-63），默认 4
- 5 -  mono，F 目标（FX-bus 0-63），默认 5
- 6 -  mono，G 目标（FX-bus 0-63），默认 6
- 7 -  mono，H 目标（FX-bus 0-63），默认 7
- 8 -  left，A 目标（FX-bus 0-63），默认 0
- 9 -  left，B 目标（FX-bus 0-63），默认 1
- 10 -  left，C 目标（FX-bus 0-63），默认 2
- 11 -  left，D 目标（FX-bus 0-63），默认 3
- 12 -  left，E 目标（FX-bus 0-63），默认 4
- 13 -  left，F 目标（FX-bus 0-63），默认 5
- 14 -  left，G 目标（FX-bus 0-63），默认 6
- 15 -  left，H 目标（FX-bus 0-63），默认 7
- 16 -  right，A 目标（FX-bus 0-63），默认 0
- 17 -  right，B 目标（FX-bus 0-63），默认 1
- 18 -  right，C 目标（FX-bus 0-63），默认 2
- 19 -  right，D 目标（FX-bus 0-63），默认 3
- 20 -  right，E 目标（FX-bus 0-63），默认 4
- 21 -  right，F 目标（FX-bus 0-63），默认 5
- 22 -  right，G 目标（FX-bus 0-63），默认 6
- 23 -  right，H 目标（FX-bus 0-63），默认 7

请不要忘记：将同一通道多次分配到相同的 FX-bus 累加器是非法的（即 0=0 && 1=0 是一个无效组合）。

### name='EMU10K1 PCM Send Volume',索引 0-31

它指定给定目标的衰减量（amount），范围 0–255。通道映射如下：

- 0 -  mono，A 目标 attn，默认 255（无衰减）
- 1 -  mono，B 目标 attn，默认 255（无衰减）
- 2 -  mono，C 目标 attn，默认 0（静音）
- 3 -  mono，D 目标 attn，默认 0（静音）
- 4 -  mono，E 目标 attn，默认 0（静音）
- 5 -  mono，F 目标 attn，默认 0（静音）
- 6 -  mono，G 目标 attn，默认 0（静音）
- 7 -  mono，H 目标 attn，默认 0（静音）
- 8 -  left，A 目标 attn，默认 255（无衰减）
- 9 -  left，B 目标 attn，默认 0（静音）
- 10 -  left，C 目标 attn，默认 0（静音）
- 11 -  left，D 目标 attn，默认 0（静音）
- 12 -  left，E 目标 attn，默认 0（静音）
- 13 -  left，F 目标 attn，默认 0（静音）
- 14 -  left，G 目标 attn，默认 0（静音）
- 15 -  left，H 目标 attn，默认 0（静音）
- 16 -  right，A 目标 attn，默认 0（静音）
- 17 -  right，B 目标 attn，默认 255（无衰减）
- 18 -  right，C 目标 attn，默认 0（静音）
- 19 -  right，D 目标 attn，默认 0（静音）
- 20 -  right，E 目标 attn，默认 0（静音）
- 21 -  right，F 目标 attn，默认 0（静音）
- 22 -  right，G 目标 attn，默认 0（静音）
- 23 -  right，H 目标 attn，默认 0（静音）

## 手册 / 专利

参见 sb-live-mixer.rst。
