######## DVB munmap()


## 名称


dmx-munmap - 解除设备内存映射


## 摘要



    #include <unistd.h>
    #include <sys/mman.h>


## 参数


`start`
    由 `mmap()` 函数返回的已映射缓冲区的地址。

`length`
    已映射缓冲区的长度。该值必须与传给 `mmap()` 的值相同。

## 说明


解除先前通过 `mmap()` 函数映射的缓冲区，并在可能时释放它。

## 返回值


成功时 `munmap()` 返回 0，失败时返回 -1 并相应地设置 `errno` 变量：

EINVAL
    `start` 或 `length` 不正确，或者尚未映射任何缓冲区。
