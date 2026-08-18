
## 娴嬭瘯


鏈枃妗ｅ寘鍚浣曟祴璇曞唴鏍镐腑 Rust 浠ｇ爜鐨勬湁鐢ㄤ俊鎭€?
娴嬭瘯鍏辨湁涓夌锛?
- KUnit 娴嬭瘯銆?- `#[test]` 娴嬭瘯銆?- Kselftest锛堝唴鏍歌嚜娴嬭瘯锛夈€?
### KUnit 娴嬭瘯


杩欎簺娴嬭瘯鏉ヨ嚜 Rust 鏂囨。涓殑绀轰緥锛屽畠浠細琚浆鎹㈡垚 KUnit 娴嬭瘯銆?
######## 鐢ㄦ硶


杩欎簺娴嬭瘯鍙互閫氳繃 KUnit 杩愯銆備緥濡傞€氳繃 `kunit_tool`锛坄kunit.py`锛?```
	./tools/testing/kunit/kunit.py run --make_options LLVM=1 --arch x86_64 --kconfig_add CONFIG_RUST=y

```
鍙﹀锛孠Unit 涔熷彲浠ュ湪鍚姩鏃跺皢瀹冧滑浣滀负鍐呮牳鍐呭缓妯″潡杩愯銆傚叧浜庨€氱敤鐨?KUnit 鏂囨。锛?璇峰弬闃?Documentation/dev-tools/kunit/index.rst锛涘叧浜庡唴鏍稿唴寤轰笌鍛戒护琛屾祴璇曠殑
缁嗚妭锛岃鍙傞槄 Documentation/dev-tools/kunit/architecture.rst銆?
```
	CONFIG_KUNIT
	   Kernel hacking -> Kernel Testing and Coverage -> KUnit - Enable support for unit tests
	CONFIG_RUST_KERNEL_DOCTESTS
	   Kernel hacking -> Rust hacking -> Doctests for the `kernel` crate

```
鍦ㄥ唴鏍搁厤缃郴缁熶腑銆?
######## KUnit 娴嬭瘯鍗虫枃妗ｆ祴璇?

杩欎簺鏂囨。娴嬭瘯閫氬父鏄换鎰忔潯鐩紙渚嬪鍑芥暟銆佺粨鏋勪綋銆佹ā鍧椻€︹€︼級鐨勭敤娉曠ず渚嬨€?
瀹冧滑闈炲父鏂逛究锛屽洜涓哄彧闇€鍐欏湪鏂囨。鏃佽竟鍗冲彲銆備緥濡傦細


	/// Sums two numbers.
	///
	/// ```
	/// assert_eq!(mymod::f(10, 20), 30);
	/// ```
	pub fn f(a: i32, b: i32) -> i32 {
	    a + b
	}

鍦ㄧ敤鎴风┖闂翠腑锛岃繖浜涙祴璇曠敱 `rustdoc` 鏀堕泦骞惰繍琛屻€傜洿鎺ヤ娇鐢ㄨ宸ュ叿宸茬粡寰堟湁鐢紝鍥犱负
瀹冨彲浠ラ獙璇佺ず渚嬭兘澶熺紪璇戯紙浠庤€屽己鍒跺畠浠笌鎵€鏂囨。鍖栫殑浠ｇ爜淇濇寔鍚屾锛夛紝鍚屾椂涔熷彲浠?杩愯閭ｄ簺涓嶄緷璧栧唴鏍稿唴 API 鐨勭ず渚嬨€?
鐒惰€岋紝瀵逛簬鍐呮牳锛岃繖浜涙祴璇曚細琚浆鎹㈡垚 KUnit 娴嬭瘯濂椾欢銆傝繖鎰忓懗鐫€鏂囨。娴嬭瘯浼氳缂栬瘧涓?Rust 鍐呮牳瀵硅薄锛屼粠鑰岃兘澶熼拡瀵瑰凡鏋勫缓鐨勫唴鏍歌繍琛屻€?
杩欑 KUnit 闆嗘垚鐨勪竴涓ソ澶勬槸锛孯ust 鏂囨。娴嬭瘯鍙互澶嶇敤宸叉湁鐨?```
	KTAP version 1
	1..1
	    KTAP version 1
	    # Subtest: rust_doctests_kernel
	    1..59
	    # rust_doctest_kernel_build_assert_rs_0.location: rust/kernel/build_assert.rs:13
	    ok 1 rust_doctest_kernel_build_assert_rs_0
	    # rust_doctest_kernel_build_assert_rs_1.location: rust/kernel/build_assert.rs:56
	    ok 2 rust_doctest_kernel_build_assert_rs_1
	    # rust_doctest_kernel_init_rs_0.location: rust/kernel/init.rs:122
	    ok 3 rust_doctest_kernel_init_rs_0
	    ...
	    # rust_doctest_kernel_types_rs_2.location: rust/kernel/types.rs:150
	    ok 59 rust_doctest_kernel_types_rs_2
	# rust_doctests_kernel: pass:59 fail:0 skip:0 total:59
	# Totals: pass:59 fail:0 skip:0 total:59
	ok 1 rust_doctests_kernel

```
浣跨敤 `? <https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator>`_
杩愮畻绗︾殑娴嬭瘯涔熺収甯告敮鎸侊紝渚嬪锛?

	/// ```
	/// # use kernel::{spawn_work_item, workqueue};
	/// spawn_work_item!(workqueue::system(), || pr_info!("x\n"))?;
	/// # Ok::<(), Error>(())
	/// ```

杩欎簺娴嬭瘯涔熶細鍦?`CLIPPY=1` 涓嬩娇鐢?Clippy 杩涜缂栬瘧锛屽氨鍍忔櫘閫氫唬鐮佷竴鏍凤紝鍥犳涔熻兘
鍙楃泭浜庨澶栫殑 lint 妫€鏌ャ€?
涓轰簡璁╁紑鍙戣€呰兘澶熻交鏉剧湅鍒版槸鍝竴琛屾枃妗ｆ祴璇曚唬鐮佸鑷翠簡澶辫触锛屼細鍚戞棩蹇楁墦鍗颁竴琛?KTAP 璇婃柇淇℃伅銆傚叾涓寘鍚師濮嬫祴璇曠殑浣嶇疆锛堟枃浠跺拰琛屽彿锛夛紝鍗筹紙鑰岄潪杞崲鍚庝唬鐮佷腑鐨?浣嶇疆锛?```
	# rust_doctest_kernel_types_rs_2.location: rust/kernel/types.rs:150

```
Rust 娴嬭瘯浼间箮浣跨敤 Rust 鏍囧噯搴擄紙`core`锛変腑甯哥敤鐨?`assert!` 鍜?`assert_eq!`
瀹忔潵杩涜鏂█銆傛垜浠彁渚涗簡涓€涓嚜瀹氫箟鐗堟湰锛屽皢璋冪敤杞彂鍒?KUnit銆傞噸瑕佺殑鏄紝杩欎簺
瀹忎笉闇€瑕佷紶鍏ヤ笂涓嬫枃锛坈ontext锛夛紝杩欎笌 KUnit 娴嬭瘯鎵€鐢ㄧ殑瀹忥紙鍗?`struct kunit *`锛?涓嶅悓銆傝繖浣垮緱瀹冧滑鏇存槗浣跨敤锛屽苟涓旀枃妗ｇ殑璇昏€呮棤闇€鍏冲績浣跨敤鐨勬槸鍝釜娴嬭瘯妗嗘灦銆傛澶栵紝
杩欏彲鑳借鎴戜滑鍦ㄦ湭鏉ユ洿杞绘澗鍦版祴璇曠涓夋柟浠ｇ爜銆?
褰撳墠鐨勪竴涓檺鍒舵槸 KUnit 涓嶆敮鎸佸湪鍏朵粬浠诲姟涓繘琛屾柇瑷€銆傚洜姝わ紝鎴戜滑鐩墠濡傛灉鏂█
纭疄澶辫触锛屽氨鍙槸鍚戝唴鏍告棩蹇楁墦鍗颁竴涓敊璇€傚彟澶栵紝鏂囨。娴嬭瘯涓嶄細閽堝闈炲叕寮€鍑芥暟杩愯銆?
鐢变簬杩欎簺娴嬭瘯灏辨槸绀轰緥锛屽嵆瀹冧滑鏄枃妗ｇ殑涓€閮ㄥ垎锛屽洜姝ら€氬父搴斿綋鍍忊€滅湡瀹炰唬鐮佲€濋偅鏍风紪鍐欍€?鍥犳锛屼緥濡傦紝涓庡叾浣跨敤 `unwrap()` 鎴?`expect()`锛屼笉濡備娇鐢?`?` 杩愮畻绗︺€傛洿澶氳儗鏅?璇峰弬瑙侊細

	https://rust.docs.kernel.org/kernel/error/type.Result.html#error-codes-in-c-and-rust

### ``#[test]`` 娴嬭瘯


姝ゅ锛岃繕鏈?`#[test]` 娴嬭瘯銆備笌鏂囨。娴嬭瘯绫讳技锛屽畠浠篃涓庝綘鍦ㄧ敤鎴风┖闂存墍鏈熸湜鐨勯涓?鐩镐技锛屽苟涓斿畠浠篃琚槧灏勫埌 KUnit銆?
杩欎簺娴嬭瘯鐢?`kunit_tests` 杩囩▼瀹忓紩鍏ワ紝璇ュ畯浠ユ祴璇曞浠剁殑鍚嶇О浣滀负鍙傛暟銆?
渚嬪锛屽亣璁炬垜浠娴嬭瘯鏂囨。娴嬭瘯灏忚妭涓殑鍑芥暟 `f`銆傛垜浠彲浠ュ湪涓庡嚱鏁版墍鍦ㄧ殑鍚屼竴涓?鏂囦欢涓紪鍐欙細


	#[kunit_tests(rust_kernel_mymod)]
	mod tests {
	    use super::*;

	    #[test]
	    fn test_f() {
	        assert_eq!(f(10, 20), 30);
	    }
	}

```
	    KTAP version 1
	    # Subtest: rust_kernel_mymod
	    # speed: normal
	    1..1
	    # test_f.speed: normal
	    ok 1 test_f
	ok 1 rust_kernel_mymod

```
涓庢枃妗ｆ祴璇曚竴鏍凤紝`assert!` 鍜?`assert_eq!` 瀹忚鏄犲皠鍥?KUnit锛屼笖涓嶄細 panic銆?绫讳技鍦帮紝`? <https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator>`_
杩愮畻绗︿篃鍙楁敮鎸侊紝鍗虫祴璇曞嚱鏁板彲浠ヨ繑鍥炵┖锛堝嵆鍗曞厓绫诲瀷 `()`锛夋垨 `Result`锛堝嵆浠绘剰
`Result<T, E>`锛夈€備緥濡傦細


	#[kunit_tests(rust_kernel_mymod)]
	mod tests {
	    use super::*;

	    #[test]
	    fn test_g() -> Result {
	        let x = g()?;
	        assert_eq!(x, 30);
	        Ok(())
	    }
	}

```
	    KTAP version 1
	    # Subtest: rust_kernel_mymod
	    # speed: normal
	    1..1
	    # test_g: ASSERTION FAILED at rust/kernel/lib.rs:335
	    Expected is_test_result_ok(test_g()) to be true, but is false
	    # test_g.speed: normal
	    not ok 1 test_g
	not ok 1 rust_kernel_mymod

```
濡傛灉涓€涓?`#[test]` 娴嬭瘯鑳戒綔涓虹敤鎴风殑绀轰緥鑰屾湁浠峰€硷紝閭ｄ箞璇锋敼鐢ㄦ枃妗ｆ祴璇曘€傚嵆浣挎槸
API 鐨勮竟鐣屾儏鍐碉紝渚嬪閿欒鎴栬竟鐣屾儏褰紝涔熷€煎緱鍦ㄧず渚嬩腑灞曠ず銆?
### ``rusttest`` 涓绘満娴嬭瘯


杩欎簺鏄敤鎴风┖闂存祴璇曪紝鍙互鍦ㄤ富鏈猴紙鍗宠繍琛岀紪璇戠殑鐜锛変笂鏋勫缓骞惰繍琛?```
	make LLVM=1 rusttest

```
杩欓渶瑕佸唴鏍哥殑 `.config`銆?
鐩墠锛屽畠浠富瑕佺敤浜庢祴璇?`macros` crate 鐨勭ず渚嬨€?
### Kselftest锛堝唴鏍歌嚜娴嬭瘯锛?

Kselftest 涔熷彲鍦?`tools/testing/selftests/rust` 鐩綍涓壘鍒般€?
娴嬭瘯鎵€闇€鐨勫唴鏍搁厤缃€夐」鍒楀湪 `tools/testing/selftests/rust/config` 鏂囦欢涓紝
鍙€熷姪浠ヤ笅鍛戒护鍖呭惈杩涙潵
```
	./scripts/kconfig/merge_config.sh .config tools/testing/selftests/rust/config

```
Kselftest 鍦ㄥ唴鏍告簮鐮佹爲鍐呮瀯寤猴紝鏃ㄥ湪杩愯浜庡畨瑁呬簡鐩稿悓鍐呮牳鐨勭郴缁熶笂銆?
涓€鏃﹀畨瑁呭苟鍚姩浜嗕笌婧愮爜鏍戝尮閰嶇殑鍐呮牳锛屾墽琛?```
	make TARGETS="rust" kselftest

```
鍏充簬閫氱敤鐨?Kselftest 鏂囨。锛岃鍙傞槄 Documentation/dev-tools/kselftest.rst銆?