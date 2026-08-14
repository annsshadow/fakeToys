## NVIDIA Tegra410 SoC 非核心性能监控单元（PMU）


NVIDIA Tegra410 SoC 包含多个系统级 PMU，用于测量诸如内存带宽、延迟和利用率等关键性能指标：

- Unified Coherence Fabric (UCF)
- PCIE
- PCIE-TGT
- CPU Memory (CMEM) Latency
- NVLink-C2C
- NV-CLink
- NV-DLink

### PMU 驱动


该 PMU 驱动在 sysfs 中描述每个 PMU 可用的事件与配置。请参阅以下各节以获取每个 PMU 的 sysfs 路径。与其他非核心（uncore）PMU 驱动一样，该驱动提供 "cpumask" sysfs 属性以显示用于处理该 PMU 事件的 CPU id。还有一个 "associated_cpus" sysfs 属性，其中包含与该 PMU 实例相关联的一组 CPU。

### UCF PMU


NVIDIA Tegra410 SoC 中的 Unified Coherence Fabric (UCF) 充当一个分布式缓存（CPU 内存与 CXL 内存的最后一级缓存），以及支持多个连贯缓存代理（coherently caching agent）之间硬件一致性的缓存一致互连，这些代理包括：

  - CPU 簇（cluster）
  - GPU
  - PCIe 排序控制器单元（OCU, Ordering Controller Unit）
  - 其他 IO 一致请求者

此 PMU 设备的事件与配置选项在 sysfs 中描述，参见 /sys/bus/event_source/devices/nvidia_ucf_pmu_<socket-id>。

此 PMU 中可用的部分事件可用于测量带宽与利用率：

  - slc_access_rd：统计对 SLC 的读请求数量。
  - slc_access_wr：统计对 SLC 的写请求数量。
  - slc_bytes_rd：统计由 slc_access_rd 传输的字节数。
  - slc_bytes_wr：统计由 slc_access_wr 传输的字节数。
  - mem_access_rd：统计对本地或远端内存的读请求数量。
  - mem_access_wr：统计对本地或远端内存的写请求数量。
  - mem_bytes_rd：统计由 mem_access_rd 传输的字节数。
  - mem_bytes_wr：统计由 mem_access_wr 传输的字节数。
  - cycles：统计 UCF 周期数。

```

   AVG_SLC_READ_BANDWIDTH_IN_GBPS = SLC_BYTES_RD / ELAPSED_TIME_IN_NS
   AVG_SLC_WRITE_BANDWIDTH_IN_GBPS = SLC_BYTES_WR / ELAPSED_TIME_IN_NS
   AVG_MEM_READ_BANDWIDTH_IN_GBPS = MEM_BYTES_RD / ELAPSED_TIME_IN_NS
   AVG_MEM_WRITE_BANDWIDTH_IN_GBPS = MEM_BYTES_WR / ELAPSED_TIME_IN_NS

```
```

   AVG_SLC_READ_REQUEST_RATE = SLC_ACCESS_RD / CYCLES
   AVG_SLC_WRITE_REQUEST_RATE = SLC_ACCESS_WR / CYCLES
   AVG_MEM_READ_REQUEST_RATE = MEM_ACCESS_RD / CYCLES
   AVG_MEM_WRITE_REQUEST_RATE = MEM_ACCESS_WR / CYCLES

```
关于还有哪些其他事件可用的更多细节，可在 Tegra410 SoC 技术参考手册中找到。

这些事件可以根据源或目的地进行过滤。源过滤器指示发往 SLC 的流量发起者，例如本地 CPU、非 CPU 设备，或远端 socket。目的过滤器指定目的内存类型，例如本地系统内存（CMEM）、本地 GPU 内存（GMEM），或远端内存。目的过滤器的本地/远端分类基于地址的 home socket，而不是数据实际所在的位置。可用的过滤器在 /sys/bus/event_source/devices/nvidia_ucf_pmu_<socket-id>/format/ 中描述。

UCF PMU 事件过滤器列表：

- 源过滤器：

  - src_loc_cpu：若设置，统计来自本地 CPU 的事件
  - src_loc_noncpu：若设置，统计来自本地非 CPU 设备的事件
  - src_rem：若设置，统计来自远端 socket 的 CPU、GPU、PCIE 设备的事件

- 目的过滤器：

  - dst_loc_cmem：若设置，统计到本地系统内存（CMEM）地址的事件
  - dst_loc_gmem：若设置，统计到本地 GPU 内存（GMEM）地址的事件
  - dst_loc_other：若设置，统计到本地 CXL 内存地址的事件
  - dst_rem：若设置，统计到远端 socket 的 CPU、GPU 和 CXL 内存地址的事件

如果未指定源，PMU 将统计来自所有源的事件。如果未指定目的，PMU 将统计到所有目的地的事件。

使用示例：

```

    perf stat -a -e nvidia_ucf_pmu_0/event=0x0/

```
- 在 socket 0 中统计事件 id 0x0，源过滤器 = 本地 CPU，目的过滤器
```

    perf stat -a -e nvidia_ucf_pmu_0/event=0x0,src_loc_cpu=0x1,dst_loc_cmem=0x1/

```
- 在 socket 1 中统计事件 id 0x0，源过滤器 = 本地非 CPU 设备，且
```

    perf stat -a -e nvidia_ucf_pmu_1/event=0x0,src_loc_noncpu=0x1,dst_rem=0x1/

```
### PCIE PMU


此 PMU 位于连接 PCIE 根复合体（RC, root complex）与内存子系统的 SoC 互连中。它监控来自根端口（root port）或某个 PCIE RC 中特定 BDF 到本地或远端内存的所有读/写流量。SoC 中每个 PCIE RC 有一个 PMU。每个 RC 最多可有 16 条通道（lane），可被分叉（bifurcate）为最多 8 个根端口。来自每个根端口的流量可以使用 RP 或 BDF 过滤器进行过滤。例如，指定 "src_rp_mask=0xFF" 表示 PMU 计数器将捕获来自所有 RP 的流量。更多细节请参阅下文。

此 PMU 设备的事件与配置选项在 sysfs 中描述，参见 /sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>_rc_<pcie-rc-id>。

此 PMU 中的事件可用于测量带宽、利用率和延迟：

  - rd_req：统计 PCIE 设备的读请求数量。
  - wr_req：统计 PCIE 设备的写请求数量。
  - rd_bytes：统计由 rd_req 传输的字节数。
  - wr_bytes：统计由 wr_req 传输的字节数。
  - rd_cum_outs：统计每个周期的未完成（outstanding）rd_req。
  - cycles：统计连接到 PCIE 接口的 SoC 互连的时钟周期数。

```

   AVG_RD_BANDWIDTH_IN_GBPS = RD_BYTES / ELAPSED_TIME_IN_NS
   AVG_WR_BANDWIDTH_IN_GBPS = WR_BYTES / ELAPSED_TIME_IN_NS

```
```

   AVG_RD_REQUEST_RATE = RD_REQ / CYCLES
   AVG_WR_REQUEST_RATE = WR_REQ / CYCLES


```
```

   FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS
   AVG_LATENCY_IN_CYCLES = RD_CUM_OUTS / RD_REQ
   AVERAGE_LATENCY_IN_NS = AVG_LATENCY_IN_CYCLES / FREQ_IN_GHZ

```
PMU 事件可以根据流量源和目的地进行过滤。源过滤器指示将被监控的 PCIE 设备。目的过滤器指定目的内存类型，例如本地系统内存（CMEM）、本地 GPU 内存（GMEM），或远端内存。目的过滤器的本地/远端分类基于地址的 home socket，而不是数据实际所在的位置。这些过滤器可在 /sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>_rc_<pcie-rc-id>/format/ 中找到。

事件过滤器列表：

- 源过滤器：

  - src_rp_mask：将被监控的根端口的位掩码（bitmask）。此位掩码中的每一位代表 RC 中的 RP 索引。若某位被置位，则关联 RP 下的所有设备都将被监控。例如 "src_rp_mask=0xF" 将监控根端口 0 到 3 中的设备。
  - src_bdf：将被监控的 BDF。这是一个 16 位值，遵循公式：(bus << 8) + (device << 3) + (function)。例如，BDF 27:01.1 的值为 0x2781。
  - src_bdf_en：启用 BDF 过滤器。若设置，则使用 "src_bdf" 中的 BDF 过滤值来过滤流量。

  注意，Root-Port 与 BDF 过滤器是互斥的，且每个 RC 中的 PMU 对于整个计数器只能有一个 BDF 过滤器。如果启用了 BDF 过滤器，该 BDF 过滤值将应用于所有事件。

- 目的过滤器：

  - dst_loc_cmem：若设置，统计到本地系统内存（CMEM）地址的事件
  - dst_loc_gmem：若设置，统计到本地 GPU 内存（GMEM）地址的事件
  - dst_loc_pcie_p2p：若设置，统计到本地 PCIE 对等（peer）地址的事件
  - dst_loc_pcie_cxl：若设置，统计到本地 CXL 内存地址的事件
  - dst_rem：若设置，统计到远端内存地址的事件

如果未指定源过滤器，PMU 将统计来自所有根端口的事件。如果未指定目的过滤器，PMU 将统计到所有目的地的事件。

使用示例：

- 在 socket 0 上统计来自 PCIE RC-0 根端口 0、目标为所有目的地的事件 id 0x0
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_0/event=0x0,src_rp_mask=0x1/

```
- 在 socket 0 上统计来自 PCIE RC-1 根端口 0 和 1、且
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_1/event=0x1,src_rp_mask=0x3,dst_loc_cmem=0x1/

```
- 在 socket 1 上统计来自 PCIE RC-2 根端口 0、目标为所有目的地的事件 id 0x2
```

    perf stat -a -e nvidia_pcie_pmu_1_rc_2/event=0x2,src_rp_mask=0x1/

```
- 在 socket 1 上统计来自 PCIE RC-3 根端口 0 和 1、且
```

    perf stat -a -e nvidia_pcie_pmu_1_rc_3/event=0x3,src_rp_mask=0x3,dst_loc_cmem=0x1/

```
- 在 socket 0 上统计来自 PCIE RC-4 的 BDF 01:01.0、目标为所有目的地的事件 id 0x4
```

    perf stat -a -e nvidia_pcie_pmu_0_rc_4/event=0x4,src_bdf=0x0180,src_bdf_en=0x1/

```

#### 将 RC# 映射到 lspci 段号


将 RC# 映射到 lspci 段号可能并不容易；因此为每个 RP 在 PCIE 配置空间中添加了一个新的 NVIDIA 指定厂商特定能力（DVSEC, Designated Vendor Specific Capability）寄存器。此 DVSEC 的厂商 id 为 "10de"，DVSEC id 为 "0x4"。该 DVSEC 寄存器包含以下信息，用于将 RP 下的 PCIE 设备映射回其 RC#：

  - Bus#（字节 0xc）：由 lspci 输出报告的 bus 号
  - Segment#（字节 0xd）：由 lspci 输出报告的 segment 号
  - RP#（字节 0xe）：对于具有根端口能力的设备，由 lspci 的 LnkCap 属性报告的端口号
  - RC#（字节 0xf）：与该 RP 关联的根复合体号
  - Socket#（字节 0x10）：与该 RP 关联的 socket 号

```

  #!/bin/bash
  while read bdf rest; do
    dvsec4_reg=$(lspci -vv -s $bdf | awk '
      /Designated Vendor-Specific: Vendor=10de ID=0004/ {
        match($0, /\[([0-9a-fA-F]+)/, arr);
        print "0x" arr[1];
        exit
      }
    ')
    if [ -n "$dvsec4_reg" ]; then
      bus=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xc))).b)
      segment=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xd))).b)
      rp=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xe))).b)
      rc=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0xf))).b)
      socket=$(setpci -s $bdf $(printf '0x%x' $((${dvsec4_reg} + 0x10))).b)
      echo "$bdf: Bus=$bus, Segment=$segment, RP=$rp, RC=$rc, Socket=$socket"
    fi
  done < <(lspci -d 10de:)

```
```

  0001:00:00.0: Bus=00, Segment=01, RP=00, RC=00, Socket=00
  0002:80:00.0: Bus=80, Segment=02, RP=01, RC=01, Socket=00
  0002:a0:00.0: Bus=a0, Segment=02, RP=02, RC=01, Socket=00
  0002:c0:00.0: Bus=c0, Segment=02, RP=03, RC=01, Socket=00
  0002:e0:00.0: Bus=e0, Segment=02, RP=04, RC=01, Socket=00
  0003:00:00.0: Bus=00, Segment=03, RP=00, RC=02, Socket=00
  0004:00:00.0: Bus=00, Segment=04, RP=00, RC=03, Socket=00
  0005:00:00.0: Bus=00, Segment=05, RP=00, RC=04, Socket=00
  0005:40:00.0: Bus=40, Segment=05, RP=01, RC=04, Socket=00
  0005:c0:00.0: Bus=c0, Segment=05, RP=02, RC=04, Socket=00
  0006:00:00.0: Bus=00, Segment=06, RP=00, RC=05, Socket=00
  0009:00:00.0: Bus=00, Segment=09, RP=00, RC=00, Socket=01
  000a:80:00.0: Bus=80, Segment=0a, RP=01, RC=01, Socket=01
  000a:a0:00.0: Bus=a0, Segment=0a, RP=02, RC=01, Socket=01
  000a:e0:00.0: Bus=e0, Segment=0a, RP=03, RC=01, Socket=01
  000b:00:00.0: Bus=00, Segment=0b, RP=00, RC=02, Socket=01
  000c:00:00.0: Bus=00, Segment=0c, RP=00, RC=03, Socket=01
  000d:00:00.0: Bus=00, Segment=0d, RP=00, RC=04, Socket=01
  000d:40:00.0: Bus=40, Segment=0d, RP=01, RC=04, Socket=01
  000d:c0:00.0: Bus=c0, Segment=0d, RP=02, RC=04, Socket=01
  000e:00:00.0: Bus=00, Segment=0e, RP=00, RC=05, Socket=01

```
### PCIE-TGT PMU


此 PMU 位于连接 PCIE 根复合体（RC）与内存子系统的 SoC 互连中。它监控以 PCIE BAR 和 CXL HDM 范围为目标的流量。SoC 中每个 PCIE RC 有一个 PCIE-TGT PMU。Tegra410 SoC 中的每个 RC 最多可有 16 条通道，可被分叉为最多 8 个根端口（RP）。该 PMU 提供 RP 过滤器来统计到每个 RP 的 PCIE BAR 流量，以及地址过滤器来统计对 PCIE BAR 或 CXL HDM 范围的访问。过滤器的细节在以下各节描述。

将 RC# 映射到 lspci 段号的方式与 PCIE PMU 类似。更多信息请参阅 NVIDIA_T410_PCIE_PMU_RC_Mapping_Section。

此 PMU 设备的事件与配置选项在 sysfs 中可用，参见 /sys/bus/event_source/devices/nvidia_pcie_tgt_pmu_<socket-id>_rc_<pcie-rc-id>。

此 PMU 中的事件可用于测量带宽和利用率：

  - rd_req：统计到 PCIE 的读请求数量。
  - wr_req：统计到 PCIE 的写请求数量。
  - rd_bytes：统计由 rd_req 传输的字节数。
  - wr_bytes：统计由 wr_req 传输的字节数。
  - cycles：统计连接到 PCIE 接口的 SoC 互连的时钟周期数。

```

   AVG_RD_BANDWIDTH_IN_GBPS = RD_BYTES / ELAPSED_TIME_IN_NS
   AVG_WR_BANDWIDTH_IN_GBPS = WR_BYTES / ELAPSED_TIME_IN_NS

```
```

   AVG_RD_REQUEST_RATE = RD_REQ / CYCLES
   AVG_WR_REQUEST_RATE = WR_REQ / CYCLES

```
PMU 事件可以根据目的根端口或目标地址范围进行过滤。基于 RP 的过滤仅对 PCIE BAR 流量可用。地址过滤器对 PCIE BAR 和 CXL HDM 范围都有效。这些过滤器可在 sysfs 中找到，参见 /sys/bus/event_source/devices/nvidia_pcie_tgt_pmu_<socket-id>_rc_<pcie-rc-id>/format/。

目的过滤器设置：

- dst_rp_mask：选择要监控的根端口的位掩码。例如 "dst_rp_mask=0xFF" 对应 PCIE RC 中的所有根端口（从 0 到 7）。注意此过滤器仅对 PCIE BAR 流量可用。
- dst_addr_base：BAR 或 CXL HDM 过滤器基址。
- dst_addr_mask：BAR 或 CXL HDM 过滤器地址掩码。
- dst_addr_en：启用 BAR 或 CXL HDM 地址范围过滤器。若设置，则使用 "dst_addr_base" 和 "dst_addr_mask" 指定的地址范围来过滤 PCIE BAR 和 CXL HDM 流量地址。PMU 使用如下比较
```

    (txn's addr & dst_addr_mask) == (dst_addr_base & dst_addr_mask)

  如果比较成功，则该事件会被统计。

```
如果未指定目的过滤器，RP 过滤器默认会被配置为统计到所有根端口的 PCIE BAR 流量。

使用示例：

```

    perf stat -a -e nvidia_pcie_tgt_pmu_0_rc_0/event=0x0,dst_rp_mask=0x3/

```
- 统计对 PCIE BAR 或 CXL HDM 地址范围访问的事件 id 0x1
```

    perf stat -a -e nvidia_pcie_tgt_pmu_0_rc_1/event=0x1,dst_addr_base=0x10000,dst_addr_mask=0xFFF00,dst_addr_en=0x1/

```
### CPU Memory (CMEM) Latency PMU


此 PMU 监控从 Unified Coherence Fabric (UCF) 边缘到本地 CPU DRAM 的内存读请求延迟事件：

  - RD_REQ 计数器：统计读请求数量（每个请求 32B）。
  - RD_CUM_OUTS 计数器：累计未完成请求计数器，跟踪读请求处于在途（in flight）状态的周期数。
  - CYCLES 计数器：统计经过的周期数。

```

   FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS
   AVG_LATENCY_IN_CYCLES = RD_CUM_OUTS / RD_REQ
   AVERAGE_LATENCY_IN_NS = AVG_LATENCY_IN_CYCLES / FREQ_IN_GHZ

```
此 PMU 设备的事件与配置选项在 sysfs 中描述，参见 /sys/bus/event_source/devices/nvidia_cmem_latency_pmu_<socket-id>。

```

  perf stat -a -e '{nvidia_cmem_latency_pmu_0/rd_req/,nvidia_cmem_latency_pmu_0/rd_cum_outs/,nvidia_cmem_latency_pmu_0/cycles/}'

```
### NVLink-C2C PMU


此 PMU 监控穿过 NVIDIA Chip-to-Chip (C2C) 接口的读/写内存请求的延迟事件。与 Grace（Tegra241 SoC）中的 C2C PMU 不同，此 PMU 中没有带宽事件。

此 PMU 设备的事件与配置选项在 sysfs 中可用，参见 /sys/bus/event_source/devices/nvidia_nvlink_c2c_pmu_<socket-id>。

事件列表：

  - IN_RD_CUM_OUTS：进入的读请求的累计未完成请求（以周期计）。
  - IN_RD_REQ：进入的读请求数量。
  - IN_WR_CUM_OUTS：进入的写请求的累计未完成请求（以周期计）。
  - IN_WR_REQ：进入的写请求数量。
  - OUT_RD_CUM_OUTS：发出的读请求的累计未完成请求（以周期计）。
  - OUT_RD_REQ：发出的读请求数量。
  - OUT_WR_CUM_OUTS：发出的写请求的累计未完成请求（以周期计）。
  - OUT_WR_REQ：发出的写请求数量。
  - CYCLES：NVLink-C2C 接口周期计数。

进入（incoming）的事件统计从远端设备到 SoC 的读/写。发出的（outgoing）事件统计从 SoC 到远端设备的读/写。

sysfs 中的 /sys/bus/event_source/devices/nvidia_nvlink_c2c_pmu_<socket-id>/peer 包含所连接设备的信息。

当 C2C 接口连接到 GPU 时，用户可以使用 "gpu_mask" 参数来过滤到/来自特定 GPU 的流量。每一位代表 GPU 索引，例如 "gpu_mask=0x1" 对应 GPU 0，"gpu_mask=0x3" 对应 GPU 0 和 1。如果未指定，PMU 默认监控所有 GPU。

当连接到另一个 SoC 时，只有读事件可用。

```

   C2C_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   IN_WR_AVG_LATENCY_IN_CYCLES = IN_WR_CUM_OUTS / IN_WR_REQ
   IN_WR_AVG_LATENCY_IN_NS = IN_WR_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   OUT_RD_AVG_LATENCY_IN_CYCLES = OUT_RD_CUM_OUTS / OUT_RD_REQ
   OUT_RD_AVG_LATENCY_IN_NS = OUT_RD_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

   OUT_WR_AVG_LATENCY_IN_CYCLES = OUT_WR_CUM_OUTS / OUT_WR_REQ
   OUT_WR_AVG_LATENCY_IN_NS = OUT_WR_AVG_LATENCY_IN_CYCLES / C2C_FREQ_IN_GHZ

```
使用示例：

```

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_req/

  * Count incoming traffic from GPU 0 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_cum_outs,gpu_mask=0x1/

  * Count incoming traffic from GPU 1 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/in_rd_cum_outs,gpu_mask=0x2/

  * Count outgoing traffic to all GPUs connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_req/

  * Count outgoing traffic to GPU 0 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_cum_outs,gpu_mask=0x1/

  * Count outgoing traffic to GPU 1 connected via NVLink-C2C::

      perf stat -a -e nvidia_nvlink_c2c_pmu_0/out_rd_cum_outs,gpu_mask=0x2/

```
### NV-CLink PMU


此 PMU 监控穿过 NV-CLINK 接口的读内存请求的延迟事件。此 PMU 中没有带宽事件。在 Tegra410 SoC 中，NV-CLink 接口用于连接到另一个 Tegra410 SoC，且此 PMU 只统计读流量。

此 PMU 设备的事件与配置选项在 sysfs 中可用，参见 /sys/bus/event_source/devices/nvidia_nvclink_pmu_<socket-id>。

事件列表：

  - IN_RD_CUM_OUTS：进入的读请求的累计未完成请求（以周期计）。
  - IN_RD_REQ：进入的读请求数量。
  - OUT_RD_CUM_OUTS：发出的读请求的累计未完成请求（以周期计）。
  - OUT_RD_REQ：发出的读请求数量。
  - CYCLES：NV-CLINK 接口周期计数。

进入（incoming）的事件统计从远端设备到 SoC 的读。发出的（outgoing）事件统计从 SoC 到远端设备的读。

```

   CLINK_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / CLINK_FREQ_IN_GHZ

   OUT_RD_AVG_LATENCY_IN_CYCLES = OUT_RD_CUM_OUTS / OUT_RD_REQ
   OUT_RD_AVG_LATENCY_IN_NS = OUT_RD_AVG_LATENCY_IN_CYCLES / CLINK_FREQ_IN_GHZ

```
使用示例：

```

      perf stat -a -e nvidia_nvclink_pmu_0/in_rd_req/

  * Count outgoing read traffic to remote SoC connected via NV-CLINK::

      perf stat -a -e nvidia_nvclink_pmu_0/out_rd_req/

```
### NV-DLink PMU


此 PMU 监控穿过 NV-DLINK 接口的读内存请求的延迟事件。此 PMU 中没有带宽事件。在 Tegra410 SoC 中，此 PMU 只统计 CXL 内存读流量。

此 PMU 设备的事件与配置选项在 sysfs 中可用，参见 /sys/bus/event_source/devices/nvidia_nvdlink_pmu_<socket-id>。

事件列表：

  - IN_RD_CUM_OUTS：到 CXL 内存的累计未完成读请求（以周期计）。
  - IN_RD_REQ：到 CXL 内存的读请求数量。
  - CYCLES：NV-DLINK 接口周期计数。

```

   DLINK_FREQ_IN_GHZ = CYCLES / ELAPSED_TIME_IN_NS

   IN_RD_AVG_LATENCY_IN_CYCLES = IN_RD_CUM_OUTS / IN_RD_REQ
   IN_RD_AVG_LATENCY_IN_NS = IN_RD_AVG_LATENCY_IN_CYCLES / DLINK_FREQ_IN_GHZ

```
使用示例：

```

      perf stat -a -e '{nvidia_nvdlink_pmu_0/in_rd_req/,nvidia_nvdlink_pmu_0/in_rd_cum_outs/}'

```
