
## SMB Direct - 基于 RDMA 的 SMB3


本文档介绍如何将 Linux 的 SMB 客户端与服务器配置为使用 RDMA。

## 概述

Linux SMB 内核客户端支持 SMB Direct，这是 SMB3 的一种传输方案，它使用 RDMA（远程直接内存访问）绕过传统的 TCP/IP 协议栈，从而提供高吞吐量和低延迟。
Linux SMB 客户端上的 SMB Direct 可以针对 KSMBD（一个内核态 SMB 服务器）进行测试。

## 安装

- 安装一个 RDMA 设备。只要该 RDMA 设备驱动被内核支持，即可工作。这包括软件模拟器（soft RoCE、soft iWARP）和硬件设备（InfiniBand、RoCE、iWARP）。

- 安装一个支持 SMB Direct 的内核。首个在客户端和服务器端均支持 SMB Direct 的内核版本是 5.15。因此，需要使用与内核 5.15 或更高版本兼容的发行版。

- 安装 cifs-utils，它提供用于挂载 SMB 共享的 `mount.cifs` 命令。

- 配置 RDMA 协议栈

  请确保你的内核配置已启用 RDMA 支持。在 Device Drivers -> Infiniband support 下，更新内核配置以启用 Infiniband 支持。

  根据你的硬件，启用相应的 IB HCA 支持或 iWARP 适配器支持。

  如果你使用的是 InfiniBand，请启用 IP-over-InfiniBand 支持。

  对于软 RDMA，请启用 soft iWARP（`RDMA _SIW`）或 soft RoCE（`RDMA_RXE`）模块。安装 `iproute2` 软件包，并使用 `rdma link add` 命令加载模块并创建 RDMA 接口。

  例如，如果你的本地以太网接口是 `eth0`，可以使用：

    .. code-block:: bash

        sudo rdma link add siw0 type siw netdev eth0

- 在内核配置中为服务器和客户端同时启用 SMB Direct 支持。

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

- 编译并安装内核。SMB Direct 支持将被编入 cifs.ko 和 ksmbd.ko 模块。

## 配置与使用


- 按照 `KSMBD 文档 <https://www.kernel.org/doc/Documentation/filesystems/smb/ksmbd.rst>`_ 中所述搭建并启动一个 KSMBD 服务器。同时在 ksmbd.conf 中添加 "server multi channel support = yes" 参数。

- 在客户端上，使用 `rdma` 挂载选项挂载共享以使用 SMB Direct（通过 `vers` 指定 SMB 3.0 或更高版本）。

  例如：

    .. code-block:: bash

        mount -t cifs //server/share /mnt/point -o vers=3.1.1,rdma

- 要验证挂载是否正在使用 SMB Direct，可在挂载后检查 dmesg 中是否出现以下日志行：

    .. code-block:: text

        CIFS: VFS: RDMA transport established

  或者，在 `/proc/mounts` 中验证该共享的 `rdma` 挂载选项：

    .. code-block:: bash

        cat /proc/mounts | grep cifs
