
## Linux 虚拟文件系统概述


原始作者：Richard Gooch <rgooch@atnf.csiro.au>

- 版权所(C) 1999 Richard Gooch
- 版权所(C) 2005 Pekka Enberg


## 引言

虚拟文件系统（Virtual File System，也称为虚拟文件系统交换机，Virtual
Filesystem Switch）是内核中提供文件系统接口给用户空间程序的软件层。它还在
内核内部提供了一种抽象，使得不同的文件系统实现能够共存
VFS 系统调用 open(2)、stat(2)、read(2)、write(2)、chmod(2) 等是从进程上下文调用的。文件系统加锁在文档 Documentation/filesystems/locking.rst 中描述

### 目录项缓存（dcache
VFS 实现open(2)、stat(2)、chmod(2) 及类似的系统调用。传递给它们的路径名
参数VFS 用来在目录项缓存（也称为 dentry 缓存dcache）中进行查找。这提供
了一种非常快速的查找机制，用于将路径名（文件名）转换为特定的 dentry。Dentry
存在RAM 中，从不被保存到磁盘：它们仅为性能而存在
dentry 缓存旨在作为你整个文件空间的视图。由于大多数计算机无法同时将全部
dentry 放入 RAM，缓存中某些部分是缺失的。为了将你的路径名解析为一dentryVFS 可能不得不沿途创dentry，然后加inode。这是通过查找 inode 来完成的

### Inode 对象

单个 dentry 通常有一个指inode 的指针。Inode 是文件系统对象，例如普通文件目录、FIFO 及其他一些对象。它们要么位于磁盘上（对于块设备文件系统），要么位于
内存中（对于伪文件系统）。位于磁盘上inode 在需要时被复制到内存中，inode
的修改会被写回磁盘。单inode 可以被多dentry 指向（例如硬链接就是这样做的）
要查找一inode，需VFS 调用父目inode lookup() 方法。该方法inode
所在的特定文件系统实现安装。一VFS 拿到了所需dentry（进而拿inode），
我们就可以做那些无聊的事情了，比如用 open(2) 打开文件，或stat(2) 偷看 inode
数据。stat(2) 操作相当简单：一VFS 拿到 dentry，它就偷inode 数据并将其中
一部分传回用户空间

### File 对象

打开一个文件需要另一个操作：分配一file 结构（这是文件描述符在内核侧实现）。新分配file 结构用指dentry 的一组文件操作成员函数以及一个指dentry 的指针初始化。这些取inode 数据。然后调open() 文件方法，以便特定的
文件系统实现能够完成它的工作。你可以看到这是 VFS 执行的又一个切换。该 file
结构被放入进程的 file descriptor 表中
读取、写入和关闭文件（以及其他各VFS 操作）是通过使用用户空间文件描述来获取相应的 file 结构，然后调用所需file 结构方法以完成所需工作来完成的只要文件是打开的，它就保持 dentry 在使用中，而这又意味着 VFS inode 仍在使用中

## 注册与挂载一个文件系
要注册和注销一个文件系统，请使用以API 函数

	#include <linux/fs.h>

	extern int register_filesystem(struct file_system_type *);
	extern int unregister_filesystem(struct file_system_type *);

所传入struct file_system_type 描述了你的文件系统。当请求将某个文件系统挂到你的命名空间中的某个目录时，VFS 会调用该特定文件系统的相get_tree() 方法详见 Documentation/filesystems/mount_api.rst
你可以在 /proc/filesystems 文件中看到注册到内核的所有文件系统

### struct file_system_type

这描述了文件系统。定义了以下成员

	struct file_system_type {
		const char *name;
		int fs_flags;
		int (**init_fs_context)(struct fs_context **);
		const struct fs_parameter_spec *parameters;
		void (**kill_sb) (struct super_block **);
		struct module *owner;
		struct file_system_type * next;
		struct hlist_head fs_supers;

		struct lock_class_key s_lock_key;
		struct lock_class_key s_umount_key;
		struct lock_class_key s_vfs_rename_key;
		struct lock_class_key s_writers_key[SB_FREEZE_LEVELS];

		struct lock_class_key i_lock_key;
		struct lock_class_key i_mutex_key;
		struct lock_class_key invalidate_lock_key;
		struct lock_class_key i_mutex_dir_key;
	};

`name`
	文件系统类型的名称，例如 "ext2"iso9660"msdos" 
`fs_flags`
	各种标志（如 FS_REQUIRES_DEV、FS_NO_DCACHE 等）

`init_fs_context`
	用文件系统特定的数据初始'struct fs_context' ->ops ->fs_private 字段
`parameters`
	指向文件系统参数描述符数'struct fs_parameter_spec' 的指针	更多信息Documentation/filesystems/mount_api.rst
`kill_sb`
	当该文件系统的一个实例应当关闭时调用的方
`owner`
	VFS 内部使用：在大多数情况下你应将其初始化为 THIS_MODULE
`next`
	VFS 内部使用：你应将其初始化NULL

`fs_supers`
	VFS 内部使用：文件系统实例（超级块）hlist

  s_lock_key、s_umount_key、s_vfs_rename_key、s_writers_key  i_lock_key、i_mutex_key、invalidate_lock_key、i_mutex_dir_key：lockdep 专用

## 超级块（Superblock）对

一个超级块对象代表一个已挂载的文件系统

### struct super_operations

这描述了 VFS 如何操作你的文件系统的超级块。定义了以下成员

	struct super_operations {
		struct inode **(**alloc_inode)(struct super_block *sb);
		void (**destroy_inode)(struct inode **);
		void (**free_inode)(struct inode **);

		void (**dirty_inode) (struct inode **, int flags);
		int (**write_inode) (struct inode **, struct writeback_control *wbc);
		int (**drop_inode) (struct inode **);
		void (**evict_inode) (struct inode **);
		void (**put_super) (struct super_block **);
		int (**sync_fs)(struct super_block **sb, int wait);
		int (**freeze_super) (struct super_block **sb,
					enum freeze_holder who);
		int (**freeze_fs) (struct super_block **);
		int (**thaw_super) (struct super_block **sb,
					enum freeze_wholder who);
		int (**unfreeze_fs) (struct super_block **);
		int (**statfs) (struct dentry **, struct kstatfs *);
		void (**umount_begin) (struct super_block **);

		int (**show_options)(struct seq_file **, struct dentry *);
		int (**show_devname)(struct seq_file **, struct dentry *);
		int (**show_path)(struct seq_file **, struct dentry *);
		int (**show_stats)(struct seq_file **, struct dentry *);

		ssize_t (**quota_read)(struct super_block **, int, char *, size_t, loff_t);
		ssize_t (**quota_write)(struct super_block **, int, const char *, size_t, loff_t);
		struct dquot **(**get_dquots)(struct inode **);

		long (**nr_cached_objects)(struct super_block **,
					struct shrink_control *);
		long (**free_cached_objects)(struct super_block **,
					struct shrink_control *);
	};

除非另有说明，所有方法都在不持有任何锁的情况下调用。这意味着大多数方法可安全地阻塞。所有方法都只从进程上下文调用（即不是从中断处理程序或底半部调用）
`alloc_inode`
	该方法由 alloc_inode() 调用，为 struct inode 分配内存并初始化它。如果未
	定义此函数，则分配一个简单的 'struct inode'。通常 alloc_inode 会被用来
	分配一个更大的、其中内嵌了 'struct inode' 的结构
`destroy_inode`
	该方法由 destroy_inode() 调用，以释放struct inode 分配resource	仅当定义->alloc_inode，并且只是撤销 ->alloc_inode 所做的一切时才需要它
`free_inode`
	该方法从 RCU 回调中调用。如果你->destroy_inode 中使call_rcu() 	释放 'struct inode' 内存，那么最好在该方法中释放内存
`dirty_inode`
	inode 被标记为脏时VFS 调用。这特指 inode 自身被标记为脏，而非	数据。如果更新需要由 fdatasync() 持久化，则会flags 参数中设	I_DIRTY_DATASYNC。如果启用了 lazytime，且 struct inode 自上->dirty_inode
	调用以来更新了时间，则会flags 中设I_DIRTY_TIME
`write_inode`
	VFS 需要将一inode 写入磁盘时调用。第二个参数指示写入是否应为同步的，
	并非所有文件系统都会检查该标志
`drop_inode`
	当对 inode 的最后一次访问被放弃时调用，此时持有 inode->i_lock 自旋锁
	该方法应NULL（普UNIX 文件系统语义），或为 "inode_just_drop"（对	不希望缓inode 的文件系统——导致无i_nlink 值为何，"delete_inode" 总是
	被调用）
	"inode_just_drop()" 行为与在 put_inode() 情况下使"force_delete" 的旧做法
	等效，但没有 "force_delete()" 方法所存在的竞态
`evict_inode`
	VFS 想要驱逐（evict）一inode 时调用。调用*不会**驱pagecache 	inode 关联的元数据缓冲区；该方法必须使truncate_inode_pages_final() 	清除它们。调用者确保在 ->evict_inode() 被调用期间（或之后）不会有针对该
	inode 的异步回写运行。可选
`put_super`
	VFS 希望释放超级块（即卸载）时调用。调用时持有超级块锁
`sync_fs`
	VFS 正在写出与一个超级块关联的所有脏数据时调用。第二个参数指示该方法是
	否应等待写出完成。可选
`freeze_super`
	如果提供，则代替 ->freeze_fs 回调调用。主要区别在->freeze_super 	不获down_write(&sb->s_umount) 的情况下调用。如果文件系统实现了它并	也希望调->freeze_fs，则它必须显式地从此回调中调->freeze_fs。可选
`freeze_fs`
	VFS 锁定一个文件系统并强制其进入一致状态时调用。该方法当前被逻辑卷管理器
	（LVM）和 ioctl(FIFREEZE) 使用。可选
`thaw_super`
	VFS ->freeze_super 之后解锁一个文件系统并使其再次可写时调用。可选
`unfreeze_fs`
	VFS ->freeze_fs 之后解锁一个文件系统并使其再次可写时调用。可选
`statfs`
	VFS 需要获取文件系统统计信息时调用
`umount_begin`
	VFS 正在卸载一个文件系统时调用
`show_options`
	VFS 调用，用于显/proc/<pid>/mounts /proc/<pid>/mountinfo 的挂	选项。（挂载选项"一节）

`show_devname`
	可选。由 VFS 调用，用于显/proc/<pid>/{mounts,mountinfo,mountstats} 	设备名。如果未提供，则将使'(struct mount).mnt_devname'
`show_path`
	可选。由 VFS 调用（针/proc/<pid>/mountinfo），用于显示相对于文件系统根	挂载dentry 路径
`show_stats`
	可选。由 VFS 调用（针/proc/<pid>/mountstats），用于显示文件系统特定的挂	统计信息
`quota_read`
	VFS 调用，以从文件系统配额文件读取
`quota_write`
	VFS 调用，以向文件系统配额文件写入
`get_dquots`
	quota 调用，以获取某个特定 inode 'struct dquot' 数组。可选
`nr_cached_objects`
	由文件系统的 sb 缓存收缩函数调用，以返回它所包含的、可释放的缓存对象数量	可选
`free_cache_objects`
	由文件系统的 sb 缓存收缩函数调用，以扫描指定数量的对象的尝试释放它们	可选，但任何实现此方法的文件系统也需要实->nr_cached_objects 才能被正	调用
	我们对文件系统可能遇到的任何错误都无能为力，因此返回类型void。如VM
	试图GFP_NOFS 条件下回收，则永远不会调用它，因此该方法自身无需处理那种
	情况
	实现必须在任何所做的扫描循环中包含有条件的重调度（reschedule）调用。这使得
	VFS 能够确定合适的扫描批大小，而无需担心实现会因为大的扫描批大小而导	停顿（holdoff）问题
设置 inode 的人负责填写 "i_op" 字段。这是一个指"struct inode_operations" 指针，后者描述了可在单个 inode 上执行的方法

### struct xattr_handler


在支持扩展属性（xattr）的文件系统上，s_xattr 超级块字段指向一个以 NULL 结尾xattr 处理器数组。扩展属性是 名称:对
`name`
	指示该处理器匹配具有指定名称（如 "system.posix_acl_access"）的属性；prefix
	字段必须NULL
`prefix`
	指示该处理器匹配具有指定名称前缀（如 "user."）的所有属性；name 字段必须	NULL
`list`
	确定是否应当为某个特dentry 列出匹配xattr 处理器的属性。被某些
	listxattr 实现（如 generic_listxattr）使用
`get`
	VFS 调用，以获取某个特定扩展属性的值。该方法getxattr(2) 系统调用调用
`set`
	VFS 调用，以设置某个特定扩展属性的值。当新值为 NULL 时，调用以移除某	特定扩展属性。该方法setxattr(2) removexattr(2) 系统调用调用
当文件系统的 xattr 处理器都不匹配指定的属性名，或者文件系统不支持扩展属性时各种 `*xattr(2)` 系统调用返回 -EOPNOTSUPP

## Inode 对象


一inode 对象代表文件系统中的一个对象

### struct inode_operations

这描述了 VFS 如何操作你的文件系统中的 inode。自内核 2.6.22 起，定义了以下成员：


	struct inode_operations {
		int (**create) (struct mnt_idmap **, struct inode **,struct dentry **, umode_t, bool);
		struct dentry ** (**lookup) (struct inode **,struct dentry **, unsigned int);
		int (**link) (struct dentry **,struct inode **,struct dentry **);
		int (**unlink) (struct inode **,struct dentry *);
		int (**symlink) (struct mnt_idmap **, struct inode **,struct dentry **,const char *);
		struct dentry **(**mkdir) (struct mnt_idmap **, struct inode **,struct dentry *,umode_t);
		int (**rmdir) (struct inode **,struct dentry *);
		int (**mknod) (struct mnt_idmap **, struct inode **,struct dentry **,umode_t,dev_t);
		int (**rename) (struct mnt_idmap **, struct inode **, struct dentry **,
			       struct inode **, struct dentry **, unsigned int);
		int (**readlink) (struct dentry **, char __user *,int);
		const char **(**get_link) (struct dentry **, struct inode **,
					 struct delayed_call *);
		int (**permission) (struct mnt_idmap **, struct inode *, int);
		struct posix_acl ** (**get_inode_acl)(struct inode *, int, bool);
		int (**setattr) (struct mnt_idmap **, struct dentry **, struct iattr **);
		int (**getattr) (struct mnt_idmap **, const struct path **, struct kstat **, u32, unsigned int);
		ssize_t (**listxattr) (struct dentry **, char *, size_t);
		void (**update_time)(struct inode **inode, enum fs_update_time type,
				    int flags);
		void (**sync_lazytime)(struct inode **inode);
		int (**atomic_open)(struct inode **, struct dentry **, struct file **,
				   unsigned open_flag, umode_t create_mode);
		int (**tmpfile) (struct mnt_idmap **, struct inode **, struct file **, umode_t);
		struct posix_acl ** (**get_acl)(struct mnt_idmap **, struct dentry **, int);
	        int (**set_acl)(struct mnt_idmap **, struct dentry **, struct posix_acl **, int);
		int (**fileattr_set)(struct mnt_idmap **idmap,
				    struct dentry **dentry, struct file_kattr **fa);
		int (**fileattr_get)(struct dentry **dentry, struct file_kattr *fa);
	        struct offset_ctx **(**get_offset_ctx)(struct inode *inode);
	};

同样，除非另有说明，所有方法都在不持有任何锁的情况下调用
`create`
	open(2) creat(2) 系统调用调用。仅当你想要支持普通文件时才需要。你得到	dentry 不应inode（即它应是一个负 dentry）。这里你大概会用 d_instantiate()
	连同 dentry 与新建的 inode 一起调用
`lookup`
	VFS 需要在父目录中查找一inode 时调用。要查找的名称在 dentry 中。该
	方法必须调用 d_add() 将找到的 inode 插入 dentry。inode 结构中的 "i_count"
	字段应当递增。如果指定的 inode 不存在，则应dentry 中插入一NULL inode
	（这称为dentry）。从该例程返回错误码必须只在发生真实错误时才进行，否	使用 create(2)、mknod(2)、mkdir(2) 等系统调用创inode 将会失败。如果你希望
	重载 dentry 方法，那么你应该初始dentry 中的 "d_dop" 字段；这是一个指	struct "dentry_operations" 的指针。调用该方法时持有目inode 信号量
`link`
	link(2) 系统调用调用。仅当你想要支持硬链接时才需要。你大概需要像	create() 方法中那样调d_instantiate()
`unlink`
	unlink(2) 系统调用调用。仅当你想要支持删除 inode 时才需要
`symlink`
	symlink(2) 系统调用调用。仅当你想要支持符号链接时才需要。你大概需要像	create() 方法中那样调d_instantiate()
`mkdir`
	mkdir(2) 系统调用调用。仅当你想要支持创建子目录时才需要。你大概需要像	create() 方法中那样调d_instantiate_new()
	如果未使d_instantiate_new()，且提供fh_to_dentry() 导出操作，或者存储可	通过另一条路径（例如通过网络文件系统）被访问，则可能需要更加小心。重要的
	是，如果 inode 已不再是 I_NEW 且存在该 inode 可能已经被附加到某个 dentry 的任	可能，则不应使用 d_instantate()。这是因VFS 中一条硬性规则：一个目录只能有
	一dentry
	例如，如果一NFS 文件系统被挂载两次，新的目录可能在原始挂载点之前就在另一	挂载点上可见，并且一name_to_handle_at()、open_by_handle_at() 调用可能在第一	mkdir 返回之前，用一IS_ROOT() dentry 实例化该目录 inode
	如果存在任何这种可能性，则新inode 应当d_drop() 掉，并用 d_splice_alias()
	附加。返回的 dentry（如果有）应->mkdir() 返回
`rmdir`
	rmdir(2) 系统调用调用。仅当你想要支持删除子目录时才需要
`mknod`
	mknod(2) 系统调用调用，以创建设备（字符、块）inode 或命名管道（FIFO）或
	套接字。仅当你想要支持创建这些类型inode 时才需要。你大概需要像create()
	方法中那样调d_instantiate()
`rename`
	rename(2) 系统调用调用，以将该对象重命名为由第二个 inode dentry 给出	父目录和名称
	文件系统必须为任何不受支持或未知flags 返回 -EINVAL。当前实现了以下标志	(1) RENAME_NOREPLACE：该标志表示，如rename 的目标存在，rename 应当
	-EEXIST 失败，而非替换目标。VFS 已经检查了存在性，因此对于本地文件系统	RENAME_NOREPLACE 的实现等同于普通的 rename	(2) RENAME_EXCHANGE：交换源与目标。两者都必须存在；这VFS 检查。与普	rename 不同，源与目标可以是不同的类型
`get_link`
	VFS 调用，以跟随一个符号链接到它所指向inode。仅当你想要支持符号链接	才需要。该方法返回要遍历的符号链接体（并可能用 nd_jump_link() 重置当前位置）	如果符号链接体在 inode 消失之前都不会消失，则无需做其他事情；如果它需要以
	其他方式被固定（pinned），则通过get_link(..., ..., done) 调用
	set_delayed_call(done, destructor, argument) 来安排其释放。在那种情况下，一	VFS 处理完你返回的链接体，就会调destructor(argument)。可能在 RCU 模式	调用；这NULL dentry 参数指示。如果无法在不离开 RCU 模式的情况下处理请求	则让它返ERR_PTR(-ECHILD)
	如果文件系统将符号链接目标存储在 ->i_link 中，VFS 可能直接使用它而无需调用
	->get_link()；然而，->get_link() 仍必须提供>i_link RCU 宽限期之后才	被释放。在 iget() 之后的时间写->i_link 需要一'release' 内存屏障
`readlink`
	现在它只readlink(2) 在某些情况下使用的一个覆盖：->get_link 使用
	nd_jump_link() 或对象实际上不是符号链接时。通常文件系统应当只实->get_link
	用于符号链接，readlink(2) 将自动使用它
`permission`
	VFS 调用，以检查类 POSIX 文件系统的访问权限
	可能rcu-walk 模式下调用（mask & MAY_NOT_BLOCK）。如果在 rcu-walk 模式下，
	文件系统必须在不阻塞或不写入 inode 的情况下检查权限
	如果遇到 rcu-walk 无法处理的情况，返回 -ECHILD，它将在 ref-walk 模式下再次被调用
`setattr`
	VFS 调用，以设置文件的属性。该方法chmod(2) 及相关的系统调用调用
`getattr`
	VFS 调用，以获取文件的属性。该方法stat(2) 及相关的系统调用调用
`listxattr`
	VFS 调用，以列出给定文件的所有扩展属性。该方法listxattr(2) 系统调用调用
`update_time`
	VFS 调用，以更新 inode 的特定时间或 i_version。如果未定义此函数，VFS 	自行更新 inode 并调mark_inode_dirty_sync
`sync_lazytime`	由回写（writeback）代码调用，以将惰性时间戳更新为会被同步进磁盘 inode 	常规时间戳更新
`atomic_open`
	open 的最后一个分量上调用。使用该可选方法，文件系统可以在一次原子操作中
	查找、可能创建并打开文件。如果它想把实际的打开留给调用者（例如，如果文	结果是符号链接、设备，或只是文件系统不会进行原子打开的东西），它可以通过
	返回 finish_no_open(file, dentry) 来发出此信号。该方法仅在最后一个分量是负的
	或需要查找时才被调用。缓存的dentry 仍由 f_op->open() 处理。如果文件被创建	则应file->f_mode 中设FMODE_CREATED 标志。在 O_EXCL 的情况下，该方法必须
	仅当文件不存在时才成功，因此 FMODE_CREATED 在成功时应当总是被设置
`tmpfile`
	O_TMPFILE open() 的末尾调用。可选，等价于在给定目录中原子地创建、打开	解除链接一个文件。成功时需要返回时文件已经打开；这可以通过在末尾直接调	finish_open_simple() 来完成
`fileattr_get`
	ioctl(FS_IOC_GETFLAGS) ioctl(FS_IOC_FSGETXATTR) 上调用，以检索杂项文	标志与属性。在相关SET 操作之前也会被调用，以检查将要改变的内容（此	持有 i_rwsem 排他锁）。如果未设置，则回退f_op->ioctl()
`fileattr_set`
	ioctl(FS_IOC_SETFLAGS) ioctl(FS_IOC_FSSETXATTR) 上调用，以更改杂项文	标志与属性。调用者持i_rwsem 排他锁。如果未设置，则回退f_op->ioctl()
`get_offset_ctx`
	被调用以获取目录 inode offset 上下文。文件系统必须定义此操作才能使用
	simple_offset_dir_operations銆。
## 地址空间（Address Space）对

地址空间对象用于对页缓存（page cache）中的页进行分组与管理。它可用于跟踪一文件（或其他任何东西）中的页，并跟踪文件各段到进程地址空间的映射
地址空间可以提供若干不同但相关的服务。这些包括传达内存压力、按地址进行页查找，
以及跟踪被标记为脏（Dirty）或回写（Writeback）的页
第一个可以独立于其他服务使用。VM 可以尝试释放干净页以重用它们。为此，它可在带private 标志的干净 folio 上调->release_folio。没PagePrivate 没有外部引用的干净页将在不通知地址空间的情况下被释放
为了实现该功能，页需要被放在一LRU 上（通过 lru_cache_add），并且每当页被
使用时都需要调mark_page_active
页通常->index 保存在一个基数树（radix tree）索引中。该树维护每个页PG_Dirty
PG_Writeback 状态信息，以便可以快速找到带有这两个标志中任意一个的页
Dirty 标签主要mpage_writepages——默认的 ->writepages 方法使用。它使用该标来查找要回写的脏页。如果未使用 mpage_writepages（即地址空间提供了自己的
->writepages），PAGECACHE_TAG_DIRTY 标签几乎未被使用。write_inode_now sync_inode 确实使用它（通过 __sync_single_inode）来检->writepages 是否成功
写出了整个地址空间
Writeback 标签filemap**wait** sync_page* 函数通过 filemap_fdatawait_range
使用，以等待所有回写完成
地址空间处理器可以将额外信息附加到页上，通常使用 'struct page' 中的 'private'
字段。如果附加了此类信息，则应当设置 PG_Private 标志。这将导致各VM 例程地址空间处理器进行额外调用，以处理该数据
地址空间充当存储与应用程序之间的中介。数据以整页为单位读入地址空间，并通过
复制该页或内存映射该页提供给应用程序。数据由应用程序写入地址空间，然后通常整页写回存储，不过地址空间对写入大小有更精细的控制
读取过程本质上只需'read_folio'。写入过程更复杂，它使用 write_begin/write_end
dirty_folio 将数据写入地址空间，并使用 writepages 将数据回写至存储
从地址空间移除页需要排他地持有 inode i_rwsem，而向地址空间添加页需要排他地
持有 inode i_mapping->invalidate_lock
当数据被写入页时，应当设PG_Dirty 标志。它通常一直保持设置，直到 writepages
要求将其写出。这应当清除 PG_Dirty 并设PG_Writeback。在 PG_Dirty 被清除后任何时刻都可以实际写出。一旦确定安全，PG_Writeback 被清除
回写利用writeback_control 结构来指导操作。这writepages 操作提供了一些关回写请求的性质与原因、以及执行时约束条件的信息。它也被用来将结果信息返回给调用者
### 回写期间的错误处
大多数进行缓I/O 的应用程序会定期调用文件同步调用（fsync、fdatasync、msync sync_file_range），以确保写入的数据已经到达后备存储。当回写期间发生错误时，它们
期望在发出文件同步请求时报告该错误。在一个请求上报告错误之后，同一文件描述符上后续请求应当返回 0，除非自上次文件同步以来发生了进一步的回写错误
理想情况下，内核只会向那些确实进行了写入、但随后回写失败的文件描述符报告错误然而，通用的页缓存基础设施并不跟踪弄脏了每个单独页的文件描述符，因此无法确哪些文件描述符应当收到错误
相反，内核中通用的回写错误跟踪基础设施满足于将错误报告给在发生错误时打开的所文件描述符上fsync。在多个写入者的情况下，它们都会在后续的 fsync 上收到一个错误，
即使通过该特定文件描述符所做的所有写入都成功了（甚至即使该文件描述符上根本没任何写入）
希望使用此基础设施的文件系统应该在错误发生时调mapping_set_error，将错误记录
在地址空间中。然后，在通过它们file->fsync 操作从页缓存回写数据之后，它们应调用 file_check_and_advance_wb_err，以确保 struct file 的错误游标已经推进到后备
设备发出的错误流中的正确位置

### struct address_space_operations

这描述了 VFS 如何操作你的文件系统中文件到页缓存的映射。定义了以下成员

	struct address_space_operations {
		int (**read_folio)(struct file **, struct folio *);
		int (**writepages)(struct address_space **, struct writeback_control *);
		bool (**dirty_folio)(struct address_space **, struct folio *);
		void (**readahead)(struct readahead_control **);
		int (**write_begin)(const struct kiocb **, struct address_space *mapping,
				   loff_t pos, unsigned len,
				   struct page **pagep, void **fsdata);
		int (**write_end)(const struct kiocb **, struct address_space *mapping,
				 loff_t pos, unsigned len, unsigned copied,
				 struct folio **folio, void **fsdata);
		sector_t (**bmap)(struct address_space **, sector_t);
		void (**invalidate_folio) (struct folio **, size_t start, size_t len);
		bool (**release_folio)(struct folio **, gfp_t);
		void (**free_folio)(struct folio **);
		ssize_t (**direct_IO)(struct kiocb **, struct iov_iter *iter);
		int (**migrate_folio)(struct mapping **, struct folio *dst,
				struct folio *src, enum migrate_mode);
		int (**launder_folio) (struct folio **);

		bool (**is_partially_uptodate) (struct folio **, size_t from,
					       size_t count);
		void (**is_dirty_writeback)(struct folio **, bool **, bool **);
		int (**error_remove_folio)(struct mapping **mapping, struct folio *);
		int (**swap_activate)(struct swap_info_struct **sis, struct file **f, sector_t **span)
		int (**swap_deactivate)(struct file **);
		int (**swap_rw)(struct kiocb **iocb, struct iov_iter *iter);
	};

`read_folio`
	由页缓存调用，以从后备存储读取一foliofile' 参数为网络文件系统提供认	信息，块设备文件系统通常不使用它。如果调用者没有打开的文件（例如，如果内	正在为自己执行读取，而非代表带有打开文件的用户空间进程），它可能NULL
	如果映射不支持大 folio，则 folio 将包含单个页。调read_folio folio 会被
	锁定。如果读取成功完成，folio 应被标记uptodate。无论成功与否，文件系统	应在读取完成后解folio。文件系统无需修改 folio 上的引用计数；页缓存持有引用
	计数，并且在 folio 解锁之前不会释放它
	文件系统可以同步地实->read_folio()。在正常操作中，folio 是通过 ->readahead()
	方法读取的。只有在该方法失败，或调用者需要等待读取完成时，页缓存才会调用
	->read_folio()。文件系统不应尝试在 ->read_folio() 操作中执行自己的预读
	如果文件系统此时无法执行读取，它可以解锁 folio，执行它需要确保读取将来会成功
	所需的任何动作，并返AOP_TRUNCATED_PAGE。在这种情况下，调用者应当查folio	锁定它，并再次调->read_folio
	调用者可以直接调->read_folio() 方法，但使用 read_mapping_folio() 将负责加锁	等待读取完成，并处理 AOP_TRUNCATED_PAGE 等情况
`writepages`
	VM 调用，以写出与地址空间对象关联的页。如wbc->sync_mode WB_SYNC_ALL	writeback_control 将指定一个必须被写出的页范围。如果它WB_SYNC_NONE，则
	给出 nr_to_write，并应当尽可能多地写出那么多的页。如果没有给->writepages	则改mpage_writepages。它将从地址空间中选择被标记为 DIRTY 的页并将它们回写
`dirty_folio`
	VM 调用，以将一folio 标记为脏。如果地址空间将私有数据附加到 folio，并	该数据需要在 folio 变脏时更新，则特别需要它。例如，当一个内存映射的页被修改	就会调用它。如果定义了它，它应当设folio 的脏标志，以i_pages 中的
	PAGECACHE_TAG_DIRTY 搜索标记
`readahead`
	VM 调用，以读取与地址空间对象关联的页。这些页在页缓存中是连续的，并且	被锁定的。实现应当在对每个页启动 I/O 之后递减页引用计数。通常该页会由 I/O
	完成处理程序解锁。这组页被分成一些同步页，后跟一些异步页，rac->ra->async_size
	给出异步页的数量。文件系统应当尝试读取所有同步页，但一旦到达异步页就可以决	停止。如果它确实决定停止尝试 I/O，它可以简单地返回。调用者将从地址空间中移	剩余的页、解锁它们并递减页引用计数。如I/O 成功完成，则设置 PageUptodate
`write_begin`
	由通用的缓冲写入代码调用，以请文件系统准备在文件中给定偏移处写len 字节	地址空间应当通过必要地分配空间以及进行任何其他内部记账，来检查写入是否能	完成。如果写入将更新存储上任何基本块（basic-block）的部分，那么那些块应当	预读（如果尚未读取），以便更新后的块能够被正确写出
	文件系统必须为指定偏移处返回锁定的页缓存 folio，放`*foliop` 中，供调用	写入
	它必须能够处理短写入（传递给 write_begin 的长度大于被复制folio 中的字节	的情况）
	可以fsdata 中返回一void *，它随后被传write_end
	成功时返0；失败时返回 < 0（即错误码），此时不调用 write_end
`write_end`
	在成功的 write_begin 与数据复制之后，必须调用 write_end。len 是传write_begin
	的原len，copied 是能够复制的字节数
	文件系统必须负责解锁 folio、递减其引用计数并更新 i_size
	失败时返< 0，否则返回能够复制到页缓存中的字节数= 'copied'）
`bmap`
	VFS 调用，以将对象内的逻辑块偏移映射为物理块号。该方法FIBMAP ioctl 以及
	用于处理交换文件（swap-file）使用。为了能够交换到文件，该文件必须具有到块设备
	的稳定映射。交换系统不经过文件系统，而是使用 bmap 找出文件中块的位置并直接使用
	那些地址
`invalidate_folio`
	如果 folio 带有私有数据，那么当 folio 的部分或全部要从地址空间移除时，将调	invalidate_folio。这通常对应于截断、打洞（punch hole）或地址空间的完全失效（	后一种情况下 'offset' 总是0length' folio_size()）。任何与 folio 关联	私有数据都应当被更新以反映此截断。如offset 0 length folio_size()	则应当释放私有数据，因为 folio 必须能够被完全丢弃。这可以通过调用 ->release_folio
	函数来完成，但在这种情况下释放必须成功
`release_folio`
	release_folio 在带有私有数据的 folio 上调用，以告知文件系统该 folio 即将被释放	->release_folio 应当从该 folio 中移除任何私有数据并清除 private 标志。如	release_folio() 失败，它应当返回 false。release_folio() 用于两个不同但相关的
	情况。第一个是VM 想要释放一个没有活动用户的干净 folio 时。如->release_folio
	成功，该 folio 将从地址空间移除并被释放
	第二种情况是当请求使地址空间中的部分或全folio 失效时。这可能通过
	fadvise(POSIX_FADV_DONTNEED) 系统调用发生，或者由文件系统显式请求（如nfs 	9p 所做的，当它们认为缓存可能与存储不一致时）通过调用 invalidate_inode_pages2()	如果文件系统进行了这样的调用，并且需要确保所有的 folio 都被失效，那么它	release_folio 将需要确保这一点。如果它尚不能释放私有数据，它或许可以清uptodate
	标志
`free_folio`
	一folio 在页缓存中不再可见，就调free_folio，以允许清理任何私有数据。由	它可能由内存回收器调用，它不应假设原始的地址空间映射仍然存在，并且不应阻塞
`direct_IO`
	由通用的读/写例程调用，以执direct_IO——即绕过页缓存、直接在存储与应用程	地址空间之间传输数据I/O 请求
`migrate_folio`
	这用于压缩物理内存的使用。如VM 想要重定位一folio（也许从一个发出即	故障信号的存储设备），它会向该函数传入一个新folio 与一个旧folio	migrate_folio 应当转移任何私有数据，并更新它对 folio 的任何引用
`launder_folio`
	在释folio 之前调用——它将脏 folio 回写。为了防folio 再次变脏，它在整	操作期间保持锁定
`is_partially_uptodate`
	当通过页缓存读取文件，且底层块大小小于 folio 大小时由 VM 调用。如果所需的块
	是最新的，则读取无需 I/O 即可完成，而无需将整个页更新到最新
`is_dirty_writeback`
	VM 尝试回收一folio 时调用。VM 使用脏与回写信息来确定是否需要停顿（stall	以给 flusher 一个完成某I/O 的机会。通常它可以使folio_test_dirty 	folio_test_writeback，但某些文件系统有更复杂的状态（NFS 中不稳定folio 会阻	回收），或者由于加锁问题而不设置那些标志。该回调允许文件系统VM 指示一	folio 是否应当为了停顿的目的而被当作脏的或回写的
`error_remove_folio`
	如果对该地址空间允许截断，通常设为 generic_error_remove_folio。用于内存故	（memory failure）处理。设置它意味着你要处理页在你之下消失的情况，除非你	将它们锁定或增加了引用计数
`swap_activate`
	被调用以为给定的文件准备交换。它应当执行任何必要的验证与准备，以确保写入能够
	以最小的内存分配完成。它应当调用 add_swap_extent()，或辅助函数
	iomap_swapfile_activate()，并返回所添加的区段（extent）数量。如I/O 应当通过
	->swap_rw() 提交，它应当设置 SWP_FS_OPS，否I/O 将被直接提交到块设备
	`sis->bdev`銆。
`swap_deactivate`
	在对 swap_activate 成功的文件执swapoff 期间调用
`swap_rw`
	当设置了 SWP_FS_OPS 时调用，以读取或写入交换页
## File 对象


一file 对象代表一个被进程打开的文件。在 POSIX 术语中，这也被称打开文件
描述"（open file description）

### struct file_operations

这描述了 VFS 如何操作一个打开的文件。自内核 4.18 起，定义了以下成员：


	struct file_operations {
		struct module *owner;
		fop_flags_t fop_flags;
		loff_t (**llseek) (struct file **, loff_t, int);
		ssize_t (**read) (struct file **, char __user **, size_t, loff_t **);
		ssize_t (**write) (struct file **, const char __user **, size_t, loff_t **);
		ssize_t (**read_iter) (struct kiocb **, struct iov_iter *);
		ssize_t (**write_iter) (struct kiocb **, struct iov_iter *);
		int (**iopoll)(struct kiocb **kiocb, struct io_comp_batch *,
				unsigned int flags);
		int (**iterate_shared) (struct file **, struct dir_context *);
		__poll_t (**poll) (struct file **, struct poll_table_struct *);
		long (**unlocked_ioctl) (struct file **, unsigned int, unsigned long);
		long (**compat_ioctl) (struct file **, unsigned int, unsigned long);
		int (**mmap) (struct file **, struct vm_area_struct *);
		int (**open) (struct inode **, struct file *);
		int (**flush) (struct file **, fl_owner_t id);
		int (**release) (struct inode **, struct file *);
		int (**fsync) (struct file **, loff_t, loff_t, int datasync);
		int (**fasync) (int, struct file **, int);
		int (**lock) (struct file **, int, struct file_lock *);
		unsigned long (**get_unmapped_area)(struct file **, unsigned long, unsigned long, unsigned long, unsigned long);
		int (*check_flags)(int);
		int (**flock) (struct file **, int, struct file_lock *);
		ssize_t (**splice_write)(struct pipe_inode_info **, struct file **, loff_t **, size_t, unsigned int);
		ssize_t (**splice_read)(struct file **, loff_t **, struct pipe_inode_info **, size_t, unsigned int);
		void (**splice_eof)(struct file **file);
		int (**setlease)(struct file **, int, struct file_lease **, void **);
		long (**fallocate)(struct file **file, int mode, loff_t offset,
				  loff_t len);
		void (**show_fdinfo)(struct seq_file **m, struct file *f);
	#ifndef CONFIG_MMU
		unsigned (**mmap_capabilities)(struct file **);
	#endif
		ssize_t (**copy_file_range)(struct file **, loff_t, struct file *,
				loff_t, size_t, unsigned int);
		loff_t (**remap_file_range)(struct file **file_in, loff_t pos_in,
					   struct file *file_out, loff_t pos_out,
					   loff_t len, unsigned int remap_flags);
		int (**fadvise)(struct file **, loff_t, loff_t, int);
		int (**uring_cmd)(struct io_uring_cmd **ioucmd, unsigned int issue_flags);
		int (**uring_cmd_iopoll)(struct io_uring_cmd **, struct io_comp_batch *,
					unsigned int poll_flags);
		int (**mmap_prepare)(struct vm_area_desc **);
	};

同样，除非另有说明，所有方法都在不持有任何锁的情况下调用
`llseek`
	VFS 需要移动文件位置索引时调用

`read`
	read(2) 及相关的系统调用调用

`read_iter`
	可能异步的读取，iov_iter 为目
`write`
	write(2) 及相关的系统调用调用

`write_iter`
	可能异步的写入，iov_iter 为源

`iopoll`
	aio 想要HIPRI iocb 上轮询完成时调用

`iterate_shared`
	VFS 需要读取目录内容时调用

`poll`
	当进程想要检查该文件上是否有活动，并（可选地）一直睡眠直到有活动时，VFS
	调用。由 select(2) poll(2) 系统调用调用
`unlocked_ioctl`
	ioctl(2) 系统调用调用
`compat_ioctl`
	当在 64 位内核上使用 32 位系统调用时，由 ioctl(2) 系统调用调用
`mmap`
	mmap(2) 系统调用调用。已废弃，推荐使`mmap_prepare`
`open`
	当应当打开一inode 时由 VFS 调用。当 VFS 打开一个文件时，它创建一个新	"struct file"。然后它为这个新分配的文件结构调open 方法。你也许会认open
	方法确实属于 "struct inode_operations"，你也许是对的。我想它是以现在这种方式
	完成的，因为这让文件系统实现起来更简单。如果你想要指向一个设备结构，open()
	方法是初始化 file 结构中的 "private_data" 成员的好地方
`flush`
	close(2) 系统调用调用，以刷新一个文件
`release`
	当对一个打开文件的最后一次引用被关闭时调用
`fsync`
	fsync(2) 系统调用调用。另见上文标题为"回写期间的错误处一节
`fasync`
	当为文件启用异步（非阻塞）模式时，由 fcntl(2) 系统调用调用
`lock`
	fcntl(2) 系统调用针对 F_GETLK、F_SETLK F_SETLKW 命令调用
`get_unmapped_area`
	mmap(2) 系统调用调用
`check_flags`
	fcntl(2) 系统调用针对 F_SETFL 命令调用
`flock`
	flock(2) 系统调用调用
`splice_write`
	VFS 调用，以将数据从管道拼接（splice）到文件。该方法splice(2) 系统调用使用
`splice_read`
	VFS 调用，以将数据从文件拼接到管道。该方法splice(2) 系统调用使用
`setlease`
	VFS 调用，以设置或释放文件锁租约（lease）。希望使用内核内部租约实现的本地
	文件系统应将此设generic_setlease()。其setlease 实现应在设置之后调用
	generic_setlease() 来记录或移除 inode 中的租约。当设为 NULL 时，尝试设置或移	租约将返-EINVAL
`fallocate`
	VFS 调用，以预分配块或打洞（punch a hole）
`copy_file_range`
	copy_file_range(2) 系统调用调用
`remap_file_range`
	ioctl(2) 系统调用针对 FICLONERANGE、FICLONE FIDEDUPERANGE 命令调用，以重映	文件范围。一个实现应当将源文pos_in 处的 len 字节重映射到目标文件 pos_out 处	实现必须处理调用者传len == 0 的情况；这意味着"重映射到源文件的末尾"。返回	应是被重映射的字节数，或者如果在任何字节被重映射之前发生错误，则是通常的负错误码	remap_flags 参数接受 REMAP_FILE_* 标志。如果设置了 REMAP_FILE_DEDUP，则实现必须
	仅在所请求的文件范围内容完全相同时才重映射。如果设置了 REMAP_FILE_CAN_SHORTEN	调用者可以接受实现缩短请求长度以满足对齐EOF 要求（或任何其他原因）
`fadvise`
	可能fadvise64() 系统调用调用
`mmap_prepare`
	mmap(2) 系统调用调用。允VFS 建立文件支持的（file-backed）内存映射，最显著	是建立相关的私有状态与 VMA 回调
	如果还需要进一步的操作，例如页表的预填充（pre-population），这可以通过
	vm_area_desc->action 字段及相关的参数来指定
注意，文件操作是inode 所在的特定文件系统实现的。当打开一个设备节点（字符或块
特殊文件）时，大多数文件系统会调VFS 中的特殊支持例程，这些例程将定位所需的设驱动信息。这些支持例程将文件系统的文件操作替换为设备驱动的那些操作，然后继续调用
该文件新open() 方法。这就是在文件系统中打开一个设备文件最终会调用到设备驱open() 方法的方式

## 目录项缓存（dcache


### struct dentry_operations

这描述了文件系统如何重载标准dentry 操作。Dentry dcache VFS 与各个文件系实现的地盘。设备驱动与此无关。这些方法可以设NULL，因为它们要么是可选的，要VFS 使用默认值。自内核 2.6.22 起，定义了以下成员：


	struct dentry_operations {
		int (**d_revalidate)(struct inode **, const struct qstr *,
				    struct dentry *, unsigned int);
		int (**d_weak_revalidate)(struct dentry **, unsigned int);
		int (**d_hash)(const struct dentry **, struct qstr *);
		int (**d_compare)(const struct dentry **,
				 unsigned int, const char **, const struct qstr **);
		int (**d_delete)(const struct dentry **);
		int (**d_init)(struct dentry **);
		void (**d_release)(struct dentry **);
		void (**d_iput)(struct dentry **, struct inode *);
		char **(**d_dname)(struct dentry **, char **, int);
		struct vfsmount **(**d_automount)(struct path *);
		int (**d_manage)(const struct path **, bool);
		struct dentry **(**d_real)(struct dentry *, enum d_real_type type);
		bool (**d_unalias_trylock)(const struct dentry **);
		void (**d_unalias_unlock)(const struct dentry **);
	};

`d_revalidate`
	VFS 需要重新验证（revalidate）一dentry 时调用。每当名称查找在 dcache 	找到一dentry 时就会调用它。大多数本地文件系统将其保留NULL，因为它们在
	dcache 中的所dentry 都是有效的。网络文件系统则不同，因为服务器上的事情可以	客户端未必知情的情况下发生变化
	如果 dentry 仍然有效，该函数应返回一个正值；如果无效，则返回零或一个负的错误码
	d_revalidate 可能rcu-walk 模式下调用（flags & LOOKUP_RCU）。如果在 rcu-walk
	模式下，文件系统必须在不阻塞或不写入 dentry 的情况下重新验证 dentry，d_parent 	d_inode 不应在没有小心的情况下使用（因为它们可能改变，并且在 d_inode 的情况下	甚至可能在我们的处理过程中变NULL）
	如果遇到 rcu-walk 无法处理的情况，返回 -ECHILD，它将在 ref-walk 模式下再次被调用
`d_weak_revalidate`
	VFS 需要重新验证一跳过（jumped）dentry 时调用。这在一个路径遍历结束于
	一个不是通过在父目录中查找而获得的 dentry 时调用。这包括 "/"." ".."	以及 procfs 风格的符号链接与挂载点遍历
	在这种情况下，我们较少关dentry 是否仍然完全正确，而更关心 inode 是否仍然有效	d_revalidate 一样，大多数本地文件系统会将其设为 NULL，因为它们的 dcache 条目
	总是有效的
	该函数的返回码语义与 d_revalidate 相同
	d_weak_revalidate 只在离开 rcu-walk 模式之后调用
`d_hash`
	VFS 将一dentry 加入哈希表时调用。传d_hash 的第一dentry 是要将名	哈希到的父目录
	关于什么可以安全解引用等，d_compare 有相同的加锁与同步规则
`d_compare`
	调用以将 dentry 名称与给定名称比较。第一dentry 是要比较dentry 的父目录	第二个是dentry。len name 字符串是要比较的 dentry 的属性。qstr 是要与之
	比较的名称
	必须是常量且幂等的，并应尽可能不加锁，且不应写入 dentry。不应在没有大量小心	情况下解引用 dentry 之外的指针（例如，不应使d_parent、d_inode、d_name）
	然而，我们vfsmount 是被固定的，且持RCU，因dentry inode 不会消失，我	sb 或文件系统模块也不会。可以使->d_sb
	这是一个棘手的调用约定，因为它需要在"rcu-walk"下调用，即没有任何锁或对事物	引用
`d_delete`
	当对一dentry 的最后一次引用被放弃、且 dcache 正在决定是否缓存它时调用。返1
	表示立即删除，或返回 0 表示缓存dentry。默认是 NULL，意味着总是缓存一个可达的
	dentry。d_delete 必须是常量且幂等的
`d_init`
	当一dentry 被分配时调用
`d_release`
	当一dentry 真正被释放时调用
`d_iput`
	当一dentry 失去inode 时（就在它被释放之前）调用。当它为 NULL 时的默认行为	VFS 调用 iput()。如果你定义了该方法，则必须自己调用 iput()
`d_dname`
	当需要生成一dentry 的路径名时调用。对某些伪文件系统（sockfs、pipefs 等）有用	用于延迟路径名生成。（不是dentry 创建时做，而是仅在需要路径时才做。）真实文件
	系统大概不想要使用它，因为它们的 dentry 存在于全局 dcache 哈希中，因此它们的哈	应当是不变的。由于没有持锁，d_dname() 不应尝试修改 dentry 本身，除非使用了适当	SMP 安全手段。注意：d_path() 的逻辑相当棘手。例如返"Hello" 的正确方式是将它
	放在缓冲区的末尾，并返回一个指向第一个字符的指针。提供了 dynamic_dname() 辅助
	函数来处理这件事
	示例

	static char **pipefs_dname(struct dentry **dent, char *buffer, int buflen)
	{
		return dynamic_dname(dentry, buffer, buflen, "pipe:[%lu]",
				dentry->d_inode->i_ino);
	}

`d_automount`
	当要遍历一个自动挂载（automount）dentry 时调用（可选）。这应当创建一个新VFS
	挂载记录，并将该记录返回给调用者。调用者被提供一path 参数，给出用于描述自动挂	目标的自动挂载目录，以及提供可继承挂载参数的VFS 挂载记录。如果其他人率先完成	自动挂载，则应返NULL。如vfsmount 创建失败，则应返回一个错误码。如果返	-EISDIR，则该目录将被视为普通目录并返还pathwalk 以继续遍历
	如果返回了一vfsmount，调用者将尝试将其挂载到挂载点上，并在失败的情况下将其	过期列表中移除
	该函数仅dentry 上设置了 DCACHE_NEED_AUTOMOUNT 时使用。如果添加到 inode 时设置了
	S_AUTOMOUNT，则这由 __d_instantiate() 设置
`d_manage`
	调用以允许文件系统管理从一dentry 的过渡（可选）。这允许 autofs 例如留住等待	"挂载后面探索的客户端，同时让守护进程过去并在那里构造子树。应当返0 以让调用
	进程继续。可以返-EISDIR 以告pathwalk 将该目录用作普通目录，忽略挂载在其上的
	任何东西，并且不检查自动挂载标志。任何其他错误码将完全中pathwalk
	如果 'rcu_walk' 参数为真，则调用者正RCU-walk 模式下进行路径遍历。在该模式下
	不允许睡眠，并且可以通过返回 -ECHILD 来请调用者离开该模式并再次调用。也可以返回
	-EISDIR 以告pathwalk 忽略 d_automount 或任何挂载
	该函数仅在正被离开dentry 上设置了 DCACHE_MANAGE_TRANSIT 时使用
`d_real`
	overlay/union 类型文件系统实现此方法，以返回被 overlay 隐藏的普通文件的一个底	dentry
	'type' 参数取D_REAL_DATA D_REAL_METADATA，用于返回指向托管该文件数据	元数据的 inode 的真实底dentry
	对于非普通文件，返回 'dentry' 参数
`d_unalias_trylock`
	如果存在，将d_splice_alias() 在移动一个预先存在的已附加别名之前调用。返false
	会阻__d_move()，使 d_splice_alias() -ESTALE 失败
	理由：设FS_RENAME_DOES_D_MOVE 将阻止来自文件系统方法外部的 d_move() 	d_exchange() 调用；然而，它不能保证已附加dentry 不会d_splice_alias() 找到
	目录 inode 的预先存在的别名而重命名或移动。通常我们不会在意；不过，有某种东西想	在阻塞操作期间稳定整个到根的路径时可能需要它。参9p 作为一个（且希望是唯一的）
	例子
`d_unalias_unlock`
	应与 `d_unalias_trylock` 配对；后者在 __d_unalias() 中的 __d_move() 调用之后调用

每个 dentry 都有一个指向其dentry 的指针，以及一个子 dentry 的哈希列表。子 dentry
基本上就像目录中的文件

### 目录项缓API

定义了许多允许文件系统操dentry 的函数：

`dget`
	为已存在dentry 打开一个新句柄（这只是递增使用计数）
`dput`
	关闭一dentry 的句柄（递减使用计数）。如果使用计数降0，且 dentry 仍在它父
	目录的哈希中，则调用 "d_delete" 方法检查它是否应当被缓存。如果不应被缓存，或	如果 dentry 未被哈希，则它被删除。否则，缓存dentry 被放入一LRU 列表，以便在
	内存不足时被回收
`d_drop`
	这将一dentry 从它父目录的哈希列表中取消哈希。随后对 dput() 的调用将在此 dentry
	的使用计数降0 时释放它
`d_delete`
	删除一dentry。如果没有其他对dentry 的打开引用，则dentry 被转为一个负 dentry
	（调d_iput() 方法）。如果有其他引用，则改为调用 d_drop()
`d_add`
	将一dentry 添加到它父目录的哈希列表，然后调d_instantiate()
`d_instantiate`
	将一dentry 添加到该 inode 的别名哈希列表，并更"d_inode" 成员。inode 结构中的
	"i_count" 成员应当被设递增。如inode 指针NULL，该 dentry 被称dentry"	当一inode 为已存在的负 dentry 创建时，通常会调用此函数
`d_lookup`
	给定其父目录与路径名分量，查找一dentry。它dcache 哈希表中查找具有该名称的
	子项。如果找到，则递增引用计数并返回该 dentry。调用者使用完毕后必须dput() 释放
	璇?dentry銆。

## 挂载选项



### 解析选项

在挂载与重新挂载时，文件系统会收到一个字符串，其中包含以逗号分隔的挂载选项列表选项可以是以下两种形式之一
  option
  option=value

<linux/parser.h> 头文件定义了一个有助于解析这些选项API。在现有的文件系统中，有
大量如何使用它的示例

### 显示选项

如果一个文件系统接受挂载选项，它必须定义 show_options() 以显示所有当前活动的选项规则是：

  - 必须显示那些非默认、或其值与默认值不同的选项

  - 可以显示那些默认启用、或具有默认值的选项

仅在挂载辅助程序与内核之间内部使用（例如文件描述符），或仅在挂载期间起作用（例如
控制日志（journal）创建的那些）的选项免于上述规则
上述规则的根本原因是确保可以基于 /proc/mounts 中找到的信息准确地复制一次挂载（例如
卸载后再次挂载）

## 资源


（注意，其中一些资源未与最新内核版本保持同步。）

Creating Linux virtual filesystems. 2002
    <https://lwn.net/Articles/13325/>

The Linux Virtual File-system Layer by Neil Brown. 1999
    <http://www.cse.unsw.edu.au/~neilb/oss/linux-commentary/vfs.html>

A tour of the Linux VFS by Michael K. Johnson. 1996
    <https://www.tldp.org/LDP/khg/HyperNews/get/fs/vfstour.html>

A small trail through the Linux kernel by Andries Brouwer. 2001
    <https://www.win.tue.nl/~aeb/linux/vfs/trail.html>
