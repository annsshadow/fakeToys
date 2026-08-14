## Linux 内核 Tracepoint API


:Author: Jason Baron
:Author: William Cohen

## 简介


Tracepoint 是位于内核各处关键点的静态探测点。'Probes' 通过回调机制
向 tracepoint 注册/注销。'probes' 是严格类型化的函数，接收每个
tracepoint 定义的唯一参数集。

通过这一简单的回调机制，'probes' 可用于剖析、调试和理解内核行为。有若干
工具提供了使用 'probes' 的框架。这些工具包括 Systemtap、ftrace 和 LTTng。

Tracepoint 通过多种宏定义于多个头文件中。因此，本文档的目的是对可用
tracepoint 提供清晰的统计。意图不仅是理解有哪些 tracepoint 可用，还要
理解未来可能在何处添加 tracepoint。

所呈现的 API 具有形如 `trace_tracepointname(function parameters)` 的函数。
这些是位于代码各处的 tracepoint 回调。向这些回调站点注册和注销 probes 的
内容在 `Documentation/trace/*` 目录中说明。

## IRQ


   :internal:

## SIGNAL


   :internal:

## Block IO


   :internal:

## Workqueue


   :internal:
