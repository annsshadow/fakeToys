
## BPF_MAP_TYPE_ARRAY and BPF_MAP_TYPE_PERCPU_ARRAY


   - `BPF_MAP_TYPE_ARRAY` 鍦?kernel 3.19 鐗堟湰涓紩鍏?   - `BPF_MAP_TYPE_PERCPU_ARRAY` 鍦?4.6 鐗堟湰涓紩鍏?
`BPF_MAP_TYPE_ARRAY` 鍜?`BPF_MAP_TYPE_PERCPU_ARRAY` 鎻愪緵閫氱敤鐨勬暟缁勫瓨鍌ㄣ€傞敭绫诲瀷鏄棤绗﹀彿 32 浣嶆暣鏁帮紙4 瀛楄妭锛夛紝鏄犲皠鐨勫ぇ灏忔亽瀹氥€俙max_entries` 鍦ㄥ垱寤烘椂瀹氫箟浜嗘暟缁勭殑澶у皬銆傛墍鏈夋暟缁勫厓绱犲湪鍒涘缓鏃堕兘浼氳棰勫垎閰嶅苟闆跺垵濮嬪寲銆俙BPF_MAP_TYPE_PERCPU_ARRAY` 涓烘瘡涓?CPU 浣跨敤涓嶅悓鐨勫唴瀛樺尯鍩燂紝鑰?`BPF_MAP_TYPE_ARRAY` 浣跨敤鐩稿悓鐨勫唴瀛樺尯鍩熴€傚浜?`BPF_MAP_TYPE_ARRAY`锛屾墍瀛樺偍鐨勫€煎彲浠ユ槸浠绘剰澶у皬锛涜€屽浜?`BPF_MAP_TYPE_PERCPU_ARRAY`锛屽叾鍊间笉寰楀ぇ浜?`PCPU_MIN_UNIT_SIZE`锛?2 kB锛夈€傛墍鏈夋暟缁勫厓绱犻兘鎸?8 瀛楄妭瀵归綈銆?
鑷?kernel 5.5 璧凤紝鍙€氳繃璁剧疆 `BPF_F_MMAPABLE` 鏍囧織涓?`BPF_MAP_TYPE_ARRAY` 鍚敤鍐呭瓨鏄犲皠銆傛槧灏勫畾涔夋寜椤靛榻愶紝骞朵粠绗竴椤靛紑濮嬨€傜郴缁熶細鍒嗛厤瓒冲鏁伴噺鐨勯〉澶у皬銆侀〉瀵归綈鐨勫唴瀛樺潡锛堜粠绗簩椤靛紑濮嬶級鏉ュ瓨鍌ㄦ墍鏈夋暟缁勫€硷紝鍦ㄦ煇浜涙儏鍐典笅杩欎細瀵艰嚧鍐呭瓨鐨勮繃搴﹀垎閰嶃€傝繖鏍峰仛鐨勫ソ澶勬槸鎻愬崌浜嗘€ц兘骞剁畝鍖栦簡浣跨敤锛屽洜涓虹敤鎴风┖闂寸▼搴忔棤闇€浣跨敤杈呭姪鍑芥暟鏉ヨ闂拰淇敼鏁版嵁銆?
## Usage


### Kernel BPF


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

鏁扮粍鍏冪礌鍙娇鐢?`bpf_map_lookup_elem()` 杈呭姪鍑芥暟妫€绱€傝杈呭姪鍑芥暟杩斿洖鎸囧悜鏁扮粍鍏冪礌鐨勬寚閽堬紝鍥犳涓洪伩鍏嶄笌璇诲彇璇ュ€肩殑鐢ㄦ埛绌洪棿鍙戠敓鏁版嵁绔炰簤锛岀敤鎴峰湪鍘熷湴鏇存柊璇ュ€兼椂搴斾娇鐢?`__sync_fetch_and_add()` 涔嬬被鐨勫師璇€?
#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

鏁扮粍鍏冪礌鍙娇鐢?`bpf_map_update_elem()` 杈呭姪鍑芥暟鏇存柊銆?
`bpf_map_update_elem()` 鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
鐢变簬鏁扮粍澶у皬鎭掑畾锛屽洜姝や笉鏀寔 `bpf_map_delete_elem()`銆傝娓呯┖鏌愪釜鏁扮粍鍏冪礌锛屽彲浠ヤ娇鐢?`bpf_map_update_elem()` 鍚戣绱㈠紩鎻掑叆涓€涓浂鍊笺€?
### Per CPU Array


`BPF_MAP_TYPE_ARRAY` 涓瓨鍌ㄧ殑鍊煎彲浠ヨ涓嶅悓 CPU 涓婄殑澶氫釜绋嬪簭璁块棶銆傝灏嗗瓨鍌ㄩ檺鍒跺埌鍗曚釜 CPU锛屽彲浠ヤ娇鐢?`BPF_MAP_TYPE_PERCPU_ARRAY`銆?
浣跨敤 `BPF_MAP_TYPE_PERCPU_ARRAY` 鏃讹紝`bpf_map_update_elem()` 鍜?`bpf_map_lookup_elem()` 杈呭姪鍑芥暟浼氳嚜鍔ㄨ闂綋鍓?CPU 瀵瑰簲鐨勬Ы浣嶃€?
#### bpf_map_lookup_percpu_elem()


   void **bpf_map_lookup_percpu_elem(struct bpf_map **map, const void *key, u32 cpu)

`bpf_map_lookup_percpu_elem()` 杈呭姪鍑芥暟鍙敤浜庢煡鎵剧壒瀹?CPU 鐨勬暟缁勫€笺€傛垚鍔熸椂杩斿洖鍊硷紝濡傛灉鏈壘鍒板搴旀潯鐩垨 `cpu` 鏃犳晥鍒欒繑鍥?`NULL`銆?
### Concurrency


鑷?kernel 5.1 鐗堟湰璧凤紝BPF 鍩虹璁炬柦鎻愪緵 `struct bpf_spin_lock` 鏉ュ悓姝ヨ闂€?
### Userspace


鐢ㄦ埛绌洪棿鐨勮闂娇鐢ㄤ笌涓婅堪鍚屽悕鐨?libbpf API锛屾槧灏勯€氳繃鍏?`fd` 鏍囪瘑銆?
## Examples


鍔熻兘绀轰緥璇峰弬瑙?`tools/testing/selftests/bpf` 鐩綍銆備笅闈㈢殑浠ｇ爜绀轰緥婕旂ず浜?API 鐨勭敤娉曘€?
### Kernel BPF


姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍦?BPF 绋嬪簭涓０鏄庝竴涓暟缁勩€?

    struct {
            __uint(type, BPF_MAP_TYPE_ARRAY);
            __type(key, u32);
            __type(value, long);
            __uint(max_entries, 256);
    } my_map SEC(".maps");


姝ょず渚?BPF 绋嬪簭灞曠ず浜嗗浣曡闂暟缁勫厓绱犮€?

    int bpf_prog(struct __sk_buff *skb)
    {
            struct iphdr ip;
            int index;
            long *value;

            if (bpf_skb_load_bytes(skb, ETH_HLEN, &ip, sizeof(ip)) < 0)
                    return 0;

            index = ip.protocol;
            value = bpf_map_lookup_elem(&my_map, &index);
            if (value)
                    __sync_fetch_and_add(value, skb->len);

            return 0;
    }

### Userspace


#### BPF_MAP_TYPE_ARRAY


姝や唬鐮佺墖娈靛睍绀轰簡濡備綍浣跨敤 `bpf_map_create_opts` 璁剧疆鏍囧織鏉ュ垱寤轰竴涓暟缁勩€?

    #include <bpf/libbpf.h>
    #include <bpf/bpf.h>

    int create_array()
    {
            int fd;
            LIBBPF_OPTS(bpf_map_create_opts, opts, .map_flags = BPF_F_MMAPABLE);

            fd = bpf_map_create(BPF_MAP_TYPE_ARRAY,
                                "example_array",       /** name **/
                                sizeof(__u32),         /** key size **/
                                sizeof(long),          /** value size **/
                                256,                   /** max entries **/
                                &opts);                /** create opts **/
            return fd;
    }

姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍒濆鍖栨暟缁勭殑鍏冪礌銆?

    int initialize_array(int fd)
    {
            __u32 i;
            long value;
            int ret;

            for (i = 0; i < 256; i++) {
                    value = i;
                    ret = bpf_map_update_elem(fd, &i, &value, BPF_ANY);
                    if (ret < 0)
                            return ret;
            }

            return ret;
    }

姝や唬鐮佺墖娈靛睍绀轰簡濡備綍浠庢暟缁勪腑妫€绱㈠厓绱犲€笺€?

    int lookup(int fd)
    {
            __u32 index = 42;
            long value;
            int ret;

            ret = bpf_map_lookup_elem(fd, &index, &value);
            if (ret < 0)
                    return ret;

            /** use value here **/
            assert(value == 42);

            return ret;
    }

#### BPF_MAP_TYPE_PERCPU_ARRAY


姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍒濆鍖栨瘡 CPU 鏁扮粍鐨勫厓绱犮€?

    int initialize_array(int fd)
    {
            int ncpus = libbpf_num_possible_cpus();
            long values[ncpus];
            __u32 i, j;
            int ret;

            for (i = 0; i < 256 ; i++) {
                    for (j = 0; j < ncpus; j++)
                            values[j] = i;
                    ret = bpf_map_update_elem(fd, &i, &values, BPF_ANY);
                    if (ret < 0)
                            return ret;
            }

            return ret;
    }

姝や唬鐮佺墖娈靛睍绀轰簡濡備綍璁块棶鏁扮粍鍊肩殑姣?CPU 鍏冪礌銆?

    int lookup(int fd)
    {
            int ncpus = libbpf_num_possible_cpus();
            __u32 index = 42, j;
            long values[ncpus];
            int ret;

            ret = bpf_map_lookup_elem(fd, &index, &values);
            if (ret < 0)
                    return ret;

            for (j = 0; j < ncpus; j++) {
                    /** Use per CPU value here **/
                    assert(values[j] == 42);
            }

            return ret;
    }

## Semantics


濡備笂渚嬫墍绀猴紝鍦ㄧ敤鎴风┖闂磋闂?`BPF_MAP_TYPE_PERCPU_ARRAY` 鏃讹紝姣忎釜鍊奸兘鏄竴涓寘鍚?`ncpus` 涓厓绱犵殑鏁扮粍銆?
璋冪敤 `bpf_map_update_elem()` 鏃讹紝瀵逛簬杩欎簺鏄犲皠涓嶈兘浣跨敤 `BPF_NOEXIST` 鏍囧織銆?