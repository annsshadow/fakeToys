## Linux WatchDog Timer 椹卞姩鏍稿績鍐呮牳 API


鏈€鍚庡闃咃細2013-02-12

Wim Van Sebroeck <wim@iguana.be>

### 绠€浠?

鏈枃妗ｅ苟涓嶆弿杩颁粈涔堟槸鐪嬮棬鐙楀畾鏃跺櫒锛圵DT锛夐┍鍔ㄦ垨璁惧锛屼篃涓嶆弿杩扮敤鎴风┖闂村彲鐢ㄤ簬涓?鐪嬮棬鐙楀畾鏃跺櫒閫氫俊鐨?API銆傚鏋滀綘鎯崇煡閬撹繖浜涳紝璇烽槄璇讳互涓嬫枃浠讹細
Documentation/watchdog/watchdog-api.rst 銆?
閭ｄ箞鏈枃妗ｆ弿杩颁粈涔堬紵瀹冩弿杩颁簡甯屾湜浣跨敤鐪嬮棬鐙楀畾鏃跺櫒椹卞姩鏍稿績妗嗘灦鐨勭湅闂ㄧ嫍瀹氭椂鍣?椹卞姩鎵€鑳戒娇鐢ㄧ殑 API銆傝妗嗘灦鎻愪緵浜嗘墍鏈夐潰鍚戠敤鎴风┖闂寸殑鎺ュ彛锛屽洜姝ゅ悓鏍风殑浠ｇ爜鏃犻渶
姣忔閮介噸澶嶇紪鍐欍€傝繖涔熸剰鍛崇潃鐪嬮棬鐙楀畾鏃跺櫒椹卞姩鍙渶瑕佹彁渚涙帶鍒剁湅闂ㄧ嫍瀹氭椂鍣紙WDT锛?鐨勪笉鍚屼緥绋嬶紙鎿嶄綔锛夈€?
### API


姣忎釜甯屾湜浣跨敤鐪嬮棬鐙楀畾鏃跺櫒椹卞姩鏍稿績鐨勭湅闂ㄧ嫍瀹氭椂鍣ㄩ┍鍔ㄩ兘蹇呴』 #include
<linux/watchdog.h>锛堝湪缂栧啓鐪嬮棬鐙楄澶囬┍鍔ㄦ椂鏃犺濡備綍浣犻兘寰楄繖涔堝仛锛夈€傝澶存枃浠?鍖呭惈濡備笅
```

	extern int watchdog_register_device(struct watchdog_device *);
	extern void watchdog_unregister_device(struct watchdog_device *);

```
watchdog_register_device 渚嬬▼娉ㄥ唽涓€涓湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囥€傝渚嬬▼鐨勫弬鏁版槸涓€涓寚鍚?watchdog_device 缁撴瀯鐨勬寚閽堛€傝渚嬬▼鎴愬姛鏃惰繑鍥為浂锛屽け璐ユ椂杩斿洖璐熺殑 errno 鐮併€?
watchdog_unregister_device 渚嬬▼娉ㄩ攢涓€涓凡娉ㄥ唽鐨勭湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囥€傝渚嬬▼鐨?鍙傛暟鏄凡娉ㄥ唽鐨?watchdog_device 缁撴瀯鐨勬寚閽堛€?
鐪嬮棬鐙楀瓙绯荤粺鍖呭惈涓€涓敞鍐屽欢杩熸満鍒讹紝鍏佽浣犲湪鍚姩杩囩▼涓敖鏃╁湴娉ㄥ唽涓€涓湅闂ㄧ嫍銆?
```

  struct watchdog_device {
	int id;
	struct device *parent;
	const struct attribute_group **groups;
	const struct watchdog_info *info;
	const struct watchdog_ops *ops;
	const struct watchdog_governor *gov;
	unsigned int bootstatus;
	unsigned int timeout;
	unsigned int pretimeout;
	unsigned int min_timeout;
	unsigned int max_timeout;
	unsigned int min_hw_heartbeat_ms;
	unsigned int max_hw_heartbeat_ms;
	struct notifier_block reboot_nb;
	struct notifier_block restart_nb;
	void *driver_data;
	struct watchdog_core_data *wd_data;
	unsigned long status;
	struct list_head deferred;
  };

```
瀹冨寘鍚互涓嬪瓧娈碉細

- id锛氱敱 watchdog_register_device 璁剧疆锛宨d 0 鏄壒娈婄殑銆傚畠鍚屾椂鎷ユ湁 /dev/watchdog0
  cdev锛堝姩鎬佷富璁惧鍙凤紝娆¤澶囧彿 0锛変互鍙婃棫鐨?/dev/watchdog miscdev銆傝皟鐢?  watchdog_register_device 鏃朵細鑷姩璁剧疆璇?id銆?- parent锛氬湪璋冪敤 watchdog_register_device 涔嬪墠锛屽皢鍏惰缃负鐖惰澶囷紙鎴?NULL锛夈€?- groups锛氬垱寤虹湅闂ㄧ嫍璁惧鏃惰鍒涘缓鐨?sysfs 灞炴€х粍鍒楄〃銆?- info锛氫竴涓寚鍚?watchdog_info 缁撴瀯鐨勬寚閽堛€傝缁撴瀯缁欏嚭鍏充簬鐪嬮棬鐙楀畾鏃跺櫒鑷韩鐨勪竴浜?  闄勫姞淇℃伅锛堝鍏跺敮涓€鍚嶇О锛夈€?- ops锛氫竴涓寚鍚戠湅闂ㄧ嫍鎵€鏀寔鐨勬搷浣滃垪琛ㄧ殑鎸囬拡銆?- gov锛氫竴涓寚鍚戝凡鍒嗛厤鐨勭湅闂ㄧ嫍璁惧 pretimeout 绠＄悊鍣紙governor锛夌殑鎸囬拡锛屾垨 NULL銆?- timeout锛氱湅闂ㄧ嫍瀹氭椂鍣ㄧ殑瓒呮椂鍊硷紙浠ョ涓哄崟浣嶏級銆傚鏋滆缃簡 WDOG_ACTIVE锛岃繖鏄?  鍦ㄧ敤鎴风┖闂翠笉鍙戦€佸績璺宠姹傜殑鎯呭喌涓嬬郴缁熷皢浼氶噸鍚殑鏃堕棿銆?- pretimeout锛氱湅闂ㄧ嫍瀹氭椂鍣ㄧ殑 pretimeout 鍊硷紙浠ョ涓哄崟浣嶏級銆?- min_timeout锛氱湅闂ㄧ嫍瀹氭椂鍣ㄧ殑鏈€灏忚秴鏃跺€硷紙浠ョ涓哄崟浣嶏級銆傝嫢璁剧疆锛屽垯涓?'timeout'
  鍙厤缃殑鏈€灏忓€笺€?- max_timeout锛氱湅闂ㄧ嫍瀹氭椂鍣ㄧ殑鏈€澶ц秴鏃跺€硷紙浠ョ涓哄崟浣嶏級锛屼粠鐢ㄦ埛绌洪棿鍙銆傝嫢璁剧疆锛?  鍒欎负 'timeout' 鍙厤缃殑鏈€澶у€笺€傚綋 max_hw_heartbeat_ms 闈為浂鏃朵笉浣跨敤銆?- min_hw_heartbeat_ms锛氬績璺充箣闂存渶灏忔椂闂撮棿闅旂殑纭欢闄愬埗锛屼互姣涓哄崟浣嶃€傝鍊奸€氬父涓?  0锛涘彧鏈夊綋纭欢鏃犳硶瀹瑰繊鏇寸煭鐨勫績璺抽棿闅旀椂鎵嶅簲鎻愪緵銆?- max_hw_heartbeat_ms锛氭渶澶х‖浠跺績璺筹紝浠ユ绉掍负鍗曚綅銆傝嫢璁剧疆锛屽綋 'timeout' 澶т簬
  max_hw_heartbeat_ms 鏃讹紝鍩虹璁炬柦浼氬悜鐪嬮棬鐙楅┍鍔ㄥ彂閫佸績璺筹紝闄ら潪璁剧疆浜?WDOG_ACTIVE
  涓旂敤鎴风┖闂磋嚦灏戝湪 'timeout' 绉掑唴鏈兘鍙戦€佷竴娆″績璺炽€傚鏋滈┍鍔ㄦ病鏈夊疄鐜?stop 鍑芥暟锛?  鍒欏繀椤昏缃?max_hw_heartbeat_ms銆?- reboot_nb锛氫负閲嶅惎閫氱煡娉ㄥ唽鐨?notifier 鍧楋紝浠呬緵鍐呴儴浣跨敤銆傚鏋滈┍鍔ㄨ皟鐢?  watchdog_stop_on_reboot锛岀湅闂ㄧ嫍鏍稿績浼氬湪鏀跺埌姝ょ被閫氱煡鏃跺仠姝㈢湅闂ㄧ嫍銆?- restart_nb锛氫负鏈哄櫒閲嶅惎娉ㄥ唽鐨?notifier 鍧楋紝浠呬緵鍐呴儴浣跨敤銆傚鏋滅湅闂ㄧ嫍鑳藉閲嶅惎鏈哄櫒锛?  瀹冨簲瀹氫箟 ops->restart銆備紭鍏堢骇鍙€氳繃 watchdog_set_restart_priority 鏇存敼銆?- bootstatus锛氬惎鍔ㄥ悗璁惧鐨勭姸鎬侊紙浠ョ湅闂ㄧ嫍 WDIOF_* 鐘舵€佷綅鎶ュ憡锛夈€?- driver_data锛氭寚鍚戠湅闂ㄧ嫍璁惧椹卞姩绉佹湁鏁版嵁鐨勬寚閽堛€傝鏁版嵁搴斾粎閫氳繃 watchdog_set_drvdata
  涓?watchdog_get_drvdata 渚嬬▼璁块棶銆?- wd_data锛氭寚鍚戠湅闂ㄧ嫍鏍稿績鍐呴儴鏁版嵁鐨勬寚閽堛€?- status锛氳瀛楁鍖呭惈涓€浜涚姸鎬佷綅锛屾彁渚涘叧浜庤澶囩姸鎬佺殑棰濆淇℃伅锛堜緥濡傦細鐪嬮棬鐙楀畾鏃跺櫒
  鏄惁姝ｅ湪杩愯/婵€娲伙紝鎴?nowayout 浣嶆槸鍚﹀凡璁剧疆锛夈€?- deferred锛歸td_deferred_reg_list 涓殑涓€椤癸紝鐢ㄤ簬娉ㄥ唽鎻愬墠鍒濆鍖栫殑鐪嬮棬鐙椼€?
```

  struct watchdog_ops {
	struct module *owner;
	/* mandatory operations */
	int (*start)(struct watchdog_device *);
	/* optional operations */
	int (*stop)(struct watchdog_device *);
	int (*ping)(struct watchdog_device *);
	unsigned int (*status)(struct watchdog_device *);
	int (*set_timeout)(struct watchdog_device *, unsigned int);
	int (*set_pretimeout)(struct watchdog_device *, unsigned int);
	unsigned int (*get_timeleft)(struct watchdog_device *);
	int (*restart)(struct watchdog_device *);
	long (*ioctl)(struct watchdog_device *, unsigned int, unsigned long);
  };

```
棣栧厛瀹氫箟鐪嬮棬鐙楀畾鏃跺櫒椹卞姩鎿嶄綔鐨勬ā鍧楁墍鏈夎€呴潪甯搁噸瑕併€傝妯″潡鎵€鏈夎€呯敤浜庡湪鐪嬮棬鐙?婵€娲绘椂閿佸畾妯″潡锛堣繖鏄负浜嗛伩鍏嶅湪鍗歌浇妯″潡鑰?/dev/watchdog 浠嶆墦寮€鏃堕€犳垚绯荤粺宕╂簝锛夈€?
鏈変簺鎿嶄綔鏄己鍒剁殑锛屾湁浜涙槸鍙€夌殑銆傚己鍒剁殑鎿嶄綔鏄細

- start锛氳繖鏄竴涓寚鍚戝惎鍔ㄧ湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囦緥绋嬬殑鎸囬拡銆傝渚嬬▼闇€瑕佷互鐪嬮棬鐙楀畾鏃跺櫒
  璁惧缁撴瀯涓哄弬鏁般€傛垚鍔熸椂杩斿洖闆讹紝澶辫触鏃惰繑鍥炶礋鐨?errno 鐮併€?
骞堕潪鎵€鏈夌湅闂ㄧ嫍瀹氭椂鍣ㄧ‖浠堕兘鏀寔鐩稿悓鐨勫姛鑳姐€傝繖灏辨槸涓轰粈涔堟墍鏈夊叾浠栦緥绋?鎿嶄綔閮芥槸
鍙€夌殑銆傚畠浠彧闇€瑕佸湪鍙楁敮鎸佹椂鎵嶉渶瑕佹彁渚涖€傝繖浜涘彲閫夌殑渚嬬▼/鎿嶄綔鏄細

- stop锛氶€氳繃璇ヤ緥绋嬪仠姝㈢湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囥€傝渚嬬▼闇€瑕佷互鐪嬮棬鐙楀畾鏃跺櫒璁惧缁撴瀯涓?  鍙傛暟銆傛垚鍔熸椂杩斿洖闆讹紝澶辫触鏃惰繑鍥炶礋鐨?errno 鐮併€傛湁浜涚湅闂ㄧ嫍瀹氭椂鍣ㄧ‖浠跺彧鑳藉惎鍔ㄨ€?  涓嶈兘鍋滄銆傛敮鎸佹绫荤‖浠剁殑椹卞姩鏃犻渶瀹炵幇 stop 渚嬬▼銆傚鏋滈┍鍔ㄦ病鏈?stop 鍑芥暟锛岀湅闂ㄧ嫍
  鏍稿績浼氳缃?WDOG_HW_RUNNING锛屽苟鍦ㄧ湅闂ㄧ嫍璁惧鍏抽棴鍚庡紑濮嬭皟鐢ㄩ┍鍔ㄧ殑 keepalive ping
  鍑芥暟銆傚鏋滅湅闂ㄧ嫍椹卞姩娌℃湁瀹炵幇 stop 鍑芥暟锛屽畠蹇呴』璁剧疆 max_hw_heartbeat_ms銆?- ping锛氳繖鏄悜鐪嬮棬鐙楀畾鏃跺櫒纭欢鍙戦€?keepalive ping 鐨勪緥绋嬨€傝渚嬬▼闇€瑕佷互鐪嬮棬鐙?  瀹氭椂鍣ㄨ澶囩粨鏋勪负鍙傛暟銆傛垚鍔熸椂杩斿洖闆讹紝澶辫触鏃惰繑鍥炶礋鐨?errno 鐮併€傚ぇ澶氭暟涓嶆敮鎸?  灏嗗叾浣滀负鐙珛鍔熻兘鐨勭‖浠朵細浣跨敤 start 鍑芥暟鏉ラ噸鍚湅闂ㄧ嫍瀹氭椂鍣ㄧ‖浠躲€傝€岃繖姝ｆ槸鐪嬮棬鐙?  瀹氭椂鍣ㄩ┍鍔ㄦ牳蹇冩墍鍋氱殑锛氫负浜嗗悜鐪嬮棬鐙楀畾鏃跺櫒纭欢鍙戦€?keepalive ping锛屽畠瑕佷箞浣跨敤
  ping 鎿嶄綔锛堝彲鐢ㄦ椂锛夛紝瑕佷箞浣跨敤 start 鎿嶄綔锛坧ing 鎿嶄綔涓嶅彲鐢ㄦ椂锛夈€傦紙娉ㄦ剰锛歐DIOC_KEEPALIVE
  ioctl 璋冪敤浠呭湪鐪嬮棬鐙?info 缁撴瀯鐨?option 瀛楁涓缃簡 WDIOF_KEEPALIVEPING 浣嶆椂
  鎵嶄細鐢熸晥锛夈€?- status锛氳渚嬬▼妫€鏌ョ湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囩殑鐘舵€併€傝澶囩姸鎬佷互鐪嬮棬鐙?WDIOF_* 鐘舵€佹爣蹇?
  浣嶆姤鍛娿€俉DIOF_MAGICCLOSE 涓?WDIOF_KEEPALIVEPING 鐢辩湅闂ㄧ嫍鏍稿績鎶ュ憡锛涙棤闇€浠庨┍鍔?  鎶ュ憡杩欎簺浣嶃€傛澶栵紝濡傛灉椹卞姩鏈彁渚?status 鍑芥暟锛岀湅闂ㄧ嫍鏍稿績浼氭姤鍛?struct
  watchdog_device 鐨?bootstatus 鍙橀噺涓彁渚涚殑鐘舵€佷綅銆?- set_timeout锛氳渚嬬▼妫€鏌ュ苟鏇存敼鐪嬮棬鐙楀畾鏃跺櫒璁惧鐨勮秴鏃躲€傛垚鍔熸椂杩斿洖 0锛屸€滃弬鏁拌秴鍑?  鑼冨洿鈥濊繑鍥?-EINVAL锛屸€滄棤娉曞皢鍊煎啓鍏ョ湅闂ㄧ嫍鈥濊繑鍥?-EIO銆傛垚鍔熸椂锛岃渚嬬▼搴斿皢
  watchdog_device 鐨勮秴鏃跺€艰缃负瀹為檯杈惧埌鐨勮秴鏃跺€硷紙鍙兘涓庤姹傚€间笉鍚岋紝鍥犱负鐪嬮棬鐙?  涓嶄竴瀹氬叿鏈?1 绉掔殑鍒嗚鲸鐜囷級銆傚疄鐜颁簡 max_hw_heartbeat_ms 鐨勯┍鍔ㄤ細灏嗙‖浠剁湅闂ㄧ嫍蹇冭烦
  璁剧疆涓?timeout 涓?max_hw_heartbeat_ms 涓殑杈冨皬鑰呫€傝繖浜涢┍鍔ㄥ皢 watchdog_device 鐨?  瓒呮椂鍊艰缃负璇锋眰鐨勮秴鏃跺€硷紙濡傛灉瀹冨ぇ浜?max_hw_heartbeat_ms锛夛紝鎴栬€呰缃负瀹為檯杈惧埌
  鐨勮秴鏃跺€笺€傦紙娉ㄦ剰锛氶渶瑕佸湪鐪嬮棬鐙?info 缁撴瀯鐨?options 瀛楁涓缃?WDIOF_SETTIMEOUT锛夈€?  濡傛灉鐪嬮棬鐙楅┍鍔ㄩ櫎浜嗚缃?watchdog_device.timeout 涔嬪鏃犻渶鎵ц浠讳綍鍔ㄤ綔锛屽垯鍙互鐪佺暐
  姝ゅ洖璋冦€傚鏋滄湭鎻愪緵 set_timeout 浣嗚缃簡 WDIOF_SETTIMEOUT锛岀湅闂ㄧ嫍鍩虹璁炬柦浼氬湪
  鍐呴儴灏?watchdog_device 鐨勮秴鏃跺€兼洿鏂颁负璇锋眰鍊笺€傚鏋滀娇鐢ㄤ簡 pretimeout 鐗规€?  锛圵DIOF_PRETIMEOUT锛夛紝閭ｄ箞 set_timeout 杩樺繀椤昏礋璐ｆ鏌?pretimeout 鏄惁浠嶇劧鏈夋晥锛?  骞剁浉搴斿湴璁剧疆瀹氭椂鍣ㄣ€傝繖鍦ㄦ牳蹇冧腑鏃犳硶鍦ㄦ棤绔炰簤鐨勬儏鍐典笅瀹屾垚锛屽洜姝ゆ槸椹卞姩鐨勮亴璐ｃ€?- set_pretimeout锛氳渚嬬▼妫€鏌ュ苟鏇存敼鐪嬮棬鐙楃殑 pretimeout 鍊笺€傚畠鏄彲閫夌殑锛屽洜涓哄苟闈?  鎵€鏈夌湅闂ㄧ嫍閮芥敮鎸?pretimeout 閫氱煡銆傝瓒呮椂鍊煎苟闈炵粷瀵规椂闂达紝鑰屾槸璺濈瀹為檯瓒呮椂鍙戠敓
  涔嬪墠鐨勭鏁般€傛垚鍔熸椂杩斿洖 0锛屸€滃弬鏁拌秴鍑鸿寖鍥粹€濊繑鍥?-EINVAL锛屸€滄棤娉曞皢鍊煎啓鍏ョ湅闂ㄧ嫍鈥?  杩斿洖 -EIO銆傚€?0 琛ㄧず绂佺敤 pretimeout 閫氱煡銆傦紙娉ㄦ剰锛氶渶瑕佸湪鐪嬮棬鐙?info 缁撴瀯鐨?  options 瀛楁涓缃?WDIOF_PRETIMEOUT锛夈€傚鏋滅湅闂ㄧ嫍椹卞姩闄や簡璁剧疆
  watchdog_device.pretimeout 涔嬪鏃犻渶鎵ц浠讳綍鍔ㄤ綔锛屽垯鍙互鐪佺暐姝ゅ洖璋冦€傝繖鎰忓懗鐫€濡傛灉
  鏈彁渚?set_pretimeout 浣嗚缃簡 WDIOF_PRETIMEOUT锛岀湅闂ㄧ嫍鍩虹璁炬柦浼氬湪鍐呴儴灏?  watchdog_device 鐨?pretimeout 鍊兼洿鏂颁负璇锋眰鍊笺€?- get_timeleft锛氳渚嬬▼杩斿洖閲嶅惎涔嬪墠鍓╀綑鐨勬椂闂淬€?- restart锛氳渚嬬▼閲嶅惎鏈哄櫒銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖璐熺殑 errno 鐮併€?- ioctl锛氬鏋滃瓨鍦ㄦ渚嬬▼锛岄偅涔堝畠浼氬湪鎴戜滑鑷繁鐨勫唴閮?ioctl 璋冪敤澶勭悊涔嬪墠棣栧厛琚皟鐢ㄣ€?  褰撳懡浠や笉鍙楁敮鎸佹椂锛岃渚嬬▼搴旇繑鍥?-ENOIOCTLCMD銆備紶閫掔粰 ioctl 璋冪敤鐨勫弬鏁版槸锛?  watchdog_device銆乧md 涓?arg銆?
鐘舵€佷綅锛堟渶濂斤級搴斾娇鐢?set_bit 涓?clear_bit 涔嬬被鐨勪綅鎿嶄綔鏉ヨ缃€傛墍瀹氫箟鐨勭姸鎬佷綅
濡備笅锛?
- WDOG_ACTIVE锛氳鐘舵€佷綅浠庣敤鎴疯搴︽寚绀虹湅闂ㄧ嫍瀹氭椂鍣ㄨ澶囨槸鍚﹀浜庢椿鍔ㄧ姸鎬併€傚湪姝ゆ爣蹇?  琚缃湡闂达紝鐢ㄦ埛绌洪棿搴斿悜椹卞姩鍙戦€佸績璺宠姹傘€?- WDOG_NO_WAY_OUT锛氳浣嶅瓨鍌ㄧ湅闂ㄧ嫍鐨?nowayout 璁剧疆銆傚鏋滆缃簡璇ヤ綅锛屽垯鐪嬮棬鐙楀畾鏃跺櫒
  灏嗘棤娉曞仠姝€?- WDOG_HW_RUNNING锛氬鏋滅‖浠剁湅闂ㄧ嫍姝ｅ湪杩愯锛岀敱鐪嬮棬鐙楅┍鍔ㄨ缃€傚鏋滅湅闂ㄧ嫍瀹氭椂鍣ㄧ‖浠?  鏃犳硶鍋滄锛屽垯蹇呴』璁剧疆璇ヤ綅銆傚鏋滅湅闂ㄧ嫍瀹氭椂鍣ㄥ湪鍚姩鍚庛€佺湅闂ㄧ嫍璁惧琚墦寮€涔嬪墠灏卞湪
  杩愯锛屼篃鍙互璁剧疆璇ヤ綅銆傚鏋滆缃紝鐪嬮棬鐙楀熀纭€璁炬柦浼氬湪 WDOG_ACTIVE 鏈缃椂鍚戠湅闂ㄧ嫍
  纭欢鍙戦€?keepalive銆傛敞鎰忥細褰撲綘甯︾潃璇ヤ綅琚缃潵娉ㄥ唽鐪嬮棬鐙楀畾鏃跺櫒璁惧鏃讹紝鎵撳紑
  /dev/watchdog 灏嗚烦杩?start 鎿嶄綔锛岃€屾槸鍙戦€佷竴涓?keepalive 璇锋眰銆?
  瑕佽缃?WDOG_NO_WAY_OUT 鐘舵€佷綅锛堝湪娉ㄥ唽浣犵殑鐪嬮棬鐙楀畾鏃跺櫒璁惧涔嬪墠锛夛紝浣犲彲浠ワ細

  - 鍦ㄤ綘鐨?watchdog_device 缁撴瀯涓潤鎬佽缃?
	.status = WATCHDOG_NOWAYOUT_INIT_STATUS,

    锛堣繖浼氬皢鍊艰缃负涓?CONFIG_WATCHDOG_NOWAYOUT 鐩稿悓锛夋垨
```

	static inline void watchdog_set_nowayout(struct watchdog_device *wdd,
						 int nowayout)

```
娉ㄦ剰锛?   鐪嬮棬鐙楀畾鏃跺櫒椹卞姩鏍稿績鏀寔 magic close 鐗规€т笌 nowayout 鐗规€с€傝浣跨敤 magic close
   鐗规€э紝浣犲繀椤诲湪鐪嬮棬鐙?info 缁撴瀯鐨?options 瀛楁涓缃?WDIOF_MAGICCLOSE 浣嶃€?
nowayout 鐗规€т細瑕嗙洊 magic close 鐗规€с€?
瑕佽幏鍙栨垨璁剧疆椹卞姩鐗瑰畾鏁版嵁锛屽簲浣跨敤浠ヤ笅涓や釜杈呭姪鍑芥暟
```

  static inline void watchdog_set_drvdata(struct watchdog_device *wdd,
					  void *data)
  static inline void *watchdog_get_drvdata(struct watchdog_device *wdd)

```
watchdog_set_drvdata 鍑芥暟鍏佽浣犳坊鍔犻┍鍔ㄧ壒瀹氭暟鎹€傝鍑芥暟鐨勫弬鏁版槸浣犺鍚戝叾娣诲姞椹卞姩
鐗瑰畾鏁版嵁鐨勭湅闂ㄧ嫍璁惧锛屼互鍙婃寚鍚戞暟鎹湰韬殑鎸囬拡銆?
watchdog_get_drvdata 鍑芥暟鍏佽浣犲彇鍥為┍鍔ㄧ壒瀹氭暟鎹€傝鍑芥暟鐨勫弬鏁版槸浣犺浠庝腑鍙栧洖鏁版嵁
鐨勭湅闂ㄧ嫍璁惧銆傝鍑芥暟杩斿洖鎸囧悜椹卞姩鐗瑰畾鏁版嵁鐨勬寚閽堛€?
```

  extern int watchdog_init_timeout(struct watchdog_device *wdd,
                                   unsigned int timeout_parm,
                                   const struct device *dev);

```
watchdog_init_timeout 鍑芥暟鍏佽浣犱娇鐢ㄦā鍧?timeout 鍙傛暟锛屾垨浠庤澶囨爲鑾峰彇 timeout-sec
灞炴€э紙濡傛灉妯″潡 timeout 鍙傛暟鏃犳晥锛夋潵鍒濆鍖?timeout 瀛楁銆傛渶浣冲疄璺垫槸鍏堝皢榛樿瓒呮椂鍊?璁句负 watchdog_device 涓殑瓒呮椂鍊硷紝鐒跺悗浣跨敤姝ゅ嚱鏁拌缃敤鎴封€滃亸濂解€濈殑瓒呮椂鍊笺€傝渚嬬▼
鎴愬姛鏃惰繑鍥為浂锛屽け璐ユ椂杩斿洖璐熺殑 errno 鐮併€?
```

  static inline void watchdog_stop_on_reboot(struct watchdog_device *wdd);

```
瑕佸湪娉ㄩ攢鐪嬮棬鐙楁椂绂佺敤瀹冿紝鐢ㄦ埛蹇呴』璋冪敤浠ヤ笅杈呭姪鍑芥暟銆傛敞鎰忥紝鍙湁褰?nowayout 鏍囧織
鏈缃椂锛岃繖鎵嶄細鍋滄鐪嬮棬鐙椼€?
```

  static inline void watchdog_stop_on_unregister(struct watchdog_device *wdd);

```
瑕佹洿鏀归噸鍚鐞嗙▼搴忕殑浼樺厛绾э紝搴斾娇鐢ㄤ互涓嬭緟鍔╁嚱鏁?```

  void watchdog_set_restart_priority(struct watchdog_device *wdd, int priority);

```
鐢ㄦ埛搴旈伒寰互涓嬭缃紭鍏堢骇鐨勫噯鍒欙細

- 0锛氬簲鍦ㄦ渶鍚庣殑鎵嬫涓皟鐢紝閲嶅惎鑳藉姏鏈夐檺
- 128锛氶粯璁ら噸鍚鐞嗙▼搴忥紝鍦ㄩ鏈熸病鏈夊叾浠栧鐞嗙▼搴忓彲鐢紝鍜?鎴栭噸鍚冻浠ラ噸鍚暣涓郴缁?  鏃朵娇鐢?- 255锛氭渶楂樹紭鍏堢骇锛屽皢鎶㈠崰鎵€鏈夊叾浠栭噸鍚鐞嗙▼搴?
```

  void watchdog_notify_pretimeout(struct watchdog_device *wdd)

```
璇ュ嚱鏁板彲浠ュ湪涓柇涓婁笅鏂囦腑璋冪敤銆傚鏋滃惎鐢ㄤ簡鐪嬮棬鐙?pretimeout 绠＄悊鍣ㄦ鏋讹紙kbuild
CONFIG_WATCHDOG_PRETIMEOUT_GOV 绗﹀彿锛夛紝鍒欑敱棰勫厛鍒嗛厤缁欑湅闂ㄧ嫍璁惧鐨勩€侀鍏堥厤缃ソ鐨?pretimeout 绠＄悊鍣ㄩ噰鍙栬鍔ㄣ€傚鏋滄湭鍚敤鐪嬮棬鐙?pretimeout 绠＄悊鍣ㄦ鏋讹紝
watchdog_notify_pretimeout() 浼氬悜鍐呮牳鏃ュ織缂撳啿鍖烘墦鍗颁竴鏉￠€氱煡娑堟伅銆?
瑕佽缃湅闂ㄧ嫍鏈€鍚庝竴娆″凡鐭ョ殑纭欢 keepalive 鏃堕棿锛屼娇鐢ㄤ互涓嬪嚱鏁?```

  int watchdog_set_last_hw_keepalive(struct watchdog_device *wdd,
                                     unsigned int last_ping_ms)

```
璇ュ嚱鏁板繀椤诲湪鐪嬮棬鐙楁敞鍐屼箣鍚庣珛鍗宠皟鐢ㄣ€傚畠灏嗘渶鍚庝竴娆″凡鐭ョ殑纭欢蹇冭烦璁剧疆涓哄湪褰撳墠鏃堕棿
涔嬪墠 last_ping_ms 姣鏃跺彂鐢熴€傚彧鏈夊綋 probe 琚皟鐢ㄦ椂鐪嬮棬鐙楀凡缁忓湪杩愯锛屼笖鐪嬮棬鐙楀彧鑳?鍦ㄨ嚜涓婃 ping 璧风粡杩?min_hw_heartbeat_ms 鏃堕棿涔嬪悗鎵嶈兘琚?ping 鏃讹紝鎵嶉渶瑕佽皟鐢ㄦ鍑芥暟銆?