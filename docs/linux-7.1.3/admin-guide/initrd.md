## 使用初始 RAM 磁盘（initrd）


Written 1996,2000 by Werner Almesberger <werner.almesberger@epfl.ch> and
Hans Lermen <lermen@fgan.de>


initrd 提供了通过引导加载程序加载一个 RAM 磁盘的能力。随后该 RAM 磁盘可以被挂载
为根文件系统，并可以从中运行程序。之后，可以从另一个设备挂载一个新的根文件系统。
先前的根（来自 initrd）随后被移动到一个目录，并可以在之后被卸载。

initrd 主要设计用于让系统启动分两个阶段进行：内核以一组最小的内置驱动启动，而额外
的模块从 initrd 加载。

本文档简要概述 initrd 的使用。关于启动过程的更详细讨论可参见 [#f1]_。

### 操作


使用 initrd 时，系统通常如下启动：

  1) 引导加载程序加载内核和初始 RAM 磁盘
  2) 内核将 initrd 转换为一个“普通”的 RAM 磁盘，并释放 initrd 占用的内存
  3) 如果根设备不是 `/dev/ram0`，则遵循旧的（已废弃的）change_root 流程。参见
     下文“过时的根切换机制”一节。
  4) 挂载根设备。如果是 `/dev/ram0`，则将 initrd 镜像挂载为根
  5) 执行 /sbin/init（这可以是任何有效的可执行文件，包括 shell 脚本；它以 uid 0
     运行，几乎可以做 init 能做的任何事情）
  6) init 挂载“真正的”根文件系统
  7) init 使用 pivot_root 系统调用将根文件系统放到根目录
  8) init 在新根文件系统上 exec `/sbin/init`，执行通常的启动序列
  9) 移除 initrd 文件系统

注意，更改根目录并不涉及卸载它。因此，在此过程中可以让进程继续在 initrd 上运行。
同时也要注意，在 initrd 下挂载的文件系统在此期间仍可访问。


### 引导命令行选项


```
  initrd=<path>    (e.g. LOADLIN)

    Loads the specified file as the initial RAM disk. When using LILO, you
    have to specify the RAM disk image file in /etc/lilo.conf, using the
    INITRD configuration variable.

  noinitrd

    initrd data is preserved but it is not converted to a RAM disk and
    the "normal" root file system is mounted. initrd data can be read
    from /dev/initrd. Note that the data in initrd can have any structure
    in this case and doesn't necessarily have to be a file system image.
    This option is used mainly for debugging.

    Note: /dev/initrd is read-only and it can only be used once. As soon
    as the last process has closed it, all data is freed and /dev/initrd
    can't be opened anymore.

  root=/dev/ram0

    initrd is mounted as root, and the normal boot procedure is followed,
    with the RAM disk mounted as root.
```

### 压缩的 cpio 镜像


近期的内核支持从一个压缩的 cpio 归档来填充 ramdisk。在这类系统上，创建 ramdisk
镜像不再需要涉及特殊的块设备或回环设备；你只需在磁盘上创建一个包含所需 initrd
内容的目录，cd 进入该目录，然后运行（以
```
	find . | cpio --quiet -H newc -o | gzip -9 -n > /boot/imagefile.img
```
```
	mkdir /tmp/imagefile
	cd /tmp/imagefile
	gzip -cd /boot/imagefile.img | cpio -imd --quiet
```
### 安装


首先，必须在
```
	# mkdir /initrd
```
上创建一个用于 initrd 文件系统的目录。该名称并不重要。更多细节可在 `pivot_root(2)`
手册页中找到。

如果根文件系统是在启动过程中创建的（即如果你在制作一张安装软盘），则根文件系统的
创建过程应当创建 `/initrd` 目录。

如果在某些情况下 initrd 不会被挂载，其内容仍然是
```
	# mknod /dev/initrd b 1 250
	# chmod 400 /dev/initrd
```
其次，内核必须编译时启用 RAM 磁盘支持以及初始 RAM 磁盘支持。同时，至少所有从
initrd 执行程序所需的组件（例如可执行文件格式和文件系统）都必须编译进内核。

第三，你必须创建 RAM 磁盘镜像。这通过在一个块设备上创建文件系统、按需将文件复制到
其中，然后将该块设备的内容复制到 initrd 文件来完成。对于近期的内核，至少有三类
设备适合于此：

 - 软盘（到处可用但慢得令人痛苦）
 - RAM 磁盘（快，但会分配物理内存）
 - 回环设备（最优雅的方案）

我们将描述回环设备方法：

 1) 确保回环块设备已配置进内核
```
	# dd if=/dev/zero of=initrd bs=300k count=1
	# mke2fs -F -m0 initrd

    (if space is critical, you may want to use the Minix FS instead of Ext2)
 3) mount the file system, e.g.::

	# mount -t ext2 -o loop initrd /mnt

 4) create the console device::

    # mkdir /mnt/dev
    # mknod /mnt/dev/console c 5 1

 5) copy all the files that are needed to properly use the initrd
    environment. Don't forget the most important file, ``/sbin/init``

    .. note:: ``/sbin/init`` permissions must include "x" (execute).

 6) correct operation the initrd environment can frequently be tested
    even without rebooting with the command::

	# chroot /mnt /sbin/init

    This is of course limited to initrds that do not interfere with the
    general system state (e.g. by reconfiguring network interfaces,
    overwriting mounted devices, trying to start already running demons,
    etc. Note however that it is usually possible to use pivot_root in
    such a chroot'ed initrd environment.)
 7) unmount the file system::

	# umount /mnt

 8) the initrd is now in the file "initrd". Optionally, it can now be
    compressed::

	# gzip -9 initrd
```
为了试验 initrd，你可能想拿一张救援软盘，并只从 `/sbin/init` 添加一个指向
`/bin/sh` 的符号链接。另外，你也可以尝试实验性的 newlib 环境 [#f2]_ 来创建一个
小型 initrd。

最后，你必须引导内核并加载 initrd。几乎所有 Linux 引导加载程序都支持 initrd。由于
启动过程仍与旧机制兼容，以下引导命令行参数
```
  root=/dev/ram0 rw
```
（rw 只有在需要写入 initrd 文件系统时才是必要的）
```
     LOADLIN <kernel> initrd=<disk_image>
```
```
	LOADLIN C:\LINUX\BZIMAGE initrd=C:\LINUX\INITRD.GZ root=/dev/ram0 rw
```
使用 LILO，你可以在 `/etc/lilo.conf` 的全局段或相应内核的段中添加选项
`INITRD=<path>`，并传递
```
  image = /bzImage
    initrd = /boot/initrd.gz
    append = "root=/dev/ram0 rw"
```
然后运行 `/sbin/lilo`

关于其它引导加载程序，请参考各自的文档。

现在你可以引导并享受使用 initrd 了。


### 更改根设备


完成其职责后，init 通常会更改根设备，并继续在“真正的”根设备上启动 Linux 系统。

该流程包含以下步骤：
 - 挂载新的根文件系统
 - 将其变为根文件系统
 - 移除对旧（initrd）根文件系统的所有访问
 - 卸载 initrd 文件系统并释放 RAM 磁盘

挂载新的根文件系统很容易：只需将其挂载到
```
	# mkdir /new-root
	# mount -o ro /dev/hda1 /new-root
```
根切换通过 pivot_root 系统调用完成，该调用也可以通过 `pivot_root` 实用程序获得
（参见 `pivot_root(8)` 手册页；`pivot_root` 随 util-linux 2.10h 或更高版本分发
[#f3]_）。`pivot_root` 将当前根移动为新根下的一个目录，并将新根放到它的位置。旧根
的目录
```
	# cd /new-root
	# mkdir initrd
	# pivot_root . initrd
```
现在，init 进程仍可通过其可执行文件、共享库、标准输入/输出/错误以及其当前根目录
访问旧根。所有这些引用通过
```
	# exec chroot . what-follows <dev/console >dev/console 2>&1
```
被丢弃，其中 what-follows 是新根下的一个程序，例如 `/sbin/init`。如果新根文件系统
将与 udev 一起使用且没有有效的 `/dev` 目录，则必须在调用 chroot 之前初始化 udev，
以提供 `/dev/console`。

注意：pivot_root 的实现细节可能随时间变化。为了确保兼容性，应注意以下几点：

 - 在调用 pivot_root 之前，调用进程的当前目录应指向新根目录
 - 使用 . 作为第一个参数，并将旧根目录的_相对_路径作为第二个参数
 - 在旧根和新根下都必须有一个可用的 chroot 程序
 - 之后 chroot 到新根
 - 在 exec 命令中对 dev/console 使用相对路径

现在，initrd 可以被卸载，RAM 分配的内存可以被
```
	# umount /initrd
	# blockdev --flushbufs /dev/ram0
```
也可以将 initrd 与 NFS 挂载的根一起使用，详见 `pivot_root(8)` 手册页。


### 使用场景


实现 initrd 的主要动机是允许在系统安装时进行模块化内核配置。流程如下：

  1) 系统从软盘或其它介质以最小内核（例如支持 RAM 磁盘、initrd、a.out 和 Ext2
     FS）启动并加载 initrd
  2) `/sbin/init` 确定需要什么来（1）挂载“真正的”根 FS（即设备类型、设备驱动、
     文件系统）以及（2）发行介质（例如 CD-ROM、网络、磁带……）。这可以通过询问
     用户、自动探测或混合方法来完成。
  3) `/sbin/init` 加载必要的内核模块
  4) `/sbin/init` 创建并填充根文件系统（这还不必是一个非常好用的系统）
  5) `/sbin/init` 调用 `pivot_root` 来更改根文件系统，并通过 chroot exec 一个
     继续安装的程序
  6) 安装引导加载程序
  7) 引导加载程序被配置为加载一个包含用于启动系统的模块集的 initrd（例如可以
     修改 `/initrd`，然后卸载，最后将镜像从 `/dev/ram0` 或 `/dev/rd/0` 写入
     文件）
  8) 现在系统可引导，并且可以执行额外的安装任务

initrd 在这里的关键作用是，在正常系统运行期间复用配置数据，而无需使用一个臃肿的
“通用”内核，也无需重新编译或重新链接内核。

第二种场景用于这样的安装：Linux 运行在单一管理域内、具有不同硬件配置的系统上。在
这种情况下，最好只生成一小部分内核（理想情况下只有一个），并让配置信息中系统特定的
部分尽可能小。在这种情况下，可以生成一个包含所有必要模块的公共 initrd。那么，只有
`/sbin/init` 或它所读取的一个文件需要不同。

第三种场景是更方便的救援盘，因为像根 FS 分区位置这类信息无需在启动时提供，而从
initrd 加载的系统可以调用一个用户友好的对话框，并且还可以执行一些健全性检查（甚至
某种形式的自动检测）。

最后但并非最不重要的是，CD-ROM 发行商可以借助它实现更好的从 CD 安装，例如通过使用
引导软盘并通过 initrd 从 CD 引导一个更大的 RAM 磁盘；或者通过一个像 `LOADLIN`
这样的加载程序直接从 CD-ROM 引导，并从 CD 加载 RAM 磁盘而无需软盘。


### 过时的根切换机制


以下机制在引入 pivot_root 之前使用。当前内核仍然支持它，但你不应当依赖它继续
可用。

它通过在内核映像中用 rdev 设置、或在引导命令行用 root=... 设置的“真正的”根设备，
在 linuxrc 退出时挂载为根文件系统来工作。然后 initrd 文件系统被卸载，或者，如果它
仍然忙，则被移动到一个目录 `/initrd`（如果该目录存在于新根文件系统上）。

为了使用这种机制，你无需指定引导命令选项 root、init 或 rw。（如果指定了，它们将
影响真正的根文件系统，而不是 initrd 环境。）

如果挂载了 /proc，“真正的”根设备可以通过从 linuxrc 内部将新根 FS 设备的编号写入
特殊的
```
  # echo 0x301 >/proc/sys/kernel/real-root-dev
```
注意，该机制与 NFS 及类似文件系统不兼容。

这个旧的、已废弃的机制通常称为 `change_root`，而新的、受支持的机制称为 `pivot_root`。


### change_root 与 pivot_root 混合机制


如果你不想使用 `root=/dev/ram0` 来触发 pivot_root 机制，你可以在 initrd 镜像中
同时创建 `/linuxrc` 和 `/sbin/init`。

```
	#! /bin/sh
	mount -n -t proc proc /proc
	echo 0x0100 >/proc/sys/kernel/real-root-dev
	umount -n /proc
```
一旦 linuxrc 退出，内核会再次将你的 initrd 挂载为根，这次执行 `/sbin/init`。同样，
将由这个 init 负责在最终执行真正的 `/sbin/init` 之前构建正确的环境（也许使用
命令行上传入的 `root= device`）。


### 资源


    https://www.almesberger.net/cv/papers/ols2k-9.ps.gz
    https://www.sourceware.org/newlib/
    https://www.kernel.org/pub/linux/utils/util-linux/
