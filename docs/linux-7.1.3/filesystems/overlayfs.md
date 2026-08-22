
作者：Neil Brown
发送问题请MAINTAINERS 文件
## 叠加文件系统（Overlay Filesystem
本文档描述了Linux 中提供叠加文件系统（overlay-filesystem）功能的一种新方法的原型（有时称为联合文件系统 union-filesystems）。叠加文件系统尝试呈现一个文件系统，它是将一个文件系统叠加在另一个文件系统之上的结果
### 叠加对象

叠加文件系统的方法是"混合"的，因为出现在文件系统中的对象并不总是看起来属于该文件系统。在许多情况下，在联合（union）中访问的对象与从原始文件系统访问相应对象无法区分。这一点从 stat(2) 返回'st_dev' 字段最为明显
虽然目录会报告来自叠加文件系统的 st_dev，但非目录对象可能报告提供该对象的下层文件系统或上层文件系统st_dev。类似地，st_ino 只有st_dev 结合时才唯一，并且这两者都可能在非目录对象的生命周期内发生变化。许多应用程序和工具会忽略这些值，不会受到影响
在一种特殊情况下，即所有叠加层都在同一个底层文件系统上时，所有对象都会报告来自叠加文件系统的 st_dev 和来自底层文件系统的 st_ino。这将使叠加挂载更符合文件系统扫描器的要求，并且叠加对象将能与原始文件系统中的相应对象区分开来
64 位系统上，即使并非所有叠加层都在同一个底层文件系统上，也可以通过 "xino" 特性获得相同的合规行为xino" 特性由真实对象st_ino 和一个底fsid 编号组合成一个唯一的对象标识符xino" 特性使inode 号的高位作为 fsid，因为底层文件系统很少使inode 号的高位。如果底inode 号溢出进入高位的 xino 位，叠加文件系统将对inode 回退到非 xino 行为
"xino" 特性可以通过 "-o xino=on" 叠加挂载选项启用。如果所有底层文件系统都支持 NFS 文件句柄，则叠加文件系统对象st_ino 值不仅是唯一的，而且在文件系统生命周期内是持久的-o xino=auto" 叠加挂载选项仅在满足持久 st_ino 要求时才启用 "xino" 特性
下表总结了在不同叠加配置下可以预期的行为
inode 属````````````````
+--------------+------------+------------+-----------------+----------------+
|Configuration | Persistent | Uniform    | st_ino == d_ino | d_ino == i_ino |
|              | st_ino     | st_dev     |                 | [*]            |
+==============+=====+======+=====+======+========+========+========+=======+
|              | dir | !dir | dir | !dir |  dir   |  !dir  |  dir   | !dir  |
+--------------+-----+------+-----+------+--------+--------+--------+-------+
| All layers   |  Y  |  Y   |  Y  |  Y   |  Y     |   Y    |  Y     |  Y    |
| on same fs   |     |      |     |      |        |        |        |       |
+--------------+-----+------+-----+------+--------+--------+--------+-------+
| Layers not   |  N  |  N   |  Y  |  N   |  N     |   Y    |  N     |  Y    |
| on same fs,  |     |      |     |      |        |        |        |       |
| xino=off     |     |      |     |      |        |        |        |       |
+--------------+-----+------+-----+------+--------+--------+--------+-------+
| xino=on/auto |  Y  |  Y   |  Y  |  Y   |  Y     |   Y    |  Y     |  Y    |
+--------------+-----+------+-----+------+--------+--------+--------+-------+
| xino=on/auto,|  N  |  N   |  Y  |  N   |  N     |   Y    |  N     |  Y    |
| ino overflow |     |      |     |      |        |        |        |       |
+--------------+-----+------+-----+------+--------+--------+--------+-------+

[*] nfsd v3 readdirplus 会校d_ino == i_ino。i_ino 通过若干
/proc 文件暴露，例/proc/locks 以及 inotify
文件描述符的 /proc/self/fdinfo/<fd>
### 上层与下
叠加文件系统组合了两个文件系统——一上层"文件系统和一下层"文件系统。当一个名称同时存在于两个文件系统中时上层"文件系统中的对象可见，下层"文件系统中的对象被隐藏，或者在目录的情况下上层"对象合并
更正确的做法是称之为上层和下目录而不文件系统"，因为两个目录树完全可能位于同一个文件系统中，并且对上层或下层的根都没有要求必须是文件系统的根
Linux 支持的各种文件系统都可以是下层文件系统，但并非所Linux 可挂载的文件系统都具OverlayFS 工作所需的特性。下层文件系统不需要可写。下层文件系统甚至可以是另一overlayfs。上层文件系统通常是可写的，如果是，则必须支持创建 trusted.** user.** 扩展属性，并且必须readdir 响应中提供有效的 d_type，因NFS 不适用
由两台只读文件系统组成的只读叠加可以使用任何文件系统类型
### 目录

叠加主要涉及目录。如果给定名称同时出现在上层和下层文件系统中，并且在任一者中都指向非目录，则下层对象被隐藏——该名称仅指向上层对象
当上层和下层对象都是目录时，会形成一个合并目录
在挂载时，作为挂载选项 "lowerdir" "upperdir" 给出的两个目录：

```
  mount -t overlay overlay -olowerdir=/lower,upperdir=/upper,\
  workdir=/work /merged

```
"workdir" 需要是upperdir 位于同一文件系统上的空目录
然后，每当在此类合并目录中请求查找时，会在每个实际目录中执行查找，并将组合结果缓存在属于叠加文件系统dentry 中。如果两个实际查找都找到目录，则两者都被存储并创建一个合并目录，否则只存储一个：如果存在上层则存储上层，否则存储下层
只合并目录中的名称列表。其他内容（如元数据和扩展属性）仅针对上层目录报告。下层目录的这些属性被隐藏
### 白化（whiteouts）与不透明目录

为了支持 rm rmdir 而不修改下层文件系统，叠加文件系统需要在上层文件系统中记录文件已被删除。这是通过使用白化和不透明目录（非目录始终opaue 的）来完成的
白化被创建为设备号为 0/0 的字符设备，或者被创建为带xattr "trusted.overlay.whiteout" 的零长度普通文件
当在合并目录的上层发现白化时，下层中任何匹配的名称都会被忽略，并且白化本身也被隐藏
通过xattr "trusted.overlay.opaque" 设置"y" 来使目录不透明。当上层文件系统包含不透明目录时，下层文件系统中同名的任何目录都会被忽略
不透明目录不应包含任何白化，因为它们没有任何作用。包含带xattr "trusted.overlay.whiteout" 的普通文件的合并目录，应额外通过在合并目录本身上设置 xattr "trusted.overlay.opaque" "x" 来标记。这是为了避免在常见情况readdir 期间检查所有条目的 "trusted.overlay.whiteout" 所带来的开销
### readdir

当对合并目录发出 'readdir' 请求时，会分别读取上层和下层目录，并以明显的方式合并名称列表（先读上层，再读下层——已存在的条目不会被重新添加）。这个合并后的名称列表被缓存'struct file' 中，因此只要文件保持打开就会一直存在。如果目录被两个进程同时打开并读取，它们各自会有独立的缓存。将 seekdir 到目录开头（偏移0）然后再进行 readdir 将导致缓存被丢弃并重建
这意味着对合并目录的更改在目录被读取期间不会显现。大多数程序不太会注意到这一点
当读取目录时，seek 偏移量是顺序分配的。因此，如果
 - 读取目录的一部分
 - 记住一个偏移量，并关闭目录
 - 稍后重新打开目录
 - 定位到记住的偏移
那么在文件名列表中，旧位置和新位置之间可能几乎没有关联，特别是在目录中有任何变化的情况下
对非合并目录Readdir 直接由底层目录（上层或下层）处理
### 重命名目
当重命名位于下层或合并的目录时（即该目录最初并不是在上层创建的），overlayfs 可以用两种不同的方式处理
1. 返回 EXDEV 错误：当尝试跨文件系统边界移动文件或目录时，rename(2) 会返回此错误。因此应用程序通常已准备好处理此错误（例如 mv(1) 会递归复制目录树）。这是默认行为
2. 如果启用"redirect_dir" 特性，则目录会被复制上来（但内容不会）。然后将 "trusted.overlay.redirect" 扩展属性设置为从叠加根到原始位置的路径。最后该目录被移动到新位置
有几种方法可以调"redirect_dir" 特性
内核配置选项
- OVERLAY_FS_REDIRECT_DIR    如果启用，则 redirect_dir 默认开启- OVERLAY_FS_REDIRECT_ALWAYS_FOLLOW    如果启用，则默认始终遵循重定向。启    这会导致安全性较低的配置。仅当担心与具有
    redirect_dir 特性并且即使关闭也会遵循重定向的内核的
    向后兼容性时，才启用此选项
模块选项（也可以通过 /sys/module/overlay/parameters/ 更改）：

- "redirect_dir=BOOL"    参见上面OVERLAY_FS_REDIRECT_DIR 内核配置选项- "redirect_always_follow=BOOL"    参见上面OVERLAY_FS_REDIRECT_ALWAYS_FOLLOW 内核配置选项- "redirect_max=NUM"    绝对重定向中的最大字节数（默认是 256）
挂载选项
- "redirect_dir=on"    启用重定向- "redirect_dir=follow"    不创建重定向，但遵循它们- "redirect_dir=nofollow"    不创建也不遵循重定向- "redirect_dir=off"    如果内核/模块配置中启用了 "redirect_always_follow"    则此 "off" 转换"follow"，否则转换为 "nofollow"
当启NFS 导出特性时，每个被复制上来的目录都由下inode 的文件句柄索引，并且上层目录的文件句柄被存储在索引条目上"trusted.overlay.upper" 扩展属性中。在查找合并目录时，如果上层目录与索引中存储的文件句柄不匹配，则表明可能有多个上层目录被重定向到同一个下层目录。在这种情况下，查找会返回错误并警告可能存在的不一致
因为下层重定向无法通过索引验证，所以在没有上层层的叠加文件系统上启NFS 导出支持需要关闭重定向遵循（例"redirect_dir=nofollow"）
### 闈炵洰褰。
非目录对象（文件、符号链接、设备特殊文件等）会适当地来自上层或下层文件系统。当以需要写访问的方式访问下层文件系统中的文件时，例如为写访问而打开、更改某些元数据等，该文件会首先从下层文件系统复制到上层文件系统（copy_up）。注意，创建硬链接也需copy_up，当然创建符号链接则不需要
copy_up 可能被证明是不必要的，例如，如果文件以读写方式打开但数据未被修改
copy_up 过程首先确保包含目录在上层文件系统中存在——必要时创建它及其任何父目录。然后它用相同的元数据（所有者、mode、mtime、符号链接目标等）创建对象，然后如果该对象是文件，则将数据从下层复制到上层文件系统。最后复制任何扩展属性
一copy_up 完成，叠加文件系统就简单地提供对新创建的上层文件的文件的直接访问——对该文件的后续操作几乎不会被叠加文件系统注意到（当然，对文件名称的操作（如重命名或解除链接）会被注意到并处理）
### 权限模型

叠加文件系统会存放（stash）用于在访问下层或上层文件系统时使用的凭证
在旧的挂API 中，存放的是调用 mount(2) 的任务的凭证。在新的挂载 API 中，存放的是通过 fsconfig(2) FSCONFIG_CMD_CREATE 命令创建超级块的任务的凭证
从内v6.15 开始，可以使用 "override_creds" 挂载选项，它将记录调用任务的凭证。注意，"override_creds" 仅在与新挂载 API 一起使用时才有意义，因为旧挂载 API 在单mount(2) 系统调用中结合了设置选项和超级块创建
叠加文件系统中的权限检查遵循以下原则：

 1) 权限检查在 copy up 前后应返回相同的结果

 2) 创建叠加挂载的任务不得获得额外的特权

 3) 与直接访问底层下层或上层文件系统相比，任务[*]可以通过叠加获得额外的特
这通过对每次访问执行两次权限检查来实现
 a) 检查当前任务是否基于本DAC（所有者、组、mode posix acl）以MAC 检查被允许访问

 b) 检查存放的凭证是否基于底层文件系统权限（同样包MAC 检查）被允许在下层或上层执行实际操
检(a) 确保了一致(1)，因为所有者、组、mode posix acl 都被复制上来了。另一方面，它可能导致服务器强制的权限（例NFS 使用的）被忽(3)
检(b) 确保没有任务获得存放凭证所没有的底层层的权(2)。这也意味着有可能创建一致性规(1) 不成立的情况；然而，通常情况下，存放的凭证将有足够的特权来执行所有操作
```
  mount -t overlay overlay -olowerdir=/lower,upperdir=/upper,... /merged

```
```
  cp -a /lower /upper
  mount --bind /upper /merged

```
产生的访问权限应该是相同的。区别在于复制的时间（按需复制与预先复制）
### 多个下层

现在可以使用冒号:"）给出多个下层层
```
  mount -t overlay overlay -olowerdir=/lower1:/lower2:/lower3 /merged

```
如示例所示，"upperdir=" "workdir=" 可以省略。在这种情况下，叠加将是只读的
指定的下层目录将从最右边的目录开始向左堆叠。在上面的示例中，lower1 是顶层，lower2 是中间层，lower3 是底层
注意：包含冒号的目录名可以通过以下方式作为下层层提供：

```
  mount -t overlay overlay -olowerdir=/a\:lower\:\:dir /merged

```
从内核版v6.8 开始，也可以使"lowerdir+" 挂载选项fsconfig 将包含冒号的目录名配置为下层层：

```
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "lowerdir+", "/a:lower::dir", 0);

```
在后一种情况下，下层目录名中的冒号/proc/self/mountinfo 中显示时将被转义为八进制字符（\072）
### 仅元数据复制上来（Metadata only copy up
当启"metacopy" 特性时，overlayfs 将只复制元数据（而不是整个文件），当执行 chown/chmod 等特定于元数据的操作时。处于此状态的上层文件"trusted.overlayfs.metacopy" xattr 标记，表示该上层文件不包含数据。数据将在文件为 WRITE 操作打开时稍后复制上来。在下层文件的数据被复制上来后，"trusted.overlayfs.metacopy" xattr 会从上层文件中移除
换句话说，这是延迟的数据复制上来操作，当确实需要修改数据时才会复制数据
有多种方法可以启禁用此特性。可以通过设置/取消 CONFIG_OVERLAY_FS_METACOPY 配置选项来默认启禁用此特性。或者可以在模块加载时使用模块参metacopy=on/off 启用/禁用它。最后，还有一个每挂载选项 metacopy=on/off 来按挂载启用/禁用此特性
不要对不受信任的上层/下层目录使用 metacopy=on。否则，攻击者可能会创建带有适当 REDIRECT METACOPY xattr 的精心构造的文件，并获得REDIRECT 指向的下层文件的访问权限。在本地系统上这不应该发生，因为设置 "trusted." xattr 需CAP_SYS_ADMIN。但对于不受信任的层（如来自 U 盘）应该是可能的
注意：redirect_dir={off|nofollow|follow[*]} nfs_export=on 挂载选项metacopy=on 冲突，并会导致错误
[*] 仅当给定 upperdir=... 时，redirect_dir=follow 才与 metacopy=on 冲突
### 仅数据下层（Data-only lower layers
启用 "metacopy" 特性后，overlayfs 普通文件可能是来自最多三个不同层的信息的组合
 1) 来自上层文件中元数据的元数据

 2) 来自下层文件st_ino st_dev 对象标识
 3) 来自另一个下层（更下方）文件中数
"lower data" 文件可以位于任何下层，但最顶层的下层除外
在最高下层之下，任意数量的底层可以定义为 **作为 "data-only" 下层，使用双冒号:"** : "）分隔符。普通下层不允许位于 data-only 层之下，因此单个 **冒号分隔符不允许出现在双冒号:"** : "）分隔符的右侧
```
  mount -t overlay overlay -olowerdir=/l1:/l2:/l3::/do1::/do2 /merged

```
"data-only" 下层中文件的路径在合并后overlayfs 目录中不可见，并"data-only" 下层中文件的元数据和 st_ino/st_dev overlayfs inode 中不可见
只有当其上方某个下层中的 "metacopy" 文件具有指向 "data-only" 下层"lower data" 文件的绝对路径的 "redirect" 时，"data-only" 下层中文件的数据才可能可见
无需显式启用 "metacopy=on"，只需指定至少一data-only 层即可启用向 data-only 层的数据重定向。在这种情况下，其他形式metacopy 会被拒绝。注意：这样，data-only 层可以与 "userxattr" 一起使用，在这种情况下必须仔细注意更改 "user.overlay.redirect" xattr 所需的特权以防止滥用
从内核版v6.8 开始，"data-only" 下层也可以使"datadir+" 挂载选项以及新挂API fsconfig 系统调用来添加
```
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "lowerdir+", "/l1", 0);
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "lowerdir+", "/l2", 0);
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "lowerdir+", "/l3", 0);
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "datadir+", "/do1", 0);
  fsconfig(fs_fd, FSCONFIG_SET_STRING, "datadir+", "/do2", 0);

```
### 通过文件描述符指定层

从内v6.13 开始，除了以路径指定层之外，overlayfs 还支持通过文件描述符指定层。此特性适用"datadir+"lowerdir+"upperdir" "workdir+" 挂载选项，配
```
  fsconfig(fs_fd, FSCONFIG_SET_FD, "lowerdir+", NULL, fd_lower1);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "lowerdir+", NULL, fd_lower2);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "lowerdir+", NULL, fd_lower3);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "datadir+", NULL, fd_data1);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "datadir+", NULL, fd_data2);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "workdir", NULL, fd_work);
  fsconfig(fs_fd, FSCONFIG_SET_FD, "upperdir", NULL, fd_upper);

```
### fs-verity 支持

在下层文件的元数据复制上来期间，如果源文件启用了 fs-verity 且叠verity 支持已启用，则下层文件的摘要会被添加"trusted.overlay.metacopy" xattr 中。随后每次打开 metacopy 文件时都会用它来验证下层文件的内容
当使用包verity xattr 的层时，这意味着上层中的任何此类 metacopy 文件都保证与复制上来时下层中的内容相匹配。如果在任何时候（挂载期间、重新挂载后等）下层中的此类文件被替换或以任何方式修改，overlayfs 中相应文件的访问将导EIO 错误（在打开时由overlayfs 摘要检查，或在稍后读取时由fs-verity），并且详细的错误会被打印到内核日志中。有fs-verity 文件访问如何工作的更多详细信息，请参:ref:`Documentation/filesystems/fsverity.rst
<accessing_verity_files>`銆。
Verity 可用作一般的健壮性检查，以检测在使用中的 overlayfs 目录中的意外更改。但是，借助额外的注意，它也可以提供更有力的保证。例如，如果上层完全受信任（通过使用 dm-verity 或类似的东西），那么不受信任的下层可用于为所metacopy 文件提供经过验证的文件内容。如果此外不受信任的下层目录被指定为 "Data-only"，那么它们只能提供此类文件内容，并且整个挂载可以被信任为与上层匹配
此特性由 "verity" 挂载选项控制，它支持以下值：

- "off"    metacopy 摘要永远不会生成或使用。如果未指定 verity 选项，这是默认值- "on"    每当 metacopy 文件指定了期望的摘要，相应的数据文件必须匹配指定的摘要。在生成 metacopy 文件时，会基于源文件（如果它有）在其中设verity 摘要- "require"    "on" 相同，但此外所metacopy 文件必须指定摘要（否则在打开时返EIO）。这意味着仅当数据文件启用fs-verity 时才会使用元数据复制上来，否则使用完整复制上来
### 共享与复制层

下层层可以在多个叠加挂载之间共享，这确实是一种非常常见的做法。一个叠加挂载可以使用与另一个叠加挂载相同的下层路径，并且它可以使用位于另一个叠加下层路径之下或之上的下层路径
使用已被另一个叠加挂载使用的上层路径workdir 路径是不允许的，并且可能EBUSY 失败。使用部分重叠的路径是不允许的，并且可能EBUSY 失败。如果从共享或重叠上层和/workdir 路径的两overlayfs 挂载访问文件，叠加的行为是未定义的，尽管不会导致崩溃或死锁
使用上层路径挂载叠加（该上层路径之前曾被另一个已挂载叠加与不同的下层路径组合使用）是允许的，除非启用"index" "metacopy" 特性
对于 "index" 特性，在首次挂载时，下层根目录NFS 文件句柄以及下层文件系统UUID 会被编码并存储在上层根目录的 "trusted.overlay.origin" 扩展属性中。在后续挂载尝试时，会将下层根目录文件句柄和下层文件系统 UUID 与上层根目录中存储的 origin 进行比较。如果验证下层根 origin 失败，挂载将ESTALE 失败。启用了 "index" overlayfs 挂载在下层文件系统不支持 NFS 导出、下层文件系统没有有UUID 或上层文件系统不支持扩展属性时，将EOPNOTSUPP 失败
对于 "metacopy" 特性，在挂载时没有验证机制。因此，如果以不同的下层集合挂载相同的上层，挂载可能会成功，但之后要做好出现意外的准备。所以不要这样做
将叠加层复制到相同或不同的底层文件系统上的不同目录树，甚至复制到不同的机器，是一种相当常见的做法。对"index" 特性，尝试挂载复制的层将无法通过下层根文件句柄的验证
### 嵌套 overlayfs 挂载

可以使用存储overlayfs 挂载上的下层目录。对于普通文件，这不需要任何特殊注意。但是，具有 overlayfs 属性（如白化或 "overlay.*" xattr）的文件会被底层 overlayfs 挂载解释并剥离。为了让第二overlayfs 挂载看到这些属性，必须对它们进行转义
Overlayfs 特定xattr 通过使用 "overlay.overlay." 的特殊前缀进行转义。因此，下层目录中带"trusted.overlay.overlay.metacopy" xattr 的文件将overlayfs 挂载中作为带"trusted.overlay.metacopy" xattr 的普通文件暴露。这可以通过重复前缀来嵌套，因为每个实例只移除一个前缀
带有普通白化的下层目录总是overlayfs 挂载处理，因此为了支持在 overlayfs 挂载中存储有效的白化文件，支持一种替代形式的白化。这种形式是一个普通的、零长度的文件，在其所在的目录中设置了 "overlay.whiteout" xattr，该目录设置"overlay.opaque" xattr "x"（参`whiteouts and opaque directories`_）。这些替代白化从不被 overlayfs 创建，但可以被生成下层的用户空间工具（如容器）使用。这些替代白化可以使用标准的 xattr 转义机制进行转义，以便正确嵌套到任意深度
### 非标准行
当前版本overlayfs 可以充当基本符合 POSIX 的文件系统
以下overlayfs 当前不处理的情况列表
 a) POSIX 要求为读取更st_atime。当前在文件位于下层的情况下不会这样做
 b) 如果位于下层的文件以只读方式打开，然后用 MAP_SHARED 进行内存映射，则对该文件的后续更改不会反映在内存映射中
 c) 如果位于下层的文件正在被执行，则为写而打开该文件或截断该文件不会以 ETXTBSY 拒绝
以下选项允许 overlayfs 表现得更像符合标准的文件系统
redirect_dir
````````````
通过挂载选项或模块选项 "redirect_dir=on" 或内核配置选项 CONFIG_OVERLAY_FS_REDIRECT_DIR=y 启用
如果禁用此特性，则对下层或合并目录的 rename(2) 将以 EXDEVInvalid cross-device link"）失败
index
`````
通过挂载选项或模块选项 "index=on" 或内核配置选项 CONFIG_OVERLAY_FS_INDEX=y 启用
如果禁用此特性，并且具有多个硬链接的文件被复制上来，则将"断开"该链接。更改不会传播到引用同一 inode 的其他名称
xino
````
通过挂载选项 "xino=auto" "xino=on"、模块选项 "xino_auto=on" 或内核配置选项 CONFIG_OVERLAY_FS_XINO_AUTO=y 启用。当组成叠加的所有层使用相同的底层文件系统时也会隐式启用
如果禁用此特性，或者底层文件系统的 inode 号中没有足够的空闲位，则 overlayfs 将无法保stat(2) 返回st_ino st_dev 的值以readdir(3) 返回d_ino 的值会像在普通文件系统上一样表现。例如，同一叠加文件系统中两个对象的 st_dev 值可能不同，并且文件系统对象st_ino 值可能不是持久的，甚至可能在叠加文件系统处于挂载状态时发生变化，如上面`Inode properties`_ 表所总结
### 对底层文件系统的更改

作为已挂载叠加文件系统一部分的底层文件系统的更改是不允许的。如果底层文件系统被更改，叠加的行为是未定义的，尽管不会导致崩溃或死锁
当叠加未挂载时，离线更改允许用于上层树。仅"metacopy"index"xino" "redirect_dir" 特性未被使用时，才允许对下层树进行离线更改。如果修改了下层树并且使用了其中任何特性，叠加的行为是未定义的，尽管不会导致崩溃或死锁
当叠NFS 导出特性启用时，叠加文件系统对底层下层的离线更改行为与禁用 NFS 导出时的行为不同
在每copy_up 时，下层 inode NFS 文件句柄以及下层文件系统UUID 会被编码并存储在扩展属"trusted.overlay.origin" 中的上层 inode 上
当启NFS 导出特性时，对合并目录的查找，如果在查找路径或 "trusted.overlay.redirect" 扩展属性指向的路径处找到下层目录，将验证找到的下层目录文件句柄和下层文件系UUID 是否copy_up 时存储的 origin 文件句柄匹配。如果找到的下层目录与存储的 origin 不匹配，该目录将不会与上层目录合并
### NFS 导出

当下层文件系统支NFS 导出并且启用"nfs_export" 特性时，叠加文件系统可以导出到 NFS
对于 "nfs_export" 特性，在任何下层对象的 copy_up 时，会在索引目录下创建一个索引条目。索引条目名称是 copy up origin 文件句柄的十六进制表示。对于非目录对象，索引条目是到上inode 的硬链接。对于目录对象，索引条目具有一个扩展属"trusted.overlay.upper"，其中包含上层目inode 的编码文件句柄
从叠加文件系统对象编码文件句柄时，适用以下规则
 1. 对于非上层对象，从下inode 编码下层文件句柄
 2. 对于已索引对象，copy_up origin 编码下层文件句柄
 3. 对于纯上层对象以及现有的非索引上层对象，从上inode 编码上层文件句柄

编码后的叠加文件句柄包括
 - 包含路径类型信息（如下层/上层）的头部
 - 底层文件系统UUID
 - 底层 inode 的底层文件系统编
此编码格式与存储在扩展属"trusted.overlay.origin" 中的文件句柄的编码格式相同
解码叠加文件句柄时，遵循以下步骤
 1. 通过 UUID 和路径类型信息找到底层层 2. 将底层文件系统文件句柄解码为底层 dentry 3. 对于下层文件句柄，按名称在索引目录中查找该句柄 4. 如果在索引中找到白化，则返回 ESTALE。这表示一个在其文件句柄被编码后被删除的叠加对象 5. 对于非目录，从解码的底层 dentry、路径类型和索引 inode（如果找到）实例化一个断开连接的叠dentry 6. 对于目录，使用连接的底层解码 dentry、路径类型和索引来查找一个连接的叠加 dentry
解码非目录文件句柄可能返回一个断开连接dentry。该断开连接 dentry copy_up 将创建一个没有上层别名的上层索引条目
当叠加文件系统具有多个下层时，中间层目录可能具有到下层目录的 "redirect"。因为中间层 "redirect" 未被索引，所以从 "redirect" origin 目录编码的下层文件句柄不能用于查找中间层或上层目录。类似地，从 "redirect" origin 目录的后代编码的下层文件句柄不能用于重建连接的叠加路径。为了缓解无法从下层文件句柄解码的目录情况，这些目录在编码时会被复制上来并作为上层文件句柄编码。在没有上层的叠加文件系统上无法使用此缓解措施，此设置中NFS 导出需要关闭重定向遵循（例"redirect_dir=nofollow"）
叠加文件系统不支持非目录可连接文件句柄，因此使用 'subtree_check' exportfs 配置导出将导致通过 NFS 查找文件失败
当启NFS 导出特性时，所有目录索引条目都会在挂载时验证，以检查上层文件句柄是否过时。在某些情况此验证可能导致显著的开销
注意：挂载选项 index=off,nfs_export=on 对于读写挂载是冲突的，并会导致错误
注意：挂载选项 uuid=off 可用于将文件句柄中底层文件系统的 UUID 替换null，以放宽 UUID 检查。这在底层磁盘被复制并且此副本的 UUID 被更改的情况下很有用。这仅适用于所有下层目录都在同一文件系统上的情况，否则将回退到正常行为
### UUID 涓?fsid

overlayfs 实例本身UUID 以及 statfs(2) 报告fsid "uuid" 挂载选项控制，它支持以下值：

- "null"    overlayfs UUID null。fsid 取自最上层文件系统- "off"    overlayfs UUID null。fsid 取自最上层文件系统    底层层的 UUID 被忽略，改用 null- "on"    生成 overlayfs UUID 并用于报告唯一fsid    UUID 存储xattr "trusted.overlay.uuid" 中，overlayfs fsid
    唯一且持久。此选项需要支xattr 的上层文件系统- "auto"：（默认    如果存在，则xattr "trusted.overlay.uuid" 获取 UUID    对于满足先决条件的全overlayfs 的首次挂载，升级"uuid=on"    对于从未"uuid=on" 挂载过的现有 overlayfs，降级为 "uuid=null"
### 持久性与复制上来

fsync(2) 系统调用确保文件的数据和元数据被安全地写入后备存储，这应当保证系统崩溃后信息的存在
如果没有 fsync(2) 调用，则不能保证系统崩溃后观察到的数据是旧数据还是新数据，但在实践中，崩溃后观察到的数据通常是旧数据或新数据，或两者的混合
overlayfs 文件首次被修改时，copy up 将在上层创建下层文件及其父目录的副本。由Linux 文件系统 API 在不显式 fsync(2) 调用的情况下不强制对存储更改施加任何特定顺序，因此在系统崩溃的情况下，上层文件最终可能根本没有数据（即全零），这将是一个不寻常的结果。为了避免这种体验，overlayfs 在使rename(2) link(2) 完成数据 copy up 之前，会对上层文件调fsync(2)，以copy up 成为"原子"的
默认情况下，overlayfs 不会在复制上来的目录或仅元数据的 copy up 上显式调fsync(2)，因此它不保证持久化用户的修改，除非用户调用 fsync(2)。copy up 期间fsync 仅保证如果在崩溃后观察到 copy up，观察到的数据不是来copy up 暂存区的零或中间值
在具有单个日志的传统本地文件系统（例ext4、xfs）上，文件上fsync 也会持久化父目录的更改，因为它们通常在同一事务中被修改，因此在数据 copy up 期间的元数据持久性实际上是免费的。Overlayfs 通过禁止网络文件系统作为上层来进一步限制风险
Overlayfs 可以调整为在存储到下层上层时偏好性能或持久性。这"fsync" 挂载选项控制，它支持以下值：

- "auto"：（默认    在完成数copy up 之前对上层文件调fsync(2)    不对目录或仅元数据的 copy up 进行显式 fsync(2)- "strict"    在完成任copy up 之前对上层文件及目录调用 fsync(2)- "volatile"：[*]
    偏好性能而非持久性（参见 `Volatile mount`_
[*] 挂载选项 "volatile" "fsync=volatile" 的别名
### 易失挂载（Volatile mount
这通过 "volatile" 挂载选项启用。易失挂载不能保证在崩溃后存活。强烈建议仅当写入叠加的数据可以毫不费力地重新创建时才使用易失挂载
使用 "volatile" 选项挂载的优点是省略了对上层文件系统的所有形式的 sync 调用
为了避免给出虚假的安全感，易失挂载的 syncfs（和 fsync）语义与 VFS 的其余部分略有不同。如果在发生易失挂载之后上层目录的文件系统上发生任何回写错误，所sync 函数都将返回错误。一旦达到此条件，文件系统将不会恢复，并且每次后续的 sync 调用都将返回错误，即使自上次 sync 调用以来上层目录没有经历新的错误
当以 "volatile" 选项挂载叠加时，会创建目"$workdir/work/incompat/volatile"。在下一次挂载时，叠加会检查此目录，如果存在则拒绝挂载。这是一个强烈的指示，表明用户应该丢弃上层和工作目录并创建新的。在极少数情况下，用户知道系统没有崩溃并且上层目录的内容完好无损，可以删"volatile" 目录
### 用户 xattr

"-o userxattr" 挂载选项强制 overlayfs 使用 "user.overlay." xattr 命名空间而不"trusted.overlay."。这对于 overlayfs 的免特权挂载很有用
### 测试套件

有一个最初由 David Howells 开发、目前由 Amir Goldstein 维护的测试套件，位于
https://github.com/amir73il/unionmount-testsuite.git

```
  # cd unionmount-testsuite
  # ./run --ov --verify

```
