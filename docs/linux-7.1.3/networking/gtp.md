
## Linux 内核 GTP 隧道模块


文档作者：
		 Harald Welte <laforge@gnumonks.org> 鍜?		 Andreas Schultz <aschultz@tpip.net>

'drivers/net/gtp.c' 中，你可以找到一GTP 隧道端点的内核级实现
## 什么是 GTP


GTP 是通用隧道协议（Generic Tunnel Protocol），它是一3GPP 协议，用于在移动（手机、调制解调器）与外部分组数据网络（如互联网）之间的互联中隧道传输用户 IP 载荷
因此，当你从手机发起一数据连接"时，手机会使用控制平面来信令请求在该外部数据网络
与手机之间建立这样一条隧道。于是隧道端点就驻留在手机和网关上。所有中间节点只是传被封装的数据包
手机本身并不实现 GTP，而是使用某种其他与具体技术相关的协议栈来传输用户 IP 载荷，例LLC/SNDCP/RLC/MAC
在蜂窝运营商基础设施内部的某个网元（对于 GPRS/EGPRS 或经UMTS SGSN，对3G
 femtocell hNodeB，对4G/LTE eNodeB）处，蜂窝协议栈被转换为 GTP*且不打破
端到端隧*。因此中间节点只执行某些特定的中继功能
在某个时刻，GTP 数据包最终到达所谓的 GGSN（GSM/UMTS）或 P-GW（LTE），后者终结隧道，
解封装数据包并将其转发到外部分组数据网络。这可以是公共互联网，但也可以是任何私有 IP
网络（理论上甚至可以是某些非 IP 网络，如 X.25）
你可以在 3GPP TS 29.060 中找到协议规范，该规范可通过 3GPP 网站公开获取http://www.3gpp.org/DynaReport/29060.htm

为方便起见，下面提供v13.6.0 的直PDF 链接http://www.etsi.org/deliver/etsi_ts/129000_129099/129060/13.06.00_60/ts_129060v130600p.pdf

## Linux GTP 隧道模块


该模块实现了隧道端点的功能，即它能够解封装手机在上行方向发起的隧IP 数据包，封装从外部分组网络接收到的原IP 数据包，在下行方向发往手机
**实现了所谓的"用户平面"（user plane），承载用户 IP 载荷，称GTP-U。它不实"控制平面"（control plane），即用于建立和拆除 GTP 隧道（GTP-C）的信令协议
因此，为了拥有一个可工作GGSN/P-GW 配置，你需要一个实现了 GTP-C 协议、然后使用内核中
GTP-U 模块提供netlink 接口来配置该内核模块的用户空间程序
这种分离架构遵循其他协议的隧道模块，例如 PPPoE L2TP，在那里你同样运行一个用户空守护进程来处理隧道建立、认证等，而只有数据平面在内核中得到加速
不要被术语搞混：GTP 用户平面走内核加速路径，GTP 控制平面走用户空:)

该模块的官方主页位于
https://osmocom.org/projects/linux-kernel-gtp-u/wiki

## 具有 Linux 内核 GTP-U 支持的用户空间程

在撰写本文时，至少有两个自由软件实现实现GTP-C，并可以使用 netlink 接口来利Linux
内核GTP-U 支持
- OpenGGSN（C 语言编写的经2G/3G GGSN）：
  https://osmocom.org/projects/openggsn/wiki/OpenGGSN

- ergw（Erlang 编写GGSN + P-GW）：
  https://github.com/travelping/ergw

## 用户空间/ 命令行工

有一个名'libgtpnl' 的用户空间库，它基于 libmnl，并实现了面向内GTP 模块所提供netlink 接口C 语言 API
http://git.osmocom.org/libgtpnl/

## 协议版本


GTP-U 有两个不同的版本：v0 [GSM TS 09.60] v1 [3GPP TS 29.281]。两者都在内GTP
模块中实现。版0 是一个遗留版本，在近期的 3GPP 规范中已被废弃
GTP-U 使用 UDP 来传PDU。接UDP 端口对于 GTPv1-U 2151，对GTPv0-U 3386
GTP-C 有三个版本：v0、v1 v2。由于内核不实现 GTP-C，我们无需为此担心。这由用户空中控制平面的实现负责
## IPv6


3GPP 规范表明，在内层（用户）IP 层或外层（传输）IP 层上，都可以使用 IPv4 IPv6
遗憾的是，目前内核模块对于用IP 载荷和外IP 层都不支IPv6。非常欢迎提供补丁或
其他贡献来修复这一点！

## 邮件列表


如果你有关于如何从你自己的软件使用内GTP 模块的问题，或者想要为代码做出贡献，请使用
osmocom-net-gprs 邮件列表进行相关讨论。该列表可通过 osmocom-net-gprs@lists.osmocom.org
访问，管理你订阅mailman 界面位于
https://lists.osmocom.org/mailman/listinfo/osmocom-net-gprs

## 问题跟踪

Osmocom 项目在以下位置维护着内核 GTP-U 模块的问题跟踪器https://osmocom.org/projects/linux-kernel-gtp-u/issues

## 历史 / 致谢


该模块最初由 Harald Welte 2012 年创建，但从未完成。Pablo 介入完成Harald 留下烂摊子。但由于缺乏用户兴趣，它从未被合入主线
2015 年，Andreas Schultz 出手修复了更多缺陷，扩展了新的特性，并最终推动我们所有人将其
合入主线，于 4.7.0 版本合入
## 架构细节


### 本地 GTP-U 实体与隧道识

GTP-U 使用 UDP 来传PDU。接UDP 端口对于 GTPv1-U 2152，对GTPv0-U 3386
每个 IP 地址只有一GTP-U 实体（因此也只有一SGSN/GGSN/S-GW/PDN-GW 实例）。隧道端标识符（TEID）在每个 GTP-U 实体内是唯一的
一条特定的隧道仅由目的实体定义。由于目的端口是常量，只有目IP TEID 定义一条隧道IP 和端口对隧道没有意义
因此
  - 发送时，远程实体由远程 IP 和隧道端ID 定义。源 IP 和端口没有意义，可以随时更改
  - 接收时，本地实体由本地目IP 和隧道端ID 定义。源 IP 和端口没有意义，可以随时
    更改
```

   GTP-U 头中TEID 用于对来自远程隧道端点的入向流量进行解复用，使其以允许复用不   用户、不同分组协议和不同 QoS 级别的方式交付给用户平面实体   因此，除作为移动性过程一部分的数据转发外，不应有两个远程 GTP-U 端点使用相同   TEID 值向一GTP-U 协议实体发送流量
```
上面的定义仅规定两个远程 GTP-U 端点**不应**发送到相同TEID，它**并不**禁止或排这样的场景。事实上，所提到的移动性过程使GTP-U 实体有必要接受来自多个或未知对端TEID 的流量
因此，接收端仅基TEID 而非IP 来识别隧道！

## APN 与网络设

GTP-U 驱动为每Gi/SGi 接口创建一Linux 网络设备
[3GPP TS 29.281] Gi/SGi 参考点称为一个接口。这可能让人产生 GGSN/P-GW 只能有一个这接口的印象
正确的说法是，Gi/SGi 参考点定义了基GTP-U 隧道3GPP 分组域（PDN）与基于 IP 的网之间的互通
任何 3GPP 文档中都没有限制 GGSN/P-GW 所实现Gi/SGi 接口数量的规定
[3GPP TS 29.061] 11.3 节明确指出，特定 Gi/SGi 接口的选择是通过接入点名称（APN）做出的
```

   2. 每个私有网络管理自己的编址。一般而言，这将导致不同的私有网络具有重叠的地址
      范围。在 GGSN/P-GW 与每个私有网络之间使用逻辑上独立的连接（例IP-in-IP 隧道
      或二层虚拟电路）
      在这种情况下，仅IP 地址不一定唯一。APN IPv4 地址IPv6 前缀这对      是唯一的
```
为了支持重叠地址范围的使用场景，每个 APN 都被映射到一个单独的 Gi/SGi 接口（网络设备）
   接入点名称纯粹是一个控制平面（GTP-C）概念   GTP-U 层面，GTP-U 数据包和网络设备中只存在隧道端点标识
因此对于给定UE，IP PDN 网络的映射为
  - 网络设备 + MS IP -> 对端 IP + 对端 TEID
而从 PDN IP 网络
  - 本地 GTP-U IP + TEID  -> 网络设备

此外，在收到T-PDU 被注入网络设备之前，会将 MS IP PDP 上下文中记录IP 进行核对