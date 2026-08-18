
## 浠ュお缃戞ˉ鎺?
## 绠€浠?
IEEE 802.1Q-2022锛圔ridges and Bridged Networks锛屾ˉ鎺ヤ笌妗ユ帴缃戠粶锛夋爣鍑嗗畾涔変簡妗ユ帴鍦ㄨ绠楁満缃戠粶涓殑杩愪綔鏂瑰紡銆傚湪璇ユ爣鍑嗙殑璇涓嬶紝妗ワ紙bridge锛夋槸涓€绉嶈繛鎺ヤ袱涓垨澶氫釜缃戞銆佸苟杩愯鍦?OSI锛圤pen Systems Interconnection锛屽紑鏀剧郴缁熶簰杩烇級妯″瀷鐨勬暟鎹摼璺眰锛圠ayer 2锛岀浜屽眰锛夌殑璁惧銆傛ˉ鐨勪綔鐢ㄦ槸渚濇嵁鐩殑 MAC锛圡edia Access Control锛屼粙璐ㄨ闂帶鍒讹級鍦板潃鍦ㄤ笉鍚岀綉娈典箣闂磋繃婊ゅ苟杞彂甯с€?
## 妗ユ帴 kAPI

涓嬮潰鏄ˉ鎺ヤ唬鐮佺殑涓€浜涙牳蹇冪粨鏋勪綋銆傝娉ㄦ剰锛宬API 鏄?*涓嶇ǔ瀹?*鐨勶紝闅忔椂鍙兘琚慨鏀广€?
   :identifiers: net_bridge_vlan

## 妗ユ帴 uAPI

鐜颁唬 Linux 妗ユ帴 uAPI 閫氳繃 Netlink 鎺ュ彛璁块棶銆備綘鍙互鍦ㄤ笅闈㈢殑鏂囦欢涓壘鍒版ˉ鎺ヤ互鍙婃ˉ鎺ョ鍙ｇ殑 netlink 灞炴€у畾涔夈€?
### 妗ユ帴 netlink 灞炴€?
   :doc: Bridge enum definition

### 妗ユ帴绔彛 netlink 灞炴€?
   :doc: Bridge port enum definition

### 妗ユ帴 sysfs

sysfs 鎺ュ彛宸茶寮冪敤锛岃嫢鏂板閫夐」鍒欎笉搴斿啀鎵╁睍瀹冦€?
## STP锛堢敓鎴愭爲鍗忚锛?
Linux 妗ユ帴椹卞姩涓殑 STP锛圫panning Tree Protocol锛岀敓鎴愭爲鍗忚锛夊疄鐜版槸涓€涓叧閿壒鎬э紝瀹冮€氳繃璇嗗埆骞剁鐢ㄥ啑浣欓摼璺紝甯姪闃叉浠ュお缃戠綉缁滀腑鐨勭幆璺笌骞挎挱椋庢毚銆傚湪 Linux 妗ユ帴鐨勮澧冧笅锛孲TP 瀵圭綉缁滅殑绋冲畾鎬т笌鍙敤鎬ц嚦鍏抽噸瑕併€?
STP 鏄竴涓繍琛屽湪 OSI 妯″瀷鏁版嵁閾捐矾灞傜殑 Layer 2 鍗忚銆傚畠鏈€鍒濅綔涓?IEEE 802.1D 寮€鍙戯紝姝ゅ悗婕旇繘鍑轰簡澶氫釜鐗堟湰锛屽寘鎷?Rapid Spanning Tree Protocol锛圧STP锛屽揩閫熺敓鎴愭爲鍗忚锛変互鍙?`Multiple Spanning Tree Protocol (MSTP)
<https://lore.kernel.org/netdev/20220316150857.2442916-1-tobias@waldekranz.com/>`_銆?
802.1D-2004 绉婚櫎浜嗘渶鍒濈殑 Spanning Tree Protocol锛岃浆鑰岀撼鍏ヤ簡 Rapid Spanning Tree Protocol锛圧STP锛夈€傚埌 2014 骞达紝IEEE 802.1D 瀹氫箟鐨勫叏閮ㄥ姛鑳介兘宸茶鍚堝苟鍒?IEEE 802.1Q锛圔ridges and Bridged Networks锛屾ˉ鎺ヤ笌妗ユ帴缃戠粶锛夋垨 IEEE 802.1AC锛圡AC Service Definition锛孧AC 鏈嶅姟瀹氫箟锛変箣涓€?02.1D 宸蹭簬 2022 骞存寮忔挙閿€銆?
### 妗ユ帴绔彛涓?STP 鐘舵€?
鍦?STP 鐨勮澧冧笅锛屾ˉ鎺ョ鍙ｅ彲澶勪簬浠ヤ笅鐘舵€佷箣涓€锛?  - Blocking锛堥樆濉烇級锛氱鍙ｈ绂佹鏁版嵁娴侀噺锛屽彧渚﹀惉鏉ヨ嚜鍏朵粬璁惧鐨?BPDU锛圔ridge Protocol Data Units锛屾ˉ鍗忚鏁版嵁鍗曞厓锛夛紝浠ョ‘瀹氱綉缁滄嫇鎵戙€?  - Listening锛堜睛鍚級锛氱鍙ｅ紑濮嬪弬涓?STP 杩囩▼骞朵睛鍚?BPDU銆?  - Learning锛堝涔狅級锛氱鍙ｇ户缁睛鍚?BPDU锛屽苟寮€濮嬩粠 incoming 甯т腑瀛︿範 MAC 鍦板潃锛屼絾涓嶈浆鍙戞暟鎹抚銆?  - Forwarding锛堣浆鍙戯級锛氱鍙ｅ畬鍏ㄥ彲鐢紝鍚屾椂杞彂 BPDU 涓庢暟鎹抚銆?  - Disabled锛堢鐢級锛氱鍙ｈ绠＄悊鎬х鐢紝涓嶅弬涓?STP 杩囩▼锛屾暟鎹抚杞彂涔熻绂佺敤銆?
### 鏍规ˉ涓庢敹鏁?
鍦?Linux 缃戠粶涓庝互澶綉妗ユ帴鐨勮澧冧笅锛屾牴妗ワ紙root bridge锛夋槸妗ユ帴缃戠粶涓竴涓鎸囧畾鐨勪氦鎹㈡満锛屽畠浣滀负鐢熸垚鏍戠畻娉曠殑鍙傝€冪偣锛岀敤浜庡垱寤烘棤鐜嫇鎵戙€?
浠ヤ笅鏄?STP 鐨勫伐浣滃師鐞嗕互鍙婃牴妗ョ殑閫変妇鏂瑰紡锛?  1. Bridge Priority锛堟ˉ浼樺厛绾э級锛氭瘡涓繍琛岀敓鎴愭爲鍗忚鐨勬ˉ閮芥湁涓€涓彲閰嶇疆鐨?Bridge Priority 鍊笺€傚€艰秺灏忥紝浼樺厛绾ц秺楂樸€傞粯璁ゆ儏鍐典笅锛孊ridge Priority 琚缃负涓€涓爣鍑嗗€硷紙渚嬪 32768锛夈€?  2. Bridge ID锛堟ˉ ID锛夛細Bridge ID 鐢变袱閮ㄥ垎缁勬垚锛欱ridge Priority 涓庢ˉ鐨?MAC 鍦板潃銆傚畠鍦ㄧ綉缁滀腑鍞竴鏍囪瘑姣忎釜妗ャ€侭ridge ID 鐢ㄤ簬姣旇緝涓嶅悓妗ョ殑浼樺厛绾с€?  3. Bridge Election锛堟ˉ閫変妇锛夛細缃戠粶鍚姩鏃讹紝鎵€鏈夋ˉ鏈€鍒濋兘鍋囧畾鑷繁鏄牴妗ャ€傚畠浠紑濮嬪悜閭诲眳閫氬憡 Bridge Protocol Data Units锛圔PDU锛屾ˉ鍗忚鏁版嵁鍗曞厓锛夛紝鍏朵腑鍖呭惈鑷韩鐨?Bridge ID 鍙婂叾浠栦俊鎭€?  4. BPDU Comparison锛圔PDU 姣旇緝锛夛細妗ヤ箣闂寸浉浜掍氦鎹?BPDU 浠ョ‘瀹氭牴妗ャ€傛瘡涓ˉ妫€鏌ユ敹鍒扮殑 BPDU锛堝寘鎷?Bridge Priority 涓?Bridge ID锛夛紝鏉ュ垽鏂槸鍚﹀簲璋冩暣鑷韩鐨勪紭鍏堢骇銆侭ridge ID 鏈€灏忕殑妗ュ皢鎴愪负鏍规ˉ銆?  5. Root Bridge Announcement锛堟牴妗ラ€氬憡锛夛細涓€鏃︾‘瀹氫簡鏍规ˉ锛屽畠灏变細鍚戠綉缁滀腑鎵€鏈夊叾浠栨ˉ鍙戦€佸寘鍚牴妗ヤ俊鎭殑 BPDU銆傚叾浠栨ˉ鍒╃敤杩欎簺淇℃伅璁＄畻鍑哄埌鏍规ˉ鐨勬渶鐭矾寰勶紝浠庤€屽垱寤烘棤鐜嫇鎵戙€?  6. Forwarding Ports锛堣浆鍙戠鍙ｏ級锛氭牴妗ラ€夊畾銆佺敓鎴愭爲鎷撴墤寤虹珛涔嬪悗锛屾瘡涓ˉ閮戒細纭畾鍏跺摢浜涚鍙ｅ簲澶勪簬杞彂鐘舵€侊紙鐢ㄤ簬鏁版嵁娴侀噺锛夈€佸摢浜涘簲澶勪簬闃诲鐘舵€侊紙鐢ㄤ簬闃叉鐜矾锛夈€傛牴妗ョ殑鎵€鏈夌鍙ｉ兘澶勪簬杞彂鐘舵€侊紝鑰屽叾浠栨ˉ鍒欐湁涓€浜涚鍙ｅ浜庨樆濉炵姸鎬佷互閬垮厤鐜矾銆?  7. Root Ports锛堟牴绔彛锛夛細鏍规ˉ閫夊畾銆佺敓鎴愭爲鎷撴墤寤虹珛涔嬪悗锛屾瘡涓潪鏍规ˉ澶勭悊鏀跺埌鐨?BPDU锛屽苟鏍规嵁鍏朵腑淇℃伅纭畾鍝釜绔彛鎻愪緵浜嗗埌鏍规ˉ鐨勬渶鐭矾寰勩€傝绔彛琚寚瀹氫负鏍圭鍙ｏ紝涓斿浜?Forwarding锛堣浆鍙戯級鐘舵€侊紝鍙富鍔ㄨ浆鍙戠綉缁滄祦閲忋€?  8. Designated ports锛堟寚瀹氱鍙ｏ級锛氭寚瀹氱鍙ｆ槸闈炴牴妗ョ敤鏉ュ悜鎸囧畾缃戞杞彂娴侀噺鐨勭鍙ｃ€傛寚瀹氱鍙ｈ缃簬 Forwarding锛堣浆鍙戯級鐘舵€併€傞潪鏍规ˉ涓婃墍鏈夋湭琚寚瀹氱粰鐗瑰畾缃戞鐨勭鍙ｉ兘琚疆浜?Blocking锛堥樆濉烇級鐘舵€侊紝浠ラ槻姝㈢綉缁滅幆璺€?
STP 閫氳繃璁＄畻鏈€鐭矾寰勫苟绂佺敤鍐椾綑閾捐矾鏉ヤ繚闅滅綉缁滄敹鏁涖€傚綋缃戠粶鎷撴墤鍙戠敓鍙樺寲锛堜緥濡傞摼璺晠闅滐級鏃讹紝STP 浼氶噸鏂拌绠楃綉缁滄嫇鎵戯紝鍦ㄩ伩鍏嶇幆璺殑鍚屾椂鎭㈠杩為€氭€с€?
瀵?STP 鍙傛暟锛堜緥濡?bridge priority锛屾ˉ浼樺厛绾э級鐨勬纭厤缃紝浼氬奖鍝嶇綉缁滄€ц兘銆佽矾寰勯€夋嫨浠ュ強鍝釜妗ユ垚涓烘牴妗ワ紙Root Bridge锛夈€?
### 鐢ㄦ埛绌洪棿 STP 杈呭姪绋嬪簭

鐢ㄦ埛绌洪棿鐨?STP 杈呭姪绋嬪簭 **bridge-stp** 鏄竴涓敤浜庢帶鍒舵槸鍚︿娇鐢ㄧ敤鎴锋ā寮忕敓鎴愭爲锛坰panning tree锛夌殑绋嬪簭銆傚綋妗ヤ笂鍚敤/绂佺敤 STP 鏃讹紙閫氳繃 `brctl stp <bridge> <on|off>` 鎴?``ip link set <bridge> type bridge
stp_state <0|1>``锛夛紝鍐呮牳浼氳皟鐢?`/sbin/bridge-stp <bridge> <start|stop>`銆傝嫢璇ュ懡浠よ繑鍥?0锛屽唴鏍稿惎鐢?user_stp 妯″紡锛涜嫢杩斿洖鍏朵粬鍊硷紝鍒欏惎鐢?kernel_stp 妯″紡銆?
### STP 妯″紡閫夋嫨

`IFLA_BR_STP_MODE` 妗ユ帴灞炴€у厑璁稿湪 STP 鍚敤鏃舵樉寮忔帶鍒跺叾杩愪綔鏂瑰紡锛屽浜?`user` 涓?`kernel` 妯″紡鍙畬鍏ㄧ粫杩?`/sbin/bridge-stp` 杈呭姪绋嬪簭銆?
   :doc: Bridge STP mode values

榛樿妯″紡涓?`BR_STP_MODE_AUTO`锛屼繚鐣欎簡璋冪敤 `/sbin/bridge-stp` 杈呭姪绋嬪簭鐨勪紶缁熻涓恒€俙user` 涓?`kernel` 妯″紡鍦?helper 鏈哄埗涓嶅彲鐢ㄧ殑缃戠粶鍛藉悕绌洪棿鐜涓挨鍏舵湁鐢紝鍥犱负 `call_usermodehelper()` 琚檺鍒跺湪鍒濆缃戠粶鍛藉悕绌洪棿涓€?
```

  ip link set dev br0 type bridge stp_mode user stp_state 1

```

璇ユā寮忓彧鑳藉湪 STP 琚鐢ㄦ椂淇敼銆?
## VLAN锛堣櫄鎷熷眬鍩熺綉锛?
LAN锛圠ocal Area Network锛屽眬鍩熺綉锛夋槸瑕嗙洊杈冨皬鍦扮悊鍖哄煙鐨勭綉缁滐紝閫氬父浣嶄簬涓€鏍嬪缓绛戞垨涓€涓洯鍖哄唴銆侺AN 鐢ㄤ簬杩炴帴 localized 鍖哄煙鍐呯殑璁＄畻鏈恒€佹湇鍔″櫒銆佹墦鍗版満鍙婂叾浠栬仈缃戣澶囥€侺AN 鍙互鏄湁绾跨殑锛堜娇鐢ㄤ互澶綉鐢电紗锛夋垨鏃犵嚎鐨勶紙浣跨敤 Wi-Fi锛夈€?
VLAN锛圴irtual Local Area Network锛岃櫄鎷熷眬鍩熺綉锛夋槸瀵圭墿鐞嗙綉缁滅殑閫昏緫鍒嗗壊锛屽舰鎴愬涓浉浜掗殧绂荤殑骞挎挱鍩熴€俈LAN 鐢ㄤ簬灏嗕竴涓墿鐞?LAN 鍒掑垎涓哄涓櫄鎷?LAN锛屼娇涓嶅悓缁勭殑璁惧鍙互鍍忚韩澶勭嫭绔嬬殑鐗╃悊缃戠粶涓€鏍风浉浜掗€氫俊銆?
閫氬父鏈変袱绉?VLAN 瀹炵幇锛欼EEE 802.1Q 涓?IEEE 802.1ad锛堜篃绉?QinQ锛夈€侷EEE 802.1Q 鏄互澶綉涓?VLAN 鏍囪锛坱agging锛夌殑鏍囧噯銆傚畠鍏佽缃戠粶绠＄悊鍛樺湪鐗╃悊缃戠粶涓婂垱寤洪€昏緫 VLAN锛屽苟鐢ㄤ互 VLAN 淇℃伅鏍囪浠ュお缃戝抚锛岃繖琚О涓?*VLAN 鏍囪甯э紙VLAN-tagged frames锛?*銆侷EEE 802.1ad 閫氬父绉颁负 QinQ 鎴?Double VLAN锛屾槸 IEEE 802.1Q 鏍囧噯鐨勬墿灞曘€俀inQ 鍏佽鍦ㄥ崟涓互澶綉甯у唴鍫嗗彔澶氫釜 VLAN 鏍囪銆侺inux 妗ュ悓鏃舵敮鎸?IEEE 802.1Q 浠ュ強 `802.1AD
<https://lore.kernel.org/netdev/1402401565-15423-1-git-send-email-makita.toshiaki@lab.ntt.co.jp/>`_
杩欎袱绉嶇敤浜?VLAN 鏍囪鐨勫崗璁€?
`VLAN filtering <https://lore.kernel.org/netdev/1360792820-14116-1-git-send-email-vyasevic@redhat.com/>`_
鍦ㄦˉ涓婇粯璁ゆ槸绂佺敤鐨勩€傚湪妗ヤ笂鍚敤 VLAN filtering 鍚庯紝瀹冨皢渚濇嵁鐩殑 MAC 鍦板潃涓?VLAN 鏍囪锛堜袱鑰呴兘蹇呴』鍖归厤锛夋妸甯ц浆鍙戝埌鍚堥€傜殑鐩爣銆?
## 缁勬挱锛圡ulticast锛?
Linux 妗ユ帴椹卞姩鏀寔缁勬挱锛屼娇鍏惰兘澶熷鐞?Internet Group Management Protocol锛圛GMP锛屽洜鐗圭綉缁勭鐞嗗崗璁級鎴?Multicast Listener Discovery锛圡LD锛岀粍鎾睛鍚€呭彂鐜帮級娑堟伅锛屽苟楂樻晥鍦拌浆鍙戠粍鎾暟鎹寘銆傝妗ユ帴椹卞姩鏀寔 IGMPv2/IGMPv3 涓?MLDv1/MLDv2銆?
### 缁勬挱渚﹀惉锛圡ulticast snooping锛?
Multicast snooping 鏄竴椤圭綉缁滄妧鏈紝瀹冧娇缃戠粶浜ゆ崲鏈鸿兘澶熷湪灞€鍩熺綉锛圠AN锛夊唴鏅鸿兘鍦扮鐞嗙粍鎾祦閲忋€?
浜ゆ崲鏈轰細缁存姢涓€寮犵粍鎾粍琛紝璁板綍缁勬挱缁勫湴鍧€涓庝富鏈哄凡鍔犲叆杩欎簺缁勭殑绔彛涔嬮棿鐨勫叧鑱斻€傝缁勮〃鏍规嵁鏀跺埌鐨?IGMP/MLD 娑堟伅鍔ㄦ€佹洿鏂般€傚€熷姪閫氳繃 snooping 鏀堕泦鐨勭粍鎾粍淇℃伅锛屼氦鎹㈡満浼樺寲缁勬挱娴侀噺鐨勮浆鍙戙€傚畠涓嶄細鐩茬洰鍦板皢缁勬挱娴侀噺骞挎挱鍒版墍鏈夌鍙ｏ紝鑰屾槸浠呮牴鎹洰鐨?MAC 鍦板潃灏嗙粍鎾祦閲忓彂閫佸埌宸茶闃呯浉搴旂洰鐨勭粍鎾粍鐨勭鍙ｃ€?
Linux 妗ユ帴璁惧鍦ㄥ垱寤烘椂榛樿鍚敤 multicast snooping銆傚畠浼氱淮鎶や竴涓?Multicast forwarding database锛圡DB锛岀粍鎾浆鍙戣〃锛夛紝鐢ㄤ簬璁板綍绔彛涓庣粍涔嬮棿鐨勫叧绯汇€?
### IGMPv3/MLDv2 EHT 鏀寔

Linux 妗ユ敮鎸?IGMPv3/MLDv2 EHT锛圗xplicit Host Tracking锛屾樉寮忎富鏈鸿窡韪級锛屽畠鐢?`474ddb37fa3a ("net: bridge: multicast: add EHT allow/block handling")
<https://lore.kernel.org/netdev/20210120145203.1109140-1-razor@blackwall.org/>`_
鍔犲叆銆?
鏄惧紡涓绘満璺熻釜浣胯澶囪兘澶熻褰曞姞鍏ユ煇涓壒瀹氱粍鎴栭€氶亾鐨勬瘡涓€鍙扮嫭绔嬩富鏈恒€侷GMP 涓樉寮忎富鏈鸿窡韪殑涓昏濂藉锛屾槸鑳藉鍦ㄤ富鏈虹寮€鏌愪釜缁勬挱缁勬垨閫氶亾鏃跺疄鐜版渶灏忕殑绂诲紑寤惰繜锛坙eave latency锛夈€?
浠庝富鏈烘兂瑕佺寮€鍒拌澶囧仠姝㈣浆鍙戞祦閲忎箣闂寸殑鏃堕棿闂撮殧绉颁负 IGMP leave latency锛堢寮€寤惰繜锛夈€傞厤缃簡 IGMPv3 鎴?MLDv2 骞跺紑鍚樉寮忚窡韪殑璁惧锛屽湪鏈€鍚庤姹傛帴鏀惰璁惧娴侀噺鐨勪富鏈鸿〃绀轰笉鍐嶅笇鏈涙帴鏀舵祦閲忔椂锛屽彲绔嬪嵆鍋滄杞彂娴侀噺銆傚洜姝わ紝绂诲紑寤惰繜浠呭彈澶氳矾璁块棶缃戠粶涓殑鏁版嵁鍖呬紶杈撳欢杩熶互鍙婅澶囧鐞嗘椂闂寸殑闄愬埗銆?
### 鍏朵粬缁勬挱鐗规€?
Linux 妗ヨ繕鏀寔 `per-VLAN multicast snooping
<https://lore.kernel.org/netdev/20210719170637.435541-1-razor@blackwall.org/>`_
锛堥粯璁ょ鐢ㄤ絾鍙惎鐢級锛屼互鍙?`Multicast Router Discovery
<https://lore.kernel.org/netdev/20190121062628.2710-1-linus.luessing@c0d3.blue/>`_
锛堢粍鎾矾鐢卞櫒鍙戠幇锛夛紝鍚庤€呯敤浜庡府鍔╄瘑鍒粍鎾矾鐢卞櫒鐨勪綅缃€?
## Switchdev

Linux Bridge Switchdev 鏄?Linux 鍐呮牳涓殑涓€椤圭壒鎬э紝瀹冩墿灞曚簡浼犵粺 Linux 妗ョ殑鑳藉姏锛屼娇鍏惰兘涓庢敮鎸?switchdev 鐨勭‖浠朵氦鎹㈡満鏇撮珮鏁堝湴鍗忓悓宸ヤ綔銆傚€熷姪 Linux Bridge Switchdev锛岃浆鍙戙€佽繃婊ゃ€佸涔犱互澶綉甯х瓑鏌愪簺缃戠粶鍔熻兘鍙鍗歌浇锛坥ffload锛夊埌纭欢浜ゆ崲鏈轰笂銆傝繖绉嶅嵏杞藉噺杞讳簡 Linux 鍐呮牳涓?CPU 鐨勮礋鎷咃紝浠庤€屾彁鍗囩綉缁滄€ц兘骞堕檷浣庡欢杩熴€?
瑕佷娇鐢?Linux Bridge Switchdev锛屼綘闇€瑕佹敮鎸?switchdev 鎺ュ彛鐨勭‖浠朵氦鎹㈡満銆傝繖鎰忓懗鐫€浜ゆ崲鏈虹‖浠跺繀椤诲叿澶囧繀瑕佺殑椹卞姩涓庡姛鑳斤紝鎵嶈兘涓?Linux 鍐呮牳鍗忓悓宸ヤ綔銆?
鏇村缁嗚妭璇峰弬闃?switchdev 鏂囨。銆?
## Netfilter

bridge netfilter 妯″潡鏄竴椤归仐鐣欑壒鎬э紝瀹冨厑璁镐娇鐢?iptables 涓?ip6tables 杩囨护琚ˉ鎺ョ殑鏁版嵁鍖呫€備笉鎺ㄨ崘浣跨敤瀹冿紝鐢ㄦ埛搴旇€冭檻浣跨敤 nftables 杩涜鍖呰繃婊ゃ€?
杈冭€佺殑 ebtables 宸ュ叿鐩告瘮 nftables 鍔熻兘鏇翠负鏈夐檺锛屼絾鍜?nftables 涓€鏍凤紝瀹冧篃涓嶉渶瑕佹妯″潡鍗冲彲宸ヤ綔銆?
br_netfilter 妯″潡浼氭嫤鎴繘鍏ユˉ鐨勬暟鎹寘锛屽 ipv4 涓?ipv6 鏁版嵁鍖呮墽琛屾渶鍩烘湰鐨勫仴鍏ㄦ€ф鏌ワ紝鐒跺悗鍋囪杩欎簺鏁版嵁鍖呮鍦ㄨ璺敱鑰岄潪妗ユ帴銆傞殢鍚?br_netfilter 浠庢ˉ鎺ュ眰璋冪敤 ip 涓?ipv6 鐨?netfilter 閽╁瓙锛屼篃灏辨槸璇?ip(6)tables 瑙勫垯闆嗕篃浼氱湅鍒拌繖浜涙暟鎹寘銆?
br_netfilter 涔熸槸 iptables **physdev** 鍖归厤瀛樺湪鐨勫師鍥狅細鍦?iptables 瑙勫垯闆嗕腑锛屾鍖归厤鏄彲闈犲尯鍒嗚矾鐢卞寘涓庢ˉ鎺ュ寘鐨勫敮涓€鏂瑰紡銆?
娉ㄦ剰锛宔btables 涓?nftables 鍦ㄦ病鏈?br_netfilter 妯″潡鏃朵篃鑳芥甯稿伐浣溿€俰ptables/ip6tables/arptables 瀵规ˉ鎺ユ祦閲忎笉璧蜂綔鐢紝鍥犱负瀹冧滑鎻掑叆浜嗚矾鐢辨爤銆俰p/ip6/inet/arp 鏃忕殑 nftables 瑙勫垯鍚屾牱鐪嬩笉鍒扮敱妗ヨ浆鍙戠殑鏁版嵁鍖咃紝浣嗚繖鏈潵灏辨槸搴旀湁鐨勮涓恒€?
鍘嗗彶涓?ebtables 鐨勫姛鑳介泦闈炲父鏈夐檺锛堣嚦浠婁粛鏄姝わ級锛屽姞鍏ユ妯″潡鏄负浜嗗亣瑁呮暟鎹寘琚矾鐢憋紝骞朵粠妗ユ帴灞傝皟鐢?ipv4/ipv6 鐨?netfilter 閽╁瓙锛屼娇鐢ㄦ埛寰椾互浣跨敤鍔熻兘鏇翠赴瀵岀殑 iptables 鍖归厤鑳藉姏锛堝寘鎷?conntrack锛夈€俷ftables 娌℃湁杩欑闄愬埗锛屽嚑涔庡叏閮ㄧ壒鎬ч兘涓嶅彈鍗忚鏃忓奖鍝嶈€屾甯稿伐浣溿€?
鍥犳锛屽彧鏈夊湪鐢ㄦ埛鍑轰簬鏌愪簺鍘熷洜闇€瑕佷娇鐢?ip(6)tables 鏉ヨ繃婊ょ敱妗ヨ浆鍙戠殑鏁版嵁鍖咃紝鎴栧妗ユ帴娴侀噺鍋?NAT 鏃讹紝鎵嶉渶瑕?br_netfilter銆傚浜庣函閾捐矾灞傝繃婊わ紝鍒欎笉闇€瑕佹妯″潡銆?
## 鍏朵粬鐗规€?
Linux 妗ヨ繕鏀寔 `IEEE 802.11 Proxy ARP
<https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=958501163ddd6ea22a98f94fa0e7ce6d4734e5c4>`_銆?`Media Redundancy Protocol (MRP)
<https://lore.kernel.org/netdev/20200426132208.3232-1-horatiu.vultur@microchip.com/>`_銆?`Media Redundancy Protocol (MRP) LC mode
<https://lore.kernel.org/r/20201124082525.273820-1-horatiu.vultur@microchip.com>`_銆?`IEEE 802.1X port authentication
<https://lore.kernel.org/netdev/20220218155148.2329797-1-schultz.hans+netdev@gmail.com/>`_锛?浠ュ強 `MAC Authentication Bypass (MAB)
<https://lore.kernel.org/netdev/20221101193922.2125323-2-idosch@nvidia.com/>`_銆?
## 甯歌闂锛團AQ锛?
### 妗ョ殑浣滅敤鏄粈涔堬紵

妗ュ湪澶氫釜缃戠粶鎺ュ彛涔嬮棿閫忔槑鍦拌浆鍙戞祦閲忋€傞€氫織鍦拌锛岃繖鎰忓懗鐫€妗ュ皢涓や釜鎴栧涓墿鐞嗕互澶綉缃戠粶杩炴帴鍦ㄤ竴璧凤紝褰㈡垚涓€涓洿澶х殑锛堥€昏緫涓婄殑锛変互澶綉缃戠粶銆?
### 瀹冩槸鍚︿笌 L3 鍗忚鏃犲叧锛?
鏄殑銆傛ˉ浼氱湅鍒版墍鏈夊抚锛屼絾瀹?*浠呬娇鐢?* L2 澶撮儴/淇℃伅銆傚洜姝わ紝妗ユ帴鍔熻兘涓庡崗璁棤鍏筹紝杞彂 IPX銆丯etBEUI銆両P銆両Pv6 绛夐兘涓嶄細鏈夐棶棰樸€?
## 鑱旂郴淇℃伅

璇ヤ唬鐮佺洰鍓嶇敱 Roopa Prabhu <roopa@nvidia.com> 涓?Nikolay Aleksandrov <razor@blackwall.org> 缁存姢銆傛ˉ鐨勭己闄蜂笌澧炲己鍦?linux-netdev 閭欢鍒楄〃 netdev@vger.kernel.org 浠ュ強 bridge@lists.linux.dev 涓婅璁恒€?
璇ュ垪琛ㄥ浠讳綍鎰熷叴瓒ｇ殑浜哄紑鏀撅細http://vger.kernel.org/vger-lists.html#netdev

## 澶栭儴閾炬帴

Linux 妗ユ帴鐨勬棫鐗堟枃妗ｄ綅浜庯細
https://wiki.linuxfoundation.org/networking/bridge
