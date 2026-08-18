
## Linux 涓嬬殑 BFS 鏂囦欢绯荤粺


BFS 鏂囦欢绯荤粺琚?SCO UnixWare 鎿嶄綔绯荤粺鐢ㄤ簬 /stand 鍒囩墖锛坰lice锛夛紝璇ュ垏鐗囬€氬父鍖呭惈鍐呮牳闀滃儚浠ュ強鍚姩杩囩▼鎵€闇€鐨勫皯鏁板叾浠栨枃浠躲€?
涓轰簡鍦?Linux 涓嬭闂?/stand 鍒嗗尯锛屼綘鏄剧劧闇€瑕佺煡閬撳垎鍖哄彿锛屽苟涓斿唴鏍稿繀椤绘敮鎸?UnixWare 纾佺洏鍒囩墖锛圕ONFIG_UNIXWARE_DISKLABEL 閰嶇疆閫夐」锛夈€備笉杩?BFS 鏀寔骞朵笉渚濊禆浜?UnixWare 纾佺洏鏍囩鏀寔锛屽洜涓轰篃鍙互鎸傝浇
```

    # losetup /dev/loop0 stand.img
    # mount -t bfs /dev/loop0 /mnt/stand

```
鍏朵腑 stand.img 鏄寘鍚?BFS 鏂囦欢绯荤粺闀滃儚鐨勬枃浠躲€傚綋浣犱娇鐢ㄥ畬姣曞苟鍗歌浇鍚庯紝杩橀渶瑕侀噴鏀?```

    # losetup -d /dev/loop0

```

```
    # mount -t bfs -o loop stand.img /mnt/stand

```
杩欏皢鑷姩鍒嗛厤绗竴涓彲鐢ㄧ殑鍥炵幆璁惧锛堝苟鍦ㄥ繀瑕佹椂鍔犺浇 loop.o 鍐呮牳妯″潡锛夈€傚鏋滃洖鐜┍鍔ㄦ病鏈夎鑷姩鍔犺浇锛岃纭繚浣犲凡缁忕紪璇戜簡璇ユā鍧楀苟涓?modprobe 宸ヤ綔姝ｅ父銆傛敞鎰忥紝濡傛灉浣犵殑绯荤粺涓?/etc/mtab 鏂囦欢鏄埌 /proc/mounts 鐨勭鍙烽摼鎺ワ紝閭ｄ箞 umount 涓嶄細閲婃斁 /dev/loopN 璁惧銆備綘闇€瑕佷娇鐢?losetup(8) 鐨?"-d" 寮€鍏虫墜鍔ㄥ畬鎴愩€傛洿澶氫俊鎭闃呰 losetup(8) 鎵嬪唽椤点€?
瑕佸湪 UnixWare 涓嬪垱寤?BFS 闀滃儚锛屼綘棣栧厛闇€瑕佹壘鍑?```

    # prtvtoc /dev/rdsk/c0b0t0d0s0

```
锛堝亣璁句綘鐨勬牴纾佺洏浣嶄簬 target=0銆乴un=0銆乥us=0銆乧ontroller=0锛夈€傜劧鍚庝綘瀵绘壘鏍囪涓?"STAND" 鐨勫垏鐗囷紝閫氬父灏辨槸鍒囩墖 10銆傛湁浜嗗畠涔嬪悗
```

    # umount /stand
    # dd if=/dev/rdsk/c0b0t0d0sa of=stand.img bs=512

```
浠ラ槻涓囦竴锛屼綘鍙互閫氳繃妫€鏌ヤ互涓嬪唴瀹规潵楠岃瘉浣犲仛瀵逛簡
```

    # od -Ad -tx4 stand.img | more

```
鍓?4 涓瓧鑺傚簲璇ユ槸 0x1badface銆?
濡傛灉浣犲杩欎釜 BFS 瀹炵幇鏈変换浣曡ˉ涓併€侀棶棰樻垨寤鸿锛岃鑱旂郴浣滆€咃細

Tigran Aivazian <aivazian.tigran@gmail.com>
