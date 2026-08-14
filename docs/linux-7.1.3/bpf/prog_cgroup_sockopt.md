
## BPF_PROG_TYPE_CGROUP_SOCKOPT


`BPF_PROG_TYPE_CGROUP_SOCKOPT` 程序类型可以附加到两个
cgroup 钩子（hook）上：

- `BPF_CGROUP_GETSOCKOPT` - 在进程每次执行 `getsockopt`
  系统调用时调用。
- `BPF_CGROUP_SETSOCKOPT` - 在进程每次执行 `setsockopt`
  系统调用时调用。

上下文（`struct bpf_sockopt`）关联了套接字（`sk`）以及
所有输入参数：`level`、`optname`、`optval` 和 `optlen`。

## BPF_CGROUP_SETSOCKOPT


`BPF_CGROUP_SETSOCKOPT` 在内核处理 sockopt **之前**被触发，
并且其上下文是可写的：它可以在将提供的参数向下传递给内核之前
修改这些参数。该钩子可以访问 cgroup
与套接字本地存储（socket local storage）。

如果 BPF 程序将 `optlen` 设置为 -1，则在 cgroup 链中
所有其他 BPF 程序执行完毕后，控制将返回用户空间
（即内核的 `setsockopt` 处理将**不会**被执行）。

注意，`optlen` 不能增加到超过用户提供的
值。它只能被减小或设置为 -1。任何其他值都会
触发 `EFAULT`。

### 返回类型


- `0` - 拒绝该 syscall，将向用户空间返回 `EPERM`。
- `1` - 成功，继续执行 cgroup 链中的下一个 BPF 程序。

## BPF_CGROUP_GETSOCKOPT


`BPF_CGROUP_GETSOCKOPT` 在内核处理 sockopt **之后**被触发。
如果 BPF 钩子对内核返回的任何内容感兴趣，它可以观察
`optval`、`optlen` 和 `retval`。BPF 钩子可以覆盖
上述值，调整 `optlen` 并将 `retval` 重置为 0。如果 `optlen`
被增加到超过初始的 `getsockopt` 值（即用户空间缓冲区太小），
则会返回 `EFAULT`。

该钩子可以访问 cgroup 与套接字本地存储。

注意，可以设置给 `retval` 的唯一可接受值是 0 以及
内核返回的原始值。任何其他值都会触发 `EFAULT`。

### 返回类型


- `0` - 拒绝该 syscall，将向用户空间返回 `EPERM`。
- `1` - 成功：将 `optval` 和 `optlen` 复制到用户空间，并从
  syscall 返回 `retval`（注意这可能会被父 cgroup 的
  BPF 程序覆盖）。

## Cgroup 继承


假设存在如下 cgroup 层级，其中每个 cgroup 在每个层级都附加了
`BPF_CGROUP_GETSOCKOPT`，其中
```

  A (root, parent)
   \
    B (child)

```

当应用程序从 cgroup B 调用 `getsockopt` syscall 时，
程序自底向上执行：B、A。第一个程序
（B）看到内核 `getsockopt` 的结果。它可以选择性地
调整 `optval`、`optlen` 并将 `retval` 重置为 0。之后
控制将传递给第二个（A）程序，该程序将看到
与 B 相同的上下文，包括任何潜在的修改。

`BPF_CGROUP_SETSOCKOPT` 同理：如果程序被附加到
A 和 B，触发顺序是 B，然后 A。如果 B 对输入参数
（`level`、`optname`、`optval`、`optlen`）做了任何修改，
那么链中的下一个程序（A）将看到那些修改，
**而非**原始的输入 `setsockopt` 参数。这些可能被
修改的值随后会被向下传递给内核。

## 较大的 optval


当 `optval` 大于 `PAGE_SIZE` 时，BPF 程序
只能访问该数据的第一个 `PAGE_SIZE`。因此它有两个选择：

- 将 `optlen` 设置为零，这表示内核应使用
  来自用户空间的原始缓冲区。BPF 程序对 `optval` 所做的任何修改
  都将被忽略。
- 将 `optlen` 设置为小于 `PAGE_SIZE` 的值，这表示
  内核应使用 BPF 裁剪后的 `optval`。

当 BPF 程序以大于 `PAGE_SIZE` 的 `optlen` 返回时，
用户空间将收到原始的内核缓冲区，而 BPF 程序可能施加的
任何修改都不会被应用。

## 示例


处理 BPF 程序的推荐方式如下：


	SEC("cgroup/getsockopt")
	int getsockopt(struct bpf_sockopt *ctx)
	{
		/** 自定义套接字选项。 **/
		if (ctx->level == MY_SOL && ctx->optname == MY_OPTNAME) {
			ctx->retval = 0;
			optval[^0^] = ...;
			ctx->optlen = 1;
			return 1;
		}

		/** 修改内核的套接字选项。 **/
		if (ctx->level == SOL_IP && ctx->optname == IP_FREEBIND) {
			ctx->retval = 0;
			optval[^0^] = ...;
			ctx->optlen = 1;
			return 1;
		}

		/** optval 大于 PAGE_SIZE 时使用内核缓冲区。 **/
		if (ctx->optlen > PAGE_SIZE)
			ctx->optlen = 0;

		return 1;
	}

	SEC("cgroup/setsockopt")
	int setsockopt(struct bpf_sockopt *ctx)
	{
		/** 自定义套接字选项。 **/
		if (ctx->level == MY_SOL && ctx->optname == MY_OPTNAME) {
			/** 执行某些操作 **/
			ctx->optlen = -1;
			return 1;
		}

		/** 修改内核的套接字选项。 **/
		if (ctx->level == SOL_IP && ctx->optname == IP_FREEBIND) {
			optval[^0^] = ...;
			return 1;
		}

		/** optval 大于 PAGE_SIZE 时使用内核缓冲区。 **/
		if (ctx->optlen > PAGE_SIZE)
			ctx->optlen = 0;

		return 1;
	}

有关处理套接字选项的 BPF 程序示例，请参见
`tools/testing/selftests/bpf/progs/sockopt_sk.c`。
