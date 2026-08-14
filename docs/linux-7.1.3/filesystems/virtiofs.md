

## virtiofs：virtio-fs 主机<->客户机共享文件系统


- Copyright (C) 2019 Red Hat, Inc.

## 简介


Linux 的 virtiofs 文件系统为半虚拟化（paravirtualized）的 VIRTIO "virtio-fs" 设备实现了一个驱动，用于客户机<->主机文件系统共享。它允许客户机挂载在主机上导出的一个目录。

客户机经常需要访问位于主机或远程系统上的文件。用例包括：在安装期间向新客户机提供文件、从位于主机上的根文件系统启动、为无状态或临时客户机提供持久存储，以及在客户机之间共享目录。

尽管可以使用现有的网络文件系统来完成其中部分任务，但它们需要难以自动化的配置步骤，并且会将存储网络暴露给客户机。virtio-fs 设备被设计成通过提供无需网络连接的文件系统访问来解决这些问题。

此外，virtio-fs 设备利用客户机和主机的同置（co-location）来提升性能，并提供网络文件系统所不可能实现的语义。

## 用法


挂载标签为 `myfs` 的文件系统到 `/mnt`：


  guest# mount -t virtiofs myfs /mnt

有关如何配置 QEMU 和 virtiofsd 守护进程的详细信息，请参阅 https://virtio-fs.gitlab.io/。

### 挂载选项


virtiofs 支持通用的 VFS 挂载选项，例如 remount、ro、rw、context 等。它也支持 FUSE 挂载选项。

##### atime 行为


与 atime 相关的挂载选项（例如 noatime、strictatime）会被忽略。virtiofs 的 atime 行为与主机上所导出目录的底层文件系统相同。

## 内部实现


由于 virtio-fs 设备使用 FUSE 协议处理文件系统请求，Linux 的 virtiofs 文件系统与 FUSE 文件系统客户端紧密集成。客户机充当 FUSE 客户端，而主机充当 FUSE 服务器。内核与用户空间之间的 /dev/fuse 接口被 virtio-fs 设备接口所取代。

FUSE 请求被放入 virtqueue 并由主机处理。缓冲区的响应部分由主机填充，客户机处理请求完成。

将 /dev/fuse 映射到 virtqueue 需要解决 /dev/fuse 与 virtqueue 之间语义上的差异。每次读取 /dev/fuse 设备时，FUSE 客户端可以选择传输哪个请求，从而可以优先处理某些请求。virtqueue 具有队列语义，无法改变已入队请求的顺序。这一点在 virtqueue 变满时尤为重要，因为此时无法再加入高优先级请求。为了解决这一差异，virtio-fs 设备使用一个专门的 "hiprio" virtqueue 来承载优先于普通请求的请求。
