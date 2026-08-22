## dm-delay


Device-Mapper "delay" 目标会延迟读或写
或刷新操作，并可选择性地将它们映射到不同的设备

```
<device> <offset> <delay> [<write_device> <write_offset> <write_delay>
			       [<flush_device> <flush_offset> <flush_delay>]]

```
Table 行必须包3 9 个参数：

3：对设备上的读、写和刷新操作应用偏移和延迟

6：对设备应用偏移和延迟，同时应用 write_offset write_delay
   对可选的不同 write_device 上的写和刷新操作
   使用可选的不同扇区偏移

9：与 6 个参数相同，额外显式定义 flush_offset flush_delay
   位于/使用可选的不同 flush_device/flush_offset

偏移以扇区为单位指定

延迟以毫秒为单位指定


## 示例脚本


```
	#!/bin/sh
	#
	# Create mapped device named "delayed" delaying read, write and flush operations for 500ms.
	#
	dmsetup create delayed --table  "0 `blockdev --getsz $1` delay $1 0 500"

```
```
	#!/bin/sh
	#
	# Create mapped device delaying write and flush operations for 400ms and
	# splitting reads to device $1 but writes and flushes to different device $2
	# to different offsets of 2048 and 4096 sectors respectively.
	#
	dmsetup create delayed --table "0 `blockdev --getsz $1` delay $1 2048 0 $2 4096 400"

```
```
	#!/bin/sh
	#
	# Create mapped device delaying reads for 50ms, writes for 100ms and flushes for 333ms
	# onto the same backing device at offset 0 sectors.
	#
	dmsetup create delayed --table "0 `blockdev --getsz $1` delay $1 0 50 $2 0 100 $1 0 333"

```
