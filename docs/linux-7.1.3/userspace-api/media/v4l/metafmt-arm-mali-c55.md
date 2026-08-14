


######## V4L2_META_FMT_MALI_C55_STATS ('C55S'), V4L2_META_FMT_MALI_C55_PARAMS ('C55P')


## 3A 缁熻淇℃伅


ISP 璁惧浼氶拡瀵硅緭鍏ョ殑 bayer 甯ф敹闆嗕笉鍚岀殑缁熻淇℃伅銆傜敤鎴风┖闂村彲浠ラ€氳繃
mali-c55 3a stats <mali-c55-3a-stats> 鍏冩暟鎹崟鑾疯棰戣妭鐐癸紝浣跨敤
`v4l2_meta_format` 鎺ュ彛鏉ヨ幏鍙栬繖浜涚粺璁′俊鎭€傜紦鍐插尯鍖呭惈 `mali-c55-config.h` 涓?瀹氫箟鐨?C 缁撴瀯浣?`mali_c55_stats_buffer` 鐨勫崟涓疄渚嬶紝鍥犳鍙互閫氳繃濡備笅鏂瑰紡浠?缂撳啿鍖轰腑鑾峰彇璇ョ粨鏋勪綋锛?

	struct mali_c55_stats_buffer *stats =
		(struct mali_c55_stats_buffer *)buf;

鏈夊叧缁熻淇℃伅鐨勭粏鑺傝鍙傝 `mali_c55_stats_buffer`銆?
## 閰嶇疆鍙傛暟


閰嶇疆鍙傛暟閫氳繃 :ref:`mali-c55 3a params <mali-c55-3a-params>` 鍏冩暟鎹緭鍑鸿棰戣妭鐐?浼犻€掞紝浣跨敤 `v4l2_meta_format` 鎺ュ彛銆備笌涓烘瘡涓彲閰嶇疆 ISP 鍖哄煙鍖呭惈瀛愮粨鏋勪綋鐨勫崟涓€
缁撴瀯浣撲笉鍚岋紝Mali-C55 鐨勫弬鏁颁娇鐢?v4l2-isp parameters 绯荤粺锛岄€氳繃璇ョ郴缁燂紝鍙傛暟缁勮
瀹氫箟涓轰笉鍚岀殑缁撴瀯浣撴垨鈥滃潡鈥濓紙blocks锛夛紝鍙互琚坊鍔犲埌 `v4l2_isp_params_buffer` 鐨?data 鎴愬憳涓€傜敤鎴风┖闂磋礋璐ｇ敤闇€瑕佺敱椹卞姩閰嶇疆鐨勫潡鏉ュ～鍏?data 鎴愬憳銆傛瘡涓潡鐗规湁鐨?缁撴瀯浣撳皢鍏剁涓€涓垚鍛樺祵鍏?`v4l2_isp_params_block_header`锛屽苟涓旂敤鎴风┖闂村繀椤荤敤
`mali_c55_param_block_type` 涓殑涓€涓€兼潵濉厖 type 鎴愬憳銆?

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

## Arm Mali-C55 uAPI 鏁版嵁绫诲瀷
