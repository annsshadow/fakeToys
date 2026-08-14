
## Netlink specification support for raw Netlink families


鏈枃妗ｆ弿杩颁簡璇稿 `NETLINK_ROUTE` 杩欑被浣跨敤 `netlink-raw` 鍗忚瑙勮寖鐨勫師濮嬶紙raw锛塏etlink 鏃忔墍闇€鐨勯澶栧睘鎬с€?
## Specification


netlink-raw schema 閫氳繃鍘熷 netlink 鏃忔墍闇€鐨勫崗璁彿鍜岀粍鎾?ID 绛夊睘鎬э紝鎵╁睍浜?[genetlink-legacy <genetlink-legacy>](genetlink-legacy <genetlink-legacy>) schema銆傛洿澶氫俊鎭鍙傞槄 classic_netlink銆傚師濮?netlink 鏃忎篃浣跨敤鐗瑰畾绫诲瀷鐨勫瓙娑堟伅锛坰ub-message锛夈€?
### Globals


#### protonum


`protonum` 灞炴€х敤浜庢寚瀹氭墦寮€ netlink 濂楁帴瀛楁椂瑕佷娇鐢ㄧ殑鍗忚鍙枫€?

  # SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)

  name: rt-addr
  protocol: netlink-raw
  protonum: 0             # NETLINK_ROUTE 鍗忚鐨勪竴閮ㄥ垎

### Multicast group properties


#### value


`value` 灞炴€х敤浜庢寚瀹氱粍鎾粍娉ㄥ唽瑕佷娇鐢ㄧ殑缁?ID銆?

  mcast-groups:
    list:
      -
        name: rtnlgrp-ipv4-ifaddr
        value: 5
      -
        name: rtnlgrp-ipv6-ifaddr
        value: 9
      -
        name: rtnlgrp-mctp-ifaddr
        value: 34

### Sub-messages


鍑犱釜鍘熷 netlink 鏃忥紝濡?rt-link<netlink-rt-link> 鍜?tc<netlink-tc> 浣跨敤灞炴€у祵濂楋紙attribute nesting锛変綔涓轰竴绉嶆娊璞℃潵鎼哄甫妯″潡鐗瑰畾淇℃伅銆?
```

    [OUTER NEST OR MESSAGE LEVEL]
      [GENERIC ATTR 1]
      [GENERIC ATTR 2]
      [GENERIC ATTR 3]
      [GENERIC ATTR - wrapper]
        [MODULE SPECIFIC ATTR 1]
        [MODULE SPECIFIC ATTR 2]

```
澶栧眰绾у埆鐨?`GENERIC ATTRs` 瀹氫箟鍦ㄦ牳蹇冿紙鎴?rt_link 鎴栨牳蹇?TC锛変腑锛岃€岀壒瀹氱殑椹卞姩銆乀C 鍒嗙被鍣ㄣ€乹disc 绛夊彲浠ユ惡甯﹀畠浠嚜宸辩殑銆佸寘瑁瑰湪 `GENERIC ATTR - wrapper` 涓殑淇℃伅銆傚敖绠′笂闈㈢殑渚嬪瓙鏄剧ず浜嗗睘鎬у祵濂楀湪 wrapper 鍐呴儴锛屼絾妯″潡閫氬父鎷ユ湁瀹氫箟宓屽鏍煎紡鐨勫畬鍏ㄨ嚜鐢便€傚疄闄呬笂锛寃rapper 灞炴€х殑璐熻浇涓?netlink 娑堟伅鍏锋湁闈炲父鐩镐技鐨勭壒寰併€傚畠鍙兘鍖呭惈鍥哄畾澶撮儴/缁撴瀯銆乶etlink 灞炴€э紝鎴栦袱鑰呯殕鏈夈€傜敱浜庤繖浜涘叡鍚岀壒寰侊紝鎴戜滑灏?wrapper 灞炴€х殑璐熻浇绉颁负瀛愭秷鎭紙sub-message锛夈€?
瀛愭秷鎭睘鎬т娇鐢ㄥ彟涓€涓睘鎬х殑鍊间綔涓洪€夋嫨閿紙selector key锛夋潵閫夋嫨姝ｇ‘鐨勫瓙娑堟伅鏍煎紡銆備緥濡傦紝濡傛灉宸茬粡瑙ｇ爜浜嗕互涓嬪睘鎬э細


  { "kind": "gre" }

骞朵笖鎴戜滑閬囧埌浠ヤ笅灞炴€ц鑼冿細


  -
    name: data
    type: sub-message
    sub-message: linkinfo-data-msg
    selector: kind

閭ｄ箞鎴戜滑浼氭煡鎵惧悕涓?`linkinfo-data-msg` 鐨勫瓙娑堟伅瀹氫箟锛屽苟浣跨敤 `kind` 灞炴€х殑鍊硷紙鍗?`gre`锛変綔涓洪敭鏉ラ€夋嫨璇ュ瓙娑堟伅鐨勬纭牸寮忥細


  sub-messages:
    name: linkinfo-data-msg
    formats:
      -
        value: bridge
        attribute-set: linkinfo-bridge-attrs
      -
        value: gre
        attribute-set: linkinfo-gre-attrs
      -
        value: geneve
        attribute-set: linkinfo-geneve-attrs

杩欎細灏嗚灞炴€у€艰В鐮佷负浠ュ悕涓?`linkinfo-gre-attrs` 鐨?attribute-set 浣滀负灞炴€х┖闂寸殑瀛愭秷鎭€?
瀛愭秷鎭彲浠ユ湁涓€涓彲閫夌殑 `fixed-header`锛屽悗璺熸潵鑷?`attribute-set` 鐨勯浂涓垨澶氫釜灞炴€с€備緥濡傦紝浠ヤ笅 `tc-options-msg` 瀛愭秷鎭畾涔変簡娣峰悎浣跨敤 `fixed-header`銆乣attribute-set` 鎴栦袱鑰呭吋鏈夌殑娑堟伅鏍煎紡锛?

  sub-messages:
    -
      name: tc-options-msg
      formats:
        -
          value: bfifo
          fixed-header: tc-fifo-qopt
        -
          value: cake
          attribute-set: tc-cake-attrs
        -
          value: netem
          fixed-header: tc-netem-qopt
          attribute-set: tc-netem-attrs

璇锋敞鎰忥紝selector 灞炴€у繀椤诲嚭鐜板湪浠讳綍渚濊禆浜庡畠鐨勫瓙娑堟伅灞炴€т箣鍓嶏紝鍑虹幇鍦?netlink 娑堟伅涓€?
濡傛灉鍍?`kind` 杩欐牱鐨勫睘鎬у畾涔夊湪澶氫釜宓屽绾у埆涓婏紝閭ｄ箞瀛愭秷鎭€夋嫨鍣ㄥ皢浣跨敤"鏈€鎺ヨ繎"閫夋嫨鍣ㄧ殑閭ｄ釜鍊兼潵瑙ｆ瀽銆備緥濡傦紝濡傛灉鍚屼竴涓睘鎬у悕瀹氫箟鍦ㄤ竴涓祵濂楃殑 `attribute-set` 涓紙涓庡瓙娑堟伅閫夋嫨鍣ㄤ竴璧凤級浠ュ強椤跺眰鐨?`attribute-set` 涓紝閭ｄ箞閫夋嫨鍣ㄥ皢浣跨敤"鏈€鎺ヨ繎"閫夋嫨鍣ㄧ殑閭ｄ釜鍊兼潵瑙ｆ瀽銆傚鏋滆鍊兼病鏈夊嚭鐜板湪涓庤鑼冨畾涔夌浉鍚岀骇鍒殑娑堟伅涓紝鍒欒繖鏄竴涓敊璇€?
### Nested struct definitions


璁稿鍘熷 netlink 鏃忥紝濡?tc<netlink-tc>锛屼娇鐢ㄥ祵濂楃粨鏋勪綋瀹氫箟銆俙netlink-raw` schema 浣垮緱鍙互浣跨敤 `struct` 灞炴€у皢缁撴瀯浣撳祵鍏ュ埌缁撴瀯浣撳畾涔変腑銆備緥濡傦紝浠ヤ笅缁撴瀯浣撳畾涔夊皢 `tc-ratespec` 缁撴瀯浣撳畾涔夊祵鍏ュ埌 `struct tc-tbf-qopt` 鐨?`rate` 鍜?`peakrate` 鎴愬憳涓€?

  -
    name: tc-tbf-qopt
    type: struct
    members:
      -
        name: rate
        type: binary
        struct: tc-ratespec
      -
        name: peakrate
        type: binary
        struct: tc-ratespec
      -
        name: limit
        type: u32
      -
        name: buffer
        type: u32
      -
        name: mtu
        type: u32
