
## BPF_MAP_TYPE_HASH锛屽強鍏?PERCPU 涓?LRU 鍙樹綋


   - `BPF_MAP_TYPE_HASH` 鍦ㄥ唴鏍哥増鏈?3.19 涓紩鍏?   - `BPF_MAP_TYPE_PERCPU_HASH` 鍦ㄧ増鏈?4.6 涓紩鍏?   - `BPF_MAP_TYPE_LRU_HASH` 鍜?`BPF_MAP_TYPE_LRU_PERCPU_HASH`
     鍧囧湪鐗堟湰 4.10 涓紩鍏?
`BPF_MAP_TYPE_HASH` 鍜?`BPF_MAP_TYPE_PERCPU_HASH` 鎻愪緵閫氱敤鐨勫搱甯屾槧灏勫瓨鍌ㄣ€傞敭鍜屽€奸兘鍙互鏄粨鏋勪綋锛屼粠鑰屽厑璁稿鍚堥敭鍜屽€笺€?
鍐呮牳璐熻矗鍒嗛厤鍜岄噴鏀鹃敭鍊煎锛屼笂闄愪负鎮ㄦ寚瀹氱殑 max_entries銆傚搱甯屾槧灏勯粯璁ら鍒嗛厤鍝堝笇琛ㄥ厓绱犮€傚綋棰勫垎閰嶇殑鍐呭瓨寮€閿€杩囧ぇ鏃讹紝鍙互浣跨敤 `BPF_F_NO_PREALLOC` 鏍囧織鏉ョ鐢ㄩ鍒嗛厤銆?
`BPF_MAP_TYPE_PERCPU_HASH` 涓烘瘡涓?CPU 鎻愪緵涓€涓嫭绔嬬殑鍊兼Ы浣嶃€傛瘡 CPU 鐨勫€煎湪鍐呴儴浠ユ暟缁勫舰寮忓瓨鍌ㄣ€?
`BPF_MAP_TYPE_LRU_HASH` 鍜?`BPF_MAP_TYPE_LRU_PERCPU_HASH` 鍙樹綋涓哄悇鑷殑鍝堝笇琛ㄥ鍔犱簡 LRU 璇箟銆傚綋鍝堝笇琛ㄨ揪鍒板閲忔椂锛孡RU 鍝堝笇浼氳嚜鍔ㄩ┍閫愭渶杩戞渶灏戜娇鐢ㄧ殑鏉＄洰銆侺RU 鍝堝笇缁存姢涓€涓唴閮ㄧ殑 LRU 鍒楄〃锛岀敤浜庨€夋嫨瑕侀┍閫愮殑鍏冪礌銆傝繖涓唴閮?LRU 鍒楄〃鍦?CPU 涔嬮棿鍏变韩锛屼絾鍦ㄨ皟鐢?`bpf_map_create` 鏃讹紝鍙互閫氳繃 `BPF_F_NO_COMMON_LRU` 鏍囧織璇锋眰涓€涓瘡 CPU 鐨?LRU 鍒楄〃銆備笅琛ㄦ牴鎹槧灏勭被鍨嬩互鍙婄敤浜庡垱寤烘槧灏勭殑鏍囧織锛屾杩颁簡 LRU 鏄犲皠鐨勫睘鎬с€?
======================== ========================= ================================
Flag                     `BPF_MAP_TYPE_LRU_HASH` `BPF_MAP_TYPE_LRU_PERCPU_HASH`
======================== ========================= ================================
**BPF_F_NO_COMMON_LRU**  姣?CPU LRU锛屽叏灞€鏄犲皠       姣?CPU LRU锛屾瘡 CPU 鏄犲皠
**!BPF_F_NO_COMMON_LRU** 鍏ㄥ眬 LRU锛屽叏灞€鏄犲皠        鍏ㄥ眬 LRU锛屾瘡 CPU 鏄犲皠
======================== ========================= ================================

## 鐢ㄦ硶


### 鍐呮牳 BPF


#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

鍝堝笇鏉＄洰鍙互浣跨敤 `bpf_map_update_elem()` 杈呭姪鍑芥暟娣诲姞鎴栨洿鏂般€傝杈呭姪鍑芥暟浠ュ師瀛愭柟寮忔浛鎹㈠凡鏈夊厓绱犮€俙flags` 鍙傛暟鍙敤浜庢帶鍒舵洿鏂拌涓猴細

- `BPF_ANY` 灏嗗垱寤烘柊鍏冪礌鎴栨洿鏂板凡鏈夊厓绱?- `BPF_NOEXIST` 浠呭綋鍏冪礌灏氫笉瀛樺湪鏃舵墠鍒涘缓鏂板厓绱?- `BPF_EXIST` 灏嗘洿鏂板凡鏈夊厓绱?
`bpf_map_update_elem()` 鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

鍝堝笇鏉＄洰鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟妫€绱€傝杈呭姪鍑芥暟杩斿洖涓?`key` 鍏宠仈鐨勫€肩殑鎸囬拡锛屽鏋滄湭鎵惧埌鏉＄洰鍒欒繑鍥?`NULL`銆?
#### bpf_map_delete_elem()


   long bpf_map_delete_elem(struct bpf_map **map, const void **key)

鍝堝笇鏉＄洰鍙互浣跨敤 `bpf_map_delete_elem()` 杈呭姪鍑芥暟鍒犻櫎銆傝杈呭姪鍑芥暟鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
### 姣?CPU 鍝堝笇


瀵逛簬 `BPF_MAP_TYPE_PERCPU_HASH` 鍜?`BPF_MAP_TYPE_LRU_PERCPU_HASH`锛宍bpf_map_update_elem()` 鍜?`bpf_map_lookup_elem()` 杈呭姪鍑芥暟浼氳嚜鍔ㄨ闂綋鍓?CPU 鐨勫搱甯屾Ы浣嶃€?
#### bpf_map_lookup_percpu_elem()


   void **bpf_map_lookup_percpu_elem(struct bpf_map **map, const void *key, u32 cpu)

`bpf_map_lookup_percpu_elem()` 杈呭姪鍑芥暟鍙敤浜庢煡鎵剧壒瀹?CPU 鐨勫搱甯屾Ы浣嶄腑鐨勫€笺€傝繑鍥?`cpu` 涓婁笌 `key` 鍏宠仈鐨勫€硷紝濡傛灉鏈壘鍒版潯鐩垨 `cpu` 鏃犳晥鍒欒繑鍥?`NULL`銆?
### 骞跺彂


瀛樺偍鍦?`BPF_MAP_TYPE_HASH` 涓殑鍊煎彲浠ヨ杩愯鍦ㄤ笉鍚?CPU 涓婄殑绋嬪簭骞跺彂璁块棶銆傝嚜鍐呮牳鐗堟湰 5.1 璧凤紝BPF 鍩虹璁炬柦鎻愪緵 `struct bpf_spin_lock` 鏉ュ悓姝ヨ闂€?鍙傝 `tools/testing/selftests/bpf/progs/test_spin_lock.c`銆?
### 鐢ㄦ埛绌洪棿


#### bpf_map_get_next_key()


   int bpf_map_get_next_key(int fd, const void **cur_key, void **next_key)

鍦ㄧ敤鎴风┖闂翠腑锛屽彲浠ヤ娇鐢?libbpf 鐨?`bpf_map_get_next_key()` 鍑芥暟閬嶅巻鍝堝笇鐨勯敭銆傚彲浠ラ€氳繃灏?`cur_key` 璁句负 `NULL` 鏉ヨ皟鐢?`bpf_map_get_next_key()` 鑾峰彇绗竴涓敭銆傚悗缁皟鐢ㄥ皢鑾峰彇褰撳墠閿箣鍚庣殑涓嬩竴涓敭銆俙bpf_map_get_next_key()` 鎴愬姛鏃惰繑鍥?0锛屽鏋?cur_key 鏄搱甯屼腑鐨勬渶鍚庝竴涓敭鍒欒繑鍥?-ENOENT锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
璇锋敞鎰忥紝濡傛灉 `cur_key` 琚垹闄わ紝閭ｄ箞 `bpf_map_get_next_key()` 鍙嶈€屼細杩斿洖鍝堝笇琛ㄤ腑鐨?*绗竴涓?*閿紝杩欐槸涓嶇悊鎯崇殑銆傚鏋滀細鍦?`bpf_map_get_next_key()` 鎿嶄綔杩囩▼涓贩鍚堣繘琛岄敭鍒犻櫎锛屽缓璁娇鐢ㄦ壒閲忔煡鎵俱€?
## 绀轰緥


璇峰弬闃?`tools/testing/selftests/bpf` 鐩綍涓殑鍔熻兘绀轰緥銆備笅闈㈢殑浠ｇ爜鐗囨婕旂ず浜?API 鐢ㄦ硶銆?
姝ょず渚嬪睍绀哄浣曞０鏄庝竴涓甫鏈夌粨鏋勪綋閿拰缁撴瀯浣撳€肩殑 LRU Hash銆?

    #include <linux/bpf.h>
    #include <bpf/bpf_helpers.h>

    struct key {
        __u32 srcip;
    };

    struct value {
        __u64 packets;
        __u64 bytes;
    };

    struct {
            __uint(type, BPF_MAP_TYPE_LRU_HASH);
            __uint(max_entries, 32);
            __type(key, struct key);
            __type(value, struct value);
    } packet_stats SEC(".maps");

姝ょず渚嬪睍绀哄浣曚娇鐢ㄥ師瀛愭寚浠ゅ垱寤烘垨鏇存柊鍝堝笇鍊硷細


    static void update_stats(__u32 srcip, int bytes)
    {
            struct key key = {
                    .srcip = srcip,
            };
            struct value *value = bpf_map_lookup_elem(&packet_stats, &key);

            if (value) {
                    __sync_fetch_and_add(&value->packets, 1);
                    __sync_fetch_and_add(&value->bytes, bytes);
            } else {
                    struct value newval = { 1, bytes };

                    bpf_map_update_elem(&packet_stats, &key, &newval, BPF_NOEXIST);
            }
    }

鍦ㄧ敤鎴风┖闂翠腑閬嶅巻涓婇潰澹版槑鐨勬槧灏勫厓绱狅細


    #include <bpf/libbpf.h>
    #include <bpf/bpf.h>

    static void walk_hash_elements(int map_fd)
    {
            struct key *cur_key = NULL;
            struct key next_key;
            struct value value;
            int err;

            for (;;) {
                    err = bpf_map_get_next_key(map_fd, cur_key, &next_key);
                    if (err)
                            break;

                    bpf_map_lookup_elem(map_fd, &next_key, &value);

                    // 鍦ㄦ澶勪娇鐢ㄩ敭鍜屽€?
                    cur_key = &next_key;
            }
    }

## 鍐呴儴瀹炵幇


鏈枃妗ｇ殑杩欎竴閮ㄥ垎闈㈠悜 Linux 寮€鍙戣€咃紝鎻忚堪浜嗕笉琚浣滅ǔ瀹?ABI 鐨勬槧灏勫疄鐜扮粏鑺傘€備互涓嬬粏鑺傚彲鑳藉湪鏈潵鐨勫唴鏍哥増鏈腑鍙戠敓鍙樺寲銆?
### ``BPF_MAP_TYPE_LRU_HASH`` 鍙婂叾鍙樹綋


鏇存柊 LRU 鏄犲皠涓殑鍏冪礌鏃讹紝褰撴槧灏勫閲忚揪鍒颁笂闄愶紝鍙兘浼氳Е鍙戦┍閫愯涓恒€傛洿鏂扮畻娉曚細灏濊瘯鑻ュ共姝ラ浠ュ己鍒舵墽琛?LRU 灞炴€э紝杩欎簺姝ラ瀵瑰悗缁搷浣滃皾璇曚腑鎵€娑夊強鐨勫叾瀹?CPU 鐨勫奖鍝嶈秺鏉ヨ秺澶э細

- 灏濊瘯浣跨敤 CPU 鏈湴鐘舵€佹潵鎵归噺鎿嶄綔
- 灏濊瘯浠庡叏灞€鍒楄〃鑾峰彇 `target_free` 涓┖闂茶妭鐐?- 灏濊瘯浠庡叏灞€鍒楄〃鎷夊彇浠绘剰鑺傜偣骞跺皢鍏朵粠鍝堝笇鏄犲皠涓Щ闄?- 灏濊瘯浠庝换鎰?CPU 鐨勫垪琛ㄤ腑鎷夊彇浠绘剰鑺傜偣骞跺皢鍏朵粠鍝堝笇鏄犲皠涓Щ闄?
鎵归噺浠庡叏灞€鍒楄〃鍊熺敤鐨勮妭鐐规暟閲?`target_free` 鍙栧喅浜庢槧灏勭殑澶у皬銆傝緝澶х殑鎵归噺澶у皬鍙噺灏戦攣绔炰簤锛屼絾涔熷彲鑳借€楀敖鍏ㄥ眬缁撴瀯銆傝鍊煎湪鏄犲皠鍒濆鍖栨椂璁＄畻锛屼互閬垮厤鑰楀敖鈥斺€旀柟娉曟槸灏嗘墍鏈?CPU 鐨勮仛鍚堥鐣欓檺鍒朵负鏄犲皠澶у皬鐨勪竴鍗娿€傛渶灏忓€间负鍗曚釜鍏冪礌锛屾渶澶ч绠椾负涓€娆?128 涓€?
璇ョ畻娉曞湪涓嬪浘涓洿瑙傚湴鎻忚堪銆傛湁鍏崇浉搴旀搷浣滅殑瀹屾暣瑙ｉ噴锛岃鍙傝鎻愪氦 3a08c2fd7634锛堚€渂pf: LRU List鈥濓級锛?
   :alt:    鎻忚堪鏄犲皠鏇存柊鏈熼棿鎵€閲囧彇鐨?LRU 椹遍€愭楠ょ殑鍥剧ず銆?
   LRU 鍝堝笇鍦ㄦ槧灏勬洿鏂版湡闂寸殑椹遍€愶紝閽堝 `BPF_MAP_TYPE_LRU_HASH` 鍙婂叾
   鍙樹綋銆傛湁鍏冲唴鏍稿嚱鏁板悕绉颁唬鐮佸紩鐢ㄧ殑鐐规枃浠舵簮锛岃鍙傝瀵瑰簲鐨?dot 鏂囦欢銆?
鏄犲皠鏇存柊浠庡彸涓婅鐨勬き鍦?鈥渂egin `bpf_map_update()`鈥?寮€濮嬶紝骞舵部鐫€鍥惧悜涓嬫帹杩涳紝鏈€缁堢粨鏋滃彲鑳芥槸鏇存柊鎴愬姛锛屼篃鍙兘鏄甫鏈夊悇绉嶉敊璇爜鐨勫け璐ャ€傚彸涓婅鐨勫浘渚嬫寚绀轰簡鍝簺閿佸彲鑳藉弬涓庣壒瀹氭搷浣溿€傝繖鏃ㄥ湪浣滀负涓€涓洿瑙傛彁绀猴紝鐢ㄤ簬鎺ㄧ悊鏄犲皠绔炰簤濡備綍褰卞搷鏇存柊鎿嶄綔锛屼笉杩囧熀浜庝笂琛ㄦ弿杩扮殑閫昏緫锛屾槧灏勭被鍨嬪拰鏍囧織鍙兘浼氬奖鍝嶈繖浜涢攣涓婄殑瀹為檯绔炰簤銆備緥濡傦紝濡傛灉鏄犲皠浠ョ被鍨?`BPF_MAP_TYPE_LRU_PERCPU_HASH` 鍜屾爣蹇?`BPF_F_NO_COMMON_LRU` 鍒涘缓锛岄偅涔堟墍鏈夋槧灏勫睘鎬ч兘灏嗘槸姣?CPU 鐨勩€?