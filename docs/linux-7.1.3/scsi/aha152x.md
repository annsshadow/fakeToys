
## Linux 下的 Adaptec AHA-1520/1522 SCSI 驱动（aha152x）


Copyright |copy| 1993-1999 Jürgen Fischer <fischer@norbit.de>

TC1550 补丁由 Luuk van Dijk (ldz@xs4all.nl) 提供


第 2 版中该驱动进行了大量修改（尤其是底半部处理函数 complete()）。

驱动现在更加整洁，支持 2.3 中的新错误处理代码，产生的 CPU 负载更低
（轮询循环大幅减少），吞吐量也略有提高（至少在我那台老旧的测试机上；
一台 i486/33Mhz/20MB）。


## 配置参数


============  ========================================  ======================
IOPORT        IO 基址                                   (0x340/0x140)
IRQ           中断级别                                  (9-12; 默认 11)
SCSI_ID       控制器的 SCSI ID                          (0-7; 默认 7)
RECONNECT     允许目标设备从总线断开                    (0/1; 默认 1 [开])
PARITY        启用奇偶校验                              (0/1; 默认 1 [开])
SYNCHRONOUS   启用同步传输                              (0/1; 默认 1 [开])
DELAY:        总线复位延迟                              (默认 100)
EXT_TRANS:    启用扩展转换                              (0/1: 默认 0 [关])
              (见“注意事项”)
============  ========================================  ======================

## 编译期配置


（进入 drivers/scsi/Makefile 中的 AHA152X）：

- DAUTOCONF
    使用控制器报告出来的配置（仅 AHA-152x）

- DSKIP_BIOSTEST
    不测试 BIOS 签名（AHA-1510 或 BIOS 被禁用时）

- DSETUP0="{ IOPORT, IRQ, SCSI_ID, RECONNECT, PARITY, SYNCHRONOUS, DELAY, EXT_TRANS }"
    对第一个控制器的覆盖配置

- DSETUP1="{ IOPORT, IRQ, SCSI_ID, RECONNECT, PARITY, SYNCHRONOUS, DELAY, EXT_TRANS }"
    对第二个控制器的覆盖配置

- DAHA152X_DEBUG
    启用调试输出

- DAHA152X_STAT
    启用一些统计信息


## LILO 命令行选项


```

    aha152x=<IOPORT>[,<IRQ>[,<SCSI-ID>[,<RECONNECT>[,<PARITY>[,<SYNCHRONOUS>[,<DELAY> [,<EXT_TRANS]]]]]]]

 正常的配置可以通过指定命令行来覆盖。这样做时会跳过 BIOS 测试。输入
 的值必须有效（为已知值）。不要使用在正常操作中不被支持的值。如果你
 认为需要其他值：请联系我。对于两个控制器，请使用两次 aha152x 语句。

```
## 模块配置的符号


有两种选择：

```

    aha152x=IOPORT,IRQ,SCSI_ID,RECONNECT,PARITY,SYNCHRONOUS,DELAY,EXT_TRANS

  第一个控制器的配置覆盖

  ::

    aha152x1=IOPORT,IRQ,SCSI_ID,RECONNECT,PARITY,SYNCHRONOUS,DELAY,EXT_TRANS

  第二个控制器的配置覆盖

```
2. 只指定你需要的（irq 或 io 是必需的；新增）

io=IOPORT0[,IOPORT1]
  第一个和第二个控制器的 IOPORT

irq=IRQ0[,IRQ1]
  第一个和第二个控制器的 IRQ

scsiid=SCSIID0[,SCSIID1]
  第一个和第二个控制器的 SCSIID

reconnect=RECONNECT0[,RECONNECT1]
  第一个和第二个控制器是否允许目标设备断开

parity=PAR0[PAR1]
  第一个和第二个控制器是否使用奇偶校验

sync=SYNCHRONOUS0[,SYNCHRONOUS1]
  第一个和第二个控制器是否启用同步传输

delay=DELAY0[,DELAY1]
  第一个和第二个控制器的复位 DELAY

exttrans=EXTTRANS0[,EXTTRANS1]
  第一个和第二个控制器是否启用扩展转换


如果两种方式都使用，则采用第一种。


## 关于 EXT_TRANS 的说明


SCSI 使用块号来寻址设备上的块/扇区。而 BIOS 使用的是柱面/磁头/扇区
（C/H/S）寻址方案。DOS 期望一个能理解这种 C/H/S 寻址的 BIOS 或驱动。

柱面/磁头/扇区的数量称为几何参数（geometry），是 C/H/S 寻址请求的基础。
SCSI 只了解磁盘以块（扇区）计的总容量。

因此 SCSI 的 BIOS/DOS 驱动必须计算出一个逻辑/虚拟几何参数，才能支持
这种寻址方案。SCSI BIOS 返回的几何参数纯属计算结果，与磁盘真实/物理
的几何参数毫无关系（而后者通常也无关紧要）。

基本上这对 Linux 毫无影响，因为它同样使用块而非 C/H/S 寻址。不幸的是，
C/H/S 寻址也用于分区表中，因此每个操作系统都必须知道正确的几何参数
才能解读它。

此外，C/H/S 寻址方案存在某些限制，即地址空间被限制在最多 255 个磁头、
最多 63 个扇区，以及最多 1023 个柱面。

AHA-1522 的 BIOS 通过将磁头数固定为 64、扇区数固定为 32，并用磁盘报告的
容量除以 64*32（1 MB）来计算柱面数，从而得出几何参数。这被视为默认转换。

考虑到 C/H/S 的 1023 柱面限制，你在分区表中只能寻址磁盘的前 1 GB。因此，
基于 AIC-6260/6360 的一些较新控制器的 BIOS 支持扩展转换。这意味着一旦
BIOS 看到大于 1 GB 的磁盘，它就会将磁头数取 255、扇区数取 63，然后用
磁盘容量除以 255*63（约 8 MB）。这样分区表中可寻址的磁盘空间最大约为
8 GB（不过如今已经有更大的磁盘了）。

更复杂的是，在某些 BIOS 设置中，转换模式可能可以、也可能不可配置。

本驱动会进行一些或多或少的“故障安全”猜测，以便在大多数情况下得到
正确的几何参数：

- 对于 <1GB 的磁盘：使用默认转换（C/32/64）

- 对于 >1GB 的磁盘：

  - 从分区表获取当前几何参数（使用 scsicam_bios_param，且只接受“有效”
    的几何参数，即 (C/32/64) 或 (C/63/255)）。即使驱动未启用扩展转换，
    这也可能是扩展转换。

  - 如果失败，则采用由覆盖配置、内核或模块参数启用的扩展转换；否则
    采用默认转换，并请求用户确认。这种情况可能出现在尚未分区的磁盘上。


## 参考文档


 "AIC-6260 SCSI Chip Specification", Adaptec Corporation.

 "SCSI COMPUTER SYSTEM INTERFACE - 2 (SCSI-2)", X3T9.2/86-109 rev. 10h

 "Writing a SCSI device driver for Linux", Rik Faith (faith@cs.unc.edu)

 "Kernel Hacker's Guide", Michael K. Johnson (johnsonm@sunsite.unc.edu)

 "Adaptec 1520/1522 User's Guide", Adaptec Corporation.

 Michael K. Johnson (johnsonm@sunsite.unc.edu)

 Drew Eckhardt (drew@cs.colorado.edu)

 Eric Youngdale (eric@andante.org)

 特别感谢 Eric Youngdale 免费（！）提供关于该芯片的文档。
