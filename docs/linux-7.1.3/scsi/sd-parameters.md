## Linux SCSI 磁盘驱动（sd）参数


### cache_type（RW）


启用/禁用驱动器写缓存与读缓存。

===========================   === ===   ===========   ==========
 cache_type 字符串            WCE RCD   写缓存        读缓存
===========================   === ===   ===========   ==========
 write through                0   0     关闭          开启
 none                         0   1     关闭          关闭
 write back                   1   0     开启          开启
 write back, no read (daft)   1   1     开启          关闭
===========================   === ===   ===========   ==========

```

  # echo "write back" > cache_type

```
要修改缓存模式而不使更改持久化，前置

```

  # echo "temporary write back" > cache_type

```
