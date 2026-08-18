## Dell 杩滅▼ BIOS 鏇存柊椹卞姩锛坉ell_rbu锛?

## 鐩殑


鏈枃妗ｆ紨绀哄浣曚娇鐢?Dell 杩滅▼ BIOS 鏇存柊椹卞姩鍦?Dell 鏈嶅姟鍣ㄥ拰鍙板紡鏈轰笂鏇存柊
BIOS 闀滃儚銆?
## 鑼冨洿


鏈枃妗ｄ粎璁ㄨ rbu 椹卞姩鐨勫姛鑳姐€傚畠涓嶆兜鐩栧簲鐢ㄧ▼搴忔墍闇€鐨勩€佺敤浜庝娇 BIOS 鑳藉鐢?涓嬭浇鍒板唴瀛樹腑鐨勯暅鍍忚嚜鎴戞洿鏂扮殑鏀寔銆?
## 姒傝堪


璇ラ┍鍔ㄤ笌 Dell OpenManage 鎴?Dell 鏇存柊鍖咃紙DUP锛夐厤鍚堬紝鐢ㄤ簬鏇存柊 Dell 鏈嶅姟鍣?锛堣嚜 1999 骞磋捣閿€鍞殑鏈嶅姟鍣級銆佸彴寮忔満鍜岀瑪璁版湰锛堣嚜 2005 骞磋捣閿€鍞殑浜у搧锛変笂鐨?BIOS銆?
璇疯闂?http://support.dell.com 娉ㄥ唽锛屼綘鍙互鍦ㄩ偅閲屾壘鍒板叧浜?OpenManage 鍜?Dell 鏇存柊鍖咃紙DUP锛夌殑淇℃伅銆?
涔熷彲浠ヤ娇鐢?Libsmbios 鏉ユ洿鏂?Dell 绯荤粺涓婄殑 BIOS锛岃鎯呰璁块棶
https://linux.dell.com/libsmbios/銆?
Dell_RBU 椹卞姩鏀寔浣跨敤鏁翠綋寮忥紙monolithic锛夐暅鍍忓拰鏁版嵁鍖呭紡锛坧acketized锛夐暅鍍?涓ょ鏂规硶鏉ユ洿鏂?BIOS銆傚浜庢暣浣撳紡锛岄┍鍔ㄥ垎閰嶄竴鍧楄繛缁殑鐗╃悊椤碉紝鐢ㄤ簬瀛樻斁 BIOS
闀滃儚銆傚浜庢暟鎹寘寮忥紝浣跨敤璇ラ┍鍔ㄧ殑搴旂敤绋嬪簭灏嗛暅鍍忔媶鍒嗘垚鍥哄畾澶у皬鐨勬暟鎹寘锛岀敱
椹卞姩鎶婃瘡涓暟鎹寘鏀惧埌杩炵画鐨勭墿鐞嗗唴瀛樹腑銆傞┍鍔ㄨ繕缁存姢涓€涓暟鎹寘閾捐〃浠ヤ究鍥炶銆?
濡傛灉 dell_rbu 椹卞姩琚嵏杞斤紝鎵€鏈夊凡鍒嗛厤鐨勫唴瀛橀兘浼氳閲婃斁銆?
rbu 椹卞姩闇€瑕佹湁涓€涓簲鐢ㄧ▼搴忥紙濡傚墠鎵€杩帮級鏉ラ€氱煡 BIOS 鍦ㄤ笅娆＄郴缁熼噸鍚椂鍚敤鏇存柊銆?
鐢ㄦ埛涓嶅簲鍦ㄤ笅杞?BIOS 闀滃儚鎴栨洿鏂颁箣鍚庡嵏杞?rbu 椹卞姩銆?
```

	/sys/class/firmware/dell_rbu/loading
	/sys/class/firmware/dell_rbu/data
	/sys/devices/platform/dell_rbu/image_type
	/sys/devices/platform/dell_rbu/data
	/sys/devices/platform/dell_rbu/packet_size

```
璇ラ┍鍔ㄦ敮鎸佷袱绉嶆洿鏂版満鍒讹細鏁翠綋寮忓拰鏁版嵁鍖呭紡銆傝繖浜涙洿鏂版満鍒跺彇鍐充簬绯荤粺褰撳墠杩愯鐨?BIOS銆傚ぇ澶氭暟 Dell 绯荤粺鏀寔鏁翠綋寮忔洿鏂帮紝鍗虫妸 BIOS 闀滃儚澶嶅埗鍒颁竴鍧楄繛缁殑鐗╃悊
鍐呭瓨涓€?
鍦ㄦ暟鎹寘鏈哄埗涓嬶紝鍗曞潡鍐呭瓨鍙互琚媶鍒嗘垚鏇村皬鐨勮繛缁唴瀛樺潡锛孊IOS 闀滃儚琚垎鏁ｅ埌
杩欎簺鏁版嵁鍖呬腑銆?
榛樿鎯呭喌涓嬶紝椹卞姩浣跨敤鏁翠綋寮忓唴瀛樹綔涓烘洿鏂扮被鍨嬨€傝繖鍙互閫氳繃鍦ㄩ┍鍔ㄥ姞杞芥椂鎸囧畾
鍔犺浇鍙傛暟鏉ユ敼涓烘暟鎹寘寮忥細
```

	echo packet > /sys/devices/platform/dell_rbu/image_type

```
鍦ㄦ暟鎹寘鏇存柊妯″紡涓嬶紝蹇呴』鍏堢粰鍑烘暟鎹寘澶у皬锛岀劧鍚庢墠鑳藉彂閫佷换浣曟暟鎹寘锛?```

	echo XXXX > /sys/devices/platform/dell_rbu/packet_size

```
鍦ㄦ暟鎹寘鏇存柊鏈哄埗涓紝鐢ㄦ埛闇€瑕佸垱寤轰竴涓柊鏂囦欢锛屽叾涓暟鎹寘鏁版嵁棣栧熬鐩告帴渚濇
鎺掑垪銆傚仛娉曞涓嬶細鐢ㄦ埛鍒涘缓鏁版嵁鍖呭ご锛屽彇鍑轰竴鍧?BIOS 闀滃儚鏀惧湪鍖呭ご鏃佽竟锛涙鏃讹紝
鍖呭ご + BIOS 闀滃儚鍧楀姞鍦ㄤ竴璧峰簲褰撲笌鎸囧畾鐨?packet_size 鐩哥瓑銆傝繖鏍峰氨鏋勬垚浜嗕竴涓?鏁版嵁鍖咃紝鐢ㄦ埛闇€瑕佷粠鏁翠釜 BIOS 闀滃儚鏂囦欢涓垱寤烘洿澶氳繖鏍风殑鏁版嵁鍖咃紝鐒跺悗灏嗚繖浜?鏁版嵁鍖呴灏剧浉鎺ユ帓鎴愪竴涓崟鐙殑鏂囦欢銆?
闅忓悗灏嗚鏂囦欢澶嶅埗鍒?/sys/class/firmware/dell_rbu/data銆備竴鏃﹁鏂囦欢鍒拌揪椹卞姩锛?椹卞姩灏变粠鏂囦欢涓彁鍙?packet_size 澶у皬鐨勬暟鎹紝骞跺皢鍏跺垎甯冨埌杩炵画銆佸ぇ灏忎负
packet_size 鐨勭墿鐞嗗唴瀛樼┖闂翠腑銆?
杩欑鏂规硶纭繚鎵€鏈夌殑鏁版嵁鍖呭湪涓€娆℃搷浣滀腑閮借兘閫佽揪椹卞姩銆?
鍦ㄦ暣浣撳紡鏇存柊涓紝鐢ㄦ埛鍙渶鐩存帴鑾峰彇 BIOS 闀滃儚锛?hdr 鏂囦欢锛夊苟鎸夊師鏍峰鍒跺埌 data
鏂囦欢锛屼笉瀵?BIOS 闀滃儚鏈韩鍋氫换浣曟敼鍔ㄣ€?
鎸変互涓嬫楠や笅杞?BIOS 闀滃儚锛?
1) echo 1 > /sys/class/firmware/dell_rbu/loading
2) cp bios_image.hdr /sys/class/firmware/dell_rbu/data
3) echo 0 > /sys/class/firmware/dell_rbu/loading

/sys/class/firmware/dell_rbu/ 涓嬬殑鏉＄洰浼氫竴鐩翠繚鐣欙紝鐩村埌鎵ц浠ヤ笅鎿嶄綔锛?
```

	echo -1 > /sys/class/firmware/dell_rbu/loading

```
鍦ㄥ畬鎴愭姝ラ涔嬪墠锛岄┍鍔ㄦ棤娉曡鍗歌浇銆?
姝ゅ锛屽悜 image_type 鍐欏叆 mono銆乸acket 鎴?init 閮戒細閲婃斁椹卞姩宸插垎閰嶇殑鍐呭瓨銆?
濡傛灉鐢ㄦ埛鎰忓鍦版墽琛屼簡涓婇潰鐨勭 1 姝ュ拰绗?3 姝ヨ€屾病鏈夋墽琛岀 2 姝ワ紝灏嗗鑷?/sys/class/firmware/dell_rbu/ 涓嬬殑鏉＄洰娑堝け銆?
```

	echo init > /sys/devices/platform/dell_rbu/image_type

```

姝ゅ锛岄┍鍔ㄨ繕鎻愪緵 /sys/devices/platform/dell_rbu/data 鍙鏂囦欢锛岀敤浜庡洖璇?宸蹭笅杞界殑闀滃儚銆?

   鏇存柊瀹?BIOS 闀滃儚鍚庯紝鐢ㄦ埛鎬佸簲鐢ㄧ▼搴忛渶瑕佹墽琛屽悜 BIOS 鍙戦€?BIOS 鏇存柊璇锋眰鐨?   浠ｇ爜銆傝繖鏍峰湪涓嬫閲嶅惎鏃讹紝BIOS 灏辩煡閬撴湁鏂颁笅杞界殑闀滃儚骞惰嚜鎴戞洿鏂般€傚彟澶栵紝濡傛灉
   瑕佹洿鏂伴暅鍍忥紝涓嶈鍗歌浇 rbu 椹卞姩銆?