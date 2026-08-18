## 鍒囨崲璋冨害鍣?

姣忎釜 IO 闃熷垪閮芥湁涓€缁勪笌涔嬪叧鑱旂殑 IO 璋冨害鍣ㄥ彲璋冨弬鏁般€傝繖浜涘彲璋冨弬鏁版帶鍒?IO 璋冨害鍣ㄧ殑宸ヤ綔鏂瑰紡銆傚亣璁炬偍宸插皢 sysfs 鎸傝浇鍒?/sys锛屾偍鍙互鍦ㄤ互涓嬩綅缃?鎵惧埌杩欎簺鏉＄洰锛?
```
	/sys/block/<device>/queue/iosched
```

濡傛灉鎮ㄦ病鏈夋寕杞?sysfs锛?
```
	# mount none /sys -t sysfs
```

鍙互瀹炴椂鏇存敼缁欏畾鍧楄澶囩殑 IO 璋冨害鍣紝浠ラ€夋嫨 mq-deadline銆乶one銆乥fq 鎴?kyber 璋冨害鍣ㄤ箣涓€鈥斺€旇繖鍙互鎻愰珮璇ヨ澶囩殑鍚炲悙閲忋€?
```
	echo SCHEDNAME > /sys/block/DEV/queue/scheduler
```

鍏朵腑 SCHEDNAME 鏄凡瀹氫箟 IO 璋冨害鍣ㄧ殑鍚嶇О锛孌EV 鏄澶囧悕锛坔da銆乭db銆乻ga
鎴栨偍鎷ユ湁鐨勪换浣曡澶囷級銆?
鍙渶鎵ц "cat /sys/block/DEV/queue/scheduler" 鍗冲彲鎵惧埌宸插畾涔夎皟搴﹀櫒鐨?鍒楄〃鈥斺€旀湁鏁堝悕绉板垪琛ㄥ涓嬶細

```
  # cat /sys/block/sda/queue/scheduler
  [mq-deadline] kyber bfq none
  # echo none >/sys/block/sda/queue/scheduler
  # cat /sys/block/sda/queue/scheduler
  [none] mq-deadline kyber bfq
```
