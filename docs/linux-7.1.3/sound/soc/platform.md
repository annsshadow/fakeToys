## ASoC 平台驱动


ASoC 平台驱动类可以分为音DMA 驱动、SoC DAI 驱动DSP 驱动。平台驱动只针对 SoC CPU，必须不包含任何板级特定代码
## 音频 DMA


平台 DMA 驱动可选地支持以下 ALSA 操作
```

  /* SoC audio ops */
  struct snd_soc_ops {
	int (*startup)(struct snd_pcm_substream *);
	void (*shutdown)(struct snd_pcm_substream *);
	int (*hw_params)(struct snd_pcm_substream *, struct snd_pcm_hw_params *);
	int (*hw_free)(struct snd_pcm_substream *);
	int (*prepare)(struct snd_pcm_substream *);
	int (*trigger)(struct snd_pcm_substream *, int);
  };

```
平台驱动通过 struct snd_soc_component_driver 导出DMA 功能
```

  struct snd_soc_component_driver {
	const char *name;

	...
	int (*probe)(struct snd_soc_component *);
	void (*remove)(struct snd_soc_component *);
	int (*suspend)(struct snd_soc_component *);
	int (*resume)(struct snd_soc_component *);

	/* pcm creation and destruction */
	int (*pcm_new)(struct snd_soc_pcm_runtime *);
	void (*pcm_free)(struct snd_pcm *);

	...
	const struct snd_pcm_ops *ops;
	const struct snd_compr_ops *compr_ops;
	...
  };

```
有关音频 DMA 的详细信息，请参:doc:`ALSA 驱动文档
<../kernel-api/writing-an-alsa-driver>`銆。
一DMA 驱动示例soc/pxa/pxa2xx-pcm.c


## SoC DAI 驱动


每个 SoC DAI 驱动必须提供以下特性：-

1. 数字音频接口（DAI）描2. 数字音频接口配置
3. PCM 描述
4. SYSCLK 配置
5. 挂起和恢复（可选）

关于1 - 4 项的描述，请参见 codec.rst

## SoC DSP 驱动


每个 SoC DSP 驱动通常提供以下特性：-

1. DAPM 2. 混音器控3. 进出 DSP 缓冲区的 DMA IO（如适用4. DSP 前端（FE）PCM 设备的定义
关于4 项的描述，请参见 DPCM.txt