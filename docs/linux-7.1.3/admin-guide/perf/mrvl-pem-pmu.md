## Marvell Odyssey PEM 鎬ц兘鐩戞帶鍗曞厓锛圥MU UNCORE锛?

PCI Express 鎺ュ彛鍗曞厓锛圥EM锛夊叧鑱旂潃涓€涓浉搴旂殑鐩戞帶鍗曞厓銆傚叾涓寘鍚敤浜庤窡韪€氳繃
PCIe 閾捐矾浼犺緭鐨勫悇绉嶆暟鎹壒寰佺殑鎬ц兘璁℃暟鍣ㄣ€?
璁℃暟鍣ㄨ窡韪叆绔欎笌鍑虹珯浜嬪姟锛屽叾涓寘鎷拡瀵?posted/non-posted/completion TLP 鐨?鐙珛璁℃暟鍣ㄣ€傛澶栵紝鍏ョ珯涓庡嚭绔欑殑鍐呭瓨璇昏姹傚強鍏跺欢杩熶篃鍙互琚洃鎺с€傚湴鍧€杞崲鏈嶅姟
锛圓TS锛変簨浠讹紝渚嬪 ATS Translation銆丄TS Page Request銆丄TS Invalidation 鍙婂叾
鐩稿簲鐨勫欢杩熶篃閮戒細琚窡韪€?
鏈夌嫭绔嬬殑 64 浣嶈鏁板櫒鐢ㄤ簬娴嬮噺鍏ョ珯涓庡嚭绔欎簨鍔′腑鐨?posted/non-posted/completion
TLP銆侫TS 浜嬩欢鍒欑敱涓嶅悓鐨勮鏁板櫒娴嬮噺銆?
PMU 椹卞姩閫氳繃 sysfs 涓嬬殑 /sys/bus/event_source/devices/mrvl_pcie_rc_pmu_<>/events/
涓?/sys/bus/event_source/devices/mrvl_pcie_rc_pmu_<>/format/ 鏆撮湶鍙敤鐨勪簨浠朵笌
鏍煎紡閫夐」銆?
```

  # perf list | grep mrvl_pcie_rc_pmu
  mrvl_pcie_rc_pmu_<>/ats_inv/             [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ats_inv_latency/     [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ats_pri/             [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ats_pri_latency/     [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ats_trans/           [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ats_trans_latency/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_inflight/         [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_reads/            [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_req_no_ro_ebus/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_req_no_ro_ncb/    [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_cpl_partid/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_dwords_cpl_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_dwords_npr/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_dwords_pr/    [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_npr/          [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ib_tlp_pr/           [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_inflight_partid/  [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_merges_cpl_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_merges_npr_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_merges_pr_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_reads_partid/     [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_cpl_partid/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_dwords_cpl_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_dwords_npr_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_dwords_pr_partid/ [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_npr_partid/   [Kernel PMU event]
  mrvl_pcie_rc_pmu_<>/ob_tlp_pr_partid/    [Kernel PMU event]


  # perf stat -e ib_inflight,ib_reads,ib_req_no_ro_ebus,ib_req_no_ro_ncb <workload>

```
