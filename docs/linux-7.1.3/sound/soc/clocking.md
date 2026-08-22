## 音频时钟


本文描述 ASoC 以及数字音频中通用的音频时钟术语。注意：音频时钟可能很复杂！


### 主时

每个音频子系统都由一个主时钟（有时称MCLK SYSCLK）驱动。这个音频主时钟可以来自多个源头（例如晶振、PLL、CPU 时钟），并负责产生正确的音频播放与采集采样率
某些主时钟（例如 PLL 和基CPU 的时钟）是可配置的，其速度可由软件改变（取决于系统用途并用于省电）。其他主时钟则以固定频率运行（即晶振）

### DAI 时钟

数字音频接口通常由一个位时钟（常称为 BCLK）驱动。该时钟用于驱动 codec CPU 之间的链路上的数字音频数据
DAI 还有一个帧时钟用于标记每个音频帧的开始。该时钟有时称为 LRC（左/右时钟）FRAME。该时钟以与采样率完全相同的频率运行（LRC = Rate）
位时钟可按如下方式生成：

- BCLK = MCLK / x，或
- BCLK = LRC * x，或
- BCLK = LRC ** Channels ** Word Size

这种关系取决于具体的 codec SoC CPU。一般来说，最好将 BCLK 配置为尽可能低的速度（取决于你的速率、通道数和字长）以节省功耗
如果可能，通常更希望使codec 来驱动（或作为主设备）音频时钟，因为它通常CPU 提供更精确的采样率
### ASoC 提供的时API


   :identifiers: snd_soc_dai_set_sysclk

   :identifiers: snd_soc_dai_set_clkdiv

   :identifiers: snd_soc_dai_set_pll

   :identifiers: snd_soc_dai_set_bclk_ratio
