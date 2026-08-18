## 娴锋€?PCIe 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?
鍦?Hip09 涓婏紝娴锋€?PCIe 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛夊彲浠ョ洃鎺?PCIe 鐨勫甫瀹姐€佸欢杩熴€佹€荤嚎鍒╃敤鐜囦互鍙婄紦鍐插尯鍗犵敤鏁版嵁銆?
姣忎釜 PCIe Core 閮芥湁涓€涓?PMU锛岀敤浜庣洃鎺ц PCIe Core 鐨勫涓?Root Port 浠ュ強杩欎簺 Root Port 涓嬫父鐨勬墍鏈?Endpoint銆?
## 娴锋€?PCIe PMU 椹卞姩

PCIe PMU 椹卞姩浠ュ畠鐨?sicl-id 鍜?PCIe 鍛藉悕娉ㄥ唽涓€涓?perf PMU
```

  /sys/bus/event_source/hisi_pcie<sicl>_core<core>

```
PMU 椹卞姩鍦?sysfs 涓彁渚涘彲鐢ㄤ簨浠跺拰杩囨护閫夐」鐨勬弿杩帮紝瑙?/sys/bus/event_source/devices/hisi_pcie<sicl>_core<core>銆?
"format" 鐩綍鎻忚堪浜?perf_event_attr 缁撴瀯鐨?config锛堜簨浠讹級鍜?config1锛堣繃婊ら€夐」锛夊瓧娈电殑鎵€鏈夋牸寮忋€?events" 鐩綍鎻忚堪浜?perf list 涓樉绀虹殑鎵€鏈夊凡璁板綍浜嬩欢銆?
"identifier" sysfs 鏂囦欢鍏佽鐢ㄦ埛璇嗗埆 PMU 纭欢璁惧鐨勭増鏈€?
"bus" sysfs 鏂囦欢鍏佽鐢ㄦ埛鑾峰彇琚?PMU 鐩戞帶鐨?Root Port 鐨勬€荤嚎鍙枫€傛澶栵紝鐢ㄦ埛鍙互鍒嗗埆浠?"bdf_min" 鍜?"bdf_max" sysfs 灞炴€ц幏寰?[bdf_min, bdf_max] 涓殑 Root Port 鑼冨洿銆?
```

  $# perf list
  hisi_pcie0_core0/rx_mwr_latency/ [kernel PMU event]
  hisi_pcie0_core0/rx_mwr_cnt/ [kernel PMU event]
  ------------------------------------------

  $# perf stat -e hisi_pcie0_core0/rx_mwr_latency,port=0xffff/
  $# perf stat -e hisi_pcie0_core0/rx_mwr_cnt,port=0xffff/

```
鐩稿叧浜嬩欢閫氬父鐢ㄤ簬璁＄畻甯﹀銆佸欢杩熸垨鍏朵粬鎸囨爣銆傚畠浠渶瑕佸湪鍚屼竴鏃跺埢寮€濮嬪拰缁撴潫璁℃暟锛屽洜姝ょ浉鍏充簨浠舵渶濂藉湪鍚屼竴涓簨浠剁粍涓互寰楀埌鏈熸湜鍊笺€傛湁涓ょ鏂规硶鍙互鐭ラ亾瀹冧滑鏄惁涓虹浉鍏充簨浠讹細

a) 閫氳繃浜嬩欢鍚嶇О锛屼緥濡傚欢杩熶簨浠?"xxx_latency, xxx_cnt" 鎴栧甫瀹戒簨浠?"xxx_flux, xxx_time"銆?b) 閫氳繃浜嬩欢绫诲瀷锛屼緥濡?"event=0xXXXX, event=0x1XXXX"銆?
```

  $# perf stat -e "{hisi_pcie0_core0/rx_mwr_latency,port=0xffff/,hisi_pcie0_core0/rx_mwr_cnt,port=0xffff/}"

```
褰撳墠鐨勯┍鍔ㄤ笉鏀寔閲囨牱銆傚洜姝や笉鏀寔 "perf record"銆傚浜?PCIe PMU 涔熶笉鏀寔缁戝畾鍒颁竴涓换鍔°€?
### 杩囨护閫夐」

1. 鐩爣杩囨护

   PMU 鍙兘鐩戞帶涓嬫父鐩爣 Root Port 鎴栦笅娓哥洰鏍?Endpoint 娴侀噺鐨勬€ц兘銆侾CIe PMU 椹卞姩涓虹敤鎴锋彁渚?"port" 鍜?"bdf" 鎺ュ彛銆?   璇锋敞鎰忥紝杩欎袱涓帴鍙ｅ繀椤昏缃叾涓竴涓紝骞朵笖杩欎袱涓帴鍙ｄ笉鑳藉悓鏃跺彈鏀寔銆傚鏋滀袱鑰呴兘璁剧疆浜嗭紝鍒欏彧鏈?"port" 杩囨护鏈夋晥銆?   濡傛灉 "port" 杩囨护鏈璁剧疆锛屾垨琚樉寮忚涓?0锛堥粯璁ゅ€硷級锛屽垯 "bdf" 杩囨护鐢熸晥锛屽洜涓?"bdf=0" 琛ㄧず 0000:000:00.0銆?
   - port

     "port" 杩囨护鍙敤浜庢墍鏈?PCIe PMU 浜嬩欢锛屽彲浠ラ€氳繃閰嶇疆 16 浣嶄綅鍥?"port" 鏉ラ€夋嫨鐩爣 Root Port銆傚浜?AP 灞備簨浠跺彲浠ラ€夋嫨澶氫釜 port锛岃€屽浜?TL/DL 灞備簨浠跺彧鑳介€夋嫨涓€涓?port銆?
     渚嬪锛屽鏋滅洰鏍?Root Port 鏄?0000:00:00.0锛坸8 閫氶亾锛夛紝搴旇缃綅鍥?bit0锛屽嵆 port=0x1锛涘鏋滅洰鏍?Root Port 鏄?0000:00:04.0锛坸4 閫氶亾锛夛紝璁剧疆 bit8锛屽嵆 port=0x100锛涘鏋滆繖涓や釜 Root Port 閮借鐩戞帶锛屽垯 port=0x101銆?
```

       $# perf stat -e hisi_pcie0_core0/rx_mwr_latency,port=0x1/ sleep 5

   - bdf

     "bdf" 杩囨护鍙兘鐢ㄤ簬甯﹀浜嬩欢锛岄€氳繃鎶?BDF 閰嶇疆涓?"bdf" 鏉ラ€夋嫨鐩爣 Endpoint銆傝鏁板櫒鍙粺璁＄敱鐩爣 Endpoint 璇锋眰鐨勬秷鎭殑甯﹀銆?
     渚嬪锛?bdf=0x3900" 琛ㄧず鐩爣 Endpoint 鐨?BDF 鏄?0000:39:00.0銆?
     perf 鐢ㄦ硶绀轰緥濡備笅锛?
       $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,bdf=0x3900/ sleep 5

```
2. 瑙﹀彂杩囨护

   褰?TLP 闀垮害绗竴娆″ぇ浜?灏忎簬瑙﹀彂鏉′欢鏃讹紝浜嬩欢缁熻寮€濮嬨€傚彲浠ラ€氳繃鍐欏叆 "trig_len" 璁剧疆瑙﹀彂鏉′欢锛岄€氳繃鍐欏叆 "trig_mode" 璁剧疆瑙﹀彂妯″紡銆傝杩囨护鍙兘鐢ㄤ簬甯﹀浜嬩欢銆?
   渚嬪锛?trig_len=4" 琛ㄧず瑙﹀彂鏉′欢涓?2^4 DW锛?trig_mode=0" 琛ㄧず褰?TLP 闀垮害 > 瑙﹀彂鏉′欢鏃剁粺璁″紑濮嬶紝"trig_mode=1" 琛ㄧず褰?TLP 闀垮害 < 鏉′欢鏃跺紑濮嬨€?
```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,trig_len=0x4,trig_mode=1/ sleep 5

```
3. 闃堝€艰繃婊?
   褰?TLP 闀垮害鍦ㄦ寚瀹氳寖鍥村唴鏃惰鏁般€傚彲浠ラ€氳繃鍐欏叆 "thr_len" 璁剧疆闃堝€硷紝閫氳繃鍐欏叆 "thr_mode" 璁剧疆闃堝€兼ā寮忋€傝杩囨护鍙兘鐢ㄤ簬甯﹀浜嬩欢銆?
   渚嬪锛?thr_len=4" 琛ㄧず闃堝€间负 2^4 DW锛?thr_mode=0" 琛ㄧず褰?TLP 闀垮害 >= 闃堝€兼椂璁℃暟锛?thr_mode=1" 琛ㄧず褰?TLP 闀垮害 < 闃堝€兼椂璁℃暟銆?
```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,thr_len=0x4,thr_mode=1/ sleep 5

```
4. TLP 闀垮害杩囨护

   鍦ㄧ粺璁″甫瀹芥椂锛屾暟鎹彲浠ョ敱 TLP 鍖呯殑鏌愪簺閮ㄥ垎缁勬垚銆備綘鍙互閫氳繃 "len_mode" 鎸囧畾锛?
   - 2'b00锛氫繚鐣欙紙涓嶈浣跨敤锛屽洜涓鸿涓烘湭瀹氫箟锛?   - 2'b01锛歍LP 杞借嵎鐨勫甫瀹?   - 2'b10锛歍LP 澶寸殑甯﹀
   - 2'b11锛歍LP 杞借嵎鍜屽ご鐨勫甫瀹?
   渚嬪锛?len_mode=2" 琛ㄧず鍙粺璁?TLP 澶寸殑甯﹀锛?len_mode=3" 琛ㄧず鏈€缁堝甫瀹芥暟鎹敱 TLP 澶村拰杞借嵎鍏卞悓缁勬垚銆傛湭鎸囧畾鏃堕粯璁ゅ€间负 2'b11銆?
```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,len_mode=0x1/ sleep 5

```
