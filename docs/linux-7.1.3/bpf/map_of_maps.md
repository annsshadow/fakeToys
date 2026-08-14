## BPF_MAP_TYPE_ARRAY_OF_MAPS 与 BPF_MAP_TYPE_HASH_OF_MAPS


   - `BPF_MAP_TYPE_ARRAY_OF_MAPS` 和 `BPF_MAP_TYPE_HASH_OF_MAPS` 于内核版本 4.12 中引入

`BPF_MAP_TYPE_ARRAY_OF_MAPS` 和 `BPF_MAP_TYPE_HASH_OF_MAPS` 提供对“map 中嵌套 map”存储的通用支持。支持一层嵌套，其中外层 map 包含单一类型的内层 map 的实例，例如 `array_of_maps->sock_map`。

创建外层 map 时，使用一个内层 map 实例来初始化外层 map 持有的、关于其内层 map 的元数据。该内层 map 具有与外层 map 独立的生命周期，并且可以在外层 map 创建之后被删除。

外层 map 支持使用 syscall API 从用户空间进行元素查找、更新和删除。BPF 程序只允许在外层 map 中进行元素查找。

   - 不支持多级嵌套。
   - 任何 BPF map 类型都可以用作内层 map，除了 `BPF_MAP_TYPE_PROG_ARRAY`。
   - BPF 程序不能更新或删除外层 map 条目。

对于 `BPF_MAP_TYPE_ARRAY_OF_MAPS`，键是一个无符号 32 位整数索引，用于索引数组。该数组是固定大小的，具有 `max_entries` 个元素，在创建时零初始化。

对于 `BPF_MAP_TYPE_HASH_OF_MAPS`，键类型可以在定义 map 时选择。内核负责分配和释放键/值对，上限为你指定的 max_entries。默认情况下，哈希 map 使用哈希表元素的预分配。`BPF_F_NO_PREALLOC` 标志可用于在预分配过于耗费内存时禁用预分配。

## 用法


### 内核 BPF 辅助函数


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

内层 map 可以使用 `bpf_map_lookup_elem()` 辅助函数获取。该辅助函数返回指向内层 map 的指针，若未找到条目则返回 `NULL`。

## 示例


### 内核 BPF 示例


此代码片段展示了如何在 BPF 程序中创建并初始化一个 devmap 数组。注意，外层数组只能从用户空间使用 syscall API 修改。

    struct inner_map {
            __uint(type, BPF_MAP_TYPE_DEVMAP);
            __uint(max_entries, 10);
            __type(key, __u32);
            __type(value, __u32);
    } inner_map1 SEC(".maps"), inner_map2 SEC(".maps");

    struct {
            __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
            __uint(max_entries, 2);
            __type(key, __u32);
            __type(value, struct inner_map);
    } outer_map SEC(".maps") = {
            .values = { &inner_map1,
                        &inner_map2 }
    };

有关外层 map 声明式初始化的更多示例，请参见 `tools/testing/selftests/bpf` 中的 `progs/test_btf_map_in_map.c`。

### 用户空间


此代码片段展示了如何创建基于数组的外层 map：

    int create_outer_array(int inner_fd) {
            LIBBPF_OPTS(bpf_map_create_opts, opts, .inner_map_fd = inner_fd);
            int fd;

            fd = bpf_map_create(BPF_MAP_TYPE_ARRAY_OF_MAPS,
                                "example_array",       /** name **/
                                sizeof(__u32),         /** key size **/
                                sizeof(__u32),         /** value size **/
                                256,                   /** max entries **/
                                &opts);                /** create opts **/
            return fd;
    }

此代码片段展示了如何向内层 map 添加到一个外层 map：

    int add_devmap(int outer_fd, int index, const char *name) {
            int fd;

            fd = bpf_map_create(BPF_MAP_TYPE_DEVMAP, name,
                                sizeof(__u32), sizeof(__u32), 256, NULL);
            if (fd < 0)
                    return fd;

            return bpf_map_update_elem(outer_fd, &index, &fd, BPF_ANY);
    }

## 参考资料


- https://lore.kernel.org/netdev/20170322170035.923581-3-kafai@fb.com/
- https://lore.kernel.org/netdev/20170322170035.923581-4-kafai@fb.com/
