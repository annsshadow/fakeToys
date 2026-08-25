


######## 数字电视 mmap()


## 名称


dmx-mmap - 将设备内存映射到应用程序地址空间


## 简

    #include <unistd.h>
    #include <sys/mman.h>


## 参数


`start`
    将缓冲区映射到应用程序地址空间中的该地址    当指定了 `MAP_FIXED` 标志时，`start` 必须    页大小的整数倍，且当指定地址不可用时 mmap 将失败。不鼓励
    使用该选项；应用程序应直接在此处指定一`NULL` 指针
`length`
    要映射的内存区域长度。这必须DVB 数据包长    （多数驱动为 188）的整数倍
`prot`
    `prot` 参数描述期望的内存保护    无论设备类型和数据交换方向如何，都应设置    `PROT_READ` | `PROT_WRITE`，以允许对图像缓冲区进行
    读写访问。驱动至少应支持这一标志组合
`flags`
    `flags` 参数指定映射对象的类型、映射选项    以及对已映射页副本的修改是仅对该进程私有，还    与其他引用共享
    `MAP_FIXED` 要求驱动只选择指定的地址    而不选择其他地址。如果指定地址不可用，`mmap()` 将失败    如果指定`MAP_FIXED`，`start` 必须是页大小的整数倍    不鼓励使用该选项
    必须设置 `MAP_SHARED` `MAP_PRIVATE` 之一    `MAP_SHARED` 允许应用程序将映射内存与其他（例如子）进程共享
```

       Linux 数字电视应用程序不应设置
       ``MAP_PRIVATE``、``MAP_DENYWRITE``、``MAP_EXECUTABLE`` ``MAP_ANON``
       标志
```
`fd`
    `open()` 返回的文件描述符
`offset`
    设备在内存中的缓冲区偏移量，DMX_QUERYBUF ioctl 返回
## 描述


`mmap()` 函数请求`fd` 所指定设备内存中从 `offset`
开始的 `length` 字节映射到应用程序地址空间，最好映射到地址
`start`。后一地址仅为提示，通常指定0
合适的长度和偏移量参数通过 DMX_QUERYBUF ioctl 查询缓冲区必须先通过 DMX_REQBUFS ioctl 分配，然后才能查询
使用 `munmap()` 函数解除缓冲区映射
## 杩斿洖鍊。

成功`mmap()` 返回指向已映射缓冲区的指针。出错时返回
`MAP_FAILED`1），并相应地设置 `errno` 变量。可能的错误码如下：

EBADF
    `fd` 不是有效的文件描述符
EACCES
    `fd` 未以读写方式打开
EINVAL
    `start`、`length` `offset` 不合适。（例如
    它们过大，或未按 `PAGESIZE` 边界对齐。）

    `flags` `prot` 值不受支持
    尚未通过 DMX_REQBUFS ioctl 分配任何缓冲区
ENOMEM
    没有足够的物理或虚拟内存来完成该请求