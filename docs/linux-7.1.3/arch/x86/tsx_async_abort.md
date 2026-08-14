
## TSX 寮傛涓锛圱AA锛夌紦瑙?

鏈枃妗ｈ鏄庨拡瀵?Intel 澶勭悊鍣ㄧ殑 TSX 寮傛涓锛圱AA锛変晶淇￠亾婕忔礊鐨勭紦瑙ｆ帾鏂斤紝浠嬬粛鍏跺師鐞嗐€佸唴鏍稿彲鐢ㄧ殑缂撹В妯″紡锛堝绂佺敤 TSX銆佹竻闄?CPU 缂撳啿鍖猴級浠ュ強瀵瑰簲鐨勫惎鍔ㄥ弬鏁颁笌閰嶇疆鏂规硶銆?


### 姒傝堪


TSX 寮傛涓锛圱SX Async Abort, TAA锛夋槸閽堝鏌愪簺 Intel 澶勭悊鍣ㄥ唴閮ㄧ紦鍐插尯鐨勪晶淇￠亾鏀诲嚮锛?
绫讳技浜庡井鏋舵瀯鏁版嵁閲囨牱锛圡icroarchitectural Data Sampling, MDS锛夈€傚湪杩欑鎯呭喌涓嬶紝褰撳湪
浜嬪姟鍚屾鎵╁睍锛圱ransactional Synchronization Extensions, TSX锛変簨鍔′腑瀛樺湪鎸傝捣鐨勫紓姝?
涓鏉′欢鏃讹紝鏌愪簺鍔犺浇鍙兘浼氭姇鏈哄湴灏嗘棤鏁堟暟鎹紶閫掔粰渚濊禆鎿嶄綔銆傝繖鍖呮嫭娌℃湁 fault 鎴?assist
鏉′欢鐨勫姞杞姐€傛绫诲姞杞藉彲鑳藉儚 MDS 涓€鏍锋姇鏈哄湴鏆撮湶鏉ヨ嚜鐩稿悓 uarch 鏁版嵁缁撴瀯鐨勯檲鏃ф暟鎹紝鏆撮湶
鑼冨洿鐩稿悓锛屽嵆鍚岀嚎绋嬪拰璺ㄧ嚎绋嬨€傛闂褰卞搷鎵€鏈夊綋鍓嶆敮鎸?TSX 鐨勫鐞嗗櫒銆?

### 缂撹В绛栫暐


a) 绂佺敤 TSX 鈥斺€?缂撹В鎺柦涔嬩竴鏄鐢?TSX銆備竴涓柊鐨?MSR IA32_TSX_CTRL 灏嗗湪鏈潵鐨勪互鍙?
褰撳墠鐨勫鐞嗗櫒閫氳繃寰爜鏇存柊鍚庡彲鐢紝鍙敤浜庣鐢?TSX銆傛澶栵紝瀹冩帶鍒?CPUID 涓?TSX 鐗规€т綅
锛圧TM 鍜?HLE锛夌殑鏋氫妇銆?

b) 娓呴櫎 CPU 缂撳啿鍖?鈥斺€?涓?MDS 绫讳技锛屾竻闄?CPU 缂撳啿鍖哄彲缂撹В姝ゆ紡娲炪€傛湁鍏虫鏂规硶鐨勬洿澶?
璇︾粏淇℃伅锛岃鍙傞槄 Documentation/admin-guide/hw-vuln/mds.rst <mds>銆?

### 鍐呮牳鍐呴儴缂撹В妯″紡


 =============    ============================================================
 off              缂撹В宸茬鐢ㄣ€傝涔?CPU 涓嶅彈褰卞搷锛岃涔堝湪鍐呮牳鍛戒护琛屼笂鎻愪緵浜?
                  tsx_async_abort=off銆?

 tsx disabled     缂撹В宸插惎鐢ㄣ€傚湪鏀寔 TSX 鎺у埗鐨勫鐞嗗櫒涓婏紝TSX 鐗规€у湪鍚姩鏃堕粯璁ょ鐢ㄣ€?

 verw             缂撹В宸插惎鐢ㄣ€侰PU 鍙楀奖鍝嶏紝涓?MD_CLEAR 鍦?CPUID 涓€氬憡銆?

 ucode needed     缂撹В宸插惎鐢ㄣ€侰PU 鍙楀奖鍝嶏紝浣?MD_CLEAR 鏈湪 CPUID 涓€氬憡銆傝繖涓昏鐢ㄤ簬
                  铏氭嫙鍖栧満鏅紝鍏朵腑瀹夸富鏈烘湁鏇存柊鐨勫井鐮侊紝浣?hypervisor 鏈湪 CPUID 涓毚闇?
                  MD_CLEAR銆傝繖鏄竴绉嶅敖鍔涜€屼负鐨勬柟娉曪紝涓嶆彁渚涗繚璇併€?
 =============    ============================================================

濡傛灉 CPU 鍙楀奖鍝嶄笖鏈彁渚?"tsx_async_abort" 鍐呮牳鍛戒护琛屽弬鏁帮紝鍒欏唴鏍镐細鏍规嵁 RTM 鍜?
MD_CLEAR 鐨?CPUID 浣嶇姸鎬侀€夋嫨閫傚綋鐨勭紦瑙ｆ帾鏂姐€?

涓嬭〃鎸囩ず浜?tsx=on|off|auto 鍛戒护琛岄€夐」瀵瑰悇绉?MSR_IA32_ARCH_CAPABILITIES 浣嶇粍鍚堜笅鐨?
TAA 缂撹В鐘舵€併€乂ERW 琛屼负鍜?TSX 鐗规€х殑褰卞搷銆?

1. "tsx=off"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=off
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default         Yes           Same as MDS           Same as MDS
    0          0           1        Invalid case   Invalid case       Invalid case          Invalid case
    0          1           0         HW default         No         Need ucode update     Need ucode update
    0          1           1          Disabled          Yes           TSX disabled          TSX disabled
    1          X           1          Disabled           X             None needed           None needed
=========  =========  ============  ============  ==============  ===================  ======================

2. "tsx=on"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=on
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default        Yes            Same as MDS          Same as MDS
    0          0           1        Invalid case   Invalid case       Invalid case         Invalid case
    0          1           0         HW default        No          Need ucode update     Need ucode update
    0          1           1          Enabled          Yes               None              Same as MDS
    1          X           1          Enabled          X              None needed          None needed
=========  =========  ============  ============  ==============  ===================  ======================

3. "tsx=auto"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=auto
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default    Yes                Same as MDS           Same as MDS
    0          0           1        Invalid case  Invalid case        Invalid case          Invalid case
    0          1           0         HW default    No              Need ucode update     Need ucode update
    0          1           1          Disabled      Yes               TSX disabled          TSX disabled
    1          X           1          Enabled       X                 None needed           None needed
=========  =========  ============  ============  ==============  ===================  ======================

鍦ㄨ〃涓紝TSX_CTRL_MSR 鏄?MSR_IA32_ARCH_CAPABILITIES 涓殑涓€涓柊浣嶏紝鎸囩ず鏄惁鏀寔
MSR_IA32_TSX_CTRL銆?

IA32_TSX_CTRL MSR 涓湁涓や釜鎺у埗浣嶏細

      Bit 0: 璁剧疆鏃剁鐢?TSX 鐨勫彈闄愪簨鍔″唴瀛橈紙Restricted Transactional Memory, RTM锛?
             瀛愮壒鎬э紙灏嗗己鍒舵墍鏈変簨鍔″湪 XBEGIN 鎸囦护涓婁腑姝級銆?

      Bit 1: 璁剧疆鏃剁鐢?RTM 鍜?HLE 鐗规€х殑鏋氫妇锛堝嵆瀹冧細浣?CPUID(EAX=7).EBX{bit4} 鍜?
             CPUID(EAX=7).EBX{bit11} 璇讳负 0锛夈€?
