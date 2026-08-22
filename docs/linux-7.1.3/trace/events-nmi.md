## NMI 跟踪事件


这些事件通常出现在这里：

	/sys/kernel/tracing/events/nmi


### nmi_handler


如果你怀疑你NMI 处理程序占用了大CPU 时间，你可能想使用这个跟踪点。内
```
	INFO: NMI handler took too long to run: 9.207 msecs
```
而这个跟踪点将允许你深入查看并获取更多细节

假设你怀perf_event_nmi_handler() 给你带来了一些问题，而你只想跟踪那个处理程序
```
	$ grep perf_event_nmi_handler /proc/kallsyms
	ffffffff81625600 t perf_event_nmi_handler
```
再假设你只对那个函数真正占用大量 CPU 时间（例如一次一毫秒）感兴趣。注意内核的输出以毫秒为单位，但输入
```
	cd /sys/kernel/tracing/events/nmi/nmi_handler
	echo 'handler==0xffffffff81625600 && delta_ns>1000000' > filter
	echo 1 > enable
```
```
	$ cat /sys/kernel/tracing/trace_pipe
	<idle>-0     [000] d.h3   505.397558: nmi_handler: perf_event_nmi_handler() delta_ns: 3236765 handled: 1
	<idle>-0     [000] d.h3   505.805893: nmi_handler: perf_event_nmi_handler() delta_ns: 3174234 handled: 1
	<idle>-0     [000] d.h3   506.158206: nmi_handler: perf_event_nmi_handler() delta_ns: 3084642 handled: 1
	<idle>-0     [000] d.h3   506.334346: nmi_handler: perf_event_nmi_handler() delta_ns: 3080351 handled: 1
```
