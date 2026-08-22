
## i.MX 视频捕获驱动


## 事件


### ipuX_csiY


该子设备在启用第二个 IDMAC pad 时可以生成以下事件：

- V4L2_EVENT_IMX_FRAME_INTERVAL_ERROR

用户应用程序可以ipuX_csiY 子设备节点订阅此事件。该事件由帧间隔监视器（FIM）生成（关于 FIM 详见下文）
## 控制


### ipuX_csiY 中的帧间隔监视器


adv718x 解码器在 NTSC/PAL 信号重新同步期间偶尔会发送损坏的场（视频行数过少或过多）。发生这种情况时，IPU 会触发一种机制，通过每帧添加 1 行伪行来重新建立垂直同步，这会导致图像之间出现滚动效应，并可能在恢复稳定图像之前持续很长时间。有时该机制完全不起作用，导致永久的分裂图像（一帧包含来自两个连续捕获图像的行）
通过实验发现，在图像滚动期间，帧间隔（两EOF 之间经过的时间）会低于当前标准的标称值约一个帧时间0 微秒），并保持该值直到滚动停止
虽然造成该现象的原因未知（IPU 的伪行机制本应使间隔每帧增加一个行时间，而不是一个固定值），但我们可以利用它通过帧间隔监视器来检测损坏的场。如FIM 检测到一个坏的帧间隔，ipuX_csiY 子设备将发送事V4L2_EVENT_IMX_FRAME_INTERVAL_ERROR。用户态可以使FIM 事件通知ipuX_csiY 子设备节点上注册。用户态在收到此事件时可以发起一次流重启以修正滚分裂图像
ipuX_csiY 子设备包含用于微FIM 部分参数的自定义控制。如果在流传输期间更改了其中某个控制，FIM 将被重置并以新设置继续
- V4L2_CID_IMX_FIM_ENABLE

启用/禁用 FIM
- V4L2_CID_IMX_FIM_NUM

在与传感器报告的标称帧间隔比较之前，要平均多少个帧间隔测量值。这可减少由中断延迟引起的噪声
- V4L2_CID_IMX_FIM_TOLERANCE_MIN

如果平均间隔偏离标称值超过此量（单位为微秒），则发V4L2_EVENT_IMX_FRAME_INTERVAL_ERROR 事件
- V4L2_CID_IMX_FIM_TOLERANCE_MAX

如果任何间隔高于此值，则这些样本将被丢弃，不进入平均。这可用于丢弃可能因高系统负载中断延迟导致的极高间隔错误
- V4L2_CID_IMX_FIM_NUM_SKIP

FIM 重置或流重启后，在开始平均间隔之前要跳过的帧数
- V4L2_CID_IMX_FIM_ICAP_CHANNEL / V4L2_CID_IMX_FIM_ICAP_EDGE

这些控制将配置一个输入捕获通道作为测量帧间隔的方法。这优于默认的通过 EOF 中断测量帧间隔的方法，因为它不受中断延迟引入的不确定误差影响
输入捕获需要硬件支持。必须将 VSYNC 信号路由到某i.MX6 输入捕获通道 pad
V4L2_CID_IMX_FIM_ICAP_CHANNEL 配置使用哪个 i.MX6 输入捕获通道。必须为 0 1
V4L2_CID_IMX_FIM_ICAP_EDGE 配置哪个信号边沿触发输入捕获事件。默认输入捕获方法被禁用，值为 IRQ_TYPE_NONE。将该控制设IRQ_TYPE_EDGE_RISING、IRQ_TYPE_EDGE_FALLING IRQ_TYPE_EDGE_BOTH 可启用输入捕获，在给定信号边沿触发
输入捕获被禁用时，帧间隔将通过 EOF 中断测量

### 文件列表


drivers/staging/media/imx/
include/media/imx.h
include/linux/imx-media.h


### 作

- Steve Longerbeam <steve_longerbeam@mentor.com>
- Philipp Zabel <kernel@pengutronix.de>
- Russell King <linux@armlinux.org.uk>

Copyright (C) 2012-2017 Mentor Graphics Inc.
