
## CEC 鍐呮牳鏀寔


CEC 妗嗘灦涓?HDMI CEC 纭欢鎻愪緵浜嗕竴涓粺涓€鐨勫唴鏍告帴鍙ｃ€傚畠鏃ㄥ湪澶勭悊澶氱绫诲瀷鐨勭‖浠讹紙鎺ユ敹鍣ㄣ€佸彂閫佸櫒銆乁SB 閫傞厤鍣級銆傝妗嗘灦杩樻彁渚涗簡鍦ㄥ唴鏍搁┍鍔ㄤ腑鍋氫粈涔堛€佷互鍙婂湪鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涓鐞嗕粈涔堢殑閫夐」銆傛澶栵紝瀹冨皢閬ユ帶鍣ㄩ€忎紶鐗规€ч泦鎴愬埌浜嗗唴鏍哥殑閬ユ帶鍣ㄦ鏋朵腑銆?

### CEC 鍗忚


CEC 鍗忚浣挎秷璐圭數瀛愯澶囪兘澶熼€氳繃 HDMI 杩炴帴鐩镐簰閫氫俊銆傝鍗忚鍦ㄩ€氫俊涓娇鐢ㄩ€昏緫鍦板潃銆傞€昏緫鍦板潃涓庤澶囨墍鎻愪緵鐨勫姛鑳戒弗鏍肩浉鍏炽€傚厖褰撻€氫俊鏋㈢航鐨勭數瑙嗘€绘槸琚垎閰嶅湴鍧€ 0銆傜墿鐞嗗湴鍧€鐢辫澶囦箣闂寸殑鐗╃悊杩炴帴鍐冲畾銆?
姝ゅ鎻忚堪鐨?CEC 妗嗘灦涓?CEC 2.0 瑙勮寖淇濇寔鍚屾銆傚畠鍦?HDMI 1.4 瑙勮寖涓湁璁拌浇锛屾柊鐨?2.0 閮ㄥ垎璁板綍鍦?HDMI 2.0 瑙勮寖涓€備絾瀵逛簬澶у鏁扮壒鎬ц€岃█锛屽彲鍏嶈垂鑾峰彇鐨?HDMI 1.3a 瑙勮寖宸茶冻澶燂細

https://www.hdmi.org/spec/index


### CEC 閫傞厤鍣ㄦ帴鍙?

struct cec_adapter 琛ㄧず CEC 閫傞厤鍣ㄧ‖浠躲€傚畠閫氳繃璋冪敤 cec_allocate_adapter() 鍒涘缓锛岄€氳繃璋冪敤 cec_delete_adapter() 鍒犻櫎锛?
   struct cec_adapter *cec_allocate_adapter(const struct cec_adap_ops *ops, \
					    void *priv, const char *name, \
					    u32 caps, u8 available_las);

   void cec_delete_adapter(struct cec_adapter *adap);

瑕佸垱寤轰竴涓€傞厤鍣紝浣犻渶瑕佷紶鍏ヤ互涓嬩俊鎭細

ops:
	鐢?CEC 妗嗘灦璋冪敤銆佷笖浣犻渶瑕佸疄鐜扮殑閫傞厤鍣ㄦ搷浣溿€?
priv:
	浼氳瀛樺偍鍦?adap->priv 涓紝骞跺彲渚涢€傞厤鍣ㄦ搷浣滀娇鐢ㄣ€備娇鐢?cec_get_drvdata(adap) 鑾峰彇璇?priv 鎸囬拡銆?
name:
	CEC 閫傞厤鍣ㄧ殑鍚嶇О銆傛敞鎰忥細姝ゅ悕绉颁細琚鍒躲€?
caps:
	CEC 閫傞厤鍣ㄧ殑鑳藉姏銆傝繖浜涜兘鍔涘喅瀹氫簡纭欢鐨勮兘鍔涳紝浠ュ強鍝簺閮ㄥ垎鐢辩敤鎴风┖闂村鐞嗐€佸摢浜涢儴鍒嗙敱鍐呮牳绌洪棿澶勭悊銆傝繖浜涜兘鍔涚敱 CEC_ADAP_G_CAPS 杩斿洖銆?
available_las:
	璇ラ€傞厤鍣ㄨ兘鍚屾椂澶勭悊鐨勯€昏緫鍦板潃鏁伴噺銆傚繀椤绘弧瓒?1 <= available_las <= CEC_MAX_LOG_ADDRS銆?
瑕佽幏鍙?priv 鎸囬拡锛屼娇鐢ㄦ杈呭姪鍑芥暟锛?
	void *cec_get_drvdata(const struct cec_adapter *adap);

瑕佹敞鍐?/dev/cecX 璁惧鑺傜偣鍜岄仴鎺у櫒璁惧锛堝鏋滆缃簡 CEC_CAP_RC锛夛紝浣犺皟鐢細

	int cec_register_adapter(struct cec_adapter *adap, \
				 struct device *parent);

鍏朵腑 parent 鏄埗璁惧銆?
瑕佹敞閿€璁惧锛岃皟鐢細

	void cec_unregister_adapter(struct cec_adapter *adap);

娉ㄦ剰锛氬鏋?cec_register_adapter() 澶辫触锛屽垯璋冪敤 cec_delete_adapter() 杩涜娓呯悊銆備絾濡傛灉 cec_register_adapter() 鎴愬姛锛屽垯鍙皟鐢?cec_unregister_adapter() 娓呯悊锛岀粷涓嶈璋冪敤 cec_delete_adapter()銆備竴鏃﹁ /dev/cecX 璁惧鐨勬渶鍚庝竴涓敤鎴峰叧闂簡鍏舵枃浠跺彞鏌勶紝娉ㄩ攢鍑芥暟灏嗚嚜鍔ㄥ垹闄ら€傞厤鍣ㄣ€?

### 瀹炵幇搴曞眰 CEC 閫傞厤鍣?

浠ヤ笅搴曞眰閫傞厤鍣ㄦ搷浣滃繀椤诲湪浣犵殑椹卞姩涓疄鐜帮細


	struct cec_adap_ops
	{
		/** Low-level callbacks **/
		int (*adap_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_monitor_all_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_monitor_pin_enable)(struct cec_adapter *adap, bool enable);
		int (*adap_log_addr)(struct cec_adapter *adap, u8 logical_addr);
		void (*adap_unconfigured)(struct cec_adapter *adap);
		int (*adap_transmit)(struct cec_adapter *adap, u8 attempts,
				      u32 signal_free_time, struct cec_msg *msg);
		void (*adap_nb_transmit_canceled)(struct cec_adapter *adap,
						  const struct cec_msg *msg);
		void (*adap_status)(struct cec_adapter *adap, struct seq_file *file);
		void (*adap_free)(struct cec_adapter *adap);

		/** Error injection callbacks **/
		...

		/** High-level callback **/
		...
	};

杩欎簺搴曞眰鎿嶄綔鐢ㄤ簬澶勭悊鎺у埗 CEC 閫傞厤鍣ㄧ‖浠剁殑鍚勪釜鏂归潰銆傚畠浠兘鍦ㄦ寔鏈変簰鏂ラ攣 adap->lock 鐨勬儏鍐典笅琚皟鐢ㄣ€?

```

	int (*adap_enable)(struct cec_adapter *adap, bool enable);

```
姝ゅ洖璋冨惎鐢ㄦ垨绂佺敤 CEC 纭欢銆傚惎鐢?CEC 纭欢鎰忓懗鐫€灏嗗叾涓婄數鍒颁竴涓湭澹版槑浠讳綍閫昏緫鍦板潃鐨勭姸鎬併€傚鏋滆缃簡 CEC_CAP_NEEDS_HPD锛岀墿鐞嗗湴鍧€灏嗗缁堟湁鏁堛€傚鏋滄湭璁剧疆璇ヨ兘鍔涳紝鍒欑墿鐞嗗湴鍧€鍙兘鍦?CEC 纭欢鍚敤鏈熼棿鍙戠敓鍙樺寲銆侰EC 椹卞姩涓嶅簲璁剧疆 CEC_CAP_NEEDS_HPD锛岄櫎闈炵‖浠惰璁℃湁姝よ姹傦紝鍥犱负杩欎細浣挎棤娉曞敜閱掑湪寰呮満妯″紡涓嬪皢 HPD 鎷変綆鐨勬樉绀哄櫒銆傝皟鐢?cec_allocate_adapter() 鍚?CEC 閫傞厤鍣ㄧ殑鍒濆鐘舵€佹槸绂佺敤鐨勩€?
娉ㄦ剰锛屽鏋?enable 涓?false锛宎dap_enable 蹇呴』杩斿洖 0銆?

```

	int (*adap_monitor_all_enable)(struct cec_adapter *adap, bool enable);

```
濡傛灉鍚敤锛屽垯閫傞厤鍣ㄥ簲琚疆浜庝竴绉嶄篃鐩戣闈炲彂閫佺粰鏈満鐨勬秷鎭殑妯″紡銆傚苟闈炴墍鏈夌‖浠堕兘鏀寔姝ゅ姛鑳斤紝涓斿彧鏈夊湪璁剧疆浜?CEC_CAP_MONITOR_ALL 鑳藉姏鏃舵墠浼氳皟鐢ㄦ鍑芥暟銆傛鍥炶皟鏄彲閫夌殑锛堟煇浜涚‖浠跺彲鑳藉缁堝浜庘€渕onitor all鈥濇ā寮忥級銆?
娉ㄦ剰锛屽鏋?enable 涓?false锛宎dap_monitor_all_enable 蹇呴』杩斿洖 0銆?

```

	int (*adap_monitor_pin_enable)(struct cec_adapter *adap, bool enable);

```
濡傛灉鍚敤锛屽垯閫傞厤鍣ㄥ簲琚疆浜庝竴绉嶄篃鐩戣 CEC 寮曡剼鍙樺寲鐨勬ā寮忋€傚苟闈炴墍鏈夌‖浠堕兘鏀寔姝ゅ姛鑳斤紝涓斿彧鏈夊湪璁剧疆浜?CEC_CAP_MONITOR_PIN 鑳藉姏鏃舵墠浼氳皟鐢ㄦ鍑芥暟銆傛鍥炶皟鏄彲閫夌殑锛堟煇浜涚‖浠跺彲鑳藉缁堝浜庘€渕onitor pin鈥濇ā寮忥級銆?
娉ㄦ剰锛屽鏋?enable 涓?false锛宎dap_monitor_pin_enable 蹇呴』杩斿洖 0銆?

```

	int (*adap_log_addr)(struct cec_adapter *adap, u8 logical_addr);

```
濡傛灉 logical_addr == CEC_LOG_ADDR_INVALID锛屽垯鎵€鏈夊凡缂栫▼鐨勯€昏緫鍦板潃閮藉簲琚摝闄ゃ€傚惁鍒欏簲缂栫▼缁欏畾鐨勯€昏緫鍦板潃銆傚鏋滆秴杩囦簡鍙敤閫昏緫鍦板潃鐨勬渶澶ф暟閲忥紝鍒欏簲杩斿洖 -ENXIO銆備竴鏃︽煇涓€昏緫鍦板潃琚紪绋嬶紝CEC 纭欢灏辫兘鎺ユ敹鍙戝線璇ュ湴鍧€鐨勫畾鍚戞秷鎭€?
娉ㄦ剰锛屽鏋?logical_addr 涓?CEC_LOG_ADDR_INVALID锛宎dap_log_addr 蹇呴』杩斿洖 0銆?

```

	void (*adap_unconfigured)(struct cec_adapter *adap);

```
閫傞厤鍣ㄥ凡鍙栨秷閰嶇疆銆傚鏋滈┍鍔ㄥ湪鍙栨秷閰嶇疆鍚庡繀椤婚噰鍙栫壒瀹氭搷浣滐紝鍒欏彲浠ラ€氳繃姝ゅ彲閫夊洖璋冩潵瀹屾垚銆?

```

	int (*adap_transmit)(struct cec_adapter *adap, u8 attempts,
			     u32 signal_free_time, struct cec_msg *msg);

```
杩欎細鍙戦€佷竴鏉℃柊娑堟伅銆俛ttempts 鍙傛暟鏄缓璁殑鍙戦€佸皾璇曟鏁般€?
signal_free_time 鏄€傞厤鍣ㄥ湪绾胯矾绌洪棽鏃躲€佸皾璇曞彂閫佹秷鎭墠搴旂瓑寰呯殑鏁版嵁浣嶅懆鏈熸暟銆傝鍊煎彇鍐充簬鏈鍙戦€佹槸閲嶈瘯銆佹潵鑷柊鍙戣捣鑰呯殑娑堟伅锛岃繕鏄悓涓€鍙戣捣鑰呯殑鏂版秷鎭€傚ぇ澶氭暟纭欢浼氳嚜鍔ㄥ鐞嗚繖涓€鐐癸紝浣嗗湪鏌愪簺鎯呭喌涓嬮渶瑕佹淇℃伅銆?
CEC_FREE_TIME_TO_USEC 瀹忓彲鐢ㄤ簬灏?signal_free_time 杞崲涓哄井绉掞紙涓€涓暟鎹綅鍛ㄦ湡涓?2.4 ms锛夈€?

```

	void (*adap_nb_transmit_canceled)(struct cec_adapter *adap,
					  const struct cec_msg *msg);

```
姝ゅ彲閫夊洖璋冨彲鐢ㄤ簬鑾峰彇搴忓垪鍙蜂负 msg->sequence 鐨勮鍙栨秷闈為樆濉炲彂閫佺殑缁撴灉銆傚湪浠ヤ笅鎯呭喌涓嬭皟鐢細鍙戦€佽涓銆佸彂閫佽秴鏃讹紙鍗崇‖浠朵粠鏈彂鍑哄彂閫佸畬鎴愮殑淇″彿锛夛紝鎴栬€呭彂閫佹垚鍔熶絾绛夊緟棰勬湡鍥炲鏃惰涔堣涓瑕佷箞瓒呮椂銆?

```

	void (*adap_status)(struct cec_adapter *adap, struct seq_file *file);

```
姝ゅ彲閫夊洖璋冨彲鐢ㄤ簬鏄剧ず CEC 纭欢鐨勭姸鎬併€傝鐘舵€佸彲閫氳繃 debugfs 鑾峰彇锛歝at /sys/kernel/debug/cec/cecX/status


```

	void (*adap_free)(struct cec_adapter *adap);

```
姝ゅ彲閫夊洖璋冨彲鐢ㄤ簬閲婃斁椹卞姩鍙兘宸插垎閰嶇殑浠讳綍璧勬簮銆傚畠鐢?cec_delete_adapter 璋冪敤銆?

浣犵殑閫傞厤鍣ㄩ┍鍔ㄨ繕蹇呴』鍦ㄤ互涓嬫儏鍐典笅锛堥€氬父鏄腑鏂┍鍔級閫氳繃璋冪敤妗嗘灦鏉ュ搷搴斾簨浠讹細

```

	void cec_transmit_done(struct cec_adapter *adap, u8 status,
			       u8 arb_lost_cnt,  u8 nack_cnt, u8 low_drive_cnt,
			       u8 error_cnt);

```
```

	void cec_transmit_attempt_done(struct cec_adapter *adap, u8 status);

```
   鐘舵€佸彲浠ユ槸浠ヤ笅涔嬩竴锛?
CEC_TX_STATUS_OK:
	鍙戦€佹垚鍔熴€?
CEC_TX_STATUS_ARB_LOST:
	浠茶澶辫触锛氬彟涓€涓?CEC 鍙戣捣鑰呮帶鍒朵簡 CEC 绾胯矾锛屼綘澶卞幓浜嗕徊瑁併€?
CEC_TX_STATUS_NACK:
	娑堟伅琚?nack锛堝浜庡畾鍚戞秷鎭級鎴?ack锛堝浜庡箍鎾秷鎭級銆傞渶瑕侀噸浼犮€?
CEC_TX_STATUS_LOW_DRIVE:
	鍦?CEC 鎬荤嚎涓婃娴嬪埌 low drive銆傝繖琛ㄦ槑鏌愪釜璺熼殢鑰呮娴嬪埌鎬荤嚎涓婄殑閿欒骞惰姹傞噸浼犮€?
CEC_TX_STATUS_ERROR:
	鍙戠敓浜嗘煇绉嶆湭鎸囧畾鐨勯敊璇細濡傛灉纭欢鏃犳硶鍖哄垎锛岃繖鍙兘鏄?ARB_LOST 鎴?LOW_DRIVE 涔嬩竴锛屾垨鑰呮槸瀹屽叏涓嶅悓鐨勬儏鍐点€傛煇浜涚‖浠跺彧鏀寔灏?OK 鍜?FAIL 浣滀负鍙戦€佺粨鏋滐紝鍗虫棤娉曞尯鍒嗕笉鍚岀殑鍙兘閿欒銆傚湪杩欑鎯呭喌涓嬶紝灏?FAIL 鏄犲皠涓?CEC_TX_STATUS_NACK 鑰岄潪 CEC_TX_STATUS_ERROR銆?
CEC_TX_STATUS_MAX_RETRIES:
	灏濊瘯澶氭鍚庝粛鏃犳硶鍙戦€佹秷鎭€傚簲浠呯敱鍏锋湁娑堟伅閲嶈瘯纭欢鏀寔鐨勯┍鍔ㄨ缃€傚鏋滆缃紝妗嗘灦浼氬亣瀹氬畠鏃犻渶鍐嶆灏濊瘯鍙戦€佽娑堟伅锛屽洜涓虹‖浠跺凡缁忚繖鏍峰仛浜嗐€?
纭欢蹇呴』鑳藉鍖哄垎 OK銆丯ACK 鍜屸€滃叾浠栨儏鍐碘€濄€?
\*_cnt 鍙傛暟鏄墍瑙傚療鍒扮殑閿欒鏉′欢鏁伴噺銆傚鏋滄病鏈夊彲鐢ㄤ俊鎭紝鍙互涓?0銆備笉鏀寔纭欢閲嶈瘯鐨勯┍鍔ㄥ彧闇€灏嗕笌鍙戦€侀敊璇搴旂殑璁℃暟鍣ㄨ涓?1锛涘鏋滅‖浠剁‘瀹炴敮鎸侀噸璇曪紝鍒欏綋纭欢涓嶆彁渚涘彂鐢熶簡鍝簺閿欒浠ュ強鍙戠敓娆℃暟鐨勫弽棣堟椂锛屽皢杩欎簺璁℃暟鍣ㄨ涓?0锛屽惁鍒欏～鍏ョ‖浠舵姤鍛婄殑姝ｇ‘鍊笺€?
璇锋敞鎰忥紝濡傛灉瀛樺湪鎺掗槦涓緟鍙戦€佺殑娑堟伅锛岃皟鐢ㄨ繖浜涘嚱鏁板彲鑳戒細绔嬪嵆寮€濮嬩竴娆℃柊鐨勫彂閫併€傚洜姝わ紝鍦ㄨ皟鐢ㄨ繖浜涘嚱鏁?*涔嬪墠**锛岃纭繚纭欢澶勪簬鍙互寮€濮嬫柊鍙戦€佺殑鐘舵€併€?
cec_transmit_attempt_done() 鍑芥暟鏄竴涓緟鍔╁嚱鏁帮紝鐢ㄤ簬纭欢浠庝笉閲嶈瘯鐨勬儏鍐碉紝鍥犳鍙戦€佹€绘槸鍙湁鍗曟灏濊瘯銆傚畠浼氭帴鐫€璋冪敤 cec_transmit_done()锛屽皢瀵瑰簲鐘舵€佺殑 count 鍙傛暟濉负 1銆傚鏋滅姸鎬佷负 OK锛屽垯鍏ㄩ儴濉?0銆?
褰撴帴鏀跺埌涓€鏉?CEC 娑堟伅鏃讹細

	void cec_received_msg(struct cec_adapter *adap, struct cec_msg *msg);

涓嶈█鑷槑銆?
### 瀹炵幇涓柇澶勭悊绋嬪簭


閫氬父锛孋EC 纭欢浼氭彁渚涗腑鏂紝鐢ㄤ簬鎸囩ず鍙戦€佷綍鏃跺畬鎴愪互鍙婃槸鍚︽垚鍔燂紝骞跺湪鎺ユ敹鍒?CEC 娑堟伅鏃舵彁渚涗腑鏂€?
CEC 椹卞姩搴斿缁堝厛澶勭悊鍙戦€佷腑鏂紝鍐嶅鐞嗘帴鏀朵腑鏂€傛鏋舵湡鏈涘湪 cec_received_msg 璋冪敤涔嬪墠鐪嬪埌 cec_transmit_done 璋冪敤锛屽惁鍒欏鏋滄帴鏀跺埌鐨勬秷鎭槸瀵瑰凡鍙戦€佹秷鎭殑鍥炲锛屾鏋跺彲鑳戒細娣锋穯銆?
### 鍙€夛細瀹炵幇閿欒娉ㄥ叆鏀寔


濡傛灉 CEC 閫傞厤鍣ㄦ敮鎸侀敊璇敞鍏ワ紙Error Injection锛夊姛鑳斤紝鍒欏彲浠ラ€氳繃閿欒娉ㄥ叆鍥炶皟灏嗗叾鏆撮湶鍑烘潵锛?

	struct cec_adap_ops {
		/** Low-level callbacks **/
		...

		/** Error injection callbacks **/
		int (*error_inj_show)(struct cec_adapter *adap, struct seq_file *sf);
		bool (*error_inj_parse_line)(struct cec_adapter *adap, char *line);

		/** High-level CEC message callback **/
		...
	};

濡傛灉涓や釜鍥炶皟閮借璁剧疆锛屽垯浼氬湪 debugfs 涓嚭鐜颁竴涓?`error-inj` 鏂囦欢銆傚熀鏈娉曞涓嬶細

   鍓嶅绌烘牸/鍒惰〃绗︿細琚拷鐣ャ€傚鏋滀笅涓€涓瓧绗︽槸 `#` 鎴栧埌杈句簡琛屽熬锛屽垯鏁磋琚拷鐣ャ€傚惁鍒欓鏈熸槸涓€鏉″懡浠ゃ€?
   姝ゅ熀鏈В鏋愬湪 CEC 妗嗘灦涓畬鎴愩€傜敱椹卞姩鍐冲畾瀹炵幇鍝簺鍛戒护銆傚敮涓€鐨勮姹傛槸蹇呴』瀹炵幇涓嶅甫浠讳綍鍙傛暟鐨?`clear` 鍛戒护锛屽苟涓斿畠浼氱Щ闄ゆ墍鏈夊綋鍓嶇殑閿欒娉ㄥ叆鍛戒护銆?
   杩欑‘淇濅綘濮嬬粓鍙互鎵ц `echo clear >error-inj` 鏉ユ竻闄や换浣曢敊璇敞鍏ワ紝鑰屾棤闇€浜嗚В椹卞姩鐗瑰畾鍛戒护鐨勭粏鑺傘€?
   娉ㄦ剰 `error-inj` 鐨勮緭鍑哄簲鍙綔涓?`error-inj` 鐨勮緭鍏ャ€傚洜姝よ繖蹇呴』鏈夋晥锛?

	$ cat error-inj >einj.txt
	$ cat einj.txt >error-inj

绗竴涓洖璋冨湪璇诲彇姝ゆ枃浠舵椂琚皟鐢紝瀹冨簲鏄剧ず
```

	int (*error_inj_show)(struct cec_adapter *adap, struct seq_file *sf);

```
   寤鸿瀹冧互涓€涓甫鏈夊熀鏈敤娉曚俊鎭殑娉ㄩ噴鍧楀紑澶淬€傛垚鍔熸椂杩斿洖 0锛屽惁鍒欒繑鍥為敊璇€?```

	bool (*error_inj_parse_line)(struct cec_adapter *adap, char *line);

```
   `line` 鍙傛暟鎸囧悜鍛戒护鐨勮捣濮嬩綅缃€備换浣曞墠瀵肩┖鏍兼垨鍒惰〃绗﹂兘宸茶璺宠繃銆傚畠鍙槸涓€琛岋紙鍥犳娌℃湁鍐呭祵鐨勬崲琛岀锛夛紝骞朵互 0 缁撳熬銆傝鍥炶皟鍙互鑷敱淇敼缂撳啿鍖虹殑鍐呭銆傚畠浠呭鍖呭惈鍛戒护鐨勮璋冪敤锛屽洜姝ゅ浜庣┖琛屾垨娉ㄩ噴琛屾案杩滀笉浼氳皟鐢ㄦ鍥炶皟銆?
   濡傛灉鍛戒护鏈夋晥鍒欒繑鍥?true锛屽鏋滃瓨鍦ㄨ娉曢敊璇垯杩斿洖 false銆?
### 瀹炵幇楂樺眰 CEC 閫傞厤鍣?

搴曞眰鎿嶄綔椹卞姩纭欢锛岄珮灞傛搷浣滅敱 CEC 鍗忚椹卞姩銆傞珮灞傚洖璋冨湪鏈寔鏈?adap->lock 浜掓枼閿佺殑鎯呭喌涓嬭璋冪敤銆傚彲鐢ㄧ殑楂樺眰鍥炶皟濡備笅锛?

	struct cec_adap_ops {
		/** Low-level callbacks **/
		...

		/** Error injection callbacks **/
		...

		/** High-level CEC message callback **/
		void (*configured)(struct cec_adapter *adap);
		int (*received)(struct cec_adapter *adap, struct cec_msg *msg);
	};

```

	void (*configured)(struct cec_adapter *adap);

```
   閫傞厤鍣ㄥ凡瀹屽叏閰嶇疆锛屽嵆鎵€鏈夐€昏緫鍦板潃閮藉凡鎴愬姛澹版槑銆傚鏋滈┍鍔ㄥ湪閰嶇疆鍚庡繀椤婚噰鍙栫壒瀹氭搷浣滐紝鍒欏彲浠ラ€氳繃姝ゅ彲閫夊洖璋冩潵瀹屾垚銆?

received() 鍥炶皟鍏佽椹卞姩鍙€夊湴澶勭悊涓€鏉℃柊
```

	int (*received)(struct cec_adapter *adap, struct cec_msg *msg);

```
   濡傛灉椹卞姩鎯宠澶勭悊涓€鏉?CEC 娑堟伅锛屽垯鍙互瀹炵幇姝ゅ洖璋冦€傚鏋滃畠涓嶆兂澶勭悊璇ユ秷鎭紝鍒欏簲杩斿洖 -ENOMSG锛屽惁鍒?CEC 妗嗘灦浼氬亣瀹氬畠宸插鐞嗚娑堟伅锛屽苟涓斾笉浼氬啀瀵瑰叾鍋氫换浣曞鐞嗐€?

### CEC 妗嗘灦鍑芥暟


CEC 閫傞厤鍣ㄩ┍鍔ㄥ彲浠ヨ皟鐢ㄤ互涓?CEC 妗嗘灦鍑芥暟锛?
   int cec_transmit_msg(struct cec_adapter *adap, struct cec_msg *msg, \
			bool block);

   鍙戦€佷竴鏉?CEC 娑堟伅銆傚鏋?block 涓?true锛屽垯绛夊緟娑堟伅琚彂閫佸畬姣曪紝鍚﹀垯鍙皢鍏跺叆闃熷苟杩斿洖銆?
   void cec_s_phys_addr(struct cec_adapter *adap, u16 phys_addr, bool block);

   鏇存敼鐗╃悊鍦板潃銆傛鍑芥暟浼氳缃?adap->phys_addr锛屽苟鍦ㄥ叾鍙戠敓鍙樺寲鏃跺彂閫佷竴涓簨浠躲€傚鏋滃凡璋冪敤 cec_s_log_addrs() 涓旂墿鐞嗗湴鍧€宸插彉涓烘湁鏁堬紝鍒?CEC 妗嗘灦灏嗗紑濮嬪０鏄庨€昏緫鍦板潃銆傚鏋?block 涓?true锛屽垯姝ゅ嚱鏁板湪璇ヨ繃绋嬪畬鎴愪箣鍓嶄笉浼氳繑鍥炪€?
   褰撶墿鐞嗗湴鍧€琚涓烘湁鏁堝€兼椂锛孋EC 閫傞厤鍣ㄥ皢琚惎鐢紙鍙傝 adap_enable 鎿嶄綔锛夈€傚綋瀹冭璁句负 CEC_PHYS_ADDR_INVALID 鏃讹紝CEC 閫傞厤鍣ㄥ皢琚鐢ㄣ€傚鏋滀綘灏嗘湁鏁堢殑鐗╃悊鍦板潃鏇存敼涓哄彟涓€涓湁鏁堢殑鐗╃悊鍦板潃锛屽垯姝ゅ嚱鏁颁細鍏堝皢鍦板潃璁句负 CEC_PHYS_ADDR_INVALID锛屽啀鍚敤鏂扮殑鐗╃悊鍦板潃銆?
   void cec_s_phys_addr_from_edid(struct cec_adapter *adap, \
				  const struct edid *edid);

   涓€涓緟鍔╁嚱鏁帮紝浠?edid 缁撴瀯浣撲腑鎻愬彇鐗╃悊鍦板潃锛屽苟鐢ㄨ鍦板潃璋冪敤 cec_s_phys_addr()锛屾垨鑰呭鏋?EDID 涓嶅寘鍚墿鐞嗗湴鍧€鎴?edid 涓?NULL 鎸囬拡锛屽垯浣跨敤 CEC_PHYS_ADDR_INVALID 璋冪敤銆?
	int cec_s_log_addrs(struct cec_adapter *adap, \
			    struct cec_log_addrs *log_addrs, bool block);

   澹版槑 CEC 閫昏緫鍦板潃銆傚鏋滆缃簡 CEC_CAP_LOG_ADDRS锛屽垯缁濅笉搴旇皟鐢ㄣ€傚鏋?block 涓?true锛屽垯绛夊緟閫昏緫鍦板潃琚０鏄庯紝鍚﹀垯鍙皢鍏跺叆闃熷苟杩斿洖銆傝鍙栨秷閰嶇疆鎵€鏈夐€昏緫鍦板潃锛屽彲灏?log_addrs 璁句负 NULL锛屾垨灏?log_addrs->num_log_addrs 璁句负 0 鏉ヨ皟鐢ㄦ鍑芥暟銆傚彇娑堥厤缃椂浼氬拷鐣?block 鍙傛暟銆傚鏋滅墿鐞嗗湴鍧€鏃犳晥锛屾鍑芥暟灏嗙洿鎺ヨ繑鍥炪€備竴鏃︾墿鐞嗗湴鍧€鍙樹负鏈夋晥锛屾鏋跺皢灏濊瘯澹版槑杩欎簺閫昏緫鍦板潃銆?

### CEC 寮曡剼妗嗘灦


澶у鏁?CEC 纭欢鍩轰簬瀹屾暣鐨?CEC 娑堟伅宸ヤ綔锛氳蒋浠舵彁渚涙秷鎭紝纭欢澶勭悊搴曞眰 CEC 鍗忚銆備絾鏈変簺纭欢鍙┍鍔?CEC 寮曡剼锛岃蒋浠跺繀椤诲鐞嗗簳灞?CEC 鍗忚銆侰EC 寮曡剼妗嗘灦姝ｆ槸涓哄鐞嗘绫昏澶囪€屽垱寤虹殑銆?
娉ㄦ剰锛岀敱浜庢帴杩戝疄鏃剁殑瑕佹眰锛屾案杩滄棤娉曚繚璇佸叾 100% 宸ヤ綔銆傝妗嗘灦鍦ㄥ唴閮ㄤ娇鐢ㄩ珮绮惧害瀹氭椂鍣紙highres timers锛夛紝浣嗗鏋滃畾鏃跺櫒鏅氳Е鍙戣秴杩?300 寰锛屽氨鍙兘鍑虹幇閿欒缁撴灉銆傚疄闄呬笂瀹冧技涔庣浉褰撳彲闈犮€?
杩欑搴曞眰瀹炵幇鐨勪竴涓紭鍔挎槸瀹冨彲浠ヤ綔涓轰竴绉嶅粔浠风殑 CEC 鍒嗘瀽浠娇鐢紝鐗瑰埆鏄湪鍙互浣跨敤涓柇鏉ユ娴?CEC 寮曡剼浠庝綆鍒伴珮锛堟垨鍙嶄箣锛夌殑璺冲彉鏃躲€?

### CEC 閫氱煡鍣ㄦ鏋?

澶у鏁?drm HDMI 瀹炵幇閮芥湁闆嗘垚鐨?CEC 瀹炵幇锛屼笉闇€瑕侀€氱煡鍣ㄦ敮鎸併€備絾鏈変簺鍏锋湁鐙珛鐨?CEC 瀹炵幇锛屽畠浠嫢鏈夎嚜宸辩殑椹卞姩銆傝繖鍙兘鏄?SoC 鐨勪竴涓?IP 鍧楋紝鎴栬€呮槸澶勭悊 CEC 寮曡剼鐨勫畬鍏ㄧ嫭绔嬬殑鑺墖銆傚浜庤繖浜涙儏鍐碉紝drm 椹卞姩鍙互瀹夎涓€涓€氱煡鍣紙notifier锛夛紝骞朵娇鐢ㄨ閫氱煡鍣ㄥ皢鐗╃悊鍦板潃鐨勫彉鍖栧憡鐭?CEC 椹卞姩銆?