
######## Stateless Codec Control Reference


无状态编解码器（Stateless Codec）控制类旨在支持无状态的解码器与编码器（即硬件加速器）。

这些驱动通常由 stateless_decoder 支持，并处理已解析的像素格式，例如 V4L2_PIX_FMT_H264_SLICE。

## 无状态编解码器控制 ID



`V4L2_CID_CODEC_STATELESS_CLASS (class)`
    无状态编解码器类描述符。


`V4L2_CID_STATELESS_H264_SPS (struct)`
    指定与相应 H264 切片数据关联的序列参数集（从码流中提取）。其中包含配置 H264
    无状态硬件解码流水线所需的参数。码流参数依据 h264 标准第 7.4.2.1.1 节
    “Sequence Parameter Set Data Semantics”（序列参数集数据语义）定义。除非有明确注释
    另有说明，否则进一步的文档请参考上述规范。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `profile_idc`
      -
    - - __u8
      - `constraint_set_flags`
      - 参见序列参数集约束标志 <h264_sps_constraints_set_flags>
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
      - 参见序列参数集标志 <h264_sps_flags>


    \normalsize


`序列参数集约束标志`


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


`序列参数集标志`


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
    指定与相应 H264 切片数据关联的图像参数集（从码流中提取）。其中包含配置 H264
    无状态硬件解码流水线所需的参数。码流参数依据 h264 标准第 7.4.2.2 节
    “Picture Parameter Set RBSP Semantics”（图像参数集 RBSP 语义）定义。除非有明确注释
    另有说明，否则进一步的文档请参考上述规范。



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
      - 参见图像参数集标志 <h264_pps_flags>


    \normalsize


`图像参数集标志`


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
      - 必须对该图像使用 `V4L2_CID_STATELESS_H264_SCALING_MATRIX`。


    \endgroup

`V4L2_CID_STATELESS_H264_SCALING_MATRIX (struct)`
    指定与相应 H264 切片数据关联的缩放矩阵（从码流中提取）。码流参数依据 h264 标准
    第 7.4.2.1.1.1 节 “Scaling List Semantics”（缩放列表语义）定义。除非有明确注释
    另有说明，否则进一步的文档请参考上述规范。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `scaling_list_4x4[^6^][^16^]`
      - 应用逆扫描过程后的缩放矩阵。期望的列表顺序为：帧内 Y、帧内 Cb、帧内 Cr、
        帧间 Y、帧间 Cb、帧间 Cr。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_8x8[^6^][^64^]`
      - 应用逆扫描过程后的缩放矩阵。期望的列表顺序为：帧内 Y、帧间 Y、帧内 Cb、
        帧间 Cb、帧内 Cr、帧间 Cr。每个缩放列表中的值按光栅扫描顺序排列。

`V4L2_CID_STATELESS_H264_SLICE_PARAMS (struct)`
    指定与相应 H264 切片数据关联的切片参数（从码流中提取）。其中包含配置 H264
    无状态硬件解码流水线所需的参数。码流参数依据 h264 标准第 7.4.3 节
    “Slice Header Semantics”（切片头语义）定义。除非有明确注释另有说明，否则进一步的
    文档请参考上述规范。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `header_bit_size`
      - 从本切片起始处到 slice_data() 的偏移量（以比特计）。
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
      - 若未设置 num_ref_idx_active_override_flag，本字段必须设为
        num_ref_idx_l0_default_active_minus1 的值。
    - - __u8
      - `num_ref_idx_l1_active_minus1`
      - 若未设置 num_ref_idx_active_override_flag，本字段必须设为
        num_ref_idx_l1_default_active_minus1 的值。
    - - __u8
      - `reserved`
      - 应用程序与驱动必须将本字段置为零。
    - - struct `v4l2_h264_reference`
      - `ref_pic_list0[^32^]`
      - 应用逐切片修改之后的参考图像列表。
    - - struct `v4l2_h264_reference`
      - `ref_pic_list1[^32^]`
      - 应用逐切片修改之后的参考图像列表。
    - - __u32
      - `flags`
      - 参见切片参数标志 <h264_slice_flags>


    \normalsize


`切片参数集标志`


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
    依据 h264 标准第 7.4.3.2 节 “Prediction Weight Table Semantics”（预测加权表语义）
    定义的预测加权表。预测加权表必须在第 7.3.3 节 “Slice header syntax”（切片头语法）
    所述条件下由应用程序传入。



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
      - 索引 0 处的加权因子对应参考列表 0，索引 1 处的加权因子对应参考列表 1。


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

`图像参考`



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `fields`
      - 指定该图像如何被引用。参见参考字段 <h264_ref_fields>
    - - __u8
      - `index`
      - 指向 `v4l2_ctrl_h264_decode_params`.dpb 数组的索引。


`参考字段`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_TOP_FIELD_REF`
      - 0x1
      - 字段对中顶场用于短期参考。
    - - `V4L2_H264_BOTTOM_FIELD_REF`
      - 0x2
      - 字段对中底场用于短期参考。
    - - `V4L2_H264_FRAME_REF`
      - 0x3
      - 帧（或顶/底场，若其为字段对）用于短期参考。


    \normalsize

`V4L2_CID_STATELESS_H264_DECODE_PARAMS (struct)`
    指定与相应 H264 切片数据关联的解码参数（从码流中提取）。其中包含配置 H264
    无状态硬件解码流水线所需的参数。码流参数依据 h264 标准定义。除非有明确注释
    另有说明，否则进一步的文档请参考上述规范。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_h264_dpb_entry`
      - `dpb[^16^]`
      -
    - - __u16
      - `nal_ref_idc`
      - 来自 NAL 单元头的 NAL 参考 ID 值。
    - - __u16
      - `frame_num`
      -
    - - __s32
      - `top_field_order_cnt`
      - 编码顶场的图像顺序计数（Picture Order Count）。
    - - __s32
      - `bottom_field_order_cnt`
      - 编码底场的图像顺序计数。
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
      - dec_ref_pic_marking() 语法元素的大小（以比特计）。
    - - __u32
      - `pic_order_cnt_bit_size`
      - 与图像顺序计数相关的语法元素的合并大小（以比特计）：pic_order_cnt_lsb、
        delta_pic_order_cnt_bottom、delta_pic_order_cnt0 以及 delta_pic_order_cnt1。
    - - __u32
      - `slice_group_change_cycle`
      -
    - - __u32
      - `reserved`
      - 应用程序与驱动必须将本字段置为零。
    - - __u32
      - `flags`
      - 参见解码参数标志 <h264_decode_params_flags>


    \normalsize


`解码参数标志`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_DECODE_PARAM_FLAG_IDR_PIC`
      - 0x00000001
      - 该图像为 IDR 图像。
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
      - 用作参考的 V4L2 捕获缓冲区的时间戳，与 B 帧和 P 帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u32
      - `pic_num`
      - 对于短期参考，本字段必须与推导值 PicNum (8-28) 一致；对于长期参考，必须与推导值
        LongTermPicNum (8-29) 一致。解码帧（而非场）时，pic_num 与 FrameNumWrap 相同。
    - - __u16
      - `frame_num`
      - 对于短期参考，本字段必须与切片头语法中的 frame_num 值一致（驱动会在需要时对该值
        进行环绕处理）。对于长期参考，本字段必须设为 dec_ref_pic_marking() 语法中描述的
        long_term_frame_idx 的值。
    - - __u8
      - `fields`
      - 指定该 DPB 条目如何被引用。参见参考字段 <h264_ref_fields>
    - - __u8
      - `reserved[^5^]`
      - 应用程序与驱动必须将本字段置为零。
    - - __s32
      - `top_field_order_cnt`
      -
    - - __s32
      - `bottom_field_order_cnt`
      -
    - - __u32
      - `flags`
      - 参见 DPB 条目标志 <h264_dpb_flags>


    \normalsize


`DPB 条目标志`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_H264_DPB_ENTRY_FLAG_VALID`
      - 0x00000001
      - 该 DPB 条目有效（非空），应当予以考虑。
    - - `V4L2_H264_DPB_ENTRY_FLAG_ACTIVE`
      - 0x00000002
      - 该 DPB 条目用作参考。
    - - `V4L2_H264_DPB_ENTRY_FLAG_LONG_TERM`
      - 0x00000004
      - 该 DPB 条目用作长期参考。
    - - `V4L2_H264_DPB_ENTRY_FLAG_FIELD`
      - 0x00000008
      - 该 DPB 条目为单个场或互补场对。


    \normalsize

`V4L2_CID_STATELESS_H264_DECODE_MODE (enum)`
    指定要使用的解码模式。目前提供基于切片和基于帧的解码，但后续可能会新增其他模式。
    该控件用作 V4L2_PIX_FMT_H264_SLICE 像素格式的修饰符。支持 V4L2_PIX_FMT_H264_SLICE
    的应用程序必须设置该控件，以指定缓冲区所期望的解码模式。
    驱动可能根据其所支持的能力，暴露单个或多个解码模式。



    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_H264_DECODE_MODE_SLICE_BASED`
      - 0
      - 以切片粒度进行解码。OUTPUT 缓冲区必须包含单个切片。选择该模式时，
        必须设置 `V4L2_CID_STATELESS_H264_SLICE_PARAMS` 控件。当多个切片构成一个帧时，
        需要使用 `V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF` 标志。
    - - `V4L2_STATELESS_H264_DECODE_MODE_FRAME_BASED`
      - 1
      - 以帧粒度进行解码。OUTPUT 缓冲区必须包含解码该帧所需的全部切片，
        并且必须同时包含两个场。该模式由在硬件中解析切片头的设备支持。选择该模式时，
        不应设置 `V4L2_CID_STATELESS_H264_SLICE_PARAMS` 控件。


    \normalsize

`V4L2_CID_STATELESS_H264_START_CODE (enum)`
    指定每个切片所期望的 H264 切片起始码。该控件用作 V4L2_PIX_FMT_H264_SLICE 像素格式
    的修饰符。支持 V4L2_PIX_FMT_H264_SLICE 的应用程序必须设置该控件，以指定缓冲区所期望的
    起始码。驱动可能根据其所支持的能力，暴露单个或多个起始码。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       4 1 4

    - - `V4L2_STATELESS_H264_START_CODE_NONE`
      - 0
      - 选择该值表示 H264 切片不带任何起始码地传递给驱动。码流数据应遵循 h264 7.3.1
        NAL 单元语法，因此在需要时会包含仿真预防字节（emulation prevention bytes）。
    - - `V4L2_STATELESS_H264_START_CODE_ANNEX_B`
      - 1
      - 选择该值表示期望 H264 切片以 Annex B 起始码作为前缀。依据 h264，有效的起始码可以是
        3 字节的 0x000001 或 4 字节的 0x00000001。


    \normalsize


`V4L2_CID_STATELESS_FWHT_PARAMS (struct)`
    指定与相应 FWHT 数据关联的 FWHT（快速 Walsh-Hadamard 变换）参数（从码流中提取）。
    其中包含配置 FWHT 无状态硬件解码流水线所需的参数。该编解码器专用于 vicodec 测试驱动。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `backward_ref_ts`
      - 用作后向参考的 V4L2 捕获缓冲区的时间戳，与 P 帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u32
      - `version`
      - 编解码器版本。设为 `V4L2_FWHT_VERSION`。
    - - __u32
      - `width`
      - 帧的宽度。
    - - __u32
      - `height`
      - 帧的高度。
    - - __u32
      - `flags`
      - 帧的标志，参见 fwht-flags。
    - - __u32
      - `colorspace`
      - 帧的色彩空间，取自枚举 `v4l2_colorspace`。
    - - __u32
      - `xfer_func`
      - 传输函数，取自枚举 `v4l2_xfer_func`。
    - - __u32
      - `ycbcr_enc`
      - Y'CbCr 编码，取自枚举 `v4l2_ycbcr_encoding`。
    - - __u32
      - `quantization`
      - 量化范围，取自枚举 `v4l2_quantization`。


    \normalsize


## FWHT 标志



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FWHT_FL_IS_INTERLACED`
      - 0x00000001
      - 若设置，表示为隔行格式。
    - - `V4L2_FWHT_FL_IS_BOTTOM_FIRST`
      - 0x00000002
      - 若设置，表示为底场优先（NTSC）的隔行格式。
    - - `V4L2_FWHT_FL_IS_ALTERNATE`
      - 0x00000004
      - 若设置，表示每个“帧”仅包含一个场。
    - - `V4L2_FWHT_FL_IS_BOTTOM_FIELD`
      - 0x00000008
      - 若设置了 V4L2_FWHT_FL_IS_ALTERNATE，则本标志在该“帧”为底场时设置，
        否则为顶场。
    - - `V4L2_FWHT_FL_LUMA_IS_UNCOMPRESSED`
      - 0x00000010
      - 若设置，表示 Y'（亮度）平面未经压缩。
    - - `V4L2_FWHT_FL_CB_IS_UNCOMPRESSED`
      - 0x00000020
      - 若设置，表示 Cb 平面未经压缩。
    - - `V4L2_FWHT_FL_CR_IS_UNCOMPRESSED`
      - 0x00000040
      - 若设置，表示 Cr 平面未经压缩。
    - - `V4L2_FWHT_FL_CHROMA_FULL_HEIGHT`
      - 0x00000080
      - 若设置，表示色度平面与亮度平面高度相同，否则色度平面高度为亮度平面的一半。
    - - `V4L2_FWHT_FL_CHROMA_FULL_WIDTH`
      - 0x00000100
      - 若设置，表示色度平面与亮度平面宽度相同，否则色度平面宽度为亮度平面的一半。
    - - `V4L2_FWHT_FL_ALPHA_IS_UNCOMPRESSED`
      - 0x00000200
      - 若设置，表示 alpha 平面未经压缩。
    - - `V4L2_FWHT_FL_I_FRAME`
      - 0x00000400
      - 若设置，表示为 I 帧。
    - - `V4L2_FWHT_FL_COMPONENTS_NUM_MSK`
      - 0x00070000
      - 颜色分量数减一。
    - - `V4L2_FWHT_FL_PIXENC_MSK`
      - 0x00180000
      - 像素编码的掩码。
    - - `V4L2_FWHT_FL_PIXENC_YUV`
      - 0x00080000
      - 若设置，表示像素编码为 YUV。
    - - `V4L2_FWHT_FL_PIXENC_RGB`
      - 0x00100000
      - 若设置，表示像素编码为 RGB。
    - - `V4L2_FWHT_FL_PIXENC_HSV`
      - 0x00180000
      - 若设置，表示像素编码为 HSV。


    \normalsize


`V4L2_CID_STATELESS_VP8_FRAME (struct)`
    指定与相应 VP8 已解析帧数据关联的帧参数。其中包含配置 VP8 无状态硬件解码流水线
    所需的参数。码流参数依据 vp8 标准定义。



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_vp8_segment`
      - `segment`
      - 包含基于分段的调整元数据的结构体。
    - - struct `v4l2_vp8_loop_filter`
      - `lf`
      - 包含环路滤波器等级调整元数据的结构体。
    - - struct `v4l2_vp8_quantization`
      - `quant`
      - 包含 VP8 反量化索引元数据的结构体。
    - - struct `v4l2_vp8_entropy`
      - `entropy`
      - 包含 VP8 熵编码器概率元数据的结构体。
    - - struct `v4l2_vp8_entropy_coder_state`
      - `coder_state`
      - 包含 VP8 熵编码器状态的结构体。
    - - __u16
      - `width`
      - 帧的宽度。所有帧都必须设置。
    - - __u16
      - `height`
      - 帧的高度。所有帧都必须设置。
    - - __u8
      - `horizontal_scale`
      - 水平缩放因子。
    - - __u8
      - `vertical_scale`
      - 垂直缩放因子。
    - - __u8
      - `version`
      - 码流版本。
    - - __u8
      - `prob_skip_false`
      - 表示宏块未被跳过的概率。
    - - __u8
      - `prob_intra`
      - 表示宏块进行帧内预测的概率。
    - - __u8
      - `prob_last`
      - 表示帧间预测中使用上一参考帧的概率。
    - - __u8
      - `prob_gf`
      - 表示帧间预测中使用黄金参考帧的概率。
    - - __u8
      - `num_dct_parts`
      - DCT 系数分区的数量。必须为 1、2、4 或 8 之一。
    - - __u32
      - `first_part_size`
      - 第一个分区（即控制分区）的大小。
    - - __u32
      - `first_part_header_bits`
      - 第一个分区头部部分的大小（以比特计）。
    - - __u32
      - `dct_part_sizes[^8^]`
      - DCT 系数的大小。
    - - __u64
      - `last_frame_ts`
      - 用作上一参考帧的 V4L2 捕获缓冲区的时间戳，与帧间编码帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u64
      - `golden_frame_ts`
      - 用作上一参考帧的 V4L2 捕获缓冲区的时间戳，与帧间编码帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u64
      - `alt_frame_ts`
      - 用作备用参考帧的 V4L2 捕获缓冲区的时间戳，与帧间编码帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u64
      - `flags`
      - 参见帧标志 <vp8_frame_flags>


    \normalsize


`帧标志`



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_FRAME_FLAG_KEY_FRAME`
      - 0x01
      - 表示该帧是否为关键帧。
    - - `V4L2_VP8_FRAME_FLAG_EXPERIMENTAL`
      - 0x02
      - 实验性码流。
    - - `V4L2_VP8_FRAME_FLAG_SHOW_FRAME`
      - 0x04
      - 显示帧标志，表示该帧是否用于显示。
    - - `V4L2_VP8_FRAME_FLAG_MB_NO_SKIP_COEFF`
      - 0x08
      - 启用/禁用跳过无非零系数的宏块。
    - - `V4L2_VP8_FRAME_FLAG_SIGN_BIAS_GOLDEN`
      - 0x10
      - 引用黄金帧时运动矢量的符号。
    - - `V4L2_VP8_FRAME_FLAG_SIGN_BIAS_ALT`
      - 0x20
      - 引用备用帧时运动矢量的符号。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `range`
      - “Range”的编码器状态值。
    - - __u8
      - `value`
      - “Value”的编码器状态值。
    - - __u8
      - `bit_count`
      - 剩余的比特数。
    - - __u8
      - `padding`
      - 应用程序与驱动必须将本字段置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `quant_update[^4^]`
      - 有符号量化器值更新。
    - - __s8
      - `lf_update[^4^]`
      - 有符号环路滤波器等级值更新。
    - - __u8
      - `segment_probs[^3^]`
      - 分段概率。
    - - __u8
      - `padding`
      - 应用程序与驱动必须将本字段置为零。
    - - __u32
      - `flags`
      - 参见分段标志 <vp8_segment_flags>


`分段标志`


    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_SEGMENT_FLAG_ENABLED`
      - 0x01
      - 启用/禁用基于分段的调整。
    - - `V4L2_VP8_SEGMENT_FLAG_UPDATE_MAP`
      - 0x02
      - 表示本帧是否更新宏块分段映射。
    - - `V4L2_VP8_SEGMENT_FLAG_UPDATE_FEATURE_DATA`
      - 0x04
      - 表示本帧是否更新分段特征数据。
    - - `V4L2_VP8_SEGMENT_FLAG_DELTA_VALUE_MODE`
      - 0x08
      - 若设置，分段特征数据模式为 delta-value（差值）；若清除，则为 absolute-value
        （绝对值）。


    \normalsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `ref_frm_delta[^4^]`
      - 参考调整（有符号）差值。
    - - __s8
      - `mb_mode_delta[^4^]`
      - 宏块预测模式调整（有符号）差值。
    - - __u8
      - `sharpness_level`
      - 锐度等级。
    - - __u8
      - `level`
      - 滤波器等级。
    - - __u16
      - `padding`
      - 应用程序与驱动必须将本字段置为零。
    - - __u32
      - `flags`
      - 参见环路滤波器标志 <vp8_loop_filter_flags>


`环路滤波器标志`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP8_LF_ADJ_ENABLE`
      - 0x01
      - 启用/禁用宏块级环路滤波器调整。
    - - `V4L2_VP8_LF_DELTA_UPDATE`
      - 0x02
      - 表示调整中所使用的差值是否更新。
    - - `V4L2_VP8_LF_FILTER_TYPE_SIMPLE`
      - 0x04
      - 若设置，表示滤波器类型为 simple（简单型）；若清除，则为 normal（普通型）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `y_ac_qi`
      - 亮度 AC 系数表索引。
    - - __s8
      - `y_dc_delta`
      - 亮度 DC 差值。
    - - __s8
      - `y2_dc_delta`
      - Y2 块 DC 差值。
    - - __s8
      - `y2_ac_delta`
      - Y2 块 AC 差值。
    - - __s8
      - `uv_dc_delta`
      - 色度 DC 差值。
    - - __s8
      - `uv_ac_delta`
      - 色度 AC 差值。
    - - __u16
      - `padding`
      - 应用程序与驱动必须将本字段置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `coeff_probs[^4^][^8^][^3^][^11^]`
      - 系数更新概率。
    - - __u8
      - `y_mode_probs[^4^]`
      - 亮度模式更新概率。
    - - __u8
      - `uv_mode_probs[^3^]`
      - 色度模式更新概率。
    - - __u8
      - `mv_probs[^2^][^19^]`
      - MV 解码更新概率。
    - - __u8
      - `padding[^3^]`
      - 应用程序与驱动必须将本字段置为零。


`V4L2_CID_STATELESS_MPEG2_SEQUENCE (struct)`
    指定与相应 MPEG-2 切片数据关联的序列参数（从码流中提取）。其中包含与 mpeg2part2
    规范中序列头（sequence header）和序列扩展（sequence extension）部分语法元素对应的字段。



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `horizontal_size`
      - 帧亮度分量可显示部分的宽度。
    - - __u16
      - `vertical_size`
      - 帧亮度分量可显示部分的高度。
    - - __u32
      - `vbv_buffer_size`
      - 用于计算视频缓冲校验器（video buffering verifier）所需大小，以比特定义为：
        16 * 1024 * vbv_buffer_size。
    - - __u16
      - `profile_and_level_indication`
      - 从码流中提取的当前档次与级别指示。
    - - __u8
      - `chroma_format`
      - 色度子采样格式（1：4:2:0，2：4:2:2，3：4:4:4）。
    - - __u8
      - `flags`
      - 参见 MPEG-2 序列标志 <mpeg2_sequence_flags>。


`MPEG-2 序列标志`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_MPEG2_SEQ_FLAG_PROGRESSIVE`
      - 0x01
      - 指示该序列的所有帧均为逐行（progressive）而非隔行（interlaced）。


    \normalsize

`V4L2_CID_STATELESS_MPEG2_PICTURE (struct)`
    指定与相应 MPEG-2 切片数据关联的图像参数（从码流中提取）。其中包含与 mpeg2part2
    规范中图像头（picture header）和图像编码扩展（picture coding extension）部分语法元素
    对应的字段。



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `backward_ref_ts`
      - 用作后向参考的 V4L2 捕获缓冲区的时间戳，与 B 帧和 P 帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u64
      - `forward_ref_ts`
      - 用作前向参考的 V4L2 捕获缓冲区的时间戳，与 B 帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u32
      - `flags`
      - 参见 MPEG-2 图像标志 <mpeg2_picture_flags>。
    - - __u8
      - `f_code[^2^][^2^]`
      - 运动矢量码。
    - - __u8
      - `picture_coding_type`
      - 当前切片所覆盖帧的图像编码类型（V4L2_MPEG2_PIC_CODING_TYPE_I、
        V4L2_MPEG2_PIC_CODING_TYPE_P 或 V4L2_MPEG2_PIC_CODING_TYPE_B）。
    - - __u8
      - `picture_structure`
      - 图像结构（1：隔行顶场，2：隔行底场，3：逐行帧）。
    - - __u8
      - `intra_dc_precision`
      - 离散余弦变换（DCT）的精度（0：8 位精度，1：9 位精度，2：10 位精度，3：11 位精度）。
    - - __u8
      - `reserved[^5^]`
      - 应用程序与驱动必须将本字段置为零。


`MPEG-2 图像标志`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_MPEG2_PIC_FLAG_TOP_FIELD_FIRST`
      - 0x00000001
      - 若设置且为隔行码流，则先输出顶场。
    - - `V4L2_MPEG2_PIC_FLAG_FRAME_PRED_DCT`
      - 0x00000002
      - 若设置，则仅使用帧 DCT 与帧预测。
    - - `V4L2_MPEG2_PIC_FLAG_CONCEALMENT_MV`
      - 0x00000004
      - 若设置，则为帧内宏块编码运动矢量。
    - - `V4L2_MPEG2_PIC_FLAG_Q_SCALE_TYPE`
      - 0x00000008
      - 该标志影响反量化过程。
    - - `V4L2_MPEG2_PIC_FLAG_INTRA_VLC`
      - 0x00000010
      - 该标志影响变换系数数据的解码。
    - - `V4L2_MPEG2_PIC_FLAG_ALT_SCAN`
      - 0x00000020
      - 该标志影响变换系数数据的解码。
    - - `V4L2_MPEG2_PIC_FLAG_REPEAT_FIRST`
      - 0x00000040
      - 该标志影响逐行帧的解码过程。
    - - `V4L2_MPEG2_PIC_FLAG_PROGRESSIVE`
      - 0x00000080
      - 指示当前帧是否为逐行。


    \normalsize

`V4L2_CID_STATELESS_MPEG2_QUANTISATION (struct)`
    以之字形扫描顺序指定与相应 MPEG-2 切片数据关联的量化矩阵。该控件由内核初始化为
    矩阵的默认值。若码流传输了用户自定义的量化矩阵加载，则应用程序应使用该控件。
    若需要重置量化矩阵（例如在序列头处），应用程序还应设置该控件以加载默认值。
    该过程由规范第 6.3.7 节 “Quant matrix extension”（量化矩阵扩展）规定。



    \small

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `intra_quantiser_matrix[^64^]`
      - 帧内编码帧的量化矩阵系数，按之字形扫描顺序。它对亮度和色度分量均相关，
        但在非 4:2:0 的 YUV 格式下，可被色度专用矩阵取代。
    - - __u8
      - `non_intra_quantiser_matrix[^64^]`
      - 非帧内编码帧的量化矩阵系数，按之字形扫描顺序。它对亮度和色度分量均相关，
        但在非 4:2:0 的 YUV 格式下，可被色度专用矩阵取代。
    - - __u8
      - `chroma_intra_quantiser_matrix[^64^]`
      - 帧内编码帧色度分量的量化矩阵系数，按之字形扫描顺序。仅与非 4:2:0 的 YUV 格式相关。
    - - __u8
      - `chroma_non_intra_quantiser_matrix[^64^]`
      - 非帧内编码帧色度分量的量化矩阵系数，按之字形扫描顺序。仅与非 4:2:0 的 YUV 格式相关。


    \normalsize


`V4L2_CID_STATELESS_VP9_COMPRESSED_HDR (struct)`
    存储从当前压缩帧头解析得到的 VP9 概率更新。数组元素中的零值表示不更新相应的概率。
    与运动矢量相关的更新包含新值或零。所有其他更新包含经 inv_map_table[] 转换后的值
    （参见 vp9 规范 6.3.5 节）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `tx_mode`
      - 指定 TX 模式。更多细节参见 TX 模式 <vp9_tx_mode>。
    - - __u8
      - `tx8[^2^][^1^]`
      - TX 8x8 概率差值。
    - - __u8
      - `tx16[^2^][^2^]`
      - TX 16x16 概率差值。
    - - __u8
      - `tx32[^2^][^3^]`
      - TX 32x32 概率差值。
    - - __u8
      - `coef[^4^][^2^][^2^][^6^][^6^][^3^]`
      - 系数概率差值。
    - - __u8
      - `skip[^3^]`
      - 跳过概率差值。
    - - __u8
      - `inter_mode[^7^][^3^]`
      - 帧间预测模式概率差值。
    - - __u8
      - `interp_filter[^4^][^2^]`
      - 插值滤波器概率差值。
    - - __u8
      - `is_inter[^4^]`
      - 是否为帧间块概率差值。
    - - __u8
      - `comp_mode[^5^]`
      - 复合预测模式概率差值。
    - - __u8
      - `single_ref[^5^][^2^]`
      - 单一参考概率差值。
    - - __u8
      - `comp_ref[^5^]`
      - 复合参考概率差值。
    - - __u8
      - `y_mode[^4^][^9^]`
      - Y 预测模式概率差值。
    - - __u8
      - `uv_mode[^10^][^9^]`
      - UV 预测模式概率差值。
    - - __u8
      - `partition[^16^][^3^]`
      - 分区概率差值。
    - - __u8
      - `mv.joint[^3^]`
      - 运动矢量联合概率差值。
    - - __u8
      - `mv.sign[^2^]`
      - 运动矢量符号概率差值。
    - - __u8
      - `mv.classes[^2^][^10^]`
      - 运动矢量类别概率差值。
    - - __u8
      - `mv.class0_bit[^2^]`
      - 运动矢量 class0 比特概率差值。
    - - __u8
      - `mv.bits[^2^][^10^]`
      - 运动矢量比特概率差值。
    - - __u8
      - `mv.class0_fr[^2^][^2^][^3^]`
      - 运动矢量 class0 分数比特概率差值。
    - - __u8
      - `mv.fr[^2^][^3^]`
      - 运动矢量分数比特概率差值。
    - - __u8
      - `mv.class0_hp[^2^]`
      - 运动矢量 class0 高精度分数比特概率差值。
    - - __u8
      - `mv.hp[^2^]`
      - 运动矢量高精度分数比特概率差值。


`TX 模式`

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_TX_MODE_ONLY_4X4`
      - 0
      - 变换尺寸为 4x4。
    - - `V4L2_VP9_TX_MODE_ALLOW_8X8`
      - 1
      - 变换尺寸最大可为 8x8。
    - - `V4L2_VP9_TX_MODE_ALLOW_16X16`
      - 2
      - 变换尺寸最大可为 16x16。
    - - `V4L2_VP9_TX_MODE_ALLOW_32X32`
      - 3
      - 变换尺寸最大可为 32x32。
    - - `V4L2_VP9_TX_MODE_SELECT`
      - 4
      - 码流中包含每个块的变换尺寸。

参见 vp9 规范 “7.3.1 Tx mode semantics”（Tx 模式语义）一节获取更多细节。

`V4L2_CID_STATELESS_VP9_FRAME (struct)`
    指定与相应 VP9 帧解码请求关联的帧参数。其中包含配置 VP9 无状态硬件解码流水线
    所需的参数。码流参数依据 vp9 标准定义。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_vp9_loop_filter`
      - `lf`
      - 环路滤波器参数。更多细节参见结构体 `v4l2_vp9_loop_filter`。
    - - struct `v4l2_vp9_quantization`
      - `quant`
      - 量化参数。更多细节参见 `v4l2_vp9_quantization`。
    - - struct `v4l2_vp9_segmentation`
      - `seg`
      - 分段参数。更多细节参见 `v4l2_vp9_segmentation`。
    - - __u32
      - `flags`
      - V4L2_VP9_FRAME_FLAG_* 标志的组合。参见帧标志 <vp9_frame_flags>。
    - - __u16
      - `compressed_header_size`
      - 压缩头部的大小（字节）。
    - - __u16
      - `uncompressed_header_size`
      - 未压缩头部的大小（字节）。
    - - __u16
      - `frame_width_minus_1`
      - 加 1 得到以像素表示的帧宽度。参见 vp9 规范第 7.2.3 节。
    - - __u16
      - `frame_height_minus_1`
      - 加 1 得到以像素表示的帧高度。参见 vp9 规范第 7.2.3 节。
    - - __u16
      - `render_width_minus_1`
      - 加 1 得到期望的渲染宽度（以像素表示）。该值不用于解码过程，但可能被硬件缩放器
        用于准备可供扫描输出（scanout）的帧。参见 vp9 规范第 7.2.4 节。
    - - __u16
      - render_height_minus_1
      - 加 1 得到期望的渲染高度（以像素表示）。该值不用于解码过程，但可能被硬件缩放器
        用于准备可供扫描输出的帧。参见 vp9 规范第 7.2.4 节。
    - - __u64
      - `last_frame_ts`
      - “last”参考缓冲区的时间戳。该时间戳引用 struct `v4l2_buffer` 中的 `timestamp`
        字段。使用 `v4l2_timeval_to_ns()` 函数将 struct `v4l2_buffer` 中的
        struct `timeval` 转换为 __u64。
    - - __u64
      - `golden_frame_ts`
      - “golden”参考缓冲区的时间戳。该时间戳引用 struct `v4l2_buffer` 中的 `timestamp`
        字段。使用 `v4l2_timeval_to_ns()` 函数将 struct `v4l2_buffer` 中的
        struct `timeval` 转换为 __u64。
    - - __u64
      - `alt_frame_ts`
      - “alt”参考缓冲区的时间戳。该时间戳引用 struct `v4l2_buffer` 中的 `timestamp`
        字段。使用 `v4l2_timeval_to_ns()` 函数将 struct `v4l2_buffer` 中的
        struct `timeval` 转换为 __u64。
    - - __u8
      - `ref_frame_sign_bias`
      - 位域，指定是否为给定参考帧设置了符号偏置。更多细节参见参考帧符号偏置
        <vp9_ref_frame_sign_bias>。
    - - __u8
      - `reset_frame_context`
      - 指定是否应将帧上下文重置为默认值。更多细节参见重置帧上下文
        <vp9_reset_frame_context>。
    - - __u8
      - `frame_context_idx`
      - 应被使用/更新的帧上下文。
    - - __u8
      - `profile`
      - VP9 档次（profile）。可以为 0、1、2 或 3。
    - - __u8
      - `bit_depth`
      - 分量位深（比特）。可以为 8、10 或 12。注意并非所有档次都支持 10 和/或 12 位深。
    - - __u8
      - `interpolation_filter`
      - 指定用于执行帧间预测所选择的滤波器。更多细节参见插值滤波器
        <vp9_interpolation_filter>。
    - - __u8
      - `tile_cols_log2`
      - 指定每个 tile 宽度的以 2 为底的对数（宽度以 8x8 块为单位度量）。必须小于或等于 6。
    - - __u8
      - `tile_rows_log2`
      - 指定每个 tile 高度的以 2 为底的对数（高度以 8x8 块为单位度量）。
    - - __u8
      - `reference_mode`
      - 指定要使用的帧间预测类型。更多细节参见参考模式 <vp9_reference_mode>。注意该值
        是作为压缩头部解析过程的一部分推导出来的，因此本应属于 :c:type:
        `v4l2_ctrl_vp9_compressed_hdr` 可选控件。若驱动不需要压缩头部，将本值设为
        零是安全的。
    - - __u8
      - `reserved[^7^]`
      - 应用程序与驱动必须将本字段置为零。


    \normalsize


`帧标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_FRAME_FLAG_KEY_FRAME`
      - 0x001
      - 该帧为关键帧。
    - - `V4L2_VP9_FRAME_FLAG_SHOW_FRAME`
      - 0x002
      - 该帧应被显示。
    - - `V4L2_VP9_FRAME_FLAG_ERROR_RESILIENT`
      - 0x004
      - 解码应具有错误韧性。
    - - `V4L2_VP9_FRAME_FLAG_INTRA_ONLY`
      - 0x008
      - 该帧不参考其他帧。
    - - `V4L2_VP9_FRAME_FLAG_ALLOW_HIGH_PREC_MV`
      - 0x010
      - 该帧可以使用高精度运动矢量。
    - - `V4L2_VP9_FRAME_FLAG_REFRESH_FRAME_CTX`
      - 0x020
      - 解码后应当更新帧上下文。
    - - `V4L2_VP9_FRAME_FLAG_PARALLEL_DEC_MODE`
      - 0x040
      - 使用了并行解码。
    - - `V4L2_VP9_FRAME_FLAG_X_SUBSAMPLING`
      - 0x080
      - 启用了垂直子采样。
    - - `V4L2_VP9_FRAME_FLAG_Y_SUBSAMPLING`
      - 0x100
      - 启用了水平子采样。
    - - `V4L2_VP9_FRAME_FLAG_COLOR_RANGE_FULL_SWING`
      - 0x200
      - 使用了完整的 UV 范围。


`参考帧符号偏置`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SIGN_BIAS_LAST`
      - 0x1
      - 为 last 参考帧设置了符号偏置。
    - - `V4L2_VP9_SIGN_BIAS_GOLDEN`
      - 0x2
      - 为 golden 参考帧设置了符号偏置。
    - - `V4L2_VP9_SIGN_BIAS_ALT`
      - 0x2
      - 为 alt 参考帧设置了符号偏置。


`重置帧上下文`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_RESET_FRAME_CTX_NONE`
      - 0
      - 不重置任何帧上下文。
    - - `V4L2_VP9_RESET_FRAME_CTX_SPEC`
      - 1
      - 重置由 `v4l2_ctrl_vp9_frame`.frame_context_idx 指向的帧上下文。
    - - `V4L2_VP9_RESET_FRAME_CTX_ALL`
      - 2
      - 重置所有帧上下文。

更多细节参见 vp9 规范 “7.2 Uncompressed header semantics”（未压缩头部语义）一节。


`插值滤波器`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP`
      - 0
      - 八抽头滤波器。
    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP_SMOOTH`
      - 1
      - 八抽头平滑滤波器。
    - - `V4L2_VP9_INTERP_FILTER_EIGHTTAP_SHARP`
      - 2
      - 八抽头锐利滤波器。
    - - `V4L2_VP9_INTERP_FILTER_BILINEAR`
      - 3
      - 双线性滤波器。
    - - `V4L2_VP9_INTERP_FILTER_SWITCHABLE`
      - 4
      - 滤波器选择于块级别发出信号。

更多细节参见 vp9 规范 “7.2.7 Interpolation filter semantics”（插值滤波器语义）一节。


`参考模式`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_REFERENCE_MODE_SINGLE_REFERENCE`
      - 0
      - 表示所有帧间块仅使用单个参考帧来生成运动补偿预测。
    - - `V4L2_VP9_REFERENCE_MODE_COMPOUND_REFERENCE`
      - 1
      - 要求所有帧间块使用复合模式，不允许单参考帧预测。
    - - `V4L2_VP9_REFERENCE_MODE_SELECT`
      - 2
      - 允许每个独立的帧间块在单参考与复合预测模式之间选择。

更多细节参见 vp9 规范 “7.3.6 Frame reference mode semantics”（帧参考模式语义）一节。


编码量化参数。更多细节参见 vp9 规范 “7.2.10 Segmentation params syntax”（分段参数语法）一节。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `feature_data[^8^][^4^]`
      - 附带在每个特征上的数据。仅当特征被启用时数据条目才有效。该数组应以段编号作为
        第一维（0..7）、以 V4L2_VP9_SEG_* 之一作为第二维进行索引。参见分段特征 ID
        <vp9_segment_feature>。
    - - __u8
      - `feature_enabled[^8^]`
      - 位掩码，定义每个段中启用了哪些特征。每个段的值为 V4L2_VP9_SEGMENT_FEATURE_ENABLED(id)
        值的组合，其中 id 为 V4L2_VP9_SEG_* 之一。参见分段特征 ID <vp9_segment_feature>。
    - - __u8
      - `tree_probs[^7^]`
      - 指定解码 Segment-ID 时要使用的概率值。更多细节参见 vp9 规范的 “5.15 Segmentation map”
        （分段映射）一节。
    - - __u8
      - `pred_probs[^3^]`
      - 指定解码 Predicted-Segment-ID 时要使用的概率值。更多细节参见 vp9 规范的
        “6.4.14 Get segment id syntax”（获取段 ID 语法）一节。
    - - __u8
      - `flags`
      - V4L2_VP9_SEGMENTATION_FLAG_* 标志的组合。参见分段标志 <vp9_segmentation_flags>。
    - - __u8
      - `reserved[^5^]`
      - 应用程序与驱动必须将本字段置为零。


`分段特征 ID`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SEG_LVL_ALT_Q`
      - 0
      - 量化器分段特征。
    - - `V4L2_VP9_SEG_LVL_ALT_L`
      - 1
      - 环路滤波器分段特征。
    - - `V4L2_VP9_SEG_LVL_REF_FRAME`
      - 2
      - 参考帧分段特征。
    - - `V4L2_VP9_SEG_LVL_SKIP`
      - 3
      - 跳过分段特征。
    - - `V4L2_VP9_SEG_LVL_MAX`
      - 4
      - 分段特征的数量。


`分段标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_SEGMENTATION_FLAG_ENABLED`
      - 0x01
      - 表示该帧使用了分段工具（segmentation tool）。
    - - `V4L2_VP9_SEGMENTATION_FLAG_UPDATE_MAP`
      - 0x02
      - 表示该帧的解码过程中应当更新分段映射。
    - - `V4L2_VP9_SEGMENTATION_FLAG_TEMPORAL_UPDATE`
      - 0x04
      - 表示分段映射的更新是相对于已存在的分段映射编码的。
    - - `V4L2_VP9_SEGMENTATION_FLAG_UPDATE_DATA`
      - 0x08
      - 表示即将为每个段指定新的参数。
    - - `V4L2_VP9_SEGMENTATION_FLAG_ABS_OR_DELTA_UPDATE`
      - 0x10
      - 表示分段参数代表要使用的实际值。


编码量化参数。更多细节参见 VP9 规范 “7.2.9 Quantization params syntax”（量化参数语法）一节。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `base_q_idx`
      - 表示基础帧 qindex。
    - - __s8
      - `delta_q_y_dc`
      - 表示相对 base_q_idx 的 Y DC 量化器。
    - - __s8
      - `delta_q_uv_dc`
      - 表示相对 base_q_idx 的 UV DC 量化器。
    - - __s8
      - `delta_q_uv_ac`
      - 表示相对 base_q_idx 的 UV AC 量化器。
    - - __u8
      - `reserved[^4^]`
      - 应用程序与驱动必须将本字段置为零。


该结构体包含全部与环路滤波器相关的参数。更多细节参见 vp9 规范 “7.2.8 Loop filter semantics”
（环路滤波器语义）一节。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `ref_deltas[^4^]`
      - 包含基于所选参考帧对滤波器等级所需的调整。
    - - __s8
      - `mode_deltas[^2^]`
      - 包含基于所选模式对滤波器等级所需的调整。
    - - __u8
      - `level`
      - 表示环路滤波器强度。
    - - __u8
      - `sharpness`
      - 表示锐度等级。
    - - __u8
      - `flags`
      - V4L2_VP9_LOOP_FILTER_FLAG_* 标志的组合。参见环路滤波器标志 <vp9_loop_filter_flags>。
    - - __u8
      - `reserved[^7^]`
      - 应用程序与驱动必须将本字段置为零。



`环路滤波器标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_VP9_LOOP_FILTER_FLAG_DELTA_ENABLED`
      - 0x1
      - 当设置时，滤波器等级取决于用于预测某个块的模式和参考帧。
    - - `V4L2_VP9_LOOP_FILTER_FLAG_DELTA_UPDATE`
      - 0x2
      - 当设置时，码流包含额外的语法元素，用于指定哪些模式和参考帧的差值需要更新。


`V4L2_CID_STATELESS_HEVC_SPS (struct)`
    指定与相应 HEVC 切片数据关联的序列参数集字段（从码流中提取）。这些码流参数依据
    hevc 标准定义，并在规范的 “7.4.3.2 Sequence parameter set RBSP semantics”
    （序列参数集 RBSP 语义）一节中描述。



    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `video_parameter_set_id`
      - 指定活动 VPS 的 vps_video_parameter_set_id 值，如 H.265 规范
        “7.4.3.2.1 General sequence parameter set RBSP semantics”（通用序列参数集 RBSP 语义）
        一节所述。
    - - __u8
      - `seq_parameter_set_id`
      - 为 SPS 提供一个标识符，供其他语法元素引用，如 H.265 规范
        “7.4.3.2.1 General sequence parameter set RBSP semantics”（通用序列参数集 RBSP 语义）
        一节所述。
    - - __u16
      - `pic_width_in_luma_samples`
      - 指定每幅解码图像的宽度，以亮度样本为单位。
    - - __u16
      - `pic_height_in_luma_samples`
      - 指定每幅解码图像的高度，以亮度样本为单位。
    - - __u8
      - `bit_depth_luma_minus8`
      - 该值加 8 指定亮度数组样本的位深。
    - - __u8
      - `bit_depth_chroma_minus8`
      - 该值加 8 指定色度数组样本的位深。
    - - __u8
      - `log2_max_pic_order_cnt_lsb_minus4`
      - 指定变量 MaxPicOrderCntLsb 的值。
    - - __u8
      - `sps_max_dec_pic_buffering_minus1`
      - 该值加 1 指定编码视频序列（CVS）所需的解码图像缓冲区最大大小。
    - - __u8
      - `sps_max_num_reorder_pics`
      - 表示允许的最大图像数量。
    - - __u8
      - `sps_max_latency_increase_plus1`
      - 用于发信号传递 MaxLatencyPictures，表示在输出顺序上可以位于任意图像之前、并在解码
        顺序上跟随该图像的最大图像数量。
    - - __u8
      - `log2_min_luma_coding_block_size_minus3`
      - 该值加 3 指定最小亮度编码块大小。
    - - __u8
      - `log2_diff_max_min_luma_coding_block_size`
      - 指定最大与最小亮度编码块大小之间的差值。
    - - __u8
      - `log2_min_luma_transform_block_size_minus2`
      - 该值加 2 指定最小亮度变换块大小。
    - - __u8
      - `log2_diff_max_min_luma_transform_block_size`
      - 指定最大与最小亮度变换块大小之间的差值。
    - - __u8
      - `max_transform_hierarchy_depth_inter`
      - 指定以帧间预测模式编码的编码单元的变换单元的最大层级深度。
    - - __u8
      - `max_transform_hierarchy_depth_intra`
      - 指定以帧内预测模式编码的编码单元的变换单元的最大层级深度。
    - - __u8
      - `pcm_sample_bit_depth_luma_minus1`
      - 该值加 1 指定用于表示亮度分量的每个 PCM 样本值的比特数。
    - - __u8
      - `pcm_sample_bit_depth_chroma_minus1`
      - 指定用于表示色度分量的每个 PCM 样本值的比特数。
    - - __u8
      - `log2_min_pcm_luma_coding_block_size_minus3`
      - 加 3 指定编码块的最小大小。
    - - __u8
      - `log2_diff_max_min_pcm_luma_coding_block_size`
      - 指定编码块最大与最小大小之间的差值。
    - - __u8
      - `num_short_term_ref_pic_sets`
      - 指定 SPS 中包含的 st_ref_pic_set() 语法结构的数量。
    - - __u8
      - `num_long_term_ref_pics_sps`
      - 指定在 SPS 中指定的候选长期参考图像的数量。
    - - __u8
      - `chroma_format_idc`
      - 指定色度采样方式。
    - - __u8
      - `sps_max_sub_layers_minus1`
      - 该值加 1 指定时间子层的最大数量。
    - - __u64
      - `flags`
      - 参见序列参数集标志 <hevc_sps_flags>


    \normalsize


`序列参数集标志`


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
    指定与相应 HEVC 切片数据关联的图像参数集字段（从码流中提取）。这些码流参数依据
    hevc 标准定义，并在规范的 “7.4.3.3 Picture parameter set RBSP semantics”
    （图像参数集 RBSP 语义）一节中描述。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `pic_parameter_set_id`
      - 为 PPS 提供一个标识符，供其他语法元素引用。
    - - __u8
      - `num_extra_slice_header_bits`
      - 指定引用该 PPS 的编码图像的切片头 RBSP 中存在的额外切片头比特数量。
    - - __u8
      - `num_ref_idx_l0_default_active_minus1`
      - 该值加 1 指定 num_ref_idx_l0_active_minus1 的推导值。
    - - __u8
      - `num_ref_idx_l1_default_active_minus1`
      - 该值加 1 指定 num_ref_idx_l1_active_minus1 的推导值。
    - - __s8
      - `init_qp_minus26`
      - 该值加 26 指定引用该 PPS 的每个切片的 SliceQp Y 初始值。
    - - __u8
      - `diff_cu_qp_delta_depth`
      - 指定亮度编码树块大小与传达 cu_qp_delta_abs 和 cu_qp_delta_sign_flag 的编码单元的
        最小亮度编码块大小之间的差值。
    - - __s8
      - `pps_cb_qp_offset`
      - 指定对亮度量化参数 Cb 的偏移。
    - - __s8
      - `pps_cr_qp_offset`
      - 指定对亮度量化参数 Cr 的偏移。
    - - __u8
      - `num_tile_columns_minus1`
      - 该值加 1 指定将图像划分成的 tile 列数。
    - - __u8
      - `num_tile_rows_minus1`
      - 该值加 1 指定将图像划分成的 tile 行数。
    - - __u8
      - `column_width_minus1[^20^]`
      - 该值加 1 指定第 i 个 tile 列的宽度，以编码树块为单位。
    - - __u8
      - `row_height_minus1[^22^]`
      - 该值加 1 指定第 i 个 tile 行的高度，以编码树块为单位。
    - - __s8
      - `pps_beta_offset_div2`
      - 指定 beta 的默认去块参数偏移除以 2。
    - - __s8
      - `pps_tc_offset_div2`
      - 指定 tC 的默认去块参数偏移除以 2。
    - - __u8
      - `log2_parallel_merge_level_minus2`
      - 该值加 2 指定变量 Log2ParMrgLevel 的值。
    - - __u8
      - `padding[^4^]`
      - 应用程序与驱动必须将本字段置为零。
    - - __u64
      - `flags`
      - 参见图像参数集标志 <hevc_pps_flags>


`图像参数集标志`


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
      - 指定 PPS 中是否存在去块滤波器控制语法元素。
    - - `V4L2_HEVC_PPS_FLAG_UNIFORM_SPACING`
      - 0x00100000
      - 指定 tile 列边界以及 tile 行边界在图像上均匀分布。


    \normalsize

`V4L2_CID_STATELESS_HEVC_SLICE_PARAMS (struct)`
    指定各种切片特定参数，特别是来自 NAL 单元头、通用切片段头以及码流中加权预测参数部分
    的参数。这些码流参数依据 hevc 标准定义，并在规范的 “7.4.7 General slice segment header
    semantics”（通用切片段头语义）一节中描述。该控件为动态大小的 1 维数组，使用时必须设置
    V4L2_CTRL_FLAG_DYNAMIC_ARRAY 标志。



    \scriptsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `bit_size`
      - 当前切片数据的大小（比特）。
    - - __u32
      - `data_byte_offset`
      - 指向当前切片数据中视频数据的偏移量（字节）。
    - - __u32
      - `num_entry_point_offsets`
      - 指定切片头中入口点偏移语法元素的数量。当驱动支持时，必须设置
        `V4L2_CID_STATELESS_HEVC_ENTRY_POINT_OFFSETS`。
    - - __u8
      - `nal_unit_type`
      - 指定切片的编码类型（B、P 或 I）。
    - - __u8
      - `nuh_temporal_id_plus1`
      - 减 1 指定 NAL 单元的时间标识符。
    - - __u8
      - `slice_type`
      -
	（V4L2_HEVC_SLICE_TYPE_I、V4L2_HEVC_SLICE_TYPE_P 或
	V4L2_HEVC_SLICE_TYPE_B）。
    - - __u8
      - `colour_plane_id`
      - 指定与当前切片关联的色平面。
    - - __s32
      - `slice_pic_order_cnt`
      - 指定图像顺序计数。
    - - __u8
      - `num_ref_idx_l0_active_minus1`
      - 该值加 1 指定可用于解码该切片的参考图像列表 0 的最大参考索引。
    - - __u8
      - `num_ref_idx_l1_active_minus1`
      - 该值加 1 指定可用于解码该切片的参考图像列表 1 的最大参考索引。
    - - __u8
      - `collocated_ref_idx`
      - 指定用于时间运动矢量预测的协同（collocated）图像的参考索引。
    - - __u8
      - `five_minus_max_num_merge_cand`
      - 指定切片所支持的最大合并运动矢量预测候选数，从 5 中减去。
    - - __s8
      - `slice_qp_delta`
      - 指定用于切片中编码块初始的 QpY 值。
    - - __s8
      - `slice_cb_qp_offset`
      - 指定要加到 pps_cb_qp_offset 值上的差值。
    - - __s8
      - `slice_cr_qp_offset`
      - 指定要加到 pps_cr_qp_offset 值上的差值。
    - - __s8
      - `slice_act_y_qp_offset`
      - 指定第 8.6.2 节推导出的量化参数 qP 的亮度偏移。
    - - __s8
      - `slice_act_cb_qp_offset`
      - 指定第 8.6.2 节推导出的量化参数 qP 的 cb 偏移。
    - - __s8
      - `slice_act_cr_qp_offset`
      - 指定第 8.6.2 节推导出的量化参数 qP 的 cr 偏移。
    - - __s8
      - `slice_beta_offset_div2`
      - 指定 beta 的去块参数偏移除以 2。
    - - __s8
      - `slice_tc_offset_div2`
      - 指定 tC 的去块参数偏移除以 2。
    - - __u8
      - `pic_struct`
      - 指示图像应作为帧还是作为一个或多个场显示。
    - - __u32
      - `slice_segment_addr`
      - 指定切片段中第一个编码树块的地址。
    - - __u8
      - `ref_idx_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - L0 参考元素列表，以 DPB 中的索引表示。
    - - __u8
      - `ref_idx_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - L1 参考元素列表，以 DPB 中的索引表示。
    - - __u16
      - `short_term_ref_pic_set_size`
      - 指定短期参考图像集的大小（比特），在规范中描述为 st_ref_pic_set()，包含在切片头或
        SPS 中（第 7.3.6.1 节）。
    - - __u16
      - `long_term_ref_pic_set_size`
      - 指定长期参考图像集的大小（比特），包含在切片头或 SPS 中。即规范第 7.3.6.1 节中
        条件块 if(long_term_ref_pics_present_flag) 内的比特数。
    - - __u8
      - `padding`
      - 应用程序与驱动必须将本字段置为零。
    - - struct `v4l2_hevc_pred_weight_table`
      - `pred_weight_table`
      - 用于帧间图像预测的预测加权系数。
    - - __u64
      - `flags`
      - 参见切片参数标志 <hevc_slice_params_flags>


    \normalsize


`切片参数标志`


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
    指定入口点偏移（字节）。该控件为动态大小数组，入口点偏移的数量由 `elems` 字段报告。
    该码流参数依据 hevc 标准定义，并在规范的 “7.4.7.1 General slice segment header
    semantics”（通用切片段头语义）一节中描述。当一个请求中提交多个切片时，该数组的长度
    必须为请求中所有切片的 num_entry_point_offsets 之和。

`V4L2_CID_STATELESS_HEVC_SCALING_MATRIX (struct)`
    指定用于变换系数缩放过程的 HEVC 缩放矩阵参数。这些矩阵与参数依据 hevc 标准定义，并在
    规范的 “7.4.5 Scaling list data semantics”（缩放列表数据语义）一节中描述。



    \scriptsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `scaling_list_4x4[^6^][^16^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_8x8[^6^][^64^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_16x16[^6^][^64^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_32x32[^2^][^64^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_dc_coef_16x16[^6^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。
    - - __u8
      - `scaling_list_dc_coef_32x32[^2^]`
      - 缩放列表用于变换系数的缩放过程。每个缩放列表中的值按光栅扫描顺序排列。


    \normalsize



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `timestamp`
      - 用作参考的 V4L2 捕获缓冲区的时间戳，与 B 帧和 P 帧配合使用。该时间戳引用
        struct `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()`
        函数将 struct `v4l2_buffer` 中的 struct `timeval` 转换为 __u64。
    - - __u8
      - `flags`
      - 参考帧的长期标志（V4L2_HEVC_DPB_ENTRY_LONG_TERM_REFERENCE）。该标志的设置如 ITU HEVC
        规范 “8.3.2 Decoding process for reference picture set”（参考图像集解码过程）一章所述。
    - - __u8
      - `field_pic`
      - 该参考是场图像还是帧图像。参见 HEVC dpb 场图像标志 <hevc_dpb_field_pic_flags>。
    - - __s32
      - `pic_order_cnt_val`
      - 当前图像的图像顺序计数。
    - - __u8
      - `padding[^2^]`
      - 应用程序与驱动必须将本字段置为零。


    \normalsize


`HEVC dpb 场图像标志`


    \scriptsize

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME`
      - 0
      - （逐行）帧
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_FIELD`
      - 1
      - 顶场
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_FIELD`
      - 2
      - 底场
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM`
      - 3
      - 顶场、底场，按此顺序
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP`
      - 4
      - 底场、顶场，按此顺序
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_BOTTOM_TOP`
      - 5
      - 顶场、底场、顶场重复，按此顺序
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM`
      - 6
      - 底场、顶场、底场重复，按此顺序
    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME_DOUBLING`
      - 7
      - 帧翻倍（Frame doubling）
    - - `V4L2_HEVC_SEI_PIC_STRUCT_FRAME_TRIPLING`
      - 8
      - 帧三倍（Frame tripling）
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_PREVIOUS_BOTTOM`
      - 9
      - 顶场与上一个底场在输出顺序上配对
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_PREVIOUS_TOP`
      - 10
      - 底场与上一个顶场在输出顺序上配对
    - - `V4L2_HEVC_SEI_PIC_STRUCT_TOP_PAIRED_NEXT_BOTTOM`
      - 11
      - 顶场与下一个底场在输出顺序上配对
    - - `V4L2_HEVC_SEI_PIC_STRUCT_BOTTOM_PAIRED_NEXT_TOP`
      - 12
      - 底场与下一个顶场在输出顺序上配对


    \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s8
      - `delta_luma_weight_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 应用于列表 0 亮度预测值的加权因子的差值。
    - - __s8
      - `luma_offset_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 应用于列表 0 亮度预测值的加性偏移。
    - - __s8
      - `delta_chroma_weight_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 应用于列表 0 色度预测值的加权因子的差值。
    - - __s8
      - `chroma_offset_l0[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 应用于列表 0 色度预测值的加性偏移的差值。
    - - __s8
      - `delta_luma_weight_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 应用于列表 1 亮度预测值的加权因子的差值。
    - - __s8
      - `luma_offset_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 应用于列表 1 亮度预测值的加性偏移。
    - - __s8
      - `delta_chroma_weight_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 应用于列表 1 色度预测值的加权因子的差值。
    - - __s8
      - `chroma_offset_l1[V4L2_HEVC_DPB_ENTRIES_NUM_MAX][^2^]`
      - 应用于列表 1 色度预测值的加性偏移的差值。
    - - __u8
      - `luma_log2_weight_denom`
      - 所有亮度加权因子分母的以 2 为底的对数。
    - - __s8
      - `delta_chroma_log2_weight_denom`
      - 所有色度加权因子分母的以 2 为底的对数的差值。
    - - __u8
      - `padding[^6^]`
      - 应用程序与驱动必须将本字段置为零。


    \normalsize

`V4L2_CID_STATELESS_HEVC_DECODE_MODE (enum)`
    指定要使用的解码模式。目前提供基于切片和基于帧的解码，但后续可能会新增其他模式。
    该控件用作 V4L2_PIX_FMT_HEVC_SLICE 像素格式的修饰符。支持 V4L2_PIX_FMT_HEVC_SLICE
    的应用程序必须设置该控件，以指定缓冲区所期望的解码模式。驱动可能根据其所支持的能力，
    暴露单个或多个解码模式。



    \small


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_HEVC_DECODE_MODE_SLICE_BASED`
      - 0
      - 以切片粒度进行解码。OUTPUT 缓冲区必须包含单个切片。
    - - `V4L2_STATELESS_HEVC_DECODE_MODE_FRAME_BASED`
      - 1
      - 以帧粒度进行解码。OUTPUT 缓冲区必须包含解码该帧所需的全部切片。


    \normalsize

`V4L2_CID_STATELESS_HEVC_START_CODE (enum)`
    指定每个 HEVC 切片所期望的切片起始码。该控件用作 V4L2_PIX_FMT_HEVC_SLICE 像素格式的
    修饰符。支持 V4L2_PIX_FMT_HEVC_SLICE 的应用程序必须设置该控件，以指定缓冲区所期望的
    起始码。驱动可能根据其所支持的能力，暴露单个或多个起始码。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_STATELESS_HEVC_START_CODE_NONE`
      - 0
      - 选择该值表示 HEVC 切片不带任何起始码地传递给驱动。码流数据应遵循 hevc 7.3.1.1
        General NAL unit syntax（通用 NAL 单元语法），因此在需要时会包含仿真预防字节。
    - - `V4L2_STATELESS_HEVC_START_CODE_ANNEX_B`
      - 1
      - 选择该值表示期望 HEVC 切片以 Annex B 起始码作为前缀。依据 hevc，有效的起始码可以是
        3 字节的 0x000001 或 4 字节的 0x00000001。


    \normalsize

`V4L2_CID_MPEG_VIDEO_BASELAYER_PRIORITY_ID (integer)`
    为 NAL 单元指定一个优先级标识符，将应用于基础层（base layer）。默认情况下，基础层该值
    设为 0，下一层将被分配优先级 ID 为 1、2、3 等等。视频编码器无法决定要应用于某层的
    优先级 ID，因此必须由客户端提供。这适用于 H264，有效范围为 0 到 63。
    来源：Rec. ITU-T H.264 (06/2019)；G.7.4.1.1、G.8.8.1。

`V4L2_CID_MPEG_VIDEO_LTR_COUNT (integer)`
    指定编码器在任何给定时刻可以保留的长期参考（LTR）帧的最大数量。这适用于 H264 和 HEVC
    编码器。

`V4L2_CID_MPEG_VIDEO_FRAME_LTR_INDEX (integer)`
    设置该控件后，接下来将排队的帧将被标记为长期参考（LTR）帧，并获得该 LTR 索引，索引范围
    从 0 到 LTR_COUNT-1。这适用于 H264 和 HEVC 编码器。来源：Rec. ITU-T H.264 (06/2019)；
    表 7.9。

`V4L2_CID_MPEG_VIDEO_USE_LTR_FRAMES (bitmask)`
    指定用于编码设置该控件后下一个排队帧的长期参考（LTR）帧。这提供一个位掩码，由比特
    [0, LTR_COUNT-1] 组成。这适用于 H264 和 HEVC 编码器。

`V4L2_CID_STATELESS_HEVC_DECODE_PARAMS (struct)`
    指定各种解码参数，特别是所有列表（短期、长期、之前、当前、之后）的参考图像顺序计数
    （POC）以及每个列表的条目数。这些参数依据 hevc 标准定义，并在规范的 “8.3 Slice decoding
    process”（切片解码过程）一节中描述。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s32
      - `pic_order_cnt_val`
      - PicOrderCntVal，如规范 “8.3.1 Decoding process for picture order count”
        （图像顺序计数解码过程）一节所述。
    - - __u16
      - `short_term_ref_pic_set_size`
      - 指定第一个切片的短期参考图像集的大小（比特），该集合在规范中描述为
        st_ref_pic_set()，包含在切片头或 SPS 中（第 7.3.6.1 节）。
    - - __u16
      - `long_term_ref_pic_set_size`
      - 指定第一个切片中包含的长期参考图像集的大小（比特），包含在切片头或 SPS 中。
        即规范第 7.3.6.1 节中条件块 if(long_term_ref_pics_present_flag) 内的比特数。
    - - __u8
      - `num_active_dpb_entries`
      - `dpb` 中的条目数。
    - - __u8
      - `num_poc_st_curr_before`
      - 在当前帧之前的短期集合中的参考图像数量。
    - - __u8
      - `num_poc_st_curr_after`
      - 在当前帧之后的短期集合中的参考图像数量。
    - - __u8
      - `num_poc_lt_curr`
      - 长期集合中的参考图像数量。
    - - __u8
      - `poc_st_curr_before[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocStCurrBefore，如规范 “8.3.2 Decoding process for reference picture set”
        （参考图像集解码过程）一节所述：提供 DPB 数组中当前帧之前的短期参考的索引。
    - - __u8
      - `poc_st_curr_after[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocStCurrAfter，如规范 “8.3.2 Decoding process for reference picture set”
        一节所述：提供 DPB 数组中当前帧之后的短期参考的索引。
    - - __u8
      - `poc_lt_curr[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - PocLtCurr，如规范 “8.3.2 Decoding process for reference picture set”
        一节所述：提供 DPB 数组中长期参考的索引。
    - - __u8
      - `num_delta_pocs_of_ref_rps_idx`
      - 当切片头中 short_term_ref_pic_set_sps_flag 等于 0 时，其值与推导值
        NumDeltaPocs[RefRpsIdx] 相同。它可用于解析切片头中的 RPS 数据，而非使用
        @short_term_ref_pic_set_size 跳过它。当切片头中 short_term_ref_pic_set_sps_flag
        的值等于 1 时，num_delta_pocs_of_ref_rps_idx 应设为 0。
    - - struct `v4l2_hevc_dpb_entry`
      - `dpb[V4L2_HEVC_DPB_ENTRIES_NUM_MAX]`
      - 解码图像缓冲区，存放关于参考帧的元数据。
    - - __u64
      - `flags`
      - 参见解码参数标志 <hevc_decode_params_flags>


`解码参数标志`


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
    `v4l2_ctrl_hevc_sps` 控件的子集。它以长期参考集参数列表对其进行扩展。这些参数依据
    hevc 标准定义，并在规范的 “7.4.3.2.1 General sequence parameter set RBSP semantics”
    （通用序列参数集 RBSP 语义）一节中描述。该控件为动态大小的 1 维数组。当
    num_long_term_ref_pics_sps 为 0，或 `v4l2_ctrl_hevc_sps` 中未设置
    V4L2_HEVC_SPS_FLAG_LONG_TERM_REF_PICS_PRESENT 标志时，数组中的值应被忽略。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u16
      - `lt_ref_pic_poc_lsb_sps`
      - 长期参考图像顺序计数，如规范 “7.4.3.2.1 General sequence parameter set RBSP semantics”
        （通用序列参数集 RBSP 语义）一节所述。
    - - __u16
      - `flags`
      - 参见扩展长期 RPS 标志 <hevc_ext_sps_lt_rps_flags>


`扩展 SPS 长期 RPS 标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_EXT_SPS_LT_RPS_FLAG_USED_LT`
      - 0x00000001
      - 指定长期参考图像是否被使用，见规范 7.4.3.2.1 “General sequence parameter set RBSP
        semantics”（通用序列参数集 RBSP 语义）一节。


`V4L2_CID_STATELESS_HEVC_EXT_SPS_ST_RPS (struct)`
    `v4l2_ctrl_hevc_sps` 控件的子集。它以短期参考集参数列表对其进行扩展。这些参数依据
    hevc 标准定义，并在规范的 “7.4.8 Short-term reference picture set semantics”
    （短期参考图像集语义）一节中描述。该控件为动态大小的 1 维数组。当 num_short_term_ref_pic_sets
    为 0 时，数组中的值应被忽略。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `delta_idx_minus1`
      - 指定与索引比较的 delta 值。详见规范 “7.4.8 Short-term reference picture set semantics”
        （短期参考图像集语义）一节。
    - - __u8
      - `delta_rps_sign`
      - delta 的符号，如规范 “7.4.8 Short-term reference picture set semantics” 一节所述。
    - - __u8
      - `num_negative_pics`
      - 图像顺序计数值小于当前图像顺序计数值的短期 RPS 条目数量。
    - - __u8
      - `num_positive_pics`
      - 图像顺序计数值大于当前图像顺序计数值的短期 RPS 条目数量。
    - - __u32
      - `used_by_curr_pic`
      - 第 i 位指定短期 RPS i 是否被当前图像使用。
    - - __u32
      - `use_delta_flag`
      - 第 i 位指定短期 RPS i 是否被包含在短期 RPS 条目中。
    - - __u16
      - `abs_delta_rps_minus1`
      - 绝对 delta RPS，如规范 “7.4.8 Short-term reference picture set semantics” 一节所述。
    - - __u16
      - `delta_poc_s0_minus1[^16^]`
      - 指定短期 RPS 中第 i 个条目的负图像顺序计数 delta。详见规范 “7.4.8 Short-term
        reference picture set semantics” 一节。
    - - __u16
      - `delta_poc_s1_minus1[^16^]`
      - 指定短期 RPS 中第 i 个条目的正图像顺序计数 delta。详见规范 “7.4.8 Short-term
        reference picture set semantics” 一节。
    - - __u16
      - `flags`
      - 参见扩展短期 RPS 标志 <hevc_ext_sps_st_rps_flags>


`扩展 SPS 短期 RPS 标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_HEVC_EXT_SPS_ST_RPS_FLAG_INTER_REF_PIC_SET_PRED`
      - 0x00000001
      - 指定短期 RPS 是否从另一个短期 RPS 预测得到。详见规范 “7.4.8 Short-term reference
        picture set semantics” 一节。


`V4L2_CID_STATELESS_AV1_SEQUENCE (struct)`
    表示一个 AV1 序列 OBU（Open Bitstream Unit，开放码流单元）。更多细节参见 av1 规范
    第 5.5 节 “Sequence header OBU syntax”（序列头 OBU 语法）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `flags`
      - 参见 AV1 序列标志 <av1_sequence_flags>。
    - - __u8
      - `seq_profile`
      - 指定编码视频序列中可使用的特性。
    - - __u8
      - `order_hint_bits`
      - 指定每帧中 order_hint 字段所用的比特数。
    - - __u8
      - `bit_depth`
      - 用于该序列的位深，更多细节见 av1 规范第 5.5.2 节 “Color config syntax”
        （颜色配置语法）。
    - - __u8
      - `reserved`
      - 应用程序与驱动必须将本字段置为零。
    - - __u16
      - `max_frame_width_minus_1`
      - 指定由该序列头所表示的最大帧宽度减 1。
    - - __u16
      - `max_frame_height_minus_1`
      - 指定由该序列头所表示的最大帧高度减 1。


`AV1 序列标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEQUENCE_FLAG_STILL_PICTURE`
      - 0x00000001
      - 若设置，指定编码视频序列仅包含一个编码帧。若未设置，指定编码视频序列包含一个或多个
        编码帧。
    - - `V4L2_AV1_SEQUENCE_FLAG_USE_128X128_SUPERBLOCK`
      - 0x00000002
      - 若设置，表示 superblock 包含 128x128 亮度样本。为 0 时，表示 superblock 包含
        64x64 亮度样本。所包含的色度样本数量取决于 subsampling_x 和 subsampling_y。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_FILTER_INTRA`
      - 0x00000004
      - 若设置，指定 use_filter_intra 语法元素可能呈现。若未设置，指定 use_filter_intra
        语法元素不会出现。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTRA_EDGE_FILTER`
      - 0x00000008
      - 指定是否启用帧内边缘滤波过程。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_INTERINTRA_COMPOUND`
      - 0x00000010
      - 若设置，指定帧间块的 mode info 可包含 interintra 语法元素。若未设置，指定 interintra
        语法元素不会出现。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_MASKED_COMPOUND`
      - 0x00000020
      - 若设置，指定帧间块的 mode info 可包含 compound_type 语法元素。若未设置，指定
        compound_type 语法元素不会出现。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_WARPED_MOTION`
      - 0x00000040
      - 若设置，表示 allow_warped_motion 语法元素可能出现。若未设置，表示 allow_warped_motion
        语法元素不会出现。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_DUAL_FILTER`
      - 0x00000080
      - 若设置，表示帧间预测滤波器类型可在水平和垂直方向上独立指定。若标志为 0，则只能指定
        一种滤波器类型，并在两个方向均使用。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_ORDER_HINT`
      - 0x00000100
      - 若设置，表示可基于 order hint 值使用相应工具。若未设置，表示基于 order hint 的工具
        被禁用。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_JNT_COMP`
      - 0x00000200
      - 若设置，表示距离加权过程可用于帧间预测。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_REF_FRAME_MVS`
      - 0x00000400
      - 若设置，表示 use_ref_frame_mvs 语法元素可能出现。若未设置，表示 use_ref_frame_mvs
        语法元素不会出现。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_SUPERRES`
      - 0x00000800
      - 若设置，指定未压缩头中将出现 use_superres 语法元素。若未设置，指定 use_superres
        语法元素不会出现（而是在未压缩头中直接将 use_superres 设为 0，无需读取）。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_CDEF`
      - 0x00001000
      - 若设置，指定可启用 cdef 滤波。若未设置，指定 cdef 滤波被禁用。
    - - `V4L2_AV1_SEQUENCE_FLAG_ENABLE_RESTORATION`
      - 0x00002000
      - 若设置，指定可启用环路恢复滤波。若未设置，指定环路恢复滤波被禁用。
    - - `V4L2_AV1_SEQUENCE_FLAG_MONO_CHROME`
      - 0x00004000
      - 若设置，表示视频不包含 U 和 V 颜色平面。若未设置，表示视频包含 Y、U 和 V 颜色平面。
    - - `V4L2_AV1_SEQUENCE_FLAG_COLOR_RANGE`
      - 0x00008000
      - 若设置，发信号表示全摆动（full swing）表示，即“全范围量化（Full Range Quantization）”。
        若未设置，发信号表示演播室摆动（studio swing）表示，即“限制范围量化（Limited Range
        Quantization）”。
    - - `V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_X`
      - 0x00010000
      - 指定色度子采样格式。
    - - `V4L2_AV1_SEQUENCE_FLAG_SUBSAMPLING_Y`
      - 0x00020000
      - 指定色度子采样格式。
    - - `V4L2_AV1_SEQUENCE_FLAG_FILM_GRAIN_PARAMS_PRESENT`
      - 0x00040000
      - 指定编码视频序列中是否存在胶片颗粒（film grain）参数。
    - - `V4L2_AV1_SEQUENCE_FLAG_SEPARATE_UV_DELTA_Q`
      - 0x00080000
      - 若设置，表示 U 和 V 平面可具有独立的 delta 量化器值。若未设置，表示 U 和 V 平面将
        共享相同的 delta 量化器值。


`V4L2_CID_STATELESS_AV1_TILE_GROUP_ENTRY (struct)`
    表示 AV1 Tile Group 内的单个 AV1 tile。注意 MiRowStart、MiRowEnd、MiColStart 和 MiColEnd
    可通过使用 tile_row 和 tile_col，从 struct v4l2_ctrl_av1_frame 中的 struct
    v4l2_av1_tile_info 获取。更多细节参见 av1 规范第 6.10.1 节 “General tile group OBU
    semantics”（通用 tile group OBU 语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tile_offset`
      - 距 OBU 数据的偏移，即编码 tile 数据实际开始的位置。
    - - __u32
      - `tile_size`
      - 指定编码 tile 的大小（字节）。等价于 av1 中的 “TileSize”。
    - - __u32
      - `tile_row`
      - 指定当前 tile 的行。等价于 av1 中的 “TileRow”。
    - - __u32
      - `tile_col`
      - 指定当前 tile 的列。等价于 av1 中的 “TileColumn”。


	AV1 扭曲模型（Warp Model），如 av1 规范第 3 节 “Symbols and abbreviated terms”
	（符号与缩写术语）所述。


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_WARP_MODEL_IDENTITY`
      - 0
      - 扭曲模型仅为恒等变换。
    - - `V4L2_AV1_WARP_MODEL_TRANSLATION`
      - 1
      - 扭曲模型为纯平移。
    - - `V4L2_AV1_WARP_MODEL_ROTZOOM`
      - 2
      - 扭曲模型为旋转 + 对称缩放 + 平移。
    - - `V4L2_AV1_WARP_MODEL_AFFINE`
      - 3
      - 扭曲模型为通用仿射变换。


AV1 参考帧，如 av1 规范第 6.10.24 节 “Ref frames semantics”（参考帧语义）所述。


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_REF_INTRA_FRAME`
      - 0
      - 帧内参考帧。
    - - `V4L2_AV1_REF_LAST_FRAME`
      - 1
      - Last 帧参考。
    - - `V4L2_AV1_REF_LAST2_FRAME`
      - 2
      - Last2 帧参考。
    - - `V4L2_AV1_REF_LAST3_FRAME`
      - 3
      - Last3 帧参考。
    - - `V4L2_AV1_REF_GOLDEN_FRAME`
      - 4
      - Golden 帧参考。
    - - `V4L2_AV1_REF_BWDREF_FRAME`
      - 5
      - BWD 帧参考。
    - - `V4L2_AV1_REF_ALTREF2_FRAME`
      - 6
      - ALTREF2 帧参考。
    - - `V4L2_AV1_REF_ALTREF_FRAME`
      - 7
      - ALTREF 帧参考。


AV1 全局运动参数，如 av1 规范第 6.8.17 节 “Global motion params semantics”（全局运动参数语义）所述。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 包含每个参考帧标志的位域。更多细节参见 AV1 全局运动标志 <av1_global_motion_flags>。
    - - enum `v4l2_av1_warp_model`
      - `type[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 所使用的全局运动变换类型。
    - - __s32
      - `params[V4L2_AV1_TOTAL_REFS_PER_FRAME][^6^]`
      - 本字段与 av1 中的 “gm_params” 含义相同。
    - - __u8
      - `invalid`
      - 位域，指示某个给定参考帧的全局运动参数是否无效。参见第 7.11.3.6 节 Setup shear 过程
        以及变量 “warpValid”。使用 V4L2_AV1_GLOBAL_MOTION_IS_INVALID(ref) 来构造合适的掩码。
    - - __u8
      - `reserved[^3^]`
      - 应用程序与驱动必须将本字段置为零。


`AV1 全局运动标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_GLOBAL`
      - 0x00000001
      - 指定某个特定参考帧是否存在全局运动参数。
    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_ROT_ZOOM`
      - 0x00000002
      - 指定某个特定参考帧是否使用旋转与缩放全局运动。
    - - `V4L2_AV1_GLOBAL_MOTION_FLAG_IS_TRANSLATION`
      - 0x00000004
      - 指定某个特定参考帧是否使用平移全局运动。


AV1 帧恢复类型。


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FRAME_RESTORE_NONE`
      - 0
      - 不应用任何滤波。
    - - `V4L2_AV1_FRAME_RESTORE_WIENER`
      - 1
      - 调用 Wiener 滤波器过程。
    - - `V4L2_AV1_FRAME_RESTORE_SGRPROJ`
      - 2
      - 调用自引导（self guided）滤波器过程。
    - - `V4L2_AV1_FRAME_RESTORE_SWITCHABLE`
      - 3
      - 恢复滤波器可切换。


AV1 环路恢复，如 av1 规范第 6.10.15 节 “Loop restoration params semantics”（环路恢复参数语义）所述。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 参见 AV1 环路恢复标志 <av1_loop_restoration_flags>。
    - - __u8
      - `lr_unit_shift`
      - 指定亮度恢复尺寸是否应减半。
    - - __u8
      - `lr_uv_shift`
      - 指定色度尺寸是否应为亮度尺寸的一半。
    - - __u8
      - `reserved`
      - 应用程序与驱动必须将本字段置为零。
    - - `v4l2_av1_frame_restoration_type`
      - `frame_restoration_type[V4L2_AV1_NUM_PLANES_MAX]`
      - 指定每个平面使用的恢复类型。
    - - __u8
      - `loop_restoration_size[V4L2_AV1_MAX_NUM_PLANES]`
      - 指定环路恢复单元的大小，以当前平面中的样本为单位。


`AV1 环路恢复标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_LOOP_RESTORATION_FLAG_USES_LR`
      - 0x00000001
      - 与 av1 中的 UsesLr 含义相同。
    - - `V4L2_AV1_LOOP_RESTORATION_FLAG_USES_CHROMA_LR`
      - 0x00000002
      - 与 av1 中的 UsesChromaLr 含义相同。


AV1 CDEF 参数语义，如 av1 规范第 6.10.14 节 “CDEF params semantics”（CDEF 参数语义）所述。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `damping_minus_3`
      - 控制去环（deringing）滤波器中的阻尼量。
    - - __u8
      - `bits`
      - 指定指定要应用哪个 CDEF 滤波器所需的比特数。
    - - __u8
      - `y_pri_strength[V4L2_AV1_CDEF_MAX]`
      - 指定主滤波器强度。
    - - __u8
      - `y_sec_strength[V4L2_AV1_CDEF_MAX]`
      - 指定次滤波器强度。
    - - __u8
      - `uv_pri_strength[V4L2_AV1_CDEF_MAX]`
      - 指定主滤波器强度。
    - - __u8
      - `uv_sec_strength[V4L2_AV1_CDEF_MAX]`
      - 指定次滤波器强度。


AV1 分段特征，如 av1 规范第 3 节 “Symbols and abbreviated terms”（符号与缩写术语）所述。


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEG_LVL_ALT_Q`
      - 0
      - 量化器分段特征的索引。
    - - `V4L2_AV1_SEG_LVL_ALT_LF_Y_V`
      - 1
      - 垂直亮度环路滤波器分段特征的索引。
    - - `V4L2_AV1_SEG_LVL_REF_FRAME`
      - 5
      - 参考帧分段特征的索引。
    - - `V4L2_AV1_SEG_LVL_REF_SKIP`
      - 6
      - 跳过（skip）分段特征的索引。
    - - `V4L2_AV1_SEG_LVL_REF_GLOBALMV`
      - 7
      - 全局 mv（global motion vector）特征的索引。
    - - `V4L2_AV1_SEG_LVL_MAX`
      - 8
      - 分段特征的数量。


AV1 分段参数，定义于 av1 规范第 6.8.13 节 “Segmentation params semantics”（分段参数语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 参见 AV1 分段标志 <av1_segmentation_flags>。
    - - __u8
      - `last_active_seg_id`
      - 指示具有某个已启用特征的最高编号分段 id。这在解码分段 id 时用于仅解码与所用分段
        对应的选项。
    - - __u8
      - `feature_enabled[V4L2_AV1_MAX_SEGMENTS]`
      - 位掩码，定义每个分段中启用了哪些特征。使用 V4L2_AV1_SEGMENT_FEATURE_ENABLED 来
        构造合适的掩码。
    - - __u16
      - `feature_data[V4L2_AV1_MAX_SEGMENTS][V4L2_AV1_SEG_LVL_MAX]`
      - 附带在每个特征上的数据。仅当特征被启用时数据条目才有效。


`AV1 分段标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_SEGMENTATION_FLAG_ENABLED`
      - 0x00000001
      - 若设置，表示本帧使用了分段工具（segmentation tool）。若未设置，表示本帧未使用分段。
    - - `V4L2_AV1_SEGMENTATION_FLAG_UPDATE_MAP`
      - 0x00000002
      - 若设置，表示分段映射在本帧解码过程中更新。若未设置，表示使用上一帧的分段映射。
    - - `V4L2_AV1_SEGMENTATION_FLAG_TEMPORAL_UPDATE`
      - 0x00000004
      - 若设置，表示分段映射的更新是相对于已存在的分段映射编码的。若未设置，表示新的分段映射
        不参考已存在的分段映射编码。
    - - `V4L2_AV1_SEGMENTATION_FLAG_UPDATE_DATA`
      - 0x00000008
      - 若设置，表示分段映射的更新是相对于已存在的分段映射编码的。若未设置，表示新的分段映射
        不参考已存在的分段映射编码。
    - - `V4L2_AV1_SEGMENTATION_FLAG_SEG_ID_PRE_SKIP`
      - 0x00000010
      - 若设置，表示分段 id 将在 skip 语法元素之前读取。若未设置，表示 skip 语法元素将首先读取。


AV1 环路滤波器参数，定义于 av1 规范第 6.8.10 节 “Loop filter semantics”（环路滤波器语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 更多细节参见 AV1 环路滤波器标志 <av1_loop_filter_flags>。
    - - __u8
      - `level[^4^]`
      - 一个包含环路滤波器强度值的数组。根据被滤波的图像平面以及被滤波的边缘方向（垂直或水平），
        使用数组中不同的环路滤波器强度值。
    - - __u8
      - `sharpness`
      - 表示锐度等级。loop_filter_level 与 loop_filter_sharpness 共同决定何时对块边缘进行
        滤波，以及滤波对样本值的改变量。环路滤波过程在 av1 规范第 7.14 节中描述。
    - - __u8
      - `ref_deltas[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 包含基于所选参考帧对滤波器等级所需的调整。若该语法元素不存在，则保持其先前的值。
    - - __u8
      - `mode_deltas[^2^]`
      - 包含基于所选模式对滤波器等级所需的调整。若该语法元素不存在，则保持其先前的值。
    - - __u8
      - `delta_lf_res`
      - 指定应作用于已解码环路滤波器 delta 值的左移量。


`AV1 环路滤波器标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_ENABLED`
      - 0x00000001
      - 若设置，表示滤波器等级取决于用于预测某个块的模式和参考帧。若未设置，表示滤波器等级
        不取决于模式和参考帧。
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_UPDATE`
      - 0x00000002
      - 若设置，表示存在额外的语法元素，用于指定哪些模式和参考帧的 delta 需要更新。若未设置，
        表示这些语法元素不存在。
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_PRESENT`
      - 0x00000004
      - 指定是否存在环路滤波器 delta 值。
    - - `V4L2_AV1_LOOP_FILTER_FLAG_DELTA_LF_MULTI`
      - 0x00000008
      - 值为 1 指定为水平亮度边缘、垂直亮度边缘、U 边缘和 V 边缘分别发送独立的环路滤波器
        delta。delta_lf_multi 值为 0 指定所有边缘使用相同的环路滤波器 delta。


AV1 量化参数，定义于 av1 规范第 6.8.11 节 “Quantization params semantics”（量化参数语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 更多细节参见 AV1 环路滤波器标志 <av1_quantization_flags>。
    - - __u8
      - `base_q_idx`
      - 指示基础帧 qindex。用于 Y AC 系数，并作为其他量化器的基础值。
    - - __u8
      - `delta_q_y_dc`
      - 指示相对 base_q_idx 的 Y DC 量化器。
    - - __u8
      - `delta_q_u_dc`
      - 指示相对 base_q_idx 的 U DC 量化器。
    - - __u8
      - `delta_q_u_ac`
      - 指示相对 base_q_idx 的 U AC 量化器。
    - - __u8
      - `delta_q_v_dc`
      - 指示相对 base_q_idx 的 V DC 量化器。
    - - __u8
      - `delta_q_v_ac`
      - 指示相对 base_q_idx 的 V AC 量化器。
    - - __u8
      - `qm_y`
      - 指定用于亮度平面解码的量化矩阵等级。
    - - __u8
      - `qm_u`
      - 指定用于色度 U 平面解码的量化矩阵等级。
    - - __u8
      - `qm_v`
      - 指定用于色度 V 平面解码的量化矩阵等级。
    - - __u8
      - `delta_q_res`
      - 指定应作用于已解码量化器索引 delta 值的左移量。


`AV1 量化标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_QUANTIZATION_FLAG_DIFF_UV_DELTA`
      - 0x00000001
      - 若设置，表示 U 和 V delta 量化器值被分别编码。若未设置，表示 U 和 V delta 量化器
        值共享一个公共值。
    - - `V4L2_AV1_QUANTIZATION_FLAG_USING_QMATRIX`
      - 0x00000002
      - 若设置，指定将使用量化矩阵来计算量化器。
    - - `V4L2_AV1_QUANTIZATION_FLAG_DELTA_Q_PRESENT`
      - 0x00000004
      - 指定是否存在量化器索引 delta 值。


AV1 Tile 信息，定义于 ref:`av1` 规范第 6.8.14 节 “Tile info semantics”（Tile 信息语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 更多细节参见 AV1 Tile 信息标志 <av1_tile_info_flags>。
    - - __u8
      - `context_update_tile_id`
      - 指定用于 CDF 更新的 tile。
    - - __u8
      - `tile_cols`
      - 指定跨帧的 tile 数量。
    - - __u8
      - `tile_rows`
      - 指定沿帧向下的 tile 数量。
    - - __u32
      - `mi_col_starts[V4L2_AV1_MAX_TILE_COLS + 1]`
      - 一个数组，指定每个 tile 跨图像的起始列（以 4x4 亮度样本为单位）。
    - - __u32
      - `mi_row_starts[V4L2_AV1_MAX_TILE_ROWS + 1]`
      - 一个数组，指定每个 tile 跨图像的起始行（以 4x4 亮度样本为单位）。
    - - __u32
      - `width_in_sbs_minus_1[V4L2_AV1_MAX_TILE_COLS]`
      - 指定 tile 的宽度减 1，以 superblock 为单位。
    - - __u32
      - `height_in_sbs_minus_1[V4L2_AV1_MAX_TILE_ROWS]`
      - 指定 tile 的高度减 1，以 superblock 为单位。
    - - __u8
      - `tile_size_bytes`
      - 指定编码每个 tile 大小所需的比特数。
    - - __u8
      - `reserved[^3^]`
      - 应用程序与驱动必须将本字段置为零。


`AV1 Tile 信息标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_TILE_INFO_FLAG_UNIFORM_TILE_SPACING`
      - 0x00000001
      - 若设置，表示 tile 在帧上均匀分布（换言之，除右侧和底部边缘可能较小的 tile 外，所有
        tile 大小相同）。若未设置，表示 tile 大小是被编码的。


AV1 帧类型


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_KEY_FRAME`
      - 0
      - 关键帧。
    - - `V4L2_AV1_INTER_FRAME`
      - 1
      - 帧间帧。
    - - `V4L2_AV1_INTRA_ONLY_FRAME`
      - 2
      - 仅帧内帧。
    - - `V4L2_AV1_SWITCH_FRAME`
      - 3
      - 切换帧。


AV1 插值滤波器


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP`
      - 0
      - 八抽头滤波器。
    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SMOOTH`
      - 1
      - 八抽头平滑滤波器。
    - - `V4L2_AV1_INTERPOLATION_FILTER_EIGHTTAP_SHARP`
      - 2
      - 八抽头锐利滤波器。
    - - `V4L2_AV1_INTERPOLATION_FILTER_BILINEAR`
      - 3
      - 双线性滤波器。
    - - `V4L2_AV1_INTERPOLATION_FILTER_SWITCHABLE`
      - 4
      - 滤波器选择于块级别发出信号。


AV1 Tx 模式，如 av1 规范第 6.8.21 节 “TX mode semantics”（TX 模式语义）所述。


    \scriptsize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_TX_MODE_ONLY_4X4`
      - 0
      - 逆变换将仅使用 4x4 变换。
    - - `V4L2_AV1_TX_MODE_LARGEST`
      - 1
      - 逆变换将使用能放入块内的最大变换尺寸。
    - - `V4L2_AV1_TX_MODE_SELECT`
      - 2
      - 变换尺寸的选择为每个块显式指定。


`V4L2_CID_STATELESS_AV1_FRAME (struct)`
    表示一个帧头 OBU。更多细节参见 av1 规范第 6.8 节 “Frame Header OBU semantics”
    （帧头 OBU 语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_av1_tile_info`
      - `tile_info`
      - Tile 信息。
    - - struct `v4l2_av1_quantization`
      - `quantization`
      - 量化参数。
    - - __u8
      - `superres_denom`
      - 上采样比例分母。
    - - struct `v4l2_av1_segmentation`
      - `segmentation`
      - 分段参数。
    - - struct `v4l2_av1_loop_filter`
      - `loop_filter`
      - 环路滤波器参数。
    - - struct `v4l2_av1_cdef`
      - `cdef`
      - CDEF 参数。
    - - __u8
      - `skip_mode_frame[^2^]`
      - 指定当 skip_mode 等于 1 时用于复合预测的帧。
    - - __u8
      - `primary_ref_frame`
      - 指定哪个参考帧包含在 CDF 值以及应在帧开始时加载的其他状态。
    - - struct `v4l2_av1_loop_restoration`
      - `loop_restoration`
      - 环路恢复参数。
    - - struct `v4l2_av1_global_motion`
      - `global_motion`
      - 全局运动参数。
    - - __u32
      - `flags`
      - 更多细节参见 AV1 帧标志 <av1_frame_flags>。
    - - enum `v4l2_av1_frame_type`
      - `frame_type`
      - 指定 AV1 帧类型。
    - - __u32
      - `order_hint`
      - 指定本帧期望输出顺序的 OrderHintBits 个最低有效位。
    - - __u32
      - `upscaled_width`
      - 上采样宽度。
    - - enum `v4l2_av1_interpolation_filter`
      - `interpolation_filter`
      - 指定用于执行帧间预测的滤波器选择。
    - - enum `v4l2_av1_tx_mode`
      - `tx_mode`
      - 指定变换尺寸如何确定。
    - - __u32
      - `frame_width_minus_1`
      - 加 1 得到帧的宽度。
    - - __u32
      - `frame_height_minus_1`
      - 加 1 得到帧的高度。
    - - __u16
      - `render_width_minus_1`
      - 加 1 得到以亮度样本表示的帧渲染宽度。
    - - __u16
      - `render_height_minus_1`
      - 加 1 得到以亮度样本表示的帧渲染高度。
    - - __u32
      - `current_frame_id`
      - 指定当前帧的帧 id 编号。帧 id 编号是不影响解码过程的附加信息，但为解码器提供了检测
        缺失参考帧的方式，以便采取适当措施。
    - - __u8
      - `buffer_removal_time[V4L2_AV1_MAX_OPERATING_POINTS]`
      - 指定对于操作点 opNum，从最后一个随机访问点的移除时间起算、以 DecCT 时钟节拍为单位的
        帧移除时间。
    - - __u8
      - `reserved[^4^]`
      - 应用程序与驱动必须将本字段置为零。
    - - __u32
      - `order_hints[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 指定每个参考帧的期望输出顺序提示。本字段对应于规范（第 5.9.2 节 “Uncompressed header
        syntax”，未压缩头部语法）中的 OrderHints 变量。因此，它仅用于非帧内帧，否则被忽略。
        order_hints[^0^] 始终被忽略。
    - - __u64
      - `reference_frame_ts[V4L2_AV1_TOTAL_REFS_PER_FRAME]`
      - 从 enum `v4l2_av1_reference_frame` 的 `V4L2_AV1_REF_LAST_FRAME` 开始枚举的每个参考帧
        的 V4L2 时间戳。这表示规范中描述的参考槽状态，并由用户空间通过第 7.20 节的
        “Reference frame update process”（参考帧更新过程）更新。该时间戳引用 struct
        `v4l2_buffer` 中的 `timestamp` 字段。使用 `v4l2_timeval_to_ns()` 函数将 struct
        `timeval` 在 struct `v4l2_buffer` 中转换为 __u64。
    - - __s8
      - `ref_frame_idx[V4L2_AV1_REFS_PER_FRAME]`
      - 指向 `reference_frame_ts` 的索引，表示帧间帧使用的参考有序列表。与同名的码流语法元素
        匹配。
    - - __u8
      - `refresh_frame_flags`
      - 包含一个位掩码，指定解码后哪些参考帧槽将用当前帧更新。


`AV1 帧标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FRAME_FLAG_SHOW_FRAME`
      - 0x00000001
      - 若设置，指定本帧一旦解码就应立即输出。若未设置，指定本帧不应立即输出；若后续未压缩头
        使用 show_existing_frame 等于 1，则可能在之后输出。
    - - `V4L2_AV1_FRAME_FLAG_SHOWABLE_FRAME`
      - 0x00000002
      - 若设置，指定本帧可使用 show_existing_frame 机制输出。若未设置，指定本帧不会使用
        show_existing_frame 机制输出。
    - - `V4L2_AV1_FRAME_FLAG_ERROR_RESILIENT_MODE`
      - 0x00000004
      - 指定是否启用了错误韧性模式。
    - - `V4L2_AV1_FRAME_FLAG_DISABLE_CDF_UPDATE`
      - 0x00000008
      - 指定是否应禁用符号解码过程中的 CDF 更新。
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_SCREEN_CONTENT_TOOLS`
      - 0x00000010
      - 若设置，表示帧内块可使用调色板（palette）编码。若未设置，表示从不使用调色板编码。
    - - `V4L2_AV1_FRAME_FLAG_FORCE_INTEGER_MV`
      - 0x00000020
      - 若设置，指定运动矢量将始终为整数。若未设置，指定运动矢量可包含分数比特。
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_INTRABC`
      - 0x00000040
      - 若设置，表示本帧中可使用帧内块复制（intra block copy）。若未设置，表示本帧不允许
        帧内块复制。
    - - `V4L2_AV1_FRAME_FLAG_USE_SUPERRES`
      - 0x00000080
      - 若设置，表示需要上采样。
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_HIGH_PRECISION_MV`
      - 0x00000100
      - 若设置，指定运动矢量以八分之一像素精度指定。若未设置，指定运动矢量以四分之一像素精度
        指定。
    - - `V4L2_AV1_FRAME_FLAG_IS_MOTION_MODE_SWITCHABLE`
      - 0x00000200
      - 若未设置，指定仅使用 SIMPLE 运动模式。
    - - `V4L2_AV1_FRAME_FLAG_USE_REF_FRAME_MVS`
      - 0x00000400
      - 若设置，指定解码当前帧时可使用来自上一帧的运动矢量信息。若未设置，指定不使用该信息。
    - - `V4L2_AV1_FRAME_FLAG_DISABLE_FRAME_END_UPDATE_CDF`
      - 0x00000800
      - 若设置，表示帧末尾的 CDF 更新被禁用。若未设置，表示帧末尾的 CDF 更新被启用。
    - - `V4L2_AV1_FRAME_FLAG_ALLOW_WARPED_MOTION`
      - 0x00001000
      - 若设置，表示 motion_mode 语法元素可能出现；若未设置，表示 motion_mode 语法元素不会
        出现。
    - - `V4L2_AV1_FRAME_FLAG_REFERENCE_SELECT`
      - 0x00002000
      - 若设置，指定帧间块的 mode info 包含 comp_mode 语法元素，指示使用单参考还是复合参考
        预测。若未设置，指定所有帧间块使用单预测。
    - - `V4L2_AV1_FRAME_FLAG_REDUCED_TX_SET`
      - 0x00004000
      - 若设置，指定本帧被限制为完整变换类型集合的一个缩减子集。
    - - `V4L2_AV1_FRAME_FLAG_SKIP_MODE_ALLOWED`
      - 0x00008000
      - 本标志与 av1 中的 SkipModeAllowed 含义相同。
    - - `V4L2_AV1_FRAME_FLAG_SKIP_MODE_PRESENT`
      - 0x00010000
      - 若设置，指定 skip_mode 语法元素将出现；若未设置，指定本帧不使用 skip_mode。
    - - `V4L2_AV1_FRAME_FLAG_FRAME_SIZE_OVERRIDE`
      - 0x00020000
      - 若设置，指定帧大小将指定为某个参考帧的大小，或从 frame_width_minus_1 和
        frame_height_minus_1 语法元素计算得出。若未设置，指定帧大小等于序列头中的大小。
    - - `V4L2_AV1_FRAME_FLAG_BUFFER_REMOVAL_TIME_PRESENT`
      - 0x00040000
      - 若设置，指定 buffer_removal_time 存在。若未设置，指定 buffer_removal_time 不存在。
    - - `V4L2_AV1_FRAME_FLAG_FRAME_REFS_SHORT_SIGNALING`
      - 0x00080000
      - 若设置，表示仅显式发出两个参考帧的信号。若未设置，表示所有参考帧都被显式发出信号。


`V4L2_CID_STATELESS_AV1_FILM_GRAIN (struct)`
    表示可选的胶片颗粒参数。更多细节参见 av1 规范第 6.8.20 节 “Film grain params semantics”
    （胶片颗粒参数语义）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `flags`
      - 参见 AV1 胶片颗粒标志 <av1_film_grain_flags>。
    - - __u8
      - `cr_mult`
      - 表示用于推导 cr 分量缩放函数输入索引的 cr 分量的乘数。
    - - __u16
      - `grain_seed`
      - 指定胶片颗粒合成过程中所使用的伪随机数的起始值。
    - - __u8
      - `film_grain_params_ref_idx`
      - 指示哪个参考帧包含用于本帧的胶片颗粒参数。
    - - __u8
      - `num_y_points`
      - 指定亮度分量分段线性缩放函数的点数。
    - - __u8
      - `point_y_value[V4L2_AV1_MAX_NUM_Y_POINTS]`
      - 表示亮度分量分段线性缩放函数第 i 个点的 x（亮度值）坐标。这些值在 0..255 的范围上发出
        信号。对于 10 位视频，这些值对应于除以 4 的亮度值；对于 12 位视频，对应于除以 16 的
        亮度值。
    - - __u8
      - `point_y_scaling[V4L2_AV1_MAX_NUM_Y_POINTS]`
      - 表示亮度分量分段线性缩放函数第 i 个点的缩放（输出）值。
    - - __u8
      - `num_cb_points`
      - 指定 cb 分量分段线性缩放函数的点数。
    - - __u8
      - `point_cb_value[V4L2_AV1_MAX_NUM_CB_POINTS]`
      - 表示 cb 分量分段线性缩放函数第 i 个点的 x 坐标。这些值在 0..255 的范围上发出信号。
    - - __u8
      - `point_cb_scaling[V4L2_AV1_MAX_NUM_CB_POINTS]`
      - 表示 cb 分量分段线性缩放函数第 i 个点的缩放（输出）值。
    - - __u8
      - `num_cr_points`
      - 表示 cr 分量分段线性缩放函数的点数。
    - - __u8
      - `point_cr_value[V4L2_AV1_MAX_NUM_CR_POINTS]`
      - 表示 cr 分量分段线性缩放函数第 i 个点的 x 坐标。这些值在 0..255 的范围上发出信号。
    - - __u8
      - `point_cr_scaling[V4L2_AV1_MAX_NUM_CR_POINTS]`
      - 表示 cr 分量分段线性缩放函数第 i 个点的缩放（输出）值。
    - - __u8
      - `grain_scaling_minus_8`
      - 表示应用于色度分量值的移位减 8。grain_scaling_minus_8 可取 0..3 的值，并决定胶片颗粒
        标准差的范围与量化步长。
    - - __u8
      - `ar_coeff_lag`
      - 指定亮度与色度的自回归系数数量。
    - - __u8
      - `ar_coeffs_y_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 指定用于 Y 平面的自回归系数。
    - - __u8
      - `ar_coeffs_cb_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 指定用于 U 平面的自回归系数。
    - - __u8
      - `ar_coeffs_cr_plus_128[V4L2_AV1_AR_COEFFS_SIZE]`
      - 指定用于 V 平面的自回归系数。
    - - __u8
      - `ar_coeff_shift_minus_6`
      - 指定自回归系数的范围。值 0、1、2、3 分别对应自回归系数的范围 [-2, 2)、[-1, 1)、
        [-0.5, 0.5) 与 [-0.25, 0.25)。
    - - __u8
      - `grain_scale_shift`
      - 指定在颗粒合成过程中高斯随机数应缩小的程度。
    - - __u8
      - `cb_mult`
      - 表示用于推导 cb 分量缩放函数输入索引的 cb 分量的乘数。
    - - __u8
      - `cb_luma_mult`
      - 表示用于推导 cb 分量缩放函数输入索引的平均亮度分量的乘数。
    - - __u8
      - `cr_luma_mult`
      - 表示用于推导 cr 分量缩放函数输入索引的平均亮度分量的乘数。
    - - __u16
      - `cb_offset`
      - 表示用于推导 cb 分量缩放函数输入索引的偏移。
    - - __u16
      - `cr_offset`
      - 表示用于推导 cr 分量缩放函数输入索引的偏移。
    - - __u8
      - `reserved[^4^]`
      - 应用程序与驱动必须将本字段置为零。


`AV1 胶片颗粒标志`


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - `V4L2_AV1_FILM_GRAIN_FLAG_APPLY_GRAIN`
      - 0x00000001
      - 若设置，指定应向本帧添加胶片颗粒。若未设置，指定不应添加胶片颗粒。
    - - `V4L2_AV1_FILM_GRAIN_FLAG_UPDATE_GRAIN`
      - 0x00000002
      - 若设置，表示应发送一组新参数。若未设置，指定应使用前一组参数。
    - - `V4L2_AV1_FILM_GRAIN_FLAG_CHROMA_SCALING_FROM_LUMA`
      - 0x00000004
      - 若设置，指定色度缩放由亮度缩放推导得出。
    - - `V4L2_AV1_FILM_GRAIN_FLAG_OVERLAP`
      - 0x00000008
      - 若设置，表示应应用胶片颗粒块之间的重叠。若未设置，表示不应应用胶片颗粒块之间的重叠。
    - - `V4L2_AV1_FILM_GRAIN_FLAG_CLIP_TO_RESTRICTED_RANGE`
      - 0x00000010
      - 若设置，表示在对样本值添加胶片颗粒后，应将其裁剪到受限（演播室，即限制）范围
        （参见 color_range 语义中关于 studio swing 的解释）。若未设置，表示在对样本值添加
        胶片颗粒后，应将其裁剪到全范围。

