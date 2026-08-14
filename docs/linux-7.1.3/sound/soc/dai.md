## ASoC 数字音频接口（DAI）


ASoC 目前支持如今 SoC 控制器与便携式音频 CODEC 上常见的三种主要数字音频接口
（DAI），即 AC97、I2S 与 PCM。


## AC97


AC97 是一种五线接口，常见于许多 PC 声卡上。现在在许多便携式设备中也颇为流行。
该 DAI 有一条 RESET 线，并在其 SDATA_OUT（回放）与 SDATA_IN（采集）线上对
数据进行时分复用。位时钟（BCLK）总是由 CODEC 驱动（通常为 12.288MHz），而
帧（FRAME）（通常为 48kHz）总是由控制器驱动。每个 AC97 帧长 21uS，并分为
13 个时隙。

AC97 规范可在以下地址找到：
https://www.intel.com/p/en_US/business/design


## I2S


I2S 是一种常见于 HiFi、STB 与便携式设备的四线 DAI。Tx 与 Rx 线用于音频传输，
而位时钟（BCLK）与左/右时钟（LRC）用于同步链路。I2S 的灵活性在于控制器或
CODEC 都可以驱动（作为主设备）BCLK 与 LRC 时钟线。位时钟通常随采样率与主
系统时钟（SYSCLK）而变化。LRCLK 与采样率相同。少数设备支持独立的 ADC 与
DAC LRCLK，这允许以不同的采样率同时进行采集与回放。

I2S 有几种不同的工作模式：

I2S
  MSB 在 LRC 跳变后的第一个 BCLK 的下降沿发送。

Left Justified
  MSB 在 LRC 跳变时发送。

Right Justified
  MSB 在 LRC 跳变前 sample size 个 BCLK 时发送。

## PCM


PCM 是另一种四线接口，与 I2S 非常相似，但可以支持更灵活的协议。它有位时钟
（BCLK）与同步（SYNC）线用于同步链路，而 Tx 与 Rx 线用于发送与接收音频数据。
位时钟通常随采样率变化，而同步运行在采样率上。PCM 还支持时分复用（TDM），
即多个设备可以同时使用总线（有时称为网络模式）。

常见的 PCM 工作模式：

Mode A
  MSB 在 FRAME/SYNC 之后的第一个 BCLK 的下降沿发送。

Mode B
  MSB 在 FRAME/SYNC 的上升沿发送。
