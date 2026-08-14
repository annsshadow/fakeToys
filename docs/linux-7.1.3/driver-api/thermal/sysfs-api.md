## Generic Thermal Sysfs driver How To锛堥€氱敤 Thermal Sysfs 椹卞姩浣跨敤鎸囧崡锛?

Written by Sujith Thomas <sujith.thomas@intel.com>, Zhang Rui <rui.zhang@intel.com>

Copyright (c)  2008 Intel Corporation


## 0. 绠€浠嬶紙Introduction锛?

閫氱敤鐨?thermal sysfs 鎻愪緵浜嗕竴缁勬帴鍙ｏ紝渚?thermal zone 璁惧锛堜紶鎰熷櫒锛夊拰 thermal cooling 璁惧锛堥鎵囥€佸鐞嗗櫒鈥︹€︼級鍚戠儹绠＄悊鏂规娉ㄥ唽骞舵垚涓哄叾涓殑涓€閮ㄥ垎銆?
鏈娇鐢ㄦ寚鍗椾晶閲嶄簬浣挎柊鐨?thermal zone 鍜?cooling 璁惧鑳藉鍙備笌鐑鐞嗐€?璇ユ柟妗堟槸涓庡钩鍙版棤鍏崇殑锛屼换浣曠被鍨嬬殑 thermal zone 璁惧鍜?cooling 璁惧閮藉簲褰撹兘澶熷埄鐢ㄨ繖涓€鍩虹璁炬柦銆?
thermal sysfs 椹卞姩鐨勪富瑕佷换鍔℃槸灏?thermal zone 灞炴€т互鍙?cooling 璁惧灞炴€ф毚闇茬粰鐢ㄦ埛绌洪棿銆?涓€涓櫤鑳界殑鐑鐞嗗簲鐢ㄧ▼搴忓彲浠ユ牴鎹潵鑷?thermal zone 灞炴€э紙褰撳墠娓╁害鍜岃Е鍙戠偣娓╁害锛夌殑杈撳叆鏉ュ仛鍑哄喅绛栵紝骞跺鐩稿簲鐨勮澶囪繘琛岃妭娴侊紙throttle锛夈€?
- `[0-*]`		琛ㄧず浠讳綍浠?0 寮€濮嬬殑姝ｆ暣鏁?- `[1-*]`		琛ㄧず浠讳綍浠?1 寮€濮嬬殑姝ｆ暣鏁?
## 1. thermal sysfs 椹卞姩鎺ュ彛鍑芥暟


### 1.1 thermal zone device interface锛坱hermal zone 璁惧鎺ュ彛锛?

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
### 1.2 thermal cooling device interface锛坱hermal cooling 璁惧鎺ュ彛锛?


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
### 1.4 Thermal Zone Parameters锛圱hermal Zone 鍙傛暟锛?

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
## 2. sysfs 灞炴€х粨鏋勶紙sysfs attributes structure锛?

==	================
RO	read only value
WO	write only value
RW	read/write value
==	================

Thermal sysfs 灞炴€у皢琛ㄧず鍦?/sys/class/thermal 涔嬩笅銆?濡傛灉 hwmon 琚紪璇戣繘鍐呮牳鎴栦綔涓烘ā鍧楁瀯寤猴紝hwmon sysfs 鎺ュ彛鎵╁睍涔熷彲鍦?/sys/class/hwmon 涓嬩娇鐢ㄣ€?
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
鎺ヤ笅鏉ョ殑涓や釜鍔ㄦ€佸睘鎬ф槸鎴愬鍒涘缓/绉婚櫎鐨勩€傚畠浠〃绀?thermal zone 涓庡叾鍏宠仈鐨?cooling 璁惧涔嬮棿鐨勫叧绯汇€?
```
  /sys/class/thermal/thermal_zone[0-*]:
    |---cdev[0-*]:		[0-*]th cooling device in current thermal zone
    |---cdev[0-*]_trip_point:	Trip point that cdev[0-*] is associated with
    |---cdev[0-*]_weight:       Influence of the cooling device in
				this thermal zone

```
闄や簡 thermal zone 璁惧 sysfs 鎺ュ彛鍜?cooling 璁惧 sysfs 鎺ュ彛澶栵紝閫氱敤鐨?thermal 椹卞姩杩樹細涓烘瘡绉?_type_ 鐨?thermal zone 璁惧鍒涘缓涓€涓?hwmon sysfs 鎺ュ彛銆備緥濡傦紝閫氱敤鐨?thermal 椹卞姩浼氭敞鍐屼竴涓?hwmon 绫昏澶囷紝骞朵负鎵€鏈夊凡娉ㄥ唽鐨?ACPI thermal zone 鏋勫缓鍏宠仈鐨?hwmon sysfs 鎺ュ彛銆?
璇烽槄璇?Documentation/ABI/testing/sysfs-class-thermal 浠ヤ簡瑙?thermal zone 鍜?cooling 璁惧灞炴€х殑璇︾粏淇℃伅銆?
```
  /sys/class/hwmon/hwmon[0-*]:
    |---name:			The type of the thermal zone devices
    |---temp[1-*]_input:	The current temperature of thermal zone [1-*]
    |---temp[1-*]_critical:	The critical trip point of thermal zone [1-*]

```
璇烽槄璇?Documentation/hwmon/sysfs-interface.rst 浠ヨ幏鍙栨洿澶氫俊鎭€?
## 3. 涓€涓畝鍗曠殑瀹炵幇锛圓 simple implementation锛?

ACPI thermal zone 鍙兘鏀寔澶氫釜瑙﹀彂鐐癸紝濡?critical銆乭ot銆乸assive銆乤ctive銆傚鏋滀竴涓?ACPI thermal zone 鍚屾椂鏀寔 critical銆乸assive銆乤ctive[^0^] 鍜?active[^1^]锛屽畠鍙互灏嗚嚜宸辨敞鍐屼负涓€涓甫 4 涓Е鍙戠偣鐨?thermal_zone_device锛坱hermal_zone1锛夈€傚畠鏈変竴涓鐞嗗櫒鍜屼竴鍙伴鎵囷紝涓よ€呴兘娉ㄥ唽涓?thermal_cooling_device銆備袱鑰呭湪鍐峰嵈璇?thermal zone 鏂归潰琚涓哄叿鏈夌浉鍚岀殑鏈夋晥鎬с€?
濡傛灉澶勭悊鍣ㄥ垪鍦?_PSL 鏂规硶涓紝涓旈鎵囧垪鍦?_AL0 鏂规硶涓?```
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
## 4. Export Symbol APIs锛堝鍑虹鍙?API锛?

### 4.1. get_tz_trend


姝ゅ嚱鏁拌繑鍥?thermal zone 鐨勮秼鍔匡紝鍗宠 thermal zone 娓╁害鐨勫彉鍖栫巼銆傜悊鎯虫儏鍐典笅锛宼hermal 浼犳劅鍣ㄩ┍鍔ㄥ簲褰撳疄鐜拌鍥炶皟銆傚鏋滄病鏈夛紝thermal 妗嗘灦浼氶€氳繃姣旇緝鍏堝墠鍜屽綋鍓嶇殑娓╁害鍊兼潵璁＄畻瓒嬪娍銆?
### 4.2. thermal_cdev_update


姝ゅ嚱鏁板厖褰撲徊瑁佽€呮潵璁剧疆 cooling 璁惧鐨勭姸鎬併€傚鏋滃彲鑳斤紝瀹冧細灏?cooling 璁惧璁剧疆涓烘渶娣辩殑鍐峰嵈鐘舵€併€?
## 5. 鍏抽敭浜嬩欢锛圕ritical Events锛?

褰撳彂鐢?critical 瑙﹀彂娓╁害瓒婄晫浜嬩欢鏃讹紝thermal 妗嗘灦灏嗚Е鍙戠‖浠朵繚鎶ゆ€х殑鏂數锛堝叧鏈猴級鎴栭噸鍚紝鍏蜂綋鍙栧喅浜庨厤缃€?
棣栧厛锛屽唴鏍镐細灏濊瘯鏈夊簭鍦版柇鐢垫垨閲嶅惎锛屼絾浼氭帴鍙椾竴涓欢杩燂紝瓒呰繃璇ュ欢杩熷悗瀹冨皢鍒嗗埆缁х画杩涜寮哄埗鏂數鎴栭噸鍚€傚鏋滆繖澶辫触锛屽皢浣滀负鏈€鍚庢墜娈佃皟鐢?`emergency_restart()`銆?
搴斿綋浠旂粏鍒嗘瀽璇ュ欢杩燂紝浠ヤ究涓烘湁搴忔柇鐢垫垨閲嶅惎鐣欏嚭鍏呰冻鐨勬椂闂淬€?
濡傛灉璇ュ欢杩熻璁剧疆涓?0锛屽垯灏嗕笉鏀寔绱ф€ュ姩浣溿€傚洜姝わ紝瑕佽Е鍙戠揣鎬ュ姩浣滐紝蹇呴』缁忚繃浠旂粏鍒嗘瀽鐨勯潪闆舵鏁存暟鍊笺€?