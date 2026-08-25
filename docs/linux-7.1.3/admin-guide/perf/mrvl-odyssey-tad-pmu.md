## Marvell Odyssey LLC-TAD 性能监控单元（PMU UNCORE

每个 TAD 提供八个 64 位计数器用于监控缓存行为。驱动始终为所TAD 配置相同的计数器。用户最终会有效地在每个 TAD 中保留八个计数器之一，以便跨所TAD 进行观察事件的发生次数会被聚合，并在工作负载运行结束时呈现给用户。驱动没有提供让用户TAD 进行分区以便不同 TAD 用于不同应用程序的方法
性能事件反映了各种内部或接口活动。通过组合多个性能计数器的值，可以以缓存缺失率、缓存分配、接口重试率、内部资源占用率等方式来衡量缓存性能，等等
```

        /sys/bus/event_source/devices/tad/events/
        /sys/bus/event_source/devices/tad/format/

```
```

   $ perf list | grep tad
        tad/tad_alloc_any/                                 [Kernel PMU event]
        tad/tad_alloc_dtg/                                 [Kernel PMU event]
        tad/tad_alloc_ltg/                                 [Kernel PMU event]
        tad/tad_hit_any/                                   [Kernel PMU event]
        tad/tad_hit_dtg/                                   [Kernel PMU event]
        tad/tad_hit_ltg/                                   [Kernel PMU event]
        tad/tad_req_msh_in_exlmn/                          [Kernel PMU event]
        tad/tad_tag_rd/                                    [Kernel PMU event]
        tad/tad_tot_cycle/                                 [Kernel PMU event]

   $ perf stat -e tad_alloc_dtg,tad_alloc_ltg,tad_alloc_any,tad_hit_dtg,tad_hit_ltg,tad_hit_any,tad_tag_rd <workload>

```
