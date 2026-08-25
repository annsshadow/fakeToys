


## Kernel driver dell-smm-hwmon

本页介绍 Dell 笔记本上 dell-smm-hwmon 内核驱动，说明如何通过系统管理模式（SMM）BIOS 读取风扇与温度传感器状态，并经由标hwmon sysfs 接口向用户空间暴露相关监测属性

## 内核驱动 dell-smm-hwmon


:Copyright: |copy| 2002-2005 Massimo Dal Zotto <dz@debian.org>
:Copyright: |copy| 2019 Giovanni Mascellani <gio@debian.org>

### Description

### 描述


On many Dell laptops the System Management Mode (SMM) BIOS can be
queried for the status of fans and temperature sensors.  Userspace
utilities like `sensors` can be used to return the readings. The
userspace suite `i8kutils`__ can also be used to read the sensors and
automatically adjust fan speed (please notice that it currently uses
the deprecated `/proc/i8k` interface).

在许Dell 笔记本上，可以查询系统管理模式（SMM）BIOS 以获取风扇和温度传感器的状态
可以使用诸如 `sensors` 这样的用户空间工具来返回读数。用户空间的 `i8kutils`__ 套件
也可用于读取传感器并自动调节风扇速度（请注意，它目前使用已废弃的 `/proc/i8k` 接口）

 __ https://github.com/vitorafsr/i8kutils

### ``sysfs`` interface

### ``sysfs`` 接口


Temperature sensors and fans can be queried and set via the standard
`hwmon` interface on `sysfs`, under the directory
`/sys/class/hwmon/hwmonX` for some value of `X` (search for the
`X` such that `/sys/class/hwmon/hwmonX/name` has content
`dell_smm`). A number of other attributes can be read or written:

温度和风扇可以通过 `sysfs` 上标`hwmon` 接口进行查询和设置，位于目录
`/sys/class/hwmon/hwmonX`（其`X` 为某个值）下（查找使得
`/sys/class/hwmon/hwmonX/name` 内容`dell_smm` 的那`X`）。还有许多其
属性可以读取或写入

=============================== ======= =======================================
Name				Perm	Description
=============================== ======= =======================================
fan[1-4]_input                  RO      Fan speed in RPM.
fan[1-4]_label                  RO      Fan label.
fan[1-4]_min                    RO      Minimal Fan speed in RPM
fan[1-4]_max                    RO      Maximal Fan speed in RPM
fan[1-4]_target                 RO      Expected Fan speed in RPM
pwm[1-4]                        RW      Control the fan PWM duty-cycle.
pwm[1-4]_enable                 RW/WO   Enable or disable automatic BIOS fan
                                        control (not supported on all laptops,
                                        see below for details).
temp[1-10]_input                RO      Temperature reading in milli-degrees
                                        Celsius.
temp[1-10]_label                RO      Temperature sensor label.
=============================== ======= =======================================

Due to the nature of the SMM interface, each pwmX attribute controls
fan number X.

由于 SMM 接口的特性，每个 pwmX 属性控制编号为 X 的风扇

### Enabling/Disabling automatic BIOS fan control

### 启用/禁用 BIOS 自动风扇控制


There exist two methods for enabling/disabling automatic BIOS fan control:

有两种方法来启用/禁用 BIOS 自动风扇控制

1. Separate SMM commands to enable/disable automatic BIOS fan control for all fans.

1. 使用独立SMM 命令为所有风扇启禁用 BIOS 自动风扇控制

2. A special fan state that enables automatic BIOS fan control for a individual fan.

2. 一种特殊的风扇状态，可为单个风扇启用 BIOS 自动风扇控制

The driver cannot reliably detect what method should be used on a given
device, so instead the following heuristic is used:

驱动无法可靠地检测在某个给定设备上应当使用哪种方法，因此改用以下启发式规则：

- use fan state 3 for enabling BIOS fan control if the maximum fan state
  setable by the user is smaller than 3 (default setting).

- 如果用户可设置的最大风扇状态小3（默认设置），则使用风扇状3 来启BIOS 风扇
  控制

- use separate SMM commands if device is whitelisted to support them.

- 如果设备在白名单中支持独SMM 命令，则使用独立 SMM 命令

When using the first method, each fan will have a standard `pwmX_enable`
sysfs attribute. Writing `1` into this attribute will disable automatic
BIOS fan control for the associated fan and set it to maximum speed. Enabling
BIOS fan control again can be achieved by writing `2` into this attribute.
Reading this sysfs attributes returns the current setting as reported by
the underlying hardware.

使用第一种方法时，每个风扇会有一个标准的 `pwmX_enable` sysfs 属性。向该属性写`1`
会禁用对应风扇的 BIOS 自动风扇控制，并将其设为最大速度。再次启BIOS 风扇控制
通过向该属性写`2` 来实现。读取此 sysfs 属性会返回底层硬件报告的当前设置

When using the second method however, only the `pwm1_enable` sysfs attribute
will be available to enable/disable automatic BIOS fan control globaly for all
fans available on a given device. Additionally, this sysfs attribute is write-only
as there exists no SMM command for reading the current fan control setting.

然而，使用第二种方法时，只`pwm1_enable` sysfs 属性可用于全局启用/禁用给定设备
所有风扇的 BIOS 自动风扇控制。此外，sysfs 属性是只写的，因为不存在用于读取当
风扇控制设置SMM 命令

If no `pwmX_enable` attributes are available, then it means that the driver
cannot use the first method and the SMM codes for enabling and disabling automatic
BIOS fan control are not whitelisted for your device. It is possible that codes
that work for other laptops actually work for yours as well, or that you have to
discover new codes.

如果没有 `pwmX_enable` 属性可用，则意味着驱动无法使用第一种方法，并且用于启用和禁
BIOS 自动风扇控制SMM 代码未列入你设备的白名单。适用于其他笔记本的代码可能也适用
于你的设备，或者你可能需要发现新的代码

Check the list `i8k_whitelist_fan_control` in file
`drivers/hwmon/dell-smm-hwmon.c` in the kernel tree: as a first
attempt you can try to add your machine and use an already-known code
pair. If, after recompiling the kernel, you see that `pwm1_enable`
is present and works (i.e., you can manually control the fan speed),
then please submit your finding as a kernel patch, so that other users
can benefit from it. Please see
Documentation/process/submitting-patches.rst <submittingpatches>
for information on submitting patches.

请查看内核树中文`drivers/hwmon/dell-smm-hwmon.c` 里的列表
`i8k_whitelist_fan_control`：作为首次尝试，你可以试着添加你的机器并使用一对已知的
代码。如果在重新编译内核后，你发`pwm1_enable` 存在且工作正常（即你可以手动控制
风扇速度），请将你的发现作为内核补丁提交，以便其他用户也能受益。关于提交补丁的信息
请参Documentation/process/submitting-patches.rst <submittingpatches>

If no known code works on your machine, you need to resort to do some
probing, because unfortunately Dell does not publish datasheets for
its SMM. You can experiment with the code in `this repository`__ to
probe the BIOS on your machine and discover the appropriate codes.

如果没有已知代码在你的机器上工作，你需要进行一些探测，因为遗憾的是 Dell 没有发布
SMM 的数据手册。你可以`this repository`__ 中的代码在你机器上探BIOS 并发
相应的代码

 __ https://github.com/clopez/dellfan/

Again, when you find new codes, we'd be happy to have your patches!

同样，当你发现新代码时，我们很乐意收到你的补丁！

### ``thermal`` interface

### ``thermal`` 接口


The driver also exports the fans as thermal cooling devices with
`type` set to `dell-smm-fan[1-4]`. This allows for easy fan control
using one of the thermal governors.

该驱动还将风扇导出为散热冷却设备，其 `type` 设为 `dell-smm-fan[1-4]`。这使得使用
某个 thermal governor 可以轻松控制风扇

### Module parameters

### 模块参数


- force:bool
                   Force loading without checking for supported
                   models. (default: 0)

- force:bool
                   强制加载而不检查受支持的型号。（默认

- ignore_dmi:bool
                   Continue probing hardware even if DMI data does not
                   match. (default: 0)

- ignore_dmi:bool
                   即使 DMI 数据不匹配也继续探测硬件。（默认

- restricted:bool
                   Allow fan control only to processes with the
                   `CAP_SYS_ADMIN` capability set or processes run
                   as root when using the legacy `/proc/i8k`
                   interface. In this case normal users will be able
                   to read temperature and fan status but not to
                   control the fan.  If your notebook is shared with
                   other users and you don't trust them you may want
                   to use this option. (default: 1, only available
                   with `CONFIG_I8K`)

- restricted:bool
                   仅允许具`CAP_SYS_ADMIN` 能力的进程，或在使用旧的
                   `/proc/i8k` 接口时以 root 运行的进程控制风扇。在这种情况下，普通用
                   能够读取温度和风扇状态，但不能控制风扇。如果你的笔记本与其他用户共
                   且你不信任他们，你可能会想使用此选项。（默认，仅
                   `CONFIG_I8K` 下可用）

- power_status:bool
                   Report AC status in `/proc/i8k`. (default: 0,
                   only available with `CONFIG_I8K`)

- power_status:bool
                   `/proc/i8k` 中报告交流电源状态。（默认，仅
                   `CONFIG_I8K` 下可用）

- fan_mult:uint
                   Factor to multiply fan speed with. (default:
                   autodetect)

- fan_mult:uint
                   用于乘以风扇速度的系数。（默认：自动检测）

- fan_max:uint
                   Maximum configurable fan speed. (default:
                   autodetect)

- fan_max:uint
                   可配置的最大风扇速度。（默认：自动检测）

### Legacy ``/proc`` interface

### 旧版 ``/proc`` 接口


             used in new applications. This interface is only
             available when kernel is compiled with option
             `CONFIG_I8K`.

             用于新应用中。此接口仅在以内核选项 `CONFIG_I8K` 编译时才可用

The information provided by the kernel driver can be accessed by
```

    $ cat /proc/i8k
    1.0 A17 2J59L02 52 2 1 8040 6420 1 2

```

```
    1.0 A17 2J59L02 52 2 1 8040 6420 1 2
    |   |   |       |  | | |    |    | |
    |   |   |       |  | | |    |    | +------- 10. buttons status
    |   |   |       |  | | |    |    +--------- 9.  AC status
    |   |   |       |  | | |    +-------------- 8.  fan0 RPM
    |   |   |       |  | | +------------------- 7.  fan1 RPM
    |   |   |       |  | +--------------------- 6.  fan0 status
    |   |   |       |  +----------------------- 5.  fan1 status
    |   |   |       +-------------------------- 4.  temp0 reading (Celsius)
    |   |   +---------------------------------- 3.  Dell service tag (later known as 'serial number')
    |   +-------------------------------------- 2.  BIOS version
    +------------------------------------------ 1.  /proc/i8k format version

```
A negative value, for example -22, indicates that the BIOS doesn't
return the corresponding information. This is normal on some
models/BIOSes.

负值，例如 -22，表BIOS 没有返回相应的信息。在某些型号/BIOS 上是正常的

For performance reasons the `/proc/i8k` doesn't report by default
the AC status since this SMM call takes a long time to execute and is
not really needed.  If you want to see the ac status in `/proc/i8k`
you must explictitly enable this option by passing the
`power_status=1` parameter to insmod. If AC status is not
available -1 is printed instead.

出于性能原因，`/proc/i8k` 默认不报告交流电源状态，因为SMM 调用执行时间较长且并
真正需要。如果你想在 `/proc/i8k` 中看到交流电源状态，必须通过insmod 传
`power_status=1` 参数来显式启用此选项。如果交流电源状态不可用，则打印 -1

The driver provides also an ioctl interface which can be used to
obtain the same information and to control the fan status. The ioctl
interface can be accessed from C programs or from shell using the
i8kctl utility. See the source file of `i8kutils` for more
information on how to use the ioctl interface.

该驱动还提供了一ioctl 接口，可用于获取相同的信息并控制风扇状态。该 ioctl 接口
可从 C 程序或通过使用 i8kctl 工具shell 访问。关于如何使ioctl 接口的更多信息，
请参`i8kutils` 的源文件

### SMM Interface

### SMM 接口


             since Dell did not provide any Documentation,
             please keep that in mind.

             由于 Dell 没有提供任何文档，请记住这一点

The driver uses the SMM interface to send commands to the system BIOS.
This interface is normally used by Dell's 32-bit diagnostic program or
on newer notebook models by the buildin BIOS diagnostics.
The SMM may cause short hangs when the BIOS code is taking too long to
execute.

该驱动使SMM 接口向系BIOS 发送命令。此接口通常Dell 32 位诊断程序，或在
较新的笔记本型号上由内置BIOS 诊断功能使用。当 BIOS 代码执行时间过长时，SMM 可能
导致短暂的挂起

The SMM handler inside the system BIOS looks at the contents of the
`eax`, `ebx`, `ecx`, `edx`, `esi` and `edi` registers.
Each register has a special purpose:

系统 BIOS 中的 SMM 处理程序会查`eax`、`ebx`、`ecx`、`edx`、`esi` `edi`
寄存器的内容。每个寄存器都有特殊用途：

=============== ==================================
Register        Purpose
=============== ==================================
eax             Holds the command code before SMM,
                holds the first result after SMM.
ebx             Holds the arguments.
ecx             Unknown, set to 0.
edx             Holds the second result after SMM.
esi             Unknown, set to 0.
edi             Unknown, set to 0.
=============== ==================================

The SMM handler can signal a failure by either:

SMM 处理程序可以通过以下任一方式发出失败信号

- setting the lower sixteen bits of `eax` to `0xffff`
- not modifying `eax` at all
- setting the carry flag (legacy SMM interface only)

- `eax` 的低 16 位设`0xffff`
- 完全不修`eax`
- 设置进位标志（仅旧版 SMM 接口

### Legacy SMM Interface

### 旧版 SMM 接口


When using the legacy SMM interface, a SMM is triggered by writing the least significant byte
of the command code to the special ioports `0xb2` and `0x84`. This interface is not
described inside the ACPI tables and can thus only be detected by issuing a test SMM call.

使用旧版 SMM 接口时，向特ioport `0xb2` `0x84` 写入命令码的最低有效字节来触发
SMM。此接口不在 ACPI 表中描述，因此只能通过发出测试 SMM 调用来检测

### WMI SMM Interface

### WMI SMM 接口


On modern Dell machines, the SMM calls are done over ACPI WMI:

在现Dell 机器上，SMM 调用通过 ACPI WMI 完成

```

 #pragma namespace("\\\\.\\root\\dcim\\sysman\\diagnostics")
 [WMI, Provider("Provider_DiagnosticsServices"), Dynamic, Locale("MS\\0x409"),
  Description("RunDellDiag"), guid("{F1DDEE52-063C-4784-A11E-8A06684B9B01}")]
  class LegacyDiags {
  [key, read] string InstanceName;
  [read] boolean Active;

  [WmiMethodId(1), Implemented, read, write, Description("Legacy Method ")]
  void Execute([in, out] uint32 EaxLen, [in, out, WmiSizeIs("EaxLen") : ToInstance] uint8 EaxVal[],
               [in, out] uint32 EbxLen, [in, out, WmiSizeIs("EbxLen") : ToInstance] uint8 EbxVal[],
               [in, out] uint32 EcxLen, [in, out, WmiSizeIs("EcxLen") : ToInstance] uint8 EcxVal[],
               [in, out] uint32 EdxLen, [in, out, WmiSizeIs("EdxLen") : ToInstance] uint8 EdxVal[]);
 };

```

Some machines support only the WMI SMM interface, while some machines support both interfaces.
The driver automatically detects which interfaces are present and will use the WMI SMM interface
if the legacy SMM interface is not present. The WMI SMM interface is usually slower than the
legacy SMM interface since ACPI methods need to be called in order to trigger a SMM.

有些机器只支WMI SMM 接口，而有些机器两种接口都支持。驱动会自动检测存在哪些接口，
如果旧版 SMM 接口不存在，则使WMI SMM 接口。WMI SMM 接口通常比旧SMM 接口慢，因为
需要调ACPI 方法来触SMM

### SMM command codes

### SMM 鍛戒护鐮。


=============== ======================= ================================================
Command Code    Command Name            Description
=============== ======================= ================================================
`0x0025`      Get Fn key status       Returns the Fn key pressed after SMM:

                                        - 9th bit in `eax` indicates Volume up
                                        - 10th bit in `eax` indicates Volume down
                                        - both bits indicate Volume mute

`0xa069`      Get power status        Returns current power status after SMM:

                                        - 1st bit in `eax` indicates Battery connected
                                        - 3th bit in `eax` indicates AC connected

`0x00a3`      Get fan state           Returns current fan state after SMM:

                                        - 1st byte in `eax` holds the current
                                          fan state (0 - 2 or 3)

`0x01a3`      Set fan state           Sets the fan speed:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the desired
                                          fan state (0 - 2 or 3)

`0x02a3`      Get fan speed           Returns the current fan speed in RPM:

                                        - 1st byte in `ebx` holds the fan number
                                        - 1st word in `eax` holds the current
                                          fan speed in RPM (after SMM)

`0x03a3`      Get fan type            Returns the fan type:

                                        - 1st byte in `ebx` holds the fan number
                                        - 1st byte in `eax` holds the
                                          fan type (after SMM):

                                          - 5th bit indicates docking fan
                                          - 1 indicates Processor fan
                                          - 2 indicates Motherboard fan
                                          - 3 indicates Video fan
                                          - 4 indicates Power supply fan
                                          - 5 indicates Chipset fan
                                          - 6 indicates other fan type

`0x04a3`      Get nominal fan speed   Returns the nominal RPM in each fan state:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the fan state
                                          in question (0 - 2 or 3)
                                        - 1st word in `eax` holds the nominal
                                          fan speed in RPM (after SMM)

`0x05a3`      Get fan speed tolerance Returns the speed tolerance for each fan state:

                                        - 1st byte in `ebx` holds the fan number
                                        - 2nd byte in `ebx` holds the fan state
                                          in question (0 - 2 or 3)
                                        - 1st byte in `eax` returns the speed
                                          tolerance

`0x10a3`      Get sensor temperature  Returns the measured temperature:

                                        - 1st byte in `ebx` holds the sensor number
                                        - 1st byte in `eax` holds the measured
                                          temperature (after SMM)

`0x11a3`      Get sensor type         Returns the sensor type:

                                        - 1st byte in `ebx` holds the sensor number
                                        - 1st byte in `eax` holds the
                                          temperature type (after SMM):

                                          - 1 indicates CPU sensor
                                          - 2 indicates GPU sensor
                                          - 3 indicates SODIMM sensor
                                          - 4 indicates other sensor type
                                          - 5 indicates Ambient sensor
                                          - 6 indicates other sensor type

`0xfea3`      Get SMM signature       Returns Dell signature if interface
                                        is supported (after SMM):

                                        - `eax` holds 1145651527
                                          (0x44494147 or "DIAG")
                                        - `edx` holds 1145392204
                                          (0x44454c4c or "DELL")

`0xffa3`      Get SMM signature       Same as `0xfea3`, check both.
=============== ======================= ================================================

There are additional commands for enabling (`0x31a3` or `0x35a3`) and
disabling (`0x30a3` or `0x34a3`) automatic fan speed control.
The commands are however causing severe sideeffects on many machines, so
they are not used by default.

还有用于启用（`0x31a3` `0x35a3`）和禁用（`0x30a3` `0x34a3`）自动风扇速度控制
额外命令。然而这些命令在许多机器上会造成严重的副作用，因此默认不使用

On several machines (Inspiron 3505, Precision 490, Vostro 1720, ...), the
fans supports a 4th "magic" state, which signals the BIOS that automatic
fan control should be enabled for a specific fan.
However there are also some machines who do support a 4th regular fan state too,
but in case of the "magic" state, the nominal RPM reported for this state is a
placeholder value, which however is not always detectable.

在若干机器上（Inspiron 3505、Precision 490、Vostro 1720 等），风扇支持第 4 魔法"
状态，它向 BIOS 发出信号，应为特定风扇启用自动风扇控制。不过也有一些机器同时支
4 个常规风扇状态，但在"魔法"状态下，为此状态报告的标称 RPM 是一个占位值，然而这
并非总是可检测的

### Firmware Bugs

### 固件缺陷


The SMM calls can behave erratic on some machines:

SMM 调用在某些机器上表现可能不稳定：

======================================================= =================
Firmware Bug                                            Affected Machines
======================================================= =================
Reading of fan states return spurious errors.           Precision 490

                                                        OptiPlex 7060

Reading of fan types causes erratic fan behaviour.      Studio XPS 8000

                                                        Studio XPS 8100

                                                        Inspiron 580

                                                        Inspiron 3505

Fan-related SMM calls take too long (about 500ms).      Inspiron 7720

                                                        Vostro 3360

                                                        XPS 13 9333

                                                        XPS 15 L502X
======================================================= =================

In case you experience similar issues on your Dell machine, please
submit a bugreport on bugzilla to we can apply workarounds.

如果你在 Dell 机器上遇到类似问题，请在 bugzilla 上提bugreport，以便我们应用变
方法

### Limitations

### 限制


The SMM calls can take too long to execute on some machines, causing
short hangs and/or audio glitches.
Also the fan state needs to be restored after suspend, as well as
the automatic mode settings.
When reading a temperature sensor, values above 127 degrees indicate
a BIOS read error or a deactivated sensor.

SMM 调用在某些机器上执行可能耗时过长，导致短暂的挂起或音频故障。此外，风扇状
需要在挂起后恢复，自动模式设置也是如此。读取温度传感器时，高于 127 度的值表BIOS
读取错误或传感器被停用
