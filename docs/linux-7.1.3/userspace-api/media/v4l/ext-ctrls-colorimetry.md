


######## 色彩管理（Colorimetry）控制参

Colorimetry 类包含用于表示数字图像和视频中颜色的高动态范围（HDR）成像控制。这些控制应用于视频和图像编解码，以HDMI 接收器和发送器
### Colorimetry 控制 ID


`V4L2_CID_COLORIMETRY_CLASS (class)`
    Colorimetry 类描述符。对该控制调VIDIOC_QUERYCTRL 将返回该控制类的描述
`V4L2_CID_COLORIMETRY_HDR10_CLL_INFO (struct)`
    内容亮度级别（Content Light Level）定义了图像标称目标亮度光照级别的上限



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `max_content_light_level`
      - 视频序列各图像所有单个采样中最大光照级别的上限，单cd/m\ `2`        当为 0 时表示不存在这样的上限    - - __u16
      - `max_pic_average_light_level`
      - 视频序列任意单幅图像采样中最大平均光照级别的上限，单cd/m\ `2`        当为 0 时表示不存在这样的上限
`V4L2_CID_COLORIMETRY_HDR10_MASTERING_DISPLAY (struct)`
    母版显示（mastering display）定义了被认为是当前视频内容母版显示的显示器的色域（颜色原色、白点和亮度范围）



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `display_primaries_x[^3^]`
      - 指定母版显示颜色原色分量 c 的归一x 色度坐标，增量为 0.00002        对于使用红、绿、蓝颜色原色的母版显示，索引c 等于 0 对应绿原色，
        c 等于 1 对应蓝原色，c 等于 2 对应红色原色    - - __u16
      - `display_primaries_y[^3^]`
      - 指定母版显示颜色原色分量 c 的归一y 色度坐标，增量为 0.00002        对于使用红、绿、蓝颜色原色的母版显示，索引c 等于 0 对应绿原色，
        c 等于 1 对应蓝原色，c 等于 2 对应红色原色    - - __u16
      - `white_point_x`
      - 指定母版显示白点的归一x 色度坐标，增量为 0.00002    - - __u16
      - `white_point_y`
      - 指定母版显示白点的归一y 色度坐标，增量为 0.00002    - - __u32
      - `max_luminance`
      - 指定母版显示的标称最大显示亮度，单位 0.0001 cd/m\ `2`    - - __u32
      - `min_luminance`
      - 指定母版显示的标称最小显示亮度，单位 0.0001 cd/m\ `2`