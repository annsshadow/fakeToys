## APM X-Gene SoC 性能监视单元（PMU

X-Gene SoC PMU 由多个相互独立的系统设备 PMU 组成，例L3 cache（L3 缓存）、I/O bridge（I/O 桥）、内存控制器桥（memory controller bridge）以及内存控制器（memory controller）。这PMU 设备采用松散架构，遵循与 ARM 核心 PMU 相同的模型。这PMU 共享相同的最高级中断和状CSR 区域
### PMU（perf）驱

xgene-pmu 驱动会注册多perf PMU 驱动。每perf 驱动都在 sysfs 中提供其可用事件与配置选项的描述，参见 /sys/bus/event_source/devices/<l3cX/iobX/mcbX/mcX>/
“format”目录描perf_event_attr 结构config（事ID）、config1（代ID）字段的格式。“events”目录提供所有受支持事件类型的配置模板，可与 perf 工具一起使用。例如，“l3c0/bank-fifo-full/”等价于“l3c0/config=0x0b/”
大多SoC PMU 都有一份用于监视特定数据通路性能的特agent ID 列表。例如，L3 缓存agent 可以是某个特CPU 或某I/O 桥。每PMU 都有一2 个寄存器，能够屏蔽请求来agent。若设置了与agent 对应的位号所对应的位，则仅当该事件由来自agent 的请求引起时才会计数。每agent ID 位与“config1”字段中的相应位呈反相映射。默认情况下，事件会对所agent 请求计数（config1 = 0x0）。各 PMU 受支持的所agent，请参阅 APM X-Gene 用户手册
每个 perf 驱动还提供“cpumask”sysfs 属性，其中包含将用于处理所PMU 事件的单CPU ID
```

 / # perf list | grep -e l3c -e iob -e mcb -e mc
   l3c0/ackq-full/                                    [Kernel PMU event]
 <...>
   mcb1/mcb-csw-stall/                                [Kernel PMU event]

 / # perf stat -a -e l3c0/read-miss/,mcb1/csw-write-request/ sleep 1

 / # perf stat -a -e l3c0/read-miss,config1=0xfffffffffffffffe/ sleep 1

```
该驱动不支持采样，因此“perf record”无法工作。不支持按任务（不带a”）perf 会话