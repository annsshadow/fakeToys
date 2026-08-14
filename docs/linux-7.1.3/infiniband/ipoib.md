## 鍩轰簬 InfiniBand 鐨?IP锛圛PoIB锛?

  ib_ipoib 椹卞姩瀹炵幇浜嗗湪 InfiniBand 涔嬩笂鐨?IP锛圛P over InfiniBand锛夊崗璁紝绗﹀悎
  IETF ipoib 宸ヤ綔缁勫彂甯冪殑 RFC 4391 鍜?4392 瑙勮寖銆傚畠鏄€滃師鐢熲€濆疄鐜帮紝鍗虫妸鎺ュ彛绫诲瀷璁句负
  ARPHRD_INFINIBAND銆佺‖浠跺湴鍧€闀垮害璁句负 20锛堟棭鏈熺殑绉佹湁瀹炵幇鏄吉瑁呮垚浠ュお缃戞帴鍙ｅ悜鍐呮牳
  娉ㄥ唽鐨勶級銆?
## 鍒嗗尯涓?P_Key


  褰?IPoIB 椹卞姩琚姞杞芥椂锛屽畠浼氫负姣忎釜绔彛浣跨敤绱㈠紩 0 澶勭殑 P_Key 鍒涘缓涓€涓帴鍙ｃ€傝鍒涘缓
  涓€涓娇鐢ㄤ笉鍚?P_Key 鐨勬帴鍙ｏ紝鍙皢鏈熸湜鐨?P_Key 鍐欏叆涓绘帴鍙ｇ殑
```

    echo 0x8001 > /sys/class/net/ib0/create_child

  杩欏皢鍒涘缓涓€涓悕涓?ib0.8001銆丳_Key 涓?0x8001 鐨勬帴鍙ｃ€傝鍒犻櫎涓€涓瓙鎺ュ彛锛屼娇鐢?  "delete_child" 鏂囦欢::

    echo 0x8001 > /sys/class/net/ib0/delete_child

  P_Key 鍙€氳繃 "pkey" 鏂囦欢鑾峰彇锛屽瓙鎺ュ彛鐨勪富鎺ュ彛鍦?"parent" 涓€?
  瀛愭帴鍙ｇ殑鍒涘缓/鍒犻櫎涔熷彲浠ヤ娇鐢?IPoIB 鐨?rtnl_link_ops 瀹屾垚锛屼袱绉嶆柟寮忓垱寤虹殑瀛愭帴鍙?  琛屼负涓€鑷淬€?
```
## 鏁版嵁鎶ユā寮忎笌杩炴帴妯″紡


  IPoIB 椹卞姩鏀寔涓ょ鎿嶄綔妯″紡锛氭暟鎹姤锛坉atagram锛夊拰杩炴帴锛坈onnected锛夈€傛ā寮忛€氳繃鎺ュ彛
  鐨?/sys/class/net/<intf name>/mode 鏂囦欢璁剧疆鍜岃鍙栥€?
  鍦ㄦ暟鎹姤妯″紡涓嬶紝浣跨敤 IB UD锛堜笉鍙潬鏁版嵁鎶ワ級浼犺緭锛屽洜姝ゆ帴鍙?MTU 绛変簬 IB L2 MTU
  鍑忓幓 IPoIB 灏佽澶达紙4 瀛楄妭锛夈€備緥濡傚湪鍏稿瀷鐨?2K MTU 鐨?IB 浜ゆ崲缁撴瀯涓紝IPoIB MTU 涓?  2048 - 4 = 2044 瀛楄妭銆?
  鍦ㄨ繛鎺ユā寮忎笅锛屼娇鐢?IB RC锛堝彲闈犺繛鎺ワ級浼犺緭銆傝繛鎺ユā寮忓埄鐢ㄤ簡 IB 浼犺緭鐨勯潰鍚戣繛鎺ョ壒鎬э紝
  鍏佽 MTU 鏈€澶ц揪鍒?64K 鐨?IP 鍖呭ぇ灏忥紝浠庤€屽噺灏戝鐞嗗ぇ鍨?UDP 鏁版嵁鎶ャ€乀CP 娈电瓑鎵€闇€鐨?  IP 鍖呮暟閲忥紝骞舵彁鍗囧ぇ娑堟伅鐨勬€ц兘銆?
  鍦ㄨ繛鎺ユā寮忎笅锛屾帴鍙ｇ殑 UD QP 浠嶇敤浜庣粍鎾拰涓庝笉鏀寔杩炴帴妯″紡鐨勫绔€氫俊銆傝繖绉嶆儏鍐典笅锛?  浣跨敤 ICMP PMTU 鍖呯殑 RX 妯℃嫙鏉ヤ績浣跨綉缁滄爤瀵硅繖浜涢偦灞呬娇鐢ㄨ緝灏忕殑 UD MTU銆?
## 鏃犵姸鎬佸嵏杞?

  濡傛灉 IB 纭欢鏀寔 IPoIB 鏃犵姸鎬佸嵏杞斤紝IPoIB 浼氬悜缃戠粶鏍堥€氬憡 TCP/IP 鏍￠獙鍜屽拰/鎴栧ぇ鍙戦€?  锛圠SO锛夊嵏杞借兘鍔涖€?
  澶ф帴鏀讹紙LRO锛夊嵏杞戒篃宸插疄鐜帮紝鍙€氳繃 ethtool 璋冪敤寮€鍚?鍏抽棴銆傜洰鍓?LRO 浠呮敮鎸佸叿澶?  鏍￠獙鍜屽嵏杞借兘鍔涚殑璁惧銆?
  鏃犵姸鎬佸嵏杞戒粎鍦ㄦ暟鎹姤妯″紡涓嬪彈鏀寔銆?
## 涓柇 moderation


  濡傛灉搴曞眰 IB 璁惧鏀寔 CQ 浜嬩欢 moderation锛屽彲浠ヤ娇鐢?ethtool 璁剧疆涓柇缂撹В鍙傛暟锛屼粠鑰?  鍑忓皯澶勭悊涓柇甯︽潵鐨勫紑閿€銆侷PoIB 鐨勪富浠ｇ爜璺緞涓嶄娇鐢ㄤ簨浠舵潵鍋?TX 瀹屾垚閫氱煡锛屽洜姝ゅ彧鏀寔
  RX moderation銆?
## 璋冭瘯淇℃伅


  閫氳繃灏?CONFIG_INFINIBAND_IPOIB_DEBUG 缂栬瘧閫夐」璁句负 'y'锛岃窡韪俊鎭細琚紪璇戣繘椹卞姩銆?  閫氳繃灏嗘ā鍧楀弬鏁?debug_level 鍜?mcast_debug_level 璁句负 1 鏉ュ紑鍚€傝繖浜涘弬鏁板彲浠ュ湪杩愯鏃?  閫氳繃 /sys/module/ib_ipoib/ 涓嬬殑鏂囦欢杩涜鎺у埗銆?
  CONFIG_INFINIBAND_IPOIB_DEBUG 杩樹細鍦?debugfs 涓惎鐢ㄦ枃浠?```

    mount -t debugfs none /sys/kernel/debug

  杩欐牱灏卞彲浠ヤ粠 /sys/kernel/debug/ipoib/ib0_mcg 绛夋枃浠惰幏鍙栧叧浜庣粍鎾粍鐨勭粺璁′俊鎭€?
  璇ラ€夐」鐨勬€ц兘褰卞搷鍙拷鐣ヤ笉璁★紝鍥犳瀵逛簬姝ｅ父鎿嶄綔锛屽皢 debug_level 璁句负 0 鍚敤姝ら€夐」鏄?  瀹夊叏鐨勩€?
  CONFIG_INFINIBAND_IPOIB_DEBUG_DATA 浼氬湪 data_debug_level 璁句负 1 鏃跺湪鏁版嵁璺緞涓?  杈撳嚭鏇村璋冭瘯淇℃伅銆傜劧鑰岋紝鍗充娇鍏抽棴浜嗚緭鍑猴紝鍚敤璇ラ厤缃€夐」涔熶細褰卞搷鎬ц兘锛屽洜涓哄畠浼氬悜
  蹇€熻矾寰勪腑娣诲姞鍒ゆ柇銆?
```
## 鍙傝€冭祫鏂?

  Transmission of IP over InfiniBand (IPoIB) (RFC 4391)
    http://ietf.org/rfc/rfc4391.txt

  IP over InfiniBand (IPoIB) Architecture (RFC 4392)
    http://ietf.org/rfc/rfc4392.txt

  IP over InfiniBand: Connected Mode (RFC 4755)
    http://ietf.org/rfc/rfc4755.txt
