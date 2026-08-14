## 内核驱动 bel-pfe


支持的芯片：

  - BEL PFE1100

    Prefixes: 'pfe1100'

    Addresses scanned: -

    Datasheet: https://www.belfuse.com/resources/datasheets/powersolutions/ds-bps-pfe1100-12-054xa.pdf

  - BEL PFE3000

    Prefixes: 'pfe3000'

    Addresses scanned: -

    Datasheet: https://www.belfuse.com/resources/datasheets/powersolutions/ds-bps-pfe3000-series.pdf

作者：Tao Ren <rentao.bupt@gmail.com>


### 描述


该驱动支持以下支持 PMBus 协议的电源设备的硬件监控：

  - BEL PFE1100

    1100 瓦 AC 转 DC 功率因数校正（PFC）电源。PMBus 通信手册未公开提供。

  - BEL PFE3000

    3000 瓦 AC/DC 功率因数校正（PFC）与 DC-DC 电源。PMBus 通信手册未公开提供。

该驱动是核心 PMBus 驱动的客户端驱动。有关 PMBus 客户端驱动的详情，请参阅 Documentation/hwmon/pmbus.rst。


### 使用说明


该驱动不会自动检测设备。你需要显式实例化设备。详情请参阅 Documentation/i2c/instantiating-devices.rst。

示例：以下命令将为地址 0x20 处的 PFE3000 加载驱动
```

	$ modprobe bel-pfe
	$ echo pfe3000 0x20 > /sys/bus/i2c/devices/i2c-1/new_device


```

### 平台数据支持


该驱动支持标准的 PMBus 驱动平台数据。


### Sysfs 条目


======================= =======================================================
curr1_label		"iin"
curr1_input		测量的输入电流
curr1_max               输入电流最大值
curr1_max_alarm         输入电流最大报警

curr[2-3]_label		"iout[1-2]"
curr[2-3]_input		测量的输出电流
curr[2-3]_max           输出电流最大值
curr[2-3]_max_alarm     输出电流最大报警

fan[1-2]_input          风扇 1 与 2 的转速（RPM）
fan1_target             为两个风扇设置转速参考

in1_label		"vin"
in1_input		测量的输入电压
in1_crit		输入电压临界最大值
in1_crit_alarm		输入电压临界最大报警
in1_lcrit               输入电压临界最小值
in1_lcrit_alarm         输入电压临界最小报警
in1_max                 输入电压最大值
in1_max_alarm           输入电压最大报警

in2_label               "vcap"
in2_input               保持电容电压

in[3-8]_label		"vout[1-3,5-7]"
in[3-8]_input		测量的输出电压
in[3-4]_alarm           vout[1-2] 输出电压报警

power[1-2]_label	"pin[1-2]"
power[1-2]_input        测量的输入功率
power[1-2]_alarm	输入功率过高报警

power[3-4]_label	"pout[1-2]"
power[3-4]_input	测量的输出功率

temp[1-3]_input		测量的温度
temp[1-3]_alarm         温度报警
======================= =======================================================


    - curr3、fan2、vout[2-7]、vcap、pin2、pout2 与 temp3 属性仅存在于 PFE3000。
