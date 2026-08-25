

# 数字电视网络API


数字电视网络设备控制数据包的映射，这些数据包
要映射到虚拟网络接口的传输流
通过标准Linux网络协议栈可见

目前支持两种封装

- `Multi Protocol Encapsulation (MPE) <http://en.wikipedia.org/wiki/Multiprotocol_Encapsulation>`__

- `Ultra Lightweight Encapsulation (ULE) <http://en.wikipedia.org/wiki/Unidirectional_Lightweight_Encapsulation>`__

为了创建 Linux 虚拟网络接口，需要一个应用程
需要告诉内PID 和封装是什
传输流上存在的类型。这是通过
`/dev/dvb/adapter/net`设备节点。数据将通过以下方式提供
虚拟`dvb?_?`网络接口，并将通过
标准 ip 工具（如 ip、route、netstat、ifconfig 等）

数据类型和ioctl定义通过`linux/dvb/net.h`定义
标头



# 数字电视网络功能调用


- [net-types](net-types)
- [net-add-if](net-add-if)
- [net-remove-if](net-remove-if)
- [net-get-if](net-get-if)
