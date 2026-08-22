
## mgb4 驱动


Copyright |copy| 2023 - 2025 Digiteq Automotive
    author: Martin T暖ma <martin.tuma@digiteqautomotive.com>

这是一个面Digiteq Automotive FrameGrabber 4 v4l2 设备驱动，该设备是一PCIe 卡，能够采集并生FPD-Link III GMSL2/3 视频流，用于汽车工业领域
### sysfs 接口


mgb4 驱动提供了一sysfs 接口，用于配置视频流相关参数（其中部分参数必须在
v4l2 设备打开前正确设置）并获取视频设视频流的状态
参数分为两类 —全局 / PCI 卡相关参数位`/sys/class/video4linux/videoX/device` 下，模块相关参数位于
`/sys/class/video4linux/videoX` 下
#### 全局（PCI 卡）参数


**module_type** (R):
    模块类型
    | 0 - 鏃犳ā鍧?    | 1 - FPDL3
    | 2 - GMSL3（一个串行器，两个菊花链串联的解串器    | 3 - GMSL3（一个串行器，两个解串器    | 4 - GMSL3（两个解串器，带两个菊花链输出）
    | 6 - GMSL1
    | 8 - GMSL3 同轴

**module_version** (R):
    模块版本号。模块缺失时0
**fw_type** (R):
    固件类型
    | 1 - FPDL3
    | 2 - GMSL3
    | 3 - GMSL1

**fw_version** (R):
    固件版本号
**serial_number** (R):
```
        PRODUCT-REVISION-SERIES-SERIAL

    where each component is a 8b number.

```
#### 通用 FPDL3/GMSL 输入参数


**input_id** (R):
    输入编号 ID，从 0 开始
**oldi_lane_width** (RW):
    解串器输出通道（lane）数量
    | 0 - 单通道
    | 1 - 双通道（默认）

**color_mapping** (RW):
    信号中传入的比特到像素颜色比特的映射
    | 0 - OLDI/JEIDA
    | 1 - SPWG/VESA（默认）
    | 2 - ZDML

**link_status** (R):
    视频链路状态。若链路已锁定，则芯片已正确连接并以相同的速率和协议通信    即使没有活动的视频流，链路也可以处于锁定状态
    值为 0 等价V4L2 VIDIOC_ENUMINPUT 状态位V4L2_IN_ST_NO_SYNC 标志
    | 0 - 未锁    | 1 - 锁定

**stream_status** (R):
    视频流状态。当链路已锁定、输入像素时钟在运行DE 信号在移动时，即检测到流
    值为 0 等价V4L2 VIDIOC_ENUMINPUT 状态位V4L2_IN_ST_NO_SIGNAL 标志
    | 0 - 未检测到
    | 1 - 已检测到

**video_width** (R):
    视频流宽度。这是硬件检测到的实际宽度
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 width 字段    返回的值相同
**video_height** (R):
    视频流高度。这是硬件检测到的实际高度
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 height 字段    返回的值相同
**vsync_status** (R):
    视频格式检测器检测到VSYNC 脉冲类型
    该值等价于 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 polarities
    字段中返回的标志
    | 0 - 低有    | 1 - 高有    | 2 - 不可
**hsync_status** (R):
    视频格式检测器检测到HSYNC 脉冲类型
    该值等价于 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 polarities
    字段中返回的标志
    | 0 - 低有    | 1 - 高有    | 2 - 不可
**vsync_gap_length** (RW):
    如果输入视频信号不包含同VSYNC HSYNC 脉冲，则必须FPGA 内部生成这些
    脉冲以实现正确的帧排序。此值表示生成内VSYNC 脉冲需要多少个“空”像    （Data Enable 信号被撤销的像素）
**hsync_gap_length** (RW):
    如果输入视频信号不包含同VSYNC HSYNC 脉冲，则必须FPGA 内部生成这些
    脉冲以实现正确的帧排序。此值表示生成内HSYNC 脉冲需要多少个“空”像    （Data Enable 信号被撤销的像素）。该值必须大1 且小vsync_gap_length
**pclk_frequency** (R):
    输入像素时钟频率，单kHz
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 pixelclock 字段    返回的值相同
    *注意：必须首先正确设frequency_range 参数，才能在此处获得有效频率

**hsync_width** (R):
    HSYNC 信号宽度，以 PCLK 时钟周期计
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 hsync 字段    返回的值相同
**vsync_width** (R):
    VSYNC 信号宽度，以 PCLK 时钟周期计
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 vsync 字段    返回的值相同
**hback_porch** (R):
    HSYNC 信号撤销到视频行中第一个有效像素（DE=1 标记）之间的 PCLK 脉冲数
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 hbackporch 字段    返回的值相同
**hfront_porch** (R):
    视频行中最后一个有效像素（DE=1 标记）结束到 HSYNC 信号置位之间    PCLK 脉冲数
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 hfrontporch 字段    返回的值相同
**vback_porch** (R):
    VSYNC 信号撤销到包含第一个有效像素（DE=1 标记）的视频行之间的视频行数
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 vbackporch 字段    返回的值相同
**vfront_porch** (R):
    最后一个有效像素行（由 DE=1 标记）结束到 VSYNC 信号置位之间的视频行数
    该值与 VIDIOC_QUERY_DV_TIMINGS v4l2_bt_timings 结构体的 vfrontporch 字段    返回的值相同
**frequency_range** (RW):
    OLDI 输入时钟发生器的 PLL 频率范围。PLL 频率由像素时钟频率（PCLK）派生，
    oldi_lane_width 设为 "single" 时等PCLK，设"dual" 时等PCLK/2
    | 0 - PLL < 50MHz（默认）
    | 1 - PLL >= 50MHz

    *注意：在输入 v4l2 设备打开时，此参数不可更改

#### 通用 FPDL3/GMSL 输出参数


**output_id** (R):
    输出编号 ID，从 0 开始
**video_source** (RW):
    输出视频源。设0 1 时，源为相应的卡输入，且 v4l2 输出设备被禁用    设为 2 3 时，源为相应v4l2 视频输出设备。默认为相应v4l2 输出    OUT1 2，OUT2 3
    | 0 - 输入 0
    | 1 - 输入 1
    | 2 - v4l2 输出 0
    | 3 - v4l2 输出 1

    *注意：在任何输入/输出 v4l2 设备打开时，此参数不可更改

**display_width** (RW):
    显示宽度。由于没有对连接显示的自动检测，必须在开始流式传输前设置正确的值    默认宽度1280
    *注意：在输出 v4l2 设备打开时，此参数不可更改

**display_height** (RW):
    显示高度。由于没有对连接显示的自动检测，必须在开始流式传输前设置正确的值    默认高度640
    *注意：在输出 v4l2 设备打开时，此参数不可更改

**color_mapping** (RW):
    信号中传出的比特到像素颜色比特的映射
    | 0 - OLDI/JEIDA
    | 1 - SPWG/VESA（默认）
    | 2 - ZDML

**frame_rate** (RW):
    输出视频信号帧率限制，单fps。由于输出像素时钟步进有限，卡并不总能生成
    与连接显示所需值完全匹配的帧率。使用该参数可以通过“削弱”信号来限制帧率    使各行不相等（最后一行的 porch 不同），但信号对连接显示呈现出精确的帧率    默认帧率限制60Hz
**hsync_polarity** (RW):
    HSYNC 信号极性
    | 0 - 低有效（默认    | 1 - 高有
**vsync_polarity** (RW):
    VSYNC 信号极性
    | 0 - 低有效（默认    | 1 - 高有
**de_polarity** (RW):
    DE 信号极性
    | 0 - 低有    | 1 - 高有效（默认
**pclk_frequency** (RW):
    输出像素时钟频率。允许值在 25000-190000(kHz) 之间，两个相邻允许频率之间为
    非线性步进。驱动会找到最接近给定值的允许频率并设置为该值。读取此属性时    得到的是驱动设置的确切频率。默认频率为 61150kHz
    *注意：在输出 v4l2 设备打开时，此参数不可更改

**hsync_width** (RW):
    HSYNC 信号宽度，以像素计。默认值为 40
**vsync_width** (RW):
    VSYNC 信号宽度，以视频行计。默认值为 20
**hback_porch** (RW):
    HSYNC 信号撤销到视频行中第一个有效像素（DE=1 标记）之间的 PCLK 脉冲数    默认值为 50
**hfront_porch** (RW):
    视频行中最后一个有效像素（DE=1 标记）结束到 HSYNC 信号置位之间    PCLK 脉冲数。默认值为 50
**vback_porch** (RW):
    VSYNC 信号撤销到包含第一个有效像素（DE=1 标记）的视频行之间的视频行数    默认值为 31
**vfront_porch** (RW):
    最后一个有效像素行（由 DE=1 标记）结束到 VSYNC 信号置位之间的视频行数    默认值为 30
#### FPDL3 专有输入参数


**fpdl3_input_width** (RW):
    解串器输入线路数量
    | 0 - 自动（默认）
    | 1 - 鍗?    | 2 - 鍙。
#### FPDL3 专有输出参数


**fpdl3_output_width** (RW):
    串行器输出线路数量
    | 0 - 自动（默认）
    | 1 - 鍗?    | 2 - 鍙。
#### GMSL 专有输入参数


**gmsl_mode** (RW):
    GMSL 速率模式
    | 0 - 12Gb/s（默认）
    | 1 - 6Gb/s
    | 2 - 3Gb/s
    | 3 - 1.5Gb/s

**gmsl_stream_id** (RW):
    GMSL 多流最多包含四个视频流。此参数选择由视频输入捕获哪个流。该值为流的
    从零开始的索引。默认流 id 0
    *注意：在输入 v4l2 设备打开时，此参数不可更改

**gmsl_fec** (RW):
    GMSL 前向纠错（FEC）
    | 0 - 禁用
    | 1 - 启用（默认）

### MTD 分区


mgb4 驱动会创建一个带两个分区MTD 设备 - mgb4-fw.X - FPGA 固件 - mgb4-data.X - 出厂设置，例如卡序列号
**mgb4-fw** 分区可写，用于固件更新；**mgb4-data** 只读。附加在分区名上**X**
代表卡编号。根CONFIG_MTD_PARTITIONED_MASTER 内核配置，系统中可能还会第三个名**mgb4-flash** 的分区可用。该分区代表整块未分区的FLASH 内存不应去改动它…
### IIO（触发器

mgb4 驱动会创建一个工I/O（IIO）设备，提供触发与信号电平状态能力。以下扫元素可用
**activity**:
	触发电平与挂起状态
	| bit 1 - 触发 1 挂起
	| bit 2 - 触发 2 挂起
	| bit 5 - 触发 1 电平
	| bit 6 - 触发 2 电平

**timestamp**:
	触发事件时间戳
iio 设备可运行在“raw”模式下，通过 sysfs 访问获取信号电平（activity 5 6），
或运行在触发缓冲模式下。在触发缓冲模式下，可通过 /dev 中的 iio 设备跟踪信号电平
变化（activity 1 2）。若启用时间戳，还能获得可与视频帧匹配的精确触发事件
时间（每mgb4 视频帧都带有一个使用相同时钟源的时间戳）
*注意：尽activity 样本始终包含所有状态位，但raw 模式下获取挂起位、或触发缓冲模式下获取电平位没有意义 —这种情况下这些值并不代表有效数据
