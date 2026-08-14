
## TTY 线路规程（Line Discipline）


TTY 线路规程处理进出 tty 设备的所有传入与传出字符。默认的线路规程是 [N_TTY <n_tty>](N_TTY <n_tty>)。如果为某个 tty 建立任何其他规程失败，它也是一个回退选择。如果连 N_TTY 也失败，则由 N_NULL 接管。它永远不会失败，但也不处理任何字符——它会将字符丢弃。

## 注册


线路规程通过 tty_register_ldisc() 并传入 ldisc 结构来注册。在注册之时，该规程必须已准备好可用，并且有可能在调用返回成功之前它就已经被使用。如果调用返回错误，则它不会被调用。不要复用 ldisc 编号，因为它们是用户空间 ABI 的一部分，覆盖已有的 ldisc 会导致恶魔吃掉你的计算机。即使使用相同的数据，你也绝不能在该线路规程之上重新注册，否则你的计算机将再次被恶魔吃掉。要移除一个线路规程，请调用 tty_unregister_ldisc()。

注意此警告：ldisc 表中已注册副本的 tty_ldisc 结构的引用计数字段统计使用该规程的行数。tty 内 tty_ldisc 结构的引用计数统计此刻该 ldisc 的活动用户数。实际上它统计的是 ldisc 方法内的执行线程数（加上那些即将进入和退出的线程，尽管这个细节无关紧要）。

   :identifiers: tty_register_ldisc tty_unregister_ldisc

## 其他函数


   :identifiers: tty_set_ldisc tty_ldisc_flush

## 线路规程操作参考


   :identifiers: tty_ldisc_ops

## 驱动访问


线路规程方法可以调用底层硬件驱动的方法。这些作为 struct tty_operations 的一部分被记录。

## TTY 标志


线路规程方法可以访问 :c`tty_struct.flags` 字段。参见 [tty_struct](tty_struct)。

## 加锁


从 tty 层调用线路规程函数的调用者需要获取线路规程锁。来自驱动侧的调用也是如此，但尚未强制执行。

   :identifiers: tty_ldisc_ref_wait tty_ldisc_ref tty_ldisc_deref

虽然这些函数比旧代码稍慢，但影响应该最小，因为大多数接收逻辑使用 flip 缓冲区，并且它们只需要在通过驱动向上推送比特时获取一次引用。

一个注意点：:c`tty_ldisc_ops.open()`、:c`tty_ldisc_ops.close()` 和 :c`tty_driver.set_ldisc()` 函数是在 ldisc 不可用时被调用的。因此在这些函数内部使用时，tty_ldisc_ref() 会失败。ldisc 与驱动代码在调用其自身函数时必须在此情况下小心。

## 内部函数


   :internal:
