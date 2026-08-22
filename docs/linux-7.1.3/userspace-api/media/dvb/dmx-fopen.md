


## 数字电视 demux open()


### 名称


Digital TV demux open()

### 概要



### 参数


`name`
  特定数字电视 demux 设备的名称
`flags`
  以下标志的按OR

    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    -
       - `O_RDONLY`
       - 只读访问

    -
       - `O_RDWR`
       - 写访
    -
       - `O_NONBLOCK`
       - 以非阻塞模式打开
         （阻塞模式是默认模式
### 描述


该系统调用与设备`/dev/dvb/adapter/demux` 一起使用，分配一个新的过滤器并返回可用于后续控制该过滤器的句柄。必须为要使用的每个过滤器进行此调用，即每个返回的文件描述符都是对单个过滤器的引用。`/dev/dvb/adapter/dvr` 是一个逻辑设备，用于检索用于数字视频录制的传输流（Transport Stream）。从此设备读取时，得到的是传输流，其中包含来自相demux 设备（`/dev/dvb/adapter/demux`）中所有输出被设为 `DMX_OUT_TS_TAP` PES 过滤器的包录制的传输流通过写入此设备来回放
阻塞或非阻塞模式的意义在存在差异的函数文档中描述。它不影`open()` 调用本身的语义。以阻塞模式打开的设备之后可以使fcntl 系统调用`F_SETFL` 命令置于非阻塞模式（反之亦然）
### 杩斿洖鍊。

成功时返0
出错时返-1，并相应地设`errno` 变量

    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - -  `EMFILE`
       - "打开的文件过，即没有更多可用的过滤器
通用错误码在 Generic Error Codes <gen-errors> 章节中描述