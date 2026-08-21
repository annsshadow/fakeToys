## ALSA Jack 中使M-Audio Audiophile USB 指南


v1.5

Thibault Le Meur <Thibault.LeMeur@supelec.fr>

本文档是一份在 ALSA JACK 中使M-Audio Audiophile USB (tm) 设备的指南
## 历史


- v1.4 - Thibault Le Meur (2007-07-11)

  - 增补Hakan Lennestal <Hakan.Lennestal@brfsodrahamn.se> 发现16 位模式的低字节序（Little Endian）特  - 调整文档结构

- v1.5 - Thibault Le Meur (2007-07-12)
  - 增补 AC3/DTS 透传（passthru）信

## Audiophile USB 规格与正确使

本部分提醒关于该设备功能与限制的重要事实
该设备有 4 个音频接口，以及 2 MIDI 端口
 - 模拟立体声输入（Ai
   - 该端口支持两对线路电平音频输入（1/4" TS RCA   - 当连1/4" TS（jack）连接器时，RCA 连接器被禁用

 - 模拟立体声输出（Ao - 数字立体声输入（Di - 数字立体声输出（Do - MIDI 输入（Mi - MIDI 输出（Mo
内部 DAC/ADC 具有以下特性：

- 16 位或 24 位采样深- 8kHz 96kHz 采样- 两个接口不能同时采用不同的采样深度
此外，Audiophile USB 文档给出了以下警告：
  在切换位深之前，请退出任何正在运行的音频应用程序

由于 USB 1.1 带宽限制，根据所选音频模式，可同时激活的接口数量有限
 - 16 48kHz ==> 4 通道输入 + 4 通道输出

   - Ai+Ao+Di+Do

 - 24 48kHz ==> 4 通道输入 + 2 通道输出   2 通道输入 + 4 通道输出

   - Ai+Ao+Do 鎴?Ai+Di+Ao 鎴?Ai+Di+Do 鎴?Di+Ao+Do

 - 24 96kHz ==> 2 通道输入 _或_ 2 通道输出（仅半双工）

   - Ai 鎴?Ao 鎴?Di 鎴?Do

### 关于数字接口的重要事实：


 - Do 端口额外支持环绕编码AC-3 DTS 透传   不过我尚未在 Linux 下测试过

   - 注意，在此配置下只有 Do 接口可被启用

 - 除了录制数字音频流，启用 Di 端口也是一种将设备同步到外部采样时钟的方式

   - 因此，只有在连接了有效数字源时才应启Di 端口
   - 在未连接数字源时启用 Di 可能导致同步错误（例如以异常的采样率播放声音

## ALSA 中的 Audiophile USB MIDI 支持


一旦加载以下模块，Audiophile USB MIDI 端口将被自动支持
 - snd-usb-audio
 - snd-seq-midi

无需额外设置

## ALSA 中的 Audiophile USB 音频支持


Audiophile USB 设备的音频功能由 snd-usb-audio 模块处理。该模块可以工作在默认模式（不带任何设备特定参数），或带有名`device_setup` 的设备特定参数的"高级"模式
### 默认 ALSA 驱动模式


snd-usb-audio 驱动的默认行为是在启动时列出设备能力，并在应用程序需要时激活所需模式：例如，如果用户先以 24 位深度模式录音，紧接着又想切换16 位深度模式，snd-usb-audio 模块会即时重新配置设备
这种方式的优点是让驱动根据用户需要自动在采样位深之间切换。然而，Windows 下使用过该设备的人都知道，这并不是设备预期的工作方式：在 Windows 下必须先关闭应用程序，才能使M-Audio 控制面板切换设备工作模式。因此正如我们在下一节将看到的，这种默认 ALSA 驱动模式可能导致设备配置错误
言归正传，回到默认 ALSA 驱动模式。在此情况下，Audiophile 接口按如下方式映射到 ALSA PCM 设备（假设设备索引为 1）：

 - hw:1,0 在回放时Ao，在捕获时为 Di
 - hw:1,1 在回放时Do，在捕获时为 Ai
 - hw:1,2 AC3/DTS 透传模式Do

在此模式下，设备使用大端（Big Endian）字节编码，因此支持的音频格式为 16 位深度模式的 S16_BE 24 位深度模式的 S24_3BE
一个例外是 hw:1,2 端口，曾被报告为符合小端（Little Endian）（应支S16_LE），但实际上只处S16_BE 流。此问题已在内核 2.6.23 及以上版本中修复，现在在此默认驱动模式下 hw:1,2 接口被报告为大端
示例
```

   % aplay -D hw:1,0 -c2 -t raw -r48000 -fS24_3BE test.raw

 * Ai 端口录制一S24_3BE 编码的裸文件::

   % arecord -D hw:1,1 -c2  -t raw -r48000 -fS24_3BE test.raw

 * Do 端口播放一S16_BE 编码的裸文件::

   % aplay -D hw:1,1 -c2 -t raw -r48000 -fS16_BE test.raw

 * Do 端口播放一ac3 示例文件::

   % aplay -D hw:1,2 --channels=6 ac3_S16_BE_encoded_file.raw

```
如果你对默认 ALSA 驱动模式满意，并且此模式下没有任何问题，可以跳过下一章
### 高级模块设置


由于上述硬件限制，ALSA 驱动在默认模式下对设备的初始化可能导致设备处于损坏状态。例如，一个特别恼人的问题是，Ai 接口捕获的声音听起来失真（仿佛被过高的音量增益增强）
对于有此问题的用户，snd-usb-audio 模块新增了一个名`device_setup` 的模块参数（该参数在 2.6.17 内核版本中引入）

#### 初始Audiophile USB 的工作模

Audiophile USB 设备而言，此值让用户指定
 - 采样深度
 - 采样 - 是否使用 Di 端口

当以 `device_setup=0x00` 初始化时，snd-usb-audio 模块的行为与省略该参数时相同（见上文"默认 ALSA 驱动模式"一节）

其他模式在以下小节中描述
#### 16 位模

支持两种模式
 - `device_setup=0x01`

   - 禁用 Di 16 48kHz 模式
   - Ai、Ao、Do 可同时使   - hw:1,0 在捕获模式下不可   - hw:1,2 不可
 - `device_setup=0x11`

   - 启用 Di 16 48kHz 模式
   - Ai、Ao、Di、Do 可同时使   - hw:1,0 在捕获模式下可用
   - hw:1,2 不可
在此模式下，设备仅工作于 16 位模式。在内核 2.6.23 之前，设备被报告为大端（Big-Endian），而实际上它们是小端（Little-Endian），因此播放文件需要使用：
```

   % aplay -D hw:1,1 -c2 -t raw -r48000 -fS16_BE test_S16_LE.raw

```
其中 "test_S16_LE.raw" 实际上是一个小端样本文件
感谢 Hakan Lennestal（他在这些模式下发现了设备的低字节序特性），一个修复已提交（预期进入内2.6.23），ALSA 现在报告为小端接口。因此现在播放文件只需简单地使用```

   % aplay -D hw:1,1 -c2 -t raw -r48000 -fS16_LE test_S16_LE.raw


```
#### 24 位模

支持三种模式
 - `device_setup=0x09`

   - 禁用 Di 24 48kHz 模式
   - Ai、Ao、Do 可同时使   - hw:1,0 在捕获模式下不可   - hw:1,2 不可
 - `device_setup=0x19`

   - 启用 Di 24 48kHz 模式
   - 可同时使{Ai,Ao,Di,Do} 中的 3 个端   - hw:1,0 在捕获模式下可用，且必须Di 连接一个有效的数字   - hw:1,2 不可
 - `device_setup=0x0D` 鎴?`0x10`

   - 24 96kHz 模式
   - 此模式默认启Di，但无需连接到有效源
   - 同一时间只能使用 {Ai,Ao,Di,Do} 中的 1 个端   - hw:1,0 在捕获模式下可用
   - hw:1,2 不可
在这些模式下，设备仅符合大端（Big-Endian）（aplay 命令示例见上默认 ALSA 驱动模式"
#### DTS 透传AC3 模式


感谢 Hakan Lennestal，我现在收到一份报告称此模式可用
 - `device_setup=0x03`

   - 仅启Do 端口16 48kHz 模式
   - DTS 透传AC3
   - 注意，在此配置下 Do 端口被映射到 PCM 设备 hw:1,0

在此模式下用于回AC3/DTS 编码.wav 文件的命令行```

   % aplay -D hw:1,0 --channels=6 ac3_S16_LE_encoded_file.raw

```
#### 如何使用 ``device_setup`` 参数


可通过以下方式给出该参数：

```

   # modprobe -r snd-usb-audio
   # modprobe snd-usb-audio index=1 device_setup=0x09

 * 或在你的模块配置文件中配置模块选项时（通常/etc/modprobe.d/ 目录下的一.conf 文件:::

       alias snd-card-1 snd-usb-audio
       options snd-usb-audio index=1 device_setup=0x09

```
### 初始化设备时的注意事

 - 正确初始化设备要device_setup 在设备通电之前交给模块。因此，如果你使用上述的"手动探测"方法，请务必在初始化之后再给设备上电
 - 未能遵守这一点将导致设备配置错误。此时请关闭设备，卸snd-usb-audio 模块，然后用正确device_setup 参数再次探测，之后（且仅在此之后）重新打开设备
 - 如果你已正确地将设备初始化为某个有效模式，随后想切换到另一个模式（可能采用不同的采样深度），也请使用以下流程：

   - 首先关闭设备
   - 注销 snd-usb-audio 模块（modprobe -r   - 通过修改 `/etc/modprobe.d/*.conf` 中的 device_setup 选项来改device_setup 参数
   - 打开设备

 - 针对最后一个问题的变通方案已应用于内2.6.23，但它可能不足以保证设备初始化的"稳定
### 面向黑客的技术细

本节面向想要理解设备内部细节以及 ALSA 如何支持它的黑客
#### Audiophile USB ``device_setup`` 结构


如果你想理解 Audiophile USB device_setup 幻数，需要一些非常基础的二进制运算知识。不过，使用此参数并不要求了解这些，你可以跳过本节
device_setup 长度为一个字节，其结构如下：
```

       +---+---+---+---+---+---+---+---+
       | b7| b6| b5| b4| b3| b2| b1| b0|
       +---+---+---+---+---+---+---+---+
       | 0 | 0 | 0 | Di|24B|96K|DTS|SET|
       +---+---+---+---+---+---+---+---+

```
其中
 - b0 鏄?`SET` 浣。
   - 如果初始化了 device_setup，则必须设置此位

 - b1 鏄?`DTS` 浣。
   - 仅在使用 DTS/AC3 的数字输出时设置
   - 此配置未经测
 - b2 是速率选择标志

   - `1` 时，速率范围48.1-96kHz
   - 否则采样率范围为 8-48kHz

 - b3 是位深选择标志

   - `1` 时样本为 24    - 否则16    - 注意 b2 隐含 b3，因96kHz 模式仅支24 位样
 - b4 是数字输入标
   - `1` 时，设备假定已连接一个有效的数字   - 如果端口上看不到任何源，则不应启Di（这会导致同步问题）
   - b4 b2 隐含（因为一次只启用一个端口，不会发生同步错误
 - b5 b7 保留供将来使用，必须设为 `0`

   - 可能分别成为 b7、b6、b4 对应Ao、Do、Ai

注意
 - 对你给出device_setup 值没有任何检
   - 例如选择 0x056 96kHz）会回退0x09，因b2 隐含 b3。但 /var/log/messages 中_不会有任何警告_

 - 由于 USB 总线限制带来的硬件约束未被检
   - 选择 b2 将为所有接口准24 96kHz，但同一时间你只能使用一
#### 该设备的 USB 实现细节


如果你对驱动 hacking 不感兴趣，可以放心跳过本节
本节描述该设备的一些内部方面，并总结了我通过 usb-snooping（嗅探）Windows Linux 驱动得到的数据
M-Audio Audiophile USB 7 USB 接口一"USB 接口"
 - USB 接口0
 - USB 接口1

   - 音频控制功能

 - USB 接口2

   - 模拟输出

 - USB 接口3

   - 数字输出

 - USB 接口4

   - 模拟输入

 - USB 接口5

   - 数字输入

 - USB 接口6

   - 符合 MIDIMAN quirk MIDI 接口

每个接口5 个备用设置（AltSet 1,2,3,4,5），例外如下
 - 接口 3（数字输出）有一个额外的 AltSet 6
 - 接口 5（数字输入）没有 AltSet 3 5

以下是对AltSet 能力的简要说明：

- AltSet 1 对应

  - 24 位深度，48.1-96kHz 采样模式
  - 自适应回放（Ao Do），同步捕获（Ai），或异步捕获（Di
- AltSet 2 对应

  - 24 位深度，8-48kHz 采样模式
  - 异步捕获与回放（Ao、Ai、Do、Di
- AltSet 3 对应

  - 24 位深度，8-48kHz 采样模式
  - 同步捕获（Ai）与自适应回放（Ao、Do
- AltSet 4 对应

  - 16 位深度，8-48kHz 采样模式
  - 异步捕获与回放（Ao、Ai、Do、Di
- AltSet 5 对应

  - 16 位深度，8-48kHz 采样模式
  - 同步捕获（Ai）与自适应回放（Ao、Do
- AltSet 6 对应

  - 16 位深度，8-48kHz 采样模式
  - 同步回放（Do），音频格式类型 III IEC1937_AC-3

为了确保设备正确初始化，驱动**必须**知道设备将如何使用：

 - 如果选择DTS，则只注册带 AltSet 6 的接2
 - 如果96KHz，则每个接口只能选择 AltSet 1
 - 如果样本使用 24 48KHz，则在连接数字输入时使用 AltSet 2，未连接数字输入时仅使用 AltSet 3
 - 如果样本使用 16 48KHz，则在连接数字输入时使用 AltSet 4，未连接数字输入时仅使用 AltSet 5

device_setup 作为 snd-usb-audio 模块的参数给出时，parse_audio_endpoints 函数使用一个名`audiophile_skip_setting_quirk` quirk，以防止device_setup 不对应的 AltSet 被注册到驱动中
## Audiophile USB Jack 支持


本节讨论 Audiophile USB 设备Jack 中的支持
使用 Jackd 配合该设备时有两大潜在问题：

- 24 位模式下对大端设备的支持
- 4 / 4 出通道的支
### Jackd 中的直接支持


Jack 仅在较新版本中支持大端设备（感谢 Andreas Steinmetz 的第一个大端补丁）。我记不清该支持确切何时进入 jackd，只能说jackd 0.103.0 版本中基本可用（只是有一个小 bug 影响 16 位大端设备，但既然你已仔细阅读了上面各段，你现在使用的是 >= 2.6.23 的内核，而你16 位设备现在是小端的了 ;-) ）
你可以用以下命令运行 jackd，以 Ao 回放、Ai 录制```

  % jackd -R -dalsa -Phw:1,0 -r48000 -p128 -n2 -D -Chw:1,1

```
### 使用 ALSA plughw


如果你没有安装较新的 Jackd，可以退而使ALSA `plug` 转换器
例如，以下是一种以 Ao 2 个回放通道、Ai 2 个捕获通道运行 Jack 的方式：
```

  % jackd -R -dalsa -dplughw:1 -r48000 -p256 -n2 -D -Cplughw:1,1

```
但你可能会看到以下警告信息：
  你似乎正在使ALSA 软件 "plug" 层，这很可能是使用了 ALSA "default" 设备的结果。它的效率不如应有的高。建议使用硬件设备，而不是使plug 层
### Jack 中获2 个输入和/或输出接

如你所见，以这种方式启Jack 服务器只会启1 个立体声输入（Di Ai）和 1 个立体声输出（Ao Do）
这是由于以下限制
- Jack 一次只能打开一个捕获设备和一个回放设- Audiophile USB 被看2（或 3）个 ALSA 设备：hw:1,0、hw:1,1（以及可选的 hw:1,2
如果你想Jack 中获Ai+Di Ao+Do 的支持，需要将多个 ALSA 设备组合成一个逻辑上的"复合"设备
如果你想尝试，我建议阅读此页面的信息：http://www.sound-man.co.uk/linuxaudio/ice1712multi.html
它与另一个设备（ice1712）相关，但可以改造以适配 Audiophile USB
Jackd 启用多个 Audiophile USB 接口肯定需要：

- 确保你的 Jackd 版本带有 MMAP_COMPLEX 补丁（见 ice1712 页面- （可能需要）修补 alsa-lib/src/pcm/pcm_multi.c 文件（见 ice1712 页面- .asoundrc 文件中定义一multi 设备（hw:1,0 hw:1,1 的组合）
- 用该设备启动 jackd

我目前尚未测试成功，如果你在这种配置上取得成功，请给我发封邮件