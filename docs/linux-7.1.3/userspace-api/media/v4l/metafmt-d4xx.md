
######## V4L2_META_FMT_D4XX ('D4XX')


Intel D4xx UVC 摄像头元数据


## 描述


Intel D4xx（D435、D455 及其他）摄像头在UVC 有效负载头部中包含每帧元数据遵循 Microsoft(R) UVC 扩展提案 [1_]。这意味着，遵循标UVC 头部的私D4XX
元数据按块组织。D4xx 摄像头实现了 Microsoft 提出的若干标准块类型，以及若专有块类型。支持的标准元数据类型为 MetadataId_CaptureStats（ID 3）MetadataId_CameraExtrinsics（ID 4）和 MetadataId_CameraIntrinsics（ID 5）其说明见 [1_]。本文档描述 D4xx 摄像头使用的专有元数据类型
V4L2_META_FMT_D4XX 缓冲区遵V4L2_META_FMT_UVC 的元数据缓冲区布局，唯一区别在于它还包含专有有效负载头部数据。D4xx 摄像头使用批量传输，每帧仅发一个有效负载，因此其头部不能超255 字节
本文档实Intel 配置版本 3 [9_]
以下D4xx 摄像头使用的专有 Microsoft 风格元数据类型，所有字段均采用小端序：



    :widths: 1 2
    :header-rows:  1
    :stub-columns: 0

    - - **字段**
      - **描述**
    - - `1` **深度控制**
    - - __u32 ID
      - 0x80000000
    - - __u32 Size
      - 字节数，包含 ID（所有协议版本：60    - - __u32 Version
      - 本结构体的版本。本文档涵盖版本 1 3。新增字段时版本号会递增    - - __u32 Flags
      - 标志位掩码：见下[2_]
    - - __u32 Gain
      - 以内部单位表示的增益值，与用于采集该帧的 V4L2_CID_GAIN 控件相同
    - - __u32 Exposure
      - 采集该帧所用的曝光时间（微秒）
    - - __u32 Laser power
      - 激LED 的功0-360，用于深度测    - - __u32 AE mode
      - 0：手动；1：自动曝    - - __u32 Exposure priority
      - 曝光优先级值：0 - 恒定帧率
    - - __u32 AE ROI left
      - AE 感兴趣区域（ROI）的左边界（所ROI 值均以像素为单位，且分别介于 0 与最大宽度或最大高度之间）
    - - __u32 AE ROI right
      - AE ROI 的右边界
    - - __u32 AE ROI top
      - AE ROI 的上边界
    - - __u32 AE ROI bottom
      - AE ROI 的下边界
    - - __u32 Preset
      - 预设选择器值，默认值：0，除非用户修    - - __u8 Emitter mode (v3 only) (__u32 Laser mode for v1) [8_]
      - 0：关闭，1：开启，v1 __u32 Laser mode 相同
    - - __u8 RFU byte (v3 only)
      - 预留字节，供将来使用
    - - __u16 LED Power (v3 only)
      - LED 功率0-360（F416 SKU
    - - `1` **采集时序**
    - - __u32 ID
      - 0x80000001
    - - __u32 Size
      - 字节数，包含 ID（所有协议版本：40    - - __u32 Version
      - 本结构体的版本。本文档对应版本 xxx。新增字段时版本号会递增    - - __u32 Flags
      - 标志位掩码：见下[3_]
    - - __u32 Frame counter
      - 单调递增计数    - - __u32 Optical time
      - 从帧开始到帧中间的时间（微秒）
    - - __u32 Readout time
      - 读出一帧所用的时间（微秒）
    - - __u32 Exposure time
      - 帧曝光时间（微秒    - - __u32 Frame interval
      - 单位微秒 = 1000000 / 帧率
    - - __u32 Pipe latency
      - 从帧开始到数据进入 USB 缓冲区的时间（微秒）

    - - `1` **配置**
    - - __u32 ID
      - 0x80000002
    - - __u32 Size
      - 字节数，包含 ID（v1:36，v3:40    - - __u32 Version
      - 本结构体的版本。本文档对应版本 xxx。新增字段时版本号会递增    - - __u32 Flags
      - 标志位掩码：见下[4_]
    - - __u8 Hardware type
      - 摄像头硬件版[5_]
    - - __u8 SKU ID
      - 摄像头硬件配[6_]
    - - __u32 Cookie
      - 内部同步
    - - __u16 Format
      - 图像格式代码 [7_]
    - - __u16 Width
      - 宽度（像素）
    - - __u16 Height
      - 高度（像素）
    - - __u16 Framerate
      - 请求的每秒帧    - - __u16 Trigger
      - 字节 0：bit 0：深度与 RGB 已同步，bit 1：外部触    - - __u16 Calibration count (v3 only)
      - 校准计数器，见下[4_]
    - - __u8 GPIO input data (v3 only)
      - GPIO 读出，见下方 [4_]（自固件 5.12.7.0 起支持）
    - - __u32 Sub-preset info (v3 only)
      - 子预设选择信息，见下方 [4_]
    - - __u8 reserved (v3 only)
      - RFU 字节

[^1^] https://docs.microsoft.com/en-us/windows-hardware/drivers/stream/uvc-extensions-1-5


```
0x00000001 Gain
0x00000002 Exposure
0x00000004 Laser power
0x00000008 AE mode
0x00000010 Exposure priority
0x00000020 AE ROI
0x00000040 Preset
0x00000080 Emitter mode
0x00000100 LED Power
```

```
0x00000001 Frame counter
0x00000002 Optical time
0x00000004 Readout time
0x00000008 Exposure time
0x00000010 Frame interval
0x00000020 Pipe latency
```

```
0x00000001 Hardware type
0x00000002 SKU ID
0x00000004 Cookie
0x00000008 Format
0x00000010 Width
0x00000020 Height
0x00000040 Framerate
0x00000080 Trigger
0x00000100 Cal count
0x00000200 GPIO Input Data
0x00000400 Sub-preset Info
```

```
0 DS5
1 IVCAM2
```

```
  [1:0] depthCamera
	00: no depth
	01: standard depth
	10: wide depth
	11: reserved
  [2]   depthIsActive - has a laser projector
  [3]   RGB presence
  [4]   Inertial Measurement Unit (IMU) presence
  [5]   projectorType
	0: HPTG
	1: Princeton
  [6]   0: a projector, 1: an LED
  [7]   reserved
```

[^7^] 各视频流接口的图像格式代码：

```
1 Z16
2 Z
```

```
1 Y8
2 UYVY
3 R8L8
4 Calibration
5 W10
```

```
1 RAW8
```

[^8^] "Laser mode" 在版3 中已被三个不同的字段取代由于摄像头投影仪有多种技术，"Laser" 已重命名"Emitter"。由于另"Laser Power" 字段，我们为额外的发射器引入"LED Power"
```
   1 __u8 Emitter mode
   2 __u8 RFU byte
   3 __u16 LED Power
```
这是版本 1 3 之间的变更。版1 均与相同的数据格式向后兼容，
且均受支持。哪些属性有效请[2_]

[^9^] LibRealSense SDK 元数据来源：
https://github.com/IntelRealSense/librealsense/blob/master/src/metadata.h
