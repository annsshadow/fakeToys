## Freescale i.MX8 DDR 性能监控单元（PMU

DRAM 控制器内部没有性能计数器，因此性能信号被引出到控制器的边缘，在那里
实现了一4 x 32 位的计数器。这组计数器由在计数器控制寄存器中编程的 CSV
模式控制，从而会产生大量PERF 信号
每个计数器的值通过 config 寄存器进行选择。每个计数器对应一个寄存器。计数器 0
比较特殊，它总是计数“时间”，并在到期时对自身以及其它计数器加锁，并触发一中断。如果任何其它计数器溢出，它会继续计数，且不会触发中断
"format" 目录描述perf_event_attr 结构config（事ID）以config1/2
（AXI 过滤器设置）字段的格式，参见 /sys/bus/event_source/devices/imx8_ddr0/format/"events" 目录描述了可perf 工具配合使用的、硬件支持的事件类型，参/sys/bus/event_source/devices/imx8_ddr0/events/caps" 目录描述DDR PMU
中实现的过滤特性，参见 /sys/bus/events_source/devices/imx8_ddr0/caps/
    .. code-block:: bash

        perf stat -a -e imx8_ddr0/cycles/ cmd
        perf stat -a -e imx8_ddr0/read/,imx8_ddr0/write/ cmd

AXI 过滤仅被 CSV 模式 0x41（axid-read）和 0x42（axid-write）使用，用于计数
与过滤设置相匹配的读或写操作。过滤设置因不同DRAM 控制器实现而异，这由驱中的 quirks 来区分。你也可以从用户空间转储信息caps" 目录会显AXI 过滤的类型（filter、enhanced_filter super_filter）。0 表示不支持，1 表示
支持
- With DDR_CAP_AXI_ID_FILTER quirk(filter: 1, enhanced_filter: 0, super_filter: 0).
  quirk 通过以下两部分配置来定义过滤器：
  --AXI_ID defines AxID matching value.
  --AXI_MASKING defines which bits of AxID are meaningful for the matching.

      - 0: corresponding bit is masked.
      - 1: corresponding bit is not masked, i.e. used to do the matching.

  AXI_ID AXI_MASKING 被映射到性能计数器中DPCR1 寄存器。当非屏蔽位  相应AXI_ID 位匹配时，计数器```
        AxID && AXI_MASKING == AXI_ID && AXI_MASKING

  This filter doesn't support filter different AXI ID for axid-read and axid-write
  event at the same time as this filter is shared between counters.

  .. code-block:: bash

      perf stat -a -e imx8_ddr0/axid-read,axi_mask=0xMMMM,axi_id=0xDDDD/ cmd
      perf stat -a -e imx8_ddr0/axid-write,axi_mask=0xMMMM,axi_id=0xDDDD/ cmd

  .. note::

      axi_mask is inverted in userspace(i.e. set bits are bits to mask), and
      it will be reverted in driver automatically. so that the user can just specify
      axi_id to monitor a specific id, rather than having to specify axi_mask.

  .. code-block:: bash

        perf stat -a -e imx8_ddr0/axid-read,axi_id=0x12/ cmd, which will monitor ARID=0x12

```
- With DDR_CAP_AXI_ID_FILTER_ENHANCED quirk(filter: 1, enhanced_filter: 1, super_filter: 0).
  这是DDR_CAP_AXI_ID_FILTER quirk 的扩展，它允许在与另一组数据计数器并发  情况下，计数来自 DDR 读写事务的字节数（而非突发次数）
- With DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER quirk(filter: 0, enhanced_filter: 0, super_filter: 1).
  先前AXI 过滤器存在限制，由于过滤器在计数器间共享，它无法同时过滤不同ID  quirk AXI ID 过滤器的扩展。一处改进是计数1-3 拥有各自的过滤器，意味着
  它支持并发过滤不同的 ID。另一处改进是计数1-3 支持 AXI PORT CHANNEL 选择  支持选择地址通道或数据通道
  Filter is defined with 2 configuration registers per counter 1-3.
  --Counter N MASK COMP register - including AXI_ID and AXI_MASKING.
  --Counter N MUX CNTL register - including AXI CHANNEL and AXI PORT.

      - 0: address channel
      - 1: data channel

  DDR 子系统中PMU 仅存在单一port0，因axi_port 被保留，应为 0
  .. code-block:: bash

      perf stat -a -e imx8_ddr0/axid-read,axi_mask=0xMMMM,axi_id=0xDDDD,axi_channel=0xH/ cmd
      perf stat -a -e imx8_ddr0/axid-write,axi_mask=0xMMMM,axi_id=0xDDDD,axi_channel=0xH/ cmd

```
      axi_channel is inverted in userspace, and it will be reverted in driver
      automatically. So that users do not need specify axi_channel if want to
      monitor data channel from DDR transactions, since data channel is more
      meaningful.

```
