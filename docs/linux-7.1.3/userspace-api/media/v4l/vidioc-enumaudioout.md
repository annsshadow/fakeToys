


######## ioctl VIDIOC_ENUMAUDOUT


## 名称


VIDIOC_ENUMAUDOUT - 枚举音频输出

## 概要


`int ioctl(int fd, VIDIOC_ENUMAUDOUT, struct v4l2_audioout *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_audioout` 的指针。

## 描述


要查询音频输出的属性，应用程序初始化 struct `v4l2_audioout` 的 `index` 字段
并清零 `reserved` 数组，然后以指向该结构的指针调用 `VIDIOC_G_AUDOUT` ioctl。
当索引越界时，驱动填充结构的其余部分或返回 `EINVAL` 错误码。要枚举所有音频
输出，应用程序应从索引零开始，每次递增一，直到驱动返回 `EINVAL`。


    将电视卡上用于把接收到的音频信号环回到声卡的连接器，在此意义上不是音频
    输出。

有关 struct `v4l2_audioout` 的描述，请参阅 VIDIOC_G_AUDIOout <VIDIOC_G_AUDOUT>。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    音频输出的编号越界。
