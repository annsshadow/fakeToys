
## Devpts 文件系统


现在每次 devpts 文件系统的挂载都是独立的，使得在一个挂载中分配pty 及其
索引独立于所有其他挂载中pty 及其索引
现在所devpts 文件系统的挂载都会创建一个权限为 `0000` `/dev/pts/ptmx`
节点
为保持向后兼容，当打开`mknod name c 5 2` 创建ptmx 设备节点（即任何
此类节点）时，会在与 ptmx 设备节点相同的目录下查找名为 `pts` devpts
实例
作为一种选择，除了在 `/dev/ptmx` 放置 `/dev/ptmx` 设备节点外，也可以在
`/dev/ptmx` 放置指向 `/dev/pts/ptmx` 的符号链接，或将 `/dev/ptx/ptmx` 绑定
挂载`/dev/ptmx`。如果你选择以这种方式使devpts 文件系统，则应以
`ptmxmode=0666` 挂载 devpts，或调用 `chmod 0666 /dev/pts/ptmx`
```

    kernel.pty.max = 4096	- 全局限制
    kernel.pty.reserve = 1024	- 为从初始挂载命名空间挂载的文件系统保    kernel.pty.nr		- 当前 pty 计数

```
每实例限制可通过添加挂载选项 `max=<count>` 设置
此特性在内核 3.4 中与 `sysctl kernel.pty.reserve` 一并加入
在早3.4 的内核中，sysctl `kernel.pty.max` 作为每实例限制工作