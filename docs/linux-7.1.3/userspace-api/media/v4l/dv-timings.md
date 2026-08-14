


######## 数字视频（DV）时序（Digital Video Timings）


到目前为止所讨论的视频标准一直针对模拟电视（Analog TV）及其相应的视频时序。
如今有众多不同的硬件接口，例如高清电视接口（HDMI）、VGA、DVI 连接器等，它们
承载视频信号，因此需要扩展 API 来为这些接口选择视频时序。由于受限于可用的位数，
无法扩展 v4l2_std_id <v4l2-std-id>，因此新增了一组 ioctl 用于在输入与输出
端设置/获取视频时序。

这些 ioctl 处理定义每种视频格式的具体数字视频时序，包括活动视频宽度与高度、
信号极性、前肩（frontporch）、后肩（backporch）、同步宽度等参数。
`linux/v4l2-dv-timings.h` 头文件可用于获取 cea861 与 vesadmt 标准中各种格式的
时序。

为了枚举并查询设备所支持的 DV 时序属性，应用程序使用 VIDIOC_ENUM_DV_TIMINGS 与
VIDIOC_DV_TIMINGS_CAP ioctl。要设置设备的 DV 时序，应用程序使用
VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl；要获取当前的 DV 时序，则使用
VIDIOC_G_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> ioctl。要检测视频接收端看到的 DV
时序，应用程序使用 VIDIOC_QUERY_DV_TIMINGS ioctl。

当硬件检测到视频源发生变化（例如视频信号出现或消失，或视频分辨率改变）时，它会
发出一个 `V4L2_EVENT_SOURCE_CHANGE` 事件。使用 ioctl
VIDIOC_SUBSCRIBE_EVENT <VIDIOC_SUBSCRIBE_EVENT> 与 VIDIOC_DQEVENT 来检查该事件
是否已被上报。

如果视频信号发生变化，那么应用程序必须停止流传输、释放所有缓冲区，并调用
VIDIOC_QUERY_DV_TIMINGS 以获取新的视频时序；如果它们有效，则可以通过调用 ioctl
VIDIOC_S_DV_TIMINGS <VIDIOC_G_DV_TIMINGS> 来设置它们。这同时也会更新格式，因此
使用 ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 来获取新格式。现在应用程序可以分配新的
缓冲区并再次开始流传输。

VIDIOC_QUERY_DV_TIMINGS 只会报告硬件检测到的内容，它永远不会更改配置。如果当前
设置的时序与实际检测到的时序不同，通常这意味着你将无法采集到任何视频。正确的
做法是依赖 `V4L2_EVENT_SOURCE_CHANGE` 事件，以便知道何时发生了变化。

应用程序可以利用 input-capabilities 与 output-capabilities 标志来判断数字视频
ioctl 是否可用于给定的输入或输出。
