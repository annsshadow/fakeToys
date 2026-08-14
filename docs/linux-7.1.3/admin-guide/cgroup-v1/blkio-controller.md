## 块 IO 控制器


## 概述

cgroup 子系统 "blkio" 实现了块 IO 控制器。在存储层级中，无论是叶子
节点还是中间节点，似乎都需要各种类型的 IO 控制策略（例如按比例带宽、
最大带宽）。其计划是对 blkio 控制器使用相同的基于 cgroup 的管理接口，
并根据用户选项在后台切换 IO 策略。

一种 IO 控制策略是节流（throttling）策略，可用于在设备上指定 IO 速率
上限。该策略在通用块层中实现，既可用于叶子节点，也可用于设备映射器
等更高级别的逻辑设备。

## HOWTO


### 节流/上限策略

```

	CONFIG_BLK_CGROUP=y

```
```

	CONFIG_BLK_DEV_THROTTLING=y

```
```

        mount -t cgroup -o blkio none /sys/fs/cgroup/blkio

```
为 root 组在指定设备上指定带宽速率。其格式
```

        echo "8:16  1048576" > /sys/fs/cgroup/blkio/blkio.throttle.read_bps_device

```
这将对设备主/次设备号为 8:16 上发生的读操作施加 1MB/秒的限制。

```

        # dd iflag=direct if=/mnt/common/zerofile of=/dev/null bs=4K count=1024
        1024+0 records in
        1024+0 records out
        4194304 bytes (4.2 MB) copied, 4.0001 s, 1.0 MB/s

```
写入的限制可通过 blkio.throttle.write_bps_device 文件设置。

## 层级式 cgroups


节流实现了层级支持；不过，节流的层级支持仅在 cgroup 侧启用了
"sane_behavior" 时才生效，而该选项目前仍是开发选项，尚未公开提供。

```

			root
			/  \
		     test1 test2
			|
		     test3

```
启用了 "sane_behavior" 的节流能正确处理层级关系。对于节流，所有限制
都作用于整个子树，而所有统计信息仅针对该 cgroup 内任务直接生成的 IO
的本地数据。

未启用 cgroup 侧 "sane_behavior" 的节流，实际上会把所有组视为同一
层级，就好像看起来像下面的
```

				pivot
			     /  /   \  \
			root  test1 test2  test3

```
## 各种用户可见的配置选项


  CONFIG_BLK_CGROUP
	  块 IO 控制器。

  CONFIG_BFQ_CGROUP_DEBUG
	  调试辅助。若启用此选项，cgroup 中会出现一些额外的统计文件。

  CONFIG_BLK_DEV_THROTTLING
	  在块层启用块设备节流支持。

## cgroup 文件详情


### 按比例权重策略文件


  blkio.bfq.weight
	  指定每个 cgroup 的权重。这是该组在所有设备上的默认权重，
	  除非被每设备规则覆盖（见下面的 `blkio.bfq.weight_device`）。

	  当前允许的权重范围是 1 到 1000。更多细节，参见
	  Documentation/block/bfq-iosched.rst。

  blkio.bfq.weight_device
	  指定每个 cgroup 每设备的权重，覆盖默认组权重。更多细节，
	  参见 Documentation/block/bfq-iosched.rst。

```

	    # echo dev_maj:dev_minor weight > blkio.bfq.weight_device

	  Configure weight=300 on /dev/sdb (8:16) in this cgroup::

	    # echo 8:16 300 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:16    300

	  Configure weight=500 on /dev/sda (8:0) in this cgroup::

	    # echo 8:0 500 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:0     500
	    8:16    300

	  Remove specific weight for /dev/sda in this cgroup::

	    # echo 8:0 0 > blkio.bfq.weight_device
	    # cat blkio.bfq.weight_device
	    dev     weight
	    8:16    300

  blkio.time
	  按设备分配给 cgroup 的磁盘时间，单位为毫秒。前两个字段指定
	  设备的主设备号和次设备号，第三个字段指定分配给该组的磁盘
	  时间（毫秒）。

  blkio.sectors
	  该组在磁盘上传输（读或写）的扇区数。前两个字段指定设备的
	  主设备号和次设备号，第三个字段指定该组在设备上传输的
	  扇区数。

  blkio.io_service_bytes
	  该组在磁盘上传输的字节数。这些字节数进一步按操作类型
	  （读或写、同步或异步）划分。前两个字段指定设备的主设备号
	  和次设备号，第三个字段指定操作类型，第四个字段指定
	  字节数。

  blkio.io_serviced
	  该组向磁盘发出的 IO 数（bio）。这些 IO 数进一步按操作类型
	  （读或写、同步或异步）划分。前两个字段指定设备的主设备号
	  和次设备号，第三个字段指定操作类型，第四个字段指定 IO 数。

  blkio.io_service_time
	  该 cgroup 所发出 IO 从请求分发到请求完成的总耗时。采用纳秒
	  单位以便对闪存设备也有意义。对于队列深度为 1 的设备，该时间
	  代表实际服务时间。当 queue_depth > 1 时，这不再成立，因为请求
	  可能乱序得到服务。这可能导致某个给定 IO 的服务时间包含了
	  多个 IO 的服务时间（当乱序服务时），从而可能造成总
	  io_service_time 大于实际经过的时间。该时间进一步按操作类型
	  （读或写、同步或异步）划分。前两个字段指定设备的主设备号
	  和次设备号，第三个字段指定操作类型，第四个字段指定
	  io_service_time（单位为 ns）。

  blkio.io_wait_time
	  该 cgroup 的 IO 在调度器队列中等待服务的总时间。由于它是
	  所有 IO 累积的 io_wait_time，因此可能大于总经过时间。它不是
	  衡量 cgroup 等待总时间的指标，而是衡量其各个 IO 等待时间的
	  指标。对于队列深度 > 1 的设备，该指标不包含 IO 被分发到
	  设备后直到实际得到服务所等待的时间（由于设备对请求的重排序，
	  这里可能存在时间差）。采用纳秒单位以便对闪存设备也有意义。
	  该时间进一步按操作类型（读或写、同步或异步）划分。前两个
	  字段指定设备的主设备号和次设备号，第三个字段指定操作类型，
	  第四个字段指定 io_wait_time（单位为 ns）。

  blkio.io_merged
	  合并到属于该 cgroup 的请求中的 bios/请求总数。进一步按
	  操作类型（读或写、同步或异步）划分。

  blkio.io_queued
	  任意时刻为该 cgroup 排队的总请求数。进一步按操作类型
	  （读或写、同步或异步）划分。

  blkio.avg_queue_size
	  仅在 CONFIG_BFQ_CGROUP_DEBUG=y 时启用的调试辅助。
	  该 cgroup 在其整个存在期间的平均队列大小。每当该 cgroup 的
	  某个队列获得一个时间片时采集队列大小样本。

  blkio.group_wait_time
	  仅在 CONFIG_BFQ_CGROUP_DEBUG=y 时启用的调试辅助。
	  这是 cgroup 自变为繁忙（即从 0 个请求排队变为 1 个请求排队）
	  起，到为其某个队列获得时间片所等待的时间。这与 io_wait_time
	  不同，后者是该 cgroup 中每个 IO 在调度器队列中等待的累积
	  总时间。采用纳秒单位。如果在 cgroup 处于等待（等待时间片）
	  状态时读取该统计，则只会报告截至上次获得时间片为止累积的
	  group_wait_time，不包含当前的增量。

  blkio.empty_time
	  仅在 CONFIG_BFQ_CGROUP_DEBUG=y 时启用的调试辅助。
	  这是 cgroup 在没有待处理请求且未被服务时（即不包含为 cgroup
	  某个队列空闲等待的任何时间）所花费的时间。采用纳秒单位。
	  如果在 cgroup 处于空状态时读取该统计，则只会报告截至上次有
	  待处理请求为止累积的 empty_time，不包含当前的增量。

  blkio.idle_time
	  仅在 CONFIG_BFQ_CGROUP_DEBUG=y 时启用的调试辅助。
	  这是 IO 调度器为了期待来自其他队列/cgroup 的更好请求而
	  为给定 cgroup 空闲等待的时间。采用纳秒单位。如果在 cgroup
	  处于空闲状态时读取该统计，则只会报告截至上个空闲周期为止
	  累积的 idle_time，不包含当前的增量。

  blkio.dequeue
	  仅在 CONFIG_BFQ_CGROUP_DEBUG=y 时启用的调试辅助。它提供
	  关于一个组从设备的服务树中出队次数的统计。前两个字段指定
	  设备的主设备号和次设备号，第三个字段指定一个组从特定设备
	  出队的次数。

  blkio.*_recursive
	  各种统计的递归版本。这些文件显示的信息与其非递归对应项
	  相同，但包含来自所有后代 cgroup 的统计。

```
### 节流/上限策略文件

  blkio.throttle.read_bps_device
	  指定从设备读取速率的上限。IO 速率以字节/秒为单位指定。规则
	  按设备划分。格式如下
```

	    echo "<major>:<minor>  <rate_bytes_per_second>" > /cgrp/blkio.throttle.read_bps_device

  blkio.throttle.write_bps_device
	  指定向设备写入速率的上限。IO 速率以字节/秒为单位指定。规则
	  按设备划分。格式如下::

	    echo "<major>:<minor>  <rate_bytes_per_second>" > /cgrp/blkio.throttle.write_bps_device

  blkio.throttle.read_iops_device
	  指定从设备读取速率的上限。IO 速率以 IO/秒为单位指定。规则
	  按设备划分。格式如下::

	   echo "<major>:<minor>  <rate_io_per_second>" > /cgrp/blkio.throttle.read_iops_device

  blkio.throttle.write_iops_device
	  指定向设备写入速率的上限。IO 速率以 IO/秒为单位指定。规则
	  按设备划分。格式如下::

	    echo "<major>:<minor>  <rate_io_per_second>" > /cgrp/blkio.throttle.write_iops_device

          注意：如果为某个设备同时指定了 BW 和 IOPS 规则，则该 IO 受
          这两种约束的限制。

  blkio.throttle.io_serviced
	  该组向磁盘发出的 IO 数（bio）。这些 IO 数进一步按操作类型
	  （读或写、同步或异步）划分。前两个字段指定设备的主设备号
	  和次设备号，第三个字段指定操作类型，第四个字段指定 IO 数。

  blkio.throttle.io_service_bytes
	  该组在磁盘上传输的字节数。这些字节数进一步按操作类型
	  （读或写、同步或异步）划分。前两个字段指定设备的主设备号
	  和次设备号，第三个字段指定操作类型，第四个字段指定字节数。

```
### 各策略之间通用的文件

  blkio.reset_stats
	  向该文件写入一个 int 将重置该 cgroup 的所有统计信息。

