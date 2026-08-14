
## Linux 内核中的文件系统


这份仍在开发中（尚未完成）的手册，有朝一日会提供关于 Linux 虚拟文件系统（VFS）
层如何工作，以及位于其之下的各文件系统如何工作的全面信息。目前，我们所拥有的
内容如下。

## 核心 VFS 文档


关于 VFS 层本身及其算法如何工作的文档，请参阅以下手册。

- [vfs](vfs)
- [path-lookup](path-lookup)
- [api-summary](api-summary)
- [splice](splice)
- [locking](locking)
- [directory-locking](directory-locking)
- [devpts](devpts)
- [dnotify](dnotify)
- [fiemap](fiemap)
- [files](files)
- [locks](locks)
- [mmap_prepare](mmap_prepare)
- [multigrain-ts](multigrain-ts)
- [mount_api](mount_api)
- [quota](quota)
- [seq_file](seq_file)
- [sharedsubtree](sharedsubtree)
- [idmappings](idmappings)
- [iomap/index](iomap/index)
- [automount-support](automount-support)
- [caching/index](caching/index)
- [porting](porting)

## 文件系统支持层


文件系统层内、供文件系统实现使用的支持代码的文档。

- [buffer](buffer)
- [journalling](journalling)
- [fscrypt](fscrypt)
- [fsverity](fsverity)
- [netfs_library](netfs_library)

## 文件系统


文件系统实现的文档。

- [9p](9p)
- [adfs](adfs)
- [affs](affs)
- [afs](afs)
- [autofs](autofs)
- [autofs-mount-control](autofs-mount-control)
- [befs](befs)
- [bfs](bfs)
- [btrfs](btrfs)
- [ceph](ceph)
- [coda](coda)
- [configfs](configfs)
- [cramfs](cramfs)
- [dax](dax)
- [debugfs](debugfs)
- [dlmfs](dlmfs)
- [ecryptfs](ecryptfs)
- [efivarfs](efivarfs)
- [erofs](erofs)
- [ext2](ext2)
- [ext3](ext3)
- [ext4/index](ext4/index)
- [f2fs](f2fs)
- [gfs2/index](gfs2/index)
- [hfs](hfs)
- [hfsplus](hfsplus)
- [hpfs](hpfs)
- [fuse/index](fuse/index)
- [inotify](inotify)
- [isofs](isofs)
- [nilfs2](nilfs2)
- [nfs/index](nfs/index)
- [ntfs](ntfs)
- [ntfs3](ntfs3)
- [ocfs2](ocfs2)
- [ocfs2-online-filecheck](ocfs2-online-filecheck)
- [omfs](omfs)
- [orangefs](orangefs)
- [overlayfs](overlayfs)
- [proc](proc)
- [qnx6](qnx6)
- [ramfs-rootfs-initramfs](ramfs-rootfs-initramfs)
- [relay](relay)
- [resctrl](resctrl)
- [romfs](romfs)
- [smb/index](smb/index)
- [spufs/index](spufs/index)
- [squashfs](squashfs)
- [sysfs](sysfs)
- [tmpfs](tmpfs)
- [ubifs](ubifs)
- [ubifs-authentication](ubifs-authentication)
- [udf](udf)
- [virtiofs](virtiofs)
- [vfat](vfat)
- [xfs/index](xfs/index)
- [zonefs](zonefs)
