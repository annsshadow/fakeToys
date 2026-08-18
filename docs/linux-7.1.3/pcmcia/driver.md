## PCMCIA 椹卞姩


### sysfs


鏂扮殑 PCMCIA ID 鍙坊鍔犲埌璁惧椹卞姩鐨?pcmcia_device_id 琛ㄤ腑锛岄€氳繃锛?```
  echo "match_flags manf_id card_id func_id function device_no \
  prod_id_hash[0] prod_id_hash[1] prod_id_hash[2] prod_id_hash[3]" > \
  /sys/bus/pcmcia/drivers/{driver}/new_id
```
鎵€鏈夊瓧娈靛潎浠ュ崄鍏繘鍒跺€间紶鍏ワ紙涓嶅甫鍓嶅 0x锛夈€傚叾鍚箟鍦?PCMCIA 瑙勮寖涓弿杩帮紝match_flags 鏄敱 include/linux/mod_devicetable.h 涓畾涔夌殑 PCMCIA_DEV_ID_MATCH_* 甯搁噺鎸変綅鎴栫粍鍚堣€屾垚銆?
娣诲姞鍚庯紝閽堝鍏讹紙鏂版洿鏂扮殑锛塸cmcia_device_id 鍒楄〃涓换浣曟湭琚棰嗙殑 PCMCIA 璁惧锛屽皢璋冪敤椹卞姩鐨?probe 渚嬬▼銆?
涓€涓父瑙佺敤渚嬫槸鏍规嵁鍒堕€犲晢 ID 涓庡崱 ID锛堝彇鑷澶囨爲涓殑 manf_id 涓?card_id 鏂囦欢锛夋坊鍔犳柊璁惧锛?```
  echo "0x3 manf_id card_id 0 0 0 0 0 0 0" > \
    /sys/bus/pcmcia/drivers/{driver}/new_id
```
鍦ㄥ姞杞介┍鍔ㄤ箣鍚庛€?