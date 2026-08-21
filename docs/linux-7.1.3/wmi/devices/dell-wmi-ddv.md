
## Dell DDV WMI 接口驱动（dell-wmi-ddv
## 简介（Introduction
2020 年前后生产的许多 Dell 笔记本支持一个基WMI 的接口，用于获取各种系统数据，例如电温度、ePPID、诊断数据以及风温度传感器数据
该接口很可能Windows 上的 `Dell Data Vault` 软件所使用，因此被称为 `DDV`。目`dell-wmi-ddv`
驱动支持该接口的2 版和3 版，新接口版本的添加也很方便
             文档是可用的。因此所有知识都来自试错（trial-and-error），请牢记这一点
## Dell ePPID（电子部件标识，electronic Piece Part Identification
Dell ePPID 用于唯一标识 Dell 机器中的组件，包括电池。其形式类似`CC-PPPPPP-MMMMM-YMD-SSSS-FFF`，并包含以下信息
- 原产国代码（CC）- 部件号，首字符为填充数字（PPPPPP）- 制造商标识（MMMMM）- 制造年/日（YMD），采用 36 进制，其Y 为年份的最后一位数字- 制造序列号（SSSS）- 可选固件版修订号（FFF）
可以使用 `eppidtool <https://pypi.org/project/eppidtool>`_ python 工具来解码并显示这些信息
关于 Dell ePPID 的所有信息都来自 Dell 支持文档以及
`这个网站 <https://telcontar.net/KBK/Dell/date_codes>`_
## WMI 接口描述（WMI interface description
WMI 接口描述可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具从内嵌的二进MOF（bmof数据中解码出来：

```

 [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("WMI Function"), guid("{8A42EA14-4F2A-FD45-6422-0087F7A7E608}")]
 class DDVWmiMethodFunction {
   [key, read] string InstanceName;
   [read] boolean Active;

   [WmiMethodId(1), Implemented, read, write, Description("Return Battery Design Capacity.")] void BatteryDesignCapacity([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(2), Implemented, read, write, Description("Return Battery Full Charge Capacity.")] void BatteryFullChargeCapacity([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(3), Implemented, read, write, Description("Return Battery Manufacture Name.")] void BatteryManufactureName([in] uint32 arg2, [out] string argr);
   [WmiMethodId(4), Implemented, read, write, Description("Return Battery Manufacture Date.")] void BatteryManufactureDate([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(5), Implemented, read, write, Description("Return Battery Serial Number.")] void BatterySerialNumber([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(6), Implemented, read, write, Description("Return Battery Chemistry Value.")] void BatteryChemistryValue([in] uint32 arg2, [out] string argr);
   [WmiMethodId(7), Implemented, read, write, Description("Return Battery Temperature.")] void BatteryTemperature([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(8), Implemented, read, write, Description("Return Battery Current.")] void BatteryCurrent([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(9), Implemented, read, write, Description("Return Battery Voltage.")] void BatteryVoltage([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(10), Implemented, read, write, Description("Return Battery Manufacture Access(MA code).")] void BatteryManufactureAceess([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(11), Implemented, read, write, Description("Return Battery Relative State-Of-Charge.")] void BatteryRelativeStateOfCharge([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(12), Implemented, read, write, Description("Return Battery Cycle Count")] void BatteryCycleCount([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(13), Implemented, read, write, Description("Return Battery ePPID")] void BatteryePPID([in] uint32 arg2, [out] string argr);
   [WmiMethodId(14), Implemented, read, write, Description("Return Battery Raw Analytics Start")] void BatteryeRawAnalyticsStart([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(15), Implemented, read, write, Description("Return Battery Raw Analytics")] void BatteryeRawAnalytics([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(16), Implemented, read, write, Description("Return Battery Design Voltage.")] void BatteryDesignVoltage([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(17), Implemented, read, write, Description("Return Battery Raw Analytics A Block")] void BatteryeRawAnalyticsABlock([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(18), Implemented, read, write, Description("Return Version.")] void ReturnVersion([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(32), Implemented, read, write, Description("Return Fan Sensor Information")] void FanSensorInformation([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(34), Implemented, read, write, Description("Return Thermal Sensor Information")] void ThermalSensorInformation([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
 };

```
每个 WMI 方法都接受一个包32 位索引的 ACPI 缓冲区作为输入参数，其中最8 位用于在使用
电池相关 WMI 方法时指定电池。其WMI 方法可能会忽略该参数，或以不同方式解释它。WMI 方法输出格式各不相同
- 如果函数只有一个输出，则返回相应类型的 ACPI 对象
- 如果函数有多个输出，则返回包含按相同顺序排布的各输出ACPI package

应当彻底检查输出的格式，因为在出错时许多方法会返回格式不正确的数据
许多电池相关方法的数据格式似乎基`Smart Battery Data Specification`（智能电池数据规范）因此未知的电池相关方法很可能以某种方式遵循该标准
### WMI 方法 GetBatteryDesignCapacity()

返回电池的设计容量（单位 mAh），类型u16
### WMI 方法 BatteryFullCharge()

返回电池的完整充电容量（单位 mAh），类型u16
### WMI 方法 BatteryManufactureName()

返回电池的制造商名称，类型为 ASCII 字符串
### WMI 方法 BatteryManufactureDate()

返回电池的制造日期，类型u16日期按以下方式编码：

- 0 4 位包含制造日- 5 8 位包含制造月- 9 15 位包含相1980 年的制造年
### WMI 方法 BatterySerialNumber()

返回电池的序列号，类型为 u16
### WMI 方法 BatteryChemistryValue()

返回电池的化学成分，类型ASCII 字符串已知的值为
- "Li-I" 表示 Li-Ion（锂离子
### WMI 方法 BatteryTemperature()

返回电池的温度（单位：十分之一开尔文），类型u16
### WMI 方法 BatteryCurrent()

返回电池的电流（单位 mA），类型s16负值表示正在放电
### WMI 方法 BatteryVoltage()

返回电池的电压（单位 mV），类型u16
### WMI 方法 BatteryManufactureAccess()

返回电池的健康状态，类型u16健康状态按以下方式编码
 - 第三个半字节（nibble）包含一般故障模 - 第四个半字节包含具体故障代码

有效的故障模式有
 - 永久故障（`0x9` - 过热故障（`0xa` - 过流故障（`0xb`
所有其它故障模式都应视为正常
以下故障代码对永久故障有效：

 - 保险丝熔断（`0x0` - 电芯失衡（`0x1` - 过压（`0x2` - FET 故障（`0x3`
当电池报出永久故障时，故障代码的最后两位应忽略
以下故障代码对过热故障有效：

 - 充电开始时过热（`0x5` - 充电期间过热（`0x7` - 放电期间过热（`0x8`
以下故障代码对过流故障有效：

 - 充电期间过流（`0x6` - 放电期间过流（`0xb`
### WMI 方法 BatteryRelativeStateOfCharge()

返回电池的容量百分比，类型为 u16
### WMI 方法 BatteryCycleCount()

返回电池的循环次数，类型u16
### WMI 方法 BatteryePPID()

返回电池ePPID，类型为 ASCII 字符串
### WMI 方法 BatteryeRawAnalyticsStart()

对电池执行一次分析并返回状态码
- `0x0`：成- `0x1`：接口不支持
- `0xfffffffe`：错超时

   该方法的含义在很大程度上仍未知
### WMI 方法 BatteryeRawAnalytics()

返回一个通常包含 12 个分析数据块的缓冲区这些块包含：

- 0 开始的块编号（u8- 31 字节的未知数
   该方法的含义在很大程度上仍未知
### WMI 方法 BatteryDesignVoltage()

返回电池的设计电压（单位 mV），类型u16
### WMI 方法 BatteryeRawAnalyticsABlock()

返回单块分析数据，索引的第二个字节用于选择块编号
**WMI 接口3 版起支持*

   该方法的含义在很大程度上仍未知
### WMI 方法 ReturnVersion()

返回 WMI 接口版本，类型为 u32
### WMI 方法 FanSensorInformation()

返回一个包含风扇传感器条目的缓冲区，以单个 `0xff` 结尾这些条目包含
- 风扇类型（u8- 风扇转速（单位 RPM，小端序 u16
### WMI 方法 ThermalSensorInformation()

返回一个包含温度传感器条目的缓冲区，以单个 `0xff` 结尾这些条目包含
- 温度类型（u8- 当前温度（s8- 最低温度（s8- 最高温度（s8- 未知字段（u8
   TODO：弄清楚最后一个字节的含义
## ACPI 电池匹配算法（ACPI battery matching algorithm
用于ACPI 电池与索引匹配的算法，基于在 OEM 软件日志消息中找到的信息
基本上，对于每个新的 ACPI 电池，会把索1 3 背后电池的序列号ACPI 电池的序列号进行
比较。由ACPI 电池的序列号既可能被编码为普通整数，也可能被编码为十六进制值，两种情况需要检查。然后选择序列号匹配的第一个索引
序列号为 0 表示该索引未关联实际电池，或所关联的电池不存在
某些机器（如 Dell Inspiron 3505）只支持单块电池，因此忽略电池索引。正因如此，驱动依赖 ACPI
电池 hook 机制来发现电池
## 逆向工程 DDV WMI 接口（Reverse-Engineering the DDV WMI interface
1. 找一台受支持Dell 笔记本，通常是在 2020 年之后生产的2. 导出 ACPI 表并搜索 WMI 设备（通常称为 "ADDV"）3. 解码相应bmof 数据并查ASL 代码4. 尝试通过比较控制流与其它 ACPI 方法（例如电池相关方法的 _BIX _BIF），来推断某WMI
   方法的含义5. 使用内建UEFI 诊断程序查看风扇/温度相关方法的传感器类型/值（有时覆盖静ACPI 数据字段
   可用于测试不同的传感器类型值，因为在某些机器上，该数据在热重置后不会被重新初始化）
或者：

1. 加载 `dell-wmi-ddv` 驱动，必要时使用 `force` 模块参数2. 使用 debugfs 接口访问原始的风温度传感器缓冲区数据3. 将数据与内建 UEFI 诊断程序进行比较
如果Dell 笔记本上可用DDV WMI 接口版本不受支持，或者你看到了未知的风扇/温度传感器，`bugzilla <https://bugzilla.kernel.org>`_ 上提交缺陷报告，以便把它们加`dell-wmi-ddv`
驱动
更多信息请参Documentation/admin-guide/reporting-issues.rst