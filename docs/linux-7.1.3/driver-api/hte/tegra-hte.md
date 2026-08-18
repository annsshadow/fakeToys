
## HTE 鍐呮牳鎻愪緵鑰呴┍鍔?

### 鎻忚堪

Nvidia tegra HTE 鎻愪緵鑰咃紙涔熺О涓?GTE锛孏eneric Timestamping Engine锛岄€氱敤鏃堕棿鎴冲紩鎿庯級椹卞姩瀹炵幇浜嗕袱涓?GTE 瀹炰緥锛?) GPIO GTE 鍜?2) LIC锛圠egacy Interrupt Controller锛屼紶缁熶腑鏂帶鍒跺櫒锛塈RQ GTE銆備袱涓?GTE 瀹炰緥閮戒粠绯荤粺璁℃暟鍣?TSC 鑾峰彇鏃堕棿鎴筹紝鍏舵椂閽熼鐜囦负 31.25MHz锛岄┍鍔ㄥ湪灏嗗叾瀛樺偍涓烘椂闂存埑鍊间箣鍓嶄細灏嗘椂閽熻妭鎷嶇巼杞崲涓虹撼绉掋€?
### GPIO GTE


璇?GTE 瀹炰緥瀵?GPIO 杩涜瀹炴椂鏃堕棿鎴虫爣璁般€備负姝わ紝GPIO 闇€瑕佽閰嶇疆涓鸿緭鍏ャ€傚彧鏈夊父寮€锛圓ON锛塆PIO 鎺у埗鍣ㄥ疄渚嬫敮鎸佸 GPIO 杩涜瀹炴椂鏃堕棿鎴虫爣璁帮紝鍥犱负瀹冧笌 GPIO GTE 绱у瘑鑰﹀悎銆備负姝わ紝GPIOLIB 鏂板浜嗕袱涓彲閫?API锛屽涓嬫墍杩般€侴PIO GTE 浠ｇ爜鍚屾椂鏀寔鍐呮牳鎬佸拰鐢ㄦ埛鎬佹秷璐硅€呫€傚唴鏍告€佹秷璐硅€呭彲浠ョ洿鎺ヤ笌 HTE 瀛愮郴缁熼€氫俊锛岃€岀敤鎴锋€佹秷璐硅€呯殑鏃堕棿鎴宠姹傚垯缁忕敱 GPIOLIB CDEV 妗嗘灦鍒拌揪 HTE 瀛愮郴缁熴€備綅浜?`Documentation/devicetree/bindings/timestamp` 鐨?hte 璁惧鏍戠粦瀹氭彁渚涗簡涓€涓秷璐硅€呭浣曡姹備竴鏉?GPIO 绾跨殑绀轰緥銆?
鍙傝 gpiod_enable_hw_timestamp_ns() 鍜?gpiod_disable_hw_timestamp_ns()銆?
瀵逛簬鐢ㄦ埛鎬佹秷璐硅€咃紝蹇呴』鍦?IOCTL 璋冪敤鏈熼棿鎸囧畾 GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE 鏍囧織銆傚弬鑰?`tools/gpio/gpio-event-mon.c`锛屽畠浼氫互绾崇涓哄崟浣嶈繑鍥炴椂闂存埑銆?
### LIC锛圠egacy Interrupt Controller锛屼紶缁熶腑鏂帶鍒跺櫒锛塈RQ GTE


璇?GTE 瀹炰緥瀵?LIC IRQ 绾胯繘琛屽疄鏃舵椂闂存埑鏍囪銆備綅浜?`Documentation/devicetree/bindings/timestamp` 鐨?hte 璁惧鏍戠粦瀹氭彁渚涗簡涓€涓秷璐硅€呭浣曡姹備竴鏉?IRQ 绾跨殑绀轰緥銆傜敱浜庡畠涓?IRQ GTE 鎻愪緵鑰呮槸鈥斺€斿搴旂殑鏄犲皠鍏崇郴锛屾秷璐硅€呭彧闇€鐩存帴鎸囧畾鍏舵劅鍏磋叮鐨?IRQ 鍙峰嵆鍙€侶TE 妗嗘灦鐩墠涓嶆敮鎸佽 GTE 瀹炰緥鐨勭敤鎴锋€佹秷璐硅€呫€?
涓や釜 IRQ 鍜?GPIO GTE 瀹炰緥鐨勬彁渚涜€呮簮浠ｇ爜浣嶄簬 `drivers/hte/hte-tegra194.c`銆傛祴璇曢┍鍔?`drivers/hte/hte-tegra194-test.c` 婕旂ず浜?IRQ 鍜?GPIO GTE 鐨?HTE API 鐢ㄦ硶銆?