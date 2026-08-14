
## Ramfs銆乺ootfs 涓?initramfs

2005 骞?10 鏈?17 鏃?
:Author: Rob Landley <rob@landley.net>

### 浠€涔堟槸 ramfs锛?
ramfs 鏄竴涓潪甯哥畝鍗曠殑鏂囦欢绯荤粺锛屽畠灏?Linux 鐨勭鐩樼紦瀛樻満鍒讹紙page cache 涓?dentry cache锛変綔涓轰竴涓彲鍔ㄦ€佽皟鏁村ぇ灏忕殑銆佸熀浜?RAM 鐨勬枃浠剁郴缁熷鍑恒€?
閫氬父锛孡inux 浼氬皢鎵€鏈夋枃浠剁紦瀛樺湪鍐呭瓨涓€備粠鍚庡瀛樺偍锛坆acking store锛岄€氬父鏄枃浠剁郴缁熸寕杞藉叾涓婄殑鍧楄澶囷級璇诲嚭鐨勬暟鎹〉浼氳淇濈暀锛屼互澶囧啀娆￠渶瑕佹椂浣跨敤锛屼絾鍚屾椂浼氳鏍囪涓?clean锛堝彲閲婃斁锛夛紝浠ヤ究铏氭嫙鍐呭瓨锛圴irtual Memory锛夌郴缁熷湪闇€瑕佸唴瀛樺仛鍏朵粬鐢ㄩ€旀椂灏嗗叾鍥炴敹銆傜被浼煎湴锛屽啓鍏ユ枃浠剁殑鏁版嵁涓€鏃﹀啓鍏ュ悗澶囧瓨鍌ㄥ氨浼氳鏍囪涓?clean锛屼絾浼氫负浜嗙紦瀛樼洰鐨勮€屼繚鐣欙紝鐩村埌 VM 閲嶆柊鍒嗛厤璇ュ唴瀛樸€傜被浼肩殑鏈哄埗锛坉entry cache锛夊ぇ澶у姞蹇簡璁块棶鐩綍鐨勯€熷害銆?
瀵逛簬 ramfs锛屾病鏈夊悗澶囧瓨鍌ㄣ€傚啓鍏?ramfs 鐨勬枃浠朵細鍍忓線甯镐竴鏍峰垎閰?dentry 涓?page cache锛屼絾鍗存病鏈夊湴鏂瑰彲浠ュ啓鍏ャ€傝繖鎰忓懗鐫€杩欎簺椤垫案杩滀笉浼氳鏍囪涓?clean锛屽洜姝ゅ綋 VM 鎯宠鍥炴敹鍐呭瓨鏃舵棤娉曢噴鏀惧畠浠€?
瀹炵幇 ramfs 鎵€闇€鐨勪唬鐮侀噺闈炲父灏忥紝鍥犱负鎵€鏈夊伐浣滈兘鐢辩幇鏈夌殑 Linux 缂撳瓨鍩虹璁炬柦瀹屾垚銆備粠鏍规湰涓婅锛屼綘灏辨槸鎶婄鐩樼紦瀛樺綋浣滄枃浠剁郴缁熸寕杞姐€傛鍥犲姝わ紝ramfs 涓嶆槸涓€涓彲閫氳繃 menuconfig 绉婚櫎鐨勫彲閫夌粍浠讹紝鍥犱负閭ｆ牱鑺傜渷鐨勭┖闂村井涔庡叾寰€?
### ramfs 涓?ramdisk锛堝唴瀛樼洏锛?
杈冭€佺殑 "ram disk"锛堝唴瀛樼洏锛夋満鍒朵粠涓€鍧?RAM 鍖哄煙涓垱寤轰竴涓悎鎴愮殑鍧楄澶囷紝骞跺皢鍏剁敤浣滄枃浠剁郴缁熺殑鍚庡瀛樺偍銆傝繖涓潡璁惧澶у皬鍥哄畾锛屽洜姝ゆ寕杞藉叾涓婄殑鏂囦欢绯荤粺澶у皬涔熷浐瀹氥€備娇鐢?ram disk 杩橀渶瑕佷笉蹇呰鍦版妸鍐呭瓨浠庨偅涓亣鐨勫潡璁惧澶嶅埗鍒?page cache锛堝苟鎶婃敼鍔ㄥ鍒跺洖鍘伙級锛屼互鍙婂垱寤哄拰閿€姣?dentry銆傛澶栵紝瀹冭繕闇€瑕佷竴涓枃浠剁郴缁熼┍鍔紙濡?ext2锛夋潵鏍煎紡鍖栧苟瑙ｆ瀽杩欎簺鏁版嵁銆?
涓?ramfs 鐩告瘮锛岃繖浼氭氮璐瑰唴瀛橈紙浠ュ強鍐呭瓨鎬荤嚎甯﹀锛夛紝缁?CPU 甯︽潵涓嶅繀瑕佺殑宸ヤ綔锛屽苟姹℃煋 CPU 缂撳瓨銆傦紙铏界劧鏈変竴浜涢€氳繃鎿嶄綔椤佃〃鏉ラ伩鍏嶈繖绉嶅鍒剁殑鎶€宸э紝浣嗗畠浠鏉傚緱浠や汉涓嶅揩锛岃€屼笖缁撴灉璇佹槑浠ｄ环涓庡鍒跺樊涓嶅銆傦級鏇村叧閿殑鏄紝ramfs 鎵€鍋氱殑鎵€鏈夊伐浣滄棤璁哄浣曢兘蹇呴』鍙戠敓锛屽洜涓烘墍鏈夋枃浠惰闂兘瑕佺粡杩?page 涓?dentry 缂撳瓨銆俁AM disk 鏍规湰灏辨槸澶氫綑鐨勶紱ramfs 鍦ㄥ唴閮ㄨ绠€鍗曞緱澶氥€?
ramdisk 鍗婅繃鏃剁殑鍙︿竴涓師鍥犳槸锛宭oopback 璁惧锛堝洖鐜澶囷級鐨勫紩鍏ユ彁渚涗簡涓€绉嶆洿鐏垫椿銆佹洿鏂逛究鐨勬柟寮忔潵鍒涘缓鍚堟垚鍧楄澶団€斺€旂幇鍦ㄤ粠鏂囦欢鑰屼笉鏄粠鍐呭瓨鍧楀垱寤恒€傝瑙?losetup(8)銆?
### ramfs 涓?tmpfs

ramfs 鐨勪竴涓己鐐规槸锛屼綘鍙互涓€鐩村悜鍏朵腑鍐欏叆鏁版嵁锛岀洿鍒板～婊℃墍鏈夊唴瀛橈紝鑰?VM 鏃犳硶閲婃斁瀹冿紝鍥犱负 VM 璁や负鏂囦欢搴斿綋琚啓鍏ュ悗澶囧瓨鍌紙鑰岄潪浜ゆ崲绌洪棿锛夛紝浣?ramfs 娌℃湁浠讳綍鍚庡瀛樺偍銆傚洜姝わ紝鍙簲鍏佽 root锛堟垨涓€涓彈淇′换鐨勭敤鎴凤級瀵?ramfs 鎸傝浇鐐规嫢鏈夊啓璁块棶鏉冦€?
涓€涓悕涓?tmpfs 鐨?ramfs 琛嶇敓鐗╄鍒涢€犲嚭鏉ワ紝鐢ㄤ簬澧炲姞澶у皬闄愬埗锛屼互鍙婂皢鏁版嵁鍐欏叆浜ゆ崲绌洪棿鐨勮兘鍔涖€傛櫘閫氱敤鎴峰彲浠ヨ鍏佽瀵?tmpfs 鎸傝浇鐐规嫢鏈夊啓璁块棶鏉冦€傛洿澶氫俊鎭鍙傞槄 Documentation/filesystems/tmpfs.rst銆?
### 浠€涔堟槸 rootfs锛?
rootfs 鏄?ramfs锛堣嫢鍚敤浜?tmpfs锛屽垯涓?tmpfs锛夌殑涓€涓壒娈婂疄渚嬶紝瀹冨缁堝瓨鍦ㄤ簬 Linux 绯荤粺涓€傚唴鏍镐娇鐢ㄤ竴涓悕涓?nullfs 鐨勪笉鍙彉鐨勭┖鏂囦欢绯荤粺浣滀负 VFS 灞傜骇缁撴瀯鐨勭湡姝ｆ牴锛岃€屽彲鍙樼殑 rootfs锛坱mpfs/ramfs锛夋寕杞藉湪瀹冧箣涓娿€傝繖浣垮緱 pivot_root() 涓?initramfs 鐨勫嵏杞借兘澶熸甯稿伐浣溿€?
澶у鏁扮郴缁熷彧鏄妸鍙︿竴涓枃浠剁郴缁熸寕杞藉埌 rootfs 涔嬩笂锛岀劧鍚庡拷鐣ュ畠銆備竴涓┖鐨?ramfs 瀹炰緥鎵€鍗犵敤鐨勭┖闂存瀬灏忋€?
濡傛灉鍚敤浜?CONFIG_TMPFS锛宺ootfs 榛樿灏嗕娇鐢?tmpfs 鑰岄潪 ramfs銆傝嫢瑕佸己鍒朵娇鐢?ramfs锛屽彲鍦ㄥ唴鏍稿懡浠よ涓姞鍏?"rootfstype=ramfs"銆?
### 浠€涔堟槸 initramfs锛?
鎵€鏈?2.6 鐗堟湰鐨?Linux 鍐呮牳閮藉寘鍚竴涓?gzip 鍘嬬缉鐨?"cpio" 鏍煎紡褰掓。锛屽畠浼氬湪鍐呮牳鍚姩鏃惰В鍘嬪埌 rootfs 涓€傝В鍘嬩箣鍚庯紝鍐呮牳妫€鏌?rootfs 鏄惁鍖呭惈涓€涓悕涓?"init" 鐨勬枃浠讹紝濡傛灉鏄紝灏辨妸瀹冧綔涓?PID 1 鎵ц銆傝嫢璇?init 杩涚▼瀛樺湪锛屽畠璐熻矗灏嗙郴缁熺殑鍏朵綑閮ㄥ垎鍚姩璧锋潵锛屽寘鎷畾浣嶅苟鎸傝浇鐪熸鐨勬牴璁惧锛堝鏋滄湁锛夈€傚鏋滃唴宓岀殑 cpio 褰掓。瑙ｅ帇鍒?rootfs 涔嬪悗锛宺ootfs 涓粛涓嶅寘鍚?init 绋嬪簭锛屽唴鏍稿氨浼氶€€鍥炲埌杈冩棫鐨勪唬鐮侊紝鍘诲畾浣嶅苟鎸傝浇涓€涓牴鍒嗗尯锛岀劧鍚庝粠涓?exec 鏌愪釜鍙樹綋鐨?/sbin/init銆?
杩欎竴鍒囦笌鏃х殑 initrd 鍦ㄥ嚑涓柟闈㈡湁鎵€涓嶅悓锛?
  - 鏃х殑 initrd 濮嬬粓鏄竴涓嫭绔嬬殑鏂囦欢锛岃€?initramfs 褰掓。琚摼鎺ヨ繘 Linux 鍐呮牳鏄犲儚涓€傦紙`linux-*/usr` 鐩綍灏变笓闂ㄧ敤浜庡湪鍐呮牳鏋勫缓鏈熼棿鐢熸垚杩欎釜褰掓。銆傦級

  - 鏃х殑 initrd 鏂囦欢鏄竴涓?gzip 鍘嬬缉鐨勬枃浠剁郴缁熸槧鍍忥紙閲囩敤鏌愮鏂囦欢鏍煎紡锛屽 ext2锛岄渶瑕佸唴鏍稿唴寤洪┍鍔級锛岃€屾柊鐨?initramfs 褰掓。鏄竴涓?gzip 鍘嬬缉鐨?cpio 褰掓。锛堢被浼?tar锛屼絾鏇寸畝鍗曪紝鍙傝 cpio(1) 涓?Documentation/driver-api/early-userspace/buffer-format.rst锛夈€傚唴鏍哥殑 cpio 瑙ｅ帇浠ｇ爜涓嶄粎鏋佸叾灏忓阀锛岃€屼笖灞炰簬 __init 鏂囨湰涓庢暟鎹紝鍙互鍦ㄥ惎鍔ㄨ繃绋嬩腑琚涪寮冦€?
  - 鏃х殑 initrd 杩愯鐨勭▼搴忥紙鍚嶄负 /initrd锛岃€岄潪 /init锛変細鍋氫竴浜涜缃紝鐒跺悗杩斿洖鍐呮牳锛涜€屾潵鑷?initramfs 鐨?init 绋嬪簭涓嶅簲杩斿洖鍐呮牳銆傦紙濡傛灉 /init 闇€瑕佷氦鍑烘帶鍒舵潈锛屽畠鍙互鐢ㄤ竴涓柊鐨勬牴璁惧瑕嗙洊鎸傝浇 / 骞?exec 鍙︿竴涓?init 绋嬪簭銆傚弬瑙佷笅鏂囩殑 switch_root 宸ュ叿銆傦級

  - 褰撳垏鎹㈠埌鍙︿竴涓牴璁惧鏃讹紝initrd 浼氭墽琛?pivot_root锛岀劧鍚?umount 璇?ramdisk銆傜敱浜?nullfs 鏄湡姝ｇ殑鏍癸紝pivot_root() 鍙互姝ｅ父宸ヤ綔

```
      chdir(new_root);
      pivot_root(".", ".");
      umount2(".", MNT_DETACH);

    This is the preferred method for switching root filesystems.

```

### 濉厖 initramfs

2.6 鍐呮牳鐨勬瀯寤鸿繃绋嬫€绘槸鍒涘缓涓€涓?gzip 鍘嬬缉鐨?cpio 鏍煎紡 initramfs 褰掓。锛屽苟灏嗗叾閾炬帴杩涙渶缁堢殑鍐呮牳浜岃繘鍒舵枃浠朵腑銆傞粯璁ゆ儏鍐典笅锛岃繖涓綊妗ｆ槸绌虹殑锛堝湪 x86 涓婂崰鐢?134 瀛楄妭锛夈€?
閰嶇疆閫夐」 CONFIG_INITRAMFS_SOURCE锛堜綅浜?menuconfig 鐨?General Setup 涓紝瀹氫箟浜?usr/Kconfig锛夊彲鐢ㄤ簬鎸囧畾 initramfs 褰掓。鐨勬潵婧愶紝瀹冧細鑷姩琚苟鍏ユ渶缁堜簩杩涘埗鏂囦欢涓€傝閫夐」鍙互鎸囧悜涓€涓凡鏈夌殑 gzip 鍘嬬缉 cpio 褰掓。銆佷竴涓寘鍚緟褰掓。鏂囦欢鐨勭洰褰曪紝鎴栦竴涓枃鏈枃浠?
```
  dir /dev 755 0 0
  nod /dev/console 644 0 0 c 5 1
  nod /dev/loop0 644 0 0 b 7 0
  dir /bin 755 1000 1000
  slink /bin/sh busybox 777 0 0
  file /bin/busybox initramfs/busybox 755 0 0
  dir /proc 755 0 0
  dir /sys 755 0 0
  dir /mnt 755 0 0
  file /init initramfs/init.sh 755 0 0

```

鍦ㄥ唴鏍告瀯寤轰箣鍚庤繍琛?"usr/gen_init_cpio" 鍙互鑾峰彇鎻忚堪涓婅堪鏂囦欢鏍煎紡鐨勪娇鐢ㄨ鏄庛€?
閰嶇疆鏂囦欢鐨勪竴涓紭鐐规槸锛屽湪鏂板綊妗ｄ腑璁剧疆鏉冮檺鎴栧垱寤鸿澶囪妭鐐逛笉闇€瑕?root 鏉冮檺銆傦紙娉ㄦ剰锛岄偅涓や釜绀轰緥 "file" 鏉＄洰鏈熸湜鍦?linux-2.6.* 鐩綍涓嬪悕涓?"initramfs" 鐨勫瓙鐩綍涓壘鍒板悕涓?"init.sh" 涓?"busybox" 鐨勬枃浠躲€傛洿澶氱粏鑺傝鍙傞槄 Documentation/driver-api/early-userspace/early_userspace_support.rst銆傦級

鍐呮牳骞朵笉渚濊禆澶栭儴鐨?cpio 宸ュ叿銆傚鏋滀綘鎸囧畾鐨勬槸涓€涓洰褰曡€屼笉鏄厤缃枃浠讹紝鍐呮牳鐨勬瀯寤哄熀纭€璁炬柦浼氱敱璇ョ洰褰曠敓鎴愪竴涓厤缃枃浠讹紙usr/Makefile 璋冪敤 usr/gen_initramfs.sh锛夛紝鐒跺悗缁х画浣跨敤璇ラ厤缃枃浠舵墦鍖呰鐩綍锛堝皢鍏跺杺缁?usr/gen_init_cpio锛屽悗鑰呯敱 usr/gen_init_cpio.c 鐢熸垚锛夈€傚唴鏍稿湪鏋勫缓鏃跺垱寤?cpio 鐨勪唬鐮佹槸瀹屽叏鑷寘鍚殑锛屽唴鏍稿湪鍚姩鏃剁殑瑙ｅ帇鍣ㄥ悓鏍凤紙鏄剧劧锛夋槸鑷寘鍚殑銆?
浣犲敮涓€鍙兘闇€瑕佸畨瑁呭閮?cpio 宸ュ叿鐨勬儏鍐垫槸锛岃鍒涘缓鎴栬В鍘嬩綘鑷繁棰勫厛鍑嗗濂界殑銆佸杺缁欏唴鏍告瀯寤虹殑 cpio 鏂囦欢锛堣€屼笉鏄敤閰嶇疆鏂囦欢鎴栫洰褰曪級銆?
浠ヤ笅鍛戒护琛屽彲浠ヨВ鍘嬩竴涓?cpio 鏄犲儚锛堟棤璁烘槸閫氳繃涓婇潰鐨勮剼鏈?
```
  cpio -i -d -H newc -F initramfs_data.cpio --no-absolute-filenames

```

浠ヤ笅 shell 鑴氭湰鍙互鍒涘缓涓€涓鏋勫缓鐨?cpio 褰掓。锛屼綘鍙互

```
  #!/bin/sh

  # Copyright 2006 Rob Landley <rob@landley.net> and TimeSys Corporation.
  # Licensed under GPL version 2

  if [ $# -ne 2 ]
  then
    echo "usage: mkinitramfs directory imagename.cpio.gz"
    exit 1
  fi

  if [ -d "$1" ]
  then
    echo "creating $2 from $1"
    (cd "$1"; find . | cpio -o -H newc | gzip) > "$2"
  else
    echo "First argument must be a directory"
    exit 1
  fi

```

    cpio 鐨?man 鎵嬪唽椤靛寘鍚竴浜涚碂绯曠殑寤鸿锛屽鏋滀綘鐓у仛浼氱牬鍧忎綘鐨?initramfs 褰掓。銆傚畠璇?鐢熸垚鏂囦欢鍚嶅垪琛ㄧ殑鍏稿瀷鏂瑰紡鏄娇鐢?find 鍛戒护锛涗綘搴旇缁?find 鍔犱笂 -depth 閫夐」锛屼互灏介噺鍑忓皯瀵逛笉鍙啓鎴栦笉鍙悳绱㈢洰褰曠殑鏉冮檺闂銆?鍦ㄥ垱寤?initramfs.cpio.gz 鏄犲儚鏃跺垏鍕胯繖鏍峰仛锛岄偅鏍锋槸琛屼笉閫氱殑銆侺inux 鍐呮牳鐨?cpio 瑙ｅ帇鍣ㄤ笉浼氬湪涓嶅瓨鍦ㄧ殑鐩綍涓垱寤烘枃浠讹紝鍥犳鐩綍鏉＄洰蹇呴』鍑虹幇鍦ㄨ鐩綍涓殑鏂囦欢涔嬪墠銆備笂闈㈢殑鑴氭湰浠ユ纭殑椤哄簭鐢熸垚瀹冧滑銆?
### 澶栭儴 initramfs 鏄犲儚

濡傛灉鍐呮牳鍚敤浜?initrd 鏀寔锛屼篃鍙互鎶婁竴涓閮ㄧ殑 cpio.gz 褰掓。褰撲綔 initrd 浼犲叆 2.6 鍐呮牳銆傚湪杩欑鎯呭喌涓嬶紝鍐呮牳浼氳嚜鍔ㄦ娴嬪叾绫诲瀷锛坕nitramfs锛岃€岄潪 initrd锛夛紝骞跺湪灏濊瘯杩愯 /init 涔嬪墠灏嗚澶栭儴 cpio 褰掓。瑙ｅ帇鍒?rootfs 涓€?
杩欏叿鏈?initramfs 鐨勫唴瀛樻晥鐜囦紭鍔匡紙娌℃湁 ramdisk 鍧楄澶囷級锛屽悓鏃跺張鏈?initrd 鐨勭嫭绔嬫墦鍖呯壒鎬э紙濡傛灉浣犳兂浠?initramfs 杩愯闈?GPL 鐨勪唬鐮侊紝鑰屽張涓嶆妸瀹冧笌浠?GPL 璁稿彲鐨?Linux 鍐呮牳浜岃繘鍒舵贩鍦ㄤ竴璧凤紝杩欏氨寰堟柟渚匡級銆?
### initramfs 鐨勫唴瀹?
initramfs 褰掓。鏄竴涓畬鏁淬€佽嚜鍖呭惈鐨?Linux 鏍规枃浠剁郴缁熴€傚鏋滀綘杩樹笉浜嗚В瑕佽涓€涓渶灏忔牴鏂囦欢绯荤粺鍚姩杩愯闇€瑕佸摢浜涘叡浜簱銆佽澶囧拰璺緞锛屽彲浠ュ弬鑰冧互涓嬭祫鏂欙細

- https://www.tldp.org/HOWTO/Bootdisk-HOWTO/
- https://www.tldp.org/HOWTO/From-PowerUp-To-Bash-Prompt-HOWTO.html
- http://www.linuxfromscratch.org/lfs/view/stable/

"klibc" 杞欢鍖咃紙https://www.kernel.org/pub/linux/libs/klibc锛夎璁捐鎴愪竴涓瀬灏忕殑 C 搴擄紝鐢ㄤ簬鏃╂湡鐢ㄦ埛绌洪棿浠ｇ爜鐨勯潤鎬侀摼鎺ワ紝骞堕檮甯︿竴浜涚浉鍏崇殑宸ュ叿銆傚畠閲囩敤 BSD 璁稿彲銆?
鎴戣嚜宸变娇鐢?uClibc锛坔ttps://www.uclibc.org锛変笌 busybox锛坔ttps://www.busybox.net锛夈€傚畠浠垎鍒噰鐢?LGPL 涓?GPL 璁稿彲銆傦紙busybox 1.3 鐗堟湰璁″垝鎻愪緵涓€涓嚜鍖呭惈鐨?initramfs 杞欢鍖呫€傦級

鐞嗚涓婁綘鍙互浣跨敤 glibc锛屼絾瀹冨苟涓嶉€傚悎杩欑被灏忓瀷宓屽叆寮忕敤閫斻€傦紙涓€涓潤鎬侀摼鎺?glibc 鐨?"hello world" 绋嬪簭瓒呰繃 400k锛岃€岀敤 uClibc 鍙湁 7k銆傝繕瑕佹敞鎰忥紝glibc 浼氶€氳繃 dlopen 鍔犺浇 libnss 鏉ュ仛鍚嶇О鏌ユ壘锛屽嵆浣垮叾浠栧湴鏂规槸闈欐€侀摼鎺ョ殑銆傦級

涓€涓ソ鐨勭涓€姝ユ槸璁?initramfs 杩愯涓€涓潤鎬侀摼鎺ョ殑 "hello world" 绋嬪簭浣滀负 init锛屽苟鍦?qemu锛坵ww.qemu.org锛変箣绫荤殑妯℃嫙鍣ㄤ笅娴嬭瘯瀹冿紝鎴栬€?
```
  cat > hello.c << EOF
  #include <stdio.h>
  #include <unistd.h>

  int main(int argc, char *argv[])
  {
    printf("Hello world!\n");
    sleep(999999999);
  }
  EOF
  gcc -static hello.c -o init
  echo init | cpio -o -H newc | gzip > test.cpio.gz
  # Testing external initramfs using the initrd loading mechanism.
  qemu -kernel /boot/vmlinuz -initrd test.cpio.gz /dev/zero

```

鍦ㄨ皟璇曚竴涓櫘閫氭牴鏂囦欢绯荤粺鏃讹紝鑳藉鐢?"init=/bin/sh" 鍚姩鏄緢鏂逛究鐨勩€俰nitramfs 鐨勭瓑浠峰仛娉曟槸 "rdinit=/bin/sh"锛屽畠鍚屾牱鏈夌敤銆?
### 涓轰粈涔堢敤 cpio 鑰屼笉鏄?tar锛?
杩欎竴鍐冲畾鏄湪 2001 骞?12 鏈堝仛鍑虹殑銆傝璁哄浜庢澶勶細

- https://lore.kernel.org/lkml/a03cke$640$1@cesium.transmeta.com/

骞剁敱姝ゅ紩鍙戜簡绗簩涓璁轰覆锛堜笓闂ㄥ叧浜?tar 涓?cpio 鐨勫姣旓級锛屽浜庢澶勶細

- https://lore.kernel.org/lkml/3C25a06d.7030408@zytor.com/

绠€鏄庢壖瑕佺殑鎬荤粨鐗堟湰锛堜笉鑳芥浛浠ｉ槄璇讳笂杩拌璁轰覆锛夊涓嬶細

1) cpio 鏄竴涓爣鍑嗐€傚畠宸叉湁鍑犲崄骞村巻鍙诧紙鍙拷婧埌 AT&T 鏃朵唬锛夛紝骞朵笖宸茬粡鍦?Linux 涓婅骞挎硾浣跨敤锛堝湪 RPM銆丷ed Hat 鐨勮澶囬┍鍔ㄧ洏鍐咃級銆傝繖閲屾湁涓€绡?1996 骞村叧浜庡畠鐨?Linux Journal 鏂囩珷锛?
      http://www.linuxjournal.com/article/1213

   瀹冧笉濡?tar 娴佽锛屾槸鍥犱负浼犵粺鐨?cpio 鍛戒护琛屽伐鍏烽渶瑕?_truly_hideous_锛堟瀬鍏朵笐闄嬶級鐨勫懡浠よ鍙傛暟銆備絾杩欏褰掓。鏍煎紡鏈韩鐨勫ソ涓庡潖骞舵棤浠讳綍璇存槑锛岃€屼笖杩樻湁鏇夸唬宸ュ叿锛屼緥濡傦細

      https://linux.die.net/man/1/afio

2) 鍐呮牳鎵€閫夌殑 cpio 褰掓。鏍煎紡姣斾换浣曚竴绉嶏紙鐪熸湁鍑犲崄绉嶏級tar 褰掓。鏍煎紡閮芥洿绠€鍗曘€佹洿骞插噣锛堝洜鑰屼篃鏇村鏄撳垱寤哄拰瑙ｆ瀽锛夈€傚畬鏁寸殑 initramfs 褰掓。鏍煎紡鍦?buffer-format.rst 涓湁璇存槑锛岀敱 usr/gen_init_cpio.c 鐢熸垚锛屽苟鐢?init/initramfs.c 瑙ｅ帇銆備笁鑰呭悎鍦ㄤ竴璧凤紝浜虹被鍙鏂囨湰鎬婚噺涓嶅埌 26k銆?
3) GNU 椤圭洰灏?tar 鏍囧噯鍖栵紝鍏剁浉鍏虫€уぇ绾︾瓑鍚屼簬 Windows 灏?zip 鏍囧噯鍖栥€侺inux 涓嶅睘浜庡叾涓换浣曚竴鏂癸紝鍙互鑷敱鍋氬嚭鑷繁鐨勬妧鏈喅绛栥€?
4) 鏃㈢劧杩欐槸鍐呮牳鍐呴儴鏍煎紡锛屽畠鏈彲浠ヨ交鏄撳湴鏄竴绉嶅叏鏂扮殑涓滆タ銆傛棤璁哄浣曪紝鍐呮牳閮芥彁渚涗簡鑷繁鐨勫伐鍏锋潵鍒涘缓鍜岃В鍘嬭繖绉嶆牸寮忋€備娇鐢ㄧ幇鏈夋爣鍑嗘槸鏇村彲鍙栫殑锛屼絾骞堕潪蹇呰銆?
5) 杩欎竴鍐冲畾鐢?Al Viro 鍋氬嚭锛堝紩鏂囷細"tar is ugly as hell and not going to be supported on the kernel side"锛坱ar 涓戦檵鑷虫瀬锛屽唴鏍镐晶涓嶄細鎻愪緵鏀寔锛夛級锛?
    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112222109050.21702-100000@weyl.math.psu.edu/

   浠栬В閲婁簡鑷繁鐨勭悊鐢憋細

    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112222240530.21702-100000@weyl.math.psu.edu/
    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112230849550.23300-100000@weyl.math.psu.edu/

   骞朵笖锛屾渶閲嶈鐨勬槸锛屼粬璁捐骞跺疄鐜颁簡 initramfs 浠ｇ爜銆?
### 鏈潵鏂瑰悜

濡備粖锛?.6.16锛夛紝initramfs 鎬绘槸琚紪璇戣繘鍐呮牳锛屼絾骞朵笉鎬绘槸琚娇鐢ㄣ€傚唴鏍镐細鍥為€€鍒颁紶缁熷惎鍔ㄤ唬鐮侊紝鑰岃浠ｇ爜鍙湁鍦?initramfs 涓嶅寘鍚?/init 绋嬪簭鏃舵墠浼氳瑙﹀強銆傝繖涓洖閫€浠ｇ爜鏄仐鐣欎唬鐮侊紝鐢ㄤ簬纭繚骞虫粦杩囨浮锛屽苟鍏佽鏃╂湡鍚姩鍔熻兘閫愭杩佺Щ鍒?"early userspace"锛堝嵆 initramfs锛夈€?
鍚?early userspace 杩佺Щ鏄繀瑕佺殑锛屽洜涓烘煡鎵惧苟鎸傝浇鐪熸鐨勬牴璁惧鍗佸垎澶嶆潅銆傛牴鍒嗗尯鍙互璺ㄨ秺澶氫釜璁惧锛坮aid 鎴栫嫭绔嬫棩蹇楋級銆傚畠浠彲浠ヤ綅浜庣綉缁滀笂锛堥渶瑕?dhcp銆佽缃壒瀹?MAC 鍦板潃銆佺櫥褰曟湇鍔″櫒绛夛級銆傚畠浠彲浠ヤ綅浜庡彲绉诲姩浠嬭川涓婏紝甯︽湁鍔ㄦ€佸垎閰嶇殑 major/minor 鍙蜂互鍙婃寔涔呭懡鍚嶉棶棰橈紝闇€瑕佸畬鏁寸殑 udev 瀹炵幇鏉ョ悊椤恒€傚畠浠彲浠ユ槸鍘嬬缉鐨勩€佸姞瀵嗙殑銆佸啓鏃跺鍒剁殑銆乴oopback 鎸傝浇鐨勩€佷互濂囩壒鏂瑰紡鍒嗗尯鐨勶紝绛夌瓑銆?
杩欑被澶嶆潅鎬э紙涓嶅彲閬垮厤鍦板寘鍚瓥鐣ワ級搴斿綋鍦ㄧ敤鎴风┖闂翠腑濡ュ杽澶勭悊銆俴libc 涓?busybox/uClibc 閮藉湪寮€鍙戝彲浠ユ斁鍏ュ唴鏍告瀯寤虹殑绠€鍗?initramfs 杞欢鍖呫€?
klibc 杞欢鍖呯幇鍦ㄥ凡琚帴鍙楄繘鍏?Andrew Morton 鐨?2.6.17-mm 鏍戙€傚唴鏍稿綋鍓嶇殑鏃╂湡鍚姩浠ｇ爜锛堝垎鍖烘娴嬬瓑锛夊緢鍙兘浼氳杩佺Щ鍒颁竴涓粯璁ょ殑 initramfs 涓紝鐢卞唴鏍告瀯寤鸿嚜鍔ㄥ垱寤哄苟浣跨敤銆?