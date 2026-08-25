
## Digital TV demux write()


### 名称


Digital TV demux write()

### 概要


### 参数


`fd`
  由之前对 `open()` 的调用返回的文件描述符

`buf`
     待写入数据的缓冲

`count`
    缓冲区中的字节数

### 描述


该系统调用仅由逻辑设备 `/dev/dvb/adapter/dvr` 提供，该设备与提供实DVR 功能的物理解复用设备相关联。它用于重放数字录制的传输流。必须在相应的物理解复用设备 `/dev/dvb/adapter/demux` 中定义匹配的过滤器。要传输的数据量count 隐含指定

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - -  `EWOULDBLOCK`
       - 没有数据被写入。如果指定了 `O_NONBLOCK` 且不再有可用缓冲区空间，则可能发生（如果未指`O_NONBLOCK`，该函数将阻塞直到有可用缓冲区空间）

    - -  `EBUSY`
       - 该错误码表示存在冲突的请求。相应的解复用设备被设置为从前端接收数据。请确保停止这些过滤器，并启动输入设置为 `DMX_IN_DVR` 的过滤器

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
