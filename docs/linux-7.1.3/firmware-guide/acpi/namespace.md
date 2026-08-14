
## ACPI 璁惧鏍?鈥斺€?ACPI 鍛藉悕绌洪棿鐨勮〃绀?

:Copyright: |copy| 2013, Intel Corporation

:Author: Lv Zheng <lv.zheng@intel.com>

:Credit:   Thanks for the help from Zhang Rui <rui.zhang@intel.com> and
           Rafael J.Wysocki <rafael.j.wysocki@intel.com>.

## 鎽樿

Linux ACPI 瀛愮郴缁熷皢 ACPI 鍛藉悕绌洪棿瀵硅薄杞崲涓?/sys/devices/LNXSYSTM:00 涓嬬殑 Linux 璁惧鏍戯紝
骞跺湪鎺ユ敹鍒?ACPI 鐑彃鎷旈€氱煡浜嬩欢鏃舵洿鏂板畠銆傚浜庢灞傛缁撴瀯涓殑姣忎釜璁惧瀵硅薄锛屽湪
/sys/bus/acpi/devices 涓兘鏈変竴涓搴旂殑绗﹀彿閾炬帴銆?
鏈枃妗ｈ鏄庝簡 ACPI 璁惧鏍戠殑缁撴瀯銆?
## ACPI 瀹氫箟鍧?

ACPI 鍥轰欢鍦ㄧ郴缁熷唴瀛樺湴鍧€绌洪棿涓缃?RSDP锛圧oot System Description Pointer锛屾牴绯荤粺鎻忚堪鎸囬拡锛夛紝
鎸囧悜 XSDT锛圗xtended System Description Table锛屾墿灞曠郴缁熸弿杩拌〃锛夈€俋SDT 鎬绘槸閫氳繃鍏剁涓€涓潯鐩?鎸囧悜 FADT锛團ixed ACPI Description Table锛屽浐瀹?ACPI 鎻忚堪琛級锛孎ADT 鍐呯殑鏁版嵁鍖呭惈鎻忚堪纭欢鍥哄畾
ACPI 鐗规€х殑鍚勭瀹氶暱鏉＄洰銆侳ADT 鍖呭惈涓€涓寚鍚?DSDT锛圖ifferentiated System Description Table锛?宸紓鍖栫郴缁熸弿杩拌〃锛夌殑鎸囬拡銆俋SDT 杩樺寘鍚寚鍚戝彲鑳藉涓?SSDT锛圫econdary System Description
Table锛岃緟鍔╃郴缁熸弿杩拌〃锛夌殑鏉＄洰銆?
DSDT 鍜?SSDT 鏁版嵁缁勭粐鍦ㄧО涓哄畾涔夊潡鐨勬暟鎹粨鏋勪腑锛岃繖浜涘潡鍖呭惈鍚勭瀵硅薄鐨勫畾涔夛紝鍖呮嫭浠?AML
锛圓CPI Machine Language锛孉CPI 鏈哄櫒璇█锛夌紪鐮佺殑 ACPI 鎺у埗鏂规硶銆侱SDT 鐨勬暟鎹潡杩炲悓 SSDT 鐨?鍐呭鍏卞悓琛ㄧず涓€涓О涓?ACPI 鍛藉悕绌洪棿鐨勫垎灞傛暟鎹粨鏋勶紝鍏舵嫇鎵戠粨鏋勫弽鏄犱簡搴曞眰纭欢骞冲彴鐨勭粨鏋勩€?
涓婅堪 ACPI 绯荤粺瀹氫箟琛ㄤ箣闂寸殑鍏崇郴

```

   +---------+    +-------+    +--------+    +------------------------+
   |  RSDP   | +->| XSDT  | +->|  FADT  |    |  +-------------------+ |
   +---------+ |  +-------+ |  +--------+  +-|->|       DSDT        | |
   | Pointer | |  | Entry |-+  | ...... |  | |  +-------------------+ |
   +---------+ |  +-------+    | X_DSDT |--+ |  | Definition Blocks | |
   | Pointer |-+  | ..... |    | ...... |    |  +-------------------+ |
   +---------+    +-------+    +--------+    |  +-------------------+ |
                  | Entry |------------------|->|       SSDT        | |
                  +- - - -+                  |  +-------------------| |
                  | Entry | - - - - - - - -+ |  | Definition Blocks | |
                  +- - - -+                | |  +-------------------+ |
                                           | |  +- - - - - - - - - -+ |
                                           +-|->|       SSDT        | |
                                             |  +-------------------+ |
                                             |  | Definition Blocks | |
                                             |  +- - - - - - - - - -+ |
                                             +------------------------+
                                                          |
                                             OSPM Loading |
                                                         \|/
                                                   +----------------+
                                                   | ACPI Namespace |
                                                   +----------------+

                  Figure 1. ACPI Definition Blocks

```
   Description Table锛夈€傚钩鍙版彁渚?RSDT 浠ュ疄鐜颁笌 ACPI 1.0 鎿嶄綔绯荤粺鐨勫吋瀹规€с€傚鏋滃瓨鍦紝
   鎿嶄綔绯荤粺搴斾娇鐢?XSDT銆?

## ACPI 鍛藉悕绌洪棿绀轰緥


鎵€鏈夊畾涔夊潡閮借鍔犺浇鍒板崟涓€鐨勫懡鍚嶇┖闂翠腑銆傚懡鍚嶇┖闂存槸涓€涓敱鍚嶇О鍜岃矾寰勬爣璇嗙殑瀵硅薄灞傛缁撴瀯銆?浠ヤ笅鍛藉悕绾﹀畾閫傜敤浜?ACPI 鍛藉悕绌洪棿涓殑瀵硅薄鍚嶇О锛?
   1. 鎵€鏈夊悕绉板潎涓?32 浣嶉暱銆?   2. 鍚嶇О鐨勭涓€涓瓧鑺傚繀椤绘槸 'A' - 'Z'銆?_' 涔嬩竴銆?   3. 鍚嶇О鐨勫叾浣欐瘡涓瓧鑺傚繀椤绘槸 'A' - 'Z'銆?0' - '9'銆?_' 涔嬩竴銆?   4. 浠?'_' 寮€澶寸殑鍚嶇О鐢?ACPI 瑙勮寖淇濈暀銆?   5. '\' 绗﹀彿琛ㄧず鍛藉悕绌洪棿鐨勬牴锛堝嵆浠?'\' 涓哄墠缂€鐨勫悕绉扮浉瀵逛簬鍛藉悕绌洪棿鏍癸級銆?   6. '^' 绗﹀彿琛ㄧず褰撳墠鍛藉悕绌洪棿鑺傜偣鐨勭埗鑺傜偣锛堝嵆浠?'^' 涓哄墠缂€鐨勫悕绉扮浉瀵逛簬褰撳墠鍛藉悕绌洪棿鑺傜偣鐨?      鐖惰妭鐐癸級銆?
```

   +------+
   | \    |                     Root
   +------+
     |
     | +------+
     +-| _PR  |                 Scope(_PR): the processor namespace
     | +------+
     |   |
     |   | +------+
     |   +-| CPU0 |             Processor(CPU0): the first processor
     |     +------+
     |
     | +------+
     +-| _SB  |                 Scope(_SB): the system bus namespace
     | +------+
     |   |
     |   | +------+
     |   +-| LID0 |             Device(LID0); the lid device
     |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _HID |         Name(_HID, "PNP0C0D"): the hardware ID
     |   |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _STA |         Method(_STA): the status control method
     |   |     +------+
     |   |
     |   | +------+
     |   +-| PCI0 |             Device(PCI0); the PCI root bridge
     |     +------+
     |       |
     |       | +------+
     |       +-| _HID |         Name(_HID, "PNP0A08"): the hardware ID
     |       | +------+
     |       |
     |       | +------+
     |       +-| _CID |         Name(_CID, "PNP0A03"): the compatible ID
     |       | +------+
     |       |
     |       | +------+
     |       +-| RP03 |         Scope(RP03): the PCI0 power scope
     |       | +------+
     |       |   |
     |       |   | +------+
     |       |   +-| PXP3 |     PowerResource(PXP3): the PCI0 power resource
     |       |     +------+
     |       |
     |       | +------+
     |       +-| GFX0 |         Device(GFX0): the graphics adapter
     |         +------+
     |           |
     |           | +------+
     |           +-| _ADR |     Name(_ADR, 0x00020000): the PCI bus address
     |           | +------+
     |           |
     |           | +------+
     |           +-| DD01 |     Device(DD01): the LCD output device
     |             +------+
     |               |
     |               | +------+
     |               +-| _BCL | Method(_BCL): the backlight control method
     |                 +------+
     |
     | +------+
     +-| _TZ  |                 Scope(_TZ): the thermal zone namespace
     | +------+
     |   |
     |   | +------+
     |   +-| FN00 |             PowerResource(FN00): the FAN0 power resource
     |   | +------+
     |   |
     |   | +------+
     |   +-| FAN0 |             Device(FAN0): the FAN0 cooling device
     |   | +------+
     |   |   |
     |   |   | +------+
     |   |   +-| _HID |         Name(_HID, "PNP0A0B"): the hardware ID
     |   |     +------+
     |   |
     |   | +------+
     |   +-| TZ00 |             ThermalZone(TZ00); the FAN thermal zone
     |     +------+
     |
     | +------+
     +-| _GPE |                 Scope(_GPE): the GPE namespace
       +------+

                     Figure 2. Example ACPI Namespace


```
## Linux ACPI 璁惧瀵硅薄


Linux 鍐呮牳鐨勬牳蹇?ACPI 瀛愮郴缁熶负琛ㄧず璁惧銆佺數婧愯祫婧愩€佸鐞嗗櫒銆佺儹鍖虹殑 ACPI 鍛藉悕绌洪棿瀵硅薄鍒涘缓
struct acpi_device 瀵硅薄銆傝繖浜涘璞￠€氳繃 sysfs 浣滀负 /sys/devices/LNXSYSTM:00 涓嬪瓙鏍戜腑鐨勭洰褰?瀵煎嚭鍒扮敤鎴风┖闂淬€傚畠浠悕绉扮殑鏍煎紡涓?<bus_id:instance>锛屽叾涓?'bus_id' 鎸囦唬缁欏畾瀵硅薄鐨?ACPI
鍛藉悕绌洪棿琛ㄧず锛?instance' 鐢ㄤ簬鍖哄垎鍏锋湁鐩稿悓 'bus_id' 鐨勪笉鍚屽璞★紙瀹冩槸鏃犵鍙锋暣鏁扮殑涓や綅
鍗佽繘鍒惰〃绀猴級銆?
'bus_id' 鐨勫€煎彇鍐充簬鍏跺悕绉版墍灞炲璞＄殑绫诲瀷

```

                +---+-----------------+-------+----------+
                |   | Object/Feature  | Table | bus_id   |
                +---+-----------------+-------+----------+
                | N | Root            | xSDT  | LNXSYSTM |
                +---+-----------------+-------+----------+
                | N | Device          | xSDT  | _HID     |
                +---+-----------------+-------+----------+
                | N | Processor       | xSDT  | LNXCPU   |
                +---+-----------------+-------+----------+
                | N | ThermalZone     | xSDT  | LNXTHERM |
                +---+-----------------+-------+----------+
                | N | PowerResource   | xSDT  | LNXPOWER |
                +---+-----------------+-------+----------+
                | N | Other Devices   | xSDT  | device   |
                +---+-----------------+-------+----------+
                | F | PWR_BUTTON      | FADT  | LNXPWRBN |
                +---+-----------------+-------+----------+
                | F | SLP_BUTTON      | FADT  | LNXSLPBN |
                +---+-----------------+-------+----------+
                | M | Video Extension | xSDT  | LNXVIDEO |
                +---+-----------------+-------+----------+
                | M | ATA Controller  | xSDT  | LNXIOBAY |
                +---+-----------------+-------+----------+
                | M | Docking Station | xSDT  | LNXDOCK  |
                +---+-----------------+-------+----------+

                 Table 1. ACPI Namespace Objects Mapping

```
鍦ㄥ熀浜?ACPI 绯荤粺鎻忚堪琛ㄧ殑鍐呭锛堝涓婃枃琛ㄦ牸绗竴鍒楃殑瀛楁瘝鍜岀浜屽垪鐨勮鍙锋墍绀猴級鍒涘缓
struct acpi_device 瀵硅薄鏃讹紝閫傜敤浠ヤ笅瑙勫垯锛?
   N:
      瀵硅薄鐨勬潵婧愭槸涓€涓?ACPI 鍛藉悕绌洪棿鑺傜偣锛堝绗簩鍒椾腑鍛藉悕瀵硅薄鐨勭被鍨嬫墍绀猴級銆傚湪杩欑鎯呭喌涓嬶紝
      璇ュ璞″湪 sysfs 涓殑鐩綍灏嗗寘鍚?'path' 灞炴€э紝鍏跺€间负浠庡懡鍚嶇┖闂存牴鍒拌鑺傜偣鐨勫畬鏁磋矾寰勩€?   F:
      涓哄浐瀹氱殑纭欢鐗规€у垱寤?struct acpi_device 瀵硅薄锛堝绗簩鍒椾腑鍥哄畾鐗规€ф爣蹇楃殑鍚嶇О鎵€绀猴級锛?      鍥犳鍏?sysfs 鐩綍涓嶄細鍖呭惈 'path' 灞炴€с€?   M:
      涓哄叿鏈夌壒瀹氭帶鍒舵柟娉曠殑 ACPI 鍛藉悕绌洪棿鑺傜偣鍒涘缓 struct acpi_device 瀵硅薄锛堝绗簩鍒椾腑 ACPI
      瀹氫箟鐨勮澶囩被鍨嬫墍绀猴級銆傚寘鍚叾鍛藉悕绌洪棿璺緞鐨?'path' 灞炴€у皢鍑虹幇鍦ㄥ叾 sysfs 鐩綍涓€備緥濡傦紝
      濡傛灉鏌愪釜 ACPI 鍛藉悕绌洪棿鑺傜偣瀛樺湪 _BCL 鏂规硶锛屽垯浼氫负鍏跺垱寤轰竴涓?'bus_id' 涓?LNXVIDEO 鐨?      struct acpi_device 瀵硅薄銆?
涓婅〃鐨勭涓夊垪鎸囩ず鍝簺 ACPI 绯荤粺鎻忚堪琛ㄥ寘鍚敤浜庡垱寤虹粰瀹氳鎵€琛ㄧず struct acpi_device 瀵硅薄鐨?淇℃伅锛坸SDT 琛ㄧず DSDT 鎴?SSDT锛夈€?
涓婅〃鐨勭鍥涘垪鎸囩ず struct acpi_device 瀵硅薄鐨?'bus_id' 鐢熸垚瑙勫垯锛?
   _HID:
      _HID 鍦ㄨ〃涓渶鍚庝竴鍒楁剰鍛崇潃瀵硅薄鐨?bus_id 娲剧敓鑷浉搴?ACPI 鍛藉悕绌洪棿鑺傜偣涓嬬殑 _HID/_CID
      鏍囪瘑瀵硅薄銆傝瀵硅薄鐨?sysfs 鐩綍灏嗛殢鍚庡寘鍚?'hid' 鍜?'modalias' 灞炴€э紝鍙敤浜庢绱㈣瀵硅薄鐨?      _HID 鍜?_CID銆?   LNXxxxxx:
      瀵逛簬 bus_id 涓?"LNXxxxxx" 褰㈠紡锛堜吉璁惧锛夌殑 struct acpi_device 瀵硅薄锛屼篃瀛樺湪 'modalias'
      灞炴€э紝鍦ㄨ繖绉嶆儏鍐典笅瀹冨寘鍚?bus_id 瀛楃涓叉湰韬€?   device:
      琛ㄤ腑鏈€鍚庝竴鍒楃殑 'device' 琛ㄧず璇ュ璞＄殑 bus_id 鏃犳硶浠庣浉搴?ACPI 鍛藉悕绌洪棿鑺傜偣鐨?_HID/_CID
      纭畾锛屽敖绠¤瀵硅薄琛ㄧず涓€涓澶囷紙渚嬪锛屽畠鍙兘鏄竴涓畾涔変簡 _ADR 浣嗘病鏈?_HID 鎴?_CID 鐨?      PCI 璁惧锛夈€傚湪杩欑鎯呭喌涓嬶紝瀛楃涓?'device' 灏嗙敤浣滆瀵硅薄鐨?bus_id銆?

## Linux ACPI 鐗╃悊璁惧绮樺悎


ACPI 璁惧锛堝嵆 struct acpi_device锛夊璞″彲浠ラ摼鎺ュ埌 Linux 璁惧灞傛缁撴瀯涓〃绀衡€滅墿鐞嗏€濊澶囩殑
鍏朵粬瀵硅薄锛堜緥濡?PCI 鎬荤嚎涓婄殑璁惧锛夈€傚鏋滃彂鐢熻繖绉嶆儏鍐碉紝鎰忓懗鐫€璇?ACPI 璁惧瀵硅薄鏄煇涓互鍏朵粬
鏂瑰紡琛ㄧず鐨勮澶囩殑鈥滀即鐢熷璞♀€濓紙companion锛夛紝骞剁敤浜庯紙1锛夋彁渚涙棤娉曢€氳繃鍏朵粬鏂瑰紡鑾峰緱鐨勫叧浜庤
璁惧鐨勯厤缃俊鎭紝浠ュ強锛?锛夊€熷姪鍏?ACPI 鎺у埗鏂规硶瀵硅璁惧鎵ц鐗瑰畾鎿嶄綔銆備竴涓?ACPI 璁惧瀵硅薄鍙互
浠ヨ繖绉嶆柟寮忛摼鎺ュ埌澶氫釜鈥滅墿鐞嗏€濊澶囥€?
濡傛灉鏌愪釜 ACPI 璁惧瀵硅薄閾炬帴鍒扳€滅墿鐞嗏€濊澶囷紝鍏?sysfs 鐩綍灏嗗寘鍚寚鍚戠洰鏍囪澶囧璞?sysfs 鐩綍鐨?"physical_node" 绗﹀彿閾炬帴銆傚弽杩囨潵锛岀洰鏍囪澶囩殑 sysfs 鐩綍灏嗗寘鍚寚鍚戜即鐢?ACPI 璁惧瀵硅薄 sysfs
鐩綍鐨?"firmware_node" 绗﹀彿閾炬帴銆傞摼鎺ユ満鍒朵緷璧?ACPI 鍛藉悕绌洪棿鎻愪緵鐨勮澶囨爣璇嗐€備緥濡傦紝濡傛灉瀛樺湪
涓€涓〃绀?PCI 璁惧鐨?ACPI 鍛藉悕绌洪棿瀵硅薄锛堝嵆琛ㄧず PCI 妗ョ殑 ACPI 鍛藉悕绌洪棿瀵硅薄涓嬬殑璁惧瀵硅薄锛夛紝
鍏?_ADR 杩斿洖 0x00020000锛屼笖鐖?PCI 妗ョ殑鎬荤嚎鍙蜂负 0锛屽垯琛ㄧず涓鸿 ACPI 鍛藉悕绌洪棿瀵硅薄鍒涘缓鐨?struct acpi_device 瀵硅薄鐨?sysfs 鐩綍灏嗗寘鍚寚鍚戠浉搴?PCI 璁惧鐨?/sys/devices/pci0000:00/0000:00:02:0/ sysfs 鐩綍鐨?'physical_node' 绗﹀彿閾炬帴銆?
閾炬帴鏈哄埗閫氬父鏄€荤嚎鐗瑰畾鐨勩€傚叾瀹炵幇鐨勬牳蹇冧綅浜?drivers/acpi/glue.c 鏂囦欢涓紝浣嗚繕鏈変綅浜庡叾浠?浣嶇疆鐨勩€佸彇鍐充簬鐩稿叧鎬荤嚎绫诲瀷鐨勮ˉ鍏呴儴鍒嗐€備緥濡傦紝鍏?PCI 鐗瑰畾閮ㄥ垎浣嶄簬 drivers/pci/pci-acpi.c
涓€?

## Linux ACPI 璁惧鏍戠ず渚?

涓庡浘 2 鎵€绀虹ず渚?ACPI 鍛藉悕绌洪棿瀵瑰簲鐨?struct acpi_device 瀵硅薄鐨?sysfs 灞傛缁撴瀯锛屽苟闄勫姞

```

   +--------------+---+-----------------+
   | LNXSYSTM:00  | \ | acpi:LNXSYSTM:  |
   +--------------+---+-----------------+
     |
     | +-------------+-----+----------------+
     +-| LNXPWRBN:00 | N/A | acpi:LNXPWRBN: |
     | +-------------+-----+----------------+
     |
     | +-------------+-----+----------------+
     +-| LNXSLPBN:00 | N/A | acpi:LNXSLPBN: |
     | +-------------+-----+----------------+
     |
     | +-----------+------------+--------------+
     +-| LNXCPU:00 | \_PR_.CPU0 | acpi:LNXCPU: |
     | +-----------+------------+--------------+
     |
     | +-------------+-------+----------------+
     +-| LNXSYBUS:00 | \_SB_ | acpi:LNXSYBUS: |
     | +-------------+-------+----------------+
     |   |
     |   | +- - - - - - - +- - - - - - +- - - - - - - -+
     |   +-| PNP0C0D:00 | \_SB_.LID0 | acpi:PNP0C0D: |
     |   | +- - - - - - - +- - - - - - +- - - - - - - -+
     |   |
     |   | +------------+------------+-----------------------+
     |   +-| PNP0A08:00 | \_SB_.PCI0 | acpi:PNP0A08:PNP0A03: |
     |     +------------+------------+-----------------------+
     |       |
     |       | +-----------+-----------------+-----+
     |       +-| device:00 | \_SB_.PCI0.RP03 | N/A |
     |       | +-----------+-----------------+-----+
     |       |   |
     |       |   | +-------------+----------------------+----------------+
     |       |   +-| LNXPOWER:00 | \_SB_.PCI0.RP03.PXP3 | acpi:LNXPOWER: |
     |       |     +-------------+----------------------+----------------+
     |       |
     |       | +-------------+-----------------+----------------+
     |       +-| LNXVIDEO:00 | \_SB_.PCI0.GFX0 | acpi:LNXVIDEO: |
     |         +-------------+-----------------+----------------+
     |           |
     |           | +-----------+-----------------+-----+
     |           +-| device:01 | \_SB_.PCI0.DD01 | N/A |
     |             +-----------+-----------------+-----+
     |
     | +-------------+-------+----------------+
     +-| LNXSYBUS:01 | \_TZ_ | acpi:LNXSYBUS: |
       +-------------+-------+----------------+
         |
         | +-------------+------------+----------------+
         +-| LNXPOWER:0a | \_TZ_.FN00 | acpi:LNXPOWER: |
         | +-------------+------------+----------------+
         |
         | +------------+------------+---------------+
         +-| PNP0C0B:00 | \_TZ_.FAN0 | acpi:PNP0C0B: |
         | +------------+------------+---------------+
         |
         | +-------------+------------+----------------+
         +-| LNXTHERM:00 | \_TZ_.TZ00 | acpi:LNXTHERM: |
           +-------------+------------+----------------+

                  Figure 3. Example Linux ACPI Device Tree

```

   1. 'object' 鏄璞″湪 sysfs 涓洰褰曠殑鍚嶇О銆?   2. 'path' 鏄浉搴?ACPI 鍛藉悕绌洪棿瀵硅薄鐨?ACPI 鍛藉悕绌洪棿璺緞锛岀敱璇ュ璞＄殑 'path' sysfs 灞炴€ц繑鍥炪€?   3. 'modalias' 鏄璞＄殑 'modalias' sysfs 灞炴€х殑鍊硷紙濡傛湰鏂囨。鍓嶆枃鎵€杩帮級銆?
   'modalias' attribute.
