## 进程数控制器（Process Number Controller）


### 摘要


进程数控制器用于允许 cgroup 层级在达到某个限制后阻止任何新任务被 fork() 或 clone()。

由于在不触及任何 kmemcg 限制的情况下就很容易达到任务上限，PID 是一种基本资源。因此，必须通过允许对 cgroup 中任务数量进行资源限制，在 cgroup 层级范围内预防 PID 耗尽。

### 用法


要使用 `pids` 控制器，设置 pids.max 中的最大任务数（出于显而易见的原因，这在根 cgroup 中不可用）。cgroup 中当前的进程数由 pids.current 给出。

组织操作不会被 cgroup 策略阻塞，因此可能出现 pids.current > pids.max。这可能是通过将限制设置为小于 pids.current，或者将足够多的进程附加到 cgroup 使得 pids.current > pids.max 来实现的。但是，不可能通过 fork() 或 clone() 违反 cgroup 策略。如果创建新进程会导致违反 cgroup 策略，fork() 和 clone() 将返回 -EAGAIN。

要将某个 cgroup 设为无限制，将 pids.max 设为 “max”。这是所有新 cgroup 的默认值（注意：PID 限制是分层的，因此遵循层级中最严格的限制）。

pids.current 追踪所有子 cgroup 层级，因此 parent/pids.current 是 parent/child/pids.current 的超集。

pids.events 文件包含事件计数器：

  - max：在自身或祖先中因达到限制而导致 fork 失败的次数。

### 示例


```

	# mkdir -p /sys/fs/cgroup/pids
	# mount -t cgroup -o pids none /sys/fs/cgroup/pids

```
```

	# mkdir -p /sys/fs/cgroup/pids/parent/child
	# echo 2 > /sys/fs/cgroup/pids/parent/pids.max
	# echo $$ > /sys/fs/cgroup/pids/parent/cgroup.procs
	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	#

```
应注意，试图突破设定的限制（本例中为 2）将
```

	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	# ( /bin/echo "Here's some processes for you." | cat )
	sh: fork: Resource temporary unavailable
	#

```
即使我们迁移到子 cgroup（它没有设定限制），我们也无法突破层级中最严格的限制（本例中，
```

	# echo $$ > /sys/fs/cgroup/pids/parent/child/cgroup.procs
	# cat /sys/fs/cgroup/pids/parent/pids.current
	2
	# cat /sys/fs/cgroup/pids/parent/child/pids.current
	2
	# cat /sys/fs/cgroup/pids/parent/child/pids.max
	max
	# ( /bin/echo "Here's some processes for you." | cat )
	sh: fork: Resource temporary unavailable
	#

```
我们可以设置一个小于 pids.current 的限制，这将完全阻止任何新进程被 fork（注意 shell 本身也算作
```

	# echo 1 > /sys/fs/cgroup/pids/parent/pids.max
	# /bin/echo "We can't even spawn a single process now."
	sh: fork: Resource temporary unavailable
	# echo 0 > /sys/fs/cgroup/pids/parent/pids.max
	# /bin/echo "We can't even spawn a single process now."
	sh: fork: Resource temporary unavailable
	#

```
