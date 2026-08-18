
## 鏃堕挓涓庡畾鏃跺櫒


### arm64

鍦?arm64 涓婏紝Hyper-V 瀵?ARMv8 鏋舵瀯鐨勭郴缁熻鏁板櫒锛坰ystem counter锛夊拰瀹氭椂鍣紙timer锛?杩涜浜嗚櫄鎷熷寲銆傚鎴锋満 VM 閫氳繃鏍囧噯鐨?arm_arch_timer.c 椹卞姩灏嗚繖涓€铏氭嫙鍖栫殑纭欢鐢ㄤ綔
Linux 鐨?clocksource 鍜?clockevents锛屽氨鍍忓湪瑁告満涓婁竴鏍枫€傚湪 Hyper-V 涓婄殑瀹㈡埛鏈?VM
涓紝閽堝鏋舵瀯绯荤粺璁℃暟鍣ㄧ殑 Linux vDSO 鏀寔鏄彲鐢ㄧ殑銆傝櫧鐒?Hyper-V 杩樻彁渚涗簡涓€涓悎鎴?绯荤粺鏃堕挓鍜屽洓涓悎鎴?per-CPU 瀹氭椂鍣紙濡?TLFS 涓墍杩帮級锛屼絾鍦?arm64 涓婄殑 Hyper-V 瀹㈡埛鏈?涓紝Linux 鍐呮牳骞舵湭浣跨敤瀹冧滑銆備笉杩囷紝杈冩棫鐗堟湰鐨?arm64 Hyper-V 鍙儴鍒嗚櫄鎷熷寲 ARMv8
鏋舵瀯瀹氭椂鍣紝瀵艰嚧璇ュ畾鏃跺櫒涓嶄細鍦?VM 涓敓鎴愪腑鏂€傜敱浜庤繖涓€闄愬埗锛屽湪杩欎簺杈冩棫鐨?Hyper-V 鐗堟湰涓婅繍琛屽綋鍓嶇殑 Linux 鍐呮牳鐗堟湰锛岄渶瑕佷竴涓爲澶栵紙out-of-tree锛夎ˉ涓侊紝浠ユ敼鐢?Hyper-V 鍚堟垚鏃堕挓/瀹氭椂鍣ㄣ€?
### x86/x64

鍦?x86/x64 涓婏紝Hyper-V 鍚戝鎴锋満 VM 鎻愪緵濡?TLFS 涓墍杩扮殑鍚堟垚绯荤粺鏃堕挓鍜屽洓涓悎鎴?per-CPU 瀹氭椂鍣ㄣ€侶yper-V 杩橀€氳繃 RDTSC 鍙婄浉鍏虫寚浠ゆ彁渚涘铏氭嫙鍖?TSC 鐨勮闂€傝繖浜?TSC
鎸囦护涓嶄細闄峰叆锛坱rap锛夊埌 hypervisor锛屽洜姝ゅ湪 VM 涓彁渚涘嚭鑹茬殑鎬ц兘銆侶yper-V 鎵ц TSC
鏍″噯锛屽苟閫氳繃涓€涓悎鎴?MSR 灏?TSC 棰戠巼鎻愪緵缁欏鎴锋満 VM銆侺inux 涓殑 Hyper-V 鍒濆鍖?浠ｇ爜璇诲彇璇?MSR 浠ヨ幏鍙栭鐜囷紝鍥犳瀹冧細璺宠繃 TSC 鏍″噯骞惰缃?tsc_reliable銆侶yper-V 鎻愪緵
浜嗚櫄鎷熷寲鐨?PIT锛堜粎闄?Hyper-V 绗竴浠?VM锛夈€乴ocal APIC timer 鍜?RTC銆侶yper-V 涓嶅湪
瀹㈡埛鏈?VM 涓彁渚涜櫄鎷熷寲鐨?HPET銆?
Hyper-V 鍚堟垚绯荤粺鏃堕挓鍙互閫氳繃涓€涓悎鎴?MSR 璇诲彇锛屼絾杩欑璁块棶浼氶櫡鍏ュ埌 hypervisor銆備綔涓?鏇村揩鐨勬浛浠ｆ柟妗堬紝瀹㈡埛鏈哄彲浠ラ厤缃竴涓湪瀹㈡埛鏈轰笌 hypervisor 涔嬮棿鍏变韩鐨勫唴瀛橀〉銆侶yper-V
鍦ㄨ鍐呭瓨椤典腑濉叆涓€涓?64 浣嶇殑 scale 鍊煎拰 offset 鍊笺€傝璇诲彇鍚堟垚鏃堕挓鐨勫€硷紝瀹㈡埛鏈鸿鍙?TSC锛岀劧鍚庢寜鐓?Hyper-V TLFS 涓殑鎻忚堪搴旂敤 scale 鍜?offset銆傚緱鍒扮殑缁撴灉浠ユ亽瀹氱殑 10 MHz
棰戠巼鍓嶈繘銆傚湪瀹炴椂杩佺Щ鍒板叿鏈変笉鍚?TSC 棰戠巼鐨勪富鏈虹殑鎯呭喌涓嬶紝Hyper-V 浼氳皟鏁村叡浜〉涓殑
scale 鍜?offset 鍊硷紝浠ョ淮鎸?10 MHz 鐨勯鐜囥€?
浠?Windows Server 2022 Hyper-V 寮€濮嬶紝Hyper-V 浣跨敤瀵?TSC 棰戠巼缂╂斁鐨勭‖浠舵敮鎸侊紝浠?瀹炵幇 VM 鍦?TSC 棰戠巼鍙兘涓嶅悓鐨?Hyper-V 涓绘満涔嬮棿鐨勫疄鏃惰縼绉汇€傚綋 Linux 瀹㈡埛鏈烘娴嬪埌璇?Hyper-V 鍔熻兘鍙敤鏃讹紝瀹冨€惧悜浜庝娇鐢?Linux 鏍囧噯鐨勫熀浜?TSC 鐨?clocksource銆傚惁鍒欙紝瀹冧細
浣跨敤閫氳繃鍏变韩椤靛疄鐜扮殑 Hyper-V 鍚堟垚绯荤粺鏃堕挓鐨?clocksource锛堟爣璇嗕负
"hyperv_clocksource_tsc_page"锛夈€?
Hyper-V 鍚堟垚绯荤粺鏃堕挓鍙€氳繃 vDSO 鎻愪緵缁欑敤鎴风┖闂达紝gettimeofday() 鍙婄浉鍏崇殑绯荤粺璋冪敤
鍙互瀹屽叏鍦ㄧ敤鎴风┖闂翠腑鎵ц銆倂DSO 閫氳繃灏嗗甫鏈?scale 鍜?offset 鍊肩殑鍏变韩椤垫槧灏勫埌鐢ㄦ埛绌洪棿
鏉ュ疄鐜般€傜敤鎴风┖闂翠唬鐮佹墽琛岀浉鍚岀殑绠楁硶锛氳鍙?TSC 骞跺簲鐢?scale 鍜?offset 鏉ュ緱鍒版亽瀹氱殑
10 MHz 鏃堕挓銆?
Linux 鐨?clockevents 鍩轰簬 Hyper-V 鍚堟垚瀹氭椂鍣?0锛坰timer0锛夈€傝櫧鐒?Hyper-V 涓烘瘡涓?CPU
鎻愪緵 4 涓悎鎴愬畾鏃跺櫒锛屼絾 Linux 鍙娇鐢ㄥ畾鏃跺櫒 0銆傚湪杈冩棫鐗堟湰鐨?Hyper-V 涓紝鏉ヨ嚜 stimer0
鐨勪腑鏂細浜х敓涓€涓?VMBus 鎺у埗娑堟伅锛岀敱 vmbus_isr() 杩涜瑙ｅ鐢紝濡?Documentation/virt/hyperv/vmbus.rst 鏂囨。涓墍杩般€傚湪杈冩柊鐗堟湰鐨?Hyper-V 涓紝stimer0
涓柇鍙互鏄犲皠鍒颁竴涓灦鏋勪腑鏂紝杩欒绉颁负鈥淒irect Mode鈥濓紙鐩存帴妯″紡锛夈€侺inux 鍦ㄥ彲鐢ㄦ椂鍊惧悜
浜庝娇鐢?Direct Mode銆傜敱浜?x86/x64 涓嶆敮鎸?per-CPU 涓柇锛孌irect Mode 浼氬湪鎵€鏈?CPU 涓?闈欐€佸垎閰嶄竴涓?x86 涓柇鍚戦噺锛圚YPERV_STIMER0_VECTOR锛夛紝骞舵樉寮忕紪鐮佷互璋冪敤 stimer0 涓柇
澶勭悊绋嬪簭銆傚洜姝わ紝鏉ヨ嚜 stimer0 鐨勪腑鏂褰曞湪 /proc/interrupts 鐨勨€淗VS鈥濊涓紝鑰屼笉鏄?涓庢煇涓?Linux IRQ 鍏宠仈銆傚熀浜庤櫄鎷熷寲 PIT 鍜?local APIC timer 鐨?clockevents 涔熻兘宸ヤ綔锛?浣?Hyper-V 鐨?stimer0 鏄閫夈€?
Hyper-V 鍚堟垚绯荤粺鏃堕挓鍜屽畾鏃跺櫒鐨勯┍鍔ㄤ綅浜?drivers/clocksource/hyperv_timer.c銆?