
## BPF_MAP_TYPE_XSKMAP


   - `BPF_MAP_TYPE_XSKMAP` 鍦ㄥ唴鏍哥増鏈?4.18 涓紩鍏?
`BPF_MAP_TYPE_XSKMAP` 鐢ㄤ綔 XDP BPF 杈呭姪鍑芥暟 `bpf_redirect_map()` 鍜?`XDP_REDIRECT` 鍔ㄤ綔鐨勫悗绔槧灏勶紝绫讳技浜?'devmap' 鍜?'cpumap'銆?璇ユ槧灏勭被鍨嬪皢鍘熷 XDP 甯ч噸瀹氬悜鍒?`AF_XDP`_ 濂楁帴瀛楋紙XSK锛夛紝杩欐槸鍐呮牳涓竴绉嶆柊鐨勫湴鍧€鏃忥紝鍏佽灏嗗抚浠庨┍鍔ㄩ噸瀹氬悜鍒扮敤鎴风┖闂磋€屾棤闇€閬嶅巻瀹屾暣鐨勭綉缁滃崗璁爤銆備竴涓?AF_XDP 濂楁帴瀛楃粦瀹氬埌鍗曚釜 netdev 闃熷垪銆俋SK 鍒伴槦鍒楃殑鏄犲皠濡備笅鍥炬墍绀猴細


    +---------------------------------------------------+
    |     xsk A      |     xsk B       |      xsk C     |<---+ User space
    =========================================================|==========
    |    Queue 0     |     Queue 1     |     Queue 2    |    |  Kernel
    +---------------------------------------------------+    |
    |                  Netdev eth0                      |    |
    +---------------------------------------------------+    |
    |                            +=============+        |    |
    |                            | key |  xsk  |        |    |
    |  +---------+               +=============+        |    |
    |  |         |               |  0  | xsk A |        |    |
    |  |         |               +-------------+        |    |
    |  |         |               |  1  | xsk B |        |    |
    |  | BPF     |-- redirect -->+-------------+-------------+
    |  | prog    |               |  2  | xsk C |        |
    |  |         |               +-------------+        |
    |  |         |                                      |
    |  |         |                                      |
    |  +---------+                                      |
    |                                                   |
    +---------------------------------------------------+

    缁戝畾鍒版煇涓?<netdev/queue_id> 鐨?AF_XDP 濂楁帴瀛楀皢**鍙?*鎺ュ彈鏉ヨ嚜璇?<netdev/queue_id> 鐨?XDP 甯с€傚鏋?XDP 绋嬪簭璇曞浘浠庡鎺ュ瓧缁戝畾鐨?<netdev/queue_id> 涔嬪鐨勯槦鍒楄繘琛岄噸瀹氬悜锛岃甯у皢涓嶄細鍦ㄥ鎺ュ瓧涓婅鎺ユ敹銆?
閫氬父姣忎釜 netdev 鍒涘缓涓€涓?XSKMAP銆傝鏄犲皠鍖呭惈涓€涓?XSK 鏂囦欢鎻忚堪绗︼紙FD锛夋暟缁勩€傛暟缁勫厓绱犵殑鏁伴噺閫氬父閫氳繃 `max_entries` 鏄犲皠鍙傛暟璁剧疆鎴栬皟鏁淬€傚浜?AF_XDP锛宍max_entries` 绛変簬 netdev 鏀寔鐨勯槦鍒楁暟閲忋€?
    鏄犲皠鐨勯敭鍜屽€肩殑澶у皬閮藉繀椤讳负 4 瀛楄妭銆?
## 鐢ㄦ硶


### 鍐呮牳 BPF

##### bpf_redirect_map()


    long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

灏嗘暟鎹寘閲嶅畾鍚戝埌 `map` 涓储寮?`key` 澶勬墍寮曠敤鐨勭鐐广€?瀵逛簬 `BPF_MAP_TYPE_XSKMAP`锛岃鏄犲皠鍖呭惈缁戝畾鍒?netdev 闃熷垪鐨勫鎺ュ瓧鐨?XSK FD 寮曠敤銆?
    濡傛灉鏄犲皠鍦ㄦ煇涓储寮曞涓虹┖锛屽垯鏁版嵁鍖呰涓㈠純銆傝繖鎰忓懗鐫€蹇呴』鍔犺浇涓€涓?XDP 绋嬪簭锛屼笖 XSKMAP 涓嚦灏戝寘鍚竴涓?XSK锛屾墠鑳介€氳繃濂楁帴瀛楀皢浠讳綍娴侀噺閫佸埌鐢ㄦ埛绌洪棿銆?
##### bpf_map_lookup_elem()


    void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

绫诲瀷涓?`struct xdp_sock *` 鐨?XSK 鏉＄洰寮曠敤鍙互浣跨敤 `bpf_map_lookup_elem()` 杈呭姪鍑芥暟鑾峰彇銆?
### 鐢ㄦ埛绌洪棿

    XSK 鏉＄洰鍙兘浠庣敤鎴风┖闂存洿鏂?鍒犻櫎锛岃€屼笉鑳戒粠 BPF 绋嬪簭涓洿鏂?鍒犻櫎銆傚皾璇曚粠鍐呮牳 BPF 绋嬪簭璋冪敤杩欎簺鍑芥暟灏嗗鑷寸▼搴忓姞杞藉け璐ュ苟鍙戝嚭楠岃瘉鍣ㄨ鍛娿€?
##### bpf_map_update_elem()


	int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags)

鍙互浣跨敤 `bpf_map_update_elem()` 杈呭姪鍑芥暟娣诲姞鎴栨洿鏂?XSK 鏉＄洰銆?`key` 鍙傛暟绛変簬 XSK 鎵€缁戝畾鍒扮殑闃熷垪鐨?queue_id銆傝€?`value` 鍙傛暟鏄濂楁帴瀛楃殑 FD 鍊笺€?
鍦ㄥ簳灞傦紝XSKMAP 鐨勬洿鏂板嚱鏁颁娇鐢?XSK FD 鍊兼潵鑾峰彇鍏宠仈鐨?`struct xdp_sock` 瀹炰緥銆?
flags 鍙傛暟鍙互鏄互涓嬩箣涓€锛?
- BPF_ANY锛氬垱寤烘柊鍏冪礌鎴栨洿鏂扮幇鏈夊厓绱犮€?- BPF_NOEXIST锛氫粎鍦ㄥ厓绱犱笉瀛樺湪鏃跺垱寤烘柊鍏冪礌銆?- BPF_EXIST锛氭洿鏂扮幇鏈夊厓绱犮€?
##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value)

鎴愬姛鏃惰繑鍥?`struct xdp_sock *`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key)

鍙互浣跨敤 `bpf_map_delete_elem()` 杈呭姪鍑芥暟鍒犻櫎 XSK 鏉＄洰銆?璇ヨ緟鍔╁嚱鏁板湪鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
    褰?`libxdp`_ 鍒犻櫎涓€涓?XSK 鏃讹紝瀹冧篃浼氫粠 XSKMAP 涓Щ闄ゅ叧鑱旂殑濂楁帴瀛楁潯鐩€?
## 绀轰緥

### 鍐呮牳


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曞０鏄庝竴涓悕涓?`xsks_map` 鐨?`BPF_MAP_TYPE_XSKMAP`锛屼互鍙婂浣曞皢鏁版嵁鍖呴噸瀹氬悜鍒颁竴涓?XSK銆?

	struct {
		__uint(type, BPF_MAP_TYPE_XSKMAP);
		__type(key, __u32);
		__type(value, __u32);
		__uint(max_entries, 64);
	} xsks_map SEC(".maps");


	SEC("xdp")
	int xsk_redir_prog(struct xdp_md *ctx)
	{
		__u32 index = ctx->rx_queue_index;

		if (bpf_map_lookup_elem(&xsks_map, &index))
			return bpf_redirect_map(&xsks_map, index, 0);
		return XDP_PASS;
	}

### 鐢ㄦ埛绌洪棿


浠ヤ笅浠ｇ爜鐗囨灞曠ず浜嗗浣曠敤 XSK 鏉＄洰鏇存柊涓€涓?XSKMAP銆?

	int update_xsks_map(struct bpf_map *xsks_map, int queue_id, int xsk_fd)
	{
		int ret;

		ret = bpf_map_update_elem(bpf_map__fd(xsks_map), &queue_id, &xsk_fd, 0);
		if (ret < 0)
			fprintf(stderr, "Failed to update xsks_map: %s\n", strerror(errno));

		return ret;
	}

鍏充簬濡備綍鍒涘缓 AF_XDP 濂楁帴瀛楃殑绀轰緥锛岃鍙傞槄 `libxdp`_ 浠撳簱涓?`bpf-examples`_ 鐩綍涓嬬殑 AF_XDP-example 鍜?AF_XDP-forwarding 绋嬪簭銆?鍏充簬 AF_XDP 鎺ュ彛鐨勮缁嗚鏄庯紝璇峰弬闃咃細

- `libxdp-readme`_銆?- `AF_XDP`_ 鍐呮牳鏂囨。銆?
    浣跨敤 XSKMAP 鍜?AF_XDP 鏈€鍏ㄩ潰鐨勮祫婧愭槸 `libxdp`_銆?