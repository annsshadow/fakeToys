
######## ioctl VIDIOC_G_ENC_INDEX


## 名称


VIDIOC_G_ENC_INDEX - 获取压缩视频流的元数
## 摘要


`int ioctl(int fd, VIDIOC_G_ENC_INDEX, struct v4l2_enc_idx *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_enc_idx` 的指针
## 描述


VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX> ioctl 提供关于当前由本驱动或另一应用程序驱动读取的压缩视频流的元数据，这对于无需解码即可随机访问该流非常有用
为了读取数据，应用程序必须调VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX>，并传入一指向 struct `v4l2_enc_idx` 的指针。成功时，驱动会填充 `entry` 数组将写入的元素个数存入 `entries` 字段，并初始`entries_cap` 字段
`entry` 数组的每个元素包含一幅图像的元数据。一VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX>
调用会从驱动缓冲区中读取最`V4L2_ENC_IDX_ENTRIES` 个条目，该缓冲区最多可容纳
`entries_cap` 个条目。该数字可以高于或低`V4L2_ENC_IDX_ENTRIES`，但不能为零。当
应用程序未能及时读取元数据时，最旧的条目将会丢失。当缓冲区为空或没有进行捕获/编码时，
`entries` 将为零
目前ioctl 仅针MPEG-2 program stream video elementary stream 定义


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 3 8

    - - __u32
      - `entries`
      - 驱动存入 `entry` 数组的条目数量    - - __u32
      - `entries_cap`
      - 驱动可缓冲的条目数量。必须大于零    - - __u32
      - `reserved`\ [^4^]
      - 保留供将来扩展。驱动必须将数组置零    - - struct `v4l2_enc_idx_entry`
      - `entry`\ [`V4L2_ENC_IDX_ENTRIES`]
      - 关于压缩视频流的元数据。数组的每个元素对应一幅图像，	`offset` 升序排列


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `offset`
      - 从压缩视频流开头到本幅图像开头的字节偏移量，mpeg2part1
	中定义的 *PES 包头，或 mpeg2part2 中定义的 **图像头部*	当编码器停止时，驱动将偏移量重置为零    - - __u64
      - `pts`
      - 本幅图像33 **显示时间戳（Presentation Time Stamp*	定义mpeg2part1    - - __u32
      - `length`
      - 本幅图像的字节长度    - - __u32
      - `flags`
      - 包含本幅图像编码类型的标志位，参enc-idx-flags    - - __u32
      - `reserved`\ [^2^]
      - 保留供将来扩展。驱动必须将数组置零


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_IDX_FRAME_I`
      - 0x00
      - 这是帧内编码图像（I 帧）    - - `V4L2_ENC_IDX_FRAME_P`
      - 0x01
      - 这是前向预测编码图像（P 帧）    - - `V4L2_ENC_IDX_FRAME_B`
      - 0x02
      - 这是双向预测编码图像（B 帧）    - - `V4L2_ENC_IDX_FRAME_MASK`
      - 0x0F
      - flags 字段与此掩码**AND** 运算即可得到图像编码类型
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并设`errno` 变量通用错误码在 Generic Error Codes <gen-errors> 一章中描述