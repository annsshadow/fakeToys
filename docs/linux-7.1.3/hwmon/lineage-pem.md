## 内核驱动 lineage-pem


支持的设备：

  - Lineage Compact Power Line 电源入口模块

    Prefix: 'lineage-pem'

    Addresses scanned: -

    Documentation:

	http://www.lineagepower.com/oem/pdf/CPLI2C.pdf

Author: Guenter Roeck <linux@roeck-us.net>


### 描述


本驱动支持多Lineage Compact Power Line DC/DC AC/DC
转换器，例如 CP1800、CP2000AC、CP2000DC、CP2100DC 等
Lineage CPL 电源入口模块名义上兼PMBus。然而，大多标准 PMBus 命令并不受支持。具体而言，所有硬件监与状态上报命令都是非标准的。因此，无法使用标准PMBus 驱动

### 使用注意


本驱动不会探Lineage CPL 设备，因为没有可供安全用识别芯片的寄存器。你必须显式实例化这些设备
示例：以下命令将为地址 0x40 处的 Lineage PEM 加载驱动

```

	$ modprobe lineage-pem
	$ echo lineage-pem 0x40 > /sys/bus/i2c/devices/i2c-1/new_device

```
所Lineage CPL 电源入口模块都内置了一I2C 总线主选择（PCA9541）。为确保设备访问，本驱动只能作为
pca9541 I2C 主选择器驱动的客户端驱动使用

### Sysfs 条目


所Lineage CPL 设备都会上报输出电压与设备温度，以及
输出电压、温度、输入电压、输入电流、输入功率和风扇状态的告警
输入电压、输入电流、输入功率和风扇转速测量仅在新款设备上
受支持。驱动会检测这些属性是否受支持，并仅在受支持时
创建相应sysfs 条目
======================= ===============================
in1_input		输出电压（mVin1_min_alarm		输出欠压告警
in1_max_alarm		输出过压告警
in1_crit		输出电压严重告警

in2_input		输入电压（mV，可选）
in2_alarm		输入电压告警

curr1_input		输入电流（mA，可选）
curr1_alarm		输入过流告警

power1_input		输入功率（uW，可选）
power1_alarm		输入功率告警

fan1_input		风扇 1 转速（rpm，可选）
fan2_input		风扇 2 转速（rpm，可选）
fan3_input		风扇 3 转速（rpm，可选）

temp1_input
temp1_max
temp1_crit
temp1_alarm
temp1_crit_alarm
temp1_fault
======================= ===============================
