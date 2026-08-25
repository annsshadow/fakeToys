
######## 扩展控制 API


## 简

最初设计的控制机制旨在用于用户设置（亮度、饱和度等）。然而，实践证明对于实现更复杂的驱动 API 也是一个非常有用的模型，在这种 API 中每个驱只实现更大的 API 的一个子集
MPEG 编码 API 是设计和实现这一扩展控制机制的驱动力：MPEG 标准相当庞大而当前受支持的硬MPEG 编码器各自只实现了该标准的一个子集。此外，许多
关于如何将视频编码为 MPEG 流的相关参数特定MPEG 编码芯片，因MPEG
标准只定义了最MPEG 流的格式，而非视频实际被编码为该格式的方式
遗憾的是，原始的控制 API 缺少这些新用途所需的一些特性，因此它被扩展（命名并不十分有创意的）扩展控制 API
尽管 MPEG 编码 API 是使用扩展控API 的首次尝试，如今也出现了其他类别
的扩展控制，例如 Camera Controls（摄像头控制）和 FM Transmitter Controls
（FM 发射器控制）。扩展控API 以及所有扩展控制类别在下文中描述

## 扩展控制 API


有三个新ioctl 可用：VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>。这ioctl 作用于控制数（相对于作用于单个控制的 VIDIOC_G_CTRL <VIDIOC_G_CTRL> VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl）。这是必需的，因为常常需要原子地
一次性更改多个控制
每个新的 ioctl 都期望一个指struct `v4l2_ext_controls` 的指针。该结构
包含一个指向控制数组的指针、数组中控制数量的一个计数，以及一个控制类别控制类别用于将相似的控制归为单一类别。例如，控制类别 `V4L2_CTRL_CLASS_USER`
包含所有用户控制（即也能使用旧VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl
设置的所有控制）。控制类`V4L2_CTRL_CLASS_CODEC` 包含与编解码器相关的
控制
控制数组中的所有控制都必须属于指定的控制类别。否则会返回错误
也可以使用一个空的控制数组（`count` == 0）来检查指定的控制类别是否受支持
控制数组是一struct `v4l2_ext_control` 数组。struct `v4l2_ext_control`
struct `v4l2_control` 非常相似，只是它还允许传64 位值和指针
由于 struct `v4l2_ext_control` 支持指针，现在也可以拥有复合类型（如
N 维数组和/或结构体）的控制。在枚举控制时，你需要指`V4L2_CTRL_FLAG_NEXT_COMPOUND` 才能实际看到这类复合控制。换言之，这些
复合类型的控制只应以编程方式使用
由于这类复合控制需要暴露比 VIDIOC_QUERYCTRL <VIDIOC_QUERYCTRL> 所能提的更多信息，因此增加VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL> ioctl特别地，当该控制由多个元素组成时，此 ioctl 会给N 维数组的维度

   #. 重要的是要认识到，由于控制的灵活性，有必要检查你想要设置的控制是      确实受驱动支持，以及其有效值范围是什么。所以请使用 VIDIOC_QUERYCTRL
      来检查
   #. 类型`V4L2_CTRL_TYPE_MENU` 的控制中，某些菜单索引可能不受支      （`VIDIOC_QUERYMENU` 会返回错误）。一个很好的例子是受支持MPEG
      音频比特率列表。有些驱动只支持一两种比特率，另一些则支持更宽的范围
所有控制都使用机器字节序

## 枚举扩展控制


推荐的枚举扩展控制的方式是使VIDIOC_QUERYCTRL 配合
`V4L2_CTRL_FLAG_NEXT_CTRL` 标志


    struct v4l2_queryctrl qctrl;

    qctrl.id = V4L2_CTRL_FLAG_NEXT_CTRL;
    while (0 == ioctl (fd, VIDIOC_QUERYCTRL, &qctrl)) {
	/** ... **/
	qctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL;
    }

初始的控ID 被设0 `V4L2_CTRL_FLAG_NEXT_CTRL` 标志相或的结果`VIDIOC_QUERYCTRL` ioctl 将返ID 比指定值更高的第一个控制。当找不到这的控制时，会返回错误
如果你想获取特定控制类别内的所有控制，可以将初始的 `qctrl.id` 值设为该
控制类别，并增加一个额外的检查，以便在发现属于另一控制类别的控制时跳出
循环


    qctrl.id = V4L2_CTRL_CLASS_CODEC | V4L2_CTRL_FLAG_NEXT_CTRL;
    while (0 == ioctl(fd, VIDIOC_QUERYCTRL, &qctrl)) {
	if (V4L2_CTRL_ID2CLASS(qctrl.id) != V4L2_CTRL_CLASS_CODEC)
	    break;
	/** ... **/
	qctrl.id |= V4L2_CTRL_FLAG_NEXT_CTRL;
    }

32 位的 `qctrl.id` 值被划分为三个位段：最高的 4 位保留给标志（例`V4L2_CTRL_FLAG_NEXT_CTRL`），并不属于 ID 本身。剩下的 28 位构成控ID其中最12 位定义控制类别，最16 位标识该控制类别内的控制。可以保这些最后的 16 位对于所有控制都非零x1000 及以上的范围保留给驱动私控制。宏 `V4L2_CTRL_ID2CLASS(id)` 根据控制 ID 返回控制类别 ID
如果驱动不支持扩展控制，那么 `VIDIOC_QUERYCTRL` `V4L2_CTRL_FLAG_NEXT_CTRL`
配合使用时将失败。这种情况下应使用旧的枚举控制方法（enum_all_controls）但如果受支持，则保证会枚举所有控制，包括驱动私有控制

## 创建控制面板


可以为图形用户界面创建控制面板，让用户可以选择各种控制。基本上你将需使用上述方法遍历所有控制。每个控制类别都以一个类型为
`V4L2_CTRL_TYPE_CTRL_CLASS` 的控制开始。`VIDIOC_QUERYCTRL` 将返回此控制
类别的名称，可用作控制面板中标签页的标题
struct v4l2_queryctrl <v4l2-queryctrl> flags 字段也包含关于控制行为的
提示。详VIDIOC_QUERYCTRL 文档