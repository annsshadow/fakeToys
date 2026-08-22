## S/390 通用 I/O 层（common I/O-Layer

## 命令行参数、procfs debugfs 条目


### 命令行参

- ccw_timeout_log

  ccw 设备超时时启用调试信息的记录
- cio_ignore = device[,device[,..]]

	device := {all | [!]ipldev | [!]condev | [!]<devno> | [!]<devno>-<devno>}

  通用 I/O 层将忽略给定的设备；不会对这些设备中的任何一  进行探测和设备感知（sensing）。相关设备所连接的子通道（subchannel  将被当作没有设备连接来处理
  一个被忽略的设备之后可以取消忽略；细节请参见proc 条目”一节
  设备必须作为总线 ID.x.abcd）或十六进制设备  xabcd abcd，为2.4 的向后兼容）给出。如果你给出设备  0xabcd，它将被解释0.0.abcd
  你可以使'all' 关键字忽略所有设备ipldev' 'condev'
  关键字可分别用于指代基于 CCW 的启动设备和 CCW 控制台设  （这些可能只有与 '!' 运算符结合使用时才有用）!' 运算符会
  I/O **忽略某个设备。命令行从左向右解析
```

	cio_ignore=0.0.0023-0.0.0042,0.0.4711

  将忽略从 0.0.0023 0.0.0042 的所有设备，以及设备
  0.0.4711（如果检测到的话）
  另一个例:

	cio_ignore=all,!0.0.4711,!0.0.fd00-0.0.fd02

  将忽略除 0.0.4711.0.fd00.0.fd01.0.fd02
  之外的所有设备
  默认情况下，没有设备被忽略

```
### /proc 条目


- /proc/cio_ignore

  按总线 ID 列出被通用 I/O 忽略的设备范围
  你可以通过/proc/cio_ignore 写入来取消忽略某些或所有设备  "free all" 将取消忽略所有被忽略的设备，
  "free <device range>, <device range>, ..." 将取消忽略指定的
  设备
  例如，如果设0.0.0023 0.0.0042 以及 0.0.4711 被忽略，

  - echo free 0.0.0030-0.0.0032 > /proc/cio_ignore
    将取消忽略设0.0.0030 0.0.0032，并保留设备 0.0.0023
    0.0.002f.0.0033 0.0.0042 以及 0.0.4711 被忽略；
  - echo free 0.0.0041 > /proc/cio_ignore 将进一步取消忽略设    0.0.0041  - echo free all > /proc/cio_ignore 将取消忽略所有剩余的被忽    设备
  当设备被取消忽略时，会执行设备识别与感知，并且如果可能，
  设备驱动会被通知，于是设备将变得对系统可用。注意取消忽略是
  异步执行的
  你也可以通过/proc/cio_ignore 写入来添加要忽略的设备范围；
  "add <device range>, <device range>, ..." 将忽略指定的设备
  注意：虽然已知的设备可以被添加到要忽略的设备列表中，
  但当时不会有任何效果。然而，如果这样的设备消失后又重新出现，
  它随后将被忽略。要使已知设备消失，你需"purge" 命令
  （见下文）
```

	"echo add 0.0.a000-0.0.accc, 0.0.af00-0.0.afff > /proc/cio_ignore"

  将把 0.0.a000-0.0.accc 0.0.af00-0.0.afff 添加到被忽略
  设备的列表中
  你可以通过以下方式移除已知但当前被忽略的设:

	"echo purge > /proc/cio_ignore"

  所有被忽略但仍在注册且未上线（=未在使用中）的设  将被注销，从而从系统中移除
  设备可以按总线 ID.x.abcd）指定，或者为2.4 的向后兼容，
  按十六进制设备号xabcd abcd）指定。以 0xabcd 给出的设备号
  将被解释0.0.abcd
```
- /proc/cio_settle

  对该文件的写请求会被阻塞，直到所有排队的 cio 操作都被
  处理完毕。这将允许用户空间在更改 cio_ignore 或硬件配置后  等待影响设备可用性的待处理工作完成
- 对于 2.4 /proc 文件系统里存在的部分信息（即
  /proc/subchannels /proc/chpids），请参driver-model.txt  原先位于 /proc/irq_count 的信息现在位/proc/interrupts

### debugfs 条目


- /sys/kernel/debug/s390dbf/cio_*/（S/390 调试特性）

  由调试特性生成的一些视图，用于保存各种调试输出
  - /sys/kernel/debug/s390dbf/cio_crw/sprintf
    来自待处理通道报告字（channel report word，机器检查处理）
    处理过程的消息
  - /sys/kernel/debug/s390dbf/cio_msg/sprintf
    来自通用 I/O 层的各种调试消息
  - /sys/kernel/debug/s390dbf/cio_trace/hex_ascii
    记录通用 I/O 层中函数的调用，以及（如果适用）是为哪    子通道调用的，还有一些数据结构（如出错情况下irb）的转储
  可以通过/sys/kernel/debug/s390dbf/cio_*/level 写入一0 6
  之间的数字来改变日志记录的详细程度；细节请参  关于 S/390 调试特性（Documentation/arch/s390/s390dbf.rst）的文档