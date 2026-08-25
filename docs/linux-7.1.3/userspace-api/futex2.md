## futex2


:Author: Andr茅 Almeida <andrealmeid@collabora.com>

futex，即 fast user mutex（快速用户态互斥体），是一组系统调用，允许用户创建高性能的同步机制，例如用户态中的互斥体、信号量和条件变量。C 标准库（glibc）将其用作实现更高层接口（如 pthreads）的手段
futex2 是最初的 futex 系统调用的后续版本，旨在克服原始接口的局限性
## 用户 API


### ``futex_waitv()``


```
futex_waitv(struct futex_waitv *waiters, unsigned int nr_futexes,
            unsigned int flags, struct timespec *timeout, clockid_t clockid)

  struct futex_waitv {
        __u64 val;
        __u64 uaddr;
        __u32 flags;
        __u32 __reserved;
  };

```
用户态设置一struct futex_waitv 数组（最128 个条目），使`uaddr` 表示
要等待的地址，`val` 表示期望值，`flags` 指定 futex 的类型（如私有）和大小`__reserved` 必须0，但可用于未来扩展。数组第一个条目的指针作为 `waiters`
传入。若 `waiters` 或任`uaddr` 地址无效，则返回 `-EFAULT`
如果用户态使32 位指针，应进行显式转换以确保高位被清零。`uintptr_t` 可巧地完成这一工作，且32/64 位指针均适用
`nr_futexes` 指定数组的大小。超[1, 128] 区间的数值将使系统调用返`-EINVAL`
系统调用`flags` 参数需要为 0，但可用于未来扩展
对于 `waiters` 数组中的每个条目，将 `uaddr` 处的当前值与 `val` 比较。若不同系统调用将撤销迄今为止所做的全部工作并返`-EAGAIN`。若所有测试与校验成功，系统调用将等待直到发生以下情况之一
- 超时到期，返`-ETIMEOUT`- 向睡眠任务发送了信号，返`-ERESTARTSYS`- 列表中的某个 futex 被唤醒，返回被唤futex 的索引
如何使用该接口的示例可在 `tools/testing/selftests/futex/functional/futex_waitv.c` 中找到
### 超时（Timeout

`struct timespec *timeout` 参数是一个可选参数，指向一个绝对超时。需要在
`clockid` 参数中指定所用时钟的类型。支`CLOCK_MONOTONIC` `CLOCK_REALTIME`。该系统调用只接64 timespec 结构体
### futex 的类

futex 可以是私有的或共享的。私futex 用于共享同一内存空间、且 futex 虚拟地址对所有进程都相同的进程。这允许内核进行优化。要使用私有 futex，需futex 标志中指`FUTEX_PRIVATE_FLAG`。对于不共享同一内存空间、因此同一
futex 可能具有不同虚拟地址的进程（例如使用文件支持的共享内存），则需要不同的
内部机制才能被正确入队。这是默认行为，且对私有和共futex 都适用
futex 可以有不同的大小62 64 位。目前唯一受支持的32 位大小的
futex，且必须使用 `FUTEX_32` 标志指定