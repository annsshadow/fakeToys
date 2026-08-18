
## Intel(R) 浠ュお缃戞帶鍒跺櫒 800 绯诲垪 Linux 鍩虹椹卞姩


Intel ice Linux 椹卞姩銆?鐗堟潈鎵€鏈?c) 2018-2021 鑻辩壒灏斿叕鍙搞€?
## 鐩綍


- 姒傝堪
- 璇嗗埆鎮ㄧ殑閫傞厤鍣?- 閲嶈璇存槑
- 闄勫姞鍔熻兘涓庨厤缃?- 鎬ц兘浼樺寲


姝ら┍鍔ㄥ搴旂殑铏氭嫙鍔熻兘锛圴F锛夐┍鍔ㄤ负 iavf銆?
椹卞姩淇℃伅鍙€氳繃 ethtool 鍜?lspci 鑾峰彇銆?
鏈夊叧纭欢瑕佹眰鐨勭枒闂紝璇峰弬闃呴殢 Intel 閫傞厤鍣ㄦ彁渚涚殑鏂囨。銆傚垪鍑虹殑鎵€鏈夌‖浠惰姹傚潎閫傜敤浜?Linux 鐜銆?
鏈┍鍔ㄦ敮鎸?XDP锛圗xpress Data Path锛屽揩閫熸暟鎹矾寰勶級鍜?AF_XDP 闆舵嫹璐濄€傛敞鎰忓浜庡抚澶у皬瓒呰繃 3KB 鐨勬儏鍐碉紝XDP 浼氳绂佺敤銆?

## 璇嗗埆鎮ㄧ殑閫傞厤鍣?
鏈夊叧濡備綍璇嗗埆閫傞厤鍣ㄤ互鍙婅幏鍙栨渶鏂?Intel 缃戠粶椹卞姩鐨勪俊鎭紝璇峰弬闃?Intel 鏀寔缃戠珯锛?https://www.intel.com/support


## 閲嶈璇存槑


### 鎺ユ敹鍘嬪姏鍙兘瀵艰嚧涓㈠寘

鍩轰簬 Intel(R) 浠ュお缃戞帶鍒跺櫒 800 绯诲垪鐨勮澶囪璁＄敤浜庡湪 PCIe 鍜?DMA 浜嬪姟鏈熼棿瀹瑰繊鏈夐檺鐨勭郴缁熷欢杩熴€傚鏋滆繖浜涗簨鍔¤€楁椂瓒呰繃鍙蹇嶇殑寤惰繜锛屽氨浼氬奖鍝嶆暟鎹寘鍦ㄨ澶囧強鐩稿叧鍐呭瓨涓殑缂撳啿鏃堕暱锛屽彲鑳藉鑷翠涪鍖呫€傚湪鏍囧噯宸ヤ綔璐熻浇涓嬶紝杩欎簺涓㈠寘閫氬父涓嶄細瀵瑰悶鍚愰噺鍜屾€ц兘浜х敓鏄庢樉褰卞搷銆?
濡傛灉杩欎簺涓㈠寘浼间箮褰卞搷浜嗘偍鐨勫伐浣滆礋杞斤紝浠ヤ笅鎺柦鍙兘鏀瑰杽鎯呭喌锛?
1) 纭繚绯荤粺鐗╃悊鍐呭瓨澶勪簬楂樻€ц兘閰嶇疆锛屽骞冲彴渚涘簲鍟嗘墍寤鸿銆傚父瑙佺殑寤鸿鏄墍鏈夐€氶亾鍧囨彃婊″崟鏉?DIMM 妯″潡銆?2) 鍦ㄧ郴缁熺殑 BIOS/UEFI 璁剧疆涓€夋嫨"鎬ц兘"閰嶇疆妗ｃ€?3) 鎮ㄧ殑鍙戣鐗堝彲鑳芥彁渚涜濡?"tuned" 涔嬬被鐨勫伐鍏凤紝鍙府鍔╄皟鏁村唴鏍歌缃紝涓轰笉鍚屽伐浣滆礋杞借幏寰楁洿濂界殑鏍囧噯璁剧疆銆?

### 閰嶇疆 SR-IOV 浠ユ彁鍗囩綉缁滃畨鍏ㄦ€?
鍦ㄨ櫄鎷熷寲鐜涓紝鍦ㄦ敮鎸?SR-IOV 鐨?Intel(R) 浠ュお缃戠綉缁滈€傞厤鍣ㄤ笂锛岃櫄鎷熷姛鑳斤紙VF锛夊彲鑳戒細閬彈鎭舵剰琛屼负銆傝蒋浠剁敓鎴愮殑浜屽眰甯э紝濡?IEEE 802.3x锛堥摼璺祦鎺э級銆両EEE 802.1Qbb锛堝熀浜庝紭鍏堢骇鐨勬祦鎺э級浠ュ強鍏朵粬鍚岀被鍨嬪抚锛屾槸棰勬湡涔嬪鐨勶紝骞跺彲鑳芥壖鍒朵富鏈轰笌铏氭嫙浜ゆ崲鏈轰箣闂寸殑娴侀噺锛岄檷浣庢€ц兘銆備负瑙ｅ喅杩欎釜闂骞剁‘淇濅笌鎰忓娴侀噺娴佺殑闅旂锛岃浠?PF 鐨勭鐞嗘帴鍙ｉ厤缃墍鏈夊惎鐢?SR-IOV 鐨勭鍙ｈ繘琛?VLAN 鏍囪銆傝閰嶇疆鍙涪寮冩剰澶栦笖鍙兘鎭舵剰鐨勫抚銆?
鏈夊叧閰嶇疆璇存槑锛岃鍙傞槄鏈枃妗ｅ悗鏂囩殑"鍦ㄥ惎鐢?SR-IOV 鐨勯€傞厤鍣ㄧ鍙ｄ笂閰嶇疆 VLAN 鏍囪"銆?

### 鑻ョ粦瀹氫簡娲诲姩铏氭嫙鏈虹殑 VF 缁戝畾鍒扮鍙ｏ紝璇峰嬁鍗歌浇绔彛椹卞姩

濡傛灉鏌愯櫄鎷熷姛鑳斤紙VF锛夌粦瀹氫簡娲诲姩铏氭嫙鏈猴紙VM锛夛紝璇峰嬁鍗歌浇璇ョ鍙ｇ殑椹卞姩銆傝繖鏍峰仛浼氬鑷寸鍙ｇ湅浼兼寕璧枫€備竴鏃?VM 鍏抽棴鎴栦互鍏朵粬鏂瑰紡閲婃斁璇?VF锛屽懡浠ゆ墠浼氬畬鎴愩€?

## 闄勫姞鍔熻兘涓庨厤缃?

### ethtool

椹卞姩浣跨敤 ethtool 鎺ュ彛杩涜椹卞姩閰嶇疆鍜岃瘖鏂紝骞舵樉绀虹粺璁′俊鎭€傛鍔熻兘闇€瑕佹渶鏂扮増鏈殑 ethtool銆備笅杞藉湴鍧€锛?https://kernel.org/pub/software/network/ethtool/

娉ㄦ剰锛氱敱浜庤澶囦細鍓ョ 4 瀛楄妭 CRC锛宔thtool 鐨?rx_bytes 鍊间笌 Netdev 鐨?rx_bytes 鍊间笉涓€鑷淬€備袱鑰?rx_bytes 鍊肩殑宸€间负 Rx 鏁版嵁鍖呮暟閲忕殑 4 鍊嶃€備緥濡傦紝鑻?Rx 鏁版嵁鍖呬负 10 涓笖 Netdev锛堣蒋浠剁粺璁★級鏄剧ず rx_bytes 涓?"X"锛屽垯 ethtool锛堢‖浠剁粺璁★級灏嗘樉绀?rx_bytes 涓?"X+40"锛? 瀛楄妭 CRC 脳 10 涓暟鎹寘锛夈€?
### ethtool 澶嶄綅

椹卞姩鏀寔 3 绉嶇被鍨嬬殑澶嶄綅锛?
- PF 澶嶄綅 - 浠呭浣嶄笌缁欏畾 PF 鐩稿叧鐨勭粍浠讹紝涓嶅奖鍝嶅叾浠?PF

- CORE 澶嶄綅 - 鏁翠釜閫傞厤鍣ㄥ彈褰卞搷锛屽浣嶆墍鏈?PF

- GLOBAL 澶嶄綅 - 涓?CORE 鐩稿悓锛屼絾 mac 鍜?phy 缁勪欢涔熶細琚噸鏂板垵濮嬪寲

杩欎簺瀵瑰簲浜?ethtool 澶嶄綅鏍囧織濡備笅锛?
- PF 澶嶄綅锛?
  # ethtool --reset <ethX> irq dma filter offload

- CORE 澶嶄綅锛?
  # ethtool --reset <ethX> irq-shared dma-shared filter-shared offload-shared \
  ram-shared

- GLOBAL 澶嶄綅锛?
  # ethtool --reset <ethX> irq-shared dma-shared filter-shared offload-shared \
  mac-shared phy-shared ram-shared

鍦?switchdev 妯″紡涓嬶紝鎮ㄥ彲浠ヤ娇鐢ㄧ鍙ｄ唬琛紙port representor锛夊浣?VF锛?
  # ethtool --reset <repr> irq dma filter offload


### 鏌ョ湅閾捐矾娑堟伅

濡傛灉鍙戣鐗堥檺鍒朵簡绯荤粺娑堟伅锛岄摼璺秷鎭皢涓嶄細鏄剧ず鍒版帶鍒跺彴銆傝鍦ㄧ綉缁滀笂鏌ョ湅缃戠粶椹卞姩閾捐矾娑堟伅

```

  # dmesg -n 8

```
娉ㄦ剰锛氭璁剧疆涓嶄細鍦ㄩ噸鍚悗淇濈暀銆?

### 鍔ㄦ€佽澶囦釜鎬у寲锛圖DP锛?
鍔ㄦ€佽澶囦釜鎬у寲锛圖DP锛夊厑璁告偍鍦ㄨ繍琛屾椂閫氳繃鍚戣澶囧簲鐢ㄩ厤缃枃浠跺寘鏉ユ洿鏀硅澶囩殑鏁版嵁鍖呭鐞嗘祦姘寸嚎銆備緥濡傦紝閰嶇疆鏂囦欢鍙敤浜庢坊鍔犲鏂板崗璁殑鏀寔銆佹洿鏀圭幇鏈夊崗璁垨鏇存敼榛樿璁剧疆銆侱DP 閰嶇疆鏂囦欢涔熷彲浠ュ湪涓嶉噸鍚郴缁熺殑鎯呭喌涓嬪洖婊氥€?
DDP 鍖呭湪璁惧鍒濆鍖栨湡闂村姞杞姐€傞┍鍔ㄤ細鍦ㄥ浐浠舵牴鐩綍锛堥€氬父涓?`/lib/firmware/` 鎴?`/lib/firmware/updates/`锛変腑鏌ユ壘 `intel/ice/ddp/ice.pkg`锛屽苟妫€鏌ュ叾鏄惁鍖呭惈鏈夋晥鐨?DDP 鍖呮枃浠躲€?
娉ㄦ剰锛氭偍鐨勫彂琛岀増寰堝彲鑳藉凡鎻愪緵鏈€鏂扮殑 DDP 鏂囦欢锛屼絾濡傛灉缂哄皯 ice.pkg锛屾偍鍙互鍦?linux-firmware 浠撳簱鎴栦粠 intel.com 鎵惧埌瀹冦€?
濡傛灉椹卞姩鏃犳硶鍔犺浇 DDP 鍖咃紝璁惧灏嗚繘鍏ュ畨鍏ㄦā寮忥紙Safe Mode锛夈€傚畨鍏ㄦā寮忎細绂佺敤楂樼骇鍜屾€ц兘鐗规€э紝浠呮敮鎸佸熀鏈祦閲忓拰鏈€灏忓姛鑳斤紝渚嬪鏇存柊 NVM 鎴栦笅杞芥柊椹卞姩鎴?DDP 鍖呫€傚畨鍏ㄦā寮忎粎閫傜敤浜庡彈褰卞搷鐨勭墿鐞嗗姛鑳斤紝涓嶄細褰卞搷浠讳綍鍏朵粬 PF銆傛湁鍏?DDP 鍜屽畨鍏ㄦā寮忕殑鏇村璇︾粏淇℃伅锛岃鍙傞槄"Intel(R) 浠ュお缃戦€傞厤鍣ㄥ拰璁惧鐢ㄦ埛鎸囧崡"銆?
娉ㄦ剰锛?
- 濡傛灉鎮ㄩ亣鍒?DDP 鍖呮枃浠剁殑闂锛屽彲鑳介渶瑕佷笅杞芥洿鏂扮殑椹卞姩鎴?DDP 鍖呮枃浠躲€傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃呮棩蹇楁秷鎭€?
- ice.pkg 鏂囦欢鏄寚鍚戦粯璁?DDP 鍖呮枃浠剁殑绗﹀彿閾炬帴銆?
- 濡傛灉浠讳綍 PF 椹卞姩宸插姞杞斤紝鎮ㄦ棤娉曟洿鏂?DDP 鍖呫€傝瑕嗙洊鏌愪釜鍖咃紝璇峰嵏杞芥墍鏈?PF锛岀劧鍚庝娇鐢ㄦ柊鍖呴噸鏂板姞杞介┍鍔ㄣ€?
- 姣忎釜璁惧鍙湁绗竴涓姞杞界殑 PF 鎵嶈兘涓嬭浇璇ヨ澶囩殑鍖呫€?
鎮ㄥ彲浠ュ湪鍚屼竴绯荤粺涓负涓嶅悓鐨勭墿鐞嗚澶囧畨瑁呯壒瀹氱殑 DDP 鍖呮枃浠躲€傝瀹夎鐗瑰畾鐨?DDP 鍖呮枃浠讹細

1. 涓嬭浇鎮ㄨ澶囨墍闇€鐨?DDP 鍖呮枃浠躲€?
2. 灏嗘枃浠堕噸鍛藉悕涓?ice-xxxxxxxxxxxxxxxx.pkg锛屽叾涓?'xxxxxxxxxxxxxxxx' 鏄涓嬭浇璇ュ寘鐨勮澶囧敮涓€鐨?64 浣?PCI Express 璁惧搴忓垪鍙凤紙鍗佸叚杩涘埗锛夈€傛枃浠跺悕蹇呴』鍖呭惈瀹屾暣鐨勫簭鍒楀彿锛堝寘鎷墠瀵奸浂锛変笖鍏ㄩ儴灏忓啓銆備緥濡傦紝鑻?64 浣嶅簭鍒楀彿涓?b887a3ffffca0568锛屽垯鏂囦欢鍚嶅簲涓?ice-b887a3ffffca0568.pkg銆?
   瑕佷粠 PCI 鎬荤嚎鍦板潃鏌ユ壘搴忓垪鍙凤紝鍙互浣跨敤浠ヤ笅鍛戒护锛?
```

     # lspci -vv -s af:00.0 | grep -i Serial
     Capabilities: [150 v1] Device Serial Number b8-87-a3-ff-ff-ca-05-68

   鎮ㄥ彲浠ヤ娇鐢ㄤ互涓嬪懡浠ゅ皢搴忓垪鍙锋牸寮忓寲锛堝幓鎺夌煭妯嚎锛夛細

     # lspci -vv -s af:00.0 | grep -i Serial | awk '{print $7}' | sed s/-//g
     b887a3ffffca0568

```
3. 灏嗛噸鍛藉悕鍚庣殑 DDP 鍖呮枃浠跺鍒跺埌 `/lib/firmware/updates/intel/ice/ddp/`銆傚鏋滆鐩綍灏氫笉瀛樺湪锛岃鍦ㄥ鍒舵枃浠跺墠鍒涘缓瀹冦€?
4. 鍗歌浇璁惧涓婃墍鏈夌殑 PF銆?
5. 浣跨敤鏂板寘閲嶆柊鍔犺浇椹卞姩銆?
娉ㄦ剰锛氳澶囩壒瀹氱殑 DDP 鍖呮枃浠剁殑瀛樺湪浼氳鐩栭粯璁?DDP 鍖呮枃浠讹紙ice.pkg锛夌殑鍔犺浇銆?

### Intel(R) 浠ュお缃戞祦瀵煎悜鍣?
Intel 浠ュお缃戞祦瀵煎悜鍣ㄦ墽琛屼互涓嬩换鍔★細

- 鏍规嵁鏁版嵁娴佸皢鎺ユ敹鏁版嵁鍖呭鍚戜笉鍚岄槦鍒?- 瀹炵幇瀵瑰钩鍙颁腑鏁版嵁娴佽矾鐢辩殑绱у瘑鎺у埗
- 灏嗘祦涓?CPU 鏍稿績鍖归厤浠ュ疄鐜版祦浜插拰

娉ㄦ剰锛氭湰椹卞姩鏀寔浠ヤ笅娴佺被鍨嬶細

- IPv4
- TCPv4
- UDPv4
- SCTPv4
- IPv6
- TCPv6
- UDPv6
- SCTPv6

姣忕娴佺被鍨嬫敮鎸?IP 鍦板潃锛堟簮鎴栫洰鐨勶級涓?UDP/TCP/SCTP 绔彛锛堟簮鍜岀洰鐨勶級鐨勬湁鏁堢粍鍚堛€傛偍鍙互鎻愪緵浠呮簮 IP 鍦板潃銆佹簮 IP 鍦板潃鍔犵洰鐨勭鍙ｏ紝鎴栬繖鍥涗釜鍙傛暟涓换鎰忎竴涓垨澶氫釜鐨勭粍鍚堛€?
娉ㄦ剰锛氭湰椹卞姩鍏佽鎮ㄤ娇鐢?ethtool 鐨?user-def 鍜?mask 瀛楁锛屽熀浜庣敤鎴峰畾涔夌殑鍙屽瓧鑺傛ā寮忓拰鍋忕Щ鏉ヨ繃婊ゆ祦閲忋€傜敤鎴峰畾涔夌殑鐏垫椿杩囨护鍣ㄤ粎鏀寔 L3 鍜?L4 娴佺被鍨嬨€傚浜庣粰瀹氱殑娴佺被鍨嬶紝鍦ㄦ洿鏀硅緭鍏ラ泦锛堥拡瀵硅娴佺被鍨嬶級涔嬪墠锛屽繀椤诲厛娓呴櫎鎵€鏈?Intel 浠ュお缃戞祦瀵煎悜鍣ㄨ繃婊ゅ櫒銆?

### 娴佸鍚戝櫒杩囨护鍣?
娴佸鍚戝櫒杩囨护鍣ㄧ敤浜庡鍚戜笌鎸囧畾鐗瑰緛鍖归厤鐨勬暟鎹祦銆傚畠浠€氳繃 ethtool 鐨?ntuple 鎺ュ彛鍚敤銆傝鍚敤

```

  # ethtool -K <ethX> ntuple <off|on>

```
娉ㄦ剰锛氬綋鎮ㄧ鐢?ntuple 杩囨护鍣ㄦ椂锛屾墍鏈夌敤鎴风紪绋嬬殑杩囨护鍣ㄩ兘浼氫粠椹卞姩缂撳瓨鍜岀‖浠朵腑娓呴櫎銆傞噸鏂板惎鐢?ntuple 鏃讹紝蹇呴』閲嶆柊娣诲姞鎵€鏈夐渶瑕佺殑杩囨护鍣ㄣ€?
```

  # ethtool -u <ethX>

```
```

  # ethtool -U <ethX> flow-type <type> src-ip <ip> [m <ip_mask>] dst-ip <ip>
  [m <ip_mask>] src-port <port> [m <port_mask>] dst-port <port> [m <port_mask>]
  action <queue>

  Where:
    <ethX> - the Ethernet device to program
    <type> - can be ip4, tcp4, udp4, sctp4, ip6, tcp6, udp6, sctp6
    <ip> - the IP address to match on
    <ip_mask> - the IPv4 address to mask on
              NOTE: These filters use inverted masks.
    <port> - the port number to match on
    <port_mask> - the 16-bit integer for masking
              NOTE: These filters use inverted masks.
    <queue> - the queue to direct traffic toward (-1 discards the
              matched traffic)

```
```

  # ethtool -U <ethX> delete <N>

  Where <N> is the filter ID displayed when printing all the active filters,
  and may also have been specified using "loc <N>" when adding the filter.

```
绀轰緥锛?
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 src-port 2000 dst-port 2001 action 2 [loc 1]

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 action 2 [loc 1]

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 user-def 0x4FFFF action 2 [loc 1]

  where the value of the user-def field contains the offset (4 bytes) and
  the pattern (0xffff).

```
瑕佸尮閰嶄粠 192.168.0.1銆佺鍙?5300 鍙戝嚭銆佸畾鍚戝埌 192.168.0.5 鐨?TCP 娴侀噺锛?
```

  # ethtool -U enp130s0 flow-type tcp4 src-ip 192.168.0.1 dst-ip 192.168.0.5
  src-port 5300 dst-port 80 action 7

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.0.0 m 0.255.255.255 dst-ip
  192.168.5.12 src-port 12600 dst-port 31 action 12

```
娉ㄦ剰锛?
瀵逛簬姣忎釜娴佺被鍨嬶紝缂栫▼鐨勮繃婊ゅ櫒蹇呴』鍏ㄩ儴鍏锋湁鐩稿悓鐨勫尮閰?
```

  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.5 src-port 55 action 10

```
鐒惰€岋紝鍙戝嚭鎺ヤ笅鏉ョ殑涓ゆ潯鍛戒护鏄笉鍙帴鍙楃殑锛屽洜涓虹涓€鏉?
```

  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  # ethtool -U enp130s0 flow-type ip4 dst-ip 192.168.0.5 src-port 55 action 10

```
绗簩鏉″懡浠ゅ皢澶辫触骞舵姤閿欍€傛偍鍙互浣跨敤鐩稿悓瀛楁銆佷笉鍚屽€肩紪绋嬪涓繃婊ゅ櫒锛屼絾鍦ㄤ竴涓澶囦笂锛屾偍涓嶈兘缂栫▼涓や釜鍏锋湁涓嶅悓鍖归厤瀛楁鐨?tcp4 杩囨护鍣ㄣ€?
ice 椹卞姩涓嶆敮鎸佸瀛楁鐨勫瓙閮ㄥ垎杩涜鍖归厤锛屽洜姝や笉鏀寔閮ㄥ垎鎺╃爜瀛楁銆?

### 鐏垫椿瀛楄妭娴佸鍚戝櫒杩囨护鍣?
椹卞姩杩樻敮鎸佸尮閰嶆暟鎹寘杞借嵎涓殑鐢ㄦ埛瀹氫箟鏁版嵁銆傛鐏垫椿鏁版嵁閫氳繃 ethtool 鍛戒护鐨?"user-def" 瀛楁浠ヤ笅鍒楁柟寮忔寚瀹氾細


    ============================== ============================
    `31    28    24    20    16` `15    12    8    4    0`
    `offset into packet payload` `2 bytes of flexible data`
    ============================== ============================

渚嬪锛?
```

  ... user-def 0x4FFFF ...

```
鎸囩ず杩囨护鍣ㄥ湪杞借嵎涓煡鎵?4 瀛楄妭锛屽苟灏嗚鍊间笌 0xFFFF 鍖归厤銆傚亸绉诲熀浜庤浇鑽风殑璧峰浣嶇疆锛岃€岄潪鏁版嵁鍖呯殑璧峰浣嶇疆銆傚洜姝?
```

  flow-type tcp4 ... user-def 0x8BEAF ...

```
灏嗗尮閰?TCP/IPv4 杞借嵎涓 8 瀛楄妭澶勫€间负 0xBEAF 鐨?TCP/IPv4 鏁版嵁鍖呫€?
娉ㄦ剰 ICMP 澶撮儴琚В鏋愪负 4 瀛楄妭澶撮儴鍜?4 瀛楄妭杞借嵎銆傚洜姝よ鍖归厤杞借嵎鐨勭涓€涓瓧鑺傦紝瀹為檯涓婂繀椤荤粰鍋忕Щ鍔?4 瀛楄妭銆傚彟璇锋敞鎰忥紝ip4 杩囨护鍣ㄥ悓鏃跺尮閰?ICMP 甯т互鍙婂師濮嬶紙鏈煡锛塱p4 甯э紝鍏惰浇鑽峰皢鏄?IP4 甯х殑 L3 杞借嵎銆?
鏈€澶у亸绉讳负 64銆傜‖浠跺彧浼氫粠杞借嵎涓鍙栨渶澶?64 瀛楄妭鏁版嵁銆傚亸绉诲繀椤讳负鍋舵暟锛屽洜涓虹伒娲绘暟鎹负 2 瀛楄妭闀匡紝涓斿繀椤讳笌鏁版嵁鍖呰浇鑽风殑瀛楄妭 0 瀵归綈銆?
鐢ㄦ埛瀹氫箟鐨勭伒娲诲亸绉讳篃琚涓鸿緭鍏ラ泦鐨勪竴閮ㄥ垎锛屼笉鑳介拡瀵瑰悓涓€绫诲瀷鐨勫涓繃婊ゅ櫒鍗曠嫭缂栫▼銆備絾鏄紝鐏垫椿鏁版嵁涓嶅睘浜庤緭鍏ラ泦锛屽涓繃婊ゅ櫒鍙互浣跨敤鐩稿悓鍋忕Щ浣嗗尮閰嶄笉鍚屾暟鎹€?

### RSS 鍝堝笇娴?
鍏佽鎮ㄤ负姣忎釜娴佺被鍨嬭缃搱甯屽瓧鑺傦紝浠ュ強鎺ユ敹绔缉鏀撅紙RSS锛夊搱甯屽瓧鑺傞厤缃殑涓€涓垨澶氫釜閫夐」缁勫悎銆?
```

  # ethtool -N <ethX> rx-flow-hash <type> <option>

  Where <type> is:
    tcp4    signifying TCP over IPv4
    udp4    signifying UDP over IPv4
    gtpc4   signifying GTP-C over IPv4
    gtpc4t  signifying GTP-C (include TEID) over IPv4
    gtpu4   signifying GTP-U over IPV4
    gtpu4e  signifying GTP-U and Extension Header over IPV4
    gtpu4u  signifying GTP-U PSC Uplink over IPV4
    gtpu4d  signifying GTP-U PSC Downlink over IPV4
    tcp6    signifying TCP over IPv6
    udp6    signifying UDP over IPv6
    gtpc6   signifying GTP-C over IPv6
    gtpc6t  signifying GTP-C (include TEID) over IPv6
    gtpu6   signifying GTP-U over IPV6
    gtpu6e  signifying GTP-U and Extension Header over IPV6
    gtpu6u  signifying GTP-U PSC Uplink over IPV6
    gtpu6d  signifying GTP-U PSC Downlink over IPV6
  And <option> is one or more of:
    s     Hash on the IP source address of the Rx packet.
    d     Hash on the IP destination address of the Rx packet.
    f     Hash on bytes 0 and 1 of the Layer 4 header of the Rx packet.
    n     Hash on bytes 2 and 3 of the Layer 4 header of the Rx packet.
    e     Hash on GTP Packet on TEID (4bytes) of the Rx packet.


```
### 鍔犻€熸帴鏀舵祦瀵煎悜锛坅RFS锛?
鍩轰簬 Intel(R) 浠ュお缃戞帶鍒跺櫒 800 绯诲垪鐨勮澶囧湪 PF 涓婃敮鎸佸姞閫熸帴鏀舵祦瀵煎悜锛坅RFS锛夈€俛RFS 鏄竴绉嶈礋杞藉潎琛℃満鍒讹紝鍏佽鎮ㄥ皢鏁版嵁鍖呭鍚戣繍琛屾垨娑堣垂璇ユ祦涓暟鎹寘鐨勫悓涓€ CPU銆?
娉ㄦ剰锛?
- aRFS 闇€瑕侀€氳繃 ethtool 鍚敤 ntuple 杩囨护銆?- aRFS 鏀寔浠呴檺浜庝互涓嬫暟鎹寘绫诲瀷锛?
    - IPv4 鍜?IPv6 涓婄殑 TCP
    - IPv4 鍜?IPv6 涓婄殑 UDP
    - 闈炲垎鐗囨暟鎹寘

- aRFS 浠呮敮鎸佹祦瀵煎悜鍣ㄨ繃婊ゅ櫒锛屽叾鐢辨簮/鐩殑 IP 鍦板潃鍜屾簮/鐩殑绔彛缁勬垚銆?- aRFS 鍜?ethtool 鐨?ntuple 鎺ュ彛閮戒娇鐢ㄨ澶囩殑娴佸鍚戝櫒銆俛RFS 鍜?ntuple 鐗规€у彲浠ュ叡瀛橈紝浣嗗鏋?aRFS 涓?ntuple 璇锋眰涔嬮棿瀛樺湪鍐茬獊锛屽彲鑳戒細閬囧埌鎰忓缁撴灉銆傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃?Intel(R) 浠ュお缃戞祦瀵煎悜鍣?銆?
璁剧疆 aRFS锛?
1. 浣跨敤 ethtool 鍚敤 Intel 浠ュお缃戞祦瀵煎悜鍣ㄥ拰 ntuple 杩囨护鍣ㄣ€?
```

   # ethtool -K <ethX> ntuple on

```
2. 璁剧疆鍏ㄥ眬娴佽〃涓殑鏉＄洰鏁般€備緥濡傦細

```

   # NUM_RPS_ENTRIES=16384
   # echo $NUM_RPS_ENTRIES > /proc/sys/net/core/rps_sock_flow_entries

```
3. 璁剧疆姣忛槦鍒楁祦琛ㄤ腑鐨勬潯鐩暟銆備緥濡傦細

```

   # NUM_RX_QUEUES=64
   # for file in /sys/class/net/$IFACE/queues/rx-*/rps_flow_cnt; do
   # echo $(($NUM_RPS_ENTRIES/$NUM_RX_QUEUES)) > $file;
   # done

```
4. 绂佺敤 IRQ 鍧囪　瀹堟姢杩涚▼锛堣繖鍙槸鏈嶅姟鐨勪复鏃跺仠姝紝鐩村埌涓嬫閲嶅惎锛夈€?
```

   # systemctl stop irqbalance

```
5. 閰嶇疆涓柇浜插拰鎬с€?
   鍙傝 `/Documentation/core-api/irq/irq-affinity.rst`


```

  # ethtool -K <ethX> ntuple off

```
娉ㄦ剰锛氭鍛戒护灏嗙鐢?ntuple 杩囨护鍣紝骞舵竻闄よ蒋浠朵笌纭欢涓殑浠讳綍 aRFS 杩囨护鍣ㄣ€?
鐢ㄤ緥绀轰緥锛?
1. 灏嗘湇鍔″櫒搴旂敤绋嬪簭璁剧疆鍦ㄦ墍闇€ CPU 涓婏紙渚嬪 CPU 4锛夈€?
```

   # taskset -c 4 netserver

```
2. 浣跨敤 netperf 鍦ㄥ凡閰嶇疆 aRFS 鐨勬儏鍐典笅锛屽皢鏉ヨ嚜瀹㈡埛绔殑娴侀噺璺敱鍒版湇鍔″櫒涓婄殑 CPU 4銆傛湰渚嬩娇鐢?IPv4 涓婄殑 TCP銆?
```

   # netperf -H <Host IPv4 Address> -t TCP_STREAM


```
### 鍚敤铏氭嫙鍔熻兘锛圴F锛?
浣跨敤 sysfs 鍚敤铏氭嫙鍔熻兘锛圴F锛夈€?
```

  # echo 4 > /sys/class/net/<ethX>/device/sriov_numvfs

```
```

  # echo 0 > /sys/class/net/<ethX>/device/sriov_numvfs

```
ice 椹卞姩鏀寔鐨?VF 鏈€澶ф€绘暟涓?256锛堟墍鏈夌鍙ｏ級銆傝妫€鏌?
```

  # cat /sys/class/net/<ethX>/device/sriov_totalvfs

```
娉ㄦ剰锛氬綋閾捐矾鑱氬悎锛圠AG锛?bonding 澶勪簬娲诲姩鐘舵€佹椂锛屾偍涓嶈兘浣跨敤 SR-IOV锛屽弽涔嬩害鐒躲€備负寮哄埗鎵ц姝よ鍒欙紝椹卞姩浼氭鏌ヨ繖绉嶄簰鏂ュ叧绯汇€?

### 鍦?PF 涓婃樉绀?VF 缁熻淇℃伅

```

  # ip -s link show dev <ethX>

```
娉ㄦ剰锛氱敱浜庡彲鑳界殑 VF 鏁伴噺寰堝ぇ锛屾鍛戒护鐨勮緭鍑哄彲鑳介潪甯稿簽澶с€?
PF 椹卞姩灏嗘樉绀?PF 浠ュ強鎵€鏈夊凡閰嶇疆 VF 鐨勯儴鍒嗙粺璁′俊鎭€侾F 灏嗗缁堜负姣忎釜鍙兘鐨?VF 鎵撳嵃涓€涓粺璁″潡锛屽浜庢湭閰嶇疆鐨?VF 鍒欐樉绀洪浂銆?

### 鍦ㄥ惎鐢?SR-IOV 鐨勯€傞厤鍣ㄧ鍙ｄ笂閰嶇疆 VLAN 鏍囪

瑕佷负鍚敤 SR-IOV 鐨勯€傞厤鍣ㄤ笂鐨勭鍙ｉ厤缃?VLAN 鏍囪锛岃浣跨敤浠ヤ笅鍛戒护銆俈LAN 閰嶇疆搴斿湪鍔犺浇 VF 椹卞姩鎴栧惎鍔?VM 涔嬪墠瀹屾垚銆俈F 涓嶄細鎰熺煡鍦ㄥ彂閫佹椂鎻掑叆銆佹帴鏀舵椂绉婚櫎鐨?VLAN 鏍囪锛堟湁鏃剁О涓?绔彛 VLAN"妯″紡锛夈€?
```

  # ip link set dev <ethX> vf <id> vlan <vlan id>

```
```

  # ip link set dev eth0 vf 0 vlan 10


```
### 鑻ョ鍙ｆ柇寮€鍒欏惎鐢?VF 閾捐矾

濡傛灉鐗╃悊鍔熻兘锛圥F锛夐摼璺柇寮€锛屾偍鍙互浠庝富鏈?PF 寮哄埗浠讳綍缁戝畾鍒拌 PF 鐨勮櫄鎷熷姛鑳斤紙VF锛夐摼璺?up銆?
```

  # ip link set eth0 vf 0 state enable

```
娉ㄦ剰锛氬鏋滃懡浠や笉璧蜂綔鐢紝鍙兘鏄偍鐨勭郴缁熶笉鏀寔銆?

### 璁剧疆 VF 鐨?MAC 鍦板潃

```

  # ip link set <ethX> vf 0 mac <address>

```
```

  # ip link set <ethX> vf 0 mac 00:01:02:03:04:05

```
姝よ缃寔缁埌 PF 閲嶆柊鍔犺浇銆?
娉ㄦ剰锛氫粠涓绘満涓?VF 鍒嗛厤 MAC 鍦板潃灏嗙鐢ㄦ潵鑷?VM 鍐呴儴鐨勪换浣曞悗缁洿鏀?MAC 鍦板潃鐨勮姹傘€傝繖鏄竴椤瑰畨鍏ㄧ壒鎬с€俈M 涓嶄細鎰熺煡姝ら檺鍒讹紝鍥犳濡傛灉鍦?VM 涓皾璇曪紝灏嗚Е鍙?MDD 浜嬩欢銆?

### 鍙椾俊浠?VF 涓?VF 娣锋潅妯″紡

姝ょ壒鎬у厑璁告偍灏嗙壒瀹?VF 鎸囧畾涓哄彈淇′换锛屽苟鍏佽璇ュ彈淇′换鐨?VF 鍦ㄧ墿鐞嗗姛鑳斤紙PF锛変笂璇锋眰閫夋嫨鎬ф贩鏉傛ā寮忋€?
瑕佸皢 VF 璁剧疆涓哄彈淇′换鎴栦笉鍙椾俊浠伙紝璇疯緭鍏ヤ互涓嬪懡浠?
```

  # ip link set dev <ethX> vf 1 trust [on|off]

```
娉ㄦ剰锛氬湪璁剧疆娣锋潅妯″紡涔嬪墠锛屽厛灏?VF 璁剧疆涓哄彈淇′换闈炲父閲嶈銆傚鏋?VM 涓嶅彈淇′换锛孭F 灏嗗拷鐣ユ潵鑷?VF 鐨勬贩鏉傛ā寮忚姹傘€傚鏋滃湪 VF 椹卞姩鍔犺浇鍚?VM 鍙樹负鍙椾俊浠伙紝鎮ㄥ繀椤婚噸鏂板彂鍑鸿姹備互灏?VF 璁剧疆涓烘贩鏉傛ā寮忋€?
涓€鏃?VF 琚寚瀹氫负鍙椾俊浠伙紝璇蜂娇鐢?VM 涓殑浠ヤ笅鍛戒护灏?VF 璁剧疆涓烘贩鏉傛ā寮忋€?
```

  # ip link set <ethX> promisc on
  Where <ethX> is a VF interface in the VM

```
```

  # ip link set <ethX> allmulticast on
  Where <ethX> is a VF interface in the VM

```
娉ㄦ剰锛氶粯璁ゆ儏鍐典笅锛宔thtool 绉佹湁鏍囧織 vf-true-promisc-support 璁剧疆涓?"off"锛屾剰鍛崇潃 VF 鐨勬贩鏉傛ā寮忓皢鍙楅檺銆傝灏?VF 鐨勬贩鏉傛ā寮忚缃负鐪熸鐨勬贩鏉傛ā寮忓苟鍏佽 VF 鐪嬪埌鎵€鏈?
```

  # ethtool --set-priv-flags <ethX> vf-true-promisc-support on

```
vf-true-promisc-support 绉佹湁鏍囧織骞朵笉鍚敤娣锋潅妯″紡锛涚浉鍙嶏紝瀹冩寚瀹氬綋鎮ㄤ娇鐢ㄤ笂杩?ip link 鍛戒护鍚敤娣锋潅妯″紡鏃讹紝灏嗚幏寰楀摢绉嶇被鍨嬬殑娣锋潅妯″紡锛堝彈闄愭垨鐪熸锛夈€傝娉ㄦ剰杩欐槸涓€涓奖鍝嶆暣涓澶囩殑鍏ㄥ眬璁剧疆銆備絾鏄紝vf-true-promisc-support 绉佹湁鏍囧織浠呮毚闇茬粰璁惧鐨勭涓€涓?PF銆傛棤璁?vf-true-promisc-support 璁剧疆濡備綍锛孭F 濮嬬粓淇濇寔鍙楅檺娣锋潅妯″紡銆?
```

  # ip link add link eth2 name eth2.100 type vlan id 100

```
璇锋敞鎰忥紝鎮ㄥ皢 VF 璁剧疆涓烘贩鏉傛ā寮忎笌娣诲姞 VLAN 鎺ュ彛鐨勯『搴忔棤鍏筹紙鍙换鎰忓厛鍋氬叾涓€锛夈€傛湰渚嬬殑缁撴灉鏄?VF 灏嗚幏寰楁墍鏈夋爣璁颁簡 VLAN 100 鐨勬祦閲忋€?

### 閽堝 VF 鐨勬伓鎰忛┍鍔ㄦ娴嬶紙MDD锛?
涓€浜?Intel 浠ュお缃戣澶囦娇鐢ㄦ伓鎰忛┍鍔ㄦ娴嬶紙MDD锛夋潵妫€娴嬫潵鑷?VF 鐨勬伓鎰忔祦閲忥紝骞跺湪 VF 椹卞姩澶嶄綅鍙戠敓鍓嶇鐢?Tx/Rx 闃熷垪鎴栦涪寮冭繚瑙勬暟鎹寘銆傛偍鍙互浣跨敤 dmesg 鍛戒护鍦?PF 鐨勭郴缁熸棩蹇椾腑鏌ョ湅 MDD 娑堟伅銆?
- 濡傛灉 PF 椹卞姩璁板綍浜嗘潵鑷?VF 鐨?MDD 浜嬩欢锛岃纭宸插畨瑁呮纭殑 VF 椹卞姩銆?- 瑕佹仮澶嶅姛鑳斤紝鎮ㄥ彲浠ユ墜鍔ㄩ噸鏂板姞杞?VF 鎴?VM锛屾垨鍚敤鑷姩 VF 澶嶄綅銆?- 鍚敤鑷姩 VF 澶嶄綅鍚庯紝PF 椹卞姩鍦ㄦ帴鏀惰矾寰勪笂妫€娴嬪埌 MDD 浜嬩欢鏃朵細绔嬪嵆澶嶄綅 VF 骞堕噸鏂板惎鐢ㄩ槦鍒椼€?- 濡傛灉绂佺敤鑷姩 VF 澶嶄綅锛孭F 鍦ㄦ娴嬪埌 MDD 浜嬩欢鏃朵笉浼氳嚜鍔ㄥ浣?VF銆?
```

  # ethtool --set-priv-flags <ethX> mdd-auto-reset-vf on|off


```
### 閽堝 VF 鐨?MAC 涓?VLAN 闃叉楠楃壒鎬?
褰撹櫄鎷熷姛鑳斤紙VF锛夋帴鍙ｄ笂鐨勬伓鎰忛┍鍔ㄥ皾璇曞彂閫佹楠楁暟鎹寘鏃讹紝纭欢浼氬皢鍏朵涪寮冭€屼笉浼犺緭銆?
```

  # ip link set <ethX> vf <vf id> spoofchk {off|on}


```
### 宸ㄥ瀷甯?
閫氳繃灏嗘渶澶т紶杈撳崟鍏冿紙MTU锛夋洿鏀逛负澶т簬榛樿鍊?1500 鐨勫€兼潵鍚敤宸ㄥ瀷甯ф敮鎸併€?
浣跨敤 ifconfig 鍛戒护澧炲ぇ MTU 澶у皬銆備緥濡傦紝杈撳叆

```

  # ifconfig <ethX> mtu 9000 up

```
```

  # ip link set mtu 9000 dev <ethX>
  # ip link set up dev <ethX>

```
姝よ缃笉浼氬湪閲嶅惎鍚庝繚鐣欍€?

娉ㄦ剰锛氬法鍨嬪抚鐨勬渶澶?MTU 璁剧疆涓?9702銆傝繖瀵瑰簲浜?9728 瀛楄妭鐨勬渶澶у法鍨嬪抚澶у皬銆?
娉ㄦ剰锛氭湰椹卞姩灏嗗皾璇曚娇鐢ㄥ涓〉澶у皬鐨勭紦鍐插尯鏉ユ帴鏀舵瘡涓法鍨嬫暟鎹寘銆傝繖鏈夊姪浜庨伩鍏嶅湪鍒嗛厤鎺ユ敹鏁版嵁鍖呮椂鍑虹幇缂撳啿鍖鸿€楀敖闂銆?
娉ㄦ剰锛氫娇鐢ㄥ法鍨嬪抚鏃讹紝涓㈠寘鍙兘瀵瑰悶鍚愰噺浜х敓鏇村ぇ褰卞搷銆傚鏋滃湪鍚敤宸ㄥ瀷甯у悗瑙傚療鍒版€ц兘涓嬮檷锛屽惎鐢ㄦ祦鎺у彲鑳戒細缂撹В璇ラ棶棰樸€?

### 閫熺巼涓庡弻宸ラ厤缃?
鍦ㄨВ鍐抽€熺巼鍜屽弻宸ラ厤缃棶棰樻椂锛屾偍闇€瑕佸尯鍒嗗熀浜庨摐缂嗙殑閫傞厤鍣ㄥ拰鍩轰簬鍏夌氦鐨勯€傞厤鍣ㄣ€?
鍦ㄩ粯璁ゆā寮忎笅锛屼娇鐢ㄩ摐缂嗚繛鎺ョ殑 Intel(R) 浠ュお缃戠綉缁滈€傞厤鍣ㄥ皢灏濊瘯涓庡叾閾捐矾浼欎即鑷姩鍗忓晢浠ョ‘瀹氭渶浣宠缃€傚鏋滈€傞厤鍣ㄦ棤娉曢€氳繃鑷姩鍗忓晢涓庨摼璺紮浼村缓绔嬮摼璺紝鎮ㄥ彲鑳介渶瑕佹墜鍔ㄥ皢閫傞厤鍣ㄥ拰閾捐矾浼欎即閰嶇疆涓虹浉鍚岃缃紝浠ュ缓绔嬮摼璺苟浼犺緭鏁版嵁鍖呫€傝繖浠呭湪灏濊瘯涓庝笉鏀寔鑷姩鍗忓晢鐨勮€佹棫浜ゆ崲鏈鸿繛鎺ワ紝鎴栧凡琚己鍒朵负鐗瑰畾閫熺巼鎴栧弻宸ユā寮忔椂鎵嶉渶瑕併€傛偍鐨勯摼璺紮浼村繀椤讳笌鎮ㄩ€夋嫨鐨勮缃尮閰嶃€? Gbps 鍙婃洿楂橀€熺巼鏃犳硶琚己鍒躲€備娇鐢ㄨ嚜鍔ㄥ崗鍟嗛€氬憡璁剧疆鎵嬪姩涓?1 Gbps 鍙婃洿楂橀€熺巼璁剧疆璁惧銆?
閫熺巼銆佸弻宸ュ拰鑷姩鍗忓晢閫氬憡閫氳繃 ethtool 宸ュ叿閰嶇疆銆傛湁鍏虫渶鏂扮増鏈紝璇蜂粠浠ヤ笅缃戠珯涓嬭浇骞跺畨瑁?ethtool锛?
   https://kernel.org/pub/software/network/ethtool/

```

  # ethtool <ethX>

```
璀﹀憡锛氬彧鏈夌粡楠屼赴瀵岀殑缃戠粶绠＄悊鍛樻墠搴斿己鍒惰缃€熺巼鍜屽弻宸ワ紝鎴栨墜鍔ㄦ洿鏀硅嚜鍔ㄥ崗鍟嗛€氬憡銆備氦鎹㈡満涓婄殑璁剧疆蹇呴』濮嬬粓涓庨€傞厤鍣ㄨ缃尮閰嶃€傚鏋滄偍灏嗛€傞厤鍣ㄩ厤缃緱涓庝氦鎹㈡満涓嶅悓锛岄€傞厤鍣ㄦ€ц兘鍙兘浼氫笅闄嶆垨鏃犳硶宸ヤ綔銆?

### 鏁版嵁涓績妗ユ帴锛圖CB锛?
娉ㄦ剰锛氬唴鏍稿亣瀹?TC0 鍙敤锛屽鏋?TC0 涓嶅彲鐢紝灏嗗湪璁惧涓婄鐢ㄤ紭鍏堢骇娴佹帶锛圥FC锛夈€傝瑙ｅ喅姝ら棶棰橈紝璇峰湪浜ゆ崲鏈轰笂璁剧疆 DCB 鏃剁‘淇濆惎鐢?TC0銆?
DCB 鏄‖浠朵腑鐨勬湇鍔¤川閲忥紙QoS锛夐厤缃疄鐜般€傚畠浣跨敤 VLAN 浼樺厛绾ф爣绛撅紙802.1p锛夋潵杩囨护娴侀噺銆傝繖鎰忓懗鐫€娴侀噺鍙互琚繃婊ゅ埌 8 涓笉鍚岀殑浼樺厛绾с€傚畠杩樺惎鐢ㄤ簡浼樺厛绾ф祦鎺э紙802.1Qbb锛夛紝鍙互鍦ㄧ綉缁滃帇鍔涙湡闂撮檺鍒舵垨娑堥櫎涓㈠寘鏁伴噺銆傚彲浠ヤ负杩欎簺浼樺厛绾т腑鐨勬瘡涓€涓垎閰嶅甫瀹斤紝璇ュ垎閰嶅湪纭欢绾у埆寮哄埗鎵ц锛?02.1Qaz锛夈€?
DCB 閫氬父浣跨敤 DCBX 鍗忚锛?02.1Qaz锛孡LDP锛?02.1AB锛夌殑鐗瑰寲鐗堬級鍦ㄧ綉缁滀笂閰嶇疆銆俰ce 椹卞姩鏀寔浠ヤ笅浜掓枼鐨?DCBX 鏀寔鍙樹綋锛?
1) 鍩轰簬鍥轰欢鐨?LLDP 浠ｇ悊
2) 鍩轰簬杞欢鐨?LLDP 浠ｇ悊

鍦ㄥ熀浜庡浐浠舵ā寮忎笅锛屽浐浠舵嫤鎴墍鏈?LLDP 娴侀噺骞堕€忔槑鍦颁负鐢ㄦ埛澶勭悊 DCBX 鍗忓晢銆傚湪姝ゆā寮忎笅锛岄€傞厤鍣ㄤ互"willing" DCBX 妯″紡杩愯锛屼粠閾捐矾浼欎即锛堥€氬父鏄氦鎹㈡満锛夋帴鏀?DCB 璁剧疆銆傛湰鍦扮敤鎴峰彧鑳芥煡璇㈠崗鍟嗗悗鐨?DCB 閰嶇疆銆傛湁鍏冲湪浜ゆ崲鏈轰笂閰嶇疆 DCBX 鍙傛暟鐨勪俊鎭紝璇锋煡闃呬氦鎹㈡満鍒堕€犲晢鐨勬枃妗ｃ€?
鍦ㄥ熀浜庤蒋浠舵ā寮忎笅锛孡LDP 娴侀噺琚浆鍙戝埌缃戠粶鏍堝拰鐢ㄦ埛绌洪棿锛岀敱杞欢浠ｇ悊澶勭悊銆傚湪姝ゆā寮忎笅锛岄€傞厤鍣ㄥ彲浠ヤ互"willing"鎴?nonwilling" DCBX 妯″紡杩愯锛屽苟涓?DCB 閰嶇疆鏃㈠彲浠ヨ鏌ヨ涔熷彲浠ュ湪鏈湴璁剧疆銆傛妯″紡闇€瑕佺鐢ㄥ熀浜?FW 鐨?LLDP 浠ｇ悊銆?
娉ㄦ剰锛?
- 鎮ㄥ彲浠ヤ娇鐢?ethtool 绉佹湁鏍囧織鍚敤鍜岀鐢ㄥ熀浜庡浐浠剁殑 LLDP 浠ｇ悊銆傛湁鍏虫洿澶氫俊鎭紝璇峰弬闃呮湰鏂囨。涓殑"FW-LLDP锛堝浐浠堕摼璺眰鍙戠幇鍗忚锛?涓€鑺傘€?- 鍦ㄥ熀浜庤蒋浠剁殑 DCBX 妯″紡涓嬶紝鎮ㄥ彲浠ヤ娇鐢ㄤ笌 Linux 鍐呮牳鐨?DCB Netlink API 鎺ュ彛鐨勮蒋浠?LLDP/DCBX 浠ｇ悊閰嶇疆 DCB 鍙傛暟銆傛垜浠缓璁湪杞欢妯″紡涓嬭繍琛屾椂浣跨敤 OpenLLDP 浣滀负 DCBX 浠ｇ悊銆傛洿澶氫俊鎭紝璇峰弬闃?OpenLLDP 鐨勬墜鍐岄〉鍜?https://github.com/intel/openlldp銆?- 椹卞姩瀹炵幇浜?DCB netlink 鎺ュ彛灞傦紝浠ュ厑璁哥敤鎴风┖闂翠笌椹卞姩閫氫俊骞舵煡璇㈢鍙ｇ殑 DCB 閰嶇疆銆?- 涓嶆敮鎸佸甫 DCB 鐨?iSCSI銆?

### FW-LLDP锛堝浐浠堕摼璺眰鍙戠幇鍗忚锛?
浣跨敤 ethtool 鏇存敼 FW-LLDP 璁剧疆銆侳W-LLDP 璁剧疆涓烘瘡绔彛璁剧疆锛屽苟鍦ㄩ噸鍚悗淇濈暀銆?
```

  # ethtool --set-priv-flags <ethX> fw-lldp-agent on

```
```

  # ethtool --set-priv-flags <ethX> fw-lldp-agent off

```
```

  # ethtool --show-priv-flags <ethX>

```
娉ㄦ剰锛氭偍蹇呴』鍚敤 UEFI HII 鐨?"LLDP Agent" 灞炴€э紝姝よ缃墠鑳界敓鏁堛€傚鏋?"LLDP AGENT" 琚缃负绂佺敤锛屾偍鏃犳硶浠庢搷浣滅郴缁熷惎鐢ㄥ畠銆?

### 娴佹帶

浠ュお缃戞祦鎺э紙IEEE 802.3x锛夊彲閫氳繃 ethtool 閰嶇疆锛屼互鍚敤 ice 鐨勬帴鏀跺拰鍙戦€佹殏鍋滃抚銆傚惎鐢ㄥ彂閫佹椂锛屽綋鎺ユ敹鏁版嵁鍖呯紦鍐插尯瓒婅繃棰勫畾涔夐槇鍊兼椂浼氱敓鎴愭殏鍋滃抚銆傚惎鐢ㄦ帴鏀舵椂锛屽彂閫佸崟鍏冨皢鍦ㄦ敹鍒版殏鍋滃抚鏃舵寚瀹氱殑寤惰繜鏃堕棿鍐呮殏鍋溿€?
娉ㄦ剰锛氭偍蹇呴』鏈変竴涓敮鎸佹祦鎺х殑閾捐矾浼欎即銆?
娴佹帶榛樿绂佺敤銆?
浣跨敤 ethtool 鏇存敼娴佹帶璁剧疆銆?
```

  # ethtool -A <ethX> rx <on|off> tx <on|off>

```
娉ㄦ剰锛氭鍛戒护浠呭湪绂佺敤鑷姩鍗忓晢鏃舵墠鍚敤鎴栫鐢ㄦ祦鎺с€傚鏋滃惎鐢ㄤ簡鑷姩鍗忓晢锛屾鍛戒护浼氭洿鏀逛笌閾捐矾浼欎即杩涜鑷姩鍗忓晢鎵€鐢ㄧ殑鍙傛暟銆?
娉ㄦ剰锛氭祦鎺ц嚜鍔ㄥ崗鍟嗘槸閾捐矾鑷姩鍗忓晢鐨勪竴閮ㄥ垎銆傛牴鎹偍鐨勮澶囷紝鎮ㄥ彲鑳芥棤娉曟洿鏀硅嚜鍔ㄥ崗鍟嗚缃€?
娉ㄦ剰锛?
- ice 椹卞姩瑕佹眰绔彛鍜岄摼璺紮浼翠袱绔兘鍚敤娴佹帶銆傚鏋滃叾涓竴绔鐢ㄤ簡娴佹帶锛岀鍙ｅ湪閲嶆祦閲忎笅鍙兘鐪嬩技鎸傝捣銆?- 绂佺敤 DCB 鍚庯紝鎮ㄥ彲鑳戒細閬囧埌閾捐矾绾ф祦鎺э紙LFC锛夐棶棰樸€侺FC 鐘舵€佸彲鑳芥樉绀轰负宸插惎鐢紝浣嗘祦閲忓苟鏈鏆傚仠銆傝瑙ｅ喅

```

   # ethtool -A <ethX> rx off tx off
   # ethtool -A <ethX> rx on tx on


```
### NAPI


鏈┍鍔ㄦ敮鎸?NAPI锛圧x 杞妯″紡锛夈€?
鍙傝 Documentation/networking/napi.rst <napi> 鑾峰彇鏇村淇℃伅銆?
### MACVLAN

鏈┍鍔ㄦ敮鎸?MACVLAN銆傚彲浠ラ€氳繃妫€鏌ユ槸鍚﹀凡鍔犺浇 MACVLAN 椹卞姩鏉ユ祴璇曞唴鏍告槸鍚︽敮鎸?MACVLAN銆傛偍鍙互杩愯 'lsmod | grep macvlan' 鏌ョ湅鏄惁宸插姞杞?MACVLAN 椹卞姩锛屾垨杩愯 'modprobe macvlan' 灏濊瘯鍔犺浇 MACVLAN 椹卞姩銆?
娉ㄦ剰锛?
- 鍦?passthru 妯″紡涓嬶紝鎮ㄥ彧鑳借缃竴涓?MACVLAN 璁惧銆傚畠灏嗙户鎵垮簳灞?PF锛堢墿鐞嗗姛鑳斤級璁惧鐨?MAC 鍦板潃銆?

### IEEE 802.1ad锛圦inQ锛夋敮鎸?
IEEE 802.1ad 鏍囧噯锛岄€氬父绉颁负 QinQ锛屽厑璁稿湪鍗曚釜浠ュお缃戝抚涓寘鍚涓?VLAN ID銆俈LAN ID 鏈夋椂绉颁负"鏍囩"锛屽洜姝ゅ涓?VLAN ID 绉颁负"鏍囩鏍?銆傛爣绛炬爤鍏佽 L2 闅ч亾浠ュ強鍦ㄧ壒瀹?VLAN ID 鍐呴殧绂绘祦閲忕瓑鐢ㄩ€斻€?
娉ㄦ剰锛?
- 802.1ad锛圦inQ锛夋暟鎹寘涓嶆敮鎸佹帴鏀舵牎楠屽拰鍗歌浇鍜?VLAN 鍔犻€熴€?
- 闄ら潪閫氳繃浠ヤ笅鏂瑰紡绂佺敤 VLAN 鍓ョ锛屽惁鍒欎笉浼氭帴鏀?0x88A8 娴侀噺

```

    # ethtool -K <ethX> rxvlan off

```
- 鍚屼竴绔彛涓婇厤缃簡 0x8100 VLAN 鏃讹紝涓嶈兘浣跨敤 0x88A8/0x8100 鍙?VLAN 涓?0x8100 鎴?0x8100/0x8100 VLAN銆傚鏋滈厤缃簡 0x8100 VLAN锛屽皢涓嶄細鎺ユ敹 0x88a8/0x8100 娴侀噺銆?
- VF 浠呭湪涓嬭堪鏉′欢涓嬫墠鑳藉彂閫?0x88A8/0x8100锛堝嵆 802.1ad/802.1Q锛夋祦閲忥細

    1) VF 鏈鍒嗛厤绔彛 VLAN銆?    2) 浠?PF 绂佺敤浜?spoofchk銆傚鏋滃惎鐢?spoofchk锛孷F 灏嗕笉浼氬彂閫?0x88A8/0x8100 娴侀噺銆?
- 鍦?SR-IOV 妯″紡涓嬪惎鐢?VF 鐪熸娣锋潅妯″紡锛坴f-true-promisc-support锛夊拰鍙?VLAN 鏃讹紝VF 鍙兘鏃犳硶鏍规嵁鍐呴儴 VLAN 澶存帴鏀舵墍鏈夌綉缁滄祦閲忋€?
```

  # ip link add link eth0 eth0.24 type vlan proto 802.1ad id 24
  # ip link add link eth0.24 eth0.24.371 type vlan proto 802.1Q id 371

  Where "24" and "371" are example VLAN IDs.


```
### 闅ч亾/鍙犲姞鏃犵姸鎬佸嵏杞?
鏀寔鐨勯毀閬撳拰鍙犲姞鍖呮嫭 VXLAN銆丟ENEVE 浠ュ強鍙栧喅浜庣‖浠跺拰杞欢閰嶇疆鐨勫叾浠栫被鍨嬨€傛棤鐘舵€佸嵏杞介粯璁ゅ惎鐢ㄣ€?
```

  # ethtool -k <ethX>


```
### UDP 鍒嗘鍗歌浇

鍏佽閫傞厤鍣ㄥ皢鏈夋晥杞借嵎鏈€澶?64K 鐨?UDP 鏁版嵁鍖呯殑鍙戦€佸垎娈靛嵏杞藉埌鏈夋晥鐨勪互澶綉甯т腑銆傜敱浜庨€傞厤鍣ㄧ‖浠惰兘澶熸瘮鎿嶄綔绯荤粺杞欢鏇村揩鍦板畬鎴愭暟鎹垎娈碉紝姝ょ壒鎬у彲鏀瑰杽浼犺緭鎬ц兘銆?姝ゅ锛岄€傞厤鍣ㄥ彲鑳戒娇鐢ㄦ洿灏戠殑 CPU 璧勬簮銆?
娉ㄦ剰锛?
- 鍙戦€?UDP 鏁版嵁鍖呯殑搴旂敤绋嬪簭蹇呴』鏀寔 UDP 鍒嗘鍗歌浇銆?
```

  # ethtool -K <ethX> tx-udp-segmentation [off|on]

```
### PTP 寮曡剼鎺ュ彛

鎵€鏈夐€傞厤鍣ㄩ兘鏀寔鏍囧噯 PTP 寮曡剼鎺ュ彛銆係DP锛圫oftware Definable Pin锛岃蒋浠跺彲瀹氫箟寮曡剼锛夋槸鏀寔鍛ㄦ湡杈撳嚭鍜屽閮ㄦ椂闂存埑鐨勫崟绔紩鑴氥€傝繕鏈夌壒瀹氱殑宸垎杈撳叆/杈撳嚭寮曡剼锛圱IME_SYNC銆?PPS锛夛紝姣忕浠呮敮鎸佸叾涓竴绉嶅姛鑳姐€?
鏈変簺閫傞厤鍣ㄥ甫鏈?DPLL锛屽叾寮曡剼杩炴帴鍒?DPLL 鑰岄潪鏆撮湶鍦ㄦ澘鍗′笂銆傛偍闇€瑕佹敞鎰忥紝鍦ㄨ繖浜涢厤缃腑锛屼粎鏆撮湶 SDP 寮曡剼锛屼笖姣忎釜寮曡剼鏈夊叾鍥哄畾鐨勬柟鍚戙€傝鍦ㄨ繖浜?PTP 寮曡剼涓婄湅鍒拌緭鍏ヤ俊鍙凤紝鎮ㄥ繀椤绘纭厤缃?DPLL銆傝緭鍑轰俊鍙蜂粎鍦?DPLL 涓婂彲瑙侊紝瑕佸皢鍏跺彂閫佸埌鏉垮崱鐨?SMA/U.FL 寮曡剼锛屽繀椤绘墜鍔ㄩ厤缃?DPLL 杈撳嚭寮曡剼銆?
### GNSS 妯″潡

闇€瑕佸唴鏍镐互 CONFIG_GNSS=y 鎴?CONFIG_GNSS=m 缂栬瘧銆傚厑璁哥敤鎴蜂粠 GNSS 纭欢妯″潡璇诲彇娑堟伅骞跺啓鍏ュ彈鏀寔鐨勫懡浠ゃ€傚鏋滄ā鍧楃墿鐞嗗瓨鍦紝灏嗙敓鎴愪竴涓?GNSS 璁惧锛歚/dev/gnss<id>`銆傚啓鍏ュ懡浠ょ殑鍗忚鍙栧喅浜?GNSS 纭欢妯″潡锛屽洜涓洪┍鍔ㄩ€氳繃 i2c 灏嗗師濮嬪瓧鑺傜敱 GNSS 瀵硅薄鍐欏叆鎺ユ敹鍣ㄣ€傛湁鍏抽厤缃鎯咃紝璇峰弬闃呯‖浠?GNSS 妯″潡鏂囨。銆?

### 鍥轰欢锛團W锛夋棩蹇?
椹卞姩浠呴€氳繃 PF 0 涓婄殑 debugfs 鎺ュ彛鏀寔 FW 鏃ュ織銆傝繍琛屽湪 NIC 涓婄殑 FW 蹇呴』鏀寔 FW 鏃ュ織锛涘鏋?FW 涓嶆敮鎸?FW 鏃ュ織锛屽垯涓嶄細鍦?ice debugfs 鐩綍涓垱寤?'fwlog' 鏂囦欢銆?
#### 妯″潡閰嶇疆

鍥轰欢鏃ュ織鎸夋ā鍧楄繘琛岄厤缃€傛瘡涓ā鍧楀彲浠ヨ缃负鐙珛浜庡叾浠栨ā鍧楃殑鍊硷紙闄ら潪鎸囧畾妯″潡 'all'锛夈€傝繖浜涙ā鍧楀皢鍦?'fwlog/modules' 鐩綍涓嬪疄渚嬪寲銆?
鐢ㄦ埛鍙互閫氳繃鍐欏叆妯″潡鏂囦欢鏉ヨ缃ā鍧楃殑鏃ュ織绾у埆锛屽

```

  # echo <log_level> > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/<module>

```
鍏朵腑

- log_level 鏄涓嬫墍杩扮殑鍚嶇О銆傛瘡涓骇鍒寘鍚墠涓€涓?鏇翠綆绾у埆鐨勬秷鎭?
      - none
      - error
      - warning
      - normal
      - verbose

- module 鏄〃绀鸿鎺ユ敹浜嬩欢鐨勬ā鍧楃殑鍚嶇О銆傛ā鍧楀悕绉颁负

      - general
      - ctrl
      - link
      - link_topo
      - dnl
      - i2c
      - sdp
      - mdio
      - adminq
      - hdma
      - lldp
      - dcbx
      - dcb
      - xlr
      - nvm
      - auth
      - vpd
      - iosf
      - parser
      - sw
      - scheduler
      - txq
      - rsvd
      - post
      - watchdog
      - task_dispatch
      - mng
      - synce
      - health
      - tsdrv
      - pfreg
      - mdlver
      - all

鍚嶇О 'all' 鏄壒娈婄殑锛屽厑璁哥敤鎴峰皢鎵€鏈夋ā鍧楄缃负鎸囧畾鐨?log_level锛屾垨璇诲彇鎵€鏈夋ā鍧楃殑 log_level銆?
##### 閰嶇疆妯″潡鐨勭ず渚嬬敤娉?

```

  # echo verbose > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/link

```
```

  # echo verbose > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/link
  # echo warning > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/ctrl
  # echo none > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/dcb

```
```

  # echo normal > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/all

```
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/general

```
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/all

```
#### 鍚敤 FW 鏃ュ織

閰嶇疆妯″潡浼氶€氱煡 FW锛岄厤缃殑妯″潡搴旂敓鎴愰┍鍔ㄦ劅鍏磋叮鐨勪簨浠讹紝浣嗗湪鍚?FW 鍙戦€?enable 娑堟伅涔嬪墠锛屽畠**涓嶄細**灏嗚繖浜涗簨浠跺彂閫佺粰椹卞姩銆備负姝わ紝鐢ㄦ埛鍙互鍚?'fwlog/enable' 鍐欏叆 1锛堝惎鐢級鎴?0锛堢鐢級銆傜ず渚?
```

  # echo 1 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/enable

```
#### 鑾峰彇 FW 鏃ュ織鏁版嵁

鍙互閫氳繃璇诲彇 'fwlog/data' 鑾峰彇 FW 鏃ュ織鏁版嵁銆傜敤鎴峰彲浠ュ悜 'fwlog/data' 鍐欏叆浠绘剰鍊间互娓呴櫎鏁版嵁銆傛暟鎹彧鑳藉湪绂佺敤 FW 鏃ュ織鏃舵竻闄ゃ€侳W 鏃ュ織鏁版嵁鏄彂閫佺粰 Intel 骞剁敤浜庡府鍔╄皟璇曠敤鎴烽棶棰樼殑浜岃繘鍒舵枃浠躲€?
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/data > fwlog.bin

```
```

  # echo 0 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/data

```
#### 鏇存敼鏃ュ織浜嬩欢鍙戦€佸埌椹卞姩鐨勯鐜?
椹卞姩浠庣鐞嗘帴鏀堕槦鍒楋紙ARQ锛夋帴鏀?FW 鏃ュ織鏁版嵁銆侳W 鍙戦€?ARQ 浜嬩欢鐨勯鐜囧彲浠ラ€氳繃鍐欏叆 'fwlog/nr_messages' 鏉ラ厤缃€傝寖鍥存槸 1-128锛? 琛ㄧず鎺ㄩ€佹瘡鏉℃棩蹇楁秷鎭紝128 琛ㄧず浠呭湪鏈€澶?AQ 鍛戒护缂撳啿鍖烘弧鏃舵帹閫侊級銆傚缓璁€间负 10銆傜敤鎴峰彲浠ラ€氳繃璇诲彇

```

  # echo 50 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/nr_messages

```
#### 閰嶇疆鐢ㄤ簬瀛樺偍 FW 鏃ュ織鏁版嵁鐨勫唴瀛橀噺

椹卞姩鍦ㄩ┍鍔ㄥ唴閮ㄥ瓨鍌?FW 鏃ュ織鏁版嵁銆傜敤浜庡瓨鍌ㄦ暟鎹殑榛樿鍐呭瓨澶у皬涓?1MB銆傛煇浜涚敤渚嬪彲鑳介渶瑕佹洿澶氭垨鏇村皯鏁版嵁锛屽洜姝ょ敤鎴峰彲浠ユ洿鏀逛负 FW 鏃ュ織鏁版嵁鍒嗛厤鐨勫唴瀛橀噺銆傝鏇存敼鍐呭瓨閲忥紝璇峰啓鍏?'fwlog/log_size'銆傚€煎繀椤讳负浠ヤ笅涔嬩竴锛?28K銆?56K銆?12K銆?M 鎴?2M銆傚繀椤荤鐢?FW 鏃ュ織鎵嶈兘鏇存敼

```

  # echo 128K > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/log_size


```
## 鎬ц兘浼樺寲

椹卞姩榛樿鍊兼棬鍦ㄩ€傚簲鍚勭宸ヤ綔璐熻浇锛屼絾濡傛灉闇€瑕佽繘涓€姝ヤ紭鍖栵紝鎴戜滑寤鸿灏濊瘯浠ヤ笅璁剧疆銆?

### Rx 鎻忚堪绗︾幆澶у皬

瑕佸噺灏?Rx 鏁版嵁鍖呬涪寮冪殑鏁伴噺锛岃浣跨敤 ethtool 澧炲姞姣忎釜 Rx 鐜殑 Rx 鎻忚堪绗︽暟閲忋€?
  妫€鏌ユ帴鍙ｆ槸鍚﹀洜缂撳啿鍖烘弧鑰屼涪寮?Rx 鏁版嵁鍖?
```

    # ethtool -S <ethX> | grep "rx_dropped"

  濡傛灉涓婁竴鏉″懡浠ゆ樉绀洪槦鍒椾笂鏈変涪寮冿紝浣跨敤 'ethtool -G' 澧炲姞鎻忚堪绗︽暟閲忓彲鑳戒細鏈夊府鍔╋細

    # ethtool -G <ethX> rx <N>
    Where <N> is the desired number of ring entries/descriptors

  杩欏彲浠ヤ负 CPU 澶勭悊鎻忚堪绗︽椂浜х敓寤惰繜鐨勯棶棰樻彁渚涗复鏃剁紦鍐层€?

```
### 涓柇閫熺巼闄愬埗

鏈┍鍔ㄦ敮鎸侀拡瀵归€氱敤宸ヤ綔璐熻浇璋冧紭鐨勮嚜閫傚簲涓柇鑺傛祦鐜囷紙ITR锛夋満鍒躲€傜敤鎴峰彲浠ラ€氳繃 ethtool 閽堝鐗瑰畾宸ヤ綔璐熻浇鑷畾涔変腑鏂€熺巼鎺у埗锛岃皟鏁翠腑鏂箣闂寸殑寰鏁般€?
```

  # ethtool -C <ethX> adaptive-rx off adaptive-tx off

```
涓洪檷浣?CPU 鍒╃敤鐜囷細

  绂佺敤鑷€傚簲 ITR 骞堕檷浣?Rx 鍜?Tx 涓柇銆備互涓嬬ず渚嬪奖鍝嶆寚瀹氭帴鍙ｇ殑姣忎釜闃熷垪銆?
  灏?rx-usecs 鍜?tx-usecs 璁剧疆涓?80 浼氬皢涓柇闄愬埗鍦ㄥぇ绾?
```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 80 tx-usecs 80

```
涓洪檷浣庡欢杩燂細

  閫氳繃灏?rx-usecs 鍜?tx-usecs 璁剧疆涓?0 鏉ョ鐢ㄨ嚜閫傚簲 ITR 鍜?ITR

```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 0 tx-usecs 0

```
姣忛槦鍒椾腑鏂€熺巼璁剧疆锛?
  浠ヤ笅绀轰緥閽堝闃熷垪 1 鍜?3锛屼絾鎮ㄥ彲浠ヨ皟鏁村叾浠栭槦鍒椼€?
  瑕佺鐢?Rx 鑷€傚簲 ITR 骞跺皢闈欐€?Rx ITR 璁剧疆涓?10 寰鎴?
```

    # ethtool --per-queue <ethX> queue_mask 0xa --coalesce adaptive-rx off
    rx-usecs 10

  瑕佹樉绀洪槦鍒?1 鍜?3 鐨勫綋鍓嶅悎骞惰缃細

    # ethtool --per-queue <ethX> queue_mask 0xa --show-coalesce

```
浣跨敤 rx-usecs-high 闄愬埗涓柇閫熺巼锛?
  :鏈夋晥鑼冨洿锛?-236锛?=鏃犻檺鍒讹級

   0-236 寰鐨勮寖鍥存彁渚涙瘡绉?4,237 鍒?250,000 娆′腑鏂殑鏈夋晥鑼冨洿銆俽x-usecs-high 鐨勫€煎彲浠ュ湪鍚屼竴 ethtool 鍛戒护涓嫭绔嬩簬 rx-usecs 鍜?tx-usecs 璁剧疆锛屽苟涓斾篃鐙珛浜庤嚜閫傚簲涓柇璋冭妭绠楁硶銆傚簳灞傜‖浠舵敮鎸?4 寰闂撮殧鐨勭矑搴︼紝鍥犳鐩搁偦鍊煎彲鑳藉鑷寸浉鍚岀殑涓柇閫熺巼銆?
  浠ヤ笅鍛戒护灏嗙鐢ㄨ嚜閫傚簲涓柇璋冭妭锛屽苟鍏佽鍦ㄦ寚绀烘帴鏀舵垨鍙戦€佸畬鎴愪箣鍓嶆渶澶?5 寰銆傜劧鑰岋紝瀹冧笉浼氬儚鍙兘浜х敓澶氳揪姣忕 200,000 娆′腑鏂偅鏍凤紝鑰屾槸閫氳繃 rx-usecs-high 鍙傛暟灏嗘€讳腑鏂檺鍒朵负姣忕 50,000 娆°€?
```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs-high 20
    rx-usecs 5 tx-usecs 5


```
### 铏氭嫙鍖栫幆澧?
闄ゆ湰鑺傜殑鍏朵粬寤鸿澶栵紝浠ヤ笅寤鸿鍙兘鏈夊姪浜庝紭鍖?VM 涓殑鎬ц兘銆?
  鍦?VM 涓娇鐢ㄩ€傚綋鐨勬満鍒讹紙vcpupin锛夛紝灏?CPU 鍥哄畾鍒板悇涓?LCPU锛岀‘淇濅娇鐢ㄥ寘鍚湪璁惧 local_cpulist 涓殑涓€缁?CPU锛歚/sys/class/net/<ethX>/device/local_cpulist`銆?
  鍦?VM 涓厤缃敖鍙兘澶氱殑鍙敤 Rx/Tx 闃熷垪銆傦紙鏈夊叧 iavf 椹卞姩

```

    # ethtool -L <virt_interface> rx <max> tx <max>


```
## 鏀寔

鏈夊叧涓€鑸俊鎭紝璇疯闂?Intel 鏀寔缃戠珯锛?https://www.intel.com/support/

濡傛灉鍦ㄥ彈鏀寔鐨勫唴鏍镐笂浣跨敤鍙楁敮鎸佺殑閫傞厤鍣紝鍙戠幇宸插彂甯冩簮鐮佸瓨鍦ㄩ棶棰樻椂锛岃灏嗕笌璇ラ棶棰樼浉鍏崇殑鍏蜂綋淇℃伅鍙戦€佽嚦 intel-wired-lan@lists.osuosl.org銆?

## 鍟嗘爣

Intel 鏄?Intel 鍏徃鎴栧叾瀛愬叕鍙稿湪缇庡浗鍜?鎴栧叾浠栧浗瀹?鍦板尯鐨勫晢鏍囨垨娉ㄥ唽鍟嗘爣銆?
- 鍏朵粬鍚嶇О鍜屽搧鐗屽彲鑳借澹扮О涓哄叾浠栨柟鐨勮储浜с€?