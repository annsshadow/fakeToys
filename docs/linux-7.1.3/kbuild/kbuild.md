## Kbuild


## 杈撳嚭鏂囦欢


### modules.order

璇ユ枃浠惰褰曚簡妯″潡鍦?Makefile 涓嚭鐜扮殑椤哄簭銆俶odprobe 鍒╃敤瀹冩潵纭畾鎬у湴瑙ｆ瀽涓庡涓ā鍧楀尮閰嶇殑鍒悕銆?
### modules.builtin

璇ユ枃浠跺垪鍑轰簡鎵€鏈夊唴寤哄埌鍐呮牳涓殑妯″潡銆俶odprobe 鍊熸鍦ㄥ皾璇曞姞杞芥煇涓唴寤烘ā鍧楁椂涓嶄細澶辫触銆?
### modules.builtin.modinfo

璇ユ枃浠跺寘鍚唴寤哄埌鍐呮牳涓殑鎵€鏈夋ā鍧楃殑 modinfo銆備笌鐙珛妯″潡鐨?modinfo 涓嶅悓锛屾墍鏈夊瓧娈甸兘浠ユā鍧楀悕浣滀负鍓嶇紑銆?
### modules.builtin.ranges

璇ユ枃浠跺寘鍚唴寤哄埌鍐呮牳涓殑鎵€鏈夋ā鍧楋紙鎸?ELF 娈碉級鐨勫湴鍧€鍋忕Щ鑼冨洿銆傜粨鍚?System.map锛屽彲鐢ㄤ簬灏嗘ā鍧楀悕涓庣鍙峰叧鑱旇捣鏉ャ€?
## 鐜鍙橀噺


### KCPPFLAGS

浼犻€掔粰棰勫鐞嗛樁娈电殑棰濆閫夐」銆傝繖浜涢澶勭悊閫夐」浼氬湪 kbuild 杩涜棰勫鐞嗙殑鎵€鏈夊満鏅腑浣跨敤锛屽寘鎷紪璇?C 鏂囦欢鍜屾眹缂栨枃浠躲€?
### KAFLAGS

浼犻€掔粰姹囩紪鍣ㄧ殑棰濆閫夐」锛堢敤浜庡唴寤轰唬鐮佸拰妯″潡锛夈€?
### AFLAGS_MODULE

妯″潡涓撶敤鐨勯澶栨眹缂栧櫒閫夐」銆?
### AFLAGS_KERNEL

鍐呭缓浠ｇ爜涓撶敤鐨勯澶栨眹缂栧櫒閫夐」銆?
### KCFLAGS

浼犻€掔粰 C 缂栬瘧鍣ㄧ殑棰濆閫夐」锛堢敤浜庡唴寤轰唬鐮佸拰妯″潡锛夈€?
### KRUSTFLAGS

浼犻€掔粰 Rust 缂栬瘧鍣ㄧ殑棰濆閫夐」锛堢敤浜庡唴寤轰唬鐮佸拰妯″潡锛夈€?
### CFLAGS_KERNEL

褰撲娇鐢?$(CC) 缂栬瘧浣滀负鍐呭缓鐨勪唬鐮佹椂锛屼负鍏舵彁渚涚殑棰濆閫夐」銆?
### CFLAGS_MODULE

浣跨敤 $(CC) 鏃堕噰鐢ㄧ殑銆佹ā鍧椾笓鐢ㄧ殑棰濆閫夐」銆?
### RUSTFLAGS_KERNEL

褰撲娇鐢?$(RUSTC) 缂栬瘧浣滀负鍐呭缓鐨勪唬鐮佹椂锛屼负鍏舵彁渚涚殑棰濆閫夐」銆?
### RUSTFLAGS_MODULE

浣跨敤 $(RUSTC) 鏃堕噰鐢ㄧ殑銆佹ā鍧椾笓鐢ㄧ殑棰濆閫夐」銆?
### LDFLAGS_MODULE

浣跨敤 $(LD) 閾炬帴妯″潡鏃朵娇鐢ㄧ殑棰濆閫夐」銆?
### HOSTCFLAGS

鏋勫缓瀹夸富绋嬪簭鏃朵紶閫掔粰 $(HOSTCC) 鐨勯澶栨爣蹇椼€?
### HOSTCXXFLAGS

鏋勫缓瀹夸富绋嬪簭鏃朵紶閫掔粰 $(HOSTCXX) 鐨勯澶栨爣蹇椼€?
### HOSTRUSTFLAGS

鏋勫缓瀹夸富绋嬪簭鏃朵紶閫掔粰 $(HOSTRUSTC) 鐨勯澶栨爣蹇椼€?
### PROCMACROLDFLAGS

閾炬帴 Rust 杩囩▼瀹忔椂浼犻€掔殑鏍囧織銆傜敱浜庤繃绋嬪畯鍦ㄦ瀯寤烘椂鐢?rustc 鍔犺浇锛屽畠浠繀椤讳互涓庢墍鐢?rustc 宸ュ叿閾惧吋瀹圭殑鏂瑰紡閾炬帴銆?
渚嬪锛屽綋 rustc 浣跨敤鐨?C 搴撲笌鐢ㄦ埛甯屾湜鐢ㄤ簬瀹夸富绋嬪簭鐨?C 搴撲笉鍚屾椂锛岃繖浼氬緢鏈夌敤銆?
鑻ユ湭璁剧疆锛屽垯榛樿浣跨敤閾炬帴瀹夸富绋嬪簭鏃朵紶閫掔殑鏍囧織銆?
### HOSTLDFLAGS

閾炬帴瀹夸富绋嬪簭鏃朵紶閫掔殑棰濆鏍囧織銆?
### HOSTLDLIBS

鏋勫缓瀹夸富绋嬪簭鏃堕渶閾炬帴鐨勯澶栧簱銆?

### USERCFLAGS

缂栬瘧 userprogs 鏃剁敤浜?$(CC) 鐨勯澶栭€夐」銆?
### USERLDFLAGS

閾炬帴 userprogs 鏃剁敤浜?$(LD) 鐨勯澶栭€夐」銆倁serprogs 浣跨敤 CC 杩涜閾炬帴锛屽洜姝?$(USERLDFLAGS) 搴斿寘鍚€傜敤鐨?"-Wl," 鍓嶇紑銆?
### KBUILD_KCONFIG

灏嗚鐜鍙橀噺鐨勫€艰涓洪《灞?Kconfig 鏂囦欢銆傞粯璁ゅ悕绉颁负 "Kconfig"銆?
### KBUILD_VERBOSE

璁剧疆 kbuild 鐨勮缁嗙▼搴︺€傚彲璧嬩簣涓?"V=..." 鐩稿悓鐨勫€笺€?
瀹屾暣鍒楄〃鍙傝 make help銆?
璁剧疆 "V=..." 鐨勪紭鍏堢骇楂樹簬 KBUILD_VERBOSE銆?
### KBUILD_EXTMOD

璁剧疆鏋勫缓澶栭儴妯″潡鏃舵煡鎵惧唴鏍告簮鐮佺殑鐩綍銆?
璁剧疆 "M=..." 鐨勪紭鍏堢骇楂樹簬 KBUILD_EXTMOD銆?
### KBUILD_OUTPUT

鏋勫缓鍐呮牳鏃舵寚瀹氳緭鍑虹洰褰曘€?
璇ュ彉閲忎篃鍙敤浜庡湪鐙珛鏋勫缓鐩綍涓拡瀵归鏋勫缓鐨勫唴鏍告瀯寤哄閮ㄦā鍧楁椂锛屾寚鍚戝唴鏍歌緭鍑虹洰褰曘€傝娉ㄦ剰锛岃繖骞朵笉鎸囧畾澶栭儴妯″潡鑷韩鐨勮緭鍑虹洰褰曪紙涓烘璇蜂娇鐢?KBUILD_EXTMOD_OUTPUT锛夈€?
杈撳嚭鐩綍涔熷彲浣跨敤 "O=..." 鎸囧畾銆?
璁剧疆 "O=..." 鐨勪紭鍏堢骇楂樹簬 KBUILD_OUTPUT銆?
### KBUILD_EXTMOD_OUTPUT

鎸囧畾澶栭儴妯″潡鐨勮緭鍑虹洰褰曘€?
璁剧疆 "MO=..." 鐨勪紭鍏堢骇楂樹簬 KBUILD_EXTMOD_OUTPUT銆?
### KBUILD_EXTRA_WARN

鎸囧畾棰濆鐨勬瀯寤烘鏌ャ€傞€氳繃鍛戒护琛屼紶閫?W=... 鍙祴浜堢浉鍚岀殑鍊笺€?
鏀寔鐨勫€煎垪琛ㄥ弬瑙?`make help`銆?
璁剧疆 "W=..." 鐨勪紭鍏堢骇楂樹簬 KBUILD_EXTRA_WARN銆?
### KBUILD_DEBARCH

瀵逛簬 deb-pkg 鐩爣锛屽厑璁歌鐩?deb-pkg 鎵€閲囩敤鐨勫父瑙勫惎鍙戝紡鍒ゆ柇銆傞€氬父 deb-pkg 浼氬熀浜?UTS_MACHINE 鍙橀噺锛屽湪鏌愪簺鏋舵瀯涓婅繕浼氬熀浜庡唴鏍搁厤缃紝鏉ョ寽娴嬫纭殑鏋舵瀯銆侹BUILD_DEBARCH 鐨勫€艰鍋囧畾锛堣€岄潪妫€鏌ワ級涓轰竴涓湁鏁堢殑 Debian 鏋舵瀯銆?
### KDOCFLAGS

涓烘瀯寤鸿繃绋嬩腑鐨?kernel-doc 妫€鏌ユ寚瀹氶澶栫殑锛堣鍛?閿欒锛夋爣蹇楋紝鏀寔鍝簺鏍囧織鍙傝 tools/docs/kernel-doc銆傛敞鎰忚繖锛堢洰鍓嶏級涓嶉€傜敤浜庢枃妗ｆ瀯寤恒€?
### ARCH

灏?ARCH 璁句负瑕佹瀯寤虹殑鏋舵瀯銆?
鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝鏋舵瀯鍚嶇О涓?arch/ 鐩綍涓殑鐩綍鍚嶇浉鍚屻€?
浣嗘煇浜涙灦鏋勶紙濡?x86 鍜?sparc锛夋湁鍒悕銆?
- x86锛?2 浣嶄负 i386锛?4 浣嶄负 x86_64
- parisc锛?4 浣嶄负 parisc64
- sparc锛?2 浣嶄负 sparc32锛?4 浣嶄负 sparc64

### CROSS_COMPILE

鎸囧畾 binutils 鏂囦欢鍚嶇殑鍥哄畾閮ㄥ垎锛堝彲閫夛級銆侰ROSS_COMPILE 鍙互鏄枃浠跺悕鐨勪竴閮ㄥ垎锛屼篃鍙互鏄畬鏁磋矾寰勩€?
鍦ㄦ煇浜涢厤缃腑锛孋ROSS_COMPILE 涔熺敤浜?ccache銆?
### CF

sparse 鐨勯澶栭€夐」銆?
```

    make CF=-Wbitwise C=2

```
### INSTALL_PATH

INSTALL_PATH 鎸囧畾鏀剧疆鏇存柊鍚庣殑鍐呮牳涓庣郴缁熸槧灏勯暅鍍忕殑浣嶇疆銆傞粯璁や负 /boot锛屼絾涔熷彲璁句负鍏朵粬鍊笺€?
### INSTALLKERNEL

浣跨敤 "make install" 鏃惰皟鐢ㄧ殑瀹夎鑴氭湰銆傞粯璁ゅ悕绉颁负 "installkernel"銆?
璇ヨ剼鏈皢浣跨敤浠ヤ笅鍙傛暟璋冪敤锛?
   - $1 - 鍐呮牳鐗堟湰
   - $2 - 鍐呮牳闀滃儚鏂囦欢
   - $3 - 鍐呮牳鏄犲皠鏂囦欢
   - $4 - 榛樿瀹夎璺緞锛堣嫢涓虹┖鍒欎娇鐢ㄦ牴鐩綍锛?
"make install" 鐨勫疄鐜版槸鏋舵瀯鐩稿叧鐨勶紝鍙兘涓庝笂杩颁笉鍚屻€?
鎻愪緵 INSTALLKERNEL 鏄负浜嗚兘澶熷湪浜ゅ弶缂栬瘧鍐呮牳鏃舵寚瀹氳嚜瀹氫箟瀹夎鍣ㄣ€?
### MODLIB

鎸囧畾妯″潡鐨勫畨瑁呬綅缃€?```

     $(INSTALL_MOD_PATH)/lib/modules/$(KERNELRELEASE)

```
璇ュ€煎彲琚鐩栵紝姝ゆ椂榛樿鍊艰蹇界暐銆?
### INSTALL_MOD_PATH

INSTALL_MOD_PATH 涓?MODLIB 鎸囧畾涓€涓墠缂€锛岀敤浜?build root 鎵€闇€鐨勬ā鍧楃洰褰曢噸瀹氫綅銆傚畠鍦?makefile 涓湭瀹氫箟锛屼絾濡傛湁闇€瑕佸彲灏嗘鍙傛暟浼犻€掔粰 make銆?
### INSTALL_MOD_STRIP

鑻ュ畾涔変簡 INSTALL_MOD_STRIP锛屼細瀵艰嚧妯″潡鍦ㄥ畨瑁呭悗琚?strip銆傝嫢 INSTALL_MOD_STRIP 涓?'1'锛屽垯浣跨敤榛樿閫夐」 --strip-debug銆傚惁鍒欙紝INSTALL_MOD_STRIP 鐨勫€煎皢浣滀负浼犻€掔粰 strip 鍛戒护鐨勯€夐」銆?
### INSTALL_HDR_PATH

INSTALL_HDR_PATH 鎸囧畾鎵ц "make headers_*" 鏃剁敤鎴风┖闂村ご鏂囦欢鐨勫畨瑁呬綅缃€?
```

    $(objtree)/usr

```
$(objtree) 鏄繚瀛樿緭鍑烘枃浠剁殑鐩綍銆?
杈撳嚭鐩綍閫氬父閫氳繃鍛戒护琛屼笂鐨?"O=..." 璁剧疆銆?
璇ュ€煎彲琚鐩栵紝姝ゆ椂榛樿鍊艰蹇界暐銆?
### INSTALL_DTBS_PATH

INSTALL_DTBS_PATH 鎸囧畾璁惧鏍?blob 鐨勫畨瑁呬綅缃紝鐢ㄤ簬 build root 鎵€闇€鐨勯噸瀹氫綅銆傚畠鍦?makefile 涓湭瀹氫箟锛屼絾濡傛湁闇€瑕佸彲灏嗘鍙傛暟浼犻€掔粰 make銆?
### KBUILD_ABS_SRCTREE

鍦ㄥ彲鑳界殑鎯呭喌涓嬶紝Kbuild 浣跨敤鐩稿璺緞鏉ユ寚鍚戞簮鐮佹爲銆備緥濡傦紝鍦ㄦ簮鐮佹爲涓瀯寤烘椂锛屾簮鐮佹爲璺緞涓?'.'銆?
璁剧疆姝ゆ爣蹇椾細瑕佹眰 Kbuild 浣跨敤婧愮爜鏍戠殑缁濆璺緞銆傝繖鍦ㄦ煇浜涘満鏅笅寰堟湁鐢紝渚嬪鐢熸垚甯︽湁缁濆璺緞鏉＄洰鐨?tag 鏂囦欢绛夈€?
### KBUILD_SIGN_PIN

褰撳鍐呮牳妯″潡绛惧悕涓旂閽ラ渶瑕佸彛浠ゆ垨 PIN 鏃讹紝璇ュ彉閲忓厑璁稿皢鍙ｄ护鎴?PIN 浼犻€掔粰 sign-file 宸ュ叿銆?
### KBUILD_MODPOST_WARN

KBUILD_MODPOST_WARN 鍙缃负閬垮厤鍦ㄦ渶缁堟ā鍧楅摼鎺ラ樁娈靛嚭鐜版湭瀹氫箟绗﹀彿鏃舵姤閿欍€傚畠浼氬皢杩欎簺閿欒杞负璀﹀憡銆?
### KBUILD_MODPOST_NOFINAL

KBUILD_MODPOST_NOFINAL 鍙缃负璺宠繃妯″潡鐨勬渶缁堥摼鎺ャ€傝繖浠呯敤浜庡姞閫熸祴璇曠紪璇戙€?
### KBUILD_EXTRA_SYMBOLS

鐢ㄤ簬浣跨敤鏉ヨ嚜鍏朵粬妯″潡鐨勭鍙风殑妯″潡銆傛洿澶氱粏鑺傚弬瑙?modules.rst銆?
### ALLSOURCE_ARCHS

瀵逛簬 tags/TAGS/cscope 鐩爣锛屽彲浠ユ寚瀹氬涓灦鏋?```

    $ make ALLSOURCE_ARCHS="x86 mips arm" tags

```
```

    $ make ALLSOURCE_ARCHS=all tags

```
### IGNORE_DIRS

瀵逛簬 tags/TAGS/cscope 鐩爣锛屽彲浠ラ€夋嫨鎺掗櫎鍝簺鐩綍
```

    $ make IGNORE_DIRS="drivers/gpu/drm/radeon tools" cscope

```
### KBUILD_BUILD_TIMESTAMP

灏嗗叾璁剧疆涓烘棩鏈熷瓧绗︿覆锛屼細瑕嗙洊 UTS_VERSION 瀹氫箟涓娇鐢ㄧ殑锛堣繍琛屽唴鏍镐腑 uname -v 鎵€鍦ㄧ殑锛夋椂闂存埑銆傝鍊煎繀椤?```

    $ KBUILD_BUILD_TIMESTAMP="Mon Oct 13 00:00:00 UTC 2025" make

```
榛樿鍊间负鏋勫缓杩囩▼涓煇涓椂鍒?date 鍛戒护鐨勮緭鍑恒€傚鏋滄彁渚涗簡璇ユ椂闂存埑锛屽畠涔熷皢鐢ㄤ簬浠讳綍 initramfs 褰掓。涓殑 mtime 瀛楁銆侷nitramfs 鐨?mtime 鏄?32 浣嶇殑锛屽洜姝?1970 骞?Unix 绾厓涔嬪墠鎴?2106-02-07 06:28:15 UTC 涔嬪悗鐨勬棩鏈熶細澶辫触銆?
### KBUILD_BUILD_USER, KBUILD_BUILD_HOST

杩欎袱涓彉閲忓厑璁歌鐩栧惎鍔ㄦ湡闂村拰 /proc/version 涓樉绀虹殑 user@host 瀛楃涓层€傞粯璁ゅ€煎垎鍒负 whoami 鍜?host 鍛戒护鐨勮緭鍑恒€?
### LLVM

濡傛灉灏嗚鍙橀噺璁句负 1锛孠build 灏嗕娇鐢?Clang 鍜?LLVM 宸ュ叿閾撅紙鑰岄潪 GCC 涓?GNU binutils锛夋潵鏋勫缓鍐呮牳銆?