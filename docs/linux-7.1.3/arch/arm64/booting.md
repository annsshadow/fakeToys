## 寮曞 AArch64 Linux

浣滆€咃細Will Deacon <will.deacon@arm.com>

鏃ユ湡锛?012 骞?9 鏈?7 鏃?
鏈枃妗ｅ熀浜?Russell King 鎾板啓鐨?ARM 寮曞鏂囨。锛岄€傜敤浜?AArch64 Linux 鍐呮牳鐨勬墍鏈夊叕寮€鐗堟湰銆?
AArch64 寮傚父妯″瀷鐢辫嫢骞插紓甯哥骇鍒紙EL0 - EL3锛夋瀯鎴愶紝鍏朵腑 EL0銆丒L1 鍜?EL2 鍚勮嚜鎷ユ湁涓€涓畨鍏ㄤ笌闈炲畨鍏ㄥ壇鏈€侲L2 鏄?hypervisor锛堣櫄鎷熸満鐩戣鍣級绾у埆锛孍L3 鏄渶楂樹紭鍏堢骇绾у埆锛屼笖浠呭瓨鍦ㄤ簬瀹夊叏妯″紡銆備袱鑰呭湪鏋舵瀯涓婂潎涓哄彲閫夐」銆?
鍦ㄦ湰鏂囨。涓紝鎴戜滑浣跨敤鏈 `boot loader`锛堝紩瀵煎姞杞界▼搴忥級鏉ユ硾鎸囧湪鎺у埗鏉冪Щ浜ょ粰 Linux 鍐呮牳涔嬪墠锛屽湪 CPU 涓婃墽琛岀殑鎵€鏈夎蒋浠躲€傝繖鍙兘鍖呮嫭瀹夊叏鐩戣鍣紙secure monitor锛夊拰 hypervisor 浠ｇ爜锛屼篃鍙兘浠呬粎鏄敤浜庡噯澶囨渶灏忓紩瀵肩幆澧冪殑灏戞暟鍑犳潯鎸囦护銆?
鏈川涓婏紝寮曞鍔犺浇绋嬪簭鑷冲皯搴旀彁渚涗互涓嬪唴瀹癸細

1. 寤虹珛骞跺垵濮嬪寲 RAM
2. 寤虹珛璁惧鏍戯紙device tree锛?3. 瑙ｅ帇鍐呮牳闀滃儚
4. 璋冪敤鍐呮牳闀滃儚

### 1. 寤虹珛骞跺垵濮嬪寲 RAM

瑕佹眰锛氬己鍒讹紙MANDATORY锛?
寮曞鍔犺浇绋嬪簭搴斿綋鎵惧埌骞跺垵濮嬪寲鍐呮牳鍦ㄧ郴缁熶腑鐢ㄤ簬瀛樺偍鏄撳け鎬ф暟鎹殑鎵€鏈?RAM銆傚畠浠ヤ笌鏈哄櫒鐩稿叧鐨勬柟寮忓畬鎴愯繖椤瑰伐浣溿€傦紙瀹冨彲浠ラ噰鐢ㄥ唴閮ㄧ畻娉曡嚜鍔ㄥ畾浣嶅苟娴嬬畻鎵€鏈?RAM 鐨勫ぇ灏忥紝涔熷彲浠ュ埄鐢ㄦ満鍣ㄤ腑 RAM 鐨勭浉鍏充俊鎭紝鎴栭噰鐢ㄥ紩瀵煎姞杞界▼搴忚璁¤€呰涓哄悎閫傜殑浠讳綍鍏朵粬鏂瑰紡銆傦級

瀵逛簬 Arm 鏈哄瘑璁＄畻鍩燂紙Confidential Compute Realms锛夛紝杩欏寘鎷‘淇濇墍鏈夊彈淇濇姢 RAM 鍏锋湁 "RAM" 鐘舵€佺殑 Realm IPA 鐘舵€侊紙RIPAS锛夈€?
### 2. 寤虹珛璁惧鏍?
瑕佹眰锛氬己鍒讹紙MANDATORY锛?
璁惧鏍?blob锛坉tb锛夊繀椤绘斁缃湪 8 瀛楄妭瀵归綈鐨勮竟鐣屼笂锛屼笖澶у皬涓嶅緱瓒呰繃 2 鍏嗗瓧鑺傘€傜敱浜?dtb 灏嗕互鏈€澶?2 鍏嗗瓧鑺傚ぇ灏忕殑鍧楄鏄犲皠涓哄彲缂撳瓨锛坈acheable锛夛紝鍥犳瀹冧笉鑳芥斁缃湪蹇呴』浣跨敤鐗瑰畾灞炴€ц繘琛屾槧灏勭殑浠讳綍 2M 鍖哄煙鍐呫€?
娉ㄦ剰锛歷4.2 涔嬪墠鐨勭増鏈繕瑕佹眰 DTB 鏀剧疆鍦ㄤ粠鍐呮牳 Image 涓嬫柟 text_offset 瀛楄妭澶勫紑濮嬬殑 512 MB 鍖哄煙鍐呫€?
### 3. 瑙ｅ帇鍐呮牳闀滃儚

瑕佹眰锛氬彲閫夛紙OPTIONAL锛?
AArch64 鍐呮牳鐩墠涓嶆彁渚涜В鍘嬬▼搴忥紝鍥犳濡傛灉浣跨敤鍘嬬缉鐨?Image 鐩爣锛堜緥濡?Image.gz锛夛紝鍒欓渶瑕佺敱寮曞鍔犺浇绋嬪簭鎵ц瑙ｅ帇锛坓zip 绛夛級銆傚浜庢湭瀹炵幇姝よ姹傜殑寮曞鍔犺浇绋嬪簭锛屽彲浣跨敤鏈帇缂╃殑 Image 鐩爣浣滀负鏇夸唬銆?
### 4. 璋冪敤鍐呮牳闀滃儚

瑕佹眰锛氬己鍒讹紙MANDATORY锛?
```

  u32 code0;			/* Executable code */
  u32 code1;			/* Executable code */
  u64 text_offset;		/* Image load offset, little endian */
  u64 image_size;		/* Effective Image size, little endian */
  u64 flags;			/* kernel flags, little endian */
  u64 res2	= 0;		/* reserved */
  u64 res3	= 0;		/* reserved */
  u64 res4	= 0;		/* reserved */
  u32 magic	= 0x644d5241;	/* Magic number, little endian, "ARM\x64" */
  u32 res5;			/* reserved (used for PE COFF offset) */


```
澶撮儴娉ㄨ锛?
- 鑷?v3.17 璧凤紝闄ら潪鍙︽湁璇存槑锛屾墍鏈夊瓧娈靛潎涓哄皬绔紙little endian锛夈€?
- code0/code1 璐熻矗璺宠浆鍒?stext銆?
- 閫氳繃 EFI 寮曞鏃讹紝code0/code1 鏈€鍒濅細琚烦杩囥€俽es5 鏄埌 PE 澶撮儴鐨勫亸绉婚噺锛孭E 澶撮儴鍖呭惈 EFI 鍏ュ彛鐐癸紙efi_stub_entry锛夈€傚綋 stub 瀹屾垚鍏跺伐浣滃悗锛屼細璺宠浆鍒?code0 浠ユ仮澶嶆甯稿紩瀵兼祦绋嬨€?
- 鍦?v3.17 涔嬪墠锛宼ext_offset 鐨勫瓧鑺傚簭鏈瑙勫畾銆傚湪杩欎簺鎯呭喌涓?image_size 涓洪浂锛宼ext_offset 涓哄唴鏍稿瓧鑺傚簭涓嬬殑 0x80000銆傚綋 image_size 闈為浂鏃讹紝image_size 涓哄皬绔紝蹇呴』琚伒瀹堛€傚綋 image_size 涓洪浂鏃讹紝鍙亣瀹?text_offset 涓?0x80000銆?
- flags 瀛楁锛堝湪 v3.17 涓紩鍏ワ級鏄竴涓皬绔?64 浣嶅瓧娈碉紝鏋勬垚濡備笅锛?
  ============= ===============================================================
  Bit 0		鍐呮牳瀛楄妭搴忋€侭E 涓?1锛孡E 涓?0銆?  Bit 1-2	鍐呮牳椤靛ぇ灏忋€?
   - 0 - 鏈寚瀹氥€?   - 1 - 4K
   - 2 - 16K
   - 3 - 64K
  Bit 3		鍐呮牳鐗╃悊鏀剧疆浣嶇疆

			0
			  2MB 瀵归綈鐨勫熀鍧€搴斿敖鍙兘鎺ヨ繎 DRAM 鐨勫熀鍧€锛屽洜涓哄叾涓嬫柟
			  鐨勫唴瀛樻棤娉曢€氳繃绾挎€ф槧灏勮闂?			1
			  2MB 瀵归綈鐨勫熀鍧€锛屼娇寰椾粠闀滃儚璧峰澶勫紑濮嬭鏁扮殑鎵€鏈?			  image_size 瀛楄妭閮戒綅浜庣墿鐞嗗唴瀛樼殑 48 浣嶅彲瀵诲潃鑼冨洿鍐?  Bits 4-63	淇濈暀銆?  ============= ===============================================================

- 褰?image_size 涓洪浂鏃讹紝寮曞鍔犺浇绋嬪簭搴斿皾璇曞湪鍐呮牳闀滃儚缁撴潫涔嬪悗锛屽敖鍙兘澶氬湴灏嗗唴瀛樹繚鐣欑粰鍐呮牳浣跨敤銆傛墍闇€绌洪棿澶у皬鍙栧喅浜庢墍閫夌壒鎬э紝瀹為檯涓婃病鏈変笂闄愩€?
Image 蹇呴』鏀剧疆鍦ㄤ笌绯荤粺 RAM 涓换鎰忎綅缃?2MB 瀵归綈鍩哄潃鐩歌窛 text_offset 瀛楄妭澶勶紝骞跺湪璇ュ琚皟鐢ㄣ€?MB 瀵归綈鍩哄潃涓庨暅鍍忚捣濮嬩綅缃箣闂寸殑鍖哄煙瀵瑰唴鏍告病鏈夌壒娈婃剰涔夛紝鍙敤浜庡叾浠栫敤閫斻€備粠闀滃儚璧峰澶勫紑濮嬭嚦灏?image_size 瀛楄妭蹇呴』鍙緵鍐呮牳浣跨敤銆?娉ㄦ剰锛歷4.6 涔嬪墠鐨勭増鏈棤娉曚娇鐢?Image 鐗╃悊鍋忕Щ涓嬫柟鐨勫唴瀛橈紝鍥犳寤鸿灏?Image 鏀剧疆寰楀敖鍙兘鎺ヨ繎绯荤粺 RAM 鐨勮捣濮嬩綅缃€?
濡傛灉鍦ㄥ紩瀵兼椂灏?initrd/initramfs 浼犻€掔粰鍐呮牳锛屽畠蹇呴』瀹屾暣浣嶄簬涓€涓?1 GB 瀵归綈銆佹渶澶?32 GB 澶у皬鐨勭墿鐞嗗唴瀛樼獥鍙ｅ唴锛屽苟涓旇绐楀彛涔熻瀹屾暣瑕嗙洊鍐呮牳 Image銆?
浠讳綍鎻忚堪缁欏唴鏍哥殑鍐呭瓨锛堝嵆浣挎槸闀滃儚璧峰浣嶇疆涓嬫柟鐨勫唴瀛橈級锛屽彧瑕佹湭琚爣璁颁负浠庡唴鏍镐繚鐣欙紙渚嬪閫氳繃璁惧鏍戜腑鐨?memreserve 鍖哄煙锛夛紝閮藉皢琚涓哄彲渚涘唴鏍镐娇鐢ㄣ€?
鍦ㄨ烦鍏ュ唴鏍镐箣鍓嶏紝蹇呴』婊¤冻浠ヤ笅鏉′欢锛?
- 浣挎墍鏈夊叿澶?DMA 鑳藉姏鐨勮澶囬潤榛橈紙quiesce锛夛紝浠ュ厤鍐呭瓨琚櫄鍋囩殑缃戠粶鏁版嵁鍖呮垨纾佺洏鏁版嵁鐮村潖銆傝繖灏嗕负浣犺妭鐪佸ぇ閲忚皟璇曟椂闂淬€?
- 涓?CPU 閫氱敤瀵勫瓨鍣ㄨ缃細

    - x0 = 绯荤粺 RAM 涓澶囨爲 blob锛坉tb锛夌殑鐗╃悊鍦板潃銆?    - x1 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?    - x2 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?    - x3 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?
- CPU 妯″紡

  鎵€鏈夊舰寮忕殑涓柇蹇呴』鍦?PSTATE.DAIF锛圖ebug銆丼Error銆両RQ 鍜?FIQ锛変腑琚睆钄姐€?  CPU 蹇呴』澶勪簬闈炲畨鍏ㄧ姸鎬侊紝瑕佷箞澶勪簬 EL2锛堜负璁块棶铏氭嫙鍖栨墿灞曪紝鎺ㄨ崘濡傛锛夛紝瑕佷箞澶勪簬 EL1銆?
- 缂撳瓨銆丮MU

  MMU 蹇呴』鍏抽棴銆?
  鎸囦护缂撳瓨鍙互寮€鍚垨鍏抽棴锛屽苟涓斾笉寰椾繚鐣欎笌宸插姞杞藉唴鏍搁暅鍍忓搴旂殑浠讳綍闄堟棫鏉＄洰銆?
  涓庡凡鍔犺浇鍐呮牳闀滃儚瀵瑰簲鐨勫湴鍧€鑼冨洿蹇呴』娓呮礂锛坈lean锛夊埌 PoC锛堢紦瀛樹竴鑷存€х偣锛夈€傚湪瀛樺湪绯荤粺缂撳瓨鎴栧叾浠栫紦瀛樺凡鍚敤鐨勪竴鑷翠富鎺у櫒鐨勬儏鍐典笅锛岃繖閫氬父闇€瑕侀€氳繃鎸?VA 杩涜缂撳瓨缁存姢锛岃€屼笉鏄泦鍚?璺紙set/way锛夋搷浣溿€?  閬靛惊鎸?VA 鏋舵瀯缂撳瓨缁存姢鎿嶄綔鐨勭郴缁熺紦瀛樺繀椤昏閰嶇疆锛屽苟鍙互鍚敤銆?  涓嶉伒寰寜 VA 鏋舵瀯缂撳瓨缁存姢鎿嶄綔鐨勭郴缁熺紦瀛橈紙涓嶆帹鑽愶級蹇呴』琚厤缃苟绂佺敤銆?
- 鏋舵瀯瀹氭椂鍣?
  CNTFRQ 蹇呴』琚紪绋嬩负瀹氭椂鍣ㄩ鐜囷紝涓?CNTVOFF 蹇呴』鍦ㄦ墍鏈?CPU 涓婄紪绋嬩负涓€鑷寸殑鍊笺€傚鏋滃湪 EL1 杩涘叆鍐呮牳锛屽垯 CNTHCTL_EL2 蹇呴』鍦ㄥ彲鐢ㄦ椂璁剧疆 EL1PCTEN锛坆it 0锛夈€?
- 涓€鑷存€э紙Coherency锛?
  鎵€鏈夊皢鐢卞唴鏍稿紩瀵肩殑 CPU 鍦ㄨ繘鍏ュ唴鏍告椂蹇呴』灞炰簬鍚屼竴涓竴鑷存€у煙銆傝繖鍙兘闇€瑕?IMPLEMENTATION DEFINED锛堝疄鐜板畾涔夛級鐨勫垵濮嬪寲锛屼互鍦ㄦ瘡涓?CPU 涓婂惎鐢ㄦ帴鏀剁淮鎶ゆ搷浣溿€?
- 绯荤粺瀵勫瓨鍣?
  鍦ㄥ唴鏍搁暅鍍忓皢杩涘叆鐨勫紓甯哥骇鍒強鍏朵互涓嬶紝鎵€鏈夊彲鍐欑殑鏋舵瀯绯荤粺瀵勫瓨鍣ㄥ繀椤荤敱鏇撮珮寮傚父绾у埆鐨勮蒋浠惰繘琛屽垵濮嬪寲锛屼互闃叉鍦?UNKNOWN锛堟湭鐭ワ級鐘舵€佷笅鎵ц銆?
  瀵逛簬鎵€鏈夌郴缁燂細
  - 濡傛灉 EL3 瀛樺湪锛?
    - 鍦ㄥ唴鏍告墽琛岀殑鎵€鏈?CPU 涓婏紝SCR_EL3.FIQ 蹇呴』鍏锋湁鐩稿悓鐨勫€笺€?    - 鍙鍐呮牳鍦ㄦ墽琛岋紝SCR_EL3.FIQ 鐨勫€煎氨蹇呴』涓庡紩瀵兼椂鐨勫€肩浉鍚屻€?
  - 濡傛灉 EL3 瀛樺湪涓斿唴鏍稿湪 EL2 杩涘叆锛?
    - SCR_EL3.HCE锛坆it 8锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬闇€鍦?v5 妯″紡涓嬩娇鐢ㄧ殑銆佸甫鏈?GICv5 涓柇鎺у埗鍣ㄧ殑绯荤粺锛?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
      - ICH_HFGRTR_EL2.ICC_PPI_ACTIVERn_EL1锛坆it 20锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_PPI_PRIORITYRn_EL1锛坆it 19锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_PPI_PENDRn_EL1锛坆it 18锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_PPI_ENABLERn_EL1锛坆it 17锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_PPI_HMRn_EL1锛坆it 16锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_IAFFIDR_EL1锛坆it 7锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_ICSR_EL1锛坆it 6锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_PCR_EL1锛坆it 5锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_HPPIR_EL1锛坆it 4锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_HAPR_EL1锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_CR0_EL1锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_IDRn_EL1锛坆it 1锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGRTR_EL2.ICC_APR_EL1锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b1銆?
      - ICH_HFGWTR_EL2.ICC_PPI_ACTIVERn_EL1锛坆it 20锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_PPI_PRIORITYRn_EL1锛坆it 19锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_PPI_PENDRn_EL1锛坆it 18锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_PPI_ENABLERn_EL1锛坆it 17锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_ICSR_EL1锛坆it 6锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_PCR_EL1锛坆it 5锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_CR0_EL1锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGWTR_EL2.ICC_APR_EL1锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b1銆?
      - ICH_HFGITR_EL2.GICRCDNMIA锛坆it 10锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICRCDIA锛坆it 9锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDDI锛坆it 8锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDEOI锛坆it 7锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDHM锛坆it 6锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDRCFG锛坆it 5锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDPEND锛坆it 4锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDAFF锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDPRI锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDDIS锛坆it 1锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICH_HFGITR_EL2.GICCDEN锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - DT 鎴?ACPI 琛ㄥ繀椤绘弿杩颁竴涓?GICv5 涓柇鎺у埗鍣ㄣ€?
  瀵逛簬闇€鍦?v3 妯″紡涓嬩娇鐢ㄧ殑銆佸甫鏈?GICv3 涓柇鎺у埗鍣ㄧ殑绯荤粺锛?  - 濡傛灉 EL3 瀛樺湪锛?
      - ICC_SRE_EL3.Enable锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICC_SRE_EL3.SRE锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b1銆?      - ICC_CTLR_EL3.PMHE锛坆it 6锛夊繀椤诲湪鍐呮牳鎵ц鐨勬墍鏈?CPU 涓婅缃负鐩稿悓鐨勫€硷紝骞朵笖蹇呴』鍦ㄥ唴鏍哥殑鏁翠釜鐢熷懡鍛ㄦ湡鍐呬繚鎸佹亽瀹氥€?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆锛?
      - ICC_SRE_EL2.Enable锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1
      - ICC_SRE_EL2.SRE锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - DT 鎴?ACPI 琛ㄥ繀椤绘弿杩颁竴涓?GICv3 涓柇鎺у埗鍣ㄣ€?
  瀵逛簬闇€鍦ㄥ吋瀹癸紙v2锛夋ā寮忎笅浣跨敤鐨勩€佸甫鏈?GICv3 涓柇鎺у埗鍣ㄧ殑绯荤粺锛?
  - 濡傛灉 EL3 瀛樺湪锛?
      ICC_SRE_EL3.SRE锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b0銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆锛?
      ICC_SRE_EL2.SRE锛坆it 0锛夊繀椤昏鍒濆鍖栦负 0b0銆?
  - DT 鎴?ACPI 琛ㄥ繀椤绘弿杩颁竴涓?GICv2 涓柇鎺у埗鍣ㄣ€?
  瀵逛簬甯︽湁鎸囬拡璁よ瘉锛坧ointer authentication锛夊姛鑳界殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SCR_EL3.APK锛坆it 16锛夊繀椤昏鍒濆鍖栦负 0b1
    - SCR_EL3.API锛坆it 17锛夊繀椤昏鍒濆鍖栦负 0b1

  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆锛?
    - HCR_EL2.APK锛坆it 40锛夊繀椤昏鍒濆鍖栦负 0b1
    - HCR_EL2.API锛坆it 41锛夊繀椤昏鍒濆鍖栦负 0b1

  瀵逛簬甯︽湁娲诲姩鐩戣鍣ㄥ崟鍏?v1锛圓MUv1锛夋墿灞曠殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - CPTR_EL3.TAM锛坆it 30锛夊繀椤昏鍒濆鍖栦负 0b0
    - CPTR_EL2.TAM锛坆it 30锛夊繀椤昏鍒濆鍖栦负 0b0
    - AMCNTENSET0_EL0 蹇呴』琚垵濮嬪寲涓?0b1111
    - AMCNTENSET1_EL0 蹇呴』琚垵濮嬪寲涓哄钩鍙扮浉鍏冲€硷紝瀵逛簬瀛樺湪鐨勬瘡涓緟鍔╄鏁板櫒锛屽湪鐩稿簲浣嶄笂璁剧疆 0b1銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆锛?
    - AMCNTENSET0_EL0 蹇呴』琚垵濮嬪寲涓?0b1111
    - AMCNTENSET1_EL0 蹇呴』琚垵濮嬪寲涓哄钩鍙扮浉鍏冲€硷紝瀵逛簬瀛樺湪鐨勬瘡涓緟鍔╄鏁板櫒锛屽湪鐩稿簲浣嶄笂璁剧疆 0b1銆?
  瀵逛簬甯︽湁缁嗙矑搴﹂櫡闃憋紙Fine Grained Traps锛孎EAT_FGT锛夋墿灞曠殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪涓斿唴鏍稿湪 EL2 杩涘叆锛?
    - SCR_EL3.FGTEn锛坆it 27锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁缁嗙矑搴﹂櫡闃?2锛團EAT_FGT2锛夋墿灞曠殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪涓斿唴鏍稿湪 EL2 杩涘叆锛?
    - SCR_EL3.FGTEn2锛坆it 59锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁 HCRX_EL2 鏀寔锛團EAT_HCX锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪涓斿唴鏍稿湪 EL2 杩涘叆锛?
    - SCR_EL3.HXEn锛坆it 38锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁楂樼骇 SIMD 鍜屾诞鐐规敮鎸佺殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - CPTR_EL3.TFP锛坆it 10锛夊繀椤昏鍒濆鍖栦负 0b0銆?
  - 濡傛灉 EL2 瀛樺湪涓斿唴鏍稿湪 EL1 杩涘叆锛?
    - CPTR_EL2.TFP锛坆it 10锛夊繀椤昏鍒濆鍖栦负 0b0銆?
  瀵逛簬甯︽湁鍙几缂╁悜閲忔墿灞曪紙Scalable Vector Extension锛孎EAT_SVE锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - CPTR_EL3.EZ锛坆it 8锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - ZCR_EL3.LEN 蹇呴』鍦ㄥ唴鏍告墽琛岀殑鎵€鏈?CPU 涓婂垵濮嬪寲涓虹浉鍚岀殑鍊笺€?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - CPTR_EL2.TZ锛坆it 8锛夊繀椤昏鍒濆鍖栦负 0b0銆?
    - CPTR_EL2.ZEN锛坆its 17:16锛夊繀椤昏鍒濆鍖栦负 0b11銆?
    - ZCR_EL2.LEN 蹇呴』鍒濆鍖栦负鍐呮牳灏嗘墽琛岀殑鎵€鏈?CPU 涓婄浉鍚岀殑鍊笺€?
  瀵逛簬甯︽湁鍙几缂╃煩闃垫墿灞曪紙Scalable Matrix Extension锛孎EAT_SME锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - CPTR_EL3.ESM锛坆it 12锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - SCR_EL3.EnTP2锛坆it 41锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - SMCR_EL3.LEN 蹇呴』鍒濆鍖栦负鍐呮牳灏嗘墽琛岀殑鎵€鏈?CPU 涓婄浉鍚岀殑鍊笺€?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - CPTR_EL2.TSM锛坆it 12锛夊繀椤昏鍒濆鍖栦负 0b0銆?
    - CPTR_EL2.SMEN锛坆its 25:24锛夊繀椤昏鍒濆鍖栦负 0b11銆?
    - SCTLR_EL2.EnTP2锛坆it 60锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - SMCR_EL2.LEN 蹇呴』鍒濆鍖栦负鍐呮牳灏嗘墽琛岀殑鎵€鏈?CPU 涓婄浉鍚岀殑鍊笺€?
    - HFGRTR_EL2.nTPIDR2_EL0锛坆it 55锛夊繀椤昏鍒濆鍖栦负 0b01銆?
    - HFGWTR_EL2.nTPIDR2_EL0锛坆it 55锛夊繀椤昏鍒濆鍖栦负 0b01銆?
    - HFGRTR_EL2.nSMPRI_EL1锛坆it 54锛夊繀椤昏鍒濆鍖栦负 0b01銆?
    - HFGWTR_EL2.nSMPRI_EL1锛坆it 54锛夊繀椤昏鍒濆鍖栦负 0b01銆?
  瀵逛簬甯︽湁鍙几缂╃煩闃垫墿灞?FA64 鐗规€э紙FEAT_SME_FA64锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SMCR_EL3.FA64锛坆it 31锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - SMCR_EL2.FA64锛坆it 31锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁鍐呭瓨鏍囪鎵╁睍鐗规€э紙FEAT_MTE2锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SCR_EL3.ATA锛坆it 26锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCR_EL2.ATA锛坆it 56锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁鍙几缂╃煩闃垫墿灞曠増鏈?2锛團EAT_SME2锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SMCR_EL3.EZT0锛坆it 30锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - SMCR_EL2.EZT0锛坆it 30锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁鍒嗘敮璁板綍缂撳啿鍖烘墿灞曪紙FEAT_BRBE锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - MDCR_EL3.SBRBE锛坆its 33:32锛夊繀椤昏鍒濆鍖栦负 0b01 鎴?0b11銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - BRBCR_EL2.CC锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - BRBCR_EL2.MPRED锛坆it 4锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HDFGRTR_EL2.nBRBDATA锛坆it 61锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGRTR_EL2.nBRBCTL  锛坆it 60锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGRTR_EL2.nBRBIDR  锛坆it 59锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HDFGWTR_EL2.nBRBDATA锛坆it 61锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGWTR_EL2.nBRBCTL  锛坆it 60锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGITR_EL2.nBRBIALL锛坆it 56锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HFGITR_EL2.nBRBINJ  锛坆it 55锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁鎬ц兘鐩戣鍣ㄦ墿灞曪紙FEAT_PMUv3p9锛夌殑 CPU锛?
 - 濡傛灉 EL3 瀛樺湪锛?
    - MDCR_EL3.EnPM2锛坆it 7锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HDFGRTR2_EL2.nPMICNTR_EL0锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGRTR2_EL2.nPMICFILTR_EL0锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGRTR2_EL2.nPMUACR_EL1锛坆it 4锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HDFGWTR2_EL2.nPMICNTR_EL0锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGWTR2_EL2.nPMICFILTR_EL0锛坆it 3锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGWTR2_EL2.nPMUACR_EL1锛坆it 4锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁 SPE 鏁版嵁婧愯繃婊わ紙FEAT_SPE_FDS锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - MDCR_EL3.EnPMS3锛坆it 42锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HDFGRTR2_EL2.nPMSDSFR_EL1锛坆it 19锛夊繀椤昏鍒濆鍖栦负 0b1銆?    - HDFGWTR2_EL2.nPMSDSFR_EL1锛坆it 19锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁鍐呭瓨澶嶅埗涓庡唴瀛樿缃寚浠わ紙FEAT_MOPS锛夌殑 CPU锛?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCRX_EL2.MSCEn锛坆it 11锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HCRX_EL2.MCE2锛坆it 10锛夊繀椤昏鍒濆鍖栦负 0b1锛屼笖 hypervisor 蹇呴』鎸夌収 arm64_mops_hyp 涓墍杩板鐞?MOPS 寮傚父銆?
  瀵逛簬甯︽湁鎵╁睍杞崲鎺у埗瀵勫瓨鍣ㄧ壒鎬э紙FEAT_TCR2锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SCR_EL3.TCR2En锛坆it 43锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCRX_EL2.TCR2En锛坆it 14锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬甯︽湁绗?1 闃舵鏉冮檺闂存帴鎵╁睍鐗规€э紙FEAT_S1PIE锛夌殑 CPU锛?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SCR_EL3.PIEn锛坆it 45锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HFGRTR_EL2.nPIR_EL1锛坆it 58锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGWTR_EL2.nPIR_EL1锛坆it 58锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGRTR_EL2.nPIRE0_EL1锛坆it 57锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGRWR_EL2.nPIRE0_EL1锛坆it 57锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 瀵逛簬甯︽湁鍙椾繚鎶ゆ帶鍒舵爤锛圙uarded Control Stacks锛孎EAT_GCS锛夌殑 CPU锛?
  - GCSCR_EL1 蹇呴』琚垵濮嬪寲涓?0銆?
  - GCSCRE0_EL1 蹇呴』琚垵濮嬪寲涓?0銆?
  - 濡傛灉 EL3 瀛樺湪锛?
    - SCR_EL3.GCSEn锛坆it 39锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  - 濡傛灉 EL2 瀛樺湪锛?
    - GCSCR_EL2 蹇呴』琚垵濮嬪寲涓?0銆?
 - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCRX_EL2.GCSEn 蹇呴』琚垵濮嬪寲涓?0b1銆?
    - HFGITR_EL2.nGCSEPP锛坆it 59锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGITR_EL2.nGCSSTR_EL1锛坆it 58锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGITR_EL2.nGCSPUSHM_EL1锛坆it 57锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGRTR_EL2.nGCS_EL1锛坆it 53锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGRTR_EL2.nGCS_EL0锛坆it 52锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGWTR_EL2.nGCS_EL1锛坆it 53锛夊繀椤昏鍒濆鍖栦负 0b1銆?
    - HFGWTR_EL2.nGCS_EL0锛坆it 52锛夊繀椤昏鍒濆鍖栦负 0b1銆?
 - 瀵逛簬甯︽湁璋冭瘯鏋舵瀯锛堝嵆 FEAT_Debugv8pN锛屾墍鏈夌増鏈級鐨?CPU锛?
 - 濡傛灉 EL3 瀛樺湪锛?
   - MDCR_EL3.TDA锛坆it 9锛夊繀椤昏鍒濆鍖栦负 0b0

 - 瀵逛簬甯︽湁 FEAT_PMUv3 鐨?CPU锛?
 - 濡傛灉 EL3 瀛樺湪锛?
   - MDCR_EL3.TPM锛坆it 6锛夊繀椤昏鍒濆鍖栦负 0b0

  瀵逛簬鏀寔鏃犵姸鎬?64 瀛楄妭鍔犺浇涓庡瓨鍌紙FEAT_LS64锛夌殑 CPU锛?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCRX_EL2.EnALS锛坆it 1锛夊繀椤昏鍒濆鍖栦负 0b1銆?
  瀵逛簬鏀寔甯︾姸鎬?64 瀛楄妭瀛樺偍锛團EAT_LS64_V锛夌殑 CPU锛?
  - 濡傛灉鍐呮牳鍦?EL1 杩涘叆涓?EL2 瀛樺湪锛?
    - HCRX_EL2.EnASR锛坆it 2锛夊繀椤昏鍒濆鍖栦负 0b1銆?
涓婅堪鍏充簬 CPU 妯″紡銆佺紦瀛樸€丮MU銆佹灦鏋勫畾鏃跺櫒銆佷竴鑷存€у拰绯荤粺瀵勫瓨鍣ㄧ殑瑕佹眰閫傜敤浜庢墍鏈?CPU銆傛墍鏈?CPU 蹇呴』浠ョ浉鍚岀殑寮傚父绾у埆杩涘叆鍐呮牳銆傚湪鏂囨。鍖栫殑鍊肩鐢ㄩ櫡闃憋紙traps锛夌殑鎯呭喌涓嬶紝鍙杩欎簺闄烽槺鐢辨洿楂樺紓甯哥骇鍒€忔槑鍦板鐞嗭紝濡傚悓璁剧疆浜嗘枃妗ｅ寲鐨勫€间竴鏍凤紝鍏佽鍚敤杩欎簺闄烽槺銆?
寮曞鍔犺浇绋嬪簭搴斾互濡備笅鏂瑰紡鍦ㄦ瘡涓?CPU 涓婅繘鍏ュ唴鏍革細

- 涓?CPU 蹇呴』鐩存帴璺宠浆鍒板唴鏍搁暅鍍忕殑绗竴鏉℃寚浠ゃ€傝 CPU 浼犻€掔殑璁惧鏍?blob 蹇呴』鍖呭惈姣忎釜 cpu 鑺傜偣鐨?'enable-method' 灞炴€с€傛敮鎸佺殑 enable-method 濡備笅鎵€杩般€?
  棰勬湡寮曞鍔犺浇绋嬪簭灏嗙敓鎴愯繖浜涜澶囨爲灞炴€э紝骞跺湪杩涘叆鍐呮牳涔嬪墠灏嗗叾鎻掑叆 blob 涓€?
- 甯︽湁 "spin-table" enable-method 鐨?CPU 蹇呴』鍦ㄥ叾 cpu 鑺傜偣涓叿鏈?'cpu-release-addr' 灞炴€с€傝灞炴€ф爣璇嗕竴涓嚜鐒跺榻愮殑銆侀浂鍒濆鍖栫殑 64 浣嶅唴瀛樹綅缃€?
  杩欎簺 CPU 搴斿綋鍦ㄥ唴鏍稿閮ㄧ殑涓€涓繚鐣欏唴瀛樺尯鍩燂紙閫氳繃璁惧鏍戜腑鐨?/memreserve/ 鍖哄煙鍛婄煡鍐呮牳锛変腑鑷棆锛坰pin锛夛紝杞鍏?cpu-release-addr 浣嶇疆锛岃浣嶇疆蹇呴』鍖呭惈鍦ㄤ繚鐣欏尯鍩熷唴銆傚彲浠ユ彃鍏ヤ竴鏉?wfe 鎸囦护浠ラ檷浣庡繖寰幆鐨勯澶栧紑閿€锛屼富 CPU 灏嗗彂鍑轰竴鏉?sev 鎸囦护銆傚綋 cpu-release-addr 鎵€鎸囧悜浣嶇疆琚鍑洪潪闆跺€兼椂锛岃 CPU 蹇呴』璺宠浆鍒版鍊笺€傝鍊煎皢浠ュ崟涓?64 浣嶅皬绔€煎啓鍏ワ紝鍥犳 CPU 鍦ㄨ烦杞箣鍓嶅繀椤诲皢璇诲嚭鐨勫€艰浆鎹负鍏舵湰鏈哄瓧鑺傚簭銆?
- 甯︽湁 "psci" enable method 鐨?CPU 搴斾繚鐣欏湪鍐呮牳涔嬪锛堝嵆锛屽湪璁惧鏍?memory 鑺傜偣涓弿杩扮粰鍐呮牳鐨勫唴瀛樺尯鍩熶箣澶栵紝鎴栧湪璁惧鏍戜腑閫氳繃 /memreserve/ 鍖哄煙鎻忚堪缁欏唴鏍哥殑淇濈暀鍐呭瓨鍖哄煙鍐咃級銆傚唴鏍稿皢鍙戝嚭 CPU_ON 璋冪敤锛屽 ARM 鏂囨。缂栧彿 ARM DEN 0022A锛堛€夾RM 澶勭悊鍣ㄤ笂鐨勭數婧愮姸鎬佸崗璋冩帴鍙ｇ郴缁熻蒋浠躲€嬶級鎵€杩帮紝浠ュ皢 CPU 甯﹀叆鍐呮牳銆?
  璁惧鏍戝簲鍖呭惈 'psci' 鑺傜偣锛屽 Documentation/devicetree/bindings/arm/psci.yaml 涓墍杩般€?
- 娆＄骇 CPU 閫氱敤瀵勫瓨鍣ㄨ缃?
  - x0 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?  - x1 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?  - x2 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?  - x3 = 0锛堜繚鐣欎緵灏嗘潵浣跨敤锛?