## Linux 中的凭证


作者：David Howells <dhowells@redhat.com>


## 概述


当一个对象作用于另一个对象时，Linux 执行的安全检查包含几个部分：

 1. 对象（Objects）

     对象是系统中可以被用户空间程序直接作用的事物。Linux 有多种可操作的对象，
     包括

 - 任务（Tasks
 - 文件/inode
 - 套接字（Sockets
 - 消息队列（Message queues
 - 共享内存段（Shared memory segments
 - 信号量（Semaphores
 - 密钥（Keys

     作为所有这些对象描述的一部分，都有一组凭证。这组凭证中包含什么取决于
     对象的类型

 2. 对象所有权（Object ownership）

     在大多数对象的凭证中，会有一个子集指示该对象的所有权。这用于资源记账
     限制（例如磁盘配额和任务rlimits）

     例如，在标准UNIX 文件系统中，这由标记inode 上的 UID 定义

 3. 客观上下文（The objective context）

     同样在这些对象的凭证中，会有一个子集指示该对象的“客观上下文”。这可能
     ）中的集合相同，也可能不同——例如，在标UNIX 文件中，这由标记inode
     上的 UID GID 定义

     客观上下文被作为安全计算的一部分使用，该计算在对象被作用时执行

 4. 主体（Subjects）

     主体是一个作用于另一个对象的对象

     系统中的大多数对象都是非活跃的：它们不会对系统中的其他对象起作用。进
     任务是明显的例外：它们会做事；它们会访问并操纵事物

     除任务之外的对象在某些情况下也可能成为主体。例如，一个打开的文件可能会使用
     由对其调用了 `fcntl(F_SETOWN)` 的任务赋予的 UID EUID，向某个任务发
     SIGIO。在这种情况下，file 结构也将拥有一个主观上下文

 5. 主观上下文（The subjective context）

     主体对其凭证有额外的解释。其凭证的一个子集构成“主观上下文”。主观上下文
     作为安全计算的一部分使用，该计算在主体起作用时执行

     例如，一Linux 任务在作用于一个文件时拥有 FSUID、FSGID 和附加组列表—
     这与通常构成任务客观上下文的真实 UID GID 是完全分开的

 6. 动作（Actions）

     Linux 提供了一系列主体可以对对象执行的动作。可用动作的集合取决于主体和
     对象的性质

     动作包括读取、写入、创建和删除文件；fork 或向任务发送信号和跟踪任务

 7. 规则、访问控制列表与安全计算（Rules, access control lists and security calculations）

     当主体作用于一个对象时，会进行一次安全计算。这涉及获取主观上下文、客观上下文
     和动作，并搜索一个或多个规则集，以判断在给定这些上下文的情况下，主体是否
     授予或拒绝以期望的方式作用于该对象的权限

     规则有两大来源：

     a. 自主访问控制（DAC）：

	 有时对象会作为自身描述的一部分包含规则集。这就是“访问控制列表”或“ACL”
	 一Linux 文件可能提供多个 ACL

	 例如，传统的 UNIX 文件包含一个权限掩码，它是一个缩写的 ACL，包含三个固
	 的主体类别（'user'group' 'other'），每一类都可能被授予某些特
	 read'write' 'execute'——无论它们映射到该对象的什么含义）。然而，
	 UNIX 文件权限不允许任意指定主体，因此用途有限

	 一Linux 文件也可能带POSIX ACL。这是一个向任意主体授予各种权限的规
	 列表

     b. 强制访问控制（MAC）：

	 整个系统可能拥有一组或多组规则，无论其来源如何，都会应用于所有主体和
	 对象。SELinux Smack 就是这方面的例子

	 对于 SELinux Smack，每个对象都被赋予一个标签作为其凭证的一部分。当请求
	 一个动作时，它们会获取主体标签、对象标签和动作，并查找一条说明该动作是被
	 授予还是被拒绝的规则


## 凭证的类


Linux 内核支持以下类型的凭证：

 1. 传统UNIX 凭证

 - 真实用户 ID（Real User ID
 - 真实ID（Real Group ID

     大多数（如果不是全部）Linux 对象都携UID GID，即使在有些情况下必
     去“发明”它们（例如 FAT CIFS 文件，它们派生自 Windows）。这些（大多）定
     了该对象的客观上下文，任务在某些情况下略有不同

 - 有效、保存和 FS 用户 ID（Effective, Saved and FS User ID
 - 有效、保存和 FS ID（Effective, Saved and FS Group ID
 - 附加组（Supplementary groups

     这些仅由任务使用的附加凭证。通常，EUID/EGID/GROUPS 会被用作主观上下文，
     真实UID/GID 会被用作客观上下文。对于任务，应当注意这并非总是成立

 2. 能力（Capabilities）

 - 允许的能力集合（Set of permitted capabilities
 - 可继承的能力集合（Set of inheritable capabilities
 - 有效的能力集合（Set of effective capabilities
 - 能力边界集合（Capability bounding set

     这些仅由任务携带。它们表示以零散方式授予任务的、普通任务原本不会拥有的
     高级能力。它们会随着传统 UNIX 凭证的改变而被隐式操纵，但也可以通过 `capset()`
     系统调用直接操纵

     允许的能力是进程可能通过 `capset()` 授予自身、加入到其有效或允许集合中的那些
     能力。这个可继承集合也可能受到这样的约束

     有效能力是任务实际被允许自己使用的能力

     可继承能力是可能跨越 `execve()` 传递下去的能力

     边界集合限制了可能跨`execve()` 继承的能力，尤其是当执行的二进制文件将以
     UID 0 的身份执行时

 3. 安全管理标志（securebits）

     这些仅由任务携带。它们管理上述凭证在某些操作（如 execve()）之上被操纵
     继承的方式。它们不直接用作客观或主观凭证

 4. 密钥与密钥环（Keys and keyrings）

     这些仅由任务携带。它们携带并缓存不适合放入其他标准 UNIX 凭证的安全令牌。它
     用于使网络文件系统密钥等事物对进程执行的文件访问可用，而无需普通程序必
     了解其中涉及的安全细节

     密钥环是一种特殊类型的密钥。它们携带一组其他的密钥，并可以被搜索以找到所需
     的密钥。每个进程可以订阅若干密钥环

	每线程密钥环（Per-thread keying
	每进程密钥环（Per-process keyring
	每会话密钥环（Per-session keyring

     当进程访问一个密钥时，如果它尚未存在，通常会被缓存到这些密钥环之一上，以便
     将来的访问找到它

     关于使用密钥的更多信息，请参`Documentation/security/keys/*`

 5. LSM

     Linux 安全模块（Linux Security Module）允许对任务可能执行的操作施加额外的
     控制。目Linux 支持若干 LSM 选项

     有些通过给系统中的对象打标签，然后应用一系列规则（策略），说明拥有某个标
     的任务可以对拥有另一个标签的对象做什么操作

 6. AF_KEY

     这是一种基于套接字的、面向网络栈的凭证管理方[RFC 2367]。本文档不讨论它
     因为它不直接与任务和文件凭证交互；相反，它维护系统级别的凭证


当打开一个文件时，打开任务的主观上下文的一部分会被记录在所创建file 结构中。这
使得使用file 结构的操作可以使用那些凭证，而不是发出该操作的任务的主观上下文
一个例子是打开在网络文件系统上的文件，其中被打开文件的凭证应当呈现给服务器，
不管实际是谁在进行读或写


## 文件标记


磁盘上或通过网络获取的文件可能带有注释，这些注释构成该文件的客观安全上下文
根据文件系统的类型，这可能包含一个或多个以下内容

 - UNIX UID、GID、模式（mode）；
 - Windows 用户 ID
 - 访问控制列表（Access control list）；
 - LSM 安全标签
 - UNIX 执行权限提升位（SUID/SGID）；
 - 文件能力执行权限提升位（File capabilities exec privilege escalation bits）

这些会被与该任务的主观安全上下文进行比较，并因此而允许或禁止某些操作。在 execve()
的情况下，权限提升位会起作用，并可能基于可执行文件上的注释，赋予结果进程额外
特权


## 任务凭证


Linux 中，任务的所有凭证都通过 (uid, gid) 或经(groups, keys, LSM security)
保存在一个类型为 'struct cred' 的、带引用计数的结构中。每个任务通过一个名为其
task_struct 中的 'cred' 的指针指向它的凭证

一旦一组凭证被准备好并提交，它就不得被更改，除非以下例外：

 1. 它的引用计数可能被更改；

 2. 它所指向group_info 结构的引用计数可能被更改

 3. 它所指向的安全数据的引用计数可能被更改；

 4. 它所指向的任何密钥环的引用计数可能被更改

 5. 它所指向的任何密钥环可能被吊销、过期或更改其安全属性；以及

 6. 它所指向的任何密钥环的内容可能被更改（密钥环的全部意义在于作为一组共享的
    凭证，任何拥有适当访问权限的人都可以修改）

要更cred 结构中的任何内容，必须遵循“复替换”原则。首先取一份副本，然后
更改副本，然后使RCU 更改任务指针，使其指向新的副本。有一些包装函数可以辅
完成此事（见下文）

一个任务只能更改它_自己的_凭证；不再允许一个任务更改另一个任务的凭证。这意味着
`capset()` 系统调用不再被允许接受除当前进程 PID 之外的任PID。同样，`keyctl_instantiate()`
`keyctl_negate()` 函数也不再允许附加到请求进程中的进程特定密钥环，因为实例
进程可能需要创建它们


### 不可变凭


一旦一组凭证被公开（例如通过调用 `commit_creds()`），它必须被视为不可变的，除
两个例外

 1. 引用计数可能被更改

 2. 虽然一组凭证的密钥环订阅不能被更改，但所订阅的密钥环的内容可以被更改

为了在编译时捕获意外的凭证更改，struct task_struct 以及 struct file 都拥有指向其
凭证集合_const_ 指针。此外，`get_cred()` `put_cred()` 等某些函数在 const
指针上操作，从而无需进行类型转换，但需要临时去const 限定以便能够更改引用计数


### 访问任务凭证


一个任务只能更改它自己的凭证，这使得当前进程无需任何形式锁定的情况下就可以读
或替换它自己的凭证：

```
	const struct cred *current_cred()
```

来获取指向其凭证结构的指针，并且之后不必释放它

有一些方便使用的包装函数用于获取任务凭证的特定方面：

```
	uid_t current_uid(void)		Current 的真UID
	gid_t current_gid(void)		Current 的真GID
	uid_t current_euid(void)	Current 的有UID
	gid_t current_egid(void)	Current 的有GID
	uid_t current_fsuid(void)	Current 的文件访UID
	gid_t current_fsgid(void)	Current 的文件访GID
	kernel_cap_t current_cap(void)	Current 的有效能
	struct user_struct *current_user(void)  Current 的用户账
```

还有一些方便使用的包装函数用于获取特定的关联对

```
	void current_uid_gid(uid_t *, gid_t *);
	void current_euid_egid(uid_t *, gid_t *);
	void current_fsuid_fsgid(uid_t *, gid_t *);
```

这些函数在从当前任务的凭证中检索出这些成对的值之后，通过它们的参数返回这些值


此外，还有一个用于获取当前凭证引用的函数

```
	const struct cred *get_current_cred(void);
```

以及用于获取某个凭证引用的函数：

```
	struct user_struct *get_current_user(void);
	struct group_info *get_current_groups(void);
```

它们分别获取对当前进程的用户记账结构和附加组列表的引用

一旦获得引用，必须酌情使用 `put_cred()`、`free_uid()` `put_group_info()` 释放


### 访问另一个任务的凭证


虽然一个任务可以在无需锁定的情况下访问自己的凭证，但想要访问另一个任务凭证的任务
并非如此。它必须使用 RCU 读锁`rcu_dereference()`

```
	const struct cred *__task_cred(struct task_struct *task);
```

```
	void foo(struct task_struct *t, struct foo_data *f)
	{
		const struct cred *tcred;
		...
		rcu_read_lock();
		tcred = __task_cred(t);
		f->uid = tcred->uid;
		f->gid = tcred->gid;
		f->groups = get_group_info(tcred->groups);
		rcu_read_unlock();
		...
	}
```

如果确实需要长时间持有另一个任务的凭证，并且可能在此过程中睡眠，那么调用者应
获取一个引用：

```
	const struct cred *get_task_cred(struct task_struct *task);
```

这会在内部完成所有的 RCU 魔法。调用者必须在用完所获凭证后对其调用 put_cred()

   `__task_cred()` 的结果不应直接传`get_cred()`，因为这可能`commit_cred()`
   产生竞争

有两个方便函数用于访问另一个任务凭证的某些部分

```
	uid_t task_uid(task)		Task 的真UID
	uid_t task_euid(task)		Task 的有UID
```

```
	__task_cred(task)->uid
	__task_cred(task)->euid
```

应该改用它们。类似地，如果需要访问任务凭证的多个方面，则应使RCU 读锁，调
`__task_cred()`，将结果保存在一个临时指针中，然后在释放锁之前从中访问凭证的
各个方面。这可以防止可能代价高昂RCU 魔法被调用多次

如果还需要访问另一个任务凭证的某些其他单一方面

```
	task_cred_xxx(task, member)
```

```
	uid_t task_cred_xxx(task, suid);
```

**将从中获'struct cred** suid：task，并执行适当RCU 魔法。这不得用于指针
成员，因为它们所指向的内容可能在 RCU 读锁被释放的瞬间消失


### 更改凭证


如前所述，一个任务只能更改它自己的凭证，而不能更改另一个任务的凭证。这意味着
不需要使用任何锁来更改自己的凭证

要更改当前进程的凭证，一个函数应首先准备一组新的凭证：

```
	struct cred *prepare_creds(void);
```

这会锁定 current->cred_replace_mutex，然后分配并构造当前进程凭证的一个副本，如果
成功则在仍持mutex 的情况下返回。如果不成功（内存不足）则返NULL

mutex 防止 `ptrace()` 在正在对凭证进行构造和更改的安全检查时改变一个进程的
ptrace 状态，因为 ptrace 状态可能改变结果，尤其是在 `execve()` 的情况下

新的凭证集应当被适当地更改，并执行任何安全检查和钩子。当前和拟议的凭证集都可
用于此目的，因为 current_cred() 此时仍会返回当前集合

在替换组列表时，新列表在被加入凭证之前必须已经排序，因为会用二分查找来测试成
关系。在实践中，这意味着应在 set_groups() set_current_groups() 之前调用
groups_sort()。groups_sort() 不得在一个共享的 `struct group_list` 上调用，因为
即使数组已经有序，它也可能在排序过程中置换元素

当凭证集准备就绪时，应将其提交给当前进程

```
	int commit_creds(struct cred *new);
```

这会更改凭证和进程的各个方面，给 LSM 一个同样处理的机会，然后它会使
`rcu_assign_pointer()` 来实际将新凭证提交给 `current->cred`，它会释
`current->cred_replace_mutex` 以允`ptrace()` 发生，并且会通知调度器及其他部分
关于这些更改

该函数保证返0，因此可以在 `sys_setresuid()` 等函数的末尾进行尾调用

注意，该函数消费调用者对新凭证的引用。调用者之后_不应_对新凭证调用 `put_cred()`

此外，一旦对已一组新凭证调用了该函数，那些凭证就_不能再_被进一步更改


如果在调`prepare_creds()` 之后安全检查失败或发生了其他一些错误，那么应使
以下函数

```
	void abort_creds(struct cred *new);
```

这会释放 `prepare_creds()` 获取`current->cred_replace_mutex` 上的锁，然后释放
新的凭证


```
	int alter_suid(uid_t suid)
	{
		struct cred *new;
		int ret;

		new = prepare_creds();
		if (!new)
			return -ENOMEM;

		new->suid = suid;
		ret = security_alter_suid(new);
		if (ret < 0) {
			abort_creds(new);
			return ret;
		}

		return commit_creds(new);
	}
```


### 管理凭证


有一些函数可以帮助管理凭证：

 - `void put_cred(const struct cred *cred);`

     这会释放对给定凭证集的一个引用。如果引用计数达到零，该凭证将被 RCU 系统
     安排销毁

 - `const struct cred **get_cred(const struct cred **cred);`

     这会获取对一个存活凭证集的引用，返回指向该凭证集的指针


## 打开文件的凭


当打开一个新文件时，会获取对打开任务凭证的一个引用，并将其作`f_cred` 附加
file 结构上，取代 `f_uid` `f_gid`。过去用于访`file->f_uid` `file->f_gid`
的代码现在应访问 `file->f_cred->fsuid` `file->f_cred->fsgid`

访问 `f_cred` 时可以不使用 RCU 或锁定，因为file 结构的生命周期内该指针不会改变，
其所指向cred 结构的内容也不会改变，除了上面列出的例外（参见任务凭证一节）

为了避免“混淆代理（confused deputy）”权限提升攻击，在已打开文件上的后续操作
进行的访问控制检查应使用这些凭证，而不是“current”的凭证，因为该文件可能已被
传递给一个权限更高的进程

## 覆盖 VFS 对凭证的使用


在某些情形下，期望覆VFS 使用的凭证，这可以通过使用一组不同的凭证调用
`vfs_mkdir()` 来实现。这在以下地方完成：

 - `sys_faccessat()`銆。
 - `vfs_coredump()`銆。
 - nfs4recover.c銆。
