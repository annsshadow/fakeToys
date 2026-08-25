


######## V4L2_META_FMT_C3ISP_STATS ('C3ST'), V4L2_META_FMT_C3ISP_PARAMS ('C3PM')


## 3A 统计信息


C3 ISP 可以采集输入 Bayer 帧上的不同统计信息这些统计信息通过 "c3-isp-stats" 元数据捕获视频节点，使用 `v4l2_meta_format` 接口获取它们按照 `c3_isp_stats_info` 结构体的描述进行格式化
采集的统计信息为自动白平衡（Auto-white balance）、自动曝光（Auto-exposure）与自动对焦（Auto-focus）信息

## 配置参数


配置参数通过 c3-isp-params 元数据输出视频节点，使用 `v4l2_meta_format` 接口传递与包含一个用ISP 各可配置区域的子结构体的单一结构体不同，C3-ISP 的参数被定义为不同的结构体或“块（blocks）”，可以添加`c3_isp_params_cfg` data 成员中。用户空间负责用需要由驱动配置的块来填data 成员，但无需*所*块来填充它，如果没有任何配置更改需要做出，甚至根本不需要填充任何块。已填充的块**必须**在缓冲区中连续。为了帮助用户空间与驱动识别块，每个块特定的结构体将其第一个成员嵌`c3_isp_params_block_header`，并且用户空间必须用来自 `c3_isp_params_block_type` 的值填type 成员。一旦这些块被填充进数据缓冲区，所有已填充块的总大小应设置`c3_isp_params_cfg` data_size 成员中。例如：


	struct c3_isp_params_cfg *params =
		(struct c3_isp_params_cfg *)buffer;

	params->version = C3_ISP_PARAM_BUFFER_V0;
	params->data_size = 0;

	void **data = (void **)params->data;

	struct c3_isp_params_awb_gains *gains =
		(struct c3_isp_params_awb_gains *)data;

	gains->header.type = C3_ISP_PARAMS_BLOCK_AWB_GAINS;
	gains->header.flags = C3_ISP_PARAMS_BLOCK_FL_ENABLE;
	gains->header.size = sizeof(struct c3_isp_params_awb_gains);

	gains->gr_gain = 256;
	gains->r_gain = 256;
	gains->b_gain = 256;
	gains->gb_gain = 256;

	data += sizeof(struct c3_isp__params_awb_gains);
	params->data_size += sizeof(struct c3_isp_params_awb_gains);

	struct c3_isp_params_awb_config *awb_cfg =
		(struct c3_isp_params_awb_config *)data;

	awb_cfg->header.type = C3_ISP_PARAMS_BLOCK_AWB_CONFIG;
	awb_cfg->header.flags = C3_ISP_PARAMS_BLOCK_FL_ENABLE;
	awb_cfg->header.size = sizeof(struct c3_isp_params_awb_config);

	awb_cfg->tap_point = C3_ISP_AWB_STATS_TAP_BEFORE_WB;
	awb_cfg->satur = 1;
	awb_cfg->horiz_zones_num = 32;
	awb_cfg->vert_zones_num = 24;

	params->data_size += sizeof(struct c3_isp_params_awb_config);

## Amlogic C3 ISP uAPI 数据类型
