## How to use dm-crypt and swsusp together


浣滆€咃細Andreas Steinmetz <ast@domdv.de>



涓€浜涘厛鍐虫潯浠讹細
浣犱簡瑙?dm-crypt 鐨勫伐浣滃師鐞嗐€傚鏋滀笉浜嗚В锛岃璁块棶浠ヤ笅缃戦〉锛?
http://www.saout.de/misc/dm-crypt/
浣犲凡闃呰 Documentation/power/swsusp.rst 骞剁悊瑙ｄ簡瀹冦€?
浣犵‘瀹為槄璇昏繃 Documentation/admin-guide/initrd.rst 骞朵簡瑙?initrd 鐨勫伐浣滃師鐞嗐€?
浣犵煡閬撳浣曞垱寤烘垨淇敼涓€涓?initrd銆?

鐜板湪浣犵殑绯荤粺宸叉纭缃紝闄や簡浜ゆ崲璁惧浠ュ強鍙兘鍖呭惈鐢ㄤ簬鍔犲瘑璁剧疆鍜?鎴栨晳鎻寸敤閫旂殑
杩蜂綘绯荤粺鐨勫紩瀵煎垎鍖哄锛屼綘鐨勭鐩樺凡鍔犲瘑銆備綘鐢氳嚦鍙兘宸茬粡鏈変竴涓細瀹屾垚褰撳墠鍔犲瘑璁剧疆鐨?initrd銆?

姝ゆ椂浣犱篃鎯冲姞瀵嗕綘鐨勪氦鎹㈠垎鍖恒€傚悓鏃朵綘浠嶅笇鏈涜兘澶熼€氳繃 swsusp 鎸傝捣銆傜劧鑰岋紝杩欐剰鍛崇潃浣?
蹇呴』鑳藉鍦ㄦ仮澶嶄箣鍓嶈緭鍏ュ彛浠わ紝鎴栬€呬粠澶栭儴璁惧锛堝 pcmcia 闂瓨鐩樻垨 usb 瀛樺偍妫掞級璇诲彇瀵嗛挜銆?
鍥犳浣犻渶瑕佷竴涓?initrd锛屽畠鍏堣缃?dm-crypt锛岀劧鍚庤 swsusp 浠庡姞瀵嗙殑浜ゆ崲璁惧鎭㈠銆?

鏈€閲嶈鐨勪竴鐐规槸锛屼綘璁剧疆 dm-crypt 鐨勬柟寮忓繀椤讳娇寰椾綘鎸傝捣/鎭㈠鍒扮殑浜ゆ崲璁惧鍦?initrd 鍐?
浠ュ強杩愯涓殑绯荤粺鍐呭缁堝叿鏈夌浉鍚岀殑涓?娆¤澶囧彿銆傛渶绠€鍗曠殑瀹炵幇鏂瑰紡鏄缁堥鍏堢敤 dmsetup
璁剧疆璇ヤ氦鎹㈣澶囷紝杩欐牱
```

  brw-------  1 root root 254, 0 Jul 28 13:37 /dev/mapper/swap0

```
鐜板湪灏嗕綘鐨勫唴鏍歌缃负浣跨敤 /dev/mapper/swap0 浣滀负榛樿鐨?
```

  CONFIG_PM_STD_PARTITION="/dev/mapper/swap0"

```
鍑嗗濂戒綘鐨勫紩瀵煎姞杞界▼搴忎互浣跨敤浣犲皢鍒涘缓鎴栦慨鏀圭殑 initrd銆傚浜?lilo锛屾渶绠€鍗曠殑璁剧疆濡備笅鎵€绀?
```

  image=/boot/vmlinuz
  initrd=/boot/initrd.gz
  label=linux
  append="root=/dev/ram0 init=/linuxrc rw"

```
鏈€鍚庝綘闇€瑕佸垱寤烘垨淇敼浣犵殑 initrd銆傚亣璁句綘瑕佸垱寤轰竴涓粠 pcmcia 闂瓨鍗¤鍙栨墍闇€ dm-crypt
璁剧疆鐨?initrd銆傝鍗℃牸寮忓寲涓?ext2 鏂囦欢绯荤粺锛屾彃鍏ユ椂浣嶄簬 /dev/hde1銆傝鍗¤嚦灏戝寘鍚竴涓悕涓?
鈥渟wapkey鈥濈殑鏂囦欢锛屽叾涓瓨鏀剧潃鍔犲瘑鐨勪氦鎹㈣缃€備綘 initrd 鐨?/etc/fstab 涓惈鏈夌被浼煎涓嬪唴瀹?
```

  /dev/hda1   /mnt    ext3      ro                            0 0
  none        /proc   proc      defaults,noatime,nodiratime   0 0
  none        /sys    sysfs     defaults,noatime,nodiratime   0 0

```
/dev/hda1 鍖呭惈涓€涓湭鍔犲瘑鐨勮糠浣犵郴缁燂紝瀹冨悓鏍烽€氳繃浠?pcmcia 闂瓨鐩樿鍙栬缃潵閰嶇疆浣犳墍鏈夌殑
鍔犲瘑璁惧銆備互涓嬫槸浣犵殑 initrd 鐨勪竴涓?/linuxrc锛屽畠鍏佽浣犱粠鍔犲瘑浜ゆ崲鎭㈠锛屽苟鍦ㄦ仮澶嶅け璐ユ椂
缁х画鐢?/dev/hda1 涓婄殑杩蜂綘绯荤粺寮曞
```

  #!/bin/sh
  PATH=/sbin:/bin:/usr/sbin:/usr/bin
  mount /proc
  mount /sys
  mapped=0
  noresume=`grep -c noresume /proc/cmdline`
  if [ "$*" != "" ]
  then
    noresume=1
  fi
  dmesg -n 1
  /sbin/cardmgr -q
  for i in 1 2 3 4 5 6 7 8 9 0
  do
    if [ -f /proc/ide/hde/media ]
    then
      usleep 500000
      mount -t ext2 -o ro /dev/hde1 /mnt
      if [ -f /mnt/swapkey ]
      then
        dmsetup create swap0 /mnt/swapkey > /dev/null 2>&1 && mapped=1
      fi
      umount /mnt
      break
    fi
    usleep 500000
  done
  killproc /sbin/cardmgr
  dmesg -n 6
  if [ $mapped = 1 ]
  then
    if [ $noresume != 0 ]
    then
      mkswap /dev/mapper/swap0 > /dev/null 2>&1
    fi
    echo 254:0 > /sys/power/resume
    dmsetup remove swap0
  fi
  umount /sys
  mount /mnt
  umount /proc
  cd /mnt
  pivot_root . mnt
  mount /proc
  umount -l /mnt
  umount /proc
  exec chroot . /sbin/init $* < dev/console > dev/console 2>&1

```
璇蜂笉瑕佷粙鎰忎笂闈㈣繖涓鎬殑寰幆锛宐usybox 鐨?msh 涓嶈璇?let 璇彞銆傞偅涔堬紝杩欎釜鑴氭湰閲屽彂鐢熶簡浠€涔堬紵
棣栧厛鎴戜滑蹇呴』鍐冲畾鏄惁瑕佸皾璇曟仮澶嶃€傚鏋滄垜浠互鈥渘oresume鈥濇垨浠讳綍缁?init 鐨勫弬鏁帮紙濡傗€渟ingle鈥?
鎴栤€渆mergency鈥濓級浣滀负寮曞鍙傛暟鍚姩锛屾垜浠皢涓嶆仮澶嶃€?

鐒跺悗鎴戜滑闇€瑕佺敤鏉ヨ嚜 pcmcia 闂瓨鐩樼殑璁剧疆鏁版嵁璁剧疆 dmcrypt銆傚鏋滄垚鍔燂紝涓旀垜浠笉鎯虫仮澶嶏紝
鍒欓渶瑕侀噸缃氦鎹㈣澶囥€傞殢鍚庘€渆cho 254:0 > /sys/power/resume鈥濊繖涓€琛屽皾璇曚粠绗竴涓澶囨槧灏?
璁惧鎭㈠銆傛敞鎰忥紝鏃犺鏄惁鎭㈠锛屽湪 /sys/power/resume 涓缃澶囬兘寰堥噸瑕侊紝鍚﹀垯鍚庣画鎸傝捣浼氬け璐ャ€?
濡傛灉鎭㈠寮€濮嬶紝鑴氭湰鎵ц鍒版缁堟銆?

鍚﹀垯鎴戜滑鍙槸绉婚櫎鍔犲瘑鐨勪氦鎹㈣澶囷紝骞跺皢鍏剁暀缁?/dev/hda1 涓婄殑杩蜂綘绯荤粺鏉ュ畬鎴愭暣涓姞瀵嗙殑璁剧疆
锛堜綘鍙互鏍规嵁闇€瑕佽嚜琛屼慨鏀癸級銆?

鎺ヤ笅鏉ュ氨鏄紬鎵€鍛ㄧ煡鐨勫垏鎹㈡牴鏂囦欢绯荤粺骞朵粠涓户缁紩瀵肩殑杩囩▼銆傛垜鍊惧悜浜庡湪缁х画寮曞涔嬪墠鍗歌浇
initrd锛屼絾杩欑敱浣犺嚜琛屼慨鏀广€?

