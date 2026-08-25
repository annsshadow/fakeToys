
## Lenovo WMI Interface Other Mode Driver (lenovo-wmi-other)


## Introduction

Lenovo WMI Other Mode 接口拆分为多GUID，主 Other Mode 接口提供高级电源调优特性，例如 Package Power Tracking（PPT）。它与多个提供各方法上下文的数据GUID 配对
### Other Mode


WMI GUID `DC2A8805-3A8C-41BA-A6F7-092E0089CD3B`

Other Mode WMI 接口使用 firmware_attributes 类在 sysfs 中暴露该接口提供的各WMI 属性。这使得 CPU GPU 的功耗限制调优，以及属于 Lenovo “Gaming Series”设备的一系列其他属性成为可能。Other Mode 接口暴露的每个属性都有对应的能力数据块，使驱动能够探测有关该属性的细节。每个属性有多个页面，分别对应于 Gamezone 接口管理的每个平台配置（profile）。属性通过以下路径sysfs 中暴露：

```

  /sys/class/firmware-attributes/lenovo-wmi-other/attributes/<attribute>/

```
此外，该驱动还把属性导出到 HWMON
### LENOVO_CAPABILITY_DATA_00


WMI GUID `362A3AFE-3D96-4665-8530-96DAD5BB300E`

LENOVO_CAPABILITY_DATA_00 接口提供不依gamezone 散热模式的各类信息
实现了以HWMON 属性：
 - fanX_div: 内部 RPM 除数
 - fanX_input: 当前 RPM
 - fanX_target: 目标 RPM（可调节=自动
由于内部 RPM 除数，当目标 RPM 会被向下取整到最近的整数倍。该除数本身不必2 的幂
### LENOVO_CAPABILITY_DATA_01


WMI GUID `7A8F5407-CB67-4D6E-B547-39B3BE018154`

LENOVO_CAPABILITY_DATA_01 接口提供依赖 gamezone 散热模式的各类信息，包括集成CPU GPU 组件的功耗限制
每个属性具有以下属性：
 - current_value
 - default_value
 - display_name
 - max_value
 - min_value
 - scalar_increment
 - type

实现了以firmware-attributes - ppt_pl1_spl: Platform Profile Tracking Sustained Power Limit
 - ppt_pl2_sppt: Platform Profile Tracking Slow Package Power Tracking
 - ppt_pl3_fppt: Platform Profile Tracking Fast Package Power Tracking

### LENOVO_FAN_TEST_DATA


WMI GUID `B642801B-3D21-45DE-90AE-6E86F164FB21`

LENOVO_FAN_TEST_DATA 接口提供冷却风扇自检的参考数据
实现了以HWMON 属性：
 - fanX_max: 最RPM
 - fanX_min: 最RPM

## WMI interface description


WMI 接口描述可以使用 `bmfdec <https://github.com/pali/bmfdec>`_ 工具从内嵌的二进MOF（bmof）数据中解码
```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO_OTHER_METHOD class"), guid("{dc2a8805-3a8c-41ba-a6f7-092e0089cd3b}")]
  class LENOVO_OTHER_METHOD {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(17), Implemented, Description("Get Feature Value ")] void GetFeatureValue([in] uint32 IDs, [out] uint32 value);
    [WmiMethodId(18), Implemented, Description("Set Feature Value ")] void SetFeatureValue([in] uint32 IDs, [in] uint32 value);
    [WmiMethodId(19), Implemented, Description("Get Data By Command ")] void GetDataByCommand([in] uint32 IDs, [in] uint32 Command, [out] uint32 DataSize, [out, WmiSizeIs("DataSize")] uint32 Data[]);
    [WmiMethodId(99), Implemented, Description("Get Data By Package for TAC")] void GetDataByPackage([in, Max(40)] uint8 Input[], [out] uint32 DataSize, [out, WmiSizeIs("DataSize")] uint8 Data[]);
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 00"), guid("{362a3afe-3d96-4665-8530-96dad5bb300e}")]
  class LENOVO_CAPABILITY_DATA_00 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Capability Default Value.")] uint32 DefaultValue;
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 01"), guid("{7a8f5407-cb67-4d6e-b547-39b3be018154}")]
  class LENOVO_CAPABILITY_DATA_01 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Default Value.")] uint32 DefaultValue;
    [WmiDataId(4), read, Description("Step.")] uint32 Step;
    [WmiDataId(5), read, Description("Minimum Value.")] uint32 MinValue;
    [WmiDataId(6), read, Description("Maximum Value.")] uint32 MaxValue;
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 02"), guid("{bbf1f790-6c2f-422b-bc8c-4e7369c7f6ab}")]
  class LENOVO_CAPABILITY_DATA_02 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Data Size.")] uint32 DataSize;
    [WmiDataId(4), read, Description("Default Value"), WmiSizeIs("DataSize")] uint8 DefaultValue[];
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("Definition of Fan Test Data"), guid("{B642801B-3D21-45DE-90AE-6E86F164FB21}")]
  class LENOVO_FAN_TEST_DATA {
    [key, read] string InstanceName;
    [read] boolean Active;
    [WmiDataId(1), read, Description("Mode.")] uint32 NumOfFans;
    [WmiDataId(2), read, Description("Fan ID."), WmiSizeIs("NumOfFans")] uint32 FanId[];
    [WmiDataId(3), read, Description("Maximum Fan Speed."), WmiSizeIs("NumOfFans")] uint32 FanMaxSpeed[];
    [WmiDataId(4), read, Description("Minumum Fan Speed."), WmiSizeIs("NumOfFans")] uint32 FanMinSpeed[];
  };

```
