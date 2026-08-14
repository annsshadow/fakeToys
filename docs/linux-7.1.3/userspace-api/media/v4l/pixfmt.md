

# 图像格式

V4L2 API主要是为交换图像数据的设备而设计的
与应用程序。结构体`v4l2_pix_format`和
struct `v4l2_pix_format_mplane` 结构体定义了
内存中图像的格式和布局。前者与
单平面 API，而后者与多平面 API 一起使用
版本（请参阅 planar-apis）。图像格式与协商
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。 （这里的解释
专注于视频捕获和输出，适用于覆盖帧缓冲区格式
另请参见 VIDIOC_G_FBUF <VIDIOC_G_FBUF>。）


- [pixfmt-v4l2](pixfmt-v4l2)
- [pixfmt-v4l2-mplane](pixfmt-v4l2-mplane)
- [pixfmt-intro](pixfmt-intro)
- [pixfmt-indexed](pixfmt-indexed)
- [pixfmt-rgb](pixfmt-rgb)
- [pixfmt-bayer](pixfmt-bayer)
- [yuv-formats](yuv-formats)
- [hsv-formats](hsv-formats)
- [depth-formats](depth-formats)
- [pixfmt-compressed](pixfmt-compressed)
- [sdr-formats](sdr-formats)
- [tch-formats](tch-formats)
- [meta-formats](meta-formats)
- [pixfmt-reserved](pixfmt-reserved)
- [colorspaces](colorspaces)
- [colorspaces-defs](colorspaces-defs)
- [colorspaces-details](colorspaces-details)
