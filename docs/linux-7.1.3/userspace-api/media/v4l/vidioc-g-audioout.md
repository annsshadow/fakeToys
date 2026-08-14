######## ioctl VIDIOC_G_AUDOUT, VIDIOC_S_AUDOUT


## 名称


VIDIOC_G_AUDOUT - VIDIOC_S_AUDOUT - 查询或选择当前的音频输出

## 摘要


`int ioctl(int fd, VIDIOC_G_AUDOUT, struct v4l2_audioout *argp)`


`int ioctl(int fd, VIDIOC_S_AUDOUT, const struct v4l2_audioout *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_audioout` 的指针。

## 描述


要查询当前的音频输出，应用程序需将一个 struct `v4l2_audioout` 的 `reserved`
数组清零，并以指向该结构的指针调用 `VIDIOC_G_AUDOUT` ioctl。驱动会填充结构的
其余部分；当设备没有音频输入，或与当前视频输出无法组合时，则返回 `EINVAL`
错误码。

音频输出没有可写属性。不过，要选择当前的音频输出，应用程序可以初始化一个
struct `v4l2_audioout` 结构的 `index` 字段和 `reserved` 数组（未来可能包含
可写属性），然后调用 `VIDIOC_S_AUDOUT` ioctl。驱动会切换到所请求的输出，或当
index 越界时返回 `EINVAL` 错误码。这是一个只写 ioctl，它不会像 `VIDIOC_G_AUDOUT`
那样返回当前音频输出的属性。


   TV 卡上用于把接收到的音频信号环回到声卡的接口不属于此意义上的音频输出。


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 标识音频输出，由驱动或应用程序设置。
    - - __u8
      - `name`\ [^32^]
      - 音频输出的名称，一个以 NUL 结尾的 ASCII 字符串，例如：“Line Out”。
	此信息供用户参考，最好是设备本身的接口标签。
    - - __u32
      - `capability`
      - 音频能力标志，目前尚未定义。驱动必须将本字段设置为零。
    - - __u32
      - `mode`
      - 音频模式，目前尚未定义。驱动和应用程序（在 `VIDIOC_S_AUDOUT` 时）
	必须将本字段设置为零。
    - - __u32
      - `reserved`\ [^2^]
      - 为未来扩展保留。驱动和应用程序必须将本数组设置为零。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    没有与当前视频输出组合的音频输出，或者所选音频输出的编号越界，或无法组合。
