
## SCTP


## SCTP LSM 鏀寔


### 瀹夊叏閽╁瓙锛圫ecurity Hooks锛?

```

    security_sctp_assoc_request()
    security_sctp_bind_connect()
    security_sctp_sk_clone()
    security_sctp_assoc_established()

```

杩欎簺閽╁瓙鐨勭敤娉曞湪 `SCTP SELinux Support`_ 绔犺妭涓互 SELinux 瀹炵幇涓轰緥杩涜浜嗘弿杩般€?
#### security_sctp_assoc_request()


灏嗗叧鑱旓紙association锛塈NIT 鍖呯殑 `@asoc` 鍜?`@chunk->skb` 浼犻€掔粰瀹夊叏妯″潡銆傛垚鍔熸椂杩斿洖 0锛屽け璐?鏃惰繑鍥為敊璇€?```

    @asoc - 鎸囧悜 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @skb - 鎸囧悜鍏宠仈鍖?skbuff 鐨勬寚閽堛€?

```

#### security_sctp_bind_connect()


鍩轰簬 `@optname` 灏嗕竴涓垨澶氫釜 ipv4/ipv6 鍦板潃浼犻€掔粰瀹夊叏妯″潡杩涜楠岃瘉锛屽叾缁撴灉灏嗘槸 bind 鎴?connect
鏈嶅姟锛屽涓嬮潰鐨勬潈闄愭鏌ヨ〃鎵€绀恒€傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖閿欒銆?```

    @sk      - 鎸囧悜 sock 缁撴瀯浣撶殑鎸囬拡銆?    @optname - 瑕侀獙璇佺殑閫夐」鍚嶇О銆?    @address - 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃銆?    @addrlen - 鍦板潃鐨勬€婚暱搴︺€傚姣忎釜 ipv4 鎴?ipv6 鍦板潃浣跨敤 sizeof(struct sockaddr_in) 鎴?               sizeof(struct sockaddr_in6) 璁＄畻銆?
  ------------------------------------------------------------------
  |                     BIND 绫诲瀷妫€鏌?                             |
  |       @optname             |         @address 鍖呭惈            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_BINDX_ADD     | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_PRIMARY_ADDR          | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  | SCTP_SET_PEER_PRIMARY_ADDR | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  ------------------------------------------------------------------

  ------------------------------------------------------------------
  |                   CONNECT 绫诲瀷妫€鏌?                            |
  |       @optname             |         @address 鍖呭惈            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_CONNECTX      | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_PARAM_ADD_IP          | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_SENDMSG_CONNECT       | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  | SCTP_PARAM_SET_PRIMARY     | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  ------------------------------------------------------------------

```

```

    SCTP_SOCKOPT_BINDX_ADD - 鍏佽鍦紙鍙€夛級璋冪敤 bind(3) 涔嬪悗鍏宠仈棰濆鐨勭粦瀹氬湴鍧€銆?                             sctp_bindx(3) 鍦ㄥ鎺ュ瓧涓婃坊鍔犱竴缁勭粦瀹氬湴鍧€銆?
    SCTP_SOCKOPT_CONNECTX - 鍏佽鍒嗛厤澶氫釜鍦板潃浠ュ埌杈惧绛夌锛堝瀹夸富锛宮ulti-homed锛夈€?                            sctp_connectx(3) 浣跨敤澶氫釜鐩爣鍦板潃鍦?SCTP 濂楁帴瀛椾笂鍙戣捣杩炴帴銆?
    SCTP_SENDMSG_CONNECT  - 鍙戣捣鐢?sendmsg(2) 鎴?sctp_sendmsg(3) 鍦ㄦ柊鍏宠仈涓婄敓鎴愮殑杩炴帴銆?
    SCTP_PRIMARY_ADDR     - 璁剧疆鏈湴涓诲湴鍧€銆?
    SCTP_SET_PEER_PRIMARY_ADDR - 璇锋眰瀵圭瓑绔皢鍦板潃璁句负鍏宠仈鐨勪富鍦板潃銆?
    SCTP_PARAM_ADD_IP          - 鍚敤鍔ㄦ€佸湴鍧€閲嶉厤缃椂浣跨敤锛屽涓嬫墍杩般€?    SCTP_PARAM_SET_PRIMARY     -

```

涓轰簡鏀寔鍔ㄦ€佸湴鍧€閲嶉厤缃紝蹇呴』璁剧疆浠ヤ笅鍙傛暟
```

    /proc/sys/net/sctp/addip_enable
    /proc/sys/net/sctp/addip_noauth_enable

```

鐒跺悗锛屽湪鍚敤鍔ㄦ€佸湴鍧€閲嶉厤缃椂锛屼互涓?**_PARAM_** 浼氳鍙戦€佺粰瀵圭瓑绔?```

          @optname                      ASCONF 鍙傛暟
         ----------                    ------------------
    SCTP_SOCKOPT_BINDX_ADD     ->   SCTP_PARAM_ADD_IP
    SCTP_SET_PEER_PRIMARY_ADDR ->   SCTP_PARAM_SET_PRIMARY


```

#### security_sctp_sk_clone()


姣忓綋閫氳繃 **accept**\(2)锛堝嵆 TCP 椋庢牸濂楁帴瀛楋級鍒涘缓鏂板鎺ュ瓧锛屾垨濂楁帴瀛楄鈥滃墺绂伙紙peeled off锛夆€?锛堜緥濡傜敤鎴风┖闂磋皟鐢?**sctp_peeloff**\(3)锛夋椂璋冪敤銆?```

    @asoc - 鎸囧悜褰撳墠 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @sk - 鎸囧悜褰撳墠 sock 缁撴瀯浣撶殑鎸囬拡銆?    @newsk - 鎸囧悜鏂?sock 缁撴瀯浣撶殑鎸囬拡銆?

```

#### security_sctp_assoc_established()


褰撴敹鍒?COOKIE ACK 鏃惰皟鐢紝瀵圭瓑绔?secid 灏?```

    @asoc - 鎸囧悜 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @skb - 鎸囧悜 COOKIE ACK 鍖呯殑 skbuff 鐨勬寚閽堛€?

```

### 鐢ㄤ簬鍏宠仈寤虹珛鐨勫畨鍏ㄩ挬瀛?

涓嬪浘灞曠ず浜嗗湪寤虹珛鍏宠仈鏃朵娇鐢?`security_sctp_bind_connect()`銆乣security_sctp_assoc_request()`銆?`security_sctp_assoc_established()` 鐨勬儏鍐点€?```

      SCTP 绔偣 "A"                                SCTP 绔偣 "Z"
      =================                                =================
    sctp_sf_do_prm_asoc()
 鍏宠仈寤虹珛鍙敱
 connect(2)銆乻ctp_connectx(3)銆? sendmsg(2) 鎴?sctp_sendmsg(3) 鍙戣捣銆? 杩欎簺灏嗗鑷村
 security_sctp_bind_connect() 鐨勮皟鐢紝浠? 鍚?SCTP 瀵圭瓑绔?"Z" 鍙戣捣鍏宠仈銆?         INIT --------------------------------------------->
                                                   sctp_sf_do_5_1B_init()
                                                 鍝嶅簲涓€涓?INIT 鍧椼€?                                             SCTP 瀵圭瓑绔?"A" 姝ｅ湪璇锋眰
                                             涓€涓复鏃跺叧鑱斻€?                                             璋冪敤 security_sctp_assoc_request()
                                             浠ヨ缃绛夌鏍囩锛堣嫢鏄娆?                                             鍏宠仈锛夈€?                                             鑻ヤ笉鏄娆″叧鑱旓紝鍒欐鏌?                                             鏄惁鍏佽锛岃嫢鍏佽鍒欏彂閫侊細
          <----------------------------------------------- INIT ACK
          |                                  鍚﹀垯璁板綍瀹¤浜嬩欢骞堕潤榛?          |                                       涓㈠純璇ュ寘銆?          |
    COOKIE ECHO ------------------------------------------>
                                                  sctp_sf_do_5_1D_ce()
                                             鍝嶅簲涓€涓?COOKIE ECHO 鍧椼€?                                             纭 cookie 骞跺垱寤轰竴涓?                                             姘镐箙鍏宠仈銆?                                             璋冪敤 security_sctp_assoc_request() 浠?                                             鎵ц涓?INIT 鍧楀搷搴旂浉鍚岀殑鎿嶄綔銆?          <------------------------------------------- COOKIE ACK
          |                                               |
    sctp_sf_do_5_1E_ca                                    |
 璋冪敤 security_sctp_assoc_established()                   |
 浠ヨ缃绛夌鏍囩銆?                                      |
          |                                               |
          |                              鑻?SCTP_SOCKET_TCP 鎴栬鍓ョ鐨?          |                              濂楁帴瀛楋紝鍒欒皟鐢?security_sctp_sk_clone()
          |                              浠ュ厠闅嗘柊濂楁帴瀛椼€?          |                                               |
      ESTABLISHED                                    ESTABLISHED
          |                                               |
    ------------------------------------------------------------------
    |                     鍏宠仈宸插缓绔?                               |
    ------------------------------------------------------------------


```

## SCTP SELinux 鏀寔


### 瀹夊叏閽╁瓙锛圫ecurity Hooks锛?

涓婇潰鐨?`SCTP LSM Support`_ 绔犺妭鎻忚堪浜嗕互涓?SCTP 瀹夊叏
```

    security_sctp_assoc_request()
    security_sctp_bind_connect()
    security_sctp_sk_clone()
    security_sctp_assoc_established()


```

#### security_sctp_assoc_request()


灏嗗叧鑱?INIT 鍖呯殑 `@asoc` 鍜?`@chunk->skb` 浼犻€掔粰瀹夊叏妯″潡銆傛垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖閿欒銆?```

    @asoc - 鎸囧悜 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @skb - 鎸囧悜鍏宠仈鍖?skbuff 鐨勬寚閽堛€?
```

瀹夊叏妯″潡鎵ц浠ヤ笅鎿嶄綔锛?     濡傛灉璇ュ叧鑱旀槸 `@asoc->base.sk` 涓婄殑绗竴涓叧鑱旓紝鍒欏皢 peer sid 璁句负 `@skb` 涓殑鍊笺€傝繖灏?     纭繚鍙湁涓€涓?peer sid 琚垎閰嶇粰 `@asoc->base.sk`锛屽畠鍙兘鏀寔澶氫釜鍏宠仈銆?
     鍚﹀垯锛屾牴鎹?`@skb peer sid` 楠岃瘉 `@asoc->base.sk peer_sid`锛屼互纭畾鏄惁鍏佽璇ュ叧鑱斻€?
     灏?sctp `@asoc sid` 璁句负濂楁帴瀛楃殑 sid锛堟潵鑷?`asoc->base.sk`锛夛紝MLS 閮ㄥ垎鍙栬嚜 `@skb peer sid`銆?     杩欏皢琚?SCTP TCP 椋庢牸濂楁帴瀛楀拰鍓ョ鐨勮繛鎺ヤ娇鐢紝鍥犱负瀹冧滑浼氱敓鎴愪竴涓柊鐨勫鎺ュ瓧銆?
     濡傛灉閰嶇疆浜?IP 瀹夊叏閫夐」锛圕IPSO/CALIPSO锛夛紝鍒欏湪璇ュ鎺ュ瓧涓婅缃?ip 閫夐」銆?
#### security_sctp_bind_connect()


鍩轰簬 `@optname` 妫€鏌?ipv4/ipv6 鍦板潃鎵€闇€鐨勬潈闄?```

  ------------------------------------------------------------------
  |                   BIND 鏉冮檺妫€鏌?                               |
  |       @optname             |         @address 鍖呭惈            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_BINDX_ADD     | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_PRIMARY_ADDR          | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  | SCTP_SET_PEER_PRIMARY_ADDR | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  ------------------------------------------------------------------

  ------------------------------------------------------------------
  |                 CONNECT 鏉冮檺妫€鏌?                              |
  |       @optname             |         @address 鍖呭惈            |
  |----------------------------|-----------------------------------|
  | SCTP_SOCKOPT_CONNECTX      | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_PARAM_ADD_IP          | 涓€涓垨澶氫釜 ipv4 / ipv6 鍦板潃       |
  | SCTP_SENDMSG_CONNECT       | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  | SCTP_PARAM_SET_PRIMARY     | 鍗曚釜 ipv4 鎴?ipv6 鍦板潃            |
  ------------------------------------------------------------------


```

`SCTP LSM Support`_ 缁欏嚭浜?`@optname` 鏉＄洰鐨勬憳瑕侊紝骞舵弿杩颁簡鍚敤鍔ㄦ€佸湴鍧€閲嶉厤缃椂鐨?ASCONF 鍧?澶勭悊銆?
#### security_sctp_sk_clone()


姣忓綋閫氳繃 **accept**\(2)锛堝嵆 TCP 椋庢牸濂楁帴瀛楋級鍒涘缓鏂板鎺ュ瓧锛屾垨濂楁帴瀛楄鈥滃墺绂伙紙peeled off锛夆€?锛堜緥濡傜敤鎴风┖闂磋皟鐢?**sctp_peeloff**\(3)锛夋椂璋冪敤銆俙security_sctp_sk_clone()` 浼氬皢鏂板鎺ュ瓧鐨?sid 鍜?peer sid 鍒嗗埆璁句负 `@asoc sid` 鍜?`@asoc peer sid` 涓寘鍚殑鍊笺€?```

    @asoc - 鎸囧悜褰撳墠 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @sk - 鎸囧悜褰撳墠 sock 缁撴瀯浣撶殑鎸囬拡銆?    @newsk - 鎸囧悜鏂?sock 缁撴瀯浣撶殑鎸囬拡銆?

```

#### security_sctp_assoc_established()


褰撴敹鍒?COOKIE ACK 鏃惰皟鐢紝姝ゆ椂璁剧疆杩炴帴鐨?peer sid
```

    @asoc - 鎸囧悜 sctp 鍏宠仈缁撴瀯浣撶殑鎸囬拡銆?    @skb - 鎸囧悜 COOKIE ACK 鍖呯殑 skbuff 鐨勬寚閽堛€?

```

### 绛栫暐璇彞锛圥olicy Statements锛?

鏀寔 SCTP 鐨勪互涓嬬被鍜屾潈闄愬湪
```

    class sctp_socket inherits socket { node_bind }

```

```

    policycap extended_socket_class;

```

SELinux 鐨?SCTP 鏀寔澧炲姞浜嗙敤浜庤繛鎺ュ埌鐗瑰畾绔彛绫诲瀷鐨?`name_connect` 鏉冮檺锛屼互鍙婁笅鏂囨墍杩扮珷鑺?瑙ｉ噴鐨?`association` 鏉冮檺銆?
濡傛灉鐢ㄦ埛绌洪棿宸ュ叿宸叉洿鏂帮紝SCTP 灏嗘敮鎸?`portcon`
```

    portcon sctp 1024-1036 system_u:object_r:sctp_ports_t:s0


```

### SCTP 瀵圭瓑绔爣绛撅紙Peer Labeling锛?

涓€涓?SCTP 濂楁帴瀛楀皢鍙垎閰嶄竴涓绛夌鏍囩銆傝鏍囩鍦ㄥ缓绔嬬涓€涓叧鑱旀椂鍒嗛厤銆傝濂楁帴瀛椾笂鐨勪换浣?杩涗竴姝ュ叧鑱旓紝鍏跺寘鐨勫绛夌鏍囩閮藉皢涓庡鎺ュ瓧鐨勫绛夌鏍囩姣旇緝锛屽彧鏈夊綋瀹冧滑涓嶅悓鏃讹紝鎵嶄細楠岃瘉
`association` 鏉冮檺銆傝繖鏄€氳繃妫€鏌ュ鎺ュ瓧 peer sid 涓庢敹鍒扮殑鍖?peer sid 鏉ョ‘瀹氭槸鍚﹀厑璁歌鍏宠仈銆?
娉ㄦ剰锛?   1) 濡傛灉鏈惎鐢ㄥ绛夌鏍囩锛屽垯 peer 涓婁笅鏂囧皢濮嬬粓涓?`SECINITSID_UNLABELED`
      锛堝弬鑰冪瓥鐣ヤ腑鐨?`unlabeled_t`锛夈€?
   2) 鐢变簬 SCTP 鍦ㄥ崟涓鎺ュ瓧涓婂彲浠ユ敮鎸佹瘡涓鐐瑰涓紶杈撳湴鍧€锛堝瀹夸富锛宮ulti-homing锛夛紝鍙互
      閰嶇疆绛栫暐涓?NetLabel 涓烘瘡涓紶杈撳湴鍧€鎻愪緵涓嶅悓鐨勫绛夌鏍囩銆傜敱浜庡鎺ュ瓧 peer 鏍囩鐢辩涓€涓?      鍏宠仈鐨勪紶杈撳湴鍧€鍐冲畾锛屽缓璁墍鏈夊绛夌鏍囩淇濇寔涓€鑷淬€?
   3) **getpeercon**\(3) 鍙敱鐢ㄦ埛绌洪棿鐢ㄤ簬妫€绱㈠鎺ュ瓧鐨勫绛夌涓婁笅鏂囥€?
   4) 铏界劧涓嶆槸 SCTP 鐗规湁鐨勶紝浣嗚娉ㄦ剰锛氫娇鐢?NetLabel 鏃讹紝濡傛灉鏍囩琚垎閰嶇粰鏌愪釜鐗瑰畾鎺ュ彛锛岃€岃
      鎺ュ彛鈥渄own 鎺夆€濓紝NetLabel 鏈嶅姟灏嗙Щ闄よ鏉＄洰銆傚洜姝よ纭繚缃戠粶鍚姩鑴氭湰璋冪敤 **netlabelctl**\(8)
      鏉ヨ缃墍闇€鏍囩锛堣瑙?**netlabel-config**\(8) 杈呭姪鑴氭湰锛夈€?
   5) NetLabel 鐨?SCTP 瀵圭瓑绔爣绛捐鍒欓€傜敤浜?https://www.paul-moore.com/blog/t 涓婃爣绛句负
      "netlabel" 鐨勮繖缁勫笘瀛愪腑鐨勮璁恒€?
   6) CIPSO 浠呮敮鎸?IPv4 瀵诲潃锛歚socket(AF_INET, ...)`锛汣ALIPSO 浠呮敮鎸?IPv6 瀵诲潃锛?      `socket(AF_INET6, ...)`

      娴嬭瘯 CIPSO/CALIPSO 鏃惰娉ㄦ剰浠ヤ笅鍑犵偣锛?         a) 濡傛灉 SCTP 鍖呭洜鏍囩鏃犳晥鑰屾棤娉曟姇閫掞紝CIPSO 浼氬彂閫佷竴涓?ICMP 鍖呫€?         b) CALIPSO 涓嶅彂閫?ICMP 鍖咃紝鍙槸闈欓粯涓㈠純銆?
   7) IPSEC 涓嶅彈鏀寔锛屽洜涓?RFC 3554鈥斺€攕ctp/ipsec 鏀寔灏氭湭鍦ㄧ敤鎴风┖闂村疄鐜帮紙**racoon**\(8) 鎴?      **ipsec_pluto**\(8)锛夛紝灏界鍐呮牳鏀寔 SCTP/IPSEC銆?