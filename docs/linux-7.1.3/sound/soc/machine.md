## ASoC 机器（Machine）驱

ASoC 机器（或板级）驱动是将所有组件驱动（如编解码codec、平platform DAI）粘合在一起的代码。它还描述了各组件之间的关系，包括音频路径、GPIO、中断、时钟、插孔（jack）和电压调节器
机器驱动可以包含编解码器和平台相关的代码。它将音频子系统作为平台设备向内核注册，并由以下 struct 表示```

  /* SoC machine */
  struct snd_soc_card {
	char *name;

	...

	int (*probe)(struct platform_device *pdev);
	int (*remove)(struct platform_device *pdev);

	/* pre post PM 函数用于codec DAI 进行任何 PM 工作之前和之后完PM 相关工作*/
	int (*suspend_pre)(struct platform_device *pdev, pm_message_t state);
	int (*suspend_post)(struct platform_device *pdev, pm_message_t state);
	int (*resume_pre)(struct platform_device *pdev);
	int (*resume_post)(struct platform_device *pdev);

	...

	/* CPU <--> Codec DAI links  */
	struct snd_soc_dai_link *dai_link;
	int num_links;

	...
  };

```
### probe()/remove()

probe/remove 是可选的。在此处完成任何机器相关的探测
### suspend()/resume()

机器驱动具有 suspend resume pre post 版本，用于照顾在 codec、DAI DMA 挂起和恢复前后必须完成的机器音频任务。可选
### 机器 DAI 配置

机器 DAI 配置将所codec CPU DAI 粘合在一起。它也可用于设置 DAI 系统时钟，以及进行任何与机器相关DAI 初始化，例如机器音频映射可连接到 codec 音频映射，未连接codec 引脚可相应设置
struct snd_soc_dai_link 用于设置机器中的每个 DAI。例如：
```

  /* corgi 数字音频接口粘合 - 连接 codec <--> CPU */
  static struct snd_soc_dai_link corgi_dai = {
	.name = "WM8731",
	.stream_name = "WM8731",
	.cpu_dai_name = "pxa-is2-dai",
	.codec_dai_name = "wm8731-hifi",
	.platform_name = "pxa-pcm-audio",
	.codec_name = "wm8713-codec.0-001a",
	.init = corgi_wm8731_init,
	.ops = &corgi_ops,
  };

```
在上struct 中，DAI 使用名称注册，但你可以传DAI 名称或设备树节点，不能同时传入两者。此外，这里用于 cpu/codec/platform DAI 的名称应全局唯一
此外，下面的示例宏可用于注册 cpu、codec ```

  SND_SOC_DAILINK_DEFS(wm2200_cpu_dsp,
	DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
	DAILINK_COMP_ARRAY(COMP_CODEC("spi0.0", "wm0010-sdi1")),
	DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));

```
struct snd_soc_card 随后用其 DAI 设置机器。例如：
```

  /* corgi 音频机器驱动 */
  static struct snd_soc_card snd_soc_corgi = {
	.name = "Corgi",
	.dai_link = &corgi_dai,
	.num_links = 1,
  };

```
之后，可使用 `devm_snd_soc_register_card` 注册声卡。在注册过程中，会探codec、CPU platform 等各个组件。如果这些组件都成功被探测，声卡即被注册
### 机器电源映射

机器驱动可以选择性地扩展 codec 电源映射，成为音频子系统的音频电源映射。这允许扬声耳机放大器等的自动上断电。codec 引脚可在机器初始化函数中连接到机器的插孔插座
### 机器控制

可在 DAI 初始化函数中添加机器相关的音频混音器控制
### 时钟控制

如前所述，时钟配置在机器驱动内处理。关于机器驱动可用于设置的时API 的细节，请参Documentation/sound/soc/clocking.rst。但是，回调需要由 CPU/Codec/Platform 驱动注册，以配置相应设备操作所需的时钟