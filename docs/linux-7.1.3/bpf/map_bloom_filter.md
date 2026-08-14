
## BPF_MAP_TYPE_BLOOM_FILTER


   - `BPF_MAP_TYPE_BLOOM_FILTER` 鍦?5.16 鐗堝唴鏍镐腑寮曞叆

`BPF_MAP_TYPE_BLOOM_FILTER` 鎻愪緵浜嗕竴绉?BPF 甯冮殕杩囨护鍣紙bloom filter锛夋槧灏勩€傚竷闅嗚繃婊ゅ櫒
鏄竴绉嶇┖闂撮珮鏁堢殑姒傜巼鎬ф暟鎹粨鏋勶紝鐢ㄤ簬蹇€熷垽鏂煇涓厓绱犳槸鍚﹀瓨鍦ㄤ簬涓€涓泦鍚堜腑銆傚湪甯冮殕杩囨护鍣ㄤ腑锛?鍙兘鍑虹幇鍋囬槼鎬э紙false positive锛夛紝浣嗕笉浼氬嚭鐜板亣闃存€э紙false negative锛夈€?
甯冮殕杩囨护鍣ㄦ槧灏勬病鏈夐敭锛坘ey锛夛紝鍙湁鍊硷紙value锛夈€傚垱寤哄竷闅嗚繃婊ゅ櫒鏄犲皠鏃讹紝蹇呴』浠?`key_size` 涓?0
鏉ュ垱寤恒€傚竷闅嗚繃婊ゅ櫒鏄犲皠鏀寔涓ょ鎿嶄綔锛?
- push锛氬悜鏄犲皠涓坊鍔犱竴涓厓绱?- peek锛氬垽鏂煇涓厓绱犳槸鍚﹀瓨鍦ㄤ簬鏄犲皠涓?
BPF 绋嬪簭蹇呴』浣跨敤 `bpf_map_push_elem` 鏉ュ悜甯冮殕杩囨护鍣ㄦ槧灏勬坊鍔犲厓绱狅紝浣跨敤 `bpf_map_peek_elem`
鏉ユ煡璇㈡槧灏勩€傝繖浜涙搷浣滈€氳繃宸叉湁鐨?`bpf` 绯荤粺璋冪敤浠ヤ笅鍒楁柟寮忔毚闇茬粰鐢ㄦ埛绌洪棿搴旂敤绋嬪簭锛?
- `BPF_MAP_UPDATE_ELEM` -> push
- `BPF_MAP_LOOKUP_ELEM` -> peek

鍒涘缓鏄犲皠鏃舵寚瀹氱殑 `max_entries` 澶у皬鐢ㄤ簬涓哄竷闅嗚繃婊ゅ櫒浼扮畻涓€涓悎鐞嗙殑浣嶅浘澶у皬锛岄櫎姝や箣澶栧苟涓嶈
涓ユ牸寮哄埗銆傚鏋滅敤鎴峰笇鏈涘悜甯冮殕杩囨护鍣ㄤ腑鎻掑叆姣?`max_entries` 鏇村鐨勬潯鐩紝鍙兘浼氬鑷存洿楂樼殑鍋囬槼鎬х巼銆?
甯冮殕杩囨护鍣ㄤ娇鐢ㄧ殑鍝堝笇鏁伴噺鍙湪鍒涘缓鏄犲皠鏃堕€氳繃 `union bpf_attr` 涓?`map_extra` 鐨勪綆 4 浣嶆潵閰嶇疆銆?濡傛灉鏈寚瀹氭暟閲忥紝榛樿浣跨敤 5 涓搱甯屽嚱鏁般€備竴鑸€岃█锛屼娇鐢ㄦ洿澶氱殑鍝堝笇浼氶檷浣庡亣闃虫€х巼锛屼絾涔熶細闄嶄綆
鏌ユ壘閫熷害銆?
鏃犳硶浠庡竷闅嗚繃婊ゅ櫒鏄犲皠涓垹闄ゅ厓绱犮€傚竷闅嗚繃婊ゅ櫒鏄犲皠鍙敤浣滃唴閮ㄦ槧灏勶紙inner map锛夈€傜敤鎴疯礋璐ｅ悓姝?骞跺彂鐨勬洿鏂板拰鏌ユ壘锛屼互纭繚涓嶄細鍙戠敓鍋囬槾鎬ф煡鎵俱€?
## 鐢ㄦ硶


### 鍐呮牳 BPF


#### bpf_map_push_elem()


   long bpf_map_push_elem(struct bpf_map **map, const void **value, u64 flags)

鍙互浣跨敤 `bpf_map_push_elem()` 杈呭姪鍑芥暟鍚戝竷闅嗚繃婊ゅ櫒娣诲姞涓€涓?`value`銆傚悜甯冮殕杩囨护鍣ㄦ坊鍔犳潯鐩椂锛?`flags` 鍙傛暟蹇呴』璁句负 `BPF_ANY`銆傝杈呭姪鍑芥暟鍦ㄦ垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_peek_elem()


   long bpf_map_peek_elem(struct bpf_map **map, void **value)

`bpf_map_peek_elem()` 杈呭姪鍑芥暟鐢ㄤ簬鍒ゆ柇 `value` 鏄惁瀛樺湪浜庡竷闅嗚繃婊ゅ櫒鏄犲皠涓€傚鏋?`value` 寰堝彲鑳?瀛樺湪浜庢槧灏勪腑锛岃杈呭姪鍑芥暟杩斿洖 `0`锛涘鏋?`value` 涓€瀹氫笉瀛樺湪浜庢槧灏勪腑锛屽垯杩斿洖 `-ENOENT`銆?
### 鐢ㄦ埛绌洪棿


#### bpf_map_update_elem()


   int bpf_map_update_elem (int fd, const void **key, const void **value, __u64 flags)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_update_elem` 鍑芥暟鍚戝竷闅嗚繃婊ゅ櫒娣诲姞涓€涓?`value`銆俙key`
鍙傛暟蹇呴』璁句负 `NULL`锛宍flags` 蹇呴』璁句负 `BPF_ANY`銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem (int fd, const void **key, void **value)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_lookup_elem` 鍑芥暟鍒ゆ柇 `value` 鏄惁瀛樺湪浜庡竷闅嗚繃婊ゅ櫒涓€?`key` 鍙傛暟蹇呴』璁句负 `NULL`銆傚鏋?`value` 寰堝彲鑳藉瓨鍦ㄤ簬鏄犲皠涓繑鍥?`0`锛屽鏋?`value` 涓€瀹氫笉瀛樺湪浜?鏄犲皠涓垯杩斿洖 `-ENOENT`銆?
## 绀轰緥


### 鍐呮牳 BPF


姝ょ墖娈靛睍绀轰簡濡備綍鍦?BPF 绋嬪簭涓０鏄庝竴涓竷闅嗚繃婊ゅ櫒锛?

    struct {
            __uint(type, BPF_MAP_TYPE_BLOOM_FILTER);
            __type(value, __u32);
            __uint(max_entries, 1000);
            __uint(map_extra, 3);
    } bloom_filter SEC(".maps");

姝ょ墖娈靛睍绀轰簡濡備綍鍦?BPF 绋嬪簭涓垽鏂竷闅嗚繃婊ゅ櫒涓煇涓€兼槸鍚﹀瓨鍦細


    void *lookup(__u32 key)
    {
            if (bpf_map_peek_elem(&bloom_filter, &key) == 0) {
                    /* 楠岃瘉涓嶆槸鍋囬槼鎬э紝骞朵娇鐢ㄦ绾ф煡鎵撅紙渚嬪鍦ㄥ搱甯岃〃涓級
                     - 鑾峰彇鍏宠仈鐨勫€?                     */
                    return bpf_map_lookup_elem(&hash_table, &key);
            }
            return 0;
    }

### 鐢ㄦ埛绌洪棿


姝ょ墖娈靛睍绀轰簡濡備綍浣跨敤 libbpf 浠庣敤鎴风┖闂村垱寤轰竴涓竷闅嗚繃婊ゅ櫒鏄犲皠锛?

    int create_bloom()
    {
            LIBBPF_OPTS(bpf_map_create_opts, opts,
                        .map_extra = 3);             /** 鍝堝笇鏁伴噺 **/

            return bpf_map_create(BPF_MAP_TYPE_BLOOM_FILTER,
                                  "ipv6_bloom",      /** 鍚嶇О **/
                                  0,                 /** 閿ぇ灏忥紝蹇呴』涓?0 **/
                                  sizeof(ipv6_addr), /** 鍊煎ぇ灏?**/
                                  10000,             /** 鏈€澶ф潯鐩暟 **/
                                  &opts);            /** 鍒涘缓閫夐」 **/
    }

姝ょ墖娈靛睍绀轰簡濡備綍浠庣敤鎴风┖闂村悜甯冮殕杩囨护鍣ㄦ坊鍔犱竴涓厓绱狅細


    int add_element(struct bpf_map *bloom_map, __u32 value)
    {
            int bloom_fd = bpf_map__fd(bloom_map);
            return bpf_map_update_elem(bloom_fd, NULL, &value, BPF_ANY);
    }

## 鍙傝€冭祫鏂?

https://lwn.net/ml/bpf/20210831225005.2762202-1-joannekoong@fb.com/
