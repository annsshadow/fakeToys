
## 浣跨敤 Clang/LLVM 鏋勫缓 Linux


鏈枃妗ｄ粙缁嶅浣曚娇鐢?Clang 鍜?LLVM 宸ュ叿鏋勫缓 Linux 鍐呮牳銆?
### 绠€浠?

Linux 鍐呮牳鍘嗘潵閮芥槸浣跨敤 GNU 宸ュ叿閾撅紙濡?GCC 鍜?binutils锛夌紪璇戠殑銆傛寔缁殑鍔姏宸茬粡浣垮緱 `Clang <https://clang.llvm.org/>`_ 鍜?`LLVM <https://llvm.org/>`_ 宸ュ叿鍙互浣滀负鍙鐨勬浛浠ｆ柟妗堜娇鐢ㄣ€傝濡?`Android <https://www.android.com/>`_銆乣ChromeOS <https://www.chromium.org/chromium-os>`_銆乣OpenMandriva <https://www.openmandriva.org/>`_ 鍜?`Chimera Linux <https://chimera-linux.org/>`_ 绛夊彂琛岀増浣跨敤 Clang 鏋勫缓鐨勫唴鏍搞€侴oogle 鍜?Meta 鐨勬暟鎹腑蹇冮泦缇や篃杩愯鐢?Clang 鏋勫缓鐨勫唴鏍搞€?
`LLVM 鏄竴缁勪互 C++ 瀵硅薄褰㈠紡瀹炵幇鐨勫伐鍏烽摼缁勪欢 <https://www.aosabook.org/en/llvm.html>`_銆侰lang 鏄?LLVM 鐨勫墠绔紝鏀寔鍐呮牳鎵€闇€鐨?C 璇█浠ュ強 GNU C 鎵╁睍锛屽叾鍙戦煶涓?"klang"锛岃€屼笉鏄?"see-lang"銆?
### 浣跨敤 LLVM 鏋勫缓


```
make LLVM=1
```
```
make LLVM=1 ARCH=arm64
```
LLVM 鎻愪緵浜?GNU binutils 宸ュ叿鐨勬浛浠ｅ搧銆傚彲浠ラ€氳繃浠ヤ笅鏂瑰紡鍚敤瀹冧滑锛?```
make CC=clang LD=ld.lld AR=llvm-ar NM=llvm-nm STRIP=llvm-strip \
  OBJCOPY=llvm-objcopy OBJDUMP=llvm-objdump READELF=llvm-readelf \
  HOSTCC=clang HOSTCXX=clang++ HOSTAR=llvm-ar HOSTLD=ld.lld
```
`LLVM=1` 鍗冲睍寮€涓轰笂杩板懡浠ゃ€?
濡傛灉浣犵殑 LLVM 宸ュ叿涓嶅湪 PATH 涓紝鍙互鎻愪緵瀹冧滑鐨勮矾寰勶細
```
make LLVM=/path/to/llvm/
```
杩欏皢浣跨敤 `/path/to/llvm/clang`銆乣/path/to/llvm/ld.lld` 绛夈€備互涓嬫柟寮忥細
```
PATH=/path/to/llvm:$PATH make LLVM=1
```
濡傛灉浣犵殑 LLVM 宸ュ叿甯︽湁鐗堟湰鍚庣紑锛屽苟涓旀兂浣跨敤璇ユ樉寮忕増鏈紝鑰屼笉鏄儚 `LLVM=1` 閭ｆ牱浣跨敤涓嶅甫鍚庣紑鐨勫彲鎵ц鏂囦欢锛屽彲浠ワ細
```
make LLVM=-14
```
杩欏皢浣跨敤 `clang-14`銆乣ld.lld-14` 绛夈€?
涓轰簡鏀寔鏍戝璺緞涓庣増鏈悗缂€鐨勭粍鍚堬紝鍙互锛?```
PATH=/path/to/llvm/:$PATH make LLVM=-14
```
濡傛灉浣跨敤鐩镐簰鐙珛鐨勫懡浠よ繘琛岄厤缃拰鏋勫缓锛岄偅涔堟瘡娆¤皟鐢?`make` 鏃堕兘搴旇缃笌 `LLVM=` 鐩稿悓鐨勫€笺€傚湪杩愯鏈€缁堜細鎵ц `make` 鐨勮剼鏈椂锛宍LLVM=` 涔熷簲浣滀负鐜鍙橀噺杩涜璁剧疆銆?
### 浜ゅ弶缂栬瘧


鍗曚竴鐨?Clang 缂栬瘧鍣ㄤ簩杩涘埗鏂囦欢锛堜互鍙婄浉搴旂殑 LLVM 宸ュ叿锛夐€氬父鍖呭惈鎵€鏈夊彈鏀寔鐨勫悗绔紝杩欐湁鍔╀簬绠€鍖栦氦鍙夌紪璇戯紝灏ゅ叾鏄湪浣跨敤 `LLVM=1` 鏃躲€傚鏋滃彧浣跨敤 LLVM 宸ュ叿锛屽彲浠ワ細
```
make LLVM=1 ARCH=arm64
```
涓嬮潰鏄竴涓贩鍚堜娇鐢?LLVM 涓?GNU 宸ュ叿鐨勪緥瀛愶紝瀵逛簬鍍?`ARCH=s390` 杩欐牱灏氫笉鏀寔 `ld.lld` 鎴?`llvm-objcopy` 鐨勭洰鏍囷紝鍙互锛?```
make LLVM=1 ARCH=s390 LD=s390x-linux-gnu-ld.bfd \
  OBJCOPY=s390x-linux-gnu-objcopy
```
璇ョず渚嬪皢璋冪敤 `s390x-linux-gnu-ld.bfd` 浣滀负閾炬帴鍣ㄤ互鍙?`s390x-linux-gnu-objcopy`锛屽洜姝よ纭繚瀹冧滑鍙湪浣犵殑 `$PATH` 涓壘鍒般€?
`CROSS_COMPILE` 涓嶄細鍍忔湭璁剧疆 `LLVM=1` 鏃朵负 GNU 宸ュ叿鎵€鍋氱殑閭ｆ牱锛屼綔涓?Clang 缂栬瘧鍣ㄤ簩杩涘埗鏂囦欢锛堟垨鐩稿簲鐨?LLVM 宸ュ叿锛夌殑鍓嶇紑銆?
### LLVM_IAS= 鍙傛暟


Clang 鑳藉姹囩紪姹囩紪浠ｇ爜銆備綘鍙互浼犲叆 `LLVM_IAS=0` 鏉ョ鐢ㄨ琛屼负锛岃 Clang 璋冪敤瀵瑰簲鐨勯潪闆嗘垚姹囩紪鍣細
```
make LLVM=1 LLVM_IAS=0
```
鍦ㄤ氦鍙夌紪璇戝苟浣跨敤 `LLVM_IAS=0` 鏃讹紝蹇呴』浣跨敤 `CROSS_COMPILE` 鏉ヤ负缂栬瘧鍣ㄨ缃?`--prefix=`锛屼互渚挎壘鍒板搴旂殑闈為泦鎴愭眹缂栧櫒锛堥€氬父浣犲苟涓嶆兂浣跨敤锛?```
make LLVM=1 ARCH=arm LLVM_IAS=0 CROSS_COMPILE=arm-linux-gnueabi-

```

### Ccache


`ccache` 鍙互涓?`clang` 閰嶅悎浣跨敤浠ユ敼鍠勫悗缁瀯寤猴紙涓嶈繃鍦ㄥ娆℃瀯寤轰箣闂达紝KBUILD_BUILD_TIMESTAMP_ 搴旇缃负纭畾鍊硷級锛?```
KBUILD_BUILD_TIMESTAMP='' make LLVM=1 CC="ccache clang"
```

### 鍙楁敮鎸佺殑鏋舵瀯


LLVM 骞舵湭浠?Linux 鏀寔鐨勬墍鏈夋灦鏋勪负鐩爣锛屼粎浠呭洜涓烘煇涓洰鏍囧湪 LLVM 涓彈鏀寔锛屼篃骞朵笉鎰忓懗鐫€鍐呮牳鑳藉姣棤闂鍦版瀯寤烘垨杩愯銆備笅闈㈡槸鐩墠鍙互浣跨敤 `CC=clang` 鎴?`LLVM=1` 姝ｅ父宸ヤ綔鐨勬灦鏋勭殑鎬讳綋姒傝堪銆傛敮鎸佺骇鍒搴斾簬 MAINTAINERS 鏂囦欢涓殑 "S" 鍊笺€傚鏋滄煇涓灦鏋勬湭鍒楀嚭锛屽垯鎰忓懗鐫€ LLVM 骞舵湭浠ュ叾涓虹洰鏍囷紝鎴栬€呭瓨鍦ㄥ凡鐭ラ棶棰樸€備娇鐢?LLVM 鐨勬渶鏂扮ǔ瀹氱増鏈敋鑷冲紑鍙戝垎鏀€氬父鑳借幏寰楁渶浣崇粨鏋溿€傛煇涓灦鏋勭殑 `defconfig` 閫氬父棰勬湡鑳借壇濂藉伐浣滐紝浣嗘煇浜涢厤缃彲鑳戒粛瀛樺湪灏氭湭鍙戠幇鐨勯棶棰樸€傛杩庡湪涓嬮潰鐨?issue 杩借釜鍣ㄤ腑鎻愪氦 bug 鎶ュ憡锛?
   :widths: 10 10 10
   :header-rows: 1

   - - 鏋舵瀯
     - 鏀寔绾у埆
     - `make` 鍛戒护
   - - arm
     - 鍙楁敮鎸?     - `LLVM=1`
   - - arm64
     - 鍙楁敮鎸?     - `LLVM=1`
   - - hexagon
     - 缁存姢涓?     - `LLVM=1`
   - - loongarch
     - 缁存姢涓?     - `LLVM=1`
   - - mips
     - 缁存姢涓?     - `LLVM=1`
   - - powerpc
     - 缁存姢涓?     - `LLVM=1`
   - - riscv
     - 鍙楁敮鎸?     - `LLVM=1`
   - - s390
     - 缁存姢涓?     - `LLVM=1` (LLVM >= 18.1.0), `CC=clang` (LLVM < 18.1.0)
   - - sparc (sparc64 only)
     - 缁存姢涓?     - `CC=clang LLVM_IAS=0` (LLVM >= 20)
   - - um (User Mode)
     - 缁存姢涓?     - `LLVM=1`
   - - x86
     - 鍙楁敮鎸?     - `LLVM=1`

### 鑾峰彇甯姪


- `缃戠珯 <https://clangbuiltlinux.github.io/>`_
- `閭欢鍒楄〃 <https://lore.kernel.org/llvm/>`_: <llvm@lists.linux.dev>
- `鏃ч偖浠跺垪琛ㄥ綊妗?<https://groups.google.com/g/clang-built-linux>`_
- `Issue 杩借釜鍣?<https://github.com/ClangBuiltLinux/linux/issues>`_
- IRC: #clangbuiltlinux on irc.libera.chat
- `Telegram <https://t.me/ClangBuiltLinux>`_: @ClangBuiltLinux
- `缁村熀 <https://github.com/ClangBuiltLinux/linux/wiki>`_
- `鏂版墜 Bug <https://github.com/ClangBuiltLinux/linux/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22>`_

### 鑾峰彇 LLVM


鎴戜滑鍦?`kernel.org <https://kernel.org/pub/tools/llvm/>`_ 涓婃彁渚涢鏋勫缓鐨勭ǔ瀹氱増鏈?LLVM銆傝繖浜涚増鏈凡浣跨敤 profile 鏁版嵁杩涜浼樺寲浠ユ瀯寤?Linux 鍐呮牳锛岀浉姣斿叾浠?LLVM 鍙戣鐗堝簲鑳芥敼鍠勫唴鏍告瀯寤烘椂闂淬€?
涓嬮潰鍒楀嚭鐨勪竴浜涢摼鎺ュ彲鑳芥湁鍔╀簬浠庢簮鐮佹瀯寤?LLVM锛屾垨閫氳繃鍙戣鐗堢殑鍖呯鐞嗗櫒鑾峰彇 LLVM銆?
- https://releases.llvm.org/download.html
- https://github.com/llvm/llvm-project
- https://llvm.org/docs/GettingStarted.html
- https://llvm.org/docs/CMake.html
- https://apt.llvm.org/
- https://www.archlinux.org/packages/extra/x86_64/llvm/
- https://github.com/ClangBuiltLinux/tc-build
- https://github.com/ClangBuiltLinux/linux/wiki/Building-Clang-from-source
- https://android.googlesource.com/platform/prebuilts/clang/host/linux-x86/
