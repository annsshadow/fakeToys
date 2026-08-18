
## 闈㈠悜 AMD/Pensando(R) DSC 閫傞厤鍣ㄧ郴鍒楃殑 PCI vDPA 椹卞姩


AMD/Pensando vDPA VF 璁惧椹卞姩

Copyright(c) 2023 Advanced Micro Devices, Inc

## 姒傝堪


`pds_vdpa` 椹卞姩鏄竴涓緟鍔╂€荤嚎锛坅uxiliary bus锛夐┍鍔紝鎻愪緵涓€涓緵 virtio 缃戠粶鍗忚鏍堜娇鐢ㄧ殑 vDPA 璁惧銆傚畠涓庢彁渚?vDPA 涓?virtio 闃熷垪鏈嶅姟鐨?Pensando 铏氭嫙鍔熻兘锛圴irtual Function锛夎澶囦竴璧蜂娇鐢ㄣ€傚畠渚濊禆 `pds_core` 椹卞姩涓庣‖浠舵潵澶勭悊 PF 涓?VF 鐨?PCI 浜嬪姟锛屼互鍙婅澶囬厤缃湇鍔°€?
## 浣跨敤璁惧


`pds_vdpa` 璁惧閫氳繃澶氫釜閰嶇疆姝ラ鍚敤锛屽苟渚濊禆 `pds_core` 椹卞姩鏉ュ垱寤哄苟鍚敤 SR-IOV 铏氭嫙鍔熻兘璁惧銆傚湪 VF 鍚敤鍚庯紝鎴戜滑鍦?`pds_core` 璁惧涓惎鐢?vDPA 鏈嶅姟锛屼互鍒涘缓渚?pds_vdpa 浣跨敤鐨勮緟鍔╄澶囥€?
绀轰緥姝ラ锛?

  #!/bin/bash

  modprobe pds_core
  modprobe vdpa
  modprobe pds_vdpa

  PF_BDF=`ls /sys/module/pds_core/drivers/pci\:pds_core/*/sriov_numvfs | awk -F / '{print $7}'`

  # 鍦?PF 涓惎鐢?vDPA VF 杈呭姪璁惧
  devlink dev param set pci/$PF_BDF name enable_vnet cmode runtime value true

  # 涓?vDPA 鍒涘缓涓€涓?VF
  echo 1 > /sys/bus/pci/drivers/pds_core/$PF_BDF/sriov_numvfs

  # 鏌ユ壘鍙敤鐨?vDPA 鏈嶅姟/璁惧
  PDS_VDPA_MGMT=`vdpa mgmtdev show | grep vDPA | head -1 | cut -d: -f1`

  # 鍒涘缓涓€涓敤浜?virtio 缃戠粶閰嶇疆鐨?vDPA 璁惧
  vdpa dev add name vdpa1 mgmtdev $PDS_VDPA_MGMT mac 00:11:22:33:44:55

  # 鍦ㄨ vdpa 璁惧涓婂缓绔嬩互澶綉鎺ュ彛
  modprobe virtio_vdpa



## 鍚敤椹卞姩


璇ラ┍鍔ㄩ€氳繃鏍囧噯鍐呮牳閰嶇疆绯荤粺鍚敤锛?```

  make oldconfig/menuconfig/etc.

```
璇ラ┍鍔ㄤ綅浜庤彍鍗曠粨鏋勪腑鐨勶細

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> Pensando devices
          -> Pensando Ethernet PDS_VDPA Support

## 鏀寔


瀵逛簬涓€鑸?Linux 缃戠粶鏀寔锛岃浣跨敤 netdev 閭欢鍒楄〃
```

  netdev@vger.kernel.org

```
瀵逛簬鏇村叿浣撶殑鏀寔闇€姹傦紝璇蜂娇鐢?Pensando 椹卞姩鏀寔
```

  drivers@pensando.io

```
