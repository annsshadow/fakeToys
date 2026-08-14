
######## 示例

本页提供 V4L2 选择（selection）接口的使用示例，通过 C 代码片段演示裁剪（crop）与合成（compose）参数的查询、重置与缩放等操作，供应用开发者在用户空间实现视频捕获或输出时参考。



（假定为视频捕获设备；对其他设备请更改 `V4L2_BUF_TYPE_VIDEO_CAPTURE`；
如需配置合成区域，请将目标改为 `V4L2_SEL_TGT_COMPOSE_*` 系列）

## 示例：重置裁剪参数



	struct v4l2_selection sel = {
	    .type = V4L2_BUF_TYPE_VIDEO_CAPTURE,
	    .target = V4L2_SEL_TGT_CROP_DEFAULT,
	};
	ret = ioctl(fd, VIDIOC_G_SELECTION, &sel);
	if (ret)
	    exit(-1);
	sel.target = V4L2_SEL_TGT_CROP;
	ret = ioctl(fd, VIDIOC_S_SELECTION, &sel);
	if (ret)
	    exit(-1);

在显示器中央设置一个输出合成区域，其大小**至多**为限制值的一半。

## 示例：简单缩小



	struct v4l2_selection sel = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_COMPOSE_BOUNDS,
	};
	struct v4l2_rect r;

	ret = ioctl(fd, VIDIOC_G_SELECTION, &sel);
	if (ret)
	    exit(-1);
	/** 设置更小的合成矩形 **/
	r.width = sel.r.width / 2;
	r.height = sel.r.height / 2;
	r.left = sel.r.width / 4;
	r.top = sel.r.height / 4;
	sel.r = r;
	sel.target = V4L2_SEL_TGT_COMPOSE;
	sel.flags = V4L2_SEL_FLAG_LE;
	ret = ioctl(fd, VIDIOC_S_SELECTION, &sel);
	if (ret)
	    exit(-1);

假定为视频输出设备；对其他设备请更改 `V4L2_BUF_TYPE_VIDEO_OUTPUT`

## 示例：查询缩放因子



	struct v4l2_selection compose = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_COMPOSE,
	};
	struct v4l2_selection crop = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_CROP,
	};
	double hscale, vscale;

	ret = ioctl(fd, VIDIOC_G_SELECTION, &compose);
	if (ret)
	    exit(-1);
	ret = ioctl(fd, VIDIOC_G_SELECTION, &crop);
	if (ret)
	    exit(-1);

	/** 计算缩放因子 **/
	hscale = (double)compose.r.width / crop.r.width;
	vscale = (double)compose.r.height / crop.r.height;
