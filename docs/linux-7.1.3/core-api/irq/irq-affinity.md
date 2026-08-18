## SMP IRQ 浜插拰鎬?

ChangeLog:
 - 鐢?Ingo Molnar <mingo@redhat.com> 鍙戣捣
 - 鐢?Max Krasnyansky <maxk@qualcomm.com> 鏇存柊


/proc/irq/IRQ#/smp_affinity 鍜?/proc/irq/IRQ#/smp_affinity_list 鎸囧畾浜嗗浜庣粰瀹氱殑
IRQ 婧愬厑璁哥殑鐩爣 CPU銆傚畠鏄竴涓綅鎺╃爜锛坰mp_affinity锛夋垨鍏佽鐨?CPU 鍒楄〃锛坰mp_affinity_list锛夈€?涓嶅厑璁稿叧闂墍鏈?CPU锛屽鏋滄煇涓?IRQ 鎺у埗鍣ㄤ笉鏀寔 IRQ 浜插拰鎬э紝鍒欒鍊煎皢淇濇寔榛樿鍊硷紙鎵€鏈?CPU锛?涓嶅彉銆?
/proc/irq/default_smp_affinity 鎸囧畾搴旂敤浜庢墍鏈夐潪娲诲姩 IRQ 鐨勯粯璁や翰鍜屾€ф帺鐮併€備竴鏃?IRQ 琚垎閰?/婵€娲伙紝鍏朵翰鍜屾€т綅鎺╃爜灏嗚璁剧疆涓鸿榛樿鎺╃爜銆備箣鍚庡彲浠ュ涓婃墍杩拌繘琛屾洿鏀广€傞粯璁ゆ帺鐮佷负 0xffffffff銆?
涓嬮潰鏄竴涓皢 IRQ44锛坋th1锛夐檺鍒跺埌 CPU0-3锛岀劧鍚庡啀闄愬埗鐨勭ず渚?```

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
浠庝笂闈㈢殑琛屽彲浠ョ湅鍑猴紝IRQ44 鍙浼犻€掔粰浜嗗墠鍥涗釜澶勭悊鍣紙0-3锛夈€?鐜板湪璁╂垜浠皢璇?IRQ 闄愬埗鍒?CPU锛?-7锛夈€?
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
杩欐 IRQ44 鍙浼犻€掔粰浜嗘渶鍚庡洓涓鐞嗗櫒銆?鍗?CPU0-3 鐨勮鏁板櫒娌℃湁鍙樺寲銆?
```

	[root@moon 44]# echo 1024-1031 > smp_affinity_list
	[root@moon 44]# cat smp_affinity_list
	1024-1031

```
娉ㄦ剰锛岃鐢ㄤ綅鎺╃爜鍋氬埌杩欎竴鐐癸紝闇€瑕佸湪鐩稿叧浣嶆帺鐮佷箣鍚庤窡闅?32 涓浂浣嶆帺鐮併€?