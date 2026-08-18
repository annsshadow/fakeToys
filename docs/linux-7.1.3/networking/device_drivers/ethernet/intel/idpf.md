
## idpf Linux* 鍩虹椹卞姩锛岀敤浜?Intel(R) 鍩虹璁炬柦鏁版嵁璺緞鍔熻兘锛圛nfrastructure Data Path Function锛?

Intel idpf Linux 椹卞姩銆?Copyright(C) 2023 Intel Corporation銆?

idpf 椹卞姩鍚屾椂浣滀负 Intel(R) 鍩虹璁炬柦鏁版嵁璺緞鍔熻兘锛圛nfrastructure Data Path
Function锛夌殑鐗╃悊鍔熻兘锛圥F锛夐┍鍔ㄥ拰铏氭嫙鍔熻兘锛圴F锛夐┍鍔ㄣ€?
鍙互浣跨敤 ethtool銆乴spci 鍜?ip 鑾峰彇椹卞姩淇℃伅銆?
鍏充簬纭欢瑕佹眰鐨勭浉鍏抽棶棰橈紝璇峰弬闃呴殢浣犵殑 Intel 閫傞厤鍣ㄦ彁渚涚殑鏂囨。銆傛墍鍒楀嚭鐨勬墍鏈?纭欢瑕佹眰鍧囬€傜敤浜庡湪 Linux 涓嬩娇鐢ㄣ€?
## 璇嗗埆浣犵殑閫傞厤鍣?
鍏充簬濡備綍璇嗗埆浣犵殑閫傞厤鍣紝浠ュ強鑾峰彇鏈€鏂扮殑 Intel 缃戠粶椹卞姩锛岃鍙傞槄 Intel 鏀寔缃戠珯锛?http://www.intel.com/support

## 闄勫姞鐗规€т笌閰嶇疆


### ethtool

椹卞姩鍒╃敤 ethtool 鎺ュ彛杩涜椹卞姩閰嶇疆鍜岃瘖鏂紝浠ュ強鏄剧ず缁熻淇℃伅銆傛鍔熻兘闇€瑕佹渶鏂扮殑
ethtool 鐗堟湰銆傚鏋滀綘杩樻病鏈夛紝鍙互鍦ㄤ互涓嬪湴鍧€鑾峰彇锛?https://kernel.org/pub/software/network/ethtool/

### 鏌ョ湅閾捐矾娑堟伅

濡傛灉鍙戣鐗堥檺鍒剁郴缁熸秷鎭紝閾捐矾娑堟伅灏嗕笉浼氭樉绀哄埌鎺у埗鍙般€備负浜嗙湅鍒扮綉缁滈┍鍔ㄧ殑閾捐矾
娑堟伅锛屾墽琛?```

  # dmesg -n 8

```
   璇ヨ缃笉浼氬湪閲嶅惎鍚庝繚鐣欍€?
### 宸ㄥ抚锛圝umbo Frames锛?
閫氳繃鎶婃渶澶т紶杈撳崟鍏冿紙MTU锛夋敼涓哄ぇ浜庨粯璁ゅ€?1500 鐨勫€兼潵鍚敤宸ㄥ抚鏀寔銆?
浣跨敤 ip 鍛戒护澧炲ぇ MTU 澶у皬銆備緥濡傦紝杈撳叆浠ヤ笅鍛戒护
```

  # ip link set mtu 9000 dev <ethX>
  # ip link set up dev <ethX>

```
   宸ㄥ抚鐨勬渶澶?MTU 璁剧疆涓?9706銆傝繖瀵瑰簲浜?9728 瀛楄妭鐨勬渶澶у法甯уぇ灏忋€?
   璇ラ┍鍔ㄥ皢灏濊瘯浣跨敤澶氫釜椤靛ぇ灏忕殑缂撳啿鍖烘潵鎺ユ敹姣忎釜宸ㄥ抚鏁版嵁鍖呫€傝繖搴旀湁鍔╀簬鍦ㄥ垎閰?   鎺ユ敹鏁版嵁鍖呮椂閬垮厤缂撳啿鍖哄尞涔忛棶棰樸€?
   褰撲綘浣跨敤宸ㄥ抚鏃讹紝涓㈠寘鍙兘瀵瑰悶鍚愰噺鏈夋洿澶х殑褰卞搷銆傚鏋滃湪鍚敤宸ㄥ抚鍚庤瀵熷埌鎬ц兘
   涓嬮檷锛屽惎鐢ㄦ祦鎺у彲鑳戒細缂撹В璇ラ棶棰樸€?
## 鎬ц兘浼樺寲

椹卞姩榛樿鍊兼棬鍦ㄩ€傚簲鍚勭鍚勬牱鐨勫伐浣滆礋杞斤紝浣嗗鏋滈渶瑕佽繘涓€姝ヤ紭鍖栵紝鎴戜滑寤鸿瀵逛互涓?璁剧疆杩涜璇曢獙銆?
### 涓柇閫熺巼闄愬埗

璇ラ┍鍔ㄦ敮鎸佷竴绉嶄负閫氱敤宸ヤ綔璐熻浇璋冧紭鐨勮嚜閫傚簲涓柇鑺傛祦閫熺巼锛圛TR锛夋満鍒躲€傜敤鎴峰彲浠ラ€氳繃
ethtool 鑷畾涔夌壒瀹氬伐浣滆礋杞界殑涓柇閫熺巼鎺у埗锛岃皟鏁翠腑鏂箣闂寸殑寰鏁般€?
```
  # ethtool -C <ethX> adaptive-rx off adaptive-tx off

```
涓轰簡鏇翠綆鐨?CPU 鍗犵敤锛? - 绂佺敤鑷€傚簲 ITR 骞堕檷浣?Rx 鍜?Tx 涓柇銆備笅闈㈢殑绀轰緥褰卞搷鎸囧畾鎺ュ彛鐨勬瘡涓槦鍒椼€?
 - 灏?rx-usecs 鍜?tx-usecs 璁句负 80 浼氭妸涓柇闄愬埗鍦ㄥぇ绾?```
     # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 80
     tx-usecs 80

```
涓轰簡鏇翠綆鐨勫欢杩燂細
 - 閫氳繃灏?rx-usecs 鍜?tx-usecs 璁句负 0 鏉ョ鐢ㄨ嚜閫傚簲 ITR 鍜?ITR
```
     # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 0
     tx-usecs 0

```
姣忛槦鍒椾腑鏂€熺巼璁剧疆锛? - 浠ヤ笅绀轰緥閽堝闃熷垪 1 鍜?3锛屼絾浣犲彲浠ヨ皟鏁村叾瀹冮槦鍒椼€?
 - 瑕佺鐢?Rx 鑷€傚簲 ITR 骞跺皢闈欐€?Rx ITR 璁句负 10 寰锛屾墽琛?```
     # ethtool --per-queue <ethX> queue_mask 0xa --coalesce adaptive-rx off
     rx-usecs 10

 - 瑕佹樉绀洪槦鍒?1 鍜?3 褰撳墠鐨勫悎骞讹紙coalesce锛夎缃細锛?
     # ethtool --per-queue <ethX> queue_mask 0xa --show-coalesce



```
### 铏氭嫙鍖栫幆澧?
闄ゆ湰鑺備腑鐨勫叾瀹冨缓璁锛屼互涓嬪唴瀹瑰彲鑳芥湁鍔╀簬浼樺寲铏氭嫙鏈轰腑鐨勬€ц兘銆?
 - 鍦?VM 涓娇鐢ㄩ€傚綋鐨勬満鍒讹紙vcpupin锛夛紝灏?CPU 鍥哄畾鍒板悇涓?LCPU锛岀‘淇濅娇鐢ㄥ寘鍚湪
   璁惧 local_cpulist 涓殑涓€缁?CPU锛?sys/class/net/<ethX>/device/local_cpulist銆?
 - 鍦?VM 涓厤缃敖鍙兘澶氱殑 Rx/Tx 闃熷垪锛堝弬瑙?idpf 椹卞姩
```
     # ethtool -L <virt_interface> rx <max> tx <max>


```
## 鏀寔

鍏充簬涓€鑸俊鎭紝璇疯闂?Intel 鏀寔缃戠珯锛?http://www.intel.com/support/

濡傛灉鍙戠幇宸插彂甯冩簮浠ｇ爜鍦ㄥ彈鏀寔鐨勫唴鏍稿拰鍙楁敮鎸佺殑閫傞厤鍣ㄤ笂瀛樺湪闂锛岃灏嗕笌璇ラ棶棰?鐩稿叧鐨勫叿浣撲俊鎭彂閫佽嚦 intel-wired-lan@lists.osuosl.org銆?
## 鍟嗘爣

Intel 鏄?Intel Corporation 鎴栧叾瀛愬叕鍙稿湪缇庡浗鍜?鎴栧叾瀹冨浗瀹?鍦板尯鐨勫晢鏍囨垨娉ㄥ唽鍟嗘爣銆?
- 鍏跺畠鍚嶇О鍜屽搧鐗屽彲鑳借瑙嗕负浠栦汉鐨勮储浜с€?