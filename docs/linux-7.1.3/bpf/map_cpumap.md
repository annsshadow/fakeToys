
## BPF_MAP_TYPE_CPUMAP


   - `BPF_MAP_TYPE_CPUMAP` 4.15 版本的内核中被引
 :doc: cpu map

这种映射类型的一个示例用例是基于软件的接收端缩放（Receive Side Scaling，RSS）
CPUMAP 表示系统中的 CPU，以 map-key 为索引，map-value 是配置设置（每个 CPUMAP 条目）。每CPUMAP 条目都有一个专用于
给定 CPU 的内核线程，用以表示远程 CPU 执行单元
Linux 内核 5.9 版本开始，CPUMAP 可以在远CPU 运行第二XDP 程序。这允许 XDP 程序将其处理拆分多个 CPU 上。例如，这样一种场景：初始 CPU（看接收
数据包的那个）只需做最少的包处理，而远CPU（数据包
被定向到的那个）可以花费更多周期来处理该帧。初CPU
是执XDP redirect 程序的地方。远CPU 接收原始`xdp_frame` 对象
## 用法


### 内核 BPF


##### bpf_redirect_map()


     long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

将数据包重定向到 `map` 中索引为 `key` 的端点对于 `BPF_MAP_TYPE_CPUMAP`，该映射包含CPU 的引用
如果映射查找失败，`flags` 的低位两位将用作返回码这样返回值可以是调用者所选的、最高到 `XDP_TX` XDP 程序返回码之一
### 用户空间


    CPUMAP 条目只能从用户空间更查找/删除，而不    eBPF 程序中进行。试图从内核 eBPF 程序调用这些函数
    将导致程序加载失败并出现验证器（verifier）警告
##### bpf_map_update_elem()


    int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags);

可以使用 `bpf_map_update_elem()` helper 添加或更CPU 条目。该 helper 以原子方式替换现有元素。`value` 参数
可以`struct bpf_cpumap_val`
 .. code-block:: c

    struct bpf_cpumap_val {
        __u32 qsize;  /** queue size to remote target CPU **/
        union {
            int   fd; /** prog fd on map write **/
            __u32 id; /** prog id on map read **/
        } bpf_prog;
    };

flags 参数可以是以下之一  - BPF_ANY：创建一个新元素或更新一个已存在的元素  - BPF_NOEXIST：仅当元素不存在时才创建一个新元素  - BPF_EXIST：更新一个已存在的元素
##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value);

可以使用 `bpf_map_lookup_elem()` helper 检CPU 条目
##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key);

可以使用 `bpf_map_delete_elem()` helper 删除
CPU 条目。成功时helper 返回 0，失败时返回负的错误码
## 示例


### 内核


以下代码片段展示了如何声明一个名`cpu_map` `BPF_MAP_TYPE_CPUMAP`，以及如何使用轮询（round robin）方将数据包重定向到远程 CPU

   struct {
        __uint(type, BPF_MAP_TYPE_CPUMAP);
        __type(key, __u32);
        __type(value, struct bpf_cpumap_val);
        __uint(max_entries, 12);
    } cpu_map SEC(".maps");

    struct {
        __uint(type, BPF_MAP_TYPE_ARRAY);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 12);
    } cpus_available SEC(".maps");

    struct {
        __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 1);
    } cpus_iterator SEC(".maps");

    SEC("xdp")
    int  xdp_redir_cpu_round_robin(struct xdp_md *ctx)
    {
        __u32 key = 0;
        __u32 cpu_dest = 0;
        __u32 **cpu_selected, **cpu_iterator;
        __u32 cpu_idx;

        cpu_iterator = bpf_map_lookup_elem(&cpus_iterator, &key);
        if (!cpu_iterator)
            return XDP_ABORTED;
        cpu_idx = *cpu_iterator;

        *cpu_iterator += 1;
        if (*cpu_iterator == bpf_num_possible_cpus())
            *cpu_iterator = 0;

        cpu_selected = bpf_map_lookup_elem(&cpus_available, &cpu_idx);
        if (!cpu_selected)
            return XDP_ABORTED;
        cpu_dest = *cpu_selected;

        if (cpu_dest >= bpf_num_possible_cpus())
            return XDP_ABORTED;

        return bpf_redirect_map(&cpu_map, cpu_dest, 0);
    }

### 用户空间


以下代码片段展示了如何将 CPUMAP max_entries
动态设置为系统上可用的 CPU 最大数量

    int set_max_cpu_entries(struct bpf_map *cpu_map)
    {
        if (bpf_map__set_max_entries(cpu_map, libbpf_num_possible_cpus()) < 0) {
            fprintf(stderr, "Failed to set max entries for cpu_map map: %s",
                strerror(errno));
            return -1;
        }
        return 0;
    }

## 参

- https://developers.redhat.com/blog/2021/05/13/receive-side-scaling-rss-with-ebpf-and-cpumap#redirecting_into_a_cpumap
