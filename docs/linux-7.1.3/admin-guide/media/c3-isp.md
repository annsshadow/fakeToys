
## Amlogic C3 图像信号处理（C3ISP）驱

## 简

本文件记录位drivers/media/platform/amlogic/c3/isp 下的 Amlogic C3ISP 驱动
当前版本的驱动支Amlogic C308L 处理器上C3ISP
该驱动实V4L2、Media controller V4L2 子设备接口。支持内核中使用 V4L2 子设备接口的
摄像头传感器
该驱动已AW419-C308L-Socket 平台上测试
## Amlogic C3 ISP


C308L 处理器上由驱动支持的摄像头硬件包括：

- 1 MIPI-CSI-2 模块：处MIPI CSI-2 接收器的物理层，并从连接的摄像头传感器接收数据- 1 MIPI-ADAPTER 模块：组MIPI 数据以满ISP 输入要求，并MIPI 数据发送给 ISP- 1 ISP（图像信号处理）模块：包含一条图像处理硬件块流水线。ISP 流水线末端有三个
  缩放器，每个缩放器都连接到一DMA 接口，将输出数据写入内存
```

                                                                   +----------+    +-------+
                                                                   | Resizer  |--->| WRMIF |
  +---------+    +------------+    +--------------+    +-------+   |----------+    +-------+
  | Sensor  |--->| MIPI CSI-2 |--->| MIPI ADAPTER |--->|  ISP  |---|----------+    +-------+
  +---------+    +------------+    +--------------+    +-------+   | Resizer  |--->| WRMIF |
                                                                   +----------+    +-------+
                                                                   |----------+    +-------+
                                                                   | Resizer  |--->| WRMIF |
                                                                   +----------+    +-------+

```
## 驱动架构与设

为了对模块之间的硬件链接建模，并暴露一个清晰、合乎逻辑且易用的接口，该驱动注册以下
V4L2 子设备：

- 1 `c3-mipi-csi2` 子设- MIPI CSI-2 接收- 1 `c3-mipi-adapter` 子设- MIPI 适配- 1 `c3-isp-core` 子设- ISP 核心
- 3 `c3-isp-resizer` 子设- ISP 缩放
`c3-isp-core` 子设备链接到 2 个视频设备节点，分别用于统计信息捕获与参数编程：

- 用于统计信息捕获`c3-isp-stats` 捕获视频设备节点
- 用于参数编程`c3-isp-params` 输出视频设备

每个 `c3-isp-resizer` 子设备链接到一个用于捕获帧的捕获视频设备节点：

- `c3-isp-resizer0` 链接`c3-isp-cap0` 捕获视频设备
- `c3-isp-resizer1` 链接`c3-isp-cap1` 捕获视频设备
- `c3-isp-resizer2` 链接`c3-isp-cap2` 捕获视频设备

媒体控制器流水线图如下（连接IMX290 摄像头传感器）：


    :alt:   c3-isp.dot
    :align: center

    媒体流水线拓
## 实现


ISP 硬件的运行时配置`c3-isp-params` 视频设备节点上执行，使用 :ref:`V4L2_META_FMT_C3ISP_PARAMS
<v4l2-meta-fmt-c3isp-params>` 作为数据格式。缓冲区结构`c3_isp_params_cfg` 定义
统计信息使用 V4L2_META_FMT_C3ISP_STATS <v4l2-meta-fmt-c3isp-stats> 数据格式`c3-isp-stats` 视频设备节点捕获
最终的图片尺寸与格式使`c3-isp-cap[0, 2]` 视频设备节点上的 V4L2 视频捕获接口配置
Amlogic C3 ISP `libcamera <https://libcamera.org>`_ 支持，带有一个专用的流水线处理器
以及执行运行时图像校正与增强的算法