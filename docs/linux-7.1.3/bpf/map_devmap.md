## BPF_MAP_TYPE_DEVMAP 涓?BPF_MAP_TYPE_DEVMAP_HASH


   - `BPF_MAP_TYPE_DEVMAP` 在内核版4.14 中引   - `BPF_MAP_TYPE_DEVMAP_HASH` 在内核版5.4 中引
`BPF_MAP_TYPE_DEVMAP` `BPF_MAP_TYPE_DEVMAP_HASH` 是主要用XDP BPF 辅助调用 `bpf_redirect_map()` 后端映射BPF 映射。`BPF_MAP_TYPE_DEVMAP` 由一个数组支撑，该数组使用键（key）作为索引来查找对网络设备（net device）的引用。`BPF_MAP_TYPE_DEVMAP_HASH` 由一个哈希表支撑，该哈希表使用键来查找对网络设备的引用。用户提<`key`/ `ifindex`> <`key`/ `struct bpf_devmap_val`> 对来用新的网络设备更新映射
    - 哈希映射的键不必`ifindex`    - 虽然 `BPF_MAP_TYPE_DEVMAP_HASH` 允许对网络设备进行紧凑打包，但其代价是在执行查找时需要对键进行哈希
两种类型 devmap 的初始化和数据包入队/发送代码是共享的；只有查找和插入不同
## 用法（Usage

### 内核 BPF


##### bpf_redirect_map()


    long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

将数据包重定向到 `map` 中索引为 `key` 所引用的端点。对`BPF_MAP_TYPE_DEVMAP` `BPF_MAP_TYPE_DEVMAP_HASH`，该映射包含对网络设备（用于通过其他端口转发数据包）的引用
**flags** 的低两位用作映射查找失败时的返回码。这样返回值可以是调用者所选的、最高到 `XDP_TX` XDP 程序返回码之一。`flags` 的高位可以设置为 `BPF_F_BROADCAST` `BPF_F_EXCLUDE_INGRESS`，如下所述
使用 `BPF_F_BROADCAST` 时，数据包将被广播到映射中的所有接口；使用 `BPF_F_EXCLUDE_INGRESS` 时，ingress 接口将被排除在广播之外
    - 如果设置BPF_F_BROADCAST，则键被忽略    - 广播特性也可用于实现组播转发：只需创建多个 DEVMAP，每个对应一个组播组
该辅助函数在成功时返`XDP_REDIRECT`，若映射查找失败则返`flags` 参数的低两位值
关于重定向的更多信息可参[redirect](redirect)

##### bpf_map_lookup_elem()


   void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

可以使用 `bpf_map_lookup_elem()` 辅助函数获取网络设备条目
### 用户空间


    DEVMAP 条目只能从用户空间更删除，而不能从 eBPF 程序中更删除    尝试从内eBPF 程序调用这些函数将导致程序加载失败并出现验证器（verifier）警告
##### bpf_map_update_elem()


   int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags);

可以使用 `bpf_map_update_elem()` 辅助函数添加或更新网络设备条目。该辅助函数以原子方式替换现有元素。`value` 参数可以`struct bpf_devmap_val`，或者为了向后兼容，也可以是一个简单的 `int ifindex`
 .. code-block:: c

    struct bpf_devmap_val {
        __u32 ifindex;   /** 设备索引 **/
        union {
            int   fd;  /** 写映射时prog fd **/
            __u32 id;  /** 读映射时prog id **/
        } bpf_prog;
    };

`flags` 参数可以是以下之一  - `BPF_ANY`：创建新元素或更新现有元素  - `BPF_NOEXIST`：仅当元素不存在时才创建新元素  - `BPF_EXIST`：更新现有元素
DEVMAP 可以通过`bpf_prog.fd` 添加`struct bpf_devmap_val` 来将程序与设备条目关联。程序在 `XDP_REDIRECT` 之后运行，并且可以同时访Rx 设备Tx 设备。与 `fd` 关联的程序必须具有类XDP 且期望附加类型为 `xdp_devmap`。当程序与设备索引关联时，程序在 `XDP_REDIRECT` 时、并且在该缓冲区被加入每 CPU 队列之前运行。如何附使用 xdp_devmap 程序的示例可以在内核自测中找到：

- `tools/testing/selftests/bpf/prog_tests/xdp_devmap_attach.c`
- `tools/testing/selftests/bpf/progs/test_xdp_with_devmap_helpers.c`

##### bpf_map_lookup_elem()


   int bpf_map_lookup_elem(int fd, const void **key, void **value);

可以使用 `bpf_map_lookup_elem()` 辅助函数获取网络设备条目
##### bpf_map_delete_elem()


   int bpf_map_delete_elem(int fd, const void *key);

可以使用 `bpf_map_delete_elem()` 辅助函数删除网络设备条目。该辅助函数在成功时返回 0，失败时返回负的错误码
## 示例（Examples

### 内核 BPF


以下代码片段展示了如何声明一个名tx_port `BPF_MAP_TYPE_DEVMAP`

    struct {
        __uint(type, BPF_MAP_TYPE_DEVMAP);
        __type(key, __u32);
        __type(value, __u32);
        __uint(max_entries, 256);
    } tx_port SEC(".maps");

以下代码片段展示了如何声明一个名forward_map `BPF_MAP_TYPE_DEVMAP_HASH`

    struct {
        __uint(type, BPF_MAP_TYPE_DEVMAP_HASH);
        __type(key, __u32);
        __type(value, struct bpf_devmap_val);
        __uint(max_entries, 32);
    } forward_map SEC(".maps");


    DEVMAP 中上述的值类型是 `struct bpf_devmap_val`

以下代码片段展示了一个简单的 xdp_redirect_map 程序。该程序会配合一个用户空间程序工作，该程序基ingress ifindex 填充 devmap `forward_map`。BPF 程序（如下）使用 ingress `ifindex` 作为 `key` 来重定向数据包

    SEC("xdp")
    int xdp_redirect_map_func(struct xdp_md *ctx)
    {
        int index = ctx->ingress_ifindex;

        return bpf_redirect_map(&forward_map, index, 0);
    }

以下代码片段展示了一个将数据包广播到 `tx_port` devmap 中所有接口的 BPF 程序

    SEC("xdp")
    int xdp_redirect_map_func(struct xdp_md *ctx)
    {
        return bpf_redirect_map(&tx_port, 0, BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS);
    }

### 用户空间


以下代码片段展示了如何更新一个名`tx_port` devmap

    int update_devmap(int ifindex, int redirect_ifindex)
    {
        int ret;

        ret = bpf_map_update_elem(bpf_map__fd(tx_port), &ifindex, &redirect_ifindex, 0);
        if (ret < 0) {
            fprintf(stderr, "Failed to update devmap_ value: %s\n",
                strerror(errno));
        }

        return ret;
    }

以下代码片段展示了如何更新一个名`forward_map` hash_devmap

    int update_devmap(int ifindex, int redirect_ifindex)
    {
        struct bpf_devmap_val devmap_val = { .ifindex = redirect_ifindex };
        int ret;

        ret = bpf_map_update_elem(bpf_map__fd(forward_map), &ifindex, &devmap_val, 0);
        if (ret < 0) {
            fprintf(stderr, "Failed to update devmap_ value: %s\n",
                strerror(errno));
        }
        return ret;
    }

## 参考（References

- https://lwn.net/Articles/728146/
- https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/commit/?id=6f9d451ab1a33728adb72d7ff66a7b374d665176
- https://elixir.bootlin.com/linux/latest/source/net/core/filter.c#L4106
