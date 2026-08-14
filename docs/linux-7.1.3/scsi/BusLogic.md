
## 閫傜敤浜?Linux 鐨?BusLogic MultiMaster 涓?FlashPoint SCSI 椹卞姩

			 Version 2.0.15 for Linux 2.0

			 Version 2.1.15 for Linux 2.1

			      PRODUCTION RELEASE

				17 August 1998

			       Leonard N. Zubkoff

			       Dandelion Digital

			       lnz@dandelion.com

	 Copyright 1995-1998 by Leonard N. Zubkoff <lnz@dandelion.com>


## 绠€浠?
BusLogic, Inc. 璁捐骞跺埗閫犱簡澶氱楂樻€ц兘 SCSI 涓绘満閫傞厤鍣紙host
adapter锛夛紝瀹冧滑鍊熷姪 MultiMaster ASIC 鎶€鏈紝鍦ㄧ绫荤箒澶氱殑鎬荤嚎鏋舵瀯涓?鍏变韩涓€濂楅€氱敤鐨勭紪绋嬫帴鍙ｃ€侭usLogic 浜?1996 骞?2 鏈堣 Mylex Corporation
鏀惰喘锛屼絾鏈┍鍔ㄦ墍鏀寔鐨勪骇鍝佹渶鍒濇槸浠?BusLogic 鍚嶄箟鎺ㄥ嚭鐨勶紝鍥犳璇ュ悕绉?琚繚鐣欏湪婧愪唬鐮佷笌鏂囨。涓€?
鏈┍鍔ㄦ敮鎸佺洰鍓嶆墍鏈夌殑 BusLogic MultiMaster 涓绘満閫傞厤鍣紝骞朵笖鍙渶鏋佸皯
鐢氳嚦鏃犻渶淇敼锛屽氨搴旇兘鏀寔鏈潵浠讳綍 MultiMaster 璁捐銆傝緝杩戞椂鏈燂紝BusLogic
鎺ㄥ嚭浜?FlashPoint 涓绘満閫傞厤鍣紝瀹冧滑鎴愭湰鏇翠綆锛屽苟涓斾緷璧栦富鏈?CPU锛岃€岄潪
鏉胯浇澶勭悊鍣ㄣ€傚敖绠℃病鏈夋澘杞?CPU锛孎lashPoint 涓绘満閫傞厤鍣ㄤ粛琛ㄧ幇闈炲父鍑鸿壊锛?鍛戒护寤惰繜鏋佷綆銆侭usLogic 杩戞湡鍚戞垜鎻愪緵浜?FlashPoint Driver Developer's
Kit锛屽叾涓寘鍚?FlashPoint SCCB Manager 鐨勬枃妗ｄ笌鍙嚜鐢卞啀鍒嗗彂鐨勬簮浠ｇ爜銆?SCCB Manager 鏄竴濂楄繍琛屽湪涓绘満 CPU 涓婄殑浠ｇ爜搴擄紝鎵ц鐨勫姛鑳界被浼间簬
MultiMaster 涓绘満閫傞厤鍣ㄤ笂鐨勫浐浠躲€傚緱鐩婁簬浠栦滑鎻愪緵浜?SCCB Manager锛屾湰椹卞姩
鐜板凡鍚屾牱鏀寔 FlashPoint 涓绘満閫傞厤鍣ㄣ€?
鎴戜负 Linux 缂栧啓杩欎釜鍏ㄦ柊 BusLogic 椹卞姩鐨勪富瑕佺洰鏍囨槸锛氬厖鍒嗗彂鎸?BusLogic
SCSI 涓绘満閫傞厤鍣ㄤ笌鐜颁唬 SCSI 澶栬鎵€鑳借揪鍒扮殑瀹屾暣鎬ц兘锛屽苟鎻愪緵涓€涓珮搴?鍋ュ．鐨勯┍鍔紝鍙緷璧栧叾鐢ㄤ簬楂樻€ц兘銆佸叧閿换鍔＄殑鍦哄悎銆傛墍鏈変富瑕佺殑鎬ц兘鐗规€?閮藉彲浠ヤ粠 Linux 鍐呮牳鍛戒护琛屾垨妯″潡鍒濆鍖栨椂閰嶇疆锛屼娇鍚勪釜瀹夎鑳藉閽堝鍏?鐗瑰畾闇€姹傝皟鏁撮┍鍔ㄦ€ц兘涓庨敊璇仮澶嶃€?
鍏充簬 BusLogic SCSI 涓绘満閫傞厤鍣?Linux 鏀寔鐨勬渶鏂颁俊鎭紝浠ュ強鏈┍鍔ㄧ殑鏈€鏂?鍙戝竷鐗堟湰鍜?BT-948/958/958D 鐨勬渶鏂板浐浠讹紝灏嗗缁堝彲浠ヤ粠鎴戠殑 Linux 涓婚〉
URL "http://sourceforge.net/projects/dandelion/" 鑾峰彇銆?
缂洪櫡鎶ュ憡搴旈€氳繃鐢靛瓙閭欢鍙戦€佽嚦 "lnz@dandelion.com"銆傝鍦ㄧ己闄锋姤鍛婁腑鍖呭惈
椹卞姩涓?SCSI 瀛愮郴缁熷湪鍚姩鏃舵姤鍛婄殑瀹屾暣閰嶇疆淇℃伅锛屼互鍙婁换浣曚笌 SCSI 鎿嶄綔
鐩稿叧鐨勫悗缁郴缁熸秷鎭紝骞惰缁嗘弿杩颁綘绯荤粺鐨勭‖浠堕厤缃€?
Mylex 鏄竴瀹堕潪甯稿€煎緱鍚堜綔鐨勫叕鍙革紝鎴戝悜 Linux 绀惧尯澶у姏鎺ㄨ崘浠栦滑鐨勪骇鍝併€?1995 骞?11 鏈堬紝鎴戞湁鏈轰細鎴愪负浠栦滑鏈€鏂?MultiMaster 浜у搧鈥斺€擝T-948 PCI
Ultra SCSI 涓绘満閫傞厤鍣ㄢ€斺€旂殑 beta 娴嬭瘯绔欑偣锛岄殢鍚庡湪 1996 骞?1 鏈堝張鎴愪负
BT-958 PCI Wide Ultra SCSI 涓绘満閫傞厤鍣ㄧ殑娴嬭瘯绔欑偣銆傝繖鏄簰鍒╀簰鎯犵殑锛屽洜涓?Mylex 鑾峰緱浜嗗叾鑷韩娴嬭瘯鍥㈤槦闅句互杞绘槗杈炬垚鐨勪竴瀹氱▼搴︾殑娴嬭瘯锛岃€?Linux 绀惧尯
鍒欏緱浠ユ嫢鏈夊湪涓婂競鍓嶅氨宸茬敤 Linux 鍏呭垎娴嬭瘯杩囩殑楂樻€ц兘涓绘満閫傞厤鍣ㄣ€傝繖绉嶅叧绯?涔熻鎴戞湁鏈轰細鐩存帴涓庝粬浠殑鎶€鏈洟闃熶氦娴侊紝鏇村鍦颁簡瑙ｅ叾浜у搧鐨勫唴閮ㄨ繍浣滐紝骞?鍙嶈繃鏉ュ悜浠栦滑璇存槑 Linux 绀惧尯鐨勯渶姹備笌娼滃姏銆?
杈冭繎鏃舵湡锛孧ylex 閲嶇敵浜嗗叕鍙告敮鎸?Linux 绀惧尯鐨勫叴瓒ｏ紝鑰屾垜鐩墠姝ｅ湪涓?DAC960
PCI RAID 鎺у埗鍣ㄧ紪鍐欎竴涓?Linux 椹卞姩銆侻ylex 鐨勫叴瓒ｄ笌鏀寔浠や汉鍗佸垎鎰熸縺銆?
涓庢煇浜涘叾浠栧巶鍟嗕笉鍚岋紝濡傛灉浣犲湪浣跨敤 Linux 鏃惰仈绯?Mylex 鎶€鏈敮鎸佸姹傞棶棰?甯姪锛屼粬浠笉浼氬憡璇変綘浣跨敤鍏朵骇鍝佷笉鍙楁敮鎸併€備粬浠渶鏂扮殑浜у搧甯傚満璧勬枡鐢氳嚦
鍐欐槑 "Mylex SCSI host adapters are compatible with all major operating
systems including: ... Linux ..."銆?
Mylex Corporation 浣嶄簬 34551 Ardenwood Blvd., Fremont, California 94555,
USA锛屽彲閫氳繃 510/796-6100 鑱旂郴锛屾垨鍦ㄤ竾缁寸綉涓婇€氳繃 http://www.mylex.com
鑱旂郴銆侻ylex HBA 鎶€鏈敮鎸佸彲閫氳繃鐢靛瓙閭欢 techsup@mylex.com銆佽闊?510/608-2400
鎴栦紶鐪?510/745-7715 鑱旂郴銆傛娲蹭笌鏃ユ湰鍔炰簨澶勭殑鑱旂郴淇℃伅鍙湪缃戠珯涓婅幏鍙栥€?

## 椹卞姩鐗规€?

### 閰嶇疆鎶ュ憡涓庢祴璇?
  鍦ㄧ郴缁熷垵濮嬪寲鏈熼棿锛岄┍鍔ㄤ細骞挎硾鍦版姤鍛婁富鏈洪€傞厤鍣ㄧ殑纭欢閰嶇疆锛屽寘鎷笌姣忎釜
  鐩爣璁惧璇锋眰骞跺崗鍟嗙殑鍚屾浼犺緭鍙傛暟銆備細閽堝姣忎釜鐩爣璁惧鎶ュ憡鍚屾鍗忓晢銆?   Wide 鍗忓晢浠ュ強鏂紑/閲嶈繛锛圖isconnect/Reconnect锛夌殑 AutoSCSI 璁剧疆锛屼互鍙?  鏍囪闃熷垪锛圱agged Queuing锛夌殑鐘舵€併€傚鏋滄墍鏈夌洰鏍囪澶囬兘閲囩敤鍚屼竴璁剧疆锛屽垯
  浣跨敤涓€涓崟璇嶆垨鐭琛ㄧず锛涘惁鍒欙紝浼氫负姣忎釜鐩爣璁惧鎻愪緵涓€涓瓧姣嶄互琛ㄧず鍏?  鍚勮嚜鐨勭姸鎬併€備互涓嬬ず渚嬪簲鑳介槓鏄庤繖绉嶆姤鍛婃牸寮忥細

    Synchronous Negotiation: Ultra

      宸插鎵€鏈夌洰鏍囪澶囧惎鐢ㄥ悓姝ュ崗鍟嗭紝涓绘満閫傞厤鍣ㄥ皢灏濊瘯鍗忓晢 20.0 鍏嗕紶杈?绉掋€?
    Synchronous Negotiation: Fast

      宸插鎵€鏈夌洰鏍囪澶囧惎鐢ㄥ悓姝ュ崗鍟嗭紝涓绘満閫傞厤鍣ㄥ皢灏濊瘯鍗忓晢 10.0 鍏嗕紶杈?绉掋€?
    Synchronous Negotiation: Slow

      宸插鎵€鏈夌洰鏍囪澶囧惎鐢ㄥ悓姝ュ崗鍟嗭紝涓绘満閫傞厤鍣ㄥ皢灏濊瘯鍗忓晢 5.0 鍏嗕紶杈?绉掋€?
    Synchronous Negotiation: Disabled

      宸茬鐢ㄥ悓姝ュ崗鍟嗭紝鎵€鏈夌洰鏍囪澶囪闄愬埗涓哄紓姝ユ搷浣溿€?
    Synchronous Negotiation: UFSNUUU#UUUUUUUU

      宸插鐩爣璁惧 0 浠ュ強 4 鍒?15 鍚敤 Ultra 閫熷害鐨勫悓姝ュ崗鍟嗭紝瀵圭洰鏍囪澶?1
      鍚敤 Fast 閫熷害锛屽鐩爣璁惧 2 鍚敤 Slow 閫熷害锛屼笖涓嶅厑璁哥洰鏍囪澶?3 浣跨敤銆?      涓绘満閫傞厤鍣ㄧ殑 SCSI ID 鐢?"#" 琛ㄧず銆?
    Wide 鍗忓晢銆佹柇寮€/閲嶈繛浠ュ強鏍囪闃熷垪鐨勭姸鎬佷細琚姤鍛婁负 "Enabled"銆?    "Disabled"锛屾垨涓€涓?"Y" 鍜?"N" 瀛楁瘝銆?
### 鎬ц兘鐗规€?
  BusLogic SCSI 涓绘満閫傞厤鍣ㄧ洿鎺ュ疄鐜颁簡 SCSI-2 鏍囪闃熷垪锛屽洜姝ら┍鍔ㄤ腑鍖呭惈浜?  瀵规姤鍛婂叿澶囨爣璁伴槦鍒楄兘鍔涚殑浠讳綍鐩爣璁惧浣跨敤鏍囪闃熷垪鐨勬敮鎸併€傛爣璁伴槦鍒楀厑璁?  鍚戞瘡涓洰鏍囪澶囨垨閫昏緫鍗曞厓鍙戝嚭澶氫釜 outstanding 鍛戒护锛屽苟鍙樉钁楁彁鍗?I/O
  鎬ц兘銆傛澶栵紝浣跨敤浜?BusLogic 鐨勪弗鏍艰疆璇紙Strict Round Robin锛夋ā寮忔潵浼樺寲
  涓绘満閫傞厤鍣ㄦ€ц兘锛屽苟涓斿垎鏁?鑱氶泦锛坰catter/gather锛塈/O 鑳藉鏀寔 Linux I/O
  瀛愮郴缁熷彲鏈夋晥鍒╃敤鐨勪换鎰忓涓銆傞€氳繃鍐呮牳鍛戒护琛屾垨妯″潡鍒濆鍖栨椂鎻愪緵鐨勯┍鍔?  閫夐」锛屽彲浠ユ帶鍒舵瘡涓洰鏍囪澶囧鏍囪闃熷垪鐨勪娇鐢紝浠ュ強鍗曠嫭閫夋嫨鏍囪闃熷垪娣卞害銆?  榛樿鎯呭喌涓嬶紝闃熷垪娣卞害浼氭牴鎹富鏈洪€傞厤鍣ㄧ殑鎬婚槦鍒楁繁搴︿互鍙婃墍鍙戠幇鐩爣璁惧鐨?  鏁伴噺銆佺被鍨嬨€侀€熷害鍜岃兘鍔涜嚜鍔ㄧ‘瀹氥€傛澶栵紝鍙宸茬煡涓绘満閫傞厤鍣ㄥ浐浠剁増鏈湭姝ｇ‘
  瀹炵幇鏍囪闃熷垪锛屾垨鑰呬竴鏃﹂€夋嫨浜嗛槦鍒楁繁搴︿负 1锛屾爣璁伴槦鍒楀氨浼氳嚜鍔ㄧ鐢ㄣ€傚浜?  宸茬鐢ㄦ柇寮€/閲嶈繛鐨勭洰鏍囪澶囷紝鍏舵爣璁伴槦鍒椾篃浼氳绂佺敤銆?
### 鍋ュ．鎬х壒鎬?
  椹卞姩瀹炵幇浜嗗箍娉涚殑閿欒鎭㈠娴佺▼銆傚綋 SCSI 瀛愮郴缁熻緝楂樺眰璇锋眰閲嶇疆涓€涓秴鏃剁殑
  鍛戒护鏃讹紝浼氭牴鎹?SCSI 瀛愮郴缁熺殑寤鸿锛屽湪瀹屾暣鐨勪富鏈洪€傞厤鍣ㄧ‖澶嶄綅涓?SCSI 鎬荤嚎
  澶嶄綅涔嬮棿锛屼互鍙婂悜鍚勪釜鐩爣璁惧鍙戦€佹€荤嚎璁惧澶嶄綅娑堟伅涔嬮棿杩涜閫夋嫨銆傞敊璇仮澶?  绛栫暐鍙€氳繃椹卞姩閫夐」涓烘瘡涓洰鏍囪澶囧崟鐙€夋嫨锛屼篃鍖呮嫭鍚戜笌姝ｅ湪琚噸缃殑鍛戒护
  鐩稿叧鑱旂殑鐗瑰畾鐩爣璁惧鍙戦€佹€荤嚎璁惧澶嶄綅娑堟伅锛屼互鍙婂畬鍏ㄦ姂鍒堕敊璇仮澶嶄互閬垮厤
  骞叉壈杩愯涓嶆甯哥殑璁惧銆傚鏋滈€夋嫨浜嗘€荤嚎璁惧澶嶄綅閿欒鎭㈠绛栫暐锛岃€屽彂閫佹€荤嚎
  璁惧澶嶄綅鏈兘鎭㈠姝ｇ‘鎿嶄綔锛屽垯涓嬩竴涓閲嶇疆鐨勫懡浠ゅ皢寮哄埗杩涜涓€娆″畬鏁寸殑涓绘満
  閫傞厤鍣ㄧ‖澶嶄綅涓?SCSI 鎬荤嚎澶嶄綅銆傜敱鍏朵粬璁惧寮曡捣骞惰涓绘満閫傞厤鍣ㄦ娴嬪埌鐨?SCSI
  鎬荤嚎澶嶄綅锛屼篃浼氶€氳繃鍚戜富鏈洪€傞厤鍣ㄥ彂鍑鸿蒋澶嶄綅骞堕噸鏂板垵濮嬪寲鏉ュ鐞嗐€傛渶鍚庯紝濡傛灉
  鏍囪闃熷垪澶勪簬娲昏穬鐘舵€侊紝涓斿湪 10 鍒嗛挓闂撮殧鍐呭彂鐢熶簡澶氭鍛戒护閲嶇疆锛屾垨鑰呭鏋滃湪
  杩愯鐨勫墠 10 鍒嗛挓鍐呭彂鐢熶簡鍛戒护閲嶇疆锛屽垯浼氱鐢ㄨ鐩爣璁惧鐨勬爣璁伴槦鍒椼€傝繖浜?  閿欒鎭㈠閫夐」閫氳繃闃叉涓埆鍑洪敊璁惧瀵艰嚧鏁翠釜绯荤粺閿佸畾鎴栧穿婧冿紝浠庤€屾彁鍗囨暣浣?  绯荤粺鐨勫仴澹€э紝骞剁敱姝ゅ湪绉婚櫎杩濊閮ㄤ欢鍚庡厑璁歌繘琛屽共鍑€鐨勫叧鏈轰笌閲嶅惎銆?
### PCI 閰嶇疆鏀寔

  鍦ㄨ繍琛屽惎鐢ㄤ簡 PCI BIOS 鏀寔鐨勫唴鏍哥殑 PCI 绯荤粺涓婏紝鏈┍鍔ㄥ皢鏌ヨ PCI 閰嶇疆
  绌洪棿锛屽苟浣跨敤鐢辩郴缁?BIOS 鍒嗛厤鐨?I/O 绔彛鍦板潃锛岃€岄潪 ISA 鍏煎鐨?I/O 绔彛
  鍦板潃銆傞殢鍚庨┍鍔ㄤ細绂佺敤 ISA 鍏煎鐨?I/O 绔彛銆傚湪 PCI 绯荤粺涓婏紝杩樺缓璁娇鐢?  AutoSCSI 宸ュ叿瀹屽叏绂佺敤 ISA 鍏煎 I/O 绔彛锛屽洜涓哄畠骞舵棤蹇呰銆傚湪 BT-948/958/958D
  涓婏紝ISA 鍏煎 I/O 绔彛榛樿鏄鐢ㄧ殑銆?
### /proc 鏂囦欢绯荤粺鏀寔

  涓绘満閫傞厤鍣ㄩ厤缃俊鎭殑鍓湰锛岃繛鍚屾洿鏂扮殑鏁版嵁浼犺緭涓庨敊璇仮澶嶇粺璁′俊鎭紝鍙?  閫氳繃 /proc/scsi/BusLogic/<N> 鎺ュ彛鑾峰彇銆?
### 鍏变韩涓柇鏀寔

  鍦ㄦ敮鎸佸叡浜腑鏂殑绯荤粺涓婏紝浠绘剰鏁伴噺鐨?BusLogic 涓绘満閫傞厤鍣ㄥ彲鍏变韩鍚屼竴涓?  涓柇璇锋眰閫氶亾銆?

## 鍙楁敮鎸佺殑涓绘満閫傞厤鍣?
浠ヤ笅鍒楄〃鍖呭惈鎴嚦鏈枃妗ｆ棩鏈熸墍鏀寔鐨?BusLogic SCSI 涓绘満閫傞厤鍣ㄣ€傚缓璁换浣?鎵撶畻璐拱涓嬪垪琛ㄤ腑鏈垪鍑虹殑 BusLogic 涓绘満閫傞厤鍣ㄧ殑浜轰簨鍏堣仈绯讳綔鑰咃紝浠ョ‘璁ゅ叾
褰撳墠鎴栧皢鏉ョ殑鏀寔鎯呭喌銆?
FlashPoint 绯诲垪 PCI 涓绘満閫傞厤鍣細

=======================	=============================================
FlashPoint LT (BT-930)	Ultra SCSI-3
FlashPoint LT (BT-930R)	Ultra SCSI-3 with RAIDPlus
FlashPoint LT (BT-920)	Ultra SCSI-3 (BT-930 without BIOS)
FlashPoint DL (BT-932)	Dual Channel Ultra SCSI-3
FlashPoint DL (BT-932R)	Dual Channel Ultra SCSI-3 with RAIDPlus
FlashPoint LW (BT-950)	Wide Ultra SCSI-3
FlashPoint LW (BT-950R)	Wide Ultra SCSI-3 with RAIDPlus
FlashPoint DW (BT-952)	Dual Channel Wide Ultra SCSI-3
FlashPoint DW (BT-952R)	Dual Channel Wide Ultra SCSI-3 with RAIDPlus
=======================	=============================================

MultiMaster "W" 绯诲垪涓绘満閫傞厤鍣細

=======     ===		==============================
BT-948	    PCI		Ultra SCSI-3
BT-958	    PCI		Wide Ultra SCSI-3
BT-958D	    PCI		Wide Differential Ultra SCSI-3
=======     ===		==============================

MultiMaster "C" 绯诲垪涓绘満閫傞厤鍣細

========    ====	==============================
BT-946C	    PCI		Fast SCSI-2
BT-956C	    PCI		Wide Fast SCSI-2
BT-956CD    PCI		Wide Differential Fast SCSI-2
BT-445C	    VLB		Fast SCSI-2
BT-747C	    EISA	Fast SCSI-2
BT-757C	    EISA	Wide Fast SCSI-2
BT-757CD    EISA	Wide Differential Fast SCSI-2
========    ====	==============================

MultiMaster "S" 绯诲垪涓绘満閫傞厤鍣細

=======     ====	==============================
BT-445S	    VLB		Fast SCSI-2
BT-747S	    EISA	Fast SCSI-2
BT-747D	    EISA	Differential Fast SCSI-2
BT-757S	    EISA	Wide Fast SCSI-2
BT-757D	    EISA	Wide Differential Fast SCSI-2
BT-742A	    EISA	SCSI-2 (742A revision H)
=======     ====	==============================

MultiMaster "A" 绯诲垪涓绘満閫傞厤鍣細

=======     ====	==============================
BT-742A	    EISA	SCSI-2 (742A revisions A - G)
=======     ====	==============================

鐪熸灞炰簬 BusLogic MultiMaster 鍏嬮殕鐨?AMI FastDisk 涓绘満閫傞厤鍣ㄤ篃鍙楁湰椹卞姩
鏀寔銆?
BusLogic SCSI 涓绘満閫傞厤鍣ㄦ棦鏈夎８鏉垮舰寮忥紝涔熸湁闆跺敭濂楄褰㈠紡銆備笂琛ㄤ腑鐨?BT-
鍨嬪彿鎸囩殑鏄８鏉垮寘瑁呫€傞浂鍞瑁呯殑鍨嬪彿鍙€氳繃灏嗕笂琛ㄤ腑鐨?BT- 鏇挎崲涓?KT- 寰楀埌銆?闆跺敭濂楄鍖呭惈瑁告澘涓庢墜鍐岋紝浠ュ強瑁告澘鎵€涓嶆彁渚涚殑绾跨紗銆侀┍鍔ㄤ粙璐ㄤ笌鏂囨。銆?

## FlashPoint 瀹夎璇存槑


### RAIDPlus 鏀寔

  FlashPoint 涓绘満閫傞厤鍣ㄧ幇鍦ㄥ寘鍚?RAIDPlus鈥斺€擬ylex 鐨勫彲寮曞杞欢 RAID銆?  RAIDPlus 鍦?Linux 涓婁笉鍙楁敮鎸侊紝涔熸病鏈夎鍒掓敮鎸佸畠銆侺inux 2.0 涓殑 MD 椹卞姩
  鎻愪緵涓叉帴锛圠INEAR锛変笌鏉″甫鍖栵紙RAID-0锛夛紝鑰屽闀滃儚锛圧AID-1锛夈€佸浐瀹氬鍋舵牎楠?  锛圧AID-4锛夊拰鍒嗗竷寮忓鍋舵牎楠岋紙RAID-5锛夌殑鏀寔鍙彟琛岃幏鍙栥€傚唴寤虹殑 Linux RAID
  鏀寔閫氬父鏇寸伒娲伙紝棰勮鎬ц兘涔熶細浼樹簬 RAIDPlus锛屽洜姝ゅ皢 RAIDPlus 鏀寔绾冲叆
  BusLogic 椹卞姩鐨勫姩鏈哄緢灏忋€?
### 鍚敤 UltraSCSI 浼犺緭

  FlashPoint 涓绘満閫傞厤鍣ㄥ嚭鍘傛椂閰嶇疆涓?"Factory Default"锛堝嚭鍘傞粯璁わ級璁剧疆锛?  杩欎簺璁剧疆杈冧负淇濆畧锛屼笉鍏佽鍗忓晢 UltraSCSI 閫熷害銆傝繖鏍峰湪灏嗚繖浜涗富鏈洪€傞厤鍣?  瀹夎鍒板竷绾挎垨缁堢鐢甸樆涓嶈冻浠ユ敮鎸?UltraSCSI 鎿嶄綔鐨勭郴缁熶腑鏃讹紝鎴栫幇鏈?SCSI
  璁惧鏈纭搷搴?UltraSCSI 閫熷害鐨勫悓姝ヤ紶杈撳崗鍟嗘椂锛屽彲鍑忓皯闂銆傚彲浣跨敤
  AutoSCSI 杞藉叆 "Optimum Performance"锛堟渶浣虫€ц兘锛夎缃紝浠ュ厑璁镐笌鎵€鏈夎澶?  鍗忓晢 UltraSCSI 閫熷害锛屼篃鍙互閫愪釜璁惧鍦板惎鐢?UltraSCSI 閫熷害銆傚缓璁湪杞藉叆
  "Optimum Performance" 璁剧疆鍚庢墜鍔ㄧ鐢?SCAM銆?

## BT-948/958/958D 瀹夎璇存槑

BT-948/958/958D PCI Ultra SCSI 涓绘満閫傞厤鍣ㄦ湁涓€浜涚壒鎬э紝鍦ㄥ畨瑁?Linux 鏃?鍦ㄦ煇浜涙儏鍐典笅鍙兘闇€瑕佺暀鎰忋€?
### PCI I/O 绔彛鍒嗛厤

  閰嶇疆涓哄嚭鍘傞粯璁よ缃椂锛孊T-948/958/958D 鍙細璇嗗埆鐢变富鏉?PCI BIOS 鍋氬嚭鐨?  PCI I/O 绔彛鍒嗛厤銆侭T-948/958/958D 涓嶄細鍝嶅簲姝ゅ墠 BusLogic SCSI 涓绘満閫傞厤鍣?  鎵€鍝嶅簲鐨勪换浣?ISA 鍏煎 I/O 绔彛銆傛湰椹卞姩鏀寔 PCI I/O 绔彛鍒嗛厤锛屽洜姝よ繖鏄?  棣栭€夐厤缃€傜劧鑰岋紝濡傛灉鐢变簬鏌愮鍘熷洜蹇呴』浣跨敤宸茶繃鏃剁殑 BusLogic 椹卞姩锛堜緥濡?  鏌愪釜 Linux 鍙戣鐗堝叾寮曞鍐呮牳灏氭湭浣跨敤鏈┍鍔級锛孊usLogic 鎻愪緵浜嗕竴涓?AutoSCSI
  閰嶇疆閫夐」浠ュ惎鐢ㄤ竴涓紶缁熺殑 ISA 鍏煎 I/O 绔彛銆?
  瑕佸惎鐢ㄨ繖涓悜鍚庡吋瀹归€夐」锛屽彲鍦ㄧ郴缁熷惎鍔ㄦ椂閫氳繃 Ctrl-B 璋冪敤 AutoSCSI 宸ュ叿锛?  閫夋嫨 "Adapter Configuration"銆?View/Modify Configuration"锛岀劧鍚庡皢
  "ISA Compatible Port" 璁剧疆浠?"Disable" 鏀逛负 "Primary" 鎴?"Alternate"銆?  涓€鏃︽湰椹卞姩瀹夎瀹屾瘯锛屽簲灏?"ISA Compatible Port" 閫夐」璁惧洖 "Disable"锛屼互
  閬垮厤灏嗘潵鍙兘鍑虹幇鐨?I/O 绔彛鍐茬獊銆傝緝鑰佺殑 BT-946C/956C/956CD 涔熸湁姝ら厤缃?  閫夐」锛屼絾鍏跺嚭鍘傞粯璁よ缃负 "Primary"銆?
### PCI 鎻掓Ы鎵弿椤哄簭

  鍦ㄩ厤鏈夊涓?BusLogic PCI 涓绘満閫傞厤鍣ㄧ殑绯荤粺涓紝涓?BT-946C/956C/956CD 鐩告瘮锛?  BT-948/958/958D 鎵弿 PCI 鎻掓Ы鐨勯『搴忓彲鑳界湅浼肩浉鍙嶃€傝浣夸粠 SCSI 纾佺洏寮曞
  姝ｇ‘宸ヤ綔锛屼富鏈洪€傞厤鍣ㄧ殑 BIOS 涓庡唴鏍稿繀椤诲氨鍝釜纾佺洏鏄紩瀵艰澶囪揪鎴愪竴鑷达紝
  杩欒姹傚畠浠互鐩稿悓椤哄簭璇嗗埆 PCI 涓绘満閫傞厤鍣ㄣ€備富鏉?PCI BIOS 鎻愪緵浜嗕竴绉嶆灇涓?  PCI 涓绘満閫傞厤鍣ㄧ殑鏍囧噯鏂瑰紡锛孡inux 鍐呮牳灏变娇鐢ㄨ繖绉嶆柟寮忋€傛煇浜?PCI BIOS 瀹炵幇
  鎸夋€荤嚎鍙峰拰璁惧鍙烽€掑鐨勯『搴忔灇涓?PCI 鎻掓Ы锛岃€屽彟涓€浜涘垯鎸夌浉鍙嶆柟鍚戞灇涓俱€?
  閬楁喚鐨勬槸锛孧icrosoft 鍐冲畾 Windows 95 灏嗗缁堟寜鎬荤嚎鍙峰拰璁惧鍙烽€掑鐨勯『搴?  鏋氫妇 PCI 鎻掓Ы锛岃€屼笉绠?PCI BIOS 鐨勬灇涓鹃『搴忥紝骞朵笖瑕佹眰涓绘満閫傞厤鍣ㄧ殑 BIOS
  鏀寔鍏舵柟妗堜互鑾峰緱 Windows 95 璁よ瘉銆傚洜姝わ紝BT-948/958/958D 鐨勫嚭鍘傞粯璁よ缃?  鎸夋€荤嚎鍙峰拰璁惧鍙烽€掑鐨勯『搴忔灇涓句富鏈洪€傞厤鍣ㄣ€傝绂佺敤姝ょ壒鎬э紝鍙湪绯荤粺鍚姩鏃?  閫氳繃 Ctrl-B 璋冪敤 AutoSCSI 宸ュ叿锛岄€夋嫨 "Adapter Configuration"銆?View/Modify
  Configuration"锛屾寜 Ctrl-F10锛岀劧鍚庡皢 "Use Bus And Device # For PCI Scanning
  Seq." 閫夐」鏀逛负 OFF銆?
  鏈┍鍔ㄥ皢鏌ヨ PCI 鎵弿椤哄簭锛圫canning Sequence锛夐€夐」鐨勮缃紝浠ヤ究浠ヤ笌涓绘満
  閫傞厤鍣?BIOS 鏋氫妇鐩稿悓鐨勯『搴忚瘑鍒富鏈洪€傞厤鍣ㄣ€?
### 鍚敤 UltraSCSI 浼犺緭

  BT-948/958/958D 鍑哄巶鏃堕厤缃负 "Factory Default"锛堝嚭鍘傞粯璁わ級璁剧疆锛岃繖浜涜缃?  杈冧负淇濆畧锛屼笉鍏佽鍗忓晢 UltraSCSI 閫熷害銆傝繖鏍峰湪灏嗚繖浜涗富鏈洪€傞厤鍣ㄥ畨瑁呭埌甯冪嚎鎴?  缁堢鐢甸樆涓嶈冻浠ユ敮鎸?UltraSCSI 鎿嶄綔鐨勭郴缁熶腑鏃讹紝鎴栫幇鏈?SCSI 璁惧鏈纭搷搴?  UltraSCSI 閫熷害鐨勫悓姝ヤ紶杈撳崗鍟嗘椂锛屽彲鍑忓皯闂銆傚彲浣跨敤 AutoSCSI 杞藉叆
  "Optimum Performance"锛堟渶浣虫€ц兘锛夎缃紝浠ュ厑璁镐笌鎵€鏈夎澶囧崗鍟?UltraSCSI
  閫熷害锛屼篃鍙互閫愪釜璁惧鍦板惎鐢?UltraSCSI 閫熷害銆傚缓璁湪杞藉叆 "Optimum Performance"
  璁剧疆鍚庢墜鍔ㄧ鐢?SCAM銆?

## 椹卞姩閫夐」

BusLogic 椹卞姩閫夐」鍙€氳繃 Linux 鍐呮牳鍛戒护琛岋紝鎴栭€氳繃鍙姞杞藉唴鏍告ā鍧楀畨瑁呭伐鍏?锛圠oadable Kernel Module Installation Facility锛夋寚瀹氥€傚涓富鏈洪€傞厤鍣ㄧ殑
椹卞姩閫夐」鍙互閫氳繃鍒嗗彿鍒嗛殧閫夐」瀛楃涓叉潵鎸囧畾锛屼篃鍙互鍦ㄥ懡浠よ涓婃寚瀹氬涓?"BusLogic=" 瀛楃涓层€傚崟涓富鏈洪€傞厤鍣ㄧ殑鍚勪釜閫夐」瑙勮寖浠ラ€楀彿鍒嗛殧銆傛帰娴嬩笌璋冭瘯
閫夐」閫傜敤浜庢墍鏈変富鏈洪€傞厤鍣紝鑰屽叾浣欓€夐」浠呭崟鐙€傜敤浜庢墍閫夌殑涓绘満閫傞厤鍣ㄣ€?
BusLogic 椹卞姩鐨勬帰娴嬮€夐」鍖呭惈濡備笅鍐呭锛?
NoProbe

  "NoProbe" 閫夐」绂佺敤鎵€鏈夋帰娴嬶紝鍥犳涓嶄細妫€娴嬪埌浠讳綍 BusLogic 涓绘満閫傞厤鍣ㄣ€?
NoProbePCI

  "NoProbePCI" 閫夐」绂佺敤瀵?PCI 閰嶇疆绌洪棿锛圥CI Configuration Space锛夌殑鏌ヨ锛?  鍥犳鍙細妫€娴嬪埌 ISA MultiMaster 涓绘満閫傞厤鍣紝浠ュ強 ISA 鍏煎 I/O 绔彛璁句负
  "Primary" 鎴?"Alternate" 鐨?PCI MultiMaster 涓绘満閫傞厤鍣ㄣ€?
NoSortPCI

  "NoSortPCI" 閫夐」寮哄埗 PCI MultiMaster 涓绘満閫傞厤鍣ㄦ寜 PCI BIOS 鎻愪緵鐨勯『搴?  鏋氫妇锛屽拷鐣?AutoSCSI "Use Bus And Device # For PCI Scanning Seq." 閫夐」鐨?  浠讳綍璁剧疆銆?
MultiMasterFirst

  "MultiMasterFirst" 閫夐」寮哄埗鍏堟帰娴?MultiMaster 涓绘満閫傞厤鍣紝鍐嶆帰娴?FlashPoint
  涓绘満閫傞厤鍣ㄣ€傞粯璁ゆ儏鍐典笅锛屽鏋滃悓鏃跺瓨鍦?FlashPoint 鍜?PCI MultiMaster 涓绘満
  閫傞厤鍣紝鏈┍鍔ㄤ細鍏堟帰娴?FlashPoint 涓绘満閫傞厤鍣紝闄ら潪 BIOS 涓荤鐩樼敱绗竴涓?  PCI MultiMaster 涓绘満閫傞厤鍣ㄦ帶鍒讹紝鍦ㄦ鎯呭喌涓嬩細鍏堟帰娴?MultiMaster 涓绘満閫傞厤鍣ㄣ€?
FlashPointFirst

  "FlashPointFirst" 閫夐」寮哄埗鍏堟帰娴?FlashPoint 涓绘満閫傞厤鍣紝鍐嶆帰娴?MultiMaster
  涓绘満閫傞厤鍣ㄣ€?
BusLogic 椹卞姩鐨勬爣璁伴槦鍒楅€夐」鍏佽鏄惧紡鎸囧畾闃熷垪娣卞害锛屼互鍙婃槸鍚︿负姣忎釜鐩爣璁惧
锛堝墠鎻愭槸璇ョ洰鏍囪澶囨敮鎸佹爣璁伴槦鍒楋級鍏佽鏍囪闃熷垪銆傞槦鍒楁繁搴︽槸鍏佽鍚屾椂鎻愪氦鎵ц
锛堟棤璁烘槸鎻愪氦缁欎富鏈洪€傞厤鍣ㄨ繕鏄洰鏍囪澶囷級鐨?SCSI 鍛戒护鏁伴噺銆傝娉ㄦ剰锛屾樉寮忓惎鐢?鏍囪闃熷垪鍙兘瀵艰嚧闂锛涘惎鐢ㄦ垨绂佺敤鏍囪闃熷垪鐨勯€夐」涓昏鏄负浜嗚閭ｄ簺鏈纭疄鐜?鏍囪闃熷垪鐨勭洰鏍囪澶囪兘澶熺鐢ㄥ畠銆傚彲鐢ㄩ€夐」濡備笅锛?
QueueDepth:<integer>

  "QueueDepth:" 鎴?"QD:" 閫夐」鎸囧畾鐢ㄤ簬鎵€鏈夋敮鎸佹爣璁伴槦鍒楃殑鐩爣璁惧鐨勯槦鍒楁繁搴︼紝
  浠ュ強鐢ㄤ簬涓嶆敮鎸佹爣璁伴槦鍒楃殑璁惧鐨勯槦鍒楁繁搴︿笂闄愩€傚鏋滄湭鎻愪緵闃熷垪娣卞害閫夐」锛?  闃熷垪娣卞害灏嗘牴鎹富鏈洪€傞厤鍣ㄧ殑鎬婚槦鍒楁繁搴︿互鍙婃墍妫€娴嬪埌鐨勭洰鏍囪澶囩殑鏁伴噺銆佺被鍨嬨€?  閫熷害鍜岃兘鍔涜嚜鍔ㄧ‘瀹氥€備笉鏀寔鏍囪闃熷垪鐨勭洰鏍囪澶囧叾闃熷垪娣卞害濮嬬粓琚涓?  BusLogic_UntaggedQueueDepth 鎴?BusLogic_UntaggedQueueDepthBB锛岄櫎闈炴彁渚涗簡
  鏇翠綆鐨勯槦鍒楁繁搴﹂€夐」銆傞槦鍒楁繁搴︿负 1 浼氳嚜鍔ㄧ鐢ㄦ爣璁伴槦鍒椼€?
QueueDepth:[<integer>,<integer>...]

  "QueueDepth:[...]" 鎴?"QD:[...]" 閫夐」涓烘瘡涓洰鏍囪澶囧崟鐙寚瀹氶槦鍒楁繁搴︺€傚鏋?  鐪佺暐鏌愪釜 <integer>锛岀浉搴旂殑鐩爣璁惧灏嗚嚜鍔ㄩ€夋嫨鍏堕槦鍒楁繁搴︺€?
TaggedQueuing:Default

  "TaggedQueuing:Default" 鎴?"TQ:Default" 閫夐」鏍规嵁 BusLogic 涓绘満閫傞厤鍣ㄧ殑鍥轰欢
  鐗堟湰锛屼互鍙婇槦鍒楁繁搴︽槸鍚﹀厑璁告帓闃熷涓懡浠わ紝鏉ュ喅瀹氭槸鍚﹀厑璁告爣璁伴槦鍒椼€?
TaggedQueuing:Enable

  "TaggedQueuing:Enable" 鎴?"TQ:Enable" 閫夐」瀵规湰涓绘満閫傞厤鍣ㄤ笂鐨勬墍鏈夌洰鏍囪澶?  鍚敤鏍囪闃熷垪锛岃鐩栦换浣曞師鏈細鍩轰簬涓绘満閫傞厤鍣ㄥ浐浠剁増鏈柦鍔犵殑闄愬埗銆?
TaggedQueuing:Disable

  "TaggedQueuing:Disable" 鎴?"TQ:Disable" 閫夐」瀵规湰涓绘満閫傞厤鍣ㄤ笂鐨勬墍鏈夌洰鏍囪澶?  绂佺敤鏍囪闃熷垪銆?
TaggedQueuing:<Target-Spec>

  "TaggedQueuing:<Target-Spec>" 鎴?"TQ:<Target-Spec>" 閫夐」涓烘瘡涓洰鏍囪澶囧崟鐙?  鎺у埗鏍囪闃熷垪銆?Target-Spec> 鏄竴涓?"Y"銆?N" 鍜?"X" 瀛楃銆?Y" 鍚敤鏍囪闃熷垪锛?  "N" 绂佺敤鏍囪闃熷垪锛?X" 鎺ュ彈鍩轰簬鍥轰欢鐗堟湰鐨勯粯璁ゅ€笺€傜涓€涓瓧绗︽寚鐩爣璁惧 0锛?  绗簩涓寚鐩爣璁惧 1锛屼互姝ょ被鎺紱濡傛灉 "Y"銆?N"銆?X" 瀛楃搴忓垪鏈鐩栨墍鏈夌洰鏍?  璁惧锛屾湭鎸囧畾鐨勫瓧绗﹀亣瀹氫负 "X"銆?
BusLogic 椹卞姩鐨勬潅椤归€夐」鍖呭惈濡備笅鍐呭锛?
BusSettleTime:<seconds>

  "BusSettleTime:" 鎴?"BST:" 閫夐」浠ョ涓哄崟浣嶆寚瀹氭€荤嚎绋冲畾鏃堕棿锛圔us Settle
  Time锛夈€傛€荤嚎绋冲畾鏃堕棿鏄寚鍦ㄤ笂涓€娆″彂璧?SCSI 鎬荤嚎澶嶄綅鐨勪富鏈洪€傞厤鍣ㄧ‖澶嶄綅锛屼笌
  鍙戝嚭浠讳綍 SCSI 鍛戒护涔嬮棿闇€瑕佺瓑寰呯殑鏃堕棿閲忋€傚鏋滄湭鎸囧畾锛岄粯璁や负
  BusLogic_DefaultBusSettleTime銆?
InhibitTargetInquiry

  "InhibitTargetInquiry" 閫夐」绂佹鍦?MultiMaster 涓绘満閫傞厤鍣ㄤ笂鎵ц鏌ヨ鐩爣璁惧
  锛圛nquire Target Devices锛夋垨鏌ヨ宸插畨瑁呰澶囷紙Inquire Installed Devices锛?  鍛戒护銆傚綋鏌愪簺杈冭€佺殑鐩爣璁惧鍦ㄥ鍧€閫昏緫鍗曞厓 0 浠ヤ笂鏃朵笉鍋氬嚭姝ｇ‘鍝嶅簲鏃讹紝杩欏彲鑳?  鏄繀瑕佺殑銆?
BusLogic 椹卞姩鐨勮皟璇曢€夐」鍖呭惈濡備笅鍐呭锛?
TraceProbe

  "TraceProbe" 閫夐」鍚敤瀵逛富鏈洪€傞厤鍣ㄦ帰娴嬬殑璺熻釜銆?
TraceHardwareReset

  "TraceHardwareReset" 閫夐」鍚敤瀵逛富鏈洪€傞厤鍣ㄧ‖浠跺浣嶇殑璺熻釜銆?
TraceConfiguration

  "TraceConfiguration" 閫夐」鍚敤瀵逛富鏈洪€傞厤鍣ㄩ厤缃殑璺熻釜銆?
TraceErrors

  "TraceErrors" 閫夐」鍚敤瀵逛粠鐩爣璁惧杩斿洖閿欒鐨?SCSI 鍛戒护鐨勮窡韪€傚浜庢瘡涓?  澶辫触鐨?SCSI 鍛戒护锛屽皢鎵撳嵃鍏?CDB 涓?Sense Data銆?
Debug

  "Debug" 閫夐」鍚敤鎵€鏈夎皟璇曢€夐」銆?
浠ヤ笅绀轰緥婕旂ず灏嗙涓€涓富鏈洪€傞厤鍣ㄤ笂鐩爣璁惧 1 鍜?2 鐨勯槦鍒楁繁搴﹁涓?7 鍜?15锛?灏嗙浜屼釜涓绘満閫傞厤鍣ㄤ笂鎵€鏈夌洰鏍囪澶囩殑闃熷垪娣卞害璁句负 31锛屽苟灏嗙浜屼釜涓绘満閫傞厤鍣ㄧ殑
鎬荤嚎绋冲畾鏃堕棿璁句负 30 绉掋€?
```

  linux BusLogic=QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30

```
```
  append = "BusLogic=QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30"

```
```
  insmod BusLogic.o \
      'BusLogic="QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30"'


```

      Module Utilities 2.1.71 鎴栨洿楂樼増鏈槸姝ｇ‘瑙ｆ瀽鍖呭惈閫楀彿鐨勯┍鍔ㄩ€夐」鎵€蹇呴渶鐨勩€?

## 椹卞姩瀹夎

鏈彂琛岀増鏄负 Linux 鍐呮牳鐗堟湰 2.0.35 鍑嗗鐨勶紝浣嗗簲涓?2.0.4 鎴栦换浣曟洿鏅氱殑 2.0
绯诲垪鍐呮牳鍏煎銆?
瑕佸畨瑁呮柊鐨?BusLogic SCSI 椹卞姩锛屼綘鍙互浣跨敤浠ヤ笅鍛戒护锛?```

  cd /usr/src
  tar -xvzf BusLogic-2.0.15.tar.gz
  mv README.* LICENSE.* BusLogic.[ch] FlashPoint.c linux/drivers/scsi
  patch -p0 < BusLogic.patch (only for 2.0.33 and below)
  cd linux
  make config
  make zImage

```
鐒跺悗灏?"arch/x86/boot/zImage" 瀹夎涓轰綘鐨勬爣鍑嗗唴鏍革紝濡傞€傜敤鍒欒繍琛?lilo锛屽苟
閲嶅惎銆?

## BusLogic 鍏憡閭欢鍒楄〃

BusLogic 鍏憡閭欢鍒楄〃鎻愪緵浜嗕竴涓鍧涳紝鐢ㄤ簬鍚?Linux 鐢ㄦ埛閫氬憡鏂扮殑椹卞姩鍙戝竷浠ュ強
鏈夊叧 BusLogic SCSI 涓绘満閫傞厤鍣?Linux 鏀寔鐨勫叾浠栧叕鍛娿€傝鍔犲叆閭欢鍒楄〃锛岃鍙戦€?涓€灏侀偖浠跺埌 "buslogic-announce-request@dandelion.com"锛屽苟鍦ㄩ偖浠舵鏂囦腑鍐欎笂
"subscribe" 涓€琛屻€?