
## Netdev 鐗规€т贡璞′笌鑴卞洶鎸囧崡


Author:
	Micha艂 Miros艂aw <mirq-linux@rere.qmqm.pl>



## 绗竴閮ㄥ垎锛氱壒鎬ч泦鍚?

缃戝崱鍙槸鍘熷皝涓嶅姩鍦版敹鍙戝寘鐨勬棩瀛愭棭宸蹭竴鍘讳笉杩斻€傚浠婄殑瑷倷娣诲姞浜嗗绉嶁€滅壒鎬р€濅笌鈥滅己闄封€濓紙璇绘噦浜嗭細鍗?offload 鍗歌浇锛夛紝鎶婄敓鎴愪笌鏍￠獙鏍￠獙鍜屻€佹媶鍒嗘暟鎹寘銆佸鏁版嵁鍖呭垎绫荤瓑鍚勭浠诲姟浠庢搷浣滅郴缁熻韩涓婂嵏涓嬨€傝繖浜涜兘鍔涘強鍏剁姸鎬佸湪 Linux 鍐呮牳涓€氬父琚О涓?netdev 鐗规€с€?
鐩墠涓庨┍鍔ㄧ浉鍏崇殑鐗规€ч泦鍚堟湁涓夌粍锛屽彟鏈夌綉缁滄牳蹇冨唴閮ㄤ娇鐢ㄧ殑涓€缁勶細

 1. netdev->hw_features 闆嗗悎鍖呭惈閭ｄ簺鐘舵€佸彲鑳藉簲鏌愪釜璁惧鐨勭敤鎴疯姹傝€屾敼鍙橈紙鍚敤鎴栫鐢級鐨勭壒鎬с€傝闆嗗悎搴斿湪 ndo_init 鍥炶皟涓垵濮嬪寲锛屼箣鍚庝笉鍙洿鏀广€?
 2. netdev->features 闆嗗悎鍖呭惈褰撳墠涓烘煇璁惧鍚敤鐨勭壒鎬с€傚畠鍙簲鐢辩綉缁滄牳蹇冩垨鍦?ndo_set_features 鍥炶皟鐨勫嚭閿欒矾寰勪腑淇敼銆?
 3. netdev->vlan_features 闆嗗悎鍖呭惈鍏剁姸鎬佷細琚瓙 VLAN 璁惧缁ф壙鐨勭壒鎬э紙鍙?netdev->features 闆嗗悎闄愬埗锛夈€傜洰鍓嶅畠鐢ㄤ簬鎵€鏈?VLAN 璁惧锛屾棤璁烘爣绛炬槸鍦ㄧ‖浠惰繕鏄蒋浠朵腑鍓ョ鎴栨彃鍏ャ€?
 4. netdev->wanted_features 闆嗗悎鍖呭惈鐢ㄦ埛璇锋眰鐨勭壒鎬ч泦鍚堛€傛瘡褰撴湰闆嗗悎鎴栨煇浜涜澶囩壒瀹氭潯浠跺彂鐢熷彉鍖栨椂锛屽畠閮戒細琚?ndo_fix_features 鍥炶皟杩囨护銆傝闆嗗悎鏄綉缁滄牳蹇冨唴閮ㄤ娇鐢ㄧ殑锛岄┍鍔ㄤ腑涓嶅簲寮曠敤銆?


## 绗簩閮ㄥ垎锛氭帶鍒跺凡鍚敤鐨勭壒鎬?

褰撹鏀瑰彉褰撳墠鐗规€ч泦鍚堬紙netdev->features锛夋椂锛屼細璋冪敤 ndo_fix_features 鍥炶皟涓?netdev_fix_features() 璁＄畻鍑烘柊鐨勯泦鍚堝苟瀵瑰叾杩涜杩囨护銆傝嫢缁撴灉闆嗗悎涓庡綋鍓嶉泦鍚堜笉鍚岋紝鍒欏皢鍏朵紶鍏?ndo_set_features 鍥炶皟锛屽苟鍦紙璇ュ洖璋冭繑鍥炴垚鍔熷悗锛夋浛鎹?netdev->features 涓瓨鍌ㄧ殑鍊笺€備箣鍚庡彧瑕佸綋鍓嶉泦鍚堝彲鑳藉彂鐢熷彉鍖栵紝灏变細鍙戝嚭 NETDEV_FEAT_CHANGE 閫氱煡銆?
浠ヤ笅浜嬩欢浼氳Е鍙戦噸鏂拌绠楋細
 1. 璁惧娉ㄥ唽鍚庯紝ndo_init 杩斿洖鎴愬姛
 2. 鐢ㄦ埛璇锋眰鏀瑰彉鐗规€х姸鎬? 3. 璋冪敤浜?netdev_update_features()

ndo_*_features 鍥炶皟鍦ㄦ寔鏈?rtnl_lock 鐨勬儏鍐典笅琚皟鐢ㄣ€傜己澶辩殑鍥炶皟琚涓烘€绘槸杩斿洖鎴愬姛銆?
鎯宠瑙﹀彂閲嶆柊璁＄畻鐨勯┍鍔ㄥ繀椤婚€氳繃鎸佹湁 rtnl_lock 鏃惰皟鐢?netdev_update_features() 鏉ュ疄鐜般€備笉搴斾粠 ndo_*_features 鍥炶皟涓墽琛屾鎿嶄綔銆傞櫎閫氳繃 ndo_fix_features 鍥炶皟澶栵紝椹卞姩涓嶅簲淇敼 netdev->features銆?


## 绗笁閮ㄥ垎锛氬疄鐜版彁绀?

 - ndo_fix_features:

鐗规€т箣闂寸殑鎵€鏈変緷璧栧叧绯婚兘搴斿湪姝ゅ瑙ｅ喅銆傜粨鏋滈泦鍚堣繕鍙兘琚綉缁滄牳蹇冩柦鍔犵殑闄愬埗杩涗竴姝ョ缉鍑忥紙濡?netdev_fix_features() 涓墍缂栧啓锛夈€傚洜姝わ紝褰撴煇鐗规€х殑渚濊禆鏈弧瓒虫椂锛岀鐢ㄨ鐗规€ф瘮寮哄埗寮€鍚叾渚濊禆鏇村畨鍏ㄣ€?
璇ュ洖璋冧笉搴斾慨鏀圭‖浠舵垨椹卞姩鐘舵€侊紙搴旀槸鏃犵姸鎬佺殑锛夈€傚湪杩炵画鐨?ndo_set_features 璋冪敤涔嬮棿锛屽畠鍙兘琚娆¤皟鐢ㄣ€?
鍥炶皟涓嶅緱鏇存敼 NETIF_F_SOFT_FEATURES 鎴?NETIF_F_NEVER_CHANGE 闆嗗悎涓寘鍚殑鐗规€с€傚敮涓€鐨勪緥澶栨槸 NETIF_F_VLAN_CHALLENGED锛屼絾闇€璋ㄦ厧锛屽洜涓鸿繖绉嶆洿鏀逛笉浼氬奖鍝嶅凡閰嶇疆鐨?VLAN銆?
 - ndo_set_features:

搴旈噸鏂伴厤缃‖浠朵互鍖归厤浼犲叆鐨勭壒鎬ч泦鍚堛€傞櫎闈炲嚭鐜版棤娉曞湪 ndo_fix_features 涓彲闈犳娴嬬殑閿欒鎯呭喌锛屽惁鍒欎笉搴旀洿鏀硅闆嗗悎銆傚湪杩欑鎯呭喌涓嬶紝鍥炶皟搴斿皢 netdev->features 鏇存柊涓轰笌鏈€缁堢‖浠剁姸鎬佷竴鑷淬€傝繑鍥炵殑閿欒涓嶄細锛堜篃鏃犳硶锛夎浼犳挱鍒?dmesg 浠ュ鐨勪换浣曞湴鏂广€傦紙娉細鎴愬姛杩斿洖涓洪浂锛?0 琛ㄧず闈欓粯閿欒銆傦級



## 绗洓閮ㄥ垎锛氱壒鎬?

鏈夊叧鐗规€х殑褰撳墠鍒楄〃锛岃鍙傞槄 include/linux/netdev_features.h銆傛湰鑺傛弿杩板叾涓儴鍒嗙壒鎬х殑璇箟銆?
 - Transmit checksumming锛堝彂閫佹牎楠屽拰鍗歌浇锛?
瀹屾暣璇存槑璇峰弬闃?include/linux/skbuff.h 椤堕儴鐨勬敞閲娿€?
娉ㄦ剰锛歂ETIF_F_HW_CSUM 鏄?NETIF_F_IP_CSUM + NETIF_F_IPV6_CSUM 鐨勮秴闆嗐€傝繖鎰忓懗鐫€璁惧鍙互鍦ㄦ暟鎹寘鐨勪换浣曚綅缃紙鏃犺瀛樺湪浣曠澶撮儴锛夊～鍐欑被浼?TCP/UDP 鐨勬牎楠屽拰銆?
 - Transmit TCP segmentation offload锛堝彂閫?TCP 鍒嗘鍗歌浇锛?
NETIF_F_TSO_ECN 琛ㄧず纭欢鑳藉姝ｇ‘鍦版媶鍒嗚缃簡 CWR 浣嶇殑鏁版嵁鍖咃紝鏃犺鏄?TCPv4锛堝惎鐢?NETIF_F_TSO 鏃讹級杩樻槸 TCPv6锛圢ETIF_F_TSO6锛夈€?
 - Transmit UDP segmentation offload锛堝彂閫?UDP 鍒嗘鍗歌浇锛?
NETIF_F_GSO_UDP_L4 鎺ュ彈涓€涓甫鏈夎秴杩?gso_size 鐨勮礋杞界殑鍗曚釜 UDP 澶撮儴銆傚湪鍒嗘鏃讹紝瀹冧細鎸?gso_size 杈圭晫瀵硅礋杞借繘琛屽垎娈碉紝骞跺鍒剁綉缁滀笌 UDP 澶撮儴锛堣嫢鏈€鍚庝竴娈靛皬浜?gso_size 鍒欒繘琛屼慨姝ｏ級銆?
 - Transmit DMA from high memory锛堜粠楂樼鍐呭瓨鍙戦€?DMA锛?
鍦ㄧ浉鍏崇殑骞冲彴涓婏紝NETIF_F_HIGHDMA 琛ㄧず ndo_start_xmit 鑳藉澶勭悊鍒嗙墖锛坒rags锛変綅浜庨珮绔唴瀛樼殑 skb銆?
 - Transmit scatter-gather锛堝彂閫佸垎鏁?鑱氶泦锛?
杩欎簺鐗规€ц〃绀?ndo_start_xmit 鑳藉澶勭悊鍒嗘鐨?skb锛歂ETIF_F_SG 鈥斺€?鍒嗛〉 skb锛坰kb_shinfo()->frags锛夛紝NETIF_F_FRAGLIST 鈥斺€?閾捐〃寮?skb锛坰kb->next/prev 閾捐〃锛夈€?
 - Software features锛堣蒋浠剁壒鎬э級

NETIF_F_SOFT_FEATURES 涓寘鍚殑鐗规€у睘浜庣綉缁滄爤鐨勭壒鎬с€傞┍鍔ㄤ笉搴斿熀浜庤繖浜涚壒鎬ф敼鍙樿涓恒€?
 - VLAN challenged锛堝彈 VLAN 闄愬埗锛?
NETIF_F_VLAN_CHALLENGED 搴旇缃簬閭ｄ簺鏃犳硶澶勭悊 VLAN 澶撮儴鐨勮澶囥€傛煇浜涢┍鍔ㄨ缃畠鏄洜涓虹綉鍗℃棤娉曞鐞嗘洿澶х殑 MTU銆俒FIXME锛氳繖浜涙儏鍐靛彲鍦?VLAN 浠ｇ爜涓€氳繃鍙厑璁稿噺灏?MTU 鐨?VLAN 鏉ヤ慨澶嶃€備笉杩囪繖鍙兘鐢ㄥ涓嶅ぇ銆俔

- rx-fcs

璇ョ壒鎬ц姹?NIC 灏嗕互澶綉甯ф牎楠屽拰锛團CS锛夐檮鍔犲埌 skb 鏁版嵁鐨勬湯灏俱€傝繖鏍峰梾鎺㈠櫒鍙婂叾浠栧伐鍏峰氨鑳借鍙?NIC 鍦ㄦ敹鍒版暟鎹寘鏃惰褰曠殑 CRC銆?
- rx-all

璇ョ壒鎬ц姹?NIC 鎺ユ敹鎵€鏈夊彲鑳界殑甯э紝鍖呮嫭鍑洪敊鐨勫抚锛堝閿欒鐨?FCS 绛夛級銆傚湪鍡呮帰瀛樺湪鍧忓寘鐨勯摼璺椂浼氬緢鏈夊府鍔┿€傛煇浜?NIC 鍦ㄥ悓鏃惰繘鍏ユ櫘閫?PROMISC锛堟贩鏉傦級妯″紡鏃跺彲鑳戒細鏀跺埌鏇村鏁版嵁鍖呫€?
- rx-gro-hw

璇ョ壒鎬ц姹?NIC 鍚敤纭欢 GRO锛堥€氱敤鎺ユ敹鍗歌浇锛夈€傜‖浠?GRO 鍩烘湰涓婃槸 TSO 鐨勯€嗗悜鎿嶄綔锛屼笖閫氬父姣旂‖浠?LRO 鏇翠弗鏍笺€傜敱纭欢 GRO 鍚堝苟鐨勬暟鎹寘娴佸繀椤昏兘琚?GSO 鎴?TSO 閲嶆柊鍒嗘鍥炲畬鍏ㄥ師濮嬬殑鍖呮祦銆傜‖浠?GRO 渚濊禆 RXCSUM锛屽洜涓虹‖浠舵垚鍔熷悎骞剁殑姣忎釜鏁版嵁鍖呬篃蹇呴』鐢辩‖浠跺畬鎴愭牎楠屽拰楠岃瘉銆?
- hsr-tag-ins-offload

搴斿湪閭ｄ簺鑳借嚜鍔ㄦ彃鍏?HSR锛堥珮鍙敤鏃犵紳鍐椾綑锛夋垨 PRP锛堝苟琛屽啑浣欏崗璁級鏍囩鐨勮澶囦笂璁剧疆姝ょ壒鎬с€?
- hsr-tag-rm-offload

搴斿湪閭ｄ簺鑳借嚜鍔ㄧЩ闄?HSR锛堥珮鍙敤鏃犵紳鍐椾綑锛夋垨 PRP锛堝苟琛屽啑浣欏崗璁級鏍囩鐨勮澶囦笂璁剧疆姝ょ壒鎬с€?
- hsr-fwd-offload

搴斿湪閭ｄ簺鑳藉湪纭欢涓皢 HSR锛堥珮鍙敤鏃犵紳鍐椾綑锛夊抚浠庝竴涓鍙ｈ浆鍙戝埌鍙︿竴涓鍙ｇ殑璁惧涓婅缃鐗规€с€?
- hsr-dup-offload

搴斿湪閭ｄ簺鑳藉湪纭欢涓嚜鍔ㄥ鍒跺鍙戠殑 HSR锛堥珮鍙敤鏃犵紳鍐椾綑锛夋垨 PRP锛堝苟琛屽啑浣欏崗璁級鏍囩甯х殑璁惧涓婅缃鐗规€с€?
- netmem-tx

搴斿湪鏀寔 netmem TX 鐨勮澶囦笂璁剧疆姝ょ壒鎬с€傝鍙傞槄 Documentation/networking/netmem.rst
