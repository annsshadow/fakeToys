## 通用闪存存储（Universal Flash Storage）


   1. 概述（Overview）
   2. UFS 架构概述（UFS Architecture Overview）
     2.1 应用层（Application Layer）
     2.2 UFS 传输协议（UTP）层
     2.3 UFS 互连（UIC）层
   3. UFSHCD 概述
     3.1 UFS 控制器初始化
     3.2 UTP 传输请求
     3.3 UFS 错误处理
     3.4 SCSI 错误处理
   4. BSG 支持
   5. UFS 参考时钟频率配置


## 1. 概述（Overview）


通用闪存存储（UFS）是针对闪存设备的存储规范。它旨在为智能手机和平板电脑等移动设备中基于嵌入式与可移动闪存的存储提供统一的存储接口。该规范由 JEDEC 固态技术协会定义。UFS 基于 MIPI M-PHY 物理层标准。UFS 使用 MIPI M-PHY 作为物理层，使用 MIPI Unipro 作为链路层。

UFS 的主要目标是提供：

 - 优化的性能：

   UFS 1.0 和 1.1 版本的目标性能如下：

   - 必须支持 Gear1（速率 A：1248Mbps，速率 B：1457.6Mbps）
   - 可选支持 Gear2（速率 A：2496Mbps，速率 B：2915.2Mbps）

   未来的标准版本，

   - Gear3（速率 A：4992Mbps，速率 B：5830.4Mbps）

 - 低功耗
 - 高随机 IOPS 和低延迟


## 2. UFS 架构概述（UFS Architecture Overview）


UFS 拥有一个基于 SCSI SAM-5 架构模型的层次化通信架构。

UFS 通信架构由以下层次组成。

### 2.1 应用层（Application Layer）


  应用层由 UFS 命令集层（UCS）、任务管理器和设备管理器等组成。UFS 接口被设计为协议无关，但 SCSI 被选为 UFS 协议层 1.0 和 1.1 版本的基线协议。

  UFS 支持由 SPC-4 和 SBC-3 定义的 SCSI 命令的一个子集。

  - UCS：
     它处理 UFS 规范支持的 SCSI 命令。
  - 任务管理器（Task manager）：
     它处理由 UFS 定义的、用于命令队列控制的任务管理功能。
  - 设备管理器（Device manager）：
     它处理设备级操作和设备配置操作。设备级操作主要涉及设备电源管理操作以及对互连层的命令。设备级配置涉及处理用于修改和检索设备配置信息的查询请求。

### 2.2 UFS 传输协议（UTP）层


  UTP 层通过服务访问点（Service Access Points）为上层提供服务。UTP 为上层定义了 3 个服务访问点。

  - UDM_SAP：设备管理器服务访问点，暴露给设备管理器用于设备级操作。这些设备级操作通过查询请求完成。
  - UTP_CMD_SAP：命令服务访问点，暴露给 UFS 命令集层（UCS）以传输命令。
  - UTP_TM_SAP：任务管理服务访问点，暴露给任务管理器以传输任务管理功能。

  UTP 通过 UFS 协议信息单元（UPIU）传输消息。

### 2.3 UFS 互连（UIC）层


  UIC 是 UFS 层次化架构中的最底层。它处理 UFS 主机与 UFS 设备之间的连接。UIC 由 MIPI UniPro 和 MIPI M-PHY 组成。UIC 为上层提供 2 个服务访问点：

  - UIC_SAP：在 UFS 主机与 UFS 设备之间传输 UPIU。
  - UIO_SAP：向 Unipro 层发出命令。


## 3. UFSHCD 概述


UFS 主机控制器驱动（UFSHCD）基于 Linux SCSI 框架。UFSHCD 是一个底层设备驱动，充当 SCSI 中间层与基于 PCIe 的 UFS 主机控制器之间的接口。

当前 UFSHCD 实现支持以下功能：

### 3.1 UFS 控制器初始化


  初始化模块将 UFS 主机控制器带入活动状态，并准备控制器在 UFSHCD 与 UFS 设备之间传输命令/响应。

### 3.2 UTP 传输请求


  传输请求处理模块接收来自 SCSI 中间层的 SCSI 命令，构造 UPIU 并将其发送给 UFS 主机控制器。同时，该模块将以 UPIU 形式从 UFS 主机控制器接收到的响应进行解码，并将命令状态通知 SCSI 中间层。

### 3.3 UFS 错误处理


  错误处理模块处理主机控制器致命错误、设备致命错误以及与 UIC 互连层相关的错误。

### 3.4 SCSI 错误处理


  这通过注册到 SCSI 中间层的 UFSHCD SCSI 错误处理例程完成。由 SCSI 中间层发出的一些错误处理命令示例包括中止任务（Abort task）、LUN 复位和主机复位。用于执行这些任务的 UFSHCD 例程通过 .eh_abort_handler、.eh_device_reset_handler 和 .eh_host_reset_handler 注册到 SCSI 中间层。

在本版本的 UFSHCD 中，查询请求和电源管理功能尚未实现。

## 4. BSG 支持


该传输驱动支持与 UFS 设备交换 UFS 协议信息单元（UPIU）。通常，用户空间会分配 struct ufs_bsg_request 和 struct ufs_bsg_reply（见 ufs_bsg.h）分别作为 request_upiu 和 reply_upiu。填写这些 UPIU 应符合 JEDEC 规范 UFS2.1 第 10.7 节。
**Caveat emptor（买者自负）**：驱动不再做进一步的输入校验，而是按原样将 UPIU 发送给设备。在 /dev/ufs-bsg 打开 bsg 设备，并

```
	io_hdr_v4.guard = 'Q';
	io_hdr_v4.protocol = BSG_PROTOCOL_SCSI;
	io_hdr_v4.subprotocol = BSG_SUB_PROTOCOL_SCSI_TRANSPORT;
	io_hdr_v4.response = (__u64)reply_upiu;
	io_hdr_v4.max_response_len = reply_len;
	io_hdr_v4.request_len = request_len;
	io_hdr_v4.request = (__u64)request_upiu;
	if (dir == SG_DXFER_TO_DEV) {
		io_hdr_v4.dout_xfer_len = (uint32_t)byte_cnt;
		io_hdr_v4.dout_xferp = (uintptr_t)(__u64)buff;
	} else {
		io_hdr_v4.din_xfer_len = (uint32_t)byte_cnt;
		io_hdr_v4.din_xferp = (uintptr_t)(__u64)buff;
	}
```

如果你希望读取或写入描述符，请使用 sg_io_v4 相应的 xferp。

与 ufs-bsg 端点交互并使用其基于 UPIU 协议的用户空间工具位于：

	https://github.com/westerndigitalcorporation/ufs-tool

有关该工具及其所支持功能的更详细信息，请参见该工具的 README。

UFS 规范可在以下位置找到：

- UFS - http://www.jedec.org/sites/default/files/docs/JESD220.pdf
- UFSHCI - http://www.jedec.org/sites/default/files/docs/JESD223.pdf

## 5. UFS 参考时钟频率配置


设备树（Devicetree）可以在 UFS 控制器节点下定义一个名为 "ref_clk" 的时钟，用以指定 UFS 存储部件的期望参考时钟频率。基于 ACPI 的系统可以使用名为 "ref-clk-freq" 的 ACPI 设备特定数据（Device-Specific Data）属性来指定频率。两种方式下，该值都被解释为以 Hz 为单位的频率，并且必须与 UFS 规范中给出的某个值匹配。UFS 子系统会在执行通用控制器初始化时尝试读取该值。如果该值可用，UFS 子系统将确保 UFS 存储设备的 bRefClkFreq 属性被相应设置，并在不匹配时修改它。
