## Iforce 协议


:作者: Johann Deneux <johann.deneux@gmail.com>

主页位于 `<http://web.archive.org/web/*/http://www.esil.univ-mrs.fr>`_

:补充: 由 Vojtech Pavlik 添加。


## 简介


本文档描述了我设法发现的、用于向 I-Force 2.0 设备指定力反馈效果（force effects）的协议信息。这些信息均非来自 Immerse（ Immersion 公司）。因此，你不应轻信本文档所写的内容。本文档旨在帮助理解该协议，
并非一份参考手册。欢迎提出意见与修正。如需联系我，请发送邮件至：johann.deneux@gmail.com


    如果你尝试依据本文档中所读到的内容向你的 I-Force 设备发送数据，由此造成的任何损坏或伤害，我概不负责。

## 预备说明


所有数值均为十六进制，采用大端（big-endian）编码（最高有效位在左）。但请注意，
数据包内部的值采用小端（little-endian）编码。作用未知字节标记为 ??? 需要进一步深入检查的信息标记为 (?)

### 数据包的一般形式


以下是设备使用 rs232 进行通信时数据包的样子。

== == === ==== ==
2B OP LEN DATA CS
== == === ==== ==

CS 是校验和。它等于所有字节的异或（exclusive or）结果。

使用 USB 时：

== ====
OP DATA
== ====

2B、LEN 和 CS 字段已消失，大概是因为 USB 会处理帧，并且数据损坏或被妥善处理，或影响可忽略。

首先，我描述由设备发送给计算机的效果。

## 设备输入状态


此数据包用于指示每个按钮的状态以及每个轴的值：

```
    OP= 01 for a joystick, 03 for a wheel
    LEN= Varies from device to device
    00 X-Axis lsb
    01 X-Axis msb
    02 Y-Axis lsb, or gas pedal for a wheel
    03 Y-Axis msb, or brake pedal for a wheel
    04 Throttle
    05 Buttons
    06 Lower 4 bits: Buttons
       Upper 4 bits: Hat
    07 Rudder

```
## 设备效果状态


```
    OP= 02
    LEN= Varies
    00 ? Bit 1 (Value 2) is the value of the deadman switch
    01 Bit 8 is set if the effect is playing. Bits 0 to 7 are the effect id.
    02 ??
    03 Address of parameter block changed (lsb)
    04 Address of parameter block changed (msb)
    05 Address of second parameter block changed (lsb)
    ... depending on the number of parameter blocks updated

```
### 力反馈效果


```
    OP=  01
    LEN= 0e
    00 Channel (when playing several effects at the same time, each must
                be assigned a channel)
    01 Wave form
	    Val 00 Constant
	    Val 20 Square
	    Val 21 Triangle
	    Val 22 Sine
	    Val 23 Sawtooth up
	    Val 24 Sawtooth down
	    Val 40 Spring (Force = f(pos))
	    Val 41 Friction (Force = f(velocity)) and Inertia
	           (Force = f(acceleration))

    02 Axes affected and trigger
	    Bits 4-7: Val 2 = effect along one axis. Byte 05 indicates direction
		    Val 4 = X axis only. Byte 05 must contain 5a
		    Val 8 = Y axis only. Byte 05 must contain b4
		    Val c = X and Y axes. Bytes 05 must contain 60
	    Bits 0-3: Val 0 = No trigger
		    Val x+1 = Button x triggers the effect
	    When the whole byte is 0, cancel the previously set trigger

    03-04 Duration of effect (little endian encoding, in ms)

    05 Direction of effect, if applicable. Else, see 02 for value to assign.

    06-07 Minimum time between triggering.

    08-09 Address of periodicity or magnitude parameters
    0a-0b Address of attack and fade parameters, or ffff if none.
    *or*
    08-09 Address of interactive parameters for X-axis,
          or ffff if not applicable
    0a-0b Address of interactive parameters for Y-axis,
	  or ffff if not applicable

    0c-0d Delay before execution of effect (little endian encoding, in ms)

```
### 基于时间的参数


##### 起音与淡出


```
    OP=  02
    LEN= 08
    00-01 Address where to store the parameters
    02-03 Duration of attack (little endian encoding, in ms)
    04 Level at end of attack. Signed byte.
    05-06 Duration of fade.
    06 Level at end of fade.

```
##### 幅值


```
    OP=  03
    LEN= 03
    00-01 Address
    02 Level. Signed byte.

```
##### 周期性


```
    OP=  04
    LEN= 07
    00-01 Address
    02 Magnitude. Signed byte.
    03 Offset. Signed byte.
    04 Phase. Val 00 = 0 deg, Val 40 = 90 degs.
    05-06 Period (little endian encoding, in ms)

```
### 交互参数


```
    OP=  05
    LEN= 0a
    00-01 Address
    02 Positive Coeff
    03 Negative Coeff
    04+05 Offset (center)
    06+07 Dead band (Val 01F4 = 5000 (decimal))
    08 Positive saturation (Val 0a = 1000 (decimal) Val 64 = 10000 (decimal))
    09 Negative saturation

```
此处的编码有些特殊：对于系数（coeffs），这些是有符号值。最大值为 64（十进制 100），最小值为 9c。
对于偏移（offset），最小值为 FE0C，最大值为 01F4。
对于死区（deadband），最小值为 0，最大值为 03E8。

### 控制


```
    OP=  41
    LEN= 03
    00 Channel
    01 Start/Stop
	    Val 00: Stop
	    Val 01: Start and play once.
	    Val 41: Start and play n times (See byte 02 below)
    02 Number of iterations n.

```
### 初始化


##### 查询特性


```
    OP=  ff
    Query command. Length varies according to the query type.
    The general format of this packet is:
    ff 01 QUERY [INDEX] CHECKSUM
    responses are of the same form:
    FF LEN QUERY VALUE_QUERIED CHECKSUM2
    where LEN = 1 + length(VALUE_QUERIED)

```
#### 查询 RAM 大小


```
    QUERY = 42 ('B'uffer size)

```
设备应以相同的数据包加两个额外字节（包含内存大小）作为回应：
ff 03 42 03 e8 CS 表示设备有 1000 字节的 RAM 可用。

#### 查询效果数量


```
    QUERY = 4e ('N'umber of effects)

```
设备应通过发送可同时播放的效果数量（一个字节）来回应
ff 02 4e 14 CS 表示 20 个效果。

#### 厂商 ID


```
    QUERY = 4d ('M'anufacturer)

```
查询厂商 ID（2 字节）

#### 产品 ID


```
    QUERY = 50 ('P'roduct)

```
查询产品 ID（2 字节）

#### 打开设备


```
    QUERY = 4f ('O'pen)

```
无数据返回。

#### 关闭设备


```
    QUERY = 43 ('C')lose

```
无数据返回。

#### 查询效果


```
    QUERY = 45 ('E')

```
发送效果类型。
若支持则返回非零值（2 字节）

#### 固件版本


```
    QUERY = 56 ('V'ersion)

```
返回 3 个字节 —— 主版本、次版本、修订版本

##### 设备的初始化


#### 设置控制


    设备相关，在不同型号上可能不同！

```
    OP=  40 <idx> <val> [<val>]
    LEN= 2 or 3
    00 Idx
       Idx 00 Set dead zone (0..2048)
       Idx 01 Ignore Deadman sensor (0..1)
       Idx 02 Enable comm watchdog (0..1)
       Idx 03 Set the strength of the spring (0..100)
       Idx 04 Enable or disable the spring (0/1)
       Idx 05 Set axis saturation threshold (0..2048)

```
#### 设置效果状态


```
    OP=  42 <val>
    LEN= 1
    00 State
       Bit 3 Pause force feedback
       Bit 2 Enable force feedback
       Bit 0 Stop all effects

```
#### 设置整体增益


```
    OP=  43 <val>
    LEN= 1
    00 Gain
       Val 00 = 0%
       Val 40 = 50%
       Val 80 = 100%

```
### 参数内存


每个设备都有一定数量的用于存储效果参数的内存。
RAM 的大小可能不同，我遇到过 200 到 1000 字节之间的值。以下是每组参数明显所需的内存量：

 - period : 0c
 - magnitude : 02
 - attack and fade : 0e
 - interactive : 08

## 附录：如何研究该协议？


1. 使用随 DirectX SDK 提供的力反馈编辑器生成效果，或
   使用 Immersion Studio（可在其网站开发者专区免费获取：
   www.immersion.com）
2. 启动一个对 RS232 或 USB 进行嗅探（spying）的软件（取决于你将摇杆/方向盘连接到何处）。我使用了 fCoder 的 ComPortSpy（alpha 版本！）
3. 播放该效果，并观察嗅探屏幕上的变化。

关于 ComPortSpy 的几句话：
乍一看，这个软件似乎，嗯，有点……有 bug。实际上，数据会出现几秒钟的延迟。就我个人而言，每次播放效果时我都会重启它。
请记住它是免费的（如同免费啤酒般免费）而且还是 alpha 版！

## URLS


查看 http://www.immerse.com 获取 Immersion Studio，
以及 http://www.fcoder.com 获取 ComPortSpy。


I-Force 是 Immersion Corp. 的商标。
