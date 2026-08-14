## Synopsys DesignWare Cores (DWC) PCIe 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?


## DesignWare Cores (DWC) PCIe PMU


璇?PMU 鏄敱姣忎釜 PCIe Root Port 鍦ㄥ悕涓?RAS D.E.S锛圖ebug銆丒rror injection銆丼tatistics锛岃皟璇曘€侀敊璇敞鍏ヤ笌缁熻锛夌殑鍘傚晢鐗瑰畾鎵╁睍鑳藉姏锛圴endor-Specific Extended Capability锛変腑鎻愪緵鐨?PCIe 閰嶇疆绌洪棿瀵勫瓨鍣ㄥ潡銆?

椤惧悕鎬濅箟锛孯AS DES 鑳藉姏鏀寔绯荤粺绾ц皟璇曘€丄ER 閿欒娉ㄥ叆浠ュ強缁熻淇℃伅鐨勬敹闆嗐€備负渚夸簬缁熻淇℃伅鐨勬敹闆嗭紝Synopsys DesignWare Cores PCIe 鎺у埗鍣ㄦ彁渚涗互涓嬩袱涓壒鎬э細

- 涓€涓敤浜庡熀浜庢椂闂村垎鏋愶紙RX/TX 鏁版嵁鍚炲悙閲忎笌鍦ㄥ悇浣庡姛鑰?LTSSM 鐘舵€佷笂鑺辫垂鐨勬椂闂达級鐨?64 浣嶈鏁板櫒锛屼互鍙?
- 姣忎釜浜嬩欢涓€涓?32 浣嶈鏁板櫒锛岀敤浜庝簨浠惰鏁帮紙鎸囧畾閫氶亾鐨勯敊璇笌闈為敊璇簨浠讹級

娉ㄦ剰锛氳鏁板櫒婧㈠嚭娌℃湁涓柇銆?

### 鍩轰簬鏃堕棿鐨勫垎鏋?


浣跨敤璇ョ壒鎬э紝浣犲彲浠ヨ幏寰楁湁鍏虫帶鍒跺櫒 RX/TX 鏁版嵁鍚炲悙閲忎笌鍦ㄥ悇浣庡姛鑰?LTSSM 鐘舵€佷笂鑺辫垂鏃堕棿鐨勪俊鎭€侾MU 灏嗘暟鎹殑娴嬮噺鍒嗕负涓ょ被锛?

- 缁?0锛氭帶鍒跺櫒鍋滅暀鍦?LTSSM 鐘舵€佺殑鏃堕棿鐧惧垎姣斻€?
- 缁?1锛氬鐞嗙殑鏁版嵁閲忥紙浠?16 瀛楄妭涓哄崟浣嶏級銆?

### 閫氶亾浜嬩欢璁℃暟鍣?


浣跨敤璇ョ壒鎬э紝浣犲彲浠ヨ幏寰楁帶鍒跺櫒鍦ㄧ壒瀹氶€氶亾涓婄殑閿欒涓庨潪閿欒淇℃伅銆侾MU 浜嬩欢鐢变互涓嬪叏閮ㄩ€夋嫨锛?

- 缁?i
- 缁?i 涓殑浜嬩欢 j
- 閫氶亾 k

鏌愪簺浜嬩欢浠呭瓨鍦ㄤ簬鐗瑰畾閰嶇疆涓€?

## DesignWare Cores (DWC) PCIe PMU 椹卞姩


璇ラ┍鍔ㄤ负姣忎釜 PCIe Root Port 娣诲姞 PMU 璁惧锛屽悕绉板熀浜庤 Root Port 鐨?SBDF銆備緥濡傦紝

    0001:30:03.0 PCI bridge: Device 1ded:8000 (rev 01)

璇?Root Port 鐨?PMU 璁惧鍚嶇О涓?dwc_rootport_13018銆?

DWC PCIe PMU 椹卞姩娉ㄥ唽涓€涓?perf PMU 椹卞姩锛屽畠鍦?sysfs 涓彁渚涘彲鐢ㄤ簨浠朵笌閰嶇疆閫夐」鐨勬弿杩帮紝瑙?/sys/bus/event_source/devices/dwc_rootport_{sbdf}銆?

"format" 鐩綍鎻忚堪 perf_event_attr 缁撴瀯浣撶殑 config 瀛楁鏍煎紡銆?events" 鐩綍涓烘墍鏈夊凡鏂囨。鍖栫殑浜嬩欢鎻愪緵閰嶇疆妯℃澘銆備緥濡傦紝"rx_pcie_tlp_data_payload" 绛変环浜?"eventid=0x21,type=0x0"銆?

```

    $# perf list | grep dwc_rootport
    <...>
    dwc_rootport_13018/Rx_PCIe_TLP_Data_Payload/        [Kernel PMU event]
    <...>
    dwc_rootport_13018/rx_memory_read,lane=?/               [Kernel PMU event]

```

### 鍩轰簬鏃堕棿鐨勫垎鏋愪簨浠剁敤娉?


```

    $# perf stat -a -e dwc_rootport_13018/Rx_PCIe_TLP_Data_Payload/

```

骞冲潎 RX/TX 甯﹀鍙娇鐢ㄤ互涓嬪叕寮忚绠楋細

    PCIe RX Bandwidth = rx_pcie_tlp_data_payload / Measure_Time_Window
    PCIe TX Bandwidth = tx_pcie_tlp_data_payload / Measure_Time_Window

### 閫氶亾浜嬩欢鐢ㄦ硶


姣忎釜閫氶亾鍏锋湁鐩稿悓鐨勪簨浠堕泦鍚堬紝涓洪伩鍏嶇敓鎴愭暟鐧句釜鏉＄洰鐨勫垪琛?
```

    $# perf stat -a -e dwc_rootport_13018/rx_memory_read,lane=4/

```

璇ラ┍鍔ㄤ笉鏀寔閲囨牱锛屽洜姝?"perf record" 鏃犳硶宸ヤ綔銆備笉鏀寔鎸変换鍔★紙涓嶅甫 "-a"锛夌殑 perf 浼氳瘽銆?
