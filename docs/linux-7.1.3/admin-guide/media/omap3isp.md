## OMAP 3 图像信号处理器（ISP）驱

Copyright |copy| 2010 Nokia Corporation

Copyright |copy| 2009 Texas Instruments, Inc.

Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>,
Sakari Ailus <sakari.ailus@iki.fi>, David Cohen <dacohen@gmail.com>


### 简

本文档描述了位于 drivers/media/platform/ti/omap3isp Texas Instruments OMAP 3 图像信号
处理器（ISP）驱动。原始驱动由 Texas Instruments 编写，但此后Nokia 被重写（两次）
该驱动已成功用于以下版本OMAP 3
- 3430
- 3530
- 3630

该驱动实现了 V4L2、Media controller v4l2_subdev 接口。支持内核中使用 v4l2_subdev 接口传感器、镜头和闪光灯驱动

### 拆分subdev


OMAP 3 ISP 被拆分为 V4L2 subdev，ISP 内部的每个模块都有一subdev 来表示它。每subdev
都向用户空间提供一V4L2 subdev 接口
- OMAP3 ISP CCP2
- OMAP3 ISP CSI2a
- OMAP3 ISP CCDC
- OMAP3 ISP preview
- OMAP3 ISP resizer
- OMAP3 ISP AEWB
- OMAP3 ISP AF
- OMAP3 ISP histogram

ISP 中每个可能的链接都由 Media controller 接口中的一个链接来建模。示例程序见 [#]_

### 控制 OMAP 3 ISP


一般而言，提供给 OMAP 3 ISP 的设置会在下一帧开始时生效。这发生在模块在传感器的垂直消隐
期间变为空闲时。在内存到内存操作中，流水线一次运行一帧。设置的应用在帧之间进行
ISP 中的所有模块，CSI-2 以及可能还有 CCP2 接收器外，都坚持接收完整的帧。因此传感器
绝不能向 ISP 发送不完整的帧
至少3430 上，autoidle 与某ISP 模块存在问题。autoidle 仅在 3630 上、且 omap3isp 模块
参数 autoidle 非零时启用
### 技术参考手册（TRM）及其他文档


OMAP 3430 TRM:
<URL:http://focus.ti.com/pdfs/wtbu/OMAP34xx_ES3.1.x_PUBLIC_TRM_vZM.zip>
Referenced 2011-03-05.

OMAP 35xx TRM:
<URL:http://www.ti.com/litv/pdf/spruf98o> Referenced 2011-03-05.

OMAP 3630 TRM:
<URL:http://focus.ti.com/pdfs/wtbu/OMAP36xx_ES1.x_PUBLIC_TRM_vQ.zip>
Referenced 2011-03-05.

DM 3730 TRM:
<URL:http://www.ti.com/litv/pdf/sprugn4h> Referenced 2011-03-06.


### 参考文