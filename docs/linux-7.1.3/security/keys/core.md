## 内核密钥保留服务


该服务允许将加密密钥、认证令牌、跨
用户映射及类似内容缓存到内核中，
文件系统和其它内核服务使用

允许使用密钥环（keyring）；密钥环是一种特殊的密钥，可持有指向
其它密钥的链接。每个进程有三个标准的密钥环订阅，内核服务可
在其中搜索相关密钥

可以通过启用以下选项来配置密钥服务：

	"Security options"/"Enable access key retention support" (CONFIG_KEYS)

本文档包含以下章节：



## 密钥概述


在此语境下，密钥代表加密数据、认
令牌、密钥环等的单元。它们在内核中由 struct key 表示

每个密钥具有若干属性：

 - 序列号
 - 类型
 - 描述（用于在搜索中匹配密钥）
 - 访问控制信息
 - 过期时间
 - 载荷（payload）
 - 状态


  - 每个密钥都被分配一key_serial_t 类型的序列号，该序列号在
     该密钥的生命周期内是唯一的。所有序列号都是正的非零 32 
     整数

     用户空间程序可以使用密钥的序列号作为获取其访问权限的
     一种方式，但需经过权限检查

  - 每个密钥属于一种已定义的“type”。类型必须先在内核中
     由某个内核服务（例如文件系统）注册，之后该类型的密钥
     才能被添加或使用。用户空间程序不能直接定义新类型

     密钥类型在内核中struct key_type 表示。它定义了一
     可对该类型密钥执行的操作

     如果某个类型从系统中移除，该类型的所有密钥都将被
     置为无效

  - 每个密钥都有描述。它应当是一个可打印字符串。密
     类型提供一个操作，用于在密钥的描述
     一个条件字符串之间进行匹配

  - 每个密钥都有一个属主用ID、一个组 ID 和一个权限掩码。它
     用于控制进程能从用户空间对密钥执行什么操作，以及
     内核服务是否能够找到该密钥

  - 每个密钥都可以通过密钥类型
     实例化函数设置为在特定时间过期。密钥也可以是永生的（immortal）

  - 每个密钥都可以有载荷。这是表示实
     “key”的数据量。对于密钥环，这是它链接到的
     密钥列表；对于用户定义的密钥，它是一个任意的
     数据块（blob）

     载荷并非必需；事实上，载荷可以只
     存储struct key 自身中的一个值

     当密钥被实例化时，会调用密钥类型的实例化函数
     并传入一个数据块，随后以某种方式创建
     密钥的载荷

     类似地，当用户空间想要读回密钥内容时，若
     被允许，则会调用另一个密钥类型操作，将密
     附带的载荷转换回数据块

  - 每个密钥可以处于以下几种基本状态之一

      - 未实例化（Uninstantiated）。密钥存在，但没有附加任何数据
     	 从用户空间请求的密钥会处于此状态

      - 已实例化（Instantiated）。这是正常状态。密钥已完全成型，并
	 已附加数据

      - 负向（Negative）。这是一个相对短暂的状态。密钥充当一
	 标记，表示之前对用户空间的回调失败，并作
	 密钥查找的节流器。负向密钥可以被更新为正
	 状态

      - 已过期（Expired）。密钥可以设置生命周期。如果超出其生命周期
	 它们会进入此状态。已过期的密钥可以被更新
	 正常状态

      - 已撤销（Revoked）。密钥由用户空间操作进入此状态。它不能
	 找到或操作（除了将其取消链接）

      - 已废弃（Dead）。密钥的类型已被注销，因此该密钥现在已无用

处于后三种状态的密钥会受到垃圾回收。参
“垃圾回收”一节


## 密钥服务概述


除了密钥之外，密钥服务还提供许多特性：

  - 密钥服务定义了三种特殊的密钥类型

     (+) "keyring"

	 密钥环是包含其它密钥列表的特殊密钥。密钥环
	 列表可以使用各种系统调用来修改。密钥环在创建时
	 不应被赋予载荷

     (+) "user"

	 这种类型的密钥具有任意的
	 描述和数据块（blob）作为载荷。它们可以由用户空间创建、更新和读取
	 但不打算供内核服务使用

     (+) "logon"

	 与“user”密钥类似，“logon”密钥的载荷也是任意
	 数据块。它旨在作为存储密钥（secret）的地方，这些密
	 可供内核访问，但用户空间程序无法访问

	 描述可以是任意的，但必须带有一个非零长度的前缀字符串，
	 用于描述密钥的“subclass”。subclass
	 通过”与其余描述分隔。“logon”密钥可
	 由用户空间创建和更新，但其载荷只
	 从内核空间读取

  - 每个进程订阅三个密钥环：线程专属密钥环
     进程专属密钥环，以及会话专属密钥环

     在发生任何形式的 clone、fork、vfork execve 时，线程专属密钥环会从子进程
     丢弃。只有在需要时才会创建新的密钥环
     需要时才创建

     在子进程中，进程专属密钥环会
     clone、fork、vfork 时被一个空的替换，除非提供CLONE_THREAD
     此时它会被共享。execve 也会丢弃进程的进程密钥环，并创建
     一个新的

     会话专属密钥环在 clone、fork、vfork 
     execve 过程中保持不变，即使后者执行了 set-UID set-GID 二进制文件。但
     进程可以用一个新的密钥环替换其当前会话密钥环
     方法是使PR_JOIN_SESSION_KEYRING。允许请求一个匿名的
     新密钥环，或尝试创建/加入一个指定名称的密钥环

     当线程的真实 UID GID 发生
     变化时，线程密钥环的属主也会改变

  - 系统中驻留的每个用户 ID 持有两种特殊密钥环：用户
     专属密钥环和一个默认用户会话密钥环。默认会
     密钥环初始化时带有指向用户专属密钥环的链接

     当进程改变其真实 UID 时，如果它以前没有会话密钥，
     它将被订阅到UID 的默认会话密钥

     如果进程在其没有会话密钥的情况下尝试访问会话密钥
     它将被订阅到其当UID 的默认密钥

  - 每个用户有两个配额，用于跟踪其拥有的密钥。一
     限制密钥和密钥环的总数，另一个限制可消耗的
     描述和载荷空间的总量

     用户可以通过 procfs 文件查看
     关于此及其它统计信息。root 用户也可以通过 sysctl 文件
     修改配额限制（参见“New procfs files”一节）

     进程专属和线程专属密钥环不计
     用户的配额

     如果某个以某种方式修改密钥或密钥环的系统调用会导
     用户超出配额，该操作会被拒绝并返回错EDQUOT

  - 存在一个系统调用接口，用户空间程序可通过它创建和
     操作密钥和密钥环

  - 存在一个内核接口，服务可通过它注册类型并搜索
     密钥

  - 存在一种机制，使得从内核发起的搜索可以回调
     用户空间，以请求一个在进程密钥环中找不到的密钥

  - 提供了一个可选的文件系统，可通过
     查看和操作密钥数据库


## 密钥访问权限


密钥具有一个属主用ID、一个组访问 ID，以及一个权限掩码。该掩码
为持有者、用户、组和其它访问各提供最8 位。只
每组 8 位中6 位被定义。所授予的权限如下：

  - 查看（View

     这允许查看密钥或密钥环的属性——包括密
     类型和描述

  - 读取（Read

     这允许查看密钥的载荷，或密钥环的链接
     密钥列表

  - 写入（Write

     这允许实例化或更新密钥的载荷，或允许向密钥环
     添加或移除链接

  - 搜索（Search

     这允许搜索密钥环并找到密钥。搜索只
     递归进入设置了搜索权限的嵌套密钥环

  - 链接（Link

     这允许将密钥或密钥环链接到。要从密钥环创建到某个密钥的链接
     进程必须对该密钥环具Write 权限，并
     该密钥具Link 权限

  - 设置属性（Set Attribute

     这允许更改密钥的 UID、GID 和权限掩码

要更改属主、组 ID 或权限掩码，成为密钥
属主或具sysadmin 能力（capability）即可


## SELinux 支持


已将安全类“key”添加到 SELinux，以便可以对
在各种上下文中创建的密钥施加强制访问控制。该支持
尚属初步阶段，在不久的将来很可能会发生显著变化
目前，上述所有基本权限在 SELinux 中也都已提供
SELinux 只是在所有基本权限检查完成后被调用
     执行

/proc/self/attr/keycreate 文件的值会影响
新创建密钥的标记（labeling）。如果该文件的内容对应于某个 SELinux
安全上下文，则该密钥会被分配该上下文。否则，
密钥会被分配调用密钥
创建请求的任务的当前上下文。任务必须被授予显式权限，才能将
特定上下文分配给新创建的密钥，这需要使用密
     安全类中的“create”权限

与用户关联的默认密钥环将被标记为用户的默
上下文，当且仅当登录程序已被改造为
登录过程中正确初始化 keycreate。否则，它们将被
标记为登录程序自身的上下文

但请注意，与 root 用户关联的默认密钥环被标记为
默认的内核上下文，因为它们在
启动过程的早期创建，root 有机会登录之前

与新线程关联的密钥环各自被标记为
其关联线程的上下文，会话和进程密钥环
     类似处理


## 新增ProcFS 文件


已向 procfs 添加两个文件，管理员可通过它们了解
密钥服务的状态：

  - /proc/keys

     它列出当前读取该文件的任务可查看的密钥，
     提供关于其类型、描述和权限的信息
     无法通过此方式查看密钥的载荷，不过可能会给出
     一些相关信息

     列表中仅包含那些向读取进程授予了 View 权限的密钥，
     无论该进程是否持有它们。注意，LSM
     安全检查仍会执行，并可能进一步过滤掉
     当前进程无权查看的密钥

```

	SERIAL   FLAGS  USAGE EXPY PERM     UID   GID   TYPE      DESCRIPTION: SUMMARY
	00000001 I-----    39 perm 1f3f0000     0     0 keyring   _uid_ses.0: 1/4
	00000002 I-----     2 perm 1f3f0000     0     0 keyring   _uid.0: empty
	00000007 I-----     1 perm 1f3f0000     0     0 keyring   _pid.1: empty
	0000018d I-----     1 perm 1f3f0000     0     0 keyring   _pid.412: empty
	000004d2 I--Q--     1 perm 1f3f0000    32    -1 keyring   _uid.32: 1/4
	000004d3 I--Q--     3 perm 1f3f0000    32    -1 keyring   _uid_ses.32: empty
	00000892 I--QU-     1 perm 1f000000     0     0 user      metal:copper: 0
	00000893 I--Q-N     1  35s 1f3f0000     0     0 user      metal:silver: 0
	00000894 I--Q--     1  10h 003f0000     0     0 user      metal:gold: 0

     The flags are::

	I	Instantiated
	R	Revoked
	D	Dead
	Q	Contributes to user's quota
	U	Under construction by callback to userspace
	N	Negative key


  *  /proc/key-users

     This file lists the tracking data for each user that has at least one key
     on the system.  Such data includes quota information and statistics::

	[root@andromeda root]# cat /proc/key-users
	0:     46 45/45 1/100 13/10000
	29:     2 2/2 2/100 40/10000
	32:     2 2/2 2/100 40/10000
	38:     2 2/2 2/100 40/10000

     The format of each line is::

	<UID>:			User ID to which this applies
	<usage>			Structure refcount
	<inst>/<keys>		Total number of keys and number instantiated
	<keys>/<max>		Key count quota
	<bytes>/<max>		Key size quota


```
还新增了四个 sysctl 文件，用于控
密钥的配额限制：

  - /proc/sys/kernel/keys/root_maxkeys
     /proc/sys/kernel/keys/root_maxbytes

     这些文件保存root 可拥有的密钥最大数量，以及
     root 可在这些密钥中存储的数据字节总数
     的最大值

  - /proc/sys/kernel/keys/maxkeys
     /proc/sys/kernel/keys/maxbytes

     这些文件保存了每个非 root 用户可拥有的密钥最大数量，
     以及这些用户各自可在其密钥中存储的数
     字节总数最大值

root 可以通过把每个新限制以十进制数字字符串的形式写入
相应文件来更改它们


## 用户空间系统调用接口


用户空间可通过三个新的系统调用直接操作密钥：add_key
request_key keyctl。后者提供了一系列用于
操作密钥的函数

直接引用密钥时，用户空间程序应使用密钥的
序列号（一个正32 位整数）。不过，也存在一些特
值，用于引用与特定进程相关的特殊密钥和密钥环
```

	CONSTANT			VALUE	KEY REFERENCED
	==============================	======	===========================
	KEY_SPEC_THREAD_KEYRING		-1	thread-specific keyring
	KEY_SPEC_PROCESS_KEYRING	-2	process-specific keyring
	KEY_SPEC_SESSION_KEYRING	-3	session-specific keyring
	KEY_SPEC_USER_KEYRING		-4	UID-specific keyring
	KEY_SPEC_USER_SESSION_KEYRING	-5	UID-session keyring
	KEY_SPEC_GROUP_KEYRING		-6	GID-specific keyring
	KEY_SPEC_REQKEY_AUTH_KEY	-7	assumed request_key()
						  authorisation key


```
主要的系统调用如下：

  - 创建给定类型、描述和载荷的新密钥，并将其添加
```

	key_serial_t add_key(const char *type, const char *desc,
			     const void *payload, size_t plen,
			     key_serial_t keyring);

     If a key of the same type and description as that proposed already exists
     in the keyring, this will try to update it with the given payload, or it
     will return error EEXIST if that function is not supported by the key
     type. The process must also have permission to write to the key to be able
     to update it. The new key will have all user permissions granted and no
     group or third party permissions.

     Otherwise, this will attempt to create a new key of the specified type and
     description, and to instantiate it with the supplied payload and attach it
     to the keyring. In this case, an error will be generated if the process
     does not have permission to write to the keyring.

     If the key type supports it, if the description is NULL or an empty
     string, the key type will try and generate a description from the content
     of the payload.

     The payload is optional, and the pointer can be NULL if not required by
     the type. The payload is plen in size, and plen can be zero for an empty
     payload.

     A new keyring can be generated by setting type "keyring", the keyring name
     as the description (or NULL) and setting the payload to NULL.

     User defined keys can be created by specifying type "user". It is
     recommended that a user defined key's description by prefixed with a type
     ID and a colon, such as "krb5tgt:" for a Kerberos 5 ticket granting
     ticket.

     Any other type must have been registered with the kernel in advance by a
     kernel service such as a filesystem.

     The ID of the new or updated key is returned if successful.


  *  Search the process's keyrings for a key, potentially calling out to
     userspace to create it::

	key_serial_t request_key(const char *type, const char *description,
				 const char *callout_info,
				 key_serial_t dest_keyring);

     This function searches all the process's keyrings in the order thread,
     process, session for a matching key. This works very much like
     KEYCTL_SEARCH, including the optional attachment of the discovered key to
     a keyring.

     If a key cannot be found, and if callout_info is not NULL, then
     /sbin/request-key will be invoked in an attempt to obtain a key. The
     callout_info string will be passed as an argument to the program.

     To link a key into the destination keyring the key must grant link
     permission on the key to the caller and the keyring must grant write
     permission.

     See also Documentation/security/keys/request-key.rst.


```
keyctl 系统调用的函数如下：

```

	key_serial_t keyctl(KEYCTL_GET_KEYRING_ID, key_serial_t id,
			    int create);

     The special key specified by "id" is looked up (with the key being created
     if necessary) and the ID of the key or keyring thus found is returned if
     it exists.

     If the key does not yet exist, the key will be created if "create" is
     non-zero; and the error ENOKEY will be returned if "create" is zero.


  *  Replace the session keyring this process subscribes to with a new one::

	key_serial_t keyctl(KEYCTL_JOIN_SESSION_KEYRING, const char *name);

     If name is NULL, an anonymous keyring is created attached to the process
     as its session keyring, displacing the old session keyring.

     If name is not NULL, if a keyring of that name exists, the process
     attempts to attach it as the session keyring, returning an error if that
     is not permitted; otherwise a new keyring of that name is created and
     attached as the session keyring.

     To attach to a named keyring, the keyring must have search permission for
     the process's ownership.

     The ID of the new session keyring is returned if successful.


  *  Update the specified key::

	long keyctl(KEYCTL_UPDATE, key_serial_t key, const void *payload,
		    size_t plen);

     This will try to update the specified key with the given payload, or it
     will return error EOPNOTSUPP if that function is not supported by the key
     type. The process must also have permission to write to the key to be able
     to update it.

     The payload is of length plen, and may be absent or empty as for
     add_key().


  *  Revoke a key::

	long keyctl(KEYCTL_REVOKE, key_serial_t key);

     This makes a key unavailable for further operations. Further attempts to
     use the key will be met with error EKEYREVOKED, and the key will no longer
     be findable.


  *  Change the ownership of a key::

	long keyctl(KEYCTL_CHOWN, key_serial_t key, uid_t uid, gid_t gid);

     This function permits a key's owner and group ID to be changed. Either one
     of uid or gid can be set to -1 to suppress that change.

     Only the superuser can change a key's owner to something other than the
     key's current owner. Similarly, only the superuser can change a key's
     group ID to something other than the calling process's group ID or one of
     its group list members.


  *  Change the permissions mask on a key::

	long keyctl(KEYCTL_SETPERM, key_serial_t key, key_perm_t perm);

     This function permits the owner of a key or the superuser to change the
     permissions mask on a key.

     Only bits the available bits are permitted; if any other bits are set,
     error EINVAL will be returned.


  *  Describe a key::

	long keyctl(KEYCTL_DESCRIBE, key_serial_t key, char *buffer,
		    size_t buflen);

     This function returns a summary of the key's attributes (but not its
     payload data) as a string in the buffer provided.

     Unless there's an error, it always returns the amount of data it could
     produce, even if that's too big for the buffer, but it won't copy more
     than requested to userspace. If the buffer pointer is NULL then no copy
     will take place.

     A process must have view permission on the key for this function to be
     successful.

     If successful, a string is placed in the buffer in the following format::

	<type>;<uid>;<gid>;<perm>;<description>

     Where type and description are strings, uid and gid are decimal, and perm
     is hexadecimal. A NUL character is included at the end of the string if
     the buffer is sufficiently big.

     This can be parsed with::

	sscanf(buffer, "%[^;];%d;%d;%o;%s", type, &uid, &gid, &mode, desc);


  *  Clear out a keyring::

	long keyctl(KEYCTL_CLEAR, key_serial_t keyring);

     This function clears the list of keys attached to a keyring. The calling
     process must have write permission on the keyring, and it must be a
     keyring (or else error ENOTDIR will result).

     This function can also be used to clear special kernel keyrings if they
     are appropriately marked if the user has CAP_SYS_ADMIN capability.  The
     DNS resolver cache keyring is an example of this.


  *  Link a key into a keyring::

	long keyctl(KEYCTL_LINK, key_serial_t keyring, key_serial_t key);

     This function creates a link from the keyring to the key. The process must
     have write permission on the keyring and must have link permission on the
     key.

     Should the keyring not be a keyring, error ENOTDIR will result; and if the
     keyring is full, error ENFILE will result.

     The link procedure checks the nesting of the keyrings, returning ELOOP if
     it appears too deep or EDEADLK if the link would introduce a cycle.

     Any links within the keyring to keys that match the new key in terms of
     type and description will be discarded from the keyring as the new one is
     added.


  *  Move a key from one keyring to another::

	long keyctl(KEYCTL_MOVE,
		    key_serial_t id,
		    key_serial_t from_ring_id,
		    key_serial_t to_ring_id,
		    unsigned int flags);

     Move the key specified by "id" from the keyring specified by
     "from_ring_id" to the keyring specified by "to_ring_id".  If the two
     keyrings are the same, nothing is done.

     "flags" can have KEYCTL_MOVE_EXCL set in it to cause the operation to fail
     with EEXIST if a matching key exists in the destination keyring, otherwise
     such a key will be replaced.

     A process must have link permission on the key for this function to be
     successful and write permission on both keyrings.  Any errors that can
     occur from KEYCTL_LINK also apply on the destination keyring here.


  *  Unlink a key or keyring from another keyring::

	long keyctl(KEYCTL_UNLINK, key_serial_t keyring, key_serial_t key);

     This function looks through the keyring for the first link to the
     specified key, and removes it if found. Subsequent links to that key are
     ignored. The process must have write permission on the keyring.

     If the keyring is not a keyring, error ENOTDIR will result; and if the key
     is not present, error ENOENT will be the result.


  *  Search a keyring tree for a key::

	key_serial_t keyctl(KEYCTL_SEARCH, key_serial_t keyring,
			    const char *type, const char *description,
			    key_serial_t dest_keyring);

     This searches the keyring tree headed by the specified keyring until a key
     is found that matches the type and description criteria. Each keyring is
     checked for keys before recursion into its children occurs.

     The process must have search permission on the top level keyring, or else
     error EACCES will result. Only keyrings that the process has search
     permission on will be recursed into, and only keys and keyrings for which
     a process has search permission can be matched. If the specified keyring
     is not a keyring, ENOTDIR will result.

     If the search succeeds, the function will attempt to link the found key
     into the destination keyring if one is supplied (non-zero ID). All the
     constraints applicable to KEYCTL_LINK apply in this case too.

     Error ENOKEY, EKEYREVOKED or EKEYEXPIRED will be returned if the search
     fails. On success, the resulting key ID will be returned.


  *  Read the payload data from a key::

	long keyctl(KEYCTL_READ, key_serial_t keyring, char *buffer,
		    size_t buflen);

     This function attempts to read the payload data from the specified key
     into the buffer. The process must have read permission on the key to
     succeed.

     The returned data will be processed for presentation by the key type. For
     instance, a keyring will return an array of key_serial_t entries
     representing the IDs of all the keys to which it is subscribed. The user
     defined key type will return its data as is. If a key type does not
     implement this function, error EOPNOTSUPP will result.

     If the specified buffer is too small, then the size of the buffer required
     will be returned.  Note that in this case, the contents of the buffer may
     have been overwritten in some undefined way.

     Otherwise, on success, the function will return the amount of data copied
     into the buffer.

  *  Instantiate a partially constructed key::

	long keyctl(KEYCTL_INSTANTIATE, key_serial_t key,
		    const void *payload, size_t plen,
		    key_serial_t keyring);
	long keyctl(KEYCTL_INSTANTIATE_IOV, key_serial_t key,
		    const struct iovec *payload_iov, unsigned ioc,
		    key_serial_t keyring);

     If the kernel calls back to userspace to complete the instantiation of a
     key, userspace should use this call to supply data for the key before the
     invoked process returns, or else the key will be marked negative
     automatically.

     The process must have write access on the key to be able to instantiate
     it, and the key must be uninstantiated.

     If a keyring is specified (non-zero), the key will also be linked into
     that keyring, however all the constraints applying in KEYCTL_LINK apply in
     this case too.

     The payload and plen arguments describe the payload data as for add_key().

     The payload_iov and ioc arguments describe the payload data in an iovec
     array instead of a single buffer.


  *  Negatively instantiate a partially constructed key::

	long keyctl(KEYCTL_NEGATE, key_serial_t key,
		    unsigned timeout, key_serial_t keyring);
	long keyctl(KEYCTL_REJECT, key_serial_t key,
		    unsigned timeout, unsigned error, key_serial_t keyring);

     If the kernel calls back to userspace to complete the instantiation of a
     key, userspace should use this call mark the key as negative before the
     invoked process returns if it is unable to fulfill the request.

     The process must have write access on the key to be able to instantiate
     it, and the key must be uninstantiated.

     If a keyring is specified (non-zero), the key will also be linked into
     that keyring, however all the constraints applying in KEYCTL_LINK apply in
     this case too.

     If the key is rejected, future searches for it will return the specified
     error code until the rejected key expires.  Negating the key is the same
     as rejecting the key with ENOKEY as the error code.


  *  Set the default request-key destination keyring::

	long keyctl(KEYCTL_SET_REQKEY_KEYRING, int reqkey_defl);

     This sets the default keyring to which implicitly requested keys will be
     attached for this thread. reqkey_defl should be one of these constants::

	CONSTANT				VALUE	NEW DEFAULT KEYRING
	======================================	======	=======================
	KEY_REQKEY_DEFL_NO_CHANGE		-1	No change
	KEY_REQKEY_DEFL_DEFAULT			0	Default[1]
	KEY_REQKEY_DEFL_THREAD_KEYRING		1	Thread keyring
	KEY_REQKEY_DEFL_PROCESS_KEYRING		2	Process keyring
	KEY_REQKEY_DEFL_SESSION_KEYRING		3	Session keyring
	KEY_REQKEY_DEFL_USER_KEYRING		4	User keyring
	KEY_REQKEY_DEFL_USER_SESSION_KEYRING	5	User session keyring
	KEY_REQKEY_DEFL_GROUP_KEYRING		6	Group keyring

     The old default will be returned if successful and error EINVAL will be
     returned if reqkey_defl is not one of the above values.

     The default keyring can be overridden by the keyring indicated to the
     request_key() system call.

     Note that this setting is inherited across fork/exec.

     [1] The default is: the thread keyring if there is one, otherwise
     the process keyring if there is one, otherwise the session keyring if
     there is one, otherwise the user default session keyring.


  *  Set the timeout on a key::

	long keyctl(KEYCTL_SET_TIMEOUT, key_serial_t key, unsigned timeout);

     This sets or clears the timeout on a key. The timeout can be 0 to clear
     the timeout or a number of seconds to set the expiry time that far into
     the future.

     The process must have attribute modification access on a key to set its
     timeout. Timeouts may not be set with this function on negative, revoked
     or expired keys.


  *  Assume the authority granted to instantiate a key::

	long keyctl(KEYCTL_ASSUME_AUTHORITY, key_serial_t key);

     This assumes or divests the authority required to instantiate the
     specified key. Authority can only be assumed if the thread has the
     authorisation key associated with the specified key in its keyrings
     somewhere.

     Once authority is assumed, searches for keys will also search the
     requester's keyrings using the requester's security label, UID, GID and
     groups.

     If the requested authority is unavailable, error EPERM will be returned,
     likewise if the authority has been revoked because the target key is
     already instantiated.

     If the specified key is 0, then any assumed authority will be divested.

     The assumed authoritative key is inherited across fork and exec.


  *  Get the LSM security context attached to a key::

	long keyctl(KEYCTL_GET_SECURITY, key_serial_t key, char *buffer,
		    size_t buflen)

     This function returns a string that represents the LSM security context
     attached to a key in the buffer provided.

     Unless there's an error, it always returns the amount of data it could
     produce, even if that's too big for the buffer, but it won't copy more
     than requested to userspace. If the buffer pointer is NULL then no copy
     will take place.

     A NUL character is included at the end of the string if the buffer is
     sufficiently big.  This is included in the returned count.  If no LSM is
     in force then an empty string will be returned.

     A process must have view permission on the key for this function to be
     successful.


  *  Install the calling process's session keyring on its parent::

	long keyctl(KEYCTL_SESSION_TO_PARENT);

     This functions attempts to install the calling process's session keyring
     on to the calling process's parent, replacing the parent's current session
     keyring.

     The calling process must have the same ownership as its parent, the
     keyring must have the same ownership as the calling process, the calling
     process must have LINK permission on the keyring and the active LSM module
     mustn't deny permission, otherwise error EPERM will be returned.

     Error ENOMEM will be returned if there was insufficient memory to complete
     the operation, otherwise 0 will be returned to indicate success.

     The keyring will be replaced next time the parent process leaves the
     kernel and resumes executing userspace.


  *  Invalidate a key::

	long keyctl(KEYCTL_INVALIDATE, key_serial_t key);

     This function marks a key as being invalidated and then wakes up the
     garbage collector.  The garbage collector immediately removes invalidated
     keys from all keyrings and deletes the key when its reference count
     reaches zero.

     Keys that are marked invalidated become invisible to normal key operations
     immediately, though they are still visible in /proc/keys until deleted
     (they're marked with an 'i' flag).

     A process must have search permission on the key for this function to be
     successful.

  *  Compute a Diffie-Hellman shared secret or public key::

	long keyctl(KEYCTL_DH_COMPUTE, struct keyctl_dh_params *params,
		    char *buffer, size_t buflen, struct keyctl_kdf_params *kdf);

     The params struct contains serial numbers for three keys::

	 - The prime, p, known to both parties
	 - The local private key
	 - The base integer, which is either a shared generator or the
	   remote public key

     The value computed is::

	result = base ^ private (mod prime)

     If the base is the shared generator, the result is the local
     public key.  If the base is the remote public key, the result is
     the shared secret.

     If the parameter kdf is NULL, the following applies:

	 - The buffer length must be at least the length of the prime, or zero.

	 - If the buffer length is nonzero, the length of the result is
	   returned when it is successfully calculated and copied in to the
	   buffer. When the buffer length is zero, the minimum required
	   buffer length is returned.

     The kdf parameter allows the caller to apply a key derivation function
     (KDF) on the Diffie-Hellman computation where only the result
     of the KDF is returned to the caller. The KDF is characterized with
     struct keyctl_kdf_params as follows:

	 - ``char *hashname`` specifies the NUL terminated string identifying
	   the hash used from the kernel crypto API and applied for the KDF
	   operation. The KDF implementation complies with SP800-56A as well
	   as with SP800-108 (the counter KDF).

	 - ``char *otherinfo`` specifies the OtherInfo data as documented in
	   SP800-56A section 5.8.1.2. The length of the buffer is given with
	   otherinfolen. The format of OtherInfo is defined by the caller.
	   The otherinfo pointer may be NULL if no OtherInfo shall be used.

     This function will return error EOPNOTSUPP if the key type is not
     supported, error ENOKEY if the key could not be found, or error
     EACCES if the key is not readable by the caller. In addition, the
     function will return EMSGSIZE when the parameter kdf is non-NULL
     and either the buffer length or the OtherInfo length exceeds the
     allowed length.


  *  Restrict keyring linkage::

	long keyctl(KEYCTL_RESTRICT_KEYRING, key_serial_t keyring,
		    const char *type, const char *restriction);

     An existing keyring can restrict linkage of additional keys by evaluating
     the contents of the key according to a restriction scheme.

     "keyring" is the key ID for an existing keyring to apply a restriction
     to. It may be empty or may already have keys linked. Existing linked keys
     will remain in the keyring even if the new restriction would reject them.

     "type" is a registered key type.

     "restriction" is a string describing how key linkage is to be restricted.
     The format varies depending on the key type, and the string is passed to
     the lookup_restriction() function for the requested type.  It may specify
     a method and relevant data for the restriction such as signature
     verification or constraints on key payload. If the requested key type is
     later unregistered, no keys may be added to the keyring after the key type
     is removed.

     To apply a keyring restriction the process must have Set Attribute
     permission and the keyring must not be previously restricted.

     One application of restricted keyrings is to verify X.509 certificate
     chains or individual certificate signatures using the asymmetric key type.
     See Documentation/crypto/asymmetric-keys.rst for specific restrictions
     applicable to the asymmetric key type.


  *  Query an asymmetric key::

	long keyctl(KEYCTL_PKEY_QUERY,
		    key_serial_t key_id, unsigned long reserved,
		    const char *params,
		    struct keyctl_pkey_query *info);

     Get information about an asymmetric key.  Specific algorithms and
     encodings may be queried by using the ``params`` argument.  This is a
     string containing a space- or tab-separated string of key-value pairs.
     Currently supported keys include ``enc`` and ``hash``.  The information
     is returned in the keyctl_pkey_query struct::

	__u32	supported_ops;
	__u32	key_size;
	__u16	max_data_size;
	__u16	max_sig_size;
	__u16	max_enc_size;
	__u16	max_dec_size;
	__u32	__spare[10];

     ``supported_ops`` contains a bit mask of flags indicating which ops are
     supported.  This is constructed from a bitwise-OR of::

	KEYCTL_SUPPORTS_{ENCRYPT,DECRYPT,SIGN,VERIFY}

     ``key_size`` indicated the size of the key in bits.

     ``max_*_size`` indicate the maximum sizes in bytes of a blob of data to be
     signed, a signature blob, a blob to be encrypted and a blob to be
     decrypted.

     ``__spare[]`` must be set to 0.  This is intended for future use to hand
     over one or more passphrases needed unlock a key.

     If successful, 0 is returned.  If the key is not an asymmetric key,
     EOPNOTSUPP is returned.


  *  Encrypt, decrypt, sign or verify a blob using an asymmetric key::

	long keyctl(KEYCTL_PKEY_ENCRYPT,
		    const struct keyctl_pkey_params *params,
		    const char *info,
		    const void *in,
		    void *out);

	long keyctl(KEYCTL_PKEY_DECRYPT,
		    const struct keyctl_pkey_params *params,
		    const char *info,
		    const void *in,
		    void *out);

	long keyctl(KEYCTL_PKEY_SIGN,
		    const struct keyctl_pkey_params *params,
		    const char *info,
		    const void *in,
		    void *out);

	long keyctl(KEYCTL_PKEY_VERIFY,
		    const struct keyctl_pkey_params *params,
		    const char *info,
		    const void *in,
		    const void *in2);

     Use an asymmetric key to perform a public-key cryptographic operation a
     blob of data.  For encryption and verification, the asymmetric key may
     only need the public parts to be available, but for decryption and signing
     the private parts are required also.

     The parameter block pointed to by params contains a number of integer
     values::

	__s32		key_id;
	__u32		in_len;
	__u32		out_len;
	__u32		in2_len;

     ``key_id`` is the ID of the asymmetric key to be used.  ``in_len`` and
     ``in2_len`` indicate the amount of data in the in and in2 buffers and
     ``out_len`` indicates the size of the out buffer as appropriate for the
     above operations.

     For a given operation, the in and out buffers are used as follows::

	Operation ID		in,in_len	out,out_len	in2,in2_len
	=======================	===============	===============	===============
	KEYCTL_PKEY_ENCRYPT	Raw data	Encrypted data	-
	KEYCTL_PKEY_DECRYPT	Encrypted data	Raw data	-
	KEYCTL_PKEY_SIGN	Raw data	Signature	-
	KEYCTL_PKEY_VERIFY	Raw data	-		Signature

     ``info`` is a string of key=value pairs that supply supplementary
     information.  These include:

	``enc=<encoding>`` The encoding of the encrypted/signature blob.  This
			can be "pkcs1" for RSASSA-PKCS1-v1.5 or
			RSAES-PKCS1-v1.5; "pss" for "RSASSA-PSS"; "oaep" for
			"RSAES-OAEP".  If omitted or is "raw", the raw output
			of the encryption function is specified.

	``hash=<algo>``	If the data buffer contains the output of a hash
			function and the encoding includes some indication of
			which hash function was used, the hash function can be
			specified with this, eg. "hash=sha256".

     The ``__spare[]`` space in the parameter block must be set to 0.  This is
     intended, amongst other things, to allow the passing of passphrases
     required to unlock a key.

     If successful, encrypt, decrypt and sign all return the amount of data
     written into the output buffer.  Verification returns 0 on success.


  *  Watch a key or keyring for changes::

	long keyctl(KEYCTL_WATCH_KEY, key_serial_t key, int queue_fd,
		    const struct watch_notification_filter *filter);

     This will set or remove a watch for changes on the specified key or
     keyring.

     "key" is the ID of the key to be watched.

     "queue_fd" is a file descriptor referring to an open pipe which
     manages the buffer into which notifications will be delivered.

     "filter" is either NULL to remove a watch or a filter specification to
     indicate what events are required from the key.

     See Documentation/core-api/watch_queue.rst for more information.

     Note that only one watch may be emplaced for any particular { key,
     queue_fd } combination.

     Notification records look like::

	struct key_notification {
		struct watch_notification watch;
		__u32	key_id;
		__u32	aux;
	};

     In this, watch::type will be "WATCH_TYPE_KEY_NOTIFY" and subtype will be
     one of::

	NOTIFY_KEY_INSTANTIATED
	NOTIFY_KEY_UPDATED
	NOTIFY_KEY_LINKED
	NOTIFY_KEY_UNLINKED
	NOTIFY_KEY_CLEARED
	NOTIFY_KEY_REVOKED
	NOTIFY_KEY_INVALIDATED
	NOTIFY_KEY_SETATTR

     Where these indicate a key being instantiated/rejected, updated, a link
     being made in a keyring, a link being removed from a keyring, a keyring
     being cleared, a key being revoked, a key being invalidated or a key
     having one of its attributes changed (user, group, perm, timeout,
     restriction).

     If a watched key is deleted, a basic watch_notification will be issued
     with "type" set to WATCH_TYPE_META and "subtype" set to
     watch_meta_removal_notification.  The watchpoint ID will be set in the
     "info" field.

     This needs to be configured by enabling:

	"Provide key/keyring change notifications" (KEY_NOTIFICATIONS)


```
## 内核服务


用于密钥管理的内核服务相当容易处理。它们可
分为两个领域：密钥和密钥类型

处理密钥相当直接。首先，内核服务
注册其类型，然后搜索该类型的密钥。它应保
该密钥，只要它还需要，然后应当释放它。对
文件系统或设备文件，搜索可能open
调用期间执行，并close 时释放密钥。如何处理由
两个不同用户打开同一文件而产生的密钥冲突，留给文件系统作
去解决

```

	<linux/key.h>

```
特定的密钥类型应include/keys/ 下有一个头文件，该文件
```

	<keys/user-type.h>

```
注意，可能存在两种不同类型的指向密钥的指针：
     遇到

  - struct key *

     这简单地指向密钥结构本身。密钥结构将
     至少按四字节对齐

  - key_ref_t

     这等价于一struct key *，但最低有效位被置位，
     如果调用者“持有（possesses）”该密钥。所谓“持有”，是指
     调用进程从其某个密钥环拥有到该密钥的
```

	key_ref_t make_key_ref(const struct key *key, bool possession);

	struct key *key_ref_to_ptr(const key_ref_t key_ref);

	bool is_key_possessed(const key_ref_t key_ref);

     The first function constructs a key reference from a key pointer and
     possession information (which must be true or false).

     The second function retrieves the key pointer from a reference and the
     third retrieves the possession flag.

```
在访问密钥的载荷内容时，必须采取某些预防措施
以防止访问与修改之间的竞争。参见“访
     载荷内容的注意事项”一节以获取更多信息

```

	struct key *request_key(const struct key_type *type,
				const char *description,
				const char *callout_info);

    This is used to request a key or keyring with a description that matches
    the description specified according to the key type's match_preparse()
    method. This permits approximate matching to occur. If callout_string is
    not NULL, then /sbin/request-key will be invoked in an attempt to obtain
    the key from userspace. In that case, callout_string will be passed as an
    argument to the program.

    Should the function fail error ENOKEY, EKEYEXPIRED or EKEYREVOKED will be
    returned.

    If successful, the key will have been attached to the default keyring for
    implicitly obtained request-key keys, as set by KEYCTL_SET_REQKEY_KEYRING.

    See also Documentation/security/keys/request-key.rst.


 *  To search for a key in a specific domain, call::

	struct key *request_key_tag(const struct key_type *type,
				    const char *description,
				    struct key_tag *domain_tag,
				    const char *callout_info);

    This is identical to request_key(), except that a domain tag may be
    specifies that causes search algorithm to only match keys matching that
    tag.  The domain_tag may be NULL, specifying a global domain that is
    separate from any nominated domain.


 *  To search for a key, passing auxiliary data to the upcaller, call::

	struct key *request_key_with_auxdata(const struct key_type *type,
					     const char *description,
					     struct key_tag *domain_tag,
					     const void *callout_info,
					     size_t callout_len,
					     void *aux);

    This is identical to request_key_tag(), except that the auxiliary data is
    passed to the key_type->request_key() op if it exists, and the
    callout_info is a blob of length callout_len, if given (the length may be
    0).


 *  To search for a key under RCU conditions, call::

	struct key *request_key_rcu(const struct key_type *type,
				    const char *description,
				    struct key_tag *domain_tag);

    which is similar to request_key_tag() except that it does not check for
    keys that are under construction and it will not call out to userspace to
    construct a key if it can't find a match.


 *  When it is no longer required, the key should be released using::

	void key_put(struct key *key);

    Or::

	void key_ref_put(key_ref_t key_ref);

    These can be called from interrupt context. If CONFIG_KEYS is not set then
    the argument will not be parsed.


 *  Extra references can be made to a key by calling one of the following
    functions::

	struct key *__key_get(struct key *key);
	struct key *key_get(struct key *key);

    Keys so references will need to be disposed of by calling key_put() when
    they've been finished with.  The key pointer passed in will be returned.

    In the case of key_get(), if the pointer is NULL or CONFIG_KEYS is not set
    then the key will not be dereferenced and no increment will take place.


 *  A key's serial number can be obtained by calling::

	key_serial_t key_serial(struct key *key);

    If key is NULL or if CONFIG_KEYS is not set then 0 will be returned (in the
    latter case without parsing the argument).


 *  If a keyring was found in the search, this can be further searched by::

	key_ref_t keyring_search(key_ref_t keyring_ref,
				 const struct key_type *type,
				 const char *description,
				 bool recurse)

    This searches the specified keyring only (recurse == false) or keyring tree
    (recurse == true) specified for a matching key. Error ENOKEY is returned
    upon failure (use IS_ERR/PTR_ERR to determine). If successful, the returned
    key will need to be released.

    The possession attribute from the keyring reference is used to control
    access through the permissions mask and is propagated to the returned key
    reference pointer if successful.


 *  A keyring can be created by::

	struct key *keyring_alloc(const char *description, uid_t uid, gid_t gid,
				  const struct cred *cred,
				  key_perm_t perm,
				  struct key_restriction *restrict_link,
				  unsigned long flags,
				  struct key *dest);

    This creates a keyring with the given attributes and returns it.  If dest
    is not NULL, the new keyring will be linked into the keyring to which it
    points.  No permission checks are made upon the destination keyring.

    Error EDQUOT can be returned if the keyring would overload the quota (pass
    KEY_ALLOC_NOT_IN_QUOTA in flags if the keyring shouldn't be accounted
    towards the user's quota).  Error ENOMEM can also be returned.

    If restrict_link is not NULL, it should point to a structure that contains
    the function that will be called each time an attempt is made to link a
    key into the new keyring.  The structure may also contain a key pointer
    and an associated key type.  The function is called to check whether a key
    may be added into the keyring or not.  The key type is used by the garbage
    collector to clean up function or data pointers in this structure if the
    given key type is unregistered.  Callers of key_create_or_update() within
    the kernel can pass KEY_ALLOC_BYPASS_RESTRICTION to suppress the check.
    An example of using this is to manage rings of cryptographic keys that are
    set up when the kernel boots where userspace is also permitted to add keys
    - provided they can be verified by a key the kernel already has.

    When called, the restriction function will be passed the keyring being
    added to, the key type, the payload of the key being added, and data to be
    used in the restriction check.  Note that when a new key is being created,
    this is called between payload preparsing and actual key creation.  The
    function should return 0 to allow the link or an error to reject it.

    A convenience function, restrict_link_reject, exists to always return
    -EPERM to in this case.


 *  To check the validity of a key, this function can be called::

	int validate_key(struct key *key);

    This checks that the key in question hasn't expired or and hasn't been
    revoked. Should the key be invalid, error EKEYEXPIRED or EKEYREVOKED will
    be returned. If the key is NULL or if CONFIG_KEYS is not set then 0 will be
    returned (in the latter case without parsing the argument).


 *  To register a key type, the following function should be called::

	int register_key_type(struct key_type *type);

    This will return error EEXIST if a type of the same name is already
    present.


 *  To unregister a key type, call::

	void unregister_key_type(struct key_type *type);


```
在某些情况下，可能需要处理一组密钥（bundle）
```

	struct key_type key_type_keyring;

```
这可request_key() 等函数配合使用，以在进程
密钥环中找到特定的密钥环。随后找到的密钥环可以用
keyring_search() 搜索。注意，无法使用 request_key() 
搜索特定的密钥环，因此以这种方式使用密钥环效用有限


## 访问载荷内容的注意事


最简单的载荷就是直接存储key->payload 中的数据。在这种情况
下，访问载荷时无需使用 RCU 或加锁

更复杂的载荷内容必须被分配，并将其指针设置到
key->payload.data[] 数组中。必须选择以下方式之一
     访问数据

  1) 不可修改的密钥类型

     如果密钥类型没有 modify 方法，那么密钥的载荷可以
     无需任何加锁的情况下访问，前提是已知它已
     实例化（未实例化的密钥无法被“找到”）

  2) 密钥的信号量

     信号量可用于管理对载荷的访问，并控制
     载荷指针。修改时必须对它加写锁，一般访问时
     必须加读锁。这样做的缺点是
     访问者可能需要休眠

  3) RCU銆。

     当信号量尚未持有时必须使RCU；如果信号量
     已被持有，则内容不会在你不知情的情况下意外改变，因为
     仍须用信号量来串行化对密钥的修改。密
     管理代码会为密钥类型处理好这一点

```

	rcu_read_lock() ... rcu_dereference() ... rcu_read_unlock()

     to read the pointer, and::

	rcu_dereference() ... rcu_assign_pointer() ... call_rcu()

     to set the pointer and dispose of the old contents after a grace period.
     Note that only the key type should ever modify a key's payload.

     Furthermore, an RCU controlled payload must hold a struct rcu_head for the
     use of call_rcu() and, if the payload is of variable size, the length of
     the payload. key->datalen cannot be relied upon to be consistent with the
     payload just dereferenced if the key's semaphore is not held.

     Note that key->payload.data[0] has a shadow that is marked for __rcu
     usage.  This is called key->payload.rcu_data0.  The following accessors
     wrap the RCU calls to this element:

     a) Set or change the first payload pointer::

		rcu_assign_keypointer(struct key *key, void *data);

     b) Read the first payload pointer with the key semaphore held::

		[const] void *dereference_key_locked([const] struct key *key);

	 Note that the return value will inherit its constness from the key
	 parameter.  Static analysis will give an error if it things the lock
	 isn't held.

     c) Read the first payload pointer with the RCU read lock held::

		const void *dereference_key_rcu(const struct key *key);


```
## 定义密钥类型


内核服务可能希望定义自己的密钥类型。例如，AFS
文件系统可能希望定义 Kerberos 5 票据密钥类型。为此，
需要填充一key_type 结构体并向系统注册

```

	<linux/key-type.h>

```
该结构体有多个字段，其中一些是必填的：

  - `const char *name`

     密钥类型的名称。它用于将用户空间提供的密钥类型名称
     转换为指向该结构体的指针


  - `size_t def_datalen`

     这是可选的——它提供作为配额计入的默认载荷数据长度
     如果密钥类型的载荷总是或几
     总是相同大小，那么这是一种更高效的做法

     特定密钥上的数据长度（和配额）始终可以更改
```

	int key_payload_reserve(struct key *key, size_t datalen);

     With the revised data length. Error EDQUOT will be returned if this is not
     viable.


  *  ``int (*vet_description)(const char *description);``

     This optional method is called to vet a key description.  If the key type
     doesn't approve of the key description, it may return an error, otherwise
     it should return 0.


  *  ``int (*preparse)(struct key_preparsed_payload *prep);``

     This optional method permits the key type to attempt to parse payload
     before a key is created (add key) or the key semaphore is taken (update or
     instantiate key).  The structure pointed to by prep looks like::

	struct key_preparsed_payload {
		char		*description;
		union key_payload payload;
		const void	*data;
		size_t		datalen;
		size_t		quotalen;
		time_t		expiry;
	};

     Before calling the method, the caller will fill in data and datalen with
     the payload blob parameters; quotalen will be filled in with the default
     quota size from the key type; expiry will be set to TIME_T_MAX and the
     rest will be cleared.

     If a description can be proposed from the payload contents, that should be
     attached as a string to the description field.  This will be used for the
     key description if the caller of add_key() passes NULL or "".

     The method can attach anything it likes to payload.  This is merely passed
     along to the instantiate() or update() operations.  If set, the expiry
     time will be applied to the key if it is instantiated from this data.

     The method should return 0 if successful or a negative error code
     otherwise.


  *  ``void (*free_preparse)(struct key_preparsed_payload *prep);``

     This method is only required if the preparse() method is provided,
     otherwise it is unused.  It cleans up anything attached to the description
     and payload fields of the key_preparsed_payload struct as filled in by the
     preparse() method.  It will always be called after preparse() returns
     successfully, even if instantiate() or update() succeed.


  *  ``int (*instantiate)(struct key *key, struct key_preparsed_payload *prep);``

     This method is called to attach a payload to a key during construction.
     The payload attached need not bear any relation to the data passed to this
     function.

     The prep->data and prep->datalen fields will define the original payload
     blob.  If preparse() was supplied then other fields may be filled in also.

     If the amount of data attached to the key differs from the size in
     keytype->def_datalen, then key_payload_reserve() should be called.

     This method does not have to lock the key in order to attach a payload.
     The fact that KEY_FLAG_INSTANTIATED is not set in key->flags prevents
     anything else from gaining access to the key.

     It is safe to sleep in this method.

     generic_key_instantiate() is provided to simply copy the data from
     prep->payload.data[] to key->payload.data[], with RCU-safe assignment on
     the first element.  It will then clear prep->payload.data[] so that the
     free_preparse method doesn't release the data.


  *  ``int (*update)(struct key *key, const void *data, size_t datalen);``

     If this type of key can be updated, then this method should be provided.
     It is called to update a key's payload from the blob of data provided.

     The prep->data and prep->datalen fields will define the original payload
     blob.  If preparse() was supplied then other fields may be filled in also.

     key_payload_reserve() should be called if the data length might change
     before any changes are actually made. Note that if this succeeds, the type
     is committed to changing the key because it's already been altered, so all
     memory allocation must be done first.

     The key will have its semaphore write-locked before this method is called,
     but this only deters other writers; any changes to the key's payload must
     be made under RCU conditions, and call_rcu() must be used to dispose of
     the old payload.

     key_payload_reserve() should be called before the changes are made, but
     after all allocations and other potentially failing function calls are
     made.

     It is safe to sleep in this method.


  *  ``int (*match_preparse)(struct key_match_data *match_data);``

     This method is optional.  It is called when a key search is about to be
     performed.  It is given the following structure::

	struct key_match_data {
		bool (*cmp)(const struct key *key,
			    const struct key_match_data *match_data);
		const void	*raw_data;
		void		*preparsed;
		unsigned	lookup_type;
	};

     On entry, raw_data will be pointing to the criteria to be used in matching
     a key by the caller and should not be modified.  ``(*cmp)()`` will be pointing
     to the default matcher function (which does an exact description match
     against raw_data) and lookup_type will be set to indicate a direct lookup.

     The following lookup_type values are available:

       *  KEYRING_SEARCH_LOOKUP_DIRECT - A direct lookup hashes the type and
      	  description to narrow down the search to a small number of keys.

       *  KEYRING_SEARCH_LOOKUP_ITERATE - An iterative lookup walks all the
      	  keys in the keyring until one is matched.  This must be used for any
      	  search that's not doing a simple direct match on the key description.

     The method may set cmp to point to a function of its choice that does some
     other form of match, may set lookup_type to KEYRING_SEARCH_LOOKUP_ITERATE
     and may attach something to the preparsed pointer for use by ``(*cmp)()``.
     ``(*cmp)()`` should return true if a key matches and false otherwise.

     If preparsed is set, it may be necessary to use the match_free() method to
     clean it up.

     The method should return 0 if successful or a negative error code
     otherwise.

     It is permitted to sleep in this method, but ``(*cmp)()`` may not sleep as
     locks will be held over it.

     If match_preparse() is not provided, keys of this type will be matched
     exactly by their description.


  *  ``void (*match_free)(struct key_match_data *match_data);``

     This method is optional.  If given, it called to clean up
     match_data->preparsed after a successful call to match_preparse().


  *  ``void (*revoke)(struct key *key);``

     This method is optional.  It is called to discard part of the payload
     data upon a key being revoked.  The caller will have the key semaphore
     write-locked.

     It is safe to sleep in this method, though care should be taken to avoid
     a deadlock against the key semaphore.


  *  ``void (*destroy)(struct key *key);``

     This method is optional. It is called to discard the payload data on a key
     when it is being destroyed.

     This method does not need to lock the key to access the payload; it can
     consider the key as being inaccessible at this time. Note that the key's
     type may have been changed before this function is called.

     It is not safe to sleep in this method; the caller may hold spinlocks.


  *  ``void (*describe)(const struct key *key, struct seq_file *p);``

     This method is optional. It is called during /proc/keys reading to
     summarise a key's description and payload in text form.

     This method will be called with the RCU read lock held. rcu_dereference()
     should be used to read the payload pointer if the payload is to be
     accessed. key->datalen cannot be trusted to stay consistent with the
     contents of the payload.

     The description will not change, though the key's state may.

     It is not safe to sleep in this method; the RCU read lock is held by the
     caller.


  *  ``long (*read)(const struct key *key, char __user *buffer, size_t buflen);``

     This method is optional. It is called by KEYCTL_READ to translate the
     key's payload into something a blob of data for userspace to deal with.
     Ideally, the blob should be in the same format as that passed in to the
     instantiate and update methods.

     If successful, the blob size that could be produced should be returned
     rather than the size copied.

     This method will be called with the key's semaphore read-locked. This will
     prevent the key's payload changing. It is not necessary to use RCU locking
     when accessing the key's payload. It is safe to sleep in this method, such
     as might happen when the userspace buffer is accessed.


  *  ``int (*request_key)(struct key_construction *cons, const char *op, void *aux);``

     This method is optional.  If provided, request_key() and friends will
     invoke this function rather than upcalling to /sbin/request-key to operate
     upon a key of this type.

     The aux parameter is as passed to request_key_async_with_auxdata() and
     similar or is NULL otherwise.  Also passed are the construction record for
     the key to be operated upon and the operation type (currently only
     "create").

     This method is permitted to return before the upcall is complete, but the
     following function must be called under all circumstances to complete the
     instantiation process, whether or not it succeeds, whether or not there's
     an error::

	void complete_request_key(struct key_construction *cons, int error);

     The error parameter should be 0 on success, -ve on error.  The
     construction record is destroyed by this action and the authorisation key
     will be revoked.  If an error is indicated, the key under construction
     will be negatively instantiated if it wasn't already instantiated.

     If this method returns an error, that error will be returned to the
     caller of request_key*().  complete_request_key() must be called prior to
     returning.

     The key under construction and the authorisation key can be found in the
     key_construction struct pointed to by cons:

      *  ``struct key *key;``

     	 The key under construction.

      *  ``struct key *authkey;``

     	 The authorisation key.


  *  ``struct key_restriction *(*lookup_restriction)(const char *params);``

     This optional method is used to enable userspace configuration of keyring
     restrictions. The restriction parameter string (not including the key type
     name) is passed in, and this method returns a pointer to a key_restriction
     structure containing the relevant functions and data to evaluate each
     attempted key link operation. If there is no match, -EINVAL is returned.


  *  ``asym_eds_op`` and ``asym_verify_signature``::

       int (*asym_eds_op)(struct kernel_pkey_params *params,
			  const void *in, void *out);
       int (*asym_verify_signature)(struct kernel_pkey_params *params,
				    const void *in, const void *in2);

     These methods are optional.  If provided the first allows a key to be
     used to encrypt, decrypt or sign a blob of data, and the second allows a
     key to verify a signature.

     In all cases, the following information is provided in the params block::

	struct kernel_pkey_params {
		struct key	*key;
		const char	*encoding;
		const char	*hash_algo;
		char		*info;
		__u32		in_len;
		union {
			__u32	out_len;
			__u32	in2_len;
		};
		enum kernel_pkey_operation op : 8;
	};

     This includes the key to be used; a string indicating the encoding to use
     (for instance, "pkcs1" may be used with an RSA key to indicate
     RSASSA-PKCS1-v1.5 or RSAES-PKCS1-v1.5 encoding or "raw" if no encoding);
     the name of the hash algorithm used to generate the data for a signature
     (if appropriate); the sizes of the input and output (or second input)
     buffers; and the ID of the operation to be performed.

     For a given operation ID, the input and output buffers are used as
     follows::

	Operation ID		in,in_len	out,out_len	in2,in2_len
	=======================	===============	===============	===============
	kernel_pkey_encrypt	Raw data	Encrypted data	-
	kernel_pkey_decrypt	Encrypted data	Raw data	-
	kernel_pkey_sign	Raw data	Signature	-
	kernel_pkey_verify	Raw data	-		Signature

     asym_eds_op() deals with encryption, decryption and signature creation as
     specified by params->op.  Note that params->op is also set for
     asym_verify_signature().

     Encrypting and signature creation both take raw data in the input buffer
     and return the encrypted result in the output buffer.  Padding may have
     been added if an encoding was set.  In the case of signature creation,
     depending on the encoding, the padding created may need to indicate the
     digest algorithm - the name of which should be supplied in hash_algo.

     Decryption takes encrypted data in the input buffer and returns the raw
     data in the output buffer.  Padding will get checked and stripped off if
     an encoding was set.

     Verification takes raw data in the input buffer and the signature in the
     second input buffer and checks that the one matches the other.  Padding
     will be validated.  Depending on the encoding, the digest algorithm used
     to generate the raw data may need to be indicated in hash_algo.

     If successful, asym_eds_op() should return the number of bytes written
     into the output buffer.  asym_verify_signature() should return 0.

     A variety of errors may be returned, including EOPNOTSUPP if the operation
     is not supported; EKEYREJECTED if verification fails; ENOPKG if the
     required crypto isn't available.


  *  ``asym_query``::

       int (*asym_query)(const struct kernel_pkey_params *params,
			 struct kernel_pkey_query *info);

     This method is optional.  If provided it allows information about the
     public or asymmetric key held in the key to be determined.

     The parameter block is as for asym_eds_op() and co. but in_len and out_len
     are unused.  The encoding and hash_algo fields should be used to reduce
     the returned buffer/data sizes as appropriate.

     If successful, the following information is filled in::

	struct kernel_pkey_query {
		__u32		supported_ops;
		__u32		key_size;
		__u16		max_data_size;
		__u16		max_sig_size;
		__u16		max_enc_size;
		__u16		max_dec_size;
	};

     The supported_ops field will contain a bitmask indicating what operations
     are supported by the key, including encryption of a blob, decryption of a
     blob, signing a blob and verifying the signature on a blob.  The following
     constants are defined for this::

	KEYCTL_SUPPORTS_{ENCRYPT,DECRYPT,SIGN,VERIFY}

     The key_size field is the size of the key in bits.  max_data_size and
     max_sig_size are the maximum raw data and signature sizes for creation and
     verification of a signature; max_enc_size and max_dec_size are the maximum
     raw data and signature sizes for encryption and decryption.  The
     max_*_size fields are measured in bytes.

     If successful, 0 will be returned.  If the key doesn't support this,
     EOPNOTSUPP will be returned.


```
## Request-Key 回调服务


为创建新密钥，内核将尝试执行以下命令
```

	/sbin/request-key create <key> <uid> <gid> \
		<threadring> <processring> <sessionring> <callout_info>

```
<key> 是正在构造的密钥，三个密钥环是发
搜索的进程所属的密钥环。它们被包含进来
     有两个原因：

   1  某个密钥环中可能有一个认证令牌，
      获取该密钥需要它，例如：一Kerberos 票据授予票据（Ticket-Granting Ticket）

   2  新密钥可能应被缓存到这些环之一中

该程序应在尝试访问更多密钥之前，
     将其 UID GID 设置为指定的值。然后它可能寻找一
     用户专属进程来转交该请求（也许是放在另一个密钥中
     例如KDE 桌面管理器放置的路径）

该程序（或其调用的任何东西）应通过调用
KEYCTL_INSTANTIATE KEYCTL_INSTANTIATE_IOV 来完成密钥的构造，
     这还允许它在返回前将密钥缓存到某个密钥环（可能是会话环）中
     或者，可以使用 KEYCTL_NEGATE 将密钥标记为负向
     或使KEYCTL_REJECT；这也允许将密钥缓存
     某个密钥环中

如果它返回时密钥仍处于未构造状态，则该密钥将被
     标记为负向，被添加到会话密钥环，并向
     密钥请求者返回错误

调用此服务的任何一方都可能提供补充信息
     这将作为 <callout_info> 参数传入。如果没有提
     此类信息，则会传入”作为该参数
     替代


类似地，内核可能尝试更新一个已过期或即将过期的密钥
```

	/sbin/request-key update <key> <uid> <gid> \
		<threadring> <processring> <sessionring>

```
在这种情况下，程序不需要实际将密钥附加到某个环上；
     提供这些环仅供参考


## 垃圾回收


已废弃的密钥（其类型已被移除）会自动
     指向它们的密钥环中取消链接，并尽快由
     后台垃圾回收器删除

类似地，已撤销和已过期的密钥也会被垃圾回收，但仅在
```

	/proc/sys/kernel/keys/gc_delay

```