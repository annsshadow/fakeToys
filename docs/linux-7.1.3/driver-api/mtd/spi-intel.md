## 浣跨敤 spi-intel 鍗囩骇 BIOS


璁稿 Intel CPU锛堝 Baytrail 鍜?Braswell锛夊寘鍚?SPI 涓茶闂瓨涓绘帶鍒跺櫒锛岀敤浜庝繚瀛?BIOS 鍜屽叾浠栧钩鍙扮壒瀹氭暟鎹€傜敱浜?SPI 涓茶闂瓨鐨勫唴瀹瑰浜庢満鍣ㄨ繍琛岃嚦鍏抽噸瑕侊紝瀹冮€氬父鍙楀埌涓嶅悓纭欢淇濇姢鏈哄埗鐨勪繚鎶わ紝浠ラ伩鍏嶆剰澶栵紙鎴栬搫鎰忥級瑕嗙洊鍐呭銆?
骞堕潪鎵€鏈夊埗閫犲晢閮戒繚鎶?SPI 涓茶闂瓨锛屼富瑕佹槸鍥犱负杩欏厑璁哥洿鎺ヤ粠鎿嶄綔绯荤粺鍗囩骇 BIOS 闀滃儚銆?
spi-intel 椹卞姩浣垮緱鍦ㄧ壒瀹氱殑淇濇姢浣嶆湭琚缃苟閿佸畾鐨勬儏鍐典笅锛屽彲浠ヨ鍐?SPI 涓茶闂瓨銆傚鏋滃畠鍙戠幇鍏朵腑浠讳綍涓€浣嶈璁剧疆锛屾暣涓?MTD 璁惧灏嗚璁句负鍙锛屼互闃叉閮ㄥ垎瑕嗙洊銆傞粯璁ゆ儏鍐典笅锛岄┍鍔ㄥ皢 SPI 涓茶闂瓨鍐呭浣滀负鍙鏆撮湶锛屼絾鍙互閫氳繃鍐呮牳鍛戒护琛屼紶閫?鈥渟pi_intel.writeable=1鈥?鏉ユ洿鏀广€?
璇疯浣忥紝瑕嗙洊 SPI 涓茶闂瓨涓婄殑 BIOS 闀滃儚鍙兘浼氫娇鏈哄櫒鏃犳硶鍚姩锛屽苟闇€瑕佸儚 Dediprog 杩欐牱鐨勭壒娈婅澶囨潵鎭㈠銆傚凡缁忚鍛婅繃浣犱簡锛?
浠ヤ笅鏄粠 Linux 鐩存帴鍗囩骇 MinnowBoard MAX BIOS 鐨勬楠ゃ€?
 1) 涓嬭浇骞惰В鍘嬫渶鏂扮殑 Minnowboard MAX BIOS SPI 闀滃儚
    [^1^]銆傛挵鍐欐湰鏂囨椂鏈€鏂伴暅鍍忔槸 v92銆?
 2) 瀹夎 mtd-utils 杞欢鍖?[^2^]銆傛垜浠渶瑕佸畠鏉ユ摝闄?SPI
    涓茶闂瓨銆傚儚 Debian 鍜?Fedora 杩欐牱鐨勫彂琛岀増宸插皢鍏舵墦鍖咃紝鍚嶄负 鈥渕td-utils鈥濄€?
 3) 灏?鈥渟pi_intel.writeable=1鈥?娣诲姞鍒板唴鏍稿懡浠よ骞堕噸鍚?    寮€鍙戞澘锛堜綘涔熷彲浠ラ噸鏂板姞杞介┍鍔紝灏?鈥渨riteable=1鈥?浣滀负妯″潡鍙傛暟浼犻€掔粰 modprobe锛夈€?
 4) 寮€鍙戞澘閲嶆柊鍚姩杩愯鍚庯紝鎵惧埌姝ｇ‘鐨?MTD 鍒嗗尯
```

	# cat /proc/mtd
	dev:    size   erasesize  name
	mtd0: 00800000 00001000 "BIOS"

    鍥犳杩欓噷灏嗘槸 /dev/mtd0锛屼絾鍙兘鏈夋墍涓嶅悓銆?
 5) 棣栧厛澶囦唤鐜版湁闀滃儚锛氾細

	# dd if=/dev/mtd0ro of=bios.bak
	16384+0 records in
	16384+0 records out
	8388608 bytes (8.4 MB) copied, 10.0269 s, 837 kB/s

 6) 楠岃瘉澶囦唤锛氾細

	# sha1sum /dev/mtd0ro bios.bak
	fdbb011920572ca6c991377c4b418a0502668b73  /dev/mtd0ro
	fdbb011920572ca6c991377c4b418a0502668b73  bios.bak

    SHA1 鏍￠獙鍜屽繀椤诲尮閰嶃€傚惁鍒欎笉瑕佺户缁紒

 7) 鎿﹂櫎 SPI 涓茶闂瓨銆傛姝ラ涔嬪悗锛屼笉瑕侀噸鍚?    寮€鍙戞澘锛佸惁鍒欏畠灏嗘棤娉曞啀鍚姩锛氾細

	# flash_erase /dev/mtd0 0 0
	Erasing 4 Kibyte @ 7ff000 -- 100 % complete

 8) 鏃犻敊璇湴瀹屾垚鍚庯紝浣犲彲浠ュ啓鍏ユ柊鐨?BIOS 闀滃儚锛氾細

    # dd if=MNW2MAX1.X64.0092.R01.1605221712.bin of=/dev/mtd0

 9) 楠岃瘉 SPI 涓茶闂瓨鐨勬柊鍐呭鏄惁涓庢柊鐨?BIOS 闀滃儚鍖归厤锛氾細

	# sha1sum /dev/mtd0ro MNW2MAX1.X64.0092.R01.1605221712.bin
	9b4df9e4be2057fceec3a5529ec3d950836c87a2  /dev/mtd0ro
	9b4df9e4be2057fceec3a5529ec3d950836c87a2 MNW2MAX1.X64.0092.R01.1605221712.bin

    SHA1 鏍￠獙鍜屽簲褰撳尮閰嶃€?
 10) 鐜板湪浣犲彲浠ラ噸鍚紑鍙戞澘锛岃瀵熸柊鐨?BIOS 姝ｅ父鍚姩銆?
```
### 鍙傝€冩枃鐚?

[^1^] https://firmware.intel.com/sites/default/files/MinnowBoard%2EMAX_%2EX64%2E92%2ER01%2Ezip

[^2^] http://www.linux-mtd.infradead.org/
