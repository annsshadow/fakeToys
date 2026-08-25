## Changes since 2.5.0:

## 2.5.0 起的变更


---


**recommended**

**建议**

New helpers: sb_bread(), sb_getblk(), sb_find_get_block(), set_bh(),
sb_set_blocksize() and sb_min_blocksize().

新增辅助函数：sb_bread()、sb_getblk()、sb_find_get_block()、set_bh()
sb_set_blocksize() 鍜?sb_min_blocksize()銆。

Use them.

请使用它们

(sb_find_get_block() replaces 2.4's get_hash_table())

（sb_find_get_block() 取代 2.4 get_hash_table()

---


**recommended**

**建议**

New methods: ->alloc_inode() and ->destroy_inode().

新增方法>alloc_inode() ->destroy_inode()

Remove inode->u.foo_inode_i

移除 inode->u.foo_inode_i

```

	struct foo_inode_info {
		/* fs-private stuff */
		struct inode vfs_inode;
	};
	static inline struct foo_inode_info *FOO_I(struct inode *inode)
	{
		return list_entry(inode, struct foo_inode_info, vfs_inode);
	}

```

Use FOO_I(inode) instead of &inode->u.foo_inode_i;

使用 FOO_I(inode) 取代 &inode->u.foo_inode_i

Add foo_alloc_inode() and foo_destroy_inode() - the former should allocate
foo_inode_info and return the address of ->vfs_inode, the latter should free
FOO_I(inode) (see in-tree filesystems for examples).

新增 foo_alloc_inode() foo_destroy_inode()——前者应分配
foo_inode_info 并返->vfs_inode 的地址，后者应释放
FOO_I(inode)（参见树内文件系统的示例）

Make them ->alloc_inode and ->destroy_inode in your super_operations.

在你super_operations 中将它们设为 ->alloc_inode ->destroy_inode

Keep in mind that now you need explicit initialization of private data
typically between calling iget_locked() and unlocking the inode.

请注意，现在你需要在调用 iget_locked() 与解inode 之间显式
初始化私有数据

At some point that will become mandatory.

这一点在某个时候将变为强制要求

**mandatory**

**强制**

The foo_inode_info should always be allocated through alloc_inode_sb() rather
than kmem_cache_alloc() or kmalloc() related to set up the inode reclaim context
correctly.

foo_inode_info 应始终通过 alloc_inode_sb() 而非 kmem_cache_alloc() 
kmalloc() 分配，以正确建立 inode 回收上下文

---


**mandatory**

**强制**

Change of file_system_type method (->read_super to ->get_sb)

file_system_type 方法的变更（->read_super 改为 ->get_sb

->read_super() is no more.  Ditto for DECLARE_FSTYPE and DECLARE_FSTYPE_DEV.

->read_super() 已不复存在。DECLARE_FSTYPE DECLARE_FSTYPE_DEV 同理

Turn your foo_read_super() into a function that would return 0 in case of
success and negative number in case of error (-EINVAL unless you have more

将你foo_read_super() 改成一个函数，成功时返0，出错时返回负数
（除非你有更具体的错误码，否则为 -EINVAL）：

```

  int foo_get_sb(struct file_system_type *fs_type,
	int flags, const char *dev_name, void *data, struct vfsmount *mnt)
  {
	return get_sb_bdev(fs_type, flags, dev_name, data, foo_fill_super,
			   mnt);
  }

```

(or similar with s/bdev/nodev/ or s/bdev/single/, depending on the kind of
filesystem).

（或者视文件系统类型，用 s/bdev/nodev/ s/bdev/single/ 的类似写法）

Replace DECLARE_FSTYPE... with explicit initializer and have ->get_sb set as
foo_get_sb.

DECLARE_FSTYPE... 替换为显式初始化器，并将 ->get_sb 设为
foo_get_sb銆。

---


**mandatory**

**强制**

Locking change: ->s_vfs_rename_sem is taken only by cross-directory renames.
Most likely there is no need to change anything, but if you relied on
global exclusion between renames for some internal purpose - you need to
change your internal locking.  Otherwise exclusion warranties remain the
same (i.e. parents and victim are locked, etc.).

加锁变更>s_vfs_rename_sem 仅由跨目录重命名获取。很可能你不需
做任何改动，但如果你为了某些内部目的依赖重命名之间的全局互斥——你
需要修改你的内部加锁。否则排它保证保持不变（即父目录和受害者被
锁定等）

---


**informational**

**说明**

Now we have the exclusion between ->lookup() and directory removal (by
->rmdir() and ->rename()).  If you used to need that exclusion and do
it by internal locking (most of filesystems couldn't care less) - you
can relax your locking.

现在我们有了 ->lookup() 与目录删除（通过 ->rmdir() ->rename()
之间的互斥。如果你曾经需要该互斥并通过内部加锁来实现（大多数文件系
根本不关心）——你可以放宽你的加锁

---


**mandatory**

**强制**

->lookup(), ->truncate(), ->create(), ->unlink(), ->mknod(), ->mkdir(),
->rmdir(), ->link(), ->lseek(), ->symlink(), ->rename()
and ->readdir() are called without BKL now.  Grab it on entry, drop upon return
- that will guarantee the same locking you used to have.  If your method or its
parts do not need BKL - better yet, now you can shift lock_kernel() and
unlock_kernel() so that they would protect exactly what needs to be
protected.

->lookup()銆?>truncate()銆?>create()銆?>unlink()銆?>mknod()銆?>mkdir()銆。
->rmdir()銆?>link()銆?>lseek()銆?>symlink()銆?>rename()
->readdir() 现在不再持有 BKL 时被调用。在入口处获取它，在返回时释
——这将保证与你以往拥有的相同加锁。如果你的方法或其部分不需BKL—
那就更好了，现在你可以移lock_kernel() unlock_kernel()，使它们
恰好保护需要保护的内容

---


**mandatory**

**强制**

BKL is also moved from around sb operations. BKL should have been shifted into
individual fs sb_op functions.  If you don't need it, remove it.

BKL 也已sb 操作周围移走。BKL 本应被移入各个文件系统自己的 sb_op
函数。如果你不需要它，就移除它

---


**informational**

**说明**

check for ->link() target not being a directory is done by callers.  Feel
free to drop it...

->link() 目标不是目录的检查已由调用方完成。可以放心地去掉它…

---


**informational**

**说明**

->link() callers hold ->i_mutex on the object we are linking to.  Some of your
problems might be over...

->link() 的调用方对我们所链接到的对象持有 ->i_mutex。你的一些问题可
就此解决了…

---


**mandatory**

**强制**

new file_system_type method - kill_sb(superblock).  If you are converting

新的 file_system_type 方法——kill_sb(superblock)。如果你正在转换

```

	FS_REQUIRES_DEV		-	kill_block_super
	FS_LITTER		-	kill_litter_super
	neither			-	kill_anon_super

```

FS_LITTER is gone - just remove it from fs_flags.

FS_LITTER 已不存在——只需fs_flags 中移除它

---


**mandatory**

**强制**

FS_SINGLE is gone (actually, that had happened back when ->get_sb()
went in - and hadn't been documented ;-/).  Just remove it from fs_flags
(and see ->get_sb() entry for other actions).

FS_SINGLE 已不存在（实际上，它->get_sb() 引入时就已经消失——只
没有被文档记;-/）。只需fs_flags 中移除它（并参见 ->get_sb() 条目
了解其他操作）

---


**mandatory**

**强制**

->setattr() is called without BKL now.  Caller _always_ holds ->i_mutex, so
watch for ->i_mutex-grabbing code that might be used by your ->setattr().
Callers of notify_change() need ->i_mutex now.

->setattr() 现在不再持有 BKL 时被调用。调用方_始终_持有 ->i_mutex，因
要注意你可能->setattr() 使用的获->i_mutex 的代码。notify_change()
的调用方现在需->i_mutex

---


**recommended**

**建议**

New super_block field `struct export_operations *s_export_op` for
explicit support for exporting, e.g. via NFS.  The structure is fully
documented at its declaration in include/linux/fs.h, and in
Documentation/filesystems/nfs/exporting.rst.

新的 super_block 字段 `struct export_operations *s_export_op` 用于
显式支持导出，例如通过 NFS。该结构在其 include/linux/fs.h 中的声明处，
以及 Documentation/filesystems/nfs/exporting.rst 中有完整文档

Briefly it allows for the definition of decode_fh and encode_fh operations
to encode and decode filehandles, and allows the filesystem to use
a standard helper function for decode_fh, and provide file-system specific
support for this helper, particularly get_parent.

简而言之，它允许定decode_fh encode_fh 操作来编码和解码文件句柄
（filehandle），并允许文件系统为 decode_fh 使用一个标准辅助函数，以及
为该辅助函数提供文件系统特定的支持，尤其get_parent

It is planned that this will be required for exporting once the code
settles down a bit.

计划是，待代码稍微稳定后，这将成为导出所必须的

**mandatory**

**强制**

s_export_op is now required for exporting a filesystem.
isofs, ext2, ext3, fat
can be used as examples of very different filesystems.

导出文件系统现在要求提供 s_export_op
isofs、ext2、ext3、fat
可作为差异很大的文件系统的示例

---


**mandatory**

**强制**

iget4() and the read_inode2 callback have been superseded by iget5_locked()

iget4() read_inode2 回调已被 iget5_locked() 取代

```

    struct inode *iget5_locked(struct super_block *sb, unsigned long ino,
				int (*test)(struct inode *, void *),
				int (*set)(struct inode *, void *),
				void *data);

```

'test' is an additional function that can be used when the inode
number is not sufficient to identify the actual file object. 'set'
should be a non-blocking function that initializes those parts of a
newly created inode to allow the test function to succeed. 'data' is
passed as an opaque value to both test and set functions.

'test' 是一个附加函数，inode 号不足以标识实际文件对象时可使用它
'set' 应是一个非阻塞函数，负责初始化新创inode 的那些部分，以使
test 函数能够成功data' 作为一个不透明值传递给 test set 两个函数

When the inode has been created by iget5_locked(), it will be returned with the
I_NEW flag set and will still be locked.  The filesystem then needs to finalize
the initialization. Once the inode is initialized it must be unlocked by
calling unlock_new_inode().

inode iget5_locked() 创建时，它会被返回且带有 I_NEW 标志，并仍被
锁定。然后文件系统需要完成初始化。一inode 初始化完成，必须通过调用
unlock_new_inode() 解锁

The filesystem is responsible for setting (and possibly testing) i_ino
when appropriate. There is also a simpler iget_locked function that
just takes the superblock and inode number as arguments and does the
test and set for you.

文件系统负责在适当的时候设置（并可能测试）i_ino。还有一个更简单的
iget_locked 函数，它只接super_block inode 号作为参数，并为
完成 test set

```

	inode = iget_locked(sb, ino);
	if (inode_state_read_once(inode) & I_NEW) {
		err = read_inode_from_disk(inode);
		if (err < 0) {
			iget_failed(inode);
			return err;
		}
		unlock_new_inode(inode);
	}

```

Note that if the process of setting up a new inode fails, then iget_failed()
should be called on the inode to render it dead, and an appropriate error
should be passed back to the caller.

注意，如果建立新 inode 的过程失败，则应在该 inode 上调iget_failed()
使其失效，并向调用方返回适当的错误

---


**recommended**

**建议**

->getattr() finally getting used.  See instances in nfs, minix, etc.

->getattr() 终于被用上了。参nfs、minix 等的实例

---


**mandatory**

**强制**

->revalidate() is gone.  If your filesystem had it - provide ->getattr()
and let it call whatever you had as ->revlidate() + (for symlinks that
had ->revalidate()) add calls in ->follow_link()/->readlink().

->revalidate() 已不存在。如果你的文件系统曾有它——请提供 ->getattr()
并让它调用你原来->revlidate()，且（对于曾拥有 ->revalidate() 
符号链接）在 ->follow_link()/->readlink() 中添加调用

---


**mandatory**

**强制**

->d_parent changes are not protected by BKL anymore.  Read access is safe
if at least one of the following is true:

->d_parent 的变更不再受 BKL 保护。如果以下至少一项成立，则读取访问是
安全的：

 - filesystem has no cross-directory rename()
 - we know that parent had been locked (e.g. we are looking at
	  ->d_parent of ->lookup() argument).
 - we are called from ->rename().
 - the child's ->d_lock is held

 - 文件系统没有跨目rename()
 - 我们知道父目录已被锁定（例如，我们正在查->lookup() 参数
	  ->d_parent）
 - 我们正从 ->rename() 中被调用
 - 子项->d_lock 被持

Audit your code and add locking if needed.  Notice that any place that is
not protected by the conditions above is risky even in the old tree - you
had been relying on BKL and that's prone to screwups.  Old tree had quite
a few holes of that kind - unprotected access to ->d_parent leading to
anything from oops to silent memory corruption.

审查你的代码，并在需要时添加加锁。请注意，即便在旧代码树中，任何
受上述条件保护的地方也是有风险的——你曾依BKL，而那很容易出错。旧代码
有相当多此类漏洞——对 ->d_parent 的无保护访问会导致从 oops 到静默内
损坏等各种问题

---


**mandatory**

**强制**

FS_NOMOUNT is gone.  If you use it - just set SB_NOUSER in flags
(see rootfs for one kind of solution and bdev/socket/pipe for another).

FS_NOMOUNT 已不存在。如果你使用它——只需flags 中设SB_NOUSER
（参rootfs 作为一类解决方案，以及 bdev/socket/pipe 作为另一类）

---


**recommended**

**建议**

Use bdev_read_only(bdev) instead of is_read_only(kdev).  The latter
is still alive, but only because of the mess in drivers/s390/block/dasd.c.
As soon as it gets fixed is_read_only() will die.

使用 bdev_read_only(bdev) 取代 is_read_only(kdev)。后者仍然存在，但仅
是因drivers/s390/block/dasd.c 中的混乱。一旦它被修复，is_read_only()
就会消失

---


**mandatory**

**强制**

->permission() is called without BKL now. Grab it on entry, drop upon
return - that will guarantee the same locking you used to have.  If
your method or its parts do not need BKL - better yet, now you can
shift lock_kernel() and unlock_kernel() so that they would protect
exactly what needs to be protected.

->permission() 现在不再持有 BKL 时被调用。在入口处获取它，在返回时释
——这将保证与你以往拥有的相同加锁。如果你的方法或其部分不需BKL—
那就更好了，现在你可以移lock_kernel() unlock_kernel()，使它们
恰好保护需要保护的内容

---


**mandatory**

**强制**

->statfs() is now called without BKL held.  BKL should have been
shifted into individual fs sb_op functions where it's not clear that
it's safe to remove it.  If you don't need it, remove it.

->statfs() 现在不再持有 BKL 时被调用。BKL 本应被移入各个文件系统自己的
sb_op 函数，在那些尚不清楚能否安全移除它的地方。如果你不需要它，就移除它

---


**mandatory**

**强制**

is_read_only() is gone; use bdev_read_only() instead.

is_read_only() 已不存在；请改用 bdev_read_only()

---


**mandatory**

**强制**

destroy_buffers() is gone; use invalidate_bdev().

destroy_buffers() 已不存在；请改用 invalidate_bdev()

---


**mandatory**

**强制**

fsync_dev() is gone; use fsync_bdev().  NOTE: lvm breakage is
deliberate; as soon as struct block_device * is propagated in a reasonable
way by that code fixing will become trivial; until then nothing can be
done.

fsync_dev() 已不存在；请改用 fsync_bdev()。注意：lvm 的破坏是
有意为之；只struct block_device * 被该代码以合理方式传播，修复
变得轻而易举；在此之前无能为力

**mandatory**

**强制**

block truncation on error exit from ->write_begin, and ->direct_IO
moved from generic methods (block_write_begin, cont_write_begin,
nobh_write_begin, blockdev_direct_IO*) to callers.  Take a look at
ext2_write_failed and callers for an example.

->write_begin 出错退出时的块截断，以->direct_IO 已从通用方法
（block_write_begin、cont_write_begin、nobh_write_begin
blockdev_direct_IO*）移至调用方。参ext2_write_failed 及其调用方作
示例

**mandatory**

**强制**

->truncate is gone.  The whole truncate sequence needs to be
implemented in ->setattr, which is now mandatory for filesystems
implementing on-disk size changes.  Start with a copy of the old inode_setattr
and vmtruncate, and the reorder the vmtruncate + foofs_vmtruncate sequence to
be in order of zeroing blocks using block_truncate_page or similar helpers,
size update and on finally on-disk truncation which should not fail.
setattr_prepare (which used to be inode_change_ok) now includes the size checks
for ATTR_SIZE and must be called in the beginning of ->setattr unconditionally.

->truncate 已不存在。整个截断序列需要在 ->setattr 中实现，对于实现
磁盘上大小变更的文件系统，该方法现在是强制的。从旧版 inode_setattr
vmtruncate 的副本开始，并将 vmtruncate + foofs_vmtruncate 序列重新排序
使用 block_truncate_page 或类似辅助函数清零块、更新大小，最后进行不应失败的
磁盘截断。setattr_prepare（原inode_change_ok）现在包含对 ATTR_SIZE 
大小检查，且必须在 ->setattr 的开头无条件调用

**mandatory**

**强制**

->clear_inode() and ->delete_inode() are gone; ->evict_inode() should
be used instead.  It gets called whenever the inode is evicted, whether it has
remaining links or not.  Caller does **not** evict the pagecache or inode-associated
metadata buffers; the method has to use truncate_inode_pages_final() to get rid
of those. Caller makes sure async writeback cannot be running for the inode while
(or after) ->evict_inode() is called.

->clear_inode() ->delete_inode() 已不存在；应改用 ->evict_inode()
只要 inode 被逐出（无论它是否还有剩余链接），它就会被调用。调用方**不会**
逐出 pagecache inode 关联的元数据缓冲区；该方法必须使
truncate_inode_pages_final() 来清除它们。调用方确保->evict_inode()
被调用时（或之后）不会对inode 运行异步回写

->drop_inode() returns int now; it's called on final iput() with
inode->i_lock held and it returns true if filesystems wants the inode to be
dropped.  As before, inode_generic_drop() is still the default and it's been
updated appropriately.  inode_just_drop() is also alive and it consists
simply of return 1.  Note that all actual eviction work is done by caller after
->drop_inode() returns.

->drop_inode() 现在返回 int；它在最iput() 时被调用，持
inode->i_lock，如果文件系统希望丢弃该 inode 则返true。和以前一样，
inode_generic_drop() 仍是默认实现，并且已被适当更新。inode_just_drop()
也仍然有效，它只是简单地 return 1。注意，所有实际的逐出工作都由调用方在
->drop_inode() 返回后完成

As before, clear_inode() must be called exactly once on each call of
->evict_inode() (as it used to be for each call of ->delete_inode()).  Unlike
before, if you are using inode-associated metadata buffers (i.e.
mark_buffer_dirty_inode()), it's your responsibility to call
invalidate_inode_buffers() before clear_inode().

和以前一样，clear_inode() 必须在每->evict_inode() 调用时恰好调用一
（正如以前对每次 ->delete_inode() 调用那样）。与以前不同的是，如果你使用
inode 关联的元数据缓冲区（mark_buffer_dirty_inode()），clear_inode()
之前调用 invalidate_inode_buffers() 是你的责任

NOTE: checking i_nlink in the beginning of ->write_inode() and bailing out
if it's zero is not **and** **never** **had** **been** enough.  Final unlink() and iput()
may happen while the inode is in the middle of ->write_inode(); e.g. if you blindly
free the on-disk inode, you may end up doing that while ->write_inode() is writing
to it.

注意：在 ->write_inode() 开头检i_nlink，若为零就退出的做法**现在不是**
*从来都不*足够。最终的 unlink() iput() 可能在该 inode 正处
->write_inode() 过程中时发生；例如，如果你盲目释放磁盘上inode，你可能
->write_inode() 正在向它写入时这么做

---


**mandatory**

**强制**

.d_delete() now only advises the dcache as to whether or not to cache
unreferenced dentries, and is now only called when the dentry refcount goes to
0. Even on 0 refcount transition, it must be able to tolerate being called 0,
1, or more times (eg. constant, idempotent).

.d_delete() 现在只是dcache 建议是否缓存未引用的 dentries，并且现
仅在 dentry 引用计数降为 0 时被调用。即使在引用计数降为 0 的转变时，它
必须能够容忍被调0 次 次或多次（例如，常量、幂等）

---


**mandatory**

**强制**

.d_compare() calling convention and locking rules are significantly
changed. Read updated documentation in Documentation/filesystems/vfs.rst (and
look at examples of other filesystems) for guidance.

.d_compare() 的调用约定和加锁规则已发生重大变化。请阅读
Documentation/filesystems/vfs.rst 中更新后的文档（并参考其他文件系统的
示例）以获取指导

---


**mandatory**

**强制**

.d_hash() calling convention and locking rules are significantly
changed. Read updated documentation in Documentation/filesystems/vfs.rst (and
look at examples of other filesystems) for guidance.

.d_hash() 的调用约定和加锁规则已发生重大变化。请阅读
Documentation/filesystems/vfs.rst 中更新后的文档（并参考其他文件系统的
示例）以获取指导

---


**mandatory**

**强制**

dcache_lock is gone, replaced by fine grained locks. See fs/dcache.c
for details of what locks to replace dcache_lock with in order to protect
particular things. Most of the time, a filesystem only needs ->d_lock, which
protects **all** the dcache state of a given dentry.

dcache_lock 已不存在，被细粒度锁取代。关于应分别用哪些锁来取dcache_lock
以保护特定内容，详见 fs/dcache.c。大多数情况下，文件系统只需->d_lock
它保护给dentry *所* dcache 状态

---


**mandatory**

**强制**

Filesystems must RCU-free their inodes, if they can have been accessed
via rcu-walk path walk (basically, if the file can have had a path name in the
vfs namespace).

如果文件系统可能通过 rcu-walk 路径遍历被访问（基本上，即文件可能在 vfs
命名空间中拥有过路径名），则必须RCU 方式释放inode

Even though i_dentry and i_rcu share storage in a union, we will
initialize the former in inode_init_always(), so just leave it alone in
the callback.  It used to be necessary to clean it there, but not anymore
(starting at 3.2).

尽管 i_dentry i_rcu 在一个联合中共享存储，我们将inode_init_always()
中初始化前者，因此在回调中放着不管即可。过去需要在那里清理它，但现
不再需要了（自 3.2 起）

---


**recommended**

**建议**

vfs now tries to do path walking in "rcu-walk mode", which avoids
atomic operations and scalability hazards on dentries and inodes (see
Documentation/filesystems/path-lookup.txt). d_hash and d_compare changes
(above) are examples of the changes required to support this. For more complex
filesystem callbacks, the vfs drops out of rcu-walk mode before the fs call, so
no changes are required to the filesystem. However, this is costly and loses
the benefits of rcu-walk mode. We will begin to add filesystem callbacks that
are rcu-walk aware, shown below. Filesystems should take advantage of this
where possible.

vfs 现在尝试rcu-walk 模式"进行路径遍历，这避免dentries inodes 上的
原子操作及可伸缩性隐患（Documentation/filesystems/path-lookup.txt）。上
d_hash d_compare 变更就是为支持这一点所需改动的示例。对于更复杂的文件系
回调，vfs 会在调用文件系统之前退rcu-walk 模式，因此文件系统无需改动。然而，
这代价高昂并会丧rcu-walk 模式的优势。我们将开始添加感rcu-walk 的文件系
回调，如下所示。文件系统应在可能时加以利用

---


**mandatory**

**强制**

d_revalidate is a callback that is made on every path element (if
the filesystem provides it), which requires dropping out of rcu-walk mode. This
may now be called in rcu-walk mode (nd->flags & LOOKUP_RCU). -ECHILD should be
returned if the filesystem cannot handle rcu-walk. See
Documentation/filesystems/vfs.rst for more details.

d_revalidate 是一个对每个路径元素进行的回调（如果文件系统提供它），它要求
退rcu-walk 模式。现在它可能rcu-walk 模式（nd->flags & LOOKUP_RCU）下
调用。如果文件系统无法处rcu-walk，应返回 -ECHILD。详
Documentation/filesystems/vfs.rst銆。

permission is an inode permission check that is called on many or all
directory inodes on the way down a path walk (to check for exec permission). It
must now be rcu-walk aware (mask & MAY_NOT_BLOCK).  See
Documentation/filesystems/vfs.rst for more details.

permission 是一inode 权限检查，在路径遍历向下过程中对许多或全部目录 inode
调用（用于检查执行权限）。它现在必须感知 rcu-walk（mask & MAY_NOT_BLOCK）
详见 Documentation/filesystems/vfs.rst

---


**mandatory**

**强制**

In ->fallocate() you must check the mode option passed in.  If your
filesystem does not support hole punching (deallocating space in the middle of a
file) you must return -EOPNOTSUPP if FALLOC_FL_PUNCH_HOLE is set in mode.
Currently you can only have FALLOC_FL_PUNCH_HOLE with FALLOC_FL_KEEP_SIZE set,
so the i_size should not change when hole punching, even when puching the end of
a file off.

->fallocate() 中，你必须检查传入的 mode 选项。如果你的文件系统不支持穿孔
（在文件中间释放空间），则当 mode 中设置了 FALLOC_FL_PUNCH_HOLE 时必须返
-EOPNOTSUPP。目前你只能在同时设置了 FALLOC_FL_KEEP_SIZE 时使
FALLOC_FL_PUNCH_HOLE，因此穿孔时 i_size 不应改变，即使穿孔的是文件尾部

---


**mandatory**

**强制**

->get_sb() and ->mount() are gone. Switch to using the new mount API. See
Documentation/filesystems/mount_api.rst for more details.

->get_sb() ->mount() 已不存在。请切换到使用新的挂API。详
Documentation/filesystems/mount_api.rst銆。

---


**mandatory**

**强制**

->permission() and generic_permission()have lost flags
argument; instead of passing IPERM_FLAG_RCU we add MAY_NOT_BLOCK into mask.

->permission() generic_permission() 已不再有 flags 参数；我们不再传
IPERM_FLAG_RCU，而是MAY_NOT_BLOCK 加入 mask

generic_permission() has also lost the check_acl argument; ACL checking
has been taken to VFS and filesystems need to provide a non-NULL
->i_op->get_inode_acl to read an ACL from disk.

generic_permission() 也不再具check_acl 参数；ACL 检查已被纳VFS
文件系统需要提供一个非 NULL ->i_op->get_inode_acl 来从磁盘读取 ACL

---


**mandatory**

**强制**

If you implement your own ->llseek() you must handle SEEK_HOLE and
SEEK_DATA.  You can handle this by returning -EINVAL, but it would be nicer to
support it in some way.  The generic handler assumes that the entire file is
data and there is a virtual hole at the end of the file.  So if the provided
offset is less than i_size and SEEK_DATA is specified, return the same offset.
If the above is true for the offset and you are given SEEK_HOLE, return the end
of the file.  If the offset is i_size or greater return -ENXIO in either case.

如果你实现了自己->llseek()，你必须处理 SEEK_HOLE SEEK_DATA。你可以通过
返回 -EINVAL 来处理，但最好以某种方式支持它。通用处理函数假定整个文件都是
数据，并且文件末尾有一个虚拟的空洞。因此，如果提供的偏移量小于 i_size 
指定SEEK_DATA，则返回相同的偏移量。如果上述对偏移量成立且你收SEEK_HOLE
则返回文件末尾。如果偏移量i_size 或更大，则两种情况下都返-ENXIO

**mandatory**

**强制**

If you have your own ->fsync() you must make sure to call
filemap_write_and_wait_range() so that all dirty pages are synced out properly.
You must also keep in mind that ->fsync() is not called with i_mutex held
anymore, so if you require i_mutex locking you must make sure to take it and
release it yourself.

如果你有自己实现->fsync()，你必须确保调用 filemap_write_and_wait_range()
以便所有脏页被正确同步写出。你还必须记住，->fsync() 不再在持i_mutex 
被调用，因此如果你需i_mutex 加锁，你必须确保自己获取并在之后释放它

---


**mandatory**

**强制**

d_alloc_root() is gone, along with a lot of bugs caused by code
misusing it.  Replacement: d_make_root(inode).  On success d_make_root(inode)
allocates and returns a new dentry instantiated with the passed in inode.
On failure NULL is returned and the passed in inode is dropped so the reference
to inode is consumed in all cases and failure handling need not do any cleanup
for the inode.  If d_make_root(inode) is passed a NULL inode it returns NULL

d_alloc_root() 已不存在，连同由滥用它的代码引起的许bug 一起。替代者：
d_make_root(inode)。成功时 d_make_root(inode) 分配并返回一个用传入 inode 实例化的
dentry。失败时返回 NULL，并且传入的 inode 被丢弃，因此在所有情况下inode 
引用都被消费，错误处理无需inode 做任何清理。如d_make_root(inode) 被传入一
NULL inode，它返回 NULL

```

	inode = foofs_new_inode(....);
	s->s_root = d_make_root(inode);
	if (!s->s_root)
		/* Nothing needed for the inode cleanup */
		return -ENOMEM;
	...

```

---


**mandatory**

**强制**

The witch is dead!  Well, 2/3 of it, anyway.  ->d_revalidate() and
->lookup() do **not** take struct nameidata anymore; just the flags.

女巫死了！嗯，至少死2/3>d_revalidate() ->lookup() 不再接受
struct nameidata；只接受 flags

---


**mandatory**

**强制**

->create() doesn't take `struct nameidata *`; unlike the previous
two, it gets "is it an O_EXCL or equivalent?" boolean argument.  Note that
local filesystems can ignore this argument - they are guaranteed that the
object doesn't exist.  It's remote/distributed ones that might care...

->create() 不再接受 `struct nameidata *`；与前两个不同，它获得一它是
O_EXCL 或等价的吗？"布尔参数。注意本地文件系统可以忽略这个参数——它们被保证
对象不存在。真正可能在意的是远分布式文件系统…

---


**mandatory**

**强制**

FS_REVAL_DOT is gone; if you used to have it, add ->d_weak_revalidate()
in your dentry operations instead.

FS_REVAL_DOT 已不存在；如果你曾经使用它，请改在你dentry 操作中添
->d_weak_revalidate()銆。

---


**mandatory**

**强制**

vfs_readdir() is gone; switch to iterate_dir() instead

vfs_readdir() 已不存在；请改用 iterate_dir()

---


**mandatory**

**强制**

->readdir() is gone now; switch to ->iterate_shared()

->readdir() 现在已不存在；请改用 ->iterate_shared()

**mandatory**

**强制**

vfs_follow_link has been removed.  Filesystems must use nd_set_link
from ->follow_link for normal symlinks, or nd_jump_link for magic
/proc/<pid> style links.

vfs_follow_link 已被移除。文件系统必须对普通符号链接使用来->follow_link 
nd_set_link，或对特殊的 /proc/<pid> 风格链接使用 nd_jump_link

---


**mandatory**

**强制**

iget5_locked()/ilookup5()/ilookup5_nowait() test() callback used to be
called with both ->i_lock and inode_hash_lock held; the former is **not**
taken anymore, so verify that your callbacks do not rely on it (none
of the in-tree instances did).  inode_hash_lock is still held,
of course, so they are still serialized wrt removal from inode hash,
as well as wrt set() callback of iget5_locked().

iget5_locked()/ilookup5()/ilookup5_nowait() test() 回调过去在同时持
->i_lock inode_hash_lock 时被调用；前*不再**被持有，因此请确认你的回
不依赖于它（树内所有实例都没有依赖）。当然，inode_hash_lock 仍被持有，因此它
相对于从 inode 哈希中移除、以及相对于 iget5_locked() set() 回调仍然是串行的

---


**mandatory**

**强制**

d_materialise_unique() is gone; d_splice_alias() does everything you
need now.  Remember that they have opposite orders of arguments ;-/

d_materialise_unique() 已不存在；d_splice_alias() 现在完成了你需要的一切
记住它们的参数顺序是相反;-/

---


**mandatory**

**强制**

f_dentry is gone; use f_path.dentry, or, better yet, see if you can avoid
it entirely.

f_dentry 已不存在；请使用 f_path.dentry，或者最好看看你是否能完全避免使用它

---


**mandatory**

**强制**

never call ->read() and ->write() directly; use __vfs_{read,write} or
wrappers; instead of checking for ->write or ->read being NULL, look for
FMODE_CAN_{WRITE,READ} in file->f_mode.

切勿直接调用 ->read() ->write()；请使用 __vfs_{read,write} 或包装函数；
不要检->write ->read 是否NULL，而是查看 file->f_mode 中的
FMODE_CAN_{WRITE,READ}銆。

---


**mandatory**

**强制**

do _not_ use new_sync_{read,write} for ->read/->write; leave it NULL
instead.

请勿->read/->write 使用 new_sync_{read,write}；而是将其保留NULL

---


**mandatory**

	->aio_read/->aio_write are gone.  Use ->read_iter/->write_iter.

	->aio_read/->aio_write 已不存在。请使用 ->read_iter/->write_iter

---


**recommended**

**建议**

for embedded ("fast") symlinks just set inode->i_link to wherever the
symlink body is and use simple_follow_link() as ->follow_link().

对于内嵌fast"）符号链接，只需inode->i_link 设为符号链接正文所在处，并
使用 simple_follow_link() 作为 ->follow_link()

---


**mandatory**

**强制**

calling conventions for ->follow_link() have changed.  Instead of returning
cookie and using nd_set_link() to store the body to traverse, we return
the body to traverse and store the cookie using explicit void ** argument.
nameidata isn't passed at all - nd_jump_link() doesn't need it and
nd_[gs]et_link() is gone.

->follow_link() 的调用约定已改变。我们不再返cookie 并使nd_set_link() 存储
要遍历的正文，而是返回要遍历的正文，并使用显式void ** 参数存储 cookie
nameidata 根本不再传入——nd_jump_link() 不再需要它，nd_[gs]et_link() 也已消失

---


**mandatory**

**强制**

calling conventions for ->put_link() have changed.  It gets inode instead of
dentry,  it does not get nameidata at all and it gets called only when cookie
is non-NULL.  Note that link body isn't available anymore, so if you need it,
store it as cookie.

->put_link() 的调用约定已改变。它获得 inode 而非 dentry，完全不再获nameidata
并且仅在 cookie NULL 时被调用。注意链接正文不再可用，因此如果你需要它，请将其
作为 cookie 存储

---


**mandatory**

**强制**

any symlink that might use page_follow_link_light/page_put_link() must
have inode_nohighmem(inode) called before anything might start playing with
its pagecache.  No highmem pages should end up in the pagecache of such
symlinks.  That includes any preseeding that might be done during symlink
creation.  page_symlink() will honour the mapping gfp flags, so once
you've done inode_nohighmem() it's safe to use, but if you allocate and
insert the page manually, make sure to use the right gfp flags.

任何可能使用 page_follow_link_light/page_put_link() 的符号链接必须在任何可能
开始操作其 pagecache 之前调用 inode_nohighmem(inode)。此类符号链接的 pagecache 
不应出现高端内存（highmem）页。这包括在符号链接创建过程中可能做的任何预置
page_symlink() 会遵循映射的 gfp 标志，因此一旦你调用inode_nohighmem() 就可
安全使用它，但如果你是手动分配并插入页面，请确保使用正确gfp 标志

---


**mandatory**

**强制**

->follow_link() is replaced with ->get_link(); same API, except that

->follow_link() ->get_link() 取代；API 相同，除了：

 - ->get_link() gets inode as a separate argument
 - ->get_link() may be called in RCU mode - in that case NULL
	  dentry is passed

 - ->get_link() 额外获得 inode 作为参数
 - ->get_link() 可能RCU 模式下被调用——此时传
	  鐨?dentry 涓?NULL

---


**mandatory**

**强制**

->get_link() gets struct delayed_call `*done` now, and should do
set_delayed_call() where it used to set `*cookie`.

->get_link() 现在获得 struct delayed_call `*done`，并且应在过去设`*cookie`
的地方改set_delayed_call()

->put_link() is gone - just give the destructor to set_delayed_call()
in ->get_link().

->put_link() 已不存在——只需->get_link() 中将析构函数交给 set_delayed_call()

---


**mandatory**

**强制**

->getxattr() and xattr_handler.get() get dentry and inode passed separately.
dentry might be yet to be attached to inode, so do _not_ use its ->d_inode
in the instances.  Rationale: !@#!@# security_d_instantiate() needs to be
called before we attach dentry to inode.

->getxattr() xattr_handler.get() dentry inode 被分开传入。dentry 可能
尚未附加inode，因此实例中**不要**使用->d_inode。理由：!@#!@# 
security_d_instantiate() 需要在我们dentry 附加inode 之前被调用

---


**mandatory**

**强制**

symlinks are no longer the only inodes that do **not** have i_bdev/i_cdev/
i_pipe/i_link union zeroed out at inode eviction.  As the result, you can't
assume that non-NULL value in ->i_nlink at ->destroy_inode() implies that
it's a symlink.  Checking ->i_mode is really needed now.  In-tree we had
to fix shmem_destroy_callback() that used to take that kind of shortcut;
watch out, since that shortcut is no longer valid.

符号链接不再是唯一inode 逐出**i_bdev/i_cdev/i_pipe/i_link 联合清零
inode。因此，你不能再假定 ->destroy_inode() ->i_nlink 中的NULL 值意味着
它是符号链接。现在确实有必要检->i_mode。在树内我们不得不修复曾经走这种
捷径shmem_destroy_callback()；请当心，因为该捷径已不再有效

---


**mandatory**

**强制**

->i_mutex is replaced with ->i_rwsem now.  inode_lock() et.al. work as
they used to - they just take it exclusive.  However, ->lookup() may be
called with parent locked shared.  Its instances must not

->i_mutex 现在->i_rwsem 取代。inode_lock() 等的工作方式如同以往——它们只
以独占方式获取它。但是，->lookup() 可能在父目录被共享锁定时被调用。其实现必须
不：

 - use d_instantiate) and d_rehash() separately - use d_add() or
	  d_splice_alias() instead.
 - use d_rehash() alone - call d_add(new_dentry, NULL) instead.
 - in the unlikely case when (read-only) access to filesystem
	  data structures needs exclusion for some reason, arrange it
	  yourself.  None of the in-tree filesystems needed that.
 - rely on ->d_parent and ->d_name not changing after dentry has
	  been fed to d_add() or d_splice_alias().  Again, none of the
	  in-tree instances relied upon that.

 - 单独使用 d_instantiate() d_rehash()——请改用 d_add() 
	  d_splice_alias()銆。
 - 单独使用 d_rehash()——请改调d_add(new_dentry, NULL)
 - 在极少数情况下，如果（只读）访问文件系统数据结构出于某种原因需要互斥，
	  请自行安排。树内没有任何文件系统需要那样做
 - 依赖 ->d_parent ->d_name dentry 被交d_add() 
	  d_splice_alias() 后不再改变。同样，树内没有任何实例依赖这一点

We are guaranteed that lookups of the same name in the same directory
will not happen in parallel ("same" in the sense of your ->d_compare()).
Lookups on different names in the same directory can and do happen in
parallel now.

我们保证同一目录中相同名称的查找不会并行发生相同"指你->d_compare()
意义上的相同）。同一目录中不同名称的查找现在可以并且确实会并行发生

---


**mandatory**

**强制**

->iterate_shared() is added.
Exclusion on struct file level is still provided (as well as that
between it and lseek on the same struct file), but if your directory
has been opened several times, you can get these called in parallel.
Exclusion between that method and all directory-modifying ones is
still provided, of course.

新增->iterate_shared()。在 struct file 级别上的互斥仍然提供（以及它与同一
struct file 上的 lseek 之间的互斥），但如果你的目录被打开了多次，你可能会
并行地收到这些调用。当然，该方法与所有目录修改方法之间的互斥仍然提供

If you have any per-inode or per-dentry in-core data structures modified
by ->iterate_shared(), you might need something to serialize the access
to them.  If you do dcache pre-seeding, you'll need to switch to
d_alloc_parallel() for that; look for in-tree examples.

如果你有任何会被 ->iterate_shared() 修改的每 inode 或每 dentry 内存数据结构
你可能需要某种机制来串行化对它们的访问。如果你进行dcache 预置，你将需
为此切换d_alloc_parallel()；请查找树内示例

---


**mandatory**

**强制**

->atomic_open() calls without O_CREAT may happen in parallel.

不带 O_CREAT ->atomic_open() 调用可能会并行发生

---


**mandatory**

**强制**

->setxattr() and xattr_handler.set() get dentry and inode passed separately.
The xattr_handler.set() gets passed the user namespace of the mount the inode
is seen from so filesystems can idmap the i_uid and i_gid accordingly.
dentry might be yet to be attached to inode, so do _not_ use its ->d_inode
in the instances.  Rationale: !@#!@# security_d_instantiate() needs to be
called before we attach dentry to inode and !@#!@##!@$!$#!@#$!@$!@$ smack
->d_instantiate() uses not just ->getxattr() but ->setxattr() as well.

->setxattr() xattr_handler.set() dentry inode 被分开传入
xattr_handler.set() 会收到该 inode 所属挂载的用户命名空间，以便文件系
相应地对 i_uid i_gid 进行 id 映射。dentry 可能尚未附加inode，因
实例*不要**使用->d_inode。理由：!@#!@# security_d_instantiate() 需要在
我们dentry 附加inode 之前被调用，并且 !@#!@##!@$!$#!@#$!@$!@$ smack 
->d_instantiate() 不仅使用 ->getxattr()，也使用 ->setxattr()

---


**mandatory**

**强制**

->d_compare() doesn't get parent as a separate argument anymore.  If you
used it for finding the struct super_block involved, dentry->d_sb will
work just as well; if it's something more complicated, use dentry->d_parent.
Just be careful not to assume that fetching it more than once will yield
the same value - in RCU mode it could change under you.

->d_compare() 不再将父目录作为单独参数。如果你曾用它来寻找相关struct
super_block，dentry->d_sb 同样适用；如果是更复杂的情况，请使用 dentry->d_parent
只是要小心，不要假定多次获取它会得到相同的值——在 RCU 模式下它可能在你不知情时
改变

---


**mandatory**

**强制**

->rename() has an added flags argument.  Any flags not handled by the
filesystem should result in EINVAL being returned.

->rename() 新增了一flags 参数。文件系统未处理的任何标志都应导致返EINVAL

---


**recommended**

**建议**

->readlink is optional for symlinks.  Don't set, unless filesystem needs
to fake something for readlink(2).

->readlink 对符号链接是可选的。除非文件系统需要为 readlink(2) 伪造某些内容，
否则不要设置它

---


**mandatory**

**强制**

->getattr() is now passed a struct path rather than a vfsmount and
dentry separately, and it now has request_mask and query_flags arguments
to specify the fields and sync type requested by statx.  Filesystems not
supporting any statx-specific features may ignore the new arguments.

->getattr() 现在传入一struct path，而不是单独的 vfsmount dentry，并
它现在有 request_mask query_flags 参数来指statx 所请求的字段和同步类型
不支持任statx 特定特性的文件系统可以忽略这些新参数

---


**mandatory**

**强制**

->atomic_open() calling conventions have changed.  Gone is `int *opened`,
along with FILE_OPENED/FILE_CREATED.  In place of those we have
FMODE_OPENED/FMODE_CREATED, set in file->f_mode.  Additionally, return
value for 'called finish_no_open(), open it yourself' case has become
0, not 1.  Since finish_no_open() itself is returning 0 now, that part
does not need any changes in ->atomic_open() instances.

->atomic_open() 的调用约定已改变。`int *opened` 连同 FILE_OPENED/FILE_CREATED
一起消失了。取而代之的是设置在 file->f_mode 中的 FMODE_OPENED/FMODE_CREATED
此外调用finish_no_open()，由你自己打开"这种情况的返回值变成了 0 而非 1
由于 finish_no_open() 本身现在返回 0，那部分->atomic_open() 实例中不需要任
改动

---


**mandatory**

**强制**

alloc_file() has become static now; two wrappers are to be used instead.
alloc_file_pseudo(inode, vfsmount, name, flags, ops) is for the cases
when dentry needs to be created; that's the majority of old alloc_file()
users.  Calling conventions: on success a reference to new struct file
is returned and callers reference to inode is subsumed by that.  On
failure, ERR_PTR() is returned and no caller's references are affected,
so the caller needs to drop the inode reference it held.
alloc_file_clone(file, flags, ops) does not affect any caller's references.
On success you get a new struct file sharing the mount/dentry with the
original, on failure - ERR_PTR().

alloc_file() 现在已成为静态函数；应改用两个包装函数
alloc_file_pseudo(inode, vfsmount, name, flags, ops) 用于需要创dentry 的情况；
这是大多数旧 alloc_file() 用户的情形。调用约定：成功时返回一个对struct file
的引用，调用方对 inode 的引用被其吸收。失败时返回 ERR_PTR()，且不影响调用方的任
引用，因此调用方需要释放它持有inode 引用
alloc_file_clone(file, flags, ops) 不影响调用方的任何引用。成功时你获得一个与原始
共享挂载/dentry 的新 struct file，失败时——返ERR_PTR()

---


**mandatory**

**强制**

->clone_file_range() and ->dedupe_file_range have been replaced with
->remap_file_range().  See Documentation/filesystems/vfs.rst for more
information.

->clone_file_range() ->dedupe_file_range 已被 ->remap_file_range() 取代
详见 Documentation/filesystems/vfs.rst

---


**recommended**

**建议**

```

	if (IS_ERR(inode))
		return ERR_CAST(inode);
	return d_splice_alias(inode, dentry);

```

don't need to bother with the check - d_splice_alias() will do the
right thing when given ERR_PTR(...) as inode.  Moreover, passing NULL
inode to d_splice_alias() will also do the right thing (equivalent of
d_add(dentry, NULL); return NULL;), so that kind of special cases
also doesn't need a separate treatment.

无需费心做该检查——当传入inode ERR_PTR(...) 时，d_splice_alias() 会做正确
事。此外，d_splice_alias() 传入 NULL inode 也会做正确的事（等价
d_add(dentry, NULL); return NULL;），因此此类特殊情况也不需单独处理

---


**strongly recommended**

**强烈建议**

take the RCU-delayed parts of ->destroy_inode() into a new method -
->free_inode().  If ->destroy_inode() becomes empty - all the better,
just get rid of it.  Synchronous work (e.g. the stuff that can't
be done from an RCU callback, or any WARN_ON() where we want the
stack trace) **might** be movable to ->evict_inode(); however,
that goes only for the things that are not needed to balance something
done by ->alloc_inode().  IOW, if it's cleaning up the stuff that
might have accumulated over the life of in-core inode, ->evict_inode()
might be a fit.

->destroy_inode() RCU 延迟的部分提取到一个新方法 ->free_inode() 中。如
->destroy_inode() 变空——那就更好了，直接去掉它。同步工作（例如无法RCU 回调
完成的事情，或任何我们希望获得栈回溯WARN_ON()*可能**可以移到 ->evict_inode()
不过，这只适用于那些不需要用来平->alloc_inode() 所完成之事的内容。换言之，如果
它是在清理可能在内存 inode 生命周期内累积的东西>evict_inode() 可能合适

Rules for inode destruction:

inode 销毁规则：

 - if ->destroy_inode() is non-NULL, it gets called
 - if ->free_inode() is non-NULL, it gets scheduled by call_rcu()
 - combination of NULL ->destroy_inode and NULL ->free_inode is
	  treated as NULL/free_inode_nonrcu, to preserve the compatibility.

 - 如果 ->destroy_inode() NULL，则会被调用
 - 如果 ->free_inode() NULL，则会被 call_rcu() 调度
 - NULL ->destroy_inode NULL ->free_inode 的组合被视为
	  NULL/free_inode_nonrcu，以保持兼容性

Note that the callback (be it via ->free_inode() or explicit call_rcu()
in ->destroy_inode()) is **NOT** ordered wrt superblock destruction;
as the matter of fact, the superblock and all associated structures
might be already gone.  The filesystem driver is guaranteed to be still
there, but that's it.  Freeing memory in the callback is fine; doing
more than that is possible, but requires a lot of care and is best
avoided.

注意，该回调（无论是通过 ->free_inode() 还是 ->destroy_inode() 中的显式
call_rcu()）与超级块销*没有**顺序保证；事实上，超级块及所有相关结
可能早已消失。文件系统驱动保证仍然存在，但仅此而已。在回调中释放内存没问题
做更多事情是可能的，但需要极度小心，最好避免

---


**mandatory**

**强制**

DCACHE_RCUACCESS is gone; having an RCU delay on dentry freeing is the
default.  DCACHE_NORCU opts out, and only d_alloc_pseudo() has any
business doing so.

DCACHE_RCUACCESS 已不存在；在 dentry 释放时带RCU 延迟是默认行为。DCACHE_NORCU
选择退出，并且只有 d_alloc_pseudo() 才应当那样做

---


**mandatory**

**强制**

d_alloc_pseudo() is internal-only; uses outside of alloc_file_pseudo() are
very suspect (and won't work in modules).  Such uses are very likely to
be misspelled d_alloc_anon().

d_alloc_pseudo() 仅限内部使用；在 alloc_file_pseudo() 之外的使用非常可疑（且在
模块中无法工作）。此类使用很可能是把 d_alloc_anon() 拼错了

---


**mandatory**

**强制**

[should've been added in 2016] stale comment in finish_open() notwithstanding,
failure exits in ->atomic_open() instances should **NOT** fput() the file,
no matter what.  Everything is handled by the caller.

[本应2016 年加入] 尽管 finish_open() 中有过时的注释，->atomic_open() 实例
的失败退*不应** fput() 该文件，无论如何都不应。一切由调用方处理

---


**mandatory**

**强制**

clone_private_mount() returns a longterm mount now, so the proper destructor of
its result is kern_unmount() or kern_unmount_array().

clone_private_mount() 现在返回一个长期挂载（longterm mount），因此其结果的适当
析构函数kern_unmount() kern_unmount_array()

---


**mandatory**

**强制**

zero-length bvec segments are disallowed, they must be filtered out before
passed on to an iterator.

零长度的 bvec 段是不允许的，在传递给迭代器之前必须将其过滤掉

---


**mandatory**

**强制**

For bvec based itererators bio_iov_iter_get_pages() now doesn't copy bvecs but
uses the one provided. Anyone issuing kiocb-I/O should ensure that the bvec and
page references stay until I/O has completed, i.e. until ->ki_complete() has
been called or returned with non -EIOCBQUEUED code.

对于基于 bvec 的迭代器，bio_iov_iter_get_pages() 现在不再复制 bvec，而是使用提供
那一个。任何发kiocb-I/O 的人都应确保 bvec 和页引用一直保持到 I/O 完成，即直到
->ki_complete() 被调用或以非 -EIOCBQUEUED 代码返回

---


**mandatory**

**强制**

mnt_want_write_file() can now only be paired with mnt_drop_write_file(),
whereas previously it could be paired with mnt_drop_write() as well.

mnt_want_write_file() 现在只能mnt_drop_write_file() 配对，而此前它也可以与
mnt_drop_write() 配对

---


**mandatory**

**强制**

iov_iter_copy_from_user_atomic() is gone; use copy_page_from_iter_atomic().
The difference is copy_page_from_iter_atomic() advances the iterator and
you don't need iov_iter_advance() after it.  However, if you decide to use
only a part of obtained data, you should do iov_iter_revert().

iov_iter_copy_from_user_atomic() 已不存在；请使用 copy_page_from_iter_atomic()
区别在于 copy_page_from_iter_atomic() 会推进迭代器，之后你不需iov_iter_advance()
但是，如果你决定只使用所获数据的一部分，你应该iov_iter_revert()

---


**mandatory**

**强制**

Calling conventions for file_open_root() changed; now it takes struct path *
instead of passing mount and dentry separately.  For callers that used to
pass <mnt, mnt->mnt_root> pair (i.e. the root of given mount), a new helper
is provided - file_open_root_mnt().  In-tree users adjusted.

file_open_root() 的调用约定改变了；现在它接受 struct path * 而非分别传入挂载
dentry。对于过去传<mnt, mnt->mnt_root> 对（即给定挂载的根）的调用方，提供了一
新辅助函数——file_open_root_mnt()。树内用户已相应调整

---


**mandatory**

**强制**

no_llseek is gone; don't set .llseek to that - just leave it NULL instead.
Checks for "does that file have llseek(2), or should it fail with ESPIPE"
should be done by looking at FMODE_LSEEK in file->f_mode.

no_llseek 已不存在；不要将 .llseek 设为它——只需将其保留NULL。对"该文件是否有
llseek(2)，还是应当以 ESPIPE 失败"的检查应通过查看 file->f_mode 中的 FMODE_LSEEK
来完成

---


**mandatory**

**强制**

filldir_t (readdir callbacks) calling conventions have changed.  Instead of
returning 0 or -E... it returns bool now.  false means "no more" (as -E... used
to) and true - "keep going" (as 0 in old calling conventions).  Rationale:
callers never looked at specific -E... values anyway. -> iterate_shared()
instances require no changes at all, all filldir_t ones in the tree
converted.

filldir_t（readdir 回调）的调用约定已改变。它现在返回 bool，而非 0 -E...
false 表示"没有（如同过去的 -E...），true 表示"继续"（如同旧调用约定中的 0）
理由：调用方反正从未查看具体-E... 值>iterate_shared() 实例完全无需改动，树
所filldir_t 都已被转换

---


**mandatory**

**强制**

Calling conventions for ->tmpfile() have changed.  It now takes a struct
file pointer instead of struct dentry pointer.  d_tmpfile() is similarly
changed to simplify callers.  The passed file is in a non-open state and on
success must be opened before returning (e.g. by calling
finish_open_simple()).

->tmpfile() 的调用约定已改变。它现在接受一struct file 指针而非 struct dentry
指针。d_tmpfile() 也做了类似改动以简化调用方。传入的文件处于未打开状态，成功
必须在返回前打开（例如通过调用 finish_open_simple()）

---


**mandatory**

**强制**

Calling convention for ->huge_fault has changed.  It now takes a page
order instead of an enum page_entry_size, and it may be called without the
mmap_lock held.  All in-tree users have been audited and do not seem to
depend on the mmap_lock being held, but out of tree users should verify
for themselves.  If they do need it, they can return VM_FAULT_RETRY to
be called with the mmap_lock held.

->huge_fault 的调用约定已改变。它现在接受一个页阶（page order）而非 enum
page_entry_size，并且可能在未持mmap_lock 时被调用。树内所有用户都已被审查，似
不依赖持mmap_lock，但树外用户应自行核实。如果它们确实需要，可以返回
VM_FAULT_RETRY 以便在持mmap_lock 时被调用

---


**mandatory**

**强制**

The order of opening block devices and matching or creating superblocks has
changed.

打开块设备与匹配或创建超级块的顺序已改变

The old logic opened block devices first and then tried to find a
suitable superblock to reuse based on the block device pointer.

旧逻辑先打开块设备，然后尝试根据块设备指针寻找一个可复用的合适超级块

The new logic tries to find a suitable superblock first based on the device
number, and opening the block device afterwards.

新逻辑先尝试根据设备号寻找合适的超级块，之后再打开块设备

Since opening block devices cannot happen under s_umount because of lock
ordering requirements s_umount is now dropped while opening block devices and
reacquired before calling fill_super().

由于加锁顺序要求，打开块设备不能在 s_umount 下进行，因此现在在打开块设备时会释
s_umount，并在调fill_super() 之前重新获取

In the old logic concurrent mounters would find the superblock on the list of
superblocks for the filesystem type. Since the first opener of the block device
would hold s_umount they would wait until the superblock became either born or
was discarded due to initialization failure.

在旧逻辑中，并发挂载者会在文件系统类型的超级块列表中找到该超级块。由于块设备
第一个打开者会持有 s_umount，它们会等待直到该超级块要么"出生"，要么因初始化失
而被丢弃

Since the new logic drops s_umount concurrent mounters could grab s_umount and
would spin. Instead they are now made to wait using an explicit wait-wake
mechanism without having to hold s_umount.

由于新逻辑释放s_umount，并发挂载者可能获s_umount 并自旋。相反，现在它们使用
一个显式的等待-唤醒机制来等待，而无需持有 s_umount

---


**mandatory**

**强制**

The holder of a block device is now the superblock.

块设备的持有者现在是超级块

The holder of a block device used to be the file_system_type which wasn't
particularly useful. It wasn't possible to go from block device to owning
superblock without matching on the device pointer stored in the superblock.
This mechanism would only work for a single device so the block layer couldn't
find the owning superblock of any additional devices.

块设备的持有者过去是 file_system_type，这并不特别有用。如果不匹配超级块中存储
设备指针，就无法从块设备回溯到所属超级块。该机制只能用于单个设备，因此块层无
找到任何额外设备的所属超级块

In the old mechanism reusing or creating a superblock for a racing mount(2) and
umount(2) relied on the file_system_type as the holder. This was severely
underdocumented however:

在旧机制中，为竞态中mount(2) umount(2) 复用或创建超级块依赖于作为持有者的
file_system_type。然而这一点文档记录严重不足：

(1) Any concurrent mounter that managed to grab an active reference on an
    existing superblock was made to wait until the superblock either became
    ready or until the superblock was removed from the list of superblocks of
    the filesystem type. If the superblock is ready the caller would simple
    reuse it.

(1) 任何成功获取现有超级块活动引用的并发挂载者会被要求等待，直到该超级块要么
    准备就绪，要么从文件系统类型的超级块列表中被移除。如果超级块已就绪，调用
    只需复用它

(2) If the mounter came after deactivate_locked_super() but before
    the superblock had been removed from the list of superblocks of the
    filesystem type the mounter would wait until the superblock was shutdown,
    reuse the block device and allocate a new superblock.

(2) 如果挂载者在 deactivate_locked_super() 之后、但超级块从文件系统类型的超级块
    列表中被移除之前到来，挂载者会等待直到该超级块关闭，复用块设备并分配一个新
    超级块

(3) If the mounter came after deactivate_locked_super() and after
    the superblock had been removed from the list of superblocks of the
    filesystem type the mounter would reuse the block device and allocate a new
    superblock (the bd_holder point may still be set to the filesystem type).

(3) 如果挂载者在 deactivate_locked_super() 之后、且超级块已从文件系统类型的超级
    列表中被移除之后到来，挂载者会复用块设备并分配一个新的超级块（bd_holder 指针
    可能仍被设为文件系统类型）

Because the holder of the block device was the file_system_type any concurrent
mounter could open the block devices of any superblock of the same
file_system_type without risking seeing EBUSY because the block device was
still in use by another superblock.

由于块设备的持有者是 file_system_type，任何并发挂载者都可以打开同一 file_system_type
的任何超级块的块设备，而不必担心看EBUSY，因为该块设备仍被另一个超级块使用

Making the superblock the owner of the block device changes this as the holder
is now a unique superblock and thus block devices associated with it cannot be
reused by concurrent mounters. So a concurrent mounter in (2) could suddenly
see EBUSY when trying to open a block device whose holder was a different
superblock.

让超级块成为块设备的拥有者改变了这一点，因为持有者现在是一个唯一的超级块，因
与之关联的块设备不能被并发挂载者复用。所以，(2) 中的并发挂载者在尝试打开一个持有者为
不同超级块的块设备时，可能突然看EBUSY

The new logic thus waits until the superblock and the devices are shutdown in
->kill_sb(). Removal of the superblock from the list of superblocks of the
filesystem type is now moved to a later point when the devices are closed:

因此新逻辑会等待直到超级块和设备在 ->kill_sb() 中被关闭。超级块从文件系统类型的
超级块列表中移除，现在被推迟到设备关闭时

(1) Any concurrent mounter managing to grab an active reference on an existing
    superblock is made to wait until the superblock is either ready or until
    the superblock and all devices are shutdown in ->kill_sb(). If the
    superblock is ready the caller will simply reuse it.

(1) 任何成功获取现有超级块活动引用的并发挂载者会被要求等待，直到该超级块要么就绪
    要么超级块和所有设备在 ->kill_sb() 中被关闭。如果超级块已就绪，调用方只需复用它

(2) If the mounter comes after deactivate_locked_super() but before
    the superblock has been removed from the list of superblocks of the
    filesystem type the mounter is made to wait until the superblock and the
    devices are shut down in ->kill_sb() and the superblock is removed from the
    list of superblocks of the filesystem type. The mounter will allocate a new
    superblock and grab ownership of the block device (the bd_holder pointer of
    the block device will be set to the newly allocated superblock).

(2) 如果挂载者在 deactivate_locked_super() 之后、但超级块从文件系统类型的超级块
    列表中被移除之前到来，挂载者会被要求等待，直到超级块和设备->kill_sb() 中关闭，
    并且该超级块从文件系统类型的超级块列表中被移除。挂载者将分配一个新的超级块并获
    块设备的所有权（块设备bd_holder 指针将被设为新分配的超级块）

(3) This case is now collapsed into (2) as the superblock is left on the list
    of superblocks of the filesystem type until all devices are shutdown in
    ->kill_sb(). In other words, if the superblock isn't on the list of
    superblock of the filesystem type anymore then it has given up ownership of
    all associated block devices (the bd_holder pointer is NULL).

(3) 这种情况现在被并(2)，因为超级块会一直留在文件系统类型的超级块列表中，直到所
    设备->kill_sb() 中关闭。换言之，如果超级块已不在文件系统类型的超级块列表中，
    那么它已经放弃了所有关联块设备的所有权（bd_holder 指针NULL）

As this is a VFS level change it has no practical consequences for filesystems
other than that all of them must use one of the provided kill_litter_super(),
kill_anon_super(), or kill_block_super() helpers.

由于这是一VFS 级别的变更，它对文件系统没有实际影响，只是所有文件系统都必须使用
所提供kill_litter_super()、kill_anon_super() kill_block_super() 辅助函数之一

---


**mandatory**

**强制**

Lock ordering has been changed so that s_umount ranks above open_mutex again.
All places where s_umount was taken under open_mutex have been fixed up.

加锁顺序已改变，使得 s_umount 再次排在 open_mutex 之上。所有在 open_mutex 下获
s_umount 的地方都已被修正

---


**mandatory**

**强制**

export_operations ->encode_fh() no longer has a default implementation to
encode FILEID_INO32_GEN* file handles.
Filesystems that used the default implementation may use the generic helper
generic_encode_ino32_fh() explicitly.

export_operations ->encode_fh() 不再有用于编FILEID_INO32_GEN* 文件句柄的默
实现。曾使用默认实现的文件系统可以显式使用通用辅助函数 generic_encode_ino32_fh()

---


**mandatory**

**强制**

If ->rename() update of .. on cross-directory move needs an exclusion with
directory modifications, do **not** lock the subdirectory in question in your
->rename() - it's done by the caller now [that item should've been added in
28eceeda130f "fs: Lock moved directories"].

如果 ->rename() 在跨目录移动时对 .. 的更新需要目录修改的互斥，请**不要**在你
->rename() 中锁定相关的子目录——现在它由调用方完成 [该项本应28eceeda130f
"fs: Lock moved directories" 中加入]

---


**mandatory**

**强制**

On same-directory ->rename() the (tautological) update of .. is not protected
by any locks; just don't do it if the old parent is the same as the new one.
We really can't lock two subdirectories in same-directory rename - not without
deadlocks.

在同目录 ->rename() 中，.. 的（同义反复式的）更新不受任何锁保护；如果旧父目
与新父目录相同，就干脆不要做它。我们确实无法在同目录重命名中锁定两个子目录——否
会死锁

---


**mandatory**

**强制**

lock_rename() and lock_rename_child() may fail in cross-directory case, if
their arguments do not have a common ancestor.  In that case ERR_PTR(-EXDEV)
is returned, with no locks taken.  In-tree users updated; out-of-tree ones
would need to do so.

lock_rename() lock_rename_child() 在跨目录情况下可能失败，如果它们的参数没
共同祖先。此时返ERR_PTR(-EXDEV)，且不获取任何锁。树内用户已更新；树外用户需
自行更新

---


**mandatory**

**强制**

The list of children anchored in parent dentry got turned into hlist now.
Field names got changed (->d_children/->d_sib instead of ->d_subdirs/->d_child
for anchor/entries resp.), so any affected places will be immediately caught
by compiler.

锚定在父 dentry 中的子项列表现在变成hlist。字段名也改了（锚点/子项分别
->d_children/->d_sib 取代 ->d_subdirs/->d_child），因此任何受影响的地方会被编译
立即捕获

---


**mandatory**

**强制**

->d_delete() instances are now called for dentries with ->d_lock held
and refcount equal to 0.  They are not permitted to drop/regain ->d_lock.
None of in-tree instances did anything of that sort.  Make sure yours do not...

->d_delete() 实例现在在持->d_lock 且引用计数为 0 dentries 上被调用。它
不允许释重新获取 ->d_lock。树内实例都没有做这类事情。请确保你的实例也不要…

---


**mandatory**

**强制**

->d_prune() instances are now called without ->d_lock held on the parent.
->d_lock on dentry itself is still held; if you need per-parent exclusions (none
of the in-tree instances did), use your own spinlock.

->d_prune() 实例现在在父目录未持->d_lock 时被调用。dentry 自身->d_lock 仍被
持有；如果你需要每父目录的互斥（树内实例都不需要），请使用你自己的自旋锁

->d_iput() and ->d_release() are called with victim dentry still in the
list of parent's children.  It is still unhashed, marked killed, etc., just not
removed from parent's ->d_children yet.

->d_iput() ->d_release() 在受害dentry 仍位于父目录的子项列表中被调用。它
仍未被散列、被标记为已杀死等，只是尚未从父目录的 ->d_children 中移除

Anyone iterating through the list of children needs to be aware of the
half-killed dentries that might be seen there; taking ->d_lock on those will
see them negative, unhashed and with negative refcount, which means that most
of the in-kernel users would've done the right thing anyway without any adjustment.

任何遍历子项列表的人都需要意识到那里可能看到半杀死（half-killed）的 dentries；对
它们获取 ->d_lock 会看到它们是负的、未散列的，且引用计数为负，这意味着大多数内核内
用户无论如何都会做正确的事，无需任何调整

---


**recommended**

**建议**

Block device freezing and thawing have been moved to holder operations.

块设备的冻结（freezing）与解冻（thawing）已移至持有者操作

Before this change, get_active_super() would only be able to find the
superblock of the main block device, i.e., the one stored in sb->s_bdev. Block
device freezing now works for any block device owned by a given superblock, not
just the main block device. The get_active_super() helper and bd_fsfreeze_sb
pointer are gone.

在此变更之前，get_active_super() 只能找到主块设备（即存储sb->s_bdev 中的那个
的超级块。块设备冻结现在适用于给定超级块拥有的任何块设备，而不仅仅是主块设备
get_active_super() 辅助函数bd_fsfreeze_sb 指针已不存在

---


**mandatory**

**强制**

set_blocksize() takes opened struct file instead of struct block_device now
and it **must** be opened exclusive.

set_blocksize() 现在接受已打开struct file 而非 struct block_device，并且它
**必须**以独占方式打开

---


**mandatory**

**强制**

->d_revalidate() gets two extra arguments - inode of parent directory and
name our dentry is expected to have.  Both are stable (dir is pinned in
non-RCU case and will stay around during the call in RCU case, and name
is guaranteed to stay unchanging).  Your instance doesn't have to use
either, but it often helps to avoid a lot of painful boilerplate.
Note that while name->name is stable and NUL-terminated, it may (and
often will) have name->name[name->len] equal to '/' rather than '\0' -
in normal case it points into the pathname being looked up.
NOTE: if you need something like full path from the root of filesystem,
you are still on your own - this assists with simple cases, but it's not
magic.

->d_revalidate() 获得两个额外的参数——父目录inode 和我们的 dentry 预期拥有
名字。两者都是稳定的（在RCU 情况dir 被固定，RCU 情况下调用期间也会保留，
且名字保证保持不变）。你的实例不一定要使用它们，但使用它们通常有助于避免大量痛苦的
样板代码。注意，虽然 name->name 是稳定且NUL 结尾的，但它可能（且经常）使
name->name[name->len] 等于 '/' 而不'\0'——正常情况下它指向正在查找的路径名
注意：如果你需要类似从文件系统根开始的完整路径，仍需自己处理——这只能协助简单情况，
并非魔法

---


**recommended**

**建议**

kern_path_locked() and user_path_locked() no longer return a negative
dentry so this doesn't need to be checked.  If the name cannot be found,
ERR_PTR(-ENOENT) is returned.

kern_path_locked() user_path_locked() 不再返回dentry，因此无需检查这一点
如果找不到该名字，则返回 ERR_PTR(-ENOENT)

---


**recommended**

**建议**

lookup_one_qstr_excl() is changed to return errors in more cases, so
these conditions don't require explicit checks:

lookup_one_qstr_excl() 被改为在更多情况下返回错误，因此这些条件无需显式检查：

 - if LOOKUP_CREATE is NOT given, then the dentry won't be negative,
   ERR_PTR(-ENOENT) is returned instead
 - if LOOKUP_EXCL IS given, then the dentry won't be positive,
   ERR_PTR(-EEXIST) is rreturned instread

 - 如果未给LOOKUP_CREATE，则 dentry 不会是负的，改为返回
   ERR_PTR(-ENOENT)
 - 如果给定LOOKUP_EXCL，则 dentry 不会是正的，改为返回
   ERR_PTR(-EEXIST)

LOOKUP_EXCL now means "target must not exist".  It can be combined with
LOOK_CREATE or LOOKUP_RENAME_TARGET.

LOOKUP_EXCL 现在意为"目标必须不存。它可以LOOK_CREATE 
LOOKUP_RENAME_TARGET 组合

---


**mandatory**

invalidate_inodes() is gone use evict_inodes() instead.

invalidate_inodes() 已不存在；请改用 evict_inodes()

---


**mandatory**

**强制**

->mkdir() now returns a dentry.  If the created inode is found to
already be in cache and have a dentry (often IS_ROOT()), it will need to
be spliced into the given name in place of the given dentry. That dentry
now needs to be returned.  If the original dentry is used, NULL should
be returned.  Any error should be returned with ERR_PTR().

->mkdir() 现在返回一dentry。如果创建的 inode 被发现已在缓存中且拥有一dentry
（通常IS_ROOT()），则需要将其拼接进给定名字以取代给定的 dentry。现在需要返回那
dentry。如果使用了原始 dentry，则应返NULL。任何错误都应通过 ERR_PTR() 返回

In general, filesystems which use d_instantiate_new() to install the new
inode can safely return NULL.  Filesystems which may not have an I_NEW inode
should use d_drop();d_splice_alias() and return the result of the latter.

一般来说，使用 d_instantiate_new() 安装inode 的文件系统可以安全返NULL。可
没有 I_NEW inode 的文件系统应使用 d_drop();d_splice_alias() 并返回后者的结果

If a positive dentry cannot be returned for some reason, in-kernel
clients such as cachefiles, nfsd, smb/server may not perform ideally but
will fail-safe.

如果出于某种原因无法返回正的 dentry，诸cachefiles、nfsd、smb/server 等内核内
客户端可能无法达到理想表现，但会安全失败

---


** mandatory**

**强制**

lookup_one(), lookup_one_unlocked(), lookup_one_positive_unlocked() now
take a qstr instead of a name and len.  These, not the "one_len"
versions, should be used whenever accessing a filesystem from outside
that filesysmtem, through a mount point - which will have a mnt_idmap.

lookup_one()、lookup_one_unlocked()、lookup_one_positive_unlocked() 现在接受一
qstr 而非 name len。每当通过挂载点（它将有一mnt_idmap）从文件系统外部访问
文件系统时，都应使用这些而非"one_len"版本

---


** mandatory**

**强制**

Functions try_lookup_one_len(), lookup_one_len(),
lookup_one_len_unlocked() and lookup_positive_unlocked() have been
renamed to try_lookup_noperm(), lookup_noperm(),
lookup_noperm_unlocked(), lookup_noperm_positive_unlocked().  They now
take a qstr instead of separate name and length.  QSTR() can be used
when strlen() is needed for the length.

函数 try_lookup_one_len()、lookup_one_len()、lookup_one_len_unlocked() 
lookup_positive_unlocked() 已重命名try_lookup_noperm()、lookup_noperm()
lookup_noperm_unlocked()、lookup_noperm_positive_unlocked()。它们现在接qstr 而非
分开name length。当需要以 strlen() 作为长度时，可以使用 QSTR()

These function no longer do any permission checking - they previously
checked that the caller has 'X' permission on the parent.  They must
ONLY be used internally by a filesystem on itself when it knows that
permissions are irrelevant or in a context where permission checks have
already been performed such as after vfs_path_parent_lookup()

这些函数不再做任何权限检查——它们过去会检查调用方在父目录上拥'X' 权限。它
只能由文件系统在它自身上内部使用，当它知道权限无关紧要，或已在权限检查已完成的上下文
中（例如 vfs_path_parent_lookup() 之后）

---


** mandatory**

**强制**

d_hash_and_lookup() is no longer exported or available outside the VFS.
Use try_lookup_noperm() instead.  This adds name validation and takes
arguments in the opposite order but is otherwise identical.

d_hash_and_lookup() 不再被导出，也无法在 VFS 之外使用。请改用
try_lookup_noperm()。它会额外进行名字校验，并以相反顺序接受参数，除此之外完全相同

Using try_lookup_noperm() will require linux/namei.h to be included.

使用 try_lookup_noperm() 需要包linux/namei.h

---


**mandatory**

**强制**

Calling conventions for ->d_automount() have changed; we should **not** grab
an extra reference to new mount - it should be returned with refcount 1.

->d_automount() 的调用约定已改变；我*不应**对新挂载获取额外引用——它应以引用计数
1 返回

---

collect_mounts()/drop_collected_mounts()/iterate_mounts() are gone now.
Replacement is collect_paths()/drop_collected_path(), with no special
iterator needed.  Instead of a cloned mount tree, the new interface returns
an array of struct path, one for each mount collect_mounts() would've
created.  These struct path point to locations in the caller's namespace
that would be roots of the cloned mounts.

collect_mounts()/drop_collected_mounts()/iterate_mounts() 现在已不存在。取代
collect_paths()/drop_collected_path()，无需特殊迭代器。新接口不再返回克隆的挂
树，而是返回一struct path 数组，collect_mounts() 原本会为每个挂载创建一个。这
struct path 指向调用方命名空间中的位置，即那些克隆挂载的根

---


**mandatory**

**强制**

If your filesystem sets the default dentry_operations, use set_default_d_op()
rather than manually setting sb->s_d_op.

如果你的文件系统设置了默dentry_operations，请使用 set_default_d_op() 而非手动
设置 sb->s_d_op

---


**mandatory**

**强制**

d_set_d_op() is no longer exported (or public, for that matter); _if_
your filesystem really needed that, make use of d_splice_alias_ops()
to have them set.  Better yet, think hard whether you need different
->d_op for different dentries - if not, just use set_default_d_op()
at mount time and be done with that.  Currently procfs is the only
thing that really needs ->d_op varying between dentries.

d_set_d_op() 不再被导出（事实上也不再是公开的）；_如果_你的文件系统确实曾经需要它
请利d_splice_alias_ops() 来设置它们。更好的做法是，认真考虑你是否真的需要为不同
dentries 使用不同->d_op——如果不需要，只需在挂载时使用 set_default_d_op() 并了事
目前 procfs 是唯一真正需->d_op dentries 之间变化的东西

---


**highly recommended**

**高度建议**

The file operations mmap() callback is deprecated in favour of
mmap_prepare(). This passes a pointer to a vm_area_desc to the callback
rather than a VMA, as the VMA at this stage is not yet valid.

文件操作 mmap() 回调已被弃用，推荐使mmap_prepare()。它会向回调传递一个指
vm_area_desc 的指针，而非 VMA，因为在此阶VMA 尚未有效

The vm_area_desc provides the minimum required information for a filesystem
to initialise state upon memory mapping of a file-backed region, and output
parameters for the file system to set this state.

vm_area_desc 提供了文件系统在文件后备（file-backed）区域内存映射时初始化状态所需
最少信息，以及供文件系统设置该状态的输出参数

In nearly all cases, this is all that is required for a filesystem. However, if
a filesystem needs to perform an operation such a pre-population of page tables,
then that action can be specified in the vm_area_desc->action field, which can
be configured using the mmap_action_*() helpers.

在几乎所有情况下，这对文件系统而言已足够。但是，如果文件系统需要执行诸如预填充页表
之类的操作，则可以在 vm_area_desc->action 字段中指定该动作，它可以使用 mmap_action_*()
辅助函数进行配置

---


**mandatory**

**强制**

Several functions are renamed:

若干函数被重命名

- kern_path_locked -> start_removing_path
- kern_path_create -> start_creating_path
- user_path_create -> start_creating_user_path
- user_path_locked_at -> start_removing_user_path_at
- done_path_create -> end_creating_path

- kern_path_locked -> start_removing_path
- kern_path_create -> start_creating_path
- user_path_create -> start_creating_user_path
- user_path_locked_at -> start_removing_user_path_at
- done_path_create -> end_creating_path

---


**mandatory**

**强制**

Calling conventions for vfs_parse_fs_string() have changed; it does **not**
take length anymore (value ? strlen(value) : 0 is used).  If you want
a different length, use

vfs_parse_fs_string() 的调用约定已改变；它**不再**接受长度参数（使
value  strlen(value) : 0）。如果你想要不同的长度，请使

	vfs_parse_fs_qstr(fc, key, &QSTR_LEN(value, len))

instead.

来代替

---


**mandatory**

**强制**

vfs_mkdir() now returns a dentry - the one returned by ->mkdir().  If
that dentry is different from the dentry passed in, including if it is
an IS_ERR() dentry pointer, the original dentry is dput().

vfs_mkdir() 现在返回一dentry——即 ->mkdir() 所返回的那个。如果该 dentry 与传入的
dentry 不同（包括它IS_ERR() dentry 指针的情况），原dentry 会被 dput()

When vfs_mkdir() returns an error, and so both dputs() the original
dentry and doesn't provide a replacement, it also unlocks the parent.
Consequently the return value from vfs_mkdir() can be passed to
end_creating() and the parent will be unlocked precisely when necessary.

vfs_mkdir() 返回错误，从而既 dput() 原始 dentry 又不提供替代时，它还会解锁父目录
因此 vfs_mkdir() 的返回值可以传end_creating()，而父目录会在恰好必要时被解锁

---


**mandatory**

**强制**

kill_litter_super() is gone; convert to DCACHE_PERSISTENT use (as all
in-tree filesystems have done).

kill_litter_super() 已不存在；请改用 DCACHE_PERSISTENT（正如所有树内文件系统所做的
那样）

---


**mandatory**

**强制**

The ->setlease() file_operation must now be explicitly set in order to provide
support for leases. When set to NULL, the kernel will now return -EINVAL to
attempts to set a lease. Filesystems that wish to use the kernel-internal lease
implementation should set it to generic_setlease().

->setlease() 文件操作现在必须被显式设置，以提供对租约（lease）的支持。当它被设为
NULL 时，内核现在会对设置租约的尝试返-EINVAL。希望使用内核内部租约实现的文件系统
应将其设generic_setlease()

---


**mandatory**

**强制**

fs/namei.c primitives that consume filesystem references (do_renameat2(),
do_linkat(), do_symlinkat(), do_mkdirat(), do_mknodat(), do_unlinkat()
and do_rmdir()) are gone; they are replaced with non-consuming analogues
(filename_renameat2(), etc.)
Callers are adjusted - responsibility for dropping the filenames belongs
to them now.

fs/namei.c 中消费文件系统引用的原语（do_renameat2()、do_linkat()、do_symlinkat()
do_mkdirat()、do_mknodat()、do_unlinkat() do_rmdir()）已不存在；它们被非消费性的
对应物取代（filename_renameat2() 等）。调用方已相应调整——现在释放文件名的责任属
它们

---


**mandatory**

**强制**

readlink_copy() now requires link length as the 4th argument. Said length needs
to match what strlen() would return if it was ran on the string.

readlink_copy() 现在需要链接长度作为第 4 个参数。该长度需要匹配如果对该字符串运行
strlen() 会返回的值

However, if the string is freely accessible for the duration of inode's
lifetime, consider using inode_set_cached_link() instead.

但是，如果该字符串在 inode 生命周期内可自由访问，请考虑改用 inode_set_cached_link()

---


**mandatory**

**强制**

lookup_one_qstr_excl() is no longer exported - use start_creating() or
similar.

lookup_one_qstr_excl() 不再被导出——请使用 start_creating() 或类似函数

---


** mandatory**

**强制**

lock_rename(), lock_rename_child(), unlock_rename() are no
longer available.  Use start_renaming() or similar.

lock_rename()、lock_rename_child()、unlock_rename() 不再可用。请使用
start_renaming() 或类似函数

---


**recommended**

**建议**

If you really need to iterate through dentries for given inode, use
for_each_alias(dentry, inode) instead of hlist_for_each_entry; better
yet, see if any of the exported primitives could be used instead of
the entire loop.  You still need to hold ->i_lock of the inode over
either form of manual loop.

如果你确实需要遍历给inode dentries，请使用 for_each_alias(dentry, inode) 而非
hlist_for_each_entry；更好的做法是，看看是否能用任何导出的原语来取代整个循环。无
哪种形式的手动循环，你仍然需要在循环期间持有inode ->i_lock