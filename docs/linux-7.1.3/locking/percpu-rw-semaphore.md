## Percpu rw 信号量


Percpu rw semaphores 是一种新的读写信号量设计，
针对读取锁定进行了优化。

传统读写信号量的问题在于，当多个
核心获取读取锁，包含信号量的缓存行
在内核的 L1 缓存之间跳跃，导致性能下降
降解。

读取锁定非常快，它使用 RCU 并且避免任何原子操作
锁定和解锁路径中的指令。另一方面，锁定
写入是非常昂贵的，它调用synchronize_rcu()，可以采取
数百毫秒。

该锁以“struct percpu_rw_semaphore”类型声明。
锁通过 percpu_init_rwsem 初始化，成功时返回 0
和 -ENOMEM 分配失败。
必须使用 percpu_free_rwsem 释放锁以避免内存泄漏。

该锁通过 percpu_down_read、percpu_up_read 和 percpu_down_read 锁定以进行读取
用于使用 percpu_down_write、percpu_up_write 进行写入。

使用 RCU 来优化 rw-lock 的想法是由
埃里克·杜马塞特<eric.dumazet@gmail.com>。
代码由 Mikulas Patocka <mpatocka@redhat.com> 编写
