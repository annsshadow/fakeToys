
## BPF_MAP_TYPE_SK_STORAGE


   - `BPF_MAP_TYPE_SK_STORAGE` 鍦ㄥ唴鏍哥増鏈?5.2 涓紩鍏?
`BPF_MAP_TYPE_SK_STORAGE` 鐢ㄤ簬涓?BPF 绋嬪簭鎻愪緵濂楁帴瀛楁湰鍦帮紙socket-local锛?瀛樺偍銆傜被鍨嬩负 `BPF_MAP_TYPE_SK_STORAGE` 鐨勬槧灏勫０鏄庤鎻愪緵鐨勫瓨鍌ㄧ被鍨嬶紝骞朵綔涓?璁块棶濂楁帴瀛楁湰鍦板瓨鍌ㄧ殑鍙ユ焺銆傜被鍨嬩负 `BPF_MAP_TYPE_SK_STORAGE` 鐨勬槧灏勭殑鍊间笌
鏄犲皠鏈韩涓€璧峰瓨鍌ㄥ湪姣忎釜濂楁帴瀛楁湰鍦帮紝鑰屼笉鏄笌鏄犲皠涓€璧峰瓨鍌ㄣ€傚唴鏍歌礋璐ｅ湪璇锋眰鏃?涓哄鎺ュ瓧鍒嗛厤瀛樺偍锛屽苟鍦ㄦ槧灏勬垨濂楁帴瀛楄鍒犻櫎鏃堕噴鏀惧瓨鍌ㄣ€?
  - 閿被鍨嬪繀椤讳负 `int`锛屽苟涓?`max_entries` 蹇呴』璁句负 `0`銆?  - 鍒涘缓鐢ㄤ簬濂楁帴瀛楁湰鍦板瓨鍌ㄧ殑鏄犲皠鏃跺繀椤讳娇鐢?`BPF_F_NO_PREALLOC` 鏍囧織銆?
## 鐢ㄦ硶


### 鍐呮牳 BPF


#### bpf_sk_storage_get()


   void **bpf_sk_storage_get(struct bpf_map **map, void **sk, void **value, u64 flags)

鍙互浣跨敤 `bpf_sk_storage_get()` 杈呭姪鍑芥暟浠庡鎺ュ瓧 `sk` 鑾峰彇 `map` 鐨?濂楁帴瀛楁湰鍦板瓨鍌ㄣ€傚鏋滀娇鐢ㄤ簡 `BPF_LOCAL_STORAGE_GET_F_CREATE` 鏍囧織锛岄偅涔?`bpf_sk_storage_get()` 灏嗗湪 `sk` 灏氫笉瀛樺湪鏃朵负鍏跺垱寤哄瓨鍌ㄣ€傚彲浠ョ粨鍚?`BPF_LOCAL_STORAGE_GET_F_CREATE` 浣跨敤 `value` 鏉ュ垵濮嬪寲瀛樺偍鍊硷紝鍚﹀垯瀹冨皢琚?闆跺垵濮嬪寲銆傛垚鍔熸椂杩斿洖鎸囧悜璇ュ瓨鍌ㄧ殑鎸囬拡锛屽け璐ユ椂杩斿洖 `NULL`銆?
   - 瀵逛簬 LSM 鎴栬窡韪紙tracing锛夌▼搴忥紝`sk` 鏄竴涓唴鏍?`struct sock` 鎸囬拡銆?   - 瀵逛簬鍏跺畠绋嬪簭绫诲瀷锛宍sk` 鏄竴涓?`struct bpf_sock` 鎸囬拡銆?
#### bpf_sk_storage_delete()


   long bpf_sk_storage_delete(struct bpf_map **map, void **sk)

鍙互浣跨敤 `bpf_sk_storage_delete()` 杈呭姪鍑芥暟浠庡鎺ュ瓧 `sk` 鍒犻櫎 `map` 鐨?濂楁帴瀛楁湰鍦板瓨鍌ㄣ€傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒銆?
### 鐢ㄦ埛绌洪棿


#### bpf_map_update_elem()


   int bpf_map_update_elem(int map_fd, const void **key, const void **value, __u64 flags)

鍙互浣跨敤 `bpf_map_update_elem()` libbpf 鍑芥暟灏?`map` 鏄犲皠鐨勫鎺ュ瓧鏈湴瀛樺偍
娣诲姞鎴栨洿鏂板埌鏌愪釜濂楁帴瀛楁湰鍦般€傚鎺ュ瓧鐢卞瓨鍌ㄥ湪鎸囬拡 `key` 涓殑 `socket` `fd`
鏍囪瘑銆傛寚閽?`value` 鍖呭惈瑕佹坊鍔犳垨鏇存柊鍒拌濂楁帴瀛?`fd` 鐨勬暟鎹€俙value` 鐨勭被鍨?鍜屽ぇ灏忓簲涓庢槧灏勫畾涔変腑鐨勫€肩被鍨嬬浉鍚屻€?
`flags` 鍙傛暟鍙敤浜庢帶鍒舵洿鏂拌涓猴細

- `BPF_ANY` 灏嗕负 `socket` `fd` 鍒涘缓瀛樺偍鎴栨洿鏂扮幇鏈夊瓨鍌ㄣ€?- `BPF_NOEXIST` 灏嗕粎鍦?`socket` `fd` 灏氫笉瀛樺湪鏃舵墠涓哄叾鍒涘缓瀛樺偍锛屽惁鍒欒皟鐢?  灏嗕互 `-EEXIST` 澶辫触銆?- `BPF_EXIST` 灏嗕粎鍦?`socket` `fd` 宸插瓨鍦ㄦ椂鎵嶆洿鏂板叾鐜版湁瀛樺偍锛屽惁鍒欒皟鐢ㄥ皢
  浠?`-ENOENT` 澶辫触銆?
鎴愬姛鏃惰繑鍥?`0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒銆?
#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem(int map_fd, const void *key, void **value)

鍙互浣跨敤 `bpf_map_lookup_elem()` libbpf 鍑芥暟浠庢煇涓鎺ュ瓧鑾峰彇 `map` 鏄犲皠鐨?濂楁帴瀛楁湰鍦板瓨鍌ㄣ€傚瓨鍌ㄦ槸浠庣敱鎸囬拡 `key` 涓瓨鍌ㄧ殑 `socket` `fd` 鎵€鏍囪瘑鐨勫鎺ュ瓧
鑾峰彇鐨勩€傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒銆?
#### bpf_map_delete_elem()


   int bpf_map_delete_elem(int map_fd, const void *key)

鍙互浣跨敤 `bpf_map_delete_elem()` libbpf 鍑芥暟浠庢煇涓鎺ュ瓧鍒犻櫎 `map` 鏄犲皠鐨?濂楁帴瀛楁湰鍦板瓨鍌ㄣ€傚瓨鍌ㄤ粠鐢辨寚閽?`key` 涓瓨鍌ㄧ殑 `socket` `fd` 鎵€鏍囪瘑鐨勫鎺ュ瓧
鍒犻櫎銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒銆?
## 绀轰緥


### 鍐呮牳 BPF


浠ヤ笅鐗囨灞曠ず浜嗗浣曞湪 BPF 绋嬪簭涓０鏄庡鎺ュ瓧鏈湴瀛樺偍锛?

    struct {
            __uint(type, BPF_MAP_TYPE_SK_STORAGE);
            __uint(map_flags, BPF_F_NO_PREALLOC);
            __type(key, int);
            __type(value, struct my_storage);
    } socket_storage SEC(".maps");

浠ヤ笅鐗囨灞曠ず浜嗗浣曞湪 BPF 绋嬪簭涓幏鍙栧鎺ュ瓧鏈湴瀛樺偍锛?

    SEC("sockops")
    int _sockops(struct bpf_sock_ops *ctx)
    {
            struct my_storage *storage;
            struct bpf_sock *sk;

            sk = ctx->sk;
            if (!sk)
                    return 1;

            storage = bpf_sk_storage_get(&socket_storage, sk, 0,
                                         BPF_LOCAL_STORAGE_GET_F_CREATE);
            if (!storage)
                    return 1;

            /** 鍦ㄦ浣跨敤 'storage' **/

            return 1;
    }


鏈夊叧鍔熻兘绀轰緥锛岃鍙傞槄 `tools/testing/selftests/bpf` 鐩綍銆?
## 鍙傝€冭祫鏂?

https://lwn.net/ml/netdev/20190426171103.61892-1-kafai@fb.com/
