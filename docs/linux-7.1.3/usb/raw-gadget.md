## USB Raw Gadget锛堝師濮嬪皬宸ュ叿锛?

USB Raw Gadget 鏄竴涓皬宸ュ叿锛坓adget锛夐┍鍔紝瀹冭鐢ㄦ埛鍙互绌洪棿瀵?gadget 鐨勯€氫俊杩囩▼杩涜搴曞眰鎺у埗銆?
涓庝换浣曞叾浠?gadget 椹卞姩涓€鏍凤紝Raw Gadget 閫氳繃 USB gadget API 瀹炵幇 USB 璁惧銆備笌澶у鏁?gadget 椹卞姩涓嶅悓锛孯aw Gadget 鑷韩涓嶅疄鐜颁换浣曞叿浣撶殑 USB 鍔熻兘锛岃€屾槸闇€瑕佺敤鎴风┖闂存潵瀹屾垚銆?
Raw Gadget 鐩墠涓ユ牸鏉ヨ鏄竴涓皟璇曠壒鎬э紝涓嶅簲鍦ㄧ敓浜х幆澧冧腑浣跨敤銆傝鏀圭敤 GadgetFS銆?
閫氳繃 CONFIG_USB_RAW_GADGET 鍚敤銆?
#### 涓?GadgetFS 鐨勫姣?

Raw Gadget 涓?GadgetFS 绫讳技锛屼絾涓虹敤鎴风┖闂存彁渚涗簡瀵?USB gadget 灞傛洿鐩存帴鐨勮闂€傚叧閿尯鍒湪浜庯細

1. Raw Gadget 灏嗘瘡涓?USB 璇锋眰浼犻€掔粰鐢ㄦ埛绌洪棿浠ヨ幏鍙栧搷搴旓紝鑰?GadgetFS 鏍规嵁鎵€鎻愪緵鐨勬弿杩扮鍦ㄥ唴閮ㄥ搷搴旀煇浜?USB 璇锋眰銆傛敞鎰忥紝UDC 椹卞姩鍙兘浼氳嚜琛屽搷搴旀煇浜涜姹傦紝涓旀案杩滀笉浼氬皢瀹冧滑杞彂鍒?gadget 灞傘€?
2. Raw Gadget 鍏佽鎻愪緵浠绘剰鏁版嵁浣滀负瀵?USB 璇锋眰鐨勫搷搴旓紝鑰?GadgetFS 瀵规墍鎻愪緵鐨?USB 鎻忚堪绗︽墽琛屽仴鍏ㄦ€ф鏌ャ€傝繖浣垮緱 Raw Gadget 閫傚悎閫氳繃鎻愪緵鐣稿舰鏁版嵁浣滀负瀵?USB 璇锋眰鐨勫搷搴旀潵杩涜妯＄硦娴嬭瘯锛坒uzzing锛夈€?
3. Raw Gadget 鎻愪緵浜嗕竴绉嶉€夋嫨瑕佺粦瀹氱殑 UDC 璁惧/椹卞姩鐨勬柟娉曪紝鑰?GadgetFS 褰撳墠缁戝畾鍒扮涓€涓彲鐢ㄧ殑 UDC銆傝繖鍏佽澶氫釜 Raw Gadget 瀹炰緥缁戝畾鍒颁笉鍚岀殑 UDC銆?
4. Raw Gadget 鏄惧紡鏆撮湶鏈夊叧绔偣鍦板潃鍜岃兘鍔涚殑淇℃伅銆傝繖浣垮緱鐢ㄦ埛鍙互缂栧啓涓?UDC 鏃犲叧鐨?gadget銆?
5. Raw Gadget 鍏锋湁鍩轰簬 ioctl 鐨勬帴鍙ｏ紝鑰屼笉鏄熀浜庢枃浠剁郴缁熺殑鎺ュ彛銆?
#### 鐢ㄦ埛绌洪棿鎺ュ彛


鐢ㄦ埛鍙互閫氳繃鎵撳紑 `/dev/raw-gadget` 骞跺彂璧?ioctl 璋冪敤鏉ヤ笌 Raw Gadget 浜や簰锛涜瑙?include/uapi/linux/usb/raw_gadget.h 涓殑娉ㄩ噴銆傚涓?Raw Gadget 瀹炰緥锛堢粦瀹氬埌涓嶅悓鐨?UDC锛夊彲浠ュ悓鏃惰浣跨敤銆?
Raw Gadget 鐨勫吀鍨嬩娇鐢ㄥ満鏅細

1. 閫氳繃鎵撳紑 `/dev/raw-gadget` 鍒涘缓涓€涓?Raw Gadget 瀹炰緥銆?2. 閫氳繃 `USB_RAW_IOCTL_INIT` 鍒濆鍖栬瀹炰緥銆?3. 閫氳繃 `USB_RAW_IOCTL_RUN` 鍚姩璇ュ疄渚嬨€?4. 鍦ㄥ惊鐜腑鍙戣捣 `USB_RAW_IOCTL_EVENT_FETCH` 浠ユ帴鏀舵潵鑷?Raw Gadget 鐨勪簨浠讹紝骞舵牴鎹渶瑕佸疄鐜颁綍绉?USB gadget 鏉ヤ綔鍑哄弽搴斻€?
璇锋敞鎰忥紝鏌愪簺 UDC 椹卞姩涓虹鐐瑰垎閰嶄簡鍥哄畾鍦板潃锛屽洜姝ゆ弿杩扮涓笉鑳戒娇鐢ㄤ换鎰忕鐐瑰湴鍧€銆傚敖绠″姝わ紝Raw Gadget 鎻愪緵浜嗕竴绉嶄笌 UDC 鏃犲叧鐨勭紪鍐?USB gadget 鐨勬柟娉曘€備竴鏃﹂€氳繃 `USB_RAW_IOCTL_EVENT_FETCH` 鏀跺埌 `USB_RAW_EVENT_CONNECT`锛屽氨鍙互浣跨敤 `USB_RAW_IOCTL_EPS_INFO` 鏉ユ煡鏄?UDC 椹卞姩鎵€鎷ユ湁鐨勭鐐逛俊鎭€傚湪姝ゅ熀纭€涓婏紝鐢ㄦ埛绌洪棿蹇呴』涓?gadget 閫夋嫨 UDC 绔偣锛屽苟鍦ㄧ鐐规弿杩扮涓浉搴斿湴鍒嗛厤鍦板潃銆?
Raw Gadget 鐨勪娇鐢ㄧず渚嬪拰娴嬭瘯濂椾欢锛?
https://github.com/xairy/raw-gadget

#### 鍐呴儴缁嗚妭


姣忎釜 Raw Gadget 绔偣鐨勮/鍐?ioctl 閮戒細鎻愪氦涓€涓?USB 璇锋眰骞剁瓑寰呭叾瀹屾垚銆傝繖鏍峰仛鏄晠鎰忕殑锛屼互渚块€氳繃鍗曚釜绯荤粺璋冪敤瀹屾暣澶勭悊鍗曚釜 USB 璇锋眰鏉ヨ緟鍔╄鐩栫巼寮曞鐨勬ā绯婃祴璇曘€傝繖涓€鐗规€у繀椤诲湪瀹炵幇涓繚鐣欍€?
#### 娼滃湪鐨勬湭鏉ユ敼杩?

- 鏀寔 `O_NONBLOCK` I/O銆傝繖灏嗘槸鍙︿竴绉嶆搷浣滄ā寮忥紝Raw Gadget 涓嶄細绛夊緟姣忎釜 USB 璇锋眰瀹屾垚銆?
- 鏀寔 USB 3 鐗规€э紙鍦ㄥ惎鐢ㄧ鐐规椂鎺ュ彈 SS 绔偣浼撮殢鎻忚堪绗︼紱鍏佽涓烘壒閲忎紶杈撴彁渚?`stream_id`锛夈€?
- 鏀寔绛夋椂锛圛SO锛変紶杈撶壒鎬э紙涓哄凡瀹屾垚鐨勮姹傛毚闇?`frame_number`锛夈€?