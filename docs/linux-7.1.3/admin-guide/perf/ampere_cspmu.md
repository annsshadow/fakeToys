## Ampere SoC 性能监控单元（PMU


Ampere SoC PMU 是一个遵Arm CoreSight PMU 架构的通用 PMU IP。因此，该驱动作arm_cspmu 驱动submodule 实现。在第一阶段，它用于统计 AmpereOne 上的 MCU 事件


### MCU PMU 事件


PMU 驱动支持"rank"bank" "threshold" 设置过滤器。注意，过滤器是PMU 实例而非按事件设置的

```

  / # perf list ampere

    ampere_mcu_pmu_0/act_sent/                         [Kernel PMU event]
    <...>
    ampere_mcu_pmu_1/rd_sent/                          [Kernel PMU event]
    <...>

  / # perf stat -a -e ampere_mcu_pmu_0/act_sent,bank=5,rank=3,threshold=2/,ampere_mcu_pmu_1/rd_sent/ \
        sleep 1

```
