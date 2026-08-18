## SoundWire 閿佹満鍒?


鏂囨。 explains 閿佹満鍒?mechanism SoundWire 鎬荤嚎. 鎬荤嚎 uses
following locks order avoid race conditions 鎬荤嚎 鎿嶄綔
shared 璧勬簮.

- 鎬荤嚎 閿?

- Message 閿?

## 鎬荤嚎 閿?


SoundWire 鎬荤嚎 閿?浜掓枼浣?part 鎬荤嚎 鏁版嵁 缁撴瀯浣?
(sdw_bus) 浣跨敤 every 鎬荤嚎 instance. 閿?浣跨敤
serialize following 鎿嶄綔(s) SoundWire 鎬荤嚎 instance.

- Addition removal Slave(s), changing Slave 鐘舵€?

- Prepare, 鍚敤, 绂佺敤 De-prepare stream 鎿嶄綔.

- 璁块棶 Stream 鏁版嵁 缁撴瀯浣?

## Message 閿?


SoundWire message transfer 閿? 浜掓枼浣?part
鎬荤嚎 鏁版嵁 缁撴瀯浣?(sdw_bus). 閿?浣跨敤 serialize message
transfers (璇诲彇/鍐欏叆) SoundWire 鎬荤嚎 instance.

绀轰緥 鏄剧ず locks acquired.

### 绀轰緥 1


Message transfer.

1. every message transfer

. Acquire Message 閿?

b. Transfer message (璇诲彇/鍐欏叆) Slave1 broadcast message
鎬荤嚎 case bank switch.

c. 閲婃斁 Message 閿?

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
### 绀轰緥 2


Prepare 鎿嶄綔.

1. Acquire 閿?鎬荤嚎 instance associated Master 1.

2. every message transfer Prepare 鎿嶄綔

. Acquire Message 閿?

b. Transfer message (璇诲彇/鍐欏叆) Slave1 broadcast message
鎬荤嚎 case bank switch.

c. 閲婃斁 Message 閿?

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