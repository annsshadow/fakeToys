
## HPSA - Hewlett Packard Smart Array 驱动


本文件描述用HP Smart Array 控制器的 hpsa SCSI 驱动hpsa 驱动旨在取代较新 Smart Array 控制器的 cciss 驱动hpsa 驱动是一SCSI 驱动，cciss 驱动是一个“块”（block）驱动实际cciss 既是一个块驱动（用于逻辑驱动器）也是一SCSI 驱动
（用于磁带机）。cciss 驱动这种“分裂”的设计是过度复杂性的一个来源，
而消除这种复杂性正hpsa 存在的理由之一
## 支持的设

- Smart Array P212
- Smart Array P410
- Smart Array P410i
- Smart Array P411
- Smart Array P812
- Smart Array P712m
- Smart Array P711m
- StorageWorks P1210m

此外，如果指定了内核启动参数 "hpsa_allow_any=1"，较旧的 Smart Array
也可能与 hpsa 驱动一起工作，但这些并未经HP 使用此驱动进行测试或支持对于较旧Smart Array，仍应使cciss 驱动
"hpsa_simple_mode=1" 启动参数可用于阻止驱动将控制器置于“performant模式。区别在于，simple 模式下，每次命令完成都需要一个中断，而在
“performant 模式”（默认且通常性能更好）下，可以由单个中断指示多个
命令完成
## /sys HPSA 特有的条

  除了 /sys 中可用的通用 SCSI 属性外，hpsa 还支持以下属性：

## HPSA 特有的主机（host）属

```

    /sys/class/scsi_host/host*/rescan
    /sys/class/scsi_host/host*/firmware_revision
    /sys/class/scsi_host/host*/resettable
    /sys/class/scsi_host/host*/transport_mode

  host "rescan" 属性是一个只写属性。写入该属性将导致驱动
  扫描新添加、更改或移除的设备（例如热插拔的磁带机，或新配置  删除的逻辑驱动器等），并将检测到的任何变化通知 SCSI 中间层（midlayer）  通常这由 HP Array Configuration Utility（GUI 或命令行版本）自动触发，
  因此对于逻辑驱动器的更改，用户通常不必使用它。在热插拔诸如磁带机  或包含预配置逻辑驱动器的整个存储箱等设备时，它可能很有用
  "firmware_revision" 属性包Smart Array 的固件版本。例:

	root@host:/sys/class/scsi_host/host4# cat firmware_revision
	7.14

  transport_mode 指示控制器处"performant" 还是 "simple" 模式  这由 "hpsa_simple_mode" 模块参数控制
  "resettable" 只读属性指示特定控制器是否能够响应 "reset_devices"
  内核参数。如果设备可重置，该文件将包"1"，否则为 "0"。例如，
  kdump 使用该参数在驱动加载时重置控制器，以消除控制器上任何未完成的
  命令，并将控制器置于已知状态，以便 kdump 发起I/O 能够正常工作  而不会被来自先前内核的陈旧命令或控制器上残留的其他陈旧状态以任何方式干扰  该属性使 kexec 工具能够在用户试图将一个无法响reset_devices 内核参数  设备指定为转储设备时，对用户发出警告
```
### HPSA 特有的磁盘（disk）属

```

    /sys/class/scsi_disk/c:b:t:l/device/unique_id
    /sys/class/scsi_disk/c:b:t:l/device/raid_level
    /sys/class/scsi_disk/c:b:t:l/device/lunid

  （其c:b:t:l 分别是设备的控制器、总线、目标与 lun
  例如::

	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat unique_id
	600508B1001044395355323037570F77
	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat lunid
	0x0000004000000000
	root@host:/sys/class/scsi_disk/4:0:0:0/device# cat raid_level
	RAID 0

```
## HPSA 特有ioctl


  为了与为 cciss 驱动编写的应用程序兼容，hpsa 驱动也支cciss 驱动
  支持的许多（但并非全部）ioctl。这些所使用的数据结构在
  include/linux/cciss_ioctl.h 中描述
  CCISS_DEREGDISK, CCISS_REGNEWDISK, CCISS_REGNEWD
	上述三个 ioctl 做的事情完全相同，即导致驱动
	重新扫描新设备。这与写hpsa 特有host "rescan" 属性做的事情完全相同
  CCISS_GETPCIINFO
	返回 PCI 域、总线、设备与功能以及 "board ID"（PCI 子系ID）
  CCISS_GETDRIVVER
```

		(major_version << 16) | (minor_version << 8) | (subminor_version)

  CCISS_PASSTHRU, CCISS_BIG_PASSTHRU
	允许"BMIC" "CISS" 命令透传Smart Array	这些HP Array Configuration Utility、SNMP 存储代理等广泛使用	有关一些示例，请参http://cciss.sf.net 上的 cciss_vol_status
```
