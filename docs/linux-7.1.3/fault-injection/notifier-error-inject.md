## 閫氱煡鍣ㄩ敊璇敞鍏?

閫氱煡鍣ㄩ敊璇敞鍏ユ彁渚涗簡鍚戞寚瀹氱殑閫氱煡鍣ㄩ摼鍥炶皟涓敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠瀵逛簬娴嬭瘯閫氱煡鍣ㄨ皟鐢ㄩ摼澶辫触锛堣繖绉嶆儏鍐靛緢灏戣鎵ц锛夌殑閿欒澶勭悊闈炲父鏈夌敤銆傛湁涓€浜涘唴鏍告ā鍧楀彲鐢ㄤ簬娴嬭瘯浠ヤ笅閫氱煡鍣ㄣ€?
 - PM 閫氱煡鍣? - 鍐呭瓨鐑彃鎷旈€氱煡鍣? - powerpc pSeries reconfig 閫氱煡鍣? - 缃戠粶璁惧锛圢etdevice锛夐€氱煡鍣?
### PM 閫氱煡鍣ㄩ敊璇敞鍏ユā鍧?
璇ョ壒鎬ч€氳繃 debugfs 鎺ュ彛鎺у埗

  /sys/kernel/debug/notifier-error-inject/pm/actions/<notifier event>/error

鍙兘琚疆涓哄け璐ョ殑 PM 閫氱煡鍣ㄤ簨浠舵湁锛?
 - PM_HIBERNATION_PREPARE
 - PM_SUSPEND_PREPARE
 - PM_RESTORE_PREPARE

```

	# cd /sys/kernel/debug/notifier-error-inject/pm/
	# echo -12 > actions/PM_SUSPEND_PREPARE/error
	# echo mem > /sys/power/state
	bash: echo: write error: Cannot allocate memory

```
### 鍐呭瓨鐑彃鎷旈€氱煡鍣ㄩ敊璇敞鍏ユā鍧?
璇ョ壒鎬ч€氳繃 debugfs 鎺ュ彛鎺у埗

  /sys/kernel/debug/notifier-error-inject/memory/actions/<notifier event>/error

鍙兘琚疆涓哄け璐ョ殑鍐呭瓨閫氱煡鍣ㄤ簨浠舵湁锛?
 - MEM_GOING_ONLINE
 - MEM_GOING_OFFLINE

```

	# cd /sys/kernel/debug/notifier-error-inject/memory
	# echo -12 > actions/MEM_GOING_OFFLINE/error
	# echo offline > /sys/devices/system/memory/memoryXXX/state
	bash: echo: write error: Cannot allocate memory

```
### powerpc pSeries reconfig 閫氱煡鍣ㄩ敊璇敞鍏ユā鍧?
璇ョ壒鎬ч€氳繃 debugfs 鎺ュ彛鎺у埗

  /sys/kernel/debug/notifier-error-inject/pSeries-reconfig/actions/<notifier event>/error

鍙兘琚疆涓哄け璐ョ殑 pSeries reconfig 閫氱煡鍣ㄤ簨浠舵湁锛?
 - PSERIES_RECONFIG_ADD
 - PSERIES_RECONFIG_REMOVE
 - PSERIES_DRCONF_MEM_ADD
 - PSERIES_DRCONF_MEM_REMOVE

### 缃戠粶璁惧閫氱煡鍣ㄩ敊璇敞鍏ユā鍧?
璇ョ壒鎬ч€氳繃 debugfs 鎺ュ彛鎺у埗

  /sys/kernel/debug/notifier-error-inject/netdev/actions/<notifier event>/error

鍙缃负澶辫触鐨勭綉缁滆澶囬€氱煡鍣ㄤ簨浠舵湁锛?
 - NETDEV_REGISTER
 - NETDEV_CHANGEMTU
 - NETDEV_CHANGENAME
 - NETDEV_PRE_UP
 - NETDEV_PRE_TYPE_CHANGE
 - NETDEV_POST_INIT
 - NETDEV_PRECHANGEMTU
 - NETDEV_PRECHANGEUPPER
 - NETDEV_CHANGEUPPER

```

	# cd /sys/kernel/debug/notifier-error-inject/netdev
	# echo -22 > actions/NETDEV_CHANGEMTU/error
	# ip link set eth0 mtu 1024
	RTNETLINK answers: Invalid argument

```
### 鏇村浣跨敤绀轰緥

鏈変竴浜?tools/testing/selftests 浣跨敤浜嗛€氱煡鍣ㄩ敊璇敞鍏ョ壒鎬ф潵娴嬭瘯 CPU 鍜屽唴瀛橀€氱煡鍣ㄣ€?
 - tools/testing/selftests/cpu-hotplug/cpu-on-off-test.sh
 - tools/testing/selftests/memory-hotplug/mem-on-off-test.sh

杩欎簺鑴氭湰棣栧厛杩涜绠€鍗曠殑涓婄嚎涓庝笅绾挎祴璇曪紝鐒跺悗鍦ㄩ€氱煡鍣ㄩ敊璇敞鍏ユā鍧楀彲鐢ㄦ椂杩涜鏁呴殰娉ㄥ叆娴嬭瘯銆?