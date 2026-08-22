## 设备白名单控制器


## 1. 描述

实现一cgroup，用于跟踪并强制对设备文件施open mknod 限制。一个设cgroup
将设备访问白名单与每cgroup 关联。白名单项有 4 个字段type' a（all，全部）c（char，字符设备）b（block，块设备）all' 表示它适用于所有类型以及所有主设备和次设备号。主设备号和次设备号可以是整数，也可以是表示全部*。访问权限（Accessr（read，读）、w（write，写）和 m（mknod）组合而成
根设cgroup 起始拥有'all' rwm 权限。子设备 cgroup 会获得其父级的副本。然管理员可以从白名单中移除设备，或添加新的项。子 cgroup 永远不能获得被其父级拒绝设备访问权限
## 2. 用户界面

使用 devices.allow 添加一项，使用
```

	echo 'c 1:3 mr' > /sys/fs/cgroup/1/devices.allow

```
允许 cgroup 1 读取并对通常被称为以下名称的设备执行 mknod
```

	echo a > /sys/fs/cgroup/1/devices.deny

```
```

	echo a > /sys/fs/cgroup/1/devices.allow

```
会向白名单添'a **:** rwm' 项
## 3. 瀹夊叏鎬。
任何任务都可以在 cgroup 之间移动自身。这显然不够，但我们可以在人们积累一些使经验后，决定最佳的、足以充分限制移动的方式。我们也许只想要CAP_SYS_ADMIN它至少是CAP_MKNOD 不同的一个位。我们可能想拒绝移动到非当前 cgroup 后代cgroup。或者我们可能想使用 CAP_MAC_ADMIN，因为我们确实是在试图锁root
修改白名单或将另一个任务移动到cgroup 需CAP_SYS_ADMIN。（同样，我们可会想改变这一点）
一cgroup 获得的权限不能超过其 cgroup 父级拥有的权限
## 4. 层级结构

设备 cgroup 通过确保一cgroup 永远不具有比其父级更多的访问权限来维护层级结构每次向某cgroup devices.deny 文件写入一项时，其所有子级都会从白名单中移除该项并且所有本地设置的白名单项都会被重新评估。如果某个本地设置的白名单项会提供比cgroup 父级更多的访问权限，它会被从白名单中移除
```

      A
     / \
        B

    group        behavior	exceptions
    A            allow		"b 8:* rwm", "c 116:1 rw"
    B            deny		"c 1:3 rwm", "c 116:2 rwm", "b 3:* rwm"

```
```

	# echo "c 116:* r" > A/devices.deny

```
它会向下传播，在重新验证 B 的项之后，白名单项变```

    group        whitelist entries                        denied devices
    A            all                                      "b 8:* rwm", "c 116:* rw"
    B            "c 1:3 rwm", "b 3:* rwm"                 all the rest

```
如果父级的例外发生变化，且本地例外不再被允许，它们将被删除
```

      A
     / \
        B

    group        whitelist entries                        denied devices
    A            "c 1:3 rwm", "c 1:5 r"                   all the rest
    B            "c 1:3 rwm", "c 1:5 r"                   all the rest

```
```

	# echo "c *:3 rwm" >A/devices.allow

```
```

    group        whitelist entries                        denied devices
    A            "c *:3 rwm", "c 1:5 r"                   all the rest
    B            "c 1:3 rwm", "c 1:5 r"                   all the rest

```
```

	# echo "c 2:3 rwm" >B/devices.allow
	# echo "c 50:3 r" >B/devices.allow

```
```

	# echo "c *:3 rwm" >B/devices.allow

```
一旦设cgroup 拥有子级，就不能再通过devices.allow devices.deny 写入 'a'
来允许或拒绝全部
### 4.1 层级结构（内部实现）

设备 cgroup 在内部使用一个行为（ALLOW、DENY）和一个例外列表来实现。内部状态使相同的用户接口来控制，以保持与之前仅白名单实现的兼容性。会减少设备访问权限的例外的
移除或添加，会沿层级结构向下传播。对于每一个被传播的例外，有效规则会基于当前父级的
访问规则被重新评估