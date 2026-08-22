

######## FE_GET_EVENT


## 姓名


FE_GET_EVENT


## 概要



`int ioctl(int fd, FE_GET_EVENT, struct dvb_frontend_event *ev)`

## 论点


`fd`
`open()`返回的文件描述符

`ev`
指向要存储事件（如果有）的位置

## 描述


ioctl 调用返回前端事件（如果可用）。如果一个事件是
不可用，行为取决于设备是否处于阻塞状
或非阻塞模式。在后一种情况下，调用立即失
errno 设置为`EWOULDBLOCK`。在前一种情况下，调用会阻塞，直
事件变得可用

## 杩斿洖鍊。


成功时返0

错误时返1，并设置`errno`变量
适当地

：标题行
：存根列

    - ..绗?1 琛。

       - `EWOULDBLOCK`

       - 没有待处理的事件，并且设备处于非阻塞模式

    - ..绗，琛。

       - `EOVERFLOW`

       - 事件队列溢出 - 一个或多个事件丢失

通用错误代码的描述见
通用错误代码 <gen-errors> 章节
