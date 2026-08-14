
## Adaptec Ultra320 绯诲垪绠＄悊濂椾欢


Linux 鎿嶄綔绯荤粺鑷堪鏂囦欢


  1. 鏀寔鐨勭‖浠?  2. 鐗堟湰鍘嗗彶
  3. 鍛戒护琛岄€夐」
  4. 琛ュ厖璇存槑
  5. 鑱旂郴 Adaptec

## 1. 鏀寔鐨勭‖浠?

   鏈┍鍔ㄥ浠舵敮鎸佷互涓?Adaptec SCSI 涓绘満閫傞厤鍣ㄣ€?
   =============              =========================================
   Ultra320 ASIC              Description
   =============              =========================================
   AIC-7901A                  Single Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC
   AIC-7901B                  Single Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC with Retained Training
   AIC-7902A4                 Dual Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC
   AIC-7902B                  Dual Channel 64-bit PCI-X 133MHz to
                              Ultra320 SCSI ASIC with Retained Training
   =============              =========================================

   ========================== ===================================== ============
   Ultra320 Adapters          Description                              ASIC
   ========================== ===================================== ============
   Adaptec SCSI Card 39320    Dual Channel 64-bit PCI-X 133MHz to   7902A4/7902B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin)
   Adaptec SCSI Card 39320A   Dual Channel 64-bit PCI-X 133MHz to      7902B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin)
   Adaptec SCSI Card 39320D   Dual Channel 64-bit PCI-X 133MHz to      7902A4
                              Ultra320 SCSI Card (two external VHDC
                              and one internal 68-pin)
   Adaptec SCSI Card 39320D   Dual Channel 64-bit PCI-X 133MHz to      7902A4
                              Ultra320 SCSI Card (two external VHDC
                              and one internal 68-pin) based on the
                              AIC-7902B ASIC
   Adaptec SCSI Card 29320    Single Channel 64-bit PCI-X 133MHz to    7901A
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin, one
                              internal 50-pin)
   Adaptec SCSI Card 29320A   Single Channel 64-bit PCI-X 133MHz to    7901B
                              Ultra320 SCSI Card (one external
                              68-pin, two internal 68-pin, one
                              internal 50-pin)
   Adaptec SCSI Card 29320LP  Single Channel 64-bit Low Profile        7901A
                              PCI-X 133MHz to Ultra320 SCSI Card
                              (One external VHDC, one internal
                              68-pin)
   Adaptec SCSI Card 29320ALP Single Channel 64-bit Low Profile        7901B
                              PCI-X 133MHz to Ultra320 SCSI Card
                              (One external VHDC, one internal
                              68-pin)
   ========================== ===================================== ============

## 2. 鐗堟湰鍘嗗彶


 - 3.0	  (2005 骞?12 鏈?1 鏃?
 - 鏇存柊椹卞姩浠ヤ娇鐢?SCSI transport class 鍩虹璁炬柦
 - 浠?Adaptec 鍙戝竷鐨?2.0.15 鐗堥┍鍔ㄤ腑鎻愬彇搴忓垪鍣ㄥ拰鏍稿績淇

 - 1.3.11 (2003 骞?7 鏈?11 鏃?
        - 淇鑻ュ共姝婚攣闂銆?        - 娣诲姞 29320ALP 鍜?39320B 鐨?ID銆?
 - 1.3.10 (2003 骞?6 鏈?3 鏃?
        - 灏?SCB_TAG 瀛楁瀵归綈鍒?16 瀛楄妭杈圭晫銆傝繖閬垮厤浜嗗湪鏌愪簺
          PCI-33 鎬荤嚎涓婂嚭鐜?SCB 鎹熷潖銆?        - 淇 Rev B. 纭欢涓婄殑闈為浂 lun銆?        - 閽堝 2.5.X SCSI proc FS 鎺ュ彛鐨勬敼鍔ㄨ繘琛屾洿鏂般€?        - 褰撻€氳繃 8bit WDTR 娑堟伅鍗忓晢涓哄紓姝ユ椂锛屽彂閫佷竴涓亸绉婚噺涓?0 鐨?          SDTR锛屼互纭繚鐩爣绔煡閬撴垜浠浜庡紓姝ユā寮忋€傝繖缁曡繃浜?          Quantum Atlas 10K 鍥轰欢鐨勪竴涓己闄枫€?        - 瀹炵幇鎺у埗鍣ㄧ殑鎸傝捣鍜屾仮澶嶃€?        - 鍦ㄩ┍鍔ㄦ寕杞芥湡闂存竻闄?PCI 閿欒鐘舵€侊紝浠ュ厤鐢变簬鍦ㄦ垜浠０鏄?          鎺у埗鍣ㄤ箣鍓嶅叾浠栭┍鍔ㄦ帰娴嬩骇鐢熺殑鏉傛暎鍐欐搷浣滆€屽鑷?          鍐呭瓨鏄犲皠 I/O 琚鐢ㄣ€?
 - 1.3.9 (2003 骞?5 鏈?22 鏃?
        - 淇缂栬瘧鍣ㄩ敊璇€?        - 绉婚櫎瀵硅法瓒?4GB 杈圭晫鐨勬杩涜 S/G 鎷嗗垎銆傚湪 Linux 涓?          淇濊瘉涓嶄細鍙戠敓杩欑鎯呭喌銆?        - 娣诲姞瀵?2.5.X 鍐呮牳涓?scsi_report_device_reset() 鐨勬敮鎸併€?        - 娣诲姞 7901B 鏀寔銆?        - 绠€鍖栨墦鍖?lun Rev A workaround 鐨勫鐞嗐€?        - 淇骞剁畝鍖栧蹇界暐瀹芥畫宸紙ignore wide residue锛夋秷鎭殑澶勭悊銆?          涔嬪墠鐨勪唬鐮佸湪浜嬪姟鏁版嵁闀垮害涓哄伓鏁颁笖鎴戜滑鏀跺埌 IWR 娑堟伅鏃?          浼氭棤娉曟姤鍛婃畫宸€?
 - 1.3.8 (2003 骞?4 鏈?29 鏃?
        - 淇閫氳繃鍛戒护琛屾帴鍙ｄ唬鐮佽闂殑绫诲瀷銆?        - 鎵ц鑻ュ共鍥轰欢浼樺寲銆?        - 淇 "Unexpected PKT busfree" 閿欒銆?        - 浣跨敤搴忓垪鍣ㄤ腑鏂潵閫氱煡涓绘満瀛樺湪鐘舵€侀敊璇殑鍛戒护銆傛垜浠皢
          閫氱煡鎺ㄨ繜鍒版病鏈夋湭鍐抽€夋嫨鏃讹紝浠ョ‘淇濅富鏈鸿涓柇鐨勬椂闂村敖鍙兘鐭€?        - 绉婚櫎瀵?2.2.X 涔嬪墠鐗堟湰鐨勬敮鎸併€?        - 娣诲姞瀵规柊鐨?2.5.X 涓柇 API 鐨勬敮鎸併€?        - 淇澶х鏋舵瀯鏀寔銆?
 - 1.3.7 (2003 骞?4 鏈?16 鏃?
        - 浣跨敤 del_timer_sync() 纭繚鍦ㄦ帶鍒跺櫒鍏抽棴鏈熼棿娌℃湁
          寰呭鐞嗙殑瓒呮椂銆?        - 瀵逛簬 2.5.X 涔嬪墠鐨勫唴鏍革紝浠旂粏璋冩暣鎴戜滑鐨勬鍒楄〃澶у皬锛屼互閬垮厤
          SCSI malloc 姹犵鐗囧寲銆?        - 娓呯悊 /proc 杈撳嚭涓殑閫氶亾鏄剧ず銆?        - 鍦?add-single-device 鏈熼棿缁曡繃涓棿灞傝澶囧垪琛ㄤ腑閲嶅鐨?          璁惧鏉＄洰銆?
 - 1.3.6 (2003 骞?3 鏈?28 鏃?
        - 淇 Domain Validation 浠ｇ爜涓殑鍙岄噸閲婃斁銆?        - 淇鎺у埗鍣ㄥ叧闂湡闂村宸查噴鏀惧唴瀛樼殑寮曠敤銆?        - 鍦?SE->LVD 鍒囨崲鏃跺浣嶆€荤嚎銆傝繖鏄负閲嶇疆鎴戜滑鐨勬敹鍙戝櫒鎵€蹇呴』鐨勩€?
 - 1.3.5 (2003 骞?3 鏈?24 鏃?
        - 淇鑻ュ共瀵勫瓨鍣ㄧ獥鍙ｆā寮?bug銆?        - 鍦ㄦ垜浠瘖鏂互鍙?/proc 涓樉绀虹殑 PPR 鏍囧織閲屽寘鍚娴佸紡銆?        - 娣诲姞瀵?2.5.X 鍐呮牳鐨?PCI 鐑彃鎷旀敮鎸併€?        - 淇 RevA 纭欢鐨勯粯璁ら琛ュ伩鍊笺€?        - 淇 Domain Validation 绾跨▼鍏抽棴銆?        - 娣诲姞涓€涓浐浠?workaround锛屼娇 H2A4 涓婃墦鍖呮搷浣滄湡闂寸殑
          LED 闂儊鏇翠寒銆?        - 淇鐢ㄦ埛璇绘祦寮忚缃殑 /proc 鏄剧ず銆?        - 閫氳繃浠庝腑闂村眰杩涘叆椹卞姩鏃堕噴鏀?io_request_lock 鏉ョ畝鍖?          椹卞姩鍔犻攣銆?        - 娓呯悊鍛戒护琛岃В鏋愶紝骞跺皢澶ч儴鍒嗕唬鐮佺Щ鑷?aiclib銆?
 - 1.3.4 (2003 骞?2 鏈?28 鏃?
        - 淇閿欒鎭㈠澶勭悊绋嬪簭涓殑绔炴€佹潯浠躲€?        - 鍏佽鍦?Domain Validation 鏈熼棿 Test Unit Ready 鍛戒护鍗犵敤瀹屾暣鐨?5 绉掋€?
 - 1.3.2 (2003 骞?2 鏈?19 鏃?
        - 淇鐢变簬 1.3.1 涓寘鍚殑 GEM318 鍏煎鎬т慨澶嶅鑷寸殑 Rev B. 鍥炲綊銆?
 - 1.3.1 (2003 骞?2 鏈?11 鏃?
        - 娣诲姞瀵?39320A 鐨勬敮鎸併€?        - 鏀硅繘瀵规煇浜?PCI-X 閿欒鐨勬仮澶嶃€?        - 淇瀵瑰悓涓€鍐欏叆浜嬪姟涓彲鑳藉嚭鐜扮殑銆佷腑闂存病鏈夎缁冪殑
          LQ/DATA/LQ/DATA 鐨勫鐞嗐€?        - 淇涓?GEM318 鏈虹鏈嶅姟璁惧鐨勫吋瀹规€ч棶棰樸€?        - 淇鍦ㄩ珮鏍囩娣卞害鍐欒礋杞戒笅鍑虹幇鐨勬暟鎹崯鍧忛棶棰樸€?        - 閫傞厤 2.5.X daemonize() API 鐨勫彉鏇淬€?        - 淇 "Missing case in ahd_handle_scsiint" 鎭愭厡銆?
 - 1.3.0 (2003 骞?1 鏈?21 鏃?
        - 瀹屾垚鎵€鏈?U320 浜у搧鐨勫畬鏁村洖褰掓祴璇曘€?        - 娣诲姞 abort 鍜岀洰鏍?lun 澶嶄綅閿欒鎭㈠澶勭悊绋嬪簭浠ュ強
          涓柇鑱氬悎锛坕nterrupt coalescing锛夈€?
 - 1.2.0 (2002 骞?11 鏈?14 鏃?
        - 娣诲姞瀵?Domain Validation 鐨勬敮鎸?        - 娣诲姞瀵规儬鏅紙Hewlett-Packard锛夌増鏈殑 39320D 鍜?AIC-7902
          閫傞厤鍣ㄧ殑鏀寔銆?
        瀵逛箣鍓嶉€傞厤鍣ㄧ殑鏀寔灏氭湭缁忚繃瀹屾暣娴嬭瘯锛屽簲浠呭湪瀹㈡埛鑷鎵挎媴
        椋庨櫓鐨勬儏鍐典笅浣跨敤銆?
 - 1.1.1 (2002 骞?9 鏈?24 鏃?
        - 娣诲姞瀵?Linux 2.5.X 鍐呮牳绯诲垪鐨勬敮鎸?
 - 1.1.0 (2002 骞?9 鏈?17 鏃?
        - 娣诲姞瀵瑰彟澶栧洓绉?SCSI 浜у搧鐨勬敮鎸侊細
          ASC-39320銆丄SC-29320銆丄SC-29320LP銆丄IC-7901銆?
 - 1.0.0 (2002 骞?5 鏈?30 鏃?
        - 椹卞姩鍒濆鍙戝竷銆?
 - 2.1. 杞欢/纭欢鐗规€?        - 鏀寔 SPI-4 "Ultra320" 鏍囧噯锛?          - 320MB/s 浼犺緭閫熺巼
          - 160MB/s 鍜?320MB/s 鐨勬墦鍖?SCSI 鍗忚
          - 蹇€熶徊瑁侀€夋嫨锛圦AS锛?          - 淇濈暀璁粌淇℃伅锛堜粎 Rev B. ASIC锛?        - 涓柇鑱氬悎锛圛nterrupt Coalescing锛?        - 鍙戣捣鑰呮ā寮忥紙鐩墠涓嶆敮鎸佺洰鏍囨ā寮忥級
        - 鏀寔鏈€楂?133MHz 鐨?PCI-X 鏍囧噯
        - 鏀寔 PCI v2.2 鏍囧噯
        - Domain Validation

 - 2.2. 鎿嶄綔绯荤粺鏀寔锛?        - Redhat Linux 7.2銆?.3銆?.0銆丄dvanced Server 2.1
        - SuSE Linux 7.3銆?.0銆?.1銆丒nterprise Server 7
        - 鐩墠浠呮敮鎸?Intel 鍜?AMD x86
        - 鏀寔 >4GB 鍐呭瓨閰嶇疆銆?
     鏇村璇︽儏璇峰弬闃呯敤鎴锋寚鍗椼€?
## 3. 鍛戒护琛岄€夐」


```

	         ALTERING OR ADDING THESE DRIVER PARAMETERS
                 INCORRECTLY CAN RENDER YOUR SYSTEM INOPERABLE.
                 USE THEM WITH CAUTION.

   Put a .conf file in the /etc/modprobe.d/ directory and add/edit a
   line containing ``options aic79xx aic79xx=[command[,command...]]`` where
   ``command`` is one or more of the following:


```
verbose
    :Definition: 鍦ㄩ┍鍔ㄨ繍琛屾湡闂村惎鐢ㄩ澶栫殑淇℃伅鎬ф秷鎭€?    :Possible Values: 璇ラ€夐」涓轰竴涓爣蹇?    :Default Value: 绂佺敤

debug:[value]
    :Definition: 鍚敤涓嶅悓绾у埆鐨勮皟璇曚俊鎭€?                 璋冭瘯鎺╃爜鐨勪綅瀹氫箟鍙互鍦?drivers/scsi/aic7xxx/aic79xx.h
                 鐨?"Debug" 鏍囬涓嬫壘鍒般€?    :Possible Values: 0x0000 = 鏃犺皟璇曪紝0xffff = 瀹屾暣璋冭瘯
    :Default Value: 0x0000

no_reset
    :Definition: 鍦ㄥ垵濮嬫帰娴嬮樁娈典笉澶嶄綅鎬荤嚎
    :Possible Values: 璇ラ€夐」涓轰竴涓爣蹇?    :Default Value: 绂佺敤

extended
    :Definition: 鍦ㄦ帶鍒跺櫒涓婂己鍒朵娇鐢ㄦ墿灞曡浆鎹?    :Possible Values: 璇ラ€夐」涓轰竴涓爣蹇?    :Default Value: 绂佺敤

periodic_otag
    :Definition: 鍛ㄦ湡鎬у彂閫佷竴涓湁搴忔爣绛句互闃叉鏍囩楗ラタ銆傛煇浜涜緝鏃х殑璁惧闇€瑕併€?    :Possible Values: 璇ラ€夐」涓轰竴涓爣蹇?    :Default Value: 绂佺敤

reverse_scan
    :Definition: 浠ュ弽鍚戦『搴忔帰娴?scsi 鎬荤嚎锛屼粠鐩爣 15 寮€濮?    :Possible Values: 璇ラ€夐」涓轰竴涓爣蹇?    :Default Value: 绂佺敤

global_tag_depth
    :Definition: 鎵€鏈夋€荤嚎涓婃墍鏈夌洰鏍囩殑鍏ㄥ眬鏍囩娣卞害銆?		 璇ラ€夐」璁剧疆榛樿鏍囩娣卞害锛屽彲琚?		 tag_info 閫夐」鏈夐€夋嫨鍦拌鐩栥€?
    :Possible Values: 1 - 253
    :Default Value: 32

tag_info:{{value[,value...]}[,{value[,value...]}...]}
    :Definition: 鍩轰簬姣忎釜鎺у埗鍣ㄨ缃瘡涓洰鏍囩殑鏍囪闃熷垪娣卞害銆傛帶鍒跺櫒鍜岀洰鏍?                 鍧囧彲鐪佺暐锛岃〃绀哄畠浠簲淇濈暀榛樿鏍囩娣卞害銆?
    :Possible Values: 1 - 253
    :Default Value: 32

    Examples:


```

	    tag_info:{{16,32,32,64,8,8,,32,32,32,32,32,32,32,32,32}

	鍦ㄦ帶鍒跺櫒 0 涓?
	    - 涓虹洰鏍?0 鎸囧畾鏍囩娣卞害 16
	    - 涓虹洰鏍?3 鎸囧畾鏍囩娣卞害 64
	    - 涓虹洰鏍?4 鍜?5 鎸囧畾鏍囩娣卞害 8
	    - 鐩爣 6 淇濈暀榛樿鍊?	    - 涓虹洰鏍?1,2,7-15 鎸囧畾鏍囩娣卞害 32

	鎵€鏈夊叾浠栫洰鏍囦繚鐣欓粯璁ゆ繁搴︺€?
	::

	    tag_info:{{},{32,,32}}

	鍦ㄦ帶鍒跺櫒 1 涓?
	    - 涓虹洰鏍?0 鍜?2 鎸囧畾鏍囩娣卞害 32

	鎵€鏈夊叾浠栫洰鏍囦繚鐣欓粯璁ゆ繁搴︺€?

```
rd_strm: {rd_strm_bitmask[,rd_strm_bitmask...]}
    :Definition: 鍩轰簬姣忎釜鐩爣鍚敤璇绘祦寮忋€?		 rd_strm_bitmask 鏄竴涓?16 浣嶅崄鍏繘鍒跺€硷紝鍏朵腑
		 姣忎竴浣嶄唬琛ㄤ竴涓洰鏍囥€傚皢璇ョ洰鏍囩殑浣嶈涓?'1' 鍗充负璇?		 鐩爣鍚敤璇绘祦寮忋€傛帶鍒跺櫒鍙互鐪佺暐锛岃〃绀哄畠浠簲淇濈暀
		 榛樿鐨勮娴佸紡璁剧疆銆?
    Examples:

```

		rd_strm:{0x0041}

	    鍦ㄦ帶鍒跺櫒 0 涓?
		- 涓虹洰鏍?0 鍜?6 鍚敤璇绘祦寮忋€?		- 瀵圭洰鏍?1-5,7-15 绂佺敤璇绘祦寮忋€?
	    鎵€鏈夊叾浠栫洰鏍囦繚鐣欓粯璁ょ殑璇绘祦寮忚缃€?
	    ::

		rd_strm:{0x0023,,0xFFFF}

	    鍦ㄦ帶鍒跺櫒 0 涓?
		- 涓虹洰鏍?1銆? 鍜?5 鍚敤璇绘祦寮忋€?		- 瀵圭洰鏍?3銆?銆?-15 绂佺敤璇绘祦寮忋€?
	    鍦ㄦ帶鍒跺櫒 2 涓?
		- 涓烘墍鏈夌洰鏍囧惎鐢ㄨ娴佸紡銆?
	    鎵€鏈夊叾浠栫洰鏍囦繚鐣欓粯璁ょ殑璇绘祦寮忚缃€?
    :Possible Values: 0x0000 - 0xffff
    :Default Value: 0x0000

```
dv: {value[,value...]}
    :Definition: 鍩轰簬姣忎釜鎺у埗鍣ㄨ缃?Domain Validation 绛栫暐銆?                 鎺у埗鍣ㄥ彲浠ョ渷鐣ワ紝琛ㄧず瀹冧滑搴斾繚鐣欓粯璁ょ殑璇绘祦寮忚缃€?
     :Possible Values:

		      ==== ===============================
		       < 0 浣跨敤鏉ヨ嚜涓茶 EEPROM 鐨勮缃€?                         0 绂佺敤 DV
		       > 0 鍚敤 DV
		      ==== ===============================

    :Default Value: DV 涓茶 EEPROM 閰嶇疆璁剧疆銆?
    Example:

```

	    dv:{-1,0,,1,1,0}

	- 鍦ㄦ帶鍒跺櫒 0 涓婁繚鎸?DV 涓洪粯璁よ缃€?	- 鍦ㄦ帶鍒跺櫒 1 涓婄鐢?DV銆?	- 璺宠繃鎺у埗鍣?2 鐨勯厤缃€?	- 鍦ㄦ帶鍒跺櫒 3 鍜?4 涓婂惎鐢?DV銆?	- 鍦ㄦ帶鍒跺櫒 5 涓婄鐢?DV銆?
```
seltime:[value]
    :Definition: 鎸囧畾閫夋嫨瓒呮椂鍊?    :Possible Values: 0 = 256ms锛? = 128ms锛? = 64ms锛? = 32ms
    :Default Value: 0


    浠ヤ笅涓変釜閫夐」鍙兘鍦ㄦ妧鏈敮鎸佷唬琛ㄧ殑鎸囧涓嬫洿鏀广€?

precomp: {value[,value...]}
    :Definition: 鍩轰簬姣忎釜鎺у埗鍣ㄨ缃?IO Cell 棰勮ˉ鍋垮€笺€?                 鎺у埗鍣ㄥ彲浠ョ渷鐣ワ紝琛ㄧず瀹冧滑搴斾繚鐣欓粯璁ょ殑棰勮ˉ鍋胯缃€?
    :Possible Values: 0 - 7
    :Default Value: 闅忚姱鐗囦慨璁㈢増鏈€屼笉鍚?
    Examples:

```

	    precomp:{0x1}

	鍦ㄦ帶鍒跺櫒 0 涓婂皢棰勮ˉ鍋胯涓?1銆?
	::

	    precomp:{1,,7}

	- 鍦ㄦ帶鍒跺櫒 0 涓婂皢棰勮ˉ鍋胯涓?1銆?	- 鍦ㄦ帶鍒跺櫒 2 涓婂皢棰勮ˉ鍋胯涓?8銆?
```
slewrate: {value[,value...]}
    :Definition: 鍩轰簬姣忎釜鎺у埗鍣ㄨ缃?IO Cell 鍘嬫憜鐜囥€?                      鎺у埗鍣ㄥ彲浠ョ渷鐣ワ紝琛ㄧず瀹冧滑搴斾繚鐣欓粯璁ょ殑鍘嬫憜鐜囪缃€?
    :Possible Values: 0 - 15
    :Default Value: 闅忚姱鐗囦慨璁㈢増鏈€屼笉鍚?
    Examples:

```

	    slewrate:{0x1}

	- 鍦ㄦ帶鍒跺櫒 0 涓婂皢鍘嬫憜鐜囪涓?1銆?
	::

	    slewrate :{1,,8}

	- 鍦ㄦ帶鍒跺櫒 0 涓婂皢鍘嬫憜鐜囪涓?1銆?	- 鍦ㄦ帶鍒跺櫒 2 涓婂皢鍘嬫憜鐜囪涓?8銆?
```
amplitude: {value[,value...]}
    :Definition: 鍩轰簬姣忎釜鎺у埗鍣ㄨ缃?IO Cell 淇″彿骞呭害銆?                 鎺у埗鍣ㄥ彲浠ョ渷鐣ワ紝琛ㄧず瀹冧滑搴斾繚鐣欓粯璁ょ殑璇绘祦寮忚缃€?
    :Possible Values: 1 - 7
    :Default Value: 闅忚姱鐗囦慨璁㈢増鏈€屼笉鍚?
    Examples:

```

	amplitude:{0x1}

    鍦ㄦ帶鍒跺櫒 0 涓婂皢骞呭害璁句负 1銆?
    ::

	amplitude :{1,,7}

    - 鍦ㄦ帶鍒跺櫒 0 涓婂皢骞呭害璁句负 1銆?    - 鍦ㄦ帶鍒跺櫒 2 涓婂皢骞呭害璁句负 7銆?
```
```

    options aic79xx aic79xx=verbose,rd_strm:{{0x0041}}

```
鍦ㄩ┍鍔ㄤ腑鍚敤璇︾粏杈撳嚭锛屽苟涓烘帶鍒跺櫒 0 鐨勭洰鏍?0 鍜?6 鎵撳紑璇绘祦寮忋€?
## 4. 琛ュ厖璇存槑


### 4.1. 宸茬煡/鏈В鍐虫垨浠呬緵鍙傝€冪殑闂


        - 鍦?SuSE Linux Enterprise 7 涓嬶紝鐢变簬 Linux 鍐呮牳涓?PCI 涓柇璺敱鐨?          闂锛岄┍鍔ㄥ彲鑳芥棤娉曟纭繍琛屻€傝鑱旂郴 SuSE 鑾峰彇鏇存柊鐨?Linux 鍐呮牳銆?
### 4.2. 绗笁鏂瑰吋瀹规€ч棶棰?

        - Adaptec 浠呮敮鎸佽繍琛屾渶鏂板彲鐢ㄥ浐浠剁殑 Ultra320 纭洏銆傝涓庢偍鐨勭‖鐩?          鍒堕€犲晢纭鎮ㄦ嫢鏈夋渶鏂扮増鏈€?
### 4.3. 鎿嶄綔绯荤粺鎴栨妧鏈檺鍒?

        - PCI 鐑彃鎷旀湭缁忔祴璇曪紝鍙兘瀵艰嚧鎿嶄綔绯荤粺鍋滄鍝嶅簲銆?        - 涓嶄粠 0 寮€濮嬭繛缁紪鍙风殑 lun 鍙兘鍦ㄧ郴缁熷惎鍔ㄦ湡闂翠笉浼氳鑷姩鎺㈡祴銆?          杩欐槸鎿嶄綔绯荤粺鐨勯檺鍒躲€傝鑱旂郴鎮ㄧ殑 Linux 鍙戣鍟嗕互鑾峰彇鎵嬪姩鎺㈡祴
          闈炶繛缁?lun 鐨勮鏄庛€?        - 鍦?RedHat 涓嬫搷浣滅郴缁熷畨瑁呮湡闂翠娇鐢ㄦ湰杞欢鍖呯殑椹卞姩鏇存柊鐩樼増鏈紝鍙兘
          瀵艰嚧绯荤粺妯″潡鐩綍涓畨瑁呬簡鏈┍鍔ㄧ殑涓や釜鐗堟湰銆傝繖鍙兘寮曡捣
          /sbin/mkinitrd 绋嬪簭鍜?鎴栧皾璇曞畨瑁呯郴缁熷寘鐨勫叾浠?RPM 鍖呭嚭鐜伴棶棰樸€?          绯荤粺杩愯鍚庣籂姝ｆ闂鐨勬渶浣虫柟娉曟槸瀹夎鏈┍鍔ㄧ殑鏈€鏂?RPM 鍖呯増鏈紝
          鍙粠 http://www.adaptec.com 鑾峰彇銆?

## 5. Adaptec 瀹㈡埛鏀寔


   鐢宠 Adaptec 鎶€鏈敮鎸侀渶瑕佷竴涓妧鏈敮鎸佹爣璇嗭紙TSID锛夌紪鍙枫€?
    - 12 浣嶇殑 TSID 鍙湪浜у搧鍖呰鐩掑唴鎵€闄勭殑鐧借壊鏉″舰鐮佹爣绛句笂鎵惧埌銆俆SID 鍙?      閫氳繃鍑嗙‘璇嗗埆鎮ㄧ殑浜у搧鍜屾敮鎸佺姸鎬侊紝甯姪鎴戜滑鎻愪緵鏇撮珮鏁堢殑鏈嶅姟銆?
   鏀寔閫夐」
    - 鍦?http://ask.adaptec.com 鎼滅储 Adaptec 鏀寔鐭ヨ瘑搴擄紙ASK锛夛紝鑾峰彇鏈夊叧
      鎮ㄤ骇鍝佺殑鏂囩珷銆佹晠闅滄帓闄ゆ妧宸у拰甯歌闂瑙ｇ瓟銆?    - 濡傞渶閫氳繃鐢靛瓙閭欢鑾峰緱鏀寔锛岃鍦?http://ask.adaptec.com/ 鍚?Adaptec 鐨?      鎶€鏈敮鎸佷笓瀹舵彁浜ゆ偍鐨勯棶棰樸€?
   鍖楃編
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.com/銆?    - 鏈夊叧 Adaptec 鏀寔閫夐」鐨勪俊鎭紝璇疯嚧鐢?408-957-2550锛屾瘡澶?24 灏忔椂锛?      姣忓懆 7 澶┿€?    - 濡傞渶涓庢妧鏈敮鎸佷笓瀹堕€氳瘽锛?
      - 纭欢浜у搧锛岃鑷寸數 408-934-7274锛屽懆涓€鑷冲懆浜旓紝澶钩娲嬪浠ゆ椂
        鍑屾櫒 3:00 鑷充笅鍗?5:00銆?      - RAID 鍜屽厜绾ら€氶亾浜у搧锛岃鑷寸數 321-207-2000锛屽懆涓€鑷冲懆浜旓紝澶钩娲嬪浠ゆ椂
        鍑屾櫒 3:00 鑷充笅鍗?5:00銆?
      涓哄姞蹇湇鍔￠€熷害锛岃鍑嗗濂芥偍鐨勮绠楁満銆?    - 璁㈣喘 Adaptec 浜у搧锛堝寘鎷厤浠跺拰绾跨紗锛夛紝璇疯嚧鐢?408-957-7274銆傚湪绾胯璐?      绾跨紗璇疯闂?http://www.adaptec.com/buy-cables/銆?
   娆ф床
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.com/en-US/_common/world_index銆?    - 濡傞渶涓庢妧鏈敮鎸佷笓瀹堕€氳瘽锛岃鑷寸數鎴栧彂鐢靛瓙閭欢锛?
      - 寰疯锛?+49 89 4366 5522锛屽懆涓€鑷冲懆浜旓紝涓鏃堕棿 9:00-17:00锛?        http://ask-de.adaptec.com/銆?      - 娉曡锛?+49 89 4366 5533锛屽懆涓€鑷冲懆浜旓紝涓鏃堕棿 9:00-17:00锛?	http://ask-fr.adaptec.com/銆?      - 鑻辫锛?+49 89 4366 5544锛屽懆涓€鑷冲懆浜旓紝鏍兼灄灏兼不鏍囧噯鏃堕棿 9:00-17:00锛?	http://ask.adaptec.com/銆?
    - 鎮ㄥ彲浠ュ湪绾胯璐?Adaptec 绾跨紗锛?      http://www.adaptec.com/buy-cables/銆?
   鏃ユ湰
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.co.jp/銆?    - 濡傞渶涓庢妧鏈敮鎸佷笓瀹堕€氳瘽锛岃鑷寸數 +81 3 5308 6120锛屽懆涓€鑷冲懆浜旓紝
      涓婂崍 9:00 鑷?12:00锛屼笅鍗?1:00 鑷?6:00銆?
Copyright |copy| 2003 Adaptec Inc. 691 S. Milpitas Blvd., Milpitas CA 95035 USA.
All rights reserved.

You are permitted to redistribute, use and modify this README file in whole
or in part in conjunction with redistribution of software governed by the
General Public License, provided that the following conditions are met:

1. Redistributions of README file must retain the above copyright
   notice, this list of conditions, and the following disclaimer,
   without modification.
2. The name of the author may not be used to endorse or promote products
   derived from this software without specific prior written permission.
3. Modifications or new contributions must be attributed in a copyright
   notice identifying the author ("Contributor") and added below the
   original copyright notice. The copyright notice is for purposes of
   identifying contributors and should not be deemed as permission to alter
   the permissions given by Adaptec.

THIS README FILE IS PROVIDED BY ADAPTEC AND CONTRIBUTORS `AS IS` AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, ANY
WARRANTIES OF NON-INFRINGEMENT OR THE IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
ADAPTEC OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS README
FILE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
