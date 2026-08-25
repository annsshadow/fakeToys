
## RCU 概念


RCU（read-copy update，读-复制-更新）背后的基本思想是将破坏性操作拆分为两部分，一部分阻止任何人看到正在被销毁的数据项，另一部分实际执行销毁。这两部分之间必须经历一个“宽限期（grace period）”，且该宽限期必须足够长，使得任何正在访问被删除项的读者此后都已放弃其引用。例如，RCU 保护的链表进行删除，会先将该项从链表中移除，等待宽限期过去，然后释放该元素。关于在链表上使RCU 的更多信息，请参listRCU.rst
### 常见问题


- 为什么会有人想要使用 RCU
  RCU 两分法方法的优势在于 RCU 读者无需获取任何锁、执行任何原子指令、写入共享内存，或在（Alpha 以外的）CPU 上执行任何内存屏障。这些操作在现代 CPU 上相当昂贵，这正RCU 在读多场景中具有性能优势的原因。RCU 读者无需获取锁也极大简化了避免死锁的代码
- 如果 RCU 读者在完成后没有任何指示，更新者如何判断宽限期已经完成
  与自旋锁一样，RCU 读者不允许阻塞、切换到用户态执行或进入空闲循环。因此，一旦看到某CPU 经历了这三种状态之一，我们就知道CPU 已经退出了任何先前RCU 读侧临界区。所以，如果我们从链表中移除一项，然后等待所CPU 都进行了上下文切换、在用户态执行或进入了空闲循环，就可以安全地释放该项
  RCU 的可抢占变体（CONFIG_PREEMPT_RCU）达到相同效果，但要求读者操CPU 本地的计数器。这些计数器允许RCU 读侧临界区中进行有限类型的阻塞。SRCU 也使CPU 本地计数器，并允许在 RCU 读侧临界区中进行一般阻塞。这RCU 变体通过采样这些计数器来检测宽限期
- 如果我运行在只能一次做一件事的单处理器（uniprocessor）内核上，为什么还要等待宽限期
  更多信息请参UP.rst
- 如何查看 RCU 当前Linux 内核中的使用位置
  搜索 "rcu_read_lock"rcu_read_unlock"call_rcu"rcu_read_lock_bh"rcu_read_unlock_bh"srcu_read_lock"srcu_read_unlock"synchronize_rcu"synchronize_net"synchronize_srcu" 以及其他 RCU 原语。或者从以下地址获取某个 cscope 数据库：

  (http://www.rdrop.com/users/paulmck/RCU/linuxusage/rculocktab.html)銆。
- 编写使用 RCU 的代码时应遵循哪些准则？

  请参checklist.rst
- 为什么叫 "RCU"
  "RCU" 代表 "read-copy update"（读-复制-更新）。listRCU.rst 中有关于该名称由来的更多信息，搜"read-copy update" 即可找到
- 我听RCU 有专利？这是怎么回事
  是的，它有专利。有多个已知的与 RCU 相关的专利，Documentation/RCU/RTFP.txt 中搜索字符串 "Patent" 即可找到它们。其中一项已被受让人放弃，其余已根据 GPL 贡献Linux 内核。许多（但并非全部）早已过期。现在也LGPL 的实现（用户RCU）可用（https://liburcu.org/）
- 我听RCU 需要进行工作以支持实时（realtime）内核？

  实时友好RCU 通过 CONFIG_PREEMPTION 内核配置参数启用
- 在哪里可以找到关RCU 的更多信息？

  请参Documentation/RCU/RTFP.txt 文件  或将浏览器指(https://docs.google.com/document/d/1X0lThx8OK0ZgLMqVoXiR4ZrGURHrXK6NyLRbeXe3Xac/edit)
  鎴?(https://docs.google.com/document/d/1GCdQC8SDbb54W1shjEXqGZ0Rq8a6kIeYutdSIajfpLA/edit?usp=sharing)銆。