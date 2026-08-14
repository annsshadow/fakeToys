
## bttv 驱动


### bttv 发行说明


```
    ./scripts/config -e PCI
    ./scripts/config -m I2C
    ./scripts/config -m INPUT
    ./scripts/config -m MEDIA_SUPPORT
    ./scripts/config -e MEDIA_PCI_SUPPORT
    ./scripts/config -e MEDIA_ANALOG_TV_SUPPORT
    ./scripts/config -e MEDIA_DIGITAL_TV_SUPPORT
    ./scripts/config -e MEDIA_RADIO_SUPPORT
    ./scripts/config -e RC_CORE
    ./scripts/config -m VIDEO_BT848

```
```
    ./scripts/config -m DVB_BT8XX

```
在这种情况下，请参阅 Documentation/admin-guide/media/bt8xx.rst 获取额外说明。

### 让你的卡使用 bttv


如果你已经编译并安装了 bttv，只需启动内核就应当足以让它尝试探测。不过，取决于型号，内核可能需要关于硬件的额外信息，因为设备可能无法直接向内核提供此类信息。

如果 bttv 没能工作，很可能是它无法自动探测到你的卡，需要一些 insmod 选项。bttv 最重要的 insmod 选项是 "card=n"，用于选择正确的卡类型。如果你得到了视频但没有声音，你很可能指定了错误（或没有指定）的卡类型。受支持卡的列表在 Documentation/admin-guide/media/bttv-cardlist.rst。

如果 bttv 加载非常慢（在使用没有调谐器的廉价卡时有时会发生），尝试把以下内容加入你的模块配置文件（通常它是 `/etc/modules.conf` 或 `/etc/modules-load.d/` 下的某个文件，但具体位置取决于你的发行版）：

```
	options i2c-algo-bit bit_test=1

```
某些卡可能需要一个额外的固件文件才能工作。例如，对于 WinTV/PVR，你需要其驱动 CD 中的一个名为 `hcwamc.rbf` 的固件文件。它在一个名为 `pvr45xxx.exe` 的自解压 zip 文件中。只需把它放在 `/etc/firmware` 目录下，就足以让它在驱动探测模式（例如内核启动时或手动通过 `modprobe` 命令加载驱动时）被自动加载。

如果你的卡没有列在 Documentation/admin-guide/media/bttv-cardlist.rst，或者你在让音频工作时遇到困难，请阅读 still_doesnt_work。

### 自动探测卡


bttv 使用 PCI 子系统 ID 来自动探测卡类型。lspci 在第二行列出子系统 ID，看起来像这样：

```
	00:0a.0 Multimedia video controller: Brooktree Corporation Bt878 (rev 02)
		Subsystem: Hauppauge computer works Inc. WinTV/GO
		Flags: bus master, medium devsel, latency 32, IRQ 5
		Memory at e2000000 (32-bit, prefetchable) [size=4K]

```
只有基于 bt878 的卡可以有子系统 ID（这并不意味着每张卡真的都有）。bt848 卡不能有子系统 ID，因此无法被自动探测。在 Documentation/admin-guide/media/bttv-cardlist.rst 有一份带有这些 ID 的列表（如果你感兴趣或想发补丁更新）。


### 仍然不工作？


我家里没有摆着 30 多块不同采集板的实验室，也没有 PAL/NTSC/SECAM 测试信号发生器，所以我常常无法复现你的问题。这使得调试对我来说非常困难。

如果你有一些知识和空闲时间，请尝试自己修复（当然非常欢迎补丁……）你知道的：Linux 的口号是 "Do it yourself"（自己动手）。

有一个邮件列表在
http://vger.kernel.org/vger-lists.html#linux-media

如果你在某块特定电视卡上遇到麻烦，请尝试在那里提问，而不是直接给我发邮件。那里有使用相同卡的人听到的可能性要高得多……

对于声音问题：世界各地用于电视声音的系统有很多不同种类。并且也有不同的芯片来解码音频信号。关于声音问题的报告（"立体声不工作"）除非你包含一些关于你的硬件以及你所在国家使用的电视声音制式的细节（或至少是你所居住的国家），否则基本没有用处。

### Modprobe 选项



   The following argument list can be outdated, as we might add more
   options if ever needed. In case of doubt, please check with
   `modinfo <module>`.

   该命令打印关于内核模块的各种信息，其中包括一份完整且最新的 insmod 选项列表。

   This command prints various information about a kernel
   module, among them a complete and up-to-date list of insmod options.


bttv
	bt848/878（采集芯片）驱动

```
	    card=n		card type, see CARDLIST for a list.
	    tuner=n		tuner type, see CARDLIST for a list.
	    radio=0/1	card supports radio
	    pll=0/1/2	pll settings

			    0: don't use PLL
			    1: 28 MHz crystal installed
			    2: 35 MHz crystal installed

	    triton1=0/1     for Triton1 (+others) compatibility
	    vsfx=0/1	yet another chipset bug compatibility bit
			    see README.quirks for details on these two.

	    bigendian=n	Set the endianness of the gfx framebuffer.
			    Default is native endian.
	    fieldnr=0/1	Count fields.  Some TV descrambling software
			    needs this, for others it only generates
			    50 useless IRQs/sec.  default is 0 (off).
	    autoload=0/1	autoload helper modules (tuner, audio).
			    default is 1 (on).
	    bttv_verbose=0/1/2  verbose level (at insmod time, while
			    looking at the hardware).  default is 1.
	    bttv_debug=0/1	debug messages (for capture).
			    default is 0 (off).
	    irq_debug=0/1	irq handler debug messages.
			    default is 0 (off).
	    gbuffers=2-32	number of capture buffers for mmap'ed capture.
			    default is 4.
	    gbufsize=	size of capture buffers. default and
			    maximum value is 0x208000 (~2MB)
	    no_overlay=0	Enable overlay on broken hardware.  There
			    are some chipsets (SIS for example) which
			    are known to have problems with the PCI DMA
			    push used by bttv.  bttv will disable overlay
			    by default on this hardware to avoid crashes.
			    With this insmod option you can override this.
	    no_overlay=1	Disable overlay. It should be used by broken
			    hardware that doesn't support PCI2PCI direct
			    transfers.
	    automute=0/1	Automatically mutes the sound if there is
			    no TV signal, on by default.  You might try
			    to disable this if you have bad input signal
			    quality which leading to unwanted sound
			    dropouts.
	    chroma_agc=0/1	AGC of chroma signal, off by default.
	    adc_crush=0/1	Luminance ADC crush, on by default.
	    i2c_udelay=     Allow reduce I2C speed. Default is 5 usecs
			    (meaning 66,67 Kbps). The default is the
			    maximum supported speed by kernel bitbang
			    algorithm. You may use lower numbers, if I2C
			    messages are lost (16 is known to work on
			    all supported cards).

	    bttv_gpio=0/1
	    gpiomask=
	    audioall=
	    audiomux=
			    See Sound-FAQ for a detailed description.

	remap, card, radio and pll accept up to four comma-separated arguments
	(for multiple boards).

```
tuner
	调谐器驱动。除非你只想配合摄像头使用，或者板子不提供模拟电视调谐，否则你需要它。

```
		debug=1		print some debug info to the syslog
		type=n		type of the tuner chip. n as follows:
				see CARDLIST for a complete list.
		pal=[bdgil]	select PAL variant (used for some tuners
				only, important for the audio carrier).

```
tvaudio
	为所有简单的 i2c 音频控制芯片（tda/tea*）提供单一驱动。

```
		tda8425  = 1	enable/disable the support for the
		tda9840  = 1	various chips.
		tda9850  = 1	The tea6300 can't be autodetected and is
		tda9855  = 1	therefore off by default, if you have
		tda9873  = 1	this one on your card (STB uses these)
		tda9874a = 1	you have to enable it explicitly.
		tea6300  = 0	The two tda985x chips use the same i2c
		tea6420  = 1	address and can't be disturgished from
		pic16c54 = 1	each other, you might have to disable
				the wrong one.
		debug = 1	print debug messages

```
msp3400
	msp34xx 声音处理器芯片的驱动。如果你有一块立体声卡，你可能想 insmod 这个。

```
		debug=1/2	print some debug info to the syslog,
				2 is more verbose.
		simple=1	Use the "short programming" method.  Newer
				msp34xx versions support this.  You need this
				for dbx stereo.  Default is on if supported by
				the chip.
		once=1		Don't check the TV-stations Audio mode
				every few seconds, but only once after
				channel switches.
		amsound=1	Audio carrier is AM/NICAM at 6.5 Mhz.  This
				should improve things for french people, the
				carrier autoscan seems to work with FM only...

```
### 如果机器在 bttv 下硬冻结


它可能是 bttv 驱动的 bug，也可能是有问题的硬件，也可能是别的原因……

仅仅给我发一封 "bttv freezes"（bttv 冻结）的邮件是帮不上什么忙的。这个 README 中有一些提示，可以帮你定位问题。


#### bttv bug


如果某个版本工作而另一个不工作，那很可能是驱动 bug。如果你能说出它确切在哪里坏掉的（即最后一个能工作的版本和第一个坏掉的版本），会非常有帮助。

对于硬冻结，你大概不会在日志文件中找到任何东西。捕获任何内核消息的唯一方法是接上一个串口控制台，并让某个终端应用程序记录消息。/me 使用 screen。关于设置串口控制台的细节，请参阅 Documentation/admin-guide/serial-console.rst。

阅读 Documentation/admin-guide/bug-hunting.rst，了解如何从内核在保护故障（所谓 "kernel oops"）时打印的寄存器+栈转储中获取任何有用的信息。

如果你遇到某种死锁，可以尝试用 sysrq-t 转储每个进程的调用栈跟踪（参见 Documentation/admin-guide/sysrq.rst）。这样就能弄清楚处于 "D" 状态的进程 **确切** 卡在哪里。

我见过这样的报告：对某些人来说 bttv 0.7.x 崩溃，而 0.8.x 工作得非常稳定。因此大概是 bttv 0.7.x 中某处还残留着一个小的 buglet。我不知道确切在哪里，它对我和许多其他人都稳定工作。但如果你在 0.7.x 版本上遇到问题，可以尝试一下 0.8.x……


#### 硬件 bug


某些硬件无法处理 PCI-PCI 传输（即采集器 => vga）。有时问题就因为 PCI 总线上的高负载而出现在 bttv 上。bt848/878 芯片对已知的兼容性问题有几个变通方法，参见 README.quirks。

有些人报告说提高 PCI latency（延迟）也有帮助，虽然我不确定这到底是真正修复了问题，还是只是让它不太可能发生。bttv 和 btaudio 都有一个 insmod 选项来设置设备的 PCI 延迟。

某些主板在正确处理多个设备同时进行 DMA 时有问题。bttv + ide 有时会导致这种情况，如果是这样，你大概只会在视频和硬盘访问同时进行时看到冻结。更新 IDE 驱动以获取针对硬件 bug 的最新最全的变通方法，可能会修复这些问题。


#### 其他


如果你使用了某些仅二进制的东西（比如 nvidia 模块），尝试在不使用它的情况下复现问题。

IRQ 共享在某些情况下已知会引起问题。理论上和在许多配置中它工作得很好。不过，值得一试去重新摆放 PCI 卡，给 bttv 另一个 IRQ，或者让它和别的硬件共享 IRQ。与 VGA 卡共享 IRQ 有时似乎会带来麻烦。我也见过 bttv 与 ACPI 桥（以及启用了 apci 的内核）共享 IRQ 时的奇怪现象。

### Bttv 兼容性（quirks）


下面是 bt878 数据手册关于 bt878 芯片 PCI bug 兼容模式的说明。

triton1 insmod 选项设置控制寄存器中的 EN_TBFX 位。vsfx insmod 选项对 EN_VSFX 位做同样的事情。如果你有稳定性问题，可以尝试其中一个选项是否能让你的机器稳定工作。

drivers/pci/quirks.c 了解这些问题，这样这些位就会为已知的有 bug 芯片组自动启用（查看内核消息，bttv 会告诉你）。

#### 普通 PCI 模式


PCI REQ 信号是对输入的功能请求的逻辑或（logical-or）。内部的 GNT[0:1] 信号与 GNT 异步选通，并由音频请求信号解复用。因此仲裁器在加电时默认为视频功能，并在没有总线访问请求时停在那里。这是可取的，因为视频会更频繁地请求总线。不过，音频将拥有最高的总线访问优先级。因此，即使音频在视频请求之后、但在 PCI 外部仲裁器授予对 Bt879 的访问之前发出请求，音频也将首先获得总线访问权。一旦某个功能上了总线，另一个功能就无法抢占它。把整个视频 PCI FIFO 倒空到 PCI 总线上的时间，相对于音频 PCI FIFO 所能容忍的总线访问延迟来说非常短。


#### 430FX 兼容模式


使用 430FX PCI 时，以下规则将确保兼容性：

 (1) 在断言 FRAME 的同时撤销（deassert）REQ。
 (2) 在结束前一个事务之前，不要重新断言 REQ 以请求另一个总线事务。

由于各个总线主控不能直接控制 REQ，视频和音频请求简单的逻辑或会违反规则。因此，仲裁器和发起方都包含 430FX 兼容模式逻辑。要启用 430FX 模式，请按第 104 页设备控制寄存器中的指示设置 EN_TBFX 位。

当启用 EN_TBFX 时，仲裁器确保满足这两个兼容规则。在 PCI 仲裁器断言 GNT 之前，这个内部仲裁器仍然可以将两个请求逻辑或起来。然而，一旦 GNT 被发出，这个仲裁器必须锁定它的决定，现在只把被授予的请求路由到 REQ 引脚。仲裁器决定锁定不管 FRAME 的状态如何都会发生，因为它不知道 FRAME 何时会被断言（典型情况是——每个发起方会在 GNT 之后的周期断言 FRAME）。当 FRAME 被断言时，移除其请求是发起方的责任。允许这个请求流经到 REQ 而不允许另一个请求保持 REQ 被断言，是仲裁器的责任。决定锁定可以在事务结束时解除：例如，当总线空闲时（FRAME 和 IRDY）。然后仲裁器决定可以继续异步进行，直到 GNT 再次被断言。


#### 与不符合 PCI 2.1 的核心逻辑接口


一小部分核心逻辑设备可能在 GNT 被撤销的同一周期启动一个总线事务。这不符合 PCI 2.1。为确保与使用这些 PCI 控制器的 PC 兼容，必须启用 EN_VSFX 位（参见第 104 页设备控制寄存器）。在这种模式下，仲裁器不会把 GNT 传递给内部功能，除非 REQ 被断言。这防止了总线事务在 GNT 被撤销的同一周期启动。这也有一个副作用，即无法利用总线停放（bus parking），从而降低了仲裁性能。Bt879 驱动必须查询这些不兼容的设备，并且仅在需要时设置 EN_VSFX 位。


#### tvcards 数组的其他元素


如果你正试图让一张新卡工作，你可能会发现查看以下内容很有用：

```
	video_inputs    - # of video inputs the card has
	audio_inputs    - historical cruft, not used any more.
	tuner           - which input is the tuner
	svhs            - which input is svhs (all others are labeled composite)
	muxsel          - video mux, input->registervalue mapping
	pll             - same as pll= insmod option
	tuner_type      - same as tuner= insmod option
	*_modulename    - hint whenever some card needs this or that audio
			module loaded to work properly.
	has_radio	- whenever this TV card has a radio tuner.
	no_msp34xx	- "1" disables loading of msp3400.o module
	no_tda9875	- "1" disables loading of tda9875.o module
	needs_tvaudio	- set to "1" to load tvaudio.o module

```
如果某个配置项同时从 tvcards 数组和 insmod 选项指定，则 insmod 选项优先。

### 卡



   For a more updated list, please check
   https://linuxtv.org/wiki/index.php/Hardware_Device_Information

#### 受支持的卡：Bt848/Bt848a/Bt849/Bt878/Bt879 卡


所有带有 Bt848/Bt848a/Bt849/Bt878/Bt879 以及普通 Composite/S-VHS 输入的卡都受支持。通过软件中的 VBI 采样解码，所有卡都支持图文电视（Teletext）和 Intercast（仅 PAL）。

某些带有额外输入复用或其他花哨芯片的卡只得到部分支持（除非卡制造商提供了规格说明）。当一张卡列在这里时，它不一定被完全支持。

所有其他卡只是通过调谐器、声音解码器、EEPROM、图文电视解码器等额外组件而不同。


#### MATRIX Vision


MV-Delta
- Bt848A
- 4 个 Composite 输入，1 个 S-VHS 输入（与第 4 个 composite 共享）
- EEPROM

http://www.matrix-vision.de/

这张卡没有调谐器，但支持 Bt848A 的全部 4 个 composite（其中 1 个与 S-VHS 输入共享）。如果你只有卫星电视、但有多个调谐器通过 composite 连到卡上，这是一张非常不错的卡。

非常感谢 Matrix-Vision 免费给了我们 2 张卡，使得 Bt848a/Bt849 单晶振操作支持成为可能！！！


#### Miro/Pinnacle PCTV


- Bt848
  有些（全部？？）带 2 个晶振，用于 PAL/SECAM 和 NTSC
- PAL、SECAM 或 NTSC 电视调谐器（Philips 或 TEMIC）
- MSP34xx 声音解码器在附加板上
  解码器受支持，但据我所知还不能用
  （GPIO 端口中需要其他声音 MUX 设置？？？有人修复了这个问题吗？？？）
- 1 个调谐器，1 个 composite 和 1 个 S-VHS 输入
- 调谐器类型自动探测

http://www.miro.de/
http://www.miro.com/


非常感谢这张免费卡，使 1997 年的首个 NTSC 支持成为可能！


#### Hauppauge Win/TV pci


有许多不同版本的 Hauppauge 卡，带有不同的调谐器（TV+Radio……）、图文电视解码器。注意，即使型号编号相同的卡（取决于修订版本）上面的芯片也不同。

- Bt848（以及其他，但总是以 2 晶振操作？？？）
  较新的卡有 Bt878

- PAL、SECAM、NTSC 调谐器，带或不带 Radio 支持

例如：

- PAL:

  - TDA5737: VHF、超高频带（hyperband）和 UHF 混频器/振荡器，用于 TV 和 VCR 3 频段调谐器
  - TSA5522: 1.4 GHz I2C 总线控制合成器，I2C 0xc2-0xc3

- NTSC:

  - TDA5731: VHF、超高频带和 UHF 混频器/振荡器，用于 TV 和 VCR 3 频段调谐器
  - TSA5518: Philips 站点上没有数据手册可用

- Philips SAA5246 或 SAA5284（或无）图文电视解码器芯片
  带缓冲 RAM（例如 Winbond W24257AS-35: 32Kx8 CMOS 静态 RAM）
  SAA5246（I2C 0x22）受支持

- 256 字节 EEPROM: Microchip 24LC02B 或 Philips 8582E2Y
  带有配置信息
  I2C 地址 0xa0（24LC02B 也响应 0xa2-0xaf）

- 1 个调谐器，1 个 composite 和（取决于型号）1 个 S-VHS 输入

- 14052B: 用于选择声音源的复用器（mux）

- 声音解码器: TDA9800、MSP34xx（立体声卡）


#### Askey CPH 系列


由 TelSignal(?) 开发，由许多厂商 OEM（Typhoon、Anubis、Dynalink）

- 卡系列:
  - CPH01x: BT848 仅采集
  - CPH03x: BT848
  - CPH05x: BT878 带 FM
  - CPH06x: BT878（无 FM）
  - CPH07x: BT878 仅采集

- 电视标准:
  - CPH0x0: NTSC-M/M
  - CPH0x1: PAL-B/G
  - CPH0x2: PAL-I/I
  - CPH0x3: PAL-D/K
  - CPH0x4: SECAM-L/L
  - CPH0x5: SECAM-B/G
  - CPH0x6: SECAM-D/K
  - CPH0x7: PAL-N/N
  - CPH0x8: PAL-B/H
  - CPH0x9: PAL-M/M

- CPH03x 常作为 "TV capturer" 出售。

识别:

  #) 878 卡可以通过 PCI 子系统 ID 识别:
     - 144f:3000 = CPH06x
     - 144F:3002 = CPH05x w/ FM
     - 144F:3005 = CPH06x_LC（无遥控）
  #) 卡背面有一个带 "CPH" 型号的贴纸。
  #) 这些卡在调谐器金属盒正上方的 PCB 上印有一个数字:
     - "80-CP2000300-x" = CPH03X
     - "80-CP2000500-x" = CPH05X
     - "80-CP2000600-x" = CPH06X / CPH06x_LC

  Askey 把这些卡作为 "Magic TView series" 出售，品牌为 "MagicXpress"。
  其他 OEM 常称这些为 "Tview"、"TView99" 等。

#### Lifeview Flyvideo 系列:


这些系列的命名随时间与地域而不同。

识别:
  #) 某些型号可以通过 PCI 子系统 ID 识别:

     - 1852:1852 = Flyvideo 98 FM
     - 1851:1850 = Flyvideo 98
     - 1851:1851 = Flyvideo 98 EZ（仅采集）

  #) PCB 上有一个印字:

     - LR25       = Flyvideo（Zoran ZR36120, SAA7110A）
     - LR26 Rev.N = Flyvideo II（Bt848）
     - LR26 Rev.O = Flyvideo II（Bt878）
     - LR37 Rev.C = Flyvideo EZ（仅采集, ZR36120 + SAA7110）
     - LR38 Rev.A1= Flyvideo II EZ（Bt848 仅采集）
     - LR50 Rev.Q = Flyvideo 98（带 eeprom 和 PCI 子系统 ID）
     - LR50 Rev.W = Flyvideo 98（无 eeprom）
     - LR51 Rev.E = Flyvideo 98 EZ（仅采集）
     - LR90       = Flyvideo 2000（Bt878）
     - LR90 Flyvideo 2000S（Bt878）带立体声 TV（包装含 LR91 子板）
     - LR91       = LR90 的立体声子卡
     - LR97       = Flyvideo DVBS
     - LR99 Rev.E = 用于 OEM 集成的薄型卡（仅内部音频！）bt878
     - LR136	 = Flyvideo 2100/3100（薄型, SAA7130/SAA7134）
     - LR137      = Flyvideo DV2000/DV3000（SAA7130/SAA7134 + IEEE1394）
     - LR138 Rev.C= Flyvideo 2000（SAA7130）
     - LR138 Flyvideo 3000（SAA7134）带立体声 TV

 - 这些存在带 FM 和带 Remote 的变体，有时用后缀 "FM" 和 "R" 表示。

  #) 你有一台笔记本（miniPCI 卡）:

      - Product    = FlyTV Platinum Mini
      - Model/Chip = LR212/saa7135

      - Lifeview.com.tw 说明（2002 年 2 月）:
        "FlyVideo2000 和 FlyVideo2000s 产品名已重命名为 FlyVideo98。"
        它们的 Bt8x8 卡被列为已停产。
      - Flyvideo 2000S 在某些国家（欧洲？）可能作为 Flyvideo 3000 出售。
        新的 Flyvideo 2000/3000 是基于 SAA7130/SAA7134 的。

"Flyvideo II" 曾是 848 卡的名称，如今（在德国）这个名字被重新用于 LR50 Rev.W。

Lifeview 网站曾在某些时候提到 Flyvideo III，但这样的卡尚未见过（也许它是 LR90 [立体声] 的德文名）。这些卡也被许多 OEM 出售。

FlyVideo A2（Elta 8680）= LR90 Rev.F（带 Remote，无 FM，立体声 TV 由 tda9821）{德国}

Lifeview 3000（Elta 8681）按 Plus（2002 年 4 月，德国）出售 = LR138 w/ saa7134

##### lifeview 在 gpio 引脚 0-9 上的配置编码


- LR50 rev. Q（"PARTS: 7031505116），调谐器被识别为 Nr. 5，输入
  SVideo、TV、Composite、Audio、Remote:

 - CP9..1=100001001（1: 0 欧姆电阻未焊接到 GND；0: 已焊接）


#### Typhoon 电视卡系列:


这些可以是 CPH、Flyvideo、Pixelview 或 KNC1 系列。

Typhoon 是 Anubis 的品牌。

型号 50680 被重新使用，某些型号编号随时间有不同内容。

型号:

  - 50680 "TV Tuner PCI Pal BG"（旧，红色包装）= 可以是 CPH03x(bt848) 或 CPH06x(bt878)
  - 50680 "TV Tuner Pal BG"（蓝色包装）= Pixelview PV-BT878P+（Rev 9B）
  - 50681 "TV Tuner PCI Pal I"（50680 的变体）
  - 50682 "TView TV/FM Tuner Pal BG"       = Flyvideo 98FM（LR50 Rev.Q）

```
	 The package has a picture of CPH05x (which would be a real TView)

  - 50683 "TV Tuner PCI SECAM"（50680 的变体）
  - 50684 "TV Tuner Pal BG"                = Pixelview 878TV(Rev.3D)
  - 50686 "TV Tuner"                       = KNC1 TV Station
  - 50687 "TV Tuner stereo"                = KNC1 TV Station pro
  - 50688 "TV Tuner RDS"（黑色包装）   = KNC1 TV Station RDS
  - 50689  TV SAT DVB-S CARD CI PCI (SAA7146AH, SU1278?) = "KNC1 TV Station DVB-S"
  - 50692 "TV/FM Tuner"（小 PCB）
  - 50694  TV TUNER CARD RDS（PHILIPS CHIPSET SAA7134HL）
  - 50696  TV TUNER STEREO（PHILIPS CHIPSET SAA7134HL, MK3ME Tuner）
  - 50804  PC-SAT TV/Audio Karte = Techni-PC-Sat（ZORAN 36120PQC, Tuner:Alps）
  - 50866  TVIEW SAT RECEIVER+ADR
  - 50868 "TV/FM Tuner Pal I"（50682 的变体）
  - 50999 "TV/FM Tuner Secam"（50682 的变体）

```
#### Guillemot


型号:

- Maxi-TV PCI（ZR36120）
- Maxi TV Video 2 = LR50 Rev.Q（FI1216MF, PAL BG+SECAM）
- Maxi TV Video 3 = CPH064（PAL BG + SECAM）

#### Mentor


Mentor TV card（"55-878TV-U1"）= Pixelview 878TV(Rev.3F)（带 FM 带 Remote）

#### Prolink


- 电视卡:

  - PixelView Play TV pro - (Model: PV-BT878P+ REV 8E)
  - PixelView Play TV pro - (Model: PV-BT878P+ REV 9D)
  - PixelView Play TV pro - (Model: PV-BT878P+ REV 4C / 8D / 10A )
  - PixelView Play TV - (Model: PV-BT848P+)
  - 878TV - (Model: PV-BT878TV)

- 多媒体电视套装（卡 + 软件包）:

  - PixelView Play TV Theater - (Model: PV-M4200) =  PixelView Play TV pro + Software
  - PixelView Play TV PAK -     (Model: PV-BT878P+ REV 4E)
  - PixelView Play TV/VCR -     (Model: PV-M3200 REV 4C / 8D / 10A )
  - PixelView Studio PAK -      (Model:    M2200 REV 4C / 8D / 10A )
  - PixelView PowerStudio PAK - (Model: PV-M3600 REV 4E)
  - PixelView DigitalVCR PAK -  (Model: PV-M2400 REV 4C / 8D / 10A )
  - PixelView PlayTV PAK II (TV/FM card + usb camera)  PV-M3800
  - PixelView PlayTV XP PV-M4700,PV-M4700(w/FM)
  - PixelView PlayTV DVR PV-M4600  包装内容:PixelView PlayTV pro, windvr & videoMail s/w

- 更多卡:

  - PV-BT878P+rev.9B（Play TV Pro, 可选带 FM 带 NICAM）
  - PV-BT878P+rev.2F
  - PV-BT878P Rev.1D (bt878, 仅采集)

  - XCapture PV-CX881P (cx23881)
  - PlayTV HD PV-CX881PL+, PV-CX881PL+(w/FM) (cx23881)

  - DTV3000 PV-DTV3000P+ DVB-S CI = Twinhan VP-1030
  - DTV2000 DVB-S = Twinhan VP-1020

- 视频会议:

  - PixelView Meeting PAK - (Model: PV-BT878P)
  - PixelView Meeting PAK Lite - (Model: PV-BT878P)
  - PixelView Meeting PAK plus - (Model: PV-BT878P+rev 4C/8D/10A)
  - PixelView Capture - (Model: PV-BT848P)
  - PixelView PlayTV USB pro
  - Model No. PV-NT1004+, PV-NT1004+ (w/FM) = NT1004 USB 解码芯片 + SAA7113 视频解码芯片

#### Dynalink


这些是 CPH 系列。

#### Phoebemicro


- TV Master    = CPH030 或 CPH060
- TV Master FM = CPH050

#### Genius/Kye


- Video Wonder/Genius Internet Video Kit = LR37 Rev.C
- Video Wonder Pro II（848 或 878）= LR26

#### Tekram


- VideoCap C205（Bt848）
- VideoCap C210（zr36120 +Philips）
- CaptureTV M200（ISA）
- CaptureTV M205（Bt848）

#### Lucky Star


- Image World Conference TV = LR50 Rev. Q

#### Leadtek


- WinView 601（Bt848）
- WinView 610（Zoran）
- WinFast2000
- WinFast2000 XP

##### 对 Leadtek WinView 601 TV/FM 的支持


本节的作者: Jon Tombs <jon@gte.esi.us.es>

这张卡基本上和所有其他卡一样（Bt484A, Philips 调谐器），主要区别是它们把可编程衰减器接到了 3 个 GPIO 线上，以提供一些音量控制。它们还在板上装了一个红外遥控解码器，等有时间我会加上对它的支持（它很简单，每次按键产生一个中断，键码放在 GPIO 端口中）。

我还没有任何应用程序来测试收音机支持。调谐器频率设置应当能用，但音频复用器可能是错的。如果它不工作，给我发邮件。

- 不感谢 Leadtek，他们拒绝回答任何关于其硬件的问题。这个驱动是通过目视检查卡写出来的。如果你使用这个驱动，给他们发一封辱骂邮件，告诉他们除非他们支持 Linux，否则你不会再购买他们的硬件。

- 略微感谢普林斯顿科技（Princeton Technology Corp，http://www.princeton.com.tw），他们制造了音频衰减器。他们网站上公开可用的数据手册不包含芯片编程信息！他们服务器上藏着完整的数据手册，但别问我是怎么找到的。

要使用这个驱动，我使用以下选项，调谐器和 pll 设置可能在你的国家不同。你可以通过 modprobe 参数强制设置。

```
    modprobe bttv  tuner=1 pll=28 radio=1 card=17

```
设置调谐器类型 1（Philips PAL_I），带 28 MHz 晶振的 PLL，启用 FM 收音机，并选择 bttv 卡 ID 17（Leadtek WinView 601）。


#### KNC One


- TV-Station
- TV-Station SE（+软件包）
- TV-Station pro（+电视立体声）
- TV-Station FM（+收音机）
- TV-Station RDS（+RDS）
- TV Station SAT（模拟卫星）
- TV-Station DVB-S


#### Provideo


- PV951 或 PV-951，现在命名为 PV-951T
   （也作为以下名称出售:
   Boeder TV-FM Video Capture Card,
   Titanmedia Supervision TV-2400,
   Provideo PV951 TF,
   3DeMon PV951,
   MediaForte TV-Vision PV951,
   Yoko PV951,
   Vivanco Tuner Card PCI Art.-Nr.: 68404
   )

- 监控系列:

 - PV-141
 - PV-143
 - PV-147
 - PV-148（仅采集）
 - PV-150
 - PV-151

- TV-FM 调谐器系列:

 - PV-951TDV（tv tuner + 1394）
 - PV-951T/TF
 - PV-951PT/TF
 - PV-956T/TF 薄型
 - PV-911

#### Highscreen


型号:

- TV Karte = LR50 Rev.S
- TV-Boostar = Terratec Terra TV+ Version 1.0（Bt848, tda9821）"ceb105.pcb"

#### Zoltrix


型号:

- Face to Face Capture（Bt848 仅采集）（PCB "VP-2848"）
- Face To Face TV MAX（Bt848）（PCB "VP-8482 Rev1.3"）
- Genie TV（Bt878）（PCB "VP-8790 Rev 2.1"）
- Genie Wonder Pro

#### AVerMedia


- AVer FunTV Lite（ISA, AV3001 芯片组）  "M101.C"
- AVerTV
- AVerTV Stereo
- AVerTV Studio（带 FM）
- AVerMedia TV98 带 Remote
- AVerMedia TV/FM98 Stereo
- AVerMedia TVCAM98
- TVCapture（Bt848）
- TVPhone（Bt848）
- TVCapture98（="AVerMedia TV98" 在美国）（Bt878）
- TVPhone98（Bt878, 带 FM）

======== =========== =============== ======= ====== ======== =======================
PCB      PCI-ID      Model-Name      Eeprom  Tuner  Sound    Country
======== =========== =============== ======= ====== ======== =======================
M101.C   ISA !
M108-B      Bt848                     --     FR1236		 US   [#f2]_, [#f3]_
M1A8-A      Bt848    AVer TV-Phone           FM1216  --
M168-T   1461:0003   AVerTV Studio   48:17   FM1216 TDA9840T  D    [#f1]_ w/FM w/Remote
M168-U   1461:0004   TVCapture98     40:11   FI1216   --      D    w/Remote
M168II-B 1461:0003   Medion MD9592   48:16   FM1216 TDA9873H  D    w/FM
======== =========== =============== ======= ====== ======== =======================


- 美国站点对这些型号有不同的驱动（截至 2002 年 09 月）:

  - EZ Capture/InterCam PCI（BT-848 芯片）
  - EZ Capture/InterCam PCI（BT-878 芯片）
  - TV-Phone（BT-848 芯片）
  - TV98（BT-848 芯片）
  - TV98 With Remote（BT-848 芯片）
  - TV98（BT-878 芯片）
  - TV98 With Remote（BT-878）
  - TV/FM98（BT-878 芯片）
  - AVerTV
  - AverTV Stereo
  - AVerTV Studio

DE 对这些型号有各种驱动（截至 2002 年 09 月）:

  - TVPhone（848）带 Philips 调谐器 FR12X6（带 FM 收音机）
  - TVPhone（848）带 Philips 调谐器 FM12X6（带 FM 收音机）
  - TVCapture（848）带 Philips 调谐器 FI12X6
  - TVCapture（848）非 Philips 调谐器
  - TVCapture98（Bt878）
  - TVPhone98（Bt878）
  - AVerTV 和 TVCapture98 带 VCR（Bt 878）
  - AVerTVStudio 和 TVPhone98 带 VCR（Bt878）
  - AVerTV GO Series（无 SVideo 输入）
  - AVerTV98（BT-878 芯片）
  - AVerTV98 带 Fernbedienung（遥控）（BT-878 芯片）
  - AVerTV/FM98（BT-878 芯片）

  - VDOmate（www.averm.com.cn）= M168U ？

#### Aimslab


型号:

- Video Highway 或 "Video Highway TR200"（ISA）
- Video Highway Xtreme（aka "VHX"）（Bt848, FM w/ TEA5757）

#### IXMicro（前: IMS=Integrated Micro Solutions）


型号:

- IXTV BT848（=TurboTV）
- IXTV BT878
- IMS TurboTV（Bt848）

#### Lifetec/Medion/Tevion/Aldi


型号:

- LT9306/MD9306 = CPH061
- LT9415/MD9415 = LR90 Rev.F 或 Rev.G
- MD9592 = Avermedia TVphone98（PCI_ID=1461:0003）, PCB-Rev=M168II-B（带 TDA9873H）
- MD9717 = KNC One（Rev D4, saa7134, FM1216 MK2 调谐器）
- MD5044 = KNC One（Rev D4, saa7134, FM1216ME MK3 调谐器）

#### Modular Technologies（www.modulartech.com）UK


型号:

- MM100 PCTV（Bt848）
- MM201 PCTV（Bt878, Bt832）带 Quartzsight 摄像头
- MM202 PCTV（Bt878, Bt832, tda9874）
- MM205 PCTV（Bt878）
- MM210 PCTV（Bt878）（Galaxy TV, Galaxymedia ?）

#### Terratec


型号:

- Terra TV+ Version 1.0（Bt848）, PCB 上印有 "ceb105.PCB", TDA9821
- Terra TV+ Version 1.1（Bt878）, PCB 上印有 "LR74 Rev.E", TDA9821
- Terra TValueRadio,             PCB 上印有 "LR102 Rev.C"
- Terra TV/Radio+ Version 1.0,   PCB 上印有 "80-CP2830100-0" TTTV3,
  PCB 背面有 "CPH010-E83", SAA6588T, TDA9873H
- Terra TValue Version BT878,    PCB 上印有 "80-CP2830110-0 TTTV4",
  背面有 "CPH011-D83"
- Terra TValue Version 1.0       "ceb105.PCB"（与 Terra TV+ Version 1.0 完全相同）
- Terra TValue New Revision	  "LR102 Rec.C"
- Terra Active Radio Upgrade（tea5757h, saa6588t）

- LR74 是 ceb105 的一个较新 PCB 修订版（两者都含用于 Active Radio Upgrade 的连接器）

- Cinergy 400（saa7134）, PCB 上印有 "E877 11(S)", "PM820092D"
- Cinergy 600（saa7134）

#### Technisat


型号:

- Discos ADR PC-Karte ISA（无 TV！）
- Discos ADR PC-Karte PCI（大概无 TV？）
- Techni-PC-Sat（Sat. analog）
  Rev 1.2（zr36120, vpx3220, stv0030, saa5246, BSJE3-494A）
- Mediafocus I（zr36120/zr36125, drp3510, Sat. analog + ADR Radio）
- Mediafocus II（saa7146, Sat. analog）
- SatADR Rev 2.1（saa7146a, saa7113h, stv0056a, msp3400c, drp3510a, BSKE3-307A）
- SkyStar 1 DVB  (AV7110) = Technotrend Premium
- SkyStar 2 DVB  (B2C2) (=Sky2PC)

#### Siemens


Multimedia eXtension Board（MXB）（SAA7146, SAA7111）

#### Powercolor


型号:

- MTV878
       包装带有不同内容:

           a) pcb "MTV878"（CARD=75）
           b) Pixelview Rev. 4\_

- MTV878R 带 Remote Control
- MTV878F 带 Remote Control 带 FM 收音机

#### Pinnacle


PCTV 型号:

- Mirovideo PCTV（Bt848）
- Mirovideo PCTV SE（Bt848）
- Mirovideo PCTV Pro（Bt848 + 用于 TV 立体声和 FM 的子板）
- Studio PCTV Rave（Bt848 Version = Mirovideo PCTV）
- Studio PCTV Rave（Bt878 包装，无红外）
- Studio PCTV      (Bt878)
- Studio PCTV Pro  (Bt878 stereo 带 FM)
- Pinnacle PCTV    (Bt878, MT2032)
- Pinnacle PCTV Pro (Bt878, MT2032)
- Pinncale PCTV Sat (bt878a, HM1821/1221) ["Conexant CX24110 with CX24108 tuner, aka HM1221/HM1811"]
- Pinnacle PCTV Sat XE

M(J)PEG 采集与回放型号:

- DC1+（ISA）
- DC10  (zr36057,     zr36060,      saa7110, adv7176)
- DC10+ (zr36067,     zr36060,      saa7110, adv7176)
- DC20  (ql16x24b,zr36050, zr36016, saa7110, saa7187 ...)
- DC30  (zr36057, zr36050, zr36016, vpx3220, adv7176, ad1843, tea6415, miro FST97A1)
- DC30+ (zr36067, zr36050, zr36016, vpx3220, adv7176)
- DC50  (zr36067, zr36050, zr36016, saa7112, adv7176 (2 pcs.?), ad1843, miro FST97A1, Lattice ???)

#### Lenco


型号:

- MXR-9565 (=Technisat Mediafocus?)
- MXR-9571（Bt848）(=CPH031?)
- MXR-9575
- MXR-9577（Bt878）(=Prolink 878TV Rev.3x)
- MXTV-9578CP（Bt878）(= Prolink PV-BT878P+4E)

#### Iomega


Buz（zr36067, zr36060, saa7111, saa7185）

#### LML

   LML33（zr36067, zr36060, bt819, bt856）

#### Grandtec


型号:

- Grand Video Capture（Bt848）
- Multi Capture Card  (Bt878)

#### Koutech


型号:

- KW-606（Bt848）
- KW-607（Bt848 仅采集）
- KW-606RSF
- KW-607A（仅采集）
- KW-608（Zoran 仅采集）

#### IODATA（jp）


型号:

- GV-BCTV/PCI
- GV-BCTV2/PCI
- GV-BCTV3/PCI
- GV-BCTV4/PCI
- GV-VCP/PCI（仅采集）
- GV-VCP2/PCI（仅采集）

#### Canopus（jp）


WinDVR	= Kworld "KW-TVL878RF"

#### www.sigmacom.co.kr


Sigma Cyber TV II

#### www.sasem.co.kr


Litte OnAir TV

#### hama


TV/Radio-Tuner Card, PCI（Model 44677）= CPH051

#### Sigma Designs


Hollywood plus（em8300, em9010, adv7175）, (PCB "M340-10") MPEG DVD 解码器

#### Formac


型号:

- iProTV（用于 iMac Mezzanine 槽的卡, Bt848+SCSI）
- ProTV（Bt848）
- ProTV II = ProTV Stereo（Bt878）["stereo" 指 FM 立体声, tv 仍是单声道]

#### ATI


型号:

- TV-Wonder
- TV-Wonder VE

#### Diamond Multimedia


DTV2000（Bt848, tda9875）

#### Aopen


- VA1000 Plus（带 Stereo）
- VA1000 Lite
- VA1000 (=LR90)

#### Intel


型号:

- Smart Video Recorder（ISA 全长者）
- Smart Video Recorder pro（ISA 半长者）
- Smart Video Recorder III（Bt848）

#### STB


型号:

- STB Gateway 6000704（bt878）
- STB Gateway 6000699（bt848）
- STB Gateway 6000402（bt848）
- STB TV130 PCI

#### Videologic


型号:

- Captivator Pro/TV（ISA？）
- Captivator PCI/VC（Bt848 与摄像头捆绑）（仅采集）

#### Technotrend


型号:

- TT-SAT PCI（PCB "Sat-PCI Rev.:1.3.1"; zr36125, vpx3225d, stc0056a, Tuner:BSKE6-155A
- TT-DVB-Sat
   - 修订版 1.1, 1.3, 1.5, 1.6 和 2.1
   - 这张卡作为 OEM 出售自:

 - Siemens DVB-s Card
 - Hauppauge WinTV DVB-S
 - Technisat SkyStar 1 DVB
 - Galaxis DVB Sat

   - 如今这张卡称为 TT-PCline Premium Family
   - TT-Budget（saa7146, bsru6-701a）
     这张卡作为 OEM 出售自:

 - Hauppauge WinTV Nova
 - Satelco Standard PCI（DVB-S）
   - TT-DVB-C PCI

#### Teles


 DVB-s（Rev. 2.2, BSRV2-301A, 仅数据？）

#### Remote Vision


MX RV605（Bt848 仅采集）

#### Boeder


型号:

- PC ChatCam（Model 68252）（Bt848 仅采集）
- Tv/Fm Capture Card  (Model 68404) = PV951

#### Media-Surfer  (esc-kathrein.de)


型号:

- Sat-Surfer（ISA）
- Sat-Surfer PCI = Techni-PC-Sat
- Cable-Surfer 1
- Cable-Surfer 2
- Cable-Surfer PCI（zr36120）
- Audio-Surfer（ISA Radio card）

#### Jetway（www.jetway.com.tw）


型号:

- JW-TV 878M
- JW-TV 878  = KWorld KW-TV878RF

#### Galaxis


型号:

- Galaxis DVB Card S CI
- Galaxis DVB Card C CI
- Galaxis DVB Card S
- Galaxis DVB Card C
- Galaxis plug.in S [neuer Name: Galaxis DVB Card S CI

#### Hauppauge


型号:

- 许多许多 WinTV 型号……
- WinTV DVBs = Technotrend Premium 1.3
- WinTV NOVA = Technotrend Budget 1.1 "S-DVB DATA"
- WinTV NOVA-CI "SDVBACI"
- WinTV Nova USB (=Technotrend USB 1.0)
- WinTV-Nexus-s (=Technotrend Premium 2.1 或 2.2)
- WinTV PVR
- WinTV PVR 250
- WinTV PVR 450

美国型号

-990 WinTV-PVR-350 (249USD) (iTVC15 chipset + radio)
-980 WinTV-PVR-250 (149USD) (iTVC15 chipset)
-880 WinTV-PVR-PCI (199USD) (KFIR chipset + bt878)
-881 WinTV-PVR-USB
-190 WinTV-GO
-191 WinTV-GO-FM
-404 WinTV
-401 WinTV-radio
-495 WinTV-Theater
-602 WinTV-USB
-621 WinTV-USB-FM
-600 USB-Live
-698 WinTV-HD
-697 WinTV-D
-564 WinTV-Nexus-S

Deutsche Modelle（德国型号）:

-603 WinTV GO
-719 WinTV Primio-FM
-718 WinTV PCI-FM
-497 WinTV Theater
-569 WinTV USB
-568 WinTV USB-FM
-882 WinTV PVR
-981 WinTV PVR 250
-891 WinTV-PVR-USB
-541 WinTV Nova
-488 WinTV Nova-Ci
-564 WinTV-Nexus-s
-727 WinTV-DVB-c
-545 Common Interface
-898 WinTV-Nova-USB

UK 型号:

-607 WinTV Go
-693,793 WinTV Primio FM
-647,747 WinTV PCI FM
-498 WinTV Theater
-883 WinTV PVR
-893 WinTV PVR USB  (Duplicate entry)
-566 WinTV USB (UK)
-573 WinTV USB FM
-429 Impact VCB (bt848)
-600 USB Live (Video-In 1x Comp, 1xSVHS)
-542 WinTV Nova
-717 WinTV DVB-S
-909 Nova-t PCI
-893 Nova-t USB   (Duplicate entry)
-802 MyTV
-804 MyView
-809 MyVideo
-872 MyTV2Go FM
-546 WinTV Nova-S CI
-543 WinTV Nova
-907 Nova-S USB
-908 Nova-T USB
-717 WinTV Nexus-S
-157 DEC3000-s Standalone + USB

Spain（西班牙）:

-685 WinTV-Go
-690 WinTV-PrimioFM
-416 WinTV-PCI Nicam Estereo
-677 WinTV-PCI-FM
-699 WinTV-Theater
-683 WinTV-USB
-678 WinTV-USB-FM
-983 WinTV-PVR-250
-883 WinTV-PVR-PCI
-993 WinTV-PVR-350
-893 WinTV-PVR-USB
-728 WinTV-DVB-C PCI
-832 MyTV2Go
-869 MyTV2Go-FM
-805 MyVideo (USB)


#### Matrix-Vision


型号:

- MATRIX-Vision MV-Delta
- MATRIX-Vision MV-Delta 2
- MVsigma-SLC（Bt848）

#### Conceptronic（.net）


型号:

- TVCON FM,  TV card w/ FM = CPH05x
- TVCON = CPH06x

#### BestData


型号:

- HCC100 = VCC100rev1 + camera
- VCC100 rev1（bt848）
- VCC100 rev2（bt878）

#### Gallant  (www.gallantcom.com) www.minton.com.tw


型号:

- Intervision IV-510（仅采集 bt8x8）
- Intervision IV-550（bt8x8）
- Intervision IV-100（zoran）
- Intervision IV-1000（bt8x8）

#### Asonic（www.asonic.com.cn）（网站已关闭）


SkyEye tv 878

#### Hoontech


878TV/FM

#### Teppro（www.itcteppro.com.tw）


型号:

- ITC PCITV（Card Ver 1.0）"Teppro TV1/TVFM1 Card"
- ITC PCITV（Card Ver 2.0）
- ITC PCITV（Card Ver 3.0）= "PV-BT878P+ (REV.9D)"
- ITC PCITV（Card Ver 4.0）
- TEPPRO IV-550（For BT848 Main Chip）
- ITC DSTTV（bt878, satellite）
- ITC VideoMaker（saa7146, StreamMachine sm2110, tvtuner）"PV-SM2210P+ (REV:1C)"

#### Kworld（www.kworld.com.tw）


PC TV Station:

- KWORLD KW-TV878R  TV（无收音机）
- KWORLD KW-TV878RF TV（带收音机）
- KWORLD KW-TVL878RF（薄型）
- KWORLD KW-TV713XRF（saa7134）


 MPEG TV Station（与上述相同的卡，加上 WinDVR 软件 MPEG 编/解码器）

- KWORLD KW-TV878R -Pro   TV（无 Radio）
- KWORLD KW-TV878RF-Pro   TV（带 Radio）
- KWORLD KW-TV878R -Ultra TV（无 Radio）
- KWORLD KW-TV878RF-Ultra TV（带 Radio）

#### JTT/ Justy Corp.(http://www.jtt.ne.jp/)


JTT-02（JTT TV）"TV watchmate pro"（bt848）

#### ADS www.adstech.com


型号:

- Channel Surfer TV（ CHX-950 ）
- Channel Surfer TV+FM（ CHX-960FM ）

#### AVEC www.prochips.com


AVEC Intercapture（bt848, tea6320）

#### NoBrand


TV Excel = "PV-BT878P+ 8E" 或 "878TV Rev.3\_" 的澳大利亚名

#### Mach www.machspeed.com


Mach TV 878

#### Eline www.eline-net.com/


型号:

- Eline Vision TVMaster / TVMaster FM (ELV-TVM/ ELV-TVM-FM) = LR26  (bt878)
- Eline Vision TVMaster-2000 (ELV-TVM-2000, ELV-TVM-2000-FM)= LR138 (saa713x)

#### Spirit


- Spirit TV Tuner/Video Capture Card（bt848）

#### Boser www.boser.com.tw


型号:

- HS-878 Mini PCI Capture Add-on Card
- HS-879 Mini PCI 3D Audio and Capture Add-on Card (w/ ES1938 Solo-1)

#### Satelco www.citycom-gmbh.de, www.satelco.de


型号:

- TV-FM =KNC1 saa7134
- Standard PCI（DVB-S）= Technotrend Budget
- Standard PCI（DVB-S）带 CI
- Satelco Highend PCI（DVB-S）= Technotrend Premium


#### Sensoray www.sensoray.com


型号:

- Sensoray 311（PC/104 总线）
- Sensoray 611（PCI）

#### CEI（Chartered Electronics Industries Pte Ltd [CEI] [FCC ID HBY]）


型号:

- TV Tuner  -  HBY-33A-RAFFLES  Brooktree Bt848KPF + Philips
- TV Tuner MG9910  -  HBY33A-TVO  CEI + Philips SAA7110 + OKI M548262 + ST STV8438CV
- Primetime TV（ISA）

  - 被新加坡科技（Singapore Technologies）收购
  - 现在作为 Chartered Semiconductor Manufacturing 运营
  - 显卡制造商列为:

    - Cogent Electronics Industries [CEI]

#### AITech


型号:

- Wavewatcher TV（ISA）
- AITech WaveWatcher TV-PCI = 可以是 LR26（Bt848）或 LR50（BT878）
- WaveWatcher TVR-202 TV/FM Radio Card（ISA）

#### MAXRON


Maxron MaxTV/FM Radio（KW-TV878-FNT）= Kworld 或 JW-TV878-FBK

#### www.ids-imaging.de


型号:

- Falcon Series（仅采集）

In USA: http://www.theimagingsource.com/
- DFG/LC1

#### www.sknet-web.co.jp


SKnet Monster TV（saa7134）

#### A-Max www.amaxhk.com（Colormax, Amax, Napa）


APAC Viewcomp 878

#### Cybertainment


型号:

- CyberMail AV Video Email Kit w/ PCI Capture Card（仅采集）
- CyberMail Xtreme

These are Flyvideo（这些是 Flyvideo）

#### VCR（http://www.vcrinc.com/）


Video Catcher 16

#### Twinhan


型号:

- DST Card/DST-IP（bt878, twinhan asic）VP-1020
  - 作为以下名称出售:

    - KWorld DVBS Satellite TV-Card
    - Powercolor DSTV Satellite Tuner Card
    - Prolink Pixelview DTV2000
    - Provideo PV-911 Digital Satellite TV Tuner Card With Common Interface ?

- DST-CI Card（DVB Satellite）VP-1030
- DCT Card（DVB cable）

#### MSI


型号:

- MSI TV@nywhere Tuner Card（MS-8876）（CX23881/883）不兼容 Bt878。
- MS-8401 DVB-S

#### Focus www.focusinfo.com


InVideo PCI（bt878）

#### Sdisilk www.sdisilk.com/


型号:

- SDI Silk 100
- SDI Silk 200 SDI Input Card

#### www.euresys.com


PICOLO series（PICOLO 系列）

#### PMC/Pace


www.pacecom.co.uk 网站已关闭

#### Mercury www.kobian.com（UK and FR）


型号:

- LR50
- LR138RBG-Rx  == LR138

#### TEC sound


TV-Mate = Zoltrix VP-8482

虽然通过有技巧的谷歌搜索找到了: www.techmakers.com

（包装和手册没有任何其他制造商信息）TecSound

#### Lorenzen www.lorenzen.de


SL DVB-S PCI = Technotrend Budget PCI（su1278 或 bsru 版本）

#### Origo（.uk）www.origo2000.com


PC TV Card = LR50

#### I/O Magic www.iomagic.com


PC PVR - Desktop TV Personal Video Recorder DR-PCTV100 = Pinnacle ROB2D-51009464 4.0 + Cyberlink PowerVCR II

#### Arowana


TV-Karte / Poso Power TV（?）= Zoltrix VP-8482（？）

#### iTVC15 板


kuroutoshikou.com ITVC15

yuan.com MPG160 PCI TV（Internal PCI MPEG2 encoder card plus TV-tuner）

#### Asus www.asuscom.com


型号:

- Asus TV Tuner Card 880 NTSC（薄型, cx23880）
- Asus TV（saa7134）

#### Hoontech


http://www.hoontech.de/

- HART Vision 848（H-ART Vision 848）
- HART Vision 878（H-Art Vision 878）



### bttv 设备使用的芯片


- 所有板:

  - Brooktree Bt848/848A/849/878/879: 视频采集芯片

- 板特定

  - Miro PCTV:

    - Philips 或 Temic 调谐器

  - Hauppauge Win/TV pci（version 405）:

    - Microchip 24LC02B 或 Philips 8582E2Y:

       - 256 字节 EEPROM 带配置信息
       - I2C 0xa0-0xa1,（24LC02B 也响应 0xa2-0xaf）

    - Philips SAA5246AGP/E: 图文电视解码器芯片, I2C 0x22-0x23

    - TDA9800: 声音解码器

    - Winbond W24257AS-35: 32Kx8 CMOS 静态 RAM（图文电视缓冲内存）

    - 14052B: 用于选择声音源的模拟开关

- PAL:

  - TDA5737: VHF、超高频带和 UHF 混频器/振荡器，用于 TV 和 VCR 3 频段调谐器
  - TSA5522: 1.4 GHz I2C 总线控制合成器, I2C 0xc2-0xc3

- NTSC:

  - TDA5731: VHF、超高频带和 UHF 混频器/振荡器，用于 TV 和 VCR 3 频段调谐器
  - TSA5518: Philips 站点上没有数据手册可用

- STB TV pci:

  - ???
  - 如果你想要对 STB 卡更好的支持，给我发信息！
    看看板子！上面有哪些芯片？



### 规格（Specs）


Philips		http://www.Semiconductors.COM/pip/

Conexant	http://www.conexant.com/

Micronas	http://www.micronas.com/en/home/index.html

### 致谢


非常感谢:

- Markus Schroeder <schroedm@uni-duesseldorf.de>，提供了关于 Bt848 和调谐器编程的信息以及他的控制程序 xtvc。

- Martin Buck <martin-2.buck@student.uni-ulm.de>，提供了他出色的图文电视包。

- Gerd Hoffmann，提供了 MSP3400 支持和模块化的 I2C、调谐器……支持。


- MATRIX Vision，免费给了我们 2 张卡，使单晶振操作的支持成为可能。

- MIRO，提供了一张免费的 PCTV 卡以及关于他们卡上组件的详细信息。（例如调谐器类型是如何探测的）没有他们的卡，我无法调试 NTSC 模式。

- Hauppauge，告知如何选择声音输入，以及他们在收音机卡上使用和将会使用哪些组件。也非常感谢给我传真 FM1216 数据手册。

### 贡献者


Michael Chu <mmchu@pobox.com>
  AverMedia 修复以及更灵活的卡识别

Alan Cox <alan@lxorguk.ukuu.org.uk>
  Video4Linux 接口以及 2.1.x 内核适配

Chris Kleitsch
  Hardware I2C

Gerd Hoffmann
  Radio card（ITT 声音处理器）

bigfoot <bigfoot@net-way.net>

Ragnar Hojland Espinosa <ragnar@macula.net>
  ConferenceTV card


- 还有更多人（如果你不在这个列表中但希望被提及，请给我发邮件）
