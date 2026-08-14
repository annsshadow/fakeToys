## 标准 ALSA 控件名称


本文档描述混音器控件的标准名称。

### 标准语法


语法：[位置 LOCATION] 源 SOURCE [通道 CHANNEL] [方向 DIRECTION] 功能 FUNCTION


#### 方向 DIRECTION

================	===============
<无>			两个方向
Playback		单个方向
Capture			单个方向
Bypass Playback		单个方向
Bypass Capture		单个方向
================	===============

#### 功能 FUNCTION

========	=================================
Switch		开关（开/关）
Volume		放大器
Route		路由控制，与硬件相关
========	=================================

#### 通道 CHANNEL

============	==================================================
<无>		与通道无关，或适用于所有通道
Front		前置左/右通道
Surround	4.0/5.1 环绕中的后置左/右
CLFE		C/LFE 通道
Center		中置通道
LFE		LFE 通道
Side		7.1 环绕中的侧置左/右
============	==================================================

#### 位置 LOCATION（源的物理位置）

============	=====================
Front		前置位置
Rear		后置位置
Dock		位于扩展坞上
Internal	内部
============	=====================

#### 源 SOURCE

===================	=================================================
Master
Master Mono
Hardware Master
Speaker			内部扬声器
Bass Speaker		内部 LFE 扬声器
Headphone
Line Out
Beep			蜂鸣发生器
Phone
Phone Input
Phone Output
Synth
FM
Mic
Headset Mic		组合式耳机插孔（4 针：耳机 + 麦克风）中的麦克风部分
Headphone Mic		二选一式（3 针：耳机或麦克风）中的麦克风部分
Line			仅输入，输出请使用 "Line Out"
CD
Video
Zoom Video
Aux
PCM
PCM Pan
Loopback
Analog Loopback		D/A -> A/D 回环
Digital Loopback		回放 -> 捕获回环 - 不经过模拟路径
Mono
Mono Output
Multi
ADC
Wave
Music
I2S
IEC958
HDMI
SPDIF			仅输出
SPDIF In
Digital In
HDMI/DP			 HDMI 或 DisplayPort 之一
===================	=================================================

### 例外（已废弃）


=====================================	=======================
[Analogue|Digital] Capture Source
[Analogue|Digital] Capture Switch	即输入增益开关
[Analogue|Digital] Capture Volume	即输入增益音量
[Analogue|Digital] Playback Switch	即输出增益开关
[Analogue|Digital] Playback Volume	即输出增益音量
Tone Control - Switch
Tone Control - Bass
Tone Control - Treble
3D Control - Switch
3D Control - Center
3D Control - Depth
3D Control - Wide
3D Control - Space
3D Control - Level
Mic Boost [(?dB)]
=====================================	=======================

### PCM 接口


===================	========================================
Sample Clock Source	{ "Word", "Internal", "AutoSync" }
Clock Sync Status	{ "Lock", "Sync", "No Lock" }
External Rate		外部捕获速率
Capture Rate		从外部源获取的捕获速率
===================	========================================

### IEC958 (S/PDIF) 接口


============================================	======================================
IEC958 [...] [Playback|Capture] Switch		打开/关闭 IEC958 接口
IEC958 [...] [Playback|Capture] Volume		数字音量控制
IEC958 [...] [Playback|Capture] Default		默认或全局值 - 读/写
IEC958 [...] [Playback|Capture] Mask		消费类与专业类掩码
IEC958 [...] [Playback|Capture] Con Mask	消费类掩码
IEC958 [...] [Playback|Capture] Pro Mask	专业类掩码
IEC958 [...] [Playback|Capture] PCM Stream	赋给某个 PCM 流的设置
IEC958 Q-subcode [Playback|Capture] Default	Q-subcode 位

IEC958 Preamble [Playback|Capture] Default	突发前导字（4*16bits）
============================================	======================================
