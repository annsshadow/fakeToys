## BPF_MAP_TYPE_ARRAY_OF_MAPS 涓?BPF_MAP_TYPE_HASH_OF_MAPS


   - `BPF_MAP_TYPE_ARRAY_OF_MAPS` 鍜?`BPF_MAP_TYPE_HASH_OF_MAPS` 浜庡唴鏍哥増鏈?4.12 涓紩鍏?

`BPF_MAP_TYPE_ARRAY_OF_MAPS` 鍜?`BPF_MAP_TYPE_HASH_OF_MAPS` 鎻愪緵瀵光€渕ap 涓祵濂?map鈥濆瓨鍌ㄧ殑閫氱敤鏀寔銆傛敮鎸佷竴灞傚祵濂楋紝鍏朵腑澶栧眰 map 鍖呭惈鍗曚竴绫诲瀷鐨勫唴灞?map 鐨勫疄渚嬶紝渚嬪 `array_of_maps->sock_map`銆?

鍒涘缓澶栧眰 map 鏃讹紝浣跨敤涓€涓唴灞?map 瀹炰緥鏉ュ垵濮嬪寲澶栧眰 map 鎸佹湁鐨勩€佸叧浜庡叾鍐呭眰 map 鐨勫厓鏁版嵁銆傝鍐呭眰 map 鍏锋湁涓庡灞?map 鐙珛鐨勭敓鍛藉懆鏈燂紝骞朵笖鍙互鍦ㄥ灞?map 鍒涘缓涔嬪悗琚垹闄ゃ€?

澶栧眰 map 鏀寔浣跨敤 syscall API 浠庣敤鎴风┖闂磋繘琛屽厓绱犳煡鎵俱€佹洿鏂板拰鍒犻櫎銆侭PF 绋嬪簭鍙厑璁稿湪澶栧眰 map 涓繘琛屽厓绱犳煡鎵俱€?

   - 涓嶆敮鎸佸绾у祵濂椼€?
   - 浠讳綍 BPF map 绫诲瀷閮藉彲浠ョ敤浣滃唴灞?map锛岄櫎浜?`BPF_MAP_TYPE_PROG_ARRAY`銆?
   - BPF 绋嬪簭涓嶈兘鏇存柊鎴栧垹闄ゅ灞?map 鏉＄洰銆?

瀵逛簬 `BPF_MAP_TYPE_ARRAY_OF_MAPS`锛岄敭鏄竴涓棤绗﹀彿 32 浣嶆暣鏁扮储寮曪紝鐢ㄤ簬绱㈠紩鏁扮粍銆傝鏁扮粍鏄浐瀹氬ぇ灏忕殑锛屽叿鏈?`max_entries` 涓厓绱狅紝鍦ㄥ垱寤烘椂闆跺垵濮嬪寲銆?

瀵逛簬 `BPF_MAP_TYPE_HASH_OF_MAPS`锛岄敭绫诲瀷鍙互鍦ㄥ畾涔?map 鏃堕€夋嫨銆傚唴鏍歌礋璐ｅ垎閰嶅拰閲婃斁閿?鍊煎锛屼笂闄愪负浣犳寚瀹氱殑 max_entries銆傞粯璁ゆ儏鍐典笅锛屽搱甯?map 浣跨敤鍝堝笇琛ㄥ厓绱犵殑棰勫垎閰嶃€俙BPF_F_NO_PREALLOC` 鏍囧織鍙敤浜庡湪棰勫垎閰嶈繃浜庤€楄垂鍐呭瓨鏃剁鐢ㄩ鍒嗛厤銆?

## 鐢ㄦ硶


### 鍐呮牳 BPF 杈呭姪鍑芥暟


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

鍐呭眰 map 鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鑾峰彇銆傝杈呭姪鍑芥暟杩斿洖鎸囧悜鍐呭眰 map 鐨勬寚閽堬紝鑻ユ湭鎵惧埌鏉＄洰鍒欒繑鍥?`NULL`銆?

## 绀轰緥


### 鍐呮牳 BPF 绀轰緥


姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍦?BPF 绋嬪簭涓垱寤哄苟鍒濆鍖栦竴涓?devmap 鏁扮粍銆傛敞鎰忥紝澶栧眰鏁扮粍鍙兘浠庣敤鎴风┖闂翠娇鐢?syscall API 淇敼銆?

    struct inner_map {
            __uint(type, BPF_MAP_TYPE_DEVMAP);
            __uint(max_entries, 10);
            __type(key, __u32);
            __type(value, __u32);
    } inner_map1 SEC(".maps"), inner_map2 SEC(".maps");

    struct {
            __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
            __uint(max_entries, 2);
            __type(key, __u32);
            __type(value, struct inner_map);
    } outer_map SEC(".maps") = {
            .values = { &inner_map1,
                        &inner_map2 }
    };

鏈夊叧澶栧眰 map 澹版槑寮忓垵濮嬪寲鐨勬洿澶氱ず渚嬶紝璇峰弬瑙?`tools/testing/selftests/bpf` 涓殑 `progs/test_btf_map_in_map.c`銆?

### 鐢ㄦ埛绌洪棿


姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍒涘缓鍩轰簬鏁扮粍鐨勫灞?map锛?

    int create_outer_array(int inner_fd) {
            LIBBPF_OPTS(bpf_map_create_opts, opts, .inner_map_fd = inner_fd);
            int fd;

            fd = bpf_map_create(BPF_MAP_TYPE_ARRAY_OF_MAPS,
                                "example_array",       /** name **/
                                sizeof(__u32),         /** key size **/
                                sizeof(__u32),         /** value size **/
                                256,                   /** max entries **/
                                &opts);                /** create opts **/
            return fd;
    }

姝や唬鐮佺墖娈靛睍绀轰簡濡備綍鍚戝唴灞?map 娣诲姞鍒颁竴涓灞?map锛?

    int add_devmap(int outer_fd, int index, const char *name) {
            int fd;

            fd = bpf_map_create(BPF_MAP_TYPE_DEVMAP, name,
                                sizeof(__u32), sizeof(__u32), 256, NULL);
            if (fd < 0)
                    return fd;

            return bpf_map_update_elem(outer_fd, &index, &fd, BPF_ANY);
    }

## 鍙傝€冭祫鏂?


- https://lore.kernel.org/netdev/20170322170035.923581-3-kafai@fb.com/
- https://lore.kernel.org/netdev/20170322170035.923581-4-kafai@fb.com/
