
# KUnit 鏋舵瀯


## KUnit 鏋舵瀯鍒嗕负涓や釜閮ㄥ垎锛?

- `鍐呮牳鍐呮祴璇曟鏋禶_ 
- `kunit_tool锛堝懡浠よ娴嬭瘯宸ュ叿锛塦_

## 鍐呮牳鍐呮祴璇曟鏋?


鍐呮牳娴嬭瘯搴撴敮鎸佷娇鐢?KUnit 浠?C 璇█缂栧啓鐨?KUnit 娴嬭瘯銆傝繖浜?KUnit 娴嬭瘯鏄唴鏍镐唬鐮併€侹Unit 鎵ц浠ヤ笅浠诲姟锛?

- 缁勭粐娴嬭瘯
- 鎶ュ憡娴嬭瘯缁撴灉
- 鎻愪緵娴嬭瘯宸ュ叿

## 娴嬭瘯鐢ㄤ緥


娴嬭瘯鐢ㄤ緥鏄?KUnit 涓殑鍩烘湰鍗曞厓銆侹Unit 娴嬭瘯鐢ㄤ緥琚粍缁囨垚娴嬭瘯濂椾欢锛坰uite锛夈€備竴涓?KUnit 娴嬭瘯鐢ㄤ緥鏄竴涓被鍨嬩负 `void (**)(struct kunit **test)` 鐨勫嚱鏁般€傝繖浜涙祴璇曠敤渚嬪嚱鏁拌鍖呰鍦ㄤ竴涓悕涓?struct kunit_case 鐨勭粨鏋勪綋涓€?

	`generate_params` 瀵逛簬闈炲弬鏁板寲娴嬭瘯鏄彲閫夌殑銆?

姣忎釜 KUnit 娴嬭瘯鐢ㄤ緥閮戒細鎺ユ敹涓€涓?`struct kunit` 涓婁笅鏂囧璞★紝鐢ㄤ簬璺熻釜姝ｅ湪杩愯鐨勬祴璇曘€侹Unit 鏂█瀹忓拰鍏朵粬 KUnit 宸ュ叿浣跨敤 `struct kunit` 涓婁笅鏂囧璞°€備綔涓轰竴涓緥澶栵紝鏈変袱涓瓧娈碉細

- `->priv`锛氬垵濮嬪寲锛坰etup锛夊嚱鏁板彲浠ョ敤瀹冩潵瀛樺偍浠绘剰鐨勬祴璇曠敤鎴锋暟鎹€?

- `->param_value`锛氬畠鍖呭惈鍙互鍦ㄥ弬鏁板寲娴嬭瘯涓绱㈠埌鐨勫弬鏁板€笺€?

## 娴嬭瘯濂椾欢


涓€涓?KUnit 濂椾欢鍖呭惈涓€缁勬祴璇曠敤渚嬨€侹Unit 濂椾欢鐢?`struct kunit_suite` 琛ㄧず銆備緥濡傦細


	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE(example_test_foo),
		KUNIT_CASE(example_test_bar),
		KUNIT_CASE(example_test_baz),
		{}
	};

	static struct kunit_suite example_test_suite = {
		.name = "example",
		.init = example_test_init,
		.exit = example_test_exit,
		.test_cases = example_test_cases,
	};
	kunit_test_suite(example_test_suite);

鍦ㄤ笂闈㈢殑渚嬪瓙涓紝娴嬭瘯濂椾欢 `example_test_suite` 杩愯娴嬭瘯鐢ㄤ緥 `example_test_foo`銆乣example_test_bar` 鍜?`example_test_baz`銆傚湪杩愯娴嬭瘯涔嬪墠锛屼細璋冪敤 `example_test_init`锛屽湪杩愯娴嬭瘯涔嬪悗锛屼細璋冪敤 `example_test_exit`銆俙kunit_test_suite(example_test_suite)` 灏嗚娴嬭瘯濂椾欢娉ㄥ唽鍒?KUnit 娴嬭瘯妗嗘灦涓€?

## 鎵ц鍣紙Executor锛?


KUnit 鎵ц鍣ㄥ彲浠ュ湪鍚姩鏃跺垪鍑哄苟杩愯鍐呯疆鐨?KUnit 娴嬭瘯銆傝繖浜涙祴璇曞浠跺瓨鍌ㄥ湪涓€涓悕涓?`.kunit_test_suites` 鐨勯摼鎺ュ櫒娈碉紙linker section锛変腑銆傜浉鍏充唬鐮佸弬瑙?`include/asm-generic/vmlinux.lds.h <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/asm-generic/vmlinux.lds.h?h=v6.0#n950>`_ 涓殑 `KUNIT_TABLE()` 瀹忓畾涔夈€傝閾炬帴鍣ㄦ鐢变竴涓寚鍚?`struct kunit_suite` 鐨勬寚閽堟暟缁勭粍鎴愶紝骞剁敱 `kunit_test_suites()` 瀹忓～鍏呫€侹Unit 鎵ц鍣ㄩ亶鍘嗚閾炬帴鍣ㄦ鏁扮粍锛屼互杩愯缂栬瘧杩涘唴鏍哥殑鎵€鏈夋祴璇曘€?

## :alt:	KUnit 濂椾欢鍐呭瓨

## KUnit 濂椾欢鍐呭瓨鍥?

鍦ㄥ唴鏍稿惎鍔ㄦ椂锛孠Unit 鎵ц鍣ㄤ娇鐢ㄨ娈电殑璧峰鍜岀粨鏉熷湴鍧€鏉ラ亶鍘嗗苟杩愯鎵€鏈夋祴璇曘€傛湁鍏虫墽琛屽櫒鐨勫疄鐜帮紝璇峰弬瑙?`lib/kunit/executor.c <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/lib/kunit/executor.c>`_銆傚綋浠ュ唴鏍告ā鍧楀舰寮忔瀯寤烘椂锛宍kunit_test_suites()` 瀹忎細瀹氫箟涓€涓?`module_init()` 鍑芥暟锛岃鍑芥暟杩愯缂栬瘧鍗曞厓涓殑鎵€鏈夋祴璇曪紝鑰屼笉鏄娇鐢ㄦ墽琛屽櫒銆?

鍦?KUnit 娴嬭瘯涓紝鏌愪簺閿欒绫讳笉浼氬奖鍝嶅叾浠栨祴璇曟垨鍐呮牳鐨勫叾浠栭儴鍒嗭紝姣忎釜 KUnit 鐢ㄤ緥鍦ㄧ嫭绔嬬殑绾跨▼涓婁笅鏂囦腑鎵ц銆傝鍙傝 `lib/kunit/try-catch.c <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/lib/kunit/try-catch.c?h=v5.15#n58>`_ 涓殑 `kunit_try_catch_run()` 鍑芥暟銆?

## 鏂█瀹?


KUnit 娴嬭瘯浣跨敤鏈熸湜锛坋xpectation锛?鏂█锛坅ssertion锛夋潵楠岃瘉鐘舵€併€傛墍鏈夋湡鏈?鏂█鐨勬牸寮忎负锛?
`KUNIT_{EXPECT|ASSERT}_<op>[_MSG](kunit, property[, message])`

- `{EXPECT|ASSERT}` 鍐冲畾璇ユ鏌ユ槸鏂█杩樻槸鏈熸湜銆傚湪澶辫触鏃讹紝娴嬭瘯娴佺▼鐨勫尯鍒涓嬶細

 - 瀵逛簬鏈熸湜锛屾祴璇曡鏍囪涓哄け璐ワ紝骞惰褰曡澶辫触銆?

 - 鍙︿竴鏂归潰锛屾柇瑷€澶辫触浼氬鑷存祴璇曠敤渚嬬珛鍗崇粓姝€?

  - 鏂█浼氳皟鐢ㄥ嚱鏁帮細
		  `void __noreturn __kunit_abort(struct kunit *)`銆?

  - `__kunit_abort` 璋冪敤鍑芥暟锛?
		  `void __noreturn kunit_try_catch_throw(struct kunit_try_catch *try_catch)`銆?

  - `kunit_try_catch_throw` 璋冪敤鍑芥暟锛?
		  `void kthread_complete_and_exit(struct completion *, long) __noreturn;`
		  骞剁粓姝㈣鐗规畩绾跨▼涓婁笅鏂囥€?

- `<op>` 琛ㄧず甯︽湁浠ヤ笅閫夐」鐨勬鏌ワ細`TRUE`锛堟墍鎻愪緵鐨勫睘鎬у叿鏈夊竷灏斿€?鈥渢rue鈥濓級銆乣EQ`锛堟墍鎻愪緵鐨勪袱涓睘鎬х浉绛夛級銆乣NOT_ERR_OR_NULL`锛堟墍鎻愪緵鐨勬寚閽堜笉涓虹┖涓斾笉鍖呭惈 鈥渆rr鈥?鍊硷級銆?

- `[_MSG]` 鍦ㄥけ璐ユ椂鎵撳嵃鑷畾涔夋秷鎭€?

## 娴嬭瘯缁撴灉鎶ュ憡

KUnit 浠?KTAP 鏍煎紡鎵撳嵃娴嬭瘯缁撴灉銆侹TAP 鍩轰簬 TAP14锛屽弬瑙?Documentation/dev-tools/ktap.rst銆侹TAP 鍙笌 KUnit 鍜?Kselftest 閰嶅悎浣跨敤銆侹Unit 鎵ц鍣ㄥ皢 KTAP 缁撴灉鎵撳嵃鍒?dmesg 鍜?debugfs锛堝鏋滃凡閰嶇疆锛夈€?

## 鍙傛暟鍖栨祴璇?


姣忎釜 KUnit 鍙傛暟鍖栨祴璇曢兘鍏宠仈涓€缁勫弬鏁般€傝娴嬭瘯浼氳澶氭璋冪敤锛屾瘡涓弬鏁板€艰皟鐢ㄤ竴娆★紝骞朵笖鍙傛暟瀛樺偍鍦?`param_value` 瀛楁涓€傛祴璇曠敤渚嬪寘鍚竴涓帴鍙楃敓鎴愬櫒鍑芥暟鐨?KUNIT_CASE_PARAM() 瀹忋€傜敓鎴愬櫒鍑芥暟鎺ユ敹鍓嶄竴涓弬鏁板苟杩斿洖涓嬩竴涓弬鏁般€傚畠杩樺寘鍚竴涓敤浜庣敓鎴愬熀浜庢暟缁勭殑甯歌鎯呭喌鐢熸垚鍣ㄧ殑瀹忋€?

## kunit_tool锛堝懡浠よ娴嬭瘯宸ュ叿锛?


`kunit_tool` 鏄竴涓?Python 鑴氭湰锛屼綅浜?`tools/testing/kunit/kunit.py`銆傚畠鐢ㄤ簬閰嶇疆銆佹瀯寤恒€佹墽琛屻€佽В鏋愭祴璇曠粨鏋滐紝骞舵寜姝ｇ‘椤哄簭杩愯鍓嶉潰鎵€鏈夊懡浠わ紙鍗抽厤缃€佹瀯寤恒€佹墽琛屽拰瑙ｆ瀽锛夈€傝繍琛?KUnit 娴嬭瘯鏈変袱绉嶉€夋嫨锛氳涔堟瀯寤轰竴涓惎鐢ㄤ簡 KUnit 鐨勫唴鏍稿苟鎵嬪姩瑙ｆ瀽缁撴灉锛堝弬瑙?Documentation/dev-tools/kunit/run_manual.rst锛夛紝瑕佷箞浣跨敤 `kunit_tool`锛堝弬瑙?Documentation/dev-tools/kunit/run_wrapper.rst锛夈€?

- `configure` 鍛戒护浠?`.kunitconfig` 鏂囦欢锛堜互鍙婁换浣曟灦鏋勭壒瀹氱殑閫夐」锛夌敓鎴愬唴鏍?`.config`銆俙qemu_configs` 鏂囦欢澶逛腑鎻愪緵鐨?Python 鑴氭湰锛堜緥濡?`tools/testing/kunit/qemu configs/powerpc.py`锛夊寘鍚壒瀹氭灦鏋勭殑棰濆閰嶇疆閫夐」銆傚畠浼氳В鏋愮幇鏈夌殑 `.config` 鍜?`.kunitconfig` 鏂囦欢锛屼互纭繚 `.config` 鏄?`.kunitconfig` 鐨勮秴闆嗐€傚鏋滀笉鏄紝瀹冧細灏嗕袱鑰呭悎骞跺苟杩愯 `make olddefconfig` 鏉ラ噸鏂扮敓鎴?`.config` 鏂囦欢銆傜劧鍚庡畠妫€鏌?`.config` 鏄惁宸叉垚涓鸿秴闆嗐€傝繖楠岃瘉浜嗘墍鏈?Kconfig 渚濊禆椤归兘鍦?`.kunitconfig` 鏂囦欢涓纭寚瀹氥€俙kunit_config.py` 鑴氭湰鍖呭惈瑙ｆ瀽 Kconfig 鐨勪唬鐮併€傝繍琛?`make olddefconfig` 鐨勪唬鐮佸睘浜?`kunit_kernel.py` 鑴氭湰鐨勪竴閮ㄥ垎銆備綘鍙互閫氳繃浠ヤ笅鍛戒护璋冪敤姝ゅ懡浠わ細`./tools/testing/kunit/kunit.py config`锛屽苟鐢熸垚 `.config` 鏂囦欢銆?
- `build` 鍦ㄥ唴鏍告爲涓婁娇鐢ㄦ墍闇€閫夐」锛堝彇鍐充簬鏋舵瀯鍜屾煇浜涢€夐」锛屼緥濡?build_dir锛夎繍琛?`make`锛屽苟鎶ュ憡浠讳綍閿欒銆傝浠庡綋鍓嶇殑 `.config` 鏋勫缓 KUnit 鍐呮牳锛屼綘鍙互浣跨敤 `build` 鍙傛暟锛歚./tools/testing/kunit/kunit.py build`銆?
- `exec` 鍛戒护鐩存帴锛堜娇鐢?User-mode Linux 閰嶇疆锛夋垨閫氳繃 QEMU 绛夋ā鎷熷櫒鎵ц鍐呮牳缁撴灉銆傚畠浣跨敤鏍囧噯杈撳嚭锛坰tdout锛変粠鏃ュ織涓鍙栫粨鏋滐紝骞跺皢鍏朵紶閫掔粰 `parse` 杩涜瑙ｆ瀽銆傚鏋滀綘宸茬粡鏋勫缓浜嗕竴涓甫鏈夊唴缃?KUnit 娴嬭瘯鐨勫唴鏍革紝鍙互浣跨敤 `exec` 鍙傛暟杩愯鍐呮牳骞舵樉绀烘祴璇曠粨鏋滐細`./tools/testing/kunit/kunit.py exec`銆?
- `parse` 浠庡唴鏍告棩蹇椾腑鎻愬彇 KTAP 杈撳嚭锛岃В鏋愭祴璇曠粨鏋滐紝骞舵墦鍗版憳瑕併€傚浜庡け璐ョ殑娴嬭瘯锛屼細鍖呭惈浠讳綍璇婃柇杈撳嚭銆?
