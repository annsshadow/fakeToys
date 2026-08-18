
## 绠€浠嬶紙Introduction锛?


ATA over Ethernet锛圓oE锛屼互澶綉涔嬩笂鐨?ATA锛夋槸涓€绉嶇綉缁滃崗璁紝鎻愪緵瀵?LAN 涓婂潡瀛樺偍鐨勭畝鍗曡闂€?

  http://support.coraid.com/documents/AoEr11.txt

閫傜敤浜?2.6 鍜?3.x 鍐呮牳鐨?EtherDrive(R) HOWTO 浣嶄簬鈥︹€?

  http://support.coraid.com/support/linux/EtherDrive-2.6-HOWTO.html

鍏朵腑鏈夎澶氭妧宸т笌鎻愮ず锛佽鐗瑰埆鍙傝鍏充簬铏氭嫙鍐呭瓨鐨勬帹鑽愯皟浼橈細

  http://support.coraid.com/support/linux/EtherDrive-2.6-HOWTO-5.html#ss5.19

aoetools 鏄笓闂ㄩ厤鍚堟椹卞姩宸ヤ綔鐨勭敤鎴锋€佺▼搴忥紝鍙湪 sourceforge 涓婅幏鍙栥€?

  http://aoetools.sourceforge.net/

鏈?Documentation/admin-guide/aoe 鐩綍涓殑鑴氭湰鏃ㄥ湪璇存槑璇ラ┍鍔ㄧ殑浣跨敤鏂规硶锛涘鏋滀綘瀹夎浜?aoetools锛屽垯骞朵笉闇€瑕佸畠浠€?


## 鍒涘缓璁惧鑺傜偣锛圕reating Device Nodes锛?


  浣跨敤 udev 鐨勭敤鎴峰簲褰撲細鍙戠幇鍧楄澶囪妭鐐逛細琚嚜鍔ㄥ垱寤猴紱浣嗚鍒涘缓鎵€鏈夊繀瑕佺殑璁惧鑺傜偣锛岃浣跨敤鏈洰褰曚腑 udev.txt 鎻愪緵鐨?udev 閰嶇疆瑙勫垯銆?

  鏈変竴涓?udev-install.sh 鑴氭湰锛屾紨绀轰簡濡備綍鍦ㄤ綘鐨勭郴缁熶笂瀹夎杩欎簺瑙勫垯銆?

  杩樻湁涓€涓?autoload锛堣嚜鍔ㄥ姞杞斤級鑴氭湰锛屾紨绀轰簡濡備綍缂栬緫 /etc/modprobe.d/aoe.conf锛屼互纭繚 aoe 妯″潡鍦ㄩ渶瑕佹椂浼氳鍔犺浇銆備笉杩囷紝棰勫姞杞?aoe 妯″潡浼樹簬鑷姩鍔犺浇锛屽洜涓?AoE 鍙戠幇闇€瑕佸嚑绉掗挓鏃堕棿銆傚綋棣栨杩愯 a 鍛戒护鏃?AoE 璁惧灏氫笉瀛樺湪銆佽€屼竴绉掗挓鍚庡畠鍙堝嚭鐜版椂锛屼細浠や汉鍥版儜銆?

## 浣跨敤璁惧鑺傜偣锛圲sing Device Nodes锛?


  "cat /dev/etherd/err" 浼氶樆濉烇紝绛夊緟閿欒璇婃柇杈撳嚭锛屼緥濡傞噸浼犵殑鏁版嵁鍖呫€?

  "echo eth2 eth4 > /dev/etherd/interfaces" 鍛婅瘔 aoe 椹卞姩灏?ATA over Ethernet 娴侀噺闄愬埗鍒?eth2 鍜?eth4銆傚嚭浜庡畨鍏ㄨ€冭檻锛屽簲蹇界暐鏉ヨ嚜涓嶅彲淇＄綉缁滅殑 AoE 娴侀噺銆傚彟璇峰弬瑙佷笅鏂囨弿杩扮殑 aoe_iflist 椹卞姩閫夐」銆?

  "echo > /dev/etherd/discover" 鍛婅瘔椹卞姩鍘诲彂鐜版湁鍝簺 AoE 璁惧鍙敤銆?

  灏嗘潵杩欎簺瀛楃璁惧鍙兘浼氭秷澶憋紝骞惰 sysfs 涓殑瀵瑰簲椤瑰彇浠ｃ€備娇鐢?aoetools 涓殑鍛戒护鍙互灏嗙敤鎴蜂笌杩欎簺瀹炵幇缁嗚妭闅旂寮€鏉ャ€?

```

	e{shelf}.{slot}
	e{shelf}.{slot}p{part}

  鈥︹€﹀洜姝?"e0.2" 琛ㄧず绗竴涓満鏋讹紙shelf 鍦板潃涓?0锛変腑浠庡乏鏁扮涓変釜鍒€鐗囷紙slot 2锛夈€傝繖灏辨槸鏁村潡纾佺洏銆傝纾佺洏鐨勭涓€涓垎鍖哄皢鏄?"e0.2p1"銆?

```
## 浣跨敤 sysfs锛圲sing sysfs锛?


  /sys/block 涓殑姣忎釜 aoe 鍧楄澶囬兘鍏锋湁 state銆乵ac 鍜?netif 绛夐澶栧睘鎬с€傚綋璁惧宸插噯澶囧ソ杩涜 I/O 鏃讹紝state 灞炴€т负 "up"锛涜嫢宸茶妫€娴嬪埌浣嗕笉鍙敤锛屽垯涓?"down"銆?down,closewait" 鐘舵€佽〃绀鸿澶囦粛澶勪簬鎵撳紑鐘舵€侊紝鍦ㄥ叧闂箣鍓嶆棤娉曞啀娆′笂绾裤€?

  mac 灞炴€ф槸杩滅 AoE 璁惧鐨勪互澶綉鍦板潃銆俷etif 灞炴€ф槸鏈湴涓绘満涓婄敤浜庝笌杩滅 AoE 璁惧閫氫俊鐨勭綉缁滄帴鍙ｃ€?

  鏈洰褰曚腑鏈変竴涓剼鏈彲浠ユ柟渚垮湴鏍煎紡鍖栬繖浜涗俊鎭€備娇鐢?aoetools 鐨勭敤鎴峰簲浣跨敤 aoe-stat
```

    root@makki root# sh Documentation/admin-guide/aoe/status.sh
       e10.0            eth3              up
       e10.1            eth3              up
       e10.2            eth3              up
       e10.3            eth3              up
       e10.4            eth3              up
       e10.5            eth3              up
       e10.6            eth3              up
       e10.7            eth3              up
       e10.8            eth3              up
       e10.9            eth3              up
        e4.0            eth1              up
        e4.1            eth1              up
        e4.2            eth1              up
        e4.3            eth1              up
        e4.4            eth1              up
        e4.5            eth1              up
        e4.6            eth1              up
        e4.7            eth1              up
        e4.8            eth1              up
        e4.9            eth1              up

  浣跨敤 /sys/module/aoe/parameters/aoe_iflist锛堟垨鑰呮洿濂藉湴锛屼娇鐢ㄤ笅鏂囪璁虹殑椹卞姩閫夐」锛夎€屼笉鏄?/dev/etherd/interfaces锛屽皢 AoE 娴侀噺闄愬埗鍒扮粰瀹氱┖鐧界鍒嗛殧鍒楄〃涓殑缃戠粶鎺ュ彛銆備笌鏃х殑瀛楃璁惧涓嶅悓锛宻ysfs 椤规棦鍙涔熷彲鍐欍€?

  璁剧疆瀹屽厑璁哥殑鎺ュ彛鍒楄〃鍚庯紝瑙﹀彂鍙戠幇鎿嶄綔鏄湁甯姪鐨勩€俛oetools 杞欢鍖呬负姝ゆ彁渚涗簡 aoe-discover 鑴氭湰銆備綘涔熷彲浠ョ洿鎺ヤ娇鐢ㄤ笂鏂囨弿杩扮殑 /dev/etherd/discover 鐗规畩鏂囦欢銆?

```
## 椹卞姩閫夐」锛圖river Options锛?


  鍐呯疆 aoe 椹卞姩鏈変竴涓惎鍔ㄩ€夐」浠ュ強瀵瑰簲鐨勬ā鍧楀弬鏁?aoe_iflist銆傚鏋滄病鏈夎閫夐」锛屾墍鏈夌綉缁滄帴鍙ｉ兘鍙兘琚敤浜?ATA over Ethernet銆備笅闈㈡槸涓€涓ず渚?
```

    modprobe aoe_iflist="eth1 eth3"

```
  aoe_deadsecs 妯″潡鍙傛暟鍐冲畾椹卞姩绛夊緟 AoE 璁惧瀵?AoE 鍛戒护浣滃嚭鍝嶅簲鐨勬渶澶х鏁般€傜粡杩?aoe_deadsecs 绉掑悗锛岃 AoE 璁惧灏嗚鏍囪涓?"down"銆傚嚭浜庢祴璇曠洰鐨勬敮鎸佸彇鍊间负闆讹紝浼氫娇 aoe 椹卞姩姘歌繙涓嶆柇閲嶈瘯 AoE 鍛戒护銆?

  aoe_maxout 妯″潡鍙傛暟榛樿鍊间负 128銆傝繖鏄竴娆℃€у彂寰€鏌愪釜 AoE 鐩爣鐨勬渶澶ф湭搴旂瓟鏁版嵁鍖呮暟閲忋€?

  aoe_dyndevs 妯″潡鍙傛暟榛樿鍊间负 1锛岃〃绀洪┍鍔ㄤ細鏍规嵁鍙戠幇椤哄簭涓哄彂鐜扮殑 AoE 鐩爣鍒嗛厤鍧楄澶囨璁惧鍙枫€傚湪浣跨敤鍔ㄦ€佹璁惧鍙风殑鎯呭喌涓嬶紝鍙互鏀寔鏇村ぇ鑼冨洿鐨?AoE 鏈烘灦涓庢Ы浣嶅湴鍧€銆備娇鐢?udev 鐨勭敤鎴锋案杩滄棤闇€鍏冲績娆¤澶囧彿銆備娇鐢?aoe_dyndevs=0 鍒欏厑璁镐娇鐢?aoetools 涓殑 aoe-mkshelf 鑴氭湰銆侀€氳繃闈欐€佹璁惧鍙锋柟妗堥鍏堝垱寤鸿澶囪妭鐐广€?
