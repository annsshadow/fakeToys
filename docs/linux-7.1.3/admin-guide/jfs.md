## Linux 上的 IBM 日志型文件系统（JFS, Journaled File System）


JFS 主页：  http://jfs.sourceforge.net/

支持以下挂载选项：

(*) == 默认值

iocharset=name
                用于将 Unicode 转换为 ASCII 的字符集。默认不进行转换。使用
                iocharset=utf8 进行 UTF-8 转换。这需要在内核 .config 文件中
                设置 CONFIG_NLS_UTF8。iocharset=none 显式指定默认行为。

resize=value
                将卷大小调整为 <value> 个块。JFS 仅支持扩大卷，而不支持缩小
                它。该选项仅在以读写方式挂载卷的重新挂载（remount）期间有效。
                不带值的 resize 关键字会将卷扩大到分区的完整大小。

nointegrity
                不写入日志。该选项的主要用途是在从备份介质恢复卷时获得更高
                的性能。如果系统异常终止，卷的完整性无法得到保证。

integrity(*)
                将元数据变更提交到日志。使用此选项可以重新挂载此前指定了
                nointegrity 选项的卷，以恢复正常行为。

errors=continue
                        文件系统出错时继续运行。
errors=remount-ro(*)
                        出错时将文件系统以只读方式重新挂载。
errors=panic
                        如果发生错误，触发 panic 并停机。

uid=value
                用指定的值覆盖磁盘上的 uid。
gid=value
                用指定的值覆盖磁盘上的 gid。
umask=value
                用指定的八进制值覆盖磁盘上的 umask。对于目录，如果相应的
                读位被设置，执行位也会被设置。

discard=minlen, discard/nodiscard(*)
                启用/禁用 discard/TRIM 命令的使用。当块被释放时，discard/TRIM
                命令会被发送给底层块设备。这对 SSD 设备以及稀疏/精简配置的 LUN
                很有用。FITRIM ioctl 命令也可与 nodiscard 选项一起使用。minlen
                的值指定最小块数，当达到该值时，向块设备发送 TRIM 命令才被认为
                有用。如果没有为 discard 选项提供值，它默认为 64 个块，在 JFS 中
                即 256KiB。discard 的 minlen 值会覆盖 FITRIM ioctl() 给出的 minlen
                值。

可以通过我们网页 http://jfs.sourceforge.net/ 上标记为“Mail list Subscribe”的链接
来订阅 JFS 邮件列表。
