## Linux IPMI 椹卞姩


:Author: Corey Minyard <minyard@mvista.com> / <minyard@acm.org>

IPMI锛圛ntelligent Platform Management Interface锛屾櫤鑳藉钩鍙扮鐞嗘帴鍙ｏ級鏄竴绉?鐢ㄤ簬鎺у埗鐩戞帶绯荤粺鐨勬櫤鑳借澶囩殑鏍囧噯銆傚畠鏀寔绯荤粺涓紶鎰熷櫒鐨勫姩鎬佸彂鐜帮紝浠ュ強
鐩戣浼犳劅鍣ㄥ苟鍦ㄤ紶鎰熷櫒鏁板€煎彉鍖栨垨瓒呭嚭鏌愪簺杈圭晫鏃跺緱鍒伴€氱煡鐨勮兘鍔涖€傚畠杩樻嫢鏈?鐢ㄤ簬鐜板満鍙洿鎹㈠崟鍏冿紙FRU锛夌殑鏍囧噯鍖栨暟鎹簱浠ュ強涓€涓湅闂ㄧ嫍瀹氭椂鍣ㄣ€?
瑕佷娇鐢ㄥ畠锛屼綘闇€瑕佸湪绯荤粺涓湁涓€涓埌 IPMI 鎺у埗鍣ㄧ殑鎺ュ彛锛堢О涓哄熀鏉跨鐞嗘帶鍒跺櫒锛?鍗?BMC锛夛紝浠ュ強鑳藉浣跨敤 IPMI 绯荤粺鐨勭鐞嗚蒋浠躲€?
鏈枃妗ｆ弿杩板浣曞湪 Linux 涓嬩娇鐢?IPMI 椹卞姩銆傚鏋滀綘鏈韩涓嶇啛鎮?IPMI锛岃鍙傞槄
缃戠珯 https://www.intel.com/design/servers/ipmi/index.htm銆侷PMI 鏄釜寰堝ぇ鐨?涓婚锛屾垜鏃犳硶鍦ㄨ繖閲屽叏閮ㄨ鐩栵紒

### 閰嶇疆

Linux IPMI 椹卞姩鏄ā鍧楀寲鐨勶紝杩欐剰鍛崇潃浣犻渶瑕佹牴鎹綘鐨勭‖浠堕€夊彇鑻ュ共椤规墠鑳借瀹?姝ｅ父宸ヤ綔銆傚叾涓ぇ澶氭暟浣嶄簬 'Character Devices' 鑿滃崟涓嬬殑 IPMI 鑿滃崟涓€?
鏃犺濡備綍锛屼綘蹇呴』閫夊彇 'IPMI top-level message handler' 鎵嶈兘浣跨敤 IPMI銆傚湪姝?涔嬪鍋氫粈涔堝彇鍐充簬浣犵殑闇€姹傚拰纭欢銆?
娑堟伅澶勭悊绋嬪簭涓嶆彁渚涗换浣曠敤鎴风骇鎺ュ彛銆傚唴鏍镐唬鐮侊紙濡傜湅闂ㄧ嫍锛変粛鍙互浣跨敤瀹冦€傚鏋?浣犻渶瑕佷粠鐢ㄦ埛绌洪棿璁块棶锛屽苟涓旀兂閫氳繃璁惧椹卞姩璁块棶锛屽垯闇€瑕侀€夊彇 'Device interface
for IPMI'銆?
椹卞姩鎺ュ彛鍙栧喅浜庝綘鐨勭‖浠躲€傚鏋滅郴缁熸纭彁渚涗簡 IPMI 鐨?SMBIOS 淇℃伅锛岄┍鍔ㄤ細
妫€娴嬪埌瀹冨苟鐩存帴宸ヤ綔銆傚鏋滀綘鏈変竴鍧楀甫鏈夋爣鍑嗘帴鍙ｇ殑鏉垮瓙锛堥€氬父杩欎簺瑕佷箞鏄?"KCS"銆?"SMIC" 鎴?"BT"锛岃鏌ラ槄浣犵殑纭欢鎵嬪唽锛夛紝閫夋嫨 'IPMI SI handler' 閫夐」銆備篃瀛樺湪涓€涓?鐢ㄤ簬鐩存帴 I2C 璁块棶 IPMI 绠＄悊鎺у埗鍣ㄧ殑椹卞姩銆傛湁浜涙澘瀛愭敮鎸佽繖绉嶈闂紝浣嗕笉鐭ラ亾瀹?鏄惁鑳藉湪姣忓潡鏉垮瓙涓婇兘宸ヤ綔銆備负姝わ紝閫夋嫨 'IPMI SMBus handler'锛屼絾濡傛灉 SMBIOS/ACPI
淇℃伅閿欒鎴栦笉瀛樺湪锛屼綘瑕佸噯澶囧ソ鑷繁鍘绘懜绱㈠畠鑳藉惁鍦ㄤ綘鐨勭郴缁熶笂宸ヤ綔銆傚悓鏃跺惎鐢ㄨ繖涓や釜
閫氬父鏄畨鍏ㄧ殑锛岃椹卞姩鑷姩鎺㈡祴瀛樺湪鍝簺鎺ュ彛銆?
浣犻€氬父搴旇鍦ㄧ郴缁熶笂鍚敤 ACPI锛屽洜涓哄甫鏈?IPMI 鐨勭郴缁熷彲鑳芥嫢鏈夋弿杩板畠浠殑 ACPI
琛ㄣ€?
濡傛灉浣犳湁鏍囧噯鎺ュ彛涓旀澘瀛愬埗閫犲晢姝ｇ‘鍦板畬鎴愪簡浠栦滑鐨勫伐浣滐紝IPMI 鎺у埗鍣ㄥ簲褰撹
鑷姩妫€娴嬪埌锛堥€氳繃 ACPI 鎴?SMBIOS 琛級骞剁洿鎺ュ伐浣溿€傞仐鎲剧殑鏄紝璁稿鏉垮瓙娌℃湁
杩欎簺淇℃伅銆傞┍鍔ㄤ細灏濊瘯鏍囧噯榛樿鍊硷紝浣嗗畠浠彲鑳戒笉宸ヤ綔銆傚鏋滀綘閬囧埌杩欑鎯呭喌锛?浣犻渶瑕侀槄璇讳笅闈㈠悕涓?'The SI Driver' 鎴?"The SMBus Driver" 鐨勫皬鑺傦紝浜嗚В濡備綍
鎵嬪姩閰嶇疆浣犵殑绯荤粺銆?
IPMI 瀹氫箟浜嗕竴涓爣鍑嗙湅闂ㄧ嫍瀹氭椂鍣ㄣ€備綘鍙互鐢?'IPMI Watchdog Timer' 閰嶇疆閫夐」
鏉ュ惎鐢ㄥ畠銆傚鏋滀綘鎶婇┍鍔ㄧ紪璇戣繘鍐呮牳锛岄偅涔堥€氳繃鍐呮牳鍛戒护琛岄€夐」锛屽彲浠ヨ鐪嬮棬鐙?瀹氭椂鍣ㄥ湪鍒濆鍖栧悗绔嬪嵆鍚姩銆傚畠杩樻湁寰堝鍏朵粬閫夐」锛岃瑙佷笅鏂囩殑 'Watchdog' 灏忚妭銆?娉ㄦ剰锛屼綘涔熷彲浠ヨ鐪嬮棬鐙楀湪鍏抽棴鏃剁户缁繍琛岋紙榛樿鍦ㄥ叧闂椂绂佺敤锛夈€傝繘鍏?'Watchdog
Cards' 鑿滃崟锛屽惎鐢?'Watchdog Timer Support'锛屽苟鍚敤閫夐」 'Disable watchdog
shutdown on close'銆?
IPMI 绯荤粺閫氬父鍙互浣跨敤 IPMI 鍛戒护鍏虫満銆傞€夋嫨 'IPMI Poweroff' 鏉ュ仛鍒拌繖涓€鐐广€?椹卞姩浼氳嚜鍔ㄦ帰娴嬬郴缁熸槸鍚﹁兘琚?IPMI 鍏虫満銆傚嵆浣夸綘鐨勭郴缁熶笉鏀寔姝ら€夐」锛屽惎鐢ㄥ畠涔?鏄畨鍏ㄧ殑銆傝繖鍦?ATCA 绯荤粺銆丷adisys CPI1 鍗★紝浠ュ強浠讳綍鏀寔鏍囧噯鏈虹绠＄悊鍛戒护鐨?IPMI 绯荤粺涓婃湁鏁堛€?
濡傛灉浣犲笇鏈涢┍鍔ㄥ湪 panic 鏃跺悜浜嬩欢鏃ュ織鍐欏叆涓€涓簨浠讹紝鍚敤 'Generate a panic event
to all BMCs on a panic' 閫夐」銆傚鏋滀綘甯屾湜鐢?OEM 浜嬩欢鎶婃暣涓?panic 瀛楃涓插啓鍏ヤ簨浠?鏃ュ織锛屽惎鐢?'Generate OEM events containing the panic string' 閫夐」銆備綘涔熷彲浠ラ€氳繃灏?ipmi_msghandler 妯″潡涓悕涓?"panic_op" 鐨勬ā鍧楀弬鏁拌涓?"event" 鎴?"string" 鏉ュ姩鎬?鍚敤瀹冧滑銆傚皢璇ュ弬鏁拌涓?"none" 鍒欑鐢ㄦ鍔熻兘銆?
### 鍩烘湰璁捐

Linux IPMI 椹卞姩璁捐寰楅潪甯告ā鍧楀寲鍜岀伒娲伙紝浣犲彧闇€鍙栫敤浣犻渶瑕佺殑閮ㄥ垎锛屽氨鍙互鐢ㄥ绉?涓嶅悓鏂瑰紡浣跨敤瀹冦€傛鍥犲姝わ紝瀹冭鎷嗗垎鎴愪簡璁稿浠ｇ爜鍧椼€傝繖浜涗唬鐮佸潡锛堟寜妯″潡鍚嶏級鏄細

ipmi_msghandler - 杩欐槸 IPMI 绯荤粺鐨勬牳蹇冭蒋浠堕儴鍒嗐€傚畠澶勭悊鎵€鏈夋秷鎭€佹秷鎭椂搴忓拰
鍝嶅簲銆侷PMI 鐢ㄦ埛鎺ュ叆杩欓噷锛孖PMI 鐗╃悊鎺ュ彛锛堢О涓虹郴缁熺鐞嗘帴鍙ｏ紝鍗?SMI锛変篃鎺ュ叆杩欓噷銆?瀹冩彁渚?IPMI 鐨勫唴鏍告€佹帴鍙ｏ紝浣嗕笉鎻愪緵渚涘簲鐢ㄧ▼搴忚繘绋嬩娇鐢ㄧ殑鎺ュ彛銆?
ipmi_devintf - 杩欎负 IPMI 椹卞姩鎻愪緵涓€涓敤鎴锋€?IOCTL 鎺ュ彛锛屾璁惧鐨勬瘡娆℃墦寮€鏂囦欢
閮戒綔涓轰竴涓?IPMI 鐢ㄦ埛鎺ュ叆娑堟伅澶勭悊绋嬪簭銆?
ipmi_si - 涓€涓敤浜庡悇绉嶇郴缁熸帴鍙ｇ殑椹卞姩銆傚畠鏀寔 KCS銆丼MIC 鍜?BT 鎺ュ彛銆傞櫎闈炰綘鏈?SMBus 鎺ュ彛鎴栬嚜宸辩殑瀹氬埗鎺ュ彛锛屽惁鍒欎綘寰堝彲鑳介渶瑕佷娇鐢ㄥ畠銆?
ipmi_ssif - 涓€涓敤浜庤闂?SMBus 涓?BMC 鐨勯┍鍔ㄣ€傚畠浣跨敤 I2C 鍐呮牳椹卞姩鐨?SMBus 鎺ュ彛
鏉ラ€氳繃 SMBus 鏀跺彂 IPMI 娑堟伅銆?
ipmi_powernv - 涓€涓敤浜庤闂?POWERNV 绯荤粺涓?BMC 鐨勯┍鍔ㄣ€?
ipmi_watchdog - IPMI 瑕佹眰绯荤粺鍏峰涓€涓潪甯稿己澶х殑鐪嬮棬鐙楀畾鏃跺櫒銆傛椹卞姩鍦?IPMI 娑堟伅
澶勭悊绋嬪簭涔嬩笂瀹炵幇浜嗘爣鍑嗙殑 Linux 鐪嬮棬鐙楀畾鏃跺櫒鎺ュ彛銆?
ipmi_poweroff - 鏌愪簺绯荤粺鏀寔閫氳繃 IPMI 鍛戒护鍏虫満銆?
bt-bmc - 杩欎笉鏄富椹卞姩鐨勪竴閮ㄥ垎锛岃€屾槸涓€涓敤浜庤闂?BT 鎺ュ彛鐨?BMC 渚ф帴鍙ｇ殑椹卞姩銆?瀹冪敤浜庤繍琛?Linux 鐨?BMC锛屼互鍚戜富鏈烘彁渚涙帴鍙ｃ€?
杩欎簺閮藉彲浠ラ€氳繃閰嶇疆閫夐」鍗曠嫭閫夊彇銆?
鎺ュ彛鐨勫緢澶氭枃妗ｅ湪澶存枃浠朵腑銆侷PMI 澶存枃浠舵湁锛?
linux/ipmi.h - 鍖呭惈 IPMI 鐨勭敤鎴锋帴鍙ｅ拰 IOCTL 鎺ュ彛銆?
linux/ipmi_smi.h - 鍖呭惈渚涚郴缁熺鐞嗘帴鍙ｏ紙瀵规帴 IPMI 鎺у埗鍣ㄧ殑閭ｄ簺涓滆タ锛変娇鐢ㄧ殑鎺ュ彛銆?
linux/ipmi_msgdefs.h - 鍩虹 IPMI 娑堟伅浼犻€掔殑閫氱敤瀹氫箟銆?

### 瀵诲潃

IPMI 瀵诲潃宸ヤ綔璧锋潵寰堝儚 IP 鍦板潃锛屼綘鏈変竴涓鐩栧眰
```

  struct ipmi_addr
  {
	int   addr_type;
	short channel;
	char  data[IPMI_MAX_ADDR_SIZE];
  };

```
addr_type 鍐冲畾浜嗗湴鍧€绌剁珶鏄粈涔堛€傞┍鍔ㄧ洰鍓嶇悊瑙ｄ袱绉嶄笉鍚岀被鍨嬬殑鍦板潃銆?
```

  struct ipmi_system_interface_addr
  {
	int   addr_type;
	short channel;
  };

```
绫诲瀷鏄?IPMI_SYSTEM_INTERFACE_ADDR_TYPE銆傝繖鐢ㄤ簬鐩存帴涓庡綋鍓嶅崱涓婄殑 BMC 閫氫俊銆俢hannel
蹇呴』鏄?IPMI_BMC_CHANNEL銆?
鍙戝線 IPMB 鎬荤嚎銆佺粡鐢?```

  struct ipmi_ipmb_addr
  {
	int           addr_type;
	short         channel;
	unsigned char slave_addr;
	unsigned char lun;
  };

```
鐨?address 鐨勬秷鎭€?channel" 杩欓噷閫氬父涓洪浂锛屼絾鏈変簺璁惧鏀寔澶氫簬涓€涓€氶亾锛屽畠瀵瑰簲
IPMI 瑙勮寖涓畾涔夌殑閫氶亾銆?
杩樻湁涓€绉?IPMB 鐩磋繛鍦板潃锛岀敤浜庡彂閫佽€呯洿鎺ヤ綅浜?IPMB 鎬荤嚎涓娿€佹棤闇€缁忚繃 BMC 鐨勬儏鍐点€?浣犲彲浠ュ悜
```

  struct ipmi_ipmb_direct_addr
  {
	int           addr_type;
	short         channel;
	unsigned char slave_addr;
	unsigned char rq_lun;
	unsigned char rs_lun;
  };

```
涓婄殑鐗瑰畾绠＄悊鎺у埗鍣紙MC锛夊彂閫佹秷鎭€俢hannel 濮嬬粓涓洪浂銆備綘涔熷彲浠ユ帴鏀舵潵鑷綘宸叉敞鍐?澶勭悊骞跺搷搴旂殑鍏朵粬 MC 鐨勫懡浠わ紝鍥犳浣犲彲浠ョ敤瀹冩潵瀹炵幇鎬荤嚎涓婄殑涓€涓鐞嗘帶鍒跺櫒銆?
### 娑堟伅

```

  struct ipmi_msg
  {
	unsigned char netfn;
	unsigned char lun;
	unsigned char cmd;
	unsigned char *data;
	int           data_len;
  };

```
椹卞姩璐熻矗娣诲姞/鍓ョ澶撮儴淇℃伅銆俤ata 閮ㄥ垎鍙槸瑕佸彂閫佺殑鏁版嵁锛堜笉瑕佹妸瀵诲潃淇℃伅鏀惧湪杩欓噷锛?鎴栧搷搴斻€傛敞鎰忥紝鍝嶅簲鐨勫畬鎴愮爜锛坈ompletion code锛夋槸 "data" 涓殑绗竴椤癸紝瀹冩病鏈夎鍓ョ
鍑烘潵锛屽洜涓鸿繖灏辨槸瑙勮寖涓墍鏈夋秷鎭殑瀹氫箟鏂瑰紡锛堣繖涔熻鍋忕Щ璁℃暟绋嶅井瀹规槗涓€浜?:-锛夈€?
浠庣敤鎴锋€佷娇鐢?IOCTL 鎺ュ彛鏃讹紝鍗充娇鏄湪鎺ユ敹娑堟伅鏃讹紝浣犱篃蹇呴』涓?"data" 鎻愪緵涓€鍧楁暟鎹€?濉厖瀹冿紝骞跺皢 data_len 璁句负璇ユ暟鎹潡鐨勯暱搴︺€傚惁鍒欓┍鍔ㄦ棤澶勬斁缃秷鎭€?
浠庡唴鏍告€佺殑娑堟伅澶勭悊绋嬪簭涓婃潵鐨勬秷鎭細浠?```

  struct ipmi_recv_msg
  {
	struct list_head link;

	/* The type of message as defined in the "Receive Types"
           defines above. */
	int         recv_type;

	ipmi_user_t      *user;
	struct ipmi_addr addr;
	long             msgid;
	struct ipmi_msg  msg;

	/* Call this when done with the message.  It will presumably free
	   the message and do any other necessary cleanup. */
	void (*done)(struct ipmi_recv_msg *msg);

	/* Place-holder for the data, don't make any assumptions about
	   the size or existence of this, since it may change. */
	unsigned char   msg_data[IPMI_MAX_MSG_LENGTH];
  };

```
鐨勫舰寮忓埌鏉ャ€備綘搴旇鏌ョ湅鎺ユ敹绫诲瀷骞堕€傚綋鍦板鐞嗘秷鎭€?

### 涓婂眰鎺ュ彛锛堟秷鎭鐞嗙▼搴忥級

涓婂眰鎺ュ彛涓虹敤鎴锋彁渚涘 IPMI 鎺ュ彛鐨勪竴鑷磋鍥俱€傚畠鍏佽澶氫釜 SMI 鎺ュ彛琚鍧€锛堝洜涓烘煇浜?鏉垮瓙涓婂疄闄呬笂鏈夊涓?BMC 鍦ㄥ畠浠箣涓婏級锛岃€岀敤鎴锋棤闇€鍏冲績瀹冧滑涓嬮潰鏄粈涔堢被鍨嬬殑 SMI銆?

##### 鐩戣鎺ュ彛

褰撲綘鐨勪唬鐮佸惎鍔ㄦ椂锛孖PMI 椹卞姩鍙兘宸叉娴嬪埌涔熷彲鑳藉皻鏈娴嬪埌 IPMI 璁惧鏄惁瀛樺湪銆傚洜姝?浣犲彲鑳介渶瑕佹帹杩熶綘鐨勮缃紝鐩村埌璁惧琚娴嬪埌锛屾垨鑰呬綘鍙兘鑳藉绔嬪嵆杩涜銆備负浜嗗鐞嗚繖绉?鎯呭喌锛屽苟鏀寔鍙戠幇锛屼綘鍙互鐢?ipmi_smi_watcher_register() 娉ㄥ唽涓€涓?SMI 鐩戣鍣紙watcher锛夛紝
浠ラ亶鍘嗘帴鍙ｅ苟鍦ㄥ畠浠嚭鐜板拰娑堝け鏃堕€氱煡浣犮€?

##### 鍒涘缓鐢ㄦ埛

瑕佷娇鐢ㄦ秷鎭鐞嗙▼搴忥紝浣犲繀椤诲厛鐢?ipmi_create_user 鍒涘缓涓€涓敤鎴枫€傛帴鍙ｅ彿鎸囧畾浣犳兂
杩炴帴鍒扮殑 SMI锛屽苟涓斾綘蹇呴』鎻愪緵鍦ㄦ暟鎹埌鏉ユ椂琚皟鐢ㄧ殑鍥炶皟鍑芥暟銆傝繖涔熷厑璁镐綘浼犲叆涓€鍧?鏁版嵁 handler_data锛屽畠浼氬湪鎵€鏈夎皟鐢ㄤ腑鍥炰紶缁欎綘銆?
涓€鏃﹀畬鎴愶紝璋冪敤 ipmi_destroy_user() 鏉ョЩ闄よ鐢ㄦ埛銆?
鍦ㄧ敤鎴锋€侊紝鎵撳紑璁惧浼氳嚜鍔ㄥ垱寤轰竴涓敤鎴凤紝鍏抽棴璁惧浼氳嚜鍔ㄩ攢姣佽鐢ㄦ埛銆?

##### 娑堟伅浼犻€?
瑕佷粠鍐呮牳鎬佸彂閫佹秷鎭紝ipmi_request_settime() 璋冪敤鍑犱箮瀹屾垚浜嗘墍鏈夋秷鎭鐞嗐€傚ぇ澶氭暟
鍙傛暟涓嶈█鑷槑銆備絾瀹冩帴鍙椾竴涓?"msgid" 鍙傛暟銆傝繖 **涓嶆槸** 娑堟伅鐨勫簭鍒楀彿銆傚畠鍙槸涓€涓?闀挎暣鍨嬪€硷紝鍦ㄦ秷鎭殑鍝嶅簲杩斿洖鏃惰鍥炰紶銆備綘鍙互闅忔剰浣跨敤瀹冦€?
鍝嶅簲浼氬湪浣犱紶缁?ipmi_create_user() 鐨?"handler" 鐨?ipmi_recv_hndl 瀛楁鎵€鎸囧悜鐨?鍑芥暟涓繑鍥炪€備篃璁板緱鏌ョ湅鎺ユ敹绫诲瀷銆?
鍦ㄧ敤鎴锋€侊紝浣犲～鍏呬竴涓?ipmi_req_t 缁撴瀯骞朵娇鐢?IPMICTL_SEND_COMMAND ioctl銆傚浜庝紶鍏?鐨勫唴瀹癸紝浣犲彲浠ヤ娇鐢?select() 鎴?poll() 绛夊緟娑堟伅鍒版潵銆備絾鏄紝浣犱笉鑳戒娇鐢?read() 鑾峰彇
瀹冧滑锛屼綘蹇呴』璋冪敤甯?ipmi_recv_t 缁撴瀯鐨?IPMICTL_RECEIVE_MSG 鏉ョ湡姝ｈ幏鍙栨秷鎭€傝浣忎綘
蹇呴』鍦?msg.data 瀛楁鎻愪緵涓€涓寚鍚戞暟鎹潡鐨勬寚閽堬紝骞朵笖蹇呴』鍦?msg.data_len 瀛楁濉叆
鏁版嵁鐨勫ぇ灏忋€傝繖缁欐帴鏀惰€呬竴涓疄闄呮斁缃秷鎭殑鍦版柟銆?
濡傛灉娑堟伅鏃犳硶鏀惧叆浣犳彁渚涚殑鏁版嵁涓紝浣犲皢寰楀埌涓€涓?EMSGSIZE 閿欒锛屽苟涓旈┍鍔ㄤ細鎶婃暟鎹暀鍦?鎺ユ敹闃熷垪涓€傚鏋滀綘鎯宠幏鍙栧畠骞惰娑堟伅琚埅鏂紝璇蜂娇鐢?IPMICTL_RECEIVE_MSG_TRUNC ioctl銆?
褰撲綘鍦?IPMB 鎬荤嚎涓婂彂閫佷竴鏉″懡浠わ紙鎸?IPMI 瑙勮寖鐢?netfn 鐨勬渶浣庝綅瀹氫箟锛夋椂锛岄┍鍔ㄤ細鑷姩
涓哄懡浠ゅ垎閰嶅簭鍒楀彿骞朵繚瀛樿鍛戒护銆傚鏋滃湪 IPMI 瑙勫畾鐨?5 绉掑唴娌℃湁鏀跺埌鍝嶅簲锛屽畠浼氳嚜鍔ㄧ敓鎴?涓€涓搷搴旓紝琛ㄧず鍛戒护瓒呮椂銆傚鏋滄敹鍒颁竴涓湭缁忚姹傦紙unsolicited锛夌殑鍝嶅簲锛堜緥濡傚畠鍦?5 绉掑悗
鎵嶅埌锛夛紝璇ュ搷搴斿皢琚拷鐣ャ€?
鍦ㄥ唴鏍告€侊紝鏀跺埌涓€鏉℃秷鎭苟澶勭悊濂戒箣鍚庯紝浣犲繀椤诲瀹冭皟鐢?ipmi_free_recv_msg()锛屽惁鍒欎細
娉勬紡娑堟伅銆傛敞鎰忎綘缁濅笉搴旇鍔ㄦ秷鎭殑 "done" 瀛楁锛岄偅鏄纭竻鐞嗘秷鎭墍蹇呴渶鐨勩€?
娉ㄦ剰锛屽彂閫佹椂鏈変竴涓?ipmi_request_supply_msgs() 璋冪敤锛屽厑璁镐綘鎻愪緵 smi 鍜屾帴鏀舵秷鎭€傝繖瀵?鍗充娇绯荤粺缂撳啿鍖鸿€楀敖涔熼渶瑕佸伐浣滅殑浠ｇ爜寰堟湁鐢紙渚嬪鐪嬮棬鐙楀畾鏃跺櫒灏辩敤浜嗚繖涓級銆備綘鎻愪緵鑷繁鐨?缂撳啿鍖哄拰鑷繁鐨勯噴鏀句緥绋嬨€備笉杩囦笉寤鸿姝ｅ父浣跨敤锛屽洜涓虹鐞嗚嚜宸辩殑缂撳啿鍖哄緢妫樻墜銆?

##### 浜嬩欢涓庝紶鍏ュ懡浠?
椹卞姩璐熻矗杞 IPMI 浜嬩欢鍜屾帴鏀跺懡浠わ紙鍛戒护鏄苟闈炲搷搴旂殑娑堟伅锛屽畠浠槸 IPMB 鎬荤嚎涓婂叾浠栦笢瑗?鍙戠粰浣犵殑鍛戒护锛夈€傝鎺ユ敹杩欎簺锛屼綘蹇呴』涓哄畠浠敞鍐岋紝瀹冧滑涓嶄細琚嚜鍔ㄥ彂閫佺粰浣犮€?
瑕佹帴鏀朵簨浠讹紝浣犲繀椤昏皟鐢?ipmi_set_gets_events() 骞跺皢 "val" 璁句负闈為浂銆傝嚜鍚姩浠ユ潵椹卞姩宸?鏀跺埌鐨勪换浣曚簨浠堕兘浼氱珛鍗虫姇閫掔粰绗竴涓负浜嬩欢娉ㄥ唽鐨勭敤鎴枫€備箣鍚庯紝濡傛灉澶氫釜鐢ㄦ埛娉ㄥ唽浜嗕簨浠讹紝
瀹冧滑閮戒細鏀跺埌鎵€鏈夎繘鏉ョ殑浜嬩欢銆?
瀵逛簬鎺ユ敹鍛戒护锛屼綘蹇呴』涓轰綘鎯虫帴鏀剁殑鍛戒护閫愪釜娉ㄥ唽銆傝皟鐢?ipmi_register_for_cmd() 骞朵负姣忎釜
浣犳兂鎺ユ敹鐨勫懡浠ゆ彁渚?netfn 鍜屽懡浠ゅ悕銆備綘杩樻寚瀹氫竴涓綘甯屾湜浠庝腑鎺ユ敹鍛戒护鐨勯€氶亾浣嶆帺鐮侊紙濡傛灉
涓嶅湪涔庯紝鍙互浣跨敤 IPMI_CHAN_ALL 琛ㄧず鎵€鏈夐€氶亾锛夈€傚浜庢瘡涓?netfn/cmd/channel 鍙兘娉ㄥ唽涓€涓?鐢ㄦ埛锛屼絾涓嶅悓鐨勭敤鎴峰彲浠ユ敞鍐屼笉鍚岀殑鍛戒护锛屾垨鑰呭鏋滈€氶亾浣嶆帺鐮佷笉閲嶅彔锛屽垯鍙互娉ㄥ唽鐩稿悓鐨勫懡浠ゃ€?
瑕佸搷搴旀敹鍒扮殑鍛戒护锛屽湪杩斿洖鐨?netfn 涓缃搷搴斾綅锛屼娇鐢ㄦ敹鍒版秷鎭殑鍦板潃锛屽苟浣跨敤浣犲湪鏀跺埌
娑堟伅涓緱鍒扮殑鐩稿悓 msgid銆?
鍦ㄧ敤鎴锋€侊紝鎻愪緵浜嗙瓑浠风殑 IOCTL 鏉ユ墽琛岃繖浜涘姛鑳姐€?

### 涓嬪眰锛圫MI锛夋帴鍙?
濡傚墠鎵€杩帮紝澶氫釜 SMI 鎺ュ彛鍙互娉ㄥ唽鍒版秷鎭鐞嗙▼搴忥紝瀹冧滑鍚勮嚜鍦ㄦ敞鍐屾椂鑾峰緱涓€涓帴鍙ｅ彿銆?瀹冧滑閫氬父鎸夋敞鍐岄『搴忓垎閰嶏紝涓嶈繃濡傛灉涓€涓?SMI 娉ㄩ攢鍚庡彟涓€涓啀娉ㄥ唽锛岄偅灏变竴鍒囬毦璇翠簡銆?
ipmi_smi.h 瀹氫箟浜嗙鐞嗘帴鍙ｇ殑鎺ュ彛锛岃瑙佽鏂囦欢銆?

### SI 椹卞姩

SI 椹卞姩鍏佽鍦ㄧ郴缁熶腑閰嶇疆 KCS銆丅T 鍜?SMIC 鎺ュ彛銆傚畠鏍规嵁绯荤粺锛岄€氳繃澶氱涓嶅悓鐨勬柟娉曞彂鐜?鎺ュ彛銆?
浣犲彲浠ュ湪妯″潡鍔犺浇琛屼笂鏈€澶氭寚瀹氬洓涓帴鍙ｏ紝浠ュ強
```

  modprobe ipmi_si.o type=<type1>,<type2>....
       ports=<port1>,<port2>... addrs=<addr1>,<addr2>...
       irqs=<irq1>,<irq2>...
       regspacings=<sp1>,<sp2>,... regsizes=<size1>,<size2>,...
       regshifts=<shift1>,<shift2>,...
       slave_addrs=<addr1>,<addr2>,...
       force_kipmid=<enable1>,<enable2>,...
       kipmid_max_busy_us=<ustime1>,<ustime2>,...
       unload_when_empty=[0|1]
       trydmi=[0|1] tryacpi=[0|1]
       tryplatform=[0|1] trypci=[0|1]

```
闄や簡 try... 椤逛互澶栵紝杩欎簺姣忎竴椤归兘鏄竴涓垪琛紝绗竴椤瑰搴旂涓€涓帴鍙ｏ紝绗簩椤瑰搴旂浜屼釜
鎺ュ彛锛屼互姝ょ被鎺ㄣ€?
si_type 鍙互鏄?"kcs"銆?smic" 鎴?"bt"銆傚鏋滅暀绌猴紝榛樿涓?"kcs"銆?
濡傛灉浣犱负鏌愪釜鎺ュ彛鎸囧畾浜嗛潪闆剁殑 addrs锛岄┍鍔ㄥ皢鎶婄粰瀹氱殑鍐呭瓨鍦板潃鐢ㄤ綔璁惧鍦板潃銆傝繖浼氳鐩?si_ports銆?
濡傛灉浣犱负鏌愪釜鎺ュ彛鎸囧畾浜嗛潪闆剁殑 ports锛岄┍鍔ㄥ皢鎶婄粰瀹氱殑 I/O 绔彛鐢ㄤ綔璁惧鍦板潃銆?
濡傛灉浣犱负鏌愪釜鎺ュ彛鎸囧畾浜嗛潪闆剁殑 irqs锛岄┍鍔ㄥ皢灏濊瘯鎶婄粰瀹氱殑涓柇鐢ㄤ簬璇ヨ澶囥€?
鍏朵粬 try... 椤归€氳繃鍏跺搴斿悕绉扮鐢ㄥ彂鐜般€傝繖浜涢粯璁ゅ叏閮ㄥ惎鐢紝璁句负 0 浠ョ鐢ㄥ畠浠€倀ryplatform
绂佺敤 openfirmware銆?
鎺ヤ笅鏉ョ殑涓変釜鍙傛暟涓庡瘎瀛樺櫒甯冨眬鏈夊叧銆傛帴鍙ｄ娇鐢ㄧ殑瀵勫瓨鍣ㄥ彲鑳戒笉鍑虹幇鍦ㄨ繛缁殑浣嶇疆锛屼篃鍙兘
涓嶅湪 8 浣嶅瘎瀛樺櫒涓€傝繖浜涘弬鏁板厑璁告洿绮剧‘鍦版寚瀹氬瘎瀛樺櫒涓暟鎹殑甯冨眬銆?
regspacings 鍙傛暟缁欏嚭杩炵画瀵勫瓨鍣ㄨ捣濮嬪湴鍧€涔嬮棿鐨勫瓧鑺傛暟銆備緥濡傦紝濡傛灉 regspacing 璁句负 4锛?璧峰鍦板潃涓?0xca2锛岄偅涔堢浜屼釜瀵勫瓨鍣ㄧ殑鍦板潃灏嗘槸 0xca6銆傞粯璁や负 1銆?
regsizes 鍙傛暟缁欏嚭瀵勫瓨鍣ㄧ殑澶у皬锛堝瓧鑺傦級銆侷PMI 浣跨敤鐨勬暟鎹槸 8 浣嶅锛屼絾瀹冨彲鑳藉湪鏇村ぇ鐨?瀵勫瓨鍣ㄥ唴閮ㄣ€傛鍙傛暟鍏佽鎸囧畾璇诲啓绫诲瀷銆傚畠鍙互鏄?1銆?銆? 鎴?8銆傞粯璁や负 1銆?
鐢变簬瀵勫瓨鍣ㄥぇ灏忓彲鑳藉ぇ浜?32 浣嶏紝IPMI 鏁版嵁鍙兘涓嶅湪浣?8 浣嶃€俽egshifts 鍙傛暟缁欏嚭涓轰簡寰楀埌
瀹為檯 IPMI 鏁版嵁鎵€闇€鐨勭Щ浣嶉噺銆?
slave_addrs 鎸囧畾鏈湴 BMC 鐨?IPMI 鍦板潃銆傚畠閫氬父鏄?0x20锛岄┍鍔ㄩ粯璁ゅ姝わ紝浣嗚嫢涓嶆槸锛屽彲浠ュ湪
椹卞姩鍚姩鏃舵寚瀹氥€?
force_ipmid 鍙傛暟寮哄埗鍚敤锛堣涓?1锛夋垨绂佺敤锛堣涓?0锛夊唴鏍?IPMI 瀹堟姢杩涚▼銆傞€氬父杩欑敱椹卞姩鑷姩
鎺㈡祴锛屼絾涓柇鎹熷潖鐨勭郴缁熷彲鑳介渶瑕佸惎鐢紝鎴栬€呬笉鎯崇敤瀹堟姢杩涚▼锛堜笉闇€瑕佹€ц兘銆佷笉鎯冲崰鐢?CPU锛夌殑
鐢ㄦ埛鍙互绂佺敤瀹冦€?
濡傛灉 unload_when_empty 璁句负 1锛屽綋椹卞姩鎵句笉鍒颁换浣曟帴鍙ｆ垨鎵€鏈夋帴鍙ｉ兘澶辨晥鏃讹紝椹卞姩灏嗚鍗歌浇銆?榛樿涓?1銆傝涓?0 閰嶅悎 hotmod 鏃舵湁鐢紝浣嗘樉鐒跺彧瀵规ā鍧楁湁鎰忎箟銆?
褰撶紪璇戣繘鍐呮牳鏃讹紝鍙傛暟鍙互鍦?```

  ipmi_si.type=<type1>,<type2>...
       ipmi_si.ports=<port1>,<port2>... ipmi_si.addrs=<addr1>,<addr2>...
       ipmi_si.irqs=<irq1>,<irq2>...
       ipmi_si.regspacings=<sp1>,<sp2>,...
       ipmi_si.regsizes=<size1>,<size2>,...
       ipmi_si.regshifts=<shift1>,<shift2>,...
       ipmi_si.slave_addrs=<addr1>,<addr2>,...
       ipmi_si.force_kipmid=<enable1>,<enable2>,...
       ipmi_si.kipmid_max_busy_us=<ustime1>,<ustime2>,...

```
涓婃寚瀹氥€傚畠涓庡悓鍚嶇殑妯″潡鍙傛暟宸ヤ綔鏂瑰紡鐩稿悓銆?
濡傛灉浣犵殑 IPMI 鎺ュ彛涓嶆敮鎸佷腑鏂紝骞朵笖鏄?KCS 鎴?SMIC 鎺ュ彛锛孖PMI 椹卞姩浼氫负璇ユ帴鍙ｅ惎鍔ㄤ竴涓?鍐呮牳绾跨▼浠ュ姞蹇€熷害銆傝繖鏄竴涓綆浼樺厛绾у唴鏍哥嚎绋嬶紝鍦?IPMI 鎿嶄綔杩涜鏈熼棿涓嶆柇杞 IPMI 椹卞姩銆?force_kipmid 妯″潡鍙傛暟鍏佽鐢ㄦ埛寮哄埗寮€鍚垨鍏抽棴姝ょ嚎绋嬨€傚鏋滀綘寮哄埗鍏抽棴瀹冧笖娌℃湁涓柇锛岄┍鍔?灏嗚繍琛屽緱闈炲父鎱€傚埆鎬垜锛岃繖浜涙帴鍙ｅお鐑備簡銆?
閬楁喚鐨勬槸锛岃繖涓嚎绋嬪彲鑳戒細鍗犵敤澶ч噺 CPU锛屽彇鍐充簬鎺ュ彛鐨勬€ц兘銆傝繖浼氭氮璐瑰緢澶?CPU 骞跺紩鍙戝悇绉?妫€娴嬬┖闂?CPU 鍜屼娇鐢ㄩ澶栧姛鑰楃殑闂銆備负閬垮厤姝ら棶棰橈紝kipmid_max_busy_us 璁剧疆 kipmid 鍦?浼戠湢涓€涓?tick 涔嬪墠鑷棆鐨勬渶闀挎椂闂达紙寰锛夈€傝繖涓€煎湪鎬ц兘鍜?CPU 娴垂涔嬮棿璁惧畾浜嗕竴涓钩琛★紝
闇€瑕佹牴鎹綘鐨勯渶姹傝皟鏁淬€備篃璁告湁涓€澶╀細鍔犲叆鑷姩璋冧紭锛屼絾杩欎笉鏄欢绠€鍗曠殑浜嬶紝鍗充究鑷姩璋冧紭涔?闇€瑕佹牴鎹敤鎴锋湡鏈涚殑鎬ц兘鏉ヨ皟鏁淬€?
椹卞姩鏀寔鎺ュ彛鐨勭儹娣诲姞鍜岀Щ闄ゃ€傝繖鏍凤紝鍙互鍦ㄥ唴鏍稿惎鍔ㄥ苟杩愯涔嬪悗娣诲姞鎴栫Щ闄ゆ帴鍙ｃ€傝繖鏄€氳繃
/sys/modules/ipmi_si/parameters/hotmod 瀹屾垚鐨勶紝瀹冩槸涓€涓彧鍐欏弬鏁般€備綘鍚戣鎺ュ彛鍐欏叆涓€涓?瀛楃涓层€傝瀛楃涓?```

   <op1>[:op2[:op3...]]

```
```

   add|remove,kcs|bt|smic,mem|i/o,<address>[,<opt1>[,<opt2>[,...]]]

```
```

   rsp=<regspacing>
   rsi=<regsize>
   rsh=<regshift>
   irq=<irq>
   ipmb=<ipmb slave addr>

```
瀹冧滑鐨勫惈涔変笌涓婃枃璁ㄨ鐨勭浉鍚屻€傛敞鎰忎綘涔熷彲浠ュ湪鍐呮牳鍛戒护琛屼笂浣跨敤瀹冿紝浠ヨ幏寰楁洿绱у噾鐨勬寚瀹?鎺ュ彛鐨勬牸寮忋€傛敞鎰忥紝褰撶Щ闄や竴涓帴鍙ｆ椂锛屽彧鏈夊墠涓変釜鍙傛暟锛坰i 绫诲瀷銆佸湴鍧€绫诲瀷鍜屽湴鍧€锛夌敤浜?姣旇緝銆備换浣曢€夐」鍦ㄧЩ闄ゆ椂閮戒細琚拷鐣ャ€?
### SMBus 椹卞姩锛圫SIF锛?
SMBus 椹卞姩鍏佽鍦ㄧ郴缁熶腑閰嶇疆鏈€澶?4 涓?SMBus 璁惧銆傞粯璁ゆ儏鍐典笅锛岄┍鍔ㄥ彧浼氬湪瀹冨湪 DMI 鎴?ACPI 琛ㄤ腑鍙戠幇鐨勪笢瑗夸笂娉ㄥ唽銆備綘鍙互杩欐牱鏇存敼
```

  modprobe ipmi_ssif.o
	addr=<i2caddr1>[,<i2caddr2>[,...]]
	adapter=<adapter1>[,<adapter2>[...]]
	dbg=<flags1>,<flags2>...
	slave_addrs=<addr1>,<addr2>,...
	tryacpi=[0|1] trydmi=[0|1]
	[dbg_probe=1]
	alerts_broken

```
杩欎簺鍦板潃鏄櫘閫氱殑 I2C 鍦板潃銆俛dapter 鏄€傞厤鍣ㄧ殑瀛楃涓插悕绉帮紝濡?/sys/bus/i2c/devices/i2c-<n>/name 鎵€绀恒€傚畠 **涓嶆槸** i2c-<n> 鏈韩銆傛澶栵紝姣旇緝鏃跺拷鐣?绌烘牸锛屾墍浠ュ鏋滃悕绉版槸 "This is an I2C chip"锛屼綘鍙互璇?adapter_name=ThisisanI2cchip銆?杩欐槸鍥犱负鍦ㄥ唴鏍稿弬鏁颁腑寰堥毦浼犲叆绌烘牸銆?
璋冭瘯鏍囧織鏄搴旀瘡涓彂鐜扮殑 BMC 鐨勪綅鏍囧織锛屽畠浠槸锛?IPMI 娑堟伅锛?锛岄┍鍔ㄧ姸鎬侊細2锛屾椂搴忥細4锛孖2C 鎺㈡祴锛?

tryxxx 鍙傛暟鍙敤浜庣鐢ㄤ粠鍚勭鏉ユ簮妫€娴嬫帴鍙ｃ€?
灏?dbg_probe 璁句负 1 浼氬惎鐢ㄥ SMBus 涓?BMC 鎺㈡祴鍜屾娴嬭繃绋嬬殑璋冭瘯銆?
slave_addrs 鎸囧畾鏈湴 BMC 鐨?IPMI 鍦板潃銆傚畠閫氬父鏄?0x20锛岄┍鍔ㄩ粯璁ゅ姝わ紝浣嗚嫢涓嶆槸锛屽彲浠?鍦ㄩ┍鍔ㄥ惎鍔ㄦ椂鎸囧畾銆?
alerts_broken 涓嶄负 SSIF 鍚敤 SMBus alert銆傚惁鍒?SMBus alert 浼氬湪鍙楁敮鎸佺殑纭欢涓婅鍚敤銆?
鍦?SMBus 涓婂彂鐜扮鍚?IPMI 鐨?BMC 鍙兘瀵艰嚧 I2C 鎬荤嚎涓婄殑璁惧澶辫触銆係MBus 椹卞姩鍚?I2C 鎬荤嚎
浠ュ潡鍐欐柟寮忓啓鍏ヤ竴鏉?"Get Device ID" IPMI 娑堟伅骞剁瓑寰呭搷搴斻€傛鍔ㄤ綔瀵规煇浜?I2C 璁惧鏄湁瀹崇殑銆?寮虹儓寤鸿灏嗗凡鐭?I2C 鍦板潃閫氳繃 smb_addr 鍙傛暟鎻愪緵缁?SMBus 椹卞姩锛岄櫎闈炰綘鏈?DMI 鎴?ACPI 鏁版嵁
鍛婅瘔椹卞姩璇ョ敤浠€涔堛€?
褰撶紪璇戣繘鍐呮牳鏃讹紝鍦板潃鍙互鍦?```

  ipmb_ssif.addr=<i2caddr1>[,<i2caddr2>[...]]
	ipmi_ssif.adapter=<adapter1>[,<adapter2>[...]]
	ipmi_ssif.dbg=<flags1>[,<flags2>[...]]
	ipmi_ssif.dbg_probe=1
	ipmi_ssif.slave_addrs=<addr1>[,<addr2>[...]]
	ipmi_ssif.tryacpi=[0|1] ipmi_ssif.trydmi=[0|1]

```
涓婃寚瀹氥€傝繖浜涢€夐」涓庢ā鍧楀懡浠よ涓婄殑鐩稿悓銆?
I2C 椹卞姩涓嶆敮鎸侀潪闃诲璁块棶鎴栬疆璇紝鍥犳濡傛灉娌℃湁鐗规畩鐨勫唴鏍歌ˉ涓佸拰椹卞姩淇敼锛屾椹卞姩鏃犳硶鍋?IPMI panic 浜嬩欢銆佸湪 panic 鏃跺欢闀跨湅闂ㄧ嫍鎴栧叾浠栦笌 panic 鐩稿叧鐨?IPMI 鍔熻兘銆備綘鍙互鍦?openipmi
缃戦〉涓婅幏鍙栧畠浠€?
椹卞姩閫氳繃 I2C sysfs 鎺ュ彛鏀寔鎺ュ彛鐨勭儹娣诲姞鍜岀Щ闄ゃ€?
### IPMI IPMB 椹卞姩

姝ら┍鍔ㄧ敤浜庢敮鎸佷綅浜?IPMB 鎬荤嚎涓婄殑绯荤粺锛涘畠璁╄鎺ュ彛鐪嬭捣鏉ュ儚涓€涓櫘閫氱殑 IPMI 鎺ュ彛銆傚悜瀹?鍙戦€佺郴缁熸帴鍙ｅ鍧€鐨勬秷鎭細瀵艰嚧娑堟伅鍙戝線绯荤粺涓婂凡娉ㄥ唽鐨?BMC锛堥粯璁ゅ湪 IPMI 鍦板潃 0x20锛夈€?
瀹冭繕鍏佽浣犱娇鐢?ipmb 鐩磋繛瀵诲潃鐩存帴瀵诲潃鎬荤嚎涓婄殑鍏朵粬 MC銆備綘鍙互鎺ユ敹鏉ヨ嚜鎬荤嚎涓婂叾浠?MC 鐨?鍛戒护锛屽畠浠細閫氳繃涓婃枃鎻忚堪鐨勬櫘閫氭帴鏀跺懡浠ゆ満鍒跺鐞嗐€?
```

  ipmi_ipmb.bmcaddr=<address to use for system interface addresses messages>
	ipmi_ipmb.retry_time_ms=<Time between retries on IPMB>
	ipmi_ipmb.max_retries=<Number of times to retry a message>

```
鍔犺浇妯″潡涓嶄細浣块┍鍔ㄨ嚜鍔ㄥ惎鍔紝闄ら潪鏈夎澶囨爲淇℃伅鏉ヨ缃畠銆傚鏋?```

  echo ipmi-ipmb <addr> > /sys/class/i2c-dev/i2c-<n>/device/new_device

```
娉ㄦ剰浣犲湪杩欓噷缁欏嚭鐨勫湴鍧€鏄?I2C 鍦板潃锛屼笉鏄?IPMI 鍦板潃銆傛墍浠ュ鏋滀綘甯屾湜浣犵殑 MC 鍦板潃鏄?0x60锛?浣犲湪杩欓噷鏀?0x30銆傝瑙?I2C 椹卞姩淇℃伅銆?
閫氳繃姝ゆ帴鍙ｅ悜鍏朵粬 IPMB 鎬荤嚎妗ユ帴鍛戒护涓嶈捣浣滅敤銆傛帴鏀舵秷鎭槦鍒楁寜璁捐鏈疄鐜般€侭MC 涓婂彧鏈変竴涓?鎺ユ敹娑堟伅闃熷垪锛岄偅鏄粰涓绘満椹卞姩鐢ㄧ殑锛岃€屼笉鏄粰 IPMB 鎬荤嚎涓婄殑涓滆タ鐢ㄧ殑銆?
BMC 鍙兘鏈夊涓?IPMB 鎬荤嚎锛屼綘鐨勮澶囦綅浜庡摢鏉℃€荤嚎鍙栧喅浜庣郴缁熺殑鎺ョ嚎鏂瑰紡銆備綘鍙互鐢?"ipmitool channel info <n>" 鑾峰彇閫氶亾锛屽叾涓?<n> 鏄€氶亾锛岄€氶亾涓?0-7锛岃瘯璇?IPMB 閫氶亾銆?
### 鍏朵粬閮ㄥ垎

### 鑾峰彇涓?IPMI 璁惧鐩稿叧鐨勮缁嗕俊鎭?
鏈変簺鐢ㄦ埛闇€瑕佸叧浜庤澶囩殑鏇磋缁嗕俊鎭紝姣斿鍦板潃浠庝綍鑰屾潵锛屾垨 IPMI 鎺ュ彛鐨勫師濮嬪熀纭€璁惧銆?浣犲彲浠ヤ娇鐢?IPMI smi_watcher 鍦?IPMI 鎺ュ彛鍑虹幇鎴栨秷澶辨椂鎹曟崏瀹冧滑锛屽苟涓轰簡鑾峰彇淇℃伅锛屼綘鍙互
浣跨敤鍑芥暟
```

  struct ipmi_smi_info {
	enum ipmi_addr_src addr_src;
	struct device *dev;
	union {
		struct {
			void *acpi_handle;
		} acpi_info;
	} addr_info;
  };

```
鐩墠浠呰繑鍥?SI_ACPI 鍦板潃婧愮殑鐗规畩淇℃伅銆傚繀瑕佹椂鍙兘浼氭坊鍔犲叾浠栦俊鎭€?
娉ㄦ剰涓婅堪缁撴瀯涓寘鍚簡 dev 鎸囬拡锛屽亣璁?ipmi_smi_get_info 杩斿洖鎴愬姛锛屼綘蹇呴』瀵?dev 鎸囬拡璋冪敤
put_device銆?
### 鐪嬮棬鐙?
鎻愪緵浜嗕竴涓疄鐜颁簡 Linux 鏍囧噯鐪嬮棬鐙楀畾鏃跺櫒鎺ュ彛鐨勭湅闂ㄧ嫍瀹氭椂鍣ㄣ€傚畠鏈変笁涓ā鍧楀弬鏁板彲浠?```

  modprobe ipmi_watchdog timeout=<t> pretimeout=<t> action=<action type>
      preaction=<preaction type> preop=<preop type> start_now=x
      nowayout=x ifnum_to_use=n panic_wdt_timeout=<t>

```
ifnum_to_use 鎸囧畾鐪嬮棬鐙楀畾鏃跺櫒搴斾娇鐢ㄥ摢涓帴鍙ｃ€傞粯璁や负 -1锛岃〃绀洪€夊彇绗竴涓敞鍐岀殑鎺ュ彛銆?
timeout 鏄埌鍔ㄤ綔鍙戠敓鐨勭鏁帮紝pretimeout 鏄湪閲嶇疆涔嬪墠澶氬皯绉掑彂鐢熼瓒呮椂 panic锛堝鏋?pretimeout 涓洪浂锛屽垯涓嶄細鍚敤 pretimeout锛夈€傛敞鎰?pretimeout 鏄渶缁堣秴鏃朵箣鍓嶇殑鏃堕棿銆傚洜姝ゅ鏋?timeout 鏄?50 绉掋€乸retimeout 鏄?10 绉掞紝閭ｄ箞 pretimeout 灏嗗湪 40 绉掓椂鍙戠敓锛堣秴鏃跺墠 10 绉掞級銆?panic_wdt_timeout 鏄湪鍐呮牳 panic 鏃惰缃殑 timeout 鍊硷紝浠ヤ究璁╄濡?kdump 涔嬬被鐨勫姩浣滃湪 panic
鏈熼棿鍙戠敓銆?
action 鍙互鏄?"reset"銆?power_cycle" 鎴?"power_off"锛屾寚瀹氬畾鏃跺櫒瓒呮椂鏃跺仛浠€涔堬紝榛樿涓?"reset"銆?
preaction 鍙互鏄?"pre_smi"锛堥€氳繃 SMI 鎺ュ彛鎸囩ず锛夈€?pre_int"锛堥€氳繃 SMI 甯︿腑鏂寚绀猴級锛屾垨
"pre_nmi"锛坧reaction 涓婄殑 NMI锛夈€傝繖灏辨槸椹卞姩琚憡鐭?pretimeout 鐨勬柟寮忋€?
preop 鍙互璁句负 "preop_none"锛坧retimeout 鏃朵笉鎿嶄綔锛夈€?preop_panic"锛堝皢棰勬搷浣滆涓?panic锛夛紝
鎴?"preop_give_data"锛堝湪 pretimeout 鍙戠敓鏃舵彁渚涘彲浠庣湅闂ㄧ嫍璁惧璇诲彇鐨勬暟鎹級銆?pre_nmi" 璁剧疆
**涓嶈兘** 涓?"preop_give_data" 涓€璧蜂娇鐢紝鍥犱负浣犳棤娉曚粠 NMI 鍋氭暟鎹搷浣溿€?
褰?preop 璁句负 "preop_give_data" 鏃讹紝鍦?pretimeout 鍙戠敓鏃惰澶囦細鏈変竴涓瓧鑺傚氨缁彲渚涜鍙栥€?select 鍜?fasync 鍦ㄨ澶囦笂涔熸湁鏁堛€?
濡傛灉 start_now 璁句负 1锛岀湅闂ㄧ嫍瀹氭椂鍣ㄥ皢鍦ㄩ┍鍔ㄥ姞杞藉悗绔嬪嵆寮€濮嬭繍琛屻€?
濡傛灉 nowayout 璁句负 1锛岀湅闂ㄧ嫍瀹氭椂鍣ㄥ湪鍏抽棴鐪嬮棬鐙楄澶囨椂涓嶄細鍋滄銆傚鏋滃惎鐢ㄤ簡
CONFIG_WATCHDOG_NOWAYOUT 閫夐」锛宯owayout 鐨勯粯璁ゅ€间负鐪燂紝鍚﹀垯涓哄亣銆?
褰撶紪璇戣繘鍐呮牳鏃讹紝鍐呮牳鍛戒护琛屽彲鐢?```

  ipmi_watchdog.timeout=<t> ipmi_watchdog.pretimeout=<t>
	ipmi_watchdog.action=<action type>
	ipmi_watchdog.preaction=<preaction type>
	ipmi_watchdog.preop=<preop type>
	ipmi_watchdog.start_now=x
	ipmi_watchdog.nowayout=x
	ipmi_watchdog.panic_wdt_timeout=<t>

```
閫夐」涓庢ā鍧楀弬鏁伴€夐」鐩稿悓銆?
鐪嬮棬鐙楀湪鏀跺埌棰勫姩浣滄椂浼?panic 骞跺惎鍔ㄤ竴涓?120 绉掔殑閲嶇疆瓒呮椂銆傚湪 panic 鎴栭噸鍚湡闂达紝濡傛灉鐪嬮棬鐙?姝ｅ湪杩愯锛屽畠浼氬惎鍔ㄤ竴涓?120 绉掑畾鏃跺櫒浠ョ‘淇濋噸鍚彂鐢熴€?
娉ㄦ剰锛屽鏋滀綘瀵圭湅闂ㄧ嫍浣跨敤 NMI preaction锛屼綘 **缁濅笉鑳?* 浣跨敤 nmi 鐪嬮棬鐙椼€傛病鏈夊悎鐞嗙殑鏂规硶鍒ゆ柇
NMI 鏄惁鏉ヨ嚜 IPMI 鎺у埗鍣紝鍥犳蹇呴』鍋囪濡傛灉瀹冩敹鍒颁竴涓湰搴旀湭琚鐞嗙殑 NMI锛屽畠蹇呭畾鏉ヨ嚜 IPMI锛?骞朵細绔嬪嵆 panic銆?
涓€鏃︿綘鎵撳紑浜嗙湅闂ㄧ嫍瀹氭椂鍣紝浣犲繀椤诲悜璁惧鍐欏叆涓€涓?'V' 瀛楃鏉ュ叧闂畠锛屽惁鍒欏畾鏃跺櫒涓嶄細鍋滄銆傝繖瀵?椹卞姩鏄竴涓柊鐨勮涔夛紝浣嗚瀹冨拰 Linux 涓叾浠栫湅闂ㄧ嫍椹卞姩淇濇寔涓€鑷淬€?

### Panic 瓒呮椂

OpenIPMI 椹卞姩鏀寔鍦ㄥ彂鐢?panic 鏃跺皢鍗婂畾鍒跺拰鑷畾涔変簨浠舵斁鍏ョ郴缁熶簨浠舵棩蹇楃殑鑳藉姏銆傚鏋滀綘鍚敤
'Generate a panic event to all BMCs on a panic' 閫夐」锛屼綘浼氬湪 panic 鏃跺緱鍒颁竴涓爣鍑?IPMI 浜嬩欢
鏍煎紡鐨勪簨浠躲€傚鏋滀綘鍚敤 'Generate OEM events containing the panic string' 閫夐」锛屼綘杩樹細寰楀埌
涓€鎵规寔鏈?panic 瀛楃涓茬殑 OEM 浜嬩欢銆?

浜嬩欢鐨勫瓧娈佃缃涓嬶細

- Generator ID: 0x21锛堝唴鏍革級
- EvM Rev: 0x03锛堟浜嬩欢浠?IPMI 1.0 鏍煎紡鏍煎紡鍖栵級
- Sensor Type: 0x20锛圤S critical stop sensor锛?- Sensor #: panic 瀛楃涓茬殑绗竴涓瓧鑺傦紙鑻ユ棤 panic 瀛楃涓插垯涓?0锛?- Event Dir | Event Type: 0x6f锛圓ssertion锛宻ensor-specific event info锛?- Event Data 1: 0xa1锛圧untime stop in OEM bytes 2 and 3锛?- Event data 2: panic 瀛楃涓茬殑绗簩涓瓧鑺?- Event data 3: panic 瀛楃涓茬殑绗笁涓瓧鑺?
璇﹁ IPMI 瑙勮寖浜嗚В浜嬩欢甯冨眬鐨勭粏鑺傘€傛浜嬩欢鎬绘槸鍙戝線鏈湴绠＄悊鎺у埗鍣ㄣ€傚畠浼氳礋璐ｆ妸娑堟伅璺敱鍒?姝ｇ‘鐨勫湴鏂?
鍏朵粬 OEM 浜嬩欢鍏锋湁浠ヤ笅鏍煎紡锛?
- Record ID锛堝瓧鑺?0-1锛夛細鐢?SEL 璁剧疆銆?- Record type锛堝瓧鑺?2锛夛細0xf0锛圤EM non-timestamped锛?- byte 3: 淇濆瓨 panic 鐨勫崱鐨勪粠鍦板潃锛坰lave address锛?- byte 4: 涓€涓簭鍒楀彿锛堜粠闆跺紑濮嬶級
  鍏朵綑瀛楄妭锛?1 瀛楄妭锛夋槸 panic 瀛楃涓层€傚鏋?panic 瀛楃涓茶秴杩?11 瀛楄妭锛屽皢鍙戦€佸鏉℃秷鎭紝
  搴忓垪鍙烽€掑銆?
鍥犱负浣犳棤娉曚娇鐢ㄦ爣鍑嗘帴鍙ｅ彂閫?OEM 浜嬩欢锛屾鍔熻兘浼氬皾璇曟壘涓€涓?SEL 骞舵妸浜嬩欢鍔犺繘鍘汇€傚畠浼氶鍏?鏌ヨ鏈湴绠＄悊鎺у埗鍣ㄧ殑鑳藉姏銆傚鏋滃畠鏈変竴涓?SEL锛岄偅涔堝畠浠細琚瓨鍌ㄥ湪鏈湴绠＄悊鎺у埗鍣ㄧ殑 SEL 涓€?濡傛灉娌℃湁锛屼笖鏈湴绠＄悊鎺у埗鍣ㄦ槸涓€涓簨浠剁敓鎴愬櫒锛屽垯浼氭煡璇㈡湰鍦扮鐞嗘帶鍒跺櫒鐨勪簨浠舵帴鏀惰€咃紝骞跺皢
浜嬩欢鍙戝線璇ヨ澶囦笂鐨?SEL銆傚惁鍒欙紝浜嬩欢鏃犲鍙幓锛屽洜涓烘病鏈夊湴鏂瑰彂缁欏畠浠€?

### 鍏虫満

濡傛灉閫夋嫨浜嗗叧鏈鸿兘鍔涳紝IPMI 椹卞姩浼氬悜鏍囧噯鍏虫満鍑芥暟鎸囬拡瀹夎涓€涓叧鏈哄嚱鏁般€傝繖鍦?ipmi_poweroff
妯″潡涓€傚綋绯荤粺璇锋眰鏂數鏃讹紝瀹冧細鍙戦€佹纭殑 IPMI 鍛戒护鏉ュ畬鎴愩€傝繖鍦ㄥ涓钩鍙颁笂鍙楁敮鎸併€?
鏈変竴涓悕涓?"poweroff_powercycle" 鐨勬ā鍧楀弬鏁帮紝鍙互涓洪浂锛堟墽琛屾柇鐢碉級鎴栭潪闆讹紙鎵ц鐢垫簮寰幆锛?鍗冲厛缁欑郴缁熸柇鐢碉紝鐒跺悗鍦ㄥ嚑绉掑唴閲嶆柊涓婄數锛夈€傚湪鍐呮牳鍛戒护琛屼笂璁剧疆 ipmi_poweroff.poweroff_control=x
浼氬仛鍚屾牱鐨勪簨銆傝鍙傛暟涔熷彲閫氳繃 proc 鏂囦欢绯荤粺鍦?/proc/sys/dev/ipmi/poweroff_powercycle 涓幏鍙栥€?娉ㄦ剰锛屽鏋滅郴缁熶笉鏀寔鐢垫簮寰幆锛屽畠鎬绘槸浼氭墽琛屾柇鐢点€?
"ifnum_to_use" 鍙傛暟鎸囧畾鍏虫満浠ｇ爜搴斾娇鐢ㄥ摢涓帴鍙ｃ€傞粯璁や负 -1锛岃〃绀洪€夊彇绗竴涓敞鍐岀殑鎺ュ彛銆?
娉ㄦ剰锛屽鏋滀綘鍚敤浜?ACPI锛岀郴缁熶細浼樺厛浣跨敤 ACPI 鍏虫満銆?