## Linux I2C 浠庢満 EEPROM 鍚庣


浣滆€咃細Wolfram Sang <wsa@sang-engineering.com> in 2014-20

鏈悗绔湪杩炴帴鐨?I2C 鎬荤嚎涓婃ā鎷熶竴涓?EEPROM銆傚叾鍐呭瓨鍐呭
```

	/sys/bus/i2c/devices/<device-directory>/slave-eeprom

```
鍙敤鐨勭被鍨嬫湁锛?4c02銆?4c32銆?4c64 鍜?24c512銆備篃鏀寔鍙鍙樹綋銆?
瀹炰緥鍖栨墍闇€鐨勫悕绉板舰寮忎负 'slave-<type>[ro]'銆傜ず渚嬪涓嬶細

24c02锛岃/鍐欙紝鍦板潃 0x64锛?
  # echo slave-24c02 0x1064 > /sys/bus/i2c/devices/i2c-1/new_device

24c512锛屽彧璇伙紝鍦板潃 0x42锛?
  # echo slave-24c512ro 0x1042 > /sys/bus/i2c/devices/i2c-1/new_device

濡傛灉鍦ㄥ惎鍔ㄦ椂棰勫姞杞芥暟鎹紝涓斿悕涓?'firmware-name' 鐨勮澶囧睘鎬?
鍖呭惈涓€涓湁鏁堢殑鏂囦欢鍚嶏紙浠呴檺 DT 鎴?ACPI锛夈€?

鎴嚦 2015 骞达紝Linux 涓嶆敮鎸佸浜岃繘鍒?sysfs 鏂囦欢杩涜 poll锛屽洜姝ゅ綋鍙︿竴涓?
涓昏澶囨敼鍙樺唴瀹规椂涓嶄細鏈夐€氱煡銆?

