## SCSI FC 传输（Transport

Date:  11/18/2008


```

  rports : <<TBS>>
  vports : 2.6.22
  bsg support : 2.6.30 （TBD

```

## 简

本文件记录了 SCSI FC 传输（Transport）的特性和组件。它也提供了
传输层与 FC LLDD 之间API 文档

```

  drivers/scsi/scsi_transport_fc.c
  include/scsi/scsi_transport_fc.h
  include/scsi/scsi_netlink_fc.h
  include/scsi/scsi_bsg_fc.h


```

本文件位Documentation/scsi/scsi_fc_transport.rst


## FC 远程端口（rports

  在光纤通道（Fibre Channel，FC）子系统中，远程端口（rport）指的是本地端口
  能够与之通信的远程光纤通道节点。它们通常是存储目标（例如磁盘阵列、磁带机），
  通过 FC 传输响应 SCSI 命令
  Linux 中，rports FC 传输类管理，并在 sysfs 中以如下路径表示
    /sys/class/fc_remote_ports/

  每个 rport 目录包含描述该远程端口的属性，例如端口 ID、节点名  端口状态和链路速度
  rports 通常FC 传输fabric 登录或扫描过程中发现新设备时创建  并一直存在，直到设备被移除或链路丢失
  常见属性：
  - node_name：World Wide Node Name（WWNN，全球节点名）  - port_name：World Wide Port Name（WWPN，全球端口名）  - port_id：远程端口的 FC 地址  - roles：指示该端口initiator（发起方）、target（目标），还是两者兼备  - port_state：显示当前运行状态
  发现远程端口后，驱动通常会填充一fc_rport_identifiers 结构，并调用
  fc_remote_port_add() 来通过光纤通道（FC）传输类创建该远程端口并  SCSI 子系统注册
  rports 也可以通过 sysfs 作为 FC 主机适配器的子对象可见
  对开发者而言：在实现FC 传输类交互的驱动时，请使  fc_remote_port_add() fc_remote_port_delete()

## FC 虚拟端口（vports

### 概述


  新的 FC 标准定义了允许单个物理端口表现为多个通信端口的机制。使  N_Port Id 虚拟化（NPIV）机制，Fabric 的点对点连接可以被分配多1   N_Port_ID。每N_Port_ID fabric 上的其他端点而言表现为一个独立的端口  尽管它与交换机共享一条物理链路进行通信。每N_Port_ID 可以基于 fabric
  分区（zoning）和阵列 LUN 掩码拥有fabric 的独特视图（就像普通的NPIV
  适配器一样）。使用虚Fabric（VF）机制，为每个帧添加 fabric 头部使端  能够Fabric Port 交互以加入多fabric。端口将在其加入的每fabric   获得一N_Port_ID。每fabric 都将拥有自己对端点和配置参数的独特视图  NPIV 可与 VF 一起使用，以便端口能在每个虚拟 fabric 上获得多N_Port_ID
  FC 传输现在引入了一个新的对象——vport。vport 是一个拥有全球唯一  World Wide Port Name（wwpn）和 World Wide Node Name（wwnn）的实体。传输层
  还允许为 vport 指定 FC4 角色，其FCP_Initiator 是预期的主要角色。一  通过上述某种方法实例化，它将拥有一个独特的 N_Port_ID 以及fabric 端点  存储实体的视图。与物理适配器关联的 fc_host 将导出创vport 的能力。传输层
  将在 Linux 设备树中创建 vport 对象，并指示 fc_host 的驱动实例化该虚拟端口  通常，驱动会vport 上创建一个新scsi_host 实例，从而为 vport 产生一  独特<H,C,T,L> 命名空间。因此，无论 FC 端口是基于物理端口还是虚拟端口，
  每个都将表现为一个具有自target LUN 空间的独scsi_host

```

    At this time, the transport is written to create only NPIV-based
    vports. However, consideration was given to VF-based vports and it
    should be a minor change to add support if needed.  The remaining
    discussion will concentrate on NPIV.

  .. Note::
    World Wide Name assignment (and uniqueness guarantees) are left
    up to an administrative entity controlling the vport. For example,
    if vports are to be associated with virtual machines, a XEN mgmt
    utility would be responsible for creating wwpn/wwnn's for the vport,
    using its own naming authority and OUI. (Note: it already does this
    for virtual MAC addresses).


```

### 设备树与 Vport 对象

  如今，设备树通常包含 scsi_host 对象，其下方rports scsi target
  对象。目FC 传输会创vport 对象，并将其放置在对应于物理适配器的
  scsi_host 对象之下。LLDD 会为 vport 分配一个新scsi_host，并将其对象
  链接vport 之下。vport scsi_host 之下的其余树结构与非 NPIV 情况相同  传输层的当前实现很容易允vport 的父对象不是 scsi_host。未来这可用于将
  对象链接到特定于虚拟机的设备树。如vport 的父对象不是物理端口  scsi_host，则会在物理端口scsi_host 中放置一个指vport 对象的符号链接
  以下是设备树中可预期的内容：


```

     /sys/devices/.../host17/

   and it has the typical descendant tree::

     /sys/devices/.../host17/rport-17:0-0/target17:0:0/17:0:0:0:

   and then the vport is created on the Physical Port::

     /sys/devices/.../host17/vport-17:0-0

   and the vport's Scsi_Host is then created::

     /sys/devices/.../host17/vport-17:0-0/host18

   and then the rest of the tree progresses, such as::

     /sys/devices/.../host17/vport-17:0-0/host18/rport-18:0-0/target18:0:0/18:0:0:0:

  Here's what to expect in the sysfs tree::

   scsi_hosts:
     /sys/class/scsi_host/host17                physical port's scsi_host
     /sys/class/scsi_host/host18                vport's scsi_host
   fc_hosts:
     /sys/class/fc_host/host17                  physical port's fc_host
     /sys/class/fc_host/host18                  vport's fc_host
   fc_vports:
     /sys/class/fc_vports/vport-17:0-0          the vport's fc_vport
   fc_rports:
     /sys/class/fc_remote_ports/rport-17:0-0    rport on the physical port
     /sys/class/fc_remote_ports/rport-18:0-0    rport on the vport


```

### Vport 属

  新的 fc_vport 类对象具有以下属性：

     node_name:                                                 Read_Only
       vport 鐨?WWNN銆。
     port_name:                                                 Read_Only
       vport 鐨?WWPN銆。
     roles:                                                     Read_Only
       指示vport 上启用的 FC4 角色
     symbolic_name:                                             Read_Write
       一个字符串，附加到驱动symbolic port name 字符串之后，该字符串
       会被注册到交换机以标vport。例如，hypervisor 可以将此字符串设置为
       "Xen Domain 2 VM 5 Vport 2"，这组标识符可在交换机管理界面上看到       用以标识该端口
     vport_delete:                                              Write_Only
       写入 "1" 时，将拆除该 vport
     vport_disable:                                            Write_Only
       写入 "1" 时，将把 vport 转换disabled（禁用）状态       vport 仍会Linux 内核中实例化，但不会FC 链路上处于活动状态       写入 "0" 时，将启用该 vport
     vport_last_state:                                         Read_Only
       指示 vport 的前一个状态。参见下文“Vport 状态”一节
     vport_state:                                              Read_Only
       指示 vport 的状态。参见下文“Vport 状态”一节
     vport_type:                                               Read_Only
       反映用于创建该虚拟端口的 FC 机制       目前仅支NPIV

  对于 fc_host 类对象，vports 添加了以下属性：

     max_npiv_vports:                                          Read_Only
       指示驱动/适配器在fc_host 上能够支持的基于 NPIV vport 的最大数量
     npiv_vports_inuse:                                        Read_Only
       指示已在 fc_host 上实例化的基NPIV vport 数量
     vport_create:                                             Write_Only
       一个“简单”的创建接口，用于在 fc_host 上实例化一vport       向该属性写入一"<WWPN>:<WWNN>" 字符串。随后传输层会实例化 vport 对象       并调LLDD FCP_Initiator 角色创建vport。每WWN 指定16        十六进制字符，且**不能**包含任何前缀（例0x、x 等）
     vport_delete:                                             Write_Only
       一个“简单”的删除接口，用于拆除一vport。向该属性写入一       "<WWPN>:<WWNN>" 字符串。传输层会在 fc_host 上找到具有相WWN vport
       并将其拆除。每WWN 指定16 个十六进制字符，*不能**包含任何前缀
       （例0x、x 等）

### Vport 状

  Vport 实例化由两部分组成：

    - 与内核和 LLDD 一起创建。这意味着所有传输层和驱动的数据结构被建立，
      并且设备对象被创建。这等效于在适配器上的驱动“attach（附加）”，
      它与适配器的链路状态无关    - 通过 ELS 流量等在 FC 链路上实例化 vport。这等效于“link up（链路就绪）      以及成功的链路初始化
  更多信息可在下文Vport Creation 接口一节中找到
  一vport 已与内核/LLDD 一起实例化，就可以通过 sysfs 属性报vport 状态  存在以下几种状态：

    FC_VPORT_UNKNOWN            - Unknown（未知）
      一个临时状态，通常仅在 vport 正在与内核和 LLDD 一起实例化时设置
    FC_VPORT_ACTIVE             - Active（活动）
      vport 已成功在 FC 链路上创建。它功能完备
    FC_VPORT_DISABLED           - Disabled（禁用）
      vport 已实例化，但处于“disabled”状态。该 vport 未在 FC 链路上实例化      这等效于链路“down（断开）”的物理端口
    FC_VPORT_LINKDOWN           - Linkdown（链路断开      vport 不可运行，因为物理链路不可运行
    FC_VPORT_INITIALIZING       - Initializing（初始化中）
      vport 正在 FC 链路上实例化的过程中。LLDD 将在开始用于创vport       ELS 流量之前设置此状态。此状态将持续，直vport 成功创建（状态变      FC_VPORT_ACTIVE）或失败（状态变为下述某个值）。由于此状态是瞬态的      它不会被保留"vport_last_state" 中
    FC_VPORT_NO_FABRIC_SUPP     - No Fabric Support（无 Fabric 支持      vport 不可运行。遇到了以下条件之一
       - FC 拓扑不是点对点（Point-to-Point）       - FC 端口未连接到 F_Port       - F_Port 表示不支NPIV
    FC_VPORT_NO_FABRIC_RSCS     - No Fabric Resources（无 Fabric 资源      vport 不可运行。Fabric FDISC 失败，其状态表明它没有足够的资源来完成
      该操作
    FC_VPORT_FABRIC_LOGOUT      - Fabric Logout（Fabric 注销      vport 不可运行。Fabric 已对与该 vport 关联N_Port_ID 执行LOGO
    FC_VPORT_FABRIC_REJ_WWN     - Fabric Rejected WWN（Fabric 拒绝 WWN      vport 不可运行。Fabric FDISC 失败，其状态表WWN 无效
    FC_VPORT_FAILED             - VPort Failed（VPort 失败      vport 不可运行。这是所有其他错误条件的兜底状态

  以下状态表列出了不同的状态转换：

   +------------------+--------------------------------+---------------------+
   | State            | Event                          | New State           |
   +==================+================================+=====================+
   | n/a              | Initialization                 | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Unknown:         | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & Loop                 | No Fabric Support   |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & no Fabric            | No Fabric Support   |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & FLOGI response       | No Fabric Support   |
   |                  | indicates no NPIV support      |                     |
   |                  +--------------------------------+---------------------+
   |                  | Link Up & FDISC being sent     | Initializing        |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Linkdown:        | Link Up                        | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Initializing:    | FDISC ACC                      | Active              |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT w/ no resources   | No Fabric Resources |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT w/ invalid        | Fabric Rejected WWN |
   |		      | pname or invalid nport_id      |                     |
   |                  +--------------------------------+---------------------+
   |                  | FDISC LS_RJT failed for        | Vport Failed        |
   |                  | other reasons                  |                     |
   |                  +--------------------------------+---------------------+
   |                  | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Disable:         | Enable request                 | Unknown             |
   +------------------+--------------------------------+---------------------+
   | Active:          | LOGO received from fabric      | Fabric Logout       |
   |                  +--------------------------------+---------------------+
   |                  | Link Down                      | Linkdown            |
   |                  +--------------------------------+---------------------+
   |                  | Disable request                | Disable             |
   +------------------+--------------------------------+---------------------+
   | Fabric Logout:   | Link still up                  | Unknown             |
   +------------------+--------------------------------+---------------------+

```

    No Fabric Support:
    No Fabric Resources:
    Fabric Rejected WWN:
    Vport Failed:
                        Disable request                 Disable
                        Link goes down                  Linkdown


```

### 传输<-> LLDD 接口


LLDD vport 的支持：

  LLDD 通过在传输模板中提供 vport_create() 函数来表明对 vports 的支持  该函数的存在会导致在 fc_host 上创建新的属性。作为物理端口相对于传输  完成其初始化的一部分，它应当设置 max_npiv_vports 属性，以指示驱动和/  适配器所支持vport 的最大数量

Vport 创建（Vport Creation）：

```

      int vport_create(struct fc_vport *vport, bool disable)

  where:

      =======   ===========================================================
      vport     Is the newly allocated vport object
      disable   If "true", the vport is to be created in a disabled stated.
                If "false", the vport is to be enabled upon creation.
      =======   ===========================================================

  When a request is made to create a new vport (via sgio/netlink, or the
  vport_create fc_host attribute), the transport will validate that the LLDD
  can support another vport (e.g. max_npiv_vports > npiv_vports_inuse).
  If not, the create request will be failed.  If space remains, the transport
  will increment the vport count, create the vport object, and then call the
  LLDD's vport_create() function with the newly allocated vport object.

  As mentioned above, vport creation is divided into two parts:

    - Creation with the kernel and LLDD. This means all transport and
      driver data structures are built up, and device objects created.
      This is equivalent to a driver "attach" on an adapter, which is
      independent of the adapter's link state.
    - Instantiation of the vport on the FC link via ELS traffic, etc.
      This is equivalent to a "link up" and successful link initialization.

  The LLDD's vport_create() function will not synchronously wait for both
  parts to be fully completed before returning. It must validate that the
  infrastructure exists to support NPIV, and complete the first part of
  vport creation (data structure build up) before returning.  We do not
  hinge vport_create() on the link-side operation mainly because:

    - The link may be down. It is not a failure if it is. It simply
      means the vport is in an inoperable state until the link comes up.
      This is consistent with the link bouncing post vport creation.
    - The vport may be created in a disabled state.
    - This is consistent with a model where:  the vport equates to a
      FC adapter. The vport_create is synonymous with driver attachment
      to the adapter, which is independent of link state.

  .. Note::

      special error codes have been defined to delineate infrastructure
      failure cases for quicker resolution.

  The expected behavior for the LLDD's vport_create() function is:

    - Validate Infrastructure:

        - If the driver or adapter cannot support another vport, whether
            due to improper firmware, (a lie about) max_npiv, or a lack of
            some other resource - return VPCERR_UNSUPPORTED.
        - If the driver validates the WWN's against those already active on
            the adapter and detects an overlap - return VPCERR_BAD_WWN.
        - If the driver detects the topology is loop, non-fabric, or the
            FLOGI did not support NPIV - return VPCERR_NO_FABRIC_SUPP.

    - Allocate data structures. If errors are encountered, such as out
        of memory conditions, return the respective negative Exxx error code.
    - If the role is FCP Initiator, the LLDD is to :

        - Call scsi_host_alloc() to allocate a scsi_host for the vport.
        - Call scsi_add_host(new_shost, &vport->dev) to start the scsi_host
          and bind it as a child of the vport device.
        - Initializes the fc_host attribute values.

    - Kick of further vport state transitions based on the disable flag and
        link state - and return success (zero).

  LLDD Implementers Notes:

  - It is suggested that there be a different fc_function_templates for
    the physical port and the virtual port.  The physical port's template
    would have the vport_create, vport_delete, and vport_disable functions,
    while the vports would not.
  - It is suggested that there be different scsi_host_templates
    for the physical port and virtual port. Likely, there are driver
    attributes, embedded into the scsi_host_template, that are applicable
    for the physical port only (link speed, topology setting, etc). This
    ensures that the attributes are applicable to the respective scsi_host.


```

Vport 禁用/启用（Vport Disable/Enable）：

```

      int vport_disable(struct fc_vport *vport, bool disable)

  where:

      =======   =======================================
      vport     Is vport to be enabled or disabled
      disable   If "true", the vport is to be disabled.
                If "false", the vport is to be enabled.
      =======   =======================================

  When a request is made to change the disabled state on a vport, the
  transport will validate the request against the existing vport state.
  If the request is to disable and the vport is already disabled, the
  request will fail. Similarly, if the request is to enable, and the
  vport is not in a disabled state, the request will fail.  If the request
  is valid for the vport state, the transport will call the LLDD to
  change the vport's state.

  Within the LLDD, if a vport is disabled, it remains instantiated with
  the kernel and LLDD, but it is not active or visible on the FC link in
  any way. (see Vport Creation and the 2 part instantiation discussion).
  The vport will remain in this state until it is deleted or re-enabled.
  When enabling a vport, the LLDD reinstantiates the vport on the FC
  link - essentially restarting the LLDD statemachine (see Vport States
  above).


```

Vport 删除（Vport Deletion）：

```

      int vport_delete(struct fc_vport *vport)

  where:

      vport:    Is vport to delete

  When a request is made to delete a vport (via sgio/netlink, or via the
  fc_host or fc_vport vport_delete attributes), the transport will call
  the LLDD to terminate the vport on the FC link, and teardown all other
  datastructures and references.  If the LLDD completes successfully,
  the transport will teardown the vport objects and complete the vport
  removal.  If the LLDD delete request fails, the vport object will remain,
  but will be in an indeterminate state.

  Within the LLDD, the normal code paths for a scsi_host teardown should
  be followed. E.g. If the vport has a FCP Initiator role, the LLDD
  will call fc_remove_host() for the vports scsi_host, followed by
  scsi_remove_host() and scsi_host_put() for the vports scsi_host.


```

其他（Other）：
  fc_host port_type 属性：
    有一个新fc_host port_type 取值——FC_PORTTYPE_NPIV。此取值必须在所    基于 vport fc_host 上设置。通常，在物理端口上，port_type 属性会基于
    拓扑类型fabric 的存在被设置NPORT、NLPORT 等。由于这不适用vport    因此报告用于创建vport FC 机制更为合理
  驱动卸载（Driver unload）：
    FC 驱动在被调用 scsi_remove_host() 之前必须先调fc_remove_host()    这允fc_host scsi_host 被拆除之前先拆除所有远程端口。fc_remove_host()
    调用也已更新，会同时移除fc_host 的所vport

### 传输层提供的函数


The following functions are supplied by the FC-transport for use by LLDs.

   ==================   =========================
   fc_vport_create      create a vport
   fc_vport_terminate   detach and remove a vport
   ==================   =========================

```

    /**
    * fc_vport_create - Admin App or LLDD requests creation of a vport
    * @shost:     scsi host the virtual port is connected to.
    * @ids:       The world wide names, FC4 port roles, etc for
    *              the virtual port.
    *
    * Notes:
    *     This routine assumes no locks are held on entry.
    */
    struct fc_vport *
    fc_vport_create(struct Scsi_Host *shost, struct fc_vport_identifiers *ids)

    /**
    * fc_vport_terminate - Admin App or LLDD requests termination of a vport
    * @vport:      fc_vport to be terminated
    *
    * Calls the LLDD vport_delete() function, then deallocates and removes
    * the vport from the shost and object tree.
    *
    * Notes:
    *      This routine assumes no locks are held on entry.
    */
    int
    fc_vport_terminate(struct fc_vport *vport)


```

## FC BSG 支持（CT & ELS 透传，以及更多）


<< To Be Supplied >>



## 致谢


The following people have contributed to this document:





James Smart
james.smart@broadcom.com
