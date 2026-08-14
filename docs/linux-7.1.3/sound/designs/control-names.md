锘?# 鏍囧噯 ALSA 鎺т欢鍚嶇О


鏈枃妗ｆ弿杩版贩闊冲櫒鎺т欢鐨勬爣鍑嗗悕绉般€?
### 鏍囧噯璇硶


璇硶锛歔浣嶇疆 LOCATION] 婧?SOURCE [閫氶亾 CHANNEL] [鏂瑰悜 DIRECTION] 鍔熻兘 FUNCTION


#### 鏂瑰悜 DIRECTION

================	===============
<鏃?			涓や釜鏂瑰悜
Playback		鍗曚釜鏂瑰悜
Capture			鍗曚釜鏂瑰悜
Bypass Playback		鍗曚釜鏂瑰悜
Bypass Capture		鍗曚釜鏂瑰悜
================	===============

#### 鍔熻兘 FUNCTION

========	=================================
Switch		寮€鍏筹紙寮€/鍏筹級
Volume		鏀惧ぇ鍣?Route		璺敱鎺у埗锛屼笌纭欢鐩稿叧
========	=================================

#### 閫氶亾 CHANNEL

============	==================================================
<鏃?		涓庨€氶亾鏃犲叧锛屾垨閫傜敤浜庢墍鏈夐€氶亾
Front		鍓嶇疆宸?鍙抽€氶亾
Surround	4.0/5.1 鐜粫涓殑鍚庣疆宸?鍙?CLFE		C/LFE 閫氶亾
Center		涓疆閫氶亾
LFE		LFE 閫氶亾
Side		7.1 鐜粫涓殑渚х疆宸?鍙?============	==================================================

#### 浣嶇疆 LOCATION锛堟簮鐨勭墿鐞嗕綅缃級

============	=====================
Front		鍓嶇疆浣嶇疆
Rear		鍚庣疆浣嶇疆
Dock		浣嶄簬鎵╁睍鍧炰笂
Internal	鍐呴儴
============	=====================

#### 婧?SOURCE

===================	=================================================
Master
Master Mono
Hardware Master
Speaker			鍐呴儴鎵０鍣?Bass Speaker		鍐呴儴 LFE 鎵０鍣?Headphone
Line Out
Beep			铚傞福鍙戠敓鍣?Phone
Phone Input
Phone Output
Synth
FM
Mic
Headset Mic		缁勫悎寮忚€虫満鎻掑瓟锛? 閽堬細鑰虫満 + 楹﹀厠椋庯級涓殑楹﹀厠椋庨儴鍒?Headphone Mic		浜岄€変竴寮忥紙3 閽堬細鑰虫満鎴栭害鍏嬮锛変腑鐨勯害鍏嬮閮ㄥ垎
Line			浠呰緭鍏ワ紝杈撳嚭璇蜂娇鐢?"Line Out"
CD
Video
Zoom Video
Aux
PCM
PCM Pan
Loopback
Analog Loopback		D/A -> A/D 鍥炵幆
Digital Loopback		鍥炴斁 -> 鎹曡幏鍥炵幆 - 涓嶇粡杩囨ā鎷熻矾寰?Mono
Mono Output
Multi
ADC
Wave
Music
I2S
IEC958
HDMI
SPDIF			浠呰緭鍑?SPDIF In
Digital In
HDMI/DP			 HDMI 鎴?DisplayPort 涔嬩竴
===================	=================================================

### 渚嬪锛堝凡搴熷純锛?

=====================================	=======================
[Analogue|Digital] Capture Source
[Analogue|Digital] Capture Switch	鍗宠緭鍏ュ鐩婂紑鍏?[Analogue|Digital] Capture Volume	鍗宠緭鍏ュ鐩婇煶閲?[Analogue|Digital] Playback Switch	鍗宠緭鍑哄鐩婂紑鍏?[Analogue|Digital] Playback Volume	鍗宠緭鍑哄鐩婇煶閲?Tone Control - Switch
Tone Control - Bass
Tone Control - Treble
3D Control - Switch
3D Control - Center
3D Control - Depth
3D Control - Wide
3D Control - Space
3D Control - Level
Mic Boost [锛坉B)]
=====================================	=======================

### PCM 鎺ュ彛


===================	========================================
Sample Clock Source	{ "Word", "Internal", "AutoSync" }
Clock Sync Status	{ "Lock", "Sync", "No Lock" }
External Rate		澶栭儴鎹曡幏閫熺巼
Capture Rate		浠庡閮ㄦ簮鑾峰彇鐨勬崟鑾烽€熺巼
===================	========================================

### IEC958 (S/PDIF) 鎺ュ彛


============================================	======================================
IEC958 [...] [Playback|Capture] Switch		鎵撳紑/鍏抽棴 IEC958 鎺ュ彛
IEC958 [...] [Playback|Capture] Volume		鏁板瓧闊抽噺鎺у埗
IEC958 [...] [Playback|Capture] Default		榛樿鎴栧叏灞€鍊?- 璇?鍐?IEC958 [...] [Playback|Capture] Mask		娑堣垂绫讳笌涓撲笟绫绘帺鐮?IEC958 [...] [Playback|Capture] Con Mask	娑堣垂绫绘帺鐮?IEC958 [...] [Playback|Capture] Pro Mask	涓撲笟绫绘帺鐮?IEC958 [...] [Playback|Capture] PCM Stream	璧嬬粰鏌愪釜 PCM 娴佺殑璁剧疆
IEC958 Q-subcode [Playback|Capture] Default	Q-subcode 浣?
IEC958 Preamble [Playback|Capture] Default	绐佸彂鍓嶅瀛楋紙4*16bits锛?============================================	======================================
