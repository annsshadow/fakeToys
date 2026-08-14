## /proc/sys/sunrpc/ 文档


内核版本 2.2.10

Copyright (c) 1998, 1999,  Rik van Riel <riel@nl.linux.org>

一般性说明与法律声明，请参见 index.rst。

------------------------------------------------------------------------------

本文件包含 /proc/sys/sunrpc 中 sysctl 文件的文档，适用于 Linux 内核版本 2.2。

该目录下的文件可用于（重新）设置 Linux 内核中 SUN 远程过程调用（RPC）子系统的调试标志。这些内容用于 NFS、KNFSD 以及可能还有其他一些功能。

其中的文件用于控制调试标志：
rpc_debug、nfs_debug、nfsd_debug 和 nlm_debug。

这些标志仅供内核开发者使用。更多信息请阅读 net/sunrpc/ 中的源代码。
