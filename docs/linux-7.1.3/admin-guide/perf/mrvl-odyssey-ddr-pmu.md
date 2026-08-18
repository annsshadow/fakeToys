## Marvell Odyssey DDR PMU 鎬ц兘鐩戞帶鍗曞厓锛圥MU UNCORE锛?

Odyssey DRAM 瀛愮郴缁熸敮鎸佸叓涓鏁板櫒鐢ㄤ簬鐩戞帶鎬ц兘锛岃蒋浠跺彲浠ュ杩欎簺璁℃暟鍣ㄨ繘琛?缂栫▼浠ョ洃鎺т换浣曞凡瀹氫箟鐨勬€ц兘浜嬩欢銆傚彈鏀寔鐨勬€ц兘浜嬩欢鍖呮嫭鍦?DDR 鎺у埗鍣ㄤ笌 PHY
涔嬮棿鐨勬帴鍙ｃ€丏DR 鎺у埗鍣ㄤ笌 CHI 浜掕繛涔嬮棿鐨勬帴鍙ｏ紝鎴?DDR 鎺у埗鍣ㄥ唴閮ㄨ鏁扮殑浜嬩欢銆?
姝ゅ锛孌SS 杩樻敮鎸佷袱涓浐瀹氱殑鎬ц兘浜嬩欢璁℃暟鍣紝涓€涓敤浜?ddr 璇伙紝鍙︿竴涓敤浜?ddr 鍐欍€?
璇ヨ鏁板櫒灏嗗湪鎵嬪姩锛坢anual锛夋垨鑷姩锛坅uto锛夋ā寮忎笅杩愯銆?
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
