## 为 ALSA dapm 创建 codec 到 codec 的 dai link


大多数情况下，音频流总是从 CPU 到 codec，因此你的系统看起来如下：
```

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
codec-2 将接收音频数据，而用户希望不经过 CPU 就通过 codec-3 播放该音频。上述情况正是应该使用 codec 到 codec 连接的理想情形。

你的 dai_link 在你的机器文件中应如下所示：
```

 /*
  * 此 pcm 流仅支持 24 bit、2 通道和
  * 48k 采样率。
  */
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
上述代码片段的灵感来自 sound/soc/samsung/speyside.c。

注意 “c2c_params” 回调，它让 dapm 知道此 dai_link 是一个 codec 到 codec 的连接。

在 dapm 核心中，会在 cpu_dai 播放（playback）widget 和 codec_dai 捕获（capture）widget 之间创建一条路由用于播放路径，反之亦然用于捕获路径。为了使上述这条路由被触发，DAPM 需要找到一个有效的端点，该端点可以分别是对应于播放和捕获路径的 sink 或 source widget。

为了触发此 dai_link widget，可以为扬声器放大器创建一个轻量的 codec 驱动，如 wm8727.c 文件所示，即使不需要任何控制，它也会为设备设置适当的约束。

确保将相应的 cpu 和 codec 播放与捕获 dai 名称分别以 “Playback” 和 “Capture” 结尾命名，因为 dapm 核心会根据名称链接并为这些 dai 供电。

在 “simple-audio-card” 中，当链接上的所有 DAI 都属于 codec 组件时，该 dai_link 会被自动识别为 codec 到 codec。该 dai_link 将使用链接上所有 DAI 支持的流参数（通道数、格式、采样率）的子集进行初始化。由于在设备树中无法提供这些参数，这主要用于与简单的固定功能 codec 通信，例如蓝牙控制器或蜂窝调制解调器。
