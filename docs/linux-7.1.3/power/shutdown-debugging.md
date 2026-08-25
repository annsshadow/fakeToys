
使用 pstore 调试内核关机挂起
+++++++++++++++++++++++++++++++++++++++++++

## 概述


如果系统在关机时挂起，可能需要获取内核日
以调试该问题

在有可用 UART 的系统中，最好将内核配置为使用该
UART 作为内核控制台输出

如果没有可用UART，`pstore` 子系统提供了一种机制，
在系统复位时持久化这些数据，从而在下次
启动时获取

## 内核配置


要启`pstore` 并保存内核环形缓冲区日志，请设置
以下内核配置选项

- `CONFIG_PSTORE=y`
- `CONFIG_PSTORE_CONSOLE=y`

此外，需启用一个后端来存储数据。根据你的平台，
一些可选方案包括：

- `CONFIG_EFI_VARS_PSTORE=y`
- `CONFIG_PSTORE_RAM=y`
- `CONFIG_CHROMEOS_PSTORE=y`
- `CONFIG_PSTORE_BLK=y`

## 内核命令行参


将这些参数添加到你的内核命令行：

- `printk.always_kmsg_dump=Y`
 - 强制内核在关机期间将整个消息缓冲区转储到 pstore
		shutdown
- `efi_pstore.pstore_disable=N`
 - 对于基于 EFI 的系统，确保 EFI 后端处于活动状

## 用户空间交互与日志获


在挂起后的下次启动时，pstore 日志将位pstore
文件系统（`/sys/fs/pstore`）中，并可由用户空间获取

systemd 系统中，`systemd-pstore` 服务将帮助完成以下操作：

#. `/sys/fs/pstore` 中定pstore 数据
#. 将其读取并保存到 `/var/lib/systemd/pstore`
#. 为下一次事件清pstore 数据
