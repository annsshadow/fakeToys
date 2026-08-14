## 鍔ㄦ€?PCM


## 鎻忚堪


鍔ㄦ€?PCM锛圖ynamic PCM锛夊厑璁镐竴涓?ALSA PCM 璁惧鍦?PCM 娴佽繍琛屾湡闂村皢鍏?PCM 闊抽鏁板瓧璺敱鍒?鍚勭鏁板瓧绔偣銆備緥濡傦紝PCM0 鍙互灏嗘暟瀛楅煶棰戣矾鐢卞埌 I2S DAI0銆両2S DAI1 鎴?PDM DAI2銆傝繖瀵逛簬
鏆撮湶澶氫釜 ALSA PCM 涓斿彲璺敱鍒板涓?DAI 鐨勭墖涓?SoC DSP 椹卞姩寰堟湁鐢ㄣ€?
DPCM 杩愯鏃惰矾鐢辩敱 ALSA 娣烽煶鍣ㄨ缃喅瀹氾紝鏂瑰紡涓庡湪 ASoC 缂栬В鐮佸櫒椹卞姩涓矾鐢辨ā鎷熶俊鍙风浉鍚屻€?DPCM 浣跨敤涓€涓唬琛?DSP 鍐呴儴闊抽璺緞鐨?DAPM 鍥撅紝骞跺埄鐢ㄦ贩闊冲櫒璁剧疆鏉ョ‘瀹氭瘡涓?ALSA PCM 鎵€浣跨敤鐨?璺緞銆?
DPCM 澶嶇敤鎵€鏈夌幇鏈夌殑缁勪欢缂栬В鐮佸櫒銆佸钩鍙颁互鍙?DAI 椹卞姩锛屾棤闇€浠讳綍淇敼銆?

### 甯?SoC DSP 鐨勬墜鏈洪煶棰戠郴缁?

鑰冭檻濡備笅鎵嬫満闊抽瀛愮郴缁熴€傛湰鏂囧皢鐢ㄥ畠鏉ュ睍绀烘墍鏈夌ず渚嬶細

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

璇ュ浘灞曠ず浜嗕竴涓畝鍗曠殑鏅鸿兘鎵嬫満闊抽瀛愮郴缁熴€傚畠鏀寔钃濈墮銆丗M 鏁板瓧鏀堕煶鏈恒€佹壃澹板櫒銆佽€虫満鎻掑瓟銆?鏁板瓧楹﹀厠椋庝互鍙婅渹绐濊皟鍒惰В璋冨櫒銆傛澹板崱鏆撮湶 4 涓?DSP 鍓嶇锛團E锛堿LSA PCM 璁惧锛屽苟鏀寔 6 涓?鍚庣锛圔E锛塂AI銆傛瘡涓?FE PCM 閮藉彲浠ュ皢闊抽鏁版嵁鏁板瓧璺敱鍒颁换鎰?BE DAI銆侳E PCM 璁惧涔熷彲浠ュ皢
闊抽璺敱鍒板浜?1 涓?BE DAI銆?

### 绀轰緥 - 灏嗘挱鏀句粠 DAI0 鍒囨崲鍒?DAI1 鐨?DPCM


闊抽姝ｅ湪鎾斁鍒拌€虫満銆傝繃浜嗕竴浼氬効鐢ㄦ埛鎷斾笅鑰虫満锛岄煶棰戠户缁湪鎵０鍣ㄤ笂鎾斁銆?
PCM0 鍒拌€虫満鐨勬挱鏀剧湅璧锋潵濡備笅锛?
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

鐢ㄦ埛浠庢彃瀛斾腑鎷斿嚭鑰虫満锛屽洜姝ょ幇鍦ㄥ繀椤讳娇鐢ㄦ壃澹板櫒锛?
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

闊抽椹卞姩鎸夊涓嬫柟寮忓鐞嗭細

1. 鏈哄櫒椹卞姩鏀跺埌鎻掑瓟鎷斿嚭浜嬩欢銆?
2. 鏈哄櫒椹卞姩鎴栭煶棰?HAL 绂佺敤鑰虫満璺緞銆?
3. 鐢变簬璺緞鐜板凡绂佺敤锛孌PCM 鍦?DAI0 涓婂鑰虫満杩愯 PCM trigger(stop)銆乭w_free()銆乻hutdown()
   鎿嶄綔銆?
4. 鏈哄櫒椹卞姩鎴栭煶棰?HAL 鍚敤鎵０鍣ㄨ矾寰勩€?
5. 鐢变簬璺緞宸插惎鐢紝DPCM 鍦?DAI1 鎵０鍣ㄤ笂杩愯 startup()銆乭w_params()銆乸repare() 浠ュ強
   trigger(start) 杩欎簺 PCM 鎿嶄綔銆?
鍦ㄦ绀轰緥涓紝鏈哄櫒椹卞姩鎴栫敤鎴风┖闂撮煶棰?HAL 鍙互鏀瑰彉璺敱锛岀劧鍚?DPCM 浼氳礋璐ｇ鐞嗕笌 DAI PCM
鐩稿叧鐨勬搷浣滐紝浠ヤ娇閾捐矾 up 鎴?down銆傚湪姝よ浆鎹㈡湡闂撮煶棰戞挱鏀句笉浼氬仠姝€?

## DPCM 鏈哄櫒椹卞姩


鏀寔 DPCM 鐨?ASoC 鏈哄櫒椹卞姩涓庢櫘閫氱殑鏈哄櫒椹卞姩绫讳技锛屽彧鏄垜浠繕蹇呴』锛?
1. 瀹氫箟 FE 鍜?BE DAI 閾捐矾銆?
2. 瀹氫箟浠讳綍 FE/BE PCM 鎿嶄綔銆?
3. 瀹氫箟閮ㄤ欢鍥捐繛鎺ャ€?

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

瀵逛簬涓婇潰鐨勭ず渚嬶紝鎴戜滑蹇呴』瀹氫箟 4 涓?FE DAI 閾捐矾鍜?6 涓?BE DAI 閾捐矾銆侳E DAI 閾捐矾瀹氫箟濡備笅锛?
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

姝?FE DAI 閾捐矾涓庡父瑙?DAI 閾捐矾闈炲父鐩镐技锛屽彧鏄垜浠繕閫氳繃璁剧疆 `dynamic = 1` 灏嗚 DAI 閾捐矾
鏍囪涓?DPCM FE銆傝繕鏈変竴涓€夐」鍙互鎸囧畾姣忎釜 FE 鐨?trigger 璋冪敤椤哄簭銆傝繖鍏佽 ASoC 鏍稿績鍦?鍏朵粬缁勪欢涔嬪墠鎴栦箣鍚庤Е鍙?DSP锛堝洜涓烘煇浜?DSP 瀵?DAI/DSP 鍚姩鍜屽仠姝㈠簭鍒楃殑椤哄簭鏈変弗鏍艰姹傦級銆?
涓婇潰鐨?FE DAI 灏嗙紪瑙ｇ爜鍣ㄥ拰 codec DAI 璁剧疆涓鸿櫄鎷熻澶囷紝鍥犱负 BE 鏄姩鎬佺殑锛屽苟浼氶殢杩愯鏃堕厤缃?鑰屾敼鍙樸€?
BE DAI 閰嶇疆濡備笅锛?
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

姝?BE DAI 閾捐矾灏?DAI0 杩炴帴鍒扮紪瑙ｇ爜鍣紙鏈緥涓负 RT5460 AIF1锛夈€傚畠璁剧疆 `no_pcm` 鏍囧織浠?灏嗗叾鏍囪涓?BE銆?
BE 杩樿缃簡鐢ㄤ簬蹇界暐鎸傝捣鍜?PM down 鏃堕棿鐨勬爣蹇椼€傝繖鍏佽 BE 浠ユ棤涓绘満锛坔ostless锛夋ā寮忓伐浣滐紝
鍗冲湪涓绘満 CPU 涓嶄紶杈撴暟鎹紙濡傝摑鐗欑數璇濆懠鍙級鐨勬儏鍐典笅锛?
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

杩欏厑璁镐富鏈?CPU 鍦?DSP銆丮ODEM DAI 鍜?BT DAI 浠嶅湪杩愯鏃朵紤鐪犮€?
濡傛灉缂栬В鐮佸櫒鏄敱澶栭儴绠＄悊鐨勮澶囷紝BE DAI 閾捐矾涔熷彲浠ュ皢缂栬В鐮佸櫒璁剧疆涓鸿櫄鎷熻澶囥€?
鍚屾牱锛屽鏋?CPU DAI 鐢?DSP 鍥轰欢绠＄悊锛孊E DAI 涔熷彲浠ュ皢铏氭嫙 cpu DAI 璁剧疆涓鸿櫄鎷熻澶囥€?

### FE/BE PCM 鎿嶄綔


涓婇潰鐨?BE 杩樺鍑轰簡涓€浜?PCM 鎿嶄綔鍜屼竴涓?`fixup` 鍥炶皟銆傛満鍣ㄩ┍鍔ㄤ娇鐢ㄨ fixup 鍥炶皟鍩轰簬 FE
纭欢鍙傛暟锛坔w params锛夋潵锛堥噸鏂帮級閰嶇疆 DAI銆傚嵆 DSP 鍙互鍦?FE 鍒?BE 涔嬮棿鎵ц SRC 鎴?ASRC銆?
渚嬪锛孌SP 灏嗘墍鏈?FE hw params 杞崲涓轰互鍥哄畾鐨?48k銆?6bit銆佺珛浣撳０杩愯浜?DAI0銆傝繖鎰忓懗鐫€鎵€鏈?FE hw_params 閮藉繀椤诲湪鏈哄櫒椹卞姩涓负 DAI0 淇锛屼互渚?DAI 浠ユ湡鏈涚殑閰嶇疆杩愯锛岃€屼笉绠?FE 鐨勯厤缃?濡備綍銆?
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

鍏朵粬 PCM 鎿嶄綔涓庡父瑙?DAI 閾捐矾鐩稿悓銆傛寜闇€浣跨敤銆?

### 閮ㄤ欢鍥捐繛鎺?

BE DAI 閾捐矾閫氬父浼氬湪鍒濆鍖栨椂鐢?ASoC DAPM 鏍稿績杩炴帴鍒板浘涓€備絾鏄紝濡傛灉 BE 缂栬В鐮佸櫒鎴?BE DAI
鏄櫄鎷熺殑锛屽垯蹇呴』鍦ㄩ┍鍔ㄤ腑鏄惧紡璁剧疆锛?
```

  /* BE for codec Headset -  DAI0 is dummy and managed by DSP FW */
  {"DAI0 CODEC IN", NULL, "AIF1 Capture"},
  {"AIF1 Playback", NULL, "DAI0 CODEC OUT"},


```

## 缂栧啓 DPCM DSP 椹卞姩


DPCM DSP 椹卞姩鐪嬭捣鏉ュ緢鍍忔爣鍑嗙殑骞冲彴绫?ASoC 椹卞姩锛岃瀺鍚堜簡缂栬В鐮佸櫒绫婚┍鍔ㄧ殑鍏冪礌銆備竴涓?DSP 骞冲彴
椹卞姩蹇呴』瀹炵幇锛?
1. 鍓嶇 PCM DAI鈥斺€斿嵆 struct snd_soc_dai_driver銆?
2. 鏄剧ず浠?FE DAI 鍒?BE 鐨?DSP 闊抽璺敱鐨?DAPM 鍥俱€?
3. 鏉ヨ嚜 DSP 鍥剧殑 DAPM 閮ㄤ欢銆?
4. 鐢ㄤ簬澧炵泭銆佽矾鐢辩瓑鐨勬贩闊冲櫒銆?
5. DMA 閰嶇疆銆?
6. BE AIF 閮ㄤ欢銆?
绗?6 椤瑰浜庡皢闊抽璺敱鍒?DSP 澶栭儴寰堥噸瑕併€傞渶瑕佷负姣忎釜 BE 鍜屾瘡涓祦鏂瑰悜瀹氫箟 AIF銆備緥濡傦紝瀵逛簬涓婇潰鐨?BE DAI0锛屾垜浠皢鏈夛細

```

  SND_SOC_DAPM_AIF_IN("DAI0 RX", NULL, 0, SND_SOC_NOPM, 0, 0),
  SND_SOC_DAPM_AIF_OUT("DAI0 TX", NULL, 0, SND_SOC_NOPM, 0, 0),

```

BE AIF 鐢ㄤ簬灏?DSP 鍥捐繛鎺ュ埌鍏朵粬缁勪欢椹卞姩锛堜緥濡傜紪瑙ｇ爜鍣ㄥ浘锛夌殑鍥俱€?

## 鏃犱富鏈?PCM 娴?

鏃犱富鏈?PCM 娴佹槸涓嶇粡杩囦富鏈?CPU 璺敱鐨勬祦銆傝繖鏂归潰鐨勪竴涓緥瀛愭槸浠庢墜鏈哄埌璋冨埗瑙ｈ皟鍣ㄧ殑鐢佃瘽鍛煎彨銆?
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

鍦ㄨ繖绉嶆儏鍐典笅锛孭CM 鏁版嵁閫氳繃 DSP 璺敱銆備富鏈?CPU 鍦ㄦ鐢ㄤ緥涓粎鐢ㄤ簬鎺у埗锛屽苟鍙湪娴佽繍琛屾湡闂翠紤鐪犮€?
涓绘満鍙互閫氳繃浠ヤ笅涓ょ鏂瑰紡鎺у埗鏃犱富鏈洪摼璺細

  1. 灏嗛摼璺厤缃负 CODEC <-> CODEC 椋庢牸鐨勯摼璺€傚湪杩欑鎯呭喌涓嬶紝閾捐矾鐢?DAPM 鍥剧殑鐘舵€佸惎鐢ㄦ垨
     绂佺敤銆傝繖閫氬父鎰忓懗鐫€鏈変竴涓贩闊冲櫒鎺т欢鍙敤浜庤繛鎺ユ垨鏂紑涓や釜 DAI 涔嬮棿鐨勮矾寰勩€?
  2. 鏃犱富鏈?FE銆傛 FE 鍦?DAPM 鍥句笂涓?BE DAI 閾捐矾鍏锋湁铏氭嫙杩炴帴銆傞殢鍚庡儚甯歌 PCM 鎿嶄綔涓€鏍风敱
     FE 鎵ц鎺у埗銆傛鏂规硶瀵?DAI 閾捐矾鎻愪緵鏇村己鐨勬帶鍒讹紝浣嗛渶瑕佸寰楀鐨勭敤鎴风┖闂翠唬鐮佹潵鎺у埗閾捐矾銆?     闄ら潪浣犵殑纭欢闇€瑕佸 PCM 鎿嶄綔杩涜鏇寸粏绮掑害鐨勬帓搴忥紝鍚﹀垯寤鸿浣跨敤 CODEC<->CODEC銆?

### CODEC <-> CODEC 閾捐矾


褰?DAPM 妫€娴嬪埌 DAPM 鍥惧唴鐨勬湁鏁堣矾寰勬椂锛屾 DAI 閾捐矾琚惎鐢ㄣ€傛満鍣ㄩ┍鍔ㄥ悜 DAI 閾捐矾璁剧疆涓€浜涢檮鍔?鍙傛暟锛屽嵆锛?
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

褰?DAPM 妫€娴嬪埌鏈夋晥璺緞骞堕殢鍚庤皟鐢?PCM 鎿嶄綔浠ュ惎鍔ㄩ摼璺椂锛屽皢浣跨敤杩欎簺鍙傛暟鏉ラ厤缃?DAI hw_params()銆?褰撹矾寰勪笉鍐嶆湁鏁堟椂锛孌APM 涔熶細璋冪敤閫傚綋鐨?PCM 鎿嶄綔鏉ョ鐢?DAI銆?

### 鏃犱富鏈?FE


DAI 閾捐矾鐢变竴涓笉璇诲彇鎴栧啓鍏ヤ换浣?PCM 鏁版嵁鐨?FE 鍚敤銆傝繖鎰忓懗鐫€鍒涘缓涓€涓笌涓や釜 DAI 閾捐矾鍏锋湁
铏氭嫙杩炴帴鐨勬柊 FE銆傚綋 FE PCM 鍚姩鏃讹紝DAI 閾捐矾灏嗗惎鍔紱褰?FE PCM 鍋滄鏃讹紝DAI 閾捐矾灏嗗仠姝€傛敞鎰忥紝
鍦ㄦ閰嶇疆涓?FE PCM 涓嶈兘璇诲彇鎴栧啓鍏ユ暟鎹€?