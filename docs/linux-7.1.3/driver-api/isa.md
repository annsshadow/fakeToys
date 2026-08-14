## ISA 椹卞姩


浠ヤ笅鏂囨湰鏀圭紪鑷敱 Rene Herman 鎾板啓鐨?ISA 鎬荤嚎椹卞姩鍒濆鎻愪氦鐨勬彁浜よ鏄庛€?

鍦ㄨ繎鏈熷叧浜?浣跨敤 platform 璁惧鐨?ISA 椹卞姩"鐨勮璁轰腑锛屾湁浜烘寚鍑猴紙ALSA 鐨勶級ISA
椹卞姩閬囧埌浜嗕竴涓棶棰橈細鐢变簬鍦ㄩ┍鍔ㄦā鍨嬩腑 probe() 閿欒娌℃湁琚悜涓婁紶閫掞紝鍥犳褰?
鎺㈡祴涓嶅埌纭欢鏃讹紝瀹冧滑鏃犳硶閫夋嫨璁╅┍鍔ㄥ姞杞斤紙鏇村噯纭湴璇存槸璁惧娉ㄥ唽锛夊け璐ャ€傚湪姝?
杩囩▼涓紝鎴戝缓璁崟鐙绔嬩竴鏉?ISA 鎬荤嚎鍙兘鏄渶濂界殑鏂规锛汻ussell King 琛ㄧず鍚屾剰锛?
骞跺缓璁鎬荤嚎鍙互浣跨敤 .match() 鏂规硶鏉ヨ繘琛屽疄闄呯殑璁惧鍙戠幇銆?

闄勫甫鐨勫疄鐜版鏄姝ゃ€傚浜庤繖绉嶆棫鐨勩€佷笉鍙紙閫氱敤鍦帮級鍙戠幇鐨?ISA 纭欢锛屽彧鏈夐┍鍔?
鑷韩鎵嶈兘杩涜鍙戠幇锛屽洜姝や笌 platform_bus 涓嶅悓锛宨sa_bus 涔熷皢 match() 鍚戜笂鍒嗗彂鍒?
椹卞姩銆?

鍙︿竴涓笉鍚岀偣鏄細杩欎簺璁惧涔嬫墍浠ュ瓨鍦ㄤ簬椹卞姩妯″瀷涓紝鍙槸鍥犱负椹卞姩涓轰簡椹卞姩瀹冧滑鑰?
鍒涘缓浜嗗畠浠紝杩欐剰鍛崇潃鎵€鏈夌殑璁惧鍒涘缓涔熼兘宸茶鍐呴儴鍖栥€?

杩欑鏂瑰紡鎻愪緵鐨勪娇鐢ㄦā鍨嬪緢濂斤紝骞朵笖宸茬粡寰楀埌 ALSA 鏂归潰 Takashi Iwai 鍜?
Jaroslav Kysela 鐨勮鍙€侫LSA 椹卞姩鐨?module_init 鍥犳涓庡叾瀹冩€荤嚎妯″瀷闈炲父鐩镐技銆?
杩欎粠 ALSA 鐨?ISA 椹卞姩涓Щ闄や簡澶ч噺閲嶅鐨勫垵濮嬪寲浠ｇ爜銆?
```

	static int __init alsa_card_foo_init(void)
	{
		return isa_register_driver(&snd_foo_isa_driver, SNDRV_CARDS);
	}

	static void __exit alsa_card_foo_exit(void)
	{
		isa_unregister_driver(&snd_foo_isa_driver);
	}

```
浼犲叆鐨?isa_driver 缁撴瀯浣撳氨鏄父瑙勭殑椹卞姩缁撴瀯浣擄紝鍐呭祵浜嗕竴涓?struct device_driver銆?
甯歌鐨?probe/remove/shutdown/suspend/resume 鍥炶皟锛屼互鍙婂鍓嶆墍杩扮殑 .match 鍥炶皟銆?

浣犵湅鍒扮殑浼犲叆鐨?"SNDRV_CARDS" 鏄竴涓?"unsigned int ndev" 鍙傛暟锛岃〃绀鸿鍒涘缓
澶氬皯涓澶囧苟浠ヤ箣璋冪敤鎴戜滑鐨勬柟娉曘€?

platform_driver 鐨勫洖璋冧互涓€涓?platform_device 鍙傛暟琚皟鐢紱isa_driver 鐨勫洖璋?
鍒欑洿鎺ヤ互 鉄0鉄truct device *dev, unsigned int id鉄0鉄?瀵硅璋冪敤鈥斺€旂敱浜庤澶囧垱寤?
瀹屽叏鍦ㄦ€荤嚎鍐呴儴锛屽畬鍏ㄤ笉娉勬紡 isa_dev 鏄渶骞插噣鐨勫仛娉曘€俰d 姣曠珶鏄垜浠櫎浜?
struct device 涔嬪鍞竴鎯宠鐨勪笢瑗匡紝杩欎篃璁╁洖璋冧腑鐨勪唬鐮佹洿缇庤銆?

鍊熷姪杩欎釜棰濆鐨?.match() 鍥炶皟锛孖SA 椹卞姩鎷ユ湁浜嗗叏閮ㄩ€夐」銆傚鏋?ALSA 鎯充繚鐣欐棫鐨?
"涓嶅姞杞?琛屼负锛屽畠鍙互鎶婂叏閮ㄦ棫鐨?.probe 鏀捐繘 .match 涓紝杩欐牱鍙湁鍦ㄤ竴鍒囬兘瀛樺湪涓?
榻愬鏃舵墠淇濇寔娉ㄥ唽銆傚鏋滃畠鎯宠濮嬬粓鍔犺浇鐨勮涓猴紙鍦ㄥ悜 platform 璁惧鍒囨崲鍚庢浘鐭殏鍦?
鏃犳剰涓姝わ級锛屽畠鍙互骞茶剢涓嶆彁渚?.match()锛屽苟鍍忎互鍓嶄竴鏍峰湪 .probe() 涓仛鎵€鏈変簨鎯呫€?

濡傛灉瀹冿紙姝ｅ Takashi Iwai 鏃╁厛寤鸿鐨勩€佷綔涓轰竴绉嶆洿璐磋繎鍋ュ悍鎬荤嚎妯″瀷鐨勬柟寮忥級鎯冲湪
绋嶅悗鐨勭粦瀹氬彲鑳芥垚鍔熸椂鍔犺浇锛屽畠鍙互鍦?.match() 涓鐞嗗墠缃潯浠讹紙渚嬪妫€鏌ョ敤鎴锋槸鍚?
甯屾湜鍚敤璇ュ崱锛屼互鍙?port/irq/dma 鍊兼槸鍚﹀凡缁忎紶鍏ワ級锛岃€屾妸鍏朵綑涓€鍒囨斁鍦?.probe() 涓€?
杩欐槸鏈€鐞嗘兂鐨勬ā鍨嬨€?

杩涘叆浠ｇ爜鈥︹€?

瀹冨彧瀵煎嚭涓や釜鍑芥暟锛歩sa_{,un}register_driver()銆?

isa_register_driver() 娉ㄥ唽 struct device_driver锛岀劧鍚庨亶鍘嗕紶鍏ョ殑 ndev锛屽垱寤?
璁惧骞舵敞鍐屽畠浠€?

瀹冨仛鐨勭涓€浠朵簨鏄鏌ヨ璁惧鏄惁纭疄鏄椹卞姩鐨勮澶囦箣涓€锛屾柟寮忔槸鏌ョ湅璁惧鐨?
platform_data 鎸囬拡鏄惁琚涓烘湰椹卞姩銆俻latform 璁惧姣旇緝瀛楃涓诧紝浣嗘棦鐒朵竴鍒囬兘宸?
鍐呴儴鍖栵紝鎴戜滑灏辨棤闇€閭ｆ牱鍋氾紝鍥犳 isa_register_driver() 鎶?dev->platform_data
褰撲綔 isa_driver 鎸囬拡鏉ョ敤锛屼互渚垮湪姝ゅ妫€鏌ャ€?
```

	int isa_bus_match(struct device *dev, struct device_driver *driver)
	{
		struct isa_driver *isa_driver = to_isa_driver(driver);

		if (dev->platform_data == isa_driver) {
			if (!isa_driver->match ||
				isa_driver->match(dev, to_isa_dev(dev)->id))
				return 1;
			dev->platform_data = NULL;
		}
		return 0;
	}

```
鎴戠浉淇?platform_data 鍙敤浜庢鐩殑锛屼絾濡傛灉骞朵笉鎰挎剰锛屾妸 isa_driver 鎸囬拡绉诲埌绉佹湁鐨?
struct isa_dev 涓綋鐒朵篃瀹屽叏鍙互銆?

鐒跺悗锛屽鏋滈┍鍔ㄦ病鏈夋彁渚?.match锛屽垯鍖归厤銆傚鏋滄彁渚涗簡锛屽氨璋冪敤椹卞姩鐨?match() 鏂规硶
鏉ュ垽瀹氭槸鍚﹀尮閰嶃€?

濡傛灉**娌℃湁**鍖归厤锛宒ev->platform_data 浼氳閲嶇疆浠ュ悜 isa_register_driver 琛ㄦ槑杩欎竴鐐癸紝
鍚庤€呴殢鍚庡彲浠ュ啀娆℃敞閿€璇ヨ澶囥€?

濡傛灉鍦ㄨ繖涓€鍒囪繃绋嬩腑鍙戠敓浠讳綍閿欒锛屾垨鑰呮牴鏈病鏈夎澶囧尮閰嶏紝鍒欎竴鍒囬兘浼氳鍥為€€锛?
骞惰繑鍥炶閿欒鎴?-ENODEV銆?

isa_unregister_driver() 鍙槸娉ㄩ攢宸插尮閰嶇殑璁惧浠ュ強椹卞姩鑷韩銆?

module_isa_driver 鏄竴涓敤浜?ISA 椹卞姩鐨勮緟鍔╁畯锛岄€傜敤浜庨偅浜涘湪妯″潡 init/exit 涓?
涓嶅仛浠讳綍鐗规畩浜嬫儏鐨勯┍鍔ㄣ€傚畠娑堥櫎浜嗗ぇ閲忔牱鏉夸唬鐮併€傛瘡涓ā鍧楀彧鑳戒娇鐢ㄨ瀹忎竴娆★紝璋冪敤
瀹冧細鏇挎崲 module_init 鍜?module_exit銆?

max_num_isa_dev 鏄竴涓畯锛岀敤浜庡湪缁欏畾 ISA 璁惧鐨勫湴鍧€鑼冨洿鏃讹紝纭畾鍦?I/O 绔彛
鍦板潃绌洪棿涓彲鑳芥敞鍐岀殑鏈€澶?ISA 璁惧鏁伴噺銆?
