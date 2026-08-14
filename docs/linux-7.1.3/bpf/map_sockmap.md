
## BPF_MAP_TYPE_SOCKMAP 与 BPF_MAP_TYPE_SOCKHASH


   - `BPF_MAP_TYPE_SOCKMAP` 于内核版本 4.14 引入
   - `BPF_MAP_TYPE_SOCKHASH` 于内核版本 4.18 引入

`BPF_MAP_TYPE_SOCKMAP` 与 `BPF_MAP_TYPE_SOCKHASH` 类型的映射可用于在套接字之间重定向 skb（数据
包），或借助 BPF 辅助函数 `bpf_sk_redirect_map()`、`bpf_sk_redirect_hash()`、
`bpf_msg_redirect_map()` 与 `bpf_msg_redirect_hash()`，基于 BPF（verdict，裁决）程序的运行结果在套接字
层级施加策略。

`BPF_MAP_TYPE_SOCKMAP` 底层是一个数组，使用整数键作为索引来查找对 `struct sock` 的引用。该映射的
值即为套接字描述符。类似地，`BPF_MAP_TYPE_SOCKHASH` 是一种以哈希为底层支持的 BPF 映射，它通过
套接字描述符持有对套接字的引用。

    值的类型为 __u32 或 __u64；后者（__u64）用于支持向用户空间返回套接字 cookie。将映射持有的
    `struct sock *` 返回给用户空间既不安全也无用处。

这些映射可以附加 BPF 程序，具体而言是一个解析（parser）程序和一个裁决（verdict）程序。解析程序
决定已解析的数据量，从而决定需要排队多少数据才能得出裁决。裁决程序本质上就是重定向程序，可以
返回 `__SK_DROP`、`__SK_PASS` 或 `__SK_REDIRECT` 这样的裁决结果。

当一个套接字被插入到这些映射之一时，它的套接字回调会被替换，并且会为其附加一个 `struct sk_psock`。
此外，这个 `sk_psock` 会继承附加到该映射上的程序。

一个 sock 对象可以存在于多个映射中，但只能继承单一的解析程序或裁决程序。如果将一个 sock 对象
加入某个映射会导致出现多个解析程序，则该更新会返回 EBUSY 错误。

可以向这些映射附加的受支持程序如下：


	struct sk_psock_progs {
		struct bpf_prog *msg_parser;
		struct bpf_prog *stream_parser;
		struct bpf_prog *stream_verdict;
		struct bpf_prog	*skb_verdict;
	};

    不允许将 `stream_verdict` 与 `skb_verdict` 程序附加到同一个映射。

这些映射的程序附加类型如下：

- `msg_parser` 程序 - `BPF_SK_MSG_VERDICT`。
- `stream_parser` 程序 - `BPF_SK_SKB_STREAM_PARSER`。
- `stream_verdict` 程序 - `BPF_SK_SKB_STREAM_VERDICT`。
- `skb_verdict` 程序 - `BPF_SK_SKB_VERDICT`。

解析程序与裁决程序还可使用额外的辅助函数：`bpf_msg_apply_bytes()` 与
`bpf_msg_cork_bytes()`。借助 `bpf_msg_apply_bytes()`，BPF 程序可以告知基础设施给定的裁决应
作用多少字节。辅助函数 `bpf_msg_cork_bytes()` 处理另一种情况：BPF 程序在收到更多字节之前无法对
某条 msg 做出裁决，且在该 msg 被确认无误之前不希望转发该数据包。

最后，辅助函数 `bpf_msg_pull_data()` 与 `bpf_msg_push_data()` 可供
`BPF_PROG_TYPE_SK_MSG` 类型的 BPF 程序使用，用于拉入数据并将起始与结束指针设置为给定值，或向
``struct sk_msg_buff *msg`` 追加元数据。

以上所有辅助函数将在此后逐一详细说明。

## 用法

### 内核 BPF

##### bpf_msg_redirect_map()


	long bpf_msg_redirect_map(struct sk_msg_buff **msg, struct bpf_map **map, u32 key, u64 flags)

该辅助函数用于实现套接字层级的策略。如果消息 `msg` 被允许通过（即裁决 BPF 程序返回
`SK_PASS`），则将其重定向到 `map`（类型为 `BPF_MAP_TYPE_SOCKMAP`）中索引 `key` 所引用的那个
套接字。入口（ingress）与出口（egress）接口均可用于重定向。`flags` 中的 `BPF_F_INGRESS` 值
用于选择入口路径，否则选择出口路径。这是目前唯一受支持的标志。

成功时返回 `SK_PASS`，出错时返回 `SK_DROP`。

##### bpf_sk_redirect_map()


    long bpf_sk_redirect_map(struct sk_buff **skb, struct bpf_map **map, u32 key u64 flags)

将数据包重定向到 `map`（类型为 `BPF_MAP_TYPE_SOCKMAP`）中索引 `key` 所引用的那个套接字。入口与
出口接口均可用于重定向。`flags` 中的 `BPF_F_INGRESS` 值用于选择入口路径，否则选择出口路径。这是
目前唯一受支持的标志。

成功时返回 `SK_PASS`，出错时返回 `SK_DROP`。

##### bpf_map_lookup_elem()


    void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

类型为 `struct sock *` 的套接字条目可通过 `bpf_map_lookup_elem()` 辅助函数取出。

##### bpf_sock_map_update()


    long bpf_sock_map_update(struct bpf_sock_ops **skops, struct bpf_map **map, void *key, u64 flags)

向一个引用套接字的 `map` 添加条目，或更新其中的条目。`skops` 被用作与 `key` 相关联条目的新值。
`flags` 参数可以是以下之一：

- `BPF_ANY`：创建一个新元素或更新一个已存在的元素。
- `BPF_NOEXIST`：仅当元素不存在时才创建一个新元素。
- `BPF_EXIST`：更新一个已存在的元素。

如果该 `map` 带有 BPF 程序（解析程序与裁决程序），这些程序会被正在添加的套接字所继承。如果该
套接字已经附加了 BPF 程序，则会导致错误。

成功时返回 0，失败时返回一个负的错误码。

##### bpf_sock_hash_update()


    long bpf_sock_hash_update(struct bpf_sock_ops **skops, struct bpf_map **map, void *key, u64 flags)

向一个引用套接字的 sockhash `map` 添加条目，或更新其中的条目。`skops` 被用作与 `key` 相关联
条目的新值。

`flags` 参数可以是以下之一：

- `BPF_ANY`：创建一个新元素或更新一个已存在的元素。
- `BPF_NOEXIST`：仅当元素不存在时才创建一个新元素。
- `BPF_EXIST`：更新一个已存在的元素。

如果该 `map` 带有 BPF 程序（解析程序与裁决程序），这些程序会被正在添加的套接字所继承。如果该
套接字已经附加了 BPF 程序，则会导致错误。

成功时返回 0，失败时返回一个负的错误码。

##### bpf_msg_redirect_hash()


    long bpf_msg_redirect_hash(struct sk_msg_buff **msg, struct bpf_map **map, void *key, u64 flags)

该辅助函数用于实现套接字层级的策略。如果消息 `msg` 被允许通过（即裁决 BPF 程序返回
`SK_PASS`），则使用哈希 `key` 将其重定向到 `map`（类型为 `BPF_MAP_TYPE_SOCKHASH`）所引用的那个
套接字。入口与出口接口均可用于重定向。`flags` 中的 `BPF_F_INGRESS` 值用于选择入口路径，否则选择
出口路径。这是目前唯一受支持的标志。

成功时返回 `SK_PASS`，出错时返回 `SK_DROP`。

##### bpf_sk_redirect_hash()


    long bpf_sk_redirect_hash(struct sk_buff **skb, struct bpf_map **map, void *key, u64 flags)

该辅助函数用于实现 skb 套接字层级的策略。如果 sk_buff `skb` 被允许通过（即裁决 BPF 程序返回
`SK_PASS`），则使用哈希 `key` 将其重定向到 `map`（类型为 `BPF_MAP_TYPE_SOCKHASH`）所引用的那个
套接字。入口与出口接口均可用于重定向。`flags` 中的 `BPF_F_INGRESS` 值用于选择入口路径，否则选择
出口路径。这是目前唯一受支持的标志。

成功时返回 `SK_PASS`，出错时返回 `SK_DROP`。

##### bpf_msg_apply_bytes()


    long bpf_msg_apply_bytes(struct sk_msg_buff *msg, u32 bytes)

对于套接字策略，将 BPF 程序的裁决应用到消息 `msg` 接下来的 `bytes`（字节数）上。例如，该辅助函数
可用于以下情形：

- 单次 `sendmsg()` 或 `sendfile()` 系统调用包含多条逻辑消息，BPF 程序应当读取这些消息并为其
  做出裁决。
- BPF 程序只关心读取 `msg` 的前 `bytes` 个字节。如果消息的负载很大，那么即使裁决已经确定，仍然
  为全部字节反复 setup 并调用 BPF 程序，会造成不必要的开销。

返回 0。

##### bpf_msg_cork_bytes()


    long bpf_msg_cork_bytes(struct sk_msg_buff *msg, u32 bytes)

对于套接字策略，在累积到 `bytes` 个字节之前，阻止裁决 BPF 程序对消息 `msg` 的执行。

当需要在做出裁决之前获得特定数量的字节时可以使用该辅助函数，即便数据跨越了多次 `sendmsg()` 或
`sendfile()` 调用。

返回 0。

##### bpf_msg_pull_data()


    long bpf_msg_pull_data(struct sk_msg_buff *msg, u32 start, u32 end, u64 flags)

对于套接字策略，从用户空间拉入 `msg` 的非线性数据，并将指针 `msg->data` 与 `msg->data_end`
分别设置为 `msg` 中 `start` 与 `end` 字节的偏移量。

如果类型为 `BPF_PROG_TYPE_SK_MSG` 的程序在 `msg` 上运行，它只能解析（`data`，`data_end`）指针
已经消费过的数据。对于 `sendmsg()` 钩子而言，这通常就是第一个 scatterlist 元素。但对于依赖
MSG_SPLICE_PAGES 的调用（例如 `sendfile()`）而言，其范围将是（**0**，**0**），因为数据与用户空间
共享，而默认目标是在 BPF 裁决做出期间（或之后）避免允许用户空间修改数据。该辅助函数可用于拉入
数据并将起始与结束指针设置为给定值。必要时会复制数据（即当数据不是线性的、且起始与结束指针不指向
同一数据块时）。

调用该辅助函数可能会改变底层的数据包缓冲区。因此，在加载时，校验器（verifier）此前对所有指针
所做的检查都会失效，如果在该辅助函数与直接数据包访问配合使用时，必须重新执行这些检查。

`flags` 的所有取值都保留供将来使用，必须保持为零。

成功时返回 0，失败时返回一个负的错误码。

##### bpf_map_lookup_elem()




	void **bpf_map_lookup_elem(struct bpf_map **map, const void *key)

在 sockmap 或 sockhash 映射中查找一个套接字条目。

返回与 `key` 相关联的套接字条目，如果没有找到条目则返回 NULL。

##### bpf_map_update_elem()


	long bpf_map_update_elem(struct bpf_map **map, const void **key, const void *value, u64 flags)

在 sockmap 或 sockhash 中添加或更新一个套接字条目。

flags 参数可以是以下之一：

- BPF_ANY：创建一个新元素或更新一个已存在的元素。
- BPF_NOEXIST：仅当元素不存在时才创建一个新元素。
- BPF_EXIST：更新一个已存在的元素。

成功时返回 0，失败时返回一个负的错误码。

##### bpf_map_delete_elem()


    long bpf_map_delete_elem(struct bpf_map **map, const void **key)

从 sockmap 或 sockhash 中删除一个套接字条目。

成功时返回 0，失败时返回一个负的错误码。

### 用户空间

##### bpf_map_update_elem()


	int bpf_map_update_elem(int fd, const void **key, const void **value, __u64 flags)

可以使用 `bpf_map_update_elem()` 函数添加或更新 sockmap 条目。`key` 参数是 sockmap 数组的
索引值，`value` 参数是该套接字的 FD 值。

在底层，sockmap 的更新函数会使用套接字 FD 值去取出相关联的套接字及其附加的 psock。

flags 参数可以是以下之一：

- BPF_ANY：创建一个新元素或更新一个已存在的元素。
- BPF_NOEXIST：仅当元素不存在时才创建一个新元素。
- BPF_EXIST：更新一个已存在的元素。

##### bpf_map_lookup_elem()


    int bpf_map_lookup_elem(int fd, const void **key, void **value)

可以使用 `bpf_map_lookup_elem()` 函数取出 sockmap 条目。

	返回的条目是一个套接字 cookie，而非套接字本身。

##### bpf_map_delete_elem()


    int bpf_map_delete_elem(int fd, const void *key)

可以使用 `bpf_map_delete_elem()` 函数删除 sockmap 条目。

成功时返回 0，失败时返回一个负的错误码。

## 示例


### 内核 BPF

关于 sockmap API 用法的若干示例可以在以下位置找到：

- `tools/testing/selftests/bpf/progs/test_sockmap_kern.h`_
- `tools/testing/selftests/bpf/progs/sockmap_parse_prog.c`_
- `tools/testing/selftests/bpf/progs/sockmap_verdict_prog.c`_
- `tools/testing/selftests/bpf/progs/test_sockmap_listen.c`_
- `tools/testing/selftests/bpf/progs/test_sockmap_update.c`_

以下代码片段展示了如何声明一个 sockmap。


	struct {
		__uint(type, BPF_MAP_TYPE_SOCKMAP);
		__uint(max_entries, 1);
		__type(key, __u32);
		__type(value, __u64);
	} sock_map_rx SEC(".maps");

以下代码片段展示了一个示例解析程序。


	SEC("sk_skb/stream_parser")
	int bpf_prog_parser(struct __sk_buff *skb)
	{
		return skb->len;
	}

以下代码片段展示了一个简单的裁决程序，它与一个 sockmap 交互，根据本地端口将流量重定向到另一个
套接字。


	SEC("sk_skb/stream_verdict")
	int bpf_prog_verdict(struct __sk_buff *skb)
	{
		__u32 lport = skb->local_port;
		__u32 idx = 0;

		if (lport == 10000)
			return bpf_sk_redirect_map(skb, &sock_map_rx, idx, 0);

		return SK_PASS;
	}

以下代码片段展示了如何声明一个 sockhash 映射。


	struct socket_key {
		__u32 src_ip;
		__u32 dst_ip;
		__u32 src_port;
		__u32 dst_port;
	};

	struct {
		__uint(type, BPF_MAP_TYPE_SOCKHASH);
		__uint(max_entries, 1);
		__type(key, struct socket_key);
		__type(value, __u64);
	} sock_hash_rx SEC(".maps");

以下代码片段展示了一个简单的裁决程序，它与一个 sockhash 交互，根据 skb 某些参数的哈希值将流量
重定向到另一个套接字。


	static inline
	void extract_socket_key(struct __sk_buff **skb, struct socket_key **key)
	{
		key->src_ip = skb->remote_ip4;
		key->dst_ip = skb->local_ip4;
		key->src_port = skb->remote_port >> 16;
		key->dst_port = (bpf_htonl(skb->local_port)) >> 16;
	}

	SEC("sk_skb/stream_verdict")
	int bpf_prog_verdict(struct __sk_buff *skb)
	{
		struct socket_key key;

		extract_socket_key(skb, &key);

		return bpf_sk_redirect_hash(skb, &sock_hash_rx, &key, 0);
	}

### 用户空间

关于 sockmap API 用法的若干示例可以在以下位置找到：

- `tools/testing/selftests/bpf/prog_tests/sockmap_basic.c`_
- `tools/testing/selftests/bpf/test_sockmap.c`_
- `tools/testing/selftests/bpf/test_maps.c`_

以下代码示例展示了如何创建一个 sockmap、附加一个解析程序与裁决程序，并添加一个套接字条目。


	int create_sample_sockmap(int sock, int parse_prog_fd, int verdict_prog_fd)
	{
		int index = 0;
		int map, err;

		map = bpf_map_create(BPF_MAP_TYPE_SOCKMAP, NULL, sizeof(int), sizeof(int), 1, NULL);
		if (map < 0) {
			fprintf(stderr, "Failed to create sockmap: %s\n", strerror(errno));
			return -1;
		}

		err = bpf_prog_attach(parse_prog_fd, map, BPF_SK_SKB_STREAM_PARSER, 0);
		if (err){
			fprintf(stderr, "Failed to attach_parser_prog_to_map: %s\n", strerror(errno));
			goto out;
		}

		err = bpf_prog_attach(verdict_prog_fd, map, BPF_SK_SKB_STREAM_VERDICT, 0);
		if (err){
			fprintf(stderr, "Failed to attach_verdict_prog_to_map: %s\n", strerror(errno));
			goto out;
		}

		err = bpf_map_update_elem(map, &index, &sock, BPF_NOEXIST);
		if (err) {
			fprintf(stderr, "Failed to update sockmap: %s\n", strerror(errno));
			goto out;
		}

	out:
		close(map);
		return err;
	}

## 参考资料


- https://github.com/jrfastab/linux-kernel-xdp/commit/c89fd73cb9d2d7f3c716c3e00836f07b1aeb261f
- https://lwn.net/Articles/731133/
- http://vger.kernel.org/lpc_net2018_talks/ktls_bpf_paper.pdf
- https://lwn.net/Articles/748628/
- https://lore.kernel.org/bpf/20200218171023.844439-7-jakub@cloudflare.com/
