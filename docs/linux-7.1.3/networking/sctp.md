
## Linux 内核 SCTP


这是 Linux 内核 SCTP 参考实现的当前 BETA 版本。

SCTP（流控制传输协议，Stream Control Transmission Protocol）是一种基于 IP 的、面向消息的、可靠的传输协议，具有拥塞控制、对透明多宿主（multi-homing）的支持，以及多路有序的消息流。RFC2960 定义了核心协议。IETF SIGTRAN 工作组最初开发了 SCTP 协议，后来将协议移交给传输领域（TSVWG）工作组，以作为通用传输协议继续演进 SCTP。

有关 SCTP 的更多文档，请参阅 IETF 网站（http://www.ietf.org）。参阅 http://www.ietf.org/rfc/rfc2960.txt

最初的项目目标是创建一个符合 RFC 2960 的 Linux 内核 SCTP 参考实现，并提供称为 SCTP 套接字扩展的 UDP 风格 API 的编程接口，如 IETF Internet-Drafts 中所提议。

## 注意事项


- lksctp 可以静态构建或作为模块构建。但是，请注意，移除 lksctp 模块目前还不是一项安全的操作。

- 对 IPv6 有试探性支持，但大部分工作都投入到了 IPv4 上 lksctp 的实现和测试。


更多信息，请访问 lksctp 项目网站：

   http://www.sf.net/projects/lksctp

或通过邮件列表联系 lksctp 开发者：

   <linux-sctp@vger.kernel.org>
