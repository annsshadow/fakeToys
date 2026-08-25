
## BPF_MAP_TYPE_XSKMAP


   - `BPF_MAP_TYPE_XSKMAP` 在内核版4.18 中引
`BPF_MAP_TYPE_XSKMAP` 用作 XDP BPF 辅助函数 `bpf_redirect_map()` `XDP_REDIRECT` 动作的后端映射，类似'devmap' 'cpumap'该映射类型将原始 XDP 帧重定向`AF_XDP`_ 套接字（XSK），这是内核中一种新的地址族，允许将帧从驱动重定向到用户空间而无需遍历完整的网络协议栈。一AF_XDP 套接字绑定到单个 netdev 队列。XSK 到队列的映射如下图所示：


    +---------------------------------------------------+
    |     xsk A      |     xsk B       |      xsk C     |<---+ User space
    =========================================================|==========
    |    Queue 0     |     Queue 1     |     Queue 2    |    |  Kernel
    +---------------------------------------------------+    |
    |                  Netdev eth0                      |    |
    +---------------------------------------------------+    |
    |                            +=============+        |    |
    |                            | key |  xsk  |        |    |
    |  +---------+               +=============+        |    |
    |  |         |               |  0  | xsk A |        |    |
    |  |         |               +-------------+        |    |
    |  |         |               |  1  | xsk B |        |    |
    |  | BPF     |-- redirect -->+-------------+-------------+
    |  | prog    |               |  2  | xsk C |        |
    |  |         |               +-------------+        |
    |  |         |                                      |
    |  |         |                                      |
    |  +---------+                                      |
    |                                                   |
    +---------------------------------------------------+

    绑定到某<netdev/queue_id> AF_XDP 套接字将***接受来自<netdev/queue_id> XDP 帧。如XDP 程序试图从套接字绑定<netdev/queue_id> 之外的队列进行重定向，该帧将不会在套接字上被接收
通常每个 netdev 创建一XSKMAP。该映射包含一XSK 文件描述符（FD）数组。数组元素的数量通常通过 `max_entries` 映射参数设置或调整。对AF_XDP，`max_entries` 等于 netdev 支持的队列数量
    映射的键和值的大小都必须为 4 字节
## 用法


### 内核 BPF

##### bpf_redirect_map()


    long bpf_redirect_map(struct bpf_map *map, u32 key, u64 flags)

将数据包重定向到 `map` 中索`key` 处所引用的端点对于 `BPF_MAP_TYPE_XSKMAP`，该映射包含绑定netdev 队列的套接字XSK FD 引用
    如果映射在某个索引处为空，则数据包被丢弃。这意味着必须加载一XDP 程序，且 XSKMAP 中至少包含一XSK，才能通过套接字将任何流量送到用户空间
##### bpf_map_lookup_elem()


    void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

类型`struct xdp_sock *` XSK 条目引用可以使用 `bpf_map_lookup_elem()` 辅助函数获取
### 用户空间

    XSK 条目只能从用户空间更删除，而不能从 BPF 程序中更删除。尝试从内核 BPF 程序调用这些函数将导致程序加载失败并发出验证器警告
##### bpf_map_update_elem()


	int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags)

可以使用 `bpf_map_update_elem()` 辅助函数添加或更XSK 条目`key` 参数等于 XSK 所绑定到的队列queue_id。`value` 参数是该套接字的 FD 值
在底层，XSKMAP 的更新函数使XSK FD 值来获取关联`struct xdp_sock` 实例
flags 参数可以是以下之一
- BPF_ANY：创建新元素或更新现有元素- BPF_NOEXIST：仅在元素不存在时创建新元素- BPF_EXIST：更新现有元素
##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value)

成功时返`struct xdp_sock *`，失败时返回负的错误码
##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key)

可以使用 `bpf_map_delete_elem()` 辅助函数删除 XSK 条目该辅助函数在成功时返0，失败时返回负的错误码
    `libxdp`_ 删除一XSK 时，它也会从 XSKMAP 中移除关联的套接字条目
## 示例

### 内核


以下代码片段展示了如何声明一个名`xsks_map` `BPF_MAP_TYPE_XSKMAP`，以及如何将数据包重定向到一XSK

	struct {
		__uint(type, BPF_MAP_TYPE_XSKMAP);
		__type(key, __u32);
		__type(value, __u32);
		__uint(max_entries, 64);
	} xsks_map SEC(".maps");


	SEC("xdp")
	int xsk_redir_prog(struct xdp_md *ctx)
	{
		__u32 index = ctx->rx_queue_index;

		if (bpf_map_lookup_elem(&xsks_map, &index))
			return bpf_redirect_map(&xsks_map, index, 0);
		return XDP_PASS;
	}

### 用户空间


以下代码片段展示了如何用 XSK 条目更新一XSKMAP

	int update_xsks_map(struct bpf_map *xsks_map, int queue_id, int xsk_fd)
	{
		int ret;

		ret = bpf_map_update_elem(bpf_map__fd(xsks_map), &queue_id, &xsk_fd, 0);
		if (ret < 0)
			fprintf(stderr, "Failed to update xsks_map: %s\n", strerror(errno));

		return ret;
	}

关于如何创建 AF_XDP 套接字的示例，请参阅 `libxdp`_ 仓库`bpf-examples`_ 目录下的 AF_XDP-example AF_XDP-forwarding 程序关于 AF_XDP 接口的详细说明，请参阅：

- `libxdp-readme`_- `AF_XDP`_ 内核文档
    使用 XSKMAP AF_XDP 最全面的资源是 `libxdp`_