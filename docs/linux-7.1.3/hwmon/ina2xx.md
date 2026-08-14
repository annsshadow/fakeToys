## Kernel driver ina2xx


Supported chips:

  - Texas Instruments INA219


    Prefix: 'ina219'
    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Texas Instruments INA220

    Prefix: 'ina220'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Texas Instruments INA226

    Prefix: 'ina226'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Texas Instruments INA230

    Prefix: 'ina230'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Texas Instruments INA231

    Prefix: 'ina231'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Texas Instruments INA260

    Prefix: 'ina260'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

  - Silergy SY24655

    Prefix: 'sy24655'

    Addresses: I2C 0x40 - 0x4f

    Datasheet: Publicly available at the Silergy website

	       https://us1.silergy.com/


  - Texas Instruments INA234

    Prefix: 'ina234'

    Addresses: I2C 0x40 - 0x43

    Datasheet: Publicly available at the Texas Instruments website

	       https://www.ti.com/

Author: Lothar Felten <lothar.felten@gmail.com>

### Description


INA219 是一款带有 I2C 接口的高端电流分流与功率监视器。INA219 同时监视分流压降和电源电压，具有可编程的转换时间和滤波功能。

INA220 是一款带有 I2C 接口的高边或低边电流分流与功率监视器。INA220 同时监视分流压降和电源电压。

INA226 是一款带有 I2C 接口的电流分流与功率监视器。INA226 同时监视分流电压降和总线电源电压。

INA230、INA231 和 INA234 是带有 I2C 接口的高边或低边电流分流与功率监视器。这些芯片同时监视分流电压降和总线电源电压。

INA260 是一款带有集成分流电阻的高边或低边电流与功率监视器。

SY24655 是一款带有 I2C 接口的高边和低边电流分流与功率监视器。SY24655 支持分流压降和电源电压，具有可编程的校准值和转换时间。SY24655 还可以计算平均功率，用于能量转换。

分流电阻值（以微欧为单位）可在编译时通过 platform data 或 device tree 设置，也可在运行时通过 sysfs 中的 shunt_resistor 属性设置。如果使用 device tree，请参阅 Documentation/devicetree/bindings/hwmon/ti,ina2xx.yaml 了解相关绑定。

此外，ina226 支持 update_interval 属性，详见 Documentation/hwmon/sysfs-interface.rst。在内部，该间隔等于总线电压和分流电压转换时间之和乘以平均速率。我们不会改动转换时间，只修改平均次数。update_interval 的下限为 2 ms，上限为 2253 ms。实际编程的间隔可能会与期望值有所偏差。

### General sysfs entries


======================= ===============================================
in0_input		Shunt voltage(mV) channel
in1_input		Bus voltage(mV) channel
curr1_input		Current(mA) measurement channel
power1_input		Power(uW) measurement channel
shunt_resistor		Shunt resistance(uOhm) channel (not for ina260)
======================= ===============================================

### Additional sysfs entries


以下芯片还提供额外的 sysfs 属性：

  - ina226
  - ina230
  - ina231
  - ina234
  - ina260
  - sy24655

======================= ====================================================
curr1_lcrit		Critical low current
curr1_crit		Critical high current
curr1_lcrit_alarm	Current critical low alarm
curr1_crit_alarm	Current critical high alarm
in0_lcrit		Critical low shunt voltage
in0_crit		Critical high shunt voltage
in0_lcrit_alarm		Shunt voltage critical low alarm
in0_crit_alarm		Shunt voltage critical high alarm
in1_lcrit		Critical low bus voltage
in1_crit		Critical high bus voltage
in1_lcrit_alarm		Bus voltage critical low alarm
in1_crit_alarm		Bus voltage critical high alarm
power1_crit		Critical high power
power1_crit_alarm	Power critical high alarm
update_interval		data conversion time; affects number of samples used
			to average results for shunt and bus voltages.
======================= ====================================================

### Sysfs entries for sy24655 only


======================= ====================================================
power1_average		average power from last reading to the present.
======================= ====================================================


   - 在配置 `power1_crit` 之前先配置 `shunt_resistor`，因为 power 值是基于所设置的 `shunt_resistor` 计算得出的。
   - 由于底层的寄存器实现，同一时刻只能有一个 `*crit` 设置及其 `alarm` 处于活动状态。写入某个 `*crit` 设置会清除其他的 `*crit` 设置和 alarm。向任意 `**crit` 设置写入 0 会清除所有 `*crit` 设置和 alarm。
