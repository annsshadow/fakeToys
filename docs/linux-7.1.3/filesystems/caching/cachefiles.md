
## 已挂载文件系统上的缓

 (*) 概述 (*) 要求 (*) 配置 (*) 启动缓存 (*) 需要避免的事项 (*) 缓存剔除（Culling） (*) 缓存结构 (*) 安全模型SELinux (*) 关于安全性的说明 (*) 统计信息 (*) 调试 (*) 按需读取（On-demand Read）

## 概述


CacheFiles 是一个缓存后端，旨在使用已挂载的本地类型文件系统（例Ext3）上某个目录作为缓存
CacheFiles 使用一个用户空间守护进程来完成部分缓存管理工作——例如回收过期节点和剔除（culling）。该守护进程名为 cachefilesd，位/sbin
缓存的文件系统与数据完整性，并不亚于提供后端服务的文件系统。请注意，CacheFiles 不尝试做任何日志（journal），因为各种文件系统的日志接口性质上非常特定
CacheFiles 创建一misc 字符设备—/dev/cachefiles"——用于与守护进程通信。同一时刻只能有一个实体打开它，在其打开期间，缓存至少部分存在。守护进程打开它并向其发送命令以控制缓存
CacheFiles 目前限制为单个缓存
CacheFiles 试图在文件系统上维持至少一定量的空闲空间，必要时通过剔除其所含对象来收缩缓存以腾出空间——参Cache Culling"一节。这意味着它可以放置在与活动数据集相同的介质上，并将扩展以利用空闲空间，并在数据集需要更多空间时自动收缩

## 要求


使用 CacheFiles 及其守护进程需要系统和缓存文件系统中提供以下特性：

 - dnotify銆。
 - 扩展属性（xattrs）
 - openat() 及其相关函数
 - 文件系统中文件的 bmap() 支持（FIBMAP ioctl）
 - 使用 bmap() 检测文件末尾的局部页（partial page）
强烈建议在用作缓存的 Ext3 文件系统上启dir_index"选项

## 配置


缓存/etc/cachefilesd.conf 中的一个脚本配置。这些命令将缓存设置为就绪可用。可用的脚本命令如下
 brun <N>%, bcull <N>%, bstop <N>%, frun <N>%, fcull <N>%, fstop <N>%
	配置剔除（culling）限制。可选。参见关于剔除的章节	默认值分别为 7%（run）%（cull）和 1%（stop）
	'b' 开头的命令是文件空间（块）限制，以 'f' 开头的命令是文件数量限制
 dir <path>
	指定包含缓存根目录的目录。必填
 tag <name>
	指定一tag FS-Cache，用于区分多个缓存	可选。默认值为"CacheFiles"
 debug <mask>
	指定一个数值位掩码，用于控制内核模块中的调试	可选。默认值为零（全部关闭）。可将以下OR 进掩码以收集各种信息
		==	=================================================
		1	开启函数入口跟踪（_enter() 宏）
		2	开启函数出口跟踪（_leave() 宏）
		4	开启内部调试点跟踪（_debug()		==	=================================================

```
	echo 5 > /sys/module/cachefiles/parameters/debug


```
## 启动缓存


缓存通过运行守护进程来启动。守护进程打开缓存设备，配置缓存并告知其开始缓存。此时缓存绑定到 fscache，缓存变为活动状态
```
	/sbin/cachefilesd [-d]* [-s] [-n] [-f <configfile>]

```
这些标志的含义如下：

 `-d`
	提高调试级别。可多次指定，且自身累积
 `-s`
	将消息发送到 stderr 而非 syslog
 `-n`
	不守护进程化，不进入后台
 `-f <configfile>`
	使用替代配置文件而非默认配置文件

## 需要避免的事项


不要在缓存内挂载其他内容，因为这会导致问题。内核模块包含其自身非常精简的路径遍历设施，它会忽略挂载点，但守护进程无法避开它们
在缓存处于活动状态时，不要创建、重命名或删除缓存中的文件和目录，因为这可能导致状态变得不确定
重命名缓存中的文件可能使对象看起来像其他对象（文件名是查找键的一部分）
不要更改或删除缓存附加到缓存文件上的扩展属性，因为这会导致缓存状态管理混乱
不要在缓存中创建文件或目录，以免缓存混乱或提供不正确的数据
不要 chmod 缓存中的文件。该模块以最小权限创建内容，以防止随机用户能够直接访问它们

## 缓存剔除


缓存可能需要偶尔剔除以腾出空间。这涉及丢弃缓存中比任何其他对象更久未被使用的对象。剔除基于数据对象的访问时间。空目录在不使用时会被剔除
缓存剔除基于底层文件系统中可用块和可用文件的百分比进行。共有六限制"
 brun, frun
     如果缓存中的空闲空间量和可用文件数都高于这两个限制，则关闭剔除
 bcull, fcull
     如果缓存中的可用空间量或可用文件数低于其中任一限制，则启动剔除
 bstop, fstop
     如果缓存中的可用空间量或可用文件数低于其中任一限制，则在剔除将数值重新提升到这些限制之上之前，不允许进一步分配磁盘空间或文件
```
	0 <= bstop < bcull < brun < 100
	0 <= fstop < fcull < frun < 100

```
请注意，这些是可用空间和可用文件的百分比，并_不_表现100 减去"df"程序显示的百分比
用户空间守护进程扫描缓存以构建可剔除对象的表。然后按最近最少使用（LRU）顺序剔除它们。一旦表中腾出空间，就会启动对缓存的新扫描。如果对象的 atime 已改变，或内核模块表示仍在使用它们，则会跳过这些对象

## 缓存结构


CacheFiles 模块会在它所给定的目录中创建两个目录
 - cache/
 - graveyard/

活动的缓存对象全部驻留在第一个目录中。CacheFiles 内核模块将任何已退役或已剔除、且无法简unlink 的对象移动到 graveyard，守护进程将从那里实际删除它们
守护进程使用 dnotify 监视 graveyard 目录，并会删除其中出现的任何内容

该模块将索引对象表示为以文件I..."J..."命名的目录。请注意cache/"目录本身就是一个特殊的索引
数据对象在没有子对象时表示为文件，有子对象时表示为目录。它们的文件名都D..."E..."开头。如果表示为目录，数据对象将有一个名data"的文件位于该目录中，实际持有数据
特殊对象与数据对象类似，只是其文件名S..."T..."开头

如果对象有子对象，则它将被表示为一个目录。在表示目录中紧邻的位置是一组目录，这些目录以子对象键的哈希值命名，并带有前缀'@'。如果可能，子对象的表示将被放入此目录中

```
	 /INDEX    /INDEX     /INDEX                            /DATA FILES
	/=========/==========/=================================/================
	cache/@4a/I03nfs/@30/Ji000000000000000--fHg8hi8400
	cache/@4a/I03nfs/@30/Ji000000000000000--fHg8hi8400/@75/Es0g000w...DB1ry
	cache/@4a/I03nfs/@30/Ji000000000000000--fHg8hi8400/@75/Es0g000w...N22ry
	cache/@4a/I03nfs/@30/Ji000000000000000--fHg8hi8400/@75/Es0g000w...FP1ry


```
如果键的长度过长，加上修饰后超出NAME_MAX，那么它将被切成多段，前几段用于创建嵌套目录，最后一段将是最后一个目录中的对象。中间目录的名称将带
```
	J1223/@23/+xy...z/+kl...m/Epqr


```
请注意，键是原始数据，它们不仅大小可能超NAME_MAX，还可能包含诸如'/'NUL 字符之类的内容，因此可能不适合直接转换为文件名
为处理这一点，CacheFiles 会直接使用一个合适的可打印文件名，并对那些不直接合适的文件名进base-64"编码。对象文件名的两种版本指示了编码方式
	===============	===============	===============
	OBJECT TYPE	PRINTABLE	ENCODED
	===============	===============	===============
	Index		"I..."		"J..."
	Data		"D..."		"E..."
	Special		"S..."		"T..."
	===============	===============	===============

中间目录始终@"+"（视情况而定）

缓存中的每个对象都有一个扩展属性标签，保存对象类型 ID（用于区分特殊对象）以及来自 netfs 的辅助数据。后者用于检测缓存中的过期对象并更新或退役它们

请注意，CacheFiles 会清除缓存中任何它无法识别或类型不正确的文件（例FIFO 文件或设备文件）

## 安全模型SELinux


CacheFiles 的实现能够正确处Linux kernel LSM 安全特性和 SELinux 设施
CacheFiles 面临的问题之一是，它通常代表某个进程行事，并运行在该进程的上下文中，这包含一个不适合访问缓存的安全上下文——要么是因为缓存中的文件对该进程不可访问，要么是因为如果该进程在缓存中创建文件，该文件可能对其他进程不可访问
CacheFiles 的工作方式是临时更改进程所充当的安全上下文（fsuid、fsgid actor 安全标签）——而不更改当该进程作为其他进程所执行操作的目标时的安全上下文（因此信号等仍能正常工作）

当要CacheFiles 模块绑定到其缓存时，它会
 (1) 找到附加到根缓存目录的安全标签，并将其用作创建文件时所用的安全标签。默认情况下
```
	cachefiles_var_t

 (2) Finds the security label of the process which issued the bind request
     (presumed to be the cachefilesd daemon), which by default will be::

	cachefilesd_t

     and asks LSM to supply a security ID as which it should act given the
     daemon's label.  By default, this will be::

	cachefiles_kernel_t

     SELinux transitions the daemon's security ID to the module's security ID
     based on a rule of this form in the policy::

	type_transition <daemon's-ID> kernel_t : process <module's-ID>;

     For instance::

	type_transition cachefilesd_t kernel_t : process cachefiles_kernel_t;


```
模块security ID 赋予它在缓存中创建、移动和删除文件与目录，查找并访问缓存中的目录和文件，设置和访问缓存对象上的扩展属性，以及读写缓存中文件的权限
守护进程security ID 只赋予它一组非常受限的权限：它可以扫描目录、stat 文件并删除文件和目录。它不能读写缓存中的文件，因此无法访问其中缓存的数据；也不允许在缓存中创建新文件

可用的策略源文件位于
	https://people.redhat.com/~dhowells/fscache/cachefilesd-0.8.tar.bz2

```
	cachefilesd.te
	cachefilesd.fc
	cachefilesd.if

```
它们RPM 直接构建并安装
如果使用的是非基RPM 的系统，则将以上文件复制到它们各自的

```
	make -f /usr/share/selinux/devel/Makefile
	semodule -i cachefilesd.pp

```
在构建之前，你需要安checkpolicy selinux-policy-devel

默认情况下，缓存位于 /var/fscache，但如果希望它位于其他地方，则必须修改上述策略文件，或者安装一个辅助策略来标记缓存的备用位置
有关如何添加辅助策略以使缓存能够

```
	/usr/share/doc/cachefilesd-*/move-cache.txt

```
当安装了 cachefilesd rpm 时；或者，该文档也可以在源码中找到

## 关于安全性的说明


CacheFiles 利用task_struct 中的拆分安全（split security）。它分配自己task_security 结构，并在代表另一个进程行事时，在该进程的上下文中current->cred 重定向指向它
它这样做的原因是，它调用 vfs_mkdir() 之类的函数，而不是直接绕过安全调inode 操作。因此，在某些情形下，由于缓存代码运行在最初对 netfs 发起系统调用的那个进程的安全上下文中，VFS LSM 可能会拒CacheFiles 访问缓存数据
此外，如CacheFiles 创建了文件或目录，创建该对象时的安全参数（UID、GID、安全标签）将源自发起系统调用的那个进程，从而可能阻止其他进程访问缓存——包CacheFiles 的缓存管理守护进程（cachefilesd）
所需的是临时覆盖发起系统调用的进程的安全。然而，我们不能仅仅就地更改安全数据，因为那会影响进程作为对象（object）的方面，而不仅仅是作为主体（subject）的方面。这意味着它可能丢失信号或 ptrace 事件等，并影响该进程/proc 中的外观
因此，CacheFiles 利用了客观安全（task->real_cred）与主观安全（task->cred）之间的逻辑拆分。客观安全持有进程的内在安全属性，且永不被覆盖。这就是出现/proc 中的内容，也是当进程作为其他进程所执行操作的目标（例如 SIGKILL）时所使用的内容
主观安全持有进程的活跃安全属性，且可被覆盖。它在外部不可见，并在进程作用于另一对象时使用，例如 SIGKILL 另一个进程或打开文件
存在 LSM 钩子，允SELinux（或 Smack 或其他）拒绝 CacheFiles 以特定安全标签的上下文运行的请求，或以另一安全标签创建文件和目录的请求

## 统计信息


```
	CONFIG_CACHEFILES_HISTOGRAM=y

```
然后它将收集某些统计信息并通过一proc 文件显示
 /proc/fs/cachefiles/histogram

```
	cat /proc/fs/cachefiles/histogram
	JIFS  SECS  LOOKUPS   MKDIRS    CREATES
	===== ===== ========= ========= =========

     This shows the breakdown of the number of times each amount of time
     between 0 jiffies and HZ-1 jiffies a variety of tasks took to run.  The
     columns are as follows:

	=======		=======================================================
	COLUMN		TIME MEASUREMENT
	=======		=======================================================
	LOOKUPS		Length of time to perform a lookup on the backing fs
	MKDIRS		Length of time to perform a mkdir on the backing fs
	CREATES		Length of time to perform a create on the backing fs
	=======		=======================================================

     Each row shows the number of events that took a particular range of times.
     Each step is 1 jiffy in size.  The JIFS column indicates the particular
     jiffy range covered, and the SECS field the equivalent number of seconds.


```
## 调试


如果启用CONFIG_CACHEFILES_DEBUG，CacheFiles 设施可以具有运行
```
	/sys/module/cachefiles/parameters/debug

```
这是一个用于启用调试流的位掩码
	=======	=======	===============================	=======================
	BIT	VALUE	STREAM				POINT
	=======	=======	===============================	=======================
	0	1	General				函数入口跟踪
	1	2					函数出口跟踪
	2	4					General
	=======	=======	===============================	=======================

应将适当的一组OR 在一起，并将结果写入

```
	echo $((1|4|8)) >/sys/module/cachefiles/parameters/debug

```
将开启所有函数入口调试

## 按需读取


在其原始模式下工作时，CacheFiles 作为远程网络文件系统的本地缓存；而在按需读取模式下，CacheFiles 可以提升需要按需读取语义的场景，例如容器镜像分发
这两种模式之间的本质区别体现在发生缓存未命中（cache miss）时：在原始模式下，netfs 将从远程服务器获取数据，然后将其写入缓存文件；在按需读取模式下，获取数据并将其写入缓存被委托给用户守护进程
应启`CONFIG_CACHEFILES_ONDEMAND` 以支持按需读取模式

### 协议通信


按需读取模式使用一个简单的协议用于 kernel 与用户守护进程之间的通信

```
	kernel --[request]--> user daemon --[reply]--> kernel

```
CacheFiles 会在需要时向用户守护进程发送请求。用户守护进程应轮询 devnode/dev/cachefiles'）以检查是否有待处理的请求需要处理。当有挂起的请求时，将返POLLIN 事件
然后用户守护进程读取 devnode 以获取要处理的请求。需注意，每次读取只获取一个请求。当它完成请求的处理后，用户守护进程应将回复写入 devnode
```
	struct cachefiles_msg {
		__u32 msg_id;
		__u32 opcode;
		__u32 len;
		__u32 object_id;
		__u8  data[];
	};

```
其中
 - `msg_id` 是在所有挂起请求中标识此请求的唯一 ID
 - `opcode` 指示此请求的类型
 - `object_id` 是标识所操作的缓存文件的唯一 ID
 - `data` 指示此请求的载荷
 - `len` 指示此请求的整个长度，包括头部和随后的类型相关载荷

### 开启按需模式


```
	bind [ondemand]

```
bind"命令不带参数时，默认为原始模式。当给定"ondemand"参数时，bind ondemand"，将启用按需读取模式

### OPEN 请求


netfs 首次打开缓存文件时，将发送一个带CACHEFILES_OP_OPEN 操作码（OPEN 请求）的请求给用
```
	struct cachefiles_open {
		__u32 volume_key_size;
		__u32 cookie_key_size;
		__u32 fd;
		__u32 flags;
		__u8  data[];
	};

```
其中
 - `data` 包含紧随其后volume_key cookie_key	  volume key 是一个以 NUL 结尾的字符串；cookie key 是二进制数据
 - `volume_key_size` 指示 volume key 的大小（字节）
 - `cookie_key_size` 指示 cookie key 的大小（字节）
 - `fd` 指示一个引用该缓存文件的匿fd，用户守护进程可借此对缓存文件执write/llseek 文件操作

用户守护进程可以使用给定(volume_key, cookie_key) 对来区分所请求的缓存文件。借助给定的匿fd，用户守护进程可以在后台获取数据并将其写入缓存文件，即使 kernel 尚未触发缓存未命中
需注意，每个缓存文件都有唯一object_id，但可能有多个匿fd。用户守护进程可以通过 dup() @fd 字段指示的初始匿fd 复制匿名 fd。因此每object_id 可以映射到多个匿fd，而用户守护进程自身需要维护该映射
在实现用户守护进程时，请注意 RLIMIT_NOFILE、`/proc/sys/fs/nr_open` `/proc/sys/fs/file-max`。通常这些不需要很大，因为它们与打开的设blob 数量相关，而非每个独立文件系统的打开文件数
用户守护进程应通过发起一copen"（complete

```
	copen <msg_id>,<cache_size>

```
其中
 - `msg_id` 必须OPEN 请求msg_id 字段匹配
 - >= 0 时，`cache_size` 指示缓存文件的大小；
	  < 0 时，`cache_size` 指示用户守护进程遇到的任何错误码

### CLOSE 请求


cookie 被撤销（withdrawn）时，将向用户守护进程发送一CLOSE 请求（操作码 CACHEFILES_OP_CLOSE）。这告诉用户守护进程关闭与给object_id 关联的所有匿fd。CLOSE 请求没有额外载荷，不应被回复

### READ 请求


在按需读取模式下遇到缓存未命中时，CacheFiles 将向用户守护进程发送一READ 请求（操作码 CACHEFILES_OP_READ）。这告诉用户守护进程获取所请求文件范围的内容。载荷为

```
	struct cachefiles_read {
		__u64 off;
		__u64 len;
	};

```
其中
 - `off` 指示所请求文件范围的起始偏移
 - `len` 指示所请求文件范围的长度

当用户守护进程收READ 请求时，应获取所请求的数据并将其写入object_id 标识的缓存文件
当用户守护进程完READ 请求的处理后，应通过READ 请求中给object_id 关联的某个匿fd 使用 CACHEFILES_IOC_READ_COMPLETE ioctl 来回复。该 ioctl 
```
	ioctl(fd, CACHEFILES_IOC_READ_COMPLETE, msg_id);

```
其中
 - `fd` 是与给定 object_id 关联的匿fd 之一
 - `msg_id` 必须READ 请求msg_id 字段匹配