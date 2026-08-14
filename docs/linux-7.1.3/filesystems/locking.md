## 加锁（Locking）


下文描述了与 VFS 相关方法的加锁规则。它（据信）是最新的。**请**，如果你更改了任何原型或加锁协议——请更新本文件。并更新代码树中的相关实例，不要留给文件系统/设备等的维护者去做。至少，把可疑情况的列表放到本文件末尾。不要把它变成日志——树外代码的维护者应当能够使用 diff(1)。

此处目前缺失的内容：socket 操作。Alexey？

## dentry_operations


```
	int (*d_revalidate)(struct inode *, const struct qstr *,
			    struct dentry *, unsigned int);
	int (*d_weak_revalidate)(struct dentry *, unsigned int);
	int (*d_hash)(const struct dentry *, struct qstr *);
	int (*d_compare)(const struct dentry *,
			unsigned int, const char *, const struct qstr *);
	int (*d_delete)(struct dentry *);
	int (*d_init)(struct dentry *);
	void (*d_release)(struct dentry *);
	void (*d_iput)(struct dentry *, struct inode *);
	char *(*d_dname)((struct dentry *dentry, char *buffer, int buflen);
	struct vfsmount *(*d_automount)(struct path *path);
	int (*d_manage)(const struct path *, bool);
	struct dentry *(*d_real)(struct dentry *, enum d_real_type type);
	bool (*d_unalias_trylock)(const struct dentry *);
	void (*d_unalias_unlock)(const struct dentry *);

```
加锁规则：

================== ===========	========	==============	========
ops		   rename_lock	->d_lock	may block	rcu-walk
================== ===========	========	==============	========
d_revalidate:	   no		no		yes (ref-walk)	maybe
d_weak_revalidate: no		no		yes	 	no
d_hash		   no		no		no		maybe
d_compare:	   yes		no		no		maybe
d_delete:	   no		yes		no		no
d_init:		   no		no		yes		no
d_release:	   no		no		yes		no
d_prune:           no		yes		no		no
d_iput:		   no		no		yes		no
d_dname:	   no		no		no		no
d_automount:	   no		no		yes		no
d_manage:	   no		no		yes (ref-walk)	maybe
d_real		   no		no		yes 		no
d_unalias_trylock  yes		no		no 		no
d_unalias_unlock   yes		no		no 		no
================== ===========	========	==============	========

## inode_operations


```
	int (*create) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t, bool);
	struct dentry * (*lookup) (struct inode *,struct dentry *, unsigned int);
	int (*link) (struct dentry *,struct inode *,struct dentry *);
	int (*unlink) (struct inode *,struct dentry *);
	int (*symlink) (struct mnt_idmap *, struct inode *,struct dentry *,const char *);
	struct dentry *(*mkdir) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t);
	int (*rmdir) (struct inode *,struct dentry *);
	int (*mknod) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t,dev_t);
	int (*rename) (struct mnt_idmap *, struct inode *, struct dentry *,
			struct inode *, struct dentry *, unsigned int);
	int (*readlink) (struct dentry *, char __user *,int);
	const char *(*get_link) (struct dentry *, struct inode *, struct delayed_call *);
	void (*truncate) (struct inode *);
	int (*permission) (struct mnt_idmap *, struct inode *, int, unsigned int);
	struct posix_acl * (*get_inode_acl)(struct inode *, int, bool);
	int (*setattr) (struct mnt_idmap *, struct dentry *, struct iattr *);
	int (*getattr) (struct mnt_idmap *, const struct path *, struct kstat *, u32, unsigned int);
	ssize_t (*listxattr) (struct dentry *, char *, size_t);
	int (*fiemap)(struct inode *, struct fiemap_extent_info *, u64 start, u64 len);
	void (*update_time)(struct inode *inode, enum fs_update_time type,
			    int flags);
	void (*sync_lazytime)(struct inode *inode);
	int (*atomic_open)(struct inode *, struct dentry *,
				struct file *, unsigned open_flag,
				umode_t create_mode);
	int (*tmpfile) (struct mnt_idmap *, struct inode *,
			struct file *, umode_t);
	int (*fileattr_set)(struct mnt_idmap *idmap,
			    struct dentry *dentry, struct file_kattr *fa);
	int (*fileattr_get)(struct dentry *dentry, struct file_kattr *fa);
	struct posix_acl * (*get_acl)(struct mnt_idmap *, struct dentry *, int);
	struct offset_ctx *(*get_offset_ctx)(struct inode *inode);

```
加锁规则：
	全部都可能阻塞

==============	==================================================
ops		i_rwsem(inode)
==============	==================================================
lookup:		shared
create:		exclusive
link:		exclusive (both)
mknod:		exclusive
symlink:	exclusive
mkdir:		exclusive
unlink:		exclusive (both)
rmdir:		exclusive (both)(see below)
rename:		exclusive (both parents, some children)	(see below)
readlink:	no
get_link:	no
setattr:	exclusive
permission:	no (may not block if called in rcu-walk mode)
get_inode_acl:	no
get_acl:	no
getattr:	no
listxattr:	no
fiemap:		no
update_time:	no
sync_lazytime:	no
atomic_open:	shared (exclusive if O_CREAT is set in open flags)
tmpfile:	no
fileattr_get:	no or exclusive
fileattr_set:	exclusive
get_offset_ctx  no
==============	==================================================


	此外，->rmdir()、->unlink() 和 ->rename() 对被操作对象（victim）持有
	->i_rwsem 的独占锁。
	跨目录的 ->rename() 持有（每个超级块）->s_vfs_rename_sem。
	->unlink() 和 ->rename() 对所有涉及的非目录项持有 ->i_rwsem 独占锁。
	->rename() 对任何改变父目录的子目录持有 ->i_rwsem 独占锁。

有关目录操作加锁方案的更详细讨论，请参阅
Documentation/filesystems/directory-locking.rst。

## xattr_handler operations


```
	bool (*list)(struct dentry *dentry);
	int (*get)(const struct xattr_handler *handler, struct dentry *dentry,
		   struct inode *inode, const char *name, void *buffer,
		   size_t size);
	int (*set)(const struct xattr_handler *handler,
                   struct mnt_idmap *idmap,
                   struct dentry *dentry, struct inode *inode, const char *name,
                   const void *buffer, size_t size, int flags);

```
加锁规则：
	全部都可能阻塞

=====		==============
ops		i_rwsem(inode)
=====		==============
list:		no
get:		no
set:		exclusive
=====		==============

## super_operations


```
	struct inode *(*alloc_inode)(struct super_block *sb);
	void (*free_inode)(struct inode *);
	void (*destroy_inode)(struct inode *);
	void (*dirty_inode) (struct inode *, int flags);
	int (*write_inode) (struct inode *, struct writeback_control *wbc);
	int (*drop_inode) (struct inode *);
	void (*evict_inode) (struct inode *);
	void (*put_super) (struct super_block *);
	int (*sync_fs)(struct super_block *sb, int wait);
	int (*freeze_fs) (struct super_block *);
	int (*unfreeze_fs) (struct super_block *);
	int (*statfs) (struct dentry *, struct kstatfs *);
	void (*umount_begin) (struct super_block *);
	int (*show_options)(struct seq_file *, struct dentry *);
	ssize_t (*quota_read)(struct super_block *, int, char *, size_t, loff_t);
	ssize_t (*quota_write)(struct super_block *, int, const char *, size_t, loff_t);

```
加锁规则：
	全部都可能阻塞 [并非如此，见下文]

======================	============	========================
ops			s_umount	note
======================	============	========================
alloc_inode:
free_inode:				called from RCU callback
destroy_inode:
dirty_inode:
write_inode:
drop_inode:				!!!inode->i_lock!!!
evict_inode:
put_super:		write
sync_fs:		read
freeze_fs:		write
unfreeze_fs:		write
statfs:			maybe(read)	(see below)
umount_begin:		no
show_options:		no		(namespace_sem)
quota_read:		no		(see below)
quota_write:		no		(see below)
======================	============	========================

->statfs() 在由 ustat(2)（原生或兼容）调用时持有 s_umount（共享），但这是糟糕 API 的意外产物；s_umount 用于在我们只有用户态给出的 dev_t 来标识超级块时将其固定住。其他一切（statfs()、fstatfs() 等）在调用 ->statfs() 时并不持有它——超级块通过解析传给系统调用的路径名来固定。

->quota_read() 和 ->quota_write() 这两个函数都保证是由配额代码（通过 dqio_sem）操作配额文件的唯一函数（除非管理员真的想搞砸什么，在配额开启时写入配额文件）。有关加锁的其他细节，另请参阅 dquot_operations 一节。

## file_system_type


```
	void (*kill_sb) (struct super_block *);

```
加锁规则：

=======		=========
ops		may block
=======		=========
kill_sb		yes
=======		=========

->kill_sb() 持有一个写锁定的超级块，在其上完成所有关闭工作，解锁并释放引用。

## address_space_operations

```
	int (*read_folio)(struct file *, struct folio *);
	int (*writepages)(struct address_space *, struct writeback_control *);
	bool (*dirty_folio)(struct address_space *, struct folio *folio);
	void (*readahead)(struct readahead_control *);
	int (*write_begin)(const struct kiocb *, struct address_space *mapping,
				loff_t pos, unsigned len,
				struct folio **foliop, void **fsdata);
	int (*write_end)(const struct kiocb *, struct address_space *mapping,
				loff_t pos, unsigned len, unsigned copied,
				struct folio *folio, void *fsdata);
	sector_t (*bmap)(struct address_space *, sector_t);
	void (*invalidate_folio) (struct folio *, size_t start, size_t len);
	bool (*release_folio)(struct folio *, gfp_t);
	void (*free_folio)(struct folio *);
	int (*direct_IO)(struct kiocb *, struct iov_iter *iter);
	int (*migrate_folio)(struct address_space *, struct folio *dst,
			struct folio *src, enum migrate_mode);
	int (*launder_folio)(struct folio *);
	bool (*is_partially_uptodate)(struct folio *, size_t from, size_t count);
	int (*error_remove_folio)(struct address_space *, struct folio *);
	int (*swap_activate)(struct swap_info_struct *sis, struct file *f, sector_t *span)
	int (*swap_deactivate)(struct file *);
	int (*swap_rw)(struct kiocb *iocb, struct iov_iter *iter);

```
加锁规则：
	除 dirty_folio 和 free_folio 外，全部都可能阻塞

======================	======================== =========	===============
ops			folio locked		 i_rwsem	invalidate_lock
======================	======================== =========	===============
read_folio:		yes, unlocks				shared
writepages:
dirty_folio:		maybe
readahead:		yes, unlocks				shared
write_begin:		locks the folio		 exclusive
write_end:		yes, unlocks		 exclusive
bmap:
invalidate_folio:	yes					exclusive
release_folio:		yes
free_folio:		yes
direct_IO:
migrate_folio:		yes (both)
launder_folio:		yes
is_partially_uptodate:	yes
error_remove_folio:	yes
swap_activate:		no
swap_deactivate:	no
swap_rw:		yes, unlocks
======================	======================== =========	===============

->write_begin()、->write_end() 和 ->read_folio() 可能从请求处理程序（/dev/loop）调用。

->read_folio() 会解锁该 folio，无论是同步地还是通过 I/O 完成。

->readahead() 像 ->read_folio() 一样，对尝试进行 I/O 的 folio 进行解锁。

->writepages() 用于周期性回写以及由系统调用发起的同步操作。address_space 应当针对至少 `**nr_to_write` 个页启动 I/O。每写入一个页，必须递减 `**nr_to_write`。address_space 的实现写入的页可能比 `*nr_to_write` 要求的多（或少），但应尽量接近。如果 nr_to_write 为 NULL，则必须写入所有脏页。

writepages 应当_只_写入当前存在于 mapping->i_pages 中的页。

->dirty_folio() 在目标 folio 被标记为需要回写时，由内核中的多处位置调用。该 folio 不能被截断，因为要么调用者持有 folio 锁，要么调用者在持有页表锁的情况下找到了该 folio，而页表锁会阻止截断。

->bmap() 目前由某些文件系统提供的遗留 ioctl()（FIBMAP）以及交换器（swapper）使用。后者最终会消失。请保持现状，不要滋生新的调用者。

->invalidate_folio() 在文件系统必须尝试在页被截断时丢弃该页的部分或全部缓冲区时调用。成功时返回零。文件系统必须在截断/打洞路径中使页缓存失效（并因此调用 ->invalidate_folio）之前，独占获取 invalidate_lock，以阻止页缓存失效与页缓存填充函数（缺页、读……）之间的竞争。

->release_folio() 在 MM 想要对 folio 做出会使文件系统的私有数据失效的修改时调用。例如，它可能即将从 address_space 中移除或被拆分。该 folio 处于锁定状态且不在回写中。它可能是脏的。gfp 参数通常不用于分配，而是用来指示文件系统可以做什么来尝试释放私有数据。文件系统可以返回 false 以表示该 folio 的私有数据无法释放。如果返回 true，它应该已经将私有数据从该 folio 中移除。如果文件系统没有提供 ->release_folio 方法，页缓存将假定私有数据是 buffer_heads 并调用 try_to_free_buffers()。

->free_folio() 在内核将该 folio 从页缓存中丢弃时调用。

->launder_folio() 可能在释放一个 folio 之前，如果它仍被发现是脏的，被调用。如果 folio 被成功清理则返回零，否则返回错误值。注意，为了防止 folio 被重新映射回来并重新变脏，它需要在整个操作期间保持锁定。

->swap_activate() 将被调用来为给定的文件准备交换。它应当执行任何必要的验证和准备工作，以确保写入可以在最小内存分配的情况下进行。它应当调用 add_swap_extent()，或辅助函数 iomap_swapfile_activate()，并返回所添加区段的数量。如果 IO 应当通过 ->swap_rw() 提交，它应当设置 SWP_FS_OPS，否则 IO 将被直接提交到块设备 `sis->bdev`。

->swap_deactivate() 将在 ->swap_activate() 返回成功之后，在 sys_swapoff() 路径中被调用。

->swap_rw 将在设置了 SWP_FS_OPS 时，为交换 IO 被调用。

## file_lock_operations


```
	void (*fl_copy_lock)(struct file_lock *, struct file_lock *);
	void (*fl_release_private)(struct file_lock *);


```
加锁规则：

===================	=============	=========
ops			inode->i_lock	may block
===================	=============	=========
fl_copy_lock:		yes		no
fl_release_private:	maybe		maybe[^1^]_
===================	=============	=========

   ->fl_release_private 对于 flock 或 POSIX 锁，目前允许阻塞。但对于租约（lease），仍然可以在持有 i_lock 时释放，因此租约上调用的 fl_release_private 不应阻塞。

## lock_manager_operations


```
	void (*lm_notify)(struct file_lock *);  /* unblock callback */
	int (*lm_grant)(struct file_lock *, struct file_lock *, int);
	void (*lm_break)(struct file_lock *); /* break_lease callback */
	int (*lm_change)(struct file_lock **, int);
	bool (*lm_breaker_owns_lease)(struct file_lock *);
        bool (*lm_lock_expirable)(struct file_lock *);
        void (*lm_expire_lock)(void);
        bool (*lm_breaker_timedout)(struct file_lease *);

```
加锁规则：

======================	=============	=================	=========
ops			   flc_lock  	blocked_lock_lock	may block
======================	=============	=================	=========
lm_notify:		no      	yes			no
lm_grant:		no		no			no
lm_break:		yes		no			no
lm_change		yes		no			no
lm_breaker_owns_lease:	yes     	no			no
lm_lock_expirable	yes		no			no
lm_expire_lock		no		no			yes
lm_open_conflict	yes		no			no
lm_breaker_timedout     yes             no                      no
======================	=============	=================	=========

## buffer_head


```
	void (*b_end_io)(struct buffer_head *bh, int uptodate);

```
加锁规则：

从中断中调用。换句话说，这里需要极度小心。bh 是锁定的，但那是这里仅有的保证。目前只有 RAID1、highmem、fs/buffer.c 和 fs/ntfs/aops.c 提供这些。块设备在 IO 完成时调用此方法。

## block_device_operations

```
	int (*open) (struct block_device *, fmode_t);
	int (*release) (struct gendisk *, fmode_t);
	int (*ioctl) (struct block_device *, fmode_t, unsigned, unsigned long);
	int (*compat_ioctl) (struct block_device *, fmode_t, unsigned, unsigned long);
	int (*direct_access) (struct block_device *, sector_t, void **,
				unsigned long *);
	void (*unlock_native_capacity) (struct gendisk *);
	int (*getgeo)(struct gendisk *, struct hd_geometry *);
	void (*swap_slot_free_notify) (struct block_device *, unsigned long);

```
加锁规则：

======================= ===================
ops			open_mutex
======================= ===================
open:			yes
release:		yes
ioctl:			no
compat_ioctl:		no
direct_access:		no
unlock_native_capacity:	no
getgeo:			no
swap_slot_free_notify:	no	(see below)
======================= ===================

swap_slot_free_notify 在持有 swap_lock 并且有时持有页锁的情况下被调用。


## file_operations


```
	loff_t (*llseek) (struct file *, loff_t, int);
	ssize_t (*read) (struct file *, char __user *, size_t, loff_t *);
	ssize_t (*write) (struct file *, const char __user *, size_t, loff_t *);
	ssize_t (*read_iter) (struct kiocb *, struct iov_iter *);
	ssize_t (*write_iter) (struct kiocb *, struct iov_iter *);
	int (*iopoll) (struct kiocb *kiocb, bool spin);
	int (*iterate_shared) (struct file *, struct dir_context *);
	__poll_t (*poll) (struct file *, struct poll_table_struct *);
	long (*unlocked_ioctl) (struct file *, unsigned int, unsigned long);
	long (*compat_ioctl) (struct file *, unsigned int, unsigned long);
	int (*mmap) (struct file *, struct vm_area_struct *);
	int (*open) (struct inode *, struct file *);
	int (*flush) (struct file *);
	int (*release) (struct inode *, struct file *);
	int (*fsync) (struct file *, loff_t start, loff_t end, int datasync);
	int (*fasync) (int, struct file *, int);
	int (*lock) (struct file *, int, struct file_lock *);
	unsigned long (*get_unmapped_area)(struct file *, unsigned long,
			unsigned long, unsigned long, unsigned long);
	int (*check_flags)(int);
	int (*flock) (struct file *, int, struct file_lock *);
	ssize_t (*splice_write)(struct pipe_inode_info *, struct file *, loff_t *,
			size_t, unsigned int);
	ssize_t (*splice_read)(struct file *, loff_t *, struct pipe_inode_info *,
			size_t, unsigned int);
	int (*setlease)(struct file *, long, struct file_lock **, void **);
	long (*fallocate)(struct file *, int, loff_t, loff_t);
	void (*show_fdinfo)(struct seq_file *m, struct file *f);
	unsigned (*mmap_capabilities)(struct file *);
	ssize_t (*copy_file_range)(struct file *, loff_t, struct file *,
			loff_t, size_t, unsigned int);
	loff_t (*remap_file_range)(struct file *file_in, loff_t pos_in,
			struct file *file_out, loff_t pos_out,
			loff_t len, unsigned int remap_flags);
	int (*fadvise)(struct file *, loff_t, loff_t, int);

```
加锁规则：
	全部都可能阻塞。

->llseek() 的加锁已从 llseek 移到了各个 llseek 实现中。如果你的文件系统没有使用 generic_file_llseek，则需要在你的 ->llseek() 中获取并释放适当的锁。对于许多文件系统来说，获取 inode 互斥体或干脆改用 i_size_read() 可能是安全的。注意：这并不能保护 file->f_pos 免受并发修改，因为这是用户态需要自行处理的事情。

->iterate_shared() 在持有 i_rwsem（读）以及 file 的 f_pos_lock（独占）的情况下被调用。

->fasync() 负责维护 filp->f_flags 中的 FASYNC 位。大多数实例调用 fasync_helper()，由它完成该维护，所以这通常不是需要担心的事。大于 0 的返回值会在 VFS 层被映射为零。

->readdir() 和目录上的 ->ioctl() 必须被修改。理想情况下，我们会把 ->readdir() 移到 inode_operations，并为目录 ->ioctl() 使用一个单独的方法，或者干脆完全去掉后者。问题之一是，对于任何类似于联合挂载（union-mount）的情况，我们并不会为所有组件都持有一个 struct file。而且当前接口之所以一团糟还有其他原因……

->read 对目录的读取很可能必须去掉——我们应当直接在 sys_read() 及其同类中强制返回 -EISDIR。

->setlease 操作应当在各个文件系统中设置租约之前或之后调用 generic_setlease()，以记录操作的结果。

->fallocate 实现必须非常小心，在打洞或执行其他使页缓存内容失效的操作时，保持页缓存的一致性。通常文件系统需要调用 truncate_inode_pages_range() 来使页缓存的相关范围失效。然而文件系统通常还需要更新其内部的（以及磁盘上的）文件偏移 -> 磁盘块映射视图。在这个更新完成之前，文件系统需要阻止页错误以及从磁盘重新加载现已过时的页缓存内容的读操作。由于 VFS 在从磁盘加载页时（filemap_fault()、filemap_read()、readahead 路径）以共享模式获取 mapping->invalidate_lock，fallocate 实现必须获取 invalidate_lock 来阻止重新加载。

->copy_file_range 和 ->remap_file_range 实现需要在操作运行期间，针对文件数据的修改进行串行化。要阻止通过 write(2) 及类似操作进行的修改，可以使用 inode->i_rwsem。要阻止通过内存映射在操作期间修改文件内容，文件系统必须获取 mapping->invalidate_lock 来与 ->page_mkwrite 协调。

## dquot_operations


```
	int (*write_dquot) (struct dquot *);
	int (*acquire_dquot) (struct dquot *);
	int (*release_dquot) (struct dquot *);
	int (*mark_dirty) (struct dquot *);
	int (*write_info) (struct super_block *, int);

```
这些操作旨在成为或多或少包装性的函数，确保正确的加锁（相对于文件系统）并调用通用的配额操作。

文件系统可以从通用配额函数中期待什么：

==============	============	=========================
ops		FS recursion	Held locks when called
==============	============	=========================
write_dquot:	yes		dqonoff_sem or dqptr_sem
acquire_dquot:	yes		dqonoff_sem or dqptr_sem
release_dquot:	yes		dqonoff_sem or dqptr_sem
mark_dirty:	no		-
write_info:	yes		dqonoff_sem
==============	============	=========================

FS recursion 指从超级块操作中调用 ->quota_read() 和 ->quota_write()。

有关配额加锁的更多细节可以在 fs/dquot.c 中找到。

## vm_operations_struct


```
	void (*open)(struct vm_area_struct *);
	void (*close)(struct vm_area_struct *);
	vm_fault_t (*fault)(struct vm_fault *);
	vm_fault_t (*huge_fault)(struct vm_fault *, unsigned int order);
	vm_fault_t (*map_pages)(struct vm_fault *, pgoff_t start, pgoff_t end);
	vm_fault_t (*page_mkwrite)(struct vm_area_struct *, struct vm_fault *);
	vm_fault_t (*pfn_mkwrite)(struct vm_area_struct *, struct vm_fault *);
	int (*access)(struct vm_area_struct *, unsigned long, void*, int, int);

```
加锁规则：

=============	==========	===========================
ops		mmap_lock	PageLocked(page)
=============	==========	===========================
open:		write
close:		read/write
fault:		read		can return with page locked
huge_fault:	maybe-read
map_pages:	maybe-read
page_mkwrite:	read		can return with page locked
pfn_mkwrite:	read
access:		read
=============	==========	===========================

->fault() 在即将对一个先前不存在的 pte 产生缺页时调用。文件系统必须找到并返回与传入 vm_fault 结构中的 "pgoff" 关联的页。如果页有可能被截断和/或失效，则文件系统必须锁定 invalidate_lock，然后确保该页尚未被截断（invalidate_lock 会阻止后续的截断），然后以 VM_FAULT_LOCKED 返回，并且该页处于锁定状态。VM 将解锁该页。

->huge_fault() 在不存在 PUD 或 PMD 项时被调用。这给文件系统提供了安装一个 PUD 或 PMD 大小页的机会。文件系统也可以使用 ->fault 方法返回 PMD 大小的页，因此实现此函数可能不是必需的。特别地，文件系统不应从 ->huge_fault() 中调用 filemap_fault()。调用此方法时可能不持有 mmap_lock。

->map_pages() 在 VM 要求映射易于访问的页时被调用。文件系统应当找到并映射与从 "start_pgoff" 到 "end_pgoff" 偏移量关联的页。->map_pages() 在持有 RCU 锁的情况下调用，且不能阻塞。如果无法在不阻塞的情况下到达某个页，文件系统应当跳过它。文件系统应当使用 set_pte_range() 来设置页表项。与页关联的项的指针通过 vm_fault 结构中的 "pte" 字段传入。其他偏移量的项指针应当相对 "pte" 计算。

->page_mkwrite() 在先前只读的 pte 即将变为可写时被调用。文件系统同样必须确保不存在 truncate/invalidate 竞争，或与诸如 ->remap_file_range 或 ->copy_file_range 等操作之间的竞争，然后以页锁定状态返回。通常 mapping->invalidate_lock 适用于适当的串行化。如果该页已被截断，文件系统不应像 ->fault() 处理程序那样查找新页，而只需以 VM_FAULT_NOPAGE 返回，这将导致 VM 重试该缺页。

->pfn_mkwrite() 与 page_mkwrite 相同，但当 pte 是 VM_PFNMAP 或 VM_MIXEDMAP 且为无页项时。期望返回 VM_FAULT_NOPAGE，或 VM_FAULT_ERROR 类型之一。此调用之后的默认行为是使 pte 变为读写，除非 pfn_mkwrite 返回错误。

->access() 在 get_user_pages() 在 access_process_vm() 中失败时调用，通常用于通过 /proc/pid/mem 或 ptrace 调试一个进程。此函数仅对 VM_IO | VM_PFNMAP 的 VMA 是必需的。

--------------------------------------------------------------------------------

			可疑 stuff

（如果你弄坏了什么，或者注意到它已损坏却没有自己修复——至少把它放在这里）
