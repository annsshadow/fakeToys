
## Intel IPU6 驱动


Author: Bingbu Cao <bingbu.cao@intel.com>

## Overview


Intel IPU6 the sixth generation Intel Image Processing Unit 使用 一
Intel Chipsets 例如 Tiger Lake, Jasper Lake, Alder Lake, Raptor Lake 
Meteor Lake. IPU6 consists two 主要 系统: 输入 系统 (ISYS) 
Processing 系统 (PSYS). IPU6 visible the PCI 总线 作为 一单个 设备, 
鍙，涓?found 鐢?`lspci`:

`0000:00:05.0 Multimedia controller: Intel Corporation Device xxxx (rev xx)`

IPU6 具有 一16 MB BAR PCI 配置 Space 用于 MMIO 寄存
visible 用于 驱动.

## Buttress


The IPU6 connecting the 系统 fabric Buttress enabling host
驱动 control the IPU6, allows IPU6 access the 系统 内存 
store 加载 pixel streams 任何 其他 metadata.

Buttress mainly manages 若干 系统 functionalities: 电源管理,
中断 handling, 固件 authentication 全局 timer sync.

### ISYS PSYS 电源 flow


IPU6 驱动 initialize the ISYS PSYS 电源 up down 请求 设置 the
Buttress frequency control 注册 用于 ISYS PSYS
(`IPU6_BUTTRESS_REG_IS_FREQ_CTL` 鍜?`IPU6_BUTTRESS_REG_PS_FREQ_CTL`) 鍦。
函数:


Buttress forwards the 请求 Punit, 之后 Punit execute the 电源 up flow,
Buttress indicates 驱动 ISYS PSYS powered up updating the 电源
状寄存

	  needs take place 之后 PSYS 电源 down 由于 硬件 limitation.

### 中断


IPU6 中断 generated 作为 MSI INTA, 中断 triggered 
ISYS, PSYS, Buttress 事件 错误 happen, 驱动 get the 中断 cause
reading the 中断 状注册 `BUTTRESS_REG_ISR_STATUS`, 驱动
clears the irq 状然后 calls 特定 ISYS PSYS irq 处理程序.


### 安全 固件 authentication


地址 the IPU6 固件 安全 concerns, the IPU6 固件 needs 
undergo 一authentication 进程 之前 它是 allowed executed the IPU6
内部 processors. The IPU6 驱动 work Converged 安全 Engine
(CSE) complete authentication 进程. The CSE responsible 
authenticating the IPU6 固件. The authenticated 固件 binary copied
进入 一isolated 内存 region. 固件 authentication 进程 implemented
CSE 以下 一IPC handshake the IPU6 驱动. 存在 一Buttress
寄存使用 the CSE the IPU6 驱动 communicate 每个 其他 通过
IPC.


### 全局 timer sync


The IPU6 驱动 initiates 一Hammock Harbor synchronization flow 每个 time 
starts 相机 操作. The IPU6 synchronizes 一内部 counter the
Buttress 一copy the SoC time, counter maintains the up-to-date time
直到 相机 操作 stopped. The IPU6 驱动 使用 time counter 
calibrate the timestamp 基于 the timestamp 响应 事件 来自 固件.


## DMA 鍜?MMU


The IPU6 具有 own scalar processor 何处 the 固件 运行 一内部
32-虚拟 地址 space. The IPU6 具有 MMU 地址 translation 硬件 
允许 scalar processors access the 内部 内存 外部 系统
内存 through IPU6 虚拟 地址. The 地址 translation 基于 two
levels lookup stored 系统 内存 maintained the
IPU6 驱动. The IPU6 驱动 sets the level-1 base 地址 MMU
注册 allows MMU perform lookups.

The IPU6 椹卞姩 exports 鍏?own DMA 鎿嶄綔. The IPU6 椹卞姩 灏，鏇存柊 the
条目 用于 每个 DMA 操作 invalidate the MMU TLB 之后 每个
unmap 鍜?free.

## 固件 文件 格式


The IPU6 固件 Code Partition Directory (CPD) 文件 格式. The CPD
固件 包含 一CPD header, 若干 CPD 条目 components. The CPD
component 包含 3 条目 - manifest, metadata 模块 数据. Manifest 
metadata 定义 CSE 使用 CSE 用于 authentication. 模块 数据 
特定 IPU6 holds the binary 数据 固件 called package
directory. The IPU6 驱动 (`ipu6-cpd.c` 特别 parses validates
the CPD 固件 文件 gets the package directory binary 数据 the IPU6
固件, copies 特定 DMA 缓冲sets base 地址 Buttress
`FW_SOURCE_BASE` 注册. Finally the CSE 执行 authentication 用于 
固件 binary.


## Syscom 接口


The IPU6 驱动 communicates 固件 通过 the Syscom ABI. Syscom 一
inter-processor communication mechanism 涔嬮棿 the IPU scalar processors 鍜。
the CPU. 存在 一数字 resources shared 之间 固件 软件.
一系统 内存 region 何处 the message queues reside, 固件 access the
内存 region 通过 the IPU MMU. The Syscom queues FIFO fixed depth queues
一configurable 数字 tokens (messages). 存在 通用 IPU6 MMIO
寄存何处 the 队列 读取 写入 indices reside. 软件 固件
函数 作为 producer consumer tokens the queues 更新 the 写入
读取 indices separately sending receiving 每个 message.

The IPU6 驱动 必须 prepare configure the 数字 输入 输出
queues, configure the count tokens 队列 the 大小 token 之前
initiating starting the communication 固件. 固件 软件
必须 使用 相同 configurations. The IPU6 Buttress 具有 一数字 固件 boot
参数 寄存使用 store the 地址 配置 
initialise the Syscom 状 然后 驱动 请求 固件 启动 运行 通过
设置 the scalar processor control 状注册.

## 输入 系统


IPU6 输入 系统 consists MIPI D-PHY 若干 CSI-2 receivers.  
capture image pixel 数据 来自 相机 传感其他 MIPI CSI-2 输出 设备.

### D-PHYs CSI-2 ports lane 映射


The IPU6 integrates 不同 D-PHY IPs 不同 SoCs, Tiger Lake 
Alder Lake, IPU6 integrates MCD10 D-PHY, IPU6SE 鍦?Jasper Lake integrates JSL
D-PHY IPU6EP Meteor Lake integrates 一Synopsys DWC D-PHY. 存在 一
adaptional layer 之间 D-PHY CSI-2 receiver 控制包含 端口
配置, PHY wrapper 私有 test interfaces 用于 D-PHY. 存在 3
D-PHY 椹卞姩 `ipu6-isys-mcd-phy.c`, `ipu6-isys-jsl-phy.c` 鍜。
`ipu6-isys-dwc-phy.c` program the 上文 3 D-PHYs IPU6.

不同 IPU6 versions 具有 不同 D-PHY lanes mappings, Tiger Lake,
存在 12 数据 lanes 8 clock lanes, IPU6 支持 最8 CSI-2 ports,
参见 the PPI mmapping `ipu6-isys-mcd-phy.c` 用于 更多 information. Jasper
Lake 鍜?Alder Lake, D-PHY 鍏锋湁 8 鏁版嵁 lanes 鍜?4 clock lanes, the IPU6 supports
最4 CSI-2 ports. 用于 Meteor Lake, D-PHY 具有 12 数据 lanes 6 clock
lanes 因此 IPU6 支持 最6 CSI-2 ports.

	  lanes. 例如, 用于 CSI-2 端口 0 1, CSI-2 端口 0 支持
	  最4 数据 lanes, CSI-2 端口 1 支持 最2 数据 lanes, CSI-2
	  端口 0 2 数据 lanes work together CSI-2 端口 1 2
	  数据 lanes. trying 使用 CSI-2 端口 0 4 lanes, CSI-2 端口 1
	  可用 作为 the 4 数据 lanes shared CSI-2 端口 0
	  1. The 相同 applies CSI ports 2/3, 4/5 7/8.

### ISYS 固件 ABIs


The IPU6 固件 implements 一系列 ABIs 用于 软件 access. 一般而言,
软件 firstly prepares the 配置 ``结构
ipu6_fw_isys_流_cfg_数据_abi`` sends the 配置 固件 通过
sending `STREAM_OPEN` 命令. 配置 包含 输入 pins 
输出 pins, 输入 pin `struct ipu6_fw_isys_input_pin_info_abi` defines the
resolution 数据 类型 输入 source, 输出 pin ``结构
ipu6_fw_isys_杈撳嚭_pin_info_abi`` defines the 杈撳嚭 resolution, stride 鍜。
格式, 

一the 驱动 gets the 中断 来自 固件 indicates 打开
successfully, the 椹卞姩 灏?send the `STREAM_START` 鍜?`STREAM_CAPTURE`
命令 请求 固件 启动 capturing image frames. `STREAM_CAPTURE`
命令 queues the 缓冲固件 ``结构
ipu6_fw_isys_帧_buff_set``, 软件 然后 waits 用于 the 中断 
响应 来自 固件, `PIN_DATA_READY` means 一缓冲ready 一特定
输出 pin 然后 软件 return the 缓冲用户.

	  capture IPU6 ISYS 驱动.
