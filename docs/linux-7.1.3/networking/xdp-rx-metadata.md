
## XDP RX 鍏冩暟鎹?
鏈枃妗ｆ弿杩颁簡 XDP锛坋Xpress Data Path锛屽揩閫熸暟鎹矾寰勶級绋嬪簭濡備綍浣跨敤涓€缁勮緟鍔╁嚱鏁版潵
璁块棶涓庢暟鎹寘鐩稿叧鐨勭‖浠跺厓鏁版嵁锛屼互鍙婂浣曞皢璇ュ厓鏁版嵁浼犻€掔粰鍏朵粬娑堣垂鑰呫€?
## 鎬讳綋璁捐

XDP 鍙互璁块棶涓€缁?kfunc 鏉ユ搷浣?XDP 甯т腑鐨勫厓鏁版嵁銆傛瘡涓笇鏈涙毚闇查澶栨暟鎹寘鍏冩暟鎹殑
璁惧椹卞姩閮藉彲浠ュ疄鐜拌繖浜?kfunc銆傝繖缁?kfunc 閫氳繃 `XDP_METADATA_KFUNC_xxx` 鍦?`include/net/xdp.h` 涓０鏄庛€?
鐩墠鏀寔浠ヤ笅 kfunc銆傛湭鏉ワ紝闅忕潃鏀寔鏇村鍏冩暟鎹紝杩欑粍闆嗗悎灏嗕細鎵╁ぇ锛?
   :identifiers: bpf_xdp_metadata_rx_timestamp

   :identifiers: bpf_xdp_metadata_rx_hash

   :identifiers: bpf_xdp_metadata_rx_vlan_tag

XDP 绋嬪簭鍙互浣跨敤杩欎簺 kfunc 灏嗗厓鏁版嵁璇诲叆鏍堜笂鐨勫彉閲忎緵鑷韩浣跨敤銆傛垨鑰咃紝涓轰簡灏嗗厓鏁版嵁
浼犻€掔粰鍏朵粬娑堣垂鑰咃紝XDP 绋嬪簭鍙互灏嗗叾瀛樺偍鍒版暟鎹寘鍓嶉潰鎼哄甫鐨勫厓鏁版嵁鍖哄煙涓€傚苟闈炴墍鏈?鏁版嵁鍖呴兘蹇呯劧浼氭湁鎵€璇锋眰鐨勫厓鏁版嵁鍙敤锛屽湪杩欑鎯呭喌涓嬮┍鍔ㄤ細杩斿洖 `-ENODATA`銆?
骞堕潪鎵€鏈?kfunc 閮藉繀椤荤敱璁惧椹卞姩瀹炵幇锛涘綋鏈疄鐜版椂锛屽皢浣跨敤杩斿洖 `-EOPNOTSUPP` 鐨?榛樿鐗堟湰锛屼互琛ㄦ槑璁惧椹卞姩灏氭湭瀹炵幇璇?kfunc銆?
鍦?XDP 甯т腑锛屽厓鏁版嵁甯冨眬锛堥€氳繃 `xdp_buff` 璁块棶锛変负
```

  +----------+-----------------+------+
  | headroom | custom metadata | data |
  +----------+-----------------+------+
             ^                 ^
             |                 |
   xdp_buff->data_meta   xdp_buff->data

```
XDP 绋嬪簭鍙互浠ュ畠閫夋嫨鐨勪换浣曟牸寮忓皢鍚勪釜鍏冩暟鎹」瀛樺偍鍒拌繖涓?`data_meta` 鍖哄煙涓€傚悗缁殑
鍏冩暟鎹秷璐硅€呭繀椤婚€氳繃鏌愮甯﹀绾﹀畾鏉ュ氨鏍煎紡杈炬垚涓€鑷达紙渚嬪瀵逛簬 AF_XDP 鐢ㄤ緥锛岃涓嬫枃锛夈€?
## AF_XDP

[af_xdp](af_xdp) 鐢ㄤ緥鎰忓懗鐫€锛屽皢 XDP 甯ч噸瀹氬悜鍒?`AF_XDP` 濂楁帴瀛楋紙`XSK`锛夌殑 BPF
绋嬪簭涓庢渶缁堟秷璐硅€呬箣闂村瓨鍦ㄤ竴涓害瀹氥€傚洜姝わ紝BPF 绋嬪簭閫氳繃 `bpf_xdp_adjust_meta` 浠?鍏冩暟鎹腑鎵嬪姩鍒嗛厤鍥哄畾鏁伴噺鐨勫瓧鑺傦紝骞惰皟鐢ㄩ儴鍒?kfunc 鏉ュ～鍏呭畠銆傜敤鎴风┖闂?`XSK` 娑堣垂鑰?璁＄畻 `xsk_umem__get_data() - METADATA_SIZE` 鏉ュ畾浣嶈鍏冩暟鎹€傛敞鎰忥紝`xsk_umem__get_data`
瀹氫箟鍦?`libxdp` 涓紝鑰?`METADATA_SIZE` 鏄竴涓簲鐢ㄧ壒瀹氱殑甯搁噺锛坄AF_XDP` 鎺ユ敹鎻忚堪绗?骞禵涓峗鏄惧紡鎼哄甫鍏冩暟鎹殑澶у皬锛夈€?```

  +----------+-----------------+------+
  | headroom | custom metadata | data |
  +----------+-----------------+------+
                               ^
                               |
                        rx_desc->address

```
## XDP_PASS

杩欐槸灏?XDP 绋嬪簭澶勭悊杩囩殑鍖呬紶鍏ュ唴鏍哥殑璺緞銆傚唴鏍告牴鎹?`xdp_buff` 鐨勫唴瀹瑰垱寤?`skb`銆?鐩墠锛屾瘡涓┍鍔ㄩ兘鏈夎嚜瀹氫箟鐨勬牳鍐呬唬鐮侊紝鍦ㄨ繘琛?`xdp_buff->skb` 杞崲鏃惰В鏋愭弿杩扮骞?濉厖 `skb` 鍏冩暟鎹紝鑰屽湪鏋勫缓 `skbs` 鏃跺唴鏍稿苟涓嶄細浣跨敤 XDP 鍏冩暟鎹€備笉杩囷紝TC-BPF
绋嬪簭鍙互浣跨敤 `data_meta` 鎸囬拡璁块棶 XDP 鍏冩暟鎹尯鍩熴€?
鏈潵锛屾垜浠笇鏈涙敮鎸佽繖鏍蜂竴绉嶆儏鍐碉細XDP 绋嬪簭鍙互瑕嗙洊鐢ㄤ簬鏋勫缓 `skbs` 鐨勯儴鍒嗗厓鏁版嵁銆?
## bpf_redirect_map

`bpf_redirect_map` 鍙互灏嗗抚閲嶅畾鍚戝埌鍙︿竴涓澶囥€傛煇浜涜澶囷紙濡傝櫄鎷熶互澶綉閾捐矾锛夋敮鎸?鍦ㄩ噸瀹氬悜鍚庤繍琛岀浜屼釜 XDP 绋嬪簭銆備絾鏄紝鏈€缁堢殑娑堣垂鑰呮棤娉曡闂師濮嬬殑纭欢鎻忚堪绗︼紝涔?鏃犳硶璁块棶浠讳綍鍘熷鍏冩暟鎹€傝繖鍚屾牱閫傜敤浜庡畨瑁呭埌 devmap 鍜?cpumap 涓殑 XDP 绋嬪簭銆?
杩欐剰鍛崇潃瀵逛簬閲嶅畾鍚戝悗鐨勬暟鎹寘锛岀洰鍓嶅彧鏀寔鑷畾涔夊厓鏁版嵁锛屼笖蹇呴』鐢卞垵濮嬬殑 XDP 绋嬪簭
鍦ㄩ噸瀹氬悜涔嬪墠鍑嗗濂姐€傚鏋滆甯ф渶缁堣浼犲叆鍐呮牳锛岄偅涔堢敱璇ュ抚鍒涘缓鐨?`skb` 涓皢涓嶄細
濉厖浠讳綍纭欢鍏冩暟鎹€傚鏋滆繖鏍风殑鏁版嵁鍖呭悗鏉ヨ閲嶅畾鍚戝埌 `XSK`锛屽畠涔熷彧浼氳闂埌鑷畾涔?鍏冩暟鎹€?
## bpf_tail_call

鐩墠涓嶆敮鎸佸皢璁块棶鍏冩暟鎹?kfunc 鐨勭▼搴忔坊鍔犲埌 `BPF_MAP_TYPE_PROG_ARRAY` 涓€?
## 鏀寔鐨勮澶?
鍙互閫氳繃 netlink 鏌ヨ鐗瑰畾 netdev 瀹炵幇浜嗗摢涓?kfunc銆傚弬瑙?`Documentation/netlink/specs/netdev.yaml` 涓殑 `xdp-rx-metadata-features` 灞炴€ч泦銆?
## 椹卞姩瀹炵幇

鏌愪簺璁惧鍙兘浼氬湪琚帴鏀剁殑鏁版嵁鍖呭墠闈㈡坊鍔犲厓鏁版嵁銆備絾鏄埅鑷崇洰鍓嶏紝`AF_XDP` 缂轰箯灏?`data_meta` 鍖哄煙鐨勫ぇ灏忎紶閫掔粰娑堣垂鑰呯殑鑳藉姏銆傚洜姝わ紝椹卞姩鏈夎矗浠诲皢璁惧淇濈暀鐨勪换浣曞厓鏁版嵁
浠庡厓鏁版嵁鍖哄煙涓鍒跺嚭鏉ワ紝骞剁‘淇濆湪灏嗗抚鍛堢幇缁?XDP 绋嬪簭涔嬪墠锛宍xdp_buff->data_meta`
鎸囧悜 `xdp_buff->data`銆傝繖鏄繀瑕佺殑锛岃繖鏍峰湪 XDP 绋嬪簭璋冩暣鍏冩暟鎹尯鍩熶箣鍚庯紝娑堣垂鑰呮墠鑳?鍙潬鍦颁娇鐢?`METADATA_SIZE` 鍋忕Щ閲忔绱㈠埌鍏冩暟鎹湴鍧€銆?
涓嬮潰鐨勭ず鎰忓浘灞曠ず浜嗚嚜瀹氫箟鍏冩暟鎹浉瀵逛簬
```

              |<-- bpf_xdp_adjust_meta(xdp_buff, -METADATA_SIZE) --|
  new xdp_buff->data_meta                              old xdp_buff->data_meta
              |                                                    |
              |                                            xdp_buff->data
              |                                                    |
   +----------+----------------------------------------------------+------+
   | headroom |                  custom metadata                   | data |
   +----------+----------------------------------------------------+------+
              |                                                    |
              |                                            xdp_desc->addr
              |<------ xsk_umem__get_data() - METADATA_SIZE -------|

```
`bpf_xdp_adjust_meta` 纭繚 `METADATA_SIZE` 鎸?4 瀛楄妭瀵归綈锛屼笉瓒呰繃 252 瀛楄妭锛屽苟涓?鏋勫缓 xdp_frame 鐣欏嚭瓒冲绌洪棿銆傚鏋滀笉婊¤冻杩欎簺鏉′欢锛屽畠浼氳繑鍥炰竴涓礋鐨勯敊璇爜銆傚湪杩欑
鎯呭喌涓嬶紝BPF 绋嬪簭涓嶅簲缁х画鍚?`data_meta` 鍖哄煙濉厖鏁版嵁銆?
## 绀轰緥

鏈夊叧澶勭悊 XDP 鍏冩暟鎹殑 BPF 绋嬪簭绀轰緥锛岃鍙傝
`tools/testing/selftests/bpf/progs/xdp_metadata.c` 鍜?`tools/testing/selftests/bpf/prog_tests/xdp_metadata.c`銆?