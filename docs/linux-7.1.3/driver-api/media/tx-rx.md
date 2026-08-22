


## 像素数据发送与接收驱动


V4L2 支持各种发送和接收像素数据的设备。这些设备的例子包括摄像头传感器、TV 调谐器，
以及 SoC 中的并行接收器、BT.656 接收器或 CSI-2 接收器
### 总线类型


以下总线最为常见。本节仅讨论这两种
##### MIPI CSI-2


CSI-2 是一种用于将图像从摄像头传输到主SoC 的数据总线。它`MIPI alliance`_ 定义
##### 并行总线BT.656


并行总线`BT.656`_ 总线在每个时钟周期每条数据线上传输一位数据。并行总线使用同步
和其他附加信号，BT.656 则将同步信息嵌入其中
### 发送器驱动


发送器驱动通常需要向接收器驱动提供发送器的配置。所需内容取决于总线类型。以下两为两条总线所共有
##### 濯掍綋鎬荤嚎鍍忕礌鐮。

参见 v4l2-mbus-pixelcode
##### 链路频率


V4L2_CID_LINK_FREQ <v4l2-cid-link-freq> 控件用于告知接收器总线的频率（即它与符号率
不同）
没有用户可配置链路频率的驱动应该通过 `.get_mbus_config()` 子设pad 操作，在
struct v4l2_mbus_config `link_freq` 字段中报告它，而不是通过控件
接收器驱动应使用 `v4l2_get_link_freq` 辅助函数从发送器子设备获取链路频率
##### ``.enable_streams()`` ``.disable_streams()`` 回调


struct v4l2_subdev_pad_ops->enable_streams() 鍜?struct
v4l2_subdev_pad_ops->disable_streams() 回调由接收器驱动用于控制发送器驱动的流状态这些回调不应被直接调用，而应通过 `v4l2_subdev_enable_streams()` `v4l2_subdev_disable_streams()` 调用
##### 停止发送器


发送器通过调用 `.disable_streams()` 回调来停止发送图像流。某些发送器可能会在帧边处停止流，而另一些会立即停止，从而实际上使当前帧未完成。接收器驱动不应在任一方面
做假设，而应在两种情况下都能正常工作
### CSI-2 发送器驱动


##### 鍍忕礌鐜。

```

	pixel_rate = link_freq * 2 * nr_of_lanes * 16 / k / bits_per_sample

```
其中

   :header-rows: 1

   - - variable or constant
     - description
   - - link_freq
     - `V4L2_CID_LINK_FREQ` integer64 菜单项的值   - - nr_of_lanes
     - CSI-2 链路上使用的数据通道数   - - 2
     - 数据在信号的上升沿和下降沿都传输   - - bits_per_sample
     - 每样本的位数   - - k
     - D-PHY 16，C-PHY 7
关于使用的是 D-PHY 还是 C-PHY，以`nr_of_lanes` 的值，可以OF 端点配置中获取
	以这种方式计算的像素*并不**等同于摄像头传感器像素阵列上的像素率，后者由
	V4L2_CID_PIXEL_RATE <v4l2-cid-pixel-rate> 控件指示
##### LP-11 LP-111 状

作为过渡到高速模式的一部分，CSI-2 发送器通常会根PHY 的不同，短暂地将总线设置LP-11 LP-111 状态。这段时间可能短100 µs，在此期间接收器观察到此状态并继续自身的高速模式过渡
大多数接收器一旦被软件配置好，就能自主处理这一点，但也存在需要软件参与观LP-11 LP-111 状态的接收器00 µs 在软件中是一个很短的时间窗口，尤其是在没有中断告知发了什么的情况下
一种解决方法是显式地将发送器侧配置为 LP-11 LP-111 状态，这需要发送器硬件的支持这并非普遍可用。许多设备在停止流后返回到此状态，而加电后的状态是 LP-00 LP-000
`.pre_streamon()` 回调可用于为发送器准备过渡到流状态，但还不开始流。类似地`.post_streamoff()` 回调用于撤销 `.pre_streamon()` 回调所做的事情。因此，`.pre_streamon()`
的调用者必须为每次成功`.pre_streamon()` 调用去调`.post_streamoff()`
CSI-2 的语境下，`.pre_streamon()` 回调用于将发送器过渡LP-11 LP-111 状态。这
也需要给设备上电，因此应仅在需要时这样做
不需要显LP-11 LP-111 状态设置的接收器驱动免于调用这两个回调