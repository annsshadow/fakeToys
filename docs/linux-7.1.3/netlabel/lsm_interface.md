## NetLabel Linux 安全模块（LSM）接口


Paul Moore, paul.moore@hp.com

2006 年 5 月 17 日

## 概述


NetLabel 是一种能够从网络数据包中设置和获取安全属性的机制。它旨在
供那些希望为若干不同的数据包标记协议使用公共代码基的 LSM 开发者使用。
NetLabel 安全模块 API 定义于 'include/net/netlabel.h'，但下面给出一个
简要概述。

## NetLabel 安全属性


由于 NetLabel 支持多种不同的数据包标记协议和 LSM，它使用“安全属性”
这一概念来指代数据包的安全标签。NetLabel 安全属性由 NetLabel 头文件中的
'netlbl_lsm_secattr' 结构体定义。在内部，NetLabel 子系统会根据 NetLabel
的构建时与运行时配置，将安全属性转换为正确的底层数据包标签，或反向转换。
由 LSM 开发者负责将 NetLabel 安全属性转换为其特定 LSM 所使用的任何安全
标识符。

## NetLabel LSM 协议操作


这些是允许 LSM 开发者操纵出站数据包上的标签、以及读取入站数据包上的
标签的函数。既有作用于套接字的函数，也有直接作用于 sk_buff 的函数。
这些高层函数会根据管理员对 NetLabel 子系统的配置，被转换为底层协议操作。

## NetLabel 标签映射缓存操作


取决于具体的配置，网络数据包标签与内部 LSM 安全标识符之间的转换可能
相当耗时。NetLabel 标签映射缓存是一种缓存机制，一旦建立映射，就可以
避开大部分此类开销。在 LSM 收到一个数据包、使用 NetLabel 解码其安全
属性、并将安全属性转换为 LSM 内部标识符之后，LSM 可以使用 NetLabel 的
缓存函数将该 LSM 内部标识符与网络数据包的标签关联起来。这意味着，将来
当入站数据包匹配某个缓存值，不仅 NetLabel 的内部转换机制被绕过，LSM 的
转换机制也被绕过，从而应当能显著减少开销。
