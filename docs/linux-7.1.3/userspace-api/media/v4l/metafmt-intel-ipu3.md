######## V4L2_META_FMT_IPU3_PARAMS ('ip3p'), V4L2_META_FMT_IPU3_3A ('ip3s')


## 3A 缁熻


IPU3 ImgU 3A 缁熻鍔犻€熷櫒鍦ㄨ緭鍏ョ殑 Bayer 甯т笂鏀堕泦涓嶅悓鐨勭粺璁′俊鎭€傝繖浜涚粺璁′俊鎭€氳繃 鈥渋pu3-imgu [^01^] 3a
stat鈥?鍏冩暟鎹崟鑾疯棰戣妭鐐癸紝浣跨敤 `v4l2_meta_format` 鎺ュ彛鑾峰彇銆傚畠浠殑鏍煎紡濡?`ipu3_uapi_stats_3a`
缁撴瀯鎵€鎻忚堪銆?
鏀堕泦鐨勭粺璁′俊鎭寘鎷?AWB锛堣嚜鍔ㄧ櫧骞宠　锛塕GBS锛堢孩銆佺豢銆佽摑鍜岄ケ鍜屽害娴嬮噺锛夊崟鍏冦€丄WB 婊ゆ尝鍣ㄥ搷搴斻€?AF锛堣嚜鍔ㄥ鐒︼級婊ゆ尝鍣ㄥ搷搴旓紝浠ュ強 AE锛堣嚜鍔ㄦ洕鍏夛級鐩存柟鍥俱€?
struct `ipu3_uapi_4a_config` 淇濆瓨鎵€鏈夊彲閰嶇疆鍙傛暟銆?

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


## 娴佹按绾垮弬鏁?

娴佹按绾垮弬鏁伴€氳繃 鈥渋pu3-imgu [^01^] parameters鈥?鍏冩暟鎹緭鍑鸿棰戣妭鐐癸紝浣跨敤 `v4l2_meta_format`
鎺ュ彛浼犻€掋€傚畠浠殑鏍煎紡濡?`ipu3_uapi_params` 缁撴瀯鎵€鎻忚堪銆?
姝ゅ鎻忚堪鐨?3A 缁熻鍜屾祦姘寸嚎鍙傛暟閮戒笌搴曞眰鐨勭浉鏈哄瓙绯荤粺锛圕SS锛堿PI 绱у瘑鐩稿叧銆傚畠浠€氬父鐢卞寘鍚噸瑕?璋冧紭宸ュ叿鐨勪笓鐢ㄧ敤鎴风┖闂村簱娑堣垂鍜屼骇鍑猴紝浠庤€岃寮€鍙戜汉鍛樹笉蹇呰搴曞眰鐨勭‖浠跺拰绠楁硶缁嗚妭鎵€鍥版壈銆?

	struct ipu3_uapi_params {
		/** 鏍囧織涓嬫柟鍝簺璁剧疆灏嗚搴旂敤 **/
		struct ipu3_uapi_flags use;

		/** 鍔犻€熷櫒闆嗙兢鍙傛暟 **/
		struct ipu3_uapi_acc_param acc_param;

		/** ISP 鍚戦噺鍦板潃绌洪棿鍙傛暟 **/
		struct ipu3_uapi_isp_lin_vmem_params lin_vmem_params;
		struct ipu3_uapi_isp_tnr3_vmem_params tnr3_vmem_params;
		struct ipu3_uapi_isp_xnr3_vmem_params xnr3_vmem_params;

		/** ISP 鏁版嵁鍐呭瓨锛圖MEM锛夊弬鏁?**/
		struct ipu3_uapi_isp_tnr3_params tnr3_dmem_params;
		struct ipu3_uapi_isp_xnr3_params xnr3_dmem_params;

		/** 鍏夊榛戠數骞宠ˉ鍋?**/
		struct ipu3_uapi_obgrid_param obgrid_param;
	};

## Intel IPU3 ImgU uAPI 鏁版嵁绫诲瀷
