:orphan:

## MSM 崩溃转储格式


在 GPU 挂起之后，MSM 驱动通过 /sys/kernel/dri/X/show 或通过 devcoredump
（/sys/class/devcoredump/dcdX/data）输出调试信息。本文档描述输出的格式。

每个条目都是 key: value 的形式。节的标题没有值，并且该节的所有内容会从标题缩进
两个空格。每个节可能有多个数组条目，数组条目的开始由一个 (-) 标记。

### 映射（Mappings）


kernel
	生成该转储的内核版本（UTS_RELEASE）。

module
	生成该崩溃转储的模块。

time
	崩溃时的内核时间，格式为 秒.微秒。

comm
	产生故障的二进制文件的 comm 字符串。

cmdline
	产生故障的二进制文件的命令行。

revision
	产生崩溃的 GPU 的 ID，格式为 core.major.minor.patchlevel，以句点分隔。

rbbm-status
	RBBM_STATUS 的当前值，显示崩溃时正在使用的顶层 GPU 组件。

ringbuffer
	包含每个 ringbuffer 内容的节。每个 ringbuffer 用一个 id 编号标识。

	id
		Ringbuffer ID（从 0 开始的索引）。该节中的每个 ringbuffer 都有
		自己唯一的 id。
	iova
		ringbuffer 的 GPU 地址。

	last-fence
		在该 ringbuffer 上发出的最后一个 fence

	retired-fence
		在该 ringbuffer 上退役的最后一个 fence。

	rptr
		该 ringbuffer 的当前读指针（rptr）。

	wptr
		该 ringbuffer 的当前写指针（wptr）。

	size
		在硬件中编程的 ringbuffer 的最大大小。

	data
		以 ascii85 编码的 ring 内容。只会打印 ring 中被使用的部分。

bo
	来自挂起提交的缓冲区列表（如果可用）。每个缓冲区对象会有一个唯一的 iova。

	iova
		缓冲区对象的 GPU 地址。

	size
		缓冲区对象分配的大小。

	data
		以 ascii85 编码的缓冲区对象内容。只会跳过缓冲区末尾的尾随零。

registers
	一组寄存器值。每个条目独占一行，用括号 { } 括起来。

	offset
		寄存器距 GPU 内存区域起始处的字节偏移。

	value
		寄存器的十六进制值。

registers-hlsq
		（仅 5xx）来自 HLSQ 孔径的寄存器值。格式与 register 节相同。
