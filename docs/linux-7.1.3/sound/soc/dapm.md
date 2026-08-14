## 闈㈠悜渚挎惡璁惧鐨勫姩鎬侀煶棰戠數婧愮鐞嗭紙DAPM锛?
## 鎻忚堪

鍔ㄦ€侀煶棰戠數婧愮鐞嗭紙Dynamic Audio Power Management锛孌APM锛夋棬鍦ㄨ渚挎惡 Linux 璁惧
濮嬬粓鍦ㄩ煶棰戝瓙绯荤粺涓娇鐢ㄦ渶灏戠殑鍔熻€椼€傚畠鐙珛浜庡叾浠栧唴鏍哥數婧愮鐞嗘鏋讹紝鍥犳鍙互杞绘澗
鍦颁笌瀹冧滑鍏卞瓨銆?
DAPM 瀵规墍鏈夌敤鎴风┖闂村簲鐢ㄧ▼搴忎篃瀹屽叏閫忔槑锛屽洜涓烘墍鏈夌殑鐢垫簮鍒囨崲閮藉湪 ASoC 鏍稿績鍐呴儴
瀹屾垚銆傜敤鎴风┖闂村簲鐢ㄧ▼搴忎笉闇€瑕佷慨鏀逛唬鐮佹垨閲嶆柊缂栬瘧銆侱APM 鏍规嵁浠讳綍闊抽娴侊紙閲囬泦/鎾斁锛?鐨勬椿鍔ㄤ互鍙婅澶囧唴鐨勯煶棰戞贩闊冲櫒璁剧疆鏉ュ仛鍑虹數婧愬垏鎹㈠喅绛栥€?
DAPM 鍩轰簬涓や釜鍩烘湰鍏冪礌锛岀О涓?widget锛堥儴浠讹級鍜?route锛堣矾鐢憋級锛?
 - **widget** 鏄煶棰戠‖浠剁殑姣忎竴涓儴鍒嗭紝鍦ㄤ娇鐢ㄦ椂鍙敱杞欢鍚敤锛屼笉浣跨敤鏃剁鐢ㄤ互鐪佺數
 - **route** 鏄?widget 涔嬮棿鐨勪簰杩烇紝褰撳０闊宠兘澶熶粠涓€涓?widget 娴佸悜鍙︿竴涓?widget 鏃跺瓨鍦?
鎵€鏈?DAPM 鐢垫簮鍒囨崲鍐崇瓥閮芥槸閫氳繃鏌ヨ闊抽璺敱鍥捐嚜鍔ㄥ仛鍑虹殑銆傝繖涓浘瀵规瘡涓０鍗￠兘鏄?鐗瑰畾鐨勶紝骞朵笖璺ㄨ秺鏁翠釜澹板崱锛屽洜姝ゆ煇浜?DAPM 璺敱浼氳繛鎺ュ睘浜庝笉鍚岀粍浠剁殑 widget锛堜緥濡?涓€涓?CODEC 鐨?LINE OUT 寮曡剼涓庝竴涓斁澶у櫒鐨勮緭鍏ュ紩鑴氾級銆?
STM32MP1-DK1 澹板崱鐨勮矾鐢卞浘濡備笅鎵€绀猴細

    :alt:   Example DAPM graph
    :align: center

浣犱篃鍙互浣跨敤 `tools/sound/dapm-graph` 宸ュ叿涓轰綘鐨勫０鍗＄敓鎴愬吋瀹圭殑鍥俱€?
## DAPM 鐢垫簮鍩?
DAPM 鍐呴儴鏈?4 涓數婧愬煙锛?
Codec 鍋忕疆锛坆ias锛夊煙
      VREF銆乂MID锛堟牳蹇?codec 涓庨煶棰戠數婧愶級

      閫氬父鍦?codec 鎺㈡祴/绉婚櫎浠ュ強鎸傝捣/鎭㈠鏃舵帶鍒讹紝灏界濡傛灉渚ч煶绛変笉闇€瑕佺數婧愭椂涔熷彲浠ュ湪
      娴佹椂闂磋缃€?
骞冲彴/鏈哄櫒鍩?      鐗╃悊杩炴帴鐨勮緭鍏ヤ笌杈撳嚭

      鍙栧喅浜庡钩鍙?鏈哄櫒浠ュ強鐢ㄦ埛鎿嶄綔锛岀敱鏈哄櫒椹卞姩閰嶇疆锛屽苟鍝嶅簲寮傛浜嬩欢锛屼緥濡傛彃鍏ヨ€虫満锛圚P锛夋椂銆?
璺緞鍩?      闊抽瀛愮郴缁熶俊鍙疯矾寰?
      鍦ㄧ敤鎴锋洿鏀规贩闊冲櫒鍜屽璺鐢紙mux锛夎缃椂鑷姩璁剧疆銆備緥濡?alsamixer銆乤mixer銆?
娴佸煙
      DAC 鍜?ADC銆?
      鍦ㄦ祦鎾斁/閲囬泦鍒嗗埆寮€濮嬪拰鍋滄鏃跺惎鐢ㄥ拰绂佺敤銆備緥濡?aplay銆乤record銆?
## DAPM Widgets

闊抽 DAPM widget 鍙垎涓鸿嫢骞茬被鍨嬶細

Mixer
	灏嗚嫢骞叉ā鎷熶俊鍙锋贩鍚堜负鍗曚釜妯℃嫙淇″彿銆?Mux
	涓€涓彧杈撳嚭澶氫釜杈撳叆涓煇涓€涓殑妯℃嫙寮€鍏炽€?PGA
	涓€涓彲缂栫▼澧炵泭鏀惧ぇ鍣ㄦ垨琛板噺 widget銆?ADC
	妯℃暟杞崲鍣紙Analog to Digital Converter锛?DAC
	鏁版ā杞崲鍣紙Digital to Analog Converter锛?Switch
	涓€涓ā鎷熷紑鍏?Input
	涓€涓?codec 杈撳叆寮曡剼
Output
	涓€涓?codec 杈撳嚭寮曡剼
Headphone
	鑰虫満锛堜互鍙婂彲閫夌殑 Jack锛?Mic
	楹﹀厠椋庯紙浠ュ強鍙€夌殑 Jack锛?Line
	绾胯矾杈撳叆/杈撳嚭锛堜互鍙婂彲閫夌殑 Jack锛?Speaker
	鎵０鍣?Supply
	琚叾浠?widget 浣跨敤鐨勭數婧愭垨鏃堕挓渚涘簲 widget銆?Regulator
	涓洪煶棰戠粍浠朵緵鐢电殑澶栭儴绋冲帇鍣ㄣ€?Clock
	涓洪煶棰戠粍浠舵彁渚涙椂閽熺殑澶栭儴鏃堕挓銆?AIF IN
	闊抽鎺ュ彛杈撳叆锛堝甫 TDM 鏃堕殭鎺╃爜锛夈€?AIF OUT
	闊抽鎺ュ彛杈撳嚭锛堝甫 TDM 鏃堕殭鎺╃爜锛夈€?Siggen
	淇″彿鍙戠敓鍣ㄣ€?DAI IN
	鏁板瓧闊抽鎺ュ彛杈撳叆銆?DAI OUT
	鏁板瓧闊抽鎺ュ彛杈撳嚭銆?DAI Link
	涓や釜 DAI 缁撴瀯涔嬮棿鐨?DAI 閾捐矾銆?Pre
	鐗规畩鐨?PRE widget锛堝湪鎵€鏈夊叾浠栦箣鍓嶆墽琛岋級
Post
	鐗规畩鐨?POST widget锛堝湪鎵€鏈夊叾浠栦箣鍚庢墽琛岋級
Buffer
	DSP 鍐呴儴閮ㄤ欢涔嬮棿鐨勯煶棰戞暟鎹紦鍐插尯銆?Scheduler
	璋冨害缁勪欢/娴佹按绾垮鐞嗗伐浣滅殑 DSP 鍐呴儴璋冨害鍣ㄣ€?Effect
	鎵ц闊抽澶勭悊鏁堟灉鐨?widget銆?SRC
	DSP 鎴?CODEC 鍐呯殑閲囨牱鐜囪浆鎹㈠櫒锛圫ample Rate Converter锛夈€?ASRC
	DSP 鎴?CODEC 鍐呯殑寮傛閲囨牱鐜囪浆鎹㈠櫒锛圓synchronous Sample Rate Converter锛夈€?Encoder
	灏嗘暟鎹粠涓€绉嶆牸寮忥紙閫氬父鏄?PCM锛夌紪鐮佷负鍙︿竴绉嶉€氬父鍘嬬缉绋嬪害鏇撮珮鐨勬牸寮忕殑 widget銆?Decoder
	灏嗘暟鎹粠鍘嬬缉鏍煎紡瑙ｇ爜涓?PCM 绛夋湭鍘嬬缉鏍煎紡鐨?widget銆?
锛圵idget 鍦?include/sound/soc-dapm.h 涓畾涔夛級

Widget 鍙互鐢变换浣曠粍浠堕┍鍔ㄧ被鍨嬫坊鍔犲埌澹板崱涓€俿oc-dapm.h 涓畾涔変簡渚夸簬浣跨敤鐨勫畯锛?鍙敤浜庡揩閫熸瀯寤?codec 涓庢満鍣?DAPM widget 鐨勫垪琛ㄣ€?
澶у鏁?widget 鍏锋湁 name銆乺egister銆乻hift 鍜?invert銆傛煇浜?widget 甯︽湁鐢ㄤ簬娴佸悕鍜?kcontrol 鐨勯澶栧弬鏁般€?
### 娴佸煙 Widgets

娴?widget 涓庢祦鐢垫簮鍩熺浉鍏筹紝浠呯敱 ADC锛堟ā鏁拌浆鎹㈠櫒锛夈€丏AC锛堟暟妯¤浆鎹㈠櫒锛夈€丄IF IN 鍜?AIF OUT 缁勬垚銆?
娴?widget 鍏锋湁浠ヤ笅鏍煎紡锛?```
  SND_SOC_DAPM_DAC(name, stream name, reg, shift, invert),
  SND_SOC_DAPM_AIF_IN(name, stream, slot, reg, shift, invert)
```

娉ㄦ剰锛氭祦鍚嶅繀椤讳笌浣?codec 涓?snd_soc_dai_driver 閲屽搴旂殑娴佸悕鐩稿尮閰嶃€?
渚嬪 HiFi 鎾斁涓庨噰闆嗙殑娴?widget锛?```
  SND_SOC_DAPM_DAC("HiFi DAC", "HiFi Playback", REG, 3, 1),
  SND_SOC_DAPM_ADC("HiFi ADC", "HiFi Capture", REG, 2, 1),
```

渚嬪 AIF 鐨勬祦 widget锛?```
  SND_SOC_DAPM_AIF_IN("AIF1RX", "AIF1 Playback", 0, SND_SOC_NOPM, 0, 0),
  SND_SOC_DAPM_AIF_OUT("AIF1TX", "AIF1 Capture", 0, SND_SOC_NOPM, 0, 0),
```

### 璺緞鍩?Widgets

璺緞鍩?widget 鍏锋湁鎺у埗鎴栧奖鍝嶉煶棰戝瓙绯荤粺鍐呴煶棰戜俊鍙锋垨闊抽璺緞鐨勮兘鍔涖€傚畠浠叿鏈?浠ヤ笅褰㈠紡锛?```
  SND_SOC_DAPM_PGA(name, reg, shift, invert, controls, num_controls)
```

浠讳綍 widget 鐨?kcontrol 閮藉彲浠ラ€氳繃 controls 鍜?num_controls 鎴愬憳璁剧疆銆?
渚嬪 Mixer widget锛坘control 鍏堝０鏄庯級锛?```
  /* Output Mixer */
  static const snd_kcontrol_new_t wm8731_output_mixer_controls[] = {
  SOC_DAPM_SINGLE("Line Bypass Switch", WM8731_APANA, 3, 1, 0),
  SOC_DAPM_SINGLE("Mic Sidetone Switch", WM8731_APANA, 5, 1, 0),
  SOC_DAPM_SINGLE("HiFi Playback Switch", WM8731_APANA, 4, 1, 0),
  };

  SND_SOC_DAPM_MIXER("Output Mixer", WM8731_PWR, 4, 1, wm8731_output_mixer_controls,
	ARRAY_SIZE(wm8731_output_mixer_controls)),
```

濡傛灉浣犱笉甯屾湜娣烽煶鍣ㄥ厓绱犱互娣烽煶鍣?widget 鐨勫悕绉颁綔涓哄墠缂€锛屽彲浠ヤ娇鐢?SND_SOC_DAPM_MIXER_NAMED_CTL 鏉ヤ唬鏇裤€傚弬鏁颁笌 SND_SOC_DAPM_MIXER 鐩稿悓銆?
### 鏈哄櫒鍩?Widgets

鏈哄櫒 widget 涓?codec widget 鐨勪笉鍚屼箣澶勫湪浜庡畠浠病鏈変笌涔嬪叧鑱旂殑 codec 瀵勫瓨鍣ㄤ綅銆?姣忎釜鍙互鐙珛渚涚數鐨勬満鍣ㄩ煶棰戠粍浠讹紙闈?codec 鎴?DSP锛夐兘浼氳鍒嗛厤缁欎竴涓満鍣?widget銆?渚嬪锛?
- 鎵０鍣ㄦ斁澶у櫒锛圫peaker Amp锛?- 楹﹀厠椋庡亸缃紙Microphone Bias锛?- Jack 杩炴帴鍣?
涓€涓満鍣?widget 鍙互鏈変竴涓彲閫夌殑鍥炶皟鍑芥暟銆?
渚嬪鐢ㄤ簬澶栭儴 Mic 鐨?Jack 杩炴帴鍣?widget锛屽畠鍚敤 Mic Bias锛?```
  static int spitz_mic_bias(struct snd_soc_dapm_widget* w, int event)
  {
	gpio_set_value(SPITZ_GPIO_MIC_BIAS, SND_SOC_DAPM_EVENT_ON(event));
	return 0;
  }

  SND_SOC_DAPM_MIC("Mic Jack", spitz_mic_bias),
```

### Codec锛圔IAS锛夊煙

codec 鍋忕疆鐢垫簮鍩熸病鏈?widget锛岀敱 codec DAPM 浜嬩欢澶勭悊绋嬪簭澶勭悊銆傚綋 codec 鐢垫簮鐘舵€?鐩稿浜庝换浣曟祦浜嬩欢鎴栧唴鏍?PM 浜嬩欢鍙戠敓鏀瑰彉鏃讹紝浼氳皟鐢ㄨ澶勭悊绋嬪簭銆?
### 铏氭嫙 Widgets

鏈夋椂 codec 鎴栨満鍣ㄩ煶棰戝浘涓瓨鍦ㄤ竴浜?widget锛屽畠浠病鏈変换浣曞搴旂殑杞數婧愭帶鍒躲€傚湪杩欑
鎯呭喌涓嬶紝鏈夊繀瑕佸垱寤轰竴涓櫄鎷?widget鈥斺€斾竴涓病鏈夋帶鍒朵綅鐨?widget锛屼緥濡傦細
```
  SND_SOC_DAPM_MIXER("AC97 Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
```

杩欏彲鐢ㄤ簬鍦ㄨ蒋浠朵腑灏嗕袱鏉′俊鍙疯矾寰勫悎骞跺湪涓€璧枫€?
## 娉ㄥ唽 DAPM 鎺т欢

鍦ㄨ澶氭儏鍐典笅锛孌APM widget 闈欐€佸湴瀹炵幇鍦?codec 椹卞姩涓殑涓€涓?``static const struct
snd_soc_dapm_widget`` 鏁扮粍涓紝骞剁畝鍗曞湴閫氳繃 `struct snd_soc_component_driver` 鐨?`dapm_widgets` 鍜?`num_dapm_widgets` 瀛楁鏉ュ０鏄庛€?
绫讳技鍦帮紝杩炴帴瀹冧滑鐨勮矾鐢遍潤鎬佸湴瀹炵幇鍦?``static const struct snd_soc_dapm_route``
鏁扮粍涓紝骞堕€氳繃鍚屼竴涓?struct 鐨?`dapm_routes` 鍜?`num_dapm_routes` 瀛楁澹版槑銆?
鍦ㄥ０鏄庝簡涓婅堪鍐呭涔嬪悗锛岄┍鍔ㄦ敞鍐屼細澶勭悊锛?```
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

鍦ㄦ洿澶嶆潅鐨勬儏鍐典笅锛孌APM widget 鍜?鎴栬矾鐢卞垪琛ㄥ彧鑳藉湪鎺㈡祴锛坧robe锛夋椂鎵嶈兘纭畾銆備緥濡?褰撻┍鍔ㄦ敮鎸佸叿鏈変笉鍚岀壒鎬ч泦鍚堢殑涓嶅悓鍨嬪彿鏃跺氨浼氬彂鐢熻繖绉嶆儏鍐点€傚湪杩欎簺鎯呭喌涓嬶紝瀹炵幇
鐗瑰畾浜庤鎯呭喌鐗规€х殑鐙珛 widget 鍜岃矾鐢辨暟缁勶紝鍙互閫氳繃璋冪敤 snd_soc_dapm_new_controls()
鍜?snd_soc_dapm_add_routes() 浠ョ紪绋嬫柟寮忔敞鍐屻€?
## Codec/DSP Widget 浜掕繛

Widget 閫氳繃闊抽璺緞锛堢О涓轰簰杩烇級鍦?codec銆佸钩鍙板拰鏈哄櫒鍐呴儴鐩镐簰杩炴帴銆傚繀椤诲畾涔夋瘡涓?浜掕繛锛屼互渚垮垱寤?widget 涔嬮棿鎵€鏈夐煶棰戣矾寰勭殑鍥俱€?
杩欏湪浣跨敤 codec 鎴?DSP 鐨勫浘锛堜互鍙婃満鍣ㄩ煶棰戠郴缁熺殑鍘熺悊鍥撅級鏃舵渶涓哄鏄擄紝鍥犱负瀹冮渶瑕?閫氳繃鍚勮嚜鐨勯煶棰戜俊鍙疯矾寰勫皢 widget 杩炴帴鍦ㄤ竴璧枫€?
渚嬪 WM8731 杈撳嚭娣烽煶鍣紙wm8731.c锛夋湁 3 涓緭鍏ワ紙婧愶級锛?
1. Line Bypass 杈撳叆
2. DAC锛圚iFi 鎾斁锛?3. Mic Sidetone 杈撳叆

姝ょず渚嬩腑鐨勬瘡涓緭鍏ラ兘鏈変竴涓浉鍏宠仈鐨?kcontrol锛堝湪涓婇潰绀轰緥涓畾涔夛級锛屽苟閫氳繃鍏?kcontrol
鍚嶈繛鎺ュ埌杈撳嚭娣烽煶鍣ㄣ€傛垜浠幇鍦ㄥ彲浠ュ皢鐩爣 widget锛堝氨闊抽淇″彿鑰岃█锛変笌鍏讹細
```
	/* output mixer */
	{"Output Mixer", "Line Bypass Switch", "Line Input"},
	{"Output Mixer", "HiFi Playback Switch", "DAC"},
	{"Output Mixer", "Mic Sidetone Switch", "Mic Bias"},
```

浜庢槸鎴戜滑鏈夛細

- 鐩爣 Widget <=== 璺緞鍚?<=== 婧?Widget锛屾垨鑰?- Sink銆丳ath銆丼ource锛屾垨鑰?- `Output Mixer` 閫氳繃 `HiFi Playback Switch` 杩炴帴鍒?`DAC`銆?
褰撴病鏈夎矾寰勫悕杩炴帴 widget 鏃讹紙渚嬪鐩存帴杩炴帴锛夛紝鎴戜滑涓鸿矾寰勫悕浼?NULL銆?
```
  snd_soc_dapm_connect_input(codec, sink, path, source);
```

鏈€鍚庯紝蹇呴』鍦ㄦ墍鏈?widget 鍜屼簰杩為兘鍚戞牳蹇冩敞鍐屼箣鍚庤皟鐢?snd_soc_dapm_new_widgets()銆?杩欎細瀵艰嚧鏍稿績鎵弿 codec 鍜屾満鍣紝浣垮唴閮?DAPM 鐘舵€佷笌鏈哄櫒鐨勭墿鐞嗙姸鎬佺浉鍖归厤銆?
### 鏈哄櫒 Widget 浜掕繛

鏈哄櫒 widget 浜掕繛鐨勫垱寤烘柟寮忎笌 codec 鐨勭浉鍚岋紝骞剁洿鎺ュ皢 codec 寮曡剼杩炴帴鍒版満鍣ㄧ骇 widget銆?
渚嬪灏嗘壃澹板櫒杈撳嚭 codec 寮曡剼杩炴帴鍒板唴閮ㄦ壃澹板櫒銆?```
	/* ext speaker connected to codec pins LOUT2, ROUT2  */
	{"Ext Spk", NULL , "ROUT2"},
	{"Ext Spk", NULL , "LOUT2"},
```

杩欏厑璁?DAPM 鍒嗗埆鎵撳紑鍜屽叧闂凡杩炴帴锛堜笖鍦ㄤ娇鐢ㄤ腑锛夌殑寮曡剼浠ュ強 NC锛堟湭杩炴帴锛夊紩鑴氥€?
## 绔偣 Widgets

绔偣鏄満鍣ㄥ唴闊抽淇″彿鐨勮捣鐐规垨缁堢偣锛坵idget锛夛紝骞跺寘鍚?codec銆備緥濡傦細

- 鑰虫満 Jack
- 鍐呴儴鎵０鍣?- 鍐呴儴楹﹀厠椋?- 楹﹀厠椋?Jack
- Codec 寮曡剼

绔偣琚坊鍔犲埌 DAPM 鍥句腑锛屼互渚垮彲浠ョ‘瀹氬畠浠殑浣跨敤鎯呭喌浠ヨ妭鐪佺數婧愩€備緥濡?NC 鐨?codec
寮曡剼灏嗚鍏抽棴锛屾湭杩炴帴鐨?jack 涔熷彲浠ヨ鍏抽棴銆?
## DAPM Widget 浜嬩欢

闇€瑕佸疄鐜版瘮 DAPM 鎵€鑳藉仛鐨勬洿澶嶆潅琛屼负鐨?widget锛屽彲浠ラ€氳繃璁剧疆涓€涓嚱鏁版寚閽堟潵璁剧疆
鑷畾涔夌殑"浜嬩欢澶勭悊绋嬪簭"銆備緥濡傦細
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

鏈夊叧鎵€鏈夊叾浠栨敮鎸佷簨浠剁殑 widget锛岃鍙傞槄 soc-dapm.h銆?
### 浜嬩欢绫诲瀷

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
