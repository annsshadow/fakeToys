## CPU 记账控制器


CPU 记账控制器用于通过 cgroups 对任务进行分组，并对这些任务组的 CPU 使用量进行记账。

CPU 记账控制器支持多层级组。一个记账组会累加其所有子组以及直接位于其组中的任务的 CPU 使用量。

```

  # mount -t cgroup -ocpuacct none /sys/fs/cgroup

```
经过上述步骤后，初始或父记账组在 /sys/fs/cgroup 处可见。在启动（bootup）时，该组包含系统中的所有任务。/sys/fs/cgroup/tasks 列出了该 cgroup 中的任务。/sys/fs/cgroup/cpuacct.usage 给出该组获得的 CPU 时间（以纳秒为单位），这本质上就是系统中所有任务获得的 CPU 时间。

```

  # cd /sys/fs/cgroup
  # mkdir g1
  # echo $$ > g1/tasks

```
上述步骤创建了一个新组 g1，并将当前 shell 进程（bash）移入其中。该 bash 及其子进程消耗的 CPU 时间可从 g1/cpuacct.usage 获取，并且同样会累加到 /sys/fs/cgroup/cpuacct.usage 中。

cpuacct.stat 文件列出了一些统计信息，将 cgroup 获得的 CPU 时间进一步划分为用户时间与系统时间。目前支持以下统计信息：

user：cgroup 的任务在用户模式下花费的时间。
system：cgroup 的任务在内核模式下花费的时间。

user 和 system 以 USER_HZ 为单位。

cpuacct 控制器使用 percpu_counter 接口来收集用户时间和系统时间。这有两个副作用：

- 理论上可能看到 user 和 system 时间的错误值。这是因为在 32 位系统上 percpu_counter_read() 对于并发写入并不安全。
- 由于 percpu_counter 的批处理特性，可能会看到略微过时的 user 和 system 时间值。
