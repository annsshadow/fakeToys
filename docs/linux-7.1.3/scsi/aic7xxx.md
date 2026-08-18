
## Adaptec Aic7xxx Fast -> Ultra160 Family Manager Set v7.0


Linux 鎿嶄綔绯荤粺 README

鏈枃浠朵腑鍖呭惈浠ヤ笅淇℃伅锛?
  1. 鏀寔鐨勭‖浠?  2. 鐗堟湰鍘嗗彶
  3. 鍛戒护琛岄€夐」
  4. 鑱旂郴 Adaptec

## 1. 鏀寔鐨勭‖浠?

   aic7xxx 椹卞姩鏀寔浠ヤ笅 Adaptec SCSI 鑺墖涓庝富鏈洪€傞厤鍣ㄣ€?
   ======== ===== ========= ======== ========= ===== ===============
   Chip     MIPS  Host Bus  MaxSync  MaxWidth  SCBs  Notes
   ======== ===== ========= ======== ========= ===== ===============
   aic7770  10    EISA/VL   10MHz    16Bit      4    1
   aic7850  10    PCI/32    10MHz    8Bit       3
   aic7855  10    PCI/32    10MHz    8Bit       3
   aic7856  10    PCI/32    10MHz    8Bit       3
   aic7859  10    PCI/32    20MHz    8Bit       3
   aic7860  10    PCI/32    20MHz    8Bit       3
   aic7870  10    PCI/32    10MHz    16Bit      16
   aic7880  10    PCI/32    20MHz    16Bit      16
   aic7890  20    PCI/32    40MHz    16Bit      16      3 4 5 6 7 8
   aic7891  20    PCI/64    40MHz    16Bit      16      3 4 5 6 7 8
   aic7892  20    PCI/64-66 80MHz    16Bit      16      3 4 5 6 7 8
   aic7895  15    PCI/32    20MHz    16Bit      16    2 3 4 5
   aic7895C 15    PCI/32    20MHz    16Bit      16    2 3 4 5     8
   aic7896  20    PCI/32    40MHz    16Bit      16    2 3 4 5 6 7 8
   aic7897  20    PCI/64    40MHz    16Bit      16    2 3 4 5 6 7 8
   aic7899  20    PCI/64-66 80MHz    16Bit      16    2 3 4 5 6 7 8
   ======== ===== ========= ======== ========= ===== ===============

   1. 澶氳矾澶嶇敤鍙岄€氶亾璁惧 - 鍗曚釜鎺у埗鍣ㄦ湇鍔′袱鏉℃€荤嚎銆?   2. 澶氬姛鑳藉弻閫氶亾璁惧 - 鍗曡姱鐗囦笂闆嗘垚涓や釜鎺у埗鍣ㄣ€?   3. 鍛戒护閫氶亾娆＄骇 DMA 寮曟搸 - 鍏佽鍒嗘暎/鑱氶泦鍒楄〃涓?SCB 棰勫彇銆?   4. 64 瀛楄妭 SCB 鏀寔 - 鍏佽涓烘墍鏈夊彲鑳界殑鐩爣/lun 缁勫悎寤虹珛鏂紑銆佹棤鏍囩璇锋眰琛ㄣ€?   5. 鍧楃Щ鍔ㄦ寚浠ゆ敮鎸?- 浣挎煇浜涙椂搴忓櫒鎿嶄綔閫熷害缈诲€嶃€?   6. 'Bayonet' 椋庢牸鍒嗘暎/鑱氶泦寮曟搸 - 鎻愬崌 S/G 棰勫彇鎬ц兘銆?   7. 鎺掗槦瀵勫瓨鍣?- 鍏佽鍦ㄤ笉鏆傚仠鏃跺簭鍣ㄧ殑鎯呭喌涓嬫帓闃熸柊浜嬪姟銆?   8. 澶氱洰鏍?ID - 鍏佽鎺у埗鍣ㄤ綔涓虹洰鏍囧湪澶氫釜 SCSI ID 涓婂搷搴旈€夋嫨銆?
   ============== ======= =========== =============== =============== =========
   Controller      Chip   Host-Bus    Int-Connectors  Ext-Connectors  Notes
   ============== ======= =========== =============== =============== =========
   AHA-274X[A]    aic7770   EISA         SE-50M         SE-HD50F
   AHA-274X[A]W   aic7770   EISA         SE-HD68F       SE-HD68F
                                         SE-50M
   AHA-274X[A]T   aic7770   EISA       2 X SE-50M       SE-HD50F
   AHA-2842       aic7770    VL          SE-50M         SE-HD50F
   AHA-2940AU     aic7860   PCI/32       SE-50M         SE-HD50F
   AVA-2902I      aic7860   PCI/32       SE-50M
   AVA-2902E      aic7860   PCI/32       SE-50M
   AVA-2906       aic7856   PCI/32       SE-50M         SE-DB25F
   APC-7850       aic7850   PCI/32       SE-50M                       1
   AVA-2940       aic7860   PCI/32       SE-50M
   AHA-2920B      aic7860   PCI/32       SE-50M
   AHA-2930B      aic7860   PCI/32       SE-50M
   AHA-2920C      aic7856   PCI/32       SE-50M         SE-HD50F
   AHA-2930C      aic7860   PCI/32       SE-50M
   AHA-2930C      aic7860   PCI/32       SE-50M
   AHA-2910C      aic7860   PCI/32       SE-50M
   AHA-2915C      aic7860   PCI/32       SE-50M
   AHA-2940AU/CN  aic7860   PCI/32       SE-50M         SE-HD50F
   AHA-2944W      aic7870   PCI/32     HVD-HD68F        HVD-HD68F
                                       HVD-50M
   AHA-3940W      aic7870   PCI/32     2 X SE-HD68F     SE-HD68F        2
   AHA-2940UW     aic7880   PCI/32       SE-HD68F
                                         SE-50M         SE-HD68F
   AHA-2940U      aic7880   PCI/32       SE-50M         SE-HD50F
   AHA-2940D      aic7880   PCI/32
   aHA-2940 A/T   aic7880   PCI/32
   AHA-2940D A/T  aic7880   PCI/32
   AHA-3940UW     aic7880   PCI/32     2 X SE-HD68F     SE-HD68F          3
   AHA-3940UWD    aic7880   PCI/32     2 X SE-HD68F   2 X SE-VHD68F       3
   AHA-3940U      aic7880   PCI/32     2 X SE-50M       SE-HD50F          3
   AHA-2944UW     aic7880   PCI/32      HVD-HD68F       HVD-HD68F
                                        HVD-50M
   AHA-3944UWD    aic7880   PCI/32     2 X HVD-HD68F  2 X HVD-VHD68F      3
   AHA-4944UW     aic7880   PCI/32
   AHA-2930UW     aic7880   PCI/32
   AHA-2940UW Pro aic7880   PCI/32      SE-HD68F        SE-HD68F            4
                                        SE-50M
   AHA-2940UW/CN  aic7880   PCI/32
   AHA-2940UDual  aic7895   PCI/32
   AHA-2940UWDual aic7895   PCI/32
   AHA-3940UWD    aic7895   PCI/32
   AHA-3940AUW    aic7895   PCI/32
   AHA-3940AUWD   aic7895   PCI/32
   AHA-3940AU     aic7895   PCI/32
   AHA-3944AUWD   aic7895   PCI/32     2 X HVD-HD68F  2 X HVD-VHD68F
   AHA-2940U2B    aic7890   PCI/32      LVD-HD68F       LVD-HD68F
   AHA-2940U2 OEM aic7891   PCI/64
   AHA-2940U2W    aic7890   PCI/32      LVD-HD68F       LVD-HD68F
                                        SE-HD68F
                                        SE-50M
   AHA-2950U2B    aic7891   PCI/64      LVD-HD68F       LVD-HD68F
   AHA-2930U2     aic7890   PCI/32      LVD-HD68F       SE-HD50F
                                        SE-50M
   AHA-3950U2B    aic7897   PCI/64
   AHA-3950U2D    aic7897   PCI/64
   AHA-29160      aic7892   PCI/64-66
   AHA-29160 CPQ  aic7892   PCI/64-66
   AHA-29160N     aic7892   PCI/32      LVD-HD68F       SE-HD50F
                                        SE-50M
   AHA-29160LP    aic7892   PCI/64-66
   AHA-19160      aic7892   PCI/64-66
   AHA-29150LP    aic7892   PCI/64-66
   AHA-29130LP    aic7892   PCI/64-66
   AHA-3960D      aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   AHA-3960D CPQ  aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   AHA-39160      aic7899   PCI/64-66  2 X LVD-HD68F  2 X LVD-VHD68F
                                       LVD-50M
   ============== ======= =========== =============== =============== =========

   1. 涓嶆敮鎸?BIOS
   2. 娆＄骇鎬荤嚎涓婂甫澶氫釜鎺у埗鍣ㄨ姱鐗囩殑 DEC21050 PCI-PCI 妗ユ帴鍣?   3. 娆＄骇鎬荤嚎涓婂甫澶氫釜鎺у埗鍣ㄨ姱鐗囩殑 DEC2115X PCI-PCI 妗ユ帴鍣?   4. 涓変釜 SCSI 杩炴帴鍣ㄥ彲鍚屾椂浣跨敤锛岃€屼笉浼氫骇鐢?SCSI "stub" 鏁堝簲銆?
## 2. 鐗堟湰鍘嗗彶


   - 7.0	  (2005骞?鏈?鏃?
 - 鏇存柊椹卞姩浠ヤ娇鐢?SCSI 浼犺緭绫诲熀纭€璁炬柦銆? - 浠?Adaptec 鍙戝竷鐨勪笂涓€鐗堥┍鍔ㄤ腑鎻愬彇浜嗘椂搴忓櫒涓庢牳蹇冧慨澶嶃€?
   - 6.2.36 (2003骞?鏈?鏃?
        - 淇绂佺敤 PCI 濂囧伓鏍￠獙閿欒妫€娴嬬殑浠ｇ爜銆?        - 淇骞剁畝鍖栧蹇界暐瀹戒綑閲忥紙ignore wide residue锛夋秷鎭殑澶勭悊銆傚鏋滀簨鍔℃暟鎹暱搴︿负鍋舵暟涓旀垜浠敹鍒?IWR 娑堟伅锛屾棫浠ｇ爜灏嗘棤娉曟姤鍛婂墿浣欐暟鎹€?        - 澧炲姞瀵?2.5.X EISA 妗嗘灦鐨勬敮鎸併€?        - 閽堝 2.5.X SCSI proc 鏂囦欢绯荤粺鎺ュ彛鐨勫彉鏇磋繘琛屾洿鏂般€?        - 淇鍩熼獙璇侊紙Domain Validation锛夊懡浠よ閫夐」瑙ｆ瀽銆?        - 褰撻€氳繃 8 浣?WDTR 娑堟伅鍗忓晢寮傛妯″紡鏃讹紝鍙戦€佸亸绉婚噺涓?0 鐨?SDTR锛屼互纭繚鐩爣璁惧鐭ラ亾鎴戜滑澶勪簬寮傛妯″紡銆傝繖鍙閬?Quantum Atlas 10K 鐨勫浐浠剁己闄枫€?        - 鍦ㄩ┍鍔ㄦ寕杞芥湡闂存竻闄?PCI 閿欒鐘舵€侊紝浠ュ厤鍦ㄦ垜浠帴绠℃帶鍒跺櫒涔嬪墠鍏朵粬椹卞姩鎺㈡祴浜х敓鐨勬潅鏁ｅ啓鍏ュ鑷村唴瀛樻槧灏?I/O 琚鐢ㄣ€?
   - 6.2.35 (2003骞?鏈?4鏃?
        - 淇鑻ュ共 GCC 3.3 缂栬瘧鍣ㄨ鍛娿€?        - 淇 EISA 鍙岄€氶亾鎺у埗鍣ㄧ殑杩愯銆?        - 澧炲姞瀵?2.5.X 鐨?scsi_report_device_reset() 鐨勬敮鎸併€?
   - 6.2.34 (2003骞?鏈?鏃?
        - 淇 6.2.29 寮曞叆鐨勯攣鍥炲綊闂锛岃闂鍙兘瀵艰嚧 io_request_lock 涓庢垜浠殑 per-softc 閿佷箣闂村嚭鐜伴攣椤哄簭鍙嶈浆銆傛闂浠呭湪 RH9銆丼uSE 浠ュ強 kernel.org 鐨?2.4.X 鍐呮牳涓婂彲鑳藉嚭鐜般€?
   - 6.2.33 (2003骞?鏈?0鏃?
        - 鍦ㄥ凡鍚戠敤鎴锋姤鍛?10 娆￠敊璇悗锛屽姩鎬佺鐢?PCI 濂囧伓鏍￠獙閿欒鎶ュ憡銆傝繖浜涢敊璇槸鐢卞叾浠栬澶囧彂鍑哄鍋舵牎楠岄敊璇殑 PCI 浜嬪姟鎵€鑷淬€備竴鏃︾敤鎴峰凡琚憡鐭ヨ闂锛岀户缁姤鍛婇敊璇彧浼氶檷浣庢垜浠殑鎬ц兘銆?
   - 6.2.32 (2003骞?鏈?8鏃?
        - 鍔ㄦ€佽皟鏁?S/G 鍒楄〃澶у皬锛屼互閬垮厤 SCSI malloc 姹犵鐗囧寲鍜?SCSI 涓棿灞傛閿併€?
   - 6.2.28 (2003骞?鏈?0鏃?
        - 鍩熼獙璇佷慨澶?        - 澧炲姞绂佺敤 PCI 濂囧伓鏍￠獙閿欒妫€娴嬬殑鑳藉姏銆?        - 澧炲己鐨勫唴瀛樻槧灏?I/O 鎺㈡祴

   - 6.2.20 (2002骞?1鏈?鏃?
        - 澧炲姞鍩熼獙璇侊紙Domain Validation锛夈€?
## 3. 鍛戒护琛岄€夐」



```

                 ALTERING OR ADDING THESE DRIVER PARAMETERS
                 INCORRECTLY CAN RENDER YOUR SYSTEM INOPERABLE.
                 USE THEM WITH CAUTION.

   Put a .conf file in the /etc/modprobe.d directory and add/edit a
   line containing ``options aic7xxx aic7xxx=[command[,command...]]`` where
   ``command`` is one or more of the following:

```
verbose

    :Definition: 鍦ㄩ┍鍔ㄨ繍琛屾湡闂村惎鐢ㄩ澶栫殑淇℃伅鎬ф秷鎭€?    :Possible Values: 璇ラ€夐」鏄竴涓爣蹇?    :Default Value: 绂佺敤


debug:[value]

    :Definition: 鍚敤鍚勭骇鍒殑璋冭瘯淇℃伅
    :Possible Values: 0x0000 = 鏃犺皟璇? 0xffff = 瀹屾暣璋冭瘯
    :Default Value: 0x0000

no_probe

probe_eisa_vl

    :Definition: 涓嶆帰娴?EISA/VLB 鎺у埗鍣ㄣ€?		 杩欐槸涓€涓紑鍏炽€傚鏋滈┍鍔ㄩ粯璁ょ紪璇戜负涓嶆帰娴?EISA/VLB 鎺у埗鍣紝
		 鎸囧畾 "no_probe" 灏嗗惎鐢ㄦ鎺㈡祴銆?		 濡傛灉椹卞姩榛樿缂栬瘧涓烘帰娴?EISA/VLB
		 鎺у埗鍣紝鎸囧畾 "no_probe" 灏嗙鐢ㄦ鎺㈡祴銆?
    :Possible Values: 璇ラ€夐」鏄竴涓紑鍏?    :Default Value: EISA/VLB 鎺㈡祴榛樿琚鐢ㄣ€?
pci_parity

    :Definition: 鍒囨崲 PCI 濂囧伓鏍￠獙閿欒鐨勬娴嬨€?		 鍦ㄨ澶氶噰鐢?VIA 鑺墖缁勭殑涓绘澘涓婏紝
		 PCI 鎬荤嚎涓婄殑濂囧伓鏍￠獙鐢熸垚涓嶆纭€傜‖浠舵棤娉?		 鍖哄垎杩欎簺"铏氬亣"濂囧伓鏍￠獙閿欒涓?		 鐪熷疄濂囧伓鏍￠獙閿欒銆傚叾鐥囩姸涓?```

		    "scsi0:	Data Parity Error Detected during address or write data phase"

		 椹卞姩杈撳嚭鐨勪俊鎭€?
    :Possible Values: 璇ラ€夐」鏄竴涓紑鍏?    :Default Value: PCI 濂囧伓鏍￠獙閿欒鎶ュ憡榛樿琚鐢?
```
no_reset

    :Definition: 鍦ㄥ垵濮嬫帰娴嬮樁娈典笉閲嶇疆鎬荤嚎

    :Possible Values: 璇ラ€夐」鏄竴涓爣蹇?    :Default Value: 绂佺敤

extended

    :Definition: 鍦ㄦ帶鍒跺櫒涓婂己鍒跺惎鐢ㄦ墿灞曡浆鎹?    :Possible Values: 璇ラ€夐」鏄竴涓爣蹇?    :Default Value: 绂佺敤

periodic_otag

    :Definition: 鍛ㄦ湡鎬у彂閫佹湁搴忔爣绛句互闃叉鏍囩楗ラタ銆傛煇浜涜緝鏃х殑璁惧闇€瑕佹閫夐」銆?
    :Possible Values: 璇ラ€夐」鏄竴涓爣蹇?    :Default Value: 绂佺敤

reverse_scan

    :Definition: 浠ョ浉鍙嶉『搴忔帰娴?SCSI 鎬荤嚎锛屼粠鐩爣 15 寮€濮?
    :Possible Values: 璇ラ€夐」鏄竴涓爣蹇?    :Default Value: 绂佺敤

global_tag_depth:[value]

    :Definition: 鎵€鏈夋€荤嚎涓婃墍鏈夌洰鏍囩殑鍏ㄥ眬鏍囩娣卞害銆?		 璇ラ€夐」璁剧疆榛樿鏍囩娣卞害锛?		 鍙 tag_info 閫夐」鏈夐€夋嫨鍦拌鐩栥€?
    :Possible Values: 1 - 253
    :Default Value: 32

tag_info:{{value[,value...]}[,{value[,value...]}...]}

    :Definition: 鎸夋帶鍒跺櫒璁剧疆姣忎釜鐩爣鐨勬爣璁伴槦鍒楁繁搴︺€?		 鎺у埗鍣ㄥ拰鐩爣鍧囧彲鐪佺暐锛岃〃绀?		 瀹冧滑搴斾繚鎸侀粯璁ゆ爣绛炬繁搴︺€?
    :Possible Values: 1 - 253
    :Default Value: 32

    绀轰緥锛?
```

	        tag_info:{{16,32,32,64,8,8,,32,32,32,32,32,32,32,32,32}

	    鍦ㄦ帶鍒跺櫒 0 涓婏細

		- 涓虹洰鏍?0 鎸囧畾鏍囩娣卞害 16
		- 涓虹洰鏍?3 鎸囧畾鏍囩娣卞害 64
		- 涓虹洰鏍?4 鍜?5 鎸囧畾鏍囩娣卞害 8
		- 鐩爣 6 淇濇寔榛樿娣卞害
		- 涓虹洰鏍?1銆?銆?-15 鎸囧畾鏍囩娣卞害 32
		- 鎵€鏈夊叾浠栫洰鏍囦繚鎸侀粯璁ゆ繁搴︺€?
	    ::

                tag_info:{{},{32,,32}}

	    鍦ㄦ帶鍒跺櫒 1 涓婏細

		- 涓虹洰鏍?0 鍜?2 鎸囧畾鏍囩娣卞害 32
		- 鎵€鏈夊叾浠栫洰鏍囦繚鎸侀粯璁ゆ繁搴︺€?
```
seltime:[value]

    :Definition: 鎸囧畾閫夋嫨瓒呮椂鍊?    :Possible Values: 0 = 256ms, 1 = 128ms, 2 = 64ms, 3 = 32ms
    :Default Value: 0

dv: {value[,value...]}

    :Definition: 鎸夋帶鍒跺櫒璁剧疆鍩熼獙璇侊紙Domain Validation锛夌瓥鐣ャ€?		 鎺у埗鍣ㄥ彲鐪佺暐锛岃〃绀?		 瀹冧滑搴斾繚鎸侀粯璁よ鍙栨祦璁剧疆銆?
    :Possible Values:

		      ==== ===============================
		       < 0 浣跨敤涓茶 EEPROM 涓殑璁剧疆銆?                         0 绂佺敤 DV
		       > 0 鍚敤 DV
		      ==== ===============================


    :Default Value: 瀵逛簬鏈?DV 鐨?SCSI Select 閫夐」鐨勬帶鍒跺櫒锛屽彇 SCSI-Select 璁剧疆銆?		   鍚﹀垯锛屾敮鎸?U160 閫熷害鐨勬帶鍒跺櫒涓哄紑鍚紝鍏朵粬鎵€鏈夋帶鍒跺櫒绫诲瀷涓哄叧闂€?
    绀轰緥锛?
```

		dv:{-1,0,,1,1,0}

	   - 鎺у埗鍣?0 淇濇寔 DV 榛樿璁剧疆銆?	   - 鎺у埗鍣?1 绂佺敤 DV銆?	   - 璺宠繃鎺у埗鍣?2 鐨勯厤缃€?	   - 鎺у埗鍣?3 鍜?4 鍚敤 DV銆?	   - 鎺у埗鍣?5 绂佺敤 DV銆?
```
```

    options aic7xxx aic7xxx=verbose,no_probe,tag_info:{{},{,,10}},seltime:1

```
鍚敤璇︾粏鏃ュ織锛岀鐢?EISA/VLB 鎺㈡祴锛屽苟灏嗘帶鍒跺櫒 1/鐩爣 2 鐨勬爣绛炬繁搴﹁缃负 10銆?
## 4. Adaptec 瀹㈡埛鏀寔


   Adaptec 鎶€鏈敮鎸侀渶瑕佷竴涓妧鏈敮鎸佹爣璇嗭紙TSID锛夌紪鍙枫€?
    - 12 浣?TSID 鍙湪浜у搧鍖呰鐩掑唴鐨勭櫧鑹叉潯褰㈢爜鏍囩涓婃壘鍒般€俆SID 閫氳繃鍑嗙‘璇嗗埆鎮ㄧ殑浜у搧鍜屾敮鎸佺姸鎬侊紝甯姪鎴戜滑鎻愪緵鏇撮珮鏁堢殑鏈嶅姟銆?
   鏀寔閫夐」
    - 鍦?http://ask.adaptec.com 鎼滅储 Adaptec 鏀寔鐭ヨ瘑搴擄紙ASK锛夛紝鑾峰彇鏈夊叧鎮ㄤ骇鍝佺殑鏂囩珷銆佹帓闅滄妧宸у拰甯歌闂瑙ｇ瓟銆?    - 濡傞渶閫氳繃鐢靛瓙閭欢鑾峰緱鏀寔锛岃灏嗘偍鐨勯棶棰樻彁浜よ嚦 http://ask.adaptec.com/ 鐨?Adaptec 鎶€鏈敮鎸佷笓瀹躲€?
   鍖楃編
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.com/銆?    - 鏈夊叧 Adaptec 鏀寔閫夐」鐨勮祫璁紝璇锋嫧鎵?408-957-2550锛屾瘡澶?24 灏忔椂锛屾瘡鍛?7 澶┿€?    - 濡傞渶涓庝竴鍚嶆妧鏈敮鎸佷笓瀹堕€氳瘽锛?
      - 纭欢浜у搧璇锋嫧鎵?408-934-7274锛屽懆涓€鑷冲懆浜旓紝澶钩娲嬪浠ゆ椂 3:00 鑷?17:00銆?      - RAID 涓庡厜绾ら€氶亾浜у搧璇锋嫧鎵?321-207-2000锛屽懆涓€鑷冲懆浜旓紝澶钩娲嬪浠ゆ椂 3:00 鑷?17:00銆?
      涓哄姞蹇湇鍔★紝璇峰噯澶囧ソ鎮ㄧ殑璁＄畻鏈恒€?    - 璁㈣喘 Adaptec 浜у搧锛堝寘鎷厤浠跺拰绾跨紗锛夛紝璇锋嫧鎵?408-957-7274锛涘湪绾胯璐嚎缂嗚璁块棶 http://www.adaptec.com/buy-cables/銆?
   娆ф床
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.com/en-US/_common/world_index銆?    - 濡傞渶涓庝竴鍚嶆妧鏈敮鎸佷笓瀹堕€氳瘽锛岃鎷ㄦ墦鎴栧彂閫佺數瀛愰偖浠惰嚦锛?
      - 寰疯锛?+49 89 4366 5522锛屽懆涓€鑷冲懆浜旓紝涓鏃堕棿 9:00-17:00锛?        http://ask-de.adaptec.com/銆?      - 娉曡锛?+49 89 4366 5533锛屽懆涓€鑷冲懆浜旓紝涓鏃堕棿 9:00-17:00锛?	http://ask-fr.adaptec.com/銆?      - 鑻辫锛?+49 89 4366 5544锛屽懆涓€鑷冲懆浜旓紝鏍兼灄灏兼不鏍囧噯鏃堕棿 9:00-17:00锛?	http://ask.adaptec.com/銆?
    - 鎮ㄥ彲浠ュ湪绾胯璐?Adaptec 绾跨紗锛岀綉鍧€ http://www.adaptec.com/buy-cables/銆?
   鏃ユ湰
    - 璁块棶鎴戜滑鐨勭綉绔?http://www.adaptec.co.jp/銆?    - 濡傞渶涓庝竴鍚嶆妧鏈敮鎸佷笓瀹堕€氳瘽锛岃鎷ㄦ墦 +81 3 5308 6120锛屽懆涓€鑷冲懆浜旓紝涓婂崍 9:00 鑷?12:00锛屼笅鍗?13:00 鑷?18:00銆?
鐗堟潈 |copy| 2003 Adaptec Inc. 691 S. Milpitas Blvd., Milpitas CA 95035 USA.

淇濈暀鎵€鏈夋潈鍒┿€?
鍏佽鎮ㄥ湪閬靛畧浠ヤ笅鏉′欢鐨勫墠鎻愪笅锛岄殢鍚屽彈閫氱敤鍏叡璁稿彲璇侊紙General Public License锛夌害鏉熺殑杞欢鐨勫啀鍒嗗彂锛屾暣浣撴垨閮ㄥ垎鍦板啀鍒嗗彂銆佷娇鐢ㄥ拰淇敼鏈?README 鏂囦欢锛?
1. README 鏂囦欢鐨勫啀鍒嗗彂蹇呴』淇濈暀涓婅堪鐗堟潈澹版槑銆佹湰鏉′欢鍒楄〃浠ュ強浠ヤ笅鍏嶈矗澹版槑锛屼笉寰椾慨鏀广€?2. 鏈粡鏄庣‘鐨勪簨鍏堜功闈㈣鍙紝涓嶅緱浣跨敤浣滆€呭鍚嶆潵鑳屼功鎴栨帹骞挎簮鑷湰杞欢鐨勪骇鍝併€?3. 淇敼鎴栨柊澧炵殑璐＄尞蹇呴』鍦ㄧ増鏉冨０鏄庝腑娉ㄦ槑浣滆€咃紙"Contributor"锛夛紝骞舵坊鍔犲湪鍘熷鐗堟潈澹版槑涔嬩笅銆傝鐗堟潈澹版槑浠呯敤浜庢爣璇嗚础鐚€咃紝涓嶅簲琚涓哄厑璁告洿鏀?Adaptec 鎵€鎺堜簣鐨勬潈闄愩€?
鏈?README 鏂囦欢鐢?ADAPTEC 鍙婅础鐚€?`AS IS` 鎻愪緵锛屼换浣曟槑绀烘垨榛樼ず鐨勪繚璇侊紙鍖呮嫭浣嗕笉闄愪簬閽堝闈炰镜鏉冩€х殑淇濊瘉锛屾垨鍏充簬閫傞攢鎬т笌鐗瑰畾鐢ㄩ€旈€傜敤鎬х殑榛樼ず淇濊瘉锛夊潎琚惁璁ゃ€傚湪浠讳綍鎯呭喌涓嬶紝ADAPTEC 鎴栬础鐚€呭潎涓嶅鍥犱娇鐢ㄦ湰 README 鏂囦欢锛堝嵆浣垮凡琚憡鐭ユ绫绘崯瀹崇殑鍙兘鎬э級鑰屼互浠讳綍璐ｄ换鐞嗚锛堟棤璁烘槸鍚堝悓銆佷弗鏍艰矗浠昏繕鏄镜鏉冿紝鍖呮嫭鐤忓拷鎴栧叾浠栵級寮曡捣鐨勪换浣曠洿鎺ャ€侀棿鎺ャ€佸伓鐒躲€佺壒娈娿€佹儵鎴掓€ф垨鍚庢灉鎬ф崯瀹筹紙鍖呮嫭浣嗕笉闄愪簬鏇夸唬鍟嗗搧鎴栨湇鍔＄殑閲囪喘銆佷娇鐢ㄦ崯澶便€佹暟鎹垨鍒╂鼎鎹熷け锛屾垨涓氬姟涓柇锛夋壙鎷呰矗浠汇€?