
## 跟踪性能监控与诊断聚合器（TPDA

    :Author:   Jinlong Mao <quic_jinlmao@quicinc.com>
    :Date:     January 2023

### 硬件描述


TPDA（跟踪性能监控与诊断聚合器，trace performance monitoring and diagnostics aggregator）简而言之，充当性能监控与诊断网络规范的仲裁与打包引擎TPDA 的主要用途是Monitor 数据进行打包、汇聚（funneling）与时间戳标记
### Sysfs 文件与目

根目录：`/sys/bus/coresight/devices/tpda<N>`

### 配置细节


tpdm tpda 节点应在 coresight 路径 "/sys/bus/coresight/devices" 下查看例如/sys/bus/coresight/devices # ls -l | grep tpd
tpda0 -> ../../../devices/platform/soc@0/6004000.tpda/tpda0
tpdm0 -> ../../../devices/platform/soc@0/6c08000.mm.tpdm/tpdm0

我们可以使用类似于下面的命令来验TPDM先启coresight sink。执行以下命令后，连接到 tpdm tpda 端口将被启用
echo 1 > /sys/bus/coresight/devices/tmc_etf0/enable_sink
echo 1 > /sys/bus/coresight/devices/tpdm0/enable_source
echo 1 > /sys/bus/coresight/devices/tpdm0/integration_test
echo 2 > /sys/bus/coresight/devices/tpdm0/integration_test

测试数据将被收集到已启用coresight sink 中如果在执integration_test sink rwp 寄存器持续更新（通过 cat tmc_etf0/mgmt/rwp），则意味着有从 TPDM sink 的数据生成
tpdm sink 之间必须存在一tpda。当同一 HW 块中存在其他跟踪事件硬件组件tpdm 一起时，tpdm 与这些硬件组件将连接coresight funnel。当 HW 块中只有 tpdm 跟踪硬件时，tpdm 将直接连接到 tpda