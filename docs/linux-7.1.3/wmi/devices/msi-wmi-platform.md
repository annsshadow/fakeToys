
## MSI WMI 平台特性驱动（msi-wmi-platform）


## 简介


许多 MSI 笔记本支持各种特性，例如读取风扇传感器。这些特性由嵌入式控制器
控制，ACPI 固件在嵌入式控制器接口之上暴露了一个标准的 ACPI WMI 接口。

## WMI 接口描述


WMI 接口描述可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具从
嵌入式二进制 MOF（bmof）数据中解码出来：

```

  [WMI, Locale("MS\0x409"),
   Description("This class contains the definition of the package used in other classes"),
   guid("{ABBC0F60-8EA1-11d1-00A0-C90629100000}")]
  class Package {
    [WmiDataId(1), read, write, Description("16 bytes of data")] uint8 Bytes[16];
  };

  [WMI, Locale("MS\0x409"),
   Description("This class contains the definition of the package used in other classes"),
   guid("{ABBC0F63-8EA1-11d1-00A0-C90629100000}")]
  class Package_32 {
    [WmiDataId(1), read, write, Description("32 bytes of data")] uint8 Bytes[32];
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\0x409"),
   Description("Class used to operate methods on a package"),
   guid("{ABBC0F6E-8EA1-11d1-00A0-C90629100000}")]
  class MSI_ACPI {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(1), Implemented, read, write, Description("Return the contents of a package")]
    void GetPackage([out, id(0)] Package Data);

    [WmiMethodId(2), Implemented, read, write, Description("Set the contents of a package")]
    void SetPackage([in, id(0)] Package Data);

    [WmiMethodId(3), Implemented, read, write, Description("Return the contents of a package")]
    void Get_EC([out, id(0)] Package_32 Data);

    [WmiMethodId(4), Implemented, read, write, Description("Set the contents of a package")]
    void Set_EC([in, id(0)] Package_32 Data);

    [WmiMethodId(5), Implemented, read, write, Description("Return the contents of a package")]
    void Get_BIOS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(6), Implemented, read, write, Description("Set the contents of a package")]
    void Set_BIOS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(7), Implemented, read, write, Description("Return the contents of a package")]
    void Get_SMBUS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(8), Implemented, read, write, Description("Set the contents of a package")]
    void Set_SMBUS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(9), Implemented, read, write, Description("Return the contents of a package")]
    void Get_MasterBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(10), Implemented, read, write, Description("Set the contents of a package")]
    void Set_MasterBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(11), Implemented, read, write, Description("Return the contents of a package")]
    void Get_SlaveBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(12), Implemented, read, write, Description("Set the contents of a package")]
    void Set_SlaveBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(13), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Temperature([in, out, id(0)] Package_32 Data);

    [WmiMethodId(14), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Temperature([in, out, id(0)] Package_32 Data);

    [WmiMethodId(15), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Thermal([in, out, id(0)] Package_32 Data);

    [WmiMethodId(16), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Thermal([in, out, id(0)] Package_32 Data);

    [WmiMethodId(17), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Fan([in, out, id(0)] Package_32 Data);

    [WmiMethodId(18), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Fan([in, out, id(0)] Package_32 Data);

    [WmiMethodId(19), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Device([in, out, id(0)] Package_32 Data);

    [WmiMethodId(20), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Device([in, out, id(0)] Package_32 Data);

    [WmiMethodId(21), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Power([in, out, id(0)] Package_32 Data);

    [WmiMethodId(22), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Power([in, out, id(0)] Package_32 Data);

    [WmiMethodId(23), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Debug([in, out, id(0)] Package_32 Data);

    [WmiMethodId(24), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Debug([in, out, id(0)] Package_32 Data);

    [WmiMethodId(25), Implemented, read, write, Description("Return the contents of a package")]
    void Get_AP([in, out, id(0)] Package_32 Data);

    [WmiMethodId(26), Implemented, read, write, Description("Set the contents of a package")]
    void Set_AP([in, out, id(0)] Package_32 Data);

    [WmiMethodId(27), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Data([in, out, id(0)] Package_32 Data);

    [WmiMethodId(28), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Data([in, out, id(0)] Package_32 Data);

    [WmiMethodId(29), Implemented, read, write, Description("Return the contents of a package")]
    void Get_WMI([out, id(0)] Package_32 Data);
  };

```
由于 Windows 处理 `CreateByteField()` ACPI 运算符方式上的一个特殊性（仅当
最终访问了一个无效的字节字段时才会发生错误），所有方法都需要一个 32 字节
的输入缓冲区，即便 Binary MOF 另有说明。

输入缓冲区包含一个用于选择要访问的子特性的单字节，以及 31 字节的输入
数据，其含义取决于所访问的子特性。

输出缓冲区包含一个用于指示成功或失败的单字节（`0x00` 表示失败）以及 31 字节
的输出数据，其含义取决于所访问的子特性。

   负责处理 WMI 方法调用的 ACPI 控制方法并非线程安全的。这是一个需要在
   驱动内部自行处理的固件缺陷。

### WMI 方法 Get_EC()


返回嵌入式控制器信息，所选子特性无关紧要。输出数据包含一个标志字节和一个
28 字节的控制器固件版本字符串。

标志字节的前 4 位包含嵌入式控制器接口的次版本号，接下来的 2 位包含嵌入式
控制器接口的主版本号。

第 7 位表示嵌入式控制器页面是否发生了变化（确切含义未知），最后一位表示
平台是否为 Tigerlake 平台。

MSI 软件似乎仅在该最后一位被置位时才使用此接口。

### WMI 方法 Get_Fan()


可以通过选择子特性 `0x00` 来访问风扇转速传感器。输出数据最多包含四个以
大端格式存储的 16 位风扇转速读数。大多数机器并不支持全部四个风扇转速
传感器，因此剩余的读数被硬编码为 `0x0000`。

风扇 RPM 读数可以用下面的公式计算：

        RPM = 480000 / <fan speed reading>

如果风扇转速读数为零，则风扇 RPM 也为零。

### WMI 方法 Get_WMI()


返回 ACPI WMI 接口的版本，所选子特性无关紧要。输出数据包含两个字节，第一个
包含主版本号，最后一个包含 ACPI WMI 接口的次修订号。

MSI 软件似乎仅当主版本号大于 2 时才使用此接口。

## 逆向工程 MSI WMI 平台接口


             以连接到机器并产生其他不良影响，请务必小心。

底层的嵌入式控制器接口由 `msi-ec` 驱动使用，并且似乎许多方法只是把嵌入式
控制器内存的一部分复制到输出缓冲区中。

这意味着，剩余的 WMI 方法可以通过观察 ACPI AML 代码访问了嵌入式控制器
内存的哪一部分来进行逆向工程。该驱动还支持一个 debugfs 接口，用于直接执行
WMI 方法。此外，任何关于不支持硬件的安全检查都可以通过将模块以
`force=true` 加载来禁用。

关于 MSI 嵌入式控制器接口的更多信息，可以在
`msi-ec project <https://github.com/BeardOverflow/msi-ec>`_ 找到。

特别感谢 github 用户 `glpnk` 展示了如何解码风扇转速读数。
