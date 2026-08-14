
## Uniwill 笔记本驱动（uniwill-laptop）


## 简介


Uniwill 制造的许多笔记本（无论是直接制造还是作为 ODM）提供了一个 EC 接口，
用于控制传感器和风扇控制等各类平台设置。该接口被 `uniwill-laptop` 驱动用来
将这些功能映射到标准的内核接口上。

## EC WMI 接口描述


EC WMI 接口描述可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具从
内嵌的二进制 MOF（bmof）数据中解码出来：

```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"),
   Description("Class used to operate methods on a ULong"),
   guid("{ABBC0F6F-8EA1-11d1-00A0-C90629100000}")]
  class AcpiTest_MULong {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(1), Implemented, read, write, Description("Return the contents of a ULong")]
    void GetULong([out, Description("Ulong Data")] uint32 Data);

    [WmiMethodId(2), Implemented, read, write, Description("Set the contents of a ULong")]
    void SetULong([in, Description("Ulong Data")] uint32 Data);

    [WmiMethodId(3), Implemented, read, write,
     Description("Generate an event containing ULong data")]
    void FireULong([in, Description("WMI requires a parameter")] uint32 Hack);

    [WmiMethodId(4), Implemented, read, write, Description("Get and Set the contents of a ULong")]
    void GetSetULong([in, Description("Ulong Data")] uint64 Data,
                     [out, Description("Ulong Data")] uint32 Return);

    [WmiMethodId(5), Implemented, read, write,
     Description("Get and Set the contents of a ULong for Dollby button")]
    void GetButton([in, Description("Ulong Data")] uint64 Data,
                   [out, Description("Ulong Data")] uint32 Return);
  };

```
大部分 WMI 相关代码是从 Windows 驱动示例复制而来的，遗憾的是这意味着该
WMI-GUID 并不唯一。这使得该 WMI-GUID 无法用于自动加载。

### WMI 方法 GetULong()


此 WMI 方法是从 Windows 驱动示例复制而来，没有实际功能。

### WMI 方法 SetULong()


此 WMI 方法是从 Windows 驱动示例复制而来，没有实际功能。

### WMI 方法 FireULong()


此 WMI 方法允许注入一个带有 32 位负载的 WMI 事件。其主要用途似乎是调试。

### WMI 方法 GetSetULong()


此 WMI 方法用于与 EC 通信。`Data` 参数包含以下信息（从最低有效字节开始）：

1. 16 位地址
2. 16 位数据（读取时设为 `0x0000`）
3. 16 位操作（`0x0100` 表示读取，`0x0000` 表示写入）
4. 16 位保留（设为 `0x0000`）

`Return` 值的前 8 位包含在读取时 EC 返回的数据。特殊值 `0xFEFEFEFE` 用于
指示与 EC 通信失败。

### WMI 方法 GetButton()


此 WMI 方法并非在所有机器上都已实现，用途未知。

## 逆向工程 EC WMI 接口


             存在副作用，请小心。

`GetSetULong` 方法背后的 EC 由制造商提供的 OEM 软件使用。由于该软件使用了
混淆器，逆向工程比较困难，但其中部分内容并未被混淆。在这种情况下，`dnSpy
<https://github.com/dnSpy/dnSpy>`_ 也可能有所帮助。

在 Windows 下可以使用 powershell（需要管理员权限）访问 EC：

```

  > $obj = Get-CimInstance -Namespace root/wmi -ClassName AcpiTest_MULong | Select-Object -First 1
  > Invoke-CimMethod -InputObject $obj -MethodName GetSetULong -Arguments @{Data = <input>}

```
## WMI 事件接口描述


WMI 接口描述同样可以从内嵌的二进制 MOF（bmof）数据中解码：

```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"),
   Description("Class containing event generated ULong data"),
   guid("{ABBC0F72-8EA1-11d1-00A0-C90629100000}")]
  class AcpiTest_EventULong : WmiEvent {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, write, Description("ULong Data")] uint32 ULong;
  };

```
大部分 WMI 相关代码同样是从 Windows 驱动示例复制而来，导致此 WMI 接口受到
与上述 EC WMI 接口相同的限制。

### WMI 事件数据


WMI 事件数据包含一个单独的 32 位值，用于指示各种平台事件。

## 逆向工程 Uniwill WMI 事件接口


驱动在收到 WMI 事件时会记录调试消息。因此启用调试消息有助于查找未知的事件
代码。

## EC ACPI 接口描述


`INOU0000` ACPI 设备是一个虚拟设备，用于访问 Uniwill 制造的笔记本上可用的
各种硬件寄存器。通过调用 ACPI 控制方法来读写这些寄存器。`uniwill-laptop`
驱动使用此设备与 EC 通信，因为 ACPI 控制方法比上述 WMI 方法更快。

用于读取寄存器的 ACPI 控制方法接受一个包含待读取寄存器地址的 ACPI 整数，
并返回一个包含该寄存器内数据的 ACPI 整数。而用于写入寄存器的 ACPI 控制方法
则接受两个 ACPI 整数，额外的 ACPI 整数包含要写入寄存器的数据。此类 ACPI 控制
方法不返回任何内容。

### 系统内存


系统内存可以以单字节粒度访问（`MMRB` 用于读取，`MMWB` 用于写入），或以四字节
粒度访问（`MMRD` 用于读取，`MMWD` 用于写入）。这些 ACPI 控制方法未被使用，因为
与内核提供的原生内存访问函数相比，它们没有提供任何好处。

### EC RAM


EC 的内部 RAM 可以使用 `ECRR`（读）和 `ECRW`（写）ACPI 控制方法以单字节粒度
访问，最大寄存器地址为 `0xFFF`。OEM 软件在调用其中一个 ACPI 控制方法后会等待
6 ms，可能是为了避免通过 LPC 连接时使 EC 过载。

### PCI 配置空间


PCI 配置空间可以使用 `PCRD`（读）和 `PCWD`（写）ACPI 控制方法以四字节粒度访问。
确切的地址格式未知，并且随意探测随机 PCI 设备可能会扰乱 PCI 子系统。因此这些
ACPI 控制方法未被使用。

### IO 端口


IO 端口可以使用 `IORD`（读）和 `IOWD`（写）ACPI 控制方法以四字节粒度访问。这些
ACPI 控制方法未被使用，因为与内核提供的原生 IO 端口访问函数相比，它们没有提供
任何好处。

### CMOS RAM


CMOS RAM 可以使用 `RCMS`（读）和 `WCMS` ACPI 控制方法以单字节粒度访问。由于使用
了索引 IO，使用这些 ACPI 方法可能会干扰内核提供的原生 CMOS RAM 访问函数，因此
它们未被使用。

### 索引 IO


使用 IO 端口、以单字节粒度的索引 IO 可以通过 `RIOP`（读）和 `WIOP`（写）ACPI 控制
方法执行。这些 ACPI 方法未被使用，因为与内核提供的原生 IO 端口访问函数相比，它们
没有提供任何好处。

特此感谢 github 用户 `pobrn`，其开发的 `qc71_laptop
<https://github.com/pobrn/qc71_laptop>`_ 驱动是本驱动的部分基础。Tuxedo Computers
也是如此，其开发的 `tuxedo-drivers
<https://gitlab.com/tuxedocomputers/development/packages/tuxedo-drivers>`_ 软件包
也作为本驱动的基础。
