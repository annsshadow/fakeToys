## 自动挂载支持


希望支持自动挂载的文件系统（例如可在 fs/afs/ 找到kAFS，以fs/nfs/ 中的 NFS）可以使用该支持。该机制包括允许执行内核内挂载，
以及请求挂载点降级。后者也可由用户空间请求

## 内核内自动挂

请参Documentation/filesystems/autofs.rst 的“挂载陷阱（Mount Traps）”一节
```
	[root@andromeda root]# mount -t afs \#root.afs. /afs
	[root@andromeda root]# ls /afs
	asd  cambridge  cambridge.redhat.com  grand.central.org
	[root@andromeda root]# ls /afs/cambridge
	afsdoc
	[root@andromeda root]# ls /afs/cambridge/afsdoc/
	ChangeLog  html  LICENSE  pdf  RELNOTES-1.2.2

```
```
	[root@andromeda root]# cat /proc/mounts
	...
	#root.afs. /afs afs rw 0 0
	#root.cell. /afs/cambridge.redhat.com afs rw 0 0
	#afsdoc. /afs/cambridge.redhat.com/afsdoc afs rw 0 0


```
## 鎸傝浇鐐硅嚜鍔ㄨ繃鏈。

只要你在前述自动挂载流程中挂载了将要过期的挂载点，挂载点的自动过就很简单
要进行过期处理，你需要遵循以下步骤：

 (1) 创建至少一个列表，用于挂接将要过期vfsmount
 (2) ->d_automount 方法中创建新的挂载点时，添加
```
             mnt_set_expiry(newmnt, &afs_vfsmounts);

 (3) When you want mountpoints to be expired, call mark_mounts_for_expiry()
     with a pointer to this list. This will process the list, marking every
     vfsmount thereon for potential expiry on the next call.

     If a vfsmount was already flagged for expiry, and if its usage count is 1
     (it's only referenced by its parent vfsmount), then it will be deleted
     from the namespace and thrown away (effectively unmounted).

     It may prove simplest to simply call this at regular intervals, using
     some sort of timed event to drive it.

```
过期标志由对 mntput 的调用清除。这意味着过期只会在挂载点最后一次被访问之后第二次过期请求时发生
如果挂载点被移动，它会从过期列表中移除。如果在可过期挂载上建立了绑定挂载，
新的 vfsmount 将不在过期列表中，也不会过期
如果命名空间被复制，其中包含的所有挂载点都将被复制，并且那些位于过期列表中的
挂载点的副本会被加入同一个过期列表

## 用户空间驱动的过

作为替代，用户空间可以请求任何挂载点的过期（尽管有些会被拒绝——例如当前进所认为rootfs）。它通过umount() 传入 MNT_EXPIRE 标志来实现。该标志被认MNT_FORCE MNT_DETACH 不兼容
如果相关挂载点被 umount() 或其父挂载点以外的东西所引用，将返回 EBUSY 错误并且该挂载点不会被标记为过期或卸载
如果该挂载点当时尚未被标记为过期，将给出 EAGAIN 错误，且不会被卸载
否则，如果它已被标记且未被引用，卸载将照常进行
同样，每当除 umount() 之外的任何东西查看某个挂载点时，过期标志都会被清除