## SD 与 MMC 设备分区


设备分区是 SD/MMC 设备上存在的附加逻辑块设备。

截至撰写本文时，MMC 启动分区受支持并作为 /dev/mmcblkXboot0 与
/dev/mmcblkXboot1 暴露，其中 X 是父设备 /dev/mmcblkX 的索引。

## MMC 启动分区


对两个 MMC 启动分区提供读和写访问。由于启动分区内容的敏感性——通常存储
对平台启动至关重要的引导加载程序或引导加载程序配置表——默认禁用写访问，
以降低意外变砖的概率。

要启用对 /dev/mmcblkXbootY 的写访问，禁用强制只读
```

	echo 0 > /sys/block/mmcblkXbootY/force_ro


```
```

	echo 1 > /sys/block/mmcblkXbootY/force_ro


```
启动分区也可以被锁定为只读直到下次上电，
```

	echo 1 > /sys/block/mmcblkXbootY/ro_lock_until_next_power_on


```
这是卡的特性而非内核的特性。如果卡不支持启动分区锁定，该文件将不存在。如果
该特性在卡上已被禁用，该文件将为只读。

启动分区也可以被永久锁定，但为了避免意外或恶意的变砖，该特性无法通过 sysfs
访问。
