## Marvell Odyssey DDR PMU 性能监控单元（PMU UNCORE）


Odyssey DRAM 子系统支持八个计数器用于监控性能，软件可以对这些计数器进行
编程以监控任何已定义的性能事件。受支持的性能事件包括在 DDR 控制器与 PHY
之间的接口、DDR 控制器与 CHI 互连之间的接口，或 DDR 控制器内部计数的事件。

此外，DSS 还支持两个固定的性能事件计数器，一个用于 ddr 读，另一个用于
ddr 写。

该计数器将在手动（manual）或自动（auto）模式下运行。

```

        /sys/bus/event_source/devices/mrvl_ddr_pmu_<>/events/
        /sys/bus/event_source/devices/mrvl_ddr_pmu_<>/format/

```
```

        $ perf list | grep ddr
        mrvl_ddr_pmu_<>/ddr_act_bypass_access/   [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_bsm_alloc/           [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_bsm_starvation/      [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_active_access/   [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_mwr/             [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_rd_active_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_rd_or_wr_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_read/            [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_wr_access/       [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_cam_write/           [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_capar_error/         [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_crit_ref/            [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_ddr_reads/           [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_ddr_writes/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dfi_cmd_is_retry/    [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dfi_cycles/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dfi_parity_poison/   [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dfi_rd_data_access/  [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dfi_wr_data_access/  [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dqsosc_mpc/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_dqsosc_mrr/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_enter_mpsm/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_enter_powerdown/     [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_enter_selfref/       [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hif_pri_rdaccess/    [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hif_rd_access/       [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hif_rd_or_wr_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hif_rmw_access/      [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hif_wr_access/       [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_hpri_sched_rd_crit_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_load_mode/           [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_lpri_sched_rd_crit_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_precharge/           [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_precharge_for_other/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_precharge_for_rdwr/  [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_raw_hazard/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_rd_bypass_access/    [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_rd_crc_error/        [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_rd_uc_ecc_error/     [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_rdwr_transitions/    [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_refresh/             [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_retry_fifo_full/     [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_spec_ref/            [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_tcr_mrr/             [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_war_hazard/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_waw_hazard/          [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_win_limit_reached_rd/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_win_limit_reached_wr/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_wr_crc_error/        [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_wr_trxn_crit_access/ [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_write_combine/       [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_zqcl/                [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_zqlatch/             [Kernel PMU event]
        mrvl_ddr_pmu_<>/ddr_zqstart/             [Kernel PMU event]

        $ perf stat -e ddr_cam_read,ddr_cam_write,ddr_cam_active_access,ddr_cam
          rd_or_wr_access,ddr_cam_rd_active_access,ddr_cam_mwr <workload>

```
