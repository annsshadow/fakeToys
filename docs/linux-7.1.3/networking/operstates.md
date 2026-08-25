
## 操作状

## 1. 简

Linux 区分接口的管理状态（administrative state）与操作状态（operational state）管理状态是“ip link set dev <dev> up down”的结果，反映管理员是否希望使用设备传输流量
然而，接口并非仅仅因为管理员启用了它就可用——以太网需要插到交换机上，并且根据
站点的网络策略与配置，在用户数据传输之前还需要进802.1X 认证。操作状态反了一个接口传输这些用户数据的能力
得益802.1X，必须允许用户空间影响操作状态。为此，操作状态被拆分为两部分：两只能由驱动设置的标志，以及一个由这些标志、一项策略以及在某些规则下可由用户空更改的、与 RFC2863 兼容的状态

## 2. 从用户空间查

管理状态与操作状态都可以通过 netlink 操作 RTM_GETLINK 查询。也可以订阅
RTNLGRP_LINK 以在接口处于管理 up 时收到更新通知。这对于从用户空间进行设置很重要
这些值包含接口状态：

**ifinfomsg**
: if_flags & IFF_UP 接口处于管理 up
**ifinfomsg**
: if_flags & IFF_RUNNING 接口处于 RFC2863 操作状UP UNKNOWN。这是为了向后兼容，路由守护进程 dhcp 客户端可用此标志来判断是否应该使用该接口
**ifinfomsg**
: if_flags & IFF_LOWER_UP 驱动已发netif_carrier_on() 信号
**ifinfomsg**
: if_flags & IFF_DORMANT 驱动已发netif_dormant_on() 信号
### TLV IFLA_OPERSTATE


包含接口RFC2863 状态，以数值表示：

IF_OPER_UNKNOWN (0) 接口处于未知状态，驱动和用户空间都未设置操作状态。由于并非每个驱动都实现 操作状态设置，接口在考虑用户数据时必须被视为未知
IF_OPER_NOTPRESENT (1) 当前内核中未使用（notpresent 接口通常会消失），仅作数值占位
IF_OPER_DOWN (2) 接口无法L1 上传输数据，例如以太网未插线，或接口处于 ADMIN down
IF_OPER_LOWERLAYERDOWN (3) 堆叠IF_OPER_DOWN 接口之上的接口显示此状态（例如 VLAN）
IF_OPER_TESTING (4) 接口处于测试模式，例如正在执行驱动自检或介质（线缆）测试。在测试完成之前不能
 用于正常流量
IF_OPER_DORMANT (5) 接口 L1 up，但在等待一个外部事件，例如等待某个协议建立02.1X）
IF_OPER_UP (6) 接口操作 up，可以使用
TLV 也可通过 sysfs 查询
### TLV IFLA_LINKMODE


包含链路策略。下面描述的用户空间交互需要它
TLV 也可通过 sysfs 查询

## 3. 内核驱动 API


内核驱动可以访问映射IFF_LOWER_UP IFF_DORMANT 的两个标志。这些标志可以在
任何地方设置，甚至可以在中断中设置。虽然没有其它部分拥有写权限，但如果驱动的不层操作同一个标志，驱动必须提供必要的同步
__LINK_STATE_NOCARRIER，映射到 !IFF_LOWER_UP
驱动使用 netif_carrier_on() 清除该标志，使用 netif_carrier_off() 设置它。在
netif_carrier_off() 时，调度器停止发送包。名称“carrier”及其取反是历史原因，可
将其理解为下层（lower layer）
注意，对于某些不管理任何真实硬件的软设备，可以从用户空间设置此位。应使用 TLV
IFLA_CARRIER 来这么做
netif_carrier_ok() 可用于查询该位
__LINK_STATE_DORMANT，映射到 IFF_DORMANT
由驱动设置，表示设备由于某些驱动控制的协议建立尚未完成而暂时无法使用。对应的函数
netif_dormant_on() 设置该标志，netif_dormant_off() 清除它，netif_dormant()
用于查询
在设备分配时，__LINK_STATE_NOCARRIER __LINK_STATE_DORMANT 两个标志都被清除因此有效状态等同于 netif_carrier_ok() !netif_dormant()

每当驱动更改这两个标志之一时，会调度一个工作队列事件，将标志组合转换为
IFLA_OPERSTATE，如下所示：

!netif_carrier_ok() 若接口是堆叠的则IF_OPER_LOWERLAYERDOWN，否则为 IF_OPER_DOWN。内核可以识 堆叠接口，因为它们的 ifindex != iflink
netif_carrier_ok() && netif_dormant()锛? IF_OPER_DORMANT

netif_carrier_ok() && !netif_dormant() 若用户空间交互被禁用则为 IF_OPER_UP。否则为 IF_OPER_DORMANT，之后用户空间可 发起IF_OPER_UP 的转换

## 4. 从用户空间设

应用程序必须使用 netlink 接口来影响接口的 RFC2863 操作状态。通过 RTM_SETLINK IFLA_LINKMODE 设为 1 会指示内核：当驱动设netif_carrier_ok() && !netif_dormant() 组合时，接口应进IF_OPER_DORMANT 而非
IF_OPER_UP。之后，只要驱动没有设置 netif_carrier_off() netif_dormant_on()用户空间应用程序就可以将 IFLA_OPERSTATE 设为 IF_OPER_DORMANT IF_OPER_UP。用空间所做的更改会在 netlink RTNLGRP_LINK 上广播
因此，一802.1X 请求方（supplicant）与内核的交互大致如下：

- 订阅 RTNLGRP_LINK
- 通过 RTM_SETLINK IFLA_LINKMODE 设为 1
- 查询一RTM_GETLINK 以获取初始状- 如果初始标志不是 (IFF_LOWER_UP && !IFF_DORMANT)，则等待直到 netlink 多播发出
  此状态信- 执行 802.1X，如果标志再次变 down 则中- 如果认证成功，发RTM_SETLINK operstate 设为 IF_OPER_UP，否则设  IF_OPER_DORMANT
- 观察 operstate IFF_RUNNING 如何通过 netlink 多播回显
- 如果 802.1X 重新认证失败，将接口设回 IF_OPER_DORMANT
- 如果内核更改IFF_LOWER_UP IFF_DORMANT 标志，则重新开
如果请求方退出，IFLA_LINKMODE 恢复0，并IFLA_OPERSTATE 恢复为一个合理的值
路由守护进程dhcp 客户端只需关注 IFF_RUNNING，或在考虑使用该接/ 查询 DHCP
地址之前，等operstate 变为 IF_OPER_UP/IF_OPER_UNKNOWN

技术问题及/或意见请发邮件给 Stefan Rompf（stefan at loplof.de）