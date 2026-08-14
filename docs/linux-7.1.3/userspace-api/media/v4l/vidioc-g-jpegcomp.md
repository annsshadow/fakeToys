


######## ioctl VIDIOC_G_JPEGCOMP, VIDIOC_S_JPEGCOMP


## 名称（Name）


VIDIOC_G_JPEGCOMP - VIDIOC_S_JPEGCOMP

## 概要（Synopsis）


`int ioctl(int fd, VIDIOC_G_JPEGCOMP, v4l2_jpegcompression *argp)`


`int ioctl(int fd, VIDIOC_S_JPEGCOMP, const v4l2_jpegcompression *argp)`

## 参数（Arguments）


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_jpegcompression` 的指针。

## 描述（Description）


这些 ioctl 已**被弃用**。新的驱动与应用程序应使用 JPEG 类控件 <jpeg-controls> 来
控制图像质量与 JPEG 标记（markers）。

[待办]

Ronald Bultje 详细说明：

APP 是一些应用程序特定的信息。应用程序可以自行设置它，它会被存储在 JPEG 编码字段
中（例如，用于 AVI 中的交错信息等）。COM 与之相同，但它是注释，比如“由我编码”之类。

jpeg_markers 描述是否应将 Huffman 表、量化表与重启间隔信息（都是 JPEG 特定的
内容）存储在 JPEG 编码字段中。它们定义了 JPEG 字段如何被编码。如果省略它们，应用
程序会假定你使用了标准编码。你通常确实想要添加它们。


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - int
      - `quality`
      - 已弃用。如果驱动暴露了
	V4L2_CID_JPEG_COMPRESSION_QUALITY <jpeg-quality-control>
	控件，应用程序应使用它，并忽略此字段。
    - - int
      - `APPn`
      -
    - - int
      - `APP_len`
      -
    - - char
      - `APP_data`\ [^60^]
      -
    - - int
      - `COM_len`
      -
    - - char
      - `COM_data`\ [^60^]
      -
    - - __u32
      - `jpeg_markers`
      - 参见 jpeg-markers。已弃用。如果驱动暴露了
	V4L2_CID_JPEG_ACTIVE_MARKER <jpeg-active-marker-control>
	控件，应用程序应使用它，并忽略此字段。


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_JPEG_MARKER_DHT`
      - (1<<3)
      - 定义 Huffman 表
    - - `V4L2_JPEG_MARKER_DQT`
      - (1<<4)
      - 定义量化表
    - - `V4L2_JPEG_MARKER_DRI`
      - (1<<5)
      - 定义重启间隔
    - - `V4L2_JPEG_MARKER_COM`
      - (1<<6)
      - 注释段
    - - `V4L2_JPEG_MARKER_APP`
      - (1<<7)
      - App 段，驱动将始终使用 APP0

## 返回值（Return Value）


成功时返回 0，出错时返回 -1，并适当地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。
