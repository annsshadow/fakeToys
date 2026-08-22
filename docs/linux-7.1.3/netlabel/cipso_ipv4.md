## NetLabel CIPSO/IPv4 协议引擎


Paul Moore, paul.moore@hp.com

May 17, 2006

## 概述


NetLabel CIPSO/IPv4 协议引擎基于 1992 7 16 日的 IETF 商用 IP 安全选项（CIPSO）草案。该草案的副本可以在此目录中找到（draft-ietf-cipso-ipsecurity-01.txt）。尽管该 IETF 草案从未成为 RFC 标准，但它已成为标记网络（labeled networking）事实上的标准，并被许多可信操作系统使用
## 出站报文处理


CIPSO/IPv4 协议引擎通过CIPSO 标签添加到套接字，从而将 CIPSO IP 选项应用于报文。这会使通过该套接字离开系统的所有报文都应用 CIPSO IP 选项。套接字CIPSO 标签可以在任意时刻更改，不过建议在套接字创建时设置。LSM 可以使用 NetLabel 安全模块 API 设置套接字的 CIPSO 标签；如NetLabel “domain”被配置为使CIPSO 进行报文标记，则会生成一CIPSO IP 选项并附加到套接字上
## 入站报文处理


CIPSO/IPv4 协议引擎IP 层验证它发现的每CIPSO IP 选项，无需 LSM 做任何特殊处理。然而，为了解码并转换报文上CIPSO 标签，LSM 必须使用 NetLabel 安全模块 API 提取报文的安全属性。这通常是在套接字层使用 'socket_sock_rcv_skb()' LSM 钩子完成
## 标签转换


CIPSO/IPv4 协议引擎包含一种机制，用于CIPSO 安全属性（如敏感级别与类别）转换为适合主机的值。这些映射作CIPSO 解释域（Domain Of Interpretation，DOI）定义的一部分定义，并通过 NetLabel 用户空间通信层配置。每DOI 定义可以有不同的安全属性映射表
## 标签转换缓存


NetLabel 系统提供了一个框架，用于缓存从网络标签到相应 LSM 标识符的安全属性映射。CIPSO/IPv4 协议引擎支持该缓存机制