
## Xilinx SD-FEC Driver


## Overview


该驱动支持 Zynq |Ultrascale+ (TM)| RFSoC 的 SD-FEC 集成模块。

   .. with trademark sign

有关 SD-FEC 核心功能的完整描述，请参阅 `SD-FEC Product Guide (PG256) <https://www.xilinx.com/cgi-bin/docs/ipdoc?c=sd_fec;v=latest;d=pg256-sdfec-integrated-block.pdf>`_

该驱动支持以下特性：

  - 获取集成模块的配置和状态信息
  - 配置 LDPC 码
  - 配置 Turbo 解码
  - 监视错误

SD-FEC 驱动缺失的特性、已知问题及限制如下：

  - 任何时刻对驱动的任何实例只允许单个打开的文件句柄
  - SD-FEC 集成模块的复位不由该驱动控制
  - 不支持共享 LDPC 码表回绕（wraparound）

设备树条目描述于：
`linux-xlnx/Documentation/devicetree/bindings/misc/xlnx,sd-fec.yaml <https://github.com/Xilinx/linux-xlnx/blob/master/Documentation/devicetree/bindings/misc/xlnx%2Csd-fec.yaml>`_


### Modes of Operation


该驱动在两种操作模式下与 SD-FEC 核心协同工作：

  - 运行时配置
  - 可编程逻辑（PL）初始化


#### Run-time Configuration


对于运行时配置，驱动的作用是允许软件应用执行以下操作：

 - 加载 Turbo 解码或 LDPC 编码或解码的配置参数
 - 激活 SD-FEC 核心
 - 监视 SD-FEC 核心是否出错
 - 获取 SD-FEC 核心的状态和配置

#### Programmable Logic (PL) Initialization


对于 PL 初始化，支持逻辑会加载 Turbo 解码或 LDPC 编码或解码的配置参数。驱动的作用是
允许软件应用执行以下操作：

 - 激活 SD-FEC 核心
 - 监视 SD-FEC 核心是否出错
 - 获取 SD-FEC 核心的状态和配置


## Driver Structure


该驱动提供一个平台设备，其中提供了 `probe` 和 `remove` 操作。

  - probe：用设备树条目更新配置寄存器，并确定核心当前的激活状态，例如核心是否被旁路或
    核心是否已启动。


该驱动定义了以下驱动文件操作，以提供用户应用接口：

  - open：实现限制，即每个 SD-FEC 实例在任何时刻只能打开一个文件描述符
  - release：允许打开另一个文件描述符，即当前文件描述符关闭之后
  - poll：提供一种监视 SD-FEC 错误事件的方法
  - unlocked_ioctl：提供以下 ioctl 命令，允许应用配置 SD-FEC 核心：

  - `XSDFEC_START_DEV`
  - `XSDFEC_STOP_DEV`
  - `XSDFEC_GET_STATUS`
  - `XSDFEC_SET_IRQ`
  - `XSDFEC_SET_TURBO`
  - `XSDFEC_ADD_LDPC_CODE_PARAMS`
  - `XSDFEC_GET_CONFIG`
  - `XSDFEC_SET_ORDER`
  - `XSDFEC_SET_BYPASS`
  - `XSDFEC_IS_ACTIVE`
  - `XSDFEC_CLEAR_STATS`
  - `XSDFEC_SET_DEFAULT_CONFIG`


## Driver Usage



### Overview


打开驱动后，用户应确定需要执行哪些操作来配置和激活 SD-FEC 核心，并确定驱动的配置。
以下是用户应当遵循的流程：

  - 确定配置
  - 设置顺序（order），如果尚未按期望配置
  - 设置 Turbo 解码、LDPC 编码或解码参数，具体取决于 SD-FEC 核心的配置方式，以及 SD-FEC
    是否尚未配置为 PL 初始化
  - 启用中断（如果尚未启用）
  - 旁路 SD-FEC 核心（如果需要）
  - 启动 SD-FEC 核心（如果尚未启动）
  - 获取 SD-FEC 核心状态
  - 监视中断
  - 停止 SD-FEC 核心


注意：在监视中断时，如果检测到需要复位的关键错误，则需要驱动加载默认配置。


### Determine Configuration


通过使用 ioctl `XSDFEC_GET_CONFIG` 确定 SD-FEC 核心的配置。

### Set the Order


设置顺序（order）决定了从输入到输出时 Block 的顺序如何变化。

设置顺序是通过使用 ioctl `XSDFEC_SET_ORDER` 完成的

只有在满足以下限制时才能设置顺序：

 - 由 ioctl `XSDFEC_GET_STATUS` 填充的 struct `xsdfec_status <xsdfec_status>` 的
   `state` 成员指示 SD-FEC 核心尚未 STARTED


### Add LDPC Codes


以下步骤说明如何向 SD-FEC 核心添加 LDPC 码：

 - 使用自动生成的参数填充所需 LDPC 码的 `struct xsdfec_ldpc_params <xsdfec_ldpc_params>`。
 - 为 LPDC 参数以及结构 `struct xsdfec_ldpc_params <xsdfec_ldpc_params>` 中的参数设置 SC、
   QA 和 LA 表偏移
 - 在结构 `struct xsdfec_ldpc_params <xsdfec_ldpc_params>` 中设置期望的 Code Id 值
 - 使用 ioctl `XSDFEC_ADD_LDPC_CODE_PARAMS` 添加 LPDC 码参数
 - 对于所应用的 LPDC 码参数，使用函数 `xsdfec_calculate_shared_ldpc_table_entry_size`
   计算共享 LPDC 码表的大小。这让用户能够确定共享表的使用情况，从而在选择下一个 LDPC 码
   参数的表偏移时可以选择未使用的表区域。
 - 对每个 LDPC 码参数重复上述步骤。

只有在满足以下限制时才能添加 LDPC 码：

 - 由 ioctl `XSDFEC_GET_CONFIG` 填充的 `struct xsdfec_config <xsdfec_config>` 的 `code`
   成员指示 SD-FEC 核心已配置为 LDPC
 - 由 ioctl `XSDFEC_GET_CONFIG` 填充的 `struct xsdfec_config <xsdfec_config>` 的
   `code_wr_protect` 指示未启用写保护
 - 由 ioctl `XSDFEC_GET_STATUS` 填充的 struct `xsdfec_status <xsdfec_status>` 的 `state`
   成员指示 SD-FEC 核心尚未启动

### Set Turbo Decode


配置 Turbo 解码参数是通过使用 ioctl `XSDFEC_SET_TURBO` 完成的，使用自动生成的参数填充
所需 Turbo 码的 `struct xsdfec_turbo <xsdfec_turbo>`。

只有在满足以下限制时才能添加 Turbo 解码：

 - 由 ioctl `XSDFEC_GET_CONFIG` 填充的 `struct xsdfec_config <xsdfec_config>` 的 `code`
   成员指示 SD-FEC 核心已配置为 TURBO
 - 由 ioctl `XSDFEC_GET_STATUS` 填充的 struct `xsdfec_status <xsdfec_status>` 的 `state`
   成员指示 SD-FEC 核心尚未 STARTED

### Enable Interrupts


启用或禁用中断是通过使用 ioctl `XSDFEC_SET_IRQ` 完成的。传递给 ioctl 的参数
`struct xsdfec_irq <xsdfec_irq>` 的成员用于设置和清除不同类别的中断。中断类别的控制
如下：

  - `enable_isr` 控制 `tlast` 中断
  - `enable_ecc_isr` 控制 ECC 中断

如果由 ioctl `XSDFEC_GET_CONFIG` 填充的 `struct xsdfec_config <xsdfec_config>` 的 `code`
成员指示 SD-FEC 核心已配置为 TURBO，则不需要启用 ECC 错误。

### Bypass the SD-FEC


旁路 SD-FEC 是通过使用 ioctl `XSDFEC_SET_BYPASS` 完成的

只有在满足以下限制时才能旁路 SD-FEC：

 - 由 ioctl `XSDFEC_GET_STATUS` 填充的 struct `xsdfec_status <xsdfec_status>` 的 `state`
   成员指示 SD-FEC 核心尚未 STARTED

### Start the SD-FEC core


通过使用 ioctl `XSDFEC_START_DEV` 启动 SD-FEC 核心

### Get SD-FEC Status


通过使用 ioctl `XSDFEC_GET_STATUS` 获取设备的 SD-FEC 状态，它将填充
`struct xsdfec_status <xsdfec_status>`

### Monitor for Interrupts


 - 使用 poll 系统调用监视中断。poll 系统调用等待中断将其唤醒，若无中断发生则超时。
 - 返回时 poll 的 `revents` 将指示 stats 和/或 state 是否已更新
  - `POLLPRI` 表示关键错误，用户应使用 `XSDFEC_GET_STATUS` 和 `XSDFEC_GET_STATS` 来确认
  - `POLLRDNORM` 表示发生了非关键错误，用户应使用 `XSDFEC_GET_STATS` 来确认
 - 使用 ioctl `XSDFEC_GET_STATS` 获取统计信息
  - 对于关键错误，`struct xsdfec_stats <xsdfec_stats>` 的 `isr_err_count` 或
    `uecc_count` 成员非零
  - 对于非关键错误，`struct xsdfec_stats <xsdfec_stats>` 的 `cecc_count` 成员非零
 - 使用 ioctl `XSDFEC_GET_STATUS` 获取状态
  - 对于关键错误，`xsdfec_status <xsdfec_status>` 的 `state` 将指示需要复位
 - 使用 ioctl `XSDFEC_CLEAR_STATS` 清除统计信息

如果检测到需要复位的关键错误，应用程序需要在复位后调用 ioctl `XSDFEC_SET_DEFAULT_CONFIG`，
而不需要调用 ioctl `XSDFEC_STOP_DEV`

注意：使用 poll 系统调用可避免通过 `XSDFEC_GET_STATS` 和 `XSDFEC_GET_STATUS` 进行忙循环

### Stop the SD-FEC Core


通过使用 ioctl `XSDFEC_STOP_DEV` 停止设备

### Set the Default Configuration


通过使用 ioctl `XSDFEC_SET_DEFAULT_CONFIG` 加载默认配置以恢复驱动。

### Limitations


用户不应复制 SD-FEC 设备文件句柄，例如 fork() 或 dup() 一个已创建 SD-FEC 文件句柄的进程。

## Driver IOCTLs


   :doc: XSDFEC_START_DEV

   :doc: XSDFEC_STOP_DEV

   :doc: XSDFEC_GET_STATUS

   :doc: XSDFEC_SET_IRQ

   :doc: XSDFEC_SET_TURBO

   :doc: XSDFEC_ADD_LDPC_CODE_PARAMS

   :doc: XSDFEC_GET_CONFIG

   :doc: XSDFEC_SET_ORDER

   :doc: XSDFEC_SET_BYPASS

   :doc: XSDFEC_IS_ACTIVE

   :doc: XSDFEC_CLEAR_STATS

   :doc: XSDFEC_GET_STATS

   :doc: XSDFEC_SET_DEFAULT_CONFIG

## Driver Type Definitions


   :internal:
