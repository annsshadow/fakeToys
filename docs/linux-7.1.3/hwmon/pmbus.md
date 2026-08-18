## 鍐呮牳 pmbus 椹卞姩


鏀寔鐨勮姱鐗囷細

  - Flex BMR310, BMR453, BMR454, BMR456, BMR457, BMR458, BMR480,
    BMR490, BMR491, BMR492

    鍓嶇紑锛?bmr310', 'bmr453', 'bmr454', 'bmr456', 'bmr457', 'bmr458', 'bmr480',
    'bmr490', 'bmr491', 'bmr492'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛?
	https://flexpowermodules.com/products


  - ON Semiconductor ADP4000, NCP4200, NCP4208

    鍓嶇紑锛?adp4000', 'ncp4200', 'ncp4208'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛?
	https://www.onsemi.com/pub_link/Collateral/ADP4000-D.PDF

	https://www.onsemi.com/pub_link/Collateral/NCP4200-D.PDF

	https://www.onsemi.com/pub_link/Collateral/JUNE%202009-%20REV.%200.PDF

  - Lineage Power

    鍓嶇紑锛?mdt040', 'pdt003', 'pdt006', 'pdt012', 'udt020'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛?
	http://www.lineagepower.com/oem/pdf/PDT003A0X.pdf

	http://www.lineagepower.com/oem/pdf/PDT006A0X.pdf

	http://www.lineagepower.com/oem/pdf/PDT012A0X.pdf

	http://www.lineagepower.com/oem/pdf/UDT020A0X.pdf

	http://www.lineagepower.com/oem/pdf/MDT040A0X.pdf

  - Texas Instruments TPS40400, TPS544B20, TPS544B25, TPS544C20, TPS544C25

    鍓嶇紑锛?tps40400', 'tps544b20', 'tps544b25', 'tps544c20', 'tps544c25'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛?
	https://www.ti.com/lit/gpn/tps40400

	https://www.ti.com/lit/gpn/tps544b20

	https://www.ti.com/lit/gpn/tps544b25

	https://www.ti.com/lit/gpn/tps544c20

	https://www.ti.com/lit/gpn/tps544c25

  - Maxim MAX20796

    鍓嶇紑锛?max20796'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛?
	https://www.analog.com/media/en/technical-documentation/data-sheets/MAX20796.pdf

  - Generic PMBus devices

    鍓嶇紑锛?pmbus'

    鎵弿鐨勫湴鍧€锛?

    鏁版嵁鎵嬪唽锛歯.a.


浣滆€咃細Guenter Roeck <linux@roeck-us.net>


### 鎻忚堪


璇ラ┍鍔ㄦ敮鎸佸绉嶇鍚?PMBus 瑙勮寖璁惧鐨勭‖浠剁洃鎺с€傚畠鏀寔璁惧鎵€鑳芥彁渚涚殑鐢靛帇銆佺數娴併€佸姛鐜囦笌娓╁害浼犳劅鍣ㄣ€?
姣忎釜琚洃鎺х殑閫氶亾閮芥湁鍚勮嚜鐨勯珮銆佷綆闄愬埗锛屼互鍙婁竴涓复鐣岄檺鍒躲€?
椋庢墖鏀寔灏嗗湪鏈┍鍔ㄧ殑鍚庣画鐗堟湰涓姞鍏ャ€?

### 浣跨敤璇存槑


璇ラ┍鍔ㄤ笉浼氫富鍔ㄦ帰娴?PMBus 璁惧锛屽洜涓烘病鏈夊彲浠ュ畨鍏ㄧ敤浜庤瘑鍒姱鐗囩殑瀵勫瓨鍣紙MFG_ID 瀵勫瓨鍣ㄥ苟闈炴墍鏈夎姱鐗囬兘鏀寔锛夛紝鑰屼笖 PMBus 璁惧涔熸病鏈夋槑纭晫瀹氱殑鍦板潃鑼冨洿銆備綘蹇呴』鏄惧紡鍦板疄渚嬪寲杩欎簺璁惧銆?
绀轰緥锛氫互涓嬪懡浠ゅ皢涓哄湴鍧€ 0x60 澶勭殑 LTC2978 鍔犺浇璇ラ┍鍔?```

	$ modprobe pmbus
	$ echo ltc2978 0x60 > /sys/bus/i2c/devices/i2c-1/new_device


```
### 骞冲彴鏁版嵁鏀寔


鍙€氳繃鍦ㄦ柊鐨勮姱鐗囦笓鐢ㄩ┍鍔ㄦ枃浠朵腑瀹氫箟鑺墖鍙傛暟锛屾潵娣诲姞瀵规洿澶?PMBus 鑺墖鐨勬敮鎸併€備緥濡傦紝鐢ㄤ簬娣诲姞浠ヤ笅鑺墖鏀寔锛堟湭缁忔祴璇曪級鐨勪唬鐮侊細
```

  static struct pmbus_driver_info ds1200_info = {
	.pages = 1,
	/* Note: All other sensors are in linear mode */
	.direct[PSC_VOLTAGE_OUT] = true,
	.direct[PSC_TEMPERATURE] = true,
	.direct[PSC_CURRENT_OUT] = true,
	.m[PSC_VOLTAGE_IN] = 1,
	.b[PSC_VOLTAGE_IN] = 0,
	.R[PSC_VOLTAGE_IN] = 3,
	.m[PSC_VOLTAGE_OUT] = 1,
	.b[PSC_VOLTAGE_OUT] = 0,
	.R[PSC_VOLTAGE_OUT] = 3,
	.m[PSC_TEMPERATURE] = 1,
	.b[PSC_TEMPERATURE] = 0,
	.R[PSC_TEMPERATURE] = 3,
	.func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_IIN | PMBUS_HAVE_STATUS_INPUT
		   | PMBUS_HAVE_VOUT | PMBUS_HAVE_STATUS_VOUT
		   | PMBUS_HAVE_IOUT | PMBUS_HAVE_STATUS_IOUT
		   | PMBUS_HAVE_PIN | PMBUS_HAVE_POUT
		   | PMBUS_HAVE_TEMP | PMBUS_HAVE_STATUS_TEMP
		   | PMBUS_HAVE_FAN12 | PMBUS_HAVE_STATUS_FAN12,
  };

  static int ds1200_probe(struct i2c_client *client)
  {
	return pmbus_do_probe(client, &ds1200_info);
  }

  static const struct i2c_device_id ds1200_id[] = {
	{"ds1200"},
	{}
  };

  MODULE_DEVICE_TABLE(i2c, ds1200_id);

  /* This is the driver that will be inserted */
  static struct i2c_driver ds1200_driver = {
	.driver = {
		   .name = "ds1200",
		   },
	.probe = ds1200_probe,
	.id_table = ds1200_id,
  };

  static int __init ds1200_init(void)
  {
	return i2c_add_driver(&ds1200_driver);
  }

  static void __exit ds1200_exit(void)
  {
	i2c_del_driver(&ds1200_driver);
  }


```
### Sysfs 鎺ュ彛


鍦ㄦ帰娴嬭姱鐗囨椂锛岄┍鍔ㄤ細璇嗗埆鏀寔鍝簺 PMBus 瀵勫瓨鍣紝骞舵嵁姝ょ‘瀹氬彲鐢ㄧ殑浼犳劅鍣ㄣ€傚睘鎬ф枃浠朵粎鍦ㄨ姱鐗囨敮鎸佺浉搴斾紶鎰熷櫒鏃舵墠瀛樺湪銆傛彁渚涙爣绛剧敤浜庡憡鐭ョ敤鎴锋煇涓?sysfs 鎺ュ彛鎵€瀵瑰簲鐨勪紶鎰熷櫒銆?
鏀寔浠ヤ笅灞炴€с€傞檺鍒剁被灞炴€т负鍙鍐欙紱鍏朵綑灞炴€у潎涓哄彧璇汇€?
======================= ========================================================
inX_input		娴嬪緱鐢靛帇銆傛潵鑷?READ_VIN 鎴?READ_VOUT 瀵勫瓨鍣ㄣ€?inX_min			鏈€灏忕數鍘嬨€?			鏉ヨ嚜 VIN_UV_WARN_LIMIT 鎴?VOUT_UV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?inX_max			鏈€澶х數鍘嬨€?			鏉ヨ嚜 VIN_OV_WARN_LIMIT 鎴?VOUT_OV_WARN_LIMIT 瀵勫瓨鍣ㄣ€?inX_lcrit		涓寸晫鏈€灏忕數鍘嬨€?			鏉ヨ嚜 VIN_UV_FAULT_LIMIT 鎴?VOUT_UV_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?inX_crit		涓寸晫鏈€澶х數鍘嬨€?			鏉ヨ嚜 VIN_OV_FAULT_LIMIT 鎴?VOUT_OV_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?inX_min_alarm		鐢靛帇杩囦綆鍛婅銆傛潵鑷?VOLTAGE_UV_WARNING 鐘舵€併€?inX_max_alarm		鐢靛帇杩囬珮鍛婅銆傛潵鑷?VOLTAGE_OV_WARNING 鐘舵€併€?inX_lcrit_alarm		鐢靛帇涓寸晫杩囦綆鍛婅銆?			鏉ヨ嚜 VOLTAGE_UV_FAULT 鐘舵€併€?inX_crit_alarm		鐢靛帇涓寸晫杩囬珮鍛婅銆?			鏉ヨ嚜 VOLTAGE_OV_FAULT 鐘舵€併€?inX_label		"vin"銆?vcap" 鎴?"voutY"
inX_rated_min		棰濆畾鏈€灏忕數鍘嬨€?			鏉ヨ嚜 MFR_VIN_MIN 鎴?MFR_VOUT_MIN 瀵勫瓨鍣ㄣ€?inX_rated_max		棰濆畾鏈€澶х數鍘嬨€?			鏉ヨ嚜 MFR_VIN_MAX 鎴?MFR_VOUT_MAX 瀵勫瓨鍣ㄣ€?
currX_input		娴嬪緱鐢垫祦銆傛潵鑷?READ_IIN 鎴?READ_IOUT 瀵勫瓨鍣ㄣ€?currX_max		鏈€澶х數娴併€?			鏉ヨ嚜 IIN_OC_WARN_LIMIT 鎴?IOUT_OC_WARN_LIMIT 瀵勫瓨鍣ㄣ€?currX_lcrit		涓寸晫鏈€灏忚緭鍑虹數娴併€?			鏉ヨ嚜 IOUT_UC_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?currX_crit		涓寸晫鏈€澶х數娴併€?			鏉ヨ嚜 IIN_OC_FAULT_LIMIT 鎴?IOUT_OC_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?currX_alarm		鐢垫祦杩囬珮鍛婅銆?			鏉ヨ嚜 IIN_OC_WARNING 鎴?IOUT_OC_WARNING 鐘舵€併€?currX_max_alarm		鐢垫祦杩囬珮鍛婅銆?			鏉ヨ嚜 IIN_OC_WARN_LIMIT 鎴?IOUT_OC_WARN_LIMIT 鐘舵€併€?currX_lcrit_alarm	杈撳嚭鐢垫祦涓寸晫杩囦綆鍛婅銆?			鏉ヨ嚜 IOUT_UC_FAULT 鐘舵€併€?currX_crit_alarm	鐢垫祦涓寸晫杩囬珮鍛婅銆?			鏉ヨ嚜 IIN_OC_FAULT 鎴?IOUT_OC_FAULT 鐘舵€併€?currX_label		"iin"銆?iinY"銆?iinY.Z"銆?ioutY" 鎴?"ioutY.Z"锛?			鍏朵腑 Y 琛ㄧず椤靛彿锛孼 琛ㄧず鐩镐綅銆?currX_rated_max		棰濆畾鏈€澶х數娴併€?			鏉ヨ嚜 MFR_IIN_MAX 鎴?MFR_IOUT_MAX 瀵勫瓨鍣ㄣ€?
powerX_input		娴嬪緱鍔熺巼銆傛潵鑷?READ_PIN 鎴?READ_POUT 瀵勫瓨鍣ㄣ€?powerX_cap		杈撳嚭鍔熺巼涓婇檺銆傛潵鑷?POUT_MAX 瀵勫瓨鍣ㄣ€?powerX_max		鍔熺巼闄愬埗銆?			鏉ヨ嚜 PIN_OP_WARN_LIMIT 鎴?POUT_OP_WARN_LIMIT 瀵勫瓨鍣ㄣ€?powerX_crit		涓寸晫杈撳嚭鍔熺巼闄愬埗銆?			鏉ヨ嚜 POUT_OP_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?powerX_alarm		鍔熺巼杩囬珮鍛婅銆?			鏉ヨ嚜 PIN_OP_WARNING 鎴?POUT_OP_WARNING 鐘舵€併€?powerX_crit_alarm	杈撳嚭鍔熺巼涓寸晫杩囬珮鍛婅銆?			鏉ヨ嚜 POUT_OP_FAULT 鐘舵€併€?powerX_label		"pin"銆?pinY"銆?pinY.Z"銆?poutY" 鎴?"poutY.Z"锛?			鍏朵腑 Y 琛ㄧず椤靛彿锛孼 琛ㄧず鐩镐綅銆?powerX_rated_max	棰濆畾鏈€澶у姛鐜囥€?			鏉ヨ嚜 MFR_PIN_MAX 鎴?MFR_POUT_MAX 瀵勫瓨鍣ㄣ€?
tempX_input		娴嬪緱娓╁害銆?			鏉ヨ嚜 READ_TEMPERATURE_X 瀵勫瓨鍣ㄣ€?tempX_min		鏈€灏忔俯搴︺€傛潵鑷?UT_WARN_LIMIT 瀵勫瓨鍣ㄣ€?tempX_max		鏈€澶ф俯搴︺€傛潵鑷?OT_WARN_LIMIT 瀵勫瓨鍣ㄣ€?tempX_lcrit		涓寸晫浣庢俯銆傛潵鑷?UT_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?tempX_crit		涓寸晫楂樻俯銆傛潵鑷?OT_FAULT_LIMIT 瀵勫瓨鍣ㄣ€?tempX_min_alarm		鑺墖娓╁害杩囦綆鍛婅銆傚綋 TEMP_UT_WARNING 鐘舵€佺疆浣嶆椂锛?			閫氳繃灏?READ_TEMPERATURE_X 涓?UT_WARN_LIMIT 姣旇緝鏉ヨ缃€?tempX_max_alarm		鑺墖娓╁害杩囬珮鍛婅銆傚綋 TEMP_OT_WARNING 鐘舵€佺疆浣嶆椂锛?			閫氳繃灏?READ_TEMPERATURE_X 涓?OT_WARN_LIMIT 姣旇緝鏉ヨ缃€?tempX_lcrit_alarm	鑺墖娓╁害涓寸晫杩囦綆鍛婅銆傚綋 TEMP_UT_FAULT 鐘舵€佺疆浣嶆椂锛?			閫氳繃灏?READ_TEMPERATURE_X 涓?UT_FAULT_LIMIT 姣旇緝鏉ヨ缃€?tempX_crit_alarm	鑺墖娓╁害涓寸晫杩囬珮鍛婅銆傚綋 TEMP_OT_FAULT 鐘舵€佺疆浣嶆椂锛?			閫氳繃灏?READ_TEMPERATURE_X 涓?OT_FAULT_LIMIT 姣旇緝鏉ヨ缃€?tempX_rated_min		棰濆畾鏈€灏忔俯搴︺€?			鏉ヨ嚜 MFR_TAMBIENT_MIN 瀵勫瓨鍣ㄣ€?tempX_rated_max		棰濆畾鏈€澶ф俯搴︺€?			鏉ヨ嚜 MFR_TAMBIENT_MAX銆丮FR_MAX_TEMP_1銆丮FR_MAX_TEMP_2 鎴?MFR_MAX_TEMP_3 瀵勫瓨鍣ㄣ€?======================= ========================================================
