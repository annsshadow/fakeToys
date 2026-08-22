## ALSA dapm 创建 codec codec dai link


大多数情况下，音频流总是CPU codec，因此你的系统看起来如下```

   ---------          ---------
  |         |  dai   |         |
      CPU    ------->    codec
  |         |        |         |
   ---------          ---------

```
如果你的系统看起来如下：
```

                       ---------
                      |         |
                        codec-2
                      |         |
                      ---------
                           |
                         dai-2
                           |
   ----------          ---------
  |          |  dai-1 |         |
      CPU     ------->  codec-1
  |          |        |         |
   ----------          ---------
                           |
                         dai-3
                           |
                       ---------
                      |         |
                        codec-3
                      |         |
                       ---------

```
假设 codec-2 是一个蓝牙芯片，codec-3 连接到一个扬声器，并且你有以下场景：
codec-2 将接收音频数据，而用户希望不经过 CPU 就通过 codec-3 播放该音频。上述情况正是应该使codec codec 连接的理想情形
你的 dai_link 在你的机器文件中应如下所示：
```

 /*
  * pcm 流仅支持 24 bit 通道  * 48k 采样率  */
 static const struct snd_soc_pcm_stream dsp_codec_params = {
        .formats = SNDRV_PCM_FMTBIT_S24_LE,
        .rate_min = 48000,
        .rate_max = 48000,
        .channels_min = 2,
        .channels_max = 2,
 };

 {
    .name = "CPU-DSP",
    .stream_name = "CPU-DSP",
    .cpu_dai_name = "samsung-i2s.0",
    .codec_name = "codec-2,
    .codec_dai_name = "codec-2-dai_name",
    .platform_name = "samsung-i2s.0",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
            | SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    .c2c_params = &dsp_codec_params,
    .num_c2c_params = 1,
 },
 {
    .name = "DSP-CODEC",
    .stream_name = "DSP-CODEC",
    .cpu_dai_name = "wm0010-sdi2",
    .codec_name = "codec-3,
    .codec_dai_name = "codec-3-dai_name",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
            | SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    .c2c_params = &dsp_codec_params,
    .num_c2c_params = 1,
 },

```
上述代码片段的灵感来sound/soc/samsung/speyside.c
注意 “c2c_params回调，它dapm 知道dai_link 是一codec codec 的连接
dapm 核心中，会在 cpu_dai 播放（playback）widget codec_dai 捕获（capture）widget 之间创建一条路由用于播放路径，反之亦然用于捕获路径。为了使上述这条路由被触发，DAPM 需要找到一个有效的端点，该端点可以分别是对应于播放和捕获路径的 sink source widget
为了触发dai_link widget，可以为扬声器放大器创建一个轻量的 codec 驱动，如 wm8727.c 文件所示，即使不需要任何控制，它也会为设备设置适当的约束
确保将相应的 cpu codec 播放与捕dai 名称分别“Playback“Capture结尾命名，因dapm 核心会根据名称链接并为这dai 供电
“simple-audio-card中，当链接上的所DAI 都属codec 组件时，dai_link 会被自动识别codec codec。该 dai_link 将使用链接上所DAI 支持的流参数（通道数、格式、采样率）的子集进行初始化。由于在设备树中无法提供这些参数，这主要用于与简单的固定功能 codec 通信，例如蓝牙控制器或蜂窝调制解调器