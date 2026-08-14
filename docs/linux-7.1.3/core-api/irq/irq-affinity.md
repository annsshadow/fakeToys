## SMP IRQ 亲和性


ChangeLog:
 - 由 Ingo Molnar <mingo@redhat.com> 发起
 - 由 Max Krasnyansky <maxk@qualcomm.com> 更新


/proc/irq/IRQ#/smp_affinity 和 /proc/irq/IRQ#/smp_affinity_list 指定了对于给定的
IRQ 源允许的目标 CPU。它是一个位掩码（smp_affinity）或允许的 CPU 列表（smp_affinity_list）。
不允许关闭所有 CPU，如果某个 IRQ 控制器不支持 IRQ 亲和性，则该值将保持默认值（所有 CPU）
不变。

/proc/irq/default_smp_affinity 指定应用于所有非活动 IRQ 的默认亲和性掩码。一旦 IRQ 被分配
/激活，其亲和性位掩码将被设置为该默认掩码。之后可以如上所述进行更改。默认掩码为 0xffffffff。

下面是一个将 IRQ44（eth1）限制到 CPU0-3，然后再限制的示例
```

	[root@moon 44]# cd /proc/irq/44
	[root@moon 44]# cat smp_affinity
	ffffffff

	[root@moon 44]# echo 0f > smp_affinity
	[root@moon 44]# cat smp_affinity
	0000000f
	[root@moon 44]# ping -f h
	PING hell (195.4.7.3): 56 data bytes
	...
	--- hell ping statistics ---
	6029 packets transmitted, 6027 packets received, 0% packet loss
	round-trip min/avg/max = 0.1/0.1/0.4 ms
	[root@moon 44]# cat /proc/interrupts | grep 'CPU\|44:'
		CPU0       CPU1       CPU2       CPU3      CPU4       CPU5        CPU6       CPU7
	44:       1068       1785       1785       1783         0          0           0         0    IO-APIC-level  eth1

```
从上面的行可以看出，IRQ44 只被传递给了前四个处理器（0-3）。
现在让我们将该 IRQ 限制到 CPU（4-7）。

```

	[root@moon 44]# echo f0 > smp_affinity
	[root@moon 44]# cat smp_affinity
	000000f0
	[root@moon 44]# ping -f h
	PING hell (195.4.7.3): 56 data bytes
	..
	--- hell ping statistics ---
	2779 packets transmitted, 2777 packets received, 0% packet loss
	round-trip min/avg/max = 0.1/0.5/585.4 ms
	[root@moon 44]# cat /proc/interrupts |  'CPU\|44:'
		CPU0       CPU1       CPU2       CPU3      CPU4       CPU5        CPU6       CPU7
	44:       1068       1785       1785       1783      1784       1069        1070       1069   IO-APIC-level  eth1

```
这次 IRQ44 只被传递给了最后四个处理器。
即 CPU0-3 的计数器没有变化。

```

	[root@moon 44]# echo 1024-1031 > smp_affinity_list
	[root@moon 44]# cat smp_affinity_list
	1024-1031

```
注意，要用位掩码做到这一点，需要在相关位掩码之后跟随 32 个零位掩码。
