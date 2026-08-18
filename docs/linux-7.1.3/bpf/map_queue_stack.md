
## BPF_MAP_TYPE_QUEUE 涓?BPF_MAP_TYPE_STACK


   - `BPF_MAP_TYPE_QUEUE` 涓?`BPF_MAP_TYPE_STACK` 鍦ㄥ唴鏍哥増鏈?4.20 涓紩鍏?
`BPF_MAP_TYPE_QUEUE` 涓?BPF 绋嬪簭鎻愪緵 FIFO锛堝厛杩涘厛鍑猴級瀛樺偍锛宍BPF_MAP_TYPE_STACK` 鎻愪緵 LIFO
锛堝悗杩涘厛鍑猴級瀛樺偍銆傝繖浜涙槧灏勬敮鎸?peek銆乸op 涓?push 鎿嶄綔锛岄€氳繃鐩稿簲鐨勮緟鍔╁嚱鏁版毚闇茬粰 BPF 绋嬪簭銆?杩欎簺鎿嶄綔閫氳繃鐜版湁鐨?`bpf` 绯荤粺璋冪敤浠ヤ笅鍒楁柟寮忔毚闇茬粰鐢ㄦ埛绌洪棿搴旂敤绋嬪簭锛?
- `BPF_MAP_LOOKUP_ELEM` -> peek锛堟煡鐪嬶級
- `BPF_MAP_LOOKUP_AND_DELETE_ELEM` -> pop锛堝脊鍑猴級
- `BPF_MAP_UPDATE_ELEM` -> push锛堝帇鍏ワ級

`BPF_MAP_TYPE_QUEUE` 涓?`BPF_MAP_TYPE_STACK` 涓嶆敮鎸?`BPF_F_NO_PREALLOC`銆?
## 鐢ㄦ硶


### 鍐呮牳 BPF


#### bpf_map_push_elem()


   long bpf_map_push_elem(struct bpf_map **map, const void **value, u64 flags)

鍙娇鐢?`bpf_map_push_elem` 杈呭姪鍑芥暟灏嗕竴涓厓绱?`value` 娣诲姞鍒伴槦鍒楁垨鏍堛€傚繀椤诲皢 `flags` 鍙傛暟
璁句负 `BPF_ANY` 鎴?`BPF_EXIST`銆傚鏋滃皢 `flags` 璁句负 `BPF_EXIST`锛屽垯褰撻槦鍒楁垨鏍堝凡婊℃椂锛屽皢绉婚櫎
鏈€鏃х殑鍏冪礌浠ヨ吘鍑虹┖闂存坊鍔?`value`銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_peek_elem()


   long bpf_map_peek_elem(struct bpf_map **map, void **value)

璇ヨ緟鍔╁嚱鏁颁粠闃熷垪鎴栨爤涓幏鍙栦竴涓厓绱?`value` 鑰屼笉绉婚櫎瀹冦€傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_pop_elem()


   long bpf_map_pop_elem(struct bpf_map **map, void **value)

璇ヨ緟鍔╁嚱鏁板皢涓€涓厓绱犵Щ闄ゅ埌 `value` 涓紝浠庨槦鍒楁垨鏍堜腑寮瑰嚭銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?

### 鐢ㄦ埛绌洪棿


#### bpf_map_update_elem()


   int bpf_map_update_elem (int fd, const void **key, const void **value, __u64 flags)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_update_elem` 鍑芥暟灏?`value` 鍘嬪叆闃熷垪鎴栨爤銆俙key` 鍙傛暟
蹇呴』璁句负 `NULL`锛屼笖 `flags` 蹇呴』璁句负 `BPF_ANY` 鎴?`BPF_EXIST`锛岃涔変笌 `bpf_map_push_elem`
鍐呮牳杈呭姪鍑芥暟鐩稿悓銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem (int fd, const void **key, void **value)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_lookup_elem` 鍑芥暟鏌ョ湅闃熷垪鎴栨爤澶撮儴锛坔ead锛夌殑 `value`銆?`key` 鍙傛暟蹇呴』璁句负 `NULL`銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
#### bpf_map_lookup_and_delete_elem()


   int bpf_map_lookup_and_delete_elem (int fd, const void **key, void **value)

鐢ㄦ埛绌洪棿绋嬪簭鍙互浣跨敤 libbpf 鐨?`bpf_map_lookup_and_delete_elem` 鍑芥暟浠庨槦鍒楁垨鏍堝ご閮ㄥ脊鍑轰竴涓?`value`銆俙key` 鍙傛暟蹇呴』璁句负 `NULL`銆傛垚鍔熸椂杩斿洖 `0`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
## 绀轰緥


### 鍐呮牳 BPF


浠ヤ笅鐗囨灞曠ず濡備綍鍦?BPF 绋嬪簭涓０鏄庝竴涓槦鍒楋細


    struct {
            __uint(type, BPF_MAP_TYPE_QUEUE);
            __type(value, __u32);
            __uint(max_entries, 10);
    } queue SEC(".maps");


### 鐢ㄦ埛绌洪棿


浠ヤ笅鐗囨灞曠ず濡備綍浣跨敤 libbpf 鐨勪綆绾?API 浠庣敤鎴风┖闂村垱寤轰竴涓槦鍒楋細


    int create_queue()
    {
            return bpf_map_create(BPF_MAP_TYPE_QUEUE,
                                  "sample_queue", /** name **/
                                  0,              /** key size, must be zero **/
                                  sizeof(__u32),  /** value size **/
                                  10,             /** max entries **/
                                  NULL);          /** create options **/
    }


## 鍙傝€?

https://lwn.net/ml/netdev/153986858555.9127.14517764371945179514.stgit@kernel/
