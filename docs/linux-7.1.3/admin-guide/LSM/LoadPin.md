## LoadPin


LoadPin 是一Linux 安全模块（LSM），它确保所有内核加载的文件
（模块、固件等）都来自同一个文件系统，
预期该文件系统由只读设备（如
dm-verity CDROM）提供支持。这使得已验
或不可变文件系统的系统能够在无需
对每个文件单独签名的情况下强制实施模块和固件加载限制

LSM 可在构建时通过 `CONFIG_SECURITY_LOADPIN` 选择，并
可在启动时通过内核命令行选项
"`loadpin.enforce`" 进行控制。默认情况下它处于启用状态，但可
启动时禁用（"`loadpin.enforce=0`"）

LoadPin 在看到第一个被加载的文件时开始固定（pinning）。如
支持该文件系统的块设备不是只读的，则会创建一sysctl 
切换固定状态：`/proc/sys/kernel/loadpin/enabled`。（拥有
可变文件系统意味着固定状态也是可变的，但拥有
sysctl 便于在可变文件系统上进行测试。）

也可以使用内核命令行选项 "`loadpin.exclude`" 将特定文件类型从 LoadPin 中排除
默认情况下包含所有文件，但可以使用诸
"`loadpin.exclude=kernel-module,kexec-image`" 的内核命令行选项将其排除。这允许使用
`CONFIG_MODULE_SIG` 鍜。
`CONFIG_KEXEC_VERIFY_SIG` 等不同机制来验证内核模块和内核镜像，同时
仍使LoadPin 来保护内核加载的其他文件的完整性。完
有效文件类型列表可在 `kernel_read_file_str` 中找到，
其定义位`include/linux/kernel_read_file.h`
