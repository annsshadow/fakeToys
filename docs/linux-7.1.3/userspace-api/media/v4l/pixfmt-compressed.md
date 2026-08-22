
######## 压缩格式



    \small


    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - 标识      - 代码
      - 说明
    - .. _V4L2-PIX-FMT-JPEG:

      - `V4L2_PIX_FMT_JPEG`
      - 'JPEG'
      - 待定。另请参VIDIOC_G_JPEGCOMP <VIDIOC_G_JPEGCOMP>	VIDIOC_S_JPEGCOMP <VIDIOC_G_JPEGCOMP>    - .. _V4L2-PIX-FMT-MPEG:

      - `V4L2_PIX_FMT_MPEG`
      - 'MPEG'
      - MPEG 多路复用流。实际格式由扩展控制
	`V4L2_CID_MPEG_STREAM_TYPE` 决定，参	mpeg-control-id    - .. _V4L2-PIX-FMT-H264:

      - `V4L2_PIX_FMT_H264`
      - 'H264'
      - H264 访问单元（Access Unit）	解码器期望每个缓冲区包含一个访问单元	编码器为每个缓冲区生成一个访问单元	如果 VIDIOC_ENUM_FMT 报告 `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`	则解码器没有特殊要求，因为它可以从原始字节流中解析出所	信息    - .. _V4L2-PIX-FMT-H264-NO-SC:

      - `V4L2_PIX_FMT_H264_NO_SC`
      - 'AVC1'
      - 不带起始码的 H264 视频基本流    - .. _V4L2-PIX-FMT-H264-MVC:

      - `V4L2_PIX_FMT_H264_MVC`
      - 'M264'
      - H264 MVC 视频基本流    - .. _V4L2-PIX-FMT-H264-SLICE:

      - `V4L2_PIX_FMT_H264_SLICE`
      - 'S264'
      - 经过解析H264 slice 数据，包slice 头，可带也可不带
	起始码，提取H264 比特流	该格式适用于通过 stateless_decoder 实现 H264
	流水线的无状态（stateless）视频解码器	此像素格式有两个修改量，必须至少通过
	`V4L2_CID_STATELESS_H264_DECODE_MODE`
        `V4L2_CID_STATELESS_H264_START_CODE` 控件设置一次	此外，与待解码帧相关的元数据必须通过
	`V4L2_CID_STATELESS_H264_SPS`	`V4L2_CID_STATELESS_H264_PPS`	`V4L2_CID_STATELESS_H264_SCALING_MATRIX`	`V4L2_CID_STATELESS_H264_SLICE_PARAMS` 	`V4L2_CID_STATELESS_H264_DECODE_PARAMS` 控件传入。参	相关Codec Control IDs <v4l2-codec-stateless-h264>        使用此像素格式时必须提供恰好一个输出缓冲区和一个捕获缓冲区	输出缓冲区必须包含适当数量的宏块，以解码出与对应捕获缓冲区
	相匹配的完整帧
	该格式的语法记录h264 规范	7.3.2.8 节“Slice layer without partitioning RBSP syntax”及
	后续章节中
    - .. _V4L2-PIX-FMT-H263:

      - `V4L2_PIX_FMT_H263`
      - 'H263'
      - H263 视频基本流    - .. _V4L2-PIX-FMT-SPK:

      - `V4L2_PIX_FMT_SPK`
      - 'SPK0'
      - Sorenson Spark H.263 的一种实现，用于 Flash Video Adobe Flash 文件    - .. _V4L2-PIX-FMT-MPEG1:

      - `V4L2_PIX_FMT_MPEG1`
      - 'MPG1'
      - MPEG1 图像。每个缓冲区Picture 头开始，随后根据需要包	其他头，并以 Picture 数据结束	如果 VIDIOC_ENUM_FMT 报告 `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`	则解码器没有特殊要求，因为它可以从原始字节流中解析出所	信息    - .. _V4L2-PIX-FMT-MPEG2:

      - `V4L2_PIX_FMT_MPEG2`
      - 'MPG2'
      - MPEG2 图像。每个缓冲区Picture 头开始，随后根据需要包	其他头，并以 Picture 数据结束	如果 VIDIOC_ENUM_FMT 报告 `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`	则解码器没有特殊要求，因为它可以从原始字节流中解析出所	信息    - .. _V4L2-PIX-FMT-MPEG2-SLICE:

      - `V4L2_PIX_FMT_MPEG2_SLICE`
      - 'MG2S'
      - 经过解析MPEG-2 slice 数据，提取自 MPEG-2 比特流	该格式适用于通过 stateless_decoder 实现 MPEG-2
	流水线的无状态视频解码器	与待解码帧相关的元数据必须通过
	`V4L2_CID_STATELESS_MPEG2_SEQUENCE`         `V4L2_CID_STATELESS_MPEG2_PICTURE` 控件传入        量化矩阵可通过
	`V4L2_CID_STATELESS_MPEG2_QUANTISATION` 控件可选地指定	参见相关Codec Control IDs <v4l2-codec-stateless-mpeg2>	使用此像素格式时必须提供恰好一个输出缓冲区和一个捕获缓冲区	输出缓冲区必须包含适当数量的宏块，以解码出与对应捕获缓冲区
	相匹配的完整帧    - .. _V4L2-PIX-FMT-MPEG4:

      - `V4L2_PIX_FMT_MPEG4`
      - 'MPG4'
      - MPEG4 视频基本流    - .. _V4L2-PIX-FMT-XVID:

      - `V4L2_PIX_FMT_XVID`
      - 'XVID'
      - Xvid 视频基本流    - .. _V4L2-PIX-FMT-VC1-ANNEX-G:

      - `V4L2_PIX_FMT_VC1_ANNEX_G`
      - 'VC1G'
      - VC1，符SMPTE 421M Annex G 的流    - .. _V4L2-PIX-FMT-VC1-ANNEX-L:

      - `V4L2_PIX_FMT_VC1_ANNEX_L`
      - 'VC1L'
      - VC1，符SMPTE 421M Annex L 的流    - .. _V4L2-PIX-FMT-VP8:

      - `V4L2_PIX_FMT_VP8`
      - 'VP80'
      - VP8 压缩视频帧。编码器为每个缓冲区生成一个压缩帧	解码器每个缓冲区需要一个压缩帧    - .. _V4L2-PIX-FMT-VP8-FRAME:

      - `V4L2_PIX_FMT_VP8_FRAME`
      - 'VP8F'
      - 经过解析VP8 帧，包含帧头，提取自容器	该格式适用于通过 stateless_decoder 实现 VP8
	流水线的无状态视频解码器	与待解码帧相关的元数据必须通过
	`V4L2_CID_STATELESS_VP8_FRAME` 控件传入	参见相关Codec Control IDs <v4l2-codec-stateless-vp8>	使用此像素格式时必须提供恰好一个输出缓冲区和一个捕获缓冲区	输出缓冲区必须包含适当数量的宏块，以解码出与对应捕获缓冲区
	相匹配的完整帧
    - .. _V4L2-PIX-FMT-VP9:

      - `V4L2_PIX_FMT_VP9`
      - 'VP90'
      - VP9 压缩视频帧。编码器为每个缓冲区生成一个压缩帧	解码器每个缓冲区需要一个压缩帧    - .. _V4L2-PIX-FMT-VP9-FRAME:

      - `V4L2_PIX_FMT_VP9_FRAME`
      - 'VP9F'
      - 经过解析VP9 帧，包含帧头，提取自容器	该格式适用于通过 stateless_decoder 实现 VP9
	流水线的无状态视频解码器	与待解码帧相关的元数据必须通过
	`V4L2_CID_STATELESS_VP9_FRAME` 	`V4L2_CID_STATELESS_VP9_COMPRESSED_HDR` 控件传入	参见相关Codec Control IDs <v4l2-codec-stateless-vp9>	使用此像素格式时必须提供恰好一个输出缓冲区和一个捕获缓冲区	输出缓冲区必须包含适当数量的宏块，以解码出与对应捕获缓冲区
	相匹配的完整帧    - .. _V4L2-PIX-FMT-HEVC:

      - `V4L2_PIX_FMT_HEVC`
      - 'HEVC'
      - HEVC/H.265 访问单元	解码器期望每个缓冲区包含一个访问单元	编码器为每个缓冲区生成一个访问单元	如果 VIDIOC_ENUM_FMT 报告 `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`	则解码器没有特殊要求，因为它可以从原始字节流中解析出所	信息    - .. _V4L2-PIX-FMT-HEVC-SLICE:

      - `V4L2_PIX_FMT_HEVC_SLICE`
      - 'S265'
      - 经过解析HEVC slice 数据，提取自 HEVC 比特流	该格式适用于实HEVC 流水线（使用 mem2mem 	media-request-api）的无状态视频解码器	此像素格式有两个修改量，必须至少通过
	`V4L2_CID_MPEG_VIDEO_HEVC_DECODE_MODE`
        `V4L2_CID_MPEG_VIDEO_HEVC_START_CODE` 控件设置一次	与待解码帧相关的元数据必须通过以下控件传入        `V4L2_CID_MPEG_VIDEO_HEVC_SPS`        `V4L2_CID_MPEG_VIDEO_HEVC_PPS`         `V4L2_CID_MPEG_VIDEO_HEVC_SLICE_PARAMS`	参见相关Codec Control IDs <v4l2-codec-stateless-hevc>	与此像素格式关联的缓冲区必须包含适当数量的宏块，
	以解码出完整的对应帧    - .. _V4L2-PIX-FMT-FWHT:

      - `V4L2_PIX_FMT_FWHT`
      - 'FWHT'
      - 使用基于快Walsh-Hadamard 变换（Fast Walsh Hadamard
        Transform）的编解码器生成的视频基本流。该编解码器	vicodecVirtual Codec'）驱动实现。更多细节参	codec-fwht.h 头文件	VIDIOC_ENUM_FMT 报告 `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`	因为解码器可以从原始字节流中解析出所有信息    - .. _V4L2-PIX-FMT-FWHT-STATELESS:

      - `V4L2_PIX_FMT_FWHT_STATELESS`
      - 'SFWH'
      - 格式V4L2_PIX_FMT_FWHT 相同，但要求无状态编解码器实现        与待解码帧相关的元数据必须通过
        `V4L2_CID_STATELESS_FWHT_PARAMS` 控件传入	参见相关Codec Control ID <codec-stateless-fwht>    - .. _V4L2-PIX-FMT-RV30:

      - `V4L2_PIX_FMT_RV30`
      - 'RV30'
      - RealVideo（也拼作 Real Video）是一套由 RealNetworks
        开发的专有视频压缩格式，具体格式随版本变化        RealVideo 编解码器由四字符代码（FourCC）标识        RV30 对应 RealVideo 8，推测主要基H.264 的早期草案    - .. _V4L2-PIX-FMT-RV40:

      - `V4L2_PIX_FMT_RV40`
      - 'RV40'
      - RV40 代表 RealVideo 9 RealVideo 10        RealVideo 9 推测基于 H.264        RealVideo 10（又RV9 EHQ）指RV9 格式的改进编码器	它与 RV9 播放器完全向后兼容——格式和解码器没有变化，
	只有编码器发生了变化。因此，它使用相同的 FourCC
    - .. _V4L2-PIX-FMT-AV1-FRAME:

      - `V4L2_PIX_FMT_AV1_FRAME`
      - 'AV1F'
      - 经过解析AV1 帧，包含帧头，提取自容器        该格式适用于通过 stateless_decoder 实现 AV1
        流水线的无状态视频解码器。与待解码帧相关的元数据必须通过
        `V4L2_CID_STATELESS_AV1_SEQUENCE`、`V4L2_CID_STATELESS_AV1_FRAME`        `V4L2_CID_STATELESS_AV1_TILE_GROUP_ENTRY` 控件传入        参见相关Codec Control IDs <v4l2-codec-stateless-av1>        使用此像素格式时必须提供恰好一个输出缓冲区和一个捕获缓冲区	输出缓冲区必须包含适当数量的宏块，以解码出与对应捕获缓冲区
	相匹配的完整帧
    - .. _V4L2-PIX-FMT-AV1:

      - `V4L2_PIX_FMT_AV1`
      - 'AV01'
      - AV1 压缩视频帧。该格式适用于实AV1 流水线        解码器实现的是有状态（stateful）视频解码器，期望每个缓冲区
	OBU 流格式中包含一个时间单元（temporal unit）        编码器为每个缓冲区生成一个时间单元
    \normalsize
