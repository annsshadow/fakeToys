
## SMB Direct - 基于 RDMA SMB3


本文档介绍如何将 Linux SMB 客户端与服务器配置为使用 RDMA
## 概述

Linux SMB 内核客户端支SMB Direct，这SMB3 的一种传输方案，它使RDMA（远程直接内存访问）绕过传统TCP/IP 协议栈，从而提供高吞吐量和低延迟Linux SMB 客户端上SMB Direct 可以针对 KSMBD（一个内核SMB 服务器）进行测试
## 安装

- 安装一RDMA 设备。只要该 RDMA 设备驱动被内核支持，即可工作。这包括软件模拟器（soft RoCE、soft iWARP）和硬件设备（InfiniBand、RoCE、iWARP）
- 安装一个支SMB Direct 的内核。首个在客户端和服务器端均支SMB Direct 的内核版本是 5.15。因此，需要使用与内核 5.15 或更高版本兼容的发行版
- 安装 cifs-utils，它提供用于挂载 SMB 共享`mount.cifs` 命令
- 配置 RDMA 协议
  请确保你的内核配置已启用 RDMA 支持。在 Device Drivers -> Infiniband support 下，更新内核配置以启Infiniband 支持
  根据你的硬件，启用相应的 IB HCA 支持iWARP 适配器支持
  如果你使用的InfiniBand，请启用 IP-over-InfiniBand 支持
  对于RDMA，请启用 soft iWARP（`RDMA _SIW`）或 soft RoCE（`RDMA_RXE`）模块。安`iproute2` 软件包，并使`rdma link add` 命令加载模块并创RDMA 接口
  例如，如果你的本地以太网接口`eth0`，可以使用：

    .. code-block:: bash

        sudo rdma link add siw0 type siw netdev eth0

- 在内核配置中为服务器和客户端同时启用 SMB Direct 支持
    Server Setup

    .. code-block:: text

        Network File Systems  --->
            <M> SMB3 server support
                [*] Support for SMB Direct protocol

    Client Setup

    .. code-block:: text

        Network File Systems  --->
            <M> SMB3 and CIFS support (advanced network filesystem)
                [*] SMB Direct support

- 编译并安装内核。SMB Direct 支持将被编入 cifs.ko ksmbd.ko 模块
## 配置与使

- 按照 `KSMBD 文档 <https://www.kernel.org/doc/Documentation/filesystems/smb/ksmbd.rst>`_ 中所述搭建并启动一KSMBD 服务器。同时在 ksmbd.conf 中添"server multi channel support = yes" 参数
- 在客户端上，使用 `rdma` 挂载选项挂载共享以使SMB Direct（通过 `vers` 指定 SMB 3.0 或更高版本）
  例如
    .. code-block:: bash

        mount -t cifs //server/share /mnt/point -o vers=3.1.1,rdma

- 要验证挂载是否正在使SMB Direct，可在挂载后检dmesg 中是否出现以下日志行
    .. code-block:: text

        CIFS: VFS: RDMA transport established

  或者，`/proc/mounts` 中验证该共享`rdma` 挂载选项
    .. code-block:: bash

        cat /proc/mounts | grep cifs
