
## FUSE I/O 模式


Fuse 支持以下 I/O 模式
- direct-io（直IO- cached（缓存）
  - write-through（写穿）
  - writeback-cache（回写缓存）

direct-io 模式可以通过 FUSE_OPEN 回复中的 FOPEN_DIRECT_IO 标志来选择
direct-io 模式下，读写完全绕过页缓存。不会发生预读（read-ahead）。默认禁用共mmap。要允许共享 mmap，可以在 FUSE_INIT 回复中启FOPEN_DIRECT_IO_ALLOW_MMAP 标志
cached 模式下，读操作可以由页缓存满足，并且内核可以进行预读以填充缓存。在对该文件的任何写操作之后，缓存始终保持一致。支持所mmap 模式
cached 模式有两个子模式，用于控制写操作的处理方式。write-through 模式是默认模式，在所有内核上都受支持。writeback-cache 模式可以通过 FUSE_INIT 回复中的 FUSE_WRITEBACK_CACHE 标志来选择
write-through 模式下，每次写操作都会立即作为一个或多个 WRITE 请求发送给用户空间，同时更新任何已缓存的页（并缓存之前未缓存但被完整写入的页）。写操作永远不会发READ 请求，因此当未缓存的页被部分写入时，该页会被丢弃
writeback-cache 模式（由 FUSE_WRITEBACK_CACHE 标志启用）下，写操作只进入缓存，这意味着 write(2) 系统调用通常可以非常快地完成。脏页会被隐式回写（后台回写或在内存压力下回收页），或显式回写（close(2)、fsync(2) 触发，以及在 munmap(2) 释放文件的最后一个引用时触发）。该模式假定对文件系统的所有更改都经过 FUSE 内核模块（大小和 atime/ctime/mtime 属性由内核保持最新），因此通常不适合网络文件系统。如果写入了部分页，则需要先从用户空间读取该页。这意味着，即使是对于O_WRONLY 打开的文件，内核也可能会产生 READ 请求