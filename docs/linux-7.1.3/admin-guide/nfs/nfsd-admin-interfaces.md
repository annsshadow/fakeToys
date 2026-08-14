## nfsd 的管理接口


注意，通常这些接口仅由 nfs-utils 中的工具使用。

nfsd 主要由 "nfsd" 文件系统下的伪文件控制，该文件系统通常挂载在 /proc/fs/nfsd/。

服务器总是在首次向 nfsd/threads 写入非零值时启动。

在此之前，可以通过向 nfsd/portlist 写入来告诉 NFSD 监听哪些套接字；该写入可以是：

 - 一个 ascii 编码的文件描述符，它应当引用一个已绑定（对于 tcp 还需处于监听状态）的套接字，或
 - "transportname port"，其中 transportname 目前是 "udp"、"tcp" 或 "rdma" 之一。

如果 nfsd 在没有任何上述操作的情况下启动，那么它将在端口 2049 创建一个 udp 和一个 tcp 监听器（参见 nfsd_init_socks）。

在启动时，nfsd 和 lockd 的宽限期开始。nfsd 通过向 nfsd/threads 写入 0 来关闭。此时所有锁和状态都被丢弃。

在启动和关闭之间，可以通过对 nfsd/threads 的额外写入或通过对 nfsd/pool_threads 的写入来调高或调低线程数。

有关 nfsd/ 下的文件及其控制内容的更多细节，请参阅 fs/nfsd/nfsctl.c；其中大部分都有详细的注释。

## 实现说明


请注意，rpc 服务器要求调用者对监听套接字的添加和移除以及服务器的启动和关闭进行串行化。对于 nfsd，这通过 nfsd_mutex 完成。
