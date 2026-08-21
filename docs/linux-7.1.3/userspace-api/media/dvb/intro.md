
######## Introduction



## 你需要了解的内容


阅读本文档的读者需要掌握数字视频广播（Digital TV）领域的一些知识，并且应当熟悉 MPEG2 规范I 部分 ISO/IEC 13818（即 ITU-T H.222），也就是说，你应该知道什么是节目 / 传输流（PS/TS），以及什么是打包基本流（PES）或 I 帧

各种数字电视标准文档可在以下地址下载

- 欧洲标准（DVB）：http://www.dvb.org http://www.etsi.org
- 美国标准（ATSC）：https://www.atsc.org/standards/
- 日本标准（ISDB）：http://www.dibeg.org/

同时还需要知道如何访Linux 设备以及如何使用 ioctl 调用。这也包括对 C C++ 的了解

## 历史


我们1999 年底Convergence 使用的第一个数字电视卡 API，是 Video4Linux API 的扩展，而后者主要是为帧抓取卡开发的。因此，它并不是很适合用于数字电视卡及其新特性，例如录制 MPEG 流以及同时过滤多section PES 数据流

2000 年初，Nokia Convergence 提出了一项新Linux 数字电视 API 标准提案。作为对基于开放标准的终端开发的承诺，Nokia Convergence 将其提供给所Linux 开发者，并于 2000 9 月在 https://linuxtv.org 上发布。凭Siemens/Hauppauge DVB PCI 卡的 Linux 驱动，Convergence 提供Linux 数字电视 API 的首次实现。Convergence 在早期是 Linux 数字电视 API 的维护者

如今，该 API LinuxTV 社区（也就是你，本文档的读者）维护。Linux 数字电视 API 随着内核中该子系统核心的改进而不断地被评审和完善

## 概述




    :alt:   dvbstb.svg
    :align: center

    数字电视/ STB 的组成部

数字电视卡或机顶盒（STB）通常由以下主要硬件组件构成：

Frontend 由调谐器和数字电视解调器组成
   原始的射频信号从这里通过卫星天线或天线，或直接从有线电视线缆到达数字电视硬件。Frontend 将该信号下变频并解调MPEG 传输流（TS）。对于卫frontend，这还包括卫星设备控制（SEC）功能，用于LNB 极化、多馈源开关或天线转盘

条件接收（CA）硬件，CI 适配器和智能卡插
   完整TS 会经CA 硬件。用户有权访问（由智能卡控制）的节目会被实时解码并重新插入到 TS 中

```
	并非所有数字电视硬件都提供条件接收硬件。```
解复用器，用于过滤传入的数字电视 MPEG-TS 
   解复用器TS 拆分为其各个组件，如音频和视频流。除了通常有好几个这样的音频和视频流之外，它还包含带有关于本流或同一提供商其他流中提供的节目信息的数据流

音频和视频解码器
   解复用器的主要目标是音频和视频解码器。解码后，它们将未压缩的音频和视频传递给计算机屏幕或电视机

```
	现代硬件通常没有独立的解码器硬件，因为此类功能可以由CPU、系统的图形适配器，
	或嵌入在片上系统（SoC）集成电路中的信号处理硬件提供

	对于某些用途也可能并不需要它（例如仅数据用途，如“通过卫星上网”）。```
stb_components 展示了这些组件之间控制和数据流的大致示意图

## Linux 数字电视设备


Linux 数字电视 API 让你通过当前六个Unix 字符设备来控制这些硬件组件，分别用于视频、音频、frontend、demux、CA 以及 IP-over-DVB 网络。视频和音频设备控制 MPEG2 解码器硬件，frontend 设备控制调谐器和数字电视解调器。demux 设备让你控制硬件PES section 过滤器。如果硬件不支持过滤，这些过滤器可以用软件实现。最后，CA 设备控制硬件的所有条件接收能力。根据平台各自的安全要求，通过该设备向应用程序开放多少以及哪CA 功能可能会有所不同

所有设备都可以`/dev` 树下`/dev/dvb` 中找到。各个设备称为：

- `/dev/dvb/adapterN/audioM`锛。

- `/dev/dvb/adapterN/videoM`锛。

- `/dev/dvb/adapterN/frontendM`锛。

- `/dev/dvb/adapterN/netM`锛。

- `/dev/dvb/adapterN/demuxM`锛。

- `/dev/dvb/adapterN/dvrM`锛。

- `/dev/dvb/adapterN/caM`锛。

其中 `N` 0 开始枚举系统中的数字电视卡，`M` 也从 0 开始枚举每个适配器内每种类型的设备。在后续这些设备的讨论中，我们将省略 "`/dev/dvb/adapterN/`\ "

关于所有设备的数据结构和函数调用的更多细节在后续章节中描述

## API 头文


对于每个数字电视设备，都存在一个对应的头文件。数字电API 的头文件应在应用程序源码中通过如下部分路径包含



	#include <linux/dvb/ca.h>

	#include <linux/dvb/dmx.h>

	#include <linux/dvb/frontend.h>

	#include <linux/dvb/net.h>


为了让应用程序能够支持不同的 API 版本，还提供了一个额外的头文`linux/dvb/version.h`，它定义了常`DVB_API_VERSION`。本文档描述 `DVB_API_VERSION 5.10`
