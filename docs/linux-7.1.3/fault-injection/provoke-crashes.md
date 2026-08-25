
## 使用 Linux 内核转储测试模块（LKDTM）触发崩

lkdtm 模块提供了一个接口，用于在预定义的代码位置中断（通常导致崩溃）内核，以评估内核异常处理（exception handling）的可靠性，并测试使用不同转储方案获得的崩溃转储（crash dump）。该模块使用 KPROBE 来插桩（instrument）触发位置，但也可以通过 debugfs 在没KPROBE 支持的情况下直接触发内核
你可以选择触发的位置（“崩溃点名称”，crash point name）和动作类型（“崩溃点类型”，crash point type），既可以通过插入模块时的模块参数，也可以通过 debugfs 接口
```

	insmod lkdtm.ko [recur_count={>0}] cpoint_name=<> cpoint_type=<>
			[cpoint_count={>0}]

```
recur_count
	栈溢出测试的递归层级。默认情况下根据内核配置动态计算，目标是刚好大到足以耗尽内核栈。该值可`/sys/module/lkdtm/parameters/recur_count` 查看
cpoint_name
	在内核中的何处触发动作。可以是 INT_HARDWARE_ENTRY、INT_HW_IRQ_EN、INT_TASKLET_ENTRY、FS_SUBMIT_BH、MEM_SWAPOUT、TIMERADD、SCSI_QUEUE_RQ DIRECT 之一
cpoint_type
	指示命中崩溃点时要采取的动作。种类很多，最好直接从 debugfs 查询。一些常见的PANIC、BUG、EXCEPTION、LOOP OVERFLOW。完整列表参`/sys/kernel/debug/provoke-crash/DIRECT` 的内容
cpoint_count
	指示在触发动作之前崩溃点需要被命中的次数。默认是 10（DIRECT 除外，它总是立即触发）
你也可以通过挂载 debugfs 并写入类型来引发故障
```

  mount -t debugfs debugfs /sys/kernel/debug
  echo EXCEPTION > /sys/kernel/debug/provoke-crash/INT_HARDWARE_ENTRY

```
特殊文件 `DIRECT` 会在没有 KPROBE 插桩的情况下直接引发动作。当模块以如下方式构建时，这是唯一可用的模```

  # 与其BUG 杀掉你shell，不如让它杀“cat”：
  cat <(echo WRITE_RO) >/sys/kernel/debug/provoke-crash/DIRECT

```
