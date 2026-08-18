## TI EMIF SDRAM 鎺у埗鍣ㄩ┍鍔?

## 浣滆€?
Aneesh V <aneesh@ti.com>

## 浣嶇疆

driver/memory/emif.c

## 鏀寔鐨?SoC锛?
TI OMAP44xx
TI OMAP54xx

## Menuconfig 閫夐」锛?
Device Drivers
	Memory devices
		Texas Instruments EMIF driver

## 鎻忚堪

璇ラ┍鍔ㄧ敤浜?Texas Instruments SoC 涓彲鐢ㄧ殑 EMIF 妯″潡銆侲MIF 鏄竴涓?SDRAM
鎺у埗鍣紝鏍规嵁鍏剁増鏈敮鎸?DDR2銆丏DR3 鍜?LPDDR2 SDRAM 鍗忚涓殑涓€涓垨澶氫釜銆傜洰鍓?璇ラ┍鍔ㄤ粎澶勭悊 LPDDR2 瀛樺偍鍣ㄣ€傞┍鍔ㄧ殑鍔熻兘鍖呮嫭鍦ㄩ鐜囥€佺數鍘嬪拰娓╁害鍙樺寲鏈熼棿閲嶆柊
閰嶇疆 AC 鏃跺簭鍙傛暟鍙婂叾浠栬缃€?
## 骞冲彴鏁版嵁锛堣 include/linux/platform_data/emif_plat.h锛?
DDR 璁惧缁嗚妭浠ュ強鍏朵粬渚濊禆鏉垮拰渚濊禆 SoC 鐨勪俊鎭彲浠ラ€氳繃骞冲彴鏁版嵁
锛坰truct emif_platform_data锛変紶閫掋€?
- DDR 璁惧缁嗚妭锛?struct ddr_device_info'
- 璁惧 AC 鏃跺簭锛?struct lpddr2_timings' 鍜?'struct lpddr2_min_tck'
- 鑷畾涔夐厤缃細閫氳繃 'struct emif_custom_configs' 鐨勫彲瀹氬埗绛栫暐閫夐」
- IP 鐗堟湰
- PHY 绫诲瀷

## 涓庡閮ㄤ笘鐣岀殑鎺ュ彛

EMIF 椹卞姩涓哄奖鍝?EMIF 鐨勭數鍘嬪拰棰戠巼鍙樺寲娉ㄥ唽閫氱煡鍣紝骞跺湪瀹冧滑琚皟鐢ㄦ椂閲囧彇閫傚綋鐨?鎿嶄綔銆?
- freq_pre_notify_handling()
- freq_post_notify_handling()
- volt_notify_handling()

## Debugfs

璇ラ┍鍔ㄤ负姣忎釜璁惧鍒涘缓涓や釜 debugfs 鏉＄洰銆?
- regcache_dump锛氬埌鐩墠涓烘鎵€鏈変娇鐢ㄨ繃鐨勯鐜囪绠楀苟淇濆瓨鐨勫瘎瀛樺櫒鍊艰浆鍌ㄣ€?- mr4锛歀PDDR2 璁惧涓?MR4 瀵勫瓨鍣ㄧ殑鏈€鍚庤疆璇㈠€笺€侻R4 鎸囩ず璁惧褰撳墠鐨勬俯搴︾瓑绾с€?