## HNS3 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?
HNS3锛圚iSilicon network system 3锛夋€ц兘鐩戞帶鍗曞厓锛圥MU锛夋槸涓€涓鐐癸紙End Point锛夎澶囷紝
鐢ㄤ簬鏀堕泦 HiSilicon SoC NIC 鐨勬€ц兘缁熻淇℃伅銆傚湪 Hip09 涓婏紝姣忎釜 SICL锛圫uper I/O
cluster锛岃秴绾?I/O 绨囷級閮芥湁涓€涓?PMU 璁惧銆?
HNS3 PMU 鏀寔鏀堕泦甯﹀銆佸欢杩熴€佸寘閫熺巼鍜屼腑鏂€熺巼绛夋€ц兘缁熻淇℃伅銆?
姣忎釜 HNS3 PMU 鏀寔 8 涓‖浠朵簨浠躲€?
## HNS3 PMU 椹卞姩

```

  /sys/bus/event_source/devices/hns3_pmu_sicl_<sicl_id>

```
PMU 椹卞姩鍦?sysfs 涓彁渚涘彲鐢ㄤ簨浠躲€佽繃婊ゆā寮忋€佹牸寮忋€佹爣璇嗙锛坕dentifier锛夊拰 cpumask 鐨?鎻忚堪銆?
"events" 鐩綍鎻忚堪浜?perf list 涓樉绀虹殑鎵€鏈夊彈鏀寔浜嬩欢鐨勪簨浠剁爜銆?
"filtermode" 鐩綍鎻忚堪浜嗘瘡涓簨浠舵墍鏀寔鐨勮繃婊ゆā寮忋€?
"format" 鐩綍鎻忚堪浜?perf_event_attr 缁撴瀯鐨?config锛堜簨浠讹級鍜?config1锛堣繃婊ら€夐」锛?瀛楁鐨勬墍鏈夋牸寮忋€?
"identifier" 鏂囦欢鏄剧ず PMU 纭欢璁惧鐨勭増鏈€?
"bdf_min" 鍜?"bdf_max" 鏂囦欢鏄剧ず姣忎釜 pmu 璁惧鎵€鏀寔鐨?bdf 鑼冨洿銆?
"hw_clk_freq" 鏂囦欢鏄剧ず姣忎釜 pmu 璁惧鐨勭‖浠舵椂閽熼鐜囥€?
```

  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/events/dly_tx_normal_to_mac_time
  config=0x00204
  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/events/dly_tx_normal_to_mac_packet_num
  config=0x10204

```
姣忎釜鎬ц兘缁熻閲忛兘鏈変竴瀵逛簨浠讹紝鐢ㄤ簬鑾峰彇涓や釜鍊硷紝浠庤€屽湪璁＄畻锛堢敤鎴风┖闂达級涓畻鍑虹湡瀹炵殑
鎬ц兘鏁版嵁銆?
config 鐨?0~15 浣嶆槸鐪熸鐨勭‖浠朵簨浠剁爜銆傚鏋滀袱涓簨浠剁殑 config 鐨?0~15 浣嶅彇鍊肩浉鍚岋紝
灏辫〃绀哄畠浠槸涓€瀵逛簨浠躲€傝€?config 鐨勭 16 浣嶈〃绀鸿幏鍙栫‖浠朵簨浠惰鏁板櫒 0 杩樻槸璁℃暟鍣?1銆?
鍦ㄧ敤鎴风┖闂磋幏寰椾簨浠跺鐨勪袱涓€间箣鍚庯紝璁＄畻鍏紡濡備笅
```

  counter 0 / counter 1

```
```

  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/filtermode/bw_ssu_rpu_byte_num
  filter mode supported: global/port/port-tc/func/func-queue/

```
```

  $# perf list
  hns3_pmu_sicl_0/bw_ssu_rpu_byte_num/ [kernel PMU event]
  hns3_pmu_sicl_0/bw_ssu_rpu_time/     [kernel PMU event]
  ------------------------------------------

  $# perf stat -g -e hns3_pmu_sicl_0/bw_ssu_rpu_byte_num,global=1/ -e hns3_pmu_sicl_0/bw_ssu_rpu_time,global=1/ -I 1000
  or
  $# perf stat -g -e hns3_pmu_sicl_0/config=0x00002,global=1/ -e hns3_pmu_sicl_0/config=0x10002,global=1/ -I 1000


```
### 杩囨护妯″紡

1. global 妯″紡
PMU 鏀堕泦 IO DIE 鐨勬墍鏈?HNS3 PCIe 鍔熻兘鐨勬€ц兘缁熻淇℃伅銆傚皢 "global" 杩囨护閫夐」璁句负 1
鍗冲彲鍚敤姝ゆā寮忋€?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,global=1/ -I 1000

```
2. port 妯″紡
PMU 鏀堕泦鏁翠釜涓€涓墿鐞嗙鍙ｇ殑鎬ц兘缁熻淇℃伅銆傜鍙?id 涓?mac id 鐩稿悓銆傚湪姝ゆā寮忎笅锛?tc"
杩囨护閫夐」蹇呴』璁句负 0xF锛岃繖閲?tc 浠ｈ〃娴侀噺绫诲埆锛坱raffic class锛夈€?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,port=0,tc=0xF/ -I 1000

```
3. port-tc 妯″紡
PMU 鏀堕泦鐗╃悊绔彛鏌愪竴涓?tc 鐨勬€ц兘缁熻淇℃伅銆傜鍙?id 涓?mac id 鐩稿悓銆傚湪姝ゆā寮忎笅锛?tc"
杩囨护閫夐」蹇呴』璁句负 0 ~ 7銆?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,port=0,tc=0/ -I 1000

```
4. func 妯″紡
PMU 鏀堕泦涓€涓?PF/VF 鐨勬€ц兘缁熻淇℃伅銆傚姛鑳?id 鏄?PF/VF 鐨?BDF锛屽叾涓?```

  func = (bus << 8) + (device << 3) + (function)

```
渚嬪锛?  BDF         func
  35:00.0    0x3500
  35:00.1    0x3501
  35:01.0    0x3508

鍦ㄦ妯″紡涓嬶紝"queue" 杩囨护閫夐」蹇呴』璁句负 0xFFFF銆?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,bdf=0x3500,queue=0xFFFF/ -I 1000

```
5. func-queue 妯″紡
PMU 鏀堕泦涓€涓?PF/VF 鐨勬煇涓€涓槦鍒楃殑鎬ц兘缁熻淇℃伅銆傚姛鑳?id 鏄?PF/VF 鐨?BDF锛?queue"
杩囨护閫夐」蹇呴』璁句负璇ュ姛鑳界‘鍒囩殑闃熷垪 id銆?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,bdf=0x3500,queue=0/ -I 1000

```
6. func-intr 妯″紡
PMU 鏀堕泦涓€涓?PF/VF 鐨勬煇涓€娆′腑鏂殑鎬ц兘缁熻淇℃伅銆傚姛鑳?id 鏄?PF/VF 鐨?BDF锛?intr"
杩囨护閫夐」蹇呴』璁句负璇ュ姛鑳界‘鍒囩殑涓柇 id銆?```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x00301,bdf=0x3500,intr=0/ -I 1000

```
