


######## ioctl FE_SET_PROPERTY, FE_GET_PROPERTY


## 名称


FE_SET_PROPERTY - FE_GET_PROPERTY - FE_SET_PROPERTY 设置一个或多个前端属性。- FE_GET_PROPERTY 返回一个或多个前端属性。

## 概要



`int ioctl(int fd, FE_GET_PROPERTY, struct dtv_properties *argp)`


`int ioctl(int fd, FE_SET_PROPERTY, struct dtv_properties *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向结构体 `dtv_properties` 的指针。

## 描述


所有数字电视前端设备都支持 `FE_SET_PROPERTY` 和
`FE_GET_PROPERTY` ioctls。支持的属性和统计信息
取决于传输系统和设备：

- `FE_SET_PROPERTY:`

   - 该 ioctl 用于设置一个或多前端属性。

   - 这是请求前端调谐到某个频率并开始解码
      数字电视信号的基本命令。

   - 该调用需要对设备具有读写访问权限。


   返回时，值不会更新以反映实际
   使用的参数。如果需要实际参数，则需显式
   调用 `FE_GET_PROPERTY`。

- `FE_GET_PROPERTY:`

   - 该 ioctl 用于从前端获取属性
      和统计信息。

   - 不会更改任何属性，也不会重置统计信息。

   - 该调用仅需要对设备具有只读访问权限。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在
Generic Error Codes <gen-errors> 章节中描述。
