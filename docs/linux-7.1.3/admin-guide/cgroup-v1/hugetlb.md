## HugeTLB Controller


HugeTLB 控制器可以通过先挂载 cgroup 文件系统来创建。

# mount -t cgroup -o hugetlb none /sys/fs/cgroup

经过上述步骤，初始的或父 HugeTLB 组在 /sys/fs/cgroup 处可见。在启动（bootup）时，该组包含系统中所有任务。/sys/fs/cgroup/tasks 列出了该 cgroup 中的任务。

```

  # cd /sys/fs/cgroup
  # mkdir g1
  # echo $$ > g1/tasks

```
上述步骤创建了一个新组 g1，并把当前 shell 进程（bash）移入其中。

```

 hugetlb.<hugepagesize>.rsvd.limit_in_bytes            # set/show limit of "hugepagesize" hugetlb reservations
 hugetlb.<hugepagesize>.rsvd.max_usage_in_bytes        # show max "hugepagesize" hugetlb reservations and no-reserve faults
 hugetlb.<hugepagesize>.rsvd.usage_in_bytes            # show current reservations and no-reserve faults for "hugepagesize" hugetlb
 hugetlb.<hugepagesize>.rsvd.failcnt                   # show the number of allocation failure due to HugeTLB reservation limit
 hugetlb.<hugepagesize>.limit_in_bytes                 # set/show limit of "hugepagesize" hugetlb faults
 hugetlb.<hugepagesize>.max_usage_in_bytes             # show max "hugepagesize" hugetlb  usage recorded
 hugetlb.<hugepagesize>.usage_in_bytes                 # show current usage for "hugepagesize" hugetlb
 hugetlb.<hugepagesize>.failcnt                        # show the number of allocation failure due to HugeTLB usage limit
 hugetlb.<hugepagesize>.numa_stat                      # show the numa information of the hugetlb memory charged to this cgroup

```
对于支持三种大页大小（64k、32M 和 1G）的系统，控制
```

  hugetlb.1GB.limit_in_bytes
  hugetlb.1GB.max_usage_in_bytes
  hugetlb.1GB.numa_stat
  hugetlb.1GB.usage_in_bytes
  hugetlb.1GB.failcnt
  hugetlb.1GB.rsvd.limit_in_bytes
  hugetlb.1GB.rsvd.max_usage_in_bytes
  hugetlb.1GB.rsvd.usage_in_bytes
  hugetlb.1GB.rsvd.failcnt
  hugetlb.64KB.limit_in_bytes
  hugetlb.64KB.max_usage_in_bytes
  hugetlb.64KB.numa_stat
  hugetlb.64KB.usage_in_bytes
  hugetlb.64KB.failcnt
  hugetlb.64KB.rsvd.limit_in_bytes
  hugetlb.64KB.rsvd.max_usage_in_bytes
  hugetlb.64KB.rsvd.usage_in_bytes
  hugetlb.64KB.rsvd.failcnt
  hugetlb.32MB.limit_in_bytes
  hugetlb.32MB.max_usage_in_bytes
  hugetlb.32MB.numa_stat
  hugetlb.32MB.usage_in_bytes
  hugetlb.32MB.failcnt
  hugetlb.32MB.rsvd.limit_in_bytes
  hugetlb.32MB.rsvd.max_usage_in_bytes
  hugetlb.32MB.rsvd.usage_in_bytes
  hugetlb.32MB.rsvd.failcnt



```
1. Page fault accounting

```

  hugetlb.<hugepagesize>.limit_in_bytes
  hugetlb.<hugepagesize>.max_usage_in_bytes
  hugetlb.<hugepagesize>.usage_in_bytes
  hugetlb.<hugepagesize>.failcnt

```
HugeTLB 控制器允许用户限制每个控制组的 HugeTLB 使用量（page fault），并在缺页时强制执行限制。由于 HugeTLB 不支持页面回收（page reclaim），在缺页时强制限制意味着，如果应用程序试图缺页调入超出其限制的 HugeTLB 页面，它将收到 SIGBUS 信号。因此应用程序需要事先确切知道自己使用了多少 HugeTLB 页面，并且系统管理员需要确保机器上有足够的可用页面供所有用户使用，以避免进程收到 SIGBUS。


2. Reservation accounting

```

  hugetlb.<hugepagesize>.rsvd.limit_in_bytes
  hugetlb.<hugepagesize>.rsvd.max_usage_in_bytes
  hugetlb.<hugepagesize>.rsvd.usage_in_bytes
  hugetlb.<hugepagesize>.rsvd.failcnt

```
HugeTLB 控制器允许限制每个控制组的 HugeTLB 预留，并在预留时以及为不存在预留的 HugeTLB 内存缺页时强制执行控制器限制。由于预留限制在预留时（mmap 或 shget 时）强制执行，如果内存事先已预留，预留限制永远不会导致应用程序收到 SIGBUS 信号。对于 MAP_NORESERVE 分配，预留限制的行为与缺页限制相同，在缺页时强制执行内存使用，并在越过限制时导致应用程序收到 SIGBUS。

预留限制优于上面描述的缺页限制，因为预留限制在预留时（mmap 或 shget 时）强制执行，如果内存事先已预留，就永远不会导致应用程序收到 SIGBUS 信号。这使得更容易回退到替代方案，例如非 HugeTLB 内存。而在缺页记账的情况下，由于系统管理员需要精确知道系统中所有任务的 HugeTLB 使用量并确保在所有请求前有足够页面，要避免进程收到 SIGBUS 非常困难。在过量承诺（overcommitted）的系统上，用缺页记账实际上不可能避免任务收到 SIGBUS。


3. Caveats with shared memory

对于共享的 HugeTLB 内存，HugeTLB 预留和缺页都计入第一个导致该内存被预留或缺页的任务，而随后对该已预留或已缺页内存的所有使用都不计入。


共享的 HugeTLB 内存只有在被解除预留或释放时才解除计费。这通常发生在 HugeTLB 文件被删除时，而不是在导致预留或缺页的任务退出时。


4. Caveats with HugeTLB cgroup offline.

当一个 HugeTLB cgroup 在仍有某些预留或缺页计入它的情况下下线时，行为如下：

- 缺页计费被计入父 HugeTLB cgroup（重新归属，reparented），
- 预留计费保留在该离线的 HugeTLB cgroup 上。

这意味着，如果一个 HugeTLB cgroup 在下线时仍有 HugeTLB 预留计入，该 cgroup 会作为僵尸（zombie）一直存在，直到所有 HugeTLB 预留都解除计费。HugeTLB 预留以这种方式工作，是为了与内存控制器保持一致，后者的 cgroup 也会作为僵尸一直存在，直到所有计费内存都解除计费。此外，与追踪 HugeTLB 缺页相比，追踪 HugeTLB 预留要更复杂一些，因此在下线时重新归属预留也要困难得多。
