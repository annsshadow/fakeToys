## NVIDIA Tegra241 SoC 闈炴牳锛圲ncore锛夋€ц兘鐩戞帶鍗曞厓锛圥MU锛?

NVIDIA Tegra241 SoC 鍖呭惈澶氱绯荤粺绾?PMU锛岀敤浜庢祴閲忚濡傚唴瀛樺甫瀹姐€佸欢杩熷拰鍒╃敤鐜囩瓑鍏抽敭鎬ц兘鎸囨爣锛?
- Scalable Coherency Fabric (SCF)
- NVLink-C2C0
- NVLink-C2C1
- CNVLink
- PCIE

### PMU 椹卞姩


鏈枃妗ｄ腑鐨?PMU 鍩轰簬 ARM CoreSight PMU 鏋舵瀯锛屽鏂囨。 ARM IHI 0091 鎵€杩般€傜敱浜庤繖鏄竴绉嶆爣鍑嗘灦鏋勶紝杩欎簺 PMU 鐢变竴涓€氱敤椹卞姩 "arm-cs-arch-pmu" 绠＄悊銆傝椹卞姩鍦?sysfs 涓弿杩版瘡涓?PMU 鍙敤鐨勪簨浠跺拰閰嶇疆銆傝鍙傝涓嬮潰鍚勮妭浠ヨ幏鍙栨瘡涓?PMU 鐨?sysfs 璺緞銆備笌鍏朵粬 uncore PMU 椹卞姩涓€鏍凤紝璇ラ┍鍔ㄦ彁渚?"cpumask" sysfs 灞炴€ф潵鏄剧ず鐢ㄤ簬澶勭悊 PMU 浜嬩欢鐨?CPU id銆傛澶栬繕鏈変竴涓?"associated_cpus" sysfs 灞炴€э紝鍏朵腑鍖呭惈涓庤 PMU 瀹炰緥鍏宠仈鐨?CPU 鍒楄〃銆?

### SCF PMU


SCF PMU 鐩戣绯荤粺绾х紦瀛樹簨浠躲€丆PU 娴侀噺锛屼互鍙婂彂寰€鏈湴/杩滅▼鍐呭瓨鐨勫己搴忥紙SO锛塒CIE 鍐欐祦閲忋€傛湁鍏?PMU 娴侀噺瑕嗙洊鐨勬洿澶氫俊鎭紝璇峰弬瑙?NVIDIA_Uncore_PMU_Traffic_Coverage_Section銆?
璇?PMU 璁惧鐨勪簨浠跺拰閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝瑙?/sys/bus/event_source/devices/nvidia_scf_pmu_<socket-id>銆?
浣跨敤绀轰緥锛?
```

   perf stat -a -e nvidia_scf_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_scf_pmu_1/event=0x0/

```
### NVLink-C2C0 PMU


NVLink-C2C0 PMU 鐩戣鏉ヨ嚜閫氳繃 NVLink-C2C锛圕hip-2-Chip锛変簰杩炶繛鎺ョ殑 GPU/CPU 鐨勪紶鍏ユ祦閲忋€傝 PMU 鎹曡幏鐨勬祦閲忕被鍨嬪彇鍐充簬鑺墖閰嶇疆锛?
- NVIDIA Grace Hopper Superchip锛欻opper GPU 涓?Grace SoC 鐩歌繛銆?
  鍦ㄦ閰嶇疆涓嬶紝PMU 鎹曡幏鏉ヨ嚜 GPU 鐨?GPU ATS 杞崲鎴?EGM 娴侀噺銆?
- NVIDIA Grace CPU Superchip锛氫袱涓?Grace CPU SoC 鐩歌繛銆?
  鍦ㄦ閰嶇疆涓嬶紝PMU 鎹曡幏鏉ヨ嚜杩滅 SoC 鐨?PCIE 璁惧鐨勮鍜屾澗寮涘簭锛圧O锛夊啓銆?
鏈夊叧 PMU 娴侀噺瑕嗙洊鐨勬洿澶氫俊鎭紝璇峰弬瑙?NVIDIA_Uncore_PMU_Traffic_Coverage_Section銆?
璇?PMU 璁惧鐨勪簨浠跺拰閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝瑙?/sys/bus/event_source/devices/nvidia_nvlink_c2c0_pmu_<socket-id>銆?
浣跨敤绀轰緥锛?
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_1/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_2/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_3/event=0x0/

```
NVLink-C2C 鏈変袱涓鍙ｏ紝鍙互杩炴帴鍒颁竴涓?GPU锛堝崰鐢ㄤ袱涓鍙ｏ級鎴栦袱涓?GPU锛堟瘡涓鍙ｄ竴涓?GPU锛夈€傜敤鎴峰彲浠ヤ娇鐢?"port" 浣嶅浘鍙傛暟鏉ラ€夋嫨瑕佺洃瑙嗙殑绔彛銆傛瘡涓€浣嶄唬琛ㄧ鍙ｅ彿锛屼緥濡?"port=0x1" 瀵瑰簲绔彛 0锛?port=0x3" 瀵瑰簲绔彛 0 鍜?1銆傚鏋滄湭鎸囧畾锛孭MU 榛樿鐩戣涓や釜绔彛銆?
绔彛杩囨护绀轰緥锛?
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0,port=0x1/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0,port=0x3/

```
### NVLink-C2C1 PMU


NVLink-C2C1 PMU 鐩戣鏉ヨ嚜閫氳繃 NVLink-C2C锛圕hip-2-Chip锛変簰杩炶繛鎺ョ殑 GPU 鐨勪紶鍏ユ祦閲忋€傝 PMU 鎹曡幏鏈浆鎹㈢殑 GPU 娴侀噺锛岃繖涓庢崟鑾?ATS 杞崲娴侀噺鐨?NVLink-C2C0 PMU 涓嶅悓銆傛湁鍏?PMU 娴侀噺瑕嗙洊鐨勬洿澶氫俊鎭紝璇峰弬瑙?NVIDIA_Uncore_PMU_Traffic_Coverage_Section銆?
璇?PMU 璁惧鐨勪簨浠跺拰閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝瑙?/sys/bus/event_source/devices/nvidia_nvlink_c2c1_pmu_<socket-id>銆?
浣跨敤绀轰緥锛?
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_1/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_2/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_3/event=0x0/

```
NVLink-C2C 鏈変袱涓鍙ｏ紝鍙互杩炴帴鍒颁竴涓?GPU锛堝崰鐢ㄤ袱涓鍙ｏ級鎴栦袱涓?GPU锛堟瘡涓鍙ｄ竴涓?GPU锛夈€傜敤鎴峰彲浠ヤ娇鐢?"port" 浣嶅浘鍙傛暟鏉ラ€夋嫨瑕佺洃瑙嗙殑绔彛銆傛瘡涓€浣嶄唬琛ㄧ鍙ｅ彿锛屼緥濡?"port=0x1" 瀵瑰簲绔彛 0锛?port=0x3" 瀵瑰簲绔彛 0 鍜?1銆傚鏋滄湭鎸囧畾锛孭MU 榛樿鐩戣涓や釜绔彛銆?
绔彛杩囨护绀轰緥锛?
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0,port=0x1/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0,port=0x3/

```
### CNVLink PMU


CNVLink PMU 鐩戣鏉ヨ嚜杩滅鎻掓Ы涓?GPU 鍜?PCIE 璁惧鍙戝線鏈湴鍐呭瓨鐨勬祦閲忋€傚浜?PCIE 娴侀噺锛岃 PMU 鎹曡幏璇诲拰鏉惧紱搴忥紙RO锛夊啓娴侀噺銆傛湁鍏?PMU 娴侀噺瑕嗙洊鐨勬洿澶氫俊鎭紝璇峰弬瑙?NVIDIA_Uncore_PMU_Traffic_Coverage_Section銆?
璇?PMU 璁惧鐨勪簨浠跺拰閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝瑙?/sys/bus/event_source/devices/nvidia_cnvlink_pmu_<socket-id>銆?
姣忎釜 SoC 鎻掓Ы鍙互閫氳繃 CNVLink 杩炴帴鍒颁竴涓垨澶氫釜鎻掓Ы銆傜敤鎴峰彲浠ヤ娇鐢?"rem_socket" 浣嶅浘鍙傛暟鏉ラ€夋嫨瑕佺洃瑙嗙殑杩滅鎻掓Ы銆傛瘡涓€浣嶄唬琛ㄦ彃妲藉彿锛屼緥濡?"rem_socket=0xE" 瀵瑰簲鎻掓Ы 1 鍒?3銆傚鏋滄湭鎸囧畾锛孭MU 榛樿鐩戣鎵€鏈夎繙绔彃妲姐€?/sys/bus/event_source/devices/nvidia_cnvlink_pmu_<socket-id>/format/rem_socket 鏄剧ず鍙互鍦?"rem_socket" 鍙傛暟涓缃殑鏈夋晥浣嶃€?
璇?PMU 鏃犳硶鍖哄垎杩滅娴侀噺鐨勫彂璧疯€咃紝鍥犳涓嶆彁渚涚敤浜庨€夋嫨瑕佺洃瑙嗘祦閲忔簮鐨勮繃婊ゅ櫒銆傚畠鎶ュ憡鏉ヨ嚜杩滅 GPU 鍜?PCIE 璁惧鐨勫悎骞舵祦閲忋€?
浣跨敤绀轰緥锛?
```

   perf stat -a -e nvidia_cnvlink_pmu_0/event=0x0,rem_socket=0xE/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_1/event=0x0,rem_socket=0xD/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_2/event=0x0,rem_socket=0xB/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_3/event=0x0,rem_socket=0x7/


```
### PCIE PMU


PCIE PMU 鐩戣浠?PCIE 鏍圭鍙ｅ彂寰€鏈湴/杩滅▼鍐呭瓨鐨勬墍鏈夎/鍐欐祦閲忋€傛湁鍏?PMU 娴侀噺瑕嗙洊鐨勬洿澶氫俊鎭紝璇峰弬瑙?NVIDIA_Uncore_PMU_Traffic_Coverage_Section銆?
璇?PMU 璁惧鐨勪簨浠跺拰閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝瑙?/sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>銆?
姣忎釜 SoC 鎻掓Ы鍙互鏀寔澶氫釜鏍圭鍙ｃ€傜敤鎴峰彲浠ヤ娇鐢?"root_port" 浣嶅浘鍙傛暟鏉ラ€夋嫨瑕佺洃瑙嗙殑绔彛锛屽嵆 "root_port=0xF" 瀵瑰簲鏍圭鍙?0 鍒?3銆傚鏋滄湭鎸囧畾锛孭MU 榛樿鐩戣鎵€鏈夋牴绔彛銆?/sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>/format/root_port 鏄剧ず鍙互鍦?"root_port" 鍙傛暟涓缃殑鏈夋晥浣嶃€?
浣跨敤绀轰緥锛?
```

   perf stat -a -e nvidia_pcie_pmu_0/event=0x0,root_port=0x3/

```
```

   perf stat -a -e nvidia_pcie_pmu_1/event=0x0,root_port=0x3/

```

### 娴侀噺瑕嗙洊


PMU 鐨勬祦閲忚鐩栧彲鑳藉洜鑺墖閰嶇疆鑰屽紓锛?
- **NVIDIA Grace Hopper Superchip**锛欻opper GPU 涓?Grace SoC 鐩歌繛銆?
```

   *********************************          *********************************
   * SOCKET-A                      *          * SOCKET-B                      *
   *                               *          *                               *
   *                     ::::::::  *          *  ::::::::                     *
   *                     : PCIE :  *          *  : PCIE :                     *
   *                     ::::::::  *          *  ::::::::                     *
   *                         |     *          *      |                        *
   *                         |     *          *      |                        *
   *  :::::::            ::::::::: *          *  :::::::::            ::::::: *
   *  :     :            :       : *          *  :       :            :     : *
   *  : GPU :<--NVLink-->: Grace :<---CNVLink--->: Grace :<--NVLink-->: GPU : *
   *  :     :    C2C     :  SoC  : *          *  :  SoC  :    C2C     :     : *
   *  :::::::            ::::::::: *          *  :::::::::            ::::::: *
   *     |                   |     *          *      |                   |    *
   *     |                   |     *          *      |                   |    *
   *  &&&&&&&&           &&&&&&&&  *          *   &&&&&&&&           &&&&&&&& *
   *  & GMEM &           & CMEM &  *          *   & CMEM &           & GMEM & *
   *  &&&&&&&&           &&&&&&&&  *          *   &&&&&&&&           &&&&&&&& *
   *                               *          *                               *
   *********************************          *********************************

   GMEM = GPU Memory (e.g. HBM)
   CMEM = CPU Memory (e.g. LPDDR5X)

  |
  | Following table contains traffic coverage of Grace SoC PMU in socket-A:

  ::

   +--------------+-------+-----------+-----------+-----+----------+----------+
   |              |                        Source                             |
   +              +-------+-----------+-----------+-----+----------+----------+
   | Destination  |       |GPU ATS    |GPU Not-ATS|     | Socket-B | Socket-B |
   |              |PCI R/W|Translated,|Translated | CPU | CPU/PCIE1| GPU/PCIE2|
   |              |       |EGM        |           |     |          |          |
   +==============+=======+===========+===========+=====+==========+==========+
   | Local        | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF | SCF PMU  | CNVLink  |
   | SYSRAM/CMEM  | PMU   |PMU        |PMU        | PMU |          | PMU      |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Local GMEM   | PCIE  |    N/A    |NVLink-C2C1| SCF | SCF PMU  | CNVLink  |
   |              | PMU   |           |PMU        | PMU |          | PMU      |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Remote       | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF |          |          |
   | SYSRAM/CMEM  | PMU   |PMU        |PMU        | PMU |   N/A    |   N/A    |
   | over CNVLink |       |           |           |     |          |          |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Remote GMEM  | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF |          |          |
   | over CNVLink | PMU   |PMU        |PMU        | PMU |   N/A    |   N/A    |
   +--------------+-------+-----------+-----------+-----+----------+----------+

   PCIE1 traffic represents strongly ordered (SO) writes.
   PCIE2 traffic represents reads and relaxed ordered (RO) writes.

```
- **NVIDIA Grace CPU Superchip**锛氫袱涓?Grace CPU SoC 鐩歌繛銆?
```

   *******************             *******************
   * SOCKET-A        *             * SOCKET-B        *
   *                 *             *                 *
   *    ::::::::     *             *    ::::::::     *
   *    : PCIE :     *             *    : PCIE :     *
   *    ::::::::     *             *    ::::::::     *
   *        |        *             *        |        *
   *        |        *             *        |        *
   *    :::::::::    *             *    :::::::::    *
   *    :       :    *             *    :       :    *
   *    : Grace :<--------NVLink------->: Grace :    *
   *    :  SoC  :    *     C2C     *    :  SoC  :    *
   *    :::::::::    *             *    :::::::::    *
   *        |        *             *        |        *
   *        |        *             *        |        *
   *     &&&&&&&&    *             *     &&&&&&&&    *
   *     & CMEM &    *             *     & CMEM &    *
   *     &&&&&&&&    *             *     &&&&&&&&    *
   *                 *             *                 *
   *******************             *******************

   GMEM = GPU Memory (e.g. HBM)
   CMEM = CPU Memory (e.g. LPDDR5X)

  |
  | Following table contains traffic coverage of Grace SoC PMU in socket-A:

  ::

   +-----------------+-----------+---------+----------+-------------+
   |                 |                      Source                  |
   +                 +-----------+---------+----------+-------------+
   | Destination     |           |         | Socket-B | Socket-B    |
   |                 |  PCI R/W  |   CPU   | CPU/PCIE1| PCIE2       |
   |                 |           |         |          |             |
   +=================+===========+=========+==========+=============+
   | Local           |  PCIE PMU | SCF PMU | SCF PMU  | NVLink-C2C0 |
   | SYSRAM/CMEM     |           |         |          | PMU         |
   +-----------------+-----------+---------+----------+-------------+
   | Remote          |           |         |          |             |
   | SYSRAM/CMEM     |  PCIE PMU | SCF PMU |   N/A    |     N/A     |
   | over NVLink-C2C |           |         |          |             |
   +-----------------+-----------+---------+----------+-------------+

   PCIE1 traffic represents strongly ordered (SO) writes.
   PCIE2 traffic represents reads and relaxed ordered (RO) writes.

```
