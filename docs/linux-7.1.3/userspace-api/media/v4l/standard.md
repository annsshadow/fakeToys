######## 视频标准

视频设备通常支持一个或多个不同的视频标准或标准的变体。每个视频输入和输出可能支持另一组标准。这组标准由 VIDIOC_ENUMINPUT 和 VIDIOC_ENUMOUTPUT ioctl 分别返回的 struct `v4l2_input` 和 struct `v4l2_output` 的 `std` 字段报告。

V4L2 为当前全球使用的每种模拟视频标准定义了一位，并为驱动定义的标准预留了位，例如用于在 PAL 电视上观看 NTSC 录像带（反之亦然）的混合标准。应用程序可以使用预定义的位来选择特定标准，尽管更推荐向用户展示一个受支持标准菜单。要枚举并查询受支持标准的属性，应用程序使用 VIDIOC_ENUMSTD ioctl。

许多已定义的标准实际上只是少数几个主要标准的变体。硬件实际上可能不区分它们，或者在内部区分并自动切换。因此枚举出的标准也包含一组或多组标准位。

假设一个假想的调谐器能够解调 B/PAL、G/PAL 和 I/PAL 信号。第一个枚举出的标准是一组 B 和 G/PAL，根据在 UHF 或 VHF 波段选择的射频自动切换。枚举给出 "PAL-B/G" 或 "PAL-I" 选项。类似地，一个复合输入可能会合并标准，枚举出 "PAL-B/G/H/I"、"NTSC-M" 和 "SECAM-D/K"。[#f1]_

要查询和选择当前视频输入或输出所使用的标准，应用程序分别调用 VIDIOC_G_STD <VIDIOC_G_STD> 和 VIDIOC_S_STD <VIDIOC_G_STD> ioctl。被**接收**到的标准可以通过 VIDIOC_QUERYSTD ioctl 感知。

   所有这些 ioctl 的参数都是一个指向 v4l2_std_id <v4l2-std-id> 类型（一个标准集合）的指针，**而不是**标准枚举中的索引。当设备具有一个或多个视频输入或输出时，驱动必须实现所有视频标准 ioctl。

对于诸如 USB 摄像头这类视频标准概念意义不大的设备，有特殊的规则。更一般地说，对于任何捕获或输出设备，如果它：

- 无法以视频标准的标称速率捕获场或帧，或者

- 根本不支持视频标准格式。

在此，驱动应将 struct `v4l2_input` 和 struct `v4l2_output` 的 `std` 字段设为零，并且 VIDIOC_G_STD <VIDIOC_G_STD>、VIDIOC_S_STD <VIDIOC_G_STD>、VIDIOC_QUERYSTD 和 VIDIOC_ENUMSTD ioctl 应返回 `ENOTTY` 错误码或 `EINVAL` 错误码。

应用程序可以利用 input-capabilities 和 output-capabilities 标志来确定视频标准 ioctl 是否可与给定的输入或输出一起使用。

## 示例：关于当前视频标准的信息

    v4l2_std_id std_id;
    struct v4l2_standard standard;

    if (-1 == ioctl(fd, VIDIOC_G_STD, &std_id)) {
	/* Note when VIDIOC_ENUMSTD always returns ENOTTY this
	   is no video device or it falls under the USB exception,
	   and VIDIOC_G_STD returning ENOTTY is no error. */

	perror("VIDIOC_G_STD");
	exit(EXIT_FAILURE);
    }

    memset(&standard, 0, sizeof(standard));
    standard.index = 0;

    while (0 == ioctl(fd, VIDIOC_ENUMSTD, &standard)) {
	if (standard.id & std_id) {
	       printf("Current video standard: %s\\n", standard.name);
	       exit(EXIT_SUCCESS);
	}

	standard.index++;
    }

    /* EINVAL indicates the end of the enumeration, which cannot be
       empty unless this device falls under the USB exception. */

    if (errno == EINVAL || standard.index == 0) {
	perror("VIDIOC_ENUMSTD");
	exit(EXIT_FAILURE);
    }

## 示例：列出当前输入支持的视频标准

    struct v4l2_input input;
    struct v4l2_standard standard;

    memset(&input, 0, sizeof(input));

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &input.index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUM_INPUT");
	exit(EXIT_FAILURE);
    }

    printf("Current input %s supports:\\n", input.name);

    memset(&standard, 0, sizeof(standard));
    standard.index = 0;

    while (0 == ioctl(fd, VIDIOC_ENUMSTD, &standard)) {
	if (standard.id & input.std)
	    printf("%s\\n", standard.name);

	standard.index++;
    }

    /* EINVAL indicates the end of the enumeration, which cannot be
       empty unless this device falls under the USB exception. */

    if (errno != EINVAL || standard.index == 0) {
	perror("VIDIOC_ENUMSTD");
	exit(EXIT_FAILURE);
    }

## 示例：选择一个新的视频标准

    struct v4l2_input input;
    v4l2_std_id std_id;

    memset(&input, 0, sizeof(input));

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &input.index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUM_INPUT");
	exit(EXIT_FAILURE);
    }

    if (0 == (input.std & V4L2_STD_PAL_BG)) {
	fprintf(stderr, "Oops. B/G PAL is not supported.\\n");
	exit(EXIT_FAILURE);
    }

    /* Note this is also supposed to work when only B
       or G/PAL is supported. */

    std_id = V4L2_STD_PAL_BG;

    if (-1 == ioctl(fd, VIDIOC_S_STD, &std_id)) {
	perror("VIDIOC_S_STD");
	exit(EXIT_FAILURE);
    }

   一些用户已经被 PAL、NTSC 和 SECAM 这些技术术语搞糊涂了。当软件或硬件可以自动完成时，没有必要要求他们去区分 B、G、D 或 K。
