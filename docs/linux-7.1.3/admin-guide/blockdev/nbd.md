## 网络块设备（TCP 版本

### 1) 概述


什么是 NBD：在内核中编译进该功能（或作为模块）后，Linux 可将远程服务器用作其块设备之一。因此，每当客户端计算机想要读取（例如）/dev/nb0 时，它会通过 TCP 向服务器发送请求，服务器将回复所读取的数据。这可用于磁盘空间较小（甚至无盘）的工作站从另一台计算机借用磁盘空间NFS 不同，可以在其上放置任意文件系统等
如需更多信息，或下载 nbd-client nbd-server 工具，请访问 https://github.com/NetworkBlockDevice/nbd
nbd 内核模块只需安装在客户端系统上，因为 nbd-server 完全位于用户空间。事实上，nbd-server 已成功移植到其他操作系统，包Windows
### A) NBD 参数


max_part
	每个设备的分区数（默认值：0）
nbds_max
	应初始化的块设备数量（默认值：16）