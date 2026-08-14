## ALSA PCM 时间戳


ALSA API 可以提供两种不同的系统时间戳：

- Trigger_tstamp（触发时间戳）是在 .trigger 回调被调用时获取的系统时间快照。在一般情况下，该快照由 ALSA 核心获取，但特定硬件可能具备同步能力，或者相反地，只能延迟一段时间后提供正确的估计值。在后两种情况下，底层驱动负责在最合适、最精确的时刻更新 trigger_tstamp。应用程序不应仅依赖第一个 trigger_tstamp，而应在驱动带有延迟地提供经过提炼的估计值时，更新其内部计算。

- tstamp（时间戳）是上一次事件或应用程序查询期间更新的当前系统时间戳。
  差值（tstamp - trigger_tstamp）定义了经过的时间。

ALSA API 提供两条基本信息：avail（可用空间）和 delay（延迟），它们与触发时间戳和当前系统时间戳相结合，使应用程序能够跟踪环形缓冲区的“填充度”以及已排队样本的数量。

使用这些不同指针和时间信息的方式取决于应用程序的需求：

- `avail` 报告环形缓冲区中还可以写入多少数据
- `delay` 报告在播放完所有已排队样本后，听到一个新样本所需的时间。

当启用时间戳时，avail/delay 信息会随系统时间快照一起报告。应用程序可以从 `CLOCK_REALTIME`（包含 NTP 校正，包括回退）、`CLOCK_MONOTONIC`（包含 NTP 校正但从不回退）、`CLOCK_MONOTIC_RAW`（不包含 NTP 校正）中选择，并通过 sw_params 动态更改模式。


ALSA API 还提供 audio_tstamp（音频时间戳），它反映由音频硬件不同组件测得的时间流逝。用 ascii 示意图表示如下（以播放为例）：
```

  --------------------------------------------------------------> time
    ^               ^              ^                ^           ^
    |               |              |                |           |
   analog         link            dma              app       FullBuffer
   time           time           time              time        time
    |               |              |                |           |
    |< codec delay >|<--hw delay-->|<queued samples>|<---avail->|
    |<----------------- delay---------------------->|           |
                                   |<----ring buffer length---->|


```
模拟时间（analog time）在播放的最后一级获取，尽可能接近实际的换能器（transducer）。

链路时间（link time）在 SoC/芯片组的输出处获取，此时样本正被推送到链路上。如果硬件支持，链路时间可以通过样本计数器或墙上时钟（例如 HDAudio 的 24MHz 时钟，或网络化方案的 PTP 时钟）直接测量，也可以通过间接方式估计（例如使用 USB 中的帧计数器）。

DMA 时间通过计数器测量——由于 DMA 传输的突发特性，它通常是所有测量中最不可靠的。

应用时间（app time）对应应用程序写入环形缓冲区后所跟踪的时间。

应用程序可以查询硬件能力，通过选择 audio_tstamp_config 字段中的相关设置来定义希望报告的音频时间，从而估算时间戳的精度。它还可以要求在测量中包含到模拟端的延迟。在提供嵌入式 DSP 的平台上，直接访问链路时间非常有意义；使用专用硬件直接测量链路时间（可能与系统时间同步），就无需再跟踪内部 DSP 的处理时间和延迟。

如果应用程序请求的音频时间戳在硬件/底层驱动中不受支持，则该类型会被覆盖为 DEFAULT，时间戳将基于 hw_pointer 值报告 DMA 时间。

为了与未提供时间戳选择的早期实现保持向后兼容，当使用零值的 COMPAT 时间戳类型时，播放流的结果将默认使用 HDAudio 墙上时钟，其他所有情况下则使用 DMA 时间（hw_ptr）。

音频时间戳的精度可以返回给用户空间，以便做出适当的决策：

- 对于 DMA 时间（默认），传输的粒度可以从更新之间的间隔推断出来，进而提供关于应用程序指针可以安全回退多少的信息。

- 链路时间可用于通过 (tstamp-trigger_tstamp)/audio_tstamp 比值来跟踪音频时间与系统时间之间的长期漂移，其精度有助于确定需要多少平滑/低通滤波。链路时间可以在启动时复位，也可以按原样报告（后者对于比较不同流的进度很有用——但可能要求墙上时钟始终运行，且在空闲期间不会回绕）。如果硬件支持，绝对链路时间也可以用于定义精确的启动时间（补丁开发中）。

- 在音频时间戳中包含延迟可能会反直觉地不会提高时间戳的精度，例如，如果编解码器包含可变延迟的 DSP 处理，或者由一串硬件组件组成，则延迟通常无法精确获知。

精度以纳秒为单位报告（使用一个无符号 32 位字），最大精度为 4.29 秒，对音频应用来说绰绰有余……

由于时间戳需求的多样性，即便是对于单个应用程序，audio_tstamp_config 也可以动态更改。在 `STATUS` ioctl 中，参数是只读的，不允许任何应用程序选择。为了在不影响遗留应用程序的情况下规避这一限制，引入了一个新的 `STATUS_EXT` ioctl，其参数为可读写。ALSA-lib 将被修改以使用 `STATUS_EXT`，从而实际上弃用 `STATUS`。

ALSA API 一次只允许报告单个音频时间戳。这是一个有意的设计决定，因为从硬件寄存器或 IPC 读取音频时间戳需要时间，读取的时间戳越多，合并测量的精度就越低。为避免任何解释上的问题，只报告一个（系统，音频）时间戳。需要不同时间戳的应用程序必须发出多次查询并对结果进行插值。

在某些特定硬件配置中，系统时间戳由底层音频子系统锁存，并将信息提供回驱动。由于与硬件通信可能存在延迟，存在与 avail 和 delay 信息错位的风险。为确保应用程序不被混淆，在 snd_pcm_status 结构体中增加了一个 driver_timestamp 字段；该时间戳显示了驱动在从 `STATUS` 和 `STATUS_EXT` ioctl 返回之前将信息汇总在一起的时间。在大多数情况下，这个 driver_timestamp 与常规的系统 tstamp 相同。

使用 HDAudio 的时间戳示例：

1. DMA 时间戳，不补偿 DMA+模拟延迟```

  $ ./audio_time  -p --ts_type=1
  playback: systime: 341121338 nsec, audio time 342000000 nsec, 	systime delta -878662
  playback: systime: 426236663 nsec, audio time 427187500 nsec, 	systime delta -950837
  playback: systime: 597080580 nsec, audio time 598000000 nsec, 	systime delta -919420
  playback: systime: 682059782 nsec, audio time 683020833 nsec, 	systime delta -961051
  playback: systime: 852896415 nsec, audio time 853854166 nsec, 	systime delta -957751
  playback: systime: 937903344 nsec, audio time 938854166 nsec, 	systime delta -950822

```
2. DMA 时间戳，补偿 DMA+模拟延迟
```

  $ ./audio_time  -p --ts_type=1 -d
  playback: systime: 341053347 nsec, audio time 341062500 nsec, 	systime delta -9153
  playback: systime: 426072447 nsec, audio time 426062500 nsec, 	systime delta 9947
  playback: systime: 596899518 nsec, audio time 596895833 nsec, 	systime delta 3685
  playback: systime: 681915317 nsec, audio time 681916666 nsec, 	systime delta -1349
  playback: systime: 852741306 nsec, audio time 852750000 nsec, 	systime delta -8694

```
3. 链路时间戳，补偿 DMA+模拟延迟
```

  $ ./audio_time  -p --ts_type=2 -d
  playback: systime: 341060004 nsec, audio time 341062791 nsec, 	systime delta -2787
  playback: systime: 426242074 nsec, audio time 426244875 nsec, 	systime delta -2801
  playback: systime: 597080992 nsec, audio time 597084583 nsec, 	systime delta -3591
  playback: systime: 682084512 nsec, audio time 682088291 nsec, 	systime delta -3779
  playback: systime: 852936229 nsec, audio time 852940916 nsec, 	systime delta -4687
  playback: systime: 938107562 nsec, audio time 938112708 nsec, 	systime delta -5146

```
示例 1 表明，DMA 级别的时间戳比实际播放时间超前约 1ms（顺便说一句，这类测量有助于定义回退保护措施）。在示例 2 中补偿 DMA-链路延迟有助于消除硬件缓冲，但信息仍然非常抖动，误差最多可达一个样本。在示例 3 中，时间戳是用链路墙上时钟测量的，显示出单调的行为和更低的离散度。

示例 3 和 4 针对 USB 音频类。示例 3 由于缓冲而显示出音频时间与系统时间之间存在较大的偏移。示例 4 展示了补偿延迟如何暴露出 1ms 的精度（得益于驱动使用了帧计数器）。

示例 3：DMA 时间戳，不补偿延迟，delta 约 5ms
```

  $ ./audio_time -p -Dhw:1 -t1
  playback: systime: 120174019 nsec, audio time 125000000 nsec, 	systime delta -4825981
  playback: systime: 245041136 nsec, audio time 250000000 nsec, 	systime delta -4958864
  playback: systime: 370106088 nsec, audio time 375000000 nsec, 	systime delta -4893912
  playback: systime: 495040065 nsec, audio time 500000000 nsec, 	systime delta -4959935
  playback: systime: 620038179 nsec, audio time 625000000 nsec, 	systime delta -4961821
  playback: systime: 745087741 nsec, audio time 750000000 nsec, 	systime delta -4912259
  playback: systime: 870037336 nsec, audio time 875000000 nsec, 	systime delta -4962664

```
示例 4：DMA 时间戳，补偿延迟，延迟约 1ms
```

  $ ./audio_time -p -Dhw:1 -t1 -d
  playback: systime: 120190520 nsec, audio time 120000000 nsec, 	systime delta 190520
  playback: systime: 245036740 nsec, audio time 244000000 nsec, 	systime delta 1036740
  playback: systime: 370034081 nsec, audio time 369000000 nsec, 	systime delta 1034081
  playback: systime: 495159907 nsec, audio time 494000000 nsec, 	systime delta 1159907
  playback: systime: 620098824 nsec, audio time 619000000 nsec, 	systime delta 1098824
  playback: systime: 745031847 nsec, audio time 744000000 nsec, 	systime delta 1031847

