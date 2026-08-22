## 智能键盘（ikbd）协
## 简
Atari 公司的智能键盘（Intelligent Keyboard，ikbd）是一个通用的键盘控制器，其
灵活性足以在多种产品中不经修改即可使用。该键盘连同其微控制器，为鼠标和开关型
（switch-type）游戏杆（joystick）提供了一个方便的连接点。ikbd 处理器还维护一分辨率为一秒的日期时钟（time-of-day clock）ikbd 的设计具有足够的通用性，可用于多种新的计算机产品。按键开关数量、鼠标分辨率
等方面的产品差异都可以被容纳ikbd 通过一个高速双向串行接口与主处理器通信。它可以工作于多种模式，以方便键盘游戏杆或鼠标的不同应用。通过精心设计默认模式，即使在只有单向通信介质可用的应中，也可以有限度地使用该控制器
## 键盘

键盘始终返回按键按下/释放（make/break）的扫描码。ikbd 为每个按键的按下和释生成键盘扫描码。按键扫描按下（按键闭合）码1 开始，并在附录 A 中定义。例如，
扫描码表ISO 键的位置应当存在，即使某个特定键盘上该位置没有按键开关。每个按的释放（break）码通过对按下（make）码进行 OR 0x80 操作得到
特殊0xF6 0xFF 保留用于以下用途：

=================== ====================================================
    Code            Command
=================== ====================================================
    0xF6            status report
    0xF7            absolute mouse position record
    0xF8-0xFB       relative mouse position records (lsbs determined by
                    mouse button states)
    0xFC            time-of-day
    0xFD            joystick report (both sticks)
    0xFE            joystick 0 event
    0xFF            joystick 1 event
=================== ====================================================

在此模式下，两个 Shift 键返回不同的扫描码。ENTER 键和 RETurn 键也是不同的
## 鼠标

鼠标端口应当能够支持分辨率约为每英寸行程 200 个计数（相位变化或“点击（click）”）鼠标。鼠标应以能够允许在高达每秒 10 英寸的速度下准确跟踪的速率进行扫描ikbd 可以通过三种明显不同的方式报告鼠标运动。它可以报告相对运动、在 ikbd 内部
维护的坐标系中的绝对运动，或者将鼠标运动转换为键盘光标控制键等效键鼠标按键可以视为鼠标的一部分，也可以视为额外的键盘按键
### 相对位置报告

在相对位置模式下，每当发生鼠标事件时，ikbd 都会返回相对鼠标位置记录。鼠标事包括鼠标按键被按下或释放，或者任一轴的运动超过可设置的阈值。无论阈值如何，所分辨率位都会返回给主机（host）计算机注意，ikbd 返回的相对鼠标位置报告中delta x y 可能显著大于阈值。这可能发生因为不会生成相对鼠标运动事件的情况是a) 当键盘已被“暂停”（事件将被存储直到键盘
通信恢复）时b) 当任何事件正在传输时
相对鼠标位置记录是一个三字节记录，形式为```

    %111110xy           ; mouse position record flag
                        ; where y is the right button state
                        ; and x is the left button state
    X                   ; delta x as twos complement integer
    Y                   ; delta y as twos complement integer

```
注意，即MOUSE BUTTON ACTION 已将按键设置为像键盘的一部分那样工作，按键状态位
的值也应当是有效的如果在生成报告数据包之前累积的运动超出了 +127...-128 范围，则该运动会分解为多数据包注意，所报告delta y 的符号是所Y 原点的函数
### 绝对位置报告

ikbd 也可以维护绝对鼠标位置。存在用于重置鼠标位置、设X/Y 缩放比例以及查询当前
鼠标位置的命令
### 鼠标光标键模
ikbd 可以将鼠标运动转换为等效的光标按键。每个轴每按键一次的鼠标点击次数是可独立
编程的。ikbd 内部以可用的最高分辨率维护鼠标运动信息，并且仅为比例因子的每个倍数
生成一对光标键事件鼠标运动产生光标键的按下（make）码，紧随其后的是相应光标键的释放（break）码。鼠按键产生的扫描码高于通常为最大设想键盘所分配的范围（LEFT=0x74 RIGHT=0x75）
## 游戏杆（Joystick
### 游戏杆事件报
在此模式下，每当游戏杆位置改变时（即每当游戏杆开关或扳机（trigger）闭合或断开
时），ikbd 都会生成一个记录
```

    %1111111x           ; Joystick event marker
                        ; where x is Joystick 0 or 1
    %x000yyyy           ; where yyyy is the stick position
                        ; and x is the trigger

```

### 游戏杆查
在此模式下，可随时通过ikbd 发送“Interrogate Joystick（查询游戏杆）”命令来
查询游戏杆端口的当前状态
```

    0xFD                ; joystick report header
    %x000yyyy           ; Joystick 0
    %x000yyyy           ; Joystick 1
                        ; where x is the trigger
                        ; and yyy is the stick position

```

### 游戏杆监
提供一种模式，该模式几乎将所有键盘通信时间都用于在用户可指定速率下报告游戏杆
端口的状态。它会一直保持此模式，直到被重置或被命令进入另一种模式。此模式下的
PAUSE 命令不仅停止输出，还会暂时停止扫描游戏杆（样本不被排队）
### 扳机按钮监视

提供一种模式，用于以高速率监视单个输入位。在此模式下，ikbd 以串行通信通道允许最大速率监视游戏1 的扳机（fire）按钮的状态。数据被打包为每字节 8 位传输给主机ikbd 一直保持此模式，直到被重置或被命令进入另一种模式。此模式下的 PAUSE 命令不仅
停止输出，还会暂时停止扫描按钮（样本不被排队）
### 游戏杆键码模
可以命令 ikbd 将任一游戏杆的使用转换为等效的光标控制按键。ikbd 提供一个单一
断点（breakpoint）速度游戏杆光标游戏杆事件产生按下（make）码，紧随其后的是相应光标运动键的释放（break）码。游戏杆扳机或开火（fire）按钮产生的伪按键扫描码高于所设想的最大按键矩阵所使用的范（即 JOYSTICK0=0x74，JOYSTICK1=0x75）
## 日期时钟（Time-of-Day Clock
ikbd 还为系统维护一个日期时钟。提供有用于设置和查询该日期时钟的命令。计时（Time-keeping的维护分辨率可达一秒
## 状态查
可以通过发送与 ikbd 设置命令相对应的状态查询命令，来获ikbd 模式和参数的
当前状态
## 上电模式（Power-Up Mode
键盘控制器在上电时将执行一个简单的自检，以检测主要的控制器故障（ROM 校验和与
RAM 测试）以及诸如卡键（stuck keys）之类的问题。上电时按下的任何键都被假定为卡住，
并返回其 BREAK（原文如此）码（在没有前MAKE 码的情况下，这是一个键盘错误的标志）如果控制器自检无误完成，则返回代码 0xF0。（该代码将用于指示 ikbd 控制器的版本/发布ikbd 的首个发布版本为 0xF0，若发布第二个版本则0xF1，依此类推。）
ikbd 默认为鼠标位置报告模式，两个轴的阈值均1 个单位，Y=0 原点位于屏幕顶部，并
对游戏杆 1 采用游戏杆事件报告模式，两个按键在逻辑上都分配给鼠标。在任何游戏杆命之后，ikbd 假定游戏0 和游戏杆 1 均已连接。然后任何鼠标命令（MOUSE DISABLE 除外会使端口 0 再次被当作鼠标扫描，并且两个按键在逻辑上都连接到它。如果在假定端口 0 鼠标时收到鼠标禁用命令，则该按键在逻辑上被分配给游戏杆 1（直到鼠标被另一个鼠标命重新启用）
## ikbd 鍛戒护闆?
本节包含可发送给 ikbd 的命令列表。未指定的命令码（如 0x00）应当不执行任何操作
（NOPs）
### RESET

```

    0x80
    0x01

```

注意：RESET 命令ikbd 所能理解的唯一一个双字节命令。任何跟0x80 命令字节之后0x01 以外的字节都将被忽略（并导致 0x80 被忽略）也可以通过ikbd 发送持续至200mS break 来引起复位执行 RESET 命令会使键盘返回其默认（上电）模式和参数设置。它不影响日期时钟RESET 命令或功能会ikbd 执行一个简单的自检。如果测试成功，ikbd 将在收到 RESET
命令（或 break 结束，或上电）后300mS 内发送代0xF0。然ikbd 会扫描按键矩以查找任何卡住（闭合）的键。发现的任何闭合键都会导致生成释放（break）扫描码（释码在没有前置按下（make）码的情况下到达，就是按键矩阵错误的标志）
### SET MOUSE BUTTON ACTION

```

    0x07
    %00000mss           ; mouse button action
                        ;       (m is presumed = 1 when in MOUSE KEYCODE mode)
                        ; mss=0xy, mouse button press or release causes mouse
                        ;  position report
                        ;  where y=1, mouse key press causes absolute report
                        ;  and x=1, mouse key release causes absolute report
                        ; mss=100, mouse buttons act like keys

```

此命令设ikbd 应如何处理鼠标上的按键。默认的鼠标按键动作模式%00000000，按在逻辑上被视为鼠标的一部分当按键表现得像按键时，LEFT=0x74 RIGHT=0x75
### SET RELATIVE MOUSE POSITION REPORTING

```

    0x08

```

设置相对鼠标位置报告。（默认）每当任一轴的运动超过可设置的阈值时，鼠标位置数据包
ikbd 异步生成（参SET MOUSE THRESHOLD）。根据鼠标键模式的不同，鼠标位置报告
也可能在两个鼠标按键中的任何一个被按下或释放时生成。否则鼠标按键的行为就像键盘
按键一样
### SET ABSOLUTE MOUSE POSITIONING

```

    0x09
    XMSB                ; X maximum (in scaled mouse clicks)
    XLSB
    YMSB                ; Y maximum (in scaled mouse clicks)
    YLSB

```

设置绝对鼠标位置维护。重ikbd 维护X Y 坐标在此模式下，内部维护的坐标值不会在 0 和较大的正数之间回绕（wrap）。低0 的过运动被忽略。该命令设置可在缩放坐标系统中达到的最大正值。超出该值的运动也被忽略
### SET MOUSE KEYCODE MODE

```

    0x0A
    deltax              ; distance in X clicks to return (LEFT) or (RIGHT)
    deltay              ; distance in Y clicks to return (UP) or (DOWN)

```

设置鼠标监视例程以返回光标运动键码，而不是相对或绝对运动记录。ikbd 在任一轴的鼠标
移动量超过用户指定的 delta 后返回相应的光标键码。当键盘处于键扫描码模式时，鼠标运动
会导致按下（make）码之后紧跟着释放（break）码。注意，此命令不受鼠标运动原点的影响
### SET MOUSE THRESHOLD

```

    0x0B
    X                   ; x threshold in mouse ticks (positive integers)
    Y                   ; y threshold in mouse ticks (positive integers)

```

此命令设置在生成鼠标事件之前的阈值。注意，它不影响返回给主机的数据的分辨率。此
命令仅在 RELATIVE MOUSE POSITIONING 模式下有效。阈值在 RESET（或上电）时默认1
### SET MOUSE SCALE

```

    0x0C
    X                   ; horizontal mouse ticks per internal X
    Y                   ; vertical mouse ticks per internal Y

```

此命令设ABSOLUTE MOUSE POSITIONING 模式的比例因子。在此模式下，必须发生指定数的鼠标相位变化（“点击（click）”），内部的维护坐标才会改变 1（每个轴独立缩放）请记住，除非已命ikbd 在按键按下或释放时进行报告（参见 SET MOUSE BUTTON ACTION），
否则鼠标位置信息仅能通过查询 ABSOLUTE MOUSE POSITIONING 模式下的 ikbd 获得
### INTERROGATE MOUSE POSITION

```

    0x0D
    Returns:
            0xF7       ; absolute mouse position header
    BUTTONS
            0000dcba   ; where a is right button down since last interrogation
                       ; b is right button up since last
                       ; c is left button down since last
                       ; d is left button up since last
            XMSB       ; X coordinate
            XLSB
            YMSB       ; Y coordinate
            YLSB

```

INTERROGATE MOUSE POSITION 命令ABSOLUTE MOUSE POSITIONING 模式下有效，无论
MOUSE BUTTON ACTION 的设置如何
### LOAD MOUSE POSITION

```

    0x0E
    0x00                ; filler
    XMSB                ; X coordinate
    XLSB                ; (in scaled coordinate system)
    YMSB                ; Y coordinate
    YLSB

```

此命令允许用户预设内部维护的绝对鼠标位置
### SET Y=0 AT BOTTOM

```

    0x0F

```

此命令使 Y 轴的原点位于 ikbd 内部所有相对或绝对鼠标运动逻辑坐标系的底部。这朝向用户的鼠标运动符号为负，远离用户的鼠标运动符号为正
### SET Y=0 AT TOP

```

    0x10

```

Y 轴的原点位于 ikbd 内部所有相对或绝对鼠标运动逻辑坐标系的顶部。（默认这使朝向用户的鼠标运动符号为正，远离用户的鼠标运动符号为负
### RESUME

```

    0x11

```

恢复向主机发送数据。由ikbd 在输出被暂停后收到的任何命令也会导致隐式 RESUME，因此命令可被视为一个无操作（NO OPERATION）命令。如ikbd 收到此命令而它并未处于 PAUSED
状态，则简单地将其忽略
### DISABLE MOUSE

```

    0x12

```

禁用所有鼠标事件报告（并且扫描可能在内部被禁用）。任何有效的鼠标模式命令都会恢复
鼠标运动监视。（有效的鼠标模式命令有 SET RELATIVE MOUSE POSITION REPORTING、SET
ABSOLUTE MOUSE POSITIONING 以及 SET MOUSE KEYCODE MODE。）
注意：如果鼠标按键已被命令表现得像键盘按键，此命*确实**会影响它们的动作
### PAUSE OUTPUT

```

    0x13

```

停止向主机发送数据，直到收到另一个有效命令。按键矩阵活动仍被监视，扫描码或 ASCII
字符被排队（最多受微控制器支持的数量），以便在主机允许恢复输出时发送。如果处JOYSTICK EVENT REPORTING 模式，游戏杆事件也会被排队在输出暂停期间，鼠标运动应当被累积。如ikbd 处于 RELATIVE MOUSE POSITIONING
REPORTING 模式，运动会在正常阈值限制之外累积，以在输出恢复时产生传输所需的最数据包数量。如果鼠标处RELATIVE MOUSE POSITION REPORTING 模式，按下或释放任一
鼠标按键会导致任何累积的运动立即作为数据包排队由于微控制器内存的限制，此命令应当谨慎使用，并且每次关闭输出的时间不应超<tbd> 毫秒输出仅在当前“事件（event）”结束时才停止。如果在多字节报告的中途收PAUSE OUTPUT
命令，该数据包仍将被传输完毕，然PAUSE 才会生效ikbd 处于 JOYSTICK MONITORING 模式FIRE BUTTON MONITORING 模式时，PAUSE OUTPUT
命令也会暂时停止监视过程（即样本不被排队以传输）
### SET JOYSTICK EVENT REPORTING

```

    0x14

```

进入 JOYSTICK EVENT REPORTING 模式（默认）。游戏杆开关或扳机的每次断开或闭合都导致生成一个游戏杆事件记录
### SET JOYSTICK INTERROGATION MODE

```

    0x15

```

禁用 JOYSTICK EVENT REPORTING。主机必须发送单独的 JOYSTICK INTERROGATE 命令来感游戏杆状态
### JOYSTICK INTERROGATE

```

    0x16

```

返回一个指示游戏杆当前状态的记录。此命令JOYSTICK EVENT REPORTING 模式JOYSTICK INTERROGATION MODE 下均有效
### SET JOYSTICK MONITORING

```

    0x17
    rate                ; time between samples in hundredths of a second
    Returns: (in packets of two as long as in mode)
            %000000xy   ; where y is JOYSTICK1 Fire button
                        ; and x is JOYSTICK0 Fire button
            %nnnnmmmm   ; where m is JOYSTICK1 state
                        ; and n is JOYSTICK0 state

```

设置 ikbd 只监视串行命令线、维护日期时钟并监视游戏杆。rate 设置游戏杆采样之间的
间隔注意：用户不应将 rate 设置得高于串行通信通道所能允许传输这 2 字节数据包的速率
### SET FIRE BUTTON MONITORING

```

    0x18
    Returns: (as long as in mode)
            %bbbbbbbb   ; state of the JOYSTICK1 fire button packed
                        ; 8 bits per byte, the first sample if the MSB

```

设置 ikbd 只监视串行命令线、维护日期时钟并监视游戏1 上的开火（fire）按钮。开按钮的扫描速率使得在前一个字节发送给主机所需的时间内进行 8 次采样（即扫描速率 =
8/10 × 波特率）。采样间隔应尽可能恒定
### SET JOYSTICK KEYCODE MODE

```

    0x19
    RX                  ; length of time (in tenths of seconds) until
                        ; horizontal velocity breakpoint is reached
    RY                  ; length of time (in tenths of seconds) until
                        ; vertical velocity breakpoint is reached
    TX                  ; length (in tenths of seconds) of joystick closure
                        ; until horizontal cursor key is generated before RX
                        ; has elapsed
    TY                  ; length (in tenths of seconds) of joystick closure
                        ; until vertical cursor key is generated before RY
                        ; has elapsed
    VX                  ; length (in tenths of seconds) of joystick closure
                        ; until horizontal cursor keystrokes are generated
                        ; after RX has elapsed
    VY                  ; length (in tenths of seconds) of joystick closure
                        ; until vertical cursor keystrokes are generated
                        ; after RY has elapsed

```

在此模式下，游戏0 以一种模拟光标按键的方式被扫描。在初始闭合时，生成一对按（按释放）。然后在最Rn 个十分之一秒之后，Tn 个十分之一秒生成一对按键。在
达到 Rn 断点后，Vn 个十分之一秒生成一对按键。这提供了一个速度（自动重复）断点
特性注意，通过RX RY 设置为零，可以禁用速度特性。此TX TY 的值变得无意义而光标“按键”的生成VX VY 决定
### DISABLE JOYSTICKS

```

    0x1A

```

禁用任何游戏杆事件的生成（并且扫描可能在内部被禁用）。任何有效的游戏杆模式命令都恢复游戏杆监视。（游戏杆模式命令包SET JOYSTICK EVENT REPORTING、SET JOYSTICK
INTERROGATION MODE、SET JOYSTICK MONITORING、SET FIRE BUTTON MONITORING 以及
SET JOYSTICK KEYCODE MODE。）

### TIME-OF-DAY CLOCK SET

```

    0x1B
    YY                  ; year (2 least significant digits)
    MM                  ; month
    DD                  ; day
    hh                  ; hour
    mm                  ; minute
    ss                  ; second

```

所有日期时钟数据都应以压缩 BCD 格式发送给 ikbd任何不是有效 BCD 位的数字应被视为“不关心（don't care）”，并且不改变日期或时间该特定字段。这允许只设置日期时钟的某些子字段
### INTERROGATE TIME-OF-DAT CLOCK

```

    0x1C
    Returns:
            0xFC        ; time-of-day event header
            YY          ; year (2 least significant digits)
            MM          ; month
            DD          ; day
            hh          ; hour
            mm          ; minute
            ss          ; second

    All time-of-day is sent in packed BCD format.

```

### MEMORY LOAD

```

    0x20
    ADRMSB              ; address in controller
    ADRLSB              ; memory to be loaded
    NUM                 ; number of bytes (0-128)
    { data }

```

此命令允许主机将任意值加载到 ikbd 控制器内存中。数据字节之间的时间间隔必须小于 20ms
### MEMORY READ

```

    0x21
    ADRMSB              ; address in controller
    ADRLSB              ; memory to be read
    Returns:
            0xF6        ; status header
            0x20        ; memory access
            { data }    ; 6 data bytes starting at ADR

```

此命令允许主机从 ikbd 控制器内存中读取
### CONTROLLER EXECUTE

```

    0x22
    ADRMSB              ; address of subroutine in
    ADRLSB              ; controller memory to be called

```

此命令允许主机命令执ikbd 控制器内存中的一个子例程
### STATUS INQUIRIES

```

    Status commands are formed by inclusively ORing 0x80 with the
    relevant SET command.

    Example:
    0x88 (or 0x89 or 0x8A)  ; request mouse mode
    Returns:
            0xF6        ; status response header
            mode        ; 0x08 is RELATIVE
                        ; 0x09 is ABSOLUTE
                        ; 0x0A is KEYCODE
            param1      ; 0 is RELATIVE
                        ; XMSB maximum if ABSOLUTE
                        ; DELTA X is KEYCODE
            param2      ; 0 is RELATIVE
                        ; YMSB maximum if ABSOLUTE
                        ; DELTA Y is KEYCODE
            param3      ; 0 if RELATIVE
                        ; or KEYCODE
                        ; YMSB is ABSOLUTE
            param4      ; 0 if RELATIVE
                        ; or KEYCODE
                        ; YLSB is ABSOLUTE
            0           ; pad
            0

```

STATUS INQUIRY 命令请求 ikbd 返回当前模式或与给定命令关联的参数。所有状态报告都填充为形8 字节长的返回数据包。对状态请求的响应被设计成这样：主机可以将它们存储
起来（在剥离状态报告头字节之后），并在以后作为命令发回ikbd 以恢复其状态 填充
字节会被 ikbd 视为 NOP
```

            0x87    mouse button action
            0x88    mouse mode
            0x89
            0x8A
            0x8B    mnouse threshold
            0x8C    mouse scale
            0x8F    mouse vertical coordinates
            0x90    ( returns       0x0F Y=0 at bottom
                            0x10 Y=0 at top )
            0x92    mouse enable/disable
                    ( returns       0x00 enabled)
                            0x12 disabled )
            0x94    joystick mode
            0x95
            0x96
            0x9A    joystick enable/disable
                    ( returns       0x00 enabled
                            0x1A disabled )

```

在同一时间只有一个未答复的查询处于进行中，这是（主机）程序员的责任如果 ikbd 处于 JOYSTICK MONITORING 模式FIRE BUTTON MONITORING 模式，则 STATUS
INQUIRY 命令无效
## 扫描码（SCAN CODES
ikbd 返回的键扫描码被选择为简GSX 的实现
GSX Standard Keyboard Mapping

======= ============
Hex	Keytop
======= ============
01	Esc
02	1
03	2
04	3
05	4
06	5
07	6
08	7
09	8
0A	9
0B	0
0C	\-
0D	\=
0E	BS
0F	TAB
10	Q
11	W
12	E
13	R
14	T
15	Y
16	U
17	I
18	O
19	P
1A	[
1B	]
1C	RET
1D	CTRL
1E	A
1F	S
20	D
21	F
22	G
23	H
24	J
25	K
26	L
27	;
28	'
29	\`
2A	(LEFT) SHIFT
2B	\\
2C	Z
2D	X
2E	C
2F	V
30	B
31	N
32	M
33	,
34	.
35	/
36	(RIGHT) SHIFT
37	{ NOT USED }
38	ALT
39	SPACE BAR
3A	CAPS LOCK
3B	F1
3C	F2
3D	F3
3E	F4
3F	F5
40	F6
41	F7
42	F8
43	F9
44	F10
45	{ NOT USED }
46	{ NOT USED }
47	HOME
48	UP ARROW
49	{ NOT USED }
4A	KEYPAD -
4B	LEFT ARROW
4C	{ NOT USED }
4D	RIGHT ARROW
4E	KEYPAD +
4F	{ NOT USED }
50	DOWN ARROW
51	{ NOT USED }
52	INSERT
53	DEL
54	{ NOT USED }
5F	{ NOT USED }
60	ISO KEY
61	UNDO
62	HELP
63	KEYPAD (
64	KEYPAD /
65	KEYPAD *
66	KEYPAD *
67	KEYPAD 7
68	KEYPAD 8
69	KEYPAD 9
6A	KEYPAD 4
6B	KEYPAD 5
6C	KEYPAD 6
6D	KEYPAD 1
6E	KEYPAD 2
6F	KEYPAD 3
70	KEYPAD 0
71	KEYPAD .
72	KEYPAD ENTER
======= ============
