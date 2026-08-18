
## 鍙椾繚鎶ゆ墽琛岃鏂斤紙Protected Execution Facility锛?

    :depth: 3

# 绠€浠?

    Protected Execution Facility锛圥EF锛屽彈淇濇姢鎵ц璁炬柦锛夋槸 POWER 9 鐨勪竴椤规灦鏋勬敼鍔紝
    鐢ㄤ簬鍚敤瀹夊叏铏氭嫙鏈猴紙SVM锛夈€侱D2.3 鑺墖锛圥VR=0x004e1203锛夋垨鏇撮珮鐗堟湰灏嗗叿澶?PEF 鑳藉姏銆?    涓€涓柊鐨?ISA 鐗堟湰灏嗗寘鍚?PEF RFC02487 鐨勬敼鍔ㄣ€?
    鍚敤鍚庯紝PEF 鍚?POWER 鏋舵瀯娣诲姞浜嗕竴绉嶆柊鐨勩€佺壒鏉冩洿楂樼殑妯″紡锛岀О涓?Ultravisor锛堣秴绾?    鐩戠鑰咃級妯″紡銆傞厤鍚堣繖涓€鏂版ā寮忥紝杩樺嚭鐜颁簡涓€涓柊鐨勫浐浠讹紝绉颁负 Protected Execution
    Ultravisor锛堝彈淇濇姢鎵ц Ultravisor锛岀畝绉?Ultravisor锛夈€俇ltravisor 妯″紡鏄?POWER
    鏋舵瀯涓壒鏉冩渶楂樼殑妯″紡銆?
	+------------------+
	| Privilege States |
	+==================+
	|  Problem         |
	+------------------+
	|  Supervisor      |
	+------------------+
	|  Hypervisor      |
	+------------------+
	|  Ultravisor      |
	+------------------+

    PEF 淇濇姢 SVM 鍏嶅彈 Hypervisor銆佺壒鏉冪敤鎴蜂互鍙婄郴缁熶腑鍏朵粬铏氭嫙鏈虹殑渚靛銆係VM 鍦ㄩ潤姝㈢姸鎬佷笅
    涔熷彈鍒颁繚鎶わ紝骞朵笖鍙兘鐢辩粡杩囨巿鏉冪殑鏈哄櫒鎵ц銆傛墍鏈夎櫄鎷熸満閮藉埄鐢?Hypervisor 鏈嶅姟銆?    Ultravisor 浼氳繃婊?SVM 涓?Hypervisor 涔嬮棿鐨勮皟鐢紝浠ョ‘淇濅俊鎭笉浼氭剰澶栨硠闇层€傞櫎 H_RANDOM
    涔嬪鐨勬墍鏈?hypercall锛堣秴绾ц皟鐢級閮戒細琚弽灏勶紙reflect锛夊埌 Hypervisor銆侶_RANDOM 涓嶈
    鍙嶅皠锛屼互闃叉 Hypervisor 褰卞搷 SVM 涓殑闅忔満鍊笺€?
    涓轰簡鏀寔杩欎竴鐐癸紝闇€瑕佸 CPU 涓祫婧愮殑鎵€鏈夋潈杩涜閲嶆瀯銆備竴浜涘厛鍓嶅睘浜?Hypervisor 鐗规潈鐨?    璧勬簮鐜板湪鏀逛负灞炰簬 Ultravisor 鐗规潈銆?
## 纭欢


    Hardware锛堢‖浠讹級鏂归潰鐨勬敼鍔ㄥ寘鎷互涓嬪唴瀹癸細

    - MSR 涓湁涓€涓柊鐨勪綅锛岀敤浜庣‘瀹氬綋鍓嶈繘绋嬫槸鍚﹀湪瀹夊叏妯″紡涓嬭繍琛岋紝鍗?MSR(S) 浣?41銆?      MSR(S)=1 鏃讹紝杩涚▼澶勪簬瀹夊叏妯″紡锛汳SR(s)=0 鏃讹紝杩涚▼澶勪簬鏅€氭ā寮忋€?
    - MSR(S) 浣嶅彧鑳界敱 Ultravisor 璁剧疆銆?
    - HRFID 涓嶈兘鐢ㄤ簬璁剧疆 MSR(S) 浣嶃€傚鏋?Hypervisor 闇€瑕佽繑鍥炲埌鏌愪釜 SVM锛屽畠蹇呴』浣跨敤
      ultracall锛堣秴绾ц皟鐢級銆傚畠鍙互纭畾瑕佽繑鍥炵殑 VM 鏄惁鏄畨鍏ㄧ殑銆?
    - 鏈変竴涓柊鐨?Ultravisor 鐗规潈瀵勫瓨鍣?SMFCTRL锛屽叾涓湁涓€涓娇鑳?绂佺敤浣?SMFCTRL(E)銆?
    - 杩涚▼鐨勭壒鏉冪幇鍦ㄧ敱涓変釜 MSR 浣?MSR(S, HV, PR) 鍐冲畾銆傚湪涓嬮潰姣忎釜琛ㄤ腑锛屾ā寮忔寜浠庢渶浣?      鐗规潈鍒版渶楂樼壒鏉冩帓鍒椼€傝緝楂樼壒鏉冪殑妯″紡鍙互璁块棶杈冧綆鐗规潈妯″紡鐨勬墍鏈夎祫婧愩€?
      **瀹夊叏妯″紡 MSR 璁剧疆**

      +---+---+---+---------------+
      | S | HV| PR|Privilege      |
      +===+===+===+===============+
      | 1 | 0 | 1 | Problem       |
      +---+---+---+---------------+
      | 1 | 0 | 0 | Privileged(OS)|
      +---+---+---+---------------+
      | 1 | 1 | 0 | Ultravisor    |
      +---+---+---+---------------+
      | 1 | 1 | 1 | Reserved      |
      +---+---+---+---------------+

      **鏅€氭ā寮?MSR 璁剧疆**

      +---+---+---+---------------+
      | S | HV| PR|Privilege      |
      +===+===+===+===============+
      | 0 | 0 | 1 | Problem       |
      +---+---+---+---------------+
      | 0 | 0 | 0 | Privileged(OS)|
      +---+---+---+---------------+
      | 0 | 1 | 0 | Hypervisor    |
      +---+---+---+---------------+
      | 0 | 1 | 1 | Problem (Host)|
      +---+---+---+---------------+

    - 鍐呭瓨琚垝鍒嗕负瀹夊叏鍐呭瓨涓庢櫘閫氬唴瀛樸€傚彧鏈夎繍琛屽湪瀹夊叏妯″紡涓嬬殑杩涚▼鎵嶈兘璁块棶瀹夊叏鍐呭瓨銆?
    - 纭欢涓嶅厑璁镐换浣曟湭杩愯鍦ㄥ畨鍏ㄦā寮忎笅鐨勫疄浣撹闂畨鍏ㄥ唴瀛樸€傝繖鎰忓懗鐫€ Hypervisor 鏃犳硶
      鍦ㄤ笉浣跨敤 ultracall锛堣姹?Ultravisor锛夌殑鎯呭喌涓嬭闂?SVM 鐨勫唴瀛樸€俇ltravisor 鍙細
      鍏佽 Hypervisor 浠ュ姞瀵嗗舰寮忕湅鍒?SVM 鐨勫唴瀛樸€?
    - I/O 绯荤粺涓嶅厑璁哥洿鎺ュ鍧€瀹夊叏鍐呭瓨銆傝繖闄愬埗 SVM 鍙兘浣跨敤铏氭嫙 I/O銆?
    - 鏋舵瀯鍏佽 SVM 涓?Hypervisor 鍏变韩涓嶅彈鍔犲瘑淇濇姢鐨勯〉闈€備絾鏄紝杩欑鍏变韩蹇呴』鐢?SVM 鍙戣捣銆?
    - 褰撹繘绋嬭繍琛屽湪瀹夊叏妯″紡鏃讹紝鎵€鏈?hypercall锛坰yscall lev=1锛夐兘浼氳繘鍏?Ultravisor銆?
    - 褰撹繘绋嬪浜庡畨鍏ㄦā寮忔椂锛屾墍鏈変腑鏂兘浼氳繘鍏?Ultravisor銆?
    - 浠ヤ笅璧勬簮宸叉垚涓?Ultravisor 鐗规潈璧勬簮锛岄渶瑕?Ultravisor 鎺ュ彛鎵嶈兘杩涜鎿嶆帶锛?
      - 澶勭悊鍣ㄩ厤缃瘎瀛樺櫒锛圫COM锛夈€?
      - 鍋滄鐘舵€侊紙stop state锛変俊鎭€?
      - 璋冭瘯瀵勫瓨鍣?CIABR銆丏AWR 鍜?DAWRX锛屽綋 SMFCTRL(D) 琚缃椂銆傚鏋?SMFCTRL(D) 鏈?        璁剧疆锛屽垯瀹冧滑鍦ㄥ畨鍏ㄦā寮忎笅涓嶈捣浣滅敤銆傚綋琚缃椂锛岃鍐欓渶瑕佷竴娆?Ultravisor 璋冪敤锛?        鍚﹀垯灏嗗鑷翠竴娆?Hypervisor Emulation Assistance锛圚ypervisor 浠跨湡杈呭姪锛変腑鏂€?
      - PTCR 涓庡垎鍖鸿〃椤癸紙鍒嗗尯琛ㄤ綅浜庡畨鍏ㄥ唴瀛樹腑锛夈€傚皾璇曞啓鍏?PTCR 灏嗗鑷翠竴娆?Hypervisor
        Emulation Assistance 涓柇銆?
      - LDBAR锛圠D Base Address Register锛屽姞杞藉熀鍧€瀵勫瓨鍣級涓?IMC锛圛n-Memory Collection锛?        鍐呭瓨鍐呴噰闆嗭級闈炴灦鏋勫瘎瀛樺櫒銆傚皾璇曞啓鍏ュ畠浠皢瀵艰嚧涓€娆?Hypervisor Emulation
        Assistance 涓柇銆?
      - SVM 鐨勫垎椤点€佷笌 Hypervisor 鍏变韩 SVM 鐨勫唴瀛樸€傦紙鍖呮嫭 Virtual Processor Area锛圴PA锛?        铏氭嫙澶勭悊鍣ㄥ尯锛変笌铏氭嫙 I/O銆傦級


## 杞欢/寰爜


    Software/Microcode锛堣蒋浠?寰爜锛夋柟闈㈢殑鏀瑰姩鍖呮嫭锛?
    - SVM 鏄娇鐢?IBM 鎻愪緵鐨勶紙寮€婧愶級宸ュ叿浠庢櫘閫?VM 鍒涘缓鐨勩€?
    - 鎵€鏈?SVM 閮戒綔涓烘櫘閫?VM 鍚姩锛屽苟鍒╃敤涓€娆?ultracall锛屽嵆 UV_ESM锛圗nter Secure Mode锛?      杩涘叆瀹夊叏妯″紡锛夋潵瀹屾垚杞崲銆?
    - 褰撹繘琛?UV_ESM ultracall 鏃讹紝Ultravisor 灏?VM 澶嶅埗鍒板畨鍏ㄥ唴瀛橈紝瑙ｅ瘑楠岃瘉淇℃伅锛屽苟
      妫€鏌?SVM 鐨勫畬鏁存€с€傚鏋滃畬鏁存€ф鏌ラ€氳繃锛孶ltravisor 灏嗗湪瀹夊叏妯″紡涓嬬Щ浜ゆ帶鍒舵潈銆?
    - 楠岃瘉淇℃伅鍖呭惈涓?SVM 鍏宠仈鐨勫姞瀵嗙鐩樼殑鍙ｄ护锛坧ass phrase锛夈€傝鍙ｄ护鍦?SVM 璇锋眰鏃?      鎻愪緵缁欏畠銆?
    - Ultravisor 涓嶅弬涓庝繚鎶ゅ浜庨潤姝㈢姸鎬佺殑 SVM 鍔犲瘑纾佺洏銆?
    - 瀵逛簬澶栭儴涓柇锛孶ltravisor 淇濆瓨 SVM 鐨勭姸鎬侊紝骞跺皢涓柇鍙嶅皠缁?Hypervisor 杩涜澶勭悊銆?      瀵逛簬 hypercall锛孶ltravisor 鍚戞墍鏈?hypercall 涓嶉渶瑕佺殑瀵勫瓨鍣ㄦ彃鍏ヤ腑鎬х姸鎬侊紝鐒跺悗灏?      璋冪敤鍙嶅皠缁?Hypervisor 澶勭悊銆侶_RANDOM hypercall 鐢?Ultravisor 鎵ц锛屼笉琚弽灏勩€?
    - 涓轰簡浣胯櫄鎷?I/O 宸ヤ綔锛屽繀椤昏繘琛屽脊璺崇紦鍐诧紙bounce buffering锛夈€?
    - Ultravisor 浣跨敤 AES锛圛APM锛夋潵淇濇姢 SVM 鍐呭瓨銆侷APM 鏄?AES 鐨勪竴绉嶆ā寮忥紝鍙悓鏃舵彁渚?      瀹屾暣鎬т笌鏈哄瘑鎬с€?
    - 鏅€氶〉闈笌瀹夊叏椤甸潰涔嬮棿鏁版嵁鐨勭Щ鍔紝鐢?Hypervisor 涓竴涓柊鐨?HMM 鎻掍欢涓?Ultravisor
      鍗忚皟瀹屾垚銆?
    Ultravisor 鍚?Hypervisor 涓?SVM 鎻愪緵鏂扮殑鏈嶅姟銆傝繖浜涙湇鍔￠€氳繃 ultracall 璁块棶銆?
## 鏈


    - Hypercalls锛堣秴绾ц皟鐢級锛氱敤浜庡悜 Hypervisor 璇锋眰鏈嶅姟鐨勭壒娈婄郴缁熻皟鐢ㄣ€?
    - Normal memory锛堟櫘閫氬唴瀛橈級锛欻ypervisor 鍙闂殑鍐呭瓨銆?
    - Normal page锛堟櫘閫氶〉锛夛細鐢辨櫘閫氬唴瀛樻敮鎸併€佸彲渚?Hypervisor 浣跨敤鐨勯〉銆?
    - Shared page锛堝叡浜〉锛夛細鐢辨櫘閫氬唴瀛樻敮鎸併€丠ypervisor/QEMU 涓?SVM 鍧囧彲璁块棶鐨勯〉
      锛堝嵆璇ラ〉鍦?SVM 涓?Hypervisor/QEMU 涓兘鏈夋槧灏勶級銆?
    - Secure memory锛堝畨鍏ㄥ唴瀛橈級锛氫粎 Ultravisor 涓?SVM 鍙闂殑鍐呭瓨銆?
    - Secure page锛堝畨鍏ㄩ〉锛夛細鐢卞畨鍏ㄥ唴瀛樻敮鎸併€佷粎 Ultravisor 涓?SVM 鍙闂殑椤点€?
    - SVM锛歋ecure Virtual Machine锛堝畨鍏ㄨ櫄鎷熸満锛夈€?
    - Ultracalls锛堣秴绾ц皟鐢級锛氱敤浜庡悜 Ultravisor 璇锋眰鏈嶅姟鐨勭壒娈婄郴缁熻皟鐢ㄣ€?

# Ultravisor 璋冪敤 API


    鏈妭鎻忚堪鏀寔瀹夊叏铏氭嫙鏈猴紙SVM锛変笌鍗婅櫄鎷熷寲 KVM 鎵€闇€鐨?Ultravisor 璋冪敤锛坲ltracall锛夈€?    ultracall 鍏佽 SVM 涓?Hypervisor 鍚?Ultravisor 璇锋眰鏈嶅姟锛屼緥濡傝闂彧鑳藉湪 Ultravisor
    鐗规潈妯″紡涓嬭繍琛屾椂鎵嶈兘璁块棶鐨勫瘎瀛樺櫒鎴栧唴瀛樺尯鍩熴€?
    闇€瑕佺敱 ultracall 鎻愪緵鐨勭壒瀹氭湇鍔″湪瀵勫瓨鍣?R3 涓寚瀹氾紙ultracall 鐨勭涓€涓弬鏁帮級銆?    ultracall 鐨勫叾浠栧弬鏁帮紙濡傛灉鏈夛級鍦ㄥ瘎瀛樺櫒 R4 鍒?R12 涓寚瀹氥€?
    鎵€鏈?ultracall 鐨勮繑鍥炲€奸兘鍦ㄥ瘎瀛樺櫒 R3 涓€倁ltracall 鐨勫叾浠栬緭鍑哄€硷紙濡傛灉鏈夛級鍦ㄥ瘎瀛樺櫒
    R4 鍒?R12 涓繑鍥炪€傝繖绉嶅瘎瀛樺櫒鐢ㄦ硶鍞竴鐨勪緥澶栨槸涓嬮潰鎻忚堪鐨?`UV_RETURN` ultracall銆?
    姣忎釜 ultracall 杩斿洖鍦ㄧ壒瀹?ultracall 涓婁笅鏂囦腑閫傜敤鐨勭壒瀹氶敊璇爜銆備笉杩囷紝涓?PowerPC
    Architecture Platform Reference锛圥APR锛孭owerPC 鏋舵瀯骞冲彴鍙傝€冿級涓€鏍凤紝濡傛灉娌℃湁涓虹壒瀹?    鎯呭喌瀹氫箟鍏蜂綋鐨勯敊璇爜锛岄偅涔?ultracall 灏嗗洖閫€鍒板熀浜庨敊璇弬鏁颁綅缃紙parameter-position
    based锛夌殑鐮侊紝鍗?U_PARAMETER銆乁_P2銆乁_P3 绛夛紝鍙栧喅浜庡彲鑳藉鑷撮敊璇殑 ultracall 鍙傛暟銆?
    涓€浜?ultracall 娑夊強鍦?Ultravisor 涓?Hypervisor 涔嬮棿浼犺緭涓€椤垫暟鎹€備粠瀹夊叏鍐呭瓨浼犺緭鍒?    鏅€氬唴瀛樼殑瀹夊叏椤靛彲浠ヤ娇鐢ㄥ姩鎬佺敓鎴愮殑瀵嗛挜杩涜鍔犲瘑銆傚綋瀹夊叏椤佃浼犲洖瀹夊叏鍐呭瓨鏃讹紝鍙互浣跨敤
    鐩稿悓鐨勫姩鎬佺敓鎴愬瘑閽ヨ繘琛岃В瀵嗐€傝繖浜涘瘑閽ョ殑鐢熸垚涓庣鐞嗗皢鍦ㄥ崟鐙殑鏂囨。涓鏄庛€?
    鐩墠杩欓噷鍙兜鐩?Hypervisor 涓?SVM 褰撳墠宸插疄鐜板苟姝ｅ湪浣跨敤鐨?ultracall锛屼絾鍦ㄥ悎鐞嗘椂鍙互鍦?    姝ゆ坊鍔犲叾浠?ultracall銆?
    鎵€鏈?hypercall/ultracall 鐨勫畬鏁磋鑼冩渶缁堝皢鍦?PAPR 瑙勮寖鐨?public/OpenPower 鐗堟湰涓?    鎻愪緵銆?
```

        If PEF is not enabled, the ultracalls will be redirected to the
        Hypervisor which must handle/fail the calls.

```
## Hypervisor 浣跨敤鐨?Ultracalls


    鏈妭鎻忚堪 Hypervisor 鐢ㄤ簬绠＄悊 SVM 鐨勮櫄鎷熷唴瀛樼鐞?ultracall銆?
### UV_PAGE_OUT


    灏嗕竴椤靛唴瀹瑰姞瀵嗗苟浠庡畨鍏ㄥ唴瀛樼Щ鍔ㄥ埌鏅€氬唴瀛樸€?
#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_OUT,
		uint16_t lpid,		/** LPAR ID **/
		uint64_t dest_ra,	/** real address of destination page **/
		uint64_t src_gpa,	/** source guest-physical-address **/
		uint8_t  flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `dest_ra` is invalid.
 - U_P3		if the `src_gpa` address is invalid.
 - U_P4		if any bit in the `flags` is unrecognized
 - U_P5		if the `order` parameter is unsupported.
 - U_FUNCTION	if functionality is not supported.
 - U_BUSY	if page cannot be currently paged-out.

#### Description


    鍔犲瘑涓€涓?secure-page锛堝畨鍏ㄩ〉锛夌殑鍐呭锛屽苟浣垮叾鍦ㄦ櫘閫氶〉涓彲渚?Hypervisor 浣跨敤銆?
    榛樿鎯呭喌涓嬶紝婧愰〉浼氫粠 SVM 鐨勫垎鍖轰綔鐢ㄥ煙椤佃〃锛坧artition-scoped page table锛変腑鍙栨秷鏄犲皠銆?    浣?Hypervisor 鍙互閫氳繃鍦?`flags` 鍙傛暟涓缃?`UV_SNAPSHOT` 鏍囧織锛屽悜 Ultravisor 鎻愪緵
    淇濈暀璇ラ〉鏄犲皠鐨勬彁绀恒€?
    濡傛灉婧愰〉宸茬粡鏄叡浜〉锛屽垯璇ヨ皟鐢ㄨ繑鍥?U_SUCCESS锛屼笉鍋氫换浣曟搷浣溿€?
#### Use cases


    #. QEMU 灏濊瘯璁块棶灞炰簬 SVM 鐨勬煇涓湴鍧€锛屼絾璇ュ湴鍧€鐨勯〉甯у皻鏈槧灏勫埌 QEMU 鐨勫湴鍧€绌洪棿銆?       鍦ㄨ繖绉嶆儏鍐典笅锛孒ypervisor 灏嗗垎閰嶄竴涓〉甯э紝灏嗗叾鏄犲皠鍒?QEMU 鐨勫湴鍧€绌洪棿锛屽苟鍙戝嚭
       `UV_PAGE_OUT` 璋冪敤浠ュ彇鍥炶椤电殑鍔犲瘑鍐呭銆?
    #. 褰?Ultravisor 瀹夊叏鍐呭瓨涓嶈冻锛岄渶瑕佹崲鍑猴紙page-out锛変竴涓?LRU 椤垫椂銆傛鏃?Ultravisor
       浼氬悜 Hypervisor 鍙戝嚭 `H_SVM_PAGE_OUT` hypercall銆傜劧鍚?Hypervisor 灏嗗垎閰嶄竴涓櫘閫?       椤碉紝骞跺彂鍑?`UV_PAGE_OUT` ultracall锛孶ltravisor 鍒欏皢璇ュ畨鍏ㄩ〉鐨勫唴瀹瑰姞瀵嗗苟绉诲姩鍒?       鏅€氶〉涓€?
    #. 褰?Hypervisor 璁块棶 SVM 鏁版嵁鏃讹紝Hypervisor 璇锋眰 Ultravisor 灏嗙浉搴旂殑椤典紶杈撳埌涓€涓?       闈炲畨鍏ㄩ〉锛孒ypervisor 鍙互璁块棶璇ラ〉銆備笉杩囨櫘閫氶〉涓殑鏁版嵁灏嗘槸鍔犲瘑鐨勩€?
### UV_PAGE_IN


    灏嗕竴椤靛唴瀹逛粠鏅€氬唴瀛樼Щ鍔ㄥ埌瀹夊叏鍐呭瓨銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_IN,
		uint16_t lpid,		/** the LPAR ID **/
		uint64_t src_ra,	/** source real address of page **/
		uint64_t dest_gpa,	/** destination guest physical address **/
		uint64_t flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_BUSY	if page cannot be currently paged-in.
 - U_FUNCTION	if functionality is not supported
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `src_ra` is invalid.
 - U_P3		if the `dest_gpa` address is invalid.
 - U_P4		if any bit in the `flags` is unrecognized
 - U_P5		if the `order` parameter is unsupported.

#### Description


    灏?`src_ra` 鏍囪瘑鐨勯〉鐨勫唴瀹逛粠鏅€氬唴瀛樼Щ鍔ㄥ埌瀹夊叏鍐呭瓨锛屽苟灏嗗叾鏄犲皠鍒板鎴锋満鐗╃悊鍦板潃
    `dest_gpa`銆?
    濡傛灉 `dest_gpa` 寮曠敤涓€涓叡浜湴鍧€锛屽垯灏嗚椤垫槧灏勫埌 SVM 鐨勫垎鍖轰綔鐢ㄥ煙椤佃〃涓€傚鏋?    `dest_gpa` 涓嶆槸鍏变韩鐨勶紝鍒欏皢璇ラ〉鐨勫唴瀹瑰鍒跺埌鐩稿簲鐨勫畨鍏ㄩ〉涓€傛牴鎹笂涓嬫枃锛屽湪澶嶅埗鍓嶅
    璇ラ〉杩涜瑙ｅ瘑銆?
    璋冪敤鑰呴€氳繃 `flags` 鍙傛暟鎻愪緵椤电殑灞炴€с€俙flags` 鐨勬湁鏁堝€间负锛?
 - CACHE_INHIBITED
 - CACHE_ENABLED
 - WRITE_PROTECTION

    鍦ㄨ繘琛?`UV_PAGE_IN` ultracall 涔嬪墠锛孒ypervisor 蹇呴』灏嗛〉鍥哄畾鍦ㄥ唴瀛樹腑銆?
#### Use cases


    #. 褰撴櫘閫?VM 鍒囨崲鍒板畨鍏ㄦā寮忔椂锛屽叾椹荤暀鍦ㄦ櫘閫氬唴瀛樹腑鐨勬墍鏈夐〉閮借绉诲姩鍒板畨鍏ㄥ唴瀛樹腑銆?
    #. 褰?SVM 璇锋眰涓?Hypervisor 鍏变韩涓€椤垫椂锛孒ypervisor 鍒嗛厤涓€椤靛苟鍛婄煡 Ultravisor銆?
    #. 褰?SVM 璁块棶宸茶鎹㈠嚭锛坧age-out锛夌殑瀹夊叏椤垫椂锛孶ltravisor 璋冪敤 Hypervisor 鏉ュ畾浣嶈
       椤点€傚畾浣嶅埌璇ラ〉鍚庯紝Hypervisor 浣跨敤 UV_PAGE_IN 浣胯椤靛 Ultravisor 鍙敤銆?
### UV_PAGE_INVAL


    浣?Ultravisor 瀵逛竴椤电殑鏄犲皠澶辨晥銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_PAGE_INVAL,
		uint16_t lpid,		/** the LPAR ID **/
		uint64_t guest_pa,	/** destination guest-physical-address **/
		uint64_t order)		/** page size order **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `guest_pa` is invalid (or corresponds to a secure
                        page mapping).
 - U_P3		if the `order` is invalid.
 - U_FUNCTION	if functionality is not supported.
 - U_BUSY	if page cannot be currently invalidated.

#### Description


    姝?ultracall 鍛婄煡 Ultravisor锛孒ypervisor 涓搴斾簬缁欏畾瀹㈡埛鏈虹墿鐞嗗湴鍧€鐨勯〉鏄犲皠宸插け鏁堬紝
    Ultravisor 涓嶅簲鍐嶈闂椤点€傚鏋滄寚瀹氱殑 `guest_pa` 瀵瑰簲浜庝竴涓畨鍏ㄩ〉锛孶ltravisor 灏?    蹇界暐浣垮叾澶辨晥鐨勫皾璇曞苟杩斿洖 U_P2銆?
#### Use cases


    #. 褰撳叡浜〉浠?QEMU 鐨勯〉琛ㄤ腑鍙栨秷鏄犲皠锛堝彲鑳芥槸鍥犱负瀹冭鎹㈠嚭鍒扮鐩橈級鏃讹紝Ultravisor 闇€瑕?       鐭ラ亾璇ラ〉涔熶笉搴斾粠瀹冭繖涓€渚ц璁块棶銆?

### UV_WRITE_PATE


    楠岃瘉骞跺啓鍏ョ粰瀹氬垎鍖虹殑鍒嗗尯琛ㄩ」锛圥ATE锛夈€?
#### Syntax


	uint64_t ultracall(const uint64_t UV_WRITE_PATE,
		uint32_t lpid,		/** the LPAR ID **/
		uint64_t dw0		/** the first double word to write **/
		uint64_t dw1)		/** the second double word to write **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_BUSY	if PATE cannot be currently written to.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `dw0` is invalid.
 - U_P3		if the `dw1` address is invalid.
 - U_PERMISSION	if the Hypervisor is attempting to change the PATE
			of a secure virtual machine or if called from a
			context other than Hypervisor.

#### Description


    楠岃瘉骞跺啓鍏ョ粰瀹?LPID 鍙婂叾鍒嗗尯琛ㄩ」銆傚鏋?LPID 宸插垎閰嶅苟鍒濆鍖栵紝姝よ皟鐢ㄥ皢瀵艰嚧鏇存敼鍒嗗尯琛ㄩ」銆?
#### Use cases


    #. 鍒嗗尯琛ㄩ┗鐣欏湪瀹夊叏鍐呭瓨涓紝鍏跺悇椤癸紙绉颁负 PATE锛孭artition Table Entries锛屽垎鍖鸿〃椤癸級
       鎸囧悜 Hypervisor 浠ュ強姣忎釜铏氭嫙鏈猴紙鍖呮嫭瀹夊叏涓庢櫘閫氾級鐨勫垎鍖轰綔鐢ㄥ煙椤佃〃銆侶ypervisor 鍦?       鍒嗗尯 0 涓繍琛岋紝鍏跺垎鍖轰綔鐢ㄥ煙椤佃〃椹荤暀鍦ㄦ櫘閫氬唴瀛樹腑銆?
    #. 姝?ultracall 鍏佽 Hypervisor 鍚?Ultravisor 娉ㄥ唽 Hypervisor 涓庡叾浠栧垎鍖猴紙铏氭嫙鏈猴級
       鐨勫垎鍖轰綔鐢ㄥ煙涓庤繘绋嬩綔鐢ㄥ煙椤佃〃椤广€?
    #. 濡傛灉鐜版湁鍒嗗尯锛圴M锛夌殑 PATE 鍊煎彂鐢熷彉鍖栵紝璇ュ垎鍖虹殑 TLB 缂撳瓨浼氳鍒锋柊銆?
    #. Hypervisor 璐熻矗鍒嗛厤 LPID銆侺PID 涓庡叾 PATE 椤逛竴璧锋敞鍐屻€侶ypervisor 绠＄悊鏅€?VM 鐨?       PATE 椤癸紝骞跺彲浠ラ殢鏃舵洿鏀广€俇ltravisor 绠＄悊 SVM 鐨?PATE 椤癸紝涓嶅厑璁?Hypervisor 淇敼
       瀹冧滑銆?
### UV_RETURN


    鍦ㄥ鐞嗗畬琚浆鍙戯紙鍙堢О **reflected**锛屽弽灏勶級缁?Hypervisor 鐨?hypercall 鎴栦腑鏂悗锛屽皢
    鎺у埗鏉冧粠 Hypervisor 浜よ繕缁?Ultravisor銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_RETURN)

#### Return values


     鎴愬姛鏃舵璋冪敤缁濅笉杩斿洖鍒?Hypervisor銆傚鏋?ultracall 涓嶆槸浠?Hypervisor 涓婁笅鏂囧彂鍑猴紝
     鍒欒繑鍥?U_INVALID銆?
#### Description


    褰?SVM 鍙戝嚭 hypercall 鎴栭伃閬囧叾浠栧紓甯告椂锛孶ltravisor 閫氬父灏嗗紓甯歌浆鍙戯紙鍙堢О **reflects**锛?    鍙嶅皠锛夌粰 Hypervisor銆傚鐞嗗畬寮傚父鍚庯紝Hypervisor 浣跨敤 `UV_RETURN` ultracall 灏嗘帶鍒舵潈
    浜よ繕缁?SVM銆?
    杩涘叆姝?ultracall 鏃舵湡鏈涚殑瀵勫瓨鍣ㄧ姸鎬佷负锛?
    - 闈炴槗澶卞瘎瀛樺櫒琚仮澶嶄负鍏跺師濮嬪€笺€?    - 濡傛灉浠?hypercall 杩斿洖锛屽瘎瀛樺櫒 R0 鍖呭惈杩斿洖鍊硷紙**涓庡叾浠?ultracall 涓嶅悓**锛夛紝骞朵笖
      瀵勫瓨鍣?R4 鍒?R12 鍖呭惈 hypercall 鐨勪换浣曡緭鍑哄€笺€?    - R3 鍖呭惈 ultracall 缂栧彿锛屽嵆 UV_RETURN銆?    - 濡傛灉甯︾潃鍚堟垚鐨勪腑鏂繑鍥烇紝R2 鍖呭惈鍚堟垚鐨勪腑鏂彿銆?
#### Use cases


    #. Ultravisor 渚濊禆 Hypervisor 涓?SVM 鎻愪緵鑻ュ共鏈嶅姟锛屼緥濡傚鐞?hypercall 涓庡叾浠栧紓甯搞€?       澶勭悊瀹屽紓甯稿悗锛孒ypervisor 浣跨敤 UV_RETURN 灏嗘帶鍒舵潈浜よ繕缁?Ultravisor銆?
    #. Hypervisor 蹇呴』浣跨敤姝?ultracall 灏嗘帶鍒舵潈浜よ繕缁?SVM銆?

### UV_REGISTER_MEM_SLOT


    浠ユ寚瀹氬睘鎬ф敞鍐屼竴涓?SVM 鍦板潃鑼冨洿銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_REGISTER_MEM_SLOT,
		uint64_t lpid,		/** LPAR ID of the SVM **/
		uint64_t start_gpa,	/** start guest physical address **/
		uint64_t size,		/** size of address range in bytes **/
		uint64_t flags		/** reserved for future expansion **/
		uint16_t slotid)	/** slot identifier **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `start_gpa` is invalid.
 - U_P3		if `size` is invalid.
 - U_P4		if any bit in the `flags` is unrecognized.
 - U_P5		if the `slotid` parameter is unsupported.
 - U_PERMISSION	if called from context other than Hypervisor.
 - U_FUNCTION	if functionality is not supported.


#### Description


    涓?SVM 娉ㄥ唽涓€涓唴瀛樿寖鍥淬€傝鍐呭瓨鑼冨洿浠庡鎴锋満鐗╃悊鍦板潃 `start_gpa` 寮€濮嬶紝闀垮害涓?`size`
    瀛楄妭銆?
#### Use cases


    #. 褰撹櫄鎷熸満鍙樹负瀹夊叏鏃讹紝Hypervisor 绠＄悊鐨勬墍鏈夊唴瀛樻Ы閮借繘鍏ュ畨鍏ㄥ唴瀛樸€侶ypervisor 閬嶅巻
       姣忎釜鍐呭瓨妲斤紝骞跺悜 Ultravisor 娉ㄥ唽璇ユЫ銆侶ypervisor 鍙兘浼氫涪寮冩煇浜涙Ы锛屼緥濡傜敤浜庡浐浠?       锛圫LOF锛夌殑妲姐€?
    #. 褰撶儹鎻掓嫈锛坔ot-plug锛夋柊鍐呭瓨鏃讹紝浼氭敞鍐屼竴涓柊鐨勫唴瀛樻Ы銆?

### UV_UNREGISTER_MEM_SLOT


    娉ㄩ攢鍏堝墠浣跨敤 UV_REGISTER_MEM_SLOT 娉ㄥ唽鐨?SVM 鍦板潃鑼冨洿銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_UNREGISTER_MEM_SLOT,
		uint64_t lpid,		/** LPAR ID of the SVM **/
		uint64_t slotid)	/** reservation slotid **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_P2 		if `slotid` is invalid.
 - U_PERMISSION	if called from context other than Hypervisor.

#### Description


    閲婃斁鐢?`slotid` 鏍囪瘑鐨勫唴瀛樻Ы锛屽苟閲婃斁鍒嗛厤缁欒棰勭暀鐨勬墍鏈夎祫婧愩€?
#### Use cases


    #. 鍐呭瓨鐑Щ闄わ紙hot-remove锛夈€?

### UV_SVM_TERMINATE


    缁堟涓€涓?SVM 骞堕噴鏀惧叾璧勬簮銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_SVM_TERMINATE,
		uint64_t lpid,		/** LPAR ID of the SVM **/)

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_PARAMETER	if `lpid` is invalid.
 - U_INVALID	if VM is not secure.
 - U_PERMISSION  if not called from a Hypervisor context.

#### Description


    缁堟涓€涓?SVM 骞堕噴鏀惧叾鎵€鏈夎祫婧愩€?
#### Use cases


    #. 鍦ㄧ粓姝?SVM 鏃剁敱 Hypervisor 璋冪敤銆?

## SVM 浣跨敤鐨?Ultracalls


### UV_SHARE_PAGE


    涓?Hypervisor 鍏变韩涓€缁勫鎴锋満鐗╃悊椤点€?
#### Syntax


	uint64_t ultracall(const uint64_t UV_SHARE_PAGE,
		uint64_t gfn,	/** guest page frame number **/
		uint64_t num)	/** number of pages of size PAGE_SIZE **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if the VM is not secure.
 - U_PARAMETER	if `gfn` is invalid.
 - U_P2 		if `num` is invalid.

#### Description


    涓?Hypervisor 鍏变韩浠庡鎴锋満鐗╃悊甯у彿 `gfn` 寮€濮嬬殑 `num` 涓〉銆傚亣璁鹃〉澶у皬涓?PAGE_SIZE
    瀛楄妭銆傚湪杩斿洖鍓嶅皢椤垫竻闆躲€?
    濡傛灉璇ュ湴鍧€宸茬敱瀹夊叏椤垫敮鎸侊紝鍒欏彇娑堣椤电殑鏄犲皠锛屽苟鍦?Hypervisor 鐨勫府鍔╀笅鐢ㄩ潪瀹夊叏椤?    鏀寔瀹冦€傚鏋滃畠杩樻湭琚换浣曢〉鏀寔锛屽垯灏?PTE 鏍囪涓轰笉瀹夊叏锛屽苟鍦ㄨ闂鍦板潃鏃剁敤闈炲畨鍏ㄩ〉
    鏀寔瀹冦€傚鏋滃畠宸茬粡鐢遍潪瀹夊叏椤垫敮鎸侊紝鍒欏皢椤垫竻闆跺苟杩斿洖銆?
#### Use cases


    #. Hypervisor 鏃犳硶璁块棶 SVM 鐨勯〉锛屽洜涓哄畠浠敱瀹夊叏椤垫敮鎸併€傚洜姝?SVM 蹇呴』鏄惧紡鍦板悜
       Ultravisor 璇锋眰鑳戒笌 Hypervisor 鍏变韩鐨勯〉銆?
    #. SVM 涓渶瑕佸叡浜〉鏉ユ敮鎸?virtio 涓?Virtual Processor Area锛圴PA锛岃櫄鎷熷鐞嗗櫒鍖猴級銆?

### UV_UNSHARE_PAGE


    灏嗗叡浜殑 SVM 椤垫仮澶嶅埌鍏跺垵濮嬬姸鎬併€?
#### Syntax


	uint64_t ultracall(const uint64_t UV_UNSHARE_PAGE,
		uint64_t gfn,	/** guest page frame number **/
		uint73 num)	/** number of pages of size PAGE_SIZE**/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if VM is not secure.
 - U_PARAMETER	if `gfn` is invalid.
 - U_P2 		if `num` is invalid.

#### Description


    鍋滄涓?Hypervisor 鍏变韩浠?`gfn` 寮€濮嬬殑 `num` 涓〉銆傚亣璁鹃〉澶у皬涓?PAGE_SIZE銆傚湪杩斿洖鍓?    灏嗛〉娓呴浂銆?
    濡傛灉璇ュ湴鍧€宸茬敱闈炲畨鍏ㄩ〉鏀寔锛屽垯鍙栨秷璇ラ〉鐨勬槧灏勶紝骞剁敤瀹夊叏椤垫敮鎸佸畠銆傚憡鐭?Hypervisor
    閲婃斁瀵瑰叾鍏变韩椤电殑寮曠敤銆傚鏋滆鍦板潃灏氭湭琚〉鏀寔锛屽垯灏?PTE 鏍囪涓哄畨鍏紝骞跺湪璁块棶璇ュ湴鍧€
    鏃剁敤瀹夊叏椤垫敮鎸佸畠銆傚鏋滃畠宸茬粡鐢卞畨鍏ㄩ〉鏀寔锛屽垯灏嗛〉娓呴浂骞惰繑鍥炪€?
#### Use cases


    #. SVM 鍙兘鍐冲畾鍙栨秷涓?Hypervisor 鍏变韩鏌愪釜椤点€?

### UV_UNSHARE_ALL_PAGES


    鍙栨秷 SVM 涓?Hypervisor 鍏变韩鐨勬墍鏈夐〉銆?
#### Syntax


	uint64_t ultracall(const uint64_t UV_UNSHARE_ALL_PAGES)

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success.
 - U_FUNCTION	if functionality is not supported.
 - U_INVAL	if VM is not secure.

#### Description


    鍙栨秷涓?Hypervisor 鍏变韩鐨勬墍鏈夐〉銆傛墍鏈夎鍙栨秷鍏变韩鐨勯〉鍦ㄨ繑鍥炴椂閮借娓呴浂銆傚彧鏈夌敱 SVM 鏄惧紡
    涓?Hypervisor 鍏变韩鐨勯〉锛堜娇鐢?UV_SHARE_PAGE ultracall锛夋墠浼氳鍙栨秷鍏变韩銆俇ltravisor 鍙兘
    鍦ㄥ唴閮ㄤ笌 Hypervisor 鍏变韩鏌愪簺椤佃€屾棤闇€ SVM 鏄惧紡璇锋眰銆傝繖浜涢〉涓嶄細琚 ultracall 鍙栨秷
    鍏变韩銆?
#### Use cases


    #. 褰撲娇鐢?`kexec` 寮曞涓嶅悓鐨勫唴鏍告椂闇€瑕佹璋冪敤銆傚湪 SVM 閲嶇疆鏈熼棿涔熷彲鑳介渶瑕併€?
### UV_ESM


    淇濇姢铏氭嫙鏈猴紙**杩涘叆瀹夊叏妯″紡**锛夈€?
#### Syntax


	uint64_t ultracall(const uint64_t UV_ESM,
		uint64_t esm_blob_addr,	/** location of the ESM blob **/
		unint64_t fdt)		/** Flattened device tree **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - U_SUCCESS	on success (including if VM is already secure).
 - U_FUNCTION	if functionality is not supported.
 - U_INVALID	if VM is not secure.
 - U_PARAMETER	if `esm_blob_addr` is invalid.
 - U_P2 		if `fdt` is invalid.
 - U_PERMISSION	if any integrity checks fail.
 - U_RETRY	insufficient memory to create SVM.
 - U_NO_KEY	symmetric key unavailable.

#### Description


    淇濇姢铏氭嫙鏈恒€傛垚鍔熷畬鎴愬悗锛屽湪 ESM blob 涓寚瀹氱殑鍦板潃灏嗘帶鍒舵潈浜よ繕缁欒櫄鎷熸満銆?
#### Use cases


    #. 鏅€氳櫄鎷熸満鍙互閫夋嫨鍒囨崲鍒板畨鍏ㄦā寮忋€?
# Hypervisor 璋冪敤 API


    鏈枃妗ｆ弿杩版敮鎸?Ultravisor 鎵€闇€鐨?Hypervisor 璋冪敤锛坔ypercall锛夈€侶ypercall 鏄?Hypervisor
    鎻愪緵缁欒櫄鎷熸満涓?Ultravisor 鐨勬湇鍔°€?
    杩欎簺 hypercall 鐨勫瘎瀛樺櫒浣跨敤鏂瑰紡涓?Power Architecture Platform Reference锛圥APR锛夋枃妗?    涓畾涔夌殑鍏朵粬 hypercall 鐩稿悓銆傚嵆鍦ㄨ緭鍏ユ椂锛屽瘎瀛樺櫒 R3 鏍囪瘑鎵€璇锋眰鐨勫叿浣撴湇鍔★紝瀵勫瓨鍣?R4
    鍒?R11 鍖呭惈 hypercall 鐨勫叾浠栧弬鏁帮紙濡傛灉鏈夛級銆傚湪杈撳嚭鏃讹紝瀵勫瓨鍣?R3 鍖呭惈杩斿洖鍊硷紝瀵勫瓨鍣?    R4 鍒?R9 鍖呭惈 hypercall 鐨勪换浣曞叾浠栬緭鍑哄€笺€?
    鏈枃妗ｄ粎娑电洊褰撳墠宸插疄鐜?璁″垝鐢ㄤ簬 Ultravisor 鐨?hypercall锛屼絾鍦ㄥ悎鐞嗘椂鍙互鍦ㄦ娣诲姞
    鍏朵粬 hypercall銆?
    鎵€鏈?hypercall/ultracall 鐨勫畬鏁磋鑼冩渶缁堝皢鍦?PAPR 瑙勮寖鐨?public/OpenPower 鐗堟湰涓?    鎻愪緵銆?
## 鏀寔 Ultravisor 鐨?Hypervisor 璋冪敤


    浠ヤ笅鏄竴缁勬敮鎸?Ultravisor 鎵€闇€鐨?hypercall銆?
### H_SVM_INIT_START


    寮€濮嬪皢鏅€氳櫄鎷熸満杞崲涓?SVM 鐨勮繃绋嬨€?
#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_START)

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - H_SUCCESS	 on success.
        - H_STATE        if the VM is not in a position to switch to secure.

#### Description


    鍚姩淇濇姢铏氭嫙鏈虹殑杩囩▼銆傝繖娑夊強涓?Ultravisor 鍗忚皟锛堜娇鐢?ultracall锛変互鍦?Ultravisor 涓?    涓烘柊 SVM 鍒嗛厤璧勬簮銆佸皢 VM 鐨勯〉浠庢櫘閫氬唴瀛樹紶杈撳埌瀹夊叏鍐呭瓨绛夈€傚綋杩囩▼瀹屾垚鏃讹紝Ultravisor
    鍙戝嚭 H_SVM_INIT_DONE hypercall銆?
#### Use cases


     #. Ultravisor 浣跨敤姝?hypercall 鍛婄煡 Hypervisor 鏌愪釜 VM 宸插惎鍔ㄥ垏鎹㈠埌瀹夊叏妯″紡鐨勮繃绋嬨€?

### H_SVM_INIT_DONE


    瀹屾垚淇濇姢 SVM 鐨勮繃绋嬨€?
#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_DONE)

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - H_SUCCESS 		on success.
 - H_UNSUPPORTED		if called from the wrong context (e.g.
				from an SVM or before an H_SVM_INIT_START
				hypercall).
 - H_STATE		if the hypervisor could not successfully
                                transition the VM to Secure VM.

#### Description


    瀹屾垚淇濇姢铏氭嫙鏈虹殑杩囩▼銆傛璋冪敤蹇呴』鍦ㄥ厛鍓嶇殑 `H_SVM_INIT_START` hypercall 涔嬪悗鍙戝嚭銆?
#### Use cases


    鎴愬姛淇濇姢铏氭嫙鏈哄悗锛孶ltravisor 浼氬憡鐭?Hypervisor銆侶ypervisor 鍙互浣跨敤姝よ皟鐢ㄥ畬鎴愯缃?    璇ヨ櫄鎷熸満鐨勫唴閮ㄧ姸鎬併€?

### H_SVM_INIT_ABORT


    涓淇濇姢 SVM 鐨勮繃绋嬨€?
#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_INIT_ABORT)

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - H_PARAMETER 		on successfully cleaning up the state,
				Hypervisor will return this value to the
				**guest**, to indicate that the underlying
				UV_ESM ultracall failed.

 - H_STATE		if called after a VM has gone secure (i.e
				H_SVM_INIT_DONE hypercall was successful).

 - H_UNSUPPORTED		if called from a wrong context (e.g. from a
				normal VM).

#### Description


    涓淇濇姢铏氭嫙鏈虹殑杩囩▼銆傛璋冪敤蹇呴』鍦ㄥ厛鍓嶇殑 `H_SVM_INIT_START` hypercall 涔嬪悗銆佷笖鍦?    `H_SVM_INIT_DONE` 璋冪敤涔嬪墠鍙戝嚭銆?
    杩涘叆姝?hypercall 鏃讹紝闈炴槗澶?GPR 涓?FPR 搴斿寘鍚?VM 鍙戝嚭 UV_ESM ultracall 鏃跺畠浠墍鍏锋湁
    鐨勫€笺€傛澶栵紝`SRR0` 搴斿寘鍚?UV_ESM ultracall 涔嬪悗閭ｆ潯鎸囦护鐨勫湴鍧€锛宍SRR1` 搴斿寘鍚敤浜?    杩斿洖鍒?VM 鐨?MSR 鍊笺€?
    姝?hypercall 灏嗘竻鐞嗚嚜鍏堝墠鐨?`H_SVM_INIT_START` hypercall 浠ユ潵涓鸿 VM 寤虹珛鐨勪换浣曢儴鍒?    鐘舵€侊紝鍖呮嫭灏嗗凡鎹㈠叆瀹夊叏鍐呭瓨鐨勯〉鎹㈠嚭锛屽苟鍙戝嚭 `UV_SVM_TERMINATE` ultracall 浠ョ粓姝㈣ VM銆?
    娓呯悊瀹岄儴鍒嗙姸鎬佸悗锛屾帶鍒舵潈杩斿洖鍒?VM锛?*鑰岄潪 Ultravisor**锛夛紝鍦板潃涓?`SRR0` 鎵€鎸囧畾锛?    MSR 鍊艰缃负 `SRR1` 涓殑鍊笺€?
#### Use cases


    濡傛灉鍦ㄦ垚鍔熻皟鐢?`H_SVM_INIT_START` 涔嬪悗锛孶ltravisor 鍦ㄤ繚鎶よ櫄鎷熸満鏃堕亣鍒伴敊璇紝鏃犺鏄?    鐢变簬璧勬簮涓嶈冻杩樻槸鐢变簬 VM 鐨勫畨鍏ㄤ俊鎭棤娉曡楠岃瘉锛孶ltravisor 閮戒細鍛婄煡 Hypervisor銆?    Hypervisor 搴斾娇鐢ㄦ璋冪敤娓呯悊璇ヨ櫄鎷熸満鐨勪换浣曞唴閮ㄧ姸鎬佸苟杩斿洖鍒?VM銆?
### H_SVM_PAGE_IN


    灏嗕竴椤靛唴瀹逛粠鏅€氬唴瀛樼Щ鍔ㄥ埌瀹夊叏鍐呭瓨銆?
#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_PAGE_IN,
		uint64_t guest_pa,	/** guest-physical-address **/
		uint64_t flags,		/** flags **/
		uint64_t order)		/** page size order **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - H_SUCCESS	on success.
 - H_PARAMETER	if `guest_pa` is invalid.
 - H_P2		if `flags` is invalid.
 - H_P3		if `order` of page is invalid.

#### Description


    鍙栧洖灞炰簬 VM銆佷綅浜庢寚瀹氬鎴锋満鐗╃悊鍦板潃鐨勯〉鐨勫唴瀹广€?
    `flags` 涓粎鏈夌殑鏈夋晥鍊间负锛?
        - H_PAGE_IN_SHARED 琛ㄧず灏嗕笌 Ultravisor 鍏变韩璇ラ〉銆?
 - H_PAGE_IN_NONSHARED 琛ㄧず UV 涓嶅啀瀵硅椤垫劅鍏磋叮銆傞€傜敤浜庤椤典负鍏变韩椤电殑鎯呭喌銆?
    `order` 鍙傛暟蹇呴』瀵瑰簲浜庨厤缃ソ鐨勯〉澶у皬銆?
#### Use cases


    #. 褰撴櫘閫?VM 鍙樹负瀹夊叏 VM锛堜娇鐢?UV_ESM ultracall锛夋椂锛孶ltravisor 浣跨敤姝?hypercall
       灏?VM 姣忎竴椤电殑鍐呭浠庢櫘閫氬唴瀛樼Щ鍔ㄥ埌瀹夊叏鍐呭瓨銆?
    #. Ultravisor 浣跨敤姝?hypercall 璇锋眰 Hypervisor 鎻愪緵涓€涓彲鍦?SVM 涓?Hypervisor 涔嬮棿
       鍏变韩鐨勬櫘閫氬唴瀛橀〉銆?
    #. Ultravisor 浣跨敤姝?hypercall 鎹㈠叆锛坧age-in锛変竴涓鎹㈠嚭鐨勯〉銆傝繖鍙湪 SVM 瑙︾涓€涓?       琚崲鍑虹殑椤垫椂鍙戠敓銆?
    #. 濡傛灉 SVM 鎯崇姝笌 Hypervisor 鍏变韩椤碉紝瀹冨彲浠ュ憡鐭?Ultravisor 杩欐牱鍋氥€俇ltravisor
       闅忓悗灏嗕娇鐢ㄦ hypercall 骞跺憡鐭?Hypervisor 瀹冨凡閲婃斁瀵硅鏅€氶〉鐨勮闂€?
### H_SVM_PAGE_OUT


    灏嗛〉鐨勫唴瀹圭Щ鍔ㄥ埌鏅€氬唴瀛樸€?
#### Syntax


	uint64_t hypercall(const uint64_t H_SVM_PAGE_OUT,
		uint64_t guest_pa,	/** guest-physical-address **/
		uint64_t flags,		/** flags (currently none) **/
		uint64_t order)		/** page size order **/

#### Return values


    浠ヤ笅鍊间箣涓€锛?
 - H_SUCCESS	on success.
 - H_PARAMETER	if `guest_pa` is invalid.
 - H_P2		if `flags` is invalid.
 - H_P3		if `order` is invalid.

#### Description


    灏?`guest_pa` 鏍囪瘑鐨勯〉鐨勫唴瀹圭Щ鍔ㄥ埌鏅€氬唴瀛樸€?
    鐩墠 `flags` 鏈娇鐢紝蹇呴』璁剧疆涓?0銆俙order` 鍙傛暟蹇呴』瀵瑰簲浜庨厤缃ソ鐨勯〉澶у皬銆?
#### Use cases


    #. 濡傛灉 Ultravisor 鐨勫畨鍏ㄩ〉涓嶈冻锛屽畠鍙互浣跨敤姝?hypercall 灏嗘煇浜涘畨鍏ㄩ〉鐨勫唴瀹圭Щ鍔ㄥ埌
       鏅€氶〉涓€傚唴瀹瑰皢琚姞瀵嗐€?
# 鍙傝€?

- `Supporting Protected Computing on IBM Power Architecture <https://developer.ibm.com/articles/l-support-protected-computing/>`_
