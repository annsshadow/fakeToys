## Marvell Odyssey PEM 性能监控单元（PMU UNCORE）


PCI Express 接口单元（PEM）关联着一个相应的监控单元。其中包含用于跟踪通过
PCIe 链路传输的各种数据特征的性能计数器。

计数器跟踪入站与出站事务，其中包括针对 posted/non-posted/completion TLP 的
独立计数器。此外，入站与出站的内存读请求及其延迟也可以被监控。地址转换服务
（ATS）事件，例如 ATS Translation、ATS Page Request、ATS Invalidation 及其
相应的延迟也都会被跟踪。

有独立的 64 位计数器用于测量入站与出站事务中的 posted/non-posted/completion
TLP。ATS 事件则由不同的计数器测量。

PMU 驱动通过 sysfs 下的 /sys/bus/event_source/devices/mrvl_pcie_rc_pmu_<>/events/
与 /sys/bus/event_source/devices/mrvl_pcie_rc_pmu_<>/format/ 暴露可用的事件与
格式选项。

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
