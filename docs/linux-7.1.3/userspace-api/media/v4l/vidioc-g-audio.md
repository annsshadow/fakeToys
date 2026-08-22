


######## ioctl VIDIOC_G_AUDIO, VIDIOC_S_AUDIO


## 名称


VIDIOC_G_AUDIO - VIDIOC_S_AUDIO - 查询或选择当前的音频输入及其属
## 概要


`int ioctl(int fd, VIDIOC_G_AUDIO, struct v4l2_audio *argp)`


`int ioctl(int fd, VIDIOC_S_AUDIO, const struct v4l2_audio *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_audio` 的指针
## 描述


要查询当前音频输入，应用程序先将 struct `v4l2_audio` `reserved` 数组清零，然后以指向该结构的指针调用 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl。当设备没有音频输入，或者没有与当前视频输入相组合的音频输入时，驱动会填充结构的其余部分，或者返`EINVAL` 错误码
音频输入有一个可写属性，即音频模式。要选择当前音频输入***更改音频模式，应用程序初始化 struct `v4l2_audio` 结构`index` `mode` 字段以及 `reserved` 数组，然后调VIDIOC_S_AUDIO <VIDIOC_G_AUDIO> ioctl。如果请求无法被满足，驱动可能会切换到不同的音频模式。不过，这是一个只写（write-only）ioctl，它不会返回实际的新的音频模式


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 标识音频输入，由驱动或应用程序设置    - - __u8
      - `name`\ [^32^]
      - 音频输入的名称，一个以 NUL 结尾ASCII 字符串，例如Line In"。此信息供用户使用，最好是设备本身上的连接器标签    - - __u32
      - `capability`
      - 音频能力标志，参audio-capability    - - __u32
      - `mode`
      - 由驱动和应用程序设置的音频模式标志（VIDIOC_S_AUDIO <VIDIOC_G_AUDIO> ioctl 中），参audio-mode    - - __u32
      - `reserved`\ [^2^]
      - 保留供将来扩展。驱动和应用程序必须将该数组置零



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_AUDCAP_STEREO`
      - 0x00001
      - 这是一个立体声输入。该标志用于在信号始终为单声道时自动禁用立体声录制等。除非音频输入属于调谐器，否API 没有提供检测是*接收*立体声的手段    - - `V4L2_AUDCAP_AVL`
      - 0x00002
      - 支持自动音量电平（Automatic Volume Level）模式



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_AUDMODE_AVL`
      - 0x00001
      - AVL 模式开启
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 章节中描述
EINVAL
    没有音频输入与当前视频输入组合，或者所选音频输入的编号超出范围，或者它无法组合