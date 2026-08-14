
## SRBDS - 鐗规畩瀵勫瓨鍣ㄧ紦鍐插尯鏁版嵁閲囨牱


SRBDS 鏄竴绉嶇‖浠舵紡娲烇紝瀹冨厑璁?MDS
Documentation/admin-guide/hw-vuln/mds.rst 鎶€鏈潵鎺ㄦ柇浠庣壒娈婂瘎瀛樺櫒璁块棶杩斿洖鐨勫€笺€傜壒娈婂瘎瀛樺櫒
璁块棶鏄鏍稿锛坥ff core锛夊瘎瀛樺櫒鐨勮闂€傛牴鎹?Intel 鐨勮瘎浼帮紝鍏锋湁闅愮瀹夊叏鎬ч鏈熺殑閭ｄ簺鐗规畩
瀵勫瓨鍣ㄨ鍙栨槸 RDRAND銆丷DSEED 涓?SGX EGETKEY銆?
褰撲娇鐢?RDRAND銆丷DSEED 涓?EGETKEY 鎸囦护鏃讹紝鏁版嵁閫氳繃鏄撳彈 MDS 鏀诲嚮鐨勭壒娈婂瘎瀛樺櫒鏈哄埗绉诲姩鍒?鏍稿績銆?
### 鍙楀奖鍝嶇殑澶勭悊鍣?

瀹炵幇浜?RDRAND 鍜?鎴?RDSEED 鐨勬牳蹇冨瀷鍙凤紙妗岄潰銆佺Щ鍔ㄣ€乆eon-E3锛夊彲鑳戒細鍙楀埌褰卞搷銆?
濡傛灉澶勭悊鍣ㄧ殑 Family_Model 涓?stepping 鍦ㄤ互涓嬪垪琛ㄤ腑锛屽垯鍙?SRBDS 褰卞搷锛屼絾浠ヤ笅渚嬪锛氬垪鍑虹殑
澶勭悊鍣ㄥ湪 Intel TSX 鍙敤鍗存湭鍚敤鏃跺鍑?MDS_NO銆傚悗涓€绫诲鐞嗗櫒浠呭綋杞欢浣跨敤 TSX_CTRL_MSR
鍚敤 Intel TSX 鏃舵墠鍙楀奖鍝嶏紝鍚﹀垯涓嶅彈褰卞搷銆?
  =============  ============  ========
  common name    Family_Model  Stepping
  =============  ============  ========
  IvyBridge      06_3AH        All

  Haswell        06_3CH        All
  Haswell_L      06_45H        All
  Haswell_G      06_46H        All

  Broadwell_G    06_47H        All
  Broadwell      06_3DH        All

  Skylake_L      06_4EH        All
  Skylake        06_5EH        All

  Kabylake_L     06_8EH        <= 0xC
  Kabylake       06_9EH        <= 0xD
  =============  ============  ========

### 鐩稿叧 CVE


浠ヤ笅 CVE 鏉＄洰涓?SRBDS 闂鐩稿叧锛?
    ==============  =====  =====================================
    CVE-2020-0543   SRBDS  鐗规畩瀵勫瓨鍣ㄧ紦鍐插尯鏁版嵁閲囨牱
    ==============  =====  =====================================

### 鏀诲嚮鍦烘櫙


闈炵壒鏉冪敤鎴峰彲浠ヤ娇鐢?MDS 鎶€鏈紝鎻愬彇鍦ㄥ彟涓€涓牳蹇冩垨鍏勫紵绾跨▼涓婃墽琛岀殑 RDRAND 涓?RDSEED 鎵€杩斿洖鐨?鍊笺€?

### 缂撹В鏈哄埗


Intel 灏嗗彂甯冨井鐮佹洿鏂帮紝淇敼 RDRAND銆丷DSEED 涓?EGETKEY 鎸囦护锛屽湪绉樺瘑鐗规畩瀵勫瓨鍣ㄦ暟鎹鍙︿竴涓?閫昏緫澶勭悊鍣ㄨ闂箣鍓嶏紝瑕嗙洊鍏变韩鏆傚瓨缂撳啿鍖轰腑鐨勭瀵嗙壒娈婂瘎瀛樺櫒鏁版嵁銆?
鍦ㄦ墽琛?RDRAND銆丷DSEED 鎴?EGETKEY 鎸囦护鏈熼棿锛屾潵鑷叾浠栭€昏緫澶勭悊鍣ㄧ殑鏍稿璁块棶灏嗚寤惰繜锛岀洿鍒?鐗规畩瀵勫瓨鍣ㄨ鍙栧畬鎴愶紝骞朵笖鍏变韩鏆傚瓨缂撳啿鍖轰腑鐨勭瀵嗘暟鎹瑕嗙洊銆?
杩欏鎬ц兘鏈変笁涓奖鍝嶏細

#. RDRAND銆丷DSEED 鎴?EGETKEY 鎸囦护鍏锋湁鏇撮珮鐨勫欢杩熴€?
#. 鍦ㄥ涓€昏緫澶勭悊鍣ㄤ笂鍚屾椂鎵ц RDRAND 灏嗚涓茶鍖栵紝瀵艰嚧 RDRAND 鐨勬渶澶у甫瀹芥暣浣撲笅闄嶃€?
#. 鎵ц RDRAND銆丷DSEED 鎴?EGETKEY 浼氬欢杩熸潵鑷叾浠栭€昏緫澶勭悊鍣ㄣ€佹湭鍛戒腑鍏舵牳蹇冪紦瀛樼殑鍐呭瓨璁块棶锛?   鍏跺奖鍝嶇被浼间簬浼犵粺鐨勯攣瀹氱紦瀛樿鎷嗗垎锛坙ocked cache-line-split锛夎闂€?
寰爜鏇存柊鎻愪緵浜嗕竴绉嶉€€鍑烘満鍒讹紙RNGDS_MITG_DIS锛夛紝鐢ㄤ簬鍦?Intel Software Guard Extensions
锛圛ntel SGX锛夊鐨?enclave 涓墽琛?RDRAND 涓?RDSEED 鎸囦护鏃剁鐢ㄧ紦瑙ｃ€傚湪浣跨敤姝ら€€鍑烘満鍒剁鐢?缂撹В鐨勯€昏緫澶勭悊鍣ㄤ笂锛孯DRAND 涓?RDSEED 鎵ц涓嶄細鑺辫垂鏇撮暱鏃堕棿锛屼篃涓嶄細褰卞搷鍏勫紵閫昏緫澶勭悊鍣ㄧ殑
鍐呭瓨璁块棶鎬ц兘銆傝閫€鍑烘満鍒朵笉褰卞搷 Intel SGX enclave锛堝寘鎷湪 enclave 鍐呮墽琛?RDRAND 鎴?RDSEED锛?浠ュ強 EGETKEY 鐨勬墽琛岋級銆?
### IA32_MCU_OPT_CTRL MSR 瀹氫箟


闄や簡閽堝姝ら棶棰樼殑缂撹В鎺柦澶栵紝Intel 杩樻柊澧炰簡涓€涓嚎绋嬩綔鐢ㄥ煙鐨?IA32_MCU_OPT_CTRL MSR
锛堝湴鍧€ 0x123锛夈€傝 MSR 浠ュ強 RNGDS_MITG_DIS锛堜綅 0锛夌殑瀛樺湪鐢?CPUID.(EAX=07H,ECX=0).EDX[SRBDS_CTRL = 9]==1 鏋氫妇銆傝 MSR 閫氳繃寰爜鏇存柊寮曞叆銆?
灏嗘煇涓€昏緫澶勭悊鍣ㄧ殑 IA32_MCU_OPT_CTRL[^0^]锛圧NGDS_MITG_DIS锛夎涓?1锛屼細绂佺敤璇ラ€昏緫澶勭悊鍣ㄤ笂
鍦?Intel SGX enclave 澶栨墽琛岀殑 RDRAND 涓?RDSEED 鐨勭紦瑙ｃ€備负鏌愪釜鐗瑰畾閫昏緫澶勭悊鍣ㄩ€€鍑虹紦瑙ｏ紝涓嶄細
褰卞搷鍏朵粬閫昏緫澶勭悊鍣ㄧ殑 RDRAND 涓?RDSEED 缂撹В銆?
娉ㄦ剰锛屽湪 Intel SGX enclave 鍐呴儴锛屾棤璁?RNGDS_MITG_DS 鐨勫€煎浣曪紝閮戒細搴旂敤缂撹В銆?
### 鍐呮牳鍛戒护琛屼笂鐨勭紦瑙ｆ帶鍒?

鍐呮牳鍛戒护琛屽厑璁稿湪寮曞鏃堕€氳繃 "srbds=" 閫夐」鎺у埗 SRBDS 缂撹В銆傝閫夐」涓猴細

  ============= =============================================================
  off           姝ら€夐」鍦ㄥ彈褰卞搷鐨勫钩鍙颁笂绂佺敤 RDRAND 涓?RDSEED 鐨?SRBDS 缂撹В銆?  ============= =============================================================

### SRBDS 绯荤粺淇℃伅


Linux 鍐呮牳閫氳繃 sysfs 鎻愪緵婕忔礊鐘舵€佷俊鎭€傚浜?SRBDS锛屽彲閫氳繃浠ヤ笅 sysfs 鏂囦欢璁块棶锛?/sys/devices/system/cpu/vulnerabilities/srbds

璇ユ枃浠跺彲鑳藉寘鍚殑鍊间负锛?
 ============================== =============================================
 Not affected                   澶勭悊鍣ㄤ笉瀛樺湪婕忔礊
 Vulnerable                     澶勭悊鍣ㄥ瓨鍦ㄦ紡娲炰笖缂撹В宸茬鐢? Vulnerable: No microcode       澶勭悊鍣ㄥ瓨鍦ㄦ紡娲炰笖缂哄皯缂撹В寰爜
 Mitigation: Microcode          澶勭悊鍣ㄥ瓨鍦ㄦ紡娲炰笖缂撹В宸茬敓鏁? Mitigation: TSX disabled       澶勭悊鍣ㄤ粎褰?TSX 鍚敤鏃跺瓨鍦ㄦ紡娲烇紝鑰屾湰绯荤粺鍚姩鏃?                                浠?TSX 绂佺敤鏂瑰紡寮曞
 Unknown: Dependent on
 hypervisor status              杩愯鍦ㄥ彈褰卞搷浣嗘棤娉曞緱鐭ュ涓绘満澶勭悊鍣ㄦ槸鍚﹀凡缂撹В鎴?                                瀛樺湪婕忔礊鐨勮櫄鎷熷鎴锋満澶勭悊鍣ㄤ笂
 ============================== =============================================

### SRBDS 榛樿缂撹В


杩欎竴鏂扮殑寰爜鍦ㄦ墽琛?RDRAND銆丷DSEED 鏃朵覆琛屽寲澶勭悊鍣ㄨ闂紝纭繚鍏变韩缂撳啿鍖哄湪琚噴鏀惧鐢ㄤ箣鍓?琚鐩栥€備娇鐢?"srbds=off" 鍐呮牳鍛戒护琛屾潵绂佺敤 RDRAND 涓?RDSEED 鐨勭紦瑙ｃ€?