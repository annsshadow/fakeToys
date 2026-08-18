
## Dell AWCC WMI 鎺ュ彛椹卞姩锛坅lienware-wmi锛?

鏈枃妗ｆ弿杩?Alienware 涓?Dell G 绯诲垪鏈哄瀷涓婂疄鐜扮殑 AWCC WMI 鎺ュ彛锛坅lienware-wmi 椹卞姩锛夛紝浠嬬粛鍏堕€氳繃 Platform Profile API 鏆撮湶鐨勬暎鐑笌瓒呴鎺у埗鏂规硶锛屼互鍙婄敱绀惧尯閫嗗悜宸ョ▼寰楀埌鐨?AWCCMethodFunction 宸ヤ綔鏈哄埗銆?


## 绠€浠?


WMI 璁惧 WMAX 宸插湪璁稿 Alienware 涓?Dell G 绯诲垪鏈哄瀷涓婂疄鐜般€傚湪杩欎簺鏈哄瀷涓紝宸茶瘑鍒嚭涓ょ瀹炵幇銆傜涓€绉嶇敤浜庤緝鑰佺殑绯荤粺锛屽鐞?HDMI銆佷寒搴︺€丷GB銆佹斁澶у櫒涓庢繁搴︾潯鐪犳帶鍒躲€傜浜岀鐢ㄤ簬杈冩柊鐨勭郴缁燂紝涓昏澶勭悊鏁ｇ儹鎺у埗涓庤秴棰戙€?

鎴戜滑鎬€鐤戝悗鑰呰 Alienware Command Center锛圓WCC锛夌敤鏉ョ鐞嗗巶鍟嗛瀹氫箟鐨勬暎鐑厤缃紙thermal profile锛夈€俛lienware-wmi 椹卞姩閫氳繃 Platform Profile API 鏆撮湶 Thermal_Information 涓?Thermal_Control 鏂规硶锛屼互妯℃嫙 AWCC 鐨勮涓恒€?

杩欎釜杈冩柊鐨勬帴鍙ｅ悕涓?AWCCMethodFunction锛屾槸鍦?Dell 娌℃湁鎻愪緵浠讳綍瀹樻柟鏂囨。鐨勬儏鍐典笅閫氳繃閫嗗悜宸ョ▼寰楀埌鐨勩€傛垜浠皢灏藉姏鎻忚堪鍏跺凡琚彂鐜扮殑鍐呭湪宸ヤ綔鏈哄埗銆?

   浠ヤ笅鏂规硶鎻忚堪鍙兘涓嶅畬鏁达紝骞朵笖鏌愪簺鎿嶄綔鍦ㄤ笉鍚岃澶囦箣闂村瓨鍦ㄤ笉鍚屽疄鐜般€?

### WMI 鎺ュ彛鎻忚堪


WMI 鎺ュ彛鎻忚堪鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿浠庡祵鍏ョ殑浜岃繘鍒?MOF锛坆mof锛夋暟鎹腑瑙ｇ爜锛?

```
 [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("WMI Function"), guid("{A70591CE-A997-11DA-B012-B622A1EF5492}")]
 class AWCCWmiMethodFunction {
   [key, read] string InstanceName;
   [read] boolean Active;

   [WmiMethodId(13), Implemented, read, write, Description("Return Overclocking Report.")] void Return_OverclockingReport([out] uint32 argr);
   [WmiMethodId(14), Implemented, read, write, Description("Set OCUIBIOS Control.")] void Set_OCUIBIOSControl([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(15), Implemented, read, write, Description("Clear OC FailSafe Flag.")] void Clear_OCFailSafeFlag([out] uint32 argr);
   [WmiMethodId(19), Implemented, read, write, Description("Get Fan Sensors.")] void GetFanSensors([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(20), Implemented, read, write, Description("Thermal Information.")] void Thermal_Information([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(21), Implemented, read, write, Description("Thermal Control.")] void Thermal_Control([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(23), Implemented, read, write, Description("MemoryOCControl.")] void MemoryOCControl([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(26), Implemented, read, write, Description("System Information.")] void SystemInformation([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(28), Implemented, read, write, Description("Power Information.")] void PowerInformation([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(32), Implemented, read, write, Description("FW Update GPIO toggle.")] void FWUpdateGPIOtoggle([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(33), Implemented, read, write, Description("Read Total of GPIOs.")] void ReadTotalofGPIOs([out] uint32 argr);
   [WmiMethodId(34), Implemented, read, write, Description("Read GPIO pin Status.")] void ReadGPIOpPinStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(35), Implemented, read, write, Description("Read Chassis Color.")] void ReadChassisColor([out] uint32 argr);
   [WmiMethodId(36), Implemented, read, write, Description("Read Platform Properties.")] void ReadPlatformProperties([out] uint32 argr);
   [WmiMethodId(37), Implemented, read, write, Description("Game Shift Status.")] void GameShiftStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(128), Implemented, read, write, Description("Caldera SW installation.")] void CalderaSWInstallation([out] uint32 argr);
   [WmiMethodId(129), Implemented, read, write, Description("Caldera SW is released.")] void CalderaSWReleased([out] uint32 argr);
   [WmiMethodId(130), Implemented, read, write, Description("Caldera Connection Status.")] void CalderaConnectionStatus([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(131), Implemented, read, write, Description("Surprise Unplugged Flag Status.")] void SurpriseUnpluggedFlagStatus([out] uint32 argr);
   [WmiMethodId(132), Implemented, read, write, Description("Clear Surprise Unplugged Flag.")] void ClearSurpriseUnpluggedFlag([out] uint32 argr);
   [WmiMethodId(133), Implemented, read, write, Description("Cancel Undock Request.")] void CancelUndockRequest([out] uint32 argr);
   [WmiMethodId(135), Implemented, read, write, Description("Devices in Caldera.")] void DevicesInCaldera([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(136), Implemented, read, write, Description("Notify BIOS for SW ready to disconnect Caldera.")] void NotifyBIOSForSWReadyToDisconnectCaldera([out] uint32 argr);
   [WmiMethodId(160), Implemented, read, write, Description("Tobii SW installation.")] void TobiiSWinstallation([out] uint32 argr);
   [WmiMethodId(161), Implemented, read, write, Description("Tobii SW Released.")] void TobiiSWReleased([out] uint32 argr);
   [WmiMethodId(162), Implemented, read, write, Description("Tobii Camera Power Reset.")] void TobiiCameraPowerReset([out] uint32 argr);
   [WmiMethodId(163), Implemented, read, write, Description("Tobii Camera Power On.")] void TobiiCameraPowerOn([out] uint32 argr);
   [WmiMethodId(164), Implemented, read, write, Description("Tobii Camera Power Off.")] void TobiiCameraPowerOff([out] uint32 argr);
 };
```

鏈枃妗ｄ腑鏈弿杩扮殑鏂规硶琛屼负鏈煡銆?

### 鍙傛暟缁撴瀯


鎵€鏈夎緭鍏ュ弬鏁扮殑绫诲瀷鍧囦负 **uint32**锛屽苟涓斿畠浠湪鍚勬柟娉曚箣闂寸殑缁撴瀯闈炲父鐩镐技銆傞€氬父锛岀涓€涓瓧鑺傚搴斾簬鏂规硶鎵ц鐨勭壒瀹?*鎿嶄綔**锛岄殢鍚庣殑瀛楄妭瀵瑰簲浜庝紶缁欒**鎿嶄綔**鐨?*鍙傛暟**銆備緥濡傦紝濡傛灉鏌愪釜鎿嶄綔鐮佷负 0x01 涓旈渶瑕佷竴涓?ID 0xA0锛屽垯浣犱紶缁欒鏂规硶鐨勫弬鏁颁负 0xA001銆?

## 鏁ｇ儹鏂规硶


### WMI 鏂规硶 GetFanSensors([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 鑾峰彇涓庢煇涓鎵?ID 鐩稿叧鐨勬俯搴︿紶鎰?  | - Byte 1: Fan ID   |
|                    | 鍣ㄦ暟閲?                            |                    |
+--------------------+------------------------------------+--------------------+
| 0x02               | 鑾峰彇涓庢煇涓鎵囦紶鎰熷櫒 ID 鐩稿叧鐨勬俯   | - Byte 1: Fan ID   |
|                    | 搴︿紶鎰熷櫒 ID                        | - Byte 2: Index    |
+--------------------+------------------------------------+--------------------+

### WMI 鏂规硶 Thermal_Information([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 鏈煡銆?                            | - None             |
+--------------------+------------------------------------+--------------------+
| 0x02               | 鑾峰彇鍏锋湁浠ヤ笅缁撴瀯鐨勭郴缁熸弿杩扮紪鍙凤細   | - None             |
|                    |                                    |                    |
|                    | - Byte 0: 椋庢墖鏁伴噺                 |                    |
|                    | - Byte 1: 娓╁害浼犳劅鍣ㄦ暟閲?          |                    |
|                    | - Byte 2: 鏈煡                     |                    |
|                    | - Byte 3: 鏁ｇ儹閰嶇疆锛坧rofile锛夋暟閲? |                    |
+--------------------+------------------------------------+--------------------+
| 0x03               | 鍦ㄧ粰瀹氱储寮曞鍒楀嚭涓€涓?ID 鎴栬祫婧愩€?  | - Byte 1: Index    |
|                    | 椋庢墖 ID銆佹俯搴?ID銆佹湭鐭?ID 涓庢暎鐑?  |                    |
|                    | 閰嶇疆 ID 鎸夎纭垏椤哄簭鍒楀嚭銆?        |                    |
|                    |                                    |                    |
|                    | 鎿嶄綔 0x02 鐢ㄤ簬浜嗚В鍝簺绱㈠紩鏄犲皠鍒?  |                    |
|                    | 鍝簺璧勬簮銆?                        |                    |
|                    |                                    |                    |
|                    | **杩斿洖锛?* 缁欏畾绱㈠紩澶勭殑 ID         |                    |
+--------------------+------------------------------------+--------------------+
| 0x04               | 鑾峰彇缁欏畾娓╁害浼犳劅鍣ㄧ殑褰撳墠娓╁害銆?    | - Byte 1: Sensor   |
|                    |                                    |   ID               |
+--------------------+------------------------------------+--------------------+
| 0x05               | 鑾峰彇缁欏畾椋庢墖鐨勫綋鍓?RPM銆?          | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x06               | 鑾峰彇椋庢墖杞€熺櫨鍒嗘瘮銆傦紙骞堕潪姣忎釜鍨嬪彿 | - Byte 1: Fan ID   |
|                    | 閮藉疄鐜帮級                           |                    |
+--------------------+------------------------------------+--------------------+
| 0x07               | 鏈煡銆?                            | - Unknown          |
+--------------------+------------------------------------+--------------------+
| 0x08               | 鑾峰彇缁欏畾椋庢墖 ID 鐨勬渶灏?RPM銆?      | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x09               | 鑾峰彇缁欏畾椋庢墖 ID 鐨勬渶澶?RPM銆?      | - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+
| 0x0A               | 鑾峰彇鍧囪　鏁ｇ儹閰嶇疆 ID銆?             | - None             |
+--------------------+------------------------------------+--------------------+
| 0x0B               | 鑾峰彇褰撳墠鏁ｇ儹閰嶇疆 ID銆?             | - None             |
+--------------------+------------------------------------+--------------------+
| 0x0C               | 鑾峰彇缁欏畾椋庢墖 ID 鐨勫綋鍓?`boost` 鍊笺€倈 - Byte 1: Fan ID   |
|                    |                                    |                    |
+--------------------+------------------------------------+--------------------+

### WMI 鏂规硶 Thermal_Control([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 婵€娲荤粰瀹氱殑鏁ｇ儹閰嶇疆銆?              | - Byte 1: Thermal  |
|                    |                                    |   profile ID       |
+--------------------+------------------------------------+--------------------+
| 0x02               | 涓虹粰瀹氶鎵?ID 璁剧疆 `boost` 鍊笺€?   | - Byte 1: Fan ID   |
|                    |                                    | - Byte 2: Boost    |
+--------------------+------------------------------------+--------------------+

宸茬煡鐨勬暎鐑厤缃唬鐮佸涓嬶細

+------------------------------+----------+------+
| Thermal Profile              | Type     | ID   |
+==============================+==========+======+
| Custom                       | Special  | 0x00 |
+------------------------------+----------+------+
| G-Mode                       | Special  | 0xAB |
+------------------------------+----------+------+
| Quiet                        | Legacy   | 0x96 |
+------------------------------+----------+------+
| Balanced                     | Legacy   | 0x97 |
+------------------------------+----------+------+
| Balanced Performance         | Legacy   | 0x98 |
+------------------------------+----------+------+
| Performance                  | Legacy   | 0x99 |
+------------------------------+----------+------+
| Balanced                     | USTT     | 0xA0 |
+------------------------------+----------+------+
| Balanced Performance         | USTT     | 0xA1 |
+------------------------------+----------+------+
| Cool                         | USTT     | 0xA2 |
+------------------------------+----------+------+
| Quiet                        | USTT     | 0xA3 |
+------------------------------+----------+------+
| Performance                  | USTT     | 0xA4 |
+------------------------------+----------+------+
| Low Power                    | USTT     | 0xA5 |
+------------------------------+----------+------+

濡傛灉鏌愬瀷鍙锋敮鎸?User Selectable Thermal Tables锛圲STT锛岀敤鎴峰彲閫夋暎鐑〃锛夐厤缃紝瀹冨皢涓嶆敮鎸?Legacy 閰嶇疆锛屽弽涔嬩害鐒躲€?

姣忎釜鍨嬪彿閮芥敮鎸?CUSTOM锛?x00锛夋暎鐑厤缃€傚湪 G 绯诲垪绗旇鏈腑锛孏MODE 鍙栦唬 PERFORMANCE銆?

### WMI 鏂规硶 GameShiftStatus([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| 0x01               | 鍒囨崲 **Game Shift**銆?             | - None             |
+--------------------+------------------------------------+--------------------+
| 0x02               | 鑾峰彇 **Game Shift** 鐘舵€併€?        | - None             |
+--------------------+------------------------------------+--------------------+

Game Shift 鐘舵€佷笉浼氭敼鍙橀鎵囬€熷害閰嶇疆锛屼絾瀹冨彲鑳芥槸鏌愮 CPU/GPU 鐢垫簮閰嶇疆銆傚皻鏈繘琛岃繃鍩哄噯娴嬭瘯銆?

璇ユ柟娉曚粎瀛樺湪浜?Dell 鐨?G 绯诲垪绗旇鏈腑锛屽叾瀹炵幇鎰忓懗鐫€ GMODE 鏁ｇ儹閰嶇疆鍙敤锛屽嵆渚?Thermal_Information 鐨勬搷浣?0x03 骞舵湭鍒楀嚭瀹冦€?

Dell G 绯诲垪绗旇鏈笂鐨?G 閿篃浼氭敼鍙?Game Shift 鐘舵€侊紝鍥犳浜岃€呯洿鎺ョ浉鍏炽€?

## 瓒呴鏂规硶


### WMI 鏂规硶 MemoryOCControl([in] uint32 arg2, [out] uint32 argr)


AWCC 鏀寔鍐呭瓨瓒呴锛屼絾璇ユ柟娉曢潪甯稿鏉傦紝灏氭湭琚牬璇戙€?

## GPIO 鎺у埗鏂规硶


甯︽湁 AWCC 鎺ュ彛鐨?Alienware 涓?Dell G 绯诲垪璁惧閫氬父鏈変竴涓祵鍏ョ殑 STM32 RGB 鐏厜鎺у埗鍣紝鍏峰 USB/HID 鑳藉姏銆傚叾鍘傚晢 ID 涓?`187c`锛岃€屼骇鍝?ID 鍙兘鍥犲瀷鍙疯€屽紓銆?

璇?MCU 鐨勪袱涓?GPIO 寮曡剼鐨勬帶鍒惰浣滀负 WMI 鏂规硶鏆撮湶鍑烘潵锛岀敤浜庤皟璇曠洰鐨勩€?

+--------------+--------------------------------------------------------------+
| Pin          | Description                                                  |
+==============+===============================+==============================+
| 0            | 璁惧鍥轰欢鏇存柊锛圖FU锛夋ā寮忓紩鑴氥€?| **HIGH**锛氫笅娆?MCU 鍚姩鏃跺惎鐢?DFU 妯″紡銆?|
|              |                               +------------------------------+
|              |                               | **LOW**锛氫笅娆?MCU 鍚姩鏃剁鐢?DFU 妯″紡銆? |
+--------------+-------------------------------+------------------------------+
| 1            | 璐熷浣嶏紙NRST锛夊紩鑴氥€?         | **HIGH**锛歁CU 寮€鍚€?        |
|              |                               |                              |
|              |                               +------------------------------+
|              |                               | **LOW**锛歁CU 鍏抽棴銆?         |
|              |                               |                              |
+--------------+-------------------------------+------------------------------+

鍏充簬璇?MCU 鐨勬洿澶氫俊鎭鍙傝鑷磋阿閮ㄥ垎銆?

   鏌愪簺 GPIO 鎺у埗鏂规硶鎵撶牬浜嗛€氬父鐨勫弬鏁扮粨鏋勶紝鍦ㄧ涓€涓瓧鑺備笂浣跨敤**寮曡剼鍙?*鑰岄潪鎿嶄綔鐮併€?

### WMI 鏂规硶 FWUpdateGPIOtoggle([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| Pin number         | 璁剧疆寮曡剼鐘舵€?                      | - Byte 1: Pin      |
|                    |                                    |   status           |
+--------------------+------------------------------------+--------------------+

### WMI 鏂规硶 ReadTotalofGPIOs([out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| N/A                | 鑾峰彇 GPIO 鐨勬€绘暟                   | - None             |
+--------------------+------------------------------------+--------------------+

   鐢变簬 WMI 鏂规硶鍦ㄥ浐浠跺眰闈㈢殑瀹炵幇鏂瑰紡锛岃鏂规硶鍦ㄨ璋冪敤鏃堕渶瑕佷竴涓搼锛坉ummy锛塽int32 杈撳叆鍙傛暟銆?

### WMI 鏂规硶 ReadGPIOpPinStatus([in] uint32 arg2, [out] uint32 argr)


+--------------------+------------------------------------+--------------------+
| Operation (Byte 0) | Description                        | Arguments          |
+====================+====================================+====================+
| Pin number         | 鑾峰彇寮曡剼鐘舵€?                      | - None             |
+--------------------+------------------------------------+--------------------+

   鍦ㄦ煇浜涚瑪璁版湰涓瓨鍦ㄥ凡鐭ョ殑鍥轰欢缂洪櫡锛岃鍙栨煇涓紩鑴氱殑鐘舵€佸悓鏃朵細缈昏浆瀹冦€?

## 鍏跺畠淇℃伅鏂规硶


### WMI 鏂规硶 ReadChassisColor([out] uint32 argr)


杩斿洖鏈虹棰滆壊鐨勫唴閮?ID銆?

## 鑷磋阿


鎰熻阿

- `AlexIII <https://github.com/AlexIII/tcc-g15>`_
- `T-Troll <https://github.com/T-Troll/alienfx-tools/>`_
- `Gabriel Marcano <https://gabriel.marcanobrady.family/blog/2024/12/16/dell-g5-5505-se-acpi-or-figuring-out-how-to-reset-the-rgb-controller/>`_

璁板綍骞舵祴璇曚簡璇ヨ澶囩殑閮ㄥ垎鍔熻兘锛屼娇寰楁湰椹卞姩寰椾互娉涘寲銆?
