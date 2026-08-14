


## ASPEED 视频驱动


AST2400/2500/2600 SoC 上的 ASPEED 视频引擎（Video Engine）支持高性能视频压缩，
具有广泛的视频质量和压缩比选项。所采用的压缩算法是一种修改过的 JPEG 算法。

该 IP 中有两种类型的压缩。

- JPEG JFIF 标准模式：用于单帧和管理压缩
- ASPEED 专有模式：用于多帧和差分压缩。
  支持 2-pass（高质量）视频压缩方案（ASPEED 专利申请中）。
  提供视觉无损的视频压缩质量，或在内部网 KVM 应用中降低网络平均负载。

VIDIOC_S_FMT 可用于选择你想要的格式。V4L2_PIX_FMT_JPEG
代表 JPEG JFIF 标准模式；V4L2_PIX_FMT_AJPG 代表 ASPEED 专有模式。

关于 ASPEED 视频硬件操作的更多细节，可在 `github <https://github.com/AspeedTech-BMC/openbmc/releases/>`__ 上提供的 SDK_User_Guide 的 **第 6.2.16 节 KVM Video Driver** 中找到。

ASPEED 视频驱动实现了以下驱动特定的控制：

### ``V4L2_CID_ASPEED_HQ_MODE``

    启用/禁用 ASPEED 的高质量模式。这是一个私有控制，
    可用于为 ASPEED 专有模式启用高质量。

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 4

    - - `(0)`
      - ASPEED HQ 模式已禁用。
    - - `(1)`
      - ASPEED HQ 模式已启用。

### ``V4L2_CID_ASPEED_HQ_JPEG_QUALITY``

    定义 ASPEED 高质量模式的质量。这是一个私有控制，
    如果启用了高质量模式，可用于决定压缩质量。
    值越大，质量越好，体积也越大。

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 4

    - - `(1)`
      - 最小值
    - - `(12)`
      - 最大值
    - - `(1)`
      - 步长
    - - `(1)`
      - 默认值

**Copyright** |copy| 2022 ASPEED Technology Inc.
