
## Linux 下的 BFS 文件系统


BFS 文件系统SCO UnixWare 操作系统用于 /stand 切片（slice），该切片通常包含内核镜像以及启动过程所需的少数其他文件
为了Linux 下访/stand 分区，你显然需要知道分区号，并且内核必须支UnixWare 磁盘切片（CONFIG_UNIXWARE_DISKLABEL 配置选项）。不BFS 支持并不依赖UnixWare 磁盘标签支持，因为也可以挂载
```

    # losetup /dev/loop0 stand.img
    # mount -t bfs /dev/loop0 /mnt/stand

```
其中 stand.img 是包BFS 文件系统镜像的文件。当你使用完毕并卸载后，还需要释```

    # losetup -d /dev/loop0

```

```
    # mount -t bfs -o loop stand.img /mnt/stand

```
这将自动分配第一个可用的回环设备（并在必要时加载 loop.o 内核模块）。如果回环驱动没有被自动加载，请确保你已经编译了该模块并modprobe 工作正常。注意，如果你的系统/etc/mtab 文件是到 /proc/mounts 的符号链接，那么 umount 不会释放 /dev/loopN 设备。你需要使losetup(8) "-d" 开关手动完成。更多信息请阅读 losetup(8) 手册页
要在 UnixWare 下创BFS 镜像，你首先需要找```

    # prtvtoc /dev/rdsk/c0b0t0d0s0

```
（假设你的根磁盘位于 target=0、lun=0、bus=0、controller=0）。然后你寻找标记"STAND" 的切片，通常就是切片 10。有了它之后
```

    # umount /stand
    # dd if=/dev/rdsk/c0b0t0d0sa of=stand.img bs=512

```
以防万一，你可以通过检查以下内容来验证你做对了
```

    # od -Ad -tx4 stand.img | more

```
4 个字节应该是 0x1badface
如果你对这个 BFS 实现有任何补丁、问题或建议，请联系作者：

Tigran Aivazian <aivazian.tigran@gmail.com>
