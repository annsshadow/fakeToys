## Generic Thermal Sysfs driver How To（通用 Thermal Sysfs 驱动使用指南）


Written by Sujith Thomas <sujith.thomas@intel.com>, Zhang Rui <rui.zhang@intel.com>

Copyright (c)  2008 Intel Corporation


## 0. 简介（Introduction）


通用的 thermal sysfs 提供了一组接口，供 thermal zone 设备（传感器）和 thermal cooling 设备（风扇、处理器……）向热管理方案注册并成为其中的一部分。

本使用指南侧重于使新的 thermal zone 和 cooling 设备能够参与热管理。
该方案是与平台无关的，任何类型的 thermal zone 设备和 cooling 设备都应当能够利用这一基础设施。

thermal sysfs 驱动的主要任务是将 thermal zone 属性以及 cooling 设备属性暴露给用户空间。
一个智能的热管理应用程序可以根据来自 thermal zone 属性（当前温度和触发点温度）的输入来做出决策，并对相应的设备进行节流（throttle）。

- `[0-*]`		表示任何从 0 开始的正整数
- `[1-*]`		表示任何从 1 开始的正整数

## 1. thermal sysfs 驱动接口函数


### 1.1 thermal zone device interface（thermal zone 设备接口）


```
	struct thermal_zone_device *
	thermal_zone_device_register_with_trips(const char *type,
					const struct thermal_trip *trips,
					int num_trips, void *devdata,
					const struct thermal_zone_device_ops *ops,
					const struct thermal_zone_params *tzp,
					unsigned int passive_delay,
					unsigned int polling_delay)

    This interface function adds a new thermal zone device (sensor) to the
    /sys/class/thermal folder as `thermal_zone[0-*]`. It tries to bind all the
    thermal cooling devices registered to it at the same time.

    type:
	the thermal zone type.
    trips:
	the table of trip points for this thermal zone.
    devdata:
	device private data
    ops:
	thermal zone device call-backs.

	.should_bind:
		check whether or not a given cooling device should be bound to
		a given trip point in this thermal zone.
	.get_temp:
		get the current temperature of the thermal zone.
	.set_trips:
		set the trip points window. Whenever the current temperature
		is updated, the trip points immediately below and above the
		current temperature are found.
	.change_mode:
		change the mode (enabled/disabled) of the thermal zone.
	.set_trip_temp:
		set the temperature of a given trip point.
	.get_crit_temp:
		get the critical temperature for this thermal zone.
	.set_emul_temp:
		set the emulation temperature which helps in debugging
		different threshold temperature points.
	.get_trend:
		get the trend of most recent zone temperature changes.
	.hot:
		hot trip point crossing handler.
	.critical:
		critical trip point crossing handler.
    tzp:
	thermal zone platform parameters.
    passive_delay:
	number of milliseconds to wait between polls when performing passive
	cooling.
    polling_delay:
	number of milliseconds to wait between polls when checking
	whether trip points have been crossed (0 for interrupt driven systems).

    ::

	void thermal_zone_device_unregister(struct thermal_zone_device *tz)

    This interface function removes the thermal zone device.
    It deletes the corresponding entry from /sys/class/thermal folder and
    unbinds all the thermal cooling devices it uses.

	::

	   struct thermal_zone_device
	   *thermal_zone_of_sensor_register(struct device *dev, int sensor_id,
				void *data,
				const struct thermal_zone_of_device_ops *ops)

	This interface adds a new sensor to a DT thermal zone.
	This function will search the list of thermal zones described in
	device tree and look for the zone that refer to the sensor device
	pointed by dev->of_node as temperature providers. For the zone
	pointing to the sensor node, the sensor will be added to the DT
	thermal zone device.

	The parameters for this interface are:

	dev:
			Device node of sensor containing valid node pointer in
			dev->of_node.
	sensor_id:
			a sensor identifier, in case the sensor IP has more
			than one sensors
	data:
			a private pointer (owned by the caller) that will be
			passed back, when a temperature reading is needed.
	ops:
			`struct thermal_zone_of_device_ops *`.

			==============  =======================================
			get_temp	a pointer to a function that reads the
					sensor temperature. This is mandatory
					callback provided by sensor driver.
			set_trips	a pointer to a function that sets a
					temperature window. When this window is
					left the driver must inform the thermal
					core via thermal_zone_device_update.
			get_trend 	a pointer to a function that reads the
					sensor temperature trend.
			set_emul_temp	a pointer to a function that sets
					sensor emulated temperature.
			==============  =======================================

	The thermal zone temperature is provided by the get_temp() function
	pointer of thermal_zone_of_device_ops. When called, it will
	have the private pointer @data back.

	It returns error pointer if fails otherwise valid thermal zone device
	handle. Caller should check the return handle with IS_ERR() for finding
	whether success or not.

	::

	    void thermal_zone_of_sensor_unregister(struct device *dev,
						   struct thermal_zone_device *tzd)

	This interface unregisters a sensor from a DT thermal zone which was
	successfully added by interface thermal_zone_of_sensor_register().
	This function removes the sensor callbacks and private data from the
	thermal zone device registered with thermal_zone_of_sensor_register()
	interface. It will also silent the zone by remove the .get_temp() and
	get_trend() thermal zone device callbacks.

	::

	  struct thermal_zone_device
	  *devm_thermal_zone_of_sensor_register(struct device *dev,
				int sensor_id,
				void *data,
				const struct thermal_zone_of_device_ops *ops)

	This interface is resource managed version of
	thermal_zone_of_sensor_register().

	All details of thermal_zone_of_sensor_register() described in
	section 1.1.3 is applicable here.

	The benefit of using this interface to register sensor is that it
	is not require to explicitly call thermal_zone_of_sensor_unregister()
	in error path or during driver unbinding as this is done by driver
	resource manager.

	::

		void devm_thermal_zone_of_sensor_unregister(struct device *dev,
						struct thermal_zone_device *tzd)

	This interface is resource managed version of
	thermal_zone_of_sensor_unregister().
	All details of thermal_zone_of_sensor_unregister() described in
	section 1.1.4 is applicable here.
	Normally this function will not need to be called and the resource
	management code will ensure that the resource is freed.

	::

		int thermal_zone_get_slope(struct thermal_zone_device *tz)

	This interface is used to read the slope attribute value
	for the thermal zone device, which might be useful for platform
	drivers for temperature calculations.

	::

		int thermal_zone_get_offset(struct thermal_zone_device *tz)

	This interface is used to read the offset attribute value
	for the thermal zone device, which might be useful for platform
	drivers for temperature calculations.

```
### 1.2 thermal cooling device interface（thermal cooling 设备接口）



```
	struct thermal_cooling_device
	*thermal_cooling_device_register(char *name,
			void *devdata, struct thermal_cooling_device_ops *)

    This interface function adds a new thermal cooling device (fan/processor/...)
    to /sys/class/thermal/ folder as `cooling_device[0-*]`. It tries to bind itself
    to all the thermal zone devices registered at the same time.

    name:
	the cooling device name.
    devdata:
	device private data.
    ops:
	thermal cooling devices call-backs.

	.get_max_state:
		get the Maximum throttle state of the cooling device.
	.get_cur_state:
		get the Currently requested throttle state of the
		cooling device.
	.set_cur_state:
		set the Current throttle state of the cooling device.

    ::

	void thermal_cooling_device_unregister(struct thermal_cooling_device *cdev)

    This interface function removes the thermal cooling device.
    It deletes the corresponding entry from /sys/class/thermal folder and
    unbinds itself from all the thermal zone devices using it.

```
### 1.4 Thermal Zone Parameters（Thermal Zone 参数）


```
	struct thermal_zone_params

    This structure defines the platform level parameters for a thermal zone.
    This data, for each thermal zone should come from the platform layer.
    This is an optional feature where some platforms can choose not to
    provide this data.

    .governor_name:
	       Name of the thermal governor used for this zone
    .no_hwmon:
	       a boolean to indicate if the thermal to hwmon sysfs interface
	       is required. when no_hwmon == false, a hwmon sysfs interface
	       will be created. when no_hwmon == true, nothing will be done.
	       In case the thermal_zone_params is NULL, the hwmon interface
	       will be created (for backward compatibility).

```
## 2. sysfs 属性结构（sysfs attributes structure）


==	================
RO	read only value
WO	write only value
RW	read/write value
==	================

Thermal sysfs 属性将表示在 /sys/class/thermal 之下。
如果 hwmon 被编译进内核或作为模块构建，hwmon sysfs 接口扩展也可在 /sys/class/hwmon 下使用。

```
  /sys/class/thermal/thermal_zone[0-*]:
    |---type:			Type of the thermal zone
    |---temp:			Current temperature
    |---mode:			Working mode of the thermal zone
    |---policy:			Thermal governor used for this zone
    |---available_policies:	Available thermal governors for this zone
    |---trip_point_[0-*]_temp:	Trip point temperature
    |---trip_point_[0-*]_type:	Trip point type
    |---trip_point_[0-*]_hyst:	Hysteresis value for this trip point
    |---emul_temp:		Emulated temperature set node
    |---sustainable_power:      Sustainable dissipatable power
    |---k_po:                   Proportional term during temperature overshoot
    |---k_pu:                   Proportional term during temperature undershoot
    |---k_i:                    PID's integral term in the power allocator gov
    |---k_d:                    PID's derivative term in the power allocator
    |---integral_cutoff:        Offset above which errors are accumulated
    |---slope:                  Slope constant applied as linear extrapolation
    |---offset:                 Offset constant applied as linear extrapolation

```
```
  /sys/class/thermal/cooling_device[0-*]:
    |---type:			Type of the cooling device(processor/fan/...)
    |---max_state:		Maximum cooling state of the cooling device
    |---cur_state:		Current cooling state of the cooling device
    |---stats:			Directory containing cooling device's statistics
    |---stats/reset:		Writing any value resets the statistics
    |---stats/time_in_state_ms:	Time (msec) spent in various cooling states
    |---stats/total_trans:	Total number of times cooling state is changed
    |---stats/trans_table:	Cooling state transition table


```
接下来的两个动态属性是成对创建/移除的。它们表示 thermal zone 与其关联的 cooling 设备之间的关系。

```
  /sys/class/thermal/thermal_zone[0-*]:
    |---cdev[0-*]:		[0-*]th cooling device in current thermal zone
    |---cdev[0-*]_trip_point:	Trip point that cdev[0-*] is associated with
    |---cdev[0-*]_weight:       Influence of the cooling device in
				this thermal zone

```
除了 thermal zone 设备 sysfs 接口和 cooling 设备 sysfs 接口外，通用的 thermal 驱动还会为每种 _type_ 的 thermal zone 设备创建一个 hwmon sysfs 接口。例如，通用的 thermal 驱动会注册一个 hwmon 类设备，并为所有已注册的 ACPI thermal zone 构建关联的 hwmon sysfs 接口。

请阅读 Documentation/ABI/testing/sysfs-class-thermal 以了解 thermal zone 和 cooling 设备属性的详细信息。

```
  /sys/class/hwmon/hwmon[0-*]:
    |---name:			The type of the thermal zone devices
    |---temp[1-*]_input:	The current temperature of thermal zone [1-*]
    |---temp[1-*]_critical:	The critical trip point of thermal zone [1-*]

```
请阅读 Documentation/hwmon/sysfs-interface.rst 以获取更多信息。

## 3. 一个简单的实现（A simple implementation）


ACPI thermal zone 可能支持多个触发点，如 critical、hot、passive、active。如果一个 ACPI thermal zone 同时支持 critical、passive、active[^0^] 和 active[^1^]，它可以将自己注册为一个带 4 个触发点的 thermal_zone_device（thermal_zone1）。它有一个处理器和一台风扇，两者都注册为 thermal_cooling_device。两者在冷却该 thermal zone 方面被认为具有相同的有效性。

如果处理器列在 _PSL 方法中，且风扇列在 _AL0 方法中
```
 /sys/class/thermal:
  |thermal_zone1:
    |---type:			acpitz
    |---temp:			37000
    |---mode:			enabled
    |---policy:			step_wise
    |---available_policies:	step_wise fair_share
    |---trip_point_0_temp:	100000
    |---trip_point_0_type:	critical
    |---trip_point_1_temp:	80000
    |---trip_point_1_type:	passive
    |---trip_point_2_temp:	70000
    |---trip_point_2_type:	active0
    |---trip_point_3_temp:	60000
    |---trip_point_3_type:	active1
    |---cdev0:			--->/sys/class/thermal/cooling_device0
    |---cdev0_trip_point:	1	/* cdev0 can be used for passive */
    |---cdev0_weight:           1024
    |---cdev1:			--->/sys/class/thermal/cooling_device3
    |---cdev1_trip_point:	2	/* cdev1 can be used for active[0]*/
    |---cdev1_weight:           1024

  |cooling_device0:
    |---type:			Processor
    |---max_state:		8
    |---cur_state:		0

  |cooling_device3:
    |---type:			Fan
    |---max_state:		2
    |---cur_state:		0

 /sys/class/hwmon:
  |hwmon0:
    |---name:			acpitz
    |---temp1_input:		37000
    |---temp1_crit:		100000

```
## 4. Export Symbol APIs（导出符号 API）


### 4.1. get_tz_trend


此函数返回 thermal zone 的趋势，即该 thermal zone 温度的变化率。理想情况下，thermal 传感器驱动应当实现该回调。如果没有，thermal 框架会通过比较先前和当前的温度值来计算趋势。

### 4.2. thermal_cdev_update


此函数充当仲裁者来设置 cooling 设备的状态。如果可能，它会将 cooling 设备设置为最深的冷却状态。

## 5. 关键事件（Critical Events）


当发生 critical 触发温度越界事件时，thermal 框架将触发硬件保护性的断电（关机）或重启，具体取决于配置。

首先，内核会尝试有序地断电或重启，但会接受一个延迟，超过该延迟后它将分别继续进行强制断电或重启。如果这失败，将作为最后手段调用 `emergency_restart()`。

应当仔细分析该延迟，以便为有序断电或重启留出充足的时间。

如果该延迟被设置为 0，则将不支持紧急动作。因此，要触发紧急动作，必须经过仔细分析的非零正整数值。
