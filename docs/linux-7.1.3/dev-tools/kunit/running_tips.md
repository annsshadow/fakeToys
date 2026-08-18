
## 杩愯 KUnit 娴嬭瘯鐨勬彁绀?

## 浣跨敤 ``kunit.py run``锛?kunit 宸ュ叿"锛?

### 浠庝换鎰忕洰褰曡繍琛?

鍒涘缓涓€涓被浼间笅闈㈣繖鏍风殑 bash 鍑芥暟浼氬緢鏂逛究锛?

	function run_kunit() {
	  ( cd "$(git rev-parse --show-toplevel)" && ./tools/testing/kunit/kunit.py run "$@" )
	}

	`kunit.py` 鐨勬棭鏈熺増鏈紙5.6 涔嬪墠锛夊彧鏈夊湪浠庡唴鏍告牴鐩綍杩愯鏃舵墠宸ヤ綔锛屽洜姝よ繖閲屼娇鐢ㄤ簡瀛?shell 鍜?`cd`銆?
### 杩愯娴嬭瘯瀛愰泦


`kunit.py run` 鎺ュ彈涓€涓彲閫夌殑 glob 鍙傛暟鏉ヨ繃婊ゆ祴璇曘€傛牸寮忎负 `"<suite_glob>[.test_glob]"`銆?
鍋囪鎴戜滑鎯宠繍琛?sysctl 娴嬭瘯锛屽彲浠ヨ繖鏍凤細


	$ echo -e 'CONFIG_KUNIT=y\nCONFIG_KUNIT_ALL_TESTS=y' > .kunit/.kunitconfig
	$ ./tools/testing/kunit/kunit.py run 'sysctl*'

鎴戜滑鍙互閫氳繃浠ヤ笅鏂瑰紡杩涗竴姝ヨ繃婊わ紝鍙繍琛?write"娴嬭瘯锛?

	$ echo -e 'CONFIG_KUNIT=y\nCONFIG_KUNIT_ALL_TESTS=y' > .kunit/.kunitconfig
	$ ./tools/testing/kunit/kunit.py run 'sysctl**.**write*'

浠ヨ繖绉嶆柟寮忔垜浠粯鍑轰簡鏋勫缓澶氫簬鎵€闇€娴嬭瘯鐨勬垚鏈紝浣嗗畠姣旀憜寮?`.kunitconfig` 鏂囦欢鎴栨敞閲婃帀 `kunit_suite` 瑕佸鏄撱€?
涓嶈繃锛屽鏋滀綘鎯充互涓嶉偅涔堜复鏃剁殑鏂瑰紡鏉ュ畾涔変竴缁勬祴璇曪紝涓嬩竴鏉℃彁绀轰細寰堟湁鐢ㄣ€?
### 瀹氫箟涓€缁勬祴璇?

`kunit.py run`锛堜互鍙?`build` 鍜?`config`锛夋敮鎸佷竴涓?`--kunitconfig` 鏍囧織銆傚洜姝わ紝濡傛灉浣犳湁涓€缁勬兂瑕佸畾鏈熻繍琛岀殑娴嬭瘯锛堝挨鍏舵槸瀹冧滑杩樻湁鍏朵粬渚濊禆鏃讹級锛屽彲浠ヤ负瀹冧滑鍒涘缓涓€涓壒瀹氱殑 `.kunitconfig`銆?
渚嬪锛宬unit 涓哄叾娴嬭瘯灏辨湁涓€涓細


	$ ./tools/testing/kunit/kunit.py run --kunitconfig=lib/kunit/.kunitconfig

鎴栬€咃紝濡傛灉浣犻伒寰皢鏂囦欢鍛藉悕涓?`.kunitconfig` 鐨勭害瀹氾紝浣犲彲浠ュ彧浼犲叆鐩綍锛屼緥濡傦細


	$ ./tools/testing/kunit/kunit.py run --kunitconfig=lib/kunit

	杩欐槸涓€涓浉瀵硅緝鏂扮殑鐗规€э紙5.12+锛夛紝鍥犳鍏充簬鍝簺鏂囦欢搴斿綋妫€鍏ャ€佸摢浜涘彧淇濈暀鍦ㄦ湰鍦帮紝鎴戜滑杩樻病鏈変换浣曠害瀹氥€備竴涓厤缃槸鍚︽湁鐢ㄥ埌鍊煎緱鎻愪氦锛堝苟鍥犳蹇呴』缁存姢锛夛紝鐢变綘鍜屼綘鐨勭淮鎶よ€呭喅瀹氥€?
	鍦ㄧ埗鐩綍鍜屽瓙鐩綍涓悓鏃舵嫢鏈?`.kunitconfig` 鐗囨鏄垚闂鐨勩€傛湁浜哄湪璁ㄨ鍦ㄨ繖浜涙枃浠朵腑娣诲姞涓€鏉?import"璇彞锛屼互渚胯椤跺眰閰嶇疆鑳藉杩愯鏉ヨ嚜鎵€鏈夊瓙鐩綍鐨勬祴璇曘€備絾閭ｅ皢鎰忓懗鐫€ `.kunitconfig` 鏂囦欢涓嶅啀鏄畝鍗曠殑 .config 鐗囨銆?
	鍙︿竴绉嶆浛浠ｆ柟妗堟槸璁?kunit 宸ュ叿鑷姩閫掑綊鍚堝苟閰嶇疆锛屼絾娴嬭瘯鍦ㄧ悊璁轰笂鍙兘渚濊禆浜庝笉鍏煎鐨勯€夐」锛屽洜姝ゅ鐞嗚捣鏉ヤ細寰堟鎵嬨€?
### 璁剧疆鍐呮牳鍛戒护琛屽弬鏁?

浣犲彲浠ヤ娇鐢?`--kernel_args` 鏉ヤ紶閫掍换鎰忓唴鏍稿弬鏁帮紝渚嬪锛?

	$ ./tools/testing/kunit/kunit.py run --kernel_args=param=42 --kernel_args=param2=false


### 鍦?UML 涓嬬敓鎴愪唬鐮佽鐩栫巼鎶ュ憡


	TODO(brendanhiggins@google.com): UML 涓?gcc 7 鍙婃洿楂樼増鏈瓨鍦ㄥ悇绉嶉棶棰樸€備綘寰堝彲鑳戒細閬囧埌缂哄け鐨?`.gcda` 鏂囦欢鎴栫紪璇戦敊璇€?
杩欎笌 Documentation/dev-tools/gcov.rst 涓褰曠殑鑾峰彇瑕嗙洊鐜囦俊鎭殑"甯歌"鏂瑰紡涓嶅悓銆?
鎴戜滑鍙互涓嶅惎鐢?`CONFIG_GCOV_KERNEL=y`锛岃€屾槸璁剧疆杩欎簺閫夐」锛?

	CONFIG_DEBUG_KERNEL=y
	CONFIG_DEBUG_INFO=y
	CONFIG_DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT=y
	CONFIG_GCOV=y


灏嗗叾缁勫悎鎴愪竴涓彲澶嶅埗绮樿创鐨勫懡浠ゅ簭鍒楋細


	# 灏嗚鐩栫巼閫夐」杩藉姞鍒板綋鍓嶉厤缃?	$ ./tools/testing/kunit/kunit.py run --kunitconfig=.kunit/ --kunitconfig=tools/testing/kunit/configs/coverage_uml.config
	# 浠庢瀯寤虹洰褰曪紙.kunit/锛変腑鎻愬彇瑕嗙洊鐜囦俊鎭?	$ lcov -t "my_kunit_tests" -o coverage.info -c -d .kunit/

	# 浠庤繖閲屽紑濮嬶紝杩囩▼涓?CONFIG_GCOV_KERNEL=y 鏃剁浉鍚?	# 渚嬪锛屽彲浠ュ湪 tmp 鐩綍涓敓鎴?HTML 鎶ュ憡锛屽涓嬶細
	$ genhtml -o /tmp/coverage_html coverage.info


濡傛灉浣犲畨瑁呯殑 gcc 鐗堟湰涓嶅伐浣滐紝浣犲彲浠ヨ皟鏁存楠わ細


	$ ./tools/testing/kunit/kunit.py run --make_options=CC=/usr/bin/gcc-6
	$ lcov -t "my_kunit_tests" -o coverage.info -c -d .kunit/ --gcov-tool=/usr/bin/gcov-6

鎴栬€咃紝涔熷彲浠ヤ娇鐢ㄥ熀浜?LLVM 鐨勫伐鍏烽摼锛?

	# 浣跨敤 LLVM 鏋勫缓骞跺皢瑕嗙洊鐜囬€夐」杩藉姞鍒板綋鍓嶉厤缃?	$ ./tools/testing/kunit/kunit.py run --make_options LLVM=1 --kunitconfig=.kunit/ --kunitconfig=tools/testing/kunit/configs/coverage_uml.config
	$ llvm-profdata merge -sparse default.profraw -o default.profdata
	$ llvm-cov export --format=lcov .kunit/vmlinux -instr-profile default.profdata > coverage.info
	# coverage.info 鏂囦欢鏄?lcov 鍏煎鏍煎紡锛屽彲鐢ㄤ簬渚嬪鐢熸垚 HTML 鎶ュ憡
	$ genhtml -o /tmp/coverage_html coverage.info


## 鎵嬪姩杩愯娴嬭瘯


涓嶄娇鐢?`kunit.py run` 鏉ヨ繍琛屾祴璇曚篃鏄竴涓噸瑕佺殑浣跨敤鍦烘櫙銆傜洰鍓嶏紝濡傛灉浣犳兂鍦?UML 涔嬪鐨勬灦鏋勪笂娴嬭瘯锛岃繖鏄綘鍞竴鐨勯€夋嫨銆?
鐢变簬鍦?UML 涓嬭繍琛屾祴璇曠浉褰撶洿鎺ワ紙閰嶇疆骞剁紪璇戝唴鏍革紝杩愯 `./linux` 浜岃繘鍒讹級锛屾湰鑺傚皢鑱氱劍浜庢祴璇曢潪 UML 鏋舵瀯銆?

### 杩愯鍐呭缓娴嬭瘯


褰撳皢娴嬭瘯璁剧疆涓?`=y` 鏃讹紝娴嬭瘯浼氫綔涓哄惎鍔ㄧ殑涓€閮ㄥ垎杩愯锛屽苟浠?TAP 鏍煎紡灏嗙粨鏋滄墦鍗板埌 dmesg銆傚洜姝や綘鍙渶瑕佸儚寰€甯镐竴鏍峰皢娴嬭瘯鍔犲叆浣犵殑 `.config`锛屾瀯寤哄苟鍚姩鍐呮牳銆?
鍥犳锛屽鏋滄垜浠敤浠ヤ笅閰嶇疆缂栬瘧鍐呮牳锛?

	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=y

閭ｄ箞鎴戜滑浼氱湅鍒?dmesg 涓嚭鐜扮被浼煎涓嬬殑杈撳嚭锛岃〃鏄庢祴璇曞凡杩愯骞堕€氳繃锛?

	TAP version 14
	1..1
	    # Subtest: example
	    1..1
	    # example_simple_test: initializing
	    ok 1 - example_simple_test
	ok 1 - example

### 浠ユā鍧楁柟寮忚繍琛屾祴璇?

鏍规嵁娴嬭瘯鐨勪笉鍚岋紝浣犲彲浠ュ皢瀹冧滑鏋勫缓涓哄彲鍔犺浇妯″潡銆?
渚嬪锛屾垜浠皢涔嬪墠鐨勯厤缃€夐」鏀逛负


	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=m

鐒跺悗鍦ㄥ惎鍔ㄨ繘鍏ユ垜浠殑鍐呮牳涔嬪悗锛屾垜浠彲浠ラ€氳繃浠ヤ笅鏂瑰紡杩愯娴嬭瘯锛?

	$ modprobe kunit-example-test

闅忓悗瀹冨皢鍚?stdout 鎵撳嵃 TAP 杈撳嚭銆?
	`modprobe` 鍦ㄤ换浣曟祴璇曞け璐ユ椂锛堟埅鑷?5.13锛?*涓嶄細**鏈夐潪闆堕€€鍑虹爜銆備絾 `kunit.py parse` 浼氭湁锛岃涓嬫枃銆?
	浣犱篃鍙互璁剧疆 `CONFIG_KUNIT=m`锛屼絾鏄紝鏌愪簺鐗规€у皢涓嶈兘宸ヤ綔锛屽洜姝ゆ煇浜涙祴璇曞彲鑳戒細鍑洪敊銆傜悊鎯虫儏鍐典笅锛屾祴璇曚細鍦ㄥ叾 `Kconfig` 涓０鏄庡畠浠緷璧栦簬 `KUNIT=y`锛屼絾杩欐槸涓€涓ぇ澶氭暟娴嬭瘯浣滆€呬笉浼氳€冭檻鐨勮竟鐣屾儏鍐点€?	鎴嚦 5.13锛屽敮涓€鐨勫尯鍒槸 `current->kunit_test` 灏嗕笉瀛樺湪銆?
### 缇庡寲鎵撳嵃缁撴灉


浣犲彲浠ヤ娇鐢?`kunit.py parse` 鏉ヨВ鏋?dmesg 涓殑娴嬭瘯杈撳嚭锛屽苟浠?`kunit.py run` 閭ｆ牱鐔熸倝鐨勬牸寮忔墦鍗扮粨鏋溿€?

	$ ./tools/testing/kunit/kunit.py parse /var/log/dmesg


### 鑾峰彇姣忎釜娴嬭瘯濂椾欢鐨勭粨鏋?

鏃犺浣犲浣曡繍琛屾祴璇曪紝閮藉彲浠ュ惎鐢?`CONFIG_KUNIT_DEBUGFS` 鏉ュ鍑烘瘡涓浠朵互 TAP 鏍煎紡鍛堢幇鐨勭粨鏋滐細


	CONFIG_KUNIT=y
	CONFIG_KUNIT_EXAMPLE_TEST=m
	CONFIG_KUNIT_DEBUGFS=y

姣忎釜濂椾欢鐨勭粨鏋滃皢鏆撮湶鍦?`/sys/kernel/debug/kunit/<suite>/results` 涓嬨€傚洜姝や娇鐢ㄦ垜浠殑绀轰緥閰嶇疆锛?

	$ modprobe kunit-example-test > /dev/null
	$ cat /sys/kernel/debug/kunit/example/results
	... <TAP output> ...

	# 绉婚櫎妯″潡鍚庯紝鐩稿簲鐨勬枃浠朵細娑堝け
	$ modprobe -r kunit-example-test
	$ cat /sys/kernel/debug/kunit/example/results
	/sys/kernel/debug/kunit/example/results: No such file or directory

### 鐢熸垚浠ｇ爜瑕嗙洊鐜囨姤鍛?

璇﹁ Documentation/dev-tools/gcov.rst 浜嗚В濡備綍鎵ц姝ゆ搷浣溿€?
杩欓噷鍞竴鏈夌偣 KUnit 鐗规€х殑寤鸿鏄紝浣犲彲鑳藉笇鏈涘皢娴嬭瘯鏋勫缓涓烘ā鍧椼€傝繖鏍蜂綘鍙互灏嗘祴璇曠殑瑕嗙洊鐜囦笌鍚姩鏈熼棿鎵ц鐨勫叾浠栦唬鐮佺殑瑕嗙洊鐜囬殧绂诲紑锛屼緥濡傦細


	# 鍦ㄨ繍琛屾祴璇曞墠閲嶇疆瑕嗙洊鐜囪鏁板櫒銆?	$ echo 0 > /sys/kernel/debug/gcov/reset
	$ modprobe kunit-example-test


## 娴嬭瘯灞炴€т笌杩囨护


娴嬭瘯濂椾欢鍜屾祴璇曠敤渚嬪彲浠ョ敤娴嬭瘯灞炴€э紙渚嬪娴嬭瘯鐨勯€熷害锛夋潵鏍囪銆傝繖浜涘睘鎬х◢鍚庝細鎵撳嵃鍦ㄦ祴璇曡緭鍑轰腑锛屽苟鍙敤浜庤繃婊ゆ祴璇曟墽琛屻€?
### 鏍囪娴嬭瘯灞炴€?

閫氳繃鍦ㄦ祴璇曞畾涔変腑鍖呭惈涓€涓?`kunit_attributes` 瀵硅薄鏉ョ敤灞炴€ф爣璁版祴璇曘€?
娴嬭瘯鐢ㄤ緥鍙互浣跨敤 `KUNIT_CASE_ATTR(test_name, attributes)` 瀹忔潵瀹氫箟娴嬭瘯鐢ㄤ緥锛屼互鏇夸唬 `KUNIT_CASE(test_name)`銆?

	static const struct kunit_attributes example_attr = {
		.speed = KUNIT_VERY_SLOW,
	};

	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE_ATTR(example_test, example_attr),
	};

	瑕佸皢涓€涓祴璇曠敤渚嬫爣璁颁负鎱㈤€燂紝浣犱篃鍙互浣跨敤 `KUNIT_CASE_SLOW(test_name)`銆?	杩欐槸涓€涓湁鐢ㄧ殑瀹忥紝鍥犱负 slow 灞炴€ф槸鏈€甯哥敤鐨勩€?
娴嬭瘯濂椾欢鍙互閫氳繃鍦ㄥ浠跺畾涔変腑璁剧疆 "attr" 瀛楁鏉ョ敤灞炴€ф爣璁般€?

	static const struct kunit_attributes example_attr = {
		.speed = KUNIT_VERY_SLOW,
	};

	static struct kunit_suite example_test_suite = {
		...,
		.attr = example_attr,
	};

	骞堕潪 `kunit_attributes` 瀵硅薄涓殑鎵€鏈夊睘鎬ч兘闇€瑕佽缃€傛湭璁剧疆鐨勫睘鎬у皢淇濇寔鏈垵濮嬪寲锛屽苟琛ㄧ幇寰楀鍚岃灞炴€ц璁句负 0 鎴?NULL銆傚洜姝わ紝濡傛灉涓€涓睘鎬ц璁句负 0锛屽畠琚涓烘湭璁剧疆銆?	杩欎簺鏈缃殑灞炴€т笉浼氳鎶ュ憡锛屽苟鍙兘浣滀负杩囨护鐩殑鐨勯粯璁ゅ€笺€?
### 鎶ュ憡灞炴€?

褰撶敤鎴疯繍琛屾祴璇曟椂锛屽睘鎬т細瀛樺湪浜庡師濮嬪唴鏍歌緭鍑轰腑锛堜互 KTAP 鏍煎紡锛夈€傛敞鎰忥紝瀵逛簬鎵€鏈夐€氳繃鐨勬祴璇曪紝灞炴€ч粯璁や細鍦?kunit.py 杈撳嚭涓殣钘忥紝浣嗗彲浠ヤ娇鐢?`--raw_output` 鏍囧織璁块棶鍘熷鍐呮牳杈撳嚭銆備笅闈㈡槸娴嬭瘯鐢ㄤ緥鐨勬祴璇曞睘鎬у湪鍐呮牳杈撳嚭涓殑鏍煎紡鍖栫ず渚嬶細


	# example_test.speed: slow
	ok 1 example_test

涓嬮潰鏄祴璇曞浠剁殑娴嬭瘯灞炴€у湪鍐呮牳杈撳嚭涓殑鏍煎紡鍖栫ず渚嬶細


	  KTAP version 2
	  # Subtest: example_suite
	  # module: kunit_example_test
	  1..3
	  ...
	ok 1 example_suite

姝ゅ锛岀敤鎴峰彲浠ヤ娇鐢ㄥ懡浠よ鏍囧織 `--list_tests_attr` 杈撳嚭甯︽湁鍏跺睘鎬х殑娴嬭瘯鐨勫畬鏁村睘鎬ф姤鍛婏細


	kunit.py run "example" --list_tests_attr

	鍦ㄦ墜鍔ㄨ繍琛?KUnit 鏃讹紝鍙互閫氳繃浼犲叆妯″潡鍙傛暟 `kunit.action=list_attr` 鏉ヨ闂鎶ュ憡銆?
### 杩囨护


鐢ㄦ埛鍙互鍦ㄨ繍琛屾祴璇曟椂浣跨敤 `--filter` 鍛戒护琛屾爣蹇楁潵杩囨护娴嬭瘯銆備緥濡傦細


	kunit.py run --filter speed=slow


浣犺繕鍙互瀵硅繃婊ゅ櫒浣跨敤浠ヤ笅杩愮畻绗︼細"<"銆?>"銆?<="銆?>="銆?!=" 鍜?"="銆備緥濡傦細


	kunit.py run --filter "speed>slow"

姝ょず渚嬪皢杩愯鎵€鏈夐€熷害姣?slow 鏇村揩鐨勬祴璇曘€傛敞鎰忥紝瀛楃 < 鍜?> 缁忓父琚?shell 瑙ｉ噴锛屽洜姝ゅ彲鑳介渶瑕佸儚涓婇潰閭ｆ牱鍔犲紩鍙锋垨杞箟銆?
姝ゅ锛屼綘鍙互涓€娆′娇鐢ㄥ涓繃婊ゅ櫒銆傚彧闇€鐢ㄩ€楀彿鍒嗛殧杩囨护鍣ㄥ嵆鍙€備緥濡傦細


	kunit.py run --filter "speed>slow, module=kunit_example_test"

	鍦ㄦ墜鍔ㄨ繍琛?KUnit 鏃讹紝浣犲彲浠ラ€氳繃灏嗚繃婊ゅ櫒浣滀负妯″潡鍙傛暟浼犲叆鏉ヤ娇鐢ㄦ杩囨护鐗规€э細`kunit.filter="speed>slow, speed<=normal"`銆?
琚繃婊ゆ帀鐨勬祴璇曞皢涓嶄細杩愯锛屼篃涓嶄細鍑虹幇鍦ㄦ祴璇曡緭鍑轰腑銆備綘鍙互浣跨敤 `--filter_action=skip` 鏍囧織鏉ユ敼涓鸿烦杩囪杩囨护鐨勬祴璇曘€傝繖浜涙祴璇曚細鏄剧ず鍦ㄦ祴璇曡緭鍑轰腑浣嗕笉浼氳繍琛屻€傚湪鎵嬪姩杩愯 KUnit 鏃讹紝浣跨敤妯″潡鍙傛暟 `kunit.filter_action=skip` 鏉ュ惎鐢ㄦ鐗规€с€?
### 杩囨护杩囩▼瑙勫垯


鐢变簬濂椾欢鍜屾祴璇曠敤渚嬮兘鍙互鍏锋湁灞炴€э紝杩囨护鏈熼棿灞炴€т箣闂村彲鑳藉瓨鍦ㄥ啿绐併€傝繃婊よ繃绋嬮伒寰互涓嬭鍒欙細

- 杩囨护濮嬬粓鍦ㄥ崟涓祴璇曠骇鍒繘琛屻€?
- 濡傛灉涓€涓祴璇曡缃簡鏌愪釜灞炴€э紝鍒欐牴鎹娴嬭瘯鐨勫€艰繘琛岃繃婊ゃ€?
- 鍚﹀垯锛屽洖閫€鍒拌濂椾欢鐨勫€笺€?
- 濡傛灉涓よ€呴兘鏈缃紝鍒欎娇鐢ㄨ灞炴€х殑鍏ㄥ眬"榛樿"鍊笺€?
### 褰撳墠灞炴€у垪琛?

`speed`

姝ゅ睘鎬ф寚绀烘祴璇曟墽琛岀殑閫熷害锛堟祴璇曟槸鎱㈣繕鏄揩锛夈€?
姝ゅ睘鎬т繚瀛樹负涓€涓灇涓撅紝鍖呭惈浠ヤ笅绫诲埆锛?normal"銆?slow" 鎴?"very_slow"銆傛祴璇曠殑鍋囧畾榛樿閫熷害涓?"normal"銆傝繖琛ㄧず娴嬭瘯鑺辫垂鐨勬椂闂寸浉瀵瑰井涓嶈冻閬擄紙灏戜簬 1 绉掞級锛屾棤璁哄叾杩愯鐨勬満鍣ㄥ浣曘€備换浣曟瘮杩欐洿鎱㈢殑娴嬭瘯閮藉彲浠ユ爣璁颁负 "slow" 鎴?"very_slow"銆?
瀹?`KUNIT_CASE_SLOW(test_name)` 鍙互鏂逛究鍦扮敤浜庡皢娴嬭瘯鐢ㄤ緥鐨勯€熷害璁句负 "slow"銆?
`module`

姝ゅ睘鎬ф寚绀轰笌娴嬭瘯鐩稿叧鑱旂殑妯″潡鐨勫悕绉般€?
姝ゅ睘鎬ц嚜鍔ㄤ繚瀛樹负瀛楃涓诧紝骞朵负姣忎釜濂椾欢鎵撳嵃銆傛祴璇曚篃鍙互浣跨敤姝ゅ睘鎬ц繘琛岃繃婊ゃ€?
`is_init`

姝ゅ睘鎬ф寚绀烘祴璇曟槸鍚︿娇鐢ㄤ簡 init 鏁版嵁鎴栧嚱鏁般€?
姝ゅ睘鎬ц嚜鍔ㄤ繚瀛樹负甯冨皵鍊硷紝娴嬭瘯涔熷彲浠ヤ娇鐢ㄦ灞炴€ц繘琛岃繃婊ゃ€?