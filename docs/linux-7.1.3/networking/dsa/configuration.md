
## 浠庣敤鎴风┖闂撮厤缃?DSA 浜ゆ崲鏈?

鐩墠锛孌SA 浜ゆ崲鏈洪厤缃皻鏈泦鎴愬埌涓绘祦鐨勭敤鎴风┖闂寸綉缁滈厤缃浠朵腑锛屽繀椤绘墜鍔ㄨ繘琛屻€?

### 閰嶇疆绀轰緥


瑕侀厤缃竴涓?DSA 浜ゆ崲鏈猴紝闇€瑕佹墽琛岃嫢骞插懡浠ゃ€傛湰鏂囨。灏嗕竴浜涘父瑙侀厤缃満鏅綔涓虹ず渚嬭繘琛岃瑙ｏ細

**鍗曠鍙ｏ紙single port锛?*
  姣忎釜浜ゆ崲鏈虹鍙ｉ兘浣滀负涓€涓彲閰嶇疆鐨勭嫭绔嬩互澶綉绔彛

**妗ユ帴锛坆ridge锛?*
  姣忎釜浜ゆ崲鏈虹鍙ｉ兘鏄竴涓彲閰嶇疆浠ュお缃戞ˉ鐨勪竴閮ㄥ垎

**缃戝叧锛坓ateway锛?*
  闄や竴涓笂娓哥鍙ｅ鐨勬瘡涓氦鎹㈡満绔彛閮芥槸鍙厤缃互澶綉妗ョ殑涓€閮ㄥ垎銆?  涓婃父绔彛浣滀负涓€涓彲閰嶇疆鐨勭嫭绔嬩互澶綉绔彛銆?
鎵€鏈夐厤缃兘浣跨敤鏉ヨ嚜 iproute2 鐨勫伐鍏峰畬鎴愶紝iproute2 鍙湪
https://www.kernel.org/pub/linux/utils/net/iproute2/ 鑾峰彇銆?
閫氳繃 DSA锛屼氦鎹㈡満鐨勬瘡涓鍙ｉ兘鍍忔櫘閫氱殑 Linux 浠ュお缃戞帴鍙ｄ竴鏍疯澶勭悊銆侰PU 绔彛鏄繛鎺ュ埌浠ュお缃?MAC
鑺墖鐨勪氦鎹㈡満绔彛銆傜浉搴旂殑 Linux 浠ュお缃戞帴鍙ｇО涓?conduit 鎺ュ彛锛堝绠℃帴鍙ｏ級銆傛墍鏈夊叾浠栫浉搴旂殑 Linux
鎺ュ彛绉颁负鐢ㄦ埛鎺ュ彛銆?
鐢ㄦ埛鎺ュ彛渚濊禆浜?conduit 鎺ュ彛澶勪簬 up 鐘舵€佹墠鑳藉彂閫佹垨鎺ユ敹娴侀噺銆傚湪鍐呮牳 v5.12 涔嬪墠锛宑onduit 鎺ュ彛鐨?鐘舵€佸繀椤荤敱鐢ㄦ埛鏄惧紡绠＄悊銆備粠鍐呮牳 v5.12 寮€濮嬶紝琛屼负濡備笅锛?
- 褰撲竴涓?DSA 鐢ㄦ埛鎺ュ彛琚媺璧凤紙up锛夋椂锛宑onduit 鎺ュ彛浼氳鑷姩鎷夎捣銆?- 褰?conduit 鎺ュ彛琚叧闂紙down锛夋椂锛屾墍鏈?DSA 鐢ㄦ埛鎺ュ彛浼氳鑷姩鍏抽棴銆?
鏈枃妗ｄ腑浣跨敤浠ヤ笅浠ュお缃戞帴鍙ｏ細

**eth0**
  conduit 鎺ュ彛

**eth1**
  鍙︿竴涓?conduit 鎺ュ彛

**lan1**
  涓€涓敤鎴锋帴鍙?
**lan2**
  鍙︿竴涓敤鎴锋帴鍙?
**lan3**
  绗笁涓敤鎴锋帴鍙?
**wan**
  涓撶敤浜庝笂娓告祦閲忕殑鐢ㄦ埛鎺ュ彛

鍙互杩涗竴姝ヤ互绫讳技鏂瑰紡閰嶇疆鍏朵粬浠ュお缃戞帴鍙ｃ€傞厤缃殑 IP 涓庣綉缁滃涓嬶細

**鍗曠鍙?*
  - lan1: 192.0.2.1/30 (192.0.2.0 - 192.0.2.3)
  - lan2: 192.0.2.5/30 (192.0.2.4 - 192.0.2.7)
  - lan3: 192.0.2.9/30 (192.0.2.8 - 192.0.2.11)

**妗ユ帴**
  - br0: 192.0.2.129/25 (192.0.2.128 - 192.0.2.255)

**缃戝叧**
  - br0: 192.0.2.129/25 (192.0.2.128 - 192.0.2.255)
  - wan: 192.0.2.1/30 (192.0.2.0 - 192.0.2.3)


### 甯︽爣璁版敮鎸佺殑閰嶇疆


鍩轰簬鏍囪锛坱agging锛夌殑閰嶇疆鏄ぇ澶氭暟 DSA 浜ゆ崲鏈烘墍鏈熸湜骞舵敮鎸佺殑銆傝繖浜涗氦鎹㈡満鑳藉鍦ㄤ笉浣跨敤鍩轰簬 VLAN
閰嶇疆鐨勬儏鍐典笅锛屽 incoming 鍜?outgoing 娴侀噺杩涜鏍囪銆?
**鍗曠鍙?*
  .. code-block:: sh

    # configure each interface
    ip addr add 192.0.2.1/30 dev lan1
    ip addr add 192.0.2.5/30 dev lan2
    ip addr add 192.0.2.9/30 dev lan3

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

**妗ユ帴**
  .. code-block:: sh

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up

**缃戝叧**
  .. code-block:: sh

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set wan up
    ip link set lan1 up
    ip link set lan2 up

    # configure the upstream port
    ip addr add 192.0.2.1/30 dev wan

    # create bridge
    ip link add name br0 type bridge

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up


### 涓嶅甫鏍囪鏀寔鐨勯厤缃?

灏戞暟浜ゆ崲鏈烘棤娉曚娇鐢ㄦ爣璁板崗璁紙DSA_TAG_PROTO_NONE锛夈€傝繖浜涗氦鎹㈡満鍙互閫氳繃鍩轰簬 VLAN 鐨勯厤缃繘琛岄厤缃€?
**鍗曠鍙?*
  璇ラ厤缃彧鑳介€氳繃 VLAN 鏍囪鍜屾ˉ鎺ヨ缃潵寤虹珛銆?
  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1
    ip link add link eth0 name eth0.2 type vlan id 2
    ip link add link eth0 name eth0.3 type vlan id 3

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up
    ip link set eth0.2 up
    ip link set eth0.3 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridges
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 2 pvid untagged
    bridge vlan add dev lan3 vid 3 pvid untagged

    # configure the VLANs
    ip addr add 192.0.2.1/30 dev eth0.1
    ip addr add 192.0.2.5/30 dev eth0.2
    ip addr add 192.0.2.9/30 dev eth0.3

    # bring up the bridge devices
    ip link set br0 up


**妗ユ帴**
  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0
    ip link set eth0.1 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 1 pvid untagged
    bridge vlan add dev lan3 vid 1 pvid untagged

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up

**缃戝叧**
  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1
    ip link add link eth0 name eth0.2 type vlan id 2

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up
    ip link set eth0.2 up

    # bring up the user interfaces
    ip link set wan up
    ip link set lan1 up
    ip link set lan2 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridges
    ip link set dev wan master br0
    ip link set eth0.1 master br0
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 1 pvid untagged
    bridge vlan add dev wan vid 2 pvid untagged

    # configure the VLANs
    ip addr add 192.0.2.1/30 dev eth0.2
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge devices
    ip link set br0 up

### 杞彂鏁版嵁搴擄紙FDB锛夌鐞?

鐜版湁鐨?DSA 浜ゆ崲鏈烘病鏈夊繀瑕佺殑纭欢鏀寔鏉ヤ娇妗ユ帴鐨勮蒋浠?FDB 涓庣‖浠惰〃淇濇寔鍚屾锛屽洜姝よ繖涓や釜琛ㄦ槸鍒嗗紑
绠＄悊鐨勶紙`bridge fdb show` 浼氭煡璇袱鑰咃紝骞朵笖鏍规嵁浣跨敤鐨勬槸 `self` 杩樻槸 `master` 鏍囧織锛宍`bridge fdb
add`` 鎴?`bridge fdb del`` 鍛戒护浣滅敤浜庡叾涓竴涓垨涓や釜琛ㄩ噷鐨勬潯鐩級銆?
鐩村埌鍐呮牳 v4.14锛孌SA 浠呮敮鎸佷娇鐢ㄦˉ鎺ユ梺璺搷浣滐紙杩欎簺鎿嶄綔涓嶆洿鏂拌蒋浠?FDB锛屽彧鏇存柊纭欢 FDB锛夋潵鐢辩敤鎴?绌洪棿绠＄悊妗ユ帴 FDB 鏉＄洰锛屼娇鐢?`self` 鏍囧織锛堣鏍囧織鏄彲閫夌殑锛屽彲浠ョ渷鐣ワ級銆?
  .. code-block:: sh

    bridge fdb add dev swp0 00:01:02:03:04:05 self static
    # or shorthand
    bridge fdb add dev swp0 00:01:02:03:04:05 static

鐢变簬涓€涓?bug锛孌SA 鎻愪緵鐨勬ˉ鎺ユ梺璺?FDB 瀹炵幇娌℃湁鍖哄垎 `static` 涓?`local` FDB 鏉＄洰锛坄static` 鏃ㄥ湪
琚浆鍙戯紝鑰?`local` 鏃ㄥ湪琚湰鍦扮粓缁擄紝鍗冲彂寰€涓绘満绔彛锛夈€傜浉鍙嶏紝鎵€鏈夊甫鏈?`self` 鏍囧織锛堥殣寮忔垨鏄惧紡锛?鐨?FDB 鏉＄洰閮借 DSA 褰撲綔 `static` 澶勭悊锛屽嵆浣垮畠浠疄闄呬笂鏄?`local`銆?
  .. code-block:: sh

    # This command:
    bridge fdb add dev swp0 00:01:02:03:04:05 static
    # behaves the same for DSA as this command:
    bridge fdb add dev swp0 00:01:02:03:04:05 local
    # or shorthand, because the 'local' flag is implicit if 'static' is not
    # specified, it also behaves the same as:
    bridge fdb add dev swp0 00:01:02:03:04:05

鏈€鍚庝竴鏉″懡浠ゆ槸浣跨敤妗ユ帴鏃佽矾鎿嶄綔鍚?DSA 浜ゆ崲鏈烘坊鍔犻潤鎬佹ˉ鎺?FDB 鏉＄洰鐨勪笉姝ｇ‘鏂瑰紡锛屽畠纰板阀鑳藉宸ヤ綔銆?鍏朵粬椹卞姩浼氬皢鍚屼竴鍛戒护娣诲姞鐨?FDB 鏉＄洰褰撲綔 `local` 澶勭悊锛屽洜姝や笉浼氳浆鍙戝畠锛岃繖涓?DSA 涓嶅悓銆?
鍦ㄥ唴鏍?v4.14 鍒?v5.14 涔嬮棿锛孌SA 骞惰鏀寔涓ょ鍚戜氦鎹㈡満娣诲姞妗ユ帴 FDB 鏉＄洰鐨勬ā寮忥細涓婃枃璁ㄨ鐨勬ˉ鎺?鏃佽矾锛屼互鍙婁竴绉嶄娇鐢?`master` 鏍囧織鐨勬柊妯″紡锛岃妯″紡涔熶細灏?FDB 鏉＄洰瀹夎杩涜蒋浠舵ˉ銆?
  .. code-block:: sh

    bridge fdb add dev swp0 00:01:02:03:04:05 master static

鑷唴鏍?v5.14 璧凤紝DSA 鑾峰緱浜嗕笌妗ユ帴杞欢 FDB 鏇村己鐨勯泦鎴愶紝骞朵笖瀵规ˉ鎺ユ梺璺?FDB 瀹炵幇锛堜娇鐢?`self`
鏍囧織锛夌殑鏀寔宸茶绉婚櫎銆傝繖瀵艰嚧浜嗕互涓嬪彉鍖栵細

  .. code-block:: sh

    # This is the only valid way of adding an FDB entry that is supported,
    # compatible with v4.14 kernels and later:
    bridge fdb add dev swp0 00:01:02:03:04:05 master static
    # This command is no longer buggy and the entry is properly treated as
    # 'local' instead of being forwarded:
    bridge fdb add dev swp0 00:01:02:03:04:05
    # This command no longer installs a static FDB entry to hardware:
    bridge fdb add dev swp0 00:01:02:03:04:05 static

鍥犳锛岃剼鏈紪鍐欒€呭湪澶勭悊 DSA 浜ゆ崲鏈烘帴鍙ｄ笂鐨勬ˉ鎺?FDB 鏉＄洰鏃讹紝榧撳姳浣跨敤 `master static` 杩欑粍鏍囧織銆?
### 鐢ㄦ埛绔彛鍒?CPU 绔彛鐨勪翰鍜屾€?

閫氬父锛孌SA 浜ゆ崲鏈洪€氳繃鍗曚釜浠ュお缃戞帴鍙ｈ繛鎺ュ埌涓绘満锛屼絾鍦ㄤ氦鎹㈡満鑺墖鏄垎绔嬶紙discrete锛夌殑鎯呭喌涓嬶紝纭欢
璁捐鍙兘鍏佽澶氳揪 2 涓垨鏇村绔彛杩炴帴鍒颁富鏈猴紝浠ユ彁楂樼粓缁撳悶鍚愰噺銆?
DSA 鍙互閫氳繃涓ょ鏂瑰紡鍒╃敤澶氫釜 CPU 绔彛銆傞鍏堬紝鍙互闈欐€佸湴灏嗕笌鏌愪釜鐢ㄦ埛绔彛鐩稿叧鑱旂殑缁堢粨娴侀噺鍒嗛厤
缁欐煇涓壒瀹氱殑 CPU 绔彛澶勭悊銆傝繖鏍凤紝鐢ㄦ埛绌洪棿鍙互閫氳繃鏍规嵁鍙敤鐨?CPU 绔彛鏉ュ垎鏁ｄ翰鍜屾€э紝瀹炵幇鐢ㄦ埛绔彛
涔嬮棿闈欐€佽礋杞藉潎琛＄殑鑷畾涔夌瓥鐣ャ€?
鍏舵锛屽彲浠ュ湪姣忎釜鏁版嵁鍖呯殑鍩虹涓婅€屼笉鏄潤鎬佸湴灏嗙敤鎴风鍙ｅ垎閰嶇粰 CPU 绔彛锛屼粠鑰屽湪 CPU 绔彛涔嬮棿鎵ц
璐熻浇鍧囪　銆傝繖鍙互閫氳繃灏?DSA conduit 缃簬涓€涓?LAG 鎺ュ彛锛坆onding 鎴?team锛変笅鏉ュ疄鐜般€侱SA 鐩戞帶姝?鎿嶄綔锛屽苟鍦ㄦ瀯鎴?LAG 浠庤澶囩殑銆侀潰鍚戠墿鐞?DSA conduit 鐨?CPU 绔彛涓婂垱寤鸿杞欢 LAG 鐨勯暅鍍忋€?
涓轰簡鍒╃敤澶氫釜 CPU 绔彛锛屼氦鎹㈡満鐨勫浐浠讹紙璁惧鏍戯級鎻忚堪蹇呴』浣跨敤 `ethernet` 寮曠敤/phandle 鏍囪鎵€鏈?CPU 绔彛涓庡叾 DSA conduit 涔嬮棿鐨勯摼鎺ャ€傚湪鍚姩鏃讹紝鍙細浣跨敤涓€涓崟涓€鐨?CPU 绔彛鍜?DSA conduit 鈥斺€?鍗冲浐浠舵弿杩颁腑鏁板€间笂绗竴涓甫鏈?`ethernet` 灞炴€х殑绔彛銆傜敱鐢ㄦ埛鏉ラ厤缃郴缁熶互浣夸氦鎹㈡満浣跨敤鍏朵粬 conduit銆?
DSA 浣跨敤 `rtnl_link_ops` 鏈哄埗锛堝甫鏈?"dsa" `kind`锛夋潵鍏佽鏇存敼鐢ㄦ埛绔彛鐨?DSA conduit銆俙IFLA_DSA_CONDUIT`
u32 netlink 灞炴€у寘鍚鐞嗘瘡涓敤鎴疯澶囩殑 conduit 璁惧鐨?ifindex銆侱SA conduit 蹇呴』鏄竴涓熀浜庡浐浠?鑺傜偣淇℃伅鐨勬湁鏁堝€欓€夛紝鎴栬€呬竴涓彧鍖呭惈鏈夋晥鍊欓€変綔涓轰粠璁惧鐨?LAG 鎺ュ彛銆?
浣跨敤 iproute2锛屽彲浠ヨ繘琛屼互涓嬫搷浣滐細

  .. code-block:: sh

    # See the DSA conduit in current use
    ip -d link show dev swp0
        (...)
        dsa master eth0

    # Static CPU port distribution
    ip link set swp0 type dsa master eth1
    ip link set swp1 type dsa master eth0
    ip link set swp2 type dsa master eth1
    ip link set swp3 type dsa master eth0

    # CPU ports in LAG, using explicit assignment of the DSA conduit
    ip link add bond0 type bond mode balance-xor && ip link set bond0 up
    ip link set eth1 down && ip link set eth1 master bond0
    ip link set swp0 type dsa master bond0
    ip link set swp1 type dsa master bond0
    ip link set swp2 type dsa master bond0
    ip link set swp3 type dsa master bond0
    ip -d link show dev swp0
        (...)
        dsa master bond0

    # CPU ports in LAG, relying on implicit migration of the DSA conduit
    ip link add bond0 type bond mode balance-xor && ip link set bond0 up
    ip link set eth0 down && ip link set eth0 master bond0
    ip link set eth1 down && ip link set eth1 master bond0
    ip -d link show dev swp0
        (...)
        dsa master bond0

娉ㄦ剰锛屽湪 CPU 绔彛浣嶄簬 LAG 涔嬩笅鐨勬儏鍐典笅锛屼娇鐢?`IFLA_DSA_CONDUIT` netlink 灞炴€у苟闈炰弗鏍奸渶瑕侊紝鐩稿弽锛?DSA 浼氬鍏跺綋鍓?conduit锛坄eth0`锛夌殑 `IFLA_MASTER` 灞炴€у彉鏇村仛鍑哄弽搴旓紝骞跺皢鎵€鏈夌敤鎴风鍙ｈ縼绉诲埌 `eth0`
鐨勬柊涓婂眰 `bond0`銆傜被浼煎湴锛屽綋浣跨敤 `RTM_DELLINK` 閿€姣?`bond0` 鏃讹紝DSA 浼氬皢鍏跺垎閰嶇殑鐢ㄦ埛绔彛杩佺Щ鍒?鍩轰簬鍥轰欢鎻忚堪绗﹀悎鏉′欢鐨勭涓€涓墿鐞?DSA conduit锛堝畠瀹為檯涓婁細鍥為€€鍒板惎鍔ㄩ厤缃級銆?
鍥犳锛屽湪鍏锋湁瓒呰繃 2 涓墿鐞?CPU 绔彛鐨勮缃腑锛屽彲浠ュ皢闈欐€佺殑鐢ㄦ埛鍒?CPU 绔彛鍒嗛厤涓?DSA conduit 涔嬮棿鐨?LAG 娣峰悎浣跨敤銆備笉鍙兘灏嗙敤鎴风鍙ｉ潤鎬佸垎閰嶇粰鍏锋湁浠讳綍涓婂眰鎺ュ彛锛堣繖鍖呮嫭 LAG 璁惧鈥斺€旀鏃?conduit 蹇呴』
濮嬬粓鏄 LAG锛夌殑 DSA conduit銆?
鍏佽鍦ㄨ繍琛屾椂鏇存敼鐢ㄦ埛绔彛鐨?DSA conduit锛堜互鍙?CPU 绔彛锛変翰鍜屾€э紝浠ュ厑璁告牴鎹祦閲忚繘琛屽姩鎬侀噸鏂板垎閰嶃€?
鐗╃悊 DSA conduit 鍙互闅忔椂鍔犲叆鍜岀寮€鐢ㄤ綔 DSA conduit 鐨?LAG 鎺ュ彛锛涗絾鏄紝闄ら潪璇?LAG 鎺ュ彛鑷冲皯鏈変竴涓?鐗╃悊 DSA conduit 浣滀负浠庤澶囷紝鍚﹀垯 DSA 浼氭嫆缁濆皢鍏朵綔涓?DSA conduit 鐨勬湁鏁堝€欓€夈€?