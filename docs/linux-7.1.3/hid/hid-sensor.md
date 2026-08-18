## HID Sensors 妗嗘灦

HID sensor 妗嗘灦鎻愪緵浜嗗疄鐜?sensor 椹卞姩鎵€闇€鐨勬帴鍙ｏ紝杩欎簺椹卞姩杩炴帴鍒?sensor hub銆俿ensor hub 鏄竴涓?HID 璁惧锛屽畠鎻愪緵涓€涓鍚?HID 1.12 sensor usage 琛ㄧ殑鎶ュ憡鎻忚堪绗︺€?

鏉ヨ嚜 HID 1.12 鈥淗ID Sensor Usages鈥?瑙勮寖鐨勬弿杩帮細
鈥滃 sensor 鐨?HID usage 杩涜鏍囧噯鍖栵紝鍙互锛堜絾涓嶆槸蹇呴』锛夎 sensor 纭欢鍘傚晢鍦?USB 杈圭晫鎻愪緵涓€涓竴鑷寸殑鍗虫彃鍗崇敤锛圥lug And Play锛夋帴鍙ｏ紝浠庤€屼娇鏌愪簺鎿嶄綔绯荤粺鑳藉闆嗘垚鍙湪鍘傚晢涔嬮棿澶嶇敤鐨勯€氱敤璁惧椹卞姩锛屽厤闄ゅ巶鍟嗚嚜琛屾彁渚涢┍鍔ㄧ殑闇€瑕併€傗€?

璇ヨ鑼冨畾涔変簡澶ч噺 usage ID锛屽畠浠弿杩?sensor 鐨勭被鍨嬩互鍙婂悇涓暟鎹瓧娈点€傛瘡涓?sensor 鍙互鍏锋湁鍙彉鏁伴噺鐨勬暟鎹瓧娈点€傚瓧娈电殑闀垮害鍜岄『搴忕敱鎶ュ憡鎻忚堪绗︽寚瀹氥€傚浜?

```
     INPUT(1)[INPUT]
   ..
      Field(2)
        Physical(0020.0073)
        Usage(1)
          0020.045f
        Logical Minimum(-32767)
        Logical Maximum(32767)
        Report Size(8)
        Report Count(1)
        Report Offset(16)
        Flags(Variable Absolute)
  ..
  ..

```
璇ユ姤鍛婅〃鏄?鈥渟ensor page (0x20)鈥?鍖呭惈涓€涓?accelerometer-3D锛堝姞閫熷害璁?3D锛?x73锛夈€傝繖涓?accelerometer-3D 鍏锋湁涓€浜涘瓧娈点€備緥濡傝繖閲屽瓧娈?2 鏄?motion intensity锛堣繍鍔ㄥ己搴︼紝0x045f锛夛紝鍏堕€昏緫鏈€灏忓€间负 -32767锛岄€昏緫鏈€澶у€间负 32767銆傚瓧娈电殑椤哄簭鍜屾瘡涓瓧娈电殑闀垮害寰堥噸瑕侊紝鍥犱负杈撳叆浜嬩欢鐨勫師濮嬫暟鎹皢浣跨敤杩欑鏍煎紡銆?


## 瀹炵幇


璇ヨ鑼冨畾涔変簡澶氱鍏锋湁涓嶅悓鏁版嵁瀛楁闆嗗悎鐨?sensor 绫诲瀷銆傚浜庝笉鍚岀殑 sensor锛屽緢闅炬湁涓€涓€氱敤鐨勮緭鍏ヤ簨浠朵紶缁欑敤鎴风┖闂村簲鐢ㄧ▼搴忋€備緥濡傚姞閫熷害璁″彲浠ュ彂閫?X銆乊 鍜?Z 鏁版嵁锛岃€岀幆澧冨厜 sensor 鍙互鍙戦€佺収搴︽暟鎹€?
鍥犳瀹炵幇鍒嗕负涓ら儴鍒嗭細

- Core HID 椹卞姩
- 鍗曠嫭鐨?sensor 澶勭悊閮ㄥ垎锛坰ensor 椹卞姩锛?

### Core 椹卞姩

core 椹卞姩锛坔id-sensor-hub锛変綔涓轰竴涓?HID 椹卞姩娉ㄥ唽銆傚畠瑙ｆ瀽鎶ュ憡鎻忚堪绗﹀苟璇嗗埆鎵€鏈夊瓨鍦ㄧ殑 sensor銆傚畠娣诲姞涓€涓悕涓?HID-SENSOR-xxxx 鐨?MFD 璁惧锛堝叾涓?xxxx 鏄鑼冧腑鐨?usage id锛夈€?

渚嬪锛?

HID-SENSOR-200073 娉ㄥ唽涓轰竴涓?Accelerometer 3D锛堜笁缁村姞閫熷害璁★級椹卞姩銆?

鍥犳锛屽鏋滄彃鍏ヤ簡浠讳綍鍏锋湁璇ュ悕绉扮殑椹卞姩锛屽氨浼氳皟鐢ㄨ鍑芥暟鐨?probe 渚嬬▼銆傛墍浠ヤ竴涓姞閫熷害璁″鐞嗛┍鍔ㄥ彲浠ョ敤璇ュ悕绉版敞鍐岋紝骞跺湪妫€娴嬪埌 accelerometer-3D 鏃惰 probe銆?

core 椹卞姩鎻愪緵浜嗕竴缁?API锛屼緵澶勭悊椹卞姩鐢ㄦ潵娉ㄥ唽骞惰幏鍙栬 usage id 鐨勪簨浠躲€傚悓鏃跺畠杩樻彁渚涜В鏋愬嚱鏁帮紝鐢ㄤ簬鑾峰彇鍜岃缃瘡涓?input/feature/output 鎶ュ憡銆?

### 鍗曠嫭鐨?sensor 澶勭悊閮ㄥ垎锛坰ensor 椹卞姩锛?


澶勭悊椹卞姩灏嗕娇鐢?core 椹卞姩鎻愪緵鐨勬帴鍙ｆ潵瑙ｆ瀽鎶ュ憡骞惰幏鍙栧瓧娈电殑绱㈠紩锛屼篃鍙互鑾峰彇浜嬩欢銆傝椹卞姩鍙互浣跨敤 IIO 鎺ュ彛鏉ヤ娇鐢ㄤ负鏌愮被 sensor 瀹氫箟鐨勬爣鍑?ABI銆?


## Core 椹卞姩鎺ュ彛


```
  Each processing driver can use this structure to set some callbacks.
	int (*suspend)(..): Callback when HID suspend is received
	int (*resume)(..): Callback when HID resume is received
	int (*capture_sample)(..): Capture a sample for one of its data fields
	int (*send_event)(..): One complete event is received which can have
                               multiple data fields.

```
```

  int sensor_hub_register_callback(struct hid_sensor_hub_device *hsdev,
			u32 usage_id,
			struct hid_sensor_hub_callbacks *usage_callback):

```
涓烘煇涓?usage id 娉ㄥ唽鍥炶皟銆傚洖璋冨嚱鏁颁笉鍏佽

```

  int sensor_hub_remove_callback(struct hid_sensor_hub_device *hsdev,
			u32 usage_id):

```
绉婚櫎鏌愪釜 usage id 鐨勫洖璋冦€?


```

  int sensor_hub_input_get_attribute_info(struct hid_sensor_hub_device *hsdev,
			u8 type,
			u32 usage_id, u32 attr_usage_id,
			struct hid_sensor_hub_attribute_info *info);

```
澶勭悊椹卞姩鍙互鏌ユ壘鏌愪釜鎰熷叴瓒ｇ殑瀛楁锛屽苟妫€鏌ュ畠鏄惁瀛樺湪浜庢姤鍛婃弿杩扮涓€傚鏋滃瓨鍦紝瀹冨皢瀛樺偍蹇呰鐨勪俊鎭紝浠ヤ究鍙互鍗曠嫭鍦拌缃垨鑾峰彇瀛楁銆?
杩欎簺绱㈠紩閬垮厤浜嗘瘡娆￠兘鍘绘悳绱㈠苟鑾峰彇瀛楁绱㈠紩鏉ヨ繘琛岃缃垨鑾峰彇銆?


```

  int sensor_hub_set_feature(struct hid_sensor_hub_device *hsdev, u32 report_id,
			u32 field_index, s32 value);

```
璇ユ帴鍙ｇ敤浜庤缃?feature 鎶ュ憡涓煇涓瓧娈电殑鍊笺€備緥濡傦紝濡傛灉瀛樺湪涓€涓瓧娈?report_interval锛堜箣鍓嶇敱瀵?sensor_hub_input_get_attribute_info 鐨勮皟鐢ㄨВ鏋愬緱鍒帮級锛岄偅涔堝畠鍙互鐩存帴璁剧疆璇?


```

  int sensor_hub_get_feature(struct hid_sensor_hub_device *hsdev, u32 report_id,
			u32 field_index, s32 *value);

```
璇ユ帴鍙ｇ敤浜庤幏鍙?input 鎶ュ憡涓煇涓瓧娈电殑鍊笺€備緥濡傦紝濡傛灉瀛樺湪涓€涓瓧娈?report_interval锛堜箣鍓嶇敱瀵?sensor_hub_input_get_attribute_info 鐨勮皟鐢ㄨВ鏋愬緱鍒帮級锛岄偅涔堝畠鍙互鐩存帴鑾峰彇璇?


```

  int sensor_hub_input_attr_get_raw_value(struct hid_sensor_hub_device *hsdev,
			u32 usage_id,
			u32 attr_usage_id, u32 report_id);

```
璇ユ帴鍙ｇ敤浜庨€氳繃 input 鎶ュ憡鑾峰彇鏌愪釜鐗瑰畾瀛楁鐨勫€笺€備緥濡傚姞閫熷害璁℃兂瑕佽疆璇?X 杞寸殑鍊硷紝灏卞彲浠ョ敤 X 杞寸殑 usage id 璋冪敤姝ゅ嚱鏁般€侶ID sensor 鍙互鎻愪緵浜嬩欢锛屽洜姝や笉蹇呰疆璇换浣曞瓧娈点€傚鏋滄湁鏂版牱鏈紝core 椹卞姩浼氳皟鐢ㄥ凡娉ㄥ唽鐨勫洖璋冨嚱鏁版潵瀵规牱鏈繘琛屽鐞嗐€?


----------

### HID Custom 涓?Generic Sensor


HID Sensor 瑙勮寖瀹氫箟浜嗕袱绉嶇壒娈婄殑 sensor usage 绫诲瀷銆傜敱浜庡畠浠笉浠ｈ〃鏍囧噯 sensor锛屽洜姝ゆ棤娉曠敤 Linux IIO 绫诲瀷鎺ュ彛鏉ュ畾涔夈€?
杩欎簺 sensor 鐨勭洰鐨勬槸鎵╁睍鍔熻兘锛屾垨鎻愪緵涓€绉嶆柟寮忔潵娣锋穯 sensor 鎵€浼犻€掔殑鏁版嵁銆傚湪涓嶇煡閬撴暟鎹笌鍏跺皝瑁呭舰寮忎箣闂寸殑鏄犲皠鍏崇郴鏃讹紝搴旂敤绋嬪簭/椹卞姩寰堥毦鍒ゆ柇 sensor 姝ｅ湪浼犻€掍粈涔堟暟鎹€?
杩欏厑璁镐竴浜涘樊寮傚寲鐨勭敤渚嬶紝鍘傚晢鍙互鍦ㄥ叾涓彁渚涘簲鐢ㄧ▼搴忋€備竴浜涘父瑙佺敤渚嬫槸璋冭瘯鍏朵粬 sensor锛屾垨鎻愪緵涓€浜涜濡傞敭鐩樻帴鍏?绉婚櫎銆佺洊瀛愬紑/鍚堜箣绫荤殑浜嬩欢銆?

涓轰簡璁╁簲鐢ㄧ▼搴忚兘澶熷埄鐢ㄨ繖浜?sensor锛岃繖閲岄€氳繃 sysfs 灞炴€х粍銆佸睘鎬т互鍙?misc 璁惧鎺ュ彛灏嗗畠浠鍑恒€?

```

  /sys/devices/pci0000:00/INT33C2:00/i2c-0/i2c-INT33D1:00/0018:8086:09FA.0001/HID-SENSOR-2000e1.6.auto$ tree -R
  .
  鈹偮犅?鈹溾攢鈹€  enable_sensor
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-maximum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-minimum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-size
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-unit-expo
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-units
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-0-200316-value
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-maximum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-minimum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-size
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-unit-expo
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-units
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ feature-1-200201-value
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-maximum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-minimum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-size
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-unit-expo
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-units
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-0-200201-value
  鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-maximum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-minimum
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-name
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-size
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-unit-expo
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-units
  鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ input-1-200202-value

```
杩欓噷鏄竴涓叿鏈夊洓涓瓧娈电殑 custom sensor锛氫袱涓?feature 鍜屼袱涓?input銆?
姣忎釜瀛楁鐢变竴缁勫睘鎬ц〃绀恒€傞櫎 鈥渧alue鈥?涔嬪鐨勬墍鏈夊瓧娈甸兘鏄彧璇荤殑銆倂alue 瀛楁鏄彲璇诲啓瀛楁銆?

```

  /sys/bus/platform/devices/HID-SENSOR-2000e1.6.auto/feature-0-200316$ grep -r . *
  feature-0-200316-maximum:6
  feature-0-200316-minimum:0
  feature-0-200316-name:property-reporting-state
  feature-0-200316-size:1
  feature-0-200316-unit-expo:0
  feature-0-200316-units:25
  feature-0-200316-value:1

```
##### 濡備綍鍚敤姝ょ被 sensor锛?


榛樿鎯呭喌涓?sensor 鍙互澶勪簬鐢垫簮闂ㄦ帶锛坧ower gated锛夌姸鎬併€傝鍚敤鍙互閫氳繃 sysfs 灞炴€?鈥渆nable鈥?

```

	$ echo 1 > enable_sensor

```
涓€鏃﹀惎鐢ㄥ苟涓婄數锛宻ensor 灏卞彲浠ラ€氳繃 HID 鎶ュ憡涓婃姤鍊笺€?
```

	/dev$ tree | grep HID-SENSOR-2000e1.6.auto
	鈹偮犅?鈹偮犅?鈹偮犅?鈹溾攢鈹€ 10:53 -> ../HID-SENSOR-2000e1.6.auto
	鈹偮犅?鈹溾攢鈹€  HID-SENSOR-2000e1.6.auto

```
姣忎釜鎶ュ憡鍙互鏄暱搴﹀彲鍙樸€佸墠闈㈠甫鏈夊ご閮ㄧ殑褰㈠紡銆傝澶撮儴鐢变竴涓?32 浣嶇殑 usage id銆?4 浣嶇殑鏃堕棿鎴充互鍙?32 浣嶇殑鍘熷鏁版嵁闀垮害瀛楁缁勬垚銆?

