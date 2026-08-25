
## Linux USB 视频类（UVC）驱

本文件记UVC 驱动中一些驱动特有的内容，例如驱动专用的 ioctl 以及实现说明
问题和意见可以发送到 Linux UVC 开发邮件列linux-media@vger.kernel.org

### 扩展单元（XU）支

#### 简

UVC 规范允许通过扩展单元（XU）实现厂商自定义的扩展。Linux UVC 驱动通过两种独立的机制来支持扩展单元控制（XU 控制）：

  - 通过XU 控制映射V4L2 控制
  - 通过驱动专用ioctl 接口

第一种机制允许通用V4L2 应用程序在使用某XU 控制时，将其映射V4L2 控制上，这些控制随后会在常规的控制枚举过程中出现
第二种机制需要应用程序具uvcvideo 相关的专门知识才能访XU 控制，但它将整个 UVC XU 概念暴露给用户空间，以获得最大的灵活性
这两种机制互为补充，下文将分别详细介绍

#### 控制映射


UVC 驱动为用户空间应用程序提供了一种在运行时定义所谓“控制映射”的 API。这些映射允许将单个 XU 控制或其字节范围映射到新V4L2 控制。这类控制的表现和行为与普V4L2 控制（即亮度、对比度等标准控制）完全一致。不过，对这V4L2 控制的读或写会触发对相应 XU 控制的读或写
用于创建这些控制映射ioctl 名为 UVCIOC_CTRL_MAP。早期驱动版本（0.2.0 之前）需要事先使用另一ioctl（UVCIOC_CTRL_ADD）将 XU 控制信息传递给 UVC 驱动。这已不再必要，因为较新uvcvideo 版本会直接从设备查询该信息
关于 UVCIOC_CTRL_MAP ioctl 的详细信息，请参阅下文“IOCTL 参考”一节

3. 驱动专用XU 控制接口

对于需要直接访XU 控制的应用程序（例如出于测试、固件上传或访问二进制控制的目的），提供了第二种访问 XU 控制的机制，其形式为驱动专用ioctl，即 UVCIOC_CTRL_QUERY
对该 ioctl 的调用允许应用程序向 UVC 驱动发送查询，这些查询会直接映射到底层UVC 控制请求
为了发起这样的请求，需要先知道该控制的扩展单元 ID（UVC unit ID）和控制选择子（control selector）。这些信息要么需要在应用程序中硬编码，要么需要通过其他方式查询，例如解UVC 描述符，或者在可用的情况下使用媒体控制API 来枚举设备的实体（entity）
除非已经知道控制的大小，否则有必要先发起一UVC_GET_LEN 请求，以便分配足够大的缓冲区并将缓冲区大小设置为正确的值。类似地，要确认 UVC_GET_CUR UVC_SET_CUR 是否对某个给定控制是有效的请求，应当先发起一UVC_GET_INFO 请求。结果字节的0 位（支持 GET）和1 位（支持 SET）指示哪些请求是有效的
随着 UVCIOC_CTRL_QUERY ioctl 的加入，UVCIOC_CTRL_GET UVCIOC_CTRL_SET 这两ioctl 已经过时，因为它们的功能只是前者功能的子集。目前它们仍被支持，但我们鼓励应用程序开发者改UVCIOC_CTRL_QUERY
关于 UVCIOC_CTRL_QUERY ioctl 的详细信息，请参阅下文“IOCTL 参考”一节

#### 瀹夊叏鎬。

API 目前不提供细粒度的访问控制机制。UVCIOC_CTRL_ADD UVCIOC_CTRL_MAP 这两ioctl 需要超级用户权限
欢迎提出改进建议

#### 调试


为了调试XU 控制或一般控制相关的问题，建议在模块参数 'trace' 中启UVC_TRACE_CONTROL 位。这会使额外的输出被写入系统日志

#### IOCTL 参

##### UVCIOC_CTRL_MAP —UVC 控制映射V4L2 控制


参数：struct uvc_xu_control_mapping

**描述**
	ioctl UVC 控制或其一部分与某V4L2 控制之间创建映射。一旦定义好映射，用户空间应用程序就可以通过 V4L2 控制 API 访问厂商自定义的 UVC 控制
	要创建映射，应用程序需要用一个已经由 UVCIOC_CTRL_ADD 定义的现UVC 控制的信息，以及一个新V4L2 控制，来填充 uvc_xu_control_mapping 结构体
	一UVC 控制可以映射到多V4L2 控制。例如，一UVC 平移/倾斜（pan/tilt）控制可以被映射到独立的平移和倾斜 V4L2 控制。UVC 控制使用 'size' 'offset' 字段被划分为互不重叠的字段，然后分别映射V4L2 控制
	对于有符号整数的 V4L2 控制，data_type 字段应设UVC_CTRL_DATA_TYPE_SIGNED。其他取值目前被忽略
**返回*
	成功时返0。出错时返回 -1，并相应地设errno
	ENOMEM
		没有足够的内存来执行该操作	EPERM
		权限不足（需要超级用户权限）	EINVAL
		不存在这样的 UVC 控制	EOVERFLOW
		请求offset size 会使 UVC 控制溢出	EEXIST
		映射已存在
**数据类型**

 - struct uvc_xu_control_mapping

	__u32	id		V4L2 控制标识	__u8	name[^32^]	V4L2 控制名称
	__u8	entity[^16^]	UVC 扩展单元 GUID
	__u8	selector	UVC 控制选择	__u8	size		V4L2 控制大小（以位为单位	__u8	offset		V4L2 控制偏移（以位为单位	enum v4l2_ctrl_type
		v4l2_type	V4L2 控制类型
	enum uvc_control_data_type
		data_type	UVC 控制数据类型
	struct uvc_menu_info
		*menu_info	菜单项数组（仅用于菜单型控制	__u32	menu_count	菜单项数量（仅用于菜单型控制
 - struct uvc_menu_info

	__u32	value		设备使用的菜单项	__u8	name[^32^]	菜单项名

 - enum uvc_control_data_type

	UVC_CTRL_DATA_TYPE_RAW		原始控制（字节数组）
	UVC_CTRL_DATA_TYPE_SIGNED	有符号整	UVC_CTRL_DATA_TYPE_UNSIGNED	无符号整	UVC_CTRL_DATA_TYPE_BOOLEAN	布尔	UVC_CTRL_DATA_TYPE_ENUM		枚举
	UVC_CTRL_DATA_TYPE_BITMASK	位掩	UVC_CTRL_DATA_TYPE_RECT		矩形区域


##### UVCIOC_CTRL_QUERY —查询一UVC XU 控制


参数：struct uvc_xu_control_query

**描述**
	ioctl 查询一个由其扩展单ID 和控制选择子标识的 UVC XU 控制
	有多种不同的查询可用，它们与 UVC 规范中描述的底层控制请求紧密对应。这些请求包括：

	UVC_GET_CUR
		获取控制的当前值	UVC_GET_MIN
		获取控制的最小值	UVC_GET_MAX
		获取控制的最大值	UVC_GET_DEF
		获取控制的默认值	UVC_GET_RES
		查询控制的分辨率，即允许的控制值的步长大小	UVC_GET_LEN
		查询控制的大小（以字节为单位）	UVC_GET_INFO
		查询控制信息位图，指示是否支get/set 请求	UVC_SET_CUR
		更新控制的值
	应用程序必须'size' 字段设置为该控制的正确长度。例外情况是 UVC_GET_LEN UVC_GET_INFO 查询，它们的 size 必须分别设为 2 1data' 字段必须指向一个有效的、可写的缓冲区，且足够大以容纳指定数量的数据字节
	数据直接从设备复制，不经过任何驱动侧的处理。应用程序负责对数据缓冲区进行格式化，包括小大端转换。这一点对UVC_GET_LEN 请求的结果尤其重要，该结果始终由设备以小16 位整数的形式返回
**返回*
	成功时返0。出错时返回 -1，并相应地设errno
	ENOENT
		设备不支持给定的控制，或者找不到指定的扩展单元	ENOBUFS
		指定的缓冲区大小不正确（过大或过小）	EINVAL
		传入了无效的请求码	EBADRQC
		给定的控制不支持该请求	EFAULT
		data 指针引用了不可访问的内存区域
**数据类型**

 - struct uvc_xu_control_query

	__u8	unit		扩展单元 ID
	__u8	selector	控制选择	__u8	query		要发送给设备的请求码
	__u16	size		控制数据大小（以字节为单位）
	__u8	*data		鎺у埗鍊。

### 驱动专用V4L2 控制


uvcvideo 驱动实现了以UVC 专用的控制：

`V4L2_CID_UVC_REGION_OF_INTEREST_RECT (struct)`
	该控制决定感兴趣区域（ROI）。ROI 是一个由结构`v4l2_rect` 表示的矩形区域。该矩形采用全局传感器坐标，以像素为单位。它独立于视场（field of view），不受任何裁剪或缩放的影响
	使用 `V4L2_CTRL_WHICH_MIN_VAL` `V4L2_CTRL_WHICH_MAX_VAL` 来查询矩形大小的范围
	设置一ROI 可以让相机针对该区域优化采集。`V4L2_CID_REGION_OF_INTEREST_AUTO` 控制的值决定了具体的行为
	该控制的使用示例可参见：
	`Chrome OS USB camera HAL銆?	<https://chromium.googlesource.com/chromiumos/platform2/+/refs/heads/release-R121-15699.B/camera/hal/usb/>`


`V4L2_CID_UVC_REGION_OF_INTEREST_AUTO (bitmask)`
	该控制决定哪些（如果有的话）板载功能应当跟踪当前 `V4L2_CID_UVD__REGION_OF_INTEREST_RECT` 值所指定的感兴趣区域
	最大值是一个指示所有受支持自动控制的掩码
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_EXPOSURE`
      - 设置该位会使自动曝光跟踪感兴趣区域，而不是整幅图像    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_IRIS`
      - 设置该位会使自动光圈（iris）跟踪感兴趣区域，而不是整幅图像    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_WHITE_BALANCE`
      - 设置该位会使自动白平衡跟踪感兴趣区域，而不是整幅图像    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_FOCUS`
      - 设置该位会使自动对焦调整跟踪感兴趣区域，而不是整幅图像    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_FACE_DETECT`
      - 设置该位会使自动人脸检测跟踪感兴趣区域，而不是整幅图像    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_DETECT_AND_TRACK`
      - 设置该位会启用自动人脸检测与跟踪。驱动可能会更新 `V4L2_CID_REGION_OF_INTEREST_RECT` 的当前值    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_IMAGE_STABILIZATION`
      - 设置该位会启用自动图像稳定。驱动可能会更新 `V4L2_CID_REGION_OF_INTEREST_RECT` 的当前值    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_HIGHER_QUALITY`
      - 设置该位会在可能的情况下以更高质量自动采集指定区域