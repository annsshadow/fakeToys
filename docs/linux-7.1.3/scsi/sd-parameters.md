## Linux SCSI 磁盘驱动（sd）参


### cache_type（RW


启用/禁用驱动器写缓存与读缓存

===========================   === ===   ===========   ==========
 cache_type 字符           WCE RCD   写缓       读缓
===========================   === ===   ===========   ==========
 write through                0   0     关闭          开
 none                         0   1     关闭          关闭
 write back                   1   0     开         开
 write back, no read (daft)   1   1     开         关闭
===========================   === ===   ===========   ==========

```

  # echo "write back" > cache_type

```
要修改缓存模式而不使更改持久化，前

```

  # echo "temporary write back" > cache_type

```
