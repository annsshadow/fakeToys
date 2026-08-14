## ALSA 中的跟踪点

2017/07/02
Takasahi Sakamoto

## ALSA PCM 核心中的跟踪点

ALSA PCM 核心向内核跟踪点系统注册了 `snd_pcm` 子系统。该子系统包含两类跟踪点：
一类用于 PCM 缓冲区的状态，另一类用于 PCM 硬件参数的处理。当启用相应的内核配置
时，这些跟踪点才可用。当启用 `CONFIG_SND_DEBUG` 时，后一类跟踪点可用。当还启用了
`SND_PCM_XRUN_DEBUG` 时，前一类跟踪点也会被启用。

### 用于 PCM 缓冲区状态的跟踪点

该类别包含四个跟踪点：`hwptr`、`applptr`、`xrun` 和 `hw_ptr_error`。

### 用于 PCM 硬件参数处理的跟踪点

该类别包含两个跟踪点：`hw_mask_param` 和 `hw_interval_param`。

在 ALSA PCM 核心的设计中，数据传输被抽象为 PCM 子流（substream）。应用程序管理
PCM 子流以维护 PCM 帧的数据传输。在开始数据传输之前，应用程序需要配置 PCM 子流。
在此过程中，PCM 硬件参数由应用程序与 ALSA PCM 核心之间的交互来决定。一旦决定，
PCM 子流的运行时（runtime）就会保存这些参数。

这些参数在 struct snd_pcm_hw_params 中描述。该结构体包含几种类型的参数。应用程序
为这些参数设置偏好的值，然后执行带 SNDRV_PCM_IOCTL_HW_REFINE 或
SNDRV_PCM_IOCTL_HW_PARAMS 的 ioctl(2)。前者仅用于精简可用的参数集合，后者用于
实际决定参数。

struct snd_pcm_hw_params 结构体具有以下成员：

`flags`
        可配置。ALSA PCM 核心和某些驱动会处理该标志，以选择方便的参数或改变其行为。
`masks`
        可配置。这类参数在 struct snd_mask 中描述，表示掩码值。截至 PCM 协议
        v2.0.13，定义了三种类型。

        - SNDRV_PCM_HW_PARAM_ACCESS
        - SNDRV_PCM_HW_PARAM_FORMAT
        - SNDRV_PCM_HW_PARAM_SUBFORMAT
`intervals`
        可配置。这类参数在 struct snd_interval 中描述，表示带范围的值。截至
        PCM 协议 v2.0.13，定义了十二种类型。

        - SNDRV_PCM_HW_PARAM_SAMPLE_BITS
        - SNDRV_PCM_HW_PARAM_FRAME_BITS
        - SNDRV_PCM_HW_PARAM_CHANNELS
        - SNDRV_PCM_HW_PARAM_RATE
        - SNDRV_PCM_HW_PARAM_PERIOD_TIME
        - SNDRV_PCM_HW_PARAM_PERIOD_SIZE
        - SNDRV_PCM_HW_PARAM_PERIOD_BYTES
        - SNDRV_PCM_HW_PARAM_PERIODS
        - SNDRV_PCM_HW_PARAM_BUFFER_TIME
        - SNDRV_PCM_HW_PARAM_BUFFER_SIZE
        - SNDRV_PCM_HW_PARAM_BUFFER_BYTES
        - SNDRV_PCM_HW_PARAM_TICK_TIME
`rmask`
        可配置。仅在带 SNDRV_PCM_IOCTL_HW_REFINE 的 ioctl(2) 中求值。应用程序
        可以选择哪些掩码/区间参数可以由 ALSA PCM 核心更改。对于
        SNDRV_PCM_IOCTL_HW_PARAMS，该掩码会被忽略，所有参数都将被更改。
`cmask`
        只读。从 ioctl(2) 返回后，用户空间中用于 struct snd_pcm_hw_params 的
        缓冲区包含每次操作的结果。该掩码表示实际更改了哪个掩码/区间参数。
`info`
        只读。以 SNDRV_PCM_INFO_XXX 位标志表示硬件/驱动能力。通常，应用程序
        执行带 SNDRV_PCM_IOCTL_HW_REFINE 的 ioctl(2) 来检索该标志，然后决定
        参数的候选值，并执行带 SNDRV_PCM_IOCTL_HW_PARAMS 的 ioctl(2) 来配置
        PCM 子流。
`msbits`
        只读。该值表示 PCM 样本中 MSB 一侧可用的位宽。当
        SNDRV_PCM_HW_PARAM_SAMPLE_BITS 参数被决定为一个固定数值时，该值也会
        据此计算出来。否则为零。但该行为取决于驱动侧的实现。
`rate_num`
        只读。该值表示分数表示法中采样率的分子。基本上，当 SNDRV_PCM_HW_PARAM_RATE
        参数被决定为单一值时，该值也会据此计算出来。否则为零。但该行为取决于
        驱动侧的实现。
`rate_den`
        只读。该值表示分数表示法中采样率的分母。基本上，当 SNDRV_PCM_HW_PARAM_RATE
        参数被决定为单一值时，该值也会据此计算出来。否则为零。但该行为取决于
        驱动侧的实现。
`fifo_size`
        只读。该值表示硬件串行音频接口中 FIFO 的大小。基本上，每个驱动都可以
        为该参数分配合适的值，但某些驱动出于对硬件设计或数据传输协议的考虑
        会故意设为零。

当应用程序执行带 SNDRV_PCM_IOCTL_HW_REFINE 或 SNDRV_PCM_IOCTL_HW_PARAMS 的
ioctl(2) 时，ALSA PCM 核心会处理 struct snd_pcm_hw_params 的缓冲区。缓冲区中的
参数会根据 struct snd_pcm_hardware 以及运行时中的约束规则而改变。该结构体描述
所处理硬件的能力。这些规则描述了参数依据若干参数被决定的依赖关系。一条规则带有
一个回调函数，驱动可以注册任意函数来计算目标参数。ALSA PCM 核心会默认向运行时
注册一些规则。

只要驱动在 struct snd_pcm_ops.open 的回调中准备好了两件事，就可以参与这一交互。

1. 在该回调中，驱动应当依据相应硬件的能力，改变运行时中 struct snd_pcm_hardware
   类型的成员。
2. 在同一个回调中，当若干参数因硬件设计而存在依赖关系时，驱动还应当向运行时
   注册额外的约束规则。

驱动可以在 struct snd_pcm_ops.hw_params 的回调中引用交互的结果，但不应更改其内容。

该类别中的跟踪点旨在追踪掩码/区间参数的变化。当 ALSA PCM 核心更改它们时，会根据
所更改参数的类型探测到 `hw_mask_param` 或 `hw_interval_param` 事件。

ALSA PCM 核心还为每个跟踪点提供了漂亮的打印格式。下面是 `hw_mask_param` 的示例。

```

    hw_mask_param: pcmC0D0p 001/023 FORMAT 00000000000000000000001000000044 00000000000000000000001000000044

```
下面是 `hw_interval_param` 的示例。

```

    hw_interval_param: pcmC0D0p 000/023 BUFFER_SIZE 0 0 [0 4294967295] 0 1 [0 4294967295]

```
前三个字段是通用的。它们依次表示 ALSA PCM 字符设备的名称、约束规则以及被更改
参数的名称。约束规则字段由两个子字段组成：所应用规则的索引，以及添加到运行时的
规则总数。作为例外，索引 000 表示该参数由 ALSA PCM 核心更改，与规则无关。

其余字段表示参数更改之前/之后的状态。这些字段根据参数的类型而不同。对于掩码类型
的参数，这些字段表示该参数内容的十六进制转储。对于区间类型的参数，这些字段按
此顺序表示 struct snd_interval 中 `empty`、`integer`、`openmin`、`min`、`max`、
`openmax` 各成员的值。

## 驱动中的跟踪点

某些驱动为了开发者的便利提供了跟踪点。关于它们，请参考各自的文档或实现。
