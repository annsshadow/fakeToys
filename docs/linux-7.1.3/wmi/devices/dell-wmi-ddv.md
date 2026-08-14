
## Dell DDV WMI 鎺ュ彛椹卞姩锛坉ell-wmi-ddv锛?
## 绠€浠嬶紙Introduction锛?
2020 骞村墠鍚庣敓浜х殑璁稿 Dell 绗旇鏈敮鎸佷竴涓熀浜?WMI 鐨勬帴鍙ｏ紝鐢ㄤ簬鑾峰彇鍚勭绯荤粺鏁版嵁锛屼緥濡傜數姹?娓╁害銆乪PPID銆佽瘖鏂暟鎹互鍙婇鎵?娓╁害浼犳劅鍣ㄦ暟鎹€?
璇ユ帴鍙ｅ緢鍙兘琚?Windows 涓婄殑 `Dell Data Vault` 杞欢鎵€浣跨敤锛屽洜姝よ绉颁负 `DDV`銆傜洰鍓?`dell-wmi-ddv`
椹卞姩鏀寔璇ユ帴鍙ｇ殑绗?2 鐗堝拰绗?3 鐗堬紝鏂版帴鍙ｇ増鏈殑娣诲姞涔熷緢鏂逛究銆?
             鏂囨。鏄彲鐢ㄧ殑銆傚洜姝ゆ墍鏈夌煡璇嗛兘鏉ヨ嚜璇曢敊锛坱rial-and-error锛夛紝璇风墷璁拌繖涓€鐐广€?
## Dell ePPID锛堢數瀛愰儴浠舵爣璇嗭紝electronic Piece Part Identification锛?
Dell ePPID 鐢ㄤ簬鍞竴鏍囪瘑 Dell 鏈哄櫒涓殑缁勪欢锛屽寘鎷數姹犮€傚叾褰㈠紡绫讳技浜?`CC-PPPPPP-MMMMM-YMD-SSSS-FFF`锛屽苟鍖呭惈浠ヤ笅淇℃伅锛?
- 鍘熶骇鍥戒唬鐮侊紙CC锛夈€?- 閮ㄤ欢鍙凤紝棣栧瓧绗︿负濉厖鏁板瓧锛圥PPPPP锛夈€?- 鍒堕€犲晢鏍囪瘑锛圡MMMM锛夈€?- 鍒堕€犲勾/鏈?鏃ワ紙YMD锛夛紝閲囩敤 36 杩涘埗锛屽叾涓?Y 涓哄勾浠界殑鏈€鍚庝竴浣嶆暟瀛椼€?- 鍒堕€犲簭鍒楀彿锛圫SSS锛夈€?- 鍙€夊浐浠剁増鏈?淇鍙凤紙FFF锛夈€?
鍙互浣跨敤 `eppidtool <https://pypi.org/project/eppidtool>`_ python 宸ュ叿鏉ヨВ鐮佸苟鏄剧ず杩欎簺淇℃伅銆?
鍏充簬 Dell ePPID 鐨勬墍鏈変俊鎭兘鏉ヨ嚜 Dell 鏀寔鏂囨。浠ュ強
`杩欎釜缃戠珯 <https://telcontar.net/KBK/Dell/date_codes>`_銆?
## WMI 鎺ュ彛鎻忚堪锛圵MI interface description锛?
WMI 鎺ュ彛鎻忚堪鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿浠庡唴宓岀殑浜岃繘鍒?MOF锛坆mof锛?鏁版嵁涓В鐮佸嚭鏉ワ細

```

 [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("WMI Function"), guid("{8A42EA14-4F2A-FD45-6422-0087F7A7E608}")]
 class DDVWmiMethodFunction {
   [key, read] string InstanceName;
   [read] boolean Active;

   [WmiMethodId(1), Implemented, read, write, Description("Return Battery Design Capacity.")] void BatteryDesignCapacity([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(2), Implemented, read, write, Description("Return Battery Full Charge Capacity.")] void BatteryFullChargeCapacity([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(3), Implemented, read, write, Description("Return Battery Manufacture Name.")] void BatteryManufactureName([in] uint32 arg2, [out] string argr);
   [WmiMethodId(4), Implemented, read, write, Description("Return Battery Manufacture Date.")] void BatteryManufactureDate([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(5), Implemented, read, write, Description("Return Battery Serial Number.")] void BatterySerialNumber([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(6), Implemented, read, write, Description("Return Battery Chemistry Value.")] void BatteryChemistryValue([in] uint32 arg2, [out] string argr);
   [WmiMethodId(7), Implemented, read, write, Description("Return Battery Temperature.")] void BatteryTemperature([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(8), Implemented, read, write, Description("Return Battery Current.")] void BatteryCurrent([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(9), Implemented, read, write, Description("Return Battery Voltage.")] void BatteryVoltage([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(10), Implemented, read, write, Description("Return Battery Manufacture Access(MA code).")] void BatteryManufactureAceess([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(11), Implemented, read, write, Description("Return Battery Relative State-Of-Charge.")] void BatteryRelativeStateOfCharge([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(12), Implemented, read, write, Description("Return Battery Cycle Count")] void BatteryCycleCount([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(13), Implemented, read, write, Description("Return Battery ePPID")] void BatteryePPID([in] uint32 arg2, [out] string argr);
   [WmiMethodId(14), Implemented, read, write, Description("Return Battery Raw Analytics Start")] void BatteryeRawAnalyticsStart([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(15), Implemented, read, write, Description("Return Battery Raw Analytics")] void BatteryeRawAnalytics([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(16), Implemented, read, write, Description("Return Battery Design Voltage.")] void BatteryDesignVoltage([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(17), Implemented, read, write, Description("Return Battery Raw Analytics A Block")] void BatteryeRawAnalyticsABlock([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(18), Implemented, read, write, Description("Return Version.")] void ReturnVersion([in] uint32 arg2, [out] uint32 argr);
   [WmiMethodId(32), Implemented, read, write, Description("Return Fan Sensor Information")] void FanSensorInformation([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
   [WmiMethodId(34), Implemented, read, write, Description("Return Thermal Sensor Information")] void ThermalSensorInformation([in] uint32 arg2, [out] uint32 RawSize, [out, WmiSizeIs("RawSize") : ToInstance] uint8 RawData[]);
 };

```
姣忎釜 WMI 鏂规硶閮芥帴鍙椾竴涓寘鍚?32 浣嶇储寮曠殑 ACPI 缂撳啿鍖轰綔涓鸿緭鍏ュ弬鏁帮紝鍏朵腑鏈€浣?8 浣嶇敤浜庡湪浣跨敤
鐢垫睜鐩稿叧 WMI 鏂规硶鏃舵寚瀹氱數姹犮€傚叾瀹?WMI 鏂规硶鍙兘浼氬拷鐣ヨ鍙傛暟锛屾垨浠ヤ笉鍚屾柟寮忚В閲婂畠銆俉MI 鏂规硶鐨?杈撳嚭鏍煎紡鍚勪笉鐩稿悓锛?
- 濡傛灉鍑芥暟鍙湁涓€涓緭鍑猴紝鍒欒繑鍥炵浉搴旂被鍨嬬殑 ACPI 瀵硅薄
- 濡傛灉鍑芥暟鏈夊涓緭鍑猴紝鍒欒繑鍥炲寘鍚寜鐩稿悓椤哄簭鎺掑竷鐨勫悇杈撳嚭鐨?ACPI package

搴斿綋褰诲簳妫€鏌ヨ緭鍑虹殑鏍煎紡锛屽洜涓哄湪鍑洪敊鏃惰澶氭柟娉曚細杩斿洖鏍煎紡涓嶆纭殑鏁版嵁銆?
璁稿鐢垫睜鐩稿叧鏂规硶鐨勬暟鎹牸寮忎技涔庡熀浜?`Smart Battery Data Specification`锛堟櫤鑳界數姹犳暟鎹鑼冿級锛?鍥犳鏈煡鐨勭數姹犵浉鍏虫柟娉曞緢鍙兘浠ユ煇绉嶆柟寮忛伒寰鏍囧噯銆?
### WMI 鏂规硶 GetBatteryDesignCapacity()

杩斿洖鐢垫睜鐨勮璁″閲忥紙鍗曚綅 mAh锛夛紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryFullCharge()

杩斿洖鐢垫睜鐨勫畬鏁村厖鐢靛閲忥紙鍗曚綅 mAh锛夛紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryManufactureName()

杩斿洖鐢垫睜鐨勫埗閫犲晢鍚嶇О锛岀被鍨嬩负 ASCII 瀛楃涓层€?
### WMI 鏂规硶 BatteryManufactureDate()

杩斿洖鐢垫睜鐨勫埗閫犳棩鏈燂紝绫诲瀷涓?u16銆?鏃ユ湡鎸変互涓嬫柟寮忕紪鐮侊細

- 绗?0 鍒?4 浣嶅寘鍚埗閫犳棩銆?- 绗?5 鍒?8 浣嶅寘鍚埗閫犳湀銆?- 绗?9 鍒?15 浣嶅寘鍚浉瀵?1980 骞寸殑鍒堕€犲勾銆?
### WMI 鏂规硶 BatterySerialNumber()

杩斿洖鐢垫睜鐨勫簭鍒楀彿锛岀被鍨嬩负 u16銆?
### WMI 鏂规硶 BatteryChemistryValue()

杩斿洖鐢垫睜鐨勫寲瀛︽垚鍒嗭紝绫诲瀷涓?ASCII 瀛楃涓层€?宸茬煡鐨勫€间负锛?
- "Li-I" 琛ㄧず Li-Ion锛堥攤绂诲瓙锛?
### WMI 鏂规硶 BatteryTemperature()

杩斿洖鐢垫睜鐨勬俯搴︼紙鍗曚綅锛氬崄鍒嗕箣涓€寮€灏旀枃锛夛紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryCurrent()

杩斿洖鐢垫睜鐨勭數娴侊紙鍗曚綅 mA锛夛紝绫诲瀷涓?s16銆?璐熷€艰〃绀烘鍦ㄦ斁鐢点€?
### WMI 鏂规硶 BatteryVoltage()

杩斿洖鐢垫睜鐨勭數鍘嬶紙鍗曚綅 mV锛夛紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryManufactureAccess()

杩斿洖鐢垫睜鐨勫仴搴风姸鎬侊紝绫诲瀷涓?u16銆?鍋ュ悍鐘舵€佹寜浠ヤ笅鏂瑰紡缂栫爜锛?
 - 绗笁涓崐瀛楄妭锛坣ibble锛夊寘鍚竴鑸晠闅滄ā寮? - 绗洓涓崐瀛楄妭鍖呭惈鍏蜂綋鏁呴殰浠ｇ爜

鏈夋晥鐨勬晠闅滄ā寮忔湁锛?
 - 姘镐箙鏁呴殰锛坄0x9`锛? - 杩囩儹鏁呴殰锛坄0xa`锛? - 杩囨祦鏁呴殰锛坄0xb`锛?
鎵€鏈夊叾瀹冩晠闅滄ā寮忛兘搴旇涓烘甯搞€?
浠ヤ笅鏁呴殰浠ｇ爜瀵规案涔呮晠闅滄湁鏁堬細

 - 淇濋櫓涓濈啍鏂紙`0x0`锛? - 鐢佃姱澶辫　锛坄0x1`锛? - 杩囧帇锛坄0x2`锛? - FET 鏁呴殰锛坄0x3`锛?
褰撶數姹犳姤鍑烘案涔呮晠闅滄椂锛屾晠闅滀唬鐮佺殑鏈€鍚庝袱浣嶅簲蹇界暐銆?
浠ヤ笅鏁呴殰浠ｇ爜瀵硅繃鐑晠闅滄湁鏁堬細

 - 鍏呯數寮€濮嬫椂杩囩儹锛坄0x5`锛? - 鍏呯數鏈熼棿杩囩儹锛坄0x7`锛? - 鏀剧數鏈熼棿杩囩儹锛坄0x8`锛?
浠ヤ笅鏁呴殰浠ｇ爜瀵硅繃娴佹晠闅滄湁鏁堬細

 - 鍏呯數鏈熼棿杩囨祦锛坄0x6`锛? - 鏀剧數鏈熼棿杩囨祦锛坄0xb`锛?
### WMI 鏂规硶 BatteryRelativeStateOfCharge()

杩斿洖鐢垫睜鐨勫閲忕櫨鍒嗘瘮锛岀被鍨嬩负 u16銆?
### WMI 鏂规硶 BatteryCycleCount()

杩斿洖鐢垫睜鐨勫惊鐜鏁帮紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryePPID()

杩斿洖鐢垫睜鐨?ePPID锛岀被鍨嬩负 ASCII 瀛楃涓层€?
### WMI 鏂规硶 BatteryeRawAnalyticsStart()

瀵圭數姹犳墽琛屼竴娆″垎鏋愬苟杩斿洖鐘舵€佺爜锛?
- `0x0`锛氭垚鍔?- `0x1`锛氭帴鍙ｄ笉鏀寔
- `0xfffffffe`锛氶敊璇?瓒呮椂

   璇ユ柟娉曠殑鍚箟鍦ㄥ緢澶х▼搴︿笂浠嶆湭鐭ャ€?
### WMI 鏂规硶 BatteryeRawAnalytics()

杩斿洖涓€涓€氬父鍖呭惈 12 涓垎鏋愭暟鎹潡鐨勭紦鍐插尯銆?杩欎簺鍧楀寘鍚細

- 浠?0 寮€濮嬬殑鍧楃紪鍙凤紙u8锛?- 31 瀛楄妭鐨勬湭鐭ユ暟鎹?
   璇ユ柟娉曠殑鍚箟鍦ㄥ緢澶х▼搴︿笂浠嶆湭鐭ャ€?
### WMI 鏂规硶 BatteryDesignVoltage()

杩斿洖鐢垫睜鐨勮璁＄數鍘嬶紙鍗曚綅 mV锛夛紝绫诲瀷涓?u16銆?
### WMI 鏂规硶 BatteryeRawAnalyticsABlock()

杩斿洖鍗曞潡鍒嗘瀽鏁版嵁锛岀储寮曠殑绗簩涓瓧鑺傜敤浜庨€夋嫨鍧楃紪鍙枫€?
**鑷?WMI 鎺ュ彛绗?3 鐗堣捣鏀寔锛?*

   璇ユ柟娉曠殑鍚箟鍦ㄥ緢澶х▼搴︿笂浠嶆湭鐭ャ€?
### WMI 鏂规硶 ReturnVersion()

杩斿洖 WMI 鎺ュ彛鐗堟湰锛岀被鍨嬩负 u32銆?
### WMI 鏂规硶 FanSensorInformation()

杩斿洖涓€涓寘鍚鎵囦紶鎰熷櫒鏉＄洰鐨勭紦鍐插尯锛屼互鍗曚釜 `0xff` 缁撳熬銆?杩欎簺鏉＄洰鍖呭惈锛?
- 椋庢墖绫诲瀷锛坲8锛?- 椋庢墖杞€燂紙鍗曚綅 RPM锛屽皬绔簭 u16锛?
### WMI 鏂规硶 ThermalSensorInformation()

杩斿洖涓€涓寘鍚俯搴︿紶鎰熷櫒鏉＄洰鐨勭紦鍐插尯锛屼互鍗曚釜 `0xff` 缁撳熬銆?杩欎簺鏉＄洰鍖呭惈锛?
- 娓╁害绫诲瀷锛坲8锛?- 褰撳墠娓╁害锛坰8锛?- 鏈€浣庢俯搴︼紙s8锛?- 鏈€楂樻俯搴︼紙s8锛?- 鏈煡瀛楁锛坲8锛?
   TODO锛氬紕娓呮鏈€鍚庝竴涓瓧鑺傜殑鍚箟銆?
## ACPI 鐢垫睜鍖归厤绠楁硶锛圓CPI battery matching algorithm锛?
鐢ㄤ簬鎶?ACPI 鐢垫睜涓庣储寮曞尮閰嶇殑绠楁硶锛屽熀浜庡湪 OEM 杞欢鏃ュ織娑堟伅涓壘鍒扮殑淇℃伅銆?
鍩烘湰涓婏紝瀵逛簬姣忎釜鏂扮殑 ACPI 鐢垫睜锛屼細鎶婄储寮?1 鍒?3 鑳屽悗鐢垫睜鐨勫簭鍒楀彿涓?ACPI 鐢垫睜鐨勫簭鍒楀彿杩涜
姣旇緝銆傜敱浜?ACPI 鐢垫睜鐨勫簭鍒楀彿鏃㈠彲鑳借缂栫爜涓烘櫘閫氭暣鏁帮紝涔熷彲鑳借缂栫爜涓哄崄鍏繘鍒跺€硷紝涓ょ鎯呭喌閮?闇€瑕佹鏌ャ€傜劧鍚庨€夋嫨搴忓垪鍙峰尮閰嶇殑绗竴涓储寮曘€?
搴忓垪鍙蜂负 0 琛ㄧず璇ョ储寮曟湭鍏宠仈瀹為檯鐢垫睜锛屾垨鎵€鍏宠仈鐨勭數姹犱笉瀛樺湪銆?
鏌愪簺鏈哄櫒锛堝 Dell Inspiron 3505锛夊彧鏀寔鍗曞潡鐢垫睜锛屽洜姝ゅ拷鐣ョ數姹犵储寮曘€傛鍥犲姝わ紝椹卞姩渚濊禆 ACPI
鐢垫睜 hook 鏈哄埗鏉ュ彂鐜扮數姹犮€?
## 閫嗗悜宸ョ▼ DDV WMI 鎺ュ彛锛圧everse-Engineering the DDV WMI interface锛?
1. 鎵句竴鍙板彈鏀寔鐨?Dell 绗旇鏈紝閫氬父鏄湪 2020 骞翠箣鍚庣敓浜х殑銆?2. 瀵煎嚭 ACPI 琛ㄥ苟鎼滅储 WMI 璁惧锛堥€氬父绉颁负 "ADDV"锛夈€?3. 瑙ｇ爜鐩稿簲鐨?bmof 鏁版嵁骞舵煡鐪?ASL 浠ｇ爜銆?4. 灏濊瘯閫氳繃姣旇緝鎺у埗娴佷笌鍏跺畠 ACPI 鏂规硶锛堜緥濡傜數姹犵浉鍏虫柟娉曠殑 _BIX 鎴?_BIF锛夛紝鏉ユ帹鏂煇涓?WMI
   鏂规硶鐨勫惈涔夈€?5. 浣跨敤鍐呭缓鐨?UEFI 璇婃柇绋嬪簭鏌ョ湅椋庢墖/娓╁害鐩稿叧鏂规硶鐨勪紶鎰熷櫒绫诲瀷/鍊硷紙鏈夋椂瑕嗙洊闈欐€?ACPI 鏁版嵁瀛楁
   鍙敤浜庢祴璇曚笉鍚岀殑浼犳劅鍣ㄧ被鍨嬪€硷紝鍥犱负鍦ㄦ煇浜涙満鍣ㄤ笂锛岃鏁版嵁鍦ㄧ儹閲嶇疆鍚庝笉浼氳閲嶆柊鍒濆鍖栵級銆?
鎴栬€咃細

1. 鍔犺浇 `dell-wmi-ddv` 椹卞姩锛屽繀瑕佹椂浣跨敤 `force` 妯″潡鍙傛暟銆?2. 浣跨敤 debugfs 鎺ュ彛璁块棶鍘熷鐨勯鎵?娓╁害浼犳劅鍣ㄧ紦鍐插尯鏁版嵁銆?3. 灏嗘暟鎹笌鍐呭缓 UEFI 璇婃柇绋嬪簭杩涜姣旇緝銆?
濡傛灉浣?Dell 绗旇鏈笂鍙敤鐨?DDV WMI 鎺ュ彛鐗堟湰涓嶅彈鏀寔锛屾垨鑰呬綘鐪嬪埌浜嗘湭鐭ョ殑椋庢墖/娓╁害浼犳劅鍣紝璇?鍦?`bugzilla <https://bugzilla.kernel.org>`_ 涓婃彁浜ょ己闄锋姤鍛婏紝浠ヤ究鎶婂畠浠姞鍏?`dell-wmi-ddv`
椹卞姩銆?
鏇村淇℃伅璇峰弬闃?Documentation/admin-guide/reporting-issues.rst銆?