## GPU 调试


## 通用调试选项


DebugFS 一节提供了若干文件的文档，用于辅助调试 GPU 上的问题

## GPUVM 调试


为了辅助调试 GPU 虚拟内存相关的问题，驱动支持以下若干模块参数选项
`vm_fault_stop` - 若非 0，则在发GPU 页错误时停止 GPU 内存控制器
`vm_update_mode` - 若非 0，则使用 CPU 而非 GPU 来更GPU 页表

## 解码一GPUVM 页错

如果你在内核日志中看到一GPU 页错误，可以对其进行解码，以弄清你的应用程序中发生了什么问题。内核日志中的一次页错误可能类似如下内容
```

 [gfxhub0] no-retry page fault (src_id:0 ring:24 vmid:3 pasid:32777, for process glxinfo pid 2424 thread glxinfo:cs0 pid 2425)
   in page starting at address 0x0000800102800000 from IH client 0x1b (UTCL2)
 VM_L2_PROTECTION_FAULT_STATUS:0x00301030
 	Faulty UTCL2 client ID: TCP (0x8)
 	MORE_FAULTS: 0x0
 	WALKER_ERROR: 0x0
 	PERMISSION_FAULTS: 0x3
 	MAPPING_ERROR: 0x0
 	RW: 0x0

```
首先是内存枢纽（memory hub），gfxhub mmhub。gfxhub 是用于图形、计算和某些芯片sdma 的内存枢纽。mmhub 是用于多媒体和某些芯片上 sdma 的内存枢纽
接下来是 vmid pasid。如vmid 0，则该错误很可能由内核驱动或固件引起。如vmid 0，则通常是用户应用程序中的错误。pasid 用于vmid 关联到系统的进程 id。如果错误发生时该进程处于活动状态，则会打印进程信息
引起该错误的 GPU 虚拟地址紧随其后
客户ID 指明了引发错误的 GPU 模块。一些常见的客户ID
- CB/DB：图形管线中的颜深度后端
- CPF：命令处理器前端（Command Processor Frontend- CPC：命令处理器计算（Command Processor Compute- CPG：命令处理器图形（Command Processor Graphics- TCP/SQC/SQG：着色器（Shaders- SDMA：SDMA 引擎
- VCN：视频编解码引擎
- JPEG：JPEG 引擎

PERMISSION_FAULTS 描述了遇到了哪些错误
- bit 0：PTE 无效
- bit 1：PTE 读位未设- bit 2：PTE 写位未设- bit 3：PTE 执行位未设置

最后，RW 指示该访问是读（0）还是写）
在上面的示例中，一个着色器（客户端 id = TCP）对 GPU 虚拟地址 0x0000800102800000 处的无效页（PERMISSION_FAULTS = 0x3）发起了一次读访问（RW = 0x0）。随后用户可以检查其着色器代码和资源描述符状态，以确定是什么导致了GPU 页错误
## UMR


`umr <https://gitlab.freedesktop.org/tomstdenis/umr>`_ 是一个通用GPU 调试与诊断工具。有关其能力的更多信息，请参umr `文档 <https://umr.readthedocs.io/en/main/>`_
## 调试背光亮度

默认背光亮度应经由固件所通告的策略来设置。固件通常会为交流（AC）或直流（DC）供电提供不同的默认值。此外，某些用户空间软件会在上一次启动时保存背光亮度，并尝试恢复它
某些固件还支持一项称为“Custom Backlight Curves（自定义背光曲线）”的功能，在该功能中将亮度输入值沿一条与显示特性更匹配的亮度值线性插值曲线进行映射
在背光出现问题时，有一个可在启动时启用trace 事件，用于记录每一次亮度变更请求。这有助于定位问题所在。要启用trace 事件，请在命令行中添加如下内容：

  tp_printk trace_event=amdgpu_dm:amdgpu_dm_brightness:mod:amdgpu trace_buf_size=1M
