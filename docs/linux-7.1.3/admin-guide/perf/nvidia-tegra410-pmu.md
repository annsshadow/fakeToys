## NVIDIA Tegra410 SoC 闈炴牳蹇冩€ц兘鐩戞帶鍗曞厓锛圥MU锛?

NVIDIA Tegra410 SoC 鍖呭惈澶氫釜绯荤粺绾?PMU锛岀敤浜庢祴閲忚濡傚唴瀛樺甫瀹姐€佸欢杩熷拰鍒╃敤鐜囩瓑鍏抽敭鎬ц兘鎸囨爣锛?
- Unified Coherence Fabric (UCF)
- PCIE
- PCIE-TGT
- CPU Memory (CMEM) Latency
- NVLink-C2C
- NV-CLink
- NV-DLink

### PMU 椹卞姩


璇?PMU 椹卞姩鍦?sysfs 涓弿杩版瘡涓?PMU 鍙敤鐨勪簨浠朵笌閰嶇疆銆傝鍙傞槄浠ヤ笅鍚勮妭浠ヨ幏鍙栨瘡涓?PMU 鐨?sysfs 璺緞銆備笌鍏朵粬闈炴牳蹇冿紙uncore锛塒MU 椹卞姩涓€鏍凤紝璇ラ┍鍔ㄦ彁渚?"cpumask" sysfs 灞炴€т互鏄剧ず鐢ㄤ簬澶勭悊璇?PMU 浜嬩欢鐨?CPU id銆傝繕鏈変竴涓?"associated_cpus" sysfs 灞炴€э紝鍏朵腑鍖呭惈涓庤 PMU 瀹炰緥鐩稿叧鑱旂殑涓€缁?CPU銆?
### UCF PMU


NVIDIA Tegra410 SoC 涓殑 Unified Coherence Fabric (UCF) 鍏呭綋涓€涓垎甯冨紡缂撳瓨锛圕PU 鍐呭瓨涓?CXL 鍐呭瓨鐨勬渶鍚庝竴绾х紦瀛橈級锛屼互鍙婃敮鎸佸涓繛璐紦瀛樹唬鐞嗭紙coherently caching agent锛変箣闂寸‖浠朵竴鑷存€х殑缂撳瓨涓€鑷翠簰杩烇紝杩欎簺浠ｇ悊鍖呮嫭锛?
  - CPU 绨囷紙cluster锛?  - GPU
  - PCIe 鎺掑簭鎺у埗鍣ㄥ崟鍏冿紙OCU, Ordering Controller Unit锛?  - 鍏朵粬 IO 涓€鑷磋姹傝€?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝鍙傝 /sys/bus/event_source/devices/nvidia_ucf_pmu_<socket-id>銆?
姝?PMU 涓彲鐢ㄧ殑閮ㄥ垎浜嬩欢鍙敤浜庢祴閲忓甫瀹戒笌鍒╃敤鐜囷細

  - slc_access_rd锛氱粺璁″ SLC 鐨勮璇锋眰鏁伴噺銆?  - slc_access_wr锛氱粺璁″ SLC 鐨勫啓璇锋眰鏁伴噺銆?  - slc_bytes_rd锛氱粺璁＄敱 slc_access_rd 浼犺緭鐨勫瓧鑺傛暟銆?  - slc_bytes_wr锛氱粺璁＄敱 slc_access_wr 浼犺緭鐨勫瓧鑺傛暟銆?  - mem_access_rd锛氱粺璁″鏈湴鎴栬繙绔唴瀛樼殑璇昏姹傛暟閲忋€?  - mem_access_wr锛氱粺璁″鏈湴鎴栬繙绔唴瀛樼殑鍐欒姹傛暟閲忋€?  - mem_bytes_rd锛氱粺璁＄敱 mem_access_rd 浼犺緭鐨勫瓧鑺傛暟銆?  - mem_bytes_wr锛氱粺璁＄敱 mem_access_wr 浼犺緭鐨勫瓧鑺傛暟銆?  - cycles锛氱粺璁?UCF 鍛ㄦ湡鏁般€?
```

   AVG_SLC_READ_BANDWIDTH_IN_GBPS = SLC_BYTES_RD / ELAPSED_TIME_IN_NS
   AVG_SLC_WRITE_BANDWIDTH_IN_GBPS = SLC_BYTES_WR / ELAPSED_TIME_IN_NS
   AVG_MEM_READ_BANDWIDTH_IN_GBPS = MEM_BYTES_RD / ELAPSED_TIME_IN_NS
   AVG_MEM_WRITE_BANDWIDTH_IN_GBPS = MEM_BYTES_WR / ELAPSED_TIME_IN_NS

```
```

   AVG_SLC_READ_REQUEST_RATE = SLC_ACCESS_RD / CYCLES
   AVG_SLC_WRITE_REQUEST_RATE = SLC_ACCESS_WR / CYCLES
   AVG_MEM_READ_REQUEST_RATE = MEM_ACCESS_RD / CYCLES
   AVG_MEM_WRITE_REQUEST_RATE = MEM_ACCESS_WR / CYCLES

```
鍏充簬杩樻湁鍝簺鍏朵粬浜嬩欢鍙敤鐨勬洿澶氱粏鑺傦紝鍙湪 Tegra410 SoC 鎶€鏈弬鑰冩墜鍐屼腑鎵惧埌銆?
杩欎簺浜嬩欢鍙互鏍规嵁婧愭垨鐩殑鍦拌繘琛岃繃婊ゃ€傛簮杩囨护鍣ㄦ寚绀哄彂寰€ SLC 鐨勬祦閲忓彂璧疯€咃紝渚嬪鏈湴 CPU銆侀潪 CPU 璁惧锛屾垨杩滅 socket銆傜洰鐨勮繃婊ゅ櫒鎸囧畾鐩殑鍐呭瓨绫诲瀷锛屼緥濡傛湰鍦扮郴缁熷唴瀛橈紙CMEM锛夈€佹湰鍦?GPU 鍐呭瓨锛圙MEM锛夛紝鎴栬繙绔唴瀛樸€傜洰鐨勮繃婊ゅ櫒鐨勬湰鍦?杩滅鍒嗙被鍩轰簬鍦板潃鐨?home socket锛岃€屼笉鏄暟鎹疄闄呮墍鍦ㄧ殑浣嶇疆銆傚彲鐢ㄧ殑杩囨护鍣ㄥ湪 /sys/bus/event_source/devices/nvidia_ucf_pmu_<socket-id>/format/ 涓弿杩般€?
UCF PMU 浜嬩欢杩囨护鍣ㄥ垪琛細

- 婧愯繃婊ゅ櫒锛?
  - src_loc_cpu锛氳嫢璁剧疆锛岀粺璁℃潵鑷湰鍦?CPU 鐨勪簨浠?  - src_loc_noncpu锛氳嫢璁剧疆锛岀粺璁℃潵鑷湰鍦伴潪 CPU 璁惧鐨勪簨浠?  - src_rem锛氳嫢璁剧疆锛岀粺璁℃潵鑷繙绔?socket 鐨?CPU銆丟PU銆丳CIE 璁惧鐨勪簨浠?
- 鐩殑杩囨护鍣細

  - dst_loc_cmem锛氳嫢璁剧疆锛岀粺璁″埌鏈湴绯荤粺鍐呭瓨锛圕MEM锛夊湴鍧€鐨勪簨浠?  - dst_loc_gmem锛氳嫢璁剧疆锛岀粺璁″埌鏈湴 GPU 鍐呭瓨锛圙MEM锛夊湴鍧€鐨勪簨浠?  - dst_loc_other锛氳嫢璁剧疆锛岀粺璁″埌鏈湴 CXL 鍐呭瓨鍦板潃鐨勪簨浠?  - dst_rem锛氳嫢璁剧疆锛岀粺璁″埌杩滅 socket 鐨?CPU銆丟PU 鍜?CXL 鍐呭瓨鍦板潃鐨勪簨浠?
濡傛灉鏈寚瀹氭簮锛孭MU 灏嗙粺璁℃潵鑷墍鏈夋簮鐨勪簨浠躲€傚鏋滄湭鎸囧畾鐩殑锛孭MU 灏嗙粺璁″埌鎵€鏈夌洰鐨勫湴鐨勪簨浠躲€?
浣跨敤绀轰緥锛?
```

    perf stat -a -e nvidia_ucf_pmu_0/event=0x0/

```
- 鍦?socket 0 涓粺璁′簨浠?id 0x0锛屾簮杩囨护鍣?= 鏈湴 CPU锛岀洰鐨勮繃婊ゅ櫒
```

    perf stat -a -e nvidia_ucf_pmu_0/event=0x0,src_loc_cpu=0x1,dst_loc_cmem=0x1/

```
- 鍦?socket 1 涓粺璁′簨浠?id 0x0锛屾簮杩囨护鍣?= 鏈湴闈?CPU 璁惧锛屼笖
```

    perf stat -a -e nvidia_ucf_pmu_1/event=0x0,src_loc_noncpu=0x1,dst_rem=0x1/

```
### PCIE PMU


姝?PMU 浣嶄簬杩炴帴 PCIE 鏍瑰鍚堜綋锛圧C, root complex锛変笌鍐呭瓨瀛愮郴缁熺殑 SoC 浜掕繛涓€傚畠鐩戞帶鏉ヨ嚜鏍圭鍙ｏ紙root port锛夋垨鏌愪釜 PCIE RC 涓壒瀹?BDF 鍒版湰鍦版垨杩滅鍐呭瓨鐨勬墍鏈夎/鍐欐祦閲忋€係oC 涓瘡涓?PCIE RC 鏈変竴涓?PMU銆傛瘡涓?RC 鏈€澶氬彲鏈?16 鏉￠€氶亾锛坙ane锛夛紝鍙鍒嗗弶锛坆ifurcate锛変负鏈€澶?8 涓牴绔彛銆傛潵鑷瘡涓牴绔彛鐨勬祦閲忓彲浠ヤ娇鐢?RP 鎴?BDF 杩囨护鍣ㄨ繘琛岃繃婊ゃ€備緥濡傦紝鎸囧畾 "src_rp_mask=0xFF" 琛ㄧず PMU 璁℃暟鍣ㄥ皢鎹曡幏鏉ヨ嚜鎵€鏈?RP 鐨勬祦閲忋€傛洿澶氱粏鑺傝鍙傞槄涓嬫枃銆?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝鍙傝 /sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>_rc_<pcie-rc-id>銆?
姝?PMU 涓殑浜嬩欢鍙敤浜庢祴閲忓甫瀹姐€佸埄鐢ㄧ巼鍜屽欢杩燂細

  - rd_req锛氱粺璁?PCIE 璁惧鐨勮璇锋眰鏁伴噺銆?  - wr_req锛氱粺璁?PCIE 璁惧鐨勫啓璇锋眰鏁伴噺銆?  - rd_bytes锛氱粺璁＄敱 rd_req 浼犺緭鐨勫瓧鑺傛暟銆?  - wr_bytes锛氱粺璁＄敱 wr_req 浼犺緭鐨勫瓧鑺傛暟銆?  - rd_cum_outs锛氱粺璁℃瘡涓懆鏈熺殑鏈畬鎴愶紙outstanding锛塺d_req銆?  - cycles锛氱粺璁¤繛鎺ュ埌 PCIE 鎺ュ彛鐨?SoC 浜掕繛鐨勬椂閽熷懆鏈熸暟銆?
```

   AVG_RD_BANDWIDTH_IN_GBPS = RD_BYTES / ELAPSED_TIME_IN_NS
   AVG_WR_BANDWIDTH_IN_GBPS = WR_BYTES / ELAPSED_TIME_IN_NS

```
```

   AVG_RD_REQUEST_RATE = RD_REQ / CYCLES
   AVG_WR_REQUEST_RATE = WR_REQ / CYCLES


```
```

   FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS
   AVG_LATENCY_IN_CYCLES = RD_CUM_OUTS / RD_REQ
   AVERAGE_LATENCY_IN_NS = AVG_LATENCY_IN_CYCLES / FREQ_IN_GHZ

```
PMU 浜嬩欢鍙互鏍规嵁娴侀噺婧愬拰鐩殑鍦拌繘琛岃繃婊ゃ€傛簮杩囨护鍣ㄦ寚绀哄皢琚洃鎺х殑 PCIE 璁惧銆傜洰鐨勮繃婊ゅ櫒鎸囧畾鐩殑鍐呭瓨绫诲瀷锛屼緥濡傛湰鍦扮郴缁熷唴瀛橈紙CMEM锛夈€佹湰鍦?GPU 鍐呭瓨锛圙MEM锛夛紝鎴栬繙绔唴瀛樸€傜洰鐨勮繃婊ゅ櫒鐨勬湰鍦?杩滅鍒嗙被鍩轰簬鍦板潃鐨?home socket锛岃€屼笉鏄暟鎹疄闄呮墍鍦ㄧ殑浣嶇疆銆傝繖浜涜繃婊ゅ櫒鍙湪 /sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>_rc_<pcie-rc-id>/format/ 涓壘鍒般€?
浜嬩欢杩囨护鍣ㄥ垪琛細

- 婧愯繃婊ゅ櫒锛?
  - src_rp_mask锛氬皢琚洃鎺х殑鏍圭鍙ｇ殑浣嶆帺鐮侊紙bitmask锛夈€傛浣嶆帺鐮佷腑鐨勬瘡涓€浣嶄唬琛?RC 涓殑 RP 绱㈠紩銆傝嫢鏌愪綅琚疆浣嶏紝鍒欏叧鑱?RP 涓嬬殑鎵€鏈夎澶囬兘灏嗚鐩戞帶銆備緥濡?"src_rp_mask=0xF" 灏嗙洃鎺ф牴绔彛 0 鍒?3 涓殑璁惧銆?  - src_bdf锛氬皢琚洃鎺х殑 BDF銆傝繖鏄竴涓?16 浣嶅€硷紝閬靛惊鍏紡锛?bus << 8) + (device << 3) + (function)銆備緥濡傦紝BDF 27:01.1 鐨勫€间负 0x2781銆?  - src_bdf_en锛氬惎鐢?BDF 杩囨护鍣ㄣ€傝嫢璁剧疆锛屽垯浣跨敤 "src_bdf" 涓殑 BDF 杩囨护鍊兼潵杩囨护娴侀噺銆?
  娉ㄦ剰锛孯oot-Port 涓?BDF 杩囨护鍣ㄦ槸浜掓枼鐨勶紝涓旀瘡涓?RC 涓殑 PMU 瀵逛簬鏁翠釜璁℃暟鍣ㄥ彧鑳芥湁涓€涓?BDF 杩囨护鍣ㄣ€傚鏋滃惎鐢ㄤ簡 BDF 杩囨护鍣紝璇?BDF 杩囨护鍊煎皢搴旂敤浜庢墍鏈変簨浠躲€?
- 鐩殑杩囨护鍣細

  - dst_loc_cmem锛氳嫢璁剧疆锛岀粺璁″埌鏈湴绯荤粺鍐呭瓨锛圕MEM锛夊湴鍧€鐨勪簨浠?  - dst_loc_gmem锛氳嫢璁剧疆锛岀粺璁″埌鏈湴 GPU 鍐呭瓨锛圙MEM锛夊湴鍧€鐨勪簨浠?  - dst_loc_pcie_p2p锛氳嫢璁剧疆锛岀粺璁″埌鏈湴 PCIE 瀵圭瓑锛坧eer锛夊湴鍧€鐨勪簨浠?  - dst_loc_pcie_cxl锛氳嫢璁剧疆锛岀粺璁″埌鏈湴 CXL 鍐呭瓨鍦板潃鐨勪簨浠?  - dst_rem锛氳嫢璁剧疆锛岀粺璁″埌杩滅鍐呭瓨鍦板潃鐨勪簨浠?
濡傛灉鏈寚瀹氭簮杩囨护鍣紝PMU 灏嗙粺璁℃潵鑷墍鏈夋牴绔彛鐨勪簨浠躲€傚鏋滄湭鎸囧畾鐩殑杩囨护鍣紝PMU 灏嗙粺璁″埌鎵€鏈夌洰鐨勫湴鐨勪簨浠躲€?
浣跨敤绀轰緥锛?
- 鍦?socket 0 涓婄粺璁℃潵鑷?PCIE RC-0 鏍圭鍙?0銆佺洰鏍囦负鎵€鏈夌洰鐨勫湴鐨勪簨浠?id 0x0
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_0/event=0x0,src_rp_mask=0x1/

```
- 鍦?socket 0 涓婄粺璁℃潵鑷?PCIE RC-1 鏍圭鍙?0 鍜?1銆佷笖
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_1/event=0x1,src_rp_mask=0x3,dst_loc_cmem=0x1/

```
- 鍦?socket 1 涓婄粺璁℃潵鑷?PCIE RC-2 鏍圭鍙?0銆佺洰鏍囦负鎵€鏈夌洰鐨勫湴鐨勪簨浠?id 0x2
```

    perf stat -a -e nvidia_pcie_pmu_1_rc_2/event=0x2,src_rp_mask=0x1/

```
- 鍦?socket 1 涓婄粺璁℃潵鑷?PCIE RC-3 鏍圭鍙?0 鍜?1銆佷笖
```

    perf stat -a -e nvidia_pcie_pmu_1_rc_3/event=0x3,src_rp_mask=0x3,dst_loc_cmem=0x1/

```
- 鍦?socket 0 涓婄粺璁℃潵鑷?PCIE RC-4 鐨?BDF 01:01.0銆佺洰鏍囦负鎵€鏈夌洰鐨勫湴鐨勪簨浠?id 0x4
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_4/event=0x4,src_bdf=0x0180,src_bdf_en=0x1/

```

#### 灏?RC# 鏄犲皠鍒?lspci 娈靛彿


灏?RC# 鏄犲皠鍒?lspci 娈靛彿鍙兘骞朵笉瀹规槗锛涘洜姝や负姣忎釜 RP 鍦?PCIE 閰嶇疆绌洪棿涓坊鍔犱簡涓€涓柊鐨?NVIDIA 鎸囧畾鍘傚晢鐗瑰畾鑳藉姏锛圖VSEC, Designated Vendor Specific Capability锛夊瘎瀛樺櫒銆傛 DVSEC 鐨勫巶鍟?id 涓?"10de"锛孌VSEC id 涓?"0x4"銆傝 DVSEC 瀵勫瓨鍣ㄥ寘鍚互涓嬩俊鎭紝鐢ㄤ簬灏?RP 涓嬬殑 PCIE 璁惧鏄犲皠鍥炲叾 RC#锛?
  - Bus#锛堝瓧鑺?0xc锛夛細鐢?lspci 杈撳嚭鎶ュ憡鐨?bus 鍙?  - Segment#锛堝瓧鑺?0xd锛夛細鐢?lspci 杈撳嚭鎶ュ憡鐨?segment 鍙?  - RP#锛堝瓧鑺?0xe锛夛細瀵逛簬鍏锋湁鏍圭鍙ｈ兘鍔涚殑璁惧锛岀敱 lspci 鐨?LnkCap 灞炴€ф姤鍛婄殑绔彛鍙?  - RC#锛堝瓧鑺?0xf锛夛細涓庤 RP 鍏宠仈鐨勬牴澶嶅悎浣撳彿
  - Socket#锛堝瓧鑺?0x10锛夛細涓庤 RP 鍏宠仈鐨?socket 鍙?
```

  #!/bin/bash
  while read bdf rest; do
    dvsec4_reg=$(lspci -vv -s $bdf | awk '
      /Designated Vendor-Specific: Vendor=10de ID=0004/ {
        match($0, /\[([0-9a-fA-F]+)/, arr);
        print "0x" arr[1];
        exit
      }
    ')
    if [ -n "$dvsec4_reg" ]; then
      bus=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xc))).b)
      segment=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xd))).b)
      rp=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xe))).b)
      rc=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xf))).b)
      socket=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0x10))).b)
      echo "$bdf: Bus=$bus, Segment=$segment, RP=$rp, RC=$rc, Socket=$socket"
    fi
  done < <(lspci -d 10de:)

```
```

  0001:00:00.0: Bus=00, Segment=01, RP=00, RC=00, Socket=00
  0002:80:00.0: Bus=80, Segment=02, RP=01, RC=01, Socket=00
  0002:a0:00.0: Bus=a0, Segment=02, RP=02, RC=01, Socket=00
  0002:c0:00.0: Bus=c0, Segment=02, RP=03, RC=01, Socket=00
  0002:e0:00.0: Bus=e0, Segment=02, RP=04, RC=01, Socket=00
  0003:00:00.0: Bus=00, Segment=03, RP=00, RC=02, Socket=00
  0004:00:00.0: Bus=00, Segment=04, RP=00, RC=03, Socket=00
  0005:00:00.0: Bus=00, Segment=05, RP=00, RC=04, Socket=00
  0005:40:00.0: Bus=40, Segment=05, RP=01, RC=04, Socket=00
  0005:c0:00.0: Bus=c0, Segment=05, RP=02, RC=04, Socket=00
  0006:00:00.0: Bus=00, Segment=06, RP=00, RC=05, Socket=00
  0009:00:00.0: Bus=00, Segment=09, RP=00, RC=00, Socket=01
  000a:80:00.0: Bus=80, Segment=0a, RP=01, RC=01, Socket=01
  000a:a0:00.0: Bus=a0, Segment=0a, RP=02, RC=01, Socket=01
  000a:e0:00.0: Bus=e0, Segment=0a, RP=03, RC=01, Socket=01
  000b:00:00.0: Bus=00, Segment=0b, RP=00, RC=02, Socket=01
  000c:00:00.0: Bus=00, Segment=0c, RP=00, RC=03, Socket=01
  000d:00:00.0: Bus=00, Segment=0d, RP=00, RC=04, Socket=01
  000d:40:00.0: Bus=40, Segment=0d, RP=01, RC=04, Socket=01
  000d:c0:00.0: Bus=c0, Segment=0d, RP=02, RC=04, Socket=01
  000e:00:00.0: Bus=00, Segment=0e, RP=00, RC=05, Socket=01

```
### PCIE-TGT PMU


姝?PMU 浣嶄簬杩炴帴 PCIE 鏍瑰鍚堜綋锛圧C锛変笌鍐呭瓨瀛愮郴缁熺殑 SoC 浜掕繛涓€傚畠鐩戞帶浠?PCIE BAR 鍜?CXL HDM 鑼冨洿涓虹洰鏍囩殑娴侀噺銆係oC 涓瘡涓?PCIE RC 鏈変竴涓?PCIE-TGT PMU銆俆egra410 SoC 涓殑姣忎釜 RC 鏈€澶氬彲鏈?16 鏉￠€氶亾锛屽彲琚垎鍙変负鏈€澶?8 涓牴绔彛锛圧P锛夈€傝 PMU 鎻愪緵 RP 杩囨护鍣ㄦ潵缁熻鍒版瘡涓?RP 鐨?PCIE BAR 娴侀噺锛屼互鍙婂湴鍧€杩囨护鍣ㄦ潵缁熻瀵?PCIE BAR 鎴?CXL HDM 鑼冨洿鐨勮闂€傝繃婊ゅ櫒鐨勭粏鑺傚湪浠ヤ笅鍚勮妭鎻忚堪銆?
灏?RC# 鏄犲皠鍒?lspci 娈靛彿鐨勬柟寮忎笌 PCIE PMU 绫讳技銆傛洿澶氫俊鎭鍙傞槄 NVIDIA_T410_PCIE_PMU_RC_Mapping_Section銆?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓彲鐢紝鍙傝 /sys/bus/event_source/devices/nvidia_pcie_tgt_pmu_<socket-id>_rc_<pcie-rc-id>銆?
姝?PMU 涓殑浜嬩欢鍙敤浜庢祴閲忓甫瀹藉拰鍒╃敤鐜囷細

  - rd_req锛氱粺璁″埌 PCIE 鐨勮璇锋眰鏁伴噺銆?  - wr_req锛氱粺璁″埌 PCIE 鐨勫啓璇锋眰鏁伴噺銆?  - rd_bytes锛氱粺璁＄敱 rd_req 浼犺緭鐨勫瓧鑺傛暟銆?  - wr_bytes锛氱粺璁＄敱 wr_req 浼犺緭鐨勫瓧鑺傛暟銆?  - cycles锛氱粺璁¤繛鎺ュ埌 PCIE 鎺ュ彛鐨?SoC 浜掕繛鐨勬椂閽熷懆鏈熸暟銆?
```

   AVG_RD_BANDWIDTH_IN_GBPS = RD_BYTES / ELAPSED_TIME_IN_NS
   AVG_WR_BANDWIDTH_IN_GBPS = WR_BYTES / ELAPSED_TIME_IN_NS

```
```

   AVG_RD_REQUEST_RATE = RD_REQ / CYCLES
   AVG_WR_REQUEST_RATE = WR_REQ / CYCLES

```
PMU 浜嬩欢鍙互鏍规嵁鐩殑鏍圭鍙ｆ垨鐩爣鍦板潃鑼冨洿杩涜杩囨护銆傚熀浜?RP 鐨勮繃婊や粎瀵?PCIE BAR 娴侀噺鍙敤銆傚湴鍧€杩囨护鍣ㄥ PCIE BAR 鍜?CXL HDM 鑼冨洿閮芥湁鏁堛€傝繖浜涜繃婊ゅ櫒鍙湪 sysfs 涓壘鍒帮紝鍙傝 /sys/bus/event_source/devices/nvidia_pcie_tgt_pmu_<socket-id>_rc_<pcie-rc-id>/format/銆?
鐩殑杩囨护鍣ㄨ缃細

- dst_rp_mask锛氶€夋嫨瑕佺洃鎺х殑鏍圭鍙ｇ殑浣嶆帺鐮併€備緥濡?"dst_rp_mask=0xFF" 瀵瑰簲 PCIE RC 涓殑鎵€鏈夋牴绔彛锛堜粠 0 鍒?7锛夈€傛敞鎰忔杩囨护鍣ㄤ粎瀵?PCIE BAR 娴侀噺鍙敤銆?- dst_addr_base锛欱AR 鎴?CXL HDM 杩囨护鍣ㄥ熀鍧€銆?- dst_addr_mask锛欱AR 鎴?CXL HDM 杩囨护鍣ㄥ湴鍧€鎺╃爜銆?- dst_addr_en锛氬惎鐢?BAR 鎴?CXL HDM 鍦板潃鑼冨洿杩囨护鍣ㄣ€傝嫢璁剧疆锛屽垯浣跨敤 "dst_addr_base" 鍜?"dst_addr_mask" 鎸囧畾鐨勫湴鍧€鑼冨洿鏉ヨ繃婊?PCIE BAR 鍜?CXL HDM 娴侀噺鍦板潃銆侾MU 浣跨敤濡備笅姣旇緝
```

    (txn's addr & dst_addr_mask) == (dst_addr_base & dst_addr_mask)

  濡傛灉姣旇緝鎴愬姛锛屽垯璇ヤ簨浠朵細琚粺璁°€?
```
濡傛灉鏈寚瀹氱洰鐨勮繃婊ゅ櫒锛孯P 杩囨护鍣ㄩ粯璁や細琚厤缃负缁熻鍒版墍鏈夋牴绔彛鐨?PCIE BAR 娴侀噺銆?
浣跨敤绀轰緥锛?
```

    perf stat -a -e nvidia_pcie_tgt_pmu_0_rc_0/event=0x0,dst_rp_mask=0x3/

```
- 缁熻瀵?PCIE BAR 鎴?CXL HDM 鍦板潃鑼冨洿璁块棶鐨勪簨浠?id 0x1
```

    perf stat -a -e nvidia_pcie_tgt_pmu_0_rc_1/event=0x1,dst_addr_base=0x10000,dst_addr_mask=0xFFF00,dst_addr_en=0x1/

```
### CPU Memory (CMEM) Latency PMU


姝?PMU 鐩戞帶浠?Unified Coherence Fabric (UCF) 杈圭紭鍒版湰鍦?CPU DRAM 鐨勫唴瀛樿璇锋眰寤惰繜浜嬩欢锛?
  - RD_REQ 璁℃暟鍣細缁熻璇昏姹傛暟閲忥紙姣忎釜璇锋眰 32B锛夈€?  - RD_CUM_OUTS 璁℃暟鍣細绱鏈畬鎴愯姹傝鏁板櫒锛岃窡韪璇锋眰澶勪簬鍦ㄩ€旓紙in flight锛夌姸鎬佺殑鍛ㄦ湡鏁般€?  - CYCLES 璁℃暟鍣細缁熻缁忚繃鐨勫懆鏈熸暟銆?
```

   FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS
   AVG_LATENCY_IN_CYCLES = RD_CUM_OUTS / RD_REQ
   AVERAGE_LATENCY_IN_NS = AVG_LATENCY_IN_CYCLES / FREQ_IN_GHZ

```
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓弿杩帮紝鍙傝 /sys/bus/event_source/devices/nvidia_cmem_latency_pmu_<socket-id>銆?
```

  perf stat -a -e '{nvidia_cmem_latency_pmu_0/rd_req/,nvidia_cmem_latency_pmu_0/rd_cum_outs/,nvidia_cmem_latency_pmu_0/cycles/}'

```
### NVLink-C2C PMU


姝?PMU 鐩戞帶绌胯繃 NVIDIA Chip-to-Chip (C2C) 鎺ュ彛鐨勮/鍐欏唴瀛樿姹傜殑寤惰繜浜嬩欢銆備笌 Grace锛圱egra241 SoC锛変腑鐨?C2C PMU 涓嶅悓锛屾 PMU 涓病鏈夊甫瀹戒簨浠躲€?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓彲鐢紝鍙傝 /sys/bus/event_source/devices/nvidia_nvlink_c2c_pmu_<socket-id>銆?
浜嬩欢鍒楄〃锛?
  - IN_RD_CUM_OUTS锛氳繘鍏ョ殑璇昏姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - IN_RD_REQ锛氳繘鍏ョ殑璇昏姹傛暟閲忋€?  - IN_WR_CUM_OUTS锛氳繘鍏ョ殑鍐欒姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - IN_WR_REQ锛氳繘鍏ョ殑鍐欒姹傛暟閲忋€?  - OUT_RD_CUM_OUTS锛氬彂鍑虹殑璇昏姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - OUT_RD_REQ锛氬彂鍑虹殑璇昏姹傛暟閲忋€?  - OUT_WR_CUM_OUTS锛氬彂鍑虹殑鍐欒姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - OUT_WR_REQ锛氬彂鍑虹殑鍐欒姹傛暟閲忋€?  - CYCLES锛歂VLink-C2C 鎺ュ彛鍛ㄦ湡璁℃暟銆?
杩涘叆锛坕ncoming锛夌殑浜嬩欢缁熻浠庤繙绔澶囧埌 SoC 鐨勮/鍐欍€傚彂鍑虹殑锛坥utgoing锛変簨浠剁粺璁′粠 SoC 鍒拌繙绔澶囩殑璇?鍐欍€?
sysfs 涓殑 /sys/bus/event_source/devices/nvidia_nvlink_c2c_pmu_<socket-id>/peer 鍖呭惈鎵€杩炴帴璁惧鐨勪俊鎭€?
褰?C2C 鎺ュ彛杩炴帴鍒?GPU 鏃讹紝鐢ㄦ埛鍙互浣跨敤 "gpu_mask" 鍙傛暟鏉ヨ繃婊ゅ埌/鏉ヨ嚜鐗瑰畾 GPU 鐨勬祦閲忋€傛瘡涓€浣嶄唬琛?GPU 绱㈠紩锛屼緥濡?"gpu_mask=0x1" 瀵瑰簲 GPU 0锛?gpu_mask=0x3" 瀵瑰簲 GPU 0 鍜?1銆傚鏋滄湭鎸囧畾锛孭MU 榛樿鐩戞帶鎵€鏈?GPU銆?
褰撹繛鎺ュ埌鍙︿竴涓?SoC 鏃讹紝鍙湁璇讳簨浠跺彲鐢ㄣ€?
```

   C2C_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   IN_WR_AVG_LATENCY_IN_CYCLES = IN_WR_CUM_OUTS / IN_WR_REQ
   IN_WR_AVG_LATENCY_IN_NS = IN_WR_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   OUT_RD_AVG_LATENCY_IN_CYCLES = OUT_RD_CUM_OUTS / OUT_RD_REQ
   OUT_RD_AVG_LATENCY_IN_NS = OUT_RD_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   OUT_WR_AVG_LATENCY_IN_CYCLES = OUT_WR_CUM_OUTS / OUT_WR_REQ
   OUT_WR_AVG_LATENCY_IN_NS = OUT_WR_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

```
浣跨敤绀轰緥锛?
```

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_req/

  * Count incoming traffic from GPU 0 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_cum_outs,gpu_mask=0x1/

  * Count incoming traffic from GPU 1 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_cum_outs,gpu_mask=0x2/

  * Count outgoing traffic to all GPUs connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_req/

  * Count outgoing traffic to GPU 0 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_cum_outs,gpu_mask=0x1/

  * Count outgoing traffic to GPU 1 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_cum_outs,gpu_mask=0x2/

```
### NV-CLink PMU


姝?PMU 鐩戞帶绌胯繃 NV-CLINK 鎺ュ彛鐨勮鍐呭瓨璇锋眰鐨勫欢杩熶簨浠躲€傛 PMU 涓病鏈夊甫瀹戒簨浠躲€傚湪 Tegra410 SoC 涓紝NV-CLink 鎺ュ彛鐢ㄤ簬杩炴帴鍒板彟涓€涓?Tegra410 SoC锛屼笖姝?PMU 鍙粺璁¤娴侀噺銆?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓彲鐢紝鍙傝 /sys/bus/event_source/devices/nvidia_nvclink_pmu_<socket-id>銆?
浜嬩欢鍒楄〃锛?
  - IN_RD_CUM_OUTS锛氳繘鍏ョ殑璇昏姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - IN_RD_REQ锛氳繘鍏ョ殑璇昏姹傛暟閲忋€?  - OUT_RD_CUM_OUTS锛氬彂鍑虹殑璇昏姹傜殑绱鏈畬鎴愯姹傦紙浠ュ懆鏈熻锛夈€?  - OUT_RD_REQ锛氬彂鍑虹殑璇昏姹傛暟閲忋€?  - CYCLES锛歂V-CLINK 鎺ュ彛鍛ㄦ湡璁℃暟銆?
杩涘叆锛坕ncoming锛夌殑浜嬩欢缁熻浠庤繙绔澶囧埌 SoC 鐨勮銆傚彂鍑虹殑锛坥utgoing锛変簨浠剁粺璁′粠 SoC 鍒拌繙绔澶囩殑璇汇€?
```

   CLINK_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / CLINK_FREQ_IN_GHZ

   OUT_RD_AVG_LATENCY_IN_CYCLES = OUT_RD_CUM_OUTS / OUT_RD_REQ
   OUT_RD_AVG_LATENCY_IN_NS = OUT_RD_AVG_LATENCY_IN_CYCLES / CLINK_FREQ_IN_GHZ

```
浣跨敤绀轰緥锛?
```

      perf stat -a -e nvidia_nvclink_pmu_0/in_rd_req/

  * Count outgoing read traffic to remote SoC connected via NV-CLINK::

      perf stat -a -e nvidia_nvclink_pmu_0/out_rd_req/

```
### NV-DLink PMU


姝?PMU 鐩戞帶绌胯繃 NV-DLINK 鎺ュ彛鐨勮鍐呭瓨璇锋眰鐨勫欢杩熶簨浠躲€傛 PMU 涓病鏈夊甫瀹戒簨浠躲€傚湪 Tegra410 SoC 涓紝姝?PMU 鍙粺璁?CXL 鍐呭瓨璇绘祦閲忋€?
姝?PMU 璁惧鐨勪簨浠朵笌閰嶇疆閫夐」鍦?sysfs 涓彲鐢紝鍙傝 /sys/bus/event_source/devices/nvidia_nvdlink_pmu_<socket-id>銆?
浜嬩欢鍒楄〃锛?
  - IN_RD_CUM_OUTS锛氬埌 CXL 鍐呭瓨鐨勭疮璁℃湭瀹屾垚璇昏姹傦紙浠ュ懆鏈熻锛夈€?  - IN_RD_REQ锛氬埌 CXL 鍐呭瓨鐨勮璇锋眰鏁伴噺銆?  - CYCLES锛歂V-DLINK 鎺ュ彛鍛ㄦ湡璁℃暟銆?
```

   DLINK_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / DLINK_FREQ_IN_GHZ

```
浣跨敤绀轰緥锛?
```

      perf stat -a -e '{nvidia_nvdlink_pmu_0/in_rd_req/,nvidia_nvdlink_pmu_0/in_rd_cum_outs/}'

```
