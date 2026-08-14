######## 索引格式（Indexed Format）


在该格式中，每个像素由一个 8 位索引表示，索引指向一个包含 256 项的 ARGB 调色板。它仅用于视频输出叠加层 <osd>。没有用于访问调色板的 ioctl，这必须通过 Linux 帧缓冲 API 的 ioctl 来完成。


    :header-rows:  2
    :stub-columns: 0

    - - Identifier
      - Code
      -
      - `7` Byte 0
#     * -

      - Bit
      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0
    - .. _V4L2-PIX-FMT-PAL8:

      - `V4L2_PIX_FMT_PAL8`
      - 'PAL8'
      -
      - i\ `7`
      - i\ `6`
      - i\ `5`
      - i\ `4`
      - i\ `3`
      - i\ `2`
      - i\ `1`
      - i\ `0`
