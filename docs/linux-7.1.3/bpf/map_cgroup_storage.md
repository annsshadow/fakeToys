
## BPF_MAP_TYPE_CGROUP_STORAGE


`BPF_MAP_TYPE_CGROUP_STORAGE` 映射类型表示一个本地固定大小的存储。它仅在启用 `CONFIG_CGROUP_BPF` 时可用，并且仅对附加cgroup 的程序可用；这些程序由同一Kconfig 提供。存储由程序所附加到的 cgroup 标识
该映射在 BPF 程序所附加到的 cgroup 处提供本地存储。它比通用哈希表提供更快速、更简单的访问，后者需要进行哈希表查找，并要求用户自己跟踪活动cgroup
本文档描`BPF_MAP_TYPE_CGROUP_STORAGE` 映射类型的用法和语义。它的部分行为在 Linux 5.9 中发生了变化，本文档将描述这些差异
## Usage


该映射使用的键类型为 `__u64 cgroup_inode_id` ```

    struct bpf_cgroup_storage_key {
            __u64 cgroup_inode_id;
            __u32 attach_type;
    };

```
`cgroup_inode_id` cgroup 目录inode id。`attach_type` 是程序的附加类型
Linux 5.9 增加了对`__u64 cgroup_inode_id` 类型作为键类型的支持。当使用该键类型时，特定 cgroup 和映射的所有附加类型将共享同一个存储。否则，如果类型`struct bpf_cgroup_storage_key`，则不同附加类型的程序将被隔离并看到不同的存储
```

    void *bpf_get_local_storage(void *map, u64 flags)

```
`flags` 为未来使用保留，必须0
没有隐式同步。多个程序可以跨不同 CPU 访问 `BPF_MAP_TYPE_CGROUP_STORAGE` 的存储，用户应当自己负责同步。bpf 基础设施提供 `struct bpf_spin_lock` 来同步存储。参`tools/testing/selftests/bpf/progs/test_spin_lock.c`
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


`BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE` 是该映射类型的一个变体。这个每 CPU 变体对每个存储的每个 CPU 有不同的内存区域。非CPU 变体对每个存储有相同的内存区域
Linux 5.9 之前，存储的生命周期严格是每个附加（per-attachment），并且对于单个 `CGROUP_STORAGE` 映射，最多只能加载一个使用该映射的程序。一个程序可以附加到多个 cgroup，或具有多种附加类型，每次附加都会创建一个全新的清零存储。存储在被分离（detach）时释放
在加载验证时，每种类型（CPU 和非CPU）的映射BPF 程序之间存在一一对应关系。因此，每个映射只能被一BPF 程序使用，每BPF 程序只能使用每种类型的一个存储映射。由于映射只能被一BPF 程序使用，与其他 BPF 程序共享cgroup 的存储是不可能的
Linux 5.9 起，存储可以被多个程序共享。当程序附加cgroup 时，仅当映射中尚未包含该 cgroup 和附加类型对的条目时，内核才会创建新存储，否则新附加会复用旧存储。如果映射是附加类型共享的（attach type shared），则在比较时直接忽略附加类型。存储仅在映射或所附加cgroup 被释放时才释放。分离不会直接释放存储，但可能导致对映射的引用降为零，从而间接释放映射中的所有存储
映射不与任何 BPF 程序关联，因此使共享成为可能。但是，BPF 程序仍然只能与每种类型（CPU 和非CPU）的一个映射关联。一BPF 程序不能使用超过一`BPF_MAP_TYPE_CGROUP_STORAGE` 或超过一`BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE`
在所有版本中，用户空间都可以使用 cgroup 和附加类型这对参数（`struct bpf_cgroup_storage_key` 中）作为键，通过 BPF 映射 API 读取或更新给定附加的存储。对Linux 5.9 的附加类型共享存储，比较时仅使用结构体中的第一个值，cgroup inode id，因此用户空间可以直接指定一`__u64`
存储在附加时绑定。即使程序附加到父级并在子级中触发，存储仍然属于父级
用户空间不能在映射中创建新条目或删除已有条目。程序测试运行总是使用临时存储