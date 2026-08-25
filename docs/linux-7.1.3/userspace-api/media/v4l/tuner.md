


######## 璋冭皭鍣ㄤ笌璋冨埗鍣。


## 璋冭皭鍣。

视频输入设备可以拥有一个或多个解调 RF 信号的调谐器。每个调谐器关联一个或多个
视频输入，具体取决于调谐器上 RF 连接器的数量。由 VIDIOC_ENUMINPUT ioctl 返回相应结构`v4l2_input` `type` 字段被设`V4L2_INPUT_TYPE_TUNER`，其
`tuner` 字段包含该调谐器的索引号
射频输入设备恰好有一个索引为 0 的调谐器，没有视频输入
应用程序使用 VIDIOC_G_TUNER <VIDIOC_G_TUNER> VIDIOC_S_TUNER <VIDIOC_G_TUNER> ioctl 分别查询和更改调谐器属性。VIDIOC_G_TUNER <VIDIOC_G_TUNER>
返回`v4l2_tuner` 结构体还包含当前视频或射频输入所对应的调谐器被查询时
适用的信号状态信息

   VIDIOC_S_TUNER <VIDIOC_G_TUNER> 在有多个调谐器时并不会切换当前调谐器。调谐器
   完全由当前视频输入决定。当设备拥有一个或多个调谐器时，驱动必须同时支持这两个
   ioctl，并VIDIOC_QUERYCAP ioctl 返回`v4l2_capability` 结构体中设置
   `V4L2_CAP_TUNER` 标志

## 璋冨埗鍣。

视频输出设备可以拥有一个或多个调制器，用于将视频信号调制后辐射出去，或连接电视机或录像机的天线输入端。每个调制器关联一个或多个视频输出，具体取决于调制RF 连接器的数量。由 VIDIOC_ENUMOUTPUT ioctl 返回的相`v4l2_output` 结构体的 `type` 字段被设`V4L2_OUTPUT_TYPE_MODULATOR`，其
`modulator` 字段包含该调制器的索引号
射频输出设备恰好有一个索引为 0 的调制器，没有视频输出
视频或射频设备不能同时支持调谐器和调制器。此类硬件必须使用两个独立的设备节点一个支持调谐器功能，一个支持调制器功能。原因在VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>
ioctl 的限制：无法指明频率是给调谐器还是调制器使用
应用程序使用 VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl 查询和更改调制器属性。注意，当存在多调制器时，VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> 并不会切换当前调制器。调制器完全由当前视频输出决定。当设备拥有
一个或多个调制器时，驱动必须同时支持这两个 ioctl，并VIDIOC_QUERYCAP ioctl 返回`v4l2_capability` 结构体中设置
`V4L2_CAP_MODULATOR` 标志

## 射频


应用程序使用 VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl 来获取和设置调谐器或调制器的射频频率这两ioctl 都接受一个指`v4l2_frequency` 结构体的指针。这ioctl 同样适用电视和射频设备。当支持调谐器或调制ioctl，或设备为射频设备时，驱动必须同支持这两ioctl
