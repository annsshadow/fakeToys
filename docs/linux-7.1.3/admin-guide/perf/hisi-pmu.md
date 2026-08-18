## 娴锋€濓紙HiSilicon锛塖oC 闈炴牳锛坲ncore锛夋€ц兘鐩戞帶鍗曞厓锛圥MU锛?

娴锋€?SoC 鑺墖鍖呭惈鍚勭鐙珛鐨勭郴缁熻澶?PMU锛屼緥濡?L3 缂撳瓨锛圠3C锛夈€丠ydra Home Agent锛圚HA锛夊拰 DDRC銆傝繖浜?PMU 鏄嫭绔嬬殑锛屽苟鍏锋湁鏀堕泦缁熻鍜屾€ц兘淇℃伅鐨勭‖浠堕€昏緫銆?
娴锋€?SoC 灏佽浜嗗涓?CPU 鍜?I/O 瑁哥墖锛坉ie锛夈€傛瘡涓?CPU 绨囷紙CCL锛夌敱 4 涓叡浜竴涓?L3 缂撳瓨鐨?CPU 鏍哥粍鎴愶紱姣忎釜 CPU 瑁哥墖绉颁负瓒呯骇 CPU 绨囷紙SCCL锛夛紝鐢?6 涓?CCL 缁勬垚銆傛瘡涓?SCCL 鍒嗗埆鏈変袱涓?HHA锛? - 1锛夊拰鍥涗釜 DDRC锛? - 3锛夈€?
### 娴锋€?SoC 闈炴牳 PMU 椹卞姩


姣忎釜璁惧 PMU 閮芥湁鐢ㄤ簬浜嬩欢璁℃暟銆佹帶鍒跺拰涓柇鐨勭嫭绔嬪瘎瀛樺櫒锛孭MU 椹卞姩搴旀敞鍐屽儚 L3C銆丠HA 鍜?DDRC 绛?perf PMU 椹卞姩銆傚彲鐢ㄧ殑浜嬩欢鍜岄厤缃€夐」搴斾綅浜?```
```
/sys/bus/event_source/devices/hisi_sccl{X}_<l3c{Y}/hha{Y}/ddrc{Y}>

"perf list" 鍛戒护搴斾粠 sysfs 鍒楀嚭鍙敤浜嬩欢銆?
姣忎釜 L3C銆丠HA 鍜?DDRC 閮戒綔涓虹嫭绔嬬殑 PMU 娉ㄥ唽鍒?perf銆侾MU 鍚嶇О鍦ㄤ簨浠跺垪琛ㄤ腑鏄剧ず涓?hisi_sccl<sccl-id>_module<index-id>銆傚叾涓?"sccl-id" 鏄?SCCL 鐨勬爣璇嗙锛?index-id" 鏄ā鍧楃殑绱㈠紩銆?
渚嬪锛歨isi_sccl3_l3c0/rd_hit_cpipe 鏄?SCCL ID #3 涓?L3C 绱㈠紩 #0 鐨?READ_HIT_CPIPE 浜嬩欢銆?
渚嬪锛歨isi_sccl1_hha0/rx_operations 鏄?SCCL ID #1 涓?HHA 绱㈠紩 #0 鐨?RX_OPERATIONS 浜嬩欢銆?
椹卞姩杩樻彁渚涗竴涓?"cpumask" sysfs 灞炴€э紝鏄剧ず鐢ㄤ簬璁℃暟闈炴牳 PMU 浜嬩欢鐨?CPU 鏍?ID銆傝繕鎻愪緵 "associated_cpus" sysfs 灞炴€т互鏄剧ず涓庢 PMU 鍏宠仈鐨?CPU銆?cpumask" 鎸囩ず鎵撳紑浜嬩欢鐨?CPU锛岄€氬父浣滀负鍍?perf 杩欐牱鐨勭敤鎴风┖闂村伐鍏风殑鎻愮ず銆?瀹冨彧鍖呭惈鏉ヨ嚜 "associated_cpus" 鐨勪竴涓叧鑱?CPU銆?
```
  $# perf list
  hisi_sccl3_l3c0/rd_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl3_l3c0/wr_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl1_l3c0/rd_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl1_l3c0/wr_hit_cpipe/ [kernel PMU event]
  ------------------------------------------

  $# perf stat -a -e hisi_sccl3_l3c0/rd_hit_cpipe/ sleep 5
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02/ sleep 5
```

瀵逛簬鏍囪瘑绗︿负 0x30 鐨勬捣鎬濋潪鏍?PMU v2锛屽叾鎷撴墤涓?PMU v1 鐩稿悓锛屼絾纭欢涓婂鍔犱簡涓€浜涙柊鍔熻兘銆?
1. L3C PMU 鏀寔鎸夌皣鍐呯殑鏍?绾跨▼杩涜杩囨护锛屽彲閫氳繃
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02,tt_core=0x3/ sleep 5
```
杩欏彧浼氳鏁拌绨囦腑鏍?绾跨▼ 0 鍜?1 鐨勬搷浣溿€?
鐢ㄦ埛涓嶅簲浣跨敤 tt_core_deprecated 鏉ユ寚瀹氭牳/绾跨▼杩囨护銆傝閫夐」浠呬负浜嗗悜鍚庡吋瀹硅€屾彁渚涳紝涓斿彧鏀寔 8 浣嶏紝鍙兘鏃犳硶瑕嗙洊鍏变韩 L3C 鐨勬墍鏈夋牳/绾跨▼銆?
2. Tracetag 鍏佽鐢ㄦ埛閫氳繃 perf 涓殑 tt_req 鍙傛暟閫夋嫨鍙鏁拌銆佸啓鎴栧師瀛愭搷浣溿€傞粯璁よ鏁版墍鏈夋搷浣溿€倀t_req 涓?3 浣嶏紝3'b100 琛ㄧず璇绘搷浣滐紝3'b101 琛ㄧず鍐欐搷浣滐紝3'b110 琛ㄧず鍘熷瓙瀛樺偍鎿嶄綔锛屼笖
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02,tt_req=0x4/ sleep 5
```
杩欏彧浼氳鏁拌绨囦腑鐨勮鎿嶄綔銆?
3. Datasrc 鍏佽鐢ㄦ埛妫€鏌ユ暟鎹潵鑷綍澶勩€傚畠鏄?5 浣嶃€備竴浜涢噸瑕佺紪鐮佸涓嬶細

- 5'b00001锛氭潵鑷湰瑁哥墖鐨?L3C锛?- 5'b01000锛氭潵鑷法瑁哥墖鐨?L3C锛?- 5'b01001锛氭潵鑷彟涓€涓彃妲斤紙socket锛夌殑 L3C锛?- 5'b01110锛氭潵鑷湰鍦?DDR锛?- 5'b01111锛氭潵鑷法瑁哥墖鐨?DDR锛?- 5'b10000锛氭潵鑷法鎻掓Ы鐨?DDR锛?
绛夌瓑锛屽畠涓昏鏈夊姪浜庡彂鐜版暟鎹簮璺濈 CPU 鏍告渶杩戙€傚鏋滃湪澶氳姱鐗囦腑浣跨敤 datasrc_cfg锛屽垯 datasrc_skt 搴斾负
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0xb9,datasrc_cfg=0xE/,
  hisi_sccl3_l3c0/config=0xb9,datasrc_cfg=0xF/ sleep 5
```

4. 涓€浜涙捣鎬?SoC 灏佽浜嗗涓?CPU 鍜?I/O 瑁哥墖銆傛瘡涓?CPU 瑁哥墖鍖呭惈鑻ュ共璁＄畻绨囷紙CCL锛夈€侷/O 瑁哥墖绉颁负瓒呯骇 I/O 绨囷紙SICL锛夛紝鍖呭惈澶氫釜 I/O 绨囷紙ICL锛夈€係oC 涓殑姣忎釜 CCL/ICL 閮芥湁涓€涓敮涓€ ID銆傛瘡涓?ID 涓?11 浣嶏紝鍖呭惈 6 浣嶇殑 SCCL-ID 鍜?5 浣嶇殑 CCL/ICL-ID銆傚浜?I/O 瑁哥墖锛孖CL-ID 鍚庤窡锛?
- 5'b00000锛欼/O_MGMT_ICL锛?- 5'b00001锛歂etwork_ICL锛?- 5'b00011锛欻AC_ICL锛?- 5'b10000锛歅CIe_ICL锛?
5. uring_channel锛歎C PMU 浜嬩欢 0x47~0x59 鏀寔鎸?tx request uring 閫氶亾杩涜杩囨护銆傚畠鏄?2 浣嶃€備竴浜涢噸瑕佺紪鐮佸涓嬶細

- 2'b11锛氳鏁板彂閫佸埌 uring_ext锛圡ATA锛夐€氶亾鐨勪簨浠讹紱
- 2'b01锛氫笌 2'b11 鐩稿悓锛?- 2'b10锛氳鏁板彂閫佸埌 uring锛堥潪 MATA锛夐€氶亾鐨勪簨浠讹紱
- 2'b00锛氶粯璁ゅ€硷紝璁℃暟鍙戦€佸埌 uring 鍜?uring_ext 涓や釜閫氶亾鐨勪簨浠讹紱

6. ch锛歂oC PMU 鏀寔浣跨敤姝ら€夐」杩囨护鐗瑰畾浜嬪姟閫氶亾鐨勪簨浠惰鏁般€傚綋鍓嶆敮鎸佺殑閫氶亾濡備笅锛?
- 3'b010锛氳姹傞€氶亾锛圧equest channel锛?- 3'b100锛氫睛鍚€氶亾锛圫noop channel锛?- 3'b110锛氬搷搴旈€氶亾锛圧esponse channel锛?- 3'b111锛氭暟鎹€氶亾锛圖ata channel锛?
7. tt_en锛氬鏋滆缃簡姝ら€夐」锛孨oC PMU 浠呮敮鎸佽鏁拌缃簡 tracetag 鐨勪簨鍔°€傛湁鍏?tracetag 鐨勬洿澶氫俊鎭鍙傝绗?2 鏉″垪琛ㄣ€?
瀵逛簬鏍囪瘑绗︿负 0x40 鐨勬捣鎬濋潪鏍?PMU v3锛屼竴浜涢潪鏍?PMU 琚繘涓€姝ュ垝鍒嗕负鑻ュ共閮ㄥ垎浠ヨ幏寰楁洿缁嗙矑搴︾殑杩借釜锛屾瘡涓儴鍒嗘湁鑷繁鐨勪笓鐢?PMU锛屾墍鏈夎繖浜?PMU 涓€璧疯鐩栫壒瀹氶潪鏍歌澶囦笂鐨勪簨浠剁洃鎺т换鍔°€傛绫?PMU 鍦?sysfs 涓互濡備笅鍚嶇О鏍煎紡鎻忚堪锛?```
```
/sys/bus/event_source/devices/hisi_sccl{X}_<l3c{Y}_{Z}/ddrc{Y}_{Z}/noc{Y}_{Z}>

Z 鏄瓙 ID锛坰ub-id锛夛紝琛ㄧず纭欢璁惧鏌愪釜閮ㄥ垎鐨?PMU銆?
澶у鏁板叿鏈変笉鍚屽瓙 ID 鐨?PMU 鐢ㄦ硶鐩稿悓銆傜壒鍒湴锛孡3C PMU 鎻愪緵 `ext` 閫夐」浠ュ厑璁告帰绱?L3C PMU 鏇寸粏绮掑害鐨勭粺璁°€侺3C PMU 椹卞姩鍦ㄥ悜纭欢涓嬪彂 perf 鍛戒护鏃跺皢鍏剁敤浣滅粓姝㈡彁绀猴細

- ext=0锛氶粯璁わ紝鍙笌浜嬩欢鍚嶄竴璧蜂娇鐢ㄣ€?- ext=1 鍜?ext=2锛氬繀椤讳笌浜嬩欢鐮佷竴璧蜂娇鐢紝涓嶆敮鎸佷簨浠跺悕銆?
```
  $# perf stat -a -e hisi_sccl0_l3c1_0/rd_spipe/ sleep 5
```
```
  $# perf stat -a -e hisi_sccl0_l3c1_0/event=0x1,ext=1/ sleep 5
```
濡備笂锛宍hisi_sccl0_l3c1_0` 瀹氫綅鍒拌秴绾?CPU 绨?0銆丩3 缂撳瓨 1 鐨?pipe0銆?
绗竴鏉″懡浠ゅ畾浣嶅埌 L3C 鐨勭涓€閮ㄥ垎锛屽洜涓洪粯璁ら殣鍚?`ext=0`銆傜浜屾潯鍛戒护鍦?L3C 鐨勫彟涓€閮ㄥ垎涓婁互浜嬩欢 `0x1` 杩涜璁℃暟銆?
鐢ㄦ埛鍙互閫氳繃璁剧疆 srcid_cmd & srcid_msk 鏉ラ厤缃?ID 浠ヨ鏁版潵鑷壒瀹?CCL/ICL 鐨勬暟鎹紝骞堕€氳繃璁剧疆 tgtid_cmd & tgtid_msk 鏉ヨ鏁板彂寰€鐗瑰畾 CCL/ICL 鐨勬暟鎹€俿rcid_msk/tgtid_msk 涓疆浣嶇殑浣嶈〃绀?PMU 鍦ㄥ尮閰?srcid_cmd/tgtid_cmd 鏃朵笉浼氭鏌ヨ浣嶃€?
濡傛灉鎵€鏈夎繖浜涢€夐」閮借绂佺敤锛屽畠鍙互鎸夐粯璁ゅ€煎伐浣滐紝鍗充笉鍖哄垎杩囨护鏉′欢鍜?ID 淇℃伅锛屽苟杩斿洖 PMU 璁℃暟鍣ㄤ腑鐨勬€昏鏁板櫒鍊笺€?
褰撳墠椹卞姩涓嶆敮鎸侀噰鏍枫€傚洜姝や笉鏀寔 "perf record"銆傚悓鏍凤紝鐢变簬浜嬩欢閮芥槸闈炴牳鐨勶紝涔熶笉鏀寔闄勫姞鍒颁换鍔★紙task锛夈€?
娉ㄦ剰锛氬鏋滈渶瑕侊紝璇疯仈绯荤淮鎶よ€呰幏鍙?SoC 涓?PMU 璁惧鏀寔鐨勫畬鏁翠簨浠跺垪琛ㄥ強鍏朵俊鎭€?