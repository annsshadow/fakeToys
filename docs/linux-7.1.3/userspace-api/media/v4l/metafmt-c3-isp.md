


######## V4L2_META_FMT_C3ISP_STATS ('C3ST'), V4L2_META_FMT_C3ISP_PARAMS ('C3PM')


## 3A 缁熻淇℃伅


C3 ISP 鍙互閲囬泦杈撳叆 Bayer 甯т笂鐨勪笉鍚岀粺璁′俊鎭€?杩欎簺缁熻淇℃伅閫氳繃 "c3-isp-stats" 鍏冩暟鎹崟鑾疯棰戣妭鐐癸紝浣跨敤 `v4l2_meta_format` 鎺ュ彛鑾峰彇銆?瀹冧滑鎸夌収 `c3_isp_stats_info` 缁撴瀯浣撶殑鎻忚堪杩涜鏍煎紡鍖栥€?
閲囬泦鐨勭粺璁′俊鎭负鑷姩鐧藉钩琛★紙Auto-white balance锛夈€佽嚜鍔ㄦ洕鍏夛紙Auto-exposure锛変笌鑷姩瀵圭劍锛圓uto-focus锛変俊鎭€?

## 閰嶇疆鍙傛暟


閰嶇疆鍙傛暟閫氳繃 c3-isp-params 鍏冩暟鎹緭鍑鸿棰戣妭鐐癸紝浣跨敤 `v4l2_meta_format` 鎺ュ彛浼犻€掋€?涓庡寘鍚竴涓敤浜?ISP 鍚勫彲閰嶇疆鍖哄煙鐨勫瓙缁撴瀯浣撶殑鍗曚竴缁撴瀯浣撲笉鍚岋紝C3-ISP 鐨勫弬鏁拌瀹氫箟涓轰笉鍚岀殑缁撴瀯浣撴垨鈥滃潡锛坆locks锛夆€濓紝鍙互娣诲姞鍒?`c3_isp_params_cfg` 鐨?data 鎴愬憳涓€傜敤鎴风┖闂磋礋璐ｇ敤闇€瑕佺敱椹卞姩閰嶇疆鐨勫潡鏉ュ～鍏?data 鎴愬憳锛屼絾鏃犻渶鐢?*鎵€鏈?*鍧楁潵濉厖瀹冿紝濡傛灉娌℃湁浠讳綍閰嶇疆鏇存敼闇€瑕佸仛鍑猴紝鐢氳嚦鏍规湰涓嶉渶瑕佸～鍏呬换浣曞潡銆傚凡濉厖鐨勫潡**蹇呴』**鍦ㄧ紦鍐插尯涓繛缁€備负浜嗗府鍔╃敤鎴风┖闂翠笌椹卞姩璇嗗埆鍧楋紝姣忎釜鍧楃壒瀹氱殑缁撴瀯浣撳皢鍏剁涓€涓垚鍛樺祵鍏?`c3_isp_params_block_header`锛屽苟涓旂敤鎴风┖闂村繀椤荤敤鏉ヨ嚜 `c3_isp_params_block_type` 鐨勫€煎～鍏?type 鎴愬憳銆備竴鏃﹁繖浜涘潡琚～鍏呰繘鏁版嵁缂撳啿鍖猴紝鎵€鏈夊凡濉厖鍧楃殑鎬诲ぇ灏忓簲璁剧疆鍦?`c3_isp_params_cfg` 鐨?data_size 鎴愬憳涓€備緥濡傦細


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

## Amlogic C3 ISP uAPI 鏁版嵁绫诲瀷
