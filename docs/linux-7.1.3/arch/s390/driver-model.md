## S/390 椹卞姩妯″瀷鎺ュ彛


### 1. CCW 璁惧


鎵€鏈夊彲浠ラ€氳繃 ccw 瀵诲潃鐨勮澶囬兘琚О涓?鈥淐CW 璁惧鈥濃€斺€斿嵆浣垮畠浠疄闄呬笂骞堕潪鐢?ccw 椹卞姩銆?
鎵€鏈?ccw 璁惧閮介€氳繃涓€涓瓙閫氶亾锛坰ubchannel锛夎闂紝杩欏弽鏄犲湪

```

  devices/
     - system/
     - css0/
	   - 0.0.0000/0.0.0815/
	   - 0.0.0001/0.0.4711/
	   - 0.0.0002/
	   - 0.1.0000/0.1.1234/
	   ...
	   - defunct/

```
鍦ㄦ湰渚嬩腑锛岃澶?0815 閫氳繃瀛愰€氶亾闆?0 涓殑瀛愰€氶亾 0 璁块棶锛岃澶?4711 閫氳繃瀛愰€氶亾闆?0 涓殑瀛愰€氶亾 1 璁块棶锛岃€屽瓙閫氶亾 2 鏄竴涓潪 I/O 瀛愰€氶亾銆傝澶?1234 閫氳繃瀛愰€氶亾闆?1 涓殑瀛愰€氶亾 0 璁块棶銆?
鍚嶄负 鈥渄efunct鈥?鐨勫瓙閫氶亾涓嶄唬琛ㄧ郴缁熶笂浠讳綍鐪熷疄鐨勫瓙閫氶亾锛涘畠鏄竴涓吉瀛愰€氶亾锛屽綋鏂紑杩炴帴鐨?ccw 璁惧琚彟涓€涓湪鍏跺師瀛愰€氶亾涓婂彉涓哄彲鎿嶄綔鐨?ccw 璁惧鎸ゅ崰鏃讹紝杩欎簺鏂紑杩炴帴鐨?ccw 璁惧浼氳绉诲埌閭ｉ噷銆傚鏋滈偅浜?ccw 璁惧鍦ㄨ瀛愰€氶亾涓婂啀娆″彉涓哄彲鎿嶄綔锛屽畠浠細琚啀娆＄Щ鍥炲悎閫傜殑瀛愰€氶亾銆?
鎮ㄥ簲璇ラ€氳繃鍏?bus id锛堜緥濡?0.0.4711锛夋潵瀵诲潃涓€涓?ccw 璁惧锛涜璁惧鍙互鍦?bus/ccw/devices/ 涓嬫壘鍒般€?
鎵€鏈?ccw 璁惧閮介€氳繃 sysfs 瀵煎嚭涓€浜涙暟鎹€?
cutype:
	鎺у埗閮ㄤ欢锛坈ontrol unit锛夌被鍨?/ 鍨嬪彿銆?
devtype:
	璁惧绫诲瀷 / 鍨嬪彿锛堝鏋滈€傜敤锛夈€?
availability:
	      鍙互鏄?鈥済ood鈥?鎴?鈥渂oxed鈥濓紱瀵逛簬鏂紑杩炴帴鐨勮澶囧垯鏄?	      鈥渘o path鈥?鎴?鈥渘o device鈥濄€?
online:
	   涓€涓敤浜庡皢璁惧涓婄嚎锛坥nline锛夊拰涓嬬嚎锛坥ffline锛夌殑鎺ュ彛銆?	   鍦ㄨ澶囨柇寮€杩炴帴鐨勭壒娈婃儏鍐典笅锛堣 1.2 涓嬬殑 notify 鍑芥暟锛夛紝
	   鍚?online 鍐欏叆 0 灏嗗己鍒跺垹闄よ璁惧銆?
璁惧椹卞姩鍙互娣诲姞鏉＄洰鏉ュ鍑烘瘡璁惧鐨勬暟鎹拰鎺ュ彛銆?
杩樻湁涓€浜涙暟鎹槸鎸夊瓙閫氶亾瀵煎嚭鐨勶紙瑙?bus/css/devices/锛夛細

chpids:
	璁惧閫氳繃鍝簺 chpid 杩炴帴銆?
pimpampom:
	璺緞宸插畨瑁呫€佽矾寰勫彲鐢ㄥ拰璺緞鍙搷浣滅殑鎺╃爜銆?
鍙兘杩樻湁棰濆鐨勬暟鎹紝渚嬪閽堝鍧楄澶囥€?

### 1.1 鍚姩涓€涓?ccw 璁惧


杩欏垎鍑犱釜姝ラ瀹屾垚銆?
a. 姣忎釜椹卞姩鍙互鎻愪緵涓€涓垨澶氫釜鍙傛暟鎺ュ彛锛岀敤浜庢寚瀹氬弬鏁般€傝繖浜涙帴鍙ｄ篃鐢遍┍鍔ㄨ礋璐ｃ€?b. 鍦ㄦ墽琛?a. 涔嬪悗锛屽鏈夊繀瑕侊紝鏈€缁堥€氳繃 鈥渙nline鈥?鎺ュ彛鍚姩璁惧銆?

### 1.2 涓?ccw 璁惧缂栧啓椹卞姩


鍩烘湰鐨?struct ccw_device 鍜?struct ccw_driver 鏁版嵁缁撴瀯鍙互鍦?
```

  struct ccw_device {
	spinlock_t *ccwlock;
	struct ccw_device_private *private;
	struct ccw_device_id id;

	struct ccw_driver *drv;
	struct device dev;
	int online;

	void (*handler) (struct ccw_device *dev, unsigned long intparm,
			 struct irb *irb);
  };

  struct ccw_driver {
	struct module *owner;
	struct ccw_device_id *ids;
	int (*probe) (struct ccw_device *);
	int (*remove) (struct ccw_device *);
	int (*set_online) (struct ccw_device *);
	int (*set_offline) (struct ccw_device *);
	int (*notify) (struct ccw_device *, int);
	struct device_driver driver;
	char *name;
  };

```
鈥減rivate鈥?瀛楁鍙寘鍚唴閮?I/O 鎿嶄綔鎵€闇€鐨勬暟鎹紝璁惧椹卞姩涓嶅彲璁块棶銆?
姣忎釜椹卞姩搴斿湪 MODULE_DEVICE_TABLE 涓０鏄庡畠鎰熷叴瓒ｅ摢浜?CU 绫诲瀷/鍨嬪彿鍜?鎴栬澶囩被鍨?鍨嬪彿銆傛淇℃伅涔嬪悗鍙互鍦?
```

  struct ccw_device_id {
	__u16   match_flags;

	__u16   cu_type;
	__u16   dev_type;
	__u8    cu_model;
	__u8    dev_model;

	unsigned long driver_info;
  };

```
ccw_driver 涓殑鍑芥暟搴旀寜濡備笅鏂瑰紡浣跨敤锛?
probe:
	 璁惧灞備负姣忎釜璇ラ┍鍔ㄦ劅鍏磋叮鐨勮澶囪皟鐢ㄦ鍑芥暟銆傞┍鍔ㄥ簲鍙垎閰嶇鏈夌粨鏋勬斁鍏?dev->driver_data 骞跺垱寤哄睘鎬э紙濡傛灉闇€瑕侊級銆傚悓鏃讹紝搴斿湪姝ゅ璁剧疆涓柇澶勭悊绋嬪簭锛堣涓嬫枃锛夈€?
```

  int (*probe) (struct ccw_device *cdev);

```
鍙傛暟锛?		cdev
   - 瑕?probe 鐨勮澶囥€?

remove:
	 璁惧灞傚湪绉婚櫎椹卞姩銆佽澶囨垨妯″潡鏃惰皟鐢ㄦ鍑芥暟銆傞┍鍔ㄥ簲鍦ㄦ澶勬墽琛屾竻鐞嗐€?
```

  int (*remove) (struct ccw_device *cdev);

```
鍙傛暟锛?		cdev
   - 瑕佺Щ闄ょ殑璁惧銆?

set_online:
	    鍏叡 I/O 灞傚湪閫氳繃 鈥渙nline鈥?灞炴€ф縺娲昏澶囨椂璋冪敤姝ゅ嚱鏁般€傞┍鍔ㄥ簲鏈€缁堝湪姝ゅ璁剧疆鍜屾縺娲昏澶囥€?
```

  int (*set_online) (struct ccw_device *);

```
鍙傛暟锛?		cdev
   - 瑕佹縺娲荤殑璁惧銆傚叕鍏卞眰宸查獙璇佽璁惧灏氭湭 online銆?

set_offline: 鍏叡 I/O 灞傚湪閫氳繃 鈥渙nline鈥?灞炴€у仠鐢ㄨ澶囨椂璋冪敤姝ゅ嚱鏁般€傞┍鍔ㄥ簲鍏抽棴璁惧锛屼絾涓嶉噴鏀惧叾绉佹湁鏁版嵁銆?
```

  int (*set_offline) (struct ccw_device *);

```
鍙傛暟锛?		cdev
   - 瑕佸仠鐢ㄧ殑璁惧銆傚叕鍏卞眰宸查獙璇佽璁惧澶勪簬 online銆?

notify:
	鍏叡 I/O 灞傚湪璁惧鐨勬煇浜涚姸鎬佹敼鍙樻椂璋冪敤姝ゅ嚱鏁般€?
	鍚戦┍鍔ㄥ彂鍑虹殑淇″彿鏈夛細

 - 鍦?online 鐘舵€佷笅锛岃澶囧垎绂伙紙CIO_GONE锛夋垨鏈€鍚庝竴鏉¤矾寰勬秷澶?	  锛圕IO_NO_PATH锛夈€傞┍鍔ㄥ繀椤昏繑鍥?!0 浠ヤ繚鐣欒澶囷紱瀵逛簬
	   杩斿洖鐮?0锛岃澶囧皢鐓у父琚垹闄わ紙鍗充娇娌℃湁娉ㄥ唽 notify 鍑芥暟鏃朵篃鏄姝わ級銆傚鏋滈┍鍔ㄦ兂瑕佷繚鐣?	   璁惧锛屽畠浼氳绉诲叆鏂紑杩炴帴鐘舵€併€? - 鍦ㄦ柇寮€杩炴帴鐘舵€佷笅锛岃澶囧啀娆″彲鎿嶄綔锛圕IO_OPER锛夈€傚叕鍏?I/O 灞傚璁惧鍙峰拰
	   Device / CU 鎵ц涓€浜涘畬鏁存€ф鏌ワ紝浠ュ悎鐞嗙‘淇″畠鏄惁浠嶆槸鍚屼竴璁惧銆?	   濡傛灉涓嶆槸锛屾棫璁惧琚Щ闄ゅ苟娉ㄥ唽涓€涓柊璁惧銆傞€氳繃 notify 鍑芥暟鐨勮繑鍥炵爜锛?	   璁惧椹卞姩琛ㄦ槑瀹冩槸鍚︽兂瑕佸洖璇ヨ澶囷細!0 琛ㄧず淇濈暀锛? 琛ㄧず灏嗚澶囩Щ闄ゅ苟閲嶆柊娉ㄥ唽銆?
```

  int (*notify) (struct ccw_device *, int);

```
鍙傛暟锛?		cdev
   - 鐘舵€佹敼鍙樼殑璁惧銆?
		event
   - 鍙戠敓鐨勪簨浠躲€傚彲浠ユ槸 CIO_GONE銆?			  CIO_NO_PATH 鎴?CIO_OPER 涔嬩竴銆?
struct ccw_device 鐨?handler 瀛楁鐢ㄤ簬璁剧疆涓鸿璁惧鐨勪腑鏂鐞嗙▼搴忋€備负浜嗛€傚簲浣跨敤澶氫釜涓嶅悓澶勭悊绋嬪簭锛堜緥濡傚瀛愰€氶亾璁惧锛夌殑椹卞姩锛岃繖鏄?ccw_device 鐨勬垚鍛樿€屼笉鏄?ccw_driver 鐨勬垚鍛樸€?handler 鍦?set_online() 澶勭悊鏈熼棿銆佸湪璋冪敤椹卞姩涔嬪墠鍚戝叕鍏卞眰娉ㄥ唽锛屽苟鍦?set_offline() 鏈熼棿銆佸湪璋冪敤椹卞姩涔嬪悗娉ㄩ攢銆傛澶栵紝鍦ㄦ敞鍐屼箣鍚?/ 娉ㄩ攢涔嬪墠锛屼細鎵ц璺緞鍒嗙粍锛坧ath grouping锛夋垨璺緞缁勭殑瑙ｆ暎锛堝鏋滈€傜敤锛夈€?
```

  void (*handler) (struct ccw_device *dev, unsigned long intparm, struct irb *irb);

```
鍙傛暟锛?    dev     - 璋冪敤 handler 鐨勮澶?		intparm - 鍏佽璁惧椹卞姩璇嗗埆涓柇鎵€鍏宠仈鐨?i/o锛?			  鎴栧皢涓柇璇嗗埆涓烘湭璇锋眰鐨勶紙unsolicited锛夈€?		irb     - 鍖呭惈绱鐘舵€佺殑涓柇鍝嶅簲鍧楋紙interruption response block锛夈€?
璁惧椹卞姩浠庡叕鍏?ccw_device 灞傝皟鐢紝骞跺彲浠ヤ粠 irb 鍙傛暟妫€绱㈡湁鍏充腑鏂殑淇℃伅銆?

### 1.3 ccwgroup 璁惧


ccwgroup 鏈哄埗璁捐鐢ㄤ簬澶勭悊鐢卞涓?ccw 璁惧缁勬垚鐨勮澶囷紝渚嬪 qeth 鎴?ctc銆?
ccw 椹卞姩鎻愪緵涓€涓?鈥済roup鈥?灞炴€с€傚皢 ccw 璁惧鐨?bus id 鍐欏叆姝ゅ睘鎬т細鍒涘缓涓€涓敱杩欎簺 ccw 璁惧缁勬垚鐨?ccwgroup 璁惧锛堝鏋滃彲鑳斤級銆傝繖涓?ccwgroup 璁惧鍙互鍍忔櫘閫氱殑 ccw 璁惧涓€鏍蜂笂/涓嬬嚎銆?
姣忎釜 ccwgroup 璁惧杩樻彁渚涗竴涓?鈥渦ngroup鈥?灞炴€т互鍐嶆閿€姣佽璁惧锛堜粎鍦ㄤ笅绾挎椂锛夈€傝繖鏄竴涓€氱敤鐨?ccwgroup 鏈哄埗锛堥┍鍔ㄤ笉闇€瑕佸疄鐜拌秴鍑烘甯哥Щ闄や緥绋嬩箣澶栫殑浠讳綍涓滆タ锛夈€?
浣滀负 ccwgroup 璁惧鎴愬憳鐨?ccw 璁惧锛屽湪鍏?device 缁撴瀯鐨?driver_data 涓惡甯︿竴涓寚鍚?ccwgroup 璁惧鐨勬寚閽堛€傞┍鍔ㄤ笉寰楄Е纰版瀛楁鈥斺€斿畠搴斾娇鐢?ccwgroup 璁惧鐨?driver_data 鏉ュ瓨鏀惧叾绉佹湁鏁版嵁銆?
瑕佸疄鐜?ccwgroup 椹卞姩锛岃鍙傞槄 include/asm/ccwgroup.h銆傝璁颁綇锛屽ぇ澶氭暟椹卞姩閮介渶瑕佸悓鏃跺疄鐜?ccwgroup 鍜?ccw 椹卞姩銆?

### 2. 閫氶亾璺緞锛圕hannel paths锛?

閫氶亾璺緞涓庡瓙閫氶亾涓€鏍凤紝鍑虹幇鍦ㄩ€氶亾瀛愮郴缁熸牴锛坈ss0锛変箣涓嬶紝琚О涓?鈥渃hp0.<chpid>鈥濄€傚畠浠病鏈夐┍鍔紝涔熶笉灞炰簬浠讳綍鎬荤嚎銆?璇锋敞鎰忥紝涓?2.4 涓殑 /proc/chpids 涓嶅悓锛岄€氶亾璺緞瀵硅薄鍙弽鏄犻€昏緫鐘舵€佽€屼笉鏄墿鐞嗙姸鎬侊紝鍥犱负鐢变簬缂哄皯鏈哄櫒鏀寔锛屾垜浠棤娉曚竴鑷村湴璺熻釜鍚庤€咃紙鍙嶆鎴戜滑涔熶笉闇€瑕佺煡閬撳畠锛夈€?
status
       - 鍙互鏄?鈥渙nline鈥?鎴?鈥渙ffline鈥濄€?	 鍐欏叆 鈥渙n鈥?鎴?鈥渙ff鈥?浼氬皢 chpid 閫昏緫鍦颁笂/涓嬬嚎銆?	 鍚戜竴涓凡涓婄嚎鐨?chpid 鍐欏叆 鈥渙n鈥?浼氳Е鍙戝鍏惰繛鎺ョ殑鎵€鏈夎澶囩殑璺緞閲嶆柊鎺㈡祴銆傝繖鍙敤浜庡己鍒跺唴鏍稿鐢ㄤ竴涓敤鎴风煡閬撳凡涓婄嚎銆佷絾鏈哄櫒灏氭湭涓哄叾鍒涘缓鏈哄櫒妫€鏌ョ殑閫氶亾璺緞銆?
type
       - 閫氶亾璺緞鐨勭墿鐞嗙被鍨嬨€?
shared
       - 閫氶亾璺緞鏄惁鍏变韩銆?
cmg
       - 閫氶亾娴嬮噺缁勶紙channel measurement group锛夈€?
### 3. 绯荤粺璁惧


### 3.1 xpram


xpram 浣滀负 鈥渪pram鈥?鍑虹幇鍦?devices/system/ 涓嬨€?
### 3.2 cpus


瀵逛簬姣忎釜 cpu锛屽湪 devices/system/cpu/ 涓嬪垱寤轰竴涓洰褰曘€傛瘡涓?cpu 鏈変竴涓睘鎬?鈥渙nline鈥濓紝鍏跺€煎彲浠ユ槸 0 鎴?1銆?