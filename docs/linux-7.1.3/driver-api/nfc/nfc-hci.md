## NFC 鏍稿績鐨?HCI 鍚庣


- Author: Eric Lapuyade, Samuel Ortiz
- Contact: eric.lapuyade@intel.com, samuel.ortiz@intel.com

### 姒傝堪


HCI 灞傚疄鐜颁簡 ETSI TS 102 622 V10.2.0 瑙勮寖鐨勫緢澶ч儴鍒嗐€傚畠浣垮緱缂栧啓鍩轰簬 HCI 鐨?NFC 椹卞姩鍙樺緱瀹规槗銆?HCI 灞備綔涓?NFC 鏍稿績鐨勪竴涓悗绔繍琛岋紝瀹炵幇涓€涓娊璞＄殑 nfc 璁惧锛屽苟灏?NFC 鏍稿績 API 杞崲涓?HCI 鍛戒护
鍜屼簨浠躲€?
### HCI

HCI 浠?nfc 璁惧鐨勮韩浠藉悜 NFC 鏍稿績娉ㄥ唽銆傛潵鑷敤鎴风┖闂寸殑璇锋眰閫氳繃 netlink 濂楁帴瀛楄矾鐢卞埌 NFC 鏍稿績锛?鐒跺悗鍒?HCI銆備粠杩欎竴鐐瑰紑濮嬶紝瀹冧滑琚浆鎹负鍙戝線涓绘満鎺у埗鍣紙鑺墖锛変腑 HCI 灞傜殑涓€绯诲垪 HCI 鍛戒护銆傚懡浠ゅ彲浠?鍚屾鎵ц锛堝彂閫佷笂涓嬫枃闃诲绛夊緟鍝嶅簲锛夋垨寮傛鎵ц锛堝搷搴斾粠 HCI Rx 涓婁笅鏂囪繑鍥烇級銆侶CI 浜嬩欢涔熷彲浠ヤ粠涓绘満
鎺у埗鍣ㄦ帴鏀躲€傚畠浠皢琚鐞嗭紝骞跺湪闇€瑕佹椂鍚?NFC 鏍稿績杞彂涓€涓浆鎹㈢粨鏋溿€傛湁涓€浜涢挬瀛愯 HCI 椹卞姩澶勭悊涓撴湁
浜嬩欢鎴栬鐩栨爣鍑嗚涓恒€侶CI 浣跨敤 2 涓墽琛屼笂涓嬫枃锛?
- 涓€涓敤浜庢墽琛屽懡浠わ細nfc_hci_msg_tx_work()銆備换浣曟椂鍒诲彧鑳芥湁涓€涓懡浠ゅ湪鎵ц銆?- 涓€涓敤浜庡垎鍙戞帴鏀跺埌鐨勪簨浠跺拰鍛戒护锛歯fc_hci_msg_rx_work()銆?
### HCI 浼氳瘽鍒濆鍖?
浼氳瘽鍒濆鍖栨槸涓€涓?HCI 鏍囧噯锛屼絾閬楁喚鐨勬槸蹇呴』鏀寔涓撴湁闂紙gate锛夈€傝繖灏辨槸涓轰粈涔堥┍鍔ㄥ皢浼犻€掍竴涓繀椤讳綔涓?浼氳瘽涓€閮ㄥ垎鐨勪笓鏈夐棬鍒楄〃銆侶CI 灏嗙‘淇濆湪 hci 璁惧寤虹珛鏃舵墍鏈夎繖浜涢棬閮芥湁绠￠亾杩炴帴銆傚鏋滆姱鐗囨敮鎸侀鎵撳紑鐨勯棬
鍜屼吉闈欐€佺閬擄紝椹卞姩鍙互灏嗚淇℃伅浼犻€掔粰 HCI 鏍稿績銆?
### HCI 闂ㄤ笌绠￠亾

涓€涓棬瀹氫箟浜嗗彲浠ユ壘鍒版煇绉嶆湇鍔＄殑鈥滅鍙ｂ€濄€備负浜嗚闂竴椤规湇鍔★紝蹇呴』鍒涘缓涓€涓埌璇ラ棬鐨勭閬撳苟鎵撳紑瀹冦€傚湪姝?瀹炵幇涓紝绠￠亾瀹屽叏琚殣钘忋€傚叕鍏?API 鍙煡閬撻棬銆傝繖涓庨┍鍔ㄩ渶瑕佸悜涓撴湁闂ㄥ彂閫佸懡浠よ€屾棤闇€鐭ラ亾杩炴帴鍒板畠鐨勭閬?鐨勯渶姹傛槸涓€鑷寸殑銆?
### 椹卞姩鎺ュ彛


椹卞姩閫氬父鍒嗕袱閮ㄥ垎缂栧啓锛氱墿鐞嗛摼璺鐞嗗拰 HCI 绠＄悊銆傝繖浣垮緱缁存姢涓€涓彲浠ラ€氳繃鍚勭 phy锛坕2c銆乻pi 绛夛級杩炴帴鐨?鑺墖鐨勯┍鍔ㄦ洿瀹规槗銆?
### HCI 绠＄悊

椹卞姩閫氬父浼氬悜 HCI 娉ㄥ唽鑷繁锛屽苟鎻愪緵浠ヤ笅鍐呭锛?
```

  struct nfc_hci_ops {
	int (*open)(struct nfc_hci_dev *hdev);
	void (*close)(struct nfc_hci_dev *hdev);
	int (*hci_ready) (struct nfc_hci_dev *hdev);
	int (*xmit) (struct nfc_hci_dev *hdev, struct sk_buff *skb);
	int (*start_poll) (struct nfc_hci_dev *hdev,
			   u32 im_protocols, u32 tm_protocols);
	int (*dep_link_up)(struct nfc_hci_dev *hdev, struct nfc_target *target,
			   u8 comm_mode, u8 *gb, size_t gb_len);
	int (*dep_link_down)(struct nfc_hci_dev *hdev);
	int (*target_from_gate) (struct nfc_hci_dev *hdev, u8 gate,
				 struct nfc_target *target);
	int (*complete_target_discovered) (struct nfc_hci_dev *hdev, u8 gate,
					   struct nfc_target *target);
	int (*im_transceive) (struct nfc_hci_dev *hdev,
			      struct nfc_target *target, struct sk_buff *skb,
			      data_exchange_cb_t cb, void *cb_context);
	int (*tm_send)(struct nfc_hci_dev *hdev, struct sk_buff *skb);
	int (*check_presence)(struct nfc_hci_dev *hdev,
			      struct nfc_target *target);
	int (*event_received)(struct nfc_hci_dev *hdev, u8 gate, u8 event,
			      struct sk_buff *skb);
  };

```

- open() 鍜?close() 搴斿綋鎵撳紑鍜屽叧闂‖浠躲€?- hci_ready() 鏄竴涓彲閫夌殑鍏ュ彛鐐癸紝鍦?hci 浼氳瘽寤虹珛鍚庣珛鍗宠皟鐢ㄣ€傞┍鍔ㄥ彲浠ヤ娇鐢ㄥ畠鏉ヨ繘琛屽繀椤讳娇鐢?HCI
  鍛戒护瀹屾垚鐨勯澶栧垵濮嬪寲銆?- xmit() 搴斿綋绠€鍗曞湴鍚戠墿鐞嗛摼璺啓鍏ヤ竴甯с€?- start_poll() 鏄竴涓彲閫夌殑鍏ュ彛鐐癸紝搴斿綋灏嗙‖浠惰缃负杞妯″紡銆備粎褰撶‖浠朵娇鐢ㄤ笓鏈夐棬鎴栦笌 HCI 鏍囧噯
  鐣ユ湁涓嶅悓鐨勬満鍒舵椂鎵嶅繀椤诲疄鐜般€?- dep_link_up() 鍦ㄦ娴嬪埌 p2p 鐩爣鍚庤璋冪敤锛屼互浣跨敤闇€瑕佸洖浼犵粰 nfc 鏍稿績鐨勭‖浠跺弬鏁板畬鎴?p2p 杩炴帴璁剧疆銆?- dep_link_down() 琚皟鐢ㄤ互鏂紑 p2p 閾捐矾銆?- target_from_gate() 鏄竴涓彲閫夌殑鍏ュ彛鐐癸紝鐢ㄤ簬杩斿洖涓庝笓鏈夐棬瀵瑰簲鐨?nfc 鍗忚銆?- complete_target_discovered() 鏄竴涓彲閫夌殑鍏ュ彛鐐癸紝璁╅┍鍔ㄦ墽琛岃嚜鍔ㄦ縺娲诲凡鍙戠幇鐩爣鎵€闇€鐨勯澶栦笓鏈?  澶勭悊銆?- im_transceive() 濡傛灉鍚戞爣绛惧彂閫佹暟鎹渶瑕佷笓鏈?HCI 鍛戒护锛屽垯蹇呴』鐢遍┍鍔ㄥ疄鐜般€傛煇浜涙爣绛剧被鍨嬮渶瑕佽嚜瀹氫箟鍛戒护锛?  鍏朵粬鍙互浣跨敤鏍囧噯 HCI 鍛戒护鍐欏叆銆傞┍鍔ㄥ彲浠ユ鏌ユ爣绛剧被鍨嬶紝瑕佷箞杩涜涓撴湁澶勭悊锛岃涔堣繑鍥?1 浠ヨ姹傛爣鍑嗗鐞嗐€?  鏁版嵁浜ゆ崲鍛戒护鏈韩蹇呴』寮傛鍙戦€併€?- tm_send() 鍦?p2p 杩炴帴鐨勬儏鍐典笅琚皟鐢ㄤ互鍙戦€佹暟鎹€?- check_presence() 鏄竴涓彲閫夌殑鍏ュ彛鐐癸紝鏍稿績浼氬畾鏈熻皟鐢ㄥ畠鏉ユ鏌ュ凡婵€娲荤殑鏍囩鏄惁浠嶅湪鍦恒€傚鏋滄湭瀹炵幇锛?  鏍稿績灏嗘棤娉曞悜鐢ㄦ埛绌洪棿鎺ㄩ€?tag_lost 浜嬩欢銆?- event_received() 琚皟鐢ㄤ互澶勭悊鏉ヨ嚜鑺墖鐨勪簨浠躲€傞┍鍔ㄥ彲浠ュ鐞嗚浜嬩欢锛屾垨杩斿洖 1 璁?HCI 灏濊瘯鏍囧噯澶勭悊銆?
鍦?rx 璺緞涓婏紝椹卞姩璐熻矗浣跨敤 nfc_hci_recv_frame() 灏嗕紶鍏ョ殑 HCP 甯ф帹閫佺粰 HCI銆侶CI 灏嗚礋璐ｉ噸鏂拌仛鍚?鍜屽鐞嗐€傝繖蹇呴』鍦ㄤ竴涓彲浠ヤ紤鐪犵殑涓婁笅鏂囦腑瀹屾垚銆?
### PHY 绠＄悊

```

  struct nfc_phy_ops {
	int (*write)(void *dev_id, struct sk_buff *skb);
	int (*enable)(void *dev_id);
	void (*disable)(void *dev_id);
  };

```

enable():
	鎵撳紑 phy锛堜笂鐢碉級锛屼娇鍏跺噯澶囧ソ浼犺緭鏁版嵁銆?disable():
	鍏抽棴 phy銆?write():
	鍚戣姱鐗囧彂閫佷竴涓暟鎹抚銆傛敞鎰忥紝涓轰簡璁?llc 绛夋洿楂樺眰鑳藉瀛樺偍璇ュ抚浠ヤ究閲嶅彂锛屾鍑芥暟涓嶅緱鏀瑰彉 skb銆?	瀹冧篃涓嶅緱杩斿洖姝ｆ暟缁撴灉锛堟垚鍔熻繑鍥?0锛屽け璐ヨ繑鍥炶礋鏁帮級銆?
鏉ヨ嚜鑺墖鐨勬暟鎹簲鐩存帴鍙戦€佸埌 nfc_hci_recv_frame()銆?
### LLC

CPU 涓庤姱鐗囦箣闂寸殑閫氫俊閫氬父闇€瑕佹煇绉嶉摼璺眰鍗忚銆傝繖浜涘崗璁闅旂涓虹敱 HCI 灞傜鐞嗙殑妯″潡銆傜洰鍓嶆湁涓や釜妯″潡锛?nop锛堝師濮嬩紶杈擄級鍜?shdlc銆?
```

  struct nfc_llc_ops {
	void *(*init) (struct nfc_hci_dev *hdev, xmit_to_drv_t xmit_to_drv,
		       rcv_to_hci_t rcv_to_hci, int tx_headroom,
		       int tx_tailroom, int *rx_headroom, int *rx_tailroom,
		       llc_failure_t llc_failure);
	void (*deinit) (struct nfc_llc *llc);
	int (*start) (struct nfc_llc *llc);
	int (*stop) (struct nfc_llc *llc);
	void (*rcv_from_drv) (struct nfc_llc *llc, struct sk_buff *skb);
	int (*xmit_from_hci) (struct nfc_llc *llc, struct sk_buff *skb);
  };

```

init():
	鍒嗛厤骞跺垵濮嬪寲浣犵殑绉佹湁瀛樺偍銆?deinit():
	娓呯悊銆?start():
	寤虹珛閫昏緫杩炴帴銆?stop():
	缁堟閫昏緫杩炴帴銆?rcv_from_drv():
	澶勭悊鏉ヨ嚜鑺墖銆佸彂寰€ HCI 鐨勬暟鎹€?xmit_from_hci():
	澶勭悊鐢?HCI 鍙戦€併€佸彂寰€鑺墖鐨勬暟鎹€?
llc 蹇呴』鍦ㄤ娇鐢ㄥ墠娉ㄥ唽鍒?nfc銆傞€氳繃浠ヤ笅鏂瑰紡瀹屾垚锛?
```

	nfc_llc_register(const char *name, const struct nfc_llc_ops *ops);

```

鍐嶆娉ㄦ剰锛宭lc 涓嶅鐞嗙墿鐞嗛摼璺€傚洜姝わ紝瀵逛簬浠讳綍缁欏畾鐨勮姱鐗囬┍鍔紝寰堝鏄撳皢浠讳綍鐗╃悊閾捐矾涓庝换浣?llc 娣峰悎銆?
### 鍖呭惈鐨勯┍鍔?

鍖呭惈涓€涓熀浜?HCI 鐨?NXP PN544 椹卞姩锛岄€氳繃 I2C 鎬荤嚎杩炴帴锛屽苟浣跨敤 shdlc銆?
### 鎵ц涓婁笅鏂?
鎵ц涓婁笅鏂囧涓嬶細
- IRQ 澶勭悊绋嬪簭锛圛RQH锛夛細
  蹇€燂紝涓嶈兘浼戠湢銆傚皢浼犲叆甯у彂閫佸埌 HCI锛屽湪閭ｉ噷瀹冧滑琚紶閫掔粰褰撳墠鐨?llc銆傚湪浣跨敤 shdlc 鐨勬儏鍐典笅锛岃甯?  琚帓鍏?shdlc rx 闃熷垪銆?
- SHDLC 鐘舵€佹満宸ヤ綔绾跨▼锛圫MW锛?
  浠呭湪浣跨敤 llc_shdlc 鏃讹細澶勭悊 shdlc rx 鍜?tx 闃熷垪銆?
  鍒嗗彂 HCI 鍛戒护鍝嶅簲銆?
- HCI Tx 鍛戒护宸ヤ綔绾跨▼锛圡SGTXWQ锛?
  涓茶鍖?HCI 鍛戒护鐨勬墽琛屻€?
  鍦ㄥ搷搴旇秴鏃舵椂瀹屾垚鎵ц銆?
- HCI Rx 宸ヤ綔绾跨▼锛圡SGRXWQ锛?
  鍒嗗彂浼犲叆鐨?HCI 鍛戒护鎴栦簨浠躲€?
- 鏉ヨ嚜鐢ㄦ埛绌洪棿璋冪敤鐨勭郴缁熻皟鐢ㄤ笂涓嬫枃锛圫YSCALL锛?
  HCI 涓粠 NFC 鏍稿績璋冪敤鐨勪换浣曞叆鍙ｇ偣銆?
### 鎵ц HCI 鍛戒护鐨勫伐浣滄祦锛堜娇鐢?shdlc锛?
鎵ц涓€涓?HCI 鍛戒护鍙互寰堝鏄撳湴閫氳繃浠ヤ笅鏂瑰紡鍚屾鎵ц锛?
```

  int nfc_hci_send_cmd (struct nfc_hci_dev *hdev, u8 gate, u8 cmd,
			const u8 *param, size_t param_len, struct sk_buff **skb)

```

璇?API 蹇呴』浠庝竴涓彲浠ヤ紤鐪犵殑涓婁笅鏂囦腑璋冪敤銆傚ぇ澶氭暟鎯呭喌涓嬶紝杩欏皢鏄郴缁熻皟鐢ㄤ笂涓嬫枃銆俿kb 灏嗚繑鍥炲湪鍝嶅簲涓?鎺ユ敹鍒扮殑缁撴灉銆?
鍦ㄥ唴閮紝鎵ц鏄紓姝ョ殑銆傛墍浠ユ API 鎵€鍋氱殑鍙槸灏?HCI 鍛戒护鍏ラ槦锛屽湪鏍堜笂寤虹珛涓€涓湰鍦扮瓑寰呴槦鍒楋紝骞?wait_event() 绛夊緟瀹屾垚銆傝绛夊緟涓嶅彲涓柇锛屽洜涓烘棤璁哄浣曢兘淇濊瘉鍛戒护浼氬湪鏌愪釜杈冪煭鐨勮秴鏃跺悗瀹屾垚銆?
MSGTXWQ 涓婁笅鏂囬殢鍚庤璋冨害骞惰皟鐢?nfc_hci_msg_tx_work()銆傛鍑芥暟灏嗗嚭闃熶笅涓€涓寕璧风殑鍛戒护锛屽苟灏嗗叾 HCP
鍒嗙墖鍙戦€佸埌鎭板ソ鏄?shdlc 鐨勪笅涓€灞傘€傜劧鍚庡畠灏嗗惎鍔ㄤ竴涓畾鏃跺櫒锛屼互渚垮湪娌℃湁鍝嶅簲鍒拌揪鏃朵互瓒呮椂閿欒瀹屾垚鍛戒护銆?
SMW 涓婁笅鏂囪璋冨害骞惰皟鐢?nfc_shdlc_sm_work()銆傛鍑芥暟澶勭悊 shdlc 甯х殑鏀跺彂銆傚畠浣跨敤椹卞姩 xmit 鍙戦€佸抚锛?骞朵粠椹卞姩 IRQ 澶勭悊绋嬪簭濉厖鐨?skb 闃熷垪涓帴鏀朵紶鍏ュ抚銆係HDLC I锛堜俊鎭級甯х殑鏈夋晥璐熻浇鏄?HCP 鍒嗙墖銆傚畠浠?琚仛鍚堜互褰㈡垚瀹屾暣鐨?HCI 甯э紝鍙互鏄搷搴斻€佸懡浠ゆ垨浜嬩欢銆?
HCI 鍝嶅簲浠庢涓婁笅鏂囩珛鍗冲垎鍙戜互瑙ｉ櫎绛夊緟涓殑鍛戒护鎵ц銆傚搷搴斿鐞嗘秹鍙婅皟鐢ㄧ敱 nfc_hci_msg_tx_work() 鍦?鍙戦€佸懡浠ゆ椂鎻愪緵鐨勫畬鎴愬洖璋冦€傚畬鎴愬洖璋冮殢鍚庡敜閱掔郴缁熻皟鐢ㄤ笂涓嬫枃銆?
```

  static int nfc_hci_execute_cmd_async(struct nfc_hci_dev *hdev, u8 pipe, u8 cmd,
				       const u8 *param, size_t param_len,
				       data_exchange_cb_t cb, void *cb_context)

```

宸ヤ綔娴佺浉鍚岋紝鍙槸 API 璋冪敤绔嬪嵆杩斿洖锛屽苟涓斿洖璋冨皢浠?SMW 涓婁笅鏂囧甫涓婄粨鏋滆璋冪敤銆?
### 鎺ユ敹 HCI 浜嬩欢鎴栧懡浠ょ殑宸ヤ綔娴?
HCI 鍛戒护鎴栦簨浠朵笉浠?SMW 涓婁笅鏂囧垎鍙戙€傜浉鍙嶏紝瀹冧滑琚帓鍏?HCI rx_queue锛屽苟灏嗕粠 HCI rx 宸ヤ綔绾跨▼涓婁笅鏂?锛圡SGRXWQ锛夊垎鍙戙€傝繖鏍峰仛鏄负浜嗗厑璁?cmd 鎴栦簨浠跺鐞嗙▼搴忎篃鎵ц鍏朵粬鍛戒护锛堜緥濡傦紝澶勭悊鏉ヨ嚜 PN544 鐨?NFC_HCI_EVT_TARGET_DISCOVERED 浜嬩欢闇€瑕佸悜 reader A 闂ㄥ彂鍑?ANY_GET_PARAMETER 浠ヨ幏鍙栧叧浜庡凡鍙戠幇
鐩爣鐨勪俊鎭級銆?
閫氬父锛屾绫讳簨浠跺皢浠?MSGRXWQ 涓婁笅鏂囦紶鎾埌 NFC 鏍稿績銆?
### 閿欒绠＄悊

涓?NFC 鏍稿績璇锋眰鎵ц鍚屾鍙戠敓鐨勯敊璇紝绠€鍗曞湴浣滀负璇锋眰鐨勬墽琛岀粨鏋滆繑鍥炪€傝繖浜涘緢瀹规槗澶勭悊銆?
寮傛鍙戠敓鐨勯敊璇紙渚嬪锛屽湪鍚庡彴鍗忚澶勭悊绾跨▼涓級蹇呴』琚姤鍛婏紝浠ヤ究涓婂眰涓嶄細鍦ㄤ笅闈㈠嚭浜嗛棶棰樼殑鎯呭喌涓嬩粛钂欏湪
榧撻噷锛屽苟鐭ラ亾棰勬湡鐨勪簨浠跺緢鍙兘姘歌繙涓嶄細鍙戠敓銆傝繖浜涢敊璇殑澶勭悊濡備笅锛?
- 椹卞姩锛坧n544锛夋湭鑳介€掗€佷紶鍏ュ抚锛氬畠瀛樺偍閿欒锛屼娇寰椾换浣曞悗缁椹卞姩鐨勮皟鐢ㄩ兘浼氬鑷存閿欒銆傜劧鍚庡畠璋冪敤
  鏍囧噯鐨?nfc_shdlc_recv_frame() 骞朵紶鍏?NULL 鍙傛暟锛屼互鍚戞洿涓婂眰鎶ュ憡闂銆俿hdlc 瀛樺偍涓€涓?EREMOTEIO
  绮樻粸鐘舵€侊紝杩欏皢渚濇瑙﹀彂 SMW 鍚戜笂鎶ュ憡銆?
- SMW 鏈川涓婃槸涓€涓鐞嗕紶鍏ュ拰浼犲嚭 shdlc 甯х殑鍚庡彴绾跨▼銆傛绾跨▼杩樹細妫€鏌?shdlc 绮樻粸鐘舵€侊紝骞跺湪鍙戠幇鐢变簬
  shdlc 鎴栧叾涓嬪眰涓彂鐢熺殑涓嶅彲鎭㈠閿欒鑰屾棤娉曞啀杩愯鏃讹紝鍚?HCI 鎶ュ憡銆?
- HCI锛氬鏋滃彂鐢熷唴閮?HCI 閿欒锛堝抚涓㈠け锛夛紝鎴?HCI 浠庝笅灞傛敹鍒伴敊璇紝HCI 瑕佷箞浠ヨ閿欒瀹屾垚褰撳墠姝ｅ湪鎵ц鐨?  鍛戒护锛岃涔堝湪娌℃湁鍛戒护鎵ц鏃剁洿鎺ラ€氱煡 NFC 鏍稿績銆?
- NFC 鏍稿績锛氬綋 NFC 鏍稿績浠庝笅灞傛敹鍒伴敊璇€氱煡涓旇疆璇㈠浜庢椿鍔ㄧ姸鎬佹椂锛屽畠灏嗗悜鐢ㄦ埛绌洪棿鍙戦€佷竴涓甫鏈夌┖鏍囩
  鍒楄〃鐨勬爣绛惧彂鐜颁簨浠讹紝璁╃敤鎴风┖闂寸煡閬撹疆璇㈡搷浣滃皢姘歌繙鏃犳硶妫€娴嬪埌鏍囩銆傚鏋滆疆璇笉娲诲姩涓旈敊璇槸绮樻粸鐨勶紝
  涓嬪眰灏嗗湪涓嬫璋冪敤鏃惰繑鍥炲畠銆?