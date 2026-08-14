
## Linux 浠ュお缃?Bonding 椹卞姩浣跨敤鎵嬪唽锛圚OWTO锛?


鏈€杩戞洿鏂帮細2011 骞?4 鏈?27 鏃?

鍒濇鍙戝竷锛歍homas Davis <tadavis at lbl.gov>

淇涓庨珮鍙敤鎵╁睍锛?000/10/03-15锛?

  - Willy Tarreau <willy at meta-x.org>
  - Constantine Gavrilov <const-g at xpert.com>
  - Chad N. Tindel <ctindel at ieee dot org>
  - Janice Girouard <girouard at us dot ibm dot com>
  - Jay Vosburgh <fubar at us dot ibm dot com>

鐢?Jay Vosburgh 浜?2005 骞?2 鏈堥噸鏂扮粍缁囧苟鏇存柊
鏂板 Sysfs 淇℃伅锛?006/04/24

  - Mitch Williams <mitch.a.williams at intel.com>

## 绠€浠?


Linux bonding 椹卞姩鎻愪緵浜嗕竴绉嶅皢澶氫釜缃戠粶鎺ュ彛鑱氬悎涓哄崟涓€閫昏緫
鈥渂onded鈥濇帴鍙ｇ殑鏂规硶銆俠onded 鎺ュ彛鐨勮涓哄彇鍐充簬鎵€浣跨敤鐨勬ā寮忥紱涓€鑸?
鑰岃█锛屽悇绉嶆ā寮忔彁渚涚儹澶囦唤鎴栬礋杞藉潎琛℃湇鍔°€傛澶栵紝杩樺彲浠ユ墽琛岄摼璺?
瀹屾暣鎬х洃鎺с€?

bonding 椹卞姩鏈€鍒濇潵婧愪簬 Donald Becker 涓?2.0 鍐呮牳缂栧啓鐨?
beowulf 琛ヤ竵銆傛鍚庡畠宸茬粡鍙戠敓浜嗗緢澶у彉鍖栵紝鏉ヨ嚜 extreme-linux 鍜?
beowulf 绔欑偣鐨勫師濮嬪伐鍏峰皢鏃犳硶涓庢鐗堟湰鐨勯┍鍔ㄤ竴璧峰伐浣溿€?

鏈夊叧椹卞姩鐨勬柊鐗堟湰銆佹洿鏂板悗鐨勭敤鎴锋€佸伐鍏凤紝浠ュ強鍚戣皝瀵绘眰甯姪锛岃
鍙傝鏈枃浠舵湯灏剧殑閾炬帴銆?


   1. Bonding 椹卞姩瀹夎

   2. Bonding 椹卞姩閫夐」

   3. 閰嶇疆 Bonding 璁惧
   3.1	浣跨敤 Sysconfig 鏀寔杩涜閰嶇疆
   3.1.1		鍦?Sysconfig 涓娇鐢?DHCP
   3.1.2		鍦?Sysconfig 涓厤缃涓?Bond
   3.2	浣跨敤 Initscripts 鏀寔杩涜閰嶇疆
   3.2.1		鍦?Initscripts 涓娇鐢?DHCP
   3.2.2		鍦?Initscripts 涓厤缃涓?Bond
   3.3	浣跨敤 Ifenslave 鎵嬪姩閰嶇疆 Bonding
   3.3.1		鎵嬪姩閰嶇疆澶氫釜 Bond
   3.4	閫氳繃 Sysfs 鎵嬪姩閰嶇疆 Bonding
   3.5	浣跨敤 Interfaces 鏀寔杩涜閰嶇疆
   3.6	鐗规畩鎯呭喌涓嬬殑閰嶇疆瑕嗙洊
   3.7 浠ユ洿瀹夊叏鐨勬柟寮忎负 802.3ad 妯″紡閰嶇疆 LACP

   4. 鏌ヨ Bonding 閰嶇疆
   4.1	Bonding 閰嶇疆
   4.2	缃戠粶閰嶇疆

   5. 浜ゆ崲鏈洪厤缃?

   6. 802.1q VLAN 鏀寔

   7. 閾捐矾鐩戞帶
   7.1	ARP 鐩戞帶宸ヤ綔鏈哄埗
   7.2	閰嶇疆澶氫釜 ARP 鐩爣
   7.3	MII 鐩戞帶宸ヤ綔鏈哄埗

   8. 娼滃湪鐨勬晠闅滄潵婧?
   8.1	璺敱鏂归潰鐨勫潙
   8.2	浠ュお缃戣澶囬噸鍛藉悕
   8.3	Miimon 妫€娴嬮摼璺け璐ユ瀬鎱㈡垨瀹屽叏澶辨晥

   9. SNMP 浠ｇ悊

   10. 娣锋潅妯″紡

   11. 涓?High Availability 閰嶇疆 Bonding
   11.1	鍗曚氦鎹㈡満鎷撴墤涓殑楂樺彲鐢?
   11.2	澶氫氦鎹㈡満鎷撴墤涓殑楂樺彲鐢?
   11.2.1		澶氫氦鎹㈡満鎷撴墤鐨?HA Bonding 妯″紡閫夋嫨
   11.2.2		澶氫氦鎹㈡満鎷撴墤鐨?HA 閾捐矾鐩戞帶閫夋嫨

   12. 涓烘渶澶у悶鍚愰噺閰嶇疆 Bonding
   12.1	鍗曚氦鎹㈡満鎷撴墤涓殑鏈€澶у悶鍚愰噺
   12.1.1		鍗曚氦鎹㈡満鎷撴墤鐨?MT Bonding 妯″紡閫夋嫨
   12.1.2		鍗曚氦鎹㈡満鎷撴墤鐨?MT 閾捐矾鐩戞帶
   12.2	澶氫氦鎹㈡満鎷撴墤涓殑鏈€澶у悶鍚愰噺
   12.2.1		澶氫氦鎹㈡満鎷撴墤鐨?MT Bonding 妯″紡閫夋嫨
   12.2.2		澶氫氦鎹㈡満鎷撴墤鐨?MT 閾捐矾鐩戞帶

   13. 浜ゆ崲鏈鸿涓洪棶棰?
   13.1	閾捐矾寤虹珛涓庢晠闅滃垏鎹㈠欢杩?
   13.2	閲嶅鐨勫叆绔欐暟鎹寘

   14. 纭欢鐩稿叧娉ㄦ剰浜嬮」
   14.1	IBM BladeCenter

   15. 甯歌闂瑙ｇ瓟

   16. 璧勬簮涓庨摼鎺?


## 1. Bonding 椹卞姩瀹夎


澶у鏁版祦琛岀殑鍙戣鐗堝唴鏍搁兘闄勫甫 bonding 椹卞姩锛屽凡缁忎綔涓烘ā鍧?
鎻愪緵銆傚鏋滀綘鐨勫彂琛岀増娌℃湁闄勫甫锛屾垨鑰呬綘闇€瑕佷粠婧愮爜缂栬瘧 bonding锛堜緥濡傦紝
閰嶇疆骞跺畨瑁呮潵鑷?kernel.org 鐨?mainline 鍐呮牳锛夛紝鍒欓渶瑕佹墽琛屼互涓嬫楠わ細

### 1.1 閰嶇疆骞舵瀯寤哄甫 bonding 鐨勫唴鏍?


鏈€鏂扮増鏈殑 bonding 椹卞姩浣嶄簬鏈€杩戠殑鍐呮牳婧愮爜鐨?
drivers/net/bonding 瀛愮洰褰曚腑锛堝彲鍦?http://kernel.org 鑾峰彇锛夈€傚ぇ澶氭暟
鈥滆嚜琛屾瀯寤衡€濈殑鐢ㄦ埛浼氬笇鏈涗娇鐢ㄦ潵鑷?kernel.org 鐨勬渶鏂板唴鏍搞€?

浣跨敤 "make menuconfig"锛堟垨 "make xconfig" 鎴?
"make config"锛夐厤缃唴鏍革紝鐒跺悗鍦?"Network
device support" 涓€鑺備腑閫夋嫨 "Bonding driver support"銆傚缓璁皢椹卞姩
閰嶇疆涓烘ā鍧楋紝鍥犱负杩欐槸鐩墠鍚戦┍鍔ㄤ紶閫掑弬鏁版垨閰嶇疆澶氫釜 bonding 璁惧鐨?
鍞竴鏂瑰紡銆?

鏋勫缓骞跺畨瑁呮柊鐨勫唴鏍镐笌妯″潡銆?

### 1.2 Bonding 鎺у埗宸ュ叿


寤鸿閫氳繃 iproute2锛坣etlink锛夋垨 sysfs 閰嶇疆 bonding锛屾棫鐨?
ifenslave 鎺у埗宸ュ叿宸茬粡杩囨椂銆?

## 2. Bonding 椹卞姩閫夐」


bonding 椹卞姩鐨勯€夐」鍦ㄥ姞杞芥椂浣滀负 bonding 妯″潡鐨勫弬鏁版彁渚涳紝鎴?
閫氳繃 sysfs 鎸囧畾銆?

妯″潡閫夐」鍙互浣滀负鍛戒护琛屽弬鏁颁紶缁?insmod 鎴?modprobe 鍛戒护锛屼絾
閫氬父鍐欏湪 `/etc/modprobe.d/*.conf` 閰嶇疆鏂囦欢涓紝鎴栧啓鍦ㄦ煇涓彂琛岀増
鐗瑰畾鐨勯厤缃枃浠朵腑锛堝叾涓儴鍒嗕細鍦ㄤ笅涓€鑺傝缁嗕粙缁嶏級銆?

鏈夊叧 bonding 瀵?sysfs 鐨勬敮鎸佽瑙佷笅闈㈢殑
鈥滈€氳繃 Sysfs 鎵嬪姩閰嶇疆 Bonding鈥濅竴鑺傘€?

鍙敤鐨?bonding 椹卞姩鍙傛暟濡備笅鎵€鍒椼€傚鏋滄湭鎸囧畾鏌愪釜鍙傛暟锛屽垯浣跨敤鍏?
榛樿鍊笺€傚湪鍒濇閰嶇疆 bond 鏃讹紝寤鸿鍦ㄤ竴涓嫭绔嬬獥鍙ｄ腑杩愯
"tail -f /var/log/messages" 浠ヨ瀵?bonding 椹卞姩鐨勯敊璇秷鎭€?

鍔″繀鎸囧畾 miimon 鎴?arp_interval 涓?arp_ip_target 鍙傛暟涓殑
鑷冲皯涓€涓紝鍚﹀垯鍦ㄩ摼璺晠闅滄湡闂翠細鍑虹幇涓ラ噸鐨勭綉缁滄€ц兘涓嬮檷銆傚緢灏戞湁璁惧
涓嶆敮鎸佽嚦灏?miimon锛屽洜姝ゅ疄鍦ㄦ病鏈夌悊鐢变笉浣跨敤瀹冦€?

甯︽湁鏂囨湰鍊肩殑閫夐」鏃㈡帴鍙楁枃鏈悕绉帮紝涔燂紙涓轰簡鍚戝悗鍏煎锛夋帴鍙楅€夐」
鏁板€笺€備緥濡傦紝"mode=802.3ad" 涓?"mode=4" 璁剧疆鐨勬槸鍚屼竴涓ā寮忋€?

鍙傛暟濡備笅锛?

active_slave

	鎸囧畾鏀寔璇ラ€夐」鐨勬ā寮忕殑鏂扮殑 active slave锛坅ctive-backup銆?
	balance-alb 鍜?balance-tlb锛夈€傚彲鑳界殑鍊间负褰撳墠浠讳竴琚?enslaved 鐨?
	鎺ュ彛鍚嶇О锛屾垨绌哄瓧绗︿覆銆傚鏋滅粰瀹氫簡鍚嶇О锛屽垯璇?slave 鍙婂叾閾捐矾蹇呴』
	澶勪簬 up 鐘舵€佹墠鑳借閫変腑涓烘柊鐨?active slave銆傚鏋滄寚瀹氫簡绌哄瓧绗︿覆锛?
	鍒欐竻闄ゅ綋鍓嶇殑 active slave锛屽苟鑷姩閫夋嫨涓€涓柊鐨?active slave銆?

	娉ㄦ剰锛屾閫夐」浠呴€氳繃 sysfs 鎺ュ彛鎻愪緵銆備笉瀛樺湪浠ユ鍛藉悕鐨勬ā鍧楀弬鏁般€?

	姝ら€夐」鐨勬甯稿€间负褰撳墠 active slave 鐨勫悕绉帮紝鎴栬€呭湪娌℃湁
	active slave 鎴栧綋鍓嶆ā寮忎笉浣跨敤 active slave 鏃朵负绌哄瓧绗︿覆銆?

ad_actor_sys_prio

	鍦?AD 绯荤粺涓紝姝ゅ弬鏁版寚瀹氱郴缁熶紭鍏堢骇銆傚厑璁哥殑鑼冨洿鏄?
	1 - 65535銆傚鏋滄湭鎸囧畾璇ュ€硷紝鍒欓粯璁ゅ彇 65535銆?

	姝ゅ弬鏁颁粎鍦?802.3ad 妯″紡涓嬬敓鏁堬紝骞朵笖閫氳繃 SysFs 鎺ュ彛鎻愪緵銆?

actor_port_prio

	鍦?AD 绯荤粺涓紝姝ゅ弬鏁版寚瀹氱鍙ｄ紭鍏堢骇銆傚厑璁哥殑鑼冨洿鏄?
	1 - 65535銆傚鏋滄湭鎸囧畾璇ュ€硷紝鍒欓粯璁ゅ彇 255銆?

	姝ゅ弬鏁颁粎鍦?802.3ad 妯″紡涓嬬敓鏁堬紝骞朵笖閫氳繃 netlink 鎺ュ彛鎻愪緵銆?

ad_actor_system

	鍦?AD 绯荤粺涓紝姝ゅ弬鏁版寚瀹?actor 鍦ㄥ崗璁姤鏂囦氦鎹紙LACPDU锛変腑
	浣跨敤鐨?mac 鍦板潃銆傝鍊间笉鑳芥槸缁勬挱鍦板潃銆傚鏋滄寚瀹氫簡鍏ㄩ浂 MAC锛?
	bonding 灏嗗湪鍐呴儴浣跨敤 bond 鑷韩鐨?MAC銆傚缓璁负姝?mac 璁剧疆
	local-admin 浣嶏紝浣嗛┍鍔ㄥ苟涓嶅己鍒惰姹傘€傚鏋滄湭缁欏嚭璇ュ€硷紝鍒欑郴缁?
	榛樿浣跨敤 master 鐨?mac 鍦板潃浣滀负 actor 鐨勭郴缁熷湴鍧€銆?

	姝ゅ弬鏁颁粎鍦?802.3ad 妯″紡涓嬬敓鏁堬紝骞朵笖閫氳繃 SysFs 鎺ュ彛鎻愪緵銆?

ad_select

	鎸囧畾瑕佷娇鐢ㄧ殑 802.3ad 鑱氬悎閫夋嫨閫昏緫銆傚彲鑳界殑鍊煎強鍏舵晥鏋滃涓嬶細

	stable 鎴?0

		娲诲姩鑱氬悎鍣ㄧ敱鏈€澶х殑鑱氬悎甯﹀閫夊嚭銆?

		浠呭綋娲诲姩鑱氬悎鍣ㄧ殑鎵€鏈?slave 閮藉凡 down锛屾垨娲诲姩鑱氬悎鍣?
		宸茬粡娌℃湁 slave 鏃讹紝鎵嶄細閲嶆柊閫夋嫨娲诲姩鑱氬悎鍣ㄣ€?

		杩欐槸榛樿鍊笺€?

	bandwidth 鎴?1

		娲诲姩鑱氬悎鍣ㄧ敱鏈€澶х殑鑱氬悎甯﹀閫夊嚭銆傚湪浠ヤ笅鎯呭喌浼?
		鍙戠敓閲嶆柊閫夋嫨锛?

  - 鍚?bond 涓鍔犳垨浠?bond 涓Щ闄ゆ煇涓?slave

  - 浠绘剰 slave 鐨勯摼璺姸鎬佸彂鐢熷彉鍖?

  - 浠绘剰 slave 鐨?802.3ad 鍏宠仈鐘舵€佸彂鐢熷彉鍖?

  - bond 鐨勭鐞嗙姸鎬佸彉涓?up

	count 鎴?2

		娲诲姩鑱氬悎鍣ㄧ敱鏈€澶氱殑绔彛鏁帮紙slave锛夐€夊嚭銆傞噸鏂伴€夋嫨鐨?
		鍙戠敓鏂瑰紡濡備笂鏂?鈥渂andwidth鈥?璁剧疆鎵€杩般€?

	actor_port_prio 鎴?3

		娲诲姩鑱氬悎鍣ㄧ敱鍏舵椿鍔ㄧ鍙ｄ笂 actor 绔彛浼樺厛绾ф€诲拰鏈€楂?
		鑰呴€夊嚭銆傛敞鎰忔浼樺厛绾ф槸 actor_port_prio锛岃€岄潪姣忎釜绔彛
		鐨勪紭鍏堢骇锛堝悗鑰呯敤浜?primary reselect锛夈€?

	bandwidth銆乧ount 鍜?actor_port_prio 閫夋嫨绛栫暐鍏佽鍦ㄦ椿鍔ㄨ仛鍚堝櫒
	鍙戠敓閮ㄥ垎鏁呴殰鏃惰繘琛?802.3ad 鑱氬悎鐨勬晠闅滃垏鎹€傝繖淇濊瘉浜嗗叿鏈夋渶楂?
	鍙敤鎬х殑鑱氬悎鍣紙鏃犺鏄甫瀹姐€佺鍙ｆ暟杩樻槸绔彛浼樺厛绾ф€诲€硷級濮嬬粓
	澶勪簬娲诲姩鐘舵€併€?

	姝ら€夐」娣诲姞浜?bonding 鐗堟湰 3.4.0銆?

ad_user_port_key

	鍦?AD 绯荤粺涓紝port-key 鐢卞涓嬩笁閮ㄥ垎缁勬垚 -

	   =====  ============
	   Bits   鐢ㄩ€?
	   =====  ============
	   00     鍙屽伐
	   01-05  閫熺巼
	   06-15  鐢ㄦ埛瀹氫箟
	   =====  ============

	姝ゅ弬鏁板畾涔?port key 鐨勯珮 10 浣嶃€傚彇鍊煎彲浠ヤ负 0 - 1023銆傚鏋滄湭缁欏嚭锛?
	鍒欑郴缁熼粯璁ゅ彇 0銆?

	姝ゅ弬鏁颁粎鍦?802.3ad 妯″紡涓嬬敓鏁堬紝骞朵笖閫氳繃 SysFs 鎺ュ彛鎻愪緵銆?

all_slaves_active

	鎸囧畾搴斾涪寮冿紙0锛夎繕鏄氦浠橈紙1锛夐噸澶嶅抚锛堝湪 inactive 绔彛涓婃帴鏀跺埌鐨勶級銆?

	閫氬父鎯呭喌涓嬶紝bonding 浼氫涪寮冮噸澶嶅抚锛堝湪 inactive 绔彛涓婃帴鏀跺埌鐨勶級锛?
	杩欏澶у鏁扮敤鎴疯€岃█鏄湡鏈涚殑琛屼负銆備絾鏈夋椂鍏佽浜や粯閲嶅甯т細鏇村ソ銆?

	榛樿鍊间负 0锛堜涪寮冨湪 inactive 绔彛涓婃帴鏀跺埌鐨勯噸澶嶅抚锛夈€?

arp_interval

	鎸囧畾 ARP 閾捐矾鐩戞帶鐨勯鐜囷紝鍗曚綅涓烘绉掋€?

	ARP 鐩戞帶閫氳繃瀹氭湡妫€鏌?slave 璁惧鏉ョ‘瀹氬畠浠渶杩戞槸鍚﹀彂閫佹垨
	鎺ユ敹杩囨祦閲忥紙纭垏鐨勫垽瀹氭爣鍑嗗彇鍐充簬 bonding 妯″紡浠ュ強 slave 鐨?
	鐘舵€侊級銆傚父瑙勬祦閲忕敱閽堝 arp_ip_target 閫夐」鎵€鎸囧畾鍦板潃鍙戝嚭鐨?
	ARP 鎺㈡祴鎶ユ枃浜х敓銆?

	姝よ涓哄彲鐢变笅闈㈢殑 arp_validate 閫夐」淇敼銆?

	濡傛灉鍦?etherchannel 鍏煎妯″紡锛堟ā寮?0 鍜?2锛変笅浣跨敤 ARP 鐩戞帶锛?
	鍒欎氦鎹㈡満搴旈厤缃负鑳藉鍧囧寑鍦板皢鏁版嵁鍖呭垎甯冨埌鎵€鏈夐摼璺笂鐨勬ā寮忋€傚鏋?
	浜ゆ崲鏈洪厤缃负浠?XOR 鏂瑰紡鍒嗗彂鏁版嵁鍖咃紝鍒欐潵鑷?ARP 鐩爣鐨勫叏閮ㄥ簲绛?
	閮戒細鍦ㄥ悓涓€涓摼璺笂鏀跺埌锛岃繖鍙兘瀵艰嚧鍏朵粬缁勬垚鍛樺け璐ャ€侫RP 鐩戞帶涓嶅簲
	涓?miimon 閰嶅悎浣跨敤銆傚彇鍊间负 0 鏃剁鐢?ARP 鐩戞帶銆傞粯璁ゅ€间负 0銆?

arp_ip_target

	鎸囧畾鍦?arp_interval 澶т簬 0 鏃剁敤浣?ARP 鐩戞帶瀵圭瓑浣撶殑 IP 鍦板潃銆?
	杩欎簺鍦板潃鏄敤浜庣‘瀹氬埌鍚勭洰鏍囩殑閾捐矾鍋ュ悍鐘跺喌鐨?ARP 璇锋眰鐨勭洰鏍囥€?
	浠?ddd.ddd.ddd.ddd 鏍煎紡鎸囧畾杩欎簺鍊笺€傚涓?IP 鍦板潃蹇呴』浠ラ€楀彿
	鍒嗛殧銆傚繀椤昏嚦灏戠粰鍑轰竴涓?IP 鍦板潃锛孉RP 鐩戞帶鎵嶈兘姝ｅ父宸ヤ綔銆傚彲浠ユ寚瀹氱殑
	鐩爣鏈€澶ф暟閲忎负 16銆傞粯璁ゅ€间负鏃?IP 鍦板潃銆?

ns_ip6_target

	鎸囧畾鍦?arp_interval 澶т簬 0 鏃剁敤浣?IPv6 鐩戞帶瀵圭瓑浣撶殑 IPv6 鍦板潃銆?
	杩欎簺鍦板潃鏄敤浜庣‘瀹氬埌鍚勭洰鏍囩殑閾捐矾鍋ュ悍鐘跺喌鐨?NS 璇锋眰鐨勭洰鏍囥€?
	浠?ffff:ffff::ffff:ffff 鏍煎紡鎸囧畾杩欎簺鍊笺€傚涓?IPv6 鍦板潃蹇呴』浠?
	閫楀彿鍒嗛殧銆傚繀椤昏嚦灏戠粰鍑轰竴涓?IPv6 鍦板潃锛孨S/NA 鐩戞帶鎵嶈兘姝ｅ父宸ヤ綔銆?
	鍙互鎸囧畾鐨勭洰鏍囨渶澶ф暟閲忎负 16銆傞粯璁ゅ€间负鏃?IPv6 鍦板潃銆?

arp_validate

	鎸囧畾鍦ㄤ换浣曟敮鎸?arp 鐩戞帶鐨勬ā寮忎笅锛屾槸鍚﹀簲鏍￠獙 ARP 鎺㈡祴涓庡簲绛旓紝
	鎴栬€呮槸鍚﹀簲杩囨护锛堝拷鐣ワ級闈?ARP 娴侀噺鐢ㄤ簬閾捐矾鐩戞帶鐩殑銆?

	鍙兘鐨勫€煎涓嬶細

	none 鎴?0

		涓嶆墽琛屼换浣曟牎楠屾垨杩囨护銆?

	active 鎴?1

		浠呭 active slave 鎵ц鏍￠獙銆?

	backup 鎴?2

		浠呭 backup slave 鎵ц鏍￠獙銆?

	all 鎴?3

		瀵规墍鏈?slave 鎵ц鏍￠獙銆?

	filter 鎴?4

		瀵规墍鏈?slave 搴旂敤杩囨护銆備笉鎵ц浠讳綍鏍￠獙銆?

	filter_active 鎴?5

		瀵规墍鏈?slave 搴旂敤杩囨护锛屼粎瀵?active slave 鎵ц鏍￠獙銆?

	filter_backup 鎴?6

		瀵规墍鏈?slave 搴旂敤杩囨护锛屼粎瀵?backup slave 鎵ц鏍￠獙銆?

	鏍￠獙锛?

	鍚敤鏍￠獙浼氫娇 ARP 鐩戞帶妫€鏌ュ叆绔欑殑 ARP 璇锋眰涓庡簲绛旓紝骞朵笖浠呭綋鏌愪釜
	slave 姝ｅ湪鎺ユ敹鐩稿簲鐨?ARP 娴侀噺鏃舵墠璁や负瀹冨浜?up 鐘舵€併€?

	瀵逛簬 active slave锛屾牎楠屼細妫€鏌?ARP 搴旂瓟锛屼互纭瀹冧滑鏄敱鏌愪釜
	arp_ip_target 鐢熸垚鐨勩€傜敱浜?backup slave 閫氬父涓嶄細鎺ユ敹鍒拌繖浜涘簲绛旓紝
	鍥犳瀵?backup slave 鎵ц鐨勬牎楠屾槸閽堝缁忕敱 active slave 鍙戝嚭鐨?
	骞挎挱 ARP 璇锋眰鐨勩€傛煇浜涗氦鎹㈡満鎴栫綉缁滈厤缃彲鑳藉鑷?backup slave
	鏀朵笉鍒?ARP 璇锋眰鐨勬儏鍐碉紱鍦ㄨ繖绉嶆儏鍐典笅锛屽繀椤荤鐢ㄥ backup slave 鐨?
	鏍￠獙銆?

	瀵?backup slave 鐨?ARP 璇锋眰鏍￠獙涓昏鏄负浜嗗府鍔?bonding 鍦?active
	slave 鍙戠敓鏁呴殰鏃跺垽鏂摢浜?slave 鏇村彲鑳芥甯稿伐浣滐紝瀹冨苟涓嶈兘鐪熸淇濊瘉
	璇?backup slave 鍦ㄨ閫変负涓嬩竴涓?active slave 鏃朵竴瀹氳兘宸ヤ綔銆?

	鏍￠獙鍦ㄧ綉缁滈厤缃腑鏈夊涓?bonding 涓绘満鍚屾椂鍚戜竴涓垨澶氫釜瓒呭嚭鍏叡
	浜ゆ崲鏈虹殑鐩爣鍙戝嚭 ARP 鏃跺緢鏈夌敤銆傚鏋滀氦鎹㈡満涓庣洰鏍囦箣闂寸殑閾捐矾澶辫触
	锛堜絾浜ゆ崲鏈烘湰韬湭澶辫触锛夛紝澶氫釜 bonding 瀹炰緥浜х敓鐨勬帰娴嬫祦閲忎細娆洪獥
	鏍囧噯鐨?ARP 鐩戞帶锛屼娇鍏惰涓洪摼璺粛鐒?up銆備娇鐢ㄦ牎楠屽彲浠ヨВ鍐宠繖涓棶棰橈紝
	鍥犱负 ARP 鐩戞帶鍙細鑰冭檻涓庡叾鑷韩 bonding 瀹炰緥鐩稿叧鐨?ARP 璇锋眰涓庡簲绛斻€?

	杩囨护锛?

	鍚敤杩囨护浼氫娇 ARP 鐩戞帶浠呬娇鐢ㄥ叆绔欑殑 ARP 鏁版嵁鍖呮潵鍒ゆ柇閾捐矾鍙敤鎬с€?
	鍒拌揪鐨勯潪 ARP 鏁版嵁鍖呬細鐓у父浜や粯锛屼絾鍦ㄥ垽鏂煇涓?slave 鏄惁鍙敤鏃?
	涓嶈璁″叆銆?

	杩囨护浠呰€冭檻鍦ㄥ垽鏂摼璺彲鐢ㄦ€ф椂鏄惁鎺ユ敹鍒颁簡 ARP 鏁版嵁鍖咃紙浠绘剰 ARP
	鏁版嵁鍖咃紝鏃犺婧愭垨鐩殑锛夈€?

	杩囨护鍦ㄤ互涓嬬綉缁滈厤缃腑寰堟湁鐢細澶ч噺绗笁鏂圭殑骞挎挱娴侀噺浼氭楠楁爣鍑嗙殑
	ARP 鐩戞帶锛屼娇鍏惰涓洪摼璺粛鐒?up銆備娇鐢ㄨ繃婊ゅ彲浠ヨВ鍐宠繖涓棶棰橈紝鍥犱负
	鍙湁 ARP 娴侀噺鎵嶄細琚敤浜庡垽鏂摼璺彲鐢ㄦ€с€?

	姝ら€夐」娣诲姞浜?bonding 鐗堟湰 3.1.0銆?

arp_all_targets

	鎸囧畾涓轰簡浣?ARP 鐩戞帶璁や负鏌愪釜 slave 澶勪簬 up 鐘舵€侊紝蹇呴』鍙揪鐨?
	arp_ip_target 鐨勬暟閲忋€傛閫夐」浠呭奖鍝嶅惎鐢ㄤ簡 arp_validation 鐨?
	active-backup 妯″紡涓嬬殑 slave銆?

	鍙兘鐨勫€煎涓嬶細

	any 鎴?0

		浠呭綋浠绘剰涓€涓?arp_ip_target 鍙揪鏃舵墠璁や负璇?slave 澶勪簬 up 鐘舵€?

	all 鎴?1

		浠呭綋鎵€鏈?arp_ip_target 閮藉彲杈炬椂鎵嶈涓鸿 slave 澶勪簬 up 鐘舵€?

arp_missed_max

	鎸囧畾蹇呴』鏈夊灏戞 arp_interval 鐩戞帶妫€鏌ュけ璐ワ紝鎺ュ彛鎵嶄細琚?ARP 鐩戞帶
	鏍囪涓?down銆?

	涓轰簡鎻愪緵鏈夊簭鐨勬晠闅滃垏鎹㈣涔夛紝backup 鎺ュ彛琚厑璁稿涓€娆＄洃鎺ф鏌?
	锛堝嵆瀹冧滑蹇呴』澶辫触 arp_missed_max + 1 娆℃墠浼氳鏍囪涓?down锛夈€?

	榛樿鍊间负 2锛屽厑璁哥殑鑼冨洿鏄?1 - 255銆?

coupled_control

    鎸囧畾 802.3ad 妯″紡涓嬬殑 LACP 鐘舵€佹満鐨?MUX 鏄惁搴斿叿鏈夌嫭绔嬬殑 Collecting
    涓?Distributing 鐘舵€併€?

    杩欐槸閫氳繃闄や簡鐜版湁鐨?coupled control 鐘舵€佹満涔嬪锛岃繕瀹炵幇閬靛惊
    IEEE 802.1AX-2008 5.4.15 鐨勭嫭绔嬫帶鍒剁姸鎬佹満鏉ュ畬鎴愮殑銆?

    榛樿鍊间负 1銆傛璁剧疆涓嶄細鍒嗙 Collecting 涓?Distributing 鐘舵€侊紝
    浣?bond 淇濇寔鍦?coupled control 鐘舵€併€?

downdelay

	鎸囧畾鍦ㄦ娴嬪埌閾捐矾鏁呴殰鍚庯紝绂佺敤鏌愪釜 slave 涔嬪墠绛夊緟鐨勬椂闂达紝鍗曚綅涓?
	姣銆傛閫夐」浠呭 miimon 閾捐矾鐩戞帶鏈夋晥銆俤owndelay 鐨勫€煎簲涓?miimon
	鍊肩殑鏁存暟鍊嶏紱鍚﹀垯灏嗚鍚戜笅鍙栨暣涓烘渶鎺ヨ繎鐨勬暣鏁板€嶃€傞粯璁ゅ€间负 0銆?

fail_over_mac

	鎸囧畾 active-backup 妯″紡鏄惁搴斿湪 enslavement 鏃跺皢鍏ㄩ儴 slave 璁剧疆涓?
	鐩稿悓鐨?MAC 鍦板潃锛堜紶缁熻涓猴級锛屾垨鑰呭湪鍚敤鏃舵牴鎹墍閫夌瓥鐣ュ bond 鐨?
	MAC 鍦板潃鎵ц鐗规畩澶勭悊銆?

	鍙兘鐨勫€煎涓嬶細

	none 鎴?0

		姝よ缃鐢?fail_over_mac锛屽苟浣?bonding 鍦?enslavement 鏃?
		灏?active-backup bond 鐨勬墍鏈?slave 璁剧疆涓虹浉鍚岀殑 MAC 鍦板潃銆?
		杩欐槸榛樿鍊笺€?

	active 鎴?1

		鈥渁ctive鈥?fail_over_mac 绛栫暐琛ㄧず bond 鐨?MAC 鍦板潃搴斿缁堜负
		褰撳墠 active slave 鐨?MAC 鍦板潃銆俿lave 鐨?MAC 鍦板潃涓嶄細琚敼鍙橈紱
		鐩稿弽锛宐ond 鐨?MAC 鍦板潃浼氬湪鏁呴殰鍒囨崲鏈熼棿鍙戠敓鍙樺寲銆?

		姝ょ瓥鐣ラ€傜敤浜庨偅浜涙案杩滀笉鑳芥洿鏀瑰叾 MAC 鍦板潃鐨勮澶囷紝鎴栬€呮嫆缁?
		鎺ユ敹浠ュ叾鑷韩婧?MAC 涓虹洰鐨勭殑鍏ョ珯骞挎挱鐨勮澶囷紙杩欎細骞叉壈 ARP 鐩戞帶锛夈€?

		姝ょ瓥鐣ョ殑缂虹偣鍦ㄤ簬锛岀綉缁滀笂鐨勬瘡涓澶囬兘蹇呴』閫氳繃鍏嶈垂 ARP 鏉ユ洿鏂帮紝
		鑰屼紶缁熸柟娉曢€氬父鍙渶鏇存柊涓€涓垨澶氫釜浜ゆ崲鏈猴紙濡傛灉浜ゆ崲鏈轰細渚﹀惉
		鍏ョ珯娴侀噺浠ユ洿鏂板叾琛ㄩ」锛屽垯杩欓€氬父瀵逛换浣曞叾浠栨祦閲忋€佽€屼笉浠呬粎鏄?ARP
		娴侀噺閮戒細鍙戠敓锛夈€傚鏋滃厤璐?ARP 涓㈠け锛岄€氫俊鍙兘浼氫腑鏂€?

		褰撴绛栫暐涓?mii 鐩戞帶閰嶅悎浣跨敤鏃讹紝閭ｄ簺鍦ㄧ湡姝ｈ兘澶熸敹鍙戜箣鍓嶅氨鏂█
		閾捐矾 up 鐨勮澶囩壒鍒鏄撲涪澶卞厤璐?ARP锛屽洜姝ゅ彲鑳介渶瑕佽缃悎閫傜殑
		updelay銆?

	follow 鎴?2

		鈥渇ollow鈥?fail_over_mac 绛栫暐浣?bond 鐨?MAC 鍦板潃浠ュ父瑙勬柟寮?
		閫夊嚭锛堥€氬父涓哄姞鍏?bond 鐨勭涓€涓?slave 鐨?MAC 鍦板潃锛夈€備絾鏄紝
		绗簩鍙婂悗缁殑 slave 鍦ㄥ浜?backup 瑙掕壊鏃朵笉浼氳璁剧疆涓鸿 MAC 鍦板潃锛?
		slave 浼氬湪鏁呴殰鍒囨崲鏃惰鍐欏叆 bond 鐨?MAC 鍦板潃锛堣€屽師鍏堢殑 active
		slave 鍒欐敹鍒版柊 active slave 鐨?MAC 鍦板潃锛夈€?

		姝ょ瓥鐣ラ€傜敤浜庨偅浜涘湪澶氫釜绔彛琚缃负鐩稿悓 MAC 鍦板潃鏃朵細鎰熷埌鍥版儜
		鎴栦骇鐢熸€ц兘涓嬮檷鐨勫绔彛璁惧銆?

	榛樿绛栫暐涓?none锛岄櫎闈炵涓€涓?slave 鏃犳硶鏇存敼鍏?MAC 鍦板潃锛屾鏃堕粯璁?
	閫夋嫨 active 绛栫暐銆?

	姝ら€夐」浠呭彲鍦?bond 涓病鏈?slave 鏃堕€氳繃 sysfs 淇敼銆?

	姝ら€夐」娣诲姞浜?bonding 鐗堟湰 3.2.0銆傚叾涓?鈥渇ollow鈥?绛栫暐娣诲姞浜?bonding
	鐗堟湰 3.3.0銆?

lacp_active
	鎸囧畾鏄惁瀹氭湡鍙戦€?LACPDU 甯х殑閫夐」銆?

	off 鎴?0
		LACPDU 甯х殑琛屼负绫讳技浜庘€滆闂墠璇粹€濓紙speak when spoken to锛夈€?

	on 鎴?1
		LACPDU 甯т細娌跨潃閰嶇疆濂界殑閾捐矾瀹氭湡鍙戦€併€傛洿澶氱粏鑺傚弬瑙?lacp_rate銆?

	榛樿涓?on銆?

lacp_rate

	鎸囧畾鍦?802.3ad 妯″紡涓嬫垜浠姹傞摼璺绔彂閫?LACPDU 鏁版嵁鍖呯殑閫熺巼鐨勯€夐」銆?
	鍙兘鐨勫€煎涓嬶細

	slow 鎴?0
		璇锋眰瀵圭姣?30 绉掑彂閫佷竴娆?LACPDU

	fast 鎴?1
		璇锋眰瀵圭姣?1 绉掑彂閫佷竴娆?LACPDU

	榛樿涓?slow銆?

broadcast_neighbor

	鎸囧畾鏄惁鍚戞墍鏈?active slave 骞挎挱 ARP/ND 鏁版嵁鍖呯殑閫夐」銆傛閫夐」鍦?
	802.3ad 妯″紡涔嬪鐨勫叾浠栨ā寮忎腑涓嶈捣浣滅敤銆傞粯璁や负 off锛?锛夈€?

max_bonds

	鎸囧畾涓烘 bonding 椹卞姩瀹炰緥鍒涘缓澶氬皯涓?bonding 璁惧銆備緥濡傦紝濡傛灉
	max_bonds 涓?3锛屼笖 bonding 椹卞姩灏氭湭鍔犺浇锛屽垯灏嗗垱寤?bond0銆乥ond1
	鍜?bond2銆傞粯璁ゅ€间负 1銆傛寚瀹氬€间负 0 浼氬姞杞?bonding锛屼絾涓嶄細鍒涘缓浠讳綍
	璁惧銆?

miimon

	鎸囧畾 MII 閾捐矾鐩戞帶鐨勯鐜囷紝鍗曚綅涓烘绉掋€傚畠鍐冲畾浜嗘瘡涓?slave 鐨勯摼璺?
	鐘舵€佽妫€鏌ヤ互鍙戠幇閾捐矾鏁呴殰鐨勯鐜囥€傚彇鍊间负 0 浼氱鐢?MII 閾捐矾鐩戞帶銆?
	鍙栧€间负 100 鏄竴涓笉閿欑殑璧峰鍊笺€?

	濡傛灉鏈缃?arp_interval锛屽垯榛樿鍊间负 100銆?

min_links

	鎸囧畾鍦ㄦ柇瑷€ carrier 涔嬪墠蹇呴』澶勪簬 active 鐘舵€佺殑鏈€灏忛摼璺暟銆傚畠绫讳技浜?
	Cisco EtherChannel 鐨?min-links 鐗规€с€傝繖鍏佽璁剧疆鍦ㄥ皢 bond 璁惧
	鏍囪涓?up锛坈arrier on锛変箣鍓嶅繀椤诲浜?up锛堥摼璺?up 鐘舵€侊級鐘舵€佺殑鎴愬憳
	绔彛鐨勬渶灏忔暟閲忋€傝繖瀵逛簬闆嗙兢绛夐珮灞傛湇鍔″笇鏈涘湪鍒囨崲涔嬪墠纭繚鏈夋渶灏戞暟閲?
	鐨勪綆甯﹀閾捐矾澶勪簬娲诲姩鐘舵€佺殑鎯呭喌寰堟湁鐢ㄣ€傛閫夐」浠呭奖鍝?802.3ad 妯″紡銆?

	榛樿鍊间负 0銆傝繖灏嗗湪 802.3ad 妯″紡涓嬶紝鍙瀛樺湪娲诲姩鑱氬悎鍣紝鏃犺鍏朵腑
	鍙敤閾捐矾鏁伴噺澶氬皯锛岄兘浼氬鑷?carrier 琚柇瑷€銆傛敞鎰忥紝鐢变簬鑱氬悎鍣ㄥ湪娌℃湁
	鑷冲皯涓€涓彲鐢ㄩ摼璺殑鎯呭喌涓嬩笉鍙兘澶勪簬娲诲姩鐘舵€侊紝鍥犳灏嗘閫夐」璁句负 0
	鎴?1 鐨勬晥鏋滃畬鍏ㄧ浉鍚屻€?

mode

	鎸囧畾 bonding 绛栫暐涔嬩竴銆傞粯璁ゅ€间负 balance-rr锛坮ound robin锛夈€傚彲鑳界殑鍊?
	濡備笅锛?

	balance-rr 鎴?0

		杞绛栫暐锛氭寜椤哄簭浠庣涓€涓彲鐢?slave 鍒版渶鍚庝竴涓?slave 渚濇
		鍙戦€佹暟鎹寘銆傛妯″紡鎻愪緵璐熻浇鍧囪　涓庡閿欒兘鍔涖€?

	active-backup 鎴?1

		active-backup 绛栫暐锛歜ond 涓彧鏈変竴涓?slave 澶勪簬娲诲姩鐘舵€併€?
		褰撲笖浠呭綋 active slave 澶辫触鏃讹紝鍙︿竴涓?slave 鎵嶄細鍙樹负娲诲姩鐘舵€併€?
		bond 鐨?MAC 鍦板潃鍙湪鍚屼竴涓鍙ｏ紙缃戠粶閫傞厤鍣級涓婂澶栧彲瑙侊紝
		浠ラ伩鍏嶈浜ゆ崲鏈烘劅鍒板洶鎯戙€?

		鍦?bonding 鐗堟湰 2.6.2 鎴栨洿楂樼増鏈腑锛屽綋 active-backup 妯″紡鍙戠敓
		鏁呴殰鍒囨崲鏃讹紝bonding 浼氬湪鏂扮殑 active slave 涓婂彂鍑轰竴涓垨澶氫釜
		鍏嶈垂 ARP銆備細涓?bonding master 鎺ュ彛鍙婂叾涓婇厤缃殑姣忎釜 VLAN 鎺ュ彛
		鍚勫彂鍑轰竴涓厤璐?ARP锛屽墠鎻愭槸杩欎簺鎺ュ彛鑷冲皯閰嶇疆浜嗕竴涓?IP 鍦板潃銆?
		涓?VLAN 鎺ュ彛鍙戝嚭鐨勫厤璐?ARP 浼氬甫鏈夌浉搴旂殑 VLAN id 鏍囪銆?

		姝ゆā寮忔彁渚涘閿欒兘鍔涖€備笅闈㈡枃妗ｅ寲鐨?primary 閫夐」浼氬奖鍝嶆妯″紡鐨?
		琛屼负銆?

	balance-xor 鎴?2

		XOR 绛栫暐锛氬熀浜庢墍閫夌殑鍙戦€佸搱甯岀瓥鐣ュ彂閫併€傞粯璁ょ瓥鐣ヤ负涓€涓畝鍗曠殑
		[锛堟簮 MAC 鍦板潃 XOR 鐩殑 MAC 鍦板潃 XOR 鏁版嵁鍖呯被鍨?ID锛夊彇妯?
		slave 鏁癩銆傚彲浠ラ€氳繃涓嬮潰鏂囨。鍖栫殑 xmit_hash_policy 閫夐」閫夋嫨
		鍏朵粬鍙戦€佺瓥鐣ャ€?

		姝ゆā寮忔彁渚涜礋杞藉潎琛′笌瀹归敊鑳藉姏銆?

	broadcast 鎴?3

		骞挎挱绛栫暐锛氬湪鎵€鏈?slave 鎺ュ彛涓婂彂閫佷竴鍒囧唴瀹广€傛妯″紡鎻愪緵瀹归敊鑳藉姏銆?

	802.3ad 鎴?4

		IEEE 802.3ad 鍔ㄦ€侀摼璺仛鍚堛€傚垱寤哄叡浜浉鍚岄€熺巼涓庡弻宸ヨ缃殑鑱氬悎缁勩€?
		鏍规嵁 802.3ad 瑙勮寖锛屽皢鎵€鏈変綅浜庢椿鍔ㄨ仛鍚堝櫒涓殑 slave 閮藉埄鐢ㄨ捣鏉ャ€?

		鍑虹珯娴侀噺鐨?slave 閫夋嫨鏍规嵁鍙戦€佸搱甯岀瓥鐣ヨ繘琛岋紝璇ョ瓥鐣ュ彲浠ラ€氳繃涓嬮潰
		鏂囨。鍖栫殑 xmit_hash_policy 閫夐」浠庨粯璁ょ殑绠€鍗?XOR 绛栫暐鏇存敼銆?
		娉ㄦ剰锛屽苟闈炴墍鏈夊彂閫佺瓥鐣ラ兘绗﹀悎 802.3ad锛岀壒鍒槸鍦?802.3ad 鏍囧噯
		绗?43.2.4 鑺傚叧浜庢暟鎹寘涔卞簭鐨勮姹傛柟闈€備笉鍚岀殑瀵圭瀹炵幇瀵逛笉鍚堣
		琛屼负鐨勫蹇嶅害鍚勪笉鐩稿悓銆?

		鍏堝喅鏉′欢锛?

  1. 鍩虹椹卞姩涓敮鎸?Ethtool锛岃兘澶熻幏鍙栨瘡涓?slave 鐨勯€熺巼涓庡弻宸ャ€?

  2. 鏀寔 IEEE 802.3ad 鍔ㄦ€侀摼璺仛鍚堢殑浜ゆ崲鏈恒€?

		澶у鏁颁氦鎹㈡満闇€瑕佹煇绉嶉厤缃墠鑳藉惎鐢?802.3ad 妯″紡銆?

	balance-tlb 鎴?5

		鑷€傚簲鍙戦€佽礋杞藉潎琛★細涓嶉渶瑕佷换浣曠壒娈婁氦鎹㈡満鏀寔鐨勪俊閬撶粦瀹氥€?

		鍦?tlb_dynamic_lb=1 妯″紡涓嬶紝鍑虹珯娴侀噺鏍规嵁姣忎釜 slave 涓婄殑褰撳墠璐熻浇
		锛堢浉瀵逛簬閫熺巼璁＄畻锛夎繘琛屽垎鍙戙€?

		鍦?tlb_dynamic_lb=0 妯″紡涓嬶紝鍩轰簬褰撳墠璐熻浇鐨勮礋杞藉潎琛¤绂佺敤锛屾祦閲?
		浠呬娇鐢ㄥ搱甯屽垎鍙戙€?

		鍏ョ珯娴侀噺鐢卞綋鍓?slave 鎺ユ敹銆傚鏋滄帴鏀?slave 澶辫触锛屽彟涓€涓?slave 浼?
		鎺ョ澶辫触鎺ユ敹 slave 鐨?MAC 鍦板潃銆?

		鍏堝喅鏉′欢锛?

		鍩虹椹卞姩涓敮鎸?Ethtool锛岃兘澶熻幏鍙栨瘡涓?slave 鐨勯€熺巼銆?

	balance-alb 鎴?6

		鑷€傚簲璐熻浇鍧囪　锛氬寘鍚?balance-tlb锛屽鍔犻拡瀵?IPv4 娴侀噺鐨勬帴鏀惰礋杞?
		鍧囪　锛坮lb锛夛紝骞朵笖涓嶉渶瑕佷换浣曠壒娈婁氦鎹㈡満鏀寔銆傛帴鏀惰礋杞藉潎琛￠€氳繃
		ARP 鍗忓晢瀹炵幇銆俠onding 椹卞姩鎷︽埅鏈湴绯荤粺鍙戝嚭閫斾腑鐨?ARP 搴旂瓟锛屽苟灏?
		婧愮‖浠跺湴鍧€鏀瑰啓涓?bond 涓煇涓?slave 鐨勫敮涓€纭欢鍦板潃锛屼娇寰椾笉鍚岀殑
		瀵圭瓑浣撲娇鐢ㄤ笉鍚岀殑纭欢鍦板潃鏉ヨ闂湇鍔″櫒銆?

		鐢辨湇鍔″櫒鍒涘缓鐨勮繛鎺ョ殑鎺ユ敹娴侀噺涔熶細琚潎琛°€傚綋鏈湴绯荤粺鍙戦€?ARP
		璇锋眰鏃讹紝bonding 椹卞姩浼氫粠 ARP 鏁版嵁鍖呬腑澶嶅埗骞朵繚瀛樺绛変綋鐨?IP
		淇℃伅銆傚綋鏉ヨ嚜瀵圭瓑浣撶殑 ARP 搴旂瓟鍒拌揪鏃讹紝鍏剁‖浠跺湴鍧€琚彇鍑猴紝bonding
		椹卞姩鍚戣瀵圭瓑浣撳彂璧蜂竴涓?ARP 搴旂瓟锛屽皢鍏跺垎閰嶇粰 bond 涓殑鏌愪釜 slave銆?
		浣跨敤 ARP 鍗忓晢杩涜鍧囪　鐨勪竴涓棶棰樻槸锛屾瘡娆″箍鎾?ARP 璇锋眰鏃堕兘浼氫娇鐢?
		bond 鐨勭‖浠跺湴鍧€銆傚洜姝わ紝瀵圭瓑浣撳涔犲埌鐨勬槸 bond 鐨勭‖浠跺湴鍧€锛屾帴鏀?
		娴侀噺鐨勫潎琛″氨鍧嶇缉鍒板綋鍓?slave 涓娿€傝繖閫氳繃鍚戞墍鏈夊绛変綋鍙戦€佹洿鏂?
		锛圓RP 搴旂瓟锛屽甫鏈夊悇鑷垎閰嶇殑纭欢鍦板潃锛夋潵瑙ｅ喅锛屼粠鑰屼娇娴侀噺琚噸鏂?
		鍒嗗彂銆傚綋鏂扮殑 slave 鍔犲叆 bond锛屾垨鏌愪釜 inactive slave 琚噸鏂版縺娲绘椂锛?
		鎺ユ敹娴侀噺涔熶細閲嶆柊鍒嗗彂銆傛帴鏀惰礋杞藉湪 bond 涓渶楂橀€熺巼鐨勪竴缁?slave 闂?
		椤哄簭锛堣疆璇級鍒嗗彂銆?

		褰撻摼璺噸鏂拌繛鎺ユ垨鏈夋柊鐨?slave 鍔犲叆 bond 鏃讹紝鎺ユ敹娴侀噺浼氶€氳繃鍚戞瘡涓?
		瀹㈡埛绔彂璧峰甫鏈夋墍閫?MAC 鍦板潃鐨?ARP 搴旂瓟锛屽湪 bond 涓殑鎵€鏈?active
		slave 闂撮噸鏂板垎鍙戙€倁pdelay 鍙傛暟锛堣瑙佷笅鏂囷級蹇呴』璁剧疆涓哄ぇ浜庢垨绛変簬
		浜ゆ崲鏈虹殑杞彂寤惰繜鐨勫€硷紝浠ヤ究鍙戠粰瀵圭瓑浣撶殑 ARP 搴旂瓟涓嶄細琚氦鎹㈡満
		闃诲銆?

		鍏堝喅鏉′欢锛?

  1. 鍩虹椹卞姩涓敮鎸?Ethtool锛岃兘澶熻幏鍙栨瘡涓?slave 鐨勯€熺巼銆?

  2. 鍩虹椹卞姩鏀寔鍦ㄨ澶囧浜庢墦寮€鐘舵€佹椂璁剧疆鍏剁‖浠跺湴鍧€銆傝繖鏄繀闇€鐨勶紝
		浠ヤ繚璇佸缁堟湁涓€涓?slave 浣跨敤 bond 纭欢鍦板潃锛坈urr_active_slave锛夛紝
		鍚屾椂涓?bond 涓殑姣忎釜 slave 鎻愪緵鍞竴鐨勭‖浠跺湴鍧€銆傚鏋?curr_active_slave
		澶辫触锛屽叾纭欢鍦板潃浼氫笌閫夊嚭鐨勬柊 curr_active_slave 浜ゆ崲銆?

num_grat_arp,
num_unsol_na

	鎸囧畾鍦ㄦ晠闅滃垏鎹簨浠跺悗鍙戝嚭鐨勫绛変綋閫氱煡锛堝厤璐?ARP 涓庝富鍔ㄥ彂鍑虹殑 IPv6
	閭诲眳閫氬憡锛夌殑鏁伴噺銆備竴鏃︽柊 slave 涓婄殑閾捐矾 up锛堝彲鑳界珛鍗筹級锛屽氨浼氬湪
	bonding 璁惧鍙婂叾姣忎釜 VLAN 瀛愯澶囦笂鍙戦€佷竴涓绛変綋閫氱煡銆傚鏋滄暟閲?
	澶т簬 1锛屽垯鎸?peer_notif_delay 鎸囧畾鐨勯€熺巼閲嶅鍙戦€併€?

	鏈夋晥鑼冨洿鏄?0 - 255锛涢粯璁ゅ€间负 1銆傝繖浜涢€夐」褰卞搷 active-backup 鎴?
	802.3ad锛堝惎鐢?broadcast_neighbor 鏃讹級妯″紡銆傝繖浜涢€夐」鍒嗗埆娣诲姞浜?
	bonding 鐗堟湰 3.3.0 涓?3.4.0銆?

	浠?Linux 3.0 涓?bonding 鐗堟湰 3.7.1 璧凤紝杩欎簺閫氱煡鐢?ipv4 涓?ipv6 浠ｇ爜
	鐢熸垚锛岄噸澶嶆鏁版棤娉曠嫭绔嬭缃€?

packets_per_slave

	鎸囧畾鍦ㄥ垏鎹㈠埌涓嬩竴涓?slave 涔嬪墠锛岄€氳繃涓€涓?slave 鍙戦€佺殑鏁版嵁鍖呮暟閲忋€?
	褰撹涓?0 鏃讹紝鍒欓殢鏈洪€夋嫨涓€涓?slave銆?

	鏈夋晥鑼冨洿鏄?0 - 65535锛涢粯璁ゅ€间负 1銆傛閫夐」浠呭湪 balance-rr 妯″紡涓嬬敓鏁堛€?

peer_notif_delay

	鎸囧畾鍦ㄦ晠闅滃垏鎹簨浠跺悗鍙戝嚭鐨勬瘡涓绛変綋閫氱煡锛堝厤璐?ARP 涓庝富鍔ㄥ彂鍑虹殑
	IPv6 閭诲眳閫氬憡锛変箣闂寸殑寤惰繜锛屽崟浣嶄负姣銆傛寤惰繜搴斾负 MII 閾捐矾鐩戞帶
	闂撮殧锛坢iimon锛夌殑鏁存暟鍊嶃€?

	鏈夋晥鑼冨洿鏄?0 - 300000銆傞粯璁ゅ€间负 0锛岃〃绀轰笌 MII 閾捐矾鐩戞帶闂撮殧鐨勫€间竴鑷淬€?

prio
	slave 浼樺厛绾с€傛暟鍊艰秺澶ц〃绀轰紭鍏堢骇瓒婇珮銆俻rimary slave 鍏锋湁鏈€楂樹紭鍏堢骇銆?
	姝ら€夐」涔熼伒寰?primary_reselect 瑙勫垯銆?

	姝ら€夐」鍙兘閫氳繃 netlink 閰嶇疆锛屽苟涓斾粎瀵?active-backup(1)銆乥alance-tlb (5)
	鍜?balance-alb (6) 妯″紡鏈夋晥銆傛湁鏁堝€艰寖鍥翠负鏈夌鍙?32 浣嶆暣鏁般€?

	榛樿鍊间负 0銆?

primary

	涓€涓瓧绗︿覆锛坋th0銆乪th2 绛夛級锛屾寚瀹氬摢涓?slave 鏄?primary 璁惧銆傚彧瑕佽
	璁惧鍙敤锛屽畠灏卞缁堟槸 active slave銆備粎褰?primary 绂荤嚎鏃舵墠浼氫娇鐢ㄥ鐢?
	璁惧銆傚綋鏌愪釜 slave 浼樹簬鍙︿竴涓?slave 鏃讹紙渚嬪锛屾煇涓?slave 鐨勫悶鍚愰噺
	楂樹簬鍙︿竴涓級锛岃繖寰堟湁鐢ㄣ€?

	primary 閫夐」浠呭 active-backup(1)銆乥alance-tlb (5) 鍜?balance-alb (6)
	妯″紡鏈夋晥銆?

primary_reselect

	鎸囧畾 primary slave 鐨勯噸鏂伴€夋嫨绛栫暐銆傚畠褰卞搷鍦?active slave 澶辫触鎴?
	primary slave 鎭㈠鏃讹紝濡備綍閫夋嫨 primary slave 鎴愪负 active slave銆傛閫夐」
	鏃ㄥ湪闃叉 primary slave 涓庡叾浠?slave 涔嬮棿鍙嶅妯烦銆傚彲鑳界殑鍊煎涓嬶細

	always 鎴?0锛堥粯璁わ級

		primary slave 涓€鏃︽仮澶嶅氨绔嬪嵆鎴愪负 active slave銆?

	better 鎴?1

		primary slave 鍦ㄥ叾鎭㈠涓旈€熺巼涓庡弻宸ヤ紭浜庡綋鍓?active slave 鏃讹紝
		鎴愪负 active slave銆?

	failure 鎴?2

		primary slave 浠呭湪褰撳墠 active slave 澶辫触涓?primary slave 澶勪簬 up
		鐘舵€佹椂锛屾墠鎴愪负 active slave銆?

	鍦ㄤ袱绉嶆儏鍐典笅浼氬拷鐣?primary_reselect 璁剧疆锛?

		濡傛灉娌℃湁 slave 澶勪簬娲诲姩鐘舵€侊紝鍒欑涓€涓仮澶嶇殑 slave 浼氳璁句负
		active slave銆?

		鍦ㄥ垵娆¤ enslaved 鏃讹紝primary slave 鎬绘槸琚涓?active slave銆?

	閫氳繃 sysfs 鏇存敼 primary_reselect 绛栫暐浼氬鑷存牴鎹柊绛栫暐绔嬪嵆閫夋嫨鏈€浣崇殑
	active slave銆傛牴鎹叿浣撴儏鍐碉紝杩欏彲鑳戒細涔熷彲鑳戒笉浼氬鑷?active slave 鍙戠敓鍙樺寲銆?

	姝ら€夐」娣诲姞浜?bonding 鐗堟湰 3.6.0銆?

tlb_dynamic_lb

	鎸囧畾鍦?tlb 鎴?alb 妯″紡涓嬫槸鍚﹀惎鐢ㄦ祦鐨勫姩鎬侀噸鎺掋€傝鍊煎鍏朵粬浠讳綍妯″紡
	閮戒笉璧蜂綔鐢ㄣ€?

	tlb 妯″紡鐨勯粯璁よ涓烘槸鍦ㄨ闂撮殧鍐呭熀浜庤礋杞借法 slave 閲嶆帓娲诲姩娴併€傝繖甯︽潵浜?
	涓嶉敊鐨?lb 鐗规€э紝浣嗗彲鑳藉鑷存暟鎹寘閲嶆帓搴忋€傚鏋滈噸鎺掑簭鏄釜闂锛屽彲浣跨敤
	姝ゅ彉閲忕鐢ㄦ祦閲嶆帓锛屼粎渚濊禆鍝堝笇鍒嗗彂鎻愪緵鐨勮礋杞藉潎琛°€倄mit-hash-policy
	鍙敤浜庝负璁剧疆閫夋嫨鍚堥€傜殑鍝堝笇銆?

	sysfs 椤瑰彲鐢ㄤ簬鎸?bond 璁惧鏇存敼姝よ缃紝鍏跺垵濮嬪€煎彇鑷ā鍧楀弬鏁般€俿ysfs
	椤逛粎鍏佽鍦?bond 璁惧澶勪簬 down 鐘舵€佹椂鏇存敼銆?

	榛樿鍊间负 "1"锛屽嵆鍚敤娴侀噸鎺掞紱鍊间负 "0" 鏃剁鐢ㄥ畠銆傛閫夐」娣诲姞浜?bonding
	椹卞姩 3.7.1 鐗堟湰銆?

updelay

	鎸囧畾鍦ㄦ娴嬪埌閾捐矾鎭㈠鍚庯紝鍚敤鏌愪釜 slave 涔嬪墠绛夊緟鐨勬椂闂达紝鍗曚綅涓烘绉掋€?
	姝ら€夐」浠呭 miimon 閾捐矾鐩戞帶鏈夋晥銆倁pdelay 鐨勫€煎簲涓?miimon 鍊肩殑鏁存暟鍊嶏紱
	鍚﹀垯灏嗚鍚戜笅鍙栨暣涓烘渶鎺ヨ繎鐨勬暣鏁板€嶃€傞粯璁ゅ€间负 0銆?

use_carrier

	涓€涓繃鏃剁殑閫夐」锛屼互鍓嶇敤浜庡湪 MII / ETHTOOL ioctl 涓?netif_carrier_ok()
	涔嬮棿閫夋嫨浠ュ垽鏂摼璺姸鎬併€?

	鐜板湪鎵€鏈夐摼璺姸鎬佹鏌ラ兘閫氳繃 netif_carrier_ok() 瀹屾垚銆?

	涓轰簡鍚戝悗鍏煎锛屾閫夐」鐨勫€煎彲浠ヨ妫€鏌ユ垨璁剧疆銆傚敮涓€鏈夋晥鐨勮缃槸 1銆?

xmit_hash_policy

	閫夋嫨鐢ㄤ簬 balance-xor銆?02.3ad 鍜?tlb 妯″紡涓?slave 閫夋嫨鐨勫彂閫佸搱甯岀瓥鐣ャ€?
	鍙兘鐨勫€煎涓嬶細

	layer2

		浣跨敤纭欢 MAC 鍦板潃涓庢暟鎹寘绫诲瀷 ID 瀛楁鐨?XOR 鏉ョ敓鎴愬搱甯屻€傚叕寮忎负

		hash = 婧?MAC[^5^] XOR 鐩殑 MAC[^5^] XOR 鏁版嵁鍖呯被鍨?ID
		slave 缂栧彿 = hash 鍙栨ā slave 鏁?

		姝ょ畻娉曚細灏嗗埌鏌愪釜鐗瑰畾缃戠粶瀵圭瓑浣撶殑鎵€鏈夋祦閲忔斁鍦ㄥ悓涓€涓?slave 涓娿€?

		姝ょ畻娉曠鍚?802.3ad銆?

	layer2+3

		姝ょ瓥鐣ョ粨鍚堜娇鐢?layer2 涓?layer3 鐨勫崗璁俊鎭潵鐢熸垚鍝堝笇銆?

		浣跨敤纭欢 MAC 鍦板潃涓?IP 鍦板潃鐨?XOR 鏉ョ敓鎴愬搱甯屻€傚叕寮忎负

		hash = 婧?MAC[^5^] XOR 鐩殑 MAC[^5^] XOR 鏁版嵁鍖呯被鍨?ID
		hash = hash XOR 婧?IP XOR 鐩殑 IP
		hash = hash XOR (hash RSHIFT 16)
		hash = hash XOR (hash RSHIFT 8)
		鐒跺悗 hash 鍙栨ā slave 鏁般€?

		濡傛灉鍗忚涓?IPv6锛屽垯婧愬湴鍧€涓庣洰鐨勫湴鍧€棣栧厛浣跨敤 ipv6_addr_hash 杩涜
		鍝堝笇銆?

		姝ょ畻娉曚細灏嗗埌鏌愪釜鐗瑰畾缃戠粶瀵圭瓑浣撶殑鎵€鏈夋祦閲忔斁鍦ㄥ悓涓€涓?slave 涓娿€?
		瀵逛簬闈?IP 娴侀噺锛屽叕寮忎笌 layer2 鍙戦€佸搱甯岀瓥鐣ョ浉鍚屻€?

		姝ょ瓥鐣ユ棬鍦ㄦ彁渚涙瘮鍗曠嫭 layer2 鏇村潎琛＄殑娴侀噺鍒嗗竷锛屽挨鍏舵槸鍦ㄩ渶瑕?
		閫氳繃 layer3 缃戝叧璁惧鎵嶈兘鍒拌揪澶у鏁扮洰鐨勫湴鐨勭幆澧冧腑銆?

		姝ょ畻娉曠鍚?802.3ad銆?

	layer3+4

		姝ょ瓥鐣ュ湪鍙敤鏃朵娇鐢ㄤ笂灞傚崗璁俊鎭潵鐢熸垚鍝堝笇銆傝繖鍏佽鍒版煇涓壒瀹?
		缃戠粶瀵圭瓑浣撶殑娴侀噺璺ㄥ涓?slave锛屽敖绠″崟涓繛鎺ヤ笉浼氳法澶氫釜 slave銆?

		瀵逛簬鏈垎鐗囩殑 TCP 涓?UDP 鏁版嵁鍖咃紝鍏紡涓?

		hash = 婧愮鍙? 鐩殑绔彛锛堝澶撮儴鎵€绀猴級
		hash = hash XOR 婧?IP XOR 鐩殑 IP
		hash = hash XOR (hash RSHIFT 16)
		hash = hash XOR (hash RSHIFT 8)
		hash = hash RSHIFT 1
		鐒跺悗 hash 鍙栨ā slave 鏁般€?

		濡傛灉鍗忚涓?IPv6锛屽垯婧愬湴鍧€涓庣洰鐨勫湴鍧€棣栧厛浣跨敤 ipv6_addr_hash 杩涜
		鍝堝笇銆?

		瀵逛簬鍒嗙墖鐨?TCP 鎴?UDP 鏁版嵁鍖咃紝浠ュ強鎵€鏈夊叾浠?IPv4 涓?IPv6 鍗忚娴侀噺锛?
		鐪佺暐婧愮鍙ｄ笌鐩殑绔彛淇℃伅銆傚浜庨潪 IP 娴侀噺锛屽叕寮忎笌 layer2 鍙戦€佸搱甯?
		绛栫暐鐩稿悓銆?

		姝ょ畻娉曞苟涓嶅畬鍏ㄧ鍚?802.3ad銆備竴涓悓鏃跺寘鍚垎鐗囦笌鏈垎鐗囨暟鎹寘鐨?
		鍗曚釜 TCP 鎴?UDP 浼氳瘽锛屼細鐪嬪埌鏁版嵁鍖呰鏉″甫鍖栧埌涓や釜鎺ュ彛涓娿€傝繖鍙兘瀵艰嚧
		涔卞簭浜や粯銆傚ぇ澶氭暟娴侀噺绫诲瀷涓嶄細婊¤冻姝ゆ潯浠讹紝鍥犱负 TCP 寰堝皯瀵规祦閲忓垎鐗囷紝
		涓斿ぇ澶氭暟 UDP 娴侀噺涓嶆秹鍙婇暱鏃堕棿鐨勪細璇濄€傚叾浠?802.3ad 瀹炵幇鍙兘瀹瑰繊涔熷彲鑳?
		涓嶅蹇嶆涓嶅悎瑙勮涓恒€?

	encap2+3

		姝ょ瓥鐣ヤ娇鐢ㄤ笌 layer2+3 鐩稿悓鐨勫叕寮忥紝浣嗗畠渚濊禆 skb_flow_dissect 鏉ヨ幏鍙?
		澶撮儴瀛楁锛屽湪浣跨敤灏佽鍗忚鏃跺彲鑳戒細浣跨敤鍐呭眰澶撮儴銆備緥濡傦紝杩欏皢鎻愬崌闅ч亾
		鐢ㄦ埛鐨勬€ц兘锛屽洜涓烘暟鎹寘浼氭牴鎹皝瑁呭悗鐨勬祦杩涜鍒嗗彂銆?

	encap3+4

		姝ょ瓥鐣ヤ娇鐢ㄤ笌 layer3+4 鐩稿悓鐨勫叕寮忥紝浣嗗畠渚濊禆 skb_flow_dissect 鏉ヨ幏鍙?
		澶撮儴瀛楁锛屽湪浣跨敤灏佽鍗忚鏃跺彲鑳戒細浣跨敤鍐呭眰澶撮儴銆備緥濡傦紝杩欏皢鎻愬崌闅ч亾
		鐢ㄦ埛鐨勬€ц兘锛屽洜涓烘暟鎹寘浼氭牴鎹皝瑁呭悗鐨勬祦杩涜鍒嗗彂銆?

	vlan+srcmac

		姝ょ瓥鐣ヤ娇鐢ㄩ潪甯稿熀纭€鐨?vlan ID 涓庢簮 mac 鍝堝笇锛屾寜 vlan 杩涜璐熻浇鍧囪　锛?
		骞跺湪鏌愭潯閾捐矾澶辫触鏃舵彁渚涙晠闅滃垏鎹€傞鏈熺殑鐢ㄤ緥鏄緵澶氫釜铏氭嫙鏈哄叡浜殑
		bond 浣跨敤锛岃繖浜涜櫄鎷熸満閮介厤缃负浣跨敤鑷繁鐨?vlan锛屼互鍦ㄦ病鏈?lacp 鑳藉姏鐨?
		浜ゆ崲纭欢鐨勬儏鍐典笅鎻愪緵绫讳技 lacp 鐨勫姛鑳姐€?

	鍝堝笇鍏紡寰堢畝鍗曪細

		hash = (vlan ID) XOR (婧?MAC 鍘傚晢) XOR (婧?MAC 璁惧)

	榛樿鍊间负 layer2銆傛閫夐」娣诲姞浜?bonding 鐗堟湰 2.6.3銆傚湪鏇存棭鐨?bonding 鐗堟湰涓紝
	姝ゅ弬鏁颁笉瀛樺湪锛宭ayer2 鏄敮涓€绛栫暐銆俵ayer2+3 鍊兼坊鍔犱簬 bonding 鐗堟湰 3.2.2銆?

resend_igmp

	鎸囧畾鍦ㄦ晠闅滃垏鎹簨浠跺悗鍙戝嚭鐨?IGMP 鎴愬憳鎶ュ憡鐨勬暟閲忋€傛晠闅滃垏鎹㈠悗绔嬪嵆鍙戝嚭
	涓€浠芥垚鍛樻姤鍛婏紝鍚庣画鏁版嵁鍖呭湪姣忎釜 200ms 闂撮殧鍙戦€併€?

	鏈夋晥鑼冨洿鏄?0 - 255锛涢粯璁ゅ€间负 1銆傚€间负 0 鏃堕樆姝㈠洜鏁呴殰鍒囨崲浜嬩欢鑰屽彂鍑?
	IGMP 鎴愬憳鎶ュ憡銆?

	姝ら€夐」瀵?balance-rr (0)銆乤ctive-backup (1)銆乥alance-tlb (5) 鍜?
	balance-alb (6) 妯″紡寰堟湁鐢紝鍦ㄨ繖浜涙ā寮忎腑锛屾晠闅滃垏鎹㈠彲鑳藉皢 IGMP 娴侀噺浠?
	涓€涓?slave 鍒囨崲鍒板彟涓€涓€傚洜姝ら渶瑕佸彂鍑轰竴浠芥柊鐨?IGMP 鎶ュ憡锛屼互淇冧娇浜ゆ崲鏈?
	閫氳繃鏂伴€夊嚭鐨?slave 杞彂鍏ョ珯 IGMP 娴侀噺銆?

	姝ら€夐」娣诲姞浜?bonding 鐗堟湰 3.7.0銆?

lp_interval

	鎸囧畾 bonding 椹卞姩鍚戞瘡涓?slave 鐨勫绔氦鎹㈡満鍙戦€佸涔犳暟鎹寘鐨勯棿闅旂鏁般€?

	鏈夋晥鑼冨洿鏄?1 - 0x7fffffff锛涢粯璁ゅ€间负 1銆傛閫夐」浠呭湪 balance-tlb 涓?
	balance-alb 妯″紡涓嬬敓鏁堛€?

## 3. 閰嶇疆 Bonding 璁惧


浣犲彲浠ヤ娇鐢ㄥ彂琛岀増鐨勭綉缁滃垵濮嬪寲鑴氭湰锛屾垨鑰呮墜鍔ㄤ娇鐢?iproute2 鎴?
sysfs 鎺ュ彛鏉ラ厤缃?bonding銆傚彂琛岀増閫氬父浣跨敤涓変釜鍖呬箣涓€鏉ユ彁渚涚綉缁滃垵濮嬪寲
鑴氭湰锛歩nitscripts銆乻ysconfig 鎴?interfaces銆傝繖浜涘寘鐨勮緝鏂扮増鏈敮鎸?
bonding锛岃€岃緝鏃х増鏈笉鏀寔銆?

鎴戜滑灏嗛鍏堟弿杩伴拡瀵逛娇鐢?initscripts銆乻ysconfig 鍜?interfaces锛堝畬鍏ㄦ垨
閮ㄥ垎鏀寔 bonding锛夌殑鍙戣鐗堥厤缃?bonding 鐨勯€夐」锛岀劧鍚庢彁渚涘湪涓嶄緷璧栫綉缁?
鍒濆鍖栬剼鏈紙鍗宠緝鏃х増鏈殑 initscripts 鎴?sysconfig锛夌殑鎯呭喌涓嬪惎鐢?bonding
鐨勪俊鎭€?

濡傛灉浣犱笉纭畾浣犵殑鍙戣鐗堜娇鐢ㄧ殑鏄?sysconfig銆乮nitscripts 杩樻槸 interfaces锛?
鎴栬€呬笉鐭ラ亾瀹冩槸鍚﹀鏂帮紝涓嶇敤鎷呭績銆傚垽鏂繖涓€鐐圭浉褰撶洿鎺ャ€?

棣栧厛锛屾煡鎵?/etc/network 鐩綍涓嬪悕涓?interfaces 鐨勬枃浠躲€傚鏋?
浣犵殑绯荤粺涓瓨鍦ㄦ鏂囦欢锛屽垯浣犵殑绯荤粺浣跨敤 interfaces銆傚弬瑙佲€滀娇鐢?Interfaces
鏀寔杩涜閰嶇疆鈥濄€?

```

	$ rpm -qf /sbin/ifup

```

瀹冧細鍝嶅簲涓€琛屼互 "initscripts" 鎴?"sysconfig" 寮€澶淬€佸悗鎺ヤ竴浜涙暟瀛楃殑鏂囨湰銆?
杩欏氨鏄彁渚涗綘鐨勭綉缁滃垵濮嬪寲鑴氭湰鐨勫寘銆?

鎺ヤ笅鏉ワ紝瑕佸垽鏂綘鐨勫畨瑁呮槸鍚︽敮鎸?bonding锛?

```

    $ grep ifenslave /sbin/ifup

```

濡傛灉杩斿洖浠讳綍鍖归厤椤癸紝鍒欎綘鐨?initscripts 鎴?sysconfig 鏀寔 bonding銆?

### 3.1 浣跨敤 Sysconfig 鏀寔杩涜閰嶇疆


鏈妭閫傜敤浜庝娇鐢ㄥ甫 bonding 鏀寔鐨?sysconfig 鐗堟湰鐨勫彂琛岀増锛屼緥濡?
SuSE Linux Enterprise Server 9銆?

SuSE SLES 9 鐨勭綉缁滈厤缃郴缁熺‘瀹炴敮鎸?bonding锛屼絾鍦ㄦ挵鍐欐湰鏂囨椂锛孻aST
绯荤粺閰嶇疆鍓嶇骞舵湭鎻愪緵浠讳綍澶勭悊 bonding 璁惧鐨勬柟娉曘€備笉杩囷紝bonding 璁惧
鍙互鎵嬪姩绠＄悊锛屽涓嬫墍绀恒€?

棣栧厛锛屽鏋滃皻鏈厤缃紝璇烽厤缃?slave 璁惧銆傚湪 SLES 9 涓婏紝鏈€绠€鍗曠殑鏂规硶鏄?
杩愯 yast2 sysconfig 閰嶇疆宸ュ叿銆傜洰鏍囨槸涓烘瘡涓?slave 璁惧鍒涘缓涓€涓?
ifcfg-id 鏂囦欢銆傚畬鎴愭鎿嶄綔鏈€绠€鍗曠殑鏂瑰紡鏄皢璁惧閰嶇疆涓?DHCP锛堣繖鍙槸涓轰簡
鍒涘缓 ifcfg-id 鏂囦欢锛涘叧浜?DHCP 鐨勪竴浜涢棶棰樿涓嬫枃锛夈€傝

```

    ifcfg-id-xx:xx:xx:xx:xx:xx

```

鍏朵腑 "xx" 閮ㄥ垎灏嗚璁惧鐨勬案涔?MAC 鍦板潃涓殑鏁板瓧鏇挎崲銆?

涓€鏃﹀垱寤轰簡涓€缁?ifcfg-id-xx:xx:xx:xx:xx:xx 鏂囦欢锛屽氨闇€瑕佺紪杈戣繖浜?slave
璁惧锛堝叾 MAC 鍦板潃瀵瑰簲浜?slave 璁惧锛夌殑閰嶇疆鏂囦欢銆傚湪缂栬緫涔嬪墠锛屾枃浠跺皢鍖呭惈
澶氳锛岀湅璧锋潵

```

	BOOTPROTO='dhcp'
	STARTMODE='on'
	USERCTL='no'
	UNIQUE='XNzu.WeZGOGF+4wE'
	_nm_name='bus-pci-0001:61:01.0'

```
```
	BOOTPROTO='none'
	STARTMODE='off'

```
涓嶈鏇存敼 UNIQUE 鎴?_nm_name 琛屻€傚垹闄ゅ叾浠栨墍鏈夎锛圲SERCTL 绛夛級銆?

涓€鏃?ifcfg-id-xx:xx:xx:xx:xx:xx 鏂囦欢琚慨鏀瑰畬锛屽氨鍒颁簡涓?bonding 璁惧
鑷韩鍒涘缓閰嶇疆鏂囦欢鐨勬椂鍊欍€傝鏂囦欢鍛藉悕涓?ifcfg-bondX锛屽叾涓?X 鏄鍒涘缓鐨?
bonding 璁惧鐨勭紪鍙凤紝浠?0 寮€濮嬨€傜涓€涓繖鏍风殑鏂囦欢鏄?ifcfg-bond0锛岀浜屼釜鏄?
ifcfg-bond1锛屼緷姝ょ被鎺ㄣ€俿ysconfig 缃戠粶閰嶇疆绯荤粺鑳藉姝ｇ‘鍚姩 bonding 鐨勫涓?
瀹炰緥銆?

```

	BOOTPROTO="static"
	BROADCAST="10.0.2.255"
	IPADDR="10.0.2.10"
	NETMASK="255.255.0.0"
	NETWORK="10.0.2.0"
	REMOTE_IPADDR=""
	STARTMODE="onboot"
	BONDING_MASTER="yes"
	BONDING_MODULE_OPTS="mode=active-backup miimon=100"
	BONDING_SLAVE0="eth0"
	BONDING_SLAVE1="bus-pci-0000:06:08.1"

```
鐢ㄩ€傚悎浣犵綉缁滅殑鐩稿簲鍊兼浛鎹㈢ず渚嬩腑鐨?BROADCAST銆両PADDR銆丯ETMASK 鍜?NETWORK
鍙栧€笺€?

STARTMODE 鎸囧畾璁惧浣曟椂涓婄嚎銆傚彲鑳界殑鍊煎涓嬶細

	======== ======================================================
	onboot	 璁惧鍦ㄥ惎鍔ㄦ椂鍚姩銆傚鏋滀綘涓嶇‘瀹氾紝杩欏ぇ姒傚氨鏄?
		 浣犳兂瑕佺殑銆?

	manual	 璁惧浠呭湪鎵嬪姩璋冪敤 ifup 鏃跺惎鍔ㄣ€傚鏋滀綘鍑轰簬鏌愮
		 鍘熷洜涓嶅笇鏈涘畠浠湪寮€鏈烘椂鑷姩鍚姩锛宐onding 璁惧
		 鍙互杩欐牱閰嶇疆銆?

	hotplug  璁惧鐢辩儹鎻掓嫈浜嬩欢鍚姩銆傝繖瀵?bonding 璁惧鑰岃█涓嶆槸
		 涓€涓湁鏁堥€夋嫨銆?

	off 鎴? 璁惧鐨勯厤缃蹇界暐銆?
	ignore
	======== ======================================================

BONDING_MASTER='yes' 杩欎竴琛岃〃鏄庤璁惧鏄竴涓?bonding master 璁惧銆傚敮涓€
鏈夌敤鐨勫€兼槸 "yes"銆?

BONDING_MODULE_OPTS 鐨勫唴瀹逛細鎻愪緵缁欐璁惧鐨?bonding 妯″潡瀹炰緥銆傚湪姝ゆ寚瀹?
bonding 妯″紡銆侀摼璺洃鎺х瓑閫夐」銆備笉瑕佸寘鍚?max_bonds bonding 鍙傛暟锛涘鏋滀綘鏈?
澶氫釜 bonding 璁惧锛岃繖浼氭贩娣嗛厤缃郴缁熴€?

鏈€鍚庯紝涓烘瘡涓?slave 鎻愪緵涓€涓?BONDING_SLAVEn="slave device"銆傚叾涓?"n" 鏄竴涓?
閫掑鐨勫€硷紝姣忎釜 slave 瀵瑰簲涓€涓€?slave device" 鍙互鏄竴涓帴鍙ｅ悕绉帮紝渚嬪
"eth0"锛屼篃鍙互鏄綉缁滆澶囩殑璁惧鎻忚堪绗︺€傛帴鍙ｅ悕绉版洿瀹规槗鏌ユ壘锛屼絾 ethN 鍚嶇О鍦?
鍚姩鏃跺彲鑳戒細鍙樺寲锛屼緥濡傦紝搴忓垪涓潬鍓嶇殑鏌愪釜璁惧鍙戠敓浜嗘晠闅溿€傝澶囨弿杩扮
锛堜笂渚嬩腑鐨?bus-pci-0000:06:08.1锛夋寚瀹氱殑鏄墿鐞嗙綉缁滆澶囷紝闄ら潪璁惧鐨勬€荤嚎浣嶇疆
鍙戠敓鍙樺寲锛堜緥濡傚畠琚粠涓€鍧?PCI 鎻掓Ы绉诲埌鍙︿竴鍧楋級锛屽惁鍒欎笉浼氭敼鍙樸€備笂闈㈢殑渚嬪瓙涓?
婕旂ず鐩殑鍚勭敤浜嗕竴涓被鍨嬶紱澶у鏁伴厤缃細涓烘墍鏈?slave 璁惧缁熶竴閫夋嫨鍏朵腑涓€绉嶇被鍨嬨€?

褰撴墍鏈夐厤缃枃浠堕兘宸蹭慨鏀规垨鍒涘缓瀹屾垚鍚庯紝蹇呴』閲嶅惎缃戠粶鎵嶈兘浣块厤缃洿鏀圭敓鏁?

```

	# /etc/init.d/network restart

```

娉ㄦ剰锛岀綉缁滄帶鍒惰剼鏈紙/sbin/ifdown锛変細浣滀负缃戠粶鍏抽棴澶勭悊鐨勪竴閮ㄥ垎绉婚櫎 bonding
妯″潡锛屽洜姝わ紝渚嬪褰撴ā鍧楀弬鏁板彂鐢熷彉鍖栨椂锛屾病鏈夊繀瑕佹墜宸ョЩ闄よ妯″潡銆?

姝ゅ锛屽湪鎾板啓鏈枃鏃讹紝YaST/YaST2 涓嶄細绠＄悊 bonding 璁惧锛堝畠浠湪鍏剁綉缁滆澶?
鍒楄〃涓笉鏄剧ず bonding 鎺ュ彛锛夈€傝鏇存敼 bonding 閰嶇疆锛屽繀椤绘墜宸ョ紪杈戦厤缃枃浠躲€?

ifcfg 鏂囦欢鐨勫叾瀹冮€氱敤閫夐」涓庣粏鑺傝

```

	/etc/sysconfig/network/ifcfg.template

```

娉ㄦ剰锛岃妯℃澘骞舵湭璁板綍涓婇潰鎻忚堪鐨勫悇绉?`BONDING_*` 璁剧疆锛屼絾纭疄鎻忚堪浜嗚澶氬叾浠?
閫夐」銆?

### 3.1.1 鍦?Sysconfig 涓娇鐢?DHCP

鍦?sysconfig 涓嬶紝灏嗚澶囬厤缃负 BOOTPROTO='dhcp' 浼氬鑷村畠鍚?DHCP 鏌ヨ鍏?IP
鍦板潃淇℃伅銆傚湪鎾板啓鏈枃鏃讹紝杩欏 bonding 璁惧涓嶈捣浣滅敤锛涜剼鏈細灏濊瘯鍦ㄦ坊鍔犱换浣?
slave 璁惧涔嬪墠灏变粠 DHCP 鑾峰彇璁惧鍦板潃銆傛病鏈?active slave锛孌HCP 璇锋眰灏变笉浼氳
鍙戦€佸埌缃戠粶涓娿€?

### 3.1.2 鍦?Sysconfig 涓厤缃涓?Bond

sysconfig 缃戠粶鍒濆鍖栫郴缁熻兘澶熷鐞嗗涓?bonding 璁惧銆傚彧闇€瑕佷负姣忎釜 bonding
瀹炰緥鍑嗗涓€涓€傚綋閰嶇疆鐨?ifcfg-bondX 鏂囦欢锛堝涓婃墍杩帮級銆備笉瑕佸悜浠讳綍 bonding 瀹炰緥
鎸囧畾 "max_bonds" 鍙傛暟锛屽洜涓鸿繖浼氭贩娣?sysconfig銆傚鏋滀綘闇€瑕佸涓甫鏈夌浉鍚屽弬鏁扮殑
bonding 璁惧锛岃鍒涘缓澶氫釜 ifcfg-bondX 鏂囦欢銆?

鐢变簬 sysconfig 鑴氭湰鍦?ifcfg-bondX 鏂囦欢涓彁渚?bonding 妯″潡閫夐」锛屽洜姝ゆ病鏈夊繀瑕?
灏嗗畠浠坊鍔犲埌绯荤粺鐨?`/etc/modules.d/*.conf` 閰嶇疆鏂囦欢涓€?

### 3.2 浣跨敤 Initscripts 鏀寔杩涜閰嶇疆

鏈妭閫傜敤浜庝娇鐢ㄥ甫 bonding 鏀寔鐨勮緝鏂扮増鏈?initscripts 鐨勫彂琛岀増锛屼緥濡?Red Hat
Enterprise Linux 3 鎴栨洿楂樼増鏈€丗edora 绛夈€傚湪杩欎簺绯荤粺涓婏紝缃戠粶鍒濆鍖栬剼鏈簡瑙?
bonding锛屽苟涓斿彲浠ヨ閰嶇疆涓烘帶鍒?bonding 璁惧銆傛敞鎰忥紝杈冩棫鐗堟湰鐨?initscripts 鍖?
瀵?bonding 鐨勬敮鎸佺▼搴﹁緝浣庯紱鍦ㄩ€傜敤澶勪細鍔犱互璇存槑銆?

杩欎簺鍙戣鐗堜笉浼氳嚜鍔ㄥ姞杞界綉缁滈€傞厤鍣ㄩ┍鍔紝闄ら潪 ethX 璁惧閰嶇疆浜?IP 鍦板潃銆傜敱浜?
杩欎竴闄愬埗锛岀敤鎴峰繀椤讳负鎵€鏈夊皢鎴愪负 bondX 閾捐矾鎴愬憳鐨勭墿鐞嗛€傞厤鍣ㄦ墜宸ラ厤缃竴涓?
network-script 鏂囦欢銆俷etwork script 鏂囦欢浣嶄簬鐩綍锛?

/etc/sysconfig/network-scripts

鏂囦欢鍚嶅繀椤讳互 "ifcfg-eth" 涓哄墠缂€锛屽苟浠ラ€傞厤鍣ㄧ殑鐗╃悊閫傞厤鍣ㄧ紪鍙蜂綔涓哄悗缂€銆備緥濡傦紝
eth0 鐨勮剼鏈簲鍛藉悕涓?/etc/sysconfig/network-scripts/ifcfg-eth0銆?

```

	DEVICE=eth0
	USERCTL=no
	ONBOOT=yes
	MASTER=bond0
	SLAVE=yes
	BOOTPROTO=none

```
姣忎釜 ethX 璁惧鐨?DEVICE= 琛岄兘涓嶅悓锛屽苟涓斿繀椤讳笌鏂囦欢鍚嶇浉瀵瑰簲锛屽嵆 ifcfg-eth1
蹇呴』鍏锋湁 DEVICE=eth1 鐨勮澶囪銆侻ASTER= 琛岀殑璁剧疆涔熷彇鍐充簬涓轰綘鐨?bond 閫夋嫨鐨勬渶缁?
bonding 鎺ュ彛鍚嶇О銆備笌鍏朵粬缃戠粶璁惧涓€鏍凤紝瀹冧滑閫氬父浠?0 寮€濮嬶紝姣忎釜璁惧閫掑 1锛屽嵆
绗竴涓?bonding 瀹炰緥鏄?bond0锛岀浜屼釜鏄?bond1锛屼緷姝ょ被鎺ㄣ€?

鎺ヤ笅鏉ワ紝鍒涘缓涓€涓?bond 缃戠粶鑴氭湰銆傛鑴氭湰鐨勬枃浠跺悕灏嗘槸
/etc/sysconfig/network-scripts/ifcfg-bondX锛屽叾涓?X 鏄?bond 鐨勭紪鍙枫€傚浜?bond0锛?
璇ユ枃浠跺悕涓?"ifcfg-bond0"锛屽浜?bond1锛屾枃浠跺悕涓?"ifcfg-bond1"锛屼緷姝ょ被鎺ㄣ€傚湪璇?
鏂囦欢鍐咃紝

```

	DEVICE=bond0
	IPADDR=192.168.1.1
	NETMASK=255.255.255.0
	NETWORK=192.168.1.0
	BROADCAST=192.168.1.255
	ONBOOT=yes
	BOOTPROTO=none
	USERCTL=no

```
鍔″繀鏇存敼鐗瑰畾浜庣綉缁滅殑琛岋紙IPADDR銆丯ETMASK銆丯ETWORK 鍜?BROADCAST锛変互鍖归厤浣犵殑
缃戠粶閰嶇疆銆?

瀵逛簬杈冩柊鐗堟湰鐨?initscripts锛屼緥濡?Fedora 7锛堟垨鏇撮珮锛夊拰 Red Hat Enterprise Linux
5锛堟垨鏇撮珮锛夛紝鍙互鍦?ifcfg-bond0 涓寚瀹?bonding 閫夐」锛岃繖涓嶄粎鏄彲琛岀殑锛岃€屼笖

```

  BONDING_OPTS="mode=active-backup arp_interval=60 arp_ip_target=192.168.1.254"

```
灏嗕互鎸囧畾鐨勯€夐」閰嶇疆 bond銆侭ONDING_OPTS 涓寚瀹氱殑閫夐」涓?bonding 妯″潡鍙傛暟鐩稿悓锛?
闄や簡鍦ㄦ棭浜?8.57锛團edora 8锛夊拰 8.45.19锛圧ed Hat Enterprise Linux 5.2锛夌殑
initscripts 鐗堟湰涓?arp_ip_target 瀛楁鐨勬儏鍐点€備娇鐢ㄨ緝鏃х増鏈椂锛屾瘡涓洰鏍囧簲浣滀负
鍗曠嫭鐨勯€夐」鍖呭惈锛屽苟浠?'+' 浣滀负鍓嶇紑锛岃〃绀哄簲灏嗗叾娣诲姞鍒扮洰鏍囧垪琛?

```

    arp_ip_target=+192.168.1.1 arp_ip_target=+192.168.1.2

```
杩欐槸鎸囧畾澶氫釜鐩爣鐨勬纭娉曘€傚綋閫氳繃 BONDING_OPTS 鎸囧畾閫夐」鏃讹紝娌℃湁蹇呰缂栬緫
`/etc/modprobe.d/*.conf`銆?

瀵逛簬涓嶆敮鎸?BONDING_OPTS 鐨勬洿鏃х殑 initscripts 鐗堟湰锛岄渶瑕佺紪杈?
/etc/modprobe.d/*.conf锛堝彇鍐充簬浣犵殑鍙戣鐗堬級锛屼互渚垮湪 bond0 鎺ュ彛 up 鏃朵互浣犳墍闇€鐨?
閫夐」鍔犺浇 bonding 妯″潡銆?etc/modprobe.d/*.conf 涓殑浠ヤ笅琛屽皢鍔犺浇 bonding 妯″潡锛?
骞堕€夋嫨鍏堕€夐」锛?

	alias bond0 bonding
	options bond0 mode=balance-alb miimon=100

鐢ㄩ€傚悎浣犻厤缃殑涓€缁勯€夐」鏇挎崲绀轰緥鍙傛暟銆?

鏈€鍚庝互 root 韬唤杩愯 "/etc/rc.d/init.d/network restart"銆傝繖灏嗛噸鍚綉缁滃瓙绯荤粺锛?
浣犵殑 bond 閾捐矾鐜板湪搴旇宸茬粡 up 骞惰繍琛屻€?

### 3.2.1 鍦?Initscripts 涓娇鐢?DHCP

杈冩柊鐗堟湰鐨?initscripts锛堟嵁鎶ュ憡锛岄殢 Fedora Core 3 鍜?Red Hat Enterprise Linux 4
鎴栨洿楂樼増鏈彁渚涚殑鐗堟湰鍙敤锛夋敮鎸侀€氳繃 DHCP 涓?bonding 璁惧鍒嗛厤 IP 淇℃伅銆?

瑕佷负 DHCP 閰嶇疆 bonding锛岃鎸変笂杩版柟娉曢厤缃紝浣嗗皢 "BOOTPROTO=none" 琛屾浛鎹负
"BOOTPROTO=dhcp"锛屽苟娣诲姞涓€琛?"TYPE=Bonding"銆傛敞鎰?TYPE 鐨勫€兼槸澶у皬鍐欐晱鎰熺殑銆?

### 3.2.2 鍦?Initscripts 涓厤缃涓?Bond

闅?Fedora 7 鍜?Red Hat Enterprise Linux 5 鎻愪緵鐨?Initscripts 鍖呮敮鎸侀€氳繃绠€鍗曞湴
鍦?ifcfg-bondX锛圶 涓?bond 鐨勭紪鍙凤級涓寚瀹氶€傚綋鐨?BONDING_OPTS= 鏉ユ敮鎸佸涓?bonding
鎺ュ彛銆傛鏀寔闇€瑕佸唴鏍镐腑鐨?sysfs 鏀寔锛屼互鍙婄増鏈?3.0.0 鎴栨洿楂樼殑 bonding 椹卞姩銆?
鍏朵粬閰嶇疆鍙兘涓嶆敮鎸佽繖绉嶆寚瀹氬涓?bonding 鎺ュ彛鐨勬柟娉曪紱瀵逛簬杩欎簺鎯呭喌锛岃鍙傝涓嬮潰鐨?
鈥滄墜鍔ㄩ厤缃涓?Bond鈥濅竴鑺傘€?

### 3.3 浣跨敤 iproute2 鎵嬪姩閰嶇疆 Bonding

鏈妭閫傜敤浜庨偅浜涚綉缁滃垵濮嬪寲鑴氭湰锛坰ysconfig 鎴?initscripts 鍖咃級涓嶅叿澶?bonding
涓撻棬鐭ヨ瘑鐨勫彂琛岀増銆傚叾涓竴涓繖鏍风殑鍙戣鐗堟槸 SuSE Linux Enterprise Server 8銆?

杩欎簺绯荤粺鐨勯€氱敤鏂规硶鏄皢 bonding 妯″潡鍙傛暟鏀惧叆 /etc/modprobe.d/ 涓殑涓€涓厤缃枃浠?
锛堥€傚悎鎵€瀹夎鐨勫彂琛岀増锛夛紝鐒跺悗灏?modprobe 鍜?鎴?`ip link` 鍛戒护娣诲姞鍒扮郴缁熺殑鍏ㄥ眬
init 鑴氭湰涓€傚叏灞€ init 鑴氭湰鐨勫悕绉颁笉鍚岋紱瀵逛簬 sysconfig锛屽畠鏄?/etc/init.d/boot.local锛?
瀵逛簬 initscripts锛屽畠鏄?/etc/rc.d/rc.local銆?

渚嬪锛屽鏋滀綘鎯冲垱寤轰竴涓敱涓や釜 e100 璁惧锛堝亣瀹氫负 eth0 鍜?eth1锛夌粍鎴愮殑绠€鍗?bond锛?
骞朵笖璁╁畠鍦ㄩ噸鍚悗淇濇寔瀛樺湪锛岃缂栬緫鐩稿簲鐨勬枃浠讹紙/etc/init.d/boot.local 鎴?

```

	modprobe bonding mode=balance-alb miimon=100
	modprobe e100
	ifconfig bond0 192.168.1.1 netmask 255.255.255.0 up
	ip link set eth0 master bond0
	ip link set eth1 master bond0

```
鐢ㄩ€傚悎浣犻厤缃殑鍊兼浛鎹㈢ず渚嬩腑鐨?bonding 妯″潡鍙傛暟鍜?bond0 缃戠粶閰嶇疆锛圛P 鍦板潃銆乶etmask
绛夛級銆?

閬楁喚鐨勬槸锛屾鏂规硶涓嶄細涓?bond 璁惧鎻愪緵 ifup 鍜?ifdown 鑴氭湰鐨勬敮鎸併€傝閲嶆柊鍔犺浇
bonding

```

	# /etc/init.d/boot.local

```
```

	# /etc/rc.d/rc.local

```
鍦ㄨ繖绉嶆儏鍐典笅锛屽彲鑳藉笇鏈涘垱寤轰竴涓崟鐙殑鑴氭湰锛屽畠鍙垵濮嬪寲 bonding 閰嶇疆锛岀劧鍚庝粠
boot.local 涓皟鐢ㄨ鍗曠嫭鑴氭湰銆傝繖鏍峰氨鏃犻渶閲嶆柊杩愯鏁翠釜鍏ㄥ眬 init 鑴氭湰鍗冲彲鍚敤
bonding銆?

瑕佸叧闂?bonding 璁惧锛屽繀椤婚鍏堝皢 bonding 璁惧鏈韩鏍囪涓?down锛岀劧鍚庣Щ闄ょ浉搴旂殑
璁惧椹卞姩妯″潡銆傚浜庢垜浠笂闈㈢殑渚嬪瓙锛屼綘鍙互鎵ц

```

	# ifconfig bond0 down
	# rmmod bonding
	# rmmod e100

```
鍚屾牱锛屼负鏂逛究璧疯锛屽彲鑳藉笇鏈涘垱寤轰竴涓寘鍚繖浜涘懡浠ょ殑鑴氭湰銆?


### 3.3.1 鎵嬪姩閰嶇疆澶氫釜 Bond

鏈妭鍖呭惈涓洪偅浜涚綉缁滃垵濮嬪寲鑴氭湰涓嶆敮鎸侀厤缃涓?bond 鐨勭郴缁熼厤缃甫鏈変笉鍚岄€夐」鐨?
澶氫釜 bonding 璁惧鐨勪俊鎭€?

濡傛灉浣犻渶瑕佸涓?bonding 璁惧锛屼絾鎵€鏈夐€夐」閮界浉鍚岋紝浣犲彲鑳藉笇鏈涗娇鐢ㄤ笂闈㈡枃妗ｅ寲鐨?
"max_bonds" 妯″潡鍙傛暟銆?

瑕佸垱寤哄甫鏈変笉鍚岄€夐」鐨勫涓?bonding 璁惧锛屾渶濂戒娇鐢?sysfs 瀵煎嚭鐨?bonding 鍙傛暟锛?
璇﹁涓嬮潰涓€鑺傘€?

瀵逛簬娌℃湁 sysfs 鏀寔鐨?bonding 鐗堟湰锛屾彁渚涘甫鏈変笉鍚岄€夐」鐨勫涓?bonding 瀹炰緥鐨勫敮涓€
鏂规硶鏄娆″姞杞?bonding 椹卞姩銆傛敞鎰忥紝褰撳墠鐗堟湰鐨?sysconfig 缃戠粶鍒濆鍖栬剼鏈細鑷姩
澶勭悊杩欎竴鐐癸紱濡傛灉浣犵殑鍙戣鐗堜娇鐢ㄨ繖浜涜剼鏈紝鍒欐棤闇€鐗规畩鎿嶄綔銆傚鏋滀綘涓嶇‘瀹氫綘鐨勭綉缁?
鍒濆鍖栬剼鏈紝璇峰弬瑙佷笂闈㈢殑鈥滈厤缃?Bonding 璁惧鈥濅竴鑺傘€?

瑕佸姞杞芥ā鍧楃殑澶氫釜瀹炰緥锛屽繀椤讳负姣忎釜瀹炰緥鎸囧畾涓嶅悓鐨勫悕绉帮紙妯″潡鍔犺浇绯荤粺瑕佹眰姣忎釜宸?
鍔犺浇鐨勬ā鍧楋紝鍗充娇鏄悓涓€妯″潡鐨勫涓疄渚嬶紝閮藉叿鏈夊敮涓€鐨勫悕绉帮級銆傝繖鍙互閫氳繃鎻愪緵澶氫釜

```

	alias bond0 bonding
	options bond0 -o bond0 mode=balance-rr miimon=100

	alias bond1 bonding
	options bond1 -o bond1 mode=balance-alb miimon=50

```
灏嗗姞杞?bonding 妯″潡涓ゆ銆傜涓€涓疄渚嬪懡鍚嶄负 "bond0"锛屼互 balance-rr 妯″紡銆乵iimon
涓?100 鍒涘缓 bond0 璁惧銆傜浜屼釜瀹炰緥鍛藉悕涓?"bond1"锛屼互 balance-alb 妯″紡銆乵iimon
涓?50 鍒涘缓 bond1 璁惧銆?

鍦ㄦ煇浜涙儏鍐典笅锛堥€氬父鏄緝鏃х殑鍙戣鐗堬級锛屼笂杩版柟娉曚笉璧蜂綔鐢紝绗簩涓?bonding 瀹炰緥姘歌繙
鐪嬩笉鍒板畠鐨勯€夐」銆傚湪杩欑鎯呭喌涓嬶紝鍙互鐢ㄧ浜岃 options 鏇挎崲涓?

```

	install bond1 /sbin/modprobe --ignore-install bonding -o bond1 \
				     mode=balance-alb miimon=50

```
杩欏彲浠ラ噸澶嶄换鎰忓娆★紝涓哄悗缁瘡涓疄渚嬫寚瀹氫竴涓柊鐨勫敮涓€鍚嶇О鏉ユ浛鎹?bond1銆?

鎹瀵燂紝鏌愪簺 Red Hat 鎻愪緵鐨勫唴鏍告棤娉曞湪鍔犺浇鏃堕噸鍛藉悕妯″潡锛堝嵆 "-o bond1" 閮ㄥ垎锛夈€?
灏濊瘯灏嗚閫夐」浼犵粰 modprobe 浼氫骇鐢?"Operation not permitted" 閿欒銆傝繖鍦ㄦ煇浜?Fedora
Core 鍐呮牳涓婂凡鏈夋姤鍛婏紝鍦?RHEL 4 涓婁篃瑙佸埌杩囥€傚湪鍑虹幇姝ら棶棰樼殑鍐呮牳涓婏紝灏嗘棤娉曢厤缃?
甯︽湁涓嶅悓鍙傛暟鐨勫涓?bond锛堝洜涓哄畠浠槸杈冩棫鐨勫唴鏍革紝骞朵笖涔熺己灏?sysfs 鏀寔锛夈€?

### 3.4 閫氳繃 Sysfs 鎵嬪姩閰嶇疆 Bonding

浠庣増鏈?3.0.0 璧凤紝Channel Bonding 鍙互閫氳繃 sysfs 鎺ュ彛閰嶇疆銆傝鎺ュ彛鍏佽鍦ㄤ笉鍗歌浇
妯″潡鐨勬儏鍐典笅鍔ㄦ€侀厤缃郴缁熶腑鐨勬墍鏈?bond銆傚畠杩樺厑璁稿湪杩愯鏃舵坊鍔犲拰绉婚櫎 bond銆侷fenslave
涓嶅啀闇€瑕侊紝灏界浠嶇劧鍙楁敮鎸併€?

浣跨敤 sysfs 鎺ュ彛鍙互璁╀綘浣跨敤鍏锋湁涓嶅悓閰嶇疆鐨勫涓?bond锛岃€屾棤闇€閲嶆柊鍔犺浇妯″潡銆傚綋浣?
灏?bonding 缂栬瘧杩涘唴鏍告椂锛屽畠鍚屾牱鍏佽浣犱娇鐢ㄥ涓厤缃笉鍚岀殑 bond銆?

浣犲繀椤绘寕杞戒簡 sysfs 鏂囦欢绯荤粺鎵嶈兘浠ヨ繖绉嶆柟寮忛厤缃?bonding銆傛湰鏂囨。涓殑绀轰緥鍋囧畾浣?
浣跨敤鐨勬槸 sysfs 鐨勬爣鍑嗘寕杞界偣锛屼緥濡?/sys銆傚鏋滀綘鐨?sysfs 鏂囦欢绯荤粺鎸傝浇鍦ㄥ叾浠栦綅缃紝
浣犻渶瑕佺浉搴斿湴璋冩暣绀轰緥璺緞銆?

### 鍒涘缓涓庨攢姣?Bond

```

	# echo +foo > /sys/class/net/bonding_masters

```
```

	# echo -bar > /sys/class/net/bonding_masters

```
```

	# cat /sys/class/net/bonding_masters

```

   鐢变簬 sysfs 鏂囦欢鏈?4K 澶у皬闄愬埗锛屽鏋滀綘鏈夋暟鐧句釜浠ヤ笂鐨?bond锛屾鍒楄〃鍙兘浼氳
   鎴柇銆傚湪姝ｅ父鎿嶄綔鏉′欢涓嬭繖涓嶅お鍙兘鍙戠敓銆?

### 娣诲姞涓庣Щ闄?Slave

鍙互浣跨敤鏂囦欢 /sys/class/net/<bond>/bonding/slaves 灏嗘帴鍙?enslave 鍒版煇涓?bond銆?
璇ユ枃浠剁殑璇箟涓?bonding_masters 鏂囦欢鐩稿悓銆?

```

	# ifconfig bond0 up
	# echo +eth0 > /sys/class/net/bond0/bonding/slaves

```
```

	# echo -eth0 > /sys/class/net/bond0/bonding/slaves

```
褰撴煇涓帴鍙ｈ enslave 鍒版煇涓?bond 鏃讹紝浼氬湪 sysfs 鏂囦欢绯荤粺涓垱寤轰袱鑰呬箣闂寸殑绗﹀彿
閾炬帴銆傚湪杩欑鎯呭喌涓嬶紝浣犱細寰楀埌 /sys/class/net/bond0/slave_eth0 鎸囧悜
/sys/class/net/eth0锛屼互鍙?/sys/class/net/eth0/master 鎸囧悜 /sys/class/net/bond0銆?

杩欐剰鍛崇潃浣犲彲浠ラ€氳繃鏌ユ壘 master 绗﹀彿閾炬帴鏉ュ揩閫熷垽鏂煇涓帴鍙ｆ槸鍚﹁ enslave銆傚洜姝わ細
# echo -eth0 > /sys/class/net/eth0/master/bonding/slaves
灏嗘妸 eth0 浠庡畠琚?enslave 鐨勪换浣?bond 涓噴鏀撅紝鏃犺 bond 鎺ュ彛鐨勫悕绉版槸浠€涔堛€?

### 鏇存敼 Bond 鐨勯厤缃?

鍙互閫氳繃鎿嶄綔浣嶄簬 /sys/class/net/<bond name>/bonding 涓殑鏂囦欢鏉ュ崟鐙厤缃瘡涓?bond銆?

杩欎簺鏂囦欢鐨勫悕绉颁笌鏈枃浠朵腑鍒鎻忚堪鐨勫懡浠よ鍙傛暟鐩存帴瀵瑰簲锛屽苟涓旈櫎 arp_ip_target 澶栵紝
瀹冧滑鎺ュ彈鐩稿悓鐨勫€笺€傝鏌ョ湅褰撳墠璁剧疆锛屽彧闇€ cat 鐩稿簲鐨勬枃浠躲€?

姝ゅ缁欏嚭鍑犱釜绀轰緥锛涙湁鍏虫瘡涓弬鏁扮殑鍏蜂綋浣跨敤鎸囧崡锛岃鍙傝鏈枃妗ｄ腑鐨勭浉搴旂珷鑺傘€?

```

	# ifconfig bond0 down
	# echo 6 > /sys/class/net/bond0/bonding/mode
	- 鎴?-
	# echo balance-alb > /sys/class/net/bond0/bonding/mode

```

   鍦ㄦ洿鏀规ā寮忎箣鍓嶏紝bond 鎺ュ彛蹇呴』澶勪簬 down 鐘舵€併€?

```

	# echo 1000 > /sys/class/net/bond0/bonding/miimon

```

   濡傛灉鍚敤浜?ARP 鐩戞帶锛屽垯褰撳惎鐢?MII 鐩戞帶鏃跺畠灏嗚绂佺敤锛屽弽涔嬩害鐒躲€?

```

	# echo +192.168.0.100 > /sys/class/net/bond0/bonding/arp_ip_target
	# echo +192.168.0.101 > /sys/class/net/bond0/bonding/arp_ip_target

```

   鏈€澶氬彲浠ユ寚瀹?16 涓洰鏍囧湴鍧€銆?

```

	# echo -192.168.0.100 > /sys/class/net/bond0/bonding/arp_ip_target

```
```

	# echo 12 > /sys/class/net/bond0/bonding/lp_interval

```

   lp_interval 鏄?bonding 椹卞姩鍚戞瘡涓?slave 鐨勫绔氦鎹㈡満鍙戦€佸涔犳暟鎹寘鐨勯棿闅旂鏁般€?
   榛樿闂撮殧涓?1 绉掋€?

### 绀轰緥閰嶇疆

鎴戜滑浠庣 3.3 鑺備腑灞曠ず鐨勫悓涓€涓緥瀛愬紑濮嬶紝浣跨敤 sysfs 鎵ц锛屽苟涓斾笉浣跨敤 ifenslave銆?

瑕佸垱寤轰竴涓敱涓や釜 e100 璁惧锛堝亣瀹氫负 eth0 鍜?eth1锛夌粍鎴愮殑绠€鍗?bond锛屽苟璁╁畠鍦ㄩ噸鍚悗
淇濇寔瀛樺湪锛岃缂栬緫鐩稿簲鐨勬枃浠讹紙/etc/init.d/boot.local 鎴?/etc/rc.d/rc.local锛夛紝骞?
娣诲姞

```

	modprobe bonding
	modprobe e100
	echo balance-alb > /sys/class/net/bond0/bonding/mode
	ifconfig bond0 192.168.1.1 netmask 255.255.255.0 up
	echo 100 > /sys/class/net/bond0/bonding/miimon
	echo +eth0 > /sys/class/net/bond0/bonding/slaves
	echo +eth1 > /sys/class/net/bond0/bonding/slaves

```
瑕佹坊鍔犵浜屼釜 bond锛屽甫鏈変袱涓?e1000 鎺ュ彛锛屼娇鐢?active-backup 妯″紡锛屽苟鍚敤 ARP 鐩戞帶锛?
璇峰悜

```

	modprobe e1000
	echo +bond1 > /sys/class/net/bonding_masters
	echo active-backup > /sys/class/net/bond1/bonding/mode
	ifconfig bond1 192.168.2.1 netmask 255.255.255.0 up
	echo +192.168.2.100 /sys/class/net/bond1/bonding/arp_ip_target
	echo 2000 > /sys/class/net/bond1/bonding/arp_interval
	echo +eth2 > /sys/class/net/bond1/bonding/slaves
	echo +eth3 > /sys/class/net/bond1/bonding/slaves

```
### 3.5 浣跨敤 Interfaces 鏀寔杩涜閰嶇疆

鏈妭閫傜敤浜庨偅浜涗娇鐢?/etc/network/interfaces 鏂囦欢鏉ユ弿杩扮綉缁滄帴鍙ｉ厤缃殑鍙戣鐗堬紝鏈€
钁楀悕鐨勬槸 Debian 鍙婂叾娲剧敓鍙戣鐗堛€?

Debian 涓婄殑 ifup 鍜?ifdown 鍛戒护榛樿涓嶆敮鎸?bonding銆傚簲瀹夎 ifenslave-2.6 鍖呬互
鎻愪緵 bonding 鏀寔銆備竴鏃﹀畨瑁咃紝璇ュ寘灏嗘彁渚?`bond-*` 閫夐」锛屼緵鍦?/etc/network/interfaces
涓娇鐢ㄣ€?

娉ㄦ剰锛宨fenslave-2.6 鍖呬細鍔犺浇 bonding 妯″潡锛屽苟鍦ㄩ€傚綋鐨勬椂鍊欎娇鐢?ifenslave 鍛戒护銆?

### 绀轰緥閰嶇疆


鍦?/etc/network/interfaces 涓紝浠ヤ笅鑺傚皢閰嶇疆 bond0锛屼娇鐢?

```

	auto bond0
	iface bond0 inet dhcp
		bond-slaves eth0 eth1
		bond-mode active-backup
		bond-miimon 100
		bond-primary eth0 eth1

```
濡傛灉涓婅堪閰嶇疆涓嶈捣浣滅敤锛屼綘鍙兘浣跨敤鐨勬槸 upstart 杩涜绯荤粺鍚姩銆傛渶杩戠殑涓€浜?Ubuntu 鐗堟湰
灏ゅ叾濡傛銆?etc/network/interfaces 涓殑浠ヤ笅鑺傚皢

```

	auto bond0
	iface bond0 inet dhcp
		bond-slaves none
		bond-mode active-backup
		bond-miimon 100

	auto eth0
	iface eth0 inet manual
		bond-master bond0
		bond-primary eth0 eth1

	auto eth1
	iface eth1 inet manual
		bond-master bond0
		bond-primary eth0 eth1

```
鏈夊叧 /etc/network/interfaces 涓彈鏀寔鐨?`bond-*` 閫夐」瀹屾暣鍒楄〃锛屼互鍙婇拡瀵逛綘鐨勭壒瀹?
鍙戣鐗堝畾鍒剁殑涓€浜涙洿楂樼骇绀轰緥锛岃鍙傝 /usr/share/doc/ifenslave-2.6 涓殑鏂囦欢銆?

### 3.6 鐗规畩鎯呭喌涓嬬殑閰嶇疆瑕嗙洊


浣跨敤 bonding 椹卞姩鏃讹紝鍙戦€佹煇涓抚鐨勭墿鐞嗙鍙ｉ€氬父鐢?bonding 椹卞姩閫夋嫨锛屽鐢ㄦ埛鎴栫郴缁?
绠＄悊鍛樿€岃█骞朵笉閲嶈銆傝緭鍑虹鍙ｅ彧鏄娇鐢ㄦ墍閫?bonding 妯″紡鐨勭瓥鐣ユ潵閫夋嫨銆備笉杩囷紝鏈夋椂
灏嗘煇浜涚被鍒殑娴侀噺寮曞鍒扮壒瀹氱殑鐗╃悊杈撳嚭鎺ュ彛浠ュ疄鏂界◢寰鏉備竴浜涚殑绛栫暐鏄湁甯姪鐨勩€?
渚嬪锛岃閫氳繃涓€涓?bonded 鎺ュ彛璁块棶涓€鍙?Web 鏈嶅姟鍣紝鍏朵腑 eth0 杩炴帴鍒扮鏈夌綉缁滐紝鑰?
eth1 閫氳繃鍏叡缃戠粶杩炴帴锛屽彲鑳藉笇鏈涘亸缃 bond锛屼紭鍏堥€氳繃 eth0 鍙戦€佹绫绘祦閲忥紝浠呭湪鍥為€€
鏃舵墠浣跨敤 eth1锛岃€屾墍鏈夊叾浠栨祦閲忓彲浠ュ畨鍏ㄥ湴閫氳繃浠讳竴鎺ュ彛鍙戦€併€傛绫婚厤缃彲浠ヤ娇鐢?linux
鍥烘湁鐨勬祦閲忔帶鍒跺伐鍏锋潵瀹炵幇銆?

榛樿鎯呭喌涓嬶紝bonding 椹卞姩鏄闃熷垪鎰熺煡鐨勶紝骞跺湪椹卞姩鍒濆鍖栨椂鍒涘缓 16 涓槦鍒楋紙璇﹁
Documentation/networking/multiqueue.rst锛夈€傚鏋滈渶瑕佹洿澶氭垨鏇村皯鐨勯槦鍒楋紝鍙互浣跨敤妯″潡
鍙傛暟 tx_queues 鏉ユ洿鏀规鍊笺€傜敱浜庢病鏈?sysfs 鍙傛暟鍙敤锛堝洜涓哄垎閰嶆槸鍦ㄦā鍧楀垵濮嬪寲鏃?
瀹屾垚鐨勶級銆?

鏂囦欢 /proc/net/bonding/bondX 鐨勮緭鍑哄凡缁忔敼鍙橈紝鍥犳杈撳嚭闃熷垪

```

	Bonding Mode: fault-tolerance (active-backup)
	Primary Slave: None
	Currently Active Slave: eth0
	MII Status: up
	MII Polling Interval (ms): 0
	Up Delay (ms): 0
	Down Delay (ms): 0

	Slave Interface: eth0
	MII Status: up
	Link Failure Count: 0
	Permanent HW addr: 00:1a:a0:12:8f:cb
	Slave queue ID: 0

	Slave Interface: eth1
	MII Status: up
	Link Failure Count: 0
	Permanent HW addr: 00:1a:a0:12:8f:cc
	Slave queue ID: 2

```
```

	# echo "eth1:2" > /sys/class/net/bond0/bonding/queue_id

```
浠讳綍闇€瑕佽缃?queue_id 鐨勬帴鍙ｉ兘搴旈€氳繃绫讳技涓婇潰閭ｆ牱鐨勫娆¤皟鐢ㄦ潵璁剧疆锛岀洿鍒颁负鎵€鏈?
鎺ュ彛璁剧疆浜嗛€傚綋鐨勪紭鍏堢骇銆傚湪鍏佽閫氳繃 initscripts 閰嶇疆鐨勫彂琛岀増涓婏紝鍙互鍚?BONDING_OPTS
娣诲姞澶氫釜 'queue_id' 鍙傛暟鏉ヨ缃墍鏈夐渶瑕佺殑 slave 闃熷垪銆?

杩欎簺 queue id 鍙互涓?tc 宸ュ叿閰嶅悎浣跨敤锛岄厤缃闃熷垪 qdisc 鍜岃繃婊ゅ櫒锛屼互灏嗘煇浜涙祦閲忓亸缃?
鍒扮壒瀹氱殑 slave 璁惧涓婂彂閫併€備緥濡傦紝鍋囪鎴戜滑鎯冲湪涓婅堪閰嶇疆涓紝寮哄埗鎵€鏈夊彂寰€ 192.168.1.100
鐨勬祦閲忎娇鐢?bond 涓殑 eth1 浣滀负鍏惰緭鍑?

```

	# tc qdisc add dev bond0 handle 1 root multiq

	# tc filter add dev bond0 protocol ip parent 1: prio 1 u32 match ip \
		dst 192.168.1.100 action skbedit queue_mapping 2

```
杩欎簺鍛戒护鍛婅瘔鍐呮牳鍦?bond0 鎺ュ彛涓婇檮鍔犱竴涓闃熷垪闃熷垪瑙勫垯锛屽苟杩囨护鍏ラ槦鐨勬祦閲忥紝浣垮緱
dst ip 涓?192.168.1.100 鐨勬暟鎹寘鍏惰緭鍑洪槦鍒楁槧灏勫€艰瑕嗙洊涓?2銆傝鍊奸殢鍚庤浼犲叆椹卞姩锛?
瀵艰嚧姝ｅ父鐨勮緭鍑鸿矾寰勯€夋嫨绛栫暐琚鐩栵紝杞€岄€夋嫨 qid 2锛屽嵆鏄犲皠鍒?eth1銆?

娉ㄦ剰锛宷id 鍊间粠 1 寮€濮嬨€俀id 0 淇濈暀鐢ㄤ簬鍚戦┍鍔ㄨ〃鏄庡簲杩涜姝ｅ父鐨勮緭鍑虹瓥鐣ラ€夋嫨銆傚皢 slave
鐨?qid 绠€鍗曞湴淇濈暀涓?0 鐨勪竴涓ソ澶勬槸锛岀幇鍦?bonding 椹卞姩涓瓨鍦ㄧ殑澶氶槦鍒楁劅鐭ヨ兘鍔涖€傝繖绉?
鎰熺煡鍏佽灏?tc 杩囨护鍣ㄦ斁鍦?slave 璁惧浠ュ強 bond 璁惧涓婏紝骞朵笖 bonding 椹卞姩灏嗙畝鍗曞湴鍏呭綋
閫忎紶锛岀敤浜庡湪 slave 璁惧涓婇€夋嫨杈撳嚭闃熷垪锛岃€屼笉鏄€夋嫨杈撳嚭绔彛銆?

姝ょ壒鎬ч娆″嚭鐜板湪 bonding 椹卞姩鐗堟湰 3.7.0 涓紝骞朵笖瀵硅緭鍑?slave 閫夋嫨鐨勬敮鎸佷粎闄愪簬
round-robin 鍜?active-backup 妯″紡銆?

### 3.7 浠ユ洿瀹夊叏鐨勬柟寮忎负 802.3ad 妯″紡閰嶇疆 LACP

褰撲娇鐢?802.3ad bonding 妯″紡鏃讹紝Actor锛堜富鏈猴級涓?Partner锛堜氦鎹㈡満锛変細浜ゆ崲 LACPDU銆?
杩欎簺 LACPDU 鏃犳硶琚梾鎺紝鍥犱负瀹冧滑鍙戝線閾捐矾鏈湴 mac 鍦板潃锛堜氦鎹㈡満/缃戞ˉ涓嶅簲杞彂杩欎簺
鍦板潃锛夈€傜劧鑰岋紝澶у鏁板€煎緢瀹规槗棰勬祴锛屾垨鑰呭共鑴嗗氨鏄満鍣ㄧ殑 MAC 鍦板潃锛堝悓涓€ L2 涓殑鍏朵粬
鎵€鏈変富鏈洪兘杞绘槗鐭ラ亾锛夈€傝繖鎰忓懗鐫€ L2 鍩熶腑鐨勫叾浠栨満鍣ㄥ彲浠ヤ粠鍏朵粬涓绘満鍚戜氦鎹㈡満 spoof LACPDU
鏁版嵁鍖咃紝骞跺彲鑳介€氳繃鍔犲叆锛堜粠浜ゆ崲鏈虹殑瑙掑害鐪嬶級鍙︿竴鍙版満鍣ㄧ殑鑱氬悎鑰岄€犳垚娣蜂贡锛屼粠鑰屾帴鏀跺埌
璇ヤ富鏈哄叆绔欐祦閲忕殑涓€閮ㄥ垎锛屽拰/鎴栬嚜宸?spoof 鏉ヨ嚜璇ヤ富鏈虹殑娴侀噺锛堢敋鑷冲彲鑳芥垚鍔熺粓姝㈣涓绘満鐨?
閮ㄥ垎鏁版嵁娴侊級銆傝櫧鐒惰繖涓嶅お鍙兘鍙戠敓锛屼絾鍙互閫氳繃绠€鍗曞湴閰嶇疆鍑犱釜 bonding 鍙傛暟鏉ラ伩鍏嶈繖绉?
鍙兘鎬э細

   (a) ad_actor_system锛氫綘鍙互璁剧疆涓€涓殢鏈虹殑 mac 鍦板潃锛岀敤浜庤繖浜?LACPDU 浜ゆ崲銆傝
       鍊间笉鑳芥槸 NULL 鎴栫粍鎾湴鍧€銆傛澶栵紝鏈€濂借缃?local-admin 浣嶃€備互涓?shell 浠ｇ爜

```

	      # sys_mac_addr=$(printf '%02x:%02x:%02x:%02x:%02x:%02x' \
				       $(( (RANDOM & 0xFE) | 0x02 )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )))
	      # echo $sys_mac_addr > /sys/class/net/bond0/bonding/ad_actor_system

   (b) ad_actor_sys_prio锛氶殢鏈哄寲绯荤粺浼樺厛绾с€傞粯璁ゅ€间负 65535锛屼絾绯荤粺鍙互鍙?1 - 65535
       涔嬮棿鐨勫€笺€備互涓?shell 浠ｇ爜鐢熸垚闅忔満浼樺厛绾у苟璁剧疆瀹冿細锛?

	    # sys_prio=$(( 1 + RANDOM + RANDOM ))
	    # echo $sys_prio > /sys/class/net/bond0/bonding/ad_actor_sys_prio

   (c) ad_user_port_key锛氫娇鐢?port-key 鐨勭敤鎴烽儴鍒嗐€傞粯璁や繚鐣欎负绌恒€傝繖浜涙槸 port-key
       鐨勯珮 10 浣嶏紝鍙栧€艰寖鍥翠负 0 - 1023銆備互涓?shell 浠ｇ爜鐢熸垚杩?10 浣嶅苟璁剧疆瀹冿細锛?

	    # usr_port_key=$(( RANDOM & 0x3FF ))
	    # echo $usr_port_key > /sys/class/net/bond0/bonding/ad_user_port_key


```
## 4 鏌ヨ Bonding 閰嶇疆


### 4.1 Bonding 閰嶇疆


姣忎釜 bonding 璁惧閮芥湁涓€涓彧璇绘枃浠讹紝浣嶄簬 /proc/net/bonding 鐩綍涓€傛枃浠跺唴瀹瑰寘鍚?
鍏充簬 bonding 閰嶇疆銆侀€夐」浠ュ強姣忎釜 slave 鐘舵€佺殑淇℃伅銆?

渚嬪锛屽湪椹卞姩浠?mode=0 鍜?miimon=1000 鐨勫弬鏁板姞杞藉悗锛?proc/net/bonding/bond0 鐨勫唴瀹?
涓?

```

	Ethernet Channel Bonding Driver: 2.6.1 (October 29, 2004)
	Bonding Mode: load balancing (round-robin)
	Currently Active Slave: eth0
	MII Status: up
	MII Polling Interval (ms): 1000
	Up Delay (ms): 0
	Down Delay (ms): 0

	Slave Interface: eth1
	MII Status: up
	Link Failure Count: 1

	Slave Interface: eth0
	MII Status: up
	Link Failure Count: 1

```
纭垏鐨勬牸寮忎笌鍐呭浼氭牴鎹?bonding 閰嶇疆銆佺姸鎬佷互鍙?bonding 椹卞姩鐨勭増鏈€屽彉鍖栥€?

### 4.2 缃戠粶閰嶇疆


鍙互浣跨敤 ifconfig 鍛戒护妫€鏌ョ綉缁滈厤缃€侭onding 璁惧浼氳缃?MASTER 鏍囧織锛汢onding slave
璁惧浼氳缃?SLAVE 鏍囧織銆俰fconfig 杈撳嚭涓嶅寘鍚摢浜?slave 涓庡摢浜?master 鐩稿叧鑱旂殑淇℃伅銆?

鍦ㄤ笅闈㈢殑渚嬪瓙涓紝bond0 鎺ュ彛鏄?master锛圡ASTER锛夛紝鑰?eth0 鍜?eth1 鏄?slave锛圫LAVE锛夈€?
娉ㄦ剰锛屽浜庢墍鏈夋ā寮忥紝bond0 鐨勬墍鏈?slave 閮藉叿鏈変笌 bond0 鐩稿悓鐨?MAC 鍦板潃锛圚Waddr锛夛紝
浣?

```

  # /sbin/ifconfig
  bond0     Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    inet addr:XXX.XXX.XXX.YYY  Bcast:XXX.XXX.XXX.255  Mask:255.255.252.0
	    UP BROADCAST RUNNING MASTER MULTICAST  MTU:1500  Metric:1
	    RX packets:7224794 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:3286647 errors:1 dropped:0 overruns:1 carrier:0
	    collisions:0 txqueuelen:0

  eth0      Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    UP BROADCAST RUNNING SLAVE MULTICAST  MTU:1500  Metric:1
	    RX packets:3573025 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:1643167 errors:1 dropped:0 overruns:1 carrier:0
	    collisions:0 txqueuelen:100
	    Interrupt:10 Base address:0x1080

  eth1      Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    UP BROADCAST RUNNING SLAVE MULTICAST  MTU:1500  Metric:1
	    RX packets:3651769 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:1643480 errors:0 dropped:0 overruns:0 carrier:0
	    collisions:0 txqueuelen:100
	    Interrupt:9 Base address:0x1400

```
## 5. 浜ゆ崲鏈洪厤缃?


鍦ㄦ湰鑺備腑锛屸€滀氦鎹㈡満鈥濇寚鐨勬槸 bonded 璁惧鐩存帴杩炴帴鍒扮殑浠讳綍绯荤粺锛堝嵆缃戠嚎鐨勫彟涓€绔彃鍒扮殑
鍦版柟锛夈€傚畠鍙兘鏄竴涓湡姝ｇ殑涓撶敤浜ゆ崲鏈鸿澶囷紝涔熷彲鑳芥槸鍙︿竴涓櫘閫氱郴缁燂紙渚嬪锛屽彟涓€鍙?
杩愯 Linux 鐨勮绠楁満锛夛紝

active-backup銆乥alance-tlb 鍜?balance-alb 妯″紡涓嶉渶瑕佷氦鎹㈡満鐨勪换浣曠壒瀹氶厤缃€?

802.3ad 妯″紡瑕佹眰浜ゆ崲鏈哄皢閫傚綋鐨勭鍙ｉ厤缃负 802.3ad 鑱氬悎銆傜敤浜庨厤缃鍔熻兘鐨勫叿浣撴柟娉?
鍥犱氦鎹㈡満鑰屽紓锛屼絾渚嬪锛孋isco 3550 绯诲垪浜ゆ崲鏈鸿姹傞鍏堝皢閫傚綋鐨勭鍙ｇ粍鍚堝湪鍗曚釜
etherchannel 瀹炰緥涓紝鐒跺悗灏嗚 etherchannel 璁剧疆涓?"lacp" 妯″紡浠ュ惎鐢?802.3ad锛堣€屼笉鏄?
鏍囧噯鐨?EtherChannel锛夈€?

balance-rr銆乥alance-xor 鍜?broadcast 妯″紡閫氬父瑕佹眰浜ゆ崲鏈哄皢閫傚綋鐨勭鍙ｇ粍鍚堝湪涓€璧枫€?
杩欑缁勭殑鍛藉悕鍥犱氦鎹㈡満鑰屽紓锛屽畠鍙О涓?"etherchannel"锛堝涓婇潰鐨?Cisco 绀轰緥锛夈€?trunk
group" 鎴栧叾瀹冪被浼肩殑鍙硶銆傚浜庤繖浜涙ā寮忥紝姣忎釜浜ゆ崲鏈鸿繕浼氭湁鑷繁閽堝浜ゆ崲鏈哄埌 bond 鐨?
鍙戦€佺瓥鐣ョ殑閰嶇疆閫夐」銆傚吀鍨嬬殑閫夋嫨鍖呮嫭 MAC 鎴?IP 鍦板潃鐨?XOR銆備袱涓绔殑鍙戦€佺瓥鐣ヤ笉闇€瑕?
鍖归厤銆傚浜庤繖涓夌妯″紡锛宐onding 妯″紡瀹為檯涓婃槸涓轰竴涓?EtherChannel 缁勯€夋嫨浜嗕竴涓彂閫佺瓥鐣ワ紱
杩欎笁绉嶆ā寮忛兘灏嗕笌鍙︿竴涓?EtherChannel 缁勪簰鎿嶄綔銆?


## 6. 802.1q VLAN 鏀寔


鍙互浣跨敤 8021q 椹卞姩鍦?bond 鎺ュ彛涔嬩笂閰嶇疆 VLAN 璁惧銆傜劧鑰岋紝榛樿鎯呭喌涓嬶紝鍙湁鏉ヨ嚜
8021q 椹卞姩骞剁粡杩?bonding 鐨勬暟鎹寘鎵嶄細琚墦涓婃爣绛俱€傝嚜韬敓鎴愮殑鏁版嵁鍖咃紝渚嬪 bonding 鐨?
瀛︿範鏁版嵁鍖咃紝鎴栬€呯敱 ALB 妯″紡鎴?ARP 鐩戞帶鏈哄埗鐢熸垚鐨?ARP 鏁版嵁鍖咃紝鐢?bonding 鑷韩鍦ㄥ唴閮?
鎵撴爣绛俱€傚洜姝わ紝bonding 蹇呴』鈥滃涔犫€濋厤缃湪瀹冧箣涓婄殑 VLAN ID锛屽苟浣跨敤杩欎簺 ID 鏉ヤ负鍏惰嚜韬?
鐢熸垚鐨勬暟鎹寘鎵撴爣绛俱€?

鍑轰簬绠€鍖栦互鍙婃敮鎸佽兘澶熻繘琛?VLAN 纭欢鍔犻€熷嵏杞界殑閫傞厤鍣ㄧ殑鍘熷洜锛宐onding 鎺ュ彛澹版槑鑷繁
瀹屽叏鍏峰纭欢鍗歌浇鑳藉姏锛屽畠鑾峰彇 add_vid/kill_vid 閫氱煡浠ユ敹闆嗗繀瑕佺殑淇℃伅锛屽苟灏嗚繖浜涘姩浣?
浼犳挱鍒?slave銆傚湪娣峰悎閫傞厤鍣ㄧ被鍨嬬殑鎯呭喌涓嬶紝鏈簲缁忚繃涓嶅叿澶囧嵏杞借兘鍔涚殑閫傞厤鍣ㄧ殑纭欢鍔犻€?
甯︽爣绛炬暟鎹寘锛屼細琚?bonding 椹卞姩鈥滃幓鍔犻€熲€濓紝浣?VLAN 鏍囩浣嶄簬甯歌浣嶇疆銆?

VLAN 鎺ュ彛**蹇呴』**鍦ㄨ嚦灏?enslave 涓€涓?slave 涔嬪悗锛屾墠鑳芥坊鍔犲埌 bonding 鎺ュ彛涔嬩笂銆傚湪
娣诲姞绗竴涓?slave 涔嬪墠锛宐onding 鎺ュ彛鐨勭‖浠跺湴鍧€涓?00:00:00:00:00:00銆傚鏋滃湪绗竴娆?
enslavement 涔嬪墠鍒涘缓浜?VLAN 鎺ュ彛锛屽畠灏嗗彇寰楀叏闆剁‖浠跺湴鍧€銆備竴鏃︾涓€涓?slave 琚檮鍔犲埌
bond锛宐ond 璁惧鑷韩灏嗗彇寰楄 slave 鐨勭‖浠跺湴鍧€锛岃鍦板潃闅忓悗鍙敤浜?VLAN 璁惧銆?

鍙﹀锛岃娉ㄦ剰锛屽鏋滀粠浠嶇劧鍦ㄥ叾涓婃湁涓€涓垨澶氫釜 VLAN 鎺ュ彛鐨?bond 涓噴鏀炬墍鏈?slave锛屼篃浼?
鍙戠敓绫讳技鐨勯棶棰樸€傚綋娣诲姞涓€涓柊鐨?slave 鏃讹紝bonding 鎺ュ彛灏嗕粠绗竴涓?slave 鑾峰彇鍏剁‖浠?
鍦板潃锛岃繖鍙兘涓嶅尮閰?VLAN 鎺ュ彛鐨勭‖浠跺湴鍧€锛堝悗鑰呮渶缁堟槸浠庢洿鏃╃殑 slave 澶嶅埗鑰屾潵锛夈€?

濡傛灉鍦ㄦ墍鏈?slave 閮戒粠 bond 鎺ュ彛绉婚櫎鐨勬儏鍐典笅锛岃纭繚 VLAN 璁惧浠ユ纭殑纭欢鍦板潃杩愯锛?
鏈変袱绉嶆柟娉曪細

1. 绉婚櫎鎵€鏈?VLAN 鎺ュ彛锛岀劧鍚庨噸鏂板垱寤哄畠浠?

2. 璁剧疆 bonding 鎺ュ彛鐨勭‖浠跺湴鍧€锛屼娇鍏朵笌 VLAN 鎺ュ彛鐨勭‖浠跺湴鍧€鍖归厤

娉ㄦ剰锛屾洿鏀?VLAN 鎺ュ彛鐨?HW 鍦板潃浼氬皢搴曞眰璁惧鈥斺€斿嵆 bonding 鎺ュ彛鈥斺€旇缃负娣锋潅妯″紡锛岃繖
鍙兘骞朵笉鏄綘鎯宠鐨勩€?


## 7. 閾捐矾鐩戞帶


bonding 椹卞姩鐩墠鏀寔涓ょ鐩戞帶 slave 璁惧閾捐矾鐘舵€佺殑鏂规锛欰RP 鐩戞帶涓?MII 鐩戞帶銆?

鐩墠锛岀敱浜?bonding 椹卞姩鑷韩鐨勫疄鐜伴檺鍒讹紝鏃犳硶鍚屾椂鍚敤 ARP 涓?MII 鐩戞帶銆?

### 7.1 ARP 鐩戞帶宸ヤ綔鏈哄埗


ARP 鐩戞帶濡傚叾鍚嶇О鎵€绀鸿繍浣滐細瀹冨悜缃戠粶涓婁竴涓垨澶氫釜鎸囧畾鐨勫绔郴缁熷彂閫?ARP 鏌ヨ锛屽苟
浣跨敤鍝嶅簲浣滀负閾捐矾姝ｅ湪杩愯鐨勬寚绀恒€傝繖鎻愪緵浜嗕竴瀹氱殑淇濊瘉锛屽嵆娴侀噺纭疄鍦ㄤ笌鏈湴缃戠粶涓婄殑
涓€涓垨澶氫釜瀵圭涔嬮棿娴佸姩銆?

### 7.2 閰嶇疆澶氫釜 ARP 鐩爣

铏界劧 ARP 鐩戞帶鍙互浠呬娇鐢ㄤ竴涓洰鏍囧畬鎴愶紝浣嗗湪楂樺彲鐢ㄨ缃腑锛屾嫢鏈夊涓洰鏍囪繘琛岀洃鎺т細寰?
鏈夌敤銆傚湪鍙湁涓€涓洰鏍囩殑鎯呭喌涓嬶紝鐩爣鏈韩鍙兘瀹曟満鎴栧嚭鐜伴棶棰橈紝浠庤€屾棤娉曞搷搴?ARP 璇锋眰銆?
鎷ユ湁棰濆鐨勭洰鏍囷紙鎴栧嚑涓級鍙彁楂?ARP 鐩戞帶鐨勫彲闈犳€с€?

```

 # example options for ARP monitoring with three targets
 alias bond0 bonding
 options bond0 arp_interval=60 arp_ip_target=192.168.0.1,192.168.0.3,192.168.0.9

```
```

    # example options for ARP monitoring with one target
    alias bond0 bonding
    options bond0 arp_interval=60 arp_ip_target=192.168.0.100


```
### 7.3 MII 鐩戞帶宸ヤ綔鏈哄埗


MII 鐩戞帶鍙洃鎺ф湰鍦扮綉缁滄帴鍙ｇ殑 carrier 鐘舵€併€傚畠閫氳繃浠ヤ笅涓夌鏂瑰紡涔嬩竴瀹屾垚锛氫緷璧栬澶?
椹卞姩缁存姢鍏?carrier 鐘舵€併€佹煡璇㈣澶囩殑 MII 瀵勫瓨鍣紝鎴栧璁惧鍙戣捣 ethtool 鏌ヨ銆?

MII 鐩戞帶渚濊禆椹卞姩鑾峰彇 carrier 鐘舵€佷俊鎭紙閫氳繃 netif_carrier 瀛愮郴缁燂級銆?

## 8. 娼滃湪鐨勬晠闅滄潵婧?


### 8.1 璺敱鏂归潰鐨勫潙


閰嶇疆 bonding 鏃讹紝閲嶈鐨勬槸 slave 璁惧涓嶈鎷ユ湁鍑岄┚浜?master 璺敱涔嬩笂鐨勮矾鐢憋紙鎴栬€呬竴鑸?
鏉ヨ锛屾牴鏈笉瑕佹湁璺敱锛夈€備緥濡傦紝鍋囪 bonding 璁惧 bond0 鏈変袱涓?slave锛宔th0 鍜?eth1锛?
骞朵笖璺敱琛ㄦ槸

```

  Kernel IP routing table
  Destination     Gateway         Genmask         Flags   MSS Window  irtt Iface
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 eth0
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 eth1
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 bond0
  127.0.0.0       0.0.0.0         255.0.0.0       U        40 0          0 lo

```
杩欑璺敱閰嶇疆鍙兘浠嶄細鏇存柊椹卞姩涓殑鎺ユ敹/鍙戦€佹椂闂达紙ARP 鐩戞帶鎵€闇€锛夛紝浣嗗彲鑳戒細缁曡繃 bonding
椹卞姩锛堝洜涓哄湪鏈緥涓紝鍙戝線缃戠粶 10 涓婂彟涓€鍙颁富鏈虹殑鍑虹珯娴侀噺浼氬湪 bond0 涔嬪墠浣跨敤 eth0 鎴?
eth1锛夈€?

ARP 鐩戞帶锛堜互鍙?ARP 鏈韩锛夊彲鑳戒細琚繖绉嶉厤缃悶绯婃秱锛屽洜涓?ARP 璇锋眰锛堢敱 ARP 鐩戞帶鐢熸垚锛?
灏嗗湪涓€涓帴鍙ｏ紙bond0锛変笂鍙戦€侊紝浣嗙浉搴旂殑搴旂瓟浼氬埌杈惧彟涓€涓帴鍙ｏ紙eth0锛夈€傚浜?ARP 鑰岃█锛?
姝ゅ簲绛旂湅璧锋潵鍍忔槸鏈粡璇锋眰鐨?ARP 搴旂瓟锛堝洜涓?ARP 鏄熀浜庢帴鍙ｆ潵鍖归厤搴旂瓟鐨勶級锛屽洜鑰岃涓㈠純銆?
MII 鐩戞帶涓嶅彈璺敱琛ㄧ姸鎬佺殑褰卞搷銆?

杩欓噷鐨勮В鍐冲姙娉曞緢绠€鍗曪細纭繚 slave 娌℃湁鑷繁鐨勮矾鐢憋紝濡傛灉鐢变簬鏌愮鍘熷洜蹇呴』鏈夛紝閭ｄ簺璺敱
涔熶笉瑕佸噷椹句簬鍏?master 鐨勮矾鐢变箣涓娿€傞€氬父鎯呭喌搴斿姝わ紝浣嗕笉瀵诲父鐨勯厤缃垨閿欒鐨勪汉宸ユ垨鑷姩
闈欐€佽矾鐢辨坊鍔犲彲鑳戒細寮曞彂闂銆?

### 8.2 浠ュお缃戣澶囬噸鍛藉悕


鍦ㄩ偅浜涚綉缁滈厤缃剼鏈笉浼氬皢鐗╃悊璁惧鐩存帴涓庣綉缁滄帴鍙ｅ悕绉板叧鑱旓紙鍗冲悓涓€涓墿鐞嗚澶囧缁堝叿鏈?
鐩稿悓鐨?"ethX" 鍚嶇О锛夌殑绯荤粺涓婏紝鍙兘鏈夊繀瑕佸悜 /etc/modprobe.d/ 涓殑閰嶇疆鏂囦欢娣诲姞涓€浜?
鐗规畩閫昏緫銆?

```

	alias bond0 bonding
	options bond0 mode=some-mode miimon=50
	alias eth0 tg3
	alias eth1 tg3
	alias eth2 e1000
	alias eth3 e1000

```
濡傛灉 eth0 鍜?eth1 閮戒笉鏄?bond0 鐨?slave锛岄偅涔堝綋 bond0 鎺ュ彛 up 鏃讹紝璁惧鏈€缁堝彲鑳戒細
閲嶆柊鎺掑簭銆傚彂鐢熻繖绉嶆儏鍐垫槸鍥犱负鍏堝姞杞?bonding锛岀劧鍚庢墠鍔犺浇鍏?slave 璁惧鐨勯┍鍔ㄣ€傜敱浜庡皻鏈?
鍔犺浇鍏朵粬椹卞姩锛屽綋 e1000 椹卞姩鍔犺浇鏃讹紝瀹冨皢涓哄叾璁惧鍙栧緱 eth0 鍜?eth1锛屼絾 bonding 閰嶇疆
璇曞浘 enslave eth2 鍜?eth3锛堣繖涔嬪悗鍙兘浼氳鍒嗛厤缁?tg3 璁惧锛夈€?

```

	add above bonding e1000 tg3

```
浼氬鑷?modprobe 鍦ㄥ姞杞?bonding 鏃舵寜姝ら『搴忓厛鍔犺浇 e1000 鍐嶅姞杞?tg3銆傛鍛戒护鍦?
modules.conf 鎵嬪唽椤典腑鏈夊畬鏁存枃妗ｃ€?

鍦ㄤ娇鐢?modprobe 鐨勭郴缁熶笂涔熷彲鑳藉嚭鐜扮被浼奸棶棰樸€傚湪杩欑鎯呭喌涓嬶紝鍙互鍚戦厤缃枃浠舵坊鍔犱互涓嬪唴瀹?

```

	softdep bonding pre: tg3 e1000

```
杩欏皢鍦ㄥ姞杞?bonding 涔嬪墠鍏堝姞杞?tg3 鍜?e1000 妯″潡銆傛湁鍏虫鍐呭鐨勫畬鏁存枃妗ｅ彲鍦?modprobe.d
涓?modprobe 鎵嬪唽椤典腑鎵惧埌銆?

## 9. SNMP 浠ｇ悊

濡傛灉杩愯 SNMP 浠ｇ悊锛宐onding 椹卞姩搴斿湪浠讳綍鍙備笌 bond 鐨勭綉缁滈┍鍔ㄤ箣鍓嶅姞杞姐€傛瑕佹眰鏄洜涓?
鎺ュ彛绱㈠紩锛坕pAdEntIfIndex锛変笌鎵惧埌鐨勫叿鏈夌粰瀹?IP 鍦板潃鐨勭涓€涓帴鍙ｇ浉鍏宠仈銆備篃灏辨槸璇达紝姣忎釜
IP 鍦板潃鍙湁涓€涓?ipAdEntIfIndex銆備緥濡傦紝濡傛灉 eth0 鍜?eth1 鏄?bond0 鐨?slave锛屽苟涓?eth0
鐨勯┍鍔ㄥ湪 bonding 椹卞姩涔嬪墠鍔犺浇锛屽垯璇?IP 鍦板潃鐨勬帴鍙ｅ皢鍏宠仈鍒?eth0 鎺ュ彛銆傛閰嶇疆濡備笅鎵€绀猴紝
IP 鍦板潃 192.168.1.1 鐨勬帴鍙ｇ储寮曚负 2锛屽畠鍦?ifDescr 琛ㄤ腑绱㈠紩鍒?eth0锛坕fDescr.2锛夈€?

```

     interfaces.ifTable.ifEntry.ifDescr.1 = lo
     interfaces.ifTable.ifEntry.ifDescr.2 = eth0
     interfaces.ifTable.ifEntry.ifDescr.3 = eth1
     interfaces.ifTable.ifEntry.ifDescr.4 = eth2
     interfaces.ifTable.ifEntry.ifDescr.5 = eth3
     interfaces.ifTable.ifEntry.ifDescr.6 = bond0
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.10.10.10 = 5
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.192.168.1.1 = 2
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.74.20.94 = 4
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.127.0.0.1 = 1

```
閫氳繃鍦ㄤ换浣曞弬涓?bond 鐨勭綉缁滈┍鍔ㄤ箣鍓嶅姞杞?bonding 椹卞姩锛屽彲浠ラ伩鍏嶆闂銆備笅闈㈡槸鍏堝姞杞?
bonding 椹卞姩鐨勪緥瀛愶紝IP 鍦板潃 192.168.1.1 姝ｇ‘鍦板叧鑱斿埌 ifDescr.2銆?

     interfaces.ifTable.ifEntry.ifDescr.1 = lo
     interfaces.ifTable.ifEntry.ifDescr.2 = bond0
     interfaces.ifTable.ifEntry.ifDescr.3 = eth0
     interfaces.ifTable.ifEntry.ifDescr.4 = eth1
     interfaces.ifTable.ifEntry.ifDescr.5 = eth2
     interfaces.ifTable.ifEntry.ifDescr.6 = eth3
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.10.10.10 = 6
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.192.168.1.1 = 2
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.74.20.94 = 5
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.127.0.0.1 = 1

铏界劧鏌愪簺鍙戣鐗堝彲鑳藉湪 ifDescr 涓笉鎶ュ憡鎺ュ彛鍚嶇О锛屼絾 IP 鍦板潃涓?IfIndex 涔嬮棿鐨勫叧鑱斾粛鐒?
瀛樺湪锛屽苟涓?SNMP 鍔熻兘锛堝 Interface_Scan_Next锛変細鎶ュ憡璇ュ叧鑱斻€?

## 10. 娣锋潅妯″紡


杩愯缃戠粶鐩戞帶宸ュ叿锛堜緥濡?tcpdump锛夋椂锛岄€氬父浼氬惎鐢ㄨ澶囦笂鐨勬贩鏉傛ā寮忥紝浠ヤ究鐪嬪埌鎵€鏈夋祦閲?
锛堣€屼笉鏄彧鐪嬪埌鍙戝線鏈湴涓绘満鐨勬祦閲忥級銆俠onding 椹卞姩澶勭悊瀵?bonding master 璁惧锛堜緥濡?
bond0锛夌殑娣锋潅妯″紡鏇存敼锛屽苟灏嗚璁剧疆浼犳挱鍒?slave
璁惧銆?

瀵逛簬 balance-rr銆乥alance-xor銆乥roadcast 鍜?802.3ad 妯″紡锛屾贩鏉傛ā寮忚缃細浼犳挱鍒?
鎵€鏈?slave銆?

瀵逛簬 active-backup銆乥alance-tlb 鍜?balance-alb 妯″紡锛屾贩鏉傛ā寮忚缃粎浼犳挱鍒?active
slave銆?

瀵逛簬 balance-tlb 妯″紡锛宎ctive slave 鏄綋鍓嶆鍦ㄦ帴鏀跺叆绔欐祦閲忕殑 slave銆?

瀵逛簬 balance-alb 妯″紡锛宎ctive slave 鏄綔涓衡€減rimary鈥濅娇鐢ㄧ殑 slave銆傝 slave 鐢ㄤ簬
妯″紡鐗瑰畾鐨勬帶鍒舵祦閲忥紝鐢ㄤ簬鍚戞湭鍒嗛厤鐨勫绔彂閫佹祦閲忥紝鎴栧湪璐熻浇涓嶅潎琛℃椂鍙戦€併€?

瀵逛簬 active-backup銆乥alance-tlb 鍜?balance-alb 妯″紡锛屽綋 active slave 鍙戠敓鍙樺寲鏃?
锛堜緥濡傜敱浜庨摼璺晠闅滐級锛屾贩鏉傝缃皢琚紶鎾埌鏂扮殑 active slave銆?

## 11. 涓?High Availability 閰嶇疆 Bonding


楂樺彲鐢ㄦ寚鐨勬槸閫氳繃鎷ユ湁鍐椾綑鎴栧浠借澶囥€侀摼璺垨浜ゆ崲鏈猴紙浣嶄簬涓绘満涓庡鐣屼箣闂达級鏉ユ彁渚?
鏈€澶х綉缁滃彲鐢ㄦ€х殑閰嶇疆銆傜洰鏍囨槸鎻愪緵鏈€澶х殑缃戠粶杩炴帴鍙敤鎬э紙鍗崇綉缁滃缁堝彲鐢級锛屽嵆浣?
鍏朵粬閰嶇疆鍙兘鎻愪緵鏇撮珮鐨勫悶鍚愰噺銆?

### 11.1 鍗曚氦鎹㈡満鎷撴墤涓殑楂樺彲鐢?


濡傛灉涓や釜涓绘満锛堟垨涓€鍙颁富鏈轰笌鍗曚釜浜ゆ崲鏈猴級閫氳繃澶氭潯鐗╃悊閾捐矾鐩存帴鐩歌繛锛岄偅涔堜紭鍖栦负鏈€澶?
甯﹀涓嶄細甯︽潵鍙敤鎬ф崯澶便€傚湪杩欑鎯呭喌涓嬶紝鍙湁涓€涓氦鎹㈡満锛堟垨瀵圭锛夛紝鍥犳濡傛灉瀹冨け璐ワ紝
灏辨病鏈夊彲鏁呴殰鍒囨崲鍒扮殑澶囩敤鎺ュ叆銆傛澶栵紝bonding 璐熻浇鍧囪　妯″紡鏀寔瀵瑰叾鎴愬憳鐨勯摼璺洃鎺э紝
鍥犳濡傛灉涓埆閾捐矾澶辫触锛岃礋杞藉皢鍦ㄥ墿浣欑殑璁惧涓婇噸鏂板潎琛°€?

鏈夊叧浣跨敤鍗曚釜瀵圭璁惧閰嶇疆 bonding 鐨勪俊鎭紝璇峰弬瑙佺 12 鑺傗€滀负鏈€澶у悶鍚愰噺閰嶇疆 Bonding鈥濄€?

### 11.2 澶氫氦鎹㈡満鎷撴墤涓殑楂樺彲鐢?


鏈変簡澶氫釜浜ゆ崲鏈猴紝bonding 涓庣綉缁滅殑閰嶇疆浼氬彂鐢熷法澶у彉鍖栥€傚湪澶氫氦鎹㈡満鎷撴墤涓紝缃戠粶鍙敤鎬?
涓庡彲鐢ㄥ甫瀹戒箣闂村瓨鍦ㄦ潈琛°€?

涓嬮潰鏄竴涓厤缃负鏈€澶у寲

```

		|                                     |
		|port3                           port3|
	  +-----+----+                          +-----+----+
	  |          |port2       ISL      port2|          |
	  | switch A +--------------------------+ switch B |
	  |          |                          |          |
	  +-----+----+                          +-----++---+
		|port1                           port1|
		|             +-------+               |
		+-------------+ host1 +---------------+
			 eth0 +-------+ eth1

```
鍦ㄦ閰嶇疆涓紝涓や釜浜ゆ崲鏈轰箣闂存湁涓€鏉￠摼璺紙ISL锛屽嵆浜ゆ崲鏈洪棿閾捐矾锛夛紝骞舵湁澶氫釜绔彛杩炴帴鍒?
澶栭儴涓栫晫锛堟瘡涓氦鎹㈡満涓婄殑 鈥減ort3鈥濓級銆備粠鎶€鏈笂璁诧紝娌℃湁鐞嗙敱涓嶈兘灏嗗叾鎵╁睍鍒扮涓変釜浜ゆ崲鏈恒€?

### 11.2.1 澶氫氦鎹㈡満鎷撴墤鐨?HA Bonding 妯″紡閫夋嫨


鍦ㄥ儚涓婇潰杩欐牱鐨勬嫇鎵戜腑锛屽綋浼樺寲鍙敤鎬ф椂锛宎ctive-backup 鍜?broadcast 妯″紡鏄敮涓€鏈夌敤鐨?
bonding 妯″紡锛涘叾浠栨ā寮忚姹傛墍鏈夐摼璺兘缁堟浜庡悓涓€涓绔紝鎵嶈兘鍚堢悊鍦板伐浣溿€?

active-backup:
	杩欓€氬父鏄閫夋ā寮忥紝鐗瑰埆鏄鏋滀氦鎹㈡満鍏锋湁 ISL 骞朵笖閰嶅悎鑹ソ銆傚鏋?
	缃戠粶閰嶇疆浣垮緱鏌愪釜浜ゆ崲鏈鸿鏄庣‘鎸囧畾涓哄浠戒氦鎹㈡満锛堜緥濡傦紝瀹归噺杈冧綆銆?
	鎴愭湰杈冮珮绛夛級锛屽垯鍙互浣跨敤 primary 閫夐」鏉ョ‘淇濋閫夐摼璺湪鍙敤鏃舵€绘槸
	琚娇鐢ㄣ€?

broadcast:
	姝ゆā寮忕‘瀹炴槸涓€涓壒娈婄敤閫旀ā寮忥紝浠呴€傜敤浜庨潪甯哥壒瀹氱殑闇€姹傘€備緥濡傦紝
	濡傛灉涓や釜浜ゆ崲鏈烘湭杩炴帴锛堟棤 ISL锛夛紝骞朵笖瀹冧滑涔嬪鐨勭綉缁滃畬鍏ㄧ嫭绔嬨€傚湪
	杩欑鎯呭喌涓嬶紝濡傛灉鏌愪簺鐗瑰畾鐨勫崟鍚戞祦閲忔湁蹇呰鍒拌揪涓や釜鐙珛鐨勭綉缁滐紝閭ｄ箞
	broadcast 妯″紡鍙兘閫傜敤銆?

### 11.2.2 澶氫氦鎹㈡満鎷撴墤鐨?HA 閾捐矾鐩戞帶閫夋嫨


閾捐矾鐩戞帶鐨勯€夋嫨鏈€缁堝彇鍐充簬浣犵殑浜ゆ崲鏈恒€傚鏋滀氦鎹㈡満鑳藉鍙潬鍦板湪鍝嶅簲鍏朵粬鏁呴殰鏃跺皢绔彛
缃负澶辫触锛岄偅涔?MII 鎴?ARP 鐩戞帶閮藉簲璇ュ彲鐢ㄣ€備緥濡傦紝鍦ㄤ笂闈㈢殑渚嬪瓙涓紝濡傛灉 "port3" 閾捐矾
鍦ㄨ繙绔け璐ワ紝MII 鐩戞帶娌℃湁鐩存帴鎵嬫妫€娴嬭繖涓€鐐广€侫RP 鐩戞帶鍙互閰嶇疆 port3 杩滅鐨勪竴涓洰鏍囷紝
浠庤€屾棤闇€浜ゆ崲鏈烘敮鎸佸嵆鍙娴嬭鏁呴殰銆?

鐒惰€岋紝涓€鑸潵璇达紝鍦ㄥ浜ゆ崲鏈烘嫇鎵戜腑锛孉RP 鐩戞帶鍦ㄦ娴嬬鍒扮杩為€氭€ф晠闅滐紙鍙兘鐢变换浣曞崟涓?
缁勪欢鍥犱换浣曞師鍥犳棤娉曚紶閫掓祦閲忚€屽紩璧凤級鏂归潰鍙互鎻愪緵鏇撮珮绛夌骇鐨勫彲闈犳€с€傛澶栵紝ARP 鐩戞帶搴?
閰嶇疆澶氫釜鐩爣锛堢綉缁滀腑姣忎釜浜ゆ崲鏈鸿嚦灏戜竴涓級銆傝繖灏嗙‘淇濓紝鏃犺鍝釜浜ゆ崲鏈哄浜庢椿鍔ㄧ姸鎬侊紝
ARP 鐩戞帶閮芥湁涓€涓悎閫傜殑鐩爣鍙緵鏌ヨ銆?

鍙﹀杩樿娉ㄦ剰锛岃繎鏉ヨ澶氫氦鎹㈡満鏀寔涓€绉嶉€氬父绉颁负 鈥渢runk failover鈥?鐨勫姛鑳姐€傝繖鏄氦鎹㈡満鐨?
涓€涓壒鎬э紝褰撳彟涓€涓氦鎹㈡満绔彛鐨勭姸鎬佸彉涓?down锛堟垨 up锛夋椂锛屼細浣挎煇涓壒瀹氫氦鎹㈡満绔彛鐨?
閾捐矾鐘舵€佽缃负 down锛堟垨 up锛夈€傚叾鐩殑鏄皢閾捐矾鏁呴殰浠庨€昏緫涓娾€滃閮ㄢ€濈殑绔彛浼犳挱鍒?bonding
鑳藉閫氳繃 miimon 鐩戞帶鐨勯€昏緫涓娾€滃唴閮ㄢ€濈殑绔彛銆倀runk failover 鐨勫彲鐢ㄦ€т笌閰嶇疆鍥犱氦鎹㈡満鑰屽紓锛?
浣嗗湪浣跨敤鍚堥€傜殑浜ゆ崲鏈烘椂锛屽畠鍙互浣滀负 ARP 鐩戞帶鐨勪竴涓彲琛屾浛浠ｆ柟妗堛€?

## 12. 涓烘渶澶у悶鍚愰噺閰嶇疆 Bonding


### 12.1 鍗曚氦鎹㈡満鎷撴墤涓殑鏈€澶у悶鍚愰噺


鍦ㄥ崟浜ゆ崲鏈洪厤缃腑锛屾渶澶у寲鍚炲悙閲忕殑鏈€浣虫柟娉曞彇鍐充簬搴旂敤涓庣綉缁滅幆澧冦€傚悇绉嶈礋杞藉潎琛℃ā寮忓湪
涓嶅悓鐜涓嬪悇鏈変紭缂虹偣锛岃杩板涓嬨€?

鍦ㄦ湰鏂囪璁轰腑锛屾垜浠皢鎷撴墤鍒嗕负涓ょ被銆傛牴鎹ぇ澶氭暟娴侀噺鐨勭洰鐨勫湴锛屾垜浠皢瀹冧滑褰掔被涓?
鈥済atewayed锛堢粡缃戝叧锛夆€濇垨 鈥渓ocal锛堟湰鍦帮級鈥濋厤缃€?

鍦?gatewayed 閰嶇疆涓紝鈥滀氦鎹㈡満鈥濅富瑕佸厖褰撹矾鐢卞櫒锛屽ぇ澶氭暟娴侀噺缁忚繃姝よ矾鐢卞櫒鍒拌揪

```

     +----------+                     +----------+
     |          |eth0            port1|          | to other networks
     | Host A   +---------------------+ router   +------------------->
     |          +---------------------+          | Hosts B and C are out
     |          |eth1            port2|          | here somewhere
     +----------+                     +----------+

```
璺敱鍣ㄥ彲浠ユ槸涓€涓笓鐢ㄧ殑璺敱鍣ㄨ澶囷紝鎴栨槸鍏呭綋缃戝叧鐨勫彟涓€鍙颁富鏈恒€傚湪鏈枃璁ㄨ涓紝閲嶇偣鏄?
Host A 鐨勫ぇ澶氭暟娴侀噺鍦ㄥ埌杈惧叾鏈€缁堢洰鐨勫湴涔嬪墠锛岄兘浼氱粡杩囪矾鐢卞櫒鍒拌揪鏌愪釜鍏朵粬缃戠粶銆?

鍦?gatewayed 缃戠粶閰嶇疆涓紝铏界劧 Host A 鍙兘涓庤澶氬叾浠栫郴缁熼€氫俊锛屼絾鍏舵墍鏈夋祦閲忛兘灏嗛€氳繃
鏈湴缃戠粶涓婄殑鍙︿竴涓绔€斺€旇矾鐢卞櫒鈥斺€斿彂閫佸拰鎺ユ敹銆?

娉ㄦ剰锛屼袱鍙扮郴缁熼€氳繃澶氭潯鐗╃悊閾捐矾鐩存帴鐩歌繛鐨勬儏鍐碉紝灏遍厤缃?bonding 鑰岃█锛屼笌 gatewayed
閰嶇疆鐩稿悓銆傚湪杩欑鎯呭喌涓嬶紝纰板阀鎵€鏈夋祦閲忛兘鍙戝線鈥滅綉鍏斥€濇湰韬紝鑰屼笉鏄綉鍏充箣澶栫殑鏌愪釜鍏朵粬缃戠粶銆?

鍦?local 閰嶇疆涓紝鈥滀氦鎹㈡満鈥濅富瑕佸厖褰撲氦鎹㈡満锛屽ぇ澶氭暟娴侀噺缁忚繃姝や氦鎹㈡満鍒拌揪鍚屼竴缃戠粶涓婄殑
鍏朵粬绔欑偣銆備緥濡?

```

    +----------+            +----------+       +--------+
    |          |eth0   port1|          +-------+ Host B |
    |  Host A  +------------+  switch  |port3  +--------+
    |          +------------+          |                  +--------+
    |          |eth1   port2|          +------------------+ Host C |
    +----------+            +----------+port4             +--------+


```
鍚屾牱锛屼氦鎹㈡満鍙互鏄竴涓笓鐢ㄧ殑浜ゆ崲鏈鸿澶囷紝鎴栨槸鍏呭綋缃戝叧鐨勫彟涓€鍙颁富鏈恒€傚湪鏈枃璁ㄨ涓紝
閲嶇偣鏄?Host A 鐨勫ぇ澶氭暟娴侀噺閮藉彂寰€鍚屼竴鏈湴缃戠粶涓婄殑鍏朵粬涓绘満锛堜笂渚嬩腑鐨?Hosts B 鍜?C锛夈€?

鎬讳箣锛屽湪 gatewayed 閰嶇疆涓紝寰€杩?bonded 璁惧鐨勬祦閲忛兘灏嗗彂寰€缃戠粶涓婂悓涓€涓?MAC 灞傜骇鐨勫绔?
锛堢綉鍏虫湰韬紝鍗宠矾鐢卞櫒锛夛紝鏃犺鍏舵渶缁堢洰鐨勫湴濡備綍銆傚湪 local 閰嶇疆涓紝娴侀噺鐩存帴鍦ㄦ渶缁堢洰鐨勫湴
涔嬮棿娴佸姩锛屽洜姝ゆ瘡涓洰鐨勫湴锛圚ost B銆丠ost C锛夐兘灏嗙敱鍏跺悇鑷殑 MAC 鍦板潃鐩存帴瀵诲潃銆?

gatewayed 涓?local 缃戠粶閰嶇疆涔嬮棿鐨勮繖绉嶅尯鍒緢閲嶈锛屽洜涓鸿澶氬彲鐢ㄧ殑璐熻浇鍧囪　妯″紡閮戒娇鐢?
鏈湴缃戠粶婧愬拰鐩殑鐨?MAC 鍦板潃鏉ュ仛鍑鸿礋杞藉潎琛″喅绛栥€傛瘡绉嶆ā寮忕殑琛屼负濡備笅鎵€杩般€?


### 12.1.1 鍗曚氦鎹㈡満鎷撴墤鐨?MT Bonding 妯″紡閫夋嫨


姝ら厤缃渶瀹规槗鎼缓鍜岀悊瑙ｏ紝灏界浣犲皢涓嶅緱涓嶅喅瀹氬摢绉?bonding 妯″紡鏈€閫傚悎浣犵殑闇€姹傘€傛瘡绉嶆ā寮?
鐨勬潈琛¤杩板涓嬶細

balance-rr:
	姝ゆā寮忔槸鍞竴鍏佽鍗曚釜 TCP/IP 杩炴帴灏嗘祦閲忔潯甯﹀寲鍒板涓帴鍙ｇ殑妯″紡銆傚洜姝わ紝
	瀹冧篃鏄敮涓€鍏佽鍗曚釜 TCP/IP 娴佸埄鐢ㄨ秴杩囦竴涓帴鍙ｇ殑鍚炲悙閲忕殑妯″紡銆備絾杩?
	鏄湁浠ｄ环鐨勶細鏉″甫鍖栭€氬父浼氬鑷村绔郴缁熸敹鍒颁贡搴忕殑鏁版嵁鍖咃紝浠庤€屽紩鍙?
	TCP/IP 鐨勬嫢濉炴帶鍒剁郴缁熶粙鍏ワ紝閫氬父琛ㄧ幇涓洪噸浼犳銆?

	鍙互閫氳繃淇敼 net.ipv4.tcp_reordering sysctl 鍙傛暟鏉ヨ皟鏁?TCP/IP 鐨勬嫢濉?
	闄愬埗銆傞€氬父鐨勯粯璁ゅ€间负 3銆備絾璇疯浣忥紝TCP 鏍堝湪妫€娴嬪埌閲嶆帓搴忔椂鑳藉鑷姩
	澧炲ぇ姝ゅ€笺€?

	娉ㄦ剰锛屽皢琚贡搴忎氦浠樼殑鏁版嵁鍖呮瘮渚嬮珮搴﹀彲鍙橈紝涓嶅お鍙兘涓洪浂銆傞噸鎺掑簭鐨勭▼搴?
	鍙栧喅浜庡绉嶅洜绱狅紝鍖呮嫭缃戠粶鎺ュ彛銆佷氦鎹㈡満浠ュ強閰嶇疆鐨勬嫇鎵戙€備竴鑸潵璇达紝閫熺巼
	鏇撮珮鐨勭綉鍗′細浜х敓鏇村閲嶆帓搴忥紙鐢变簬鏁版嵁鍖呭悎骞剁瓑鍥犵礌锛夛紝骞朵笖 鈥滃瀵瑰鈥?
	鎷撴墤姣旇緝 鈥滃鎱㈠涓€蹇€?鐨勯厤缃細浠ユ洿楂樻瘮鐜囬噸鎺掑簭銆?

	璁稿浜ゆ崲鏈轰笉鏀寔浠讳綍鏉″甫鍖栨祦閲忕殑妯″紡锛堣€屾槸鍩轰簬 IP 鎴?MAC 灞傜骇鍦板潃
	閫夋嫨绔彛锛夛紱瀵逛簬杩欎簺璁惧锛屾祦缁忎氦鎹㈡満鍒拌揪 balance-rr bond 鐨勭壒瀹?
	杩炴帴鐨勬祦閲忓皢鏃犳硶鍒╃敤瓒呰繃涓€涓帴鍙ｇ殑甯﹀銆?

	濡傛灉浣犱娇鐢ㄧ殑鏄?TCP/IP 涔嬪鐨勫崗璁紙渚嬪 UDP锛夛紝骞朵笖浣犵殑搴旂敤鑳藉瀹瑰繊
	涔卞簭浜や粯锛岄偅涔堟妯″紡鍙互瀹炵幇鎺ヨ繎绾挎€х殑鍗曟祦鏁版嵁鎶ユ€ц兘鎵╁睍锛岄殢鐫€
	鎺ュ彛琚姞鍏ュ埌 bond 涓€?

	姝ゆā寮忚姹備氦鎹㈡満灏嗛€傚綋鐨勭鍙ｉ厤缃负 鈥渆therchannel鈥?鎴?鈥渢runking鈥濄€?

active-backup:
	鍦ㄨ繖绉嶇綉缁滄嫇鎵戜腑锛宎ctive-backup 妯″紡娌℃湁澶ぇ浼樺娍锛屽洜涓轰笉娲诲姩鐨勫浠?
	璁惧閮戒笌 primary 杩炴帴鍒板悓涓€涓绔€傚湪杩欑鎯呭喌涓嬶紝璐熻浇鍧囪　妯″紡锛堝甫
	閾捐矾鐩戞帶锛夊皢鎻愪緵鐩稿悓绛夌骇鐨勭綉缁滃彲鐢ㄦ€э紝浣嗗叿鏈夋洿楂樼殑鍙敤甯﹀銆傚ソ鐨勪竴闈㈡槸锛?
	active-backup 妯″紡涓嶉渶瑕佸浜ゆ崲鏈鸿繘琛屼换浣曢厤缃紝鍥犳濡傛灉鍙敤鐨勭‖浠朵笉鏀寔
	浠讳綍璐熻浇鍧囪　妯″紡锛屽畠鍙兘浠嶆湁浠峰€笺€?

balance-xor:
	姝ゆā寮忎細闄愬埗娴侀噺锛屼娇鍙戝線鐗瑰畾瀵圭鐨勬暟鎹寘鎬绘槸閫氳繃鍚屼竴涓帴鍙ｅ彂閫併€傜敱浜?
	鐩殑鍦扮敱鎵€娑夊強鐨?MAC 鍦板潃鍐冲畾锛屾妯″紡鍦?鈥渓ocal鈥?缃戠粶閰嶇疆锛堝涓婃墍杩帮級
	涓晥鏋滄渶浣筹紝涓旀墍鏈夌洰鐨勫湴閮藉湪鍚屼竴鏈湴缃戠粶涓娿€傚鏋滀綘鐨勬墍鏈夋祦閲忛兘缁忚繃鍗曚釜
	璺敱鍣紙鍗冲涓婃墍杩扮殑 鈥済atewayed鈥?缃戠粶閰嶇疆锛夛紝姝ゆā寮忓彲鑳芥浼樸€?

	涓?balance-rr 涓€鏍凤紝浜ゆ崲鏈虹鍙ｉ渶瑕侀厤缃负 鈥渆therchannel鈥?鎴?鈥渢runking鈥濄€?

broadcast:
	涓?active-backup 绫讳技锛屽湪杩欑绫诲瀷鐨勭綉缁滄嫇鎵戜腑锛屾妯″紡娌℃湁澶ぇ浼樺娍銆?

802.3ad:
	姝ゆā寮忓浜庤繖绉嶇被鍨嬬殑缃戠粶鎷撴墤鏄竴涓笉閿欑殑閫夋嫨銆?02.3ad 妯″紡鏄竴涓?IEEE
	鏍囧噯锛屽洜姝ゆ墍鏈夊疄鐜?802.3ad 鐨勫绔兘搴旇鑳借壇濂戒簰鎿嶄綔銆?02.3ad 鍗忚鍖呭惈
	鑱氬悎鐨勮嚜鍔ㄩ厤缃紝鍥犳鍙渶瑕佸浜ゆ崲鏈鸿繘琛屾渶灏戠殑鎵嬪姩閰嶇疆锛堥€氬父鍙槸鎸囧畾鏌愮粍
	璁惧鍙敤浜?802.3ad锛夈€?02.3ad 鏍囧噯杩樿姹傚抚鎸夐『搴忎氦浠橈紙鍦ㄤ竴瀹氶檺搴﹀唴锛夛紝鍥犳
	涓€鑸潵璇村崟杩炴帴涓嶄細鐪嬪埌鏁版嵁鍖呬贡搴忋€?02.3ad 妯″紡纭疄鏈変竴浜涚己鐐癸細鏍囧噯瑕佹眰鍦?
	鑱氬悎涓殑鎵€鏈夎澶囦互鐩稿悓鐨勯€熺巼鍜屽弻宸ヨ繍琛屻€傛澶栵紝涓庨櫎 balance-rr 涔嬪鐨勬墍鏈?
	bonding 璐熻浇鍧囪　妯″紡涓€鏍凤紝娌℃湁浠讳綍鍗曚釜杩炴帴鑳藉鍒╃敤瓒呰繃涓€涓帴鍙ｇ殑甯﹀銆?

	姝ゅ锛宭inux bonding 鐨?802.3ad 瀹炵幇鎸夊绔垎鍙戞祦閲忥紙浣跨敤 MAC 鍦板潃涓庢暟鎹寘
	绫诲瀷 ID 鐨?XOR锛夛紝鍥犳鍦?鈥済atewayed鈥?閰嶇疆涓紝鎵€鏈夊嚭绔欐祦閲忛€氬父閮戒細浣跨敤鍚屼竴涓?
	璁惧銆傚叆绔欐祦閲忎篃鍙兘鏈€缁堣惤鍦ㄥ崟涓澶囦笂锛屼絾杩欏彇鍐充簬瀵圭 802.3ad 瀹炵幇鐨勫潎琛?
	绛栫暐銆傚湪 鈥渓ocal鈥?閰嶇疆涓紝娴侀噺灏嗗垎甯冨湪 bond 涓殑鍚勪釜璁惧涓娿€?

	鏈€鍚庯紝802.3ad 妯″紡寮哄埗浣跨敤 MII 鐩戞帶锛屽洜姝ゅ湪姝ゆā寮忎笅 ARP 鐩戞帶涓嶅彲鐢ㄣ€?

balance-tlb:
	balance-tlb 妯″紡鎸夊绔潎琛″嚭绔欐祦閲忋€傜敱浜庡潎琛℃槸鏍规嵁 MAC 鍦板潃杩涜鐨勶紝鍦?
	鈥済atewayed鈥?閰嶇疆锛堝涓婃墍杩帮級涓紝姝ゆā寮忎細閫氳繃鍗曚釜璁惧鍙戦€佹墍鏈夋祦閲忋€傜劧鑰岋紝鍦?
	鈥渓ocal鈥?缃戠粶閰嶇疆涓紝姝ゆā寮忎互涓€绉嶆ā绯婃櫤鑳界殑鏂瑰紡锛堜笉鏄儚 balance-xor 鎴?
	802.3ad 妯″紡涓偅鏍风畝鍗曠殑 XOR锛夊湪璁惧闂村潎琛″涓湰鍦扮綉缁滃绔紝浣垮緱鏁板涓婁笉璧拌繍
	鐨?MAC 鍦板潃锛堝嵆 XOR 鍒扮浉鍚屽€肩殑閭ｄ簺锛変笉浼氬叏閮ㄢ€滆仛闆嗏€濆湪鍗曚釜鎺ュ彛涓娿€?

	涓?802.3ad 涓嶅悓锛屾帴鍙ｅ彲浠ュ叿鏈変笉鍚岀殑閫熺巼锛屽苟涓斾笉闇€瑕佺壒娈婄殑浜ゆ崲鏈洪厤缃€備笉鍒╃殑涓€闈㈡槸锛?
	鍦ㄦ妯″紡涓嬫墍鏈夊叆绔欐祦閲忛兘閫氳繃鍗曚釜鎺ュ彛鍒拌揪锛屾妯″紡瑕佹眰 slave 鎺ュ彛鐨勭綉缁滆澶囬┍鍔?
	鍏峰鐗瑰畾鐨?ethtool 鏀寔锛屽苟涓?ARP 鐩戞帶涓嶅彲鐢ㄣ€?

balance-alb:
	姝ゆā寮忓氨鏄?balance-tlb 鐨勪竴鍒囷紝鐢氳嚦鏇村銆傚畠鍏峰 balance-tlb 鐨勬墍鏈夌壒鎬э紙鍜?
	闄愬埗锛夛紝骞朵笖杩樹細鍧囪　鏉ヨ嚜鏈湴缃戠粶瀵圭鐨勫叆绔欐祦閲忥紙濡備笂闈㈢殑 Bonding 妯″潡閫夐」涓€鑺?
	鎵€杩帮級銆?

	姝ゆā寮忓敮涓€棰濆鐨勪笉鍒╀箣澶勬槸锛岀綉缁滆澶囬┍鍔ㄥ繀椤绘敮鎸佸湪璁惧澶勪簬鎵撳紑鐘舵€佹椂鏇存敼纭欢
	鍦板潃銆?

### 12.1.2 鍗曚氦鎹㈡満鎷撴墤鐨?MT 閾捐矾鐩戞帶


閾捐矾鐩戞帶鐨勯€夋嫨鍙兘鍦ㄥ緢澶х▼搴︿笂鍙栧喅浜庝綘閫夋嫨浣跨敤鐨勬ā寮忋€傛洿楂樼骇鐨勮礋杞藉潎琛℃ā寮忎笉鏀寔
浣跨敤 ARP 鐩戞帶锛屽洜姝や粎闄愪簬浣跨敤 MII 鐩戞帶锛堝畠鎻愪緵鐨勭鍒扮淇濊瘉涓嶅 ARP 鐩戞帶楂橈級銆?

### 12.2 澶氫氦鎹㈡満鎷撴墤涓殑鏈€澶у悶鍚愰噺


褰撳涓氦鎹㈡満浣滀负闅旂缃戠粶鐨勪竴閮ㄥ垎骞惰閰嶇疆鏃讹紝鍙互鍒╃敤瀹冧滑鏉ヤ紭鍖栧悶鍚愰噺

```

		       +-----------+
		       |  Host A   |
		       +-+---+---+-+
			 |   |   |
		+--------+   |   +---------+
		|            |             |
	 +------+---+  +-----+----+  +-----+----+
	 | Switch A |  | Switch B |  | Switch C |
	 +------+---+  +-----+----+  +-----+----+
		|            |             |
		+--------+   |   +---------+
			 |   |   |
		       +-+---+---+-+
		       |  Host B   |
		       +-----------+

```
鍦ㄦ閰嶇疆涓紝浜ゆ崲鏈哄郊姝ら殧绂汇€傞噰鐢ㄨ繖绉嶆嫇鎵戠殑涓€涓師鍥犳槸锛屽浜庝竴涓嫢鏈夎澶氫富鏈虹殑闅旂缃戠粶
锛堜緥濡傦紝涓洪珮鎬ц兘閰嶇疆鐨勯泦缇わ級锛屼娇鐢ㄥ涓緝灏忕殑浜ゆ崲鏈哄彲鑳芥瘮鍗曚釜杈冨ぇ鐨勪氦鎹㈡満鏇村叿鎴愭湰鏁堢泭锛?
渚嬪锛屽湪涓€涓湁 24 鍙颁富鏈虹殑缃戠粶涓婏紝涓夊彴 24 绔彛浜ゆ崲鏈哄彲鑳芥瘮鍗曞彴 72 绔彛浜ゆ崲鏈轰究瀹滃緱澶氥€?

濡傛灉闇€瑕佽闂綉缁滀箣澶栫殑璧勬簮锛屽彲浠ヤ负鍗曚釜涓绘満閰嶅涓€涓繛鎺ュ埌澶栭儴缃戠粶鐨勯澶栫綉缁滆澶囷紱
璇ヤ富鏈洪殢鍚庝篃鍏呭綋缃戝叧銆?

### 12.2.1 澶氫氦鎹㈡満鎷撴墤鐨?MT Bonding 妯″紡閫夋嫨


鍦ㄥ疄闄呬腑锛屾绫婚厤缃€氬父閲囩敤鐨?bonding 妯″紡鏄?balance-rr銆備粠鍘嗗彶涓婄湅锛屽湪杩欑缃戠粶閰嶇疆涓紝
鍏充簬鏁版嵁鍖呬贡搴忎氦浠樼殑閫氬父鍛婅浼氶€氳繃浣跨敤涓嶈繘琛屼换浣曟暟鎹寘鍚堝苟鐨勭綉缁滈€傞厤鍣紙閫氳繃浣跨敤
NAPI锛屾垨鍥犱负璁惧鏈韩鍦ㄨ嫢骞叉暟鎹寘鍒拌揪涔嬪墠涓嶄骇鐢熶腑鏂級鏉ョ紦瑙ｃ€備互杩欑鏂瑰紡浣跨敤鏃讹紝
balance-rr 妯″紡鍏佽涓ゅ彴涓绘満涔嬮棿鐨勫崟涓繛鎺ユ湁鏁堝湴鍒╃敤瓒呰繃涓€涓帴鍙ｇ殑甯﹀銆?

### 12.2.2 澶氫氦鎹㈡満鎷撴墤鐨?MT 閾捐矾鐩戞帶


鍚屾牱锛屽湪瀹為檯涓紝姝ら厤缃腑鏈€甯镐娇鐢ㄧ殑鏄?MII 鐩戞帶锛屽洜涓烘€ц兘浼樺厛浜庡彲鐢ㄦ€с€侫RP 鐩戞帶鍦ㄦ
鎷撴墤涓兘澶熷伐浣滐紝浣嗛殢鐫€娑夊強绯荤粺鏁伴噺鐨勫闀匡紝鎵€闇€鎺㈡祴鐨勬暟閲忎細鍓婂急鍏剁浉瀵逛簬 MII 鐩戞帶鐨勪紭鍔?
锛堣璁颁綇锛岀綉缁滀腑鐨勬瘡鍙颁富鏈洪兘閰嶇疆浜?bonding锛夈€?

## 13. 浜ゆ崲鏈鸿涓洪棶棰?


### 13.1 閾捐矾寤虹珛涓庢晠闅滃垏鎹㈠欢杩?


鏌愪簺浜ゆ崲鏈哄湪閾捐矾 up 涓?down 涓婃姤鐨勬椂鏈烘柟闈㈣〃鐜板嚭涓嶈壇琛屼负銆?

棣栧厛锛屽綋閾捐矾 up 鏃讹紝鏌愪簺浜ゆ崲鏈哄彲鑳芥寚绀洪摼璺凡 up锛坈arrier 鍙敤锛夛紝浣嗗湪涓€娈垫椂闂村唴涓嶉€氳繃
鎺ュ彛浼犻€掓祦閲忋€傝繖绉嶅欢杩熼€氬父鏄敱浜庢煇绉嶇被鍨嬬殑鑷姩鍗忓晢鎴栬矾鐢卞崗璁紝浣嗕篃鍙兘鍙戠敓鍦ㄤ氦鎹㈡満
鍒濆鍖栨湡闂达紙渚嬪锛屽湪浜ゆ崲鏈烘晠闅滄仮澶嶆湡闂达級銆傚鏋滀綘鍙戠幇杩欐槸涓棶棰橈紝璇蜂负 updelay bonding
妯″潡閫夐」鎸囧畾涓€涓€傚綋鐨勫€硷紝浠ュ欢杩熶娇鐢ㄧ浉鍏虫帴鍙ｃ€?

鍏舵锛屾煇浜涗氦鎹㈡満鍙兘鍦ㄩ摼璺姸鎬佹敼鍙樻椂灏嗗叾鈥滄姈鍔ㄢ€濅竴娆℃垨澶氭銆傝繖鏈€甯歌浜庝氦鎹㈡満鍒濆鍖栨湡闂淬€?
鍚屾牱锛屼竴涓€傚綋鐨?updelay 鍊煎彲鑳戒細鏈夋墍甯姪銆?

娉ㄦ剰锛屽綋 bonding 鎺ュ彛娌℃湁娲诲姩閾捐矾鏃讹紝椹卞姩灏嗙珛鍗抽噸鐢ㄧ涓€涓?up 鐨勯摼璺紝鍗充娇鎸囧畾浜?updelay
鍙傛暟锛堝湪杩欑鎯呭喌涓?updelay 琚拷鐣ワ級銆傚鏋滄湁 slave 鎺ュ彛姝ｅ湪绛夊緟 updelay 瓒呮椂鍒版湡锛屽垯
鏈€鍏堣繘鍏ヨ鐘舵€佺殑鎺ュ彛灏嗚绔嬪嵆閲嶇敤銆傚鏋?updelay 鐨勫€艰楂樹及锛岃繖浼氬噺灏戠綉缁滅殑鍋滄満鏃堕棿锛屽苟涓?
鐢变簬杩欑鎯呭喌鍙彂鐢熷湪娌℃湁杩為€氭€ф椂锛屽拷鐣?updelay 涓嶄細甯︽潵棰濆鐨勬儵缃氥€?

闄や簡瀵逛氦鎹㈡満鏃跺簭鐨勬媴蹇т箣澶栵紝濡傛灉浣犵殑浜ゆ崲鏈洪渶瑕佸緢闀挎椂闂存墠鑳借繘鍏ュ浠芥ā寮忥紝鍙兘甯屾湜鍦ㄩ摼璺?
down 鍚庝笉瑕佺珛鍗虫縺娲诲浠芥帴鍙ｃ€傚彲浠ラ€氳繃 downdelay bonding 妯″潡閫夐」寤惰繜鏁呴殰鍒囨崲銆?

### 13.2 閲嶅鐨勫叆绔欐暟鎹寘


娉ㄦ剰锛氫粠鐗堟湰 3.0.2 璧凤紝bonding 椹卞姩鍏锋湁鎶戝埗閲嶅鏁版嵁鍖呯殑閫昏緫锛岃繖搴旇鑳藉熀鏈秷闄ゆ闂銆?
浠ヤ笅鎻忚堪淇濈暀浠ヤ緵鍙傝€冦€?

鍦ㄩ娆′娇鐢?bonding 璁惧鍚庯紝鎴栧湪瀹冮棽缃竴娈垫椂闂村悗锛岃瀵熷埌鐭殏鐨勯噸澶嶆祦閲忕獊鍙戝苟涓嶇綍瑙併€傝繖
鏈€瀹规槗閫氳繃瑙傚療鍚戠綉缁滀笂鍙︿竴鍙颁富鏈哄彂鍑?鈥減ing鈥?骞舵敞鎰?ping 鐨勮緭鍑烘爣璁颁簡閲嶅椤癸紙閫氬父姣忎釜
slave 涓€涓級鏉ヨ瀵熴€?

渚嬪锛屽湪涓€涓湁浜斾釜 slave 鐨?active-backup 妯″紡 bond 涓?

```

	# ping -n 10.0.4.2
	PING 10.0.4.2 (10.0.4.2) from 10.0.3.10 : 56(84) bytes of data.
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.7 ms
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=2 ttl=64 time=0.216 ms
	64 bytes from 10.0.4.2: icmp_seq=3 ttl=64 time=0.267 ms
	64 bytes from 10.0.4.2: icmp_seq=4 ttl=64 time=0.222 ms

```
杩欏苟涓嶆槸 bonding 椹卞姩鐨勯敊璇紝鑰屾槸璁稿浜ゆ崲鏈烘洿鏂板叾 MAC 杞彂琛ㄦ柟寮忕殑涓€涓壇浣滅敤銆傛渶鍒濓紝
浜ゆ崲鏈轰笉浼氬皢鏁版嵁鍖呬腑鐨?MAC 鍦板潃涓庣壒瀹氫氦鎹㈡満绔彛鍏宠仈锛屽洜姝ゅ彲鑳戒細灏嗘祦閲忓彂閫佸埌鎵€鏈夌鍙ｏ紝
鐩村埌鍏?MAC 杞彂琛ㄦ洿鏂般€傜敱浜庨檮鍔犲埌 bond 鐨勬帴鍙ｅ彲鑳藉崰鎹崟涓氦鎹㈡満涓婄殑澶氫釜绔彛锛屽綋浜ゆ崲鏈?
锛堜复鏃讹級灏嗘祦閲忔硾娲埌鎵€鏈夌鍙ｆ椂锛宐ond 璁惧浼氭敹鍒板悓涓€鏁版嵁鍖呯殑澶氫釜鍓湰锛堟瘡涓?slave 璁惧
涓€涓級銆?

閲嶅鏁版嵁鍖呯殑琛屼负鍙栧喅浜庝氦鎹㈡満锛屾湁浜涗氦鎹㈡満浼氬嚭鐜拌繖绉嶈涓猴紝鏈変簺鍒欎笉浼氥€傚湪琛ㄧ幇鍑鸿繖绉嶈涓?
鐨勪氦鎹㈡満涓婏紝鍙互閫氳繃娓呴櫎 MAC 杞彂琛ㄦ潵寮曞彂瀹冿紙鍦ㄥぇ澶氭暟 Cisco 浜ゆ崲鏈轰笂锛岀壒鏉冨懡浠?
鈥渃lear mac address-table dynamic鈥?鍙互瀹炵幇杩欎竴鐐癸級銆?

## 14. 纭欢鐩稿叧娉ㄦ剰浜嬮」


鏈妭鍖呭惈鍦ㄧ壒瀹氱‖浠跺钩鍙颁笂閰嶇疆 bonding锛屾垨灏?bonding 涓庣壒瀹氫氦鎹㈡満鎴栧叾浠栬澶囧鎺ョ殑闄勫姞淇℃伅銆?

### 14.1 IBM BladeCenter


杩欓€傜敤浜?JS20 鍙婄被浼肩郴缁熴€?

鍦?JS20 鍒€鐗囦笂锛宐onding 椹卞姩浠呮敮鎸?balance-rr銆乤ctive-backup銆乥alance-tlb 鍜?balance-alb
妯″紡銆傝繖鍦ㄥ緢澶х▼搴︿笂鏄敱浜?BladeCenter 鍐呴儴鐨勭綉缁滄嫇鎵戯紝璇﹁涓嬫枃銆?

### JS20 缃戠粶閫傞厤鍣ㄤ俊鎭?


鎵€鏈?JS20 閮介厤鏈夐泦鎴愬湪 planar锛圛BM 鏈涓殑鈥滀富鏉库€濓級涓婄殑涓や釜 Broadcom 鍗冨厗浠ュお缃戠鍙ｃ€傚湪
BladeCenter 鏈虹涓紝鎵€鏈?JS20 鍒€鐗囩殑 eth0 绔彛閮界‖杩炵嚎鍒?I/O 妯″潡 #1锛涚被浼煎湴锛屾墍鏈?eth1
绔彛閮借繛绾垮埌 I/O 妯″潡 #2銆傚彲浠ュ湪 JS20 涓婂畨瑁呬竴涓檮鍔犵殑 Broadcom 瀛愬崱锛屼互鎻愪緵鍙﹀涓や釜鍗冨厗
浠ュお缃戠鍙ｃ€傝繖浜涚鍙?eth2 鍜?eth3 鍒嗗埆杩炵嚎鍒?I/O 妯″潡 3 鍜?4銆?

姣忎釜 I/O 妯″潡鍙互鍖呭惈涓€涓氦鎹㈡満鎴栦竴涓洿閫氭ā鍧楋紙鍏佽绔彛鐩存帴杩炴帴鍒板閮ㄤ氦鎹㈡満锛夈€傛煇浜?
bonding 妯″紡闇€瑕佺壒瀹氱殑 BladeCenter 鍐呴儴缃戠粶鎷撴墤鎵嶈兘宸ヤ綔锛涜繖浜涜杩板涓嬨€?

鏇村 BladeCenter 鐗瑰畾鐨勭綉缁滀俊鎭彲浠ュ湪涓ゆ湰 IBM Redbook锛坵ww.ibm.com/redbooks锛変腑鎵惧埌锛?

- "IBM eServer BladeCenter Networking Options"
- "IBM eServer BladeCenter Layer 2-7 Network Switching"

### BladeCenter 缃戠粶閰嶇疆


鐢变簬 BladeCenter 鍙互浠ラ潪甯稿鐨勬柟寮忛厤缃紝鏈璁哄皢浠呴檺浜庢弿杩板熀鏈厤缃€?

閫氬父锛屼互澶綉浜ゆ崲妯″潡锛圗SM锛夌敤浜?I/O 妯″潡 1 鍜?2銆傚湪姝ら厤缃腑锛孞S20 鐨?eth0 鍜?eth1 绔彛
灏嗚繛鎺ュ埌涓嶅悓鐨勫唴閮ㄤ氦鎹㈡満锛堝湪鍚勮嚜鐨?I/O 妯″潡涓級銆?

鐩撮€氭ā鍧楋紙OPM 鎴?CPM锛屽厜鍙ｆ垨閾滃彛鐩撮€氭ā鍧楋級灏?I/O 妯″潡鐩存帴杩炴帴鍒板閮ㄤ氦鎹㈡満銆傞€氳繃鍦?I/O
妯″潡 #1 鍜?#2 涓娇鐢?PM锛孞S20 鐨?eth0 鍜?eth1 鎺ュ彛鍙互琚噸瀹氬悜鍒板閮ㄤ笘鐣岋紝骞惰繛鎺ュ埌鍏卞悓鐨?
澶栭儴浜ゆ崲鏈恒€?

鏍规嵁 ESM 鍜?PM 鐨勭粍鍚堬紝缃戠粶瀵?bonding 鑰岃█瑕佷箞琛ㄧ幇涓哄崟浜ゆ崲鏈烘嫇鎵戯紙鍏ㄩ儴涓?PM锛夛紝瑕佷箞琛ㄧ幇涓?
澶氫氦鎹㈡満鎷撴墤锛堜竴涓垨澶氫釜 ESM锛岄浂涓垨澶氫釜 PM锛夈€備篃鍙互灏?ESM 浜掕繛锛屼粠鑰屼骇鐢熶竴涓潪甯哥被浼间簬
涓婇潰鈥滃浜ゆ崲鏈烘嫇鎵戜腑鐨勯珮鍙敤鈥濈ず渚嬬殑閰嶇疆銆?

### 鐗瑰畾妯″紡鐨勮姹?


balance-rr 妯″紡瑕佹眰 bond 涓殑璁惧浣跨敤鐩撮€氭ā鍧楋紝鍏ㄩ儴杩炴帴鍒颁竴涓叡鍚岀殑澶栭儴浜ゆ崲鏈恒€傝浜ゆ崲鏈哄繀椤?
鍦ㄩ€傚綋鐨勭鍙ｄ笂閰嶇疆涓?鈥渆therchannel鈥?鎴?鈥渢runking鈥濓紝杩欐槸 balance-rr 鐨勯€氬父瑕佹眰銆?

balance-alb 鍜?balance-tlb 妯″紡鍙互鍚屾椂浣跨敤浜ゆ崲鏈烘ā鍧楁垨鐩撮€氭ā鍧楋紙鎴栨贩鍚堬級銆傝繖浜涙ā寮忓敮涓€
鐗瑰畾鐨勮姹傛槸锛屾墍鏈夌綉缁滄帴鍙ｅ繀椤昏兘澶熷埌杈鹃€氳繃 bonding 璁惧鍙戦€佺殑娴侀噺鐨勬墍鏈夌洰鐨勫湴锛堝嵆缃戠粶蹇呴』
鍦?BladeCenter 涔嬪鐨勬煇涓偣姹囪仛锛夈€?

active-backup 妯″紡娌℃湁棰濆瑕佹眰銆?

### 閾捐矾鐩戞帶闂


褰撲互澶綉浜ゆ崲妯″潡灏变綅鏃讹紝鍙湁 ARP 鐩戞帶鑳藉彲闈犲湴妫€娴嬪埌涓庡閮ㄤ氦鎹㈡満鐨勯摼璺涪澶便€傝繖娌′粈涔堝紓甯革紝
浣嗘鏌?BladeCenter 鏈烘煖浼氳浜轰互涓衡€滃閮ㄢ€濈綉缁滅鍙ｅ氨鏄郴缁熺殑浠ュお缃戠鍙ｏ紝鑰屼簨瀹炰笂鍦ㄨ繖浜涒€滃閮ㄢ€?
绔彛涓?JS20 绯荤粺鏈韩涓婄殑璁惧涔嬮棿鏈変竴涓氦鎹㈡満銆侻II 鐩戞帶鍙兘妫€娴?ESM 涓?JS20 绯荤粺涔嬮棿鐨勯摼璺?
鏁呴殰銆?

褰撶洿閫氭ā鍧楀氨浣嶆椂锛孧II 鐩戞帶纭疄鑳芥娴嬪埌鈥滃閮ㄢ€濈鍙ｇ殑鏁呴殰锛岃绔彛闅忓悗鐩存帴杩炴帴鍒?JS20 绯荤粺銆?

### 鍏朵粬娉ㄦ剰浜嬮」


Serial Over LAN锛圫oL锛夐摼璺粎寤虹珛鍦ㄤ富浠ュお缃戯紙eth0锛変笂锛屽洜姝わ紝浠讳綍鍒?eth0 鐨勯摼璺涪澶遍兘灏嗗鑷?
浣犲け鍘?SoL 杩炴帴銆傚畠涓嶄細涓庡叾浠栫綉缁滄祦閲忎竴璧锋晠闅滃垏鎹紝鍥犱负 SoL 绯荤粺瓒呭嚭浜?bonding 椹卞姩鐨勬帶鍒?
鑼冨洿銆?

鍙兘甯屾湜绂佺敤浜ゆ崲鏈猴紙鏃犺鏄唴閮ㄤ互澶綉浜ゆ崲妯″潡杩樻槸澶栭儴浜ゆ崲鏈猴級涓婄殑鐢熸垚鏍戯紝浠ラ伩鍏嶅湪浣跨敤 bonding
鏃跺嚭鐜版晠闅滃垏鎹㈠欢杩熼棶棰樸€?


## 15. 甯歌闂瑙ｇ瓟


### 1.  瀹冩槸鍚?SMP 瀹夊叏锛?


鏄殑銆傛棫鐨?2.0.xx channel bonding 琛ヤ竵涓嶆槸 SMP 瀹夊叏鐨勩€傛柊鐨勯┍鍔ㄤ粠涓€寮€濮嬭璁″氨鏄?SMP 瀹夊叏鐨勩€?

### 2.  鍝簺绫诲瀷鐨勭綉鍗″彲浠ヤ笌瀹冧竴璧峰伐浣滐紵


浠讳綍浠ュお缃戠被鍨嬬殑缃戝崱锛堜綘鐢氳嚦鍙互娣风敤缃戝崱鈥斺€斾緥濡備竴鍧?Intel EtherExpress PRO/100 鍜屼竴鍧?3com
3c905b锛夈€傚浜庡ぇ澶氭暟妯″紡锛岃澶囦笉闇€瑕佸叿鏈夌浉鍚岀殑閫熺巼銆?

浠庣増鏈?3.2.1 璧凤紝bonding 杩樻敮鎸?active-backup 妯″紡涓嬬殑 Infiniband slave銆?

### 3.  鎴戝彲浠ユ湁澶氬皯涓?bonding 璁惧锛?


娌℃湁闄愬埗銆?

### 4.  涓€涓?bonding 璁惧鍙互鏈夊灏戜釜 slave锛?


杩欎粎鍙?Linux 鏀寔鐨勭綉缁滄帴鍙ｆ暟閲忥紝鍜?鎴栦綘鍙互鍦ㄧ郴缁熶腑鏀惧叆鐨勭綉缁滃崱鏁伴噺鐨勯檺鍒躲€?

### 5.  褰?slave 閾捐矾姝绘帀鏃朵細鍙戠敓浠€涔堬紵


濡傛灉鍚敤浜嗛摼璺洃鎺э紝鍒欏け璐ョ殑璁惧灏嗚绂佺敤銆俛ctive-backup 妯″紡灏嗘晠闅滃垏鎹㈠埌澶囦唤閾捐矾锛屽叾浠栨ā寮?
灏嗗拷鐣ュけ璐ョ殑閾捐矾銆傝閾捐矾灏嗙户缁鐩戞帶锛屽鏋滃畠鎭㈠锛屽畠灏嗕互閫傚悎璇ユā寮忕殑鏂瑰紡閲嶆柊鍔犲叆 bond銆傛湁鍏?
鏇村淇℃伅锛岃鍙傝楂樺彲鐢ㄤ竴鑺備互鍙婃瘡绉嶆ā寮忕殑鏂囨。銆?

閾捐矾鐩戞帶鍙互閫氳繃 miimon 鎴?arp_interval 鍙傛暟锛堝涓婃ā鍧楀弬鏁颁竴鑺傛墍杩帮級鍚敤銆備竴鑸潵璇达紝miimon
鐩戞帶搴曞眰缃戠粶璁惧鎰熺煡鍒扮殑 carrier 鐘舵€侊紝鑰?arp 鐩戞帶锛坅rp_interval锛夌洃鎺т笌鏈湴缃戠粶涓婂彟涓€鍙颁富鏈虹殑
杩為€氭€с€?

濡傛灉鏈厤缃摼璺洃鎺э紝bonding 椹卞姩灏嗘棤娉曟娴嬮摼璺晠闅滐紝骞跺皢鍋囧畾鎵€鏈夐摼璺缁堝彲鐢ㄣ€傝繖寰堝彲鑳戒細瀵艰嚧
涓㈠寘锛屼互鍙婇殢涔嬭€屾潵鐨勬€ц兘涓嬮檷銆傜‘鍒囩殑鎬ц兘鎹熷け鍙栧喅浜?bonding 妯″紡涓庣綉缁滈厤缃€?

### 6.  bonding 鑳界敤浜庨珮鍙敤鍚楋紵


鍙互銆傝瑙侀珮鍙敤涓€鑺傘€?

### 7.  瀹冮€傜敤浜庡摢浜涗氦鎹㈡満/绯荤粺锛?


瀵规鐨勫畬鏁寸瓟妗堝彇鍐充簬鎵€闇€鐨勬ā寮忋€?

鍦ㄥ熀鏈潎琛℃ā寮忥紙balance-rr 鍜?balance-xor锛変腑锛屽畠閫傜敤浜庝换浣曟敮鎸?etherchannel锛堜篃绉颁负 trunking锛?
鐨勭郴缁熴€傜洰鍓嶅ぇ澶氭暟鍙楃浜ゆ崲鏈洪兘鏈夋绫绘敮鎸侊紝璁稿闈炲彈绠′氦鎹㈡満涔熸湁銆?

楂樼骇鍧囪　妯″紡锛坆alance-tlb 鍜?balance-alb锛夋病鏈夌壒娈婄殑浜ゆ崲鏈鸿姹傦紝浣嗛渶瑕佹敮鎸佺壒瀹氬姛鑳界殑璁惧椹卞姩
锛堝湪涓婇潰鐨勬ā鍧楀弬鏁颁笅鐨勭浉搴斿皬鑺備腑鎻忚堪锛夈€?

鍦?802.3ad 妯″紡涓紝瀹冮€傜敤浜庢敮鎸?IEEE 802.3ad 鍔ㄦ€侀摼璺仛鍚堢殑绯荤粺銆傜洰鍓嶅ぇ澶氭暟鍙楃浠ュ強璁稿闈炲彈绠?
浜ゆ崲鏈洪兘鏀寔 802.3ad銆?

active-backup 妯″紡搴旇閫傜敤浜庝换浣曚簩灞傦紙Layer-II锛変氦鎹㈡満銆?

### 8.  bonding 璁惧鐨?MAC 鍦板潃浠庡摢閲屾潵锛?


褰撲娇鐢ㄥ叿鏈夊浐瀹?MAC 鍦板潃鐨?slave 璁惧锛屾垨鍚敤浜?fail_over_mac 閫夐」鏃讹紝bonding 璁惧鐨?MAC 鍦板潃鏄?
active slave 鐨?MAC 鍦板潃銆?

瀵逛簬鍏朵粬閰嶇疆锛屽鏋滄湭鏄惧紡閰嶇疆锛堜娇鐢?ifconfig 鎴?ip link锛夛紝bonding 璁惧鐨?MAC 鍦板潃鍙栬嚜鍏剁涓€涓?
slave 璁惧銆傝 MAC 鍦板潃闅忓悗琚紶閫掔粰鎵€鏈夊悗缁殑 slave锛屽苟淇濇寔鎸佷箙锛堝嵆浣跨涓€涓?slave 琚Щ闄わ級锛岀洿鍒?
bonding 璁惧琚?down 鎴栭噸鏂伴厤缃€?

濡傛灉浣犳兂鏇存敼 MAC 鍦板潃锛屽彲浠ヤ娇鐢?

```

	# ifconfig bond0 hw ether 00:11:22:33:44:55

	# ip link set bond0 address 66:77:88:99:aa:bb

```
MAC 鍦板潃涔熷彲浠ラ€氳繃灏?bond 璁惧 down/up 鏉ユ洿鏀?

```

	# ifconfig bond0 down ; modprobe -r bonding
	# ifconfig bond0 .... up
	# ifenslave bond0 eth...

```
姝ゆ柟娉曞皢鑷姩浠庢帴涓嬫潵鍔犲叆鐨?slave 鑾峰彇鍦板潃銆?

瑕佹仮澶嶄綘鐨?slave 鐨?MAC 鍦板潃锛屼綘闇€瑕佸皢瀹冧滑浠?bond 涓婂垎绂伙紙`ifenslave -d bond0 eth0`锛夈€俠onding
椹卞姩闅忓悗灏嗘仮澶嶈繖浜?slave 鍦ㄨ enslave 涔嬪墠鎷ユ湁鐨?MAC 鍦板潃銆?

### 9.  鍝簺 bonding 妯″紡鏀寔鍘熺敓 XDP锛?


  - balance-rr (0)
  - active-backup (1)
  - balance-xor (2)
  - 802.3ad (4)

娉ㄦ剰锛寁lan+srcmac 鍝堝笇绛栫暐涓嶆敮鎸佸師鐢?XDP銆傚浜庡叾浠?bonding 妯″紡锛孹DP 绋嬪簭蹇呴』浠ラ€氱敤妯″紡鍔犺浇銆?

## 16. 璧勬簮涓庨摼鎺?


bonding 椹卞姩鐨勬渶鏂扮増鏈彲浠ュ湪 linux 鍐呮牳鐨勬渶鏂扮増鏈腑鎵惧埌锛屼綅浜?http://kernel.org

鏈枃妗ｇ殑鏈€鏂扮増鏈彲浠ュ湪鏈€鏂板唴鏍告簮鐮佷腑鎵惧埌锛堝悕涓?Documentation/networking/bonding.rst锛夈€?

鏈夊叧 bonding 椹卞姩寮€鍙戠殑璁ㄨ鍙戠敓鍦ㄤ富瑕佺殑 Linux 缃戠粶閭欢鍒楄〃涓婏紝鎵樼浜?vger.kernel.org銆傝鍒楄〃鐨?
鍦板潃涓猴細

netdev@vger.kernel.org

绠＄悊鐣岄潰锛堢敤浜庤闃呮垨閫€璁級鍙互鍦ㄤ互涓嬩綅缃壘鍒帮細

http://vger.kernel.org/vger-lists.html#netdev
