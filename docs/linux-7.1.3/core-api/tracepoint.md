## Linux 内核 Tracepoint API


:Author: Jason Baron
:Author: William Cohen

## 简

Tracepoint 是位于内核各处关键点的静态探测点Probes' 通过回调机制
tracepoint 注册/注销probes' 是严格类型化的函数，接收每个
tracepoint 定义的唯一参数集
通过这一简单的回调机制probes' 可用于剖析、调试和理解内核行为。有若干
工具提供了使'probes' 的框架。这些工具包Systemtap、ftrace LTTng
Tracepoint 通过多种宏定义于多个头文件中。因此，本文档的目的是对可用
tracepoint 提供清晰的统计。意图不仅是理解有哪tracepoint 可用，还理解未来可能在何处添tracepoint
所呈现API 具有形如 `trace_tracepointname(function parameters)` 的函数这些是位于代码各处的 tracepoint 回调。向这些回调站点注册和注销 probes 内容`Documentation/trace/*` 目录中说明
## IRQ


   :internal:

## SIGNAL


   :internal:

## Block IO


   :internal:

## Workqueue


   :internal:
