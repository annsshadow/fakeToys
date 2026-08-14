
## 追踪 Remotes（远程追踪）


:Author: Vincent Donnefort <vdonnefort@google.com>

## 概述

固件和虚拟机监控器（hypervisor）对内核来说都是黑盒。能够看到它们在做什么，对调试两者都很有用。这正是远程追踪缓冲区（remote tracing buffer）的用武之地。远程追踪缓冲区是由固件或虚拟机监控器在映射到主机内核的内存中执行的一个环形缓冲区。这类似于用户空间内存映射内核环形缓冲区的方式，但在这种情况下内核扮演的是用户空间的角色，而固件或虚拟机监控器则是"内核"一侧。借助远程追踪环形缓冲区，固件和虚拟机监控器可以记录事件，主机内核能够看到这些事件并将其暴露给用户空间。

## 注册一个 remote

一个 remote 必须提供一组回调函数 `struct trace_remote_callbacks`，其描述见下文。这些回调允许 Tracefs 启用和禁用追踪与事件、加载和卸载追踪缓冲区（一组环形缓冲区），以及与头页交换一个读取器页，从而实现消费式读取。

一旦注册，该 remote 的一个实例就会出现在 Tracefs 目录 **remotes/** 下。然后可以使用常规的 Tracefs 文件 **trace_pipe** 和 **trace** 来读取缓冲区。

## 声明一个 remote 事件

提供了一些宏来简化 remote 事件的声明，其方式与内核内事件类似。声明必须提供 ID、事件参数的描述以及事件的打印方式：

	REMOTE_EVENT(foo, EVENT_FOO_ID,
		RE_STRUCT(
			re_field(u64, bar)
		),
		RE_PRINTK("bar=%lld", __entry->bar)
	);

然后必须在 C 文件中使用以下内容声明这些事件：

	#define REMOTE_EVENT_INCLUDE_FILE foo_events.h
	#include <trace/define_remote_events.h>

这会提供一个 `struct remote_event remote_event_foo`，可以把它传给 `trace_remote_register`。

已注册的事件会出现在 remote 目录下的 **events/** 中。

## 简单环形缓冲区

一个环形缓冲区写入端的简单实现可以在 kernel/trace/simple_ring_buffer.c 中找到。
