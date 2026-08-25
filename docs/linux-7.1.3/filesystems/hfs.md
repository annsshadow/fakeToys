
## Linux Macintosh HFS 文件系统


HFS 代表 `Hierarchical File System`（分层文件系统），是 Mac Plus 及所有后Macintosh 机型所使用的文件系统。更早的 Macintosh 机型使用 MFS（`Macintosh File
System`，Macintosh 文件系统），该格式不受支持；MacOS 8.1 及更新版本支持一名为 HFS+ 的文件系统，它与 HFS 类似但在多个方面进行了扩展。要Linux 访问
此类文件系统，请使用 hfsplus 文件系统驱动
## 挂载选项


挂载 HFS 文件系统时，接受以下选项
  creator=cccc, type=cccc
	指定MacOS finder 显示creator/type 值，用于创建新文件	默认值：''
  uid=n, gid=n
  	指定拥有文件系统中所有文件的用户/组	默认值：挂载进程的用id
  dir_umask=n, file_umask=n, umask=n
	指定用于所有文件、所有目录或所有文件与目录umask	默认值为挂载进程umask
  session=n
  	选择要作HFS 文件系统挂载CDROM 会话。默认交CDROM 驱动
	来决定。该选项在底层设备不CDROM 时会失败
  part=n
  	从设备中选择n 个分区。这只对 CDROM 有意义，因为 CDROM 无法	Linux 下被分区。对于磁盘设备，通用的分区解析代码会替我们完成此事	默认完全不解析分区表
  quiet
  	忽略无效的挂载选项，而不是报错

## 写入 HFS 文件系统


HFS 并非 UNIX 文件系统，因此它不具备你所期望的常见特性：

 - 你无法修改文件的 set-uid、set-gid、sticky 或可执行位，也无法修改其 uid
   gid - 你无法创建硬链接或符号链接、设备文件、socket FIFO
不过 HFS 具有每个文件多个 fork 的概念。这些非标准fork 在常规文件系统命空间中被表示为隐藏的附加文件，这多少有些 hack 的味道，并使得其语义显得有些
奇怪：

 - 你无法创建、删除或重命名文件的资源 fork Finder 的元数据 - 不过它们会随相应的数fork 或目录一起被创建（使用默认值）、删除和重命名 - 将文件复制到另一种文件系统时会丢失那些对 MacOS 正常工作必不可少的属性

## 创建 HFS 文件系统


Robert Leslie hfsutils 软件包中包含一个名hformat 的程序，可用于创HFS 文件系统。详<https://www.mars.org/home/rob/proj/hfs/>

## 致谢


HFS 驱动Paul H. Hargrove（hargrove@sccm.Stanford.EDU）编写。Roman Zippel
（roman@ardistech.com）重写了代码的大部分，并引入了源Brad Boyer hfsplus
驱动btree 例程