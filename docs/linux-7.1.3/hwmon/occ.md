## 内核驱动 occ-hwmon


支持芯片
  - POWER8
  - POWER9

Author: Eddie James <eajames@linux.ibm.com>

### 描述


本驱动支持对嵌入POWER 处理器上的片上控制器（OCC）进行硬件监控。OCC 是一个从
处理器和系统收集并聚合传感器数据的设备。OCC 既可以提供原始传感器数据，也可以在系统上
执行散热与电源管理
本驱动的 P8 版本I2C 的一个客户端驱动。如果在设备树的相应 I2C 总线节点下找"ibm,p8-occ-hwmon" 兼容设备，可以手动探测它
本驱动的 P9 版本是基FSI OCC 驱动的客户端驱动。它将由基于 FSI OCC 驱动自动
探测
### Sysfs 条目


支持以下属性。除非特别说明，所有属性都是只读的
OCC 传感ID 是一个整数，表示相对OCC 的传感器的唯一标识符。例如，系统中第三个
DIMM 插槽的温度传感器可能具有传感ID 7。设备驱动无法获取此映射，因此必须原样导传感ID
某些条目仅在某些 OCC 传感器版本下出现，或仅出现在系统中的某些 OCC 上。版本号不导给用户，但可以推断
temp[1-n]_label
	OCC 传感ID
[with temperature sensor version 1]

    temp[1-n]_input
			以千分度摄氏度测量的组件温度
[with temperature sensor version >= 2]

    temp[1-n]_type
			FRU（现场可更换单元）类			（由整数表示），表示此传感器所测量的组件    temp[1-n]_fault
			温度传感器故障布尔值；1 表示存在故障			0 表示不存在故障
    [with type == 3 (FRU type is VRM)]

	temp[1-n]_alarm
			VRM 温度告警布尔值；1 表示告警 表示无告警
    [else]

	temp[1-n]_input
			以千分度摄氏度测量的组件温度
freq[1-n]_label
			OCC 传感IDfreq[1-n]_input
			MHz 测量的组件频率power[1-n]_input
			组件最新测量的功率读数，单microwattspower[1-n]_average
			组件的平均功率，单位 microwattspower[1-n]_average_interval
			取功率平均值所经过的时间，单位微秒
[with power sensor version < 2]

    power[1-n]_label
			OCC 传感ID
[with power sensor version >= 2]

    power[1-n]_label
			OCC 传感ID + 功能 ID + 通道，形式为字符串，
			以下划线分隔，即 "0_15_1"。功ID 和通道都是
			整数，用于进一步标识功率传感器
[with power sensor version 0xa0]

    power[1-n]_label
			OCC 传感ID + 传感器类型，形式为字符串			以下划线分隔，即 "0_system"。传感器类型将是
			"system"proc"vdd" "vdn" 之一。对于此
			传感器版本，所有功率传感器OCC 传感ID 都相同
[仅在 "master" OCC 上出现；表示整个系统的功率；此类功率传感器只会有一个]

    power[1-n]_label
			"system"
    power[1-n]_input
			最新的系统输出功率，单microwatts    power[1-n]_cap
			当前系统功率上限，单microwatts    power[1-n]_cap_not_redundant
			无冗余电源时的系统功率上限，单位 microwatts    power[1-n]_cap_max
			OCC 可以强制执行的最大功率上限，单位 microwatts    power[1-n]_cap_min		OCC 可以强制执行的最小功率上限，单位
			microwatts    power[1-n]_cap_user		用户设置的功率上限，单位 microwatts			如果没有设置用户功率上限，此属性将返回 0。此属			是可读写的，但低于瓦特的任何精度写入都将被忽略，
			即请500900000 microwatts 的功率上限将导致
			一500 瓦的功率上限请求
    [with caps sensor version > 1]

	power[1-n]_cap_user_source
				指示用户功率上限是如何设置的。这是一				整数，映射到可以设置用户功率上限				系统或固件组件
以下 "extn" 传感器被导出，作OCC 提供不适合任何其他地方的数据的一种方式。这传感器的含义完全取决于其数据，无法静态定义
extn[1-n]_label
			ASCII ID OCC 传感IDextn[1-n]_flags
			这是一个单字节十六进制值。位 7 指示 label 属			的类型；1 表示传感ID 表示 ASCII ID。其他位保留extn[1-n]_input
			6 字节十六进制数据，含义由传感ID 定义