## （取消）补丁回调


Livepatch（取消）补丁回调un)patch-callbacks）提供了一种机制，livepatch 模块
内核对象被（取消）打补丁时执行回调函数。它们可以被视为一*强力特*，它**扩展
livepatching 的能*，使其包含：

  - 对全局数据的安全更

  - init probe 函数补丁"

  - 对其它无法打补丁的代码（即汇编）打补

在大多数情况下，（取消）补丁回调需要与内存屏障和内核同步原语（mutex/spinlock，甚
stop_machine()）配合使用，以避免并发问题

## 1. 动机


回调不同于现有的内核设施

  - 在禁用和重新启用一个补丁时，模init/exit 代码不会运行

  - 模块通知器（notifier）无法阻止一个待打补丁的模块加载

回调klp_object 结构的一部分，其实现特定于该 klp_object。其livepatch 对象可能
被打补丁，也可能没有，与目标 klp_object 的当前状态无关

## 2. 回调类型


可以为以livepatch 动作注册回调

  - Pre-patch（补丁前
                 - klp_object 被打补丁之前

  - Post-patch（补丁后
                 - klp_object 被打补丁且跨所有任务处于活动状态之

  - Pre-unpatch（取消补丁前
                 - klp_object 被取消补丁之前（即打补丁的代码仍然活动）
                   用于清理 post-patch 回调的资

  - Post-unpatch（取消补丁后
                 - klp_object 被取消补丁、所有代码已恢复且没有任何任务在运行
                   打补丁的代码之后，用于清pre-patch 回调的资

## 3. 工作原理


每个回调都是可选的，省略其中一个并不妨碍指定任何其它回调。然而，livepatching 核心
对称方式执行这些处理程序：pre-patch 回调有一post-unpatch 对应项，post-patch 回调
有一pre-unpatch 对应项。只有当其对应的补丁回调被执行过时，才会执行取消补丁回调
典型的使用场景是将一个获取并配置资源的补丁处理程序，与一个拆除并释放相同资源的取
补丁处理程序配对

只有当其所属的 klp_object 被加载时，回调才会执行。对于内核内vmlinux 目标，这意味着
livepatch 被启禁用时，回调总是会执行。对于补丁目标内核模块，只有当目标模块被加载
回调才会执行。当一个模块目标被（取消）加载时，仅当 livepatch 模块已启用时其回调才会执行

pre-patch 回调（如果指定）应当返回一个状态码（成功为 0，错误为 -ERRNO）。错误状态码
livepatching 核心表明当前 klp_object 的补丁操作不安全，并停止当前的补丁请求。（
没有提供 pre-patch 回调时，假定转换为安全。）如果 pre-patch 回调返回失败，内核的模块
加载器将会：

  - 如果 livepatch 在目标代码之后加载，则拒绝加livepatch

    或者：

  - 如果 livepatch 已经成功加载，则拒绝加载模块

对于某个给定klp_object，如果对象打补丁失败（由于失败的 pre_patch 回调或任何其
原因），则不会执行任post-patch、pre-unpatch post-unpatch 回调

如果补丁转换被逆转，则不会运行任何 pre-unpatch 处理程序（这遵循前面提到的对称性—
pre-unpatch 回调只有在其对应post-patch 回调执行过时才会出现）

如果对象确实成功打了补丁，但补丁转换由于某种原因从未开始（例如，如果另一个对象打补丁
失败），则只会调post-unpatch 回调

## 4. 使用场景


演示回调 API 的示livepatch 模块可以samples/livepatch/ 目录中找到。这些示例被
修改为用kselftests，可以在 lib/livepatch 目录中找到

### 全局数据更新


pre-patch 回调可用于更新全局变量。例如，提交 75ff39ccc1bdtcp: make challenge acks
less predictable"）修改了一个全局 sysctl，同时也tcp_send_challenge_ack() 函数打了补丁

在这种情况下，如果我们极度谨慎，也许可以在打补丁完成之后post-patch 回调来修补数据，
这样 tcp_send_challenge_ack() 可以首先被改为使READ_ONCE 读取
sysctl_tcp_challenge_ack_limit銆。

### __init probe 函数补丁支持


尽管 __init probe 函数不能直接livepatch，但有可能通过 pre/post-patch 回调实现
类似的更新

提交 48900cb6af42virtio-net: drop NETIF_F_FRAGLIST"）改变了 virtnet_probe() 初始
其驱net_device 特性的方式。一pre/post-patch 回调可以遍历所有此类设备，对它们的
hw_features 值做类似的修改。（该值的客户端函数可能需要相应更新。）
