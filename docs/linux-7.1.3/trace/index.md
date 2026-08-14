## Linux 追踪技术指南


Linux 内核中的追踪是一种强大的机制，它允许开发人员与系统管理员分析并调试
系统行为。本指南提供关于 Linux 内核中各类追踪框架与工具的文档。

### 追踪简介


本节概述 Linux 追踪机制与调试方法。

- [debugging](debugging)
- [tracepoints](tracepoints)
- [tracepoint-analysis](tracepoint-analysis)
- [ring-buffer-map](ring-buffer-map)

### 核心追踪框架


以下是集成到 Linux 内核中的主要追踪框架。

- [ftrace](ftrace)
- [ftrace-design](ftrace-design)
- [ftrace-uses](ftrace-uses)
- [kprobes](kprobes)
- [kprobetrace](kprobetrace)
- [fprobetrace](fprobetrace)
- [eprobetrace](eprobetrace)
- [fprobe](fprobe)
- [ring-buffer-design](ring-buffer-design)

### 事件追踪与分析


对事件追踪机制及其应用的详细说明。

- [events](events)
- [events-kmem](events-kmem)
- [events-power](events-power)
- [events-nmi](events-nmi)
- [events-msr](events-msr)
- [events-pci](events-pci)
- [events-pci-controller](events-pci-controller)
- [boottime-trace](boottime-trace)
- [histogram](histogram)
- [histogram-design](histogram-design)

### 硬件与性能追踪


本节涵盖监控硬件交互与系统性能的追踪特性。

- [intel_th](intel_th)
- [stm](stm)
- [sys-t](sys-t)
- [coresight/index](coresight/index)
- [rv/index](rv/index)
- [hisi-ptt](hisi-ptt)
- [mmiotrace](mmiotrace)
- [hwlat_detector](hwlat_detector)
- [osnoise-tracer](osnoise-tracer)
- [timerlat-tracer](timerlat-tracer)

### 用户空间追踪


这些工具可用于追踪用户空间应用程序及其交互。

- [user_events](user_events)
- [uprobetracer](uprobetracer)

### 远程追踪


本节涵盖用于读取兼容环形缓冲区的框架，这些缓冲区由内核之外的实体（最可能是
固件或虚拟机超级管理器）写入。

- [remotes](remotes)

### 附加资源


更多细节，请参阅各追踪工具与框架各自的文档。
