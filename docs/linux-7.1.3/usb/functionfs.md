## FunctionFS 工作原理


## 概述


从内核的角度看，它只是一个具有一些独特行为的复合（composite）函数。它
只能在用户空间驱动通过写入描述符和字符串注册之后，才能被添加到 USB
配置中（用户空间程序必须提供与内核级复合函数在被添加到配置时所提供相同信息）
这尤其意味着复合初始化函数可能不init 段中（即可能不使__init 标记）
从用户空间的角度看，它是一个文件系统，挂载时提供一"ep0" 文件。用空间驱动需要向该文件写入描述符和字符串。它不需要操心端点、接口或字符编号，而只需提供描述符，就好像该函数是唯一的那个（端点和字符串编号1
开始，接口编号0 开始）。FunctionFS 会按需更改它们，并处理不同配置编号不同的情况
有关 FunctionFS 描述符的更多信息，请参见 [functionfs-desc](functionfs-desc)

当描述符和字符串被写入后ep#" 文件出现（每个声明的端点一个），用于处单个端点上的通信。同样，FunctionFS 负责处理真实的编号和配置的变更（这意着 "ep1" 文件可能实际映射到（比如说）端点 3（而当配置变更时映射到（比如说端点 2））ep0" 用于接收事件和处setup 请求
当所有文件被关闭时，该函数自行禁用
我还想提及的是，FunctionFS 的设计使得可以多次挂载它，因此最终一gadget
可以使用多个 FunctionFS 函数。其思路是每FunctionFS 实例由挂载时使用设备名标识
可以设想这样一gadget：它有一个以太网、MTP HID 接口，其中后两个
通过 FunctionFS 实现。在用户空间
```

  $ insmod g_ffs.ko idVendor=<ID> iSerialNumber=<string> functions=mtp,hid
  $ mkdir /dev/ffs-mtp && mount -t functionfs mtp /dev/ffs-mtp
  $ ( cd /dev/ffs-mtp && mtp-daemon ) &
  $ mkdir /dev/ffs-hid && mount -t functionfs hid /dev/ffs-hid
  $ ( cd /dev/ffs-hid && hid-daemon ) &

```
在内核层面，gadget 检ffs_data->dev_name 以识别其 FunctionFS 是为 MTP
mtp"）还HIDhid"）设计的
如果没有提供 "functions" 模块参数，驱动只接受一个任意名称的函数
当提供了 "functions" 模块参数时，只接受列出了名称的函数。特别是，如"functions" 参数的值只是一个单元素列表，那么其行为类似于没"functions"
时的情况；但是，只接受具有指定名称的函数
只有当所有声明的函数文件系统都已挂载，并且所有函数的 USB 描述符都已写它们ep0 时，gadget 才会被注册
相反，在第一USB 函数关闭其端点之后，gadget 会被注销
## DMABUF 接口


FunctionFS 还支持一个基DMABUF 的接口，用户空间可以DMABUF 对象（在
外部创建）附加到端点，随后将它们用于数据传输
用户空间应用程序于是可以使用此接口在多个接口之间共享 DMABUF 对象，从允许它以零拷贝（zero-copy）方式传输数据，例如IIO USB 栈之间
作为此接口的一部分，增加了三个新的 IOCTL。这三个 IOCTL 必须在数据端（即不是 ep0）上执行。它们是
  `FUNCTIONFS_DMABUF_ATTACH(int)`
    将由其文件描述符标识DMABUF 对象附加到数据端点。成功时返回零，
    出错时返回负errno 值
  `FUNCTIONFS_DMABUF_DETACH(int)`
    将由其文件描述符标识的给DMABUF 对象从数据端点分离。成功时返回
    零，出错时返回负errno 值。注意，关闭端点的文件描述符将自动分    所有已附加DMABUF
  `FUNCTIONFS_DMABUF_TRANSFER(struct usb_ffs_dmabuf_transfer_req *)`
    将先前附加的 DMABUF 入队到传输队列。参数是一个结构，它打包了 DMABUF
    的文件描述符、要传输的字节大小（通常应对应于 DMABUF 的大小），以    一个目前未使用'flags' 字段。成功时返回零，出错时返回负errno 值