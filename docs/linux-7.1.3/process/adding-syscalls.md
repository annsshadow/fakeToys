

## 娣诲姞鏂扮殑绯荤粺璋冪敤


鏈枃妗ｆ弿杩颁簡鍚?Linux 鍐呮牳娣诲姞涓€涓柊鐨勭郴缁熻皟鐢ㄦ墍娑夊強鐨勫悇椤瑰伐浣滐紝瓒呭嚭浜?Documentation/process/submitting-patches.rst <submittingpatches> 涓父瑙勭殑鎻愪氦寤鸿鑼冨洿銆?

### 绯荤粺璋冪敤鐨勬浛浠ｆ柟妗?

娣诲姞鏂扮郴缁熻皟鐢ㄦ椂瑕佽€冭檻鐨勭涓€浠朵簨锛屾槸鏌愪釜鏇夸唬鏂规鏄惁鍙兘鏇村悎閫傘€傚敖绠＄郴缁熻皟鐢ㄦ槸鐢ㄦ埛绌洪棿涓庡唴鏍镐箣闂存渶浼犵粺銆佹渶鏄庢樉鐨勪氦浜掔偣锛屼絾杩樻湁鍏朵粬鍙兘鈥斺€旇閫夋嫨鏈€閫傚悎浣犵殑鎺ュ彛鐨勬柟妗堛€?
 - 濡傛灉鎵€娑夊強鐨勬搷浣滅敤璧锋潵鍙互鍍忎竴涓被鏂囦欢绯荤粺鐨勫璞★紝閭ｄ箞鍒涘缓涓€涓柊鏂囦欢绯荤粺鎴栬澶囧彲鑳芥洿鏈夋剰涔夈€傝繖涔熸洿瀹规槗鎶婃柊鍔熻兘灏佽杩涗竴涓唴鏍告ā鍧楋紝鑰屼笉蹇呰姹傚皢鍏剁紪鍏ヤ富鍐呮牳銆?
     - 濡傛灉鏂板姛鑳芥秹鍙婂唴鏍搁€氱煡鐢ㄦ埛绌洪棿鏌愪欢浜嬪凡缁忓彂鐢熺殑鎿嶄綔锛岄偅涔堜负鐩稿叧瀵硅薄杩斿洖涓€涓柊鐨勬枃浠舵弿杩扮锛屽彲璁╃敤鎴风┖闂翠娇鐢?`poll`/`select`/`epoll` 鏉ユ帴鏀惰閫氱煡銆?     - 涓嶈繃锛屾棤娉曟槧灏勫埌 `read(2)`/`write(2)` 杩欑被鎿嶄綔鐨勬搷浣滅敤 `ioctl(2)` 璇锋眰鏉ュ疄鐜帮紝杩欏彲鑳藉鑷翠竴涓笉澶€忔槑鐨?API銆?
 - 濡傛灉浣犲彧鏄鏆撮湶杩愯鏃剁郴缁熶俊鎭紝鍦?sysfs 涓柊寤轰竴涓妭鐐癸紙鍙傝 `Documentation/filesystems/sysfs.rst`锛夋垨 `/proc` 鏂囦欢绯荤粺鍙兘鏇村悎閫傘€備笉杩囷紝浣跨敤杩欎簺鏈哄埗瑕佹眰鐩稿叧鏂囦欢绯荤粺宸茶鎸傝浇锛岃€岃繖骞朵笉鎬绘槸鎴愮珛锛堜緥濡傚湪鍛藉悕绌洪棿鍖?娌欑鍖?chroot 鐨勭幆澧冧腑锛夈€傞伩鍏嶅悜 debugfs 娣诲姞浠讳綍 API锛屽洜涓哄畠涓嶈瑙嗕负闈㈠悜鐢ㄦ埛绌洪棿鐨勨€滅敓浜р€濇帴鍙ｃ€? - 濡傛灉鎿嶄綔鐗瑰畾浜庢煇涓枃浠舵垨鏂囦欢鎻忚堪绗︼紝閭ｄ箞澧炲姞涓€涓柊鐨?`fcntl(2)` 鍛戒护閫夐」鍙兘鏇村悎閫傘€備笉杩囷紝`fcntl(2)` 鏄竴涓殣钘忎簡澶ч噺澶嶆潅鎬х殑澶氳矾澶嶇敤绯荤粺璋冪敤锛屽洜姝よ閫夐」鏈€閫傚悎鏂板姛鑳戒笌鐜版湁 `fcntl(2)` 鍔熻兘闈炲父鐩镐技锛屾垨鑰呮柊鍔熻兘闈炲父绠€鍗曠殑鎯呭喌锛堜緥濡傝幏鍙?璁剧疆涓€涓笌鏂囦欢鎻忚堪绗︾浉鍏崇殑绠€鍗曟爣蹇楋級銆? - 濡傛灉鎿嶄綔鐗瑰畾浜庢煇涓换鍔℃垨杩涚▼锛岄偅涔堝鍔犱竴涓柊鐨?`prctl(2)` 鍛戒护閫夐」鍙兘鏇村悎閫傘€備笌 `fcntl(2)` 涓€鏍凤紝杩欎釜绯荤粺璋冪敤鏄竴涓鏉傜殑澶氳矾澶嶇敤鍣紝鍥犳鏈€濂界暀缁欎笌鐜版湁 `prctl()` 鍛戒护杩戜箮绛変环鐨勬儏鍐碉紝鎴栬€呰幏鍙?璁剧疆涓庤繘绋嬬浉鍏崇殑绠€鍗曟爣蹇椼€?

### 璁捐 API锛氫负鎵╁睍鍋氳鍒?

鏂扮殑绯荤粺璋冪敤鏋勬垚鍐呮牳 API 鐨勪竴閮ㄥ垎锛屽苟涓斿繀椤绘棤闄愭湡鍦板緱鍒版敮鎸併€傚洜姝わ紝鍦ㄥ唴鏍搁偖浠跺垪琛ㄤ笂鏄庣‘璁ㄨ璇ユ帴鍙ｆ槸涓潪甯稿ソ鐨勫仛娉曪紝鑰屼负鎺ュ彛鐨勬湭鏉ユ墿灞曞仛瑙勫垝涔熷緢閲嶈銆?
锛堢郴缁熻皟鐢ㄨ〃涓埌澶勯兘鏄病鏈夎繖鏍峰仛鐨勫巻鍙蹭緥瀛愶紝浠ュ強鐩稿簲鐨勫悗缁郴缁熻皟鐢ㄢ€斺€擿eventfd`/`eventfd2`銆乣dup2`/`dup3`銆乣inotify_init`/`inotify_init1`銆乣pipe`/`pipe2`銆乣renameat`/`renameat2`鈥斺€旀墍浠ヨ鍚稿彇鍐呮牳鐨勫巻鍙叉暀璁紝浠庝竴寮€濮嬪氨涓烘墿灞曞仛瑙勫垝銆傦級

瀵逛簬鍙帴鍙楀皯鏁板嚑涓弬鏁扮殑杈冪畝鍗曠郴缁熻皟鐢紝鍏佽鏈潵鎵╁睍鎬х殑棣栭€夋柟寮忔槸缁欑郴缁熻皟鐢ㄥ鍔犱竴涓?flags 鍙傛暟銆備负浜嗙‘淇濈敤鎴风┖闂寸▼搴忚兘澶熷湪涓嶅悓鍐呮牳鐗堟湰涔嬮棿瀹夊叏鍦颁娇鐢?flags锛岄渶瑕佹鏌?flags 鍊间腑鏄惁鍚湁浠讳綍鏈煡鐨?
```

    if (flags & ~(THING_FLAG1 | THING_FLAG2 | THING_FLAG3))
        return -EINVAL;

```
锛堝鏋滃皻鏈娇鐢ㄤ换浣?flags 鍊硷紝鍒欐鏌?flags 鍙傛暟鏄惁涓洪浂銆傦級

瀵逛簬娑夊強杈冨鍙傛暟鐨勬洿澶嶆潅鐨勭郴缁熻皟鐢紝棣栭€夋柟寮忔槸灏嗗ぇ閮ㄥ垎鍙傛暟灏佽杩涗竴涓€氳繃鎸囬拡浼犲叆鐨勭粨鏋勪綋涓€傝繖鏍风殑缁撴瀯浣撳彲浠ュ簲瀵规湭鏉ョ殑鎵╁睍

```

    struct xyzzy_params {
        u32 size; /* userspace sets p->size = sizeof(struct xyzzy_params) */
        u32 param_1;
        u64 param_2;
        u64 param_3;
    };

```
鍙鍚庣画娣诲姞鐨勪换浣曞瓧娈碉紙渚嬪 `param_4`锛夊湪璁捐涓婁娇寰楅浂鍊肩粰鍑哄厛鍓嶇殑琛ㄧ幇锛屽氨鑳藉簲瀵逛袱绉嶆柟鍚戠殑鐗堟湰涓嶅尮閰嶏細

 - 涓轰簡搴斿杈冩柊鐨勭敤鎴风┖闂寸▼搴忚皟鐢ㄨ緝鏃у唴鏍哥殑鎯呭喌锛屽唴鏍镐唬鐮佸簲妫€鏌ヨ秴鍑哄叾鎵€鏈熸湜鐨勭粨鏋勪綋澶у皬涔嬪鐨勪换浣曞唴瀛樻槸鍚︿负闆讹紙瀹炶川涓婂氨鏄鏌?`param_4 == 0`锛夈€? - 涓轰簡搴斿杈冩棫鐨勭敤鎴风┖闂寸▼搴忚皟鐢ㄨ緝鏂板唴鏍哥殑鎯呭喌锛屽唴鏍镐唬鐮佸彲浠ュ杈冨皬鐨勭粨鏋勪綋瀹炰緥鍋氶浂鎵╁睍锛堝疄璐ㄤ笂灏辨槸璁剧疆 `param_4 = 0`锛夈€?
鍏充簬杩欑鍋氭硶鐨勪緥瀛愶紝鍙傝 `perf_event_open(2)` 鍜?`perf_copy_attr()` 鍑芥暟锛堜綅浜?`kernel/events/core.c`锛夈€?

### 璁捐 API锛氬叾浠栨敞鎰忎簨椤?

濡傛灉浣犵殑鏂扮郴缁熻皟鐢ㄥ厑璁哥敤鎴风┖闂村紩鐢ㄤ竴涓唴鏍稿璞★紝瀹冨簲璇ヤ娇鐢ㄦ枃浠舵弿杩扮浣滀负璇ュ璞＄殑鍙ユ焺鈥斺€旀棦鐒跺唴鏍稿凡缁忔湁浜嗕娇鐢ㄦ枃浠舵弿杩扮鐨勬満鍒跺拰鏄庣‘鐨勮涔夛紝灏变笉瑕佸彂鏄庝竴绉嶆柊绫诲瀷鐨勭敤鎴风┖闂村璞″彞鏌勩€?
濡傛灉浣犵殑鏂?xyzzy(2) 绯荤粺璋冪敤纭疄杩斿洖涓€涓柊鏂囦欢鎻忚堪绗︼紝閭ｄ箞 flags 鍙傛暟搴斿綋鍖呭惈涓€涓瓑浠蜂簬鍦ㄦ柊 FD 涓婅缃?`O_CLOEXEC` 鐨勫€笺€傝繖浣跨敤鎴风┖闂磋兘澶熷叧闂?`xyzzy()` 涓庤皟鐢?`fcntl(fd, F_SETFD, FD_CLOEXEC)` 涔嬮棿鐨勬椂闂寸獥鍙ｏ紝鍚﹀垯鍙︿竴涓嚎绋嬩腑鎰忓鐨?`fork()` 鍜?`execve()` 鍙兘灏嗘弿杩扮娉勬紡缁欒 exec 鐨勭▼搴忋€傦紙涓嶈繃锛屼笉瑕佽椽鍥惧鐢?`O_CLOEXEC` 甯搁噺鐨勫疄闄呮暟鍊硷紝鍥犱负瀹冩槸鏋舵瀯鐩稿叧鐨勶紝骞朵笖灞炰簬涓€涓浉褰撴嫢鎸ょ殑 `O_*` 鏍囧織缂栧彿绌洪棿鐨勪竴閮ㄥ垎銆傦級

濡傛灉浣犵殑绯荤粺璋冪敤杩斿洖涓€涓柊鏂囦欢鎻忚堪绗︼紝浣犺繕搴斿綋鑰冭檻鍦ㄨ鏂囦欢鎻忚堪绗︿笂浣跨敤 `poll(2)` 绯诲垪绯荤粺璋冪敤鎰忓懗鐫€浠€涔堛€傝鏂囦欢鎻忚堪绗﹀浜庡彲璇绘垨鍙啓灏辩华鐘舵€侊紝鏄唴鏍稿悜鐢ㄦ埛绌洪棿鎸囩ず鐩稿簲鍐呮牳瀵硅薄涓婂凡鍙戠敓浜嬩欢鐨勬甯告柟寮忋€?
```

    int sys_xyzzy(const char __user *path, ..., unsigned int flags);

```
```

    int sys_xyzzyat(int dfd, const char __user *path, ..., unsigned int flags);

```
杩欎负鐢ㄦ埛绌洪棿鎸囧畾鐩稿叧鏂囦欢鎻愪緵浜嗘洿澶х殑鐏垫椿鎬э紱鐗瑰埆鏄畠鍏佽鐢ㄦ埛绌洪棿浣跨敤 `AT_EMPTY_PATH` 鏍囧織涓哄凡缁忔墦寮€鐨勬枃浠舵弿杩扮璇锋眰璇ュ姛鑳斤紝瀹炶川涓?
```

 - xyzzyat(AT_FDCWD, path, ..., 0) is equivalent to xyzzy(path,...)
 - xyzzyat(fd, "", ..., AT_EMPTY_PATH) is equivalent to fxyzzy(fd, ...)

```
锛堝叧浜?\*at() 璋冪敤鐞嗙敱鐨勬洿澶氱粏鑺傦紝鍙傝 `openat(2)` 鎵嬪唽椤碉紱鍏充簬 AT_EMPTY_PATH 鐨勪緥瀛愶紝鍙傝 `fstatat(2)` 鎵嬪唽椤点€傦級

濡傛灉浣犵殑鏂?xyzzy(2) 绯荤粺璋冪敤娑夊強涓€涓弿杩版枃浠跺唴鍋忕Щ閲忕殑鍙傛暟锛岃灏嗗叾绫诲瀷璁句负 `loff_t`锛屼互渚垮嵆浣垮湪 32 浣嶆灦鏋勪笂涔熻兘鏀寔 64 浣嶅亸绉婚噺銆?
濡傛灉浣犵殑鏂?xyzzy(2) 绯荤粺璋冪敤娑夊強鐗规潈鍔熻兘锛屽畠蹇呴』鍙楃浉搴旂殑 Linux capability 浣嶏紙閫氳繃璋冪敤 `capable()` 妫€鏌ワ級绠¤緰锛屽 `capabilities(7)` 鎵嬪唽椤垫墍杩般€傞€夋嫨涓€涓杈栫浉鍏冲姛鑳界殑宸叉湁 capability 浣嶏紝浣嗚灏介噺閬垮厤鎶婅澶氫粎妯＄硦鐩稿叧鐨勫姛鑳藉綊鍒板悓涓€涓綅涓嬶紝鍥犱负杩欒繚鑳屼簡 capability 鎷嗗垎 root 鏉冮檺鐨勫垵琛枫€傚挨鍏惰閬垮厤鏂板瀵瑰凡杩囧害閫氱敤鐨?`CAP_SYS_ADMIN` capability 鐨勪娇鐢ㄣ€?
濡傛灉浣犵殑鏂?xyzzy(2) 绯荤粺璋冪敤鎿嶄綔璋冪敤杩涚▼涔嬪鐨勫彟涓€涓繘绋嬶紝搴斿綋鍔犱互闄愬埗锛堥€氳繃璋冪敤 `ptrace_may_access()`锛夛紝浣垮緱鍙湁涓庣洰鏍囪繘绋嬪叿鏈夌浉鍚屾潈闄愩€佹垨鑰呭叿鏈夊繀瑕?capability 鐨勮皟鐢ㄨ繘绋嬫墠鑳芥搷浣滅洰鏍囪繘绋嬨€?
鏈€鍚庤娉ㄦ剰锛屽鏋滄樉寮忎负 64 浣嶇殑绯荤粺璋冪敤鍙傛暟钀藉湪濂囨暟鍙峰弬鏁颁笂锛堝嵆鍙傛暟 1銆?銆?锛夛紝鏌愪簺闈?x86 鏋舵瀯浼氭洿瀹规槗澶勭悊锛屼互渚夸娇鐢ㄨ繛缁殑 32 浣嶅瘎瀛樺櫒瀵广€傦紙濡傛灉鍙傛暟鏄€氳繃鎸囬拡浼犲叆鐨勭粨鏋勪綋鐨勪竴閮ㄥ垎锛屽垯涓嶅瓨鍦ㄦ闂銆傦級


### 鎻愪氦 API 鎻愭


涓轰簡璁╂柊绯荤粺璋冪敤鏄撲簬瀹℃煡锛屾渶濂芥妸琛ヤ竵闆嗘媶鍒嗘垚鐙珛鐨勫潡銆傝繖浜涘潡鑷冲皯搴斿寘鍚互涓嬩綔涓虹嫭绔嬫彁浜ょ殑椤癸紙姣忎竴椤瑰湪涓嬮潰杩涗竴姝ヨ鏄庯級锛?
 - 绯荤粺璋冪敤鐨勬牳蹇冨疄鐜帮紝杩炲悓鍘熷瀷銆侀€氱敤缂栧彿銆並config 鍙樻洿浠ュ強鍏滃簳妗╁疄鐜般€? - 涓烘煇涓壒瀹氭灦鏋勶紙閫氬父鏄?x86锛屽寘鎷?x86_64銆亁86_32 鍜?x32锛夋帴鍏ユ柊绯荤粺璋冪敤銆? - 閫氳繃 `tools/testing/selftests/` 涓殑涓€涓嚜娴嬭瘯鏉ユ紨绀烘柊绯荤粺璋冪敤鍦ㄧ敤鎴风┖闂翠腑鐨勭敤娉曘€? - 鏂扮郴缁熻皟鐢ㄧ殑鎵嬪唽椤佃崏绋匡紝鍙互浣滀负绾枃鏈斁鍦ㄥ皝闈俊涓紝涔熷彲浠ヤ綔涓鸿ˉ涓佹彁浜ゅ埌锛堢嫭绔嬬殑锛塵an-pages 浠撳簱銆?
涓庡唴鏍?API 鐨勪换浣曞彉鏇翠竴鏍凤紝鏂扮殑绯荤粺璋冪敤鎻愭搴斿綋濮嬬粓鎶勯€侊紙cc锛夊埌 linux-api@vger.kernel.org銆?

### 閫氱敤绯荤粺璋冪敤瀹炵幇


浣犵殑鏂?xyzzy(2) 绯荤粺璋冪敤鐨勪富鍏ュ彛鐐瑰皢琚О涓?`sys_xyzzy()`锛屼絾浣犲簲璇ョ敤鐩稿簲鐨?`SYSCALL_DEFINEn()` 瀹忔潵娣诲姞杩欎釜鍏ュ彛鐐癸紝鑰屼笉鏄樉寮忓湴娣诲姞銆傚叾涓殑 'n' 琛ㄧず绯荤粺璋冪敤鐨勫弬鏁颁釜鏁帮紝璇ュ畯鎺ュ彈绯荤粺璋冪敤鍚嶏紝鍚庨潰璺熶笂浣滀负鍙傛暟鐨勶紙绫诲瀷锛屽悕绉帮級瀵广€備娇鐢ㄨ繖涓畯鍙互璁╂湁鍏虫柊绯荤粺璋冪敤鐨勫厓鏁版嵁瀵瑰叾浠栧伐鍏峰彲鐢ㄣ€?
鏂扮殑鍏ュ彛鐐硅繕闇€瑕佷竴涓搴旂殑鍑芥暟鍘熷瀷锛屼綅浜?`include/linux/syscalls.h` 涓紝鏍囪涓?asmlinkage 浠ュ尮閰嶇郴缁?
```

    asmlinkage long sys_xyzzy(...);

```
鏌愪簺鏋舵瀯锛堜緥濡?x86锛夋湁瀹冧滑鑷繁鏋舵瀯鐗瑰畾鐨勭郴缁熻皟鐢ㄨ〃锛屼絾鍏朵粬涓€浜涙灦鏋勫叡浜竴涓€氱敤绯荤粺璋冪敤琛ㄣ€傞€氳繃鍦ㄤ互涓嬪垪琛ㄦ坊鍔犱竴涓潯鐩紝灏嗕綘鐨勬柊绯荤粺璋冪敤鍔犲叆閫氱敤鍒楄〃

```

    #define __NR_xyzzy 292
    __SYSCALL(__NR_xyzzy, sys_xyzzy)

```
杩樿鏇存柊 __NR_syscalls 璁℃暟浠ュ弽鏄犳柊澧炵殑绯荤粺璋冪敤锛屽苟涓旀敞鎰忥紝濡傛灉鍦ㄥ悓涓€涓悎骞剁獥鍙ｄ腑娣诲姞浜嗗涓柊绯荤粺璋冪敤锛屼綘鐨勬柊绯荤粺璋冪敤鍙峰彲鑳戒細琚皟鏁翠互瑙ｅ喅鍐茬獊銆?
鏂囦欢 `kernel/sys_ni.c` 涓烘瘡涓郴缁熻皟鐢ㄦ彁渚涗竴涓厹搴曟々瀹炵幇锛?
```

    COND_SYSCALL(xyzzy);

```
浣犵殑鏂板唴鏍稿姛鑳戒互鍙婃帶鍒跺畠鐨勭郴缁熻皟鐢ㄩ€氬父搴斿綋鏄彲閫夌殑锛屽洜姝や负瀹冩坊鍔犱竴涓?`CONFIG` 閫夐」锛堥€氬父鍦?`init/Kconfig` 涓級銆備笌鏂板 `CONFIG` 閫夐」鐨勬儻渚嬩竴鏍凤細

 - 鍖呭惈瀵硅閫夐」鎵€鎺у埗鐨勬柊鍔熻兘鍜岀郴缁熻皟鐢ㄧ殑璇存槑銆? - 濡傛灉瀹冨簲璇ュ鏅€氱敤鎴烽殣钘忥紝鍒欒璇ラ€夐」渚濊禆浜?EXPERT銆? - 璁╀换浣曞疄鐜拌鍔熻兘鐨勬簮鏂囦欢鍦?Makefile 涓緷璧栦簬璇?CONFIG 閫夐」锛堜緥濡?`obj-$(CONFIG_XYZZY_SYSCALL) += xyzzy.o`锛夈€? - 鍐嶆妫€鏌ュ湪鍐呮牳鍏抽棴璇ユ柊 CONFIG 閫夐」鏃朵粛鑳芥甯告瀯寤恒€?
鎬荤粨涓€涓嬶紝浣犻渶瑕佷竴涓寘鍚互涓嬪唴瀹圭殑鎻愪氦锛?
 - `CONFIG` option for the new function, normally in `init/Kconfig`
 - `SYSCALL_DEFINEn(xyzzy, ...)` for the entry point
 - corresponding prototype in `include/linux/syscalls.h`
 - generic table entry in `include/uapi/asm-generic/unistd.h`
 - fallback stub in `kernel/sys_ni.c`


#### Since 6.11


浠庡唴鏍哥増鏈?6.11 寮€濮嬶紝閽堝浠ヤ笅鏋舵瀯鐨勯€氱敤绯荤粺璋冪敤瀹炵幇涓嶅啀闇€瑕佷慨鏀?`include/uapi/asm-generic/unistd.h`锛?
 - arc
 - arm64
 - csky
 - hexagon
 - loongarch
 - nios2
 - openrisc
 - riscv

鍙栬€屼唬涔嬶紝浣犻渶瑕佹洿鏂?`scripts/syscall.tbl`锛屽苟鍦ㄩ€傜敤鏃惰皟鏁?`arch/*/kernel/Makefile.syscalls`銆?
鐢变簬 `scripts/syscall.tbl` 鍏呭綋璺ㄨ秺澶氫釜鏋舵瀯鐨勯€氱敤绯荤粺璋冪敤琛紝

```

    468   common   xyzzy     sys_xyzzy

```
娉ㄦ剰锛屽悜 `scripts/syscall.tbl` 娣诲姞甯︽湁 "common" ABI 鐨勬潯鐩篃浼氬奖鍝嶅叡浜琛ㄧ殑鎵€鏈夋灦鏋勩€傚浜庢洿鍙楅檺鎴栫壒瀹氫簬鏋舵瀯鐨勬洿鏀癸紝鍙互鑰冭檻浣跨敤鐗瑰畾浜庢灦鏋勭殑 ABI 鎴栧畾涔変竴涓柊鐨?ABI銆?
濡傛灉寮曞叆涓€涓柊鐨?ABI锛屼緥濡?`xyz`锛岀浉搴旂殑鏇存柊搴斾负

```

    syscall_abis_{32,64} += xyz (...)

```
鎬荤粨涓€涓嬶紝浣犻渶瑕佷竴涓寘鍚互涓嬪唴瀹圭殑鎻愪氦锛?
 - `CONFIG` option for the new function, normally in `init/Kconfig`
 - `SYSCALL_DEFINEn(xyzzy, ...)` for the entry point
 - corresponding prototype in `include/linux/syscalls.h`
 - new entry in `scripts/syscall.tbl`
 - (if needed) Makefile updates in `arch/*/kernel/Makefile.syscalls`
 - fallback stub in `kernel/sys_ni.c`


### x86 绯荤粺璋冪敤瀹炵幇


瑕佷负 x86 骞冲彴鎺ュ叆浣犵殑鏂扮郴缁熻皟鐢紝浣犻渶瑕佹洿鏂颁富绯荤粺璋冪敤琛ㄣ€傚亣璁句綘鐨勬柊绯荤粺璋冪敤鍦ㄦ煇绉嶇▼搴︿笂涓嶇壒娈婏紙瑙佷笅鏂囷級锛岃繖娑夊強鍦?
```

    333   common   xyzzy     sys_xyzzy

```
```

    380   i386     xyzzy     sys_xyzzy

```
鍚屾牱锛岃繖浜涚紪鍙峰鏋滃湪鐩稿叧鍚堝苟绐楀彛涓嚭鐜板啿绐侊紝寰堝彲鑳戒細琚洿鏀广€?

### 鍏煎鎬х郴缁熻皟鐢紙閫氱敤锛?

瀵逛簬澶у鏁扮郴缁熻皟鐢紝鍗充娇鏄敤 32 浣嶇紪璇戠殑鐢ㄦ埛绌洪棿绋嬪簭锛屼篃鍙互璋冪敤鐩稿悓鐨?64 浣嶅疄鐜帮紱鍗充娇绯荤粺璋冪敤鐨勫弬鏁板寘鍚竴涓樉寮忔寚閽堬紝杩欎篃浼氳閫忔槑鍦板鐞嗐€?
涓嶈繃锛屾湁鍑犵鎯呭喌闇€瑕佸吋瀹规€у眰鏉ュ簲瀵?32 浣嶄笌 64 浣嶄箣闂寸殑澶у皬宸紓銆?
绗竴绉嶆儏鍐垫槸锛屽鏋?64 浣嶅唴鏍稿悓鏃朵篃鏀寔 32 浣嶇敤鎴风┖闂寸▼搴忥紝鍥犳闇€瑕佽В鏋愬彲鑳戒繚瀛?32 浣嶆垨 64 浣嶅€肩殑锛坄__user`锛夊唴瀛樺尯鍩熴€傜壒鍒湴锛屽彧瑕佺郴缁熻皟鐢ㄧ殑鍙傛暟鏄互涓嬩箣涓€锛屽氨闇€瑕佽繖鏍峰仛锛?
 - 鎸囧悜鎸囬拡鐨勬寚閽? - 鎸囧悜鍖呭惈鎸囬拡鐨勭粨鏋勪綋鐨勬寚閽堬紙渚嬪 `struct iovec __user *`锛? - 鎸囧悜澶у皬鍙彉鐨勬暣鍨嬬殑鎸囬拡锛坄time_t`銆乣off_t`銆乣long`鈥︹€︼級
 - 鎸囧悜鍖呭惈澶у皬鍙彉鐨勬暣鍨嬬殑缁撴瀯浣撶殑鎸囬拡

闇€瑕佸吋瀹规€у眰鐨勭浜岀鎯呭喌鏄紝濡傛灉绯荤粺璋冪敤鐨勬煇涓弬鏁板叿鏈夊嵆浣垮湪 32 浣嶆灦鏋勪笂涔熸樉寮忎负 64 浣嶇殑绫诲瀷锛屼緥濡?`loff_t` 鎴?`__u64`銆傚湪杩欑鎯呭喌涓嬶紝浠?32 浣嶅簲鐢ㄧ▼搴忓埌杈?64 浣嶅唴鏍哥殑鍊煎皢琚媶鍒嗘垚涓や釜 32 浣嶅€硷紝闅忓悗闇€瑕佸湪鍏煎鎬у眰涓噸鏂扮粍瑁呫€?
锛堟敞鎰忥紝鎸囧悜鏄惧紡 64 浣嶇被鍨嬬殑鎸囬拡浣滀负绯荤粺璋冪敤鍙傛暟鏃?*涓?*闇€瑕佸吋瀹规€у眰锛涗緥濡傦紝`splice(2)` 涓被鍨嬩负 `loff_t __user *` 鐨勫弬鏁板苟涓嶄細瑙﹀彂瀵?`compat_` 绯荤粺璋冪敤鐨勯渶姹傘€傦級

绯荤粺璋冪敤鐨勫吋瀹规€х増鏈О涓?`compat_sys_xyzzy()`锛屼娇鐢?`COMPAT_SYSCALL_DEFINEn()` 瀹忔坊鍔狅紝绫讳技浜?SYSCALL_DEFINEn銆傝繖涓疄鐜扮増鏈綔涓?64 浣嶅唴鏍哥殑涓€閮ㄥ垎杩愯锛屼絾鏈熸湜鎺ユ敹 32 浣嶅弬鏁板€硷紝骞跺仛鎵€闇€鐨勪竴鍒囨潵澶勭悊瀹冧滑銆傦紙閫氬父锛宍compat_sys_` 鐗堟湰浼氭妸鍊艰浆鎹负 64 浣嶇増鏈紝鐒跺悗瑕佷箞璋冪敤 `sys_` 鐗堟湰锛岃涔堢敱瀹冧滑涓よ€呴兘璋冪敤涓€涓叕鍏辩殑鍐呴儴瀹炵幇鍑芥暟銆傦級

鍏煎鎬у叆鍙ｇ偣杩橀渶瑕佷竴涓搴旂殑鍑芥暟鍘熷瀷锛屼綅浜?`include/linux/compat.h` 涓紝鏍囪涓?asmlinkage 浠ュ尮閰嶇郴缁?
```

    asmlinkage long compat_sys_xyzzy(...);

```
濡傛灉绯荤粺璋冪敤娑夊強涓€涓湪 32 浣嶅拰 64 浣嶇郴缁熶笂甯冨眬涓嶅悓鐨勭粨鏋勪綋锛屼緥濡?`struct xyzzy_args`锛岄偅涔?include/linux/compat.h 澶存枃浠惰繕搴斿寘鍚竴涓缁撴瀯浣撶殑 compat 鐗堟湰锛坄`struct compat_xyzzy_args``锛夛紝鍏朵腑姣忎釜鍙彉澶у皬鐨勫瓧娈甸兘鍏锋湁涓?`struct xyzzy_args` 涓被鍨嬪搴旂殑閫傚綋鐨?`compat_` 绫诲瀷銆俙compat_sys_xyzzy()` 渚嬬▼闅忓悗渚垮彲浠ヤ娇鐢ㄨ繖涓?`compat_` 缁撴瀯浣撴潵瑙ｆ瀽鏉ヨ嚜 32 浣嶈皟鐢ㄧ殑鍙傛暟銆?
```

    struct xyzzy_args {
        const char __user *ptr;
        __kernel_long_t varying_val;
        u64 fixed_val;
        /* ... */
    };

```
```

    struct compat_xyzzy_args {
        compat_uptr_t ptr;
        compat_long_t varying_val;
        u64 fixed_val;
        /* ... */
    };

```
閫氱敤绯荤粺璋冪敤鍒楄〃涔熼渶瑕佽皟鏁翠互瀹圭撼 compat 鐗堟湰锛沗include/uapi/asm-generic/unistd.h` 涓殑鏉＄洰搴斿綋浣跨敤

```

    #define __NR_xyzzy 292
    __SC_COMP(__NR_xyzzy, sys_xyzzy, compat_sys_xyzzy)

```
鎬荤粨涓€涓嬶紝浣犻渶瑕侊細

 - a `COMPAT_SYSCALL_DEFINEn(xyzzy, ...)` for the compat entry point
 - corresponding prototype in `include/linux/compat.h`
 - (if needed) 32-bit mapping struct in `include/linux/compat.h`
 - instance of `__SC_COMP` not `__SYSCALL` in
   `include/uapi/asm-generic/unistd.h`


#### Since 6.11


杩欓€傜敤浜庘€滈€氱敤绯荤粺璋冪敤瀹炵幇鈥濅腑鍒楀嚭鐨勩€侀櫎 arm64 涔嬪鐨勬墍鏈夋灦鏋?Since 6.11<syscall_generic_6_11>銆傛洿澶氫俊鎭弬瑙?Compatibility System Calls (arm64)<compat_arm64>銆?
浣犻渶瑕佷负 `scripts/syscall.tbl` 涓殑鏉＄洰澧炲姞涓€涓澶栫殑鍒楋紝浠ユ寚绀鸿繍琛屽湪 64 浣嶅唴鏍镐笂鐨?32 浣嶇敤鎴风┖闂寸▼搴忓簲璇?
```

    468   common     xyzzy     sys_xyzzy    compat_sys_xyzzy

```
鎬荤粨涓€涓嬶紝浣犻渶瑕侊細

 - `COMPAT_SYSCALL_DEFINEn(xyzzy, ...)` for the compat entry point
 - corresponding prototype in `include/linux/compat.h`
 - modification of the entry in `scripts/syscall.tbl` to include an extra
   "compat" column
 - (if needed) 32-bit mapping struct in `include/linux/compat.h`



##### 鍏煎鎬х郴缁熻皟鐢紙arm64锛?

鍦?arm64 涓婏紝鏈変竴涓笓鐢ㄤ簬闈㈠悜 32 浣嶏紙AArch32锛夌敤鎴风┖闂寸殑鍏煎鎬х郴缁熻皟鐢ㄧ殑绯荤粺璋冪敤琛細`arch/arm64/tools/syscall_32.tbl`銆備綘闇€瑕佸悜姝よ〃娣诲姞涓€琛岋紝鎸囧畾 compat

```

    468   common     xyzzy     sys_xyzzy    compat_sys_xyzzy


```
### 鍏煎鎬х郴缁熻皟鐢紙x86锛?

瑕佷负甯︽湁鍏煎鎬х増鏈殑绯荤粺璋冪敤鎺ュ叆 x86 鏋舵瀯锛岄渶瑕佽皟鏁寸郴缁熻皟鐢ㄨ〃涓殑鏉＄洰銆?
棣栧厛锛宍arch/x86/entry/syscalls/syscall_32.tbl` 涓殑鏉＄洰浼氳幏寰椾竴涓澶栫殑鍒楋紝浠ユ寚绀鸿繍琛屽湪 64 浣嶅唴鏍镐笂鐨?32 浣嶇敤鎴风┖闂寸▼搴?
```

    380   i386     xyzzy     sys_xyzzy    __ia32_compat_sys_xyzzy

```
鍏舵锛屼綘闇€瑕佸紕娓呮鏂扮郴缁熻皟鐢ㄧ殑 x32 ABI 鐗堟湰搴斿綋濡備綍琛ㄧ幇銆傝繖閲屾湁涓€夋嫨锛氬弬鏁扮殑甯冨眬搴斿綋瑕佷箞鍖归厤 64 浣嶇増鏈紝瑕佷箞鍖归厤 32 浣嶇増鏈€?
濡傛灉娑夊強鎸囧悜鎸囬拡鐨勬寚閽堬紝鍐冲畾灏卞緢绠€鍗曪細x32 鏄?ILP32锛屽洜姝ゅ竷灞€搴斿綋鍖归厤 32 浣嶇増鏈紝骞朵笖 `arch/x86/entry/syscalls/syscall_64.tbl` 涓殑鏉＄洰浼氳鎷嗗垎锛屼娇寰?x32 绋嬪簭鍛戒腑

```

    333   64       xyzzy     sys_xyzzy
    ...
    555   x32      xyzzy     __x32_compat_sys_xyzzy

```
濡傛灉涓嶆秹鍙婁换浣曟寚閽堬紝閭ｄ箞鏈€濂戒负 x32 ABI 澶嶇敤 64 浣嶇郴缁熻皟鐢紙鍥犳 arch/x86/entry/syscalls/syscall_64.tbl 涓殑鏉＄洰淇濇寔涓嶅彉锛夈€?
鏃犺鍝鎯呭喌锛屼綘閮藉簲褰撴鏌ュ弬鏁板竷灞€涓墍娑夊強鐨勭被鍨嬬‘瀹炶兘绮剧‘鍦颁粠 x32锛?mx32锛夋槧灏勫埌 32 浣嶏紙-m32锛夋垨 64 浣嶏紙-m64锛夌殑绛変环绫诲瀷銆?

### 鍦ㄥ叾浠栦綅缃繑鍥炵殑绯荤粺璋冪敤


瀵逛簬澶у鏁扮郴缁熻皟鐢紝涓€鏃︾郴缁熻皟鐢ㄥ畬鎴愶紝鐢ㄦ埛绋嬪簭浼氭伆濂戒粠瀹冪寮€鐨勫湴鏂圭户缁€斺€斿嵆涓嬩竴鏉℃寚浠ゅ锛屾爤涓庣郴缁熻皟鐢ㄥ墠鐩稿悓锛屽ぇ澶氭暟瀵勫瓨鍣ㄤ篃鐩稿悓锛屽苟涓斿叿鏈夌浉鍚岀殑铏氭嫙鍐呭瓨绌洪棿銆?
涓嶈繃锛屽皯鏁扮郴缁熻皟鐢ㄧ殑琛屼负涓嶅悓銆傚畠浠彲鑳借繑鍥炲埌涓嶅悓鐨勪綅缃紙`rt_sigreturn`锛夛紝鎴栬€呮敼鍙樼▼搴忕殑鍐呭瓨绌洪棿锛坄fork`/`vfork`/`clone`锛夛紝鐢氳嚦鏀瑰彉鏋舵瀯锛坄execve`/`execveat`锛夈€?
涓轰簡鏀寔杩欎竴鐐癸紝绯荤粺璋冪敤鐨勫唴鏍稿疄鐜板彲鑳介渶瑕佸悜鍐呮牳鏍堜繚瀛樺苟鎭㈠棰濆鐨勫瘎瀛樺櫒锛屼粠鑰屽畬鍏ㄦ帶鍒剁郴缁熻皟鐢ㄤ箣鍚庢墽琛岀殑浣嶇疆鍜屾柟寮忋€?
杩欐槸鏋舵瀯鐩稿叧鐨勶紝浣嗛€氬父娑夊強瀹氫箟姹囩紪鍏ュ彛鐐癸紝杩欎簺鍏ュ彛鐐逛繚瀛?鎭㈠棰濆鐨勫瘎瀛樺櫒骞惰皟鐢ㄧ湡姝ｇ殑绯荤粺璋冪敤鍏ュ彛鐐广€?
瀵逛簬 x86_64锛岃繖琚疄鐜颁负 `arch/x86/entry/entry_64.S` 涓悕涓?`stub_xyzzy` 鐨勫叆鍙ｇ偣锛岃€岀郴缁熻皟鐢ㄨ〃涓殑鏉＄洰

```

    333   common   xyzzy     stub_xyzzy

```
鍦?64 浣嶅唴鏍镐笂杩愯鐨?32 浣嶇▼搴忕殑绛変环鐗╅€氬父绉颁负 `stub32_xyzzy`锛屽湪 `arch/x86/entry/entry_64_compat.S` 涓疄鐜帮紝鐩稿簲鐨勭郴缁熻皟鐢ㄨ〃璋冩暣涓?
```

    380   i386     xyzzy     sys_xyzzy    stub32_xyzzy

```
濡傛灉绯荤粺璋冪敤闇€瑕佷竴涓吋瀹规€у眰锛堝涓婁竴鑺傛墍杩帮級锛岄偅涔?`stub32_` 鐗堟湰闇€瑕佽皟鐢ㄧ郴缁熻皟鐢ㄧ殑 `compat_sys_` 鐗堟湰锛岃€屼笉鏄師鐢熺殑 64 浣嶇増鏈€傛澶栵紝濡傛灉 x32 ABI 鐨勫疄鐜颁笌 x86_64 鐗堟湰涓嶇浉鍚岋紝閭ｄ箞瀹冪殑绯荤粺璋冪敤琛ㄤ篃闇€瑕佽皟鐢ㄤ竴涓細杞悜 `compat_sys_` 鐗堟湰鐨勬々銆?
涓轰簡瀹屾暣鎬э紝鏈€濂戒篃寤虹珛涓€涓槧灏勶紝浣跨敤鎴锋€?Linux锛圲ser-Mode Linux锛変粛鑳藉伐浣溾€斺€斿畠鐨勭郴缁熻皟鐢ㄨ〃浼氬紩鐢?stub_xyzzy锛屼絾 UML 鐨勬瀯寤轰笉鍖呭惈 `arch/x86/entry/entry_64.S` 鐨勫疄鐜帮紙鍥犱负 UML 妯℃嫙浜嗗瘎瀛樺櫒绛夛級銆備慨澶嶆柟娉曞緢绠€鍗曪紝鍙渶鍚?
```

    #define stub_xyzzy sys_xyzzy


```
### 鍏朵粬缁嗚妭


鍐呮牳鐨勫ぇ閮ㄥ垎浠ラ€氱敤鏂瑰紡澶勭悊绯荤粺璋冪敤锛屼絾鍋跺皵涔熸湁渚嬪锛屽彲鑳介渶瑕佷负浣犵殑鐗瑰畾绯荤粺璋冪敤鍋氭洿鏂般€?
瀹¤锛坅udit锛夊瓙绯荤粺灏辨槸杩欐牱涓€涓壒娈婃儏鍐碉紱瀹冨寘鍚紙鏋舵瀯鐩稿叧鐨勶級鍑芥暟锛岀敤浜庡鏌愪簺鐗规畩绫诲瀷鐨勭郴缁熻皟鐢ㄨ繘琛屽垎绫烩€斺€斿叿浣撴槸鏂囦欢鎵撳紑锛坄open`/`openat`锛夈€佺▼搴忔墽琛岋紙`execve`/`exeveat`锛夋垨濂楁帴瀛楀璺鐢ㄥ櫒锛坄socketcall`锛夋搷浣溿€傚鏋滀綘鐨勬柊绯荤粺璋冪敤绫讳技浜庡叾涓箣涓€锛岄偅涔堝簲褰撴洿鏂板璁＄郴缁熴€?
鏇翠竴鑸湴璇达紝濡傛灉瀛樺湪涓庝綘鐨勬柊绯荤粺璋冪敤绫讳技鐨勫凡鏈夌郴缁熻皟鐢紝鍊煎緱鍦ㄥ唴鏍歌寖鍥村唴瀵硅宸叉湁绯荤粺璋冪敤鍋氫竴娆?grep锛屼互妫€鏌ユ槸鍚︽病鏈夊叾浠栫壒娈婃儏鍐点€?

### 娴嬭瘯


鏂扮殑绯荤粺璋冪敤鏄剧劧搴斿綋琚祴璇曪紱涓哄鏌ヨ€呮彁渚涚敤鎴风┖闂寸▼搴忓皢濡備綍浣跨敤璇ョ郴缁熻皟鐢ㄧ殑婕旂ず涔熷緢鏈夌敤銆傜粨鍚堣繖涓や釜鐩爣鐨勪竴涓ソ鍔炴硶鏄紝鍦?`tools/testing/selftests/` 涓嬬殑涓€涓柊鐩綍涓寘鍚竴涓畝鍗曠殑鑷祴璇曠▼搴忋€?
瀵逛簬鏂扮殑绯荤粺璋冪敤锛屾樉鐒朵笉浼氭湁 libc 鍖呰鍑芥暟锛屽洜姝ゆ祴璇曢渶瑕佷娇鐢?`syscall()` 鏉ヨ皟鐢ㄥ畠锛涙澶栵紝濡傛灉绯荤粺璋冪敤娑夊強涓€涓柊鐨勭敤鎴风┖闂村彲瑙佺殑缁撴瀯浣擄紝鍒欓渶瑕佸畨瑁呯浉搴旂殑澶存枃浠舵墠鑳界紪璇戞祴璇曘€?
纭繚璇ヨ嚜娴嬭瘯鍦ㄦ墍鏈夋敮鎸佺殑鏋舵瀯涓婇兘鑳芥垚鍔熻繍琛屻€備緥濡傦紝妫€鏌ュ綋瀹冭缂栬瘧涓?x86_64锛?m64锛夈€亁86_32锛?m32锛夊拰 x32锛?mx32锛?ABI 绋嬪簭鏃堕兘鑳藉伐浣溿€?
瑕佸鏂板姛鑳藉仛鏇村箍娉涘拰褰诲簳鐨勬祴璇曪紝浣犺繕搴斿綋鑰冭檻鎶婃祴璇曟坊鍔犲埌 Linux Test Project锛屾垨鑰呴拡瀵规枃浠剁郴缁熺浉鍏崇殑鏇存敼娣诲姞鍒?xfstests 椤圭洰銆?
 - https://linux-test-project.github.io/
 - git://git.kernel.org/pub/scm/fs/xfs/xfstests-dev.git


### 鎵嬪唽椤?

鎵€鏈夋柊鐨勭郴缁熻皟鐢ㄩ兘搴旈檮甯︿竴浠藉畬鏁寸殑鎵嬪唽椤碉紝鐞嗘兂鎯呭喌浣跨敤 groff 鏍囪锛屼絾绾枃鏈篃鍙互銆傚鏋滀娇鐢?groff锛屾渶濂藉湪琛ヤ竵闆嗙殑灏侀潰閭欢涓寘鍚竴浠介娓叉煋鐨?ASCII 鐗堟墜鍐岄〉锛屼互鏂逛究瀹℃煡鑰呫€?
鎵嬪唽椤靛簲褰撴妱閫侊紙cc锛夊埌 linux-man@vger.kernel.org
鏇村缁嗚妭锛屽弬瑙?https://www.kernel.org/doc/man-pages/patches.html


### 涓嶈鍦ㄥ唴鏍镐腑璋冪敤绯荤粺璋冪敤


濡備笂鎵€杩帮紝绯荤粺璋冪敤鏄敤鎴风┖闂翠笌鍐呮牳涔嬮棿鐨勪氦浜掔偣銆傚洜姝わ紝鍍?`sys_xyzzy()` 鎴?`compat_sys_xyzzy()` 杩欐牱鐨勭郴缁熻皟鐢ㄥ嚱鏁板彧搴斾粠鐢ㄦ埛绌洪棿閫氳繃绯荤粺璋冪敤琛ㄨ皟鐢紝鑰屼笉搴斾粠鍐呮牳鐨勫叾浠栧湴鏂硅皟鐢ㄣ€傚鏋滅郴缁熻皟鐢ㄧ殑鍔熻兘鍦ㄥ唴鏍稿唴閮ㄦ湁鐢ㄣ€侀渶瑕佸湪鏂版棫涓や釜绯荤粺璋冪敤涔嬮棿鍏变韩锛屾垨鑰呴渶瑕佸湪绯荤粺璋冪敤涓庡叾鍏煎鎬у彉浣撲箣闂村叡浜紝閭ｄ箞瀹冨簲褰撻€氳繃鈥滆緟鍔╋紙helper锛夆€濆嚱鏁帮紙渚嬪 `ksys_xyzzy()`锛夋潵瀹炵幇銆傝繖涓唴鏍稿嚱鏁伴殢鍚庡彲浠ュ湪绯荤粺璋冪敤妗╋紙`sys_xyzzy()`锛夈€佸吋瀹规€х郴缁熻皟鐢ㄦ々锛坄compat_sys_xyzzy()`锛夊拰/鎴栧叾浠栧唴鏍镐唬鐮佷腑璋冪敤銆?
鑷冲皯鍦?64 浣?x86 涓婏紝浠?v4.17 寮€濮嬶紝涓嶅湪鍐呮牳涓皟鐢ㄧ郴缁熻皟鐢ㄥ嚱鏁板皢鏄竴涓‖鎬ц姹傘€傚畠浣跨敤涓嶅悓鐨勭郴缁熻皟鐢ㄨ皟鐢ㄧ害瀹氾紝鍏朵腑 `struct pt_regs` 鍦ㄧ郴缁熻皟鐢ㄥ寘瑁呭櫒涓嵆鏃惰В鐮侊紝鐒跺悗灏嗗鐞嗕氦缁欏疄闄呯殑绯荤粺璋冪敤鍑芥暟銆傝繖鎰忓懗鐫€绯荤粺璋冪敤鍏ュ彛澶勫彧浼犻€掔壒瀹氱郴缁熻皟鐢ㄥ疄闄呴渶瑕佺殑閭ｄ簺鍙傛暟锛岃€屼笉鏄缁堢敤闅忔満鐨勭敤鎴风┖闂村唴瀹瑰～婊″叚涓?CPU 瀵勫瓨鍣紙杩欏彲鑳戒細鍦ㄨ皟鐢ㄩ摼涓嬫父閫犳垚涓ラ噸楹荤儲锛夈€?
姝ゅ锛屽叧浜庢暟鎹浣曡璁块棶鐨勮鍒欏湪鍐呮牳鏁版嵁涓庣敤鎴锋暟鎹箣闂村彲鑳戒笉鍚屻€傝繖鏄笉搴旇皟鐢?`sys_xyzzy()` 鐨勫彟涓€涓師鍥犮€?
杩欐潯瑙勫垯鐨勪緥澶栧彧鍏佽鍑虹幇鍦ㄦ灦鏋勭浉鍏崇殑瑕嗙洊銆佹灦鏋勭浉鍏崇殑鍏煎鎬у寘瑁呭櫒锛屾垨 arch/ 涓殑鍏朵粬浠ｇ爜閲屻€?

### 鍙傝€冭祫鏂欎笌鏉ユ簮


 - Michael Kerrisk 鍏充簬绯荤粺璋冪敤涓?flags 鍙傛暟鐢ㄦ硶鐨?LWN 鏂囩珷锛?   https://lwn.net/Articles/585415/
 - Michael Kerrisk 鍏充簬绯荤粺璋冪敤涓浣曞鐞嗘湭鐭?flags 鐨?LWN 鏂囩珷锛?   https://lwn.net/Articles/588444/
 - Jake Edge 鎻忚堪 64 浣嶇郴缁熻皟鐢ㄥ弬鏁扮害鏉熺殑 LWN 鏂囩珷锛?   https://lwn.net/Articles/311630/
 - David Drysdale 璇︾粏鎻忚堪 v3.14 绯荤粺璋冪敤瀹炵幇璺緞鐨勪竴瀵?LWN 鏂囩珷锛?
    - https://lwn.net/Articles/604287/
    - https://lwn.net/Articles/604515/

 - 绯荤粺璋冪敤鐨勬灦鏋勭壒瀹氳姹傚湪 `syscall(2)` 鎵嬪唽椤典腑璁ㄨ锛?   http://man7.org/linux/man-pages/man2/syscall.2.html#NOTES
 - Linus Torvalds 璁ㄨ `ioctl()` 闂鐨勫線鏉ラ偖浠跺悎闆嗭細
   https://yarchive.net/comp/linux/ioctl.html
 - "濡備綍涓嶅彂鏄庡唴鏍告帴鍙?锛孉rnd Bergmann锛?   https://www.ukuug.org/events/linux2007/2007/papers/Bergmann.pdf
 - Michael Kerrisk 鍏充簬閬垮厤鏂颁娇鐢?CAP_SYS_ADMIN 鐨?LWN 鏂囩珷锛?   https://lwn.net/Articles/486306/
 - Andrew Morton 寤鸿鏂扮郴缁熻皟鐢ㄧ殑鎵€鏈夌浉鍏充俊鎭簲褰撳嚭鐜板湪鍚屼竴涓偖浠剁嚎绋嬩腑锛?   https://lore.kernel.org/r/20140724144747.3041b208832bbdf9fbce5d96@linux-foundation.org
 - Michael Kerrisk 寤鸿鏂扮郴缁熻皟鐢ㄥ簲褰撻檮甯︽墜鍐岄〉锛?   https://lore.kernel.org/r/CAKgNAkgMA39AfoSoA5Pe1r9N+ZzfYQNvNPvcRN7tOvRb8+v06Q@mail.gmail.com
 - Thomas Gleixner 寤鸿 x86 鎺ュ叆搴旀斁鍦ㄥ崟鐙殑鎻愪氦涓細
   https://lore.kernel.org/r/alpine.DEB.2.11.1411191249560.3909@nanos
 - Greg Kroah-Hartman 寤鸿鏂扮郴缁熻皟鐢ㄦ渶濂介檮甯︽墜鍐岄〉鍜岃嚜娴嬭瘯锛?   https://lore.kernel.org/r/20140320025530.GA25469@kroah.com
 - Michael Kerrisk 鍏充簬鏂扮郴缁熻皟鐢ㄤ笌 `prctl(2)` 鎵╁睍鐨勮璁猴細
   https://lore.kernel.org/r/CAHO5Pa3F2MjfTtfNxa8LbnkeeU8=YJ+9tDqxZpw7Gz59E-4AUg@mail.gmail.com
 - Ingo Molnar 寤鸿娑夊強澶氫釜鍙傛暟鐨勭郴缁熻皟鐢ㄥ簲灏嗛偅浜涘弬鏁板皝瑁呰繘涓€涓粨鏋勪綋锛屽苟鍖呭惈涓€涓敤浜庢湭鏉ユ墿灞曟€х殑 size 瀛楁锛?   https://lore.kernel.org/r/20150730083831.GA22182@gmail.com
 - 鐢憋紙閲嶆柊锛変娇鐢?O_* 缂栧彿绌洪棿鏍囧織寮曡捣鐨勭紪鍙锋€薄锛?
    - commit 75069f2b5bfb ("vfs: renumber FMODE_NONOTIFY and add to uniqueness
      check")
    - commit 12ed2e36c98a ("fanotify: FMODE_NONOTIFY and __O_SYNC in sparc
      conflict")
    - commit bb458c644a59 ("Safer ABI for O_TMPFILE")

 - Matthew Wilcox 鍏充簬 64 浣嶅弬鏁伴檺鍒剁殑璁ㄨ锛?   https://lore.kernel.org/r/20081212152929.GM26095@parisc-linux.org
 - Greg Kroah-Hartman 寤鸿搴斿綋瀵规湭鐭?flags 杩涜绠℃帶锛?   https://lore.kernel.org/r/20140717193330.GB4703@kroah.com
 - Linus Torvalds 寤鸿 x32 绯荤粺璋冪敤搴斿綋浼樺厛涓?64 浣嶇増鏈€岄潪 32 浣嶇増鏈吋瀹癸細
   https://lore.kernel.org/r/CA+55aFxfmwfB7jbbrXxa=K7VBYPfAvmu3XOkGrLbB1UFjX1+Ew@mail.gmail.com
 - 淇敼绯荤粺璋冪敤琛ㄥ熀纭€璁炬柦浠ュ湪澶氫釜鏋舵瀯涓婁娇鐢?scripts/syscall.tbl 鐨勮ˉ涓佺郴鍒楋細
   https://lore.kernel.org/lkml/20240704143611.2979589-1-arnd@kernel.org
