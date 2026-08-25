
## ivtv 驱动


作者：Hans Verkuil <hverkuil@kernel.org>

这是一个面Conexant cx23415/6 MPEG 编码解码器的 v4l2 设备驱动。cx23415
既能编码也能解码，cx23416 只能进行 MPEG 编码。目前唯一支持完整解码的卡Hauppauge PVR-350

   #) 本驱动需要最新的编码器固件（版本 2.06.039，大376836 字节）。请从此处获取固件：

      https://linuxtv.org/downloads/firmware/#conexant

   #) “普通”的电视应用程序无法与本驱动配合工作，你需要一个能够处MPEG
      输入的应用程序，例如 mplayer、xine、MythTV 等
IVTV 项目的主要目标是为基iCompression iTVC15 Conexant
CX23415/CX23416 MPEG 编解码器的视频采集卡，提供一个“净室”（clean room方式Linux 开源驱动实现
### 特

 - 通过调谐器或 S-Video/复合接口及音频线路输入，对广播电视（及声音）进行硬件 mpeg2 采集 - 在存在硬件支持的情况下，FM 收音进行硬件 mpeg2 采集 - 支持带立体声NTSC、PAL、SECAM - 支持 SAP 与双语传输 - 支持原始 VBI（隐藏字幕与图文电视） - 支持切片VBI（隐藏字幕与图文电视），并能够将其插入采集到MPEG 流中 - 支持原始 YUV PCM 输入
### PVR-350（基CX23415）的附加特

 - 提供硬件 mpeg2 回放 - 提供完善OSD（屏幕显示：即在视频信号上叠加图形） - 提供一个帧缓冲（允X 应用程序显示在视频设备上） - 支持原始 YUV 输出
重要提示：遇到问题时请先阅读此页面：
	https://help.ubuntu.com/community/Install_IVTV_Troubleshooting

### 另请参阅


https://linuxtv.org

### IRC


irc://irc.freenode.net/#v4l

----------------------------------------------------------

### 设备


目前最多允12 ivtv 板卡
没有视频输出能力（即PVR350 卡）的卡缺少 vbi8、vbi16、video16 video48
设备。它们也不支持用OSD 的帧缓冲设备 /dev/fbx
radio0 设备可能存在也可能不存在，取决于该卡是否带有收音调谐器
以下是基础 v4l 设备列表

	crw-rw----    1 root     video     81,   0 Jun 19 22:22 /dev/video0
	crw-rw----    1 root     video     81,  16 Jun 19 22:22 /dev/video16
	crw-rw----    1 root     video     81,  24 Jun 19 22:22 /dev/video24
	crw-rw----    1 root     video     81,  32 Jun 19 22:22 /dev/video32
	crw-rw----    1 root     video     81,  48 Jun 19 22:22 /dev/video48
	crw-rw----    1 root     video     81,  64 Jun 19 22:22 /dev/radio0
	crw-rw----    1 root     video     81, 224 Jun 19 22:22 /dev/vbi0
	crw-rw----    1 root     video     81, 228 Jun 19 22:22 /dev/vbi8
	crw-rw----    1 root     video     81, 232 Jun 19 22:22 /dev/vbi16

### 基础设备


每多一块卡，编号就加一。例如，/dev/video0 被列为“基础”编码采集设备，于是我们有：

- /dev/video0  是第一块卡（卡 0）的编码采集设备
- /dev/video1  是第二块卡（1）的编码采集设备
- /dev/video2  是第三块卡（2）的编码采集设备

注意，如果第一块卡没有某项特性（例如没有解码器，因而没video16），第二块卡
仍将使用 video17。简单的规则是“将卡号加到基础设备号上”。如果你还有其它采集（如 WinTV PCI）且被先检测到，那么需要让 ivtv 模块知道这一点，使其1（或 2或任何值）开始计数。否则设备号会变得混乱。ivtv `ivtv_first_minor` 模块参数
可用于此目的

- /dev/video0

  编码采集设备
  只读
  从此设备读取可获MPEG1/2 节目流  示例
  .. code-block:: none

	cat /dev/video0 > my.mpg （需要按 ctrl-c 退出）


- /dev/video16

  解码器输出设
  只写。仅当存MPEG 解码器（CX23415）时才有
  发送到该设备的 mpeg2 流将显示在所选视频上，音频将出现在线路输音频输出上  它仅适用于支持视频输出的卡。示例：

  .. code-block:: none

	cat my.mpg >/dev/video16


- /dev/video24

  原始音频采集设备
  只读

  来自当前所选调谐器或音频线路输入的原始音频 PCM 立体声流。从此设备读取将得到
  原始（带符号 16 位小端8000 Hz、立体声 pcm）采集数据。此设备仅采集音频  未来应由一ALSA 设备取代。注意，没有对应的原始音频输出设备，解码器固件不
  支持这一点

- /dev/video32

  原始视频采集设备

  只读

  来自当前视频输入的原YUV 视频输出。YUV 格式16x16 线性平NV12 格式
  （V4L2_PIX_FMT_NV12_16L16）
  注意 YUV PCM 流未同步，因此用途有限

- /dev/video48

  原始视频显示设备

  只写。仅当存MPEG 解码器（CX23415）时才有
  向卡的解码器写入一YUV 流

- /dev/radio0

  收音调谐器设
  不可读也不可写
  用于启用收音调谐器并调谐到某一频率。你无法用此设备读取或写入音频流。一旦用
  此设备调谐收音机，可使用 /dev/video24 读取原始 pcm 流，或使/dev/video0
  获取带有黑屏视频mpeg2 流

- /dev/vbi0

  “垂直消隐间隔”（图文电视、隐藏字幕、WSS 等）采集设备

  只读

  采集在垂直消隐间隔期间发送的原始（或切片式）视频数据。这些数据用于编码图  电视、隐藏字幕、VPS、宽signalling、电子节目指南信息以及其它服务

- /dev/vbi8

  处理后的 vbi 反馈设备

  只读。仅当存MPEG 解码器（CX23415）时才有
  嵌入MPEG 流中的切片式 VBI 数据会在此设备上重现。因此在 /dev/video16   回放录制内容时，你可以从 /dev/vbi8 读取嵌入VBI 数据

- /dev/vbi16

  vbi“显示”设
  只写。仅当存MPEG 解码器（CX23415）时才有
  可用于向视频输出连接器发送切片式 VBI 数据