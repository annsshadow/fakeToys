
## 跟踪缓冲区扩(TRBE)

    :Author:   Anshuman Khandual <anshuman.khandual@arm.com>
    :Date:     November 2020

### 硬件描述


跟踪缓冲区扩(TRBE) 是一percpu 硬件，它在系统内存中捕获来自相应
percpu 跟踪单元CPU 跟踪。它被插入为 coresight sink 设备，因为相应的
跟踪生成(ETE) 被插入为源设备
TRBE 不符CoreSight 架构规范，但通过 CoreSight 驱动框架进行驱动，以
支持 ETE（符CoreSight 规范）的集成
### Sysfs 文件和目

TRBE 设备出现在现coresight 总线上，与其他设备并列：

```

	>$ ls /sys/bus/coresight/devices
	trbe0  trbe1  trbe2 trbe3

```
```

	>$ ls /sys/bus/coresight/devices/trbe0/
        align flag

```
**关键文件项如下：-**
   - `align`：TRBE 写指针对   - `flag`：TRBE 使用访问标志和脏标志更新内存
