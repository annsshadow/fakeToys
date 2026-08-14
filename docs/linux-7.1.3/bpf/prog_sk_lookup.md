
## BPF sk_lookup 程序


BPF sk_lookup 程序类型（`BPF_PROG_TYPE_SK_LOOKUP`）为传输层在需要将数据包
投递到本地时所执行的套接字查找引入了可编程能力。

被调用时，BPF sk_lookup 程序可以通过调用 `bpf_sk_assign()` BPF 辅助函数来选择
一个将接收该入站数据包的套接字。

针对公共附加点（`BPF_SK_LOOKUP`）的钩子同时存在于 TCP 与 UDP。

## 动机


引入 BPF sk_lookup 程序类型是为了处理那些通过 `bind()` 套接字调用将套接字绑定到
某个地址并不现实的部署场景，例如：

1. 在一组 IP 地址范围（例如 192.0.2.0/24）上接收连接，而由于端口冲突，无法绑定到
   通配地址 `INADRR_ANY`，
2. 在所有或较宽范围的端口上接收连接，即 L7 代理的使用场景。

此类部署需要为范围内的每个 IP 地址/端口创建并 `bind()` 一个套接字，从而导致资源消耗
以及在套接字查找期间的潜在延迟尖峰。

## 附加


BPF sk_lookup 程序可以使用 `bpf(BPF_LINK_CREATE, ...)` 系统调用，以 `BPF_SK_LOOKUP`
附加类型和 netns FD 作为附加 `target_fd`，附加到某个网络命名空间。

多个程序可以附加到同一个网络命名空间。程序将按它们被附加的相同顺序被调用。

## 钩子


每当传输层需要为入站数据包寻找一个监听中（TCP）或未连接（UDP）的套接字时，所附
加的 BPF sk_lookup 程序就会运行。

发往已建立连接（TCP）或已连接（UDP）套接字的入站流量照常投递，不会触发 BPF sk_lookup
钩子。

所附加以的 BPF 程序必须以 `SK_PASS` 或 `SK_DROP` 裁决码返回。与其他作为网络过滤器的
BPF 程序类型一样，`SK_PASS` 表示套接字查找应继续到常规的基于哈希表的查找，而 `SK_DROP`
会导致传输层丢弃该数据包。

BPF sk_lookup 程序也可以通过调用 `bpf_sk_assign()` BPF 辅助函数来选择接收该数据包的
套接字。通常，程序会在一个持有套接字的映射（例如 `SOCKMAP` 或 `SOCKHASH`）中查找套接字，
并将一个 `struct bpf_sock *` 传给 `bpf_sk_assign()` 辅助函数以记录该选择。只有在程序以
`SK_PASS` 码终止时，选择套接字才会生效。

当附加多个程序时，最终结果由所有程序的返回码依据以下规则确定：

1. 如果任一程序返回 `SK_PASS` 并选择了有效套接字，则该套接字被用作套接字查找的结果。
2. 如果多于一个程序返回 `SK_PASS` 并选择了套接字，则最后的选择生效。
3. 如果任一程序返回 `SK_DROP`，且没有任何程序返回 `SK_PASS` 并选择套接字，则套接字
   查找失败。
4. 如果所有程序都返回 `SK_PASS` 且均未选择套接字，则套接字查找继续。

## API


在其上下文中，BPF sk_lookup 程序会收到一个 `struct bpf_sk_lookup` 实例，其中包含
触发该套接字查找的数据包的信息。即：

- IP 版本（`AF_INET` 或 `AF_INET6`），
- L4 协议标识符（`IPPROTO_TCP` 或 `IPPROTO_UDP`），
- 源与目标 IP 地址，
- 源与目标 L4 端口，
- 已通过 `bpf_sk_assign()` 选择的套接字。

详情请参考 `linux/bpf.h` 用户 API 头文件中的 `struct bpf_sk_lookup` 声明，以及
`bpf-helpers(7)
<https://man7.org/linux/man-pages/man7/bpf-helpers.7.html>`_ 手册页中关于
`bpf_sk_assign()` 的章节。

## 示例


参考实现请见 `tools/testing/selftests/bpf/prog_tests/sk_lookup.c`。
