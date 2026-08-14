## Samsung S5P/EXYNOS4 FIMC 驱动

本文档面向媒体子系统开发者，介绍 Samsung S5P/Exynos4 平台 FIMC（数码图像处理控制器）驱动的代码组织与文件划分，列出媒体设备、相机采集、MIPI-CSI2 接收器与视频后处理器等模块对应的源文件，便于驱动开发与维护时快速定位实现。



Copyright |copy| 2012 - 2013 Samsung Electronics Co., Ltd.

### 文件划分


- 媒体设备驱动

  drivers/media/platform/samsung/exynos4-is/media-dev.[ch]

- 相机采集视频设备驱动

  drivers/media/platform/samsung/exynos4-is/fimc-capture.c

- MIPI-CSI2 接收器子设备

  drivers/media/platform/samsung/exynos4-is/mipi-csis.[ch]

- 视频后处理器（mem-to-mem）

  drivers/media/platform/samsung/exynos4-is/fimc-core.c

- 公共文件

  drivers/media/platform/samsung/exynos4-is/fimc-core.h
  drivers/media/platform/samsung/exynos4-is/fimc-reg.h
  drivers/media/platform/samsung/exynos4-is/regs-fimc.h
