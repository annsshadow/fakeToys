


######## ioctl VIDIOC_G_CTRL, VIDIOC_S_CTRL


## 名称（Name）


VIDIOC_G_CTRL - VIDIOC_S_CTRL - 获取或设置某个控件的值

## 概要（Synopsis）


`int ioctl(int fd, VIDIOC_G_CTRL, struct v4l2_control *argp)`


`int ioctl(int fd, VIDIOC_S_CTRL, struct v4l2_control *argp)`

## 参数（Arguments）


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_control` 的指针。

## 描述（Description）


为了获取某个控件的当前值，应用程序初始化一个 struct `v4l2_control` 的 `id` 字段，
并用指向该结构的指针调用 VIDIOC_G_CTRL <VIDIOC_G_CTRL> ioctl。为了更改某个控件
的值，应用程序初始化 struct `v4l2_control` 的 `id` 与 `value` 字段，并调用
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl。

当 `id` 无效时，驱动返回 `EINVAL` 错误码。当 `value` 超出范围时，驱动可以选择采用
最接近的有效值，或返回 `ERANGE` 错误码，以看起来更合适者为准。然而，VIDIOC_S_CTRL
<VIDIOC_G_CTRL> 是一个只写 ioctl，它不会返回实际的新值。如果 `value` 对于该控件
不合适（例如，它引用了菜单控件一个不受支持的菜单索引），那么也会返回 EINVAL 错误码。

这些 ioctl 仅适用于用户控件。对于其它控件类，必须使用 VIDIOC_G_EXT_CTRLS
<VIDIOC_G_EXT_CTRLS>、VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 或
VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>。


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 标识控件，由应用程序设置。
    - - __s32
      - `value`
      - 新值或当前值。

## 返回值（Return Value）


成功时返回 0，出错时返回 -1，并适当地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    结构 `v4l2_control` 的 `id` 无效，或 `value` 对于给定控件不合适（即，根据
    VIDIOC_QUERYMENU <VIDIOC_QUERYCTRL> 选择了驱动不支持的菜单项）。

ERANGE
    结构 `v4l2_control` 的 `value` 超出范围。

EBUSY
    该控件暂时不可更改，可能是因为另一个应用程序接管了此控件所属的设备功能。

EACCES
    试图设置只读控件，或获取只写控件。

    或者，如果存在试图设置一个非活动控件的操作，而驱动无法在控件再次激活之前
    缓存新值。
