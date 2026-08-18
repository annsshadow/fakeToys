## 浣跨敤鍒濆 RAM 纾佺洏锛坕nitrd锛?

Written 1996,2000 by Werner Almesberger <werner.almesberger@epfl.ch> and
Hans Lermen <lermen@fgan.de>


initrd 鎻愪緵浜嗛€氳繃寮曞鍔犺浇绋嬪簭鍔犺浇涓€涓?RAM 纾佺洏鐨勮兘鍔涖€傞殢鍚庤 RAM 纾佺洏鍙互琚寕杞?涓烘牴鏂囦欢绯荤粺锛屽苟鍙互浠庝腑杩愯绋嬪簭銆備箣鍚庯紝鍙互浠庡彟涓€涓澶囨寕杞戒竴涓柊鐨勬牴鏂囦欢绯荤粺銆?鍏堝墠鐨勬牴锛堟潵鑷?initrd锛夐殢鍚庤绉诲姩鍒颁竴涓洰褰曪紝骞跺彲浠ュ湪涔嬪悗琚嵏杞姐€?
initrd 涓昏璁捐鐢ㄤ簬璁╃郴缁熷惎鍔ㄥ垎涓や釜闃舵杩涜锛氬唴鏍镐互涓€缁勬渶灏忕殑鍐呯疆椹卞姩鍚姩锛岃€岄澶?鐨勬ā鍧椾粠 initrd 鍔犺浇銆?
鏈枃妗ｇ畝瑕佹杩?initrd 鐨勪娇鐢ㄣ€傚叧浜庡惎鍔ㄨ繃绋嬬殑鏇磋缁嗚璁哄彲鍙傝 [#f1]_銆?
### 鎿嶄綔


浣跨敤 initrd 鏃讹紝绯荤粺閫氬父濡備笅鍚姩锛?
  1) 寮曞鍔犺浇绋嬪簭鍔犺浇鍐呮牳鍜屽垵濮?RAM 纾佺洏
  2) 鍐呮牳灏?initrd 杞崲涓轰竴涓€滄櫘閫氣€濈殑 RAM 纾佺洏锛屽苟閲婃斁 initrd 鍗犵敤鐨勫唴瀛?  3) 濡傛灉鏍硅澶囦笉鏄?`/dev/ram0`锛屽垯閬靛惊鏃х殑锛堝凡搴熷純鐨勶級change_root 娴佺▼銆傚弬瑙?     涓嬫枃鈥滆繃鏃剁殑鏍瑰垏鎹㈡満鍒垛€濅竴鑺傘€?  4) 鎸傝浇鏍硅澶囥€傚鏋滄槸 `/dev/ram0`锛屽垯灏?initrd 闀滃儚鎸傝浇涓烘牴
  5) 鎵ц /sbin/init锛堣繖鍙互鏄换浣曟湁鏁堢殑鍙墽琛屾枃浠讹紝鍖呮嫭 shell 鑴氭湰锛涘畠浠?uid 0
     杩愯锛屽嚑涔庡彲浠ュ仛 init 鑳藉仛鐨勪换浣曚簨鎯咃級
  6) init 鎸傝浇鈥滅湡姝ｇ殑鈥濇牴鏂囦欢绯荤粺
  7) init 浣跨敤 pivot_root 绯荤粺璋冪敤灏嗘牴鏂囦欢绯荤粺鏀惧埌鏍圭洰褰?  8) init 鍦ㄦ柊鏍规枃浠剁郴缁熶笂 exec `/sbin/init`锛屾墽琛岄€氬父鐨勫惎鍔ㄥ簭鍒?  9) 绉婚櫎 initrd 鏂囦欢绯荤粺

娉ㄦ剰锛屾洿鏀规牴鐩綍骞朵笉娑夊強鍗歌浇瀹冦€傚洜姝わ紝鍦ㄦ杩囩▼涓彲浠ヨ杩涚▼缁х画鍦?initrd 涓婅繍琛屻€?鍚屾椂涔熻娉ㄦ剰锛屽湪 initrd 涓嬫寕杞界殑鏂囦欢绯荤粺鍦ㄦ鏈熼棿浠嶅彲璁块棶銆?

### 寮曞鍛戒护琛岄€夐」


```
  initrd=<path>    (e.g. LOADLIN)

    Loads the specified file as the initial RAM disk. When using LILO, you
    have to specify the RAM disk image file in /etc/lilo.conf, using the
    INITRD configuration variable.

  noinitrd

    initrd data is preserved but it is not converted to a RAM disk and
    the "normal" root file system is mounted. initrd data can be read
    from /dev/initrd. Note that the data in initrd can have any structure
    in this case and doesn't necessarily have to be a file system image.
    This option is used mainly for debugging.

    Note: /dev/initrd is read-only and it can only be used once. As soon
    as the last process has closed it, all data is freed and /dev/initrd
    can't be opened anymore.

  root=/dev/ram0

    initrd is mounted as root, and the normal boot procedure is followed,
    with the RAM disk mounted as root.
```

### 鍘嬬缉鐨?cpio 闀滃儚


杩戞湡鐨勫唴鏍告敮鎸佷粠涓€涓帇缂╃殑 cpio 褰掓。鏉ュ～鍏?ramdisk銆傚湪杩欑被绯荤粺涓婏紝鍒涘缓 ramdisk
闀滃儚涓嶅啀闇€瑕佹秹鍙婄壒娈婄殑鍧楄澶囨垨鍥炵幆璁惧锛涗綘鍙渶鍦ㄧ鐩樹笂鍒涘缓涓€涓寘鍚墍闇€ initrd
鍐呭鐨勭洰褰曪紝cd 杩涘叆璇ョ洰褰曪紝鐒跺悗杩愯锛堜互
```
	find . | cpio --quiet -H newc -o | gzip -9 -n > /boot/imagefile.img
```
```
	mkdir /tmp/imagefile
	cd /tmp/imagefile
	gzip -cd /boot/imagefile.img | cpio -imd --quiet
```
### 瀹夎


棣栧厛锛屽繀椤诲湪
```
	# mkdir /initrd
```
涓婂垱寤轰竴涓敤浜?initrd 鏂囦欢绯荤粺鐨勭洰褰曘€傝鍚嶇О骞朵笉閲嶈銆傛洿澶氱粏鑺傚彲鍦?`pivot_root(2)`
鎵嬪唽椤典腑鎵惧埌銆?
濡傛灉鏍规枃浠剁郴缁熸槸鍦ㄥ惎鍔ㄨ繃绋嬩腑鍒涘缓鐨勶紙鍗冲鏋滀綘鍦ㄥ埗浣滀竴寮犲畨瑁呰蒋鐩橈級锛屽垯鏍规枃浠剁郴缁熺殑
鍒涘缓杩囩▼搴斿綋鍒涘缓 `/initrd` 鐩綍銆?
濡傛灉鍦ㄦ煇浜涙儏鍐典笅 initrd 涓嶄細琚寕杞斤紝鍏跺唴瀹逛粛鐒舵槸
```
	# mknod /dev/initrd b 1 250
	# chmod 400 /dev/initrd
```
鍏舵锛屽唴鏍稿繀椤荤紪璇戞椂鍚敤 RAM 纾佺洏鏀寔浠ュ強鍒濆 RAM 纾佺洏鏀寔銆傚悓鏃讹紝鑷冲皯鎵€鏈変粠
initrd 鎵ц绋嬪簭鎵€闇€鐨勭粍浠讹紙渚嬪鍙墽琛屾枃浠舵牸寮忓拰鏂囦欢绯荤粺锛夐兘蹇呴』缂栬瘧杩涘唴鏍搞€?
绗笁锛屼綘蹇呴』鍒涘缓 RAM 纾佺洏闀滃儚銆傝繖閫氳繃鍦ㄤ竴涓潡璁惧涓婂垱寤烘枃浠剁郴缁熴€佹寜闇€灏嗘枃浠跺鍒跺埌
鍏朵腑锛岀劧鍚庡皢璇ュ潡璁惧鐨勫唴瀹瑰鍒跺埌 initrd 鏂囦欢鏉ュ畬鎴愩€傚浜庤繎鏈熺殑鍐呮牳锛岃嚦灏戞湁涓夌被
璁惧閫傚悎浜庢锛?
 - 杞洏锛堝埌澶勫彲鐢ㄤ絾鎱㈠緱浠や汉鐥涜嫤锛? - RAM 纾佺洏锛堝揩锛屼絾浼氬垎閰嶇墿鐞嗗唴瀛橈級
 - 鍥炵幆璁惧锛堟渶浼橀泤鐨勬柟妗堬級

鎴戜滑灏嗘弿杩板洖鐜澶囨柟娉曪細

 1) 纭繚鍥炵幆鍧楄澶囧凡閰嶇疆杩涘唴鏍?```
	# dd if=/dev/zero of=initrd bs=300k count=1
	# mke2fs -F -m0 initrd

    (if space is critical, you may want to use the Minix FS instead of Ext2)
 3) mount the file system, e.g.::

	# mount -t ext2 -o loop initrd /mnt

 4) create the console device::

    # mkdir /mnt/dev
    # mknod /mnt/dev/console c 5 1

 5) copy all the files that are needed to properly use the initrd
    environment. Don't forget the most important file, ``/sbin/init``

    .. note:: ``/sbin/init`` permissions must include "x" (execute).

 6) correct operation the initrd environment can frequently be tested
    even without rebooting with the command::

	# chroot /mnt /sbin/init

    This is of course limited to initrds that do not interfere with the
    general system state (e.g. by reconfiguring network interfaces,
    overwriting mounted devices, trying to start already running demons,
    etc. Note however that it is usually possible to use pivot_root in
    such a chroot'ed initrd environment.)
 7) unmount the file system::

	# umount /mnt

 8) the initrd is now in the file "initrd". Optionally, it can now be
    compressed::

	# gzip -9 initrd
```
涓轰簡璇曢獙 initrd锛屼綘鍙兘鎯虫嬁涓€寮犳晳鎻磋蒋鐩橈紝骞跺彧浠?`/sbin/init` 娣诲姞涓€涓寚鍚?`/bin/sh` 鐨勭鍙烽摼鎺ャ€傚彟澶栵紝浣犱篃鍙互灏濊瘯瀹為獙鎬х殑 newlib 鐜 [#f2]_ 鏉ュ垱寤轰竴涓?灏忓瀷 initrd銆?
鏈€鍚庯紝浣犲繀椤诲紩瀵煎唴鏍稿苟鍔犺浇 initrd銆傚嚑涔庢墍鏈?Linux 寮曞鍔犺浇绋嬪簭閮芥敮鎸?initrd銆傜敱浜?鍚姩杩囩▼浠嶄笌鏃ф満鍒跺吋瀹癸紝浠ヤ笅寮曞鍛戒护琛屽弬鏁?```
  root=/dev/ram0 rw
```
锛坮w 鍙湁鍦ㄩ渶瑕佸啓鍏?initrd 鏂囦欢绯荤粺鏃舵墠鏄繀瑕佺殑锛?```
     LOADLIN <kernel> initrd=<disk_image>
```
```
	LOADLIN C:\LINUX\BZIMAGE initrd=C:\LINUX\INITRD.GZ root=/dev/ram0 rw
```
浣跨敤 LILO锛屼綘鍙互鍦?`/etc/lilo.conf` 鐨勫叏灞€娈垫垨鐩稿簲鍐呮牳鐨勬涓坊鍔犻€夐」
`INITRD=<path>`锛屽苟浼犻€?```
  image = /bzImage
    initrd = /boot/initrd.gz
    append = "root=/dev/ram0 rw"
```
鐒跺悗杩愯 `/sbin/lilo`

鍏充簬鍏跺畠寮曞鍔犺浇绋嬪簭锛岃鍙傝€冨悇鑷殑鏂囨。銆?
鐜板湪浣犲彲浠ュ紩瀵煎苟浜彈浣跨敤 initrd 浜嗐€?

### 鏇存敼鏍硅澶?

瀹屾垚鍏惰亴璐ｅ悗锛宨nit 閫氬父浼氭洿鏀规牴璁惧锛屽苟缁х画鍦ㄢ€滅湡姝ｇ殑鈥濇牴璁惧涓婂惎鍔?Linux 绯荤粺銆?
璇ユ祦绋嬪寘鍚互涓嬫楠わ細
 - 鎸傝浇鏂扮殑鏍规枃浠剁郴缁? - 灏嗗叾鍙樹负鏍规枃浠剁郴缁? - 绉婚櫎瀵规棫锛坕nitrd锛夋牴鏂囦欢绯荤粺鐨勬墍鏈夎闂? - 鍗歌浇 initrd 鏂囦欢绯荤粺骞堕噴鏀?RAM 纾佺洏

鎸傝浇鏂扮殑鏍规枃浠剁郴缁熷緢瀹规槗锛氬彧闇€灏嗗叾鎸傝浇鍒?```
	# mkdir /new-root
	# mount -o ro /dev/hda1 /new-root
```
鏍瑰垏鎹㈤€氳繃 pivot_root 绯荤粺璋冪敤瀹屾垚锛岃璋冪敤涔熷彲浠ラ€氳繃 `pivot_root` 瀹炵敤绋嬪簭鑾峰緱
锛堝弬瑙?`pivot_root(8)` 鎵嬪唽椤碉紱`pivot_root` 闅?util-linux 2.10h 鎴栨洿楂樼増鏈垎鍙?[#f3]_锛夈€俙pivot_root` 灏嗗綋鍓嶆牴绉诲姩涓烘柊鏍逛笅鐨勪竴涓洰褰曪紝骞跺皢鏂版牴鏀惧埌瀹冪殑浣嶇疆銆傛棫鏍?鐨勭洰褰?```
	# cd /new-root
	# mkdir initrd
	# pivot_root . initrd
```
鐜板湪锛宨nit 杩涚▼浠嶅彲閫氳繃鍏跺彲鎵ц鏂囦欢銆佸叡浜簱銆佹爣鍑嗚緭鍏?杈撳嚭/閿欒浠ュ強鍏跺綋鍓嶆牴鐩綍
璁块棶鏃ф牴銆傛墍鏈夎繖浜涘紩鐢ㄩ€氳繃
```
	# exec chroot . what-follows <dev/console >dev/console 2>&1
```
琚涪寮冿紝鍏朵腑 what-follows 鏄柊鏍逛笅鐨勪竴涓▼搴忥紝渚嬪 `/sbin/init`銆傚鏋滄柊鏍规枃浠剁郴缁?灏嗕笌 udev 涓€璧蜂娇鐢ㄤ笖娌℃湁鏈夋晥鐨?`/dev` 鐩綍锛屽垯蹇呴』鍦ㄨ皟鐢?chroot 涔嬪墠鍒濆鍖?udev锛?浠ユ彁渚?`/dev/console`銆?
娉ㄦ剰锛歱ivot_root 鐨勫疄鐜扮粏鑺傚彲鑳介殢鏃堕棿鍙樺寲銆備负浜嗙‘淇濆吋瀹规€э紝搴旀敞鎰忎互涓嬪嚑鐐癸細

 - 鍦ㄨ皟鐢?pivot_root 涔嬪墠锛岃皟鐢ㄨ繘绋嬬殑褰撳墠鐩綍搴旀寚鍚戞柊鏍圭洰褰? - 浣跨敤 . 浣滀负绗竴涓弬鏁帮紝骞跺皢鏃ф牴鐩綍鐨刜鐩稿_璺緞浣滀负绗簩涓弬鏁? - 鍦ㄦ棫鏍瑰拰鏂版牴涓嬮兘蹇呴』鏈変竴涓彲鐢ㄧ殑 chroot 绋嬪簭
 - 涔嬪悗 chroot 鍒版柊鏍? - 鍦?exec 鍛戒护涓 dev/console 浣跨敤鐩稿璺緞

鐜板湪锛宨nitrd 鍙互琚嵏杞斤紝RAM 鍒嗛厤鐨勫唴瀛樺彲浠ヨ
```
	# umount /initrd
	# blockdev --flushbufs /dev/ram0
```
涔熷彲浠ュ皢 initrd 涓?NFS 鎸傝浇鐨勬牴涓€璧蜂娇鐢紝璇﹁ `pivot_root(8)` 鎵嬪唽椤点€?

### 浣跨敤鍦烘櫙


瀹炵幇 initrd 鐨勪富瑕佸姩鏈烘槸鍏佽鍦ㄧ郴缁熷畨瑁呮椂杩涜妯″潡鍖栧唴鏍搁厤缃€傛祦绋嬪涓嬶細

  1) 绯荤粺浠庤蒋鐩樻垨鍏跺畠浠嬭川浠ユ渶灏忓唴鏍革紙渚嬪鏀寔 RAM 纾佺洏銆乮nitrd銆乤.out 鍜?Ext2
     FS锛夊惎鍔ㄥ苟鍔犺浇 initrd
  2) `/sbin/init` 纭畾闇€瑕佷粈涔堟潵锛?锛夋寕杞解€滅湡姝ｇ殑鈥濇牴 FS锛堝嵆璁惧绫诲瀷銆佽澶囬┍鍔ㄣ€?     鏂囦欢绯荤粺锛変互鍙婏紙2锛夊彂琛屼粙璐紙渚嬪 CD-ROM銆佺綉缁溿€佺甯︹€︹€︼級銆傝繖鍙互閫氳繃璇㈤棶
     鐢ㄦ埛銆佽嚜鍔ㄦ帰娴嬫垨娣峰悎鏂规硶鏉ュ畬鎴愩€?  3) `/sbin/init` 鍔犺浇蹇呰鐨勫唴鏍告ā鍧?  4) `/sbin/init` 鍒涘缓骞跺～鍏呮牴鏂囦欢绯荤粺锛堣繖杩樹笉蹇呮槸涓€涓潪甯稿ソ鐢ㄧ殑绯荤粺锛?  5) `/sbin/init` 璋冪敤 `pivot_root` 鏉ユ洿鏀规牴鏂囦欢绯荤粺锛屽苟閫氳繃 chroot exec 涓€涓?     缁х画瀹夎鐨勭▼搴?  6) 瀹夎寮曞鍔犺浇绋嬪簭
  7) 寮曞鍔犺浇绋嬪簭琚厤缃负鍔犺浇涓€涓寘鍚敤浜庡惎鍔ㄧ郴缁熺殑妯″潡闆嗙殑 initrd锛堜緥濡傚彲浠?     淇敼 `/initrd`锛岀劧鍚庡嵏杞斤紝鏈€鍚庡皢闀滃儚浠?`/dev/ram0` 鎴?`/dev/rd/0` 鍐欏叆
     鏂囦欢锛?  8) 鐜板湪绯荤粺鍙紩瀵硷紝骞朵笖鍙互鎵ц棰濆鐨勫畨瑁呬换鍔?
initrd 鍦ㄨ繖閲岀殑鍏抽敭浣滅敤鏄紝鍦ㄦ甯哥郴缁熻繍琛屾湡闂村鐢ㄩ厤缃暟鎹紝鑰屾棤闇€浣跨敤涓€涓噧鑲跨殑
鈥滈€氱敤鈥濆唴鏍革紝涔熸棤闇€閲嶆柊缂栬瘧鎴栭噸鏂伴摼鎺ュ唴鏍搞€?
绗簩绉嶅満鏅敤浜庤繖鏍风殑瀹夎锛歀inux 杩愯鍦ㄥ崟涓€绠＄悊鍩熷唴銆佸叿鏈変笉鍚岀‖浠堕厤缃殑绯荤粺涓娿€傚湪
杩欑鎯呭喌涓嬶紝鏈€濂藉彧鐢熸垚涓€灏忛儴鍒嗗唴鏍革紙鐞嗘兂鎯呭喌涓嬪彧鏈変竴涓級锛屽苟璁╅厤缃俊鎭腑绯荤粺鐗瑰畾鐨?閮ㄥ垎灏藉彲鑳藉皬銆傚湪杩欑鎯呭喌涓嬶紝鍙互鐢熸垚涓€涓寘鍚墍鏈夊繀瑕佹ā鍧楃殑鍏叡 initrd銆傞偅涔堬紝鍙湁
`/sbin/init` 鎴栧畠鎵€璇诲彇鐨勪竴涓枃浠堕渶瑕佷笉鍚屻€?
绗笁绉嶅満鏅槸鏇存柟渚跨殑鏁戞彺鐩橈紝鍥犱负鍍忔牴 FS 鍒嗗尯浣嶇疆杩欑被淇℃伅鏃犻渶鍦ㄥ惎鍔ㄦ椂鎻愪緵锛岃€屼粠
initrd 鍔犺浇鐨勭郴缁熷彲浠ヨ皟鐢ㄤ竴涓敤鎴峰弸濂界殑瀵硅瘽妗嗭紝骞朵笖杩樺彲浠ユ墽琛屼竴浜涘仴鍏ㄦ€ф鏌ワ紙鐢氳嚦
鏌愮褰㈠紡鐨勮嚜鍔ㄦ娴嬶級銆?
鏈€鍚庝絾骞堕潪鏈€涓嶉噸瑕佺殑鏄紝CD-ROM 鍙戣鍟嗗彲浠ュ€熷姪瀹冨疄鐜版洿濂界殑浠?CD 瀹夎锛屼緥濡傞€氳繃浣跨敤
寮曞杞洏骞堕€氳繃 initrd 浠?CD 寮曞涓€涓洿澶х殑 RAM 纾佺洏锛涙垨鑰呴€氳繃涓€涓儚 `LOADLIN`
杩欐牱鐨勫姞杞界▼搴忕洿鎺ヤ粠 CD-ROM 寮曞锛屽苟浠?CD 鍔犺浇 RAM 纾佺洏鑰屾棤闇€杞洏銆?

### 杩囨椂鐨勬牴鍒囨崲鏈哄埗


浠ヤ笅鏈哄埗鍦ㄥ紩鍏?pivot_root 涔嬪墠浣跨敤銆傚綋鍓嶅唴鏍镐粛鐒舵敮鎸佸畠锛屼絾浣犱笉搴斿綋渚濊禆瀹冪户缁?鍙敤銆?
瀹冮€氳繃鍦ㄥ唴鏍告槧鍍忎腑鐢?rdev 璁剧疆銆佹垨鍦ㄥ紩瀵煎懡浠よ鐢?root=... 璁剧疆鐨勨€滅湡姝ｇ殑鈥濇牴璁惧锛?鍦?linuxrc 閫€鍑烘椂鎸傝浇涓烘牴鏂囦欢绯荤粺鏉ュ伐浣溿€傜劧鍚?initrd 鏂囦欢绯荤粺琚嵏杞斤紝鎴栬€咃紝濡傛灉瀹?浠嶇劧蹇欙紝鍒欒绉诲姩鍒颁竴涓洰褰?`/initrd`锛堝鏋滆鐩綍瀛樺湪浜庢柊鏍规枃浠剁郴缁熶笂锛夈€?
涓轰簡浣跨敤杩欑鏈哄埗锛屼綘鏃犻渶鎸囧畾寮曞鍛戒护閫夐」 root銆乮nit 鎴?rw銆傦紙濡傛灉鎸囧畾浜嗭紝瀹冧滑灏?褰卞搷鐪熸鐨勬牴鏂囦欢绯荤粺锛岃€屼笉鏄?initrd 鐜銆傦級

濡傛灉鎸傝浇浜?/proc锛屸€滅湡姝ｇ殑鈥濇牴璁惧鍙互閫氳繃浠?linuxrc 鍐呴儴灏嗘柊鏍?FS 璁惧鐨勭紪鍙峰啓鍏?鐗规畩鐨?```
  # echo 0x301 >/proc/sys/kernel/real-root-dev
```
娉ㄦ剰锛岃鏈哄埗涓?NFS 鍙婄被浼兼枃浠剁郴缁熶笉鍏煎銆?
杩欎釜鏃х殑銆佸凡搴熷純鐨勬満鍒堕€氬父绉颁负 `change_root`锛岃€屾柊鐨勩€佸彈鏀寔鐨勬満鍒剁О涓?`pivot_root`銆?

### change_root 涓?pivot_root 娣峰悎鏈哄埗


濡傛灉浣犱笉鎯充娇鐢?`root=/dev/ram0` 鏉ヨЕ鍙?pivot_root 鏈哄埗锛屼綘鍙互鍦?initrd 闀滃儚涓?鍚屾椂鍒涘缓 `/linuxrc` 鍜?`/sbin/init`銆?
```
	#! /bin/sh
	mount -n -t proc proc /proc
	echo 0x0100 >/proc/sys/kernel/real-root-dev
	umount -n /proc
```
涓€鏃?linuxrc 閫€鍑猴紝鍐呮牳浼氬啀娆″皢浣犵殑 initrd 鎸傝浇涓烘牴锛岃繖娆℃墽琛?`/sbin/init`銆傚悓鏍凤紝
灏嗙敱杩欎釜 init 璐熻矗鍦ㄦ渶缁堟墽琛岀湡姝ｇ殑 `/sbin/init` 涔嬪墠鏋勫缓姝ｇ‘鐨勭幆澧冿紙涔熻浣跨敤
鍛戒护琛屼笂浼犲叆鐨?`root= device`锛夈€?

### 璧勬簮


    https://www.almesberger.net/cv/papers/ols2k-9.ps.gz
    https://www.sourceware.org/newlib/
    https://www.kernel.org/pub/linux/utils/util-linux/
