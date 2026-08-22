## ASoC Codec Class Driver


codec 类驱动是通用且与硬件无关的代码，用于配置 codec、FM、MODEM、BT 或外DSP，以提供音频采集和回放。它不应包含任何特定于目标平台或机器的代码。所有平台和机器特定的代码应分别添加到平台和机器驱动中
每个 codec 类驱*必须**提供以下特性：-

1. Codec DAI 鍜?PCM 閰嶇疆
2. Codec 控制 IO - 使用 RegMap API
3. 混音器（Mixer）和音频控制
4. Codec 音频操作
5. DAPM 描述6. DAPM 事件处理程序
可选地，codec 驱动还可以提供：-

7. DAC 数字静音（digital mute）控制
最好将此指南与 sound/soc/codecs/ 中现有的 codec 驱动代码结合使用
## ASoC Codec driver breakdown


### Codec DAI and PCM configuration


每个 codec 驱动必须有一struct snd_soc_dai_driver 来定义其 DAI PCM 能力及操作。该结构被导出，以便你的机器驱动可以将其注册到核心
e.g.
```

  static struct snd_soc_dai_ops wm8731_dai_ops = {
	.prepare	= wm8731_pcm_prepare,
	.hw_params	= wm8731_hw_params,
	.shutdown	= wm8731_shutdown,
	.mute_stream	= wm8731_mute,
	.set_sysclk	= wm8731_set_dai_sysclk,
	.set_fmt	= wm8731_set_dai_fmt,
  };

  struct snd_soc_dai_driver wm8731_dai = {
	.name = "wm8731-hifi",
	.playback = {
		.stream_name = "Playback",
		.channels_min = 1,
		.channels_max = 2,
		.rates = WM8731_RATES,
		.formats = WM8731_FORMATS,},
	.capture = {
		.stream_name = "Capture",
		.channels_min = 1,
		.channels_max = 2,
		.rates = WM8731_RATES,
		.formats = WM8731_FORMATS,},
	.ops = &wm8731_dai_ops,
	.symmetric_rate = 1,
  };


```
### Codec control IO


codec 通常可以通过 I2C SPI 风格接口控制（AC97 DAI 中将控制与数据组合在一起）。codec 驱动应对所codec IO 使用 Regmap API。有regmap 用法的示例，请参include/linux/regmap.h 和现有的 codec 驱动

### Mixers and audio controls


所codec 混音器和音频控制都可以使soc.h 中定义的便捷宏来定义```

    #define SOC_SINGLE(xname, reg, shift, mask, invert)

```
定义一个单一控制如下
```

  xname = 控制名称，例"Playback Volume"
  reg = codec 寄存  shift = 控制位在寄存器中的偏  mask = 控制位的大小，例mask 7 = 3   invert = 该控制是反转
```
其他宏包括：-
```

    #define SOC_DOUBLE(xname, reg, shift_left, shift_right, mask, invert)

```
一个立体声控制
```

    #define SOC_DOUBLE_R(xname, reg_left, reg_right, shift, mask, invert)

```
一个跨2 个寄存器的立体声控制
```

    #define SOC_ENUM_SINGLE(xreg, xshift, xmask, xtexts)

```
定义一个单一枚举控制如下
```

   xreg = 寄存   xshift = 控制位在寄存器中的偏   xmask = 控制位的大小
   xtexts = 指向描述每个设置的字符串数组的指
   #define SOC_ENUM_DOUBLE(xreg, xshift_l, xshift_r, xmask, xtexts)

```
定义一个立体声枚举控制


### Codec Audio Operations


codec 驱动还支持以ALSA PCM 操作
```

  /* SoC audio ops */
  struct snd_soc_ops {
	int (*startup)(struct snd_pcm_substream *);
	void (*shutdown)(struct snd_pcm_substream *);
	int (*hw_params)(struct snd_pcm_substream *, struct snd_pcm_hw_params *);
	int (*hw_free)(struct snd_pcm_substream *);
	int (*prepare)(struct snd_pcm_substream *);
  };

```
详情请参:doc:`ALSA 驱动 PCM 文档 <../kernel-api/writing-an-alsa-driver>`

### DAPM description


动态音频电源管理（Dynamic Audio Power Management）描述描述了 codec 电源组件及其关系，并ASoC 核心注册。构建该描述的细节请参阅 dapm.rst
也请参阅其他 codec 驱动中的示例

### DAPM event handler


此函数是一个回调，处理 codec 域的 PM 调用和系统域PM 调用（例suspend resume）。它用于codec 不使用时使其进入睡眠
电源状态：-
```

	SNDRV_CTL_POWER_D0: /* 完全开*/
	/* vref/mid、clk osc 开启，激*/

	SNDRV_CTL_POWER_D1: /* 部分开*/
	SNDRV_CTL_POWER_D2: /* 部分开*/

	SNDRV_CTL_POWER_D3hot: /* 关闭，但保留电源 */
	/* vref/vmid 外全部关闭，非激*/

	SNDRV_CTL_POWER_D3cold: /* 全部关闭，无电源 */


```
### Codec DAC digital mute control


大多codec DAC 之前有一个数字静音，可用于尽量减少任何系统噪声。静音会阻止任何数字数据进入 DAC
可以创建一个由核心在应用或释放静音时为每个 codec DAI 调用的回调
i.e.
```

  static int wm8974_mute(struct snd_soc_dai *dai, int mute, int direction)
  {
	struct snd_soc_component *component = dai->component;
	u16 mute_reg = snd_soc_component_read(component, WM8974_DAC) & 0xffbf;

	if (mute)
		snd_soc_component_write(component, WM8974_DAC, mute_reg | 0x40);
	else
		snd_soc_component_write(component, WM8974_DAC, mute_reg);
	return 0;
  }

```
