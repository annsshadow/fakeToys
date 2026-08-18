
## Debugging and tracing in the media subsystem


鏈枃妗ｄ綔涓鸿捣鐐瑰拰鏌ラ槄鎵嬪唽锛岀敤浜庡湪 media 瀛愮郴缁熶腑璋冭瘯璁惧椹卞姩锛屼互鍙婁粠鐢ㄦ埛绌洪棿璋冭瘯杩欎簺椹卞姩銆?
    :depth: 3

### General debugging advice


涓€鑸€у缓璁鍙傞槄 :doc:`閫氱敤寤鸿鏂囨。 </process/debugging/index>`銆?
浠ヤ笅鍚勮妭鍚戜綘灞曠ず涓€浜涘彲鐢ㄧ殑宸ュ叿銆?
### dev_debug module parameter


姣忎釜瑙嗛璁惧閮芥彁渚涗竴涓?`dev_debug` 鍙傛暟锛屽彲鐢ㄤ簬鑾峰彇
```

  # cat /sys/class/video4linux/video3/name
  rkvdec
  # echo 0xff > /sys/class/video4linux/video3/dev_debug
  # dmesg -wH
  [...] videodev: v4l2_open: video3: open (0)
  [  +0.000036] video3: VIDIOC_QUERYCAP: driver=rkvdec, card=rkvdec,
  bus=platform:rkvdec, version=0x00060900, capabilities=0x84204000,
  device_caps=0x04204000

```
瀹屾暣鏂囨。璇峰弬闃?:ref:`driver-api/media/v4l2-dev:video device debugging`

### dev_dbg() / v4l2_dbg()


涓や釜鐗瑰畾浜庤澶囧拰 v4l2 瀛愮郴缁熺殑璋冭瘯鎵撳嵃璇彞锛岄櫎闈炲畠浠璋冩煡鍏锋湁闀挎湡浠峰€硷紝鍚﹀垯涓嶈鎶婂畠浠姞鍏ヤ綘鐨勬渶缁堟彁浜や腑銆?
姒傝璇峰弬闃?process/debugging/driver_development_debugging_guide:printk() & friends 鎸囧崡銆?
- 涓よ€呭尯鍒紵

  - v4l2_dbg() 搴曞眰浣跨敤 v4l2_printk()锛屽悗鑰呰繘涓€姝ョ洿鎺ヤ娇鐢?printk()锛屽洜姝ゆ棤娉曡 dynamic debug 瀹氫綅
  - dev_dbg() 鍙互琚?dynamic debug 瀹氫綅
  - v4l2_dbg() 瀵?media 瀛愮郴缁熸湁鏇寸壒瀹氱殑鍓嶇紑鏍煎紡锛岃€?dev_dbg 鍙珮浜樉绀洪┍鍔ㄥ悕鍜屾棩蹇椾綅缃?
### Dynamic debug


涓€绉嶆牴鎹綘鐨勯渶瑕佽鍓皟璇曡緭鍑虹殑鏂规硶銆?
涓€鑸€у缓璁鍙傞槄 process/debugging/userspace_debugging_guide:dynamic debug 鎸囧崡銆?
```

  $ alias ddcmd='echo $* > /proc/dynamic_debug/control'
  $ ddcmd '-p; file v4l2-h264.c +p'
  $ grep =p /proc/dynamic_debug/control
   drivers/media/v4l2-core/v4l2-h264.c:372 [v4l2_h264]print_ref_list_b =p
   "ref_pic_list_b%u (cur_poc %u%c) %s"
   drivers/media/v4l2-core/v4l2-h264.c:333 [v4l2_h264]print_ref_list_p =p
   "ref_pic_list_p (cur_poc %u%c) %s\n"

```
### Ftrace


涓€绉嶅彲浠ヨ拷韪潤鎬侀瀹氫箟浜嬩欢銆佸嚱鏁拌皟鐢ㄧ瓑鐨勫唴鏍稿唴閮?tracer銆傚浜庡湪涓嶄慨鏀瑰唴鏍哥殑鎯呭喌涓嬭皟璇曢棶棰樹互鍙婄悊瑙ｅ瓙绯荤粺鐨勮涓洪潪甯告湁鐢ㄣ€?
涓€鑸€у缓璁鍙傞槄 process/debugging/userspace_debugging_guide:ftrace 鎸囧崡銆?
### DebugFS


璇ュ伐鍏峰厑璁镐綘鎶婇┍鍔ㄧ殑鍐呴儴鍊艰浆鍌ㄦ垨淇敼鍒拌嚜瀹氫箟鏂囦欢绯荤粺涓殑鏂囦欢閲屻€?
涓€鑸€у缓璁鍙傞槄 process/debugging/driver_development_debugging_guide:debugfs 鎸囧崡銆?
### Perf & alternatives


鐢ㄤ簬鍦ㄨ繍琛屼腑鐨勭郴缁熶笂娴嬮噺鍚勭缁熻淇℃伅浠ヨ瘖鏂棶棰樼殑宸ュ叿銆?
涓€鑸€у缓璁鍙傞槄 process/debugging/userspace_debugging_guide:perf & alternatives 鎸囧崡銆?
media 璁惧绀轰緥锛?
鏀堕泦瑙ｇ爜浠诲姟鐨勭粺璁℃暟鎹細锛堟绀轰緥鍦ㄥ甫鏈?rkvdec 缂栬В鐮佸櫒椹卞姩銆佷娇鐢?`fluster test suite
```

  perf stat -d python3 fluster.py run -d GStreamer-H.264-V4L2SL-Gst1.0 -ts
  JVT-AVC_V1 -tv AUD_MW_E -j1
  ...
  Performance counter stats for 'python3 fluster.py run -d
  GStreamer-H.264-V4L2SL-Gst1.0 -ts JVT-AVC_V1 -tv AUD_MW_E -j1 -v':

         7794.23 msec task-clock:u                     #    0.697 CPUs utilized
               0      context-switches:u               #    0.000 /sec
               0      cpu-migrations:u                 #    0.000 /sec
           11901      page-faults:u                    #    1.527 K/sec
       882671556      cycles:u                         #    0.113 GHz                         (95.79%)
       711708695      instructions:u                   #    0.81  insn per cycle              (95.79%)
        10581935      branches:u                       #    1.358 M/sec                       (15.13%)
         6871144      branch-misses:u                  #   64.93% of all branches             (95.79%)
       281716547      L1-dcache-loads:u                #   36.144 M/sec                       (95.79%)
         9019581      L1-dcache-load-misses:u          #    3.20% of all L1-dcache accesses   (95.79%)
 <not supported>      LLC-loads:u
 <not supported>      LLC-load-misses:u

    11.180830431 seconds time elapsed

     1.502318000 seconds user
     6.377221000 seconds sys

```
鍙敤浜嬩欢鍜屾寚鏍囧彇鍐充簬浣犳墍杩愯鐨勭郴缁熴€?
### Error checking & panic analysis


鍚勭鍐呮牳閰嶇疆閫夐」锛屼互澧炲己 Linux 鍐呮牳鐨勯敊璇娴嬭兘鍔涳紝浠ｄ环鏄檷浣庢€ц兘銆?
涓€鑸€у缓璁鍙傞槄 :ref:`process/debugging/driver_development_debugging_guide:kasan, ubsan, lockdep and other error checkers` 鎸囧崡銆?
### Driver verification with v4l2-compliance


涓轰簡楠岃瘉椹卞姩鏄惁閬靛惊 v4l2 API锛屼娇鐢ㄥ伐鍏?v4l2-compliance锛屽畠鏄?`v4l_utils <https://git.linuxtv.org/v4l-utils.git>`__ 鐨勪竴閮ㄥ垎锛屽悗鑰呮槸涓€濂楃敤浜?media 瀛愮郴缁熺殑鐢ㄦ埛绌洪棿宸ュ叿銆?
```

  v4l2-compliance -M /dev/mediaX --verbose

```
浣犱篃鍙互瀵?mediaX 寮曠敤鐨勬墍鏈夎澶囪繍琛屽畬鏁寸殑鍚堣鎬ф鏌?```

  v4l2-compliance -m /dev/mediaX

```
### Debugging problems with receiving video


鍦ㄩ┍鍔ㄤ腑瀹炵幇 vidioc_log_status锛氳繖鍙互鎶婂綋鍓嶇姸鎬佽褰曞埌鍐呮牳鏃ュ織銆傚畠鐢?v4l2-ctl --log-status 璋冪敤銆傚浜庤皟璇曟帴鏀惰棰戯紙TV/S-Video/HDMI 绛夛級鐨勯棶棰橀潪甯告湁鐢紝鍥犱负瑙嗛淇″彿鏄閮ㄧ殑锛堝洜姝や笉鍙娴嬶級銆傚浜庢憚鍍忓ご浼犳劅鍣ㄨ緭鍏ョ敤澶勮緝灏忥紝鍥犱负浣犲彲浠ユ帶鍒舵憚鍍忓ご浼犳劅鍣ㄧ殑琛屼负銆?
```

  .vidioc_log_status  = v4l2_ctrl_log_status,

```
浣嗕綘涔熷彲浠ュ垱寤鸿嚜宸辩殑鍥炶皟锛屼互鍒涘缓鑷畾涔夌殑鐘舵€佹棩蹇椼€?
浣犲彲浠ュ湪 cobalt 椹卞姩涓壘鍒颁竴涓ず渚嬶紙`drivers/media/pci/cobalt/cobalt-v4l2.c <https://elixir.bootlin.com/linux/v6.11.6/source/drivers/media/pci/cobalt/cobalt-v4l2.c#L567>`__锛夈€?
**Copyright** 漏2024 : Collabora
