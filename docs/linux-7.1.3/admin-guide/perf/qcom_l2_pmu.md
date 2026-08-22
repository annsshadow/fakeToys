## Qualcomm Technologies Level-2 缓存性能监控单元（PMU


该驱动支Qualcomm Technologies Centriq SoC 中的 L2 缓存簇。存在多个物L2 缓存簇，每个都有自己PMU。每个簇关联一个或多个 CPU

对外暴露一个逻辑 L2 PMU，它聚合来自物理 PMU 的结果

该驱动在 sysfs 中提供其可用事件和配置选项的描述，参见 /sys/bus/event_source/devices/l2cache_0

"format" 目录描述了事件的格式

事件可以设想为一个二维数组。每一列代表一组事件。共8 组。每组中同一时刻只能有一个条目被使用。如果指定了来自同一组的多个事件，则相互冲突的事件无法同时被计数

事件指定0xCCG，其CC 是两个十六进制数字，指定代码（数组行），G 指定组（列）0-7

此外还有一个由0xFE 指定的周期计数器事件，它不在上述方案之内

该驱动提供一"cpumask" sysfs 属性，其中包含一个掩码，由每个簇的一CPU 组成，该 CPU 将用于处理该簇上的所PMU 事件

```
  perf stat -e l2cache_0/config=0x001/,l2cache_0/config=0x042/ -a sleep 1

  perf stat -e l2cache_0/config=0xfe/ -C 2 sleep 1

```
该驱动不支持采样，因"perf record" 无法工作。不支持按任务的 perf 会话
