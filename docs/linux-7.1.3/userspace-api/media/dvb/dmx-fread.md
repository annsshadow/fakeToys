
## Digital TV demux read()


### 名称


Digital TV demux read()

### 概要


### 参数


`fd`
  由之前对 `open()` 的调用返回的文件描述符

 `buf`
   待填充的缓冲

`count`
   最多读取的字节

### 描述


该系统调用返回经过过滤的数据，可能是段（section）或打包的基本流（PES）数据。过滤后的数据从驱动的内部循环缓冲区传输`buf`。要传输的最大数据量count 隐含指定


   如果创建了带`DMX_CHECK_CRC <dmx_sct_filter_params>` 标志集的段过滤器，则 CRC 检查失败的数据将被静默忽略

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - -  `EWOULDBLOCK`
       - 没有数据可返回，且指定了 `O_NONBLOCK`

    - -  `EOVERFLOW`
       - 过滤后的数据未能及时从缓冲区读取，导致未被读取的数据丢失。缓冲区被刷新

    - -  `ETIMEDOUT`
       - 段未能在规定的超时时间内加载
          有关如何设置超时，请参阅 ioctl DMX_SET_FILTER

    - -  `EFAULT`
       - 驱动由于无效\*buf 指针而未能写入调用者的缓冲区

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
