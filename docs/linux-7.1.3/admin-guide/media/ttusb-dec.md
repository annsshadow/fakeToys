## TechnoTrend/Hauppauge DEC USB 驱动


### 驱动状态


支持：

 - DEC2000-t
 - DEC2450-t
 - DEC3000-s
 - 视频流（Video Streaming）
 - 音频流（Audio Streaming）
 - 段过滤器（Section Filters）
 - 换台（Channel Zapping）
 - 热插拔固件加载器

待办：

 - 调谐器状态信息
 - DVB 网络接口
 - 视频流 PC->DEC
 - 2450-t 的 Conax 支持

### 获取固件

要下载固件，使用以下命令：


	scripts/get_dvb_firmware dec2000t
	scripts/get_dvb_firmware dec2540t
	scripts/get_dvb_firmware dec3000s


### 热插拔固件加载


自 2.6 内核起，固件在驱动模块加载时被加载。

将上面下载的三个文件复制到 /usr/lib/hotplug/firmware 或 /lib/firmware 目录（取决于固件热插拔的配置）。
