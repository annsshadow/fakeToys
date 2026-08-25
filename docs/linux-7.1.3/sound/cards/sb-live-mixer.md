## Sound Blaster Live 混音/ 默认 DSP 代码


EMU10K1 芯片有一DSP 部分，可以通过编程支持多种采样处理的方式，此处予以描述（本文不涉及 EMU10K1 芯片的整体功能。更多细节请参阅手册章节。）

ALSA 驱动默认以默认代码（之后可更改）对该芯片部分进行编程，提供以下功能：


## IEC958 (S/PDIF) 原始 PCM


PCM 设备（对于给定声卡，它是3 PCM 设备（索2！）和第一个子设备（索0））
允许48kHz、立体声6 位小端（little endian）流原样转发到数字输出（同轴或光纤）。通用
接口允许创建最8 个以 48kHz6 位小端运行的原始 PCM 设备。向当前代码添加对多通道设备支持很容易，但转换例程目前仅针对立体声（2 声道流）存在
有关更多细节，请参阅 lowlevel/emu10k1/emufx.c 中的 tram_poke 例程
## 数字混音器控

这些控制使用 DSP 指令构建。它们提供扩展功能。此处仅描述 ALSA 驱动中默认的嵌入式代码注意，这些控制用作衰减器（attenuator）：最大值为中性位置，保持信号不变。注意，如果同一
目标在多个控制中被提及，信号会被累加并可能被削波（clip，在不检查溢出的情况下设为最大或
最小值）
所用缩写解释：

DAC
	数模转换器（digital to analog converterADC
	模数转换器（analog to digital converterI2S
	飞利浦半导体（Philips Semiconductors）用于数字声音的单向三线制串行总线
	（该标准用于连接独立D/A A/D 转换器）
LFE
	低频效果（low frequency effects，用作低音炮信号AC97
	包含模拟混音器、D/A A/D 转换器的芯片
IEC958
	S/PDIF
FX-bus
	EMU10K1 芯片有一个包16 个累加器（accumulator）的效果总线（effect bus）	每个合成器声音（voice）都可以将其输出馈送到这些累加器，DSP 微控制器可以	结果求和进行计算
### ``name='Wave Playback Volume',index=0``


该控制用于衰减来自左PCM FX-bus 累加器的采样。ALSA 对左PCM 采样使用累加0 1结果采样被转发到 AC97 编解码器的前置（front）DAC PCM 槽位
### ``name='Wave Surround Playback Volume',index=0``


该控制用于衰减来自左PCM FX-bus 累加器的采样。ALSA 对左PCM 采样使用累加0 1结果采样被转发到后置（rear）I2S DAC。这DAC 独立运行（它们不AC97 编解码器内部）
### ``name='Wave Center Playback Volume',index=0``


该控制用于衰减来自左PCM FX-bus 累加器的采样。ALSA 对左PCM 采样使用累加0 1结果被混为单声道（单通道）信号并转发AC97 编解码器rear DAC PCM 槽位
### ``name='Wave LFE Playback Volume',index=0``


该控制用于衰减来自左PCM FX-bus 累加器的采样。ALSA 对左PCM 使用累加0 1。结被混为单声道（单通道）信号并转发AC97 编解码器rear DAC PCM 槽位
### ``name='Wave Capture Volume',index=0``, ``name='Wave Capture Switch',index=0``


这些控制用于衰减来自左右 PCM FX-bus 累加器的采样。ALSA 对左PCM 使用累加0 1。结被转发到 ADC 捕获 FIFO（从而到标准捕获 PCM 设备）
### ``name='Synth Playback Volume',index=0``


该控制用于衰减来自左MIDI FX-bus 累加器的采样。ALSA 对左MIDI 采样使用累加4 5结果采样被转发到 AC97 编解码器的前DAC PCM 槽位
### ``name='Synth Capture Volume',index=0``, ``name='Synth Capture Switch',index=0``


这些控制用于衰减来自左右 MIDI FX-bus 累加器的采样。ALSA 对左MIDI 采样使用累加4 5结果被转发到 ADC 捕获 FIFO（从而到标准捕获 PCM 设备）
### ``name='Surround Playback Volume',index=0``


该控制用于衰减来自左右后PCM FX-bus 累加器的采样。ALSA 对左右后PCM 采样使用累加2 3结果采样被转发到后置 I2S DAC。这DAC 独立运行（它们不AC97 编解码器内部）
### ``name='Surround Capture Volume',index=0``, ``name='Surround Capture Switch',index=0``


这些控制用于衰减来自左右后置 PCM FX-bus 累加器的采样。ALSA 对左右后PCM 采样使用累加2 3结果被转发到 ADC 捕获 FIFO（从而到标准捕获 PCM 设备）
### ``name='Center Playback Volume',index=0``


该控制用于衰减中PCM FX-bus 累加器的采样。ALSA 对中PCM 采样使用累加6。结果采样被
转发AC97 编解码器rear DAC PCM 槽位
### ``name='LFE Playback Volume',index=0``


该控制用于衰减中PCM FX-bus 累加器的采样。ALSA 对中PCM 采样使用累加6。结果采样被
转发AC97 编解码器rear DAC PCM 槽位
### ``name='AC97 Playback Volume',index=0``


该控制用于衰减来AC97 编解码器左右前置 ADC PCM 槽位的采样。结果采样被转发AC97 编解码器
的前DAC PCM 槽位
  该控制在标准操作中应为零，否则会激活数字回环（loopback）
### ``name='AC97 Capture Volume',index=0``


该控制用于衰减来AC97 编解码器左右前置 ADC PCM 槽位的采样。结果被转发ADC 捕获 FIFO
（从而到标准捕获 PCM 设备）
   该控制应100（最大值），否AC97 编解码器的模拟输入无法被捕获（录制）
### ``name='IEC958 TTL Playback Volume',index=0``


该控制用于衰减来自左IEC958 TTL 数字输入（通常CDROM 驱动器使用）的采样。结果采样被
转发AC97 编解码器的前DAC PCM 槽位
### ``name='IEC958 TTL Capture Volume',index=0``


该控制用于衰减来自左IEC958 TTL 数字输入（通常CDROM 驱动器使用）的采样。结果采样被
转发ADC 捕获 FIFO（从而到标准捕获 PCM 设备）
### ``name='Zoom Video Playback Volume',index=0``


该控制用于衰减来自左zoom video 数字输入（通常CDROM 驱动器使用）的采样。结果采样被
转发AC97 编解码器的前DAC PCM 槽位
### ``name='Zoom Video Capture Volume',index=0``


该控制用于衰减来自左zoom video 数字输入（通常CDROM 驱动器使用）的采样。结果采样被
转发ADC 捕获 FIFO（从而到标准捕获 PCM 设备）
### ``name='IEC958 LiveDrive Playback Volume',index=0``


该控制用于衰减来自左IEC958 光纤数字输入的采样。结果采样被转发AC97 编解码器的前DAC PCM 槽位
### ``name='IEC958 LiveDrive Capture Volume',index=0``


该控制用于衰减来自左IEC958 光纤数字输入的采样。结果采样被转发ADC 捕获 FIFO（从而到
标准捕获 PCM 设备）
### ``name='IEC958 Coaxial Playback Volume',index=0``


该控制用于衰减来自左IEC958 同轴数字输入的采样。结果采样被转发AC97 编解码器的前DAC PCM 槽位
### ``name='IEC958 Coaxial Capture Volume',index=0``


该控制用于衰减来自左IEC958 同轴数字输入的采样。结果采样被转发ADC 捕获 FIFO（从而到
标准捕获 PCM 设备）
### ``name='Line LiveDrive Playback Volume',index=0``, ``name='Line LiveDrive Playback Volume',index=1``


该控制用于衰减来自左I2S ADC 输入（在 LiveDrive 上）的采样。结果采样被转发AC97 编解码器
的前DAC PCM 槽位
### ``name='Line LiveDrive Capture Volume',index=1``, ``name='Line LiveDrive Capture Volume',index=1``


该控制用于衰减来自左I2S ADC 输入（在 LiveDrive 上）的采样。结果采样被转发ADC 捕获
FIFO（从而到标准捕获 PCM 设备）
### ``name='Tone Control - Switch',index=0``


该控制打开或关闭音调（tone）控制。前置、后置以及中/ LFE 输出的采样会受影响
### ``name='Tone Control - Bass',index=0``


该控制设置低音（bass）强度。没有中性值！！当音调控制代码被激活时，采样总是被修改。最接近
纯净信号的值是 20
### ``name='Tone Control - Treble',index=0``


该控制设置高音（treble）强度。没有中性值！！当音调控制代码被激活时，采样总是被修改。最
接近纯净信号的值是 20
### ``name='IEC958 Optical Raw Playback Switch',index=0``


如果该开关打开，则 IEC958 (S/PDIF) 数字输出的采样仅取自原始 FX8010 PCM，否则取自标准前PCM 采样
### ``name='Headphone Playback Volume',index=1``


该控制衰减耳机输出的采样
### ``name='Headphone Center Playback Switch',index=1``


如果该开关打开，则中心 PCM 的采样被送到左耳机输出（对没有独立中心/LFE 输出SB Live 声卡
有用）
### ``name='Headphone LFE Playback Switch',index=1``


如果该开关打开，则中心 PCM 的采样被送到右耳机输出（对没有独立中心/LFE 输出SB Live 声卡
有用）
## PCM 流相关的控制


### ``name='EMU10K1 PCM Volume',index 0-31``


通道音量衰减，范0-0x1fffd。中间值（无衰减）为默认。三个值的通道映射如下
- 0 - 单声道，默认 0xffff（无衰减- 1 - 左，默认 0xffff（无衰减- 2 - 右，默认 0xffff（无衰减
### ``name='EMU10K1 PCM Send Routing',index 0-31``


该控制指定目标——FX-bus 累加器。包含十二个值，映射如下
- 0 -  单声道，A 目标（FX-bus 0-15），默认 0
- 1 -  单声道，B 目标（FX-bus 0-15），默认 1
- 2 -  单声道，C 目标（FX-bus 0-15），默认 2
- 3 -  单声道，D 目标（FX-bus 0-15），默认 3
- 4 -  左，A 目标（FX-bus 0-15），默认 0
- 5 -  左，B 目标（FX-bus 0-15），默认 1
- 6 -  左，C 目标（FX-bus 0-15），默认 2
- 7 -  左，D 目标（FX-bus 0-15），默认 3
- 8 -  右，A 目标（FX-bus 0-15），默认 0
- 9 -  右，B 目标（FX-bus 0-15），默认 1
- 10 - 右，C 目标（FX-bus 0-15），默认 2
- 11 - 右，D 目标（FX-bus 0-15），默认 3

不要忘记：将某个通道多次分配给同一FX-bus 累加器是非法的（这意味着 0=0 && 1=0 是无组合）
### ``name='EMU10K1 PCM Send Volume',index 0-31``


它指定给定目标在范围 0-255 内的衰减（量）。通道映射如下
- 0 -  单声道，A 目标衰减，默255（无衰减- 1 -  单声道，B 目标衰减，默255（无衰减- 2 -  单声道，C 目标衰减，默0（静音）
- 3 -  单声道，D 目标衰减，默0（静音）
- 4 -  左，A 目标衰减，默255（无衰减- 5 -  左，B 目标衰减，默0（静音）
- 6 -  左，C 目标衰减，默0（静音）
- 7 -  左，D 目标衰减，默0（静音）
- 8 -  右，A 目标衰减，默0（静音）
- 9 -  右，B 目标衰减，默255（无衰减- 10 - 右，C 目标衰减，默0（静音）
- 11 - 右，D 目标衰减，默0（静音）


## 手册/专利


### ftp://opensource.creative.com/pub/doc


注意该站点已停用，但文档可从其他多个位置获取
LM4545.pdf
	AC97 编解码器
m2049.pdf
	EMU10K1 数字音频处理hog63.ps
	FX8010 - 用于音频效果DSP 芯片架构

### WIPO 专利


WO 9901813 (A1)
	具有多个异步流的音频效果处理	999 1 14 日）

WO 9901814 (A1)
	具有音频效果指令集的处理器（1999 1 14 日）

WO 9901953 (A1)
	具有解耦指令执行与音频数据排序的音频效果处理器
        999 1 14 日）

### 美国专利 (https://www.uspto.gov/)


US 5925841
	采用高速缓存存储器的数字采样乐器（1999 7 20 日）

US 5928342
	集成在单芯片上的音频效果处理        带有一个多端口存储器，多个异步数字声音采样可并发加载到其上
	999 7 27 日）

US 5930158
	具有音频效果指令集的处理器（1999 7 27 日）

US 6032235
	存储器初始化电路（Tram）（2000 2 29 日）

US 6138207
	缓存中音频采样的插值循环，连接到系统总线，依据循环尾端和最小块大小
        对总线传输进行优先级排序与修改
	000 10 24 日）

US 6151670
	使用短期存储器寄存器池以节省内存存储的方	000 11 21 日）

US 6195715
	多个程序通过关联程序GP 寄存器、定义中断寄存器、轮GP 寄存器并调用
        与所定义中断寄存器关联的回调例程，来与公共中断通信的中断控	001 2 27 日）
