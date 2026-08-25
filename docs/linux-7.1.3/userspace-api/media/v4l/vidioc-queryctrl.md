


######## ioctls VIDIOC_QUERYCTRL, VIDIOC_QUERY_EXT_CTRL and VIDIOC_QUERYMENU


## Name


VIDIOC_QUERYCTRL - VIDIOC_QUERY_EXT_CTRL - VIDIOC_QUERYMENU - 枚举控件和菜单控件项

## Synopsis


`int ioctl(int fd, int VIDIOC_QUERYCTRL, struct v4l2_queryctrl *argp)`


`int ioctl(int fd, VIDIOC_QUERY_EXT_CTRL, struct v4l2_query_ext_ctrl *argp)`


`int ioctl(int fd, VIDIOC_QUERYMENU, struct v4l2_querymenu *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_queryctrl`、`v4l2_query_ext_ctrl`
    `v4l2_querymenu` 的指针（取决于具体的 ioctl）
## Description


为了查询一个控件的属性，应用程序设置 struct v4l2_queryctrl <v4l2-queryctrl> `id` 字段，并调用 `VIDIOC_QUERYCTRL` ioctl，传入指向该结构的指针。驱动填充结构的
其余部分，或者当 `id` 无效时返`EINVAL` 错误码
可以通过`V4L2_CID_BASE` 开始、到（不含）`V4L2_CID_LASTP1` 为止，以连续`id` 值调`VIDIOC_QUERYCTRL` 来枚举控件。如果此范围内的某个控件不受支持，驱可能返回 `EINVAL`。进一步，应用程序可以通过`V4L2_CID_PRIVATE_BASE` 开始并递增
`id`，直到驱动返`EINVAL`，来枚举本规范未定义的私有控件
在这两种情况下，当驱动在 `flags` 字段中设置了 `V4L2_CTRL_FLAG_DISABLED` 标志时，
该控件被永久禁用，应用程序应忽略它[#f1]_

当应用程序将 `id` `V4L2_CTRL_FLAG_NEXT_CTRL` OR 运算时，驱动返回下一个受支持非复合控件，如果没有则返`EINVAL`。此外，可以指定 `V4L2_CTRL_FLAG_NEXT_COMPOUND`
标志来枚举所有的复合控件（即类型 `V4L2_CTRL_COMPOUND_TYPES` 或数组控件，换言包含多个值的控件）。同时指`V4L2_CTRL_FLAG_NEXT_CTRL` `V4L2_CTRL_FLAG_NEXT_COMPOUND` 以枚举所有控件（无论是否复合）。尚不支持这些标志的
驱动总是返回 `EINVAL`
引入 `VIDIOC_QUERY_EXT_CTRL` ioctl 是为了更好地支持可以使用复合类型的控件，并暴无法struct v4l2_queryctrl <v4l2-queryctrl> 中返回（因为该结构已满）的额外控信息
`VIDIOC_QUERY_EXT_CTRL` 的使用方式与 `VIDIOC_QUERYCTRL` 相同，只`reserved`
数组也必须被置零
菜单控件需要额外的信息：菜单项的名称。为了查询它们，应用程序设置 struct
v4l2_querymenu <v4l2-querymenu> `id` `index` 字段，并调用 `VIDIOC_QUERYMENU`
ioctl，传入指向该结构的指针。驱动填充结构的其余部分，或者当 `id` `index` 无效返回 `EINVAL` 错误码。菜单项通过以从 struct v4l2_queryctrl <v4l2-queryctrl> `minimum` `maximum`（含）的连续 `index` 值调`VIDIOC_QUERYMENU` 来枚举

   `VIDIOC_QUERYMENU` 有可能对 `minimum` `maximum` 之间的某些索引返   `EINVAL` 错误码。在这种情况下，该特定的菜单项不受此驱动支持。另请注意，
   `minimum` 值不一定为 0
另请参见 control 中的示例


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 标识控件，由应用程序设置。预定义 ID 参见 control-id。当 ID 	V4L2_CTRL_FLAG_NEXT_CTRL OR 运算时，驱动清除该标志并返回具有更高 ID 	第一个控件。尚不支持此标志的驱动总是返回 `EINVAL` 错误码    - - __u32
      - `type`
      - 控件类型，参`v4l2_ctrl_type`    - - __u8
      - `name`\ [^32^]
      - 控件名称，一个以 NUL 结尾ASCII 字符串。此信息供用户使用    - - __s32
      - `minimum`
      - 最小值，含。该字段给出控件的一个下界。关于每种可能的控件类型应如何使	最小值，参见枚举 `v4l2_ctrl_type`。注意这是一个有符号32 位值    - - __s32
      - `maximum`
      - 最大值，含。该字段给出控件的一个上界。关于每种可能的控件类型应如何使	最大值，参见枚举 `v4l2_ctrl_type`。注意这是一个有符号32 位值    - - __s32
      - `step`
      - 该字段给出控件的步长。关于每种可能的控件类型应如何使用步长值，参见枚举
	`v4l2_ctrl_type`。注意这是一个无符号32 位值
	通常驱动不应缩放硬件控制值。例如当 `name` `id` 暗示了某个特定单位，	硬件实际上只接受该单位的整数倍时，可能就有此必要。如果是这样，驱动必须注	在缩放时正确地对值进行四舍五入，以使错误不会在反复的写循环中累积
	该字段给出实际影响硬件的整数控件的最小变化量。当用户可以通过键盘GUI 按钮
	（而非滑块）改变控件时，常常需要此信息。例如，当硬件寄存器接受0-511，	驱动报告 0-65535 时，step 应为 128
	注意，尽管是有符号的，但 step 值应当始终为正    - - __s32
      - `default_value`
      - `V4L2_CTRL_TYPE_INTEGER`、`_BOOLEAN`、`_BITMASK`、`_MENU` 	`_INTEGER_MENU` 控件的默认值。对其他类型的控件无效
```

	   Drivers reset controls to their default value only when
	   the driver is first loaded, never afterwards.
    * - __u32
      - ``flags``
      - Control flags, see :ref:`control-flags`.
    * - __u32
      - ``reserved``\ [2]
      - Reserved for future extensions. Drivers must set the array to
	zero.


```




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 标识控件，由应用程序设置。预定义 ID 参见 control-id。当 ID 	V4L2_CTRL_FLAG_NEXT_CTRL OR 运算时，驱动清除该标志并返回具有更高 ID 	第一个非复合控件。当 ID `V4L2_CTRL_FLAG_NEXT_COMPOUND` OR 运算时，驱动
	清除该标志并返回具有更高 ID 的第一个复合控件。同时设置两者以获取具有更高 ID
	的第一个控件（无论是否复合）    - - __u32
      - `type`
      - 控件类型，参`v4l2_ctrl_type`    - - char
      - `name`\ [^32^]
      - 控件名称，一个以 NUL 结尾ASCII 字符串。此信息供用户使用    - - __s64
      - `minimum`
      - 最小值，含。该字段给出控件的一个下界。关于每种可能的控件类型应如何使	最小值，参见枚举 `v4l2_ctrl_type`。注意这是一个有符号64 位值    - - __s64
      - `maximum`
      - 最大值，含。该字段给出控件的一个上界。关于每种可能的控件类型应如何使	最大值，参见枚举 `v4l2_ctrl_type`。注意这是一个有符号64 位值    - - __u64
      - `step`
      - 该字段给出控件的步长。关于每种可能的控件类型应如何使用步长值，参见枚举
	`v4l2_ctrl_type`。注意这是一个无符号64 位值
	通常驱动不应缩放硬件控制值。例如当 `name` `id` 暗示了某个特定单位，	硬件实际上只接受该单位的整数倍时，可能就有此必要。如果是这样，驱动必须注	在缩放时正确地对值进行四舍五入，以使错误不会在反复的写循环中累积
	该字段给出实际影响硬件的整数控件的最小变化量。当用户可以通过键盘GUI 按钮
	（而非滑块）改变控件时，常常需要此信息。例如，当硬件寄存器接受0-511，	驱动报告 0-65535 时，step 应为 128    - - __s64
      - `default_value`
      - `V4L2_CTRL_TYPE_INTEGER`、`_INTEGER64`、`_BOOLEAN`、`_BITMASK`、`_MENU`	`_INTEGER_MENU`、`_U8` `_U16` 控件的默认值。对其他类型的控件无效
```

	   Drivers reset controls to their default value only when
	   the driver is first loaded, never afterwards.
    * - __u32
      - ``flags``
      - Control flags, see :ref:`control-flags`.
    * - __u32
      - ``elem_size``
      - The size in bytes of a single element of the array. Given a char
	pointer ``p`` to a 3-dimensional array you can find the position
	of cell ``(z, y, x)`` as follows:
	``p + ((z * dims[1] + y) * dims[0] + x) * elem_size``.
	``elem_size`` is always valid, also when the control isn't an
	array. For string controls ``elem_size`` is equal to
	``maximum + 1``.
    * - __u32
      - ``elems``
      - The number of elements in the N-dimensional array. If this control
	is not an array, then ``elems`` is 1. The ``elems`` field can
	never be 0.
    * - __u32
      - ``nr_of_dims``
      - The number of dimension in the N-dimensional array. If this
	control is not an array, then this field is 0.
    * - __u32
      - ``dims[V4L2_CTRL_MAX_DIMS]``
      - The size of each dimension. The first ``nr_of_dims`` elements of
	this array must be non-zero, all remaining elements must be zero.
    * - __u32
      - ``reserved``\ [32]
      - Reserved for future extensions. Applications and drivers must set
	the array to zero.


```




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 标识控件，由应用程序根据相应struct v4l2_queryctrl <v4l2-queryctrl>
	`id` 设置    - - __u32
      - `index`
      - 菜单项的索引，从零开始，由应用程序设置    - - union {
      - (anonymous)
    - - __u8
      - `name`\ [^32^]
      - 菜单项名称，一个以 NUL 结尾ASCII 字符串。此信息供用户使用。该字段	`V4L2_CTRL_TYPE_MENU` 类型的控件有效    - - __s64
      - `value`
      - 整数菜单项的值。该字段`V4L2_CTRL_TYPE_INTEGER_MENU` 类型的控件有效    - - }
      -
    - - __u32
      - `reserved`
      - 为将来扩展保留。驱动必须将数组置零


   \footnotesize




    :header-rows:  1
    :stub-columns: 0
    :widths:       30 5 5 5 55

    - - Type
      - `minimum`
      - `step`
      - `maximum`
      - Description
    - - `V4L2_CTRL_TYPE_INTEGER`
      - any
      - any
      - any
      - 一个取值范围从 minimum maximum（含）的整数值控件。step 值表示取值之间的
	增量    - - `V4L2_CTRL_TYPE_BOOLEAN`
      - 0
      - 1
      - 1
      - 一个布尔值控件。零对应“disabled（禁用）”，一对应“enabled（启用）”    - - `V4L2_CTRL_TYPE_MENU`
      - 鈮?0
      - 1
      - N-1
      - 该控件有一个包N 个选项的菜单。菜单项的名称可以通过 `VIDIOC_QUERYMENU`
	ioctl 枚举    - - `V4L2_CTRL_TYPE_INTEGER_MENU`
      - 鈮?0
      - 1
      - N-1
      - 该控件有一个包N 个选项的菜单。菜单项的值可以通过 `VIDIOC_QUERYMENU`
	ioctl 枚举。这`V4L2_CTRL_TYPE_MENU` 类似，只是菜单项是带符号64 	整数，而非字符串    - - `V4L2_CTRL_TYPE_BITMASK`
      - 0
      - n/a
      - any
      - 一个位掩码字段。最大值是可以使用的一组位，所有其他位应为 0。最大值被解释	一__u32，允许使用位掩码中的31 位    - - `V4L2_CTRL_TYPE_BUTTON`
      - 0
      - 0
      - 0
      - 一个在设置时执行某个动作的控件。驱动必须忽略随 `VIDIOC_S_CTRL` 传入的值，
	并在 `VIDIOC_G_CTRL` 尝试时返`EACCES` 错误码    - - `V4L2_CTRL_TYPE_INTEGER64`
      - any
      - any
      - any
      - 一64 位整数值控件。最小值、最大值和步长无法使用 `VIDIOC_QUERYCTRL`
	查询。只`VIDIOC_QUERY_EXT_CTRL` 可以检64 位的最小最大步长值，
	在使`VIDIOC_QUERYCTRL` 时应将它们解释为 n/a    - - `V4L2_CTRL_TYPE_STRING`
      - 鈮?0
      - 鈮?1
      - 鈮?0
      - 字符串的最小和最大长度。步长意味着字符串的长度必须为（minimum + N * step	个字符，其中 N 0。这些长度不包含终止零，因此为了将长度为 8 的字符串传给
	VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>，你需要将 struct
	`v4l2_ext_control` `size` 字段设置9。对VIDIOC_G_EXT_CTRLS
	<VIDIOC_G_EXT_CTRLS>，你可以`size` 字段设置`maximum` + 1。使用何	字符编码取决于字符串控件本身，并应作为控件文档的一部分    - - `V4L2_CTRL_TYPE_CTRL_CLASS`
      - n/a
      - n/a
      - n/a
      - 这不是一个控件。当以等于控件类代码（参ctrl-class）的控件 ID 1 调用
	`VIDIOC_QUERYCTRL` 时，ioctl 返回该控件类的名称以及此控件类型。不支持	特性的较旧驱动返回 `EINVAL` 错误码    - - `V4L2_CTRL_TYPE_U8`
      - any
      - any
      - any
      - 一个取值范围从 minimum maximum（含）的无符8 位值控件。step 值表	取值之间的增量    - - `V4L2_CTRL_TYPE_U16`
      - any
      - any
      - any
      - 一个取值范围从 minimum maximum（含）的无符16 位值控件。step 值表	取值之间的增量    - - `V4L2_CTRL_TYPE_U32`
      - any
      - any
      - any
      - 一个取值范围从 minimum maximum（含）的无符32 位值控件。step 值表	取值之间的增量    - - `V4L2_CTRL_TYPE_MPEG2_QUANTISATION`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_mpeg2_quantisation`，包含用于无状态视频解码器	MPEG-2 量化矩阵    - - `V4L2_CTRL_TYPE_MPEG2_SEQUENCE`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_mpeg2_sequence`，包含用于无状态视频解码器MPEG-2
	序列参数    - - `V4L2_CTRL_TYPE_MPEG2_PICTURE`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_mpeg2_picture`，包含用于无状态视频解码器MPEG-2
	图像参数    - - `V4L2_CTRL_TYPE_AREA`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_area`，包含矩形区域的宽度和高度。单位取决于具体用例    - - `V4L2_CTRL_TYPE_RECT`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_rect`，包含由左上角位置、宽度和高度描述的矩形。单	取决于具体用例。对 `V4L2_CTRL_WHICH_MIN_VAL` `V4L2_CTRL_WHICH_MAX_VAL`
	的支持是可选的，取决于 `V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX` 标志。关于如	解释最小值和最大值，请参见具体控件的文档    - - `V4L2_CTRL_TYPE_H264_SPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_h264_sps`，包含用于无状态视频解码器H264 序列
	参数    - - `V4L2_CTRL_TYPE_H264_PPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_h264_pps`，包含用于无状态视频解码器H264 图像
	参数    - - `V4L2_CTRL_TYPE_H264_SCALING_MATRIX`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_h264_scaling_matrix`，包含用于无状态视频解码器	H264 缩放矩阵    - - `V4L2_CTRL_TYPE_H264_SLICE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_h264_slice_params`，包含用于无状态视频解码器	H264 切片参数    - - `V4L2_CTRL_TYPE_H264_DECODE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_h264_decode_params`，包含用于无状态视频解码器	H264 解码参数    - - `V4L2_CTRL_TYPE_FWHT_PARAMS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_fwht_params`，包含用于无状态视频解码器FWHT 参数    - - `V4L2_CTRL_TYPE_HEVC_SPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_sps`，包含用于无状态视频解码器HEVC 序列
	参数集    - - `V4L2_CTRL_TYPE_HEVC_PPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_pps`，包含用于无状态视频解码器HEVC 图像
	参数集    - - `V4L2_CTRL_TYPE_HEVC_SLICE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_slice_params`，包含用于无状态视频解码器HEVC
	切片参数    - - `V4L2_CTRL_TYPE_HEVC_SCALING_MATRIX`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_scaling_matrix`，包含用于无状态视频解码器	HEVC 缩放矩阵    - - `V4L2_CTRL_TYPE_VP8_FRAME`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_vp8_frame`，包含用于无状态视频解码器VP8 帧参数    - - `V4L2_CTRL_TYPE_HEVC_DECODE_PARAMS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_decode_params`，包含用于无状态视频解码器HEVC
	解码参数    - - `V4L2_CTRL_TYPE_HEVC_EXT_SPS_LT_RPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_ext_sps_lt_rps`，包含用于无状态视频解码器	HEVC 扩展长期 RPS    - - `V4L2_CTRL_TYPE_HEVC_EXT_SPS_ST_RPS`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_hevc_ext_sps_st_rps`，包含用于无状态视频解码器	HEVC 扩展短期 RPS    - - `V4L2_CTRL_TYPE_VP9_COMPRESSED_HDR`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_vp9_compressed_hdr`，包含用于无状态视频解码器VP9
	概率更新    - - `V4L2_CTRL_TYPE_VP9_FRAME`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_vp9_frame`，包含用于无状态视频解码器VP9 帧解	参数    - - `V4L2_CTRL_TYPE_AV1_SEQUENCE`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_av1_sequence`，包含用于无状态视频解码器AV1 Sequence
	OBU 解码参数    - - `V4L2_CTRL_TYPE_AV1_TILE_GROUP_ENTRY`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_av1_tile_group_entry`，包含用于无状态视频解码器	AV1 Tile Group OBU 解码参数    - - `V4L2_CTRL_TYPE_AV1_FRAME`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_av1_frame`，包含用于无状态视频解码器AV1 Frame/Frame
	Header OBU 解码参数    - - `V4L2_CTRL_TYPE_AV1_FILM_GRAIN`
      - n/a
      - n/a
      - n/a
      - 一struct `v4l2_ctrl_av1_film_grain`，包含用于无状态视频解码器AV1 胶片
	颗粒参数

   \normalsize




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CTRL_FLAG_DISABLED`
      - 0x0001
      - 该控件被永久禁用，应用程序应忽略它。任何尝试改变该控件的操作都将导	`EINVAL` 错误码    - - `V4L2_CTRL_FLAG_GRABBED`
      - 0x0002
      - 该控件暂时不可更改，例如因为另一个应用程序接管了对相应资源的控制。此类控	在用户界面中可能会以特殊方式显示。尝试改变该控件可能导致 `EBUSY` 错误码    - - `V4L2_CTRL_FLAG_READ_ONLY`
      - 0x0004
      - 该控件是永久只读的。任何尝试改变该控件的操作都将导`EINVAL` 错误码    - - `V4L2_CTRL_FLAG_UPDATE`
      - 0x0008
      - 一个提示，表明改变此控件可能会影响同一控件类中其他控件的值。应用程序应相应	更新其用户界面    - - `V4L2_CTRL_FLAG_INACTIVE`
      - 0x0010
      - 该控件不适用于当前配置，在用户界面中应相应地显示。例如，当使用另一个控件选择	MPEG 音频编码级别 1 时，可能会在 MPEG 音频级别 2 码率控件上设置此标志    - - `V4L2_CTRL_FLAG_SLIDER`
      - 0x0020
      - 一个提示，表明该控件在用户界面中最适合表示为滑块式的元素    - - `V4L2_CTRL_FLAG_WRITE_ONLY`
      - 0x0040
      - 该控件是永久只写的。任何尝试读取该控件的操作都将导`EACCES` 错误码。此标志
	通常出现在相对控件或动作控件上，其中写入一个值将导致设备执行给定动作（例	电机控制），但无法返回有意义的值    - - `V4L2_CTRL_FLAG_VOLATILE`
      - 0x0080
      - 该控件是易变的（volatile），这意味着控件的值会持续变化。一个典型的例子是当设备
	处于自动增益模式时的当前增益值。在这种情况下，硬件根据可能随时间变化的照明
	条件计算增益值
```

	   Setting a new value for a volatile control will be ignored
	   unless
	   :ref:`V4L2_CTRL_FLAG_EXECUTE_ON_WRITE <FLAG_EXECUTE_ON_WRITE>`
	   is also set.
	   Setting a new value for a volatile control will *never* trigger a
	   :ref:`V4L2_EVENT_CTRL_CH_VALUE <ctrl-changes-flags>` event.
    * - ``V4L2_CTRL_FLAG_HAS_PAYLOAD``
      - 0x0100
      - This control has a pointer type, so its value has to be accessed
	using one of the pointer fields of struct
	:c:type:`v4l2_ext_control`. This flag is set
	for controls that are an array, string, or have a compound type.
	In all cases you have to set a pointer to memory containing the
	payload of the control.
    * .. _FLAG_EXECUTE_ON_WRITE:

      - ``V4L2_CTRL_FLAG_EXECUTE_ON_WRITE``
      - 0x0200
      - The value provided to the control will be propagated to the driver
	even if it remains constant. This is required when the control
	represents an action on the hardware. For example: clearing an
	error flag or triggering the flash. All the controls of the type
	``V4L2_CTRL_TYPE_BUTTON`` have this flag set.
    * .. _FLAG_MODIFY_LAYOUT:

      - ``V4L2_CTRL_FLAG_MODIFY_LAYOUT``
      - 0x0400
      - Changing this control value may modify the layout of the
        buffer (for video devices) or the media bus format (for sub-devices).

	A typical example would be the ``V4L2_CID_ROTATE`` control.

	Note that typically controls with this flag will also set the
	``V4L2_CTRL_FLAG_GRABBED`` flag when buffers are allocated or
	streaming is in progress since most drivers do not support changing
	the format in that case.
    * - ``V4L2_CTRL_FLAG_DYNAMIC_ARRAY``
      - 0x0800
      - This control is a dynamically sized 1-dimensional array. It
        behaves the same as a regular array, except that the number
	of elements as reported by the ``elems`` field is between 1 and
	``dims[0]``. So setting the control with a differently sized
	array will change the ``elems`` field when the control is
	queried afterwards.
    * - ``V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX``
      - 0x1000
      - This control supports getting minimum and maximum values using
        vidioc_g_ext_ctrls with V4L2_CTRL_WHICH_MIN/MAX_VAL.


```
## Return Value


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述
EINVAL
    struct v4l2_queryctrl <v4l2-queryctrl> `id` 无效。struct
    v4l2_querymenu <v4l2-querymenu> `id` 无效，或 `index` 超出范围（小    `minimum` 或大`maximum`），或者该特定的菜单项不受驱动支持
EACCES
    尝试读取一个只写控件
   `V4L2_CTRL_FLAG_DISABLED` 有两个用途：驱动可以跳过硬件不支持的预定义控   （尽管返`EINVAL` 也同样可以），或者在硬件检测后禁用预定义和私有控件，而无需
   重新排序控件数组和索引的麻烦（`EINVAL` 不能用于跳过私有控件，因为那会过早地
   结束枚举）