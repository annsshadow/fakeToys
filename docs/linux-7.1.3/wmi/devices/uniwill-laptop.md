
## Uniwill 绗旇鏈┍鍔紙uniwill-laptop锛?

## 绠€浠?

Uniwill 鍒堕€犵殑璁稿绗旇鏈紙鏃犺鏄洿鎺ュ埗閫犺繕鏄綔涓?ODM锛夋彁渚涗簡涓€涓?EC 鎺ュ彛锛?鐢ㄤ簬鎺у埗浼犳劅鍣ㄥ拰椋庢墖鎺у埗绛夊悇绫诲钩鍙拌缃€傝鎺ュ彛琚?`uniwill-laptop` 椹卞姩鐢ㄦ潵
灏嗚繖浜涘姛鑳芥槧灏勫埌鏍囧噯鐨勫唴鏍告帴鍙ｄ笂銆?
## EC WMI 鎺ュ彛鎻忚堪


EC WMI 鎺ュ彛鎻忚堪鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿浠?鍐呭祵鐨勪簩杩涘埗 MOF锛坆mof锛夋暟鎹腑瑙ｇ爜鍑烘潵锛?
```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"),
   Description("Class used to operate methods on a ULong"),
   guid("{ABBC0F6F-8EA1-11d1-00A0-C90629100000}")]
  class AcpiTest_MULong {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(1), Implemented, read, write, Description("Return the contents of a ULong")]
    void GetULong([out, Description("Ulong Data")] uint32 Data);

    [WmiMethodId(2), Implemented, read, write, Description("Set the contents of a ULong")]
    void SetULong([in, Description("Ulong Data")] uint32 Data);

    [WmiMethodId(3), Implemented, read, write,
     Description("Generate an event containing ULong data")]
    void FireULong([in, Description("WMI requires a parameter")] uint32 Hack);

    [WmiMethodId(4), Implemented, read, write, Description("Get and Set the contents of a ULong")]
    void GetSetULong([in, Description("Ulong Data")] uint64 Data,
                     [out, Description("Ulong Data")] uint32 Return);

    [WmiMethodId(5), Implemented, read, write,
     Description("Get and Set the contents of a ULong for Dollby button")]
    void GetButton([in, Description("Ulong Data")] uint64 Data,
                   [out, Description("Ulong Data")] uint32 Return);
  };

```
澶ч儴鍒?WMI 鐩稿叧浠ｇ爜鏄粠 Windows 椹卞姩绀轰緥澶嶅埗鑰屾潵鐨勶紝閬楁喚鐨勬槸杩欐剰鍛崇潃璇?WMI-GUID 骞朵笉鍞竴銆傝繖浣垮緱璇?WMI-GUID 鏃犳硶鐢ㄤ簬鑷姩鍔犺浇銆?
### WMI 鏂规硶 GetULong()


姝?WMI 鏂规硶鏄粠 Windows 椹卞姩绀轰緥澶嶅埗鑰屾潵锛屾病鏈夊疄闄呭姛鑳姐€?
### WMI 鏂规硶 SetULong()


姝?WMI 鏂规硶鏄粠 Windows 椹卞姩绀轰緥澶嶅埗鑰屾潵锛屾病鏈夊疄闄呭姛鑳姐€?
### WMI 鏂规硶 FireULong()


姝?WMI 鏂规硶鍏佽娉ㄥ叆涓€涓甫鏈?32 浣嶈礋杞界殑 WMI 浜嬩欢銆傚叾涓昏鐢ㄩ€斾技涔庢槸璋冭瘯銆?
### WMI 鏂规硶 GetSetULong()


姝?WMI 鏂规硶鐢ㄤ簬涓?EC 閫氫俊銆俙Data` 鍙傛暟鍖呭惈浠ヤ笅淇℃伅锛堜粠鏈€浣庢湁鏁堝瓧鑺傚紑濮嬶級锛?
1. 16 浣嶅湴鍧€
2. 16 浣嶆暟鎹紙璇诲彇鏃惰涓?`0x0000`锛?3. 16 浣嶆搷浣滐紙`0x0100` 琛ㄧず璇诲彇锛宍0x0000` 琛ㄧず鍐欏叆锛?4. 16 浣嶄繚鐣欙紙璁句负 `0x0000`锛?
`Return` 鍊肩殑鍓?8 浣嶅寘鍚湪璇诲彇鏃?EC 杩斿洖鐨勬暟鎹€傜壒娈婂€?`0xFEFEFEFE` 鐢ㄤ簬
鎸囩ず涓?EC 閫氫俊澶辫触銆?
### WMI 鏂规硶 GetButton()


姝?WMI 鏂规硶骞堕潪鍦ㄦ墍鏈夋満鍣ㄤ笂閮藉凡瀹炵幇锛岀敤閫旀湭鐭ャ€?
## 閫嗗悜宸ョ▼ EC WMI 鎺ュ彛


             瀛樺湪鍓綔鐢紝璇峰皬蹇冦€?
`GetSetULong` 鏂规硶鑳屽悗鐨?EC 鐢卞埗閫犲晢鎻愪緵鐨?OEM 杞欢浣跨敤銆傜敱浜庤杞欢浣跨敤浜?娣锋穯鍣紝閫嗗悜宸ョ▼姣旇緝鍥伴毦锛屼絾鍏朵腑閮ㄥ垎鍐呭骞舵湭琚贩娣嗐€傚湪杩欑鎯呭喌涓嬶紝`dnSpy
<https://github.com/dnSpy/dnSpy>`_ 涔熷彲鑳芥湁鎵€甯姪銆?
鍦?Windows 涓嬪彲浠ヤ娇鐢?powershell锛堥渶瑕佺鐞嗗憳鏉冮檺锛夎闂?EC锛?
```

  > $obj = Get-CimInstance -Namespace root/wmi -ClassName AcpiTest_MULong | Select-Object -First 1
  > Invoke-CimMethod -InputObject $obj -MethodName GetSetULong -Arguments @{Data = <input>}

```
## WMI 浜嬩欢鎺ュ彛鎻忚堪


WMI 鎺ュ彛鎻忚堪鍚屾牱鍙互浠庡唴宓岀殑浜岃繘鍒?MOF锛坆mof锛夋暟鎹腑瑙ｇ爜锛?
```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"),
   Description("Class containing event generated ULong data"),
   guid("{ABBC0F72-8EA1-11d1-00A0-C90629100000}")]
  class AcpiTest_EventULong : WmiEvent {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, write, Description("ULong Data")] uint32 ULong;
  };

```
澶ч儴鍒?WMI 鐩稿叧浠ｇ爜鍚屾牱鏄粠 Windows 椹卞姩绀轰緥澶嶅埗鑰屾潵锛屽鑷存 WMI 鎺ュ彛鍙楀埌
涓庝笂杩?EC WMI 鎺ュ彛鐩稿悓鐨勯檺鍒躲€?
### WMI 浜嬩欢鏁版嵁


WMI 浜嬩欢鏁版嵁鍖呭惈涓€涓崟鐙殑 32 浣嶅€硷紝鐢ㄤ簬鎸囩ず鍚勭骞冲彴浜嬩欢銆?
## 閫嗗悜宸ョ▼ Uniwill WMI 浜嬩欢鎺ュ彛


椹卞姩鍦ㄦ敹鍒?WMI 浜嬩欢鏃朵細璁板綍璋冭瘯娑堟伅銆傚洜姝ゅ惎鐢ㄨ皟璇曟秷鎭湁鍔╀簬鏌ユ壘鏈煡鐨勪簨浠?浠ｇ爜銆?
## EC ACPI 鎺ュ彛鎻忚堪


`INOU0000` ACPI 璁惧鏄竴涓櫄鎷熻澶囷紝鐢ㄤ簬璁块棶 Uniwill 鍒堕€犵殑绗旇鏈笂鍙敤鐨?鍚勭纭欢瀵勫瓨鍣ㄣ€傞€氳繃璋冪敤 ACPI 鎺у埗鏂规硶鏉ヨ鍐欒繖浜涘瘎瀛樺櫒銆俙uniwill-laptop`
椹卞姩浣跨敤姝よ澶囦笌 EC 閫氫俊锛屽洜涓?ACPI 鎺у埗鏂规硶姣斾笂杩?WMI 鏂规硶鏇村揩銆?
鐢ㄤ簬璇诲彇瀵勫瓨鍣ㄧ殑 ACPI 鎺у埗鏂规硶鎺ュ彈涓€涓寘鍚緟璇诲彇瀵勫瓨鍣ㄥ湴鍧€鐨?ACPI 鏁存暟锛?骞惰繑鍥炰竴涓寘鍚瀵勫瓨鍣ㄥ唴鏁版嵁鐨?ACPI 鏁存暟銆傝€岀敤浜庡啓鍏ュ瘎瀛樺櫒鐨?ACPI 鎺у埗鏂规硶
鍒欐帴鍙椾袱涓?ACPI 鏁存暟锛岄澶栫殑 ACPI 鏁存暟鍖呭惈瑕佸啓鍏ュ瘎瀛樺櫒鐨勬暟鎹€傛绫?ACPI 鎺у埗
鏂规硶涓嶈繑鍥炰换浣曞唴瀹广€?
### 绯荤粺鍐呭瓨


绯荤粺鍐呭瓨鍙互浠ュ崟瀛楄妭绮掑害璁块棶锛坄MMRB` 鐢ㄤ簬璇诲彇锛宍MMWB` 鐢ㄤ簬鍐欏叆锛夛紝鎴栦互鍥涘瓧鑺?绮掑害璁块棶锛坄MMRD` 鐢ㄤ簬璇诲彇锛宍MMWD` 鐢ㄤ簬鍐欏叆锛夈€傝繖浜?ACPI 鎺у埗鏂规硶鏈浣跨敤锛屽洜涓?涓庡唴鏍告彁渚涚殑鍘熺敓鍐呭瓨璁块棶鍑芥暟鐩告瘮锛屽畠浠病鏈夋彁渚涗换浣曞ソ澶勩€?
### EC RAM


EC 鐨勫唴閮?RAM 鍙互浣跨敤 `ECRR`锛堣锛夊拰 `ECRW`锛堝啓锛堿CPI 鎺у埗鏂规硶浠ュ崟瀛楄妭绮掑害
璁块棶锛屾渶澶у瘎瀛樺櫒鍦板潃涓?`0xFFF`銆侽EM 杞欢鍦ㄨ皟鐢ㄥ叾涓竴涓?ACPI 鎺у埗鏂规硶鍚庝細绛夊緟
6 ms锛屽彲鑳芥槸涓轰簡閬垮厤閫氳繃 LPC 杩炴帴鏃朵娇 EC 杩囪浇銆?
### PCI 閰嶇疆绌洪棿


PCI 閰嶇疆绌洪棿鍙互浣跨敤 `PCRD`锛堣锛夊拰 `PCWD`锛堝啓锛堿CPI 鎺у埗鏂规硶浠ュ洓瀛楄妭绮掑害璁块棶銆?纭垏鐨勫湴鍧€鏍煎紡鏈煡锛屽苟涓旈殢鎰忔帰娴嬮殢鏈?PCI 璁惧鍙兘浼氭壈涔?PCI 瀛愮郴缁熴€傚洜姝よ繖浜?ACPI 鎺у埗鏂规硶鏈浣跨敤銆?
### IO 绔彛


IO 绔彛鍙互浣跨敤 `IORD`锛堣锛夊拰 `IOWD`锛堝啓锛堿CPI 鎺у埗鏂规硶浠ュ洓瀛楄妭绮掑害璁块棶銆傝繖浜?ACPI 鎺у埗鏂规硶鏈浣跨敤锛屽洜涓轰笌鍐呮牳鎻愪緵鐨勫師鐢?IO 绔彛璁块棶鍑芥暟鐩告瘮锛屽畠浠病鏈夋彁渚?浠讳綍濂藉銆?
### CMOS RAM


CMOS RAM 鍙互浣跨敤 `RCMS`锛堣锛夊拰 `WCMS` ACPI 鎺у埗鏂规硶浠ュ崟瀛楄妭绮掑害璁块棶銆傜敱浜庝娇鐢?浜嗙储寮?IO锛屼娇鐢ㄨ繖浜?ACPI 鏂规硶鍙兘浼氬共鎵板唴鏍告彁渚涚殑鍘熺敓 CMOS RAM 璁块棶鍑芥暟锛屽洜姝?瀹冧滑鏈浣跨敤銆?
### 绱㈠紩 IO


浣跨敤 IO 绔彛銆佷互鍗曞瓧鑺傜矑搴︾殑绱㈠紩 IO 鍙互閫氳繃 `RIOP`锛堣锛夊拰 `WIOP`锛堝啓锛堿CPI 鎺у埗
鏂规硶鎵ц銆傝繖浜?ACPI 鏂规硶鏈浣跨敤锛屽洜涓轰笌鍐呮牳鎻愪緵鐨勫師鐢?IO 绔彛璁块棶鍑芥暟鐩告瘮锛屽畠浠?娌℃湁鎻愪緵浠讳綍濂藉銆?
鐗规鎰熻阿 github 鐢ㄦ埛 `pobrn`锛屽叾寮€鍙戠殑 `qc71_laptop
<https://github.com/pobrn/qc71_laptop>`_ 椹卞姩鏄湰椹卞姩鐨勯儴鍒嗗熀纭€銆俆uxedo Computers
涔熸槸濡傛锛屽叾寮€鍙戠殑 `tuxedo-drivers
<https://gitlab.com/tuxedocomputers/development/packages/tuxedo-drivers>`_ 杞欢鍖?涔熶綔涓烘湰椹卞姩鐨勫熀纭€銆?