## 用法

本模块支持 SMB3 系列高级网络协议（以及较旧的方言，最初称为 "CIFS" 或 SMB1）。

Linux 的 CIFS VFS 模块支持许多高级网络文件系统特性，例如类似分层 DFS 的命名空间、硬链接、锁定等。它被设计为符合 SNIA CIFS 技术参考（取代 1992 年的 X/Open SMB 标准），并与 Windows 2000、Windows XP、Samba 及等效服务器实现最佳实践的实际互操作。此代码是在协议自由信息基金会（Protocol Freedom Information Foundation）的参与下开发的。CIFS 以及现在的 SMB3 已成为 Mac 与 Windows 以及主要 NAS 设备之间互操作的既定标准。

更多详情请参见
MS-SMB2（SMB2/SMB3/SMB3.1.1 协议规范详情）
或 https://samba.org/samba/PFIF/ 。

如有问题或错误报告请联系：

    smfrench@gmail.com

项目页面见：https://wiki.samba.org/index.php/LinuxCIFS_utils

## 构建说明

对于 Linux：

1) 下载内核（例如从 https://www.kernel.org）
   并切换到内核目录树的顶层目录
   （例如 /usr/src/linux-2.5.73）
2) make menuconfig（或 make xconfig）
3) 在网络文件系统选项中选择 cifs
4) 保存并退出
5) make

## 安装说明

如果你已将 CIFS vfs 构建为模块（成功），只需键入 `make modules_install`（或者，手动将文件复制到模块目录，例如 /lib/modules/6.3.0-060300-generic/kernel/fs/smb/client/cifs.ko）。

如果你已将 CIFS vfs 构建进内核本身，请按照你的发行版关于如何安装新内核的说明操作（通常只需键入 `make install`）。

如果你没有 mount.cifs 工具（位于 Samba 4.x 源码树及 CIFS VFS 网站），请将其复制到挂载辅助程序所在的同一目录（通常是 /sbin）。虽然辅助软件并非必需，但推荐使用 mount.cifs。大多数发行版包含 `cifs-utils` 软件包，其中含有此工具，因此建议安装。

注意，在你的所有 Linux 客户端上运行 Winbind pam/nss 模块（登录服务）有助于在域中一致地将 Uid 和 Gid 映射到正确的网络用户。mount.cifs 挂载辅助程序可在 git.samba.org 的 cifs-utils.git 中找到。

如果 cifs 被构建为模块，则网络缓冲区的大小和数量以及对单台服务器的最大并发请求数都可以配置。

```
	modinfo <path to cifs.ko>

```
在 kernel/fs/smb/client/cifs.ko 上，可以看到可在模块初始化时（通过运行 insmod cifs.ko）进行的配置变更列表。

## 建议

为提高安全性，SMB2.1 方言或更高版本（通常将使用 SMB3.1.1）现在是新的默认值。要使用旧方言（例如挂载 Windows XP），请在挂载时使用 "vers=1.0"（或 vers=2.0 对应 Windows Vista）。注意 CIFS（vers=1.0）比默认方言 SMB3 更旧且安全性更低，SMB3 包含许多高级安全特性，例如降级攻击检测、加密共享以及更强的签名和认证算法。

还有一些额外的挂载选项可能有助于 SMB3 获得改进的 POSIX 行为（注意：可以使用 vers=3 强制使用 SMB3 或更高版本，绝不要用 2.1）：

   `mfsymlinks` 以及 `cifsacl` 或 `modefromsid`（通常与 `idsfromsid` 一起使用）

## 允许用户挂载

允许用户在他们拥有的目录上进行挂载和卸载，使用 cifs vfs 是可以实现的。启用此类挂载的一种方法是将 mount.cifs 工具标记为 suid（例如 `chmod +s /sbin/mount.cifs`）。要允许用户卸载他们所挂载的共享，需要：

1) mount.cifs 版本 1.4 或更高
2) /etc/fstab 中存在指示某用户可挂载该共享的条目

```
     //server/usersharename  /mnt/username cifs user 0 0

```
注意，当 mount.cifs 工具以 suid 方式运行（允许普通用户挂载）时，为降低风险，挂载时会传入 `nosuid` 挂载标志，以禁止执行挂载在远程目标上的 suid 程序。当以 root 身份执行挂载时，默认不会传入 nosuid，默认将启用远程目标上 suid 程序的执行。这可以改变，与 nfs 及其他文件系统一样，只需在挂载选项中指定 `nosuid` 即可。然而对于用户挂载，要能够向 mount 传递 suid 标志，需要使用以下标志重建 mount.cifs：CIFS_ALLOW_USR_SUID

在 Samba 3.0 及更高版本的源码树中的 docs/manpages/mount.cifs.8 有对应的 cifs 挂载手册页。

## 允许用户卸载

允许普通用户卸载他们以用户身份挂载的目录（见上文），可以使用 umount.cifs 工具。它可以直接调用，或者如果 umount.cifs 被放在 /sbin 中，umount 可以调用 cifs 卸载辅助程序（对于大多数版本的 umount 工具而言）来卸载 cifs 挂载，除非 umount 使用 -i 调用（这将避免调用卸载辅助程序）。与 mount.cifs 一样，要启用用户卸载，umount.cifs 必须被标记为 suid（例如 `chmod +s /sbin/umount.cifs`）或等效方式（某些发行版允许向 /etc/permissions 文件添加条目以实现等效的 suid 效果）。要使该工具成功，目标路径必须是 cifs 挂载，且当前用户的 uid 必须与挂载该资源的用户的 uid 匹配。

还需注意，允许普通用户挂载和卸载的常规方式（而不是将 mount.cifs 和 umount.cifs 用作 suid）是，为你希望挂载的每个 //server/share 向 /etc/fstab 文件添加一行，但当潜在挂载目标包含许多或不可预测的 UNC 名称时，这会变得难以管理。

## Samba 注意事项

大多数当前服务器支持更安全 SMB2.1 和 SMB3，但对于较旧且安全性较低的 CIFS 方言，有一些有用的协议扩展，因此若要使用旧方言（CIFS/SMB1）挂载以获得最大收益，我们建议使用支持 SNIA CIFS Unix 扩展标准的服务器（例如几乎任何版本的 Samba，即 2.2.5 或更高版本），但 CIFS vfs 可与各种各样的 CIFS 服务器良好协作。注意，如果你没有支持 CIFS Unix 扩展的服务器（如 Samba 2.2.5 或更高版本），uid、gid 和文件权限将显示默认值。要在 Samba 服务器上启用 Unix CIFS 扩展，请在服务器的 smb.conf 文件中添加

```
	unix extensions = yes

```
注意，当大多数客户端是 Unix 或以下设置时也很有用（在 Samba 服务器上）

```
	case sensitive = yes
	delete readonly = yes
	ea support = yes

```
注意，服务器 ea 支持是支持来自 Linux cifs 客户端的 xattrs 所必需的，且 EA 支持存在于更高版本的 Samba 中（例如 3.0.6 及更高版本，EA 支持在所有 Windows 版本中也有效，至少对 NTFS 文件系统上的共享有效）。扩展属性（xattr）支持是大多数 Linux 文件系统的可选特性，可能需要通过 make menuconfig 启用。客户端对扩展属性的支持（user xattr）可以通过在挂载时指定 `nouser_xattr` 按挂载禁用。

CIFS 客户端可以获取并设置 POSIX ACL（getfacl、setfacl）到 Samba 服务器版本 3.10 及更高版本。设置 POSIX ACL 需要在构建 cifs 模块时在 CIFS 配置选项中同时启用 XATTR 和 POSIX 支持。POSIX ACL 支持可以通过在挂载时指定 `noacl` 按挂载禁用。

一些管理员可能想要将 Samba 的 smb.conf `map archive` 和 `create mask` 参数从默认值更改。除非更改 create mask，否则新创建的文件最终可能具有不必要地严格的默认模式，这可能不是你想要的，尽管如果在服务器和客户端上启用了 CIFS Unix 扩展，后续的 setattr 调用（例如 chmod）可以修复该模式。注意，创建特殊设备（mknod）远程地可能需要向 Samba 指定一个 mkdev 函数，如果你没有使用 Samba 3.0.6 或更高版本。有关这些的更多信息请参见 Samba 服务器系统上的手册页（`man smb.conf`）。注意，cifs vfs 与 smbfs vfs 不同，它不读取客户端系统上的 smb.conf（少数可选设置通过 -o 参数在挂载时传入）。注意，Samba 2.2.7 或更高版本包含一项修复，允许 CIFS VFS 删除打开的文件（严格的 POSIX 合规所必需）。Windows 服务器已经支持此特性。Samba 服务器不允许指向共享之外文件的符号链接，因此在 3.0.6 之前的 Samba 版本中，大多数指向

```
	 ln -s /mnt/foo bar

```
的符号链接将被禁止。Samba 3.0.6 或更高版本的服务器包含通过将被不安全的符号链接（即指向服务器上共享之外文件的符号链接）转换为服务器上的特定 samba 格式来安全地创建此类符号链接的能力，该格式被本地服务器应用程序和非 cifs 客户端忽略，且不会被 Samba 服务器遍历。这对使用 cifs vfs 的 Linux 客户端应用程序是透明的。绝对符号链接在 Samba 3.0.5 或更高版本上可用，但仅适用于使用 CIFS Unix 扩展的远程客户端，并且对 Windows 客户端不可见，通常也不会影响与 Samba 运行在同一服务器上的本地应用程序。

## 使用说明

一旦 CIFS VFS 支持被构建进内核或作为模块（cifs.ko）安装，你可以使用类似以下的挂载语法来访问 Samba 或

```
  mount -t cifs //9.53.216.11/e$ /mnt -o username=myname,password=mypassword

```
在 -o 之前可以指定 -v 选项，以使 mount.cifs 挂载辅助程序更详细地显示挂载步骤。
在 -o 之后，以下常用的 cifs vfs 特定选项

```
  username=<username>
  password=<password>
  domain=<domain name>

```
下面描述了其他 cifs 挂载选项。如果安装了挂载辅助程序（mount.cifs），则可以使用 TCP 名称（除了 ip 地址）。如果你不信任所挂载到的服务器，或者你没有启用 cifs 签名（且物理网络不安全），请考虑使用标准挂载选项 `noexec` 和 `nosuid` 来降低在本地系统上运行被篡改的二进制文件（从恶意服务器下载或被恶意路由器篡改）的风险。

尽管使用对应 CIFS URL 规范的格式进行挂载在 mount.cifs 中还不可能，但可以使用服务器和共享名的替代格式（有点类似 NFS 风格挂载）

```
  mount -t cifs tcp_name_of_server:share_name /mnt -o user=myname,pass=mypasswd

```
当使用挂载辅助程序 mount.cifs 时，密码可以通过替代机制指定，而不是在命令行上 -o 之后使用正常的 `pass=` 语法指定：
1) 通过将其包含在凭证文件中。指定 credentials=filename 为一个

```
	username=someuser
	password=your_password

```
2) 通过在 PASSWD 环境变量中指定密码（类似地，用户名可以从 USER 环境变量获取）。
3) 通过 PASSWD_FILE 按名称在文件中指定密码
4) 通过 PASSWD_FD 按文件描述符在文件中指定密码

如果未提供密码，mount.cifs 将提示输入密码

## 限制

服务器必须支持 "pure-TCP"（端口 445 的 TCP/IP CIFS 连接）或用于 "Netbios-Over-TCP/IP" 的 RFC 1001/1002 支持。这通常不太可能成为问题，因为大多数服务器都支持。

有效的文件名在 Windows 和 Linux 之间有所不同。Windows 通常限制包含某些保留字符（例如字符 :，Windows 用它来分隔流名的开始）的文件名，而 Linux 允许稍宽的合法字符集。Windows 服务器可以在服务器的注册表中指定显式映射时重映射此类字符。从版本 3.10 开始的 Samba 将允许此类文件名（即包含合法 Linux 字符、通常对 Windows/CIFS 语义被禁止的文件名），只要服务器配置为 Unix 扩展（且客户端未禁用 /proc/fs/cifs/LinuxExtensionsEnabled）。此外，挂载选项 `mapposix` 可用于 CIFS（vers=1.0）以强制将非法的 Windows/NTFS/SMB 字符映射到重映射范围（此挂载参数是 SMB3 的默认值）。此重映射（`mapposix`）范围也与 Mac（以及某些较旧 Windows 上的 "Services for Mac"）兼容。当协商 SMB 3.1.1 的 POSIX 扩展时，重映射会自动禁用。

## CIFS VFS 挂载选项

以下是受支持挂载选项的部分列表：

  username
		尝试建立 CIFS 会话时使用的用户名。
  password
		用户密码。如果安装了挂载辅助程序，若未提供，将提示用户输入密码。
  ip
		目标服务器的 ip 地址
  unc
		要挂载的目标服务器通用网络名称（导出）。
  domain
		设置在建立 CIFS 会话时附加到用户名之前的 SMB/CIFS 工作组名称
  forceuid
		将 inode 的默认 uid 设置为挂载时传入的 uid。对于支持 CIFS Unix 扩展的服务器（例如正确配置的 Samba 服务器），服务器提供 uid、gid 和 mode，因此除非服务器与客户端的 uid 和 gid 编号不同，否则不应指定此参数。如果服务器和客户端在同一域中（例如运行 winbind 或 nss_ldap）且服务器支持 Unix 扩展，则可以从服务器检索 uid 和 gid（并且不必在挂载时指定 uid 和 gid）。对于不支持 CIFS Unix 扩展的服务器，查找现有文件时返回的默认 uid（和 gid）将是执行挂载的人的 uid（gid）（root，除非 mount.cifs 为用户挂载配置为 setuid），除非指定了 `uid=`（gid）挂载选项。另请注意，对文件访问的权限检查（授权检查）发生在服务器上，但在某些情况下，管理员可能也想在客户端加以限制。对于那些不报告 uid/gid 所有者的服务器（例如 Windows），也可以在客户端检查权限，并且可以通过在客户端指定 file_mode 和 dir_mode 来启用一种粗略的客户端侧权限检查。（默认）
  forcegid
		（类似于上面，但是针对组 id 而不是 uid）（默认）
  noforceuid
		如果可能，通过向服务器请求来填写文件所有者信息（uid）。使用此选项时，挂载时 `uid=` 选项中给出的值仅在服务器无法支持返回 inode 上的 uid 时使用。
  noforcegid
		（类似于上面，但是针对组所有者 gid 而不是 uid）
  uid
		设置 inode 的默认 uid，并指示 cifs 内核驱动是哪个本地用户挂载的。如果服务器支持 unix 扩展，默认的 uid 不用于填写 inode（文件）的所有者字段，除非指定了 `forceuid` 参数。
  gid
		设置 inode 的默认 gid（类似于上面）。
  file_mode
		如果服务器不支持 CIFS Unix 扩展，这将覆盖文件 inode 的默认模式。
  fsc
		使用 FS-Cache 启用本地磁盘缓存（默认关闭）。此选项可能有助于在慢速链路、负载很重的服务器和/或网络中提高性能，其中从磁盘读取比从服务器（通过网络）读取更快。由于对服务器的调用次数减少，这也可能对可扩展性产生积极影响。但是，本地缓存并不适合所有工作负载，例如只读一次类型的工作负载。因此，在使用此选项之前，你需要仔细考虑你的工作负载/场景。目前，本地磁盘缓存对以只读方式打开的 CIFS 文件是有效的。
  dir_mode
		如果服务器不支持 CIFS Unix 扩展，这将覆盖目录 inode 的默认模式。
  port
		在尝试通常的端口（端口 445，然后 139）之前，尝试在此 tcp 端口上联系服务器。
  iocharset
		用于将本地路径名与 Unicode 相互转换的代码页。如果服务器支持，网络路径名默认使用 Unicode。如果未指定 iocharset，则将使用本地客户端内核构建期间指定的 nls_default。如果服务器不支持 Unicode，此参数无用。
  rsize
		默认读取大小（通常为 16K）。客户端当前不能使用大于 CIFSMaxBufSize 的 rsize。CIFSMaxBufSize 默认为 16K，并可在模块安装时为 cifs.ko 更改（从 8K 到内核允许的最大 kmalloc 大小）。将 CIFSMaxBufSize 设置为非常大的值将导致 cifs 使用更多内存，并在某些情况下降低性能。要使用大于 127K（原始 cifs 协议最大值）的 rsize，还需要服务器支持一个新的 Unix 能力标志（用于非常大的读取），某些较新的服务器（例如 Samba 3.0.26 或更高版本）支持。rsize 可以设置为最小值 2048 到最大值 130048（127K 或 CIFSMaxBufSize，取较小者）。
  wsize
		默认写入大小（默认 57344）
		CIFS 当前允许的最大 wsize 为 57344（十四个 4096 字节页）
  actimeo=n
		属性缓存超时（秒）（默认 1 秒）。
		在此超时之后，cifs 客户端向服务器请求新的属性信息。此选项允许针对工作负载需要调整属性缓存超时。较短的超时意味着更好的缓存一致性，但增加了对服务器的调用次数。较长的超时意味着减少对服务器的调用次数，代价是较不严格的缓存一致性检查（即在短时间内属性缓存不正确）。
  rw
		以读写方式挂载网络共享（注意服务器可能仍视该共享为只读）
  ro
		以只读方式挂载网络共享
  version
		用于区分挂载辅助程序工具的不同版本（通常不需要）
  sep
		如果是第一个挂载选项（在 -o 之后），则覆盖作为挂载选项之间分隔符的逗号

```
			-o user=myname,password=mypassword,domain=mydom

		could be passed instead with period as the separator by::

			-o sep=.user=myname.password=mypassword.domain=mydom

		this might be useful when comma is contained within username
		or password or domain. This option is less important
		when the cifs mount helper cifs.mount (version 1.1 or later)
		is used.
```
  nosuid
		不允许执行带有 suid 位的远程可执行程序。这仅对支持 CIFS Unix 扩展的服务器（如 Samba）有意义。如果你不信任网络中的服务器（你的挂载目标），建议你指定此选项以获得更高的安全性。
  exec
		允许在挂载上执行二进制文件。
  noexec
		不允许在挂载上执行二进制文件。
  dev
		识别远程挂载上的块设备。
  nodev
		不识别远程挂载上的设备。
  suid
		允许在此挂载点上带有 suid 的远程文件被执行（以 root 执行挂载时的默认值，nosuid 是用户挂载的默认值）。
  credentials
		虽然被 cifs 内核组件忽略，但它被挂载辅助程序 mount.cifs 使用。安装 mount.cifs 后，它会打开并读取指定的凭证文件，以获取传递给 cifs vfs 的 userid 和 password 参数。
  guest
		虽然被内核组件忽略，但如果在挂载选项上指定了 guest，mount.cifs 挂载辅助程序将不会提示用户输入密码。如果未指定密码，将使用空密码。
  perm
		客户端进行权限检查（将文件的 uid 和 gid 对照 mode 和期望操作进行 vfs_permission 检查），
		注意这是除目标机器上由服务器软件完成的正常 ACL 检查之外的额外检查。
		客户端权限检查默认启用。
  noperm
		客户端不进行权限检查。这会将此挂载上的文件暴露给本地客户端系统上的其他用户访问。它通常仅在服务器支持 CIFS Unix 扩展，但客户端和服务器系统上的 UID/GID 不够接近以允许执行挂载的用户访问时才需要，但它可能对非 CIFS Unix 扩展挂载有用，例如当默认 mode 在挂载时指定但不应在客户端强制执行时（例如可能在启用 MultiUserMount 时）。
		注意这不影响目标机器上由服务器软件完成的正常 ACL 检查（服务器 ACL 对挂载时提供的用户名的检查）。
  serverino
		使用服务器的 inode 号，而不是在客户端自动生成递增的 inode 号。虽然这将更容易发现硬链接文件（因为它们将有相同的 inode 号），并且 inode 号可能是持久的，但请注意，如果在单个共享下导出了多个服务器端挂载，服务器不保证 inode 号是唯一的（因为如果在同一共享的更高级目录下载挂载了多个文件系统，服务器上的 inode 号可能不唯一）。注意一些较旧的（例如 Windows 2000 之前）不支持返回 UniqueID 或等效的 CIFS Unix 扩展，对于这些，此挂载选项将不起作用。在 nfsd 下导出 cifs 挂载需要在 cifs 挂载上使用此选项。
		如果服务器支持所需的网络操作，这现在是默认值。
  noserverino
		客户端生成 inode 号（而不是使用来自服务器的实际 inode 号）。这些 inode 号在卸载或重启后会变化，这可能使某些应用程序困惑，但并非所有服务器文件系统都支持唯一的 inode 号。
  setuids
		如果与服务器协商了 CIFS Unix 扩展，客户端将尝试在新创建的文件、目录和设备（create、mkdir、mknod）上设置本地进程的有效 uid 和 gid。如果未协商 CIFS Unix 扩展，对于新创建的文件和目录，客户端将缓存新文件的 uid 和 gid 本地，这意味着文件的 uid 在 inode 重新加载（或用户重新挂载共享）时可以更改，而不是使用挂载时指定的默认 uid 和 gid。
  nosetuids
		客户端不会尝试在新创建的文件、目录和设备（create、mkdir、mknod）上设置 uid 和 gid，这将导致服务器将 uid 和 gid 设置为默认值（通常是挂载共享的用户的服务器 uid）。让服务器（而不是客户端）设置 uid 和 gid 是默认值。如果未协商 CIFS Unix 扩展，则新文件的 uid 和 gid 将显示为挂载者的 uid（gid）或挂载时指定的 uid（gid）参数。
  netbiosname
		当通过端口 139 挂载到服务器时，指定在 RFC1001 netbios 会话初始化时用于表示客户端 netbios 机器名的 RFC1001 源名称。
  direct
		不对此挂载上打开的文件进行 inode 数据缓存。
		这排除了在此挂载上 mmap 文件。在某些情况下，具有快速网络且在客户端几乎没有或没有缓存收益（例如当应用程序进行大于页大小且不重读相同数据的大型顺序读取时），这可以提供比默认行为更好的性能，默认行为在获取 oplock（缓存令牌）时通过本地 Linux 客户端 pagecache 缓存读取（readahead）和写入（writebehind）。注意 direct 允许将大于页大小的写操作发送到服务器。
  strictcache
		用于开启严格缓存模式。在此模式下，客户端在拥有 Oplock Level II 时始终从缓存读取，否则从服务器读取。所有写入的数据都存储在缓存中，但如果客户端没有 Exclusive Oplock，它会将数据写入服务器。
  rwpidforward
		将打开文件的进程的 pid 转发到该文件上的任何读取或写操作。这可以防止像 WINE 这样的应用程序在使用强制 brlock 风格时读写失败。
  acl
		如果服务器支持，允许 setfacl 和 getfacl 管理 posix ACL。（默认）
  noacl
		不允许在此挂载上进行 setfacl 和 getfacl 调用
  user_xattr
		允许将用户 xattr（名称以 ``user.`` 或 ``os2.`` 开头的属性）作为 OS/2 EA（扩展属性）获取和设置到服务器。这允许支持 setfattr 和 getfattr 工具。（默认）
  nouser_xattr
		不允许 getfattr/setfattr 获取/设置/列出 xattrs
  mapchars
		将七个保留字符中的六个（反斜杠除外）翻译为：

			*?<>|:

		重映射范围（0xF000 以上），这也允许 CIFS 客户端识别由 Windows 的 POSIX 模拟以这些字符创建的文件。当挂载到大多版本的 Samba（它也禁止创建和打开名称包含这七个字符中任何一个的文件）时，这也很有用。如果服务器不支持线路上的 Unicode，则这不起作用。
  nomapchars
		不翻译这七个字符中的任何一个（默认）。
  nocase
		请求不区分大小写的路径名匹配（如果服务器支持，则区分大小写是默认）。
		（挂载选项 ``ignorecase`` 与 ``nocase`` 相同）
  posixpaths
		如果支持 CIFS Unix 扩展，尝试协商 posix 路径名支持，它允许某些在典型 CIFS 文件名中被禁止的字符，而无需重映射。（默认）
  noposixpaths
		如果支持 CIFS Unix 扩展，不请求 posix 路径名支持（这可能导致服务器拒绝创建包含某些保留字符的文件）。
  nounix
		对此挂载（树连接）禁用 CIFS Unix 扩展。这很少需要，但它可能用于一次关闭多个设置（即 posix acls、posix locks、posix paths、symlink 支持以及从服务器检索 uids/gids/mode），或用于规避实现了 Unix 扩展的服务器中的 bug。
  nobrl
		不向服务器发送字节范围锁请求。
		这对于某些因 cifs 风格强制字节范围锁而中断（且大多数 cifs 服务器尚不支持请求建议性字节范围锁）的应用程序是必要的。
  forcemandatorylock
		即使服务器支持 posix（建议性）字节范围锁定，也只发送强制锁请求。对于一些（大概很少见）最初为 DOS/Windows 编写、需要 Windows 风格强制字节范围锁的应用程序，它们可能能够利用此选项，强制 cifs 客户端只发送强制锁，即使 cifs 服务器支持 posix 建议性锁。
		``forcemand`` 被接受为此挂载选项的简写形式。
  nostrictsync
		如果设置了此挂载选项，当应用程序进行 fsync 调用时，cifs 客户端不会向服务器发送 SMB Flush（强制服务器立即将该文件的所有脏数据写入磁盘），尽管 cifs 仍将所有脏（缓存）文件数据发送到服务器并等待服务器响应写入。由于 SMB Flush 可能非常慢，且某些服务器可能足够可靠（可以冒稍微延迟将数据刷新到服务器磁盘的风险），开启此选项可能有助于改善那些 fsync 过多的应用程序的性能，但有服务器崩溃的小风险。如果未设置此挂载选项，默认情况下 cifs 会在每次 fsync 调用时发送 SMB flush 请求（并等待响应）。
  nodfs
		即使服务器声称支持，也禁用 DFS（全局命名空间支持）。这有助于规避 Samba 服务器版本 3.0.24 和 3.0.25 解析 DFS 路径的问题。
  remount
		重新挂载共享（常用于从 ro 改为 rw 挂载或反之）
  cifsacl
		根据文件的 Windows ACL 报告 mode 位（例如在 stat 上）。（实验性）
  servern
		指定尝试与服务器建立会话时要使用的服务器 netbios 名称（RFC1001 名称）。
		这对于挂载到某些较旧的服务器（例如 OS/2 或 Windows 98 和 Windows ME）是必需的，因为它们不支持默认服务器名称。服务器名称最长可达 15 个字符，通常大写。
  sfu
		当未协商 CIFS Unix 扩展时，尝试以与 Unix 服务（SFU）兼容的格式创建设备文件和 fifos。此外通过 SETFILEBITS 扩展属性（如 SFU 那样）检索 mode 的第 10-12 位。将来 mode 的低 9 位也将通过查询安全描述符（ACL）来模拟。
  mfsymlinks
		启用对 Minshall+French 符号链接的支持
		（见 http://wiki.samba.org/index.php/UNIX_Extensions#Minshall.2BFrench_symlinks）
		当与 'sfu' 选项一起指定时，此选项被忽略。即使服务器支持 CIFS Unix 扩展，也会使用 Minshall+French 符号链接。
  sign
		必须使用包签名（有助于避免路由中中间系统对数据的非预期修改）。注意签名不能与 lanman 或明文认证一起工作。
  seal
		必须在此挂载共享上密封（加密）所有数据，然后再在网络上发送。需要 Unix 扩展支持。注意这与 sign 挂载选项的不同之处在于，它导致通过此挂载共享发送的数据被加密，但挂载到同一服务器的其他共享不受影响。
  locallease
		此选项很少需要。某些应用程序（如 Samba 和 NFSv4 服务器）使用 fcntl F_SETLEASE 来检查文件是否可缓存。CIFS 无法显式请求租约，但可以检查文件是否可缓存（oplocked）。不幸的是，即使文件未被 oplocked，它仍可能是可缓存的（即如果没有其他本地进程使用该文件，cifs 客户端可以授予 fcntl 租约），例如当服务器不支持 oplocks 且用户确信对该文件的唯一更新将来自此客户端时。指定此挂载选项将允许 cifs 客户端仅为未被 oplocked 的文件在本地检查租约，而不是在这种情况下拒绝租约。（实验性）
  sec
		安全模式。允许的值为：

			none
				尝试作为空用户（无名）连接
			krb5
				使用 Kerberos 版本 5 认证
			krb5i
				使用 Kerberos 认证和包签名
			ntlm
				使用 NTLM 密码哈希（默认）
			ntlmi
				使用带签名的 NTLM 密码哈希（如果
				/proc/fs/cifs/PacketSigningEnabled 开启，
				或者如果服务器也需要签名，也可以作为默认）
			ntlmv2
				使用 NTLMv2 密码哈希
			ntlmv2i
				使用带包签名的 NTLMv2 密码哈希
			lanman
				（如果在内核配置中配置）使用较旧的
				lanman 哈希
  hard
		如果服务器无响应，重试文件操作
  soft
		限制对无响应服务器的重试（通常仅一次重试）然后返回错误。（默认）

```
The mount.cifs mount helper also accepts a few mount options before -o
including:

=============== ===============================================================
	-S      take password from stdin (equivalent to setting the environment
		variable `PASSWD_FD=0`
	-V      print mount.cifs version
	-?      display simple usage information
=============== ===============================================================

With most 2.6 kernel versions of modutils, the version of the cifs kernel
module can be displayed via modinfo.

```
mount.cifs 挂载辅助程序在 -o 之前也接受一些挂载选项，包括：

=============== ===============================================================
	-S      从 stdin 获取密码（等效于设置环境变量 `PASSWD_FD=0`）
	-V      打印 mount.cifs 版本
	-?      显示简单的用法信息
=============== ===============================================================

对于大多数 2.6 内核版本的 modutils，cifs 内核模块的版本可以通过 modinfo 显示。

## Misc /proc/fs/cifs 标志与调试信息

信息伪文件：

======================= =======================================================
DebugData		Displays information about active CIFS sessions and
			shares, features enabled as well as the cifs.ko
			version.
Stats			Lists summary resource usage information as well as per
			share statistics.
open_files		List all the open file handles on all active SMB sessions.
mount_params            List of all mount parameters available for the module
======================= =======================================================

配置伪文件：

======================= =======================================================
SecurityFlags		Flags which control security negotiation and
			also packet signing. Authentication (may/must)
			flags (e.g. for NTLMv2) may be combined with
			the signing flags.  Specifying two different password
			hashing mechanisms (as "must use") on the other hand
```
				0x00C5

			(NTLMv2 and packet signing allowed). Some SecurityFlags
			may require enabling a corresponding menuconfig option.

			  may use packet signing			0x00001
			  must use packet signing			0x01001
			  may use NTLMv2				0x00004
			  must use NTLMv2				0x04004
			  may use Kerberos security (krb5)		0x00008
			  must use Kerberos                             0x08008
			  may use NTLMSSP               		0x00080
			  must use NTLMSSP           			0x80080
			  seal (packet encryption)			0x00040
			  must seal                                     0x40040

```
cifsFYI			If set to non-zero value, additional debug information
			will be logged to the system error log.  This field
			contains three flags controlling different classes of
			debugging entries.  The maximum value it can be set
			to is 7 which enables all debugging points (default 0).
			Some debugging statements are not compiled into the
			cifs kernel unless CONFIG_CIFS_DEBUG2 is enabled in the
			kernel configuration. cifsFYI may be set to one or
```
			  +-----------------------------------------------+------+
			  | log cifs informational messages		  | 0x01 |
			  +-----------------------------------------------+------+
			  | log return codes from cifs entry points	  | 0x02 |
			  +-----------------------------------------------+------+
			  | log slow responses				  | 0x04 |
			  | (ie which take longer than 1 second)	  |      |
			  |                                               |      |
			  | CONFIG_CIFS_STATS2 must be enabled in .config |      |
			  +-----------------------------------------------+------+

```
traceSMB		If set to one, debug information is logged to the
			system error log with the start of smb requests
			and responses (default 0)
LookupCacheEnable	If set to one, inode information is kept cached
			for one second improving performance of lookups
			(default 1)
LinuxExtensionsEnabled	If set to one then the client will attempt to
			use the CIFS "UNIX" extensions which are optional
			protocol enhancements that allow CIFS servers
			to return accurate UID/GID information as well
			as support symbolic links. If you use servers
			such as Samba that support the CIFS Unix
			extensions but do not want to use symbolic link
			support and want to map the uid and gid fields
			to values supplied at mount (rather than the
			actual values, then set this to zero. (default 1)
dfscache		List the content of the DFS cache.
			If set to 0, the client will clear the cache.
======================= =======================================================

这些实验性特性和跟踪可以通过更改 /proc/fs/cifs 中的标志来启用（在 cifs 模块已安装或构建进内核之后，例如 insmod cifs）。要启用某项特性，将其设置为 1，例如要启用

```
	echo 7 > /proc/fs/cifs/cifsFYI

```
cifsFYI 充当位掩码。将其设置为 1 会启用各种信息性消息的额外内核日志记录。2 启用非零 SMB 返回码的日志记录，而 4 启用耗时超过一秒完成的请求（字节范围锁请求除外）的日志记录。将其设置为 4 需要在内核配置（.config）中设置 CONFIG_CIFS_STATS2。将其设置为 7 会启用全部三项。最后，跟踪

```
	echo 1 > /proc/fs/cifs/traceSMB

```
每个共享（每个客户端挂载）的统计信息可在 /proc/fs/cifs/Stats 中找到。如果内核配置（.config）中启用了 CONFIG_CIFS_STATS2，则可获得更多信息。返回的统计数据包括表示按请求类型（read、write、close 等）分组的已尝试和失败（即服务器的非零返回码）的 SMB3（或 cifs）请求数量的计数器。还记录了向该共享的服务器读取和写入的总字节数。注意，由于客户端缓存效应，这可能少于客户端上运行的应用程序读取和写入的字节数。可以通过 `echo 0 > /proc/fs/cifs/Stats` 将统计信息重置为零，这在比较两个不同场景的性能时可能有用。

另请注意，`cat /proc/fs/cifs/DebugData` 将显示有关活动会话和已挂载共享的信息。

启用 Kerberos（扩展安全）可以工作，但需要有版本 1.2 或更高的辅助程序 cifs.upcall 存在并配置在 /etc/request-key.conf 文件中。cifs.upcall 辅助程序来自 Samba 项目（https://www.samba.org）。NTLM、NTLMv2 和 LANMAN 支持不需要此辅助程序。注意，NTLMv2 安全（不需要 cifs.upcall 辅助程序），而不是使用 Kerberos，对一些用例已经足够。

DFS 支持允许透明重定向到 MS-DFS 命名空间中的共享。此外，对于指定为以主机名（而不是 IP 地址）开头的 UNC 名称的目标共享的 DFS 支持，需要一个用户空间辅助程序（如 cifs.upcall）存在，以便将主机名转换为 ip 地址，并且该用户空间辅助程序也必须配置在 /etc/request-key.conf 文件中。Samba、Windows 服务器和许多 NAS 设备支持 DFS，作为构建全局命名空间以简化网络配置并提高可靠性的一种方式。

要使用 cifs Kerberos 和 DFS 支持，应安装 Linux keyutils 软件包，并且应向

```
  create cifs.spnego * * /usr/local/sbin/cifs.upcall %k
  create dns_resolver * * /usr/local/sbin/cifs.upcall %k

```
## CIFS 内核模块参数

这些模块参数可以在以下时间指定或修改：

```
	/sys/module/cifs/parameters/<param>

```
```
    echo "value" > /sys/module/cifs/parameters/<param>

```
关于可用模块参数及其值的更详细描述可通过以下方式查看：

    modinfo cifs (or modinfo smb3)

================= ==========================================================
1. enable_oplocks 启用或禁用 oplocks。oplocks 默认启用。
		  [Y/y/1] 启用。要禁用可使用 [N/n/0]。
================= ==========================================================
