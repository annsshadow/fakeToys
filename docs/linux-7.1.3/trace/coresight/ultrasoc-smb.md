
## UltraSoc - SoC 上的硬件辅助追踪

   :Author:   Qi Liu <liuqi115@huawei.com>
   :Date:     2023 年 1 月

### 简介


UltraSoc SMB 是一个 per SCCL（Super CPU Cluster）的硬件。它提供了一种在共享系统内存的
某个区域中缓冲与存储 CPU 追踪消息的方式。该设备充当 coresight sink 设备，相应的追踪
生成器（ETM）作为源设备连接其上。

### Sysfs 文件与目录


SMB 设备与其他设备一起出现在已有的 coresight 总线上
```

	$# ls /sys/bus/coresight/devices/
	ultra_smb0   ultra_smb1   ultra_smb2   ultra_smb3

```
```

	$# ls /sys/bus/coresight/devices/ultra_smb0
	enable_sink   mgmt
	$# ls /sys/bus/coresight/devices/ultra_smb0/mgmt
	buf_size  buf_status  read_pos  write_pos

```
关键文件项如下：

   - `read_pos`：显示读指针寄存器的值。
   - `write_pos`：显示写指针寄存器的值。
   - `buf_status`：显示状态寄存器的值。BIT(0) 为零值，表示缓冲区为空。
   - `buf_size`：显示每个设备的缓冲区大小。

### 固件绑定


该设备仅支持 ACPI。其绑定描述设备标识符、资源信息与图结构。

该设备被标识为 ACPI HID "HISI03A1"。设备资源使用 _CRS 方法分配。每个设备必须提供两个基地址；
第一个是设备的配置基地址，第二个是共享系统内存的 32 位基地址。

```

    Device(USMB) {                                               \
      Name(_HID, "HISI03A1")                                     \
      Name(_CRS, ResourceTemplate() {                            \
          QWordMemory (ResourceConsumer, , MinFixed, MaxFixed, NonCacheable, \
		       ReadWrite, 0x0, 0x95100000, 0x951FFFFF, 0x0, 0x100000) \
          QWordMemory (ResourceConsumer, , MinFixed, MaxFixed, Cacheable, \
		       ReadWrite, 0x0, 0x50000000, 0x53FFFFFF, 0x0, 0x4000000) \
      })                                                         \
      Name(_DSD, Package() {                                     \
        ToUUID("ab02a46b-74c7-45a2-bd68-f7d344ef2153"),          \
	/* 使用 CoreSight Graph ACPI 绑定来描述连接拓扑 */
        Package() {                                              \
          0,                                                     \
          1,                                                     \
          Package() {                                            \
            1,                                                   \
            ToUUID("3ecbc8b6-1d0e-4fb3-8107-e627f805c6cd"),      \
            8,                                                   \
            Package() {0x8, 0, \_SB.S00.SL11.CL28.F008, 0},       \
            Package() {0x9, 0, \_SB.S00.SL11.CL29.F009, 0},       \
            Package() {0xa, 0, \_SB.S00.SL11.CL2A.F010, 0},       \
            Package() {0xb, 0, \_SB.S00.SL11.CL2B.F011, 0},       \
            Package() {0xc, 0, \_SB.S00.SL11.CL2C.F012, 0},       \
            Package() {0xd, 0, \_SB.S00.SL11.CL2D.F013, 0},       \
            Package() {0xe, 0, \_SB.S00.SL11.CL2E.F014, 0},       \
            Package() {0xf, 0, \_SB.S00.SL11.CL2F.F015, 0},       \
          }                                                      \
        }                                                        \
      })                                                         \
    }

```
