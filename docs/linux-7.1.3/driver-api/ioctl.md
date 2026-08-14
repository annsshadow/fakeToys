## 基于 ioctl 的接口


ioctl() 是应用程序与设备驱动交互最常见的方式。它很灵活，可以通过增加新的命令
轻松扩展，并且可以透过字符设备、块设备以及套接字和其他特殊文件描述符来传递。

然而，ioctl 命令的定义也很容易出错，而且一旦出错，在不破坏现有应用程序的前提下
很难修复，因此本文档试图帮助开发者正确地完成这件事。

## 命令号定义


命令号（command number），或称为请求号（request number），是传给 ioctl 系统调用的
第二个参数。虽然它可以是任意能唯一标识某个特定驱动中某个动作的 32 位数值，但围绕
如何定义它们存在一些约定。

`include/uapi/asm-generic/ioctl.h` 提供了四个用于定义遵循现代约定的 ioctl 命令的宏：
`_IO`、`_IOR`、`_IOW` 与 `_IOWR`。所有新命令都应使用它们，并传入正确的参数：

_IO/_IOR/_IOW/_IOWR
   宏名指明了参数将如何被使用。它可以是一个指向要传入内核的数据的指针（_IOW）、
   传出内核的指针（_IOR），或者两者兼有（_IOWR）。_IO 可以表示没有参数的命令，
   也可以表示传入一个整数值而非指针的命令。建议只对没有参数的命令使用 _IO，
   而使用指针来传递数据。

type
   一个 8 位数字，通常是一个字符字面量，特定于某个子系统或驱动，并列在
   Documentation/userspace-api/ioctl/ioctl-number.rst 中。

nr
   一个 8 位数字，用于标识具体的命令，对于给定的 'type' 值是唯一的。

data_type
   参数所指向的数据类型的名称，命令号将一个 13 位或 14 位的整数编码了 `sizeof(data_type)`
   的值，因此参数的最大尺寸被限制为 8191 字节。注意：不要把 sizeof(data_type) 类型
   传入 _IOR/_IOW/IOWR，那会导致编码 sizeof(sizeof(data_type))，即 sizeof(size_t)。
   _IO 没有 data_type 参数。


## 接口版本


有些子系统在数据结构中使用了版本号，以对命令的参数给出不同的解释。

这通常是个坏主意，因为对现有命令的改动往往会破坏现有的应用程序。

更好的做法是增加一个新的、带有新编号的 ioctl 命令。旧命令仍需要在内核中实现以保持
兼容性，但它可以作为对新实现的一层包装。

## 返回码


ioctl 命令可以返回 errno(3) 中所记录的负错误码；这些错误码会被转换为用户空间的
errno 值。成功时，返回码应为零。也可以（但不推荐）返回一个正的 'long' 值。

当 ioctl 回调以未知的命令号被调用时，处理程序返回 -ENOTTY 或 -ENOIOCTLCMD，这也会导致
系统调用返回 -ENOTTY。出于历史原因，某些子系统在这里返回 -ENOSYS 或 -EINVAL，但这
是错误的。

在 Linux 5.5 之前，compat_ioctl 处理程序需要返回 -ENOIOCTLCMD 才能使用回退转换为
原生命令。由于现在所有子系统都自行负责处理兼容（compat）模式，这已不再需要，但在
将缺陷修复向后移植到较旧内核时可能仍然需要注意。

## 时间戳


传统上，时间戳与超时值是作为 ``struct timespec` 或 `struct timeval` 传递的，但由于
在迁移到 64 位 time_t 之后，这些结构在用户空间中的定义互不兼容，这就成了问题。

可以使用 `struct __kernel_timespec` 类型来替代，在需要分离的秒/纳秒值时将其内嵌于
其他数据结构中，或者直接传递给用户空间。不过这仍然不够理想，因为该结构体既不完全
匹配内核的 timespec64，也不完全匹配用户空间的 timespec。可以使用 get_timespec64() 与
put_timespec64() 辅助函数来确保布局与用户空间保持兼容，并且正确地处理了填充（padding）。

由于将秒转换为纳秒代价很低，但反过来需要代价高昂的 64 位除法，一个单纯的 __u64 纳秒
值可以更简单、更高效。

超时值与时间戳理想情况下应使用 CLOCK_MONOTONIC 时间，正如 ktime_get_ns() 或
ktime_get_ts64() 所返回的那样。与 CLOCK_REALTIME 不同，这使得时间戳不会因闰秒调整与
clock_settime() 调用而向前或向后跳变。

ktime_get_real_ns() 可用于需要跨重启或多台机器保持一致的 CLOCK_REALTIME 时间戳。

## 32 位兼容（compat）模式


为了支持在 64 位机器上运行的 32 位用户空间，每个实现了 ioctl 回调处理程序的子系统或
驱动也必须实现相应的 compat_ioctl 处理程序。

只要遵循了所有关于数据结构的规则，这就和把 .compat_ioctl 指针设置为 compat_ptr_ioctl()
或 blkdev_compat_ptr_ioctl() 这样的辅助函数一样简单。

### compat_ptr()


在 s390 架构上，31 位用户空间对数据指针有歧义的表示，其最高位被忽略。在兼容模式下
运行这样的进程时，必须使用 compat_ptr() 辅助函数来清除 compat_uptr_t 的最高位，并
将其转化为一个有效的 64 位指针。在其他架构上，该宏只执行到 `void __user *` 指针的
转换。

在 compat_ioctl() 回调中，最后一个参数是一个 unsigned long，它可以解释为指针或标量，
取决于命令。如果它是标量，则绝不能使用 compat_ptr()，以确保 64 位内核对于最高位被
置位的参数，其行为与 32 位内核一致。

compat_ptr_ioctl() 辅助函数可用于替代自定义的 compat_ioctl 文件操作，适用于那些只
接受指向兼容数据结构的指针作为参数的驱动。

### 结构体布局


兼容的数据结构在所有架构上具有相同的布局，避免使用所有有问题的成员：

- `long` 与 `unsigned long` 的大小等于一个寄存器，因此它们可能是 32 位或 64 位宽，
  不能用于可移植的数据结构。固定长度的替代类型是 `__s32`、`__u32`、`__s64` 与 `__u64`。

- 指针有同样的问题，此外还需要使用 compat_ptr()。最好的解决办法是使用 `__u64` 代替
  指针，这需要在用户空间转换为 `uintptr_t`，并在内核中使用 u64_to_user_ptr() 将其
  转换回用户指针。

- 在 x86-32（i386）架构上，64 位变量的对齐只有 32 位，但在大多数其他架构上是自然
```
    struct foo {
        __u32 a;
        __u64 b;
        __u32 c;
    };

  has four bytes of padding between a and b on x86-64, plus another four
  bytes of padding at the end, but no padding on i386, and it needs a
  compat_ioctl conversion handler to translate between the two formats.

  To avoid this problem, all structures should have their members
  naturally aligned, or explicit reserved fields added in place of the
  implicit padding. The ``pahole`` tool can be used for checking the
  alignment.

```
- 在 ARM OABI 用户空间上，结构体被填充到 32 位的整数倍，这使得某些结构体如果不以 32 位
  边界结束，就会与现代 EABI 内核不兼容。

- 在 m68k 架构上，结构体成员不能保证有大于 16 位的对齐，这在依赖隐式填充时是个问题。

- 位域（bitfield）与枚举（enum）大体上会如人们预期的那样工作，但它们的一些特性是由
  实现定义的，因此在 ioctl 接口中最好完全避免使用它们。

- `char` 成员可能是有符号或无符号的，取决于架构，因此对于 8 位整数值应使用 __u8 与
  __s8 类型，不过对于定长字符串，char 数组则更清晰。

## 信息泄漏


未初始化的数据决不能复制回用户空间，因为这会造成信息泄漏，进而可用于攻破内核地址空间
布局随机化（KASLR），助长攻击。

出于这个原因（也为了兼容支持），最好避免数据结构中的任何隐式填充。当现有结构中存在
隐式填充时，内核驱动在将其复制到用户空间之前，必须小心地完全初始化该结构的一个实例。
这通常是在为各个成员赋值之前调用 memset() 来完成。

## 子系统抽象


虽然有些设备驱动实现了自己的 ioctl 函数，但大多数子系统会为多个驱动实现相同的命令。
理想情况下，子系统有一个 .ioctl() 处理程序，负责在用户空间与内核之间复制参数，并通过
普通的内核指针将它们传入子系统特定的回调函数。

这在多个方面有帮助：

- 如果多个驱动的用户空间 ABI 之间没有细微差异，那么为一个驱动编写的应用程序更有可能
  也能用于同一子系统中的另一个驱动。

- 用户空间访问与数据结构布局的复杂性只在一处完成，减少了出现实现缺陷的可能性。

- 当 ioctl 在多个驱动之间共享时，比它只用于单个驱动时，更有可能被有经验的开发者审查，
  从而发现接口中的问题。

## ioctl 的替代方案


在许多情况下，ioctl 并不是某个问题的最佳解决方案。替代方案包括：

- 系统调用（system call）更适合那种不绑定到物理设备、也不受字符设备节点的文件系统权限
  约束的系统级特性。

- netlink 是通过套接字配置任何网络相关对象的推荐方式。

- debugfs 用于那些不需要作为稳定接口暴露给应用程序的、用于调试功能的临时接口。

- sysfs 是暴露不绑定到文件描述符的内核对象状态的好方法。

- configfs 可用于比 sysfs 更复杂的配置。

- 自定义文件系统可以提供带有简单用户界面的额外灵活性，但会显著增加实现的复杂度。
