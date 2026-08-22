### ALPS 触控板协

### 简
目前 ALPS 触控板驱动支ALPS 触控板使用的 7 个协议版本，称为版本 1 8
2010 年中期左右起，多家厂商发布了若干新型 ALPS 触控板，并被集成到各种笔记本和上网本中。这些新触控板的行为差异足够大，以至于用于描述不同版本属性的 alps_model_data 定义表已不再适用。设计决策是重新定义 alps_model_data 表（存在对现有设备进行回归测试的风险），还是将这些新设备隔离alps_model_data 表之外。最终选择了后一种设计方案。这些新触控板的签名被命名为Rushmore"Pinnacle" "Dolphin"，你可以alps.c 代码中看到它们。就本文档而言，这ALPS 触控板被统称新型 ALPS 触控
我们尝试通过探测 ACPI 接口 _HID（硬ID_CID（兼容ID）定义来唯一标识不同ALPS 变体，但似乎并不存在一一对应关系。实际上，_HID 与实际硬件类型之间呈现的m:n 映射关系
### 检

所ALPS 触控板都应当响应 "E6 report" 命令序列：E8-E6-E6-E6-E9。如果没有按键被按下，ALPS 触控板应当返00-00-0A 00-00-64。如果某些按键被按下，第一个字节的 0-2 位将1
如果 E6 报告成功，则使用 "E7 report" 序列 E8-E7-E7-E7-E9 来识别触控板型号。响应即为型号签名，会与 alps_model_data_array 中已知的型号进行匹配
对于支持协议版本 3 4 的旧款触控板，E7 报告的型号签名始终为 73-02-64。为了区分这些版本，必须检"Enter Command Mode" 序列的响应，如下所示
新型 ALPS 触控板的 E7 签名73-03-50 73-03-0A，但似乎通过 EC Command Mode 响应能更好地区分
### 命令模式


协议版本 3 4 具有一个命令模式，用于16 位地址空间中读写单字节的设备寄存器。命令序EC-EC-EC-E9 将设备置于命令模式，设备将响88-07 以及第三个字节。这第三个字节可用于确定设备使用的是版本 3 还是版本 4 协议
要退出命令模式，向触控板发PSMOUSE_CMD_SETSTREAM（EA）
在命令模式下，可以通过先发送特定命令（v3 设备EC，v4 设备F5）来设置寄存器地址。然后逐半字节（nibble）发送地址，每个半字节被编码为一个带可选数据的命令。这种编码在 v3 v4 协议之间略有不同
一旦地址被设置，就可以通过发PSMOUSE_CMD_GETINFO（E9）来读取被寻址的寄存器。响应的前两个字节包含被读取寄存器的地址，第三个字节包含寄存器的值。寄存器则通过逐半字节写入值的方式进行写入，使用与地址相同的编码
对于新型 ALPS 触控板，使用 EC 命令进入命令模式。新ALPS 触控板的响应有显著不同，并且在确定行为时更为重要。这段代码已从原始的 alps_model_data 表中分离出来，放alps_identify 函数中。例如，根据 EC 响应的第二个字节判定Dolphin" 触控板似乎有两种硬件初始化序列
### 数据包格

```

 CAPITALS = stick, miniscules = touchpad

```
'' 在不同型号上可能有不同含义，例如滚轮旋转、额外按键、双指点杆（dualpoint）上的按键等
### PS/2 数据包格

```

 byte 0:  0    0 YSGN XSGN    1    M    R    L
 byte 1: X7   X6   X5   X4   X3   X2   X1   X0
 byte 2: Y7   Y6   Y5   Y4   Y3   Y2   Y1   Y0

```
注意设备从不发出溢出（overflow）条件信号
对于协议版本 2 设备，当使用轨迹点（trackpoint）且触控板上没有手指时，M、R、L 位表示指向杆和触控板按键的组合状态
### ALPS 绝对模式 - 协议版本 1


```

 byte 0:  1    0    0    0    1   x9   x8   x7
 byte 1:  0   x6   x5   x4   x3   x2   x1   x0
 byte 2:  0    ?    ?    l    r    ?  fin  ges
 byte 3:  0    ?    ?    ?    ?   y9   y8   y7
 byte 4:  0   y6   y5   y4   y3   y2   y1   y0
 byte 5:  0   z6   z5   z4   z3   z2   z1   z0

```
### ALPS 绝对模式 - 协议版本 2


```

 byte 0:  1    ?    ?    ?    1  PSM  PSR  PSL
 byte 1:  0   x6   x5   x4   x3   x2   x1   x0
 byte 2:  0  x10   x9   x8   x7    ?  fin  ges
 byte 3:  0   y9   y8   y7    1    M    R    L
 byte 4:  0   y6   y5   y4   y3   y2   y1   y0
 byte 5:  0   z6   z5   z4   z3   z2   z1   z0

```
协议版本 2 DualPoint 设备DualPoint Stick 发送标PS/2 鼠标数据包。M、R L 位表示指向杆和触控板按键的组合状态，Dell 双点设备除外，其指向杆按键会PSM、PSR PSL 位中单独报告
### Dualpoint 设备 -- 交错数据包格

```

 byte 0:    1    1    0    0    1    1    1    1
 byte 1:    0   x6   x5   x4   x3   x2   x1   x0
 byte 2:    0  x10   x9   x8   x7    0  fin  ges
 byte 3:    0    0 YSGN XSGN    1    1    1    1
 byte 4:   X7   X6   X5   X4   X3   X2   X1   X0
 byte 5:   Y7   Y6   Y5   Y4   Y3   Y2   Y1   Y0
 byte 6:    0   y9   y8   y7    1    m    r    l
 byte 7:    0   y6   y5   y4   y3   y2   y1   y0
 byte 8:    0   z6   z5   z4   z3   z2   z1   z0

```
使用交错格式的设备通常会为 DualPoint Stick 发送标PS/2 鼠标数据包，并为触控板发ALPS 绝对模式数据包，当指向杆和触控板同时使用时切换到交错数据包格式
### ALPS 绝对模式 - 协议版本 3


ALPS 协议版本 3 有三种不同的数据包格式。前两种与触控板事件相关，第三种与轨迹点事件相关
```

 byte 0:    1    ?   x1   x0    1    1    1    1
 byte 1:    0  x10   x9   x8   x7   x6   x5   x4
 byte 2:    0  y10   y9   y8   y7   y6   y5   y4
 byte 3:    0    M    R    L    1    m    r    l
 byte 4:    0   mt   x3   x2   y3   y2   y1   y0
 byte 5:    0   z6   z5   z4   z3   z2   z1   z0

```
注意，对于某些设备，轨迹点按键在此数据包中报告，而在另一些设备上则在轨迹点数据包中报告
第二个数据包类型包含表示 x y 轴的位图。在位图中，如果某个手指覆盖了该轴上的某个位置，则对应位被置位。因此位图数据包可用于低分辨率多点触控数据，尽管无法跟踪手指。该数据包还编码
```

 byte 0:    1    1   x1   x0    1    1    1    1
 byte 1:    0   x8   x7   x6   x5   x4   x3   x2
 byte 2:    0   y7   y6   y5   y4   y3   y2   y1
 byte 3:    0  y10   y9   y8    1    1    1    1
 byte 4:    0  x14  x13  x12  x11  x10   x9   y0
 byte 5:    0    1    ?    ?    ?    ?   f1   f0

```
此数据包仅在 mt 位被置位的定位数据包之后出现，并且通常只在一个或两个以上的触点时出现（尽管偶尔也会在只有单个触点时看到）
```

 byte 0:    1    1   x7   y7    1    1    1    1
 byte 1:    0   x6   x5   x4   x3   x2   x1   x0
 byte 2:    0   y6   y5   y4   y3   y2   y1   y0
 byte 3:    0    1   TP   SW    1    M    R    L
 byte 4:    0   z6   z5   z4   z3   z2   z1   z0
 byte 5:    0    0    1    1    1    1    1    1

```
TP 表示启用点击处理时的点击（Tap）SW 状态，或启用按压处理时的按压（Press）状态。SW 表示在有 4 个按键可用时的向上滚动（scroll up）
### ALPS 绝对模式 - 协议版本 4


```

 byte 0:    1    ?   x1   x0    1    1    1    1
 byte 1:    0  x10   x9   x8   x7   x6   x5   x4
 byte 2:    0  y10   y9   y8   y7   y6   y5   y4
 byte 3:    0    1   x3   x2   y3   y2   y1   y0
 byte 4:    0    ?    ?    ?    1    ?    r    l
 byte 5:    0   z6   z5   z4   z3   z2   z1   z0
 byte 6:    bitmap data (described below)
 byte 7:    bitmap data (described below)

```
最后两个字节表示一个部分位图数据包，需3 个完整数据包才能构造出一个完整的位图数据包。一旦组装完成，6 字节
```

 byte 0:    0    1   x7   x6   x5   x4   x3   x2
 byte 1:    0   x1   x0   y4   y3   y2   y1   y0
 byte 2:    0    0    ?  x14  x13  x12  x11  x10
 byte 3:    0   x9   x8   y9   y8   y7   y6   y5
 byte 4:    0    0    0    0    0    0    0    0
 byte 5:    0    0    0    0    0    0    0  y10

```
这里有几点值得注意
 1) 在位图数据中，字0 的第 6 位用作同步字节（sync byte），用于标识位图数据包的第一个片段
 2) 位图表示的数据与 v3 位图数据包相同，尽管数据包布局不同
 3) v4 协议数据包中似乎没有任何位置包含触点计数。触点计数的推导必须通过解析位图来完成
 4) 定位数据包与位图数据包的比例3:1。因MT 位置只能每三ST 位置更新时更新一次，触点计数也只能每三个数据包更新一次
到目前为止尚未遇到带有轨迹点v4 设备
### ALPS 绝对模式 - 协议版本 5

这基本上就是协议版本 3，但使用了不同的数据包解码逻辑。它使用相同alps_process_touchpad_packet_v3 调用，配合一个专门的解码字段（decode_fields）函数指针来正确解释数据包。这似乎仅被 Dolphin 设备使用
```

 byte 0:    1    1    0    0    1    0    0    0
 byte 1:    0   x6   x5   x4   x3   x2   x1   x0
 byte 2:    0   y6   y5   y4   y3   y2   y1   y0
 byte 3:    0    M    R    L    1    m    r    l
 byte 4:   y10  y9   y8   y7  x10   x9   x8   x7
 byte 5:    0   z6   z5   z4   z3   z2   z1   z0

```

```

 byte 0:    1    1    1    n3   1   n2   n1   x24
 byte 1:    1   y7   y6    y5  y4   y3   y2    y1
 byte 2:    ?   x2   x1   y12 y11  y10   y9    y8
 byte 3:    0  x23  x22   x21 x20  x19  x18   x17
 byte 4:    0   x9   x8    x7  x6   x5   x4    x3
 byte 5:    0  x16  x15   x14 x13  x12  x11   x10

```
### ALPS 绝对模式 - 协议版本 6


```

 byte 0:    1    1    1    1    1    1    1    1
 byte 1:    0   X6   X5   X4   X3   X2   X1   X0
 byte 2:    0   Y6   Y5   Y4   Y3   Y2   Y1   Y0
 byte 3:    ?   Y7   X7    ?    ?    M    R    L
 byte 4:   Z7   Z6   Z5   Z4   Z3   Z2   Z1   Z0
 byte 5:    0    1    1    1    1    1    1    1

```

```

 byte 0:    1    1    1    1    1    1    1    1
 byte 1:    0    0    0    0   x3   x2   x1   x0
 byte 2:    0    0    0    0   y3   y2   y1   y0
 byte 3:    ?   x7   x6   x5   x4    ?    r    l
 byte 4:    ?   y7   y6   y5   y4    ?    ?    ?
 byte 5:   z7   z6   z5   z4   z3   z2   z1   z0

```
（v6 触控板没有中键）

### ALPS 绝对模式 - 协议版本 7


```

 byte 0:    0    1    0    0    1    0    0    0
 byte 1:    1    1    *    *    1    M    R    L
 byte 2:   X7    1   X5   X4   X3   X2   X1   X0
 byte 3:   Z6    1   Y6   X6    1   Y2   Y1   Y0
 byte 4:   Y7    0   Y5   Y4   Y3    1    1    0
 byte 5:  T&P    0   Z5   Z4   Z3   Z2   Z1   Z0

```

```

         packet-fmt     b7     b6     b5     b4     b3     b2     b1     b0
 byte 0: TWO & MULTI     L      1      R      M      1   Y0-2   Y0-1   Y0-0
 byte 0: NEW             L      1   X1-5      1      1   Y0-2   Y0-1   Y0-0
 byte 1:             Y0-10   Y0-9   Y0-8   Y0-7   Y0-6   Y0-5   Y0-4   Y0-3
 byte 2:             X0-11      1  X0-10   X0-9   X0-8   X0-7   X0-6   X0-5
 byte 3:             X1-11      1   X0-4   X0-3      1   X0-2   X0-1   X0-0
 byte 4: TWO         X1-10    TWO   X1-9   X1-8   X1-7   X1-6   X1-5   X1-4
 byte 4: MULTI       X1-10    TWO   X1-9   X1-8   X1-7   X1-6   Y1-5      1
 byte 4: NEW         X1-10    TWO   X1-9   X1-8   X1-7   X1-6      0      0
 byte 5: TWO & NEW   Y1-10      0   Y1-9   Y1-8   Y1-7   Y1-6   Y1-5   Y1-4
 byte 5: MULTI       Y1-10      0   Y1-9   Y1-8   Y1-7   Y1-6    F-1    F-0

```
 L:         左键
 R / M:     非点击板（Non-clickpads）：右键 / 中键
            点击板（Clickpads）：当按下超2 个手指，且部分手指位于按键区域时，报告的两个坐标对应的是按键区域之外的手指，并且这些会被报告为右/ 左键区域中存在额外的手指。注意这些手指不会被计入 F 字段！因此，如果收到一TWO 数据包且 R = 1，则3 个手指按下，依此类推 TWO:       1：存在两次触摸，字节 0/4/5 采用 TWO 格式
            0：如果字4 的第 0 位为 1，则字节 0/4/5 采用 MULTI 格式；否则字0 的第 4 位必须被置位，且字节 0/4/5 采用 NEW 格式
 F:         手指数量3 表示 3 个手指，1 表示 4 个…

### ALPS 绝对模式 - 协议版本 8


SS43 03 14）和 SS53 03 28）硬件发出
数据包类型由 APD 字段给出，即字节 3 的第 4-5 位
```

           b7   b6   b5   b4   b3   b2   b1   b0
 byte 0:  SWM  SWR  SWL    1    1    0    0   X7
 byte 1:    0   X6   X5   X4   X3   X2   X1   X0
 byte 2:    0   Y6   Y5   Y4   Y3   Y2   Y1   Y0
 byte 3:    0  T&P    1    0    1    0    0   Y7
 byte 4:    0   Z6   Z5   Z4   Z3   Z2   Z1   Z0
 byte 5:    0    0    0    0    0    0    0    0

```
SWM、SWR、SWL：中键、右键和左键的状
```

           b7   b6   b5   b4   b3   b2   b1   b0
 byte 0:  SWM  SWR  SWL    1    1   X2   X1   X0
 byte 1:   X9   X8   X7    1   X6   X5   X4   X3
 byte 2:    0  X11  X10  LFB   Y3   Y2   Y1   Y0
 byte 3:   Y5   Y4    0    0    1 TAPF2 TAPF1 TAPF0
 byte 4:  Zv7  Y11  Y10    1   Y9   Y8   Y7   Y6
 byte 5:  Zv6  Zv5  Zv4    0  Zv3  Zv2  Zv1  Zv0

```
TAPF: ???
LFB:  ???

```

           b7   b6   b5   b4   b3   b2   b1   b0
 byte 0:  SWM  SWR  SWL    1    1  AX6  AX5  AX4
 byte 1: AX11 AX10  AX9  AX8  AX7  AZ1  AY4  AZ0
 byte 2: AY11 AY10  AY9  CONT AY8  AY7  AY6  AY5
 byte 3:    0    0    0    1    1  BX6  BX5  BX4
 byte 4: BX11 BX10  BX9  BX8  BX7  BZ1  BY4  BZ0
 byte 5: BY11 BY10  BY9    0  BY8  BY7  BY5  BY5

```
CONT: 紧随其后的是一3 4 指的数据
```

           b7   b6   b5   b4   b3   b2   b1   b0
 byte 0:  SWM  SWR  SWL    1    1  AX6  AX5  AX4
 byte 1: AX11 AX10  AX9  AX8  AX7  AZ1  AY4  AZ0
 byte 2: AY11 AY10  AY9  OVF  AY8  AY7  AY6  AY5
 byte 3:    0    0    1    1    1  BX6  BX5  BX4
 byte 4: BX11 BX10  BX9  BX8  BX7  BZ1  BY4  BZ0
 byte 5: BY11 BY10  BY9    0  BY8  BY7  BY5  BY5

```
OVF: 检测到5 个手