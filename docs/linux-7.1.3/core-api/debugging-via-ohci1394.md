## 鍒╃敤 OHCI-1394 鐏嚎锛團ireWire锛夋帶鍒跺櫒鎻愪緵鐨勭墿鐞?DMA 杩涜璋冭瘯


### 绠€浠嬶紙Introduction锛?

鍩烘湰涓婏紝褰撲粖浣跨敤鐨勬墍鏈夌伀绾挎帶鍒跺櫒閮界鍚?OHCI-1394 瑙勮寖锛岃瑙勮寖灏嗘帶鍒跺櫒瀹氫箟涓轰竴涓?PCI 鎬荤嚎涓昏澶囷紝瀹冧娇鐢?DMA 灏嗘暟鎹紶杈撲粠 CPU 涓婂嵏杞斤紝骞舵嫢鏈変竴涓€滅墿鐞嗗搷搴斿崟鍏冿紙Physical Response Unit锛夆€濓紝璇ュ崟鍏冨湪搴旂敤鐢?OHCI-1394 椹卞姩瀹氫箟鐨勮繃婊ゅ櫒涔嬪悗锛岄€氳繃 PCI 鎬荤嚎涓?DMA 鎵ц鐗瑰畾璇锋眰銆?
涓€鏃︽纭厤缃紝杩滅▼鏈哄櫒灏卞彲浠ュ彂閫佽繖浜涜姹傦紝瑕佹眰 OHCI-1394 鎺у埗鍣ㄥ鐗╃悊绯荤粺鍐呭瓨鎵ц璇诲拰鍐欒姹傦紝骞朵笖瀵逛簬璇昏姹傦紝灏嗙墿鐞嗗唴瀛樿鍙栫殑缁撴灉鍙戦€佸洖璇锋眰鏂广€?
鐢辨锛屽彲浠ラ€氳繃璇诲彇璇稿 printk 缂撳啿鍖烘垨杩涚▼琛ㄤ箣绫荤殑缂撳啿鍖虹瓑鏈夎叮鐨勫唴瀛樹綅缃潵璋冭瘯闂銆?
涔熷彲浠ラ€氳繃鐏嚎鑾峰彇瀹屾暣鐨勭郴缁熷唴瀛樿浆鍌紝鏁版嵁浼犺緭閫熺巼鍙揪 10MB/s 鎴栨洿楂樸€?
瀵逛簬澶у鏁扮伀绾挎帶鍒跺櫒锛屽唴瀛樿闂闄愬埗鍦ㄧ墿鐞嗗湴鍧€绌洪棿鐨勪綆 4 GB銆傚湪鍐呭瓨涓昏浣嶄簬璇ラ檺鍒朵箣涓婄殑鏈哄櫒涓婅繖鍙兘鎴愪负闂锛屼絾鍦?x86銆亁86-64 鍜?PowerPC 绛夋洿甯歌鐨勭‖浠朵笂寰堝皯鎴愪负闂銆?
宸茬煡鑷冲皯 LSI FW643e 鍜?FW643e2 鎺у埗鍣ㄦ敮鎸佽闂?4 GB 浠ヤ笂鐨勭墿鐞嗗湴鍧€锛屼絾 Linux 鐩墠灏氭湭鍚敤姝ゅ姛鑳姐€?
閰嶅悎 OHCI-1394 鎺у埗鍣ㄧ殑鏃╂湡鍒濆鍖栫敤浜庤皟璇曪紝璇ヨ鏂借璇佹槑瀵逛簬妫€鏌?printk 缂撳啿鍖轰腑鍐楅暱鐨勮皟璇曟棩蹇楁渶涓烘湁鐢紝浠ヨ皟璇?ACPI 绛夊尯鍩熶腑绯荤粺鏃犳硶鍚姩鐨勬棭鏈熷紩瀵奸棶棰橈紝鑰屽叾浠栬皟璇曟墜娈碉紙涓插彛锛夎涔堜笉鍙敤锛堢瑪璁版湰锛夛紝瑕佷箞瀵逛簬澶ч噺璋冭瘯淇℃伅锛堝 ACPI锛夎€岃█澶參銆?
### 椹卞姩锛圖rivers锛?

drivers/firewire 涓殑 firewire-ohci 椹卞姩榛樿浣跨敤缁忚繃杩囨护鐨勭墿鐞?DMA锛岃繖鏇村畨鍏ㄤ絾涓嶉€傚悎杩滅▼璋冭瘯銆傚悜璇ラ┍鍔ㄤ紶閫?remote_dma=1 鍙傛暟浠ヨ幏寰楁湭缁忚繃婊ょ殑鐗╃悊 DMA銆?
鐢变簬 firewire-ohci 椹卞姩渚濊禆浜?PCI 鏋氫妇鐨勫畬鎴愶紝鍥犳宸蹭负 x86 瀹炵幇浜嗕竴涓繍琛屽緱鐩稿綋鏃╃殑鍒濆鍖栦緥绋嬨€傝渚嬬▼鍦?console_init() 鑳藉琚皟鐢ㄤ箣鍓嶅緢涔呭氨杩愯锛屽嵆鍦?printk 缂撳啿鍖哄嚭鐜板湪鎺у埗鍙颁箣鍓嶃€?
瑕佹縺娲诲畠锛岃鍚敤 CONFIG_PROVIDE_OHCI1394_DMA_INIT锛圞ernel hacking 鑿滃崟锛歊emote debugging over FireWire early on boot锛夛紝骞跺湪寮曞鏃跺悜閲嶆柊缂栬瘧鐨勫唴鏍镐紶閫掑弬鏁?"ohci1394_dma=early"銆?
### 宸ュ叿锛圱ools锛?

firescope - 鏈€鍒濈敱 Benjamin Herrenschmidt 寮€鍙戯紝Andi Kleen 灏嗗叾浠?PowerPC 绉绘鍒?x86 鍜?x86_64 骞舵坊鍔犱簡鍔熻兘锛宖irescope 鐜板湪鍙敤浜庢煡鐪嬭繙绋嬫満鍣ㄧ殑 printk 缂撳啿鍖猴紝鐢氳嚦鏀寔瀹炴椂鏇存柊銆?
Bernhard Kaindl 澧炲己浜?firescope锛屼互鏀寔浠?32 浣?firescope 璁块棶 64 浣嶆満鍣紝鍙嶄箣浜︾劧锛?- http://v3.sk/~lkundrak/firescope/

骞朵笖浠栧疄鐜颁簡蹇€熺郴缁熻浆鍌紙alpha 鐗堟湰 - 璇烽槄璇?README.txt锛夛細
- http://halobates.de/firewire/firedump-0.1.tar.bz2

杩樻湁涓€涓敤浜庣伀绾跨殑 gdb 浠ｇ悊锛屽厑璁镐娇鐢?gdb 璁块棶鍙粠 gdb 鍦?vmlinux 涓壘鍒扮殑绗﹀彿鎵€寮曠敤鐨勬暟鎹細
- http://halobates.de/firewire/fireproxy-0.33.tar.bz2

姝?gdb 浠ｇ悊鐨勬渶鏂扮増鏈紙fireproxy-0.34锛夊彲浠ラ€氳繃涓€涓熀浜庡唴瀛樼殑閫氫俊妯″潡锛坘gdbom锛変笌 kgdb 閫氫俊锛堝皻涓嶇ǔ瀹氾級銆?
### 寮€濮嬩娇鐢紙Getting Started锛?

OHCI-1394 瑙勮寖瑙勫畾锛孫HCI-1394 鎺у埗鍣ㄥ繀椤诲湪姣忔鎬荤嚎澶嶄綅鏃剁鐢ㄦ墍鏈夌墿鐞?DMA銆?
杩欐剰鍛崇潃锛屽鏋滀綘鎯冲湪绯荤粺澶勪簬涓柇琚鐢ㄣ€佷笖涓嶅 OHCI-1394 鎺у埗鍣ㄨ繘琛屾€荤嚎澶嶄綅杞鐨勭姸鎬佷笅璋冭瘯鏌愪釜闂锛屼綘蹇呴』鍦ㄧ郴缁熻繘鍏ヨ繖绉嶇姸鎬乢_涔嬪墠__寤虹珛浠讳綍鐏嚎鐢电紗杩炴帴骞跺畬鍏ㄥ垵濮嬪寲鎵€鏈夌伀绾跨‖浠躲€?
浣跨敤 firescope 閰嶅悎鏃╂湡 OHCI 鍒濆鍖栫殑鍒嗘璇存槑锛?
1) 楠岃瘉浣犵殑纭欢鍙楁敮鎸侊細

   鍔犺浇 firewire-ohci 妯″潡骞舵鏌ヤ綘鐨勫唴鏍告棩蹇椼€?```
     firewire_ohci 0000:15:00.1: added OHCI v1.0 device as card 2, 4 IR + 4 IT
     ... contexts, quirks 0x11
```
   鍔犺浇椹卞姩鏃躲€傚鏋滀綘娌℃湁鍙楁敮鎸佺殑鎺у埗鍣紝璁稿瀹屽叏绗﹀悎 OHCI-1394 瑙勮寖鐨?PCI銆丆ardBus 鐢氳嚦鏌愪簺 Express 鍗￠兘鍙敤銆傚鏋滃畠涓嶉渶瑕?Windows 鎿嶄綔绯荤粺鐨勯┍鍔紝閭ｅ畠寰堝彲鑳藉氨鏄€傚彧鏈変笓闂ㄧ殑鍟嗗簵鎵嶆湁涓嶇鍚堣鑼冪殑鍗★紝瀹冧滑鍩轰簬 TI PCILynx 鑺墖骞堕渶瑕?Windows 鎿嶄綔绯荤粺鐨勯┍鍔ㄣ€?
   涓婅堪鍐呮牳鏃ュ織娑堟伅鍖呭惈瀛楃涓?"physUB"锛屽鏋滆鎺у埗鍣ㄥ疄鐜颁簡鍙啓鐨勭墿鐞嗕笂鐣岋紙Physical Upper Bound锛夊瘎瀛樺櫒銆傝繖鏄?4 GB 浠ヤ笂鐗╃悊 DMA 鎵€蹇呴渶鐨勶紙浣?Linux 灏氭湭浣跨敤锛夈€?
2) 寤虹珛鍙敤鐨勭伀绾跨數缂嗚繛鎺ワ細

   浠讳綍鐏嚎鐢电紗锛屽彧瑕佹彁渚涚數姘斿拰鏈烘涓婄ǔ瀹氱殑杩炴帴骞跺叿鏈夊尮閰嶇殑鎺ュご锛堟湁灏忓瀷 4 閽堝拰澶у瀷 6 閽堢伀绾跨鍙ｏ級鍗冲彲銆?
```
     firewire_core 0000:15:00.1: created device fw1: GUID 00061b0020105917, S400
```
   褰撶數缂嗘彃鍏ュ苟杩炴帴涓ゅ彴鏈哄櫒鏃讹紝涓ゅ彴鏈哄櫒鐨勫唴鏍告棩蹇椾腑閮戒細鍑虹幇銆?
3) 浣跨敤 firescope 娴嬭瘯鐗╃悊 DMA锛?
   鍦ㄨ皟璇曚富鏈轰笂锛岀‘淇?/dev/fw* 鍙闂紝
```
	$ firescope
	Port 0 (/dev/fw1) opened, 2 nodes detected

	FireScope
	---------
	Target : <unspecified>
	Gen    : 1
	[Ctrl-T] choose target
	[Ctrl-H] this menu
	[Ctrl-Q] quit

    ------> 鐜板湪鎸?Ctrl-T锛岃緭鍑哄簲绫讳技濡備笅锛?
	2 nodes available, local node is: 0
	 0: ffc0, uuid: 00000000 00000000 [LOCAL]
	 1: ffc1, uuid: 00279000 ba4bb801

   闄や簡 [LOCAL] 鑺傜偣澶栵紝瀹冨繀椤绘棤閿欒鍦版樉绀哄彟涓€涓妭鐐广€?
```
4) 涓洪厤鍚堟棭鏈?OHCI-1394 鍒濆鍖栬繘琛岃皟璇曞仛鍑嗗锛?
   4.1) 鍦ㄨ皟璇曠洰鏍囦笂缂栬瘧骞跺畨瑁呭唴鏍?
   缂栬瘧瑕佽皟璇曠殑鍐呮牳锛屽苟鍚敤 CONFIG_PROVIDE_OHCI1394_DMA_INIT锛圞ernel hacking锛歅rovide code for enabling DMA over FireWire early on boot锛夛紝鐒跺悗灏嗗叾瀹夎鍒拌璋冭瘯鐨勬満鍣紙璋冭瘯鐩爣锛変笂銆?
   4.2) 灏嗗彈璋冭瘯鍐呮牳鐨?System.map 浼犺緭鍒拌皟璇曚富鏈?
   灏嗗彈璋冭瘯鍐呮牳鐨?System.map 澶嶅埗鍒拌皟璇曚富鏈猴紙鍗抽€氳繃鐏嚎鐢电紗杩炴帴鍒板彈璋冭瘯鏈哄櫒鐨勪富鏈猴級銆?
5) 鑾峰彇 printk 缂撳啿鍖哄唴瀹癸細

   鍦ㄧ伀绾跨數缂嗗凡杩炴帴銆佽皟璇曚富鏈轰笂宸插姞杞?OHCI-1394 椹卞姩鐨勬儏鍐典笅锛岄噸鏂板惎鍔ㄥ彈璋冭瘯鏈哄櫒锛屽紩瀵煎惎鐢ㄤ簡 CONFIG_PROVIDE_OHCI1394_DMA_INIT 鐨勫唴鏍革紝骞朵娇鐢ㄩ€夐」 ohci1394_dma=early銆?
```
	firescope -A System.map-of-debug-target-kernel
```
   娉ㄦ剰锛?A 浼氳嚜鍔ㄨ繛鎺ュ埌绗竴涓潪鏈湴鑺傜偣銆傚畠浠呭湪浠呴€氳繃鐏嚎杩炴帴涓ゅ彴鏈哄櫒鏃舵墠鍙潬宸ヤ綔銆?
   杩炴帴鍒拌皟璇曠洰鏍囧悗锛屾寜 Ctrl-D 鏌ョ湅瀹屾暣鐨?printk 缂撳啿鍖猴紝鎴栨寜 Ctrl-U 杩涘叆鑷姩鏇存柊妯″紡锛岃幏鍙栧彈璋冭瘯鐩爣涓婅褰曠殑鏈€杩戝唴鏍告秷鎭殑瀹炴椂瑙嗗浘銆?
   璋冪敤 "firescope -h" 鍙幏鍙栨湁鍏?firescope 閫夐」鐨勬洿澶氫俊鎭€?
### 澶囨敞锛圢otes锛?

鏂囨。鍜岃鑼冿細http://halobates.de/firewire/

FireWire 鏄?Apple Inc. 鐨勫晢鏍?- 鏇村淇℃伅璇峰弬闃咃細
https://en.wikipedia.org/wiki/FireWire
