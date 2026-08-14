######## V4L2_META_FMT_IPU3_PARAMS ('ip3p'), V4L2_META_FMT_IPU3_3A ('ip3s')


## 3A 统计


IPU3 ImgU 3A 统计加速器在输入的 Bayer 帧上收集不同的统计信息。这些统计信息通过 “ipu3-imgu [^01^] 3a
stat” 元数据捕获视频节点，使用 `v4l2_meta_format` 接口获取。它们的格式如 `ipu3_uapi_stats_3a`
结构所描述。

收集的统计信息包括 AWB（自动白平衡）RGBS（红、绿、蓝和饱和度测量）单元、AWB 滤波器响应、
AF（自动对焦）滤波器响应，以及 AE（自动曝光）直方图。

struct `ipu3_uapi_4a_config` 保存所有可配置参数。


	struct ipu3_uapi_stats_3a {
		struct ipu3_uapi_awb_raw_buffer awb_raw_buffer;
		struct ipu3_uapi_ae_raw_buffer_aligned ae_raw_buffer[IPU3_UAPI_MAX_STRIPES];
		struct ipu3_uapi_af_raw_buffer af_raw_buffer;
		struct ipu3_uapi_awb_fr_raw_buffer awb_fr_raw_buffer;
		struct ipu3_uapi_4a_config stats_4a_config;
		__u32 ae_join_buffers;
		__u8 padding[^28^];
		struct ipu3_uapi_stats_3a_bubble_info_per_stripe stats_3a_bubble_per_stripe;
		struct ipu3_uapi_ff_status stats_3a_status;
	};


## 流水线参数


流水线参数通过 “ipu3-imgu [^01^] parameters” 元数据输出视频节点，使用 `v4l2_meta_format`
接口传递。它们的格式如 `ipu3_uapi_params` 结构所描述。

此处描述的 3A 统计和流水线参数都与底层的相机子系统（CSS）API 紧密相关。它们通常由包含重要
调优工具的专用用户空间库消费和产出，从而让开发人员不必被底层的硬件和算法细节所困扰。


	struct ipu3_uapi_params {
		/** 标志下方哪些设置将被应用 **/
		struct ipu3_uapi_flags use;

		/** 加速器集群参数 **/
		struct ipu3_uapi_acc_param acc_param;

		/** ISP 向量地址空间参数 **/
		struct ipu3_uapi_isp_lin_vmem_params lin_vmem_params;
		struct ipu3_uapi_isp_tnr3_vmem_params tnr3_vmem_params;
		struct ipu3_uapi_isp_xnr3_vmem_params xnr3_vmem_params;

		/** ISP 数据内存（DMEM）参数 **/
		struct ipu3_uapi_isp_tnr3_params tnr3_dmem_params;
		struct ipu3_uapi_isp_xnr3_params xnr3_dmem_params;

		/** 光学黑电平补偿 **/
		struct ipu3_uapi_obgrid_param obgrid_param;
	};

## Intel IPU3 ImgU uAPI 数据类型
