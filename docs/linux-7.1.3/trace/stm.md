

## 系统追踪模块（System Trace Module）


系统追踪模块（System Trace Module，STM）是 MIPI STP 规范中描述的一种设备，作为 STP
追踪流生成器。STP（System Trace Protocol，系统追踪协议）是一种将来自多个追踪源的数据
进行多路复用的追踪协议，其中每个追踪源都被分配一对唯一的 master 与 channel。虽然这些
master 与 channel 中的一部分被静态分配给某些硬件追踪源，但其余部分可供软件使用。软件
追踪源通常可以自行从该池子中任意选取 master/channel 组合。

在 STP 流的接收端（解码侧），追踪源只能由 master/channel 组合来识别，因此为了让解码器
能够理解涉及多个追踪源的追踪内容，它需要能够将这些 master/channel 对映射到它所认识的
追踪源。

例如，知道 syslog 消息来自 master 7 channel 15 是很有帮助的，而任意用户应用程序可以使用
master 48 到 63 以及 channel 0 到 127。

为了解决这个映射问题，stm 类通过 configfs 提供了一种策略管理机制，允许定义将字符串
标识符映射到 master 与 channel 范围的规则。如果这些规则（策略）与解码器的期望一致，它
就能正确处理追踪数据。

该策略是一个树形结构，包含规则（policy_node），每条规则都有一个名称（字符串标识符）以及
与之关联的一组 master 与 channel 范围，位于 configfs 的 "stp-policy" 子系统目录中。最顶层
目录的名称（即策略）格式为：其所适用的 STM 设备名，后跟一个由句点分隔的任意字符串标识符。
以上面的例子来说，一条规则

```

	$ ls /config/stp-policy/dummy_stm.my-policy/user
	channels masters
	$ cat /config/stp-policy/dummy_stm.my-policy/user/masters
	48 63
	$ cat /config/stp-policy/dummy_stm.my-policy/user/channels
	0 127

```
这意味着该规则的 master 分配池包含 master 48 到 63，channel 分配池包含 channel 0 到 127。
现在，任何以 "user" 标识字符串自我标识的生产者（追踪源）都将被从这些范围内分配一个 master 与 channel。

这些规则可以嵌套，例如，可以在上面例子中的 "user" 目录下定义一条名为 "dummy" 的规则，
这条新规则将用于标识字符串为 "user/dummy" 的追踪源。

追踪源必须打开 stm 类设备的节点，并将它们的追踪数据写入其文件描述符。

为了给给定的追踪源找到合适的策略节点，可以使用若干机制。首先，追踪源可以在向字符设备的
文件描述符写入任何数据之前，通过在其上调用 STP_POLICY_ID_SET ioctl 显式地自我标识，
提供它们的 id 字符串。其次，如果它们选择不进行显式标识（因为你可能不想为此修补现有软件），
它们可以直接开始写入数据，此时 stm 核心会尝试查找名称与任务名（例如 "syslogd"）匹配的策略
节点，如果存在则使用它。第三，如果在策略节点中找不到任务名，则会使用兜底条目 "default"（
如果它存在）。该条目同样需要由系统管理员或负责策略配置的相关工具创建和配置。最后，如果上述
所有步骤都失败，对 stm 文件描述符的 write() 将返回一个错误（EINVAL）。

此前，如果为某个追踪源找不到策略节点，stm 类会默默地回退到从设备 master/channel 范围的
开头分配第一块可用的连续 master/channel 范围。现在要求必须存在策略节点，这将帮助程序员和
系统管理员发现配置中的缺口，并更好地控制未标识的源。

某些 STM 设备可能允许将 channel 的 mmio 区域直接映射到用户空间以实现零拷贝写入。一个可
映射的页（就 mmu 而言）通常包含多个 channel 的 mmio，因此用户需要为自己分配那么多 channel
（通过上述 ioctl() 调用）才能做到这一点。也就是说，如果你的 stm 设备的 channel mmio 区域
为 64 字节，而硬件页大小为 4096 字节，那么在成功调用 width==64 的 STP_POLICY_ID_SET ioctl()
之后，你应该能够在此文件描述符上 mmap() 一页，并获得对 64 个 channel 的 mmio 区域的直接访问。

STM 设备的例子有 Intel(R) Trace Hub [^1^] 与 Coresight STM [^2^]。

## stm_source


对于基于内核的追踪源，存在 "stm_source" 设备类。该类的设备可以在运行时通过名为
"stm_source_link" 的 sysfs 属性连接到 stm 设备或从 stm 设备断开

```

	$ echo dummy_stm.0 > /sys/class/stm_source/console/stm_source_link

```
关于如何在内核中使用 stm_source 接口的示例，请参考 stm_console、stm_heartbeat 或
stm_ftrace 驱动。

每个 stm_source 设备都需要根据它所需的 channel 数量，占用一个 master 以及一段 channel 范围。
这些会根据策略配置为设备分配。如果策略目录的根下存在一个与 stm_source 设备名称（例如
"console"）匹配的节点，则使用该节点来分配 master 与 channel 号。如果不存在这样的策略节点，
stm 核心将使用兜底条目 "default"（如果存在）。如果两种策略节点都不存在，对 stm_source_link
的 write() 将返回一个错误。

## stm_console


上面例子中使用的该接口的另一种实现是 "stm_console" 驱动，它基本上通过 stm 设备为内核消息
提供一个单向控制台。

要配置将在 STP 流中分配给该控制台的 master/channel 对，请创建一个 "console" 策略条目
（如何创建请参见本文开头）。初始化时，它将占用一个 channel。

## stm_ftrace


这是另一个 "stm_source" 设备，一旦 stm_ftrace 与某个 stm 设备建立链接，并且启用了 "function"
追踪器，Ftrace 子系统本应存入环形缓冲区的函数地址与父函数地址，将同时通过 stm 设备导出。

目前仅支持 Ftrace 的 "function" 追踪器。

- [^1^] https://software.intel.com/sites/default/files/managed/d3/3c/intel-th-developer-manual.pdf
- [^2^] http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0444b/index.html
