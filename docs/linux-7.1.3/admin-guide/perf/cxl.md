## CXL 性能监控单元（CPMU

CXL rev 3.0 规范在第 13.2 节“Performance Monitoring（性能监控）”中给出CXL 性能
监控单元的定义
CXL 组件（例Root Port、Switch Upstream Port、End Point）可以有任意数量CPMU
实例。CPMU 能力可完全从设备中发现。该规范为所CXL 协议消息类型提供事件定义，并
CXL 设备上通常统计的事物（例如 DRAM 事件）提供一组附加事件
## CPMU 驱动


CPMU 驱动CXL 总线上注册一个名pmu_mem<X>.<Y> perf PMU，代memX 的第 Y CPMU
    /sys/bus/cxl/device/pmu_mem<X>.<Y>

关联PMU 注册
   /sys/bus/event_sources/devices/cxl_pmu_mem<X>.<Y>

与其CXL 总线设备一样，id 没有特定含义，应通过CXL 总线上设备的父设备建关系来确定其对应的具CXL 设备
PMU 驱动sysfs 中提供可用事件和过滤选项的描述
“format目录描述 perf_event_attr 结构config（事件厂id、group id mask）config1（阈值、过滤使能）config2（过滤参数）字段的所有格式。“events目录描述
perf list 中显示的所有已记录事件
perf list 中显示的事件是事件掩码中设置了单个比特的最细粒度事件。更通用的事件可通过config 中设置多个掩码位来启用。例如，所Device to Host 读请求都可以通过
设置以下所有位而在单个计数器上捕获
- d2h_req_rdcurr
- d2h_req_rdown
- d2h_req_rdshared
- d2h_req_rdany
- d2h_req_rdownnodata
```

  $#perf list
  cxl_pmu_mem0.0/clock_ticks/                        [Kernel PMU event]
  cxl_pmu_mem0.0/d2h_req_rdshared/                   [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpcur/                     [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpdata/                    [Kernel PMU event]
  cxl_pmu_mem0.0/h2d_req_snpinv/                     [Kernel PMU event]
  -----------------------------------------------------------

  $# perf stat -a -e cxl_pmu_mem0.0/clock_ticks/ -e cxl_pmu_mem0.0/d2h_req_rdshared/

```
厂商特定的事件也可能可用，若可用可通过以下方式使用

  $# perf stat -a -e cxl_pmu_mem0.0/vid=VID,gid=GID,mask=MASK/

该驱动不支持采样，因“perf record不受支持。它只支持系统范围的计数，因附加到任务不受支持