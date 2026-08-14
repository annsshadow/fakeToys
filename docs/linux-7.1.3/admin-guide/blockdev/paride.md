## Linux 和 并行 端口 IDE 设备


PARIDE v1.03   (c) 1997-8  Grant Guenther <grant@torque.net>
PATA_PARPORT   (c) 2023 Ondrej Zary

## 1. Introduction


Owing 到 the simplicity 和 near universality 的 the 并行 端口 接口
到 personal computers, 许多 外部 设备 例如 portable hard-disk,
CD-ROM, LS-120 和 tape drives 使用 the 并行 端口 到 connect 到 它们的
host computer.  同时 一些 设备 (notably scanners) 使用 ad-hoc 方法
到 pass 命令 和 数据 through the 并行 端口 接口, 大多数
外部 设备 是 actually identical 到 一个 内部 型号, 但 与
一个 parallel-port adapter 芯片 added 在.  一些 的 the original 并行 端口
adapters 曾是 little 多于 mechanisms 用于 multiplexing 一个 SCSI 总线.
(The Iomega PPA-3 adapter 使用 在 the ZIP drives 是 一个 示例 的 此
approach).  大多数 电流 designs, 然而, take 一个 不同 approach.
The adapter 芯片 reproduces 一个 small ISA 或 IDE 总线 在 the 外部 设备
和 the communication 协议 提供 操作 用于 reading 和 writing
设备 寄存器, 以及 数据 块 transfer 函数.  有时,
the 设备 正在 addressed 通过 the 并行 cable 是 一个 标准 SCSI
控制器 类似 一个 NCR 5380.  The "ditto" family 的 外部 tape
drives 使用 the ISA replicator 到 接口 一个 floppy disk 控制器,
其 是 然后 connected 到 一个 floppy-tape mechanism.  The vast majority
的 外部 并行 端口 设备, 然而, 是 现在 基于 标准
IDE 类型 设备, 其 需要 无 intermediate 控制器.  若 one
曾是 到 打开 up 一个 并行 端口 CD-ROM drive, 例如, one 将会
find 一个 标准 ATAPI CD-ROM drive, 一个 电源 supply, 和 一个 单个 adapter
该 interconnected 一个 标准 PC 并行 端口 cable 和 一个 标准
IDE cable.  它是 通常 可能 到 exchange the CD-ROM 设备 与
任何 其他 设备 使用 the IDE 接口.

The document describes the 支持 在 Linux 用于 并行 端口 IDE
设备.  它 执行 不 cover 并行 端口 SCSI 设备, "ditto" tape
drives 或 scanners.  许多 不同 设备 是 受支持 由 the
并行 端口 IDE 子系统, including:

 - MicroSolutions backpack CD-ROM
 - MicroSolutions backpack PD/CD
 - MicroSolutions backpack hard-drives
 - MicroSolutions backpack 8000t tape drive
 - SyQuest EZ-135, EZ-230 & SparQ drives
 - Avatar Shark
 - Imation Superdisk LS-120
 - Maxell Superdisk LS-120
 - FreeCom 电源 CD
 - Hewlett-Packard 5GB 和 8GB tape drives
 - Hewlett-Packard 7100 和 7200 CD-RW drives

以及 大多数 的 the clone 和 no-name products 在 the market.

到 支持 此类 一个 wide range 的 设备, pata_parport 是 actually structured
在 two parts. 存在 一个 base pata_parport 模块 其 提供 一个 接口
到 内核 libata 子系统, registry 和 一些 通用 方法 用于 accessing
the 并行 ports.

The second component 是 一个 set 的 low-level 协议 驱动 用于 每个 的 the
并行 端口 IDE adapter chips.  Thanks 到 the interest 和 encouragement 的
Linux users 来自 许多 parts 的 the world, 支持 是 可用 用于 almost 全部
known adapter 协议:

	====    ====================================== ====
        aten    ATEN EH-100                            (HK)
        bpck    Microsolutions backpack                (US)
        comm    DataStor (old-type) "commuter" adapter (TW)
        dstr    DataStor EP-2000                       (TW)
        epat    Shuttle EPAT                           (UK)
        epia    Shuttle EPIA                           (UK)
	fit2    FIT TD-2000			       (US)
	fit3    FIT TD-3000			       (US)
	friq    Freecom IQ cable                       (DE)
        frpw    Freecom 电源                          (DE)
        kbic    KingByte KBIC-951一个 和 KBIC-971一个       (TW)
	ktti    KT Technology PHd adapter              (SG)
        在20    OnSpec 90c20                           (US)
        在26    OnSpec 90c26                           (US)
	====    ====================================== ====


## 2. 使用 pata_parport 子系统


同时 configuring the Linux 内核, 您 可 choose 任一个 到 build
the pata_parport 驱动 进入 您的 内核, 或 到 build them 作为 模块.

在 任一个 case, 您 将 需要 到 select "并行 端口 IDE 设备 支持"
和 至少 one 的 the 并行 端口 communication 协议.
若 您 执行 不 know 什么 kind 的 并行 端口 adapter 是 使用 在 您的 drive,
您 可以 begin 由 checking the 文件 names 和 任何 text 文件 在 您的 DOS
installation floppy.  Alternatively, 您可以 look 在 the markings 在
the adapter 芯片 itself.  该's 通常 sufficient 到 identify the
correct 设备.

您可以 actually select 全部 the 协议 模块, 和 允许 the pata_parport
子系统 到 try them 全部 用于 您.

用于 the "brand-name" products listed 上文, 此处 是 the 协议
和 high-level 驱动 该 您 将会 使用:

	================	============	========
	Manufacturer		型号		协议
	================	============	========
	MicroSolutions		CD-ROM		bpck
	MicroSolutions		PD drive	bpck
	MicroSolutions		hard-drive	bpck
	MicroSolutions          8000t tape      bpck
	SyQuest			EZ, SparQ	epat
	Imation			Superdisk	epat
	Maxell                  Superdisk       friq
	Avatar			Shark		epat
	FreeCom			CD-ROM		frpw
	Hewlett-Packard		5GB Tape	epat
	Hewlett-Packard		7200e (CD)	epat
	Hewlett-Packard		7200e (CD-R)	epat
	================	============	========

全部 parports 和 全部 协议 驱动 是 probed automatically 除非 probe=0
参数 是 使用. 因此 just "modprobe epat" 是 enough 用于 一个 Imation SuperDisk
drive 到 work.

```

	# echo "port protocol mode unit delay" >/sys/bus/pata_parport/new_device

```
何处:

	======== ================================================
	端口	 parport name (或 "auto" 用于 全部 parports)
	协议 协议 name (或 "auto" 用于 全部 协议)
	模式	 模式 数字 (protocol-specific) 或 -1 用于 probe
	unit	 unit 数字 (用于 backpack 仅, 参见 下文)
	delay	 I/O delay (参见 troubleshooting section 下文)
	======== ================================================

若 您 happen 到 为 使用 一个 MicroSolutions backpack 设备, 您 将
也 需要 到 know the unit ID 数字 用于 每个 drive.  这是 通常
the 最后 two digits 的 the drive's 串行 数字 (但 读取 MicroSolutions'
documentation 关于 此).

若 您 omit the 参数 来自 the end, defaults 将 为 使用, e.g.:

```

	# echo auto >/sys/bus/pata_parport/new_device

```
```

	# echo "parport0 epat 4" >/sys/bus/pata_parport/new_device

```
```

	# echo "parport0 auto" >/sys/bus/pata_parport/new_device

```
```

	# echo "auto epat" >/sys/bus/pata_parport/new_device

```
```

	# echo pata_parport.0 >/sys/bus/pata_parport/delete_device


```
## 3. Troubleshooting


### 3.1  使用 EPP 模式 若 您可以


The 大多数 通用 problems 该 people report 与 the pata_parport 驱动
concern the 并行 端口 CMOS 设置.  在 此 time, none 的 the
协议 模块 支持 ECP 模式, 或 任何 ECP combination modes.
若 您 是 able 到 执行 因此, 请 set 您的 并行 端口 进入 EPP 模式
使用 您的 CMOS setup procedure.

### 3.2  Check the 端口 delay


一些 并行 ports cannot reliably transfer 数据 在 full speed.  到
偏移 the 错误, the 协议 模块 introduce 一个 "端口
delay" 之间 每个 access 到 the i/o ports.  每个 协议 sets
一个 默认 值 用于 此 delay.  在 大多数 cases, the 用户 可 override
the 默认 和 set 它 到 0 - resulting 在 somewhat higher transfer
rates.  在 一些 rare cases (especially 与 older 486 系统) the
默认 delays 是 不 long enough.  若 您 experience corrupt 数据
transfers, 或 unexpected failures, 您 可 wish 到 increase the
端口 delay.

### 3.3  一些 drives 需要 一个 打印机 reset


那里 appear 到 为 一个 数字 的 "noname" 外部 drives 在 the market
该 执行 不 始终 电源 up correctly.  我们 具有 noticed 此 与 一些
drives 基于 OnSpec 和 older Freecom adapters.  在 这些 rare cases,
the adapter 可 通常 为 reinitialised 由 issuing 一个 "打印机 reset" 在
the 并行 端口.  作为 the reset 操作 是 potentially disruptive 在
多个 设备 environments, the pata_parport 驱动 将 不 执行 它
```

	insmod lp reset=1
	rmmod lp

```
若 您 具有 one 的 这些 marginal cases, 您 应当 probably build
您的 pata_parport 驱动 作为 模块, 和 arrange 到 执行 the 打印机 reset
之前 loading the pata_parport 驱动.
