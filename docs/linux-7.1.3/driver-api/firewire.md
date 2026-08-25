## Firewire (IEEE 1394) 驱动接口指南


## 简介与概述


Linux FireWire 子系统向 Linux 系统添加了一些接口，用于使用/维护 IEEE 1394
总线上的任何资源
这些接口的主要目的是通过 ISO/IEC 13213 (IEEE 1212) 过程访问 IEEE 1394 总线
上每个节点的地址空间，并通过 IEEE 1394 过程控制总线上的等时资源
根据接口的消费者，添加了两类接口。一组用户空间接口可通过 `firewire 字符设备`
获得。一组内核接口可通过 `firewire-core` 模块中导出的符号获得
## Firewire 字符设备数据结构


    :literal:

    :internal:

## Firewire 设备探测sysfs 接口


    :literal:

    :export:

## Firewire 核心事务接口


    :export:

## Firewire 等时 I/O 接口


   :functions: fw_iso_context_schedule_flush_completions
   :export:
