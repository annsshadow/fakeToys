


## DMX_START


### 名称


DMX_START

### 概要



`int ioctl(int fd, DMX_START)`

### 参数


`fd`
    `open()` 返回的文件描述符
### 描述


ioctl 调用用于启动通过 ioctl 调用 DMX_SET_FILTER DMX_SET_PES_FILTER 定义的实际过滤操作
### 杩斿洖鍊。

成功时返0
出错时返-1，并相应地设`errno` 变量

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `EINVAL`

       - 无效参数，即未通过 DMX_SET_FILTER DMX_SET_PES_FILTER ioctl
	  提供任何过滤参数
    - .. row 2

       - `EBUSY`

       - 此错误码表示存在冲突请求。有活动的过滤器正在从另一个输入源
	 过滤数据。在启动此过滤器之前，请确保这些过滤器已停止
通用错误码的描述见通用错误<gen-errors> 章节