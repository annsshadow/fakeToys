## ASoC USB 鏀寔


## 姒傝堪

涓轰簡鍒╃敤 ALSA 涓幇鏈夌殑 USB 澹伴煶璁惧鏀寔锛屽紩鍏ヤ簡 ASoC USB API锛屼互鍏佽鍚勫瓙绯荤粺浜ゆ崲閰嶇疆淇℃伅銆?
涓€涓綔鍦ㄧ敤渚嬫槸鏀寔 USB 闊抽鍗歌浇锛圲SB audio offloading锛夛紝杩欐槸涓€绉嶅疄鐜帮紝鍏佽闊抽瀛愮郴缁熶腑涓€鏉?鏇夸唬鐨勩€佺粡杩囧姛鑰椾紭鍖栫殑璺緞鏉ュ鐞嗛€氳繃 USB 鎬荤嚎浼犺緭鐨勯煶棰戞暟鎹€傝繖灏嗚涓诲鐞嗗櫒鑳藉鍦ㄦ洿闀挎椂闂村唴淇濇寔
杈冧綆鍔熻€楁ā寮忋€備互涓嬫槸 ASoC 涓?ALSA 鍚勯儴鍒嗗浣曡繛鎺ュ湪涓€璧蜂互瀹炵幇姝ょ洰鐨勭殑绀轰緥璁捐锛?
```

               USB                   |            ASoC
                                     |  _________________________
                                     | |   ASoC Platform card    |
                                     | |_________________________|
                                     |         |           |
                                     |      ___V____   ____V____
                                     |     |ASoC BE | |ASoC FE  |
                                     |     |DAI LNK | |DAI LNK  |
                                     |     |________| |_________|
                                     |         ^  ^        ^
                                     |         |  |________|
                                     |      ___V____    |
                                     |     |SoC-USB |   |
     ________       ________               |        |   |
    |USB SND |<--->|USBSND  |<------------>|________|   |
    |(card.c)|     |offld   |<----------                |
    |________|     |________|___     | |                |
        ^               ^       |    | |    ____________V_________
        |               |       |    | |   |IPC                   |
     __ V_______________V_____  |    | |   |______________________|
    |USB SND (endpoint.c)     | |    | |              ^
    |_________________________| |    | |              |
                ^               |    | |   ___________V___________
                |               |    | |->|audio DSP              |
     ___________V_____________  |    |    |_______________________|
    |XHCI HCD                 |<-    |
    |_________________________|      |

```
## SoC USB 椹卞姩

### 缁撴瀯浣?
`struct snd_soc_usb`

  - `list`锛歋ND SoC 缁撴瀯浣撳垪琛ㄧ殑閾捐〃澶?  - `component`锛氬 ASoC 缁勪欢鐨勫紩鐢?  - `connection_status_cb`锛氱敤浜庨€氱煡杩炴帴浜嬩欢鐨勫洖璋?  - `update_offload_route_info`锛氱敤浜庤幏鍙栨墍閫?USB 澹伴煶鍗?PCM 璁惧鐨勫洖璋?  - `priv_data`锛氶┍鍔ㄦ暟鎹?
snd_soc_usb 缁撴瀯鍙互閫氳繃 ASoC 骞冲彴鍗¤澶囷紝鎴栬€呬竴涓?USB 璁惧锛坲dev->dev锛夋潵寮曠敤銆傚畠鐢?ASoC BE DAI
閾捐矾鍒涘缓锛孶SB 澹伴煶瀹炰綋灏嗚兘澶熶娇鐢ㄦ缁撴瀯鍚?ASoC BE DAI 閾捐矾浼犻€掍俊鎭€?
`struct snd_soc_usb_device`

  - `card_idx`锛氫笌 USB 澹伴煶璁惧鍏宠仈鐨勫０闊冲崱绱㈠紩
  - `chip_idx`锛歎SB 澹伴煶鑺墖鏁扮粍绱㈠紩
  - `cpcm_idx`锛氫笌璇?USB 澹伴煶璁惧鍏宠仈鐨勬崟鑾?PCM 璁惧绱㈠紩
  - `ppcm_idx`锛氫笌璇?USB 澹伴煶璁惧鍏宠仈鐨勫洖鏀?PCM 璁惧绱㈠紩
  - `num_playback`锛氬洖鏀炬祦鐨勬暟閲?  - `num_capture`锛氭崟鑾锋祦鐨勬暟閲?  - `list`锛歎SB 澹伴煶璁惧鍒楄〃鐨勯摼琛ㄥご

struct snd_soc_usb_device 鐢?USB 澹伴煶鍗歌浇椹卞姩鍒涘缓銆傚畠灏嗘惡甯︾敤浜庣‘瀹氭 USB 闊抽璁惧鍙兘鍗歌浇璺緞鐨?鍩烘湰鍙傛暟/闄愬埗銆?
### 鍑芥暟


	int snd_soc_usb_find_supported_format(int card_idx,
			struct snd_pcm_hw_params *params, int direction)
..

  - `card_idx`锛歎SB 澹伴煶鑺墖鏁扮粍鐨勭储寮曘€?  - `params`锛氭潵鑷?USB DPCM BE DAI 閾捐矾鐨勮姹?PCM 鍙傛暟
  - `direction`锛氭崟鑾锋垨鍥炴斁

**snd_soc_usb_find_supported_format()** 纭繚澶栭儴 DSP 鎵€璇锋眰鐨勯煶棰戦厤缃枃浠跺彈 USB 璁惧鏀寔銆?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖 -EOPNOTSUPP銆?

	int snd_soc_usb_connect(struct device **usbdev, struct snd_soc_usb_device **sdev)
..

  - `usbdev`锛氳鍙戠幇鐨?usb 璁惧
  - `sdev`锛氳澶囩殑鑳藉姏

**snd_soc_usb_connect()** 灏?USB 闊抽璁惧鐨勬帰娴嬮€氱煡缁?ASoC USB DPCM BE DAI 閾捐矾銆傝繖鍙敤浜?BE DAI
椹卞姩涓紝浠ヨ窡韪彲鐢ㄧ殑 USB 闊抽璁惧銆傚畠棰勬湡鐢遍┗鐣欏湪 USB SND 涓殑 USB 鍗歌浇椹卞姩璋冪敤銆?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?

	int snd_soc_usb_disconnect(struct device **usbdev, struct snd_soc_usb_device **sdev)
..

  - `usbdev`锛氳绉婚櫎鐨?usb 璁惧
  - `sdev`锛氳閲婃斁鐨勮兘鍔?
**snd_soc_usb_disconnect()** 灏?USB 闊抽璁惧鐨勭Щ闄ら€氱煡缁?ASoC USB DPCM BE DAI 閾捐矾銆傚畠棰勬湡鐢遍┗鐣欏湪
USB SND 涓殑 USB 鍗歌浇椹卞姩璋冪敤銆?

	void **snd_soc_usb_find_priv_data(struct device **usbdev)
..

  - `usbdev`锛氱敤浜庢煡鎵剧鏈夋暟鎹墍寮曠敤鐨?usb 璁惧

**snd_soc_usb_find_priv_data()** 鑾峰彇淇濆瓨鍒?SoC USB 璁惧鐨勭鏈夋暟鎹€?
鎴愬姛鏃惰繑鍥炴寚鍚?priv_data 鐨勬寚閽堬紝澶辫触鏃惰繑鍥?NULL銆?

	int snd_soc_usb_setup_offload_jack(struct snd_soc_component *component,
					struct snd_soc_jack *jack)
..

  - `component`锛氳娣诲姞 jack 鐨?ASoC 缁勪欢
  - `jack`锛氳濉厖鐨?jack 缁勪欢

**snd_soc_usb_setup_offload_jack()** 鏄竴涓緟鍔╁嚱鏁帮紝鐢ㄤ簬鍚戝钩鍙板０闊冲崱娣诲姞涓€涓０闊?jack 鎺у埗銆傝繖灏嗗厑璁?鏀寔 USB 闊抽鍗歌浇鐨勮璁′娇鐢ㄤ竴鑷寸殑鍚嶇О銆傛澶栵紝杩欏皢鍚敤 jack 浠ラ€氱煡鍙樻洿銆?
鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炶礋鍊笺€?

	int snd_soc_usb_update_offload_route(struct device *dev, int card, int pcm,
					     int direction, enum snd_soc_usb_kctl path,
					     long *route)
..

  - `dev`锛氳鏌ユ壘鍗歌浇璺緞鏄犲皠鐨?USB 璁惧
  - `card`锛歎SB 澹伴煶鍗＄储寮?  - `pcm`锛歎SB 澹伴煶 PCM 璁惧绱㈠紩
  - `direction`锛氳鑾峰彇鍗歌浇璺敱淇℃伅鐨勬柟鍚?  - `path`锛歬control 閫夋嫨鍣?- pcm 璁惧鎴栧崱绱㈠紩
  - `route`锛氬嵏杞借矾寰勭殑澹伴煶鍗″拰 pcm 绱㈠紩鏄犲皠銆傝繖鏄竴涓敱涓や釜鏁存暟缁勬垚鐨勬暟缁勶紝鎸夎鐗瑰畾椤哄簭鎼哄甫鍗″拰
	       pcm 璁惧绱㈠紩銆傚畠鍙敤浣?kcontrol 杈撳嚭鐨勬暟缁勩€?
**snd_soc_usb_update_offload_route()** 璋冪敤娉ㄥ唽鍒?USB BE DAI 閾捐矾鐨勫洖璋冿紝浠ヨ幏鍙栧叧浜庝负鎵ц璇ヨ澶囩殑
USB 闊抽鍗歌浇鑰屾槧灏勭殑 ASoC 璁惧鐨勪俊鎭€俙route` 鍙互鏄寚鍚?kcontrol 鍊艰緭鍑烘暟缁勭殑鎸囬拡锛岃鏁扮粍鍦ㄨ鍙?kcontrol 鏃舵惡甯﹀€笺€?
鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥炶礋鍊笺€?

	struct snd_soc_usb **snd_soc_usb_allocate_port(struct snd_soc_component **component,
			void *data);
..

  - `component`锛欴PCM BE DAI 閾捐矾缁勪欢
  - `data`锛氱鏈夋暟鎹?
**snd_soc_usb_allocate_port()** 鍒嗛厤涓€涓?SoC USB 璁惧骞跺～鍏呯敤浜庡悗缁搷浣滅殑鏍囧噯鍙傛暟銆?
鎴愬姛鏃惰繑鍥炴寚鍚?struct soc_usb 鐨勬寚閽堬紝閿欒鏃惰繑鍥炶礋鍊笺€?

	void snd_soc_usb_free_port(struct snd_soc_usb *usb);
..

  - `usb`锛氳閲婃斁鐨?SoC USB 璁惧

**snd_soc_usb_free_port()** 閲婃斁涓€涓?SoC USB 璁惧銆?

	void snd_soc_usb_add_port(struct snd_soc_usb *usb);
..

  - `usb`锛氳娣诲姞鐨?SoC USB 璁惧

**snd_soc_usb_add_port()** 灏嗕竴涓凡鍒嗛厤鐨?SoC USB 璁惧娣诲姞鍒?SoC USB 妗嗘灦銆備竴鏃︽坊鍔狅紝璇ヨ澶囧嵆鍙
鍚庣画鎿嶄綔寮曠敤銆?

	void snd_soc_usb_remove_port(struct snd_soc_usb *usb);
..

  - `usb`锛氳绉婚櫎鐨?SoC USB 璁惧

**snd_soc_usb_remove_port()** 浠?SoC USB 妗嗘灦涓Щ闄や竴涓?SoC USB 璁惧銆傜Щ闄よ澶囧悗锛屼换浣?SoC USB
鎿嶄綔閮藉皢鏃犳硶寮曠敤琚Щ闄ょ殑璁惧銆?
### 濡備綍娉ㄥ唽鍒?SoC USB

ASoC DPCM USB BE DAI 閾捐矾鏄礋璐ｅ湪缁勪欢缁戝畾鏃跺垎閰嶅拰娉ㄥ唽 SoC USB 瀹炰綋鐨勭粍浠躲€傚悓鏍凤紝瀹冧篃璐熻矗閲婃斁鎵€
鍒嗛厤鐨勮祫婧愩€傜ず渚嬪涓嬶細


	static int q6usb_component_probe(struct snd_soc_component *component)
	{
		...
		data->usb = snd_soc_usb_allocate_port(component, 1, &data->priv);
		if (!data->usb)
			return -ENOMEM;

		usb->connection_status_cb = q6usb_alsa_connection_cb;

		ret = snd_soc_usb_add_port(usb);
		if (ret < 0) {
			dev_err(component->dev, "failed to add usb port\n");
			goto free_usb;
		}
		...
	}

	static void q6usb_component_remove(struct snd_soc_component *component)
	{
		...
		snd_soc_usb_remove_port(data->usb);
		snd_soc_usb_free_port(data->usb);
	}

	static const struct snd_soc_component_driver q6usb_dai_component = {
		.probe = q6usb_component_probe,
		.remove = q6usb_component_remove,
		.name = "q6usb-dai-component",
		...
	};
..

BE DAI 閾捐矾鍙互灏嗕緵搴斿晢鐗瑰畾鐨勪俊鎭綔涓哄垎閰?SoC USB 璁惧璋冪敤鐨勪竴閮ㄥ垎浼犻€掋€傝繖灏嗗厑璁搁┗鐣欏湪 USB SND 涓?鐨?USB 鍗歌浇椹卞姩璁块棶浠讳綍 BE DAI 閾捐矾鍙傛暟鎴栬缃€?
### USB 闊抽璁惧杩炴帴娴佺▼

USB 璁惧鍙互闅忔椂鐑彃鎷斿埌 USB 绔彛銆侭E DAI 閾捐矾搴斿綋鐭ユ檽鐗╃悊 USB 绔彛鐨勫綋鍓嶇姸鎬侊紝鍗虫槸鍚﹁繛鎺ヤ簡浠讳綍
甯︽湁闊抽鎺ュ彛鐨?USB 璁惧銆俢onnection_status_cb() 鍙敤浜庡皢浠讳綍鍙樻洿閫氱煡缁?BE DAI 閾捐矾銆?
姣忓綋鍙戠敓 USB SND 鎺ュ彛缁戝畾鎴栫Щ闄や簨浠舵椂锛岄兘浼氫娇鐢?snd_soc_usb_connect() 鎴?snd_soc_usb_disconnect()
璋冪敤瀹冿細


	static void qc_usb_audio_offload_probe(struct snd_usb_audio *chip)
	{
		...
		snd_soc_usb_connect(usb_get_usb_backend(udev), sdev);
		...
	}

	static void qc_usb_audio_offload_disconnect(struct snd_usb_audio *chip)
	{
		...
		snd_soc_usb_disconnect(usb_get_usb_backend(chip->dev), dev->sdev);
		...
	}
..

涓轰簡搴斿椹卞姩鎴栬澶囧瓨鍦ㄦ棤娉曚繚璇佺殑鎯呭喌锛孶SB SND 鏆撮湶浜?snd_usb_rediscover_devices() 浠ラ噸鏂板彂閫佷换浣?宸茶瘑鍒?USB 闊抽鎺ュ彛鐨勮繛鎺ヤ簨浠躲€傝€冭檻浠ヤ笅鎯呭舰锛?
	**usb_audio_probe()**
	  | --> USB 闊抽娴佽鍒嗛厤骞朵繚瀛樺埌 usb_chip[]
	  | --> 灏嗚繛鎺ヤ簨浠朵紶鎾粰 USB SND 涓殑 USB 鍗歌浇椹卞姩
	  | --> **snd_soc_usb_connect()** 鍥?USB BE DAI 閾捐矾鏈氨缁€岄€€鍑?
	BE DAI 閾捐矾缁勪欢鎺㈡祴
	  | --> DAI 閾捐矾琚帰娴嬶紝SoC USB 绔彛琚垎閰?	  | --> USB 闊抽璁惧杩炴帴浜嬩欢琚敊杩?
涓虹‘淇濊繛鎺ヤ簨浠朵笉琚敊杩囷紝褰?SoC USB 璁惧琚敞鍐屾椂鎵ц **snd_usb_rediscover_devices()**銆傜幇鍦紝褰?BE DAI 閾捐矾缁勪欢鎺㈡祴鍙戠敓鏃讹紝浠ヤ笅绐佸嚭浜嗚搴忓垪锛?
	BE DAI 閾捐矾缁勪欢鎺㈡祴
	  | --> DAI 閾捐矾琚帰娴嬶紝SoC USB 绔彛琚垎閰?	  | --> SoC USB 璁惧宸叉坊鍔狅紝骞朵笖 **snd_usb_rediscover_devices()** 杩愯

	**snd_usb_rediscover_devices()**
	  | --> 閬嶅巻 usb_chip[]锛屽苟瀵归潪 NULL 椤瑰彂鍑?	  |     **connection_status_cb()**

鍦?USB 鍗歌浇椹卞姩琚В缁戣€?USB SND 灏辩华鐨勬儏鍐典笅锛?*snd_usb_rediscover_devices()** 鍦ㄦā鍧楀垵濮嬪寲鏈熼棿琚皟鐢ㄣ€?杩欎娇寰楀嵏杞借矾寰勪篃鑳介€氳繃浠ヤ笅娴佺▼琚惎鐢細

	**usb_audio_probe()**
	  | --> USB 闊抽娴佽鍒嗛厤骞朵繚瀛樺埌 usb_chip[]
	  | --> 灏嗚繛鎺ヤ簨浠朵紶鎾粰 USB SND 涓殑 USB 鍗歌浇椹卞姩
	  | --> USB 鍗歌浇椹卞姩**鏈?*灏辩华锛?
	BE DAI 閾捐矾缁勪欢鎺㈡祴
	  | --> DAI 閾捐矾琚帰娴嬶紝SoC USB 绔彛琚垎閰?	  | --> 鍥犵己灏?USB 鍗歌浇椹卞姩锛屾病鏈?USB 杩炴帴浜嬩欢

	USB 鍗歌浇椹卞姩鎺㈡祴
	  | --> **qc_usb_audio_offload_init()**
	  | --> 璋冪敤 **snd_usb_rediscover_devices()** 浠ラ€氱煡璁惧

## USB 鍗歌浇鐩稿叧鐨?Kcontrols

### 缁嗚妭

涓€缁?kcontrol 鍙緵搴旂敤绋嬪簭浣跨敤锛屼互甯姪閫夋嫨姝ｇ‘鐨勫０闊宠澶囨潵鍚敤 USB 闊抽鍗歌浇銆係oC USB 鏆撮湶浜?get_offload_dev() 鍥炶皟锛岃璁″彲鍒╃敤瀹冩潵纭繚灏嗘纭殑绱㈠紩杩斿洖缁欏簲鐢ㄧ▼搴忋€?
### 瀹炵幇


**绀轰緥锛?*

  **澹伴煶鍗?*锛?
```

	  0 [SM8250MTPWCD938]: sm8250 - SM8250-MTP-WCD9380-WSA8810-VA-D
						SM8250-MTP-WCD9380-WSA8810-VA-DMIC
	  1 [Seri           ]: USB-Audio - Plantronics Blackwire 3225 Seri
						Plantronics Plantronics Blackwire
						3225 Seri at usb-xhci-hcd.1.auto-1.1,
						full sp
	  2 [C320M          ]: USB-Audio - Plantronics C320-M
                      Plantronics Plantronics C320-M at usb-xhci-hcd.1.auto-1.2, full speed

  **PCM 璁惧**锛?
	::

	  card 0: SM8250MTPWCD938 [SM8250-MTP-WCD9380-WSA8810-VA-D], device 0: MultiMedia1 (*) []
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 0: SM8250MTPWCD938 [SM8250-MTP-WCD9380-WSA8810-VA-D], device 1: MultiMedia2 (*) []
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 1: Seri [Plantronics Blackwire 3225 Seri], device 0: USB Audio [USB Audio]
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0
	  card 2: C320M [Plantronics C320-M], device 0: USB Audio [USB Audio]
	  Subdevices: 1/1
	  Subdevice #0: subdevice #0

  **USB 澹伴煶鍗?* - card#1锛?
	::

	  USB Offload Playback Card Route PCM#0   -1 (range -1->32)
	  USB Offload Playback PCM Route PCM#0    -1 (range -1->255)

  **USB 澹伴煶鍗?* - card#2锛?
	::

	  USB Offload Playback Card Route PCM#0   0 (range -1->32)
	  USB Offload Playback PCM Route PCM#0    1 (range -1->255)

```
涓婅堪绀轰緥灞曠ず浜嗕竴涓郴缁熸嫢鏈変竴涓?ASoC 骞冲彴鍗★紙card#0锛夊苟杩炴帴浜嗕袱涓?USB 澹伴煶璁惧锛坈ard#1 鍜?card#2锛?鐨勫満鏅€傚綋璇诲彇姣忎釜 USB 闊抽璁惧鐨勫彲鐢?kcontrol 鏃讹紝浠ヤ笅 kcontrol 鍒楀嚭浜嗚鐗瑰畾 USB 璁惧鏄犲皠鐨勫嵏杞?鍗″拰 pcm 璁惧绱㈠紩锛?
	`USB Offload Playback Card Route PCM#*`

	`USB Offload Playback PCM Route PCM#*`

璇?kcontrol 鏄甫绱㈠紩鐨勶紝鍥犱负涓€涓?USB 闊抽璁惧鍙兘娼滃湪鍦版嫢鏈夊涓?PCM 璁惧銆備笂杩?kcontrol 瀹氫箟涓猴細

  - `USB Offload Playback Card Route PCM#` **(R)**锛氳繑鍥炴槧灏勫嵏杞借矾寰勭殑 ASoC 骞冲彴澹伴煶鍗＄储寮曘€傝緭鍑?    **"0"**锛堝崱绱㈠紩锛夎〃绀洪€氳繃 card#0 瀛樺湪璇?USB SND 璁惧鍙敤鐨勫嵏杞借矾寰勩€傚鏋滅湅鍒?**"-1"**锛屽垯
    璇?USB SND 璁惧娌℃湁鍙敤鐨勫嵏杞借矾寰勩€傝 kcontrol 瀵圭郴缁熶腑瀛樺湪鐨勬瘡涓?USB 闊抽璁惧閮藉瓨鍦紝棰勬湡
    鏍规嵁璇?kcontrol 鐨勮緭鍑哄€间互鍙?PCM 璺敱 kcontrol 鏉ユ帹瀵煎嵏杞界殑褰撳墠鐘舵€併€?
  - `USB Offload Playback PCM Route PCM#` **(R)**锛氳繑鍥炴槧灏勫嵏杞借矾寰勭殑 ASoC 骞冲彴澹伴煶鍗?PCM 璁惧绱㈠紩銆?    杈撳嚭 **"1"**锛圥CM 璁惧绱㈠紩锛夎〃绀洪€氳繃 PCM device#0 瀛樺湪璇?USB SND 璁惧鍙敤鐨勫嵏杞借矾寰勩€傚鏋滅湅鍒?    **"-1"**锛屽垯璇?USB SND 璁惧娌℃湁鍙敤鐨勫嵏杞借矾寰勩€傝 kcontrol 瀵圭郴缁熶腑瀛樺湪鐨勬瘡涓?USB 闊抽璁惧閮藉瓨鍦紝
    棰勬湡鏍规嵁璇?kcontrol 鐨勮緭鍑哄€间互鍙婂崱璺敱 kcontrol 鏉ユ帹瀵煎嵏杞界殑褰撳墠鐘舵€併€?
### USB 鍗歌浇鍥炴斁璺敱 Kcontrol

涓轰簡鍏佽鍦ㄩ煶棰戝嵏杞借澶囬€夋嫨涓婃湁渚涘簲鍟嗙壒瀹氱殑瀹炵幇锛孲oC USB 灞傛毚闇蹭簡浠ヤ笅鍐呭锛?

	int (**update_offload_route_info)(struct snd_soc_component **component,
					 int card, int pcm, int direction,
					 enum snd_soc_usb_kctl path,
					 long *route)
..

杩欎簺鐗瑰畾浜?**USB Offload Playback Card Route PCM#** 鍜?**USB Offload PCM Route PCM#** kcontrol銆?
褰撶敤鎴峰 kcontrol 鍙戝嚭 get 璋冪敤鏃讹紝娉ㄥ唽鐨?SoC USB 鍥炶皟灏嗘墽琛屾敞鍐屽埌 DPCM BE DAI 閾捐矾鐨勫嚱鏁拌皟鐢ㄣ€?
**鍥炶皟娉ㄥ唽锛?*


	static int q6usb_component_probe(struct snd_soc_component *component)
	{
	...
	usb = snd_soc_usb_allocate_port(component, 1, &data->priv);
	if (IS_ERR(usb))
		return -ENOMEM;

	usb->connection_status_cb = q6usb_alsa_connection_cb;
	usb->update_offload_route_info = q6usb_get_offload_dev;

	ret = snd_soc_usb_add_port(usb);
..

### 鐜版湁 USB 澹伴煶 Kcontrol

闅忕潃 USB 鍗歌浇鏀寔鐨勫紩鍏ワ紝涓婅堪 USB 鍗歌浇 kcontrol 灏嗚娣诲姞鍒扮敱 USB 澹伴煶妗嗘灦璇嗗埆鐨勫凡鏈?kcontrol 鍒楄〃涓€?杩欎簺 kcontrol 浠嶇劧鏄敤浜庝慨鏀逛笌 USB 闊抽璁惧鐩稿叧鐗规€х殑涓绘帶浠躲€?
```

	  Number of controls: 9
	  ctl     type    num     name                                    value
	  0       INT     2       Capture Channel Map                     0, 0 (range 0->36)
	  1       INT     2       Playback Channel Map                    0, 0 (range 0->36)
	  2       BOOL    1       Headset Capture Switch                  On
	  3       INT     1       Headset Capture Volume                  10 (range 0->13)
	  4       BOOL    1       Sidetone Playback Switch                On
	  5       INT     1       Sidetone Playback Volume                4096 (range 0->8192)
	  6       BOOL    1       Headset Playback Switch                 On
	  7       INT     2       Headset Playback Volume                 20, 20 (range 0->24)
	  8       INT     1       USB Offload Playback Card Route PCM#0   0 (range -1->32)
	  9       INT     1       USB Offload Playback PCM Route PCM#0    1 (range -1->255)

```
鐢变簬 USB 闊抽璁惧鎺у埗鏄€氳繃 USB 鎺у埗绔偣澶勭悊鐨勶紝璇蜂娇鐢?USB mixer 涓幇鏈夌殑鏈哄埗鏉ヨ缃煶閲忕瓑鍙傛暟銆?