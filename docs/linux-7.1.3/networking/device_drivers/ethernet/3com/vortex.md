
## 3Com Vortex 璁惧椹卞姩


Andrew Morton

2000 骞?4 鏈?30 鏃?

鏈枃妗ｆ弿杩颁簡鐢ㄤ簬 Linux 鐨?3Com "Vortex" 璁惧椹卞姩 3c59x.c 鐨勪娇鐢ㄦ柟娉曚笌鍕樿銆?
璇ラ┍鍔ㄧ敱 Donald Becker <becker@scyld.com> 缂栧啓銆?
Don 宸蹭笉鍐嶆槸姝ょ増鏈┍鍔ㄧ殑涓昏缁存姢鑰呫€傝灏嗛棶棰樻姤鍛婄粰浠ヤ笅涓€浣嶆垨澶氫綅锛?
- Andrew Morton
- Netdev 閭欢鍒楄〃 <netdev@vger.kernel.org>
- Linux 鍐呮牳閭欢鍒楄〃 <linux-kernel@vger.kernel.org>

璇锋敞鎰忔湰鏂囨。鏈熬鐨勨€滄姤鍛婁笌璇婃柇闂鈥濅竴鑺傘€?

鑷唴鏍?2.3.99-pre6 璧凤紝璇ラ┍鍔ㄥ凡鏁村悎瀵?3c575 绯诲垪 Cardbus 鍗＄殑鏀寔锛岃繖浜涘崱姝ゅ墠鐢?3c575_cb.c 澶勭悊銆?
鏈┍鍔ㄦ敮鎸佷互涓嬬‖浠讹細

 - 3c590 Vortex 10Mbps
 - 3c592 EISA 10Mbps Demon/Vortex
 - 3c597 EISA Fast Demon/Vortex
 - 3c595 Vortex 100baseTx
 - 3c595 Vortex 100baseT4
 - 3c595 Vortex 100base-MII
 - 3c900 Boomerang 10baseT
 - 3c900 Boomerang 10Mbps Combo
 - 3c900 Cyclone 10Mbps TPO
 - 3c900 Cyclone 10Mbps Combo
 - 3c900 Cyclone 10Mbps TPC
 - 3c900B-FL Cyclone 10base-FL
 - 3c905 Boomerang 100baseTx
 - 3c905 Boomerang 100baseT4
 - 3c905B Cyclone 100baseTx
 - 3c905B Cyclone 10/100/BNC
 - 3c905B-FX Cyclone 100baseFx
 - 3c905C Tornado
 - 3c920B-EMB-WNM (ATI Radeon 9100 IGP)
 - 3c980 Cyclone
 - 3c980C Python-T
 - 3cSOHO100-TX Hurricane
 - 3c555 Laptop Hurricane
 - 3c556 Laptop Tornado
 - 3c556B Laptop Hurricane
 - 3c575 [Megahertz] 10/100 LAN  CardBus
 - 3c575 Boomerang CardBus
 - 3CCFE575BT Cyclone CardBus
 - 3CCFE575CT Tornado CardBus
 - 3CCFE656 Cyclone CardBus
 - 3CCFEM656B Cyclone+Winmodem CardBus
 - 3CXFEM656C Tornado+Winmodem CardBus
 - 3c450 HomePNA Tornado
 - 3c920 Tornado
 - 3c982 Hydra Dual Port A
 - 3c982 Hydra Dual Port B
 - 3c905B-T4
 - 3c920B-EMB-WNM Tornado

## 妯″潡鍙傛暟


鍦ㄥ姞杞芥ā鍧楁椂锛屽彲浠ュ悜椹卞姩鎻愪緵鑻ュ共鍙傛暟銆傝繖浜涘弬鏁伴€氬父鏀剧疆鍦?`/etc/modprobe.d/*.conf` 涓€?
```
    options 3c59x debug=3 rx_copybreak=300

```

濡傛灉浣犱娇鐢ㄧ殑鏄?PCMCIA 宸ュ叿锛坈ardmgr锛夛紝鍒欏弬鏁板彲鑳藉涓嬶細

```
    module "3c59x" opts "debug=3 rx_copybreak=300"

```

鏀寔鐨勫弬鏁板涓嬶細

debug=N

  鍏朵腑 N 涓?0 鍒?7 涔嬮棿鐨勬暟瀛椼€傚ぇ浜?3 鐨勫€间細鍦ㄧ郴缁熸棩蹇椾腑浜х敓澶ч噺杈撳嚭銆傞粯璁ゅ€间负 debug=1銆?
options=N1,N2,N3,...

  鍒楄〃涓殑姣忎釜鏁板瓧涓哄搴旂殑缃戝崱鎻愪緵涓€椤归€夐」銆傚洜姝わ紝濡傛灉浣犳湁涓ゅ潡 3c905 骞跺笇鏈涙彁渚?
```
    options=0x204,0x204

```

  鍚勪釜閫夐」鐢辫嫢骞蹭綅瀛楁锛坆itfield锛夌粍鎴愶紝鍏跺惈涔夊涓嬶細

  鍙兘鐨勪粙璐ㄧ被鍨嬭缃?
	==	=================================
	0	10baseT
	1	10Mbs AUI
	2	undefined
	3	10base2 (BNC)
	4	100base-TX
	5	100base-FX
	6	MII (Media Independent Interface)
	7	Use default setting from EEPROM
	8       Autonegotiate
	9       External MII
	10      Use default setting from EEPROM
	==	=================================

  鍦ㄤ负 'options' 璁剧疆鐢熸垚鏁板€兼椂锛屼笂杩颁粙璐ㄩ€夋嫨鍊煎彲浠?OR锛堟垨鐩稿姞锛変笂浠ヤ笅鍊硷細

  ======  =============================================
  0x8000  Set driver debugging level to 7
  0x4000  Set driver debugging level to 2
  0x0400  Enable Wake-on-LAN
  0x0200  Force full duplex mode.
  0x0010  Bus-master enable bit (Old Vortex cards only)
  ======  =============================================

  渚嬪::

    insmod 3c59x options=0x204

  灏嗗己鍒朵娇鐢ㄥ叏鍙屽伐 100base-TX锛岃€屼笉鏄厑璁搁€氬父鐨勮嚜鍔ㄥ崗鍟嗐€?
global_options=N

  涓烘満鍣ㄤ腑鎵€鏈?3c59x NIC 璁剧疆 `options` 鍙傛暟銆備笂杩?`options` 鏁扮粍涓殑鏉＄洰灏嗚鐩栨璁剧疆銆?
full_duplex=N1,N2,N3...

  绫讳技浜?'options' 鐨勭 9 浣嶃€傚皢瀵瑰簲缃戝崱寮哄埗涓哄叏鍙屽伐妯″紡銆傝浼樺厛浣跨敤姝ゅ弬鏁拌€岄潪 `options` 鍙傛暟銆?
  浜嬪疄涓婏紝璇峰敖閲忎笉瑕佷娇鐢ㄥ畠锛佷綘鏈€濂借鑷姩鍗忓晢姝ｅ父宸ヤ綔銆?
global_full_duplex=N1

  涓烘満鍣ㄤ腑鎵€鏈?3c59x NIC 璁剧疆鍏ㄥ弻宸ユā寮忋€備笂杩?`full_duplex` 鏁扮粍涓殑鏉＄洰灏嗚鐩栨璁剧疆銆?
flow_ctrl=N1,N2,N3...

  浣跨敤 802.3x MAC 灞傛祦鎺с€?com 缃戝崱浠呮敮鎸?PAUSE 鍛戒护锛屽嵆濡傛灉鏀跺埌鏉ヨ嚜閾捐矾瀵圭鐨?PAUSE 甯э紝瀹冧滑浼氬仠姝㈠彂閫佹暟鎹寘涓€灏忔鏃堕棿銆?
  椹卞姩浠呭厑璁稿湪鍏ㄥ弻宸ユā寮忕殑閾捐矾涓婂惎鐢ㄦ祦鎺с€?
  璇ュ姛鑳藉湪 3c905 涓婁技涔庝笉璧蜂綔鐢ㄢ€斺€斾粎娴嬭瘯杩?3c905B 鍜?3c905C銆?
  3com 缃戝崱浼间箮浠呭搷搴斿彂閫佸埌淇濈暀鐩殑鍦板潃 01:80:c2:00:00:01 鐨?PAUSE 甯с€傚畠浠笉鍝嶅簲鍙戦€佸埌绔欑偣 MAC 鍦板潃鐨?PAUSE 甯с€?
rx_copybreak=M

  椹卞姩棰勫垎閰?32 涓叏灏哄锛?536 瀛楄妭锛夌綉缁滅紦鍐插尯鐢ㄤ簬鎺ユ敹銆傚綋鏁版嵁鍖呭埌杈炬椂锛岄┍鍔ㄩ渶瑕佸喅瀹氭槸灏嗘暟鎹寘鐣欏湪鍏ㄥ昂瀵哥紦鍐插尯涓紝杩樻槸鍒嗛厤涓€涓緝灏忕殑缂撳啿鍖哄苟灏嗘暟鎹寘澶嶅埗杩囧幓銆?
  杩欐槸閫熷害/绌洪棿涔嬮棿鐨勬潈琛°€?
  rx_copybreak 鐨勫€肩敤浜庡喅瀹氫綍鏃惰繘琛屽鍒躲€傚鏋滄暟鎹寘澶у皬灏忎簬 rx_copybreak锛屽垯澶嶅埗璇ユ暟鎹寘銆俽x_copybreak 鐨勯粯璁ゅ€间负 200 瀛楄妭銆?
max_interrupt_work=N

  椹卞姩鐨勪腑鏂湇鍔′緥绋嬪湪涓€娆¤皟鐢ㄤ腑鍙鐞嗚澶氭帴鏀跺拰鍙戦€佹暟鎹寘銆傚畠鍦ㄤ竴涓惊鐜腑瀹屾垚姝ゆ搷浣溿€俶ax_interrupt_work 鐨勫€兼帶鍒朵腑鏂湇鍔′緥绋嬪惊鐜殑娆℃暟銆傞粯璁ゅ€间负 32 娆″惊鐜€傚鏋滆秴杩囪鍊硷紝涓柇鏈嶅姟渚嬬▼灏嗘斁寮冨苟鐢熸垚璀﹀憡淇℃伅鈥渆th0: Too much work in interrupt鈥濄€?
hw_checksums=N1,N2,N3,...

  杈冩柊鐨?3com NIC 鑳藉鍦ㄧ‖浠朵腑鐢熸垚 IPv4銆乀CP 鍜?UDP 鏍￠獙鍜屻€侺inux 鏃╁氨浣跨敤浜?Rx 鏍￠獙鍜屽姛鑳姐€傗€滈浂鎷疯礉鈥濊ˉ涓佽鍒掔敤浜?2.4 鍐呮牳绯诲垪锛屽畠鍏佽浣犲悓鏃朵娇鐢?NIC 鐨?DMA 鍒嗘暎/鑱氶泦锛坰catter/gather锛夊拰鍙戦€佹牎楠屽拰銆?
  椹卞姩琚缃负锛氬湪搴旂敤 zerocopy 琛ヤ竵鍚庯紝鎵€鏈?Tornado 鍜?Cyclone 璁惧灏嗕娇鐢?S/G 鍜?Tx 鏍￠獙鍜屻€?
  鎻愪緵姝ゆā鍧楀弬鏁版槸涓轰簡璁╀綘鑳藉瑕嗙洊璇ュ喅瀹氥€傚鏋滀綘璁や负 Tx 鏍￠獙鍜屽鑷撮棶棰橈紝鍙互浣跨敤 `hw_checksums=0` 绂佺敤璇ュ姛鑳姐€?
  濡傛灉浣犺涓轰綘鐨?NIC 搴斿綋鎵ц Tx 鏍￠獙鍜岃€岄┍鍔ㄦ湭鍚敤瀹冿紝鍙互浣跨敤 `hw_checksums=1` 寮哄埗浣跨敤纭欢 Tx 鏍￠獙鍜屻€?
  椹卞姩浼氬湪鏃ュ織涓褰曚竴鏉′俊鎭紝琛ㄦ槑瀹冩槸鍚︽鍦ㄤ娇鐢ㄧ‖浠跺垎鏁?鑱氶泦鍜岀‖浠?Tx 鏍￠獙鍜屻€?
  鍒嗘暎/鑱氶泦鍜岀‖浠舵牎楠屽拰涓?sendfile() 绯荤粺璋冪敤甯︽潵鏄捐憲鐨勬€ц兘鎻愬崌锛屼絾浼氫娇 send() 鐨勫悶鍚愰噺鐣ユ湁涓嬮檷銆傚鎺ユ敹鏁堢巼娌℃湁褰卞搷銆?
compaq_ioaddr=N,
compaq_irq=N,
compaq_device_id=N

  鈥滅敤浜庤閬?Compaq PCI BIOS32 闂鐨勫彉閲忊€濃€︹€?
watchdog=N

  璁剧疆鏃堕棿闀垮害锛堜互姣涓哄崟浣嶏級锛岃秴杩囪鏃堕棿鍚庡唴鏍稿垽瀹氬彂閫佸櫒宸插崱浣忓苟闇€瑕佸浣嶃€傝繖涓昏鐢ㄤ簬璋冭瘯鐩殑锛屽敖绠″湪鍐茬獊鐜囬潪甯搁珮鐨勫眬鍩熺綉涓婂澶ц鍊煎彲鑳芥湁鐩娿€傞粯璁ゅ€间负 5000锛?.0 绉掞級銆?
enable_wol=N1,N2,N3,...

  涓虹浉鍏虫帴鍙ｅ惎鐢?Wake-on-LAN 鏀寔銆侱onald Becker 鐨?`ether-wake` 搴旂敤绋嬪簭鍙敤浜庡敜閱掓寕璧风殑鏈哄櫒銆?
  鍚屾椂鍚敤 NIC 鐨勭數婧愮鐞嗘敮鎸併€?
global_enable_wol=N

  涓烘満鍣ㄤ腑鎵€鏈?3c59x NIC 璁剧疆 enable_wol 妯″紡銆備笂杩?`enable_wol` 鏁扮粍涓殑鏉＄洰灏嗚鐩栨璁剧疆銆?
### 浠嬭川閫夋嫨


涓€浜涜緝鏃х殑 NIC锛屽 3c590 鍜?3c900 绯诲垪锛屽叿鏈?10base2 鍜?AUI 鎺ュ彛銆?
鍦?2001 骞?1 鏈堜箣鍓嶏紝濡傛灉鍦?10baseT 绔彛涓婃湭妫€娴嬪埌娲诲姩锛岃椹卞姩浼氳嚜鍔ㄩ€夋嫨 10base2 鎴?AUI 绔彛銆傞殢鍚庡畠浼氬崱鍦?10base2 绔彛涓婏紝蹇呴』閲嶆柊鍔犺浇椹卞姩鎵嶈兘鍒囧洖 10baseT銆傝繖绉嶈涓烘棤娉曢€氳繃妯″潡閫夐」瑕嗙洊鏉ラ樆姝€?
杈冩柊锛堝綋鍓嶏級鐗堟湰鐨勯┍鍔?纭疄*鏀寔閿佸畾浠嬭川绫诲瀷銆傚洜姝わ紝濡傛灉浣犱娇鐢ㄤ互涓嬪懡浠ゅ姞杞介┍鍔ㄦā鍧楋細

	modprobe 3c59x options=0

瀹冨皢姘镐箙閫夋嫨 10baseT 绔彛銆備笉浼氳嚜鍔ㄩ€夋嫨鍏朵粬浠嬭川绫诲瀷銆?

### 鍙戦€侀敊璇紝Tx 鐘舵€佸瘎瀛樺櫒 82


杩欐槸涓€涓父瑙侀敊璇紝鍑犱箮鎬绘槸鐢卞悓涓€缃戠粶涓婄殑鍙︿竴鍙颁富鏈哄浜庡叏鍙屽伐妯″紡銆佽€屾湰鏈哄浜庡崐鍙屽伐妯″紡瀵艰嚧銆備綘闇€瑕佹壘鍒伴偅鍙颁富鏈哄苟浣垮叾杩愯鍦ㄥ崐鍙屽伐妯″紡锛屾垨鑰呭皢鏈満淇涓哄叏鍙屽伐妯″紡銆?
浣滀负鏈€鍚庣殑鎵嬫锛屼綘鍙互浣跨敤浠ヤ笅鍛戒护灏?3c59x 椹卞姩寮哄埗涓哄叏鍙屽伐妯″紡锛?
	options 3c59x full_duplex=1

浣嗚繖搴旇瑙嗕负閽堝鎹熷潖缃戠粶璁惧鐨勫彉閫氬姙娉曪紝搴斾粎鐢ㄤ簬鏃犳硶鑷姩鍗忓晢鐨勮澶囥€?

### 闄勫姞璧勬簮


璁惧椹卞姩瀹炵幇缁嗚妭浣嶄簬婧愭枃浠堕《閮ㄣ€?
鍙湪 Don Becker 鐨?Linux 椹卞姩绔欑偣鑾峰彇棰濆鏂囨。锛?
     http://www.scyld.com/vortex.html

Donald Becker 鐨勯┍鍔ㄥ紑鍙戠珯鐐癸細

     http://www.scyld.com/network.html

Donald 鐨?vortex-diag 绋嬪簭鍙敤浜庢鏌?NIC 鐘舵€侊細

     http://www.scyld.com/ethercard_diag.html

Donald 鐨?mii-diag 绋嬪簭鍙敤浜庢鏌ュ拰鎿嶄綔 NIC 鐨勪粙璐ㄦ棤鍏虫帴鍙ｏ紙Media Independent Interface锛夊瓙绯荤粺锛?
     http://www.scyld.com/ethercard_diag.html#mii-diag

Donald 鐨?wake-on-LAN 椤甸潰锛?
     http://www.scyld.com/wakeonlan.html

3Com 鐢ㄤ簬璁剧疆 NIC EEPROM 鐨勫熀浜?DOS 鐨勫簲鐢ㄧ▼搴忥細

	ftp://ftp.3com.com/pub/nic/3c90x/3c90xx2.exe


### 鑷姩鍗忓晢璇存槑


  椹卞姩浣跨敤涓€鍒嗛挓鐨勫績璺虫潵閫傚簲澶栭儴灞€鍩熺綉鐜鐨勫彉鍖栵細閾捐矾 UP 鏃朵负璇ュ€硷紝閾捐矾 DOWN 鏃朵负 5 绉掋€傝繖鎰忓懗鐫€锛屼緥濡傦紝褰撲竴鍙版満鍣ㄤ粠闆嗙嚎寮?10baseT 灞€鍩熺綉鎷斾笅銆佹彃鍏ヤ氦鎹㈠紡 100baseT 灞€鍩熺綉鏃讹紝鍚炲悙閲忓湪闀胯揪鍏崄绉掑唴浼氱浉褰撶碂绯曘€傝鑰愬績绛夊緟銆?
  Walter Wong <wcw+@CMU.EDU> 鎻愪緵鐨?Cisco 浜掓搷浣滄€ц鏄庯細

  闄勫甫璇存槑锛屾坊鍔?HAS_NWAY 浼间箮涓?Cisco 6509 浜ゆ崲鏈哄瓨鍦ㄥ叡鍚岄棶棰樸€傚叿浣撴潵璇达紝浣犻渶瑕佸皢鏈哄櫒鎵€鎻掔鍙ｇ殑鐢熸垚鏍戝弬鏁版洿鏀逛负 'portfast' 妯″紡銆傚惁鍒欏崗鍟嗕細澶辫触銆傝繖鏄垜浠敞鎰忎簡涓€娈垫椂闂翠絾涓€鐩存病鏃堕棿杩芥煡鐨勯棶棰樸€?
  Cisco 浜ゆ崲鏈猴紙Jeff Busch <jbusch@deja.com>锛?
```
	interface FastEthernet0/N
	description machinename
	load-interval 30
	spanning-tree portfast

```

    濡傛灉鑷姩鍗忓晢鏈夐棶棰橈紝浣犲彲鑳借繕闇€瑕佹寚瀹?"speed 100" 鍜?"duplex full"锛堟垨 "speed 10" 鍜?"duplex half"锛夈€?
    WARNING: DO NOT hook up hubs/switches/bridges to these
    specially-configured ports! The switch will become very confused.


### 鎶ュ憡涓庤瘖鏂棶棰?

缁存姢鑰呭彂鐜帮紝鍑嗙‘鑰屽畬鏁寸殑闂鎶ュ憡瀵逛簬瑙ｅ喅椹卞姩闂闈炲父瀹濊吹銆傛垜浠粡甯告棤娉曞鐜伴棶棰橈紝蹇呴』渚濋潬浣犵殑鑰愬績鍜屽姫鍔涙潵鏌ユ槑闂鏍规簮銆?
濡傛灉浣犺涓洪亣鍒颁簡椹卞姩闂锛屽簲閲囧彇浠ヤ笅涓€浜涙楠わ細

- 杩欑湡鐨勬槸椹卞姩闂鍚楋紵

   鎺掗櫎涓€浜涘彉閲忥細灏濊瘯涓嶅悓鐨勭綉鍗°€佷笉鍚岀殑璁＄畻鏈恒€佷笉鍚岀殑绾跨紗銆佷氦鎹㈡満/闆嗙嚎鍣ㄤ笂鐨勪笉鍚岀鍙ｃ€佷笉鍚岀増鏈殑鍐呮牳鎴栭┍鍔ㄧ瓑銆?
- 濂界殑锛屾槸椹卞姩闂銆?
   浣犻渶瑕佺敓鎴愪竴浠芥姤鍛娿€傞€氬父杩欐槸鍙戦€佺粰缁存姢鑰呭拰/鎴?netdev@vger.kernel.org 鐨勭數瀛愰偖浠躲€傜淮鎶よ€呯殑鐢靛瓙閭欢鍦板潃鍙湪椹卞姩婧愮爜鎴?MAINTAINERS 鏂囦欢涓壘鍒般€?
- 鎶ュ憡鐨勫唴瀹逛細鍥犻棶棰樿€屾湁寰堝ぇ宸紓銆傚鏋滄槸鍐呮牳宕╂簝锛屽垯搴斿弬鑰?'Documentation/admin-guide/reporting-issues.rst'銆?
  浣嗗浜庡ぇ澶氭暟闂锛屾彁渚涗互涓嬪唴瀹瑰緢鏈夌敤锛?
   - 鍐呮牳鐗堟湰銆侀┍鍔ㄧ増鏈?
   - 椹卞姩鍒濆鍖栨椂鐢熸垚鐨勬í骞呬俊鎭殑鍓湰銆備緥濡傦細

     eth0: 3Com PCI 3c905C Tornado at 0xa400,  00:50:da:6a:88:f0, IRQ 19
     8K byte-wide RAM 5:3 Rx:Tx split, autoselect/Autonegotiate interface.
     MII transceiver found at address 24, status 782d.
     Enabling bus-master transmits and whole-frame receives.

     娉ㄦ剰锛氫綘蹇呴』鎻愪緵 `debug=2` 鐨?modprobe 閫夐」鎵嶈兘鐢熸垚

```
	modprobe 3c59x debug=2
```

   - 濡傛灉鏄?PCI 璁惧锛屾彁渚涙潵鑷?'lspci -vx' 鐨勭浉鍏宠緭鍑猴紝渚嬪锛?
```
       00:09.0 Ethernet controller: 3Com Corporation 3c905C-TX [Fast Etherlink] (rev 74)
	       Subsystem: 3Com Corporation: Unknown device 9200
	       Flags: bus master, medium devsel, latency 32, IRQ 19
	       I/O ports at a400 [size=128]
	       Memory at db000000 (32-bit, non-prefetchable) [size=128]
	       Expansion ROM at <unassigned> [disabled] [size=128K]
	       Capabilities: [dc] Power Management version 2
       00: b7 10 00 92 07 00 10 02 74 00 00 02 08 20 00 00
       10: 01 a4 00 00 00 00 00 db 00 00 00 00 00 00 00 00
       20: 00 00 00 00 00 00 00 00 00 00 00 00 b7 10 00 10
       30: 00 00 00 00 dc 00 00 00 00 00 00 00 05 01 0a 0a
```

   - 鐜鎻忚堪锛?0baseT锛?00baseT锛熷叏/鍗婂弻宸ワ紵浜ゆ崲寮忚繕鏄泦绾垮紡锛?
   - 浣犲彲鑳藉悜椹卞姩鎻愪緵鐨勪换浣曢澶栨ā鍧楀弬鏁般€?
   - 浜х敓鐨勪换浣曞唴鏍告棩蹇椼€傝秺澶氳秺濂姐€傚鏋滆繖鏄竴涓ぇ鏂囦欢涓斾綘瑕佸皢鎶ュ憡鍙戦€佺粰閭欢鍒楄〃锛岃璇存槑浣犳湁璇ユ棩蹇楁枃浠讹紝浣嗕笉瑕佸彂閫佸畠銆傚鏋滀綘鏄洿鎺ュ悜缁存姢鑰呮姤鍛婏紝鍒欑洿鎺ュ彂閫佸嵆鍙€?
     涓虹‘淇濇墍鏈夊唴鏍告棩蹇楅兘鍙敤锛岃灏嗕互涓嬭娣诲姞鍒?/etc/syslog.conf锛?
```
	 kern.* /var/log/messages
```

     鐒跺悗閲嶅惎 syslogd锛?
```
	 /etc/rc.d/init.d/syslog restart
```

     锛堜笂杩板唴瀹瑰彲鑳藉洜浣犱娇鐢ㄧ殑 Linux 鍙戣鐗堣€屽紓锛夈€?
    - 濡傛灉浣犵殑闂鍙鐜帮紝閭ｅ氨澶ソ浜嗐€傝灏濊瘯浠ヤ笅鎿嶄綔锛?
      1) 鎻愰珮璋冭瘯绾у埆銆傞€氬父閫氳繃浠ヤ笅鏂瑰紡瀹屾垚锛?
	 a) modprobe driver debug=7
	 b) 鍦?/etc/modprobe.d/driver.conf 涓細
	    options driver debug=7

      2) 浠ユ洿楂樼殑璋冭瘯绾у埆澶嶇幇闂锛屽皢鎵€鏈夋棩蹇楀彂閫佺粰缁存姢鑰呫€?
      3) 浠?Donald Becker 鐨勭綉绔?<http://www.scyld.com/ethercard_diag.html> 涓嬭浇浣犵綉鍗＄殑璇婃柇宸ュ叿銆傚悓鏃朵笅杞?mii-diag.c 骞剁紪璇戝畠浠€?
	 a) 鍦ㄧ綉鍗″伐浣滄甯告椂杩愯 'vortex-diag -aaee' 鍜?'mii-diag -v'銆備繚瀛樿緭鍑恒€?
	 b) 鍦ㄧ綉鍗″嚭鐜版晠闅滄椂杩愯涓婅堪鍛戒护銆傚彂閫佷袱缁勮緭鍑恒€?
鏈€鍚庯紝璇蜂繚鎸佽€愬績骞跺仛濂藉噯澶囧仛涓€浜涘伐浣溿€傞殢鐫€缁存姢鑰呮彁鍑烘洿澶氶棶棰樸€佽姹傛洿澶氭祴璇曘€佽姹傚簲鐢ㄨˉ涓佺瓑锛屼綘鏈€缁堝彲鑳戒細涓烘闂宸ヤ綔涓€鍛ㄦ垨鏇撮暱鏃堕棿銆傚埌澶存潵锛岄棶棰樼敋鑷冲彲鑳戒粛鐒舵病鏈夊緱鍒拌В鍐炽€?