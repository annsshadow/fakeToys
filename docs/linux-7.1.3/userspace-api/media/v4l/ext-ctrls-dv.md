

######## 数字视频控制参


数字视频（Digital Video）控制类旨在控制 `VGA <http://en.wikipedia.org/wiki/Vga>`__
`DVI <http://en.wikipedia.org/wiki/Digital_Visual_Interface>`__
（Digital Visual Interface，数字可视接口）、HDMI (hdmi) DisplayPort
(dp) 的接收器与发送器。这些控制通常预期为实现了它们的接收器或发送器子设备的私有控制
因此只暴露在 `/dev/v4l-subdev*` 设备节点上


   注意，这些设备可能有多个输入或输pad，它们连接到例如 HDMI 连接器。即使子设备
   只会向其中一pad 接收或发送视频，其他 pad EDID（Extended Display
   Identification Data，扩展显示识别数据，vesaedid）与 HDCP（High-bandwidth Digital
   Content Protection System，高带宽数字内容保护系统，hdcp）处理方面仍然可以是活跃的，
   从而使设备能够提前完成相对较慢EDID/HDCP 处理。这样就可以在连接器之间快速切换

这些 pad 在本节的多个控制中以位掩码的形式出现，每一位对应一pad。位 0 对应 pad 0
1 对应 pad 1，依此类推。控制的最大值即为有pad 的集合


## 数字视频控制 ID


`V4L2_CID_DV_CLASS (class)`
    数字视频类描述符

`V4L2_CID_DV_TX_HOTPLUG (bitmask)`
    许多连接器带有一个热插拔引脚，当来自源的 EDID 信息可用时该引脚为高电平。该控制显示发送器所见到的热插拔引脚状态。每一位对应发送器上的一个输pad。如果某个输pad 没有关联的热插拔引脚，则pad 对应的位0。该只读控制适用DVI-D、HDMI DisplayPort 连接器

`V4L2_CID_DV_TX_RXSENSE (bitmask)`
    Rx Sense 是对 TMDS 时钟线上拉电阻的检测。这通常意味着接收器已进入/退出待机（即发送器可以感知到接收器已准备好接收视频）。每一位对应发送器上的一个输pad。如果某个输pad 没有关联Rx Sense，则pad 对应的位0。该只读控制适用DVI-D HDMI 设备

`V4L2_CID_DV_TX_EDID_PRESENT (bitmask)`
    当发送器从接收器看到热插拔信号时，它会尝试读EDID。若已设置，则发送器至少已读取第一块（= 128 字节）。每一位对应发送器上的一个输pad。如果某个输pad 不支EDID，则pad 对应的位0。该只读控制适用VGA、DVI-A/D、HDMI DisplayPort 连接器

`V4L2_CID_DV_TX_MODE`
    (enum)

enum v4l2_dv_tx_mode -
    HDMI 发送器可以DVI-D 模式（仅视频）或 HDMI 模式（视+ 音频 + 辅助数据）发送。该控制选择使用哪种模式：V4L2_DV_TX_MODE_DVI_D V4L2_DV_TX_MODE_HDMI。该控制适用HDMI 连接器

`V4L2_CID_DV_TX_RGB_RANGE`
    (enum)

enum v4l2_dv_rgb_range -
    RGB 输出选择量化范围。V4L2_DV_RANGE_AUTO 遵循视频接口标准中规定的 RGB 量化范围（即 HDMI cea861）。V4L2_DV_RANGE_LIMITED V4L2_DV_RANGE_FULL 会覆盖标准，以兼容那些未正确实现标准的接收端（对HDMI DVI-D 而言这种情况相当常见）。全范围允许使用所有可能的值，而限制范围将范围设为 (16 << (N-8)) - (235 << (N-8))，其N 是每个分量的位数。该控制适用VGA、DVI-A/D、HDMI DisplayPort 连接器

`V4L2_CID_DV_TX_IT_CONTENT_TYPE`
    (enum)

enum v4l2_dv_it_content_type -
    配置所发送视频的 IT 内容类型。该信息作为 AVI InfoFrame 的一部分通过 HDMI DisplayPort 连接器发送。术语“IT Content”用于源自计算机的内容，以区别于电视广播或模拟源的内容。enum v4l2_dv_it_content_type 定义了可能的内容类型


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_DV_IT_CONTENT_TYPE_GRAPHICS`
      - 图形内容。像素数据应不经滤波、也不进行模拟重建地传递
    - - `V4L2_DV_IT_CONTENT_TYPE_PHOTO`
      - 照片内容。内容源自数字静态图片。内容应经过最小缩放与画质增强地传递
    - - `V4L2_DV_IT_CONTENT_TYPE_CINEMA`
      - 影院内容
    - - `V4L2_DV_IT_CONTENT_TYPE_GAME`
      - 游戏内容。应使音频与视频延迟最小化
    - - `V4L2_DV_IT_CONTENT_TYPE_NO_ITC`
      - 没有可用IT Content 信息，并AVI InfoFrame 中的 ITC 位被设为 0



`V4L2_CID_DV_RX_POWER_PRESENT (bitmask)`
    检测接收器是否从源接收到电源（例如 HDMI 在某根引脚上携带 5V）。这通常用于为包EDID 信息eeprom 供电，使得源即使在接收器处于待机/断电状态时也能读取 EDID。每一位对应接收器上的一个输pad。如果某个输pad 无法检测电源是否存在，则该 pad 对应的位0。该只读控制适用DVI-D、HDMI DisplayPort 连接器

`V4L2_CID_DV_RX_RGB_RANGE`
    (enum)

enum v4l2_dv_rgb_range -
    RGB 输入选择量化范围。V4L2_DV_RANGE_AUTO 遵循视频接口标准中规定的 RGB 量化范围（即 HDMI cea861）。V4L2_DV_RANGE_LIMITED V4L2_DV_RANGE_FULL 会覆盖标准，以兼容那些未正确实现标准的源（对HDMI DVI-D 而言这种情况相当常见）。全范围允许使用所有可能的值，而限制范围将范围设为 (16 << (N-8)) - (235 << (N-8))，其N 是每个分量的位数。该控制适用VGA、DVI-A/D、HDMI DisplayPort 连接器

`V4L2_CID_DV_RX_IT_CONTENT_TYPE`
    (enum)

enum v4l2_dv_it_content_type -
    读取所接收视频IT 内容类型。该信息作为 AVI InfoFrame 的一部分通过 HDMI DisplayPort 连接器发送。术语“IT Content”用于源自计算机的内容，以区别于电视广播或模拟源的内容。可用内容类型参`V4L2_CID_DV_TX_IT_CONTENT_TYPE`
