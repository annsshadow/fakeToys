
## 閫忔槑浠ｇ悊鏀寔


姝ょ壒鎬т负褰撳墠鍐呮牳娣诲姞浜嗙被浼?Linux 2.2 鐨勯€忔槑浠ｇ悊鏀寔銆傝浣跨敤璇ョ壒鎬э紝璇峰湪浣犵殑鍐呮牳閰嶇疆涓?鍚敤 socket 鍖归厤涓?TPROXY 鐩爣銆備綘杩橀渶瑕佺瓥鐣ヨ矾鐢憋紝鍥犳璇峰姟蹇呬篃鍚敤瀹冦€?
浠?Linux 4.18 璧凤紝nf_tables 涓篃鎻愪緵浜嗛€忔槑浠ｇ悊鏀寔銆?
## 1. 璁╅潪鏈湴濂楁帴瀛楀伐浣?

鍏舵€濊矾鏄細浣犻€氳繃绛栫暐璺敱璇嗗埆鐩殑鍦板潃鍖归厤鏈湴鏌愪釜鍦板潃鐨勬暟鎹寘锛屼粠鑰岃杩欎簺鏁版嵁鍖?```

    # iptables -t mangle -N DIVERT
    # iptables -t mangle -A PREROUTING -p tcp -m socket --transparent -j DIVERT
    # iptables -t mangle -A DIVERT -j MARK --set-mark 1
    # iptables -t mangle -A DIVERT -j ACCEPT

```
```

    # nft add table filter
    # nft add chain filter divert "{ type filter hook prerouting priority -150; }"
    # nft add rule filter divert meta l4proto tcp socket transparent 1 meta mark set 1 accept

```
鐒跺悗閫氳繃绛栫暐璺敱鍖归厤璇ュ€硷紝浣块偅浜涙暟鎹寘
```

    # ip rule add fwmark 1 lookup 100
    # ip route add local 0.0.0.0/0 dev lo table 100

```
鐢变簬 IPv4 璺敱杈撳嚭浠ｇ爜鐨勬煇浜涢檺鍒讹紝浣犲皢涓嶅緱涓嶄慨鏀逛綘鐨勫簲鐢ㄧ▼搴忥紝浠ュ厑璁稿畠_浠巁闈炴湰鍦?IP
鍦板潃鍙戦€佹暟鎹姤銆備綘鍙渶鍚敤锛圫OL_IP, IP_TRANSPARENT锛夊鎺ュ瓧閫夐」
```

    fd = socket(AF_INET, SOCK_STREAM, 0);
    /* - 8< -*/
    int value = 1;
    setsockopt(fd, SOL_IP, IP_TRANSPARENT, &value, sizeof(value));
    /* - 8< -*/
    name.sin_family = AF_INET;
    name.sin_port = htons(0xCAFE);
    name.sin_addr.s_addr = htonl(0xDEADBEEF);
    bind(fd, &name, sizeof(name));

```
netcat 鐨勪竴涓畝鍗曡ˉ涓佸彲鍦ㄦ澶勮幏鍙栵細
http://people.netfilter.org/hidden/tproxy/netcat-ip_transparent-support.patch

## 2. 閲嶅畾鍚戞祦閲?

閫忔槑浠ｇ悊閫氬父娑夊強鍦ㄨ矾鐢卞櫒涓娾€滄嫤鎴€濇祦閲忋€傝繖閫氬父閫氳繃 iptables 鐨?REDIRECT 鐩爣瀹屾垚锛涚劧鑰岋紝
璇ユ柟娉曞瓨鍦ㄤ弗閲嶅眬闄愩€傚叾涓竴涓富瑕侀棶棰樻槸锛屽畠瀹為檯涓婁細淇敼鏁版嵁鍖呬互鏀瑰彉鐩殑鍦板潃鈥斺€旇繖鍦ㄦ煇浜?鎯呭喌涓嬪彲鑳戒笉鍙帴鍙椼€傦紙渚嬪鎯虫兂浠ｇ悊 UDP锛氫綘灏嗘棤娉曡幏鐭ュ師濮嬬洰鐨勫湴鍧€銆傚嵆渚垮浜?TCP锛岃幏鍙?鍘熷鐩殑鍦板潃涔熷瓨鍦ㄧ珵浜夋潯浠躲€傦級

'TPROXY' 鐩爣鎻愪緵浜嗙被浼肩殑鍔熻兘锛屼笖涓嶄緷璧?NAT銆傚彧闇€
```

    # iptables -t mangle -A PREROUTING -p tcp --dport 80 -j TPROXY \
      --tproxy-mark 0x1/0x1 --on-port 50080

```
```

    # nft add rule filter divert tcp dport 80 tproxy to :50080 meta mark set 1 accept

```
娉ㄦ剰锛岃浣垮叾宸ヤ綔锛屼綘蹇呴』淇敼浠ｇ悊锛屼负鐩戝惉濂楁帴瀛楀惎鐢紙SOL_IP, IP_TRANSPARENT锛夈€?
浣滀负绀轰緥瀹炵幇锛宼cprdr 鍙湪姝ゅ鑾峰彇锛?https://git.breakpoint.cc/cgit/fw/tcprdr.git/
璇ュ伐鍏风敱 Florian Westphal 缂栧啓锛屽苟鍦?nf_tables 瀹炵幇鏈熼棿鐢ㄤ簬娴嬭瘯銆?
## 3. Iptables 涓?nf_tables 鎵╁睍


瑕佷娇鐢?tproxy锛屼綘闇€瑕佷负 iptables 缂栬瘧浠ヤ笅妯″潡锛?
 - NETFILTER_XT_MATCH_SOCKET
 - NETFILTER_XT_TARGET_TPROXY

鎴栦负 nf_tables 缂栬瘧浠ヤ笅妯″潡锛?
 - NFT_SOCKET
 - NFT_TPROXY

## 4. 搴旂敤绋嬪簭鏀寔


### 4.1. Squid


Squid 3.HEAD 宸插唴缃敮鎸併€傝浣跨敤瀹冿紝璇峰皢 '--enable-linux-netfilter' 浼犵粰 configure锛?骞跺湪浣犻€氳繃 TPROXY iptables 鐩爣閲嶅畾鍚戞祦閲忓埌鐨?HTTP 鐩戝惉鍣ㄤ笂璁剧疆 'tproxy' 閫夐」銆?
鏇村淇℃伅璇锋煡闃?Squid wiki 涓婄殑浠ヤ笅椤甸潰锛歨ttp://wiki.squid-cache.org/Features/Tproxy4
