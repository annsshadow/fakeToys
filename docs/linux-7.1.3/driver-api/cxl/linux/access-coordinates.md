
## CXL 访问坐标计算


## 延迟与带宽计算


一个内存区域的性能坐标（延迟和带宽）通常通过 ACPI 表 [SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 和 [HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 提供。然而，平台固件（BIOS）无法为热插拔的 CXL 设备标注这些信息，因为它们在平台固件初始化期间并不存在。CXL 驱动可以通过从多个组件检索数据来计算性能坐标。

[SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 提供了一个 Generic Port Affinity 子表，它把一个 proximity domain 绑定到一个设备句柄，在此情况下即为 CXL hostbridge。利用这种关联，可以从 [HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 子表检索 Generic Port 的性能坐标。这一部分表示从 CPU 到 Generic Port（CXL hostbridge）之间的性能坐标。

[CDAT <../platform/cdat>](CDAT <../platform/cdat>) 提供 CXL 设备本身的性能坐标。即访问该设备内存区域的带宽和延迟。DSMAS 子表提供一个与设备物理地址（DPA）范围绑定的 DSMADHandle。DSLBIS 子表提供与 DSMADhandle 绑定的性能坐标，这把两个表项联系在一起，为每个 DPA 区域提供性能坐标。例如，如果一个设备导出了一个 DRAM 区域和一个 PMEM 区域，那么这些区域中的每一个都会有不同性能特征。

如果拓扑中有一个 CXL 交换机，则交换机的性能坐标由 SSLBIS 子表提供。它提供在交换机上游端口（upstream port）和指向端点设备的交换机下游端口（downstream port）之间穿越交换机的带宽和延迟。

```

 GP0/HB0/ACPI0016-0
        RP0
         |
         | L0
         |
     SW 0 / USP0
     SW 0 / DSP0
         |
         | L1
         |
        EP0

```
在此示例中，端点与根端口之间有一个 CXL 交换机。此示例中的延迟计算如下：
L(EP0) - 来自 EP0 CDAT DSMAS+DSLBIS 的延迟
L(L1) - EP0 与 SW0DSP0 之间的链路延迟
L(SW0) - 来自 SW0 CDAT SSLBIS 的交换机延迟。
L(L0) - SW0 与 RP0 之间的链路延迟
L(RP0) - 经由 SRAT 和 HMAT（Generic Port）从根端口到 CPU 的延迟。
总读、写延迟是上述所有部分之和。

此示例中的带宽计算如下：
B(EP0) - 来自 EP0 CDAT DSMAS+DSLBIS 的带宽
B(L1) - EP0 与 SW0DSP0 之间的链路带宽
B(SW0) - 来自 SW0 CDAT SSLBIS 的交换机带宽。
B(L0) - SW0 与 RP0 之间的链路带宽
B(RP0) - 经由 SRAT 和 HMAT（Generic Port）从根端口到 CPU 的带宽。
总读、写带宽是上述所有部分的最小值（min()）。

要计算链路带宽：
LinkOperatingFrequency (GT/s) 是当前协商的链路速度。
DataRatePerLink (MB/s) = LinkOperatingFrequency / 8
Bandwidth (MB/s) = PCIeCurrentLinkWidth * DataRatePerLink
其中 PCIeCurrentLinkWidth 是链路中的通道数。

要计算链路延迟：
LinkLatency (picoseconds) = FlitSize / LinkBandwidth (MB/s)

细节请参见 `CXL Memory Device SW Guide r1.0 <https://www.intel.com/content/www/us/en/content-details/643805/cxl-memory-device-software-guide.html>`_ 第 2.11.3 和 2.11.4 节。

最终，所构造内存区域的访问坐标由一个或多个 CXL 设备的各个内存分区计算得出。

## 共享上游链路计算


对于某些端点位于 CXL 交换机（SW）或根端口（RP）之后的 CXL 区域构造，所有位于交换机之后的端点的总带宽有可能超过交换机上游链路。在主机内部、根端口上游也可能出现类似情况。CXL 驱动在所有目标都已到达某个区域后，会执行一个额外的遍历，以便在考虑上游链路可能成为限制因素的情况下重新计算带宽。

该算法假设配置是对称拓扑，因为这样能最大化性能。当检测到非对称拓扑时，计算中止。非对称拓扑是在拓扑遍历期间检测到的：检测为祖父节点的 RP 数量不等于在同一遍历循环中迭代的设备数量。其假设是属性上细微的不对称不会发生，且到 EP 的所有路径都相等。

一个 RP 下可以有多个交换机。一个 CXL Host Bridge（HB）下可以有多个 RP。一个 [CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 中的 CXL Fixed Memory Window Structure（CFMWS）下可以有多个 HB。

```

                CFMWS 0
                  |
         _________|_________
        |                   |
    ACPI0017-0          ACPI0017-1
 GP0/HB0/ACPI0016-0   GP1/HB1/ACPI0016-1
    |          |        |           |
   RP0        RP1      RP2         RP3
    |          |        |           |
  SW 0       SW 1     SW 2        SW 3
  |   |      |   |    |   |       |   |
 EP0 EP1    EP2 EP3  EP4  EP5    EP6 EP7

```
示例层次结构的计算：

Min (GP0 to CPU BW,
     Min(SW 0 Upstream Link to RP0 BW,
         Min(SW0SSLBIS for SW0DSP0 (EP0), EP0 DSLBIS, EP0 Upstream Link) +
         Min(SW0SSLBIS for SW0DSP1 (EP1), EP1 DSLBIS, EP1 Upstream link)) +
     Min(SW 1 Upstream Link to RP1 BW,
         Min(SW1SSLBIS for SW1DSP0 (EP2), EP2 DSLBIS, EP2 Upstream Link) +
         Min(SW1SSLBIS for SW1DSP1 (EP3), EP3 DSLBIS, EP3 Upstream link))) +
Min (GP1 to CPU BW,
     Min(SW 2 Upstream Link to RP2 BW,
         Min(SW2SSLBIS for SW2DSP0 (EP4), EP4 DSLBIS, EP4 Upstream Link) +
         Min(SW2SSLBIS for SW2DSP1 (EP5), EP5 DSLBIS, EP5 Upstream link)) +
     Min(SW 3 Upstream Link to RP3 BW,
         Min(SW3SSLBIS for SW3DSP0 (EP6), EP6 DSLBIS, EP6 Upstream Link) +
         Min(SW3SSLBIS for SW3DSP1 (EP7), EP7 DSLBIS, EP7 Upstream link))))

计算从 cxl_region_shared_upstream_perf_update() 开始。创建一个 xarray 来通过 cxl_endpoint_gather_bandwidth() 函数收集所有端点带宽。计算来自端点 CDAT 的带宽与上游链路带宽的最小值（min()）。如果端点的父节点是一个 CXL 交换机，则计算带宽与关联到该端点的交换机下游端口的 SSLBIS 带宽的最小值。最终带宽存储在 xarray 中由设备指针索引的 ‘struct cxl_perf_ctx’ 里。如果端点直接挂接到根端口（RP），设备指针将是一个 RP 设备。如果端点位于交换机之后，设备指针将是父交换机的上游设备。

在下一个阶段，代码遍历拓扑中一个或多个（如果存在）交换机。对于直接挂接到 RP 的端点，跳过此步。如果上游还有另一个交换机，代码取当前收集到的带宽与上游链路带宽的最小值。如果上游有交换机，则取上游交换机的 SSLBIS。

一旦拓扑遍历到达 RP（无论是直接挂接的端点，还是经由交换机遍历），就会调用 cxl_rp_gather_bandwidth()。此时所有带宽按每个 host bridge 聚合，这也是结果 xarray 的索引。

下一步是取每个 host bridge 的带宽与 Generic Port（GP）带宽的最小值。GP 的带宽通过 ACPI 表（[SRAT <../platform/acpi/srat>](SRAT <../platform/acpi/srat>) 和 [HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>)）检索。最小带宽在同一个 ACPI0017 设备下聚合，形成一个新的 xarray。

最后，调用 cxl_region_update_bandwidth()，并将最后一个 xarray 中所有成员的聚合带宽更新到驻留在 cxl 区域（cxlr）上下文中的访问坐标。

## QTG ID


每个 [CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 都有一个 QTG ID 字段。该字段提供与 CFMWS 窗口的 QoS Throttling Group（QTG）关联的 ID。一旦计算出访问坐标，就可以向 ACPI0016 设备发出一个 ACPI Device Specific Method，以根据所提供的访问坐标检索 QTG ID。设备的 QTG ID 可用作匹配 CFMWS 的指引，以便为设备性能设置最佳的 Linux 根解码器。
