## 鍐呮牳椹卞姩锛歺86_pkg_temp_thermal


鏀寔鐨勮姱鐗囷細

- x86锛氬叿鏈夊皝瑁呯骇鐑鐞?

锛堜娇鐢ㄤ互涓嬫柟寮忛獙璇侊細CPUID.06H:EAX[bit 6] =1锛?

Authors: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 鍙傝€?


Intel庐 64 鍜?IA-32 鏋舵瀯杞欢寮€鍙戞墜鍐岋紙2013 骞?1 鏈堬級锛?
绗?14.6 绔狅細灏佽绾х儹绠＄悊锛圥ACKAGE LEVEL THERMAL MANAGEMENT锛?

### 鎻忚堪


璇ラ┍鍔ㄥ皢 CPU 鏁板瓧娓╁害灏佽绾т紶鎰熷櫒娉ㄥ唽涓轰竴涓儹鍖猴紝鏈€澶氬彲閰嶇疆涓や釜鐢ㄦ埛妯″紡瑙﹀彂鐐广€傝Е鍙戠偣鐨勬暟閲忓彇鍐充簬灏佽鐨勮兘鍔涖€備竴鏃﹁Е鍙戠偣琚繚鍙嶏紝鐢ㄦ埛妯″紡鍙互閫氳繃鐑€氱煡鏈哄埗鎺ユ敹閫氱煡锛屽苟鍙互閲囧彇浠讳綍鎺柦鏉ユ帶鍒舵俯搴︺€?


### 闃堝€肩鐞?


姣忎釜灏佽灏嗕綔涓?/sys/class/thermal 涓嬬殑涓€涓儹鍖烘敞鍐屻€?

```
	/sys/class/thermal/thermal_zone1
```
杩欏寘鍚袱涓Е鍙戠偣锛?

- trip_point_0_temp
- trip_point_1_temp

鐢ㄦ埛鍙互璁剧疆 0 鍒?TJ-Max 娓╁害涔嬮棿鐨勪换鎰忔俯搴︺€傛俯搴﹀崟浣嶄负姣憚姘忓害銆傛湁鍏崇儹 sys-fs 鐨勭粏鑺傦紝璇峰弬闃?"Documentation/driver-api/thermal/sysfs-api.rst"銆?

杩欎簺瑙﹀彂鐐逛腑闄?0 浠ュ鐨勪换浣曞€奸兘鍙互瑙﹀彂鐑€氱煡銆傝缃负 0 浼氬仠姝㈠彂閫佺儹閫氱煡銆?

鐑€氱煡锛?
瑕佽幏鍙?kobject-uevent 閫氱煡锛岃灏嗙儹鍖虹殑绛栫暐璁剧疆涓?"user_space"銆?

```
	echo -n "user_space" > policy
```
