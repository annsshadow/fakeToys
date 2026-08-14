## 内核驱动 i2c-piix4


支持的适配器：
  - Intel 82371AB PIIX4 和 PIIX4E
  - Intel 82443MX (440MX)
    Datasheet：在 Intel 网站上公开提供
  - ServerWorks OSB4、CSB5、CSB6、HT-1000 和 HT-1100 南桥
    Datasheet：仅可通过与 ServerWorks 签署的 NDA 获取
  - ATI IXP200、IXP300、IXP400、SB600、SB700 和 SB800 南桥
    Datasheet：未公开提供
    SB700 寄存器参考可在以下位置获取：
    http://support.amd.com/us/Embedded_TechDocs/43009_sb7xx_rrg_pub_1.00.pdf
  - AMD SP5100（见于某些服务器主板的 SB700 衍生型号）
    Datasheet：在 AMD 网站上公开提供
    http://support.amd.com/us/Embedded_TechDocs/44413.pdf
  - AMD Hudson-2、ML、CZ
    Datasheet：未公开提供
  - Hygon CZ
    Datasheet：未公开提供
  - Standard Microsystems (SMSC) SLC90E66 (Victory66) 南桥
    Datasheet：在 SMSC 网站 http://www.smsc.com 公开提供

作者：
 - Frodo Looijaard <frodol@dds.nl>
 - Philip Edelbrock <phil@netroedge.com>


### 模块参数


- force: int
  强制启用 PIIX4。很危险！
- force_addr: int
  强制在给定的地址上启用 PIIX4。极度危险！

### 描述


PIIX4（正确名称为 82371AB）是一个功能丰富的 Intel 芯片。除其它功能外，它还实现了
PCI 总线。它的一个次要功能是实现一个系统管理总线（System Management Bus）。这是一个
真正的 SMBus——你无法在 I2C 层面访问它。好消息是它原生理解 SMBus 命令，你不必担心
时序问题。坏消息是连接到它的非 SMBus 设备可能会让它极度混乱。是的，这确实会发生……

```

  0000:00:02.3 Bridge: Intel Corp. 82371AB/EB/MB PIIX4 ACPI (rev 02)
	       Flags: medium devsel, IRQ 9

```
总线和设备号可能不同，但功能号必须相同（像许多 PCI 设备一样，PIIX4 包含若干个不同
的“功能”，可被视为独立设备）。如果你找到这样的条目，你就拥有一个 PIIX4 SMBus 控制器。

在某些计算机上（最著名的是某些 Dell 机器），SMBus 默认被禁用。如果你使用 insmod
参数 ‘force=1’，内核模块将尝试启用它。这非常危险！如果 BIOS 没有为此模块设置正确的
地址，你可能会陷入大麻烦（读：崩溃、数据损坏等）。仅在万不得已时才尝试（例如先尝试
更新 BIOS），并且先做好备份！一个更危险的选项是 ‘force_addr=<IOPORT>’。这不仅会像
‘force’ 那样启用 PIIX4，还会设置一个新的基地址 I/O 端口。PIIX4 的 SMBus 部分需要
连续 8 个这样的地址才能正常工作。如果这些地址已经被其它设备保留，你将会陷入大麻烦！
如果你不非常确定自己在做什么，不要使用它！

PIIX4E 只是 PIIX4 的一个新版本；它同样受支持。PIIX/PIIX3 没有实现 SMBus 或 I2C 总线，
因此你不能在这些主板上使用此驱动。

ServerWorks 南桥、Intel 440MX 和 Victory66 在 I2C/SMBus 支持上与 PIIX4 完全相同。

AMD SB700、SB800、SP5100 和 Hudson-2 芯片组实现了两个与 PIIX4 兼容的 SMBus 控制器。
如果你的 BIOS 初始化了辅助控制器，它将被此驱动检测为“Auxiliary SMBus Host Controller”
（辅助 SMBus 主控制器）。

如果你拥有 Force CPCI735 主板或其它基于 OSB4 的系统，你可能需要更改 SMBus 中断选择
寄存器，使 SMBus 控制器使用 SMI 模式。

1) 使用 `lspci` 命令并定位带有 SMBus 控制器的 PCI 设备：
   00:0f.0 ISA bridge: ServerWorks OSB4 South Bridge (rev 4f)
   不同芯片组的这一行可能有所不同。请查阅驱动源码了解所有可能的 PCI id（并用
   `lspci -n` 来匹配它们）。假设该设备位于 00:0f.0。
2) 现在你只需更改 0xD2 寄存器中的值。首先用以下命令获取它：
   `lspci -xxx -s 00:0f.0`
   如果值为 0x3，则需要将其改为 0x1：
   `setpci  -s 00:0f.0 d2.b=1`

请注意，你并非在所有情况下都需要这样做，仅当 SMBus 工作不正常时。

### 硬件相关问题


此驱动将拒绝在带有 Intel PIIX4 SMBus 的 IBM 系统上加载。其中一些机器有一个连接到
SMBus 的 RFID EEPROM（24RF08），由于状态机缺陷很容易被损坏。这些主要是 Thinkpad 笔记本，
但台式机系统也可能受影响。我们没有所有受影响系统的列表，因此唯一安全的解决方案是阻止
访问所有 IBM 系统上的 SMBus（通过 DMI 数据检测）。

### ACPI 代码中的描述


PIIX4 芯片的设备驱动为其每个
```

    $ i2cdetect -l
    ...
    i2c-7   unknown         SMBus PIIX4 adapter port 0 at 0b00      N/A
    i2c-8   unknown         SMBus PIIX4 adapter port 2 at 0b00      N/A
    i2c-9   unknown         SMBus PIIX4 adapter port 1 at 0b20      N/A
    ...

```
因此，如果你想在 ACPI 代码中访问其中一个总线，端口
```

    Scope (\_SB_.PCI0.SMBS)
    {
        Name (_ADR, 0x00140000)

        Device (SMB0) {
            Name (_ADR, 0)
        }
        Device (SMB1) {
            Name (_ADR, 1)
        }
        Device (SMB2) {
            Name (_ADR, 2)
        }
    }

```
如果你的 UEFI 固件并非如此，且你无法访问源代码，你可以使用 ACPI SSDT Overlays 来
提供缺失的部分。只需记住，在这种情况下你需要在 piix4 驱动启动之前加载额外的 SSDT
表，即你应该通过 initrd 或 EFI 变量的方式提供 SSDT，而不是通过 configfs。

作为用法示例，下面是一个将为 jc42 分配的 ACPI 代码片段代码
```

    Device (JC42) {
        Name (_HID, "PRP0001")
        Name (_DDN, "JC42 Temperature sensor")
        Name (_CRS, ResourceTemplate () {
            I2cSerialBusV2 (
                0x001c,
                ControllerInitiated,
                100000,
                AddressingMode7Bit,
                "\\_SB.PCI0.SMBS.SMB0",
                0
            )
        })

        Name (_DSD, Package () {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "compatible", Package() { "jedec,jc-42.4-temp" } },
            }
        })
    }

```
