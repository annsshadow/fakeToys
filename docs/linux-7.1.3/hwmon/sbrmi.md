
## 鍐呮牳椹卞姩 sbrmi


Supported hardware:

  - 閫氳繃 APML 杩炴帴鍒?BMC 鐨勩€佸吋瀹?Sideband Remote Management Interface
    锛圫B-RMI锛夌殑 AMD SoC 璁惧銆?
    Prefix: 'sbrmi'

    Addresses scanned: 璇ラ┍鍔ㄤ笉鏀寔鍦板潃鎵弿銆?
    瑕佸湪鏀寔 SB-RMI 鐨?AMD CPU 涓婂疄渚嬪寲璇ラ┍鍔紝i2c 鎬荤嚎缂栧彿搴斾负浠庢澘绾х鐞?    鎺у埗鍣紙BMC锛夎繛鎺ュ埌 CPU 鐨勬€荤嚎銆?    SMBus 鍦板潃瀹為檯涓?7 浣嶃€傞儴鍒嗗巶鍟嗗強 SMBus 瑙勮寖灏嗗湴鍧€琛ㄧず涓?8 浣嶃€佸乏瀵归綈锛?    骞跺皢 R/W 浣嶄綔涓哄啓锛?锛変娇 bit 0 涓?0銆傞儴鍒嗗巶鍟嗕粎浣跨敤 7 浣嶆潵鎻忚堪鍦板潃銆?    濡?AMD 鐨?APML 瑙勮寖鎵€杩帮紝SB-RMI 鍦板潃閫氬父涓?socket 0 鐨?78h(0111 100W) 鎴?    3Ch(011 1100)锛屼互鍙?socket 1 鐨?70h(0111 000W) 鎴?38h(011 1000)锛屼絾浼?    鍥犵‖浠跺湴鍧€閫夋嫨寮曡剼鑰屾湁鎵€鍙樺寲銆?
    Datasheet: SB-RMI 鎺ュ彛涓庡崗璁紝杩炲悓 Advanced Platform Management Link
               锛圓PML锛夎鑼冿紝浣滀负寮€婧?SoC 瀵勫瓨鍣ㄥ弬鑰冪殑涓€閮ㄥ垎鎻愪緵锛屼綅浜庯細

               https://www.amd.com/en/support/tech-docs?keyword=55898

Author: Akshay Gupta <akshay.gupta@amd.com>

### 鎻忚堪


APML 鎻愪緵浜嗕竴绉嶄粠澶栭儴 SMBus 涓昏澶囦笌 SB Remote Management interface锛圫B-RMI锛?妯″潡閫氫俊鐨勬柟寮忥紝鍙敤浜庨€氳繃閭鍛戒护鎶ュ憡 AMD 骞冲彴涓婄殑鎻掓Ы鍔熻€楋紝骞剁被浼间簬鍏稿瀷鐨?8 寮曡剼杩滅鐢垫簮浼犳劅鍣ㄧ殑 I2C 鎺ュ彛杩炴帴鍒?BMC銆?
璇ラ┍鍔ㄥ疄鐜颁簡褰撳墠鍔熻€椾互鍙婂姛鑰椾笂闄愪笌鏈€澶у姛鑰椾笂闄愩€?
### sysfs 鎺ュ彛


鐢垫簮浼犳劅鍣ㄥ彲閫氳繃鏍囧噯 `hwmon` 鎺ュ彛鍦?`sysfs` 涓婃煡璇笌璁剧疆锛屼綅浜庣洰褰?`/sys/class/hwmon/hwmonX`锛圶 涓烘煇鍊硷紝鏌ユ壘浣?`/sys/class/hwmon/hwmonX/name`
鍐呭涓?`sbrmi` 鐨勯偅涓?`X`锛夈€?
================ ===== ========================================================
Name             Perm   鎻忚堪
================ ===== ========================================================
power1_input     RO    褰撳墠娑堣€楀姛鐜?power1_cap       RW    鍙湪 0 涓?power1_cap_max 涔嬮棿璁剧疆鍔熻€楅檺鍒?power1_cap_max   RO    SMU FW 璁＄畻骞舵姤鍛婄殑鏈€澶у姛鑰楅檺鍒?================ ===== ========================================================

浠ヤ笅绀轰緥灞曠ず浜嗘潵鑷?i2c 鍦板潃鐨?'Power' 灞炴€?```

  # sensors
  sbrmi-i2c-1-38
  Adapter: bcm2835 I2C adapter
  power1:       61.00 W (cap = 225.00 W)

  sbrmi-i2c-1-3c
  Adapter: bcm2835 I2C adapter
  power1:       28.39 W (cap = 224.77 W)
  #

```
```
  # cat /sys/class/hwmon/hwmon1/power1_cap_max
  225000000

  # echo 180000000 > /sys/class/hwmon/hwmon1/power1_cap
  # cat /sys/class/hwmon/hwmon1/power1_cap
  180000000

```
