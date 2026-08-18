
## BPF_MAP_TYPE_CGRP_STORAGE


`BPF_MAP_TYPE_CGRP_STORAGE` 鏄犲皠绫诲瀷琛ㄧず cgroup 鐨勪竴绉嶆湰鍦板畾闀垮瓨鍌ㄣ€傚畠浠呭湪鍚敤
`CONFIG_CGROUPS` 鏃跺彲鐢ㄣ€傝繖浜涚▼搴忕敱鍚屼竴涓?Kconfig 鎻愪緵銆傜壒瀹?cgroup 鐨勬暟鎹彲浠ラ€氳繃鐢ㄨ
cgroup 鏌ユ壘璇ユ槧灏勬潵鑾峰彇銆?
鏈枃妗ｆ弿杩颁簡 `BPF_MAP_TYPE_CGRP_STORAGE` 鏄犲皠绫诲瀷鐨勭敤娉曚笌璇箟銆?
## 鐢ㄦ硶


鏄犲皠鐨勯敭蹇呴』鏄?`sizeof(int)`锛岃〃绀轰竴涓?cgroup fd銆?```

    void *bpf_cgrp_storage_get(struct bpf_map *map, struct cgroup *cgroup, void *value, u64 flags)

```
`flags` 鍙互鏄?0 鎴?`BPF_LOCAL_STORAGE_GET_F_CREATE`锛屽悗鑰呰〃绀哄鏋滃皻涓嶅瓨鍦ㄥ垯鍒涘缓涓€涓柊鐨?鏈湴瀛樺偍銆?
```

    long bpf_cgrp_storage_delete(struct bpf_map *map, struct cgroup *cgroup)

```
璇ユ槧灏勫鎵€鏈夌▼搴忕被鍨嬪潎鍙敤銆?
## 绀轰緥


```

    #include <vmlinux.h>
    #include <bpf/bpf_helpers.h>
    #include <bpf/bpf_tracing.h>

    struct {
            __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
            __uint(map_flags, BPF_F_NO_PREALLOC);
            __type(key, int);
            __type(value, long);
    } cgrp_storage SEC(".maps");

    SEC("tp_btf/sys_enter")
    int BPF_PROG(on_enter, struct pt_regs *regs, long id)
    {
            struct task_struct *task = bpf_get_current_task_btf();
            long *ptr;

            ptr = bpf_cgrp_storage_get(&cgrp_storage, task->cgroups->dfl_cgrp, 0,
                                       BPF_LOCAL_STORAGE_GET_F_CREATE);
            if (ptr)
                __sync_fetch_and_add(ptr, 1);

            return 0;
    }

```
```

    #include <linux/bpf.h>
    #include <linux/libbpf.h>

    __u32 map_lookup(struct bpf_map *map, int cgrp_fd)
    {
            __u32 *value;
            value = bpf_map_lookup_elem(bpf_map__fd(map), &cgrp_fd);
            if (value)
                return *value;
            return 0;
    }

```
## BPF_MAP_TYPE_CGRP_STORAGE 涓?BPF_MAP_TYPE_CGROUP_STORAGE 鐨勫尯鍒?

鏃х殑 cgroup 瀛樺偍鏄犲皠 `BPF_MAP_TYPE_CGROUP_STORAGE` 宸茶鏍囪涓哄簾寮冿紙閲嶅懡鍚嶄负
`BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED`锛夈€傚簲鏀圭敤鏂扮殑 `BPF_MAP_TYPE_CGRP_STORAGE` 鏄犲皠銆?浠ヤ笅璇存槑浜?`BPF_MAP_TYPE_CGRP_STORAGE` 涓?`BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 鐨勪富瑕佸尯鍒€?
(1). `BPF_MAP_TYPE_CGRP_STORAGE` 鍙鎵€鏈夌▼搴忕被鍨嬩娇鐢紝鑰?     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 浠呭 cgroup 绋嬪簭绫诲瀷锛堝
     BPF_CGROUP_INET_INGRESS 鎴?BPF_CGROUP_SOCK_OPS 绛夛級鍙敤銆?
(2). `BPF_MAP_TYPE_CGRP_STORAGE` 鏀寔瀵瑰浜庝竴涓?cgroup 鐨勬湰鍦板瓨鍌紝鑰?     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 浠呮敮鎸佷竴涓敱 BPF 绋嬪簭闄勫姞鐨?cgroup銆?
(3). `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 鍦ㄩ檮鍔犳椂鍒嗛厤鏈湴瀛樺偍锛屽洜姝?     `bpf_get_local_storage()` 鎬绘槸杩斿洖闈?NULL 鐨勬湰鍦板瓨鍌ㄣ€?     `BPF_MAP_TYPE_CGRP_STORAGE` 鍦ㄨ繍琛屾椂鍒嗛厤鏈湴瀛樺偍锛屽洜姝?`bpf_cgrp_storage_get()`
     鏈夊彲鑳借繑鍥?NULL 鐨勬湰鍦板瓨鍌ㄣ€備负閬垮厤姝ょ被 NULL 鏈湴瀛樺偍闂锛岀敤鎴风┖闂村彲浠ュ湪 BPF 绋嬪簭
     闄勫姞涔嬪墠閫氳繃 `bpf_map_update_elem()` 棰勫厛鍒嗛厤鏈湴瀛樺偍銆?
(4). `BPF_MAP_TYPE_CGRP_STORAGE` 鏀寔鐢?BPF 绋嬪簭鍒犻櫎鏈湴瀛樺偍锛岃€?     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 浠呭湪绋嬪簭鍒嗙锛坉etach锛夋椂鍒犻櫎瀛樺偍銆?
鎬讳綋鑰岃█锛宍BPF_MAP_TYPE_CGRP_STORAGE` 鏀寔 `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 鐨勬墍鏈?鍔熻兘骞舵湁鎵€鎵╁睍銆傚缓璁娇鐢?`BPF_MAP_TYPE_CGRP_STORAGE` 鑰岄潪
`BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED`銆?