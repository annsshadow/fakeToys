
## BPF maps


BPF 鐨?鈥渕aps鈥濓紙鏄犲皠锛夋彁渚涢€氱敤瀛樺偍锛屽彲鍦ㄥ唴鏍镐笌鐢ㄦ埛绌洪棿涔嬮棿鍏变韩涓嶅悓绫诲瀷鐨勬暟鎹€?鐜版湁鑻ュ共瀛樺偍绫诲瀷锛屽寘鎷搱甯岋紙hash锛夈€佹暟缁勶紙array锛夈€佸竷闅嗚繃婊ゅ櫒锛坆loom filter锛?涓庡熀鏁版爲锛坮adix-tree锛夈€傚叾涓嚑绉嶆槧灏勭被鍨嬬敤浜庢敮鎸佸熀浜庢槧灏勫唴瀹规墽琛屾搷浣滅殑鐗瑰畾
BPF 杈呭姪鍑芥暟銆傛槧灏勯€氳繃 BPF 杈呭姪鍑芥暟浠?BPF 绋嬪簭涓闂紝杩欎簺杈呭姪鍑芥暟鍦?`bpf-helpers(7)`_ 鐨?`man-pages`_ 涓湁鏂囨。璇存槑銆?
BPF 鏄犲皠閫氳繃 `bpf` 绯荤粺璋冪敤浠庣敤鎴风┖闂磋闂紝璇ョ郴缁熻皟鐢ㄦ彁渚涗簡鍒涘缓鏄犲皠銆佹煡鎵?鍏冪礌銆佹洿鏂板厓绱犱笌鍒犻櫎鍏冪礌鐨勫懡浠ゃ€傛湁鍏?BPF 绯荤粺璋冪敤鐨勬洿澶氱粏鑺傦紝璇峰弬闃?`ebpf-syscall`_ 浠ュ強 `bpf(2)`_ 鐨?`man-pages`_銆?
## 鏄犲皠绫诲瀷


- [map_*](map_*)

## 浣跨敤娉ㄦ剰


   int bpf(int command, union bpf_attr *attr, u32 size)

浣跨敤 `bpf()` 绯荤粺璋冪敤鏉ユ墽琛岀敱 `command` 鎸囧畾鐨勬搷浣溿€傝鎿嶄綔浣跨敤 `attr` 涓?鎻愪緵鐨勫弬鏁般€俙size` 鍙傛暟鏄?`attr` 涓?`union bpf_attr` 鐨勫ぇ灏忋€?
**BPF_MAP_CREATE**

浣跨敤 `attr` 涓湡鏈涚殑绫诲瀷涓庡睘鎬у垱寤轰竴涓槧灏勶細


    int fd;
    union bpf_attr attr = {
            .map_type = BPF_MAP_TYPE_ARRAY;  /** 蹇呭～ **/
            .key_size = sizeof(__u32);       /** 蹇呭～ **/
            .value_size = sizeof(__u32);     /** 蹇呭～ **/
            .max_entries = 256;              /** 蹇呭～ **/
            .map_flags = BPF_F_MMAPABLE;
            .map_name = "example_array";
    };

    fd = bpf(BPF_MAP_CREATE, &attr, sizeof(attr));

鎴愬姛鏃惰繑鍥炶繘绋嬫湰鍦扮殑鏂囦欢鎻忚堪绗︼紝澶辫触鏃惰繑鍥炶礋鐨勯敊璇爜銆傚彲浠ラ€氳繃璋冪敤
`close(fd)` 鍒犻櫎璇ユ槧灏勩€傜敱宸叉墦寮€鏂囦欢鎻忚堪绗︽寔鏈夌殑鏄犲皠浼氬湪杩涚▼閫€鍑烘椂
鑷姩鍒犻櫎銆?
   `'_'` and `'.'`銆?
**BPF_MAP_LOOKUP_ELEM**

浣跨敤 `attr->map_fd`銆乣attr->key`銆乣attr->value` 鍦ㄧ粰瀹氭槧灏勪腑鏌ユ壘閿€?鎴愬姛鏃惰繑鍥為浂骞跺皢鎵惧埌鐨勫厓绱犲瓨鍏?`attr->value`锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
**BPF_MAP_UPDATE_ELEM**

浣跨敤 `attr->map_fd`銆乣attr->key`銆乣attr->value` 鍦ㄧ粰瀹氭槧灏勪腑鍒涘缓鎴栨洿鏂?閿€煎銆傛垚鍔熸椂杩斿洖闆讹紝澶辫触鏃惰繑鍥炶礋鐨勯敊璇爜銆?
**BPF_MAP_DELETE_ELEM**

浣跨敤 `attr->map_fd`銆乣attr->key` 鍦ㄧ粰瀹氭槧灏勪腑鎸夐敭鏌ユ壘骞跺垹闄ゅ厓绱犮€傛垚鍔熸椂
杩斿洖闆讹紝澶辫触鏃惰繑鍥炶礋鐨勯敊璇爜銆?