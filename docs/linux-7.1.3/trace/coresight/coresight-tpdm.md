## 跟踪性能监控与诊断监视器（TPDM）


    :Author:   Jinlong Mao <quic_jinlmao@quicinc.com>
    :Date:     January 2023

### 硬件描述

TPDM（Trace Performance Monitoring and Diagnostics Monitor，简称 TPDM）作为各种数据集类型的数据采集组件。TPDM 的主要用例是从不同数据源收集数据，并将其发送给 TPDA 进行打包、加时间戳与汇聚。

### Sysfs 文件与目录

Root: `/sys/bus/coresight/devices/tpdm<N>`

----

:File:            `enable_source`（RW）
:Notes:
    - > 0 : 使能 TPDM 的数据集。

    - = 0 : 禁用 TPDM 的数据集。

:Syntax:
    `echo 1 > enable_source`

----

:File:            `integration_test`（wo）
:Notes:
    集成测试将为 tpdm 生成测试数据。

:Syntax:
    `echo value > integration_test`

    value -  1 或 2。

----
