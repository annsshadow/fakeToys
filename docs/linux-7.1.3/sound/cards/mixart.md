## Digigram miXart8 与 miXart8AES/EBU 声卡的 Alsa 驱动


Digigram <alsa@digigram.com>

## 概述


miXart8 是一款多声道音频处理与混音声卡，具有 4 个立体声音频输入与 4 个立体声音频输出。
miXart8AES/EBU 与之相同，但增加了一块附加卡，提供额外的 4 个数字立体声音频输入与输出。
此外，该附加卡提供外部时钟同步（AES/EBU、Word Clock、Time Code 与 Video Synchro）。

主板上有一个 PowerPC，提供板载 mpeg 编码与解码、采样率转换以及各种效果。

在加载特定固件之前，驱动根本无法正常工作，即不会出现任何 PCM 或混音器设备。
请使用 alsa-tools 软件包中的 mixartloader。

## 版本 0.1.0


一块 miXart8 板会被表示为 4 个 alsa 卡，每个卡有 1 个立体声模拟采集 'pcm0c' 与 1 个立体声模拟
回放 'pcm0p' 设备。对于 miXart8AES/EBU，每块卡另外还有 1 个立体声数字输入 'pcm1c' 与 1 个立体声
数字输出 'pcm1p'。

### 格式


U8、S16_LE、S16_BE、S24_3LE、S24_3BE、FLOAT_LE、FLOAT_BE
采样率：8000 - 48000 Hz 连续

### 回放


例如，回放设备被配置为最多 4 个子流执行硬件混音。如果需要，这可以更改为最多 24 个子流。
单声道文件将在左、右声道播放。每个声道都可以为每个流静音，以单独使用 8 个模拟/数字输出。

### 采集


每个采集设备有一个子流。例如仅支持立体声格式。

### 混音器


<Master> 与 <Master Capture>
	回放与采集 PCM 的模拟音量控制。
<PCM 0-3> 与 <PCM Capture>
	每个模拟子流的数字音量控制。
<AES 0-3> 与 <AES Capture>
	每个 AES/EBU 子流的数字音量控制。
<Monitoring>
	从 'pcm0c' 到 'pcm0p' 的环回，带数字音量与静音控制。

注意：为获得最佳音质，尽量让 PCM 与 AES 音量控制保持 0 衰减，即在 0 到 255 范围内设为 219
（使用 alsamixer 约为 86%）。

## 尚未实现


- 外部时钟支持（AES/EBU、Word Clock、Time Code、Video Sync）
- MPEG 音频格式
- 单声道录音
- 板载效果与采样率转换
- 链接流

## 固件


[自 2.6.11 起，当设置了 CONFIG_FW_LOADER 时，固件可以通过热插拔自动加载。mixartloader 仅对
较旧版本或将驱动编译进内核时是必需的。]

要在模块加载后自动加载固件，请使用 install 命令。例如，将以下条目添加到 miXart 驱动的
/etc/modprobe.d/mixart.conf：
```

	install snd-mixart /sbin/modprobe --first-time -i snd-mixart && \
			   /usr/bin/mixartloader


```
（对于 2.2/2.4 内核，改为将 "post-install snd-mixart /usr/bin/vxloader" 添加到
/etc/modules.conf。）

固件二进制文件安装在 /usr/share/alsa/firmware（或 /usr/local/share/alsa/firmware，取决于
configure 的 prefix 选项）。其中会有一个 miXart.conf 文件，定义 dsp 映像文件。

固件文件的版权归 Digigram SA 所有。

## 版权


Copyright (c) 2003 Digigram SA <alsa@digigram.com>
可在 GPL 下分发。
