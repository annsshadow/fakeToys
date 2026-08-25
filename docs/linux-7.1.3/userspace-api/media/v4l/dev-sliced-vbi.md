


######## 切片 VBI 数据接口


VBI Vertical Blanking Interval（场消隐间隔）的缩写，即模拟视频信号各行序列之间的一个间隙VBI 期间不传输图像信息，这为阴极射线管电视的电子束返回屏幕顶部留出了一些时间
切片 VBI 设备使用硬件解调VBI 中传输的数据。V4L2 驱动**不应**通过软件来完成此工作，另请参原始 VBI 接口 <raw-vbi>。数据以固定大小的短数据包形式传递，每个数据包覆盖一行扫描线每视频帧的数据包数量是可变的
切片 VBI 捕获和输出设备通过和原VBI 设备相同的字符特殊文件进行访问。当驱动同时支持这两种接口时`/dev/vbi` 设备的默认功能是**原始** VBI 捕获或输出，切片 VBI 功能仅在调用如下定义VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 后才可用。同样，`/dev/video` 设备也可能支持切VBI API但此处的默认功能是视频捕获或输出。如果驱动支持，必须使用不同的文件描述符来同时传递原始和切片 VBI 数据
## 查询能力


支持切片 VBI 捕获或输API 的设备分别设`v4l2_capability` 结构`capabilities` 字段中的 `V4L2_CAP_SLICED_VBI_CAPTURE` `V4L2_CAP_SLICED_VBI_OUTPUT` 标志，该结构体由
VIDIOC_QUERYCAP ioctl 返回。必须至少支持一read/write 或流I/O 方法 <io>。切VBI 设备可能带有调谐器或调制器
## 辅助功能


切片 VBI 设备应当支持视频输入或输<video> 以及调谐器或调制<tuner> ioctl
（如果它们具备这些能力），并且可能支持控ioctl。视频标<standard> ioctl 提供编程切片 VBI 设备所需的关键信息，因此必须支持

## 切片 VBI 格式协商


要了解硬件支持哪些数据服务，应用程序可以调用
VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl所有实现了切片 VBI 接口的驱动都必须支持ioctl。当硬件每帧能够捕获或输出的 VBI 行数或其能够在给定行上识别的服务数量受到限制时，结果可能VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 的结果不同。例如在 PAL 的第 16 行上硬件可能能够查找 VPS 或图文电视（Teletext）信号，但不能同时查找两者
要确定当前选择的服务，应用程序`v4l2_format` 结构体的 `type` 字段设置`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`，然VIDIOC_G_FMT <VIDIOC_G_FMT>
ioctl 会填`fmt.sliced` 成员，即一`v4l2_sliced_vbi_format` 结构体
应用程序可以通过初始化或修改 `fmt.sliced` 成员，并调用指向
`v4l2_format` 结构体的 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 来请求不同的参数
切片 VBI API 比原VBI API 更复杂，因为必须告诉硬件在每一行扫描线上期望哪VBI 服务。并非所服务都能被硬件在所有行上支持（对于 VBI 输出尤其如此，其中图文电视通常不受支持，而其他服务只能插入到
特定的某一行）。然而在许多情况下，只需`service_set` 字段设置为所需的服务，并让驱动根据
硬件能力来填`service_lines` 数组就足够了。只有在需要更精确的控制时，程序员才应显式设置
`service_lines` 数组
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 会根据硬件能力修改参数。当驱动在此刻分配资源时，如所需资源暂时不可用，它可能返`EBUSY` 错误码。其他可能返`EBUSY` 的资源分配点包括
VIDIOC_STREAMON ioctl 以及第一`read()`、`write()` `select()` 调用

### struct v4l2_sliced_vbi_format



    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 3 2 2 2

    - - __u16
      - `service_set`
      - `2`

	如果在使VIDIOC_S_FMT <VIDIOC_G_FMT> 	VIDIOC_TRY_FMT <VIDIOC_G_FMT> 传递时 `service_set` 非零	`service_lines` 数组将由驱动根据此字段中指定的服务进行填充	例如，如`service_set` 被初始化`V4L2_SLICED_TELETEXT_B | V4L2_SLICED_WSS_625`	cx25840 视频解码器的驱动会将两个[#f1]_ 的第 7-22 行设置为
	`V4L2_SLICED_TELETEXT_B`，并将第一个场的第 23 行设置为
	`V4L2_SLICED_WSS_625`。如`service_set` 被设置为零，则将改用
	`service_lines` 的值
	返回时，驱动将此字段设置为返回的 `service_lines` 数组中所有元素的并集	如果硬件无法同时处理更多服务，它可能包含比请求更少的服务，也许只有一个	如果所请求的服务均不受硬件支持，它可能为空（零）    - - __u16
      - `service_lines`\ [^2^][^24^]
      - `2`

	应用程序用驱动应当在相应扫描行上查找或插入的数据服务集合来初始化此数组	受硬件能力限制，驱动会返回所请求的集合、一个子集（可能只是一个服务）或一个空集	当硬件无法在同一行上处理多个服务时，驱动应当选择其中一个。无法假定驱动会选择哪个服务
	数据服务vbi-services2 中定义。数组索引映射到 ITU-R 行号\ [#f2]_，如下所示：
#     * -

      - Element
      - 525 line systems
      - 625 line systems
#     * -

      - `service_lines`\ [^0^][^1^]
      - 1
      - 1
#     * -

      - `service_lines`\ [^0^][^23^]
      - 23
      - 23
#     * -

      - `service_lines`\ [^1^][^1^]
      - 264
      - 314
#     * -

      - `service_lines`\ [^1^][^23^]
      - 286
      - 336
#     * -

      - `2` 驱动必须`service_lines` [^0^][^0^] 	`service_lines`\ [^1^][^0^] 设为零	`V4L2_VBI_ITU_525_F1_START`、`V4L2_VBI_ITU_525_F2_START`	`V4L2_VBI_ITU_625_F1_START` `V4L2_VBI_ITU_625_F2_START`
	的定义分别给出了每种 525 625 行格式各个场的起始行号，以方便使用。不要忘	ITU 行号1 开始，而不0    - - __u32
      - `io_size`
      - `2` 一`read()` `write()` 调用所传递的最大字节数	以及 VIDIOC_QBUF 	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 的缓冲区大小（以字节为单位）	驱动将此字段设置`v4l2_sliced_vbi_data` 结构体大小乘以返回的
	`service_lines` 数组中非零元素的数量（即可能携带数据的行数）    - - __u32
      - `reserved`\ [^2^]
      - `2` 此数组为未来扩展而保留
	应用程序和驱动必须将其设置为零

    \endgroup


### 切片 VBI 服务



    \footnotesize


    :header-rows:  1
    :stub-columns: 0
    :widths:       2 1 1 2 2

    - - Symbol
      - Value
      - Reference
      - Lines, usually
      - Payload
    - - `V4L2_SLICED_TELETEXT_B` (Teletext System B)
      - 0x0001
      - ets300706,

	itu653
      - PAL/SECAM 7-22 行，320-335（第二个7-22      - 45 字节图文电视数据包中的最42 个字节，即不含时钟导入和成帧码，
	最低有效位先传输    - - `V4L2_SLICED_VPS`
      - 0x0400
      - ets300231
      - PAL 16       - 根据 ETS 300 231 9，从3 字节到第 15 字节，最低有效位先传输    - - `V4L2_SLICED_CAPTION_525`
      - 0x1000
      - cea608
      - NTSC 21 行，284（第二个21      - 按传输顺序的两个字节，包含奇偶校验位，最低有效位先传输    - - `V4L2_SLICED_WSS_625`
      - 0x4000
      - itu1119,

	en300294
      - PAL/SECAM 23       - 请参见下面的 v4l2-sliced-wss-625-payload    - - `V4L2_SLICED_VBI_525`
      - 0x1000
      - `2` 适用525 行系统的服务集合    - - `V4L2_SLICED_VBI_625`
      - 0x4401
      - `2` 适用625 行系统的服务集合

    \normalsize

驱动在应用程序尝试在没有事先进行格式协商的情况下读取或写入数据、在切换视频标准之后（这可能使协商的
VBI 参数失效）以及在切换视频输入之后（这可能作为副作用改变视频标准）时，可能返回 `EINVAL` 错误码VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 在应用程序尝试在 I/O 进行期间（在
VIDIOC_STREAMON VIDIOC_STREAMOFF <VIDIOC_STREAMON> 调用之间，以及第一`read()` `write()` 调用之后）更改格式时，可能返`EBUSY` 错误码

#### V4L2_SLICED_WSS_625 负载


`V4L2_SLICED_WSS_625` 的负载为
           +-----+------------------+-----------------------+
	   |Byte |        0         |           1           |
           +-----+--------+---------+-----------+-----------+
	   |     | msb    | lsb     | msb       | lsb       |
           |     +-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+
	   | Bit |7|6|5|4 | 3|2|1|0 | x|x|13|12 | 11|10|9|8 |
           +-----+-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+

## 读取和写入切VBI 数据


一`read()` `write()` 调用必须传递属于一个视频帧的所有数据。即一`v4l2_sliced_vbi_data` 结构体数组，包含一个或多个元素，且总大小不超过 `io_size` 字节同样，在流式 I/O 模式下，一`io_size` 字节的缓冲区必须包含一帧视频的数据未使用的 `v4l2_sliced_vbi_data` 元素`id` 必须为零

### struct v4l2_sliced_vbi_data



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - __u32
      - `id`
      - 来自 vbi-services 的一个标志，标识此数据包中数据的类型。必须只设置一个位	当捕获数据包`id` 为零时，该数据包为空，其他字段的内容未定义。应用程序应当忽	空数据包。当用于输出的数据包`id` 为零时，`data` 字段的内容未定义，驱动必	不再在请求的 `field` `line` 上插入数据    - - __u32
      - `field`
      - 此数据被捕获自或将要被插入到的视频场编号。`0` 表示第一个场，`1` 表示第二个场    - - __u32
      - `line`
      - 此数据被捕获自或将要被插入到的场（相对于帧而言）行号。有效值请参见 vbi-525 	vbi-625。如果硬件无法可靠识别扫描行，切VBI 捕获设备可以将所有数据包的行号设置为
	`0`。场编号必须始终有效    - - __u32
      - `reserved`
      - 此字段为未来扩展而保留。应用程序和驱动必须将其设置为零    - - __u8
      - `data`\ [^48^]
      - 数据包负载。每种数据类型传递的内容和字节数vbi-services。此数组末尾填充字节	内容未定义，驱动和应用程序应当忽略它们
数据包始终按行号升序传递，没有重复的行号。当应用程序违反此规则时，`write()` 函数VIDIOC_QBUF ioctl 必须返回 `EINVAL` 错误码。当应用程序传递了不正确的场或行号，或者传递了
未与 VIDIOC_G_FMT <VIDIOC_G_FMT> VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 协商过的 `field`、`line` `id` 的组合时它们也必须返EINVAL 错误码。当行号未知时，驱动必须按传输顺序传递数据包。驱动可以在数据包数组的
任意位置插入 `id` 设为零的空数据包
为了确保同步并区别于丢帧，当捕获的帧不包含任何所请求的数据服务时，驱动必须传递一个或多个空数据包当应用程序未能及时传VBI 数据以进行输出时，驱动必须再次输出最后一VPS WSS 数据包，并禁隐藏字幕（Closed Caption）和图文电视数据的输出，或者输出被隐藏字幕和图文电视解码器忽略的数据
切片 VBI 设备可能支持 read/write <rw> 或流式（内存映射 <mmap> 用户指针 <userp>）I/O。后者提供了利用缓冲区时间戳来同步视频和 VBI 数据的可能性
## MPEG 流中的切VBI 数据


如果设备能够产生 MPEG 输出流，它可能能够提协商过的切片 VBI 服务 <sliced-vbi-format-negotiation>，作为嵌入在 MPEG 流中的数据用户或应用程序使V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
控制项来控制这种切片 VBI 数据的插入
如果驱动不提V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
控制项，或者只允许将该控制项设置为
V4L2_MPEG_STREAM_VBI_FMT_NONE <v4l2-mpeg-stream-vbi-fmt>则设备无法将切片 VBI 数据嵌入MPEG 流中
V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
控制项不会隐式地让设备驱动捕获或停止捕获切片 VBI 数据。该控制项仅指示MPEG 流中嵌入切片 VBI 数据
（如果应用程序已协商捕获某种切片 VBI 服务）
也可能出现设备只能将切片 VBI 数据嵌入某些类型MPEG 流中的情况：例如MPEG-2 PS 中可以，但在
MPEG-2 TS 中不行。在这种情况下，如果请求了切VBI 数据插入，切VBI 数据将被嵌入到受支持MPEG
流类型中，并在设备不支持切片 VBI 数据插入MPEG 流类型中被静默省略
以下小节规定了嵌入的切片 VBI 数据的格式
### MPEG 流嵌入的切片 VBI 数据格式：NONE


V4L2_MPEG_STREAM_VBI_FMT_NONE <v4l2-mpeg-stream-vbi-fmt>
嵌入切片 VBI 格式应被驱动解释为停止在 MPEG 流中嵌入切片 VBI 数据的控制项。设置此格式时，设备或驱都不应在 MPEG 流中插入“空的”嵌入切VBI 数据包。此格式未规定任MPEG 流数据结构
### MPEG 流嵌入的切片 VBI 数据格式：IVTV


当受支持时，V4L2_MPEG_STREAM_VBI_FMT_IVTV <v4l2-mpeg-stream-vbi-fmt>
嵌入切片 VBI 格式指示驱动MPEG 流中，于封装MPEG-2 **Program Pack**（程序包）中MPEG-2 *Private Stream 1 PES**（私有流 1 PES）数据包内，每帧嵌入最36 行切VBI 数据
**历史背景**：此格式规范源自 `ivtv` 驱动使用的一种自定义的、嵌入式的切VBI 数据格式该格式已在内核源码文`Documentation/userspace-api/media/drivers/cx2341x-uapi.rst` 中被非正式地规定。此格式负载最大大小以及其它方面，CX23415 MPEG 解码器在提取、解码和显示嵌入MPEG 流中的切VBI 数据
方面的能力和限制所决定
此格式的使用**并非** `ivtv` 驱动所**独占**，也**并非** CX2341x 设备所独占，因为将切片 VBI 数据插入MPEG 流中是由驱动软件实现的。至`cx18` 驱动也以这种格式提供了向 MPEG-2 PS 中插入切VBI 数据
以下定义规定了当设置V4L2_MPEG_STREAM_VBI_FMT_IVTV <v4l2-mpeg-stream-vbi-fmt>
时，包含切片 VBI 数据MPEG-2 *Private Stream 1 PES* 数据包的负载（此处不详述 MPEG-2 **Private Stream 1 PES** 数据包头和封装的 MPEG-2 **Program Pack** 包头有关这些数据包头的详细信息，请参MPEG-2 规范。）

包含切片 VBI 数据MPEG-2 **Private Stream 1 PES** 数据包的负载`v4l2_mpeg_vbi_fmt_ivtv` 结构体规定。负载长度是可变的，取决于视频帧中存在的切片 VBI 数据的实际行数负载末尾可以用未指定的填充字节进行填充，以使负载末尾对齐4 字节边界。负载绝不应超过 1552 字节
 个场，每个场 18 行，每行 43 字节数据，外加一4 字节的魔数）

### struct v4l2_mpeg_vbi_fmt_ivtv



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `magic`\ [^4^]
      - 来自 v4l2-mpeg-vbi-fmt-ivtv-magic 的一个“魔数”常量，用于表明这是一个有效的
	切片 VBI 数据负载，并指示匿名联合的哪个成`itv0` `ITV0` 用于负载数据    - - union {
      - (anonymous)
    - - struct `v4l2_mpeg_vbi_itv0`
      - `itv0`
      - 切片 VBI 数据负载的主要形式，包含 1 35 行切VBI 数据。这种形式的负载中提	了行掩码，指示提供了哪些 VBI 行    - - struct v4l2_mpeg_vbi_ITV0 <v4l2-mpeg-vbi-itv0-1>
      - `ITV0`
      - 当存36 行切VBI 数据时使用的切片 VBI 数据负载的另一种形式。这种形式的负载中不提供
	行掩码；所有有效的行掩码位都被隐式设置    - - }
      -


### struct v4l2_mpeg_vbi_fmt_ivtv magic 字段的魔数常


    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - Defined Symbol
      - Value
      - Description
    - - `V4L2_MPEG_VBI_IVTV_MAGIC0`
      - "itv0"
      - 表明 `v4l2_mpeg_vbi_fmt_ivtv` 结构体中联合`itv0` 成员
	有效    - - `V4L2_MPEG_VBI_IVTV_MAGIC1`
      - "ITV0"
      - 表明 `v4l2_mpeg_vbi_fmt_ivtv` 结构体中联合`ITV0` 成员
	有效，并且存36 行切VBI 数据


### structs v4l2_mpeg_vbi_itv0 鍜?v4l2_mpeg_vbi_ITV0



   \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __le32
      - `linemask`\ [^2^]
      - 指示存在VBI 服务行的位掩码。这`linemask` 值在 MPEG 流中以小端字节序存储	下面给出了一`linemask` 位位置及其对应的 VBI 行号和视频场。b\ `0` 表示
	`linemask` 值的最低有效位

```

	    linemask[0] b0:     line  6  first field
	    linemask[0] b17:    line 23  first field
	    linemask[0] b18:    line  6  second field
	    linemask[0] b31:    line 19  second field
	    linemask[1] b0:     line 20  second field
	    linemask[1] b3:     line 23  second field
	    linemask[1] b4-b31: unused and set to 0
    * - struct
	:c:type:`v4l2_mpeg_vbi_itv0_line`
      - ``line``\ [35]
      - 这是一个可变长度数组，保存 1 35 行切VBI 数据。存在的切片 VBI 数据行对应于
	``linemask`` 数组中设置的位，``linemask``\ [0] b\ :sub:`0` 开始，一直到
	``linemask``\ [0] b\ :sub:`31`，再``linemask``\ [1] b\ :sub:`0` 开始，
	一直到 ``linemask``\ [1] b\ :sub:`3`。``line``\ [0] 对应于在 ``linemask`` 数组
	中找到的第一个被设置的位，``line``\ [1] 对应于找到的第二个被设置的位，依此类推。如	没有设置 ``linemask`` 数组的位，则 ``line``\ [0] 可能包含一行应用程序应忽略	未指定数据
```

   \normalsize


### struct v4l2_mpeg_vbi_ITV0



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct
	`v4l2_mpeg_vbi_itv0_line`
      - `line`\ [^36^]
      - 一个固定长度为 36 行的切片 VBI 数据数组。`line`\ [^0^] `line`\ [^17^] 对应	第一个场的第 6 23 行。`line`\ [^18^] `line`\ [^35^] 对应于第二个场的6 23 行


### struct v4l2_mpeg_vbi_itv0_line



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `id`
      - 来自 ITV0-Line-Identifier-Constants 的一个行标识符值，指示此行上存储的切片
	VBI 数据的类型    - - __u8
      - `data`\ [^42^]
      - 该行的切VBI 数据


### struct v4l2_mpeg_vbi_itv0_line id 字段的行标识

    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - Defined Symbol
      - Value
      - Description
    - - `V4L2_MPEG_VBI_IVTV_TELETEXT_B`
      - 1
      - 有关行负载的描述，请参阅切片 VBI 服务 <vbi-services2>    - - `V4L2_MPEG_VBI_IVTV_CAPTION_525`
      - 4
      - 有关行负载的描述，请参阅切片 VBI 服务 <vbi-services2>    - - `V4L2_MPEG_VBI_IVTV_WSS_625`
      - 5
      - 有关行负载的描述，请参阅切片 VBI 服务 <vbi-services2>    - - `V4L2_MPEG_VBI_IVTV_VPS`
      - 7
      - 有关行负载的描述，请参阅切片 VBI 服务 <vbi-services2>

   根据 ETS 300 706 <ets300706>，第一个场的第 6-22 行和第二个场的第 5-22 行可能携带图文电视数据
   另请参阅 vbi-525 vbi-625