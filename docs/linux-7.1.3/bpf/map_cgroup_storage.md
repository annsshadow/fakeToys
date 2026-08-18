
## BPF_MAP_TYPE_CGROUP_STORAGE


`BPF_MAP_TYPE_CGROUP_STORAGE` 鏄犲皠绫诲瀷琛ㄧず涓€涓湰鍦板浐瀹氬ぇ灏忕殑瀛樺偍銆傚畠浠呭湪鍚敤 `CONFIG_CGROUP_BPF` 鏃跺彲鐢紝骞朵笖浠呭闄勫姞鍒?cgroup 鐨勭▼搴忓彲鐢紱杩欎簺绋嬪簭鐢卞悓涓€涓?Kconfig 鎻愪緵銆傚瓨鍌ㄧ敱绋嬪簭鎵€闄勫姞鍒扮殑 cgroup 鏍囪瘑銆?
璇ユ槧灏勫湪 BPF 绋嬪簭鎵€闄勫姞鍒扮殑 cgroup 澶勬彁渚涙湰鍦板瓨鍌ㄣ€傚畠姣旈€氱敤鍝堝笇琛ㄦ彁渚涙洿蹇€熴€佹洿绠€鍗曠殑璁块棶锛屽悗鑰呴渶瑕佽繘琛屽搱甯岃〃鏌ユ壘锛屽苟瑕佹眰鐢ㄦ埛鑷繁璺熻釜娲诲姩鐨?cgroup銆?
鏈枃妗ｆ弿杩?`BPF_MAP_TYPE_CGROUP_STORAGE` 鏄犲皠绫诲瀷鐨勭敤娉曞拰璇箟銆傚畠鐨勯儴鍒嗚涓哄湪 Linux 5.9 涓彂鐢熶簡鍙樺寲锛屾湰鏂囨。灏嗘弿杩拌繖浜涘樊寮傘€?
## Usage


璇ユ槧灏勪娇鐢ㄧ殑閿被鍨嬩负 `__u64 cgroup_inode_id` 鎴?```

    struct bpf_cgroup_storage_key {
            __u64 cgroup_inode_id;
            __u32 attach_type;
    };

```
`cgroup_inode_id` 鏄?cgroup 鐩綍鐨?inode id銆俙attach_type` 鏄▼搴忕殑闄勫姞绫诲瀷銆?
Linux 5.9 澧炲姞浜嗗灏?`__u64 cgroup_inode_id` 绫诲瀷浣滀负閿被鍨嬬殑鏀寔銆傚綋浣跨敤璇ラ敭绫诲瀷鏃讹紝鐗瑰畾 cgroup 鍜屾槧灏勭殑鎵€鏈夐檮鍔犵被鍨嬪皢鍏变韩鍚屼竴涓瓨鍌ㄣ€傚惁鍒欙紝濡傛灉绫诲瀷鏄?`struct bpf_cgroup_storage_key`锛屽垯涓嶅悓闄勫姞绫诲瀷鐨勭▼搴忓皢琚殧绂诲苟鐪嬪埌涓嶅悓鐨勫瓨鍌ㄣ€?
```

    void *bpf_get_local_storage(void *map, u64 flags)

```
`flags` 涓烘湭鏉ヤ娇鐢ㄤ繚鐣欙紝蹇呴』涓?0銆?
娌℃湁闅愬紡鍚屾銆傚涓▼搴忓彲浠ヨ法涓嶅悓 CPU 璁块棶 `BPF_MAP_TYPE_CGROUP_STORAGE` 鐨勫瓨鍌紝鐢ㄦ埛搴斿綋鑷繁璐熻矗鍚屾銆俠pf 鍩虹璁炬柦鎻愪緵 `struct bpf_spin_lock` 鏉ュ悓姝ュ瓨鍌ㄣ€傚弬瑙?`tools/testing/selftests/bpf/progs/test_spin_lock.c`銆?
## Examples


```

    #include <bpf/bpf.h>

    struct {
            __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
            __type(key, struct bpf_cgroup_storage_key);
            __type(value, __u32);
    } cgroup_storage SEC(".maps");

    int program(struct __sk_buff *skb)
    {
            __u32 *ptr = bpf_get_local_storage(&cgroup_storage, 0);
            __sync_fetch_and_add(ptr, 1);

            return 0;
    }

```

```

    #include <linux/bpf.h>
    #include <linux/libbpf.h>

    __u32 map_lookup(struct bpf_map *map, __u64 cgrp, enum bpf_attach_type type)
    {
            struct bpf_cgroup_storage_key = {
                    .cgroup_inode_id = cgrp,
                    .attach_type = type,
            };
            __u32 value;
            bpf_map_lookup_elem(bpf_map__fd(map), &key, &value);
            // error checking omitted
            return value;
    }

```

```

    #include <bpf/bpf.h>

    struct {
            __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
            __type(key, __u64);
            __type(value, __u32);
    } cgroup_storage SEC(".maps");

    int program(struct __sk_buff *skb)
    {
            __u32 *ptr = bpf_get_local_storage(&cgroup_storage, 0);
            __sync_fetch_and_add(ptr, 1);

            return 0;
    }

```

```

    #include <linux/bpf.h>
    #include <linux/libbpf.h>

    __u32 map_lookup(struct bpf_map *map, __u64 cgrp, enum bpf_attach_type type)
    {
            __u32 value;
            bpf_map_lookup_elem(bpf_map__fd(map), &cgrp, &value);
            // error checking omitted
            return value;
    }

```
## Semantics


`BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE` 鏄鏄犲皠绫诲瀷鐨勪竴涓彉浣撱€傝繖涓瘡 CPU 鍙樹綋瀵规瘡涓瓨鍌ㄧ殑姣忎釜 CPU 鏈変笉鍚岀殑鍐呭瓨鍖哄煙銆傞潪姣?CPU 鍙樹綋瀵规瘡涓瓨鍌ㄦ湁鐩稿悓鐨勫唴瀛樺尯鍩熴€?
鍦?Linux 5.9 涔嬪墠锛屽瓨鍌ㄧ殑鐢熷懡鍛ㄦ湡涓ユ牸鏄瘡涓檮鍔狅紙per-attachment锛夛紝骞朵笖瀵逛簬鍗曚釜 `CGROUP_STORAGE` 鏄犲皠锛屾渶澶氬彧鑳藉姞杞戒竴涓娇鐢ㄨ鏄犲皠鐨勭▼搴忋€備竴涓▼搴忓彲浠ラ檮鍔犲埌澶氫釜 cgroup锛屾垨鍏锋湁澶氱闄勫姞绫诲瀷锛屾瘡娆￠檮鍔犻兘浼氬垱寤轰竴涓叏鏂扮殑娓呴浂瀛樺偍銆傚瓨鍌ㄥ湪琚垎绂伙紙detach锛夋椂閲婃斁銆?
鍦ㄥ姞杞介獙璇佹椂锛屾瘡绉嶇被鍨嬶紙姣?CPU 鍜岄潪姣?CPU锛夌殑鏄犲皠涓?BPF 绋嬪簭涔嬮棿瀛樺湪涓€涓€瀵瑰簲鍏崇郴銆傚洜姝わ紝姣忎釜鏄犲皠鍙兘琚竴涓?BPF 绋嬪簭浣跨敤锛屾瘡涓?BPF 绋嬪簭鍙兘浣跨敤姣忕绫诲瀷鐨勪竴涓瓨鍌ㄦ槧灏勩€傜敱浜庢槧灏勫彧鑳借涓€涓?BPF 绋嬪簭浣跨敤锛屼笌鍏朵粬 BPF 绋嬪簭鍏变韩璇?cgroup 鐨勫瓨鍌ㄦ槸涓嶅彲鑳界殑銆?
浠?Linux 5.9 璧凤紝瀛樺偍鍙互琚涓▼搴忓叡浜€傚綋绋嬪簭闄勫姞鍒?cgroup 鏃讹紝浠呭綋鏄犲皠涓皻鏈寘鍚 cgroup 鍜岄檮鍔犵被鍨嬪鐨勬潯鐩椂锛屽唴鏍告墠浼氬垱寤烘柊瀛樺偍锛屽惁鍒欐柊闄勫姞浼氬鐢ㄦ棫瀛樺偍銆傚鏋滄槧灏勬槸闄勫姞绫诲瀷鍏变韩鐨勶紙attach type shared锛夛紝鍒欏湪姣旇緝鏃剁洿鎺ュ拷鐣ラ檮鍔犵被鍨嬨€傚瓨鍌ㄤ粎鍦ㄦ槧灏勬垨鎵€闄勫姞鐨?cgroup 琚噴鏀炬椂鎵嶉噴鏀俱€傚垎绂讳笉浼氱洿鎺ラ噴鏀惧瓨鍌紝浣嗗彲鑳藉鑷村鏄犲皠鐨勫紩鐢ㄩ檷涓洪浂锛屼粠鑰岄棿鎺ラ噴鏀炬槧灏勪腑鐨勬墍鏈夊瓨鍌ㄣ€?
鏄犲皠涓嶄笌浠讳綍 BPF 绋嬪簭鍏宠仈锛屽洜姝や娇鍏变韩鎴愪负鍙兘銆備絾鏄紝BPF 绋嬪簭浠嶇劧鍙兘涓庢瘡绉嶇被鍨嬶紙姣?CPU 鍜岄潪姣?CPU锛夌殑涓€涓槧灏勫叧鑱斻€備竴涓?BPF 绋嬪簭涓嶈兘浣跨敤瓒呰繃涓€涓?`BPF_MAP_TYPE_CGROUP_STORAGE` 鎴栬秴杩囦竴涓?`BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE`銆?
鍦ㄦ墍鏈夌増鏈腑锛岀敤鎴风┖闂撮兘鍙互浣跨敤 cgroup 鍜岄檮鍔犵被鍨嬭繖瀵瑰弬鏁帮紙鍦?`struct bpf_cgroup_storage_key` 涓級浣滀负閿紝閫氳繃 BPF 鏄犲皠 API 璇诲彇鎴栨洿鏂扮粰瀹氶檮鍔犵殑瀛樺偍銆傚浜?Linux 5.9 鐨勯檮鍔犵被鍨嬪叡浜瓨鍌紝姣旇緝鏃朵粎浣跨敤缁撴瀯浣撲腑鐨勭涓€涓€硷紝鍗?cgroup inode id锛屽洜姝ょ敤鎴风┖闂村彲浠ョ洿鎺ユ寚瀹氫竴涓?`__u64`銆?
瀛樺偍鍦ㄩ檮鍔犳椂缁戝畾銆傚嵆浣跨▼搴忛檮鍔犲埌鐖剁骇骞跺湪瀛愮骇涓Е鍙戯紝瀛樺偍浠嶇劧灞炰簬鐖剁骇銆?
鐢ㄦ埛绌洪棿涓嶈兘鍦ㄦ槧灏勪腑鍒涘缓鏂版潯鐩垨鍒犻櫎宸叉湁鏉＄洰銆傜▼搴忔祴璇曡繍琛屾€绘槸浣跨敤涓存椂瀛樺偍銆?