
## BPF_MAP_TYPE_CPUMAP


   - `BPF_MAP_TYPE_CPUMAP` 鍦?4.15 鐗堟湰鐨勫唴鏍镐腑琚紩鍏?
 :doc: cpu map

杩欑鏄犲皠绫诲瀷鐨勪竴涓ず渚嬬敤渚嬫槸鍩轰簬杞欢鐨勬帴鏀剁缂╂斁锛圧eceive Side Scaling锛孯SS锛夈€?
CPUMAP 琛ㄧず绯荤粺涓殑 CPU锛屼互 map-key 涓虹储寮曪紝鑰?map-value 鏄厤缃缃紙姣忎釜 CPUMAP 鏉＄洰锛夈€傛瘡涓?CPUMAP 鏉＄洰閮芥湁涓€涓笓鐢ㄤ簬
缁欏畾 CPU 鐨勫唴鏍哥嚎绋嬶紝鐢ㄤ互琛ㄧず杩滅▼ CPU 鎵ц鍗曞厓銆?
浠?Linux 鍐呮牳 5.9 鐗堟湰寮€濮嬶紝CPUMAP 鍙互鍦ㄨ繙绋?CPU 涓?杩愯绗簩涓?XDP 绋嬪簭銆傝繖鍏佽 XDP 绋嬪簭灏嗗叾澶勭悊鎷嗗垎鍒?澶氫釜 CPU 涓娿€備緥濡傦紝杩欐牱涓€绉嶅満鏅細鍒濆 CPU锛堢湅鍒?鎺ユ敹
鏁版嵁鍖呯殑閭ｄ釜锛夊彧闇€鍋氭渶灏戠殑鍖呭鐞嗭紝鑰岃繙绋?CPU锛堟暟鎹寘
琚畾鍚戝埌鐨勯偅涓級鍙互鑺辫垂鏇村鍛ㄦ湡鏉ュ鐞嗚甯с€傚垵濮?CPU
鏄墽琛?XDP redirect 绋嬪簭鐨勫湴鏂广€傝繙绋?CPU 鎺ユ敹鍘熷鐨?`xdp_frame` 瀵硅薄銆?
## 鐢ㄦ硶


### 鍐呮牳 BPF


##### bpf_redirect_map()


     long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

灏嗘暟鎹寘閲嶅畾鍚戝埌 `map` 涓储寮曚负 `key` 鐨勭鐐广€?瀵逛簬 `BPF_MAP_TYPE_CPUMAP`锛岃鏄犲皠鍖呭惈瀵?CPU 鐨勫紩鐢ㄣ€?
濡傛灉鏄犲皠鏌ユ壘澶辫触锛宍flags` 鐨勪綆浣嶄袱浣嶅皢鐢ㄤ綔杩斿洖鐮併€?杩欐牱杩斿洖鍊煎彲浠ユ槸璋冪敤鑰呮墍閫夌殑銆佹渶楂樺埌 `XDP_TX` 鐨?XDP 绋嬪簭杩斿洖鐮佷箣涓€銆?
### 鐢ㄦ埛绌洪棿


    CPUMAP 鏉＄洰鍙兘浠庣敤鎴风┖闂存洿鏂?鏌ユ壘/鍒犻櫎锛岃€屼笉鑳?    浠?eBPF 绋嬪簭涓繘琛屻€傝瘯鍥句粠鍐呮牳 eBPF 绋嬪簭璋冪敤杩欎簺鍑芥暟
    灏嗗鑷寸▼搴忓姞杞藉け璐ュ苟鍑虹幇楠岃瘉鍣紙verifier锛夎鍛娿€?
##### bpf_map_update_elem()


    int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags);

鍙互浣跨敤 `bpf_map_update_elem()` helper 娣诲姞鎴栨洿鏂?CPU 鏉＄洰銆傝 helper 浠ュ師瀛愭柟寮忔浛鎹㈢幇鏈夊厓绱犮€俙value` 鍙傛暟
鍙互鏄?`struct bpf_cpumap_val`銆?
 .. code-block:: c

    struct bpf_cpumap_val {
        __u32 qsize;  /** queue size to remote target CPU **/
        union {
            int   fd; /** prog fd on map write **/
            __u32 id; /** prog id on map read **/
        } bpf_prog;
    };

flags 鍙傛暟鍙互鏄互涓嬩箣涓€锛?  - BPF_ANY锛氬垱寤轰竴涓柊鍏冪礌鎴栨洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?  - BPF_NOEXIST锛氫粎褰撳厓绱犱笉瀛樺湪鏃舵墠鍒涘缓涓€涓柊鍏冪礌銆?  - BPF_EXIST锛氭洿鏂颁竴涓凡瀛樺湪鐨勫厓绱犮€?
##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value);

鍙互浣跨敤 `bpf_map_lookup_elem()` helper 妫€绱?CPU 鏉＄洰銆?
##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key);

鍙互浣跨敤 `bpf_map_delete_elem()` helper 鍒犻櫎
CPU 鏉＄洰銆傛垚鍔熸椂璇?helper 杩斿洖 0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
## 绀轰緥


### 鍐呮牳


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓悕涓?`cpu_map` 鐨?`BPF_MAP_TYPE_CPUMAP`锛屼互鍙婂浣曚娇鐢ㄨ疆璇紙round robin锛夋柟妗?灏嗘暟鎹寘閲嶅畾鍚戝埌杩滅▼ CPU銆?

   struct {
        __uint(type, BPF_MAP_TYPE_CPUMAP);
        __type(key, __u32);
        __type(value, struct bpf_cpumap_val);
        __uint(max_entries, 12);
    } cpu_map SEC(".maps");

    struct {
        __uint(type, BPF_MAP_TYPE_ARRAY);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 12);
    } cpus_available SEC(".maps");

    struct {
        __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 1);
    } cpus_iterator SEC(".maps");

    SEC("xdp")
    int  xdp_redir_cpu_round_robin(struct xdp_md *ctx)
    {
        __u32 key = 0;
        __u32 cpu_dest = 0;
        __u32 **cpu_selected, **cpu_iterator;
        __u32 cpu_idx;

        cpu_iterator = bpf_map_lookup_elem(&cpus_iterator, &key);
        if (!cpu_iterator)
            return XDP_ABORTED;
        cpu_idx = *cpu_iterator;

        *cpu_iterator += 1;
        if (*cpu_iterator == bpf_num_possible_cpus())
            *cpu_iterator = 0;

        cpu_selected = bpf_map_lookup_elem(&cpus_available, &cpu_idx);
        if (!cpu_selected)
            return XDP_ABORTED;
        cpu_dest = *cpu_selected;

        if (cpu_dest >= bpf_num_possible_cpus())
            return XDP_ABORTED;

        return bpf_redirect_map(&cpu_map, cpu_dest, 0);
    }

### 鐢ㄦ埛绌洪棿


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞皢 CPUMAP 鐨?max_entries
鍔ㄦ€佽缃负绯荤粺涓婂彲鐢ㄧ殑 CPU 鏈€澶ф暟閲忋€?

    int set_max_cpu_entries(struct bpf_map *cpu_map)
    {
        if (bpf_map__set_max_entries(cpu_map, libbpf_num_possible_cpus()) < 0) {
            fprintf(stderr, "Failed to set max entries for cpu_map map: %s",
                strerror(errno));
            return -1;
        }
        return 0;
    }

## 鍙傝€?

- https://developers.redhat.com/blog/2021/05/13/receive-side-scaling-rss-with-ebpf-and-cpumap#redirecting_into_a_cpumap
