
######## Stateless Codec Control Reference


鏃犵姸鎬佺紪瑙ｇ爜鍣紙Stateless Codec锛夋帶鍒剁被鏃ㄥ湪鏀寔鏃犵姸鎬佺殑瑙ｇ爜鍣ㄤ笌缂栫爜鍣紙鍗崇‖浠跺姞閫熷櫒锛夈€?

杩欎簺椹卞姩閫氬父鐢?stateless_decoder 鏀寔锛屽苟澶勭悊宸茶В鏋愮殑鍍忕礌鏍煎紡锛屼緥濡?V4L2_PIX_FMT_H264_SLICE銆?

## 鏃犵姸鎬佺紪瑙ｇ爜鍣ㄦ帶鍒?ID



`V4L2_CID_CODEC_STATELESS_CLASS (class)`
    鏃犵姸鎬佺紪瑙ｇ爜鍣ㄧ被鎻忚堪绗︺€?


`V4L2_CID_STATELESS_H264_SPS (struct)`
    鎸囧畾涓庣浉搴?H264 鍒囩墖鏁版嵁鍏宠仈鐨勫簭鍒楀弬鏁伴泦锛堜粠鐮佹祦涓彁鍙栵級銆傚叾涓寘鍚厤缃?H264
    鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?h264 鏍囧噯绗?7.4.2.1.1 鑺?
    鈥淪equence Parameter Set Data Semantics鈥濓紙搴忓垪鍙傛暟闆嗘暟鎹涔夛級瀹氫箟銆傞櫎闈炴湁鏄庣‘娉ㄩ噴
    鍙︽湁璇存槑锛屽惁鍒欒繘涓€姝ョ殑鏂囨。璇峰弬鑰冧笂杩拌鑼冦€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `profile_idc`
      -
    - - __u8
      - `constraint_set_flags`
      - 鍙傝搴忓垪鍙傛暟闆嗙害鏉熸爣蹇?<h264_sps_constraints_set_flags>
    - - __u8
      - `level_idc`
      -
    - - __u8
      - `seq_parameter_set_id`
      -
    - - __u8
      - `chroma_format_idc`
      -
    - - __u8
      - `bit_depth_luma_minus8`
      -
    - - __u8
      - `bit_depth_chroma_minus8`
      -
    - - __u8
      - `log2_max_frame_num_minus4`
      -
    - - __u8
      - `pic_order_cnt_type`
      -
    - - __u8
      - `log2_max_pic_order_cnt_lsb_minus4`
      -
    - - __u8
      - `max_num_ref_frames`
      -
    - - __u8
      - `num_ref_frames_in_pic_order_cnt_cycle`
      -
    - - __s32
      - `offset_for_ref_frame[^255^]`
      -
    - - __s32
      - `offset_for_non_ref_pic`
      -
    - - __s32
      - `offset_for_top_to_bottom_field`
      -
    - - __u16
      - `pic_width_in_mbs_minus1`
      -
    - - __u16
      - `pic_height_in_map_units_minus1`
      -
    - - __u32
      - `flags`
      - 鍙傝搴忓垪鍙傛暟闆嗘爣蹇?<h264_sps_flags>


    \normalsize


`搴忓垪鍙傛暟闆嗙害鏉熸爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_SPS_CONSTRAINT_SET0_FLAG`
      - 0x00000001
      -
    - - `V4L2_H264_SPS_CONSTRAINT_SET1_FLAG`
      - 0x00000002
      -
    - - `V4L2_H264_SPS_CONSTRAINT_SET2_FLAG`
      - 0x00000004
      -
    - - `V4L2_H264_SPS_CONSTRAINT_SET3_FLAG`
      - 0x00000008
      -
    - - `V4L2_H264_SPS_CONSTRAINT_SET4_FLAG`
      - 0x00000010
      -
    - - `V4L2_H264_SPS_CONSTRAINT_SET5_FLAG`
      - 0x00000020
      -


`搴忓垪鍙傛暟闆嗘爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_SPS_FLAG_SEPARATE_COLOUR_PLANE`
      - 0x00000001
      -
    - - `V4L2_H264_SPS_FLAG_QPPRIME_Y_ZERO_TRANSFORM_BYPASS`
      - 0x00000002
      -
    - - `V4L2_H264_SPS_FLAG_DELTA_PIC_ORDER_ALWAYS_ZERO`
      - 0x00000004
      -
    - - `V4L2_H264_SPS_FLAG_GAPS_IN_FRAME_NUM_VALUE_ALLOWED`
      - 0x00000008
      -
    - - `V4L2_H264_SPS_FLAG_FRAME_MBS_ONLY`
      - 0x00000010
      -
    - - `V4L2_H264_SPS_FLAG_MB_ADAPTIVE_FRAME_FIELD`
      - 0x00000020
      -
    - - `V4L2_H264_SPS_FLAG_DIRECT_8X8_INFERENCE`
      - 0x00000040
      -

`V4L2_CID_STATELESS_H264_PPS (struct)`
    鎸囧畾涓庣浉搴?H264 鍒囩墖鏁版嵁鍏宠仈鐨勫浘鍍忓弬鏁伴泦锛堜粠鐮佹祦涓彁鍙栵級銆傚叾涓寘鍚厤缃?H264
    鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?h264 鏍囧噯绗?7.4.2.2 鑺?
    鈥淧icture Parameter Set RBSP Semantics鈥濓紙鍥惧儚鍙傛暟闆?RBSP 璇箟锛夊畾涔夈€傞櫎闈炴湁鏄庣‘娉ㄩ噴
    鍙︽湁璇存槑锛屽惁鍒欒繘涓€姝ョ殑鏂囨。璇峰弬鑰冧笂杩拌鑼冦€?



    \small

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `pic_parameter_set_id`
      -
    - - __u8
      - `seq_parameter_set_id`
      -
    - - __u8
      - `num_slice_groups_minus1`
      -
    - - __u8
      - `num_ref_idx_l0_default_active_minus1`
      -
    - - __u8
      - `num_ref_idx_l1_default_active_minus1`
      -
    - - __u8
      - `weighted_bipred_idc`
      -
    - - __s8
      - `pic_init_qp_minus26`
      -
    - - __s8
      - `pic_init_qs_minus26`
      -
    - - __s8
      - `chroma_qp_index_offset`
      -
    - - __s8
      - `second_chroma_qp_index_offset`
      -
    - - __u16
      - `flags`
      - 鍙傝鍥惧儚鍙傛暟闆嗘爣蹇?<h264_pps_flags>


    \normalsize


`鍥惧儚鍙傛暟闆嗘爣蹇梎


    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}


    :header-rows:  0
    :stub-columns: 0
    :widths:       10 1 4

    - - `V4L2_H264_PPS_FLAG_ENTROPY_CODING_MODE`
      - 0x0001
      -
    - - `V4L2_H264_PPS_FLAG_BOTTOM_FIELD_PIC_ORDER_IN_FRAME_PRESENT`
      - 0x0002
      -
    - - `V4L2_H264_PPS_FLAG_WEIGHTED_PRED`
      - 0x0004
      -
    - - `V4L2_H264_PPS_FLAG_DEBLOCKING_FILTER_CONTROL_PRESENT`
      - 0x0008
      -
    - - `V4L2_H264_PPS_FLAG_CONSTRAINED_INTRA_PRED`
      - 0x0010
      -
    - - `V4L2_H264_PPS_FLAG_REDUNDANT_PIC_CNT_PRESENT`
      - 0x0020
      -
    - - `V4L2_H264_PPS_FLAG_TRANSFORM_8X8_MODE`
      - 0x0040
      -
    - - `V4L2_H264_PPS_FLAG_SCALING_MATRIX_PRESENT`
      - 0x0080
      - 蹇呴』瀵硅鍥惧儚浣跨敤 `V4L2_CID_STATELESS_H264_SCALING_MATRIX`銆?


    \endgroup

`V4L2_CID_STATELESS_H264_SCALING_MATRIX (struct)`
    鎸囧畾涓庣浉搴?H264 鍒囩墖鏁版嵁鍏宠仈鐨勭缉鏀剧煩闃碉紙浠庣爜娴佷腑鎻愬彇锛夈€傜爜娴佸弬鏁颁緷鎹?h264 鏍囧噯
    绗?7.4.2.1.1.1 鑺?鈥淪caling List Semantics鈥濓紙缂╂斁鍒楄〃璇箟锛夊畾涔夈€傞櫎闈炴湁鏄庣‘娉ㄩ噴
    鍙︽湁璇存槑锛屽惁鍒欒繘涓€姝ョ殑鏂囨。璇峰弬鑰冧笂杩拌鑼冦€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `scaling_list_4x4[^6^][^16^]`
      - 搴旂敤閫嗘壂鎻忚繃绋嬪悗鐨勭缉鏀剧煩闃点€傛湡鏈涚殑鍒楄〃椤哄簭涓猴細甯у唴 Y銆佸抚鍐?Cb銆佸抚鍐?Cr銆?
        甯ч棿 Y銆佸抚闂?Cb銆佸抚闂?Cr銆傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_8x8[^6^][^64^]`
      - 搴旂敤閫嗘壂鎻忚繃绋嬪悗鐨勭缉鏀剧煩闃点€傛湡鏈涚殑鍒楄〃椤哄簭涓猴細甯у唴 Y銆佸抚闂?Y銆佸抚鍐?Cb銆?
        甯ч棿 Cb銆佸抚鍐?Cr銆佸抚闂?Cr銆傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?

`V4L2_CID_STATELESS_H264_SLICE_PARAMS (struct)`
    鎸囧畾涓庣浉搴?H264 鍒囩墖鏁版嵁鍏宠仈鐨勫垏鐗囧弬鏁帮紙浠庣爜娴佷腑鎻愬彇锛夈€傚叾涓寘鍚厤缃?H264
    鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?h264 鏍囧噯绗?7.4.3 鑺?
    鈥淪lice Header Semantics鈥濓紙鍒囩墖澶磋涔夛級瀹氫箟銆傞櫎闈炴湁鏄庣‘娉ㄩ噴鍙︽湁璇存槑锛屽惁鍒欒繘涓€姝ョ殑
    鏂囨。璇峰弬鑰冧笂杩拌鑼冦€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `header_bit_size`
      - 浠庢湰鍒囩墖璧峰澶勫埌 slice_data() 鐨勫亸绉婚噺锛堜互姣旂壒璁★級銆?
    - - __u32
      - `first_mb_in_slice`
      -
    - - __u8
      - `slice_type`
      -
    - - __u8
      - `colour_plane_id`
      -
    - - __u8
      - `redundant_pic_cnt`
      -
    - - __u8
      - `cabac_init_idc`
      -
    - - __s8
      - `slice_qp_delta`
      -
    - - __s8
      - `slice_qs_delta`
      -
    - - __u8
      - `disable_deblocking_filter_idc`
      -
    - - __s8
      - `slice_alpha_c0_offset_div2`
      -
    - - __s8
      - `slice_beta_offset_div2`
      -
    - - __u8
      - `num_ref_idx_l0_active_minus1`
      - 鑻ユ湭璁剧疆 num_ref_idx_active_override_flag锛屾湰瀛楁蹇呴』璁句负
        num_ref_idx_l0_default_active_minus1 鐨勫€笺€?
    - - __u8
      - `num_ref_idx_l1_active_minus1`
      - 鑻ユ湭璁剧疆 num_ref_idx_active_override_flag锛屾湰瀛楁蹇呴』璁句负
        num_ref_idx_l1_default_active_minus1 鐨勫€笺€?
    - - __u8
      - `reserved`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - struct `v4l2_h264_reference`
      - `ref_pic_list0[^32^]`
      - 搴旂敤閫愬垏鐗囦慨鏀逛箣鍚庣殑鍙傝€冨浘鍍忓垪琛ㄣ€?
    - - struct `v4l2_h264_reference`
      - `ref_pic_list1[^32^]`
      - 搴旂敤閫愬垏鐗囦慨鏀逛箣鍚庣殑鍙傝€冨浘鍍忓垪琛ㄣ€?
    - - __u32
      - `flags`
      - 鍙傝鍒囩墖鍙傛暟鏍囧織 <h264_slice_flags>


    \normalsize


`鍒囩墖鍙傛暟闆嗘爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_SLICE_FLAG_DIRECT_SPATIAL_MV_PRED`
      - 0x00000001
      -
    - - `V4L2_H264_SLICE_FLAG_SP_FOR_SWITCH`
      - 0x00000002
      -

`V4L2_CID_STATELESS_H264_PRED_WEIGHTS (struct)`
    渚濇嵁 h264 鏍囧噯绗?7.4.3.2 鑺?鈥淧rediction Weight Table Semantics鈥濓紙棰勬祴鍔犳潈琛ㄨ涔夛級
    瀹氫箟鐨勯娴嬪姞鏉冭〃銆傞娴嬪姞鏉冭〃蹇呴』鍦ㄧ 7.3.3 鑺?鈥淪lice header syntax鈥濓紙鍒囩墖澶磋娉曪級
    鎵€杩版潯浠朵笅鐢卞簲鐢ㄧ▼搴忎紶鍏ャ€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `luma_log2_weight_denom`
      -
    - - __u16
      - `chroma_log2_weight_denom`
      -
    - - struct `v4l2_h264_weight_factors`
      - `weight_factors[^2^]`
      - 绱㈠紩 0 澶勭殑鍔犳潈鍥犲瓙瀵瑰簲鍙傝€冨垪琛?0锛岀储寮?1 澶勭殑鍔犳潈鍥犲瓙瀵瑰簲鍙傝€冨垪琛?1銆?


    \normalsize



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s16
      - `luma_weight[^32^]`
      -
    - - __s16
      - `luma_offset[^32^]`
      -
    - - __s16
      - `chroma_weight[^32^][^2^]`
      -
    - - __s16
      - `chroma_offset[^32^][^2^]`
      -


    \normalsize

`鍥惧儚鍙傝€僠



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `fields`
      - 鎸囧畾璇ュ浘鍍忓浣曡寮曠敤銆傚弬瑙佸弬鑰冨瓧娈?<h264_ref_fields>
    - - __u8
      - `index`
      - 鎸囧悜 `v4l2_ctrl_h264_decode_params`.dpb 鏁扮粍鐨勭储寮曘€?


`鍙傝€冨瓧娈礰


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_TOP_FIELD_REF`
      - 0x1
      - 瀛楁瀵逛腑椤跺満鐢ㄤ簬鐭湡鍙傝€冦€?
    - - `V4L2_H264_BOTTOM_FIELD_REF`
      - 0x2
      - 瀛楁瀵逛腑搴曞満鐢ㄤ簬鐭湡鍙傝€冦€?
    - - `V4L2_H264_FRAME_REF`
      - 0x3
      - 甯э紙鎴栭《/搴曞満锛岃嫢鍏朵负瀛楁瀵癸級鐢ㄤ簬鐭湡鍙傝€冦€?


    \normalsize

`V4L2_CID_STATELESS_H264_DECODE_PARAMS (struct)`
    鎸囧畾涓庣浉搴?H264 鍒囩墖鏁版嵁鍏宠仈鐨勮В鐮佸弬鏁帮紙浠庣爜娴佷腑鎻愬彇锛夈€傚叾涓寘鍚厤缃?H264
    鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?h264 鏍囧噯瀹氫箟銆傞櫎闈炴湁鏄庣‘娉ㄩ噴
    鍙︽湁璇存槑锛屽惁鍒欒繘涓€姝ョ殑鏂囨。璇峰弬鑰冧笂杩拌鑼冦€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_h264_dpb_entry`
      - `dpb[^16^]`
      -
    - - __u16
      - `nal_ref_idc`
      - 鏉ヨ嚜 NAL 鍗曞厓澶寸殑 NAL 鍙傝€?ID 鍊笺€?
    - - __u16
      - `frame_num`
      -
    - - __s32
      - `top_field_order_cnt`
      - 缂栫爜椤跺満鐨勫浘鍍忛『搴忚鏁帮紙Picture Order Count锛夈€?
    - - __s32
      - `bottom_field_order_cnt`
      - 缂栫爜搴曞満鐨勫浘鍍忛『搴忚鏁般€?
    - - __u16
      - `idr_pic_id`
      -
    - - __u16
      - `pic_order_cnt_lsb`
      -
    - - __s32
      - `delta_pic_order_cnt_bottom`
      -
    - - __s32
      - `delta_pic_order_cnt0`
      -
    - - __s32
      - `delta_pic_order_cnt1`
      -
    - - __u32
      - `dec_ref_pic_marking_bit_size`
      - dec_ref_pic_marking() 璇硶鍏冪礌鐨勫ぇ灏忥紙浠ユ瘮鐗硅锛夈€?
    - - __u32
      - `pic_order_cnt_bit_size`
      - 涓庡浘鍍忛『搴忚鏁扮浉鍏崇殑璇硶鍏冪礌鐨勫悎骞跺ぇ灏忥紙浠ユ瘮鐗硅锛夛細pic_order_cnt_lsb銆?
        delta_pic_order_cnt_bottom銆乨elta_pic_order_cnt0 浠ュ強 delta_pic_order_cnt1銆?
    - - __u32
      - `slice_group_change_cycle`
      -
    - - __u32
      - `reserved`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u32
      - `flags`
      - 鍙傝瑙ｇ爜鍙傛暟鏍囧織 <h264_decode_params_flags>


    \normalsize


`瑙ｇ爜鍙傛暟鏍囧織`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_DECODE_PARAM_FLAG_IDR_PIC`
      - 0x00000001
      - 璇ュ浘鍍忎负 IDR 鍥惧儚銆?
    - - `V4L2_H264_DECODE_PARAM_FLAG_FIELD_PIC`
      - 0x00000002
      -
    - - `V4L2_H264_DECODE_PARAM_FLAG_BOTTOM_FIELD`
      - 0x00000004
      -
    - - `V4L2_H264_DECODE_PARAM_FLAG_PFRAME`
      - 0x00000008
      -
    - - `V4L2_H264_DECODE_PARAM_FLAG_BFRAME`
      - 0x00000010
      -



    \normalsize



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `reference_ts`
      - 鐢ㄤ綔鍙傝€冪殑 V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓?B 甯у拰 P 甯ч厤鍚堜娇鐢ㄣ€傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u32
      - `pic_num`
      - 瀵逛簬鐭湡鍙傝€冿紝鏈瓧娈靛繀椤讳笌鎺ㄥ鍊?PicNum (8-28) 涓€鑷达紱瀵逛簬闀挎湡鍙傝€冿紝蹇呴』涓庢帹瀵煎€?
        LongTermPicNum (8-29) 涓€鑷淬€傝В鐮佸抚锛堣€岄潪鍦猴級鏃讹紝pic_num 涓?FrameNumWrap 鐩稿悓銆?
    - - __u16
      - `frame_num`
      - 瀵逛簬鐭湡鍙傝€冿紝鏈瓧娈靛繀椤讳笌鍒囩墖澶磋娉曚腑鐨?frame_num 鍊间竴鑷达紙椹卞姩浼氬湪闇€瑕佹椂瀵硅鍊?
        杩涜鐜粫澶勭悊锛夈€傚浜庨暱鏈熷弬鑰冿紝鏈瓧娈靛繀椤昏涓?dec_ref_pic_marking() 璇硶涓弿杩扮殑
        long_term_frame_idx 鐨勫€笺€?
    - - __u8
      - `fields`
      - 鎸囧畾璇?DPB 鏉＄洰濡備綍琚紩鐢ㄣ€傚弬瑙佸弬鑰冨瓧娈?<h264_ref_fields>
    - - __u8
      - `reserved[^5^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __s32
      - `top_field_order_cnt`
      -
    - - __s32
      - `bottom_field_order_cnt`
      -
    - - __u32
      - `flags`
      - 鍙傝 DPB 鏉＄洰鏍囧織 <h264_dpb_flags>


    \normalsize


`DPB 鏉＄洰鏍囧織`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_DPB_ENTRY_FLAG_VALID`
      - 0x00000001
      - 璇?DPB 鏉＄洰鏈夋晥锛堥潪绌猴級锛屽簲褰撲簣浠ヨ€冭檻銆?
    - - `V4L2_H264_DPB_ENTRY_FLAG_ACTIVE`
      - 0x00000002
      - 璇?DPB 鏉＄洰鐢ㄤ綔鍙傝€冦€?
    - - `V4L2_H264_DPB_ENTRY_FLAG_LONG_TERM`
      - 0x00000004
      - 璇?DPB 鏉＄洰鐢ㄤ綔闀挎湡鍙傝€冦€?
    - - `V4L2_H264_DPB_ENTRY_FLAG_FIELD`
      - 0x00000008
      - 璇?DPB 鏉＄洰涓哄崟涓満鎴栦簰琛ュ満瀵广€?


    \normalsize

`V4L2_CID_STATELESS_H264_DECODE_MODE (enum)`
    鎸囧畾瑕佷娇鐢ㄧ殑瑙ｇ爜妯″紡銆傜洰鍓嶆彁渚涘熀浜庡垏鐗囧拰鍩轰簬甯х殑瑙ｇ爜锛屼絾鍚庣画鍙兘浼氭柊澧炲叾浠栨ā寮忋€?
    璇ユ帶浠剁敤浣?V4L2_PIX_FMT_H264_SLICE 鍍忕礌鏍煎紡鐨勪慨楗扮銆傛敮鎸?V4L2_PIX_FMT_H264_SLICE
    鐨勫簲鐢ㄧ▼搴忓繀椤昏缃鎺т欢锛屼互鎸囧畾缂撳啿鍖烘墍鏈熸湜鐨勮В鐮佹ā寮忋€?
    椹卞姩鍙兘鏍规嵁鍏舵墍鏀寔鐨勮兘鍔涳紝鏆撮湶鍗曚釜鎴栧涓В鐮佹ā寮忋€?



    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_H264_DECODE_MODE_SLICE_BASED`
      - 0
      - 浠ュ垏鐗囩矑搴﹁繘琛岃В鐮併€侽UTPUT 缂撳啿鍖哄繀椤诲寘鍚崟涓垏鐗囥€傞€夋嫨璇ユā寮忔椂锛?
        蹇呴』璁剧疆 `V4L2_CID_STATELESS_H264_SLICE_PARAMS` 鎺т欢銆傚綋澶氫釜鍒囩墖鏋勬垚涓€涓抚鏃讹紝
        闇€瑕佷娇鐢?`V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF` 鏍囧織銆?
    - - `V4L2_STATELESS_H264_DECODE_MODE_FRAME_BASED`
      - 1
      - 浠ュ抚绮掑害杩涜瑙ｇ爜銆侽UTPUT 缂撳啿鍖哄繀椤诲寘鍚В鐮佽甯ф墍闇€鐨勫叏閮ㄥ垏鐗囷紝
        骞朵笖蹇呴』鍚屾椂鍖呭惈涓や釜鍦恒€傝妯″紡鐢卞湪纭欢涓В鏋愬垏鐗囧ご鐨勮澶囨敮鎸併€傞€夋嫨璇ユā寮忔椂锛?
        涓嶅簲璁剧疆 `V4L2_CID_STATELESS_H264_SLICE_PARAMS` 鎺т欢銆?


    \normalsize

`V4L2_CID_STATELESS_H264_START_CODE (enum)`
    鎸囧畾姣忎釜鍒囩墖鎵€鏈熸湜鐨?H264 鍒囩墖璧峰鐮併€傝鎺т欢鐢ㄤ綔 V4L2_PIX_FMT_H264_SLICE 鍍忕礌鏍煎紡
    鐨勪慨楗扮銆傛敮鎸?V4L2_PIX_FMT_H264_SLICE 鐨勫簲鐢ㄧ▼搴忓繀椤昏缃鎺т欢锛屼互鎸囧畾缂撳啿鍖烘墍鏈熸湜鐨?
    璧峰鐮併€傞┍鍔ㄥ彲鑳芥牴鎹叾鎵€鏀寔鐨勮兘鍔涳紝鏆撮湶鍗曚釜鎴栧涓捣濮嬬爜銆?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       4 1 4

    - - `V4L2_STATELESS_H264_START_CODE_NONE`
      - 0
      - 閫夋嫨璇ュ€艰〃绀?H264 鍒囩墖涓嶅甫浠讳綍璧峰鐮佸湴浼犻€掔粰椹卞姩銆傜爜娴佹暟鎹簲閬靛惊 h264 7.3.1
        NAL 鍗曞厓璇硶锛屽洜姝ゅ湪闇€瑕佹椂浼氬寘鍚豢鐪熼闃插瓧鑺傦紙emulation prevention bytes锛夈€?
    - - `V4L2_STATELESS_H264_START_CODE_ANNEX_B`
      - 1
      - 閫夋嫨璇ュ€艰〃绀烘湡鏈?H264 鍒囩墖浠?Annex B 璧峰鐮佷綔涓哄墠缂€銆備緷鎹?h264锛屾湁鏁堢殑璧峰鐮佸彲浠ユ槸
        3 瀛楄妭鐨?0x000001 鎴?4 瀛楄妭鐨?0x00000001銆?


    \normalsize


`V4L2_CID_STATELESS_FWHT_PARAMS (struct)`
    鎸囧畾涓庣浉搴?FWHT 鏁版嵁鍏宠仈鐨?FWHT锛堝揩閫?Walsh-Hadamard 鍙樻崲锛夊弬鏁帮紙浠庣爜娴佷腑鎻愬彇锛夈€?
    鍏朵腑鍖呭惈閰嶇疆 FWHT 鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎鎵€闇€鐨勫弬鏁般€傝缂栬В鐮佸櫒涓撶敤浜?vicodec 娴嬭瘯椹卞姩銆?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `backward_ref_ts`
      - 鐢ㄤ綔鍚庡悜鍙傝€冪殑 V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓?P 甯ч厤鍚堜娇鐢ㄣ€傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u32
      - `version`
      - 缂栬В鐮佸櫒鐗堟湰銆傝涓?`V4L2_FWHT_VERSION`銆?
    - - __u32
      - `width`
      - 甯х殑瀹藉害銆?
    - - __u32
      - `height`
      - 甯х殑楂樺害銆?
    - - __u32
      - `flags`
      - 甯х殑鏍囧織锛屽弬瑙?fwht-flags銆?
    - - __u32
      - `colorspace`
      - 甯х殑鑹插僵绌洪棿锛屽彇鑷灇涓?`v4l2_colorspace`銆?
    - - __u32
      - `xfer_func`
      - 浼犺緭鍑芥暟锛屽彇鑷灇涓?`v4l2_xfer_func`銆?
    - - __u32
      - `ycbcr_enc`
      - Y'CbCr 缂栫爜锛屽彇鑷灇涓?`v4l2_ycbcr_encoding`銆?
    - - __u32
      - `quantization`
      - 閲忓寲鑼冨洿锛屽彇鑷灇涓?`v4l2_quantization`銆?


    \normalsize


## FWHT 鏍囧織



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FWHT_FL_IS_INTERLACED`
      - 0x00000001
      - 鑻ヨ缃紝琛ㄧず涓洪殧琛屾牸寮忋€?
    - - `V4L2_FWHT_FL_IS_BOTTOM_FIRST`
      - 0x00000002
      - 鑻ヨ缃紝琛ㄧず涓哄簳鍦轰紭鍏堬紙NTSC锛夌殑闅旇鏍煎紡銆?
    - - `V4L2_FWHT_FL_IS_ALTERNATE`
      - 0x00000004
      - 鑻ヨ缃紝琛ㄧず姣忎釜鈥滃抚鈥濅粎鍖呭惈涓€涓満銆?
    - - `V4L2_FWHT_FL_IS_BOTTOM_FIELD`
      - 0x00000008
      - 鑻ヨ缃簡 V4L2_FWHT_FL_IS_ALTERNATE锛屽垯鏈爣蹇楀湪璇モ€滃抚鈥濅负搴曞満鏃惰缃紝
        鍚﹀垯涓洪《鍦恒€?
    - - `V4L2_FWHT_FL_LUMA_IS_UNCOMPRESSED`
      - 0x00000010
      - 鑻ヨ缃紝琛ㄧず Y'锛堜寒搴︼級骞抽潰鏈粡鍘嬬缉銆?
    - - `V4L2_FWHT_FL_CB_IS_UNCOMPRESSED`
      - 0x00000020
      - 鑻ヨ缃紝琛ㄧず Cb 骞抽潰鏈粡鍘嬬缉銆?
    - - `V4L2_FWHT_FL_CR_IS_UNCOMPRESSED`
      - 0x00000040
      - 鑻ヨ缃紝琛ㄧず Cr 骞抽潰鏈粡鍘嬬缉銆?
    - - `V4L2_FWHT_FL_CHROMA_FULL_HEIGHT`
      - 0x00000080
      - 鑻ヨ缃紝琛ㄧず鑹插害骞抽潰涓庝寒搴﹀钩闈㈤珮搴︾浉鍚岋紝鍚﹀垯鑹插害骞抽潰楂樺害涓轰寒搴﹀钩闈㈢殑涓€鍗娿€?
    - - `V4L2_FWHT_FL_CHROMA_FULL_WIDTH`
      - 0x00000100
      - 鑻ヨ缃紝琛ㄧず鑹插害骞抽潰涓庝寒搴﹀钩闈㈠搴︾浉鍚岋紝鍚﹀垯鑹插害骞抽潰瀹藉害涓轰寒搴﹀钩闈㈢殑涓€鍗娿€?
    - - `V4L2_FWHT_FL_ALPHA_IS_UNCOMPRESSED`
      - 0x00000200
      - 鑻ヨ缃紝琛ㄧず alpha 骞抽潰鏈粡鍘嬬缉銆?
    - - `V4L2_FWHT_FL_I_FRAME`
      - 0x00000400
      - 鑻ヨ缃紝琛ㄧず涓?I 甯с€?
    - - `V4L2_FWHT_FL_COMPONENTS_NUM_MSK`
      - 0x00070000
      - 棰滆壊鍒嗛噺鏁板噺涓€銆?
    - - `V4L2_FWHT_FL_PIXENC_MSK`
      - 0x00180000
      - 鍍忕礌缂栫爜鐨勬帺鐮併€?
    - - `V4L2_FWHT_FL_PIXENC_YUV`
      - 0x00080000
      - 鑻ヨ缃紝琛ㄧず鍍忕礌缂栫爜涓?YUV銆?
    - - `V4L2_FWHT_FL_PIXENC_RGB`
      - 0x00100000
      - 鑻ヨ缃紝琛ㄧず鍍忕礌缂栫爜涓?RGB銆?
    - - `V4L2_FWHT_FL_PIXENC_HSV`
      - 0x00180000
      - 鑻ヨ缃紝琛ㄧず鍍忕礌缂栫爜涓?HSV銆?


    \normalsize


`V4L2_CID_STATELESS_VP8_FRAME (struct)`
    鎸囧畾涓庣浉搴?VP8 宸茶В鏋愬抚鏁版嵁鍏宠仈鐨勫抚鍙傛暟銆傚叾涓寘鍚厤缃?VP8 鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎
    鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?vp8 鏍囧噯瀹氫箟銆?



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_vp8_segment`
      - `segment`
      - 鍖呭惈鍩轰簬鍒嗘鐨勮皟鏁村厓鏁版嵁鐨勭粨鏋勪綋銆?
    - - struct `v4l2_vp8_loop_filter`
      - `lf`
      - 鍖呭惈鐜矾婊ゆ尝鍣ㄧ瓑绾ц皟鏁村厓鏁版嵁鐨勭粨鏋勪綋銆?
    - - struct `v4l2_vp8_quantization`
      - `quant`
      - 鍖呭惈 VP8 鍙嶉噺鍖栫储寮曞厓鏁版嵁鐨勭粨鏋勪綋銆?
    - - struct `v4l2_vp8_entropy`
      - `entropy`
      - 鍖呭惈 VP8 鐔电紪鐮佸櫒姒傜巼鍏冩暟鎹殑缁撴瀯浣撱€?
    - - struct `v4l2_vp8_entropy_coder_state`
      - `coder_state`
      - 鍖呭惈 VP8 鐔电紪鐮佸櫒鐘舵€佺殑缁撴瀯浣撱€?
    - - __u16
      - `width`
      - 甯х殑瀹藉害銆傛墍鏈夊抚閮藉繀椤昏缃€?
    - - __u16
      - `height`
      - 甯х殑楂樺害銆傛墍鏈夊抚閮藉繀椤昏缃€?
    - - __u8
      - `horizontal_scale`
      - 姘村钩缂╂斁鍥犲瓙銆?
    - - __u8
      - `vertical_scale`
      - 鍨傜洿缂╂斁鍥犲瓙銆?
    - - __u8
      - `version`
      - 鐮佹祦鐗堟湰銆?
    - - __u8
      - `prob_skip_false`
      - 琛ㄧず瀹忓潡鏈璺宠繃鐨勬鐜囥€?
    - - __u8
      - `prob_intra`
      - 琛ㄧず瀹忓潡杩涜甯у唴棰勬祴鐨勬鐜囥€?
    - - __u8
      - `prob_last`
      - 琛ㄧず甯ч棿棰勬祴涓娇鐢ㄤ笂涓€鍙傝€冨抚鐨勬鐜囥€?
    - - __u8
      - `prob_gf`
      - 琛ㄧず甯ч棿棰勬祴涓娇鐢ㄩ粍閲戝弬鑰冨抚鐨勬鐜囥€?
    - - __u8
      - `num_dct_parts`
      - DCT 绯绘暟鍒嗗尯鐨勬暟閲忋€傚繀椤讳负 1銆?銆? 鎴?8 涔嬩竴銆?
    - - __u32
      - `first_part_size`
      - 绗竴涓垎鍖猴紙鍗虫帶鍒跺垎鍖猴級鐨勫ぇ灏忋€?
    - - __u32
      - `first_part_header_bits`
      - 绗竴涓垎鍖哄ご閮ㄩ儴鍒嗙殑澶у皬锛堜互姣旂壒璁★級銆?
    - - __u32
      - `dct_part_sizes[^8^]`
      - DCT 绯绘暟鐨勫ぇ灏忋€?
    - - __u64
      - `last_frame_ts`
      - 鐢ㄤ綔涓婁竴鍙傝€冨抚鐨?V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓庡抚闂寸紪鐮佸抚閰嶅悎浣跨敤銆傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `golden_frame_ts`
      - 鐢ㄤ綔涓婁竴鍙傝€冨抚鐨?V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓庡抚闂寸紪鐮佸抚閰嶅悎浣跨敤銆傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `alt_frame_ts`
      - 鐢ㄤ綔澶囩敤鍙傝€冨抚鐨?V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓庡抚闂寸紪鐮佸抚閰嶅悎浣跨敤銆傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `flags`
      - 鍙傝甯ф爣蹇?<vp8_frame_flags>


    \normalsize


`甯ф爣蹇梎



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_FRAME_FLAG_KEY_FRAME`
      - 0x01
      - 琛ㄧず璇ュ抚鏄惁涓哄叧閿抚銆?
    - - `V4L2_VP8_FRAME_FLAG_EXPERIMENTAL`
      - 0x02
      - 瀹為獙鎬х爜娴併€?
    - - `V4L2_VP8_FRAME_FLAG_SHOW_FRAME`
      - 0x04
      - 鏄剧ず甯ф爣蹇楋紝琛ㄧず璇ュ抚鏄惁鐢ㄤ簬鏄剧ず銆?
    - - `V4L2_VP8_FRAME_FLAG_MB_NO_SKIP_COEFF`
      - 0x08
      - 鍚敤/绂佺敤璺宠繃鏃犻潪闆剁郴鏁扮殑瀹忓潡銆?
    - - `V4L2_VP8_FRAME_FLAG_SIGN_BIAS_GOLDEN`
      - 0x10
      - 寮曠敤榛勯噾甯ф椂杩愬姩鐭㈤噺鐨勭鍙枫€?
    - - `V4L2_VP8_FRAME_FLAG_SIGN_BIAS_ALT`
      - 0x20
      - 寮曠敤澶囩敤甯ф椂杩愬姩鐭㈤噺鐨勭鍙枫€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `range`
      - 鈥淩ange鈥濈殑缂栫爜鍣ㄧ姸鎬佸€笺€?
    - - __u8
      - `value`
      - 鈥淰alue鈥濈殑缂栫爜鍣ㄧ姸鎬佸€笺€?
    - - __u8
      - `bit_count`
      - 鍓╀綑鐨勬瘮鐗规暟銆?
    - - __u8
      - `padding`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `quant_update[^4^]`
      - 鏈夌鍙烽噺鍖栧櫒鍊兼洿鏂般€?
    - - __s8
      - `lf_update[^4^]`
      - 鏈夌鍙风幆璺护娉㈠櫒绛夌骇鍊兼洿鏂般€?
    - - __u8
      - `segment_probs[^3^]`
      - 鍒嗘姒傜巼銆?
    - - __u8
      - `padding`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u32
      - `flags`
      - 鍙傝鍒嗘鏍囧織 <vp8_segment_flags>


`鍒嗘鏍囧織`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_SEGMENT_FLAG_ENABLED`
      - 0x01
      - 鍚敤/绂佺敤鍩轰簬鍒嗘鐨勮皟鏁淬€?
    - - `V4L2_VP8_SEGMENT_FLAG_UPDATE_MAP`
      - 0x02
      - 琛ㄧず鏈抚鏄惁鏇存柊瀹忓潡鍒嗘鏄犲皠銆?
    - - `V4L2_VP8_SEGMENT_FLAG_UPDATE_FEATURE_DATA`
      - 0x04
      - 琛ㄧず鏈抚鏄惁鏇存柊鍒嗘鐗瑰緛鏁版嵁銆?
    - - `V4L2_VP8_SEGMENT_FLAG_DELTA_VALUE_MODE`
      - 0x08
      - 鑻ヨ缃紝鍒嗘鐗瑰緛鏁版嵁妯″紡涓?delta-value锛堝樊鍊硷級锛涜嫢娓呴櫎锛屽垯涓?absolute-value
        锛堢粷瀵瑰€硷級銆?


    \normalsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `ref_frm_delta[^4^]`
      - 鍙傝€冭皟鏁达紙鏈夌鍙凤級宸€笺€?
    - - __s8
      - `mb_mode_delta[^4^]`
      - 瀹忓潡棰勬祴妯″紡璋冩暣锛堟湁绗﹀彿锛夊樊鍊笺€?
    - - __u8
      - `sharpness_level`
      - 閿愬害绛夌骇銆?
    - - __u8
      - `level`
      - 婊ゆ尝鍣ㄧ瓑绾с€?
    - - __u16
      - `padding`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u32
      - `flags`
      - 鍙傝鐜矾婊ゆ尝鍣ㄦ爣蹇?<vp8_loop_filter_flags>


`鐜矾婊ゆ尝鍣ㄦ爣蹇梎

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_LF_ADJ_ENABLE`
      - 0x01
      - 鍚敤/绂佺敤瀹忓潡绾х幆璺护娉㈠櫒璋冩暣銆?
    - - `V4L2_VP8_LF_DELTA_UPDATE`
      - 0x02
      - 琛ㄧず璋冩暣涓墍浣跨敤鐨勫樊鍊兼槸鍚︽洿鏂般€?
    - - `V4L2_VP8_LF_FILTER_TYPE_SIMPLE`
      - 0x04
      - 鑻ヨ缃紝琛ㄧず婊ゆ尝鍣ㄧ被鍨嬩负 simple锛堢畝鍗曞瀷锛夛紱鑻ユ竻闄わ紝鍒欎负 normal锛堟櫘閫氬瀷锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `y_ac_qi`
      - 浜害 AC 绯绘暟琛ㄧ储寮曘€?
    - - __s8
      - `y_dc_delta`
      - 浜害 DC 宸€笺€?
    - - __s8
      - `y2_dc_delta`
      - Y2 鍧?DC 宸€笺€?
    - - __s8
      - `y2_ac_delta`
      - Y2 鍧?AC 宸€笺€?
    - - __s8
      - `uv_dc_delta`
      - 鑹插害 DC 宸€笺€?
    - - __s8
      - `uv_ac_delta`
      - 鑹插害 AC 宸€笺€?
    - - __u16
      - `padding`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `coeff_probs[^4^][^8^][^3^][^11^]`
      - 绯绘暟鏇存柊姒傜巼銆?
    - - __u8
      - `y_mode_probs[^4^]`
      - 浜害妯″紡鏇存柊姒傜巼銆?
    - - __u8
      - `uv_mode_probs[^3^]`
      - 鑹插害妯″紡鏇存柊姒傜巼銆?
    - - __u8
      - `mv_probs[^2^][^19^]`
      - MV 瑙ｇ爜鏇存柊姒傜巼銆?
    - - __u8
      - `padding[^3^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`V4L2_CID_STATELESS_MPEG2_SEQUENCE (struct)`
    鎸囧畾涓庣浉搴?MPEG-2 鍒囩墖鏁版嵁鍏宠仈鐨勫簭鍒楀弬鏁帮紙浠庣爜娴佷腑鎻愬彇锛夈€傚叾涓寘鍚笌 mpeg2part2
    瑙勮寖涓簭鍒楀ご锛坰equence header锛夊拰搴忓垪鎵╁睍锛坰equence extension锛夐儴鍒嗚娉曞厓绱犲搴旂殑瀛楁銆?



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `horizontal_size`
      - 甯т寒搴﹀垎閲忓彲鏄剧ず閮ㄥ垎鐨勫搴︺€?
    - - __u16
      - `vertical_size`
      - 甯т寒搴﹀垎閲忓彲鏄剧ず閮ㄥ垎鐨勯珮搴︺€?
    - - __u32
      - `vbv_buffer_size`
      - 鐢ㄤ簬璁＄畻瑙嗛缂撳啿鏍￠獙鍣紙video buffering verifier锛夋墍闇€澶у皬锛屼互姣旂壒瀹氫箟涓猴細
        16 * 1024 * vbv_buffer_size銆?
    - - __u16
      - `profile_and_level_indication`
      - 浠庣爜娴佷腑鎻愬彇鐨勫綋鍓嶆。娆′笌绾у埆鎸囩ず銆?
    - - __u8
      - `chroma_format`
      - 鑹插害瀛愰噰鏍锋牸寮忥紙1锛?:2:0锛?锛?:2:2锛?锛?:4:4锛夈€?
    - - __u8
      - `flags`
      - 鍙傝 MPEG-2 搴忓垪鏍囧織 <mpeg2_sequence_flags>銆?


`MPEG-2 搴忓垪鏍囧織`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_MPEG2_SEQ_FLAG_PROGRESSIVE`
      - 0x01
      - 鎸囩ず璇ュ簭鍒楃殑鎵€鏈夊抚鍧囦负閫愯锛坧rogressive锛夎€岄潪闅旇锛坕nterlaced锛夈€?


    \normalsize

`V4L2_CID_STATELESS_MPEG2_PICTURE (struct)`
    鎸囧畾涓庣浉搴?MPEG-2 鍒囩墖鏁版嵁鍏宠仈鐨勫浘鍍忓弬鏁帮紙浠庣爜娴佷腑鎻愬彇锛夈€傚叾涓寘鍚笌 mpeg2part2
    瑙勮寖涓浘鍍忓ご锛坧icture header锛夊拰鍥惧儚缂栫爜鎵╁睍锛坧icture coding extension锛夐儴鍒嗚娉曞厓绱?
    瀵瑰簲鐨勫瓧娈点€?



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `backward_ref_ts`
      - 鐢ㄤ綔鍚庡悜鍙傝€冪殑 V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓?B 甯у拰 P 甯ч厤鍚堜娇鐢ㄣ€傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `forward_ref_ts`
      - 鐢ㄤ綔鍓嶅悜鍙傝€冪殑 V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓?B 甯ч厤鍚堜娇鐢ㄣ€傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u32
      - `flags`
      - 鍙傝 MPEG-2 鍥惧儚鏍囧織 <mpeg2_picture_flags>銆?
    - - __u8
      - `f_code[^2^][^2^]`
      - 杩愬姩鐭㈤噺鐮併€?
    - - __u8
      - `picture_coding_type`
      - 褰撳墠鍒囩墖鎵€瑕嗙洊甯х殑鍥惧儚缂栫爜绫诲瀷锛圴4L2_MPEG2_PIC_CODING_TYPE_I銆?
        V4L2_MPEG2_PIC_CODING_TYPE_P 鎴?V4L2_MPEG2_PIC_CODING_TYPE_B锛夈€?
    - - __u8
      - `picture_structure`
      - 鍥惧儚缁撴瀯锛?锛氶殧琛岄《鍦猴紝2锛氶殧琛屽簳鍦猴紝3锛氶€愯甯э級銆?
    - - __u8
      - `intra_dc_precision`
      - 绂绘暎浣欏鸡鍙樻崲锛圖CT锛夌殑绮惧害锛?锛? 浣嶇簿搴︼紝1锛? 浣嶇簿搴︼紝2锛?0 浣嶇簿搴︼紝3锛?1 浣嶇簿搴︼級銆?
    - - __u8
      - `reserved[^5^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`MPEG-2 鍥惧儚鏍囧織`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST`
      - 0x00000001
      - 鑻ヨ缃笖涓洪殧琛岀爜娴侊紝鍒欏厛杈撳嚭椤跺満銆?
    - - `V4L2_MPEG2_PIC_FLAG_FRAME_PRED_DCT`
      - 0x00000002
      - 鑻ヨ缃紝鍒欎粎浣跨敤甯?DCT 涓庡抚棰勬祴銆?
    - - `V4L2_MPEG2_PIC_FLAG_CONCEALMENT_MV`
      - 0x00000004
      - 鑻ヨ缃紝鍒欎负甯у唴瀹忓潡缂栫爜杩愬姩鐭㈤噺銆?
    - - `V4L2_MPEG2_PIC_FLAG_Q_SCALE_TYPE`
      - 0x00000008
      - 璇ユ爣蹇楀奖鍝嶅弽閲忓寲杩囩▼銆?
    - - `V4L2_MPEG2_PIC_FLAG_INTRA_VLC`
      - 0x00000010
      - 璇ユ爣蹇楀奖鍝嶅彉鎹㈢郴鏁版暟鎹殑瑙ｇ爜銆?
    - - `V4L2_MPEG2_PIC_FLAG_ALT_SCAN`
      - 0x00000020
      - 璇ユ爣蹇楀奖鍝嶅彉鎹㈢郴鏁版暟鎹殑瑙ｇ爜銆?
    - - `V4L2_MPEG2_PIC_FLAG_REPEAT_FIRST`
      - 0x00000040
      - 璇ユ爣蹇楀奖鍝嶉€愯甯х殑瑙ｇ爜杩囩▼銆?
    - - `V4L2_MPEG2_PIC_FLAG_PROGRESSIVE`
      - 0x00000080
      - 鎸囩ず褰撳墠甯ф槸鍚︿负閫愯銆?


    \normalsize

`V4L2_CID_STATELESS_MPEG2_QUANTISATION (struct)`
    浠ヤ箣瀛楀舰鎵弿椤哄簭鎸囧畾涓庣浉搴?MPEG-2 鍒囩墖鏁版嵁鍏宠仈鐨勯噺鍖栫煩闃点€傝鎺т欢鐢卞唴鏍稿垵濮嬪寲涓?
    鐭╅樀鐨勯粯璁ゅ€笺€傝嫢鐮佹祦浼犺緭浜嗙敤鎴疯嚜瀹氫箟鐨勯噺鍖栫煩闃靛姞杞斤紝鍒欏簲鐢ㄧ▼搴忓簲浣跨敤璇ユ帶浠躲€?
    鑻ラ渶瑕侀噸缃噺鍖栫煩闃碉紙渚嬪鍦ㄥ簭鍒楀ご澶勶級锛屽簲鐢ㄧ▼搴忚繕搴旇缃鎺т欢浠ュ姞杞介粯璁ゅ€笺€?
    璇ヨ繃绋嬬敱瑙勮寖绗?6.3.7 鑺?鈥淨uant matrix extension鈥濓紙閲忓寲鐭╅樀鎵╁睍锛夎瀹氥€?



    \small

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `intra_quantiser_matrix[^64^]`
      - 甯у唴缂栫爜甯х殑閲忓寲鐭╅樀绯绘暟锛屾寜涔嬪瓧褰㈡壂鎻忛『搴忋€傚畠瀵逛寒搴﹀拰鑹插害鍒嗛噺鍧囩浉鍏筹紝
        浣嗗湪闈?4:2:0 鐨?YUV 鏍煎紡涓嬶紝鍙鑹插害涓撶敤鐭╅樀鍙栦唬銆?
    - - __u8
      - `non_intra_quantiser_matrix[^64^]`
      - 闈炲抚鍐呯紪鐮佸抚鐨勯噺鍖栫煩闃电郴鏁帮紝鎸変箣瀛楀舰鎵弿椤哄簭銆傚畠瀵逛寒搴﹀拰鑹插害鍒嗛噺鍧囩浉鍏筹紝
        浣嗗湪闈?4:2:0 鐨?YUV 鏍煎紡涓嬶紝鍙鑹插害涓撶敤鐭╅樀鍙栦唬銆?
    - - __u8
      - `chroma_intra_quantiser_matrix[^64^]`
      - 甯у唴缂栫爜甯ц壊搴﹀垎閲忕殑閲忓寲鐭╅樀绯绘暟锛屾寜涔嬪瓧褰㈡壂鎻忛『搴忋€備粎涓庨潪 4:2:0 鐨?YUV 鏍煎紡鐩稿叧銆?
    - - __u8
      - `chroma_non_intra_quantiser_matrix[^64^]`
      - 闈炲抚鍐呯紪鐮佸抚鑹插害鍒嗛噺鐨勯噺鍖栫煩闃电郴鏁帮紝鎸変箣瀛楀舰鎵弿椤哄簭銆備粎涓庨潪 4:2:0 鐨?YUV 鏍煎紡鐩稿叧銆?


    \normalsize


`V4L2_CID_STATELESS_VP9_COMPRESSED_HDR (struct)`
    瀛樺偍浠庡綋鍓嶅帇缂╁抚澶磋В鏋愬緱鍒扮殑 VP9 姒傜巼鏇存柊銆傛暟缁勫厓绱犱腑鐨勯浂鍊艰〃绀轰笉鏇存柊鐩稿簲鐨勬鐜囥€?
    涓庤繍鍔ㄧ煝閲忕浉鍏崇殑鏇存柊鍖呭惈鏂板€兼垨闆躲€傛墍鏈夊叾浠栨洿鏂板寘鍚粡 inv_map_table[] 杞崲鍚庣殑鍊?
    锛堝弬瑙?vp9 瑙勮寖 6.3.5 鑺傦級銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `tx_mode`
      - 鎸囧畾 TX 妯″紡銆傛洿澶氱粏鑺傚弬瑙?TX 妯″紡 <vp9_tx_mode>銆?
    - - __u8
      - `tx8[^2^][^1^]`
      - TX 8x8 姒傜巼宸€笺€?
    - - __u8
      - `tx16[^2^][^2^]`
      - TX 16x16 姒傜巼宸€笺€?
    - - __u8
      - `tx32[^2^][^3^]`
      - TX 32x32 姒傜巼宸€笺€?
    - - __u8
      - `coef[^4^][^2^][^2^][^6^][^6^][^3^]`
      - 绯绘暟姒傜巼宸€笺€?
    - - __u8
      - `skip[^3^]`
      - 璺宠繃姒傜巼宸€笺€?
    - - __u8
      - `inter_mode[^7^][^3^]`
      - 甯ч棿棰勬祴妯″紡姒傜巼宸€笺€?
    - - __u8
      - `interp_filter[^4^][^2^]`
      - 鎻掑€兼护娉㈠櫒姒傜巼宸€笺€?
    - - __u8
      - `is_inter[^4^]`
      - 鏄惁涓哄抚闂村潡姒傜巼宸€笺€?
    - - __u8
      - `comp_mode[^5^]`
      - 澶嶅悎棰勬祴妯″紡姒傜巼宸€笺€?
    - - __u8
      - `single_ref[^5^][^2^]`
      - 鍗曚竴鍙傝€冩鐜囧樊鍊笺€?
    - - __u8
      - `comp_ref[^5^]`
      - 澶嶅悎鍙傝€冩鐜囧樊鍊笺€?
    - - __u8
      - `y_mode[^4^][^9^]`
      - Y 棰勬祴妯″紡姒傜巼宸€笺€?
    - - __u8
      - `uv_mode[^10^][^9^]`
      - UV 棰勬祴妯″紡姒傜巼宸€笺€?
    - - __u8
      - `partition[^16^][^3^]`
      - 鍒嗗尯姒傜巼宸€笺€?
    - - __u8
      - `mv.joint[^3^]`
      - 杩愬姩鐭㈤噺鑱斿悎姒傜巼宸€笺€?
    - - __u8
      - `mv.sign[^2^]`
      - 杩愬姩鐭㈤噺绗﹀彿姒傜巼宸€笺€?
    - - __u8
      - `mv.classes[^2^][^10^]`
      - 杩愬姩鐭㈤噺绫诲埆姒傜巼宸€笺€?
    - - __u8
      - `mv.class0_bit[^2^]`
      - 杩愬姩鐭㈤噺 class0 姣旂壒姒傜巼宸€笺€?
    - - __u8
      - `mv.bits[^2^][^10^]`
      - 杩愬姩鐭㈤噺姣旂壒姒傜巼宸€笺€?
    - - __u8
      - `mv.class0_fr[^2^][^2^][^3^]`
      - 杩愬姩鐭㈤噺 class0 鍒嗘暟姣旂壒姒傜巼宸€笺€?
    - - __u8
      - `mv.fr[^2^][^3^]`
      - 杩愬姩鐭㈤噺鍒嗘暟姣旂壒姒傜巼宸€笺€?
    - - __u8
      - `mv.class0_hp[^2^]`
      - 杩愬姩鐭㈤噺 class0 楂樼簿搴﹀垎鏁版瘮鐗规鐜囧樊鍊笺€?
    - - __u8
      - `mv.hp[^2^]`
      - 杩愬姩鐭㈤噺楂樼簿搴﹀垎鏁版瘮鐗规鐜囧樊鍊笺€?


`TX 妯″紡`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_TX_MODE_ONLY_4X4`
      - 0
      - 鍙樻崲灏哄涓?4x4銆?
    - - `V4L2_VP9_TX_MODE_ALLOW_8X8`
      - 1
      - 鍙樻崲灏哄鏈€澶у彲涓?8x8銆?
    - - `V4L2_VP9_TX_MODE_ALLOW_16X16`
      - 2
      - 鍙樻崲灏哄鏈€澶у彲涓?16x16銆?
    - - `V4L2_VP9_TX_MODE_ALLOW_32X32`
      - 3
      - 鍙樻崲灏哄鏈€澶у彲涓?32x32銆?
    - - `V4L2_VP9_TX_MODE_SELECT`
      - 4
      - 鐮佹祦涓寘鍚瘡涓潡鐨勫彉鎹㈠昂瀵搞€?

鍙傝 vp9 瑙勮寖 鈥?.3.1 Tx mode semantics鈥濓紙Tx 妯″紡璇箟锛変竴鑺傝幏鍙栨洿澶氱粏鑺傘€?

`V4L2_CID_STATELESS_VP9_FRAME (struct)`
    鎸囧畾涓庣浉搴?VP9 甯цВ鐮佽姹傚叧鑱旂殑甯у弬鏁般€傚叾涓寘鍚厤缃?VP9 鏃犵姸鎬佺‖浠惰В鐮佹祦姘寸嚎
    鎵€闇€鐨勫弬鏁般€傜爜娴佸弬鏁颁緷鎹?vp9 鏍囧噯瀹氫箟銆?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_vp9_loop_filter`
      - `lf`
      - 鐜矾婊ゆ尝鍣ㄥ弬鏁般€傛洿澶氱粏鑺傚弬瑙佺粨鏋勪綋 `v4l2_vp9_loop_filter`銆?
    - - struct `v4l2_vp9_quantization`
      - `quant`
      - 閲忓寲鍙傛暟銆傛洿澶氱粏鑺傚弬瑙?`v4l2_vp9_quantization`銆?
    - - struct `v4l2_vp9_segmentation`
      - `seg`
      - 鍒嗘鍙傛暟銆傛洿澶氱粏鑺傚弬瑙?`v4l2_vp9_segmentation`銆?
    - - __u32
      - `flags`
      - V4L2_VP9_FRAME_FLAG_* 鏍囧織鐨勭粍鍚堛€傚弬瑙佸抚鏍囧織 <vp9_frame_flags>銆?
    - - __u16
      - `compressed_header_size`
      - 鍘嬬缉澶撮儴鐨勫ぇ灏忥紙瀛楄妭锛夈€?
    - - __u16
      - `uncompressed_header_size`
      - 鏈帇缂╁ご閮ㄧ殑澶у皬锛堝瓧鑺傦級銆?
    - - __u16
      - `frame_width_minus_1`
      - 鍔?1 寰楀埌浠ュ儚绱犺〃绀虹殑甯у搴︺€傚弬瑙?vp9 瑙勮寖绗?7.2.3 鑺傘€?
    - - __u16
      - `frame_height_minus_1`
      - 鍔?1 寰楀埌浠ュ儚绱犺〃绀虹殑甯ч珮搴︺€傚弬瑙?vp9 瑙勮寖绗?7.2.3 鑺傘€?
    - - __u16
      - `render_width_minus_1`
      - 鍔?1 寰楀埌鏈熸湜鐨勬覆鏌撳搴︼紙浠ュ儚绱犺〃绀猴級銆傝鍊间笉鐢ㄤ簬瑙ｇ爜杩囩▼锛屼絾鍙兘琚‖浠剁缉鏀惧櫒
        鐢ㄤ簬鍑嗗鍙緵鎵弿杈撳嚭锛坰canout锛夌殑甯с€傚弬瑙?vp9 瑙勮寖绗?7.2.4 鑺傘€?
    - - __u16
      - render_height_minus_1
      - 鍔?1 寰楀埌鏈熸湜鐨勬覆鏌撻珮搴︼紙浠ュ儚绱犺〃绀猴級銆傝鍊间笉鐢ㄤ簬瑙ｇ爜杩囩▼锛屼絾鍙兘琚‖浠剁缉鏀惧櫒
        鐢ㄤ簬鍑嗗鍙緵鎵弿杈撳嚭鐨勫抚銆傚弬瑙?vp9 瑙勮寖绗?7.2.4 鑺傘€?
    - - __u64
      - `last_frame_ts`
      - 鈥渓ast鈥濆弬鑰冪紦鍐插尯鐨勬椂闂存埑銆傝鏃堕棿鎴冲紩鐢?struct `v4l2_buffer` 涓殑 `timestamp`
        瀛楁銆備娇鐢?`v4l2_timeval_to_ns()` 鍑芥暟灏?struct `v4l2_buffer` 涓殑
        struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `golden_frame_ts`
      - 鈥済olden鈥濆弬鑰冪紦鍐插尯鐨勬椂闂存埑銆傝鏃堕棿鎴冲紩鐢?struct `v4l2_buffer` 涓殑 `timestamp`
        瀛楁銆備娇鐢?`v4l2_timeval_to_ns()` 鍑芥暟灏?struct `v4l2_buffer` 涓殑
        struct `timeval` 杞崲涓?__u64銆?
    - - __u64
      - `alt_frame_ts`
      - 鈥渁lt鈥濆弬鑰冪紦鍐插尯鐨勬椂闂存埑銆傝鏃堕棿鎴冲紩鐢?struct `v4l2_buffer` 涓殑 `timestamp`
        瀛楁銆備娇鐢?`v4l2_timeval_to_ns()` 鍑芥暟灏?struct `v4l2_buffer` 涓殑
        struct `timeval` 杞崲涓?__u64銆?
    - - __u8
      - `ref_frame_sign_bias`
      - 浣嶅煙锛屾寚瀹氭槸鍚︿负缁欏畾鍙傝€冨抚璁剧疆浜嗙鍙峰亸缃€傛洿澶氱粏鑺傚弬瑙佸弬鑰冨抚绗﹀彿鍋忕疆
        <vp9_ref_frame_sign_bias>銆?
    - - __u8
      - `reset_frame_context`
      - 鎸囧畾鏄惁搴斿皢甯т笂涓嬫枃閲嶇疆涓洪粯璁ゅ€笺€傛洿澶氱粏鑺傚弬瑙侀噸缃抚涓婁笅鏂?
        <vp9_reset_frame_context>銆?
    - - __u8
      - `frame_context_idx`
      - 搴旇浣跨敤/鏇存柊鐨勫抚涓婁笅鏂囥€?
    - - __u8
      - `profile`
      - VP9 妗ｆ锛坧rofile锛夈€傚彲浠ヤ负 0銆?銆? 鎴?3銆?
    - - __u8
      - `bit_depth`
      - 鍒嗛噺浣嶆繁锛堟瘮鐗癸級銆傚彲浠ヤ负 8銆?0 鎴?12銆傛敞鎰忓苟闈炴墍鏈夋。娆￠兘鏀寔 10 鍜?鎴?12 浣嶆繁銆?
    - - __u8
      - `interpolation_filter`
      - 鎸囧畾鐢ㄤ簬鎵ц甯ч棿棰勬祴鎵€閫夋嫨鐨勬护娉㈠櫒銆傛洿澶氱粏鑺傚弬瑙佹彃鍊兼护娉㈠櫒
        <vp9_interpolation_filter>銆?
    - - __u8
      - `tile_cols_log2`
      - 鎸囧畾姣忎釜 tile 瀹藉害鐨勪互 2 涓哄簳鐨勫鏁帮紙瀹藉害浠?8x8 鍧椾负鍗曚綅搴﹂噺锛夈€傚繀椤诲皬浜庢垨绛変簬 6銆?
    - - __u8
      - `tile_rows_log2`
      - 鎸囧畾姣忎釜 tile 楂樺害鐨勪互 2 涓哄簳鐨勫鏁帮紙楂樺害浠?8x8 鍧椾负鍗曚綅搴﹂噺锛夈€?
    - - __u8
      - `reference_mode`
      - 鎸囧畾瑕佷娇鐢ㄧ殑甯ч棿棰勬祴绫诲瀷銆傛洿澶氱粏鑺傚弬瑙佸弬鑰冩ā寮?<vp9_reference_mode>銆傛敞鎰忚鍊?
        鏄綔涓哄帇缂╁ご閮ㄨВ鏋愯繃绋嬬殑涓€閮ㄥ垎鎺ㄥ鍑烘潵鐨勶紝鍥犳鏈簲灞炰簬 :c:type:
        `v4l2_ctrl_vp9_compressed_hdr` 鍙€夋帶浠躲€傝嫢椹卞姩涓嶉渶瑕佸帇缂╁ご閮紝灏嗘湰鍊艰涓?
        闆舵槸瀹夊叏鐨勩€?
    - - __u8
      - `reserved[^7^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


    \normalsize


`甯ф爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_FRAME_FLAG_KEY_FRAME`
      - 0x001
      - 璇ュ抚涓哄叧閿抚銆?
    - - `V4L2_VP9_FRAME_FLAG_SHOW_FRAME`
      - 0x002
      - 璇ュ抚搴旇鏄剧ず銆?
    - - `V4L2_VP9_FRAME_FLAG_ERROR_RESILIENT`
      - 0x004
      - 瑙ｇ爜搴斿叿鏈夐敊璇煣鎬с€?
    - - `V4L2_VP9_FRAME_FLAG_INTRA_ONLY`
      - 0x008
      - 璇ュ抚涓嶅弬鑰冨叾浠栧抚銆?
    - - `V4L2_VP9_FRAME_FLAG_ALLOW_HIGH_PREC_MV`
      - 0x010
      - 璇ュ抚鍙互浣跨敤楂樼簿搴﹁繍鍔ㄧ煝閲忋€?
    - - `V4L2_VP9_FRAME_FLAG_REFRESH_FRAME_CTX`
      - 0x020
      - 瑙ｇ爜鍚庡簲褰撴洿鏂板抚涓婁笅鏂囥€?
    - - `V4L2_VP9_FRAME_FLAG_PARALLEL_DEC_MODE`
      - 0x040
      - 浣跨敤浜嗗苟琛岃В鐮併€?
    - - `V4L2_VP9_FRAME_FLAG_X_SUBSAMPLING`
      - 0x080
      - 鍚敤浜嗗瀭鐩村瓙閲囨牱銆?
    - - `V4L2_VP9_FRAME_FLAG_Y_SUBSAMPLING`
      - 0x100
      - 鍚敤浜嗘按骞冲瓙閲囨牱銆?
    - - `V4L2_VP9_FRAME_FLAG_COLOR_RANGE_FULL_SWING`
      - 0x200
      - 浣跨敤浜嗗畬鏁寸殑 UV 鑼冨洿銆?


`鍙傝€冨抚绗﹀彿鍋忕疆`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SIGN_BIAS_LAST`
      - 0x1
      - 涓?last 鍙傝€冨抚璁剧疆浜嗙鍙峰亸缃€?
    - - `V4L2_VP9_SIGN_BIAS_GOLDEN`
      - 0x2
      - 涓?golden 鍙傝€冨抚璁剧疆浜嗙鍙峰亸缃€?
    - - `V4L2_VP9_SIGN_BIAS_ALT`
      - 0x2
      - 涓?alt 鍙傝€冨抚璁剧疆浜嗙鍙峰亸缃€?


`閲嶇疆甯т笂涓嬫枃`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_RESET_FRAME_CTX_NONE`
      - 0
      - 涓嶉噸缃换浣曞抚涓婁笅鏂囥€?
    - - `V4L2_VP9_RESET_FRAME_CTX_SPEC`
      - 1
      - 閲嶇疆鐢?`v4l2_ctrl_vp9_frame`.frame_context_idx 鎸囧悜鐨勫抚涓婁笅鏂囥€?
    - - `V4L2_VP9_RESET_FRAME_CTX_ALL`
      - 2
      - 閲嶇疆鎵€鏈夊抚涓婁笅鏂囥€?

鏇村缁嗚妭鍙傝 vp9 瑙勮寖 鈥?.2 Uncompressed header semantics鈥濓紙鏈帇缂╁ご閮ㄨ涔夛級涓€鑺傘€?


`鎻掑€兼护娉㈠櫒`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP`
      - 0
      - 鍏娊澶存护娉㈠櫒銆?
    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP_SMOOTH`
      - 1
      - 鍏娊澶村钩婊戞护娉㈠櫒銆?
    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP_SHARP`
      - 2
      - 鍏娊澶撮攼鍒╂护娉㈠櫒銆?
    - - `V4L2_VP9_INTERP_FILTER_BILINEAR`
      - 3
      - 鍙岀嚎鎬ф护娉㈠櫒銆?
    - - `V4L2_VP9_INTERP_FILTER_SWITCHABLE`
      - 4
      - 婊ゆ尝鍣ㄩ€夋嫨浜庡潡绾у埆鍙戝嚭淇″彿銆?

鏇村缁嗚妭鍙傝 vp9 瑙勮寖 鈥?.2.7 Interpolation filter semantics鈥濓紙鎻掑€兼护娉㈠櫒璇箟锛変竴鑺傘€?


`鍙傝€冩ā寮廯


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_REFERENCE_MODE_SINGLE_REFERENCE`
      - 0
      - 琛ㄧず鎵€鏈夊抚闂村潡浠呬娇鐢ㄥ崟涓弬鑰冨抚鏉ョ敓鎴愯繍鍔ㄨˉ鍋块娴嬨€?
    - - `V4L2_VP9_REFERENCE_MODE_COMPOUND_REFERENCE`
      - 1
      - 瑕佹眰鎵€鏈夊抚闂村潡浣跨敤澶嶅悎妯″紡锛屼笉鍏佽鍗曞弬鑰冨抚棰勬祴銆?
    - - `V4L2_VP9_REFERENCE_MODE_SELECT`
      - 2
      - 鍏佽姣忎釜鐙珛鐨勫抚闂村潡鍦ㄥ崟鍙傝€冧笌澶嶅悎棰勬祴妯″紡涔嬮棿閫夋嫨銆?

鏇村缁嗚妭鍙傝 vp9 瑙勮寖 鈥?.3.6 Frame reference mode semantics鈥濓紙甯у弬鑰冩ā寮忚涔夛級涓€鑺傘€?


缂栫爜閲忓寲鍙傛暟銆傛洿澶氱粏鑺傚弬瑙?vp9 瑙勮寖 鈥?.2.10 Segmentation params syntax鈥濓紙鍒嗘鍙傛暟璇硶锛変竴鑺傘€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `feature_data[^8^][^4^]`
      - 闄勫甫鍦ㄦ瘡涓壒寰佷笂鐨勬暟鎹€備粎褰撶壒寰佽鍚敤鏃舵暟鎹潯鐩墠鏈夋晥銆傝鏁扮粍搴斾互娈电紪鍙蜂綔涓?
        绗竴缁达紙0..7锛夈€佷互 V4L2_VP9_SEG_* 涔嬩竴浣滀负绗簩缁磋繘琛岀储寮曘€傚弬瑙佸垎娈电壒寰?ID
        <vp9_segment_feature>銆?
    - - __u8
      - `feature_enabled[^8^]`
      - 浣嶆帺鐮侊紝瀹氫箟姣忎釜娈典腑鍚敤浜嗗摢浜涚壒寰併€傛瘡涓鐨勫€间负 V4L2_VP9_SEGMENT_FEATURE_ENABLED(id)
        鍊肩殑缁勫悎锛屽叾涓?id 涓?V4L2_VP9_SEG_* 涔嬩竴銆傚弬瑙佸垎娈电壒寰?ID <vp9_segment_feature>銆?
    - - __u8
      - `tree_probs[^7^]`
      - 鎸囧畾瑙ｇ爜 Segment-ID 鏃惰浣跨敤鐨勬鐜囧€笺€傛洿澶氱粏鑺傚弬瑙?vp9 瑙勮寖鐨?鈥?.15 Segmentation map鈥?
        锛堝垎娈垫槧灏勶級涓€鑺傘€?
    - - __u8
      - `pred_probs[^3^]`
      - 鎸囧畾瑙ｇ爜 Predicted-Segment-ID 鏃惰浣跨敤鐨勬鐜囧€笺€傛洿澶氱粏鑺傚弬瑙?vp9 瑙勮寖鐨?
        鈥?.4.14 Get segment id syntax鈥濓紙鑾峰彇娈?ID 璇硶锛変竴鑺傘€?
    - - __u8
      - `flags`
      - V4L2_VP9_SEGMENTATION_FLAG_* 鏍囧織鐨勭粍鍚堛€傚弬瑙佸垎娈垫爣蹇?<vp9_segmentation_flags>銆?
    - - __u8
      - `reserved[^5^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`鍒嗘鐗瑰緛 ID`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SEG_LVL_ALT_Q`
      - 0
      - 閲忓寲鍣ㄥ垎娈电壒寰併€?
    - - `V4L2_VP9_SEG_LVL_ALT_L`
      - 1
      - 鐜矾婊ゆ尝鍣ㄥ垎娈电壒寰併€?
    - - `V4L2_VP9_SEG_LVL_REF_FRAME`
      - 2
      - 鍙傝€冨抚鍒嗘鐗瑰緛銆?
    - - `V4L2_VP9_SEG_LVL_SKIP`
      - 3
      - 璺宠繃鍒嗘鐗瑰緛銆?
    - - `V4L2_VP9_SEG_LVL_MAX`
      - 4
      - 鍒嗘鐗瑰緛鐨勬暟閲忋€?


`鍒嗘鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SEGMENTATION_FLAG_ENABLED`
      - 0x01
      - 琛ㄧず璇ュ抚浣跨敤浜嗗垎娈靛伐鍏凤紙segmentation tool锛夈€?
    - - `V4L2_VP9_SEGMENTATION_FLAG_UPDATE_MAP`
      - 0x02
      - 琛ㄧず璇ュ抚鐨勮В鐮佽繃绋嬩腑搴斿綋鏇存柊鍒嗘鏄犲皠銆?
    - - `V4L2_VP9_SEGMENTATION_FLAG_TEMPORAL_UPDATE`
      - 0x04
      - 琛ㄧず鍒嗘鏄犲皠鐨勬洿鏂版槸鐩稿浜庡凡瀛樺湪鐨勫垎娈垫槧灏勭紪鐮佺殑銆?
    - - `V4L2_VP9_SEGMENTATION_FLAG_UPDATE_DATA`
      - 0x08
      - 琛ㄧず鍗冲皢涓烘瘡涓鎸囧畾鏂扮殑鍙傛暟銆?
    - - `V4L2_VP9_SEGMENTATION_FLAG_ABS_OR_DELTA_UPDATE`
      - 0x10
      - 琛ㄧず鍒嗘鍙傛暟浠ｈ〃瑕佷娇鐢ㄧ殑瀹為檯鍊笺€?


缂栫爜閲忓寲鍙傛暟銆傛洿澶氱粏鑺傚弬瑙?VP9 瑙勮寖 鈥?.2.9 Quantization params syntax鈥濓紙閲忓寲鍙傛暟璇硶锛変竴鑺傘€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `base_q_idx`
      - 琛ㄧず鍩虹甯?qindex銆?
    - - __s8
      - `delta_q_y_dc`
      - 琛ㄧず鐩稿 base_q_idx 鐨?Y DC 閲忓寲鍣ㄣ€?
    - - __s8
      - `delta_q_uv_dc`
      - 琛ㄧず鐩稿 base_q_idx 鐨?UV DC 閲忓寲鍣ㄣ€?
    - - __s8
      - `delta_q_uv_ac`
      - 琛ㄧず鐩稿 base_q_idx 鐨?UV AC 閲忓寲鍣ㄣ€?
    - - __u8
      - `reserved[^4^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


璇ョ粨鏋勪綋鍖呭惈鍏ㄩ儴涓庣幆璺护娉㈠櫒鐩稿叧鐨勫弬鏁般€傛洿澶氱粏鑺傚弬瑙?vp9 瑙勮寖 鈥?.2.8 Loop filter semantics鈥?
锛堢幆璺护娉㈠櫒璇箟锛変竴鑺傘€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `ref_deltas[^4^]`
      - 鍖呭惈鍩轰簬鎵€閫夊弬鑰冨抚瀵规护娉㈠櫒绛夌骇鎵€闇€鐨勮皟鏁淬€?
    - - __s8
      - `mode_deltas[^2^]`
      - 鍖呭惈鍩轰簬鎵€閫夋ā寮忓婊ゆ尝鍣ㄧ瓑绾ф墍闇€鐨勮皟鏁淬€?
    - - __u8
      - `level`
      - 琛ㄧず鐜矾婊ゆ尝鍣ㄥ己搴︺€?
    - - __u8
      - `sharpness`
      - 琛ㄧず閿愬害绛夌骇銆?
    - - __u8
      - `flags`
      - V4L2_VP9_LOOP_FILTER_FLAG_* 鏍囧織鐨勭粍鍚堛€傚弬瑙佺幆璺护娉㈠櫒鏍囧織 <vp9_loop_filter_flags>銆?
    - - __u8
      - `reserved[^7^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?



`鐜矾婊ゆ尝鍣ㄦ爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_LOOP_FILTER_FLAG_DELTA_ENABLED`
      - 0x1
      - 褰撹缃椂锛屾护娉㈠櫒绛夌骇鍙栧喅浜庣敤浜庨娴嬫煇涓潡鐨勬ā寮忓拰鍙傝€冨抚銆?
    - - `V4L2_VP9_LOOP_FILTER_FLAG_DELTA_UPDATE`
      - 0x2
      - 褰撹缃椂锛岀爜娴佸寘鍚澶栫殑璇硶鍏冪礌锛岀敤浜庢寚瀹氬摢浜涙ā寮忓拰鍙傝€冨抚鐨勫樊鍊奸渶瑕佹洿鏂般€?


`V4L2_CID_STATELESS_HEVC_SPS (struct)`
    鎸囧畾涓庣浉搴?HEVC 鍒囩墖鏁版嵁鍏宠仈鐨勫簭鍒楀弬鏁伴泦瀛楁锛堜粠鐮佹祦涓彁鍙栵級銆傝繖浜涚爜娴佸弬鏁颁緷鎹?
    hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.3.2 Sequence parameter set RBSP semantics鈥?
    锛堝簭鍒楀弬鏁伴泦 RBSP 璇箟锛変竴鑺備腑鎻忚堪銆?



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `video_parameter_set_id`
      - 鎸囧畾娲诲姩 VPS 鐨?vps_video_parameter_set_id 鍊硷紝濡?H.265 瑙勮寖
        鈥?.4.3.2.1 General sequence parameter set RBSP semantics鈥濓紙閫氱敤搴忓垪鍙傛暟闆?RBSP 璇箟锛?
        涓€鑺傛墍杩般€?
    - - __u8
      - `seq_parameter_set_id`
      - 涓?SPS 鎻愪緵涓€涓爣璇嗙锛屼緵鍏朵粬璇硶鍏冪礌寮曠敤锛屽 H.265 瑙勮寖
        鈥?.4.3.2.1 General sequence parameter set RBSP semantics鈥濓紙閫氱敤搴忓垪鍙傛暟闆?RBSP 璇箟锛?
        涓€鑺傛墍杩般€?
    - - __u16
      - `pic_width_in_luma_samples`
      - 鎸囧畾姣忓箙瑙ｇ爜鍥惧儚鐨勫搴︼紝浠ヤ寒搴︽牱鏈负鍗曚綅銆?
    - - __u16
      - `pic_height_in_luma_samples`
      - 鎸囧畾姣忓箙瑙ｇ爜鍥惧儚鐨勯珮搴︼紝浠ヤ寒搴︽牱鏈负鍗曚綅銆?
    - - __u8
      - `bit_depth_luma_minus8`
      - 璇ュ€煎姞 8 鎸囧畾浜害鏁扮粍鏍锋湰鐨勪綅娣便€?
    - - __u8
      - `bit_depth_chroma_minus8`
      - 璇ュ€煎姞 8 鎸囧畾鑹插害鏁扮粍鏍锋湰鐨勪綅娣便€?
    - - __u8
      - `log2_max_pic_order_cnt_lsb_minus4`
      - 鎸囧畾鍙橀噺 MaxPicOrderCntLsb 鐨勫€笺€?
    - - __u8
      - `sps_max_dec_pic_buffering_minus1`
      - 璇ュ€煎姞 1 鎸囧畾缂栫爜瑙嗛搴忓垪锛圕VS锛夋墍闇€鐨勮В鐮佸浘鍍忕紦鍐插尯鏈€澶уぇ灏忋€?
    - - __u8
      - `sps_max_num_reorder_pics`
      - 琛ㄧず鍏佽鐨勬渶澶у浘鍍忔暟閲忋€?
    - - __u8
      - `sps_max_latency_increase_plus1`
      - 鐢ㄤ簬鍙戜俊鍙蜂紶閫?MaxLatencyPictures锛岃〃绀哄湪杈撳嚭椤哄簭涓婂彲浠ヤ綅浜庝换鎰忓浘鍍忎箣鍓嶃€佸苟鍦ㄨВ鐮?
        椤哄簭涓婅窡闅忚鍥惧儚鐨勬渶澶у浘鍍忔暟閲忋€?
    - - __u8
      - `log2_min_luma_coding_block_size_minus3`
      - 璇ュ€煎姞 3 鎸囧畾鏈€灏忎寒搴︾紪鐮佸潡澶у皬銆?
    - - __u8
      - `log2_diff_max_min_luma_coding_block_size`
      - 鎸囧畾鏈€澶т笌鏈€灏忎寒搴︾紪鐮佸潡澶у皬涔嬮棿鐨勫樊鍊笺€?
    - - __u8
      - `log2_min_luma_transform_block_size_minus2`
      - 璇ュ€煎姞 2 鎸囧畾鏈€灏忎寒搴﹀彉鎹㈠潡澶у皬銆?
    - - __u8
      - `log2_diff_max_min_luma_transform_block_size`
      - 鎸囧畾鏈€澶т笌鏈€灏忎寒搴﹀彉鎹㈠潡澶у皬涔嬮棿鐨勫樊鍊笺€?
    - - __u8
      - `max_transform_hierarchy_depth_inter`
      - 鎸囧畾浠ュ抚闂撮娴嬫ā寮忕紪鐮佺殑缂栫爜鍗曞厓鐨勫彉鎹㈠崟鍏冪殑鏈€澶у眰绾ф繁搴︺€?
    - - __u8
      - `max_transform_hierarchy_depth_intra`
      - 鎸囧畾浠ュ抚鍐呴娴嬫ā寮忕紪鐮佺殑缂栫爜鍗曞厓鐨勫彉鎹㈠崟鍏冪殑鏈€澶у眰绾ф繁搴︺€?
    - - __u8
      - `pcm_sample_bit_depth_luma_minus1`
      - 璇ュ€煎姞 1 鎸囧畾鐢ㄤ簬琛ㄧず浜害鍒嗛噺鐨勬瘡涓?PCM 鏍锋湰鍊肩殑姣旂壒鏁般€?
    - - __u8
      - `pcm_sample_bit_depth_chroma_minus1`
      - 鎸囧畾鐢ㄤ簬琛ㄧず鑹插害鍒嗛噺鐨勬瘡涓?PCM 鏍锋湰鍊肩殑姣旂壒鏁般€?
    - - __u8
      - `log2_min_pcm_luma_coding_block_size_minus3`
      - 鍔?3 鎸囧畾缂栫爜鍧楃殑鏈€灏忓ぇ灏忋€?
    - - __u8
      - `log2_diff_max_min_pcm_luma_coding_block_size`
      - 鎸囧畾缂栫爜鍧楁渶澶т笌鏈€灏忓ぇ灏忎箣闂寸殑宸€笺€?
    - - __u8
      - `num_short_term_ref_pic_sets`
      - 鎸囧畾 SPS 涓寘鍚殑 st_ref_pic_set() 璇硶缁撴瀯鐨勬暟閲忋€?
    - - __u8
      - `num_long_term_ref_pics_sps`
      - 鎸囧畾鍦?SPS 涓寚瀹氱殑鍊欓€夐暱鏈熷弬鑰冨浘鍍忕殑鏁伴噺銆?
    - - __u8
      - `chroma_format_idc`
      - 鎸囧畾鑹插害閲囨牱鏂瑰紡銆?
    - - __u8
      - `sps_max_sub_layers_minus1`
      - 璇ュ€煎姞 1 鎸囧畾鏃堕棿瀛愬眰鐨勬渶澶ф暟閲忋€?
    - - __u64
      - `flags`
      - 鍙傝搴忓垪鍙傛暟闆嗘爣蹇?<hevc_sps_flags>


    \normalsize


`搴忓垪鍙傛暟闆嗘爣蹇梎


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_SPS_FLAG_SEPARATE_COLOUR_PLANE`
      - 0x00000001
      -
    - - `V4L2_HEVC_SPS_FLAG_SCALING_LIST_ENABLED`
      - 0x00000002
      -
    - - `V4L2_HEVC_SPS_FLAG_AMP_ENABLED`
      - 0x00000004
      -
    - - `V4L2_HEVC_SPS_FLAG_SAMPLE_ADAPTIVE_OFFSET`
      - 0x00000008
      -
    - - `V4L2_HEVC_SPS_FLAG_PCM_ENABLED`
      - 0x00000010
      -
    - - `V4L2_HEVC_SPS_FLAG_PCM_LOOP_FILTER_DISABLED`
      - 0x00000020
      -
    - - `V4L2_HEVC_SPS_FLAG_LONG_TERM_REF_PICS_PRESENT`
      - 0x00000040
      -
    - - `V4L2_HEVC_SPS_FLAG_SPS_TEMPORAL_MVP_ENABLED`
      - 0x00000080
      -
    - - `V4L2_HEVC_SPS_FLAG_STRONG_INTRA_SMOOTHING_ENABLED`
      - 0x00000100
      -


    \normalsize

`V4L2_CID_STATELESS_HEVC_PPS (struct)`
    鎸囧畾涓庣浉搴?HEVC 鍒囩墖鏁版嵁鍏宠仈鐨勫浘鍍忓弬鏁伴泦瀛楁锛堜粠鐮佹祦涓彁鍙栵級銆傝繖浜涚爜娴佸弬鏁颁緷鎹?
    hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.3.3 Picture parameter set RBSP semantics鈥?
    锛堝浘鍍忓弬鏁伴泦 RBSP 璇箟锛変竴鑺備腑鎻忚堪銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `pic_parameter_set_id`
      - 涓?PPS 鎻愪緵涓€涓爣璇嗙锛屼緵鍏朵粬璇硶鍏冪礌寮曠敤銆?
    - - __u8
      - `num_extra_slice_header_bits`
      - 鎸囧畾寮曠敤璇?PPS 鐨勭紪鐮佸浘鍍忕殑鍒囩墖澶?RBSP 涓瓨鍦ㄧ殑棰濆鍒囩墖澶存瘮鐗规暟閲忋€?
    - - __u8
      - `num_ref_idx_l0_default_active_minus1`
      - 璇ュ€煎姞 1 鎸囧畾 num_ref_idx_l0_active_minus1 鐨勬帹瀵煎€笺€?
    - - __u8
      - `num_ref_idx_l1_default_active_minus1`
      - 璇ュ€煎姞 1 鎸囧畾 num_ref_idx_l1_active_minus1 鐨勬帹瀵煎€笺€?
    - - __s8
      - `init_qp_minus26`
      - 璇ュ€煎姞 26 鎸囧畾寮曠敤璇?PPS 鐨勬瘡涓垏鐗囩殑 SliceQp Y 鍒濆鍊笺€?
    - - __u8
      - `diff_cu_qp_delta_depth`
      - 鎸囧畾浜害缂栫爜鏍戝潡澶у皬涓庝紶杈?cu_qp_delta_abs 鍜?cu_qp_delta_sign_flag 鐨勭紪鐮佸崟鍏冪殑
        鏈€灏忎寒搴︾紪鐮佸潡澶у皬涔嬮棿鐨勫樊鍊笺€?
    - - __s8
      - `pps_cb_qp_offset`
      - 鎸囧畾瀵逛寒搴﹂噺鍖栧弬鏁?Cb 鐨勫亸绉汇€?
    - - __s8
      - `pps_cr_qp_offset`
      - 鎸囧畾瀵逛寒搴﹂噺鍖栧弬鏁?Cr 鐨勫亸绉汇€?
    - - __u8
      - `num_tile_columns_minus1`
      - 璇ュ€煎姞 1 鎸囧畾灏嗗浘鍍忓垝鍒嗘垚鐨?tile 鍒楁暟銆?
    - - __u8
      - `num_tile_rows_minus1`
      - 璇ュ€煎姞 1 鎸囧畾灏嗗浘鍍忓垝鍒嗘垚鐨?tile 琛屾暟銆?
    - - __u8
      - `column_width_minus1[^20^]`
      - 璇ュ€煎姞 1 鎸囧畾绗?i 涓?tile 鍒楃殑瀹藉害锛屼互缂栫爜鏍戝潡涓哄崟浣嶃€?
    - - __u8
      - `row_height_minus1[^22^]`
      - 璇ュ€煎姞 1 鎸囧畾绗?i 涓?tile 琛岀殑楂樺害锛屼互缂栫爜鏍戝潡涓哄崟浣嶃€?
    - - __s8
      - `pps_beta_offset_div2`
      - 鎸囧畾 beta 鐨勯粯璁ゅ幓鍧楀弬鏁板亸绉婚櫎浠?2銆?
    - - __s8
      - `pps_tc_offset_div2`
      - 鎸囧畾 tC 鐨勯粯璁ゅ幓鍧楀弬鏁板亸绉婚櫎浠?2銆?
    - - __u8
      - `log2_parallel_merge_level_minus2`
      - 璇ュ€煎姞 2 鎸囧畾鍙橀噺 Log2ParMrgLevel 鐨勫€笺€?
    - - __u8
      - `padding[^4^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u64
      - `flags`
      - 鍙傝鍥惧儚鍙傛暟闆嗘爣蹇?<hevc_pps_flags>


`鍥惧儚鍙傛暟闆嗘爣蹇梎


    \small

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_PPS_FLAG_DEPENDENT_SLICE_SEGMENT_ENABLED`
      - 0x00000001
      -
    - - `V4L2_HEVC_PPS_FLAG_OUTPUT_FLAG_PRESENT`
      - 0x00000002
      -
    - - `V4L2_HEVC_PPS_FLAG_SIGN_DATA_HIDING_ENABLED`
      - 0x00000004
      -
    - - `V4L2_HEVC_PPS_FLAG_CABAC_INIT_PRESENT`
      - 0x00000008
      -
    - - `V4L2_HEVC_PPS_FLAG_CONSTRAINED_INTRA_PRED`
      - 0x00000010
      -
    - - `V4L2_HEVC_PPS_FLAG_TRANSFORM_SKIP_ENABLED`
      - 0x00000020
      -
    - - `V4L2_HEVC_PPS_FLAG_CU_QP_DELTA_ENABLED`
      - 0x00000040
      -
    - - `V4L2_HEVC_PPS_FLAG_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT`
      - 0x00000080
      -
    - - `V4L2_HEVC_PPS_FLAG_WEIGHTED_PRED`
      - 0x00000100
      -
    - - `V4L2_HEVC_PPS_FLAG_WEIGHTED_BIPRED`
      - 0x00000200
      -
    - - `V4L2_HEVC_PPS_FLAG_TRANSQUANT_BYPASS_ENABLED`
      - 0x00000400
      -
    - - `V4L2_HEVC_PPS_FLAG_TILES_ENABLED`
      - 0x00000800
      -
    - - `V4L2_HEVC_PPS_FLAG_ENTROPY_CODING_SYNC_ENABLED`
      - 0x00001000
      -
    - - `V4L2_HEVC_PPS_FLAG_LOOP_FILTER_ACROSS_TILES_ENABLED`
      - 0x00002000
      -
    - - `V4L2_HEVC_PPS_FLAG_PPS_LOOP_FILTER_ACROSS_SLICES_ENABLED`
      - 0x00004000
      -
    - - `V4L2_HEVC_PPS_FLAG_DEBLOCKING_FILTER_OVERRIDE_ENABLED`
      - 0x00008000
      -
    - - `V4L2_HEVC_PPS_FLAG_PPS_DISABLE_DEBLOCKING_FILTER`
      - 0x00010000
      -
    - - `V4L2_HEVC_PPS_FLAG_LISTS_MODIFICATION_PRESENT`
      - 0x00020000
      -
    - - `V4L2_HEVC_PPS_FLAG_SLICE_SEGMENT_HEADER_EXTENSION_PRESENT`
      - 0x00040000
      -
    - - `V4L2_HEVC_PPS_FLAG_DEBLOCKING_FILTER_CONTROL_PRESENT`
      - 0x00080000
      - 鎸囧畾 PPS 涓槸鍚﹀瓨鍦ㄥ幓鍧楁护娉㈠櫒鎺у埗璇硶鍏冪礌銆?
    - - `V4L2_HEVC_PPS_FLAG_UNIFORM_SPACING`
      - 0x00100000
      - 鎸囧畾 tile 鍒楄竟鐣屼互鍙?tile 琛岃竟鐣屽湪鍥惧儚涓婂潎鍖€鍒嗗竷銆?


    \normalsize

`V4L2_CID_STATELESS_HEVC_SLICE_PARAMS (struct)`
    鎸囧畾鍚勭鍒囩墖鐗瑰畾鍙傛暟锛岀壒鍒槸鏉ヨ嚜 NAL 鍗曞厓澶淬€侀€氱敤鍒囩墖娈靛ご浠ュ強鐮佹祦涓姞鏉冮娴嬪弬鏁伴儴鍒?
    鐨勫弬鏁般€傝繖浜涚爜娴佸弬鏁颁緷鎹?hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.7 General slice segment header
    semantics鈥濓紙閫氱敤鍒囩墖娈靛ご璇箟锛変竴鑺備腑鎻忚堪銆傝鎺т欢涓哄姩鎬佸ぇ灏忕殑 1 缁存暟缁勶紝浣跨敤鏃跺繀椤昏缃?
    V4L2_CTRL_FLAG_DYNAMIC_ARRAY 鏍囧織銆?



    \scriptsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `bit_size`
      - 褰撳墠鍒囩墖鏁版嵁鐨勫ぇ灏忥紙姣旂壒锛夈€?
    - - __u32
      - `data_byte_offset`
      - 鎸囧悜褰撳墠鍒囩墖鏁版嵁涓棰戞暟鎹殑鍋忕Щ閲忥紙瀛楄妭锛夈€?
    - - __u32
      - `num_entry_point_offsets`
      - 鎸囧畾鍒囩墖澶翠腑鍏ュ彛鐐瑰亸绉昏娉曞厓绱犵殑鏁伴噺銆傚綋椹卞姩鏀寔鏃讹紝蹇呴』璁剧疆
        `V4L2_CID_STATELESS_HEVC_ENTRY_POINT_OFFSETS`銆?
    - - __u8
      - `nal_unit_type`
      - 鎸囧畾鍒囩墖鐨勭紪鐮佺被鍨嬶紙B銆丳 鎴?I锛夈€?
    - - __u8
      - `nuh_temporal_id_plus1`
      - 鍑?1 鎸囧畾 NAL 鍗曞厓鐨勬椂闂存爣璇嗙銆?
    - - __u8
      - `slice_type`
      -
	锛圴4L2_HEVC_SLICE_TYPE_I銆乂4L2_HEVC_SLICE_TYPE_P 鎴?
	V4L2_HEVC_SLICE_TYPE_B锛夈€?
    - - __u8
      - `colour_plane_id`
      - 鎸囧畾涓庡綋鍓嶅垏鐗囧叧鑱旂殑鑹插钩闈€?
    - - __s32
      - `slice_pic_order_cnt`
      - 鎸囧畾鍥惧儚椤哄簭璁℃暟銆?
    - - __u8
      - `num_ref_idx_l0_active_minus1`
      - 璇ュ€煎姞 1 鎸囧畾鍙敤浜庤В鐮佽鍒囩墖鐨勫弬鑰冨浘鍍忓垪琛?0 鐨勬渶澶у弬鑰冪储寮曘€?
    - - __u8
      - `num_ref_idx_l1_active_minus1`
      - 璇ュ€煎姞 1 鎸囧畾鍙敤浜庤В鐮佽鍒囩墖鐨勫弬鑰冨浘鍍忓垪琛?1 鐨勬渶澶у弬鑰冪储寮曘€?
    - - __u8
      - `collocated_ref_idx`
      - 鎸囧畾鐢ㄤ簬鏃堕棿杩愬姩鐭㈤噺棰勬祴鐨勫崗鍚岋紙collocated锛夊浘鍍忕殑鍙傝€冪储寮曘€?
    - - __u8
      - `five_minus_max_num_merge_cand`
      - 鎸囧畾鍒囩墖鎵€鏀寔鐨勬渶澶у悎骞惰繍鍔ㄧ煝閲忛娴嬪€欓€夋暟锛屼粠 5 涓噺鍘汇€?
    - - __s8
      - `slice_qp_delta`
      - 鎸囧畾鐢ㄤ簬鍒囩墖涓紪鐮佸潡鍒濆鐨?QpY 鍊笺€?
    - - __s8
      - `slice_cb_qp_offset`
      - 鎸囧畾瑕佸姞鍒?pps_cb_qp_offset 鍊间笂鐨勫樊鍊笺€?
    - - __s8
      - `slice_cr_qp_offset`
      - 鎸囧畾瑕佸姞鍒?pps_cr_qp_offset 鍊间笂鐨勫樊鍊笺€?
    - - __s8
      - `slice_act_y_qp_offset`
      - 鎸囧畾绗?8.6.2 鑺傛帹瀵煎嚭鐨勯噺鍖栧弬鏁?qP 鐨勪寒搴﹀亸绉汇€?
    - - __s8
      - `slice_act_cb_qp_offset`
      - 鎸囧畾绗?8.6.2 鑺傛帹瀵煎嚭鐨勯噺鍖栧弬鏁?qP 鐨?cb 鍋忕Щ銆?
    - - __s8
      - `slice_act_cr_qp_offset`
      - 鎸囧畾绗?8.6.2 鑺傛帹瀵煎嚭鐨勯噺鍖栧弬鏁?qP 鐨?cr 鍋忕Щ銆?
    - - __s8
      - `slice_beta_offset_div2`
      - 鎸囧畾 beta 鐨勫幓鍧楀弬鏁板亸绉婚櫎浠?2銆?
    - - __s8
      - `slice_tc_offset_div2`
      - 鎸囧畾 tC 鐨勫幓鍧楀弬鏁板亸绉婚櫎浠?2銆?
    - - __u8
      - `pic_struct`
      - 鎸囩ず鍥惧儚搴斾綔涓哄抚杩樻槸浣滀负涓€涓垨澶氫釜鍦烘樉绀恒€?
    - - __u32
      - `slice_segment_addr`
      - 鎸囧畾鍒囩墖娈典腑绗竴涓紪鐮佹爲鍧楃殑鍦板潃銆?
    - - __u8
      - `ref_idx_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - L0 鍙傝€冨厓绱犲垪琛紝浠?DPB 涓殑绱㈠紩琛ㄧず銆?
    - - __u8
      - `ref_idx_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - L1 鍙傝€冨厓绱犲垪琛紝浠?DPB 涓殑绱㈠紩琛ㄧず銆?
    - - __u16
      - `short_term_ref_pic_set_size`
      - 鎸囧畾鐭湡鍙傝€冨浘鍍忛泦鐨勫ぇ灏忥紙姣旂壒锛夛紝鍦ㄨ鑼冧腑鎻忚堪涓?st_ref_pic_set()锛屽寘鍚湪鍒囩墖澶存垨
        SPS 涓紙绗?7.3.6.1 鑺傦級銆?
    - - __u16
      - `long_term_ref_pic_set_size`
      - 鎸囧畾闀挎湡鍙傝€冨浘鍍忛泦鐨勫ぇ灏忥紙姣旂壒锛夛紝鍖呭惈鍦ㄥ垏鐗囧ご鎴?SPS 涓€傚嵆瑙勮寖绗?7.3.6.1 鑺備腑
        鏉′欢鍧?if(long_term_ref_pics_present_flag) 鍐呯殑姣旂壒鏁般€?
    - - __u8
      - `padding`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - struct `v4l2_hevc_pred_weight_table`
      - `pred_weight_table`
      - 鐢ㄤ簬甯ч棿鍥惧儚棰勬祴鐨勯娴嬪姞鏉冪郴鏁般€?
    - - __u64
      - `flags`
      - 鍙傝鍒囩墖鍙傛暟鏍囧織 <hevc_slice_params_flags>


    \normalsize


`鍒囩墖鍙傛暟鏍囧織`


    \scriptsize

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_SLICE_SAO_LUMA`
      - 0x00000001
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_SLICE_SAO_CHROMA`
      - 0x00000002
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_SLICE_TEMPORAL_MVP_ENABLED`
      - 0x00000004
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_MVD_L1_ZERO`
      - 0x00000008
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_CABAC_INIT`
      - 0x00000010
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_COLLOCATED_FROM_L0`
      - 0x00000020
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_USE_INTEGER_MV`
      - 0x00000040
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_SLICE_DEBLOCKING_FILTER_DISABLED`
      - 0x00000080
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_SLICE_LOOP_FILTER_ACROSS_SLICES_ENABLED`
      - 0x00000100
      -
    - - `V4L2_HEVC_SLICE_PARAMS_FLAG_DEPENDENT_SLICE_SEGMENT`
      - 0x00000200
      -


    \normalsize

`V4L2_CID_STATELESS_HEVC_ENTRY_POINT_OFFSETS (integer)`
    鎸囧畾鍏ュ彛鐐瑰亸绉伙紙瀛楄妭锛夈€傝鎺т欢涓哄姩鎬佸ぇ灏忔暟缁勶紝鍏ュ彛鐐瑰亸绉荤殑鏁伴噺鐢?`elems` 瀛楁鎶ュ憡銆?
    璇ョ爜娴佸弬鏁颁緷鎹?hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.7.1 General slice segment header
    semantics鈥濓紙閫氱敤鍒囩墖娈靛ご璇箟锛変竴鑺備腑鎻忚堪銆傚綋涓€涓姹備腑鎻愪氦澶氫釜鍒囩墖鏃讹紝璇ユ暟缁勭殑闀垮害
    蹇呴』涓鸿姹備腑鎵€鏈夊垏鐗囩殑 num_entry_point_offsets 涔嬪拰銆?

`V4L2_CID_STATELESS_HEVC_SCALING_MATRIX (struct)`
    鎸囧畾鐢ㄤ簬鍙樻崲绯绘暟缂╂斁杩囩▼鐨?HEVC 缂╂斁鐭╅樀鍙傛暟銆傝繖浜涚煩闃典笌鍙傛暟渚濇嵁 hevc 鏍囧噯瀹氫箟锛屽苟鍦?
    瑙勮寖鐨?鈥?.4.5 Scaling list data semantics鈥濓紙缂╂斁鍒楄〃鏁版嵁璇箟锛変竴鑺備腑鎻忚堪銆?



    \scriptsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `scaling_list_4x4[^6^][^16^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_8x8[^6^][^64^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_16x16[^6^][^64^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_32x32[^2^][^64^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_dc_coef_16x16[^6^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?
    - - __u8
      - `scaling_list_dc_coef_32x32[^2^]`
      - 缂╂斁鍒楄〃鐢ㄤ簬鍙樻崲绯绘暟鐨勭缉鏀捐繃绋嬨€傛瘡涓缉鏀惧垪琛ㄤ腑鐨勫€兼寜鍏夋爡鎵弿椤哄簭鎺掑垪銆?


    \normalsize



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `timestamp`
      - 鐢ㄤ綔鍙傝€冪殑 V4L2 鎹曡幏缂撳啿鍖虹殑鏃堕棿鎴筹紝涓?B 甯у拰 P 甯ч厤鍚堜娇鐢ㄣ€傝鏃堕棿鎴冲紩鐢?
        struct `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()`
        鍑芥暟灏?struct `v4l2_buffer` 涓殑 struct `timeval` 杞崲涓?__u64銆?
    - - __u8
      - `flags`
      - 鍙傝€冨抚鐨勯暱鏈熸爣蹇楋紙V4L2_HEVC_DPB_ENTRY_LONG_TERM_REFERENCE锛夈€傝鏍囧織鐨勮缃 ITU HEVC
        瑙勮寖 鈥?.3.2 Decoding process for reference picture set鈥濓紙鍙傝€冨浘鍍忛泦瑙ｇ爜杩囩▼锛変竴绔犳墍杩般€?
    - - __u8
      - `field_pic`
      - 璇ュ弬鑰冩槸鍦哄浘鍍忚繕鏄抚鍥惧儚銆傚弬瑙?HEVC dpb 鍦哄浘鍍忔爣蹇?<hevc_dpb_field_pic_flags>銆?
    - - __s32
      - `pic_order_cnt_val`
      - 褰撳墠鍥惧儚鐨勫浘鍍忛『搴忚鏁般€?
    - - __u8
      - `padding[^2^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


    \normalsize


`HEVC dpb 鍦哄浘鍍忔爣蹇梎


    \scriptsize

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME`
      - 0
      - 锛堥€愯锛夊抚
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_FIELD`
      - 1
      - 椤跺満
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_FIELD`
      - 2
      - 搴曞満
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM`
      - 3
      - 椤跺満銆佸簳鍦猴紝鎸夋椤哄簭
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP`
      - 4
      - 搴曞満銆侀《鍦猴紝鎸夋椤哄簭
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM_TOP`
      - 5
      - 椤跺満銆佸簳鍦恒€侀《鍦洪噸澶嶏紝鎸夋椤哄簭
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM`
      - 6
      - 搴曞満銆侀《鍦恒€佸簳鍦洪噸澶嶏紝鎸夋椤哄簭
    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME_DOUBLING`
      - 7
      - 甯х炕鍊嶏紙Frame doubling锛?
    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME_TRIPLING`
      - 8
      - 甯т笁鍊嶏紙Frame tripling锛?
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_PREVIOUS_BOTTOM`
      - 9
      - 椤跺満涓庝笂涓€涓簳鍦哄湪杈撳嚭椤哄簭涓婇厤瀵?
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_PREVIOUS_TOP`
      - 10
      - 搴曞満涓庝笂涓€涓《鍦哄湪杈撳嚭椤哄簭涓婇厤瀵?
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_NEXT_BOTTOM`
      - 11
      - 椤跺満涓庝笅涓€涓簳鍦哄湪杈撳嚭椤哄簭涓婇厤瀵?
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_NEXT_TOP`
      - 12
      - 搴曞満涓庝笅涓€涓《鍦哄湪杈撳嚭椤哄簭涓婇厤瀵?


    \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `delta_luma_weight_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 搴旂敤浜庡垪琛?0 浜害棰勬祴鍊肩殑鍔犳潈鍥犲瓙鐨勫樊鍊笺€?
    - - __s8
      - `luma_offset_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 搴旂敤浜庡垪琛?0 浜害棰勬祴鍊肩殑鍔犳€у亸绉汇€?
    - - __s8
      - `delta_chroma_weight_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 搴旂敤浜庡垪琛?0 鑹插害棰勬祴鍊肩殑鍔犳潈鍥犲瓙鐨勫樊鍊笺€?
    - - __s8
      - `chroma_offset_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 搴旂敤浜庡垪琛?0 鑹插害棰勬祴鍊肩殑鍔犳€у亸绉荤殑宸€笺€?
    - - __s8
      - `delta_luma_weight_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 搴旂敤浜庡垪琛?1 浜害棰勬祴鍊肩殑鍔犳潈鍥犲瓙鐨勫樊鍊笺€?
    - - __s8
      - `luma_offset_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 搴旂敤浜庡垪琛?1 浜害棰勬祴鍊肩殑鍔犳€у亸绉汇€?
    - - __s8
      - `delta_chroma_weight_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 搴旂敤浜庡垪琛?1 鑹插害棰勬祴鍊肩殑鍔犳潈鍥犲瓙鐨勫樊鍊笺€?
    - - __s8
      - `chroma_offset_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 搴旂敤浜庡垪琛?1 鑹插害棰勬祴鍊肩殑鍔犳€у亸绉荤殑宸€笺€?
    - - __u8
      - `luma_log2_weight_denom`
      - 鎵€鏈変寒搴﹀姞鏉冨洜瀛愬垎姣嶇殑浠?2 涓哄簳鐨勫鏁般€?
    - - __s8
      - `delta_chroma_log2_weight_denom`
      - 鎵€鏈夎壊搴﹀姞鏉冨洜瀛愬垎姣嶇殑浠?2 涓哄簳鐨勫鏁扮殑宸€笺€?
    - - __u8
      - `padding[^6^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


    \normalsize

`V4L2_CID_STATELESS_HEVC_DECODE_MODE (enum)`
    鎸囧畾瑕佷娇鐢ㄧ殑瑙ｇ爜妯″紡銆傜洰鍓嶆彁渚涘熀浜庡垏鐗囧拰鍩轰簬甯х殑瑙ｇ爜锛屼絾鍚庣画鍙兘浼氭柊澧炲叾浠栨ā寮忋€?
    璇ユ帶浠剁敤浣?V4L2_PIX_FMT_HEVC_SLICE 鍍忕礌鏍煎紡鐨勪慨楗扮銆傛敮鎸?V4L2_PIX_FMT_HEVC_SLICE
    鐨勫簲鐢ㄧ▼搴忓繀椤昏缃鎺т欢锛屼互鎸囧畾缂撳啿鍖烘墍鏈熸湜鐨勮В鐮佹ā寮忋€傞┍鍔ㄥ彲鑳芥牴鎹叾鎵€鏀寔鐨勮兘鍔涳紝
    鏆撮湶鍗曚釜鎴栧涓В鐮佹ā寮忋€?



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_HEVC_DECODE_MODE_SLICE_BASED`
      - 0
      - 浠ュ垏鐗囩矑搴﹁繘琛岃В鐮併€侽UTPUT 缂撳啿鍖哄繀椤诲寘鍚崟涓垏鐗囥€?
    - - `V4L2_STATELESS_HEVC_DECODE_MODE_FRAME_BASED`
      - 1
      - 浠ュ抚绮掑害杩涜瑙ｇ爜銆侽UTPUT 缂撳啿鍖哄繀椤诲寘鍚В鐮佽甯ф墍闇€鐨勫叏閮ㄥ垏鐗囥€?


    \normalsize

`V4L2_CID_STATELESS_HEVC_START_CODE (enum)`
    鎸囧畾姣忎釜 HEVC 鍒囩墖鎵€鏈熸湜鐨勫垏鐗囪捣濮嬬爜銆傝鎺т欢鐢ㄤ綔 V4L2_PIX_FMT_HEVC_SLICE 鍍忕礌鏍煎紡鐨?
    淇グ绗︺€傛敮鎸?V4L2_PIX_FMT_HEVC_SLICE 鐨勫簲鐢ㄧ▼搴忓繀椤昏缃鎺т欢锛屼互鎸囧畾缂撳啿鍖烘墍鏈熸湜鐨?
    璧峰鐮併€傞┍鍔ㄥ彲鑳芥牴鎹叾鎵€鏀寔鐨勮兘鍔涳紝鏆撮湶鍗曚釜鎴栧涓捣濮嬬爜銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_HEVC_START_CODE_NONE`
      - 0
      - 閫夋嫨璇ュ€艰〃绀?HEVC 鍒囩墖涓嶅甫浠讳綍璧峰鐮佸湴浼犻€掔粰椹卞姩銆傜爜娴佹暟鎹簲閬靛惊 hevc 7.3.1.1
        General NAL unit syntax锛堥€氱敤 NAL 鍗曞厓璇硶锛夛紝鍥犳鍦ㄩ渶瑕佹椂浼氬寘鍚豢鐪熼闃插瓧鑺傘€?
    - - `V4L2_STATELESS_HEVC_START_CODE_ANNEX_B`
      - 1
      - 閫夋嫨璇ュ€艰〃绀烘湡鏈?HEVC 鍒囩墖浠?Annex B 璧峰鐮佷綔涓哄墠缂€銆備緷鎹?hevc锛屾湁鏁堢殑璧峰鐮佸彲浠ユ槸
        3 瀛楄妭鐨?0x000001 鎴?4 瀛楄妭鐨?0x00000001銆?


    \normalsize

`V4L2_CID_MPEG_VIDEO_BASELAYER_PRIORITY_ID (integer)`
    涓?NAL 鍗曞厓鎸囧畾涓€涓紭鍏堢骇鏍囪瘑绗︼紝灏嗗簲鐢ㄤ簬鍩虹灞傦紙base layer锛夈€傞粯璁ゆ儏鍐典笅锛屽熀纭€灞傝鍊?
    璁句负 0锛屼笅涓€灞傚皢琚垎閰嶄紭鍏堢骇 ID 涓?1銆?銆? 绛夌瓑銆傝棰戠紪鐮佸櫒鏃犳硶鍐冲畾瑕佸簲鐢ㄤ簬鏌愬眰鐨?
    浼樺厛绾?ID锛屽洜姝ゅ繀椤荤敱瀹㈡埛绔彁渚涖€傝繖閫傜敤浜?H264锛屾湁鏁堣寖鍥翠负 0 鍒?63銆?
    鏉ユ簮锛歊ec. ITU-T H.264 (06/2019)锛汫.7.4.1.1銆丟.8.8.1銆?

`V4L2_CID_MPEG_VIDEO_LTR_COUNT (integer)`
    鎸囧畾缂栫爜鍣ㄥ湪浠讳綍缁欏畾鏃跺埢鍙互淇濈暀鐨勯暱鏈熷弬鑰冿紙LTR锛夊抚鐨勬渶澶ф暟閲忋€傝繖閫傜敤浜?H264 鍜?HEVC
    缂栫爜鍣ㄣ€?

`V4L2_CID_MPEG_VIDEO_FRAME_LTR_INDEX (integer)`
    璁剧疆璇ユ帶浠跺悗锛屾帴涓嬫潵灏嗘帓闃熺殑甯у皢琚爣璁颁负闀挎湡鍙傝€冿紙LTR锛夊抚锛屽苟鑾峰緱璇?LTR 绱㈠紩锛岀储寮曡寖鍥?
    浠?0 鍒?LTR_COUNT-1銆傝繖閫傜敤浜?H264 鍜?HEVC 缂栫爜鍣ㄣ€傛潵婧愶細Rec. ITU-T H.264 (06/2019)锛?
    琛?7.9銆?

`V4L2_CID_MPEG_VIDEO_USE_LTR_FRAMES (bitmask)`
    鎸囧畾鐢ㄤ簬缂栫爜璁剧疆璇ユ帶浠跺悗涓嬩竴涓帓闃熷抚鐨勯暱鏈熷弬鑰冿紙LTR锛夊抚銆傝繖鎻愪緵涓€涓綅鎺╃爜锛岀敱姣旂壒
    [0, LTR_COUNT-1] 缁勬垚銆傝繖閫傜敤浜?H264 鍜?HEVC 缂栫爜鍣ㄣ€?

`V4L2_CID_STATELESS_HEVC_DECODE_PARAMS (struct)`
    鎸囧畾鍚勭瑙ｇ爜鍙傛暟锛岀壒鍒槸鎵€鏈夊垪琛紙鐭湡銆侀暱鏈熴€佷箣鍓嶃€佸綋鍓嶃€佷箣鍚庯級鐨勫弬鑰冨浘鍍忛『搴忚鏁?
    锛圥OC锛変互鍙婃瘡涓垪琛ㄧ殑鏉＄洰鏁般€傝繖浜涘弬鏁颁緷鎹?hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.3 Slice decoding
    process鈥濓紙鍒囩墖瑙ｇ爜杩囩▼锛変竴鑺備腑鎻忚堪銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s32
      - `pic_order_cnt_val`
      - PicOrderCntVal锛屽瑙勮寖 鈥?.3.1 Decoding process for picture order count鈥?
        锛堝浘鍍忛『搴忚鏁拌В鐮佽繃绋嬶級涓€鑺傛墍杩般€?
    - - __u16
      - `short_term_ref_pic_set_size`
      - 鎸囧畾绗竴涓垏鐗囩殑鐭湡鍙傝€冨浘鍍忛泦鐨勫ぇ灏忥紙姣旂壒锛夛紝璇ラ泦鍚堝湪瑙勮寖涓弿杩颁负
        st_ref_pic_set()锛屽寘鍚湪鍒囩墖澶存垨 SPS 涓紙绗?7.3.6.1 鑺傦級銆?
    - - __u16
      - `long_term_ref_pic_set_size`
      - 鎸囧畾绗竴涓垏鐗囦腑鍖呭惈鐨勯暱鏈熷弬鑰冨浘鍍忛泦鐨勫ぇ灏忥紙姣旂壒锛夛紝鍖呭惈鍦ㄥ垏鐗囧ご鎴?SPS 涓€?
        鍗宠鑼冪 7.3.6.1 鑺備腑鏉′欢鍧?if(long_term_ref_pics_present_flag) 鍐呯殑姣旂壒鏁般€?
    - - __u8
      - `num_active_dpb_entries`
      - `dpb` 涓殑鏉＄洰鏁般€?
    - - __u8
      - `num_poc_st_curr_before`
      - 鍦ㄥ綋鍓嶅抚涔嬪墠鐨勭煭鏈熼泦鍚堜腑鐨勫弬鑰冨浘鍍忔暟閲忋€?
    - - __u8
      - `num_poc_st_curr_after`
      - 鍦ㄥ綋鍓嶅抚涔嬪悗鐨勭煭鏈熼泦鍚堜腑鐨勫弬鑰冨浘鍍忔暟閲忋€?
    - - __u8
      - `num_poc_lt_curr`
      - 闀挎湡闆嗗悎涓殑鍙傝€冨浘鍍忔暟閲忋€?
    - - __u8
      - `poc_st_curr_before[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocStCurrBefore锛屽瑙勮寖 鈥?.3.2 Decoding process for reference picture set鈥?
        锛堝弬鑰冨浘鍍忛泦瑙ｇ爜杩囩▼锛変竴鑺傛墍杩帮細鎻愪緵 DPB 鏁扮粍涓綋鍓嶅抚涔嬪墠鐨勭煭鏈熷弬鑰冪殑绱㈠紩銆?
    - - __u8
      - `poc_st_curr_after[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocStCurrAfter锛屽瑙勮寖 鈥?.3.2 Decoding process for reference picture set鈥?
        涓€鑺傛墍杩帮細鎻愪緵 DPB 鏁扮粍涓綋鍓嶅抚涔嬪悗鐨勭煭鏈熷弬鑰冪殑绱㈠紩銆?
    - - __u8
      - `poc_lt_curr[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocLtCurr锛屽瑙勮寖 鈥?.3.2 Decoding process for reference picture set鈥?
        涓€鑺傛墍杩帮細鎻愪緵 DPB 鏁扮粍涓暱鏈熷弬鑰冪殑绱㈠紩銆?
    - - __u8
      - `num_delta_pocs_of_ref_rps_idx`
      - 褰撳垏鐗囧ご涓?short_term_ref_pic_set_sps_flag 绛変簬 0 鏃讹紝鍏跺€间笌鎺ㄥ鍊?
        NumDeltaPocs[RefRpsIdx] 鐩稿悓銆傚畠鍙敤浜庤В鏋愬垏鐗囧ご涓殑 RPS 鏁版嵁锛岃€岄潪浣跨敤
        @short_term_ref_pic_set_size 璺宠繃瀹冦€傚綋鍒囩墖澶翠腑 short_term_ref_pic_set_sps_flag
        鐨勫€肩瓑浜?1 鏃讹紝num_delta_pocs_of_ref_rps_idx 搴旇涓?0銆?
    - - struct `v4l2_hevc_dpb_entry`
      - `dpb[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 瑙ｇ爜鍥惧儚缂撳啿鍖猴紝瀛樻斁鍏充簬鍙傝€冨抚鐨勫厓鏁版嵁銆?
    - - __u64
      - `flags`
      - 鍙傝瑙ｇ爜鍙傛暟鏍囧織 <hevc_decode_params_flags>


`瑙ｇ爜鍙傛暟鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_DECODE_PARAM_FLAG_IRAP_PIC`
      - 0x00000001
      -
    - - `V4L2_HEVC_DECODE_PARAM_FLAG_IDR_PIC`
      - 0x00000002
      -
    - - `V4L2_HEVC_DECODE_PARAM_FLAG_NO_OUTPUT_OF_PRIOR`
      - 0x00000004
      -


`V4L2_CID_STATELESS_HEVC_EXT_SPS_LT_RPS (struct)`
    `v4l2_ctrl_hevc_sps` 鎺т欢鐨勫瓙闆嗐€傚畠浠ラ暱鏈熷弬鑰冮泦鍙傛暟鍒楄〃瀵瑰叾杩涜鎵╁睍銆傝繖浜涘弬鏁颁緷鎹?
    hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.3.2.1 General sequence parameter set RBSP semantics鈥?
    锛堥€氱敤搴忓垪鍙傛暟闆?RBSP 璇箟锛変竴鑺備腑鎻忚堪銆傝鎺т欢涓哄姩鎬佸ぇ灏忕殑 1 缁存暟缁勩€傚綋
    num_long_term_ref_pics_sps 涓?0锛屾垨 `v4l2_ctrl_hevc_sps` 涓湭璁剧疆
    V4L2_HEVC_SPS_FLAG_LONG_TERM_REF_PICS_PRESENT 鏍囧織鏃讹紝鏁扮粍涓殑鍊煎簲琚拷鐣ャ€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `lt_ref_pic_poc_lsb_sps`
      - 闀挎湡鍙傝€冨浘鍍忛『搴忚鏁帮紝濡傝鑼?鈥?.4.3.2.1 General sequence parameter set RBSP semantics鈥?
        锛堥€氱敤搴忓垪鍙傛暟闆?RBSP 璇箟锛変竴鑺傛墍杩般€?
    - - __u16
      - `flags`
      - 鍙傝鎵╁睍闀挎湡 RPS 鏍囧織 <hevc_ext_sps_lt_rps_flags>


`鎵╁睍 SPS 闀挎湡 RPS 鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_EXT_SPS_LT_RPS_FLAG_USED_LT`
      - 0x00000001
      - 鎸囧畾闀挎湡鍙傝€冨浘鍍忔槸鍚﹁浣跨敤锛岃瑙勮寖 7.4.3.2.1 鈥淕eneral sequence parameter set RBSP
        semantics鈥濓紙閫氱敤搴忓垪鍙傛暟闆?RBSP 璇箟锛変竴鑺傘€?


`V4L2_CID_STATELESS_HEVC_EXT_SPS_ST_RPS (struct)`
    `v4l2_ctrl_hevc_sps` 鎺т欢鐨勫瓙闆嗐€傚畠浠ョ煭鏈熷弬鑰冮泦鍙傛暟鍒楄〃瀵瑰叾杩涜鎵╁睍銆傝繖浜涘弬鏁颁緷鎹?
    hevc 鏍囧噯瀹氫箟锛屽苟鍦ㄨ鑼冪殑 鈥?.4.8 Short-term reference picture set semantics鈥?
    锛堢煭鏈熷弬鑰冨浘鍍忛泦璇箟锛変竴鑺備腑鎻忚堪銆傝鎺т欢涓哄姩鎬佸ぇ灏忕殑 1 缁存暟缁勩€傚綋 num_short_term_ref_pic_sets
    涓?0 鏃讹紝鏁扮粍涓殑鍊煎簲琚拷鐣ャ€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `delta_idx_minus1`
      - 鎸囧畾涓庣储寮曟瘮杈冪殑 delta 鍊笺€傝瑙佽鑼?鈥?.4.8 Short-term reference picture set semantics鈥?
        锛堢煭鏈熷弬鑰冨浘鍍忛泦璇箟锛変竴鑺傘€?
    - - __u8
      - `delta_rps_sign`
      - delta 鐨勭鍙凤紝濡傝鑼?鈥?.4.8 Short-term reference picture set semantics鈥?涓€鑺傛墍杩般€?
    - - __u8
      - `num_negative_pics`
      - 鍥惧儚椤哄簭璁℃暟鍊煎皬浜庡綋鍓嶅浘鍍忛『搴忚鏁板€肩殑鐭湡 RPS 鏉＄洰鏁伴噺銆?
    - - __u8
      - `num_positive_pics`
      - 鍥惧儚椤哄簭璁℃暟鍊煎ぇ浜庡綋鍓嶅浘鍍忛『搴忚鏁板€肩殑鐭湡 RPS 鏉＄洰鏁伴噺銆?
    - - __u32
      - `used_by_curr_pic`
      - 绗?i 浣嶆寚瀹氱煭鏈?RPS i 鏄惁琚綋鍓嶅浘鍍忎娇鐢ㄣ€?
    - - __u32
      - `use_delta_flag`
      - 绗?i 浣嶆寚瀹氱煭鏈?RPS i 鏄惁琚寘鍚湪鐭湡 RPS 鏉＄洰涓€?
    - - __u16
      - `abs_delta_rps_minus1`
      - 缁濆 delta RPS锛屽瑙勮寖 鈥?.4.8 Short-term reference picture set semantics鈥?涓€鑺傛墍杩般€?
    - - __u16
      - `delta_poc_s0_minus1[^16^]`
      - 鎸囧畾鐭湡 RPS 涓 i 涓潯鐩殑璐熷浘鍍忛『搴忚鏁?delta銆傝瑙佽鑼?鈥?.4.8 Short-term
        reference picture set semantics鈥?涓€鑺傘€?
    - - __u16
      - `delta_poc_s1_minus1[^16^]`
      - 鎸囧畾鐭湡 RPS 涓 i 涓潯鐩殑姝ｅ浘鍍忛『搴忚鏁?delta銆傝瑙佽鑼?鈥?.4.8 Short-term
        reference picture set semantics鈥?涓€鑺傘€?
    - - __u16
      - `flags`
      - 鍙傝鎵╁睍鐭湡 RPS 鏍囧織 <hevc_ext_sps_st_rps_flags>


`鎵╁睍 SPS 鐭湡 RPS 鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_EXT_SPS_ST_RPS_FLAG_INTER_REF_PIC_SET_PRED`
      - 0x00000001
      - 鎸囧畾鐭湡 RPS 鏄惁浠庡彟涓€涓煭鏈?RPS 棰勬祴寰楀埌銆傝瑙佽鑼?鈥?.4.8 Short-term reference
        picture set semantics鈥?涓€鑺傘€?


`V4L2_CID_STATELESS_AV1_SEQUENCE (struct)`
    琛ㄧず涓€涓?AV1 搴忓垪 OBU锛圤pen Bitstream Unit锛屽紑鏀剧爜娴佸崟鍏冿級銆傛洿澶氱粏鑺傚弬瑙?av1 瑙勮寖
    绗?5.5 鑺?鈥淪equence header OBU syntax鈥濓紙搴忓垪澶?OBU 璇硶锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `flags`
      - 鍙傝 AV1 搴忓垪鏍囧織 <av1_sequence_flags>銆?
    - - __u8
      - `seq_profile`
      - 鎸囧畾缂栫爜瑙嗛搴忓垪涓彲浣跨敤鐨勭壒鎬с€?
    - - __u8
      - `order_hint_bits`
      - 鎸囧畾姣忓抚涓?order_hint 瀛楁鎵€鐢ㄧ殑姣旂壒鏁般€?
    - - __u8
      - `bit_depth`
      - 鐢ㄤ簬璇ュ簭鍒楃殑浣嶆繁锛屾洿澶氱粏鑺傝 av1 瑙勮寖绗?5.5.2 鑺?鈥淐olor config syntax鈥?
        锛堥鑹查厤缃娉曪級銆?
    - - __u8
      - `reserved`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u16
      - `max_frame_width_minus_1`
      - 鎸囧畾鐢辫搴忓垪澶存墍琛ㄧず鐨勬渶澶у抚瀹藉害鍑?1銆?
    - - __u16
      - `max_frame_height_minus_1`
      - 鎸囧畾鐢辫搴忓垪澶存墍琛ㄧず鐨勬渶澶у抚楂樺害鍑?1銆?


`AV1 搴忓垪鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEQUENCE_FLAG_STILL_PICTURE`
      - 0x00000001
      - 鑻ヨ缃紝鎸囧畾缂栫爜瑙嗛搴忓垪浠呭寘鍚竴涓紪鐮佸抚銆傝嫢鏈缃紝鎸囧畾缂栫爜瑙嗛搴忓垪鍖呭惈涓€涓垨澶氫釜
        缂栫爜甯с€?
    - - `V4L2_AV1_SEQUENCE_FLAG_USE_128X128_SUPERBLOCK`
      - 0x00000002
      - 鑻ヨ缃紝琛ㄧず superblock 鍖呭惈 128x128 浜害鏍锋湰銆備负 0 鏃讹紝琛ㄧず superblock 鍖呭惈
        64x64 浜害鏍锋湰銆傛墍鍖呭惈鐨勮壊搴︽牱鏈暟閲忓彇鍐充簬 subsampling_x 鍜?subsampling_y銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_FILTER_INTRA`
      - 0x00000004
      - 鑻ヨ缃紝鎸囧畾 use_filter_intra 璇硶鍏冪礌鍙兘鍛堢幇銆傝嫢鏈缃紝鎸囧畾 use_filter_intra
        璇硶鍏冪礌涓嶄細鍑虹幇銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTRA_EDGE_FILTER`
      - 0x00000008
      - 鎸囧畾鏄惁鍚敤甯у唴杈圭紭婊ゆ尝杩囩▼銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTERINTRA_COMPOUND`
      - 0x00000010
      - 鑻ヨ缃紝鎸囧畾甯ч棿鍧楃殑 mode info 鍙寘鍚?interintra 璇硶鍏冪礌銆傝嫢鏈缃紝鎸囧畾 interintra
        璇硶鍏冪礌涓嶄細鍑虹幇銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_MASKED_COMPOUND`
      - 0x00000020
      - 鑻ヨ缃紝鎸囧畾甯ч棿鍧楃殑 mode info 鍙寘鍚?compound_type 璇硶鍏冪礌銆傝嫢鏈缃紝鎸囧畾
        compound_type 璇硶鍏冪礌涓嶄細鍑虹幇銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_WARPED_MOTION`
      - 0x00000040
      - 鑻ヨ缃紝琛ㄧず allow_warped_motion 璇硶鍏冪礌鍙兘鍑虹幇銆傝嫢鏈缃紝琛ㄧず allow_warped_motion
        璇硶鍏冪礌涓嶄細鍑虹幇銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_DUAL_FILTER`
      - 0x00000080
      - 鑻ヨ缃紝琛ㄧず甯ч棿棰勬祴婊ゆ尝鍣ㄧ被鍨嬪彲鍦ㄦ按骞冲拰鍨傜洿鏂瑰悜涓婄嫭绔嬫寚瀹氥€傝嫢鏍囧織涓?0锛屽垯鍙兘鎸囧畾
        涓€绉嶆护娉㈠櫒绫诲瀷锛屽苟鍦ㄤ袱涓柟鍚戝潎浣跨敤銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_ORDER_HINT`
      - 0x00000100
      - 鑻ヨ缃紝琛ㄧず鍙熀浜?order hint 鍊间娇鐢ㄧ浉搴斿伐鍏枫€傝嫢鏈缃紝琛ㄧず鍩轰簬 order hint 鐨勫伐鍏?
        琚鐢ㄣ€?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_JNT_COMP`
      - 0x00000200
      - 鑻ヨ缃紝琛ㄧず璺濈鍔犳潈杩囩▼鍙敤浜庡抚闂撮娴嬨€?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_REF_FRAME_MVS`
      - 0x00000400
      - 鑻ヨ缃紝琛ㄧず use_ref_frame_mvs 璇硶鍏冪礌鍙兘鍑虹幇銆傝嫢鏈缃紝琛ㄧず use_ref_frame_mvs
        璇硶鍏冪礌涓嶄細鍑虹幇銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_SUPERRES`
      - 0x00000800
      - 鑻ヨ缃紝鎸囧畾鏈帇缂╁ご涓皢鍑虹幇 use_superres 璇硶鍏冪礌銆傝嫢鏈缃紝鎸囧畾 use_superres
        璇硶鍏冪礌涓嶄細鍑虹幇锛堣€屾槸鍦ㄦ湭鍘嬬缉澶翠腑鐩存帴灏?use_superres 璁句负 0锛屾棤闇€璇诲彇锛夈€?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_CDEF`
      - 0x00001000
      - 鑻ヨ缃紝鎸囧畾鍙惎鐢?cdef 婊ゆ尝銆傝嫢鏈缃紝鎸囧畾 cdef 婊ゆ尝琚鐢ㄣ€?
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_RESTORATION`
      - 0x00002000
      - 鑻ヨ缃紝鎸囧畾鍙惎鐢ㄧ幆璺仮澶嶆护娉€傝嫢鏈缃紝鎸囧畾鐜矾鎭㈠婊ゆ尝琚鐢ㄣ€?
    - - `V4L2_AV1_SEQUENCE_FLAG_MONO_CHROME`
      - 0x00004000
      - 鑻ヨ缃紝琛ㄧず瑙嗛涓嶅寘鍚?U 鍜?V 棰滆壊骞抽潰銆傝嫢鏈缃紝琛ㄧず瑙嗛鍖呭惈 Y銆乁 鍜?V 棰滆壊骞抽潰銆?
    - - `V4L2_AV1_SEQUENCE_FLAG_COLOR_RANGE`
      - 0x00008000
      - 鑻ヨ缃紝鍙戜俊鍙疯〃绀哄叏鎽嗗姩锛坒ull swing锛夎〃绀猴紝鍗斥€滃叏鑼冨洿閲忓寲锛團ull Range Quantization锛夆€濄€?
        鑻ユ湭璁剧疆锛屽彂淇″彿琛ㄧず婕旀挱瀹ゆ憜鍔紙studio swing锛夎〃绀猴紝鍗斥€滈檺鍒惰寖鍥撮噺鍖栵紙Limited Range
        Quantization锛夆€濄€?
    - - `V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_X`
      - 0x00010000
      - 鎸囧畾鑹插害瀛愰噰鏍锋牸寮忋€?
    - - `V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_Y`
      - 0x00020000
      - 鎸囧畾鑹插害瀛愰噰鏍锋牸寮忋€?
    - - `V4L2_AV1_SEQUENCE_FLAG_FILM_GRAIN_PARAMS_PRESENT`
      - 0x00040000
      - 鎸囧畾缂栫爜瑙嗛搴忓垪涓槸鍚﹀瓨鍦ㄨ兌鐗囬绮掞紙film grain锛夊弬鏁般€?
    - - `V4L2_AV1_SEQUENCE_FLAG_SEPARATE_UV_DELTA_Q`
      - 0x00080000
      - 鑻ヨ缃紝琛ㄧず U 鍜?V 骞抽潰鍙叿鏈夌嫭绔嬬殑 delta 閲忓寲鍣ㄥ€笺€傝嫢鏈缃紝琛ㄧず U 鍜?V 骞抽潰灏?
        鍏变韩鐩稿悓鐨?delta 閲忓寲鍣ㄥ€笺€?


`V4L2_CID_STATELESS_AV1_TILE_GROUP_ENTRY (struct)`
    琛ㄧず AV1 Tile Group 鍐呯殑鍗曚釜 AV1 tile銆傛敞鎰?MiRowStart銆丮iRowEnd銆丮iColStart 鍜?MiColEnd
    鍙€氳繃浣跨敤 tile_row 鍜?tile_col锛屼粠 struct v4l2_ctrl_av1_frame 涓殑 struct
    v4l2_av1_tile_info 鑾峰彇銆傛洿澶氱粏鑺傚弬瑙?av1 瑙勮寖绗?6.10.1 鑺?鈥淕eneral tile group OBU
    semantics鈥濓紙閫氱敤 tile group OBU 璇箟锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tile_offset`
      - 璺?OBU 鏁版嵁鐨勫亸绉伙紝鍗崇紪鐮?tile 鏁版嵁瀹為檯寮€濮嬬殑浣嶇疆銆?
    - - __u32
      - `tile_size`
      - 鎸囧畾缂栫爜 tile 鐨勫ぇ灏忥紙瀛楄妭锛夈€傜瓑浠蜂簬 av1 涓殑 鈥淭ileSize鈥濄€?
    - - __u32
      - `tile_row`
      - 鎸囧畾褰撳墠 tile 鐨勮銆傜瓑浠蜂簬 av1 涓殑 鈥淭ileRow鈥濄€?
    - - __u32
      - `tile_col`
      - 鎸囧畾褰撳墠 tile 鐨勫垪銆傜瓑浠蜂簬 av1 涓殑 鈥淭ileColumn鈥濄€?


	AV1 鎵洸妯″瀷锛圵arp Model锛夛紝濡?av1 瑙勮寖绗?3 鑺?鈥淪ymbols and abbreviated terms鈥?
	锛堢鍙蜂笌缂╁啓鏈锛夋墍杩般€?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_WARP_MODEL_IDENTITY`
      - 0
      - 鎵洸妯″瀷浠呬负鎭掔瓑鍙樻崲銆?
    - - `V4L2_AV1_WARP_MODEL_TRANSLATION`
      - 1
      - 鎵洸妯″瀷涓虹函骞崇Щ銆?
    - - `V4L2_AV1_WARP_MODEL_ROTZOOM`
      - 2
      - 鎵洸妯″瀷涓烘棆杞?+ 瀵圭О缂╂斁 + 骞崇Щ銆?
    - - `V4L2_AV1_WARP_MODEL_AFFINE`
      - 3
      - 鎵洸妯″瀷涓洪€氱敤浠垮皠鍙樻崲銆?


AV1 鍙傝€冨抚锛屽 av1 瑙勮寖绗?6.10.24 鑺?鈥淩ef frames semantics鈥濓紙鍙傝€冨抚璇箟锛夋墍杩般€?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_REF_INTRA_FRAME`
      - 0
      - 甯у唴鍙傝€冨抚銆?
    - - `V4L2_AV1_REF_LAST_FRAME`
      - 1
      - Last 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_LAST2_FRAME`
      - 2
      - Last2 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_LAST3_FRAME`
      - 3
      - Last3 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_GOLDEN_FRAME`
      - 4
      - Golden 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_BWDREF_FRAME`
      - 5
      - BWD 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_ALTREF2_FRAME`
      - 6
      - ALTREF2 甯у弬鑰冦€?
    - - `V4L2_AV1_REF_ALTREF_FRAME`
      - 7
      - ALTREF 甯у弬鑰冦€?


AV1 鍏ㄥ眬杩愬姩鍙傛暟锛屽 av1 瑙勮寖绗?6.8.17 鑺?鈥淕lobal motion params semantics鈥濓紙鍏ㄥ眬杩愬姩鍙傛暟璇箟锛夋墍杩般€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 鍖呭惈姣忎釜鍙傝€冨抚鏍囧織鐨勪綅鍩熴€傛洿澶氱粏鑺傚弬瑙?AV1 鍏ㄥ眬杩愬姩鏍囧織 <av1_global_motion_flags>銆?
    - - enum `v4l2_av1_warp_model`
      - `type[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 鎵€浣跨敤鐨勫叏灞€杩愬姩鍙樻崲绫诲瀷銆?
    - - __s32
      - `params[V4L2_AV1_TOTAL_REFS_PER_FRAME][^6^]`
      - 鏈瓧娈典笌 av1 涓殑 鈥済m_params鈥?鍚箟鐩稿悓銆?
    - - __u8
      - `invalid`
      - 浣嶅煙锛屾寚绀烘煇涓粰瀹氬弬鑰冨抚鐨勫叏灞€杩愬姩鍙傛暟鏄惁鏃犳晥銆傚弬瑙佺 7.11.3.6 鑺?Setup shear 杩囩▼
        浠ュ強鍙橀噺 鈥渨arpValid鈥濄€備娇鐢?V4L2_AV1_GLOBAL_MOTION_IS_INVALID(ref) 鏉ユ瀯閫犲悎閫傜殑鎺╃爜銆?
    - - __u8
      - `reserved[^3^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`AV1 鍏ㄥ眬杩愬姩鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_GLOBAL`
      - 0x00000001
      - 鎸囧畾鏌愪釜鐗瑰畾鍙傝€冨抚鏄惁瀛樺湪鍏ㄥ眬杩愬姩鍙傛暟銆?
    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_ROT_ZOOM`
      - 0x00000002
      - 鎸囧畾鏌愪釜鐗瑰畾鍙傝€冨抚鏄惁浣跨敤鏃嬭浆涓庣缉鏀惧叏灞€杩愬姩銆?
    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_TRANSLATION`
      - 0x00000004
      - 鎸囧畾鏌愪釜鐗瑰畾鍙傝€冨抚鏄惁浣跨敤骞崇Щ鍏ㄥ眬杩愬姩銆?


AV1 甯ф仮澶嶇被鍨嬨€?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FRAME_RESTORE_NONE`
      - 0
      - 涓嶅簲鐢ㄤ换浣曟护娉€?
    - - `V4L2_AV1_FRAME_RESTORE_WIENER`
      - 1
      - 璋冪敤 Wiener 婊ゆ尝鍣ㄨ繃绋嬨€?
    - - `V4L2_AV1_FRAME_RESTORE_SGRPROJ`
      - 2
      - 璋冪敤鑷紩瀵硷紙self guided锛夋护娉㈠櫒杩囩▼銆?
    - - `V4L2_AV1_FRAME_RESTORE_SWITCHABLE`
      - 3
      - 鎭㈠婊ゆ尝鍣ㄥ彲鍒囨崲銆?


AV1 鐜矾鎭㈠锛屽 av1 瑙勮寖绗?6.10.15 鑺?鈥淟oop restoration params semantics鈥濓紙鐜矾鎭㈠鍙傛暟璇箟锛夋墍杩般€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鍙傝 AV1 鐜矾鎭㈠鏍囧織 <av1_loop_restoration_flags>銆?
    - - __u8
      - `lr_unit_shift`
      - 鎸囧畾浜害鎭㈠灏哄鏄惁搴斿噺鍗娿€?
    - - __u8
      - `lr_uv_shift`
      - 鎸囧畾鑹插害灏哄鏄惁搴斾负浜害灏哄鐨勪竴鍗娿€?
    - - __u8
      - `reserved`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - `v4l2_av1_frame_restoration_type`
      - `frame_restoration_type[V4L2_AV1_NUM_PLANES_MAX]`
      - 鎸囧畾姣忎釜骞抽潰浣跨敤鐨勬仮澶嶇被鍨嬨€?
    - - __u8
      - `loop_restoration_size[V4L2_AV1_MAX_NUM_PLANES]`
      - 鎸囧畾鐜矾鎭㈠鍗曞厓鐨勫ぇ灏忥紝浠ュ綋鍓嶅钩闈腑鐨勬牱鏈负鍗曚綅銆?


`AV1 鐜矾鎭㈠鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_LOOP_RESTORATION_FLAG_USES_LR`
      - 0x00000001
      - 涓?av1 涓殑 UsesLr 鍚箟鐩稿悓銆?
    - - `V4L2_AV1_LOOP_RESTORATION_FLAG_USES_CHROMA_LR`
      - 0x00000002
      - 涓?av1 涓殑 UsesChromaLr 鍚箟鐩稿悓銆?


AV1 CDEF 鍙傛暟璇箟锛屽 av1 瑙勮寖绗?6.10.14 鑺?鈥淐DEF params semantics鈥濓紙CDEF 鍙傛暟璇箟锛夋墍杩般€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `damping_minus_3`
      - 鎺у埗鍘荤幆锛坉eringing锛夋护娉㈠櫒涓殑闃诲凹閲忋€?
    - - __u8
      - `bits`
      - 鎸囧畾鎸囧畾瑕佸簲鐢ㄥ摢涓?CDEF 婊ゆ尝鍣ㄦ墍闇€鐨勬瘮鐗规暟銆?
    - - __u8
      - `y_pri_strength[V4L2_AV1_CDEF_MAX]`
      - 鎸囧畾涓绘护娉㈠櫒寮哄害銆?
    - - __u8
      - `y_sec_strength[V4L2_AV1_CDEF_MAX]`
      - 鎸囧畾娆℃护娉㈠櫒寮哄害銆?
    - - __u8
      - `uv_pri_strength[V4L2_AV1_CDEF_MAX]`
      - 鎸囧畾涓绘护娉㈠櫒寮哄害銆?
    - - __u8
      - `uv_sec_strength[V4L2_AV1_CDEF_MAX]`
      - 鎸囧畾娆℃护娉㈠櫒寮哄害銆?


AV1 鍒嗘鐗瑰緛锛屽 av1 瑙勮寖绗?3 鑺?鈥淪ymbols and abbreviated terms鈥濓紙绗﹀彿涓庣缉鍐欐湳璇級鎵€杩般€?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEG_LVL_ALT_Q`
      - 0
      - 閲忓寲鍣ㄥ垎娈电壒寰佺殑绱㈠紩銆?
    - - `V4L2_AV1_SEG_LVL_ALT_LF_Y_V`
      - 1
      - 鍨傜洿浜害鐜矾婊ゆ尝鍣ㄥ垎娈电壒寰佺殑绱㈠紩銆?
    - - `V4L2_AV1_SEG_LVL_REF_FRAME`
      - 5
      - 鍙傝€冨抚鍒嗘鐗瑰緛鐨勭储寮曘€?
    - - `V4L2_AV1_SEG_LVL_REF_SKIP`
      - 6
      - 璺宠繃锛坰kip锛夊垎娈电壒寰佺殑绱㈠紩銆?
    - - `V4L2_AV1_SEG_LVL_REF_GLOBALMV`
      - 7
      - 鍏ㄥ眬 mv锛坓lobal motion vector锛夌壒寰佺殑绱㈠紩銆?
    - - `V4L2_AV1_SEG_LVL_MAX`
      - 8
      - 鍒嗘鐗瑰緛鐨勬暟閲忋€?


AV1 鍒嗘鍙傛暟锛屽畾涔変簬 av1 瑙勮寖绗?6.8.13 鑺?鈥淪egmentation params semantics鈥濓紙鍒嗘鍙傛暟璇箟锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鍙傝 AV1 鍒嗘鏍囧織 <av1_segmentation_flags>銆?
    - - __u8
      - `last_active_seg_id`
      - 鎸囩ず鍏锋湁鏌愪釜宸插惎鐢ㄧ壒寰佺殑鏈€楂樼紪鍙峰垎娈?id銆傝繖鍦ㄨВ鐮佸垎娈?id 鏃剁敤浜庝粎瑙ｇ爜涓庢墍鐢ㄥ垎娈?
        瀵瑰簲鐨勯€夐」銆?
    - - __u8
      - `feature_enabled[V4L2_AV1_MAX_SEGMENTS]`
      - 浣嶆帺鐮侊紝瀹氫箟姣忎釜鍒嗘涓惎鐢ㄤ簡鍝簺鐗瑰緛銆備娇鐢?V4L2_AV1_SEGMENT_FEATURE_ENABLED 鏉?
        鏋勯€犲悎閫傜殑鎺╃爜銆?
    - - __u16
      - `feature_data[V4L2_AV1_MAX_SEGMENTS][V4L2_AV1_SEG_LVL_MAX]`
      - 闄勫甫鍦ㄦ瘡涓壒寰佷笂鐨勬暟鎹€備粎褰撶壒寰佽鍚敤鏃舵暟鎹潯鐩墠鏈夋晥銆?


`AV1 鍒嗘鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEGMENTATION_FLAG_ENABLED`
      - 0x00000001
      - 鑻ヨ缃紝琛ㄧず鏈抚浣跨敤浜嗗垎娈靛伐鍏凤紙segmentation tool锛夈€傝嫢鏈缃紝琛ㄧず鏈抚鏈娇鐢ㄥ垎娈点€?
    - - `V4L2_AV1_SEGMENTATION_FLAG_UPDATE_MAP`
      - 0x00000002
      - 鑻ヨ缃紝琛ㄧず鍒嗘鏄犲皠鍦ㄦ湰甯цВ鐮佽繃绋嬩腑鏇存柊銆傝嫢鏈缃紝琛ㄧず浣跨敤涓婁竴甯х殑鍒嗘鏄犲皠銆?
    - - `V4L2_AV1_SEGMENTATION_FLAG_TEMPORAL_UPDATE`
      - 0x00000004
      - 鑻ヨ缃紝琛ㄧず鍒嗘鏄犲皠鐨勬洿鏂版槸鐩稿浜庡凡瀛樺湪鐨勫垎娈垫槧灏勭紪鐮佺殑銆傝嫢鏈缃紝琛ㄧず鏂扮殑鍒嗘鏄犲皠
        涓嶅弬鑰冨凡瀛樺湪鐨勫垎娈垫槧灏勭紪鐮併€?
    - - `V4L2_AV1_SEGMENTATION_FLAG_UPDATE_DATA`
      - 0x00000008
      - 鑻ヨ缃紝琛ㄧず鍒嗘鏄犲皠鐨勬洿鏂版槸鐩稿浜庡凡瀛樺湪鐨勫垎娈垫槧灏勭紪鐮佺殑銆傝嫢鏈缃紝琛ㄧず鏂扮殑鍒嗘鏄犲皠
        涓嶅弬鑰冨凡瀛樺湪鐨勫垎娈垫槧灏勭紪鐮併€?
    - - `V4L2_AV1_SEGMENTATION_FLAG_SEG_ID_PRE_SKIP`
      - 0x00000010
      - 鑻ヨ缃紝琛ㄧず鍒嗘 id 灏嗗湪 skip 璇硶鍏冪礌涔嬪墠璇诲彇銆傝嫢鏈缃紝琛ㄧず skip 璇硶鍏冪礌灏嗛鍏堣鍙栥€?


AV1 鐜矾婊ゆ尝鍣ㄥ弬鏁帮紝瀹氫箟浜?av1 瑙勮寖绗?6.8.10 鑺?鈥淟oop filter semantics鈥濓紙鐜矾婊ゆ尝鍣ㄨ涔夛級銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鏇村缁嗚妭鍙傝 AV1 鐜矾婊ゆ尝鍣ㄦ爣蹇?<av1_loop_filter_flags>銆?
    - - __u8
      - `level[^4^]`
      - 涓€涓寘鍚幆璺护娉㈠櫒寮哄害鍊肩殑鏁扮粍銆傛牴鎹婊ゆ尝鐨勫浘鍍忓钩闈互鍙婅婊ゆ尝鐨勮竟缂樻柟鍚戯紙鍨傜洿鎴栨按骞筹級锛?
        浣跨敤鏁扮粍涓笉鍚岀殑鐜矾婊ゆ尝鍣ㄥ己搴﹀€笺€?
    - - __u8
      - `sharpness`
      - 琛ㄧず閿愬害绛夌骇銆俵oop_filter_level 涓?loop_filter_sharpness 鍏卞悓鍐冲畾浣曟椂瀵瑰潡杈圭紭杩涜
        婊ゆ尝锛屼互鍙婃护娉㈠鏍锋湰鍊肩殑鏀瑰彉閲忋€傜幆璺护娉㈣繃绋嬪湪 av1 瑙勮寖绗?7.14 鑺備腑鎻忚堪銆?
    - - __u8
      - `ref_deltas[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 鍖呭惈鍩轰簬鎵€閫夊弬鑰冨抚瀵规护娉㈠櫒绛夌骇鎵€闇€鐨勮皟鏁淬€傝嫢璇ヨ娉曞厓绱犱笉瀛樺湪锛屽垯淇濇寔鍏跺厛鍓嶇殑鍊笺€?
    - - __u8
      - `mode_deltas[^2^]`
      - 鍖呭惈鍩轰簬鎵€閫夋ā寮忓婊ゆ尝鍣ㄧ瓑绾ф墍闇€鐨勮皟鏁淬€傝嫢璇ヨ娉曞厓绱犱笉瀛樺湪锛屽垯淇濇寔鍏跺厛鍓嶇殑鍊笺€?
    - - __u8
      - `delta_lf_res`
      - 鎸囧畾搴斾綔鐢ㄤ簬宸茶В鐮佺幆璺护娉㈠櫒 delta 鍊肩殑宸︾Щ閲忋€?


`AV1 鐜矾婊ゆ尝鍣ㄦ爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_ENABLED`
      - 0x00000001
      - 鑻ヨ缃紝琛ㄧず婊ゆ尝鍣ㄧ瓑绾у彇鍐充簬鐢ㄤ簬棰勬祴鏌愪釜鍧楃殑妯″紡鍜屽弬鑰冨抚銆傝嫢鏈缃紝琛ㄧず婊ゆ尝鍣ㄧ瓑绾?
        涓嶅彇鍐充簬妯″紡鍜屽弬鑰冨抚銆?
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_UPDATE`
      - 0x00000002
      - 鑻ヨ缃紝琛ㄧず瀛樺湪棰濆鐨勮娉曞厓绱狅紝鐢ㄤ簬鎸囧畾鍝簺妯″紡鍜屽弬鑰冨抚鐨?delta 闇€瑕佹洿鏂般€傝嫢鏈缃紝
        琛ㄧず杩欎簺璇硶鍏冪礌涓嶅瓨鍦ㄣ€?
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_PRESENT`
      - 0x00000004
      - 鎸囧畾鏄惁瀛樺湪鐜矾婊ゆ尝鍣?delta 鍊笺€?
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_MULTI`
      - 0x00000008
      - 鍊间负 1 鎸囧畾涓烘按骞充寒搴﹁竟缂樸€佸瀭鐩翠寒搴﹁竟缂樸€乁 杈圭紭鍜?V 杈圭紭鍒嗗埆鍙戦€佺嫭绔嬬殑鐜矾婊ゆ尝鍣?
        delta銆俤elta_lf_multi 鍊间负 0 鎸囧畾鎵€鏈夎竟缂樹娇鐢ㄧ浉鍚岀殑鐜矾婊ゆ尝鍣?delta銆?


AV1 閲忓寲鍙傛暟锛屽畾涔変簬 av1 瑙勮寖绗?6.8.11 鑺?鈥淨uantization params semantics鈥濓紙閲忓寲鍙傛暟璇箟锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鏇村缁嗚妭鍙傝 AV1 鐜矾婊ゆ尝鍣ㄦ爣蹇?<av1_quantization_flags>銆?
    - - __u8
      - `base_q_idx`
      - 鎸囩ず鍩虹甯?qindex銆傜敤浜?Y AC 绯绘暟锛屽苟浣滀负鍏朵粬閲忓寲鍣ㄧ殑鍩虹鍊笺€?
    - - __u8
      - `delta_q_y_dc`
      - 鎸囩ず鐩稿 base_q_idx 鐨?Y DC 閲忓寲鍣ㄣ€?
    - - __u8
      - `delta_q_u_dc`
      - 鎸囩ず鐩稿 base_q_idx 鐨?U DC 閲忓寲鍣ㄣ€?
    - - __u8
      - `delta_q_u_ac`
      - 鎸囩ず鐩稿 base_q_idx 鐨?U AC 閲忓寲鍣ㄣ€?
    - - __u8
      - `delta_q_v_dc`
      - 鎸囩ず鐩稿 base_q_idx 鐨?V DC 閲忓寲鍣ㄣ€?
    - - __u8
      - `delta_q_v_ac`
      - 鎸囩ず鐩稿 base_q_idx 鐨?V AC 閲忓寲鍣ㄣ€?
    - - __u8
      - `qm_y`
      - 鎸囧畾鐢ㄤ簬浜害骞抽潰瑙ｇ爜鐨勯噺鍖栫煩闃电瓑绾с€?
    - - __u8
      - `qm_u`
      - 鎸囧畾鐢ㄤ簬鑹插害 U 骞抽潰瑙ｇ爜鐨勯噺鍖栫煩闃电瓑绾с€?
    - - __u8
      - `qm_v`
      - 鎸囧畾鐢ㄤ簬鑹插害 V 骞抽潰瑙ｇ爜鐨勯噺鍖栫煩闃电瓑绾с€?
    - - __u8
      - `delta_q_res`
      - 鎸囧畾搴斾綔鐢ㄤ簬宸茶В鐮侀噺鍖栧櫒绱㈠紩 delta 鍊肩殑宸︾Щ閲忋€?


`AV1 閲忓寲鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_QUANTIZATION_FLAG_DIFF_UV_DELTA`
      - 0x00000001
      - 鑻ヨ缃紝琛ㄧず U 鍜?V delta 閲忓寲鍣ㄥ€艰鍒嗗埆缂栫爜銆傝嫢鏈缃紝琛ㄧず U 鍜?V delta 閲忓寲鍣?
        鍊煎叡浜竴涓叕鍏卞€笺€?
    - - `V4L2_AV1_QUANTIZATION_FLAG_USING_QMATRIX`
      - 0x00000002
      - 鑻ヨ缃紝鎸囧畾灏嗕娇鐢ㄩ噺鍖栫煩闃垫潵璁＄畻閲忓寲鍣ㄣ€?
    - - `V4L2_AV1_QUANTIZATION_FLAG_DELTA_Q_PRESENT`
      - 0x00000004
      - 鎸囧畾鏄惁瀛樺湪閲忓寲鍣ㄧ储寮?delta 鍊笺€?


AV1 Tile 淇℃伅锛屽畾涔変簬 ref:`av1` 瑙勮寖绗?6.8.14 鑺?鈥淭ile info semantics鈥濓紙Tile 淇℃伅璇箟锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鏇村缁嗚妭鍙傝 AV1 Tile 淇℃伅鏍囧織 <av1_tile_info_flags>銆?
    - - __u8
      - `context_update_tile_id`
      - 鎸囧畾鐢ㄤ簬 CDF 鏇存柊鐨?tile銆?
    - - __u8
      - `tile_cols`
      - 鎸囧畾璺ㄥ抚鐨?tile 鏁伴噺銆?
    - - __u8
      - `tile_rows`
      - 鎸囧畾娌垮抚鍚戜笅鐨?tile 鏁伴噺銆?
    - - __u32
      - `mi_col_starts[V4L2_AV1_MAX_TILE_COLS + 1]`
      - 涓€涓暟缁勶紝鎸囧畾姣忎釜 tile 璺ㄥ浘鍍忕殑璧峰鍒楋紙浠?4x4 浜害鏍锋湰涓哄崟浣嶏級銆?
    - - __u32
      - `mi_row_starts[V4L2_AV1_MAX_TILE_ROWS + 1]`
      - 涓€涓暟缁勶紝鎸囧畾姣忎釜 tile 璺ㄥ浘鍍忕殑璧峰琛岋紙浠?4x4 浜害鏍锋湰涓哄崟浣嶏級銆?
    - - __u32
      - `width_in_sbs_minus_1[V4L2_AV1_MAX_TILE_COLS]`
      - 鎸囧畾 tile 鐨勫搴﹀噺 1锛屼互 superblock 涓哄崟浣嶃€?
    - - __u32
      - `height_in_sbs_minus_1[V4L2_AV1_MAX_TILE_ROWS]`
      - 鎸囧畾 tile 鐨勯珮搴﹀噺 1锛屼互 superblock 涓哄崟浣嶃€?
    - - __u8
      - `tile_size_bytes`
      - 鎸囧畾缂栫爜姣忎釜 tile 澶у皬鎵€闇€鐨勬瘮鐗规暟銆?
    - - __u8
      - `reserved[^3^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`AV1 Tile 淇℃伅鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_TILE_INFO_FLAG_UNIFORM_TILE_SPACING`
      - 0x00000001
      - 鑻ヨ缃紝琛ㄧず tile 鍦ㄥ抚涓婂潎鍖€鍒嗗竷锛堟崲瑷€涔嬶紝闄ゅ彸渚у拰搴曢儴杈圭紭鍙兘杈冨皬鐨?tile 澶栵紝鎵€鏈?
        tile 澶у皬鐩稿悓锛夈€傝嫢鏈缃紝琛ㄧず tile 澶у皬鏄缂栫爜鐨勩€?


AV1 甯х被鍨?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_KEY_FRAME`
      - 0
      - 鍏抽敭甯с€?
    - - `V4L2_AV1_INTER_FRAME`
      - 1
      - 甯ч棿甯с€?
    - - `V4L2_AV1_INTRA_ONLY_FRAME`
      - 2
      - 浠呭抚鍐呭抚銆?
    - - `V4L2_AV1_SWITCH_FRAME`
      - 3
      - 鍒囨崲甯с€?


AV1 鎻掑€兼护娉㈠櫒


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP`
      - 0
      - 鍏娊澶存护娉㈠櫒銆?
    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH`
      - 1
      - 鍏娊澶村钩婊戞护娉㈠櫒銆?
    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SHARP`
      - 2
      - 鍏娊澶撮攼鍒╂护娉㈠櫒銆?
    - - `V4L2_AV1_INTERPOLATION_FILTER_BILINEAR`
      - 3
      - 鍙岀嚎鎬ф护娉㈠櫒銆?
    - - `V4L2_AV1_INTERPOLATION_FILTER_SWITCHABLE`
      - 4
      - 婊ゆ尝鍣ㄩ€夋嫨浜庡潡绾у埆鍙戝嚭淇″彿銆?


AV1 Tx 妯″紡锛屽 av1 瑙勮寖绗?6.8.21 鑺?鈥淭X mode semantics鈥濓紙TX 妯″紡璇箟锛夋墍杩般€?


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_TX_MODE_ONLY_4X4`
      - 0
      - 閫嗗彉鎹㈠皢浠呬娇鐢?4x4 鍙樻崲銆?
    - - `V4L2_AV1_TX_MODE_LARGEST`
      - 1
      - 閫嗗彉鎹㈠皢浣跨敤鑳芥斁鍏ュ潡鍐呯殑鏈€澶у彉鎹㈠昂瀵搞€?
    - - `V4L2_AV1_TX_MODE_SELECT`
      - 2
      - 鍙樻崲灏哄鐨勯€夋嫨涓烘瘡涓潡鏄惧紡鎸囧畾銆?


`V4L2_CID_STATELESS_AV1_FRAME (struct)`
    琛ㄧず涓€涓抚澶?OBU銆傛洿澶氱粏鑺傚弬瑙?av1 瑙勮寖绗?6.8 鑺?鈥淔rame Header OBU semantics鈥?
    锛堝抚澶?OBU 璇箟锛夈€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_av1_tile_info`
      - `tile_info`
      - Tile 淇℃伅銆?
    - - struct `v4l2_av1_quantization`
      - `quantization`
      - 閲忓寲鍙傛暟銆?
    - - __u8
      - `superres_denom`
      - 涓婇噰鏍锋瘮渚嬪垎姣嶃€?
    - - struct `v4l2_av1_segmentation`
      - `segmentation`
      - 鍒嗘鍙傛暟銆?
    - - struct `v4l2_av1_loop_filter`
      - `loop_filter`
      - 鐜矾婊ゆ尝鍣ㄥ弬鏁般€?
    - - struct `v4l2_av1_cdef`
      - `cdef`
      - CDEF 鍙傛暟銆?
    - - __u8
      - `skip_mode_frame[^2^]`
      - 鎸囧畾褰?skip_mode 绛変簬 1 鏃剁敤浜庡鍚堥娴嬬殑甯с€?
    - - __u8
      - `primary_ref_frame`
      - 鎸囧畾鍝釜鍙傝€冨抚鍖呭惈鍦?CDF 鍊间互鍙婂簲鍦ㄥ抚寮€濮嬫椂鍔犺浇鐨勫叾浠栫姸鎬併€?
    - - struct `v4l2_av1_loop_restoration`
      - `loop_restoration`
      - 鐜矾鎭㈠鍙傛暟銆?
    - - struct `v4l2_av1_global_motion`
      - `global_motion`
      - 鍏ㄥ眬杩愬姩鍙傛暟銆?
    - - __u32
      - `flags`
      - 鏇村缁嗚妭鍙傝 AV1 甯ф爣蹇?<av1_frame_flags>銆?
    - - enum `v4l2_av1_frame_type`
      - `frame_type`
      - 鎸囧畾 AV1 甯х被鍨嬨€?
    - - __u32
      - `order_hint`
      - 鎸囧畾鏈抚鏈熸湜杈撳嚭椤哄簭鐨?OrderHintBits 涓渶浣庢湁鏁堜綅銆?
    - - __u32
      - `upscaled_width`
      - 涓婇噰鏍峰搴︺€?
    - - enum `v4l2_av1_interpolation_filter`
      - `interpolation_filter`
      - 鎸囧畾鐢ㄤ簬鎵ц甯ч棿棰勬祴鐨勬护娉㈠櫒閫夋嫨銆?
    - - enum `v4l2_av1_tx_mode`
      - `tx_mode`
      - 鎸囧畾鍙樻崲灏哄濡備綍纭畾銆?
    - - __u32
      - `frame_width_minus_1`
      - 鍔?1 寰楀埌甯х殑瀹藉害銆?
    - - __u32
      - `frame_height_minus_1`
      - 鍔?1 寰楀埌甯х殑楂樺害銆?
    - - __u16
      - `render_width_minus_1`
      - 鍔?1 寰楀埌浠ヤ寒搴︽牱鏈〃绀虹殑甯ф覆鏌撳搴︺€?
    - - __u16
      - `render_height_minus_1`
      - 鍔?1 寰楀埌浠ヤ寒搴︽牱鏈〃绀虹殑甯ф覆鏌撻珮搴︺€?
    - - __u32
      - `current_frame_id`
      - 鎸囧畾褰撳墠甯х殑甯?id 缂栧彿銆傚抚 id 缂栧彿鏄笉褰卞搷瑙ｇ爜杩囩▼鐨勯檮鍔犱俊鎭紝浣嗕负瑙ｇ爜鍣ㄦ彁渚涗簡妫€娴?
        缂哄け鍙傝€冨抚鐨勬柟寮忥紝浠ヤ究閲囧彇閫傚綋鎺柦銆?
    - - __u8
      - `buffer_removal_time[V4L2_AV1_MAX_OPERATING_POINTS]`
      - 鎸囧畾瀵逛簬鎿嶄綔鐐?opNum锛屼粠鏈€鍚庝竴涓殢鏈鸿闂偣鐨勭Щ闄ゆ椂闂磋捣绠椼€佷互 DecCT 鏃堕挓鑺傛媿涓哄崟浣嶇殑
        甯хЩ闄ゆ椂闂淬€?
    - - __u8
      - `reserved[^4^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?
    - - __u32
      - `order_hints[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 鎸囧畾姣忎釜鍙傝€冨抚鐨勬湡鏈涜緭鍑洪『搴忔彁绀恒€傛湰瀛楁瀵瑰簲浜庤鑼冿紙绗?5.9.2 鑺?鈥淯ncompressed header
        syntax鈥濓紝鏈帇缂╁ご閮ㄨ娉曪級涓殑 OrderHints 鍙橀噺銆傚洜姝わ紝瀹冧粎鐢ㄤ簬闈炲抚鍐呭抚锛屽惁鍒欒蹇界暐銆?
        order_hints[^0^] 濮嬬粓琚拷鐣ャ€?
    - - __u64
      - `reference_frame_ts[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 浠?enum `v4l2_av1_reference_frame` 鐨?`V4L2_AV1_REF_LAST_FRAME` 寮€濮嬫灇涓剧殑姣忎釜鍙傝€冨抚
        鐨?V4L2 鏃堕棿鎴炽€傝繖琛ㄧず瑙勮寖涓弿杩扮殑鍙傝€冩Ы鐘舵€侊紝骞剁敱鐢ㄦ埛绌洪棿閫氳繃绗?7.20 鑺傜殑
        鈥淩eference frame update process鈥濓紙鍙傝€冨抚鏇存柊杩囩▼锛夋洿鏂般€傝鏃堕棿鎴冲紩鐢?struct
        `v4l2_buffer` 涓殑 `timestamp` 瀛楁銆備娇鐢?`v4l2_timeval_to_ns()` 鍑芥暟灏?struct
        `timeval` 鍦?struct `v4l2_buffer` 涓浆鎹负 __u64銆?
    - - __s8
      - `ref_frame_idx[V4L2_AV1_REFS_PER_FRAME]`
      - 鎸囧悜 `reference_frame_ts` 鐨勭储寮曪紝琛ㄧず甯ч棿甯т娇鐢ㄧ殑鍙傝€冩湁搴忓垪琛ㄣ€備笌鍚屽悕鐨勭爜娴佽娉曞厓绱?
        鍖归厤銆?
    - - __u8
      - `refresh_frame_flags`
      - 鍖呭惈涓€涓綅鎺╃爜锛屾寚瀹氳В鐮佸悗鍝簺鍙傝€冨抚妲藉皢鐢ㄥ綋鍓嶅抚鏇存柊銆?


`AV1 甯ф爣蹇梎


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FRAME_FLAG_SHOW_FRAME`
      - 0x00000001
      - 鑻ヨ缃紝鎸囧畾鏈抚涓€鏃﹁В鐮佸氨搴旂珛鍗宠緭鍑恒€傝嫢鏈缃紝鎸囧畾鏈抚涓嶅簲绔嬪嵆杈撳嚭锛涜嫢鍚庣画鏈帇缂╁ご
        浣跨敤 show_existing_frame 绛変簬 1锛屽垯鍙兘鍦ㄤ箣鍚庤緭鍑恒€?
    - - `V4L2_AV1_FRAME_FLAG_SHOWABLE_FRAME`
      - 0x00000002
      - 鑻ヨ缃紝鎸囧畾鏈抚鍙娇鐢?show_existing_frame 鏈哄埗杈撳嚭銆傝嫢鏈缃紝鎸囧畾鏈抚涓嶄細浣跨敤
        show_existing_frame 鏈哄埗杈撳嚭銆?
    - - `V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE`
      - 0x00000004
      - 鎸囧畾鏄惁鍚敤浜嗛敊璇煣鎬фā寮忋€?
    - - `V4L2_AV1_FRAME_FLAG_DISABLE_CDF_UPDATE`
      - 0x00000008
      - 鎸囧畾鏄惁搴旂鐢ㄧ鍙疯В鐮佽繃绋嬩腑鐨?CDF 鏇存柊銆?
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_SCREEN_CONTENT_TOOLS`
      - 0x00000010
      - 鑻ヨ缃紝琛ㄧず甯у唴鍧楀彲浣跨敤璋冭壊鏉匡紙palette锛夌紪鐮併€傝嫢鏈缃紝琛ㄧず浠庝笉浣跨敤璋冭壊鏉跨紪鐮併€?
    - - `V4L2_AV1_FRAME_FLAG_FORCE_INTEGER_MV`
      - 0x00000020
      - 鑻ヨ缃紝鎸囧畾杩愬姩鐭㈤噺灏嗗缁堜负鏁存暟銆傝嫢鏈缃紝鎸囧畾杩愬姩鐭㈤噺鍙寘鍚垎鏁版瘮鐗广€?
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC`
      - 0x00000040
      - 鑻ヨ缃紝琛ㄧず鏈抚涓彲浣跨敤甯у唴鍧楀鍒讹紙intra block copy锛夈€傝嫢鏈缃紝琛ㄧず鏈抚涓嶅厑璁?
        甯у唴鍧楀鍒躲€?
    - - `V4L2_AV1_FRAME_FLAG_USE_SUPERRES`
      - 0x00000080
      - 鑻ヨ缃紝琛ㄧず闇€瑕佷笂閲囨牱銆?
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_HIGH_PRECISION_MV`
      - 0x00000100
      - 鑻ヨ缃紝鎸囧畾杩愬姩鐭㈤噺浠ュ叓鍒嗕箣涓€鍍忕礌绮惧害鎸囧畾銆傝嫢鏈缃紝鎸囧畾杩愬姩鐭㈤噺浠ュ洓鍒嗕箣涓€鍍忕礌绮惧害
        鎸囧畾銆?
    - - `V4L2_AV1_FRAME_FLAG_IS_MOTION_MODE_SWITCHABLE`
      - 0x00000200
      - 鑻ユ湭璁剧疆锛屾寚瀹氫粎浣跨敤 SIMPLE 杩愬姩妯″紡銆?
    - - `V4L2_AV1_FRAME_FLAG_USE_REF_FRAME_MVS`
      - 0x00000400
      - 鑻ヨ缃紝鎸囧畾瑙ｇ爜褰撳墠甯ф椂鍙娇鐢ㄦ潵鑷笂涓€甯х殑杩愬姩鐭㈤噺淇℃伅銆傝嫢鏈缃紝鎸囧畾涓嶄娇鐢ㄨ淇℃伅銆?
    - - `V4L2_AV1_FRAME_FLAG_DISABLE_FRAME_END_UPDATE_CDF`
      - 0x00000800
      - 鑻ヨ缃紝琛ㄧず甯ф湯灏剧殑 CDF 鏇存柊琚鐢ㄣ€傝嫢鏈缃紝琛ㄧず甯ф湯灏剧殑 CDF 鏇存柊琚惎鐢ㄣ€?
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_WARPED_MOTION`
      - 0x00001000
      - 鑻ヨ缃紝琛ㄧず motion_mode 璇硶鍏冪礌鍙兘鍑虹幇锛涜嫢鏈缃紝琛ㄧず motion_mode 璇硶鍏冪礌涓嶄細
        鍑虹幇銆?
    - - `V4L2_AV1_FRAME_FLAG_REFERENCE_SELECT`
      - 0x00002000
      - 鑻ヨ缃紝鎸囧畾甯ч棿鍧楃殑 mode info 鍖呭惈 comp_mode 璇硶鍏冪礌锛屾寚绀轰娇鐢ㄥ崟鍙傝€冭繕鏄鍚堝弬鑰?
        棰勬祴銆傝嫢鏈缃紝鎸囧畾鎵€鏈夊抚闂村潡浣跨敤鍗曢娴嬨€?
    - - `V4L2_AV1_FRAME_FLAG_REDUCED_TX_SET`
      - 0x00004000
      - 鑻ヨ缃紝鎸囧畾鏈抚琚檺鍒朵负瀹屾暣鍙樻崲绫诲瀷闆嗗悎鐨勪竴涓缉鍑忓瓙闆嗐€?
    - - `V4L2_AV1_FRAME_FLAG_SKIP_MODE_ALLOWED`
      - 0x00008000
      - 鏈爣蹇椾笌 av1 涓殑 SkipModeAllowed 鍚箟鐩稿悓銆?
    - - `V4L2_AV1_FRAME_FLAG_SKIP_MODE_PRESENT`
      - 0x00010000
      - 鑻ヨ缃紝鎸囧畾 skip_mode 璇硶鍏冪礌灏嗗嚭鐜帮紱鑻ユ湭璁剧疆锛屾寚瀹氭湰甯т笉浣跨敤 skip_mode銆?
    - - `V4L2_AV1_FRAME_FLAG_FRAME_SIZE_OVERRIDE`
      - 0x00020000
      - 鑻ヨ缃紝鎸囧畾甯уぇ灏忓皢鎸囧畾涓烘煇涓弬鑰冨抚鐨勫ぇ灏忥紝鎴栦粠 frame_width_minus_1 鍜?
        frame_height_minus_1 璇硶鍏冪礌璁＄畻寰楀嚭銆傝嫢鏈缃紝鎸囧畾甯уぇ灏忕瓑浜庡簭鍒楀ご涓殑澶у皬銆?
    - - `V4L2_AV1_FRAME_FLAG_BUFFER_REMOVAL_TIME_PRESENT`
      - 0x00040000
      - 鑻ヨ缃紝鎸囧畾 buffer_removal_time 瀛樺湪銆傝嫢鏈缃紝鎸囧畾 buffer_removal_time 涓嶅瓨鍦ㄣ€?
    - - `V4L2_AV1_FRAME_FLAG_FRAME_REFS_SHORT_SIGNALING`
      - 0x00080000
      - 鑻ヨ缃紝琛ㄧず浠呮樉寮忓彂鍑轰袱涓弬鑰冨抚鐨勪俊鍙枫€傝嫢鏈缃紝琛ㄧず鎵€鏈夊弬鑰冨抚閮借鏄惧紡鍙戝嚭淇″彿銆?


`V4L2_CID_STATELESS_AV1_FILM_GRAIN (struct)`
    琛ㄧず鍙€夌殑鑳剁墖棰楃矑鍙傛暟銆傛洿澶氱粏鑺傚弬瑙?av1 瑙勮寖绗?6.8.20 鑺?鈥淔ilm grain params semantics鈥?
    锛堣兌鐗囬绮掑弬鏁拌涔夛級銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 鍙傝 AV1 鑳剁墖棰楃矑鏍囧織 <av1_film_grain_flags>銆?
    - - __u8
      - `cr_mult`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cr 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨?cr 鍒嗛噺鐨勪箻鏁般€?
    - - __u16
      - `grain_seed`
      - 鎸囧畾鑳剁墖棰楃矑鍚堟垚杩囩▼涓墍浣跨敤鐨勪吉闅忔満鏁扮殑璧峰鍊笺€?
    - - __u8
      - `film_grain_params_ref_idx`
      - 鎸囩ず鍝釜鍙傝€冨抚鍖呭惈鐢ㄤ簬鏈抚鐨勮兌鐗囬绮掑弬鏁般€?
    - - __u8
      - `num_y_points`
      - 鎸囧畾浜害鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮殑鐐规暟銆?
    - - __u8
      - `point_y_value[V4L2_AV1_MAX_NUM_Y_POINTS]`
      - 琛ㄧず浜害鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨?x锛堜寒搴﹀€硷級鍧愭爣銆傝繖浜涘€煎湪 0..255 鐨勮寖鍥翠笂鍙戝嚭
        淇″彿銆傚浜?10 浣嶈棰戯紝杩欎簺鍊煎搴斾簬闄や互 4 鐨勪寒搴﹀€硷紱瀵逛簬 12 浣嶈棰戯紝瀵瑰簲浜庨櫎浠?16 鐨?
        浜害鍊笺€?
    - - __u8
      - `point_y_scaling[V4L2_AV1_MAX_NUM_Y_POINTS]`
      - 琛ㄧず浜害鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨勭缉鏀撅紙杈撳嚭锛夊€笺€?
    - - __u8
      - `num_cb_points`
      - 鎸囧畾 cb 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮殑鐐规暟銆?
    - - __u8
      - `point_cb_value[V4L2_AV1_MAX_NUM_CB_POINTS]`
      - 琛ㄧず cb 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨?x 鍧愭爣銆傝繖浜涘€煎湪 0..255 鐨勮寖鍥翠笂鍙戝嚭淇″彿銆?
    - - __u8
      - `point_cb_scaling[V4L2_AV1_MAX_NUM_CB_POINTS]`
      - 琛ㄧず cb 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨勭缉鏀撅紙杈撳嚭锛夊€笺€?
    - - __u8
      - `num_cr_points`
      - 琛ㄧず cr 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮殑鐐规暟銆?
    - - __u8
      - `point_cr_value[V4L2_AV1_MAX_NUM_CR_POINTS]`
      - 琛ㄧず cr 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨?x 鍧愭爣銆傝繖浜涘€煎湪 0..255 鐨勮寖鍥翠笂鍙戝嚭淇″彿銆?
    - - __u8
      - `point_cr_scaling[V4L2_AV1_MAX_NUM_CR_POINTS]`
      - 琛ㄧず cr 鍒嗛噺鍒嗘绾挎€х缉鏀惧嚱鏁扮 i 涓偣鐨勭缉鏀撅紙杈撳嚭锛夊€笺€?
    - - __u8
      - `grain_scaling_minus_8`
      - 琛ㄧず搴旂敤浜庤壊搴﹀垎閲忓€肩殑绉讳綅鍑?8銆俫rain_scaling_minus_8 鍙彇 0..3 鐨勫€硷紝骞跺喅瀹氳兌鐗囬绮?
        鏍囧噯宸殑鑼冨洿涓庨噺鍖栨闀裤€?
    - - __u8
      - `ar_coeff_lag`
      - 鎸囧畾浜害涓庤壊搴︾殑鑷洖褰掔郴鏁版暟閲忋€?
    - - __u8
      - `ar_coeffs_y_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 鎸囧畾鐢ㄤ簬 Y 骞抽潰鐨勮嚜鍥炲綊绯绘暟銆?
    - - __u8
      - `ar_coeffs_cb_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 鎸囧畾鐢ㄤ簬 U 骞抽潰鐨勮嚜鍥炲綊绯绘暟銆?
    - - __u8
      - `ar_coeffs_cr_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 鎸囧畾鐢ㄤ簬 V 骞抽潰鐨勮嚜鍥炲綊绯绘暟銆?
    - - __u8
      - `ar_coeff_shift_minus_6`
      - 鎸囧畾鑷洖褰掔郴鏁扮殑鑼冨洿銆傚€?0銆?銆?銆? 鍒嗗埆瀵瑰簲鑷洖褰掔郴鏁扮殑鑼冨洿 [-2, 2)銆乕-1, 1)銆?
        [-0.5, 0.5) 涓?[-0.25, 0.25)銆?
    - - __u8
      - `grain_scale_shift`
      - 鎸囧畾鍦ㄩ绮掑悎鎴愯繃绋嬩腑楂樻柉闅忔満鏁板簲缂╁皬鐨勭▼搴︺€?
    - - __u8
      - `cb_mult`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cb 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨?cb 鍒嗛噺鐨勪箻鏁般€?
    - - __u8
      - `cb_luma_mult`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cb 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨勫钩鍧囦寒搴﹀垎閲忕殑涔樻暟銆?
    - - __u8
      - `cr_luma_mult`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cr 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨勫钩鍧囦寒搴﹀垎閲忕殑涔樻暟銆?
    - - __u16
      - `cb_offset`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cb 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨勫亸绉汇€?
    - - __u16
      - `cr_offset`
      - 琛ㄧず鐢ㄤ簬鎺ㄥ cr 鍒嗛噺缂╂斁鍑芥暟杈撳叆绱㈠紩鐨勫亸绉汇€?
    - - __u8
      - `reserved[^4^]`
      - 搴旂敤绋嬪簭涓庨┍鍔ㄥ繀椤诲皢鏈瓧娈电疆涓洪浂銆?


`AV1 鑳剁墖棰楃矑鏍囧織`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FILM_GRAIN_FLAG_APPLY_GRAIN`
      - 0x00000001
      - 鑻ヨ缃紝鎸囧畾搴斿悜鏈抚娣诲姞鑳剁墖棰楃矑銆傝嫢鏈缃紝鎸囧畾涓嶅簲娣诲姞鑳剁墖棰楃矑銆?
    - - `V4L2_AV1_FILM_GRAIN_FLAG_UPDATE_GRAIN`
      - 0x00000002
      - 鑻ヨ缃紝琛ㄧず搴斿彂閫佷竴缁勬柊鍙傛暟銆傝嫢鏈缃紝鎸囧畾搴斾娇鐢ㄥ墠涓€缁勫弬鏁般€?
    - - `V4L2_AV1_FILM_GRAIN_FLAG_CHROMA_SCALING_FROM_LUMA`
      - 0x00000004
      - 鑻ヨ缃紝鎸囧畾鑹插害缂╂斁鐢变寒搴︾缉鏀炬帹瀵煎緱鍑恒€?
    - - `V4L2_AV1_FILM_GRAIN_FLAG_OVERLAP`
      - 0x00000008
      - 鑻ヨ缃紝琛ㄧず搴斿簲鐢ㄨ兌鐗囬绮掑潡涔嬮棿鐨勯噸鍙犮€傝嫢鏈缃紝琛ㄧず涓嶅簲搴旂敤鑳剁墖棰楃矑鍧椾箣闂寸殑閲嶅彔銆?
    - - `V4L2_AV1_FILM_GRAIN_FLAG_CLIP_TO_RESTRICTED_RANGE`
      - 0x00000010
      - 鑻ヨ缃紝琛ㄧず鍦ㄥ鏍锋湰鍊兼坊鍔犺兌鐗囬绮掑悗锛屽簲灏嗗叾瑁佸壀鍒板彈闄愶紙婕旀挱瀹わ紝鍗抽檺鍒讹級鑼冨洿
        锛堝弬瑙?color_range 璇箟涓叧浜?studio swing 鐨勮В閲婏級銆傝嫢鏈缃紝琛ㄧず鍦ㄥ鏍锋湰鍊兼坊鍔?
        鑳剁墖棰楃矑鍚庯紝搴斿皢鍏惰鍓埌鍏ㄨ寖鍥淬€?

