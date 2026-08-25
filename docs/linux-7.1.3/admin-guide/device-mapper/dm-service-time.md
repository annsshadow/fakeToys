## dm-service-time


dm-service-time 是一个用device-mapper 目标的路径选择器（path selector）模块，它为输入I/O 选择估计服务时间最短的路径
每条路径的服务时间通过将路径上在途（in-flight）I/O 的总大小除以该路径的性能值来估计。性能值是路径组内所有路径之间的相对吞吐值，可以作为表参数指定
路径选择器的名称'service-time'
每条路径的表参数
    [<repeat_count> [<relative_throughput>]]
	<repeat_count>:
			在切换到下一条路径之前，使用所选路径分发的 I/O 数量			如果未给出，则使用内部默认值。要查看默认值，请参见已激活的表	<relative_throughput>:
			该路径在路径组内所有路径之间的相对吞吐值			有效范围0-100			如果未给出，则使用最小'1'			如果给定 '0'，则在有其他具有正值路径可用时，该路径不会被选中
每条路径的状态：

    <status> <fail-count> <in-flight-size> <relative_throughput>
	<status>:
		若路径活动则'A'，若路径失败则为 'F'	<fail-count>:
		路径失败的次数	<in-flight-size>:
		该路径上在I/O 的大小	<relative_throughput>:
		该路径在路径组内所有路径之间的相对吞吐值

## 算法


dm-service-time I/O 分发时将其大小加'in-flight-size'，完成时分步减去
基本上，dm-service-time 选择具有最小服务时间的路径
```

	('in-flight-size' + 'size-of-incoming-io') / 'relative_throughput'

```
然而，为了尽可能减少计算，使用了以下一些优化
 1. 如果各路径具有相同的 'relative_throughput'，跳过除法，仅比'in-flight-size'
 2. 如果各路径具有相同的 'in-flight-size'，跳过除法，仅比'relative_throughput'
 3. 如果某些路径具有非零 'relative_throughput' 而其他路径为零，则忽略那'relative_throughput' 为零的路径
如果无法应用这些优化，则计算服务时间并比较服务时间如果计算出的服务时间相等，具有最'relative_throughput' 的路径可能更好。因此接下来比较 'relative_throughput'

## 示例

2 条路径（sda sdb）被使用、repeat_count == 128、且 sda 平均吞吐量为 1GB/s、sdb 4GB/s 的情况下```

  # echo "0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 1 8:16 128 4" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 1 8:16 128 4
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 2 8:0 A 0 0 1 8:16 A 0 0 4


```
```

  # echo "0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 2 8:16 128 8" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 2 8:16 128 8
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 2 8:0 A 0 0 2 8:16 A 0 0 8

```
