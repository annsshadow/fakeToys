
## Ramfs、rootfs initramfs

2005 骞?10 鏈?17 鏃。
:Author: Rob Landley <rob@landley.net>

### 什么是 ramfs
ramfs 是一个非常简单的文件系统，它Linux 的磁盘缓存机制（page cache dentry cache）作为一个可动态调整大小的、基RAM 的文件系统导出
通常，Linux 会将所有文件缓存在内存中。从后备存储（backing store，通常是文件系统挂载其上的块设备）读出的数据页会被保留，以备再次需要时使用，但同时会被标记clean（可释放），以便虚拟内存（Virtual Memory）系统在需要内存做其他用途时将其回收。类似地，写入文件的数据一旦写入后备存储就会被标记clean，但会为了缓存目的而保留，直到 VM 重新分配该内存。类似的机制（dentry cache）大大加快了访问目录的速度
对于 ramfs，没有后备存储。写ramfs 的文件会像往常一样分dentry page cache，但却没有地方可以写入。这意味着这些页永远不会被标记clean，因此当 VM 想要回收内存时无法释放它们
实现 ramfs 所需的代码量非常小，因为所有工作都由现有的 Linux 缓存基础设施完成。从根本上说，你就是把磁盘缓存当作文件系统挂载。正因如此，ramfs 不是一个可通过 menuconfig 移除的可选组件，因为那样节省的空间微乎其微
### ramfs ramdisk（内存盘
较老的 "ram disk"（内存盘）机制从一RAM 区域中创建一个合成的块设备，并将其用作文件系统的后备存储。这个块设备大小固定，因此挂载其上的文件系统大小也固定。使ram disk 还需要不必要地把内存从那个假的块设备复制page cache（并把改动复制回去），以及创建和销dentry。此外，它还需要一个文件系统驱动（ext2）来格式化并解析这些数据
ramfs 相比，这会浪费内存（以及内存总线带宽），CPU 带来不必要的工作，并污染 CPU 缓存。（虽然有一些通过操作页表来避免这种复制的技巧，但它们复杂得令人不快，而且结果证明代价与复制差不多。）更关键的是，ramfs 所做的所有工作无论如何都必须发生，因为所有文件访问都要经page dentry 缓存。RAM disk 根本就是多余的；ramfs 在内部要简单得多
ramdisk 半过时的另一个原因是，loopback 设备（回环设备）的引入提供了一种更灵活、更方便的方式来创建合成块设备——现在从文件而不是从内存块创建。详losetup(8)
### ramfs 涓?tmpfs

ramfs 的一个缺点是，你可以一直向其中写入数据，直到填满所有内存，VM 无法释放它，因为 VM 认为文件应当被写入后备存储（而非交换空间），ramfs 没有任何后备存储。因此，只应允许 root（或一个受信任的用户）ramfs 挂载点拥有写访问权
一个名tmpfs ramfs 衍生物被创造出来，用于增加大小限制，以及将数据写入交换空间的能力。普通用户可以被允许tmpfs 挂载点拥有写访问权。更多信息请参阅 Documentation/filesystems/tmpfs.rst
### 什么是 rootfs
rootfs ramfs（若启用tmpfs，则tmpfs）的一个特殊实例，它始终存在于 Linux 系统中。内核使用一个名nullfs 的不可变的空文件系统作为 VFS 层级结构的真正根，而可变的 rootfs（tmpfs/ramfs）挂载在它之上。这使得 pivot_root() initramfs 的卸载能够正常工作
大多数系统只是把另一个文件系统挂载到 rootfs 之上，然后忽略它。一个空ramfs 实例所占用的空间极小
如果启用CONFIG_TMPFS，rootfs 默认将使tmpfs 而非 ramfs。若要强制使ramfs，可在内核命令行中加"rootfstype=ramfs"
### 什么是 initramfs
所2.6 版本Linux 内核都包含一gzip 压缩"cpio" 格式归档，它会在内核启动时解压到 rootfs 中。解压之后，内核检rootfs 是否包含一个名"init" 的文件，如果是，就把它作PID 1 执行。若init 进程存在，它负责将系统的其余部分启动起来，包括定位并挂载真正的根设备（如果有）。如果内嵌的 cpio 归档解压rootfs 之后，rootfs 中仍不包init 程序，内核就会退回到较旧的代码，去定位并挂载一个根分区，然后从exec 某个变体/sbin/init
这一切与旧的 initrd 在几个方面有所不同
  - 旧的 initrd 始终是一个独立的文件，initramfs 归档被链接进 Linux 内核映像中。（`linux-*/usr` 目录就专门用于在内核构建期间生成这个归档。）

  - 旧的 initrd 文件是一gzip 压缩的文件系统映像（采用某种文件格式，如 ext2，需要内核内建驱动），而新initramfs 归档是一gzip 压缩cpio 归档（类tar，但更简单，参见 cpio(1) Documentation/driver-api/early-userspace/buffer-format.rst）。内核的 cpio 解压代码不仅极其小巧，而且属于 __init 文本与数据，可以在启动过程中被丢弃
  - 旧的 initrd 运行的程序（名为 /initrd，而非 /init）会做一些设置，然后返回内核；而来initramfs init 程序不应返回内核。（如果 /init 需要交出控制权，它可以用一个新的根设备覆盖挂载 / exec 另一init 程序。参见下文的 switch_root 工具。）

  - 当切换到另一个根设备时，initrd 会执pivot_root，然umount ramdisk。由nullfs 是真正的根，pivot_root() 可以正常工作

```
      chdir(new_root);
      pivot_root(".", ".");
      umount2(".", MNT_DETACH);

    This is the preferred method for switching root filesystems.

```

### 填充 initramfs

2.6 内核的构建过程总是创建一gzip 压缩cpio 格式 initramfs 归档，并将其链接进最终的内核二进制文件中。默认情况下，这个归档是空的（在 x86 上占134 字节）
配置选项 CONFIG_INITRAMFS_SOURCE（位menuconfig General Setup 中，定义usr/Kconfig）可用于指定 initramfs 归档的来源，它会自动被并入最终二进制文件中。该选项可以指向一个已有的 gzip 压缩 cpio 归档、一个包含待归档文件的目录，或一个文本文
```
  dir /dev 755 0 0
  nod /dev/console 644 0 0 c 5 1
  nod /dev/loop0 644 0 0 b 7 0
  dir /bin 755 1000 1000
  slink /bin/sh busybox 777 0 0
  file /bin/busybox initramfs/busybox 755 0 0
  dir /proc 755 0 0
  dir /sys 755 0 0
  dir /mnt 755 0 0
  file /init initramfs/init.sh 755 0 0

```

在内核构建之后运"usr/gen_init_cpio" 可以获取描述上述文件格式的使用说明
配置文件的一个优点是，在新归档中设置权限或创建设备节点不需root 权限。（注意，那两个示例 "file" 条目期望linux-2.6.* 目录下名"initramfs" 的子目录中找到名"init.sh" "busybox" 的文件。更多细节请参阅 Documentation/driver-api/early-userspace/early_userspace_support.rst。）

内核并不依赖外部cpio 工具。如果你指定的是一个目录而不是配置文件，内核的构建基础设施会由该目录生成一个配置文件（usr/Makefile 调用 usr/gen_initramfs.sh），然后继续使用该配置文件打包该目录（将其喂usr/gen_init_cpio，后者由 usr/gen_init_cpio.c 生成）。内核在构建时创cpio 的代码是完全自包含的，内核在启动时的解压器同样（显然）是自包含的
你唯一可能需要安装外cpio 工具的情况是，要创建或解压你自己预先准备好的、喂给内核构建的 cpio 文件（而不是用配置文件或目录）
以下命令行可以解压一cpio 映像（无论是通过上面的脚
```
  cpio -i -d -H newc -F initramfs_data.cpio --no-absolute-filenames

```

以下 shell 脚本可以创建一个预构建cpio 归档，你可以

```
  #!/bin/sh

  # Copyright 2006 Rob Landley <rob@landley.net> and TimeSys Corporation.
  # Licensed under GPL version 2

  if [ $# -ne 2 ]
  then
    echo "usage: mkinitramfs directory imagename.cpio.gz"
    exit 1
  fi

  if [ -d "$1" ]
  then
    echo "creating $2 from $1"
    (cd "$1"; find . | cpio -o -H newc | gzip) > "$2"
  else
    echo "First argument must be a directory"
    exit 1
  fi

```

    cpio man 手册页包含一些糟糕的建议，如果你照做会破坏你initramfs 归档。它生成文件名列表的典型方式是使find 命令；你应该find 加上 -depth 选项，以尽量减少对不可写或不可搜索目录的权限问题在创initramfs.cpio.gz 映像时切勿这样做，那样是行不通的。Linux 内核cpio 解压器不会在不存在的目录中创建文件，因此目录条目必须出现在该目录中的文件之前。上面的脚本以正确的顺序生成它们
### 外部 initramfs 映像

如果内核启用initrd 支持，也可以把一个外部的 cpio.gz 归档当作 initrd 传入 2.6 内核。在这种情况下，内核会自动检测其类型（initramfs，而非 initrd），并在尝试运行 /init 之前将该外部 cpio 归档解压rootfs 中
这具initramfs 的内存效率优势（没有 ramdisk 块设备），同时又initrd 的独立打包特性（如果你想initramfs 运行GPL 的代码，而又不把它与GPL 许可Linux 内核二进制混在一起，这就很方便）
### initramfs 的内
initramfs 归档是一个完整、自包含Linux 根文件系统。如果你还不了解要让一个最小根文件系统启动运行需要哪些共享库、设备和路径，可以参考以下资料：

- https://www.tldp.org/HOWTO/Bootdisk-HOWTO/
- https://www.tldp.org/HOWTO/From-PowerUp-To-Bash-Prompt-HOWTO.html
- http://www.linuxfromscratch.org/lfs/view/stable/

"klibc" 软件包（https://www.kernel.org/pub/linux/libs/klibc）被设计成一个极小的 C 库，用于早期用户空间代码的静态链接，并附带一些相关的工具。它采用 BSD 许可
我自己使uClibc（https://www.uclibc.org）与 busybox（https://www.busybox.net）。它们分别采LGPL GPL 许可。（busybox 1.3 版本计划提供一个自包含initramfs 软件包。）

理论上你可以使用 glibc，但它并不适合这类小型嵌入式用途。（一个静态链glibc "hello world" 程序超过 400k，而用 uClibc 只有 7k。还要注意，glibc 会通过 dlopen 加载 libnss 来做名称查找，即使其他地方是静态链接的。）

一个好的第一步是initramfs 运行一个静态链接的 "hello world" 程序作为 init，并qemu（www.qemu.org）之类的模拟器下测试它，或
```
  cat > hello.c << EOF
  #include <stdio.h>
  #include <unistd.h>

  int main(int argc, char *argv[])
  {
    printf("Hello world!\n");
    sleep(999999999);
  }
  EOF
  gcc -static hello.c -o init
  echo init | cpio -o -H newc | gzip > test.cpio.gz
  # Testing external initramfs using the initrd loading mechanism.
  qemu -kernel /boot/vmlinuz -initrd test.cpio.gz /dev/zero

```

在调试一个普通根文件系统时，能够"init=/bin/sh" 启动是很方便的。initramfs 的等价做法是 "rdinit=/bin/sh"，它同样有用
### 为什么用 cpio 而不tar
这一决定是在 2001 12 月做出的。讨论始于此处：

- https://lore.kernel.org/lkml/a03cke$640$1@cesium.transmeta.com/

并由此引发了第二个讨论串（专门关tar cpio 的对比），始于此处：

- https://lore.kernel.org/lkml/3C25a06d.7030408@zytor.com/

简明扼要的总结版本（不能替代阅读上述讨论串）如下：

1) cpio 是一个标准。它已有几十年历史（可追溯到 AT&T 时代），并且已经Linux 上被广泛使用（在 RPM、Red Hat 的设备驱动盘内）。这里有一1996 年关于它Linux Journal 文章
      http://www.linuxjournal.com/article/1213

   它不tar 流行，是因为传统cpio 命令行工具需_truly_hideous_（极其丑陋）的命令行参数。但这对归档格式本身的好与坏并无任何说明，而且还有替代工具，例如：

      https://linux.die.net/man/1/afio

2) 内核所选的 cpio 归档格式比任何一种（真有几十种）tar 归档格式都更简单、更干净（因而也更容易创建和解析）。完整的 initramfs 归档格式buffer-format.rst 中有说明，由 usr/gen_init_cpio.c 生成，并init/initramfs.c 解压。三者合在一起，人类可读文本总量不到 26k
3) GNU 项目tar 标准化，其相关性大约等同于 Windows zip 标准化。Linux 不属于其中任何一方，可以自由做出自己的技术决策
4) 既然这是内核内部格式，它本可以轻易地是一种全新的东西。无论如何，内核都提供了自己的工具来创建和解压这种格式。使用现有标准是更可取的，但并非必要
5) 这一决定Al Viro 做出（引文："tar is ugly as hell and not going to be supported on the kernel side"（tar 丑陋至极，内核侧不会提供支持））
    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112222109050.21702-100000@weyl.math.psu.edu/

   他解释了自己的理由：

    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112222240530.21702-100000@weyl.math.psu.edu/
    - https://lore.kernel.org/lkml/Pine.GSO.4.21.0112230849550.23300-100000@weyl.math.psu.edu/

   并且，最重要的是，他设计并实现了 initramfs 代码
### 未来方向

如今.6.16），initramfs 总是被编译进内核，但并不总是被使用。内核会回退到传统启动代码，而该代码只有initramfs 不包/init 程序时才会被触及。这个回退代码是遗留代码，用于确保平滑过渡，并允许早期启动功能逐步迁移"early userspace"（即 initramfs）
early userspace 迁移是必要的，因为查找并挂载真正的根设备十分复杂。根分区可以跨越多个设备（raid 或独立日志）。它们可以位于网络上（需dhcp、设置特MAC 地址、登录服务器等）。它们可以位于可移动介质上，带有动态分配的 major/minor 号以及持久命名问题，需要完整的 udev 实现来理顺。它们可以是压缩的、加密的、写时复制的、loopback 挂载的、以奇特方式分区的，等等
这类复杂性（不可避免地包含策略）应当在用户空间中妥善处理。klibc busybox/uClibc 都在开发可以放入内核构建的简initramfs 软件包
klibc 软件包现在已被接受进Andrew Morton 2.6.17-mm 树。内核当前的早期启动代码（分区检测等）很可能会被迁移到一个默认的 initramfs 中，由内核构建自动创建并使用