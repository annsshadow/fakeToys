## SPEAr ARM Linux 姒傝堪


### 绠€浠?

  SPEAr锛圫tructured Processor Enhanced Architecture锛岀粨鏋勫寲澶勭悊鍣ㄥ寮烘灦鏋勶級銆?  weblink : http://www.st.com/spear

  ST Microelectronics 鐨?SPEAr 绯诲垪 ARM9/CortexA9 鐗囦笂绯荤粺锛圫ystem-on-Chip锛塁PU 鐢?ARM Linux 鐨?'spear' 骞冲彴鏀寔銆傚綋鍓嶆敮鎸?SPEAr1310銆丼PEAr1340銆丼PEAr300銆丼PEAr310銆丼PEAr320 鍜?SPEAr600 杩欎簺 SoC銆?
  SPEAr 涓殑灞傜骇缁撴瀯濡備笅锛?
  SPEAr锛堝钩鍙帮級

 - SPEAr3XX锛?XX SOC 绯诲垪锛屽熀浜?ARM9锛?  - SPEAr300锛圫OC锛?   - SPEAr300 璇勪及鏉?  - SPEAr310锛圫OC锛?   - SPEAr310 璇勪及鏉?  - SPEAr320锛圫OC锛?   - SPEAr320 璇勪及鏉? - SPEAr6XX锛?XX SOC 绯诲垪锛屽熀浜?ARM9锛?  - SPEAr600锛圫OC锛?   - SPEAr600 璇勪及鏉? - SPEAr13XX锛?3XX SOC 绯诲垪锛屽熀浜?ARM CORTEXA9锛?  - SPEAr1310锛圫OC锛?   - SPEAr1310 璇勪及鏉?  - SPEAr1340锛圫OC锛?   - SPEAr1340 璇勪及鏉?
### 閰嶇疆


  涓烘瘡鍙版満鍣ㄦ彁渚涗簡涓€涓€氱敤閰嶇疆锛屽彲浠ョ敤浣?```

	make spear13xx_defconfig
	make spear3xx_defconfig
	make spear6xx_defconfig

```
### 甯冨眬


  澶氫釜鏈哄櫒绯诲垪锛圫PEAr3xx銆丼PEAr6xx 鍜?SPEAr13xx锛夌殑鍏叡鏂囦欢浣嶄簬骞冲彴浠ｇ爜涓紝鍖呭惈鍦?arch/arm/plat-spear 涓紝澶存枃浠跺湪 plat/ 涓€?
  姣忎釜鏈哄櫒绯诲垪閮芥湁涓€涓互 arch/arm/mach-spear 鍔犵郴鍒楀悕鍛藉悕鐨勭洰褰曘€備緥濡?mach-spear3xx銆乵ach-spear6xx 鍜?mach-spear13xx銆?
  spear3xx 绯诲垪鏈哄櫒鐨勫叕鍏辨枃浠舵槸 mach-spear3xx/spear3xx.c锛宻pear6xx 鐨勬槸 mach-spear6xx/spear6xx.c锛宻pear13xx 绯诲垪鐨勬槸 mach-spear13xx/spear13xx.c銆俶ach-spear* 杩樺寘鍚?soc/鏈哄櫒鐗瑰畾鐨勬枃浠讹紝濡?spear1310.c銆乻pear1340.c銆乻pear300.c銆乻pear310.c銆乻pear320.c 鍜?spear600.c銆俶ach-spear* 涓嶅寘鍚澘绾х壒瀹氱殑鏂囦欢锛屽洜涓哄畠浠畬鍏ㄦ敮鎸?Flattened Device Tree銆?

### 鏂囨。浣滆€?

  Viresh Kumar <vireshk@kernel.org>, (c) 2010-2012 ST Microelectronics
