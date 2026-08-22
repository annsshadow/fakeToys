## NetLabel 简

Paul Moore, paul.moore@hp.com

2006 骞?8 鏈?2 鏃。
## 概述


NetLabel 是一种机制，内核安全模块可以用它来为来自用户空间应用程序outgoing 网络包附加安全属性，并从 incoming 网络包读取安全属性。它由三个主要部分组成：协议引擎、通信层，以及内核安全模块 API
## 协议引擎


协议引擎负责应用和检索网络包的安全属性。如果网络的安全属性与主机上的安全属性之间需要进行任何转换，协议引擎也会处理这些任务。其他内核子系统应避免直接调用协议引擎，而应使用下面描述NetLabel 内核安全模块 API
关于每个 NetLabel 协议引擎的详细信息可以在本目录中找到
## 閫氫俊灞。

通信层用于允许从用户空间NetLabel 进行配置和监控。NetLabel 通信层使用构建在 Generic NETLINK 传输机制之上的、基于消息的协议。这NetLabel 消息的确切格式以Generic NETLINK 系列名称可以'net/netlabel/' 目录的头文件注释以及 'include/net/netlabel.h' 中找到
## 安全模块 API


NetLabel 安全模块 API 的目的是为底NetLabel 协议引擎提供与协议无关的接口。除了协议无关之外，安全模块 API 还被设计为完LSM 无关，这应该允许多个 LSM 利用同一套代码库
关于 NetLabel 安全模块 API 的详细信息可以在 'include/net/netlabel.h' 头文件以及本目录中的 'lsm_interface.txt' 文件中找到