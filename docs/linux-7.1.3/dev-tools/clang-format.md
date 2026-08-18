
## clang-format


`clang-format` 鏄竴涓牴鎹竴缁勮鍒欏拰鍚彂寮忔柟娉曟牸寮忓寲 C/C++/... 浠ｇ爜鐨勫伐鍏枫€傚拰澶у鏁板伐鍏蜂竴鏍凤紝瀹冨苟涓嶅畬缇庯紝涔熸棤娉曡鐩栨瘡涓€绉嶆儏鍐碉紝浣嗗畠宸茬粡瓒冲濂界敤銆佽兘甯笂蹇欍€?

`clang-format` 鍙敤浜庡绉嶇敤閫旓細

  - 蹇€熷皢涓€鍧椾唬鐮侀噸鏂版牸寮忓寲涓哄唴鏍搁鏍笺€傚湪绉诲姩浠ｇ爜銆佸榻?鎺掑簭鏃剁壒鍒湁鐢ㄣ€傚弬瑙?clangformatreformat_銆?

  - 鍦ㄤ綘缁存姢鐨勬枃浠躲€佷綘瀹℃煡鐨勮ˉ涓併€乨iff 绛変腑鍙戠幇椋庢牸閿欒銆佺瑪璇互鍙婂彲鑳界殑鏀硅繘銆傚弬瑙?clangformatreview_銆?

  - 甯姪浣犻伒寰唬鐮侀鏍艰鍒欙紝瀵逛簬鍒氭帴瑙﹀唴鏍稿紑鍙戙€佹垨鑰呭悓鏃跺湪澶氫釜閲囩敤涓嶅悓浠ｇ爜椋庢牸鐨勯」鐩腑宸ヤ綔鐨勪汉鐗瑰埆鏈夌敤銆?

瀹冪殑閰嶇疆鏂囦欢鏄唴鏍告爲鏍圭洰褰曚笅鐨?`.clang-format`銆傚叾涓寘鍚殑瑙勫垯璇曞浘杩戜技鏈€甯歌鐨勫唴鏍镐唬鐮侀鏍笺€傚畠浠篃灏藉彲鑳介伒寰?Documentation/process/coding-style.rst <codingstyle>銆傜敱浜庡苟闈炴墍鏈夊唴鏍搁兘閬靛惊鐩稿悓鐨勯鏍硷紝浣犲彲鑳藉笇鏈涢拡瀵规煇涓壒瀹氱殑瀛愮郴缁熸垨鏂囦欢澶硅皟鏁撮粯璁ゅ€笺€備负姝わ紝浣犲彲浠ュ湪瀛愭枃浠跺す涓紪鍐欏彟涓€涓?`.clang-format` 鏂囦欢鏉ヨ鐩栭粯璁ゅ€笺€?

璇ュ伐鍏锋湰韬棭宸茶鍖呭惈鍦ㄦ祦琛岀殑 Linux 鍙戣鐗堜粨搴撲腑銆傝鍦ㄤ綘鐨勪粨搴撲腑鎼滅储 `clang-format`銆傚惁鍒欙紝浣犲彲浠ヤ笅杞介缂栬瘧鐨?LLVM/clang 浜岃繘鍒舵枃浠讹紝鎴栬€呬粠浠ヤ笅鍦板潃鏋勫缓婧愪唬鐮侊細

    https://releases.llvm.org/download.html

鏈夊叧璇ュ伐鍏风殑鏇村淇℃伅锛岃鍙傞槄锛?

    https://clang.llvm.org/docs/ClangFormat.html

    https://clang.llvm.org/docs/ClangFormatStyleOptions.html



### 瀹℃煡鏂囦欢鍜岃ˉ涓佺殑浠ｇ爜椋庢牸


閫氳繃浠ヨ鍐咃紙inline锛夋ā寮忚繍琛岃宸ュ叿锛屼綘鍙互瀹℃煡鏁翠釜瀛愮郴缁熴€佹枃浠跺す鎴栧崟涓枃浠剁殑浠ｇ爜椋庢牸閿欒銆佺瑪璇垨鏀硅繘涔嬪銆?

```

    # Make sure your working directory is clean!
    clang-format -i kernel/*.[ch]

```
鐒跺悗鏌ョ湅 git diff銆?

缁熻杩欑 diff 鐨勮鏁颁篃鏈夊姪浜庢敼杩?璋冩暣閰嶇疆鏂囦欢涓殑椋庢牸閫夐」锛涗互鍙婃祴璇曟柊鐨?`clang-format` 鐗规€?鐗堟湰銆?

`clang-format` 涔熸敮鎸佽鍙栫粺涓€ diff锛屽洜姝や綘鍙互杞绘澗鍦板鏌ヨˉ涓佸拰 git diff銆傝鍙傞槄鏂囨。锛?

    https://clang.llvm.org/docs/ClangFormat.html#script-for-patch-reformatting

```

    int formatted_code;
    // clang-format off
        void    unformatted_code  ;
    // clang-format on
    void formatted_code_again;

```
铏界劧浣跨敤瀹冩潵璁╂煇涓枃浠跺缁堜笌 `clang-format` 淇濇寔鍚屾鍙兘寰堣浜猴紝鐗瑰埆鏄綋浣犲湪缂栧啓鏂版枃浠舵垨鏄淮鎶よ€呮椂锛屼絾璇锋敞鎰忥紝鍏朵粬浜哄彲鑳借繍琛岀潃涓嶅悓鐗堟湰鐨?`clang-format`锛屾垨鑰呮牴鏈病鏈夎宸ュ叿銆傚洜姝わ紝浣犲彲鑳藉簲璇ラ伩鍏嶅湪鍐呮牳婧愮爜涓娇鐢ㄥ畠锛涜嚦灏戝湪鎴戜滑纭 `clang-format` 鏄惁鍙樺緱鏅強涔嬪墠銆?



### 閲嶆柊鏍煎紡鍖栦唬鐮佸潡


閫氳繃浣跨敤涓庢枃鏈紪杈戝櫒鐨勯泦鎴愶紝浣犲彲浠ョ敤涓€娆℃寜閿噸鏂版牸寮忓寲浠绘剰浠ｇ爜鍧楋紙閫夊尯锛夈€傝繖鍦ㄧЩ鍔ㄤ唬鐮併€佸鐞嗘繁搴︾缉杩涚殑澶嶆潅浠ｇ爜銆佸琛屽畯锛堜互鍙婂榻愬畠浠殑鍙嶆枩鏉狅級绛夋儏鍐典笅鐗瑰埆鏈夌敤銆?

璇疯浣忥紝鍦ㄥ伐鍏锋湭鑳藉畬缇庡鐞嗙殑閭ｄ簺鎯呭喌涓嬶紝浣犲缁堝彲浠ュ湪浜嬪悗璋冩暣杩欎簺淇敼銆備絾浣滀负鍒濇杩戜技锛屽畠浼氶潪甯告湁鐢ㄣ€?

璁稿娴佽鐨勬枃鏈紪杈戝櫒閮芥湁闆嗘垚鏀寔銆傚叾涓竴浜涳紙濡?vim銆乪macs銆丅BEdit 鍜?Visual Studio锛夊唴缃簡鏀寔銆傛湁鍏宠鏄庯紝璇烽槄璇伙細

    https://clang.llvm.org/docs/ClangFormat.html

瀵逛簬 Atom銆丒clipse銆丼ublime Text銆乂isual Studio Code銆乆Code 浠ュ強鍏跺畠缂栬緫鍣ㄥ拰 IDE锛屼綘搴旇鑳藉鎵惧埌鍗崇敤鍨嬫彃浠躲€?

瀵逛簬杩欑鐢ㄤ緥锛岃€冭檻浣跨敤涓€涓緟鍔╃殑 `.clang-format`锛屼互渚夸綘鍙互璋冩暣涓€浜涢€夐」銆傚弬瑙?clangformatextra_銆?



### 缂哄け鐨勬敮鎸?


`clang-format` 缂哄皯瀵瑰唴鏍镐唬鐮佷腑涓€浜涘父瑙佷笢瑗跨殑鏀寔銆傚畠浠緢瀹规槗璁颁綇锛屾墍浠ュ鏋滀綘缁忓父浣跨敤璇ュ伐鍏凤紝寰堝揩灏变細瀛︿細閬垮紑/蹇界暐瀹冧滑銆?

灏ゅ叾锛屼綘浼氭敞鎰忓埌涓€浜涢潪甯稿父瑙佺殑鎯呭喌锛?

```

        #define TRACING_MAP_BITS_DEFAULT       11
        #define TRACING_MAP_BITS_MAX           17
        #define TRACING_MAP_BITS_MIN           7

    vs.::

        #define TRACING_MAP_BITS_DEFAULT 11
        #define TRACING_MAP_BITS_MAX 17
        #define TRACING_MAP_BITS_MIN 7

  - Aligned designated initializers, e.g.::

        static const struct file_operations uprobe_events_ops = {
                .owner          = THIS_MODULE,
                .open           = probes_open,
                .read           = seq_read,
                .llseek         = seq_lseek,
                .release        = seq_release,
                .write          = probes_write,
        };

    vs.::

        static const struct file_operations uprobe_events_ops = {
                .owner = THIS_MODULE,
                .open = probes_open,
                .read = seq_read,
                .llseek = seq_lseek,
                .release = seq_release,
                .write = probes_write,
        };


```

### 棰濆鐨勭壒鎬?閫夐」


涓轰簡鍦ㄨ緭鍑轰笌褰撳墠浠ｇ爜涔嬮棿鐨勫樊寮傛渶灏忓寲锛岄厤缃枃浠朵腑榛樿娌℃湁鍚敤鏌愪簺鐗规€?椋庢牸閫夐」銆傛崲鍙ヨ瘽璇达紝涓轰簡璁╁樊寮傚敖鍙兘灏忥紝浠庤€屼娇鍏ㄦ枃浠堕鏍肩殑瀹℃煡浠ュ強 diff 鍜岃ˉ涓佺殑瀹℃煡灏藉彲鑳藉鏄撱€?

鍦ㄥ叾瀹冩儏鍐典笅锛堜緥濡傜壒瀹氱殑瀛愮郴缁?鏂囦欢澶?鏂囦欢锛夛紝鍐呮牳椋庢牸鍙兘鏈夋墍涓嶅悓锛屽惎鐢ㄥ叾涓竴浜涢€夐」鍙兘浼氭洿濂藉湴杩戜技閭ｉ噷鐨勯鏍笺€?

渚嬪锛?

  - 瀵归綈璧嬪€硷紙`AlignConsecutiveAssignments`锛夈€?

  - 瀵归綈澹版槑锛坄AlignConsecutiveDeclarations`锛夈€?

  - 閲嶆柊鎺掔増娉ㄩ噴涓殑鏂囨湰锛坄ReflowComments`锛夈€?

  - 鎺掑簭 `#includes`锛坄SortIncludes`锛夈€?

瀹冧滑閫氬父瀵逛唬鐮佸潡閲嶆柊鏍煎紡鍖栨洿鏈夌敤锛岃€岄潪鍏ㄦ枃浠躲€備綘鍙兘鎯冲垱寤哄彟涓€涓?`.clang-format` 鏂囦欢锛屽苟浠庝綘鐨勭紪杈戝櫒/IDE 涓敼鐢ㄥ畠銆?
