
## BPF maps


BPF 的 “maps”（映射）提供通用存储，可在内核与用户空间之间共享不同类型的数据。
现有若干存储类型，包括哈希（hash）、数组（array）、布隆过滤器（bloom filter）
与基数树（radix-tree）。其中几种映射类型用于支持基于映射内容执行操作的特定
BPF 辅助函数。映射通过 BPF 辅助函数从 BPF 程序中访问，这些辅助函数在
`bpf-helpers(7)`_ 的 `man-pages`_ 中有文档说明。

BPF 映射通过 `bpf` 系统调用从用户空间访问，该系统调用提供了创建映射、查找
元素、更新元素与删除元素的命令。有关 BPF 系统调用的更多细节，请参阅
`ebpf-syscall`_ 以及 `bpf(2)`_ 的 `man-pages`_。

## 映射类型


- [map_*](map_*)

## 使用注意


   int bpf(int command, union bpf_attr *attr, u32 size)

使用 `bpf()` 系统调用来执行由 `command` 指定的操作。该操作使用 `attr` 中
提供的参数。`size` 参数是 `attr` 中 `union bpf_attr` 的大小。

**BPF_MAP_CREATE**

使用 `attr` 中期望的类型与属性创建一个映射：


    int fd;
    union bpf_attr attr = {
            .map_type = BPF_MAP_TYPE_ARRAY;  /** 必填 **/
            .key_size = sizeof(__u32);       /** 必填 **/
            .value_size = sizeof(__u32);     /** 必填 **/
            .max_entries = 256;              /** 必填 **/
            .map_flags = BPF_F_MMAPABLE;
            .map_name = "example_array";
    };

    fd = bpf(BPF_MAP_CREATE, &attr, sizeof(attr));

成功时返回进程本地的文件描述符，失败时返回负的错误码。可以通过调用
`close(fd)` 删除该映射。由已打开文件描述符持有的映射会在进程退出时
自动删除。

   `'_'` and `'.'`。

**BPF_MAP_LOOKUP_ELEM**

使用 `attr->map_fd`、`attr->key`、`attr->value` 在给定映射中查找键。
成功时返回零并将找到的元素存入 `attr->value`，失败时返回负的错误码。

**BPF_MAP_UPDATE_ELEM**

使用 `attr->map_fd`、`attr->key`、`attr->value` 在给定映射中创建或更新
键值对。成功时返回零，失败时返回负的错误码。

**BPF_MAP_DELETE_ELEM**

使用 `attr->map_fd`、`attr->key` 在给定映射中按键查找并删除元素。成功时
返回零，失败时返回负的错误码。
