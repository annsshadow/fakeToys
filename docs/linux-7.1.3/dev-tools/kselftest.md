## Linux 鍐呮牳鑷祴闆嗭紙Kselftest锛?

鍐呮牳鍦?tools/testing/selftests/ 鐩綍涓嬪寘鍚竴缁?鑷祴璇曪紙self tests锛?銆傝繖浜涙祴璇曟棬鍦ㄤ綔涓哄皬鍨嬫祴璇曪紝鐢ㄤ簬鍗曠嫭婕旂粌鍐呮牳涓殑鍚勪釜浠ｇ爜璺緞銆傛祴璇曞簲鍦ㄦ瀯寤恒€佸畨瑁呭苟鍚姩鍐呮牳涔嬪悗杩愯銆?
涓荤嚎涓殑 kselftest 鍙互鍦ㄨ緝鏃х殑绋冲畾鍐呮牳涓婅繍琛屻€傝繍琛屼富绾挎祴璇曡兘鎻愪緵鏈€濂界殑瑕嗙洊鐜囥€傛湁澶氫釜娴嬭瘯鐜細鍦ㄧǔ瀹氱増鏈笂杩愯涓荤嚎 kselftest 娴嬭瘯濂椾欢銆傚師鍥犳槸锛氬綋鏂板涓€涓敤浜庡洖褰掓祴璇曟煇宸叉湁浠ｇ爜涓己闄风殑娴嬭瘯鏃讹紝鎴戜滑搴斿綋鑳藉鍦ㄤ竴涓緝鏃х殑鍐呮牳涓婅繍琛岃娴嬭瘯銆傚洜姝わ紝淇濈暀浠嶈兘娴嬭瘯杈冩棫鍐呮牳鐨勪唬鐮侊紝骞剁‘淇濆叾鍦ㄨ緝鏂扮増鏈笂鑳藉浼橀泤鍦拌烦杩囪娴嬭瘯锛岃繖涓€鐐瑰崄鍒嗛噸瑕併€?
鍏充簬 Kselftest 妗嗘灦浠ュ強濡備綍浣跨敤璇ユ鏋剁紪鍐欐柊娴嬭瘯鐨勬洿澶氫俊鎭紝鍙弬闃?Kselftest wiki锛?
https://kselftest.wiki.kernel.org/

鍦ㄦ煇浜涚郴缁熶笂锛岀儹鎻掓嫈娴嬭瘯鍙兘浼氭案杩滄寕璧凤紝绛夊緟 cpu 鍜屽唴瀛樺氨缁互渚跨绾裤€備负姝ゅ垱寤轰簡涓€涓壒娈婄殑鐑彃鎷旂洰鏍囨潵杩愯瀹屾暣鑼冨洿鐨勭儹鎻掓嫈娴嬭瘯銆傚湪榛樿妯″紡涓嬶紝鐑彃鎷旀祴璇曚互鍙楅檺鑼冨洿鐨勫畨鍏ㄦā寮忚繍琛屻€傚湪鍙楅檺妯″紡涓嬶紝cpu 鐑彃鎷旀祴璇曞彧鍦ㄥ崟涓?cpu 涓婅繍琛岋紝鑰岄潪鎵€鏈夋敮鎸佺儹鎻掓嫈鐨?cpu锛涘唴瀛樼儹鎻掓嫈娴嬭瘯鍙湪 2% 鐨勬敮鎸佺儹鎻掓嫈鐨勫唴瀛樹笂杩愯锛岃€岄潪 10%銆?
kselftest 浣滀负鐢ㄦ埛绌洪棿杩涚▼杩愯銆傚彲浠ュ湪鐢ㄦ埛绌洪棿缂栧啓/杩愯鐨勬祴璇曞彲鑳藉笇鏈涗娇鐢?`Test Harness`_銆傞渶瑕佸湪鍐呮牳绌洪棿杩愯鐨勬祴璇曞彲鑳藉笇鏈涗娇鐢?`Test Module`_銆?
## 娴嬭瘯鐩稿叧鏂囨。


鏈夊叧 kselftests 鏈韩鐨勬枃妗ｏ紝璇峰弬闃咃細

- [testing-devices](testing-devices)

## 杩愯鑷祴锛堢儹鎻掓嫈娴嬭瘯浠ュ彈闄愭ā寮忚繍琛岋級


```
  $ make headers
  $ make -C tools/testing/selftests
```
```
  $ make -C tools/testing/selftests run_tests
```
```
  $ make kselftest
```
娉ㄦ剰锛岄儴鍒嗘祴璇曢渶瑕?root 鏉冮檺銆?
kselftest 鏀寔灏嗚緭鍑烘枃浠朵繚瀛樺埌鍗曠嫭鐨勭洰褰曚腑锛岀劧鍚庡啀杩愯娴嬭瘯銆備负浜嗗湪鍗曠嫭鐨勭洰褰曚腑瀹氫綅杈撳嚭鏂囦欢锛屾敮鎸佷袱绉嶈娉曘€備袱绉嶆儏鍐典笅宸ヤ綔鐩綍閮藉繀椤绘槸鍐呮牳婧愮爜鏍戠殑鏍圭洰褰曘€傝繖涓€鐐瑰悓鏍烽€傜敤浜庝笅鏂囩殑"杩愯鑷祴鐨勫瓙闆?涓€鑺傘€?
```
  $ make O=/tmp/kselftest kselftest
```
```
  $ export KBUILD_OUTPUT=/tmp/kselftest; make kselftest
```
O= 璧嬪€间紭鍏堜簬 KBUILD_OUTPUT 鐜鍙橀噺銆?
涓婅堪鍛戒护榛樿杩愯娴嬭瘯骞舵墦鍗板畬鏁寸殑閫氳繃/澶辫触鎶ュ憡銆俴selftest 鏀寔"summary"閫夐」浠ヤ究鏇村鏄撶悊瑙ｆ祴璇曠粨鏋溿€傚綋鎸囧畾 summary 閫夐」鏃讹紝鍙湪 /tmp/testname 鏂囦欢涓壘鍒版瘡涓祴璇曠殑璇︾粏鍗曢」缁撴灉銆傝繖涓€鐐瑰悓鏍烽€傜敤浜庝笅鏂囩殑"杩愯鑷祴鐨勫瓙闆?涓€鑺傘€?
```
  $ make summary=1 kselftest
```

## 杩愯鑷祴鐨勫瓙闆?

浣犲彲浠ュ湪 make 鍛戒护琛屼笂浣跨敤 "TARGETS" 鍙橀噺鏉ユ寚瀹氳杩愯鐨勫崟涓祴璇曪紝鎴栦竴缁勮杩愯鐨勬祴璇曘€?
```
  $ make -C tools/testing/selftests TARGETS=ptrace run_tests
```
```
  $  make TARGETS="size timers" kselftest
```
```
  $ make O=/tmp/kselftest TARGETS="size timers" kselftest
```
```
  $ export KBUILD_OUTPUT=/tmp/kselftest; make TARGETS="size timers" kselftest
```
姝ゅ锛屼綘杩樺彲浠ュ湪 make 鍛戒护琛屼笂浣跨敤 "SKIP_TARGETS" 鍙橀噺鏉ユ寚瀹氳浠?TARGETS 鍒楄〃涓帓闄ょ殑涓€涓垨澶氫釜鐩爣銆?
```
  $ make -C tools/testing/selftests SKIP_TARGETS=ptrace run_tests
```
```
  $  make SKIP_TARGETS="size timers" kselftest
```
浣犱篃鍙互鍚屾椂鎸囧畾涓€涓彈闄愮殑娴嬭瘯鍒楄〃鏉ヨ繍琛岋紝渚嬪锛?
```
  $  make TARGETS="breakpoints size timers" SKIP_TARGETS=size kselftest
```
鎵€鏈夊彲鐢ㄧ洰鏍囩殑鍒楄〃瑙侀《灞傜殑 tools/testing/selftests/Makefile銆?
## 杩愯瀹屾暣鑼冨洿鐨勭儹鎻掓嫈鑷祴


```
  $ make -C tools/testing/selftests hotplug
```
```
  $ make -C tools/testing/selftests run_hotplug
```
娉ㄦ剰锛岄儴鍒嗘祴璇曢渶瑕?root 鏉冮檺銆?

## 瀹夎鑷祴


浣犲彲浠ヤ娇鐢?"make" 鐨?"install" 鐩爣锛堝畠浼氳皟鐢?`kselftest_install.sh` 宸ュ叿锛夊皢鑷祴瀹夎鍒伴粯璁や綅缃紙`tools/testing/selftests/kselftest_install`锛夛紝鎴栭€氳繃 `INSTALL_PATH` 杩欎釜 "make" 鍙橀噺瀹夎鍒扮敤鎴锋寚瀹氱殑浣嶇疆銆?
```
   $ make -C tools/testing/selftests install
```
```
   $ make -C tools/testing/selftests install INSTALL_PATH=/some/other/path
```

## 杩愯宸插畨瑁呯殑鑷祴


鍦ㄥ畨瑁呯洰褰曚互鍙?Kselftest tar 鍖呬腑锛岄兘鏈変竴涓悕涓?`run_kselftest.sh` 鐨勮剼鏈潵杩愯娴嬭瘯銆?
浣犲彲浠ョ畝鍗曞湴鎵ц浠ヤ笅鍛戒护鏉ヨ繍琛屽凡瀹夎鐨?Kselftests銆備緥濡傦細

```
   $ cd kselftest_install
   $ ./run_kselftest.sh
```
```
   $ ./run_kselftest.sh -l
```
`-c` 閫夐」鍙敤浜庝粠涓€涓祴璇曢泦鍚堜腑杩愯鎵€鏈夋祴璇曪紝渚嬪锛?
```
   $ ./run_kselftest.sh -c size -c seccomp -t timers:posix_timers -t timer:nanosleep
```
鍏朵粬鍔熻兘璇峰弬瑙佽剼鏈殑浣跨敤杈撳嚭锛堜娇鐢?`-h` 閫夐」鏌ョ湅锛夈€?
## 鑷祴瓒呮椂


鑷祴琚璁′负杩愯杩呴€燂紝鍥犳姣忎釜娴嬭瘯榛樿浣跨敤 45 绉掔殑瓒呮椂銆傛祴璇曞彲浠ラ€氳繃鍦ㄥ叾鐩綍涓坊鍔犱竴涓?settings 鏂囦欢骞跺湪鍏朵腑璁剧疆涓€涓?timeout 鍙橀噺锛屾潵瑕嗙洊榛樿瓒呮椂锛屽皢鍏堕厤缃负璇ユ祴璇曟湡鏈涚殑涓婇檺瓒呮椂銆傚彧鏈夊皯鏁版祴璇曚細灏嗚秴鏃惰鐩栦负楂樹簬 45 绉掔殑鍊硷紝kselftest 鍔涙眰淇濇寔杩欎竴鐘跺喌銆傝嚜娴嬩腑鐨勮秴鏃朵笉琚涓烘槸鑷村懡鐨勶紝鍥犱负杩愯娴嬭瘯鐨勭郴缁熷彲鑳戒細鍙戠敓鍙樺寲锛岃繖涔熶細鏀瑰彉杩愯娴嬭瘯鐨勯鏈熻€楁椂銆傚鏋滀綘鑳芥帶鍒跺皢杩愯杩欎簺娴嬭瘯鐨勭郴缁燂紝鍙互閫氳繃鍛戒护琛屼笂鐨?`-o` 鎴?`--override-timeout` 鍙傛暟锛屽湪杩欎簺绯荤粺涓婇厤缃祴璇曡繍琛屽櫒浣跨敤涓€涓洿澶ф垨鏇村皬鐨勮秴鏃躲€備緥濡傦紝瑕佷娇鐢?165 绉掞細

```
   $ ./run_kselftest.sh --override-timeout 165
```
浣犲彲浠ユ煡鐪?TAP 杈撳嚭鏉ュ垽鏂綘鏄惁閬囧埌浜嗚秴鏃躲€傛槑纭煡閬撴煇涓祴璇曞繀椤诲湪鐗瑰畾鏃堕棿鍐呰繍琛岀殑娴嬭瘯杩愯鍣紝闅忓悗鍙互閫夋嫨鎬у湴灏嗘绫昏秴鏃惰涓鸿嚧鍛姐€?
## 鎵撳寘鑷祴


鍦ㄦ煇浜涙儏鍐典笅闇€瑕佹墦鍖咃紝渚嬪褰撴祴璇曢渶瑕佸湪鏌愪釜鐜涓嬭繍琛屾椂锛?
```
   $ make -C tools/testing/selftests gen_tar
```
杩欎細鍦?`INSTALL_PATH/kselftest-packages` 鐩綍涓敓鎴愪竴涓?tar 鍖呫€傞粯璁や娇鐢?`.gz` 鏍煎紡銆倀ar 鐨勫帇缂╂牸寮忓彲浠ラ€氳繃鎸囧畾 `FORMAT` make 鍙橀噺鏉ヨ鐩栥€備换浣曡 `tar 鐨?auto-compress`_ 璇嗗埆鐨勫€煎潎鍙娇鐢紝渚嬪锛?
```
    $ make -C tools/testing/selftests gen_tar FORMAT=.xz
```
`make gen_tar` 浼氳皟鐢?`make install`锛屽洜姝や綘鍙互缁撳悎"杩愯鑷祴鐨勫瓙闆?涓€鑺備腑鎸囧畾鐨勫彉閲忔潵鎵撳寘鑷祴鐨勪竴涓瓙闆嗭紝渚嬪锛?
```
    $ make -C tools/testing/selftests gen_tar TARGETS="size" FORMAT=.xz
```

## 璐＄尞鏂版祴璇?

涓€鑸€岃█锛岃嚜娴嬭瘯鐨勮鍒欐槸锛?
 - 濡傛灉浣犱笉鏄?root锛屽氨灏介噺澶氬仛浜嬶紱

 - 涓嶈鑰楁椂澶箙锛?
 - 涓嶈鍦ㄤ换浣曟灦鏋勪笂鐮村潖鏋勫缓锛涘苟涓?
 - 褰撲綘鐨勫姛鑳芥湭閰嶇疆鏃讹紝涓嶈璁╅《灞傜殑 "make run_tests" 澶辫触銆?
 - 娴嬭瘯鐨勮緭鍑哄繀椤荤鍚?TAP 鏍囧噯锛屼互纭繚杈冮珮鐨勬祴璇曡川閲忥紝骞朵互鍏蜂綋缁嗚妭鎹曡幏澶辫触/閿欒銆俴selftest.h 涓?kselftest_harness.h 澶存枃浠舵彁渚涗簡杈撳嚭娴嬭瘯缁撴灉鐨勫皝瑁呫€傝繖浜涘皝瑁呭簲褰撶敤浜庨€氳繃銆佸け璐ャ€侀€€鍑哄拰璺宠繃娑堟伅銆侰I 绯荤粺鍙互杞绘澗瑙ｆ瀽 TAP 杈撳嚭娑堟伅浠ユ娴嬫祴璇曠粨鏋溿€?
## 璐＄尞鏂版祴璇曪紙缁嗚妭锛?

 - 鍦ㄤ綘鐨?Makefile 涓紝閫氳繃鍖呭惈 lib.mk 鏉ヤ娇鐢ㄥ叾涓殑璁炬柦锛岃€屼笉鏄噸澶嶉€犺疆瀛愩€傚湪鐩稿簲鐨勮涓婃寚瀹氭爣蹇楀拰浜岃繘鍒剁敓鎴愭爣蹇楋紝渚嬪锛?
```
    CFLAGS = $(KHDR_INCLUDES)
    TEST_GEN_PROGS := close_range_test
    include ../lib.mk
```

 * 濡傛灉姝ょ被浜岃繘鍒舵垨鏂囦欢鏄湪缂栬瘧鏈熼棿鐢熸垚鐨勶紝浣跨敤 TEST_GEN_XXX銆?
   TEST_PROGS銆乀EST_GEN_PROGS 琛ㄧず瀹冩槸榛樿琚祴璇曠殑鐨勫彲鎵ц鏂囦欢銆?
   TEST_GEN_MODS_DIR 搴旂敱閭ｄ簺鍦ㄦ祴璇曞紑濮嬩箣鍓嶉渶瑕佹瀯寤烘ā鍧楃殑娴嬭瘯浣跨敤銆傝鍙橀噺灏嗗寘鍚瓨鏀炬ā鍧楃殑鐩綍鍚嶃€?
   TEST_CUSTOM_PROGS 搴旂敱闇€瑕佽嚜瀹氫箟鏋勫缓瑙勫垯骞堕樆姝娇鐢ㄩ€氱敤鏋勫缓瑙勫垯鐨勬祴璇曚娇鐢ㄣ€?
   TEST_PROGS 鐢ㄤ簬娴嬭瘯 shell 鑴氭湰銆傝纭繚 shell 鑴氭湰璁剧疆浜嗗彲鎵ц浣嶃€傚惁鍒?lib.mk 鐨?run_tests 浼氫骇鐢熻鍛娿€?
   TEST_CUSTOM_PROGS 鍜?TEST_PROGS 浼氳閫氱敤鐨?run_tests 杩愯銆?
   TEST_PROGS_EXTENDED銆乀EST_GEN_PROGS_EXTENDED 琛ㄧず瀹冩槸榛樿涓嶈娴嬭瘯鐨勫彲鎵ц鏂囦欢銆?
   TEST_FILES銆乀EST_GEN_FILES 琛ㄧず瀹冩槸娴嬭瘯鎵€浣跨敤鐨勬枃浠躲€?
   TEST_INCLUDES 涓?TEST_FILES 绫讳技锛屽畠鍒楀嚭浜嗗湪瀵煎嚭鎴栧畨瑁呮祴璇曟椂搴斿寘鍚殑鏂囦欢锛屼絾鏈変互涓嬪尯鍒細

    * 鍒板叾浠栫洰褰曚腑鏂囦欢鐨勭鍙烽摼鎺ヤ細琚繚鐣?    * 鍦ㄥ皢鏂囦欢澶嶅埗鍒拌緭鍑虹洰褰曟椂锛宼ools/testing/selftests/ 涔嬩笅鐨勮矾寰勯儴鍒嗕細琚繚鐣?
   TEST_INCLUDES 鐢ㄤ簬鍒楀嚭浣嶄簬鑷祴璇曞眰娆＄粨鏋勪腑鍏朵粬鐩綍鐨勪緷璧栭」銆?
 * 棣栧厛浣跨敤鍐呮牳婧愮爜鍜?鎴?git 浠撳簱涓殑澶存枃浠讹紝鐒跺悗鍐嶄娇鐢ㄧ郴缁熷ご鏂囦欢銆傜浉瀵逛簬鍙戣鐗堝畨瑁呭埌绯荤粺涓婄殑澶存枃浠讹紝搴斿綋浼樺厛鍏虫敞璇ュ唴鏍哥増鏈殑澶存枃浠讹紝浠ヤ究鑳藉鍙戠幇鍥炲綊銆傚湪 Makefile 涓娇鐢?KHDR_INCLUDES 鏉ュ寘鍚潵鑷唴鏍告簮鐮佺殑澶存枃浠躲€?
 * 濡傛灉鏌愪釜娴嬭瘯闇€瑕佸惎鐢ㄧ壒瀹氱殑鍐呮牳閰嶇疆閫夐」锛岃鍦ㄦ祴璇曠洰褰曚腑娣诲姞涓€涓?config 鏂囦欢鏉ュ惎鐢ㄥ畠浠€?
   渚嬪锛歵ools/testing/selftests/android/config

 * 鍦ㄦ祴璇曠洰褰曞唴鍒涘缓涓€涓?.gitignore 鏂囦欢锛屽苟灏嗘墍鏈夌敓鎴愮殑 object 鍔犲叆鍏朵腑銆?
 * 鍦?selftests/Makefile 鐨?TARGETS 涓坊鍔犳柊鐨勬祴璇曞悕锛?
    TARGETS += android

 * 鎵€鏈夋敼鍔ㄩ兘搴旈€氳繃浠ヤ笅妫€鏌ワ細

```
    kselftest-{all,install,clean,gen_tar}
    kselftest-{all,install,clean,gen_tar} O=abo_path
    kselftest-{all,install,clean,gen_tar} O=rel_path
    make -C tools/testing/selftests {all,install,clean,gen_tar}
    make -C tools/testing/selftests {all,install,clean,gen_tar} O=abs_path
    make -C tools/testing/selftests {all,install,clean,gen_tar} O=rel_path
```

## 娴嬭瘯妯″潡


kselftest 浠庣敤鎴风┖闂存祴璇曞唴鏍搞€傛湁鏃堕渶瑕佷粠鍐呮牳鍐呴儴杩涜娴嬭瘯锛屼竴绉嶆柟娉曟槸鍒涘缓涓€涓祴璇曟ā鍧椼€傛垜浠彲浠ラ€氳繃涓€涓?shell 鑴氭湰娴嬭瘯杩愯鍣ㄥ皢璇ユā鍧楁帴鍏?kselftest 妗嗘灦銆俙kselftest/module.sh` 灏辨槸涓虹畝鍖栬繖涓€杩囩▼鑰岃璁＄殑銆傚悓鏃惰繕鎻愪緵浜嗕竴涓ご鏂囦欢鏉ヨ緟鍔╃紪鍐欎笌 kselftest 閰嶅悎浣跨敤鐨勫唴鏍告ā鍧楋細

- `tools/testing/selftests/kselftest_module.h`
- `tools/testing/selftests/kselftest/module.sh`

娉ㄦ剰锛屾祴璇曟ā鍧楀簲褰撲互 TAINT_TEST 姹℃煋鍐呮牳銆傚浜庝綅浜?`tools/testing/` 鐩綍涓殑妯″潡锛屾垨浣跨敤浜嗕笂杩?`kselftest_module.h` 澶存枃浠剁殑妯″潡锛岃繖浼氳嚜鍔ㄥ彂鐢熴€傚惁鍒欙紝浣犻渶瑕佸湪妯″潡婧愮爜涓坊鍔?`MODULE_INFO(test, "Y")`銆備笉鍔犺浇妯″潡鐨勮嚜娴嬭瘯閫氬父涓嶅簲姹℃煋鍐呮牳锛屼絾鍦ㄥ姞杞戒簡闈炴祴璇曟ā鍧楃殑鎯呭喌涓嬶紝鍙互閫氳繃鍚?`/proc/sys/kernel/tainted` 鍐欏叆锛屼粠鐢ㄦ埛绌洪棿鏂藉姞 TEST_TAINT銆?
### 濡備綍浣跨敤


杩欓噷鎴戜滑灞曠ず鍒涘缓娴嬭瘯妯″潡骞跺皢鍏舵帴鍏?kselftest 鐨勫吀鍨嬫楠ゃ€傛垜浠互 lib/ 鐨?kselftests 涓轰緥銆?
1. 鍒涘缓娴嬭瘯妯″潡

2. 鍒涘缓灏嗚杩愯锛堝姞杞?鍗歌浇锛夎妯″潡鐨勬祴璇曡剼鏈?   渚嬪 `tools/testing/selftests/lib/bitmap.sh`

3. 鍚?config 鏂囦欢娣诲姞涓€琛岋紝渚嬪 `tools/testing/selftests/lib/config`

4. 鍚?makefile 娣诲姞娴嬭瘯鑴氭湰锛屼緥濡?`tools/testing/selftests/lib/Makefile`

5. 楠岃瘉鍏跺伐浣滄甯革細


   # 鍋囪浣犲凡缁忓惎鍔ㄤ簡涓€涓鍐呮牳鏍戠殑鍏ㄦ柊鏋勫缓
   cd /path/to/linux/tree
   make kselftest-merge
   make modules
   sudo make modules_install
   make TARGETS=lib kselftest

### 绀轰緥妯″潡


涓€涓渶绠€鐨勬祴璇曟ā鍧楀彲鑳藉涓嬫墍绀猴細


   // SPDX-License-Identifier: GPL-2.0+

   #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

   #include "../tools/testing/selftests/kselftest_module.h"

   KSTM_MODULE_GLOBALS();

   /*
    - 鐢ㄤ簬娴嬭瘯 foobinator 鐨勫唴鏍告ā鍧?    */

   static int __init test_function()
   {
           ...
   }

   static void __init selftest(void)
   {
           KSTM_CHECK_ZERO(do_test_case("", 0));
   }

   KSTM_MODULE_LOADERS(test_foo);
   MODULE_AUTHOR("John Developer <jd@fooman.org>");
   MODULE_LICENSE("GPL");
   MODULE_INFO(test, "Y");

### 绀轰緥娴嬭瘯鑴氭湰



    #!/bin/bash
    # SPDX-License-Identifier: GPL-2.0+
    $(dirname $0)/../kselftest/module.sh "foo" test_foo


## 娴嬭瘯妗嗘灦锛圱est Harness锛?

kselftest_harness.h 鏂囦欢鍖呭惈浜嗙敤浜庢瀯寤烘祴璇曠殑鏈夌敤杈呭姪瀹忋€傝娴嬭瘯妗嗘灦鐢ㄤ簬鐢ㄦ埛绌洪棿娴嬭瘯锛屽叧浜庡唴鏍哥┖闂存祴璇曡鍙傝涓婃枃鐨?`Test Module`_銆?
tools/testing/selftests/seccomp/seccomp_bpf.c 涓殑娴嬭瘯鍙綔涓虹ず渚嬨€?
### 绀轰緥


    :doc: example


### 杈呭姪瀹?

    :functions: TH_LOG TEST TEST_SIGNAL FIXTURE FIXTURE_DATA FIXTURE_SETUP
                FIXTURE_TEARDOWN TEST_F TEST_HARNESS_MAIN FIXTURE_VARIANT
                FIXTURE_VARIANT_ADD

### 杩愮畻绗?

    :doc: operators

    :functions: ASSERT_EQ ASSERT_NE ASSERT_LT ASSERT_LE ASSERT_GT ASSERT_GE
                ASSERT_NULL ASSERT_TRUE ASSERT_NULL ASSERT_TRUE ASSERT_FALSE
                ASSERT_STREQ ASSERT_STRNE EXPECT_EQ EXPECT_NE EXPECT_LT
                EXPECT_LE EXPECT_GT EXPECT_GE EXPECT_NULL EXPECT_TRUE
                EXPECT_FALSE EXPECT_STREQ EXPECT_STRNE
