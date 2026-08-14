## 海思 PCIe 性能监控单元（PMU）

在 Hip09 上，海思 PCIe 性能监控单元（PMU）可以监控 PCIe 的带宽、延迟、总线利用率以及缓冲区占用数据。

每个 PCIe Core 都有一个 PMU，用于监控该 PCIe Core 的多个 Root Port 以及这些 Root Port 下游的所有 Endpoint。

## 海思 PCIe PMU 驱动

PCIe PMU 驱动以它的 sicl-id 和 PCIe 命名注册一个 perf PMU
```

  /sys/bus/event_source/hisi_pcie<sicl>_core<core>

```
PMU 驱动在 sysfs 中提供可用事件和过滤选项的描述，见 /sys/bus/event_source/devices/hisi_pcie<sicl>_core<core>。

"format" 目录描述了 perf_event_attr 结构的 config（事件）和 config1（过滤选项）字段的所有格式。"events" 目录描述了 perf list 中显示的所有已记录事件。

"identifier" sysfs 文件允许用户识别 PMU 硬件设备的版本。

"bus" sysfs 文件允许用户获取被 PMU 监控的 Root Port 的总线号。此外，用户可以分别从 "bdf_min" 和 "bdf_max" sysfs 属性获得 [bdf_min, bdf_max] 中的 Root Port 范围。

```

  $# perf list
  hisi_pcie0_core0/rx_mwr_latency/ [kernel PMU event]
  hisi_pcie0_core0/rx_mwr_cnt/ [kernel PMU event]
  ------------------------------------------

  $# perf stat -e hisi_pcie0_core0/rx_mwr_latency,port=0xffff/
  $# perf stat -e hisi_pcie0_core0/rx_mwr_cnt,port=0xffff/

```
相关事件通常用于计算带宽、延迟或其他指标。它们需要在同一时刻开始和结束计数，因此相关事件最好在同一个事件组中以得到期望值。有两种方法可以知道它们是否为相关事件：

a) 通过事件名称，例如延迟事件 "xxx_latency, xxx_cnt" 或带宽事件 "xxx_flux, xxx_time"。
b) 通过事件类型，例如 "event=0xXXXX, event=0x1XXXX"。

```

  $# perf stat -e "{hisi_pcie0_core0/rx_mwr_latency,port=0xffff/,hisi_pcie0_core0/rx_mwr_cnt,port=0xffff/}"

```
当前的驱动不支持采样。因此不支持 "perf record"。对于 PCIe PMU 也不支持绑定到一个任务。

### 过滤选项

1. 目标过滤

   PMU 只能监控下游目标 Root Port 或下游目标 Endpoint 流量的性能。PCIe PMU 驱动为用户提供 "port" 和 "bdf" 接口。
   请注意，这两个接口必须设置其中一个，并且这两个接口不能同时受支持。如果两者都设置了，则只有 "port" 过滤有效。
   如果 "port" 过滤未被设置，或被显式设为 0（默认值），则 "bdf" 过滤生效，因为 "bdf=0" 表示 0000:000:00.0。

   - port

     "port" 过滤可用于所有 PCIe PMU 事件，可以通过配置 16 位位图 "port" 来选择目标 Root Port。对于 AP 层事件可以选择多个 port，而对于 TL/DL 层事件只能选择一个 port。

     例如，如果目标 Root Port 是 0000:00:00.0（x8 通道），应设置位图 bit0，即 port=0x1；如果目标 Root Port 是 0000:00:04.0（x4 通道），设置 bit8，即 port=0x100；如果这两个 Root Port 都被监控，则 port=0x101。

```

       $# perf stat -e hisi_pcie0_core0/rx_mwr_latency,port=0x1/ sleep 5

   - bdf

     "bdf" 过滤只能用于带宽事件，通过把 BDF 配置为 "bdf" 来选择目标 Endpoint。计数器只统计由目标 Endpoint 请求的消息的带宽。

     例如，"bdf=0x3900" 表示目标 Endpoint 的 BDF 是 0000:39:00.0。

     perf 用法示例如下：

       $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,bdf=0x3900/ sleep 5

```
2. 触发过滤

   当 TLP 长度第一次大于/小于触发条件时，事件统计开始。可以通过写入 "trig_len" 设置触发条件，通过写入 "trig_mode" 设置触发模式。该过滤只能用于带宽事件。

   例如，"trig_len=4" 表示触发条件为 2^4 DW，"trig_mode=0" 表示当 TLP 长度 > 触发条件时统计开始，"trig_mode=1" 表示当 TLP 长度 < 条件时开始。

```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,trig_len=0x4,trig_mode=1/ sleep 5

```
3. 阈值过滤

   当 TLP 长度在指定范围内时计数。可以通过写入 "thr_len" 设置阈值，通过写入 "thr_mode" 设置阈值模式。该过滤只能用于带宽事件。

   例如，"thr_len=4" 表示阈值为 2^4 DW，"thr_mode=0" 表示当 TLP 长度 >= 阈值时计数，"thr_mode=1" 表示当 TLP 长度 < 阈值时计数。

```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,thr_len=0x4,thr_mode=1/ sleep 5

```
4. TLP 长度过滤

   在统计带宽时，数据可以由 TLP 包的某些部分组成。你可以通过 "len_mode" 指定：

   - 2'b00：保留（不要使用，因为行为未定义）
   - 2'b01：TLP 载荷的带宽
   - 2'b10：TLP 头的带宽
   - 2'b11：TLP 载荷和头的带宽

   例如，"len_mode=2" 表示只统计 TLP 头的带宽，"len_mode=3" 表示最终带宽数据由 TLP 头和载荷共同组成。未指定时默认值为 2'b11。

```

     $# perf stat -e hisi_pcie0_core0/rx_mrd_flux,port=0xffff,len_mode=0x1/ sleep 5

```
