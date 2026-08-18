
## PROPER CARE AND FEEDING OF RETURN VALUES FROM rcu_dereference()

鍚堢悊鍦扮淮鎶ゅ拰浣跨敤鏉ヨ嚜 rcu_dereference() 鐨勮繑鍥炲€?
姝ｇ‘缁存姢鍜屼娇鐢ㄥ湴鍧€渚濊禆涓庢暟鎹緷璧栵紝瀵规纭娇鐢?RCU 杩欑被鏈哄埗鑷冲叧閲嶈銆備负姝わ紝
rcu_dereference() 绯诲垪鍘熻杩斿洖鐨勬寚閽堜細鎼哄甫鍦板潃渚濊禆涓庢暟鎹緷璧栥€傝繖浜涗緷璧栦粠
rcu_dereference() 瀹忓姞杞借鎸囬拡寮€濮嬶紝涓€鐩村欢浼稿埌鍚庣画浣跨敤璇ユ寚閽堝幓璁＄畻鏌愭鍐呭瓨
璁块棶鐨勫湴鍧€锛堝搴斿湴鍧€渚濊禆锛夋垨璁＄畻鏌愭鍐呭瓨璁块棶鎵€鍐欏叆鐨勫€硷紙瀵瑰簲鏁版嵁渚濊禆锛夈€?
澶у鏁版儏鍐典笅锛岃繖浜涗緷璧栦細琚繚鐣欙紝浣夸綘鍙互鑷敱鍦颁娇鐢?rcu_dereference() 鐨勮繑鍥炲€笺€?渚嬪锛岃В寮曠敤锛堝墠缂€ `*`锛夈€佸瓧娈甸€夋嫨锛坄->`锛夈€佽祴鍊硷紙`=`锛夈€佸彇鍦板潃锛坄&`锛夈€佺被鍨嬭浆鎹€?浠ュ強瀵瑰父閲忓仛鍔犳硶鎴栧噺娉曪紝閮借兘寰堣嚜鐒躲€佸緢瀹夊叏鍦拌繘琛屻€備絾鏄紝鐢变簬褰撳墠鐨勭紪璇戝櫒
骞朵笉浼氳€冭檻鍦板潃渚濊禆鎴栨暟鎹緷璧栵紝浣犱粛鏈夊彲鑳介櫡鍏ラ夯鐑︺€?
璇烽伒寰互涓嬭鍒欐潵淇濈暀婧愯嚜 rcu_dereference() 鍙婂叾鐩稿叧璋冪敤鐨勫湴鍧€渚濊禆涓庢暟鎹緷璧栵紝
浠庤€屼繚璇佷綘鐨?RCU 璇昏€呮纭繍琛岋細

- 浣犲繀椤讳娇鐢?rcu_dereference() 绯诲垪鍘熻涔嬩竴鏉ュ姞杞藉彈 RCU 淇濇姢鐨勬寚閽堬紝鍚﹀垯
	CONFIG_PROVE_RCU 浼氬彂鍑鸿鍛娿€傛洿绯熺硶鐨勬槸锛岀敱浜庣紪璇戝櫒鍜?DEC Alpha 鍙兘
	鐜╃殑涓€浜涙妸鎴忥紝浣犵殑浠ｇ爜浼氬嚭鐜伴殢鏈虹殑鍐呭瓨鐮村潖閿欒銆傚鏋滄病鏈?rcu_dereference()
	绯诲垪鍘熻锛岀紪璇戝櫒鍙互閲嶆柊鍔犺浇璇ュ€硷紝鑰屼綘鐨勪唬鐮侀潰瀵瑰悓涓€涓寚閽堢殑涓や釜涓嶅悓鍊?	宀備笉鏄細涔卞锛佸鏋滄病鏈?rcu_dereference()锛孌EC Alpha 鍙互鍔犺浇涓€涓寚閽堛€?	瑙ｅ紩鐢ㄨ鎸囬拡锛屽苟杩斿洖璇ユ寚閽堝瓨鍌ㄤ箣鍓嶃€佸垵濮嬪寲涔嬪墠鐨勬棫鏁版嵁銆傦紙濡傚墠鏂囨墍杩帮紝
	鍦ㄨ繎鏈熺殑鏍镐腑锛孯EAD_ONCE() 涔熻兘闃绘 DEC Alpha 鐜╄繖浜涙妸鎴忋€傦級

	姝ゅ锛宺cu_dereference() 涓殑 volatile 寮哄埗杞崲鍙樆姝㈢紪璇戝櫒鎺ㄥ鍑烘墍寰楁寚閽堢殑鍊笺€?	璇峰弬瑙佹爣棰樹负"EXAMPLE WHERE THE COMPILER KNOWS TOO MUCH"鐨勫皬鑺傦紝鍏朵腑缁欏嚭浜?	缂栬瘧鍣ㄧ‘瀹炶兘澶熸帹瀵煎嚭鎸囬拡绮剧‘鍊笺€佷粠鑰岄€犳垚涔卞簭鐨勭ず渚嬨€?
- 鍦ㄨ繖鏍蜂竴绉嶇壒娈婃儏鍐典腑锛氭暟鎹彧浼氳娣诲姞銆佽€屽湪璇昏€呰闂缁撴瀯鏈熼棿姘歌繙涓嶄細琚Щ闄わ紝
	鍙互浣跨敤 READ_ONCE() 鏉ヤ唬鏇?rcu_dereference()銆傚湪姝ゆ儏鍐典笅锛屼娇鐢?READ_ONCE()
	鎵紨浜?v4.15 涓绉婚櫎鐨?lockless_dereference() 鍘熻鐨勮鑹层€?
- 浣犲彧鑳藉湪鎸囬拡鍊间笂浣跨敤 rcu_dereference()銆傜紪璇戝櫒瀵规暣鍨嬪€肩煡閬撳緱澶浜嗭紝涓嶅€煎緱
	淇′换瀹冧細閫氳繃鏁存暟杩愮畻鎵胯浇渚濊禆銆傛湁鏋佸皯鏁颁緥澶栵紝鍗充綘鍙互涓存椂灏嗘寚閽堣浆鎹负
	uintptr_t锛屼互渚匡細

 - 鍦ㄨ鎸囬拡蹇呴』涓洪浂鐨勪綆浣嶆瘮鐗逛腑缃綅鍜屾竻闆躲€傝繖鏄剧劧鎰忓懗鐫€璇ユ寚閽堝繀椤诲叿鏈夊榻?		绾︽潫锛屼緥濡傦紝杩欓€氬父瀵?char* 鎸囬拡鏄?*涓?*閫傜敤鐨勩€?
 - 瀵规寚閽堝仛寮傛垨杩愮畻鏉ヨ浆鎹㈡寚閽堬紝姝ｅ涓€浜涚粡鍏哥殑浼欎即鍒嗛厤鍣ㄧ畻娉曚腑鎵€鍋氱殑閭ｆ牱銆?
	鍦ㄦ墽琛屼换浣曞叾浠栨搷浣滀箣鍓嶏紝灏嗗€艰浆鎹㈠洖鎸囬拡鏄緢閲嶈鐨勩€?
- 浣跨敤 `+` 鍜?`-` 涓紑绠楁湳杩愮畻绗︽椂閬垮厤鐩告秷銆備緥濡傦紝瀵逛簬缁欏畾鍙橀噺 "x"锛屽 char*
	鎸囬拡閬垮厤浣跨敤 `(x-(uintptr_t)x)`銆傜紪璇戝櫒鏈夋潈鐢ㄩ浂鏇挎崲姝ょ被琛ㄨ揪寮忥紝浠庤€屼娇寰?	鍚庣画璁块棶涓嶅啀渚濊禆 rcu_dereference()锛岀敱姝ゅ彲鑳藉啀娆″洜涔卞簭瀵艰嚧閿欒銆?
	褰撶劧锛屽鏋?"p" 鏄潵鑷?rcu_dereference() 鐨勬寚閽堬紝鑰?"a" 鍜?"b" 鏄伆濂界浉绛夌殑
	鏁存暟锛岄偅涔堣〃杈惧紡 "p+a-b" 鏄畨鍏ㄧ殑锛屽洜涓哄叾鍊煎繀鐒朵粛渚濊禆 rcu_dereference()锛?	浠庤€岀淮鎸佷簡姝ｇ‘鐨勯『搴忋€?
- 濡傛灉浣犱娇鐢?RCU 鏉ヤ繚鎶?JIT 缂栬瘧鐨勫嚱鏁帮紝浣垮緱 `()` 鍑芥暟璋冪敤杩愮畻绗﹁搴旂敤鍒?	锛堢洿鎺ュ湴鎴栭棿鎺ュ湴锛変粠 rcu_dereference() 鑾峰緱鐨勫€间笂锛屼綘鍙兘闇€瑕佺洿鎺ヤ笌纭欢
	浜や簰浠ュ埛鏂版寚浠ょ紦瀛樸€傚綋鏂?JIT 鐨勫嚱鏁颁娇鐢ㄤ簡鏃╁厛鏌愪釜 JIT 鍑芥暟鎵€鐢ㄧ殑鍚屼竴鍧?	鍐呭瓨鏃讹紝鏌愪簺绯荤粺涓婁細鍑虹幇姝ら棶棰樸€?
- 瑙ｅ紩鐢ㄦ椂涓嶈浣跨敤鍏崇郴杩愮畻绗︼紙`==`銆乣!=`銆乣>`銆乣>=`銆乣<` 鎴?`<=`锛夌殑缁撴灉銆備緥濡傦紝

```
		int *p;
		int *q;

		...

		p = rcu_dereference(gp)
		q = &global_q;
		q += p > &oom_p;
		r1 = *q;  /* BUGGY!!! */

```

	濡傚墠鎵€杩帮紝杩欑鍋氭硶鏈?bug 鐨勫師鍥犳槸鍏崇郴杩愮畻绗﹂€氬父琚紪璇戞垚鍒嗘敮銆傚悓鏍峰鍓嶆枃鎵€杩帮紝
	铏界劧鍍?ARM 鎴?PowerPC 杩欐牱鐨勫急鍐呭瓨鏈哄櫒浼氬杩欑被鍒嗘敮涔嬪悗鐨勫瓨鍌ㄦ帓搴忥紝浣嗗彲浠?	瀵瑰姞杞藉仛鎶曟満鎵ц锛屼粠鑰屽啀娆″彲鑳介€犳垚涔卞簭閿欒銆?
```
- 灏嗘潵鑷?rcu_dereference() 鐨勬寚閽堜笌闈?NULL 鍊艰繘琛屾瘮杈冩椂瑕侀潪甯稿皬蹇冦€傛濡?Linus
	Torvalds 鎵€瑙ｉ噴鐨勯偅鏍凤紝濡傛灉涓や釜鎸囬拡鐩哥瓑锛岀紪璇戝櫒鍙互鐢ㄤ綘鎷挎潵姣旇緝鐨勯偅涓寚閽?	鏇挎崲琚瘮杈冪殑鎸囬拡

		p = rcu_dereference(gp);
		if (p == &default_struct)
			do_default(p->a);

	鐢变簬缂栬瘧鍣ㄧ幇鍦ㄧ煡閬?"p" 鐨勫€兼伆濂芥槸鍙橀噺 "default_struct" 鐨勫湴鍧€锛屽畠鍙互鑷敱鍦?	灏嗚繖娈典唬鐮佽浆鎹负濡備笅褰㈠紡锛?
		p = rcu_dereference(gp);
		if (p == &default_struct)
			do_default(default_struct.a);

	鍦?ARM 鍜?Power 纭欢涓婏紝瀵?"default_struct.a" 鐨勫姞杞界幇鍦ㄥ彲鑳借鎶曟満鎵ц锛?	浠庤€屽彲鑳藉彂鐢熷湪 rcu_dereference() 涔嬪墠銆傝繖鍙兘浼氬洜涔卞簭鑰屽鑷撮敊璇€?
	浣嗘槸锛屽湪浠ヤ笅鎯呭喌涓嬭繘琛屾瘮杈冩槸瀹夊叏鐨勶細

	-	涓?NULL 鎸囬拡杩涜姣旇緝銆傚鏋滅紪璇戝櫒鐭ラ亾璇ユ寚閽堜负 NULL锛屼綘鏈潵灏变笉璇?		鍘昏В寮曠敤瀹冦€傚鏋滄瘮杈冪粨鏋滄槸闈炵浉绛夛紝缂栬瘧鍣ㄤ篃涓嶆洿鑱槑銆傚洜姝わ紝
		灏嗘潵鑷?rcu_dereference() 鐨勬寚閽堜笌 NULL 鎸囬拡杩涜姣旇緝鏄畨鍏ㄧ殑銆?
	-	璇ユ寚閽堝湪琚瘮杈冧箣鍚庢案杩滀笉浼氳瑙ｅ紩鐢ㄣ€傜敱浜庝笉瀛樺湪鍚庣画鐨勮В寮曠敤锛岀紪璇戝櫒
		鏃犳硶鍒╃敤瀹冧粠姣旇緝涓鍒扮殑浠讳綍淇℃伅鏉ラ噸鎺掗偅浜涘苟涓嶅瓨鍦ㄧ殑鍚庣画瑙ｅ紩鐢ㄣ€?		杩欑姣旇緝鍦ㄦ壂鎻忓彈 RCU 淇濇姢鐨勫惊鐜摼琛ㄦ椂缁忓父鍙戠敓銆?
		娉ㄦ剰锛屽鏋滄寚閽堟瘮杈冩槸鍦?RCU 璇昏€呬复鐣屽尯涔嬪瀹屾垚鐨勶紝涓旇鎸囬拡浠庢湭琚В寮曠敤锛?		鍒欏簲褰撲娇鐢?rcu_access_pointer() 浠ｆ浛 rcu_dereference()銆傚湪澶у鏁版儏鍐典笅锛?		鏈€濂界洿鎺ユ祴璇?rcu_access_pointer() 鐨勮繑鍥炲€硷紝鑰屼笉灏嗗叾璧嬬粰鍙橀噺锛屼互閬垮厤
		鎰忓鐨勮В寮曠敤銆?
		鍦?RCU 璇昏€呬复鐣屽尯鍐呴儴锛屽嚑涔庢病鏈夌悊鐢变娇鐢?rcu_access_pointer()銆?
	-	琚瘮杈冪殑鎸囬拡鎵€寮曠敤鐨勬槸"寰堜箙浠ュ墠"灏卞凡鍒濆鍖栫殑鍐呭瓨銆傝繖绉嶆儏鍐靛畨鍏ㄧ殑
		鍘熷洜鏄紝鍗充娇鍙戠敓浜嗕贡搴忥紝杩欑涔卞簭涔熶笉浼氬奖鍝嶆瘮杈冧箣鍚庣殑閭ｄ簺璁块棶銆傞偅涔?		"寰堜箙浠ュ墠"鍒板簳鏄涔咃紵浠ヤ笅鏄竴浜涘彲鑳芥€э細

		-	缂栬瘧鏈熴€?
		-	鍚姩鏈熴€?
		-	妯″潡浠ｇ爜鐨勬ā鍧楀垵濮嬪寲鏈熴€?
		-	鍦?kthread 鍒涘缓涔嬪墠銆侀拡瀵?kthread 浠ｇ爜鐨勬椂鏈熴€?
		-	鍦ㄦ垜浠綋鍓嶆寔鏈夌殑鏌愪釜閿佺殑鏌愭杈冩棭鑾峰彇鏈熼棿銆?
		-	鍦ㄥ畾鏃跺櫒澶勭悊鍑芥暟鐨?mod_timer() 涔嬪墠銆?
		Linux 鍐呮牳鏈夊ぇ閲忕殑鍘熻浼氬鑷翠唬鐮佸湪绋嶅悗鏌愪釜鏃跺埢琚皟鐢紝杩樻湁璁稿鍏朵粬
		鍙兘鎬с€?
	-	琚瘮杈冪殑閭ｄ釜鎸囬拡鍚屾牱鏉ヨ嚜浜?rcu_dereference()銆傚湪杩欑鎯呭喌涓嬶紝涓や釜鎸囬拡
		閮戒緷璧栨煇涓?rcu_dereference() 鎴栧彟涓€涓紝鍥犳鏃犺鍝鏂瑰紡浣犻兘鑳借幏寰?		姝ｇ‘鐨勯『搴忋€?
		璇濊櫧濡傛锛岃繖绉嶆儏鍐靛彲鑳戒娇鏌愪簺 RCU 浣跨敤閿欒鏇存湁鍙兘鍙戠敓銆傝嫢杩欎簺閿欒
		鍙戠敓鍦ㄦ祴璇曟湡闂达紝杩欏€掑彲鑳芥槸浠跺ソ浜嬨€傛绫?RCU 浣跨敤閿欒鐨勪竴涓ず渚嬭鏍囬涓?		"EXAMPLE OF AMPLIFIED RCU-USAGE BUG"鐨勫皬鑺傘€?
	-	姣旇緝涔嬪悗鐨勬墍鏈夎闂兘鏄瓨鍌紝浠庤€屾帶鍒朵緷璧栦繚鐣欎簡鎵€闇€鐨勯『搴忋€傝瘽铏藉姝わ紝
		鎺у埗渚濊禆寰堝鏄撶敤閿欍€傛洿澶氱粏鑺傝鍙傞槄 Documentation/memory-barriers.txt
		涓殑"CONTROL DEPENDENCIES"灏忚妭銆?
	-	鎸囬拡涓嶇浉绛?涓?缂栬瘧鍣ㄦ病鏈夎冻澶熶俊鎭帹瀵煎嚭璇ユ寚閽堢殑鍊笺€傛敞鎰忥紝rcu_dereference()
		涓殑 volatile 寮哄埗杞崲閫氬父浼氶樆姝㈢紪璇戝櫒鐭ラ亾澶淇℃伅銆?
		浣嗘槸锛岃娉ㄦ剰锛屽鏋滅紪璇戝櫒鐭ラ亾璇ユ寚閽堝彧鍙栦袱涓€间箣涓€锛岄偅涔堜竴涓潪鐩哥瓑姣旇緝
		鎭板ソ浼氭彁渚涚紪璇戝櫒鎺ㄥ鍑鸿鎸囬拡鍊兼墍闇€鐨勪俊鎭€?
```

- 鍏抽棴缂栬瘧鍣ㄥ彲鑳芥彁渚涚殑浠讳綍鍊兼姇鏈轰紭鍖栵紝鐗瑰埆鏄綋浣犱娇鐢ㄤ簡鍩轰簬鍙嶉鐨勩€佷粠鍏堝墠
	杩愯鏀堕泦鏁版嵁鐨勪紭鍖栨椂銆傝繖绫诲€兼姇鏈轰紭鍖栧湪璁捐涓婂氨鏄噸鎺掓搷浣滅殑銆?
	杩欐潯瑙勫垯鏈変竴涓緥澶栵細鍒╃敤鍒嗘敮棰勬祴纭欢鐨勫€兼姇鏈轰紭鍖栧湪寮哄簭绯荤粺锛堝 x86锛変笂鏄?	瀹夊叏鐨勶紝浣嗗湪寮卞簭绯荤粺锛堝 ARM 鎴?Power锛変笂涓嶅畨鍏ㄣ€傝鏄庢櫤鍦伴€夋嫨浣犵殑缂栬瘧鍣?	鍛戒护琛岄€夐」锛?

### EXAMPLE OF AMPLIFIED RCU-USAGE BUG

RCU 浣跨敤閿欒琚斁澶х殑绀轰緥

鐢变簬鏇存柊鑰呭彲浠ヤ笌 RCU 璇昏€呭苟鍙戣繍琛岋紝RCU 璇昏€呭彲鑳界湅鍒伴檲鏃у拰涓嶄竴鑷寸殑鍊笺€傚鏋?RCU
璇昏€呴渶瑕佹柊椴滄垨涓€鑷寸殑鍊硷紙鏈夋椂纭疄闇€瑕侊級锛屽畠浠渶瑕佹纭湴杩涜

```

	struct foo {
		int a;
		int b;
		int c;
	};
	struct foo *gp1;
	struct foo *gp2;

	void updater(void)
	{
		struct foo *p;

		p = kmalloc(...);
		if (p == NULL)
			deal_with_it();
		p->a = 42;  /* Each field in its own cache line. */
		p->b = 43;
		p->c = 44;
		rcu_assign_pointer(gp1, p);
		p->b = 143;
		p->c = 144;
		rcu_assign_pointer(gp2, p);
	}

	void reader(void)
	{
		struct foo *p;
		struct foo *q;
		int r1, r2;

		rcu_read_lock();
		p = rcu_dereference(gp2);
		if (p == NULL)
			return;
		r1 = p->b;  /* Guaranteed to get 143. */
		q = rcu_dereference(gp1);  /* Guaranteed non-NULL. */
		if (p == q) {
			/* The compiler decides that q->c is same as p->c. */
			r2 = p->c; /* Could get 44 on weakly order system. */
		} else {
			r2 = p->c - r1; /* Unconditional access to p->c. */
		}
		rcu_read_unlock();
		do_something_with(r1, r2);
	}

```

浣犲彲鑳戒細瀵圭粨鏋?(r1 == 143 && r2 == 44) 鏄彲鑳界殑鎰熷埌鎯婅锛屼絾浣犱笉搴旀儕璁躲€傛瘯绔燂紝
鏇存柊鑰呭彲鑳藉湪 reader() 鎶婂€艰浇鍏?"r1" 涓庤浇鍏?"r2" 涔嬮棿琚浜屾璋冪敤銆傜敱浜庣紪璇戝櫒
鍜?CPU 鐨勬煇浜涢噸鎺掞紝鍚屾牱鐨勭粨鏋滀篃鍙兘鍑虹幇锛岃繖涓€鐐瑰€掓棤鍏崇揣瑕併€?
浣嗗鏋滆鑰呴渶瑕佷竴鑷寸殑瑙嗗浘鍛紵

```

	struct foo {
		int a;
		int b;
		int c;
		spinlock_t lock;
	};
	struct foo *gp1;
	struct foo *gp2;

	void updater(void)
	{
		struct foo *p;

		p = kmalloc(...);
		if (p == NULL)
			deal_with_it();
		spin_lock(&p->lock);
		p->a = 42;  /* Each field in its own cache line. */
		p->b = 43;
		p->c = 44;
		spin_unlock(&p->lock);
		rcu_assign_pointer(gp1, p);
		spin_lock(&p->lock);
		p->b = 143;
		p->c = 144;
		spin_unlock(&p->lock);
		rcu_assign_pointer(gp2, p);
	}

	void reader(void)
	{
		struct foo *p;
		struct foo *q;
		int r1, r2;

		rcu_read_lock();
		p = rcu_dereference(gp2);
		if (p == NULL)
			return;
		spin_lock(&p->lock);
		r1 = p->b;  /* Guaranteed to get 143. */
		q = rcu_dereference(gp1);  /* Guaranteed non-NULL. */
		if (p == q) {
			/* The compiler decides that q->c is same as p->c. */
			r2 = p->c; /* Locking guarantees r2 == 144. */
		} else {
			spin_lock(&q->lock);
			r2 = q->c - r1;
			spin_unlock(&q->lock);
		}
		rcu_read_unlock();
		spin_unlock(&p->lock);
		do_something_with(r1, r2);
	}

```

涓€濡傛棦寰€锛岄€夌敤鍚堥€傜殑宸ュ叿鏉ュ畬鎴愬伐浣滐紒


### EXAMPLE WHERE THE COMPILER KNOWS TOO MUCH

缂栬瘧鍣ㄧ煡閬撳緱澶鐨勭ず渚?
濡傛灉浠?rcu_dereference() 鑾峰緱鐨勬寚閽堜笌鏌愪釜鍏朵粬鎸囬拡姣旇緝涓洪潪鐩哥瓑锛岀紪璇戝櫒閫氬父
鏃犱粠寰楃煡绗竴涓寚閽堢殑鍊煎彲鑳芥槸浠€涔堛€傝繖绉嶄俊鎭己澶遍樆姝簡缂栬瘧鍣ㄦ墽琛岄偅浜涙湰鏉ュ彲鑳?鐮村潖 RCU 鎵€渚濊禆鐨勯『搴忎繚璇佺殑浼樺寲銆傝€?rcu_dereference() 涓殑 volatile 寮哄埗杞崲
搴斿綋鑳介樆姝㈢紪璇戝櫒鐚滄祴璇ュ€笺€?
浣嗘槸锛屽鏋滄病鏈?rcu_dereference()锛岀紪璇戝櫒鐭ラ亾鐨勫彲鑳芥瘮浣犳兂璞＄殑鏇村

```

	struct foo {
		int a;
		int b;
	};
	static struct foo variable1;
	static struct foo variable2;
	static struct foo *gp = &variable1;

	void updater(void)
	{
		initialize_foo(&variable2);
		rcu_assign_pointer(gp, &variable2);
		/*
		 * The above is the only store to gp in this translation unit,
		 * and the address of gp is not exported in any way.
		 */
	}

	int reader(void)
	{
		struct foo *p;

		p = gp;
		barrier();
		if (p == &variable1)
			return p->a; /* Must be variable1.a. */
		else
			return p->b; /* Must be variable2.b. */
	}

```

鐢变簬缂栬瘧鍣ㄨ兘鐪嬪埌瀵?"gp" 鐨勬墍鏈夊瓨鍌紝瀹冪煡閬?"gp" 鍙兘鐨勫€煎彧鏈?variable1 鍜?variable2 杩欎袱绉嶃€傚洜姝?reader() 涓殑姣旇緝鍗充究鍦ㄩ潪鐩哥瓑鐨勬儏鍐典笅锛屼篃鍛婅瘔浜嗙紪璇戝櫒
"p" 鐨勭簿纭€笺€傝繖浣垮緱缂栬瘧鍣ㄨ兘澶熶护杩斿洖鍊间笉渚濊禆浜庝粠 "gp" 鐨勫姞杞斤紝杩涜€岀牬鍧忎簡杩欐
鍔犺浇涓庨偅浜涜繑鍥炲€肩殑鍔犺浇涔嬮棿鐨勯『搴忓叧绯汇€傝繖浼氬鑷?"p->b" 鍦ㄥ急搴忕郴缁熶笂杩斿洖
鍒濆鍖栦箣鍓嶇殑鍨冨溇鍊笺€?
绠€鑰岃█涔嬶紝褰撲綘瑕佸幓瑙ｅ紩鐢ㄦ墍寰楁寚閽堟椂锛宺cu_dereference() **涓嶆槸**鍙湁鍙棤鐨勩€?

### WHICH MEMBER OF THE rcu_dereference() FAMILY SHOULD YOU USE?

浣犲簲璇ヤ娇鐢?rcu_dereference() 瀹舵棌涓殑鍝竴涓垚鍛橈紵

棣栧厛锛岃閬垮厤浣跨敤 rcu_dereference_raw()锛屼篃璇烽伩鍏嶄娇鐢ㄥ甫鏈夊父閲忓弬鏁板€?1锛堟垨
true锛夌殑 rcu_dereference_check() 鍜?rcu_dereference_protected()銆傚湪缁欏嚭杩欎竴
璀﹀憡涔嬪悗锛屼互涓嬫槸涓€浜涘叧浜庡湪鍚勭鎯呭舰涓嬩娇鐢?rcu_dereference() 鍝釜鎴愬憳鐨勬寚瀵硷細

1. 濡傛灉璁块棶闇€瑕佷綅浜?RCU 璇昏€呬复鐣屽尯涔嬪唴锛屼娇鐢?rcu_dereference()銆傚湪鍚堝苟鍚庣殑
	鏂?RCU 鍙樹綋涓紝杩涘叆 RCU 璇昏€呬复鐣屽尯鏄€氳繃 rcu_read_lock()銆佷换浣曠鐢ㄥ簳鍗婇儴
	鐨勬搷浣溿€佷换浣曠鐢ㄤ腑鏂殑鎿嶄綔锛屾垨浠讳綍绂佺敤鎶㈠崰鐨勬搷浣滄潵瀹炵幇鐨勩€傝娉ㄦ剰锛岃嚜鏃嬮攣
	涓寸晫鍖轰篃闅愬惈涓?RCU 璇昏€呬复鐣屽尯锛屽嵆浣垮畠浠槸鍙姠鍗犵殑锛堝湪浣跨敤 CONFIG_PREEMPT_RT=y
	鏋勫缓鐨勫唴鏍镐腑涔熸槸濡傛锛夈€?
2. 濡傛灉璁块棶鍙兘浣嶄簬 RCU 璇昏€呬复鐣屽尯涔嬪唴锛堜竴鏂归潰锛夛紝鎴栬€呭彈锛堟瘮濡傝锛塵y_lock 淇濇姢
	锛堝彟涓€鏂归潰锛夛紝浣跨敤

```

		p1 = rcu_dereference_check(p->rcu_protected_pointer,
					   lockdep_is_held(&my_lock));

```

3. 濡傛灉璁块棶鍙兘浣嶄簬 RCU 璇昏€呬复鐣屽尯涔嬪唴锛堜竴鏂归潰锛夛紝鎴栬€呭彈 my_lock 鎴?your_lock
	浜岃€呬箣涓€淇濇姢锛堝彟涓€鏂归潰锛夛紝浣跨敤

```

		p1 = rcu_dereference_check(p->rcu_protected_pointer,
					   lockdep_is_held(&my_lock) ||
					   lockdep_is_held(&your_lock));

```

4. 濡傛灉璁块棶浣嶄簬鏇存柊渚э紝鍥犺€屽缁堝彈鍒颁繚鎶わ紝浣跨敤

```

		p1 = rcu_dereference_protected(p->rcu_protected_pointer,
					       lockdep_is_held(&my_lock));

```

	杩欏彲浠ュ儚涓婇潰鐨?#3 閭ｆ牱鎵╁睍鍒板鐞嗗涓攣锛屼袱鑰呬篃閮借兘鎵╁睍涓烘鏌ュ叾浠栨潯浠躲€?
	5. 濡傛灉淇濇姢鏄敱璋冪敤鑰呮彁渚涚殑銆佸洜姝ゆ湰浠ｇ爜鏃犱粠寰楃煡锛岄偅灏辨槸鏋佸皯闇€瑕佷娇鐢?	rcu_dereference_raw() 鐨勬儏褰€傛澶栵紝褰?lockdep 琛ㄨ揪寮忎細杩囧垎澶嶆潅鏃讹紝
	rcu_dereference_raw() 鍙兘鏄悎閫傜殑锛屼笉杩囪繖绉嶆儏鍐典笅鏇村ソ鐨勫姙娉曚篃璁告槸濂藉ソ
	瀹¤涓€涓嬩綘鐨勫悓姝ヨ璁°€傚敖绠″姝わ紝杩樻槸瀛樺湪杩欐牱鐨勬暟鎹姞閿佹儏褰細鏋佸ぇ鏁伴噺鐨?	閿佹垨寮曠敤璁℃暟涓殑浠绘剰涓€涓兘瓒充互淇濇姢璇ユ寚閽堬紝鍥犳 rcu_dereference_raw() 纭湁
	鍏剁敤姝︿箣鍦般€?
	涓嶈繃锛屽畠鐨勭敤姝︿箣鍦板彲鑳芥瘮浣犱緷鎹綋鍓嶅唴鏍镐腑鐨勪娇鐢ㄦ鏁版墍棰勬湡鐨勮灏忓緱澶氥€?	瀹冪殑鍚屼箟璇?rcu_dereference_check( ... , 1)锛屼互鍙婂畠鐨勮繎浜?	rcu_dereference_protected(... , 1)锛屼篃鏄姝ゃ€?

### SPARSE CHECKING OF RCU-PROTECTED POINTERS

瀵瑰彈 RCU 淇濇姢鐨勬寚閽堝仛 sparse 妫€鏌?
sparse 闈欐€佸垎鏋愬伐鍏蜂細妫€鏌ュ鍙?RCU 淇濇姢鎸囬拡鐨勯潪 RCU 璁块棶锛岃繖绫昏闂彲鑳藉洜娑夊強
缂栬瘧鍣ㄥ彂鏄庡姞杞姐€佹垨璁歌繕鏈夊姞杞藉垎瑁傦紙load tearing锛夌殑浼樺寲鑰屽鑷?鏈夎叮"鐨?bug銆?
```

	p = q->rcu_protected_pointer;
	do_something_with(p->a);
	do_something_else_with(p->b);

```

濡傛灉瀵勫瓨鍣ㄥ帇鍔涘緢楂橈紝缂栬瘧鍣ㄥ彲鑳戒細鎶?"p" 浼樺寲鎺?
```

	do_something_with(q->rcu_protected_pointer->a);
	do_something_else_with(q->rcu_protected_pointer->b);

```

濡傛灉 q->rcu_protected_pointer 鍦ㄦ鏈熼棿鍙戠敓浜嗘敼鍙橈紝杩欏彲鑳戒細鑷村懡鍦颁护浣犵殑浠ｇ爜澶辨湜銆?鑰屼笖杩欏苟闈炵悊璁洪棶棰橈細鎭版伆杩欑被 bug 鍦?1990 骞翠唬鍒濊 Paul E. McKenney锛堜互鍙婁粬鐨?鍑犱綅鏃犺緶鍚屼簨锛夋惌涓婁簡涓€涓笁澶╃殑鍛ㄦ湯銆?
鍔犺浇鍒嗚褰撶劧鍙兘瀵艰嚧瑙ｅ紩鐢ㄤ竴瀵规寚閽堣绯呭悎鐨勭粨鏋滐紝杩欏悓鏍峰彲鑳借嚧鍛藉湴浠や綘鐨勪唬鐮佸け鏈涖€?
杩欎簺闂鏈彲浠ラ€氳繃绠€鍗曞湴璁╀唬鐮佹敼涓哄涓嬪舰寮忔潵閬垮厤

```

	p = rcu_dereference(q->rcu_protected_pointer);
	do_something_with(p->a);
	do_something_else_with(p->b);

```

閬楁喚鐨勬槸锛岃繖绫?bug 鍦ㄨ瘎瀹℃椂鏋侀毦鍙戠幇銆傝繖姝ｆ槸 sparse 宸ュ叿浠ュ強 "__rcu" 鏍囪鐨?鐢ㄦ涔嬪湴銆傚鏋滀綘缁欎竴涓寚閽堝０鏄庯紙鏃犺鏄湪缁撴瀯浣撲腑杩樻槸浣滀负褰㈠弬锛夊姞涓?"__rcu"锛?灏卞憡璇?sparse 鍦ㄨ鎸囬拡琚洿鎺ヨ闂椂鍙戝嚭璀﹀憡銆傚鏋滄煇涓湭鏍囪 "__rcu" 鐨勬寚閽堣
rcu_dereference() 鍙婂叾鐩稿叧鍘熻璁块棶锛屽畠涔熶細璁?sparse 鍙戝嚭璀﹀憡銆備緥濡傦紝
->rcu_protected_pointer 鍙兘琚０鏄庝负

```

	struct foo __rcu *rcu_protected_pointer;

```

浣跨敤 "__rcu" 鏄€夋嫨鍔犲叆锛坥pt-in锛夌殑銆傚鏋滀綘閫夋嫨涓嶄娇鐢ㄥ畠锛岄偅涔堜綘搴旇蹇界暐 sparse
鐨勮鍛娿€?