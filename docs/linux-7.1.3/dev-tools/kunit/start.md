
## Getting Started


姝?椤?鍖呭惈 涓€涓?overview 鐨?the kunit_tool 鍜?KUnit framework,
teaching 濡備綍 鍒?杩愯 existing tests 鍜?鐒跺悗 濡備綍 鍒?鍐欏叆 涓€涓?绠€鍗?test case,
鍜?covers 閫氱敤 problems users face 褰?浣跨敤 KUnit 鐢ㄤ簬 the 绗竴 time.

## Installing Dependencies

KUnit 鍏锋湁 the 鐩稿悓 dependencies 浣滀负 the Linux 鍐呮牳. 鍙 鎮ㄥ彲浠?
build the 鍐呮牳, 鎮ㄥ彲浠?杩愯 KUnit.

## 杩愯涓?tests 涓?kunit_tool

kunit_tool 鏄?涓€涓?Python script, 鍏?configures 鍜?builds 涓€涓?鍐呮牳, runs
tests, 鍜?formats the test results. 鏉ヨ嚜 the 鍐呮牳 repository, 鎮?
鍙?杩愯 kunit_tool:


	./tools/testing/kunit/kunit.py 杩愯

	鎮?鍙?鍙傝 the 浠ヤ笅 閿欒:
	"The source tree 鏄?涓?clean, 璇?杩愯 'make ARCH=um mrproper'"

	姝?happens 鍥犱负 internally kunit.py specifies `.kunit`
	(榛樿 閫夐」) 浣滀负 the build directory 鍦?the 鍛戒护 `make O=output/dir`
	through the 鍙傛暟 `--build_dir`.  Hence, 涔嬪墠 starting 涓€涓?
	out-of-tree build, the source tree 蹇呴』 涓?clean.

	瀛樺湪 涔?the 鐩稿悓 caveat mentioned 鍦?the "Build directory 鐢ㄤ簬
	the 鍐呮牳" section 鐨?the [admin-guide </admin-guide/README>](admin-guide </admin-guide/README>),
	鍗? 鍏?浣跨敤, 瀹?蹇呴』 涓?浣跨敤 鐢ㄤ簬 鍏ㄩ儴 invocations 鐨?`make`.
	The good news 鏄?璇?瀹?鍙?indeed 涓?solved 鐢?杩愯涓?
	`make ARCH=um mrproper`, just 涓?aware 璇?姝?灏?鍒犻櫎 the
	鐢垫祦 閰嶇疆 鍜?鍏ㄩ儴 generated 鏂囦欢.

鑻?everything worked correctly, 鎮?搴斿綋 鍙傝 the 浠ヤ笅:


	Configuring KUnit 鍐呮牳 ...
	Building KUnit 鍐呮牳 ...
	Starting KUnit 鍐呮牳 ...

The tests 灏?pass 鎴?fail.

   鍥犱负 瀹冩槸 building 涓€涓?lot 鐨?sources 鐢ㄤ簬 the 绗竴 time,
   the `Building KUnit Kernel` step 鍙?take 涓€涓?鍚屾椂.

鐢ㄤ簬 detailed information 鍦?姝?wrapper, 鍙傝:
Documentation/dev-tools/kunit/杩愯_wrapper.rst.

### Selecting 鍏?tests 鍒?杩愯


榛樿鎯呭喌涓? kunit_tool runs 鍏ㄩ儴 tests reachable 涓?minimal 閰嶇疆,
鍗? 浣跨敤 榛樿 鍊?鐢ㄤ簬 澶у鏁?鐨?the kconfig 閫夐」.  鐒惰€?
鎮ㄥ彲浠?select 鍏?tests 鍒?杩愯 鐢?

- `Customizing Kconfig`_ 浣跨敤 鍒?compile the 鍐呮牳, 鎴?
- `Filtering tests by name`_ 鍒?select specifically 鍏?compiled tests 鍒?杩愯.

#### Customizing Kconfig

涓€涓?good starting point 鐢ㄤ簬 the `.kunitconfig` 鏄?the KUnit 榛樿 閰嶇疆.
鑻?鎮?didn't 杩愯 `kunit.py run` 灏氭湭, 鎮ㄥ彲浠?generate 瀹?鐢?杩愯涓?


	cd $PATH_鍒癬LINUX_REPO
	tools/testing/kunit/kunit.py 閰嶇疆
	cat .kunit/.kunitconfig

   `.kunitconfig` lives 鍦?the `--build_dir` 浣跨敤 鐢?kunit.py, 鍏?鏄?
   `.kunit` 榛樿鎯呭喌涓?

涔嬪墠 杩愯涓?the tests, kunit_tool ensures 璇?鍏ㄩ儴 閰嶇疆 閫夐」
set 鍦?`.kunitconfig` 鏄?set 鍦?the 鍐呮牳 `.config`. 瀹?灏?warn
鎮?鑻?鎮?鍏锋湁 涓?included dependencies 鐢ㄤ簬 the 閫夐」 浣跨敤.

瀛樺湪 璁稿 ways 鍒?customize the configurations:

涓€涓? Edit `.kunit/.kunitconfig`. The 鏂囦欢 搴斿綋 鍖呭惈 the 鍒楀嚭 鐨?kconfig
   閫夐」 蹇呴渶 鍒?杩愯 the desired tests, including 瀹冧滑鐨?dependencies.
   鎮?鍙?甯屾湜 鍒?remove 閰嶇疆_KUNIT_鍏ㄩ儴_TESTS 鏉ヨ嚜 the `.kunitconfig` 浣滀负
   瀹?灏?鍚敤 涓€涓?鏁板瓧 鐨?棰濆 tests 璇?鎮?鍙?涓?甯屾湜.
   鑻?鎮?闇€瑕?鍒?杩愯 鍦?涓€涓?architecture 鍏朵粬 姣?UML 鍙傝 kunit-on-qemu.

b. 鍚敤 棰濆 kconfig 閫夐」 鍦ㄢ€︿箣涓?`.kunit/.kunitconfig`.
```

	./tools/testing/kunit/kunit.py run \
		--kconfig_add CONFIG_LIST_KUNIT_TEST=y

```
c. 鎻愪緵 the path 鐨?one 鎴?鏇村 .kunitconfig 鏂囦欢 鏉ヨ嚜 the tree.
```

	./tools/testing/kunit/kunit.py run \
		--kunitconfig ./fs/fat/.kunitconfig \
		--kunitconfig ./fs/ext4/.kunitconfig

```
d. 鑻?鎮?change the `.kunitconfig`, kunit.py 灏?trigger 涓€涓?rebuild 鐨?the
   `.config` 鏂囦欢. 浣?鎮ㄥ彲浠?edit the `.config` 鏂囦欢 directly 鎴?涓?
   tools 绫讳技 `make menuconfig O=.kunit`. 鍙 鍏?涓€涓?superset 鐨?
   `.kunitconfig`, kunit.py won't overwrite 鎮ㄧ殑 changes.



```

		make savedefconfig O=.kunit
		cp .kunit/defconfig .kunit/.kunitconfig

```
#### Filtering tests 鐢?name

鑻?鎮?甯屾湜 鍒?涓?鏇村 鐗瑰畾 姣?Kconfig 鍙?鎻愪緵, 瀹冩槸 涔?鍙兘
鍒?select 鍏?tests 鍒?execute 鍦?boot-time 鐢?passing 涓€涓?glob filter
(璇诲彇 instructions regarding the pattern 鍦?the manpage `glob(7)`).
鑻?瀛樺湪 涓€涓?`"."` (period) 鍦?the filter, 瀹?灏?涓?interpreted 浣滀负 涓€涓?
separator 涔嬮棿 the name 鐨?the test suite 鍜?the test case,
鍚﹀垯, 瀹?灏?涓?interpreted 浣滀负 the name 鐨?the test suite.
渚嬪, let's assume 鎴戜滑 鏄?浣跨敤 the 榛樿 閰嶇疆:

涓€涓? inform the name 鐨?涓€涓?test suite, 绫讳技 `"kunit_executor_test"`,
```

	./tools/testing/kunit/kunit.py run "kunit_executor_test"

```
b. inform the name 鐨?涓€涓?test case prefixed 鐢?鍏?test suite,
```

	./tools/testing/kunit/kunit.py run "example.example_simple_test"

```
c. 浣跨敤 wildcard characters (`*?[`) 鍒?杩愯 浠讳綍 test case 璇?matches the pattern,
   绫讳技 `"**.**64*"` 鍒?杩愯 test cases containing `"64"` 鍦?the name inside
```

	./tools/testing/kunit/kunit.py run "*.*64*"

```
## 杩愯涓?Tests 鏃?the KUnit Wrapper

鑻?鎮?鎵ц 涓?甯屾湜 鍒?浣跨敤 the KUnit Wrapper (渚嬪: 鎮?甯屾湜 code
鍦ㄢ€︿笅 test 鍒?integrate 涓?鍏朵粬 绯荤粺, 鎴?浣跨敤 涓€涓?涓嶅悓/
涓嶅彈鏀寔 architecture 鎴?閰嶇疆), KUnit 鍙?涓?included 鍦?
浠讳綍 鍐呮牳, 鍜?the results 鏄?璇诲彇 out 鍜?parsed manually.

   `CONFIG_KUNIT` 搴斿綋 涓?涓?宸插惎鐢?鍦?涓€涓?production environment.
   Enabling KUnit disables 鍐呮牳 Address-Space Layout Randomization
   (KASLR), 鍜?tests 鍙?affect the 鐘舵€?鐨?the 鍐呮牳 鍦?ways 涓?
   suitable 鐢ㄤ簬 production.

### Configuring the 鍐呮牳

鍒?鍚敤 KUnit itself, 鎮?闇€瑕?鍒?鍚敤 the `CONFIG_KUNIT` Kconfig
閫夐」 (鍦ㄢ€︿笅 鍐呮牳 Hacking/鍐呮牳 Testing 鍜?Coverage 鍦?
`menuconfig`). 鏉ヨ嚜 閭ｉ噷, 鎮ㄥ彲浠?鍚敤 浠讳綍 KUnit tests. 瀹冧滑
閫氬父 鍏锋湁 閰嶇疆 閫夐」 ending 鍦?`_KUNIT_TEST`.

KUnit 鍜?KUnit tests 鍙?涓?compiled 浣滀负 妯″潡. The tests 鍦?涓€涓?妯″潡
灏?杩愯 褰?the 妯″潡 鏄?loaded.

### 杩愯涓?Tests (鏃?KUnit Wrapper)

Build 鍜?杩愯 鎮ㄧ殑 鍐呮牳. 鍦?the 鍐呮牳 log, the test 杈撳嚭 鏄?printed
out 鍦?the TAP 鏍煎紡. 姝?灏?浠?happen 榛樿鎯呭喌涓?鑻?KUnit/tests
鏄?built-in. 鍚﹀垯 the 妯″潡 灏?闇€瑕?鍒?涓?loaded.

   涓€浜?lines 鍜?鎴?鏁版嵁 鍙?get interspersed 鍦?the TAP 杈撳嚭.

## Writing 鎮ㄧ殑 绗竴 Test

鍦?鎮ㄧ殑 鍐呮牳 repository, let's add 涓€浜?code 璇?鎴戜滑鍙互 test.

1. 鍒涘缓 涓€涓?鏂囦欢 `drivers/misc/example.h`, 鍏?鍖呭惈:


	int misc_绀轰緥_add(int left, int right);

2. 鍒涘缓 涓€涓?鏂囦欢 `drivers/misc/example.c`, 鍏?鍖呭惈:


	#鍖呭惈 <linux/errno.h>

	#鍖呭惈 "绀轰緥.h"

	int misc_绀轰緥_add(int left, int right)
	{
		return left + right;
	}

3. Add the 浠ヤ笅 lines 鍒?`drivers/misc/Kconfig`:


	閰嶇疆 MISC_绀轰緥
		bool "My 绀轰緥"

4. Add the 浠ヤ笅 lines 鍒?`drivers/misc/Makefile`:


	obj-$(閰嶇疆_MISC_绀轰緥) += 绀轰緥.o

鐜板湪 鎴戜滑 鏄?ready 鍒?鍐欏叆 the test cases.

1. Add the 涓嬫枃 test case 鍦?`drivers/misc/example_test.c`:


	#鍖呭惈 <kunit/test.h>
	#鍖呭惈 "绀轰緥.h"

	/** 瀹氫箟 the test cases. **/

	闈欐€?void misc_绀轰緥_add_test_鍩烘湰(缁撴瀯浣?kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, misc_绀轰緥_add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, misc_绀轰緥_add(1, 1));
		KUNIT_EXPECT_EQ(test, 0, misc_绀轰緥_add(-1, 1));
		KUNIT_EXPECT_EQ(test, INT_MAX, misc_绀轰緥_add(0, INT_MAX));
		KUNIT_EXPECT_EQ(test, -1, misc_绀轰緥_add(INT_MAX, INT_MIN));
	}

	闈欐€?void misc_绀轰緥_test_failure(缁撴瀯浣?kunit *test)
	{
		KUNIT_FAIL(test, "姝?test 浠庝笉 passes.");
	}

	闈欐€?缁撴瀯浣?kunit_case misc_绀轰緥_test_cases[] = {
		KUNIT_CASE(misc_绀轰緥_add_test_鍩烘湰),
		KUNIT_CASE(misc_绀轰緥_test_failure),
		{}
	};

	闈欐€?缁撴瀯浣?kunit_suite misc_绀轰緥_test_suite = {
		.name = "misc-example",
		.test_cases = misc_绀轰緥_test_cases,
	};
	kunit_test_suite(misc_绀轰緥_test_suite);

	妯″潡_LICENSE("GPL");

2. Add the 浠ヤ笅 lines 鍒?`drivers/misc/Kconfig`:


	閰嶇疆 MISC_绀轰緥_TEST
		tristate "Test 鐢ㄤ簬 my 绀轰緥" 鑻?!KUNIT_鍏ㄩ儴_TESTS
		depends 鍦?MISC_绀轰緥 && KUNIT
		榛樿 KUNIT_鍏ㄩ儴_TESTS

娉ㄦ剰: 鑻?鎮ㄧ殑 test 鎵ц 涓?鏀寔 姝ｅ湪 built 浣滀负 涓€涓?loadable 妯″潡 (鍏?鏄?
discouraged), replace tristate 鐢?bool, 鍜?depend 鍦?KUNIT=y 鑰岄潪 KUNIT.

3. Add the 浠ヤ笅 lines 鍒?`drivers/misc/Makefile`:


	obj-$(閰嶇疆_MISC_绀轰緥_TEST) += 绀轰緥_test.o

4. Add the 浠ヤ笅 lines 鍒?`.kunit/.kunitconfig`:


	閰嶇疆_MISC_绀轰緥=y
	閰嶇疆_MISC_绀轰緥_TEST=y

5. 杩愯 the test:


	./tools/testing/kunit/kunit.py 杩愯

鎮?搴斿綋 鍙傝 the 浠ヤ笅 failure:


	...
	[16:08:57] [PASSED] misc-example:misc_绀轰緥_add_test_鍩烘湰
	[16:08:57] [FAILED] misc-example:misc_绀轰緥_test_failure
	[16:08:57] EXPECTATION FAILED 鍦?椹卞姩/misc/example-test.c:17
	[16:08:57]      姝?test 浠庝笉 passes.
	...

Congrats! 鎮?just wrote 鎮ㄧ殑 绗竴 KUnit test.

## 鎺ヤ笅鏉?Steps


鑻?鎮?re interested 鍦?浣跨敤 涓€浜?鐨?the 鏇村 楂樼骇 鐗规€?鐨?kunit.py,
take 涓€涓?look 鍦?Documentation/dev-tools/kunit/杩愯_wrapper.rst

鑻?鎮?d 绫讳技 鍒?杩愯 tests 鏃?浣跨敤 kunit.py, check out
Documentation/dev-tools/kunit/杩愯_manual.rst

鐢ㄤ簬 鏇村 information 鍦?writing KUnit tests (including 涓€浜?閫氱敤 techniques
鐢ㄤ簬 testing 涓嶅悓 things), 鍙傝 Documentation/dev-tools/kunit/usage.rst
