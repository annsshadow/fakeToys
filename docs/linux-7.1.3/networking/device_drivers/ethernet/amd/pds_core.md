
## AMD/Pensando(R) DSC 閫傞厤鍣ㄧ郴鍒楃殑 Linux 椹卞姩


Copyright(c) 2023 Advanced Micro Devices, Inc

## 璇嗗埆閫傞厤鍣?


瑕佺‘瀹氱郴缁熶笂鏄惁瀹夎浜嗕竴涓垨澶氫釜 AMD/Pensando PCI Core 璁惧锛屽彲鎵ц

```
  # lspci -d 1dd8:100c
  b5:00.0 Processing accelerators: Pensando Systems Device 100c
  b6:00.0 Processing accelerators: Pensando Systems Device 100c

```

濡傛灉鍒楀嚭浜嗕笂杩拌澶囷紝鍒?`pds_core.ko` 椹卞姩搴旇兘鎵惧埌骞堕厤缃畠浠互渚涗娇鐢ㄣ€傚唴鏍告棩蹇椾腑搴旀湁濡備笅鏉＄洰

```
  $ dmesg | grep pds_core
  pds_core 0000:b5:00.0: 252.048 Gb/s available PCIe bandwidth (16.0 GT/s PCIe x16 link)
  pds_core 0000:b5:00.0: FW: 1.60.0-73
  pds_core 0000:b6:00.0: 252.048 Gb/s available PCIe bandwidth (16.0 GT/s PCIe x16 link)
  pds_core 0000:b6:00.0: FW: 1.60.0-73

```

```
  $ devlink dev info pci/0000:b5:00.0
  pci/0000:b5:00.0:
    driver pds_core
    serial_number FLM18420073
    versions:
        fixed:
          asic.id 0x0
          asic.rev 0x0
        running:
          fw 1.51.0-73
        stored:
          fw.goldfw 1.15.9-C-22
          fw.mainfwa 1.60.0-73
          fw.mainfwb 1.60.0-57

```

## Info versions


`pds_core` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `fw`
     - running
     - 璁惧涓婅繍琛岀殑鍥轰欢鐗堟湰
   - - `fw.goldfw`
     - stored
     - 瀛樺偍鍦?goldfw 妲戒綅涓殑鍥轰欢鐗堟湰
   - - `fw.mainfwa`
     - stored
     - 瀛樺偍鍦?mainfwa 妲戒綅涓殑鍥轰欢鐗堟湰
   - - `fw.mainfwb`
     - stored
     - 瀛樺偍鍦?mainfwb 妲戒綅涓殑鍥轰欢鐗堟湰
   - - `asic.id`
     - fixed
     - 璇ヨ澶囩殑 ASIC 绫诲瀷
   - - `asic.rev`
     - fixed
     - 璇ヨ澶?ASIC 鐨勪慨璁㈢増鏈?

## 鍙傛暟


`pds_core` 椹卞姩瀹炵幇浜嗕互涓嬮€氱敤鍙傛暟锛岀敤浜庢帶鍒朵綔涓?auxiliary_bus 璁惧鎻愪緵鐨勫姛鑳姐€?

   :widths: 5 5 8 82

   - - 鍚嶇О
     - 妯″紡
     - 绫诲瀷
     - 鎻忚堪
   - - `enable_vnet`
     - runtime
     - Boolean
     - 閫氳繃 auxiliary_bus 璁惧鍚敤 vDPA 鍔熻兘

## 鍥轰欢绠＄悊


`flash` 鍛戒护鍙互鏇存柊 DSC 鍥轰欢銆備笅杞界殑鍥轰欢灏嗕繚瀛樺埌鍥轰欢 bank 1 鎴?bank 2 涓殑浠绘剰涓€涓紙鍗冲綋鍓嶆湭浣跨敤鐨勯偅涓級锛?

```
  # devlink dev flash pci/0000:b5:00.0 \
            file pensando/dsc_fw_1.63.0-22.tar

```

## 鍋ュ悍鎶ュ憡


```
  # devlink health show pci/0000:2b:00.0 reporter fw
  pci/0000:2b:00.0:
    reporter fw
      state healthy error 0 recover 0
  # devlink health diagnose pci/0000:2b:00.0 reporter fw
   Status: healthy State: 1 Generation: 0 Recoveries: 0

```

## 鍚敤椹卞姩


璇ラ┍鍔ㄩ€氳繃鏍囧噯鍐呮牳閰嶇疆绯荤粺鍚敤锛?

```
  make oldconfig/menuconfig/etc.

```

璇ラ┍鍔ㄥ湪鑿滃崟缁撴瀯涓殑浣嶇疆涓猴細

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> AMD devices
          -> AMD/Pensando Ethernet PDS_CORE Support

## 鏀寔


鏈夊叧閫氱敤 Linux 缃戠粶鏀寔锛岃浣跨敤 netdev 閭欢鍒楄〃

```
  netdev@vger.kernel.org

```
