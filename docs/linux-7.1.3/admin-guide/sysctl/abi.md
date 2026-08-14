## /proc/sys/abi/ 文档


Copyright (c) 2020, Stephen Kitt

一般性信息，请参见 Documentation/admin-guide/sysctl/index.rst。

------------------------------------------------------------------------------

`/proc/sys/abi` 中的文件可用于查看和修改与 ABI 相关的设置。

目前，这些文件可能（取决于你的配置）出现在 `/proc/sys/kernel` 中：


## vsyscall32（x86）


确定内核是否将 vDSO 页面映射到 32 位进程中；可设为 1 启用，或设为 0 禁用。若设置了 `CONFIG_COMPAT_VDSO` 则默认启用，否则默认禁用。

这与 `vdso32` 内核启动参数控制的设置相同。
