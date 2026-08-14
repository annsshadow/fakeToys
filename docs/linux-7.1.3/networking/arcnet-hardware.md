锘?
## ARCnet 纭欢



:Author: Avery Pennarun <apenwarr@worldvisions.ca>


   1) 鏈枃浠舵槸 arcnet.rst 鐨勮ˉ鍏呫€傛湁鍏抽€氱敤鐨勯┍鍔ㄩ厤缃府鍔╋紝璇烽槄璇昏鏂囦欢銆?
   2) 鏈枃浠跺凡涓嶅啀鐗瑰畾浜?Linux銆傚畠鎴栬搴旇浠庡唴鏍告簮鐮佷腑绉诲嚭銆傛湁鎯虫硶鍚楋紵

鐢变簬浼间箮鏈夊緢澶氫汉锛堝寘鎷垜锛夋嬁鍒颁簡娌℃湁鎵嬪唽鐨?ARCnet 缃戝崱锛屾湰鏂囦欢鍖呭惈浜嗗 ARCnet 纭欢鐨勫揩閫熶粙缁嶃€佷竴浜涘竷绾挎彁绀猴紝浠ュ強鎴戣兘鎵惧埌鐨勬墍鏈夎烦绾胯缃垪琛ㄣ€傚鏋滀綘鏈夐拡瀵硅嚜宸辩壒瀹氱綉鍗＄殑浠讳綍璁剧疆锛屽拰/鎴栦换浣曞叾浠栦俊鎭紝璇烽殢鏃跺彂閭欢缁?netdev <arcnet-netdev>銆?


## ARCnet 绠€浠?



ARCnet 鏄竴绉嶇綉缁滅被鍨嬶紝鍏跺伐浣滄柟寮忕被浼间簬娴佽鐨?Ethernet 缃戠粶锛屼絾涔熷瓨鍦ㄤ竴浜涢潪甯搁噸瑕佺殑宸紓銆?

棣栧厛锛屼綘鍙互涔板埌鑷冲皯涓ょ閫熷害鐨?ARCnet 缃戝崱锛?.5 Mbps锛堟瘮 Ethernet 鎱級鍜?100 Mbps锛堟瘮鏅€?Ethernet 蹇級銆備簨瀹炰笂杩樻湁鍏朵粬閫熷害锛屼絾涓嶅お甯歌銆傛嵁鎴戞墍鐭ワ紝涓嶅悓鐨勭‖浠剁被鍨嬩箣闂翠簰涓嶅吋瀹癸紝鍥犳浣犱笉鑳藉皢 100 Mbps 缃戝崱鎺ュ埌 2.5 Mbps 缃戝崱涓婏紝渚濇绫绘帹銆傛嵁鎴戞墍闂伙紝鎴戠殑椹卞姩纭疄鍙互閰嶅悎 100 Mbps 缃戝崱宸ヤ綔锛屼絾鎴戣嚜宸辨棤娉曢獙璇佽繖涓€鐐癸紝鍥犱负鎴戝彧鏈?2.5 Mbps 杩欎竴绉嶃€傚畠澶ф鏃犳硶璁╀綘鐨?100 Mbps 缃戝崱璺戞弧銆傚埆鍐嶆姳鎬ㄤ簡銆?:)

浣犱篃鏃犳硶灏?ARCnet 缃戝崱杩炴帴鍒颁换浣曠绫荤殑 Ethernet 缃戝崱骞舵寚鏈涘畠鑳藉伐浣溿€?

ARCnet 鏈変袱绉嶁€滅被鍨嬧€濃€斺€旀槦鍨嬶紙STAR锛夋嫇鎵戝拰鎬荤嚎鍨嬶紙BUS锛夋嫇鎵戙€傝繖鎸囩殑鏄綉鍗″簲璇ュ浣曡繛鎺ュ湪涓€璧枫€傛牴鎹ぇ澶氭暟鍙敤鏂囨。锛屼綘鍙兘灏?STAR 缃戝崱杩炴帴鍒?STAR 缃戝崱锛屽皢 BUS 缃戝崱杩炴帴鍒?BUS 缃戝崱銆傝繖璇村緱閫氾紝瀵瑰惂锛熷棷锛岃繖骞朵笉瀹屽叏姝ｇ‘锛涜涓嬫枃鈥滃竷绾库€濅竴鑺傘€?

涓€鏃﹁法杩囪繖浜涘皬闅滅锛孉RCnet 瀹為檯涓婃槸涓€涓浉褰撶簿蹇冭璁＄殑鏍囧噯銆傚畠浣跨敤浜嗕竴绉嶇О涓衡€滄敼杩涚殑浠ょ墝浼犻€掞紙modified token passing锛夆€濈殑鏈哄埗锛岃繖浣垮叾涓庢墍璋撶殑鈥滀护鐗岀幆锛圱oken Ring锛夆€濈綉鍗″畬鍏ㄤ笉鍏煎锛屼絾涔熶娇鍏朵紶杈撴瘮 Ethernet 鍙潬寰楀銆備簨瀹炰笂锛孉RCnet 浼氫繚璇佹暟鎹寘瀹夊叏鍒拌揪鐩殑鍦帮紝鍗充娇瀹冩棤娉曟纭€佽揪锛堜緥濡傜敱浜庣數缂嗘柇瑁傦紝鎴栫洰鏍囪绠楁満涓嶅瓨鍦級锛屽畠涔熻嚦灏戜細閫氱煡鍙戦€佹柟銆?

鐢变簬鈥滀护鐗屸€濈殑鍔ㄤ綔缁忚繃绮剧‘瀹氫箟锛屽畠鎬讳細鍦ㄦ渶澶ф椂闀垮唴缁曗€滅幆鈥濅紶閫掍竴鍦堛€傝繖浣垮叾瀵瑰疄鏃剁綉缁滃緢鏈夌敤銆?

姝ゅ锛屾墍鏈夊凡鐭ョ殑 ARCnet 缃戝崱閮芥湁锛堝嚑涔庯級鐩稿悓鐨勭紪绋嬫帴鍙ｃ€傝繖鎰忓懗鐫€鐢ㄤ竴涓?ARCnet 椹卞姩灏辫兘鏀寔浠讳綍缃戝崱锛岃€?Ethernet 鍒欐瘡瀹跺埗閫犲晢浣跨敤鐨勬湁鏃舵槸瀹屽叏涓嶅悓鐨勭紪绋嬫帴鍙ｏ紝瀵艰嚧鍑虹幇澶ч噺涓嶅悓銆佹湁鏃跺張闈炲父鐩镐技鐨?Ethernet 椹卞姩銆傚綋鐒讹紝濮嬬粓浣跨敤鐩稿悓鐨勭紪绋嬫帴鍙ｄ篃鎰忓懗鐫€锛屽綋 PCI 鎬荤嚎涓绘帶 DMA 杩欑被楂樻€ц兘纭欢鐗规€у嚭鐜版椂锛屽緢闅惧姞浠ュ埄鐢ㄣ€傝繖涓垜浠氨涓嶅睍寮€璁ㄨ浜嗐€?

涓嶈繃锛屼娇 ARCnet 缃戝崱闅句互缂栫▼鐨勪竴鐐规槸鍏舵暟鎹寘澶у皬鐨勯檺鍒讹紱鏍囧噯 ARCnet 鍙兘鍙戦€侀暱搴︿笉瓒呰繃 508 瀛楄妭鐨勬暟鎹寘銆傝繖灏忎簬 Internet鈥滄渶浣庤姹傗€濈殑 576 瀛楄妭锛屾洿涓嶇敤璇?Ethernet 1500 瀛楄妭鐨?MTU 浜嗐€備綔涓鸿ˉ鍋匡紝RFC1201 瀹氫箟浜嗛澶栫殑涓€灞傚皝瑁咃紙鎴戠О涔嬩负鈥滄暟鎹寘鎷嗗垎鈥濓級锛屽畠鍏佽鈥滆櫄鎷熸暟鎹寘鈥濆澶у埌姣忎釜鏈€澶?64K锛屽敖绠″畠浠€氬父淇濇寔鍦?Ethernet 椋庢牸鐨?1500 瀛楄妭銆?

鏈夊叧 ARCnet 缃戠粶鐨勬洿澶氫俊鎭紝璇疯闂€淎RCNET Resource Center鈥漌WW 椤甸潰锛?

	https://www.arcnet.cc


## ARCnet 缃戠粶甯冪嚎



鏈妭鐢变互涓嬩汉鍛橀噸鍐欙細

	Vojtech Pavlik     <vojtech@suse.cz>

浣跨敤浜嗗浜虹殑淇℃伅锛屽寘鎷細

 - Avery Pennraun     <apenwarr@worldvisions.ca>
 - Stephen A. Wood    <saw@hallc1.cebaf.gov>
 - John Paul Morrison <jmorriso@bogomips.ee.ubc.ca>
 - Joachim Koenig     <jojo@repas.de>

搴?Vojtech 鐨勮姹傦紝Avery 鍙堝鍏跺仛浜嗕竴浜涙鼎鑹层€?

ARCnet锛堢粡鍏哥殑 2.5 Mbps 鐗堟湰锛夊彲浠ラ€氳繃涓ょ涓嶅悓鐨勭數缂嗚繛鎺ワ細鍚岃酱鐢电紗鍜屽弻缁炵嚎銆傚叾浠?ARCnet 绫诲瀷鐨勭綉缁滐紙100 Mbps TCNS 浠ュ強 320 kbps - 32 Mbps 鐨?ARCnet Plus锛変娇鐢ㄤ笉鍚岀被鍨嬬殑鐢电紗锛圱ype1銆佸厜绾ゃ€丆1銆丆4銆丆5锛夈€?

瀵逛簬鍚岃酱鐢电紗缃戠粶锛屼綘鈥滃簲璇モ€濅娇鐢?93 娆у RG-62 鐢电紗銆備絾鍏朵粬鐢电紗涔熻兘姝ｅ父宸ヤ綔锛屽洜涓?ARCnet 鏄竴绉嶉潪甯哥ǔ瀹氱殑缃戠粶銆傛垜涓汉浣跨敤鐨勬槸 75 娆у鐢佃澶╃嚎鐢电紗銆?

鐢ㄤ簬鍚岃酱鐢电紗甯冪嚎鐨勭綉鍗℃湁涓ょ鍙樹綋锛氬垎鍒敤浜?BUS 鍜?STAR 缃戠粶鎷撴墤銆傚畠浠ぇ浣撶浉鍚屻€傚敮涓€鐨勫尯鍒湪浜庢墍瀹夎鐨勬贩鍚堣姱鐗囥€侭US 缃戝崱浣跨敤楂橀樆鎶楄緭鍑猴紝鑰?STAR 浣跨敤浣庨樆鎶椼€備綆闃绘姉缃戝崱锛圫TAR锛夊湪鐢垫皵涓婄瓑鍚屼簬鎺ヤ簡缁堢鐢甸樆鐨勯珮闃绘姉缃戝崱銆?

閫氬父锛孉RCnet 缃戠粶鐢?STAR 缃戝崱鍜岄泦绾垮櫒锛坔ub锛夋瀯鎴愩€傞泦绾垮櫒鏈変袱绉嶇被鍨嬧€斺€旀湁婧愬拰鏃犳簮銆傛棤婧愰泦绾垮櫒鏄皬鐩掑瓙

```

	   |         | wires
	   R         + junction
	-R-+-R-      R 47 Ohm resistors
	   R
	   |

```

灞忚斀灞傝繛鎺ュ湪涓€璧枫€傛湁婧愰泦绾垮櫒瑕佸鏉傚緱澶氾紱瀹冧滑鏈夌數婧愶紝骞跺寘鍚敤浜庢斁澶т俊鍙峰苟灏嗗叾鍙戦€佸埌缃戠粶鍏朵粬缃戞鐨勭數瀛愬厓浠躲€傚畠浠€氬父鏈夊叓涓繛鎺ュ櫒銆傛湁婧愰泦绾垮櫒鏈変袱绉嶅彉浣撯€斺€斿搼锛坉umb锛夊拰鏅鸿兘锛坰mart锛夈€傚搼鍙樹綋鍙槸鏀惧ぇ淇″彿锛岃€屾櫤鑳藉彉浣撲細灏嗙粡杩囩殑鎵€鏈夋暟鎹寘瑙ｇ爜涓烘暟瀛楀啀閲嶆柊缂栫爜銆傚鏋滀綘鍦ㄧ綉缁滀腑鏈夊涓泦绾垮櫒锛岃繖绉嶆柟寮忚濂藉緱澶氾紝鍥犱负澶氫釜鍝戞湁婧愰泦绾垮櫒鍙兘浼氶檷浣庝俊鍙疯川閲忋€?

鐜板湪鏉ヨ璇村竷绾裤€備綘鍙互灏嗕互涓嬭澶囪繛鎺ュ湪涓€璧凤細

1. 缃戝崱瀵圭綉鍗°€傝繖鏄粍寤哄弻鏈虹綉缁滄渶绠€鍗曠殑鏂瑰紡銆?

2. 缃戝崱瀵规棤婧愰泦绾垮櫒銆傝浣忥紝闆嗙嚎鍣ㄤ笂鎵€鏈夋湭浣跨敤鐨勮繛鎺ュ櫒閮藉繀椤荤敤 93 娆у锛堝鏋滄病鏈夊悎閫傜殑锛屼篃鍙互鐢ㄥ叾浠栭樆鍊硷級鐨勭粓绔數闃绘纭鎺ャ€?

	锛圓very 娉細鍝庡憖锛屾垜褰撴椂涓嶇煡閬撹繖鐐广€備笉杩囨垜鐨勶紙鐢佃鐢电紗锛夌収鏍疯兘鐢ㄣ€傦級

3. 缃戝崱瀵规湁婧愰泦绾垮櫒銆傝繖閲屾棤闇€绔帴鏈娇鐢ㄧ殑杩炴帴鍣紝闄ら潪鍑轰簬鏌愮缇庤鑰冭檻銆備絾鏄紝浠绘剰涓ゅ彴璁＄畻鏈轰箣闂翠笉鑳芥湁瓒呰繃鍗佷竴涓殑鏈夋簮闆嗙嚎鍣ㄣ€傝繖褰撶劧涓嶉檺鍒剁綉缁滀腑鏈夋簮闆嗙嚎鍣ㄧ殑鎬绘暟銆?

4. 鏈夋簮闆嗙嚎鍣ㄥ鍙︿竴涓湁婧愰泦绾垮櫒銆?

5. 鏈夋簮闆嗙嚎鍣ㄥ鏃犳簮闆嗙嚎鍣ㄣ€?

璁颁綇锛屼綘涓嶈兘灏嗕袱涓棤婧愰泦绾垮櫒杩炴帴鍦ㄤ竴璧枫€傝繖绉嶈繛鎺ュ鑷寸殑鍔熺巼鎹熻€楄繃楂橈紝缃戠粶鏃犳硶鍙潬杩愯銆?


```

	   R                     S - STAR type card
    S------H--------A-------S    R - Terminator
	   |        |            H - Hub
	   |        |            A - Active hub
	   |   S----H----S
	   S        |
		    |
		    S

```

BUS 鎷撴墤涓?Ethernet 鎵€浣跨敤鐨勯潪甯哥浉浼笺€傚敮涓€鐨勫尯鍒湪浜庣數缂嗗拰缁堢鐢甸樆锛氬畠浠簲涓?93 娆у銆侲thernet 浣跨敤 50 娆у闃绘姉銆備綘浣跨敤 T 鍨嬭繛鎺ュ櫒灏嗚绠楁満鎺ュ叆鍗曟牴鐢电紗锛堝嵆鎬荤嚎锛夈€備綘蹇呴』鍦ㄦ€荤嚎鐨勪袱绔兘鎺ヤ笂缁堢鐢甸樆

```

    RT----T------T------T------T------TR
     B    B      B      B      B      B

  B - BUS type card
  R - Terminator
  T - T connector

```

浣嗚繖杩樹笉鏄叏閮紒杩欎袱绉嶇被鍨嬪彲浠ヨ繛鎺ュ湪涓€璧枫€傛牴鎹畼鏂规枃妗ｏ紝杩炴帴瀹冧滑鐨勫敮涓€鏂规硶鏄娇鐢ㄤ竴涓湁婧?

```

	 A------T------T------TR
	 |      B      B      B
     S---H---S
	 |
	 S

```

瀹樻柟鏂囨。杩樻寚鍑猴紝浣犲彲浠ュ湪鈥︹€︾殑鏈浣跨敤 STAR 缃戝崱

```

     S------T------T------S
	    B      B

```

涓嶈繃锛屾牴鎹垜鑷繁鐨勫疄楠岋紝浣犲彲浠ョ洿鎺ュ湪 STAR 鎷撴墤缃戠粶鐨勭數缂嗕腑闂翠换鎰忎綅缃寕涓€涓?BUS 绫诲瀷缃戝崱銆傛洿杩涗竴姝モ€斺€斿鏋滀綘浣跨敤缁堢鐢甸樆锛岃繕鍙互鐢ㄦ€荤嚎缃戝崱鏇夸唬浠讳綍鏄熷瀷缃戝崱銆傝繖鏍蜂綘灏辫兘鏋勫缓鍑烘弧瓒虫墍鏈夐渶姹傜殑闈炲父澶嶆潅鐨勭綉缁滐紒涓€涓?

```

				  S
				  |
	   RT------T-------T------H------S
	    B      B       B      |
				  |       R
    S------A------T-------T-------A-------H------TR
	   |      B       B       |       |      B
	   |   S                 BT       |
	   |   |                  |  S----A-----S
    S------H---A----S             |       |
	   |   |      S------T----H---S   |
	   S   S             B    R       S

```

鍙岀粸绾垮竷绾块噰鐢ㄤ簡涓€绉嶅熀鏈笉鍚岀殑甯冪嚎鏂规銆傛瘡涓?TP 缃戝崱鏈変袱涓?RJ锛堢數璇濈嚎椋庢牸锛夎繛鎺ュ櫒銆傜劧鍚庤繖浜涚綉鍗￠€氳繃杩炴帴鐩搁偦涓ゅ紶缃戝崱鐨勭數缂嗛灏句覆鑱旓紙daisy-chain锛夊湪涓€璧枫€備袱绔娇鐢?RJ 93 娆у缁堢鐢甸樆绔帴锛屽畠浠彃鍏?

```

	  ___________   ___________
      _R_|_         _|_|_         _|_R_
     |     |       |     |       |     |
     |Card |       |Card |       |Card |
     |_____|       |_____|       |_____|


```

TP 鎷撴墤涔熸湁闆嗙嚎鍣ㄣ€備娇鐢ㄥ畠浠苟涓嶅洶闅撅紱浣犲彧闇€灏?TP 閾捐繛鎺ュ埌闆嗙嚎鍣ㄧ殑浠绘剰涓€绔紝鐢氳嚦涓ょ閮借繛銆傝繖鏍蜂綘灏辫兘鍒涘缓鍑犱箮浠绘剰鐨勭綉缁滈厤缃€傜綉缁滀腑浠绘剰涓ゅ彴璁＄畻鏈轰箣闂存渶澶?11 涓泦绾垮櫒鐨勯檺鍒跺湪姝ゅ悓鏍烽€傜敤

```

    RP-------P--------P--------H-----P------P-----PR
			       |
      RP-----H--------P--------H-----P------PR
	     |                 |
	     PR                PR

    R - RJ Terminator
    P - TP Card
    H - TP Hub

```

涓庝换浣曠綉缁滀竴鏍凤紝ARCnet 鐨勭數缂嗛暱搴︽湁闄愩€備互涓嬫槸涓や釜鏈夋簮绔紙鏈夋簮绔寚鏈夋簮闆嗙嚎鍣ㄦ垨 STAR 缃戝崱锛変箣闂寸殑鏈€澶х數缂嗛暱搴︺€?

		========== ======= ===========
		RG-62       93 Ohm up to 650 m
		RG-59/U     75 Ohm up to 457 m
		RG-11/U     75 Ohm up to 533 m
		IBM Type 1 150 Ohm up to 200 m
		IBM Type 3 100 Ohm up to 100 m
		========== ======= ===========

杩炴帴鍒版棤婧愰泦绾垮櫒鐨勬墍鏈夌數缂嗙殑鏈€澶ч暱搴﹀浜?RG-62 甯冪嚎闄愬埗涓?65 绫筹紱鍏朵粬鐢电紗鏇寸煭銆備綘鍙互鐪嬪埌锛屽湪澶у瀷缃戠粶涓娇鐢ㄦ棤婧愰泦绾垮櫒鏄釜绯熺硶鐨勪富鎰忋€傚崟鏍光€淏US 骞茬嚎锛圔US Trunk锛夆€濈殑鏈€澶ч暱搴﹀浜?RG-62 绾︿负 300 绫炽€傜綉缁滀腑鏈€杩滀袱鐐逛箣闂寸殑鏈€澶ц窛绂婚檺鍒朵负 3000 绫炽€備袱寮犵綉鍗?闆嗙嚎鍣ㄤ箣闂?TP 鐢电紗鐨勬渶澶ч暱搴︿负 650 绫炽€?


## 璁剧疆璺崇嚎



鎵€鏈?ARCnet 缃戝崱鎬诲叡搴旀湁鍥涘埌浜旂涓嶅悓璁剧疆锛?

  - I/O 鍦板潃锛氳繖鏄綘鐨?ARCnet 缃戝崱鎵€鍦ㄧ殑鈥滅鍙ｂ€濄€侺inux ARCnet 椹卞姩涓帰娴嬬殑鍊间粎鍦?0x200 鍒?0x3F0 涔嬮棿銆傦紙濡傛灉浣犵殑缃戝崱杩樻湁鍏朵粬鍊硷紝杩欐槸鍙兘鐨勶紝璇峰憡璇夋垜銆傦級瀹冧笉搴斾笌绯荤粺涓婁换浣曞叾浠栬澶囩浉鍚屻€傛牴鎹垜浠?Novell 鎷垮埌鐨勪竴浠芥枃妗ｏ紝MS Windows 鍋忓ソ 0x300 鎴栨洿澶х殑鍊硷紝鍚﹀垯浼氬悶鎺夋垜绯荤粺涓婄殑缃戠粶杩炴帴锛堣嚦灏戞槸杩欐牱锛夈€傛垜鐚滆繖鍙兘鏄洜涓猴紝濡傛灉浣犵殑缃戝崱浣嶄簬 0x2E0锛屽 0x2E8 澶勪覆琛岀鍙ｇ殑鎺㈡祴浼氶噸缃缃戝崱锛屽苟寰堝彲鑳芥妸浜嬫儏鎼炲緱涓€鍥㈢碂銆?

 - Avery 鐨勬渶鐖憋細0x300銆?

  - IRQ锛氬湪 8 浣嶇綉鍗′笂锛屽畠鍙兘鏄?2 (9)銆?銆?銆? 鎴?7銆?
	     鍦?16 浣嶇綉鍗′笂锛屽畠鍙兘鏄?2 (9)銆?銆?銆?銆? 鎴?10-15銆?

    纭繚瀹冧笌绯荤粺涓婁换浣曞叾浠栫綉鍗￠兘涓嶅悓銆傛敞鎰忥紝灏?Linux 鑰岃█锛孖RQ2 涓?IRQ9 鏄浉鍚岀殑銆備綘鍙互鈥渃at /proc/interrupts鈥濊幏寰椾竴浠芥煇鏃跺埢鍝簺 IRQ 姝ｅ湪浣跨敤鐨勮緝涓哄畬鏁寸殑鍒楄〃銆備互涓嬫槸 Vojtech Pavlik <vojtech@suse.cz> 鎻愪緵鐨勫父瑙佺敤閫斿垪琛細

	锛堚€淣ot on bus鈥濊〃绀虹綉鍗℃棤娉曚骇鐢熸涓柇锛?

	======   =========================================================
	IRQ  0   Timer 0 (Not on bus)
	IRQ  1   Keyboard (Not on bus)
	IRQ  2   IRQ Controller 2 (Not on bus, nor does interrupt the CPU)
	IRQ  3   COM2
	IRQ  4   COM1
	IRQ  5   FREE (LPT2 if you have it; sometimes COM3; maybe PLIP)
	IRQ  6   Floppy disk controller
	IRQ  7   FREE (LPT1 if you don't use the polling driver; PLIP)
	IRQ  8   Realtime Clock Interrupt (Not on bus)
	IRQ  9   FREE (VGA vertical sync interrupt if enabled)
	IRQ 10   FREE
	IRQ 11   FREE
	IRQ 12   FREE
	IRQ 13   Numeric Coprocessor (Not on bus)
	IRQ 14   Fixed Disk Controller
	IRQ 15   FREE (Fixed Disk Controller 2 if you have it)
	======   =========================================================



```

	   IRQ 9 is used on some video cards for the "vertical retrace"
	   interrupt.  This interrupt would have been handy for things like
	   video games, as it occurs exactly once per screen refresh, but
	   unfortunately IBM cancelled this feature starting with the original
	   VGA and thus many VGA/SVGA cards do not support it.  For this
	   reason, no modern software uses this interrupt and it can almost
	   always be safely disabled, if your video card supports it at all.

	If your card for some reason CANNOT disable this IRQ (usually there
	is a jumper), one solution would be to clip the printed circuit
	contact on the board: it's the fourth contact from the left on the
	back side.  I take no responsibility if you try this.

	- Avery's favourite: IRQ2 (actually IRQ9).  Watch that VGA, though.

  - the memory address:  Unlike most cards, ARCnets use "shared memory" for
    copying buffers around.  Make SURE it doesn't conflict with any other
    used memory in your system!

    ::

	A0000		- VGA graphics memory (ok if you don't have VGA)
	B0000		- Monochrome text mode
	C0000		\  One of these is your VGA BIOS - usually C0000.
	E0000		/
	F0000		- System BIOS

    Anything less than 0xA0000 is, well, a BAD idea since it isn't above
    640k.

	- Avery's favourite: 0xD0000

  - the station address:  Every ARCnet card has its own "unique" network
    address from 0 to 255.  Unlike Ethernet, you can set this address
    yourself with a jumper or switch (or on some cards, with special
    software).  Since it's only 8 bits, you can only have 254 ARCnet cards
    on a network.  DON'T use 0 or 255, since these are reserved (although
    neat stuff will probably happen if you DO use them).  By the way, if you
    haven't already guessed, don't set this the same as any other ARCnet on
    your network!

	- Avery's favourite:  3 and 4.  Not that it matters.

  - There may be ETS1 and ETS2 settings.  These may or may not make a
    difference on your card (many manuals call them "reserved"), but are
    used to change the delays used when powering up a computer on the
    network.  This is only necessary when wiring VERY long range ARCnet
    networks, on the order of 4km or so; in any case, the only real
    requirement here is that all cards on the network with ETS1 and ETS2
    jumpers have them in the same position.  Chris Hindy <chrish@io.org>
    sent in a chart with actual values for this:

	======= ======= =============== ====================
	ET1	ET2	Response Time	Reconfiguration Time
	======= ======= =============== ====================
	open	open	74.7us		840us
	open	closed	283.4us		1680us
	closed	open	561.8us		1680us
	closed	closed	1118.6us	1680us
	======= ======= =============== ====================

    Make sure you set ETS1 and ETS2 to the SAME VALUE for all cards on your
    network.

```

姝ゅ锛岃澶氱綉鍗′笂锛堣櫧鐒朵笉鏄垜鐨勶級鏈夌孩鑹插拰缁胯壊 LED銆俈ojtech Pavlik <vojtech@suse.cz> 鍛婅瘔鎴戝畠浠殑鍚箟锛?

	=============== =============== =====================================
	GREEN           RED             Status
	=============== =============== =====================================
	OFF             OFF             Power off
	OFF             Short flashes   Cabling problems (broken cable or not
					terminated)
	OFF (short)     ON              Card init
	ON              ON              Normal state - everything OK, nothing
					happens
	ON              Long flashes    Data transfer
	ON              OFF             Never happens (maybe when wrong ID)
	=============== =============== =====================================


浠ヤ笅鏄汉浠彂缁欐垜鐨勫叧浜庝粬浠悇鑷壒瀹?ARCnet 缃戝崱鐨勫叏閮ㄥ叿浣撲俊鎭€傚畠绠€鐩存槸涓€鍥㈢碂锛屽寘鍚ぇ閲忛噸澶嶄俊鎭€傛垜娌℃椂闂村幓鏁寸悊瀹冦€傚鏋滀綘鎯虫暣鐞嗭紝璇峰姟蹇呭姩鎵嬶紒鍙渶鎶婁綘鎵€鍋氭洿鏀圭殑鈥渄iff -u鈥濆彂缁欐垜鍗冲彲銆?

鍨嬪彿 # 鍒楀湪璇ョ綉鍗″叿浣撹鏄庣殑姝ｄ笂鏂癸紝鍥犳浣犲簲璇ヨ兘澶熶娇鐢ㄦ枃鏈煡鐪嬪櫒鐨勨€渟earch鈥濆姛鑳芥壘鍒颁綘鎯宠鐨勬潯鐩€傚鏋滀綘涓嶇煡閬撹嚜宸辨嫢鏈変綍绉嶇綉鍗★紝璇曠潃缈荤湅鍚勭鍥剧ず锛岀湅鐪嬭兘鍚﹁鲸璁ゅ嚭鏉ャ€?

濡傛灉浣犵殑鍨嬪彿娌℃湁鍒楀嚭鍜?鎴栬缃笉鍚岋紝璇峰姟蹇呭憡璇夋垜銆傛垜涓嶅緱涓嶅湪娌℃湁鎵嬪唽鐨勬儏鍐典笅鑷繁鐞㈢（鍑烘潵锛岄偅鍙竴鐐归兘涓嶅ソ鐜╋紒

鍗充娇浣犵殑 ARCnet 鍨嬪彿娌℃湁鍒楀嚭锛屼絾璺崇嚎涓庡彟涓€涓凡鍒楀嚭鐨勫瀷鍙风浉鍚岋紝涔熻鍙戦偖浠跺憡璇夋垜銆?

鏈枃浠朵腑鍒楀嚭鐨勭綉鍗★紙澶ц嚧鎸夋椤哄簭锛夛細

	=============== ======================= ====
	Manufacturer	Model #			Bits
	=============== ======================= ====
	SMC		PC100			8
	SMC		PC110			8
	SMC		PC120			8
	SMC		PC130			8
	SMC		PC270E			8
	SMC		PC500			16
	SMC		PC500Longboard		16
	SMC		PC550Longboard		16
	SMC		PC600			16
	SMC		PC710			8
	SMC?		LCS-8830(-T)		8/16
	Puredata	PDI507			8
	CNet Tech	CN120-Series		8
	CNet Tech	CN160-Series		16
	Lantech?	UM9065L chipset		8
	Acer		5210-003		8
	Datapoint?	LAN-ARC-8		8
	Topware		TA-ARC/10		8
	Thomas-Conrad	500-6242-0097 REV A	8
	Waterloo?	(C)1985 Waterloo Micro. 8
	No Name		--			8/16
	No Name		Taiwan R.O.C?		8
	No Name		Model 9058		8
	Tiara		Tiara Lancard?		8
	=============== ======================= ====


- SMC = Standard Microsystems Corp锛堟爣鍑嗗井绯荤粺鍏徃锛夈€?
- CNet Tech = CNet Technology, Inc.锛圕Net 绉戞妧鍏徃锛夈€?

## 鏈垎绫诲唴瀹?



  - 璇峰彂閫佷綘鑳芥壘鍒扮殑浠讳綍鍏朵粬淇℃伅銆?


```

     From: root@ultraworld.xs4all.nl (Timo Hilbrink)
     To: apenwarr@foxnet.net (Avery Pennarun)
     Date: Wed, 26 Oct 1994 02:10:32 +0000 (GMT)
     Reply-To: timoh@xs4all.nl

     [...parts deleted...]

     About the jumpers: On my PC130 there is one more jumper, located near the
     cable-connector and it's for changing to star or bus topology;
     closed: star - open: bus
     On the PC500 are some more jumper-pins, one block labeled with RX,PDN,TXI
     and another with ALE,LA17,LA18,LA19 these are undocumented..

     [...more parts deleted...]

     --- CUT ---

```

## 鏍囧噯寰郴缁熷叕鍙革紙SMC锛?



### PC100銆丳C110銆丳C120銆丳C130锛? 浣嶇綉鍗★級浠ュ強 PC500銆丳C600锛?6 浣嶇綉鍗★級



  - 涓昏鏉ヨ嚜 Avery Pennarun <apenwarr@worldvisions.ca>銆傛墍绀烘暟鍊煎彇鑷?Avery 鐨勯厤缃€?
  - 鐗瑰埆鎰熻阿 Timo Hilbrink <timoh@xs4all.nl> 鎸囧嚭 PC120銆?30銆?00 鍜?600 涓?Avery 鐨?PC100 鍏锋湁鐩稿悓鐨勫紑鍏炽€備笉杩?PC500/600 鏈夊嚑涓澶栫殑銆佹湭鏂囨。鍖栫殑寮曡剼銆傦紙)
  - PC110 鐨勮缃凡鐢?Stephen A. Wood <saw@cebaf.gov> 楠岃瘉
  - 鍙﹀锛孞P- 鍜?S- 缂栧彿鍙兘涓庝綘鐨勭綉鍗′笉瀹屽叏瀵瑰簲銆傝瘯鐫€瀵绘壘鍏锋湁鍚屾牱鏁伴噺璁剧疆鐨勮烦绾?寮€鍏斥€斺€旇繖鏍峰彲鑳芥洿鍙潬銆?


```

	     JP5		       [|]    :    :    :    :
	(IRQ Setting)		      IRQ2  IRQ3 IRQ4 IRQ5 IRQ7
			Put exactly one jumper on exactly one set of pins.


				  1  2   3  4  5  6   7  8  9 10
	     S1                /----------------------------------\
	(I/O and Memory        |  1  1 * 0  0  0  0 * 1  1  0  1  |
	 addresses)            \----------------------------------/
				  |--|   |--------|   |--------|
				  (a)       (b)           (m)

			WARNING.  It's very important when setting these which way
			you're holding the card, and which way you think is '1'!

			If you suspect that your settings are not being made
			correctly, try reversing the direction or inverting the
			switch positions.

			a: The first digit of the I/O address.
				Setting		Value
				-------		-----
				00		0
				01		1
				10		2
				11		3

			b: The second digit of the I/O address.
				Setting		Value
				-------		-----
				0000		0
				0001		1
				0010		2
				...		...
				1110		E
				1111		F

			The I/O address is in the form ab0.  For example, if
			a is 0x2 and b is 0xE, the address will be 0x2E0.

			DO NOT SET THIS LESS THAN 0x200!!!!!


			m: The first digit of the memory address.
				Setting		Value
				-------		-----
				0000		0
				0001		1
				0010		2
				...		...
				1110		E
				1111		F

			The memory address is in the form m0000.  For example, if
			m is D, the address will be 0xD0000.

			DO NOT SET THIS TO C0000, F0000, OR LESS THAN A0000!

				  1  2  3  4  5  6  7  8
	     S2                /--------------------------\
	(Station Address)      |  1  1  0  0  0  0  0  0  |
			       \--------------------------/

				Setting		Value
				-------		-----
				00000000	00
				10000000	01
				01000000	02
				...
				01111111	FE
				11111111	FF

			Note that this is binary with the digits reversed!

			DO NOT SET THIS TO 0 OR 255 (0xFF)!


```

### PC130E/PC270E锛? 浣嶇綉鍗★級



  - 鏉ヨ嚜 Juergen Seifert <seifert@htwm.de>

鏈弿杩扮敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁浠ヤ笅鍘熷 SMC 鎵嬪唽鎾板啓

	     "Configuration Guide for ARCNET(R)-PC130E/PC270 Network
	     Controller Boards Pub. # 900.044A June, 1989"

ARCnet 鏄?Datapoint Corporation 鐨勬敞鍐屽晢鏍?
SMC 鏄?Standard Microsystems Corporation 鐨勬敞鍐屽晢鏍?

PC130E 鏄?PC130 鏉垮崱鐨勫寮虹増鏈紝閰嶅鏍囧噯鐨?BNC 姣嶅骇杩炴帴鍣紝鐢ㄤ簬杩炴帴 RG-62/U 鍚岃酱鐢电紗銆傜敱浜庤鏉垮崱鏃㈣璁＄敤浜庢槦鍨嬬綉缁滀腑鐨勭偣鍒扮偣杩炴帴锛屼篃璁捐鐢ㄤ簬鎬荤嚎缃戠粶杩炴帴锛屽洜姝ゅ畠鍚戜笅鍏煎鎵€鏈変负鍚岃酱缃戠粶璁捐鐨勫叾浠栨爣鍑嗘澘鍗★紙鍗?PC120銆丳C110 鍜?PC100 鏄熷瀷鎷撴墤鏉垮崱锛屼互鍙?PC220銆丳C210 鍜?PC200 鎬荤嚎鎷撴墤鏉垮崱锛夈€?

PC270E 鏄?PC260 鏉垮崱鐨勫寮虹増鏈紝閰嶅涓や釜妯″潡鍖栫殑 RJ11 鍨嬫彃瀛旓紝鐢ㄤ簬杩炴帴鍙岀粸绾垮竷绾裤€傚畠鍙敤浜庢槦鍨嬬綉缁滄垨鑿婅姳閾剧綉缁溿€?


```

	 8 7 6 5 4 3 2 1
    ________________________________________________________________
   |   |       S1        |                                          |
   |   |_________________|                                          |
   |    Offs|Base |I/O Addr                                         |
   |     RAM Addr |                                              ___|
   |         ___  ___                                       CR3 |___|
   |        |   \/   |                                      CR4 |___|
   |        |  PROM  |                                           ___|
   |        |        |                                        N |   | 8
   |        | SOCKET |                                        o |   | 7
   |        |________|                                        d |   | 6
   |                   ___________________                    e |   | 5
   |                  |                   |                   A | S | 4
   |       |oo| EXT2  |                   |                   d | 2 | 3
   |       |oo| EXT1  |       SMC         |                   d |   | 2
   |       |oo| ROM   |      90C63        |                   r |___| 1
   |       |oo| IRQ7  |                   |               |o|  _____|
   |       |oo| IRQ5  |                   |               |o| | J1  |
   |       |oo| IRQ4  |                   |              STAR |_____|
   |       |oo| IRQ3  |                   |                   | J2  |
   |       |oo| IRQ2  |___________________|                   |_____|
   |___                                               ______________|
       |                                             |
       |_____________________________________________|

```
```

  SMC 90C63	ARCNET Controller / Transceiver /Logic
  S1	1-3:	I/O Base Address Select
	4-6:	Memory Base Address Select
	7-8:	RAM Offset Select
  S2	1-8:	Node ID Select
  EXT		Extended Timeout Select
  ROM		ROM Enable Select
  STAR		Selected - Star Topology	(PC130E only)
		Deselected - Bus Topology	(PC130E only)
  CR3/CR4	Diagnostic LEDs
  J1		BNC RG62/U Connector		(PC130E only)
  J1		6-position Telephone Jack	(PC270E only)
  J2		6-position Telephone Jack	(PC270E only)

```

灏嗘煇涓紑鍏宠涓?Off/Open 琛ㄧず鈥?鈥濓紝On/Closed 琛ㄧず鈥?鈥濄€?


##### 璁剧疆鑺傜偣 ID



缁?S2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繖浜涘紑鍏崇殑宸ヤ綔鏂瑰紡涓?PC100 绯诲垪缃戝崱绫讳技锛涙洿澶氫俊鎭鍙傞槄璇ユ潯鐩€?


##### 璁剧疆 I/O 鍩哄潃



寮€鍏崇粍 S1 鐨勫墠涓変釜寮€鍏崇敤浜庨€夋嫨鍏朵腑涔嬩竴

```


   Switch | Hex I/O
   1 2 3  | Address
   -------|--------
   0 0 0  |  260
   0 0 1  |  290
   0 1 0  |  2E0  (Manufacturer's default)
   0 1 1  |  2F0
   1 0 0  |  300
   1 0 1  |  350
   1 1 0  |  380
   1 1 1  |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃



鍐呭瓨缂撳啿鍖洪渶瑕?16K RAM 鍧椾腑鐨?2K銆傝繖涓?16K 鍧楃殑鍩哄潃鍙互浣嶄簬鍏釜浣嶇疆涓殑浠绘剰涓€涓€傚紑鍏崇粍 S1 鐨勫紑鍏?4-6 閫夋嫨 16K 鍧楃殑鍩哄潃銆傚湪璇?16K 鍦板潃绌洪棿鍐咃紝缂撳啿鍖哄彲琚垎閰嶅埌鍥涗釜浣嶇疆涓殑浠绘剰涓€涓紝鐢卞亸绉婚噺锛堝紑鍏崇粍 S1 鐨勫紑鍏?7 鍜?8锛夊喅瀹氥€?


```

   Switch     | Hex RAM | Hex ROM
   4 5 6  7 8 | Address | Address *)
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000

  *) To enable the 8K Boot PROM install the jumper ROM.
     The default is jumper ROM not installed.


```

##### 璁剧疆瓒呮椂涓庝腑鏂?



鏍囨湁 EXT1 鍜?EXT2 鐨勮烦绾跨敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜璺崇嚎閫氬父淇濇寔鏂紑锛坥pen锛夈€?

瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇疯缃烦绾?IRQ2銆両RQ3銆両RQ4銆両RQ5銆両RQ7 涓殑涓€涓紙涓斿彧鑳戒竴涓紒锛夈€傚埗閫犲晢榛樿鍊间负 IRQ2銆?


##### 涓?PC130E 閰嶇疆鏄熷瀷鎴栨€荤嚎鍨嬫嫇鎵?



鍗曚釜鏍囨湁 STAR 鐨勮烦绾跨敤浜庝负 PC130E 鏉垮崱閰嶇疆鏄熷瀷鎴栨€荤嚎鍨嬫嫇鎵戙€傚畨瑁呰璺崇嚎鏃讹紝鏉垮崱鍙敤浜庢槦鍨嬬綉缁滐紱绉婚櫎璇ヨ烦绾挎椂锛屾澘鍗″彲鐢ㄤ簬鎬荤嚎鍨嬫嫇鎵戙€?


##### 璇婃柇 LED



鏉垮崱鍚庢尅鏉夸笂鍙涓や釜璇婃柇 LED銆傜豢鑹?LED 鐩戣缃戠粶娲诲姩锛涚孩鑹?LED 鏄剧ず

```

 Green  | Status               Red      | Status
 -------|-------------------   ---------|-------------------
  on    | normal activity      flash/on | data transfer
  blink | reconfiguration      off      | no data transfer;
  off   | defective board or            | incorrect memory or
	| node ID is zero               | I/O address


```

### PC500/PC550 Longboard锛?6 浣嶇綉鍗★級



  - 鏉ヨ嚜 Juergen Seifert <seifert@htwm.de>



```

      There is another Version of the PC500 called Short Version, which
      is different in hard- and software! The most important differences
      are:

      - The long board has no Shared memory.
      - On the long board the selection of the interrupt is done by binary
	coded switch, on the short board directly by jumper.

```

[Avery 娉細璇风壒鍒暀鎰忚繖涓€鐐癸細闀挎澘娌℃湁鍏变韩鍐呭瓨銆傝繖鎰忓懗鐫€褰撳墠鐨?Linux-ARCnet 椹卞姩鏃犳硶浣跨敤杩欎簺缃戝崱銆傛垜宸茬粡寮勫埌涓€鍧?PC500Longboard锛屽皢鏉ヤ細瀵瑰叾鍋氫竴浜涘疄楠岋紝浣嗗埆澶湡寰呫€傚啀娆℃劅璋?Juergen Seifert 鐨勫缓璁紒]

鏈弿杩扮敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁浠ヤ笅鍘熷 SMC 鎵嬪唽鎾板啓

	 "Configuration Guide for SMC ARCNET-PC500/PC550
	 Series Network Controller Boards Pub. # 900.033 Rev. A
	 November, 1989"

ARCnet 鏄?Datapoint Corporation 鐨勬敞鍐屽晢鏍?
SMC 鏄?Standard Microsystems Corporation 鐨勬敞鍐屽晢鏍?

PC500 閰嶅鏍囧噯鐨?BNC 姣嶅骇杩炴帴鍣紝鐢ㄤ簬杩炴帴 RG-62/U 鍚岃酱鐢电紗銆傝鏉垮崱鏃㈣璁＄敤浜庢槦鍨嬬綉缁滀腑鐨勭偣鍒扮偣杩炴帴锛屼篃璁捐鐢ㄤ簬鎬荤嚎缃戠粶杩炴帴銆?

PC550 閰嶅涓や釜妯″潡鍖栫殑 RJ11 鍨嬫彃瀛旓紝鐢ㄤ簬杩炴帴鍙岀粸绾垮竷绾裤€傚畠鍙敤浜庢槦鍨嬬綉缁滄垨鑿婅姳閾撅紙BUS锛夌綉缁溿€?


```

       1
       0 9 8 7 6 5 4 3 2 1     6 5 4 3 2 1
    ____________________________________________________________________
   < |         SW1         | |     SW2     |                            |
   > |_____________________| |_____________|                            |
   <   IRQ    |I/O Addr                                                 |
   >                                                                 ___|
   <                                                            CR4 |___|
   >                                                            CR3 |___|
   <                                                                 ___|
   >                                                              N |   | 8
   <                                                              o |   | 7
   >                                                              d | S | 6
   <                                                              e | W | 5
   >                                                              A | 3 | 4
   <                                                              d |   | 3
   >                                                              d |   | 2
   <                                                              r |___| 1
   >                                                        |o|    _____|
   <                                                        |o|   | J1  |
   >  3 1                                                   JP6   |_____|
   < |o|o| JP2                                                    | J2  |
   > |o|o|                                                        |_____|
   <  4 2__                                               ______________|
   >    |  |                                             |
   <____|  |_____________________________________________|

```
```

  SW1	1-6:	I/O Base Address Select
	7-10:	Interrupt Select
  SW2	1-6:	Reserved for Future Use
  SW3	1-8:	Node ID Select
  JP2	1-4:	Extended Timeout Select
  JP6		Selected - Star Topology	(PC500 only)
		Deselected - Bus Topology	(PC500 only)
  CR3	Green	Monitors Network Activity
  CR4	Red	Monitors Board Activity
  J1		BNC RG62/U Connector		(PC500 only)
  J1		6-position Telephone Jack	(PC550 only)
  J2		6-position Telephone Jack	(PC550 only)

```

灏嗘煇涓紑鍏宠涓?Off/Open 琛ㄧず鈥?鈥濓紝On/Closed 琛ㄧず鈥?鈥濄€?


##### 璁剧疆鑺傜偣 ID



缁?SW3 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐归兘蹇呴』鏈変竴涓敮涓€鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆傚紑鍏?1 浣滀负鏈€浣庢湁鏁堜綅锛圠SB锛夈€?

鑺傜偣 ID 鏄墍鏈夎涓衡€?鈥濈殑寮€鍏冲€间箣鍜?

```

    Switch | Value
    -------|-------
      1    |   1
      2    |   2
      3    |   4
      4    |   8
      5    |  16
      6    |  32
      7    |  64
      8    | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄潃



寮€鍏崇粍 SW1 鐨勫墠鍏釜寮€鍏崇敤浜庨€夋嫨鍏朵腑涔嬩竴

```

   Switch       | Hex I/O
   6 5  4 3 2 1 | Address
   -------------|--------
   0 1  0 0 0 0 |  200
   0 1  0 0 0 1 |  210
   0 1  0 0 1 0 |  220
   0 1  0 0 1 1 |  230
   0 1  0 1 0 0 |  240
   0 1  0 1 0 1 |  250
   0 1  0 1 1 0 |  260
   0 1  0 1 1 1 |  270
   0 1  1 0 0 0 |  280
   0 1  1 0 0 1 |  290
   0 1  1 0 1 0 |  2A0
   0 1  1 0 1 1 |  2B0
   0 1  1 1 0 0 |  2C0
   0 1  1 1 0 1 |  2D0
   0 1  1 1 1 0 |  2E0 (Manufacturer's default)
   0 1  1 1 1 1 |  2F0
   1 1  0 0 0 0 |  300
   1 1  0 0 0 1 |  310
   1 1  0 0 1 0 |  320
   1 1  0 0 1 1 |  330
   1 1  0 1 0 0 |  340
   1 1  0 1 0 1 |  350
   1 1  0 1 1 0 |  360
   1 1  0 1 1 1 |  370
   1 1  1 0 0 0 |  380
   1 1  1 0 0 1 |  390
   1 1  1 0 1 0 |  3A0
   1 1  1 0 1 1 |  3B0
   1 1  1 1 0 0 |  3C0
   1 1  1 1 0 1 |  3D0
   1 1  1 1 1 0 |  3E0
   1 1  1 1 1 1 |  3F0


```

##### 璁剧疆涓柇



寮€鍏崇粍 SW1 鐨勫紑鍏充竷鍒板崄鐢ㄤ簬閫夋嫨涓柇绾у埆銆備腑鏂骇鍒负浜岃繘鍒剁紪鐮侊紝鍥犳鐞嗚涓婂彲閫夋嫨 0 鍒?15锛屼絾鍙敮鎸佷互涓嬪叓涓€硷細3銆?銆?銆?銆?銆?0銆?1銆?2銆?


```

   Switch   | IRQ
   10 9 8 7 |
   ---------|--------
    0 0 1 1 |  3
    0 1 0 0 |  4
    0 1 0 1 |  5
    0 1 1 1 |  7
    1 0 0 1 |  9 (=2) (default)
    1 0 1 0 | 10
    1 0 1 1 | 11
    1 1 0 0 | 12


```

##### 璁剧疆瓒呮椂



涓や釜璺崇嚎 JP2锛?-4锛夌敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜璺崇嚎閫氬父淇濇寔鏂紑锛坥pen锛夈€傛湁鍏虫浛浠ｉ厤缃紝璇峰弬闃?COM9026 鏁版嵁鎵嬪唽銆?


##### 涓?PC500 閰嶇疆鏄熷瀷鎴栨€荤嚎鍨嬫嫇鎵?



鍗曚釜鏍囨湁 JP6 鐨勮烦绾跨敤浜庝负 PC500 鏉垮崱閰嶇疆鏄熷瀷鎴栨€荤嚎鍨嬫嫇鎵戙€傚畨瑁呰璺崇嚎鏃讹紝鏉垮崱鍙敤浜庢槦鍨嬬綉缁滐紱绉婚櫎璇ヨ烦绾挎椂锛屾澘鍗″彲鐢ㄤ簬鎬荤嚎鍨嬫嫇鎵戙€?


##### 璇婃柇 LED



鏉垮崱鍚庢尅鏉夸笂鍙涓や釜璇婃柇 LED銆傜豢鑹?LED 鐩戣缃戠粶娲诲姩锛涚孩鑹?LED 鏄剧ず

```

 Green  | Status               Red      | Status
 -------|-------------------   ---------|-------------------
  on    | normal activity      flash/on | data transfer
  blink | reconfiguration      off      | no data transfer;
  off   | defective board or            | incorrect memory or
	| node ID is zero               | I/O address


```

### PC710锛? 浣嶇綉鍗★級



  - 鏉ヨ嚜 J.S. van Oosten <jvoosten@compiler.tdcnet.nl>

娉ㄦ剰锛氳繖浜涙暟鎹槸閫氳繃瀹為獙骞跺弬鑰冨叾浠栫綉鍗＄殑淇℃伅鏀堕泦鐨勩€備笉杩囷紝鎴戠‘淇℃垜鎼炲浜?99% 鐨勮缃€?

SMC710 缃戝崱绫讳技浜?PC270 缃戝崱锛屼絾瑕佸熀纭€寰楀锛堝嵆娌℃湁

```

    _______________________________________
   | +---------+  +---------+              |____
   | |   S2    |  |   S1    |              |
   | +---------+  +---------+              |
   |                                       |
   |  +===+    __                          |
   |  | R |   |  | X-tal                 ###___
   |  | O |   |__|                      ####__'|
   |  | M |    ||                        ###
   |  +===+                                |
   |                                       |
   |   .. JP1   +----------+               |
   |   ..       | big chip |               |
   |   ..       |  90C63   |               |
   |   ..       |          |               |
   |   ..       +----------+               |
    -------                     -----------
	   |||||||||||||||||||||

```

JP1 澶勭殑璺崇嚎鎺掑疄闄呬笂鐢?8 涓烦绾跨粍鎴愶紝锛堟湁鏃舵爣娉ㄤ负锛変笌 PC270 涓婄浉鍚岀殑锛屼粠涓婂埌涓嬩緷娆′负锛欵XT2銆丒XT1銆丷OM銆両RQ7銆両RQ5銆両RQ4銆両RQ3銆両RQ2锛堝樋锛岀寽鐚滃畠浠槸骞插槢鐢ㄧ殑锛?:-) 锛?

S1 鍜?S2 鐨勫姛鑳戒笌 PC270 涓婄浉鍚岋紝鍙槸缂栧彿浜掓崲浜嗭紙S1 鏄妭鐐瑰湴鍧€锛孲2 璁剧疆 IO 鍜?RAM 鍦板潃锛夈€?

鎴戠煡閬撳畠杩炴帴鍒?PC110 绫诲瀷鐨?ARCnet 鏉垮崱鏃跺彲浠ユ甯稿伐浣溿€?


*****************************************************************************

## 鍙兘涓?SMC



### LCS-8830(-T)锛? 浣嶄笌 16 浣嶇綉鍗★級



  - 鏉ヨ嚜 Mathias Katzer <mkatzer@HRZ.Uni-Bielefeld.DE>
  - Marek Michalkiewicz <marekm@i17linuxb.ists.pwr.wroc.pl> 鎸囧嚭
    LCS-8830 涓?LCS-8830-T 鐣ユ湁涓嶅悓銆傚畠浠槸 8 浣嶃€佷粎鎬荤嚎鍨嬶紙JP0 璺崇嚎涓虹‖杩炵嚎锛夛紝涓斾粎 BNC銆?

杩欐槸鎴戣涓烘槸 SMC 鍒堕€犵殑 LCS-8830-T锛?SMC' 鍙嚭鐜板湪涓€涓?PLCC 涓婏紝鍒閮芥病鏈夛紝杩炴墜鍐岄噷閭ｅ嚑寮犲鍗扮焊涓篃娌℃湁锛夈€?


```

     ------------------------------------
    |                                    |
    |              JP3 88  8 JP2         |
    |       #####      | \               |
    |       #####    ET1 ET2          ###|
    |                              8  ###|
    |  U3   SW 1                  JP0 ###|  Phone Jacks
    |  --                             ###|
    | |  |                               |
    | |  |   SW2                         |
    | |  |                               |
    | |  |  #####                        |
    |  --   #####                       ####  BNC Connector
    |                                   ####
    |   888888 JP1                       |
    |   234567                           |
     --                           -------
       |||||||||||||||||||||||||||
	--------------------------


  SW1: DIP-Switches for Station Address
  SW2: DIP-Switches for Memory Base and I/O Base addresses

  JP0: If closed, internal termination on (default open)
  JP1: IRQ Jumpers
  JP2: Boot-ROM enabled if closed
  JP3: Jumpers for response timeout

  U3: Boot-ROM Socket


  ET1 ET2     Response Time     Idle Time    Reconfiguration Time

		 78                86               840
   X            285               316              1680
       X        563               624              1680
   X   X       1130              1237              1680

  (X means closed jumper)

  (DIP-Switch downwards means "0")

```

绔欏湴鍧€鐢?SW1 浠ヤ簩杩涘埗缂栫爜銆?

I/O 鍩哄潃鐢?SW2 鐨?DIP 寮€鍏?6銆?銆? 缂栫爜锛?

========	========
Switches        Base
678             Address
========	========
000		260-26f
100		290-29f
010		2e0-2ef
110		2f0-2ff
001		300-30f
101		350-35f
011		380-38f
111 		3e0-3ef
========	========


SW2 鐨?DIP 寮€鍏?1-5 缂栫爜 RAM 鍜?ROM 鍦板潃鑼冨洿锛?

========        ============= ================
Switches        RAM           ROM
12345           Address Range  Address Range
========        ============= ================
00000		C:0000-C:07ff	C:2000-C:3fff
10000		C:0800-C:0fff
01000		C:1000-C:17ff
11000		C:1800-C:1fff
00100		C:4000-C:47ff	C:6000-C:7fff
10100		C:4800-C:4fff
01100		C:5000-C:57ff
11100		C:5800-C:5fff
00010		C:C000-C:C7ff	C:E000-C:ffff
10010		C:C800-C:Cfff
01010		C:D000-C:D7ff
11010		C:D800-C:Dfff
00110		D:0000-D:07ff	D:2000-D:3fff
10110		D:0800-D:0fff
01110		D:1000-D:17ff
11110		D:1800-D:1fff
00001		D:4000-D:47ff	D:6000-D:7fff
10001		D:4800-D:4fff
01001		D:5000-D:57ff
11001		D:5800-D:5fff
00101		D:8000-D:87ff	D:A000-D:bfff
10101		D:8800-D:8fff
01101		D:9000-D:97ff
11101		D:9800-D:9fff
00011		D:C000-D:c7ff	D:E000-D:ffff
10011		D:C800-D:cfff
01011		D:D000-D:d7ff
11011		D:D800-D:dfff
00111		E:0000-E:07ff	E:2000-E:3fff
10111		E:0800-E:0fff
01111		E:1000-E:17ff
11111		E:1800-E:1fff
========        ============= ================


## PureData Corp



### PDI507锛? 浣嶇綉鍗★級



  - 鏉ヨ嚜 Mark Rejhon <mdrejhon@magi.com>锛圓very 鐣ユ湁淇敼锛?
  - Avery 娉細鎴戣涓?PDI508 缃戝崱锛堜絾鑲畾涓嶆槸 PDI508Plus 缃戝崱锛変笌姝ゅ熀鏈浉鍚岀殑銆侾DI508Plus 缃戝崱浼间箮涓昏鏄蒋浠堕厤缃殑銆?

璺崇嚎锛?

	缃戝崱搴曢儴銆侀潬杩戣竟缂樿繛鎺ュ櫒澶勬湁涓€缁勮烦绾块樀鍒椼€傝闃靛垪鏍囨敞涓?J1銆傚畠浠帶鍒?IRQ 鍜屽叾浠栨煇浜涘姛鑳姐€傚彧鍦?IRQ 寮曡剼涓婃斁涓€涓烦绾裤€?

	ETS1銆丒TS2 鐢ㄤ簬杩滆窛绂荤綉缁滅殑鏃跺簭銆傝鍙傞槄鏈枃浠堕《閮ㄩ檮杩戠殑鏇撮€氱敤淇℃伅銆?

	J2 鏄竴涓袱寮曡剼鐨勮烦绾裤€傚簲璇ュ湪涓婇潰鏀句竴涓烦绾匡紝鍥犱负鎴戞嬁鍒板崱鏃朵笂闈㈠氨宸茬粡鏈変簡銆備笉杩囨垜涓嶇煡閬撹繖涓烦绾挎槸鍋氫粈涔堢殑銆?

	J3 鏄竴涓袱璺崇嚎鐨勯樀鍒椼€傛垜涓嶇煡閬撳畠鏄仛浠€涔堢殑锛屼絾鎴戞嬁鍒板崱鏃朵笂闈㈠凡缁忔湁涓や釜璺崇嚎浜嗐€傚畠鏄竴涓袱琛屼笁鍒椼€佸叡鍏釜寮曡剼鐨勬爡鏍笺€傝繖浜涜烦绾挎槸

```

	   .-------.
	 o | o   o |
	   :-------:    ------> Accessible end of card with connectors
	 o | o   o |             in this direction ------->
	   `-------'

```

Carl de Billy <CARL@carainfo.com> 瑙ｉ噴浜?J3 鍜?J4锛?


```

	   .-------.
	 o | o   o |
	   :-------:    TWIST Technology
	 o | o   o |
	   `-------'
	   .-------.
	   | o   o | o
	   :-------:    COAX Technology
	   | o   o | o
	   `-------'

  - If using coax cable in a bus topology the J4 jumper must be removed;
    place it on one pin.

  - If using bus topology with twisted pair wiring move the J3
    jumpers so they connect the middle pin and the pins closest to the RJ11
    Connectors.  Also the J4 jumper must be removed; place it on one pin of
    J4 jumper for storage.

  - If using  star topology with twisted pair wiring move the J3
    jumpers so they connect the middle pin and the pins closest to the RJ11
    connectors.


```

DIP 寮€鍏筹細

	瀹夎缃戝崱鏃讹紝鍦ㄧ綉鍗″彲瑙﹀強鐨勪竴绔彲璁块棶鐨?DIP 寮€鍏崇敤浜庤缃?ARCnet 鍦板潃銆傚叡鏈?8 涓紑鍏炽€傝浣跨敤 1 鍒?254 涔嬮棿鐨勫湴鍧€銆?

	==========      =========================
	Switch No.	ARCnet address
	12345678
	==========      =========================
	00000000	FF  	(Don't use this!)
	00000001	FE
	00000010	FD
	...
	11111101	2
	11111110	1
	11111111	0	(Don't use this!)
	==========      =========================

	缃戝崱椤堕儴杩樻湁鍙︿竴缁勫叓涓?DIP 寮€鍏炽€傚叾涓湁浜斾釜鏍囨敞涓?MS0-MS4锛屼技涔庢帶鍒跺唴瀛樺湴鍧€锛涘彟澶栦笁涓爣娉ㄤ负 IO0-IO2锛屼技涔庢帶鍒剁綉鍗＄殑 I/O 鍩哄潃銆?

	閫氳繃璇曢敊鏉ユ祴璇曡繖涓€鐐瑰緢鍥伴毦锛岃€屼笖 I/O 鍦板潃鐨勯『搴忓緢濂囨€€傛祴璇曟柟娉曚负锛氳缃?DIP 寮€鍏筹紝閲嶅惎璁＄畻鏈猴紝骞跺皾璇曚互鍚勭鍦板潃锛堜富瑕佸湪 0x200 鍒?0x400 涔嬮棿锛夊姞杞?ARCETHER銆傚鑷寸孩鑹插彂閫?LED 闂儊鐨勫湴鍧€锛屽氨鏄垜璁や负鍙敤鐨勫湴鍧€銆?

	鍙﹀锛屽湴鍧€ 0x3D0 浼间箮鏈夌壒娈婂惈涔夛紝鍥犱负 ARCETHER 鍖呴┍鍔ㄥ彲浠ユ甯稿姞杞斤紝浣嗙孩鑹?LED 涓嶉棯鐑併€備笉杩囨垜涓嶇煡閬?0x3D0 鏄仛浠€涔堢殑銆傛垜寤鸿浣跨敤 0x300 鍦板潃锛屽洜涓?Windows 鍙兘涓嶅枩娆綆浜?0x300 鐨勫湴鍧€銆?

	=============   ===========
	IO Switch No.   I/O address
	210
	=============   ===========
	111             0x260
	110             0x290
	101             0x2E0
	100             0x2F0
	011             0x300
	010             0x350
	001             0x380
	000             0x3E0
	=============   ===========

	鍐呭瓨寮€鍏宠缃竴娈?0x1000 瀛楄妭锛?x100 娈靛崟浣嶏紝鍗?4k锛夌殑淇濈暀鍦板潃绌洪棿銆備緥濡傦紝濡傛灉鎴戣缃湴鍧€ 0xD000锛屽畠灏嗕娇鐢?0xD000 鍒?0xD100 鐨勫湴鍧€銆?

	鍐呭瓨寮€鍏虫槸閫氳繃浣跨敤 QEMM386 stealth 鍚姩锛屽苟鐢?LOADHI 鏌ョ湅鍝簺鍦板潃鑷姩浠庝笂浣嶅唴瀛樺尯鍩熶腑琚帓闄わ紝鐒跺悗灏濊瘯鐢ㄨ繖浜涘湴鍧€鍔犺浇 ARCETHER 鏉ユ祴璇曠殑銆?

	鎴戝缓璁娇鐢?ARCnet 鍐呭瓨鍦板潃 0xD000锛屽苟鍦?QEMM stealth 妯″紡涓嬪皢 EMS 椤靛抚鏀惧湪 0xC000锛岃繖鏍蜂綘灏辫兘浠?0xD100 寮€濮嬭幏寰楀嚑涔庝竴鐩村埌鍏嗗瓧鑺傛湯绔殑杩炵画楂樹綅鍐呭瓨銆?

	鍐呭瓨寮€鍏?0锛圡S0锛夊湪鎴戠殑鍗′笂璁句负 OFF 鏃朵技涔庝笉鑳芥甯稿伐浣溿€傚畠鍙兘鏄垜鐨勫崱涓婂嚭浜嗘晠闅溿€傚厛璇曠潃灏嗗叾璁句负 ON锛屽鏋滀笉琛岋紝鍐嶈涓?OFF銆傦紙瀹冨彲鑳芥槸 0x200 浣嶇殑淇グ浣嶏紵锛?

	=============   ============================================
	MS Switch No.
	43210           Memory address
	=============   ============================================
	00001           0xE100  (guessed - was not detected by QEMM)
	00011           0xE000  (guessed - was not detected by QEMM)
	00101           0xDD00
	00111           0xDC00
	01001           0xD900
	01011           0xD800
	01101           0xD500
	01111           0xD400
	10001           0xD100
	10011           0xD000
	10101           0xCD00
	10111           0xCC00
	11001           0xC900 (guessed - crashes tested system)
	11011           0xC800 (guessed - crashes tested system)
	11101           0xC500 (guessed - crashes tested system)
	11111           0xC400 (guessed - crashes tested system)
	=============   ============================================


## CNet Technology Inc.锛?浣嶇綉鍗★級


### 120 绯诲垪锛?浣嶇綉鍗★級


  - 鏉ヨ嚜 Juergen Seifert <seifert@htwm.de>

鏈鏄庣敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁浠ヤ笅 CNet 鍘熺増鎵嬪唽缂栧啓

	      "ARCNET USER'S MANUAL for
	      CN120A
	      CN120AB
	      CN120TP
	      CN120ST
	      CN120SBT
	      P/N:12-01-0007
	      Revision 3.00"

ARCNET 鏄?Datapoint Corporation 鐨勬敞鍐屽晢鏍?

- P/N 120A   ARCNET 8浣?XT/AT 鏄熷瀷
- P/N 120AB  ARCNET 8浣?XT/AT 鎬荤嚎
- P/N 120TP  ARCNET 8浣?XT/AT 鍙岀粸绾?
- P/N 120ST  ARCNET 8浣?XT/AT 鏄熷瀷銆佸弻缁炵嚎
- P/N 120SBT ARCNET 8浣?XT/AT 鏄熷瀷銆佹€荤嚎銆佸弻缁炵嚎


```

    __________________________________________________________________
   |                                                                  |
   |                                                               ___|
   |                                                          LED |___|
   |                                                               ___|
   |                                                            N |   | ID7
   |                                                            o |   | ID6
   |                                                            d | S | ID5
   |                                                            e | W | ID4
   |                     ___________________                    A | 2 | ID3
   |                    |                   |                   d |   | ID2
   |                    |                   |  1 2 3 4 5 6 7 8  d |   | ID1
   |                    |                   | _________________ r |___| ID0
   |                    |      90C65        ||       SW1       |  ____|
   |  JP 8 7            |                   ||_________________| |    |
   |    |o|o|  JP1      |                   |                    | J2 |
   |    |o|o|  |oo|     |                   |         JP 1 1 1   |    |
   |   ______________   |                   |            0 1 2   |____|
   |  |  PROM        |  |___________________|           |o|o|o|  _____|
   |  >  SOCKET      |  JP 6 5 4 3 2                    |o|o|o| | J1  |
   |  |______________|    |o|o|o|o|o|                   |o|o|o| |_____|
   |_____                 |o|o|o|o|o|                   ______________|
	 |                                             |
	 |_____________________________________________|

```
```

  90C65       ARCNET Probe
  S1  1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  S2  1-8:    Node ID Select (ID0-ID7)
  JP1     ROM Enable Select
  JP2     IRQ2
  JP3     IRQ3
  JP4     IRQ4
  JP5     IRQ5
  JP6     IRQ7
  JP7/JP8     ET1, ET2 Timeout Parameters
  JP10/JP11   Coax / Twisted Pair Select  (CN120ST/SBT only)
  JP12        Terminator Select       (CN120AB/ST/SBT only)
  J1      BNC RG62/U Connector        (all except CN120TP)
  J2      Two 6-position Telephone Jack   (CN120TP/ST/SBT only)

```

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?


##### 璁剧疆鑺傜偣 ID


SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆? 鍙峰紑鍏筹紙ID0锛変綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?
杩欎簺鍙栧€间负锛?

```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 涓殑鏈€鍚庝笁涓紑鍏崇敤浜庨€夋嫨涓€涓?


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖猴紙RAM锛夐渶瑕?2K銆傝缂撳啿鍖虹殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€侭oot Prom 鐨勫湴鍧€涓哄唴瀛樺熀鍧€ + 8K 鎴栧唴瀛樺熀鍧€ + 0x2000銆?
寮€鍏冲潡 SW1 鐨?1-5 鍙峰紑鍏崇敤浜庨€夋嫨鍐呭瓨鍩哄潃銆?


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM install the jumper JP1

```


      Since the switches 1 and 2 are always set to ON it may be possible
      that they can be used to add an offset of 2K, 4K or 6K to the base
      address, but this feature is not documented in the manual and I
      haven't tested it yet.


##### 璁剧疆涓柇绾?


瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇峰畨瑁呭叾涓竴涓紙涓斿彧鑳藉畨瑁呬竴涓紒锛夎烦绾?


```

   Jumper | IRQ
   -------|-----
     2    |  2
     3    |  3
     4    |  4
     5    |  5
     6    |  7


```

##### 鍦?CN120AB/TP/SBT 涓婅缃唴閮ㄧ粓绔數闃?




```

			 -----
       0                |  0  |
     -----   ON         |     |  ON
    |  0  |             |  0  |
    |     |  OFF         -----   OFF
    |  0  |                0
     -----
   Terminator          Terminator
    disabled            enabled


```

##### 鍦?CN120ST/SBT 涓婇€夋嫨杩炴帴鍣ㄧ被鍨?




```

     JP10    JP11        JP10    JP11
			 -----   -----
       0       0        |  0  | |  0  |
     -----   -----      |     | |     |
    |  0  | |  0  |     |  0  | |  0  |
    |     | |     |      -----   -----
    |  0  | |  0  |        0       0
     -----   -----
     Coaxial Cable       Twisted Pair Cable
       (Default)


```

##### 璁剧疆瓒呮椂鍙傛暟


鏍囨湁 EXT1 鍜?EXT2 鐨勮烦绾跨敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜璺崇嚎閫氬父淇濇寔鏂紑锛堝紑璺級銆?


## CNet Technology Inc.锛?6浣嶇綉鍗★級


### 160 绯诲垪锛?6浣嶇綉鍗★級


  - 鏉ヨ嚜 Juergen Seifert <seifert@htwm.de>

鏈鏄庣敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁浠ヤ笅 CNet 鍘熺増鎵嬪唽缂栧啓

	      "ARCNET USER'S MANUAL for
	      CN160A CN160AB CN160TP
	      P/N:12-01-0006 Revision 3.00"

ARCNET 鏄?Datapoint Corporation 鐨勬敞鍐屽晢鏍?

- P/N 160A   ARCNET 16浣?XT/AT 鏄熷瀷
- P/N 160AB  ARCNET 16浣?XT/AT 鎬荤嚎
- P/N 160TP  ARCNET 16浣?XT/AT 鍙岀粸绾?


```

   ___________________________________________________________________
  <                             _________________________          ___|
  >               |oo| JP2     |                         |    LED |___|
  <               |oo| JP1     |        9026             |    LED |___|
  >                            |_________________________|         ___|
  <                                                             N |   | ID7
  >                                                      1      o |   | ID6
  <                                    1 2 3 4 5 6 7 8 9 0      d | S | ID5
  >         _______________           _____________________     e | W | ID4
  <        |     PROM      |         |         SW1         |    A | 2 | ID3
  >        >    SOCKET     |         |_____________________|    d |   | ID2
  <        |_______________|          | IO-Base   | MEM   |     d |   | ID1
  >                                                             r |___| ID0
  <                                                               ____|
  >                                                              |    |
  <                                                              | J1 |
  >                                                              |    |
  <                                                              |____|
  >                            1 1 1 1                                |
  <  3 4 5 6 7      JP     8 9 0 1 2 3                                |
  > |o|o|o|o|o|           |o|o|o|o|o|o|                               |
  < |o|o|o|o|o| __        |o|o|o|o|o|o|                    ___________|
  >            |  |                                       |
  <____________|  |_______________________________________|

```
```

  9026            ARCNET Probe
  SW1 1-6:    Base I/O Address Select
      7-10:   Base Memory Address Select
  SW2 1-8:    Node ID Select (ID0-ID7)
  JP1/JP2     ET1, ET2 Timeout Parameters
  JP3-JP13    Interrupt Select
  J1      BNC RG62/U Connector        (CN160A/AB only)
  J1      Two 6-position Telephone Jack   (CN160TP only)
  LED

```

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?


##### 璁剧疆鑺傜偣 ID


SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆?
1 鍙峰紑鍏筹紙ID0锛変綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?


```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 涓殑鍓嶅叚涓紑鍏崇敤浜庨€夋嫨 I/O 鍩哄湴鍧€


```

	     Switch        | Hex I/O
    1   2   3   4   5   6  | Address
   ------------------------|--------
   OFF ON  ON  OFF OFF ON  |  260
   OFF ON  OFF ON  ON  OFF |  290
   OFF ON  OFF OFF OFF ON  |  2E0  (Manufacturer's default)
   OFF ON  OFF OFF OFF OFF |  2F0
   OFF OFF ON  ON  ON  ON  |  300
   OFF OFF ON  OFF ON  OFF |  350
   OFF OFF OFF ON  ON  ON  |  380
   OFF OFF OFF OFF OFF ON  |  3E0

```

娉ㄦ剰锛氫技涔庤繕鍙互閫夋嫨鍏朵粬 I/O 鍩哄湴鍧€锛屼絾鎵嬪唽涓粎璁板綍浜嗕笂杩扮粍鍚堛€?


##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


寮€鍏冲潡 SW1 鐨?7-10 鍙峰紑鍏崇敤浜庨€夋嫨鍐呭瓨


```

   Switch          | Hex RAM | Hex ROM
    7   8   9  10  | Address | Address
   ----------------|---------|-----------
   OFF OFF ON  ON  |  C0000  |  C8000
   OFF OFF ON  OFF |  D0000  |  D8000 (Default)
   OFF OFF OFF ON  |  E0000  |  E8000

```


      Other MEM-Base addresses seem to be selectable, but only the above
      combinations are documented.


##### 璁剧疆涓柇绾?


瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇峰畨瑁呭叾涓竴涓紙涓斿彧鑳藉畨瑁呬竴涓紒锛夎烦绾?


```

   Jumper | IRQ
   -------|-----------------
     3    |  14
     4    |  15
     5    |  12
     6    |  11
     7    |  10
     8    |   3
     9    |   4
    10    |   5
    11    |   6
    12    |   7
    13    |   2 (=9) Default!

```


       - 涓嶈浣跨敤 JP11=IRQ6锛屽畠鍙兘涓庝綘鐨勮蒋鐩樻帶鍒跺櫒鍐茬獊
	 Controller
       - 浠呭綋娌℃湁 IDE銆丮FM 鎴?RLL 纭洏鏃舵墠浣跨敤 JP3=IRQ14锛屽惁鍒欏畠鍙兘涓庤繖浜涚‖鐩樻帶鍒跺櫒鍐茬獊


### 璁剧疆瓒呮椂鍙傛暟


鏍囨湁 JP1 鍜?JP2 鐨勮烦绾跨敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜璺崇嚎閫氬父淇濇寔鏂紑锛堝紑璺級銆?


## Lantech


### 8浣嶇綉鍗★紝鍨嬪彿鏈煡


  - 鏉ヨ嚜 Vlad Lungu <vlungu@ugal.ro> 鈥斺€?鎴戝皾璇曡仈绯讳粬鏃讹紝浠栫殑鐢靛瓙閭欢鍦板潃浼间箮宸插け鏁堛€俈lad锛屽鏋滀綘娌℃湁鏀跺埌鎴戠殑鍥炲锛屾姳姝夈€?


```

   ________________________________________________________________
   |   1         8                                                 |
   |   ___________                                               __|
   |   |   SW1    |                                         LED |__|
   |   |__________|                                                |
   |                                                            ___|
   |                _____________________                       |S | 8
   |                |                   |                       |W |
   |                |                   |                       |2 |
   |                |                   |                       |__| 1
   |                |      UM9065L      |     |o|  JP4         ____|____
   |                |                   |     |o|              |  CN    |
   |                |                   |                      |________|
   |                |                   |                          |
   |                |___________________|                          |
   |                                                               |
   |                                                               |
   |      _____________                                            |
   |      |            |                                           |
   |      |    PROM    |        |ooooo|  JP6                       |
   |      |____________|        |ooooo|                            |
   |_____________                                             _   _|
		|____________________________________________| |__|


```

UM9065L锛欰RCnet 鎺у埗鍣?

SW 1    锛氬叡浜唴瀛樺湴鍧€涓?I/O 鍩哄湴鍧€



```

	ON=0

	12345|Memory Address
	-----|--------------
	00001|  D4000
	00010|  CC000
	00110|  D0000
	01110|  D1000
	01101|  D9000
	10010|  CC800
	10011|  DC800
	11110|  D1800

```

浣嶄技涔庢槸鎸夌浉鍙嶉『搴忚В閲婄殑銆傛澶栵紝浣犲繀椤绘敞鎰忓叾涓煇浜涘湴鍧€骞朵笉甯歌锛屾垜涔熸湭瀵瑰畠浠繘琛屾帰娴嬶紱鎴戞槸鍦?DOS 涓嬮€氳繃鍐呭瓨杞偍鏉ヨ瘑鍒畠浠殑銆傚浜?00000 閰嶇疆浠ュ強鎴戞湭鍦ㄦ鍐欏嚭鐨勫叾浠栦竴浜涢厤缃紝璇ョ綉鍗′技涔庝細涓庢樉鍗★紙涓€鍧?S3 GENDAC锛夊啿绐併€傝繖浜涘湴鍧€鐨勫畬鏁磋В鐮佸氨鐣欑粰浣犱簡銆?


```

	678| I/O Address
	---|------------
	000|    260
	001|    failed probe
	010|    2E0
	011|    380
	100|    290
	101|    350
	110|    failed probe
	111|    3E0

  SW 2  : Node ID (binary coded)

  JP 4  : Boot PROM enable   CLOSE - enabled
			     OPEN  - disabled

  JP 6  : IRQ set (ONLY ONE jumper on 1-5 for IRQ 2-6)


```

## Acer


### 8浣嶇綉鍗★紝鍨嬪彿 5210-003



  - 鏉ヨ嚜 Vojtech Pavlik <vojtech@suse.cz>锛屼娇鐢ㄤ簡鐜版湁 arcnet-hardware 鏂囦欢鐨勯儴鍒嗗唴瀹广€?

杩欐槸涓€鍧楀熀浜?90C26 鐨勭綉鍗°€傚叾閰嶇疆浼间箮涓?SMC PC100 绫讳技锛屼絾鏈変竴浜涙垜涓嶇煡鍏跺惈涔夌殑棰濆璺崇嚎銆?


```

	       __
	      |  |
   ___________|__|_________________________
  |         |      |                       |
  |         | BNC  |                       |
  |         |______|                    ___|
  |  _____________________             |___
  | |                     |                |
  | | Hybrid IC           |                |
  | |                     |       o|o J1   |
  | |_____________________|       8|8      |
  |                               8|8 J5   |
  |                               o|o      |
  |                               8|8      |
  |__                             8|8      |
 (|__| LED                        o|o      |
  |                               8|8      |
  |                               8|8 J15  |
  |                                        |
  |                    _____               |
  |                   |     |   _____      |
  |                   |     |  |     |  ___|
  |                   |     |  |     | |
  |  _____            | ROM |  | UFS | |
  | |     |           |     |  |     | |
  | |     |     ___   |     |  |     | |
  | |     |    |   |  |__.__|  |__.__| |
  | | NCR |    |XTL|   _____    _____  |
  | |     |    |___|  |     |  |     | |
  | |90C26|           |     |  |     | |
  | |     |           | RAM |  | UFS | |
  | |     | J17 o|o   |     |  |     | |
  | |     | J16 o|o   |     |  |     | |
  | |__.__|           |__.__|  |__.__| |
  |  ___                               |
  | |   |8                             |
  | |SW2|                              |
  | |   |                              |
  | |___|1                             |
  |  ___                               |
  | |   |10           J18 o|o          |
  | |   |                 o|o          |
  | |SW1|                 o|o          |
  | |   |             J21 o|o          |
  | |___|1                             |
  |                                    |
  |____________________________________|


```
```

  90C26       ARCNET Chip
  XTL         20 MHz Crystal
  SW1 1-6     Base I/O Address Select
      7-10    Memory Address Select
  SW2 1-8     Node ID Select (ID0-ID7)
  J1-J5       IRQ Select
  J6-J21      Unknown (Probably extra timeouts & ROM enable ...)
  LED1        Activity LED
  BNC         Coax connector (STAR ARCnet)
  RAM         2k of SRAM
  ROM         Boot ROM socket
  UFS         Unidentified Flying Sockets


```

##### 璁剧疆鑺傜偣 ID


SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖涓嶈兘涓?0銆?
1 鍙峰紑鍏筹紙ID0锛変綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

灏嗘煇涓紑鍏虫嫧鍒?OFF锛堝叧锛夎〃绀?"1"锛屾嫧鍒?ON锛堝紑锛夎〃绀?"0"銆?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?


```

   Switch | Value
   -------|-------
     1    |   1
     2    |   2
     3    |   4
     4    |   8
     5    |  16
     6    |  32
     7    |  64
     8    | 128

```

涓嶈灏嗗叾璁句负 0 鎴?255锛涜繖涓や釜鍊兼槸淇濈暀鍊笺€?


##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 鐨?1 鑷?6 鍙峰紑鍏崇敤浜庨€夋嫨涓€涓?


```

	  | Hex
   Switch | Value
   -------|-------
     1    | 200
     2    | 100
     3    |  80
     4    |  40
     5    |  20
     6    |  10

```

I/O 鍦板潃鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏崇殑鍙栧€间箣鍜屻€傝娉ㄦ剰锛?x200 浠ヤ笅鐨?I/O 鍦板潃绌洪棿鏄负涓绘澘淇濈暀鐨勶紝鍥犳 1 鍙峰紑鍏冲簲濮嬬粓鎷ㄥ埌 OFF锛堝叧锛夈€?


##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖猴紙RAM锛夐渶瑕?2K銆傝缂撳啿鍖虹殑鍩哄潃鍙綅浜庡崄鍏釜浣嶇疆涓殑浠绘剰涓€涓€備笉杩囷紝A0000 浠ヤ笅鐨勫湴鍧€鍙兘浼氬洜涓哄瓨鍦ㄤ富鍐呭瓨鑰屽鑷寸郴缁熸寕璧枫€?


```

   Switch          | Hex RAM
    7   8   9  10  | Address
   ----------------|---------
   OFF OFF OFF OFF |  F0000 (conflicts with main BIOS)
   OFF OFF OFF ON  |  E0000
   OFF OFF ON  OFF |  D0000
   OFF OFF ON  ON  |  C0000 (conflicts with video BIOS)
   OFF ON  OFF OFF |  B0000 (conflicts with mono video)
   OFF ON  OFF ON  |  A0000 (conflicts with graphics)


```

##### 璁剧疆涓柇绾?


璺崇嚎鍧?J1 鐨?1-5 鍙疯烦绾挎帶鍒?IRQ 绾у埆銆侽N锛堝紑锛夎〃绀?


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  7
    OFF ON  OFF OFF OFF |  5
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  3
    OFF OFF OFF OFF ON  |  2


```

##### 鏈煡璺崇嚎涓庢彃妲?


鎴戝杩欎簺涓€鏃犳墍鐭ャ€傛垜鐚滄祴 J16 涓?J17 鏄秴鏃惰烦绾匡紝涔熻 J18-J21 涓湁涓€涓敤浜庨€夋嫨 ROM銆傛澶栵紝J6-J10 鍜?J11-J15 灏?IRQ2-7 杩炴帴鍒?UFS 涓婄殑鏌愪簺寮曡剼銆傛垜鐚滀笉鍑哄叾鐢ㄩ€斻€?

## Datapoint锛燂紙鍘傚晢鏈煡锛?


### LAN-ARC-8锛屼竴鍧?8浣嶇綉鍗?


  - 鏉ヨ嚜 Vojtech Pavlik <vojtech@suse.cz>

杩欐槸鍙︿竴鍧楀熀浜?SMC 90C65 鐨?ARCnet 缃戝崱銆傛垜鏃犳硶纭畾鍏跺埗閫犲晢锛屼絾瀹冨彲鑳芥槸 DataPoint锛屽洜涓鸿缃戝崱鍙充笂瑙掑甫鏈夊師濮嬬殑 arcNet 鏍囧織銆?


```

	  _______________________________________________________
	 |                         _________                     |
	 |                        |   SW2   | ON      arcNet     |
	 |                        |_________| OFF             ___|
	 |  _____________         1 ______  8                |   | 8
	 | |             | SW1     | XTAL | ____________     | S |
	 | > RAM (2k)    |         |______||            |    | W |
	 | |_____________|                 |      H     |    | 3 |
	 |                        _________|_____ y     |    |___| 1
	 |  _________            |         |     |b     |        |
	 | |_________|           |         |     |r     |        |
	 |                       |     SMC |     |i     |        |
	 |                       |    90C65|     |d     |        |
	 |  _________            |         |     |      |        |
	 | |   SW1   | ON        |         |     |I     |        |
	 | |_________| OFF       |_________|_____/C     |   _____|
	 |  1       8                      |            |  |     |___
	 |  ______________                 |            |  | BNC |___|
	 | |              |                |____________|  |_____|
	 | > EPROM SOCKET |              _____________           |
	 | |______________|             |_____________|          |
	 |                                         ______________|
	 |                                        |
	 |________________________________________|

```
```

  90C65       ARCNET Chip
  SW1 1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  SW2 1-8:    Node ID Select
  SW3 1-5:    IRQ Select
      6-7:    Extra Timeout
      8  :    ROM Enable
  BNC         Coax connector
  XTAL        20 MHz Crystal


```

##### 璁剧疆鑺傜偣 ID


SW3 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖涓嶈兘涓?0銆?
1 鍙峰紑鍏充綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?


```

   Switch | Value
   -------|-------
     1    |   1
     2    |   2
     3    |   4
     4    |   8
     5    |  16
     6    |  32
     7    |  64
     8    | 128


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 涓殑鏈€鍚庝笁涓紑鍏崇敤浜庨€夋嫨涓€涓?


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖猴紙RAM锛夐渶瑕?2K銆傝缂撳啿鍖虹殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€侭oot Prom 鐨勫湴鍧€涓哄唴瀛樺熀鍧€ + 0x2000銆?

寮€鍏冲潡 SW1 鐨?3-5 鍙疯烦绾跨敤浜庨€夋嫨鍐呭瓨鍩哄潃銆?


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM set the switch 8 of switch block SW3 to position ON.

```

1 鍙峰拰 2 鍙峰紑鍏冲彲鑳戒細缁?RAM 鍩哄潃澧炲姞 0x0800 鍜?0x1000銆?


##### 璁剧疆涓柇绾?


瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇峰畨瑁呭叾涓竴涓紙涓斿彧鑳藉畨瑁呬竴涓紒锛夎烦绾?


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  3
    OFF ON  OFF OFF OFF |  4
    OFF OFF ON  OFF OFF |  5
    OFF OFF OFF ON  OFF |  7
    OFF OFF OFF OFF ON  |  2


```

##### 璁剧疆瓒呮椂鍙傛暟


寮€鍏冲潡 SW3 鐨?6-7 鍙峰紑鍏崇敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜寮€鍏抽€氬父淇濇寔鍦?OFF锛堝叧锛変綅缃€?


## Topware


### 8浣嶇綉鍗★紝TA-ARC/10


  - 鏉ヨ嚜 Vojtech Pavlik <vojtech@suse.cz>

杩欐槸鍙︿竴鍧楅潪甯哥浉浼肩殑 90C65 缃戝崱銆傚叾澶ч儴鍒嗗紑鍏冲拰璺崇嚎涓庡叾浠栧吋瀹瑰崱鐩稿悓銆?


```

   _____________________________________________________________________
  |  ___________   |                         |            ______        |
  | |SW2 NODE ID|  |                         |           | XTAL |       |
  | |___________|  |  Hybrid IC              |           |______|       |
  |  ___________   |                         |                        __|
  | |SW1 MEM+I/O|  |_________________________|                   LED1|__|)
  | |___________|           1 2                                         |
  |                     J3 |o|o| TIMEOUT                          ______|
  |     ______________     |o|o|                                 |      |
  |    |              |  ___________________                     | RJ   |
  |    > EPROM SOCKET | |                   \                    |------|
  |J2  |______________| |                    |                   |      |
  ||o|                  |                    |                   |______|
  ||o| ROM ENABLE       |        SMC         |    _________             |
  |     _____________   |       90C65        |   |_________|       _____|
  |    |             |  |                    |                    |     |___
  |    > RAM (2k)    |  |                    |                    | BNC |___|
  |    |_____________|  |                    |                    |_____|
  |                     |____________________|                          |
  | ________ IRQ 2 3 4 5 7                  ___________                 |
  ||________|   |o|o|o|o|o|                |___________|                |
  |________   J1|o|o|o|o|o|                               ______________|
	   |                                             |
	   |_____________________________________________|

```
```

  90C65       ARCNET Chip
  XTAL        20 MHz Crystal
  SW1 1-5     Base Memory Address Select
      6-8     Base I/O Address Select
  SW2 1-8     Node ID Select (ID0-ID7)
  J1          IRQ Select
  J2          ROM Enable
  J3          Extra Timeout
  LED1        Activity LED
  BNC         Coax connector (BUS ARCnet)
  RJ          Twisted Pair Connector (daisy chain)


```

##### 璁剧疆鑺傜偣 ID


SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖涓嶈兘涓?0銆? 鍙峰紑鍏筹紙ID0锛変綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?


```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 涓殑鏈€鍚庝笁涓紑鍏崇敤浜庨€夋嫨涓€涓?


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260  (Manufacturer's default)
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖猴紙RAM锛夐渶瑕?2K銆傝缂撳啿鍖虹殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€侭oot Prom 鐨勫湴鍧€涓哄唴瀛樺熀鍧€ + 0x2000銆?

寮€鍏冲潡 SW1 鐨?3-5 鍙疯烦绾跨敤浜庨€夋嫨鍐呭瓨鍩哄潃銆?


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000  (Manufacturer's default)
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

   *) To enable the Boot ROM short the jumper J2.

```

1 鍙峰拰 2 鍙疯烦绾垮彲鑳戒細缁?RAM 鍦板潃澧炲姞 0x0800 鍜?0x1000銆?


##### 璁剧疆涓柇绾?


璺崇嚎鍧?J1 鐨?1-5 鍙疯烦绾挎帶鍒?IRQ 绾у埆銆侽N锛堝紑锛夎〃绀?


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  2
    OFF ON  OFF OFF OFF |  3
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  5
    OFF OFF OFF OFF ON  |  7


```

##### 璁剧疆瓒呮椂鍙傛暟


璺崇嚎 J3 鐢ㄤ簬璁剧疆瓒呮椂鍙傛暟銆傝繖涓や釜璺崇嚎閫氬父淇濇寔鏂紑锛堝紑璺級銆?

## Thomas-Conrad


### 鍨嬪彿 #500-6242-0097 REV A锛?浣嶇綉鍗★級


  - 鏉ヨ嚜 Lars Karlsson <100617.3473@compuserve.com>


```

     ________________________________________________________
   |          ________   ________                           |_____
   |         |........| |........|                            |
   |         |________| |________|                         ___|
   |            SW 3       SW 1                           |   |
   |         Base I/O   Base Addr.                Station |   |
   |                                              address |   |
   |    ______                                    switch  |   |
   |   |      |                                           |   |
   |   |      |                                           |___|
   |   |      |                                 ______        |___._
   |   |______|                                |______|         ____| BNC
   |                                            Jumper-        _____| Connector
   |   Main chip                                block  _    __|   '
   |                                                  | |  |    RJ Connector
   |                                                  |_|  |    with 110 Ohm
   |                                                       |__  Terminator
   |    ___________                                         __|
   |   |...........|                                       |    RJ-jack
   |   |...........|    _____                              |    (unused)
   |   |___________|   |_____|                             |__
   |  Boot PROM socket IRQ-jumpers                            |_  Diagnostic
   |________                                       __          _| LED (red)
	    | | | | | | | | | | | | | | | | | | | |  |        |
	    | | | | | | | | | | | | | | | | | | | |  |________|
							      |
							      |

```

浠ヤ笅鏄缃戝崱涓婇儴鍒嗗紑鍏冲拰璺崇嚎鐨勮缃€?


```

	    I/O

	   1 2 3 4 5 6 7 8

  2E0----- 0 0 0 1 0 0 0 1
  2F0----- 0 0 0 1 0 0 0 0
  300----- 0 0 0 0 1 1 1 1
  350----- 0 0 0 0 1 1 1 0

```

涓婅堪绀轰緥涓殑 "0" 琛ㄧず寮€鍏充负鍏筹紙off锛夛紝"1" 琛ㄧず寮€鍏充负寮€锛坥n锛夈€?


```

      ShMem address.

	1 2 3 4 5 6 7 8

  CX00--0 0 1 1 | |   |
  DX00--0 0 1 0       |
  X000--------- 1 1   |
  X400--------- 1 0   |
  X800--------- 0 1   |
  XC00--------- 0 0
  ENHANCED----------- 1
  COMPATIBLE--------- 0

```
```

	 IRQ


     3 4 5 7 2
     . . . . .
     . . . . .


```

鏈変竴涓甫 8 涓紑鍏崇殑 DIP 寮€鍏筹紝鐢ㄤ簬璁剧疆瑕佷娇鐢ㄧ殑鍏变韩鍐呭瓨鍦板潃銆傚墠 6 涓紑鍏宠缃湴鍧€锛岀 7 涓病鏈変换浣曞姛鑳斤紝绗?8 涓紑鍏崇敤浜庨€夋嫨 "compatible"锛堝吋瀹癸級鎴?"enhanced"锛堝寮猴級銆傛垜鎷垮埌涓ゅ潡缃戝崱鏃讹紝鍏朵腑涓€鍧楃殑杩欎釜寮€鍏宠鍦ㄤ簡 "enhanced"锛堝寮猴級銆傞偅鍧楃綉鍗℃牴鏈棤娉曞伐浣滐紝椹卞姩绋嬪簭鐢氳嚦鏃犳硶璇嗗埆瀹冦€傚彟涓€鍧楃綉鍗¤寮€鍏宠鍦ㄤ簡 "compatible"锛堝吋瀹癸級锛岃〃鐜板畬鍏ㄦ甯搞€傛垜鐚滄祴鍏朵腑涓€鍧楃綉鍗＄殑寮€鍏冲湪浠庡師涓绘満鍙栧嚭鏃朵竴瀹氳鎰忓鏀瑰姩杩囥€?enhanced"锛堝寮猴級浣嶇疆鐨勭敤閫斿埌搴曟槸浠€涔堬紝杩欎釜闂浠嶆湭寰楀埌瑙ｇ瓟銆?

[Avery 鐨勬敞閲婏細"enhanced"锛堝寮猴級鍙兘绂佺敤鍏变韩鍐呭瓨锛堟敼鐢?I/O 绔彛锛夛紝涔熷彲鑳界鐢?I/O 绔彛锛堟敼鐢ㄥ唴瀛樺湴鍧€锛夈€傚叿浣撳洜缃戝崱绫诲瀷鑰屽紓銆傛垜瀹炲湪鐪嬩笉鍑鸿繖涓ょ鏂瑰紡鏈変綍"澧炲己"涔嬪銆傚鏋滃姝ゆā寮忔湁鏇磋缁嗙殑淇℃伅锛岃鍙戠粰鎴戯紝鍚﹀垯鐩存帴浣跨敤 "compatible"锛堝吋瀹癸級妯″紡鍗冲彲銆俔

## Waterloo Microsystems Inc.锛燂紙鍘傚晢鏈煡锛?


### 8浣嶇綉鍗★紙C锛?985


  - 鏉ヨ嚜 Robert Michael Best <rmb117@cs.usask.ca>

[Avery 鐨勬敞閲婏細鍑轰簬鏌愮鍘熷洜锛岃繖浜涚綉鍗℃棤娉曚笌鎴戠殑椹卞姩绋嬪簭閰嶅悎宸ヤ綔銆傝繖浜涚綉鍗＄殑璁剧疆浼间箮涓?PDI508Plus 绫讳技锛岃€屽悗鑰呮槸杞欢閰嶇疆鐨勶紝涔熸棤娉曚笌鎴戠殑椹卞姩绋嬪簭閰嶅悎宸ヤ綔銆?Waterloo 鑺墖"鏄竴鍧楀惎鍔?PROM锛屽緢鍙兘鏄笓闂ㄤ负婊戦搧鍗㈠ぇ瀛﹁璁＄殑銆傚鏋滀綘鏈夊叧浜庢缃戝崱鐨勬洿澶氫俊鎭紝璇峰彂鐢靛瓙閭欢缁欐垜銆俔

鎺㈡祴绋嬪簭鏃犳硶鍦ㄤ换浣?J2 璁剧疆涓嬫娴嬪埌璇ョ綉鍗★紝鍗充究鎴戝彇涓?"Waterloo" 鑺墖鍚庡啀娆″皾璇曚篃鏄姝ゃ€?


```

   _____________________________________________________________________
  | \/  \/              ___  __ __                                      |
  | C4  C4     |^|     | M ||  ^  ||^|                                  |
  | --  --     |_|     | 5 ||     || | C3                               |
  | \/  \/      C10    |___||     ||_|                                  |
  | C4  C4             _  _ |     |                 ??                  |
  | --  --            | \/ ||     |                                     |
  |                   |    ||     |                                     |
  |                   |    ||  C1 |                                     |
  |                   |    ||     |  \/                            _____|
  |                   | C6 ||     |  C9                           |     |___
  |                   |    ||     |  --                           | BNC |___|
  |                   |    ||     |          >C7|                 |_____|
  |                   |    ||     |                                     |
  | __ __             |____||_____|       1 2 3     6                   |
  ||  ^  |     >C4|                      |o|o|o|o|o|o| J2    >C4|       |
  ||     |                               |o|o|o|o|o|o|                  |
  || C2  |     >C4|                                          >C4|       |
  ||     |                                   >C8|                       |
  ||     |       2 3 4 5 6 7  IRQ                            >C4|       |
  ||_____|      |o|o|o|o|o|o| J3                                        |
  |_______      |o|o|o|o|o|o|                            _______________|
	  |                                             |
	  |_____________________________________________|

  C1 -- "COM9026
	 SMC 8638"
	In a chip socket.

  C2 -- "@Copyright
	 Waterloo Microsystems Inc.
	 1985"
	In a chip Socket with info printed on a label covering a round window
	showing the circuit inside. (The window indicates it is an EPROM chip.)

  C3 -- "COM9032
	 SMC 8643"
	In a chip socket.

  C4 -- "74LS"
	9 total no sockets.

  M5 -- "50006-136
	 20.000000 MHZ
	 MTQ-T1-S3
	 0 M-TRON 86-40"
	Metallic case with 4 pins, no socket.

  C6 -- "MOSTEK@TC8643
	 MK6116N-20
	 MALAYSIA"
	No socket.

  C7 -- No stamp or label but in a 20 pin chip socket.

  C8 -- "PAL10L8CN
	 8623"
	In a 20 pin socket.

  C9 -- "PAl16R4A-2CN
	 8641"
	In a 20 pin socket.

  C10 -- "M8640
	    NMC
	  9306N"
	 In an 8 pin socket.

  ?? -- Some components on a smaller board and attached with 20 pins all
	along the side closest to the BNC connector.  The are coated in a dark
	resin.

```

鐢佃矾鏉夸笂鏈変袱缁勬爣鏈?J2 鍜?J3 鐨勮烦绾挎帓銆傚埗閫犲晢娌℃湁鍦ㄦ澘涓婃斁缃?J1銆傛垜鎵嬩笂鐨勪袱鍧楃數璺澘閮藉悇甯︽湁涓€涓搴旇烦绾挎帓鐨勮烦绾跨洅銆?


```

  J2 -- Numbered 1 2 3 4 5 6.
	4 and 5 are not stamped due to solder points.

  J3 -- IRQ 2 3 4 5 6 7

```

鐢佃矾鏉挎湰韬湪 IRQ 璺崇嚎涓婃柟鍘嬪嵃鏈変竴鐗囨灚鍙讹紝C2 鏃佽竟鍗版湁 "-2 46-86"銆傚湪 C1 涓?C6 涔嬮棿鍘嬪嵃鏈?"ASS 'Y 300163"锛孊NC 杩炴帴鍣ㄦ涓嬫柟鍘嬪嵃鏈?"@1986 CORMAN CUSTOM ELECTRONICS CORP."銆傚叾涓嬫柟涓?"MADE IN CANADA"锛堝姞鎷垮ぇ鍒堕€狅級

## 鏃犲悕缃戝崱锛圢o Name锛?


### 8浣嶇綉鍗°€?6浣嶇綉鍗?


  - 鏉ヨ嚜 Juergen Seifert <seifert@htwm.de>

鎴戝皢杩欏潡 ARCnet 缃戝崱鍛藉悕涓?"NONAME"锛堟棤鍚嶏級锛屽洜涓哄湪瀹夎鎵嬪唽鍜屽寘瑁呯洅涓婇兘鎵句笉鍒颁换浣曞埗閫犲晢鍚嶇О銆傚敮涓€鏆楃ず瀛樺湪鍒堕€犲晢鐨勭棔杩规槸浠ラ摐绠斿嵃鍑虹殑 "Made in Taiwan"锛堝彴婀惧埗閫狅級銆?

鏈鏄庣敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁鍘熺増缂栧啓

		    "ARCnet Installation Manual"


```

    ________________________________________________________________
   | |STAR| BUS| T/P|                                               |
   | |____|____|____|                                               |
   |                            _____________________               |
   |                           |                     |              |
   |                           |                     |              |
   |                           |                     |              |
   |                           |        SMC          |              |
   |                           |                     |              |
   |                           |       COM90C65      |              |
   |                           |                     |              |
   |                           |                     |              |
   |                           |__________-__________|              |
   |                                                           _____|
   |      _______________                                     |  CN |
   |     | PROM          |                                    |_____|
   |     > SOCKET        |                                          |
   |     |_______________|         1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8 |
   |                               _______________  _______________ |
   |           |o|o|o|o|o|o|o|o|  |      SW1      ||      SW2      ||
   |           |o|o|o|o|o|o|o|o|  |_______________||_______________||
   |___         2 3 4 5 7 E E R        Node ID       IOB__|__MEM____|
       |        \ IRQ   / T T O                      |
       |__________________1_2_M______________________|

```
```

  COM90C65:       ARCnet Probe
  S1  1-8:    Node ID Select
  S2  1-3:    I/O Base Address Select
      4-6:    Memory Base Address Select
      7-8:    RAM Offset Select
  ET1, ET2    Extended Timeout Select
  ROM     ROM Enable Select
  CN              RG62 Coax Connector
  STAR| BUS | T/P Three fields for placing a sign (colored circle)
		  indicating the topology of the card

```

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?


##### 璁剧疆鑺傜偣 ID


缁?SW1 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆?
杩炴帴鍒扮綉缁滅殑姣忎釜鑺傜偣蹇呴』鍏锋湁鍞竴鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆?
8 鍙峰紑鍏充綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?


```

    Switch | Value
    -------|-------
      8    |   1
      7    |   2
      6    |   4
      5    |   8
      4    |  16
      3    |  32
      2    |  64
      1    | 128

```
```

    Switch         | Hex     | Decimal
   1 2 3 4 5 6 7 8 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏崇粍 SW2 涓殑鍓嶄笁涓紑鍏崇敤浜庨€夋嫨涓€涓?


```

   Switch      | Hex I/O
    1   2   3  | Address
   ------------|--------
   ON  ON  ON  |  260
   ON  ON  OFF |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   ON  OFF OFF |  2F0
   OFF ON  ON  |  300
   OFF ON  OFF |  350
   OFF OFF ON  |  380
   OFF OFF OFF |  3E0


```


##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖洪渶瑕?16K RAM 鍧椾腑鐨?2K銆傝 16K 鍧楃殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€傚紑鍏崇粍 SW2 鐨?4-6 鍙峰紑鍏抽€夋嫨 16K 鍧楃殑鍩哄潃銆傚湪璇?16K 鍦板潃绌洪棿鍐咃紝缂撳啿鍖哄彲琚垎閰嶅埌鍥涗釜浣嶇疆涓殑浠绘剰涓€涓紝鍏蜂綋鐢卞亸绉婚噺锛堝嵆缁?SW2 鐨?7 鍙峰拰 8 鍙峰紑鍏筹級鍐冲畾銆?


```

   Switch     | Hex RAM | Hex ROM
   4 5 6  7 8 | Address | Address *)
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000

   *) To enable the 8K Boot PROM install the jumper ROM.
      The default is jumper ROM not installed.


```

##### 璁剧疆涓柇璇锋眰绾匡紙IRQ锛?


瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇疯缃烦绾?IRQ2銆両RQ3銆両RQ4銆両RQ5 鎴?IRQ7 涓殑涓€涓紙涓斿彧鑳借缃竴涓紒锛夈€傚巶鍟嗛粯璁ゅ€间负 IRQ2銆?


##### 璁剧疆瓒呮椂


鏍囨湁 ET1 鍜?ET2 鐨勪袱涓烦绾跨敤浜庣‘瀹氳秴鏃跺弬鏁帮紙鍝嶅簲鏃堕棿涓庨噸閰嶇疆鏃堕棿锛夈€傜綉缁滀腑鐨勬瘡涓妭鐐归兘蹇呴』璁剧疆涓虹浉鍚岀殑瓒呮椂鍊笺€?


```

   ET1 ET2 | Response Time (us) | Reconfiguration Time (ms)
   --------|--------------------|--------------------------
   Off Off |        78          |          840   (Default)
   Off On  |       285          |         1680
   On  Off |       563          |         1680
   On  On  |      1130          |         1680

```

On锛堝紑锛夎〃绀哄凡瀹夎璺崇嚎锛孫ff锛堝叧锛夎〃绀烘湭瀹夎璺崇嚎


### 16浣?ARCNET


鎴戠殑 8浣?NONAME ARCnet 缃戝崱鎵嬪唽涓繕鍖呭惈浜嗗涓€鍧?16浣?鍚岃酱鐢电紗/鍙岀粸绾?缃戝崱鐨勬弿杩般€傝鎻忚堪涓嶅畬鏁达紝鍥犱负鎵嬪唽灏忓唽瀛愪腑缂轰簡涓ら〉銆傦紙鐩綍涓垪鍑轰簡椤电爜鈥︹€?-9銆?-11銆?-12銆?-1鈥︹€︼紝浣嗗皬鍐屽瓙鍐呴儴鐨勯〉鐮佺紪鎺掓柟寮忎笉鍚屸€︹€?-9銆?-10銆丄-1銆侊紙绌虹櫧椤碉級銆?-1鈥︹€︺€?-18銆丄-1锛堝啀娆″嚭鐜帮級銆丄-2锛夈€傛澶栵紝鐢佃矾鏉垮竷灞€鍥剧殑璐ㄩ噺涓嶅 8浣?缃戝崱閭ｅ紶锛屽洜涓哄浘涓婃病鏈夋爣娉ㄧ被浼?"SW1" 鐨勫瓧鏍枫€?

濡傛灉鏈変汉鎷ユ湁杩欐牱涓€鍧楃數璺澘锛岃闅忔椂琛ュ厖姝ゆ弿杩版垨缁欐垜鍙戦偖浠讹紒

鏈鏄庣敱 Juergen Seifert <seifert@htwm.de> 鏍规嵁鍘熺増缂栧啓

		    "ARCnet Installation Manual"


```

   ___________________________________________________________________
  <                    _________________  _________________           |
  >                   |       SW?       ||      SW?        |          |
  <                   |_________________||_________________|          |
  >                       ____________________                        |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |____________________|                       |
  <                                                               ____|
  >                       ____________________                   |    |
  <                      |                    |                  | J1 |
  >                      |                    <                  |    |
  <                      |____________________|  ? ? ? ? ? ?     |____|
  >                                             |o|o|o|o|o|o|         |
  <                                             |o|o|o|o|o|o|         |
  >                                                                   |
  <             __                                         ___________|
  >            |  |                                       |
  <____________|  |_______________________________________|


```

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?


##### 璁剧疆鑺傜偣 ID


缁?SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆?
8 鍙峰紑鍏充綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?

```

    Switch | Value
    -------|-------
      8    |   1
      7    |   2
      6    |   4
      5    |   8
      4    |  16
      3    |  32
      2    |  64
      1    | 128

```
```

    Switch         | Hex     | Decimal
   1 2 3 4 5 6 7 8 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏崇粍 SW1 涓殑鍓嶄笁涓紑鍏崇敤浜庨€夋嫨涓€涓?

```

   Switch      | Hex I/O
    3   2   1  | Address
   ------------|--------
   ON  ON  ON  |  260
   ON  ON  OFF |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   ON  OFF OFF |  2F0
   OFF ON  ON  |  300
   OFF ON  OFF |  350
   OFF OFF ON  |  380
   OFF OFF OFF |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖洪渶瑕?16K RAM 鍧椾腑鐨?2K銆傝 16K 鍧楃殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€?
寮€鍏崇粍 SW1 鐨?6-8 鍙峰紑鍏抽€夋嫨 16K 鍧楃殑鍩哄潃銆?
鍦ㄨ 16K 鍦板潃绌洪棿鍐咃紝缂撳啿鍖哄彲琚垎閰嶅埌鍥涗釜浣嶇疆涓殑浠绘剰涓€涓?

```

   Switch     | Hex RAM | Hex ROM
   8 7 6  5 4 | Address | Address
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000


```

##### 璁剧疆涓柇璇锋眰绾匡紙IRQ锛?


??????????????????????????????????????


##### 璁剧疆瓒呮椂


??????????????????????????????????????


### 8浣嶇綉鍗★紙"Made in Taiwan R.O.C."锛?


  - 鏉ヨ嚜 Vojtech Pavlik <vojtech@suse.cz>

鎴戝皢杩欏潡 ARCnet 缃戝崱鍛藉悕涓?"NONAME"锛堟棤鍚嶏級锛屽洜涓烘垜鍙嬁鍒颁簡杩欏紶鍗★紝娌℃湁浠讳綍鎵嬪唽锛岃€屽敮涓€鑳芥爣璇嗗埗閫犲晢鐨勬枃瀛楁槸鍗板湪鍗′笂鐨?"MADE IN TAIWAN R.O.C"銆?


```

	  ____________________________________________________________
	 |                 1 2 3 4 5 6 7 8                            |
	 | |o|o| JP1       o|o|o|o|o|o|o|o| ON                        |
	 |  +              o|o|o|o|o|o|o|o|                        ___|
	 |  _____________  o|o|o|o|o|o|o|o| OFF         _____     |   | ID7
	 | |             | SW1                         |     |    |   | ID6
	 | > RAM (2k)    |        ____________________ |  H  |    | S | ID5
	 | |_____________|       |                    ||  y  |    | W | ID4
	 |                       |                    ||  b  |    | 2 | ID3
	 |                       |                    ||  r  |    |   | ID2
	 |                       |                    ||  i  |    |   | ID1
	 |                       |       90C65        ||  d  |    |___| ID0
	 |      SW3              |                    ||     |        |
	 | |o|o|o|o|o|o|o|o| ON  |                    ||  I  |        |
	 | |o|o|o|o|o|o|o|o|     |                    ||  C  |        |
	 | |o|o|o|o|o|o|o|o| OFF |____________________||     |   _____|
	 |  1 2 3 4 5 6 7 8                            |     |  |     |___
	 |  ______________                             |     |  | BNC |___|
	 | |              |                            |_____|  |_____|
	 | > EPROM SOCKET |                                           |
	 | |______________|                                           |
	 |                                              ______________|
	 |                                             |
	 |_____________________________________________|

```
```

  90C65       ARCNET Chip
  SW1 1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  SW2 1-8:    Node ID Select (ID0-ID7)
  SW3 1-5:    IRQ Select
      6-7:    Extra Timeout
      8  :    ROM Enable
  JP1         Led connector
  BNC         Coax connector

```

灏界 SW1 鍜?SW3 鏍囨敞涓?SW 鑰岄潪 JP锛屼絾瀹冧滑鏄烦绾匡紝涓嶆槸寮€鍏炽€?

灏嗚烦绾胯涓?ON锛堝紑锛夎〃绀鸿繛鎺ヤ笂鏂圭殑涓や釜寮曡剼锛岃涓?off 琛ㄧず杩炴帴涓嬫柟涓や釜寮曡剼鈥斺€旀垨鑰呪€斺€斿湪 IRQ 璁剧疆鐨勬儏鍐典笅锛岃〃绀哄畬鍏ㄤ笉杩炴帴浠讳綍寮曡剼銆?

##### 璁剧疆鑺傜偣 ID


SW2 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖涓嶈兘涓?0銆?
1 鍙峰紑鍏筹紙ID0锛変綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?

灏嗘煇涓紑鍏虫嫧鍒?Off锛堝叧锛夎〃绀?"1"锛屾嫧鍒?On锛堝紑锛夎〃绀?"0"銆?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?

```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏冲潡 SW1 涓殑鏈€鍚庝笁涓紑鍏崇敤浜庨€夋嫨涓€涓?

```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨锛圧AM锛夌紦鍐插尯鍦板潃


鍐呭瓨缂撳啿鍖猴紙RAM锛夐渶瑕?2K銆傝缂撳啿鍖虹殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€侭oot Prom 鐨勫湴鍧€涓哄唴瀛樺熀鍧€ + 0x2000銆?

璺崇嚎鍧?SW1 鐨?3-5 鍙疯烦绾跨敤浜庨€夋嫨鍐呭瓨鍩哄潃銆?


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM set the jumper 8 of jumper block SW3 to position ON.

```

1 鍙峰拰 2 鍙疯烦绾垮彲鑳戒細缁?RAM 鍦板潃澧炲姞 0x0800銆?x1000 鍜?0x1800銆?


##### 璁剧疆涓柇绾?



```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  2
    OFF ON  OFF OFF OFF |  3
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  5
    OFF OFF OFF OFF ON  |  7


```

##### 璁剧疆瓒呮椂鍙傛暟


璺崇嚎鍧?SW3 鐨?6-7 鍙疯烦绾跨敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜璺崇嚎閫氬父淇濇寔鍦?OFF锛堝叧锛変綅缃€?



### 锛堥€氱敤鍨嬪彿 9058锛?


  - 鏉ヨ嚜 Andrew J. Kroll <ag784@freenet.buffalo.edu>
  - 鎶辨瓑杩欎唤璧勬枡鍦ㄦ垜鐨勫緟鍔炵閲屾悂缃簡杩欎箞涔咃紝Andrew锛侊紙鍝庡憖鈥斺€旇秴杩囦竴骞翠簡锛侊級


```

								      _____
								     |    <
								     | .---'
    ________________________________________________________________ | |
   |                           |     SW2     |                      |  |
   |   ___________             |_____________|                      |  |
   |  |           |              1 2 3 4 5 6                     ___|  |
   |  >  6116 RAM |         _________                         8 |   |  |
   |  |___________|        |20MHzXtal|                        7 |   |  |
   |                       |_________|       __________       6 | S |  |
   |    74LS373                             |          |-     5 | W |  |
   |   _________                            |      E   |-     4 |   |  |
   |   >_______|              ______________|..... P   |-     3 | 3 |  |
   |                         |              |    : O   |-     2 |   |  |
   |                         |              |    : X   |-     1 |___|  |
   |   ________________      |              |    : Y   |-           |  |
   |  |      SW1       |     |      SL90C65 |    :     |-           |  |
   |  |________________|     |              |    : B   |-           |  |
   |    1 2 3 4 5 6 7 8      |              |    : O   |-           |  |
   |                         |_________o____|..../ A   |-    _______|  |
   |    ____________________                |      R   |-   |       |------,
   |   |                    |               |      D   |-   |  BNC  |   #  |
   |   > 2764 PROM SOCKET   |               |__________|-   |_______|------'
   |   |____________________|              _________                |  |
   |                                       >________| <- 74LS245    |  |
   |                                                                |  |
   |___                                               ______________|  |
       |H H H H H H H H H H H H H H H H H H H H H H H|               | |
       |U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U|               | |
								      \|

```
```

  SL90C65 	ARCNET Controller / Transceiver /Logic
  SW1	1-5:	IRQ Select
	  6:	ET1
	  7:	ET2
	  8:	ROM ENABLE
  SW2	1-3:    Memory Buffer/PROM Address
	3-6:	I/O Address Map
  SW3	1-8:	Node ID Select
  BNC		BNC RG62/U Connection
		*I* have had success using RG59B/U with *NO* terminators!
		What gives?!

```

##### SW1锛氳秴鏃躲€佷腑鏂笌 ROM


瑕侀€夋嫨涓€涓‖浠朵腑鏂骇鍒紝璇峰皢 SW1 涓婏紙浣嶄簬 1-5 鍙峰紑鍏冲锛夌殑 DIP 寮€鍏充腑鐨勪竴涓嫧鍒?up锛坥n锛夛細IRQ3銆両RQ4銆両RQ5銆両RQ7銆両RQ2銆傚巶鍟嗛粯璁ゅ€间负 IRQ2銆?

SW1 涓婃爣鏈?EXT1锛? 鍙峰紑鍏筹級鍜?EXT2锛? 鍙峰紑鍏筹級鐨勫紑鍏崇敤浜庣‘瀹氳秴鏃跺弬鏁般€傝繖涓や釜 DIP 寮€鍏抽€氬父淇濇寔鍦?off锛坉own锛変綅缃€?

   瑕佸惎鐢?8K Boot PROM锛岃灏?SW1 涓婃爣涓?ROM 鐨?8 鍙峰紑鍏虫嫧鍒?on锛圲P锛夈€?  榛樿鎯呭喌涓?ROM 璺崇嚎鏈畨瑁呫€?


##### 璁剧疆 I/O 鍩哄湴鍧€


寮€鍏崇粍 SW2 涓殑鏈€鍚庝笁涓紑鍏崇敤浜庨€夋嫨涓€涓?

```


   Switch | Hex I/O
   4 5 6  | Address
   -------|--------
   0 0 0  |  260
   0 0 1  |  290
   0 1 0  |  2E0  (Manufacturer's default)
   0 1 1  |  2F0
   1 0 0  |  300
   1 0 1  |  350
   1 1 0  |  380
   1 1 1  |  3E0


```

##### 璁剧疆鍩哄潃鍐呭瓨鍦板潃锛圧AM 涓?ROM锛?


鍐呭瓨缂撳啿鍖洪渶瑕?16K RAM 鍧椾腑鐨?2K銆傝 16K 鍧楃殑鍩哄潃鍙綅浜庡叓涓綅缃腑鐨勪换鎰忎竴涓€?
寮€鍏崇粍 SW2 鐨?1-3 鍙峰紑鍏抽€夋嫨 16K 鍧楃殑鍩哄潃銆?
锛? = DOWN锛? = UP锛?
涓嶈繃锛屾垜鍙兘楠岃瘉鍏朵腑涓ょ璁剧疆鈥︹€?



```

   Switch| Hex RAM | Hex ROM
   1 2 3 | Address | Address
   ------|---------|-----------
   0 0 0 |  E0000  |  E2000
   0 0 1 |  D0000  |  D2000  (Manufacturer's default)
   0 1 0 |  ?????  |  ?????
   0 1 1 |  ?????  |  ?????
   1 0 0 |  ?????  |  ?????
   1 0 1 |  ?????  |  ?????
   1 1 0 |  ?????  |  ?????
   1 1 1 |  ?????  |  ?????


```

##### 璁剧疆鑺傜偣 ID


缁?SW3 涓殑鍏釜寮€鍏崇敤浜庤缃妭鐐?ID銆傝繛鎺ュ埌缃戠粶鐨勬瘡涓妭鐐瑰繀椤诲叿鏈夊敮涓€鐨勮妭鐐?ID锛屼笖蹇呴』涓嶅悓浜?0銆?
1 鍙峰紑鍏充綔涓烘渶浣庢湁鏁堜綅锛圠SB锛夈€?
澶勪簬 DOWN 浣嶇疆鐨勫紑鍏充负 OFF锛?锛夛紝澶勪簬 UP 浣嶇疆鐨勫紑鍏充负 ON锛?锛夈€?

鑺傜偣 ID 鏄墍鏈夋嫧鍒?"1" 鐨勫紑鍏冲彇鍊间箣鍜?

```

    Switch | Value
    -------|-------
      1    |   1
      2    |   2
      3    |   4
      4    |   8
      5    |  16
      6    |  32
      7    |  64
      8    | 128

```
```

      Switch#     |   Hex   | Decimal
  8 7 6 5 4 3 2 1 | Node ID | Node ID
  ----------------|---------|---------
  0 0 0 0 0 0 0 0 |    not allowed  <-.
  0 0 0 0 0 0 0 1 |    1    |    1    |
  0 0 0 0 0 0 1 0 |    2    |    2    |
  0 0 0 0 0 0 1 1 |    3    |    3    |
      . . .       |         |         |
  0 1 0 1 0 1 0 1 |   55    |   85    |
      . . .       |         |         + Don't use 0 or 255!
  1 0 1 0 1 0 1 0 |   AA    |  170    |
      . . .       |         |         |
  1 1 1 1 1 1 0 1 |   FD    |  253    |
  1 1 1 1 1 1 1 0 |   FE    |  254    |
  1 1 1 1 1 1 1 1 |   FF    |  255  <-'


```

## Tiara


### 锛堝瀷鍙锋湭鐭ワ級


  - 鏉ヨ嚜 Christoph Lameter <cl@gentwo.org>



```


  ----------------------------------------------- tiara
  Tiara LanCard of Tiara Computer Systems.

  +----------------------------------------------+
  !           ! Transmitter Unit !               !
  !           +------------------+             -------
  !          MEM                              Coax Connector
  !  ROM    7654321 <- I/O                     -------
  !  :  :   +--------+                           !
  !  :  :   ! 90C66LJ!                         +++
  !  :  :   !        !                         !D  Switch to set
  !  :  :   !        !                         !I  the Nodenumber
  !  :  :   +--------+                         !P
  !                                            !++
  !         234567 <- IRQ                      !
  +------------!!!!!!!!!!!!!!!!!!!!!!!!--------+
	       !!!!!!!!!!!!!!!!!!!!!!!!

```

- 0 = 宸插畨瑁呰烦绾?
- 1 = 鏂紑锛堝紑璺級

椤堕儴璺崇嚎鎺?浣?7 = ROM 浣胯兘锛?54 = 鍐呭瓨浣嶇疆锛?21 = I/O

鍐呭瓨浣嶇疆璁剧疆锛堥《閮ㄨ烦绾挎帓锛?

===     ================
456     Address selected
===     ================
000	C0000
001     C4000
010     CC000
011     D0000
100     D4000
101     D8000
110     DC000
111     E0000
===     ================

I/O 鍦板潃璁剧疆锛堥《閮ㄨ烦绾挎帓锛?

===     ====
123     Port
===     ====
000	260
001	290
010	2E0
011	2F0
100	300
101	350
110	380
111	3E0
===     ====

IRQ 閫夋嫨璁剧疆锛堝簳閮ㄨ烦绾挎帓锛?

====== =====
234567
====== =====
011111 IRQ 2
101111 IRQ 3
110111 IRQ 4
111011 IRQ 5
111110 IRQ 7
====== =====

## 鍏朵粬缃戝崱


鐩墠鎴戞病鏈変换浣曞叧浜庡叾浠栧瀷鍙?ARCnet 缃戝崱鐨勪俊鎭€?

鎰熻阿銆?
