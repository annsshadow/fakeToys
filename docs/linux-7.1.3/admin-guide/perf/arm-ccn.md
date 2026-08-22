## ARM 缓存一致性网络（Cache Coherent Network

CCN-504 是一个环形总线互连，由 11 个交叉点（XP）组成，每个交叉点最多支持两个设备端口，
因此节点（设备）0 1 连接到交叉点 0，节2 3 连接到交叉点 1，依此类推
### PMU（perf）驱

CCN 驱动注册一perf PMU 驱动，它sysfs 中提供可用事件和配置选项的描述，/sys/bus/event_source/devices/ccn*
“format目录描述perf_event_attr 结构config、config1 config2 字段的格式“events目录为所有已记录的事件提供配置模板，可与 perf 工具一起使用。例“xp_valid_flit等同“type=0x8,event=0x4”。其他参数必须显式指定
对于来源于设备的事件，“node定义其索引
交叉PMU 事件需“xp”（索引）、“bus”（总线号）“vc”（虚拟通道 ID）
基于交叉点观察点（watchpoint）的事件（特殊的 “event0xfe）需要上“xp“vc”，
外加 “port”（设备端口索引）、“dir”（发接收方向）、比较器值（“cmp_l“cmp_h”）“mask”（比较器掩码的索引）
掩码独立于事件描述定义（由于 config 值数量有限），位“cmp_mask目录中，其中8 个可用户配置，另4 个为最常用场景硬编码
周期计数器由 “type0xff 描述，不需要任何其他设置
该驱动还提供 “cpumasksysfs 属性，其中包含一个单独的 CPU ID，即用于处理所CCN PMU 事件处理器。建议用户空间工具在该处理器上请求事件（否则 perf_event->cpu 值无论如何都会被覆盖）如果该处理器被离线，事件会迁移到另一个处理器，并且该属性会被更新```

  / # perf list | grep ccn
    ccn/cycles/                                        [Kernel PMU event]
  <...>
    ccn/xp_valid_flit,xp=?,port=?,vc=?,dir=?/          [Kernel PMU event]
  <...>

  / # perf stat -a -e ccn/cycles/,ccn/xp_valid_flit,xp=1,port=0,vc=1,dir=1/ \
                                                                         sleep 1

```
该驱动不支持采样，因“perf record不会工作。不支持每任务（不带 a”）perf 会话