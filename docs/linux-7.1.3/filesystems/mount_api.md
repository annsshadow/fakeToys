
## 文件系统挂载 API（Filesystem Mount API）


 (1) 概述。

 (2) 文件系统上下文（filesystem context）。

 (3) 文件系统上下文操作。

 (4) 文件系统上下文安全。

 (5) VFS 文件系统上下文 API。

 (6) 超级块（superblock）创建辅助函数。

 (7) 参数描述。

 (8) 参数辅助函数。


## 概述

现在，新挂载的创建要在一个多步骤的过程中完成：

 (1) 创建一个文件系统上下文。

 (2) 解析参数并将它们附加到上下文。参数预期从用户空间逐个传入，不过也可以处理传统的二进制参数。

 (3) 验证并预处理上下文。

 (4) 获取或创建一个超级块以及可挂载的根。

 (5) 执行挂载。

 (6) 返回一条附加到上下文的错误消息。

 (7) 销毁上下文。

```

	int (*init_fs_context)(struct fs_context *fc);
	const struct fs_parameter_description *parameters;

```
第一个在设置文件系统上下文的文件系统相关部分时被调用，包括额外的空间；第二个指向参数描述，用于在注册时验证，以及供未来的系统调用查询。

注意，安全初始化是在调用文件系统**之后**完成的，以便可以首先调整命名空间。


## 文件系统上下文

超级块的创建与重新配置由一个文件系统
```

	struct fs_context {
		const struct fs_context_operations *ops;
		struct file_system_type *fs_type;
		void			*fs_private;
		struct dentry		*root;
		struct user_namespace	*user_ns;
		struct net		*net_ns;
		const struct cred	*cred;
		char			*source;
		char			*subtype;
		void			*security;
		void			*s_fs_info;
		unsigned int		sb_flags;
		unsigned int		sb_flags_mask;
		unsigned int		s_iflags;
		enum fs_context_purpose	purpose:8;
		...
	};

```
fs_context 的字段如下：

```

       const struct fs_context_operations *ops

     这些是可以在文件系统上下文上执行的操作（见下文）。这必须由 ->init_fs_context() file_system_type 操作设置。

   * ::

       struct file_system_type *fs_type

     指向正在构建或重新配置的文件系统的 file_system_type 的指针。这会保留对类型所有者的一个引用。

   * ::

       void *fs_private

     指向文件系统私有数据的指针。文件系统需要将其解析出的任何选项存储在这里。

   * ::

       struct dentry *root

     指向可挂载树的根（以及间接指向其超级块）的指针。这由 ->get_tree() 操作填充。如果设置了它，也必须持有对 root->d_sb 的一个活动引用。

   * ::

       struct user_namespace *user_ns
       struct net *net_ns

     这是调用进程所使用的命名空间的一个子集。它们保留对每个命名空间的引用。订阅的命名空间可能被文件系统替换，以反映其他来源，例如自动挂载（automount）时父挂载的超级块。

   * ::

       const struct cred *cred

     挂载者的凭证。这保留对凭证的一个引用。

   * ::

       char *source

     这指定了来源。它可以是一个块设备（例如 /dev/sda1），或一些更特殊的东西，例如 NFS 所期望的 "host:/path"。

   * ::

       char *subtype

     这是一个要添加到 /proc/mounts 中显示的类型的字符串，用于限定它（由 FUSE 使用）。如果文件系统需要，可以设置它。

   * ::

       void *security

     LSM 用来挂接其超级块安全数据的地方。相关的安全操作在下面描述。

   * ::

       void *s_fs_info

     为新的超级块建议的 s_fs_info，由 sget_fc() 设置在超级块中。这可用于区分超级块。

   * ::

       unsigned int sb_flags
       unsigned int sb_flags_mask

     要在 super_block::s_flags 中设置/清除哪些 SB_* 标志位。

   * ::

       unsigned int s_iflags

     这些将在创建超级块时与 s->s_iflags 做按位或。

   * ::

       enum fs_context_purpose

     这表示上下文的用途。可用的值有：

	==========================	======================================
	FS_CONTEXT_FOR_MOUNT,		New superblock for explicit mount
	FS_CONTEXT_FOR_SUBMOUNT		New automatic submount of extant mount
	FS_CONTEXT_FOR_RECONFIGURE	Change an existing mount
	==========================	======================================

```
挂载上下文通过调用 vfs_new_fs_context() 或 vfs_dup_fs_context() 创建，并通过 put_fs_context() 销毁。注意该结构没有被引用计数。

VFS、安全和文件系统的挂载选项用 vfs_parse_mount_option() 逐个设置。由旧的 mount(2) 系统调用作为一页数据提供的选项可以用 generic_parse_monolithic() 解析。

在挂载时，文件系统被允许从任何指针中取走数据并将其附加到超级块（或其他什么），前提是它清除了挂载上下文中的指针。

文件系统也被允许分配资源并用挂载上下文固定它们。例如，NFS 可能会固定相应的协议版本模块。


## 文件系统上下文操作

```

	struct fs_context_operations {
		void (*free)(struct fs_context *fc);
		int (*dup)(struct fs_context *fc, struct fs_context *src_fc);
		int (*parse_param)(struct fs_context *fc,
				   struct fs_parameter *param);
		int (*parse_monolithic)(struct fs_context *fc, void *data);
		int (*get_tree)(struct fs_context *fc);
		int (*reconfigure)(struct fs_context *fc);
	};

```
这些操作在挂载过程的各个阶段被调用来管理文件系统上下文。它们如下：

```

	void (*free)(struct fs_context *fc);

     当上下文被销毁时调用，用于清理文件系统上下文的文件系统相关部分。它应当意识到上下文的某些部分可能已被移除并被设为 NULL（由 ->get_tree() 完成）。

   * ::

	int (*dup)(struct fs_context *fc, struct fs_context *src_fc);

     当文件系统上下文被复制时调用，以复制文件系统私有数据。可以返回一个错误来指示复制失败。

     .. Warning::

         注意，即使这失败了，put_fs_context() 也会紧接其后被调用，因此 ->dup() *必须* 让文件系统私有数据对 ->free() 是安全的。

   * ::

	int (*parse_param)(struct fs_context *fc,
			   struct fs_parameter *param);

     当向文件系统上下文添加参数时调用。param 指向键名，可能还有一个值对象。VFS 相关的选项将已被剔除，并且 fc->sb_flags 已在上下文中更新。安全选项也将已被剔除，并且 fc->security 已更新。

     参数可以用 fs_parse() 和 fs_lookup_param() 来解析。注意来源（source）是作为名为 "source" 的参数呈现的。

     如果成功，应返回 0，否则返回一个负的错误码。

   * ::

	int (*parse_monolithic)(struct fs_context *fc, void *data);

     当调用 mount(2) 系统调用以一次性传入整个数据页时调用。如果预期这只是一个由逗号分隔的 "key[=val]" 条目列表，那么可以将其设为 NULL。

     返回值与 ->parse_param() 相同。

     如果文件系统（例如 NFS）需要先检查数据，然后发现它是标准的键-值列表，那么它可以转交给 generic_parse_monolithic()。

   * ::

	int (*get_tree)(struct fs_context *fc);

     调用以获取或创建可挂载的根与超级块，使用存储在文件系统上下文中的信息（重新配置通过一个不同的向量进行）。它可以将其想要的任何资源从文件系统上下文分离，并转移到它创建的超级块上。

     成功时它应将 fc->root 设置为可挂载的根并返回 0。在出错的情况下，它应返回一个负的错误码。

     在用户空间驱动的上下文上，该阶段会被设置为只允许在任何特定上下文上调用一次。

   * ::

	int (*reconfigure)(struct fs_context *fc);

     调用以使用文件系统上下文中存储的信息来实施超级块的重新配置。它可以将其想要的任何资源从文件系统上下文分离，并转移到超级块。超级块可以从 fc->root->d_sb 找到。

     成功时它应返回 0。在出错的情况下，它应返回一个负的错误码。


```
## 文件系统上下文安全

文件系统上下文包含一个安全指针，LSM 可以用它来为要挂载的超级块构建安全上下文。新的挂载代码为此目的使用了若干操作：

```

	int security_fs_context_alloc(struct fs_context *fc,
				      struct dentry *reference);

     调用以初始化 fc->security（它被预设为 NULL）并分配所需资源。成功应返回 0，失败返回负的错误码。

     reference 在上下文是为超级块重新配置（FS_CONTEXT_FOR_RECONFIGURE）而创建时为非 NULL，此时它指向要重新配置的超级块的根 dentry。在子挂载（FS_CONTEXT_FOR_SUBMOUNT）的情况下它也为非 NULL，此时它指向自动挂载点。

   * ::

	int security_fs_context_dup(struct fs_context *fc,
				    struct fs_context *src_fc);

     调用以初始化 fc->security（它被预设为 NULL）并分配所需资源。原始的文件系统上下文由 src_fc 指向，可用来参考。成功应返回 0，失败返回负的错误码。

   * ::

	void security_fs_context_free(struct fs_context *fc);

     调用以清理附加到 fc->security 的任何内容。注意其内容可能已被转移到超级块，并且指针在 get_tree 期间被清空。

   * ::

	int security_fs_context_parse_param(struct fs_context *fc,
					    struct fs_parameter *param);

     为每个挂载参数（包括来源）调用。参数与 ->parse_param() 方法相同。应返回 0 表示该参数应被传递给文件系统，返回 1 表示该参数应被丢弃，或返回一个错误以表示该参数应被拒绝。

     param 指向的值可能被修改（如果是字符串）或被窃取（前提是值指针被设为 NULL）。如果被窃取，必须返回 1 以防止它被传递给文件系统。

   * ::

	int security_fs_context_validate(struct fs_context *fc);

     在所有选项都被解析之后调用，以整体验证这一集合，并进行任何必要的分配，使得 security_sb_get_tree() 和 security_sb_reconfigure() 不太可能失败。应返回 0 或负的错误码。

     在重新配置的情况下，目标超级块可以通过 fc->root 访问。

   * ::

	int security_sb_get_tree(struct fs_context *fc);

     在挂载过程中调用，以验证指定的超级块是否被允许挂载，并将安全数据转移到那里。应返回 0 或负的错误码。

   * ::

	void security_sb_reconfigure(struct fs_context *fc);

     调用以将任何重新配置应用到 LSM 的上下文。它绝不能失败。错误检查和资源分配必须由参数解析和验证钩子提前完成。

   * ::

	int security_sb_mountpoint(struct fs_context *fc,
			           struct path *mountpoint,
				   unsigned int mnt_flags);

     在挂载过程中调用，以验证附加到上下文的根 dentry 是否允许被附加到指定的挂载点。成功应返回 0，失败返回负的错误码。


```
## VFS 文件系统上下文 API

有四个操作用于创建文件系统上下文，一个用于销毁上下文：

```

       struct fs_context *fs_context_for_mount(struct file_system_type *fs_type,
					       unsigned int sb_flags);

     分配一个文件系统上下文，用于设置一个新的挂载，无论是使用新的超级块还是共享已有的超级块。这会设置超级块标志，初始化安全，并调用 fs_type->init_fs_context() 来初始化文件系统私有数据。

     fs_type 指定管理该上下文的文件系统类型，sb_flags 预设其中存储的超级块标志。

   * ::

       struct fs_context *fs_context_for_reconfigure(
		struct dentry *dentry,
		unsigned int sb_flags,
		unsigned int sb_flags_mask);

     分配一个文件系统上下文，用于重新配置一个已有的超级块。dentry 提供对要配置的超级块的引用。sb_flags 和 sb_flags_mask 指明哪些超级块标志需要改变以及改成什么。

   * ::

       struct fs_context *fs_context_for_submount(
		struct file_system_type *fs_type,
		struct dentry *reference);

     分配一个文件系统上下文，用于为自动挂载点或其他派生的超级块创建一个新的挂载。fs_type 指定管理该上下文的文件系统类型，reference dentry 提供参数。命名空间也从 reference dentry 的超级块传播。

     注意，不要求 reference dentry 与 fs_type 属于相同的文件系统类型。

   * ::

        struct fs_context *vfs_dup_fs_context(struct fs_context *src_fc);

     复制一个文件系统上下文，复制其中记录的任何选项，并复制或额外引用其中持有的任何资源。这可用于文件系统必须在挂载内再进行挂载的情况，例如 NFS4 通过内部挂载目标服务器的根，然后做一次私有的路径遍历（pathwalk）到达目标目录。

     新上下文中的 purpose 从旧的继承而来。

   * ::

       void put_fs_context(struct fs_context *fc);

     销毁一个文件系统上下文，释放它持有的任何资源。这会调用 ->free() 操作。这预期由任何创建了文件系统上下文的人调用。

     .. Warning::

        文件系统上下文没有被引用计数，因此这会导致无条件的销毁。

```
在所有上述操作中，除了 put 操作之外，返回的是一个挂载上下文指针或一个负的错误码。

对于其余的操作，如果发生错误，将返回一个负的错误码。

```

        int vfs_parse_fs_param(struct fs_context *fc,
			       struct fs_parameter *param);

     向文件系统上下文提供单个挂载参数。这包括来源/设备的指定，它作为 "source" 参数指定（如果文件系统支持，可以多次指定）。

     param 指定参数键名和值。该参数会先被检查，看它是否对应一个标准的挂载标志（这种情况下用于设置一个 SB_xxx 标志并被消费）或一个安全选项（这种情况下由 LSM 消费），然后才被传递给文件系统。

     参数值是带类型的，可以是以下之一：

	====================		=============================
	fs_value_is_flag		Parameter not given a value
	fs_value_is_string		Value is a string
	fs_value_is_blob		Value is a binary blob
	fs_value_is_filename		Value is a filename* + dirfd
	fs_value_is_file		Value is an open file (file*)
	====================		=============================

     如果有一个值，该值存储在 struct 的一个联合体中的 param->{string,blob,name,file} 之一里。注意该函数可能会窃取并清空该指针，但随后要负责处置该对象。

   * ::

       int vfs_parse_fs_qstr(struct fs_context *fc, const char *key,
			       const struct qstr *value);

     vfs_parse_fs_param() 的一个包装，会复制传给它的 value 字符串。

   * ::

       int vfs_parse_fs_string(struct fs_context *fc, const char *key,
			       const char *value);

     vfs_parse_fs_param() 的一个包装，会复制传给它的 value 字符串。

   * ::

       int generic_parse_monolithic(struct fs_context *fc, void *data);

     解析 sys_mount() 的数据页，假设其形式为由逗号分隔的由 key[=val] 选项组成的文本列表。列表中的每一项都被传给 vfs_mount_option()。当 ->parse_monolithic() 方法为 NULL 时这是默认行为。

   * ::

       int vfs_get_tree(struct fs_context *fc);

     获取或创建可挂载的根与超级块，使用文件系统上下文中的参数来选择/配置超级块。这会调用 ->get_tree() 方法。

   * ::

       struct vfsmount *vfs_create_mount(struct fs_context *fc);

     根据给定的文件系统上下文中的参数创建一个挂载。注意这不会将挂载附加到任何东西上。


```
## 超级块创建辅助函数

VFS 提供了若干辅助函数供文件系统在创建或查找超级块时使用。

```

       struct super_block *
       sget_fc(struct fs_context *fc,
	       int (*test)(struct super_block *sb, struct fs_context *fc),
	       int (*set)(struct super_block *sb, struct fs_context *fc));

     这是核心例程。如果 test 为非 NULL，它会使用 test 函数在 fs_context 中搜索匹配条件的已有超级块。如果没找到匹配项，就创建一个新的超级块，并调用 set 函数来设置它。

     在调用 set 函数之前，fc->s_fs_info 将被转移到 sb->s_fs_info——并且如果 set 返回成功（即 0），fc->s_fs_info 将被清空。

```
以下辅助函数都包装了 sget_fc()：

	(1) vfs_get_single_super

	    系统中只能存在这样一个超级块。任何进一步获取新超级块的尝试都会得到这一个（并且任何参数差异都会被忽略）。

	(2) vfs_get_keyed_super

	    可能存在多个此类型的超级块，它们以各自的 s_fs_info 指针作为键（例如这可能指向一个命名空间）。

	(3) vfs_get_independent_super

	    可能存在多个独立的此类超级块。该函数从不匹配已有的一个，总是创建一个新的。


## 参数描述

参数使用 linux/fs_parser.h 中定义的结构来描述。
```

	struct fs_parameter_description {
		const struct fs_parameter_spec *specs;
		const struct fs_parameter_enum *enums;
	};

```
```

	enum {
		Opt_autocell,
		Opt_bar,
		Opt_dyn,
		Opt_foo,
		Opt_source,
	};

	static const struct fs_parameter_description afs_fs_parameters = {
		.specs		= afs_param_specs,
		.enums		= afs_param_enums,
	};

```
其成员如下：

```

       const struct fs_parameter_specification *specs;

     参数规格表，以一个空条目终止，其中的条目类型为::

	struct fs_parameter_spec {
		const char		*name;
		u8			opt;
		enum fs_parameter_type	type:8;
		unsigned short		flags;
	};

     'name' 字段是一个要与参数键精确匹配的字符串（不支持通配符、模式，也不区分大小写），'opt' 是 fs_parser() 函数在成功匹配的情况下返回的值。

     'type' 字段指明期望的值类型，必须是以下之一：

	=======================	=======================	=====================
	TYPE NAME		EXPECTED VALUE		RESULT IN
	=======================	=======================	=====================
	fs_param_is_flag	No value		n/a
	fs_param_is_bool	Boolean value		result->boolean
	fs_param_is_u32		32-bit unsigned int	result->uint_32
	fs_param_is_u32_octal	32-bit octal int	result->uint_32
	fs_param_is_u32_hex	32-bit hex int		result->uint_32
	fs_param_is_s32		32-bit signed int	result->int_32
	fs_param_is_u64		64-bit unsigned int	result->uint_64
	fs_param_is_enum	Enum value name 	result->uint_32
	fs_param_is_string	Arbitrary string	param->string
	fs_param_is_blockdev	Blockdev path		* Needs lookup
	fs_param_is_fd		File descriptor		result->int_32
	fs_param_is_uid		User ID (u32)           result->uid
	fs_param_is_gid		Group ID (u32)          result->gid
	=======================	=======================	=====================

     注意，如果值的类型是 fs_param_is_bool，fs_parse() 会尝试将任何字符串值与 "0"、"1"、"no"、"yes"、"false"、"true" 匹配。

     每个参数还可以用 'flags' 限定：

	=======================	================================================
	fs_param_v_optional	The value is optional
	fs_param_neg_with_no	result->negated set if key is prefixed with "no"
	fs_param_neg_with_empty	result->negated set if value is ""
	fs_param_deprecated	The parameter is deprecated.
	=======================	================================================

     它们由许多便利宏包装：

	=======================	===============================================
	MACRO			SPECIFIES
	=======================	===============================================
	fsparam_flag()		fs_param_is_flag
	fsparam_flag_no()	fs_param_is_flag, fs_param_neg_with_no
	fsparam_bool()		fs_param_is_bool
	fsparam_u32()		fs_param_is_u32
	fsparam_u32oct()	fs_param_is_u32_octal
	fsparam_s32()		fs_param_is_s32
	fsparam_u64()		fs_param_is_u64
	fsparam_enum()		fs_param_is_enum
	fsparam_string()	fs_param_is_string
	fsparam_bdev()		fs_param_is_blockdev
	fsparam_fd()		fs_param_is_fd
	fsparam_uid()		fs_param_is_uid
	fsparam_gid()		fs_param_is_gid
	=======================	===============================================

     以上全部取两个参数：name 字符串和选项编号——例如::

	static const struct fs_parameter_spec afs_param_specs[] = {
		fsparam_flag	("autocell",	Opt_autocell),
		fsparam_flag	("dyn",		Opt_dyn),
		fsparam_string	("source",	Opt_source),
		fsparam_flag_no	("foo",		Opt_foo),
		{}
	};

     还提供了一个额外的宏 __fsparam()，它取额外的一对参数来为不匹配上述任何宏的情况指定类型和标志。

 (2) ::

       const struct fs_parameter_enum *enums;

     枚举值名到整数的映射表，以一个空条目终止。其类型为::

	struct fs_parameter_enum {
		u8		opt;
		char		name[14];
		u8		value;
	};

     该数组是一个以 { 参数 ID, name } 为键的未排序元素列表，指示要映射到的 value，例如::

	static const struct fs_parameter_enum afs_param_enums[] = {
		{ Opt_bar,   "x",      1},
		{ Opt_bar,   "y",      23},
		{ Opt_bar,   "z",      42},
	};

     如果遇到 fs_param_is_enum 类型的参数，fs_parse() 会尝试在枚举表中查找该值，结果将存储在解析结果中。

```
解析器应由 file_system_type 结构中的 parser 指针指向，因为这将提供注册时的验证（如果 CONFIG_VALIDATE_FS_PARSER=y），并将允许通过 fsinfo() 系统调用从用户空间查询该描述。


## 参数辅助函数

提供了若干辅助函数来帮助文件系统或 LSM 处理它所获得的参数。

```

       int lookup_constant(const struct constant_table tbl[],
			   const char *name, int not_found);

     在“名字 -> 整数”映射表中按名字查找一个常量。该表是一个元素类型为如下的结构的数组::

	struct constant_table {
		const char	*name;
		int		value;
	};

     如果找到匹配，返回对应的值。如果没找到匹配，则改为返回 not_found 值。

   * ::

       bool fs_validate_description(const char *name,
                                    const struct fs_parameter_description *desc);

     这对参数描述执行一些验证检查。如果描述良好则返回 true，否则返回 false。如果验证失败，它会将错误记录到内核日志缓冲区。

   * ::

        int fs_parse(struct fs_context *fc,
		     const struct fs_parameter_description *desc,
		     struct fs_parameter *param,
		     struct fs_parse_result *result);

     这是参数的主解释器。它使用参数描述通过键名查找参数，并将其转换为一个选项编号（它返回该编号）。

     如果成功，并且如果参数类型指示结果是布尔、整数、枚举、uid 或 gid 类型，该值会被此函数转换，结果存储在 result->{boolean,int_32,uint_32,uint_64,uid,gid} 中。

     如果最初没有匹配，但键带有 "no" 前缀且没有值，则会尝试用去掉前缀的键去查找。如果这匹配到一个类型带有 fs_param_neg_with_no 标志的参数，则会形成匹配，并且 result->negated 会被设为 true。

     如果参数不匹配，将返回 -ENOPARAM；如果参数匹配但值有误，将返回 -EINVAL；否则会返回该参数的选项编号。

   * ::

       int fs_lookup_param(struct fs_context *fc,
			   struct fs_parameter *value,
			   bool want_bdev,
			   unsigned int flags,
			   struct path *_path);

     这接受一个携带字符串或文件名类型的参数，并尝试对其做路径查找。如果参数期望一个块设备，则会检查该 inode 是否确实代表一个块设备。

     成功时返回 0，并且 ``*_path`` 会被设置；否则返回一个负的错误码。

```
