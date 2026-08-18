
## 鐢ㄤ簬 AMD/Pensando(R) DSC 閫傞厤鍣ㄧ郴鍒楃殑 PCI VFIO 椹卞姩


AMD/Pensando Linux VFIO PCI 璁惧椹卞姩
Copyright(c) 2023 Advanced Micro Devices, Inc.

## 姒傝堪


`pds-vfio-pci` 妯″潡鏄竴涓?PCI 椹卞姩锛屾敮鎸?DSC 纭欢涓叿澶囧疄鏃惰縼绉伙紙Live Migration锛夎兘鍔涚殑铏氭嫙鍔熻兘锛圴F锛夎澶囥€?
## 浣跨敤璁惧


pds-vfio-pci 璁惧閫氳繃澶氫釜閰嶇疆姝ラ鍚敤锛屽苟渚濊禆 `pds_core` 椹卞姩鏉ュ垱寤哄拰鍚敤 SR-IOV 铏氭嫙鍔熻兘璁惧銆?
涓嬮潰灞曠ず浜嗗皢椹卞姩缁戝畾鍒颁竴涓?VF锛屼互鍙婄粦瀹氬埌鐢?`pds_core` 椹卞姩鍒涘缓鐨勫叧鑱旇緟鍔╄澶囩殑姝ラ銆傛绀轰緥鍋囪 pds_core 鍜?pds-vfio-pci 妯″潡宸茬粡鍔犺浇銆?
  :name: example-setup-script

  #!/bin/bash

  PF_BUS="0000:60"
  PF_BDF="0000:60:00.0"
  VF_BDF="0000:60:00.1"

  # 闃绘闈?vfio 鐨?VF 椹卞姩鎺㈡祴 VF 璁惧
  echo 0 > /sys/class/pci_bus/$PF_BUS/device/$PF_BDF/sriov_drivers_autoprobe

  # 閫氳繃 pds_core 鍒涘缓鍗曚釜鐢ㄤ簬瀹炴椂杩佺Щ鐨?VF
  echo 1 > /sys/bus/pci/drivers/pds_core/$PF_BDF/sriov_numvfs

  # 鍏佽灏?VF 缁戝畾鍒?pds-vfio-pci 椹卞姩
  echo "pds-vfio-pci" > /sys/class/pci_bus/$PF_BUS/device/$VF_BDF/driver_override

  # 灏?VF 缁戝畾鍒?pds-vfio-pci 椹卞姩
  echo "$VF_BDF" > /sys/bus/pci/drivers/pds-vfio-pci/bind

鎵ц涓婅堪姝ラ鍚庯紝搴斿綋鍦?/dev/vfio/<iommu_group> 涓垱寤轰簡涓€涓枃浠躲€?

## 鍚敤椹卞姩


璇ラ┍鍔ㄩ€氳繃鏍囧噯鐨勫唴鏍搁厤缃郴缁熷惎鐢紝
```

  make oldconfig/menuconfig/etc.

```
璇ラ┍鍔ㄥ湪鑿滃崟缁撴瀯涓殑浣嶇疆涓猴細

  -> 璁惧椹卞姩锛圖evice Drivers锛?    -> 闈炵壒鏉冪敤鎴风┖闂?VFIO 椹卞姩妗嗘灦锛圴FIO Non-Privileged userspace driver framework锛?      -> 鐢ㄤ簬 PDS PCI 璁惧鐨?VFIO 鏀寔锛圴FIO support for PDS PCI devices锛?
## 鏀寔


瀵逛簬涓€鑸€х殑 Linux 缃戠粶鏀寔锛岃浣跨敤 netdev 閭欢鍒楄〃
```

  netdev@vger.kernel.org

```
瀵逛簬鏇村叿浣撶殑鏀寔闇€姹傦紝璇蜂娇鐢?Pensando 椹卞姩鏀寔
```

  drivers@pensando.io

```
