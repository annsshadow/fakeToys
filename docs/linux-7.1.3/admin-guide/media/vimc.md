
## 虚拟媒体控制器驱动（vimc）


vimc 驱动使用 V4L2 API 和 Media API 模拟复杂的视频硬件。它有一个捕获设备以及三个子设备：sensor（传感器）、debayer（去拜耳）和 scaler（缩放器）。

### 拓扑


拓扑是硬编码的，不过你可以修改 vimc-core 并重新编译驱动来实现自己的拓扑。这是默认拓扑：


    :alt:   默认媒体流水线拓扑图
    :align: center

    vimc 上的媒体流水线图

#### 配置拓扑


每个子设备都会带有其默认配置（pixelformat、height、width 等）。需要配置该拓扑，使每个被链接子设备上的配置相互匹配，才能通过流水线流式传输帧。如果配置不匹配，流将失败。`v4l-utils` 软件包是一组用户空间应用程序的集合，自带 `media-ctl` 和 `v4l2-ctl`，可用于配置 vimc 配置。以下命令序列适用于默认拓扑：


        media-ctl -d platform:vimc -V '"Sensor A":0[fmt:SBGGR8_1X8/640x480]'
        media-ctl -d platform:vimc -V '"Debayer A":0[fmt:SBGGR8_1X8/640x480]'
        media-ctl -d platform:vimc -V '"Scaler":0[fmt:RGB888_1X24/640x480]'
        media-ctl -d platform:vimc -V '"Scaler":0[crop:(100,50)/400x150]'
        media-ctl -d platform:vimc -V '"Scaler":1[fmt:RGB888_1X24/300x700]'
        v4l2-ctl -z platform:vimc -d "RGB/YUV Capture" -v width=300,height=700
        v4l2-ctl -z platform:vimc -d "Raw Capture 0" -v pixelformat=BA81

### 子设备


子设备定义了拓扑中实体（entity）的行为。根据子设备的不同，实体可以具有多个 source 或 sink 类型的 pad。

vimc-sensor:
	使用视频测试图案生成器以多种格式生成图像。
	暴露：

 - 1 个 source pad

vimc-lens:
	传感器的辅助镜头。支持自动对焦控制。使用辅助链接（ancillary link）连接到 vimc-sensor。该镜头支持 FOCUS_ABSOLUTE 控制。


	media-ctl -p
	...
 - entity 28: Lens A (0 pad, 0 link)
			type V4L2 subdev subtype Lens flags 0
			device node name /dev/v4l-subdev6
 - entity 29: Lens B (0 pad, 0 link)
			type V4L2 subdev subtype Lens flags 0
			device node name /dev/v4l-subdev7
	v4l2-ctl -d /dev/v4l-subdev7 -C focus_absolute
	focus_absolute: 0


vimc-debayer:
	将拜耳（bayer）格式的图像转换为非拜耳格式。
	暴露：

 - 1 个 sink pad
 - 1 个 source pad

vimc-scaler:
	重新调整图像大小以匹配 source pad 的分辨率。例如：如果 sink pad 配置为 360x480 而 source 配置为 1280x720，图像将被拉伸以适配 source 分辨率。适用于 vimc 限制内的任何分辨率（必要时甚至缩小图像）。
	暴露：

 - 1 个 sink pad
 - 1 个 source pad

vimc-capture:
	暴露节点 /dev/videoX 以允许用户空间捕获流。
	暴露：

 - 1 个 sink pad
 - 1 个 source pad

### 模块参数


Vimc 有一个用于配置驱动的模块参数。

- `allocator=<unsigned int>`

	内存分配器选择，默认为 0。它指定缓冲区的分配方式。

  - 0: vmalloc
  - 1: dma-contig
