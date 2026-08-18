
## batman-adv


Batman advanced 鏄竴绉嶄笉鍐嶅熀浜?IP 鐨勬棤绾跨綉缁滄柊鏂规硶銆備笌 batman 瀹堟姢杩涚▼浣跨敤 UDP 鍖呬氦鎹俊鎭?骞惰缃矾鐢辫〃涓嶅悓锛宐atman-advanced 浠呰繍琛屽湪 ISO/OSI 绗簩灞傦紝骞朵娇鐢ㄣ€佽矾鐢憋紙鎴栨洿鍑嗙‘鍦拌锛?妗ユ帴锛変互澶綉甯с€傚畠妯℃嫙鎵€鏈夊弬涓庤妭鐐圭殑铏氭嫙缃戠粶浜ゆ崲鏈恒€傚洜姝ゆ墍鏈夎妭鐐圭湅璧锋潵閮芥槸閾捐矾鏈湴鐨勶紝
浜庢槸鎵€鏈夋洿楂樺眰鐨勫崗璁兘涓嶄細鍙楃綉缁滃唴閮ㄤ换浣曞彉鍖栫殑褰卞搷銆備綘鍑犱箮鍙互鍦?batman advanced 涔嬩笂杩愯
浠讳綍鍗忚锛屾樉钁楃殑渚嬪瓙鏈夛細IPv4銆両Pv6銆丏HCP銆両PX銆?
Batman advanced 琚疄鐜颁负 Linux 鍐呮牳椹卞姩锛屼互灏嗗紑閿€闄嶅埌鏈€浣庛€傚畠涓嶄緷璧栦换浣曪紙鍏跺畠锛夌綉缁滈┍鍔紝
鍙敤浜?wifi 浠ュ強浠ュお缃?LAN銆乂PN 绛夆€︹€︼紙浠讳綍鍏锋湁浠ュお缃戦鏍肩浜屽眰鐨勪粙璐級銆?

## 閰嶇疆


```

  $ insmod batman-adv.ko

```
妯″潡鐜板湪姝ｅ湪绛夊緟婵€娲汇€備綘蹇呴』娣诲姞涓€浜?batman-adv 鍙互鍦ㄥ叾涓婅繍琛岀殑鎺ュ彛銆俠atman-adv 缃戞牸鎺ュ彛
鍙互浣跨敤浠ヤ笅鍛戒护鍒涘缓
```

  $ ip link add name bat0 type batadv

```
```

  $ ip link set dev eth0 master bat0

```
瀵规墍鏈夊笇鏈涙坊鍔犵殑鎺ュ彛閲嶅姝ゆ楠ゃ€傜幇鍦?batman-adv 寮€濮嬪湪姝?杩欎簺鎺ュ彛涓婁娇鐢?骞挎挱銆?
```

  $ ip link set dev eth0 nomaster

```
```

  batctl -m bat0 interface create
  batctl -m bat0 interface add -M eth0

```
```

  batctl -m bat0 interface del -M eth0
  batctl -m bat0 interface destroy

```
姣忎釜 batadv 缃戞牸鎺ュ彛銆乿lan 鍜?hardif 閮芥湁棰濆鐨勮缃紝鍙互浣跨敤 batctl 淇敼銆傚叧浜庢鐨勮缁嗕俊鎭?鍙湪杩欎唤鎵嬪唽涓壘鍒般€?
渚嬪锛屼綘鍙互妫€鏌ュ綋鍓嶇殑婧愯妭鐐归棿闅旓紙origination interval锛屼互姣涓哄崟浣嶇殑鍊硷紝鍐冲畾 batman-adv
鍙戦€佸叾骞挎挱鐨勯鐜囷級
```

  $ batctl -M bat0 orig_interval
  1000

```
```

  $ batctl -M bat0 orig_interval 3000

```
鍦ㄩ珮搴︾Щ鍔ㄧ殑鍦烘櫙涓紝浣犲彲鑳藉笇鏈涙妸婧愯妭鐐归棿闅旇皟浣庛€傝繖灏嗕娇缃戞牸瀵规嫇鎵戝彉鍖栨洿鏁忔劅锛屼絾涔熶細澧炲姞寮€閿€銆?
鍏充簬褰撳墠鐘舵€佺殑淇℃伅鍙互閫氳繃 batadv 閫氱敤 netlink 绯诲垪璁块棶銆俠atctl 閫氳繃鍏惰皟璇曡〃锛坉ebug tables锛?瀛愬懡浠ゆ彁渚涗簡涓€涓汉绫诲彲璇荤殑鐗堟湰銆?

## 浣跨敤


瑕佷娇鐢ㄤ綘鏂板垱寤虹殑缃戞牸锛宐atman advanced 鎻愪緵浜嗕竴涓柊鐨勬帴鍙?"bat0"锛屼粠姝や綘搴旇浣跨敤瀹冦€傛墍鏈夋坊鍔犲埌
batman advanced 鐨勬帴鍙ｉ兘涓嶅啀鐩稿叧锛屽洜涓?batman 浼氫负浣犲鐞嗗畠浠€傚熀鏈笂锛屼汉浠€氳繃浣跨敤 batman
鎺ュ彛鏉モ€滀氦鍑衡€濇暟鎹紝batman 浼氱‘淇濆畠鍒拌揪鐩殑鍦般€?
"bat0" 鎺ュ彛鍙互鍍忎换浣曞叾瀹冨父瑙勬帴鍙ｄ竴鏍蜂娇鐢ㄣ€傚畠闇€瑕佷竴涓?IP 鍦板潃锛屽彲浠ユ槸闈欐€侀厤缃紝涔熷彲浠ユ槸
鍔ㄦ€佽幏鍙栵紙閫氳繃浣跨敤
```

  NodeA: ip link set up dev bat0
  NodeA: ip addr add 192.168.0.1/24 dev bat0

  NodeB: ip link set up dev bat0
  NodeB: ip addr add 192.168.0.2/24 dev bat0
  NodeB: ping 192.168.0.1

```
娉ㄦ剰锛氫负閬垮厤闂锛岃绉婚櫎涔嬪墠鍒嗛厤缁?```

  $ ip addr flush dev eth0


```
## 鏃ュ織/璋冭瘯


鎵€鏈夐敊璇秷鎭€佽鍛婂拰淇℃伅娑堟伅閮借鍙戦€佸埌鍐呮牳鏃ュ織銆傛牴鎹綘鎿嶄綔绯荤粺鐨勫彂琛岀増锛屽彲浠ラ€氳繃澶氱鏂瑰紡鏉?璇诲彇銆傚皾璇曚娇鐢ㄨ繖浜涘懡浠わ細`dmesg`銆乣logread`锛屾垨鏌ョ湅鏂囦欢 `/var/log/kern.log` 鎴?`/var/log/syslog`銆傛墍鏈?batman-adv 娑堟伅
```

  $ dmesg | grep batman-adv

```
鍦ㄧ爺绌剁綉鏍肩綉缁滅殑闂鏃讹紝鏈夋椂闇€瑕佹煡鐪嬫洿璇︾粏鐨勮皟璇曟秷鎭€傝繖蹇呴』鍦ㄧ紪璇?batman-adv 妯″潡鏃跺惎鐢ㄣ€?褰撴妸 batman-adv 浣滀负鍐呮牳鐨勪竴閮ㄥ垎鏋勫缓鏃讹紝浣跨敤 "make menuconfig" 骞跺惎鐢ㄩ€夐」
`B.A.T.M.A.N. debugging`锛坄CONFIG_BATMAN_ADV_DEBUG=y`锛夈€?
```

  $ trace-cmd stream -e batadv:batadv_dbg

```
棰濆璋冭瘯杈撳嚭榛樿鏄叧闂殑銆傚畠鍙互鍦?```

  $ batctl -m bat0 loglevel routes tt

```
鏃跺惎鐢紝灏嗕负璺敱鍜岃浆鎹㈣〃锛坱ranslation table锛夋潯鐩彉鍖栨椂鍚敤璋冭瘯娑堟伅銆?
杩涘叆鍜岀寮€ batman-adv 鐨勪笉鍚岀被鍨嬫暟鎹寘鐨勮鏁板櫒
```

  $ ethtool --statistics bat0


```
## batctl


鐢变簬 batman advanced 杩愯鍦ㄧ浜屽眰锛屽弬涓庤櫄鎷熶氦鎹㈡満鐨勬墍鏈変富鏈哄鎵€鏈夌浜屽眰涔嬩笂鐨勫崗璁畬鍏ㄩ€忔槑銆?鍥犳甯哥敤鐨勮瘖鏂伐鍏锋棤娉曟寜棰勬湡宸ヤ綔銆備负浜嗗厠鏈嶈繖浜涢棶棰橈紝鍒涘缓浜?batctl銆傜洰鍓?batctl 鍖呭惈 ping銆?traceroute銆乼cpdump 浠ュ強鍒板唴鏍告ā鍧楄缃殑鎺ュ彛銆?
鏇村淇℃伅璇峰弬闃呮墜鍐岄〉锛坄man batctl`锛夈€?
batctl 鍙湪 https://www.open-mesh.org/ 鑾峰彇銆?

## 鑱旂郴鏂瑰紡


璇峰悜鎴戜滑鍙戦€佽瘎璁恒€佺粡楠屻€侀棶棰橈紝浠讳綍鍐呭閮藉彲浠?:)

IRC:
  #batadv on ircs://irc.hackint.org/
Mailing-list:
  b.a.t.m.a.n@lists.open-mesh.org锛堝彲閫夎闃呭湴鍧€锛?  https://lists.open-mesh.org/mailman3/postorius/lists/b.a.t.m.a.n.lists.open-mesh.org/锛?
浣犱篃鍙互鑱旂郴浣滆€咃細

- Marek Lindner <marek.lindner@mailbox.org>
- Simon Wunderlich <sw@simonwunderlich.de>
