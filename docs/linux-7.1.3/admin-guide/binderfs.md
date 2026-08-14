## Android binderfs 文件系统


Android binderfs 是 Android binder IPC 机制所用的文件系统。它允许在运行时
动态添加和移除 binder 设备。位于新的 binderfs 实例中的 binder 设备独立于
其他 binderfs 实例中的 binder 设备。挂载一个新的 binderfs 实例可以获取一组
私有的 binder 设备。

### 挂载 binderfs


```
mkdir /dev/binderfs
mount -t binder binder /dev/binderfs
```
此时将在 `/dev/binderfs` 处出现一个新的 binderfs 实例。在全新的 binderfs
实例中不存在任何 binder 设备。只会有一个 `binder-control` 设备，作为
binderfs 的请求处理程序。在其他位置挂载另一个 binderfs 实例，将创建一个
独立于所有其他 binderfs 挂载的新实例。这与 `devpts` 和 `tmpfs`
等行为相同。Android binderfs 文件系统可以挂载在用户命名空间中。

### 选项

max
  binderfs 实例挂载时可对可分配的 binder 设备数量设置限制。`max=<count>`
  挂载选项充当每实例限制。如果设置了 `max=<count>`，则在此 binderfs
  实例中只能分配 `<count>` 个 binder 设备。

stats
  使用 `stats=global` 可启用全局 binder 统计信息。`stats=global` 仅适用于
  挂载在初始用户命名空间中的 binderfs 实例。尝试使用该选项挂载位于其他
  用户命名空间中的 binderfs 实例将返回权限错误。

### 分配 binder 设备


要在一个 binderfs 实例中分配新的 binder 设备，需要通过 `binder-control`
设备节点发送请求。请求以 `ioctl() <ioctl_>`_ 的形式发送。

程序需要做的是打开 `binder-control` 设备节点，并向内核发送一个
`BINDER_CTL_ADD` 请求。binderfs 的用户需要告诉内核新 binder 设备应取
什么名称。默认情况下，名称最多只能包含 `BINDERFS_MAX_NAME` 个字符
（含结尾的零字节）。

一旦通过 `ioctl() <ioctl_>`_ 将带有名称的 ``struct
binder_device`` 传递给内核发起请求，内核就会分配一个新的 binder 设备，
并在结构体中返回新设备的主、次设备号（这是必需的，因为 binderfs 会动态
分配主设备号）。`ioctl() <ioctl_>`_ 返回后，在 /dev/binderfs 下
将出现一个以所选名称命名的新 binder 设备。

### 删除 binder 设备


binderfs 的 binder 设备可通过 `unlink() <unlink_>`_ 删除。这意味着可以使用
`rm() <rm_>`_ 工具删除它们。注意 `binder-control` 设备无法被删除，因为
那样会使 binderfs 实例不可用。`binder-control` 设备会在 binderfs 实例
被卸载且对其的所有引用都被释放时被删除。

### binder 特性


假设已在 `/dev/binderfs` 挂载了一个 binderfs 实例，binder 驱动所支持的
特性可位于 `/dev/binderfs/features/` 下。可以通过测试各个文件的存在
来判断驱动是否支持某个特定特性。

```
cat /dev/binderfs/features/oneway_spam_detection
1
```
