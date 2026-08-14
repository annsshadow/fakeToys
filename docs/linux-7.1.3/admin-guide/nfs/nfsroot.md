## 通过 NFS 挂载根文件系统（nfsroot）

:Authors:
	Written 1996 by Gero Kuhlmann <gero@gkminix.han.de>

	Updated 1997 by Martin Mares <mj@atrey.karlin.mff.cuni.cz>

	Updated 2006 by Nico Schottelius <nico-kernel-nfsroot@schottelius.org>

	Updated 2006 by Horms <horms@verge.net.au>

	Updated 2018 by Chris Novakovic <chris@chrisn.me.uk>



为了使用一个无盘系统（例如 X 终端或打印服务器），根文件系统必须位于一个非磁盘设备上。这可以是一个 initramfs（参见 Documentation/filesystems/ramfs-rootfs-initramfs.rst）、一个 ramdisk（参见 Documentation/admin-guide/initrd.rst），或是一个通过 NFS 挂载的文件系统。下面的文字描述了如何使用 NFS 作为根文件系统。在本文的其余部分中，“client” 指无盘系统，“server” 指 NFS 服务器。



## Enabling nfsroot capabilities（启用 nfsroot 能力）

为了使用 nfsroot，需要在配置时将 NFS 客户端支持选为内建（built-in）。一旦选中了它，nfsroot 选项就会变得可用，该选项也应该被选中。

在网络选项中，可以选中内核级自动配置（kernel level autoconfiguration），以及要支持的自动配置类型。选中 DHCP、BOOTP 和 RARP 全部都是安全的。



## Kernel command line（内核命令行）

当内核被引导加载程序（见下文）加载后，需要告诉它要使用哪个根文件系统设备。而在 nfsroot 的情况下，还要告诉它去哪里找到服务器、以及服务器上要作为根挂载的目录名。这可以通过以下内核命令行参数来建立：

root=/dev/nfs
  这是启用伪 NFS 设备所必需的。注意它并非一个真实设备，而只是一个同义词，用来告诉内核使用 NFS 而非真实设备。

nfsroot=[<server-ip>:]<root-dir>[,<nfs-options>]
  如果命令行上没有给出 `nfsroot` 参数，将使用默认的 `"/tftpboot/%s"`。

  <server-ip>	指定 NFS 服务器的 IP 地址。
		默认地址由 ip 参数决定（见下文）。该参数允许为 IP 自动配置和 NFS 使用不同的服务器。

  <root-dir>	服务器上要作为根挂载的目录名。
		如果字符串中有一个 "%s" 记号，它将被替换为客户端 IP 地址的 ASCII 表示。

  <nfs-options>	标准 NFS 选项。所有选项以逗号分隔。
```

			port		= as given by server portmap daemon
			rsize		= 4096
			wsize		= 4096
			timeo		= 7
			retrans		= 3
			acregmin	= 3
			acregmax	= 60
			acdirmin	= 30
			acdirmax	= 60
			flags		= hard, nointr, noposix, cto, ac


```
ip=<client-ip>:<server-ip>:<gw-ip>:<netmask>:<hostname>:<device>:<autoconf>:<dns0-ip>:<dns1-ip>:<ntp0-ip>
  该参数告诉内核如何配置设备的 IP 地址，以及如何建立 IP 路由表。它最初被称为 nfsaddrs，但现在引导时的 IP 配置独立于 NFS 工作，因此被重命名为 ip，旧名称作为别名保留以兼容。

  如果该参数缺失于内核命令行，则所有字段都被假定为空，并适用下文中提到的默认值。一般而言，这意味着内核会尝试使用自动配置来配置一切。

  <autoconf> 参数可以单独作为 ip 参数的值出现（前面不带所有的 “:” 字符）。如果该值为 "ip=off" 或 "ip=none"，则不进行自动配置，否则将进行自动配置。最常用的用法是 "ip=dhcp"。

  <client-ip>	客户端的 IP 地址。
		默认：使用自动配置确定。

  <server-ip>	NFS 服务器的 IP 地址。
		如果使用 RARP 来确定客户端地址，且该参数非空，则只接受来自指定服务器的应答。

		仅在 NFS 根文件系统时才是必需的。也就是说，如果它缺失且 NFS 根未启用，则不会触发自动配置。

		该值会被导出到 /proc/net/pnp，前缀为 "bootserver "（见下文）。

		默认：使用自动配置确定。使用自动配置服务器的地址。

  <gw-ip>	如果服务器位于不同子网，则为网关的 IP 地址。
		默认：使用自动配置确定。

  <netmask>	本地网络接口的网掩码。
		如果未指定，则根据客户端 IP 地址（假设为有类地址）推导网掩码。

		默认：使用自动配置确定。

  <hostname>	客户端的名称。
		如果存在一个 "." 字符，第一个 "." 之前的内容用作客户端的主机名，其后的内容用作其 NIS 域名。可以由自动配置提供，但其缺失不会触发自动配置。
		如果指定了且使用了 DHCP，用户提供的 hostname（以及 NIS 域名，若存在）会被带入 DHCP 请求中；这可能会导致为客户端创建或更新一条 DNS 记录。

		默认：使用客户端 IP 地址的 ASCII 表示。

  <device>	要使用的网络设备的名称。
		默认：如果主机只有一个设备，则使用它。否则通过自动配置来确定该设备。做法是将自动配置请求从所有设备发出，并使用收到第一个应答的那个设备。

  <autoconf>	用于自动配置的方法。
		在指定了多个自动配置协议的情况下，会使用所有协议发送请求，并使用第一个应答的协议。

		只有编译进内核的自动配置协议才会被使用，而与该参数的值无关
```

                  off or none: don't use autoconfiguration
				(do static IP assignment instead)
		  on or any:   use any protocol available in the kernel
			       (default)
		  dhcp:        use DHCP
		  bootp:       use BOOTP
		  rarp:        use RARP
		  both:        use both BOOTP and RARP but not DHCP
		               (old option kept for backwards compatibility)

		如果使用 dhcp，可以按下述格式使用客户端标识符 "ip=dhcp,client-id-type,client-id-value"

                Default: any

  <dns0-ip>	主域名服务器的 IP 地址。
		该值会被导出到 /proc/net/pnp，前缀为 "nameserver "（见下文）。

		默认：不使用自动配置时为 None；使用自动配置时自动确定。

  <dns1-ip>	辅助域名服务器的 IP 地址。
		参见 <dns0-ip>。

  <ntp0-ip>	网络时间协议（NTP）服务器的 IP 地址。
		该值会被导出到 /proc/net/ipconfig/ntp_servers，除此之外未被使用（见下文）。

		默认：不使用自动配置时为 None；使用自动配置时自动确定。

  配置完成（无论是手动还是自动）后，会以下列格式创建两个文件；如果相应的值在配置后为空，则省略该行：

  - /proc/net/pnp:

	#PROTO: <DHCP|BOOTP|RARP|MANUAL>	(取决于配置方法)
	domain <dns-domain>			(若为自动配置，则为 DNS 域名)
	nameserver <dns0-ip>			(主域名服务器 IP)
	nameserver <dns1-ip>			(辅助域名服务器 IP)
	nameserver <dns2-ip>			(第三域名服务器 IP)
	bootserver <server-ip>			(NFS 服务器 IP)

  - /proc/net/ipconfig/ntp_servers:

	<ntp0-ip>				(NTP 服务器 IP)
	<ntp1-ip>				(NTP 服务器 IP)
	<ntp2-ip>				(NTP 服务器 IP)

  <dns-domain> 和 <dns2-ip>（位于 /proc/net/pnp 中）以及 <ntp1-ip> 和 <ntp2-ip>（位于 /proc/net/ipconfig/ntp_servers 中）是在自动配置期间请求的；它们不能作为 "ip=" 内核命令行参数的一部分来指定。

  由于 "domain" 和 "nameserver" 选项会被 DNS 解析器识别，在使用 NFS 根文件系统的系统上，/etc/resolv.conf 常常链接到 /proc/net/pnp。

  注意，内核不会与它发现的任何 NTP 服务器同步系统时间；这是用户空间进程的职责（例如，在挂载真正的根文件系统（如果它在 NFS 上）之前，将 /proc/net/ipconfig/ntp_servers 中列出的 IP 地址传递给一个 NTP 客户端的 initrd/initramfs 脚本）。


```
nfsrootdebug
  该参数使调试信息在内核引导时出现在内核日志中，以便管理员验证正确的 NFS 挂载选项、服务器地址和根路径被传递给了 NFS 客户端。


rdinit=<executable file>
  为了指定包含启动系统初始化程序的文件，管理员可以使用这个命令行参数。该参数的默认值是 "/init"。如果指定的文件存在且内核能够执行它，则与根文件系统相关的内核命令行参数（包括 'nfsroot='）都会被忽略。

  关于挂载根文件系统的过程的描述，可以在 Documentation/driver-api/early-userspace/early_userspace_support.rst 中找到。


## Boot Loader（引导加载程序）

要将内核载入内存，可以使用不同的方法。它们依赖于各种可用设施：

- Booting from a floppy using syslinux（使用 syslinux 从软盘引导）

	构建内核时，创建一个使用 syslinux 的引导软盘的一个简单方法是使用 zdisk 或 bzdisk make 目标，它们分别使用 zimage 和 bzimage 镜像。两个目标都接受 FDARGS 参数，可用于设置内核命令行。
```

	   make bzdisk FDARGS="root=/dev/nfs"

   	Note that the user running this command will need to have
     	access to the floppy drive device, /dev/fd0

     	For more information on syslinux, including how to create bootdisks
     	for prebuilt kernels, see https://syslinux.zytor.com/

	.. note::
		Previously it was possible to write a kernel directly to
		a floppy using dd, configure the boot device using rdev, and
		boot using the resulting floppy. Linux no longer supports this
		method of booting.

```
- Booting from a cdrom using isolinux（使用 isolinux 从光盘引导）

     	构建内核时，创建一个使用 isolinux 的可引导光盘的一个简单方法是使用 isoimage 目标，它使用 bzimage 镜像。与 zdisk 和 bzdisk 类似，该目标接受 FDARGS 参数，可用于设置内核命令行。
```

	  make isoimage FDARGS="root=/dev/nfs"

     	The resulting iso image will be arch/<ARCH>/boot/image.iso
     	This can be written to a cdrom using a variety of tools including
     	cdrecord.

	e.g::

	  cdrecord dev=ATAPI:1,0,0 arch/x86/boot/image.iso

     	For more information on isolinux, including how to create bootdisks
     	for prebuilt kernels, see https://syslinux.zytor.com/

```
- Using LILO（使用 LILO）

	使用 LILO 时，所有必要的命令行参数都可以使用 LILO 配置文件中的 'append=' 指令来指定。

	不过，要使用 'root=' 指令，您还需要创建一个虚拟的根设备，它可以在 LILO 运行后被移除。
```

	  mknod /dev/boot255 c 0 255

	关于配置 LILO 的信息，请参考其文档。
```

- Using GRUB（使用 GRUB）

	使用 GRUB 时，内核参数只需附加在内核说明之后：kernel <kernel> <parameters>

- Using loadlin（使用 loadlin）

	loadlin 可用于从 DOS 命令提示符引导 Linux，而无需本地硬盘作为根挂载。本文档的作者没有对其进行 thorough 测试，但一般而言，应当可以类似于 LILO 的配置方式来配置内核命令行。

	更多信息请参考 loadlin 文档。

- Using a boot ROM（使用引导 ROM）

	这可能是引导无盘客户端最优雅的方式。利用引导 ROM，内核通过 TFTP 协议加载。本文档的作者不知道有任何商业引导 ROM 支持通过网络引导 Linux。不过，有两个自由的引导 ROM 实现，netboot-nfs 和 etherboot，二者都可在 sunsite.unc.edu 上获得，且都包含引导无盘 Linux 客户端所需的一切。

- Using pxelinux（使用 pxelinux）

	Pxelinux 可用于利用许多现代网卡上存在的 PXE 引导加载程序来引导 Linux。

	使用 pxelinux 时，内核镜像通过 "kernel <relative-path-below /tftpboot>" 指定。nfsroot 参数通过将它们添加到 "append" 行来传递给内核。通常会配合使用串口控制台与 pxelinux，更多信息参见 Documentation/admin-guide/serial-console.rst。

	关于 isolinux 的更多信息，包括如何为预构建内核创建引导盘，请参见 https://syslinux.zytor.com/



## Credits（致谢）

 内核中的 nfsroot 代码以及 RARP 支持由 Gero Kuhlmann <gero@gkminix.han.de> 编写。

  其余的 IP 层自动配置代码由 Martin Mares <mj@atrey.karlin.mff.cuni.cz> 编写。

  为了编写 nfsroot 的初始版本，我要感谢 Jens-Uwe Mager <jum@anubis.han.de> 的帮助。
