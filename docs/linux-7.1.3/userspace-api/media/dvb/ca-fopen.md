
## Digital TV CA open()


### 名称


Digital TV CA open()

### 概要


### 参数


`name`
  特定 Digital TV CA 设备的名称。

`flags`
  以下标志的按位或：


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - - `O_RDONLY`
       - 只读访问

    - - `O_RDWR`
       - 读写访问

    - - `O_NONBLOCK`
       - 以非阻塞模式打开
         （默认是阻塞模式）

### 描述


该系统调用打开一个命名的 ca 设备（例如 `/dev/dvb/adapter?/ca?`）以供后续使用。

当 `open()` 调用成功后，设备即可使用。阻塞或非阻塞模式的意义在存在差异的函数文档中描述。它不影响 `open()` 调用本身的语义。以阻塞模式打开的设备之后可以使用 `fcntl` 系统调用的 `F_SETFL` 命令置于非阻塞模式（反之亦然）。这是一个标准系统调用，在 fcntl 的 Linux 手册页中有记录。只有一个用户能以 `O_RDWR` 模式打开 CA 设备。所有其它以该模式打开设备的尝试都将失败，并返回错误码。

### 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
