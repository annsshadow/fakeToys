


######## ioctl VIDIOC_G_FREQUENCY, VIDIOC_S_FREQUENCY


## 名称


VIDIOC_G_FREQUENCY - VIDIOC_S_FREQUENCY - 获取或设置调谐器（tuner）或调制器（modulator）的无线电频
## 概要



`int ioctl(int fd, VIDIOC_G_FREQUENCY, struct v4l2_frequency *argp)`


`int ioctl(int fd, VIDIOC_S_FREQUENCY, const struct v4l2_frequency *argp)`

## 参数



`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_frequency` 的指针
## 描述


要获取当前的调谐器或调制器无线电频率，应用程序将结构`v4l2_frequency` `tuner` 字段设为对应的调谐器或调制器编号（只有输入设备才有调谐器，只有输出设备才有调制器），`reserved` 数组清零，然后以指向该结构的指针调用 VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl。驱动将当前频率存入 `frequency` 字段
要更改当前的调谐器或调制器无线电频率，应用程序初始化结构`v4l2_frequency` `tuner`、`type` `frequency` 字段以及 `reserved` 数组，然后以指向该结构的指针调用 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl。当所请求的频率不可实现时，驱动会取最接近的可行值。不VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> 是一个只ioctl，它并不返回实际的新频率


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tuner`
      - 调谐器或调制器索引编号。该值与结构`v4l2_input` `tuner` 字段、结构体 `v4l2_tuner` `index` 字段，或结构`v4l2_output` `modulator` 字段、结构体 `v4l2_modulator` `index` 字段相同    - - __u32
      - `type`
      - 调谐器类型。该值与结构`v4l2_tuner` `type` 字段相同。对`/dev/radioX` 设备节点，该类型必须设为 `V4L2_TUNER_RADIO`；对于所有其他节点则设为 `V4L2_TUNER_ANALOG_TV`。对调制器应设为 `V4L2_TUNER_RADIO`（目前仅支持无线电调制器）。参`v4l2_tuner_type`
    - - __u32
      - `frequency`
      - 调谐频率，单位为 62.5 kHz；若设置了结构体 `v4l2_tuner` 或结构体 `v4l2_modulator` `capability` 标志 `V4L2_TUNER_CAP_LOW`，则单位62.5 Hz。当设置`capability` 标志 `V4L2_TUNER_CAP_1HZ` 时，使用 1 Hz 为单位    - - __u32
      - `reserved`\ [^8^]
      - 保留以备将来扩展。驱动和应用程序都必须将该数组置零
## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 一章中描述
EINVAL
    `tuner` 索引越界，或 `type` 字段中的值错误
EBUSY
    硬件搜索（seek）正在进行中