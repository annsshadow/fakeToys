## 设置 NFS/RDMA


:Author:
  NetApp and Open Grid Computing (May 29, 2008)

  本文档可能已经过时。

## 概述


本文档描述了如何安装和配置 Linux NFS/RDMA 客户端与服务端软件。

NFS/RDMA 客户端首次包含在 Linux 2.6.24 中。NFS/RDMA 服务端首次包含在此后的下一个版本 Linux 2.6.25 中。

在我们的测试中，在多种工作负载下都获得了出色的性能结果（在客户端 CPU 占用极低的情况下达到满 10Gbit 线路带宽）。该代码通过了完整的 Connectathon 测试套件，并且可以在 Infiniband 与 iWARP 两种 RDMA 适配器上运行。

## 获取帮助


如果你遇到了困难，可以在 nfs-rdma-devel@lists.sourceforge.net 邮件列表上提问。

## 安装


以下说明是构建一台用于 NFS/RDMA 的机器的分步指南。

- 安装 RDMA 设备

  只要是 drivers/infiniband/hw 中的驱动所支持的任何设备都可以。

  我们已经使用多个基于 Mellanox 的 IB 网卡、Ammasso AMS1100 iWARP 适配器以及 Chelsio cxgb3 iWARP 适配器进行了测试。

- 安装 Linux 发行版及工具

  首个同时包含 NFS/RDMA 客户端与服务端的 kernel 版本是 Linux 2.6.25，因此应当安装与此及后续 Linux 内核版本兼容的发行版。

  本文档描述的步骤已在 Red Hat 的 Fedora Project（http://fedora.redhat.com/）发行版上测试过。

- 在客户端上安装 nfs-utils-1.1.2 或更高版本

  使用 nfs-utils-1.1.2 或更高版本中的 mount.nfs 命令（nfs-utils-1.1.1 是首个支持 NFS/RDMA 挂载的 nfs-utils 版本，但出于各种原因我们建议使用 nfs-utils-1.1.2 或更高版本）即可获得 NFS/RDMA 挂载点。要查看你正在使用的 mount.nfs 版本，请输入：

  .. code-block:: sh

    $ /sbin/mount.nfs -V

  如果版本低于 1.1.2 或者该命令不存在，你应当安装最新版本的 nfs-utils。

  从以下地址下载最新的软件包：https://www.kernel.org/pub/linux/utils/nfs

  解压该软件包并按照安装说明进行操作。

  如果你不需要 idmapper 和 gssd 可执行文件（创建启用 NFS/RDMA 的挂载命令并不需要它们），则可以在运行 configure 时禁用这些特性来简化安装过程：

  .. code-block:: sh

    $ ./configure --disable-gss --disable-nfsv4

  要构建 nfs-utils，你需要安装 tcp_wrappers 软件包。有关更多信息，请参阅该软件包的 README 和 INSTALL 文件。

  构建 nfs-utils 软件包后，在 utils/mount 目录下会有一个 mount.nfs 二进制文件。该二进制文件可用于发起 NFS v2、v3 或 v4 挂载。要发起 v4 挂载，该二进制文件必须被命名为 mount.nfs4。标准做法是将一个名为 mount.nfs4 的符号链接指向 mount.nfs。

  应当将该 mount.nfs 二进制文件按如下方式安装到 /sbin/mount.nfs：

  .. code-block:: sh

    $ sudo cp utils/mount/mount.nfs /sbin/mount.nfs

  在该位置，系统 mount 命令会自动调用 mount.nfs 来进行 NFS 挂载。

```
      mount.nfs 以及 nfs-utils-1.1.2 或更高版本只需要在 NFS 客户端机器上安装。
      服务端上并不需要这个特定版本的 nfs-utils。此外，客户端上只需要 nfs-utils-1.1.2
      中的 mount.nfs 命令。

```
- 安装带有 NFS/RDMA 的 Linux 内核

  NFS/RDMA 客户端与服务端都包含在主线 Linux 内核版本 2.6.25 及之后。此版本及其他版本的 Linux 内核可在以下地址获取：https://www.kernel.org/pub/linux/kernel/

  下载源码并将其放置到合适的位置。

- 配置 RDMA 栈

  确保你的内核配置已启用 RDMA 支持。在 Device Drivers -> InfiniBand support 下，更新内核配置以启用 InfiniBand support [注意：该选项名称具有误导性。启用 InfiniBand support 对于所有 RDMA 设备（IB、iWARP 等）都是必需的]。

  启用相应的 IB HCA 支持（mlx4、mthca、ehca、ipath 等）或 iWARP 适配器支持（amso、cxgb3 等）。

  如果你使用的是 InfiniBand，请务必启用 IP-over-InfiniBand 支持。

- 配置 NFS 客户端与服务端

  你的内核配置还必须启用 NFS 文件系统支持和/或 NFS 服务端支持。这些以及其它 NFS 相关的配置选项可以在 File Systems -> Network File Systems 下找到。

- 构建、安装、重启

  如果 NFS 和 RDMA 均已开启，NFS/RDMA 代码将自动启用。NFS/RDMA 客户端与服务端是通过依赖于 SUNRPC 和 INFINIBAND 的隐藏配置选项 SUNRPC_XPRT_RDMA 进行配置的。SUNRPC_XPRT_RDMA 的值将为：

    #. 如果 SUNRPC 或 INFINIBAND 任一为 N，则值为 N，此时 NFS/RDMA 客户端与服务端将不会被构建

    #. 如果 SUNRPC 和 INFINIBAND 都已开启（M 或 Y）且至少有一个为 M，则值为 M，此时 NFS/RDMA 客户端与服务端将被构建为模块

    #. 如果 SUNRPC 和 INFINIBAND 都为 Y，则值为 Y，此时 NFS/RDMA 客户端与服务端将被构建进内核

  因此，如果你已按照上述步骤开启 NFS 和 RDMA，NFS/RDMA 客户端与服务端就会被构建。

  构建新内核，安装它，并启动它。

## 检查 RDMA 与 NFS 的安装


在配置 NFS/RDMA 软件之前，测试一下你的新内核以确保其工作正常是个好主意。特别是，验证 RDMA 栈是否按预期运行，以及基于 TCP/IP 和/或 UDP/IP 的标准 NFS 是否正常工作，都是好做法。

- 检查 RDMA 安装

  如果你将 RDMA 组件构建为模块，此时加载它们。例如，如果你使用的是 Mellanox Tavor/Sinai/Arbel 网卡：

  .. code-block:: sh

    $ modprobe ib_mthca
    $ modprobe ib_ipoib

  如果你使用的是 InfiniBand，请确保网络上正在运行一个子网管理器（SM）。如果你的 IB 交换机带有内嵌的 SM，可以使用它。否则，你将需要在某个终端节点上运行一个 SM，例如 OpenSM。

  如果你的网络上运行着 SM，你应该看到如下输出：

  .. code-block:: sh

    $ cat /sys/class/infiniband/driverX/ports/1/state
    4: ACTIVE

  其中 driverX 为 mthca0、ipath5、ehca3 等。

  要进一步测试 InfiniBand 软件栈，可以使用 IPoIB（这假设你有两台名为 host1 和 host2 的 IB 主机）：

  .. code-block:: sh

    host1$ ip link set dev ib0 up
    host1$ ip address add dev ib0 a.b.c.x
    host2$ ip link set dev ib0 up
    host2$ ip address add dev ib0 a.b.c.y
    host1$ ping a.b.c.y
    host2$ ping a.b.c.x

  对于其它设备类型，请遵循相应的步骤。

- 检查 NFS 安装

  对于上面启用的 NFS 组件（客户端和/或服务端），在标准以太网（使用 TCP/IP 或 UDP/IP）上测试它们的功能。

## NFS/RDMA 配置


我们建议你使用两台机器，一台作为客户端，一台作为服务端。

### 一次性配置：


- 在服务端系统上，配置 /etc/exports 文件并启动 NFS/RDMA 服务端。

```
  /vol0   192.168.0.47(fsid=0,rw,async,insecure,no_root_squash)
  /vol0   192.168.0.0/255.255.255.0(fsid=0,rw,async,insecure,no_root_squash)

  IP 地址是客户端的 IPoIB 地址（对于 InfiniBand HCA）或客户端的 iWARP 地址（对于 RNIC）。

  .. note::
    必须使用 "insecure" 选项，因为 NFS/RDMA 客户端不使用保留端口。

```
### 每次开机时：


- 加载并配置 RDMA 驱动

  对于使用 Mellanox 适配器的 InfiniBand：

  .. code-block:: sh

    $ modprobe ib_mthca
    $ modprobe ib_ipoib
    $ ip li set dev ib0 up
    $ ip addr add dev ib0 a.b.c.d

```
    请为客户端与服务端使用唯一的地址！

```
- 启动 NFS 服务端

  如果 NFS/RDMA 服务端被构建为模块（内核配置中 CONFIG_SUNRPC_XPRT_RDMA=m），则加载 RDMA 传输模块：

  .. code-block:: sh

    $ modprobe svcrdma

  无论服务端以何种方式构建（模块或内建），启动服务端：

  .. code-block:: sh

    $ /etc/init.d/nfs start

  或

  .. code-block:: sh

    $ service nfs start

  指示服务端监听 RDMA 传输：

  .. code-block:: sh

    $ echo rdma 20049 > /proc/fs/nfsd/portlist

- 在客户端系统上

  如果 NFS/RDMA 客户端被构建为模块（内核配置中 CONFIG_SUNRPC_XPRT_RDMA=m），加载 RDMA 客户端模块：

  .. code-block:: sh

    $ modprobe xprtrdma.ko

  无论客户端以何种方式构建（模块或内建），使用以下命令挂载 NFS/RDMA 服务端：

  .. code-block:: sh

    $ mount -o rdma,port=20049 <IPoIB-server-name-or-address>:/<export> /mnt

  要验证该挂载是否正在使用 RDMA，请运行 "cat /proc/mounts" 并检查该挂载的 "proto" 字段。

  恭喜！你正在使用 NFS/RDMA！
