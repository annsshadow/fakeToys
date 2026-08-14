## ARM 缓存一致性网络（Cache Coherent Network）


CCN-504 是一个环形总线互连，由 11 个交叉点（XP）组成，每个交叉点最多支持两个设备端口，
因此节点（设备）0 和 1 连接到交叉点 0，节点 2 和 3 连接到交叉点 1，依此类推。

### PMU（perf）驱动


CCN 驱动注册一个 perf PMU 驱动，它在 sysfs 中提供可用事件和配置选项的描述，见
/sys/bus/event_source/devices/ccn*。

“format” 目录描述了 perf_event_attr 结构的 config、config1 和 config2 字段的格式。
“events” 目录为所有已记录的事件提供配置模板，可与 perf 工具一起使用。例如 “xp_valid_flit”
等同于 “type=0x8,event=0x4”。其他参数必须显式指定。

对于来源于设备的事件，“node” 定义其索引。

交叉点 PMU 事件需要 “xp”（索引）、“bus”（总线号）和 “vc”（虚拟通道 ID）。

基于交叉点观察点（watchpoint）的事件（特殊的 “event” 值 0xfe）需要上述 “xp” 和 “vc”，
外加 “port”（设备端口索引）、“dir”（发送/接收方向）、比较器值（“cmp_l” 和 “cmp_h”）和
“mask”（比较器掩码的索引）。

掩码独立于事件描述定义（由于 config 值数量有限），位于 “cmp_mask” 目录中，其中前 8 个可由
用户配置，另有 4 个为最常用场景硬编码。

周期计数器由 “type” 值 0xff 描述，不需要任何其他设置。

该驱动还提供 “cpumask” sysfs 属性，其中包含一个单独的 CPU ID，即用于处理所有 CCN PMU 事件的
处理器。建议用户空间工具在该处理器上请求事件（否则 perf_event->cpu 值无论如何都会被覆盖）。
如果该处理器被离线，事件会迁移到另一个处理器，并且该属性会被更新。
```

  / # perf list | grep ccn
    ccn/cycles/                                        [Kernel PMU event]
  <...>
    ccn/xp_valid_flit,xp=?,port=?,vc=?,dir=?/          [Kernel PMU event]
  <...>

  / # perf stat -a -e ccn/cycles/,ccn/xp_valid_flit,xp=1,port=0,vc=1,dir=1/ \
                                                                         sleep 1

```
该驱动不支持采样，因此 “perf record” 不会工作。不支持每任务（不带 “-a”）的 perf 会话。
