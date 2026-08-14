## 内核态内存映射 I/O 跟踪


主页及可选用户空间工具的链接：

	https://nouveau.freedesktop.org/wiki/MmioTrace

MMIO 跟踪最初由 Intel 在 2003 年左右为其故障注入测试框架（Fault Injection
Test Harness）开发。在 2006 年 12 月至 2007 年 1 月期间，Jeff Muizelaar 利用
Intel 的代码，创建了一个用于跟踪 MMIO 访问的工具，其初衷是为 Nouveau 项目
服务。此后有许多人作出了贡献。

Mmiotrace 是为对任何内存映射 IO 设备进行逆向工程而构建的，Nouveau 项目是其
第一个真实用户。仅支持 x86 和 x86_64 架构。

树外（out-of-tree）的 mmiotrace 最初由 Pekka Paalanen <pq@iki.fi> 修改为可
合入主线，并适配 ftrace 框架。


### 准备


Mmiotrace 功能通过 CONFIG_MMIOTRACE 选项编译进内核。跟踪默认是关闭的，因此
将其设为 yes 是安全的。SMP 系统受支持，但如果多于一个 CPU 处于在线状态，
跟踪将不可靠并可能丢失事件，因此 mmiotrace 在运行时激活期间会使除一个 CPU
之外的所有 CPU 离线。你可以手动重新启用 CPU，但已经警告过你：由于 CPU 竞争，
无法自动检测是否正在丢失事件。


### 用法快速参考

```

	$ mount -t debugfs debugfs /sys/kernel/debug
	$ echo mmiotrace > /sys/kernel/tracing/current_tracer
	$ cat /sys/kernel/tracing/trace_pipe > mydump.txt &
	Start X or whatever.
	$ echo "X is up" > /sys/kernel/tracing/trace_marker
	$ echo nop > /sys/kernel/tracing/current_tracer
	Check for lost events.


```
### 用法


确保 debugfs 已挂载到 /sys/kernel/debug。
```

	$ mount -t debugfs debugfs /sys/kernel/debug

```
确认你即将跟踪的驱动尚未加载。

```

	$ echo mmiotrace > /sys/kernel/tracing/current_tracer

```
```

	$ cat /sys/kernel/tracing/trace_pipe > mydump.txt &

```
'cat' 进程应当保持（睡眠）在后台运行。

加载你想要跟踪的驱动并使用它。Mmiotrace 只会捕获在 mmiotrace 处于活动状态
期间被 ioremap 的区域的 MMIO 访问。

在跟踪期间，你可以通过
$ echo "X is up" > /sys/kernel/tracing/trace_marker
将注释（标记）放入跟踪记录中。这样更容易看清（庞大的）跟踪记录的哪一部分
对应哪个操作。建议放置关于你所做操作的描述性标记。

```

	$ echo nop > /sys/kernel/tracing/current_tracer

```
'cat' 进程退出。如果它没有退出，通过执行 'fg' 命令并按下 ctrl+c 来终止它。

```

	$ grep -i lost mydump.txt

```
```

	$ dmesg

```
以查看内核日志并查找 "mmiotrace has lost events" 警告。如果事件丢失了，
跟踪记录就不完整。你应该扩大缓冲区并重试。缓冲区可通过先查看当前缓冲区
有多大来扩大
```

	$ cat /sys/kernel/tracing/buffer_size_kb

```
会给出一个数字。将该数字大约翻倍并写回，例如
```

	$ echo 128000 > /sys/kernel/tracing/buffer_size_kb

```
然后从头重新开始。

如果你正在为某个驱动项目（例如 Nouveau）做跟踪，你还应当
```

	$ lspci -vvv > lspci.txt
	$ dmesg > dmesg.txt
	$ tar zcf pciid-nick-mmiotrace.tar.gz mydump.txt lspci.txt dmesg.txt

```
然后发送该 .tar.gz 文件。跟踪记录压缩效果显著。将 "pciid" 和 "nick" 替换
为正在调查硬件的 PCI ID 或型号名称以及你的昵称。


### Mmiotrace 的工作原理


对硬件 IO 内存的访问是通过调用某个 ioremap_*() 函数，将从 PCI 总线映射地址
来获得。Mmiotrace 挂载到 __ioremap() 函数，并在每次创建映射时被调用。映射是
一个被记录到跟踪日志中的事件。注意 ISA 范围的映射不会被捕获，因为该映射始终
存在并会被直接返回。

MMIO 访问通过页错误来记录。就在 __ioremap() 返回之前，被映射的页被标记为
不存在。对该页的任何访问都会引发错误。页错误处理程序调用 mmiotrace 来处理
该错误。Mmiotrace 将该页标记为存在，设置 TF 标志以实现单步执行，并退出错误
处理程序。引发错误的指令被执行并进入调试陷阱。在这里 mmiotrace 再次将该页
标记为不存在。该指令被解码以获取操作类型（读/写）、数据宽度以及读写的数值。
这些信息被存储到跟踪日志中。

在页错误处理程序中将页标记为存在在 SMP 机器上存在竞争条件。在单步执行期间，
其他 CPU 可能在该页上自由运行，事件可能在无提示的情况下丢失。不鼓励在跟踪
期间重新启用其他 CPU。


### 跟踪日志格式


原始日志是文本，可以很容易地用 grep、awk 等工具进行过滤。一条记录是日志中的
一行。记录以一个关键字开头，后跟该关键字所依赖的参数。参数之间用空格分隔，
或延续到行尾。版本 20070824 的格式如下：

### 说明	关键字	以空格分隔的参数


读事件	R	width, timestamp, map id, physical, value, PC, PID
写事件	W	width, timestamp, map id, physical, value, PC, PID
ioremap 事件	MAP	timestamp, map id, physical, virtual, length, PC, PID
iounmap 事件	UNMAP	timestamp, map id, PC, PID
标记		MARK	timestamp, text
版本		VERSION	the string "20070824"
供读取者参考的信息	LSPCI	one line from lspci -v
PCI 地址映射	PCIDEV	space-separated /proc/bus/pci/devices data
未知操作码	UNKNOWN	timestamp, map id, physical, data, PC, PID

时间戳以秒为单位，带有小数部分。Physical 是 PCI 总线地址，virtual 是内核虚拟
地址。Width 是数据的字节宽度，value 是数据值。Map id 是一个任意的标识号，用于
标识在某个操作中使用的映射。PC 是程序计数器，PID 是进程 id。如果未被记录，
PC 为零。PID 始终为零，因为尚不支持跟踪源自用户空间内存的 MMIO 访问。

例如，下面的 awk 过滤器会放行所有针对物理地址范围
[0xfb73ce40, 0xfb800000] 的 32 位写操作
```

	$ awk '/W 4 / { adr=strtonum($5); if (adr >= 0xfb73ce40 &&
	adr < 0xfb800000) print; }


```
### 面向开发者的工具


用户空间工具包含以下实用程序：
  - 用硬件寄存器名替换数字地址和数值
  - 回放 MMIO 日志，即重新执行被记录的写操作

