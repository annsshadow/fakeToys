
## 多点触摸（MT）协议

:Copyright: |copy| 2009-2010	Henrik Rydberg <rydberg@euromail.se>


### 简介

为了充分利用新的多点触摸和多用户设备的全部能力，需要一种方法来报告来自多个
接触点（即与设备表面直接接触的物体）的详细数据。本文档描述了多点触摸（MT）
协议，它允许内核驱动报告任意数量接触点的细节。

该协议根据硬件的能力分为两类。对于处理匿名接触（type A）的设备，协议描述了
如何把全部接触点的原始数据发送给接收方。对于能够跟踪可识别接触（type B）的
设备，协议描述了如何通过事件槽（slot）发送单个接触点的更新。

   MT 协议 type A 已经过时，所有内核驱动都已被转换为使用 type B。

### 协议用法

接触详情作为一组组独立的 ABS_MT 事件包被顺序发送。只有 ABS_MT 事件被识别为
接触包的一部分。由于这些事件被当前的单点触摸（ST）应用程序忽略，MT 协议可以
在现有驱动中构建于 ST 协议之上实现。

type A 设备的驱动在每个包的末尾调用 input_mt_sync() 来分隔接触包。这会生成一个
SYN_MT_REPORT 事件，它指示接收方接受当前接触的数据，并准备接收另一个。

type B 设备的驱动在每个包的开头调用 input_mt_slot()（以一个 slot 作为参数）来
分隔接触包。这会生成一个 ABS_MT_SLOT 事件，它指示接收方准备更新给定的 slot。

所有驱动都通过调用常规的 input_sync() 函数来标记多点触摸传输的结束。它指示
接收方对自上次 EV_SYN/SYN_REPORT 以来累积的事件采取行动，并准备接收一组新的
事件/包。

无状态的 type A 协议与有状态的 type B slot 协议之间的主要区别在于：使用可识别
的接触来减少发送给用户空间的数据量。slot 协议要求使用 ABS_MT_TRACKING_ID，
它要么由硬件提供，要么从原始数据计算得出 [#f5]_。

对于 type A 设备，内核驱动应当为当前表面上全部的匿名接触生成一个任意的枚举。
包在事件流中出现的顺序并不重要。事件过滤和手指跟踪留给用户空间 [#f3]_。

对于 type B 设备，内核驱动应当把每个已识别的接触与一个 slot 关联，并用那个
slot 来传播该接触的变化。接触的创建、替换和销毁是通过修改相关 slot 的
ABS_MT_TRACKING_ID 来实现的。一个非负 tracking id 被解释为一个接触，而值 -1
表示一个未使用的 slot。一个之前没有出现的 tracking id 被视为新的，而不再出现的
tracking id 被视为已移除。由于只传播变化，每个已初始化的接触的完整状态都必须
驻留在接收端。在收到一个 MT 事件时，只需更新当前 slot 的相应属性即可。

某些设备识别并/或跟踪的接触点比它们能报告给驱动的数量更多。这类设备的驱动
应当把每个由硬件报告的接触与一个 type B slot 关联。每当与一个 slot 相关联的接触
的身份发生改变时，驱动应当通过改变其 ABS_MT_TRACKING_ID 来使该 slot 失效。如果
硬件发出信号表示它正在跟踪比当前报告的更多的接触，驱动应当使用 BTN_TOOL_*TAP
事件来通知用户空间在那一刻硬件正在跟踪的接触总数。驱动应当通过显式发送相应的
BTN_TOOL_*TAP 事件、并在调用 input_mt_report_pointer_emulation() 时把 use_count
设为 false 来做到这一点。驱动应当只公布硬件能报告的那么多 slot。用户空间可以
通过注意到所支持的最大的 BTN_TOOL_*TAP 事件大于在 ABS_MT_SLOT 轴的 absinfo 中
报告的 type B slot 总数，来检测出驱动能报告的接触总数比 slot 多。

ABS_MT_SLOT 轴的最小值必须为 0。

### 协议示例 A

下面是一个双接触触摸的最小事件序列的样子
```
   ABS_MT_POSITION_X x[0]
   ABS_MT_POSITION_Y y[0]
   SYN_MT_REPORT
   ABS_MT_POSITION_X x[1]
   ABS_MT_POSITION_Y y[1]
   SYN_MT_REPORT
   SYN_REPORT

```
在移动其中一个接触之后，序列看起来完全一样；所有当前接触的原始数据都在每次与
SYN_REPORT 的同步之间被发送。

```
   ABS_MT_POSITION_X x[1]
   ABS_MT_POSITION_Y y[1]
   SYN_MT_REPORT
   SYN_REPORT

```
```
   SYN_MT_REPORT
   SYN_REPORT

```
如果驱动除了 ABS_MT 事件之外还报告 BTN_TOUCH 或 ABS_PRESSURE 之一，则可以省略
最后一个 SYN_MT_REPORT 事件。否则，最后一个 SYN_REPORT 将被输入核心丢弃，导致
不会有零接触事件到达用户空间。


### 协议示例 B

下面是一个双接触触摸的最小事件序列的样子
```
   ABS_MT_SLOT 0
   ABS_MT_TRACKING_ID 45
   ABS_MT_POSITION_X x[0]
   ABS_MT_POSITION_Y y[0]
   ABS_MT_SLOT 1
   ABS_MT_TRACKING_ID 46
   ABS_MT_POSITION_X x[1]
   ABS_MT_POSITION_Y y[1]
   SYN_REPORT

```
```
   ABS_MT_SLOT 0
   ABS_MT_POSITION_X x[0]
   SYN_REPORT

```
```
   ABS_MT_TRACKING_ID -1
   SYN_REPORT

```
正在被修改的 slot 已经是 0，所以省略了 ABS_MT_SLOT。该消息移除了 slot 0 与接触
45 的关联，从而销毁了接触 45，并释放 slot 0 以便重用于另一个接触。

```
   ABS_MT_SLOT 1
   ABS_MT_TRACKING_ID -1
   SYN_REPORT


```
### 事件用法

定义了一组带有期望属性的 ABS_MT 事件。这些事件被分成若干类别，以允许部分实现。
最小集由 ABS_MT_POSITION_X 和 ABS_MT_POSITION_Y 组成，它们允许多个接触被跟踪。
如果设备支持，ABS_MT_TOUCH_MAJOR 和 ABS_MT_WIDTH_MAJOR 可分别用来提供接触区域和
接近工具的大小。

TOUCH 和 WIDTH 参数有一个几何解释；想象透过窗户看到有人把一根手指轻轻按在玻璃
上。你会看到两个区域，一个内部区域由手指实际接触玻璃的部分组成，一个外部区域
由手指的外缘形成。接触区域的中心（a）是 ABS_MT_POSITION_X/Y，而接近手指的中心
（b）是 ABS_MT_TOOL_X/Y。触摸直径就是 ABS_MT_TOUCH_MAJOR，而手指直径是
ABS_MT_WIDTH_MAJOR。现在想象这个人把手指更用力地按在玻璃上。接触区域会增大，
并且一般地，比值 ABS_MT_TOUCH_MAJOR / ABS_MT_WIDTH_MAJOR（它总是小于 1）与接触
压力相关。对于基于压力的设备，可以用 ABS_MT_PRESSURE 来提供接触区域上的压力。
能够悬停接触的设备可以使用 ABS_MT_DISTANCE 来指示接触与表面之间的距离。

```

	  Linux MT                               Win8
         __________                     _______________________
        /          \                   |                       |
       /            \                  |                       |
      /     ____     \                 |                       |
     /     /    \     \                |                       |
     \     \  a  \     \               |       a               |
      \     \____/      \              |                       |
       \                 \             |                       |
        \        b        \            |           b           |
         \                 \           |                       |
          \                 \          |                       |
           \                 \         |                       |
            \                /         |                       |
             \              /          |                       |
              \            /           |                       |
               \__________/            |_______________________|


```
除了 MAJOR 参数之外，触摸和手指区域的椭圆形状可以通过添加 MINOR 参数来描述，
使得 MAJOR 和 MINOR 成为一个椭圆的长轴和短轴。触摸椭圆的方向可以用 ORIENTATION
参数来描述，而手指椭圆的方向由向量（a - b）给出。

对于 type A 设备，可以通过 ABS_MT_BLOB_ID 进一步指定触摸形状。

ABS_MT_TOOL_TYPE 可用来指定触摸工具是手指、笔还是其它东西。最后，ABS_MT_TRACKING_ID
事件可用来随时间跟踪已识别的接触 [#f5]_。

在 type B 协议中，ABS_MT_TOOL_TYPE 和 ABS_MT_TRACKING_ID 由输入核心隐式处理；
驱动应当改为调用 input_mt_report_slot_state()。


### 事件语义

ABS_MT_TOUCH_MAJOR
    接触区域长轴的长度。长度应当以表面单位给出。如果表面的分辨率为 X 乘以 Y，
    则 ABS_MT_TOUCH_MAJOR 的最大可能值为 sqrt(X^2 + Y^2)，即对角线 [#f4]_。

ABS_MT_TOUCH_MINOR
    接触区域短轴的长度，以表面单位计。如果接触是圆形的，则可以省略此事件 [#f4]_。

ABS_MT_WIDTH_MAJOR
    接近工具的长轴的长度，以表面单位计。这应当被理解为工具自身的大小。接触和被
    接近工具的方向被假定为相同 [#f4]_。

ABS_MT_WIDTH_MINOR
    接近工具的短轴的长度，以表面单位计。如果是圆形则省略 [#f4]_。

    上述四个值可以用来推导关于接触的额外信息。比值 ABS_MT_TOUCH_MAJOR /
    ABS_MT_WIDTH_MAJOR 近似于压力的概念。手的手指和手掌都有不同的特征宽度。

ABS_MT_PRESSURE
    接触区域上的压力，以任意单位计。对于基于压力的设备或任何具有空间信号强度
    分布的設備，可以代替 TOUCH 和 WIDTH 使用。

    如果分辨率为零，压力数据为任意单位。如果分辨率非零，压力数据单位为
    单位/克。详见 input-event-codes。

ABS_MT_DISTANCE
    接触与表面之间的距离，以表面单位计。零距离意味着接触正触摸着表面。正数意味着
    接触悬停在表面上方。

ABS_MT_ORIENTATION
    触摸椭圆的方向。该值应当描述围绕触摸中心顺时针方向、带符号的四分之一圈。
    带符号的值范围是任意的，但对于与表面 Y 轴（北）对齐的椭圆应当返回零，当椭圆
    转向左侧时返回负值，转向右侧时返回正值。当与正方向的 X 轴对齐时，应当返回
    范围最大值；当与负方向的 X 轴对齐时，应当返回范围 -max。

    默认情况下触摸椭圆是对称的。对于能够真正 360 度方向的设备，报告的方位必须
    超过范围最大值，以表示超过四分之一圈。对于倒置的手指，应当返回范围 max * 2。

    如果触摸区域是圆形的，或者内核驱动中无法获得该信息，则可以省略方向。如果设备
    能够区分两个轴、但不能（唯一地）区分其间的任何值，则部分方向支持是可能的。在
    这种情况下，ABS_MT_ORIENTATION 的范围应当是 [0, 1] [#f4]_。

ABS_MT_POSITION_X
    触摸椭圆中心的表面 X 坐标。

ABS_MT_POSITION_Y
    触摸椭圆中心的表面 Y 坐标。

ABS_MT_TOOL_X
    接近工具中心的表面 X 坐标。如果设备无法区分预期的触摸点和工具自身，则省略。

ABS_MT_TOOL_Y
    接近工具中心的表面 Y 坐标。如果设备无法区分预期的触摸点和工具自身，则省略。

    这四个位置值可用来把触摸的位置与工具的位置分开。如果两个位置都存在，工具主轴
    指向触摸点 [#f1]_。否则，工具轴与触摸轴对齐。

ABS_MT_TOOL_TYPE
    接近工具的类型。许多内核驱动无法区分不同的工具类型，例如手指或笔。在这种情况下，
    应当省略该事件。协议目前主要支持 MT_TOOL_FINGER、MT_TOOL_PEN 和
    MT_TOOL_PALM [#f2]_。对于 type B 设备，此事件由输入核心处理；驱动应当改为使用
    input_mt_report_slot_state()。一个接触的 ABS_MT_TOOL_TYPE 在仍触摸着设备的
    同时可能随时间改变，因为固件在它首次出现时可能无法确定正在使用哪种工具。

ABS_MT_BLOB_ID
    BLOB_ID 把多个包分组到一个任意形状的接触中。这些点的序列构成一个多边形，它定义
    了接触的形状。这是 type A 设备的低级匿名分组，不应与高级的 trackingID 混淆
    [#f5]_。大多数 type A 设备没有 blob 能力，因此驱动可以安全地省略此事件。

ABS_MT_TRACKING_ID
    TRACKING_ID 在一个已初始化的接触的整个生命周期内标识它 [#f5]_。TRACKING_ID 的
    值范围应当足够大，以确保对长时间维持的接触进行唯一标识。对于 type B 设备，此
    事件由输入核心处理；驱动应当改为使用 input_mt_report_slot_state()。


### 事件计算

不同硬件的多样性不可避免地导致某些设备比其他设备更契合 MT 协议。为了简化并统一
映射，本节给出如何计算某些事件的诀窍。

对于把接触报告为矩形形状的设备，无法获得带符号的方向。假设 X 和 Y 是触摸矩形
各边的长度，这里是一个保留最多信息（保留长宽方向）的简单公式
```
   ABS_MT_TOUCH_MAJOR := max(X, Y)
   ABS_MT_TOUCH_MINOR := min(X, Y)
   ABS_MT_ORIENTATION := bool(X > Y)

```
ABS_MT_ORIENTATION 的范围应当设为 [0, 1]，以指示设备能够区分沿 Y 轴的手指（0）
和沿 X 轴的手指（1）。

```
   ABS_MT_POSITION_X := T_X
   ABS_MT_POSITION_Y := T_Y
   ABS_MT_TOOL_X := C_X
   ABS_MT_TOOL_Y := C_Y

```
遗憾的是，没有足够的信息来同时指定触摸椭圆和工具椭圆，所以必须诉诸于近似。一种
```
   ABS_MT_TOUCH_MAJOR := min(X, Y)
   ABS_MT_TOUCH_MINOR := <not used>
   ABS_MT_ORIENTATION := <not used>
   ABS_MT_WIDTH_MAJOR := min(X, Y) + distance(T, C)
   ABS_MT_WIDTH_MINOR := min(X, Y)

```
理由：我们没有关于触摸椭圆方向的信息，所以用内切圆来近似它。工具椭圆应当对齐
到向量（T - C），所以直径必须随 distance(T, C) 增大。最后，假设触摸直径等于工具
厚度，我们就得到了上面的公式。

### 手指跟踪

手指跟踪的过程，即给表面上每个已初始化的接触分配一个唯一的 trackingID，是一个
欧几里得二分匹配（Euclidean Bipartite Matching）问题。在每次事件同步时，实际接触
的集合与上一次同步的接触集合相匹配。完整的实现可以在 [#f3]_ 中找到。


### 手势

在创建手势事件的特定应用中，TOUCH 和 WIDTH 参数可以用来例如近似手指压力，或者
区分食指和拇指。通过添加 MINOR 参数，还可以区分扫过的手指和指向的手指，而通过
ORIENTATION，可以检测手指的扭转。


### 注意事项

为了保持与现有应用程序的兼容，在手指包中报告的数据不得被识别为单点触摸事件。

对于 type A 设备，所有手指数据都绕过输入过滤，因为相同类型的后续事件指的是不同的
手指。
