
## Qualcomm 摄像头子系统驱动


### 简

本文件记录了位于 drivers/media/platform/qcom/camss 下的 Qualcomm 摄像子系统驱动
当前版本的驱动支持在 Qualcomm MSM8916/APQ8016 以及 MSM8996/APQ8096
处理器上发现的摄像头子系统
该驱动实现了 V4L2、Media controller（媒体控制器）以V4L2 subdev 接口支持在内核中使用 V4L2 subdev 接口的摄像头传感器
该驱动以 Code Linaro 中的 Qualcomm 摄像头子系统 Android 驱动 [#f1]_ [#f2]_
作为参考实现

### Qualcomm 摄像头子系统硬件


驱动所支持8x16 / 8x96 处理器上的摄像头子系统硬件由以下部分组成
- 2 / 3 CSIPHY 模块。它们处CSI2 接收器的物理层。每CSIPHY 模块
  可连接一个独立的摄像头传感器- 2 / 4 CSID（CSI 解码器）模块。它们处CSI2 接收器的协议层与应用
  层。一CSID 可以解码来自任意 CSIPHY 的数据流。每CSID 还包含一  TG（测试生成器）块，可用于生成人工输入数据以进行测试；
- ISPIF（ISP 接口）模块。负责将数据流从 CSID 路由VFE 的输入端- 1 / 2 VFE（视频前端）模块。包含一条图像处理的硬件块流水线。VFE
  具有多种输入接口。PIX（像素）输入接口将输入数据送入图像处理流水线  图像处理流水线末端还包含一个缩放与裁剪模块。三RDI（Raw Dump
  Interface，原始转储接口）输入接口会绕过图像处理流水线。VFE 还包  将输出数据写入内存的 AXI 总线接口

### 支持的功

当前版本的驱动支持：

- 通过 CSIPHY 来自摄像头传感器的输入；
- CSID 中的 TG 生成测试输入数据- VFE RDI 接口

  - 将输入数据原始转储到内存
    支持的格式：

    - YUYV/UYVY/YVYU/VYUY（打YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY）；
    - MIPI RAW8锛? 浣?Bayer RAW - V4L2_PIX_FMT_SRGGB8 /
      V4L2_PIX_FMT_SGRBG8 / V4L2_PIX_FMT_SGBRG8 / V4L2_PIX_FMT_SBGGR8）；
    - MIPI RAW100 位打Bayer RAW - V4L2_PIX_FMT_SBGGR10P /
      V4L2_PIX_FMT_SGBRG10P / V4L2_PIX_FMT_SGRBG10P / V4L2_PIX_FMT_SRGGB10P /
      V4L2_PIX_FMT_Y10P）；
    - MIPI RAW122 位打Bayer RAW - V4L2_PIX_FMT_SRGGB12P /
      V4L2_PIX_FMT_SGBRG12P / V4L2_PIX_FMT_SGRBG12P / V4L2_PIX_FMT_SRGGB12P）    - （仅 8x96）MIPI RAW144 位打Bayer RAW - V4L2_PIX_FMT_SRGGB14P /
      V4L2_PIX_FMT_SGBRG14P / V4L2_PIX_FMT_SGRBG14P / V4L2_PIX_FMT_SRGGB14P）
  - （仅 8x96）输入数据的格式转换
    支持的输入格式：

    - MIPI RAW100 位打Bayer RAW - V4L2_PIX_FMT_SBGGR10P / V4L2_PIX_FMT_Y10P）
    支持的输出格式：

    - Plain16 RAW100 位非打包 Bayer RAW - V4L2_PIX_FMT_SBGGR10 / V4L2_PIX_FMT_Y10）
- VFE PIX 接口

  - 输入数据的格式转换
    支持的输入格式：

    - YUYV/UYVY/YVYU/VYUY（打YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY）
    支持的输出格式：

    - NV12/NV21（双平面 YUV 4:2:0 - V4L2_PIX_FMT_NV12 / V4L2_PIX_FMT_NV21）；
    - NV16/NV61（双平面 YUV 4:2:2 - V4L2_PIX_FMT_NV16 / V4L2_PIX_FMT_NV61）    - （仅 8x96）YUYV/UYVY/YVYU/VYUY（打YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY）
  - 缩放支持。配VFE Encoder Scale 模块以进行最16x 的缩小
  - 裁剪支持。配VFE Encoder Crop 模块
- 两个x96：三个）数据输入的并发与独立使用——可以是摄像头传感器
  鍜，鎴?TG銆。

### 驱动架构与设

该驱动实现了 V4L2 subdev 接口。为了对模块之间的硬件连接进行建模，暴露一个干净、合乎逻辑且可用的接口，驱动按如下方式拆分V4L2 子设x16 / 8x96）：

- 2 / 3 CSIPHY 子设备——每CSIPHY 由一个独立的子设备表示；
- 2 / 4 CSID 子设备——每CSID 由一个独立的子设备表示；
- 2 / 4 ISPIF 子设备——ISPIF 由数量与 CSID 子设备相等的子设备表示；
- 4 / 8 VFE 子设备——VFE 由数量与输入接口数相等的子设备表示（每个
  VFE 3 RDI 1 PIX）
以此特定方式拆分驱动的理由如下：

- CSIPHY CSID 模块各自表示为一个独立的子设备，可以对这些模块之  的硬件连接进行建模；
- VFE 的每个输入接口表示为独立的子设备，可以并发且独立地使用这  输入接口，正如硬件所支持的那样；
- ISPIF 表示为数量与 CSID 子设备相等的子设备，可以在同时使用两  摄像头时创建线性的媒体控制器流水线。这避免了流水线中的分支，否  分支会要a) 用户空间以及 b) 媒体框架（例如上下电操作）对从单  媒体实体sink pad source pad 的数据流做出假设
每个 VFE 子设备都连接到一个独立的视频设备节点
媒体控制器流水线图如下（连接了两/ 三个 OV5645 摄像头传感器）：


    :alt:   qcom_camss_graph.dot
    :align: center

    Media pipeline graph 8x16

    :alt:   qcom_camss_8x96_graph.dot
    :align: center

    Media pipeline graph 8x96


### 实现


当前所支持的功能并不需要硬件的运行时配置（在流传输过程中更新设置）每个硬件模块的完整配置都STREAMON ioctl 时，根据当前激活的媒体链路格式和已设置的控制项进行应用
VFE 中缩放器模块的输出尺寸，'msm_vfe0_pix' 实体sink pad 上实际的
compose 选区矩形来配置
VFE 中裁剪模块的输出裁剪区域，由 'msm_vfe0_pix' 实体source pad 实际crop 选区矩形来配置

### 文档


APQ8016 规格https://developer.qualcomm.com/download/sd410/snapdragon-410-processor-device-specification.pdf
引用日期 2016-11-24
APQ8096 规格https://developer.qualcomm.com/download/sd820e/qualcomm-snapdragon-820e-processor-apq8096sge-device-specification.pdf
引用日期 2018-06-22
### 参