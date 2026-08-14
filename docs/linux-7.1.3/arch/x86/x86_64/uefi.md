
## 鍏充簬 [U]EFI x86_64 鏀寔鐨勯€氱敤璇存槑


鏈枃妗ｄ腑 EFI 涓?UEFI 杩欎袱涓湳璇彲浜掓崲浣跨敤銆?
灏界鏋勫缓鍐呮牳骞朵笉闇€瑕佷娇鐢ㄤ笅鍒楀伐鍏凤紝浣嗕笅鏂囧垪鍑轰簡 x86_64 骞冲彴涓?甯︽湁 EFI 鍥轰欢涓庤鑼冪殑寮曞鍔犺浇绋嬪簭鏀寔鍙婄浉鍏冲伐鍏枫€?
1. UEFI 瑙勮寖锛? http://www.uefi.org

2. 鍦?UEFI x86_64 骞冲彴涓婂紩瀵?Linux 鍐呮牳锛屾棦鍙互浣跨敤
   <Documentation/admin-guide/efi-stub.rst>锛屼篃鍙互浣跨敤鐙珛鐨?   寮曞鍔犺浇绋嬪簭銆?
3. 甯︽湁 EFI/UEFI 鍥轰欢鐨?x86_64 骞冲彴銆?
### 鏈哄埗


璇峰弬闃?<Documentation/admin-guide/efi-stub.rst> 浜嗚В濡備綍浣跨敤 EFI stub銆?
浠ヤ笅鏄湪 x86_64 骞冲彴涓婇€氱敤鐨?EFI 璁剧疆鎸囧崡锛屾棤璁轰綘浣跨敤鐨勬槸
EFI stub 杩樻槸鐙珛鐨勫紩瀵煎姞杞界▼搴忋€?
```

	CONFIG_FB_EFI=y
	CONFIG_FRAMEBUFFER_CONSOLE=y

  濡傛灉闇€瑕?EFI 杩愯鏃舵湇鍔★紝搴旈€夋嫨浠ヤ笅閰嶇疆锛?
	CONFIG_EFI=y
	CONFIG_EFIVAR_FS=y or m		# 鍙€?
```
- 鍦ㄧ鐩樹笂鍒涘缓涓€涓甫鏈?EFI System 鏍囧織鐨?VFAT 鍒嗗尯
    浣犲彲浠ヤ娇鐢?fdisk 閫氳繃浠ヤ笅鍛戒护瀹屾垚锛?
        1. g - 鍒濆鍖栦竴涓?GPT 鍒嗗尯琛?        2. n - 鍒涘缓涓€涓柊鍒嗗尯
        3. t - 灏嗗垎鍖虹被鍨嬫敼涓?鈥淓FI System鈥濓紙缂栧彿 1锛?        4. w - 鍐欏叆骞朵繚瀛樻洿鏀?
```

        mkfs.fat /dev/<your-partition>

```
- 灏嗗紩瀵兼枃浠跺鍒跺埌 VFAT 鍒嗗尯锛?    濡傛灉浣犱娇鐢?EFI stub 鏂瑰紡锛屽唴鏍稿悓鏃朵篃鍏呭綋 EFI 鍙墽琛屾枃浠躲€?
    浣犲彧闇€灏?bzImage 澶嶅埗鍒板垎鍖轰笂鐨?EFI/boot/bootx64.efi 璺緞锛?    瀹冧究浼氳嚜鍔ㄨ寮曞锛涘叧浜庝紶閫掑唴鏍稿弬鏁颁笌 initramfs 鐨勬洿澶氳鏄庯紝
    璇峰弬闃?<Documentation/admin-guide/efi-stub.rst> 椤甸潰銆?
    濡傛灉浣犱娇鐢ㄨ嚜瀹氫箟寮曞鍔犺浇绋嬪簭锛岃鍙傝€冪浉鍏虫枃妗ｄ互鑾峰緱姝ら儴鍒嗙殑甯姪銆?
- 濡傛灉閮ㄥ垎鎴栧叏閮?EFI 杩愯鏃舵湇鍔℃棤娉曞伐浣滐紝浣犲彲浠ュ皾璇曚娇鐢ㄤ互涓?    鍐呮牳鍛戒护琛屽弬鏁版潵鍏抽棴閮ㄥ垎鎴栧叏閮?EFI 杩愯鏃舵湇鍔°€?
	noefi
		鍏抽棴鎵€鏈?EFI 杩愯鏃舵湇鍔?	reboot_type=k
		鍏抽棴 EFI 閲嶅惎杩愯鏃舵湇鍔?
- 濡傛灉 EFI 鍐呭瓨鏄犲皠涓寘鍚?E820 鏄犲皠閲屾病鏈夌殑棰濆鏉＄洰锛屼綘鍙互
    浣跨敤浠ヤ笅鍐呮牳鍛戒护琛屽弬鏁帮紝灏嗚繖浜涙潯鐩撼鍏ュ唴鏍稿彲鐢ㄧ墿鐞?RAM 鐨?    鍐呭瓨鏄犲皠涓€?
	add_efi_memmap
		绾冲叆鍙敤鐗╃悊 RAM 鐨?EFI 鍐呭瓨鏄犲皠
