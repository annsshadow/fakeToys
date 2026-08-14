


######## ioctl VIDIOC_REMOVE_BUFS


## 名称


VIDIOC_REMOVE_BUFS - 从队列中移除缓冲区

## 概要



`int ioctl(int fd, VIDIOC_REMOVE_BUFS, struct v4l2_remove_buffers *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_remove_buffers` 的指针。

## 描述


应用程序可以可选地调用 VIDIOC_REMOVE_BUFS ioctl 从队列中移除缓冲区。要启用 VIDIOC_REMOVE_BUFS，必须支持 VIDIOC_CREATE_BUFS ioctl。当调用 `VIDIOC_REQBUFS` 或 `VIDIOC_CREATE_BUFS` 时，若队列上设置了 `V4L2_BUF_CAP_SUPPORTS_REMOVE_BUFS` 能力，则该 ioctl 可用。




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 要移除的起始缓冲区索引。若 count == 0，此字段被忽略。
    - - __u32
      - `count`
      - 要移除的缓冲区数量，索引从 'index' 到 'index + count - 1'。
        此范围内的所有缓冲区必须有效且处于 DEQUEUED 状态。
        VIDIOC_REMOVE_BUFS 总会检查 `type` 的有效性，若无效则返回 `EINVAL` 错误码。
        若 count 设为 0，VIDIOC_REMOVE_BUFS 将不执行任何操作并返回 0。
    - - __u32
      - `type`
      - 流或缓冲区的类型，与 struct `v4l2_format` 的 `type` 字段相同。
	有效值参见 `v4l2_buf_type`。
    - - __u32
      - `reserved`\ [^13^]
      - 为未来扩展预留的占位符。驱动与应用程序必须将该数组置零。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。若发生错误，不会释放任何缓冲区，并返回以下错误码之一：

EBUSY
    文件 I/O 正在进行中。
    `index` 到 `index + count - 1` 范围中的一个或多个缓冲区不处于 DEQUEUED 状态。

EINVAL
    `index` 到 `index + count - 1` 范围中的一个或多个缓冲区不在队列中存在。
    缓冲区类型（`type` 字段）无效。
