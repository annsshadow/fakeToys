锘?## SYM-2 椹卞姩

Written by Gerard Roudier <groudier@free.fr>

21 Rue Carnot

95170 DEUIL LA BARRE - FRANCE

Updated by Matthew Wilcox <matthew@wil.cx>

2004-10-09


   1. 简   2. 支持的芯片与 SCSI 特   3. 本驱动在新款芯片上的优势
         3.1 优化过的 SCSI SCRIPTS
         3.2 SYM53C896 引入的新特   4. 内存映射 I/O 与普I/O
   5. 标记命令队列
   6. 奇偶校验
   7. 性能分析信息
   8. 控制命令
         8.1  设置最小同步周         8.2  设置总线宽度
         8.3  设置并发标记命令的最大数         8.4  设置调试模式
         8.5  设置标志（no_disc         8.6  设置详细级别
         8.7  复位目标的所有逻辑单元
         8.8  中止目标所有逻辑单元的全部任   9. 配置参数
   10. 启动设置命令
         10.1 语法
         10.2 可用参数
                10.2.1  默认标记命令数量
                10.2.2  最大突发长                10.2.3  LED 支持
                10.2.4  差分模式
                10.2.5  IRQ 模式
                10.2.6  检SCSI 总线
                10.2.7  为主机建议一个默SCSI ID
                10.2.8  详细级别
                10.2.9  调试模式
                10.2.10 稳定延迟
                10.2.11 串行 NVRAM
                10.2.12 排除某个主机不被挂载
         10.3 从旧选项转换
         10.4 SCSI 总线检查的启动选项
   11. SCSI 问题排查
         15.1 问题追踪
         15.2 理解硬件错误报告
   12. 串行 NVRAM 支持（Richard Waltham 编写         17.1 特         17.2 Symbios NVRAM 布局
         17.3 Tekram  NVRAM 布局


## 1. 简

本驱动支持整SYM53C8XX 系列PCI-SCSI 控制器它也支持那些基于 SYM53C8XX SCRIPTS 语言LSI53C10XX PCI-SCSI
控制器的子集
它取代了 sym53c8xx+ncr53c8xx 驱动包，并与 FreeBSD SYM-2 驱动共享其核心代码使本驱动能在 Linux 下工作的“胶水”代码包含在两个名为 sym_glue.h sym_glue.c
的文件中。驱动的其他源文件被设计为不依赖于驱动所运行的操作系统
本驱动的历史可概括如下：

1993：由以下人员386bsd FreeBSD 编写 ncr 驱动
          - Wolfgang Stanglmeier        <wolf@cologne.de>
          - Stefan Esser                <se@mi.Uni-Koeln.de>

1996：将 ncr 驱动移植Linux-1.2.13 并重命名ncr53c8xx
          - Gerard Roudier

1998：基LOAD/STORE 指令、为 Linux 编写的新 sym53c8xx 驱动      增加了对 896 的完整支持，但放弃了对早NCR 设备的支持
          - Gerard Roudier

1999：将 sym53c8xx 驱动移植FreeBSD，并支持 LSI53C1010       33 MHz 66MHz Ultra-3 控制器。新驱动命名'sym'
          - Gerard Roudier

2000：为 FreeBSD 'sym' 驱动增加对早NCR 设备的支持      将驱动拆分为多个源文件，并把 OS 胶水代码与可在不      操作系统之间共享的核心代码分离开来。为 Linux 编写了胶水代码
          - Gerard Roudier

2004：移FreeBSD 兼容代码。移除对 2.6 之前版本 Linux 的支持      开始使Linux 自身提供的设施
README 文件针对该驱动的 Linux 版本。在 FreeBSD 下，驱动文档sym.8 手册页
关于新款芯片的信息可LSILOGIC Web 服务器上获取
          http://www.lsilogic.com/

SCSI 标准文档可在 T10 站点获取
          http://www.t10.org/

Eric Youngdale 编写的实SCSI 工具包含在大多数 Linux 发行版中
   ============ ==========================
   scsiinfo     command line tool
   scsi-config  TCL/Tk tool using scsiinfo
   ============ ==========================

## 2. 支持的芯片与 SCSI 特

以下特性对所有芯片均提供支持
 - 同步协商
 - 断开连接
 - 标记命令队列
 - SCSI 奇偶校验
 - PCI 主设备奇偶校
其他特性取决于芯片能力
该驱动对支持 LOAD/STORE 的设备特别使用了优化过的 SCRIPTS并对支持相应特性的设备SCRIPTS 处理 PHASE MISMATCH（相位失配）
下表展示了该芯片家族的一些特性
+--------+-----------+-----+-----------+------------+------------+---------+
|        |           |     |           |            |Load/store  |Hardware |
|        |On board   |     |           |            |scripts     |phase    |
|Chip    |SDMS BIOS  |Wide |SCSI std.  | Max. sync  |            |mismatch |
+--------+-----------+-----+-----------+------------+------------+---------+
|810     |     N     |  N  | FAST10    | 10 MB/s    |      N     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|810A    |     N     |  N  | FAST10    | 10 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|815     |     Y     |  N  | FAST10    | 10 MB/s    |      N     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|825     |     Y     |  Y  | FAST10    | 20 MB/s    |      N     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|825A    |     Y     |  Y  | FAST10    | 20 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|860     |     N     |  N  | FAST20    | 20 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|875     |     Y     |  Y  | FAST20    | 40 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|875A    |     Y     |  Y  | FAST20    | 40 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|876     |     Y     |  Y  | FAST20    | 40 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|895     |     Y     |  Y  | FAST40    | 80 MB/s    |      Y     |    N    |
+--------+-----------+-----+-----------+------------+------------+---------+
|895A    |     Y     |  Y  | FAST40    | 80 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|896     |     Y     |  Y  | FAST40    | 80 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|897     |     Y     |  Y  | FAST40    | 80 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|1510D   |     Y     |  Y  | FAST40    | 80 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|1010    |     Y     |  Y  | FAST80    |160 MB/s    |      Y     |    Y    |
+--------+-----------+-----+-----------+------------+------------+---------+
|1010_66 |     Y     |  Y  | FAST80    |160 MB/s    |      Y     |    Y    |
|[^1^]_    |           |     |           |            |            |         |
+--------+-----------+-----+-----------+------------+------------+---------+



支持的其它特性摘要：

:Module:                允许加载驱动
:Memory mapped I/O:     提升性能
:Control commands:      proc SCSI 文件系统的写操作
:Debugging information: 写入 syslog（仅限专家）
:Serial NVRAM:          Symbios Tekram 格式

- 分散 / 聚集（Scatter / gather- 共享中断
- 启动设置命令


## 3. 本驱动在新款芯片上的优势


### 3.1 优化过的 SCSI SCRIPTS


81015 825 之外，所有芯片都支持名为 LOAD STORE 的新 SCSI
SCRIPTS 指令，它们能将最1 DWORD IO 寄存器往返内存，速度远快53c7xx 53c8xx 家族所支持MOVE MEMORY 指令
LOAD/STORE 指令支持绝对寻址DSA 相对寻址模式。SCSI SCRIPTS 完全使用 LOAD/STORE 重写了，取代MOVE MEMORY 指令
由于早期芯片缺少 LOAD/STORE SCRIPTS 指令，本驱动另外包含了一套基MEMORY MOVE SCRIPTS，以便支持整SYM53C8XX 芯片家族
### 3.2 SYM53C896 引入的新特

新款芯片（见上）允许SCRIPTS 处理相位失配的上下文（避免了会停SCSI 处理器、直C 代码保存了传输上下文的相位失配中断）
896 1010 芯片支持 64 PCI 事务与寻址，895A 支持 32 PCI
事务64 位寻址。这些芯片的 SCRIPTS 处理器并非真正的 64 位，而是使用
段寄存器来处理第 32-63 位。另一个有趣的特性是，寻址片上 RAMk）的
LOAD/STORE 指令保持在芯片内部进行
## 4. 内存映射 I/O 与普I/O


内存映射 I/O 比普I/O 具有更低的延迟，也是PCI 设备交互推荐的方式。内存映I/O 在大多数硬件配置上工作良好，但一些设欠佳的芯片组可能会破坏这一特性。为此提供了一个使用普I/O 的配选项，但驱动默认使用 MMIO（内存映I/O）
## 5. 标记命令队列


向一个设备一次排队超1 条命令，可以让它根据磁头的实际位置和机械特性进行优化。这一特性还能降低平均命令延迟。要真正利用该特性，
设备必须具备合理的缓存大小（对于 128 KB 或更小的低端硬盘，不要指出现什么奇迹）
一些已知的旧款 SCSI 设备不能正确支持标记命令队列。通常，修复此问题的固件修订版可在各厂商的 Web/FTP 站点获取
我所能说的是，在使用本驱动及其前身时，我从未遇到过标记队列的问题对我而言，在使用标记命令时能正常工作的硬盘有
- IBM S12 0662
- Conner 1080S
- Quantum Atlas I
- Quantum Atlas II
- Seagate Cheetah I
- Quantum Viking II
- IBM DRVS
- Quantum Atlas IV
- Seagate Cheetah II

如果你的控制器带NVRAM，你可以通过用户设置工具按目标配置该特性Tekram 设置程序允许将排队命令的最大数量调整到 32。Symbios 设置程序
则只允许启用或禁用该特性
排队到某个设备的最大并发标记命令数目前默认设置16。该值适用大多SCSI 硬盘。对于大容量 SCSI 硬盘= 2GB、缓>= 512KB、平均寻时间 <= 10 ms），使用更大的值可能会带来更好的性能
本驱动每个设备最多支255 条命令，但使用超64 条通常并不划算除非你在使用非常大的磁盘或磁盘阵列。值得注意的是，大多数近期硬盘似乎不接受超64 条并发命令。因此，使用超过 64 条排队命令很可能
只是在浪费资源
如果你的控制器没NVRAM，或者它SDMS BIOS/SETUP 管理，你可以通过
启动设置命令配置标记队列特性和设备队列深度，例如：

```
sym53c8xx=tags:4/t2t3q15-t4q7/t1u0q32
```

将标记命令的队列深度设置如下
- 控制0 上的目标 2，所LUN --> 15
- 控制0 上的目标 3，所LUN --> 15
- 控制0 上的目标 4，所LUN -->  7
- 控制1 上的目标 1，LUN 0   --> 32
- 所有其它目LUN                -->  4

在某些特殊条件下，一SCSI 磁盘固件可能会为某条 SCSI 命令返回
QUEUE FULL（队列已满）状态。驱动使用以下启发式方法处理这种行为
- 每次返回 QUEUE FULL 状态时，标记队列深度会减少到当前已断开连接  命令的实际数量
- 每成功完200 SCSI 命令，若当前上限允许，可排队的命令最大数  就会增加
由于接收和处QUEUE FULL 状态会浪费资源，驱动默认会通过提示设备实际
使用的命令数量及其状态、以及它关于设备队列深度变化的决定，来将该问通知用户。驱动处QUEUE FULL 时使用的启发式方法确保了性能影响不至太糟。你可以通过将详细级别设置为零来消除这些消息，如下：

绗?1 绉嶆柟娉曪細
	    使用 'sym53c8xx=verb:0' 选项启动系统2 种方法：
	    在启动后，对对应于你的控制器proc fs 条目应用
            "setverbose 0" 控制命令
## 6. 奇偶校验


驱动支持 SCSI 奇偶校验PCI 总线主设备奇偶校验。必须启用这些特以确保数据传输的安全。一些有缺陷的设备或主板可能会在奇偶校验出问题。驱动中已移除了用于绕过奇偶校验检查的选项
## 7. 性能分析信息


本驱动不像其前身那样提供性能分析信息。该特性并非那么有用，而且
增加了代码的复杂度。随着驱动代码变得越来越复杂，我决定移除一看起来并非真正有用的东西
## 8. 控制命令


控制命令可以通过proc SCSI 文件系统执行写操作来发送给驱动通用命令语法为：

```
      echo "<verb> <parameters>" >/proc/scsi/sym53c8xx/0
      (assumes controller number is 0)
```

对下列命令使"all" 作为 "<target>" 参数，将作用SCSI 链上所有目标（控制器本身除外）
可用命令
### 8.1 设置最小同步周期因

    setsync <target> <period factor>

    :target:   目标编号
    :period:   最小同步周期               最大速度 = 1000/(4*周期因子)，以下特殊情况除外
    指定周期0，可强制进入异步传输模式
     - 9  表示 12.5 纳秒同步周期
     - 10 表示 25 纳秒同步周期
     - 11 表示 30 纳秒同步周期
     - 12 表示 50 纳秒同步周期

### 8.2 设置总线宽度


    setwide <target> <size>

    :target:   目标编号
    :size:     0=8 位，1=16 
### 8.3 设置并发标记命令的最大数

    settags <target> <tags>

    :target:   目标编号
    :tags:     并发标记命令的数               不得大于已配置的值（默认6
### 8.4 设置调试模式


    setdebug <list of debug flags>

    可用调试标志
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

    不带参数使用 "setdebug" 可重置调试标志

### 8.5 设置标志（no_disc

    setflag <target> <flag>

    :target:    目标编号

    目前只有一个标志可用：

        no_disc:   不允许目标断开连接
    不指定任何标志即可重置该标志。例如：

    setflag 4
      将重置目4 no_disc 标志，从而允许其断开连接    setflag all
      将允SCSI 总线上的所有设备断开连接

### 8.6 设置详细级别


    setverbose #level

    驱动的默认详细级别为 1。该命令允许在启动后改变驱动的详细级别
### 8.7 复位目标的所有逻辑单元


    resetdev <target>

    :target:    目标编号

    驱动会尝试向目标发送一BUS DEVICE RESET（总线设备复位）消息
### 8.8 中止目标所有逻辑单元的全部任

    cleardev <target>

    :target:    目标编号

    驱动会尝试向该目标的所有逻辑单元发送一ABORT（中止）消息

## 9. 配置参数


在内核配置工具（例如 make menuconfig）下，可以更改一些默认的驱动
配置参数。如果你所有设备的固件都足够完美，驱动支持的全部特性都可以启动时启用。然而，如果只有一个设备在某个 SCSI 特性上存在缺陷，你可以
Linux 启动时禁用驱动对该特性的支持，并在启动后仅为能安全支持该
特性的设备启用它
配置参数
使用普IO                          （默认回答：n    如果你怀疑你的主板不允许内存映射 I/O，回"y"    可能会略微降低性能
默认标记命令队列深度     （默认回答：16    输入 0 表示默认不使用标记命令    该参数可以从启动命令行指定
排队命令的最大数    （默认回答：32    该选项允许你指定可以排队到某个设备的最大标记命令数    最大支持值为 255
同步传输频率       （默认回答：80    该选项允许你指定驱动在启动时为同步数据传输协商所使用的频率（MHz）    0 表示“异步数据传输”
## 10. 启动设置命令


### 10.1 语法


设置命令既可以在启动时传给驱动，也可以作modprobe 的参数传入，
Documentation/admin-guide/kernel-parameters.rst 中所述
```

    lilo: linux root=/dev/sda2 sym53c8xx.cmd_per_lun=4 sym53c8xx.sync=10 sym53c8xx.debug=0x200

```
- 启用标记命令，最多排4 条标记命令- 将同步协商速度设置10 兆传/ 秒- 设置 DEBUG_NEGO 标志
以下命令将以相同方式安装驱动模块
```

    modprobe sym53c8xx cmd_per_lun=4 sync=10 debug=0x200

```
### 10.2 可用参数


##### 10.2.1  默认标记命令数量


        - cmd_per_lun=0（或 cmd_per_lun=1）禁用标记命令队        - cmd_per_lun=#tagstags > 1）启用标记命令队
  #tags 会被截断为“排队命令最大数量”配置参数
##### 10.2.2 最大突发长

	========== ======================================================
        burst=0    burst disabled
        burst=255  get burst length from initial IO register settings.
        burst=#x   burst enabled (1<<#x burst transfers max)

		   #x is an integer value which is log base 2 of the burst
		   transfers max.
	========== ======================================================

  默认情况下，驱动使用芯片支持的最大值
##### 10.2.3 LED 支持


	=====      ===================
        led=1      enable  LED support
        led=0      disable LED support
	=====      ===================

  如果你的 SCSI 板没有使SDMS BIOS，请勿启LED 支持  （见“配置参数”）

##### 10.2.4 差分模式


	======	=================================
	diff=0	never set up diff mode
        diff=1	set up diff mode if BIOS set it
        diff=2	always set up diff mode
        diff=3	set diff mode if GPIO3 is not set
	======	=================================

##### 10.2.5 IRQ 模式


	======     ================================================
        irqm=0     always open drain
        irqm=1     same as initial settings (assumed BIOS settings)
        irqm=2     always totem pole
	======     ================================================

##### 10.2.6 检SCSI 总线


        buschk=<option bits>

    可用选项位：

	===    ================================================
        0x0    No check.
        0x1    Check and do not attach the controller on error.
        0x2    Check and just warn on error.
	===    ================================================

##### 10.2.7 为主机建议一个默SCSI ID


	==========	==========================================
        hostid=255	no id suggested.
        hostid=#x	(0 < x < 7) x suggested for hosts SCSI id.
	==========	==========================================

    如果能从 NVRAM 获取主机 SCSI ID，驱动将忽略任何作为启动选项建议的值    否则，如果提供了不同255 的建议值，就会使用它。再否则，它会尝    推断之前在硬件中设置的值，并在硬件值为零时使用 7
##### 10.2.8  详细级别


	======     ========
        verb=0     minimal
        verb=1     normal
        verb=2     too much
	======     ========

##### 10.2.9 调试模式


	=========   ====================================
        debug=0	    clear debug flags
        debug=#x    set debug flags

		    #x is an integer value combining the
		    following power-of-2 values:

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
	=========   ====================================

  你可以放心地摆弄 DEBUG_NEGO。但是，其中一些标志可能会产生大量  syslog 消息
##### 10.2.10 稳定延迟


	========	===================
        settle=n	delay for n seconds
	========	===================

  总线复位后，驱动会延n 秒再与总线上的任何设备通信。默认是 3 秒，
  安全模式会默认设10 秒
##### 10.2.11 串行 NVRAM


	.. Note:: option not currently implemented.

	=======     =========================================
        nvram=n     do not look for serial NVRAM
        nvram=y     test controllers for onboard serial NVRAM
	=======     =========================================

        （备选二进制形式
        nvram=<bits options>

        ====   =================================================================
        0x01   look for NVRAM  (equivalent to nvram=y)
        0x02   ignore NVRAM "Synchronous negotiation" parameters for all devices
        0x04   ignore NVRAM "Wide negotiation"  parameter for all devices
        0x08   ignore NVRAM "Scan at boot time" parameter for all devices
        0x80   also attach controllers set to OFF in the NVRAM (sym53c8xx only)
        ====   =================================================================

##### 10.2.12 排除某个主机不被挂载


        excl=<io_address>,...

    阻止位于给定 IO 地址的主机被挂载。例'excl=0xb400,0xc000' 表示指示
    驱动不要挂载地址 0xb400 0xc000 处的主机
### 10.3 从旧式选项转换


```

	sym53c8xx=tags:4,sync:10,debug:0x200

```
由于采用了新的模块参数，上述形式已不再可用。大多数选项的语义保持不变，
tags 改为cmd_per_lun，以反映其不同的用途。上面的例子将变为：

```

	modprobe sym53c8xx cmd_per_lun=4 sync=10 debug=0x200

```

```

	sym53c8xx.cmd_per_lun=4 sym53c8xx.sync=10 sym53c8xx.debug=0x200

```
### 10.4 SCSI 总线检查的启动选项


当该选项被设置为非零值时，驱动会在断言 SCSI RESET（复位）100 微秒后，
检SCSI 信号线的逻辑状态。驱动只是读SCSI 信号线，并检查除 RESET 所有信号线读到的都FALSE。由SCSI 设备最迟应SCSI RESET 断言800
纳秒内释放总线，任何信号为 TRUE 都可能表示存SCSI 总线问题。遗憾的是，
以下常见SCSI 总线问题无法被检测到
- 只安装了一个终结器- 终结器位置不当- 劣质终结器
另一方面，错误的布线、损坏的设备、不合规的设备等都可能导致驱动读取时
某个 SCSI 信号出错
## 15. SCSI 问题排查


### 15.1 问题追踪


大多SCSI 问题都是由于不合规的 SCSI 总线或过于有缺陷的设备引起的如果你不幸遇到了 SCSI 问题，可以检查以下几点：

- SCSI 总线电缆
- SCSI 链两端终结器
- linux syslog 消息（其中一些可能对你有帮助
如果你找不到问题的根源，可以NVRAM 中将驱动或设备配置为最小特性
- 仅异步数据传- 禁用标记命令
- 不允许断开连接

现在，如果你SCSI 总线是正常的，你的系统有很大机会能在该安全配置下
工作，但性能不会是最优的
如果仍然失败，那么你可以将你的问题描述发送到合适的邮件列表或新闻组给我发一份副本，以确保我能收到。显然，驱动代码中存bug 也是有可能的
  我目前的邮箱地址：Gerard Roudier <groudier@free.fr>

如果你在 SCSI 总线上使用多个设备，允许断开连接很重要，但它经常会给
有缺陷的设备带来问题。同步数据传输能提升像硬盘这样的快速设备的吞吐量带有大缓存的优质 SCSI 硬盘能从标记命令队列中获益
### 15.2 理解硬件错误报告


当驱动检测到意外的错误状况时，它可能会显示如下信息：

```

    sym0:1: ERROR (0:48) (1-21-65) (f/95/0) @ (script 7c0:19000000).
    sym0: script cmd = 19000000
    sym0: regdump: da 10 80 95 47 0f 01 07 75 01 81 21 80 01 09 00.

```
此类消息中的某些字段可以帮助你理解错误原因：

```

    sym0:1: ERROR (0:48) (1-21-65) (f/95/0) @ (script 7c0:19000000).
    .....A.........B.C....D.E..F....G.H..I.......J.....K...L.......

```
字段 A ：目标编号  错误发生时控制器正在与之通信的设备的 SCSI ID
字段 B ：DSTAT IO 寄存器（DMA 状态）
  ========   =============================================================
  Bit 0x40   MDPE Master Data Parity Error
             Data parity error detected on the PCI BUS.
  Bit 0x20   BF   Bus Fault
             PCI bus fault condition detected
  Bit 0x01   IID  Illegal Instruction Detected
             Set by the chip when it detects an Illegal Instruction format
             on some condition that makes an instruction illegal.
  Bit 0x80   DFE Dma Fifo Empty
             Pure status bit that does not indicate an error.
  ========   =============================================================

  如果报告中的 DSTAT 值包MDPEx40）与 BFx20）的组合，则原因
  很可能来PCI 总线问题
字段 C ：SIST IO 寄存器（SCSI 中断状态）
  ========   ==================================================================
  Bit 0x08   SGE  SCSI GROSS ERROR
             Indicates that the chip detected a severe error condition
             on the SCSI BUS that prevents the SCSI protocol from functioning
             properly.
  Bit 0x04   UDC  Unexpected Disconnection
             Indicates that the device released the SCSI BUS when the chip
             was not expecting this to happen. A device may behave so to
             indicate the SCSI initiator that an error condition not reportable
             using the SCSI protocol has occurred.
  Bit 0x02   RST  SCSI BUS Reset
             Generally SCSI targets do not reset the SCSI BUS, although any
             device on the BUS can reset it at any time.
  Bit 0x01   PAR  Parity
             SCSI parity error detected.
  ========   ==================================================================

  在有故障SCSI 总线上，芯片可能会检测到 SGEx08）、UDCx04）和
  PARx01）中的任意错误状况。如果你SCSI 系统有时会遇到此类错误状况，
  尤其SCSI GROSS ERROR（SCSI 严重错误），那么 SCSI 总线问题很可能是
  这些错误的根源
对于字段 D、E、F、G H，你可以查看 sym53c8xx_defs.h 文件，其中包关于 IO 寄存器位的一些最简注释
字段 D ：SOCL  Scsi Output Control Latch
          该寄存器反映了芯片想要驱动或用于比较SCSI 控制线的状态字段 E ：SBCL  Scsi Bus Control Lines
          SCSI 总线上控制线的实际值字段 F ：SBDL  Scsi Bus Data Lines
          SCSI 总线上数据线的实际值字段 G ：SXFER  SCSI Transfer
          包含同步周期的输出设置以及当前同步偏移量（偏移量 0 表示异步）字段 H ：SCNTL3 Scsi Control Register 3
          包含异步与同步数据传输的时序值设置字段 I ：SCNTL4 Scsi Control Register 4
          仅对 53C1010 Ultra3 控制器有意义
理解字段 J、K、L 和转储需要熟练掌SCSI 标准、芯片内核功能以及驱内部数据结构。除非你想帮忙维护驱动代码，否则无需解读和理解它们
## 17. 串行 NVRAM（由 Richard Waltham 添加：dormouse@farsrobt.demon.co.uk

### 17.1 特

启用串行 NVRAM 支持后，将能检测到 Symbios 以及部分 Symbios 兼容主机
适配器、还Tekram 板上所包含的串NVRAM。Symbios Tekram 使用串行
NVRAM 来保存主机适配器及其所连驱动器的设置参数
Symbios NVRAM 还保存了在多主机适配器系统中主机适配器的启动顺序信息该信息已不再使用，因为它与热插拔 PCI 模型从根本上不兼容
使用Symbios 芯片、带NVRAM Tekram 板（DC390W/F/U）会被检测到并借此区分 Symbios 兼容Tekram 主机适配器。如果设置了
CONFIG_SCSI_53C8XX_SYMBIOS_COMPAT 配置参数，这会用来在 Tekram 板上
禁用被错误设置的 Symbios 兼容“diff”（差分）设置，从而让 Symbios Tekram 板可以一起使用，Symbios 卡能启用其全部特性，包括“diff支持。（使用 Tekram 卡时，Symbios 兼容卡的“led pin”支持可以保持启用它对 Tekram 主机适配器没有任何用处，但也不会引起问题。）

驱动能够NVRAM 获取的参数取决于所使用的数据格式，如下
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


为了加快系统启动，对于每个配置了“scan at boot time”（启动时扫描）选项
之外的设备，驱动会在收到的第一TEST UNIT READY 命令上强制产生一个错误

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

============= =================
NVRAM Address
============= =================
0x000-0x0ff   not used
0x100-0x26f   initialised data
0x270-0x7ff   not used
============= =================

```

        header  -   6 bytes,
        data    - 356 bytes (checksum is byte sum of this data)
        trailer -   6 bytes
                  ---
        total     368 bytes

```

```

        controller set up  -  20 bytes
        boot configuration -  56 bytes (4x14 bytes)
        device set up      - 128 bytes (16x8 bytes)
        unused (spare锛?   - 152 bytes (19x8 bytes)
                             ---
        total                356 bytes

```

```

    00 00   - ?? start marker
    64 01   - byte count (lsb/msb excludes header/trailer)
    8e 0b   - checksum (lsb/msb excludes header/trailer)

```

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
剩余字节未知 —在我目前的配置中，它们对于任何控制器都未发生变化
53c810a 53c875 的默认设置相（Symbios BIOS 4.09 版添加了 Removable Media
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

```
 这些数据的用法是猜测，但看起来合
剩余字节未知 —在我目前的配置中未发生变
53c810a 53c875 的默认设置相
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
剩余字节未知 —在我目前的配置中未发生变
 这些数据的用法是猜测，但看起来合（但也可能是最大总线宽度
53c810a NVRAM 的默认设53c875 NVRAM 的默认设
    - bus width     - 0x10
                                - sync offset ? - 0x10
                                - sync period   - 0x30

```

    00 00 00 00 00 00 00 00  (19x8bytes)
    .
    .
    00 00 00 00 00 00 00 00

```
53c810a 53c875 的默认设置相
```

    fe fe   - ? end marker ?
    00 00
    00 00

```
53c810a 53c875 的默认设置相
### 17.3 Tekram NVRAM 布局


nvram 64x16024 位）

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
校验和（地址 0x111111
checksum = 0x1234 - (sum addr 0-63)

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
