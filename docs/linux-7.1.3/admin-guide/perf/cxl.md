## CXL 鎬ц兘鐩戞帶鍗曞厓锛圕PMU锛?

CXL rev 3.0 瑙勮寖鍦ㄧ 13.2 鑺傗€淧erformance Monitoring锛堟€ц兘鐩戞帶锛夆€濅腑缁欏嚭浜?CXL 鎬ц兘
鐩戞帶鍗曞厓鐨勫畾涔夈€?
CXL 缁勪欢锛堜緥濡?Root Port銆丼witch Upstream Port銆丒nd Point锛夊彲浠ユ湁浠绘剰鏁伴噺鐨?CPMU
瀹炰緥銆侰PMU 鑳藉姏鍙畬鍏ㄤ粠璁惧涓彂鐜般€傝瑙勮寖涓烘墍鏈?CXL 鍗忚娑堟伅绫诲瀷鎻愪緵浜嬩欢瀹氫箟锛屽苟
涓?CXL 璁惧涓婇€氬父缁熻鐨勪簨鐗╋紙渚嬪 DRAM 浜嬩欢锛夋彁渚涗竴缁勯檮鍔犱簨浠躲€?
## CPMU 椹卞姩


CPMU 椹卞姩鍦?CXL 鎬荤嚎涓婃敞鍐屼竴涓悕涓?pmu_mem<X>.<Y> 鐨?perf PMU锛屼唬琛?memX 鐨勭 Y 涓?CPMU銆?
    /sys/bus/cxl/device/pmu_mem<X>.<Y>

鍏宠仈鐨?PMU 娉ㄥ唽涓?
   /sys/bus/event_sources/devices/cxl_pmu_mem<X>.<Y>

涓庡叾浠?CXL 鎬荤嚎璁惧涓€鏍凤紝璇?id 娌℃湁鐗瑰畾鍚箟锛屽簲閫氳繃涓?CXL 鎬荤嚎涓婅澶囩殑鐖惰澶囧缓绔?鍏崇郴鏉ョ‘瀹氬叾瀵瑰簲鐨勫叿浣?CXL 璁惧銆?
PMU 椹卞姩鍦?sysfs 涓彁渚涘彲鐢ㄤ簨浠跺拰杩囨护閫夐」鐨勬弿杩般€?
鈥渇ormat鈥?鐩綍鎻忚堪 perf_event_attr 缁撴瀯鐨?config锛堜簨浠跺巶鍟?id銆乬roup id 鍜?mask锛夈€?config1锛堥槇鍊笺€佽繃婊や娇鑳斤級鍜?config2锛堣繃婊ゅ弬鏁帮級瀛楁鐨勬墍鏈夋牸寮忋€傗€渆vents鈥?鐩綍鎻忚堪
perf list 涓樉绀虹殑鎵€鏈夊凡璁板綍浜嬩欢銆?
perf list 涓樉绀虹殑浜嬩欢鏄簨浠舵帺鐮佷腑璁剧疆浜嗗崟涓瘮鐗圭殑鏈€缁嗙矑搴︿簨浠躲€傛洿閫氱敤鐨勪簨浠跺彲浠?閫氳繃鍦?config 涓缃涓帺鐮佷綅鏉ュ惎鐢ㄣ€備緥濡傦紝鎵€鏈?Device to Host 璇昏姹傞兘鍙互閫氳繃
璁剧疆浠ヤ笅鎵€鏈変綅鑰屽湪鍗曚釜璁℃暟鍣ㄤ笂鎹曡幏锛?
- d2h_req_rdcurr
- d2h_req_rdown
- d2h_req_rdshared
- d2h_req_rdany
- d2h_req_rdownnodata
```

  $#perf list
  cxl_pmu_mem0.0/clock_ticks/                        [Kernel PMU event]
  cxl_pmu_mem0.0/d2h_req_rdshared/                   [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpcur/                     [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpdata/                    [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpinv/                     [Kernel PMU event]
  -----------------------------------------------------------

  $# perf stat -a -e cxl_pmu_mem0.0/clock_ticks/ -e cxl_pmu_mem0.0/d2h_req_rdshared/

```
鍘傚晢鐗瑰畾鐨勪簨浠朵篃鍙兘鍙敤锛岃嫢鍙敤鍙€氳繃浠ヤ笅鏂瑰紡浣跨敤

  $# perf stat -a -e cxl_pmu_mem0.0/vid=VID,gid=GID,mask=MASK/

璇ラ┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝?鈥減erf record鈥?涓嶅彈鏀寔銆傚畠鍙敮鎸佺郴缁熻寖鍥寸殑璁℃暟锛屽洜姝?闄勫姞鍒颁换鍔′笉鍙楁敮鎸併€?