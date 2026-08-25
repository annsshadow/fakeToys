## 控制组统计（Control Groupstats

Control Groupstats 的灵感来https://lore.kernel.org/r/461CF883.2030308@sw.ru 的讨论，并实现了 Andrew Morton https://lore.kernel.org/r/20070411114927.1277d7c9.akpm@linux-foundation.org 中建议的cgroup 统计
cgroup 统计的基础设施复用taskstats 接口的代码。一组新cgroup 操作
cgroup 特定的命令与属性注册。通过cgroupstats 结构添加成员，扩cgroup 统计应当非常容易
cgroupstats 当前的模型是拉取式，推送式模型（在发生有趣事件时上报统计）应当
非常容易添加。当前用户空间通过传cgroup 路径来请求统计关于 cgroup 中所有任务状态的统计返回给用户空间
注意：目前我们依赖延迟统计来提取I/O 阻塞的任务信息。如果禁用了
CONFIG_TASK_DELAY_ACCT，该信息将不可用
要提cgroup 统计，使用一个与 getdelays.c 非常相似的工```

  ~/balbir/cgroupstats # ./getdelays  -C "/sys/fs/cgroup/a"
  sleeping 1, blocked 0, running 1, stopped 0, uninterruptible 0
  ~/balbir/cgroupstats # ./getdelays  -C "/sys/fs/cgroup"
  sleeping 155, blocked 0, running 1, stopped 0, uninterruptible 2

```
