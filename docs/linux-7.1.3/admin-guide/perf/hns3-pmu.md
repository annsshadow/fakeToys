## HNS3 性能监控单元（PMU
HNS3（HiSilicon network system 3）性能监控单元（PMU）是一个端点（End Point）设备，
用于收集 HiSilicon SoC NIC 的性能统计信息。在 Hip09 上，每个 SICL（Super I/O
cluster，超I/O 簇）都有一PMU 设备
HNS3 PMU 支持收集带宽、延迟、包速率和中断速率等性能统计信息
每个 HNS3 PMU 支持 8 个硬件事件
## HNS3 PMU 驱动

```

  /sys/bus/event_source/devices/hns3_pmu_sicl_<sicl_id>

```
PMU 驱动sysfs 中提供可用事件、过滤模式、格式、标识符（identifier）和 cpumask 描述
"events" 目录描述perf list 中显示的所有受支持事件的事件码
"filtermode" 目录描述了每个事件所支持的过滤模式
"format" 目录描述perf_event_attr 结构config（事件）config1（过滤选项字段的所有格式
"identifier" 文件显示 PMU 硬件设备的版本
"bdf_min" "bdf_max" 文件显示每个 pmu 设备所支持bdf 范围
"hw_clk_freq" 文件显示每个 pmu 设备的硬件时钟频率
```

  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/events/dly_tx_normal_to_mac_time
  config=0x00204
  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/events/dly_tx_normal_to_mac_packet_num
  config=0x10204

```
每个性能统计量都有一对事件，用于获取两个值，从而在计算（用户空间）中算出真实的
性能数据
config 0~15 位是真正的硬件事件码。如果两个事件的 config 0~15 位取值相同，
就表示它们是一对事件。config 的第 16 位表示获取硬件事件计数器 0 还是计数1
在用户空间获得事件对的两个值之后，计算公式如下
```

  counter 0 / counter 1

```
```

  $# cat /sys/bus/event_source/devices/hns3_pmu_sicl_0/filtermode/bw_ssu_rpu_byte_num
  filter mode supported: global/port/port-tc/func/func-queue/

```
```

  $# perf list
  hns3_pmu_sicl_0/bw_ssu_rpu_byte_num/ [kernel PMU event]
  hns3_pmu_sicl_0/bw_ssu_rpu_time/     [kernel PMU event]
  ------------------------------------------

  $# perf stat -g -e hns3_pmu_sicl_0/bw_ssu_rpu_byte_num,global=1/ -e hns3_pmu_sicl_0/bw_ssu_rpu_time,global=1/ -I 1000
  or
  $# perf stat -g -e hns3_pmu_sicl_0/config=0x00002,global=1/ -e hns3_pmu_sicl_0/config=0x10002,global=1/ -I 1000


```
### 过滤模式

1. global 模式
PMU 收集 IO DIE 的所HNS3 PCIe 功能的性能统计信息。将 "global" 过滤选项设为 1
即可启用此模式```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,global=1/ -I 1000

```
2. port 模式
PMU 收集整个一个物理端口的性能统计信息。端id mac id 相同。在此模式下tc"
过滤选项必须设为 0xF，这tc 代表流量类别（traffic class）```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,port=0,tc=0xF/ -I 1000

```
3. port-tc 模式
PMU 收集物理端口某一tc 的性能统计信息。端id mac id 相同。在此模式下tc"
过滤选项必须设为 0 ~ 7```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,port=0,tc=0/ -I 1000

```
4. func 模式
PMU 收集一PF/VF 的性能统计信息。功id PF/VF BDF，其```

  func = (bus << 8) + (device << 3) + (function)

```
例如  BDF         func
  35:00.0    0x3500
  35:00.1    0x3501
  35:01.0    0x3508

在此模式下，"queue" 过滤选项必须设为 0xFFFF```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,bdf=0x3500,queue=0xFFFF/ -I 1000

```
5. func-queue 模式
PMU 收集一PF/VF 的某一个队列的性能统计信息。功id PF/VF BDFqueue"
过滤选项必须设为该功能确切的队列 id```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x1020F,bdf=0x3500,queue=0/ -I 1000

```
6. func-intr 模式
PMU 收集一PF/VF 的某一次中断的性能统计信息。功id PF/VF BDFintr"
过滤选项必须设为该功能确切的中断 id```

  $# perf stat -a -e hns3_pmu_sicl_0/config=0x00301,bdf=0x3500,intr=0/ -I 1000

```
