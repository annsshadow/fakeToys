
## 閫氱敤 vcpu 鎺ュ彛


铏氭嫙 CPU 鈥滆澶団€?涔熸帴鍙?ioctl KVM_SET_DEVICE_ATTR銆並VM_GET_DEVICE_ATTR 鍜?KVM_HAS_DEVICE_ATTR銆傝鎺ュ彛浣跨敤涓庡叾浠栬澶囩浉鍚岀殑 struct
kvm_device_attr锛屼絾鐩爣鏄?VCPU 绾у埆鐨勮缃拰鎺у埗銆?
姣忎釜铏氭嫙 CPU 鐨勭粍鍜屽睘鎬э紙濡傛灉鏈夌殑璇濓級鏄灦鏋勭浉鍏崇殑銆?
## 1. 缁勶細KVM_ARM_VCPU_PMU_V3_CTRL


:Architectures: ARM64

### 1.1. 灞炴€э細KVM_ARM_VCPU_PMU_V3_IRQ


:Parameters: in kvm_device_attr.addr the address for PMU overflow interrupt is a
	     pointer to an int

杩斿洖锛?
	 =======  ========================================================
	 -EBUSY   PMU 婧㈠嚭涓柇宸茬粡璁剧疆
	 -EFAULT  璇诲彇涓柇鍙锋椂鍑洪敊
	 -ENXIO   PMUv3 涓嶆敮鎸侊紝鎴栬€呭皾璇曡幏鍙栨椂婧㈠嚭涓柇鏈缃?	 -ENODEV  VCPU 缂哄皯 KVM_ARM_VCPU_PMU_V3 鐗规€?	 -EINVAL  鎻愪緵浜嗘棤鏁堢殑 PMU 婧㈠嚭涓柇鍙凤紝鎴栬€?		  鍦ㄦ湭浣跨敤鍐呮牳鍐?irqchip 鐨勬儏鍐典笅灏濊瘯璁剧疆 IRQ 鍙枫€?	 =======  ========================================================

鎻忚堪姝?vcpu 鐨?PMUv3锛圥erformance Monitor Unit v3锛屾€ц兘鐩戣鍗曞厓 v3锛夋孩鍑轰腑鏂彿鐨勪竴涓€笺€傝涓柇鍙互鏄?PPI 鎴?SPI锛屼絾姣忎釜 vcpu 鐨勪腑鏂被鍨嬪繀椤荤浉鍚屻€備綔涓?PPI 鏃讹紝鎵€鏈?vcpu 鐨勪腑鏂彿鐩稿悓锛涜€屼綔涓?SPI 鏃讹紝姣忎釜 vcpu 蹇呴』鏄崟鐙殑涓柇鍙枫€傚浜庡熀浜?GICv5 鐨勫鎴锋満锛屽繀椤讳娇鐢ㄦ灦鏋勮瀹氱殑 PPI锛?3锛夈€?
### 1.2 灞炴€э細KVM_ARM_VCPU_PMU_V3_INIT


:Parameters: no additional parameter in kvm_device_attr.addr

杩斿洖锛?
	 =======  ======================================================
	 -EEXIST  涓柇鍙峰凡琚娇鐢?	 -ENODEV  PMUv3 涓嶆敮鎸佹垨 GIC 鏈垵濮嬪寲
	 -ENXIO   PMUv3 涓嶆敮鎸併€佺己灏?VCPU 鐗规€ф垨涓柇鍙锋湭璁剧疆
		  锛堜粎闈?GICv5 瀹㈡埛鏈猴級
	 -EBUSY   PMUv3 宸茬粡鍒濆鍖?	 =======  ======================================================

璇锋眰鍒濆鍖?PMUv3銆傚鏋滈厤鍚堝唴鏍稿唴铏氭嫙 GIC 瀹炵幇浣跨敤 PMUv3锛岃繖蹇呴』鍦ㄥ垵濮嬪寲鍐呮牳鍐?irqchip 涔嬪悗杩涜銆?
### 1.3 灞炴€э細KVM_ARM_VCPU_PMU_V3_FILTER


:Parameters: in kvm_device_attr.addr the address for a PMU event filter is a
             pointer to a struct kvm_pmu_event_filter

:Returns:

	 =======  ======================================================
	 -ENODEV  PMUv3 涓嶆敮鎸佹垨 GIC 鏈垵濮嬪寲
	 -ENXIO   PMUv3 鏈纭厤缃紝鎴栬€呰皟鐢ㄦ灞炴€у墠鏈寜瑕佹眰
	 	  閰嶇疆鍐呮牳鍐?irqchip
	 -EBUSY   PMUv3 宸茬粡鍒濆鍖栵紝鎴栬€呮煇涓?VCPU 宸茬粡杩愯杩?	 -EINVAL  鏃犳晥鐨勮繃婊ゅ櫒鑼冨洿
	 =======  ======================================================

```

    struct kvm_pmu_event_filter {
	    __u16	base_event;
	    __u16	nevents;

    #define KVM_PMU_EVENT_ALLOW	0
    #define KVM_PMU_EVENT_DENY	1

	    __u8	action;
	    __u8	pad[3];
    };

```
涓€涓繃婊ゅ櫒鑼冨洿瀹氫箟涓鸿寖鍥?[@base_event, @base_event + @nevents)锛岃繛鍚?@action锛圞VM_PMU_EVENT_ALLOW 鎴?KVM_PMU_EVENT_DENY锛夈€傜涓€涓敞鍐岀殑鑼冨洿瀹氫箟浜嗗叏灞€绛栫暐锛堝鏋滅涓€涓?@action 鏄?DENY锛屽垯涓哄叏灞€ ALLOW锛涘鏋滅涓€涓?@action 鏄?ALLOW锛屽垯涓哄叏灞€ DENY锛夈€傚彲浠ョ紪绋嬪涓寖鍥达紝骞朵笖蹇呴』閫傞厤 PMU 鏋舵瀯鎵€瀹氫箟鐨勪簨浠剁┖闂达紙ARMv8.0 涓婁负 10 浣嶏紝浠?ARMv8.1 璧蜂负 16 浣嶏級銆?
娉ㄦ剰锛氶€氳繃涓哄悓涓€鑼冨洿娉ㄥ唽鐩稿弽鐨勫姩浣滄潵 鈥滃彇娑堚€?涓€涓繃婊ゅ櫒骞朵笉浼氭敼鍙橀粯璁ゅ姩浣溿€備緥濡傦紝鍏堝皢浜嬩欢鑼冨洿 [0:10) 鐨?ALLOW 杩囨护鍣ㄤ綔涓虹涓€涓繃婊ゅ櫒瀹夎锛岀劧鍚庡璇ヨ寖鍥村簲鐢?DENY 鍔ㄤ綔锛屽皢浣挎暣涓寖鍥翠繚鎸佺鐢ㄧ姸鎬併€?
闄愬埗锛氫簨浠?0锛圫W_INCR锛夋案杩滀笉浼氳杩囨护锛屽洜涓哄畠涓嶇粺璁＄‖浠朵簨浠躲€傝繃婊や簨浠?0x1E锛圕HAIN锛変篃娌℃湁鏁堟灉锛屽洜涓哄畠涓ユ牸鏉ヨ涓嶆槸涓€涓簨浠躲€傚彲浠ヤ娇鐢ㄤ簨浠?0x11锛圕PU_CYCLES锛夋潵杩囨护鍛ㄦ湡璁℃暟鍣ㄣ€?
### 1.4 灞炴€э細KVM_ARM_VCPU_PMU_V3_SET_PMU


:Parameters: in kvm_device_attr.addr the address to an int representing the PMU
             identifier.

:Returns:

	 =======  ====================================================
	 -EBUSY   PMUv3 宸茬粡鍒濆鍖栥€佹煇涓?VCPU 宸茬粡杩愯杩囷紝鎴栬€?                  宸茬粡璁剧疆浜嗕竴涓簨浠惰繃婊ゅ櫒
	 -EFAULT  璁块棶 PMU 鏍囪瘑绗︽椂鍑洪敊
	 -ENXIO   鏈壘鍒?PMU
	 -ENODEV  PMUv3 涓嶆敮鎸佹垨 GIC 鏈垵濮嬪寲
	 -ENOMEM  鏃犳硶鍒嗛厤鍐呭瓨
	 =======  ====================================================

璇锋眰 VCPU 鍦ㄥ垱寤哄鎴锋満浜嬩欢鐢ㄤ簬 PMU 浠跨湡鏃朵娇鐢ㄦ寚瀹氱殑纭欢 PMU銆侾MU 鏍囪瘑绗﹀彲浠ヤ粠 /sys/devices 涓嬫墍闇€ PMU 瀹炰緥鐨?鈥渢ype鈥?鏂囦欢锛堟垨绛変环鐨?/sys/bus/even_source锛夎鍙栥€傛灞炴€у湪鑷冲皯鏈変袱涓?CPU PMU 鐨勫紓鏋勭郴缁熶笂鐗瑰埆鏈夌敤銆備负涓€涓?VCPU 璁剧疆鐨?PMU 灏嗚鎵€鏈夊叾浠?VCPU 浣跨敤銆傚鏋滃凡缁忓瓨鍦?PMU 浜嬩欢杩囨护鍣紝鍒欐棤娉曡缃?PMU銆?
娉ㄦ剰锛孠VM 涓嶄細灏濊瘯灏嗘灞炴€ф寚瀹氱殑銆佷笌 PMU 鐩稿叧鑱旂殑鐗╃悊 CPU 涓婅繍琛?VCPU銆傝繖瀹屽叏鐣欑粰鐢ㄦ埛绌洪棿澶勭悊銆傜劧鑰岋紝灏濊瘯鍦ㄤ笌 PMU 涓嶆敮鎸佺殑鐗╃悊 CPU 涓婅繍琛?VCPU 灏嗕細澶辫触锛孠VM_RUN 灏嗕互
exit_reason = KVM_EXIT_FAIL_ENTRY 杩斿洖锛屽苟閫氳繃灏?hardare_entry_failure_reason 瀛楁璁句负 KVM_EXIT_FAIL_ENTRY_CPU_UNSUPPORTED銆佸皢 cpu 瀛楁璁句负澶勭悊鍣?id 鏉ュ～鍏?fail_entry 缁撴瀯銆?
### 1.5 灞炴€э細KVM_ARM_VCPU_PMU_V3_SET_NR_COUNTERS


:Parameters: in kvm_device_attr.addr the address to an unsigned int
	     representing the maximum value taken by PMCR_EL0.N

:Returns:

	 =======  ====================================================
	 -EBUSY   PMUv3 宸茬粡鍒濆鍖栥€佹煇涓?VCPU 宸茬粡杩愯杩囷紝鎴栬€?                  宸茬粡璁剧疆浜嗕簨浠惰繃婊ゅ櫒
	 -EFAULT  璁块棶 addr 鎵€鎸囧悜鐨勫€兼椂鍑洪敊
	 -ENODEV  PMUv3 涓嶆敮鎸佹垨 GIC 鏈垵濮嬪寲
	 -EINVAL  鏈樉寮忛€夋嫨 PMUv3锛屾垨鑰?N 鐨勫€艰秴鍑鸿寖鍥?	 =======  ====================================================

璁剧疆铏氭嫙 PMU 涓疄鐜扮殑浜嬩欢璁℃暟鍣ㄦ暟閲忋€傝繖瑕佹眰宸查€氳繃 KVM_ARM_VCPU_PMU_V3_SET_PMU 鏄惧紡閫夋嫨浜嗕竴涓?PMU锛屽苟涓斿綋鏈樉寮忛€夋嫨 PMU銆佹垨鑰呰鏁板櫒鏁伴噺瓒呭嚭鎵€閫?PMU 鐨勮寖鍥存椂浼氬け璐ャ€傞€夋嫨鏂扮殑 PMU 浼氬彇娑堣缃灞炴€х殑鏁堟灉銆?
## 2. 缁勶細KVM_ARM_VCPU_TIMER_CTRL


:Architectures: ARM64

### 2.1. 灞炴€э細KVM_ARM_VCPU_TIMER_IRQ_{VTIMER,PTIMER,HVTIMER,HPTIMER}


:Parameters: in kvm_device_attr.addr the address for the timer interrupt is a
	     pointer to an int

杩斿洖锛?
	 =======  =================================
	 -EINVAL  鏃犳晥鐨勫畾鏃跺櫒涓柇鍙?	 -EBUSY   涓€涓垨澶氫釜 VCPU 宸茬粡杩愯
	 =======  =================================

鎻忚堪杩炴帴鍒板唴鏍稿唴铏氭嫙 GIC 鏃剁殑鏋舵瀯瀹氭椂鍣ㄤ腑鏂彿銆傚畠浠繀椤绘槸 PPI锛?6 <= intid < 32锛夈€傝缃灞炴€т細瑕嗙洊榛樿鍊硷紙瑙佷笅鏂囷級銆?
==============================  ==========================================
KVM_ARM_VCPU_TIMER_IRQ_VTIMER   EL1 铏氭嫙瀹氭椂鍣?intid锛堥粯璁わ細27锛?KVM_ARM_VCPU_TIMER_IRQ_PTIMER   EL1 鐗╃悊瀹氭椂鍣?intid锛堥粯璁わ細30锛?KVM_ARM_VCPU_TIMER_IRQ_HVTIMER  EL2 铏氭嫙瀹氭椂鍣?intid锛堥粯璁わ細28锛?KVM_ARM_VCPU_TIMER_IRQ_HPTIMER  EL2 鐗╃悊瀹氭椂鍣?intid锛堥粯璁わ細26锛?==============================  ==========================================

涓轰笉鍚岀殑瀹氭椂鍣ㄨ缃浉鍚岀殑 PPI 浼氶樆姝?VCPU 杩愯銆傚湪鏌愪釜 VCPU 涓婅缃腑鏂彿浼氬皢褰撴椂鍒涘缓鐨勬墍鏈?VCPU 閰嶇疆涓哄缁欏畾瀹氭椂鍣ㄤ娇鐢ㄨ鍙风爜锛岃鐩栧叾浠?VCPU 涓婁箣鍓嶉厤缃殑浠讳綍鍊笺€傜敤鎴风┖闂村簲鍦ㄥ垱寤烘墍鏈?VCPU 涔嬪悗銆佽繍琛屼换浣?VCPU 涔嬪墠锛屽湪鑷冲皯涓€涓?VCPU 涓婇厤缃腑鏂彿銆?

## 3. 缁勶細KVM_ARM_VCPU_PVTIME_CTRL


:Architectures: ARM64

### 3.1 灞炴€э細KVM_ARM_VCPU_PVTIME_IPA


:Parameters: 64-bit base address

杩斿洖锛?
	 =======  ======================================
	 -ENXIO   鏈疄鐜扮獌鍙栨椂闂?	 -EEXIST  姝?VCPU 鐨勫熀鍦板潃宸茬粡璁剧疆
	 -EINVAL  鍩哄湴鍧€鏈寜 64 瀛楄妭瀵归綈
	 =======  ======================================

鎸囧畾姝?VCPU 鐨勭獌鍙栨椂闂寸粨鏋勭殑鍩哄湴鍧€銆傚熀鍦板潃蹇呴』鎸?64 瀛楄妭瀵归綈锛屽苟涓斾綅浜庢湁鏁堢殑瀹㈡埛鏈哄唴瀛樺尯鍩熷唴銆傛洿澶氫俊鎭紙鍖呮嫭绐冨彇鏃堕棿缁撴瀯鐨勫竷灞€锛夎鍙傝 Documentation/virt/kvm/arm/pvtime.rst銆?
## 4. 缁勶細KVM_VCPU_TSC_CTRL


:Architectures: x86

4.1 灞炴€э細KVM_VCPU_TSC_OFFSET

:Parameters: 64-bit unsigned TSC offset

杩斿洖锛?
	 ======= ======================================
	 -EFAULT 璇诲彇/鍐欏叆鎵€鎻愪緵鐨勫弬鏁板湴鍧€鏃跺嚭閿欍€?	 -ENXIO  灞炴€т笉鍙楁敮鎸?	 ======= ======================================

鎸囧畾瀹㈡埛鏈虹浉瀵逛簬涓绘満鐨?TSC 鍋忕Щ銆傚鎴锋満鐨?TSC 鐒跺悗閫氳繃浠ヤ笅绛夊紡鎺ㄥ锛?
  guest_tsc = host_tsc + KVM_VCPU_TSC_OFFSET

姝ゅ睘鎬у彲鐢ㄤ簬鍦ㄥ疄鏃惰縼绉绘椂璋冩暣瀹㈡埛鏈虹殑 TSC锛屼娇 TSC 璁″叆 VM 琚殏鍋滄湡闂寸殑鏃堕棿銆備笅闈㈡弿杩颁簡鐢ㄤ簬姝ょ洰鐨勭殑涓€绉嶅彲鑳界畻娉曘€?
鏉ヨ嚜婧?VMM 杩涚▼锛?
1. 璋冪敤 KVM_GET_CLOCK ioctl 璁板綍涓绘満 TSC锛坱sc_src锛夈€乲vmclock 绾崇锛坓uest_src锛夊拰涓绘満 CLOCK_REALTIME 绾崇锛坔ost_src锛夈€?
2. 璇诲彇姣忎釜 vCPU 鐨?KVM_VCPU_TSC_OFFSET 灞炴€т互璁板綍瀹㈡埛鏈?TSC 鍋忕Щ锛坥fs_src[i]锛夈€?
3. 璋冪敤 KVM_GET_TSC_KHZ ioctl 璁板綍瀹㈡埛鏈?TSC 鐨勯鐜囷紙freq锛夈€?
鏉ヨ嚜鐩爣 VMM 杩涚▼锛?
4. 璋冪敤 KVM_SET_CLOCK ioctl锛屽湪鍚勮嚜瀛楁涓彁渚涙潵鑷?kvmclock 鐨勬簮绾崇锛坓uest_src锛夊拰 CLOCK_REALTIME锛坔ost_src锛夈€傜‘淇濆湪鎵€鎻愪緵鐨勭粨鏋勪腑璁剧疆浜?KVM_CLOCK_REALTIME 鏍囧織銆?
   KVM 灏嗘帹杩?VM 鐨?kvmclock锛屼互璁″叆璁板綍鏃堕挓鍊间互鏉ョ粡杩囩殑鏃堕棿銆傛敞鎰忥紝闄ら潪婧愬拰鐩爣涔嬮棿鐨?CLOCK_REALTIME 鏄悓姝ョ殑锛屽苟涓旀簮鏆傚仠 VM 涓庣洰鏍囨墽琛屾楠?4-7 涔嬮棿缁忚繃鐨勬椂闂磋冻澶熺煭锛屽惁鍒欒繖浼氬湪瀹㈡埛鏈轰腑寮曞彂闂锛堜緥濡傝秴鏃讹級銆?
5. 璋冪敤 KVM_GET_CLOCK ioctl 璁板綍涓绘満 TSC锛坱sc_dest锛夊拰 kvmclock 绾崇锛坓uest_dest锛夈€?
6. 璋冩暣姣忎釜 vCPU 鐨勫鎴锋満 TSC 鍋忕Щ锛屼互璁″叆锛?锛夎褰曠姸鎬佷互鏉ョ粡杩囩殑鏃堕棿锛屼互鍙婏紙2锛夋簮鏈哄櫒鍜岀洰鏍囨満鍣ㄤ箣闂?TSC 鐨勫樊寮傦細

   ofs_dst[i] = ofs_src[i] -
     (guest_src - guest_dest) * freq +
     (tsc_src - tsc_dest)

   锛堚€渙fs[i] + tsc - guest * freq鈥?鏄搴斾簬 kvmclock 涓椂闂?0 鐨勫鎴锋満 TSC 鍊笺€備笂杩板叕寮忕‘淇濆畠涓庢簮涓婄浉鍚岋紝鍦ㄧ洰鏍囦笂涔熺浉鍚岋級銆?
7. 鐢ㄥ墠涓€姝ユ帹瀵煎嚭鐨勫悇鑷€煎啓鍑烘瘡涓?vCPU 鐨?KVM_VCPU_TSC_OFFSET 灞炴€с€?