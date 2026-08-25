## 关于内核 OSS 模拟的说

Jan. 22, 2004  Takashi Iwai <tiwai@suse.de>

## 模块

ALSA 在内核中提供了一套强大的 OSS 模拟。针PCM、混音器（mixer）和音序器（sequencer）设备的 OSS 模拟，作为附加内核模snd-pcm-oss、snd-mixer-oss snd-seq-oss 实现。当您需要访OSS PCM、mixer sequencer 设备时，必须加载相应的模块

这些模块在调用相应服务时会自动加载。其别名定义`sound-service-x-y`，其x y 分别是声卡编号和次设备号（minor unit number）。通常您不需要自己定义这些别名

实现 OSS 模块自动加载所需的唯一步骤是定
```

	alias sound-slot-0 snd-emu10k1

```
作为第二块声卡，同样定义 `sound-slot-1`。请注意，您不能将别名名称用作目标名称（`alias sound-slot-0 snd-card-0` 不再像旧modutils 那样起作用）

当前可用OSS 配置显示/proc/asound/oss/sndstat。它采用/dev/sndstat 相同的语法，而后者在商业 OSS 驱动上可用。在 ALSA 上，您可以将 /dev/sndstat 符号链接到这proc 文件

请注意，proc 文件中列出的设备只有在相应的 OSS 模拟模块加载之后才会出现。即便其中显"NOT ENABLED IN CONFIG" 也不必担心

## 设备映射

ALSA 支持以下 OSS 设备文件
```

	PCM:
		/dev/dspX
		/dev/adspX

	Mixer:
		/dev/mixerX

	MIDI:
		/dev/midi0X
		/dev/amidi0X

	Sequencer:
		/dev/sequencer
		/dev/sequencer2 (aka /dev/music)

```
其中 X 0 7 的声卡编号

（注意：某些发行版拥有诸/dev/midi0 /dev/midi1 这样的设备文件。它们并非用OSS，而是用于 tclmidi，那是完全不同的东西。）

与真正的 OSS 不同，ALSA 不能使用超出所分配范围的设备文件。例如，第一块声卡不能使/dev/dsp1 /dev/dsp2，而只能使/dev/dsp0 /dev/adsp0

如上所示，PCM MIDI 可能拥有两个设备。通常，第一PCM 设备（ALSA 中的 `hw:0,0`）映射到 /dev/dsp，而第二个设备（`hw:0,1`）映射到 /dev/adsp（如果可用）。对MIDI，则分别/dev/midi /dev/amidi

您可以通过 snd-pcm-oss snd-rawmidi 的模块选项来改变这一设备映射。就 PCM 而言，snd-pcm-oss 提供以下选项

dsp_map
	分配/dev/dspX PCM 设备编号
	（默认= 0
adsp_map
	分配/dev/adspX PCM 设备编号
	（默认= 1

例如，要将第三个 PCM 设备（`hw:0,2`）映射到 /dev/adsp0，按如下方式定义
```

	options snd-pcm-oss adsp_map=2

```
这些选项接受数组。要配置第二块声卡，请用逗号分隔指定两个条目。例如，要将第二块声卡上的第三个 PCM 设备映射/dev/adsp1，按如下方式定义
```

	options snd-pcm-oss adsp_map=0,2

```
要改MIDI 设备的映射，snd-rawmidi 提供以下选项

midi_map
	分配/dev/midi0X MIDI 设备编号
	（默认= 0
amidi_map
	分配/dev/amidi0X MIDI 设备编号
	（默认= 1

例如，要将第一块声卡上的第三个 MIDI 设备分配/dev/midi00，按如下方式定义
```

	options snd-rawmidi midi_map=2



```
## PCM 模式

默认情况下，ALSA 通过所谓的插件层（plugin layer）来模拟 OSS PCM，也就是说，当声卡本身不支持时，它会尝试自动转换采样格式、采样率或通道数。这会为某些应用程序（如 quake wine）带来一些问题，尤其是当它们仅在 MMAP 模式下使用声卡时

在这种情况下，您可以通过proc 文件写入命令来按应用程序改变 PCM 的行为。每PCM 流都有一proc 文件，`/proc/asound/cardX/pcmY[cp]/oss`，其X 是声卡编号（0 开始），Y PCM 设备编号（从 0 开始），`p` 代表回放（playback），`c` 代表采集（capture）。请注意，该 proc 文件只有snd-pcm-oss 模块加载后才存在

命令序列具有以下语法
```

	app_name fragments fragment_size [options]

```
`app_name` 是带路径（优先级更高）或不带路径的应用程序名称
`fragments` 指定片段（fragment）的数量，若未给定具体数量则0
`fragment_size` 是片段的大小（以字节为单位），若未给定则0
`options` 是可选参数。可用的选项如下

disable
	应用程序尝试为该通道打开一pcm 设备但不想使用它
direct
	不使用插
block
	强制阻塞打开模式
non-block
	强制非阻塞打开模式
partial-frag
	也写入部分片段（仅影响回放）
no-silence
	不要预先填充静音数据以避免爆

`disable` 选项在应用程序未能正确处理某一流方向（回放或采集）、而硬件本身同时支持两个方向时很有用。如上所述，`direct` 选项用于绕过自动转换，对 MMAP 应用程序很有用。例如，要针quake 在不使用插件的情况下回放第一PCM 设备，通过 echo 发送如下命令：
```

	% echo "quake 0 0 direct" > /proc/asound/card0/pcm0p/oss

```
由于 quake 只需要回放，您可以追加第二条命令，通知驱动程序即将分配的方向仅此一个：
```

	% echo "quake 0 0 disable" > /proc/asound/card0/pcm0c/oss

```
proc 文件的权限取决于 snd 的模块选项。默认情况下它被设置root，因此发送上述命令时您很可能必须是超级用户

block non-block 选项用于改变打开设备文件的行为

默认情况下，ALSA 的行为与原始 OSS 驱动一致，即在文件忙时不阻塞。这种情况下会返-EBUSY 错误

这一阻塞行为可以通过 snd-pcm-oss nonblock_open 模块选项进行全局改变。若要将阻塞模式作为 OSS 设备的默认模式，按如下方式定义：
```

	options snd-pcm-oss nonblock_open=0

```
`partial-frag` `no-silence` 这两个命令是最近才加入的。这两个命令仅用于优化。前者命令指定仅在整段片段被填满时才发起写入传输。后者会停止自动预先写入静音数据。两者默认均禁用

您可以通过读取 proc 文件来检查当前定义的配置。读取到的映像可以再次发送给 proc 文件，因此您可以保存当前配置
```

	% cat /proc/asound/card0/pcm0p/oss > /somewhere/oss-cfg

```
并按如下方式恢复
```

	% cat /somewhere/oss-cfg > /proc/asound/card0/pcm0p/oss

```
此外，要清除所有当前配置，发`erase` 命令，如下：
```

	% echo "erase" > /proc/asound/card0/pcm0p/oss


```
## 娣烽煶鍣ㄥ厓绱。

由于 ALSA 具有完全不同的混音器接口，对 OSS 混音器的模拟相对复杂。ALSA 基于名称字符串，由若干不同的 ALSA（mixer）控件构建出一个混音器元素。例如，音量元素 SOUND_MIXER_PCM 由回放方向的 "PCM Playback Volume" "PCM Playback Switch" 控件，以及采集方向（如果存在）的 "PCM Capture Volume" "PCM Capture Switch" 控件组成。当 OSS PCM 音量改变时，上述所有音量和开关控件都会自动被调整

默认情况下，ALSA OSS 音量的使用如下控件：

====================	=====================	=====
OSS volume		ALSA control		Index
====================	=====================	=====
SOUND_MIXER_VOLUME 	Master			0
SOUND_MIXER_BASS	Tone Control - Bass	0
SOUND_MIXER_TREBLE	Tone Control - Treble	0
SOUND_MIXER_SYNTH	Synth			0
SOUND_MIXER_PCM		PCM			0
SOUND_MIXER_SPEAKER	PC Speaker 		0
SOUND_MIXER_LINE		Line			0
SOUND_MIXER_MIC		Mic 			0
SOUND_MIXER_CD		CD 			0
SOUND_MIXER_IMIX		Monitor Mix 		0
SOUND_MIXER_ALTPCM	PCM			1
SOUND_MIXER_RECLEV	（未分配
SOUND_MIXER_IGAIN	Capture			0
SOUND_MIXER_OGAIN	Playback		0
SOUND_MIXER_LINE1	Aux			0
SOUND_MIXER_LINE2	Aux			1
SOUND_MIXER_LINE3	Aux			2
SOUND_MIXER_DIGITAL1	Digital			0
SOUND_MIXER_DIGITAL2	Digital			1
SOUND_MIXER_DIGITAL3	Digital			2
SOUND_MIXER_PHONEIN	Phone			0
SOUND_MIXER_PHONEOUT	Phone			1
SOUND_MIXER_VIDEO	Video			0
SOUND_MIXER_RADIO	Radio			0
SOUND_MIXER_MONITOR	Monitor			0
====================	=====================	=====

第二列是相应 ALSA 控件的基字符串（base-string）。实际上，还会额外检查带``XXX [Playback|Capture] [Volume|Switch]`` 的控件

这些混音器元素的当前分配列在 proc 文件 /proc/asound/cardX/oss_mixer 中，其形式如
```

	VOLUME "Master" 0
	BASS "" 0
	TREBLE "" 0
	SYNTH "" 0
	PCM "PCM" 0
	...

```
其中第一列是 OSS 音量元素，第二列是相ALSA 控件的基字符串，第三列是控件索引（control index）。当字符串为空时，表示相应的 OSS 控件不可用

要改变分配，您可以向这个 proc 文件写入配置。例如，要将 "Wave Playback" 映射PCM 音量，发送如下命令：
```

	% echo 'VOLUME "Wave Playback" 0' > /proc/asound/card0/oss_mixer

```
该命令与 proc 文件中列出的完全一致。您可以一次改变一个或多个元素，每行一个音量。在最后一个示例中，当 PCM 音量改变时，"Wave Playback Volume" "Wave Playback Switch" 都会受到影响

PCM proc 文件的情况一样，proc 文件的权限取决于 snd 的模块选项。发送上述命令时您很可能必须是超级用户

PCM proc 文件的情况相同，您可以通过读取并写入整个文件映像来保存和恢复当前的混音器配置

## 双工

请注意，当尝试使用单一设备文件进行回放和采集时，OSS API 无法提供方法来为两个方向分别设置不同的格式、采样率或通道数。因
```

	io_handle = open("device", O_RDWR)

```
只有在两个方向的值相同时才能正确工作

若要在两个方向使用不同的值，请同时使
```

	input_handle = open("device", O_RDONLY)
	output_handle = open("device", O_WRONLY)

```
并为相应的句柄设置值

## 不支持的特

### MMAP ICE1712 驱动

ICE1712 仅支持非惯例的格式，即交错（interleaved）的 10 通道 24 位（打包32 位）格式。因此您无法OSS 上以惯例（单声道2 通道 16 位）格式mmap 该缓冲区
