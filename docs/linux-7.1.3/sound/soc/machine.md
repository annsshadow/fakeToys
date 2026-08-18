## ASoC 鏈哄櫒锛圡achine锛夐┍鍔?

ASoC 鏈哄櫒锛堟垨鏉跨骇锛夐┍鍔ㄦ槸灏嗘墍鏈夌粍浠堕┍鍔紙濡傜紪瑙ｇ爜鍣?codec銆佸钩鍙?platform 鍜?DAI锛夌矘鍚堝湪涓€璧风殑浠ｇ爜銆傚畠杩樻弿杩颁簡鍚勭粍浠朵箣闂寸殑鍏崇郴锛屽寘鎷煶棰戣矾寰勩€丟PIO銆佷腑鏂€佹椂閽熴€佹彃瀛旓紙jack锛夊拰鐢靛帇璋冭妭鍣ㄣ€?
鏈哄櫒椹卞姩鍙互鍖呭惈缂栬В鐮佸櫒鍜屽钩鍙扮浉鍏崇殑浠ｇ爜銆傚畠灏嗛煶棰戝瓙绯荤粺浣滀负骞冲彴璁惧鍚戝唴鏍告敞鍐岋紝骞剁敱浠ヤ笅 struct 琛ㄧず锛?```

  /* SoC machine */
  struct snd_soc_card {
	char *name;

	...

	int (*probe)(struct platform_device *pdev);
	int (*remove)(struct platform_device *pdev);

	/* pre 鍜?post PM 鍑芥暟鐢ㄤ簬鍦?codec 鍜?DAI 杩涜浠讳綍 PM 宸ヤ綔涔嬪墠鍜屼箣鍚庡畬鎴?PM 鐩稿叧宸ヤ綔銆?*/
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

probe/remove 鏄彲閫夌殑銆傚湪姝ゅ瀹屾垚浠讳綍鏈哄櫒鐩稿叧鐨勬帰娴嬨€?
### suspend()/resume()

鏈哄櫒椹卞姩鍏锋湁 suspend 鍜?resume 鐨?pre 涓?post 鐗堟湰锛岀敤浜庣収椤惧湪 codec銆丏AI 鍜?DMA 鎸傝捣鍜屾仮澶嶅墠鍚庡繀椤诲畬鎴愮殑鏈哄櫒闊抽浠诲姟銆傚彲閫夈€?
### 鏈哄櫒 DAI 閰嶇疆

鏈哄櫒 DAI 閰嶇疆灏嗘墍鏈?codec 鍜?CPU DAI 绮樺悎鍦ㄤ竴璧枫€傚畠涔熷彲鐢ㄤ簬璁剧疆 DAI 绯荤粺鏃堕挓锛屼互鍙婅繘琛屼换浣曚笌鏈哄櫒鐩稿叧鐨?DAI 鍒濆鍖栵紝渚嬪鏈哄櫒闊抽鏄犲皠鍙繛鎺ュ埌 codec 闊抽鏄犲皠锛屾湭杩炴帴鐨?codec 寮曡剼鍙浉搴旇缃€?
struct snd_soc_dai_link 鐢ㄤ簬璁剧疆鏈哄櫒涓殑姣忎釜 DAI銆備緥濡傦細
```

  /* corgi 鏁板瓧闊抽鎺ュ彛绮樺悎 - 杩炴帴 codec <--> CPU */
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
鍦ㄤ笂杩?struct 涓紝DAI 浣跨敤鍚嶇О娉ㄥ唽锛屼絾浣犲彲浠ヤ紶鍏?DAI 鍚嶇О鎴栬澶囨爲鑺傜偣锛屼笉鑳藉悓鏃朵紶鍏ヤ袱鑰呫€傛澶栵紝杩欓噷鐢ㄤ簬 cpu/codec/platform DAI 鐨勫悕绉板簲鍏ㄥ眬鍞竴銆?
姝ゅ锛屼笅闈㈢殑绀轰緥瀹忓彲鐢ㄤ簬娉ㄥ唽 cpu銆乧odec 鍜?```

  SND_SOC_DAILINK_DEFS(wm2200_cpu_dsp,
	DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
	DAILINK_COMP_ARRAY(COMP_CODEC("spi0.0", "wm0010-sdi1")),
	DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));

```
struct snd_soc_card 闅忓悗鐢ㄥ叾 DAI 璁剧疆鏈哄櫒銆備緥濡傦細
```

  /* corgi 闊抽鏈哄櫒椹卞姩 */
  static struct snd_soc_card snd_soc_corgi = {
	.name = "Corgi",
	.dai_link = &corgi_dai,
	.num_links = 1,
  };

```
涔嬪悗锛屽彲浣跨敤 `devm_snd_soc_register_card` 娉ㄥ唽澹板崱銆傚湪娉ㄥ唽杩囩▼涓紝浼氭帰娴?codec銆丆PU 鍜?platform 绛夊悇涓粍浠躲€傚鏋滆繖浜涚粍浠堕兘鎴愬姛琚帰娴嬶紝澹板崱鍗宠娉ㄥ唽銆?
### 鏈哄櫒鐢垫簮鏄犲皠

鏈哄櫒椹卞姩鍙互閫夋嫨鎬у湴鎵╁睍 codec 鐢垫簮鏄犲皠锛屾垚涓洪煶棰戝瓙绯荤粺鐨勯煶棰戠數婧愭槧灏勩€傝繖鍏佽鎵０鍣?鑰虫満鏀惧ぇ鍣ㄧ瓑鐨勮嚜鍔ㄤ笂鐢?鏂數銆俢odec 寮曡剼鍙湪鏈哄櫒鍒濆鍖栧嚱鏁颁腑杩炴帴鍒版満鍣ㄧ殑鎻掑瓟鎻掑骇銆?
### 鏈哄櫒鎺у埗

鍙湪 DAI 鍒濆鍖栧嚱鏁颁腑娣诲姞鏈哄櫒鐩稿叧鐨勯煶棰戞贩闊冲櫒鎺у埗銆?
### 鏃堕挓鎺у埗

濡傚墠鎵€杩帮紝鏃堕挓閰嶇疆鍦ㄦ満鍣ㄩ┍鍔ㄥ唴澶勭悊銆傚叧浜庢満鍣ㄩ┍鍔ㄥ彲鐢ㄤ簬璁剧疆鐨勬椂閽?API 鐨勭粏鑺傦紝璇峰弬闃?Documentation/sound/soc/clocking.rst銆備絾鏄紝鍥炶皟闇€瑕佺敱 CPU/Codec/Platform 椹卞姩娉ㄥ唽锛屼互閰嶇疆鐩稿簲璁惧鎿嶄綔鎵€闇€鐨勬椂閽熴€?