
######## ioctl MEDIA_IOC_SETUP_LINK


## Name


MEDIA_IOC_SETUP_LINK - 修改链路的属性

## Synopsis


`int ioctl(int fd, MEDIA_IOC_SETUP_LINK, struct media_link_desc *argp)`

## Arguments


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向结构体 `media_link_desc` 的指针。

## Description


为了更改链路属性，应用程序需先用链路的标识信息（源 pad 和目的 pad）以及请求的新链路标志填充一个
`media_link_desc` 结构体，然后
使用该结构的指针调用 MEDIA_IOC_SETUP_LINK ioctl。

唯一可配置的属性是用于启用/禁用链路的 `ENABLED` 链路标志。被
`IMMUTABLE` 链路标志标记的链路无法被启用或禁用。

链路配置不会对其他链路产生副作用。如果在目的 pad 上已启用的链路阻止该链路被启用，驱动会返回
`EBUSY` 错误码。

只有被 `DYNAMIC` 链路标志标记的链路才能在媒体数据流传输过程中被启用/禁用。试图启用或禁用正在传输数据的非动态链路将返回
`EBUSY` 错误码。

如果指定的链路找不到，驱动会返回 `EINVAL` 错误码。

## Return Value


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    结构体 `media_link_desc` 引用了一个
    不存在的链路，或者该链路是不可变的且试图修改其配置。
