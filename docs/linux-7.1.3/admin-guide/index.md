
## Linux 内核用户与管理员指南


下面是一组随时间添加到内核中的、面向用户的文档的集合。到目前为止，这里还
几乎没有整体的顺序或组织——这些材料原本就不是作为单一、连贯的文档来编写的！
希望随着时间的推移，情况会迅速改善。

### 内核管理通用指南


这个初始部分包含总体信息，包括描述整个内核的 README 文件、关于内核参数的
文档等。

- [README](README)
- [devices](devices)
- [features](features)

内核管理接口的一大部分是 /proc 和 sysfs 虚拟文件系统；这些文档描述了如何与
tem 进行交互

- [sysfs-rules](sysfs-rules)
- [sysctl/index](sysctl/index)
- [cputopology](cputopology)
- [abi](abi)

与安全相关的文档：

- [hw-vuln/index](hw-vuln/index)
- [LSM/index](LSM/index)
- [perf-security](perf-security)

### 引导内核


- [bootconfig](bootconfig)
- [kernel-parameters](kernel-parameters)
- [efi-stub](efi-stub)
- [initrd](initrd)

### 追踪与识别问题


下面这组文档面向的是那些试图追踪特定问题与缺陷的用户。

- [reporting-issues](reporting-issues)
- [reporting-regressions](reporting-regressions)
- [quickly-build-trimmed-linux](quickly-build-trimmed-linux)
- [verify-bugs-and-bisect-regressions](verify-bugs-and-bisect-regressions)
- [bug-hunting](bug-hunting)
- [bug-bisect](bug-bisect)
- [tainted-kernels](tainted-kernels)
- [ramoops](ramoops)
- [dynamic-debug-howto](dynamic-debug-howto)
- [init](init)
- [kdump/index](kdump/index)
- [perf/index](perf/index)
- [pstore-blk](pstore-blk)
- [clearing-warn-once](clearing-warn-once)
- [kernel-per-CPU-kthreads](kernel-per-CPU-kthreads)
- [lockup-watchdogs](lockup-watchdogs)
- [RAS/index](RAS/index)
- [sysrq](sysrq)

### 核心内核子系统


这些文档描述了核心内核的管理接口，它们在几乎所有系统上都可能有用。

- [cgroup-v2](cgroup-v2)
- [cgroup-v1/index](cgroup-v1/index)
- [cpu-isolation](cpu-isolation)
- [cpu-load](cpu-load)
- [mm/index](mm/index)
- [module-signing](module-signing)
- [namespaces/index](namespaces/index)
- [numastat](numastat)
- [pm/index](pm/index)
- [syscall-user-dispatch](syscall-user-dispatch)

对非原生二进制格式的支持。请注意，其中部分文档……已经比较陈旧……

- [binfmt-misc](binfmt-misc)
- [java](java)
- [mono](mono)

### 块层与文件系统管理


- [bcache](bcache)
- [binderfs](binderfs)
- [blockdev/index](blockdev/index)
- [cifs/index](cifs/index)
- [device-mapper/index](device-mapper/index)
- [ext4](ext4)
- [filesystem-monitoring](filesystem-monitoring)
- [nfs/index](nfs/index)
- [iostats](iostats)
- [jfs](jfs)
- [md](md)
- [ufs](ufs)
- [xfs](xfs)

### 设备相关指南


如何在 Linux 系统中配置你的硬件。

- [acpi/index](acpi/index)
- [aoe/index](aoe/index)
- [auxdisplay/index](auxdisplay/index)
- [braille-console](braille-console)
- [btmrvl](btmrvl)
- [dell_rbu](dell_rbu)
- [edid](edid)
- [gpio/index](gpio/index)
- [hw_random](hw_random)
- [laptops/index](laptops/index)
- [lcd-panel-cgram](lcd-panel-cgram)
- [media/index](media/index)
- [nvme-multipath](nvme-multipath)
- [parport](parport)
- [pnp](pnp)
- [rapidio](rapidio)
- [rtc](rtc)
- [serial-console](serial-console)
- [svga](svga)
- [thermal/index](thermal/index)
- [thunderbolt](thunderbolt)
- [vga-softcursor](vga-softcursor)
- [video-output](video-output)

### 工作负载分析


这是面向应用程序开发人员与系统集成人员、用于对 Linux 内核进行安全关键型应用
分析的章节的开端。这里会收录支持分析内核与应用程序交互、以及关键内核子系统
预期的文档。

- [workload-tracing](workload-tracing)

### 其他内容


一些难以归类且通常已过时的文档。

- [ldm](ldm)
- [unicode](unicode)
