
## Lenovo WMI Interface Other Mode Driver (lenovo-wmi-other)


## Introduction

Lenovo WMI Other Mode 鎺ュ彛鎷嗗垎涓哄涓?GUID锛屼富 Other Mode 鎺ュ彛鎻愪緵楂樼骇鐢垫簮璋冧紭鐗规€э紝渚嬪 Package Power Tracking锛圥PT锛夈€傚畠涓庡涓彁渚涘悇鏂规硶涓婁笅鏂囩殑鏁版嵁鍧?GUID 閰嶅銆?
### Other Mode


WMI GUID `DC2A8805-3A8C-41BA-A6F7-092E0089CD3B`

Other Mode WMI 鎺ュ彛浣跨敤 firmware_attributes 绫诲湪 sysfs 涓毚闇茶鎺ュ彛鎻愪緵鐨勫悇绉?WMI 灞炴€с€傝繖浣垮緱 CPU 鍜?GPU 鐨勫姛鑰楅檺鍒惰皟浼橈紝浠ュ強灞炰簬 Lenovo 鈥淕aming Series鈥濊澶囩殑涓€绯诲垪鍏朵粬灞炴€ф垚涓哄彲鑳姐€侽ther Mode 鎺ュ彛鏆撮湶鐨勬瘡涓睘鎬ч兘鏈夊搴旂殑鑳藉姏鏁版嵁鍧楋紝浣块┍鍔ㄨ兘澶熸帰娴嬫湁鍏宠灞炴€х殑缁嗚妭銆傛瘡涓睘鎬ф湁澶氫釜椤甸潰锛屽垎鍒搴斾簬 Gamezone 鎺ュ彛绠＄悊鐨勬瘡涓钩鍙伴厤缃紙profile锛夈€傚睘鎬ч€氳繃浠ヤ笅璺緞鍦?sysfs 涓毚闇诧細

```

  /sys/class/firmware-attributes/lenovo-wmi-other/attributes/<attribute>/

```
姝ゅ锛岃椹卞姩杩樻妸灞炴€у鍑哄埌 HWMON銆?
### LENOVO_CAPABILITY_DATA_00


WMI GUID `362A3AFE-3D96-4665-8530-96DAD5BB300E`

LENOVO_CAPABILITY_DATA_00 鎺ュ彛鎻愪緵涓嶄緷璧?gamezone 鏁ｇ儹妯″紡鐨勫悇绫讳俊鎭€?
瀹炵幇浜嗕互涓?HWMON 灞炴€э細
 - fanX_div: 鍐呴儴 RPM 闄ゆ暟
 - fanX_input: 褰撳墠 RPM
 - fanX_target: 鐩爣 RPM锛堝彲璋冭妭锛?=鑷姩锛?
鐢变簬鍐呴儴 RPM 闄ゆ暟锛屽綋鍓?鐩爣 RPM 浼氳鍚戜笅鍙栨暣鍒版渶杩戠殑鏁存暟鍊嶃€傝闄ゆ暟鏈韩涓嶅繀鏄?2 鐨勫箓銆?
### LENOVO_CAPABILITY_DATA_01


WMI GUID `7A8F5407-CB67-4D6E-B547-39B3BE018154`

LENOVO_CAPABILITY_DATA_01 鎺ュ彛鎻愪緵渚濊禆 gamezone 鏁ｇ儹妯″紡鐨勫悇绫讳俊鎭紝鍖呮嫭闆嗘垚鐨?CPU 鍜?GPU 缁勪欢鐨勫姛鑰楅檺鍒躲€?
姣忎釜灞炴€у叿鏈変互涓嬪睘鎬э細
 - current_value
 - default_value
 - display_name
 - max_value
 - min_value
 - scalar_increment
 - type

瀹炵幇浜嗕互涓?firmware-attributes锛? - ppt_pl1_spl: Platform Profile Tracking Sustained Power Limit
 - ppt_pl2_sppt: Platform Profile Tracking Slow Package Power Tracking
 - ppt_pl3_fppt: Platform Profile Tracking Fast Package Power Tracking

### LENOVO_FAN_TEST_DATA


WMI GUID `B642801B-3D21-45DE-90AE-6E86F164FB21`

LENOVO_FAN_TEST_DATA 鎺ュ彛鎻愪緵鍐峰嵈椋庢墖鑷鐨勫弬鑰冩暟鎹€?
瀹炵幇浜嗕互涓?HWMON 灞炴€э細
 - fanX_max: 鏈€澶?RPM
 - fanX_min: 鏈€灏?RPM

## WMI interface description


WMI 鎺ュ彛鎻忚堪鍙互浣跨敤 `bmfdec <https://github.com/pali/bmfdec>`_ 宸ュ叿浠庡唴宓岀殑浜岃繘鍒?MOF锛坆mof锛夋暟鎹腑瑙ｇ爜锛?
```

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO_OTHER_METHOD class"), guid("{dc2a8805-3a8c-41ba-a6f7-092e0089cd3b}")]
  class LENOVO_OTHER_METHOD {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiMethodId(17), Implemented, Description("Get Feature Value ")] void GetFeatureValue([in] uint32 IDs, [out] uint32 value);
    [WmiMethodId(18), Implemented, Description("Set Feature Value ")] void SetFeatureValue([in] uint32 IDs, [in] uint32 value);
    [WmiMethodId(19), Implemented, Description("Get Data By Command ")] void GetDataByCommand([in] uint32 IDs, [in] uint32 Command, [out] uint32 DataSize, [out, WmiSizeIs("DataSize")] uint32 Data[]);
    [WmiMethodId(99), Implemented, Description("Get Data By Package for TAC")] void GetDataByPackage([in, Max(40)] uint8 Input[], [out] uint32 DataSize, [out, WmiSizeIs("DataSize")] uint8 Data[]);
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 00"), guid("{362a3afe-3d96-4665-8530-96dad5bb300e}")]
  class LENOVO_CAPABILITY_DATA_00 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Capability Default Value.")] uint32 DefaultValue;
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 01"), guid("{7a8f5407-cb67-4d6e-b547-39b3be018154}")]
  class LENOVO_CAPABILITY_DATA_01 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Default Value.")] uint32 DefaultValue;
    [WmiDataId(4), read, Description("Step.")] uint32 Step;
    [WmiDataId(5), read, Description("Minimum Value.")] uint32 MinValue;
    [WmiDataId(6), read, Description("Maximum Value.")] uint32 MaxValue;
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("LENOVO CAPABILITY DATA 02"), guid("{bbf1f790-6c2f-422b-bc8c-4e7369c7f6ab}")]
  class LENOVO_CAPABILITY_DATA_02 {
    [key, read] string InstanceName;
    [read] boolean Active;

    [WmiDataId(1), read, Description(" IDs.")] uint32 IDs;
    [WmiDataId(2), read, Description("Capability.")] uint32 Capability;
    [WmiDataId(3), read, Description("Data Size.")] uint32 DataSize;
    [WmiDataId(4), read, Description("Default Value"), WmiSizeIs("DataSize")] uint8 DefaultValue[];
  };

  [WMI, Dynamic, Provider("WmiProv"), Locale("MS\\0x409"), Description("Definition of Fan Test Data"), guid("{B642801B-3D21-45DE-90AE-6E86F164FB21}")]
  class LENOVO_FAN_TEST_DATA {
    [key, read] string InstanceName;
    [read] boolean Active;
    [WmiDataId(1), read, Description("Mode.")] uint32 NumOfFans;
    [WmiDataId(2), read, Description("Fan ID."), WmiSizeIs("NumOfFans")] uint32 FanId[];
    [WmiDataId(3), read, Description("Maximum Fan Speed."), WmiSizeIs("NumOfFans")] uint32 FanMaxSpeed[];
    [WmiDataId(4), read, Description("Minumum Fan Speed."), WmiSizeIs("NumOfFans")] uint32 FanMinSpeed[];
  };

```
