


######## 用户控制


设备通常拥有若干用户可设置的控制项，例如亮度、饱和度等，这些控制项会在图形用户界面中
呈现给用户。但是，不同设备可用的控制项不同，而且可能取值范围、默认值也会因设备而异。
control ioctl 提供了相关信息以及一种机制，用于为这些控制项创建能与任意设备正确配合使用
的友好用户界面。

所有控制项都通过 ID 值访问。V4L2 为特定用途定义了若干 ID。驱动也可以使用
`V4L2_CID_PRIVATE_BASE` [#f1]_ 及更高的数值来实现其自定义的控件。预定义的控制 ID 以
`V4L2_CID_` 为前缀，并在 control-id 中列出。查询控制项属性以及获取或设置当前值时都会
用到该 ID。

一般而言，应用程序在向用户呈现控制项时不应假定其用途。每个控制项都附带一个供用户理解的
名称字符串。当用途不直观时，驱动编写者应提供用户手册、用户界面插件或驱动专用的面板程序。
引入预定义 ID 是为了能够以编程方式更改少量控制项，例如在切换频道时将设备静音。

驱动在切换当前视频输入或输出、调谐器或调制器、或音频输入或输出后，可能枚举出不同的
控制项。不同体现在其他边界、另一个默认值和当前值、步进大小或其他菜单项上。带有特定
**custom** ID 的控制项也可能改变名称和类型。

如果某个控制项不适用于设备的当前配置（例如，它不适用于当前视频输入），驱动会设置
`V4L2_CTRL_FLAG_INACTIVE` 标志。

控制值以全局方式存储，切换时不会改变，除非为保持在报告的边界之内。它们也不会在设备打开或
关闭、调谐器频率改变时发生改变，通常而言除非应用程序请求，否则绝不会改变。

V4L2 规定了一种事件机制，用于在对控制项的值发生改变时通知应用程序（参见
VIDIOC_SUBSCRIBE_EVENT，事件 `V4L2_EVENT_CTRL`），面板程序可以利用这一点以始终反映
正确的控制值。

所有控制项使用机器字节序。



## 控制 ID


`V4L2_CID_BASE`
    首个预定义的 ID，等于 `V4L2_CID_BRIGHTNESS`。

`V4L2_CID_USER_BASE`
    `V4L2_CID_BASE` 的同义词。

`V4L2_CID_BRIGHTNESS` `(integer)`
    图像亮度，更准确地说是黑电平。

`V4L2_CID_CONTRAST` `(integer)`
    图像对比度或亮度增益。

`V4L2_CID_SATURATION` `(integer)`
    图像色彩饱和度或色度增益。

`V4L2_CID_HUE` `(integer)`
    色调或色彩平衡。

`V4L2_CID_AUDIO_VOLUME` `(integer)`
    整体音频音量。注意某些驱动也提供 OSS 或 ALSA 混音器接口。

`V4L2_CID_AUDIO_BALANCE` `(integer)`
    音频立体声平衡。最小值对应最左，最大值对应最右。

`V4L2_CID_AUDIO_BASS` `(integer)`
    音频低音调节。

`V4L2_CID_AUDIO_TREBLE` `(integer)`
    音频高音调节。

`V4L2_CID_AUDIO_MUTE` `(boolean)`
    静音，即把音量设为零，但不影响 `V4L2_CID_AUDIO_VOLUME`。与 ALSA 驱动类似，V4L2
    驱动必须在加载时静音以避免过量噪声。实际上整个设备应被重置为低功耗状态。

`V4L2_CID_AUDIO_LOUDNESS` `(boolean)`
    响度模式（低音增强）。

`V4L2_CID_BLACK_LEVEL` `(integer)`
    亮度的另一个名称（不是 `V4L2_CID_BRIGHTNESS` 的同义词）。此控制项已废弃，不应在
    新驱动和应用程序中使用。

`V4L2_CID_AUTO_WHITE_BALANCE` `(boolean)`
    自动白平衡（摄像机）。

`V4L2_CID_DO_WHITE_BALANCE` `(button)`
    这是一个动作型控制项。设置时（忽略其值），设备会执行一次白平衡并保持当前设置。这与
    布尔型 `V4L2_CID_AUTO_WHITE_BALANCE` 不同，后者激活后会持续调整白平衡。

`V4L2_CID_RED_BALANCE` `(integer)`
    红色色度平衡。

`V4L2_CID_BLUE_BALANCE` `(integer)`
    蓝色色度平衡。

`V4L2_CID_GAMMA` `(integer)`
    伽马调节。

`V4L2_CID_WHITENESS` `(integer)`
    灰度设备的白度。这是 `V4L2_CID_GAMMA` 的同义词。此控制项已废弃，不应在新驱动和
    应用程序中使用。

`V4L2_CID_EXPOSURE` `(integer)`
    曝光（摄像机）。[单位？]

`V4L2_CID_AUTOGAIN` `(boolean)`
    自动增益/曝光控制。

`V4L2_CID_GAIN` `(integer)`
    增益控制。

    主要用于控制电视调谐器乃至网络摄像头的增益。大多数设备仅用此控制项控制数字增益，但
    有些设备也可能包含模拟增益。能够区分数字增益与模拟增益差异的设备会使用
    `V4L2_CID_DIGITAL_GAIN` 和 `V4L2_CID_ANALOGUE_GAIN` 控制项。


`V4L2_CID_HFLIP` `(boolean)`
    水平镜像图像。


`V4L2_CID_VFLIP` `(boolean)`
    垂直镜像图像。


`V4L2_CID_POWER_LINE_FREQUENCY` `(enum)`
    启用电源线频率滤波以避免闪烁。`enum v4l2_power_line_frequency` 的可能取值为：

    ==========================================  ==
    `V4L2_CID_POWER_LINE_FREQUENCY_DISABLED`	 0
    `V4L2_CID_POWER_LINE_FREQUENCY_50HZ`	 1
    `V4L2_CID_POWER_LINE_FREQUENCY_60HZ`	 2
    `V4L2_CID_POWER_LINE_FREQUENCY_AUTO`	 3
    ==========================================  ==

`V4L2_CID_HUE_AUTO` `(boolean)`
    启用设备的自动色调控制。在启用自动色调控制时设置 `V4L2_CID_HUE` 的效果是未定义的，
    驱动应忽略此类请求。

`V4L2_CID_WHITE_BALANCE_TEMPERATURE` `(integer)`
    此控制项以开尔文为单位的色温指定白平衡设置。驱动的取值范围应至少覆盖 2800（白炽灯）
    到 6500（日光）。有关色温的更多信息，请参见
    `Wikipedia <http://en.wikipedia.org/wiki/Color_temperature>`__。

`V4L2_CID_SHARPNESS` `(integer)`
    调节摄像机中的锐化滤波器。最小值会禁用滤波器，值越大图像越锐利。

`V4L2_CID_BACKLIGHT_COMPENSATION` `(integer)`
    调节摄像机中的背光补偿。最小值会禁用背光补偿。

`V4L2_CID_CHROMA_AGC` `(boolean)`
    色度自动增益控制。

`V4L2_CID_CHROMA_GAIN` `(integer)`
    调节色度增益控制（用于色度 AGC 禁用时）。

`V4L2_CID_COLOR_KILLER` `(boolean)`
    启用消色器（即在视频信号较弱时强制输出黑白图像）。


`V4L2_CID_COLORFX` `(enum)`
    选择色彩效果。定义了以下取值：




    :header-rows:  0
    :stub-columns: 0
    :widths: 11 24

    - - `V4L2_COLORFX_NONE`
      - 色彩效果已禁用。
    - - `V4L2_COLORFX_ANTIQUE`
      - 老化（旧照片）效果。
    - - `V4L2_COLORFX_ART_FREEZE`
      - 霜冻色彩效果。
    - - `V4L2_COLORFX_AQUA`
      - 水彩色调，冷色调。
    - - `V4L2_COLORFX_BW`
      - 黑白。
    - - `V4L2_COLORFX_EMBOSS`
      - 浮雕，高光和阴影替换明暗边界，低对比度区域被设为灰色背景。
    - - `V4L2_COLORFX_GRASS_GREEN`
      - 草绿。
    - - `V4L2_COLORFX_NEGATIVE`
      - 负片。
    - - `V4L2_COLORFX_SEPIA`
      - 棕褐色调。
    - - `V4L2_COLORFX_SKETCH`
      - 素描。
    - - `V4L2_COLORFX_SKIN_WHITEN`
      - 皮肤美白。
    - - `V4L2_COLORFX_SKY_BLUE`
      - 天蓝。
    - - `V4L2_COLORFX_SOLARIZATION`
      - 色调分离（Solarization），图像色调部分反转，仅高于或低于某阈值的颜色值被反转。
    - - `V4L2_COLORFX_SILHOUETTE`
      - 剪影（轮廓）。
    - - `V4L2_COLORFX_VIVID`
      - 鲜艳色彩。
    - - `V4L2_COLORFX_SET_CBCR`
      - Cb 和 Cr 色度分量被 `V4L2_CID_COLORFX_CBCR` 控制项确定的固定系数替换。
    - - `V4L2_COLORFX_SET_RGB`
      - RGB 分量被 `V4L2_CID_COLORFX_RGB` 控制项确定的固定 RGB 分量替换。


`V4L2_CID_COLORFX_RGB` `(integer)`
    确定 `V4L2_COLORFX_SET_RGB` 色彩效果的红、绿、蓝系数。所提供的 32 位值的位 [7:0]
    解释为蓝色分量，位 [15:8] 为绿色分量，位 [23:16] 为红色分量，位 [31:24] 必须为零。

`V4L2_CID_COLORFX_CBCR` `(integer)`
    确定 `V4L2_COLORFX_SET_CBCR` 色彩效果的 Cb 和 Cr 系数。所提供的 32 位值的位 [7:0]
    解释为 Cr 分量，位 [15:8] 为 Cb 分量，位 [31:16] 必须为零。

`V4L2_CID_AUTOBRIGHTNESS` `(boolean)`
    启用自动亮度。

`V4L2_CID_ROTATE` `(integer)`
    按指定角度旋转图像。常见角度为 90、270 和 180。将图像旋转到 90 和 270 会反转显示窗口
    的高度和宽度。需要根据所选旋转角度，使用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 设置
    图像的新高度和宽度。

`V4L2_CID_BG_COLOR` `(integer)`
    设置当前输出设备上的背景色。背景色需以 RGB24 格式指定。所提供的 32 位值解释为：位 0-7
    为红色信息，位 8-15 为绿色信息，位 16-23 为蓝色信息，位 24-31 必须为零。

`V4L2_CID_ILLUMINATORS_1 V4L2_CID_ILLUMINATORS_2` `(boolean)`
    打开或关闭设备的照明器 1 或 2（通常是显微镜）。

`V4L2_CID_MIN_BUFFERS_FOR_CAPTURE` `(integer)`
    这是一个只读控制项，应用程序可读取它并用作提示，以确定要传递给 REQBUFS 的 CAPTURE
    缓冲区数量。该值是硬件工作所需的最小 CAPTURE 缓冲区数量。有状态解码器需要此控制项。

`V4L2_CID_MIN_BUFFERS_FOR_OUTPUT` `(integer)`
    这是一个只读控制项，应用程序可读取它并用作提示，以确定要传递给 REQBUFS 的 OUTPUT
    缓冲区数量。该值是硬件工作所需的最小 OUTPUT 缓冲区数量。有状态编码器需要此控制项。


`V4L2_CID_ALPHA_COMPONENT` `(integer)`
    设置 alpha 颜色分量。当采集设备（或 mem-to-mem 设备的采集队列）产生包含 alpha 分量的
    帧格式（例如 packed RGB 图像格式 <pixfmt-rgb>），且 alpha 值未由设备或 mem-to-mem
    输入数据定义时，此控制项可让你选择所有像素的 alpha 分量值。当输出设备（或 mem-to-mem
    设备的输出队列）使用不包含 alpha 分量的帧格式，且设备支持 alpha 通道处理时，此控制项可
    让你设置所有像素的 alpha 分量值，以便在设备内进一步处理。

`V4L2_CID_LASTP1`
    预定义控制 ID 的结尾（目前为 `V4L2_CID_ALPHA_COMPONENT` + 1）。

`V4L2_CID_PRIVATE_BASE`
    首个自定义（驱动特定）控制项的 ID。依赖特定自定义控制项的应用程序应检查驱动名称和版本，
    参见 querycap。

应用程序可以使用 VIDIOC_QUERYCTRL 和 VIDIOC_QUERYMENU <VIDIOC_QUERYCTRL> ioctl 枚举
可用控制项，使用 VIDIOC_G_CTRL <VIDIOC_G_CTRL> 和 VIDIOC_S_CTRL <VIDIOC_G_CTRL>
ioctl 获取和设置控制值。当设备具有一个或多个控制项时，驱动必须实现 `VIDIOC_QUERYCTRL`、
`VIDIOC_G_CTRL` 和 `VIDIOC_S_CTRL`；当具有一个或多个菜单型控制项时，必须实现
`VIDIOC_QUERYMENU`。



## 示例：枚举所有控制项



    struct v4l2_queryctrl queryctrl;
    struct v4l2_querymenu querymenu;

    static void enumerate_menu(__u32 id)
    {
	printf("  Menu items:\\n");

	memset(&querymenu, 0, sizeof(querymenu));
	querymenu.id = id;

	for (querymenu.index = queryctrl.minimum;
	     querymenu.index <= queryctrl.maximum;
	     querymenu.index++) {
	    if (0 == ioctl(fd, VIDIOC_QUERYMENU, &querymenu)) {
		printf("  %s\\n", querymenu.name);
	    }
	}
    }

    memset(&queryctrl, 0, sizeof(queryctrl));

    queryctrl.id = V4L2_CTRL_FLAG_NEXT_CTRL;
    while (0 == ioctl(fd, VIDIOC_QUERYCTRL, &queryctrl)) {
	if (!(queryctrl.flags & V4L2_CTRL_FLAG_DISABLED)) {
	    printf("Control %s\\n", queryctrl.name);

	    if (queryctrl.type == V4L2_CTRL_TYPE_MENU)
	        enumerate_menu(queryctrl.id);
        }

	queryctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL;
    }
    if (errno != EINVAL) {
	perror("VIDIOC_QUERYCTRL");
	exit(EXIT_FAILURE);
    }

## 示例：枚举所有控制项（含复合控制项）



    struct v4l2_query_ext_ctrl query_ext_ctrl;

    memset(&query_ext_ctrl, 0, sizeof(query_ext_ctrl));

    query_ext_ctrl.id = V4L2_CTRL_FLAG_NEXT_CTRL | V4L2_CTRL_FLAG_NEXT_COMPOUND;
    while (0 == ioctl(fd, VIDIOC_QUERY_EXT_CTRL, &query_ext_ctrl)) {
	if (!(query_ext_ctrl.flags & V4L2_CTRL_FLAG_DISABLED)) {
	    printf("Control %s\\n", query_ext_ctrl.name);

	    if (query_ext_ctrl.type == V4L2_CTRL_TYPE_MENU)
	        enumerate_menu(query_ext_ctrl.id);
        }

	query_ext_ctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL | V4L2_CTRL_FLAG_NEXT_COMPOUND;
    }
    if (errno != EINVAL) {
	perror("VIDIOC_QUERY_EXT_CTRL");
	exit(EXIT_FAILURE);
    }

## 示例：枚举所有用户控制项（旧式）



    memset(&queryctrl, 0, sizeof(queryctrl));

    for (queryctrl.id = V4L2_CID_BASE;
	 queryctrl.id < V4L2_CID_LASTP1;
	 queryctrl.id++) {
	if (0 == ioctl(fd, VIDIOC_QUERYCTRL, &queryctrl)) {
	    if (queryctrl.flags & V4L2_CTRL_FLAG_DISABLED)
		continue;

	    printf("Control %s\\n", queryctrl.name);

	    if (queryctrl.type == V4L2_CTRL_TYPE_MENU)
		enumerate_menu(queryctrl.id);
	} else {
	    if (errno == EINVAL)
		continue;

	    perror("VIDIOC_QUERYCTRL");
	    exit(EXIT_FAILURE);
	}
    }

    for (queryctrl.id = V4L2_CID_PRIVATE_BASE;;
	 queryctrl.id++) {
	if (0 == ioctl(fd, VIDIOC_QUERYCTRL, &queryctrl)) {
	    if (queryctrl.flags & V4L2_CTRL_FLAG_DISABLED)
		continue;

	    printf("Control %s\\n", queryctrl.name);

	    if (queryctrl.type == V4L2_CTRL_TYPE_MENU)
		enumerate_menu(queryctrl.id);
	} else {
	    if (errno == EINVAL)
		break;

	    perror("VIDIOC_QUERYCTRL");
	    exit(EXIT_FAILURE);
	}
    }


## 示例：更改控制项



    struct v4l2_queryctrl queryctrl;
    struct v4l2_control control;

    memset(&queryctrl, 0, sizeof(queryctrl));
    queryctrl.id = V4L2_CID_BRIGHTNESS;

    if (-1 == ioctl(fd, VIDIOC_QUERYCTRL, &queryctrl)) {
	if (errno != EINVAL) {
	    perror("VIDIOC_QUERYCTRL");
	    exit(EXIT_FAILURE);
	} else {
	    printf("V4L2_CID_BRIGHTNESS is not supported\n");
	}
    } else if (queryctrl.flags & V4L2_CTRL_FLAG_DISABLED) {
	printf("V4L2_CID_BRIGHTNESS is not supported\n");
    } else {
	memset(&control, 0, sizeof (control));
	control.id = V4L2_CID_BRIGHTNESS;
	control.value = queryctrl.default_value;

	if (-1 == ioctl(fd, VIDIOC_S_CTRL, &control)) {
	    perror("VIDIOC_S_CTRL");
	    exit(EXIT_FAILURE);
	}
    }

    memset(&control, 0, sizeof(control));
    control.id = V4L2_CID_CONTRAST;

    if (0 == ioctl(fd, VIDIOC_G_CTRL, &control)) {
	control.value += 1;

	/** The driver may clamp the value or return ERANGE, ignored here **/

	if (-1 == ioctl(fd, VIDIOC_S_CTRL, &control)
	    && errno != ERANGE) {
	    perror("VIDIOC_S_CTRL");
	    exit(EXIT_FAILURE);
	}
    /** Ignore if V4L2_CID_CONTRAST is unsupported **/
    } else if (errno != EINVAL) {
	perror("VIDIOC_G_CTRL");
	exit(EXIT_FAILURE);
    }

    control.id = V4L2_CID_AUDIO_MUTE;
    control.value = 1; /** silence **/

    /** Errors ignored **/
    ioctl(fd, VIDIOC_S_CTRL, &control);

   使用 `V4L2_CID_PRIVATE_BASE` 存在问题，因为不同的驱动可能将同一个
   `V4L2_CID_PRIVATE_BASE` ID 用于不同的控制项。由于该 ID 对应控制项的含义取决于驱动，
   因此很难以编程方式设置此类控制项。为了解决这个问题，驱动会使用唯一 ID，并由内核将
   `V4L2_CID_PRIVATE_BASE` ID 映射到这些唯一 ID。应将这些 `V4L2_CID_PRIVATE_BASE` ID
   视为真实 ID 的别名。

   如今许多应用程序仍在使用 `V4L2_CID_PRIVATE_BASE` ID，而不是使用带
   `V4L2_CTRL_FLAG_NEXT_CTRL` 标志的 VIDIOC_QUERYCTRL 来枚举所有 ID，因此对
   `V4L2_CID_PRIVATE_BASE` 的支持仍然存在。
