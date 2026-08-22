

## DMX_GET_STC


### 姓名


DMX_GET_STC

### 概要



`int ioctl(int fd, DMX_GET_STC, struct dmx_stc *stc)`

### 论点


`fd`
`open()`返回的文件描述符

`stc`
指向要存stc 数据的`dmx_stc` 的指针

### 描述


ioctl 调用返回系统时间计数器的当前
（由 `DMX_PES_PCR <dmx_ts_pes>` 类型PES 过滤器驱动）
某些硬件支持多个 STC，因此您必须指定哪一
在ioctl之前设置stc的`num <dmx_stc>`字段（范...n）
结果64 位分子的比率形式返回
和一32 位分母，所以真正的 90kHz STC 值为
`stc->stc / stc->base`銆。

### 杩斿洖鍊。


成功时返0

错误时返1，并设置`errno`变量
适当地


：标题行
：存根列
：宽度：1 16

    - ..绗?1 琛。

       - `EINVAL`

       - stc 编号无效

通用错误代码的描述见
通用错误代码 <gen-errors> 章节
