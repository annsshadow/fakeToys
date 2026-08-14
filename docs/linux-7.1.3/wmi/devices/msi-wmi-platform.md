
## MSI WMI 骞冲彴鐗规€ч┍鍔紙msi-wmi-platform锛?

## 绠€浠?

璁稿 MSI 绗旇鏈敮鎸佸悇绉嶇壒鎬э紝渚嬪璇诲彇椋庢墖浼犳劅鍣ㄣ€傝繖浜涚壒鎬х敱宓屽叆寮忔帶鍒跺櫒
鎺у埗锛孉CPI 鍥轰欢鍦ㄥ祵鍏ュ紡鎺у埗鍣ㄦ帴鍙ｄ箣涓婃毚闇蹭簡涓€涓爣鍑嗙殑 ACPI WMI 鎺ュ彛銆?
## WMI 鎺ュ彛鎻忚堪


WMI 鎺ュ彛鎻忚堪鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿浠?宓屽叆寮忎簩杩涘埗 MOF锛坆mof锛夋暟鎹腑瑙ｇ爜鍑烘潵锛?
```

  [WMI, Locale("MS\0x409"),
   Description("This class contains the definition of the package used in other classes"),
   guid("{ABBC0F60-8EA1-11d1-00A0-C90629100000}")]
  class Package {
    [WmiDataId(1), read, write, Description("16 bytes of data")] uint8 Bytes[16];
  };

  [WMI, Locale("MS\0x409"),
   Description("This class contains the definition of the package used in other classes"),
   guid("{ABBC0F63-8EA1-11d1-00A0-C90629100000}")]
  class Package_32 {
    [WmiDataId(1), read, write, Description("32 bytes of data")] uint8 Bytes[32];
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\0x409"),
   Description("Class used to operate methods on a package"),
   guid("{ABBC0F6E-8EA1-11d1-00A0-C90629100000}")]
  class MSI_ACPI {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(1), Implemented, read, write, Description("Return the contents of a package")]
    void GetPackage([out, id(0)] Package Data);

    [WmiMethodId(2), Implemented, read, write, Description("Set the contents of a package")]
    void SetPackage([in, id(0)] Package Data);

    [WmiMethodId(3), Implemented, read, write, Description("Return the contents of a package")]
    void Get_EC([out, id(0)] Package_32 Data);

    [WmiMethodId(4), Implemented, read, write, Description("Set the contents of a package")]
    void Set_EC([in, id(0)] Package_32 Data);

    [WmiMethodId(5), Implemented, read, write, Description("Return the contents of a package")]
    void Get_BIOS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(6), Implemented, read, write, Description("Set the contents of a package")]
    void Set_BIOS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(7), Implemented, read, write, Description("Return the contents of a package")]
    void Get_SMBUS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(8), Implemented, read, write, Description("Set the contents of a package")]
    void Set_SMBUS([in, out, id(0)] Package_32 Data);

    [WmiMethodId(9), Implemented, read, write, Description("Return the contents of a package")]
    void Get_MasterBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(10), Implemented, read, write, Description("Set the contents of a package")]
    void Set_MasterBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(11), Implemented, read, write, Description("Return the contents of a package")]
    void Get_SlaveBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(12), Implemented, read, write, Description("Set the contents of a package")]
    void Set_SlaveBattery([in, out, id(0)] Package_32 Data);

    [WmiMethodId(13), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Temperature([in, out, id(0)] Package_32 Data);

    [WmiMethodId(14), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Temperature([in, out, id(0)] Package_32 Data);

    [WmiMethodId(15), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Thermal([in, out, id(0)] Package_32 Data);

    [WmiMethodId(16), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Thermal([in, out, id(0)] Package_32 Data);

    [WmiMethodId(17), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Fan([in, out, id(0)] Package_32 Data);

    [WmiMethodId(18), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Fan([in, out, id(0)] Package_32 Data);

    [WmiMethodId(19), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Device([in, out, id(0)] Package_32 Data);

    [WmiMethodId(20), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Device([in, out, id(0)] Package_32 Data);

    [WmiMethodId(21), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Power([in, out, id(0)] Package_32 Data);

    [WmiMethodId(22), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Power([in, out, id(0)] Package_32 Data);

    [WmiMethodId(23), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Debug([in, out, id(0)] Package_32 Data);

    [WmiMethodId(24), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Debug([in, out, id(0)] Package_32 Data);

    [WmiMethodId(25), Implemented, read, write, Description("Return the contents of a package")]
    void Get_AP([in, out, id(0)] Package_32 Data);

    [WmiMethodId(26), Implemented, read, write, Description("Set the contents of a package")]
    void Set_AP([in, out, id(0)] Package_32 Data);

    [WmiMethodId(27), Implemented, read, write, Description("Return the contents of a package")]
    void Get_Data([in, out, id(0)] Package_32 Data);

    [WmiMethodId(28), Implemented, read, write, Description("Set the contents of a package")]
    void Set_Data([in, out, id(0)] Package_32 Data);

    [WmiMethodId(29), Implemented, read, write, Description("Return the contents of a package")]
    void Get_WMI([out, id(0)] Package_32 Data);
  };

```
鐢变簬 Windows 澶勭悊 `CreateByteField()` ACPI 杩愮畻绗︽柟寮忎笂鐨勪竴涓壒娈婃€э紙浠呭綋
鏈€缁堣闂簡涓€涓棤鏁堢殑瀛楄妭瀛楁鏃舵墠浼氬彂鐢熼敊璇級锛屾墍鏈夋柟娉曢兘闇€瑕佷竴涓?32 瀛楄妭
鐨勮緭鍏ョ紦鍐插尯锛屽嵆渚?Binary MOF 鍙︽湁璇存槑銆?
杈撳叆缂撳啿鍖哄寘鍚竴涓敤浜庨€夋嫨瑕佽闂殑瀛愮壒鎬х殑鍗曞瓧鑺傦紝浠ュ強 31 瀛楄妭鐨勮緭鍏?鏁版嵁锛屽叾鍚箟鍙栧喅浜庢墍璁块棶鐨勫瓙鐗规€с€?
杈撳嚭缂撳啿鍖哄寘鍚竴涓敤浜庢寚绀烘垚鍔熸垨澶辫触鐨勫崟瀛楄妭锛坄0x00` 琛ㄧず澶辫触锛変互鍙?31 瀛楄妭
鐨勮緭鍑烘暟鎹紝鍏跺惈涔夊彇鍐充簬鎵€璁块棶鐨勫瓙鐗规€с€?
   璐熻矗澶勭悊 WMI 鏂规硶璋冪敤鐨?ACPI 鎺у埗鏂规硶骞堕潪绾跨▼瀹夊叏鐨勩€傝繖鏄竴涓渶瑕佸湪
   椹卞姩鍐呴儴鑷澶勭悊鐨勫浐浠剁己闄枫€?
### WMI 鏂规硶 Get_EC()


杩斿洖宓屽叆寮忔帶鍒跺櫒淇℃伅锛屾墍閫夊瓙鐗规€ф棤鍏崇揣瑕併€傝緭鍑烘暟鎹寘鍚竴涓爣蹇楀瓧鑺傚拰涓€涓?28 瀛楄妭鐨勬帶鍒跺櫒鍥轰欢鐗堟湰瀛楃涓层€?
鏍囧織瀛楄妭鐨勫墠 4 浣嶅寘鍚祵鍏ュ紡鎺у埗鍣ㄦ帴鍙ｇ殑娆＄増鏈彿锛屾帴涓嬫潵鐨?2 浣嶅寘鍚祵鍏ュ紡
鎺у埗鍣ㄦ帴鍙ｇ殑涓荤増鏈彿銆?
绗?7 浣嶈〃绀哄祵鍏ュ紡鎺у埗鍣ㄩ〉闈㈡槸鍚﹀彂鐢熶簡鍙樺寲锛堢‘鍒囧惈涔夋湭鐭ワ級锛屾渶鍚庝竴浣嶈〃绀?骞冲彴鏄惁涓?Tigerlake 骞冲彴銆?
MSI 杞欢浼间箮浠呭湪璇ユ渶鍚庝竴浣嶈缃綅鏃舵墠浣跨敤姝ゆ帴鍙ｃ€?
### WMI 鏂规硶 Get_Fan()


鍙互閫氳繃閫夋嫨瀛愮壒鎬?`0x00` 鏉ヨ闂鎵囪浆閫熶紶鎰熷櫒銆傝緭鍑烘暟鎹渶澶氬寘鍚洓涓互
澶х鏍煎紡瀛樺偍鐨?16 浣嶉鎵囪浆閫熻鏁般€傚ぇ澶氭暟鏈哄櫒骞朵笉鏀寔鍏ㄩ儴鍥涗釜椋庢墖杞€?浼犳劅鍣紝鍥犳鍓╀綑鐨勮鏁拌纭紪鐮佷负 `0x0000`銆?
椋庢墖 RPM 璇绘暟鍙互鐢ㄤ笅闈㈢殑鍏紡璁＄畻锛?
        RPM = 480000 / <fan speed reading>

濡傛灉椋庢墖杞€熻鏁颁负闆讹紝鍒欓鎵?RPM 涔熶负闆躲€?
### WMI 鏂规硶 Get_WMI()


杩斿洖 ACPI WMI 鎺ュ彛鐨勭増鏈紝鎵€閫夊瓙鐗规€ф棤鍏崇揣瑕併€傝緭鍑烘暟鎹寘鍚袱涓瓧鑺傦紝绗竴涓?鍖呭惈涓荤増鏈彿锛屾渶鍚庝竴涓寘鍚?ACPI WMI 鎺ュ彛鐨勬淇鍙枫€?
MSI 杞欢浼间箮浠呭綋涓荤増鏈彿澶т簬 2 鏃舵墠浣跨敤姝ゆ帴鍙ｃ€?
## 閫嗗悜宸ョ▼ MSI WMI 骞冲彴鎺ュ彛


             浠ヨ繛鎺ュ埌鏈哄櫒骞朵骇鐢熷叾浠栦笉鑹奖鍝嶏紝璇峰姟蹇呭皬蹇冦€?
搴曞眰鐨勫祵鍏ュ紡鎺у埗鍣ㄦ帴鍙ｇ敱 `msi-ec` 椹卞姩浣跨敤锛屽苟涓斾技涔庤澶氭柟娉曞彧鏄妸宓屽叆寮?鎺у埗鍣ㄥ唴瀛樼殑涓€閮ㄥ垎澶嶅埗鍒拌緭鍑虹紦鍐插尯涓€?
杩欐剰鍛崇潃锛屽墿浣欑殑 WMI 鏂规硶鍙互閫氳繃瑙傚療 ACPI AML 浠ｇ爜璁块棶浜嗗祵鍏ュ紡鎺у埗鍣?鍐呭瓨鐨勫摢涓€閮ㄥ垎鏉ヨ繘琛岄€嗗悜宸ョ▼銆傝椹卞姩杩樻敮鎸佷竴涓?debugfs 鎺ュ彛锛岀敤浜庣洿鎺ユ墽琛?WMI 鏂规硶銆傛澶栵紝浠讳綍鍏充簬涓嶆敮鎸佺‖浠剁殑瀹夊叏妫€鏌ラ兘鍙互閫氳繃灏嗘ā鍧椾互
`force=true` 鍔犺浇鏉ョ鐢ㄣ€?
鍏充簬 MSI 宓屽叆寮忔帶鍒跺櫒鎺ュ彛鐨勬洿澶氫俊鎭紝鍙互鍦?`msi-ec project <https://github.com/BeardOverflow/msi-ec>`_ 鎵惧埌銆?
鐗瑰埆鎰熻阿 github 鐢ㄦ埛 `glpnk` 灞曠ず浜嗗浣曡В鐮侀鎵囪浆閫熻鏁般€?