
## 可扩展调度类


sched_ext 是一个调度类，其行为可以由一BPF 程序——BPF 调度器——来定义
- sched_ext 导出了一个完整的调度接口，从而可以在其上实现任意调度算法
- BPF 调度器可以任意对其认为合适的方式CPU 进行分组，并将它们一起调度，因为
  任务在唤醒时并未绑定到特定的 CPU
- BPF 调度器可以随时动态开启和关闭
- 无论 BPF 调度器做什么，系统完整性都得到保持。在任何时候检测到错误、可运行任务
  停滞，或调用 SysRq 键序`SysRq-S` 时，都会恢复默认的调度行为
- BPF 调度器触发错误时，会转储调试信息以辅助调试。调试转储会传递给调度器二进制
  并由其打印。也可以通过 `sched_ext_dump` tracepoint 访问调试转储。SysRq 键序  `SysRq-D` 会触发调试转储。这不会终止 BPF 调度器，并且只能通过 tracepoint 读取
## 切换进出 sched_ext


`CONFIG_SCHED_CLASS_EXT` 是启sched_ext 的配置选项，`tools/sched_ext` 包含示例
调度器。应使用以下配置选项来使sched_ext

    CONFIG_BPF=y
    CONFIG_SCHED_CLASS_EXT=y
    CONFIG_BPF_SYSCALL=y
    CONFIG_BPF_JIT=y
    CONFIG_DEBUG_INFO_BTF=y
    CONFIG_BPF_JIT_ALWAYS_ON=y
    CONFIG_BPF_JIT_DEFAULT_ON=y

sched_ext 仅在 BPF 调度器已加载并运行时使用
如果任务显式地将其调度策略设置为 `SCHED_EXT`，在 BPF 调度器加载之前，它将被当`SCHED_NORMAL` 并由公平类调度器调度
BPF 调度器已加载`ops->flags` 中未设置 `SCX_OPS_SWITCH_PARTIAL` 时，所`SCHED_NORMAL`、`SCHED_BATCH`、`SCHED_IDLE` `SCHED_EXT` 任务都由 sched_ext 调度
然而，BPF 调度器已加载`ops->flags` 中设置了 `SCX_OPS_SWITCH_PARTIAL` 时，
只有具有 `SCHED_EXT` 策略的任务由 sched_ext 调度，而具`SCHED_NORMAL``SCHED_BATCH` `SCHED_IDLE` 策略的任务由公平类调度器调度，后者的 sched_class
优先级高`SCHED_EXT`
终止 sched_ext 调度器程序、触`SysRq-S`，或检测到包括可运行任务停滞在内的任何
内部错误，都会中BPF 调度器并将所有任务交还给公平类调度器

    # make -j16 -C tools/sched_ext
    # tools/sched_ext/build/bin/scx_simple
    local=0 global=3
    local=5 global=24
    local=9 global=44
    local=13 global=56
    local=17 global=72
    ^CEXIT: BPF scheduler unregistered

BPF 调度器的当前状态可如下确定

    # cat /sys/kernel/sched_ext/state
    enabled
    # cat /sys/kernel/sched_ext/root/ops
    simple

你可以通过检查这个单调递增计数器来判断自启动以来是否曾加载过任BPF 调度（值为零表示尚未加载任BPF 调度器）

    # cat /sys/kernel/sched_ext/enable_seq
    1

每个正在运行的调度器还会`/sys/kernel/sched_ext/<scheduler-name>/events` 下暴一个每调度器的 `events` 文件，用于跟踪诊断计数器。每个计数器占一`name value`

    # cat /sys/kernel/sched_ext/simple/events
    SCX_EV_SELECT_CPU_FALLBACK 0
    SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE 0
    SCX_EV_DISPATCH_KEEP_LAST 123
    SCX_EV_ENQ_SKIP_EXITING 0
    SCX_EV_ENQ_SKIP_MIGRATION_DISABLED 0
    SCX_EV_REENQ_IMMED 0
    SCX_EV_REENQ_LOCAL_REPEAT 0
    SCX_EV_REFILL_SLICE_DFL 456789
    SCX_EV_BYPASS_DURATION 0
    SCX_EV_BYPASS_DISPATCH 0
    SCX_EV_BYPASS_ACTIVATE 0
    SCX_EV_INSERT_NOT_OWNED 0
    SCX_EV_SUB_BYPASS_DISPATCH 0

这些计数器在 `kernel/sched/ext_internal.h` 中有描述；简要地说：

- `SCX_EV_SELECT_CPU_FALLBACK`：ops.select_cpu() 返回了一个任务不可用CPU，核  调度器静默地选择了一个回退 CPU- `SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE`：由于目CPU 下线，本DSQ 分发被重定向  全局 DSQ- `SCX_EV_DISPATCH_KEEP_LAST`：由于没有其它可用任务，一个任务继续运行（仅当未设  `SCX_OPS_ENQ_LAST` 时）- `SCX_EV_ENQ_SKIP_EXITING`：一个正在退出的任务被直接分发到本地 DSQ，绕过了
  ops.enqueue()（仅当未设置 `SCX_OPS_ENQ_EXITING` 时）- `SCX_EV_ENQ_SKIP_MIGRATION_DISABLED`：一个禁用了迁移的任务被直接分发到其本地 DSQ
  （仅当未设置 `SCX_OPS_ENQ_MIGRATION_DISABLED` 时）- `SCX_EV_REENQ_IMMED`：一个以 `SCX_ENQ_IMMED` 分发的任务由于目CPU 无法立即执行
  而被重新入队- `SCX_EV_REENQ_LOCAL_REPEAT`：本DSQ 的重新入队触发了另一次重新入队；反复出现
  的计数表BPF 调度器中 `SCX_ENQ_REENQ` 处理不正确- `SCX_EV_REFILL_SLICE_DFL`：任务的时间片被以默认值（`SCX_SLICE_DFL`）补足- `SCX_EV_BYPASS_DURATION`：在 bypass 模式下花费的总纳秒数- `SCX_EV_BYPASS_DISPATCH`：在 bypass 模式下分发的任务数- `SCX_EV_BYPASS_ACTIVATE`：bypass 模式被激活的次数- `SCX_EV_INSERT_NOT_OWNED`：试图将一个不属于此调度器的任务插DSQ；此类尝试会  静默忽略- `SCX_EV_SUB_BYPASS_DISPATCH`：从子调度器 bypass DSQ 分发的任务（仅与
  `CONFIG_EXT_SUB_SCHED` 相关）
`tools/sched_ext/scx_show_state.py` 是一drgn 脚本，可显示更详细的信息

    # tools/sched_ext/scx_show_state.py
    ops           : simple
    enabled       : 1
    switching_all : 1
    switched_all  : 1
    enable_state  : enabled (2)
    bypass_depth  : 0
    nr_rejected   : 0
    enable_seq    : 1

某个给定任务是否位于 sched_ext 上可如下确定

    # grep ext /proc/self/sched
    ext.enabled                                  :                    1

## 基础


用户空间可以通过加载一组实`struct sched_ext_ops` BPF 程序来实现任BPF
调度器。唯一的必填字段是 `ops.name`，它必须是一个有效的 BPF 对象名。所有操作都可选的。以下经过修改的摘录来自 `tools/sched_ext/scx_simple.bpf.c`，展示了一最小的全局 FIFO 调度器

    /*
     - Decide which CPU a task should be migrated to before being
     - enqueued (either at wakeup, fork time, or exec time). If an
     - idle core is found by the default ops.select_cpu() implementation,
     - then insert the task directly into SCX_DSQ_LOCAL and skip the
     - ops.enqueue() callback.
     *
     - Note that this implementation has exactly the same behavior as the
     - default ops.select_cpu implementation. The behavior of the scheduler
     - would be exactly same if the implementation just didn't define the
     - simple_select_cpu() struct_ops prog.
     */
    s32 BPF_STRUCT_OPS(simple_select_cpu, struct task_struct *p,
                       s32 prev_cpu, u64 wake_flags)
    {
            s32 cpu;
            /** Need to initialize or the BPF verifier will reject the program **/
            bool direct = false;

            cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, &direct);

            if (direct)
                    scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);

            return cpu;
    }

    /*
     - Do a direct insertion of a task to the global DSQ. This ops.enqueue()
     - callback will only be invoked if we failed to find a core to insert
     - into in ops.select_cpu() above.
     *
     - Note that this implementation has exactly the same behavior as the
     - default ops.enqueue implementation, which just dispatches the task
     - to SCX_DSQ_GLOBAL. The behavior of the scheduler would be exactly same
     - if the implementation just didn't define the simple_enqueue struct_ops
     - prog.
     */
    void BPF_STRUCT_OPS(simple_enqueue, struct task_struct *p, u64 enq_flags)
    {
            scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, enq_flags);
    }

    s32 BPF_STRUCT_OPS_SLEEPABLE(simple_init)
    {
            /*
             - By default, all SCHED_EXT, SCHED_OTHER, SCHED_IDLE, and
             - SCHED_BATCH tasks should use sched_ext.
             */
            return 0;
    }

    void BPF_STRUCT_OPS(simple_exit, struct scx_exit_info *ei)
    {
            exit_type = ei->type;
    }

    SEC(".struct_ops")
    struct sched_ext_ops simple_ops = {
            .select_cpu             = (void *)simple_select_cpu,
            .enqueue                = (void *)simple_enqueue,
            .init                   = (void *)simple_init,
            .exit                   = (void *)simple_exit,
            .name                   = "simple",
    };

### 分发队列


为了匹配调度器核心与 BPF 调度器之间的阻抗，sched_ext 使用 DSQ（分发队列），它可以
同时作为 FIFO 和优先级队列运行。默认情况下，有一个全局 FIFO（`SCX_DSQ_GLOBAL`）和
每个 CPU 一个本DSQ（`SCX_DSQ_LOCAL`）。BPF 调度器可以使`scx_bpf_create_dsq()`
`scx_bpf_destroy_dsq()` 管理任意数量DSQ
CPU 总是执行其本DSQ 中的任务。任务被“插入”到一DSQ 中。位于非本地 DSQ 中的
任务被“移动”到目标 CPU 的本DSQ
CPU 寻找下一个要运行的任务时，如果本DSQ 不为空，则选取第一个任务。否则，CPU
尝试从全局 DSQ 移动一个任务。如果那也没有产生可运行任务，则调用 `ops.dispatch()`
### 调度周期


以下简要展示了唤醒的任务如何被调度和执行
1. 当一个任务被唤醒时，`ops.select_cpu()` 是第一个被调用的操作。这有两个目的   第一，CPU 选择优化提示。第二，如果空闲则唤醒所CPU
   `ops.select_cpu()` 选择CPU 是一个优化提示而非绑定。最终的决定在调度的最后一
   步做出。然而，如果 `ops.select_cpu()` 返回CPU 与任务最终运行的 CPU 相匹配，
   会有小小的性能收益
   选择 CPU 的一个副作用是将它从空闲中唤醒。虽BPF 调度器可以使`scx_bpf_kick_cpu()`
   辅助函数唤醒任何 CPU，但明智地使`ops.select_cpu()` 可以更简单、更高效
   注意，调度器核心会忽略无效的 CPU 选择，例如，如果它超出了任务的允cpumask
   一个任务可以通过调用 `scx_bpf_dsq_insert()` `scx_bpf_dsq_insert_vtime()`
   `ops.select_cpu()` 直接插入到一DSQ 中
   如果一个任务从 `ops.select_cpu()` 被插入到 `SCX_DSQ_LOCAL`，它将被添加到从
   `ops.select_cpu()` 返回的那CPU 的本DSQ 中。此外，`ops.select_cpu()`
   直接插入将导致跳`ops.enqueue()` 回调
   任何其它将任务存储在 BPF 内部数据结构中的尝试并不能阻`ops.enqueue()` 被调用   这不鼓励这样做，因为它可能引入竞态行为或不一致状态
2. 一旦目CPU 被选定，就会调`ops.enqueue()`（除非任务是`ops.select_cpu()`
   直接插入的）。`ops.enqueue()` 可以做出以下决定之一
   - 通过调用带以下选项之一`scx_bpf_dsq_insert()` 将任务立即插入全局或本DSQ     `SCX_DSQ_GLOBAL`、`SCX_DSQ_LOCAL` `SCX_DSQ_LOCAL_ON | cpu`
   - 通过调用带有小于 2^63 DSQ ID `scx_bpf_dsq_insert()` 将任务立即插入自定义
     DSQ銆。
   - BPF 侧将任务排队
   **任务状态跟踪与 ops.dequeue() 语义**

   BPF 调度器负责管理一个任务的生命周期时，该任务处于“BPF 调度器的监管（custody）   之中。当一个任务被分发到用DSQ 或存储在 BPF 调度器的内部数据结构中时，它进入
   监管状态。对于这些操作，监管只从 `ops.enqueue()` 进入。唯一的例外是   `ops.select_cpu()` 分发到用DSQ：尽管在那时该任务在技术上尚未处于 BPF 调度   监管中，但对于与监管相关的目的而言，该分发具有与从 `ops.enqueue()` 分发相同   语义效果
   一旦调用了 `ops.enqueue()`，根据调度器的行为，任务可能会或可能不会进入监管
   - **直接分发到终DSQ**（`SCX_DSQ_LOCAL`、`SCX_DSQ_LOCAL_ON | cpu`      `SCX_DSQ_GLOBAL`）：BPF 调度器对该任务的处理已完成——它要么直接进入 CPU 的本     运行队列，要么作为回退进入全局 DSQ。任务永远不会进入（或退出）BPF 监管，并     不会调用 `ops.dequeue()`
   - **分发到用户创建的 DSQ**（自定义 DSQ）：任务进入 BPF 调度器的监管。当任务稍后
     离开 BPF 监管（被分发到终DSQ、被核心调度选中，或因睡属性变更而出队）时，
     `ops.dequeue()` 将被恰好调用一次
   - **存储BPF 数据结构*（例如内BPF 队列）：任务处于 BPF 监管中。当任务
     离开时（例如，当 `ops.dispatch()` 将它移动到终DSQ，或发生属性变睡眠时）     将调`ops.dequeue()`
   当任务离开 BPF 调度器监管时，会调用 `ops.dequeue()`。出队可能因不同原因发生，由
   标志区分
   1. **常规分发**：当处于 BPF 监管中的任务`ops.dispatch()` 被分发到终结 DSQ
      （离开 BPF 监管以执行）时，会触`ops.dequeue()`，不带任何特殊标志
   2. **核心调度选取**：当启用 `CONFIG_SCHED_CORE` 且核心调度在该任务仍处于 BPF 监管
      中时选取它来执行，`ops.dequeue()` 会带`SCX_DEQ_CORE_SCHED_EXEC` 标志被调用
   3. **调度属性变*：当任务属性发生变化（通过 `sched_setaffinity()`      `sched_setscheduler()`、优先级变更、CPU 迁移等操作）而任务仍处于 BPF 监管中时      `ops.dequeue()` 会被调用，并`deq_flags` 中设`SCX_DEQ_SCHED_CHANGE` 标志
   **重要**：一旦任务离开BPF 监管（例如被分发到终DSQ 之后），属性变更将不会触发
   `ops.dequeue()`，因为该任务不再BPF 调度器管理
3. 当一CPU 准备好调度时，它首先查看其本DSQ。如果为空，则查看全局 DSQ。如   仍然没有可运行的任务，则调用 `ops.dispatch()`，它可以使用以下两个函数来填充本   DSQ
   - `scx_bpf_dsq_insert()` 将一个任务插DSQ。可以使用任何目DSQ——`SCX_DSQ_LOCAL`     `SCX_DSQ_LOCAL_ON | cpu`、`SCX_DSQ_GLOBAL` 或自定义 DSQ。虽`scx_bpf_dsq_insert()`
     目前不能在持BPF 锁的情况下调用，但这一限制正在改进中并将被支持     `scx_bpf_dsq_insert()` 安排插入而非立即执行。最多可以有 `ops.dispatch_max_batch`
     个待处理任务
   - `scx_bpf_dsq_move_to_local()` 将任务从指定的非本地 DSQ 移动到正在分发的 DSQ     此函数不能在持有任何 BPF 锁的情况下调用。`scx_bpf_dsq_move_to_local()` 在尝试从
     指定 DSQ 移动之前会刷新待处理的插入任务
4. `ops.dispatch()` 返回后，如果本地 DSQ 中有任务，CPU 运行第一个。如果为空，则采   以下步骤
   - 尝试从全局 DSQ 移动。如果成功，运行该任务
   - 如果 `ops.dispatch()` 已分发过任何任务，重#3
   - 如果前一个任务是 SCX 任务且仍然可运行，继续运行它（见 `SCX_OPS_ENQ_LAST`）
   - 进入空闲
注意，BPF 调度器总是可以选择`ops.enqueue()` 中立即分发任务，如上面的简单示所示。如果只使用内置 DSQ，则无需实现 `ops.dispatch()`，因为任务永远不会在 BPF
调度器上排队，并且本地和全局 DSQ 都会自动执行
`scx_bpf_dsq_insert()` 将任务插入目DSQ FIFO。对优先级队列请使用
`scx_bpf_dsq_insert_vtime()`。内DSQ（如 `SCX_DSQ_LOCAL` `SCX_DSQ_GLOBAL`）不
支持优先级队列分发，必须`scx_bpf_dsq_insert()` 分发。更多信息请参阅
`tools/sched_ext/scx_simple.bpf.c` 中的函数文档和用法
### 任务生命周期


以下伪代码大致概述了sched_ext 调度器管理的任务的整个生命周期：


    ops.init_task();            /** A new task is created **/
    ops.enable();               /** Enable BPF scheduling for the task **/

    while (task in SCHED_EXT) {
        if (task can migrate)
            ops.select_cpu();   /** Called on wakeup (optimization) **/

        ops.runnable();         /** Task becomes ready to run **/

        while (task_is_runnable(task)) {
            if (task is not in a DSQ || task->scx.slice == 0) {
                ops.enqueue();  /** Task can be added to a DSQ **/

                /** Task property change (i.e., affinity, nice, etc.)? **/
                if (sched_change(task)) {
                    ops.dequeue(); /** Exiting BPF scheduler custody **/
                    ops.quiescent();

                    /** Property change callback, e.g. ops.set_weight() **/

                    ops.runnable();
                    continue;
                }

                /** Any usable CPU becomes available **/

                ops.dispatch();     /** Task is moved to a local DSQ **/
                ops.dequeue();      /** Exiting BPF scheduler custody **/
            }

            ops.running();      /** Task starts running on its assigned CPU **/

            while (task_is_runnable(task) && task->scx.slice > 0) {
                ops.tick();     /** Called every 1/HZ seconds **/

                if (task->scx.slice == 0)
                    ops.dispatch(); /** task->scx.slice can be refilled **/
            }

            ops.stopping();     /** Task stops running (time slice expires or wait) **/
        }

        ops.quiescent();        /** Task releases its assigned CPU (wait) **/
    }

    ops.disable();              /** Disable BPF scheduling for the task **/
    ops.exit_task();            /** Task is destroyed **/

注意，上述伪代码并未涵盖所有可能的状态转换和边界情况，仅举几个例子：

- `ops.dispatch()` 可能由于该任务上的竞态属性变更而未能将任务移动到本DSQ，在这种
  情况`ops.dispatch()` 将被重试
- 任务可能`ops.enqueue()` 被直接分发到本地 DSQ，在这种情况下会跳过 `ops.dispatch()`
  `ops.dequeue()`，直接进`ops.running()`
- 属性变更可能发生在任务生命周期的几乎任何时刻，而不仅仅是在任务排队并等待分发时  例如，更改正在运行的任务的属性将导致回调序列 `ops.stopping()` -> `ops.quiescent()`
  ->（属性变更回调）-> `ops.runnable()` -> `ops.running()`
- 一sched_ext 任务可能被来自更高优先级调度类的任务抢占，在这种情况下，即使它是
  可运行的并且具有非零时间片，它也会退tick-dispatch 循环
有关刚唤醒的任务如何CPU 的更详细描述，请参见“调度周期”一节
## 参考位

- `include/linux/sched/ext.h` 定义了核心数据结构、ops 表和常量
- `kernel/sched/ext.c` 包含 sched_ext 核心实现和辅助函数。以 `scx_bpf_` 为前缀  函数可以BPF 调度器调用
- `kernel/sched/ext_idle.c` 包含内置的空CPU 选择策略
- `tools/sched_ext/` 托管示例 BPF 调度器实现
  - `scx_simple[.bpf].c`：使用自定义 DSQ 的最小全局 FIFO 调度器示例
  - `scx_qmap[.bpf].c`：一个多FIFO 调度器，使用 `BPF_MAP_TYPE_QUEUE` 实现五级
    优先级
  - `scx_central[.bpf].c`：一个中FIFO 调度器，所有调度决策都在一CPU 上做出，
    演示`LOCAL_ON` 分发、无滴答操作以及 kthread 抢占
  - `scx_cpu0[.bpf].c`：一个将所有任务排队到共享 DSQ 并仅CPU0 上以 FIFO 顺序分发    调度器。对测试 bypass 行为很有用
  - `scx_flatcg[.bpf].c`：一个扁平化 cgroup 层级调度器，通过将每cgroup 的份额在
    每一级复合为单一的扁平调度层，实现基于层级权重的 cgroup CPU 控制
  - `scx_pair[.bpf].c`：一个核心调度示例，总是让兄CPU 对执行来自同一 CPU cgroup
    的任务
  - `scx_sdt[.bpf].c`：`scx_simple` 的一个变体，演示了用于每任务数据BPF arena
    内存管理
  - `scx_userland[.bpf].c`：一个最小调度器，演示用户空间调度。具CPU 亲和性的任务
    FIFO 顺序直接分发；所有其它任务由一个简单的 vruntime 调度器在用户空间中调度
## 模块参数


sched_ext `sched_ext.` 前缀下暴露两个模块参数，用于控制 bypass 模式行为。这旋钮主要用于调试；在正常操作期间通常没有理由更改它们。它们可以在运行时（模式 0600通过 `/sys/module/sched_ext/parameters/` 读写
`sched_ext.slice_bypass_us`（默认：5000 µs    当调度器处于 bypass 模式（即BPF 调度器加载、卸载和错误恢复期间）时分配给所    任务的时间片。有效范围是 100 µs 100 ms
`sched_ext.bypass_lb_intv_us`（默认：500000 µs    bypass 模式负载平衡器在 CPU 之间重新分配任务的间隔。设0 可在 bypass 模式期间
    禁用负载平衡。有效范围是 0 10 s
## ABI 不稳定

sched_ext 提供BPF 调度器程序的 API 没有稳定性保证。这包括`include/linux/sched/ext.h`
中定义的 ops 表回调和常量，以`kernel/sched/ext.c` `kernel/sched/ext_idle.c`
中定义的 `scx_bpf_` kfunc
虽然我们会在可能的情况下尝试提供一个相对稳定的 API 面，但它们在不同内核版本之间可能
在没有任何警告的情况下发生变化