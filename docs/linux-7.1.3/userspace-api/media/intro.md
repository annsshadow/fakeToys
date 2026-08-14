## 简介


本文档涵盖 Linux 内核提供给用户空间的 API，这些 API 用于视频与无线电
流媒体设备，包括摄像机、模拟与数字电视接收卡、AM/FM 接收卡、软件定义
无线电（SDR）、流捕获与输出设备、编解码设备以及遥控控制器。

典型的媒体设备硬件如图所示 typical_media_device。


    :alt:   typical_media_device.svg
    :align: center

    典型媒体设备

媒体基础设施 API 旨在控制此类设备。它分为五个部分。

1. 第一部分 <v4l2spec> 涵盖无线电、视频捕获与输出、摄像机、模拟电视设备以及编解码器。

2. 第二部分 <dvbapi> 涵盖用于数字电视以及经由多种数字电视标准之一
   进行互联网接收的 API。虽然它被称为 DVB API，但实际上它涵盖若干
   不同的视频标准，包括 DVB-T/T2、DVB-S/S2、DVB-C、ATSC、ISDB-T、
   ISDB-S、DTMB 等。所支持标准的完整列表可在 `fe_delivery_system` 处找到。

3. 第三部分 <remote_controllers> 涵盖遥控控制器 API。

4. 第四部分 <media_controller> 涵盖媒体控制器 API。

5. 第五部分 <cec> 涵盖 CEC（Consumer Electronics Control，消费电子控制）API。

还应注意，媒体设备可能也具有音频组件，例如混音器、PCM 捕获、PCM 回放等，
它们通过 ALSA API 进行控制。有关更多信息以及最新的开发代码，请参阅：
`https://linuxtv.org <https://linuxtv.org>`__。有关讨论改进、报告问题、
发送新驱动等，请发送邮件至：`Linux Media Mailing List (LMML)
<http://vger.kernel.org/vger-lists.html#linux-media>`__。
