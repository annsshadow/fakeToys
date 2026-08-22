## EEVDF 璋冨害鍣。

“最早合格虚拟截止时间优先”（Earliest Eligible Virtual Deadline First，EEVDF）最早在 1995 年的一篇科学论文中引入 [^1^]。Linux 内核6.6 版本开始转EEVDF（作2024 年的一个新选项），放弃了早期的完全公平调度器（Completely Fair Scheduler，CFS），转而采Peter Zijlstra 2023 年提出的 EEVDF 版本 [2-4]。关CFS 的更多信息可Documentation/scheduler/sched-design-CFS.rst 中找到
CFS 类似，EEVDF 旨在以相同优先级在所有可运行任务间平等地分配 CPU 时间。为此，它为每个任务分配一个虚拟运行时间（virtual run time），并由此产生一个“lag（滞后）”值，可用于判断任务是否已获得其公平的 CPU 时间份额。这样，具有lag 的任务被欠予 CPU 时间，而负 lag 意味着该任务已超出其份额。EEVDF 选取 lag 大于等于零的任务，并为每个任务计算一个虚拟截止时间（VD），选择 VD 最早的那个任务作为下一个执行对象。需要注意的是，这允许具有较短时间片的延迟敏感任务被优先处理，从而有助于提升其响应性
关于如何管理 lag（尤其是对于休眠任务）仍有持续的讨论；但在撰写本文时，EEVDF 使用一种基于虚拟运行时间（VRT）的“衰减（decaying）”机制。这可以防止任务通过短暂休眠来重置其lag 以利用系统：当任务休眠时，它仍留在运行队列中但被标记为“延迟出队（deferred dequeue）”，使其 lag VRT 衰减。因此，长时间休眠的任务最终其 lag 会被重置。最后，若任务的 VD 更早，它就可以抢占其他任务，并且任务可以使用新的 sched_setattr() 系统调用请求特定的时间片，这进一步便利了延迟敏感型应用的工作
## 参

[^1^] https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=805acf7726282721504c8f00575d91ebfd750564

[^2^] https://lore.kernel.org/lkml/a79014e6-ea83-b316-1e12-2ae056bda6fa@linux.vnet.ibm.com/

[^3^] https://lwn.net/Articles/969062/

[^4^] https://lwn.net/Articles/925371/
