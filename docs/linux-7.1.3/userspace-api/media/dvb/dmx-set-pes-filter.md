

## DMX_SET_PES_FILTER


### 姓名


DMX_SET_PES_FILTER

### 概要



`int ioctl(int fd, DMX_SET_PES_FILTER, struct dmx_pes_filter_params *params)`

### 论点


`fd`
`open()`返回的文件描述符

`params`
指向包含过滤器参数的结构的指针

### 描述


ioctl 调用根据参数设置 PES 过滤
假如PES 过滤器是指仅基于
数据包标识符 (PID)，即PES 标头或有效负载过
支持能力

### 杩斿洖鍊。


成功时返0

错误时返1，并设置`errno`变量
适当地


：标题行
：存根列
：宽度：1 16

    - ..绗?1 琛。

       - `EBUSY`

       - 此错误代码表明存在冲突的请求
有源过滤器过滤来自另一个输入源的数据
在开始之前确保这些过滤器已停
筛选

通用错误代码的描述见
通用错误代码 <gen-errors> 章节
