
## cx2341x 驱动


### cx2341x 芯片上的内存


本节描述 cx2341x 的内存映射，并记录部分寄存器空间。


	这些信息是通过搜索内存和寄存器得到的，可能不正确、也肯定不完整，且仅是通过用如下命令搜索内存空间得出：

	.. code-block:: none

		ivtvctl -O min=0x02000000,max=0x020000ff

	因此请按原样看待，我一直在寻找更多内容，寄存器空间很大 :-)。

#### 内存映射


cx2341x 通过 PCI BAR0（基地址寄存器 0）把其整个 64M 内存空间暴露给 PCI 主机。这里的地址是相对于 BAR0 中所持地址的偏移。


	0x00000000-0x00ffffff Encoder memory space
	0x00000000-0x0003ffff Encode.rom
	???-???         MPEG buffer(s)
	???-???         Raw video capture buffer(s)
	???-???         Raw audio capture buffer(s)
	???-???         Display buffers (6 or 9)

	0x01000000-0x01ffffff Decoder memory space
	0x01000000-0x0103ffff Decode.rom
	???-???         MPEG buffers(s)
	0x0114b000-0x0115afff Audio.rom (deprecated?)

	0x02000000-0x0200ffff Register Space

#### 寄存器


寄存器占据从 BAR0 偏移 0x02000000 开始的 64k 空间。所有这些寄存器均为 32 位宽。


	DMA Registers 0x000-0xff:

	0x00 - Control:
		0=reset/cancel, 1=read, 2=write, 4=stop
	0x04 - DMA status:
		1=read busy, 2=write busy, 4=read error, 8=write error, 16=link list error
	0x08 - pci DMA pointer for read link list
	0x0c - pci DMA pointer for write link list
	0x10 - read/write DMA enable:
		1=read enable, 2=write enable
	0x14 - always 0xffffffff, if set any lower instability occurs, 0x00 crashes
	0x18 - ??
	0x1c - always 0x20 or 32, smaller values slow down DMA transactions
	0x20 - always value of 0x780a010a
	0x24-0x3c - usually just random values???
	0x40 - Interrupt status
	0x44 - Write a bit here and shows up in Interrupt status 0x40
	0x48 - Interrupt Mask
	0x4C - always value of 0xfffdffff,
		if changed to 0xffffffff DMA write interrupts break.
	0x50 - always 0xffffffff
	0x54 - always 0xffffffff (0x4c, 0x50, 0x54 seem like interrupt masks, are
		3 processors on chip, Java ones, VPU, SPU, APU, maybe these are the
		interrupt masks???).
	0x60-0x7C - random values
	0x80 - first write linked list reg, for Encoder Memory addr
	0x84 - first write linked list reg, for pci memory addr
	0x88 - first write linked list reg, for length of buffer in memory addr
		(|0x80000000 or this for last link)
	0x8c-0xdc - rest of write linked list reg, 8 sets of 3 total, DMA goes here
		from linked list addr in reg 0x0c, firmware must push through or
		something.
	0xe0 - first (and only) read linked list reg, for pci memory addr
	0xe4 - first (and only) read linked list reg, for Decoder memory addr
	0xe8 - first (and only) read linked list reg, for length of buffer
	0xec-0xff - Nothing seems to be in these registers, 0xec-f4 are 0x00000000.

Encoder 缓冲区的内存位置 0x700-0x7ff：

这些寄存器显示用于编码的各个缓冲区区域相关内存位置的偏移，需先左移 <<1。

- 0x07F8：编码器 SDRAM 刷新
- 0x07FC：编码器 SDRAM 预充电

Decoder 缓冲区的内存位置 0x800-0x8ff：

这些寄存器显示用于解码的各个缓冲区区域相关内存位置的偏移，需先左移 <<1。

- 0x08F8：解码器 SDRAM 刷新
- 0x08FC：解码器 SDRAM 预充电

其他内存位置：

- 0x2800：视频显示模块控制
- 0x2D00：AO（音频输出？）控制
- 0x2D24：已刷新字节数
- 0x7000：LSB I2C 写时钟位（取反）
- 0x7004：LSB I2C 写数据位（取反）
- 0x7008：LSB I2C 读时钟位
- 0x700c：LSB I2C 读数据位
- 0x9008：GPIO 获取输入状态
- 0x900c：GPIO 设置输出状态
- 0x9020：GPIO 方向（Bit7（GPIO 0..7）——0：输入，1：输出）
- 0x9050：SPU 控制
- 0x9054：复位硬件模块
- 0x9058：VPU 控制
- 0xA018：Bit6：中断挂起？
- 0xA064：APU 命令


#### 中断状态寄存器


中断状态寄存器 0x0040 与中断掩码 0x0048 中各比特的定义。如果在掩码中某比特被清零，则我们希望我们的 ISR 执行。

- bit 31 Encoder Start Capture
- bit 30 Encoder EOS
- bit 29 Encoder VBI capture
- bit 28 Encoder Video Input Module reset event
- bit 27 Encoder DMA complete
- bit 24 Decoder audio mode change detection event (through event notification)
- bit 22 Decoder data request
- bit 20 Decoder DMA complete
- bit 19 Decoder VBI re-insertion
- bit 18 Decoder DMA err (linked-list bad)

### 缺失的文档


- Encoder API post(?)
- Decoder API post(?)
- Decoder VTRACE event


### cx2341x 固件上传


本文描述如何把 cx2341x 固件上传到卡上。

#### 如何找到


有关如何获取固件的信息，请参见使用此芯片的各项目的网页。

存储在 Windows 驱动中的固件可如下检测：

- Each firmware image is 256k bytes.
- The 1st 32-bit word of the Encoder image is 0x0000da7
- The 1st 32-bit word of the Decoder image is 0x00003a7
- The 2nd 32-bit word of both images is 0xaa55bb66

#### 如何加载


- Issue the FWapi command to stop the encoder if it is running. Wait for the command to complete.
- Issue the FWapi command to stop the decoder if it is running. Wait for the command to complete.
- Issue the I2C command to the digitizer to stop emitting VSYNC events.
- Issue the FWapi command to halt the encoder's firmware.
- Sleep for 10ms.
- Issue the FWapi command to halt the decoder's firmware.
- Sleep for 10ms.
- Write 0x00000000 to register 0x2800 to stop the Video Display Module.
- Write 0x00000005 to register 0x2D00 to stop the AO (audio output?).
- Write 0x00000000 to register 0xA064 to ping? the APU.
- Write 0xFFFFFFFE to register 0x9058 to stop the VPU.
- Write 0xFFFFFFFF to register 0x9054 to reset the HW blocks.
- Write 0x00000001 to register 0x9050 to stop the SPU.
- Sleep for 10ms.
- Write 0x0000001A to register 0x07FC to init the Encoder SDRAM's pre-charge.
- Write 0x80000640 to register 0x07F8 to init the Encoder SDRAM's refresh to 1us.
- Write 0x0000001A to register 0x08FC to init the Decoder SDRAM's pre-charge.
- Write 0x80000640 to register 0x08F8 to init the Decoder SDRAM's refresh to 1us.
- Sleep for 512ms. (600ms is recommended)
- Transfer the encoder's firmware image to offset 0 in Encoder memory space.
- Transfer the decoder's firmware image to offset 0 in Decoder memory space.
- Use a read-modify-write operation to Clear bit 0 of register 0x9050 to re-enable the SPU.
- Sleep for 1 second.
- Use a read-modify-write operation to Clear bits 3 and 0 of register 0x9058 to re-enable the VPU.
- Sleep for 1 second.
- Issue status API commands to both firmware images to verify.


### 如何调用固件 API


首选的调用约定称为固件邮箱（firmware mailbox）。邮箱本质上是一个固定长度的数组，充当调用栈。

固件邮箱可以通过在编码器和解码器内存中搜索一个 16 字节的签名来定位。该签名会位于 256 字节边界上。

签名：


	0x78, 0x56, 0x34, 0x12, 0x12, 0x78, 0x56, 0x34,
	0x34, 0x12, 0x78, 0x56, 0x56, 0x34, 0x12, 0x78

固件实现了 20 个邮箱，每个 20 个 32 位字。前 10 个保留给 API 调用。后 10 个由固件用于事件通知。

  ====== =================
  索引   名称
  ====== =================
  0      标志
  1      命令
  2      返回值
  3      超时
  4-19   参数/结果
  ====== =================


标志在下面的表中定义。方向是站在固件的角度。

  ==== ========== ============================================
  位   方向       用途
  ==== ========== ============================================
  2    O          固件已处理该命令。
  1    I          驱动已完成参数设置。
  0    I          驱动正在使用此邮箱。
  ==== ========== ============================================

命令是一个 32 位枚举值。API 细节可在本章找到。

返回值是一个 32 位枚举值。目前仅定义了两个值：

- 0=success
- -1=command undefined.

共有 16 个参数/结果 32 位字段。驱动用调用所需的全部参数值填充这些字段。驱动再用调用返回的結果值覆盖这些字段。

超时值保护卡免受挂起的驱动线程影响。如果驱动没有在指定的超时内处理完该调用，固件将复位该邮箱。

要进行一次 API 调用，驱动遍历每个邮箱，寻找第一个可用的（bit 0 已被清零）。驱动置位该比特，填入命令枚举值、超时值以及任何所需参数。驱动随后置位参数就绪比特（bit 1）。固件扫描邮箱以寻找待处理命令，处理它们，设置结果码，用该调用的返回值填充结果值数组，并置位调用完成比特（bit 2）。一旦 bit 2 被置位，驱动应取回结果并清除所有标志。如果驱动没有在超时寄存器设定的时间内完成此任务，固件将复位该邮箱。

事件通知由固件发送给主机。主机通过一个 API 调用告诉固件它感兴趣的事件。该调用告诉固件使用哪个通知邮箱。固件通过一个中断向主机发信号。仅使用 16 个结果字段，标志、命令、返回值和超时字不被使用。


### OSD 固件 API 描述



#### CX2341X_OSD_GET_FRAMEBUFFER


Enum: 65/0x41

##### 描述


返回连续 OSD 内存的基址和长度。

##### 结果[0]


OSD 基址

##### 结果[1]


OSD 长度



#### CX2341X_OSD_GET_PIXEL_FORMAT


Enum: 66/0x42

##### 描述


查询 OSD 格式

##### 结果[0]


0=8bit index
1=16bit RGB 5:6:5
2=16bit ARGB 1:5:5:5
3=16bit ARGB 1:4:4:4
4=32bit ARGB 8:8:8:8



#### CX2341X_OSD_SET_PIXEL_FORMAT


Enum: 67/0x43

##### 描述


设置像素格式

##### 参数[0]


- 0=8bit index
- 1=16bit RGB 5:6:5
- 2=16bit ARGB 1:5:5:5
- 3=16bit ARGB 1:4:4:4
- 4=32bit ARGB 8:8:8:8



#### CX2341X_OSD_GET_STATE


Enum: 68/0x44

##### 描述


查询 OSD 状态

##### 结果[0]


- Bit  0   0=off, 1=on
- Bits 1:2 alpha control
- Bits 3:5 pixel format



#### CX2341X_OSD_SET_STATE


Enum: 69/0x45

##### 描述


OSD 开关

##### 参数[0]


0=off, 1=on



#### CX2341X_OSD_GET_OSD_COORDS


Enum: 70/0x46

##### 描述


取回与视频混合的 OSD 区域坐标

##### 结果[0]


OSD 缓冲区地址

##### 结果[1]


步长（像素）

##### 结果[2]


OSD 缓冲区中的行数

##### 结果[3]


缓冲区中的水平偏移

##### 结果[4]


缓冲区中的垂直偏移



#### CX2341X_OSD_SET_OSD_COORDS


Enum: 71/0x47

##### 描述


设置要与视频混合的 OSD 区域坐标

##### 参数[0]


缓冲区地址

##### 参数[1]


缓冲区步长（像素）

##### 参数[2]


缓冲区中的行数

##### 参数[3]


水平偏移

##### 参数[4]


垂直偏移



#### CX2341X_OSD_GET_SCREEN_COORDS


Enum: 72/0x48

##### 描述


取回 OSD 屏幕区域坐标

##### 结果[0]


左上角水平偏移

##### 结果[1]


左上角垂直偏移

##### 结果[2]


右下角水平偏移

##### 结果[3]


右下角垂直偏移



#### CX2341X_OSD_SET_SCREEN_COORDS


Enum: 73/0x49

##### 描述


设置要与视频混合的屏幕区域坐标

##### 参数[0]


左上角水平偏移

##### 参数[1]


左上角垂直偏移

##### 参数[2]


左下角水平偏移

##### 参数[3]


左下角垂直偏移



#### CX2341X_OSD_GET_GLOBAL_ALPHA


Enum: 74/0x4A

##### 描述


取回 OSD 全局 alpha

##### 结果[0]


全局 alpha：0=off, 1=on

##### 结果[1]


bits 0:7 global alpha



#### CX2341X_OSD_SET_GLOBAL_ALPHA


Enum: 75/0x4B

##### 描述


更新全局 alpha

##### 参数[0]


全局 alpha：0=off, 1=on

##### 参数[1]


全局 alpha（8 位）

##### 参数[2]


局部 alpha：0=on, 1=off



#### CX2341X_OSD_SET_BLEND_COORDS


Enum: 78/0x4C

##### 描述


在显示缓冲区内移动混合区域的起点

##### 参数[0]


缓冲区中的水平偏移

##### 参数[1]


缓冲区中的垂直偏移



#### CX2341X_OSD_GET_FLICKER_STATE


Enum: 79/0x4F

##### 描述


取回闪烁抑制模块状态

##### 结果[0]


闪烁状态：0=off, 1=on



#### CX2341X_OSD_SET_FLICKER_STATE


Enum: 80/0x50

##### 描述


设置闪烁抑制模块状态

##### 参数[0]


状态：0=off, 1=on



#### CX2341X_OSD_BLT_COPY


Enum: 82/0x52

##### 描述


BLT 复制

##### 参数[0]



	'0000'  zero
	'0001' ~destination AND ~source
	'0010' ~destination AND  source
	'0011' ~destination
	'0100'  destination AND ~source
	'0101'                  ~source
	'0110'  destination XOR  source
	'0111' ~destination OR  ~source
	'1000' ~destination AND ~source
	'1001'  destination XNOR source
	'1010'                   source
	'1011' ~destination OR   source
	'1100'  destination
	'1101'  destination OR  ~source
	'1110'  destination OR   source
	'1111'  one


##### 参数[1]


结果 alpha 混合

- '01' source_alpha
- '10' destination_alpha
- '11' source_alpha*destination_alpha+1
  (zero if both source and destination alpha are zero)

##### 参数[2]



	'00' output_pixel = source_pixel

	'01' if source_alpha=0:
		 output_pixel = destination_pixel
	     if 256 > source_alpha > 1:
		 output_pixel = ((source_alpha + 1)*source_pixel +
				 (255 - source_alpha)*destination_pixel)/256

	'10' if destination_alpha=0:
		 output_pixel = source_pixel
	      if 255 > destination_alpha > 0:
		 output_pixel = ((255 - destination_alpha)*source_pixel +
				 (destination_alpha + 1)*destination_pixel)/256

	'11' if source_alpha=0:
		 source_temp = 0
	     if source_alpha=255:
		 source_temp = source_pixel*256
	     if 255 > source_alpha > 0:
		 source_temp = source_pixel*(source_alpha + 1)
	     if destination_alpha=0:
		 destination_temp = 0
	     if destination_alpha=255:
		 destination_temp = destination_pixel*256
	     if 255 > destination_alpha > 0:
		 destination_temp = destination_pixel*(destination_alpha + 1)
	     output_pixel = (source_temp + destination_temp)/256

##### 参数[3]


宽度

##### 参数[4]


高度

##### 参数[5]


目标像素掩码

##### 参数[6]


目标矩形起始地址

##### 参数[7]


目标步长（dwords）

##### 参数[8]


源步长（dwords）

##### 参数[9]


源矩形起始地址



#### CX2341X_OSD_BLT_FILL


Enum: 83/0x53

##### 描述


BLT 填充颜色

##### 参数[0]


Same as Param[^0^] on API 0x52

##### 参数[1]


Same as Param[^1^] on API 0x52

##### 参数[2]


Same as Param[^2^] on API 0x52

##### 参数[3]


宽度

##### 参数[4]


高度

##### 参数[5]


目标像素掩码

##### 参数[6]


目标矩形起始地址

##### 参数[7]


目标步长（dwords）

##### 参数[8]


颜色填充值



#### CX2341X_OSD_BLT_TEXT


Enum: 84/0x54

##### 描述


用于 8 位 alpha 文本源的 BLT

##### 参数[0]


Same as Param[^0^] on API 0x52

##### 参数[1]


Same as Param[^1^] on API 0x52

##### 参数[2]


Same as Param[^2^] on API 0x52

##### 参数[3]


宽度

##### 参数[4]


高度

##### 参数[5]


目标像素掩码

##### 参数[6]


目标矩形起始地址

##### 参数[7]


目标步长（dwords）

##### 参数[8]


源步长（dwords）

##### 参数[9]


源矩形起始地址

##### 参数[10]


颜色填充值



#### CX2341X_OSD_SET_FRAMEBUFFER_WINDOW


Enum: 86/0x56

##### 描述


在屏幕上定位主输出窗口。坐标必须使得整个窗口能落入屏幕内。

##### 参数[0]


窗口宽度

##### 参数[1]


窗口高度

##### 参数[2]


左上角窗口水平偏移

##### 参数[3]


左上角窗口垂直偏移



#### CX2341X_OSD_SET_CHROMA_KEY


Enum: 96/0x60

##### 描述


色度键开关与颜色

##### 参数[0]


状态：0=off, 1=on

##### 参数[1]


颜色



#### CX2341X_OSD_GET_ALPHA_CONTENT_INDEX


Enum: 97/0x61

##### 描述


取回 alpha 内容索引

##### 结果[0]


alpha 内容索引，范围 0:15



#### CX2341X_OSD_SET_ALPHA_CONTENT_INDEX


Enum: 98/0x62

##### 描述


设置 alpha 内容索引

##### 参数[0]


alpha 内容索引，范围 0:15


### 编码器固件 API 描述


#### CX2341X_ENC_PING_FW


Enum: 128/0x80

##### 描述


空操作。可用于检查固件是否在响应。



#### CX2341X_ENC_START_CAPTURE


Enum: 129/0x81

##### 描述


开始捕获视频、音频和/或 VBI 数据。所有编码参数必须在此 API 调用之前初始化。持续捕获，直到捕获了预定义数量的帧。

##### 参数[0]


捕获流类型：

 - 0=MPEG
 - 1=Raw
 - 2=Raw passthrough
 - 3=VBI

##### 参数[1]


位掩码：

 - Bit 0 置位时，捕获 YUV
 - Bit 1 置位时，捕获 PCM 音频
 - Bit 2 置位时，捕获 VBI（同 param[^0^]=3）
 - Bit 3 置位时，捕获目标是解码器（同 param[^0^]=2）
 - Bit 4 置位时，捕获目标是主机



#### CX2341X_ENC_STOP_CAPTURE


Enum: 130/0x82

##### 描述


结束正在进行的捕获

##### 参数[0]


- 0=在 GOP 结束时停止（产生 IRQ）
- 1=立即停止（无 IRQ）

##### 参数[1]


要停止的流类型，见 API 0x81 的 param[^0^]

##### 参数[2]


子类型，见 API 0x81 的 param[^1^]



#### CX2341X_ENC_SET_AUDIO_ID


Enum: 137/0x89

##### 描述


设置编码后音频流的传输流 ID

##### 参数[0]


音频流 ID



#### CX2341X_ENC_SET_VIDEO_ID


Enum: 139/0x8B

##### 描述


设置视频传输流 ID

##### 参数[0]


视频流 ID



#### CX2341X_ENC_SET_PCR_ID


Enum: 141/0x8D

##### 描述


设置 PCR 包的传输流 ID

##### 参数[0]


PCR 流 ID



#### CX2341X_ENC_SET_FRAME_RATE


Enum: 143/0x8F

##### 描述


设置视频每秒帧数。更改在下一个 GOP 开始时生效。

##### 参数[0]


- 0=30fps
- 1=25fps



#### CX2341X_ENC_SET_FRAME_SIZE


Enum: 145/0x91

##### 描述


选择视频流编码分辨率。

##### 参数[0]


高度（行数）。默认 480

##### 参数[1]


宽度（像素）。默认 720



#### CX2341X_ENC_SET_BIT_RATE


Enum: 149/0x95

##### 描述


设置视频流平均码率。

##### 参数[0]


0=可变码率, 1=恒定码率

##### 参数[1]


码率（位每秒）

##### 参数[2]


峰值码率（位每秒），除以 400

##### 参数[3]


复用码率（位每秒），除以 400。可为 0（默认）。

##### 参数[4]


码率控制 VBR 填充

##### 参数[5]


编码器使用的 VBV 缓冲区

	#) Param\[3\] and Param\[4\] seem to be always 0
	#) Param\[5\] doesn't seem to be used.



#### CX2341X_ENC_SET_GOP_PROPERTIES


Enum: 151/0x97

##### 描述


设置 GOP 结构

##### 参数[0]


GOP 大小（最大 34）

##### 参数[1]


I 帧与 P 帧之间的 B 帧数，加 1。
例如：IBBPBBPBBPBB --> GOP 大小：12，B 帧数：2+1 = 3

	GOP 大小必须是（B 帧数 + 1）的倍数。



#### CX2341X_ENC_SET_ASPECT_RATIO


Enum: 153/0x99

##### 描述


设置编码宽高比。宽高比的改变在下一个 GOP 开始时生效。

##### 参数[0]


- '0000' forbidden
- '0001' 1:1 square
- '0010' 4:3
- '0011' 16:9
- '0100' 2.21:1
- '0101' to '1111' reserved



#### CX2341X_ENC_SET_DNR_FILTER_MODE


Enum: 155/0x9B

##### 描述


设置动态降噪（Dynamic Noise Reduction）工作模式

##### 参数[0]


Bit0：空间滤波器，置位=自动，清除=手动
Bit1：时间滤波器，置位=自动，清除=手动

##### 参数[1]


中值滤波器：

- 0=Disabled
- 1=Horizontal
- 2=Vertical
- 3=Horiz/Vert
- 4=Diagonal



#### CX2341X_ENC_SET_DNR_FILTER_PROPS


Enum: 157/0x9D

##### 描述


这些动态降噪中值滤波器的值仅当相应滤波器被设为"手动"（见 API 0x9B）时才有意义

##### 参数[0]


空间滤波器：默认 0，范围 0:15

##### 参数[1]


时间滤波器：默认 0，范围 0:31



#### CX2341X_ENC_SET_CORING_LEVELS


Enum: 159/0x9F

##### 描述


设置动态降噪中值滤波器属性。

##### 参数[0]


亮度中值滤波器启用所依据的阈值上限。
默认：0，范围 0:255

##### 参数[1]


亮度中值滤波器启用所依据的阈值下限。
默认：255，范围 0:255

##### 参数[2]


色度中值滤波器启用所依据的阈值上限。
默认：0，范围 0:255

##### 参数[3]


色度中值滤波器启用所依据的阈值下限。
默认：255，范围 0:255



#### CX2341X_ENC_SET_SPATIAL_FILTER_TYPE


Enum: 161/0xA1

##### 描述


设置空间预滤波参数

##### 参数[0]


亮度滤波器

- 0=Off
- 1=1D Horizontal
- 2=1D Vertical
- 3=2D H/V Separable (default)
- 4=2D Symmetric non-separable

##### 参数[1]


色度滤波器

- 0=Off
- 1=1D Horizontal (default)



#### CX2341X_ENC_SET_VBI_LINE


Enum: 183/0xB7

##### 描述


选择 VBI 行号。

##### 参数[0]


- Bits 0:4 	line number
- Bit  31		0=top_field, 1=bottom_field
- Bits 0:31 	all set specifies "all lines"

##### 参数[1]


VBI 行信息特性：0=disabled, 1=enabled

##### 参数[2]


切片：0=None, 1=Closed Caption
几乎可以确定未实现。设为 0。

##### 参数[3]


本行中的亮度采样数。
几乎可以确定未实现。设为 0。

##### 参数[4]


本行中的色度采样数
几乎可以确定未实现。设为 0。



#### CX2341X_ENC_SET_STREAM_TYPE


Enum: 185/0xB9

##### 描述


设置流类型


	Transport stream is not working in recent firmwares.
	And in older firmwares the timestamps in the TS seem to be
	unreliable.

##### 参数[0]


- 0=Program stream
- 1=Transport stream
- 2=MPEG1 stream
- 3=PES A/V stream
- 5=PES Video stream
- 7=PES Audio stream
- 10=DVD stream
- 11=VCD stream
- 12=SVCD stream
- 13=DVD_S1 stream
- 14=DVD_S2 stream



#### CX2341X_ENC_SET_OUTPUT_PORT


Enum: 187/0xBB

##### 描述


设置流输出端口。当数据通过 PCI 总线（DMA）复制时通常为 0，当数据流向另一颗芯片（pvrusb 和 cx88-blackbird）时为 1。

##### 参数[0]


- 0=Memory (default)
- 1=Streaming
- 2=Serial

##### 参数[1]


未知，但把它留为 0 似乎效果最好。有迹象表明这可能与 USB 支持有关，不过传入非 0 的任何值只会把事情弄糟。



#### CX2341X_ENC_SET_AUDIO_PROPERTIES


Enum: 189/0xBD

##### 描述


设置音频流属性，可在编码进行中调用。


	All bitfields are consistent with ISO11172 documentation except
	bits 2:3 which ISO docs define as:

 - '11' Layer I
 - '10' Layer II
 - '01' Layer III
 - '00' Undefined

	This discrepancy may indicate a possible error in the documentation.
	Testing indicated that only Layer II is actually working, and that
	the minimum bitrate should be 192 kbps.

##### 参数[0]


位掩码：


	   0:1  '00' 44.1Khz
		'01' 48Khz
		'10' 32Khz
		'11' reserved

	   2:3  '01'=Layer I
		'10'=Layer II

	   4:7  Bitrate:
		     Index | Layer I     | Layer II
		     ------+-------------+------------
		    '0000' | free format | free format
		    '0001' |  32 kbit/s  |  32 kbit/s
		    '0010' |  64 kbit/s  |  48 kbit/s
		    '0011' |  96 kbit/s  |  56 kbit/s
		    '0100' | 128 kbit/s  |  64 kbit/s
		    '0101' | 160 kbit/s  |  80 kbit/s
		    '0110' | 192 kbit/s  |  96 kbit/s
		    '0111' | 224 kbit/s  | 112 kbit/s
		    '1000' | 256 kbit/s  | 128 kbit/s
		    '1001' | 288 kbit/s  | 160 kbit/s
		    '1010' | 320 kbit/s  | 192 kbit/s
		    '1011' | 352 kbit/s  | 224 kbit/s
		    '1100' | 384 kbit/s  | 256 kbit/s
		    '1101' | 416 kbit/s  | 320 kbit/s
		    '1110' | 448 kbit/s  | 384 kbit/s

```
			For Layer II, not all combinations of total bitrate
			and mode are allowed. See ISO11172-3 3-Annex B,
			Table 3-B.2

	   8:9  '00'=Stereo
		'01'=JointStereo
		'10'=Dual
		'11'=Mono

		.. note::

			The cx23415 cannot decode Joint Stereo properly.

	  10:11 Mode Extension used in joint_stereo mode.
		In Layer I and II they indicate which subbands are in
		intensity_stereo. All other subbands are coded in stereo.
		    '00' subbands 4-31 in intensity_stereo, bound==4
		    '01' subbands 8-31 in intensity_stereo, bound==8
		    '10' subbands 12-31 in intensity_stereo, bound==12
		    '11' subbands 16-31 in intensity_stereo, bound==16

	  12:13 Emphasis:
		    '00' None
		    '01' 50/15uS
		    '10' reserved
		    '11' CCITT J.17

	  14 	CRC:
		    '0' off
		    '1' on

	  15    Copyright:
		    '0' off
		    '1' on

	  16    Generation:
		    '0' copy
		    '1' original

```

#### CX2341X_ENC_HALT_FW


Enum: 195/0xC3

##### 描述


固件被停止，在固件被重新上传之前不再服务任何 API 调用。



#### CX2341X_ENC_GET_VERSION


Enum: 196/0xC4

##### 描述


返回编码器固件的版本。

##### 结果[0]


版本位掩码：
- Bits  0:15 build
- Bits 16:23 minor
- Bits 24:31 major



#### CX2341X_ENC_SET_GOP_CLOSURE


Enum: 197/0xC5

##### 描述


设置 GOP 开放/闭合属性。

##### 参数[0]


- 0=Open
- 1=Closed



#### CX2341X_ENC_GET_SEQ_END


Enum: 198/0xC6

##### 描述


获取编码器缓冲区中的序列结束码。当一次捕获开始时仍会产生若干中断，其中最后一个中断的 Result[^0^] 将被置为 1，而 Result[^1^] 将包含缓冲区的大小。

##### 结果[0]


传输状态（若为最后一个缓冲区则为 1）

##### 结果[1]


若 Result[^0^] 为 1，则此处包含最后一个缓冲区的大小，否则未定义。



#### CX2341X_ENC_SET_PGM_INDEX_INFO


Enum: 199/0xC7

##### 描述


设置节目索引信息（Program Index Information）。
信息按如下方式存储：


	struct info {
		u32 length;		// Length of this frame
		u32 offset_low;		// Offset in the file of the
		u32 offset_high;	// start of this frame
		u32 mask1;		// Bits 0-2 are the type mask:
					// 1=I, 2=P, 4=B
					// 0=End of Program Index, other fields
					//   are invalid.
		u32 pts;		// The PTS of the frame
		u32 mask2;		// Bit 0 is bit 32 of the pts.
	};
	u32 table_ptr;
	struct info index[^400^];

table_ptr 是表中将写入**新**条目的编码器内存地址。

##### 参数[0]


图像掩码：
- 0=不捕获索引
- 1=I 帧
- 3=I, P 帧
- 7=I, P, B 帧

（似乎被忽略，它总是索引 I、P 和 B 帧）

##### 参数[1]


请求的元素数（最多 400）

##### 结果[0]


表起始处在编码器内存中的偏移。

##### 结果[1]


已分配的元素数，最多到 Param[^1^]



#### CX2341X_ENC_SET_VBI_CONFIG


Enum: 200/0xC8

##### 描述


配置 VBI 设置

##### 参数[0]


位图：


	    0    Mode '0' Sliced, '1' Raw
	    1:3  Insertion:
		     '000' insert in extension & user data
		     '001' insert in private packets
		     '010' separate stream and user data
		     '111' separate stream and private data
	    8:15 Stream ID (normally 0xBD)

##### 参数[1]


每个中断的帧数（最多 8）。仅在 raw 模式下有效。

##### 参数[2]


raw VBI 总帧数。仅在 raw 模式下有效。

##### 参数[3]


起始码

##### 参数[4]


停止码

##### 参数[5]


每帧行数

##### 参数[6]


每行字节数

##### 结果[0]


仅在 raw 模式下观察到的每个中断帧数。范围 1 到 Param[^1^]

##### 结果[1]


raw 模式下观察到的帧数。范围 1 到 Param[^2^]

##### 结果[2]


raw VBI 数据的起始内存偏移



#### CX2341X_ENC_SET_DMA_BLOCK_SIZE


Enum: 201/0xC9

##### 描述


设置 DMA 传输块大小

##### 参数[0]


DMA 传输块大小（字节或帧）。当单位为字节时，支持的块大小为 2^7、2^8 和 2^9 字节。

##### 参数[1]


单位：0=字节, 1=帧



#### CX2341X_ENC_GET_PREV_DMA_INFO_MB_10


Enum: 202/0xCA

##### 描述


结合中断掩码 bit 27 返回前一次 DMA 传输的信息。使用邮箱 10。

##### 结果[0]


流类型

##### 结果[1]


地址偏移

##### 结果[2]


传输的最大大小



#### CX2341X_ENC_GET_PREV_DMA_INFO_MB_9


Enum: 203/0xCB

##### 描述


结合中断掩码 bit 27 或 bit 18 返回前一次 DMA 传输的信息。使用邮箱 9。

##### 结果[0]


状态位：
- 0   read completed
- 1   write completed
- 2   DMA read error
- 3   DMA write error
- 4   Scatter-Gather array error

##### 结果[1]


DMA 类型

##### 结果[2]


呈现时间戳（Presentation Time Stamp）位 0..31

##### 结果[3]


呈现时间戳位 32



#### CX2341X_ENC_SCHED_DMA_TO_HOST


Enum: 204/0xCC

##### 描述


设置到主机的 DMA 操作

##### 参数[0]


链表的内存地址

##### 参数[1]


链表长度（wtf：什么单位 ???）

##### 参数[2]


DMA 类型（0=MPEG）



#### CX2341X_ENC_INITIALIZE_INPUT


Enum: 205/0xCD

##### 描述


初始化视频输入



#### CX2341X_ENC_SET_FRAME_DROP_RATE


Enum: 208/0xD0

##### 描述


对每一帧被捕获的帧，跳过指定数量的帧。

##### 参数[0]


要跳过的帧数



#### CX2341X_ENC_PAUSE_ENCODER


Enum: 210/0xD2

##### 描述


在暂停状态下，所有帧都被丢弃而不是被编码。

##### 参数[0]


- 0=暂停编码
- 1=继续编码



#### CX2341X_ENC_REFRESH_INPUT


Enum: 211/0xD3

##### 描述


刷新视频输入



#### CX2341X_ENC_SET_COPYRIGHT


Enum: 212/0xD4

##### 描述


设置流的版权属性

##### 参数[0]


- 0=流不受版权保护
- 1=流受版权保护



#### CX2341X_ENC_SET_EVENT_NOTIFICATION


Enum: 213/0xD5

##### 描述


设置固件以就某个特定事件通知主机。主机必须取消对该中断位的屏蔽。

##### 参数[0]


事件（0=刷新编码器输入）

##### 参数[1]


通知 0=禁用 1=启用

##### 参数[2]


中断位

##### 参数[3]


邮箱槽位，-1 表示不需要邮箱。



#### CX2341X_ENC_SET_NUM_VSYNC_LINES


Enum: 214/0xD6

##### 描述


根据所用的模拟视频解码器，设置场 1 和场 2 的行数。

##### 参数[0]


场 1 行数：
- 0x00EF for SAA7114
- 0x00F0 for SAA7115
- 0x0105 for Micronas

##### 参数[1]


场 2 行数：
- 0x00EF for SAA7114
- 0x00F0 for SAA7115
- 0x0106 for Micronas



#### CX2341X_ENC_SET_PLACEHOLDER


Enum: 215/0xD7

##### 描述


提供一种在 MPEG 流中插入自定义用户数据的机制。

##### 参数[0]


- 0=extension & user data
- 1=带流 ID 0xBD 的私有包

##### 参数[1]


插入数据的速率，单位为帧（对私有包）或 GOP（对 ext. & user data）

##### 参数[2]


要插入的数据 DWORD 数（如下）

##### 参数[3]


自定义数据 0

##### 参数[4]


自定义数据 1

##### 参数[5]


自定义数据 2

##### 参数[6]


自定义数据 3

##### 参数[7]


自定义数据 4

##### 参数[8]


自定义数据 5

##### 参数[9]


自定义数据 6

##### 参数[10]


自定义数据 7

##### 参数[11]


自定义数据 8



#### CX2341X_ENC_MUTE_VIDEO


Enum: 217/0xD9

##### 描述


视频静音

##### 参数[0]


位用法：


	 0    	'0'=video not muted
		'1'=video muted, creates frames with the YUV color defined below
	 1:7  	Unused
	 8:15 	V chrominance information
	16:23 	U chrominance information
	24:31 	Y luminance information



#### CX2341X_ENC_MUTE_AUDIO


Enum: 218/0xDA

##### 描述


音频静音

##### 参数[0]


- 0=audio not muted
- 1=audio muted (produces silent mpeg audio stream)



#### CX2341X_ENC_SET_VERT_CROP_LINE


Enum: 219/0xDB

##### 描述


与“Vertical Crop Line”相关的一些操作

##### 参数[0]


若为 saa7114 且 raw VBI 捕获且 60 Hz，则设为 10001。
否则为 0。



#### CX2341X_ENC_MISC


Enum: 220/0xDC

##### 描述


杂项操作。不能 100% 确定它的作用。它更像是一种 ioctl 调用。第一个参数是命令号，第二个是值。

##### 参数[0]


命令号：


	 1=set initial SCR value when starting encoding (works).
	 2=set quality mode (apparently some test setting).
	 3=setup advanced VIM protection handling.
	   Always 1 for the cx23416 and 0 for cx23415.
	 4=generate DVD compatible PTS timestamps
	 5=USB flush mode
	 6=something to do with the quantization matrix
	 7=set navigation pack insertion for DVD: adds 0xbf (private stream 2)
	   packets to the MPEG. The size of these packets is 2048 bytes (including
	   the header of 6 bytes: 0x000001bf + length). The payload is zeroed and
	   it is up to the application to fill them in. These packets are apparently
	   inserted every four frames.
	 8=enable scene change detection (seems to be a failure)
	 9=set history parameters of the video input module
	10=set input field order of VIM
	11=set quantization matrix
	12=reset audio interface after channel change or input switch (has no argument).
	   Needed for the cx2584x, not needed for the mspx4xx, but it doesn't seem to
	   do any harm calling it regardless.
	13=set audio volume delay
	14=set audio delay

##### 参数[1]


命令值。

### 解码器固件 API 描述



#### CX2341X_DEC_PING_FW


Enum: 0/0x00

##### 描述


此 API 调用不做任何事。可用于检查固件是否在响应。



#### CX2341X_DEC_START_PLAYBACK


Enum: 1/0x01

##### 描述


开始或恢复播放。

##### 参数[0]


从 GOP 中开始播放的、从 0 计数的帧号。

##### 参数[1]


指定在正常音频恢复之前播放的静音音频帧数。（固件未实现此功能，保留为 0）



#### CX2341X_DEC_STOP_PLAYBACK


Enum: 2/0x02

##### 描述


结束播放并清空所有解码器缓冲区。若 PTS 非零，则在指定的 PTS 处停止播放。

##### 参数[0]


显示 0=最后一帧, 1=黑屏

		this takes effect immediately, so if you want to wait for a PTS,
		then use '0', otherwise the screen goes to black at once.
		You can call this later (even if there is no playback) with a 1 value
		to set the screen to black.

##### 参数[1]


PTS 低位

##### 参数[2]


PTS 高位



#### CX2341X_DEC_SET_PLAYBACK_SPEED


Enum: 3/0x03

##### 描述


以非正常速度播放流。有两种操作模式：

 - Smooth：主机传输整个流，固件丢弃未使用的帧。
 - Coarse：主机根据索引按需丢弃帧以达到所需速度。

##### 参数[0]



	Bitmap:
	    0:7  0 normal
		 1 fast only "1.5 times"
		 n nX fast, 1/nX slow
	    30   Framedrop:
		     '0' during 1.5 times play, every other B frame is dropped
		     '1' during 1.5 times play, stream is unchanged (bitrate
			 must not exceed 8mbps)
	    31   Speed:
		     '0' slow
		     '1' fast


	n is limited to 2. Anything higher does not result in
	faster playback. Instead the host should start dropping frames.

##### 参数[1]


方向：0=forward, 1=reverse


	to make reverse playback work you have to write full GOPs in
	reverse order.

##### 参数[2]



	Picture mask:
	    1=I frames
	    3=I, P frames
	    7=I, P, B frames

##### 参数[3]


B frames per GOP (for reverse play only)


	for reverse playback the Picture Mask should be set to I or I, P.
	Adding B frames to the mask will result in corrupt video. This field
	has to be set to the correct value in order to keep the timing correct.

##### 参数[4]


Mute audio: 0=disable, 1=enable

##### 参数[5]


Display 0=frame, 1=field

##### 参数[6]


指定在正常音频恢复之前播放的静音音频帧数。（固件未实现此功能，保留为 0）



#### CX2341X_DEC_STEP_VIDEO


Enum: 5/0x05

##### 描述


此 API 的每次调用都会将播放步进到下面定义的、当前播放方向上的下一个单元。

##### 参数[0]


0=frame, 1=top field, 2=bottom field



#### CX2341X_DEC_SET_DMA_BLOCK_SIZE


Enum: 8/0x08

##### 描述


设置 DMA 传输块大小。API 0xC9 的对应项。

##### 参数[0]


DMA 传输块大小（字节）。发出 DMA 传输命令时可指定不同大小。



#### CX2341X_DEC_GET_XFER_INFO


Enum: 9/0x09

##### 描述


此 API 调用可用于检测流结束（end of stream）条件。

##### 结果[0]


流类型

##### 结果[1]


地址偏移

##### 结果[2]


最大传输字节数

##### 结果[3]


缓冲区充盈度



#### CX2341X_DEC_GET_DMA_STATUS


Enum: 10/0x0A

##### 描述


上一次 DMA 传输的状态

##### 结果[0]


Bit 1 set means transfer complete
Bit 2 set means DMA error
Bit 3 set means linked list error

##### 结果[1]


DMA type: 0=MPEG, 1=OSD, 2=YUV



#### CX2341X_DEC_SCHED_DMA_FROM_HOST


Enum: 11/0x0B

##### 描述


设置从主机的 DMA 操作。API 0xCC 的对应项。

##### 参数[0]


链表的内存地址

##### 参数[1]


要传输的总字节数

##### 参数[2]


DMA 类型（0=MPEG, 1=OSD, 2=YUV）



#### CX2341X_DEC_PAUSE_PLAYBACK


Enum: 13/0x0D

##### 描述


立即冻结播放。在此模式下，当内部缓冲区满时，不再接收更多数据，数据请求 IRQ 也会被屏蔽。

##### 参数[0]


显示：0=最后一帧, 1=黑屏



#### CX2341X_DEC_HALT_FW


Enum: 14/0x0E

##### 描述


固件被停止，在固件被重新上传之前不再服务任何 API 调用。



#### CX2341X_DEC_SET_STANDARD


Enum: 16/0x10

##### 描述


选择显示标准

##### 参数[0]


0=NTSC, 1=PAL



#### CX2341X_DEC_GET_VERSION


Enum: 17/0x11

##### 描述


返回解码器固件版本信息

##### 结果[0]


版本位掩码：
 - Bits  0:15 build
 - Bits 16:23 minor
 - Bits 24:31 major



#### CX2341X_DEC_SET_STREAM_INPUT


Enum: 20/0x14

##### 描述


选择解码器流输入端口

##### 参数[0]


0=memory (default), 1=streaming



#### CX2341X_DEC_GET_TIMING_INFO


Enum: 21/0x15

##### 描述


返回从播放开始起的时序信息

##### 结果[0]


按解码顺序的帧计数

##### 结果[1]


按显示顺序的视频 PTS 位 0:31

##### 结果[2]


按显示顺序的视频 PTS 位 32

##### 结果[3]


按显示顺序的 SCR 位 0:31

##### 结果[4]


按显示顺序的 SCR 位 32



#### CX2341X_DEC_SET_AUDIO_MODE


Enum: 22/0x16

##### 描述


选择音频模式

##### 参数[0]


双声道单声模式动作
	0=Stereo, 1=Left, 2=Right, 3=Mono, 4=Swap, -1=Unchanged

##### 参数[1]


立体声模式动作：
	0=Stereo, 1=Left, 2=Right, 3=Mono, 4=Swap, -1=Unchanged



#### CX2341X_DEC_SET_EVENT_NOTIFICATION


Enum: 23/0x17

##### 描述


设置固件以就某个特定事件通知主机。
API 0xD5 的对应项。

##### 参数[0]


事件：
 - 0=音频模式在 mono、(joint) stereo 和 dual channel 之间变化。
 - 3=解码器已启动
 - 4=未知：解码时每秒触发 10-15 次。
 - 5=某个同步事件：每帧触发一次。

##### 参数[1]


通知 0=禁用, 1=启用

##### 参数[2]


中断位

##### 参数[3]


邮箱槽位，-1 表示不需要邮箱。



#### CX2341X_DEC_SET_DISPLAY_BUFFERS


Enum: 24/0x18

##### 描述


显示缓冲区数量。要在倒放中解码所有帧，必须使用九个缓冲区。

##### 参数[0]


0=six buffers, 1=nine buffers



#### CX2341X_DEC_EXTRACT_VBI


Enum: 25/0x19

##### 描述


提取 VBI 数据

##### 参数[0]


0=从 extension & user data 提取, 1=从私有包提取

##### 结果[0]


VBI 表位置

##### 结果[1]


VBI 表大小



#### CX2341X_DEC_SET_DECODER_SOURCE


Enum: 26/0x1A

##### 描述


选择解码器源。确保传给此 API 的参数与编码器设置相匹配。

##### 参数[0]


模式：0=MPEG from host, 1=YUV from encoder, 2=YUV from host

##### 参数[1]


YUV 图像宽度

##### 参数[2]


YUV 图像高度

##### 参数[3]


位图：见 API 0xBD 的 Param[^0^]



#### CX2341X_DEC_SET_PREBUFFERING


Enum: 30/0x1E

##### 描述


解码器预缓冲，启用时，对于 <8mbps 的流缓冲最多 128KB，对于 >8mbps 的流缓冲最多 640KB。

##### 参数[0]


0=off, 1=on

### PVR350 视频解码器寄存器 0x02002800 -> 0x02002B00


Author: Ian Armstrong <ian@iarmst.demon.co.uk>

Version: v0.4

Date: 12 March 2007


此列表是通过反复试验得出的。其中会有错误和遗漏。有些寄存器没有明显的效果，所以很难说它们做什么；而另一些会相互影响，或需要特定的加载顺序。水平滤波器设置就是一个例子：有六个寄存器协同工作，并需要特定的加载顺序才能正确配置。索引色彩调色板只需两个寄存器就能更容易地设置，但同样需要特定的加载顺序。

有些寄存器对它们的设置值很挑剔。载入错误的值，解码器就会失效。重新加载固件通常能恢复，但有时需要复位。对于包含大小信息的寄存器，把它们的地址设为 0 通常是个坏主意。对于其他控制寄存器（如 2878），只有它挂起时你才会发现哪些值是坏的。


	--------------------------------------------------------------------------------
	2800
	bit 0
		Decoder enable
		0 = disable
# 		1 = enable

	2804
	bits 0:31
		Decoder horizontal Y alias register 1
	---------------
	2808
	bits 0:31
		Decoder horizontal Y alias register 2
	---------------
	280C
	bits 0:31
		Decoder horizontal Y alias register 3
	---------------
	2810
	bits 0:31
		Decoder horizontal Y alias register 4
	---------------
	2814
	bits 0:31
		Decoder horizontal Y alias register 5
	---------------
	2818
	bits 0:31
		Decoder horizontal Y alias trigger

	These six registers control the horizontal aliasing filter for the Y plane.
	The first five registers must all be loaded before accessing the trigger
	(2818), as this register actually clocks the data through for the first
	five.

	To correctly program set the filter, this whole procedure must be done 16
	times. The actual register contents are copied from a lookup-table in the
	firmware which contains 4 different filter settings.

	--------------------------------------------------------------------------------
	281C
	bits 0:31
		Decoder horizontal UV alias register 1
	---------------
	2820
	bits 0:31
		Decoder horizontal UV alias register 2
	---------------
	2824
	bits 0:31
		Decoder horizontal UV alias register 3
	---------------
	2828
	bits 0:31
		Decoder horizontal UV alias register 4
	---------------
	282C
	bits 0:31
		Decoder horizontal UV alias register 5
	---------------
	2830
	bits 0:31
		Decoder horizontal UV alias trigger

	These six registers control the horizontal aliasing for the UV plane.
	Operation is the same as the Y filter, with 2830 being the trigger
	register.

	--------------------------------------------------------------------------------
	2834
	bits 0:15
		Decoder Y source width in pixels

	bits 16:31
		Decoder Y destination width in pixels
	---------------
	2838
	bits 0:15
		Decoder UV source width in pixels

	bits 16:31
		Decoder UV destination width in pixels

	NOTE: For both registers, the resulting image must be fully visible on
	screen. If the image exceeds the right edge both the source and destination
	size must be adjusted to reflect the visible portion. For the source width,
# 	you must take into account the scaling when calculating the new value.


	283C
	bits 0:31
		Decoder Y horizontal scaling
			Normally = Reg 2854 >> 2
	---------------
	2840
	bits 0:31
		Decoder ?? unknown - horizontal scaling
		Usually 0x00080514
	---------------
	2844
	bits 0:31
		Decoder UV horizontal scaling
		Normally = Reg 2854 >> 2
	---------------
	2848
	bits 0:31
		Decoder ?? unknown - horizontal scaling
		Usually 0x00100514
	---------------
	284C
	bits 0:31
		Decoder ?? unknown - Y plane
		Usually 0x00200020
	---------------
	2850
	bits 0:31
		Decoder ?? unknown - UV plane
		Usually 0x00200020
	---------------
	2854
	bits 0:31
		Decoder 'master' value for horizontal scaling
	---------------
	2858
	bits 0:31
		Decoder ?? unknown
# 		Usually 0

	285C
	bits 0:31
		Decoder ?? unknown
		Normally = Reg 2854 >> 1
	---------------
	2860
	bits 0:31
		Decoder ?? unknown
# 		Usually 0

	2864
	bits 0:31
		Decoder ?? unknown
		Normally = Reg 2854 >> 1
	---------------
	2868
	bits 0:31
		Decoder ?? unknown
		Usually 0

	Most of these registers either control horizontal scaling, or appear linked
	to it in some way. Register 2854 contains the 'master' value & the other
	registers can be calculated from that one. You must also remember to
	correctly set the divider in Reg 2874.

	To enlarge:
		Reg 2854 = (source_width * 0x00200000) / destination_width
		Reg 2874 = No divide

	To reduce from full size down to half size:
		Reg 2854 = (source_width/2 * 0x00200000) / destination width
		Reg 2874 = Divide by 2

	To reduce from half size down to quarter size:
		Reg 2854 = (source_width/4 * 0x00200000) / destination width
		Reg 2874 = Divide by 4

	The result is always rounded up.

	--------------------------------------------------------------------------------
	286C
	bits 0:15
		Decoder horizontal Y buffer offset

	bits 15:31
		Decoder horizontal UV buffer offset

	Offset into the video image buffer. If the offset is gradually incremented,
	the on screen image will move left & wrap around higher up on the right.

	--------------------------------------------------------------------------------
	2870
	bits 0:15
		Decoder horizontal Y output offset

	bits 16:31
		Decoder horizontal UV output offset

	Offsets the actual video output. Controls output alignment of the Y & UV
	planes. The higher the value, the greater the shift to the left. Use
	reg 2890 to move the image right.

	--------------------------------------------------------------------------------
	2874
	bits 0:1
		Decoder horizontal Y output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 3

	bits 4:5
		Decoder horizontal UV output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 3

	bit 8
		Decoder ?? unknown
		0 = Normal
		1 = Affects video output levels

	bit 16
		Decoder ?? unknown
		0 = Normal
		1 = Disable horizontal filter

	--------------------------------------------------------------------------------
	2878
	bit 0
		?? unknown

	bit 1
		osd on/off
		0 = osd off
		1 = osd on

	bit 2
		Decoder + osd video timing
		0 = NTSC
		1 = PAL

	bits 3:4
		?? unknown

	bit 5
		Decoder + osd
		Swaps upper & lower fields

	--------------------------------------------------------------------------------
	287C
	bits 0:10
		Decoder & osd ?? unknown
		Moves entire screen horizontally. Starts at 0x005 with the screen
		shifted heavily to the right. Incrementing in steps of 0x004 will
		gradually shift the screen to the left.

	bits 11:31
		?? unknown

	Normally contents are 0x00101111 (NTSC) or 0x1010111d (PAL)

	--------------------------------------------------------------------------------
	2880  --------    ?? unknown
# 	2884  --------    ?? unknown

	2888
	bit 0
		Decoder + osd ?? unknown
		0 = Normal
		1 = Misaligned fields (Correctable through 289C & 28A4)

	bit 4
		?? unknown

	bit 8
		?? unknown

	Warning: Bad values will require a firmware reload to recover.
# 			Known to be bad are 0x000,0x011,0x100,0x111

	288C
	bits 0:15
		osd ?? unknown
		Appears to affect the osd position stability. The higher the value the
		more unstable it becomes. Decoder output remains stable.

	bits 16:31
		osd ?? unknown
		Same as bits 0:15

	--------------------------------------------------------------------------------
	2890
	bits 0:11
		Decoder output horizontal offset.

	Horizontal offset moves the video image right. A small left shift is
	possible, but it's better to use reg 2870 for that due to its greater
	range.

	NOTE: Video corruption will occur if video window is shifted off the right
# 	edge. To avoid this read the notes for 2834 & 2838.

	2894
	bits 0:23
		Decoder output video surround colour.

	Contains the colour (in yuv) used to fill the screen when the video is
# 	running in a window.

	2898
	bits 0:23
		Decoder video window colour
		Contains the colour (in yuv) used to fill the video window when the
		video is turned off.

	bit 24
		Decoder video output
		0 = Video on
		1 = Video off

	bit 28
		Decoder plane order
		0 = Y,UV
		1 = UV,Y

	bit 29
		Decoder second plane byte order
		0 = Normal (UV)
		1 = Swapped (VU)

	In normal usage, the first plane is Y & the second plane is UV. Though the
	order of the planes can be swapped, only the byte order of the second plane
	can be swapped. This isn't much use for the Y plane, but can be useful for
	the UV plane.

	--------------------------------------------------------------------------------
	289C
	bits 0:15
		Decoder vertical field offset 1

	bits 16:31
		Decoder vertical field offset 2

	Controls field output vertical alignment. The higher the number, the lower
	the image on screen. Known starting values are 0x011E0017 (NTSC) &
# 	0x01500017 (PAL)

	28A0
	bits 0:15
		Decoder & osd width in pixels

	bits 16:31
		Decoder & osd height in pixels

	All output from the decoder & osd are disabled beyond this area. Decoder
	output will simply go black outside of this region. If the osd tries to
# 	exceed this area it will become corrupt.

	28A4
	bits 0:11
		osd left shift.

	Has a range of 0x770->0x7FF. With the exception of 0, any value outside of
# 	this range corrupts the osd.

	28A8
	bits 0:15
		osd vertical field offset 1

	bits 16:31
		osd vertical field offset 2

	Controls field output vertical alignment. The higher the number, the lower
	the image on screen. Known starting values are 0x011E0017 (NTSC) &
# 	0x01500017 (PAL)

	28AC  --------    ?? unknown
	|
	V
# 	28BC  --------    ?? unknown

	28C0
	bit 0
		Current output field
		0 = first field
		1 = second field

	bits 16:31
		Current scanline
		The scanline counts from the top line of the first field
# 		through to the last line of the second field.

	28C4  --------    ?? unknown
	|
	V
# 	28F8  --------    ?? unknown

	28FC
	bit 0
		?? unknown
		0 = Normal
# 		1 = Breaks decoder & osd output

	2900
	bits 0:31
		Decoder vertical Y alias register 1
	---------------
	2904
	bits 0:31
		Decoder vertical Y alias register 2
	---------------
	2908
	bits 0:31
		Decoder vertical Y alias trigger

	These three registers control the vertical aliasing filter for the Y plane.
	Operation is similar to the horizontal Y filter (2804). The only real
	difference is that there are only two registers to set before accessing
	the trigger register (2908). As for the horizontal filter, the values are
	taken from a lookup table in the firmware, and the procedure must be
# 	repeated 16 times to fully program the filter.

	290C
	bits 0:31
		Decoder vertical UV alias register 1
	---------------
	2910
	bits 0:31
		Decoder vertical UV alias register 2
	---------------
	2914
	bits 0:31
		Decoder vertical UV alias trigger

	These three registers control the vertical aliasing filter for the UV
# 	plane. Operation is the same as the Y filter, with 2914 being the trigger.

	2918
	bits 0:15
		Decoder Y source height in pixels

	bits 16:31
		Decoder Y destination height in pixels
	---------------
	291C
	bits 0:15
		Decoder UV source height in pixels divided by 2

	bits 16:31
		Decoder UV destination height in pixels

	NOTE: For both registers, the resulting image must be fully visible on
	screen. If the image exceeds the bottom edge both the source and
	destination size must be adjusted to reflect the visible portion. For the
	source height, you must take into account the scaling when calculating the
# 	new value.

	2920
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2930 >> 2
	---------------
	2924
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2920 + 0x514
	---------------
	2928
	bits 0:31
		Decoder UV vertical scaling
		When enlarging = Reg 2930 >> 2
		When reducing = Reg 2930 >> 3
	---------------
	292C
	bits 0:31
		Decoder UV vertical scaling
		Normally = Reg 2928 + 0x514
	---------------
	2930
	bits 0:31
		Decoder 'master' value for vertical scaling
	---------------
	2934
	bits 0:31
		Decoder ?? unknown - Y vertical scaling
	---------------
	2938
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2930
	---------------
	293C
	bits 0:31
		Decoder ?? unknown - Y vertical scaling
	---------------
	2940
	bits 0:31
		Decoder UV vertical scaling
		When enlarging = Reg 2930 >> 1
		When reducing = Reg 2930
	---------------
	2944
	bits 0:31
		Decoder ?? unknown - UV vertical scaling
	---------------
	2948
	bits 0:31
		Decoder UV vertical scaling
		Normally = Reg 2940
	---------------
	294C
	bits 0:31
		Decoder ?? unknown - UV vertical scaling

	Most of these registers either control vertical scaling, or appear linked
	to it in some way. Register 2930 contains the 'master' value & all other
	registers can be calculated from that one. You must also remember to
	correctly set the divider in Reg 296C

	To enlarge:
		Reg 2930 = (source_height * 0x00200000) / destination_height
		Reg 296C = No divide

	To reduce from full size down to half size:
		Reg 2930 = (source_height/2 * 0x00200000) / destination height
		Reg 296C = Divide by 2

	To reduce from half down to quarter.
		Reg 2930 = (source_height/4 * 0x00200000) / destination height
		Reg 296C = Divide by 4

	--------------------------------------------------------------------------------
	2950
	bits 0:15
		Decoder Y line index into display buffer, first field

	bits 16:31
# 		Decoder Y vertical line skip, first field

	2954
	bits 0:15
		Decoder Y line index into display buffer, second field

	bits 16:31
# 		Decoder Y vertical line skip, second field

	2958
	bits 0:15
		Decoder UV line index into display buffer, first field

	bits 16:31
# 		Decoder UV vertical line skip, first field

	295C
	bits 0:15
		Decoder UV line index into display buffer, second field

	bits 16:31
# 		Decoder UV vertical line skip, second field

	2960
	bits 0:15
		Decoder destination height minus 1

	bits 16:31
# 		Decoder destination height divided by 2

	2964
	bits 0:15
		Decoder Y vertical offset, second field

	bits 16:31
		Decoder Y vertical offset, first field

	These two registers shift the Y plane up. The higher the number, the
# 	greater the shift.

	2968
	bits 0:15
		Decoder UV vertical offset, second field

	bits 16:31
		Decoder UV vertical offset, first field

	These two registers shift the UV plane up. The higher the number, the
# 	greater the shift.

	296C
	bits 0:1
		Decoder vertical Y output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 4

	bits 8:9
		Decoder vertical UV output size divider
		00 = No divide
		01 = Divide by 2
# 		10 = Divide by 4

	2970
	bit 0
		Decoder ?? unknown
		0 = Normal
		1 = Affect video output levels

	bit 16
		Decoder ?? unknown
		0 = Normal
		1 = Disable vertical filter

	--------------------------------------------------------------------------------
	2974  --------   ?? unknown
	|
	V
# 	29EF  --------   ?? unknown

	2A00
	bits 0:2
		osd colour mode
		000 = 8 bit indexed
		001 = 16 bit (565)
		010 = 15 bit (555)
		011 = 12 bit (444)
		100 = 32 bit (8888)

	bits 4:5
		osd display bpp
		01 = 8 bit
		10 = 16 bit
		11 = 32 bit

	bit 8
		osd global alpha
		0 = Off
		1 = On

	bit 9
		osd local alpha
		0 = Off
		1 = On

	bit 10
		osd colour key
		0 = Off
		1 = On

	bit 11
		osd ?? unknown
		Must be 1

	bit 13
		osd colour space
		0 = ARGB
		1 = AYVU

	bits 16:31
		osd ?? unknown
		Must be 0x001B (some kind of buffer pointer ?)

	When the bits-per-pixel is set to 8, the colour mode is ignored and
	assumed to be 8 bit indexed. For 16 & 32 bits-per-pixel the colour depth
	is honoured, and when using a colour depth that requires fewer bytes than
	allocated the extra bytes are used as padding. So for a 32 bpp with 8 bit
	index colour, there are 3 padding bytes per pixel. It's also possible to
	select 16bpp with a 32 bit colour mode. This results in the pixel width
	being doubled, but the color key will not work as expected in this mode.

	Colour key is as it suggests. You designate a colour which will become
	completely transparent. When using 565, 555 or 444 colour modes, the
	colour key is always 16 bits wide. The colour to key on is set in Reg 2A18.

	Local alpha works differently depending on the colour mode. For 32bpp & 8
	bit indexed, local alpha is a per-pixel 256 step transparency, with 0 being
	transparent and 255 being solid. For the 16bpp modes 555 & 444, the unused
	bit(s) act as a simple transparency switch, with 0 being solid & 1 being
	fully transparent. There is no local alpha support for 16bit 565.

	Global alpha is a 256 step transparency that applies to the entire osd,
	with 0 being transparent & 255 being solid.

# 	It's possible to combine colour key, local alpha & global alpha.

	2A04
	bits 0:15
		osd x coord for left edge

	bits 16:31
		osd y coord for top edge
	---------------
	2A08
	bits 0:15
		osd x coord for right edge

	bits 16:31
		osd y coord for bottom edge

	For both registers, (0,0) = top left corner of the display area. These
	registers do not control the osd size, only where it's positioned & how
	much is visible. The visible osd area cannot exceed the right edge of the
	display, otherwise the osd will become corrupt. See reg 2A10 for
# 	setting osd width.

	2A0C
	bits 0:31
		osd buffer index

	An index into the osd buffer. Slowly incrementing this moves the osd left,
# 	wrapping around onto the right edge

	2A10
	bits 0:11
		osd buffer 32 bit word width

	Contains the width of the osd measured in 32 bit words. This means that all
# 	colour modes are restricted to a byte width which is divisible by 4.

	2A14
	bits 0:15
		osd height in pixels

	bits 16:32
		osd line index into buffer
# 		osd will start displaying from this line.

	2A18
	bits 0:31
		osd colour key

# 	Contains the colour value which will be transparent.

	2A1C
	bits 0:7
		osd global alpha

# 	Contains the global alpha value (equiv ivtvfbctl --alpha XX)

	2A20  --------    ?? unknown
	|
	V
# 	2A2C  --------    ?? unknown

	2A30
	bits 0:7
		osd colour to change in indexed palette
	---------------
	2A34
	bits 0:31
		osd colour for indexed palette

	To set the new palette, first load the index of the colour to change into
	2A30, then load the new colour into 2A34. The full palette is 256 colours,
# 	so the index range is 0x00-0xFF

	2A38  --------    ?? unknown
# 	2A3C  --------    ?? unknown

	2A40
	bits 0:31
		osd ?? unknown

# 	Affects overall brightness, wrapping around to black

	2A44
	bits 0:31
		osd ?? unknown

# 	Green tint

	2A48
	bits 0:31
		osd ?? unknown

# 	Red tint

	2A4C
	bits 0:31
		osd ?? unknown

# 	Affects overall brightness, wrapping around to black

	2A50
	bits 0:31
		osd ?? unknown

# 	Colour shift

	2A54
	bits 0:31
		osd ?? unknown

# 	Colour shift

	2A58  --------    ?? unknown
	|
	V
# 	2AFC  --------    ?? unknown

	2B00
	bit 0
		osd filter control
		0 = filter off
		1 = filter on

	bits 1:4
		osd ?? unknown

	--------------------------------------------------------------------------------

### The cx231xx DMA engine



本页描述 cx2341x DMA 引擎所使用的结构与流程。

#### 简介


cx2341x 的 PCI 接口具备总线主控（busmaster）能力。这意味着它有一个 DMA 引擎，可以在卡与主存之间高效地传输大量数据，而无需 CPU 协助。和大多数硬件一样，它必须在连续的物理内存上操作。在虚拟内存机器上，大块的连续物理内存很难获得。

因此，它也支持一种称为“scatter-gather”（分散/聚集）的技术。该卡可以在一次操作中传输多个缓冲区。驱动无需分配一个大的连续缓冲区，而可以分配多个较小的缓冲区。

在实践中，我见过平均传输量大约是 80K，但超过 128K 的传输并不少见，尤其是在启动时。128K 这个数字很重要，因为它是内核通常能分配的最大块。即便如此，128K 的块也很难获得，因此强烈建议驱动编写者选择更小的块大小，并学习 scatter-gather 技术。

邮箱 #10 保留给 DMA 传输信息使用。

注意：硬件期望小端数据（'intel format'）。

#### 流程


本节总体上描述处理 DMA 传输时事件的顺序。详细信息在本节之后给出。

- 卡触发 Encoder 中断。
- 驱动从邮箱 #10 读取传输类型、偏移和大小。
- 驱动用足够多的空闲 dma 缓冲区构造 scatter-gather 数组以覆盖该大小。
- 驱动通过 ScheduleDMAtoHost API 调用调度 DMA 传输。
- 卡触发 DMA Complete 中断。
- 驱动检查 DMA 状态寄存器以发现任何错误。
- 驱动对刚传输的缓冲区进行后处理。

注意！Encoder 和 DMA Complete 中断有可能同时被触发。（上一次的结束、下一次的开始，等等。）

#### 邮箱 #10


Flags、Command、Return Value 和 Timeout 字段被忽略。

- Name:       Mailbox #10
- Results[^0^]: Type: 0: MPEG.
- Results[^1^]: Offset: 相对于卡内存空间的位置。
- Results[^2^]: Size: 要传输的确切字节数。

我推测，既然 StartCapture API 有一个可用的捕获类型 "RAW"，那么 type 字段将会有对应 YUV 和 PCM 数据的其他值。

#### Scatter-Gather 数组


scatter-gather 数组是一块连续分配的内存，它告诉卡每个待传输数据块的源和目的。卡的“地址”由邮箱 #10 提供的偏移推导而来。主机地址是目标 DMA 缓冲区的物理内存位置。

每个 S-G 数组元素是一个由三个 32 位字组成的结构体。第一个字是源地址，第二个是目的地址。两者各占满 32 位。第三个字的低 18 位是传输字节计数。第三个字的最高位是“last”标志。last 标志告诉卡触发 DMA_DONE 中断。根据我痛苦的个人经验，如果你忘了设置这个比特，卡仍会“工作”，但流极有可能被损坏。

传输计数必须是 256 的倍数。因此，驱动需要跟踪目标缓冲区中有多少数据是有效的，并相应地处理。

数组元素：

- 32 位源地址
- 32 位目的地址
- 14 位保留（最高位是 last 标志）
- 18 位字节计数

#### DMA 传输状态


寄存器 0x0004 保存 DMA 传输状态：

- bit 0:   read completed
- bit 1:   write completed
- bit 2:   DMA read error
- bit 3:   DMA write error
- bit 4:   Scatter-Gather array error


