
## BPF_MAP_TYPE_CGRP_STORAGE


`BPF_MAP_TYPE_CGRP_STORAGE` 映射类型表示 cgroup 的一种本地定长存储。它仅在启用
`CONFIG_CGROUPS` 时可用。这些程序由同一Kconfig 提供。特cgroup 的数据可以通过用该
cgroup 查找该映射来获取
本文档描述了 `BPF_MAP_TYPE_CGRP_STORAGE` 映射类型的用法与语义
## 用法


映射的键必须`sizeof(int)`，表示一cgroup fd```

    void *bpf_cgrp_storage_get(struct bpf_map *map, struct cgroup *cgroup, void *value, u64 flags)

```
`flags` 可以0 `BPF_LOCAL_STORAGE_GET_F_CREATE`，后者表示如果尚不存在则创建一个新本地存储
```

    long bpf_cgrp_storage_delete(struct bpf_map *map, struct cgroup *cgroup)

```
该映射对所有程序类型均可用
## 示例


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
## BPF_MAP_TYPE_CGRP_STORAGE BPF_MAP_TYPE_CGROUP_STORAGE 的区

旧的 cgroup 存储映射 `BPF_MAP_TYPE_CGROUP_STORAGE` 已被标记为废弃（重命名为
`BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED`）。应改用新的 `BPF_MAP_TYPE_CGRP_STORAGE` 映射以下说明`BPF_MAP_TYPE_CGRP_STORAGE` `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 的主要区别
(1). `BPF_MAP_TYPE_CGRP_STORAGE` 可被所有程序类型使用，     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 仅对 cgroup 程序类型（如
     BPF_CGROUP_INET_INGRESS BPF_CGROUP_SOCK_OPS 等）可用
(2). `BPF_MAP_TYPE_CGRP_STORAGE` 支持对多于一cgroup 的本地存储，     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 仅支持一个由 BPF 程序附加cgroup
(3). `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 在附加时分配本地存储，因     `bpf_get_local_storage()` 总是返回NULL 的本地存储     `BPF_MAP_TYPE_CGRP_STORAGE` 在运行时分配本地存储，因`bpf_cgrp_storage_get()`
     有可能返NULL 的本地存储。为避免此类 NULL 本地存储问题，用户空间可以在 BPF 程序
     附加之前通过 `bpf_map_update_elem()` 预先分配本地存储
(4). `BPF_MAP_TYPE_CGRP_STORAGE` 支持BPF 程序删除本地存储，     `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 仅在程序分离（detach）时删除存储
总体而言，`BPF_MAP_TYPE_CGRP_STORAGE` 支持 `BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED` 的所功能并有所扩展。建议使`BPF_MAP_TYPE_CGRP_STORAGE` 而非
`BPF_MAP_TYPE_CGROUP_STORAGE_DEPRECATED`銆?