## Intel 闆嗘垚浼犳劅鍣ㄤ腑鏋紙ISH锛?


浼犳劅鍣ㄤ腑鏋紙sensor hub锛夎兘澶熷皢浼犳劅鍣ㄨ疆璇㈠拰绠楁硶澶勭悊鐨勫伐浣滃嵏杞藉埌涓€涓笓鐢ㄧ殑
浣庡姛鑰楀崗澶勭悊鍣ㄤ笂銆傝繖浣垮緱鏍稿績澶勭悊鍣ㄨ兘澶熸洿棰戠箒鍦拌繘鍏ヤ綆鍔熻€楁ā寮忥紝浠庤€屽欢闀?
鐢垫睜缁埅鏃堕棿銆?

鏈夎澶氫緵搴斿晢鎻愪緵绗﹀悎 HID Sensor 浣跨敤琛ㄧ殑澶栭儴浼犳劅鍣ㄤ腑鏋€傚畠浠彲瑙佷簬骞虫澘
鐢佃剳銆佷簩鍚堜竴鍙浆鎹㈢瑪璁版湰锛?-in-1 convertible laptops锛夊拰宓屽叆寮忎骇鍝佷腑銆?
Linux 鑷?Linux 3.9 璧峰氨鏀寔杩欎竴鐗规€с€?

Intel庐 浠?Cherry Trail 寮€濮嬶紝浣滀负 SoC 鐨勪竴閮ㄥ垎寮曞叆浜嗛泦鎴愪紶鎰熷櫒涓灑锛岀洰鍓?
宸插湪澶氫唬 CPU 灏佽涓婂緱鍒版敮鎸併€傚凡缁忔湁璁稿鍟嗕笟璁惧鎼浇浜嗛泦鎴愪紶鎰熷櫒涓灑锛圛SH锛?
鍑鸿揣銆傝繖浜?ISH 鍚屾牱绗﹀悎 HID 浼犳劅鍣ㄨ鑼冿紝浣嗗尯鍒湪浜庣敤浜庨€氫俊鐨勪紶杈撳崗璁€?
褰撳墠鐨勫閮ㄤ紶鎰熷櫒涓灑涓昏浣跨敤 HID over I2C 鎴?USB銆備絾 ISH 涓よ€呴兘涓嶄娇鐢紝
鏃笉鐢?I2C 涔熶笉鐢?USB銆?

## 姒傝堪


鐢ㄤ竴涓笌 usbhid 瀹炵幇鐩哥被姣旂殑鏂瑰紡鏉ヨ锛孖SH 閬靛惊绫讳技鐨勬ā鍨?
```

	-----------------		----------------------
	|    USB HID	|	-->	|    ISH HID	     |
	-----------------		----------------------
	-----------------		----------------------
	|  USB protocol	|	-->	|    ISH Transport   |
	-----------------		----------------------
	-----------------		----------------------
	|  EHCI/XHCI	|	-->	|    ISH IPC	     |
	-----------------		----------------------
	      PCI				 PCI
	-----------------		----------------------
	|Host controller|	-->	|    ISH processor   |
	-----------------		----------------------
	     USB Link
	-----------------		----------------------
	| USB End points|	-->	|    ISH Clients     |
	-----------------		----------------------

```
灏卞儚 USB 鍗忚鎻愪緵浜嗕竴绉嶇敤浜庤澶囨灇涓俱€侀摼璺鐞嗗拰鐢ㄦ埛鏁版嵁灏佽鐨勬柟娉曚竴鏍凤紝
ISH 涔熸彁渚涚被浼肩殑鏈嶅姟銆備絾瀹冮潪甯歌交閲忥紝涓撲负绠＄悊鍜屼笌鍥轰欢涓疄鐜扮殑 ISH 瀹㈡埛绔?
搴旂敤閫氫俊鑰岄噺韬畾鍒躲€?

ISH 鍏佽鍥轰欢涓墽琛屽涓紶鎰熷櫒绠＄悊搴旂敤銆傚鍚?USB 绔偣锛屾秷鎭彲浠ュ彂寰€鎴栨潵鑷?
涓€涓鎴风銆備綔涓烘灇涓捐繃绋嬬殑涓€閮ㄥ垎锛岃繖浜涘鎴风浼氳璇嗗埆鍑烘潵銆傝繖浜涘鎴风鍙互
鏄畝鍗曠殑 HID 浼犳劅鍣ㄥ簲鐢ㄣ€佷紶鎰熷櫒鏍″噯搴旂敤鎴栦紶鎰熷櫒鍥轰欢鏇存柊搴旂敤銆?

瀹炵幇妯″瀷鏄被浼肩殑锛屽鍚?USB 鎬荤嚎锛孖SH 浼犺緭涔熻瀹炵幇涓轰竴涓€荤嚎銆傚湪 ISH
澶勭悊鍣ㄤ腑鎵ц鐨勬瘡涓鎴风搴旂敤閮藉湪璇ユ€荤嚎涓婃敞鍐屼负涓€涓澶囥€傚皢姣忎釜璁惧缁戝畾
璧锋潵鐨勯┍鍔紙ISH HID 椹卞姩锛変細璇嗗埆璁惧绫诲瀷锛屽苟鍚?HID 鏍稿績娉ㄥ唽銆?

## ISH 瀹炵幇锛氭鍥?


```

	 ---------------------------
	|  User Space Applications  |
	 ---------------------------

  ----------------IIO ABI----------------
	 --------------------------
	|  IIO Sensor Drivers	  |
	 --------------------------
	 --------------------------
	|	 IIO core	  |
	 --------------------------
	 --------------------------
	|   HID Sensor Hub MFD	  |
	 --------------------------
	 --------------------------
	|       HID Core	  |
	 --------------------------
	 --------------------------
	|   HID over ISH Client   |
	 --------------------------
	 --------------------------
	|   ISH Transport (ISHTP) |
	 --------------------------
	 --------------------------
	|      IPC Drivers	  |
	 --------------------------
  OS
  ---------------- PCI -----------------
  Hardware + Firmware
	 ----------------------------
	| ISH Hardware/Firmware(FW) |
	 ----------------------------

```
## 涓婅堪鍚勬ā鍧椾腑鐨勯珮灞傚鐞?


### 纭欢鎺ュ彛


ISH 瀵逛富鏈烘毚闇蹭负鈥淣on-VGA 鏈垎绫?PCI 璁惧鈥濄€侾CI 鐨勪骇鍝佸拰鍘傚晢 ID 鍦ㄤ笉鍚?
浠ｅ鐞嗗櫒涔嬮棿浼氬彉鍖栥€傚洜姝ょ敤浜庢灇涓鹃┍鍔ㄧ殑婧愪唬鐮侀渶瑕佷竴浠ｄ竴浠ｅ湴鏇存柊銆?

### 澶勭悊鍣ㄩ棿閫氫俊锛圛PC锛夐┍鍔?


浣嶇疆锛歞rivers/hid/intel-ish-hid/ipc

IPC 娑堟伅浣跨敤鍐呭瓨鏄犲皠 I/O銆傚瘎瀛樺櫒瀹氫箟鍦?hw-ish-regs.h 涓€?

##### IPC/FW 娑堟伅绫诲瀷


鏈変袱绫绘秷鎭紝涓€绫荤敤浜庨摼璺鐞嗭紝鍙︿竴绫荤敤浜庝紶杈撳眰涔嬮棿鐨勬秷鎭€?

浼犺緭娑堟伅鐨勫彂閫佷笌鎺ユ敹锛圱X and RX锛?
.......................................

涓€缁勫唴瀛樻槧灏勫瘎瀛樺櫒鎻愪緵瀵瑰瀛楄妭娑堟伅鍙戦€佷笌鎺ユ敹锛堜緥濡?IPC_REG_ISH2HOST_MSG銆?
IPC_REG_HOST2ISH_MSG锛夌殑鏀寔銆侷PC 灞傜淮鎶ゅ唴閮ㄩ槦鍒椾互瀵规秷鎭帓搴忓苟鎸夐『搴?
鍙戦€佺粰鍥轰欢銆傝皟鐢ㄦ柟杩樺彲浠ラ€夋嫨鎬у湴娉ㄥ唽澶勭悊绋嬪簭浠ヨ幏鍙栧畬鎴愰€氱煡銆傚湪娑堟伅浼犻€掍腑
浣跨敤闂ㄩ搩锛坉oorbell锛夋満鍒舵潵瑙﹀彂涓绘満鍜屽鎴风鍥轰欢渚х殑澶勭悊銆傚綋 ISH 涓柇澶勭悊
绋嬪簭琚皟鐢ㄦ椂锛屼富鏈洪┍鍔ㄤ娇鐢?ISH2HOST 闂ㄩ搩瀵勫瓨鍣ㄦ潵纭畾璇ヤ腑鏂槸鍙戠粰 ISH 鐨勩€?

姣忎竴渚ф湁 32 涓?32 浣嶆秷鎭瘎瀛樺櫒鍜?1 涓?32 浣嶉棬閾冦€傞棬閾?
```

  Bits 0..6: fragment length (7 bits are used)
  Bits 10..13: encapsulated protocol
  Bits 16..19: management command (for IPC management protocol)
  Bit 31: doorbell trigger (signal H/W interrupt to the other side)
  Other bits are reserved, should be 0.

```

##### 浼犺緭灞傛帴鍙?


涓轰簡鎶借薄纭欢绾х殑 IPC 閫氫俊锛屾敞鍐屼簡涓€缁勫洖璋冿紙callback锛夈€備紶杈撳眰浣跨敤瀹冧滑鏉?
鍙戦€佸拰鎺ユ敹娑堟伅銆傛湁鍏冲洖璋冭鍙傝€?struct ishtp_hw_ops銆?

### ISH 浼犺緭灞?


浣嶇疆锛歞rivers/hid/intel-ish-hid/ishtp/

##### 閫氱敤浼犺緭灞?


浼犺緭灞傛槸涓€涓弻鍚戝崗璁紝瀹冨畾涔変簡锛?
- 涓€缁勭敤浜庡惎鍔ㄣ€佸仠姝€佽繛鎺ャ€佹柇寮€鍜屾祦鎺х殑鍛戒护
锛堣瑙?ishtp/hbm.h锛?
- 涓€绉嶇敤浜庨伩鍏嶇紦鍐插尯婧㈠嚭鐨勬祦鎺ф満鍒?

璇ュ崗璁被浼间簬浠ヤ笅鏂囨。涓弿杩扮殑鎬荤嚎娑堟伅锛?
http://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/dcmi-hi-1-0-spec.pdf
鈥淐hapter 7: Bus Message Layer鈥濄€?

##### 杩炴帴涓庢祦鎺ф満鍒?


姣忎釜 FW 瀹㈡埛绔拰姣忎釜鍗忚閮界敱 UUID 鏍囪瘑銆備负浜嗕笌鏌愪釜 FW 瀹㈡埛绔€氫俊锛屽繀椤?
浣跨敤 connect 璇锋眰鍜屽搷搴旀€荤嚎娑堟伅鏉ュ缓绔嬭繛鎺ャ€傚鏋滄垚鍔燂紝涓€瀵?
锛坔ost_client_id 鍜?fw_client_id锛夊皢鏍囪瘑璇ヨ繛鎺ャ€?

涓€鏃﹁繛鎺ュ缓绔嬶紝瀵圭瓑鏂瑰郊姝ょ嫭绔嬪湴鍙戦€佹祦鎺ф€荤嚎娑堟伅銆傛瘡涓绛夋柟鍙湁鍦ㄤ箣鍓?
鏀跺埌杩囨祦鎺т俊鐢紙flow-control credit锛夋椂鎵嶅彲浠ュ彂閫佹秷鎭€備竴鏃﹀畠鍙戦€佷簡涓€鏉?
娑堟伅锛屽湪鏀跺埌涓嬩竴涓祦鎺т俊鐢ㄤ箣鍓嶏紝瀹冮兘涓嶈兘鍐嶅彂閫佸彟涓€鏉℃秷鎭€?

浠讳竴鏂归兘鍙互鍙戦€?disconnect 璇锋眰鎬荤嚎娑堟伅鏉ョ粨鏉熼€氫俊銆傛澶栵紝濡傛灉鍙戠敓閲嶅ぇ鐨?
FW 閲嶇疆锛岄摼璺篃浼氳涓㈠純銆?

##### 瀵圭瓑鏁版嵁浼犺緭


瀵圭瓑锛圥eer to Peer锛夋暟鎹紶杈撳彲浠ュ湪浣跨敤鎴栦笉浣跨敤 DMA 鐨勬儏鍐典笅鍙戠敓銆傛牴鎹?
浼犳劅鍣ㄥ甫瀹介渶姹傦紝DMA 鍙互閫氳繃 intel_ishtp 涓嬬殑妯″潡鍙傛暟 ishtp_use_dma 鏉?
鍚敤銆?

姣忎竴渚э紙涓绘満鍜?FW锛夌嫭绔嬬鐞嗗叾 DMA 浼犺緭鍐呭瓨銆傚綋鏉ヨ嚜涓绘満鎴?FW 渚х殑鏌愪釜
ISHTP 瀹㈡埛绔兂瑕佸彂閫佹煇浜涘唴瀹规椂锛屽畠浼氱嫭绔嬪湴鍐冲畾鏄€氳繃 IPC 杩樻槸 DMA 鍙戦€侊紱
姣忔浼犺緭鐨勫喅瀹氶兘鏄嫭绔嬬殑銆傚彂閫佹柟鍦ㄦ秷鎭綅浜庣浉搴旂殑涓绘満缂撳啿鍖轰腑鏃跺彂閫?
DMA_XFER 娑堟伅锛堝彂閫佹椂涓轰富鏈哄鎴风 TX锛屾帴鏀舵椂涓?FW 瀹㈡埛绔?RX锛夈€侱MA 娑堟伅鐨?
鎺ユ敹鏂逛互 DMA_XFER_ACK 鍝嶅簲锛屽悜鍙戦€佹柟琛ㄦ槑璇ユ秷鎭殑鍐呭瓨鍖哄煙鍙互琚噸鐢ㄣ€?

DMA 鍒濆鍖栫敱涓绘満鍙戦€?DMA_ALLOC_NOTIFY 鎬荤嚎娑堟伅锛堝寘鍚?RX 缂撳啿鍖猴級寮€濮嬶紝FW
浠?DMA_ALLOC_NOTIFY_ACK 鍝嶅簲銆傞櫎浜?DMA 鍦板潃閫氫俊涔嬪锛岃搴忓垪杩樻鏌ヨ兘鍔涳細
濡傛灉涓绘満涓嶆敮鎸?DMA锛岄偅涔堝畠涓嶄細鍙戦€?DMA 鍒嗛厤锛屽洜姝?FW 鏃犳硶鍙戦€?DMA锛涘鏋?
FW 涓嶆敮鎸?DMA锛岄偅涔堝畠涓嶄細浠?DMA_ALLOC_NOTIFY_ACK 鍝嶅簲锛屽湪杩欑鎯呭喌涓嬩富鏈哄皢
涓嶄娇鐢?DMA 浼犺緭銆?

杩欓噷 ISH 鍏呭綋鎬荤嚎涓绘帶锛坆usmaster锛塂MA 鎺у埗鍣ㄣ€傚洜姝わ紝褰撲富鏈哄彂閫?DMA_XFER
鏃讹紝瀹冩槸璇锋眰鎵ц host->ISH 鐨?DMA 浼犺緭锛涘綋 FW 鍙戦€?DMA_XFER 鏃讹紝鎰忓懗鐫€瀹?
宸茬粡瀹屾垚浜?DMA锛屾秷鎭┗鐣欏湪涓绘満澶勩€傚洜姝わ紝DMA_XFER 鍜?DMA_XFER_ACK 鍏呭綋
鎵€鏈夋潈鎸囩ず鍣ㄣ€?

鍦ㄥ垵濮嬬姸鎬佷笅锛屾墍鏈変紶鍑哄唴瀛橀兘灞炰簬鍙戦€佹柟锛圱X 灞炰簬涓绘満锛孯X 灞炰簬 FW锛夛紝
DMA_XFER 灏嗗寘鍚?ISHTP 娑堟伅鐨勫尯鍩熺殑鎵€鏈夋潈杞Щ缁欐帴鏀舵柟锛孌MA_XFER_ACK 灏?
鎵€鏈夋潈杩旇繕缁欏彂閫佹柟銆傚彂閫佹柟鏃犻渶绛夊緟鍏堝墠鐨?DMA_XFER 琚‘璁わ紙ack锛夛紝鍙鍏?
鎷ユ湁鐨勫墿浣欒繛缁唴瀛樿冻澶燂紝灏卞彲浠ュ彂閫佸彟涓€鏉℃秷鎭€傚師鍒欎笂锛屽彲浠ヤ竴娆℃€у彂閫佸涓?
DMA_XFER 鍜?DMA_XFER_ACK 娑堟伅锛堟渶澶氬埌 IPC MTU锛夛紝浠庤€屽厑璁歌繘琛屼腑鏂妭娴?
锛坕nterrupt throttling锛夈€傜洰鍓嶏紝濡傛灉 ISHTP 娑堟伅瓒呰繃 3 涓?IPC 鍒嗙墖锛坒ragment锛夛紝
ISH FW 鍐冲畾閫氳繃 DMA 鍙戦€侊紝鍚﹀垯閫氳繃 IPC 鍙戦€併€?

##### 鐜舰缂撳啿鍖?


褰撳鎴风鍙戣捣杩炴帴鏃讹紝浼氬垎閰嶄竴缁?RX 鍜?TX 缂撳啿鍖恒€傜幆鐨勫ぇ灏忓彲浠ョ敱瀹㈡埛绔寚瀹氥€?
HID 瀹㈡埛绔垎鍒皢 TX 鍜?RX 缂撳啿鍖鸿缃负 16 鍜?32銆傚湪瀹㈡埛绔殑鍙戦€佽姹備笂锛岃
鍙戦€佺殑鏁版嵁琚鍒跺埌鍏朵腑涓€涓彂閫佺幆褰㈢紦鍐插尯涓紝骞跺畨鎺掍娇鐢ㄦ€荤嚎娑堟伅鍗忚鍙戦€併€?
闇€瑕佽繖浜涚紦鍐插尯锛屽洜涓?FW 鍙兘灏氭湭澶勭悊涓婁竴鏉℃秷鎭紝骞朵笖鍙兘娌℃湁瓒冲鐨勬祦鎺?
淇＄敤鏉ュ彂閫併€傛帴鏀朵晶鍚屾牱濡傛锛屽洜姝ら渶瑕佹祦鎺с€?

##### 涓绘満鏋氫妇


涓绘満鏋氫妇鎬荤嚎鍛戒护鍏佽鍙戠幇 FW 涓瓨鍦ㄧ殑瀹㈡埛绔€傚彲浠ュ瓨鍦ㄥ涓紶鎰熷櫒瀹㈡埛绔互鍙?
鐢ㄤ簬鏍″噯鍔熻兘鐨勫鎴风銆?

涓轰簡绠€鍖栧疄鐜板苟鍏佽鐙珛鐨勯┍鍔ㄦ潵澶勭悊姣忎釜瀹㈡埛绔紝璇ヤ紶杈撳眰鍒╃敤浜?Linux 鎬荤嚎
椹卞姩妯″瀷銆傛瘡涓鎴风閮藉湪浼犺緭鎬荤嚎锛坕shtp 鎬荤嚎锛変笂娉ㄥ唽涓轰竴涓澶囥€?

鏋氫妇娑堟伅搴忓垪锛?

- 涓绘満鍙戦€?HOST_START_REQ_CMD锛岃〃鏄庝富鏈?ISHTP 灞傚凡灏辩华銆?
- FW 浠?HOST_START_RES_CMD 鍝嶅簲銆?
- 涓绘満鍙戦€?HOST_ENUM_REQ_CMD锛堟灇涓?FW 瀹㈡埛绔級銆?
- FW 浠?HOST_ENUM_RES_CMD 鍝嶅簲锛屽叾涓寘鍚彲鐢?FW 瀹㈡埛绔?ID 鐨勪綅鍥俱€?
- 瀵逛簬璇ヤ綅鍥句腑鎵惧埌鐨勬瘡涓?FW ID锛屼富鏈哄彂閫?
  HOST_CLIENT_PROPERTIES_REQ_CMD銆?
- FW 浠?HOST_CLIENT_PROPERTIES_RES_CMD 鍝嶅簲銆傚睘鎬у寘鎷?UUID銆?
  ISHTP 娑堟伅鏈€澶уぇ灏忕瓑銆?
- 涓€鏃︿富鏈烘敹鍒版渶鍚庝竴涓鍙戠幇鐨勫鎴风鐨勫睘鎬э紝瀹冨氨璁や负 ISHTP 璁惧宸插畬鍏?
  鍔熻兘姝ｅ父锛堝苟鍒嗛厤 DMA 缂撳啿鍖猴級銆?

### HID over ISH 瀹㈡埛绔?


浣嶇疆锛歞rivers/hid/intel-ish-hid

ISHTP 瀹㈡埛绔┍鍔ㄨ礋璐ｏ細

- 鏋氫妇 FW ISH 瀹㈡埛绔笅鐨?HID 璁惧
- 鑾峰彇鎶ュ憡鎻忚堪绗︼紙Report descriptor锛?
- 浣滀负 LL 椹卞姩鍚?HID 鏍稿績娉ㄥ唽
- 澶勭悊 Get/Set 鐗规€ц姹?
- 鑾峰彇杈撳叆鎶ュ憡

### HID 浼犳劅鍣ㄤ腑鏋?MFD 涓?IIO 浼犳劅鍣ㄩ┍鍔?


杩欎簺椹卞姩涓殑鍔熻兘涓庡閮ㄤ紶鎰熷櫒涓灑鐩稿悓銆傝鍙傝€?
Documentation/hid/hid-sensor.rst 浠ヤ簡瑙?HID 浼犳劅鍣紝
Documentation/ABI/testing/sysfs-bus-iio 浠ヤ簡瑙?IIO 鍚戠敤鎴风┖闂寸殑 ABI銆?

### 绔埌绔?HID 浼犺緭鏃跺簭鍥?


```

  HID-ISH-CLN                    ISHTP                    IPC                             HW
          |                        |                       |                               |
          |                        |                       |-----WAKE UP------------------>|
          |                        |                       |                               |
          |                        |                       |-----HOST READY--------------->|
          |                        |                       |                               |
          |                        |<----MNG_RESET_NOTIFY_ACK----- |
          |                        |                       |                               |
          |                        |<----ISHTP_START------ |                               |
          |                        |                       |                               |
          |                        |<-----------------HOST_START_RES_CMD-------------------|
          |                        |                       |                               |
          |                        |------------------QUERY_SUBSCRIBER-------------------->|
          |                        |                       |                               |
          |                        |------------------HOST_ENUM_REQ_CMD------------------->|
          |                        |                       |                               |
          |                        |<-----------------HOST_ENUM_RES_CMD--------------------|
          |                        |                       |                               |
          |                        |------------------HOST_CLIENT_PROPERTIES_REQ_CMD------>|
          |                        |                       |                               |
          |                        |<-----------------HOST_CLIENT_PROPERTIES_RES_CMD-------|
          |       Create new device on in ishtp bus        |                               |
          |                        |                       |                               |
          |                        |------------------HOST_CLIENT_PROPERTIES_REQ_CMD------>|
          |                        |                       |                               |
          |                        |<-----------------HOST_CLIENT_PROPERTIES_RES_CMD-------|
          |       Create new device on in ishtp bus        |                               |
          |                        |                       |                               |
          |                        |--Repeat HOST_CLIENT_PROPERTIES_REQ_CMD-till last one--|
          |                        |                       |                               |
       probed()
          |----ishtp_cl_connect--->|----------------- CLIENT_CONNECT_REQ_CMD-------------->|
          |                        |                       |                               |
          |                        |<----------------CLIENT_CONNECT_RES_CMD----------------|
          |                        |                       |                               |
          |register event callback |                       |                               |
          |                        |                       |                               |
          |ishtp_cl_send(
          HOSTIF_DM_ENUM_DEVICES)  |----------fill ishtp_msg_hdr struct write to HW-----  >|
          |                        |                       |                               |
          |                        |<-----IRQ(IPC_PROTOCOL_ISHTP---|
          |                        |                       |                               |
          |<--ENUM_DEVICE RSP------|                       |                               |
          |                        |                       |                               |
  for each enumerated device
          |ishtp_cl_send(
          HOSTIF_GET_HID_DESCRIPTOR|----------fill ishtp_msg_hdr struct write to HW-----  >|
          |                        |                       |                               |
          ...Response
          |                        |                       |                               |
  for each enumerated device
          |ishtp_cl_send(
       HOSTIF_GET_REPORT_DESCRIPTOR|--------------fill ishtp_msg_hdr struct write to HW-- >|
          |                        |                       |                               |
          |                        |                       |                               |
   hid_allocate_device
          |                        |                       |                               |
   hid_add_device                  |                       |                               |


```

### 浠庝富鏈哄姞杞?ISH 鍥轰欢娴佺▼


浠?Lunar Lake 杩欎竴浠ｅ紑濮嬶紝ISH 鍥轰欢琚垝鍒嗕负涓や釜缁勪欢锛屼互鑾峰緱鏇村ソ鐨勭┖闂翠紭鍖?
鍜屾洿楂樼殑鐏垫椿鎬с€傝繖浜涚粍浠跺寘鎷竴涓泦鎴愬湪 BIOS 涓殑寮曞鍔犺浇绋嬪簭锛坆ootloader锛夛紝
浠ュ強涓€涓瓨鍌ㄥ湪鎿嶄綔绯荤粺鏂囦欢绯荤粺鍐呯殑涓诲浐浠讹紙main firmware锛夈€?

璇ヨ繃绋嬪伐浣滄柟寮忓涓嬶細

- 鏈€鍒濓紝ISHTP 椹卞姩鍚?ISH 寮曞鍔犺浇绋嬪簭鍙戦€佷竴涓懡浠?HOST_START_REQ_CMD銆?
  浣滀负鍝嶅簲锛屽紩瀵煎姞杞界▼搴忓彂鍥炰竴涓?HOST_START_RES_CMD銆傝鍝嶅簲鍖呭惈
  ISHTP_SUPPORT_CAP_LOADER 浣嶃€傞殢鍚庯紝ISHTP 椹卞姩妫€鏌ヨ浣嶆槸鍚﹁璁剧疆銆傚鏋滄槸锛?
  鍒欎粠涓绘満杩涜鐨勫浐浠跺姞杞借繃绋嬪紑濮嬨€?

- 鍦ㄦ杩囩▼涓紝ISHTP 椹卞姩棣栧厛璋冪敤 request_firmware() 鍑芥暟锛岀劧鍚庡彂閫佷竴涓?
  LOADER_CMD_XFER_QUERY 鍛戒护銆傚湪鏀跺埌鏉ヨ嚜寮曞鍔犺浇绋嬪簭鐨勫搷搴斿悗锛孖SHTP 椹卞姩
  鍙戦€佷竴涓?LOADER_CMD_XFER_FRAGMENT 鍛戒护銆傚湪鏀跺埌鍙︿竴涓搷搴斿悗锛孖SHTP 椹卞姩
  鍙戦€佷竴涓?LOADER_CMD_START 鍛戒护銆傚紩瀵煎姞杞界▼搴忓仛鍑哄搷搴旓紝鐒跺悗璺宠浆鍒颁富鍥轰欢銆?

- 璇ヨ繃绋嬬粨鏉熷悗锛孖SHTP 椹卞姩璋冪敤 release_firmware() 鍑芥暟銆?

鏈夊叧鏇磋缁嗙殑淇℃伅锛岃鍙傞槄涓嬮潰鎻愪緵鐨勬祦绋嬫弿杩帮細

```

  +---------------+                                                    +-----------------+
  | ISHTP Driver  |                                                    | ISH Bootloader  |
  +---------------+                                                    +-----------------+
          |                                                                     |
          |~~~Send HOST_START_REQ_CMD~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send HOST_START_RES_CMD(Includes ISHTP_SUPPORT_CAP_LOADER bit)----|
          |                                                                     |
  ****************************************************************************************
  * if ISHTP_SUPPORT_CAP_LOADER bit is set                                               *
  ****************************************************************************************
          |                                                                     |
          |~~~start loading firmware from host process~~~+                      |
          |                                              |                      |
          |<---------------------------------------------+                      |
          |                                                                     |
  ---------------------------                                                   |
  | Call request_firmware() |                                                   |
  ---------------------------                                                   |
          |                                                                     |
          |~~~Send LOADER_CMD_XFER_QUERY~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |
          |~~~Send LOADER_CMD_XFER_FRAGMENT~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |
          |~~~Send LOADER_CMD_START~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>|
          |                                                                     |
          |<--Send response-----------------------------------------------------|
          |                                                                     |~~~Jump to Main Firmware~~~+
          |                                                                     |                           |
          |                                                                     |<--------------------------+
          |                                                                     |
  ---------------------------                                                   |
  | Call release_firmware() |                                                   |
  ---------------------------                                                   |
          |                                                                     |
  ****************************************************************************************
  * end if                                                                               *
  ****************************************************************************************
          |                                                                     |
  +---------------+                                                    +-----------------+
  | ISHTP Driver  |                                                    | ISH Bootloader  |
  +---------------+                                                    +-----------------+

```

##### 渚涘簲鍟嗚嚜瀹氫箟鍥轰欢鍔犺浇


杩愯鍦?ISH 鍐呴儴鐨勫浐浠跺彲浠ョ敱 Intel 鎻愪緵锛屼篃鍙互鐢变緵搴斿晢浣跨敤 Intel 鎻愪緵鐨?
鍥轰欢寮€鍙戝浠讹紙FDK锛孎irmware Development Kit锛夊紑鍙戙€侷ntel 浼氬皢 Intel 鏋勫缓鐨?
鍥轰欢涓婃父鍒?`linux-firmware.git` 浠撳簱锛岃矾寰勪綅浜?`intel/ish/` 涓嬨€傚浜?
Lunar Lake 骞冲彴锛孖ntel 鏋勫缓鐨?ISH 鍥轰欢灏嗗懡鍚嶄负 `ish_lnlm.bin`銆?

甯屾湜灏嗗叾鑷畾涔夊浐浠朵笂娓哥殑渚涘簲鍟嗗簲閬靛惊浠ヤ笅鍛藉悕鍏跺浐浠舵枃浠剁殑鍑嗗垯锛?

- 鍥轰欢鏂囦欢鍚嶅簲浣跨敤浠ヤ笅妯″紡涔嬩竴锛?

  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_SKU_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}.bin`
  - `ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}.bin`

- `${intel_plat_gen}` 琛ㄧず Intel 骞冲彴浠ｆ锛堜緥濡?`lnlm` 浠ｈ〃 Lunar Lake锛夛紝
  涓旈暱搴︿笉寰楄秴杩?8 涓瓧绗︺€?
- `${SYS_VENDOR_CRC32}` 鏄潵鑷?DMI 瀛楁 `DMI_SYS_VENDOR` 鐨?`sys_vendor`
  鍊肩殑 CRC32 鏍￠獙鍜屻€?
- `${PRODUCT_FAMILY_CRC32}` 鏄潵鑷?DMI 瀛楁 `DMI_PRODUCT_FAMILY` 鐨?
  `product_family` 鍊肩殑 CRC32 鏍￠獙鍜屻€?
- `${PRODUCT_NAME_CRC32}` 鏄潵鑷?DMI 瀛楁 `DMI_PRODUCT_NAME` 鐨?`product_name`
  鍊肩殑 CRC32 鏍￠獙鍜屻€?
- `${PRODUCT_SKU_CRC32}` 鏄潵鑷?DMI 瀛楁 `DMI_PRODUCT_SKU` 鐨?`product_sku`
  鍊肩殑 CRC32 鏍￠獙鍜屻€?

鍦ㄧ郴缁熷惎鍔ㄦ湡闂达紝ISH Linux 椹卞姩灏嗗皾璇曟寜浠ヤ笅椤哄簭鍔犺浇鍥轰欢锛屼紭鍏堜娇鐢ㄥ尮閰嶆ā寮?
鏇寸簿纭殑鑷畾涔夊浐浠讹細

1. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
2. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_SKU_CRC32}.bin`
3. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}_${PRODUCT_NAME_CRC32}.bin`
4. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_FAMILY_CRC32}.bin`
5. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}_${PRODUCT_SKU_CRC32}.bin`
6. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_SKU_CRC32}.bin`
7. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}_${PRODUCT_NAME_CRC32}.bin`
8. `intel/ish/ish_${intel_plat_gen}_${SYS_VENDOR_CRC32}.bin`
9. `intel/ish/ish_${intel_plat_gen}.bin`

椹卞姩灏嗗姞杞界涓€涓尮閰嶇殑鍥轰欢骞惰烦杩囧叾浣欑殑銆傚鏋滄湭鎵惧埌鍖归厤鐨勫浐浠讹紝瀹冨皢鎸夌収
鎸囧畾鐨勯『搴忕户缁皾璇曚笅涓€绉嶆ā寮忋€傚鏋滄墍鏈夋悳绱㈤兘澶辫触锛屽皢鍔犺浇涓婇潰椤哄簭涓垪鍑虹殑
鏈€鍚庣殑榛樿 Intel 鍥轰欢銆?

### ISH 璋冭瘯


```

  echo 1 > /sys/kernel/tracing/events/intel_ish/enable
  cat /sys/kernel/tracing/trace

```

### ISH IIO sysfs 鍦?Lenovo ThinkPad Yoga 260 涓婄殑绀轰緥


```

  root@otcpl-ThinkPad-Yoga-260:~# tree -l /sys/bus/iio/devices/
  /sys/bus/iio/devices/
  鈹溾攢鈹€ iio:device0 -> ../../../devices/0044:8086:22D8.0001/HID-SENSOR-200073.9.auto/iio:device0
  鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  ...
  鈹偮犅?鈹溾攢鈹€ in_accel_hysteresis
  鈹偮犅?鈹溾攢鈹€ in_accel_offset
  鈹偮犅?鈹溾攢鈹€ in_accel_sampling_frequency
  鈹偮犅?鈹溾攢鈹€ in_accel_scale
  鈹偮犅?鈹溾攢鈹€ in_accel_x_raw
  鈹偮犅?鈹溾攢鈹€ in_accel_y_raw
  鈹偮犅?鈹溾攢鈹€ in_accel_z_raw
  鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_index
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_x_type
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_index
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_y_type
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_z_en
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_accel_z_index
  鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_accel_z_type
  ...
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ devices
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_intensity_both_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_intensity_both_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_magn_z_type
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_from_north_magnetic_tilt_comp_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_rot_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_magn_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_magn_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_anglvel_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ buffer
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ enable
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ length
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ watermark
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ dev
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_hysteresis
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_offset
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_sampling_frequency
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_scale
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_raw
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ scan_elements
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_x_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_y_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_en
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ in_anglvel_z_index
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ in_anglvel_z_type
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ trigger
  鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹偮犅?鈹斺攢鈹€ current_trigger
  ...
```
