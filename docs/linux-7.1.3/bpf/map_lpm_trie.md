
## BPF_MAP_TYPE_LPM_TRIE


   - `BPF_MAP_TYPE_LPM_TRIE` 鍦?kernel 4.11 鐗堟湰涓紩鍏?
`BPF_MAP_TYPE_LPM_TRIE` 鎻愪緵浜嗕竴绉嶆渶闀垮墠缂€鍖归厤锛坙ongest prefix match锛夌畻娉曪紝鍙敤浜庡皢 IP 鍦板潃涓庝竴缁勫凡瀛樺偍鐨勫墠缂€杩涜鍖归厤銆?鍦ㄥ唴閮紝鏁版嵁瀛樺偍鍦ㄧ敱浣跨敤 `prefixlen,data` 瀵逛綔涓洪敭鐨勮妭鐐圭粍鎴愮殑涓嶅钩琛?trie 涓€俙data` 浠ュぇ绔紙缃戠粶瀛楄妭搴忥級瑙ｉ噴锛屽洜姝?`data[^0^]` 瀛樺偍鏈€楂樻湁鏁堝瓧鑺傘€?
LPM trie 鍙互鍦ㄥ垱寤烘椂鎸囧畾鏈€澶у墠缂€闀垮害锛岃闀垮害蹇呴』鏄?8 鐨勫€嶆暟锛岃寖鍥翠粠 8 鍒?2048銆傜敤浜庢煡鎵惧拰鏇存柊鎿嶄綔鐨勯敭鏄竴涓?`struct bpf_lpm_trie_key_u8`锛岀敱 `max_prefixlen/8` 瀛楄妭鎵╁睍銆?
- 瀵逛簬 IPv4 鍦板潃锛宒ata 闀垮害涓?4 瀛楄妭
- 瀵逛簬 IPv6 鍦板潃锛宒ata 闀垮害涓?16 瀛楄妭

瀛樺偍鍦?LPM trie 涓殑鍊肩被鍨嬪彲浠ユ槸浠绘剰鐢ㄦ埛瀹氫箟鐨勭被鍨嬨€?
   鍒涘缓绫诲瀷涓?`BPF_MAP_TYPE_LPM_TRIE` 鐨勬槧灏勬椂锛屽繀椤昏缃?`BPF_F_NO_PREALLOC` 鏍囧織銆?
## Usage


### Kernel BPF


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

瀵逛簬缁欏畾鐨?data 鍊硷紝鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鎵惧埌鏈€闀垮墠缂€鏉＄洰銆傝杈呭姪鍑芥暟杩斿洖涓€涓寚鍚戜笌鏈€闀垮尮閰?`key` 鍏宠仈鐨勫€肩殑鎸囬拡锛屽鏋滄湭鎵惧埌浠讳綍鏉＄洰鍒欒繑鍥?`NULL`銆?
鎵ц鏈€闀垮墠缂€鏌ユ壘鏃讹紝`key` 鐨?`prefixlen` 搴旇缃负 `max_prefixlen`銆備緥濡傦紝褰撴悳绱㈡煇涓?IPv4 鍦板潃鐨勬渶闀垮墠缂€鍖归厤鏃讹紝`prefixlen` 搴旇缃负 `32`銆?
#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

鍙互浣跨敤 `bpf_map_update_elem()` 杈呭姪鍑芥暟娣诲姞鎴栨洿鏂板墠缂€鏉＄洰銆傝杈呭姪鍑芥暟浠ュ師瀛愭柟寮忔浛鎹㈠凡鏈夌殑鍏冪礌銆?
`bpf_map_update_elem()` 鎴愬姛鏃惰繑鍥?`0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
```
    flags 鍙傛暟蹇呴』鏄?BPF_ANY銆丅PF_NOEXIST 鎴?BPF_EXIST 涔嬩竴锛屼絾璇ュ€间細琚拷鐣ワ紝浠庤€岀粰鍑?BPF_ANY 鐨勮涔夈€?
```
#### bpf_map_delete_elem()


   long bpf_map_delete_elem(struct bpf_map **map, const void **key)

鍙互浣跨敤 `bpf_map_delete_elem()` 杈呭姪鍑芥暟鍒犻櫎鍓嶇紑鏉＄洰銆傝杈呭姪鍑芥暟鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
### Userspace


鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勮闂娇鐢ㄤ笌涓婅堪鍚屽悕銆佷互 `fd` 鏍囪瘑鏄犲皠鐨?libbpf API銆?
#### bpf_map_get_next_key()


   int bpf_map_get_next_key (int fd, const void **cur_key, void **next_key)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_get_next_key()` 鍑芥暟閬嶅巻 LPM trie 涓殑鏉＄洰銆傚彲浠ラ€氳繃灏?`cur_key` 璁剧疆涓?`NULL` 鏉ヨ皟鐢?`bpf_map_get_next_key()` 鑾峰彇绗竴涓敭銆傚悗缁皟鐢ㄥ皢鑾峰彇褰撳墠閿箣鍚庣殑涓嬩竴涓敭銆俙bpf_map_get_next_key()` 鎴愬姛鏃惰繑鍥?`0`锛涘鏋?`cur_key` 鏄?trie 涓殑鏈€鍚庝竴涓敭鍒欒繑鍥?`-ENOENT`锛涘け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
`bpf_map_get_next_key()` 灏嗕粠鏈€宸︿晶鐨勫彾瀛愬紑濮嬮亶鍘?LPM trie 鍏冪礌銆傝繖鎰忓懗鐫€杩唬浼氬厛杩斿洖鏇村叿浣撶殑閿紝鐒跺悗鎵嶆槸鏇翠笉鍏蜂綋鐨勯敭銆?
## Examples


LPM trie 鍦ㄧ敤鎴风┖闂寸殑浣跨敤绀轰緥锛岃鍙傞槄 `tools/testing/selftests/bpf/test_lpm_map.c`銆備笅闈㈢殑浠ｇ爜鐗囨婕旂ず浜?API 鐢ㄦ硶銆?
### Kernel BPF


浠ヤ笅 BPF 浠ｇ爜鐗囨灞曠ず浜嗗浣曚负 IPv4 鍦板潃鍓嶇紑澹版槑涓€涓柊鐨?LPM trie锛?

    #include <linux/bpf.h>
    #include <bpf/bpf_helpers.h>

    struct ipv4_lpm_key {
            __u32 prefixlen;
            __u32 data;
    };

    struct {
            __uint(type, BPF_MAP_TYPE_LPM_TRIE);
            __type(key, struct ipv4_lpm_key);
            __type(value, __u32);
            __uint(map_flags, BPF_F_NO_PREALLOC);
            __uint(max_entries, 255);
    } ipv4_lpm_map SEC(".maps");

浠ヤ笅 BPF 浠ｇ爜鐗囨灞曠ず浜嗗浣曟寜 IPv4 鍦板潃鏌ユ壘锛?

    void *lookup(__u32 ipaddr)
    {
            struct ipv4_lpm_key key = {
                    .prefixlen = 32,
                    .data = ipaddr
            };

            return bpf_map_lookup_elem(&ipv4_lpm_map, &key);
    }

### Userspace


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞悜 LPM trie 鎻掑叆涓€涓?IPv4 鍓嶇紑鏉＄洰锛?

    int add_prefix_entry(int lpm_fd, __u32 addr, __u32 prefixlen, struct value *value)
    {
            struct ipv4_lpm_key ipv4_key = {
                    .prefixlen = prefixlen,
                    .data = addr
            };
            return bpf_map_update_elem(lpm_fd, &ipv4_key, value, BPF_ANY);
    }

浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗕竴涓亶鍘?LPM trie 鏉＄洰鐨勭敤鎴风┖闂寸▼搴忥細



    #include <bpf/libbpf.h>
    #include <bpf/bpf.h>

    void iterate_lpm_trie(int map_fd)
    {
            struct ipv4_lpm_key *cur_key = NULL;
            struct ipv4_lpm_key next_key;
            struct value value;
            int err;

            for (;;) {
                    err = bpf_map_get_next_key(map_fd, cur_key, &next_key);
                    if (err)
                            break;

                    bpf_map_lookup_elem(map_fd, &next_key, &value);

                    /** 鍦ㄦ澶勪娇鐢?key 鍜?value **/

                    cur_key = &next_key;
            }
    }
