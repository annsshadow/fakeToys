## /proc/sys/abi/ 文档


Copyright (c) 2020, Stephen Kitt

一般性信息，请参Documentation/admin-guide/sysctl/index.rst

------------------------------------------------------------------------------

`/proc/sys/abi` 中的文件可用于查看和修改ABI 相关的设置

目前，这些文件可能（取决于你的配置）出现`/proc/sys/kernel` 中：


## vsyscall32（x86


确定内核是否vDSO 页面映射32 位进程中；可设为 1 启用，或设为 0 禁用。若设置`CONFIG_COMPAT_VDSO` 则默认启用，否则默认禁用

这与 `vdso32` 内核启动参数控制的设置相同
