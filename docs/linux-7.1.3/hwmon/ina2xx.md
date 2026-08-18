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


INA219 鏄竴娆惧甫鏈?I2C 鎺ュ彛鐨勯珮绔數娴佸垎娴佷笌鍔熺巼鐩戣鍣ㄣ€侷NA219 鍚屾椂鐩戣鍒嗘祦鍘嬮檷鍜岀數婧愮數鍘嬶紝鍏锋湁鍙紪绋嬬殑杞崲鏃堕棿鍜屾护娉㈠姛鑳姐€?
INA220 鏄竴娆惧甫鏈?I2C 鎺ュ彛鐨勯珮杈规垨浣庤竟鐢垫祦鍒嗘祦涓庡姛鐜囩洃瑙嗗櫒銆侷NA220 鍚屾椂鐩戣鍒嗘祦鍘嬮檷鍜岀數婧愮數鍘嬨€?
INA226 鏄竴娆惧甫鏈?I2C 鎺ュ彛鐨勭數娴佸垎娴佷笌鍔熺巼鐩戣鍣ㄣ€侷NA226 鍚屾椂鐩戣鍒嗘祦鐢靛帇闄嶅拰鎬荤嚎鐢垫簮鐢靛帇銆?
INA230銆両NA231 鍜?INA234 鏄甫鏈?I2C 鎺ュ彛鐨勯珮杈规垨浣庤竟鐢垫祦鍒嗘祦涓庡姛鐜囩洃瑙嗗櫒銆傝繖浜涜姱鐗囧悓鏃剁洃瑙嗗垎娴佺數鍘嬮檷鍜屾€荤嚎鐢垫簮鐢靛帇銆?
INA260 鏄竴娆惧甫鏈夐泦鎴愬垎娴佺數闃荤殑楂樿竟鎴栦綆杈圭數娴佷笌鍔熺巼鐩戣鍣ㄣ€?
SY24655 鏄竴娆惧甫鏈?I2C 鎺ュ彛鐨勯珮杈瑰拰浣庤竟鐢垫祦鍒嗘祦涓庡姛鐜囩洃瑙嗗櫒銆係Y24655 鏀寔鍒嗘祦鍘嬮檷鍜岀數婧愮數鍘嬶紝鍏锋湁鍙紪绋嬬殑鏍″噯鍊煎拰杞崲鏃堕棿銆係Y24655 杩樺彲浠ヨ绠楀钩鍧囧姛鐜囷紝鐢ㄤ簬鑳介噺杞崲銆?
鍒嗘祦鐢甸樆鍊硷紙浠ュ井娆т负鍗曚綅锛夊彲鍦ㄧ紪璇戞椂閫氳繃 platform data 鎴?device tree 璁剧疆锛屼篃鍙湪杩愯鏃堕€氳繃 sysfs 涓殑 shunt_resistor 灞炴€ц缃€傚鏋滀娇鐢?device tree锛岃鍙傞槄 Documentation/devicetree/bindings/hwmon/ti,ina2xx.yaml 浜嗚В鐩稿叧缁戝畾銆?
姝ゅ锛宨na226 鏀寔 update_interval 灞炴€э紝璇﹁ Documentation/hwmon/sysfs-interface.rst銆傚湪鍐呴儴锛岃闂撮殧绛変簬鎬荤嚎鐢靛帇鍜屽垎娴佺數鍘嬭浆鎹㈡椂闂翠箣鍜屼箻浠ュ钩鍧囬€熺巼銆傛垜浠笉浼氭敼鍔ㄨ浆鎹㈡椂闂达紝鍙慨鏀瑰钩鍧囨鏁般€倁pdate_interval 鐨勪笅闄愪负 2 ms锛屼笂闄愪负 2253 ms銆傚疄闄呯紪绋嬬殑闂撮殧鍙兘浼氫笌鏈熸湜鍊兼湁鎵€鍋忓樊銆?
### General sysfs entries


======================= ===============================================
in0_input		Shunt voltage(mV) channel
in1_input		Bus voltage(mV) channel
curr1_input		Current(mA) measurement channel
power1_input		Power(uW) measurement channel
shunt_resistor		Shunt resistance(uOhm) channel (not for ina260)
======================= ===============================================

### Additional sysfs entries


浠ヤ笅鑺墖杩樻彁渚涢澶栫殑 sysfs 灞炴€э細

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


   - 鍦ㄩ厤缃?`power1_crit` 涔嬪墠鍏堥厤缃?`shunt_resistor`锛屽洜涓?power 鍊兼槸鍩轰簬鎵€璁剧疆鐨?`shunt_resistor` 璁＄畻寰楀嚭鐨勩€?   - 鐢变簬搴曞眰鐨勫瘎瀛樺櫒瀹炵幇锛屽悓涓€鏃跺埢鍙兘鏈変竴涓?`*crit` 璁剧疆鍙婂叾 `alarm` 澶勪簬娲诲姩鐘舵€併€傚啓鍏ユ煇涓?`*crit` 璁剧疆浼氭竻闄ゅ叾浠栫殑 `*crit` 璁剧疆鍜?alarm銆傚悜浠绘剰 `**crit` 璁剧疆鍐欏叆 0 浼氭竻闄ゆ墍鏈?`*crit` 璁剧疆鍜?alarm銆?