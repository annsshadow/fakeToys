## USB3 璋冭瘯绔彛


:浣滆€? Lu Baolu <baolu.lu@linux.intel.com>
:鏃ユ湡: 2017 骞?3 鏈?
## 姒傝堪锛圙ENERAL锛?

鏈枃浠嬬粛濡備綍鍦?x86 绯荤粺涓婁娇鐢?USB3 璋冭瘯绔彛銆?
鍦ㄤ娇鐢ㄤ换浣曞熀浜?USB3 鐨勮皟璇曞姛鑳戒箣鍓?

```
	1) check whether any USB3 debug port is available in
	   your system;
	2) check which port is used for debugging purposes;
	3) have a USB 3.0 super-speed A-to-A debugging cable.

```

## 浠嬬粛锛圛NTRODUCTION锛?

xHCI 璋冭瘯鑳藉姏锛圖bC锛夋槸 xHCI 涓绘満鎺у埗鍣ㄦ彁渚涚殑涓€涓彲閫変絾鐙珛鐨?鍔熻兘銆倄HCI 瑙勮寖鍦?7.6 鑺備腑鎻忚堪浜?DbC銆?
褰?DbC 琚垵濮嬪寲骞跺惎鐢ㄥ悗锛屽畠浼氶€氳繃璋冭瘯绔彛锛堥€氬父鏄涓€涓?USB3
瓒呴珮閫熺鍙ｏ級鍛堢幇涓€涓皟璇曡澶囥€傝璋冭瘯璁惧瀹屽叏绗﹀悎
USB 妗嗘灦锛屽苟鍦ㄨ皟璇曠洰鏍囷紙琚皟璇曠殑绯荤粺锛変笌璋冭瘯涓绘満锛坉ebug host锛変箣闂?鎻愪緵鐩稿綋浜庝竴鏉￠潪甯搁珮鎬ц兘鐨勩€佸叏鍙屽伐涓茶閾捐矾銆?
## 鏃╂湡鎵撳嵃锛圗ARLY PRINTK锛?

DbC 琚璁＄敤鏉ヨ褰?early printk 娑堟伅銆傝鐗规€х殑涓€涓敤閫旀槸鍐呮牳璋冭瘯銆?渚嬪锛屽綋浣犵殑鏈哄櫒鍦ㄥ父瑙勬帶鍒跺彴浠ｇ爜鍒濆鍖栦箣鍓嶅氨闈炲父鏃╁湴宕╂簝鏃躲€?鍏朵粬鐢ㄩ€斿寘鎷洿绠€鍗曘€佹棤閿侊紙lockless锛夌殑鏃ュ織锛岃€岄潪瀹屾暣鐨?printk 鎺у埗鍙伴┍鍔ㄤ笌 klogd銆?
鍦ㄨ皟璇曠洰鏍囩郴缁熶笂锛屼綘闇€瑕佸畾鍒朵竴涓惎鐢ㄤ簡
CONFIG_EARLY_PRINTK_USB_XDBC 鐨勮皟璇曞唴鏍搞€傚苟娣诲姞濡備笅
鍐呮牳鍙傛暟


```
	"earlyprintk=xdbc"

```

濡傛灉浣犵殑绯荤粺涓湁澶氫釜 xHCI 鎺у埗鍣紝浣犲彲浠?鍦ㄨ鍐呮牳鍙傛暟鍚庨檮鍔犱竴涓富鏈烘帶鍒跺櫒绱㈠紩銆傝绱㈠紩浠?0 寮€濮嬨€?
褰撳墠璁捐涓嶆敮鎸?DbC 杩愯鏃舵寕璧?鎭㈠銆傚洜姝わ紝浣犳渶濂戒负
浠ヤ笅鍙傛暟绂佺敤杩愯鏃剁數婧愮鐞?

```
	"usbcore.autosuspend=-1"

```

鍦ㄥ惎鍔ㄨ皟璇曠洰鏍囦箣鍓嶏紝浣犲簲璇ュ皢璋冭瘯绔彛杩炴帴鍒拌皟璇曚富鏈轰笂鐨?涓€涓?USB 绔彛锛堟牴绔彛鎴栦换浣曞閮ㄩ泦绾垮櫒鐨勭鍙ｏ級銆傜敤浜庤繛鎺ヨ繖涓や釜绔彛鐨?绾跨紗搴斿綋鏄竴鏉?USB 3.0 瓒呴珮閫?A-to-A 璋冭瘯绾跨紗銆?
鍦ㄨ皟璇曠洰鏍囨棭鏈熷惎鍔ㄨ繃绋嬩腑锛孌bC 浼氳妫€娴嬪埌骞跺垵濮嬪寲銆?鍒濆鍖栧畬鎴愬悗锛岃皟璇曚富鏈哄簲褰撹兘澶熸灇涓捐皟璇曠洰鏍囦腑鐨勮皟璇曡澶囥€?闅忓悗璋冭瘯涓绘満浼氬皢璋冭瘯璁惧涓?usb_debug 椹卞姩妯″潡缁戝畾锛?骞跺垱寤?/dev/ttyUSB 璁惧銆?
濡傛灉璋冭瘯璁惧鐨勬灇涓鹃『鍒╄繘琛岋紝浣犲簲璇ヨ兘澶?鐪嬪埌


```
	# tail -f /var/log/kern.log
	[ 1815.983374] usb 4-3: new SuperSpeed USB device number 4 using xhci_hcd
	[ 1815.999595] usb 4-3: LPM exit latency is zeroed, disabling LPM.
	[ 1815.999899] usb 4-3: New USB device found, idVendor=1d6b, idProduct=0004
	[ 1815.999902] usb 4-3: New USB device strings: Mfr=1, Product=2, SerialNumber=3
	[ 1815.999903] usb 4-3: Product: Remote GDB
	[ 1815.999904] usb 4-3: Manufacturer: Linux
	[ 1815.999905] usb 4-3: SerialNumber: 0001
	[ 1816.000240] usb_debug 4-3:1.0: xhci_dbc converter detected
	[ 1816.000360] usb 4-3: xhci_dbc converter now attached to ttyUSB0

```

浣犲彲浠ヤ娇鐢ㄤ换浣曢€氫俊绋嬪簭锛堜緥濡?minicom锛夋潵璇诲彇骞舵煡鐪嬭繖浜涙秷鎭€?涓嬮潰杩欎釜绠€鍗曠殑 bash 鑴氭湰鍙互甯姪浣犳鏌ヨ缃槸鍚︽纭€?

	===== start of bash scripts =============
	#!/bin/bash

	while true ; do
		while [ ! -d /sys/class/tty/ttyUSB0 ] ; do
			:
		done
	cat /dev/ttyUSB0
	done
	===== end of bash scripts ===============

## 涓茶 TTY锛圫erial TTY锛?

DbC 鏀寔宸茶娣诲姞鍒?xHCI 椹卞姩涓€備綘鍙互鍦ㄨ繍琛屾椂鑾峰緱
鐢?DbC 鎻愪緵鐨勮皟璇曡澶囥€?
涓轰簡浣跨敤姝ゅ姛鑳斤紝浣犻渶瑕佺‘淇濆唴鏍稿凡閰嶇疆涓烘敮鎸?USB_XHCI_DBGCAP銆倄HCI 璁惧鑺傜偣涓嬬殑涓€涓?sysfs 灞炴€?鐢ㄤ簬鍚敤鎴栫鐢?DbC銆傞粯璁ゆ儏鍐典笅锛?

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	disabled

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# echo enable > dbc

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	enabled

```

浣跨敤涓€鏉?USB 3.0 瓒呴珮閫?A-to-A 璋冭瘯绾跨紗灏嗚皟璇曠洰鏍囪繛鎺ュ埌
璋冭瘯涓绘満銆備綘浼氱湅鍒?/dev/ttyDBC0 琚垱寤猴紝


```
	root@target: tail -f /var/log/kern.log
	[  182.730103] xhci_hcd 0000:00:14.0: DbC connected
	[  191.169420] xhci_hcd 0000:00:14.0: DbC configured
	[  191.169597] xhci_hcd 0000:00:14.0: DbC now attached to /dev/ttyDBC0

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	configured

```

鍦ㄨ皟璇曚富鏈轰笂锛屼綘浼氱湅鍒拌皟璇曡澶囧凡琚灇涓俱€?

```
	root@host: tail -f /var/log/kern.log
	[   79.454780] usb 2-2.1: new SuperSpeed USB device number 3 using xhci_hcd
	[   79.475003] usb 2-2.1: LPM exit latency is zeroed, disabling LPM.
	[   79.475389] usb 2-2.1: New USB device found, idVendor=1d6b, idProduct=0010
	[   79.475390] usb 2-2.1: New USB device strings: Mfr=1, Product=2, SerialNumber=3
	[   79.475391] usb 2-2.1: Product: Linux USB Debug Target
	[   79.475392] usb 2-2.1: Manufacturer: Linux Foundation
	[   79.475393] usb 2-2.1: SerialNumber: 0001

```

璋冭瘯璁惧鐜板湪宸插伐浣溿€備綘鍙互浣跨敤浠讳綍閫氫俊鎴栬皟璇?绋嬪簭鍦ㄤ富鏈轰笌鐩爣涔嬮棿杩涜閫氫俊銆?