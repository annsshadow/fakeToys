
## 璺熻釜缂撳啿鍖烘墿灞?(TRBE)銆?

    :Author:   Anshuman Khandual <anshuman.khandual@arm.com>
    :Date:     November 2020

### 纭欢鎻忚堪


璺熻釜缂撳啿鍖烘墿灞?(TRBE) 鏄竴绉?percpu 纭欢锛屽畠鍦ㄧ郴缁熷唴瀛樹腑鎹曡幏鏉ヨ嚜鐩稿簲
percpu 璺熻釜鍗曞厓鐨?CPU 璺熻釜銆傚畠琚彃鍏ヤ负 coresight sink 璁惧锛屽洜涓虹浉搴旂殑
璺熻釜鐢熸垚鍣?(ETE) 琚彃鍏ヤ负婧愯澶囥€?
TRBE 涓嶇鍚?CoreSight 鏋舵瀯瑙勮寖锛屼絾閫氳繃 CoreSight 椹卞姩妗嗘灦杩涜椹卞姩锛屼互
鏀寔 ETE锛堢鍚?CoreSight 瑙勮寖锛夌殑闆嗘垚銆?
### Sysfs 鏂囦欢鍜岀洰褰?

TRBE 璁惧鍑虹幇鍦ㄧ幇鏈?coresight 鎬荤嚎涓婏紝涓庡叾浠栬澶囧苟鍒楋細

```

	>$ ls /sys/bus/coresight/devices
	trbe0  trbe1  trbe2 trbe3

```
```

	>$ ls /sys/bus/coresight/devices/trbe0/
        align flag

```
**鍏抽敭鏂囦欢椤瑰涓嬶細-**
   - `align`锛歍RBE 鍐欐寚閽堝榻?   - `flag`锛歍RBE 浣跨敤璁块棶鏍囧織鍜岃剰鏍囧織鏇存柊鍐呭瓨
