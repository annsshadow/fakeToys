## NMI 璺熻釜浜嬩欢


杩欎簺浜嬩欢閫氬父鍑虹幇鍦ㄨ繖閲岋細

	/sys/kernel/tracing/events/nmi


### nmi_handler


濡傛灉浣犳€€鐤戜綘鐨?NMI 澶勭悊绋嬪簭鍗犵敤浜嗗ぇ閲?CPU 鏃堕棿锛屼綘鍙兘鎯充娇鐢ㄨ繖涓窡韪偣銆傚唴鏍?
```
	INFO: NMI handler took too long to run: 9.207 msecs
```
鑰岃繖涓窡韪偣灏嗗厑璁镐綘娣卞叆鏌ョ湅骞惰幏鍙栨洿澶氱粏鑺傘€?

鍋囪浣犳€€鐤?perf_event_nmi_handler() 缁欎綘甯︽潵浜嗕竴浜涢棶棰橈紝鑰屼綘鍙兂璺熻釜閭ｄ釜澶勭悊绋嬪簭
```
	$ grep perf_event_nmi_handler /proc/kallsyms
	ffffffff81625600 t perf_event_nmi_handler
```
鍐嶅亣璁句綘鍙閭ｄ釜鍑芥暟鐪熸鍗犵敤澶ч噺 CPU 鏃堕棿锛堜緥濡備竴娆′竴姣锛夋劅鍏磋叮銆傛敞鎰忓唴鏍哥殑杈撳嚭浠ユ绉掍负鍗曚綅锛屼絾杈撳叆
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
