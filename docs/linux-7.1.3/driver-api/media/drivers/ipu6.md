
## Intel IPU6 驱动


Author: Bingbu Cao <bingbu.cao@intel.com>

## Overview


Intel IPU6 是 the sixth generation 的 Intel Image Processing Unit 使用 在 一些
Intel Chipsets 例如 Tiger Lake, Jasper Lake, Alder Lake, Raptor Lake 和
Meteor Lake. IPU6 consists 的 two 主要 系统: 输入 系统 (ISYS) 和
Processing 系统 (PSYS). IPU6 是 visible 在 the PCI 总线 作为 一个 单个 设备, 它
可 为 found 由 `lspci`:

`0000:00:05.0 Multimedia controller: Intel Corporation Device xxxx (rev xx)`

IPU6 具有 一个 16 MB BAR 在 PCI 配置 Space 用于 MMIO 寄存器 其 是
visible 用于 驱动.

## Buttress


The IPU6 是 connecting 到 the 系统 fabric 与 Buttress 其 是 enabling host
驱动 到 control the IPU6, 它 也 allows IPU6 access the 系统 内存 到
store 和 加载 帧 pixel streams 和 任何 其他 metadata.

Buttress mainly manages 若干 系统 functionalities: 电源管理,
中断 handling, 固件 authentication 和 全局 timer sync.

### ISYS 和 PSYS 电源 flow


IPU6 驱动 initialize the ISYS 和 PSYS 电源 up 或 down 请求 由 设置 the
Buttress frequency control 注册 用于 ISYS 和 PSYS
(`IPU6_BUTTRESS_REG_IS_FREQ_CTL` 和 `IPU6_BUTTRESS_REG_PS_FREQ_CTL`) 在
函数:


Buttress forwards the 请求 到 Punit, 之后 Punit execute the 电源 up flow,
Buttress indicates 驱动 该 ISYS 或 PSYS 是 powered up 由 updating the 电源
状态 寄存器.

	  needs take place 之后 PSYS 电源 down 由于 硬件 limitation.

### 中断


IPU6 中断 可 为 generated 作为 MSI 或 INTA, 中断 将 为 triggered 当
ISYS, PSYS, Buttress 事件 或 错误 happen, 驱动 可 get the 中断 cause
由 reading the 中断 状态 注册 `BUTTRESS_REG_ISR_STATUS`, 驱动
clears the irq 状态 和 然后 calls 特定 ISYS 或 PSYS irq 处理程序.


### 安全 和 固件 authentication


到 地址 the IPU6 固件 安全 concerns, the IPU6 固件 needs 到
undergo 一个 authentication 进程 之前 它是 allowed 到 executed 在 the IPU6
内部 processors. The IPU6 驱动 将 work 与 Converged 安全 Engine
(CSE) 到 complete authentication 进程. The CSE 是 responsible 的
authenticating the IPU6 固件. The authenticated 固件 binary 是 copied
进入 一个 isolated 内存 region. 固件 authentication 进程 是 implemented
由 CSE 以下 一个 IPC handshake 与 the IPU6 驱动. 存在 一些 Buttress
寄存器 使用 由 the CSE 和 the IPU6 驱动 到 communicate 与 每个 其他 通过
IPC.


### 全局 timer sync


The IPU6 驱动 initiates 一个 Hammock Harbor synchronization flow 每个 time 它
starts 相机 操作. The IPU6 将 synchronizes 一个 内部 counter 在 the
Buttress 与 一个 copy 的 the SoC time, 此 counter maintains the up-to-date time
直到 相机 操作 是 stopped. The IPU6 驱动 可 使用 此 time counter 到
calibrate the timestamp 基于 the timestamp 在 响应 事件 来自 固件.


## DMA 和 MMU


The IPU6 具有 其 own scalar processor 何处 the 固件 运行 在 和 一个 内部
32-位 虚拟 地址 space. The IPU6 具有 MMU 地址 translation 硬件 到
允许 该 scalar processors 到 access the 内部 内存 和 外部 系统
内存 through IPU6 虚拟 地址. The 地址 translation 是 基于 two
levels 的 页 lookup 表 stored 在 系统 内存 其 是 maintained 由 the
IPU6 驱动. The IPU6 驱动 sets the level-1 页 表 base 地址 到 MMU
注册 和 allows MMU 到 perform 页 表 lookups.

The IPU6 驱动 exports 其 own DMA 操作. The IPU6 驱动 将 更新 the
页 表 条目 用于 每个 DMA 操作 和 invalidate the MMU TLB 之后 每个
unmap 和 free.

## 固件 文件 格式


The IPU6 固件 是 在 Code Partition Directory (CPD) 文件 格式. The CPD
固件 包含 一个 CPD header, 若干 CPD 条目 和 components. The CPD
component 包含 3 条目 - manifest, metadata 和 模块 数据. Manifest 和
metadata 是 定义 由 CSE 和 使用 由 CSE 用于 authentication. 模块 数据 是
特定 到 IPU6 其 holds the binary 数据 的 固件 called package
directory. The IPU6 驱动 (`ipu6-cpd.c` 特别是) parses 和 validates
the CPD 固件 文件 和 gets the package directory binary 数据 的 the IPU6
固件, copies 它 到 特定 DMA 缓冲区 和 sets 其 base 地址 到 Buttress
`FW_SOURCE_BASE` 注册. Finally the CSE 将 执行 authentication 用于 此
固件 binary.


## Syscom 接口


The IPU6 驱动 communicates 与 固件 通过 the Syscom ABI. Syscom 是 一个
inter-processor communication mechanism 之间 the IPU scalar processors 和
the CPU. 存在 一个 数字 的 resources shared 之间 固件 和 软件.
一个 系统 内存 region 何处 the message queues reside, 固件 可 access the
内存 region 通过 the IPU MMU. The Syscom queues 是 FIFO fixed depth queues
与 一个 configurable 数字 的 tokens (messages). 存在 也 通用 IPU6 MMIO
寄存器 何处 the 队列 读取 和 写入 indices reside. 软件 和 固件
函数 作为 producer 和 consumer 的 tokens 在 the queues 和 更新 the 写入
和 读取 indices separately 当 sending 或 receiving 每个 message.

The IPU6 驱动 必须 prepare 和 configure the 数字 的 输入 和 输出
queues, configure the count 的 tokens 每 队列 和 the 大小 的 每 token 之前
initiating 和 starting the communication 与 固件. 固件 和 软件
必须 使用 相同 configurations. The IPU6 Buttress 具有 一个 数字 的 固件 boot
参数 寄存器 其 可 为 使用 到 store the 地址 的 配置 和
initialise the Syscom 状态, 然后 驱动 可 请求 固件 到 启动 和 运行 通过
设置 the scalar processor control 状态 注册.

## 输入 系统


IPU6 输入 系统 consists 的 MIPI D-PHY 和 若干 CSI-2 receivers.  它 可
capture image pixel 数据 来自 相机 传感器 或 其他 MIPI CSI-2 输出 设备.

### D-PHYs 和 CSI-2 ports lane 映射


The IPU6 integrates 不同 D-PHY IPs 在 不同 SoCs, 在 Tiger Lake 和
Alder Lake, IPU6 integrates MCD10 D-PHY, IPU6SE 在 Jasper Lake integrates JSL
D-PHY 和 IPU6EP 在 Meteor Lake integrates 一个 Synopsys DWC D-PHY. 存在 一个
adaptional layer 之间 D-PHY 和 CSI-2 receiver 控制器 其 包含 端口
配置, PHY wrapper 或 私有 test interfaces 用于 D-PHY. 存在 3
D-PHY 驱动 `ipu6-isys-mcd-phy.c`, `ipu6-isys-jsl-phy.c` 和
`ipu6-isys-dwc-phy.c` program the 上文 3 D-PHYs 在 IPU6.

不同 IPU6 versions 具有 不同 D-PHY lanes mappings, 在 Tiger Lake,
存在 12 数据 lanes 和 8 clock lanes, IPU6 支持 最大 8 CSI-2 ports,
参见 the PPI mmapping 在 `ipu6-isys-mcd-phy.c` 用于 更多 information. 在 Jasper
Lake 和 Alder Lake, D-PHY 具有 8 数据 lanes 和 4 clock lanes, the IPU6 supports
最大 4 CSI-2 ports. 用于 Meteor Lake, D-PHY 具有 12 数据 lanes 和 6 clock
lanes 因此 IPU6 支持 最大 6 CSI-2 ports.

	  lanes. 例如, 用于 CSI-2 端口 0 和 1, CSI-2 端口 0 支持
	  最大 4 数据 lanes, CSI-2 端口 1 支持 最大 2 数据 lanes, CSI-2
	  端口 0 与 2 数据 lanes 可 work together 与 CSI-2 端口 1 与 2
	  数据 lanes. 若 trying 到 使用 CSI-2 端口 0 与 4 lanes, CSI-2 端口 1
	  将 不 为 可用 作为 the 4 数据 lanes 是 shared 由 CSI-2 端口 0
	  和 1. The 相同 applies 到 CSI ports 2/3, 4/5 和 7/8.

### ISYS 固件 ABIs


The IPU6 固件 implements 一个 系列 的 ABIs 用于 软件 access. 一般而言,
软件 firstly prepares the 流 配置 ``结构体
ipu6_fw_isys_流_cfg_数据_abi`` 和 sends the 配置 到 固件 通过
sending `STREAM_OPEN` 命令. 流 配置 包含 输入 pins 和
输出 pins, 输入 pin `struct ipu6_fw_isys_input_pin_info_abi` defines the
resolution 和 数据 类型 的 输入 source, 输出 pin ``结构体
ipu6_fw_isys_输出_pin_info_abi`` defines the 输出 resolution, stride 和
帧 格式, 等.

一旦 the 驱动 gets the 中断 来自 固件 该 indicates 流 打开
successfully, the 驱动 将 send the `STREAM_START` 和 `STREAM_CAPTURE`
命令 到 请求 固件 到 启动 capturing image frames. `STREAM_CAPTURE`
命令 queues the 缓冲区 到 固件 与 ``结构体
ipu6_fw_isys_帧_buff_set``, 软件 然后 waits 用于 the 中断 和
响应 来自 固件, `PIN_DATA_READY` means 一个 缓冲区 是 ready 在 一个 特定
输出 pin 和 然后 软件 可 return the 缓冲区 到 用户.

	  capture 由 IPU6 ISYS 驱动.
