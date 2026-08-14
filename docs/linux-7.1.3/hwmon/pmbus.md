## 内核 pmbus 驱动


支持的芯片：

  - Flex BMR310, BMR453, BMR454, BMR456, BMR457, BMR458, BMR480,
    BMR490, BMR491, BMR492

    前缀：'bmr310', 'bmr453', 'bmr454', 'bmr456', 'bmr457', 'bmr458', 'bmr480',
    'bmr490', 'bmr491', 'bmr492'

    扫描的地址：-

    数据手册：

	https://flexpowermodules.com/products


  - ON Semiconductor ADP4000, NCP4200, NCP4208

    前缀：'adp4000', 'ncp4200', 'ncp4208'

    扫描的地址：-

    数据手册：

	https://www.onsemi.com/pub_link/Collateral/ADP4000-D.PDF

	https://www.onsemi.com/pub_link/Collateral/NCP4200-D.PDF

	https://www.onsemi.com/pub_link/Collateral/JUNE%202009-%20REV.%200.PDF

  - Lineage Power

    前缀：'mdt040', 'pdt003', 'pdt006', 'pdt012', 'udt020'

    扫描的地址：-

    数据手册：

	http://www.lineagepower.com/oem/pdf/PDT003A0X.pdf

	http://www.lineagepower.com/oem/pdf/PDT006A0X.pdf

	http://www.lineagepower.com/oem/pdf/PDT012A0X.pdf

	http://www.lineagepower.com/oem/pdf/UDT020A0X.pdf

	http://www.lineagepower.com/oem/pdf/MDT040A0X.pdf

  - Texas Instruments TPS40400, TPS544B20, TPS544B25, TPS544C20, TPS544C25

    前缀：'tps40400', 'tps544b20', 'tps544b25', 'tps544c20', 'tps544c25'

    扫描的地址：-

    数据手册：

	https://www.ti.com/lit/gpn/tps40400

	https://www.ti.com/lit/gpn/tps544b20

	https://www.ti.com/lit/gpn/tps544b25

	https://www.ti.com/lit/gpn/tps544c20

	https://www.ti.com/lit/gpn/tps544c25

  - Maxim MAX20796

    前缀：'max20796'

    扫描的地址：-

    数据手册：

	https://www.analog.com/media/en/technical-documentation/data-sheets/MAX20796.pdf

  - Generic PMBus devices

    前缀：'pmbus'

    扫描的地址：-

    数据手册：n.a.


作者：Guenter Roeck <linux@roeck-us.net>


### 描述


该驱动支持多种符合 PMBus 规范设备的硬件监控。它支持设备所能提供的电压、电流、功率与温度传感器。

每个被监控的通道都有各自的高、低限制，以及一个临界限制。

风扇支持将在本驱动的后续版本中加入。


### 使用说明


该驱动不会主动探测 PMBus 设备，因为没有可以安全用于识别芯片的寄存器（MFG_ID 寄存器并非所有芯片都支持），而且 PMBus 设备也没有明确界定的地址范围。你必须显式地实例化这些设备。

示例：以下命令将为地址 0x60 处的 LTC2978 加载该驱动
```

	$ modprobe pmbus
	$ echo ltc2978 0x60 > /sys/bus/i2c/devices/i2c-1/new_device


```
### 平台数据支持


可通过在新的芯片专用驱动文件中定义芯片参数，来添加对更多 PMBus 芯片的支持。例如，用于添加以下芯片支持（未经测试）的代码：
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
### Sysfs 接口


在探测芯片时，驱动会识别支持哪些 PMBus 寄存器，并据此确定可用的传感器。属性文件仅在芯片支持相应传感器时才存在。提供标签用于告知用户某个 sysfs 接口所对应的传感器。

支持以下属性。限制类属性为可读写；其余属性均为只读。

======================= ========================================================
inX_input		测得电压。来自 READ_VIN 或 READ_VOUT 寄存器。
inX_min			最小电压。
			来自 VIN_UV_WARN_LIMIT 或 VOUT_UV_WARN_LIMIT 寄存器。
inX_max			最大电压。
			来自 VIN_OV_WARN_LIMIT 或 VOUT_OV_WARN_LIMIT 寄存器。
inX_lcrit		临界最小电压。
			来自 VIN_UV_FAULT_LIMIT 或 VOUT_UV_FAULT_LIMIT 寄存器。
inX_crit		临界最大电压。
			来自 VIN_OV_FAULT_LIMIT 或 VOUT_OV_FAULT_LIMIT 寄存器。
inX_min_alarm		电压过低告警。来自 VOLTAGE_UV_WARNING 状态。
inX_max_alarm		电压过高告警。来自 VOLTAGE_OV_WARNING 状态。
inX_lcrit_alarm		电压临界过低告警。
			来自 VOLTAGE_UV_FAULT 状态。
inX_crit_alarm		电压临界过高告警。
			来自 VOLTAGE_OV_FAULT 状态。
inX_label		"vin"、"vcap" 或 "voutY"
inX_rated_min		额定最小电压。
			来自 MFR_VIN_MIN 或 MFR_VOUT_MIN 寄存器。
inX_rated_max		额定最大电压。
			来自 MFR_VIN_MAX 或 MFR_VOUT_MAX 寄存器。

currX_input		测得电流。来自 READ_IIN 或 READ_IOUT 寄存器。
currX_max		最大电流。
			来自 IIN_OC_WARN_LIMIT 或 IOUT_OC_WARN_LIMIT 寄存器。
currX_lcrit		临界最小输出电流。
			来自 IOUT_UC_FAULT_LIMIT 寄存器。
currX_crit		临界最大电流。
			来自 IIN_OC_FAULT_LIMIT 或 IOUT_OC_FAULT_LIMIT 寄存器。
currX_alarm		电流过高告警。
			来自 IIN_OC_WARNING 或 IOUT_OC_WARNING 状态。
currX_max_alarm		电流过高告警。
			来自 IIN_OC_WARN_LIMIT 或 IOUT_OC_WARN_LIMIT 状态。
currX_lcrit_alarm	输出电流临界过低告警。
			来自 IOUT_UC_FAULT 状态。
currX_crit_alarm	电流临界过高告警。
			来自 IIN_OC_FAULT 或 IOUT_OC_FAULT 状态。
currX_label		"iin"、"iinY"、"iinY.Z"、"ioutY" 或 "ioutY.Z"，
			其中 Y 表示页号，Z 表示相位。
currX_rated_max		额定最大电流。
			来自 MFR_IIN_MAX 或 MFR_IOUT_MAX 寄存器。

powerX_input		测得功率。来自 READ_PIN 或 READ_POUT 寄存器。
powerX_cap		输出功率上限。来自 POUT_MAX 寄存器。
powerX_max		功率限制。
			来自 PIN_OP_WARN_LIMIT 或 POUT_OP_WARN_LIMIT 寄存器。
powerX_crit		临界输出功率限制。
			来自 POUT_OP_FAULT_LIMIT 寄存器。
powerX_alarm		功率过高告警。
			来自 PIN_OP_WARNING 或 POUT_OP_WARNING 状态。
powerX_crit_alarm	输出功率临界过高告警。
			来自 POUT_OP_FAULT 状态。
powerX_label		"pin"、"pinY"、"pinY.Z"、"poutY" 或 "poutY.Z"，
			其中 Y 表示页号，Z 表示相位。
powerX_rated_max	额定最大功率。
			来自 MFR_PIN_MAX 或 MFR_POUT_MAX 寄存器。

tempX_input		测得温度。
			来自 READ_TEMPERATURE_X 寄存器。
tempX_min		最小温度。来自 UT_WARN_LIMIT 寄存器。
tempX_max		最大温度。来自 OT_WARN_LIMIT 寄存器。
tempX_lcrit		临界低温。来自 UT_FAULT_LIMIT 寄存器。
tempX_crit		临界高温。来自 OT_FAULT_LIMIT 寄存器。
tempX_min_alarm		芯片温度过低告警。当 TEMP_UT_WARNING 状态置位时，
			通过将 READ_TEMPERATURE_X 与 UT_WARN_LIMIT 比较来设置。
tempX_max_alarm		芯片温度过高告警。当 TEMP_OT_WARNING 状态置位时，
			通过将 READ_TEMPERATURE_X 与 OT_WARN_LIMIT 比较来设置。
tempX_lcrit_alarm	芯片温度临界过低告警。当 TEMP_UT_FAULT 状态置位时，
			通过将 READ_TEMPERATURE_X 与 UT_FAULT_LIMIT 比较来设置。
tempX_crit_alarm	芯片温度临界过高告警。当 TEMP_OT_FAULT 状态置位时，
			通过将 READ_TEMPERATURE_X 与 OT_FAULT_LIMIT 比较来设置。
tempX_rated_min		额定最小温度。
			来自 MFR_TAMBIENT_MIN 寄存器。
tempX_rated_max		额定最大温度。
			来自 MFR_TAMBIENT_MAX、MFR_MAX_TEMP_1、MFR_MAX_TEMP_2 或 MFR_MAX_TEMP_3 寄存器。
======================= ========================================================
