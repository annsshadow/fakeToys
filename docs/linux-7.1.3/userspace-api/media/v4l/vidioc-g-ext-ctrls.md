

######## ioctl VIDIOC_G_EXT_CTRLS, VIDIOC_S_EXT_CTRLS, VIDIOC_TRY_EXT_CTRLS


## 名称


VIDIOC_G_EXT_CTRLS - VIDIOC_S_EXT_CTRLS - VIDIOC_TRY_EXT_CTRLS - 获取或设置多个控件的值，尝试控件值

## 概要



`int ioctl(int fd, VIDIOC_G_EXT_CTRLS, struct v4l2_ext_controls *argp)`


`int ioctl(int fd, VIDIOC_S_EXT_CTRLS, struct v4l2_ext_controls *argp)`


`int ioctl(int fd, VIDIOC_TRY_EXT_CTRLS, struct v4l2_ext_controls *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_ext_controls` 的指针。

## 描述


这些 ioctl 允许调用者原子地获取或设置多个控件。控件 ID 被分组到控件类（见
ctrl-class）中，并且控件数组中的所有控件必须属于同一个控件类。

应用程序必须始终填写 struct `v4l2_ext_controls` 的 `count`、`which`、`controls`
和 `reserved` 字段，并初始化由 `controls` 字段指向的 struct `v4l2_ext_control`
数组。

要获取一组控件的当前值，应用程序初始化每个 struct `v4l2_ext_control` 的 `id`、
`size` 和 `reserved2` 字段，并调用 VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl。
字符串控件还必须设置 `string` 字段。复合类型（`V4L2_CTRL_FLAG_HAS_PAYLOAD`
被设置）的控件必须设置 `ptr` 字段。

如果 `size` 太小以至于无法接收控件结果（仅与字符串等指针类型控件相关），那么
驱动会将 `size` 设置为一个有效值并返回 `ENOSPC` 错误码。你应该将内存重新分配为
这个新大小并重试。对于字符串类型，如果字符串在此期间变长了，同样的问题可能再次
发生。建议先调用 VIDIOC_QUERYCTRL 并使用 `maximum`\ +1 作为新的 `size` 值。这能保证
内存足够。

N 维数组逐行设置和获取。你不能设置部分数组，必须设置或获取所有元素。总大小计算
为 `elems` * `elem_size`。这些值可以通过调用 VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>
获得。

要更改一组控件的值，应用程序初始化每个 struct `v4l2_ext_control` 的 `id`、`size`、
`reserved2` 和 `value/value64/string/ptr` 字段，并调用
VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl。只有当**所有**控件值都有效时，控件才会被设置。

要检查一组控件是否具有正确的值，应用程序初始化每个 struct `v4l2_ext_control` 的
`id`、`size`、`reserved2` 和 `value/value64/string/ptr` 字段，并调用
VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl。错误值是自动调整到有效值还是返回错误，取决于驱动。

当 `id` 或 `which` 无效时，驱动返回 `EINVAL` 错误码。当值越界时，驱动可以选择取
最接近的合法值或返回 `ERANGE` 错误码， whichever 看起来更合适。在第一种情况下，新值
被设置在 struct `v4l2_ext_control` 中。如果新的控件值不合适（例如给定的菜单索引
不被菜单控件支持），那么这也会导致一个 `EINVAL` 错误码错误。

如果 `request_fd` 被设置为一个尚未排队的 request <media-request-api> 文件描述符，
并且 `which` 被设置为 `V4L2_CTRL_WHICH_REQUEST_VAL`，那么这些控件不会在调用
VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 时立即应用，而是被驱动应用于与同一请求关联的缓冲区。
如果设备不支持请求，那么将返回 `EACCES`。如果支持请求但给出了无效的请求文件描述符，
那么将返回 `EINVAL`。

试图为已经排队的请求调用 VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 将导致一个 `EBUSY` 错误。

如果在调用 VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 期间指定了 `request_fd` 并且 `which` 被设置为
`V4L2_CTRL_WHICH_REQUEST_VAL`，那么它将返回请求完成时控件的值。如果请求尚未完成，
那么这将导致一个 `EACCES` 错误。

驱动只会在所有控件值都正确时设置/获取这些控件。这防止了只有部分控件被设置/获取
的情况。只有底层错误（例如失败的 i2c 命令）仍可能导致这种情况。




   \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 标识控件，由应用程序设置。
    - - __u32
      - `size`
      - 此控件负载的总字节大小。
    - - `2` `size` 字段通常为 0，但对于指针控件，应将其设置为包含负载或
	将接收负载的内存的大小。
	如果 VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 发现该值小于存储负载结果所需的值，
	那么它会被设置为足够存储负载结果的值，并返回 `ENOSPC`。

```

	   For string controls, this ``size`` field should
	   not be confused with the length of the string. This field refers
	   to the size of the memory that contains the string. The actual
	   *length* of the string may well be much smaller.
    * - __u32
      - ``reserved2``\ [1]
      - 为未来扩展保留。驱动和应用程序必须将数组设为零。
    * - union {
      - (anonymous)
    * - __s32
      - ``value``
      - 新值或当前值。如果此控件不是 `V4L2_CTRL_TYPE_INTEGER64` 类型且未设置
	`V4L2_CTRL_FLAG_HAS_PAYLOAD`，则有效。
    * - __s64
      - ``value64``
      - 新值或当前值。如果此控件是 `V4L2_CTRL_TYPE_INTEGER64` 类型且未设置
	`V4L2_CTRL_FLAG_HAS_PAYLOAD`，则有效。
    * - char *
      - ``string``
      - 指向字符串的指针。如果此控件是 `V4L2_CTRL_TYPE_STRING` 类型则有效。
    * - __u8 *
      - ``p_u8``
      - 指向无符号 8 位值矩阵控件的指针。如果此控件是 `V4L2_CTRL_TYPE_U8` 类型则有效。
    * - __u16 *
      - ``p_u16``
      - 指向无符号 16 位值矩阵控件的指针。如果此控件是 `V4L2_CTRL_TYPE_U16` 类型则有效。
    * - __u32 *
      - ``p_u32``
      - 指向无符号 32 位值矩阵控件的指针。如果此控件是 `V4L2_CTRL_TYPE_U32` 类型则有效。
    * - __s32 *
      - ``p_s32``
      - 指向有符号 32 位值矩阵控件的指针。如果此控件是 `V4L2_CTRL_TYPE_INTEGER` 类型且
        设置了 `V4L2_CTRL_FLAG_HAS_PAYLOAD` 则有效。
    * - __s64 *
      - ``p_s64``
      - 指向有符号 64 位值矩阵控件的指针。如果此控件是 `V4L2_CTRL_TYPE_INTEGER64` 类型且
        设置了 `V4L2_CTRL_FLAG_HAS_PAYLOAD` 则有效。
    * - struct :c:type:`v4l2_area` *
      - ``p_area``
      - 指向 struct :c:type:`v4l2_area` 的指针。如果此控件是 `V4L2_CTRL_TYPE_AREA` 类型则有效。
    * - struct :c:type:`v4l2_rect` *
      - ``p_rect``
      - 指向 struct :c:type:`v4l2_rect` 的指针。如果此控件是 `V4L2_CTRL_TYPE_RECT` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_sps` *
      - ``p_h264_sps``
      - 指向 struct :c:type:`v4l2_ctrl_h264_sps` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_SPS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_pps` *
      - ``p_h264_pps``
      - 指向 struct :c:type:`v4l2_ctrl_h264_pps` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_PPS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_scaling_matrix` *
      - ``p_h264_scaling_matrix``
      - 指向 struct :c:type:`v4l2_ctrl_h264_scaling_matrix` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_SCALING_MATRIX` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_pred_weights` *
      - ``p_h264_pred_weights``
      - 指向 struct :c:type:`v4l2_ctrl_h264_pred_weights` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_PRED_WEIGHTS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_slice_params` *
      - ``p_h264_slice_params``
      - 指向 struct :c:type:`v4l2_ctrl_h264_slice_params` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_SLICE_PARAMS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_h264_decode_params` *
      - ``p_h264_decode_params``
      - 指向 struct :c:type:`v4l2_ctrl_h264_decode_params` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_H264_DECODE_PARAMS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_fwht_params` *
      - ``p_fwht_params``
      - 指向 struct :c:type:`v4l2_ctrl_fwht_params` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_FWHT_PARAMS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_vp8_frame` *
      - ``p_vp8_frame``
      - 指向 struct :c:type:`v4l2_ctrl_vp8_frame` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_VP8_FRAME` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_mpeg2_sequence` *
      - ``p_mpeg2_sequence``
      - 指向 struct :c:type:`v4l2_ctrl_mpeg2_sequence` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_MPEG2_SEQUENCE` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_mpeg2_picture` *
      - ``p_mpeg2_picture``
      - 指向 struct :c:type:`v4l2_ctrl_mpeg2_picture` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_MPEG2_PICTURE` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_mpeg2_quantisation` *
      - ``p_mpeg2_quantisation``
      - 指向 struct :c:type:`v4l2_ctrl_mpeg2_quantisation` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_MPEG2_QUANTISATION` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_vp9_compressed_hdr` *
      - ``p_vp9_compressed_hdr_probs``
      - 指向 struct :c:type:`v4l2_ctrl_vp9_compressed_hdr` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_VP9_COMPRESSED_HDR` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_vp9_frame` *
      - ``p_vp9_frame``
      - 指向 struct :c:type:`v4l2_ctrl_vp9_frame` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_VP9_FRAME` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hdr10_cll_info` *
      - ``p_hdr10_cll``
      - 指向 struct :c:type:`v4l2_ctrl_hdr10_cll_info` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HDR10_CLL_INFO` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hdr10_mastering_display` *
      - ``p_hdr10_mastering``
      - 指向 struct :c:type:`v4l2_ctrl_hdr10_mastering_display` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HDR10_MASTERING_DISPLAY` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hevc_sps` *
      - ``p_hevc_sps``
      - 指向 struct :c:type:`v4l2_ctrl_hevc_sps` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HEVC_SPS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hevc_pps` *
      - ``p_hevc_pps``
      - 指向 struct :c:type:`v4l2_ctrl_hevc_pps` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HEVC_PPS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hevc_slice_params` *
      - ``p_hevc_slice_params``
      - 指向 struct :c:type:`v4l2_ctrl_hevc_slice_params` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HEVC_SLICE_PARAMS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hevc_scaling_matrix` *
      - ``p_hevc_scaling_matrix``
      - 指向 struct :c:type:`v4l2_ctrl_hevc_scaling_matrix` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HEVC_SCALING_MATRIX` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hevc_decode_params` *
      - ``p_hevc_decode_params``
      - 指向 struct :c:type:`v4l2_ctrl_hevc_decode_params` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HEVC_DECODE_PARAMS` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_av1_sequence` *
      - ``p_av1_sequence``
      - 指向 struct :c:type:`v4l2_ctrl_av1_sequence` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_AV1_SEQUENCE` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_av1_tile_group_entry` *
      - ``p_av1_tile_group_entry``
      - 指向 struct :c:type:`v4l2_ctrl_av1_tile_group_entry` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_AV1_TILE_GROUP_ENTRY` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_av1_frame` *
      - ``p_av1_frame``
      - 指向 struct :c:type:`v4l2_ctrl_av1_frame` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_AV1_FRAME` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_av1_film_grain` *
      - ``p_av1_film_grain``
      - 指向 struct :c:type:`v4l2_ctrl_av1_film_grain` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_AV1_FILM_GRAIN` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hdr10_cll_info` *
      - ``p_hdr10_cll_info``
      - 指向 struct :c:type:`v4l2_ctrl_hdr10_cll_info` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HDR10_CLL_INFO` 类型则有效。
    * - struct :c:type:`v4l2_ctrl_hdr10_mastering_display` *
      - ``p_hdr10_mastering_display``
      - 指向 struct :c:type:`v4l2_ctrl_hdr10_mastering_display` 的指针。如果此控件是
        `V4L2_CTRL_TYPE_HDR10_MASTERING_DISPLAY` 类型则有效。
    * - void *
      - ``ptr``
      - 指向复合类型的指针，该复合类型可以是一个 N 维数组和/或复合类型（控件的类型 >=
	`V4L2_CTRL_COMPOUND_TYPES`）。如果为此控件设置了 `V4L2_CTRL_FLAG_HAS_PAYLOAD`
	则有效。
    * - }
      -

```

   \normalsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - union {
      - (anonymous)
    - - __u32
      - `which`
      - 要获取/设置/尝试的控件的值。
    - - `2` `V4L2_CTRL_WHICH_CUR_VAL` 将返回控件的当前值，
	`V4L2_CTRL_WHICH_DEF_VAL` 将返回控件的默认值，`V4L2_CTRL_WHICH_MIN_VAL` 将返回
	控件的最小值，而 `V4L2_CTRL_WHICH_MAX_VAL` 将返回控件的最大值。
	`V4L2_CTRL_WHICH_REQUEST_VAL` 表示控件值必须从请求中获取，或针对请求尝试/设置。
	在这种情况下，`request_fd` 字段包含应使用的请求的文件描述符。如果设备不支持
	请求，那么将返回 `EACCES`。

	使用 `V4L2_CTRL_WHICH_DEF_VAL`、`V4L2_CTRL_WHICH_MIN_VAL` 或
	`V4L2_CTRL_WHICH_MAX_VAL` 时请注意，你只能获取控件的默认/最小/最大值，不能
	设置或尝试它。

	控件是否支持使用 `V4L2_CTRL_WHICH_MIN_VAL` 和 `V4L2_CTRL_WHICH_MAX_VAL` 查询
	最小值和最大值，由 `V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX` 标志指示。大多数非复合
	控件类型都支持这一点。对于具有复合类型的控件，最小/最大值的定义由控件文档
	提供。如果一个复合控件没有记录最小/最大值的含义，那么查询最小值或最大值将导致
	错误码 -EINVAL。

	为了向后兼容，你也可以在这里使用控件类（见 ctrl-class）。在这种情况下，所有
	控件必须属于该控件类。这种用法已被弃用，请改用 `V4L2_CTRL_WHICH_CUR_VAL`。
	有一些非常老的驱动尚不支持 `V4L2_CTRL_WHICH_CUR_VAL`，需要在那里指定控件类。
	你可以通过将 `which` 设为 `V4L2_CTRL_WHICH_CUR_VAL` 并以 count 为 0 调用
	VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 来测试此类驱动。如果失败，则该驱动不支持
	`V4L2_CTRL_WHICH_CUR_VAL`。
    - - __u32
      - `ctrl_class`
      - 为向后兼容保留的弃用名称。请改用 `which`。
    - - }
      -
    - - __u32
      - `count`
      - controls 数组中的控件数量。也可以为零。
    - - __u32
      - `error_idx`
      - 失败控件的索引。出错时由驱动设置。
    - - `2` 如果错误与某个特定控件相关联，那么 `error_idx` 被设置为该控件的索引。
	如果错误与特定控件无关，或者验证步骤失败（见下文），那么 `error_idx` 被设置为
	`count`。如果 ioctl 返回 0（成功），该值未定义。

	在从硬件读取/写入硬件之前会进行一个验证步骤：这会检查列表中的所有控件是否都是
	有效的控件，是否没有尝试写入只读控件或从只写控件读取，以及任何其他可以在不访问
	硬件的情况下完成的事前检查。此步骤所做的确切验证是驱动相关的，因为某些检查可能
	需要访问某些设备的硬件，从而无法事前完成。然而，驱动应尽最大努力进行尽可能多的
	事前检查。

	这样做是为了避免因容易避免的问题而使硬件处于不一致状态。但它导致了另一个问题：
	应用程序需要知道错误是来自验证步骤（意味着未触及硬件）还是在实际从硬件读取/写入
	硬件期间发生的错误。

	事后看来相当糟糕的解决方案是将验证失败时的 `error_idx` 设为 `count`。这有一个
	不幸的副作用，即无法看到哪个控件未通过验证。如果验证成功并且错误发生在访问硬件
	期间，那么 `error_idx` 小于 `count`，并且只有到 `error_idx-1` 的控件被正确地
	读取或写入，剩余控件的状态未定义。

	由于 VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 不访问硬件，因此也不需要以这种特殊方式处理
	验证步骤，所以 `error_idx` 将被设为未通过验证步骤的控件，而不是 `count`。这意味着
	如果 VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 以 `error_idx` 设为 `count` 失败，那么你可以调用
	VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 来尝试发现实际未通过验证步骤的控件。不幸的是，
	VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 没有对应的 `TRY`。
    - - __s32
      - `request_fd`
      - 此操作要使用的请求的文件描述符。仅当 `which` 被设为
	`V4L2_CTRL_WHICH_REQUEST_VAL` 时有效。如果设备不支持请求，那么将返回 `EACCES`。
	如果支持请求但给出了无效的请求文件描述符，那么将返回 `EINVAL`。
    - - __u32
      - `reserved`\ [^1^]
      - 为未来扩展保留。

	驱动和应用程序必须将数组设为零。
    - - struct `v4l2_ext_control` *
      - `controls`
      - 指向 `count` 个 v4l2_ext_control 结构数组的指针。

	如果 `count` 等于零则忽略。




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CTRL_CLASS_USER`
      - 0x980000
      - 包含用户控件的类。这些控件在 control 中描述。所有可以使用
	VIDIOC_S_CTRL <VIDIOC_G_CTRL> 和 VIDIOC_G_CTRL <VIDIOC_G_CTRL> ioctl 设置的控件都属于此类。
    - - `V4L2_CTRL_CLASS_CODEC`
      - 0x990000
      - 包含有状态编解码器控件的类。这些控件在 codec-controls 中描述。
    - - `V4L2_CTRL_CLASS_CAMERA`
      - 0x9a0000
      - 包含摄像头控件的类。这些控件在 camera-controls 中描述。
    - - `V4L2_CTRL_CLASS_FM_TX`
      - 0x9b0000
      - 包含 FM 发射器（FM TX）控件的类。这些控件在 fm-tx-controls 中描述。
    - - `V4L2_CTRL_CLASS_FLASH`
      - 0x9c0000
      - 包含闪光灯设备控件的类。这些控件在 flash-controls 中描述。
    - - `V4L2_CTRL_CLASS_JPEG`
      - 0x9d0000
      - 包含 JPEG 压缩控件的类。这些控件在 jpeg-controls 中描述。
    - - `V4L2_CTRL_CLASS_IMAGE_SOURCE`
      - 0x9e0000
      - 包含图像源控件的类。这些控件在 image-source-controls 中描述。
    - - `V4L2_CTRL_CLASS_IMAGE_PROC`
      - 0x9f0000
      - 包含图像处理控件的类。这些控件在 image-process-controls 中描述。
    - - `V4L2_CTRL_CLASS_FM_RX`
      - 0xa10000
      - 包含 FM 接收器（FM RX）控件的类。这些控件在 fm-rx-controls 中描述。
    - - `V4L2_CTRL_CLASS_RF_TUNER`
      - 0xa20000
      - 包含 RF 调谐器控件的类。这些控件在 rf-tuner-controls 中描述。
    - - `V4L2_CTRL_CLASS_DETECT`
      - 0xa30000
      - 包含运动或物体检测控件的类。这些控件在 detect-controls 中描述。
    - - `V4L2_CTRL_CLASS_CODEC_STATELESS`
      - 0xa40000
      - 包含无状态编解码器控件的类。这些控件在 codec-stateless-controls 中描述。
    - - `V4L2_CTRL_CLASS_COLORIMETRY`
      - 0xa50000
      - 包含色度学控件的类。这些控件在 colorimetry-controls 中描述。

## 返回值


成功时返回 0，出错时返回 -1 并且 `errno` 变量被适当设置。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    struct `v4l2_ext_control` 的 `id` 无效，或 struct `v4l2_ext_controls` 的
    `which` 无效，或 struct `v4l2_ext_control` 的 `value` 不合适（例如给定的菜单
    索引不被驱动支持），或 `which` 字段被设为 `V4L2_CTRL_WHICH_REQUEST_VAL` 但给定的
    `request_fd` 无效或 `V4L2_CTRL_WHICH_REQUEST_VAL` 不被内核支持。
    如果两个或更多控件值冲突，VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 和
    VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl 也会返回此错误码。

ERANGE
    struct `v4l2_ext_control` 的 `value` 越界。

EBUSY
    控件暂时不可更改，可能是因为另一个应用程序接管了此控件所属的设备功能，或（如果
    `which` 字段被设为 `V4L2_CTRL_WHICH_REQUEST_VAL`）请求已排队但尚未完成。

ENOSPC
    为控件负载保留的空间不足。`size` 字段被设为一个足够存储负载的值，并返回此错误码。

EACCES
    试图尝试或设置只读控件，或获取只写控件，或从尚未完成的请求中获取控件。

    或者 `which` 字段被设为 `V4L2_CTRL_WHICH_REQUEST_VAL` 但设备不支持请求。

    或者如果有试图设置一个非活动控件的操作，且驱动无法缓存新值直到该控件再次活动。
