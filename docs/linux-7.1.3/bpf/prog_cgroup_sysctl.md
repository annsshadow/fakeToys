
## BPF_PROG_TYPE_CGROUP_SYSCTL


本文档描`BPF_PROG_TYPE_CGROUP_SYSCTL` 程序类型，它sysctl 提供 cgroup-bpf
钩子
该钩子必须附加到一cgroup，并且每当该 cgroup 内的进程尝试从或proc 中的
sysctl 旋钮读取或写入时都会被调用
######## 1. 附加类型


必须使用 `BPF_CGROUP_SYSCTL` 附加类型来将 `BPF_PROG_TYPE_CGROUP_SYSCTL` 程序
附加cgroup
######## 2. 上下

`BPF_PROG_TYPE_CGROUP_SYSCTL` 提供对以下上下文的访问，来自
```

    struct bpf_sysctl {
        __u32 write;
        __u32 file_pos;
    };

```
- `write` 指示 sysctl 值正被读取（`0`）还是写入（`1`）。该字段是只读的
- `file_pos` 指示正在被访问、读取或写入sysctl 的文件位置。该字段是可读写的  写入该字段会设置 sysctl proc 文件的起始位置，随后`read(2)` 将从该位置读取，
  `write(2)` 将写入该位置。例如，即使在由用户空间`file_pos > 0` 时调  `write(2)` 的情况下，也可以向该字段写入零，从而通过 `bpf_sysctl_set_new_value()`
  覆盖整个 sysctl 值。向该字段写入非零值可用于从指定的 `file_pos` 开始访问部  sysctl 值。并非所sysctl 都支持以 ``file_pos != 0`` 访问，例如对数值型
  sysctl 条目的写入必须始终在文件位置 `0`。另请参`kernel.sysctl_writes_strict`
  sysctl銆。
关于如何访问上下文字段的更多细节，请参见 `linux/bpf.h`_
######## 3. 杩斿洖鐮。

`BPF_PROG_TYPE_CGROUP_SYSCTL` 程序必须返回以下返回码之一
- `0` 表示“拒绝访sysctl”；
- `1` 表示“继续访问”
如果程序返回 `0`，用户空间将`read(2)` `write(2)` 得到 `-1`，并`errno`
将被设为 `EPERM`
######## 4. 辅助函数


由于 sysctl 旋钮由名称和值表示，sysctl 专用BPF 辅助函数侧重于提供对这些
属性的访问
- `bpf_sysctl_get_name()` 用于获取 sysctl 名称（正如它`/proc/sys` 中可见的
  那样），放入BPF 程序提供的缓冲区中；

- `bpf_sysctl_get_current_value()` 用于获取 sysctl 当前持有的字符串值，放入  BPF 程序提供的缓冲区中。该辅助函数在对 sysctl `read(2)` `write(2)`   都可用；

- `bpf_sysctl_get_new_value()` 用于在实际写入发生之前，获取当前正被写入 sysctl
  的新字符串值。该辅助函数只能用于 `ctx->write == 1`
- `bpf_sysctl_set_new_value()` 用于在实际写入发生之前，覆盖当前正被写入 sysctl
  的新字符串值。sysctl 值将从当前的 `ctx->file_pos` 开始被覆盖。如果要覆盖
  整个值，BPF 程序可以在调用该辅助函数之前`file_pos` 设为零。该辅助函数只能
  用于 `ctx->write == 1`。由该辅助函数设置的新字符串值会被内核以与用户空间传  的等效字符串相同的方式对待和校验
BPF 程序看待 sysctl 值的方式与用户空间在 proc 文件系统中相同，即作为一个字符串由于许多 sysctl 值表示整数或整数向量，以下辅助函数可用于从字符串中获取数值：

- `bpf_strtol()` 用于将字符串的初始部分转换为 long 整数，类似于用户空间  `strtol(3)`_- `bpf_strtoul()` 用于将字符串的初始部分转换为 unsigned long 整数，类似于用户
  空间`strtoul(3)`_
关于此处描述辅助函数的更多细节，请参`linux/bpf.h`_
######## 5. 示例


请参`test_sysctl_prog.c`_，以获取一个用 C 编写BPF 程序示例，该程序访问
sysctl 名称和值，解析字符串值以获取整数向量，并据此做出允许或拒绝访sysctl
的决定
######## 6. 注意事项


`BPF_PROG_TYPE_CGROUP_SYSCTL` 旨在用于**可信* root 环境，例如用于监sysctl
的使用，或捕获以 root 身份在独cgroup 中运行的应用程序试图设置的不合理值
由于`sys_read` / `sys_write` 时调用了 `task_dfl_cgroup(current)`，它可能返回
`sys_open` 时不同的结果，即proc 文件系统中打开 sysctl 文件的进程可能不同于
正尝试从/向它读取或写入的进程，并且这样两个进程可能运行在不同cgroup 中，意味着 `BPF_PROG_TYPE_CGROUP_SYSCTL` 不应被用作限sysctl 使用的安全机制
与任cgroup-bpf 程序一样，如果root 身份cgroup 中运行的应用程序不应被允分离/替换由管理员附加BPF 程序，则应当额外小心
   ../../tools/testing/selftests/bpf/progs/test_sysctl_prog.c
