


######## 音频输入与输


音频输入与输出是设备的物理连接器。视频采集设备具有输入，输出设备具有输出，各有零个或多个。收音机设备没有音频输入或输出。它们正好有一个调谐器，而该调谐器事实上**就是**一个音频源，但API 仅将调谐器与视频输入或输出相关联，而收音机设备没有这些。[#f1]_ 电视卡上将接收到的音频信号回环到声卡的连接器不被视为音频输出

音频与视频的输入和输出是相关联的。选择视频源的同时也会选择音频源。当视频与音频源都是调谐器时这一点最为明显。进一步的音频连接器可以与多个视频输入或输出组合。假设存在两个复合视频输入与两个音频输入，则最多可能有四种有效组合。视频与音频连接器的关系定义于相应结构体 `v4l2_input` 或结构体 `v4l2_output` `audioset` 字段中，其中每一位代表一个音频输入或输出的索引号，从零开始

要了解可用输入与输出的数量及属性，应用程序可分别使
VIDIOC_ENUMAUDIO 涓。
VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT> ioctl 来枚举它们。VIDIOC_ENUMAUDIO ioctl 返回的结构体 `v4l2_audio` 还包含适用于查询当前音频输入时的信号状态信息

VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 涓。
VIDIOC_G_AUDOUT <VIDIOC_G_AUDOUT> ioctl 分别报告当前的音频输入与输出


   注意，与 VIDIOC_G_INPUT <VIDIOC_G_INPUT> 
   VIDIOC_G_OUTPUT <VIDIOC_G_OUTPUT> 不同，这ioctl 返回的是结构体，如同
   VIDIOC_ENUMAUDIO 涓。
   VIDIOC_ENUMAUDOUT <VIDIOC_ENUMAUDOUT> 那样，而不仅仅是一个索引

要选择音频输入并更改其属性，应用程序调用
VIDIOC_S_AUDIO <VIDIOC_G_AUDIO> ioctl。要选择音频输出（目前没有可更改的属性），应用程序调
VIDIOC_S_AUDOUT <VIDIOC_G_AUDOUT> ioctl銆。

当设备具有多个可选音频输入时，驱动必须实现所有音频输ioctl；当设备具有多个可选音频输出时，必须实现所有音频输ioctl。当设备具有任何音频输入或输出时，驱动必须在 VIDIOC_QUERYCAP ioctl 返回的结构体 `v4l2_capability` 中设`V4L2_CAP_AUDIO` 标志


## 示例：当前音频输入的信息


    struct v4l2_audio audio;

    memset(&audio, 0, sizeof(audio));

    if (-1 == ioctl(fd, VIDIOC_G_AUDIO, &audio)) {
	perror("VIDIOC_G_AUDIO");
	exit(EXIT_FAILURE);
    }

    printf("Current input: %s\n", audio.name);


## 示例：切换到第一个音频输


    struct v4l2_audio audio;

    memset(&audio, 0, sizeof(audio)); /** clear audio.mode, audio.reserved **/

    audio.index = 0;

    if (-1 == ioctl(fd, VIDIOC_S_AUDIO, &audio)) {
	perror("VIDIOC_S_AUDIO");
	exit(EXIT_FAILURE);
    }

   实际上，结构`v4l2_audio` 应当像结构体 `v4l2_input` 那样拥有一`tuner` 字段，这样不仅能API 更加一致，也能支持具有多个调谐器的收音机设备
