## Linux Macintosh HFSPlus 文件系统


HFSPlus 是首先在 MacOS 8.1 中引入的文件系统。HFSPlus HFS 进行了若扩展，包32 位分配块55 字符unicode 文件名，以及 2^63 字节的文件大小

## 挂载选项


挂载 HFSPlus 文件系统时，接受以下选项
  creator=cccc, type=cccc
	指定创建新文件时MacOS finder 显示的创建类型值	默认值：''
  uid=n, gid=n
	指定拥有文件系统上所有具有未初始化权限结构的文件	用户/组。默认：挂载进程的用id
  umask=n
	指定用于具有未初始化权限结构的文件和目录umask（八进制）	默认：挂载进程的 umask
  session=n
	选择要作HFSPlus 文件系统挂载CDROM 会话。默认交	CDROM 驱动决定。该选项在底层设备不CDROM 时会失败
  part=n
	从设备中选择分区n。该选项仅对 CDROM 有意义，因为它们	Linux 下无法分区。对于磁盘设备，通用的分区解析代码会替我	完成此工作。默认完全不解析分区表
  decompose
	分解文件名字符
  nodecompose
	不分解文件名字符
  force
	用于强制对标记为已日志化或已锁定的卷进行写访问。使用风险自负
  nls=cccc
	呈现文件名时使用的编码

## 参

内核源码	<file:fs/hfsplus>

Apple Technote 1150	https://developer.apple.com/legacy/library/technotes/tn/tn1150.html
