
## 面向 Linux 内核开发者的调试建议


### 通用指南


- [driver_development_debugging_guide](driver_development_debugging_guide)
- [gdb-kernel-debugging](gdb-kernel-debugging)
- [kgdb](kgdb)
- [userspace_debugging_guide](userspace_debugging_guide)

### 子系统特定指

- [media_specific_debugging_guide](media_specific_debugging_guide)

## 通用调试建议


根据问题的不同，有一组不同的工具可用于追查问题，乃至首先确认是否存在问题
第一步，你必须弄清楚想要调试的是哪类问题。根据答案的不同，你的方法论工具选择可能会有所不同
### 我需要在受限访问下调试吗

你对机器的访问是否受限，或者无法停止正在运行的执行
在这种情况下，你的调试能力取决于所提供发行版内核的内建调试支持[/process/debugging/userspace_debugging_guide](/process/debugging/userspace_debugging_guide)
简要概述了这种情况下的若干可能调试工具。在大多数情况下，你可以通过查看
/boot 目录下的配置文件来检查你的内核能力
### 我有系统root 访问权限吗？


你是否能够轻松地替换相关模块或安装新内核
在这种情况下，你可用的工具范围要大得多，你可以在
[/process/debugging/driver_development_debugging_guide](/process/debugging/driver_development_debugging_guide)
中找到这些工具
### 时序是一个因素吗

重要的是要理解你想要调试的问题是一致地表现出来（即给定一组输入总是得到
相同、不正确的输出），还是不一致地表现出来。如果它不一致地表现出来，可有某个时序因素在起作用。如果在代码中插入延迟确实改变了行为，那么很可能
时序是一个因素
当时序确实改变了代码执行的输出时，使用简单的 printk() 进行调试可能无效一个类似的替代方案是使trace_printk()，它会将调试消息记录trace 文件
而非内核日志
**Copyright** 漏2024 : Collabora
