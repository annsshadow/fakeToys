## NVIDIA Tegra241 SoC 非核（Uncore）性能监控单元（PMU）


NVIDIA Tegra241 SoC 包含多种系统级 PMU，用于测量诸如内存带宽、延迟和利用率等关键性能指标：

- Scalable Coherency Fabric (SCF)
- NVLink-C2C0
- NVLink-C2C1
- CNVLink
- PCIE

### PMU 驱动


本文档中的 PMU 基于 ARM CoreSight PMU 架构，如文档 ARM IHI 0091 所述。由于这是一种标准架构，这些 PMU 由一个通用驱动 "arm-cs-arch-pmu" 管理。该驱动在 sysfs 中描述每个 PMU 可用的事件和配置。请参见下面各节以获取每个 PMU 的 sysfs 路径。与其他 uncore PMU 驱动一样，该驱动提供 "cpumask" sysfs 属性来显示用于处理 PMU 事件的 CPU id。此外还有一个 "associated_cpus" sysfs 属性，其中包含与该 PMU 实例关联的 CPU 列表。


### SCF PMU


SCF PMU 监视系统级缓存事件、CPU 流量，以及发往本地/远程内存的强序（SO）PCIE 写流量。有关 PMU 流量覆盖的更多信息，请参见 NVIDIA_Uncore_PMU_Traffic_Coverage_Section。

该 PMU 设备的事件和配置选项在 sysfs 中描述，见 /sys/bus/event_source/devices/nvidia_scf_pmu_<socket-id>。

使用示例：

```

   perf stat -a -e nvidia_scf_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_scf_pmu_1/event=0x0/

```
### NVLink-C2C0 PMU


NVLink-C2C0 PMU 监视来自通过 NVLink-C2C（Chip-2-Chip）互连连接的 GPU/CPU 的传入流量。该 PMU 捕获的流量类型取决于芯片配置：

- NVIDIA Grace Hopper Superchip：Hopper GPU 与 Grace SoC 相连。

  在此配置下，PMU 捕获来自 GPU 的 GPU ATS 转换或 EGM 流量。

- NVIDIA Grace CPU Superchip：两个 Grace CPU SoC 相连。

  在此配置下，PMU 捕获来自远端 SoC 的 PCIE 设备的读和松弛序（RO）写。

有关 PMU 流量覆盖的更多信息，请参见 NVIDIA_Uncore_PMU_Traffic_Coverage_Section。

该 PMU 设备的事件和配置选项在 sysfs 中描述，见 /sys/bus/event_source/devices/nvidia_nvlink_c2c0_pmu_<socket-id>。

使用示例：

```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_1/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_2/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_3/event=0x0/

```
NVLink-C2C 有两个端口，可以连接到一个 GPU（占用两个端口）或两个 GPU（每个端口一个 GPU）。用户可以使用 "port" 位图参数来选择要监视的端口。每一位代表端口号，例如 "port=0x1" 对应端口 0，"port=0x3" 对应端口 0 和 1。如果未指定，PMU 默认监视两个端口。

端口过滤示例：

```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0,port=0x1/

```
```

   perf stat -a -e nvidia_nvlink_c2c0_pmu_0/event=0x0,port=0x3/

```
### NVLink-C2C1 PMU


NVLink-C2C1 PMU 监视来自通过 NVLink-C2C（Chip-2-Chip）互连连接的 GPU 的传入流量。该 PMU 捕获未转换的 GPU 流量，这与捕获 ATS 转换流量的 NVLink-C2C0 PMU 不同。有关 PMU 流量覆盖的更多信息，请参见 NVIDIA_Uncore_PMU_Traffic_Coverage_Section。

该 PMU 设备的事件和配置选项在 sysfs 中描述，见 /sys/bus/event_source/devices/nvidia_nvlink_c2c1_pmu_<socket-id>。

使用示例：

```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_1/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_2/event=0x0/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_3/event=0x0/

```
NVLink-C2C 有两个端口，可以连接到一个 GPU（占用两个端口）或两个 GPU（每个端口一个 GPU）。用户可以使用 "port" 位图参数来选择要监视的端口。每一位代表端口号，例如 "port=0x1" 对应端口 0，"port=0x3" 对应端口 0 和 1。如果未指定，PMU 默认监视两个端口。

端口过滤示例：

```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0,port=0x1/

```
```

   perf stat -a -e nvidia_nvlink_c2c1_pmu_0/event=0x0,port=0x3/

```
### CNVLink PMU


CNVLink PMU 监视来自远端插槽上 GPU 和 PCIE 设备发往本地内存的流量。对于 PCIE 流量，该 PMU 捕获读和松弛序（RO）写流量。有关 PMU 流量覆盖的更多信息，请参见 NVIDIA_Uncore_PMU_Traffic_Coverage_Section。

该 PMU 设备的事件和配置选项在 sysfs 中描述，见 /sys/bus/event_source/devices/nvidia_cnvlink_pmu_<socket-id>。

每个 SoC 插槽可以通过 CNVLink 连接到一个或多个插槽。用户可以使用 "rem_socket" 位图参数来选择要监视的远端插槽。每一位代表插槽号，例如 "rem_socket=0xE" 对应插槽 1 到 3。如果未指定，PMU 默认监视所有远端插槽。
/sys/bus/event_source/devices/nvidia_cnvlink_pmu_<socket-id>/format/rem_socket 显示可以在 "rem_socket" 参数中设置的有效位。

该 PMU 无法区分远端流量的发起者，因此不提供用于选择要监视流量源的过滤器。它报告来自远端 GPU 和 PCIE 设备的合并流量。

使用示例：

```

   perf stat -a -e nvidia_cnvlink_pmu_0/event=0x0,rem_socket=0xE/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_1/event=0x0,rem_socket=0xD/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_2/event=0x0,rem_socket=0xB/

```
```

   perf stat -a -e nvidia_cnvlink_pmu_3/event=0x0,rem_socket=0x7/


```
### PCIE PMU


PCIE PMU 监视从 PCIE 根端口发往本地/远程内存的所有读/写流量。有关 PMU 流量覆盖的更多信息，请参见 NVIDIA_Uncore_PMU_Traffic_Coverage_Section。

该 PMU 设备的事件和配置选项在 sysfs 中描述，见 /sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>。

每个 SoC 插槽可以支持多个根端口。用户可以使用 "root_port" 位图参数来选择要监视的端口，即 "root_port=0xF" 对应根端口 0 到 3。如果未指定，PMU 默认监视所有根端口。
/sys/bus/event_source/devices/nvidia_pcie_pmu_<socket-id>/format/root_port 显示可以在 "root_port" 参数中设置的有效位。

使用示例：

```

   perf stat -a -e nvidia_pcie_pmu_0/event=0x0,root_port=0x3/

```
```

   perf stat -a -e nvidia_pcie_pmu_1/event=0x0,root_port=0x3/

```

### 流量覆盖


PMU 的流量覆盖可能因芯片配置而异：

- **NVIDIA Grace Hopper Superchip**：Hopper GPU 与 Grace SoC 相连。

```

   *********************************          *********************************
   * SOCKET-A                      *          * SOCKET-B                      *
   *                               *          *                               *
   *                     ::::::::  *          *  ::::::::                     *
   *                     : PCIE :  *          *  : PCIE :                     *
   *                     ::::::::  *          *  ::::::::                     *
   *                         |     *          *      |                        *
   *                         |     *          *      |                        *
   *  :::::::            ::::::::: *          *  :::::::::            ::::::: *
   *  :     :            :       : *          *  :       :            :     : *
   *  : GPU :<--NVLink-->: Grace :<---CNVLink--->: Grace :<--NVLink-->: GPU : *
   *  :     :    C2C     :  SoC  : *          *  :  SoC  :    C2C     :     : *
   *  :::::::            ::::::::: *          *  :::::::::            ::::::: *
   *     |                   |     *          *      |                   |    *
   *     |                   |     *          *      |                   |    *
   *  &&&&&&&&           &&&&&&&&  *          *   &&&&&&&&           &&&&&&&& *
   *  & GMEM &           & CMEM &  *          *   & CMEM &           & GMEM & *
   *  &&&&&&&&           &&&&&&&&  *          *   &&&&&&&&           &&&&&&&& *
   *                               *          *                               *
   *********************************          *********************************

   GMEM = GPU Memory (e.g. HBM)
   CMEM = CPU Memory (e.g. LPDDR5X)

  |
  | Following table contains traffic coverage of Grace SoC PMU in socket-A:

  ::

   +--------------+-------+-----------+-----------+-----+----------+----------+
   |              |                        Source                             |
   +              +-------+-----------+-----------+-----+----------+----------+
   | Destination  |       |GPU ATS    |GPU Not-ATS|     | Socket-B | Socket-B |
   |              |PCI R/W|Translated,|Translated | CPU | CPU/PCIE1| GPU/PCIE2|
   |              |       |EGM        |           |     |          |          |
   +==============+=======+===========+===========+=====+==========+==========+
   | Local        | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF | SCF PMU  | CNVLink  |
   | SYSRAM/CMEM  | PMU   |PMU        |PMU        | PMU |          | PMU      |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Local GMEM   | PCIE  |    N/A    |NVLink-C2C1| SCF | SCF PMU  | CNVLink  |
   |              | PMU   |           |PMU        | PMU |          | PMU      |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Remote       | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF |          |          |
   | SYSRAM/CMEM  | PMU   |PMU        |PMU        | PMU |   N/A    |   N/A    |
   | over CNVLink |       |           |           |     |          |          |
   +--------------+-------+-----------+-----------+-----+----------+----------+
   | Remote GMEM  | PCIE  |NVLink-C2C0|NVLink-C2C1| SCF |          |          |
   | over CNVLink | PMU   |PMU        |PMU        | PMU |   N/A    |   N/A    |
   +--------------+-------+-----------+-----------+-----+----------+----------+

   PCIE1 traffic represents strongly ordered (SO) writes.
   PCIE2 traffic represents reads and relaxed ordered (RO) writes.

```
- **NVIDIA Grace CPU Superchip**：两个 Grace CPU SoC 相连。

```

   *******************             *******************
   * SOCKET-A        *             * SOCKET-B        *
   *                 *             *                 *
   *    ::::::::     *             *    ::::::::     *
   *    : PCIE :     *             *    : PCIE :     *
   *    ::::::::     *             *    ::::::::     *
   *        |        *             *        |        *
   *        |        *             *        |        *
   *    :::::::::    *             *    :::::::::    *
   *    :       :    *             *    :       :    *
   *    : Grace :<--------NVLink------->: Grace :    *
   *    :  SoC  :    *     C2C     *    :  SoC  :    *
   *    :::::::::    *             *    :::::::::    *
   *        |        *             *        |        *
   *        |        *             *        |        *
   *     &&&&&&&&    *             *     &&&&&&&&    *
   *     & CMEM &    *             *     & CMEM &    *
   *     &&&&&&&&    *             *     &&&&&&&&    *
   *                 *             *                 *
   *******************             *******************

   GMEM = GPU Memory (e.g. HBM)
   CMEM = CPU Memory (e.g. LPDDR5X)

  |
  | Following table contains traffic coverage of Grace SoC PMU in socket-A:

  ::

   +-----------------+-----------+---------+----------+-------------+
   |                 |                      Source                  |
   +                 +-----------+---------+----------+-------------+
   | Destination     |           |         | Socket-B | Socket-B    |
   |                 |  PCI R/W  |   CPU   | CPU/PCIE1| PCIE2       |
   |                 |           |         |          |             |
   +=================+===========+=========+==========+=============+
   | Local           |  PCIE PMU | SCF PMU | SCF PMU  | NVLink-C2C0 |
   | SYSRAM/CMEM     |           |         |          | PMU         |
   +-----------------+-----------+---------+----------+-------------+
   | Remote          |           |         |          |             |
   | SYSRAM/CMEM     |  PCIE PMU | SCF PMU |   N/A    |     N/A     |
   | over NVLink-C2C |           |         |          |             |
   +-----------------+-----------+---------+----------+-------------+

   PCIE1 traffic represents strongly ordered (SO) writes.
   PCIE2 traffic represents reads and relaxed ordered (RO) writes.

```
