
## OCFS2 文件系统


OCFS2 是一个通用的、基extent 的共享磁盘集群文件系统，ext3 有许相似之处。它支持 64 inode 号，并且具有自动扩展的元数据组，这也使它非集群使用颇具吸引力
你需要安ocfs2-tools 软件包，以便至少获得 "mount.ocfs2" "ocfs2_hb_ctl"
Project web page:    http://ocfs2.wiki.kernel.org
Tools git tree:      https://github.com/markfasheh/ocfs2-tools
OCFS2 mailing lists: https://subspace.kernel.org/lists.linux.dev.html

除另有说明外，所有代码版权归 2005 Oracle 所有
## 致谢


大量代码取自 ext3 及其它项目
按字母顺序排列的作者：

- Joel Becker   <joel.becker@oracle.com>
- Zach Brown    <zach.brown@oracle.com>
- Mark Fasheh   <mfasheh@suse.com>
- Kurt Hackel   <kurt.hackel@oracle.com>
- Tao Ma        <tao.ma@oracle.com>
- Sunil Mushran <sunil.mushran@oracle.com>
- Manish Singh  <manish.singh@oracle.com>
- Tiger Yang    <tiger.yang@oracle.com>

## 注意事项

OCFS2 尚不支持的特性：

 - 目录变更通知（F_NOTIFY - 分布式缓存（F_SETLEASE/F_GETLEASE/break_lease
## 挂载选项


OCFS2 支持以下挂载选项
(*) == 默认
======================= ========================================================
barrier=1		该选项启用/禁用屏障。barrier=0 禁用屏障			barrier=1 启用屏障errors=remount-ro(*)	出错时将文件系统以只读方式重新挂载errors=panic		出错时触panic 并停机intr		(*)	允许信号中断集群操作nointr			不允许信号中断集群操作noatime			不更新访问时间relatime(*)		若先前的 atime 早于 mtime ctime 则更atimestrictatime		总是更新 atime，但最小更新间隔由 atime_quantum 指定atime_quantum=60(*)	在该秒数过去之前，OCFS2 不会更新 atime			设为 0 则总是更新 atime。此选项需strictatime 配合使用data=ordered	(*)	在其元数据提交到日志之前，所有数据被强制直接写出
			到主文件系统data=writeback		不保留数据顺序，数据可能在其元数据提交到日志之后
			才写入主文件系统preferred_slot=0(*)	挂载时首先尝试使用该文件系统槽位。若它正被其它节			使用，则选择找到的第一个空槽。无效值将被忽略commit=nrsec	(*)	可以指示 Ocfs2 'nrsec' 秒同步其所有数据和元数据			默认值为 5 秒。这意味着若掉电，你最多可能丢失最			5 秒的工作（不过文件系统不会损坏，这得益于日志）			该默认值（或任何低值）会损害性能，但有利于数			安全。将其设0 的效果与保持默认 秒）相同			将其设为非常大的值会提升性能localalloc=8(*)		允许MB 为单位自定义 localalloc 的大小。若值过大，
			文件系统会静默地将其恢复为默认值localflocks		禁用集群感知flockinode64			表示允许 Ocfs2 在文件系统的任意位置创建 inode，包			那些会导inode 号占用超32 位有效位的情形user_xattr	(*)	启用扩展用户属性nouser_xattr		禁用扩展用户属性acl			启用 POSIX 访问控制列表支持noacl		(*)	禁用 POSIX 访问控制列表支持resv_level=2	(*)	设置分配预留的激进程度。有效值为 0（关闭预留）8
			（为预留保留最大空间）dir_resv_level=	(*)	默认情况下，目录预留会随文件预留缩放——用户很少需			更改此值。若分配预留被关闭，此选项将不起作用coherency=full  (*)	禁止并发O_DIRECT 写入，将获取集群 inode 锁以强制
			其它节点丢弃缓存，因此即使对O_DIRECT 写入也能
			保证完整的集群一致性coherency=buffered	允许节点间无需 EX 锁的并发 O_DIRECT 写入，以较高性能
			为代价，但可能在其它节点上读到陈旧数据journal_async_commit	提交块可以在不等待描述符块的情况下写入磁盘。若启用			旧内核将无法挂载该设备。这会在内部启用 'journal_checksum'======================= ========================================================
