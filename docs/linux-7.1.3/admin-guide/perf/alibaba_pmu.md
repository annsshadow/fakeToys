## 阿里巴巴 T-Head SoC Uncore 性能监控单元（PMU


Yitian 710 由阿里巴巴集团芯片开发业T-Head 定制打造，实现了用于性能与功能调试、以方便系统维护uncore PMU

## DDR 子系Driveway（DRW）PMU 驱动


Yitian 710 采用八个 DDR5/4 通道，每die 上四个。每DDR5 通道相互独立，以服务系统内存请求。一DDR5 通道被拆分为两个独立的子通道。DDR 子系Driveway 为每个子通道实现独立PMU，以监控各种性能指标

Driveway PMU 设备通过 ali_drw_<sys_base_addr> 命名，并perf 一起使用。例如，ali_drw_21000 ali_drw_21080 die 0 中同一通道的两个子通道对应的两PMU 设备。die 1 PMU 设备ali_drw_400XXXXX 为前缀，例ali_drw_40021000

每个子通道共有 36 PMU 计数器，分为四组

- 0 组：PMU 周期计数器。该组有一对计数器 pmu_cycle_cnt_low pmu_cycle_cnt_high，用作基DDRC 核心时钟的周期计数

- 1 组：PMU 带宽计数器。该组有 8 个计数器，用于统计所rank 中八bank 组，或前 4 个计数器中四rank 分别的总访问次数。基本传输单元为 64B

- 2 组：PMU 重试计数器。该组有 10 个计数器，用于统计每类不可纠正错误的总重试次数

- 3 组：PMU 通用计数器。该组有 16 个计数器，用于统计通用事件

目前，Driveway PMU 驱动仅使用第 0 组和3 组中的计数器

DDR 控制器（DDRCTL）与 DDR PHY 共同构成SoC 应用总线连接DDR 内存设备的完整方案。DDRCTL 接收Synopsys 自定义定义的 Host Interface（HIF）事务。这些事务在内部排队并调度访问，同时满足 SDRAM 协议时序要求、事务优先级以及事务之间的依赖关系。DDRCTL 进而在 DDR PHY Interface（DFI）上PHY 模块发出命令，PHY 模块SDRAM 发起并捕获数据。driveway PMU 具备硬件逻辑，用于采HIF、DFI 等上的统计与性能记录信号

通过对通过 HIF 接口发往 DDRC READ、WRITE RMW 命令进行计数，我们可以计算出带宽。统计内存的计数示例用法
```

  perf stat \
    -e ali_drw_21000/hif_wr/ \
    -e ali_drw_21000/hif_rd/ \
    -e ali_drw_21000/hif_rmw/ \
    -e ali_drw_21000/cycle/ \
    -e ali_drw_21080/hif_wr/ \
    -e ali_drw_21080/hif_rd/ \
    -e ali_drw_21080/hif_rmw/ \
    -e ali_drw_21080/cycle/ \
    -e ali_drw_23000/hif_wr/ \
    -e ali_drw_23000/hif_rd/ \
    -e ali_drw_23000/hif_rmw/ \
    -e ali_drw_23000/cycle/ \
    -e ali_drw_23080/hif_wr/ \
    -e ali_drw_23080/hif_rd/ \
    -e ali_drw_23080/hif_rmw/ \
    -e ali_drw_23080/cycle/ \
    -e ali_drw_25000/hif_wr/ \
    -e ali_drw_25000/hif_rd/ \
    -e ali_drw_25000/hif_rmw/ \
    -e ali_drw_25000/cycle/ \
    -e ali_drw_25080/hif_wr/ \
    -e ali_drw_25080/hif_rd/ \
    -e ali_drw_25080/hif_rmw/ \
    -e ali_drw_25080/cycle/ \
    -e ali_drw_27000/hif_wr/ \
    -e ali_drw_27000/hif_rd/ \
    -e ali_drw_27000/hif_rmw/ \
    -e ali_drw_27000/cycle/ \
    -e ali_drw_27080/hif_wr/ \
    -e ali_drw_27080/hif_rd/ \
    -e ali_drw_27080/hif_rmw/ \
    -e ali_drw_27080/cycle/ -- sleep 10

```
```

  perf stat -M ddr_read_bandwidth.all -- sleep 10
  perf stat -M ddr_write_bandwidth.all -- sleep 10

```
平均 DRAM 带宽可按如下方式计算

- 读带= perf_hif_rd ** DDRC_WIDTH ** DDRC_Freq / DDRC_Cycle
- 鍐欏甫瀹?= (perf_hif_wr + perf_hif_rmw) ** DDRC_WIDTH ** DDRC_Freq / DDRC_Cycle

其中，DDRC_WIDTH = 64 字节

当前驱动不支持采样。因"perf record" 不受支持。同样，由于事件均为 uncore 事件，附加到任务也不受支持
