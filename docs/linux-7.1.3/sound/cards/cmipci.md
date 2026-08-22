## C-Media 8338/8738/8768/8770 驱动简要说

Takashi Iwai <tiwai@suse.de>


### 前置/后置多声道播

CM8x38 芯片可以ADC 用作第二DAC，从而可以使用两个不同的立体声通道分别用于前置/后置播放。由于有两个 DAC，这两个数据流是独立处理的，这与下文 4/6 声道多声道播放不同
默认情况下，ALSA 驱动将第一PCM 设备（即 card#0 hw:0,0）分配给前置4/6 声道播放，而第二个 PCM 设备（hw:0,1）则分配给第二个 DAC 用于后置播放
两个 DAC 之间存在细微差别
- 第一DAC 支持 U8 S16LE 格式，而第二个 DAC 仅支S16LE- 第二DAC 仅支持双声道立体声
请注意，CM8x38 DAC 不支持连续播放速率，而仅支持固定速率51200010256000205020004100 48000 Hz
只有"Four Channel Mode"（四声道模式）开关被禁用时，才能听到后置输出。否则不会有信号路由到后置扬声器。默认情况下该开关是打开的
  "Four Channel Mode" 开关关闭时，后置扬声器的输出将是全音量，不Master PCM 音量的影[#]_。这可能会损坏你的音频设备。请在关闭此开关之前断开扬声器连接
  嗯……我曾经得到过音量正确（即与前置相同）的输出，当时非常兴奋。那甚至是在 "Four Channel" 位打开且为 "double DAC" 模式的情况下。实际上我从前置和后置扬声器听到了独立的 4 个声道！但是……重启之后，一切都不见了。很遗憾我当时没有保存寄存器转储……也许存在某个未知寄存器可以实现这一点…
如果你的声卡有一个用于后置输出的额外输出插孔，那么后置播放默认应当路由到那里。如果没有，驱动中有一个名"Line-In As Rear" 的控制开关，你可以通过 alsamixer 或其他方式进行更改。当该开关打开时，line-in 插孔被用作后置输出
还有两个与后置输出相关的控制项"Exchange DAC" 开关用于交换前置和后置播放路由，即让第二个 DAC 从前置输出
### 4/6 澶氬０閬撴挱鏀。

近期CM8738 芯片支持 4/6 多声道播放功能。这AC3 解码时尤其有用
当支持多声道时，驱动名称会带有一"-MC" 后缀，例"CMI8738-MC6"。你可以/proc/asound/cards 查看此名称
当启4/6 声道输出时，第二DAC 最多接6（或 4）个声道。虽然双 DAC 支持两种不同的速率或格式，4/6 声道播放对所有声道只支持相同的条件。由于多声道播放模式使用了两DAC，你无法以全双工方式操作
4.0 5.1 模式alsa-lib 中定义为 pcm "surround40" "surround51"。例如，你可以像下面这样播放一个包6 个声道的 WAV 文件```
	% aplay -Dsurround51 sixchannels.wav
```

要对 4/6 声道播放进行编程，你需要按需指定 PCM 声道，并将格式设置为 S16LE。例如，对于 4 声道播放```
	snd_pcm_hw_params_set_access(pcm, hw, SND_PCM_ACCESS_RW_INTERLEAVED);
	    // 或者如果你愿意，也可以使用 mmap
	snd_pcm_hw_params_set_format(pcm, hw, SND_PCM_FORMAT_S16_LE);
	snd_pcm_hw_params_set_channels(pcm, hw, 4);
```

并使用交织的 4 声道数据
有一些控制开关会影响扬声器连接：

Line-In Mode
	一个枚举控制，用于更改 line-in 插孔的行为	可以选择 "Line-In"Rear Output" "Bass Output"	最后一项仅039 或更新的型号上可用	当选择 "Rear Output" 时，环绕声道 3 4 会输出到 line-in 插孔Mic-In Mode
	一个枚举控制，用于更改 mic-in 插孔的行为	可以选择 "Mic-In" "Center/LFE Output"	当选择 "Center/LFE Output" 时，中置和低音声道（声道 5 6）会输出mic-in 插孔
### 数字 I/O


CM8x38 以非常低廉的价格提供了出色的 SPDIF 能力（没错，这就是我买这块卡的原:)

SPDIF 的播放和捕获是通过第三PCM 设备（hw:0,2）完成的。通常它被分配PCM 设备 "spdif"。可用的速率44100 48000 Hz对于使用 aplay 播放，你可以像下面这样运行：
```
	% aplay -Dhw:0,2 foo.wav
```

或```
	% aplay -Dspdif foo.wav
```

24 位格式也以实验方式受支持
通过 SPDIF 进行的播放和捕获分别使用普通的 DAC ADC，因此你无法同时播放模拟和数字数据流
要启SPDIF 输出，你需要通过混音器或 alsactl 打开 "IEC958 Output Switch" 控制IEC958" 是所S/PDIF 的官方名称）。然后你会看到卡上亮起红灯，这样你就明显知道它正在工作了 :)
SPDIF 输入始终处于启用状态，因此你可以随时通过 "IEC958 In Monitor" 开关从 line-out 听到 SPDIF 输入数据（见下文）
你甚至可以通过第一个设备（hw:0,0）进SPDIF 播放，但 SPDIF 仅在使用了正确的格式（S16LE）、采样率4100 48000）和声道数（2）时才启用。否则它会被关闭。（另外也别忘了打开 "IEC958 Output Switch"。）

此外还有一些相关的控制开关：

IEC958 Mix Analog
	将模PCM 播放FM-OPL/3 数据流混合，并通过 SPDIF 输出	此开关仅出现在旧芯片型号（CM8738 033 037）上
	注意：没有此控制你也可以PCM 输出SPDIF	这是数据流的“混合”，因此它不适用AC3 输出（见下一节）
IEC958 In Select
	选择 SPDIF 输入，内CD-in（false）或外部输入（true）
IEC958 Loop
	SPDIF 输入数据环回（loop back）到 SPDIF 输出（即旁路，bypass）
IEC958 Copyright
	设置版权位
IEC958 5V
	选择 0.5V（同轴）5V（光纤）接口	在某些卡上这不起作用，你需要通过硬件拨码开关更改配置
IEC958 In Monitor
	SPDIF 输入被路由到 DAC
IEC958 In Phase Inverse
	SPDIF 输入格式设置为反相	[FIXME: 这在所有芯片上都不起作.]

IEC958 In Valid
	设置输入有效性标志检测
注意：当 "PCM Playback Switch" 打开时，你会通过模拟 line-out 听到数字输出数据流
### AC3（原始数字）输出


驱动支持通过 SPDIF 进行原始数字（通常AC3）的 I/O。这可以通过 IEC958 播放控制来切换，但通常你需要通过 alsa-lib 来访问。更多细节请参阅 alsa-lib 文档
在原始数字模式下PCM Playback Switch" 会自动关闭，以便非音频数据不会从模拟 line-out 听到。类似地，以下开关也会关闭："IEC958 Mix Analog" "IEC958 Loop"。关SPDIF PCM 设备后，这些开关会自动恢复到之前的状态
033 型号上，AC3 是通过 alsa-lib 中的软件转换实现的。如果你需要绕IEC958 子帧的软件转换，请传"soft_ac3=0" 模块选项。这在较新的型号上无关紧要
### 模拟混音器接

CM8x38 上的混音器接口类似于 SB16Master、PCM、Synth、CD、Line、Mic PC Speaker 播放音量。与 SB16 一样，Synth、CD、Line Mic 也有播放和捕获开关
除了标准SB 混音器外，CM8x38 还提供了更多功能- PCM 播放开- PCM 捕获开关（用于捕获发送到 DAC 的数据）
- Mic Boost 开- Mic 捕获音量
- Aux 播放音量/开关以及捕获开- 3D 控制开
### MIDI 鎺у埗鍣?

对于 CMI8338 芯片，MPU401-UART 接口默认是禁用的。你需要将模块选项 "mpu_port" 设置为一个有效的 I/O 端口地址来启MIDI 支持。有效的 I/O 端口0x300x310x320 0x330。请选择一个不会与其他卡冲突的值
对于 CMI8738 及更新的芯片，MIDI 接口默认启用，驱动会自动选择一个端口地址
此芯片上 **没有** 硬件波表功能（下OPL3 合成器除外）。在 Windows 上被称为 MIDI 合成器的是软件合成器仿真。在 Linux 上请使用 TiMidity 或其他软合成器程序来播放 MIDI 音乐
### FM OPL/3 合成

FM OPL/3 默认也仅对第一块卡启用。为有更多卡设置 "fm_port" 模块选项
不过，FM OPL/3 的输出质量非常奇怪我不知道为什.

CMI8768 及更新的芯片没有 FM 合成器
### 操纵杆与调制解调

支持传统操纵杆。要启用操纵杆支持，请传joystick_port=1 模块选项。1 表示自动检测。如果自动检测失败，请尝试传入确切的 I/O 地址
调制解调器通过卡控制开"Modem" 动态启用
### 调试信息


寄存器显示在 /proc/asound/cardX/cmipci 中。如果你遇到任何问题（尤其是混音器的异常行为），请将proc 文件的输出随缺陷报告一起附上