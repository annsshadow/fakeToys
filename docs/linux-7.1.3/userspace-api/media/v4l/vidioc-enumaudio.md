

######## ioctl VIDIOC_ENUMAUDIO


## 姓名


VIDIOC_ENUMAUDIO - 枚举音频输入

## 概要



`int ioctl(int fd, VIDIOC_ENUMAUDIO, struct v4l2_audio *argp)`

## 论点


`fd`
`open()`返回的文件描述符。

`argp`
指向结构体`v4l2_audio`的指针。

## 描述


要查询音频输入应用程序的属性，请初始化
`index`字段并将结构体的`reserved`数组清零
调用 VIDIOC_ENUMAUDIO
ioctl 带有指向该结构的指针。司机填补了剩下的空缺
构造或当索引超出时返回`EINVAL`错误代码
界限。要枚举所有音频输入应用程序应从索引开始
零，加一直到驱动程序返回`EINVAL`。

有关结构体的说明，请参阅 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO>
`v4l2_audio`。

## 返回值


成功时返回 0，错误时返回 -1 并且设置 `errno` 变量
适当地。通用错误代码的描述见
通用错误代码 <gen-errors> 章节。

单项选择
音频输入数量越界。
