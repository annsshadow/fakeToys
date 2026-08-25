## device-mapper uevent（设备映射器 uevent

device-mapper uevent 代码device-mapper 增加了创建并发kobject uevent（uevent）的能力。此device-mapper 事件仅通过 ioctl 接口可用。uevent 接口的优势在于，事件包含环境属性，为事件提供了更多上下文，从而无需在收到事件后再去查询 device-mapper 设备的状态
目前 device-mapper 事件有两个函数。第一个函```

  void dm_path_uevent(enum dm_uevent_type event_type, struct dm_target *ti,
                      const char *path, unsigned nr_valid_paths)

  void dm_send_uevents(struct list_head *events, struct kobject *kobj)


```
添加uevent 环境的变量有
### 变量名：DM_TARGET

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description:
:Value: 产生该事件的 device-mapper 目标的名称
### 变量名：DM_ACTION

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description:
:Value: 导致uevent 动作device-mapper 特定动作	PATH_FAILED - 一条路径已失败	PATH_REINSTATED - 一条路径已被恢复
### 变量名：DM_SEQNUM

:Uevent Action(s): KOBJ_CHANGE
:Type: unsigned integer
:Description: 该特device-mapper 设备的序列号:Value: 有效的无符号整数范围
### 变量名：DM_PATH

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: 与本次事件相关的路径设备的主设备号和次设备号:Value: 形如 "Major:Minor" 的路径名
### 变量名：DM_NR_VALID_PATHS

:Uevent Action(s): KOBJ_CHANGE
:Type: unsigned integer
:Description:
:Value: 有效的无符号整数范围
### 变量名：DM_NAME

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: device-mapper 设备的名称:Value: 名称

### 变量名：DM_UUID

:Uevent Action(s): KOBJ_CHANGE
:Type: string
:Description: device-mapper 设备UUID:Value: UUID。（如果没有则为空字符串。）

下面是由 udevmonitor 捕获的所生成 uevent 的示
```

	UEVENT[1192521009.711215] change@/block/dm-3
	ACTION=change
	DEVPATH=/block/dm-3
	SUBSYSTEM=block
	DM_TARGET=multipath
	DM_ACTION=PATH_FAILED
	DM_SEQNUM=1
	DM_PATH=8:32
	DM_NR_VALID_PATHS=0
	DM_NAME=mpath2
	DM_UUID=mpath-35333333000002328
	MINOR=3
	MAJOR=253
	SEQNUM=1130

```
```

	UEVENT[1192521132.989927] change@/block/dm-3
	ACTION=change
	DEVPATH=/block/dm-3
	SUBSYSTEM=block
	DM_TARGET=multipath
	DM_ACTION=PATH_REINSTATED
	DM_SEQNUM=2
	DM_PATH=8:32
	DM_NR_VALID_PATHS=1
	DM_NAME=mpath2
	DM_UUID=mpath-35333333000002328
	MINOR=3
	MAJOR=253
	SEQNUM=1131

```
