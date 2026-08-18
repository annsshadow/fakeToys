## 閫氱敤闂瓨瀛樺偍锛圲niversal Flash Storage锛?

   1. 姒傝堪锛圤verview锛?   2. UFS 鏋舵瀯姒傝堪锛圲FS Architecture Overview锛?     2.1 搴旂敤灞傦紙Application Layer锛?     2.2 UFS 浼犺緭鍗忚锛圲TP锛夊眰
     2.3 UFS 浜掕繛锛圲IC锛夊眰
   3. UFSHCD 姒傝堪
     3.1 UFS 鎺у埗鍣ㄥ垵濮嬪寲
     3.2 UTP 浼犺緭璇锋眰
     3.3 UFS 閿欒澶勭悊
     3.4 SCSI 閿欒澶勭悊
   4. BSG 鏀寔
   5. UFS 鍙傝€冩椂閽熼鐜囬厤缃?

## 1. 姒傝堪锛圤verview锛?

閫氱敤闂瓨瀛樺偍锛圲FS锛夋槸閽堝闂瓨璁惧鐨勫瓨鍌ㄨ鑼冦€傚畠鏃ㄥ湪涓烘櫤鑳芥墜鏈哄拰骞虫澘鐢佃剳绛夌Щ鍔ㄨ澶囦腑鍩轰簬宓屽叆寮忎笌鍙Щ鍔ㄩ棯瀛樼殑瀛樺偍鎻愪緵缁熶竴鐨勫瓨鍌ㄦ帴鍙ｃ€傝瑙勮寖鐢?JEDEC 鍥烘€佹妧鏈崗浼氬畾涔夈€俇FS 鍩轰簬 MIPI M-PHY 鐗╃悊灞傛爣鍑嗐€俇FS 浣跨敤 MIPI M-PHY 浣滀负鐗╃悊灞傦紝浣跨敤 MIPI Unipro 浣滀负閾捐矾灞傘€?
UFS 鐨勪富瑕佺洰鏍囨槸鎻愪緵锛?
 - 浼樺寲鐨勬€ц兘锛?
   UFS 1.0 鍜?1.1 鐗堟湰鐨勭洰鏍囨€ц兘濡備笅锛?
   - 蹇呴』鏀寔 Gear1锛堥€熺巼 A锛?248Mbps锛岄€熺巼 B锛?457.6Mbps锛?   - 鍙€夋敮鎸?Gear2锛堥€熺巼 A锛?496Mbps锛岄€熺巼 B锛?915.2Mbps锛?
   鏈潵鐨勬爣鍑嗙増鏈紝

   - Gear3锛堥€熺巼 A锛?992Mbps锛岄€熺巼 B锛?830.4Mbps锛?
 - 浣庡姛鑰? - 楂橀殢鏈?IOPS 鍜屼綆寤惰繜


## 2. UFS 鏋舵瀯姒傝堪锛圲FS Architecture Overview锛?

UFS 鎷ユ湁涓€涓熀浜?SCSI SAM-5 鏋舵瀯妯″瀷鐨勫眰娆″寲閫氫俊鏋舵瀯銆?
UFS 閫氫俊鏋舵瀯鐢变互涓嬪眰娆＄粍鎴愩€?
### 2.1 搴旂敤灞傦紙Application Layer锛?

  搴旂敤灞傜敱 UFS 鍛戒护闆嗗眰锛圲CS锛夈€佷换鍔＄鐞嗗櫒鍜岃澶囩鐞嗗櫒绛夌粍鎴愩€俇FS 鎺ュ彛琚璁′负鍗忚鏃犲叧锛屼絾 SCSI 琚€変负 UFS 鍗忚灞?1.0 鍜?1.1 鐗堟湰鐨勫熀绾垮崗璁€?
  UFS 鏀寔鐢?SPC-4 鍜?SBC-3 瀹氫箟鐨?SCSI 鍛戒护鐨勪竴涓瓙闆嗐€?
  - UCS锛?     瀹冨鐞?UFS 瑙勮寖鏀寔鐨?SCSI 鍛戒护銆?  - 浠诲姟绠＄悊鍣紙Task manager锛夛細
     瀹冨鐞嗙敱 UFS 瀹氫箟鐨勩€佺敤浜庡懡浠ら槦鍒楁帶鍒剁殑浠诲姟绠＄悊鍔熻兘銆?  - 璁惧绠＄悊鍣紙Device manager锛夛細
     瀹冨鐞嗚澶囩骇鎿嶄綔鍜岃澶囬厤缃搷浣溿€傝澶囩骇鎿嶄綔涓昏娑夊強璁惧鐢垫簮绠＄悊鎿嶄綔浠ュ強瀵逛簰杩炲眰鐨勫懡浠ゃ€傝澶囩骇閰嶇疆娑夊強澶勭悊鐢ㄤ簬淇敼鍜屾绱㈣澶囬厤缃俊鎭殑鏌ヨ璇锋眰銆?
### 2.2 UFS 浼犺緭鍗忚锛圲TP锛夊眰


  UTP 灞傞€氳繃鏈嶅姟璁块棶鐐癸紙Service Access Points锛変负涓婂眰鎻愪緵鏈嶅姟銆俇TP 涓轰笂灞傚畾涔変簡 3 涓湇鍔¤闂偣銆?
  - UDM_SAP锛氳澶囩鐞嗗櫒鏈嶅姟璁块棶鐐癸紝鏆撮湶缁欒澶囩鐞嗗櫒鐢ㄤ簬璁惧绾ф搷浣溿€傝繖浜涜澶囩骇鎿嶄綔閫氳繃鏌ヨ璇锋眰瀹屾垚銆?  - UTP_CMD_SAP锛氬懡浠ゆ湇鍔¤闂偣锛屾毚闇茬粰 UFS 鍛戒护闆嗗眰锛圲CS锛変互浼犺緭鍛戒护銆?  - UTP_TM_SAP锛氫换鍔＄鐞嗘湇鍔¤闂偣锛屾毚闇茬粰浠诲姟绠＄悊鍣ㄤ互浼犺緭浠诲姟绠＄悊鍔熻兘銆?
  UTP 閫氳繃 UFS 鍗忚淇℃伅鍗曞厓锛圲PIU锛変紶杈撴秷鎭€?
### 2.3 UFS 浜掕繛锛圲IC锛夊眰


  UIC 鏄?UFS 灞傛鍖栨灦鏋勪腑鐨勬渶搴曞眰銆傚畠澶勭悊 UFS 涓绘満涓?UFS 璁惧涔嬮棿鐨勮繛鎺ャ€俇IC 鐢?MIPI UniPro 鍜?MIPI M-PHY 缁勬垚銆俇IC 涓轰笂灞傛彁渚?2 涓湇鍔¤闂偣锛?
  - UIC_SAP锛氬湪 UFS 涓绘満涓?UFS 璁惧涔嬮棿浼犺緭 UPIU銆?  - UIO_SAP锛氬悜 Unipro 灞傚彂鍑哄懡浠ゃ€?

## 3. UFSHCD 姒傝堪


UFS 涓绘満鎺у埗鍣ㄩ┍鍔紙UFSHCD锛夊熀浜?Linux SCSI 妗嗘灦銆俇FSHCD 鏄竴涓簳灞傝澶囬┍鍔紝鍏呭綋 SCSI 涓棿灞備笌鍩轰簬 PCIe 鐨?UFS 涓绘満鎺у埗鍣ㄤ箣闂寸殑鎺ュ彛銆?
褰撳墠 UFSHCD 瀹炵幇鏀寔浠ヤ笅鍔熻兘锛?
### 3.1 UFS 鎺у埗鍣ㄥ垵濮嬪寲


  鍒濆鍖栨ā鍧楀皢 UFS 涓绘満鎺у埗鍣ㄥ甫鍏ユ椿鍔ㄧ姸鎬侊紝骞跺噯澶囨帶鍒跺櫒鍦?UFSHCD 涓?UFS 璁惧涔嬮棿浼犺緭鍛戒护/鍝嶅簲銆?
### 3.2 UTP 浼犺緭璇锋眰


  浼犺緭璇锋眰澶勭悊妯″潡鎺ユ敹鏉ヨ嚜 SCSI 涓棿灞傜殑 SCSI 鍛戒护锛屾瀯閫?UPIU 骞跺皢鍏跺彂閫佺粰 UFS 涓绘満鎺у埗鍣ㄣ€傚悓鏃讹紝璇ユā鍧楀皢浠?UPIU 褰㈠紡浠?UFS 涓绘満鎺у埗鍣ㄦ帴鏀跺埌鐨勫搷搴旇繘琛岃В鐮侊紝骞跺皢鍛戒护鐘舵€侀€氱煡 SCSI 涓棿灞傘€?
### 3.3 UFS 閿欒澶勭悊


  閿欒澶勭悊妯″潡澶勭悊涓绘満鎺у埗鍣ㄨ嚧鍛介敊璇€佽澶囪嚧鍛介敊璇互鍙婁笌 UIC 浜掕繛灞傜浉鍏崇殑閿欒銆?
### 3.4 SCSI 閿欒澶勭悊


  杩欓€氳繃娉ㄥ唽鍒?SCSI 涓棿灞傜殑 UFSHCD SCSI 閿欒澶勭悊渚嬬▼瀹屾垚銆傜敱 SCSI 涓棿灞傚彂鍑虹殑涓€浜涢敊璇鐞嗗懡浠ょず渚嬪寘鎷腑姝换鍔★紙Abort task锛夈€丩UN 澶嶄綅鍜屼富鏈哄浣嶃€傜敤浜庢墽琛岃繖浜涗换鍔＄殑 UFSHCD 渚嬬▼閫氳繃 .eh_abort_handler銆?eh_device_reset_handler 鍜?.eh_host_reset_handler 娉ㄥ唽鍒?SCSI 涓棿灞傘€?
鍦ㄦ湰鐗堟湰鐨?UFSHCD 涓紝鏌ヨ璇锋眰鍜岀數婧愮鐞嗗姛鑳藉皻鏈疄鐜般€?
## 4. BSG 鏀寔


璇ヤ紶杈撻┍鍔ㄦ敮鎸佷笌 UFS 璁惧浜ゆ崲 UFS 鍗忚淇℃伅鍗曞厓锛圲PIU锛夈€傞€氬父锛岀敤鎴风┖闂翠細鍒嗛厤 struct ufs_bsg_request 鍜?struct ufs_bsg_reply锛堣 ufs_bsg.h锛夊垎鍒綔涓?request_upiu 鍜?reply_upiu銆傚～鍐欒繖浜?UPIU 搴旂鍚?JEDEC 瑙勮寖 UFS2.1 绗?10.7 鑺傘€?**Caveat emptor锛堜拱鑰呰嚜璐燂級**锛氶┍鍔ㄤ笉鍐嶅仛杩涗竴姝ョ殑杈撳叆鏍￠獙锛岃€屾槸鎸夊師鏍峰皢 UPIU 鍙戦€佺粰璁惧銆傚湪 /dev/ufs-bsg 鎵撳紑 bsg 璁惧锛屽苟

```
	io_hdr_v4.guard = 'Q';
	io_hdr_v4.protocol = BSG_PROTOCOL_SCSI;
	io_hdr_v4.subprotocol = BSG_SUB_PROTOCOL_SCSI_TRANSPORT;
	io_hdr_v4.response = (__u64)reply_upiu;
	io_hdr_v4.max_response_len = reply_len;
	io_hdr_v4.request_len = request_len;
	io_hdr_v4.request = (__u64)request_upiu;
	if (dir == SG_DXFER_TO_DEV) {
		io_hdr_v4.dout_xfer_len = (uint32_t)byte_cnt;
		io_hdr_v4.dout_xferp = (uintptr_t)(__u64)buff;
	} else {
		io_hdr_v4.din_xfer_len = (uint32_t)byte_cnt;
		io_hdr_v4.din_xferp = (uintptr_t)(__u64)buff;
	}
```

濡傛灉浣犲笇鏈涜鍙栨垨鍐欏叆鎻忚堪绗︼紝璇蜂娇鐢?sg_io_v4 鐩稿簲鐨?xferp銆?
涓?ufs-bsg 绔偣浜や簰骞朵娇鐢ㄥ叾鍩轰簬 UPIU 鍗忚鐨勭敤鎴风┖闂村伐鍏蜂綅浜庯細

	https://github.com/westerndigitalcorporation/ufs-tool

鏈夊叧璇ュ伐鍏峰強鍏舵墍鏀寔鍔熻兘鐨勬洿璇︾粏淇℃伅锛岃鍙傝璇ュ伐鍏风殑 README銆?
UFS 瑙勮寖鍙湪浠ヤ笅浣嶇疆鎵惧埌锛?
- UFS - http://www.jedec.org/sites/default/files/docs/JESD220.pdf
- UFSHCI - http://www.jedec.org/sites/default/files/docs/JESD223.pdf

## 5. UFS 鍙傝€冩椂閽熼鐜囬厤缃?

璁惧鏍戯紙Devicetree锛夊彲浠ュ湪 UFS 鎺у埗鍣ㄨ妭鐐逛笅瀹氫箟涓€涓悕涓?"ref_clk" 鐨勬椂閽燂紝鐢ㄤ互鎸囧畾 UFS 瀛樺偍閮ㄤ欢鐨勬湡鏈涘弬鑰冩椂閽熼鐜囥€傚熀浜?ACPI 鐨勭郴缁熷彲浠ヤ娇鐢ㄥ悕涓?"ref-clk-freq" 鐨?ACPI 璁惧鐗瑰畾鏁版嵁锛圖evice-Specific Data锛夊睘鎬ф潵鎸囧畾棰戠巼銆備袱绉嶆柟寮忎笅锛岃鍊奸兘琚В閲婁负浠?Hz 涓哄崟浣嶇殑棰戠巼锛屽苟涓斿繀椤讳笌 UFS 瑙勮寖涓粰鍑虹殑鏌愪釜鍊煎尮閰嶃€俇FS 瀛愮郴缁熶細鍦ㄦ墽琛岄€氱敤鎺у埗鍣ㄥ垵濮嬪寲鏃跺皾璇曡鍙栬鍊笺€傚鏋滆鍊煎彲鐢紝UFS 瀛愮郴缁熷皢纭繚 UFS 瀛樺偍璁惧鐨?bRefClkFreq 灞炴€ц鐩稿簲璁剧疆锛屽苟鍦ㄤ笉鍖归厤鏃朵慨鏀瑰畠銆?