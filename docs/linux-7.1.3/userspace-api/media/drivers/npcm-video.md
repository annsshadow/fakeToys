


## NPCM 视频驱动


该驱动用于控Nuvoton NPCM SoC 上存在的视频捕获/差分（VCD）引擎与编码压缩（ECE）引擎。VCD 可以从数字视频输入捕获一帧，并在内存中比较两帧；ECE 可以将帧数据压缩HEXTILE 格式
### 驱动专用控制


#### V4L2_CID_NPCM_CAPTURE_MODE


VCD 引擎支持两种模式
- COMPLETE 模式
  将下一完整帧捕获到内存中
- DIFF 模式
  将输入帧与内存中存储的帧进行比较，并更新内存中的差分帧
应用程序可以使用 `V4L2_CID_NPCM_CAPTURE_MODE` 控制，通过不同的控制值（enum v4l2_npcm_capture_mode）设VCD 模式
- `V4L2_NPCM_CAPTURE_MODE_COMPLETE`：将 VCD 设置COMPLETE 模式- `V4L2_NPCM_CAPTURE_MODE_DIFF`：将 VCD 设置DIFF 模式
#### V4L2_CID_NPCM_RECT_COUNT


如果使用 V4L2_PIX_FMT_HEXTILE 格式，VCD 将捕获帧数据，然ECE 将数据压缩为 HEXTILE 矩形，并按照远程帧缓冲协议（Remote Framebuffer Protocol）中定义的布局存储V4L2 视频缓冲区中```

           (RFC 6143, https://www.rfc-editor.org/rfc/rfc6143.html#section-7.6.1)

           +--------------+--------------+-------------------+
           | No. of bytes | Type [Value] | Description       |
           +--------------+--------------+-------------------+
           | 2            | U16          | x-position        |
           | 2            | U16          | y-position        |
           | 2            | U16          | width             |
           | 2            | U16          | height            |
           | 4            | S32          | encoding-type (5) |
           +--------------+--------------+-------------------+
           |             HEXTILE rectangle data              |
           +-------------------------------------------------+

```
应用程序可以通过 VIDIOC_DQBUF 获取视频缓冲区，然后调用 `V4L2_CID_NPCM_RECT_COUNT` 控制来获取该缓冲区中 HEXTILE 矩形的数量
### 参

include/uapi/linux/npcm-video.h

**Copyright** |copy| 2022 Nuvoton Technologies
