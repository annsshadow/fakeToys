
## BPF_MAP_TYPE_HASH，及其 PERCPU 与 LRU 变体


   - `BPF_MAP_TYPE_HASH` 在内核版本 3.19 中引入
   - `BPF_MAP_TYPE_PERCPU_HASH` 在版本 4.6 中引入
   - `BPF_MAP_TYPE_LRU_HASH` 和 `BPF_MAP_TYPE_LRU_PERCPU_HASH`
     均在版本 4.10 中引入

`BPF_MAP_TYPE_HASH` 和 `BPF_MAP_TYPE_PERCPU_HASH` 提供通用的哈希映射存储。键和值都可以是结构体，从而允许复合键和值。

内核负责分配和释放键值对，上限为您指定的 max_entries。哈希映射默认预分配哈希表元素。当预分配的内存开销过大时，可以使用 `BPF_F_NO_PREALLOC` 标志来禁用预分配。

`BPF_MAP_TYPE_PERCPU_HASH` 为每个 CPU 提供一个独立的值槽位。每 CPU 的值在内部以数组形式存储。

`BPF_MAP_TYPE_LRU_HASH` 和 `BPF_MAP_TYPE_LRU_PERCPU_HASH` 变体为各自的哈希表增加了 LRU 语义。当哈希表达到容量时，LRU 哈希会自动驱逐最近最少使用的条目。LRU 哈希维护一个内部的 LRU 列表，用于选择要驱逐的元素。这个内部 LRU 列表在 CPU 之间共享，但在调用 `bpf_map_create` 时，可以通过 `BPF_F_NO_COMMON_LRU` 标志请求一个每 CPU 的 LRU 列表。下表根据映射类型以及用于创建映射的标志，概述了 LRU 映射的属性。

======================== ========================= ================================
Flag                     `BPF_MAP_TYPE_LRU_HASH` `BPF_MAP_TYPE_LRU_PERCPU_HASH`
======================== ========================= ================================
**BPF_F_NO_COMMON_LRU**  每 CPU LRU，全局映射       每 CPU LRU，每 CPU 映射
**!BPF_F_NO_COMMON_LRU** 全局 LRU，全局映射        全局 LRU，每 CPU 映射
======================== ========================= ================================

## 用法


### 内核 BPF


#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

哈希条目可以使用 `bpf_map_update_elem()` 辅助函数添加或更新。该辅助函数以原子方式替换已有元素。`flags` 参数可用于控制更新行为：

- `BPF_ANY` 将创建新元素或更新已有元素
- `BPF_NOEXIST` 仅当元素尚不存在时才创建新元素
- `BPF_EXIST` 将更新已有元素

`bpf_map_update_elem()` 成功时返回 0，失败时返回负的错误码。

#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

哈希条目可以使用 `bpf_map_lookup_elem()` 辅助函数检索。该辅助函数返回与 `key` 关联的值的指针，如果未找到条目则返回 `NULL`。

#### bpf_map_delete_elem()


   long bpf_map_delete_elem(struct bpf_map **map, const void **key)

哈希条目可以使用 `bpf_map_delete_elem()` 辅助函数删除。该辅助函数成功时返回 0，失败时返回负的错误码。

### 每 CPU 哈希


对于 `BPF_MAP_TYPE_PERCPU_HASH` 和 `BPF_MAP_TYPE_LRU_PERCPU_HASH`，`bpf_map_update_elem()` 和 `bpf_map_lookup_elem()` 辅助函数会自动访问当前 CPU 的哈希槽位。

#### bpf_map_lookup_percpu_elem()


   void **bpf_map_lookup_percpu_elem(struct bpf_map **map, const void *key, u32 cpu)

`bpf_map_lookup_percpu_elem()` 辅助函数可用于查找特定 CPU 的哈希槽位中的值。返回 `cpu` 上与 `key` 关联的值，如果未找到条目或 `cpu` 无效则返回 `NULL`。

### 并发


存储在 `BPF_MAP_TYPE_HASH` 中的值可以被运行在不同 CPU 上的程序并发访问。自内核版本 5.1 起，BPF 基础设施提供 `struct bpf_spin_lock` 来同步访问。
参见 `tools/testing/selftests/bpf/progs/test_spin_lock.c`。

### 用户空间


#### bpf_map_get_next_key()


   int bpf_map_get_next_key(int fd, const void **cur_key, void **next_key)

在用户空间中，可以使用 libbpf 的 `bpf_map_get_next_key()` 函数遍历哈希的键。可以通过将 `cur_key` 设为 `NULL` 来调用 `bpf_map_get_next_key()` 获取第一个键。后续调用将获取当前键之后的下一个键。`bpf_map_get_next_key()` 成功时返回 0，如果 cur_key 是哈希中的最后一个键则返回 -ENOENT，失败时返回负的错误码。

请注意，如果 `cur_key` 被删除，那么 `bpf_map_get_next_key()` 反而会返回哈希表中的**第一个**键，这是不理想的。如果会在 `bpf_map_get_next_key()` 操作过程中混合进行键删除，建议使用批量查找。

## 示例


请参阅 `tools/testing/selftests/bpf` 目录中的功能示例。下面的代码片段演示了 API 用法。

此示例展示如何声明一个带有结构体键和结构体值的 LRU Hash。


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

此示例展示如何使用原子指令创建或更新哈希值：


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

在用户空间中遍历上面声明的映射元素：


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

                    // 在此处使用键和值

                    cur_key = &next_key;
            }
    }

## 内部实现


本文档的这一部分面向 Linux 开发者，描述了不被视作稳定 ABI 的映射实现细节。以下细节可能在未来的内核版本中发生变化。

### ``BPF_MAP_TYPE_LRU_HASH`` 及其变体


更新 LRU 映射中的元素时，当映射容量达到上限，可能会触发驱逐行为。更新算法会尝试若干步骤以强制执行 LRU 属性，这些步骤对后续操作尝试中所涉及的其它 CPU 的影响越来越大：

- 尝试使用 CPU 本地状态来批量操作
- 尝试从全局列表获取 `target_free` 个空闲节点
- 尝试从全局列表拉取任意节点并将其从哈希映射中移除
- 尝试从任意 CPU 的列表中拉取任意节点并将其从哈希映射中移除

批量从全局列表借用的节点数量 `target_free` 取决于映射的大小。较大的批量大小可减少锁竞争，但也可能耗尽全局结构。该值在映射初始化时计算，以避免耗尽——方法是将所有 CPU 的聚合预留限制为映射大小的一半。最小值为单个元素，最大预算为一次 128 个。

该算法在下图中直观地描述。有关相应操作的完整解释，请参见提交 3a08c2fd7634（“bpf: LRU List”）：

   :alt:    描述映射更新期间所采取的 LRU 驱逐步骤的图示。

   LRU 哈希在映射更新期间的驱逐，针对 `BPF_MAP_TYPE_LRU_HASH` 及其
   变体。有关内核函数名称代码引用的点文件源，请参见对应的 dot 文件。

映射更新从右上角的椭圆 “begin `bpf_map_update()`” 开始，并沿着图向下推进，最终结果可能是更新成功，也可能是带有各种错误码的失败。右上角的图例指示了哪些锁可能参与特定操作。这旨在作为一个直观提示，用于推理映射竞争如何影响更新操作，不过基于上表描述的逻辑，映射类型和标志可能会影响这些锁上的实际竞争。例如，如果映射以类型 `BPF_MAP_TYPE_LRU_PERCPU_HASH` 和标志 `BPF_F_NO_COMMON_LRU` 创建，那么所有映射属性都将是每 CPU 的。
