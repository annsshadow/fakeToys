


## MIPI CCS 摄像头传感器驱动


MIPI CCS 摄像头传感器驱动是用于符合 `MIPI CCS
<https://www.mipi.org/specifications/camera-command-set>`_ 规范的摄像头传感器的通用驱动。
它暴露三个子设备，分别代表像素阵列（pixel array）、合并器（binner）与缩放器（scaler）。

由于各个设备的能力不同，驱动基于硬件中实际存在的能力来暴露接口。

另请参见 CCS 驱动内核文档 <media-ccs-driver>。

### 像素阵列子设备


像素阵列子设备代表摄像头传感器的像素矩阵，以及许多合规设备中存在的模拟裁剪（analogue
crop）功能。模拟裁剪通过在实体的源 pad（0）上使用 `V4L2_SEL_TGT_CROP` 来配置。像素
矩阵的尺寸可通过获取 `V4L2_SEL_TGT_NATIVE_SIZE` 目标得到。

### 合并器（Binner）


binner 子设备代表传感器上的合并（binning）功能。为此，在汇（sink）pad（0）上支持
选择目标 `V4L2_SEL_TGT_COMPOSE`。

此外，如果设备没有缩放器或数字裁剪功能，源 pad（1）会暴露另一个只能在行尾与帧尾裁剪的
数字裁剪选择矩形。

### 缩放器（Scaler）


scaler 子设备代表传感器的数字裁剪与缩放功能。当支持数字裁剪时，使用 V4L2 选择目标
`V4L2_SEL_TGT_CROP` 在汇 pad（0）上配置数字裁剪。缩放也使用汇 pad（0）上的选择目标
`V4L2_SEL_TGT_COMPOSE` 配置。

此外，如果 scaler 子设备存在，其源 pad（1）会暴露另一个只能在行尾与帧尾裁剪的数字
裁剪选择矩形。

### 数字裁剪与模拟裁剪


数字裁剪功能指的是通过直接丢弃部分数据来生效的裁剪。而模拟裁剪则意味着被裁剪掉的信息
永远不会被读取。对于摄像头传感器，模拟数据永远不会从像素矩阵中位于所配置、表示裁剪的
选择矩形之外的部分读取。这种差异会影响设备时序，很可能也会影响功耗。

### 私有控件


MIPI CCS 驱动在 `V4L2_CID_USER_BASE_CCS` 下实现了若干私有控件，用于控制符合 MIPI CCS
规范的摄像头传感器。

#### 模拟增益模型


CCS 定义了一种模拟增益模型，其中增益可使用以下公式计算：

	gain = m0 ** x + c0 / (m1 ** x + c1)

m0 或 c0 其中之一将为零。设备相关的常量可从以下控件获取：

	V4L2_CID_CCS_ANALOGUE_GAIN_M0
	V4L2_CID_CCS_ANALOGUE_GAIN_M1
	V4L2_CID_CCS_ANALOGUE_GAIN_C0
	V4L2_CID_CCS_ANALOGUE_GAIN_C1

模拟增益（公式中的 `x`）在此情况下通过 `V4L2_CID_ANALOGUE_GAIN` 控制。

#### 备选模拟增益模型


CCS 定义了另一种称为“备选模拟增益（alternate analogue gain）”的模拟增益模型。在此模型下，
计算实际增益的公式由线性部分与指数部分组成：

	gain = linear * 2 ^ exponent

`linear` 与 `exponent` 因子可分别使用 `V4L2_CID_CCS_ANALOGUE_LINEAR_GAIN` 与
`V4L2_CID_CCS_ANALOGUE_EXPONENTIAL_GAIN` 控件设置。

#### 阴影校正


CCS 标准支持镜头阴影（lens shading）校正。该特性可通过 `V4L2_CID_CCS_SHADING_CORRECTION`
控制。此外，亮度校正级别可通过 `V4L2_CID_CCS_LUMINANCE_CORRECTION_LEVEL` 更改，其中
值 0 表示不校正，128 表示将角落处的亮度校正为比中心低 10 %。

亮度校正级别要生效，必须先启用阴影校正。

**Copyright** |copy| 2020 Intel Corporation
