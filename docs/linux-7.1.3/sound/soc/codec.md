## ASoC Codec Class Driver


codec 绫婚┍鍔ㄦ槸閫氱敤涓斾笌纭欢鏃犲叧鐨勪唬鐮侊紝鐢ㄤ簬閰嶇疆 codec銆丗M銆丮ODEM銆丅T 鎴栧閮?DSP锛屼互鎻愪緵闊抽閲囬泦鍜屽洖鏀俱€傚畠涓嶅簲鍖呭惈浠讳綍鐗瑰畾浜庣洰鏍囧钩鍙版垨鏈哄櫒鐨勪唬鐮併€傛墍鏈夊钩鍙板拰鏈哄櫒鐗瑰畾鐨勪唬鐮佸簲鍒嗗埆娣诲姞鍒板钩鍙板拰鏈哄櫒椹卞姩涓€?
姣忎釜 codec 绫婚┍鍔?*蹇呴』**鎻愪緵浠ヤ笅鐗规€э細-

1. Codec DAI 鍜?PCM 閰嶇疆
2. Codec 鎺у埗 IO - 浣跨敤 RegMap API
3. 娣烽煶鍣紙Mixer锛夊拰闊抽鎺у埗
4. Codec 闊抽鎿嶄綔
5. DAPM 鎻忚堪銆?6. DAPM 浜嬩欢澶勭悊绋嬪簭銆?
鍙€夊湴锛宑odec 椹卞姩杩樺彲浠ユ彁渚涳細-

7. DAC 鏁板瓧闈欓煶锛坉igital mute锛夋帶鍒躲€?
鏈€濂藉皢姝ゆ寚鍗椾笌 sound/soc/codecs/ 涓幇鏈夌殑 codec 椹卞姩浠ｇ爜缁撳悎浣跨敤銆?
## ASoC Codec driver breakdown


### Codec DAI and PCM configuration


姣忎釜 codec 椹卞姩蹇呴』鏈変竴涓?struct snd_soc_dai_driver 鏉ュ畾涔夊叾 DAI 鍜?PCM 鑳藉姏鍙婃搷浣溿€傝缁撴瀯琚鍑猴紝浠ヤ究浣犵殑鏈哄櫒椹卞姩鍙互灏嗗叾娉ㄥ唽鍒版牳蹇冦€?
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


codec 閫氬父鍙互閫氳繃 I2C 鎴?SPI 椋庢牸鎺ュ彛鎺у埗锛圓C97 鍦?DAI 涓皢鎺у埗涓庢暟鎹粍鍚堝湪涓€璧凤級銆俢odec 椹卞姩搴斿鎵€鏈?codec IO 浣跨敤 Regmap API銆傛湁鍏?regmap 鐢ㄦ硶鐨勭ず渚嬶紝璇峰弬闃?include/linux/regmap.h 鍜岀幇鏈夌殑 codec 椹卞姩銆?

### Mixers and audio controls


鎵€鏈?codec 娣烽煶鍣ㄥ拰闊抽鎺у埗閮藉彲浠ヤ娇鐢?soc.h 涓畾涔夌殑渚挎嵎瀹忔潵瀹氫箟銆?```

    #define SOC_SINGLE(xname, reg, shift, mask, invert)

```
瀹氫箟涓€涓崟涓€鎺у埗濡備笅锛?
```

  xname = 鎺у埗鍚嶇О锛屼緥濡?"Playback Volume"
  reg = codec 瀵勫瓨鍣?  shift = 鎺у埗浣嶅湪瀵勫瓨鍣ㄤ腑鐨勫亸绉?  mask = 鎺у埗浣嶇殑澶у皬锛屼緥濡?mask 涓?7 = 3 浣?  invert = 璇ユ帶鍒舵槸鍙嶈浆鐨?
```
鍏朵粬瀹忓寘鎷細-
```

    #define SOC_DOUBLE(xname, reg, shift_left, shift_right, mask, invert)

```
涓€涓珛浣撳０鎺у埗
```

    #define SOC_DOUBLE_R(xname, reg_left, reg_right, shift, mask, invert)

```
涓€涓法瓒?2 涓瘎瀛樺櫒鐨勭珛浣撳０鎺у埗
```

    #define SOC_ENUM_SINGLE(xreg, xshift, xmask, xtexts)

```
瀹氫箟涓€涓崟涓€鏋氫妇鎺у埗濡備笅锛?
```

   xreg = 瀵勫瓨鍣?   xshift = 鎺у埗浣嶅湪瀵勫瓨鍣ㄤ腑鐨勫亸绉?   xmask = 鎺у埗浣嶇殑澶у皬
   xtexts = 鎸囧悜鎻忚堪姣忎釜璁剧疆鐨勫瓧绗︿覆鏁扮粍鐨勬寚閽?
   #define SOC_ENUM_DOUBLE(xreg, xshift_l, xshift_r, xmask, xtexts)

```
瀹氫箟涓€涓珛浣撳０鏋氫妇鎺у埗


### Codec Audio Operations


codec 椹卞姩杩樻敮鎸佷互涓?ALSA PCM 鎿嶄綔锛?
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
璇︽儏璇峰弬闃?:doc:`ALSA 椹卞姩 PCM 鏂囨。 <../kernel-api/writing-an-alsa-driver>`銆?

### DAPM description


鍔ㄦ€侀煶棰戠數婧愮鐞嗭紙Dynamic Audio Power Management锛夋弿杩版弿杩颁簡 codec 鐢垫簮缁勪欢鍙婂叾鍏崇郴锛屽苟鍚?ASoC 鏍稿績娉ㄥ唽銆傛瀯寤鸿鎻忚堪鐨勭粏鑺傝鍙傞槄 dapm.rst銆?
涔熻鍙傞槄鍏朵粬 codec 椹卞姩涓殑绀轰緥銆?

### DAPM event handler


姝ゅ嚱鏁版槸涓€涓洖璋冿紝澶勭悊 codec 鍩熺殑 PM 璋冪敤鍜岀郴缁熷煙鐨?PM 璋冪敤锛堜緥濡?suspend 鍜?resume锛夈€傚畠鐢ㄤ簬鍦?codec 涓嶄娇鐢ㄦ椂浣垮叾杩涘叆鐫＄湢銆?
鐢垫簮鐘舵€侊細-
```

	SNDRV_CTL_POWER_D0: /* 瀹屽叏寮€鍚?*/
	/* vref/mid銆乧lk 鍜?osc 寮€鍚紝婵€娲?*/

	SNDRV_CTL_POWER_D1: /* 閮ㄥ垎寮€鍚?*/
	SNDRV_CTL_POWER_D2: /* 閮ㄥ垎寮€鍚?*/

	SNDRV_CTL_POWER_D3hot: /* 鍏抽棴锛屼絾淇濈暀鐢垫簮 */
	/* 闄?vref/vmid 澶栧叏閮ㄥ叧闂紝闈炴縺娲?*/

	SNDRV_CTL_POWER_D3cold: /* 鍏ㄩ儴鍏抽棴锛屾棤鐢垫簮 */


```
### Codec DAC digital mute control


澶у鏁?codec 鍦?DAC 涔嬪墠鏈変竴涓暟瀛楅潤闊筹紝鍙敤浜庡敖閲忓噺灏戜换浣曠郴缁熷櫔澹般€傞潤闊充細闃绘浠讳綍鏁板瓧鏁版嵁杩涘叆 DAC銆?
鍙互鍒涘缓涓€涓敱鏍稿績鍦ㄥ簲鐢ㄦ垨閲婃斁闈欓煶鏃朵负姣忎釜 codec DAI 璋冪敤鐨勫洖璋冦€?
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
