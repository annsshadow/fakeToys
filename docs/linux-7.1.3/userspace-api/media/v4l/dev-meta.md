


######## 元数据接口（Metadata Interface

元数据是指任何补充视频帧的额外信息的非图像数据。这可能包括基于图像计算的统计量、图像源提供的帧捕获参数，或用于指定设备如何处理图像的特定设备参数。该接口用于在用户空间和硬件之间传输元数据并控制该操作
元数据接口在视频设备节点上实现。设备可以专用于元数据，也可以根据其报告的能力同时支持视频和元数据
## 查询能力


支持元数据捕获接口的设备节点会在 `VIDIOC_QUERYCAP` ioctl 返回`v4l2_capability` 结构`device_caps` 字段中设`V4L2_CAP_META_CAPTURE` 标志。该标志表示设备可以将元数据捕获到内存。类似地，支持元数据输出接口的设备节点在 `v4l2_capability` 结构`device_caps` 字段中设`V4L2_CAP_META_OUTPUT` 标志。该标志表示设备可以从内存读取元数据
必须至少支持写或流式 I/O 方法之一

## 数据格式协商


元数据设备使format ioctl 来选择捕获格式。元数据缓冲区的內容格式绑定到所选格式。除了基本的 format ioctl，`VIDIOC_ENUM_FMT` ioctl 也必须支持
为使format ioctl，应用程序将 `v4l2_format` 结构`type` 字段设为 `V4L2_BUF_TYPE_META_CAPTURE` `V4L2_BUF_TYPE_META_OUTPUT`，并根据所需操作按需使用 `fmt` 联合`v4l2_meta_format` `meta` 成员。驱动和应用程序都必须将 `v4l2_format` 结构的其余部分设0
按行捕获元数据的设备`VIDIOC_ENUM_FMT` 时设置了 struct v4l2_fmtdesc `V4L2_FMT_FLAG_META_LINE_BASED` 标志。此类设备通常也可以捕获图像数<capture>。这主要涉及从其他设备（如相机传感器）接收数据的设备



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `dataformat`
      - 数据格式，由应用程序设置。这是一个小端序的四字符<v4l2-fourcc>。V4L2 meta-formats 中定义了元数据格式    - - __u32
      - `buffersize`
      - 数据所需的最大缓冲区大小（字节）。该值由驱动设置    - - __u32
      - `width`
      - 一行元数据在“数据单元”中的宽度。当 :c:type`v4l2_fmtdesc` 标志 `V4L2_FMT_FLAG_META_LINE_BASED` 被设置时有效，否则为零。参`VIDIOC_ENUM_FMT`    - - __u32
      - `height`
      - 元数据行数。当 :c:type`v4l2_fmtdesc` 标志 `V4L2_FMT_FLAG_META_LINE_BASED` 被设置时有效，否则为零。参`VIDIOC_ENUM_FMT`    - - __u32
      - `bytesperline`
      - 两个连续行的起始之间的字节偏移。当 :c:type`v4l2_fmtdesc` 标志 `V4L2_FMT_FLAG_META_LINE_BASED` 被设置时有效，否则为零。参`VIDIOC_ENUM_FMT`