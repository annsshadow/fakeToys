
## 内核驱动 Ampere(R) Altra(R) SMpro hwmon


支持的芯片：

  - Ampere(R) Altra(R)

    Prefix（前缀 `smpro`

    参 `Altra SoC BMC Interface Specification`

作 Thu Nguyen <thu@os.amperecomputing.com>

### 描述

smpro-hwmon 驱动基于 SMpro 协处理器（SMpro）为 Ampere(R) Altra(R) SoC 提供硬件监控支持。该驱动支持以下传感器指标：

  - 温度（temperature  - 电压（voltage  - 电流（current  - 功率（power
该接口提供用于查询各种传感器及其值的寄存器，然后由本驱动导出到用户空间
### 使用说明


该驱动为每个传感器至少创建两sysfs 文件
- `<sensor_type><idx>_label` 报告传感器标签- `<sensor_type><idx>_input` 返回传感器值
sysfs 文件分配SMpro 根目录文件夹中，每个实例对应一个根目录
SoC 关闭时，驱动读取寄存器会失败并返`-ENXIO`
### Sysfs 条目


支持以下 sysfs 文件
- Ampere(R) Altra(R)锛。
  ============    =============  ======  ===============================================
  Name            Unit           Perm    说明
  ============    =============  ======  ===============================================
  temp1_input     millicelsius   RO      SoC 温度
  temp2_input     millicelsius   RO      SoC VRD 中报告的最高温  temp2_crit      millicelsius   RO      SoC VRD HOT 阈值温  temp3_input     millicelsius   RO      DIMM VRD 中报告的最高温  temp4_input     millicelsius   RO      Core VRD 中报告的最高温  temp5_input     millicelsius   RO      CH0 DIMM0 的温  temp5_crit      millicelsius   RO      所DIMM MEM HOT 阈  temp6_input     millicelsius   RO      CH1 DIMM0 的温  temp6_crit      millicelsius   RO      所DIMM MEM HOT 阈  temp7_input     millicelsius   RO      CH2 DIMM0 的温  temp7_crit      millicelsius   RO      所DIMM MEM HOT 阈  temp8_input     millicelsius   RO      CH3 DIMM0 的温  temp8_crit      millicelsius   RO      所DIMM MEM HOT 阈  temp9_input     millicelsius   RO      CH4 DIMM0 的温  temp9_crit      millicelsius   RO      所DIMM MEM HOT 阈  temp10_input    millicelsius   RO      CH5 DIMM0 的温  temp10_crit     millicelsius   RO      所DIMM MEM HOT 阈  temp11_input    millicelsius   RO      CH6 DIMM0 的温  temp11_crit     millicelsius   RO      所DIMM MEM HOT 阈  temp12_input    millicelsius   RO      CH7 DIMM0 的温  temp12_crit     millicelsius   RO      所DIMM MEM HOT 阈  temp13_input    millicelsius   RO      RCA VRD 中报告的最高温  in0_input       millivolts     RO      Core 电压
  in1_input       millivolts     RO      SoC 电压
  in2_input       millivolts     RO      DIMM VRD1 电压
  in3_input       millivolts     RO      DIMM VRD2 电压
  in4_input       millivolts     RO      RCA VRD 电压
  cur1_input      milliamperes   RO      Core VRD 电流
  cur2_input      milliamperes   RO      SoC VRD 电流
  cur3_input      milliamperes   RO      DIMM VRD1 电流
  cur4_input      milliamperes   RO      DIMM VRD2 电流
  cur5_input      milliamperes   RO      RCA VRD 电流
  power1_input    microwatts     RO      Core VRD 功率
  power2_input    microwatts     RO      SoC VRD 功率
  power3_input    microwatts     RO      DIMM VRD1 功率
  power4_input    microwatts     RO      DIMM VRD2 功率
  power5_input    microwatts     RO      RCA VRD 功率
  ============    =============  ======  ===============================================

```

    # cat in0_input
    830
    # cat temp1_input
    37000
    # cat curr1_input
    9000
    # cat power5_input
    19500000

```
