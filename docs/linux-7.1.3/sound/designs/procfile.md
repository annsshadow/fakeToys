## ALSA 驱动程序Proc 文件


Takashi Iwai <tiwai@suse.de>

## 概述


ALSA 拥有自己proc 树，/proc/asound。许多有用的信息都可以在该树中找到。当你遇
问题需要调试时，请检查下面各节中列出的文件

每张声卡都有它自己的子树 cardX，其X 取值为 0 7。特定于声卡的文件存储在 `card*` 子目录中


## 全局信息


cards
	显示当前已配置的 ALSA 驱动列表、索引、id 字符串、简短与详细描述

version
	显示版本字符串与编译日期

modules
	列出每张声卡的模

devices
	列出 ALSA 原生设备映射

meminfo
	显示通过 ALSA 驱动分配的页面状态
	仅在 `CONFIG_SND_DEBUG=y` 时出现

hwdep
	`<card>-<device>: <name>` 的格式列出当前可用的 hwdep 设备

pcm
	`<card>-<device>: <id>: <name> : <sub-streams>` 的格式列出当前可用的 PCM 设备

timer
	列出当前可用的定时器设备


oss/devices
	列出 OSS 设备映射

oss/sndstat
	提供/dev/sndstat 兼容的输出
	你可以将其符号链接到 /dev/sndstat


## 特定于声卡的文件


特定于声卡的文件位于 `/proc/asound/card*` 目录中。一些驱动（例如 cmipci）拥有自己的
proc 条目用于寄存器转储等（例`/proc/asound/card*/cmipci` 显示寄存器转储）。这些文
对调试非常有帮助

当该声卡上有可用PCM 设备时，你可以看到诸pcm0p pcm1c 这样的目录。它们保存每
PCM 流的 PCM 信息。`pcm` 之后的数字是 PCM 设备号（0 开始），末尾的 `p` `c` 表示
回放（playback）或捕获（capture）方向。此子树中的文件将在后文描述

MIDI I/O 的状态位`midi*` 文件中。它显示设备名称以及通过 MIDI 设备接收/发送的字节数

当声卡配AC97 编解码器时，会有 `codec97#*` 子目录（后文描述）

当启用了 OSS 混音器模拟（且模块已加载）时，这里也会出oss_mixer 文件。它显示当前 OSS
混音器元素到 ALSA 控制元素的映射。你可以通过写入该设备来更改映射。详情请阅读
OSS-Emulation.txt銆。


## PCM Proc 文件


`card**/pcm**/info`
	PCM 设备的通用信息：声卡编号、设备编号、子流等

`card**/pcm**/xrun_debug`
	`CONFIG_SND_DEBUG=y` `CONFIG_SND_PCM_XRUN_DEBUG=y` 时此文件出现
	它显xrun 缓冲区溢欠载）的状态，以及ALSA PCM 中间层的
	无效 PCM 位置调试/检查。它接受一个整数值，可以通过写入来更

```

		 # echo 5 > /proc/asound/card0/pcm0p/xrun_debug

	The value consists of the following bit flags:

	* bit 0 = Enable XRUN/jiffies debug messages
	* bit 1 = Show stack trace at XRUN / jiffies check
	* bit 2 = Enable additional jiffies check

	When the bit 0 is set, the driver will show the messages to
	kernel log when an xrun is detected.  The debug message is
	shown also when the invalid H/W pointer is detected at the
	update of periods (usually called from the interrupt
	handler).

	When the bit 1 is set, the driver will show the stack trace
	additionally.  This may help the debugging.

	Since 2.6.30, this option can enable the hwptr check using
	jiffies.  This detects spontaneous invalid pointer callback
	values, but can be lead to too much corrections for a (mostly
	buggy) hardware that doesn't give smooth pointer updates.
	This feature is enabled via the bit 2.

```
`card**/pcm**/sub*/info`
	PCM 子流的通用信息

`card**/pcm**/sub*/status`
	PCM 子流的当前状态、经过时间、硬件位置等

`card**/pcm**/sub*/hw_params`
	为此子流设置的硬件参数

`card**/pcm**/sub*/sw_params`
	为此子流设置的软件参数

`card**/pcm**/sub*/prealloc`
	缓冲区预分配信息

`card**/pcm**/sub*/xrun_injection`
	当向proc 文件写入任意值时，会向正在运行的流触发一XRUN。用于故障注入
	此条目是只写的

## AC97 编解码器信息


`card**/codec97#**/ac97#?-?`
	显示AC97 编解码器芯片的通用信息，例如名称、能力、设置

`card*/codec97#0/ac97#?-?+regs`
	显示 AC97 寄存器转储。对调试很有用

	当启用了 CONFIG_SND_DEBUG 时，你可以写入此文件以直接更AC97 寄存器。传入两个十六进制数
	例如

```

	# echo 02 9f1f > /proc/asound/card0/codec97#0/ac97#0-0+regs


```
## USB 音频


`card**/stream**`
	显示给定声卡中每个音频流的分配与当前状态。此信息对调试非常有用


## HD-Audio 编解码器


`card**/codec#**`
	显示通用编解码器信息以及每个 widget 节点的属性

`card**/eld#**`
	可用HDMI DisplayPort 接口
	显示从所HDMI 接收端获取的 ELD（EDID Like Data，类 EDID 数据）信息，
	并描述其音频能力与配置

	可以通过执行 `echo name hex_value > eld#*` 来修改某ELD 字段
	只有在你确定 HDMI 接收端提供的值有误时才这样做。如果这样能让你HDMI 音频工作
	请向我们报告，以便我们在未来的内核版本中修复它


## 定序器（Sequencer）信


seq/drivers
	列出当前可用ALSA 定序器驱动

seq/clients
	显示当前可用的定序器客户端与端口列表。连接状态与运行状态也显示在此文件中

seq/queues
	列出当前已分运行的定序器队列

seq/timer
	列出当前已分运行的定序器定时器

seq/oss
	列出OSS 兼容的定序器相关内容


## 调试帮助


当问题与 PCM 相关时，首先尝试打开 xrun_debug 模式。这会在 xrun 发生的时间和位置给出
内核消息

如果这确实是一bug，请附上以下信息报告

- 驱动/声卡的名称，显示`/proc/asound/cards`
- 寄存器转储（如果可用，例`card*/cmipci`

当它PCM 问题时：

- PCM 的设置，显示PCM 子流目录中的 hw_parms、sw_params status

当它是混音器问题时：

- AC97 proc 文件，`codec97#**/**` 文件

对于 USB 音频/MIDI

- `lsusb -v` 的输
- 声卡目录中的 `stream*` 文件


ALSA bug 跟踪系统位于
https://bugtrack.alsa-project.org/alsa-bug/
