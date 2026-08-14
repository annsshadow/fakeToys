

## OMAP 3 图像信号处理器（ISP）驱动


Copyright |copy| 2010 Nokia Corporation

Copyright |copy| 2009 Texas Instruments, Inc.

联系人：Laurent Pinchart <laurent.pinchart@ideasonboard.com>、
Sakari Ailus <sakari.ailus@iki.fi>、David Cohen <dacohen@gmail.com>


### 事件


OMAP 3 ISP 驱动在 CCDC 和统计（AEWB、AF 和 histogram）子设备上支持
V4L2 事件接口。

CCDC 子设备在 HS_VS 中断上产生 V4L2_EVENT_FRAME_SYNC 类型事件，用于
指示帧开始。该驱动的早期版本为此使用 V4L2_EVENT_OMAP3ISP_HS_VS。该事件
恰好在 CCDC 模块中接收到帧的第一行时触发。该事件可以在 CCDC 子设备上
订阅。

（使用并行接口时，必须正确配置 VS 信号极性。使用串行接收器时会自动正确。）

每个统计子设备都能够产生事件。每当用户空间应用程序可以使用
VIDIOC_OMAP3ISP_STAT_REQ IOCTL 将统计缓冲区出队时，就会生成一个事件。
可用的事件有：

- V4L2_EVENT_OMAP3ISP_AEWB
- V4L2_EVENT_OMAP3ISP_AF
- V4L2_EVENT_OMAP3ISP_HIST

这些 ioctl 的事件数据类型为 struct omap3isp_stat_event_status。如果计算
统计信息出错，仍会像往常一样产生事件，但不会有关联的统计缓冲区。在这种
情况下，omap3isp_stat_event_status.buf_err 被设为非零。


### 私有 IOCTL


OMAP 3 ISP 驱动在可能且可行的情况下支持标准的 V4L2 IOCTL 和控件。然而，
ISP 提供的许多功能并不属于标准 IOCTL——例如伽马表以及统计采集的配置。

一般来说，对于每个包含硬件相关功能的模块，都有一个私有的 ioctl 用于配置。

支持的私有 IOCTL 如下：

- VIDIOC_OMAP3ISP_CCDC_CFG
- VIDIOC_OMAP3ISP_PRV_CFG
- VIDIOC_OMAP3ISP_AEWB_CFG
- VIDIOC_OMAP3ISP_HIST_CFG
- VIDIOC_OMAP3ISP_AF_CFG
- VIDIOC_OMAP3ISP_STAT_REQ
- VIDIOC_OMAP3ISP_STAT_EN

这些 ioctl 所使用的参数结构体在 include/linux/omap3isp.h 中描述。与特定
ISP 模块相关的 ISP 本身的详细功能在《技术参考手册》（TRM）中描述——文档
末尾列出了这些手册。

虽然可以在完全不使用这些私有 IOCTL 的情况下使用 ISP 驱动，但以此方式无法
获得最佳图像质量。如果不使用相应的私有 IOCTL 进行配置，就无法使用 AEWB、
AF 和 histogram 模块。


### CCDC 与预览模块 IOCTL


VIDIOC_OMAP3ISP_CCDC_CFG 和 VIDIOC_OMAP3ISP_PRV_CFG IOCTL 分别用于配置、
启用和禁用 CCDC 与预览模块中的功能。这两个 IOCTL 都控制其对应模块中的
多个功能。VIDIOC_OMAP3ISP_CCDC_CFG IOCTL 接受一个指向 struct
omap3isp_ccdc_update_config 的指针作为参数。类似地，VIDIOC_OMAP3ISP_PRV_CFG
接受一个指向 struct omap3isp_prev_update_config 的指针。这两个结构体的定义
见 [#]_。

结构体中的 update 字段指明是否更新该特定功能的配置，flag 字段指明是启用
还是禁用该功能。

update 和 flag 位掩码接受以下值。CCDC 和预览模块中的每个独立功能都关联
一个 flag（禁用或启用，属于结构体 flag 字段的一部分）以及一个指向该功能
配置数据的指针。

VIDIOC_OMAP3ISP_CCDC_CFG 的有效 update 和 flag 字段值在此列出。这些值可以
用或运算组合，以在同一次 IOCTL 调用中配置多个功能。

- OMAP3ISP_CCDC_ALAW
- OMAP3ISP_CCDC_LPF
- OMAP3ISP_CCDC_BLCLAMP
- OMAP3ISP_CCDC_BCOMP
- OMAP3ISP_CCDC_FPC
- OMAP3ISP_CCDC_CULL
- OMAP3ISP_CCDC_CONFIG_LSC
- OMAP3ISP_CCDC_TBL_LSC

VIDIOC_OMAP3ISP_PRV_CFG 的对应值如下：

- OMAP3ISP_PREV_LUMAENH
- OMAP3ISP_PREV_INVALAW
- OMAP3ISP_PREV_HRZ_MED
- OMAP3ISP_PREV_CFA
- OMAP3ISP_PREV_CHROMA_SUPP
- OMAP3ISP_PREV_WB
- OMAP3ISP_PREV_BLKADJ
- OMAP3ISP_PREV_RGB2RGB
- OMAP3ISP_PREV_COLOR_CONV
- OMAP3ISP_PREV_YC_LIMIT
- OMAP3ISP_PREV_DEFECT_COR
- OMAP3ISP_PREV_GAMMABYPASS
- OMAP3ISP_PREV_DRK_FRM_CAPTURE
- OMAP3ISP_PREV_DRK_FRM_SUBTRACT
- OMAP3ISP_PREV_LENS_SHADING
- OMAP3ISP_PREV_NF
- OMAP3ISP_PREV_GAMMA

启用某项功能时，其关联的配置指针不得为 NULL。禁用某项功能时，该配置指针
会被忽略。


### 统计模块 IOCTL


统计子设备比其它子设备提供更动态的配置选项。它们可以在流水线处于
streaming 状态时启用、禁用和重新配置。

统计模块始终从 CCDC 获取输入图像数据（因为未实现 histogram 内存读取）。
用户可以使用私有 IOCTL 从统计子设备节点将统计信息出队。

AEWB、AF 和 histogram 子设备提供的私有 IOCTL 在很大程度上反映了 ISP 硬件
所提供的寄存器级接口。有些方面纯粹与驱动实现相关，接下来将讨论这些方面。

### VIDIOC_OMAP3ISP_STAT_EN


该私有 IOCTL 启用/禁用一个统计模块。如果在 streaming 之前发出此请求，它
将在流水线开始 streaming 时立即生效。如果流水线已经在 streaming，它将在
CCDC 变为空闲时立即生效。

### VIDIOC_OMAP3ISP_AEWB_CFG、VIDIOC_OMAP3ISP_HIST_CFG 与 VIDIOC_OMAP3ISP_AF_CFG


这些 IOCTL 用于配置各模块。它们要求用户应用程序对硬件有深入的了解。大部分
字段的说明可以在 OMAP 的 TRM 中找到。上述所有配置用私有 IOCTL 共有的以下
两个字段需要进一步说明，以便更好地理解，因为它们不属于 TRM 的内容。

omap3isp_[h3a_af/h3a_aewb/hist]\_config.buf_size：

这些模块在内部处理自己的缓冲区。模块数据输出所需的缓冲区大小取决于所请求
的配置。尽管驱动支持在 streaming 时重新配置，但如果模块已启用，它不支持
需要比内部已分配缓冲区更大尺寸的重新配置，这种情况下会返回 -EBUSY。为避免
这种情况，可以禁用/重新配置/启用模块，或者在模块禁用期间于首次配置时请求
所需的缓冲区大小。

内部缓冲区大小的分配会考虑所请求配置的最小缓冲区大小，以及 buf_size 字段
设置的值。如果 buf_size 字段超出 [最小, 最大] 缓冲区大小范围，则会被钳制
以适配该范围。随后驱动会选择最大的值。修正后的 buf_size 值会被写回用户
应用程序。

omap3isp_[h3a_af/h3a_aewb/hist]\_config.config_counter：

由于配置不会与请求同步生效，驱动必须提供一种方式来跟踪此信息，以提供更准确
的数据。在请求某项配置后，返回给用户空间应用程序的 config_counter 将是与该
请求关联的唯一值。当用户应用程序收到缓冲区可用的事件，或请求新的缓冲区时，
此 config_counter 用于将缓冲区数据与配置进行匹配。

### VIDIOC_OMAP3ISP_STAT_REQ


将内部缓冲区队列中最早可用的数据发送到用户空间，并随后丢弃该缓冲区。字段
omap3isp_stat_data.frame_number 与视频缓冲区的 field_count 相匹配。


### 参考资料


