

######## ioctls CEC_ADAP_G_PHYS_ADDR 涓?CEC_ADAP_S_PHYS_ADDR


## 名称


CEC_ADAP_G_PHYS_ADDR, CEC_ADAP_S_PHYS_ADDR - 获取或设置物理地址

## 概要


`int ioctl(int fd, CEC_ADAP_G_PHYS_ADDR, __u16 *argp)`


`int ioctl(int fd, CEC_ADAP_S_PHYS_ADDR, __u16 *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 CEC 地址的指针
## 描述


要查询当前物理地址，应用程序以指向一__u16 的指针调ioctl CEC_ADAP_G_PHYS_ADDR <CEC_ADAP_G_PHYS_ADDR>，驱动会将物理地址存储在其中
要设置新的物理地址，应用程序将一__u16 中的物理地址存储好，并以指向该整数的指针调用 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR>。ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 仅在设置`CEC_CAP_PHYS_ADDR` 时可用（否则将返`ENOTTY` 错误码）。ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 只能由处于发起者（initiator）模式的文件描述符调用（参见 CEC_S_MODE），否则将返`EBUSY` 错误码
要清除已有的物理地址，请使用 `CEC_PHYS_ADDR_INVALID`。适配器将进入未配置状态
如果已定义了逻辑地址类型（参ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>），那么ioctl 会阻塞，直到所有请求的逻辑地址都被认领。如果文件描述符处于非阻塞模式，则不会等待逻辑地址被认领，而是直接返回 0
当物理地址发生变化时，会发送一CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 事件
物理地址是一16 位的数字，其中每 4 位一组代表物理地址 a.b.c.d 的一位数字，最4 位代'a'。CEC 根设备（通常是电视）的地址0.0.0.0。每个连接到电视输入端子的设备地址a.0.0.0（其'a' 1），依次连接在这些设备上的设备地址a.b.0.0，依此类推。因此支持最5 层深的设备拓扑。设备应使用的物理地址存储于接收端（sink）的 EDID 中
例如，电视每HDMI 输入端的 EDID 都会有一个形a.0.0.0 的不同物理地址，信号源会读出并将其用作自己的物理地址
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 章节中描述
ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 可返回以下错误码
ENOTTY
    未设`CEC_CAP_PHYS_ADDR` 能力，因此不支持ioctl
EBUSY
    另一个文件句柄处于独占的 follower initiator 模式，或者该文件句柄处于 `CEC_MODE_NO_INITIATOR` 模式
EINVAL
    物理地址格式错误