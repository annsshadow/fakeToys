## BPF_MAP_TYPE_DEVMAP 涓?BPF_MAP_TYPE_DEVMAP_HASH


   - `BPF_MAP_TYPE_DEVMAP` 鍦ㄥ唴鏍哥増鏈?4.14 涓紩鍏?   - `BPF_MAP_TYPE_DEVMAP_HASH` 鍦ㄥ唴鏍哥増鏈?5.4 涓紩鍏?
`BPF_MAP_TYPE_DEVMAP` 鍜?`BPF_MAP_TYPE_DEVMAP_HASH` 鏄富瑕佺敤浣?XDP BPF 杈呭姪璋冪敤 `bpf_redirect_map()` 鍚庣鏄犲皠鐨?BPF 鏄犲皠銆俙BPF_MAP_TYPE_DEVMAP` 鐢变竴涓暟缁勬敮鎾戯紝璇ユ暟缁勪娇鐢ㄩ敭锛坘ey锛変綔涓虹储寮曟潵鏌ユ壘瀵圭綉缁滆澶囷紙net device锛夌殑寮曠敤銆傝€?`BPF_MAP_TYPE_DEVMAP_HASH` 鐢变竴涓搱甯岃〃鏀拺锛岃鍝堝笇琛ㄤ娇鐢ㄩ敭鏉ユ煡鎵惧缃戠粶璁惧鐨勫紩鐢ㄣ€傜敤鎴锋彁渚?<`key`/ `ifindex`> 鎴?<`key`/ `struct bpf_devmap_val`> 瀵规潵鐢ㄦ柊鐨勭綉缁滆澶囨洿鏂版槧灏勩€?
    - 鍝堝笇鏄犲皠鐨勯敭涓嶅繀鏄?`ifindex`銆?    - 铏界劧 `BPF_MAP_TYPE_DEVMAP_HASH` 鍏佽瀵圭綉缁滆澶囪繘琛岀揣鍑戞墦鍖咃紝浣嗗叾浠ｄ环鏄湪鎵ц鏌ユ壘鏃堕渶瑕佸閿繘琛屽搱甯屻€?
涓ょ绫诲瀷 devmap 鐨勫垵濮嬪寲鍜屾暟鎹寘鍏ラ槦/鍙戦€佷唬鐮佹槸鍏变韩鐨勶紱鍙湁鏌ユ壘鍜屾彃鍏ヤ笉鍚屻€?
## 鐢ㄦ硶锛圲sage锛?

### 鍐呮牳 BPF


##### bpf_redirect_map()


    long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

灏嗘暟鎹寘閲嶅畾鍚戝埌 `map` 涓储寮曚负 `key` 鎵€寮曠敤鐨勭鐐广€傚浜?`BPF_MAP_TYPE_DEVMAP` 鍜?`BPF_MAP_TYPE_DEVMAP_HASH`锛岃鏄犲皠鍖呭惈瀵圭綉缁滆澶囷紙鐢ㄤ簬閫氳繃鍏朵粬绔彛杞彂鏁版嵁鍖咃級鐨勫紩鐢ㄣ€?
**flags** 鐨勪綆涓や綅鐢ㄤ綔鏄犲皠鏌ユ壘澶辫触鏃剁殑杩斿洖鐮併€傝繖鏍疯繑鍥炲€煎彲浠ユ槸璋冪敤鑰呮墍閫夌殑銆佹渶楂樺埌 `XDP_TX` 鐨?XDP 绋嬪簭杩斿洖鐮佷箣涓€銆俙flags` 鐨勯珮浣嶅彲浠ヨ缃负 `BPF_F_BROADCAST` 鎴?`BPF_F_EXCLUDE_INGRESS`锛屽涓嬫墍杩般€?
浣跨敤 `BPF_F_BROADCAST` 鏃讹紝鏁版嵁鍖呭皢琚箍鎾埌鏄犲皠涓殑鎵€鏈夋帴鍙ｏ紱浣跨敤 `BPF_F_EXCLUDE_INGRESS` 鏃讹紝ingress 鎺ュ彛灏嗚鎺掗櫎鍦ㄥ箍鎾箣澶栥€?
    - 濡傛灉璁剧疆浜?BPF_F_BROADCAST锛屽垯閿蹇界暐銆?    - 骞挎挱鐗规€т篃鍙敤浜庡疄鐜扮粍鎾浆鍙戯細鍙渶鍒涘缓澶氫釜 DEVMAP锛屾瘡涓搴斾竴涓粍鎾粍銆?
璇ヨ緟鍔╁嚱鏁板湪鎴愬姛鏃惰繑鍥?`XDP_REDIRECT`锛岃嫢鏄犲皠鏌ユ壘澶辫触鍒欒繑鍥?`flags` 鍙傛暟鐨勪綆涓や綅鍊笺€?
鍏充簬閲嶅畾鍚戠殑鏇村淇℃伅鍙弬瑙?[redirect](redirect)

##### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鑾峰彇缃戠粶璁惧鏉＄洰銆?
### 鐢ㄦ埛绌洪棿


    DEVMAP 鏉＄洰鍙兘浠庣敤鎴风┖闂存洿鏂?鍒犻櫎锛岃€屼笉鑳戒粠 eBPF 绋嬪簭涓洿鏂?鍒犻櫎銆?    灏濊瘯浠庡唴鏍?eBPF 绋嬪簭璋冪敤杩欎簺鍑芥暟灏嗗鑷寸▼搴忓姞杞藉け璐ュ苟鍑虹幇楠岃瘉鍣紙verifier锛夎鍛娿€?
##### bpf_map_update_elem()


   int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags);

鍙互浣跨敤 `bpf_map_update_elem()` 杈呭姪鍑芥暟娣诲姞鎴栨洿鏂扮綉缁滆澶囨潯鐩€傝杈呭姪鍑芥暟浠ュ師瀛愭柟寮忔浛鎹㈢幇鏈夊厓绱犮€俙value` 鍙傛暟鍙互鏄?`struct bpf_devmap_val`锛屾垨鑰呬负浜嗗悜鍚庡吋瀹癸紝涔熷彲浠ユ槸涓€涓畝鍗曠殑 `int ifindex`銆?
 .. code-block:: c

    struct bpf_devmap_val {
        __u32 ifindex;   /** 璁惧绱㈠紩 **/
        union {
            int   fd;  /** 鍐欐槧灏勬椂鐨?prog fd **/
            __u32 id;  /** 璇绘槧灏勬椂鐨?prog id **/
        } bpf_prog;
    };

`flags` 鍙傛暟鍙互鏄互涓嬩箣涓€锛?  - `BPF_ANY`锛氬垱寤烘柊鍏冪礌鎴栨洿鏂扮幇鏈夊厓绱犮€?  - `BPF_NOEXIST`锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓鏂板厓绱犮€?  - `BPF_EXIST`锛氭洿鏂扮幇鏈夊厓绱犮€?
DEVMAP 鍙互閫氳繃灏?`bpf_prog.fd` 娣诲姞鍒?`struct bpf_devmap_val` 鏉ュ皢绋嬪簭涓庤澶囨潯鐩叧鑱斻€傜▼搴忓湪 `XDP_REDIRECT` 涔嬪悗杩愯锛屽苟涓斿彲浠ュ悓鏃惰闂?Rx 璁惧鍜?Tx 璁惧銆備笌 `fd` 鍏宠仈鐨勭▼搴忓繀椤诲叿鏈夌被鍨?XDP 涓旀湡鏈涢檮鍔犵被鍨嬩负 `xdp_devmap`銆傚綋绋嬪簭涓庤澶囩储寮曞叧鑱旀椂锛岀▼搴忓湪 `XDP_REDIRECT` 鏃躲€佸苟涓斿湪璇ョ紦鍐插尯琚姞鍏ユ瘡 CPU 闃熷垪涔嬪墠杩愯銆傚浣曢檮鍔?浣跨敤 xdp_devmap 绋嬪簭鐨勭ず渚嬪彲浠ュ湪鍐呮牳鑷祴涓壘鍒帮細

- `tools/testing/selftests/bpf/prog_tests/xdp_devmap_attach.c`
- `tools/testing/selftests/bpf/progs/test_xdp_with_devmap_helpers.c`

##### bpf_map_lookup_elem()


   int bpf_map_lookup_elem(int fd, const void **key, void **value);

鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鑾峰彇缃戠粶璁惧鏉＄洰銆?
##### bpf_map_delete_elem()


   int bpf_map_delete_elem(int fd, const void *key);

鍙互浣跨敤 `bpf_map_delete_elem()` 杈呭姪鍑芥暟鍒犻櫎缃戠粶璁惧鏉＄洰銆傝杈呭姪鍑芥暟鍦ㄦ垚鍔熸椂杩斿洖 0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
## 绀轰緥锛圗xamples锛?

### 鍐呮牳 BPF


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓悕涓?tx_port 鐨?`BPF_MAP_TYPE_DEVMAP`銆?

    struct {
        __uint(type, BPF_MAP_TYPE_DEVMAP);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 256);
    } tx_port SEC(".maps");

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓悕涓?forward_map 鐨?`BPF_MAP_TYPE_DEVMAP_HASH`銆?

    struct {
        __uint(type, BPF_MAP_TYPE_DEVMAP_HASH);
        __type(key, __u32);
        __type(value, struct bpf_devmap_val);
        __uint(max_entries, 32);
    } forward_map SEC(".maps");


    DEVMAP 涓笂杩扮殑鍊肩被鍨嬫槸 `struct bpf_devmap_val`

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓畝鍗曠殑 xdp_redirect_map 绋嬪簭銆傝绋嬪簭浼氶厤鍚堜竴涓敤鎴风┖闂寸▼搴忓伐浣滐紝璇ョ▼搴忓熀浜?ingress ifindex 濉厖 devmap `forward_map`銆侭PF 绋嬪簭锛堝涓嬶級浣跨敤 ingress `ifindex` 浣滀负 `key` 鏉ラ噸瀹氬悜鏁版嵁鍖呫€?

    SEC("xdp")
    int xdp_redirect_map_func(struct xdp_md *ctx)
    {
        int index = ctx->ingress_ifindex;

        return bpf_redirect_map(&forward_map, index, 0);
    }

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓皢鏁版嵁鍖呭箍鎾埌 `tx_port` devmap 涓墍鏈夋帴鍙ｇ殑 BPF 绋嬪簭銆?

    SEC("xdp")
    int xdp_redirect_map_func(struct xdp_md *ctx)
    {
        return bpf_redirect_map(&tx_port, 0, BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }

### 鐢ㄦ埛绌洪棿


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曟洿鏂颁竴涓悕涓?`tx_port` 鐨?devmap銆?

    int update_devmap(int ifindex, int redirect_ifindex)
    {
        int ret;

        ret = bpf_map_update_elem(bpf_map__fd(tx_port), &ifindex, &redirect_ifindex, 0);
        if (ret < 0) {
            fprintf(stderr, "Failed to update devmap_ value: %s\n",
                strerror(errno));
        }

        return ret;
    }

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曟洿鏂颁竴涓悕涓?`forward_map` 鐨?hash_devmap銆?

    int update_devmap(int ifindex, int redirect_ifindex)
    {
        struct bpf_devmap_val devmap_val = { .ifindex = redirect_ifindex };
        int ret;

        ret = bpf_map_update_elem(bpf_map__fd(forward_map), &ifindex, &devmap_val, 0);
        if (ret < 0) {
            fprintf(stderr, "Failed to update devmap_ value: %s\n",
                strerror(errno));
        }
        return ret;
    }

## 鍙傝€冿紙References锛?

- https://lwn.net/Articles/728146/
- https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/commit/?id=6f9d451ab1a33728adb72d7ff66a7b374d665176
- https://elixir.bootlin.com/linux/latest/source/net/core/filter.c#L4106
