
## 常见问题（FAQ

     1. 在数字电视中，一个物理频道中可能包含不同的内容。规范将每个内容称为一*业务（service*。这正是电视用户所说的"频道"。因此，为了避免混淆，在FAQ 中我们将**转发器（transponder*称为物理频道，将**业务（service*称为逻辑频道
     2. LinuxTV 社区维护着一Wiki 页面，其中包含大量与媒体子系统相关的信息。如果你在这里找不到所需答案，很可能在那里能找到有用的内容。它托管在：

	https://www.linuxtv.org/wiki/

一些关Linux 数字电视支持的非常常见的问题

1. 信号似乎在调谐后几秒就消失了
	这不bug，而是特性。因为前端（frontend）有相当大的功耗需求（因此会变得非常热），如果它们未被使用（即前端设备被关闭），就会被断电。`dvb-core` 模块参数 `dvb_shutdown_timeout` 允许你更改超时时间（默认 5 秒）。将超时设为 0 会禁用超时特性
2. 我如何看电视
	数字电视开发者与 Linux 内核一起维护了一些简单的工具，主要用于测试并演示 DVB API 的工作方式。这称为 DVB v5 工具，与 `v4l-utils` git 仓库放在一起：

	    https://git.linuxtv.org/v4l-utils.git/

	你可以在 LinuxTV wiki 找到更多信息
	    https://www.linuxtv.org/wiki/index.php/DVBv5_Tools

	第一步是获取所传输的业务列表
	这可以通过使用若干现有工具完成。例如，你可以使`dvbv5-scan` 工具。你可以在以下位置找到关于它的更多信息：

	    https://www.linuxtv.org/wiki/index.php/Dvbv5-scan

	还有其他一些应用，`w_scan` [#]_，会进行盲扫，努力寻找所有可能的频道，但那些会消耗大量运行时间
	.. [#] https://www.linuxtv.org/wiki/index.php/W_scan

	此外，一些应用（`kaffeine`）有自己的代码来扫描业务。所以你不需要使用外部应用来获取这样的列表
	大多数此类工具需要一个包含你所在区域可用频道转发器列表的文件。因此，LinuxTV 开发者维护着数字电视频道转发器表，并从社区接收补丁以保持更新
	该列表托管在
	    https://git.linuxtv.org/dtv-scan-tables.git

	并且被打包进多个发行版
	Kaffeine 对一些地面标准有一定的盲扫支持。它也依DTV 扫描表，尽管它内部包含了一份副本（并且如果用户要求，它会下载更新的版本）
	如果幸运的话，你可以直接使用提供的某个频道转发器。否则，你可能需要在互联网上查找此类信息并创建一个新文件。有多个站点包含物理频道列表。对于有线和卫星，通常知道如何调谐到单个频道就足以让扫描工具识别出其他频道。在某些地方，这对地面传输也可能有效
	一旦你有了转发器列表，你需要用`dvbv5-scan` 这样的工具生成业务列表
	几乎所有现代数字电视卡都没有内置的硬件 MPEG 解码器。因此，由应用程序负责获取板卡提供的 MPEG-TS 流，将其拆分为音频、视频和其他数据并进行解码
3. 有哪些数字电视应用程序？

	多个媒体播放器应用能够调谐到数字电视频道，包Kaffeine、Vlc、mplayer MythTV
	Kaffeine 旨在非常用户友好，它由一位内核驱动开发者维护
	这些及其他应用的综合列表可在以下位置找到
	    https://www.linuxtv.org/wiki/index.php/TV_Related_Software

	下面链接了一些最流行的：

	https://kde.org/applications/multimedia/org.kde.kaffeine
		KDE 媒体播放器，专注于数字电视支
	https://www.linuxtv.org/vdrwiki/index.php/Main_Page
		Klaus Schmidinger 的视频磁盘录像机（Video Disk Recorder
	https://linuxtv.org/downloads and https://git.linuxtv.org/
		数字电视及其他媒体相关应用和内核驱动。其中的 `v4l-utils` 软件包包含若干用于数字电视的瑞士军刀式工具
	http://sourceforge.net/projects/dvbtools/
		Dave Chapman dvbtools 软件包，包括 dvbstream dvbtune

	http://www.dbox2.info/
		dBox2 上的 LinuxDVB

	http://www.tuxbox.org/
		TuxBox CVS，包含许多有趣的 DVB 应用dBox2 DVB 源码

	http://www.nenie.org/misc/mpsys/
		MPSYS：一MPEG2 系统库和工具

	https://www.videolan.org/vlc/index.pt.html
		Vlc

	http://mplayerhq.hu/
		MPlayer

	http://xine.sourceforge.net/ and http://xinehq.de/
		Xine

	http://www.mythtv.org/
		MythTV - 模拟电视和数字电PVR

	http://dvbsnoop.sourceforge.net/
		DVB 嗅探程序，用于监视、分析、调试、转储或查看 dvb/mpeg/dsm-cc/mhp 流信息（TS、PES、SECTION
4. 无法正确调谐到信
	这可能是由于很多问题。根据我的个人经验，通常电视卡比电视机需要更强的信号，并且对噪声更敏感。所以，也许你只需要更好的天线或线缆。不过，也可能是某些硬件或驱动问题
	例如，如果你使用的是不带模拟模块Technotrend/Hauppauge DVB-C 卡，你可能需要使用模块参adac=-1（dvb-ttpci.o）
	请参linuxtv.org 上的 FAQ 页面，因为它可能包含一些有价值的信息
	    https://www.linuxtv.org/wiki/index.php/FAQ_%26_Troubleshooting

	如果那没有用，请查看 linux-media 邮件列表归档，看看是否有人遇到过与你的硬件和/或数字电视服务提供商类似的问题：

	    https://lore.kernel.org/linux-media/

	如果这些都没用，你可以尝试向 linux-media 邮件列表发送电子邮件，看看是否有人能提供一些线索。电子邮件地址linux-media AT vger.kernel.org
5. dvb_net 设备完全没有给我任何数据
	`dvb0_0` 接口上运`tcpdump`。这会将接口设为混杂模式，从而接受你`dvbnet` 实用程序配置PID 传来的任何数据包。检查是否有你用 `ifconfig` `ip addr` 配置IP 地址MAC 地址的数据包
	如果 `tcpdump` 没有任何输出，请检`ifconfig` `netstat -ni` 输出的统计信息。（注意：如MAC 地址错误，`dvb_net` 将不会收到任何输入；因此你必须在检查统计信息之前先运行 `tcpdump`。）如果完全没有数据包，那么可能PID 错误。如果有错误数据包，那么要么PID 错误，要么是流不符合 MPE 标准（EN 301 192，http://www.etsi.org/）。例如，你可以使`dvbsnoop` 进行调试
6. `dvb_net` 设备没有给我任何多播数据
	检查你的路由是否包含多播地址范围。此外，确保"基于反向路径的源验证（source validation by reversed path
```
	  $ "echo 0 > /proc/sys/net/ipv4/conf/dvb0/rp_filter"

```
7. 那些需要加载的模块都是什么？

	为了使之更灵活并支持不同的硬件组合，媒体子系统以模块化的方式编写
	因此，除了主芯片组的数字电视硬件模块外，它还需要加载一个前端驱动，以及数字电视核心。如果板卡还带有遥控器，它还需要遥控器核心和遥控器表。如果板卡支持模拟电视，情况也一样：需要加video4linux 的核心支持
	实际的模块名称是特定Linux 内核版本的，因为为了增强媒体支持的灵活性，情况会不时发生变化