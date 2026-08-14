
## 内核驱动 xdpe152


支持的设备：

  - Infineon XDPE152C4

    Prefix: 'xdpe152c4'

  - Infineon XDPE15284

    Prefix: 'xdpe15284'

Authors:

    Greg Schwendimann <greg.schwendimann@infineon.com>

### 描述


本驱动实现了对 Infineon 数字多相控制器 XDPE152C4 与 XDPE15284 双环路
电压调节器的支持。
这些器件符合：

- Intel VR13、VR13HC 与 VR14 rev 1.86 转换器规范。
- Intel SVID rev 1.93 协议。
- PMBus rev 1.3.1 接口。

器件支持用线性格式读取输入与输出电压、输入与输出电流、输入与输出功率
以及温度。

器件支持两个遥测页（page）。

对于电流，驱动提供：输入、最大值与严重阈值，以及最大值与严重告警。
低严重阈值与低严重告警仅对电流输出受支持。
驱动通过 sysfs 文件导出以下属性，其中索引 1、2 对应 “iin”，3、4 对应 “iout”：

**curr[1-4]_crit**

**curr[1-4]_crit_alarm**

**curr[1-4]_input**

**curr[1-4]_label**

**curr[1-4]_max**

**curr[1-4]_max_alarm**

**curr[3-4]_lcrit**

**curr[3-4]_lcrit_alarm**

**curr[3-4]_rated_max**

对于电压，驱动提供：输入、严重与低严重阈值，以及严重与低严重告警。
驱动通过 sysfs 文件导出以下属性，其中索引 1、2 对应 “vin”，3、4 对应 “vout”：

**in[1-4]_min**

**in[1-4]_crit**

**in[1-4_crit_alarm**

**in[1-4]_input**

**in[1-4]_label**

**in[1-4]_max**

**in[1-4]_max_alarm**

**in[1-4]_min**

**in[1-4]_min_alarm**

**in[3-4]_lcrit**

**in[3-4]_lcrit_alarm**

**in[3-4]_rated_max**

**in[3-4]_rated_min**

对于功率，驱动提供：输入与告警。
驱动通过 sysfs 文件导出以下属性，其中索引 1、2 对应 “pin”，3、4 对应 “pout”：

**power[1-2]_alarm**

**power[1-4]_input**

**power[1-4]_label**

**power[1-4]_max**

**power[1-4]_rated_max**

对于温度，驱动提供：输入、最大值与严重阈值，以及最大值与严重告警。
驱动通过 sysfs 文件导出以下属性：

**temp[1-2]_crit**

**temp[1-2]_crit_alarm**

**temp[1-2]_input**

**temp[1-2]_max**

**temp[1-2]_max_alarm**
