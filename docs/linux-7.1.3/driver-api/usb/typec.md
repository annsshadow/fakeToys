
## USB Type-C connector class


### Introduction


typec 类（class）旨在以统一的方式向用户空间描述系统中的 USB Type-C 端口。该类
被设计为除用户空间接口的实现之外不提供任何其他内容，希望它能被尽可能多的平台所
使用。

各平台应将它们拥有的每个 USB Type-C 端口注册到该类。正常情况下注册由 USB Type-C
或 PD PHY 驱动完成，但也可能是一个固件接口（如 UCSI）驱动、USB PD 控制器驱动，甚至
Thunderbolt3 控制器驱动。本文档将向该类注册 USB Type-C 端口的组件称为“端口驱动”。

除了展示能力，当端口驱动能够支持这些特性时，该类还提供对用户空间对端口、伙伴（partner）
和线缆插头（cable plug）的角色与替代模式（Alternate Mode）的控制。

该类提供了本文档所描述的端口驱动所用的 API。这些属性在
Documentation/ABI/testing/sysfs-class-typec 中描述。

### User space interface

每个端口都会作为自己的设备呈现在 /sys/class/typec/ 下。第一个端口命名为“port0”，
第二个为“port1”，依此类推。

连接后，伙伴也会作为自己的设备呈现在 /sys/class/typec/ 下。伙伴设备的父设备始终是
它所连接的端口。连接到“port0”端口的伙伴将命名为“port0-partner”。设备的完整路径为
/sys/class/typec/port0/port0-partner/。

线缆及其上的两个插头也可以选择性地作为自己的设备呈现在 /sys/class/typec/ 下。连接到
“port0”端口的线缆将命名为 port0-cable，SOP Prime 端的插头（见 USB Power Delivery
规范第 2.4 章）命名为“port0-plug0”，SOP Double Prime 端命名为“port0-plug1”。线缆的
父设备始终是端口，线缆插头的父设备始终是线缆。

如果端口、伙伴或线缆插头支持替代模式，每个受支持的替代模式 SVID 都会有自己的设备来
描述它。注意替代模式设备不会挂接到 typec 类下。替代模式的父设备是支持它的设备，例如
port0-partner 的一个替代模式会呈现在 /sys/class/typec/port0-partner/ 下。每个受支持的
模式在替代模式设备下都有自己的名为“mode<index>”的组，例如
/sys/class/typec/port0/<alternate mode>/mode1/。进入/退出某个模式的请求可以通过该组中
的“active”属性文件完成。

### Driver API


#### Registering the ports


端口驱动会用 struct typec_capability 数据结构描述它们所控制的每个 Type-C 端口，并用
以下 API 注册它们：

   :functions: typec_register_port typec_unregister_port

注册端口时，struct typec_capability 中的 prefer_role 成员需要特别注意。如果正在注册的
端口没有初始角色偏好（即该端口默认不执行 Try.SNK 或 Try.SRC），该成员必须具有值
TYPEC_NO_PREFERRED_ROLE。否则，如果端口默认执行 Try.SNK，该成员必须具有值
TYPEC_DEVICE；若执行 Try.SRC，则该值必须为 TYPEC_HOST。

#### Registering Partners


在伙伴成功连接后，端口驱动需要向该类注册该伙伴。伙伴的详细信息需要在 struct
typec_partner_desc 中描述。该类在注册期间会复制伙伴的详细信息。该类提供以下 API 用于
注册/注销伙伴。

   :functions: typec_register_partner typec_unregister_partner

如果注册成功，该类会提供一个指向 struct typec_partner 的句柄，否则为 NULL。

如果伙伴支持 USB Power Delivery，且端口驱动能够展示 Discover Identity 命令的结果，伙伴
描述符结构应包含指向 struct usb_pd_identity 实例的句柄。该类随后会在伙伴设备下为身份
信息创建一个 sysfs 目录。Discover Identity 命令的结果随后可通过以下 API 上报：

   :functions: typec_partner_set_identity

#### Registering Cables


在支持 USB Power Delivery 结构化 VDM“Discover Identity”的线缆成功连接后，端口驱动需要
注册该线缆以及一个或两个插头，具体取决于线缆中是否存在 CC Double Prime 控制器。因此，
支持 SOP Prime 通信但不支持 SOP Double Prime 通信的线缆应只注册一个插头。关于 SOP 通信
的更多信息，请阅读最新 USB Power Delivery 规范中的相关章节。

插头作为自己的设备表示。先注册线缆，然后注册线缆插头。线缆将是插头的父设备。线缆的
详细信息需要在 struct typec_cable_desc 中描述，插头的详细信息在 struct typec_plug_desc
中描述。该类在注册期间会复制这些详细信息。该类提供以下 API 用于注册/注销线缆及其插头：

   :functions: typec_register_cable typec_unregister_cable typec_register_plug typec_unregister_plug

如果注册成功，该类会提供一个指向 struct typec_cable 和 struct typec_plug 的句柄，否则
为 NULL。

如果线缆支持 USB Power Delivery，且端口驱动能够展示 Discover Identity 命令的结果，线缆
描述符结构应包含指向 struct usb_pd_identity 实例的句柄。该类随后会在线缆设备下为身份
信息创建一个 sysfs 目录。Discover Identity 命令的结果随后可通过以下 API 上报：

   :functions: typec_cable_set_identity

#### Notifications


当伙伴执行了角色切换，或者在连接伙伴或线缆期间默认角色发生变化时，端口驱动必须使用
以下 API 将其报告给该类：

   :functions: typec_set_data_role typec_set_pwr_role typec_set_vconn_role typec_set_pwr_opmode

#### Alternate Modes


USB Type-C 端口、伙伴和线缆插头可能支持替代模式。每个替代模式都有一个称为 SVID 的
标识符，它要么是 USB-IF 给出的标准 ID，要么是厂商 ID；每个受支持的 SVID 可以有 1–6 个
模式。该类提供 struct typec_mode_desc 用于描述一个 SVID 的单个模式，以及 struct
typec_altmode_desc 作为所有受支持模式的容器。

支持替代模式的端口需要用以下 API 注册它们支持的每个 SVID：

   :functions: typec_port_register_altmode

如果伙伴或线缆插头以 USB Power Delivery 结构化 VDM Discover SVIDs 消息响应并提供了 SVID
列表，则每个 SVID 都需要注册。

面向伙伴的 API：

   :functions: typec_partner_register_altmode

面向线缆插头的 API：

   :functions: typec_plug_register_altmode

所以端口、伙伴和线缆插头会用自己的函数注册替代模式，但注册成功时总是返回一个指向
struct typec_altmode 的句柄，失败则为 NULL。注销会使用同一个函数：

   :functions: typec_unregister_altmode

如果伙伴或线缆插头进入或退出某个模式，端口驱动需要用以下 API 通知该类：

   :functions: typec_altmode_update_active

#### Multiplexer/DeMultiplexer Switches


USB Type-C 连接器后面可能有一个或多个 mux/demux 开关。由于插头可以正插或反插，需要
一个开关将来自连接器的正确数据对路由到 USB 控制器。如果支持替代模式或配件模式，还需要
另一个开关，将连接器上的引脚路由到 USB 以外的其它组件。USB Type-C 连接器类（Connector
Class）提供注册这些开关的 API。

   :functions: typec_switch_register typec_switch_unregister typec_mux_register typec_mux_unregister

在大多数情况下，同一个物理 mux 会同时处理方向（orientation）和模式（mode）。然而，由于
端口驱动负责方向，而替代模式驱动负责模式，二者总是被分离为各自的逻辑组件：“mux” 对应
模式，“switch” 对应方向。

当端口注册时，USB Type-C 连接器类会请求该端口的 mux 和 switch。驱动随后可以用以下 API
控制它们：

   :functions: typec_set_orientation typec_set_mode

如果连接器支持双角色（dual-role），可能还有一个用于数据角色的开关。USB Type-C 连接器类
没有为它们提供单独的 API。端口驱动可以使用 USB Role Class API 来操作它们。

```

                     ------------------------
                     |       Connector      |
                     ------------------------
                            |         |
                     ------------------------
                      \     Orientation    /
                       --------------------
                                |
                       --------------------
                      /        Mode        \
                     ------------------------
                         /              \
      ------------------------        --------------------
      |       Alt Mode       |       /      USB Role      \
      ------------------------      ------------------------
                                         /            \
                     ------------------------      ------------------------
                     |       USB Host       |      |       USB Device     |
                     ------------------------      ------------------------

```