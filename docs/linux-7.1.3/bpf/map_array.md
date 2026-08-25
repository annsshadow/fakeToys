
## BPF_MAP_TYPE_ARRAY and BPF_MAP_TYPE_PERCPU_ARRAY


   - `BPF_MAP_TYPE_ARRAY` kernel 3.19 版本中引   - `BPF_MAP_TYPE_PERCPU_ARRAY` 4.6 版本中引
`BPF_MAP_TYPE_ARRAY` `BPF_MAP_TYPE_PERCPU_ARRAY` 提供通用的数组存储。键类型是无符号 32 位整数（4 字节），映射的大小恒定。`max_entries` 在创建时定义了数组的大小。所有数组元素在创建时都会被预分配并零初始化。`BPF_MAP_TYPE_PERCPU_ARRAY` 为每CPU 使用不同的内存区域，`BPF_MAP_TYPE_ARRAY` 使用相同的内存区域。对`BPF_MAP_TYPE_ARRAY`，所存储的值可以是任意大小；而对`BPF_MAP_TYPE_PERCPU_ARRAY`，其值不得大`PCPU_MIN_UNIT_SIZE`2 kB）。所有数组元素都8 字节对齐
kernel 5.5 起，可通过设置 `BPF_F_MMAPABLE` 标志`BPF_MAP_TYPE_ARRAY` 启用内存映射。映射定义按页对齐，并从第一页开始。系统会分配足够数量的页大小、页对齐的内存块（从第二页开始）来存储所有数组值，在某些情况下这会导致内存的过度分配。这样做的好处是提升了性能并简化了使用，因为用户空间程序无需使用辅助函数来访问和修改数据
## Usage


### Kernel BPF


#### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

数组元素可使`bpf_map_lookup_elem()` 辅助函数检索。该辅助函数返回指向数组元素的指针，因此为避免与读取该值的用户空间发生数据竞争，用户在原地更新该值时应使`__sync_fetch_and_add()` 之类的原语
#### bpf_map_update_elem()


   long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

数组元素可使`bpf_map_update_elem()` 辅助函数更新
`bpf_map_update_elem()` 成功时返0，失败时返回负的错误码
由于数组大小恒定，因此不支持 `bpf_map_delete_elem()`。要清空某个数组元素，可以使`bpf_map_update_elem()` 向该索引插入一个零值
### Per CPU Array


`BPF_MAP_TYPE_ARRAY` 中存储的值可以被不同 CPU 上的多个程序访问。要将存储限制到单个 CPU，可以使`BPF_MAP_TYPE_PERCPU_ARRAY`
使用 `BPF_MAP_TYPE_PERCPU_ARRAY` 时，`bpf_map_update_elem()` `bpf_map_lookup_elem()` 辅助函数会自动访问当CPU 对应的槽位
#### bpf_map_lookup_percpu_elem()


   void **bpf_map_lookup_percpu_elem(struct bpf_map **map, const void *key, u32 cpu)

`bpf_map_lookup_percpu_elem()` 辅助函数可用于查找特CPU 的数组值。成功时返回值，如果未找到对应条目或 `cpu` 无效则返`NULL`
### Concurrency


kernel 5.1 版本起，BPF 基础设施提供 `struct bpf_spin_lock` 来同步访问
### Userspace


用户空间的访问使用与上述同名libbpf API，映射通过`fd` 标识
## Examples


功能示例请参`tools/testing/selftests/bpf` 目录。下面的代码示例演示API 的用法
### Kernel BPF


此代码片段展示了如何BPF 程序中声明一个数组

    struct {
            __uint(type, BPF_MAP_TYPE_ARRAY);
            __type(key, u32);
            __type(value, long);
            __uint(max_entries, 256);
    } my_map SEC(".maps");


此示BPF 程序展示了如何访问数组元素

    int bpf_prog(struct __sk_buff *skb)
    {
            struct iphdr ip;
            int index;
            long *value;

            if (bpf_skb_load_bytes(skb, ETH_HLEN, &ip, sizeof(ip)) < 0)
                    return 0;

            index = ip.protocol;
            value = bpf_map_lookup_elem(&my_map, &index);
            if (value)
                    __sync_fetch_and_add(value, skb->len);

            return 0;
    }

### Userspace


#### BPF_MAP_TYPE_ARRAY


此代码片段展示了如何使用 `bpf_map_create_opts` 设置标志来创建一个数组

    #include <bpf/libbpf.h>
    #include <bpf/bpf.h>

    int create_array()
    {
            int fd;
            LIBBPF_OPTS(bpf_map_create_opts, opts, .map_flags = BPF_F_MMAPABLE);

            fd = bpf_map_create(BPF_MAP_TYPE_ARRAY,
                                "example_array",       /** name **/
                                sizeof(__u32),         /** key size **/
                                sizeof(long),          /** value size **/
                                256,                   /** max entries **/
                                &opts);                /** create opts **/
            return fd;
    }

此代码片段展示了如何初始化数组的元素

    int initialize_array(int fd)
    {
            __u32 i;
            long value;
            int ret;

            for (i = 0; i < 256; i++) {
                    value = i;
                    ret = bpf_map_update_elem(fd, &i, &value, BPF_ANY);
                    if (ret < 0)
                            return ret;
            }

            return ret;
    }

此代码片段展示了如何从数组中检索元素值

    int lookup(int fd)
    {
            __u32 index = 42;
            long value;
            int ret;

            ret = bpf_map_lookup_elem(fd, &index, &value);
            if (ret < 0)
                    return ret;

            /** use value here **/
            assert(value == 42);

            return ret;
    }

#### BPF_MAP_TYPE_PERCPU_ARRAY


此代码片段展示了如何初始化每 CPU 数组的元素

    int initialize_array(int fd)
    {
            int ncpus = libbpf_num_possible_cpus();
            long values[ncpus];
            __u32 i, j;
            int ret;

            for (i = 0; i < 256 ; i++) {
                    for (j = 0; j < ncpus; j++)
                            values[j] = i;
                    ret = bpf_map_update_elem(fd, &i, &values, BPF_ANY);
                    if (ret < 0)
                            return ret;
            }

            return ret;
    }

此代码片段展示了如何访问数组值的CPU 元素

    int lookup(int fd)
    {
            int ncpus = libbpf_num_possible_cpus();
            __u32 index = 42, j;
            long values[ncpus];
            int ret;

            ret = bpf_map_lookup_elem(fd, &index, &values);
            if (ret < 0)
                    return ret;

            for (j = 0; j < ncpus; j++) {
                    /** Use per CPU value here **/
                    assert(values[j] == 42);
            }

            return ret;
    }

## Semantics


如上例所示，在用户空间访`BPF_MAP_TYPE_PERCPU_ARRAY` 时，每个值都是一个包`ncpus` 个元素的数组
调用 `bpf_map_update_elem()` 时，对于这些映射不能使用 `BPF_NOEXIST` 标志