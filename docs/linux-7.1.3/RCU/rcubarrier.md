
## RCU 与可卸载模块


[Originally published in LWN Jan. 14, 2007: http://lwn.net/Articles/217484/]

RCU 更新者有时会使用 `call_rcu()` 来发起一次异步等待，直到宽限期（grace
period）结束。该原语接受一个指向 RCU 保护数据结构内部的 `rcu_head` 结构体的
指针，以及另一个指向某个函数的指针，该函数可能在稍后被调用来释放该结构体。
从 IRQ 上下文中删除链表中元素 p 的代码可能如下：
```
	list_del_rcu(p);
	call_rcu(&p->rcu, p_callback);
```
由于 `call_rcu()` 从不阻塞，因此这段代码可以安全地用于
```
	static void p_callback(struct rcu_head *rp)
	{
		struct pstruct *p = container_of(rp, struct pstruct, rcu);

		kfree(p);
	}
```

### 卸载使用 call_rcu() 的模块


但如果 `p_callback()` 函数定义在一个可卸载的模块中呢？

如果我们在某些 RCU 回调仍然挂起时卸载该模块，那么稍后执行这些回调的 CPU
在调用时将遭遇严重的不愉快，相关情形可参见
http://lwn.net/images/ns/kernel/rcu-drop.jpg 中的生动描绘。

我们可以尝试在模块的退出代码路径中放置一个 `synchronize_rcu()`，但这并不
充分。尽管 `synchronize_rcu()` 确实会等待一个宽限期结束，但它并不会等待
回调完成。

有人可能想连续调用几个 `synchronize_rcu()`，但这仍然无法保证可行。如果存在
非常繁重的 RCU 回调负载，那么某些回调可能会被延后，以便让其他处理得以继续
进行。仅举一例：在实时内核中，为了避免过度的调度延迟，这种延后是必需的。

### rcu_barrier()


这种情况可以通过 `rcu_barrier()` 原语来处理。与等待宽限期结束不同，
`rcu_barrier()` 等待所有未决的 RCU 回调完成。请注意，`rcu_barrier()` 并
**不** 隐含 `synchronize_rcu()`；特别地，如果任何地方都没有排队的 RCU 回调，
`rcu_barrier()` 有权立即返回，而无需等待任何事情，更不用说宽限期了。

使用 `rcu_barrier()` 的伪代码如下：
```
   1. Prevent any new RCU callbacks from being posted.
   2. Execute rcu_barrier().
   3. Allow the module to be unloaded.
```
针对 SRCU 还有一个 `srcu_barrier()` 函数，当然你必须使 `srcu_barrier()` 的
类型与 `call_srcu()` 相匹配。如果你的模块使用了多个 `srcu_struct` 结构体，
那么在卸载该模块时也必须多次调用 `srcu_barrier()`。例如，如果它使用了
`call_rcu()`、`srcu_struct_1` 上的 `call_srcu()`，以及 `srcu_struct_2` 上的
`call_srcu()`，那么下面这三行代码
```
  1  rcu_barrier();
  2  srcu_barrier(&srcu_struct_1);
  3  srcu_barrier(&srcu_struct_2);
```
如果延迟至关重要，可以使用工作队列（workqueue）并发地运行这三个函数。

rcutorture 模块的一个古老版本使用了 `rcu_barrier()`：
```
  1  static void
  2  rcu_torture_cleanup(void)
  3  {
  4    int i;
  5
  6    fullstop = 1;
  7    if (shuffler_task != NULL) {
  8      VERBOSE_PRINTK_STRING("Stopping rcu_torture_shuffle task");
  9      kthread_stop(shuffler_task);
 10    }
 11    shuffler_task = NULL;
 12
 13    if (writer_task != NULL) {
 14      VERBOSE_PRINTK_STRING("Stopping rcu_torture_writer task");
 15      kthread_stop(writer_task);
 16    }
 17    writer_task = NULL;
 18
 19    if (reader_tasks != NULL) {
 20      for (i = 0; i < nrealreaders; i++) {
 21        if (reader_tasks[i] != NULL) {
 22          VERBOSE_PRINTK_STRING(
 23            "Stopping rcu_torture_reader task");
 24          kthread_stop(reader_tasks[i]);
 25        }
 26        reader_tasks[i] = NULL;
 27      }
 28      kfree(reader_tasks);
 29      reader_tasks = NULL;
 30    }
 31    rcu_torture_current = NULL;
 32
 33    if (fakewriter_tasks != NULL) {
 34      for (i = 0; i < nfakewriters; i++) {
 35        if (fakewriter_tasks[i] != NULL) {
 36          VERBOSE_PRINTK_STRING(
 37            "Stopping rcu_torture_fakewriter task");
 38          kthread_stop(fakewriter_tasks[i]);
 39        }
 40        fakewriter_tasks[i] = NULL;
 41      }
 42      kfree(fakewriter_tasks);
 43      fakewriter_tasks = NULL;
 44    }
 45
 46    if (stats_task != NULL) {
 47      VERBOSE_PRINTK_STRING("Stopping rcu_torture_stats task");
 48      kthread_stop(stats_task);
 49    }
 50    stats_task = NULL;
 51
 52    /* Wait for all RCU callbacks to fire. */
 53    rcu_barrier();
 54
 55    rcu_torture_stats_print(); /* -After- the stats thread is stopped! */
 56
 57    if (cur_ops->cleanup != NULL)
 58      cur_ops->cleanup();
 59    if (atomic_read(&n_rcu_torture_error))
 60      rcu_torture_print_module_parms("End of test: FAILURE");
 61    else
 62      rcu_torture_print_module_parms("End of test: SUCCESS");
 63  }
```
第 6 行设置一个全局变量，阻止任何 RCU 回调再次提交自身。在大多数情况下这并非
必要，因为 RCU 回调很少包含对 `call_rcu()` 的调用。不过，rcutorture 模块是
这条规则的一个例外，因此需要设置该全局变量。

第 7-50 行停止所有与 rcutorture 模块关联的 kernel task。因此，一旦执行到达
第 53 行，就不会再有 rcutorture 的 RCU 回调被提交。第 53 行的 `rcu_barrier()`
调用会等待任何预先存在的回调完成。

然后第 55-62 行打印状态并进行特定于操作的清理，之后返回，从而允许模块卸载
操作完成。

Quick Quiz #1:
	Is there any other situation where rcu_barrier() might
	be required?

Answer to Quick Quiz #1 <answer_rcubarrier_quiz_1>

你的模块可能会有额外的复杂情况。例如，如果你的模块从定时器（timer）中调用
`call_rcu()`，你将需要先停止提交新的定时器、取消（或等待）所有已经提交的
定时器，然后才能调用 `rcu_barrier()` 来等待任何剩余的 RCU 回调完成。

当然，如果你的模块使用 `call_rcu()`，你需要在卸载前调用 `rcu_barrier()`。
类似地，如果你的模块使用 `call_srcu()`，你需要在卸载前调用 `srcu_barrier()`，
且要在同一个 `srcu_struct` 结构体上。如果你的模块同时使用了 `call_rcu()`
**和** `call_srcu()`，那么（如上所述）你需要同时调用 `rcu_barrier()`
**和** `srcu_barrier()`。

### 实现 rcu_barrier()


Dipankar Sarma 对 `rcu_barrier()` 的实现利用了这样一个事实：一旦 RCU 回调被
排队到某个每-CPU 队列上，它们就永远不会被重排序。他的实现在每个每-CPU 回调
队列上都排队一个 RCU 回调，然后等待它们全部开始执行；此时，所有更早的 RCU
回调就保证已经完成了。
```
  1  void rcu_barrier(void)
  2  {
  3    BUG_ON(in_interrupt());
  4    /* Take cpucontrol mutex to protect against CPU hotplug */
  5    mutex_lock(&rcu_barrier_mutex);
  6    init_completion(&rcu_barrier_completion);
  7    atomic_set(&rcu_barrier_cpu_count, 1);
  8    on_each_cpu(rcu_barrier_func, NULL, 0, 1);
  9    if (atomic_dec_and_test(&rcu_barrier_cpu_count))
 10      complete(&rcu_barrier_completion);
 11    wait_for_completion(&rcu_barrier_completion);
 12    mutex_unlock(&rcu_barrier_mutex);
 13  }
```
第 3 行验证调用者处于进程上下文，第 5 行和第 12 行使用 `rcu_barrier_mutex`
确保同一时刻只有一个 `rcu_barrier()` 在使用全局 completion 和计数器，这些
在 6、7 行被初始化。第 8 行使每个 CPU 调用 `rcu_barrier_func()`，如下所示。
注意，`on_each_cpu()` 参数列表末尾的 “1” 确保了所有对 `rcu_barrier_func()`
的调用都将在 `on_each_cpu()` 返回前完成。第 9 行从 `rcu_barrier_cpu_count`
中移除初始计数，如果该计数现在为零，第 10 行就完成 completion，从而阻止第 11
行阻塞。无论哪种情况，第 11 行随后（如果需要）等待 completion。

Quick Quiz #2:
	Why doesn't line 8 initialize rcu_barrier_cpu_count to zero,
	thereby avoiding the need for lines 9 and 10?

Answer to Quick Quiz #2 <answer_rcubarrier_quiz_2>

这段代码在 2008 年以及此后又重写了几次，但大体思路依然如此。

`rcu_barrier_func()` 在每个 CPU 上运行，在运行时它调用 `call_rcu()`：
```
  1  static void rcu_barrier_func(void *notused)
  2  {
  3    int cpu = smp_processor_id();
  4    struct rcu_data *rdp = &per_cpu(rcu_data, cpu);
  5    struct rcu_head *head;
  6
  7    head = &rdp->barrier;
  8    atomic_inc(&rcu_barrier_cpu_count);
  9    call_rcu(head, rcu_barrier_callback);
 10  }
```
第 3、4 行定位 RCU 内部的每-CPU `rcu_data` 结构体，其中包含稍后调用
`call_rcu()` 所需的 `struct rcu_head`。第 7 行取得指向该 `struct rcu_head`
的指针，第 8 行递增全局计数器。该计数器稍后会被回调递减。第 9 行随后在当前
CPU 的队列上注册 `rcu_barrier_callback()`。

`rcu_barrier_callback()` 函数只是原子地递减 `rcu_barrier_cpu_count` 变量，
并在其为零时完成 completion：
```
  1  static void rcu_barrier_callback(struct rcu_head *notused)
  2  {
  3    if (atomic_dec_and_test(&rcu_barrier_cpu_count))
  4      complete(&rcu_barrier_completion);
  5  }
```

Quick Quiz #3:
	What happens if CPU 0's rcu_barrier_func() executes
	immediately (thus incrementing rcu_barrier_cpu_count to the
	value one), but the other CPU's rcu_barrier_func() invocations
	are delayed for a full grace period? Couldn't this result in
	rcu_barrier() returning prematurely?

Answer to Quick Quiz #3 <answer_rcubarrier_quiz_3>

当前的 `rcu_barrier()` 实现更为复杂，因为需要避免打扰空闲 CPU（尤其是在
电池供电的系统上），并且需要最小化对实时系统非空闲 CPU 的打扰。此外还应用了
大量优化。不过，上面的代码说明了其原理。

### rcu_barrier() 小结


`rcu_barrier()` 原语使用得相对较少，因为大多数使用 RCU 的代码位于核心内核
而非模块中。不过，如果你在可卸载模块中使用 RCU，就需要使用 `rcu_barrier()`，
以便你的模块能够被安全卸载。

### 快速测验答案


Quick Quiz #1:
	Is there any other situation where rcu_barrier() might
	be required?

Answer:
	有趣的是，`rcu_barrier()` 最初并不是为实现模块卸载而实现的。Nikita
	Danilov 曾在一个文件系统中使用 RCU，结果在文件系统卸载时遇到了类似的
	情况。Dipankar Sarma 为此编写了 `rcu_barrier()`，以便 Nikita 可以在
	文件系统卸载过程中调用它。

	很久以后，本人在实现 rcutorture 时遇到了 RCU 模块卸载问题，并发现
	`rcu_barrier()` 同样解决了这个问题。

Back to Quick Quiz #1 <rcubarrier_quiz_1>


Quick Quiz #2:
	Why doesn't line 8 initialize rcu_barrier_cpu_count to zero,
	thereby avoiding the need for lines 9 and 10?

Answer:
	假设第 8 行所示的 `on_each_cpu()` 函数被延迟了，使得 CPU 0 的
	`rcu_barrier_func()` 先执行、对应的宽限期也先结束，而这一切都发生在
	CPU 1 的 `rcu_barrier_func()` 开始执行之前。这将导致 `rcu_barrier_cpu_count`
	被递减到零，从而第 11 行的 `wait_for_completion()` 会立即返回，未能等待
	CPU 1 的回调被调用。

	注意，在 `rcu_barrier()` 代码于 2005 年首次加入时，这并不是一个
	问题。这是因为 `on_each_cpu()` 会禁用抢占，而禁用抢占等同于一个 RCU
	读端临界区，从而阻止 CPU 0 的宽限期在 `on_each_cpu()` 处理完所有 CPU
	之前完成。

	不过，随着 v4.20 前后的 RCU 类型合并，这种可能性再次被排除，因为合并后
	的 RCU 会再次等待非抢占的代码区域。

	尽管如此，那个额外的计数可能仍是个好主意。依赖这类实现上的偶然性，可能会
	在实现发生变化时导致日后令人惊讶的 bug。

Back to Quick Quiz #2 <rcubarrier_quiz_2>


Quick Quiz #3:
	What happens if CPU 0's rcu_barrier_func() executes
	immediately (thus incrementing rcu_barrier_cpu_count to the
	value one), but the other CPU's rcu_barrier_func() invocations
	are delayed for a full grace period? Couldn't this result in
	rcu_barrier() returning prematurely?

Answer:
	这种情况不会发生。原因在于 `on_each_cpu()` 的最后一个参数、即等待标志
	被设为 “1”。该标志会被传递进 `smp_call_function()`，并进一步传递到
	`smp_call_function_on_cpu()`，使得后者自旋，直到跨 CPU 的 `rcu_barrier_func()`
	调用完成为止。这本身就能阻止宽限期在非 `CONFIG_PREEMPTION` 内核上完成，
	因为在宽限期完成之前，每个 CPU 都必须经历一次上下文切换（或其他静止状态）。
	然而，这在 `CONFIG_PREEMPTION` 内核中毫无用处。

	因此，`on_each_cpu()` 会在其调用 `smp_call_function()` 的整个过程中以及
	本地调用 `rcu_barrier_func()` 的过程中禁用抢占。由于近期的 RCU 实现将禁用
	抢占的代码区域视为 RCU 读端临界区，这就阻止了宽限期完成。这意味着在所有
	CPU 都执行完 `rcu_barrier_func()` 之前，第一个 `rcu_barrier_callback()`
	都不可能执行，进而阻止 `rcu_barrier_cpu_count` 过早地达到零。

	但是，如果 `on_each_cpu()` 决定放弃禁用抢占（由于实时延迟方面的考量这很
	有可能发生），那么将 `rcu_barrier_cpu_count` 初始化为 1 就会挽救局面。

Back to Quick Quiz #3 <rcubarrier_quiz_3>
