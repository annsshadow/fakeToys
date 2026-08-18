
## Netfilter 鐨?flowtable 鍩虹璁炬柦


鏈枃妗ｆ弿杩颁簡 Netfilter flowtable 鍩虹璁炬柦锛屽畠鍏佽浣犻€氳繃 flowtable 鏁版嵁璺緞瀹氫箟涓€鏉?蹇€熻矾寰勶紙fastpath锛夈€傝鍩虹璁炬柦涔熸彁渚涚‖浠跺嵏杞斤紙offload锛夋敮鎸併€俧lowtable 鏀寔
绗?3 灞傜殑 IPv4 鍜?IPv6 浠ュ強绗?4 灞傜殑 TCP 鍜?UDP 鍗忚銆?
### 姒傝堪


涓€鏃︽暟鎹祦鐨勯涓暟鎹寘鎴愬姛閫氳繃 IP 杞彂璺緞锛屼粠绗簩涓暟鎹寘璧凤紝浣犲氨鍙互閫氳繃浣犵殑
瑙勫垯闆嗗皢杩欐潯娴佸嵏杞藉埌 flowtable銆俧lowtable 鍩虹璁炬柦鎻愪緵浜嗕竴绉嶈鍒欏姩浣滐紝鍏佽浣犳寚瀹?浣曟椂鍚?flowtable 娣诲姞涓€鏉℃祦銆?
鍦?flowtable 涓壘鍒板尮閰嶆潯鐩紙鍗?flowtable 鍛戒腑锛夌殑鏁版嵁鍖咃紝浼氶€氳繃 neigh_xmit() 琚?浼犻€佸埌杈撳嚭缃戠粶璁惧锛屽洜姝わ紝鏁版嵁鍖呯粫杩囦簡缁忓吀鐨?IP 杞彂璺緞锛堝彲瑙佺殑鏁堟灉鏄紝浣犱笉浼氬湪
鍏ュ彛锛坕ngress锛変箣鍚庝换浣?Netfilter 閽╁瓙涓湅鍒拌繖浜涙暟鎹寘锛夈€傚鏋?flowtable 涓病鏈?鍖归厤鐨勬潯鐩紙鍗?flowtable 鏈懡涓級锛屾暟鎹寘鍒欐部鐫€缁忓吀鐨?IP 杞彂璺緞琛岃繘銆?
flowtable 浣跨敤涓€涓彲璋冩暣澶у皬鐨勫搱甯岃〃銆傛煡鎵惧熀浜庝互涓?n 鍏冪粍閫夋嫨鍣細绗?2 灞傚崗璁皝瑁?锛圴LAN 鍜?PPPoE锛夈€佺 3 灞傛簮鍜岀洰鐨勫湴鍧€銆佺 4 灞傛簮鍜岀洰鐨勭鍙ｄ互鍙婅緭鍏ユ帴鍙ｏ紙鍦ㄦ湁澶氫釜
conntrack 鍖哄煙锛坺one锛夋椂灏卞緢鏈夌敤锛夈€?
'flow add' 鍔ㄤ綔鍏佽浣犲～鍏?flowtable锛岀敱鐢ㄦ埛閫夋嫨鎬у湴鎸囧畾鍝簺娴佽鏀惧叆 flowtable銆?鍥犳锛岄櫎闈炵敤鎴烽€氳繃绛栫暐鏄惧紡鎸囩ず鏌愪簺娴佷娇鐢ㄨ繖鏉℃柊鐨勬浛浠ｈ浆鍙戣矾寰勶紝鍚﹀垯鏁版嵁鍖呬粛娌跨粡鍏?鐨?IP 杞彂璺緞琛岃繘銆?
flowtable 鏁版嵁璺緞濡傚浘 1 鎵€绀猴紝鍏朵腑鎻忚堪浜嗗寘鍚?Netfilter 閽╁瓙鍜?flowtable 蹇€熻矾寰?缁曡鐨勭粡鍏?IP 杞彂璺緞銆?
```

					 userspace process
					  ^              |
					  |              |
				     _____|____     ____\/___
				    /          \   /         \
				    |   input   |  |  output  |
				    \__________/   \_________/
					 ^               |
					 |               |
      _________      __________      ---------     _____\/_____
     /         \    /          \     |Routing |   /            \
  -->  ingress  ---> prerouting ---> |decision|   | postrouting |--> neigh_xmit
     \_________/    \__________/     ----------   \____________/          ^
       |      ^                          |               ^                |
   flowtable  |                     ____\/___            |                |
       |      |                    /         \           |                |
    __\/___   |                    | forward |------------                |
    |-----|   |                    \_________/                            |
    |-----|   |                 'flow offload' rule                       |
    |-----|   |                   adds entry to                           |
    |_____|   |                     flowtable                             |
       |      |                                                           |
      / \     |                                                           |
     /hit\_no_|                                                           |
     \ ? /                                                                |
      \ /                                                                 |
       |__yes_________________fastpath bypass ____________________________|

	       Fig.1 Netfilter hooks and flowtable interactions

```
flowtable 鏉＄洰杩樺瓨鍌ㄤ簡 NAT 閰嶇疆锛屽洜姝ゆ墍鏈夋暟鎹寘閮芥寜鐓т粠缁忓吀 IP 杞彂璺緞鎸囧畾鐨?NAT
绛栫暐琚慨鏀广€傚湪璋冪敤 neigh_xmit() 涔嬪墠锛孴TL 浼氳鍑忎竴銆傜敱浜庣己灏戜紶杈撳眰澶达紝鍒嗙墖娴侀噺琚?鍚戜笂浼犻€掍互娌跨粡鍏?IP 杞彂璺緞琛岃繘锛屽湪杩欑鎯呭喌涓嬫棤娉曡繘琛?flowtable 鏌ユ壘銆俆CP RST 鍜?FIN 鏁版嵁鍖呬篃琚悜涓婁紶閫掑埌缁忓吀 IP 杞彂璺緞锛屼互渚夸紭闆呭湴閲婃斁娴併€傝秴杩?MTU 鐨勬暟鎹寘涔熻
鍚戜笂浼犻€掑埌缁忓吀杞彂璺緞锛屼互鍚戝彂閫佹柟鎶ュ憡鏁版嵁鍖呰繃澶х殑 ICMP 閿欒銆?
### 閰嶇疆绀轰緥


鍚敤 flowtable 缁曡鐩稿瀹规槗锛屼綘鍙渶鍒涘缓涓€涓?```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
		}
		chain y {
			type filter hook forward priority 0; policy accept;
			ip protocol tcp flow add @f
			counter packets 0 bytes 0
		}
	}

```
姝ょず渚嬪皢 flowtable 'f' 娣诲姞鍒?eth0 鍜?eth1 缃戠粶璁惧鐨?ingress 閽╁瓙涓娿€傚鏋滀綘闇€瑕?杩涜璧勬簮鍒嗗尯锛屽彲浠ュ垱寤轰换鎰忔暟閲忕殑 flowtable銆俧lowtable 浼樺厛绾у畾涔変簡绠￠亾涓挬瀛愯繍琛岀殑
椤哄簭锛岃繖鍦ㄤ綘宸茬粡鏈変竴涓?nftables ingress 閾炬椂浼氬緢鏂逛究锛堢‘淇?flowtable 鐨勪紭鍏堢骇灏忎簬
nftables ingress 閾撅紝杩欐牱 flowtable 浼氬湪绠￠亾涓厛杩愯锛夈€?
鏉ヨ嚜 forward 閾?'y' 鐨?'flow offload' 鍔ㄤ綔锛屼负鏉ヨ嚜鍥炲鏂瑰悜鐨?TCP syn-ack 鏁版嵁鍖呭悜
flowtable 娣诲姞涓€鏉℃潯鐩€備竴鏃︽祦琚嵏杞斤紝浣犱細瑙傚療鍒颁笂闈㈢ず渚嬩腑鐨勮鏁拌鍒欎笉浼氫负閫氳繃杞彂
缁曡杞彂鐨勯偅浜涙暟鎹寘鑰屾洿鏂般€?
鍦ㄥ垪鍑鸿繛鎺ヨ窡韪〃鏃讹紝浣犲彲浠ラ€氳繃 [OFFLOAD] 鏍囩鏉ヨ瘑鍒鍗歌浇鐨勬祦銆?
```

	# conntrack -L
	tcp      6 src=10.141.10.2 dst=192.168.10.2 sport=52728 dport=5201 src=192.168.10.2 dst=192.168.10.1 sport=5201 dport=52728 [OFFLOAD] mark=0 use=2


```
### 绗?2 灞傚皝瑁?

鑷?Linux 鍐呮牳 5.13 璧凤紝flowtable 鍩虹璁炬柦浼氬彂鐜?VLAN 鍜?PPPoE 缃戠粶璁惧鑳屽悗鐨勭湡瀹?缃戠粶璁惧銆俧lowtable 杞欢鏁版嵁璺緞浼氳В鏋?VLAN 鍜?PPPoE 绗?2 灞傚ご閮紝浠ユ彁鍙栫敤浜?flowtable
鏌ユ壘鐨?ethertype 鍜?VLAN ID / PPPoE 浼氳瘽 ID銆俧lowtable 鏁版嵁璺緞涔熷鐞嗙 2 灞傝В灏佽銆?
浣犳棤闇€灏?PPPoE 鍜?VLAN 璁惧娣诲姞鍒颁綘鐨?flowtable锛岀湡瀹炶澶囧氨瓒充互璁?flowtable 璺熻釜
浣犵殑娴併€?
### 妗ユ帴涓?IP 杞彂


鑷?Linux 鍐呮牳 5.13 璧凤紝浣犲彲浠ュ皢缃戞ˉ绔彛娣诲姞鍒?flowtable銆俧lowtable 鍩虹璁炬柦浼氬彂鐜?缃戞ˉ璁惧鑳屽悗鐨勬嫇鎵戙€傝繖鍏佽 flowtable 鍦ㄤ綘鐨勭綉妗ョ鍙ｏ紙鍦ㄤ笅鍥剧ず渚嬩腑琛ㄧず涓?eth1 鍜?eth2锛変笌缃戝叧璁惧锛堣〃绀轰负 eth0锛変箣闂村畾涔変竴鏉″揩閫熻矾寰勭粫琛岋紝浣嶄簬浣犵殑浜ゆ崲鏈?璺敱鍣ㄤ腑銆?
```

                      fastpath bypass
               .-------------------------.
              /                           \
              |           IP forwarding   |
              |          /             \ \/
              |       br0               eth0 ..... eth0
              .       / \                          *host B*
               -> eth1  eth2
                   .           *switch/router*
                   .
                   .
                 eth0
               *host A*

```
flowtable 鍩虹璁炬柦涔熸敮鎸佺綉妗?VLAN 杩囨护鍔ㄤ綔锛屼緥濡?PVID 鍜?untagged銆備綘涔熷彲浠ュ湪浣犵殑
缃戞ˉ绔彛涔嬩笂鍫嗗彔涓€涓粡鍏哥殑 VLAN 璁惧銆?
濡傛灉浣犲笇鏈涗綘鐨?flowtable 鍦ㄧ綉妗ョ鍙ｄ笌 IP 杞彂璺緞涔嬮棿瀹氫箟涓€鏉″揩閫熻矾寰勶紝鍒欏繀椤诲皢浣犵殑
缃戞ˉ绔彛锛堢敱鐪熷疄缃戠粶璁惧琛ㄧず锛夋坊鍔犲埌浣犵殑 flowtable 瀹氫箟涓€?
### 璁℃暟鍣?

flowtable 鍙互閫氳繃鍦ㄤ綘鐨?flowtable 瀹氫箟涓寚瀹?counter 璇彞锛屽皢鏁版嵁鍖呭拰瀛楄妭璁℃暟鍣ㄤ笌
鐜版湁鐨勮繛鎺ヨ窡韪潯鐩悓姝ワ紝渚嬪锛?
```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
			counter
		}
	}

```
璁℃暟鍣ㄦ敮鎸佽嚜 Linux 鍐呮牳 5.7 璧峰彲鐢ㄣ€?
### 纭欢鍗歌浇


濡傛灉浣犵殑缃戠粶璁惧鎻愪緵纭欢鍗歌浇鏀寔锛屼綘鍙互閫氳繃鍦ㄤ綘鐨?flowtable 瀹氫箟涓娇鐢?'offload'
鏍囧織鏉ュ紑鍚畠锛屼緥濡傦細

```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
			flags offload;
		}
	}

```
鏈変竴涓伐浣滈槦鍒楋紙workqueue锛夊皢娴佹坊鍔犲埌纭欢銆傛敞鎰忥紝鍦ㄥ伐浣滈槦鍒楁湁鏈轰細灏嗘祦鍗歌浇鍒扮綉缁滆澶?涔嬪墠锛屽皯鏁版暟鎹寘鍙兘浠嶄細杩愯鍦?flowtable 杞欢璺緞涓娿€?
鍦ㄥ垪鍑鸿繛鎺ヨ窡韪〃鏃讹紝浣犲彲浠ラ€氳繃 [HW_OFFLOAD] 鏍囩鏉ヨ瘑鍒‖浠跺嵏杞界殑娴併€傝娉ㄦ剰锛孾OFFLOAD]
鏍囩鎸囩殑鏄蒋浠跺嵏杞芥ā寮忥紝鍥犳 [OFFLOAD]锛堟寚杞欢 flowtable 蹇€熻矾寰勶級涓?[HW_OFFLOAD]
锛堟寚璇ユ祦鎵€浣跨敤鐨勭‖浠跺嵏杞芥暟鎹矾寰勶級涔嬮棿鏄湁鍖哄埆鐨勩€?
flowtable 纭欢鍗歌浇鍩虹璁炬柦涔熸敮鎸?DSA锛圖istributed Switch Architecture锛夈€?
### 灞€闄愭€?

flowtable 鐨勮涓虹被浼间簬缂撳瓨銆傚鏋滅敤浜庝紶杈撶殑鐩殑 MAC 鍦板潃鎴栧嚭鍙ｇ綉缁滆澶囧彂鐢熷彉鍖栵紝
flowtable 鏉＄洰鍙兘浼氬彉寰楅檲鏃э紙stale锛夈€?
鍦ㄤ互涓嬫儏鍐典笅杩欏彲鑳芥槸涓棶棰橈細

- 浣犲湪杞欢妯″紡涓嬭繍琛?flowtable锛屽苟涓斿湪浣犵殑閰嶇疆涓悓鏃剁粍鍚堜簡妗ユ帴鍜?IP 杞彂銆?- 鍚敤浜嗙‖浠跺嵏杞姐€?
### 寤朵几闃呰


鏈枃妗ｅ熀浜?LWN.net 鐨勬枃绔?[^1^]_\ [^2^]_銆俁afal Milecki 涔熸挵鍐欎簡涓€绡囬潪甯稿畬鏁磋€屽叏闈㈢殑
鎬荤粨锛屽悕涓?"A state of network acceleration"锛屾弿杩颁簡鍦ㄦ鍩虹璁炬柦琚悎骞跺叆涓荤嚎涔嬪墠
鐨勬儏褰?[^3^]_锛屽苟瀵规椤瑰伐浣滃仛浜嗕竴涓矖鐣ョ殑鎬荤粨 [^4^]_銆?