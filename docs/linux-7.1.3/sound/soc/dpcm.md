## 动PCM


## 描述


动PCM（Dynamic PCM）允许一ALSA PCM 设备PCM 流运行期间将PCM 音频数字路由各种数字端点。例如，PCM0 可以将数字音频路由到 I2S DAI0、I2S DAI1 PDM DAI2。这对于
暴露多个 ALSA PCM 且可路由到多DAI 的片SoC DSP 驱动很有用
DPCM 运行时路由由 ALSA 混音器设置决定，方式与在 ASoC 编解码器驱动中路由模拟信号相同DPCM 使用一个代DSP 内部音频路径DAPM 图，并利用混音器设置来确定每ALSA PCM 所使用路径
DPCM 复用所有现有的组件编解码器、平台以DAI 驱动，无需任何修改

### SoC DSP 的手机音频系

考虑如下手机音频子系统。本文将用它来展示所有示例：

```

  | Front End PCMs    |  SoC DSP  | Back End DAIs | Audio devices |

                      *************
  PCM0 <------------> *           * <----DAI0-----> Codec Headset
                      *           *
  PCM1 <------------> *           * <----DAI1-----> Codec Speakers
                      *   DSP     *
  PCM2 <------------> *           * <----DAI2-----> MODEM
                      *           *
  PCM3 <------------> *           * <----DAI3-----> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

该图展示了一个简单的智能手机音频子系统。它支持蓝牙、FM 数字收音机、扬声器、耳机插孔数字麦克风以及蜂窝调制解调器。此声卡暴露 4 DSP 前端（FE）ALSA PCM 设备，并支持 6 后端（BE）DAI。每FE PCM 都可以将音频数据数字路由到任BE DAI。FE PCM 设备也可以将
音频路由到多1 BE DAI

### 示例 - 将播放从 DAI0 切换DAI1 DPCM


音频正在播放到耳机。过了一会儿用户拔下耳机，音频继续在扬声器上播放
PCM0 到耳机的播放看起来如下
```

                      *************
  PCM0 <============> *           * <====DAI0=====> Codec Headset
                      *           *
  PCM1 <------------> *           * <----DAI1-----> Codec Speakers
                      *   DSP     *
  PCM2 <------------> *           * <----DAI2-----> MODEM
                      *           *
  PCM3 <------------> *           * <----DAI3-----> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

用户从插孔中拔出耳机，因此现在必须使用扬声器
```

                      *************
  PCM0 <============> *           * <----DAI0-----> Codec Headset
                      *           *
  PCM1 <------------> *           * <====DAI1=====> Codec Speakers
                      *   DSP     *
  PCM2 <------------> *           * <----DAI2-----> MODEM
                      *           *
  PCM3 <------------> *           * <----DAI3-----> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

音频驱动按如下方式处理：

1. 机器驱动收到插孔拔出事件
2. 机器驱动或音HAL 禁用耳机路径
3. 由于路径现已禁用，DPCM DAI0 上对耳机运行 PCM trigger(stop)、hw_free()、shutdown()
   操作
4. 机器驱动或音HAL 启用扬声器路径
5. 由于路径已启用，DPCM DAI1 扬声器上运行 startup()、hw_params()、prepare() 以及
   trigger(start) 这些 PCM 操作
在此示例中，机器驱动或用户空间音HAL 可以改变路由，然DPCM 会负责管理与 DAI PCM
相关的操作，以使链路 up down。在此转换期间音频播放不会停止

## DPCM 机器驱动


支持 DPCM ASoC 机器驱动与普通的机器驱动类似，只是我们还必须
1. 定义 FE BE DAI 链路
2. 定义任何 FE/BE PCM 操作
3. 定义部件图连接

### FE 鍜?BE DAI 閾捐矾

```

  | Front End PCMs    |  SoC DSP  | Back End DAIs | Audio devices |

                      *************
  PCM0 <------------> *           * <----DAI0-----> Codec Headset
                      *           *
  PCM1 <------------> *           * <----DAI1-----> Codec Speakers
                      *   DSP     *
  PCM2 <------------> *           * <----DAI2-----> MODEM
                      *           *
  PCM3 <------------> *           * <----DAI3-----> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

对于上面的示例，我们必须定义 4 FE DAI 链路6 BE DAI 链路。FE DAI 链路定义如下
```

 SND_SOC_DAILINK_DEFS(pcm0,
	DAILINK_COMP_ARRAY(COMP_CPU("System Pin")),
	DAILINK_COMP_ARRAY(COMP_DUMMY()),
	DAILINK_COMP_ARRAY(COMP_PLATFORM("dsp-audio")));

  static struct snd_soc_dai_link machine_dais[] = {
	{
		.name = "PCM0 System",
		.stream_name = "System Playback",
		SND_SOC_DAILINK_REG(pcm0),
		.dynamic = 1,
		.trigger = {SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST},
	},
	.....< other FE and BE DAI links here >
  };

```

FE DAI 链路与常DAI 链路非常相似，只是我们还通过设置 `dynamic = 1` 将该 DAI 链路
标记DPCM FE。还有一个选项可以指定每个 FE trigger 调用顺序。这允许 ASoC 核心其他组件之前或之后触DSP（因为某DSP DAI/DSP 启动和停止序列的顺序有严格要求）
上面FE DAI 将编解码器和 codec DAI 设置为虚拟设备，因为 BE 是动态的，并会随运行时配而改变
BE DAI 配置如下
```

 SND_SOC_DAILINK_DEFS(headset,
	DAILINK_COMP_ARRAY(COMP_CPU("ssp-dai.0")),
	DAILINK_COMP_ARRAY(COMP_CODEC("rt5640.0-001c", "rt5640-aif1")));

  static struct snd_soc_dai_link machine_dais[] = {
	.....< FE DAI links here >
	{
		.name = "Codec Headset",
		SND_SOC_DAILINK_REG(headset),
		.no_pcm = 1,
		.ignore_suspend = 1,
		.ignore_pmdown_time = 1,
		.be_hw_params_fixup = hswult_ssp0_fixup,
		.ops = &haswell_ops,
	},
	.....< other BE DAI links here >
  };

```

BE DAI 链路DAI0 连接到编解码器（本例中为 RT5460 AIF1）。它设置 `no_pcm` 标志将其标记BE
BE 还设置了用于忽略挂起PM down 时间的标志。这允许 BE 以无主机（hostless）模式工作，
即在主机 CPU 不传输数据（如蓝牙电话呼叫）的情况下
```

                      *************
  PCM0 <------------> *           * <----DAI0-----> Codec Headset
                      *           *
  PCM1 <------------> *           * <----DAI1-----> Codec Speakers
                      *   DSP     *
  PCM2 <------------> *           * <====DAI2=====> MODEM
                      *           *
  PCM3 <------------> *           * <====DAI3=====> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

这允许主CPU DSP、MODEM DAI BT DAI 仍在运行时休眠
如果编解码器是由外部管理的设备，BE DAI 链路也可以将编解码器设置为虚拟设备
同样，如CPU DAI DSP 固件管理，BE DAI 也可以将虚拟 cpu DAI 设置为虚拟设备

### FE/BE PCM 操作


上面BE 还导出了一PCM 操作和一`fixup` 回调。机器驱动使用该 fixup 回调基于 FE
硬件参数（hw params）来（重新）配置 DAI。即 DSP 可以FE BE 之间执行 SRC ASRC
例如，DSP 将所FE hw params 转换为以固定48k6bit、立体声运行DAI0。这意味着所FE hw_params 都必须在机器驱动中为 DAI0 修正，以DAI 以期望的配置运行，而不FE 的配如何
```

  static int dai0_fixup(struct snd_soc_pcm_runtime *rtd,
			struct snd_pcm_hw_params *params)
  {
	struct snd_interval *rate = hw_param_interval(params,
			SNDRV_PCM_HW_PARAM_RATE);
	struct snd_interval *channels = hw_param_interval(params,
						SNDRV_PCM_HW_PARAM_CHANNELS);

	/* The DSP will convert the FE rate to 48k, stereo */
	rate->min = rate->max = 48000;
	channels->min = channels->max = 2;

	/* set DAI0 to 16 bit */
	params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
	return 0;
  }

```

其他 PCM 操作与常DAI 链路相同。按需使用

### 部件图连

BE DAI 链路通常会在初始化时ASoC DAPM 核心连接到图中。但是，如果 BE 编解码器BE DAI
是虚拟的，则必须在驱动中显式设置
```

  /* BE for codec Headset -  DAI0 is dummy and managed by DSP FW */
  {"DAI0 CODEC IN", NULL, "AIF1 Capture"},
  {"AIF1 Playback", NULL, "DAI0 CODEC OUT"},


```

## 编写 DPCM DSP 驱动


DPCM DSP 驱动看起来很像标准的平台ASoC 驱动，融合了编解码器类驱动的元素。一DSP 平台
驱动必须实现
1. 前端 PCM DAI——即 struct snd_soc_dai_driver
2. 显示FE DAI BE DSP 音频路由DAPM 图
3. 来自 DSP 图的 DAPM 部件
4. 用于增益、路由等的混音器
5. DMA 配置
6. BE AIF 部件
6 项对于将音频路由DSP 外部很重要。需要为每个 BE 和每个流方向定义 AIF。例如，对于上面BE DAI0，我们将有：

```

  SND_SOC_DAPM_AIF_IN("DAI0 RX", NULL, 0, SND_SOC_NOPM, 0, 0),
  SND_SOC_DAPM_AIF_OUT("DAI0 TX", NULL, 0, SND_SOC_NOPM, 0, 0),

```

BE AIF 用于DSP 图连接到其他组件驱动（例如编解码器图）的图

## 无主PCM 

无主PCM 流是不经过主CPU 路由的流。这方面的一个例子是从手机到调制解调器的电话呼叫
```

                      *************
  PCM0 <------------> *           * <----DAI0-----> Codec Headset
                      *           *
  PCM1 <------------> *           * <====DAI1=====> Codec Speakers/Mic
                      *   DSP     *
  PCM2 <------------> *           * <====DAI2=====> MODEM
                      *           *
  PCM3 <------------> *           * <----DAI3-----> BT
                      *           *
                      *           * <----DAI4-----> DMIC
                      *           *
                      *           * <----DAI5-----> FM
                      *************

```

在这种情况下，PCM 数据通过 DSP 路由。主CPU 在此用例中仅用于控制，并可在流运行期间休眠
主机可以通过以下两种方式控制无主机链路：

  1. 将链路配置为 CODEC <-> CODEC 风格的链路。在这种情况下，链路DAPM 图的状态启用或
     禁用。这通常意味着有一个混音器控件可用于连接或断开两个 DAI 之间的路径
  2. 无主FE。此 FE DAPM 图上BE DAI 链路具有虚拟连接。随后像常规 PCM 操作一样由
     FE 执行控制。此方法DAI 链路提供更强的控制，但需要多得多的用户空间代码来控制链路     除非你的硬件需要对 PCM 操作进行更细粒度的排序，否则建议使用 CODEC<->CODEC

### CODEC <-> CODEC 链路


DAPM 检测到 DAPM 图内的有效路径时，此 DAI 链路被启用。机器驱动向 DAI 链路设置一些附参数，即
```

  static const struct snd_soc_pcm_stream dai_params = {
	.formats = SNDRV_PCM_FMTBIT_S32_LE,
	.rate_min = 8000,
	.rate_max = 8000,
	.channels_min = 2,
	.channels_max = 2,
  };

  static struct snd_soc_dai_link dais[] = {
	< ... more DAI links above ... >
	{
		.name = "MODEM",
		.stream_name = "MODEM",
		.cpu_dai_name = "dai2",
		.codec_dai_name = "modem-aif1",
		.codec_name = "modem",
		.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
				| SND_SOC_DAIFMT_CBP_CFP,
		.c2c_params = &dai_params,
		.num_c2c_params = 1,
	}
	< ... more DAI links here ... >

```

DAPM 检测到有效路径并随后调PCM 操作以启动链路时，将使用这些参数来配DAI hw_params()当路径不再有效时，DAPM 也会调用适当PCM 操作来禁DAI

### 鏃犱富鏈?FE


DAI 链路由一个不读取或写入任PCM 数据FE 启用。这意味着创建一个与两个 DAI 链路具有
虚拟连接的新 FE。当 FE PCM 启动时，DAI 链路将启动；FE PCM 停止时，DAI 链路将停止。注意，
在此配置FE PCM 不能读取或写入数据