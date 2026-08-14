## HID Sensors 框架

HID sensor 框架提供了实现 sensor 驱动所需的接口，这些驱动连接到 sensor hub。sensor hub 是一个 HID 设备，它提供一个符合 HID 1.12 sensor usage 表的报告描述符。

来自 HID 1.12 “HID Sensor Usages” 规范的描述：
“对 sensor 的 HID usage 进行标准化，可以（但不是必须）让 sensor 硬件厂商在 USB 边界提供一个一致的即插即用（Plug And Play）接口，从而使某些操作系统能够集成可在厂商之间复用的通用设备驱动，免除厂商自行提供驱动的需要。”

该规范定义了大量 usage ID，它们描述 sensor 的类型以及各个数据字段。每个 sensor 可以具有可变数量的数据字段。字段的长度和顺序由报告描述符指定。对于

```
     INPUT(1)[INPUT]
   ..
      Field(2)
        Physical(0020.0073)
        Usage(1)
          0020.045f
        Logical Minimum(-32767)
        Logical Maximum(32767)
        Report Size(8)
        Report Count(1)
        Report Offset(16)
        Flags(Variable Absolute)
  ..
  ..

```
该报告表明 “sensor page (0x20)” 包含一个 accelerometer-3D（加速度计 3D，0x73）。这个 accelerometer-3D 具有一些字段。例如这里字段 2 是 motion intensity（运动强度，0x045f），其逻辑最小值为 -32767，逻辑最大值为 32767。字段的顺序和每个字段的长度很重要，因为输入事件的原始数据将使用这种格式。


## 实现


该规范定义了多种具有不同数据字段集合的 sensor 类型。对于不同的 sensor，很难有一个通用的输入事件传给用户空间应用程序。例如加速度计可以发送 X、Y 和 Z 数据，而环境光 sensor 可以发送照度数据。
因此实现分为两部分：

- Core HID 驱动
- 单独的 sensor 处理部分（sensor 驱动）

### Core 驱动

core 驱动（hid-sensor-hub）作为一个 HID 驱动注册。它解析报告描述符并识别所有存在的 sensor。它添加一个名为 HID-SENSOR-xxxx 的 MFD 设备（其中 xxxx 是规范中的 usage id）。

例如：

HID-SENSOR-200073 注册为一个 Accelerometer 3D（三维加速度计）驱动。

因此，如果插入了任何具有该名称的驱动，就会调用该函数的 probe 例程。所以一个加速度计处理驱动可以用该名称注册，并在检测到 accelerometer-3D 时被 probe。

core 驱动提供了一组 API，供处理驱动用来注册并获取该 usage id 的事件。同时它还提供解析函数，用于获取和设置每个 input/feature/output 报告。

### 单独的 sensor 处理部分（sensor 驱动）


处理驱动将使用 core 驱动提供的接口来解析报告并获取字段的索引，也可以获取事件。该驱动可以使用 IIO 接口来使用为某类 sensor 定义的标准 ABI。


## Core 驱动接口


```
  Each processing driver can use this structure to set some callbacks.
	int (*suspend)(..): Callback when HID suspend is received
	int (*resume)(..): Callback when HID resume is received
	int (*capture_sample)(..): Capture a sample for one of its data fields
	int (*send_event)(..): One complete event is received which can have
                               multiple data fields.

```
```

  int sensor_hub_register_callback(struct hid_sensor_hub_device *hsdev,
			u32 usage_id,
			struct hid_sensor_hub_callbacks *usage_callback):

```
为某个 usage id 注册回调。回调函数不允许

```

  int sensor_hub_remove_callback(struct hid_sensor_hub_device *hsdev,
			u32 usage_id):

```
移除某个 usage id 的回调。


```

  int sensor_hub_input_get_attribute_info(struct hid_sensor_hub_device *hsdev,
			u8 type,
			u32 usage_id, u32 attr_usage_id,
			struct hid_sensor_hub_attribute_info *info);

```
处理驱动可以查找某个感兴趣的字段，并检查它是否存在于报告描述符中。如果存在，它将存储必要的信息，以便可以单独地设置或获取字段。
这些索引避免了每次都去搜索并获取字段索引来进行设置或获取。


```

  int sensor_hub_set_feature(struct hid_sensor_hub_device *hsdev, u32 report_id,
			u32 field_index, s32 value);

```
该接口用于设置 feature 报告中某个字段的值。例如，如果存在一个字段 report_interval（之前由对 sensor_hub_input_get_attribute_info 的调用解析得到），那么它可以直接设置该


```

  int sensor_hub_get_feature(struct hid_sensor_hub_device *hsdev, u32 report_id,
			u32 field_index, s32 *value);

```
该接口用于获取 input 报告中某个字段的值。例如，如果存在一个字段 report_interval（之前由对 sensor_hub_input_get_attribute_info 的调用解析得到），那么它可以直接获取该


```

  int sensor_hub_input_attr_get_raw_value(struct hid_sensor_hub_device *hsdev,
			u32 usage_id,
			u32 attr_usage_id, u32 report_id);

```
该接口用于通过 input 报告获取某个特定字段的值。例如加速度计想要轮询 X 轴的值，就可以用 X 轴的 usage id 调用此函数。HID sensor 可以提供事件，因此不必轮询任何字段。如果有新样本，core 驱动会调用已注册的回调函数来对样本进行处理。


----------

### HID Custom 与 Generic Sensor


HID Sensor 规范定义了两种特殊的 sensor usage 类型。由于它们不代表标准 sensor，因此无法用 Linux IIO 类型接口来定义。
这些 sensor 的目的是扩展功能，或提供一种方式来混淆 sensor 所传递的数据。在不知道数据与其封装形式之间的映射关系时，应用程序/驱动很难判断 sensor 正在传递什么数据。
这允许一些差异化的用例，厂商可以在其中提供应用程序。一些常见用例是调试其他 sensor，或提供一些诸如键盘接入/移除、盖子开/合之类的事件。

为了让应用程序能够利用这些 sensor，这里通过 sysfs 属性组、属性以及 misc 设备接口将它们导出。

```

  /sys/devices/pci0000:00/INT33C2:00/i2c-0/i2c-INT33D1:00/0018:8086:09FA.0001/HID-SENSOR-2000e1.6.auto$ tree -R
  .
  │   ├──  enable_sensor
  │   │   ├── feature-0-200316
  │   │   │   ├── feature-0-200316-maximum
  │   │   │   ├── feature-0-200316-minimum
  │   │   │   ├── feature-0-200316-name
  │   │   │   ├── feature-0-200316-size
  │   │   │   ├── feature-0-200316-unit-expo
  │   │   │   ├── feature-0-200316-units
  │   │   │   ├── feature-0-200316-value
  │   │   ├── feature-1-200201
  │   │   │   ├── feature-1-200201-maximum
  │   │   │   ├── feature-1-200201-minimum
  │   │   │   ├── feature-1-200201-name
  │   │   │   ├── feature-1-200201-size
  │   │   │   ├── feature-1-200201-unit-expo
  │   │   │   ├── feature-1-200201-units
  │   │   │   ├── feature-1-200201-value
  │   │   ├── input-0-200201
  │   │   │   ├── input-0-200201-maximum
  │   │   │   ├── input-0-200201-minimum
  │   │   │   ├── input-0-200201-name
  │   │   │   ├── input-0-200201-size
  │   │   │   ├── input-0-200201-unit-expo
  │   │   │   ├── input-0-200201-units
  │   │   │   ├── input-0-200201-value
  │   │   ├── input-1-200202
  │   │   │   ├── input-1-200202-maximum
  │   │   │   ├── input-1-200202-minimum
  │   │   │   ├── input-1-200202-name
  │   │   │   ├── input-1-200202-size
  │   │   │   ├── input-1-200202-unit-expo
  │   │   │   ├── input-1-200202-units
  │   │   │   ├── input-1-200202-value

```
这里是一个具有四个字段的 custom sensor：两个 feature 和两个 input。
每个字段由一组属性表示。除 “value” 之外的所有字段都是只读的。value 字段是可读写字段。

```

  /sys/bus/platform/devices/HID-SENSOR-2000e1.6.auto/feature-0-200316$ grep -r . *
  feature-0-200316-maximum:6
  feature-0-200316-minimum:0
  feature-0-200316-name:property-reporting-state
  feature-0-200316-size:1
  feature-0-200316-unit-expo:0
  feature-0-200316-units:25
  feature-0-200316-value:1

```
##### 如何启用此类 sensor？


默认情况下 sensor 可以处于电源门控（power gated）状态。要启用可以通过 sysfs 属性 “enable” 

```

	$ echo 1 > enable_sensor

```
一旦启用并上电，sensor 就可以通过 HID 报告上报值。
```

	/dev$ tree | grep HID-SENSOR-2000e1.6.auto
	│   │   │   ├── 10:53 -> ../HID-SENSOR-2000e1.6.auto
	│   ├──  HID-SENSOR-2000e1.6.auto

```
每个报告可以是长度可变、前面带有头部的形式。该头部由一个 32 位的 usage id、64 位的时间戳以及 32 位的原始数据长度字段组成。

