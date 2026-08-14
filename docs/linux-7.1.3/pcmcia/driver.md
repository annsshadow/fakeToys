## PCMCIA 驱动


### sysfs


新的 PCMCIA ID 可添加到设备驱动的 pcmcia_device_id 表中，通过：
```
  echo "match_flags manf_id card_id func_id function device_no \
  prod_id_hash[0] prod_id_hash[1] prod_id_hash[2] prod_id_hash[3]" > \
  /sys/bus/pcmcia/drivers/{driver}/new_id
```
所有字段均以十六进制值传入（不带前导 0x）。其含义在 PCMCIA 规范中描述，match_flags 是由 include/linux/mod_devicetable.h 中定义的 PCMCIA_DEV_ID_MATCH_* 常量按位或组合而成。

添加后，针对其（新更新的）pcmcia_device_id 列表中任何未被认领的 PCMCIA 设备，将调用驱动的 probe 例程。

一个常见用例是根据制造商 ID 与卡 ID（取自设备树中的 manf_id 与 card_id 文件）添加新设备：
```
  echo "0x3 manf_id card_id 0 0 0 0 0 0 0" > \
    /sys/bus/pcmcia/drivers/{driver}/new_id
```
在加载驱动之后。
