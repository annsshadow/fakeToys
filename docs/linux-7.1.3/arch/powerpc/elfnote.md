## ELF Note 鐨?PowerPC 鍛藉悕绌洪棿


鍐呮牳浜岃繘鍒舵枃浠朵腑 ELF Note 鐨?PowerPC 鍛藉悕绌洪棿鐢ㄤ簬瀛樺偍鍙緵寮曞鍔犺浇绋嬪簭锛坆ootloader锛夋垨鐢ㄦ埛鎬佷娇鐢ㄧ殑鍔熻兘涓庝俊鎭€?
### 绫诲瀷涓庢弿杩扮


涓?"PowerPC" 鍛藉悕绌洪棿涓€璧蜂娇鐢ㄧ殑绫诲瀷瀹氫箟鍦?[#f1]_ 涓€?
 1) PPC_ELFNOTE_CAPABILITIES

瀹氫箟鍐呮牳鏀寔/鎵€闇€鐨勫姛鑳姐€傝绫诲瀷浣跨敤浣嶅浘锛坆itmap锛変綔涓?"descriptor" 瀛楁銆傛瘡涓€浣嶅涓嬫墍杩帮細

- 鏀寔 Ultravisor 鐨勪綅锛堜粎 PowerNV锛夈€?

	#define PPCCAP_ULTRAVISOR_BIT (1 << 0)

琛ㄧず powerpc 鍐呮牳浜岃繘鍒剁煡閬撳浣曞湪鍚敤浜?ultravisor 鐨勭郴缁熶腑杩愯銆?
鍦ㄥ惎鐢ㄤ簡 ultravisor 鐨勭郴缁熶腑锛岄儴鍒嗘満鍣ㄨ祫婧愮幇鍦ㄧ敱 ultravisor 鎺у埗銆傚鏋滃唴鏍镐笉鏀寔 ultravisor锛屼絾鏈€缁堝湪甯︽湁 ultravisor 鐨勬満鍣ㄤ笂杩愯锛屽唴鏍稿湪灏濊瘯璁块棶 ultravisor 璧勬簮鏃跺彲鑳戒細宕╂簝銆備緥濡傦紝瀹冨彲鑳藉湪鏃╂湡鍚姩闃舵灏濊瘯璁剧疆鍒嗗尯琛ㄩ」 0 鏃跺穿婧冦€?
鍦ㄥ惎鐢ㄤ簡 ultravisor 鐨勭郴缁熶腑锛屽鏋?PowerPC ultravisor 鑳藉姏涓嶅瓨鍦ㄦ垨鏈缃€滄敮鎸?Ultravisor鈥濅綅锛屽紩瀵煎姞杞界▼搴忓彲浠ヨ鍛婄敤鎴锋垨闃绘鍐呮牳杩愯銆?
### 鍙傝€冭祫鏂?