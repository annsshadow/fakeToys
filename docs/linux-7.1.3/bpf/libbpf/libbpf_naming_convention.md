
## API 鍛藉悕绾﹀畾

libbpf API 鎻愪緵瀵瑰嚑缁勫湪閫昏緫涓婄浉浜掔嫭绔嬬殑鍑芥暟鍜岀被鍨嬬殑璁块棶銆傛瘡涓€缁勯兘鏈夎嚜宸卞湪杩欓噷
鎻忚堪鐨勫懡鍚嶇害瀹氥€傚缓璁湪娣诲姞鏂板嚱鏁版垨绫诲瀷鏃堕伒寰繖浜涚害瀹氾紝浠ヤ繚鎸?libbpf API 鐨?鏁存磥涓庝竴鑷淬€?
libbpf API 鎻愪緵鐨勬墍鏈夌被鍨嬪拰鍑芥暟閮藉簲甯︽湁浠ヤ笅鍓嶇紑涔嬩竴锛歚bpf_`銆乣btf_`銆乣libbpf_`銆?`btf_dump_`銆乣ring_buffer_`銆乣perf_buffer_`銆?
### 绯荤粺璋冪敤灏佽

绯荤粺璋冪敤灏佽鏄 sys_bpf 绯荤粺璋冪敤鎵€鏀寔鍛戒护鐨勭畝鍗曞皝瑁呫€傝繖浜涘皝瑁呭簲褰撴斁鍏?`bpf.h`
澶存枃浠讹紝骞朵笌鐩稿簲鍛戒护涓€涓€瀵瑰簲銆?
渚嬪锛宍bpf_map_lookup_elem` 灏佽浜?sys_bpf 鐨?`BPF_MAP_LOOKUP_ELEM` 鍛戒护锛?`bpf_prog_attach` 灏佽浜?`BPF_PROG_ATTACH`锛岀瓑绛夈€?
### 瀵硅薄

libbpf API 鎻愪緵鐨勫彟涓€绫荤被鍨嬪拰鍑芥暟鏄?瀵硅薄"浠ュ強鐢ㄤ簬澶勭悊瀹冧滑鐨勫嚱鏁般€傚璞℃槸楂樼骇
鎶借薄锛屼緥濡?BPF 绋嬪簭鎴?BPF map銆傚畠浠敱鐩稿簲鐨勭粨鏋勪綋琛ㄧず锛屼緥濡?`struct bpf_object`銆?`struct bpf_program`銆乣struct bpf_map` 绛夈€?
缁撴瀯浣撻噰鐢ㄥ墠鍚戝０鏄庯紝瀵瑰叾瀛楁鐨勮闂簲褰撻€氳繃鐩稿簲鐨?getter 鍜?setter 鎻愪緵锛岃€屼笉鏄洿鎺?璁块棶銆?
杩欎簺瀵硅薄涓庡寘鍚凡缂栬瘧 BPF 绋嬪簭鐨?ELF 瀵硅薄鐨勭浉搴旈儴鍒嗙浉鍏宠仈銆?
渚嬪锛宍struct bpf_object` 琛ㄧず浠庝竴涓?ELF 鏂囦欢鎴栫紦鍐插尯鍒涘缓鐨?ELF 瀵硅薄鏈韩锛?`struct bpf_program` 琛ㄧず ELF 瀵硅薄涓殑涓€涓▼搴忥紝`struct bpf_map` 琛ㄧず涓€涓?map銆?
澶勭悊瀵硅薄鐨勫嚱鏁扮殑鍛藉悕鐢卞璞″悕銆佸弻涓嬪垝绾垮拰鎻忚堪鍑芥暟鐢ㄩ€旂殑閮ㄥ垎缁勬垚銆?
渚嬪锛宍bpf_object__open` 鐢辩浉搴斿璞＄殑鍚嶇О `bpf_object`銆佸弻涓嬪垝绾垮拰 `open` 缁勬垚锛?鍚庤€呭畾涔変簡璇ュ嚱鏁?鎵撳紑 ELF 鏂囦欢骞朵粠涓垱寤?`bpf_object`"鐨勭敤閫斻€?
闄や笌 BTF 鐩稿叧鐨勫璞″锛屾墍鏈夊璞″強鐩稿簲鍑芥暟閮藉簲鏀惧叆 `libbpf.h`銆侭TF 绫诲瀷鍜屽嚱鏁板簲
鏀惧叆 `btf.h`銆?
### 杈呭姪鍑芥暟

涓嶉€傚悎涓婅堪浠讳綍绫诲埆鐨勮緟鍔╁嚱鏁板拰绫诲瀷搴斿綋甯︽湁 `libbpf_` 鍓嶇紑锛屼緥濡?`libbpf_get_error` 鎴?`libbpf_prog_type_by_name`銆?
### ABI

libbpf 鏃㈠彲浠ヨ闈欐€侀摼鎺ワ紝涔熷彲浠ヤ綔涓?DSO 浣跨敤銆備负浜嗛伩鍏嶄笌搴旂敤绋嬪簭閾炬帴鐨勫叾浠栧簱
鍙兘鍙戠敓鐨勫啿绐侊紝鎵€鏈夐潪闈欐€佺殑 libbpf 绗﹀彿閮藉簲甯︽湁涓婇潰 API 鏂囨。涓彁鍒扮殑鏌愪釜鍓嶇紑銆?鍙傝 API 鍛藉悕绾﹀畾锛屼负鏂扮鍙烽€夋嫨鍚堥€傜殑鍚嶇О銆?
### 绗﹀彿鍙鎬?
libbpf 閬靛惊杩欐牱鐨勬ā鍨嬶細榛樿鎯呭喌涓嬫墍鏈夊叏灞€绗﹀彿鐨勫彲瑙佹€т负 "hidden"锛岃浣夸竴涓鍙?鍙锛屽繀椤荤敤 `LIBBPF_API` 瀹忔樉寮忔爣娉ㄣ€備緥濡傦細


        LIBBPF_API int bpf_prog_get_fd_by_id(__u32 id);

杩欐牱鍙互闃叉鎰忓瀵煎嚭涓€涓湰涓嶅簲鎴愪负 ABI 涓€閮ㄥ垎鐨勭鍙凤紝浠庤€屾敼鍠?libbpf 寮€鍙戣€呭拰
鐢ㄦ埛鐨勪綋楠屻€?
### ABI 鐗堟湰鎺у埗

涓轰簡浣挎湭鏉ョ殑 ABI 鎵╁睍鎴愪负鍙兘锛宭ibbpf 鐨?ABI 杩涜浜嗙増鏈帶鍒躲€傜増鏈帶鍒堕€氳繃浼犻€掔粰
閾炬帴鍣ㄧ殑 `libbpf.map` 鐗堟湰鑴氭湰瀹炵幇銆?
鐗堟湰鍚嶄负 `LIBBPF_` 鍓嶇紑 + 涓夋寮忔暟瀛楃増鏈紝浠?`0.0.1` 寮€濮嬨€?
姣忓綋 ABI 鍙戠敓鍙樻洿锛堜緥濡傛柊澧炰簡涓€涓鍙凤紝鎴栬€呯幇鏈夌鍙风殑璇箟鍙戠敓浜嗘敼鍙橈級锛屽氨搴斿綋
鎻愬崌 ABI 鐗堟湰銆傛瘡涓唴鏍稿紑鍙戝懆鏈熸渶澶氭彁鍗囦竴娆?ABI 鐗堟湰銆?
渚嬪锛屽鏋?`libbpf.map` 鐨勫綋鍓嶇姸鎬佹槸锛?

        LIBBPF_0.0.1 {
        	global:
                        bpf_func_a;
                        bpf_func_b;
        	local:
        		\*;
        };

锛屽苟涓旇寮曞叆涓€涓柊鐨勭鍙?`bpf_func_c`锛岄偅涔?`libbpf.map` 搴斿綋杩欐牱淇敼锛?

        LIBBPF_0.0.1 {
        	global:
                        bpf_func_a;
                        bpf_func_b;
        	local:
        		\*;
        };
        LIBBPF_0.0.2 {
                global:
                        bpf_func_c;
        } LIBBPF_0.0.1;

锛屽叾涓柊鐗堟湰 `LIBBPF_0.0.2` 渚濊禆浜庡厛鍓嶇殑 `LIBBPF_0.0.1`銆?
鐗堟湰鑴氭湰鐨勬牸寮忎互鍙婂鐞?ABI 鍙樻洿锛堝寘鎷笉鍏煎鐨勫彉鏇达級鐨勬柟寮忥紝鍦?[^1^] 涓湁璇︾粏鎻忚堪銆?
### 鐙珛鏋勫缓

鍦?https://github.com/libbpf/libbpf 澶勬湁涓€涓敤浜庣嫭绔嬫瀯寤虹殑 libbpf 涓荤嚎鐗堟湰
锛堝崐鑷姩锛夐暅鍍忋€?
浣嗘槸锛屽 libbpf 浠ｇ爜搴撶殑鎵€鏈夋洿鏀归兘蹇呴』閫氳繃涓荤嚎鍐呮牳鏍戝悜涓婃父鎻愪氦銆?
## API 鏂囨。绾﹀畾

libbpf API 閫氳繃澶存枃浠朵腑瀹氫箟涓婃柟鐨勬敞閲婅繘琛屾枃妗ｅ寲銆傝繖浜涙敞閲婂彲浠ヨ doxygen 鍜?sphinx 娓叉煋涓虹粍缁囪壇濂界殑 html 杈撳嚭銆傛湰鑺傛弿杩拌繖浜涙敞閲婂簲褰撻噰鐢ㄧ殑鏍煎紡绾﹀畾銆?
浠ヤ笅鏄潵鑷?btf.h 鐨勪竴涓緥瀛愶細


        /**
         - @brief **btf__new()** creates a new instance of a BTF object from the raw
         - bytes of an ELF's BTF section
         - @param data raw bytes
         - @param size number of bytes passed in `data`
         - @return new BTF object instance which has to be eventually freed with
         - **btf__free()**
         *
         - On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
         - error code from such a pointer `libbpf_get_error()` should be used. If
         - `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
         - returned on error instead. In both cases thread-local `errno` variable is
         - always set to error code as well.
         */

娉ㄩ噴蹇呴』浠?'/\**\**' 褰㈠紡鐨勫潡娉ㄩ噴寮€濮嬨€?
鏂囨。鎬绘槸浠?@brief 鎸囦护寮€濮嬨€傝繖涓€琛屾槸瀵硅 API 鐨勭畝鐭弿杩般€傚畠浠?API 鐨勫悕绉板紑澶达紝鐢?绮椾綋琛ㄧず锛屽锛?*api_name**銆傚鏋滆繖鏄竴涓嚱鏁帮紝璇峰寘鍚竴瀵瑰乏鍙冲渾鎷彿銆傞殢鍚庤窡涓?璇?API 鐨勭畝鐭弿杩般€傛洿闀跨殑鎻忚堪鍙互鍔犲湪鏈€鍚庝竴涓寚浠や笅鏂广€佹敞閲婄殑搴曢儴銆?
鍙傛暟鐢?@param 鎸囦护琛ㄧず锛屾瘡涓弬鏁伴兘搴旀湁涓€涓€傚鏋滆繖鏄竴涓叿鏈夐潪 void 杩斿洖鍊肩殑鍑芥暟锛?璇蜂娇鐢?@return 鎸囦护鏉ヨ褰曞畠銆?
### 璁稿彲璇?
libbpf 閲囩敤 LGPL 2.1 鍜?BSD 2-Clause 鍙岄噸璁稿彲銆?
### 閾炬帴

[^1^] https://www.akkadia.org/drepper/dsohowto.pdf
    (Chapter 3. Maintaining APIs and ABIs).
