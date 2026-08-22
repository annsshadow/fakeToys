## dm-switch


device-mapper switch 目标创建一个设备，它支持在固定的一组路径上对固定大小的
I/O 区域进行任意映射。用于任何特定区域的路径可以通过向该目标发送一条消息来动切换
当存在大量固定大小的地址区域，但又不存在能够用紧凑形式（dm-stripe）表示映的简单模式时，它I/O 高效地映射到下层块设备
### 背景


Dell EqualLogic 以及一些其iSCSI 存储阵列使用一种分布式的、无框架（frameless架构。在这种架构中，存储组由若干独立的存储阵列（"members"，成员）组成，每成员拥有独立的控制器、磁盘存储和网络适配器。当创建一LUN 时，它被分布到多成员上。分布的细节对连接到该存储系统的发起端（initiator）是隐藏的。无论使用了
多少成员，存储组都只暴露单一的发现门户（target discovery portal）。当创建 iSCSI
会话时，每个会话都连接到某个成员上的 eth 端口。发往 LUN 的数据可以通过任何 iSCSI
会话发送，如果所访问的块存储在另一个成员上，I/O 会按需被转发。这种转发对发起是不可见的。存储布局也是动态的，磁盘上存储的块可能会根据需要在成员之间移动以平衡负载
这种架构简化了存储组和发起端的管理与配置。在多路径（multipathing）配置中，可建立多个 iSCSI 会话来利用主机和目标上的多个网络接口，以利用增加的网络带宽。发起端
可以使用简单的轮询（round robin）算法将所I/O 分散到所有路径上，并让存储阵成员按需转发，但将数据直接发送到正确的成员在性能上更有优势
device-mapper 表已经允许你将设备的不同区域映射到不同的目标。然而在此架构中，LUN
以大约数MB 量级的地址区域大小散布，这意味着生成的表可能有超过一百万个条目，
消耗过多的内存
使用这个 device-mapper switch 目标，我们现在可以构建一个两层设备层次结构：

    Upper Tier（上层） - 确定 I/O 应被发送到哪个阵列成员    Lower Tier（下层） - 在通向特定成员的路径之间做负载均衡
下层由每个成员一dm multipath 设备组成。每个这样的 multipath 设备在一个优先级
组中包含直接通向该阵列成员的一组路径，并利用现有的路径选择器在这些路径之间做负均衡。我们还构建了一个非首选优先级组，其中包含通往其它阵列成员的路径，用于故障
切换
上层由单dm-switch 设备组成。该设备使用位图查找 I/O 的位置，并选择适当的下设备来路I/O。通过使用位图，我们能够在 16 成员组中对每个地址范围使用 4 位（这对
我们来说已经很大了）。这dm 表的 b-tree 能达到的表示要密集得多
## 构造参

    <num_paths> <region_size> <num_optional_args> [<optional_args>...] [<dev_path> <offset>]+

	<num_paths>
	    用于分布 I/O 的路径数量
	<region_size>
	    一个区域中512 字节扇区数。每个区域可以被重定向到任何可用的路径
	<num_optional_args>
	    可选参数的数量。目前不支持任何可选参数，因此此值必须为零
	<dev_path>
	    代表指向该设备的特定路径的块设备
	<offset>
	    特定 <dev_path> 上数据起始位置的偏移（以 512 字节扇区为单位）。在将请	    转发到特定路径时，该数字被加到扇区号上。通常为零
## 消息


set_region_mappings <index>:<path_nr> [<index>]:<path_nr> [<index>]:<path_nr>...

通过指定哪些区域被重定向到哪些路径来修改区域表
<index>
    区域编号（区域大小在构造参数中指定）。如果省index，则使用下一个区    （前一index + 1）。以十六进制表示（不0x 之类的前缀）
<path_nr>
    路径编号，范0 ... (<num_paths> - 1)。以十六进制表示（不0x 之类的前缀）
R<n>,<m>
    该参数允许快速加载重复模式n> <m> 是十六进制数。最<n> 个映射在接下来的
    <m> 个槽中重复
## 状

不报告任何状态行
## 示例


假设你有大小相同的卷 vg1/switch0、vg1/switch1、vg1/switch2
```

    dmsetup create switch --table "0 `blockdev --getsz /dev/vg1/switch0`
	switch 3 128 0 /dev/vg1/switch0 0 /dev/vg1/switch1 0 /dev/vg1/switch2 0"

```
将前 7 个条目的映射设置为指向设switch0、switch1```

    dmsetup message switch 0 set_region_mappings 0:0 :1 :2 :0 :1 :2 :1

```
```

    dmsetup message switch 0 set_region_mappings 1000:1 :2 R2,10

```
```

    dmsetup message switch 0 set_region_mappings 1000:1 :2 :1 :2 :1 :2 :1 :2 \
	:1 :2 :1 :2 :1 :2 :1 :2 :1 :2

```
