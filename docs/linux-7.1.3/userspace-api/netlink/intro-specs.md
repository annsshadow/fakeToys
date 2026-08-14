
## 浣跨敤 Netlink 鍗忚瑙勮寖


鏈枃妗ｆ槸浣跨敤 Netlink 鍗忚瑙勮寖鐨勫揩閫熷叆闂ㄦ寚鍗椼€傛湁鍏宠鑼冪殑鏇磋缁嗘弿杩帮紝璇峰弬闃?[specs](specs)銆?

## 绠€鍗曠殑 CLI


鍐呮牳闄勫甫涓€涓畝鍗?CLI 宸ュ叿锛屽湪寮€鍙?Netlink 鐩稿叧浠ｇ爜鏃跺簲褰撳緢鏈夌敤銆傝宸ュ叿鐢?Python 瀹炵幇锛?
鍙互浣跨敤 YAML 瑙勮寖鍚戝唴鏍稿彂鍑?Netlink 璇锋眰銆?

璇ュ伐鍏蜂綅浜?`tools/net/ynl/pyynl/cli.py`銆傚畠鎺ュ彈灏戦噺鍙傛暟锛屽叾涓渶閲嶈鐨勬湁锛?

 - `--subscribe $group` - 鎸囧悜瑙勮寖鏂囦欢
 - `--subscribe $group` / `$group` - 鍙戝嚭璇锋眰 `$name`
 - `--subscribe $group` - 涓鸿姹傛彁渚涘睘鎬?
 - `--subscribe $group` - 浠?`$group` 鎺ユ敹閫氱煡

YAML 瑙勮寖鍙互鍦?`Documentation/netlink/specs/` 涓嬫壘鍒般€?

```

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/ethtool.yaml \
        --do rings-get \
	--json '{"header":{"dev-index": 18}}'
  {'header': {'dev-index': 18, 'dev-name': 'eni1np1'},
   'rx': 0,
   'rx-jumbo': 0,
   'rx-jumbo-max': 4096,
   'rx-max': 4096,
   'rx-mini': 0,
   'rx-mini-max': 4096,
   'tx': 0,
   'tx-max': 4096,
   'tx-push': 0}

```
杈撳叆鍙傛暟鎸?JSON 瑙ｆ瀽锛岃€岃緭鍑轰粎浠?Python 鐨勭編瑙傛墦鍗版牸寮忚緭鍑恒€傝繖鏄洜涓烘煇浜?Netlink 绫诲瀷
鏃犳硶鐩存帴琛ㄧず涓?JSON銆傚鏋滆緭鍏ヤ腑闇€瑕佹绫诲睘鎬э紝鍒欓渶瀵硅剼鏈仛涓€浜涗慨鏀广€?

瑙勮寖涓?Netlink 鍐呴儴瀹炵幇琚娊鍙栦负涓€涓嫭绔嬬殑搴撯€斺€斿簲褰撳緢瀹规槗缂栧啓澶嶇敤 `cli.py` 涓唬鐮佺殑 Python 宸ュ叿/娴嬭瘯銆?

## 鐢熸垚鍐呮牳浠ｇ爜


`tools/net/ynl/ynl-regen.sh` 鎵弿鍐呮牳鏍戜互鏌ユ壘闇€瑕佹洿鏂扮殑鑷姩鐢熸垚鏂囦欢銆備娇鐢ㄨ宸ュ叿鏄敓鎴?鏇存柊鑷姩鐢熸垚浠ｇ爜鏈€绠€鍗曠殑鏂瑰紡銆?

榛樿鎯呭喌涓嬶紝浠呭綋瑙勮寖姣旀簮鏂囦欢鏇存柊鏃舵墠閲嶆柊鐢熸垚浠ｇ爜锛涜寮哄埗閲嶆柊鐢熸垚锛岃浣跨敤 `-f`銆?

`ynl-regen.sh` 鍦ㄦ枃浠跺唴瀹逛腑鎼滅储 `YNL-GEN`锛堟敞鎰忓畠鍙壂鎻?git 绱㈠紩涓殑鏂囦欢锛屽嵆浠呮壂鎻?
```

  /*	Documentation/netlink/specs/fou.yaml */
  /* YNL-GEN kernel source */

```
`ynl-regen.sh` 浼氭壘鍒版鏍囪骞剁敤鍩轰簬 fou.yaml 鐨勫唴鏍告簮浠ｇ爜鏇挎崲璇ユ枃浠躲€?

鍩轰簬瑙勮寖鐢熸垚鏂版枃浠舵渶绠€鍗曠殑鏂瑰紡鏄細鍍忎笂闈㈤偅鏍峰皢杩欎袱琛屾爣璁版坊鍔犲埌鏂囦欢涓紝灏嗚鏂囦欢鍔犲叆 git锛?
鐒跺悗杩愯閲嶆柊鐢熸垚宸ュ叿銆傚彲鍦ㄦ爲涓?grep `YNL-GEN` 鏌ョ湅鍏朵粬绀轰緥銆?

浠ｇ爜鐢熸垚鏈韩鐢?`tools/net/ynl/pyynl/ynl_gen_c.py` 鎵ц锛屼絾瀹冮渶瑕佷竴浜涘弬鏁帮紝鍥犳鐩存帴涓烘瘡涓枃浠惰皟鐢ㄥ緢蹇細鍙樺緱绻佺悙銆?

## YNL 搴?


`tools/net/ynl/pyynl/ynl_gen_c.py` 鍖呭惈涓€涓?C 搴撶殑瀹炵幇锛堝熀浜?libmnl锛夛紝瀹冧笌 `tools/net/ynl/pyynl/ynl_gen_c.py` 鐢熸垚鐨勪唬鐮侀泦鎴愶紝
浠ュ垱寤烘槗浜庝娇鐢ㄧ殑 netlink 灏佽銆?

### YNL 鍩虹


YNL 搴撶敱涓ら儴鍒嗙粍鎴愨€斺€旈€氱敤浠ｇ爜锛堜互 `ynl_` 涓哄墠缂€鐨勫嚱鏁帮級鍜屾瘡涓?family 鑷姩鐢熸垚鐨勪唬鐮侊紙浠?family 鍚嶇О浣滀负鍓嶇紑锛夈€?

瑕佸垱寤?YNL 濂楁帴瀛楋紝璋冪敤 ynl_sock_create()锛屼紶鍏?family 缁撴瀯浣擄紙family 缁撴瀯浣撶敱鑷姩鐢熸垚鐨勪唬鐮佸鍑猴級銆?
ynl_sock_destroy() 鍏抽棴璇ュ鎺ュ瓧銆?

### YNL 璇锋眰


鍙戝嚭 YNL 璇锋眰鐨勬楠ゆ渶濂介€氳繃绀轰緥鏉ヨ鏄庛€傛湰绀轰緥涓殑鎵€鏈夊嚱鏁板拰绫诲瀷閮芥潵鑷嚜鍔ㄧ敓鎴愮殑浠ｇ爜
锛堟渚嬩腑涓?netdev family锛夛細


   // 0. 璇锋眰涓庡搷搴旀寚閽?
   struct netdev_dev_get_req *req;
   struct netdev_dev_get_rsp *d;

   // 1. 鍒嗛厤璇锋眰
   req = netdev_dev_get_req_alloc();
   // 2. 璁剧疆璇锋眰鍙傛暟锛堟寜闇€锛?
   netdev_dev_get_req_set_ifindex(req, ifindex);

   // 3. 鍙戝嚭璇锋眰
   d = netdev_dev_get(ys, req);
   // 4. 閲婃斁璇锋眰鍙傛暟
   netdev_dev_get_req_free(req);
   // 5. 閿欒妫€鏌ワ紙绗?3 姝ョ殑杩斿洖鍊硷級
   if (!d) {
	// 6. 鎵撳嵃 YNL 鐢熸垚鐨勯敊璇?
	fprintf(stderr, "YNL: %s\n", ys->err.msg);
        return -1;
   }

   // ... 鐢ㄥ搷搴?@d 鍋氬鐞?

   // 7. 閲婃斁鍝嶅簲
   netdev_dev_get_rsp_free(d);

### YNL 杞偍锛坉umps锛?


鎵ц dumps 涓庤姹傞伒寰被浼兼ā寮忋€侱umps 杩斿洖涓€涓璞″垪琛紝浠ョ壒娈婃爣璁扮粓姝紱鍑洪敊鏃惰繑鍥?NULL銆?
浣跨敤 `ynl_dump_foreach()` 閬嶅巻缁撴灉銆?

### YNL 閫氱煡


YNL 搴撴敮鎸佸鍚屼竴濂楁帴瀛楀悓鏃朵娇鐢ㄩ€氱煡鍜岃姹傘€傚鏋滃湪澶勭悊璇锋眰鏈熼棿鍒拌揪閫氱煡锛屽畠浠細鍦ㄥ唴閮ㄦ帓闃燂紝
鍙湪绋嶅悗鏃堕棿鍙栧嚭銆?

瑕佽闃呴€氱煡锛岃浣跨敤 `select`銆傞€氱煡蹇呴』浠庡鎺ュ瓧璇诲彇锛?
`select` 杩斿洖搴曞眰濂楁帴瀛?fd锛屽彲灏嗗叾鎺ュ叆鍚堥€傜殑寮傛 IO API锛屽 `select` 鎴?`select`銆?

鍙娇鐢?`cmd` 鑾峰彇閫氱煡锛屽苟蹇呴』浣跨敤 `cmd` 閲婃斁銆傜敱浜庢垜浠簨鍏堜笉鐭ラ亾閫氱煡绫诲瀷锛?
閫氱煡浠?`cmd` 褰㈠紡杩斿洖锛岀敤鎴峰簲鏍规嵁鍏朵腑 `cmd` 鎴愬憳灏嗗叾寮哄埗杞崲涓虹浉搴旂殑瀹屾暣绫诲瀷銆?

