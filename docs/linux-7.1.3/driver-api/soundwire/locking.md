## SoundWire 锁机制


文档 explains 锁机制 mechanism SoundWire 总线. 总线 uses
following locks order avoid race conditions 总线 操作
shared 资源.

- 总线 锁

- Message 锁

## 总线 锁


SoundWire 总线 锁 互斥体 part 总线 数据 结构体
(sdw_bus) 使用 every 总线 instance. 锁 使用
serialize following 操作(s) SoundWire 总线 instance.

- Addition removal Slave(s), changing Slave 状态.

- Prepare, 启用, 禁用 De-prepare stream 操作.

- 访问 Stream 数据 结构体.

## Message 锁


SoundWire message transfer 锁. 互斥体 part
总线 数据 结构体 (sdw_bus). 锁 使用 serialize message
transfers (读取/写入) SoundWire 总线 instance.

示例 显示 locks acquired.

### 示例 1


Message transfer.

1. every message transfer

. Acquire Message 锁.

b. Transfer message (读取/写入) Slave1 broadcast message
总线 case bank switch.

c. 释放 Message 锁

```

	+----------+                    +---------+
	|          |                    |         |
	|   Bus    |                    | Master  |
	|          |                    | Driver  |
	|          |                    |         |
	+----+-----+                    +----+----+
	     |                               |
	     |     bus->ops->xfer_msg()      |
	     <-------------------------------+   a. Acquire Message lock
	     |                               |   b. Transfer message
	     |                               |
	     +------------------------------->   c. Release Message lock
	     |    return success/error       |   d. Return success/error
	     |                               |
	     +                               +

```
### 示例 2


Prepare 操作.

1. Acquire 锁 总线 instance associated Master 1.

2. every message transfer Prepare 操作

. Acquire Message 锁.

b. Transfer message (读取/写入) Slave1 broadcast message
总线 case bank switch.

c. 释放 Message 锁.

```

	+----------+                    +---------+
	|          |                    |         |
	|   Bus    |                    | Master  |
	|          |                    | Driver  |
	|          |                    |         |
	+----+-----+                    +----+----+
	     |                               |
	     |    sdw_prepare_stream()       |
	     <-------------------------------+   1. Acquire bus lock
	     |                               |   2. Perform stream prepare
	     |                               |
	     |                               |
	     |     bus->ops->xfer_msg()      |
	     <-------------------------------+   a. Acquire Message lock
	     |                               |   b. Transfer message
	     |                               |
	     +------------------------------->   c. Release Message lock
	     |    return success/error       |   d. Return success/error
	     |                               |
	     |                               |
	     |    return success/error       |   3. Release bus lock
	     +------------------------------->   4. Return success/error
	     |                               |
	     +                               +

```