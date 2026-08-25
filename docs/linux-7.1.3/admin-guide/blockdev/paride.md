## Linux 并行 端口 IDE 设备


PARIDE v1.03   (c) 1997-8  Grant Guenther <grant@torque.net>
PATA_PARPORT   (c) 2023 Ondrej Zary

## 1. Introduction


Owing the simplicity near universality the 并行 端口 接口
personal computers, 许多 外部 设备 例如 portable hard-disk,
CD-ROM, LS-120 tape drives 使用 the 并行 端口 connect 它们
host computer.  同时 一设备 (notably scanners) 使用 ad-hoc 方法
pass 命令 数据 through the 并行 端口 接口, 大多
外部 设备 actually identical 一内部 型号, 
一parallel-port adapter 芯片 added   一the original 并行 端口
adapters 曾是 little 多于 mechanisms 用于 multiplexing 一SCSI 总线.
(The Iomega PPA-3 adapter 使用 the ZIP drives 一示例 
approach).  大多电流 designs, 然 take 一不同 approach.
The adapter 芯片 reproduces 一small ISA IDE 总线 the 外部 设备
the communication 协议 提供 操作 用于 reading writing
设备 寄存 以及 数据 transfer 函数.  有时,
the 设备 正在 addressed 通过 the 并行 cable 一标准 SCSI
控制类似 一NCR 5380.  The "ditto" family 外部 tape
drives 使用 the ISA replicator 接口 一floppy disk 控制
然后 connected 一floppy-tape mechanism.  The vast majority
外部 并行 端口 设备, 然 现在 基于 标准
IDE 类型 设备, 需intermediate 控制  one
曾是 打开 up 一并行 端口 CD-ROM drive, 例如, one 将会
find 一标准 ATAPI CD-ROM drive, 一电源 supply, 一单个 adapter
interconnected 一标准 PC 并行 端口 cable 一标准
IDE cable.  它是 通常 可能 exchange the CD-ROM 设备 
任何 其他 设备 使用 the IDE 接口.

The document describes the 支持 Linux 用于 并行 端口 IDE
设备.  执行 cover 并行 端口 SCSI 设备, "ditto" tape
drives scanners.  许多 不同 设备 受支the
并行 端口 IDE 子系 including:

 - MicroSolutions backpack CD-ROM
 - MicroSolutions backpack PD/CD
 - MicroSolutions backpack hard-drives
 - MicroSolutions backpack 8000t tape drive
 - SyQuest EZ-135, EZ-230 & SparQ drives
 - Avatar Shark
 - Imation Superdisk LS-120
 - Maxell Superdisk LS-120
 - FreeCom 电源 CD
 - Hewlett-Packard 5GB 鍜?8GB tape drives
 - Hewlett-Packard 7100 鍜?7200 CD-RW drives

以及 大多the clone no-name products the market.

支持 此类 一wide range 设备, pata_parport actually structured
two parts. 存在 一base pata_parport 模块 提供 一接口
内核 libata 子系 registry 一通用 方法 用于 accessing
the 并行 ports.

The second component 一set low-level 协议 驱动 用于 每个 the
并行 端口 IDE adapter chips.  Thanks the interest encouragement 
Linux users 来自 许多 parts the world, 支持 可用 用于 almost 全部
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
        kbic    KingByte KBIC-951一KBIC-971一      (TW)
	ktti    KT Technology PHd adapter              (SG)
        鍦?0    OnSpec 90c20                           (US)
        鍦?6    OnSpec 90c26                           (US)
	====    ====================================== ====


## 2. 使用 pata_parport 子系


同时 configuring the Linux 内核, choose 任一build
the pata_parport 驱动 进入 您的 内核, build them 作为 模块.

任一case, 需select "并行 端口 IDE 设备 支持"
至少 one the 并行 端口 communication 协议.
执行 know 什kind 并行 端口 adapter 使用 您的 drive,
可以 begin checking the 文件 names 任何 text 文件 您的 DOS
installation floppy.  Alternatively, 您可look the markings 
the adapter 芯片 itself.  s 通常 sufficient identify the
correct 设备.

您可actually select 全部 the 协议 模块, 允许 the pata_parport
子系try them 全部 用于 

用于 the "brand-name" products listed 上文, 此处 the 协议
high-level 驱动 将会 使用:

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

全部 parports 全部 协议 驱动 probed automatically 除非 probe=0
参数 使用. 因此 just "modprobe epat" enough 用于 一Imation SuperDisk
drive 鍒?work.

```

	# echo "port protocol mode unit delay" >/sys/bus/pata_parport/new_device

```
何处:

	======== ================================================
	端口	 parport name ("auto" 用于 全部 parports)
	协议 协议 name ("auto" 用于 全部 协议)
	模式	 模式 数字 (protocol-specific) -1 用于 probe
	unit	 unit 数字 (用于 backpack  参见 下文)
	delay	 I/O delay (参见 troubleshooting section 下文)
	======== ================================================

happen 使用 一MicroSolutions backpack 设备, 
需know the unit ID 数字 用于 每个 drive.  这是 通常
the 最two digits the drive's 串行 数字 (读取 MicroSolutions'
documentation 鍏充簬 姝?.

omit the 参数 来自 the end, defaults 使用, e.g.:

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


### 3.1  使用 EPP 模式 您可


The 大多通用 problems people report the pata_parport 驱动
concern the 并行 端口 CMOS 设置.  time, none the
协议 模块 支持 ECP 模式, 任何 ECP combination modes.
able 执行 因此, set 您的 并行 端口 进入 EPP 模式
使用 您的 CMOS setup procedure.

### 3.2  Check the 端口 delay


一并行 ports cannot reliably transfer 数据 full speed.  
偏移 the 错误, the 协议 模块 introduce 一"端口
delay" 之间 每个 access the i/o ports.  每个 协议 sets
一默认 用于 delay.  大多cases, the 用户 override
the 默认 set 0 - resulting somewhat higher transfer
rates.  一rare cases (especially older 486 系统) the
默认 delays long enough.  experience corrupt 数据
transfers, 鎴?unexpected failures, 鎮，鍙?wish 鍒?increase the
端口 delay.

### 3.3  一drives 需一打印reset


那里 appear 一数字 "noname" 外部 drives the market
执行 始终 电源 up correctly.  我们 具有 noticed 一
drives 基于 OnSpec older Freecom adapters.  这些 rare cases,
the adapter 通常 reinitialised issuing 一"打印reset" 
the 并行 端口.  作为 the reset 操作 potentially disruptive 
多个 设备 environments, the pata_parport 驱动 执行 
```

	insmod lp reset=1
	rmmod lp

```
具有 one 这些 marginal cases, 应当 probably build
您的 pata_parport 驱动 作为 模块, arrange 执行 the 打印reset
之前 loading the pata_parport 驱动.
