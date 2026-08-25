
## ACPI 设备—ACPI 命名空间的表

:Copyright: |copy| 2013, Intel Corporation

:Author: Lv Zheng <lv.zheng@intel.com>

:Credit:   Thanks for the help from Zhang Rui <rui.zhang@intel.com> and
           Rafael J.Wysocki <rafael.j.wysocki@intel.com>.

## 摘要

Linux ACPI 子系统将 ACPI 命名空间对象转换/sys/devices/LNXSYSTM:00 下的 Linux 设备树，
并在接收ACPI 热插拔通知事件时更新它。对于此层次结构中的每个设备对象，在
/sys/bus/acpi/devices 中都有一个对应的符号链接
本文档说明了 ACPI 设备树的结构
## ACPI 瀹氫箟鍧。

ACPI 固件在系统内存地址空间中设RSDP（Root System Description Pointer，根系统描述指针），
指向 XSDT（Extended System Description Table，扩展系统描述表）。XSDT 总是通过其第一个条指向 FADT（Fixed ACPI Description Table，固ACPI 描述表），FADT 内的数据包含描述硬件固定
ACPI 特性的各种定长条目。FADT 包含一个指DSDT（Differentiated System Description Table差异化系统描述表）的指针。XSDT 还包含指向可能多SSDT（Secondary System Description
Table，辅助系统描述表）的条目
DSDT SSDT 数据组织在称为定义块的数据结构中，这些块包含各种对象的定义，包括AML
（ACPI Machine Language，ACPI 机器语言）编码的 ACPI 控制方法。DSDT 的数据块连同 SSDT 内容共同表示一个称ACPI 命名空间的分层数据结构，其拓扑结构反映了底层硬件平台的结构
上述 ACPI 系统定义表之间的关系

```

   +---------+    +-------+    +--------+    +------------------------+
   |  RSDP   | +->| XSDT  | +->|  FADT  |    |  +-------------------+ |
   +---------+ |  +-------+ |  +--------+  +-|->|       DSDT        | |
   | Pointer | |  | Entry |-+  | ...... |  | |  +-------------------+ |
   +---------+ |  +-------+    | X_DSDT |--+ |  | Definition Blocks | |
   | Pointer |-+  | ..... |    | ...... |    |  +-------------------+ |
   +---------+    +-------+    +--------+    |  +-------------------+ |
                  | Entry |------------------|->|       SSDT        | |
                  +- - - -+                  |  +-------------------| |
                  | Entry | - - - - - - - -+ |  | Definition Blocks | |
                  +- - - -+                | |  +-------------------+ |
                                           | |  +- - - - - - - - - -+ |
                                           +-|->|       SSDT        | |
                                             |  +-------------------+ |
                                             |  | Definition Blocks | |
                                             |  +- - - - - - - - - -+ |
                                             +------------------------+
                                                          |
                                             OSPM Loading |
                                                         \|/
                                                   +----------------+
                                                   | ACPI Namespace |
                                                   +----------------+

                  Figure 1. ACPI Definition Blocks

```
   Description Table）。平台提RSDT 以实现与 ACPI 1.0 操作系统的兼容性。如果存在，
   操作系统应使XSDT

## ACPI 命名空间示例


所有定义块都被加载到单一的命名空间中。命名空间是一个由名称和路径标识的对象层次结构以下命名约定适用ACPI 命名空间中的对象名称
   1. 所有名称均32 位长   2. 名称的第一个字节必须是 'A' - 'Z'_' 之一   3. 名称的其余每个字节必须是 'A' - 'Z'0' - '9'_' 之一   4. '_' 开头的名称ACPI 规范保留   5. '\' 符号表示命名空间的根（即'\' 为前缀的名称相对于命名空间根）   6. '^' 符号表示当前命名空间节点的父节点（即'^' 为前缀的名称相对于当前命名空间节点      父节点）
```

   +------+
   | \    |                     Root
   +------+
     |
     | +------+
     +-| _PR  |                 Scope(_PR): the processor namespace
     | +------+
     |   |
     |   | +------+
     |   +-| CPU0 |             Processor(CPU0): the first processor
     |     +------+
     |
     | +------+
     +-| _SB  |                 Scope(_SB): the system bus namespace
     | +------+
     |   |
     |   | +------+
     |   +-| LID0 |             Device(LID0); the lid device
     |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _HID |         Name(_HID, "PNP0C0D"): the hardware ID
     |   |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _STA |         Method(_STA): the status control method
     |   |     +------+
     |   |
     |   | +------+
     |   +-| PCI0 |             Device(PCI0); the PCI root bridge
     |     +------+
     |       |
     |       | +------+
     |       +-| _HID |         Name(_HID, "PNP0A08"): the hardware ID
     |       | +------+
     |       |
     |       | +------+
     |       +-| _CID |         Name(_CID, "PNP0A03"): the compatible ID
     |       | +------+
     |       |
     |       | +------+
     |       +-| RP03 |         Scope(RP03): the PCI0 power scope
     |       | +------+
     |       |   |
     |       |   | +------+
     |       |   +-| PXP3 |     PowerResource(PXP3): the PCI0 power resource
     |       |     +------+
     |       |
     |       | +------+
     |       +-| GFX0 |         Device(GFX0): the graphics adapter
     |         +------+
     |           |
     |           | +------+
     |           +-| _ADR |     Name(_ADR, 0x00020000): the PCI bus address
     |           | +------+
     |           |
     |           | +------+
     |           +-| DD01 |     Device(DD01): the LCD output device
     |             +------+
     |               |
     |               | +------+
     |               +-| _BCL | Method(_BCL): the backlight control method
     |                 +------+
     |
     | +------+
     +-| _TZ  |                 Scope(_TZ): the thermal zone namespace
     | +------+
     |   |
     |   | +------+
     |   +-| FN00 |             PowerResource(FN00): the FAN0 power resource
     |   | +------+
     |   |
     |   | +------+
     |   +-| FAN0 |             Device(FAN0): the FAN0 cooling device
     |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _HID |         Name(_HID, "PNP0A0B"): the hardware ID
     |   |     +------+
     |   |
     |   | +------+
     |   +-| TZ00 |             ThermalZone(TZ00); the FAN thermal zone
     |     +------+
     |
     | +------+
     +-| _GPE |                 Scope(_GPE): the GPE namespace
       +------+

                     Figure 2. Example ACPI Namespace


```
## Linux ACPI 设备对象


Linux 内核的核ACPI 子系统为表示设备、电源资源、处理器、热区的 ACPI 命名空间对象创建
struct acpi_device 对象。这些对象通过 sysfs 作为 /sys/devices/LNXSYSTM:00 下子树中的目导出到用户空间。它们名称的格式<bus_id:instance>，其'bus_id' 指代给定对象ACPI
命名空间表示instance' 用于区分具有相同 'bus_id' 的不同对象（它是无符号整数的两位
十进制表示）
'bus_id' 的值取决于其名称所属对象的类型

```

                +---+-----------------+-------+----------+
                |   | Object/Feature  | Table | bus_id   |
                +---+-----------------+-------+----------+
                | N | Root            | xSDT  | LNXSYSTM |
                +---+-----------------+-------+----------+
                | N | Device          | xSDT  | _HID     |
                +---+-----------------+-------+----------+
                | N | Processor       | xSDT  | LNXCPU   |
                +---+-----------------+-------+----------+
                | N | ThermalZone     | xSDT  | LNXTHERM |
                +---+-----------------+-------+----------+
                | N | PowerResource   | xSDT  | LNXPOWER |
                +---+-----------------+-------+----------+
                | N | Other Devices   | xSDT  | device   |
                +---+-----------------+-------+----------+
                | F | PWR_BUTTON      | FADT  | LNXPWRBN |
                +---+-----------------+-------+----------+
                | F | SLP_BUTTON      | FADT  | LNXSLPBN |
                +---+-----------------+-------+----------+
                | M | Video Extension | xSDT  | LNXVIDEO |
                +---+-----------------+-------+----------+
                | M | ATA Controller  | xSDT  | LNXIOBAY |
                +---+-----------------+-------+----------+
                | M | Docking Station | xSDT  | LNXDOCK  |
                +---+-----------------+-------+----------+

                 Table 1. ACPI Namespace Objects Mapping

```
在基ACPI 系统描述表的内容（如上文表格第一列的字母和第二列的记号所示）创建
struct acpi_device 对象时，适用以下规则
   N:
      对象的来源是一ACPI 命名空间节点（如第二列中命名对象的类型所示）。在这种情况下，
      该对象在 sysfs 中的目录将包'path' 属性，其值为从命名空间根到该节点的完整路径   F:
      为固定的硬件特性创struct acpi_device 对象（如第二列中固定特性标志的名称所示）      因此sysfs 目录不会包含 'path' 属性   M:
      为具有特定控制方法的 ACPI 命名空间节点创建 struct acpi_device 对象（如第二列中 ACPI
      定义的设备类型所示）。包含其命名空间路径'path' 属性将出现在其 sysfs 目录中。例如，
      如果某个 ACPI 命名空间节点存在 _BCL 方法，则会为其创建一'bus_id' LNXVIDEO       struct acpi_device 对象
上表的第三列指示哪些 ACPI 系统描述表包含用于创建给定行所表示 struct acpi_device 对象信息（xSDT 表示 DSDT SSDT）
上表的第四列指示 struct acpi_device 对象'bus_id' 生成规则
   _HID:
      _HID 在表中最后一列意味着对象bus_id 派生自相ACPI 命名空间节点下的 _HID/_CID
      标识对象。该对象sysfs 目录将随后包'hid' 'modalias' 属性，可用于检索该对象      _HID _CID   LNXxxxxx:
      对于 bus_id "LNXxxxxx" 形式（伪设备）的 struct acpi_device 对象，也存在 'modalias'
      属性，在这种情况下它包bus_id 字符串本身   device:
      表中最后一列的 'device' 表示该对象的 bus_id 无法从相ACPI 命名空间节点_HID/_CID
      确定，尽管该对象表示一个设备（例如，它可能是一个定义了 _ADR 但没_HID _CID       PCI 设备）。在这种情况下，字符'device' 将用作该对象bus_id

## Linux ACPI 物理设备粘合


ACPI 设备（即 struct acpi_device）对象可以链接到 Linux 设备层次结构中表示“物理”设备的
其他对象（例PCI 总线上的设备）。如果发生这种情况，意味着ACPI 设备对象是某个以其他
方式表示的设备的“伴生对象”（companion），并用于（1）提供无法通过其他方式获得的关于该
设备的配置信息，以及）借助ACPI 控制方法对该设备执行特定操作。一ACPI 设备对象可以
以这种方式链接到多个“物理”设备
如果某个 ACPI 设备对象链接到“物理”设备，sysfs 目录将包含指向目标设备对sysfs 目录"physical_node" 符号链接。反过来，目标设备的 sysfs 目录将包含指向伴ACPI 设备对象 sysfs
目录"firmware_node" 符号链接。链接机制依ACPI 命名空间提供的设备标识。例如，如果存在
一个表PCI 设备ACPI 命名空间对象（即表示 PCI 桥的 ACPI 命名空间对象下的设备对象），
_ADR 返回 0x00020000，且PCI 桥的总线号为 0，则表示为该 ACPI 命名空间对象创建struct acpi_device 对象sysfs 目录将包含指向相PCI 设备/sys/devices/pci0000:00/0000:00:02:0/ sysfs 目录'physical_node' 符号链接
链接机制通常是总线特定的。其实现的核心位drivers/acpi/glue.c 文件中，但还有位于其位置的、取决于相关总线类型的补充部分。例如，PCI 特定部分位于 drivers/pci/pci-acpi.c
中

## Linux ACPI 设备树示

与图 2 所示示ACPI 命名空间对应struct acpi_device 对象sysfs 层次结构，并附加

```

   +--------------+---+-----------------+
   | LNXSYSTM:00  | \ | acpi:LNXSYSTM:  |
   +--------------+---+-----------------+
     |
     | +-------------+-----+----------------+
     +-| LNXPWRBN:00 | N/A | acpi:LNXPWRBN: |
     | +-------------+-----+----------------+
     |
     | +-------------+-----+----------------+
     +-| LNXSLPBN:00 | N/A | acpi:LNXSLPBN: |
     | +-------------+-----+----------------+
     |
     | +-----------+------------+--------------+
     +-| LNXCPU:00 | \_PR_.CPU0 | acpi:LNXCPU: |
     | +-----------+------------+--------------+
     |
     | +-------------+-------+----------------+
     +-| LNXSYBUS:00 | \_SB_ | acpi:LNXSYBUS: |
     | +-------------+-------+----------------+
     |   |
     |   | +- - - - - - - +- - - - - - +- - - - - - - -+
     |   +-| PNP0C0D:00 | \_SB_.LID0 | acpi:PNP0C0D: |
     |   | +- - - - - - - +- - - - - - +- - - - - - - -+
     |   |
     |   | +------------+------------+-----------------------+
     |   +-| PNP0A08:00 | \_SB_.PCI0 | acpi:PNP0A08:PNP0A03: |
     |     +------------+------------+-----------------------+
     |       |
     |       | +-----------+-----------------+-----+
     |       +-| device:00 | \_SB_.PCI0.RP03 | N/A |
     |       | +-----------+-----------------+-----+
     |       |   |
     |       |   | +-------------+----------------------+----------------+
     |       |   +-| LNXPOWER:00 | \_SB_.PCI0.RP03.PXP3 | acpi:LNXPOWER: |
     |       |     +-------------+----------------------+----------------+
     |       |
     |       | +-------------+-----------------+----------------+
     |       +-| LNXVIDEO:00 | \_SB_.PCI0.GFX0 | acpi:LNXVIDEO: |
     |         +-------------+-----------------+----------------+
     |           |
     |           | +-----------+-----------------+-----+
     |           +-| device:01 | \_SB_.PCI0.DD01 | N/A |
     |             +-----------+-----------------+-----+
     |
     | +-------------+-------+----------------+
     +-| LNXSYBUS:01 | \_TZ_ | acpi:LNXSYBUS: |
       +-------------+-------+----------------+
         |
         | +-------------+------------+----------------+
         +-| LNXPOWER:0a | \_TZ_.FN00 | acpi:LNXPOWER: |
         | +-------------+------------+----------------+
         |
         | +------------+------------+---------------+
         +-| PNP0C0B:00 | \_TZ_.FAN0 | acpi:PNP0C0B: |
         | +------------+------------+---------------+
         |
         | +-------------+------------+----------------+
         +-| LNXTHERM:00 | \_TZ_.TZ00 | acpi:LNXTHERM: |
           +-------------+------------+----------------+

                  Figure 3. Example Linux ACPI Device Tree

```

   1. 'object' 是对象在 sysfs 中目录的名称   2. 'path' 是相ACPI 命名空间对象ACPI 命名空间路径，由该对象的 'path' sysfs 属性返回   3. 'modalias' 是对象的 'modalias' sysfs 属性的值（如本文档前文所述）
   'modalias' attribute.
