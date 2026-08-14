## Cgroup 冻结器（Freezer）


cgroup 冻结器对批处理作业管理系统很有用，这类系统会按系统管理员的意愿
启动和停止一组任务，以调度机器的资源。这种程序常用于 HPC 集群，以
整体调度对集群的访问。cgroup 冻结器使用 cgroup 来描述要由批处理作业管理
系统启动/停止的任务集合。它还提供了启动和停止组成该作业的任务的手段。

cgroup 冻结器对检查点（checkpointing）正在运行的任务组也很有用。冻结器
允许检查点代码通过尝试将 cgroup 中的任务强制进入静止（quiescent）状态来获取
任务的一致镜像。一旦任务静止，另一个任务就可以遍历 /proc 或调用内核接口来
收集有关这些静止任务的信息。如果发生可恢复的错误，被检查点的任务可以在
之后重新启动。这也允许通过将收集到的信息复制到另一个节点并在那里重启任务，
在集群中的节点之间迁移被检查点的任务。

在用户空间中，SIGSTOP 与 SIGCONT 的序列并不总是足以停止和恢复任务。这两个
信号都可以从我们希望冻结的任务内部观察到。虽然 SIGSTOP 不能被捕获、阻塞或
忽略，但它可以被等待或 ptrace 的父任务看到。SIGCONT 尤其不合适，因为它可以被
任务捕获。任何设计用来监视 SIGSTOP 和 SIGCONT 的程序都可能因尝试使用 SIGSTOP 和
SIGCONT 来停止和恢复任务而被破坏。我们可以
```

	$ echo $$
	16644
	$ bash
	$ echo $$
	16690

	From a second, unrelated bash shell:
	$ kill -SIGSTOP 16690
	$ kill -SIGCONT 16690

	<at this point 16690 exits and causes 16644 to exit too>

```
这发生是因为 bash 可以观察到这两个信号并选择如何响应它们。

另一个捕获并响应这些信号的程序示例是 gdb。事实上，任何设计使用 ptrace 的
程序都可能在使用这种停止和恢复任务的方法时遇到问题。

相反，cgroup 冻结器使用内核冻结器代码，防止冻结/解冻周期对被冻结的任务
可见。这使得上面的 bash 示例和 gdb 能够如预期般运行。

cgroup 冻结器是分层的。冻结一个 cgroup 会冻结属于该 cgroup 及其所有后代
cgroup 的所有任务。每个 cgroup 都有自己的状态（self-state，自身状态）以及
从父级继承的状态（parent-state，父状态）。当且仅当两个状态都为 THAWED 时，
该 cgroup 才是 THAWED。

cgroup 冻结器创建以下 cgroupfs 文件。

- freezer.state：可读写。

  读取时，返回 cgroup 的有效状态——“THAWED”、“FREEZING”或“FROZEN”。
  这是自身状态与父状态的结合。如果任一个正在冻结，则该 cgroup 正在冻结
  （FREEZING 或 FROZEN）。

  FREEZING 的 cgroup 在属于该 cgroup 及其所有后代的任务都变为冻结时，
  转换为 FROZEN 状态。注意，在将一个新任务添加到该 cgroup 或其某个后代
  cgroup 之后，直到新任务被冻结之前，cgroup 会从 FROZEN 回退到 FREEZING。

  写入时，设置 cgroup 的自身状态。允许两个值——“FROZEN”和“THAWED”。
  如果写入 FROZEN，则该 cgroup（如果尚未在冻结中）连同其所有后代 cgroup
  一起进入 FREEZING 状态。

  如果写入 THAWED，则 cgroup 的自身状态改为 THAWED。注意，如果父状态仍在
  冻结中，有效状态可能不会改变为 THAWED。如果某个 cgroup 的有效状态变为
  THAWED，则所有因其冻结的后代也会离开冻结状态。

- freezer.self_freezing：只读。

  显示自身状态。如果自身状态为 THAWED 则为 0，否则为 1。
  当且仅当对 freezer.state 的最后一次写入是 “FROZEN” 时，该值为 1。

- freezer.parent_freezing：只读。

  显示父状态。如果该 cgroup 的祖先都没有被冻结则为 0，否则为 1。

根 cgroup 是不可冻结的，上述接口文件不存在。

```

   # mkdir /sys/fs/cgroup/freezer
   # mount -t cgroup -ofreezer freezer /sys/fs/cgroup/freezer
   # mkdir /sys/fs/cgroup/freezer/0
   # echo $some_pid > /sys/fs/cgroup/freezer/0/tasks

```
```

   # cat /sys/fs/cgroup/freezer/0/freezer.state
   THAWED

```
```

   # echo FROZEN > /sys/fs/cgroup/freezer/0/freezer.state
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   FREEZING
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   FROZEN

```
```

   # echo THAWED > /sys/fs/cgroup/freezer/0/freezer.state
   # cat /sys/fs/cgroup/freezer/0/freezer.state
   THAWED

```
这是一个基本机制，在简单场景下应该能为用户空间任务做正确的事。

该冻结器实现受到缺陷的影响（参见提交 76f969e8948d8（“cgroup: cgroup v2 freezer”）），
建议使用 cgroup v2 冻结器。
