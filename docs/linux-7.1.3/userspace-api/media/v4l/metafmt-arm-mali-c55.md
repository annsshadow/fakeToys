


######## V4L2_META_FMT_MALI_C55_STATS ('C55S'), V4L2_META_FMT_MALI_C55_PARAMS ('C55P')


## 3A 统计信息


ISP 设备会针对输入的 bayer 帧收集不同的统计信息。用户空间可以通过
mali-c55 3a stats <mali-c55-3a-stats> 元数据捕获视频节点，使用
`v4l2_meta_format` 接口来获取这些统计信息。缓冲区包含 `mali-c55-config.h` 中
定义的 C 结构体 `mali_c55_stats_buffer` 的单个实例，因此可以通过如下方式从
缓冲区中获取该结构体：


	struct mali_c55_stats_buffer *stats =
		(struct mali_c55_stats_buffer *)buf;

有关统计信息的细节请参见 `mali_c55_stats_buffer`。

## 配置参数


配置参数通过 :ref:`mali-c55 3a params <mali-c55-3a-params>` 元数据输出视频节点
传递，使用 `v4l2_meta_format` 接口。与为每个可配置 ISP 区域包含子结构体的单一
结构体不同，Mali-C55 的参数使用 v4l2-isp parameters 系统，通过该系统，参数组被
定义为不同的结构体或“块”（blocks），可以被添加到 `v4l2_isp_params_buffer` 的
data 成员中。用户空间负责用需要由驱动配置的块来填充 data 成员。每个块特有的
结构体将其第一个成员嵌入 `v4l2_isp_params_block_header`，并且用户空间必须用
`mali_c55_param_block_type` 中的一个值来填充 type 成员。


	struct v4l2_isp_params_buffer *params =
		(struct v4l2_isp_params_buffer *)buffer;

	params->version = V4L2_ISP_PARAMS_VERSION_V1;
	params->data_size = 0;

	void **data = (void **)params->data;

	struct mali_c55_params_awb_gains *gains =
		(struct mali_c55_params_awb_gains *)data;

	gains->header.type = MALI_C55_PARAM_BLOCK_AWB_GAINS;
	gains->header.flags |= V4L2_ISP_PARAMS_FL_BLOCK_ENABLE;
	gains->header.size = sizeof(struct mali_c55_params_awb_gains);

	gains->gain00 = 256;
	gains->gain00 = 256;
	gains->gain00 = 256;
	gains->gain00 = 256;

	data += sizeof(struct mali_c55_params_awb_gains);
	params->data_size += sizeof(struct mali_c55_params_awb_gains);

	struct mali_c55_params_sensor_off_preshading *blc =
		(struct mali_c55_params_sensor_off_preshading *)data;

	blc->header.type = MALI_C55_PARAM_BLOCK_SENSOR_OFFS;
	blc->header.flags |= V4L2_ISP_PARAMS_FL_BLOCK_ENABLE;
	blc->header.size = sizeof(struct mali_c55_params_sensor_off_preshading);

	blc->chan00 = 51200;
	blc->chan01 = 51200;
	blc->chan10 = 51200;
	blc->chan11 = 51200;

	params->data_size += sizeof(struct mali_c55_params_sensor_off_preshading);

## Arm Mali-C55 uAPI 数据类型
