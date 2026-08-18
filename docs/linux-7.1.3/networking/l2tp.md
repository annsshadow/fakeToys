
## L2TP


绗簩灞傞毀閬撳崗璁紙L2TP锛夊厑璁稿皢 L2 甯ч€氳繃 IP 缃戠粶杩涜闅ч亾浼犺緭銆?
鏈枃妗ｆ兜鐩栧唴鏍哥殑 L2TP 瀛愮郴缁熴€傚畠涓哄笇鏈涗娇鐢?L2TP 瀛愮郴缁熺殑搴旂敤绋嬪簭寮€鍙戣€呰褰曞唴鏍?API锛屽苟鎻愪緵涓€浜涘叧浜庡唴閮ㄥ疄鐜扮殑鎶€鏈粏鑺傦紝杩欎簺鍙兘瀵瑰唴鏍稿紑鍙戣€呭拰缁存姢鑰呮湁鐢ㄣ€?
## 姒傝堪


鍐呮牳鐨?L2TP 瀛愮郴缁熷疄鐜颁簡 L2TPv2 鍜?L2TPv3 鐨勬暟鎹矾寰勩€侺2TPv2 鎵胯浇浜?UDP 涔嬩笂銆侺2TPv3 鎵胯浇浜?UDP 涔嬩笂鎴栫洿鎺ユ壙杞戒簬 IP锛堝崗璁?115锛変箣涓娿€?
L2TP 鐨?RFC 瀹氫箟浜嗕袱绉嶅熀鏈被鍨嬬殑 L2TP 鏁版嵁鍖咃細鎺у埗鏁版嵁鍖咃紙"鎺у埗骞抽潰"锛夊拰鏁版嵁鍖咃紙"鏁版嵁骞抽潰"锛夈€傚唴鏍稿彧澶勭悊鏁版嵁鍖呫€傛洿澶嶆潅鐨勬帶鍒舵暟鎹寘鐢辩敤鎴风┖闂村鐞嗐€?
涓€涓?L2TP 闅ч亾鎵胯浇涓€涓垨澶氫釜 L2TP 浼氳瘽銆傛瘡涓毀閬撳叧鑱斾竴涓鎺ュ瓧銆傛瘡涓?session 鍏宠仈涓€涓櫄鎷熺綉缁滆澶囷紝渚嬪 `pppN`銆乣l2tpethN`锛屾暟鎹抚閫氳繃瀹冨湪 L2TP 涔嬮棿浼犲叆/浼犲嚭銆侺2TP 澶翠腑鐨勫瓧娈垫爣璇嗛毀閬撴垨 session锛屼互鍙婂畠鏄帶鍒跺寘杩樻槸鏁版嵁鍖呫€傚綋浣跨敤 Linux 鍐呮牳 API 寤虹珛闅ч亾鍜?session 鏃讹紝鎴戜滑鍙槸鍦ㄥ缓绔?L2TP 鏁版嵁璺緞銆傛帶鍒跺崗璁殑鎵€鏈夋柟闈㈤兘鐢辩敤鎴风┖闂村鐞嗐€?
杩欑鑱岃矗鍒掑垎瀵艰嚧鍦ㄥ缓绔嬮毀閬撳拰 session 鏃舵湁涓€涓嚜鐒剁殑鎿嶄綔搴忓垪銆傝繃绋嬪涓嬶細

    1) 鍒涘缓涓€涓毀閬撳鎺ュ瓧銆傞€氳繃璇ュ鎺ュ瓧涓庡绔氦鎹?L2TP 鎺у埗鍗忚娑堟伅锛屼互寤虹珛闅ч亾銆?
    2) 浣跨敤閫氳繃鎺у埗鍗忚娑堟伅浠庡绔幏寰楃殑淇℃伅锛屽湪鍐呮牳涓垱寤洪毀閬撲笂涓嬫枃銆?
    3) 閫氳繃闅ч亾濂楁帴瀛椾笌瀵圭浜ゆ崲 L2TP 鎺у埗鍗忚娑堟伅锛屼互寤虹珛 session銆?
    4) 浣跨敤閫氳繃鎺у埗鍗忚娑堟伅浠庡绔幏寰楃殑淇℃伅锛屽湪鍐呮牳涓垱寤?session 涓婁笅鏂囥€?
## L2TP API


鏈妭璁板綍 L2TP 瀛愮郴缁熺殑姣忎釜鐢ㄦ埛绌洪棿 API銆?
### 闅ч亾濂楁帴瀛?

L2TPv2 濮嬬粓浣跨敤 UDP銆侺2TPv3 鍙互浣跨敤 UDP 鎴?IP 灏佽銆?
瑕佸垱寤轰緵 L2TP 浣跨敤鐨勯毀閬撳鎺ュ瓧锛屼娇鐢ㄦ爣鍑?POSIX 濂楁帴瀛?API銆?
```

    int sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);

```
```

    int sockfd = socket(AF_INET6, SOCK_DGRAM, IPPROTO_L2TP);

```
UDP 濂楁帴瀛楃紪绋嬫棤闇€鍦ㄦ璧樿堪銆?
IPPROTO_L2TP 鏄唴鏍?L2TP 瀛愮郴缁熷疄鐜扮殑涓€绉?IP 鍗忚绫诲瀷銆侺2TPIP 濂楁帴瀛楀湴鍧€瀹氫箟浜?struct
sockaddr_l2tpip 鍜?struct sockaddr_l2tpip6锛屼綅浜?`include/uapi/linux/l2tp.h`_銆傝鍦板潃鍖呭惈 L2TP 闅ч亾锛堣繛鎺ワ級id銆傝浣跨敤 L2TP IP 灏佽锛孡2TPv3 搴旂敤绋嬪簭搴斾娇鐢ㄦ湰鍦板垎閰嶇殑闅ч亾 id 缁戝畾 L2TPIP 濂楁帴瀛椼€傚綋宸茬煡瀵圭鐨勯毀閬?id 鍜?IP 鍦板潃鏃讹紝蹇呴』鎵ц connect銆?
濡傛灉 L2TP 搴旂敤绋嬪簭闇€瑕佸鐞嗘潵鑷娇鐢?L2TPIP 鐨勫绔殑 L2TPv3 闅ч亾寤虹珛璇锋眰锛屽畠蹇呴』鎵撳紑涓€涓笓鐢ㄧ殑 L2TPIP 濂楁帴瀛楁潵鐩戝惉杩欎簺璇锋眰锛屽苟浣跨敤闅ч亾 id 0 缁戝畾璇ュ鎺ュ瓧锛屽洜涓洪毀閬撳缓绔嬭姹傛槸瀵诲潃鍒伴毀閬?id 0 鐨勩€?
褰撻毀閬撳鎺ュ瓧鍏抽棴鏃讹紝L2TP 闅ч亾鍙婂叾鎵€鏈?session 浼氳嚜鍔ㄥ叧闂€?
### Netlink API


L2TP 搴旂敤绋嬪簭浣跨敤 netlink 绠＄悊鍐呮牳涓殑 L2TP 闅ч亾鍜?session 瀹炰緥銆侺2TP netlink API 瀹氫箟浜?`include/uapi/linux/l2tp.h`_銆?
L2TP 浣跨敤 `Generic Netlink`_锛圙ENL锛夈€傚畾涔変簡鑻ュ共鍛戒护锛?Create銆丏elete銆丮odify 鍜?Get锛岀敤浜庨毀閬撳拰 session 瀹炰緥锛屼緥濡?`L2TP_CMD_TUNNEL_CREATE`銆侫PI 澶村垪鍑轰簡鍙笌姣忎釜鍛戒护涓€璧蜂娇鐢ㄧ殑 netlink 灞炴€х被鍨嬨€?
闅ч亾鍜?session 瀹炰緥鐢辨湰鍦板敮涓€鐨?32 浣?id 鏍囪瘑銆侺2TP 闅ч亾 id 鐢?`L2TP_ATTR_CONN_ID` 鍜?`L2TP_ATTR_PEER_CONN_ID` 灞炴€х粰鍑猴紝L2TP session id 鐢?`L2TP_ATTR_SESSION_ID` 鍜?`L2TP_ATTR_PEER_SESSION_ID`
灞炴€х粰鍑恒€傚鏋滀娇鐢?netlink 绠＄悊 L2TPv2 闅ч亾鍜?session 瀹炰緥锛孡2TPv2 鐨?16 浣嶉毀閬?session id 鍦ㄨ繖浜涘睘鎬т腑琚己鍒惰浆鎹负 32 浣嶅€笺€?
鍦?`L2TP_CMD_TUNNEL_CREATE` 鍛戒护涓紝`L2TP_ATTR_FD` 鍛婅瘔鍐呮牳姝ｅ湪浣跨敤鐨勯毀閬撳鎺ュ瓧 fd銆傚鏋滄湭鎸囧畾锛屽唴鏍镐娇鐢ㄥ湪
`L2TP_ATTR_IP[^6^]_SADDR`銆乣L2TP_ATTR_IP[^6^]_DADDR`銆?`L2TP_ATTR_UDP_SPORT`銆乣L2TP_ATTR_UDP_DPORT` 灞炴€т腑璁剧疆鐨?IP 鍙傛暟锛屼负闅ч亾鍒涘缓涓€涓唴鏍稿鎺ュ瓧銆傚唴鏍稿鎺ュ瓧鐢ㄤ簬瀹炵幇闈炴墭绠＄殑 L2TPv3 闅ч亾锛坕proute2 鐨?"ip l2tp" 鍛戒护锛夈€傚鏋滅粰鍑轰簡 `L2TP_ATTR_FD`锛屽畠蹇呴』鏄凡缁忕粦瀹氬苟杩炴帴鐨勫鎺ュ瓧 fd銆傛湰鏂囨。鍚庨潰鏈夋洿澶氬叧浜庨潪鎵樼闅ч亾鐨勪俊鎭€?
`L2TP_CMD_TUNNEL_CREATE` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        璁剧疆闅ч亾锛堣繛鎺ワ級id銆?PEER_CONN_ID       Y        璁剧疆瀵圭闅ч亾锛堣繛鎺ワ級id銆?PROTO_VERSION      Y        鍗忚鐗堟湰銆? 鎴?3銆?ENCAP_TYPE         Y        灏佽绫诲瀷锛歎DP 鎴?IP銆?FD                 N        闅ч亾濂楁帴瀛楁枃浠舵弿杩扮銆?UDP_CSUM           N        鍚敤 IPv4 UDP 鏍￠獙鍜屻€備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?UDP_ZERO_CSUM6_TX  N        鍙戦€佹椂灏?IPv6 UDP 鏍￠獙鍜岀疆闆躲€備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?UDP_ZERO_CSUM6_RX  N        鎺ユ敹鏃跺皢 IPv6 UDP 鏍￠獙鍜岀疆闆躲€備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?IP_SADDR           N        IPv4 婧愬湴鍧€銆備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?IP_DADDR           N        IPv4 鐩殑鍦板潃銆備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?UDP_SPORT          N        UDP 婧愮鍙ｃ€備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?UDP_DPORT          N        UDP 鐩殑绔彛銆備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?IP6_SADDR          N        IPv6 婧愬湴鍧€銆備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?IP6_DADDR          N        IPv6 鐩殑鍦板潃銆備粎褰撴湭璁剧疆 FD 鏃朵娇鐢ㄣ€?DEBUG              N        璋冭瘯鏍囧織銆?================== ======== ===

`L2TP_CMD_TUNNEL_DESTROY` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        鏍囪瘑瑕侀攢姣佺殑闅ч亾 id銆?================== ======== ===

`L2TP_CMD_TUNNEL_MODIFY` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        鏍囪瘑瑕佷慨鏀圭殑闅ч亾 id銆?DEBUG              N        璋冭瘯鏍囧織銆?================== ======== ===

`L2TP_CMD_TUNNEL_GET` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            N        鏍囪瘑瑕佹煡璇㈢殑闅ч亾 id銆?                            鍦?DUMP 璇锋眰涓拷鐣ャ€?================== ======== ===

`L2TP_CMD_SESSION_CREATE` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        鐖堕毀閬?id銆?SESSION_ID         Y        璁剧疆 session id銆?PEER_SESSION_ID    Y        璁剧疆鐖?session id銆?PW_TYPE            Y        璁剧疆浼嚎绫诲瀷銆?DEBUG              N        璋冭瘯鏍囧織銆?RECV_SEQ           N        鍚敤 rx 鏁版嵁搴忓垪鍙枫€?SEND_SEQ           N        鍚敤 tx 鏁版嵁搴忓垪鍙枫€?LNS_MODE           N        鍚敤 LNS 妯″紡锛堣嚜鍔ㄥ惎鐢ㄦ暟鎹簭鍒楀彿锛夈€?RECV_TIMEOUT       N        閲嶆帓搴忔帴鏀舵暟鎹寘鏃剁殑绛夊緟瓒呮椂銆?L2SPEC_TYPE        N        璁剧疆 layer2-specific-sublayer 绫诲瀷锛堜粎 L2TPv3锛夈€?COOKIE             N        璁剧疆鍙€?cookie锛堜粎 L2TPv3锛夈€?PEER_COOKIE        N        璁剧疆鍙€夊绔?cookie锛堜粎 L2TPv3锛夈€?IFNAME             N        璁剧疆鎺ュ彛鍚嶇О锛堜粎 L2TPv3锛夈€?================== ======== ===

瀵逛簬浠ュお缃?session 绫诲瀷锛岃繖灏嗗垱寤轰竴涓?l2tpeth 铏氭嫙鎺ュ彛锛岄殢鍚庡彲鎸夐渶閰嶇疆銆傚浜?PPP session 绫诲瀷锛岃繕蹇呴』鎵撳紑骞惰繛鎺ヤ竴涓?PPPoL2TP 濂楁帴瀛楋紝灏嗗叾鏄犲皠鍒版柊 session銆傝繖鍦ㄥ悗闈㈢殑"PPPoL2TP 濂楁帴瀛?涓粙缁嶃€?
`L2TP_CMD_SESSION_DESTROY` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        鏍囪瘑瑕侀攢姣佺殑 session 鐨勭埗闅ч亾 id銆?SESSION_ID         Y        鏍囪瘑瑕侀攢姣佺殑 session id銆?IFNAME             N        閫氳繃鎺ュ彛鍚嶇О鏍囪瘑 session銆傚鏋滆缃紝灏嗚鐩栦换浣?CONN_ID 鍜?SESSION_ID 灞炴€с€傜洰鍓嶄粎鏀寔 L2TPv3 浠ュお缃?session銆?================== ======== ===

`L2TP_CMD_SESSION_MODIFY` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            Y        鏍囪瘑瑕佷慨鏀圭殑 session 鐨勭埗闅ч亾 id銆?SESSION_ID         Y        鏍囪瘑瑕佷慨鏀圭殑 session id銆?IFNAME             N        閫氳繃鎺ュ彛鍚嶇О鏍囪瘑 session銆傚鏋滆缃紝灏嗚鐩栦换浣?CONN_ID 鍜?SESSION_ID 灞炴€с€傜洰鍓嶄粎鏀寔 L2TPv3 浠ュお缃?session銆?DEBUG              N        璋冭瘯鏍囧織銆?RECV_SEQ           N        鍚敤 rx 鏁版嵁搴忓垪鍙枫€?SEND_SEQ           N        鍚敤 tx 鏁版嵁搴忓垪鍙枫€?LNS_MODE           N        鍚敤 LNS 妯″紡锛堣嚜鍔ㄥ惎鐢ㄦ暟鎹簭鍒楀彿锛夈€?RECV_TIMEOUT       N        閲嶆帓搴忔帴鏀舵暟鎹寘鏃剁殑绛夊緟瓒呮椂銆?================== ======== ===

`L2TP_CMD_SESSION_GET` 灞炴€э細-

================== ======== ===
Attribute          Required Use
================== ======== ===
CONN_ID            N        鏍囪瘑瑕佹煡璇㈢殑闅ч亾 id銆?                            瀵逛簬 DUMP 璇锋眰蹇界暐銆?SESSION_ID         N        鏍囪瘑瑕佹煡璇㈢殑 session id銆?                            瀵逛簬 DUMP 璇锋眰蹇界暐銆?IFNAME             N        閫氳繃鎺ュ彛鍚嶇О鏍囪瘑 session銆?                            濡傛灉璁剧疆锛屽皢瑕嗙洊浠讳綍 CONN_ID 鍜?                            SESSION_ID 灞炴€с€傚浜?DUMP 璇锋眰蹇界暐銆傜洰鍓嶄粎鏀寔 L2TPv3
                            浠ュお缃?session銆?================== ======== ===

搴旂敤绋嬪簭寮€鍙戣€呭簲鍙傝€?`include/uapi/linux/l2tp.h`_ 鑾峰彇 netlink 鍛戒护鍜屽睘鎬у畾涔夈€?
浣跨敤 libmnl_ 鐨勭ず渚嬬敤鎴风┖闂翠唬鐮侊細

```

        struct nl_sock *nl_sock;
        int l2tp_nl_family_id;

        nl_sock = nl_socket_alloc();
        genl_connect(nl_sock);
        genl_id = genl_ctrl_resolve(nl_sock, L2TP_GENL_NAME);

  - 鍒涘缓涓€涓毀閬?:

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_TUNNEL_CREATE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_FD, tunl_sock_fd);
        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_CONN_ID, peer_tid);
        mnl_attr_put_u8(nlh, L2TP_ATTR_PROTO_VERSION, protocol_version);
        mnl_attr_put_u16(nlh, L2TP_ATTR_ENCAP_TYPE, encap);

  - 鍒涘缓涓€涓?session::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_SESSION_CREATE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_CONN_ID, peer_tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_SESSION_ID, sid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_PEER_SESSION_ID, peer_sid);
        mnl_attr_put_u16(nlh, L2TP_ATTR_PW_TYPE, pwtype);
        /* there are other session options which can be set using netlink
         * attributes during session creation -- see l2tp.h
         */

  - 鍒犻櫎涓€涓?session::

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_SESSION_DELETE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);
        mnl_attr_put_u32(nlh, L2TP_ATTR_SESSION_ID, sid);

  - 鍒犻櫎涓€涓毀閬撳強鍏舵墍鏈?session锛堝鏋滄湁锛?:

        struct nlmsghdr *nlh;
        struct genlmsghdr *gnlh;

        nlh = mnl_nlmsg_put_header(buf);
        nlh->nlmsg_type = genl_id; /* assigned to genl socket */
        nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
        nlh->nlmsg_seq = seq;

        gnlh = mnl_nlmsg_put_extra_header(nlh, sizeof(*gnlh));
        gnlh->cmd = L2TP_CMD_TUNNEL_DELETE;
        gnlh->version = L2TP_GENL_VERSION;
        gnlh->reserved = 0;

        mnl_attr_put_u32(nlh, L2TP_ATTR_CONN_ID, tid);

```
### PPPoL2TP Session 濂楁帴瀛?API


瀵逛簬 PPP session 绫诲瀷锛屽繀椤绘墦寮€涓€涓?PPPoL2TP 濂楁帴瀛楀苟杩炴帴鍒?L2TP session銆?
鍒涘缓 PPPoL2TP 濂楁帴瀛楁椂锛屽簲鐢ㄧ▼搴忓湪濂楁帴瀛?connect() 璋冪敤涓悜鍐呮牳鎻愪緵鍏充簬闅ч亾鍜?session 鐨勪俊鎭€傛彁渚涙簮鍜岀洰鐨勯毀閬撳強 session id锛屼互鍙?UDP 鎴?L2TPIP 濂楁帴瀛楃殑鏂囦欢鎻忚堪绗︺€傚弬瑙?struct
pppol2tp_addr锛屼綅浜?`include/linux/if_pppol2tp.h`_銆傚嚭浜庡巻鍙插師鍥狅紝L2TPv2/L2TPv3 IPv4/IPv6 闅ч亾涓嶅垢鏈夌暐寰笉鍚岀殑鍦板潃缁撴瀯锛岀敤鎴风┖闂村繀椤讳娇鐢ㄤ笌闅ч亾濂楁帴瀛楃被鍨嬬浉鍖归厤鐨勯€傚綋缁撴瀯銆?
鐢ㄦ埛绌洪棿鍙互浣跨敤 PPPoX 濂楁帴瀛椾笂鐨?setsockopt 鍜?ioctl 鎺у埗闅ч亾鎴?session 鐨勮涓恒€傛敮鎸佷互涓嬪鎺ュ瓧閫夐」锛?

=========   ===========================================================
DEBUG       璋冭瘯娑堟伅绫诲埆銆傝涓嬫枃銆?SENDSEQ     - 0 => 涓嶅彂閫佸甫搴忓垪鍙风殑鏁版嵁鍖?            - 1 => 鍙戦€佸甫搴忓垪鍙风殑鏁版嵁鍖?RECVSEQ     - 0 => 鎺ユ敹鏁版嵁鍖呯殑搴忓垪鍙蜂负鍙€?            - 1 => 涓㈠純涓嶅甫搴忓垪鍙风殑鎺ユ敹鏁版嵁鍖?LNSMODE     - 0 => 鍏呭綋 LAC銆?            - 1 => 鍏呭綋 LNS銆?REORDERTO   閲嶆帓搴忚秴鏃讹紙姣锛夈€傝嫢涓?0锛屽垯涓嶅皾璇曢噸鎺掑簭銆?=========   ===========================================================

闄や簡鏍囧噯 PPP ioctls 澶栵紝杩樻彁渚涗簡 PPPIOCGL2TPSTATS锛岀敤浜庝娇鐢ㄧ浉搴旈毀閬撴垨 session 鐨?PPPoX 濂楁帴瀛椾粠鍐呮牳妫€绱㈤毀閬撳拰 session 缁熻淇℃伅銆?
绀轰緥鐢ㄦ埛绌洪棿浠ｇ爜锛?
```

        /* Input: the L2TP tunnel UDP socket `tunnel_fd`, which needs to be
         * bound already (both sockname and peername), otherwise it will not be
         * ready.
         */

        struct sockaddr_pppol2tp sax;
        int session_fd;
        int ret;

        session_fd = socket(AF_PPPOX, SOCK_DGRAM, PX_PROTO_OL2TP);
        if (session_fd < 0)
                return -errno;

        sax.sa_family = AF_PPPOX;
        sax.sa_protocol = PX_PROTO_OL2TP;
        sax.pppol2tp.fd = tunnel_fd;
        sax.pppol2tp.addr.sin_addr.s_addr = addr->sin_addr.s_addr;
        sax.pppol2tp.addr.sin_port = addr->sin_port;
        sax.pppol2tp.addr.sin_family = AF_INET;
        sax.pppol2tp.s_tunnel  = tunnel_id;
        sax.pppol2tp.s_session = session_id;
        sax.pppol2tp.d_tunnel  = peer_tunnel_id;
        sax.pppol2tp.d_session = peer_session_id;

        /* session_fd is the fd of the session's PPPoL2TP socket.
         * tunnel_fd is the fd of the tunnel UDP / L2TPIP socket.
         */
        ret = connect(session_fd, (struct sockaddr *)&sax, sizeof(sax));
        if (ret < 0 ) {
                close(session_fd);
                return -errno;
        }

        return session_fd;

```
L2TP 鎺у埗鏁版嵁鍖呭湪 `tunnel_fd` 涓婁粛鐒跺彲璇汇€?
```

        /* Input: the session PPPoX data socket `session_fd` which was created
         * as described above.
         */

        int ppp_chan_fd;
        int chindx;
        int ret;

        ret = ioctl(session_fd, PPPIOCGCHAN, &chindx);
        if (ret < 0)
                return -errno;

        ppp_chan_fd = open("/dev/ppp", O_RDWR);
        if (ppp_chan_fd < 0)
                return -errno;

        ret = ioctl(ppp_chan_fd, PPPIOCATTCHAN, &chindx);
        if (ret < 0) {
                close(ppp_chan_fd);
                return -errno;
        }

        return ppp_chan_fd;

```
LCP PPP 甯у湪 `ppp_chan_fd` 涓婂彲璇汇€?
```

        /* Input: the PPP channel `ppp_chan_fd` which was created as described
         * above.
         */

        int ifunit = -1;
        int ppp_if_fd;
        int ret;

        ppp_if_fd = open("/dev/ppp", O_RDWR);
        if (ppp_if_fd < 0)
                return -errno;

        ret = ioctl(ppp_if_fd, PPPIOCNEWUNIT, &ifunit);
        if (ret < 0) {
                close(ppp_if_fd);
                return -errno;
        }

        ret = ioctl(ppp_chan_fd, PPPIOCCONNECT, &ifunit);
        if (ret < 0) {
                close(ppp_if_fd);
                return -errno;
        }

        return ppp_if_fd;

```
IPCP/IPv6CP PPP 甯у湪 `ppp_if_fd` 涓婂彲璇汇€?
ppp<ifunit> 鎺ュ彛闅忓悗鍙互浣跨敤 netlink 鐨?RTM_NEWLINK銆丷TM_NEWADDR銆丷TM_NEWROUTE锛屾垨 ioctl 鐨?SIOCSIFMTU銆丼IOCSIFADDR銆丼IOCSIFDSTADDR銆丼IOCSIFNETMASK銆丼IOCSIFFLAGS锛屾垨浣跨敤 `ip` 鍛戒护杩涜甯歌閰嶇疆銆?
  - 妗ユ帴鍏锋湁 PPP 浼嚎绫诲瀷鐨?L2TP session锛堣繖涔熺О涓?    L2TP 闅ч亾浜ゆ崲鎴?L2TP 澶氳烦锛夐€氳繃妗ユ帴 PPP
```

        /* Input: the session PPPoX data sockets `session_fd1` and `session_fd2`
         * which were created as described further above.
         */

        int ppp_chan_fd;
        int chindx1;
        int chindx2;
        int ret;

        ret = ioctl(session_fd1, PPPIOCGCHAN, &chindx1);
        if (ret < 0)
                return -errno;

        ret = ioctl(session_fd2, PPPIOCGCHAN, &chindx2);
        if (ret < 0)
                return -errno;

        ppp_chan_fd = open("/dev/ppp", O_RDWR);
        if (ppp_chan_fd < 0)
                return -errno;

        ret = ioctl(ppp_chan_fd, PPPIOCATTCHAN, &chindx1);
        if (ret < 0) {
                close(ppp_chan_fd);
                return -errno;
        }

        ret = ioctl(ppp_chan_fd, PPPIOCBRIDGECHAN, &chindx2);
        close(ppp_chan_fd);
        if (ret < 0)
                return -errno;

        return 0;

```
鍙互鐪嬪嚭锛屾ˉ鎺?PPP 閫氶亾鏃讹紝PPP session 涓嶅湪鏈湴缁堢粨锛屼篃涓嶄細鍒涘缓鏈湴 PPP 鎺ュ彛銆傚埌杈句竴涓€氶亾鐨?PPP 甯х洿鎺ヤ紶閫掔粰鍙︿竴涓€氶亾锛屽弽涔嬩害鐒躲€?
PPP 閫氶亾涓嶉渶瑕佷繚鎸佹墦寮€銆傚彧闇€淇濇寔 session 鐨?PPPoX 鏁版嵁濂楁帴瀛楁墦寮€銆?
鏇翠竴鑸湴璇达紝涔熷彲浠ヤ互鐩稿悓鏂瑰紡妗ユ帴渚嬪 PPPoL2TP PPP 閫氶亾涓庡叾浠栫被鍨嬬殑 PPP 閫氶亾锛屼緥濡?PPPoE銆?
PPP 渚х殑鏇村缁嗚妭鍙傝 ppp_generic.rst銆?
### 鏃х増浠?L2TPv2 API


褰?L2TP 鍦?2.6.23 涓娆″姞鍏?Linux 鍐呮牳鏃讹紝瀹冨彧瀹炵幇浜?L2TPv2锛屼笖涓嶅寘鍚?netlink API銆傜浉鍙嶏紝鍐呮牳涓殑闅ч亾鍜?session 瀹炰緥鐩存帴浣跨敤 PPPoL2TP 濂楁帴瀛楃鐞嗐€侾PPoL2TP 濂楁帴瀛楃殑浣跨敤濡?PPPoL2TP Session 濂楁帴瀛?API"涓€鑺傛墍杩帮紝浣嗛毀閬撳拰 session 瀹炰緥鏄湪濂楁帴瀛?connect() 鏃惰嚜鍔ㄥ垱寤猴紝鑰屼笉鏄€氳繃鍗曠嫭鐨?netlink 璇锋眰鍒涘缓锛?
    - 闅ч亾浣跨敤闅ч亾绠＄悊濂楁帴瀛楃鐞嗭紝杩欐槸涓€涓笓鐢ㄧ殑 PPPoL2TP 濂楁帴瀛楋紝杩炴帴鍒帮紙鏃犳晥鐨勶級session id 0銆傚綋 PPPoL2TP 闅ч亾绠＄悊濂楁帴瀛楄繛鎺ユ椂鍒涘缓 L2TP 闅ч亾瀹炰緥锛屽苟鍦ㄥ鎺ュ瓧鍏抽棴鏃堕攢姣併€?
    - 褰?PPPoL2TP 濂楁帴瀛楄繛鎺ュ埌闈為浂 session id 鏃讹紝鍦ㄥ唴鏍镐腑鍒涘缓 session 瀹炰緥銆俿ession 鍙傛暟浣跨敤 setsockopt 璁剧疆銆傚綋濂楁帴瀛楀叧闂椂閿€姣?L2TP session 瀹炰緥銆?
姝?API 浠嶅彈鏀寔锛屼絾涓嶉紦鍔变娇鐢ㄣ€傜浉鍙嶏紝鏂扮殑 L2TPv2 搴旂敤绋嬪簭搴旈鍏堜娇鐢?netlink 鍒涘缓闅ч亾鍜?session锛岀劧鍚庝负 session 鍒涘缓 PPPoL2TP 濂楁帴瀛椼€?
### 闈炴墭绠?L2TPv3 闅ч亾


鍐呮牳 L2TP 瀛愮郴缁熻繕鏀寔闈欐€侊紙闈炴墭绠★級L2TPv3 闅ч亾銆傞潪鎵樼闅ч亾娌℃湁鐢ㄦ埛绌洪棿闅ч亾濂楁帴瀛楋紝涓斾笌瀵圭涓嶄氦鎹㈡帶鍒舵秷鎭潵寤虹珛闅ч亾锛涢毀閬撳湪闅ч亾涓ょ鎵嬪姩閰嶇疆銆傛墍鏈夐厤缃兘浣跨敤 netlink 瀹屾垚銆傝繖绉嶆儏鍐典笅涓嶉渶瑕?L2TP 鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鈥斺€旈毀閬撳鎺ュ瓧鐢卞唴鏍稿垱寤猴紝骞朵娇鐢ㄥ湪 `L2TP_CMD_TUNNEL_CREATE` netlink 璇锋眰涓彂閫佺殑鍙傛暟閰嶇疆銆俙iproute2` 鐨?`ip` 宸ュ叿鍏锋湁绠＄悊闈欐€?L2TPv3 闅ч亾鍛戒护锛涙墽琛?``ip l2tp help`` 浜嗚В鏇村淇℃伅銆?
### 璋冭瘯


L2TP 瀛愮郴缁熼€氳繃 debugfs 鏂囦欢绯荤粺鎻愪緵涓€绯诲垪璋冭瘯鎺ュ彛銆?
```

    # mount -t debugfs debugfs /debug

```
闅忓悗鍙互璁块棶 l2tp 鐩綍涓嬬殑鏂囦欢锛屾彁渚涘綋鍓嶉毀閬撳拰 session 涓婁笅鏂囧瓨鍦ㄦ儏鍐电殑姒傝
```

    # cat /debug/l2tp/tunnels

```
璋冭瘯鏂囦欢绯荤粺鏂囦欢涓嶅簲琚簲鐢ㄧ▼搴忕敤浜庤幏鍙?L2TP 鐘舵€佷俊鎭紝鍥犱负鏂囦欢鏍煎紡鍙兘浼氭洿鏀广€傚畠瀹炵幇鐢ㄤ簬鎻愪緵棰濆鐨勮皟璇曚俊鎭互甯姪璇婃柇闂銆傚簲鐢ㄧ▼搴忓簲鏀圭敤 netlink API銆?
姝ゅ锛孡2TP 瀛愮郴缁熶娇鐢ㄦ爣鍑嗗唴鏍镐簨浠惰窡韪?API 瀹炵幇璺熻釜鐐广€傚彲鐢ㄧ殑 L2TP 浜嬩欢鍙煡鐪嬩负
```

    # find /debug/tracing/events/l2tp

```
鏈€鍚庯紝/proc/net/pppol2tp 涔熸彁渚涳紝鐢ㄤ簬涓庡師濮?pppol2tp 浠ｇ爜鍚戝悗鍏煎銆傚畠鍙垪鍑哄叧浜?L2TPv2 闅ч亾鍜?session 鐨勪俊鎭€備笉榧撳姳浣跨敤瀹冦€?
## 鍐呴儴瀹炵幇


鏈妭闈㈠悜鍐呮牳寮€鍙戣€呭拰缁存姢鑰呫€?
### 濂楁帴瀛?

UDP 濂楁帴瀛楃敱缃戠粶鏍稿績瀹炵幇銆傚綋浣跨敤 UDP 濂楁帴瀛楀垱寤?L2TP 闅ч亾鏃讹紝閫氳繃鍦?UDP 濂楁帴瀛椾笂璁剧疆 encap_rcv 鍜?encap_destroy 鍥炶皟锛屽皢璇ュ鎺ュ瓧璁剧疆涓哄皝瑁呯殑 UDP 濂楁帴瀛椼€傛帴鏀跺埌璇ュ鎺ュ瓧涓婄殑鏁版嵁鍖呮椂璋冪敤 l2tp_udp_encap_recv銆傜敤鎴风┖闂村叧闂鎺ュ瓧鏃惰皟鐢?l2tp_udp_encap_destroy銆?
L2TPIP 濂楁帴瀛楀疄鐜颁簬 `net/l2tp/l2tp_ip.c`_ 鍜?`net/l2tp/l2tp_ip6.c`_銆?
### 闅ч亾


鍐呮牳涓烘瘡涓?L2TP 闅ч亾淇濈暀涓€涓?struct l2tp_tunnel 涓婁笅鏂囥€俵2tp_tunnel 濮嬬粓涓庝竴涓?UDP 鎴?L2TP/IP 濂楁帴瀛楀叧鑱旓紝骞朵繚鐣欓毀閬撲腑鐨?session 鍒楄〃銆傚綋闅ч亾棣栨鍚?L2TP 鏍稿績娉ㄥ唽鏃讹紝濂楁帴瀛椾笂鐨勫紩鐢ㄨ鏁板鍔犮€傝繖纭繚鍦ㄥ叾鏁版嵁缁撴瀯寮曠敤璇ュ鎺ュ瓧鏃讹紝濂楁帴瀛椾笉浼氳绉婚櫎銆?
闅ч亾鐢卞敮涓€鐨勯毀閬?id 鏍囪瘑銆傝 id 鍦?L2TPv2 涓负 16 浣嶏紝鍦?L2TPv3 涓负 32 浣嶃€傚唴閮ㄤ互 32 浣嶅€煎瓨鍌ㄣ€?
闅ч亾淇濆瓨鍦ㄦ寜缃戠粶锛坧er-net锛夌殑鍒楄〃涓紝鐢遍毀閬?id 绱㈠紩銆傞毀閬?id 鍛藉悕绌洪棿鐢?L2TPv2 鍜?L2TPv3 鍏变韩銆?
澶勭悊闅ч亾濂楁帴瀛楀叧闂篃璁告槸 L2TP 瀹炵幇涓渶妫樻墜鐨勯儴鍒嗐€傚鏋滅敤鎴风┖闂村叧闂毀閬撳鎺ュ瓧锛孡2TP 闅ч亾鍙婂叾鎵€鏈?session 蹇呴』鍏抽棴骞堕攢姣併€傜敱浜庨毀閬撲笂涓嬫枃鎸佹湁瀵归毀閬撳鎺ュ瓧鐨勫紩鐢紝鍦ㄩ毀閬?sock_put 鍏跺鎺ュ瓧涔嬪墠锛屼笉浼氳皟鐢ㄥ鎺ュ瓧鐨?sk_destruct銆傚浜?UDP 濂楁帴瀛楋紝褰撶敤鎴风┖闂村叧闂毀閬撳鎺ュ瓧鏃讹紝浼氳皟鐢ㄥ鎺ュ瓧鐨?encap_destroy 澶勭悊绋嬪簭锛孡2TP 鐢ㄥ畠鏉ュ惎鍔ㄩ毀閬撳叧闂姩浣溿€傚浜?L2TPIP 濂楁帴瀛楋紝濂楁帴瀛楃殑 close 澶勭悊绋嬪簭鍚姩鐩稿悓鐨勯毀閬撳叧闂姩浣溿€傞鍏堝叧闂墍鏈?session銆傛瘡涓?session 涓㈠純鍏跺闅ч亾鐨勫紩鐢ㄣ€傚綋闅ч亾寮曠敤褰掗浂鏃讹紝闅ч亾涓㈠純鍏跺濂楁帴瀛楃殑寮曠敤銆?
### Session


鍐呮牳涓烘瘡涓?session 淇濈暀涓€涓?struct l2tp_session 涓婁笅鏂囥€傛瘡涓?session 閮芥湁绉佹湁鏁版嵁锛岀敤浜庣壒瀹氫簬 session 绫诲瀷鐨勬暟鎹€傚湪 L2TPv2 涓紝session 鎬绘槸鎵胯浇 PPP 娴侀噺銆傚湪 L2TPv3 涓紝session 鍙互鎵胯浇浠ュお缃戝抚锛堜互澶綉浼嚎锛夋垨鍏朵粬鏁版嵁绫诲瀷锛屽 PPP銆丄TM銆丠DLC 鎴栧抚涓户銆侺inux 鐩墠浠呭疄鐜颁簡浠ュお缃戝拰 PPP session 绫诲瀷銆?
鏌愪簺 L2TP session 绫诲瀷杩樻湁涓€涓鎺ュ瓧锛圥PP 浼嚎锛夛紝鑰屽叾浠栧垯娌℃湁锛堜互澶綉浼嚎锛夈€?
涓庨毀閬撶被浼硷紝L2TP session 鐢卞敮涓€鐨?session id 鏍囪瘑銆備笌闅ч亾 id 涓€鏍凤紝session id 鍦?L2TPv2 涓负 16 浣嶏紝鍦?L2TPv3 涓负 32 浣嶃€傚唴閮ㄤ互 32 浣嶅€煎瓨鍌ㄣ€?
Session 鎸佹湁瀵瑰叾鐖堕毀閬撶殑寮曠敤锛屼互纭繚鍦ㄦ湁涓€涓垨澶氫釜 session 寮曠敤闅ч亾鏃堕毀閬撲粛鐒跺瓨鍦ㄣ€?
Session 淇濆瓨鍦ㄦ寜缃戠粶锛坧er-net锛夌殑鍒楄〃涓€侺2TPv2 session 鍜?L2TPv3 session 瀛樺偍鍦ㄥ崟鐙殑鍒楄〃涓€侺2TPv2 session 鐢?16 浣嶉毀閬?ID 鍜?16 浣?session ID 缁勬垚鐨?32 浣嶉敭绱㈠紩銆侺2TPv3 session 鐢?32 浣?session ID 绱㈠紩锛屽洜涓?L2TPv3 session id 鍦ㄦ墍鏈夐毀閬撲腑鍞竴銆?
灏界 L2TPv3 RFC 瑙勫畾 L2TPv3 session id 涓嶅彈闅ч亾闄愬埗锛屼絾 Linux 瀹炵幇鍘嗘潵鍏佽濡傛銆傝繖绉?session id 鍐茬獊浣跨敤浠?sk 鍜?session ID 涓洪敭鐨勬寜缃戠粶锛坧er-net锛夊搱甯岃〃鏉ユ敮鎸併€傛煡鎵?L2TPv3 session 鏃讹紝鍒楄〃椤瑰彲鑳介摼鎺ュ埌澶氫釜鍏锋湁璇?session ID 鐨?session锛屾鏃朵娇鐢ㄥ尮閰嶇粰瀹?sk锛堥毀閬擄級鐨?session銆?
### PPP


`net/l2tp/l2tp_ppp.c`_ 瀹炵幇浜?PPPoL2TP 濂楁帴瀛楁棌銆傛瘡涓?PPP session 閮芥湁涓€涓?PPPoL2TP 濂楁帴瀛椼€?
PPPoL2TP 濂楁帴瀛楃殑 sk_user_data 寮曠敤 l2tp_session銆?
鐢ㄦ埛绌洪棿閫氳繃 PPPoL2TP 濂楁帴瀛楀彂閫佸拰鎺ユ敹 PPP 鏁版嵁鍖呫€傚彧鏈?PPP 鎺у埗甯ч€氳繃姝ゅ鎺ュ瓧锛歅PP 鏁版嵁鍖呭畬鍏ㄧ敱鍐呮牳澶勭悊锛屽湪鍐呮牳 PPP 瀛愮郴缁熺殑 PPP 閫氶亾鎺ュ彛涔嬮棿锛屽湪 L2TP session 鍙婂叾鍏宠仈鐨?`pppN` 缃戠粶璁惧涔嬮棿浼犻€掋€?
L2TP PPP 瀹炵幇閫氳繃鍏抽棴鍏剁浉搴旂殑 L2TP session 鏉ュ鐞?PPPoL2TP 濂楁帴瀛楃殑鍏抽棴銆傝繖寰堝鏉傦紝鍥犱负瀹冨繀椤昏€冭檻涓?netlink session 鍒涘缓/閿€姣佽姹備互鍙?pppol2tp_connect 灏濊瘯閲嶆柊杩炴帴鍒版鍦ㄥ叧闂繃绋嬩腑鐨?session 鐨勭珵浜夈€侾PP session 鎸佹湁瀵瑰叾鍏宠仈濂楁帴瀛楃殑寮曠敤锛屼互渚垮湪 session 寮曠敤瀹冩椂濂楁帴瀛椾粛鐒跺瓨鍦ㄣ€?
### 浠ュお缃?

`net/l2tp/l2tp_eth.c`_ 瀹炵幇 L2TPv3 浠ュお缃戜吉绾裤€傚畠涓烘瘡涓?session 绠＄悊涓€涓?netdev銆?
L2TP 浠ュお缃?session 鐢?netlink 璇锋眰鍒涘缓鍜岄攢姣侊紝鎴栧湪闅ч亾閿€姣佹椂閿€姣併€備笌 PPP session 涓嶅悓锛屼互澶綉 session 娌℃湁鍏宠仈鐨勫鎺ュ瓧銆?
## 鏉傞」


### RFCs


鍐呮牳浠ｇ爜瀹炵幇浜嗕互涓?RFC 涓瀹氱殑鏁版嵁璺緞鐗规€э細

======= =============== ===================================
RFC2661 L2TPv2          https://tools.ietf.org/html/rfc2661
RFC3931 L2TPv3          https://tools.ietf.org/html/rfc3931
RFC4719 L2TPv3 Ethernet https://tools.ietf.org/html/rfc4719
======= =============== ===================================

### 瀹炵幇


鑻ュ共寮€婧愬簲鐢ㄧ▼搴忎娇鐢?L2TP 鍐呮牳瀛愮郴缁燂細

============ ==============================================
iproute2     https://github.com/shemminger/iproute2
go-l2tp      https://github.com/katalix/go-l2tp
tunneldigger https://github.com/wlanslovenija/tunneldigger
xl2tpd       https://github.com/xelerance/xl2tpd
============ ==============================================

### 闄愬埗


褰撳墠瀹炵幇鏈変竴浜涢檺鍒讹細

  1) 涓?openvswitch 鐨勬帴鍙ｅ皻鏈疄鐜般€傚皢 OVS 浠ュお缃戝拰 VLAN 绔彛鏄犲皠鍒?L2TPv3 闅ч亾鍙兘鏈夌敤銆?
  2) VLAN 浼嚎浣跨敤閰嶇疆浜?VLAN 瀛愭帴鍙ｇ殑 `l2tpethN` 鎺ュ彛瀹炵幇銆傜敱浜?L2TPv3 VLAN 浼嚎鎵胯浇涓斾粎鎵胯浇涓€涓?VLAN锛屼娇鐢ㄥ崟涓€ netdev 鑰岄潪姣忎釜 VLAN session 浣跨敤 `l2tpethN` 鍜?`l2tpethN`:M 瀵瑰彲鑳芥洿濂姐€備负姝ゆ坊鍔犱簡 netlink 灞炴€?`L2TP_ATTR_VLAN_ID`锛屼絾瀹冧粠鏈瀹炵幇銆?
### 娴嬭瘯


闈炴墭绠?L2TPv3 浠ュお缃戠壒鎬х敱鍐呮牳鍐呯疆鐨勮嚜娴嬭瘯娴嬭瘯銆傚弬瑙?`tools/testing/selftests/net/l2tp.sh`_銆?
鍙︿竴涓祴璇曞浠?l2tp-ktest_ 瑕嗙洊浜嗘墍鏈?L2TP API 鍜岄毀閬?session 绫诲瀷銆傛湭鏉ュ彲鑳戒細闆嗘垚鍒板唴鏍稿唴缃殑 L2TP 鑷祴璇曚腑銆?