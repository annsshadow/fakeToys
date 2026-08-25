## 面向便携设备的动态音频电源管理（DAPM
## 描述

动态音频电源管理（Dynamic Audio Power Management，DAPM）旨在让便携 Linux 设备
始终在音频子系统中使用最少的功耗。它独立于其他内核电源管理框架，因此可以轻松
地与它们共存
DAPM 对所有用户空间应用程序也完全透明，因为所有的电源切换都在 ASoC 核心内部
完成。用户空间应用程序不需要修改代码或重新编译。DAPM 根据任何音频流（采集/播放的活动以及设备内的音频混音器设置来做出电源切换决策
DAPM 基于两个基本元素，称widget（部件）route（路由）
 - **widget** 是音频硬件的每一个部分，在使用时可由软件启用，不使用时禁用以省电
 - **route** widget 之间的互连，当声音能够从一widget 流向另一widget 时存
所DAPM 电源切换决策都是通过查询音频路由图自动做出的。这个图对每个声卡都特定的，并且跨越整个声卡，因此某DAPM 路由会连接属于不同组件的 widget（例一CODEC LINE OUT 引脚与一个放大器的输入引脚）
STM32MP1-DK1 声卡的路由图如下所示：

    :alt:   Example DAPM graph
    :align: center

你也可以使用 `tools/sound/dapm-graph` 工具为你的声卡生成兼容的图
## DAPM 电源
DAPM 内部4 个电源域
Codec 偏置（bias）域
      VREF、VMID（核codec 与音频电源）

      通常codec 探测/移除以及挂起/恢复时控制，尽管如果侧音等不需要电源时也可以在
      流时间设置
平台/机器      物理连接的输入与输出

      取决于平机器以及用户操作，由机器驱动配置，并响应异步事件，例如插入耳机（HP）时
路径      音频子系统信号路
      在用户更改混音器和多路复用（mux）设置时自动设置。例alsamixer、amixer
流域
      DAC 鍜?ADC銆。
      在流播放/采集分别开始和停止时启用和禁用。例aplay、arecord
## DAPM Widgets

音频 DAPM widget 可分为若干类型：

Mixer
	将若干模拟信号混合为单个模拟信号Mux
	一个只输出多个输入中某一个的模拟开关PGA
	一个可编程增益放大器或衰减 widgetADC
	模数转换器（Analog to Digital ConverterDAC
	数模转换器（Digital to Analog ConverterSwitch
	一个模拟开Input
	一codec 输入引脚
Output
	一codec 输出引脚
Headphone
	耳机（以及可选的 JackMic
	麦克风（以及可选的 JackLine
	线路输入/输出（以及可选的 JackSpeaker
	扬声Supply
	被其widget 使用的电源或时钟供应 widgetRegulator
	为音频组件供电的外部稳压器Clock
	为音频组件提供时钟的外部时钟AIF IN
	音频接口输入（带 TDM 时隙掩码）AIF OUT
	音频接口输出（带 TDM 时隙掩码）Siggen
	信号发生器DAI IN
	数字音频接口输入DAI OUT
	数字音频接口输出DAI Link
	两个 DAI 结构之间DAI 链路Pre
	特殊PRE widget（在所有其他之前执行）
Post
	特殊POST widget（在所有其他之后执行）
Buffer
	DSP 内部部件之间的音频数据缓冲区Scheduler
	调度组件/流水线处理工作的 DSP 内部调度器Effect
	执行音频处理效果widgetSRC
	DSP CODEC 内的采样率转换器（Sample Rate Converter）ASRC
	DSP CODEC 内的异步采样率转换器（Asynchronous Sample Rate Converter）Encoder
	将数据从一种格式（通常PCM）编码为另一种通常压缩程度更高的格式的 widgetDecoder
	将数据从压缩格式解码PCM 等未压缩格式widget
（Widget include/sound/soc-dapm.h 中定义）

Widget 可以由任何组件驱动类型添加到声卡中。soc-dapm.h 中定义了便于使用的宏可用于快速构codec 与机DAPM widget 的列表
大多widget 具有 name、register、shift invert。某widget 带有用于流名kcontrol 的额外参数
### 流域 Widgets

widget 与流电源域相关，仅由 ADC（模数转换器）、DAC（数模转换器）、AIF IN AIF OUT 组成
widget 具有以下格式```
  SND_SOC_DAPM_DAC(name, stream name, reg, shift, invert),
  SND_SOC_DAPM_AIF_IN(name, stream, slot, reg, shift, invert)
```

注意：流名必须与codec snd_soc_dai_driver 里对应的流名相匹配
例如 HiFi 播放与采集的widget```
  SND_SOC_DAPM_DAC("HiFi DAC", "HiFi Playback", REG, 3, 1),
  SND_SOC_DAPM_ADC("HiFi ADC", "HiFi Capture", REG, 2, 1),
```

例如 AIF 的流 widget```
  SND_SOC_DAPM_AIF_IN("AIF1RX", "AIF1 Playback", 0, SND_SOC_NOPM, 0, 0),
  SND_SOC_DAPM_AIF_OUT("AIF1TX", "AIF1 Capture", 0, SND_SOC_NOPM, 0, 0),
```

### 路径Widgets

路径widget 具有控制或影响音频子系统内音频信号或音频路径的能力。它们具以下形式```
  SND_SOC_DAPM_PGA(name, reg, shift, invert, controls, num_controls)
```

任何 widget kcontrol 都可以通过 controls num_controls 成员设置
例如 Mixer widget（kcontrol 先声明）```
  /* Output Mixer */
  static const snd_kcontrol_new_t wm8731_output_mixer_controls[] = {
  SOC_DAPM_SINGLE("Line Bypass Switch", WM8731_APANA, 3, 1, 0),
  SOC_DAPM_SINGLE("Mic Sidetone Switch", WM8731_APANA, 5, 1, 0),
  SOC_DAPM_SINGLE("HiFi Playback Switch", WM8731_APANA, 4, 1, 0),
  };

  SND_SOC_DAPM_MIXER("Output Mixer", WM8731_PWR, 4, 1, wm8731_output_mixer_controls,
	ARRAY_SIZE(wm8731_output_mixer_controls)),
```

如果你不希望混音器元素以混音widget 的名称作为前缀，可以使SND_SOC_DAPM_MIXER_NAMED_CTL 来代替。参数与 SND_SOC_DAPM_MIXER 相同
### 机器Widgets

机器 widget codec widget 的不同之处在于它们没有与之关联的 codec 寄存器位每个可以独立供电的机器音频组件（codec DSP）都会被分配给一个机widget例如
- 扬声器放大器（Speaker Amp- 麦克风偏置（Microphone Bias- Jack 连接
一个机widget 可以有一个可选的回调函数
例如用于外部 Mic Jack 连接widget，它启用 Mic Bias```
  static int spitz_mic_bias(struct snd_soc_dapm_widget* w, int event)
  {
	gpio_set_value(SPITZ_GPIO_MIC_BIAS, SND_SOC_DAPM_EVENT_ON(event));
	return 0;
  }

  SND_SOC_DAPM_MIC("Mic Jack", spitz_mic_bias),
```

### Codec（BIAS）域

codec 偏置电源域没widget，由 codec DAPM 事件处理程序处理。当 codec 电源状相对于任何流事件或内PM 事件发生改变时，会调用该处理程序
### 虚拟 Widgets

有时 codec 或机器音频图中存在一widget，它们没有任何对应的软电源控制。在这种
情况下，有必要创建一个虚widget——一个没有控制位widget，例如：
```
  SND_SOC_DAPM_MIXER("AC97 Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
```

这可用于在软件中将两条信号路径合并在一起
## 注册 DAPM 控件

在许多情况下，DAPM widget 静态地实现codec 驱动中的一``static const struct
snd_soc_dapm_widget`` 数组中，并简单地通过 `struct snd_soc_component_driver` `dapm_widgets` `num_dapm_widgets` 字段来声明
类似地，连接它们的路由静态地实现``static const struct snd_soc_dapm_route``
数组中，并通过同一struct `dapm_routes` `num_dapm_routes` 字段声明
在声明了上述内容之后，驱动注册会处理```
  static const struct snd_soc_dapm_widget wm2000_dapm_widgets[] = {
  	SND_SOC_DAPM_OUTPUT("SPKN"),
  	SND_SOC_DAPM_OUTPUT("SPKP"),
  	...
  };

  /* Target, Path, Source */
  static const struct snd_soc_dapm_route wm2000_audio_map[] = {
  	{ "SPKN", NULL, "ANC Engine" },
  	{ "SPKP", NULL, "ANC Engine" },
	...
  };

  static const struct snd_soc_component_driver soc_component_dev_wm2000 = {
	...
  	.dapm_widgets		= wm2000_dapm_widgets,
  	.num_dapm_widgets	= ARRAY_SIZE(wm2000_dapm_widgets),
  	.dapm_routes            = wm2000_audio_map,
  	.num_dapm_routes        = ARRAY_SIZE(wm2000_audio_map),
	...
  };
```

在更复杂的情况下，DAPM widget 或路由列表只能在探测（probe）时才能确定。例当驱动支持具有不同特性集合的不同型号时就会发生这种情况。在这些情况下，实现
特定于该情况特性的独立 widget 和路由数组，可以通过调用 snd_soc_dapm_new_controls()
snd_soc_dapm_add_routes() 以编程方式注册
## Codec/DSP Widget 互连

Widget 通过音频路径（称为互连）codec、平台和机器内部相互连接。必须定义每互连，以便创widget 之间所有音频路径的图
这在使用 codec DSP 的图（以及机器音频系统的原理图）时最为容易，因为它需通过各自的音频信号路径将 widget 连接在一起
例如 WM8731 输出混音器（wm8731.c）有 3 个输入（源）
1. Line Bypass 输入
2. DAC（HiFi 播放3. Mic Sidetone 输入

此示例中的每个输入都有一个相关联kcontrol（在上面示例中定义），并通过kcontrol
名连接到输出混音器。我们现在可以将目标 widget（就音频信号而言）与其：
```
	/* output mixer */
	{"Output Mixer", "Line Bypass Switch", "Line Input"},
	{"Output Mixer", "HiFi Playback Switch", "DAC"},
	{"Output Mixer", "Mic Sidetone Switch", "Mic Bias"},
```

于是我们有：

- 目标 Widget <=== 路径<=== Widget，或- Sink、Path、Source，或- `Output Mixer` 通过 `HiFi Playback Switch` 连接`DAC`
当没有路径名连接 widget 时（例如直接连接），我们为路径名NULL
```
  snd_soc_dapm_connect_input(codec, sink, path, source);
```

最后，必须在所widget 和互连都向核心注册之后调snd_soc_dapm_new_widgets()这会导致核心扫描 codec 和机器，使内DAPM 状态与机器的物理状态相匹配
### 机器 Widget 互连

机器 widget 互连的创建方式与 codec 的相同，并直接将 codec 引脚连接到机器级 widget
例如将扬声器输出 codec 引脚连接到内部扬声器```
	/* ext speaker connected to codec pins LOUT2, ROUT2  */
	{"Ext Spk", NULL , "ROUT2"},
	{"Ext Spk", NULL , "LOUT2"},
```

这允DAPM 分别打开和关闭已连接（且在使用中）的引脚以及 NC（未连接）引脚
## 端点 Widgets

端点是机器内音频信号的起点或终点（widget），并包codec。例如：

- 耳机 Jack
- 内部扬声- 内部麦克- 麦克Jack
- Codec 引脚

端点被添加到 DAPM 图中，以便可以确定它们的使用情况以节省电源。例NC codec
引脚将被关闭，未连接jack 也可以被关闭
## DAPM Widget 事件

需要实现比 DAPM 所能做的更复杂行为widget，可以通过设置一个函数指针来设置
自定义的"事件处理程序"。例如：
```
  static int sof_es8316_speaker_power_event(struct snd_soc_dapm_widget *w,
  					  struct snd_kcontrol *kcontrol, int event)
  {
  	if (SND_SOC_DAPM_EVENT_ON(event))
  		gpiod_set_value_cansleep(gpio_pa, true);
  	else
  		gpiod_set_value_cansleep(gpio_pa, false);

  	return 0;
  }

  static const struct snd_soc_dapm_widget st_widgets[] = {
  	...
  	SND_SOC_DAPM_SUPPLY("Speaker Power", SND_SOC_NOPM, 0, 0,
  			    sof_es8316_speaker_power_event,
  			    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
  };
```

有关所有其他支持事件的 widget，请参阅 soc-dapm.h
### 事件类型

```
  /* dapm event types */
  #define SND_SOC_DAPM_PRE_PMU		0x1	/* before widget power up */
  #define SND_SOC_DAPM_POST_PMU		0x2	/* after  widget power up */
  #define SND_SOC_DAPM_PRE_PMD		0x4	/* before widget power down */
  #define SND_SOC_DAPM_POST_PMD		0x8	/* after  widget power down */
  #define SND_SOC_DAPM_PRE_REG		0x10	/* before audio path setup */
  #define SND_SOC_DAPM_POST_REG		0x20	/* after  audio path setup */
  #define SND_SOC_DAPM_WILL_PMU		0x40	/* called at start of sequence */
  #define SND_SOC_DAPM_WILL_PMD		0x80	/* called at start of sequence */
  #define SND_SOC_DAPM_PRE_POST_PMD	(SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD)
  #define SND_SOC_DAPM_PRE_POST_PMU	(SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU)
```
