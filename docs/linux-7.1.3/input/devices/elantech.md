## Elantech 触控板驱

	版权所(C) 2007-2008 Arjan Opmeer <arjan@opmeer.net>

	硬件版本 1 的额外信息由 Steve Havelka 发现	提供

	版本 2（EeePC）硬件支持基于从 Xandros Woody 	收到并经eeeuser.com 论坛用户 StewieGriffin
	转发给我的补

 1. 简 2. 额外旋钮
 3. 区分硬件版本
 4. 硬件版本 1
    4.1 寄存    4.2 原生相对模式 4 字节数据包格    4.3 原生绝对模式 4 字节数据包格 5. 硬件版本 2
    5.1 寄存    5.2 原生绝对模式 6 字节数据包格        5.2.1 奇偶校验与数据包重新同步
        5.2.2 单指/三指触摸
        5.2.3 双指触摸
 6. 硬件版本 3
    6.1 寄存    6.2 原生绝对模式 6 字节数据包格        6.2.1 单指/三指触摸
        6.2.2 双指触摸
 7. 硬件版本 4
    7.1 寄存    7.2 原生绝对模式 6 字节数据包格        7.2.1 状态数据包
        7.2.2 头部数据        7.2.3 运动数据 8. 指点杆（适用于硬件版3 4    8.1 寄存    8.2 原生相对模式 6 字节数据包格        8.2.1 状态数据包



#### 简

目前 Linux Elantech 触控板驱动能够识别四种不同的硬件版本，分别被平淡地称版本 1、版2、版3 和版4。版1 出现在“较旧”的笔记本电脑中，每个数据包
使用 4 字节。版2 似乎EeePC 引入，每个数据包使用 6 字节，并提供额外的特性，
例如双指的位置和触摸宽度。硬件版3 每个数据包使6 字节（对于双指则是两6 字节数据包的拼接），并允许跟踪最3 根手指。硬件版4 每个数据包使6 字节并且可以将一个状态数据包与多个头部或运动数据包组合在一起。硬件版4 允许跟踪
最5 根手指
某些硬件版本 3 和版4 还带有一个指点杆（trackpoint），它使用单独的包格式。每数据包也6 字节
该驱动尝试同时支持这些硬件版本，并且应当Xorg Synaptics 触控板驱动及其图形化
配置工具兼容
请注意，当存在指点杆时，鼠标按键也与触控板或指点杆相关联。在 xorg 中禁用触控板
（TouchPadOff=0）也会禁用与触控板关联的按键
此外，可以通过调整触控板某些内部寄存器的内容来改变其操作。这些寄存器由驱动表示为
/sys/bus/serio/drivers/psmouse/serio 下的 sysfs 条目，可以被读取和写入
目前只有硬件版本 1 的寄存器在一定程度上被理解。硬件版2 似乎使用了其中一些相的寄存器，但尚不清楚寄存器中的位是否表示相同的含义，或者含义可能已经改变
除此之外，某些寄存器设置仅在触控板处于相对模式而非绝对模式时生效。由Linux
Elantech 触控板驱动始终将硬件置于绝对模式，下面提到的并非所有信息都能立即使用但由于没有可自由获取Elantech 文档，为了完整性，这里仍提供这些信息

#### 额外旋钮


目前 Linux Elantech 触控板驱动在 /sys/bus/serio/drivers/psmouse/serio? 下为用户
提供三个额外的旋钮
- debug（调试）

   打开或关闭不同级别的调试
   向该文件回显 "0" 将关闭所有调试
   目前值为 "1" 将打开一些基本调试，值为 "2" 将打开数据包调试。对于硬件版1   默认是关闭。对于版2，默认是 "1"
   打开数据包调试会使驱动在处理每个接收到的数据包之前将其转储到系统日志。请注意   这可能会产生相当多的数据
- paritycheck（奇偶校验）

   打开或关闭奇偶校验
   向该文件回显 "0" 将关闭奇偶校验。任何非 0 的值都会将其打开。对于硬件版1   默认是打开。对于版2，默认是关闭
   硬件版本 1 通过为每个数据包的最3 字节计算一个奇偶位来提供基本的数据完整   校验。驱动可以检查这些位并拒绝任何看起来已损坏的数据包。使用此旋钮可以绕过   检查
   硬件版本 2 不提供相同的奇偶位。只能进行一些基本的数据一致性检查。目前默认禁   检查。即使现在打开它也不会起任何作用
- crc_enabled

   crc_enabled 设置0/1。名“crc_enabled是此完整性检查的官方名称，尽   它并不是真正的循环冗余校验
   根据 crc_enabled 的状态，驱动会对硬件版本 3 4 进行一些基本的数据完整性校验   驱动会拒绝任何看起来已损坏的数据包。使用此旋钮可以改变 crc_enabled 的状态
   读取 crc_enabled 值将显示当前生效的值。向该文件回"0" "1" 会将状态设置为
   "0" 鎴?"1"銆。

#### 区分硬件版本


```

 4 bytes version: (after the arrow is the name given in the Dell-provided driver)
 02.00.22 => EF013
 02.06.00 => EF019

```
在实际使用中，似乎还有更多版本，例如 00.01.641.00.21```

 6 bytes:
 02.00.30 => EF113
 02.08.00 => EF023
 02.08.XX => EF123
 02.0B.00 => EF215
 04.01.XX => Scroll_EF051
 04.02.XX => EF051

```
在实际使用中，似乎还有更多版本，例如 04.03.014.04.11。除EF113 之外，它似乎几乎没有区别，EF113 不报告压宽度，并且具有不同的数据一致性检查
可能所param[^0^] <= 01 的版本都可以视为 4 字节/固件 1。版< 02.08.00（除02.00.30）视4 字节/固件 2。所>= 02.08.00 的版本都可以视为 6 字节

#### 硬件版本 1


### 瀵勫瓨鍣?

通过向寄存器回显一个十六进制值可以改变其内容
```

   echo -n 0x16 > reg_10

```
```

   bit   7   6   5   4   3   2   1   0
         B   C   T   D   L   A   S   E

         E: 1 = enable smart edges unconditionally
         S: 1 = enable smart edges only when dragging
         A: 1 = absolute mode (needs 4 byte packets, see reg_11)
         L: 1 = enable drag lock (see reg_22)
         D: 1 = disable dynamic resolution
         T: 1 = disable tapping
         C: 1 = enable corner tap
         B: 1 = swap left and right button

```
```

   bit   7   6   5   4   3   2   1   0
         1   0   0   H   V   1   F   P

         P: 1 = enable parity checking for relative mode
         F: 1 = enable native 4 byte packet mode
         V: 1 = enable vertical scroll area
         H: 1 = enable horizontal scroll area

```
```

         single finger width?

```
```

         scroll area width (small: 0x40 ... wide: 0xff)

```
```

         drag lock time out (short: 0x14 ... long: 0xfe;
                             0xff = tap again to release)

```
```

         tap make timeout?

```
```

         tap release timeout?

```
```

         smart edge cursor speed (0x02 = slow, 0x03 = medium, 0x04 = fast)

```
```

         smart edge activation area width?


```
### 原生相对模式 4 字节数据包格

```

   bit   7   6   5   4   3   2   1   0
         c   c  p2  p1   1   M   R   L

         L, R, M = 1 when Left, Right, Middle mouse button pressed
            some models have M as byte 3 odd parity bit
         when parity checking is enabled (reg_11, P = 1):
            p1..p2 = byte 1 and 2 odd parity bit
         c = 1 when corner tap detected

```
```

   bit   7   6   5   4   3   2   1   0
        dx7 dx6 dx5 dx4 dx3 dx2 dx1 dx0

         dx7..dx0 = x movement;   positive = right, negative = left
         byte 1 = 0xf0 when corner tap detected

```
```

   bit   7   6   5   4   3   2   1   0
        dy7 dy6 dy5 dy4 dy3 dy2 dy1 dy0

         dy7..dy0 = y movement;   positive = up,    negative = down

```
```

   parity checking enabled (reg_11, P = 1):

      bit   7   6   5   4   3   2   1   0
            w   h  n1  n0  ds3 ds2 ds1 ds0

            normally:
               ds3..ds0 = scroll wheel amount and direction
                          positive = down or left
                          negative = up or right
            when corner tap detected:
               ds0 = 1 when top right corner tapped
               ds1 = 1 when bottom right corner tapped
               ds2 = 1 when bottom left corner tapped
               ds3 = 1 when top left corner tapped
            n1..n0 = number of fingers on touchpad
               only models with firmware 2.x report this, models with
               firmware 1.x seem to map one, two and three finger taps
               directly to L, M and R mouse buttons
            h = 1 when horizontal scroll action
            w = 1 when wide finger touch?

   otherwise (reg_11, P = 0):

      bit   7   6   5   4   3   2   1   0
           ds7 ds6 ds5 ds4 ds3 ds2 ds1 ds0

            ds7..ds0 = vertical scroll amount and direction
                       negative = up
                       positive = down


```
### 原生绝对模式 4 字节数据包格

EF013 EF019 有一种特殊行为（由于固件中的缺陷？），当有一根手指触摸时，前 2 位置报告必须被丢弃。每当报告不同数量的手指时，此计数会重置
```

   firmware version 1.x:

      bit   7   6   5   4   3   2   1   0
            D   U  p1  p2   1  p3   R   L

            L, R = 1 when Left, Right mouse button pressed
            p1..p3 = byte 1..3 odd parity bit
            D, U = 1 when rocker switch pressed Up, Down

   firmware version 2.x:

      bit   7   6   5   4   3   2   1   0
           n1  n0  p2  p1   1  p3   R   L

            L, R = 1 when Left, Right mouse button pressed
            p1..p3 = byte 1..3 odd parity bit
            n1..n0 = number of fingers on touchpad

```
```

   firmware version 1.x:

      bit   7   6   5   4   3   2   1   0
            f   0  th  tw  x9  x8  y9  y8

            tw = 1 when two finger touch
            th = 1 when three finger touch
            f  = 1 when finger touch

   firmware version 2.x:

      bit   7   6   5   4   3   2   1   0
            .   .   .   .  x9  x8  y9  y8

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

         x9..x0 = absolute x value (horizontal)

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

         y9..y0 = absolute y value (vertical)


```
#### 硬件版本 2



### 瀵勫瓨鍣?

通过向寄存器回显一个十六进制值可以改变其内容
```

   echo -n 0x56 > reg_10

```
```

   bit   7   6   5   4   3   2   1   0
         0   1   0   1   0   1   D   0

         D: 1 = enable drag and drop

```
```

   bit   7   6   5   4   3   2   1   0
         1   0   0   0   S   0   1   0

         S: 1 = enable vertical scroll

```
```

         unknown (0x00)

```
```

         drag and drop release time out (short: 0x70 ... long 0x7e;
                                   0x7f = never i.e. tap again to release)


```
### 原生绝对模式 6 字节数据包格

##### 奇偶校验与数据包重新同步


没有奇偶校验，但是可以执行一些一致性检查
```

        SA1= packet[0];
        A1 = packet[1];
        B1 = packet[2];
        SB1= packet[3];
        C1 = packet[4];
        D1 = packet[5];
        if( (((SA1 & 0x3C) != 0x3C) && ((SA1 & 0xC0) != 0x80)) || // check Byte 1
            (((SA1 & 0x0C) != 0x0C) && ((SA1 & 0xC0) == 0x80)) || // check Byte 1 (one finger pressed)
            (((SA1 & 0xC0) != 0x80) && (( A1 & 0xF0) != 0x00)) || // check Byte 2
            (((SB1 & 0x3E) != 0x38) && ((SA1 & 0xC0) != 0x80)) || // check Byte 4
            (((SB1 & 0x0E) != 0x08) && ((SA1 & 0xC0) == 0x80)) || // check Byte 4 (one finger pressed)
            (((SA1 & 0xC0) != 0x80) && (( C1 & 0xF0) != 0x00))  ) // check Byte 5
		// error detected

```
```

        if( ((packet[0] & 0x0C) != 0x04) ||
            ((packet[3] & 0x0f) != 0x02) )
		// error detected


```
如果检测到错误，所有数据包都会向前移动一个字节（并丢packet[^0^]）
##### 单指/三指触摸


```

   bit   7   6   5   4   3   2   1   0
	 n1  n0  w3  w2   .   .   R   L

         L, R = 1 when Left, Right mouse button pressed
         n1..n0 = number of fingers on touchpad

```
```

   bit   7   6   5   4   3   2   1   0
	 p7  p6  p5  p4 x11 x10 x9  x8

```
```

   bit   7   6   5   4   3   2   1   0
	 x7  x6  x5  x4  x3  x2  x1  x0

         x11..x0 = absolute x value (horizontal)

```
```

   bit   7   6   5   4   3   2   1   0
	 n4  vf  w1  w0   .   .   .  b2

	 n4 = set if more than 3 fingers (only in 3 fingers mode)
	 vf = a kind of flag ? (only on EF123, 0 when finger is over one
	      of the buttons, 1 otherwise)
	 w3..w0 = width of the finger touch (not EF113)
	 b2 (on EF113 only, 0 otherwise), b2.R.L indicates one button pressed:
		0 = none
		1 = Left
		2 = Right
		3 = Middle (Left and Right)
		4 = Forward
		5 = Back
		6 = Another one
		7 = Another one

```
```

   bit   7   6   5   4   3   2   1   0
        p3  p1  p2  p0  y11 y10 y9  y8

	 p7..p0 = pressure (not EF113)

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

         y11..y0 = absolute y value (vertical)


```
##### 双指触摸


请注意，这两对坐标并不完全是两根手指的坐标，而只是左下角和右上角坐标这一对因此实际的手指可能位于由这两个点定义的正方形的另一条对角线上
```

   bit   7   6   5   4   3   2   1   0
        n1  n0  ay8 ax8  .   .   R   L

         L, R = 1 when Left, Right mouse button pressed
         n1..n0 = number of fingers on touchpad

```
```

   bit   7   6   5   4   3   2   1   0
        ax7 ax6 ax5 ax4 ax3 ax2 ax1 ax0

	 ax8..ax0 = lower-left finger absolute x value

```
```

   bit   7   6   5   4   3   2   1   0
        ay7 ay6 ay5 ay4 ay3 ay2 ay1 ay0

	 ay8..ay0 = lower-left finger absolute y value

```
```

   bit   7   6   5   4   3   2   1   0
         .   .  by8 bx8  .   .   .   .

```
```

   bit   7   6   5   4   3   2   1   0
        bx7 bx6 bx5 bx4 bx3 bx2 bx1 bx0

         bx8..bx0 = upper-right finger absolute x value

```
```

   bit   7   6   5   4   3   2   1   0
        by7 by8 by5 by4 by3 by2 by1 by0

         by8..by0 = upper-right finger absolute y value

```
#### 硬件版本 3



### 瀵勫瓨鍣?

```

   bit   7   6   5   4   3   2   1   0
         0   0   0   0   R   F   T   A

         A: 1 = enable absolute tracking
         T: 1 = enable two finger mode auto correct
         F: 1 = disable ABS Position Filter
         R: 1 = enable real hardware resolution

```
### 原生绝对模式 6 字节数据包格

单指和三指触摸共享相同的 6 字节数据包格式，不同之处在于三指触摸只报告三根手中心的共同位置
固件会为双指触摸发12 字节的数据
关于去抖的说明：
如果设备供电不稳定或存在其它电气问题，或者手指数量发生变化，固件会发送“去数据包”以通知驱动硬件处于去抖状态```

    byte 0: 0xc4
    byte 1: 0xff
    byte 2: 0xff
    byte 3: 0x02
    byte 4: 0xff
    byte 5: 0xff

```
当我们遇到这类数据包时，直接忽略它即可
##### 单指/三指触摸


```

   bit   7   6   5   4   3   2   1   0
        n1  n0  w3  w2   0   1   R   L

        L, R = 1 when Left, Right mouse button pressed
        n1..n0 = number of fingers on touchpad

```
```

   bit   7   6   5   4   3   2   1   0
        p7  p6  p5  p4 x11 x10  x9  x8

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

        x11..x0 = absolute x value (horizontal)

```
```

   bit   7   6   5   4   3   2   1   0
         0   0  w1  w0   0   0   1   0

         w3..w0 = width of the finger touch

```
```

   bit   7   6   5   4   3   2   1   0
        p3  p1  p2  p0  y11 y10 y9  y8

        p7..p0 = pressure

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

        y11..y0 = absolute y value (vertical)

```
##### 双指触摸


双指触摸的数据包格式完全相同，只是硬件发送两6 字节数据包。第一个数据包包含
第一根手指的数据，第二个数据包包含第二根手指的数据。因此双指触摸总共发12 字节
#### 硬件版本 4



### 瀵勫瓨鍣。

```

   bit   7   6   5   4   3   2   1   0
         0   0   0   0   0   0   0   A

         A: 1 = enable absolute tracking

```
### 原生绝对模式 6 字节数据包格

v4 硬件是一个真正的多点触控触控板，能够跟踪最5 根手指。不幸的是，由于 PS/2
有限的带宽，其数据包格式相当复杂
每当手指的数量或标识发生变化时，硬件会发送一个状态数据包来指示触控板上有多少以及哪些手指，随后跟着头部数据包或运动数据包。头部数据包包含手指 id、手指位（绝x、y 值）、宽度和压力的数据。运动数据包包含两根手指的位置增量
例如，当状态数据包告知触控板上2 根手指时，我们可以预期随后有两个头部数据包如果手指状态没有改变，后续的数据包将是运动数据包，仅发送手指位置的增量，直到我收到一个状态数据包
一个例外是单指触摸。当状态数据包告知我们只有一根手指时，硬件之后只会发送头数据包
##### 状态数据包


```

   bit   7   6   5   4   3   2   1   0
         .   .   .   .   0   1   R   L

         L, R = 1 when Left, Right mouse button pressed

```
```

   bit   7   6   5   4   3   2   1   0
         .   .   . ft4 ft3 ft2 ft1 ft0

         ft4 ft3 ft2 ft1 ft0 ftn = 1 when finger n is on touchpad

```
```

   not used

```
```

   bit   7   6   5   4   3   2   1   0
         .   .   .   1   0   0   0   0

         constant bits

```
```

   bit   7   6   5   4   3   2   1   0
         p   .   .   .   .   .   .   .

         p = 1 for palm

```
```

   not used

```
##### 头部数据

```

   bit   7   6   5   4   3   2   1   0
        w3  w2  w1  w0   0   1   R   L

        L, R = 1 when Left, Right mouse button pressed
        w3..w0 = finger width (spans how many trace lines)

```
```

   bit   7   6   5   4   3   2   1   0
        p7  p6  p5  p4 x11 x10  x9  x8

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

        x11..x0 = absolute x value (horizontal)

```
```

   bit   7   6   5   4   3   2   1   0
       id2 id1 id0   1   0   0   0   1

       id2..id0 = finger id

```
```

   bit   7   6   5   4   3   2   1   0
        p3  p1  p2  p0  y11 y10 y9  y8

        p7..p0 = pressure

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

        y11..y0 = absolute y value (vertical)

```
##### 运动数据

```

   bit   7   6   5   4   3   2   1   0
       id2 id1 id0   w   0   1   R   L

       L, R = 1 when Left, Right mouse button pressed
       id2..id0 = finger id
       w = 1 when delta overflows (> 127 or < -128), in this case
       firmware sends us (delta x / 5) and (delta y  / 5)

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

        x7..x0 = delta x (two's complement)

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

        y7..y0 = delta y (two's complement)

```
```

   bit   7   6   5   4   3   2   1   0
       id2 id1 id0   1   0   0   1   0

       id2..id0 = finger id

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

        x7..x0 = delta x (two's complement)

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0

        y7..y0 = delta y (two's complement)

        byte 0 ~ 2 for one finger
        byte 3 ~ 5 for another


```
#### 指点杆（适用于硬件版3 4


### 瀵勫瓨鍣。

尚未识别出任何特殊的寄存器
### 原生相对模式 6 字节数据包格

##### 状态数据包


```

   bit   7   6   5   4   3   2   1   0
         0   0  sx  sy   0   M   R   L

```
```

   bit   7   6   5   4   3   2   1   0
       ~sx   0   0   0   0   0   0   0

```
```

   bit   7   6   5   4   3   2   1   0
       ~sy   0   0   0   0   0   0   0

```
```

   bit   7   6   5   4   3   2   1   0
         0   0 ~sy ~sx   0   1   1   0

```
```

   bit   7   6   5   4   3   2   1   0
        x7  x6  x5  x4  x3  x2  x1  x0

```
```

   bit   7   6   5   4   3   2   1   0
        y7  y6  y5  y4  y3  y2  y1  y0


         x and y are written in two's complement spread
             over 9 bits with sx/sy the relative top bit and
             x7..x0 and y7..y0 the lower bits.
	 ~sx is the inverse of sx, ~sy is the inverse of sy.
         The sign of y is opposite to what the input driver
             expects for a relative movement

```
