
## BPF_MAP_TYPE_BLOOM_FILTER


   - `BPF_MAP_TYPE_BLOOM_FILTER` 5.16 版内核中引入

`BPF_MAP_TYPE_BLOOM_FILTER` 提供了一BPF 布隆过滤器（bloom filter）映射。布隆过滤器
是一种空间高效的概率性数据结构，用于快速判断某个元素是否存在于一个集合中。在布隆过滤器中可能出现假阳性（false positive），但不会出现假阴性（false negative）
布隆过滤器映射没有键（key），只有值（value）。创建布隆过滤器映射时，必须`key_size` 0
来创建。布隆过滤器映射支持两种操作
- push：向映射中添加一个元- peek：判断某个元素是否存在于映射
BPF 程序必须使用 `bpf_map_push_elem` 来向布隆过滤器映射添加元素，使用 `bpf_map_peek_elem`
来查询映射。这些操作通过已有`bpf` 系统调用以下列方式暴露给用户空间应用程序
- `BPF_MAP_UPDATE_ELEM` -> push
- `BPF_MAP_LOOKUP_ELEM` -> peek

创建映射时指定的 `max_entries` 大小用于为布隆过滤器估算一个合理的位图大小，除此之外并不被
严格强制。如果用户希望向布隆过滤器中插入`max_entries` 更多的条目，可能会导致更高的假阳性率
布隆过滤器使用的哈希数量可在创建映射时通过 `union bpf_attr` `map_extra` 的低 4 位来配置如果未指定数量，默认使用 5 个哈希函数。一般而言，使用更多的哈希会降低假阳性率，但也会降低
查找速度
无法从布隆过滤器映射中删除元素。布隆过滤器映射可用作内部映射（inner map）。用户负责同并发的更新和查找，以确保不会发生假阴性查找
## 用法


### 内核 BPF


#### bpf_map_push_elem()


   long bpf_map_push_elem(struct bpf_map **map, const void **value, u64 flags)

可以使用 `bpf_map_push_elem()` 辅助函数向布隆过滤器添加一`value`。向布隆过滤器添加条目时`flags` 参数必须设为 `BPF_ANY`。该辅助函数在成功时返回 `0`，失败时返回负的错误码
#### bpf_map_peek_elem()


   long bpf_map_peek_elem(struct bpf_map **map, void **value)

`bpf_map_peek_elem()` 辅助函数用于判断 `value` 是否存在于布隆过滤器映射中。如`value` 很可存在于映射中，该辅助函数返回 `0`；如`value` 一定不存在于映射中，则返回 `-ENOENT`
### 用户空间


#### bpf_map_update_elem()


   int bpf_map_update_elem (int fd, const void **key, const void **value, __u64 flags)

用户空间程序可以使用 libbpf `bpf_map_update_elem` 函数向布隆过滤器添加一`value`。`key`
参数必须设为 `NULL`，`flags` 必须设为 `BPF_ANY`。成功时返回 `0`，失败时返回负的错误码
#### bpf_map_lookup_elem()


   int bpf_map_lookup_elem (int fd, const void **key, void **value)

用户空间程序可以使用 libbpf `bpf_map_lookup_elem` 函数判断 `value` 是否存在于布隆过滤器中`key` 参数必须设为 `NULL`。如`value` 很可能存在于映射中返`0`，如`value` 一定不存在映射中则返回 `-ENOENT`
## 示例


### 内核 BPF


此片段展示了如何BPF 程序中声明一个布隆过滤器

    struct {
            __uint(type, BPF_MAP_TYPE_BLOOM_FILTER);
            __type(value, __u32);
            __uint(max_entries, 1000);
            __uint(map_extra, 3);
    } bloom_filter SEC(".maps");

此片段展示了如何BPF 程序中判断布隆过滤器中某个值是否存在：


    void *lookup(__u32 key)
    {
            if (bpf_map_peek_elem(&bloom_filter, &key) == 0) {
                    /* 验证不是假阳性，并使用次级查找（例如在哈希表中）
                     - 获取关联的                     */
                    return bpf_map_lookup_elem(&hash_table, &key);
            }
            return 0;
    }

### 用户空间


此片段展示了如何使用 libbpf 从用户空间创建一个布隆过滤器映射

    int create_bloom()
    {
            LIBBPF_OPTS(bpf_map_create_opts, opts,
                        .map_extra = 3);             /** 哈希数量 **/

            return bpf_map_create(BPF_MAP_TYPE_BLOOM_FILTER,
                                  "ipv6_bloom",      /** 名称 **/
                                  0,                 /** 键大小，必须0 **/
                                  sizeof(ipv6_addr), /** 鍊煎ぇ灏?**/
                                  10000,             /** 最大条目数 **/
                                  &opts);            /** 创建选项 **/
    }

此片段展示了如何从用户空间向布隆过滤器添加一个元素：


    int add_element(struct bpf_map *bloom_map, __u32 value)
    {
            int bloom_fd = bpf_map__fd(bloom_map);
            return bpf_map_update_elem(bloom_fd, NULL, &value, BPF_ANY);
    }

## 参考资

https://lwn.net/ml/bpf/20210831225005.2762202-1-joannekoong@fb.com/
