
## ARM 鍥轰欢浼瘎瀛樺櫒鎺ュ彛


KVM 鎸夌収瀹㈡埛鏈虹殑璇锋眰澶勭悊 hypercall 鏈嶅姟銆侫RM 瑙勮寖鎴?KVM锛堜綔涓轰緵搴斿晢鏈嶅姟锛変細瀹氭湡鎻愪緵鏂扮殑 hypercall 鏈嶅姟锛屽彧瑕佸畠浠粠铏氭嫙鍖栫殑瑙掑害鏉ョ湅鏄湁鎰忎箟鐨勩€?
杩欐剰鍛崇潃锛屽湪涓ょ涓嶅悓鐗堟湰鐨?KVM 涓婂惎鍔ㄧ殑瀹㈡埛鏈哄彲鑳借瀵熷埌涓ょ涓嶅悓鐨勨€滃浐浠垛€濅慨璁㈢増鏈€傚鏋滄煇涓鎴锋満缁戝畾鍒扮壒瀹氱増鏈殑 hypercall 鏈嶅姟锛屾垨鑰呬竴娆¤縼绉荤獊鐒跺悜姣棤闃插鐨勫鎴锋満鏆撮湶浜嗕笉鍚岀殑鐗堟湰锛岃繖鍙兘浼氬鑷撮棶棰樸€?
涓轰簡琛ユ晳杩欑鎯呭喌锛孠VM 鏆撮湶浜嗕竴缁勫彲浠ヤ娇鐢?GET/SET_ONE_REG 鎺ュ彛鎿嶇旱鐨勨€滃浐浠朵吉瀵勫瓨鍣ㄢ€濄€傝繖浜涘瘎瀛樺櫒鍙互鐢辩敤鎴风┖闂翠繚瀛?鎭㈠锛屽苟鏍规嵁闇€瑕佽缃负鏂逛究鐨勫€笺€?
瀹氫箟浜嗕互涓嬪瘎瀛樺櫒锛?
- KVM_REG_ARM_PSCI_VERSION:

  KVM 瀹炵幇浜?PSCI锛圥ower State Coordination Interface锛岀數婧愮姸鎬佸崗璋冩帴鍙ｏ級瑙勮寖锛屼互鍚戝鎴锋満鎻愪緵 CPU 寮€鍏虫満銆佸浣嶅拰鏂數绛夋湇鍔°€?
  - 浠呭綋 vcpu 璁剧疆浜?KVM_ARM_VCPU_PSCI_0_2 鐗规€э紙骞朵笖鍥犳宸茬粡鍒濆鍖栵級鏃舵墠鏈夋晥
  - 鍦?GET_ONE_REG 鏃惰繑鍥炲綋鍓?PSCI 鐗堟湰锛堥粯璁や负 KVM 瀹炵幇鐨勬渶楂樹笖涓?v0.2 鍏煎鐨?PSCI 鐗堟湰锛?  - 鍏佽浣跨敤 SET_ONE_REG 璁剧疆浠讳綍 KVM 瀹炵幇涓斾笌 v0.2 鍏煎鐨?PSCI 鐗堟湰
  - 褰卞搷鏁翠釜 VM锛堝嵆浣垮瘎瀛樺櫒瑙嗗浘鏄寜 vcpu 鐨勶級

- KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1:
    淇濆瓨鍥轰欢鏀寔鐨勭姸鎬侊紝鐢ㄤ簬缂撹В CVE-2017-5715锛屾濡?KVM 閫氳繃 HVC 璋冪敤鍚戝鎴锋満鎻愪緵鐨勯偅鏍枫€傝缂撹В鏂规硶鍦?[^1^] 鐨?SMCCC_ARCH_WORKAROUND_1 涓嬫弿杩般€?
  鍙帴鍙楃殑鍊间负锛?
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL:
      KVM 涓嶆彁渚?      璇ョ紦瑙ｆ柟娉曠殑鍥轰欢鏀寔銆傚瀹㈡埛鏈虹殑缂撹В鐘舵€佹湭鐭ャ€?    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL:
      璇ョ紦瑙ｆ柟娉?HVC 璋冪敤瀵?      瀹㈡埛鏈哄彲鐢紝涓旀槸缂撹В鎵€蹇呴渶鐨勩€?    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED:
      璇ョ紦瑙ｆ柟娉?HVC 璋冪敤瀵?      瀹㈡埛鏈哄彲鐢紝浣嗗湪姝?VCPU 涓婁笉闇€瑕併€?
- KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2:
    淇濆瓨鍥轰欢鏀寔鐨勭姸鎬侊紝鐢ㄤ簬缂撹В CVE-2018-3639锛屾濡?KVM 閫氳繃 HVC 璋冪敤鍚戝鎴锋満鎻愪緵鐨勯偅鏍枫€傝缂撹В鏂规硶鍦?[^1^]_ 鐨?SMCCC_ARCH_WORKAROUND_2 涓嬫弿杩般€?
  鍙帴鍙楃殑鍊间负锛?
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL:
      缂撹В鏂规硶涓?      鍙敤銆侹VM 涓嶆彁渚涜缂撹В鏂规硶鐨勫浐浠舵敮鎸併€?    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_UNKNOWN:
      缂撹В鏂规硶鐘舵€?      鏈煡銆侹VM 涓嶆彁渚涜缂撹В鏂规硶鐨勫浐浠舵敮鎸併€?    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_AVAIL:
      缂撹В鏂规硶鍙敤锛?      骞朵笖鍙互琚?vCPU 绂佺敤銆傚鏋滆缃簡
      KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_ENABLED锛屽垯瀹冨璇?vCPU 澶勪簬娲诲姩鐘舵€併€?    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED:
      璇ョ紦瑙ｆ柟娉曞缁堝湪璇?vCPU 涓婂浜庢椿鍔ㄧ姸鎬侊紝鎴栬€呬笉闇€瑕佸畠銆?
### 浣嶅浘鐗规€у浐浠跺瘎瀛樺櫒


涓庝笂杩板瘎瀛樺櫒鐩稿弽锛屼互涓嬪瘎瀛樺櫒浠ョ壒鎬т綅鍥剧殑褰㈠紡鍚戠敤鎴风┖闂存毚闇?hypercall 鏈嶅姟銆傝浣嶅浘琚浆鎹负瀵瑰鎴锋満鍙敤鐨勬湇鍔°€備负姣忎釜鏈嶅姟璋冪敤鎵€鏈夎€呭畾涔変簡涓€涓瘎瀛樺櫒锛屽苟鍙€氳繃 GET/SET_ONE_REG 鎺ュ彛璁块棶銆?
榛樿鎯呭喌涓嬶紝杩欎簺瀵勫瓨鍣ㄨ璁剧疆涓烘墍鏀寔鐗规€х殑涓婇檺銆傝繖鏍风敤鎴风┖闂村氨鍙互閫氳繃 GET_ONE_REG 鍙戠幇鎵€鏈夊彲鐢ㄧ殑 hypercall 鏈嶅姟銆傜敤鎴风┖闂村彲浠ラ€氳繃 SET_ONE_REG 灏嗘湡鏈涚殑浣嶅浘鍐欏洖銆傛湭琚Е鍙婄殑瀵勫瓨鍣紙鍙兘鏄洜涓虹敤鎴风┖闂翠笉鐭ラ亾瀹冧滑锛夌殑鐗规€у皢鎸夊師鏍锋毚闇茬粰瀹㈡埛鏈恒€?
璇锋敞鎰忥紝涓€鏃︿换浣?vCPU 鑷冲皯杩愯杩囦竴娆★紝KVM 灏嗕笉鍐嶅厑璁哥敤鎴风┖闂撮厤缃繖浜涘瘎瀛樺櫒銆傜浉鍙嶏紝瀹冧細杩斿洖 -EBUSY銆?
浼浐浠朵綅鍥惧瘎瀛樺櫒濡備笅锛?
- KVM_REG_ARM_STD_BMAP:
    鎺у埗 ARM 鏍囧噯瀹夊叏鏈嶅姟璋冪敤鐨勪綅鍥俱€?
  鎺ュ彈浠ヤ笅浣嶏細

    Bit-0: KVM_REG_ARM_STD_BIT_TRNG_V1_0:
      璇ヤ綅浠ｈ〃 ARM True Random Number Generator锛圱RNG锛岀湡闅忔満鏁扮敓鎴愬櫒锛夎鑼?v1.0锛圓RM DEN0098锛変笅鎻愪緵鐨勬湇鍔°€?
- KVM_REG_ARM_STD_HYP_BMAP:
    鎺у埗 ARM 鏍囧噯 Hypervisor 鏈嶅姟璋冪敤鐨勪綅鍥俱€?
  鎺ュ彈浠ヤ笅浣嶏細

    Bit-0: KVM_REG_ARM_STD_HYP_BIT_PV_TIME:
      璇ヤ綅浠ｈ〃鐢?ARM DEN0057A 琛ㄧず鐨勫崐铏氭嫙鍖栨椂闂达紙Paravirtualized Time锛夋湇鍔°€?
- KVM_REG_ARM_VENDOR_HYP_BMAP:
    鎺у埗渚涘簲鍟嗙壒瀹氱殑 Hypervisor 鏈嶅姟璋冪敤 [0-63] 鐨勪綅鍥俱€?
  鎺ュ彈浠ヤ笅浣嶏細

    Bit-0: KVM_REG_ARM_VENDOR_HYP_BIT_FUNC_FEAT
      璇ヤ綅浠ｈ〃 ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID
      鍜?ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID 鍑芥暟 id銆?
    Bit-1: KVM_REG_ARM_VENDOR_HYP_BIT_PTP:
      璇ヤ綅浠ｈ〃绮剧‘鏃堕棿鍗忚锛圥recision Time Protocol锛塊VM 鏈嶅姟銆?
- KVM_REG_ARM_VENDOR_HYP_BMAP_2:
    鎺у埗渚涘簲鍟嗙壒瀹氱殑 Hypervisor 鏈嶅姟璋冪敤 [64-127] 鐨勪綅鍥俱€?
  鎺ュ彈浠ヤ笅浣嶏細

    Bit-0: KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_VER
      杩欎唬琛?ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_VER_FUNC_ID
      鍑芥暟 id銆傛浣嶈澶嶄綅涓?0銆?
    Bit-1: KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_CPUS
      杩欎唬琛?ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_CPUS_FUNC_ID
      鍑芥暟 id銆傛浣嶈澶嶄綅涓?0銆?
閿欒锛?
    =======  =============================================================
    -ENOENT   璁块棶浜嗘湭鐭ュ瘎瀛樺櫒銆?    -EBUSY    鍦?VM 鍚姩鍚庡皾璇曞瀵勫瓨鍣ㄨ繘琛屸€滃啓鈥濇搷浣溿€?    -EINVAL   鍐欏叆浜嗘棤鏁堢殑浣嶅浘鍒板瘎瀛樺櫒銆?    =======  =============================================================
