
## Linux 内核用户空间 API 指南

本页是 Linux 内核用户空间 API 指南的索引，汇总内核代码树中记录的用户空间相关接口文档入口，按系统调用、安全接口、设备与 I/O 等主题分类，便于按主题快速定位。



虽然内核的大部分用户空间 API 记录在别处（尤其是在 man-pages_ 项目中），但在
内核代码树本身中也能找到一些用户空间相关信息。本手册意在成为汇集这些信息的
地方。


## 系统调用


- [unshare](unshare)
- [futex2](futex2)
- [ebpf/index](ebpf/index)
- [ioctl/index](ioctl/index)
- [mseal](mseal)
- [rseq](rseq)

## 与安全相关的接口


- [no_new_privs](no_new_privs)
- [seccomp_filter](seccomp_filter)
- [landlock](landlock)
- [lsm](lsm)
- [mfd_noexec](mfd_noexec)
- [spec_ctrl](spec_ctrl)
- [tee](tee)
- [check_exec](check_exec)

## 设备与 I/O


- [accelerators/ocxl](accelerators/ocxl)
- [dma-buf-heaps](dma-buf-heaps)
- [dma-buf-alloc-exchange](dma-buf-alloc-exchange)
- [fwctl/index](fwctl/index)
- [gpio/index](gpio/index)
- [iommufd](iommufd)
- [media/index](media/index)
- [dcdbas](dcdbas)
- [vduse](vduse)
- [isapnp](isapnp)

## 其他内容


- [ELF](ELF)
- [liveupdate](liveupdate)
- [netlink/index](netlink/index)
- [sysfs-platform_profile](sysfs-platform_profile)
- [vduse](vduse)
- [futex2](futex2)
- [perf_ring_buffer](perf_ring_buffer)
- [ntsync](ntsync)

