
## 内核驱动 gpd-fan


Author:
    - Cryolitia PukNgae <cryolitia@uniontech.com>

### 描述


深圳 GPD 科技有限公司（Shenzhen GPD Technology Co., Ltd.）的手持设备通过其嵌入式控制器提供风扇读数和风扇控制。

### 支持的设备


目前该驱动支持以下手持设备：

 - GPD Win Mini (7840U)
 - GPD Win Mini (8840U)
 - GPD Win Mini (HX370)
 - GPD Pocket 4
 - GPD Duo
 - GPD Win Max 2 (6800U)
 - GPD Win Max 2 2023 (7840U)
 - GPD Win Max 2 2024 (8840U)
 - GPD Win Max 2 2025 (HX370)
 - GPD Win 4 (6800U)
 - GPD Win 4 (7840U)
 - GPD Micro PC 2

### 模块参数


gpd_fan_board
  强制指定应使用哪个模块 quirk。
  使用方式如 "gpd_fan_board=wm2"。

   - wm2
       - GPD Win 4 (7840U)
       - GPD Win Max 2 (6800U)
       - GPD Win Max 2 2023 (7840U)
       - GPD Win Max 2 2024 (8840U)
       - GPD Win Max 2 2025 (HX370)
   - win4
       - GPD Win 4 (6800U)
   - win_mini
       - GPD Win Mini (7840U)
       - GPD Win Mini (8840U)
       - GPD Win Mini (HX370)
       - GPD Pocket 4
       - GPD Duo
   - mpc2
       - GPD Micro PC 2

### Sysfs 接口


支持以下属性：

fan1_input
  只读。读取当前风扇转速（RPM）。

pwm1_enable
  读/写。启用手动风扇控制。写入 "0" 禁用控制并以全速运行。写入 "1" 设为手动模式，写入 "2" 由 EC 控制来决定风扇转速。读取该属性可查看当前状态。

  NB：出于设备安全考虑，当设置为手动模式时，pwm 速度默认会被设为最大值（255）。你可以通过随后写入 pwm1 来设置不同的值。

pwm1
  读/写。读取该属性可查看当前占空比，范围为 [0-255]。当 pwm1_enable 设为 "1"（手动）时，写入 [0-255] 范围内的任意值来设置风扇转速。

  NB：许多主板（上述 wm2 列表之外的）不支持在自动模式下读取当前 pwm 值，那将只返回 EOPNOTSUPP。在手动模式下则始终返回真实值。
