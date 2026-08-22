
## uevents 涓?GFS2


GFS2 挂载的整个生命周期内，会生成若干 uevent。本文档说明了这些事件是什么以及它们的
用途（gfs2-utils 中的 gfs_controld 使用）
## GFS2 uevent 列表


### 1. ADD


ADD 事件发生在挂载时。它始终是新建文件系统生成的第一uevent。如果挂载成功，随后产生一ONLINE uevent；如果不成功，则随后会产生一REMOVE uevent
ADD uevent 带有两个环境变量：SPECTATOR=[0|1] RDONLY=[0|1]，分别指定了旁观者状（未分配日志的只读挂载）和只读（已分配日志）状态
### 2. ONLINE


ONLINE uevent 在挂载或重新挂载成功后生成。它ADD uevent 具有相同环境变量。ONLINE
uevent 连同用于旁观者和 RDONLY 的两个环境变量是相对较新的新增内容（2.6.32-rc+），
旧内核不会生成它们
### 3. CHANGE


CHANGE uevent 用于两处。其一是在第一个节点成功挂载文件系统时报告（FIRSTMOUNT=Done）gfs_controld 将其用作信号，表明集群中的其他节点此时可以挂载该文件系统
另一CHANGE uevent 用于通知某个文件系统日志恢复完成。它带有两个环境变量：JID= 指定
刚刚恢复的日id，以RECOVERY=[Done|Failed] 以指示操作是否成功。这uevent 会为
每一个被恢复的日志生成，无论是在初始挂载过程中，还是作为 gfs_controld 通过
/sys/fs/gfs2/<fsname>/lock_module/recovery 文件请求特定日志恢复的结果
由于（gfs_controld 的早期版本中）CHANGE uevent 在使用时并未检查环境变量来发现状态，
我们若再为其添加任何功能，就会冒有人使用旧版用户工具而导致其集群出错的风险。因此，
在新增用于挂载或重新挂载成功uevent 时，使用ONLINE uevent
### 4. OFFLINE


OFFLINE uevent 仅因文件系统错误而产生，并作为“withdraw”机制的一部分使用。目前它
并不提供关于错误是什么的任何信息，这一点有待修复
### 5. REMOVE


REMOVE uevent 在不成功挂载的末尾，或在文件系统 umount 的末尾生成。所REMOVE uevent
之前都至少已存在同一文件系统ADD uevent，并且与其他 uevent 不同，它是由内核kobject 子系统自动生成的

## 所GFS2 uevent 共有的信息（uevent 环境变量

### 1. LOCKTABLE=


LOCKTABLE 是一个字符串，由挂载命令行（locktable=）或 fstab 提供。它被用作文件系统标签，
同时也为 lock_dlm 挂载提供加入集群所需的信息
### 2. LOCKPROTO=


LOCKPROTO 是一个字符串，其值取决于挂载命令行或 fstab 中的设置。它将是 lock_nolock lock_dlm。未来可能会支持其他锁管理器
### 3. JOURNALID=


如果文件系统正在使用某个日志（旁观者挂载不会分配日志），则它会在所GFS2 uevent 中给该数值日id
### 4. UUID=


在较新版本的 gfs2-utils 中，mkfs.gfs2 会向文件系统超级块写入一UUID。如果它存在则会被包含在与该文件系统相关的每一uevent 中