
## 瀛愮郴缁熻窡韪偣锛歅CI 鎺у埗鍣?


## 姒傝堪

PCI鎺у埗鍣ㄨ窡韪郴缁熸彁渚涜窡韪偣鏉ョ洃鎺ф帶鍒跺櫒
鐢ㄤ簬璋冭瘯鐩殑鐨勭骇鍒俊鎭€備簨浠堕€氬父鏄剧ず鍦ㄨ繖閲岋細

/sys/鍐呮牳/璺熻釜/浜嬩欢/pci_controller

姣旂収銆?include/trace/events/pci_controller.h 鐢ㄤ簬浜嬩欢瀹氫箟銆?

## 鍙敤鐨勮窡韪偣


### PCIe_ltssm_state_transition


鐩戞帶 PCIe LTSSM 鐘舵€佽浆鎹紝鍖呮嫭鐘舵€佸拰閫熺巼淇℃伅
```

    pcie_ltssm_state_transition  "dev: %s state: %s rate: %s\n"

```
**鍙傛暟**锛?

- `dev` - PCIe 鎺у埗鍣ㄥ疄渚?
- `state` - PCIe LTSSM 鐘舵€?
- `rate` - PCIe 鏁版嵁閫熺巼

**鐢ㄦ硶绀轰緥**锛?


# 鍚敤璺熻釜鐐?
echo 1 > /sys/kernel/debug/tracing/events/pci_controller/pcie_ltssm_state_transition/enable

# 鐩戞帶浜嬩欢锛堣澶囬摼鎺ユ椂浼氱敓鎴愪互涓嬭緭鍑猴級
鐚?绯荤粺/鍐呮牳/璋冭瘯/璺熻釜/trace_pipe
kworker/0:0-9 [^000^] ..... 5.600221: pcie_ltssm_state_transition: dev: a40000000.pcie 鐘舵€? RCVRY_EQ2 閫熺巼: 8.0 GT/s
