

######## 保留格式标识符


这些格式未被本规范定义，仅列出以供参考并避免命名冲突。如果你想注册自己的格式，
请发送电子邮件到 linux-media 邮件列表
`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__
以将其纳入 `videodev2.h` 文件。如果你想与其他开发者共享你的格式，请添加一个指向
你文档的链接，并发送一份副本到 linux-media 邮件列表以便纳入本节。如果你认为你的
格式应该列在标准格式一节中，请在 linux-media 邮件列表上提出提案。




    \small


    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - Identifier
      - Code
      - Details
    - .. _V4L2-PIX-FMT-DV:

      - `V4L2_PIX_FMT_DV`
      - 'dvsd'
      - 未知
    - .. _V4L2-PIX-FMT-ET61X251:

      - `V4L2_PIX_FMT_ET61X251`
      - 'E625'
      - ET61X251 驱动的压缩格式。
    - .. _V4L2-PIX-FMT-HI240:

      - `V4L2_PIX_FMT_HI240`
      - 'HI24'
      - BTTV 驱动使用的 8 位 RGB 格式。
    - .. _V4L2-PIX-FMT-CPIA1:

      - `V4L2_PIX_FMT_CPIA1`
      - 'CPIA'
      - gspca cpia1 驱动使用的 YUV 格式。
    - .. _V4L2-PIX-FMT-JPGL:

      - `V4L2_PIX_FMT_JPGL`
      - 'JPGL'
      - JPEG-Light 格式（Pegasus 无损 JPEG），用于 Divio 网络摄像头 NW
	80x。
    - .. _V4L2-PIX-FMT-SPCA501:

      - `V4L2_PIX_FMT_SPCA501`
      - 'S501'
      - gspca 驱动使用的逐行 YUYV。
    - .. _V4L2-PIX-FMT-SPCA505:

      - `V4L2_PIX_FMT_SPCA505`
      - 'S505'
      - gspca 驱动使用的逐行 YYUV。
    - .. _V4L2-PIX-FMT-SPCA508:

      - `V4L2_PIX_FMT_SPCA508`
      - 'S508'
      - gspca 驱动使用的逐行 YUVY。
    - .. _V4L2-PIX-FMT-SPCA561:

      - `V4L2_PIX_FMT_SPCA561`
      - 'S561'
      - gspca 驱动使用的压缩 GBRG Bayer 格式。
    - .. _V4L2-PIX-FMT-PAC207:

      - `V4L2_PIX_FMT_PAC207`
      - 'P207'
      - gspca 驱动使用的压缩 BGGR Bayer 格式。
    - .. _V4L2-PIX-FMT-MR97310A:

      - `V4L2_PIX_FMT_MR97310A`
      - 'M310'
      - gspca 驱动使用的压缩 BGGR Bayer 格式。
    - .. _V4L2-PIX-FMT-JL2005BCD:

      - `V4L2_PIX_FMT_JL2005BCD`
      - 'JL20'
      - gspca 驱动使用的 JPEG 压缩 RGGB Bayer 格式。
    - .. _V4L2-PIX-FMT-OV511:

      - `V4L2_PIX_FMT_OV511`
      - 'O511'
      - gspca 驱动使用的 OV511 JPEG 格式。
    - .. _V4L2-PIX-FMT-OV518:

      - `V4L2_PIX_FMT_OV518`
      - 'O518'
      - gspca 驱动使用的 OV518 JPEG 格式。
    - .. _V4L2-PIX-FMT-PJPG:

      - `V4L2_PIX_FMT_PJPG`
      - 'PJPG'
      - gspca 驱动使用的 Pixart 73xx JPEG 格式。
    - .. _V4L2-PIX-FMT-SE401:

      - `V4L2_PIX_FMT_SE401`
      - 'S401'
      - gspca se401 驱动使用的压缩 RGB 格式
    - .. _V4L2-PIX-FMT-SQ905C:

      - `V4L2_PIX_FMT_SQ905C`
      - '905C'
      - gspca 驱动使用的压缩 RGGB bayer 格式。
    - .. _V4L2-PIX-FMT-MJPEG:

      - `V4L2_PIX_FMT_MJPEG`
      - 'MJPG'
      - Zoran 驱动使用的压缩格式
    - .. _V4L2-PIX-FMT-PWC1:

      - `V4L2_PIX_FMT_PWC1`
      - 'PWC1'
      - PWC 驱动的压缩格式。
    - .. _V4L2-PIX-FMT-PWC2:

      - `V4L2_PIX_FMT_PWC2`
      - 'PWC2'
      - PWC 驱动的压缩格式。
    - .. _V4L2-PIX-FMT-SN9C10X:

      - `V4L2_PIX_FMT_SN9C10X`
      - 'S910'
      - SN9C102 驱动的压缩格式。
    - .. _V4L2-PIX-FMT-SN9C20X-I420:

      - `V4L2_PIX_FMT_SN9C20X_I420`
      - 'S920'
      - gspca sn9c20x 驱动的 YUV 4:2:0 格式。
    - .. _V4L2-PIX-FMT-SN9C2028:

      - `V4L2_PIX_FMT_SN9C2028`
      - 'SONX'
      - gspca sn9c2028 驱动的压缩 GBRG bayer 格式。
    - .. _V4L2-PIX-FMT-STV0680:

      - `V4L2_PIX_FMT_STV0680`
      - 'S680'
      - gspca stv0680 驱动的 Bayer 格式。
    - .. _V4L2-PIX-FMT-WNVA:

      - `V4L2_PIX_FMT_WNVA`
      - 'WNVA'
      - 由 Winnov Videum 驱动使用，
	`http://www.thedirks.org/winnov/ <http://www.thedirks.org/winnov/>`__
    - .. _V4L2-PIX-FMT-TM6000:

      - `V4L2_PIX_FMT_TM6000`
      - 'TM60'
      - 由 Trident tm6000 使用
    - .. _V4L2-PIX-FMT-CIT-YYVYUY:

      - `V4L2_PIX_FMT_CIT_YYVYUY`
      - 'CITV'
      - 由 xirlink CIT 使用，见于 IBM 网络摄像头。

	先使用一行 Y，再使用一行 VYUY
    - .. _V4L2-PIX-FMT-KONICA420:

      - `V4L2_PIX_FMT_KONICA420`
      - 'KONI'
      - 由 Konica 网络摄像头使用。

	以 256 像素为块的 YUV420 平面格式。
    - .. _V4L2-PIX-FMT-YYUV:

      - `V4L2_PIX_FMT_YYUV`
      - 'YYUV'
      - 未知
    - .. _V4L2-PIX-FMT-Y4:

      - `V4L2_PIX_FMT_Y4`
      - 'Y04 '
      - 旧的 4 位灰度格式。每个字节仅使用最高的 4 位，其余位被置为 0。
    - .. _V4L2-PIX-FMT-Y6:

      - `V4L2_PIX_FMT_Y6`
      - 'Y06 '
      - 旧的 6 位灰度格式。每个字节仅使用最高的 6 位，其余位被置为 0。
    - .. _V4L2-PIX-FMT-S5C-UYVY-JPG:

      - `V4L2_PIX_FMT_S5C_UYVY_JPG`
      - 'S5CI'
      - Samsung S5C73MX 摄像头使用的双平面格式。第一个平面包含交错排列的
	JPEG 和 UYVY 图像数据，其后跟随着以指向 UYVY 数据块的偏移数组形式
	存在的元数据。实际指针数组紧跟在交错的 JPEG/UYVY 数据之后，该数组
	中的条目数等于 UYVY 图像的高度。每个条目是一个大端序的 4 字节无符号
	整数，它是指向 UYVY 图像单行像素的偏移。第一个平面可以以 JPEG 或
	UYVY 数据块开头。单个 UYVY 块的大小等于 UYVY 图像的宽度乘以 2。JPEG
	块的大小取决于图像，并且可能随每一行变化。

	第二个平面位于 4084 字节偏移处，包含一个指向第一个平面中指针数组的
	4 字节偏移。该偏移之后是一个指示指针数组大小的 4 字节值。第二个平面
	中的所有数字也均按大端序排列。第二个平面中的剩余数据未定义。第二个
	平面中的信息可以方便地找到指针数组的位置，该位置对于每一帧可能不同。
	对于给定的 UYVY 图像高度，指针数组的大小是恒定的。

	为了提取 UYVY 和 JPEG 帧，应用程序最初可以将数据指针设置为第一个平面的
	起始位置，然后加上指针表第一个条目的偏移。这样的指针指向 UYVY 图像
	像素行的起始。整行 UYVY 可以被复制到一个独立的缓冲区。这些步骤应当对
	每一行（即指针数组中的条目数）重复进行。位于 UYVY 行之间的任何内容都是
	JPEG 数据，应当被拼接起来形成 JPEG 流。
    - .. _V4L2-PIX-FMT-MT21C:

      - `V4L2_PIX_FMT_MT21C`
      - 'MT21'
      - 联发科 MT8173、MT8192、MT8195 等使用的压缩双平面 YVU420 格式。该压缩
	是无损的。该格式在对齐和分块方面与 `V4L2_PIX_FMT_MM21` 相似。它仍然
	是一个不透明的中间格式，必须使用 MDP 硬件将 `V4L2_PIX_FMT_MT21C` 转换为
	`V4L2_PIX_FMT_NV12M`、`V4L2_PIX_FMT_YUV420M` 或 `V4L2_PIX_FMT_YVU420`。
    - .. _V4L2-PIX-FMT-QC08C:

      - `V4L2_PIX_FMT_QC08C`
      - 'QC08C'
      - 高通平台使用的压缩宏块瓦片（Macro-tile）8 位 YUV420 格式。它是一个
	不透明的中间格式。所使用的压缩是无损的，并被各种多媒体硬件模块使用，
	如 GPU、显示控制器、ISP 和视频加速器。对于逐行视频它包含四个平面，对于
	隔行视频包含八个平面。
    - .. _V4L2-PIX-FMT-QC10C:

      - `V4L2_PIX_FMT_QC10C`
      - 'QC10C'
      - 高通平台使用的压缩宏块瓦片（Macro-tile）10 位 YUV420 格式。它是一个
	不透明的中间格式。所使用的压缩是无损的，并被各种多媒体硬件模块使用，
	如 GPU、显示控制器、ISP 和视频加速器。对于逐行视频它包含四个平面。
    - .. _V4L2-PIX-FMT-AJPG:

      - `V4L2_PIX_FMT_AJPG`
      - 'AJPG'
      - Aspeed 平台上的 aspeed-video 驱动使用的 ASPEED JPEG 格式，通常用于
	远程 KVM。在每次帧压缩时，我会将新帧与前一帧进行比较，以决定哪些宏块的
	数据发生了变化，并且只有发生变化的宏块才会被压缩。

        该实现基于 AST2600 A3 数据手册，修订版 0.9，该手册未公开提供。或者你也可以
	参考 SDK_User_Guide 中的 Video stream data format – ASPEED mode compression，
	该指南可在
	`github <https://github.com/AspeedTech-BMC/openbmc/releases/>`__ 上获取。

        解码器的实现可以在此处找到，
        `aspeed_codec <https://github.com/AspeedTech-BMC/aspeed_codec/>`__
    - .. _V4L2-PIX-FMT-HEXTILE:

      - `V4L2_PIX_FMT_HEXTILE`
      - 'HXTL'
      - Nuvoton NPCM 视频驱动使用的压缩格式。该格式定义于远程帧缓冲协议
        （RFC 6143，第 7.7.4 节 Hextile Encoding）中。

    \normalsize
