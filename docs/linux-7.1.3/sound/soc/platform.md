## ASoC 骞冲彴椹卞姩


ASoC 骞冲彴椹卞姩绫诲彲浠ュ垎涓洪煶棰?DMA 椹卞姩銆丼oC DAI 椹卞姩鍜?DSP 椹卞姩銆傚钩鍙伴┍鍔ㄥ彧閽堝 SoC CPU锛屽繀椤讳笉鍖呭惈浠讳綍鏉跨骇鐗瑰畾浠ｇ爜銆?
## 闊抽 DMA


骞冲彴 DMA 椹卞姩鍙€夊湴鏀寔浠ヤ笅 ALSA 鎿嶄綔锛?
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
骞冲彴椹卞姩閫氳繃 struct snd_soc_component_driver 瀵煎嚭鍏?DMA 鍔熻兘锛?
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
鏈夊叧闊抽 DMA 鐨勮缁嗕俊鎭紝璇峰弬鑰?:doc:`ALSA 椹卞姩鏂囨。
<../kernel-api/writing-an-alsa-driver>`銆?
涓€涓?DMA 椹卞姩绀轰緥鏄?soc/pxa/pxa2xx-pcm.c


## SoC DAI 椹卞姩


姣忎釜 SoC DAI 椹卞姩蹇呴』鎻愪緵浠ヤ笅鐗规€э細-

1. 鏁板瓧闊抽鎺ュ彛锛圖AI锛夋弿杩?2. 鏁板瓧闊抽鎺ュ彛閰嶇疆
3. PCM 鎻忚堪
4. SYSCLK 閰嶇疆
5. 鎸傝捣鍜屾仮澶嶏紙鍙€夛級

鍏充簬绗?1 - 4 椤圭殑鎻忚堪锛岃鍙傝 codec.rst銆?

## SoC DSP 椹卞姩


姣忎釜 SoC DSP 椹卞姩閫氬父鎻愪緵浠ヤ笅鐗规€э細-

1. DAPM 鍥?2. 娣烽煶鍣ㄦ帶鍒?3. 杩涘嚭 DSP 缂撳啿鍖虹殑 DMA IO锛堝閫傜敤锛?4. DSP 鍓嶇锛團E锛塒CM 璁惧鐨勫畾涔夈€?
鍏充簬绗?4 椤圭殑鎻忚堪锛岃鍙傝 DPCM.txt銆?