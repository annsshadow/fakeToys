
## Zoran 驱动


统一 zoran 驱动（zr360x7、zoran、buz、dc10(+)、dc30(+)、lml33
网站：http://mjpeg.sourceforge.net/driver-zoran/


### 常见问题

### 支持哪些
Iomega Buz、Linux Media Labs LML33/LML33R10、Pinnacle/Miro DC10/DC10+/DC30/DC30+ 以及相关的板卡（以多种名称销售）
#### Iomega Buz

- Zoran zr36067 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Philips saa7111 TV 解码- Philips saa7185 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, saa7111, saa7185, zr36060, zr36067

输入/输出：复合与 S-video

制式：PAL、SECAM20x576 @ 25 fps）、NTSC20x480 @ 29.97 fps
卡号

#### AverMedia 6 Eyes AVS6EYES

- Zoran zr36067 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Samsung ks0127 TV 解码- Conexant bt866 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, ks0127, bt866, zr36060, zr36067

输入/输出	六个物理输入-6 为复合，1-2-4-6 兼作 S-video-3 兼作分量	一个复合输出
制式：PAL、SECAM20x576 @ 25 fps）、NTSC20x480 @ 29.97 fps
卡号


    未自动检测，必须使用 card=8
#### Linux Media Labs LML33

- Zoran zr36067 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Brooktree bt819 TV 解码- Brooktree bt856 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, bt819, bt856, zr36060, zr36067

输入/输出：复合与 S-video

制式：PAL20x576 @ 25 fps）、NTSC20x480 @ 29.97 fps
卡号

#### Linux Media Labs LML33R10

- Zoran zr36067 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Philips saa7114 TV 解码- Analog Devices adv7170 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, saa7114, adv7170, zr36060, zr36067

输入/输出：复合与 S-video

制式：PAL20x576 @ 25 fps）、NTSC20x480 @ 29.97 fps
卡号

#### Pinnacle/Miro DC10（新
- Zoran zr36057 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Philips saa7110a TV 解码- Analog Devices adv7176 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, saa7110, adv7175, zr36060, zr36067

输入/输出：复合、S-video 与内
制式：PAL、SECAM68x576 @ 25 fps）、NTSC40x480 @ 29.97 fps
卡号

#### Pinnacle/Miro DC10+

- Zoran zr36067 PCI 控制- Zoran zr36060 MJPEG 编解码器
- Philips saa7110a TV 解码- Analog Devices adv7176 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, saa7110, adv7175, zr36060, zr36067

输入/输出：复合、S-video 与内
制式：PAL、SECAM68x576 @ 25 fps）、NTSC40x480 @ 29.97 fps
卡号

#### Pinnacle/Miro DC10（旧
- Zoran zr36057 PCI 控制- Zoran zr36050 MJPEG 编解码器
- Zoran zr36016 视频前端，或 Fuji md0211 视频前端（克隆？- Micronas vpx3220a TV 解码- mse3000 TV 编码Analog Devices adv7176 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, vpx3220, mse3000/adv7175, zr36050, zr36016, zr36067

输入/输出：复合、S-video 与内
制式：PAL、SECAM68x576 @ 25 fps）、NTSC40x480 @ 29.97 fps
卡号

#### Pinnacle/Miro DC30

- Zoran zr36057 PCI 控制- Zoran zr36050 MJPEG 编解码器
- Zoran zr36016 视频前端
- Micronas vpx3225d/vpx3220a/vpx3216b TV 解码- Analog Devices adv7176 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, vpx3220/vpx3224, adv7175, zr36050, zr36016, zr36067

输入/输出：复合、S-video 与内
制式：PAL、SECAM68x576 @ 25 fps）、NTSC40x480 @ 29.97 fps
卡号

#### Pinnacle/Miro DC30+

- Zoran zr36067 PCI 控制- Zoran zr36050 MJPEG 编解码器
- Zoran zr36016 视频前端
- Micronas vpx3225d/vpx3220a/vpx3216b TV 解码- Analog Devices adv7176 TV 编码
需使用的驱动：videodev, i2c-core, i2c-algo-bit, videocodec, vpx3220/vpx3224, adv7175, zr36050, zr36015, zr36067

输入/输出：复合、S-video 与内
制式：PAL、SECAM68x576 @ 25 fps）、NTSC40x480 @ 29.97 fps
卡号


    #) 目前还没mse3000 的模    #) 目前还没vpx3224 的模
### 1.1 TV 解码器能做什么、不能做什
最广为人知的电视标准是 NTSC/PAL/SECAM，但仅此信息不足以解码一帧画面。电视标准有多种格式，而且并非每个 TV 解码器都能处理每种格式。驱动也并非支持每种组合。目前全球共11 种不同的电视广播格式
CCIR 定义了广播信号所需的参数。CCIR 定义了不同的标准：A、B、D、E、F、G、H、I、K、K1、L、M、N…CCIR 对所使用的彩色制式几乎没有规定！！！而谈论彩色制式时，也不能说明它是如何广播的
CCIR 标准 A、E、F 已不再使用
当你说到 NTSC 时，通常指：使用 NTSC 彩色制式CCIR - M 标准，用于美国、日本、墨西哥、加拿大等少数国家
当你说到 PAL 时，通常指：使用 PAL 彩色制式CCIR - B/G 标准，用于许多国家
当你说到 SECAM 时，指的是：使用 SECAM 彩色制式CCIR - L 标准，用于法国等少数国家
另有版本SECAM，即 CCIR - D/K，用于保加利亚、中国、斯洛伐克、匈牙利、韩国（共和国）、波兰、罗马尼亚等地
CCIR - H 使用 PAL 彩色制式（有时为 SECAM），用于埃及、利比亚、斯里兰卡、阿拉伯叙利亚共和国等
CCIR - I 使用 PAL 彩色制式，用于英国、香港、爱尔兰、尼日利亚、南非
CCIR - N 使用 PAL 彩色制式PAL 帧尺寸，但采NTSC 帧率，用于阿根廷、乌拉圭等少数国家
我们不讨论音频是如何广播的！

关于电视标准的几个相当不错的网站是：
http://www.sony.jp/support/
http://info.electronicwerkstatt.de/bereiche/fernsehtechnik/frequenzen_und_normen/Fernsehnormen/
以及 http://www.cabl.com/restaurant/channel.html

其它怪异之处：NTSC 4.43 是一种修改过NTSC，主要用于能够播NTSC PAL 录像机。PAL 60 似乎NTSC 4.43 相同。数据手册还提到 NTSC 44，它似乎NTSC 4.43 相同NTSC Comb 似乎是一种解码器模式，其中解码器使用梳状滤波器来分离色度与亮度，而不是使用延迟线
但我始终没能确切弄清 NTSC Comb 是什么
#### Philips saa7111 TV 解码
- 1997 年推出，用于 BUZ，且
- 可处理：PAL B/G/H/I、PAL N、PAL M、NTSC M、NTSC N、NTSC 4.43 SECAM

#### Philips saa7110a TV 解码
- 1995 年推出，用于 Pinnacle/Miro DC10（新）、DC10+，且
- 可处理：PAL B/G、NTSC M SECAM

#### Philips saa7114 TV 解码
- 2000 年推出，用于 LML33R10，且
- 可处理：PAL B/G/D/H/I/N、PAL N、PAL M、NTSC M、NTSC 4.43 SECAM

#### Brooktree bt819 TV 解码
- 1996 年推出，用于 LML33，且
- 可处理：PAL B/D/G/H/I、NTSC M

#### Micronas vpx3220a TV 解码
- 1996 年推出，用于 DC30 DC30+，且
- 可处理：PAL B/G/H/I、PAL N、PAL M、NTSC M、NTSC 44、PAL 60、SECAM、NTSC Comb

#### Samsung ks0127 TV 解码
- 用于 AVS6EYES 卡，- 可处理：NTSC-M/N/44、PAL-M/N/B/G/H/I/D/K/L SECAM


### TV 编码器能做什么、不能做什
TV 编码器做与解码器“相同”的事，但方向相反。你向它输入数字数据，它生成复合SVHS 信号。关于彩色制式与电视制式的信息，请参TV 解码器一节
#### Philips saa7185 TV 编码
- 1996 年推出，用于 BUZ
- 可生成：PAL B/G、NTSC M

#### Brooktree bt856 TV 编码
- 1994 年推出，用于 LML33
- 可生成：PAL B/D/G/H/I/N、PAL M、NTSC M、PAL-N（阿根廷
#### Analog Devices adv7170 TV 编码
- 2000 年推出，用于 LML300R10
- 可生成：PAL B/D/G/H/I/N、PAL M、NTSC M、PAL 60

#### Analog Devices adv7175 TV 编码
- 1996 年推出，用于 DC10、DC10+、DC10 旧、DC30、DC30+
- 可生成：PAL B/D/G/H/I/N、PAL M、NTSC M

#### ITT mse3000 TV 编码
- 1991 年推出，用于 DC10 - 可生成：PAL、NTSC、SECAM

#### Conexant bt866 TV 编码
- 用于 AVS6EYES，且
- 可生成：NTSC/PAL、PAL-M、PAL-N

adv717x 应当能够生成 PAL N。但你在寄存器中找不到任PAL N 特有的内容。看来你必须复用其它标准来生PAL N，如果使PAL M 的设置，也许能行
### 如何让这东西正常工作

加载 zr36067.o。如果它无法自动检测你的卡，使card=X 这个 insmod 选项，其X 为上一节给出的卡号。要拥有多于一卡，使用 card=X1[,X2[,X3[,X4[..]]]]

要自动化这一点，将以下内容添加到你的 /etc/modprobe.d/zoran.conf
options zr36067 card=X1[,X2[,X3[,X4[..]]]]
alias char-major-81-0 zr36067

需要记住的一点是，这本身还不会加zr36067.o，它只是把加载自动化了。如果你开始使xawtv，在某些系统上设备不会加载，因为你正尝试以用户身份加载模块，这是不允许的（“permission denied”）。一个快速的变通方法是：当你默认使X 时，XF86Config-4 添加 'Load "v4l"'；如果你不使X，则在某个启动脚本（通常rc.local）中运行 'v4l-conf -c <device>'。这两种做法都能确保模块在启动时root 账户加载
### 我该用哪块主板（或为何我的卡不工作）

<在此插入蹩脚的免责声。简而言之：SiS/Intel，差=VIA
经验告诉我们，拥Buz 的人平均比拥DC10+/LML33 的用户遇到更多问题。经验还告诉我们，拥有基VIA 的主板（ktXXX、MVP3）的人比拥有基于其它芯片组主板的人遇到更多问题。以下是 Andrew Stevens 的一些笔记：

以下是我在各种主板上使用 LML33 Buz 的经验：

- VIA MVP3
 - 算了吧。毫无意义。无法工作- Intel 430FX（Pentium 200 - LML33 完美，Buz 勉强可用（每部影片丢 3 4 帧）
- Intel 440BX（早期步进）
 - LML33 勉强可用。Buz 开始变得恼人（每小6-10 帧）
- Intel 440BX（晚期步进）
 - Buz 勉强可用，LML33 几乎完美（偶尔丢单帧- SiS735
 - LML33 完美，Buz 勉强可用- VIA KT133(*)
 - LML33 开始变得恼人，Buz 差到让我放弃
- 两块 440BX 主板都是CPU 版本
Bernhard Praschinger 后来补充
- AMD 751
 - Buz 完美到勉强可- AMD 760
 - Buz 完美到勉强可
总的来说，如果你拥有基于 VIA 的主板，用户邮件列表上的人不会给你多少机会。它们可能便宜，但有时你宁愿在更好的板子上多花些钱。总的来说，与其它主板相比，VIA 主板IDE/PCI 性能也会差得很惨。你会注意到概览中完全没有提DC10+/DC30+。基本上，你可以假设：如Buz 能工作，LML33 也能工作；如LML33 能工作，DC10+/DC30+ 也能工作。在受支持的所有卡中，它们对不同主板芯片组最为宽容
如果在采集过程中遇到超时，买一块更好的主板，或在采集时降低质量/缓冲区大小（参见“关于缓冲区大小、质量、输出尺寸等”）。如果它挂起，目前我们几乎无能为力。检查你IRQ，并确保该卡拥有自己的中断
### 编程接口

本驱动符video4linux2。对 V4L1 以及自定zoran ioctl 的支持已在内2.6.38 中移除
关于编程示例，请查看 MJPEG-tools（http://mjpeg.sf.net/）中lavrec.c lavplay.c 代码
给软件开发者的额外说明
   驱动根据当前电视标准（norm）返maxwidth maxheight 参数。因此，与驱动通信并“询问”这些参数的软件应当首先设置正确norm。这看上去在逻辑上是正确的：相对于可能以 ITU 或方像素格式工作的各种电视采集卡的几何设置，电视标准对当前国家而言“更为恒定”
### 应用程序

已知可与该驱动配合工作的应用程序
电视观看
- xawtv
- kwintv
- 可能任何支持 video4linux video4linux2 的电视应用程序
MJPEG 采集/播放
- mjpegtools/lavtools（或 Linux Video Studio- gstreamer
- mplayer

通用原始采集
- xawtv
- gstreamer
- 可能任何支持 video4linux video4linux2 的应用程
视频编辑
- Cinelerra
- MainActor
- mjpegtools（或 Linux Video Studio

### 关于缓冲区大小、质量、输出尺寸等

zr36060 可以进行 1:2 JPEG 压缩。这确实是该芯片组能达到的理论最大值。不过，驱动可以将压缩限制为最大（尺寸:4。原因在于某些卡（例Buz）在只压缩到 1:2 时，若不进行 1:4 压缩，仅几分钟后就会停止采集。使1:4 时大多可以正常工作。如果你Buz，使'low_bitrate=1' 进入 1:4 最大压缩模式
因此00% JPEG 质量在实践中就是 1:2 压缩。对于一整帧 PAL 画面（尺720x576）而言。JPEG 场以 YUY2 格式存储，因此场的大小为 720x288x16/2 场（2 帧）= 207360 字节/x 2 = 414720 字节/帧（再加上一些用于头部以DHT（huffmanDQT（量化）表的字节:2 压缩时每帧大约会达到 512kB）。对1:4 压缩，帧的大小为此的一半
Martin Samuelsson 给出的一些额外解释，也说明了缓冲区大小的重要性：
--
> Hmm，我不认为真的是那样。用当前（周一 18:00 下载）的驱动，我得到 10 秒的输出大小> -q 50 -b 128 : 24.283.332 字节
> -q 50 -b 256 : 48.442.368
> -q 25 -b 128 : 24.655.992
> -q 25 -b 256 : 25.859.820

我醒了，而且再也睡不着。我消磨点时间解释一下为什么这对我来说并不奇怪
让我们用 704 像素的宽度做点算术。我不确Buz 是否真的用这个数字，但现在这不重要
704x288 像素，一个场，是 202752 像素。除以每64 像素；每3168 块。每个像素由两个字节组成；每128 字节；每1024 位。新驱动中的 100% 意味着 1:2 压缩；最大输出变为每512 位。实际上510，但 512 用于计算更简单
假设我们指定 d1q50。因此我们期望每256 位；乘以 3168 得到 811008 位；每场 101376 字节。我们这里谈论的是原始位与字节，所以不需要对每像素位数之类做任何花哨的校正。每101376 字节
d1 视频每帧包含两个场。它们合计为每帧 202752 字节，其中一个帧进入每个缓冲区
但是等一下！-b128 给出 128kB 缓冲区！不可能把 202752 字节JPEG 数据塞进 128kB
这正是驱动在你的示例中注意到并自动补偿的地方。让我们用这些信息做点算术：

128kB 131072 字节。在这个缓冲区中，我们要存储两个场，因此每个场剩65536 字节。每场使3168 块，我们得到每块 20.68686868…可用字节65 位。当只有 165 位可用时，我们不能允许每256 位的请求q50 选项被静默覆盖，-b128 选项优先，剩下相当于 -q32 的结果
这给了我们每165 位的数据率，乘以 3168，合计每65340 字节，在允许65536 之内。当前驱动还有另一层速率限制；它不会接受填满超过指定缓冲6/8 -q 值。（我不确定为什么。“稳妥起见”似乎是个安全的猜测。就我个人而言，我想我会把请求的每块位数减一，或类似的做法。）我们不能使用每块 165 位，而必须再次降低，降到可用缓冲区空间的 6/8：我们最终得到每124 位，相当-q24。使128kB 缓冲区时，在 -d1 下你不能使用大于 -q24 的值。（以及 PAL，以704 像素宽度……）

第三个示例通过相同过程被限制到 -q24。第二个示例，用非常相似的计算，被限制到 -q48。唯一真正以指-q 值采集的示例是最后一个，从文件大小可以清楚地看出--

结论：最终影片的质量取决于缓冲区大小、质量，以及你是否使'low_bitrate=1' 作为 zr36060.c 模块insmod 选项来进1:4 而非 1:2 的压缩，等等
如果你遇到超时，降低质量/缓冲区大小或使用 'low_bitrate=1' 作为 zr36060.o insmod 选项，实际上可能有所帮助，Buz 已经证明了这一点
### 它挂崩溃/失败/各种问题！救命！

确保该卡拥有自己的中断（参见 /proc/interrupts），以高详细程度检dmesg 的输出（debug=2 加载 zr36067.o，以 debug=1 加载所有其它模块）。检查你的主板是否有利（见问2），若不利，在另一台计算机中测试该卡。另请参阅问3 中给出的笔记，如果录制在一段时间后失败，尝试降低质缓冲区大采集大小
如果这一切都没有帮助，请清晰地描述问题，包括详细的硬件信息（内存+品牌、主芯片品牌、是哪块 MJPEG 卡、处理器、其它可能相关的 PCI 卡），给出系统的 PnP 信息proc/interruptsproc/dmaproc/devices），并给出内核版本、驱动版本、glibc 版本、gcc 版本以及任何其它可能相关的信息。同时提供高详细程度dmesg 输出。关于如何联系开发者，请参阅“联系方式”
### 维护联系方式

本驱动以往的贡献开发者有
- Laurent Pinchart <laurent.pinchart@skynet.be>
- Ronald Bultje rbultje@ronald.bitfreak.net
- Serguei Miridonov <mirsev@cicese.mx>
- Wolfgang Scherr <scherr@net4you.net>
- Dave Perks <dperks@ibm.net>
- Rainer Johanni <Rainer@Johanni.de>

### 驱动许可
    This driver is distributed under the terms of the General Public License.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

详见 http://www.gnu.org/ 获取更多信息