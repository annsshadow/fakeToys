## RDMA 鎺у埗鍣。


   1. 概述
     1-1. 什么是 RDMA 控制器？
     1-2. 为何需RDMA 控制器？
     1-3. RDMA 控制器是如何实现的？
   2. 使用示例

## 1. 概述


### 1-1. 什么是 RDMA 控制器？


RDMA 控制器允许用户限制一组给定进程可以使用的 RDMA/IB 特定资源。这些进程通过 RDMA 控制器分组
RDMA 控制器定义了两类可以针对某个 cgroup 的进程进行限制的资源
### 1-2. 为何需RDMA 控制器？


目前用户空间应用程序很容易耗尽所rdma verb 特定资源，例AH、CQ、QP、MR 等。这导致其它 cgroup 中的应用程序或内核空间的 ULP 可能连分配任rdma 资源的机会都没有。这会引发服务不可用
因此需要一RDMA 控制器来限制进程的资源消耗。通过该控制器可以对不同的 rdma 资源进行记账
### 1-3. RDMA 控制器是如何实现的？


RDMA cgroup 允许配置资源限制。Rdma cgroup 通过资源池（resource pool）结构对每个 cgroup、每个设备进行资源记账rdma cgroup 将每个资源池限制为最64 个资源，以后如有需要可扩展
该资源池对象链接cgroup css。在大多数使用场景中，每cgroup、每个设备通常0 4 个资源池实例。但并没有限制其更多。目前单cgroup 下数百个 RDMA 设备可能无法被最优处理，但也没有已知的使用场景或需求需要如此配置
由于 RDMA 资源可从任意进程分配，并可由共享地址空间的任意子进程释放，rdma 资源始终由创建者的 cgroup css 拥有。这使得进程在不cgroup 间迁移时，无需承担转移资源所有权的复杂性；因为由于 rdma 资源共享的本质，这种所有权实际上并不存在。围css 链接资源也确保进程迁移后 cgroup 可以被删除。这也允许携带活动资源进行进程迁移，尽管这不是主要使用场景
每当发生 RDMA 资源记账（charging）时，会将拥有rdma cgroup 返回给调用者。在解除记账（uncharging）该资源时，应传入同一rdma cgroup。这也允许携带活RDMA 资源迁移的进程向新拥有cgroup 记帐新资源。它还允许将已迁移至cgroup 的进程的资源从先前记帐的 cgroup 解除记帐，尽管这不是主要使用场景
资源池对象在以下情形下被创建(a) 用户设置了限制，且目cgroup 对应设备此前不存在资源池(b) 未配置任何资源限制，IB/RDMA 栈尝试对该资源记帐。这样它才能正确解除记帐；否则当应用程序在无限制情况下运行，而后续在解除记帐时实施限制，使用计数会降为负数
当所有资源限制都被设max，且该资源是最后一个被释放的资源时，资源池被销毁
如果用户意图移除/取消配置某个特定设备的资源池，应把所有限制设max 值
IB 栈会遵守 rdma 控制器实施的限制。当应用程序查询 IB 设备的最大资源限制时，它返回用户为给cgroup 所配置的值与 IB 设备所支持的值两者中的较小者
RDMA 控制器可对以下资源记账
  ==========    =============================
  hca_handle	最HCA 句柄数量
  hca_object 	最HCA 对象数量
  ==========    =============================

## 2. 使用示例


```

	echo mlx4_0 hca_handle=2 hca_object=2000 > /sys/fs/cgroup/rdma/1/rdma.max
	echo ocrdma1 hca_handle=3 > /sys/fs/cgroup/rdma/2/rdma.max

```
```

	cat /sys/fs/cgroup/rdma/2/rdma.max
	#Output:
	mlx4_0 hca_handle=2 hca_object=2000
	ocrdma1 hca_handle=3 hca_object=max

```
```

	cat /sys/fs/cgroup/rdma/2/rdma.current
	#Output:
	mlx4_0 hca_handle=1 hca_object=20
	ocrdma1 hca_handle=1 hca_object=23

```
```

	echo mlx4_0 hca_handle=max hca_object=max > /sys/fs/cgroup/rdma/1/rdma.max

```
