## HD-Audio 驱动补充说明


Takashi Iwai <tiwai@suse.de>


## 概述


HD-audio 是继 AC97 之后现代 PC 上板载音频的新标准组件。尽管 Linux 对
HD-audio 的支持由来已久，但新机器上常常出现问题。一部分问题是 BIOS 缺陷
造成的，其余则来自驱动实现。本文档简要介绍针对 HD-audio 硬件的故障排查与
调试方法。

HD-audio 组件由两部分组成：控制器芯片和 HD-audio 总线上的编解码器（codec）
芯片。Linux 为所有控制器提供一个统一的驱动 snd-hda-intel。虽然驱动名称中
包含一个知名硬件厂商的字样，但它并非专属于该厂商，而是适用于其他公司生产
的所有控制器芯片。由于 HD-audio 控制器应当保持兼容，这个单一的 snd-hda
驱动在多数情况下都能工作。但不足为奇的是，每种控制器类型都有其已知的缺陷
与问题。snd-hda-intel 驱动为此准备了如下所述的一批变通（workaround）方案。

一个控制器可能拥有多个编解码器。通常你有一个音频编解码器，以及可选的一个
调制解调器编解码器。理论上，也可能存在多个音频编解码器（例如分别用于模拟
与数字输出），而驱动可能因为混音器元素冲突而无法正常工作。如果这类硬件真的
存在，未来应当对此进行修复。

snd-hda-intel 驱动根据编解码器提供了几种不同的解析器。它有一个通用的解析器
作为后备，但到目前为止该功能相当有限。通常会使用编解码器专用的解析器（在
patch_*.c 中实现）来处理编解码器相关的逻辑。有关编解码器专用问题的细节将在
后续章节说明。

如果你对 HD-audio 的深度调试感兴趣，请先阅读 HD-audio 规范。例如，该规范可
在 Intel 的网页上找到：

- https://www.intel.com/content/www/us/en/standards/high-definition-audio-specification.html


## HD-Audio 控制器


### DMA 位置问题

控制器最常见的问题是 DMA 指针报告不准确。回放与采集的 DMA 指针可以通过两种
方式读取：经由 LPIB 寄存器，或者经由 position-buffer 映射。默认情况下，驱动
尝试从 io 映射的 position-buffer 读取，若 position-buffer 看起来失效则回退到
LPIB。然而，这种检测在某些设备上并不完善。在这种情况下，你可以通过
`position_fix` 选项更改默认方法。

`position_fix=1` 表示显式使用 LPIB 方法。
`position_fix=2` 表示使用 position-buffer。
`position_fix=3` 表示结合使用两种方法，某些 VIA 控制器需要如此。采集流的
位置通过比较 LPIB 与 position-buffer 两个值来修正。
`position_fix=4` 是另一种可用于所有控制器的组合，对回放使用 LPIB，对采集
流使用 position-buffer。
`position_fix=5` 目前专用于 Intel 平台，适用于 Skylake 及之后的型号。它
应用延迟计算以实现精确的位置报告。
`position_fix=6` 用于以固定的 FIFO 大小来修正位置，主要针对近期的 AMD 控制器。
0 是所有其他控制器的默认值，即上文所述的自动检查并回退到 LPIB。如果你遇到
声音重复的问题，该选项可能会有所帮助。

除此之外，每个控制器在唤醒时序方面都有已知的缺陷。它会在实际处理缓冲区
数据之前提前几个采样唤醒。这引发了许多问题，例如与 ALSA dmix 或 JACK 的配合。
自 2.6.27 内核起，驱动对唤醒时序人为地加入了一段延迟。该延迟由 `bdl_pos_adj`
选项控制。

当 `bdl_pos_adj` 为负值（默认值）时，会根据控制器芯片赋以适当的值。对于 Intel
芯片为 1，而其他芯片为 32。通常这样能工作。只有在它不工作且你收到警告消息时，
才应将该参数改为其他值。


### 编解码器探测问题

较少发生但更严重的问题是编解码器探测。当 BIOS 错误地报告可用的编解码器槽位
时，驱动会感到困惑并尝试访问不存在的编解码器槽位。这通常会导致彻底的混乱，
并破坏与编解码器芯片的进一步通信。其症状通常表现为如下错误消息：
```

    hda_intel: azx_get_response timeout, switching to polling mode:
          last cmd=0x12345678
    hda_intel: azx_get_response timeout, switching to single_cmd mode:
          last cmd=0x12345678

```
第一行是警告，通常相对无害。它表示编解码器响应未通过 IRQ 通知。驱动使用显式
轮询方法来读取响应。这会带来极小的 CPU 开销，但你不太会注意到。

然而，第二行是一个致命错误。如果发生这种情况，通常意味着确实出了问题。极有可能
是你正在访问一个不存在的编解码器槽位。

因此，如果出现第二条错误消息，请尝试通过 `probe_mask` 选项缩小被探测的编解码器
槽位。它是一个位掩码，每一位对应一个编解码器槽位。例如，要只探测第一个槽位，
传入 `probe_mask=1`。要探测第一和第三个槽位，传入 `probe_mask=5`（其中 5 = 1 | 4），
依此类推。

不过自 2.6.29 内核起，驱动有了更健壮的探测方法，因此这种错误可能很少发生。

在 BIOS 有缺陷的机器上，有时你需要强制驱动去探测硬件未报告的编解码器槽位以供
使用。这种情况下，请打开 `probe_mask` 选项的第 8 位（0x100）。其余 8 位将作为
编解码器槽位无条件传入进行探测。例如，`probe_mask=0x103` 将强制探测编解码器
槽位 0 和 1，无论硬件报告什么。


### 中断处理

自 2.6.33 内核起，HD-audio 驱动默认使用 MSI（若可用），因为 MSI 在某些机器上
表现更好，且一般而言对性能更有利。然而，Nvidia 控制器在 MSI 下出现了严重的
回归（尤其是在与 AMD 芯片组组合时），因此我们为它们禁用了 MSI。

似乎也还有其他一些设备无法与 MSI 配合工作。如果你在近期内核上看到声音质量
（卡顿等）方面的回归或死锁，请尝试传入 `enable_msi=0` 选项以禁用 MSI。如果它
生效了，你可以将有问题的已知设备加入 hda_intel.c 中定义的黑名单。这种情况下，请
报告并将补丁反馈给上游开发者。


## HD-Audio 编解码器


### 模型选项

HD-audio 驱动最常见的问题是不被支持的编解码器特性，或设备配置不匹配。大多数
编解码器专用代码都有若干预设模型，用于覆盖 BIOS 设置或提供更全面的功能。

驱动检查 PCI SSID，并在静态配置表中查找，直到找到匹配项。如果你有一台新机器，
可能会看到如下消息：
```

    hda_codec: ALC880: BIOS auto-probing.

```
与此同时，在较早的版本中，你可能会看到如下消息：
```

    hda_codec: Unknown model for ALC880, trying auto-probe from BIOS...

```
即使你看到这样的消息，也不要惊慌。深呼吸，拿好你的毛巾。首先，这是一条信息性
消息，没有警告，也没有错误。这意味着你的设备的 PCI SSID 没有列在已知的预设模型
（白）名单中。但这并不意味着驱动坏了。许多编解码器驱动提供了基于 BIOS 设置的
自动配置机制。

HD-audio 编解码器通常具有“pin”控件（widget），BIOS 会设置每个 pin 的默认配置，
指示位置、连接类型、插孔颜色等。HD-audio 驱动可以根据这些默认配置值推断出正确的
连接。然而 —— 某些编解码器支持代码（例如 patch_analog.c）并不支持自动探测（截至
2.6.28 仍如此）。而且，BIOS 经常（没错，是相当经常）出错。它设置错误的值并把
驱动搞乱。

预设模型（或近来被称为“fix-up”）基本上是为克服这种情况而提供的。当在白名单中
找到匹配的预设模型时，驱动会假定该预设的静态配置（含正确的 pin 设置等）。因此，
如果你有一台较新机器，其 PCI SSID（或编解码器 SSID）与现有机器略有不同，你很有
机会复用相同的模型。你可以传入 `model` 选项来指定预设模型，而无需进行 PCI（及
编解码器）SSID 查找。

`model` 选项有哪些可用取值取决于编解码器芯片。请从编解码器 proc 文件检查你的
编解码器芯片（见下文“Codec Proc-File”一节）。它会显示你的编解码器芯片的厂商/
产品名称。然后，参见 Documentation/sound/hd-audio/models.rst 文件，即 HD-audio
驱动一节。你可以找到编解码器列表以及属于每个编解码器的 `model` 选项。例如，对于
Realtek ALC262 编解码器芯片，对兼容 Samsung Q1 Ultra 的设备传入 `model=ultra`。

因此，对于任何全新、不受支持且无法工作的 HD-audio 硬件，你首先能做的，就是检查
HD-udio 编解码器以及若干不同的 `model` 选项取值。如果运气好，其中某些可能正好
适合你的设备。

有几个特殊的模型选项取值：

- 传入 'nofixup' 时，会跳过编解码器解析器中的设备专用 fixup。
- 传入 `generic` 时，会跳过编解码器专用解析器，仅使用通用解析器。

自 5.15 内核起引入的 `model` 选项新形式为传入 PCI 或编解码器 SSID，形如
`model=XXXX:YYYY`，其中 XXXX 和 YYYY 分别是子厂商和子设备 ID 的十六进制数。这
是对另一台设备的某种别名；给出这种形式时，驱动将把该 SSID 作为对 quirk 表
的引用。当目标 quirk 未列在模型表中时它尤其有用。例如，传入 model=103c:8862
将应用 HP ProBook 445 G8 的 quirk（截至撰写时它未出现在模型表中），只要该设备
被同一驱动等价地处理即可。


### 扬声器与耳机输出

HD-audio 最常见（也最明显）的缺陷之一是内置扬声器与耳机插孔之一或两者均无声音
输出。一般而言，你应该先尝试耳机输出。扬声器输出通常需要更多额外控制，例如外部
放大器位。因此耳机输出成功的机会稍高一些。

在提交缺陷报告之前，请仔细检查混音器是否设置正确。近期版本的 snd-hda-intel 驱动
提供了大多为“Master”音量控制以及“Front”音量（其中 Front 表示前置声道）。此外，
还可能有单独的“Headphone”和“Speaker”控制。

扬声器输出同理。某些编解码器上可能有“External Amplifier”开关。如果存在，请打开它。

另一个相关的问题是插入耳机时扬声器输出的自动静音。这一特性在多数情况下已实现，
但并非每个预设模型或编解码器支持代码都如此。

无论如何，如果你遇到此类问题，请尝试不同的模型选项。某些其他模型可能更匹配，并
为你提供匹配度更高的功能。如果可用模型都不奏效，请发送缺陷报告。详见缺陷报告一节。

如果你有足够的受虐倾向去调试驱动问题，请注意以下几点：

- 扬声器（以及耳机）输出通常需要外部放大器。这通常可通过 EAPD verb 或某个 GPIO
  设置。如果编解码器 pin 支持 EAPD，你更有可能通过 SET_EAPD_BTL verb (0x70c) 成功。
  其他情况下，GPIO pin（大多为 GPIO0 或 GPIO1）可以打开/关闭 EAPD。
- 某些 Realtek 编解码器需要特殊的厂商专用系数来打开放大器。参见 patch_realtek.c。
- IDT 编解码器可能在每个模拟 pin 上有额外的电源启用/禁用控制。参见 patch_sigmatel.c。
- 非常罕见，但某些设备在被触发之前不接受 pin 探测 verb。发出 GET_PIN_SENSE verb
  (0xf09) 可能导致编解码器通信停滞。一些例子见于 patch_realtek.c。


### 采集问题

采集问题通常源于混音器设置缺失。因此，在提交缺陷报告之前，请确保你正确设置了
混音器。例如，除了正确的“Capture Source”或“Input Source”选择外，“Capture Volume”
和“Capture Switch”都必须正确设置。某些设备有“Mic Boost”音量或开关。

当 PCM 设备通过“default” PCM（不含 pulse-audio 插件）打开时，你可能还会拥有
“Digital Capture Volume”控制。这是为信号提供的额外软件增益/衰减，尤其用于没有
硬件音量控制（如数字麦克风）的输入。除非确实必要，它应设置为精确的 50%，对应 0dB
—— 既无额外增益也无衰减。不过，当你使用“hw” PCM（即原始访问 PCM）时，该控制
不会产生影响。

已知某些编解码器/设备的模拟电路相当糟糕，录制的声音包含一定的直流偏移。这不是
驱动的缺陷。

大多数现代笔记本没有模拟 CD 输入连接。因此，在许多情况下，从 CD 输入录音将无法
工作，尽管驱动将其作为采集源提供。请改用 CDDA。

插入时内置与外接麦克风的自动切换在某些编解码器模型上已实现，但并非每个模型都有。
部分因为我的懒惰，但主要是因为缺少测试者。欢迎向作者提交改进补丁。


### 直接调试

如果没有任何模型选项能给你更好的结果，而你又是个敢于对抗邪恶的硬汉，可以尝试通过
向设备发送原始 HD-audio 编解码器 verb 来调试。有一些工具可用：hda-emu 和
hda-analyzer。详细描述见下文各节。使用这些工具你需要启用 hwdep。参见“内核配置”
一节。


## 其他问题


### 内核配置

一般而言，我建议你启用声音调试选项 `CONFIG_SND_DEBUG=y`，无论你是否在调试。

别忘了打开相应的 `CONFIG_SND_HDA_CODEC_*` 选项。注意每个选项对应编解码器芯片，
而非控制器芯片。因此，即使 lspci 显示的是 Nvidia 控制器，你也可能需要为其他厂商
选择选项。如果不确定，直接全部选 yes 即可。

`CONFIG_SND_HDA_HWDEP` 是一个用于调试驱动的有用选项。启用后，驱动会创建
硬件相关设备（每个编解码器一个），并可通过这些设备文件对设备进行原始访问。例如，
会为第一张卡（#0）的编解码器槽位 #2 创建 `hwC0D2`。对于 hda-verb 和 hda-analyzer
之类的调试工具，必须启用 hwdep 设备。因此，最好始终打开它。

`CONFIG_SND_HDA_RECONFIG` 是一个新选项，它依赖于上面的 hwdep 选项。启用后，你
会在相应的 hwdep 目录下获得一些 sysfs 文件。参见下文“HD-audio 重新配置”一节。

`CONFIG_SND_HDA_POWER_SAVE` 选项启用节能特性。参见下文“节能”一节。


### Codec Proc-File

编解码器 proc 文件是调试 HD-audio 的百宝箱。它显示了每个编解码器控件的大部分有用
信息。

该 proc 文件位于 /proc/asound/card**/codec#**，每个编解码器槽位一个文件。你可以
从中了解编解码器厂商、产品 ID 和名称、每个控件的类型、能力等等。不过，该文件
目前不显示插孔侦测状态。这是因为插孔侦测可能依赖于触发状态。

该文件会被调试工具读取，也可以作为主编解码器信息提供给模拟器。参见下文调试工具
一节。

该 proc 文件也可用于检查是否使用了通用解析器。当使用通用解析器时，厂商/产品 ID
名称会显示为“Realtek ID 0262”，而非“Realtek ALC262”。


### HD-Audio 重新配置

这是一个实验性特性，允许你在不重新加载驱动的情况下动态重新配置 HD-audio 编解码器。
在每个编解码器-hwdep 设备目录下（例如 /sys/class/sound/hwC0D0）提供以下 sysfs 文件：

vendor_id
    显示 32 位编解码器厂商 ID 的十六进制数。你可以通过写入该文件来更改厂商 ID 值。
subsystem_id
    显示 32 位编解码器子系统 ID 的十六进制数。你可以通过写入该文件来更改子系统 ID 值。
revision_id
    显示 32 位编解码器修订 ID 的十六进制数。你可以通过写入该文件来更改修订 ID 值。
afg
    显示 AFG ID。这是只读的。
mfg
    显示 MFG ID。这是只读的。
name
    显示编解码器名称字符串。可通过写入该文件来更改。
modelname
    显示当前设置的 `model` 选项。可通过写入该文件来更改。
init_verbs
    初始化时要执行的额外 verb。你可以通过写入该文件来添加一个 verb。传入三个数字：
    nid、verb 和 parameter（以空格分隔）。
hints
    显示/存储供编解码器解析器任意使用的提示字符串。其格式为 `key = value`。例如，传入
    `jack_detect = no` 将完全禁用机器的插孔侦测。
init_pin_configs
    显示由 BIOS 设置的初始 pin 默认配置值。
driver_pin_configs
    显示由编解码器解析器显式设置的 pin 默认值。它不显示所有 pin 值，而只显示解析器
    更改过的值。也就是说，如果解析器本身未更改 pin 默认配置值，这里将不含任何内容。
user_pin_configs
    显示用于覆盖 BIOS 设置的 pin 默认配置值。写入该文件（带两个数字，NID 和 value）
    会追加新值。在下次重新配置时，将使用所提供的值取代初始 BIOS 值。注意该配置甚至会
    覆盖驱动 pin 配置。
reconfig
    触发编解码器重新配置。当向该文件写入任意值时，驱动会重新初始化并再次解析编解码器
    树。上述 sysfs 条目所做的所有更改都会被考虑在内。
clear
    重置编解码器，移除指定编解码器的混音器元素和 PCM 相关内容，并清除所有 init verb
    和 hints。

例如，当你想把 pin 控件 0x14 的 pin 默认配置值改为 0x9993013f，并让驱动基于该状态
重新配置时，运行如下命令：
```

    # echo 0x14 0x9993013f > /sys/class/sound/hwC0D0/user_pin_configs
    # echo 1 > /sys/class/sound/hwC0D0/reconfig


```
### 提示字符串

编解码器解析器拥有若干开关与调节旋钮，以更好地匹配实际的编解码器或设备行为。其中
许多可以通过上文提到的“hints”字符串动态调整。例如，通过 sysfs 或补丁文件传入
`jack_detect = no` 字符串，你可以禁用插孔侦测，从而编解码器解析器会跳过自动静音或
麦克风自动切换等功能。作为布尔值，可以传入 `yes`、`no`、`true`、`false`、`1` 或 `0`。

通用解析器支持以下 hints：

jack_detect (bool)
    指定本机器上是否提供插孔侦测；默认 true
inv_jack_detect (bool)
    表示插孔侦测逻辑是反转的
trigger_sense (bool)
    表示插孔侦测需要显式调用 AC_VERB_SET_PIN_SENSE verb
inv_eapd (bool)
    表示 EAPD 以反转逻辑实现
pcm_format_first (bool)
    在流标签和通道 ID 之前设置 PCM 格式
sticky_stream (bool)
    尽可能长时间保持 PCM 格式、流标签和 ID；默认 true
spdif_status_reset (bool)
    每次建立 SPDIF 流时重置 SPDIF 状态位
pin_amp_workaround (bool)
    输出 pin 可能有多个 amp 值
single_adc_amp (bool)
    ADC 只能有单一输入 amp
auto_mute (bool)
    启用/禁用耳机自动静音特性；默认 true
auto_mic (bool)
    启用/禁用麦克风自动切换特性；默认 true
line_in_auto_switch (bool)
    启用/禁用 line-in 自动切换特性；默认 false
need_dac_fix (bool)
    根据通道数限制 DAC
primary_hp (bool)
    将耳机插孔作为主输出探测；默认 true
multi_io (bool)
    尝试探测多 I/O 配置（例如共享的 line-in/surround、mic/clfe 插孔）
multi_cap_vol (bool)
    提供多个采集音量
inv_dmic_split (bool)
    为相位反转的数字麦克风提供分离的内置麦克风音量/开关
indep_hp (bool)
    提供独立的耳机 PCM 流及相应的混音器控制（若可用）
add_stereo_mix_input (bool)
    若可用，将立体声混音（模拟回环混音）加入输入 mux
add_jack_modes (bool)
    为每个 I/O 插孔添加“xxx Jack Mode”枚举控制，以允许更改耳机 amp 和麦克风偏置 VREF
    能力
power_save_node (bool)
    每个控件的进阶电源管理，根据实际 pin 和流状态控制每个控件节点（widget）的电源状态
    （D0/D3）
power_down_unused (bool)
    关闭未使用控件的电源，是 power_save_node 的一个子集，未来将被移除
add_hp_mic (bool)
    尽可能将耳机加入采集源
hp_mic_detect (bool)
    针对单一内置麦克风的 hp/mic 共享输入情况，启用/禁用该特性；默认 true
vmaster (bool)
    启用/禁用虚拟 Master 控制；默认 true
mixer_nid (int)
    指定模拟回环混音器的控件 NID


### 早期补丁

当设置了 `CONFIG_SND_HDA_PATCH_LOADER=y` 时，你可以传入一个“patch”固件文件，在
初始化编解码器之前修改 HD-audio 设置。它基本类似于上面通过 sysfs 进行的重新配置，
但它在第一次编解码器配置之前执行。

补丁文件是一个纯文本文件，如下所示：

```

    [codec]
    0x12345678 0xabcd1234 2

    [model]
    auto

    [pincfg]
    0x12 0x411111f0

    [verb]
    0x20 0x500 0x03
    0x20 0x400 0xff

    [hint]
    jack_detect = no


```
该文件需要有一行 `[codec]`。下一行应包含三个数字，分别表示编解码器厂商 ID（示例中
为 0x12345678）、编解码器子系统 ID（0xabcd1234）以及编解码器地址（2）。其余补丁条目
会应用到该指定的编解码器，直到给出另一个编解码器条目。向第一个或第二个值传入 0 或
负数将跳过对应字段的检查。这对于不能正确初始化 SSID 的确实损坏的设备很有用。

`[model]` 行允许更改每个编解码器的模型名。在上例中，它会被改为 model=auto。注意这
会覆盖模块选项。

在 `[pincfg]` 行之后，内容会被解析为初始的默认 pin 配置，就像上面的 `user_pin_configs`
sysfs 一样。这些值也可以在 user_pin_configs sysfs 文件中显示。

类似地，`[verb]` 之后的行被解析为 `init_verbs` sysfs 条目，`[hint]` 之后的行被解析为
`hints` sysfs 条目。

另一个将编解码器厂商 ID 从 0x12345678 覆盖为 0xdeadbeef 的示例如下：
```

    [codec]
    0x12345678 0xabcd1234 2

    [vendor_id]
    0xdeadbeef


```
以类似方式，你可以通过 `[subsystem_id]` 覆盖编解码器 subsystem_id，通过 `[revision_id]`
行覆盖修订 ID。此外，编解码器芯片名可以通过 `[chip_name]` 行重写。
```

    [codec]
    0x12345678 0xabcd1234 2

    [subsystem_id]
    0xffff1111

    [revision_id]
    0x10

    [chip_name]
    My-own NEWS-0002


```
hd-audio 驱动通过 request_firmware() 读取该文件。因此，补丁文件必须位于适当的固件
路径，通常是 /lib/firmware。例如，当你传入选项 `patch=hda-init.fw` 时，文件
/lib/firmware/hda-init.fw 必须存在。

patch 模块选项针对每个卡实例，你需要为每个实例给出一个文件名，以逗号分隔。例如，
如果你有两张卡，一张为板载模拟，一张为 HDMI 视频板，你可以如下传入 patch 选项：
```

    options snd-hda-intel patch=on-board-patch,hdmi-patch


```
### 节能

节能是设备的一种自动挂起。当设备空闲一段时间后，设备会自动关闭以节省电力。进入
挂起的时间通过 `power_save` 模块选项指定，该选项可通过 sysfs 动态更改。

在某些编解码器上，当启用了模拟回环时节能不会工作。当你希望节能时，请确保静音所有
不必要的信号路由。

根据具体设备，节能特性可能在每次断电/上电时产生可听到的咔嗒声。其中某些可能可解，
但有些我恐怕很难解决。某些发行版（如 openSUSE）在拔掉电源线时会自动启用节能特性。
因此，如果你听到噪声，首先怀疑节能。参见 /sys/module/snd_hda_intel/parameters/power_save
以检查当前值。若非零，则该特性已开启。

近期内核也支持 HD-audio 控制器芯片的运行时 PM。这意味着 HD-audio 控制器也会动态
上电/下电。该特性仅对特定控制器芯片（如 Intel LynxPoint）启用。你可以通过设置
`power_save_controller` 选项强制启用/禁用该特性，该选项同样可在
/sys/module/snd_hda_intel/parameters 目录下找到。


### 跟踪点

hd-audio 驱动提供了一些基本的跟踪点（tracepoint）。`hda:hda_send_cmd` 跟踪每次 CORB
写入，而 `hda:hda_get_response` 跟踪来自 RIRB 的响应（仅在从编解码器驱动读取时）。
`hda:hda_bus_reset` 跟踪因致命错误等引起的总线重置，`hda:hda_unsol_event` 跟踪非请求
事件，`hda:hda_power_down` 和 `hda:hda_power_up` 跟踪通过节能行为产生的下电/上电。

启用所有跟踪点可如下进行：
```

    # echo 1 > /sys/kernel/tracing/events/hda/enable

```
然后执行一些命令后，你可以从 /sys/kernel/tracing/trace 文件跟踪。例如，当你想跟踪
发送了什么编解码器命令时，启用如下跟踪点：
```

    # cat /sys/kernel/tracing/trace
    # tracer: nop
    #
    #       TASK-PID    CPU#    TIMESTAMP  FUNCTION
    #          | |       |          |         |
	   <...>-7807  [002] 105147.774889: hda_send_cmd: [0:0] val=e3a019
	   <...>-7807  [002] 105147.774893: hda_send_cmd: [0:0] val=e39019
	   <...>-7807  [002] 105147.999542: hda_send_cmd: [0:0] val=e3a01a
	   <...>-7807  [002] 105147.999543: hda_send_cmd: [0:0] val=e3901a
	   <...>-26764 [001] 349222.837143: hda_send_cmd: [0:0] val=e3a019
	   <...>-26764 [001] 349222.837148: hda_send_cmd: [0:0] val=e39019
	   <...>-26764 [001] 349223.058539: hda_send_cmd: [0:0] val=e3a01a
	   <...>-26764 [001] 349223.058541: hda_send_cmd: [0:0] val=e3901a

```
这里 `[0:0]` 分别表示卡号和编解码器地址，`val` 显示发送给编解码器的值。该值是一个
打包值，你可以通过下面 hda-emu 包中包含的 hda-decode-verb 程序来解码它。例如，值
e3a019 用于将左输出 amp 值设为 25。
```

    % hda-decode-verb 0xe3a019
    raw value = 0x00e3a019
    cid = 0, nid = 0x0e, verb = 0x3a0, parm = 0x19
    raw value: verb = 0x3a0, parm = 0x19
    verbname = set_amp_gain_mute
    amp raw val = 0xa019
    output, left, idx=0, mute=0, val=25


```
### 开发分支

HD-audio 的最新开发代码位于 sound git 树：

- git://git.kernel.org/pub/scm/linux/kernel/git/tiwai/sound.git

master 分支或 for-next 分支通常可作为主要开发分支，而当前与下一内核的开发则分别位于
for-linus 和 for-next 分支。


### 发送缺陷报告

如果任何模型或模块选项对你的设备都无效，就该向开发者发送缺陷报告了。请在你的缺陷
报告中提供以下内容：

- 硬件厂商、产品和模型名称
- 内核版本（如果你在外部构建，还包括 ALSA-driver 版本）
- `alsa-info.sh` 输出；使用 `--no-upload` 选项运行。参见下文关于 alsa-info 的一节

如果是回归问题，最好同时发送可工作与不可工作内核的 alsa-info 输出。这非常有帮助，
因为我们可以直接比较编解码器寄存器。

通过以下任一方式发送缺陷报告：

kernel-bugzilla
    https://bugzilla.kernel.org/
alsa-devel ML
    alsa-devel@alsa-project.org


## 调试工具


本节介绍一些可用于调试 HD-audio 问题的工具。

### alsa-info

`alsa-info.sh` 脚本是一个用于收集音频设备信息非常有用的工具。它包含在 alsa-utils
包中。最新版本可在 git 仓库中找到：

- git://git.alsa-project.org/alsa-utils.git

该脚本也可从以下 URL 直接获取：

- https://www.alsa-project.org/alsa-info.sh

以 root 身份运行此脚本，它会收集重要信息，如模块列表、模块参数、proc 文件内容（包括
编解码器 proc 文件）、混音器输出和控制元素。默认情况下，它会将这些信息存储到
alsa-project.org 上的 Web 服务器。但是，如果你发送缺陷报告，最好使用 `--no-upload`
选项运行，并附上生成的文件。

还有一些其他有用的选项。详见 `--help` 选项的输出。

当发生探测错误或驱动明显分配了不匹配的模型时，以 `probe_only=1` 选项加载驱动（最好
在冷重启后）并在此状态下运行 alsa-info 会很有帮助。使用该选项时，驱动不会配置混音器
和 PCM，而只尝试探测编解码器槽位。探测后 proc 文件可用，因此你可以在驱动修改之前
获取原始编解码器信息。当然，使用 `probe_only=1` 时驱动不可用。但如果启用了 hda-reconfig
选项，你可以通过 hwdep sysfs 文件继续配置。使用 `probe_only` 掩码 2 会跳过 HDA 编解码器
的重置（作为模块选项使用 `probe_only=3`）。hwdep 接口可用于确定 BIOS 编解码器初始化。


### hda-verb

hda-verb 是一个小程序，允许你直接访问 HD-audio 编解码器。你可以用它执行原始 HD-audio
编解码器 verb。该程序访问 hwdep 设备，因此你需要事先启用内核配置 `CONFIG_SND_HDA_HWDEP=y`。

hda-verb 程序接受四个参数：hwdep 设备文件、控件 NID、verb 和参数。当你访问卡 0 槽位 2
上的编解码器时，通常将 /dev/snd/hwC0D2 传给第一个参数。（不过，真实的路径名取决于系统。）

第二个参数是要访问的控件编号 ID。第三个参数可以是一个十六进制/数字，也可以是与 verb
对应的字符串。类似地，最后一个参数是要写入的值，也可以是与参数类型对应的字符串。
```

    % hda-verb /dev/snd/hwC0D0 0x12 0x701 2
    nid = 0x12, verb = 0x701, param = 0x2
    value = 0x0

    % hda-verb /dev/snd/hwC0D0 0x0 PARAMETERS VENDOR_ID
    nid = 0x0, verb = 0xf00, param = 0x0
    value = 0x10ec0262

    % hda-verb /dev/snd/hwC0D0 2 set_a 0xb080
    nid = 0x2, verb = 0x300, param = 0xb080
    value = 0x0


```
尽管你可以用该程序发出任意 verb，但驱动状态并非总是会更新。例如，音量值通常在驱动中
被缓存，因此通过 hda-verb 直接更改控件 amp 值不会改变混音器值。

hda-verb 程序现已包含在 alsa-tools 中：

- git://git.alsa-project.org/alsa-tools.git

此外，旧的独立包可在 ftp 目录中找到：

- ftp://ftp.suse.com/pub/people/tiwai/misc/

也有一个 git 仓库可用：

- git://git.kernel.org/pub/scm/linux/kernel/git/tiwai/hda-verb.git

有关 hda-verb 程序的更多细节，请参见 tarball 中的 README 文件。


### hda-analyzer

hda-analyzer 基于 pyGTK2 绑定，提供图形界面以访问原始 HD-audio 控制。它是 hda-verb 功能
更强的版本。该程序为你提供易用的 GUI，用于显示控件信息和调节 amp 值，以及 proc 兼容输出。

hda-analyzer：

- https://git.alsa-project.org/?p=alsa.git;a=tree;f=hda-analyzer

是 alsa-project.org 上 alsa.git 仓库的一部分：

- git://git.alsa-project.org/alsa.git

### Codecgraph

Codecgraph 是一个实用程序，用于生成图形并可视化编解码器芯片的编解码器节点连接。当你
在没有合适数据手册的情况下分析或调试编解码器时，它尤其有用。该程序解析给定的编解码器
proc 文件，并通过 graphiz 程序转换为 SVG。

tarball 和 GIT 树可在以下网页找到：

- http://helllabs.org/codecgraph/


### hda-emu

hda-emu 是一个 HD-audio 模拟器。该程序的主要目的是在没有真实硬件的情况下调试 HD-audio
编解码器。因此，它不模拟真实音频 I/O 的行为，而只是转储在探测和操作 HD-audio 驱动时
的编解码器寄存器变化和 ALSA 驱动内部变化。

该程序需要一个编解码器 proc 文件来进行模拟。事先获取目标编解码器的 proc 文件，或者从
tarball 中的编解码器 proc 集合中挑选一个示例编解码器。然后，用该 proc 文件运行程序，
hda-emu 程序将开始解析编解码器文件并模拟 HD-audio 驱动：
```

    % hda-emu codecs/stac9200-dell-d820-laptop
    # Parsing..
    hda_codec: Unknown model for STAC9200, using BIOS defaults
    hda_codec: pin nid 08 bios pin config 40c003fa
    ....


```
该程序只提供一个非常简陋的命令行界面。你可以获取当前状态的 proc 文件转储、获取控制
（混音器）元素列表、设置/获取控制元素值、模拟 PCM 操作、插孔插入模拟等。

该程序位于以下 git 仓库：

- git://git.kernel.org/pub/scm/linux/kernel/git/tiwai/hda-emu.git

有关 hda-emu 程序的更多细节，请参见仓库中的 README 文件。


### hda-jack-retask

hda-jack-retask 是一个用户友好的 GUI 程序，用于操控 HD-audio pin 控制以实现插孔重分配。
如果你有关于插孔分配的问题，请尝试该程序，看看能否得到有用的结果。一旦你确定了合适的
pin 分配，它可以通过静态修改驱动代码，或传入固件补丁文件（见“早期补丁”一节）来修正。

该程序现已包含在 alsa-tools 中：

- git://git.alsa-project.org/alsa-tools.git
