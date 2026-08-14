
## Dell AWCC WMI 接口驱动（alienware-wmi）

本文档描述 Alienware 与 Dell G 系列机型上实现的 AWCC WMI 接口（alienware-wmi 驱动），介绍其通过 Platform Profile API 暴露的散热与超频控制方法，以及由社区逆向工程得到的 AWCCMethodFunction 工作机制。


## 简介


WMI 设备 WMAX 已在许多 Alienware 与 Dell G 系列机型上实现。在这些机型中，已识别出两种实现。第一种用于较老的系统，处理 HDMI、亮度、RGB、放大器与深度睡眠控制。第二种用于较新的系统，主要处理散热控制与超频。

我们怀疑后者被 Alienware Command Center（AWCC）用来管理厂商预定义的散热配置（thermal profile）。alienware-wmi 驱动通过 Platform Profile API 暴露 Thermal_Information 与 Thermal_Control 方法，以模拟 AWCC 的行为。

这个较新的接口名为 AWCCMethodFunction，是在 Dell 没有提供任何官方文档的情况下通过逆向工程得到的。我们将尽力描述其已被发现的内在工作机制。

   以下方法描述可能不完整，并且某些操作在不同设备之间存在不同实现。

### WMI 接口描述


WMI 接口描述可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具从嵌入的二进制 MOF（bmof）数据中解码：

```
 [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("WMI Function"), guid("{A70591CE-A997-11DA-B012-B622A1EF5492}")]
 class AWCCWmiMethodFunction {
   [key, read] string InstanceName;
   [read] boolean Active;

   [WmiMethodId(13), Implemented, read, write, Description("Return Overclocking Report.")] void Return_OverclockingReport([out] uint32 argr);
   [WmiMethodId(14), Implemented, read, write, Description("Set OCUIBIOS Control.")] void Set_OCUIBIOSControl([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(15), Implemented, read, write, Description("Clear OC FailSafe Flag.")] void Clear_OCFailSafeFlag([out] uint32 argr);
   [WmiMethodId(19), Implemented, read, write, Description("Get Fan Sensors.")] void GetFanSensors([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(20), Implemented, read, write, Description("Thermal Information.")] void Thermal_Information([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(21), Implemented, read, write, Description("Thermal Control.")] void Thermal_Control([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(23), Implemented, read, write, Description("MemoryOCControl.")] void MemoryOCControl([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(26), Implemented, read, write, Description("System Information.")] void SystemInformation([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(28), Implemented, read, write, Description("Power Information.")] void PowerInformation([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(32), Implemented, read, write, Description("FW Update GPIO toggle.")] void FWUpdateGPIOtoggle([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(33), Implemented, read, write, Description("Read Total of GPIOs.")] void ReadTotalofGPIOs([out] uint32 argr);
   [WmiMethodId(34), Implemented, read, write, Description("Read GPIO pin Status.")] void ReadGPIOpPinStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(35), Implemented, read, write, Description("Read Chassis Color.")] void ReadChassisColor([out] uint32 argr);
   [WmiMethodId(36), Implemented, read, write, Description("Read Platform Properties.")] void ReadPlatformProperties([out] uint32 argr);
   [WmiMethodId(37), Implemented, read, write, Description("Game Shift Status.")] void GameShiftStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(128), Implemented, read, write, Description("Caldera SW installation.")] void CalderaSWInstallation([out] uint32 argr);
   [WmiMethodId(129), Implemented, read, write, Description("Caldera SW is released.")] void CalderaSWReleased([out] uint32 argr);
   [WmiMethodId(130), Implemented, read, write, Description("Caldera Connection Status.")] void CalderaConnectionStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(131), Implemented, read, write, Description("Surprise Unplugged Flag Status.")] void SurpriseUnpluggedFlagStatus([out] uint32 argr);
   [WmiMethodId(132), Implemented, read, write, Description("Clear Surprise Unplugged Flag.")] void ClearSurpriseUnpluggedFlag([out] uint32 argr);
   [WmiMethodId(133), Implemented, read, write, Description("Cancel Undock Request.")] void CancelUndockRequest([out] uint32 argr);
   [WmiMethodId(135), Implemented, read, write, Description("Devices in Caldera.")] void DevicesInCaldera([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(136), Implemented, read, write, Description("Notify BIOS for SW ready to disconnect Caldera.")] void NotifyBIOSForSWReadyToDisconnectCaldera([out] uint32 argr);
   [WmiMethodId(160), Implemented, read, write, Description("Tobii SW installation.")] void TobiiSWinstallation([out] uint32 argr);
   [WmiMethodId(161), Implemented, read, write, Description("Tobii SW Released.")] void TobiiSWReleased([out] uint32 argr);
   [WmiMethodId(162), Implemented, read, write, Description("Tobii Camera Power Reset.")] void TobiiCameraPowerReset([out] uint32 argr);
   [WmiMethodId(163), Implemented, read, write, Description("Tobii Camera Power On.")] void TobiiCameraPowerOn([out] uint32 argr);
   [WmiMethodId(164), Implemented, read, write, Description("Tobii Camera Power Off.")] void TobiiCameraPowerOff([out] uint32 argr);
 };
```

本文档中未描述的方法行为未知。

### 参数结构


所有输入参数的类型均为 **uint32**，并且它们在各方法之间的结构非常相似。通常，第一个字节对应于方法执行的特定**操作**，随后的字节对应于传给该**操作**的**参数**。例如，如果某个操作码为 0x01 且需要一个 ID 0xA0，则你传给该方法的参数为 0xA001。

## 散热方法


### WMI 方法 GetFanSensors([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 获取与某个风扇 ID 相关的温度传感   | - Byte 1: Fan ID   |
|                    | 器数量                             |                    |
+--------------------+------------------------------------+--------------------+
| 0x02               | 获取与某个风扇传感器 ID 相关的温   | - Byte 1: Fan ID   |
|                    | 度传感器 ID                        | - Byte 2: Index    |
+--------------------+------------------------------------+--------------------+

### WMI 方法 Thermal_Information([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 未知。                             | - None             |
+--------------------+------------------------------------+--------------------+
| 0x02               | 获取具有以下结构的系统描述编号：   | - None             |
|                    |                                    |                    |
|                    | - Byte 0: 风扇数量                 |                    |
|                    | - Byte 1: 温度传感器数量           |                    |
|                    | - Byte 2: 未知                     |                    |
|                    | - Byte 3: 散热配置（profile）数量  |                    |
+--------------------+------------------------------------+--------------------+
| 0x03               | 在给定索引处列出一个 ID 或资源。   | - Byte 1: Index    |
|                    | 风扇 ID、温度 ID、未知 ID 与散热   |                    |
|                    | 配置 ID 按该确切顺序列出。         |                    |
|                    |                                    |                    |
|                    | 操作 0x02 用于了解哪些索引映射到   |                    |
|                    | 哪些资源。                         |                    |
|                    |                                    |                    |
|                    | **返回：** 给定索引处的 ID         |                    |
+--------------------+------------------------------------+--------------------+
| 0x04               | 获取给定温度传感器的当前温度。     | - Byte 1: Sensor   |
|                    |                                    |   ID               |
+--------------------+------------------------------------+--------------------+
| 0x05               | 获取给定风扇的当前 RPM。           | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x06               | 获取风扇转速百分比。（并非每个型号 | - Byte 1: Fan ID   |
|                    | 都实现）                           |                    |
+--------------------+------------------------------------+--------------------+
| 0x07               | 未知。                             | - Unknown          |
+--------------------+------------------------------------+--------------------+
| 0x08               | 获取给定风扇 ID 的最小 RPM。       | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x09               | 获取给定风扇 ID 的最大 RPM。       | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x0A               | 获取均衡散热配置 ID。              | - None             |
+--------------------+------------------------------------+--------------------+
| 0x0B               | 获取当前散热配置 ID。              | - None             |
+--------------------+------------------------------------+--------------------+
| 0x0C               | 获取给定风扇 ID 的当前 `boost` 值。| - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+

### WMI 方法 Thermal_Control([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 激活给定的散热配置。               | - Byte 1: Thermal  |
|                    |                                    |   profile ID       |
+--------------------+------------------------------------+--------------------+
| 0x02               | 为给定风扇 ID 设置 `boost` 值。    | - Byte 1: Fan ID   |
|                    |                                    | - Byte 2: Boost    |
+--------------------+------------------------------------+--------------------+

已知的散热配置代码如下：

+------------------------------+----------+------+
| Thermal Profile              | Type     | ID   |
+==============================+==========+======+
| Custom                       | Special  | 0x00 |
+------------------------------+----------+------+
| G-Mode                       | Special  | 0xAB |
+------------------------------+----------+------+
| Quiet                        | Legacy   | 0x96 |
+------------------------------+----------+------+
| Balanced                     | Legacy   | 0x97 |
+------------------------------+----------+------+
| Balanced Performance         | Legacy   | 0x98 |
+------------------------------+----------+------+
| Performance                  | Legacy   | 0x99 |
+------------------------------+----------+------+
| Balanced                     | USTT     | 0xA0 |
+------------------------------+----------+------+
| Balanced Performance         | USTT     | 0xA1 |
+------------------------------+----------+------+
| Cool                         | USTT     | 0xA2 |
+------------------------------+----------+------+
| Quiet                        | USTT     | 0xA3 |
+------------------------------+----------+------+
| Performance                  | USTT     | 0xA4 |
+------------------------------+----------+------+
| Low Power                    | USTT     | 0xA5 |
+------------------------------+----------+------+

如果某型号支持 User Selectable Thermal Tables（USTT，用户可选散热表）配置，它将不支持 Legacy 配置，反之亦然。

每个型号都支持 CUSTOM（0x00）散热配置。在 G 系列笔记本中，GMODE 取代 PERFORMANCE。

### WMI 方法 GameShiftStatus([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 切换 **Game Shift**。              | - None             |
+--------------------+------------------------------------+--------------------+
| 0x02               | 获取 **Game Shift** 状态。         | - None             |
+--------------------+------------------------------------+--------------------+

Game Shift 状态不会改变风扇速度配置，但它可能是某种 CPU/GPU 电源配置。尚未进行过基准测试。

该方法仅存在于 Dell 的 G 系列笔记本中，其实现意味着 GMODE 散热配置可用，即便 Thermal_Information 的操作 0x03 并未列出它。

Dell G 系列笔记本上的 G 键也会改变 Game Shift 状态，因此二者直接相关。

## 超频方法


### WMI 方法 MemoryOCControl([in] uint32 arg2, [out] uint32 argr)


AWCC 支持内存超频，但该方法非常复杂，尚未被破译。

## GPIO 控制方法


带有 AWCC 接口的 Alienware 与 Dell G 系列设备通常有一个嵌入的 STM32 RGB 灯光控制器，具备 USB/HID 能力。其厂商 ID 为 `187c`，而产品 ID 可能因型号而异。

该 MCU 的两个 GPIO 引脚的控制被作为 WMI 方法暴露出来，用于调试目的。

+--------------+--------------------------------------------------------------+
| Pin          | Description                                                  |
+==============+===============================+==============================+
| 0            | 设备固件更新（DFU）模式引脚。 | **HIGH**：下次 MCU 启动时启用 DFU 模式。 |
|              |                               +------------------------------+
|              |                               | **LOW**：下次 MCU 启动时禁用 DFU 模式。  |
+--------------+-------------------------------+------------------------------+
| 1            | 负复位（NRST）引脚。          | **HIGH**：MCU 开启。         |
|              |                               |                              |
|              |                               +------------------------------+
|              |                               | **LOW**：MCU 关闭。          |
|              |                               |                              |
+--------------+-------------------------------+------------------------------+

关于该 MCU 的更多信息请参见致谢部分。

   某些 GPIO 控制方法打破了通常的参数结构，在第一个字节上使用**引脚号**而非操作码。

### WMI 方法 FWUpdateGPIOtoggle([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| Pin number         | 设置引脚状态                       | - Byte 1: Pin      |
|                    |                                    |   status           |
+--------------------+------------------------------------+--------------------+

### WMI 方法 ReadTotalofGPIOs([out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| N/A                | 获取 GPIO 的总数                   | - None             |
+--------------------+------------------------------------+--------------------+

   由于 WMI 方法在固件层面的实现方式，该方法在被调用时需要一个哑（dummy）uint32 输入参数。

### WMI 方法 ReadGPIOpPinStatus([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| Pin number         | 获取引脚状态                       | - None             |
+--------------------+------------------------------------+--------------------+

   在某些笔记本中存在已知的固件缺陷，读取某个引脚的状态同时会翻转它。

## 其它信息方法


### WMI 方法 ReadChassisColor([out] uint32 argr)


返回机箱颜色的内部 ID。

## 致谢


感谢

- `AlexIII <https://github.com/AlexIII/tcc-g15>`_
- `T-Troll <https://github.com/T-Troll/alienfx-tools/>`_
- `Gabriel Marcano <https://gabriel.marcanobrady.family/blog/2024/12/16/dell-g5-5505-se-acpi-or-figuring-out-how-to-reset-the-rgb-controller/>`_

记录并测试了该设备的部分功能，使得本驱动得以泛化。
