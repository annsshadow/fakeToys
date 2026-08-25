## ASoC 数字音频接口（DAI

ASoC 目前支持如今 SoC 控制器与便携式音CODEC 上常见的三种主要数字音频接口
（DAI），AC97、I2S PCM

## AC97


AC97 是一种五线接口，常见于许PC 声卡上。现在在许多便携式设备中也颇为流行DAI 有一RESET 线，并在SDATA_OUT（回放）SDATA_IN（采集）线上数据进行时分复用。位时钟（BCLK）总是CODEC 驱动（通常12.288MHz），帧（FRAME）（通常48kHz）总是由控制器驱动。每AC97 帧长 21uS，并分为
13 个时隙
AC97 规范可在以下地址找到https://www.intel.com/p/en_US/business/design


## I2S


I2S 是一种常见于 HiFi、STB 与便携式设备的四DAI。Tx Rx 线用于音频传输，
而位时钟（BCLK）与右时钟（LRC）用于同步链路。I2S 的灵活性在于控制器CODEC 都可以驱动（作为主设备）BCLK LRC 时钟线。位时钟通常随采样率与主
系统时钟（SYSCLK）而变化。LRCLK 与采样率相同。少数设备支持独立的 ADC DAC LRCLK，这允许以不同的采样率同时进行采集与回放
I2S 有几种不同的工作模式
I2S
  MSB LRC 跳变后的第一BCLK 的下降沿发送
Left Justified
  MSB LRC 跳变时发送
Right Justified
  MSB LRC 跳变sample size BCLK 时发送
## PCM


PCM 是另一种四线接口，I2S 非常相似，但可以支持更灵活的协议。它有位时钟
（BCLK）与同步（SYNC）线用于同步链路，Tx Rx 线用于发送与接收音频数据位时钟通常随采样率变化，而同步运行在采样率上。PCM 还支持时分复用（TDM），
即多个设备可以同时使用总线（有时称为网络模式）
常见PCM 工作模式
Mode A
  MSB FRAME/SYNC 之后的第一BCLK 的下降沿发送
Mode B
  MSB FRAME/SYNC 的上升沿发送