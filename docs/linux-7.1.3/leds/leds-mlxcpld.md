## Mellanox 系统 LED 的内核驱

为以Mellanox 系统提供系统 LED 支持"msx6710"msx6720"msb7700"msn2700"msx1410""msn2410"msb7800"msn2740"msn2100"
### 描述


驱动为系"msx6710"msx6720"msb7700"msn2700""msx1410"msn2410"msb7800"msn2740" 提供以下 LED
  - mlxcpld:fan1:green
  - mlxcpld:fan1:red
  - mlxcpld:fan2:green
  - mlxcpld:fan2:red
  - mlxcpld:fan3:green
  - mlxcpld:fan3:red
  - mlxcpld:fan4:green
  - mlxcpld:fan4:red
  - mlxcpld:psu:green
  - mlxcpld:psu:red
  - mlxcpld:status:green
  - mlxcpld:status:red

 "status"
  - CPLD 寄存器偏移：0x20
  - 浣?[3:0]

 "psu"
  - CPLD 寄存器偏移：0x20
  - 浣?[7:4]

 "fan1"
  - CPLD 寄存器偏移：0x21
  - 浣?[3:0]

 "fan2"
  - CPLD 寄存器偏移：0x21
  - 浣?[7:4]

 "fan3"
  - CPLD 寄存器偏移：0x22
  - 浣?[3:0]

 "fan4"
  - CPLD 寄存器偏移：0x22
  - 浣?[7:4]

 上述所LED 的颜色掩码：

  [bit3,bit2,bit1,bit0] 鎴?  [bit7,bit6,bit5,bit4]锛。
 - [0,0,0,0] = LED 关闭
 - [0,1,0,1] = 红色常亮
 - [1,1,0,1] = 绿色常亮
 - [0,1,1,0] = 红色闪烁 3Hz
 - [1,1,1,0] = 绿色闪烁 3Hz
 - [0,1,1,1] = 红色闪烁 6Hz
 - [1,1,1,1] = 绿色闪烁 6Hz

驱动为系"msn2100" 提供以下 LED
  - mlxcpld:fan:green
  - mlxcpld:fan:red
  - mlxcpld:psu1:green
  - mlxcpld:psu1:red
  - mlxcpld:psu2:green
  - mlxcpld:psu2:red
  - mlxcpld:status:green
  - mlxcpld:status:red
  - mlxcpld:uid:blue

 "status"
  - CPLD 寄存器偏移：0x20
  - 浣?[3:0]

 "fan"
  - CPLD 寄存器偏移：0x21
  - 浣?[3:0]

 "psu1"
  - CPLD 寄存器偏移：0x23
  - 浣?[3:0]

 "psu2"
  - CPLD 寄存器偏移：0x23
  - 浣?[7:4]

 "uid"
  - CPLD 寄存器偏移：0x24
  - 浣?[3:0]

 uid 外，上述所LED 的颜色掩码：

  [bit3,bit2,bit1,bit0] 鎴?  [bit7,bit6,bit5,bit4]锛。
 - [0,0,0,0] = LED 关闭
 - [0,1,0,1] = 红色常亮
 - [1,1,0,1] = 绿色常亮
 - [0,1,1,0] = 红色闪烁 3Hz
 - [1,1,1,0] = 绿色闪烁 3Hz
 - [0,1,1,1] = 红色闪烁 6Hz
 - [1,1,1,1] = 绿色闪烁 6Hz

 uid LED 的颜色掩码：
  [bit3,bit2,bit1,bit0]锛。
 - [0,0,0,0] = LED 关闭
 - [1,1,0,1] = 蓝色常亮
 - [1,1,1,0] = 蓝色闪烁 3Hz
 - [1,1,1,1] = 蓝色闪烁 6Hz

驱动支持 3Hz 6Hz 频率0% 占空比）的硬件闪烁3Hz 时占空比周期约为 167 毫秒Hz 时约83 毫秒