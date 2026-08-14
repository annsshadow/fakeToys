
## 通过 SMB 挂载根文件系统（cifs.ko）


Written 2019 by Paulo Alcantara <palcantara@suse.de>

Written 2019 by Aurelien Aptel <aaptel@suse.com>

CONFIG_CIFS_ROOT 选项通过 cifs.ko 启用基于 SMB 协议的根文件系统实验性支持。

它引入了一个名为 'cifsroot=' 的新内核命令行选项，用于告诉内核通过网络利用 SMB 或 CIFS 协议挂载根文件系统。

为了进行挂载，还需要使用 'ip=' 配置选项来建立网络栈。更多细节，请参阅
Documentation/admin-guide/nfs/nfsroot.rst。

CIFS 根挂载目前需要使用 SMB1+UNIX 扩展，该扩展仅由 Samba 服务器支持。SMB1 是该协议的较旧且已被弃用的版本，但它已被扩展以支持 POSIX 特性（参见 [^1^]）。新版本（推荐的协议版本 SMB3）的等效扩展尚未完全实现，这意味着 SMB3 不支持某些必需的 POSIX 文件系统对象（例如块设备、管道、套接字）。

因此，CIFS 根目前默认使用 SMB1，但所使用的版本仍可通过 'vers=' 挂载选项更改。一旦 SMB3 POSIX 扩展完全实现，该默认值将会改变。

## 服务器配置


要启用 SMB1+UNIX 扩展，你需要设置这些全局
```

    [global]
    server min protocol = NT1
    unix extension = yes        # default

```
## 内核命令行


```

    root=/dev/cifs

```
这只是一个虚拟设备，基本上告诉内核通过 SMB 协议挂载根文件系统。

```

    cifsroot=//<server-ip>/<share>[,options]

```
使内核能够挂载位于本选项中指定的 <server-ip> 和 <share> 中、通过 SMB 提供的根文件系统。

默认挂载选项设置在 fs/smb/client/cifsroot.c 中。

server-ip
	服务器的 IPv4 地址。

share
	SMB 共享（rootfs）的路径。

options
	可选的挂载选项。更多信息，请参阅 mount.cifs(8)。

## 示例


```

    ...
    [linux]
	    path = /path/to/rootfs
	    read only = no
	    guest ok = yes
	    force user = root
	    force group = root
	    browseable = yes
	    writeable = yes
	    admin users = root
	    public = yes
	    create mask = 0777
	    directory mask = 0777
    ...

```
```

    # systemctl restart smb

```
在启用了 CONFIG_CIFS_ROOT 的内核下使用 QEMU 进行测试，以及
```

    # qemu-system-x86_64 -enable-kvm -cpu host -m 1024 \
    -kernel /path/to/linux/arch/x86/boot/bzImage -nographic \
    -append "root=/dev/cifs rw ip=dhcp cifsroot=//10.0.2.2/linux,username=foo,password=bar console=ttyS0 3"


```
1: https://wiki.samba.org/index.php/UNIX_Extensions
