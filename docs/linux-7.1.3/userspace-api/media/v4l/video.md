

######## 视频输入与输出


视频输入和输出是设备的物理连接器。这些可以是例如：RF 连接器（天线/有线电视）、CVBS（又称复合视频）、S-Video 和 RGB 连接器。相机传感器也被视为一种视频输入。视频和 VBI 捕获设备具有输入。视频和 VBI 输出设备具有输出，各自至少有一个。无线电设备没有视频输入或输出。

要了解可用输入和输出的数量与属性，应用程序可以分别使用
VIDIOC_ENUMINPUT 和
VIDIOC_ENUMOUTPUT ioctl 来枚举它们。VIDIOC_ENUMINPUT
ioctl 返回的 `v4l2_input` 结构体还包含适用于查询当前视频输入时的信号状态信息。

VIDIOC_G_INPUT <VIDIOC_G_INPUT> 和
VIDIOC_G_OUTPUT <VIDIOC_G_OUTPUT> ioctl 返回当前视频输入或输出的索引。要选择不同的输入或输出，应用程序调用 VIDIOC_S_INPUT <VIDIOC_G_INPUT> 和
VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> ioctl。当设备具有一个或多个输入时，驱动必须实现所有输入 ioctl；当设备具有一个或多个输出时，必须实现所有输出 ioctl。

## 示例：关于当前视频输入的信息



    struct v4l2_input input;
    int index;

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    memset(&input, 0, sizeof(input));
    input.index = index;

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUMINPUT");
	exit(EXIT_FAILURE);
    }

    printf("Current input: %s\n", input.name);


## 示例：切换到第一个视频输入



    int index;

    index = 0;

    if (-1 == ioctl(fd, VIDIOC_S_INPUT, &index)) {
	perror("VIDIOC_S_INPUT");
	exit(EXIT_FAILURE);
    }
