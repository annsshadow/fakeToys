
## 缂栧啓娴嬭瘯


### 娴嬭瘯鐢ㄤ緥


KUnit 涓殑鍩烘湰鍗曞厓鏄祴璇曠敤渚嬶紙test case锛夈€傛祴璇曠敤渚嬫槸涓€涓鍚嶄负 `void (**)(struct kunit **test)` 鐨勫嚱鏁般€傚畠浼氳皟鐢ㄨ娴嬪嚱鏁帮紝鐒跺悗涓哄簲褰撳彂鐢熺殑浜嬫儏璁剧疆**鏈熸湜锛坋xpectation锛?*銆備緥濡傦細


	void example_test_success(struct kunit *test)
	{
	}

	void example_test_failure(struct kunit *test)
	{
		KUNIT_FAIL(test, "This test never passes.");
	}

鍦ㄤ笂闈㈢殑绀轰緥涓紝`example_test_success` 鍥犱负浠€涔堥兘涓嶅仛鑰屾€绘槸閫氳繃锛涚敱浜庢病鏈夎缃换浣曟湡鏈涳紝鍥犳鎵€鏈夋湡鏈涢兘閫氳繃銆傚彟涓€鏂归潰锛宍example_test_failure` 鍥犱负璋冪敤浜?`KUNIT_FAIL` 鑰屾€绘槸澶辫触锛宍KUNIT_FAIL` 鏄竴涓壒娈婄殑鏈熸湜锛屽畠浼氳褰曚竴鏉℃秷鎭苟瀵艰嚧娴嬭瘯鐢ㄤ緥澶辫触銆?

#### 鏈熸湜

**鏈熸湜锛坋xpectation锛?*鎸囧畾鎴戜滑鏈熸湜鏌愭浠ｇ爜鍦ㄦ祴璇曚腑鍋氭煇浠朵簨銆傛湡鏈涘儚鍑芥暟涓€鏍疯璋冪敤銆傛祴璇曠敤渚嬮€氳繃涓鸿娴嬩唬鐮佺殑琛屼负璁剧疆鏈熸湜鏉ユ瀯鎴愩€傚綋涓€涓垨澶氫釜鏈熸湜澶辫触鏃讹紝娴嬭瘯鐢ㄤ緥澶辫触锛屽苟璁板綍鏈夊叧澶辫触鐨勪俊鎭€備緥濡傦細


	void add_test_basic(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, add(1, 1));
	}

鍦ㄤ笂闈㈢殑绀轰緥涓紝`add_test_basic` 瀵瑰悕涓?`add` 鐨勫嚱鏁拌涓哄仛浜嗚嫢骞叉柇瑷€銆傜涓€涓弬鏁板缁堟槸 `struct kunit *` 绫诲瀷锛屽寘鍚湁鍏冲綋鍓嶆祴璇曚笂涓嬫枃鐨勪俊鎭€傚湪鏈緥涓紝绗簩涓弬鏁版槸鏈熸湜鍊笺€傛渶鍚庝竴涓€兼槸瀹為檯鍊笺€傚鏋?`add` 閫氳繃浜嗘墍鏈夎繖浜涙湡鏈涳紝娴嬭瘯鐢ㄤ緥 `add_test_basic` 灏嗛€氳繃锛涘鏋滆繖浜涙湡鏈涗腑鏈変换浣曚竴涓け璐ワ紝娴嬭瘯鐢ㄤ緥灏嗗け璐ャ€?

褰撲换浣曟湡鏈涜杩濆弽鏃讹紝娴嬭瘯鐢ㄤ緥灏变細**澶辫触**锛涗絾鏄紝娴嬭瘯浼氱户缁繍琛岋紝骞跺皾璇曞叾浠栨湡鏈涳紝鐩村埌娴嬭瘯鐢ㄤ緥缁撴潫鎴栦互鍏朵粬鏂瑰紡琚粓姝€傝繖涓庡悗闈㈣璁虹殑**鏂█锛坅ssertion锛?*涓嶅悓銆?

瑕佷簡瑙ｆ洿澶?KUnit 鏈熸湜锛岃鍙傞槄 Documentation/dev-tools/kunit/api/test.rst銆?

   鍗曚釜娴嬭瘯鐢ㄤ緥搴斿綋绠€鐭€佹槗浜庣悊瑙ｏ紝骞朵笓娉ㄤ簬鍗曚竴琛屼负銆?

渚嬪锛屽鏋滄垜浠兂涓ユ牸娴嬭瘯涓婇潰鐨?`add` 鍑芥暟锛屽彲浠ュ垱寤洪澶栫殑娴嬭瘯鐢ㄤ緥鏉ユ祴璇?`add` 鍑芥暟搴斿綋鍏峰鐨勬瘡涓€涓睘鎬э紝濡備笅鎵€绀猴細


	void add_test_basic(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, add(1, 1));
	}

	void add_test_negative(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 0, add(-1, 1));
	}

	void add_test_max(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, INT_MAX, add(0, INT_MAX));
		KUNIT_EXPECT_EQ(test, -1, add(INT_MAX, INT_MIN));
	}

	void add_test_overflow(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, INT_MIN, add(INT_MAX, 1));
	}

#### 鏂█


鏂█绫讳技浜庢湡鏈涳紝鍙槸褰撴潯浠朵笉婊¤冻鏃讹紝鏂█浼氱珛鍗崇粓姝㈡祴璇曠敤渚嬨€備緥濡傦細


	static void test_sort(struct kunit *test)
	{
		int *a, i, r = 1;
		a = kunit_kmalloc_array(test, TEST_LEN, sizeof(*a), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a);
		for (i = 0; i < TEST_LEN; i++) {
			r = (r * 725861) % 6599;
			a[i] = r;
		}
		sort(a, TEST_LEN, sizeof(*a), cmpint, NULL);
		for (i = 0; i < TEST_LEN-1; i++)
			KUNIT_EXPECT_LE(test, a[i], a[i + 1]);
	}

鍦ㄦ绀轰緥涓紝鎴戜滑闇€瑕佽兘澶熷垎閰嶄竴涓暟缁勬潵娴嬭瘯 `sort()` 鍑芥暟銆傚洜姝ゆ垜浠娇鐢?`KUNIT_ASSERT_NOT_ERR_OR_NULL()` 鍦ㄥ嚭鐜板垎閰嶉敊璇椂涓娴嬭瘯銆?

   鍦ㄥ叾浠栨祴璇曟鏋朵腑锛宍ASSERT` 瀹忛€氬父閫氳繃璋冪敤 `return` 瀹炵幇锛屽洜姝ゅ畠浠彧鑳戒粠娴嬭瘯鍑芥暟
   涓敓鏁堛€傚湪 KUnit 涓紝鎴戜滑浼氬湪澶辫触鏃跺仠姝㈠綋鍓嶇殑 kthread锛屽洜姝ゅ彲浠ヤ粠浠讳綍鍦版柟璋冪敤瀹冧滑銆?

   璀﹀憡锛氫笂杩拌鍒欐湁涓€涓緥澶栥€備綘涓嶅簲鍦ㄦ祴璇曞浠剁殑 exit() 鍑芥暟鎴栬祫婧愮殑閲婃斁鍑芥暟涓娇鐢?
   鏂█銆傝繖浜涘嚱鏁板湪娴嬭瘯鍏抽棴鏃惰繍琛岋紝姝ゅ鐨勬柇瑷€浼氶樆姝㈠悗缁竻鐞嗕唬鐮佽繍琛岋紝鍙兘瀵艰嚧鍐呭瓨娉勬紡銆?

### 鑷畾涔夐敊璇秷鎭?


姣忎釜 `KUNIT_EXPECT` 鍜?`KUNIT_ASSERT` 瀹忛兘鏈変竴涓?`_MSG` 鍙樹綋銆傚畠浠帴鍙椾竴涓牸寮忓瓧绗︿覆鍜屽弬鏁帮紝涓鸿嚜鍔ㄧ敓鎴愮殑閿欒娑堟伅鎻愪緵棰濆鐨勪笂涓嬫枃銆?


	char some_str[^41^];
	generate_sha1_hex_string(some_str);

	/** Before. Not easy to tell why the test failed. **/
	KUNIT_EXPECT_EQ(test, strlen(some_str), 40);

	/** After. Now we see the offending string. **/
	KUNIT_EXPECT_EQ_MSG(test, strlen(some_str), 40, "some_str='%s'", some_str);

鎴栬€咃紝鍙互閫氳繃浣跨敤 `KUNIT_FAIL()` 瀹屽叏鎺у埗閿欒娑堟伅锛屼緥濡傦細


	/** Before **/
	KUNIT_EXPECT_EQ(test, some_setup_function(), 0);

	/** After: full control over the failure message. **/
	if (some_setup_function())
		KUNIT_FAIL(test, "Failed to setup thing for testing");


#### 娴嬭瘯濂椾欢


鎴戜滑闇€瑕佽澶氭祴璇曠敤渚嬫潵瑕嗙洊璇ュ崟鍏冪殑鎵€鏈夎涓恒€傛嫢鏈夎澶氱浉浼肩殑娴嬭瘯鏄緢甯歌鐨勩€備负浜嗗噺灏戣繖浜涚揣瀵嗙浉鍏虫祴璇曚腑鐨勯噸澶嶏紝澶у鏁板崟鍏冩祴璇曟鏋讹紙鍖呮嫭 KUnit锛夐兘鎻愪緵浜?*娴嬭瘯濂椾欢锛坱est suite锛?*鐨勬蹇点€傛祴璇曞浠舵槸涓€缁勬祴璇曠敤渚嬬殑闆嗗悎锛屽甫鏈夊彲閫夌殑 setup 鍜?teardown 鍑芥暟锛屽垎鍒湪鏁村濂椾欢鍜?鎴栨瘡涓祴璇曠敤渚嬩箣鍓?涔嬪悗杩愯銆?

   娴嬭瘯鐢ㄤ緥鍙湁鍦ㄤ笌鏌愪釜娴嬭瘯濂椾欢鍏宠仈鏃舵墠浼氳繍琛屻€?

渚嬪锛?


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
		.suite_init = example_suite_init,
		.suite_exit = example_suite_exit,
		.test_cases = example_test_cases,
	};
	kunit_test_suite(example_test_suite);

鍦ㄤ笂闈㈢殑绀轰緥涓紝娴嬭瘯濂椾欢 `example_test_suite` 浼氬厛杩愯 `example_suite_init`锛岀劧鍚庤繍琛屾祴璇曠敤渚?`example_test_foo`銆乣example_test_bar` 鍜?`example_test_baz`銆傛瘡涓祴璇曠敤渚嬪湪杩愯鍓嶄細绔嬪嵆璋冪敤 `example_test_init`锛岃繍琛屽悗浼氱珛鍗宠皟鐢?`example_test_exit`銆傛渶鍚庯紝鍦ㄦ墍鏈夊叾浠栧唴瀹逛箣鍚庤皟鐢?`example_suite_exit`銆俙kunit_test_suite(example_test_suite)` 灏嗚娴嬭瘯濂椾欢娉ㄥ唽鍒?KUnit 娴嬭瘯妗嗘灦銆?

   `exit` 鍜?`suite_exit` 鍑芥暟鍗充娇 `init` 鎴?`suite_init` 澶辫触涔熶細杩愯銆傝纭繚瀹冧滑鑳藉
   澶勭悊鐢?`init` 鎴?`suite_init` 閬囧埌閿欒鎴栨彁鍓嶉€€鍑烘墍瀵艰嚧鐨勪换浣曚笉涓€鑷寸姸鎬併€?

`kunit_test_suite(...)` 鏄竴涓畯锛屽畠鍛婅瘔閾炬帴鍣ㄥ皢鎸囧畾鐨勬祴璇曞浠舵斁鍏ヤ竴涓壒娈婄殑閾炬帴鍣ㄦ锛坙inker section锛夛紝浠ヤ究 KUnit 鍦?`late_init` 涔嬪悗鎴栨祴璇曟ā鍧楀姞杞芥椂锛堝鏋滄祴璇曡鏋勫缓涓烘ā鍧楋級杩愯瀹冦€?

鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/api/test.rst銆?


### 涓哄叾浠栨灦鏋勭紪鍐欐祴璇?


缂栧啓鑳藉湪 UML 涓婅繍琛岀殑娴嬭瘯锛屼紭浜庝粎鑳藉湪鐗瑰畾鏋舵瀯涓嬭繍琛岀殑娴嬭瘯銆傜紪鍐欒兘鍦?QEMU 鎴栧叾浠栨槗浜庤幏鍙栵紙涓斿厤璐癸級鐨勮蒋浠剁幆澧冧笅杩愯鐨勬祴璇曪紝浼樹簬閽堝鐗瑰畾纭欢鐨勬祴璇曘€?

灏界濡傛锛屼粛鏈夊厖鍒嗙殑鐞嗙敱缂栧啓鏋舵瀯鎴栫‖浠剁壒瀹氱殑娴嬭瘯銆備緥濡傦紝鎴戜滑鍙兘鎯虫祴璇曠湡姝ｅ睘浜?`arch/some-arch/*` 鐨勪唬鐮併€傚嵆渚垮姝わ紝涔熷敖閲忕紪鍐欎笉渚濊禆浜庣墿鐞嗙‖浠剁殑娴嬭瘯銆傛垜浠殑涓€浜涙祴璇曠敤渚嬪彲鑳戒笉闇€瑕佺‖浠讹紝鍙湁灏戞暟娴嬭瘯鐪熸闇€瑕佺‖浠舵潵娴嬭瘯銆傚綋纭欢涓嶅彲鐢ㄦ椂锛屼笌鍏剁鐢ㄦ祴璇曪紝鎴戜滑鍙互璺宠繃瀹冧滑銆?

鏃㈢劧鎴戜滑宸茬粡纭垏纭畾浜嗗摢浜涢儴鍒嗘槸纭欢鐗瑰畾鐨勶紝缂栧啓鍜岃繍琛岃繖浜涙祴璇曠殑瀹為檯杩囩▼涓庣紪鍐欐櫘閫?KUnit 娴嬭瘯鐩稿悓銆?

   鎴戜滑鍙兘闇€瑕侀噸缃‖浠剁姸鎬併€傚鏋滆繖涓嶅彲鑳斤紝鎴戜滑鍙兘鍙兘鍦ㄦ瘡娆¤皟鐢ㄤ腑杩愯涓€涓祴璇曠敤渚嬨€?

   锛堜緷璧栫‖浠剁殑 KUnit 娴嬭瘯銆傦級

## 甯歌妯″紡


### 闅旂琛屼负


鍗曞厓娴嬭瘯灏嗗緟娴嬩唬鐮佺殑鑼冨洿闄愬埗鍒板崟涓€鍗曞厓銆傚畠鎺у埗鍦ㄨ娴嬪崟鍏冭皟鐢ㄦ煇涓嚱鏁版椂杩愯鍝簺浠ｇ爜銆傚綋涓€涓嚱鏁颁綔涓?API 鐨勪竴閮ㄥ垎鏆撮湶鍑烘潵锛屼娇寰楄鍑芥暟鐨勫畾涔夊彲浠ュ湪涓嶅奖鍝嶄唬鐮佸簱鍏朵綑閮ㄥ垎鐨勬儏鍐典笅鏇存敼鏃讹紝灏卞睘浜庤繖绉嶆儏鍐点€傚湪鍐呮牳涓紝杩欐潵鑷袱绉嶆瀯閫狅細绫伙紙class锛屽嵆鍖呭惈瀹炵幇鑰呮彁渚涚殑鍑芥暟鎸囬拡鐨勭粨鏋勪綋锛夊拰鏋舵瀯鐗瑰畾鍑芥暟锛堝叾瀹氫箟鍦ㄧ紪璇戞椂閫夊畾锛夈€?

#### 绫?


绫诲苟涓嶆槸 C 缂栫▼璇█鍐呯疆鐨勬瀯閫狅紱鐒惰€岋紝瀹冩槸涓€涓鏄撴帹瀵煎嚭鐨勬蹇点€傚洜姝わ紝鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝姣忎釜涓嶄娇鐢ㄦ爣鍑嗗寲闈㈠悜瀵硅薄搴擄紙濡?GNOME 鐨?GObject锛夌殑椤圭洰閮芥湁鑷繁鐣ュ井涓嶅悓鐨勯潰鍚戝璞＄紪绋嬫柟寮忥紱Linux 鍐呮牳涔熶笉渚嬪銆?

鍐呮牳闈㈠悜瀵硅薄缂栫▼鐨勬牳蹇冩蹇垫槸绫伙紙class锛夈€傚湪鍐呮牳涓紝**绫?*鏄寘鍚嚱鏁版寚閽堢殑缁撴瀯浣撱€傝繖鍦?*瀹炵幇鑰咃紙implementer锛?*鍜?*浣跨敤鑰咃紙user锛?*涔嬮棿鍒涘缓浜嗕竴涓绾︼紝鍥犱负瀹冨己鍒跺畠浠娇鐢ㄧ浉鍚岀殑鍑芥暟绛惧悕锛岃€屾棤闇€鐩存帴璋冪敤璇ュ嚱鏁般€傝鎴愪负涓€涓被锛屽嚱鏁版寚閽堝繀椤绘寚瀹氫竴涓寚鍚戣绫荤殑鎸囬拡锛堢О涓?*绫诲彞鏌勶紙class handle锛?*锛変綔涓哄弬鏁颁箣涓€銆傚洜姝わ紝鎴愬憳鍑芥暟锛堜篃绉颁负**鏂规硶锛坢ethod锛?*锛夊彲浠ヨ闂垚鍛樺彉閲忥紙涔熺О涓?*瀛楁锛坒ield锛?*锛夛紝浣垮緱鍚屼竴涓疄鐜板彲浠ユ湁澶氫釜**瀹炰緥锛坕nstance锛?*銆?

绫诲彲浠ラ€氳繃**瀛愮被锛坈hild class锛?*宓屽叆**鐖剁被锛坧arent class锛?*鏉ヨ**閲嶅啓锛坥verride锛?*銆傜劧鍚庯紝褰撹皟鐢ㄥ瓙绫荤殑**鏂规硶**鏃讹紝瀛愮被瀹炵幇鐭ラ亾浼犻€掔粰瀹冪殑鎸囬拡鏄寘鍚湪瀛愮被涓殑鐖剁被銆傚洜姝わ紝瀛愮被鍙互璁＄畻鍑烘寚鍚戣嚜韬殑鎸囬拡锛屽洜涓烘寚鍚戠埗绫荤殑鎸囬拡涓庢寚鍚戝瓙绫荤殑鎸囬拡涔嬮棿鎬绘槸瀛樺湪鍥哄畾鐨勫亸绉婚噺銆傝繖涓亸绉婚噺灏辨槸鐖剁粨鏋勪綋鍦ㄥ瓙缁撴瀯浣撲腑鍖呭惈鐨勫亸绉婚噺銆備緥濡傦細


	struct shape {
		int (**area)(struct shape **this);
	};

	struct rectangle {
		struct shape parent;
		int length;
		int width;
	};

	int rectangle_area(struct shape *this)
	{
		struct rectangle *self = container_of(this, struct rectangle, parent);

		return self->length * self->width;
	};

	void rectangle_new(struct rectangle *self, int length, int width)
	{
		self->parent.area = rectangle_area;
		self->length = length;
		self->width = width;
	}

鍦ㄦ绀轰緥涓紝浠庢寚鍚戠埗绫荤殑鎸囬拡璁＄畻鎸囧悜瀛愮被鐨勬寚閽堢敱 `container_of` 瀹屾垚銆?

#### 浼被


涓轰簡瀵硅皟鐢ㄧ被涓煇涓柟娉曠殑浠ｇ爜杩涜鍗曞厓娴嬭瘯锛岃鏂规硶鐨勮涓哄繀椤绘槸鍙帶鐨勶紝鍚﹀垯娴嬭瘯灏变笉鍐嶆槸鍗曞厓娴嬭瘯锛岃€屽彉鎴愪簡闆嗘垚娴嬭瘯銆?

浼被锛坒ake class锛夊疄鐜颁簡涓€娈典笌鐢熶骇瀹炰緥涓繍琛岀殑浠ｇ爜涓嶅悓锛屼絾浠庤皟鐢ㄨ€呯殑瑙掑害鐪嬭涓虹浉鍚岀殑浠ｇ爜銆傝繖鏍峰仛鏄负浜嗘浛鎹㈤毦浠ュ鐞嗘垨閫熷害杈冩參鐨勪緷璧栥€備緥濡傦紝瀹炵幇涓€涓皢"鍐呭"瀛樺偍鍦ㄥ唴閮ㄧ紦鍐插尯涓殑浼?EEPROM銆傚亣璁炬垜浠湁涓€涓〃绀?EEPROM 鐨勭被锛?


	struct eeprom {
		ssize_t (**read)(struct eeprom **this, size_t offset, char *buffer, size_t count);
		ssize_t (**write)(struct eeprom **this, size_t offset, const char *buffer, size_t count);
	};

鎴戜滑鎯虫祴璇曞 EEPROM 鍐欏叆杩涜缂撳啿鐨勪唬鐮侊細


	struct eeprom_buffer {
		ssize_t (**write)(struct eeprom_buffer **this, const char *buffer, size_t count);
		int flush(struct eeprom_buffer *this);
		size_t flush_count; /** Flushes when buffer exceeds flush_count. **/
	};

	struct eeprom_buffer **new_eeprom_buffer(struct eeprom **eeprom);
	void destroy_eeprom_buffer(struct eeprom *eeprom);

鎴戜滑鍙互閫氳繃**浼寲锛坒aking out锛?*搴曞眰 EEPROM 鏉ユ祴璇曡繖娈典唬鐮侊細


	struct fake_eeprom {
		struct eeprom parent;
		char contents[FAKE_EEPROM_CONTENTS_SIZE];
	};

	ssize_t fake_eeprom_read(struct eeprom **parent, size_t offset, char **buffer, size_t count)
	{
		struct fake_eeprom *this = container_of(parent, struct fake_eeprom, parent);

		count = min(count, FAKE_EEPROM_CONTENTS_SIZE - offset);
		memcpy(buffer, this->contents + offset, count);

		return count;
	}

	ssize_t fake_eeprom_write(struct eeprom **parent, size_t offset, const char **buffer, size_t count)
	{
		struct fake_eeprom *this = container_of(parent, struct fake_eeprom, parent);

		count = min(count, FAKE_EEPROM_CONTENTS_SIZE - offset);
		memcpy(this->contents + offset, buffer, count);

		return count;
	}

	void fake_eeprom_init(struct fake_eeprom *this)
	{
		this->parent.read = fake_eeprom_read;
		this->parent.write = fake_eeprom_write;
		memset(this->contents, 0, FAKE_EEPROM_CONTENTS_SIZE);
	}

鎴戜滑鐜板湪鍙互鐢ㄥ畠鏉ユ祴璇?`struct eeprom_buffer`锛?


	struct eeprom_buffer_test {
		struct fake_eeprom *fake_eeprom;
		struct eeprom_buffer *eeprom_buffer;
	};

	static void eeprom_buffer_test_does_not_write_until_flush(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff};

		eeprom_buffer->flush_count = SIZE_MAX;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0);

		eeprom_buffer->flush(eeprom_buffer);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
	}

	static void eeprom_buffer_test_flushes_after_flush_count_met(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff};

		eeprom_buffer->flush_count = 2;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
	}

	static void eeprom_buffer_test_flushes_increments_of_flush_count(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff, 0xff};

		eeprom_buffer->flush_count = 2;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 2);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
		/** Should have only flushed the first two bytes. **/
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^2^], 0);
	}

	static int eeprom_buffer_test_init(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx;

		ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);

		ctx->fake_eeprom = kunit_kzalloc(test, sizeof(*ctx->fake_eeprom), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx->fake_eeprom);
		fake_eeprom_init(ctx->fake_eeprom);

		ctx->eeprom_buffer = new_eeprom_buffer(&ctx->fake_eeprom->parent);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx->eeprom_buffer);

		test->priv = ctx;

		return 0;
	}

	static void eeprom_buffer_test_exit(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;

		destroy_eeprom_buffer(ctx->eeprom_buffer);
	}

### 閽堝澶氫釜杈撳叆杩涜娴嬭瘯


浠呮祴璇曞皯鏁板嚑涓緭鍏ヤ笉瓒充互纭繚浠ｇ爜姝ｅ父宸ヤ綔锛屼緥濡傦細娴嬭瘯鍝堝笇鍑芥暟銆?

鎴戜滑鍙互缂栧啓涓€涓緟鍔╁畯鎴栧嚱鏁般€傝鍑芥暟閽堝姣忎釜杈撳叆琚皟鐢ㄣ€備緥濡傦紝瑕佹祴璇?`sha1sum(1)`锛屾垜浠彲浠ョ紪鍐欙細


	#define TEST_SHA1(in, want) \
		sha1sum(in, out); \
		KUNIT_EXPECT_STREQ_MSG(test, out, want, "sha1sum(%s)", in);

	char out[^40^];
	TEST_SHA1("hello world",  "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
	TEST_SHA1("hello world!", "430ce34d020724ed75a196dfc2ad67c77772d169");

娉ㄦ剰浣跨敤 `KUNIT_EXPECT_STREQ` 鐨?`_MSG` 鐗堟湰鏉ユ墦鍗版洿璇︾粏鐨勯敊璇紝骞朵娇杈呭姪瀹忎腑鐨勬柇瑷€鏇存竻鏅般€?

褰撳悓涓€涓湡鏈涜澶氭璋冪敤锛堝湪寰幆鎴栬緟鍔╁嚱鏁颁腑锛夋椂锛宍_MSG` 鍙樹綋寰堟湁鐢紝姝ゆ椂琛屽彿涓嶈冻浠ヨ瘑鍒槸鍝釜澶辫触锛屽涓嬫墍绀恒€?

鍦ㄥ鏉傛儏鍐典笅锛岀浉姣斾簬杈呭姪瀹忕殑鍙樹綋锛屾垜浠帹鑽愪娇鐢?*琛ㄩ┍鍔ㄦ祴璇曪紙table-driven test锛?*锛屼緥濡傦細


	int i;
	char out[^40^];

	struct sha1_test_case {
		const char *str;
		const char *sha1;
	};

	struct sha1_test_case cases[] = {
		{
			.str = "hello world",
			.sha1 = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed",
		},
		{
			.str = "hello world!",
			.sha1 = "430ce34d020724ed75a196dfc2ad67c77772d169",
		},
	};
	for (i = 0; i < ARRAY_SIZE(cases); ++i) {
		sha1sum(cases[i].str, out);
		KUNIT_EXPECT_STREQ_MSG(test, out, cases[i].sha1,
		                      "sha1sum(%s)", cases[i].str);
	}

杩欐秹鍙婃洿澶氱殑鏍锋澘浠ｇ爜锛屼絾瀹冨彲浠ワ細

- 褰撴湁澶氫釜杈撳叆/杈撳嚭鏃讹紙寰楃泭浜庡瓧娈靛悕锛夋洿鍏峰彲璇绘€с€?

  - 渚嬪锛屽弬瑙?`fs/ext4/inode-test.c`銆?

- 濡傛灉娴嬭瘯鐢ㄤ緥鍦ㄥ涓祴璇曚箣闂村叡浜紝鍙噺灏戦噸澶嶃€?

  - 渚嬪锛氬鏋滄垜浠兂娴嬭瘯 `sha256sum`锛屽彲浠ユ坊鍔?`sha256` 瀛楁骞堕噸鐢?`cases`銆?

- 鍙浆鎹负"鍙傛暟鍖栨祴璇?銆?

#### 鍙傛暟鍖栨祴璇?


涓轰簡璁╀竴涓祴璇曠敤渚嬮拡瀵瑰涓緭鍏ヨ繍琛岋紝KUnit 鎻愪緵浜嗗弬鏁板寲娴嬭瘯妗嗘灦銆傝鐗规€у皢鍓嶉潰璁ㄨ鐨勮〃椹卞姩娴嬭瘯姒傚康褰㈠紡鍖栧苟杩涜浜嗘墿灞曘€?

濡傛灉鍦ㄦ敞鍐屾祴璇曠敤渚嬫椂鎻愪緵浜嗗弬鏁扮敓鎴愬櫒鍑芥暟锛屽垯 KUnit 娴嬭瘯琚‘瀹氫负鍙傛暟鍖栫殑銆傛祴璇曠敤鎴峰彲浠ョ紪鍐欒嚜宸辩殑鐢熸垚鍣ㄥ嚱鏁帮紝涔熷彲浠ヤ娇鐢?KUnit 鎻愪緵鐨勭敓鎴愬櫒鍑芥暟銆傜敓鎴愬櫒鍑芥暟瀛樺偍鍦?`kunit_case->generate_params` 涓紝鍙互浣跨敤涓嬮潰灏忚妭涓弿杩扮殑瀹忚繘琛岃缃€?

涓轰簡寤虹珛鏈锛?鍙傛暟鍖栨祴璇?鏄寚涓€涓繍琛屽娆★紙姣忎釜"鍙傛暟"鎴?鍙傛暟杩愯"杩愯涓€娆★級鐨勬祴璇曘€傛瘡涓弬鏁拌繍琛岄兘鏈夎嚜宸辩嫭绔嬬殑 `struct kunit`锛?鍙傛暟杩愯涓婁笅鏂?锛夛紝骞朵笖鍙互璁块棶鍏变韩鐨勭埗绾?`struct kunit`锛?鍙傛暟鍖栨祴璇曚笂涓嬫枃"锛夈€?

##### 鍚戞祴璇曚紶閫掑弬鏁?

鏈変笁绉嶆柟寮忓悜娴嬭瘯鎻愪緵鍙傛暟锛?

鏁扮粍鍙傛暟瀹忥細

   KUnit 涓哄父瑙佺殑琛ㄩ┍鍔ㄦ祴璇曟ā寮忔彁渚涗簡鐗规畩鏀寔銆傞€氳繃瀵逛笂涓€灏忚妭鐨?`cases` 鏁扮粍搴旂敤 `KUNIT_ARRAY_PARAM` 鎴?`KUNIT_ARRAY_PARAM_DESC`锛屾垜浠彲浠ュ垱寤轰竴涓弬鏁板寲娴嬭瘯锛屽涓嬫墍绀猴細


	// This is copy-pasted from above.
	struct sha1_test_case {
		const char *str;
		const char *sha1;
	};
	static const struct sha1_test_case cases[] = {
		{
			.str = "hello world",
			.sha1 = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed",
		},
		{
			.str = "hello world!",
			.sha1 = "430ce34d020724ed75a196dfc2ad67c77772d169",
		},
	};

	// Creates `sha1_gen_params()` to iterate over `cases` while using
	// the struct member `str` for the case description.
	KUNIT_ARRAY_PARAM_DESC(sha1, cases, str);

	// Looks no different from a normal test.
	static void sha1_test(struct kunit *test)
	{
		// This function can just contain the body of the for-loop.
		// The former `cases[i]` is accessible under test->param_value.
		char out[^40^];
		struct sha1_test_case **test_param = (struct sha1_test_case **)(test->param_value);

		sha1sum(test_param->str, out);
		KUNIT_EXPECT_STREQ_MSG(test, out, test_param->sha1,
				      "sha1sum(%s)", test_param->str);
	}

	// Instead of KUNIT_CASE, we use KUNIT_CASE_PARAM and pass in the
	// function declared by KUNIT_ARRAY_PARAM or KUNIT_ARRAY_PARAM_DESC.
	static struct kunit_case sha1_test_cases[] = {
		KUNIT_CASE_PARAM(sha1_test, sha1_gen_params),
		{}
	};

鑷畾涔夊弬鏁扮敓鎴愬櫒鍑芥暟锛?

   鐢熸垚鍣ㄥ嚱鏁拌礋璐ｉ€愪釜鐢熸垚鍙傛暟锛屽叾绛惧悕濡備笅锛?
   `const void** (**)(struct kunit **test, const void **prev, char *desc)`銆?
   浣犲彲浠ュ皢鐢熸垚鍣ㄥ嚱鏁颁紶閫掔粰 `KUNIT_CASE_PARAM` 鎴?`KUNIT_CASE_PARAM_WITH_INIT` 瀹忋€?

   璇ュ嚱鏁版帴鏀跺厛鍓嶇敓鎴愮殑鍙傛暟浣滀负 `prev` 鍙傛暟锛堢涓€娆¤皟鐢ㄦ椂涓?`NULL`锛夛紝杩樺彲浠ヨ闂綔涓?
   `test` 鍙傛暟浼犻€掔殑鍙傛暟鍖栨祴璇曚笂涓嬫枃銆侹Unit 鍙嶅璋冪敤璇ュ嚱鏁帮紝鐩村埌瀹冭繑鍥?`NULL`锛岃繖琛ㄧず
   鍙傛暟鍖栨祴璇曠粨鏉熴€?

   涓嬮潰鏄畠濡備綍宸ヤ綔鐨勭ず渚嬶細


	#define MAX_TEST_BUFFER_SIZE 8

	// Example generator function. It produces a sequence of buffer sizes that
	// are powers of two, starting at 1 (e.g., 1, 2, 4, 8).
	static const void **buffer_size_gen_params(struct kunit **test, const void **prev, char **desc)
	{
		long prev_buffer_size = (long)prev;
		long next_buffer_size = 1; // Start with an initial size of 1.

		// Stop generating parameters if the limit is reached or exceeded.
		if (prev_buffer_size >= MAX_TEST_BUFFER_SIZE)
			return NULL;

		// For subsequent calls, calculate the next size by doubling the previous one.
		if (prev)
			next_buffer_size = prev_buffer_size << 1;

		return (void *)next_buffer_size;
	}

	// Simple test to validate that kunit_kzalloc provides zeroed memory.
	static void buffer_zero_test(struct kunit *test)
	{
		long buffer_size = (long)test->param_value;
		// Use kunit_kzalloc to allocate a zero-initialized buffer. This makes the
		// memory "parameter run managed," meaning it's automatically cleaned up at
		// the end of each parameter run.
		int **buf = kunit_kzalloc(test, buffer_size ** sizeof(int), GFP_KERNEL);

		// Ensure the allocation was successful.
		KUNIT_ASSERT_NOT_NULL(test, buf);

		// Loop through the buffer and confirm every element is zero.
		for (int i = 0; i < buffer_size; i++)
			KUNIT_EXPECT_EQ(test, buf[i], 0);
	}

	static struct kunit_case buffer_test_cases[] = {
		KUNIT_CASE_PARAM(buffer_zero_test, buffer_size_gen_params),
		{}
	};

鍦?init 鍑芥暟涓繍琛屾椂娉ㄥ唽鍙傛暟鏁扮粍锛?

   瀵逛簬鍙兘闇€瑕佸垵濮嬪寲鍙傛暟鍖栨祴璇曠殑鍦烘櫙锛屼綘鍙互鐩存帴灏嗗弬鏁版暟缁勬敞鍐屽埌鍙傛暟鍖栨祴璇曚笂涓嬫枃涓€?

   涓烘锛屼綘蹇呴』灏嗗弬鏁板寲娴嬭瘯涓婁笅鏂囥€佹暟缁勬湰韬€佹暟缁勫ぇ灏忎互鍙婁竴涓?`get_description()` 鍑芥暟
   浼犻€掔粰 `kunit_register_params_array()` 瀹忋€傝瀹忓～鍏呭弬鏁板寲娴嬭瘯涓婁笅鏂囦腑鐨?
   `struct kunit_params`锛屾湁鏁堝湴瀛樺偍涓€涓弬鏁版暟缁勫璞°€俙get_description()` 鍑芥暟灏嗙敤浜?
   濉厖鍙傛暟鎻忚堪锛屽叾绛惧悕濡備笅锛歚void (**)(struct kunit **test, const void **param, char **desc)`銆?
   娉ㄦ剰瀹冧篃鍙互璁块棶鍙傛暟鍖栨祴璇曚笂涓嬫枃銆?

```
         When using this way to register a parameter array, you will need to
         manually pass ``kunit_array_gen_params()`` as the generator function to
         ``KUNIT_CASE_PARAM_WITH_INIT``. ``kunit_array_gen_params()`` is a KUnit
         helper that will use the registered array to generate the parameters.

	 If needed, instead of passing the KUnit helper, you can also pass your
	 own custom generator function that utilizes the parameter array. To
	 access the parameter array from within the parameter generator
	 function use ``test->params_array.params``.

   The ``kunit_register_params_array()`` macro should be called within a
   ``param_init()`` function that initializes the parameterized test and has
   the following signature ``int (*)(struct kunit *test)``. For a detailed
   explanation of this mechanism please refer to the "Adding Shared Resources"
   section that is after this one. This method supports registering both
   dynamically built and static parameter arrays.

   The code snippet below shows the ``example_param_init_dynamic_arr`` test that
   utilizes ``make_fibonacci_params()`` to create a dynamic array, which is then
   registered using ``kunit_register_params_array()``. To see the full code
   please refer to lib/kunit/kunit-example-test.c.

```

	/*
 - Example of a parameterized test param_init() function that registers a dynamic
 - array of parameters.
	*/
	static int example_param_init_dynamic_arr(struct kunit *test)
	{
		size_t seq_size;
		int *fibonacci_params;

		kunit_info(test, "initializing parameterized test\n");

		seq_size = 6;
		fibonacci_params = make_fibonacci_params(test, seq_size);
		if (!fibonacci_params)
			return -ENOMEM;
		/*
  - Passes the dynamic parameter array information to the parameterized test
  - context struct kunit. The array and its metadata will be stored in
  - test->parent->params_array. The array itself will be located in
  - params_data.params.
		*/
		kunit_register_params_array(test, fibonacci_params, seq_size,
					example_param_dynamic_arr_get_desc);
		return 0;
	}

	static struct kunit_case example_test_cases[] = {
		/*
   - Note how we pass kunit_array_gen_params() to use the array we
   - registered in example_param_init_dynamic_arr() to generate
   - parameters.
		 */
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init_dynamic_arr,
					   kunit_array_gen_params,
					   example_param_init_dynamic_arr,
					   example_param_exit_dynamic_arr),
		{}
	};

##### 娣诲姞鍏变韩璧勬簮

鏈鏋朵腑鐨勬墍鏈夊弬鏁拌繍琛岄兘鎸佹湁涓€涓鍙傛暟鍖栨祴璇曚笂涓嬫枃鐨勫紩鐢紝鍙€氳繃鐖剁骇 `struct kunit` 鎸囬拡璁块棶銆傚弬鏁板寲娴嬭瘯涓婁笅鏂囨湰韬苟涓嶇敤浜庢墽琛屼换浣曟祴璇曢€昏緫锛涚浉鍙嶏紝瀹冧綔涓哄叡浜祫婧愮殑瀹瑰櫒銆?

鍙互閫氳繃浣跨敤 `KUNIT_CASE_PARAM_WITH_INIT` 鏉ユ坊鍔犲湪鍙傛暟鍖栨祴璇曠殑鍚勪釜鍙傛暟杩愯涔嬮棿鍏变韩鐨勮祫婧愶紝浣犻渶瑕佸悜瀹冧紶閫掕嚜瀹氫箟鐨?`param_init()` 鍜?`param_exit()` 鍑芥暟銆傝繖浜涘嚱鏁板垎鍒湪璇ュ弬鏁板寲娴嬭瘯涔嬪墠鍜屼箣鍚庡悇杩愯涓€娆°€?

`param_init()` 鍑芥暟绛惧悕涓?`int (**)(struct kunit **test)`锛屽彲鐢ㄤ簬鍚戝弬鏁板寲娴嬭瘯涓婁笅鏂囩殑 `resources` 鎴?`priv` 瀛楁娣诲姞璧勬簮銆佹敞鍐屽弬鏁版暟缁勶紝浠ュ強浠讳綍鍏朵粬鍒濆鍖栭€昏緫銆?

`param_exit()` 鍑芥暟绛惧悕涓?`void (**)(struct kunit **test)`锛屽彲鐢ㄤ簬閲婃斁浠讳綍闈炲弬鏁板寲娴嬭瘯绠＄悊鐨勮祫婧愶紙鍗冲弬鏁板寲娴嬭瘯缁撴潫鏃朵笉浼氳嚜鍔ㄦ竻鐞嗙殑璧勬簮锛夛紝浠ュ強浠讳綍鍏朵粬閫€鍑洪€昏緫銆?

`param_init()` 鍜?`param_exit()` 閮戒細鍦ㄨ儗鍚庝紶鍏ュ弬鏁板寲娴嬭瘯涓婁笅鏂囥€傜劧鑰岋紝娴嬭瘯鐢ㄤ緥鍑芥暟鎺ユ敹鐨勬槸鍙傛暟杩愯涓婁笅鏂囥€傚洜姝わ紝瑕佷粠娴嬭瘯鐢ㄤ緥鍑芥暟涓鐞嗗拰璁块棶鍏变韩璧勬簮锛屼綘蹇呴』浣跨敤 `test->parent`銆?

渚嬪锛屾煡鎵剧敱 Resource API 鍒嗛厤鐨勫叡浜祫婧愰渶瑕佸皢 `test->parent` 浼犻€掔粰 `kunit_find_resource()`銆傝繖涓€鍘熷垯涔熼€傜敤浜庢祴璇曠敤渚嬪嚱鏁颁腑鍙兘浣跨敤鐨勬墍鏈夊叾浠?API锛屽寘鎷?`kunit_kzalloc()`銆乣kunit_kmalloc_array()` 绛夛紙鍙傝 Documentation/dev-tools/kunit/api/test.rst 鍜?Documentation/dev-tools/kunit/api/resource.rst锛夈€?

   `suite->init()` 鍑芥暟鍦ㄦ瘡涓弬鏁拌繍琛屼箣鍓嶆墽琛岋紝瀹冩帴鏀剁殑鏄弬鏁拌繍琛屼笂涓嬫枃銆傚洜姝わ紝鍦?
   `suite->init()` 涓缃殑浠讳綍璧勬簮閮戒細鍦ㄦ瘡涓弬鏁拌繍琛屼箣鍚庤娓呯悊銆?

涓嬮潰鐨勪唬鐮佸睍绀轰簡濡備綍娣诲姞鍏变韩璧勬簮銆傛敞鎰忔浠ｇ爜浣跨敤浜?Resource API锛屼綘鍙互鍦ㄤ互涓嬩綅缃槄璇绘洿澶氬唴瀹癸細Documentation/dev-tools/kunit/api/resource.rst銆傝鏌ョ湅姝や唬鐮佺殑瀹屾暣鐗堟湰锛岃鍙傝€?lib/kunit/kunit-example-test.c銆?


	static int example_resource_init(struct kunit_resource **res, void **context)
	{
		... /** Code that allocates memory and stores context in res->data. **/
	}

	/** This function deallocates memory for the kunit_resource->data field. **/
	static void example_resource_free(struct kunit_resource *res)
	{
		kfree(res->data);
	}

	/** This match function locates a test resource based on defined criteria. **/
	static bool example_resource_alloc_match(struct kunit **test, struct kunit_resource **res,
						 void *match_data)
	{
		return res->data && res->free == example_resource_free;
	}

	/** Function to initialize the parameterized test. **/
	static int example_param_init(struct kunit *test)
	{
		int ctx = 3; /** Data to be stored. **/
		void *data = kunit_alloc_resource(test, example_resource_init,
						  example_resource_free,
						  GFP_KERNEL, &ctx);
		if (!data)
			return -ENOMEM;
		kunit_register_params_array(test, example_params_array,
					    ARRAY_SIZE(example_params_array));
		return 0;
	}

	/** Example test that uses shared resources in test->resources. **/
	static void example_params_test_with_init(struct kunit *test)
	{
		int threshold;
		const struct example_param *param = test->param_value;
		/**  Here we pass test->parent to access the parameterized test context. **/
		struct kunit_resource *res = kunit_find_resource(test->parent,
								 example_resource_alloc_match,
								 NULL);

		threshold = **((int **)res->data);
		KUNIT_ASSERT_LE(test, param->value, threshold);
		kunit_put_resource(res);
	}

	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init, kunit_array_gen_params,
					   example_param_init, NULL),
		{}
	};

浣滀负浣跨敤 KUnit Resource API 鍏变韩璧勬簮鐨勬浛浠ｆ柟妗堬紝浣犲彲浠ュ皢瀹冧滑鏀惧叆 `test->parent->priv`銆傝繖鏄竴绉嶆洿杞婚噺绾х殑璧勬簮瀛樺偍鏂规硶锛屾渶閫傚悎涓嶉渶瑕佸鏉傝祫婧愮鐞嗙殑鍦烘櫙銆?

濡傚墠鎵€杩帮紝`param_init()` 鍜?`param_exit()` 鑾峰彇鐨勬槸鍙傛暟鍖栨祴璇曚笂涓嬫枃銆傚洜姝わ紝浣犲彲浠ュ湪 `param_init/exit` 涓洿鎺ヤ娇鐢?`test->priv` 鏉ョ鐞嗗叡浜祫婧愩€備絾鏄紝浠庢祴璇曠敤渚嬪嚱鏁板唴閮紝浣犲繀椤诲悜涓婂鑸埌鐖剁骇 `struct kunit`锛屽嵆鍙傛暟鍖栨祴璇曚笂涓嬫枃銆傚洜姝わ紝浣犻渶瑕佷娇鐢?`test->parent->priv` 鏉ヨ闂繖浜涚浉鍚岀殑璧勬簮銆?

鏀剧疆鍦?`test->parent->priv` 涓殑璧勬簮闇€瑕佸湪鍐呭瓨涓垎閰嶏紝浠ヤ究鍦ㄥ悇涓弬鏁拌繍琛屼箣闂存寔缁瓨鍦ㄣ€傚鏋滀娇鐢?KUnit 鍐呭瓨鍒嗛厤 API锛堝湪涓嬮潰鐨?鍒嗛厤鍐呭瓨"灏忚妭涓湁鏇村璇存槑锛夊垎閰嶅唴瀛橈紝浣犲氨涓嶅繀鎷呭績閲婃斁闂銆傝繖浜?API 浼氫娇鍐呭瓨鎴愪负"鍙傛暟鍖栨祴璇曟墭绠?鐨勶紝纭繚鍦ㄥ弬鏁板寲娴嬭瘯缁撴潫鍚庤嚜鍔ㄦ竻鐞嗐€?

涓嬮潰鐨勪唬鐮佹紨绀轰簡鍦ㄥ叡浜祫婧愪腑浣跨敤 `priv` 瀛楁鐨勭ず渚嬶細


	static const struct example_param {
		int value;
	} example_params_array[] = {
		{ .value = 3, },
		{ .value = 2, },
		{ .value = 1, },
		{ .value = 0, },
	};

	/** Initialize the parameterized test context. **/
	static int example_param_init_priv(struct kunit *test)
	{
		int ctx = 3; /** Data to be stored. **/
		int arr_size = ARRAY_SIZE(example_params_array);

		/*
   - Allocate memory using kunit_kzalloc(). Since the `param_init`
   - function receives the parameterized test context, this memory
   - allocation will be scoped to the lifetime of the parameterized test.
		 */
		test->priv = kunit_kzalloc(test, sizeof(int), GFP_KERNEL);

		/** Assign the context value to test->priv.**/
		**((int **)test->priv) = ctx;

		/** Register the parameter array. **/
		kunit_register_params_array(test, example_params_array, arr_size, NULL);
		return 0;
	}

	static void example_params_test_with_init_priv(struct kunit *test)
	{
		int threshold;
		const struct example_param *param = test->param_value;

		/** By design, test->parent will not be NULL. **/
		KUNIT_ASSERT_NOT_NULL(test, test->parent);

		/** Here we use test->parent->priv to access the shared resource. **/
		threshold = **(int **)test->parent->priv;

		KUNIT_ASSERT_LE(test, param->value, threshold);
	}

	static struct kunit_case example_tests[] = {
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init_priv,
					   kunit_array_gen_params,
					   example_param_init_priv, NULL),
		{}
	};

### 鍒嗛厤鍐呭瓨


鍦ㄤ綘浼氫娇鐢?`kzalloc` 鐨勫湴鏂癸紝鍙互鏀圭敤 `kunit_kzalloc`锛屽洜涓?KUnit 浼氱‘淇濆唴瀛樺湪娴嬭瘯瀹屾垚鍚庤閲婃斁銆?

杩欏緢鏈夌敤锛屽洜涓哄畠璁╂垜浠彲浠ヤ娇鐢?`KUNIT_ASSERT_EQ` 瀹忓湪娴嬭瘯涓彁鍓嶉€€鍑猴紝鑰屾棤闇€鎷呭績璁板緱璋冪敤 `kfree`銆備緥濡傦細


	void example_test_allocation(struct kunit *test)
	{
		char *buffer = kunit_kzalloc(test, 16, GFP_KERNEL);
		/** Ensure allocation succeeded. **/
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buffer);

		KUNIT_ASSERT_STREQ(test, buffer, "");
	}

### 娉ㄥ唽娓呯悊鍔ㄤ綔


濡傛灉浣犻渶瑕佹墽琛屼竴浜涜秴鍑虹畝鍗曚娇鐢?`kunit_kzalloc` 鐨勬竻鐞嗗伐浣滐紝鍙互娉ㄥ唽涓€涓嚜瀹氫箟鐨?寤惰繜鍔ㄤ綔锛坉eferred action锛?锛屽畠鏄湪娴嬭瘯閫€鍑烘椂杩愯鐨勪竴涓竻鐞嗗嚱鏁帮紙鏃犺鏄共鍑€閫€鍑猴紝杩樻槸鍥犱负鏂█澶辫触閫€鍑猴級銆?

鍔ㄤ綔锛坅ction锛夋槸娌℃湁杩斿洖鍊笺€佸彧鏈変竴涓?`void*` 涓婁笅鏂囧弬鏁扮殑绠€鍗曞嚱鏁帮紝瀹冧滑鎵紨鐨勮鑹蹭笌 Python 鍜?Go 娴嬭瘯涓殑"cleanup"鍑芥暟銆佹敮鎸佽鐗规€х殑璇█涓殑"defer"璇彞锛屼互鍙婏紙鍦ㄦ煇浜涙儏鍐典笅锛塕AII 璇█涓殑鏋愭瀯鍑芥暟鐩稿悓銆?

杩欎簺瀵逛簬浠庡叏灞€鍒楄〃涓敞閿€鏌愪簺鍐呭銆佸叧闂枃浠舵垨鍏朵粬璧勬簮锛屾垨閲婃斁璧勬簮闈炲父鏈夌敤銆?

渚嬪锛?


	static void cleanup_device(void *ctx)
	{
		struct device **dev = (struct device **)ctx;

		device_unregister(dev);
	}

	void example_device_test(struct kunit *test)
	{
		struct my_device dev;

		device_register(&dev);

		kunit_add_action(test, &cleanup_device, &dev);
	}

娉ㄦ剰锛屽浜庡儚 device_unregister 杩欐牱鍙帴鍙楀崟涓寚閽堝ぇ灏忓弬鏁扮殑鍑芥暟锛屽彲浠ヤ娇鐢?`KUNIT_DEFINE_ACTION_WRAPPER()` 瀹忚嚜鍔ㄧ敓鎴愪竴涓寘瑁呭櫒锛屼緥濡傦細


	KUNIT_DEFINE_ACTION_WRAPPER(device_unregister, device_unregister_wrapper, struct device *);
	kunit_add_action(test, &device_unregister_wrapper, &dev);

浣犲簲璇ヤ紭鍏堣繖鏍峰仛锛岃€屼笉鏄墜鍔ㄨ浆鎹负 `kunit_action_t` 绫诲瀷锛屽洜涓鸿浆鎹㈠嚱鏁版寚閽堜細鐮村潖鎺у埗娴佸畬鏁存€э紙CFI锛夈€?

`kunit_add_action` 鍙兘浼氬け璐ワ紝渚嬪绯荤粺鍐呭瓨涓嶈冻鏃躲€備綘鍙互鏀圭敤 `kunit_add_action_or_reset`锛屽畠浼氬湪鏃犳硶寤惰繜鎵ц鏃剁珛鍗宠繍琛岃鍔ㄤ綔銆?

濡傛灉浣犻渶瑕佹洿澶氬湴鎺у埗娓呯悊鍑芥暟鐨勮皟鐢ㄦ椂鏈猴紝鍙互浣跨敤 `kunit_release_action` 鎻愬墠瑙﹀彂瀹冿紝鎴栦娇鐢?`kunit_remove_action` 瀹屽叏鍙栨秷瀹冦€?


### 娴嬭瘯闈欐€佸嚱鏁?


濡傛灉浣犳兂娴嬭瘯闈欐€佸嚱鏁帮紝鑰屽張涓嶆兂灏嗚繖浜涘嚱鏁版毚闇插埌娴嬭瘯涔嬪锛屼竴绉嶉€夋嫨鏄湁鏉′欢鍦板鍑虹鍙枫€傚綋鍚敤 KUnit 鏃讹紝璇ョ鍙疯鏆撮湶锛屽惁鍒欎繚鎸侀潤鎬併€傝浣跨敤姝ゆ柟娉曪紝璇烽伒寰互涓嬫ā鏉裤€?


	/** In the file containing functions to test "my_file.c" **/

	#include <kunit/visibility.h>
	#include <my_file.h>
	...
	VISIBLE_IF_KUNIT int do_interesting_thing()
	{
	...
	}
	EXPORT_SYMBOL_IF_KUNIT(do_interesting_thing);

	/** In the header file "my_file.h" **/

	#if IS_ENABLED(CONFIG_KUNIT)
		int do_interesting_thing(void);
	#endif

	/** In the KUnit test file "my_file_test.c" **/

	#include <kunit/visibility.h>
	#include <my_file.h>
	...
	MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
	...
	// Use do_interesting_thing() in tests

瑕佹煡鐪嬪畬鏁寸ず渚嬶紝璇峰弬闃呰繖涓?`patch <https://lore.kernel.org/all/20221207014024.340230-3-rmoar@google.com/>`_锛屽叾涓祴璇曡淇敼涓轰娇鐢ㄤ笂杩板畯鏈夋潯浠跺湴鏆撮湶闈欐€佸嚱鏁颁互渚涙祴璇曘€?

浣滀负涓婅堪鏂规硶鐨?*鏇夸唬鏂规**锛屼綘鍙互鏈夋潯浠跺湴 `#include` 娴嬭瘯鏂囦欢鍒?.c 鏂囦欢鐨勬湯灏俱€傝繖涓嶆帹鑽愶紝浣嗗湪闇€瑕佹椂鏈夋晥銆備緥濡傦細


	/** In "my_file.c" **/

	static int do_interesting_thing();

	#ifdef CONFIG_MY_KUNIT_TEST
	#include "my_kunit_test.c"
	#endif

### 娉ㄥ叆浠呯敤浜庢祴璇曠殑浠ｇ爜


涓庝笂闈㈡墍绀虹被浼硷紝鎴戜滑鍙互娣诲姞鐗瑰畾浜庢祴璇曠殑閫昏緫銆備緥濡傦細


	/** In my_file.h **/

	#ifdef CONFIG_MY_KUNIT_TEST
	/** Defined in my_kunit_test.c **/
	void test_only_hook(void);
	#else
	void test_only_hook(void) { }
	#endif

杩欑浠呯敤浜庢祴璇曠殑浠ｇ爜鍙互閫氳繃璁块棶褰撳墠鐨?`kunit_test` 鍙樺緱鏇存湁鐢紝濡備笅涓€鑺傛墍绀猴細**璁块棶褰撳墠娴嬭瘯**銆?

### 璁块棶褰撳墠娴嬭瘯


鍦ㄦ煇浜涙儏鍐典笅锛屾垜浠渶瑕佷粠娴嬭瘯鏂囦欢涔嬪璋冪敤浠呯敤浜庢祴璇曠殑浠ｇ爜銆傝繖鍦ㄦ彁渚涘嚱鏁扮殑"妯℃嫙锛坢ock锛?瀹炵幇锛屾垨浠庨敊璇鐞嗙▼搴忎腑浣夸换浣曞綋鍓嶆祴璇曞け璐ユ椂寰堟湁甯姪銆傛垜浠彲浠ラ€氳繃 `task_struct` 涓殑 `kunit_test` 瀛楁鏉ュ疄鐜帮紝璇ュ瓧娈靛彲浠ヤ娇鐢?`kunit/test-bug.h` 涓殑 `kunit_get_current_test()` 鍑芥暟璁块棶銆?

`kunit_get_current_test()` 鍗充娇鍦ㄦ湭鍚敤 KUnit 鏃惰皟鐢ㄤ篃鏄畨鍏ㄧ殑銆傚鏋滄湭鍚敤 KUnit锛屾垨鑰呭綋鍓嶄换鍔′腑娌℃湁杩愯娴嬭瘯锛屽畠灏嗚繑鍥?`NULL`銆傚畠浼氳缂栬瘧涓烘棤鎿嶄綔鎴栭潤鎬侀敭妫€鏌ワ紝鍥犳鍦ㄦ病鏈夋祴璇曡繍琛屾椂瀵规€ц兘褰卞搷鍙互蹇界暐涓嶈銆?

涓嬮潰鐨勭ず渚嬬敤瀹冩潵瀹炵幇 `foo` 鍑芥暟鐨勪竴涓?妯℃嫙"瀹炵幇锛?


	#include <kunit/test-bug.h> /** for kunit_get_current_test **/

	struct test_data {
		int foo_result;
		int want_foo_called_with;
	};

	static int fake_foo(int arg)
	{
		struct kunit *test = kunit_get_current_test();
		struct test_data *test_data = test->priv;

		KUNIT_EXPECT_EQ(test, test_data->want_foo_called_with, arg);
		return test_data->foo_result;
	}

	static void example_simple_test(struct kunit *test)
	{
		/* Assume priv (private, a member used to pass test data from
   - the init function) is allocated in the suite's .init */
		struct test_data *test_data = test->priv;

		test_data->foo_result = 42;
		test_data->want_foo_called_with = 1;

		/* In a real test, we'd probably pass a pointer to fake_foo somewhere
   - like an ops struct, etc. instead of calling it directly. */
		KUNIT_EXPECT_EQ(test, fake_foo(1), 42);
	}

鍦ㄦ绀轰緥涓紝鎴戜滑浣跨敤 `struct kunit` 鐨?`priv` 鎴愬憳浣滀负浠?init 鍑芥暟鍚戞祴璇曚紶閫掓暟鎹殑涓€绉嶆柟寮忋€傞€氬父 `priv` 鏄竴涓彲鐢ㄤ簬浠讳綍鐢ㄦ埛鏁版嵁鐨勬寚閽堛€傝繖姣斾娇鐢ㄩ潤鎬佸彉閲忔洿鍙楁杩庯紝鍥犱负瀹冮伩鍏嶄簡骞跺彂闂銆?

濡傛灉鎴戜滑鎯宠鏇寸伒娲讳竴浜涳紝鍙互浣跨敤涓€涓叿鍚嶇殑 `kunit_resource`銆傛瘡涓祴璇曞彲浠ユ湁澶氫釜璧勬簮锛屽畠浠叿鏈夊瓧绗︿覆鍚嶇О锛屾彁渚涗簡涓?`priv` 鎴愬憳鐩稿悓鐨勭伒娲绘€э紝鑰屼笖渚嬪杩樺厑璁歌緟鍔╁嚱鏁板垱寤鸿祫婧愯€屼笉浼氱浉浜掑啿绐併€傝繕鍙互涓烘瘡涓祫婧愬畾涔夋竻鐞嗗嚱鏁帮紝浠庤€岃交鏉鹃伩鍏嶈祫婧愭硠婕忋€傛洿澶氫俊鎭紝璇峰弬闃?Documentation/dev-tools/kunit/api/resource.rst銆?

### 浣垮綋鍓嶆祴璇曞け璐?


濡傛灉鎴戜滑鎯充娇褰撳墠娴嬭瘯澶辫触锛屽彲浠ヤ娇鐢?`kunit_fail_current_test(fmt, args...)`锛屽畠瀹氫箟鍦?`<kunit/test-bug.h>` 涓紝涓嶉渶瑕佸紩鍏?`<kunit/test.h>`銆備緥濡傦紝鎴戜滑鏈変竴涓€夐」鍙互鍦ㄦ煇浜涙暟鎹粨鏋勪笂鍚敤涓€浜涢澶栫殑璋冭瘯妫€鏌ワ紝濡備笅鎵€绀猴細


	#include <kunit/test-bug.h>

	#ifdef CONFIG_EXTRA_DEBUG_CHECKS
	static void validate_my_data(struct data *data)
	{
		if (is_valid(data))
			return;

		kunit_fail_current_test("data %p is invalid", data);

		/** Normal, non-KUnit, error reporting code here. **/
	}
	#else
	static void my_debug_function(void) { }
	#endif

`kunit_fail_current_test()` 鍗充娇鍦ㄦ湭鍚敤 KUnit 鏃惰皟鐢ㄤ篃鏄畨鍏ㄧ殑銆傚鏋滄湭鍚敤 KUnit锛屾垨鑰呭綋鍓嶄换鍔′腑娌℃湁杩愯娴嬭瘯锛屽畠灏嗕粈涔堥兘涓嶅仛銆傚畠浼氳缂栬瘧涓烘棤鎿嶄綔鎴栭潤鎬侀敭妫€鏌ワ紝鍥犳鍦ㄦ病鏈夋祴璇曡繍琛屾椂瀵规€ц兘褰卞搷鍙互蹇界暐涓嶈銆?

### 绠＄悊妯℃嫙璁惧涓庨┍鍔?


鍦ㄦ祴璇曢┍鍔ㄦ垨涓庨┍鍔ㄤ氦浜掔殑浠ｇ爜鏃讹紝璁稿鍑芥暟灏嗛渶瑕佷竴涓?`struct device` 鎴?`struct device_driver`銆傚湪璁稿鎯呭喌涓嬶紝娴嬭瘯鏌愪釜缁欏畾鍑芥暟骞朵笉闇€瑕佽缃竴涓湡瀹炵殑璁惧锛屽洜姝ゅ彲浠ヤ娇鐢ㄤ竴涓ā鎷熻澶囨潵浠ｆ浛銆?

KUnit 鎻愪緵浜嗙敤浜庡垱寤哄拰绠＄悊杩欎簺妯℃嫙璁惧鐨勮緟鍔╁嚱鏁帮紝瀹冧滑鍦ㄥ唴閮ㄦ槸 `struct kunit_device` 绫诲瀷锛屽苟鎸傝浇鍒颁竴涓壒娈婄殑 `kunit_bus` 涓娿€傝繖浜涜澶囨敮鎸佹墭绠＄殑璁惧璧勬簮锛坉evres锛夛紝濡?Documentation/driver-api/driver-model/devres.rst 鎵€杩般€?

瑕佸垱寤轰竴涓敱 KUnit 鎵樼鐨?`struct device_driver`锛屼娇鐢?`kunit_driver_create()`锛屽畠灏嗗湪 `kunit_bus` 涓婂垱寤轰竴涓叿鏈夌粰瀹氬悕绉扮殑椹卞姩銆傝椹卞姩浼氬湪鐩稿簲娴嬭瘯缁撴潫鏃惰嚜鍔ㄩ攢姣侊紝浣嗕篃鍙互浣跨敤 `driver_unregister()` 鎵嬪姩閿€姣併€?

瑕佸垱寤轰竴涓ā鎷熻澶囷紝浣跨敤 `kunit_device_register()`锛屽畠灏嗗垱寤哄苟娉ㄥ唽涓€涓澶囷紝浣跨敤鐢?`kunit_driver_create()` 鍒涘缓鐨勬柊 KUnit 鎵樼椹卞姩銆傝鎻愪緵鐗瑰畾鐨勩€侀潪 KUnit 鎵樼鐨勯┍鍔紝璇锋敼鐢?`kunit_device_register_with_driver()`銆備笌鎵樼椹卞姩涓€鏍凤紝KUnit 鎵樼鐨勬ā鎷熻澶囦細鍦ㄦ祴璇曠粨鏉熸椂鑷姩娓呯悊锛屼絾涔熷彲浠ヤ娇鐢?`kunit_device_unregister()` 鎻愬墠鎵嬪姩娓呯悊銆?

鍦?`root_device_register()` 涓嶉€傜敤鐨勫満鏅笅锛屽簲浼樺厛浣跨敤 KUnit 璁惧锛涘湪璁惧骞堕潪 platform 璁惧鐨勬儏鍐典笅锛屽簲浼樺厛浣跨敤 KUnit 璁惧鑰岄潪 `platform_device_register()`銆?

渚嬪锛?


	#include <kunit/device.h>

	static void test_my_device(struct kunit *test)
	{
		struct device *fake_device;
		const char *dev_managed_string;

		// Create a fake device.
		fake_device = kunit_device_register(test, "my_device");
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fake_device)

		// Pass it to functions which need a device.
		dev_managed_string = devm_kstrdup(fake_device, "Hello, World!");

		// Everything is cleaned up automatically when the test ends.
	}
