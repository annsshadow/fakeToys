## 内核内存泄漏检测器


Kmemleak 提供了一种检测可能的内核内存泄漏的方法，其方式类似于
` tracing garbage collector <https://en.wikipedia.org/wiki/Tracing_garbage_collection>`_
（跟踪式垃圾收集器），区别在于孤儿（orphan）对象不会被释放，而只是通过
/sys/kernel/debug/kmemleak 报告。Valgrind 工具（`memcheck --leak-check`）也使用
类似的方法来检测用户空间应用程序中的内存泄漏。

### 用法


必须在“Kernel hacking”中启用 CONFIG_DEBUG_KMEMLEAK。一个内核线程每隔 10 分钟
（默认）扫描一次内存，并打印找到的新未引用对象的数量。如果 `debugfs` 尚未
```
  # mount -t debugfs nodev /sys/kernel/debug/
```
```
  # cat /sys/kernel/debug/kmemleak
```
```
  # echo scan > /sys/kernel/debug/kmemleak
```
```
  # echo clear > /sys/kernel/debug/kmemleak
```
挂载，则再次读取 `/sys/kernel/debug/kmemleak` 时就会显示出新的泄漏。

请注意，孤儿对象是按它们被分配的顺序列出的，列表中开头的一个对象可能会导致后续
其他对象也被报告为孤儿。

内存扫描参数可以在运行时通过写入 `/sys/kernel/debug/kmemleak` 文件来修改。支持的
参数如下：

- off
    禁用 kmemleak（不可逆）
- stack=on
    启用任务栈扫描（默认）
- stack=off
    禁用任务栈扫描
- scan=on
    启动自动内存扫描线程（默认）
- scan=off
    停止自动内存扫描线程
- scan=<secs>
    设置自动内存扫描周期（秒）
    （默认 600，设为 0 则停止自动扫描）
- scan
    触发一次内存扫描
- clear
    清除当前内存泄漏嫌疑对象列表，做法是将所有当前已报告的未引用对象标记为
    灰色，或者在 kmemleak 已被禁用时释放所有 kmemleak 对象。
- dump=<addr>
    转储在 <addr> 处找到的对象的信息

Kmemleak 也可以通过在内核命令行传入 `kmemleak=off` 在启动时禁用。

在内核分配或释放内存的动作可能发生在 kmemleak 初始化之前，这些动作被存储在一个早期
日志缓冲区中。该缓冲区的大小通过 CONFIG_DEBUG_KMEMLEAK_MEM_POOL_SIZE 选项配置。

如果启用了 CONFIG_DEBUG_KMEMLEAK_DEFAULT_OFF，则 kmemleak 默认是禁用的。在内核
命令行传入 `kmemleak=on` 可启用该功能。

如果你遇到类似 “Error while writing to stdout” 或 “write_loop: Invalid argument”
的错误，请确保 kmemleak 已被正确启用。

### 基本算法


通过 `kmalloc`、`vmalloc`、`kmem_cache_alloc` 及其同类函数进行的内存分配会被跟踪，
指针连同大小、栈回溯等附加信息一起存储在一棵 rbtree 中。相应的释放函数调用会被
跟踪，并且指针会从 kmemleak 的数据结构中移除。

如果一个已分配的内存块，在扫描内存（包括保存的寄存器）时，找不到指向其起始地址或
块内任何位置的指针，则该内存块被视为孤儿。这意味着内核可能没有途径把该内存块的
地址传递给释放函数，因此该块被视为内存泄漏。

扫描算法的步骤：

  1. 把所有对象标记为白色（剩余的白色对象之后将被视为孤儿）
  2. 从数据段和栈开始扫描内存，把读到的值与 rbtree 中存储的地址进行比对。如果找到
     一个指向白色对象的指针，则把该对象加入灰色列表
  3. 扫描灰色对象以寻找匹配的地址（某些白色对象可能变为灰色并被加到灰色列表末尾），
     直到灰色集合处理完毕
  4. 剩余的白色对象被视为孤儿，并通过 /sys/kernel/debug/kmemleak 报告

一些已分配的内存块把指针存储在内核的内部数据结构中，它们无法被检测为孤儿。为避免
这一点，kmemleak 还可以存储需要被找到的、指向块地址范围内的地址的值的数量，以便
该块不被视为泄漏。一个例子是 __vmalloc()。

### 用 kmemleak 测试特定代码段


在初始启动后，你的 /sys/kernel/debug/kmemleak 输出页可能会相当长。如果你在开发时
有非常多缺陷的代码，也可能出现这种情况。为了应对这些情况，你可以使用 'clear' 命令
从 /sys/kernel/debug/kmemleak 的输出中清除所有已报告的未引用对象。在 'clear' 之后
发出一个 'scan'，你就可以找到新的未引用对象；这应有助于测试特定的代码段。

```
  # echo clear > /sys/kernel/debug/kmemleak
  ... 测试你的内核或模块 ...
  # echo scan > /sys/kernel/debug/kmemleak
```
```
  # cat /sys/kernel/debug/kmemleak
```

### 释放 kmemleak 内部对象


为了在 kmemleak 被用户禁用或因致命错误禁用之后，仍能访问之前发现的内存泄漏，kmemleak
的内部对象在 kmemleak 被禁用时不会被释放，而这些对象可能会占据物理内存的很大一部分。

```
  # echo clear > /sys/kernel/debug/kmemleak
```

### Kmemleak API


函数原型请参阅 include/linux/kmemleak.h 头文件。

- `kmemleak_init`		 - 初始化 kmemleak
- `kmemleak_alloc`		 - 通知一次内存块分配
- `kmemleak_alloc_percpu`	 - 通知一次 percpu 内存块分配
- `kmemleak_vmalloc`		 - 通知一次 vmalloc() 内存分配
- `kmemleak_free`		 - 通知一次内存块释放
- `kmemleak_free_part`	 - 通知一次部分内存块释放
- `kmemleak_free_percpu`	 - 通知一次 percpu 内存块释放
- `kmemleak_update_trace`	 - 更新对象分配栈回溯
- `kmemleak_not_leak`	 - 把一个对象标记为不是泄漏
- `kmemleak_transient_leak`	 - 把一个对象标记为暂时性泄漏
- `kmemleak_ignore`		 - 不扫描或不把某个对象报告为泄漏
- `kmemleak_scan_area`	 - 在内存块内增加扫描区域
- `kmemleak_no_scan`	 - 不扫描某个内存块
- `kmemleak_erase`		 - 擦除指针变量中的旧值
- `kmemleak_alloc_recursive` - 类似 kmemleak_alloc，但检查递归性
- `kmemleak_free_recursive`	 - 类似 kmemleak_free，但检查递归性

以下函数以物理地址作为对象指针，并且只在地址具有 lowmem 映射时才执行相应动作：

- `kmemleak_alloc_phys`
- `kmemleak_free_part_phys`
- `kmemleak_ignore_phys`

### 处理假阴性/假阳性


假阴性是真实的内存泄漏（孤儿对象），但由于内存扫描期间找到的值指向了这类对象而未被
kmemleak 报告。为了减少假阴性的数量，kmemleak 提供了 kmemleak_ignore、kmemleak_scan_area、
kmemleak_no_scan 和 kmemleak_erase 函数（见上）。任务栈也会增加假阴性的数量，且默认
不启用对它们的扫描。

假阳性是被错误地报告为内存泄漏（孤儿）的对象。对于已知不是泄漏的对象，kmemleak 提供了
kmemleak_not_leak 函数。如果已知该内存块不包含其他指针，也可以使用 kmemleak_ignore，
这样它将不再被扫描。

一些报告的泄漏只是暂时性的，在 SMP 系统上尤其如此，因为指针会临时存放在 CPU 寄存器
或栈中。Kmemleak 定义了 MSECS_MIN_AGE（默认为 1000），表示一个对象被报告为内存泄漏所
必须具有的最小存活时间。

### 局限与缺点


主要的缺点是内存分配和释放的性能下降。为了避免其他代价，内存扫描只在读取
/sys/kernel/debug/kmemleak 文件时才执行。总之，这个工具用于调试目的，在这些场景下性能
未必是最重要的要求。

为了让算法保持简单，kmemleak 扫描指向一个块地址范围内任何地址的值。这可能导致假阴性
数量增加。不过，真实的内存泄漏最终很可能会显现出来。

假阴性的另一个来源是存储于非指针值中的数据。在未来的版本中，kmemleak 可以只扫描已分配
结构体中的指针成员。这一特性将解决上面描述的许多假阴性情况。

该工具可能报告假阳性。这些情况包括：一个已分配块不需要被释放（init_call 函数中的某些
情况）、指针是通过 container_of 宏以外的方法计算得到的，或者指针存储在 kmemleak 未
扫描的位置。

页分配和 ioremap 不被跟踪。

### 用 kmemleak-test 进行测试


要检查你是否已准备好使用 kmemleak，可以使用 kmemleak-test 模块，这是一个会故意泄漏
内存的模块。把 CONFIG_SAMPLE_KMEMLEAK 设为模块（它不能用作内建），并用 kmemleak 启动
内核
```
        # modprobe kmemleak-test
        # echo scan > /sys/kernel/debug/kmemleak
```
请注意，你可能不会立即或在第一次扫描时就得到结果。当 kmemleak 得到结果时，它会记录
``kmemleak: <count of leaks> new suspected
```
        # cat /sys/kernel/debug/kmemleak
        unreferenced object 0xffff89862ca702e8 (size 32):
          comm "modprobe", pid 2088, jiffies 4294680594 (age 375.486s)
          hex dump (first 32 bytes):
            6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b  kkkkkkkkkkkkkkkk
            6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b a5  kkkkkkkkkkkkkkk.
          backtrace:
            [<00000000e0a73ec7>] 0xffffffffc01d2036
            [<000000000c5d2a46>] do_one_initcall+0x41/0x1df
            [<0000000046db7e0a>] do_init_module+0x55/0x200
            [<00000000542b9814>] load_module+0x203c/0x2480
            [<00000000c2850256>] __do_sys_finit_module+0xba/0xe0
            [<000000006564e7ef>] do_syscall_64+0x43/0x110
            [<000000007c873fa6>] entry_SYSCALL_64_after_hwframe+0x44/0xa9
        ...
```
用 `rmmod kmemleak_test` 移除该模块也应会触发一些 kmemleak 结果。
