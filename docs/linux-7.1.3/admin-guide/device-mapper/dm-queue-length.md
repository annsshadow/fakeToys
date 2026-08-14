## dm-queue-length


dm-queue-length 是 device-mapper 目标的路径选择器模块，它选择具有最少在途
I/O 的路径。路径选择器名称为 'queue-length'。

每条路径的表参数：[<repeat_count>]

```

	<repeat_count>：在使用所选路径分派 I/O 的数量，之后切换到下一条路径。
			若未给定，使用内部默认值。要查看默认值，请参阅已激活的表。

```
每条路径的状态：<status> <fail-count> <in-flight>

```

	<status>：路径处于活动状态为 'A'，路径失败为 'F'。
	<fail-count>：路径失败的次数。
	<in-flight>：路径上在途 I/O 的数量。


```
## 算法


dm-queue-length 在分派/完成 I/O 时分别递增/递减 'in-flight'。
dm-queue-length 选择具有最小 'in-flight' 的路径。


## 示例


在 2 条路径（sda 与 sdb）且 repeat_count == 128 的情况下使用。

```

  # echo "0 10 multipath 0 0 1 1 queue-length 0 2 1 8:0 128 8:16 128" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 queue-length 0 2 1 8:0 128 8:16 128
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 1 8:0 A 0 0 8:16 A 0 0

```
