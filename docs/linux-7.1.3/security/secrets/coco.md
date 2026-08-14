## 鏈哄瘑璁＄畻瀵嗛挜


鏈枃妗ｆ弿杩版満瀵嗚绠楋紙Confidential Computing锛夌殑瀵嗛挜娉ㄥ叆鏄浣曚粠鍥轰欢浼犻€掑埌鎿嶄綔绯荤粺锛岀粡鐢?EFI 椹卞姩鍜?efi_secret 鍐呮牳妯″潡澶勭悊鐨勩€?

## 绠€浠?


鏈哄瘑璁＄畻锛坈oco锛夌‖浠讹紝渚嬪 AMD SEV锛堝畨鍏ㄥ姞瀵嗚櫄鎷熷寲锛孲ecure Encrypted Virtualization锛夛紝鍏佽瀹㈡埛鏈烘墍鏈夎€呭皢瀵嗛挜娉ㄥ叆鍒拌櫄鎷熸満鐨勫唴瀛樹腑锛岃€屽涓绘満/hypervisor 鏃犳硶璇诲彇瀹冧滑銆傚湪 SEV 涓紝瀵嗛挜娉ㄥ叆鍦ㄨ櫄鎷熸満鍚姩杩囩▼鐨勬棭鏈熴€佸鎴锋満寮€濮嬭繍琛屼箣鍓嶆墽琛屻€?

efi_secret 鍐呮牳妯″潡鍏佽鐢ㄦ埛绌洪棿搴旂敤绋嬪簭閫氳繃 securityfs 璁块棶杩欎簺瀵嗛挜銆?

## 瀵嗛挜鏁版嵁娴?


瀹㈡埛鏈哄浐浠跺彲浠ラ鐣欎竴涓寚瀹氱殑鍐呭瓨鍖哄煙鐢ㄤ簬瀵嗛挜娉ㄥ叆锛屽苟鍦?EFI 閰嶇疆琛ㄤ腑浠ヤ竴涓?`LINUX_EFI_COCO_SECRET_AREA_GUID` 鏉＄洰锛坄adf956ad-e98c-484c-ae11-b51c7d336447`锛夊彂甯冨叾浣嶇疆锛堝熀鍧€ GPA 涓庨暱搴︼級銆傝鍐呭瓨鍖哄煙搴旂敱鍥轰欢鏍囪涓?`EFI_RESERVED_TYPE`锛屽洜姝ゅ唴鏍镐笉搴斿皢鍏剁敤浜庤嚜韬洰鐨勩€?

鍦ㄨ櫄鎷熸満鍚姩鏈熼棿锛岃櫄鎷熸満绠＄悊鍣ㄥ彲浠ュ悜璇ュ尯鍩熸敞鍏ヤ竴涓瘑閽ャ€傚湪 AMD SEV 鍜?SEV-ES 涓紝杩欐槸浣跨敤 `KVM_SEV_LAUNCH_SECRET` 鍛戒护鎵ц鐨勶紙鍙傝 [sev]_锛夈€傛敞鍏ョ殑瀹㈡埛鏈烘墍鏈夎€呭瘑閽ユ暟鎹殑缁撴瀯搴斿綋鏄竴涓敱 GUID 寮曞鐨勫瘑閽ュ€艰〃锛涘叾浜岃繘鍒舵牸寮忓湪 `drivers/virt/coco/efi_secret/efi_secret.c` 鐨勨€淓FI 瀵嗛挜鍖哄煙鐨勭粨鏋勨€濅竴鑺備腑鎻忚堪銆?

鍦ㄥ唴鏍稿惎鍔ㄦ椂锛屽唴鏍哥殑 EFI 椹卞姩灏嗗瘑閽ュ尯鍩熺殑浣嶇疆锛堝彇鑷?EFI 閰嶇疆琛級淇濆瓨鍦?`efi.coco_secret` 瀛楁涓€傞殢鍚庡畠浼氭鏌ュ瘑閽ュ尯鍩熸槸鍚﹀凡琚～鍏咃細瀹冩槧灏勮鍖哄煙骞舵鏌ュ叾鍐呭鏄惁浠?`EFI_SECRET_TABLE_HEADER_GUID`锛坄1e74f542-71dd-4d66-963e-ef4287ff173b`锛夊紑澶淬€傚鏋滃瘑閽ュ尯鍩熷凡琚～鍏咃紝EFI 椹卞姩浼氳嚜鍔ㄥ姞杞?efi_secret 鍐呮牳妯″潡锛岃妯″潡閫氳繃 securityfs 灏嗗瘑閽ユ毚闇茬粰鐢ㄦ埛绌洪棿搴旂敤绋嬪簭銆俥fi_secret 鏂囦欢绯荤粺鎺ュ彛鐨勮缁嗕俊鎭 [secrets-coco-abi]_銆?

## 搴旂敤浣跨敤绀轰緥


鑰冭檻涓€涓湪鍔犲瘑鏂囦欢涓婃墽琛岃绠楃殑瀹㈡埛鏈恒€傚鎴锋満鎵€鏈夎€呬娇鐢ㄥ瘑閽ユ敞鍏ユ満鍒舵彁渚涜В瀵嗗瘑閽ワ紙= secret锛夈€傚鎴锋満搴旂敤绋嬪簭浠?efi_secret 鏂囦欢绯荤粺璇诲彇璇ュ瘑閽ワ紝杩涜€屽皢鏂囦欢瑙ｅ瘑鍒板唴瀛樹腑锛岀劧鍚庡鍐呭鎵ц鎵€闇€鐨勮绠椼€?

鍦ㄦ绀轰緥涓紝瀹夸富鏈烘棤娉曚粠纾佺洏鏄犲儚涓鍙栨枃浠讹紝鍥犱负瀹冧滑宸茶鍔犲瘑銆傚涓绘満鏃犳硶璇诲彇瑙ｅ瘑瀵嗛挜锛屽洜涓哄畠閫氳繃瀵嗛挜娉ㄥ叆鏈哄埗锛? 瀹夊叏閫氶亾锛変紶閫掋€傚涓绘満鏃犳硶浠庡唴瀛樹腑璇诲彇瑙ｅ瘑鍚庣殑鍐呭锛屽洜涓哄畠鏄竴涓満瀵嗭紙鍐呭瓨鍔犲瘑锛夌殑瀹㈡埛鏈恒€?

浠ヤ笅鏄竴涓湪瀹㈡埛鏈轰腑浣跨敤 efi_secret 妯″潡鐨勭畝鍗曠ず渚?
```

	# ls -la /sys/kernel/security/secrets/coco
	total 0
	drwxr-xr-x 2 root root 0 Jun 28 11:54 .
	drwxr-xr-x 3 root root 0 Jun 28 11:54 ..
	-r--r----- 1 root root 0 Jun 28 11:54 736870e5-84f0-4973-92ec-06879ce3da0b
	-r--r----- 1 root root 0 Jun 28 11:54 83c83f7f-1356-4975-8b7e-d3a0b54312c6
	-r--r----- 1 root root 0 Jun 28 11:54 9553f55d-3da2-43ee-ab5d-ff17f78864d2
	-r--r----- 1 root root 0 Jun 28 11:54 e6f5a162-d67f-4750-a67c-5d065f2a9910

	# hd /sys/kernel/security/secrets/coco/e6f5a162-d67f-4750-a67c-5d065f2a9910
	00000000  74 68 65 73 65 2d 61 72  65 2d 74 68 65 2d 6b 61  |these-are-the-ka|
	00000010  74 61 2d 73 65 63 72 65  74 73 00 01 02 03 04 05  |ta-secrets......|
	00000020  06 07                                             |..|
	00000022

	# rm /sys/kernel/security/secrets/coco/e6f5a162-d67f-4750-a67c-5d065f2a9910

	# ls -la /sys/kernel/security/secrets/coco
	total 0
	drwxr-xr-x 2 root root 0 Jun 28 11:55 .
	drwxr-xr-x 3 root root 0 Jun 28 11:54 ..
	-r--r----- 1 root root 0 Jun 28 11:54 736870e5-84f0-4973-92ec-06879ce3da0b
	-r--r----- 1 root root 0 Jun 28 11:54 83c83f7f-1356-4975-8b7e-d3a0b54312c6
	-r--r----- 1 root root 0 Jun 28 11:54 9553f55d-3da2-43ee-ab5d-ff17f78864d2


```
## 鍙傝€冭祫鏂?


鏈夊叧 SEV `LAUNCH_SECRET` 鎿嶄綔鐨勬洿澶氫俊鎭紝璇峰弬瑙?[sev-api-spec]_銆?
