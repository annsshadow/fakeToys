
## padata 并行执行机制


:Date: May 2020

Padata 是一种机制，内核可以借助它将任务分派到多CPU 上并行执行，同时（可选地）保持它们的顺序
它最初是IPsec 开发的，IPsec 需要对大量数据包执行加密和解密，而不对这些数据包重新排序。目前这padata 序列化任务支持的唯一使用者
Padata 还支持多线程任务，在负载均衡和各线程之间协调的同时，将任务均匀拆分
## 运行序列化任

### 初始

使用 padata 运行序列化任务的第一步是建立一
```

    #include <linux/padata.h>

    struct padata_instance *padata_alloc(const char *name);

```
'name' 只是用于标识该实例
```

   struct padata_shell *padata_alloc_shell(struct padata_instance *pinst);

```
padata_shell 用于padata 提交一个任务，并允许一系列这样的任务被独立地序列化。一padata_instance 可以关联一个或多个 padata_shell，每个都允许一系列独立的任务
### 修改 cpumask


用于运行任务CPU 可以通过两种方式更改，一种是通过编程方式
```

    int padata_set_cpumask(struct padata_instance *pinst, int cpumask_type,
			   cpumask_var_t cpumask);

```
这里 cpumask_type PADATA_CPU_PARALLEL PADATA_CPU_SERIAL 之一，其parallel cpumask 描述将用于并行执行提交到该实例的任务的处理器，serial cpumask 定义允许用作序列化回调处理器的处理器。cpumask 指定要使用的cpumask
一个实例的 cpumask 可能有对应的 sysfs 文件。例如，pcrypt 的文件位/sys/kernel/pcrypt/<instance-name>。在一个实例的目录中有两个文件，parallel_cpumask serial_cpumask，任一 cpumask

```

    echo f > /sys/kernel/pcrypt/pencrypt/parallel_cpumask

```
读取这些文件之一会显示用户提供的 cpumask，它可能可用"cpumask 不同
padata 在内部维护两cpumask，即用户提供cpumask 可用"cpumask。（每对都由一parallel 和一serial cpumask 组成。）用户提供cpumask 在实例分配时默认为所有可能的 CPU，并可以如上所述更改。可用的 cpumask 始终是用户提供的 cpumask 的子集，并且只包含用户提供的掩码中在线的 CPU；这些才padata 实际使用cpumask。因此，padata 提供一个包含离CPU cpumask 是合法的。一旦用户提供的 cpumask 中的某个离线 CPU 上线，padata 就会使用它
更改 CPU 掩码是代价高昂的操作，因此不应过于频繁地进行
### 运行一个任

实际padata 实例提交工作，需要创
```

    struct padata_priv {
        /* Other stuff here... */
	void                    (*parallel)(struct padata_priv *padata);
	void                    (*serial)(struct padata_priv *padata);
    };

```
该结构体几乎肯定会被嵌入到某个特定于待完成工作的更大结构体中。它的多数字段对 padata 是私有的，但该结构体应在初始化时被清零，并且应提parallel() serial() 函数。这些函数将在完成工作的过程中被调用，我们稍后就会看到
```

    int padata_do_parallel(struct padata_shell *ps,
		           struct padata_priv *padata, int *cb_cpu);

```
ps padata 结构体必须按上述方式设置；cb_cpu 指向任务完成时用于最终回调的首CPU；它必须位于当前实例CPU 掩码中（否则 cb_cpu 指针会被更新为指向实际被选中CPU）。padata_do_parallel() 的返回值为 0 表示成功，表明任务正在进行中EBUSY 意味着其他地方的某个人正在干扰该实例的 CPU 掩码，-EINVAL 表示抱cb_cpu 不在 serial cpumask 中、parallel serial cpumask 中没有在CPU，或者实例已停止
提交padata_do_parallel() 的每个任务，将依次被传递给恰好一次上parallel() 函数的调用，在一CPU 上，因此真正的并行性是通过提交多个任务来实现的。parallel() 在软件中断被禁用的情况下运行，因此不能睡眠。parallel() 函数padata_priv 结构体指针作为其唯一参数；关于实际要完成的工作的信息，可能是通过使用 container_of() 找到外层结构体来获得的
注意 parallel() 没有返回值；padata 子系统假parallel() 从这一点起将负责该任务。该任务不必在这次调用期间完成，但如parallel() 留下了未完成的工作，它应该做好准备，在前一个任务完成之前被再次调用以处理一个新任务
### 序列化任

当一个任务确实完成时，parallel()（或任何实际完成该任务的函数
```

    void padata_do_serial(struct padata_priv *padata);

```
在未来的某个时刻，padata_do_serial() 将触发对 padata_priv 结构体中 serial() 函数的调用。该调用将发生在最初调padata_do_parallel() 时所请求CPU 上；它同样在本地软件中断被禁用的情况下运行。注意这个调用可能会被推迟一段时间，因为 padata 代码会不遗余力地确保任务按提交的顺序完成
### 销

清理一padata 实例，顺理成章地涉及调用两个 free

```

    void padata_free_shell(struct padata_shell *ps);
    void padata_free(struct padata_instance *pinst);

```
用户有责任确保在调用上述任何函数之前，所有未完成的任务都已结束
## 运行多线程任

一个多线程任务有一个主线程和零个或多个辅助线程，主线程参与该任务，然后等待所有辅助线程完成。padata 将任务拆分为称为 chunk 的单元，其中 chunk 是一个线程在一次对线程函数的调用中所完成的一部分任务
用户要运行一个多线程任务需要做到三件事。首先，通过定义一padata_mt_job 结构体来描述该任务，这将在接口一节中解释。这包括一个指向线程函数的指针，padata 每次将一个任chunk 分配给一个线程时都会调用该函数。然后，定义线程函数，它接受三个参数，`start`、`end` `arg`，其中前两个限定线程所操作的范围，最后一个是（如果有的话）指向任务共享状态的指针。准备共享状态，它通常在主线程的栈上分配。最后，调用 padata_do_multithreaded()，它会在任务完成时返回
## 接口
