
######## RF 调谐器控制参考


RF 调谐器（RF_TUNER）类包含具有 RF 调谐器设备的通用特性相关的控件。

在此语境下，RF 调谐器是介于天线与解调器之间的无线电接收电路。它从天线接收射频（RF），
并将接收到的信号转换为较低的中频（IF）或基带频率（BB）。能够输出基带的调谐器通常
被称为零中频（Zero-IF）调谐器。较老的调谐器通常是金属盒内的简单 PLL 调谐器，而较新的
则是高度集成的芯片（无金属盒的“硅调谐器”）。这些控件大多适用于功能丰富的新式硅调谐器，
因为较老的调谐器几乎没有多少可调特性。

有关 RF 调谐器的更多信息，请参见维基百科上的
`Tuner (radio) <http://en.wikipedia.org/wiki/Tuner_%28radio%29>`__
与 `RF front end <http://en.wikipedia.org/wiki/RF_front_end>`__。

## RF_TUNER 控件 ID


`V4L2_CID_RF_TUNER_CLASS (class)`
    RF_TUNER 类描述符。对此控件调用 VIDIOC_QUERYCTRL 将
    返回该控件类的描述。

`V4L2_CID_RF_TUNER_BANDWIDTH_AUTO (boolean)`
    启用/禁用调谐器无线电频道带宽配置。在自动模式下，带宽配置由驱动执行。

`V4L2_CID_RF_TUNER_BANDWIDTH (integer)`
    调谐器信号路径上的滤波器用于根据接收方的需求过滤信号。驱动配置滤波器以满足
    期望的带宽要求。在 V4L2_CID_RF_TUNER_BANDWIDTH_AUTO 未设置时使用。单位为 Hz。
    范围与步进由驱动决定。

`V4L2_CID_RF_TUNER_LNA_GAIN_AUTO (boolean)`
    启用/禁用 LNA 自动增益控制（AGC）

`V4L2_CID_RF_TUNER_MIXER_GAIN_AUTO (boolean)`
    启用/禁用混频器自动增益控制（AGC）

`V4L2_CID_RF_TUNER_IF_GAIN_AUTO (boolean)`
    启用/禁用 IF 自动增益控制（AGC）

`V4L2_CID_RF_TUNER_RF_GAIN (integer)`
    RF 放大器是接收信号路径上紧接天线输入之后的第一个放大器。本文档中 LNA 增益与
    RF 增益的区别在于：LNA 增益集成在调谐器芯片内，而 RF 增益是独立的芯片。
    同一设备中可能同时存在 RF 与 LNA 增益控件。范围与步进由驱动决定。

`V4L2_CID_RF_TUNER_LNA_GAIN (integer)`
    LNA（低噪声放大器）增益是 RF 调谐器信号路径上的第一级增益。它位于非常靠近调谐器
    天线输入的位置。在 `V4L2_CID_RF_TUNER_LNA_GAIN_AUTO` 未设置时使用。参见
    `V4L2_CID_RF_TUNER_RF_GAIN` 以了解 RF 增益与 LNA 增益的区别。范围与步进由驱动决定。

`V4L2_CID_RF_TUNER_MIXER_GAIN (integer)`
    混频器增益是 RF 调谐器信号路径上的第二级增益。它位于混频器块内部，RF 信号在此被
    混频器下变频。在 `V4L2_CID_RF_TUNER_MIXER_GAIN_AUTO` 未设置时使用。范围与步进
    由驱动决定。

`V4L2_CID_RF_TUNER_IF_GAIN (integer)`
    IF 增益是 RF 调谐器信号路径上的最后一级增益。它位于 RF 调谐器的输出端。它控制
    中频输出或基带输出的信号电平。在 `V4L2_CID_RF_TUNER_IF_GAIN_AUTO` 未设置时使用。
    范围与步进由驱动决定。

`V4L2_CID_RF_TUNER_PLL_LOCK (boolean)`
    合成器 PLL 是否已锁定？当该控件置位时，RF 调谐器正在接收给定的频率。这是一个只读控件。
