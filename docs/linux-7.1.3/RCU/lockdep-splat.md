
## Lockdep-RCU Splat


Lockdep-RCU 2010 年初加入 Linux 内核（http://lwn.net/Articles/371986/）。该机制检查一些常见的
RCU API 误用情况，最典型的是在未加适当保护的情况下使用 rcu_dereference()
系列之一来访问受 RCU 保护的指针。当检测到此类误用时，会发出一lockdep-RCU splat

lockdep-RCU splat 通常的成因是有人访问RCU 保护的数据结构时，既没有
）处于正确类型的 RCU 读侧临界区，也没有（2）持有正确的更新侧锁
因此该问题可能很严重：它可能导致随机的内存覆写或更糟的情况。当然也可能存在
误报，毕竟这就是现实世界

让我们看一个来3.0-rc5 RCU lockdep splat 示例，其中一
```

    =============================
    WARNING: suspicious RCU usage
    -----------------------------
    block/cfq-iosched.c:2776 suspicious rcu_dereference_protected() usage!

```
```

    rcu_scheduler_active = 1, debug_locks = 0
    3 locks held by scsi_scan_6/1552:
    #0:  (&shost->scan_mutex){+.+.}, at: [<ffffffff8145efca>]
    scsi_scan_host_selected+0x5a/0x150
    #1:  (&eq->sysfs_lock){+.+.}, at: [<ffffffff812a5032>]
    elevator_exit+0x22/0x60
    #2:  (&(&q->__queue_lock)->rlock){-.-.}, at: [<ffffffff812b6233>]
    cfq_exit_queue+0x43/0x190

    stack backtrace:
    Pid: 1552, comm: scsi_scan_6 Not tainted 3.0.0-rc5 #17
    Call Trace:
    [<ffffffff810abb9b>] lockdep_rcu_dereference+0xbb/0xc0
    [<ffffffff812b6139>] __cfq_exit_single_io_context+0xe9/0x120
    [<ffffffff812b626c>] cfq_exit_queue+0x7c/0x190
    [<ffffffff812a5046>] elevator_exit+0x36/0x60
    [<ffffffff812a802a>] blk_cleanup_queue+0x4a/0x60
    [<ffffffff8145cc09>] scsi_free_queue+0x9/0x10
    [<ffffffff81460944>] __scsi_remove_device+0x84/0xd0
    [<ffffffff8145dca3>] scsi_probe_and_add_lun+0x353/0xb10
    [<ffffffff817da069>] ? error_exit+0x29/0xb0
    [<ffffffff817d98ed>] ? _raw_spin_unlock_irqrestore+0x3d/0x80
    [<ffffffff8145e722>] __scsi_scan_target+0x112/0x680
    [<ffffffff812c690d>] ? trace_hardirqs_off_thunk+0x3a/0x3c
    [<ffffffff817da069>] ? error_exit+0x29/0xb0
    [<ffffffff812bcc60>] ? kobject_del+0x40/0x40
    [<ffffffff8145ed16>] scsi_scan_channel+0x86/0xb0
    [<ffffffff8145f0b0>] scsi_scan_host_selected+0x140/0x150
    [<ffffffff8145f149>] do_scsi_scan_host+0x89/0x90
    [<ffffffff8145f170>] do_scan_async+0x20/0x160
    [<ffffffff8145f150>] ? do_scsi_scan_host+0x90/0x90
    [<ffffffff810975b6>] kthread+0xa6/0xb0
    [<ffffffff817db154>] kernel_thread_helper+0x4/0x10
    [<ffffffff81066430>] ? finish_task_switch+0x80/0x110
    [<ffffffff817d9c04>] ? retint_restore_args+0xe/0xe
    [<ffffffff81097510>] ? __kthread_init_worker+0x70/0x70
    [<ffffffff817db150>] ? gs_change+0xb/0xb

```
```

	if (rcu_dereference(ioc->ioc_data) == cic) {

```
该形式表明它必须处于一个普通的原生 RCU 读侧临界区中，但上面的“其他信息”列
显示情况并非如此。相反，我们持有三把锁，其中一把可能与 RCU 相关
也许那把锁确实保护了这个引用。如果是这样，修复方法是告知 RCU，例如将
__cfq_exit_single_io_context() 改为cfq_exit_queue() 接收 struct request_queue 的“q”作为参数，
```

	if (rcu_dereference_protected(ioc->ioc_data,
				      lockdep_is_held(&q->queue_lock)) == cic) {

```
经过此修改，若这段代码是RCU 读侧临界区内调用，或是持有了 ->queue_lock
就不会再发出 lockdep-RCU splat。尤其是，由->queue_lock 被持有（见上面列表中#2），
这原本就会抑制上lockdep-RCU splat

另一方面，也许我们确实需要一RCU 读侧临界区。在这种情况下，临界区必须覆
rcu_dereference() 返回值的整个使用过程，或至少持续到某个引用计数被递增之类的操作完成
处理此问题的一种方式是
```

	rcu_read_lock();
	if (rcu_dereference(ioc->ioc_data) == cic) {
		spin_lock(&ioc->lock);
		rcu_assign_pointer(ioc->ioc_data, NULL);
		spin_unlock(&ioc->lock);
	}
	rcu_read_unlock();

```
经过此修改，rcu_dereference() 始终位于 RCU 读侧临界区之内，这同样会抑制上述
lockdep-RCU splat銆。

但在这一特定情况下，我们并没有真正解引用 rcu_dereference() 返回的指针
相反，该指针只是cic 指针进行比较，这意味着 rcu_dereference() 可以替换
```

	if (rcu_access_pointer(ioc->ioc_data) == cic) {

```
由于 rcu_access_pointer() 可以在无保护的情况下合法调用，此修改同样会抑制上
lockdep-RCU splat銆。

