
## BPF_MAP_TYPE_LPM_TRIE


   - `BPF_MAP_TYPE_LPM_TRIE` kernel 4.11 版本中引
`BPF_MAP_TYPE_LPM_TRIE` 提供了一种最长前缀匹配（longest prefix match）算法，可用于将 IP 地址与一组已存储的前缀进行匹配在内部，数据存储在由使用 `prefixlen,data` 对作为键的节点组成的不平trie 中。`data` 以大端（网络字节序）解释，因`data[^0^]` 存储最高有效字节
LPM trie 可以在创建时指定最大前缀长度，该长度必须8 的倍数，范围从 8 2048。用于查找和更新操作的键是一`struct bpf_lpm_trie_key_u8`，由 `max_prefixlen/8` 字节扩展
- 对于 IPv4 地址，data 长度4 字节
- 对于 IPv6 地址，data 长度16 字节

存储LPM trie 中的值类型可以是任意用户定义的类型
   创建类型`BPF_MAP_TYPE_LPM_TRIE` 的映射时，必须设`BPF_F_NO_PREALLOC` 标志
## Usage


### Kernel BPF


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

对于给定data 值，可以使用 `bpf_map_lookup_elem()` 辅助函数找到最长前缀条目。该辅助函数返回一个指向与最长匹`key` 关联的值的指针，如果未找到任何条目则返`NULL`
执行最长前缀查找时，`key` `prefixlen` 应设置为 `max_prefixlen`。例如，当搜索某IPv4 地址的最长前缀匹配时，`prefixlen` 应设置为 `32`
#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

可以使用 `bpf_map_update_elem()` 辅助函数添加或更新前缀条目。该辅助函数以原子方式替换已有的元素
`bpf_map_update_elem()` 成功时返`0`，失败时返回负的错误码
```
    flags 参数必须BPF_ANY、BPF_NOEXIST BPF_EXIST 之一，但该值会被忽略，从而给BPF_ANY 的语义
```
#### bpf_map_delete_elem()


   long bpf_map_delete_elem(struct bpf_map **map, const void **key)

可以使用 `bpf_map_delete_elem()` 辅助函数删除前缀条目。该辅助函数成功时返0，失败时返回负的错误码
### Userspace


来自用户空间的访问使用与上述同名、以 `fd` 标识映射libbpf API
#### bpf_map_get_next_key()


   int bpf_map_get_next_key (int fd, const void **cur_key, void **next_key)

用户空间程序可以使用 libbpf `bpf_map_get_next_key()` 函数遍历 LPM trie 中的条目。可以通过`cur_key` 设置`NULL` 来调`bpf_map_get_next_key()` 获取第一个键。后续调用将获取当前键之后的下一个键。`bpf_map_get_next_key()` 成功时返`0`；如`cur_key` trie 中的最后一个键则返`-ENOENT`；失败时返回负的错误码
`bpf_map_get_next_key()` 将从最左侧的叶子开始遍LPM trie 元素。这意味着迭代会先返回更具体的键，然后才是更不具体的键
## Examples


LPM trie 在用户空间的使用示例，请参阅 `tools/testing/selftests/bpf/test_lpm_map.c`。下面的代码片段演示API 用法
### Kernel BPF


以下 BPF 代码片段展示了如何为 IPv4 地址前缀声明一个新LPM trie

    #include <linux/bpf.h>
    #include <bpf/bpf_helpers.h>

    struct ipv4_lpm_key {
            __u32 prefixlen;
            __u32 data;
    };

    struct {
            __uint(type, BPF_MAP_TYPE_LPM_TRIE);
            __type(key, struct ipv4_lpm_key);
            __type(value, __u32);
            __uint(map_flags, BPF_F_NO_PREALLOC);
            __uint(max_entries, 255);
    } ipv4_lpm_map SEC(".maps");

以下 BPF 代码片段展示了如何按 IPv4 地址查找

    void *lookup(__u32 ipaddr)
    {
            struct ipv4_lpm_key key = {
                    .prefixlen = 32,
                    .data = ipaddr
            };

            return bpf_map_lookup_elem(&ipv4_lpm_map, &key);
    }

### Userspace


以下代码片段展示了如何向 LPM trie 插入一IPv4 前缀条目

    int add_prefix_entry(int lpm_fd, __u32 addr, __u32 prefixlen, struct value *value)
    {
            struct ipv4_lpm_key ipv4_key = {
                    .prefixlen = prefixlen,
                    .data = addr
            };
            return bpf_map_update_elem(lpm_fd, &ipv4_key, value, BPF_ANY);
    }

以下代码片段展示了一个遍LPM trie 条目的用户空间程序：



    #include <bpf/libbpf.h>
    #include <bpf/bpf.h>

    void iterate_lpm_trie(int map_fd)
    {
            struct ipv4_lpm_key *cur_key = NULL;
            struct ipv4_lpm_key next_key;
            struct value value;
            int err;

            for (;;) {
                    err = bpf_map_get_next_key(map_fd, cur_key, &next_key);
                    if (err)
                            break;

                    bpf_map_lookup_elem(map_fd, &next_key, &value);

                    /** 在此处使key value **/

                    cur_key = &next_key;
            }
    }
