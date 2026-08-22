## 切换调度

每个 IO 队列都有一组与之关联的 IO 调度器可调参数。这些可调参数控IO 调度器的工作方式。假设您已将 sysfs 挂载/sys，您可以在以下位找到这些条目
```
	/sys/block/<device>/queue/iosched
```

如果您没有挂sysfs
```
	# mount none /sys -t sysfs
```

可以实时更改给定块设备的 IO 调度器，以选择 mq-deadline、none、bfq kyber 调度器之一——这可以提高该设备的吞吐量
```
	echo SCHEDNAME > /sys/block/DEV/queue/scheduler
```

其中 SCHEDNAME 是已定义 IO 调度器的名称，DEV 是设备名（hda、hdb、sga
或您拥有的任何设备）
只需执行 "cat /sys/block/DEV/queue/scheduler" 即可找到已定义调度器列表——有效名称列表如下：

```
  # cat /sys/block/sda/queue/scheduler
  [mq-deadline] kyber bfq none
  # echo none >/sys/block/sda/queue/scheduler
  # cat /sys/block/sda/queue/scheduler
  [none] mq-deadline kyber bfq
```
