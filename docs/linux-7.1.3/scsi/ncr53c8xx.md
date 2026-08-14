
## NCR53C8XX/SYM53C8XX 驱动


作者：Gerard Roudier <groudier@free.fr>

21 Rue Carnot

95170 DEUIL LA BARRE - 法国

1999 年 5 月 29 日


   1. 简介
   2. 支持的芯片与 SCSI 特性
   3. 增强型 896 驱动的优势
         3.1 优化后的 SCSI SCRIPTS
         3.2 SYM53C896 的新特性（64 位 PCI 双通道 LVD SCSI 控制器）
   4. 内存映射 I/O 与普通 I/O
   5. 标记命令队列
   6. 奇偶校验
   7. 性能剖析信息
   8. 控制命令
         8.1  设置最小同步周期因子
         8.2  设置总线宽度
         8.3  设置并发标记命令的最大数量
         8.4  设置标记命令的排序类型
         8.5  设置调试模式
         8.6  清除性能剖析计数器
         8.7  设置标志位（no_disc）
         8.8  设置详细输出级别
         8.9  复位目标的全部逻辑单元
         8.10 中止目标所有逻辑单元的全部任务
   9. 配置参数
   10. 启动设置命令
         10.1 语法
         10.2 可用参数
                10.2.1  主设备奇偶校验
                10.2.2  SCSI 奇偶校验
                10.2.3  SCSI 断开连接
                10.2.4  特殊特性
                10.2.5  Ultra SCSI 支持
                10.2.6  默认标记命令数量
                10.2.7  默认同步周期因子
                10.2.8  与所有设备协商同步传输
                10.2.9  详细输出级别
                10.2.10 调试模式
                10.2.11 突发最大长度
                10.2.12 LED 支持
                10.2.13 最大总线宽度
                10.2.14 差分模式
                10.2.15 中断请求模式
                10.2.16 反向探测
                10.2.17 修复 PCI 配置空间
                10.2.18 串行 NVRAM
                10.2.19 检查 SCSI 总线
                10.2.20 排除某个主机不被挂载
                10.2.21 为主机建议默认 SCSI ID
                10.2.22 启用 IMMEDIATE ARBITRATION（立即仲裁）
         10.3 建议的启动设置命令
         10.4 PCI 配置修复启动选项
         10.5 串行 NVRAM 支持启动选项
         10.6 SCSI 总线检查启动选项
         10.7 IMMEDIATE ARBITRATION 启动选项
   11. ncr53c8xx.h 头文件中的一些常量与标志
   12. 安装
   13. 与体系结构相关的特性
   14. 已知问题
         14.1 使用 Iomega Jaz 设备的标记命令
         14.2 添加另一控制器时设备名发生变化
         14.3 在 WIDE SCSI 控制器上仅使用 8 位设备
         14.4 内存写并使无效期间可能出现的数据损坏
   15. SCSI 问题排查
         15.1 问题追踪
         15.2 理解硬件错误报告
   16. 同步传输协商表
         16.1 53C875 与 53C860 Ultra-SCSI 控制器的同步时序
         16.2 快速 SCSI-2 53C8XX 控制器的同步时序
   17. 串行 NVRAM 支持（作者 Richard Waltham）
         17.1 特性
         17.2 Symbios NVRAM 布局
         17.3 Tekram NVRAM 布局
   18. 大端支持
         18.1 大端 CPU
         18.2 运行于大端模式的 NCR 芯片


## 1. 简介


最初的 Linux ncr53c8xx 驱动是 FreeBSD 上 ncr 驱动的一个移植版本，由
以下人员在 1995 年 11 月完成：

 - Gerard Roudier              <groudier@free.fr>

最初的驱动由以下人员为 386bsd 和 FreeBSD 编写：

        - Wolfgang Stanglmeier        <wolf@cologne.de>
        - Stefan Esser                <se@mi.Uni-Koeln.de>

如今它以两个驱动捆绑的形式提供：

- ncr53c8xx 通用驱动，支持整个 SYM53C8XX 系列，包括最早的 810 rev. 1、
  最新的 896（双通道 LVD SCSI 控制器）以及新的 895A（单通道 LVD SCSI 控制器）。
- sym53c8xx 增强型驱动（又称 896 驱动），它放弃了对最老芯片的支持，
  以获得新特性的优势，例如自 810A 起可用的 LOAD/STORE 指令，以及
  896 和 895A 上可用的硬件相位失配处理。

关于 NCR 8xx 系列的技术信息，可在 Michael Will 编写的 PCI-HOWTO
以及 Drew Eckhardt 编写的 SCSI-HOWTO 中找到。

关于新芯片的信息可在 LSILOGIC 的 Web 服务器上获取：

          - http://www.lsilogic.com/

SCSI 标准文档可在 SYMBIOS 的 ftp 服务器上获取：

          - ftp://ftp.symbios.com/

Eric Youngdale 编写的一些有用的 SCSI 工具可在 tsx-11 获取：

          - ftp://tsx-11.mit.edu/pub/linux/ALPHA/scsi/scsiinfo-X.Y.tar.gz
          - ftp://tsx-11.mit.edu/pub/linux/ALPHA/scsi/scsidev-X.Y.tar.gz

这些工具并非 ALPHA 版本，而是相当干净并且工作良好的。拥有
'scsiinfo' 软件包是必不可少的。

这份简短的文档描述了通用驱动与增强型驱动的特性、配置参数，以及
通过 proc SCSI 文件系统的读/写操作可用的控制命令。

该驱动已在 linux/i386、Linux/Alpha 和 Linux/PPC 上测试通过。

最新的驱动版本与补丁可在以下位置获取：

          - ftp://ftp.tux.org/pub/people/gerard-roudier

或

          - ftp://ftp.symbios.com/mirror/ftp.tux.org/pub/tux/roudier/drivers

我并非英语母语者，这份 README 文件中大概有不少错误。欢迎任何帮助。


## 2. 支持的芯片与 SCSI 特性


以下特性对所有芯片都提供支持：

 - 同步协商
 - 断开连接
 - 标记命令队列
 - SCSI 奇偶校验
 - 主设备奇偶校验

"宽协商" 对支持它的芯片提供。下面的表格展示了 NCR 8xx 系列部分芯片
的特性，以及哪些驱动支持它们。

+--------+-----------+-----+-----------+------------+------------+------------+
|        |           |     |           |            |Supported by|Supported by|
|        |On board   |     |           |            |the generic |the enhanced|
|Chip    |SDMS BIOS  |Wide |SCSI std.  | Max. sync  |driver      |driver      |
+--------+-----------+-----+-----------+------------+------------+------------+
|810     |  N        | N   |  FAST10   | 10 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|810A    |  N        | N   |  FAST10   | 10 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|815     |  Y        | N   |  FAST10   | 10 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|825     |  Y        | Y   |  FAST10   | 20 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|825A    |  Y        | Y   |  FAST10   | 20 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|860     |  N        | N   |  FAST20   | 20 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|875     |  Y        | Y   |  FAST20   | 40 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|876     |  Y        | Y   |  FAST20   | 40 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|895     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|895A    |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|896     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|897     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1510D   |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1010    |  Y        | Y   |  FAST80   |160 MB/s    |    N       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1010_66 |  Y        | Y   |  FAST80   |160 MB/s    |    N       |    Y       |
|[^1^]_    |           |     |           |            |            |            |
+--------+-----------+-----+-----------+------------+------------+------------+



其他受支持特性摘要：

:Module:                允许加载该驱动
:Memory mapped I/O:     提升性能
:Profiling information: 来自 proc SCSI 文件系统的读操作
:Control commands:      对 proc SCSI 文件系统的写操作
:Debugging information: 写入 syslog（仅供专家）
:Serial NVRAM:          Symbios 与 Tekram 格式

- 分散 / 聚集（Scatter / gather）
- 共享中断
- 启动设置命令


## 3. 增强型 896 驱动的优势


### 3.1 优化后的 SCSI SCRIPTS


810A、825A、875、895、896 和 895A 支持名为 LOAD 和 STORE 的新 SCSI
SCRIPTS 指令，它们能够比 53c7xx 与 53c8xx 系列所支持的 MOVE MEMORY
指令更快地在 IO 寄存器与内存之间移动最多 1 个 DWORD。
LOAD/STORE 指令支持绝对寻址与 DSA 相对寻址模式。SCSI SCRIPTS 已完全
使用 LOAD/STORE 重写了，取代了 MOVE MEMORY 指令。

### 3.2 SYM53C896 的新特性（64 位 PCI 双通道 LVD SCSI 控制器）


896 与 895A 允许从 SCRIPTS 处理相位失配上下文（避免了相位失配中断，
该中断会暂停 SCSI 处理器，直到 C 代码保存了传输的上下文）。
如果不使用 LOAD/STORE 指令来实现这一点将十分痛苦，我甚至都不想尝试。

896 芯片支持 64 位 PCI 事务与寻址，而 895A 支持 32 位 PCI 事务与
64 位寻址。这些芯片的 SCRIPTS 处理器并非真正的 64 位，而是使用
段寄存器来处理第 32-63 位。另一个有趣的特性是，寻址片上 RAM（8k）的
LOAD/STORE 指令保持在芯片内部。

由于使用了 LOAD/STORE SCRIPTS 指令，该驱动不再支持以下芯片：

- SYM53C810 revision < 0x10 (16)
- SYM53C815 所有版本
- SYM53C825 revision < 0x10 (16)

## 4. 内存映射 I/O 与普通 I/O


内存映射 I/O 比普通 I/O 具有更低的延迟。自 linux-1.3.x 起，使用内存映射
I/O 而非普通 I/O。内存映射 I/O 在大多数硬件配置上似乎工作良好，但
一些设计不佳的主板可能会破坏这一特性。

配置选项 CONFIG_SCSI_NCR53C8XX_IOMAPPED 强制驱动在所有情况下
都使用普通 I/O。


## 5. 标记命令队列


向一个设备一次排队多于 1 条命令，可以让它基于实际的磁头位置及其机械
特性进行优化。这一特性还可以降低平均命令延迟。为了真正获得该特性的
优势，设备必须拥有合理的缓存大小（对于 128 KB 或更小的低端硬盘，
不要指望什么奇迹）。
一些已知的 SCSI 设备不能正确支持标记命令队列。通常，修复此类问题的
固件修订版可在相应厂商的 Web/ftp 站点获取。
我所能说的是，我机器上使用的硬盘在该驱动启用标记命令队列时表现良好：

- IBM S12 0662
- Conner 1080S
- Quantum Atlas I
- Quantum Atlas II

如果你的控制器带有 NVRAM，你可以通过用户设置工具按目标配置这一特性。
Tekram 设置程序允许将排队命令的最大数量调整到 32。Symbios 设置程序
只允许启用或禁用该特性。

排队到某个设备的同时标记命令的最大数量目前默认设为 8。这个值适用于
大多数 SCSI 硬盘。对于大型 SCSI 硬盘（>= 2GB，缓存 >= 512KB，平均
寻道时间 <= 10 ms），使用更大的值可能会获得更好的性能。

sym53c8xx 驱动每个设备最多支持 255 条命令，通用 ncr53c8xx 驱动最多支持
64 条，但使用超过 32 条通常并不划算，除非你使用的是非常大的硬盘或
磁盘阵列。值得注意的是，大多数近期的硬盘似乎不接受超过 64 条并发命令。
因此，使用超过 64 条排队命令大概只是在浪费资源。

如果你的控制器没有 NVRAM，或者它由 SDMS BIOS/SETUP 管理，你可以配置
标记队列特性以及设备队列
```
ncr53c8xx=tags:4/t2t3q15-t4q7/t1u0q32
```
会将标记命令的队列深度设置如下：

- 控制器 0 上的目标 2  所有逻辑单元  --> 15
- 控制器 0 上的目标 3  所有逻辑单元  --> 15
- 控制器 0 上的目标 4  所有逻辑单元  -->  7
- 控制器 1 上的目标 1  逻辑单元 0     --> 32
- 所有其他目标/逻辑单元             -->  4

在某些特殊条件下，某些 SCSI 硬盘固件可能会针对一条 SCSI 命令返回
QUEUE FULL（队列已满）状态。驱动使用以下启发式方法处理这种行为：

- 每次返回 QUEUE FULL 状态时，标记队列深度会被降低到当前已断开连接
  命令的实际数量。

- 每成功完成 1000 条 SCSI 命令，若当前上限允许，可排队的命令最大数量
  会递增。

由于接收和处理 QUEUE FULL 状态会浪费资源，默认情况下驱动会通过
指示实际使用的命令数量及其状态，以及它对设备队列深度变化的决定，
将此问题通知用户。
驱动处理 QUEUE FULL 所使用的启发式方法确保了性能所受影响不会太糟。你
可以通过以下方式将消息关掉：将详细输出级别设为 0，如下所示：

第一种方法：
	    使用 'ncr53c8xx=verb:0' 选项启动你的系统。

第二种方法：
	    在启动后，对与你的控制器对应的 proc 文件系统条目应用
            "setverbose 0" 控制命令。

## 6. 奇偶校验


该驱动支持 SCSI 奇偶校验与 PCI 总线主设备奇偶校验。为了确保数据传输
安全，必须启用这些特性。然而，某些有缺陷的设备或主板会在奇偶校验上
遇到问题。你可以通过在启动命令行中输入适当的选项来禁用 PCI 奇偶校验
或 SCSI 奇偶校验。（参见 10：启动设置命令）。

## 7. 性能剖析信息


性能剖析信息可通过 proc SCSI 文件系统获取。由于收集性能剖析信息可能
影响性能，该特性默认是禁用的，并且需要将该编译配置选项设为 Y。

```

          /proc/scsi/ncr53c8xx/N     (N=0,1,2 ....)

```
```

          /proc/scsi/ncr53c8xx/0

```
然而，如果驱动被编译为模块，主机的编号会在每次加载驱动时递增。

```

         cat /proc/scsi/ncr53c8xx/0

```
```

    General information:
    Chip NCR53C810, device id 0x1, revision id 0x2
    IO port address 0x6000, IRQ number 10
    Using memory mapped IO at virtual address 0x282c000
    Synchronous transfer period 25, max commands per lun 4
    Profiling information:
    num_trans    = 18014
    num_kbytes   = 671314
    num_disc     = 25763
    num_break    = 1673
    num_int      = 1685
    num_fly      = 18038
    ms_setup     = 4940
    ms_data      = 369940
    ms_disc      = 183090
    ms_post      = 1320

```
常规信息很好理解。设备 ID 与修订 ID 对 SCSI 芯片的标识如下：

======= ============= ===========
Chip    Device id     Revision Id
======= ============= ===========
810       0x1            <  0x10
810A      0x1            >= 0x10
815       0x4
825       0x3            <  0x10
860       0x6
825A      0x3            >= 0x10
875       0xf
895       0xc
======= ============= ===========

性能剖析信息在 SCSI 命令完成时更新。当主机适配器被挂载时会分配并
清零一个数据结构。因此，如果驱动是模块，则每次加载驱动时性能剖析
计数器都会被清零。"clearprof" 命令允许你随时清除这些计数器。

可用的计数器如下：

（"num" 前缀表示"数量"，
"ms" 表示毫秒）

num_trans
	已完成的命令数量
	以上示例：18014 条已完成的命令

num_kbytes
	已传输的千字节数
	以上示例：已传输 671 MB

num_disc
	SCSI 断开连接次数
	以上示例：25763 次 SCSI 断开连接

num_break
	脚本中断次数（相位失配）
	以上示例：1673 次脚本中断

num_int
	非"运行中"（on the fly）的中断次数
	以上示例：1685 次非"运行中"的中断

num_fly
	"运行中"的中断次数
	以上示例：18038 次"运行中"的中断

ms_setup
	SCSI 命令建立的耗时
	以上示例：4.94 秒

ms_data
	数据传输的耗时
	以上示例：数据传输耗时 369.94 秒

ms_disc
	SCSI 断开连接的耗时
	以上示例：断开连接耗时 183.09 秒

ms_post
	命令后处理的耗时
	（从获取 SCSI 状态到调用命令完成的时间）
	以上示例：后处理耗时 1.32 秒

由于系统时钟的 1/100 秒节拍，"ms_post" 时间可能是错误的。

在上面的示例中，我们得到了 18038 次"运行中"的中断，以及通常由于
分散列表某一段内部的断开连接所导致的 1673 次脚本中断。


## 8. 控制命令


控制命令可以通过对 proc SCSI 文件系统执行写操作发送给驱动。通用
命令语法如下：

```

      echo "<verb> <parameters>" >/proc/scsi/ncr53c8xx/0
      (assumes controller number is 0)

```
对以下命令使用 "all" 作为 "<target>" 参数将应用于 SCSI 链上的所有
目标（控制器本身除外）。

可用命令：

### 8.1 设置最小同步周期因子


    setsync <target> <period factor>

    :target:   target number
    :period:   minimum synchronous period.
               Maximum speed = 1000/(4*period factor) except for special
               cases below.

    指定周期值 255，以强制进入异步传输模式。

      - 10 表示 25 纳秒同步周期
      - 11 表示 30 纳秒同步周期
      - 12 表示 50 纳秒同步周期

### 8.2 设置总线宽度


    setwide <target> <size>

    :target:   target number
    :size:     0=8 bits, 1=16bits

### 8.3 设置并发标记命令的最大数量


    settags <target> <tags>

    :target:   target number
    :tags:     number of concurrent tagged commands
               must not be greater than SCSI_NCR_MAX_TAGS (default: 8)

### 8.4 设置标记命令的排序类型


    setorder <order>

    :order:    3 possible values:

               simple:
			use SIMPLE TAG for all operations (read and write)

               ordered:
			use ORDERED TAG for all operations

               default:
			use default tag type,
                        SIMPLE  TAG for read  operations
                        ORDERED TAG for write operations


### 8.5 设置调试模式


    setdebug <list of debug flags>

    Available debug flags:

	======== ========================================================
        alloc    print info about memory allocations (ccb, lcb)
        queue    print info about insertions into the command start queue
        result   print sense data on CHECK CONDITION status
        scatter  print info about the scatter process
        scripts  print info about the script binding process
	tiny     print minimal debugging information
	timing   print timing information of the NCR chip
	nego     print information about SCSI negotiations
	phase    print information on script interruptions
	======== ========================================================

    使用不带参数的 "setdebug" 来重置调试标志。


### 8.6 清除性能剖析计数器


    clearprof

    The profile counters are automatically cleared when the amount of
    data transferred reaches 1000 GB in order to avoid overflow.
    The "clearprof" command allows you to clear these counters at any time.


### 8.7 设置标志位（no_disc）


    setflag <target> <flag>

    target:    target number

    For the moment, only one flag is available:

        no_disc:   not allow target to disconnect.

    Do not specify any flag in order to reset the flag. For example:

    setflag 4
      will reset no_disc flag for target 4, so will allow it disconnections.

    setflag all
      will allow disconnection for all devices on the SCSI bus.


### 8.8 设置详细输出级别


    setverbose #level

    The driver default verbose level is 1. This command allows to change
    th driver verbose level after boot-up.

### 8.9 复位目标的全部逻辑单元


    resetdev <target>

    :target:   target number

    The driver will try to send a BUS DEVICE RESET message to the target.
    (Only supported by the SYM53C8XX driver and provided for test purpose)

### 8.10 中止目标所有逻辑单元的全部任务


    cleardev <target>

    :target:   target number

    The driver will try to send a ABORT message to all the logical units
    of the target.

    (Only supported by the SYM53C8XX driver and provided for test purpose)

## 9. 配置参数


如果所有设备的固件都足够完善，驱动支持的全部特性都可以在启动时启用。
然而，如果只有一个设备在某个 SCSI 特性上有缺陷，你可以在 Linux 启动时
禁用驱动对该特性的支持，并在启动后仅为能安全支持该特性的设备启用它。

CONFIG_SCSI_NCR53C8XX_IOMAPPED       (默认回答：n)
    如果你怀疑你的主板不允许内存映射 I/O，回答 "y"。

    可能会稍微降低一点性能。Linux/PPC 需要此选项，无论你在此选择什么
    都会使用它。Linux/PPC 使用此选项不会损失性能，因为所有 IO 都是
    内存映射的。

CONFIG_SCSI_NCR53C8XX_DEFAULT_TAGS    (默认回答：8)
    默认标记命令队列深度。

CONFIG_SCSI_NCR53C8XX_MAX_TAGS         (默认回答：8)
    此选项允许你指定可排队到某个设备的最大标记命令数量。最大支持值为 32。

CONFIG_SCSI_NCR53C8XX_SYNC            (默认回答：5)
    此选项允许你指定驱动在启动时用于同步数据传输协商的频率（单位 MHz）。
    此频率之后可用 "setsync" 控制命令更改。0 表示"异步数据传输"。

CONFIG_SCSI_NCR53C8XX_FORCE_SYNC_NEGO (默认回答：n)
    对所有 SCSI-2 设备强制进行同步协商。

    某些 SCSI-2 设备不在查询响应的第 7 字节中报告此特性，但却能
    正确支持它（例如 TAMARACK 扫描仪）。

CONFIG_SCSI_NCR53C8XX_NO_DISCONNECT   (默认以及唯一合理的回答：n)
    如果你怀疑你的某个设备不能正确支持断开连接，可以回答 "y"。这样，
    所有 SCSI 设备即使在执行长时间的 SCSI 操作时也永远不会断开总线。

CONFIG_SCSI_NCR53C8XX_SYMBIOS_COMPAT
    正品 SYMBIOS 板卡使用 GPIO0 作为输出以驱动控制器 LED，并使用 GPIO3
    位作为单端/差分接口的标示标志。
    如果你的系统中的所有板卡都是正品 SYMBIOS 板卡，或者使用来自 SYMBIOS
    的 BIOS 和驱动，你会希望启用此选项。

    如果你的系统至少有一个基于 53C8XX 的 SCSI 板卡带有厂商特定的 BIOS，
    则绝不能启用此选项。例如，Tekram DC-390/U、DC-390/W 和 DC-390/F
    SCSI 控制器使用厂商特定的 BIOS，并且已知不使用 SYMBIOS 兼容的 GPIO
    接线。因此，如果你的系统安装了这样的板卡，则绝不能启用此选项。

CONFIG_SCSI_NCR53C8XX_NVRAM_DETECT
    启用对 Symbios 以及部分 Symbios 兼容卡（还有 Tekram DC390W/U/F 卡）
    上的串行 NVRAM 数据的读取支持。对于拥有多个 Symbios 兼容控制器、
    其中至少一个带有串行 NVRAM 的系统，或同时混有 Symbios 与 Tekram
    卡的系统很有用。可让主机适配器的扫描顺序设置为默认顺序或"反向探测"
    顺序之外的其他顺序。
    还能区分 Symbios 卡与 Tekram 卡，从而可以在混有 Symbios 与 Tekram
    卡的系统上设置 CONFIG_SCSI_NCR53C8XX_SYMBIOS_COMPAT，使 Symbios 卡
    能够使用包括差分、LED 引脚在内的全部 Symbios 特性，而不会给 Tekram
    卡带来问题。


## 10. 启动设置命令


### 10.1 语法


启动设置命令既可以在启动时传递给驱动，也可以作为字符串变量使用
'insmod' 传递。

ncr53c8xx（sym53c8xx）驱动的启动设置命令以驱动名 "ncr53c8xx="（sym53c8xx）
开头。然后内核语法解析器期望一个可选的、以逗号分隔的整数列表，后跟一个
可选的、以逗号分隔的字符串列表。lilo 下启动设置命令的示例：

```
    lilo: linux root=/dev/hda2 ncr53c8xx=tags:4,sync:10,debug:0x200

```
- 启用标记命令，最多排队 4 条标记命令。
- 将同步协商速度设为 10 兆传输/秒。
- 设置 DEBUG_NEGO 标志。

由于在使用 'insmod' 定义字符串变量时似乎不允许使用逗号，驱动也接受
将 <空格> 作为选项分隔符。以下命令将使用与上面相同的选项安装驱动模块：

```
    insmod ncr53c8xx.o ncr53c8xx="tags:4 sync:10 debug:0x200"

```
目前，整数参数列表会被驱动丢弃。将来会用于支持按控制器进行设置。

每个字符串参数必须指定为 "keyword:value"。只允许小写字母和数字。

在包含多个 53C8xx 适配器的系统中，insmod 会在每个适配器上安装指定的
驱动。要排除某个芯片，请使用 'excl' 关键字。

```
    insmod sym53c8xx sym53c8xx=excl:0x1400
    insmod ncr53c8xx

```
将在除 IO 端口地址 0x1400 处的适配器之外的所有适配器上安装 sym53c8xx
驱动，然后在 IO 端口地址 0x1400 处的适配器上安装 ncr53c8xx 驱动。


### 10.2 可用参数


##### 10.2.1  主设备奇偶校验


	======     ========
        mpar:y     enabled
        mpar:n     disabled
	======     ========

##### 10.2.2  SCSI 奇偶校验


	======     ========
        spar:y     enabled
        spar:n     disabled
	======     ========

##### 10.2.3  SCSI 断开连接


	======     ========
        disc:y     enabled
        disc:n     disabled
	======     ========

##### 10.2.4  特殊特性


   仅适用于 810A、825A、860、875 和 895 控制器。对其他控制器无效。

	=======    =================================================
        specf:y    (or 1) enabled
        specf:n    (or 0) disabled
        specf:3           enabled except Memory Write And Invalidate
	=======    =================================================

   驱动的默认设置为 'specf:3'。因此，若要启用"内存写并使无效"
   （Memory Write And Invalidate），必须在启动设置命令中指定选项 'specf:y'。

##### 10.2.5  Ultra SCSI 支持


   仅适用于 860、875、895、895a、896、1010 和 1010_66 控制器。对其他控制器无效。

	=======    ========================
        ultra:n    All ultra speeds enabled
        ultra:2    Ultra2 enabled
        ultra:1    Ultra enabled
        ultra:0    Ultra speeds disabled
	=======    ========================

##### 10.2.6  默认标记命令数量


	======================= ===============================
        tags:0     (or tags:1 ) tagged command queuing disabled
        tags:#tags (#tags  > 1) tagged command queuing enabled
	======================= ===============================

  #tags 会被截断为"最大排队命令数"配置参数的值。此选项还允许为
  每个支持标记命令队列的设备指定命令队列深度。

```
      ncr53c8xx=tags:10/t2t3q16-t5q24/t1u2q32

  will set devices queue depth as follow:

      - controller #0 target #2 and target #3                  -> 16 commands,
      - controller #0 target #5                                -> 24 commands,
      - controller #1 target #1 logical unit #2                -> 32 commands,
      - all other logical units (all targets, all controllers) -> 10 commands.

```
##### 10.2.7  默认同步周期因子


============ ========================================================
sync:255     disabled (asynchronous transfer mode)
sync:#factor
	     ============     =======================================
	     #factor = 10     Ultra-2 SCSI 40 Mega-transfers / second
	     #factor = 11     Ultra-2 SCSI 33 Mega-transfers / second
	     #factor < 25     Ultra   SCSI 20 Mega-transfers / second
	     #factor < 50     Fast    SCSI-2
	     ============     =======================================
============ ========================================================

  在所有情况下，驱动都将根据 NCR53C8XX 芯片类型使用控制器所支持的最小
  传输周期。

##### 10.2.8  与所有设备协商同步传输


        (force sync nego)

        =====      =========
        fsn:y      enabled
        fsn:n      disabled
        =====      =========

##### 10.2.9  详细输出级别


        ======     =========
        verb:0     minimal
        verb:1     normal
        verb:2     too much
        ======     =========

##### 10.2.10 调试模式


========   ==================================================================
debug:0    clear debug flags
debug:#x   set debug flags

	    #x is an integer value combining the following power-of-2 values:

	    =============  ======
	    DEBUG_ALLOC       0x1
	    DEBUG_PHASE       0x2
	    DEBUG_POLL        0x4
	    DEBUG_QUEUE       0x8
	    DEBUG_RESULT     0x10
	    DEBUG_SCATTER    0x20
	    DEBUG_SCRIPT     0x40
	    DEBUG_TINY       0x80
	    DEBUG_TIMING    0x100
	    DEBUG_NEGO      0x200
	    DEBUG_TAGS      0x400
	    DEBUG_FREEZE    0x800
	    DEBUG_RESTART  0x1000
	    =============  ======
========   ==================================================================

  你可以安全地试用 DEBUG_NEGO。但是，其中某些标志可能会产生大量
  syslog 消息。

##### 10.2.11 突发最大长度


=========  ==================================================================
burst:0    burst disabled
burst:255  get burst length from initial IO register settings.
burst:#x   burst enabled (1<<#x burst transfers max)

	   #x 是一个整数值，为突发传输最大值的以 2 为底的对数。

	   NCR53C875 与 NCR53C825A 最多支持 128 次突发传输（#x = 7）。

	   其他芯片最多只支持 16 次（#x = 4）。

	   这是一个最大值。驱动根据芯片与修订 ID 设置突发长度。默认情况下
	   驱动使用芯片所支持的最大值。
=========  ==================================================================

##### 10.2.12 LED 支持


        =====      ===================
        led:1      enable  LED support
        led:0      disable LED support
        =====      ===================

  如果你的 SCSI 板卡不使用 SDMS BIOS，请勿启用 LED 支持。
  （参见"配置参数"）

##### 10.2.13 最大总线宽度


        ======     ===================
        wide:1      wide scsi enabled
        wide:0      wide scsi disabled
        ======     ===================

  某些 SCSI 板卡使用 875（ultra wide）但只提供窄型连接器。如果你用一根
  50 针转 68 针的电缆转接器连接了一个宽型设备，任何被接受的宽协商都会
  破坏后续的数据传输。在这种情况下，在启动命令中使用 "wide:0" 会很有帮助。

##### 10.2.14 差分模式


	======	=================================
        diff:0	never set up diff mode
        diff:1	set up diff mode if BIOS set it
        diff:2	always set up diff mode
        diff:3	set diff mode if GPIO3 is not set
	======	=================================

##### 10.2.15 中断请求模式


	=========  ========================================================
        irqm:0     always open drain
        irqm:1     same as initial settings (assumed BIOS settings)
        irqm:2     always totem pole
        irqm:0x10  driver will not use IRQF_SHARED flag when requesting irq
	=========  ========================================================
    （0x10 与 0x20 位可与硬件中断请求模式选项组合）

##### 10.2.16 反向探测


	=========   ========================================================
        revprob:n   probe chip ids from the PCI configuration in this order:
                    810, 815, 820, 860, 875, 885, 895, 896
        revprob:y   probe chip ids in the reverse order.
	=========   ========================================================

##### 10.2.17 修复 PCI 配置空间


        pcifix:<option bits>

    Available option bits:

	===    ===============================================================
        0x0    No attempt to fix PCI configuration space registers values.
        0x1    Set PCI cache-line size register if not set.
        0x2    Set write and invalidate bit in PCI command register.
        0x4    Increase if necessary PCI latency timer according to burst max.
	===    ===============================================================

    使用 'pcifix:7' 以允许驱动修复所有 PCI 特性。

##### 10.2.18 串行 NVRAM


	=======     =========================================
        nvram:n     do not look for serial NVRAM
        nvram:y     test controllers for onboard serial NVRAM
	=======     =========================================

        （二进制备选形式）
        mvram=<bits options>

        ====   =================================================================
        0x01   look for NVRAM  (equivalent to nvram=y)
        0x02   ignore NVRAM "Synchronous negotiation" parameters for all devices
        0x04   ignore NVRAM "Wide negotiation"  parameter for all devices
        0x08   ignore NVRAM "Scan at boot time" parameter for all devices
        0x80   also attach controllers set to OFF in the NVRAM (sym53c8xx only)
        ====   =================================================================

##### 10.2.19 检查 SCSI 总线


        buschk:<option bits>

    Available option bits:

        ====   ================================================
        0x0:   No check.
        0x1:   Check and do not attach the controller on error.
        0x2:   Check and just warn on error.
        0x4:   Disable SCSI bus integrity checking.
        ====   ================================================

##### 10.2.20 排除某个主机不被挂载


        excl=<io_address>

    阻止位于给定 IO 地址的主机被挂载。
    例如 'ncr53c8xx=excl:0xb400,excl:0xc000' 指示 ncr53c8xx 驱动
    不要挂载地址为 0xb400 与 0xc000 的主机。

##### 10.2.21 为主机建议默认 SCSI ID


	==========	==========================================
        hostid:255	no id suggested.
        hostid:#x	(0 < x < 7) x suggested for hosts SCSI id.
	==========	==========================================

    如果 NVRAM 中提供了主机 SCSI ID，驱动将忽略任何作为启动选项建议的
    值。否则，如果提供了不同于 255 的建议值，则会使用它。否则，它会
    尝试推断此前在硬件中设置的值，并在硬件值为零时使用值 7。

##### 10.2.22 启用 IMMEDIATE ARBITRATION（立即仲裁）


        （仅由 sym53c8xx 驱动支持。详见 10.7）

=======   =================================================================
iarb:0    do not use this feature.
iarb:#x   use this feature according to bit fields as follow:

	  ========= =======================================================
	  bit 0 (1) enable IARB each time the initiator has been reselected
		    when it arbitrated for the SCSI BUS.
	  (#x >> 4) maximum number of successive settings of IARB if the
		    initiator win arbitration and it has other commands
		    to send to a device.
	  ========= =======================================================
=======   =================================================================

Boot fail safe
    safe:y	load the following assumed fail safe initial setup

  ========================	======================	==========
  master parity			disabled		mpar:n
  scsi parity			enabled			spar:y
  disconnections		not allowed		disc:n
  special features		disabled		specf:n
  ultra scsi			disabled		ultra:n
  force sync negotiation	disabled		fsn:n
  reverse probe			disabled		revprob:n
  PCI fix up                    disabled                pcifix:0
  serial NVRAM                  enabled                 nvram:y
  verbosity level		2			verb:2
  tagged command queuing	disabled		tags:0
  synchronous negotiation	disabled		sync:255
  debug flags			none			debug:0
  burst length			from BIOS settings	burst:255
  LED support			disabled		led:0
  wide support			disabled		wide:0
  settle time			10 seconds		settle:10
  differential support		from BIOS settings	diff:1
  irq mode			from BIOS settings	irqm:1
  SCSI BUS check		do not attach on error	buschk:1
  immediate arbitration		disabled		iarb:0
  ========================	======================	==========

##### 10.3 建议的启动设置命令


如果驱动使用默认选项配置，等效的

```
   ncr53c8xx=mpar:y,spar:y,disc:y,specf:3,fsn:n,ultra:2,fsn:n,revprob:n,verb:1\
             tags:0,sync:50,debug:0,burst:7,led:0,wide:1,settle:2,diff:0,irqm:0

```
对于一个安装软盘或一个安全但不快的系统，

```
    ncr53c8xx=safe:y,mpar:y,disc:y
    ncr53c8xx=safe:y,disc:y
    ncr53c8xx=safe:y,mpar:y
    ncr53c8xx=safe:y

```
```

   ncr53c8xx=mpar:y,spar:y,disc:y,specf:1,fsn:n,ultra:2,fsn:n,revprob:n,verb:1\
             tags:32,sync:12,debug:0,burst:7,led:1,wide:1,settle:2,diff:0,irqm:0

```
当详细输出级别为 2 时，驱动会打印其实际设置。你可以尝试
"ncr53c8xx=verb:2" 来获取驱动的"静态"设置，或者在你的启动设置命令中
加上 "verb:2" 以查看驱动正在使用的实际设置。

### 10.4 PCI 配置修复启动选项


pcifix:<option bits>

Available option bits:

    ===      =====================================================
    0x1      Set PCI cache-line size register if not set.
    0x2      Set write and invalidate bit in PCI command register.
    ===      =====================================================

使用 'pcifix:3' 以允许驱动修复这两项 PCI 特性。

这些选项仅适用于新的 SYMBIOS 芯片 810A、825A、860、875 和 895，并且
仅支持 Pentium 与 486 级处理器。近期的 SYMBIOS 53C8XX SCSI 处理器能够
使用 PCI 读多个（read multiple）与 PCI 写并使无效（write and invalidate）
命令。这些特性要求芯片的 PCI 配置空间中正确设置了缓存行大小寄存器。
另一方面，芯片只有在 PCI 命令寄存器中相应位被置为 1 时才会使用 PCI 写
并使无效命令。

并非所有 PCI BIOS 都会设置 53C8XX 芯片 PCI 配置空间中的 PCI 缓存行
寄存器与 PCI 写并使无效位。优化的 PCI 访问在某些 PCI/内存控制器上可能会
出问题，或在某些 PCI 板卡上产生问题。

此修复在我的旧系统上运行完美。
（主板 Triton HX / 53C875 / 53C810A）
我使用这些选项需自担风险，如果你决定使用它们也是如此。

### 10.5 串行 NVRAM 支持启动选项


=======     =========================================
nvram:n     do not look for serial NVRAM
nvram:y     test controllers for onboard serial NVRAM
=======     =========================================

此选项也可以以十六进制值形式输入，用于控制驱动将从 NVRAM 获取哪些
信息、忽略哪些信息。
详情参见"17. 串行 NVRAM 支持"。

启用此选项时，驱动会尝试检测所有使用串行 NVRAM 的板卡。该存储器用于
保存用户设置的参数。

驱动能够从 NVRAM 获取的参数取决于所使用的数据格式，如下所示：

+-------------------------------+------------------+--------------+
|                               |Tekram format     |Symbios format|
+-------------------------------+------------------+--------------+
|General and host parameters    |                  |              |
+-------------------------------+------------------+--------------+
|  * Boot order                 |        N         |       Y      |
+-------------------------------+------------------+--------------+
|  * Host SCSI ID               |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * SCSI parity checking       |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Verbose boot messages      |        N         |       Y      |
+-------------------------------+------------------+--------------+
|SCSI devices parameters                                          |
+-------------------------------+------------------+--------------+
|  * Synchronous transfer speed |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Wide 16 / Narrow           |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Tagged Command Queuing     |        Y         |       Y      |
|    enabled                    |                  |              |
+-------------------------------+------------------+--------------+
|  * Disconnections enabled     |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Scan at boot time          |        N         |       Y      |
+-------------------------------+------------------+--------------+

为了加快系统启动，对于每个配置为不带"启动时扫描"（scan at boot time）
选项的设备，驱动会在收到的该设备的第一条 TEST UNIT READY 命令上强制
返回一个错误。

某些 SDMS BIOS 修订版似乎无法与非常快的硬盘一起干净地启动。在这种情况下，
你无法用优化后的参数值来配置 NVRAM。

'nvram' 启动选项可以以十六进制形式输入，以忽略 NVRAM 中配置的某些选项，
如下所示：

mvram=<bits options>

      ====   =================================================================
      0x01   look for NVRAM  (equivalent to nvram=y)
      0x02   ignore NVRAM "Synchronous negotiation" parameters for all devices
      0x04   ignore NVRAM "Wide negotiation"  parameter for all devices
      0x08   ignore NVRAM "Scan at boot time" parameter for all devices
      0x80   also attach controllers set to OFF in the NVRAM (sym53c8xx only)
      ====   =================================================================

选项 0x80 仅由 sym53c8xx 驱动支持，默认禁用。结果是，默认情况下（未设置
该选项），sym53c8xx 驱动不会挂载在 NVRAM 中设为 OFF 的控制器。

ncr53c8xx 始终尝试挂载所有控制器。选项 0x80 没有被加入 ncr53c8xx 驱动，
因为据报告它会让长期使用该驱动的用户感到困惑。如果你希望在 Linux 启动时
不让 ncr53c8xx 驱动挂载某个控制器，必须使用 'excl' 驱动启动选项。

##### 10.6 SCSI 总线检查启动选项。


当此选项被设为非零值时，驱动会在断言 SCSI RESET 线 100 微秒后检查 SCSI
信号线的逻辑状态。驱动只是读取 SCSI 信号线，并检查除 RESET 外所有信号线
读取结果均为 FALSE。由于 SCSI 设备在 SCSI RESET 被断言后最多 800 纳秒内
会释放总线，任何信号为 TRUE 都可能表明存在 SCSI 总线问题。遗憾的是，
以下常见的 SCSI 总线问题无法被检测到：

- 只安装了一个终结器（terminator）。
- 终结器位置错误。
- 终结器质量差。

另一方面，错误的布线、损坏的设备、不符合规范的设备……都可能导致驱动
读取时某个 SCSI 信号错误。

##### 10.7 IMMEDIATE ARBITRATION 启动选项


此选项仅由 SYM53C8XX 驱动支持（不由 NCR53C8XX 支持）。

SYMBIOS 53C8XX 芯片能够在检测到预期的断开连接（BUS FREE 相位）后
立即对 SCSI 总线进行仲裁。要使该过程启动，当芯片连接到 SCSI 总线时，
SCNTL1 IO 寄存器的位 1 必须被置位。

当为当前连接启用了此特性后，如果只有优先级较低的设备在竞争 SCSI 总线，
芯片有十足的把握赢得仲裁。顺便说一句，当芯片使用 SCSI ID 7 时，它必将
赢得下一次 SCSI 总线仲裁。

由于无法知道有哪些设备正试图对总线进行仲裁，使用此特性可能极不公平。
因此，不建议你启用它，或者最多仅为芯片在上一次仲裁中失败的情况
（启动选项 'iarb:1'）启用此特性。

此特性具有以下优点：

a) 允许 ID 为 7 的发起者在需要时赢得仲裁。
b) 将至少 4 微秒的仲裁时间与处理当前连接结束并开始下一个任务的
   SCRIPTS 执行重叠。

嗯……但是（a）可能只是阻止其他设备重新选择发起者，并延迟数据传输或
状态/完成；而（b）如果 SCRIPTS 执行持续时间超过 4 微秒，可能只是
浪费 SCSI 总线带宽。

使用 IARB 需要在编译时定义 SCSI_NCR_IARB_SUPPORT 选项，并在启动时将
'iarb' 启动选项设为非零值。它对实际工作并不是那么有用，但可用于
给 SCSI 设备施加压力，或用于某些能从中获益的应用。顺便说一句，如果
你在高 IO 负载下使用 IARB 时遇到诸如"意外的断开连接"、"错误的重新选择"
等问题，你不应该感到惊讶，因为同时强行喂入任何东西又堵住它的后路
是不可能长期奏效的。 :-))

## 11. ncr53c8xx.h 头文件中的一些常量与标志


其中一些由配置参数定义。要更改其他 "defines"，你必须编辑头文件。
只有在你知道自己在做什么时才这样做。

SCSI_NCR_SETUP_SPECIAL_FEATURES	(default: defined)
	如果定义，驱动将根据芯片与修订 ID 启用一些特殊特性。

        对于 810A、860、825A、875 和 895 这些 SCSI 芯片，此选项启用
	在 SCSI 传输处理期间减轻 PCI 总线与内存访问负载的特性：突发
	取指（burst op-code fetch）、读多个（read multiple）、读行
	（read line）、预取（prefetch）、缓存行（cache line）、写并使无效
	（write and invalidate）、突发 128（仅 875）、大 DMA FIFO
	（仅 875）、偏移 16（仅 875）。

```
	ncr53c8xx=specf:n

```
SCSI_NCR_IOMAPPED		(default: not defined)
	如果定义，强制使用普通 I/O。

SCSI_NCR_SHARE_IRQ		(default: defined)
	如果定义，请求共享 IRQ。

SCSI_NCR_MAX_TAGS		(default: 8)
	到某个设备的并发标记命令的最大数量。

	可通过 "settags <target> <maxtags>" 更改

SCSI_NCR_SETUP_DEFAULT_SYNC     (default: 50)
	驱动在启动时用于同步协商的传输周期因子。0 表示异步。

	可通过 "setsync <target> <period factor>" 更改

SCSI_NCR_SETUP_DEFAULT_TAGS     (default: 8)
	到某个设备的并发标记命令的默认数量。

	< 1 表示启动时禁用标记命令队列。

SCSI_NCR_ALWAYS_SIMPLE_TAG	(default: defined)
	对读写命令使用 SIMPLE TAG。

	可通过 "setorder <ordered|simple|default>" 更改

SCSI_NCR_SETUP_DISCONNECTION	(default: defined)
	如果定义，允许目标断开连接。

SCSI_NCR_SETUP_FORCE_SYNC_NEGO	(default: not defined)
	如果定义，对所有 SCSI-2 设备尝试同步协商。

	可通过 "setsync <target> <period>" 更改

SCSI_NCR_SETUP_MASTER_PARITY	(default: defined)
	如果定义，启用主设备奇偶校验。

SCSI_NCR_SETUP_SCSI_PARITY	(default: defined)
	如果定义，启用 SCSI 奇偶校验。

SCSI_NCR_PROFILE_SUPPORT	(default: not defined)
	如果定义，收集性能剖析信息。

SCSI_NCR_MAX_SCATTER		(default: 128)
	驱动 ccb 的分散列表大小。

SCSI_NCR_MAX_TARGET		(default: 16)
	每个主机的最大目标数量。

SCSI_NCR_MAX_HOST		(default: 2)
	主机控制器的最大数量。

SCSI_NCR_SETTLE_TIME		(default: 2)
	驱动在复位后等待的秒数。

SCSI_NCR_TIMEOUT_ALERT		(default: 3)
	如果一条挂起的命令将在该秒数之后超时，下一条命令将使用
	有序标签（ordered tag）。

	避免无序标记命令的超时。

SCSI_NCR_CAN_QUEUE		(default: 7*SCSI_NCR_MAX_TAGS)
	可排队到某个主机的最大命令数量。

SCSI_NCR_CMD_PER_LUN		(default: SCSI_NCR_MAX_TAGS)
	排队到某个主机的某个设备的命令最大数量。

SCSI_NCR_SG_TABLESIZE		(default: SCSI_NCR_MAX_SCATTER-1)
	Linux 分散/聚集列表的最大大小。

SCSI_NCR_MAX_LUN	(default: 8)
	每个目标的最大 LUN 数量。

## 12. 安装


该驱动是 Linux 内核发行版的一部分。驱动文件位于内核源代码树的
"drivers/scsi" 子目录中。

```
	README.ncr53c8xx	: this file
	ChangeLog.ncr53c8xx	: change log
	ncr53c8xx.h		: definitions
	ncr53c8xx.c		: the driver code

```
新版驱动会单独提供，以便在将其纳入 Linux 内核发行版之前测试变更与
新特性。以下 URL 提供了最新可用补丁的信息：

      ftp://ftp.tux.org/pub/people/gerard-roudier/README

## 13. 与体系结构相关的特性


<尚未编写>

## 14. 已知问题


### 14.1 使用 Iomega Jaz 设备的标记命令


我没有试用过此设备，但有人向我报告了以下情况：此设备具备标记命令
队列能力。然而在自旋启动（spinning up）期间，它会拒绝标记命令。这种
行为符合 SCSI-2 规范 6.8.2 节。驱动在这种情况下的当前行为并不令人满意。
因此，不要为能够自旋降速（spin down）的设备启用标记命令队列。另一个
可能出现的问题是超时。避免超时的唯一方法似乎是编辑
linux/drivers/scsi/sd.c 并增大当前的超时值。

### 14.2 添加另一控制器时设备名发生变化


当你向一个已经拥有一块或多块该系列控制器的系统添加一块新的
NCR53C8XX 芯片控制器时，驱动向内核注册它们的顺序可能会导致因设备名
变化而产生的问题。当至少有一块控制器使用 NvRAM 时，SDMS BIOS 4 版允许
你定义 BIOS 扫描 SCSI 板卡的顺序。如果设置了 NvRAM 检测选项，驱动会
根据 BIOS 信息挂载控制器。

如果你的控制器没有 NvRAM，你可以：

- 在启动命令行中要求驱动以相反顺序探测芯片 ID：ncr53c8xx=revprob:y
- 对 fstab 做适当的修改。
- 使用 Eric Youngdale 的 'scsidev' 工具。

### 14.3 在 WIDE SCSI 控制器上仅使用 8 位设备


当只有 8 位窄型（NARROW）设备连接到 16 位宽型（WIDE）SCSI 控制器时，
你必须确保 SCSI 总线宽型部分的信号线被上拉。这可以通过启用 SCSI
控制器卡的宽型终结器（WIDE TERMINATOR）部分来实现。

TYAN 1365 文档 1.2 版关于此类设置的描述不正确。（第 10 页，图 3.3）。

### 14.4 内存写并使无效期间可能出现的数据损坏


此问题在 SYMBIOS DEL 397、部件号 69-039241、条目 4 中有描述。

在某些复杂情况下，修订号 <= 3 的 53C875 芯片可能会从一个未与缓存行
对齐的 4 个 DWORD 边界开始 PCI 写并使无效命令。这只有在缓存行大小为
8 个 DWORD 或更大时才可能发生。Pentium 系统使用 8 个 DWORD 的缓存行大小，
因此受此芯片缺陷影响，而 i486 系统使用 4 个 DWORD 的缓存行大小，不受
影响。

当这种情况发生时，芯片可能在只填充了传输所涉及的最后一个缓存行的
部分内容后就完成了写并使无效命令，从而让该缓存行的其余部分发生数据
损坏。

不使用写并使无效显然可以规避此芯片缺陷，因此现在它是驱动的默认设置。
然而，对于像我这样想启用此特性的人，我加入了 SYMBIOS 建议的部分应对
方法。该应对方法在进入 DATA IN 相位时重置寻址逻辑，从而防止该缺陷在
相位的第一次 SCSI MOVE 时被触发。根据以下分析，该应对方法应当足够：

驱动内部唯一大于 8 个 DWORD 且由 SCRIPTS 处理器移动的数据结构是
包含 SCSI 传输上下文的"CCB 头"（CCB header）。该数据结构按 8 个 DWORD
边界（Pentium 缓存行大小）对齐，因此在 Pentium 系统上不受此芯片缺陷
影响。

但是，当使用未与缓存行对齐的 4 个 DWORD 缓冲区执行 SCSI 读命令时，
可能满足该缺陷的条件。在 Linux 下使用分散/聚集列表时不会发生这种情况，
因为它们只引用对齐良好的系统缓冲区。因此，在 Linux 下，仅当未使用
分散/聚集列表，且在相位失配后重新进入 SCSI DATA IN 相位时，才可能需要
应对方法。

## 15. SCSI 问题排查


### 15.1 问题追踪


大多数 SCSI 问题源于不符合规范的 SCSI 总线或有缺陷的设备。如果你不幸
遇到了 SCSI 问题，可以检查以下事项：

- SCSI 总线电缆
- SCSI 链两端处的终结器
- Linux 的 syslog 消息（其中一些可能会对你有帮助）

如果你找不到问题的根源，可以将驱动配置为不启用任何特性。

- 仅异步数据传输
- 禁用标记命令
- 不允许断开连接

现在，如果你的 SCSI 总线正常，你的系统很有机会在此安全配置下工作，
但性能不会是最优的。

如果仍然失败，则你可以将你的问题描述发送到相应的邮件列表或新闻组。
给我发一份副本，以确保我能收到。显然，驱动代码中可能存在 bug。

     我的电子邮件地址：Gerard Roudier <groudier@free.fr>

如果你在 SCSI 总线上使用了多个设备，允许断开连接很重要，但常常会
对有缺陷的设备造成问题。同步数据传输可提高像硬盘这样快速设备的吞吐量。
拥有大缓存的优质 SCSI 硬盘能从标记命令队列中获益。

尝试用控制命令一次启用一个特性。例如：

```
    echo "setsync all 25" >/proc/scsi/ncr53c8xx/0

```
将为所有目标启用快速同步数据传输协商。

```
    echo "setflag 3" >/proc/scsi/ncr53c8xx/0

```
将重置目标 3 的标志（no_disc），从而允许它断开 SCSI 总线。

```
    echo "settags 3 8" >/proc/scsi/ncr53c8xx/0

```
如果该设备支持，将为目标 3 启用标记命令队列。

一旦你找到了导致问题的设备与特性，只需为该设备禁用该特性即可。

### 15.2 理解硬件错误报告


当驱动检测到意外的错误条件时，它可能会显示

```
    sym53c876-0:1: ERROR (0:48) (1-21-65) (f/95) @ (script 7c0:19000000).
    sym53c876-0: script cmd = 19000000
    sym53c876-0: regdump: da 10 80 95 47 0f 01 07 75 01 81 21 80 01 09 00.

```
此类消息中的某些字段可以帮助你理解原因

```
    sym53c876-0:1: ERROR (0:48) (1-21-65) (f/95) @ (script 7c0:19000000).
    ............A.........B.C....D.E..F....G.H.......I.....J...K.......

```
字段 A：目标编号。
  发生错误时控制器正在与之通信的设备的 SCSI ID。

字段 B：DSTAT IO 寄存器（DMA 状态）
  ========   =============================================================
  Bit 0x40   MDPE Master Data Parity Error
             在 PCI 总线上检测到的数据奇偶错误。
  Bit 0x20   BF   Bus Fault
             检测到的 PCI 总线故障条件。
  Bit 0x01   IID  Illegal Instruction Detected
             当芯片在某些使指令非法的条件下检测到非法指令格式时由芯片置位。
  Bit 0x80   DFE Dma Fifo Empty
             纯状态位，不表示错误。
  ========   =============================================================

  如果报告的 DSTAT 值包含 MDPE (0x40) 与 BF (0x20) 的组合，则原因
  很可能是 PCI 总线问题。

字段 C：SIST IO 寄存器（SCSI 中断状态）
  ========   ==================================================================
  Bit 0x08   SGE  SCSI GROSS ERROR
             表示芯片在 SCSI 总线上检测到了严重的错误条件，导致 SCSI 协议
             无法正常工作。
  Bit 0x04   UDC  Unexpected Disconnection
             表示设备在芯片未预期的情况下释放了 SCSI 总线。设备可能如此
             表现，以向 SCSI 发起者指示发生了无法用 SCSI 协议报告的
             错误条件。
  Bit 0x02   RST  SCSI BUS Reset
             通常 SCSI 目标不会复位 SCSI 总线，尽管总线上的任何设备都
             可以在任何时候复位它。
  Bit 0x01   PAR  Parity
             检测到的 SCSI 奇偶错误。
  ========   ==================================================================

  在有故障的 SCSI 总线上，芯片可能检测到 SGE (0x08)、UDC (0x04) 与
  PAR (0x01) 中的任何错误条件。如果你的 SCSI 系统有时遇到此类错误条件，
  尤其是 SCSI GROSS ERROR，则 SCSI 总线问题很可能是这些错误的根源。

对于字段 D、E、F、G 和 H，你可以查看 sym53c8xx_defs.h 文件，其中包含
对 IO 寄存器位的一些简要注释。

字段 D：SOCL  Scsi Output Control Latch
          该寄存器反映芯片想要驱动或与之比较的 SCSI 控制线的状态。

字段 E：SBCL  Scsi Bus Control Lines
          SCSI 总线上控制线的实际值。

字段 F：SBDL  Scsi Bus Data Lines
          SCSI 总线上数据线的实际值。

字段 G：SXFER  SCSI Transfer
          包含用于输出的同步周期设置以及当前同步偏移（偏移 0 表示异步）。

字段 H：SCNTL3 Scsi Control Register 3
          包含异步与同步数据传输的时序设置值。

理解字段 I、J、K 与转储需要对 SCSI 标准、芯片核心功能以及驱动内部
数据结构有良好的了解。除非你想帮忙维护驱动代码，否则不需要解码并
理解它们。

## 16. 同步传输协商表


下表是通过调用驱动用于同步协商时序计算与芯片设置的例程创建的。
第一张表对应于使用 80 MHz 时钟与 5 个时钟分频器的 Ultra 芯片 53875
与 53C860。第二张表是通过将 SCSI 时钟设为 40 MHz 并使用 4 个时钟分频器
计算的，因此适用于快速 SCSI-2 模式下的所有 NCR53C8XX 芯片。

周期以纳秒为单位，速度以兆传输/秒为单位。1 兆传输/秒在 8 位 SCSI 下
表示 1 MB/s，在 Wide16 SCSI 下表示 2 MB/s。

16.1 53C895、53C875 与 53C860 SCSI 控制器的同步时序

+-----------------------------+--------+-------+--------------+
|Negotiated                   |NCR settings    |              |
+-------+--------+------------+--------+-------+              |
|Factor |Period  |Speed       |Period  |Speed  |              |
+-------+--------+------------+--------+-------+--------------+
|10     | 25     |40.000      | 25     |40.000 | (53C895 only)|
+-------+--------+------------+--------+-------+--------------+
|11     | 30.2   |33.112      | 31.25  |32.000 | (53C895 only)|
+-------+--------+------------+--------+-------+--------------+
|12     | 50     |20.000      | 50     |20.000 |              |
+-------+--------+------------+--------+-------+--------------+
|13     | 52     |19.230      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|14     | 56     |17.857      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|15     | 60     |16.666      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|16     | 64     |15.625      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|17     | 68     |14.705      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|18     | 72     |13.888      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|19     | 76     |13.157      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|20     | 80     |12.500      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|21     | 84     |11.904      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|22     | 88     |11.363      | 93     |10.666 |              |
+-------+--------+------------+--------+-------+--------------+
|23     | 92     |10.869      | 93     |10.666 |              |
+-------+--------+------------+--------+-------+--------------+
|24     | 96     |10.416      |100     |10.000 |              |
+-------+--------+------------+--------+-------+--------------+
|25     |100     |10.000      |100     |10.000 |              |
+-------+--------+------------+--------+-------+--------------+
|26     |104     | 9.615      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|27     |108     | 9.259      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|28     |112     | 8.928      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|29     |116     | 8.620      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|30     |120     | 8.333      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|31     |124     | 8.064      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|32     |128     | 7.812      |131     | 7.619 |              |
+-------+--------+------------+--------+-------+--------------+
|33     |132     | 7.575      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|34     |136     | 7.352      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|35     |140     | 7.142      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|36     |144     | 6.944      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|37     |148     | 6.756      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|38     |152     | 6.578      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|39     |156     | 6.410      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|40     |160     | 6.250      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|41     |164     | 6.097      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|42     |168     | 5.952      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|43     |172     | 5.813      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|44     |176     | 5.681      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|45     |180     | 5.555      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|46     |184     | 5.434      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|47     |188     | 5.319      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+
|48     |192     | 5.208      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+
|49     |196     | 5.102      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+

16.2 快速 SCSI-2 53C8XX 控制器的同步时序

+-----------------------------+----------------+
|Negotiated                   |NCR settings    |
+-------+--------+------------+--------+-------+
|Factor |Period  |Speed       |Period  |Speed  |
+-------+--------+------------+--------+-------+
|25     |100     |10.000      |100     |10.000 |
+-------+--------+------------+--------+-------+
|26     |104     |9.615       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|27     |108     |9.259       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|28     |112     |8.928       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|29     |116     |8.620       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|30     |120     |8.333       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|31     |124     |8.064       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|32     |128     |7.812       |131     | 7.619 |
+-------+--------+------------+--------+-------+
|33     |132     |7.575       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|34     |136     |7.352       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|35     |140     |7.142       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|36     |144     |6.944       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|37     |148     |6.756       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|38     |152     |6.578       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|39     |156     |6.410       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|40     |160     |6.250       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|41     |164     |6.097       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|42     |168     |5.952       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|43     |172     |5.813       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|44     |176     |5.681       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|45     |180     |5.555       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|46     |184     |5.434       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|47     |188     |5.319       |200     | 5.000 |
+-------+--------+------------+--------+-------+
|48     |192     |5.208       |200     | 5.000 |
+-------+--------+------------+--------+-------+
|49     |196     |5.102       |200     | 5.000 |
+-------+--------+------------+--------+-------+

## 17. 串行 NVRAM


（由 Richard Waltham 添加：dormouse@farsrobt.demon.co.uk）

### 17.1 特性


启用串行 NVRAM 支持后，可以检测 Symbios 以及部分 Symbios 兼容主机
适配器（还有 Tekram 板卡）上包含的串行 NVRAM。串行 NVRAM 被 Symbios
与 Tekram 用来保存主机适配器及其所连驱动器的设置参数。

Symbios NVRAM 还保存了拥有多个主机适配器的系统中主机适配器的启动
顺序数据。这样可以改变检测主机适配器时扫描各卡以寻找驱动器的顺序。

目前，使用"反向探测"只能在一定程度上做到这一点，而且它只会改变
不同类型卡的检测顺序。"NVRAM 启动顺序"设置既能做到这一点，也能改变
同类卡的扫描顺序，这是"反向探测"做不到的。

使用 Symbios 芯片的 Tekram 板卡（DC390W/F/U）带有 NVRAM，会被检测出来，
并用于区分 Symbios 兼容与 Tekram 主机适配器。如果设置了
CONFIG_SCSI_53C8XX_SYMBIOS_COMPAT 配置参数，这用于禁用在 Tekram 板卡上
错误设置的 Symbios 兼容"差分"（diff）设置，从而让 Symbios 卡与 Tekram
卡可以一起使用，Symbios 卡使用其全部特性，包括"差分"支持。（对 Symbios
兼容卡使用"LED 引脚"（led pin）支持可以保持启用。它对 Tekram 主机
适配器没有实际用处，但也不会造成问题。）

### 17.2 Symbios NVRAM 布局


```

    00 00
    64 01
    8e 0b

    00 30 00 00 00 00 07 00 00 00 00 00 00 00 07 04 10 04 00 00

    04 00 0f 00 00 10 00 50 00 00 01 00 00 62
    04 00 03 00 00 10 00 58 00 00 01 00 00 63
    04 00 01 00 00 10 00 48 00 00 01 00 00 61
    00 00 00 00 00 00 00 00 00 00 00 00 00 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    fe fe
    00 00
    00 00

```
NVRAM 布局详情

=============  ================
NVRAM Address
=============  ================
0x000-0x0ff    not used
0x100-0x26f    initialised data
0x270-0x7ff    not used
=============  ================

```

        header  -   6 bytes,
        data    - 356 bytes (checksum is byte sum of this data)
        trailer -   6 bytes
                  ---
        total     368 bytes

```

        controller set up  -  20 bytes
        boot configuration -  56 bytes (4x14 bytes)
        device set up      - 128 bytes (16x8 bytes)
        unused (spare?)    - 152 bytes (19x8 bytes)
                             ---
        total                356 bytes

```

    00 00   - ?? start marker
    64 01   - byte count (lsb/msb excludes header/trailer)
    8e 0b   - checksum (lsb/msb excludes header/trailer)

```

    00 30 00 00 00 00 07 00 00 00 00 00 00 00 07 04 10 04 00 00
		    |     |           |     |
		    |     |           |      -- host ID
		    |     |           |
		    |     |            --Removable Media Support
		    |     |               0x00 = none
		    |     |               0x01 = Bootable Device
		    |     |               0x02 = All with Media
		    |     |
		    |      --flag bits 2
		    |        0x00000001= scan order hi->low
		    |            (default 0x00 - scan low->hi)
			--flag bits 1
			0x00000001 scam enable
			0x00000010 parity enable
			0x00000100 verbose boot msgs

```
剩余字节未知——在我的当前设置中，对于任何控制器它们似乎都不会改变。

53c810a 与 53c875 NVRAM 的默认设置相同
（可移动介质自 Symbios BIOS 4.09 版起添加）

启动配置

```

    04 00 0f 00 00 10 00 50 00 00 01 00 00 62 -- 1st controller
    04 00 03 00 00 10 00 58 00 00 01 00 00 63    2nd controller
    04 00 01 00 00 10 00 48 00 00 01 00 00 61    3rd controller
    00 00 00 00 00 00 00 00 00 00 00 00 00 00    4th controller
	|  |  |  |     |        |     |  |
	|  |  |  |     |        |      ---- PCI io port adr
	|  |  |  |     |         --0x01 init/scan at boot time
	|  |  |  |      --PCI device/function number (0xdddddfff)
	|  |   ----- ?? PCI vendor ID (lsb/msb)
	    ----PCI device ID (lsb/msb)

    ?? use of this data is a guess but seems reasonable

```
剩余字节未知——在我的当前设置中它们似乎不会改变

### default set up is identical for 53c810a and 53c875 NVRAM


```

    0f 00 08 08 64 00 0a 00 - id 0
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00 - id 15
    |     |  |  |     |  |
    |     |  |  |      ----timeout (lsb/msb)
    |     |  |   --synch period (0x?? 40 Mtrans/sec- fast 40) (probably 0x28)
    |     |  |                  (0x30 20 Mtrans/sec- fast 20)
    |     |  |                  (0x64 10 Mtrans/sec- fast )
    |     |  |                  (0xc8  5 Mtrans/sec)
    |     |  |                  (0x00  asynchronous)
    |     |   -- ?? max sync offset (0x08 in NVRAM on 53c810a)
    |     |                         (0x10 in NVRAM on 53c875)
    |      --device bus width (0x08 narrow)
    |                         (0x10 16 bit wide)
    --flag bits
	0x00000001 - disconnect enabled
	0x00000010 - scan at boot time
	0x00000100 - scan luns
	0x00001000 - queue tags enabled

```
剩余字节未知——在我的当前设置中它们似乎不会改变

?? 此数据的用途是猜测，但似乎合理
（但它可能是最大总线宽度）

53c810a NVRAM 的默认设置
53c875 NVRAM 的默认设置

```
    - bus width     - 0x10
                                - sync offset ? - 0x10
                                - sync period   - 0x30

?? spare device space (32 bit bus ??)

```

    00 00 00 00 00 00 00 00  (19x8bytes)
    .
    .
    00 00 00 00 00 00 00 00

```

### default set up is identical for 53c810a and 53c875 NVRAM


```

    fe fe   - ? end marker ?
    00 00
    00 00

```

### default set up is identical for 53c810a and 53c875 NVRAM


### 17.3 Tekram NVRAM 布局


nvram 64x16 (1024 bit)

```

    Drive ID 0-15 (addr 0x0yyyy0 = device setup, yyyy = ID)
		(addr 0x0yyyy1 = 0x0000)

	x x x x  x x x x  x x x x  x x x x
		| | |      | |  | | | |
		| | |      | |  | | |  ----- parity check   0 - off
		| | |      | |  | | |                       1 - on
		| | |      | |  | | |
		| | |      | |  | |  ------- sync neg       0 - off
		| | |      | |  | |                         1 - on
		| | |      | |  | |
		| | |      | |  |  --------- disconnect     0 - off
		| | |      | |  |                           1 - on
		| | |      | |  |
		| | |      | |   ----------- start cmd      0 - off
		| | |      | |                              1 - on
		| | |      | |
		| | |      |  -------------- tagged cmds    0 - off
		| | |      |                                1 - on
		| | |      |
		| | |       ---------------- wide neg       0 - off
		| | |                                       1 - on
		| | |
		    --------------------------- sync rate      0 - 10.0 Mtrans/sec
							    1 -  8.0
							    2 -  6.6
							    3 -  5.7
							    4 -  5.0
							    5 -  4.0
							    6 -  3.0
							    7 -  2.0
							    7 -  2.0
							    8 - 20.0
							    9 - 16.7
							    a - 13.9
							    b - 11.9

```
全局设置

```

    x x x x  x x x x  x x x x  x x x x
    | | | |  | | | |           | | | |
    | | | |  | | | |            ----------- host ID    0x00 - 0x0f
    | | | |  | | | |
    | | | |  | | |  ----------------------- support for    0 - off
    | | | |  | | |                          > 2 drives     1 - on
    | | | |  | | |
    | | | |  | |  ------------------------- support drives 0 - off
    | | | |  | |                            > 1Gbytes      1 - on
    | | | |  | |
    | | | |  |  --------------------------- bus reset on   0 - off
    | | | |  |                                power on     1 - on
    | | | |  |
    | | | |   ----------------------------- active neg     0 - off
    | | | |                                                1 - on
    | | | |
    | | |  -------------------------------- imm seek       0 - off
    | | |                                                  1 - on
    | | |
    | |  ---------------------------------- scan luns      0 - off
    | |                                                    1 - on
    | |
     -------------------------------------- removable      0 - disable
                                            as BIOS dev    1 - boot device
                                                           2 - all

```

```

    x x x x  x x x x  x x x x  x x x x
               | | |             | | |
               | | |              --------- boot delay     0 -   3 sec
               | | |                                       1 -   5
               | | |                                       2 -  10
               | | |                                       3 -  20
               | | |                                       4 -  30
               | | |                                       5 -  60
               | | |                                       6 - 120
               | | |
                --------------------------- max tag cmds   0 -  2
                                                           1 -  4
                                                           2 -  8
                                                           3 - 16
                                                           4 - 32

```

```

    x x x x  x x x x  x x x x  x x x x
                                     |
                                      ----- F2/F6 enable   0 - off ???
                                                           1 - on  ???

```
校验和（地址 0x111111）

校验和 = 0x1234 - (地址 0-63 的求和)

----------------------------------------------------------------------------

```

    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000

    0x0f07 0x0400 0x0001 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0xfbbc


```
## 18. 大端支持


PCI 局部总线主要是为 x86 体系结构设计。因此，PCI 设备通常期望使用
小端（little endian）字节序的 DWORD。

### 18.1 大端 CPU


为了在大端（Big Endian）体系结构上支持 NCR 芯片，驱动必须在每次
需要时执行字节重排。此特性由 Cort <cort@cs.nmt.edu> 添加到驱动中，
在驱动版本 2.5 及更高版本中可用。目前大端支持仅在 Linux/PPC
（PowerPC）上测试过。

### 18.2 运行于大端模式的 NCR 芯片


在 SYMBIOS 文档中可以看到，某些芯片支持一种特殊的大端模式，理论上
包括：53C815、53C825A、53C875、53C875N、53C895。此工作模式不是由
软件选择的，而是需要将名为 BigLit 的引脚上拉。使用此模式，当驱动运行
于大端 CPU 上时，应当可以避免大部分字节重排。驱动版本 2.5 在理论上
也已为此特性做好准备。
