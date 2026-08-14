## 简介

RapidIO 标准是一种基于数据包的互连结构（fabric interconnect）标准，设计用于
嵌入式系统。RapidIO 标准的开发由 RapidIO Trade Association（RTA，RapidIO 贸易
协会）主导。当前版本的 RapidIO 规范可以从 RTA 网站公开下载 [^1^]。

本文档描述了 Linux RapidIO 子系统的基础知识，并提供了关于其主要组件的信息。

## 1 概述

由于 RapidIO 子系统遵循 Linux 设备模型，它通过定义 RapidIO 特有的设备和
总线类型、并把它们注册到设备模型中，从而类似于其它总线一样集成到内核里。

Linux RapidIO 子系统是与体系结构无关的，因此它定义了体系结构相关的接口
来为通用的 RapidIO 子系统操作提供支持。

## 2. 核心组件

一个典型的 RapidIO 网络是端点（endpoint）和交换机（switch）的组合。这些
组件中的每一个在子系统中都由相关联的数据结构来表示。RapidIO 子系统的核心
逻辑组件定义在 include/linux/rio.h 文件中。

### 2.1 主端口（Master Port）

主端口（或称 mport）是一个对正在执行 Linux 代码的处理器来说本地的 RapidIO
接口控制器。主端口生成并接收 RapidIO 数据包（事务）。在 RapidIO 子系统中，
每个主端口都由 rio_mport 数据结构表示。该结构包含主端口特有的资源，例如
邮箱（mailbox）和门铃（doorbell）。rio_mport 还包含一个唯一的宿主设备 ID，
当主端口被配置为执行枚举的宿主（host）时该 ID 有效。

RapidIO 主端口由子系统特有的 mport 设备驱动提供服务，这些驱动提供了为该
子系统定义的功能。为了给 RapidIO 子系统操作提供一个与硬件无关的接口，
rio_mport 结构包含了 rio_ops 数据结构，其中含有指向 RapidIO 函数硬件相关
实现的指针。

### 2.2 设备

RapidIO 设备是网络中除 mport 之外的任何端点或交换机。所有设备都由相应的
rio_dev 数据结构呈现在 RapidIO 子系统中。设备形成了一个全局设备列表，以及
每个网络的设备列表（取决于可用 mport 和网络的数量）。

### 2.3 交换机

RapidIO 交换机是一类特殊的设备，它在自己的各个端口之间把数据包路由向其
最终目的地。交换机内部数据包的目的端口由内部路由表定义。交换机在 RapidIO
子系统中由 rio_dev 数据结构表示，并扩展了额外的 rio_switch 数据结构，其中
包含交换机特有的信息，例如路由表的副本以及指向交换机特有函数的指针。

RapidIO 子系统为子系统特有的交换机驱动定义了格式和初始化方法，这些驱动
旨在为通用的交换机管理例程提供硬件相关的实现。

### 2.4 网络

RapidIO 网络是互连的端点和交换机设备的组合。系统所知的每个 RapidIO 网络
都由相应的 rio_net 数据结构表示。该结构包含了构成同一个网络的所有设备和
本地主端口的列表。它还包含一个指向默认主端口的指针，该主端口用于与网络
内的设备通信。

### 2.5 设备驱动

RapidIO 设备特有的驱动遵循 Linux 内核驱动模型，旨在支持挂接在 RapidIO 网络
上的特定 RapidIO 设备。

### 2.6 子系统接口

RapidIO 互连规范定义了可用于为所有参与的 RapidIO 设备提供一个或多个通用
服务层（service layer）的特性。这些通用服务可以独立于设备特有的驱动运行，
也可以被设备特有的驱动使用。此类服务提供者的一个例子是 RIONET 驱动，它
实现了 Ethernet-over-RapidIO（基于 RapidIO 的以太网）接口。因为对于一个设备
只能注册一个驱动，所有通用的 RapidIO 服务都必须注册为子系统接口。这允许
把多个通用服务挂接到同一个设备上，而不会阻塞设备特有驱动的挂接。

## 3. 子系统初始化

为了初始化 RapidIO 子系统，一个平台必须初始化并在 RapidIO 网络中注册至少
一个主端口。要在子系统中注册 mport，控制器驱动的初始化代码会针对每个可用
的主端口调用函数 rio_register_mport()。

在所有的活动主端口都注册到 RapidIO 子系统之后，枚举（enumeration）和/或
发现（discovery）例程可能会被自动调用，或者被用户空间的命令调用。

RapidIO 子系统可以被配置为内建链接（statically linked）的或模块化（modular）
的内核组件（详见下文）。

## 4. 枚举与发现

### 4.1 概述

RapidIO 子系统配置选项允许用户把枚举和发现方法构建为内建链接的组件或可加载
模块。枚举/发现方法的实现以及可用的输入参数，定义了给定的方法可以如何挂接到
可用的 RapidIO mport 上：简单地挂接到所有可用 mport，或者单独挂接到指定的
mport 设备。

根据所选的枚举/发现构建配置，有几种发起枚举和/或发现过程的方法：

  (a) 内建链接的枚举和发现过程可以在内核初始化期间使用相应的模块参数
  自动启动。这是自 RapidIO 子系统引入以来使用的原始方法。现在该方法依赖
  于枚举器模块参数，对于现有的基础枚举/发现方法，该参数为
  'rio-scan.scan'。当使用枚举/发现的自动启动时，用户必须确保所有的发现端点在
  枚举端点之前启动，并等待枚举完成。配置选项 CONFIG_RAPIDIO_DISC_TIMEOUT
  定义了发现端点等待枚举完成的时间。如果指定的超时到期，发现过程会终止，
  而无法获得 RapidIO 网络信息。注意：超时的发现过程可以在稍后使用用户空间
  命令重新启动（如下文所述），前提是该给定端点已被成功枚举。

  (b) 内建链接的枚举和发现过程可以由来自用户空间的命令启动。与上述选项 (a)
  相比，这种发起方式为系统启动提供了更大的灵活性。在所有参与的端点都成功
  启动之后，应当先通过发出用户空间命令来启动枚举过程，枚举完成之后，才可以在
  其余所有端点上启动发现过程。

  (c) 模块化的枚举和发现过程可以由来自用户空间的命令启动。在枚举/发现模块
  被加载之后，可以通过发出用户空间命令来启动网络扫描过程。与上述选项 (b)
  类似，必须先启动枚举器。

  (d) 模块化的枚举和发现过程可以由模块初始化例程启动。在这种情况下，应当先
  加载枚举模块。

当网络扫描过程启动时，它会根据主端口所配置的角色——宿主（host）或代理
（agent）——调用枚举或发现例程。

如果某个主端口被配置为主端口（host port），即分配一个大于或等于零的宿主
目标 ID，则由该主端口执行枚举。宿主目标 ID 可以根据 RapidIO 子系统的构建
配置使用多种方法分配给主端口：

  (a) 对于内建链接的 RapidIO 子系统核心，使用命令行参数 "rapidio.hdid=" 加上
  按 mport 设备注册顺序排列的目标 ID 赋值列表。例如，在一个有两个 RapidIO
  控制器的系统中，命令行参数 "rapidio.hdid=-1,7" 会导致把宿主目标 ID=7 赋值
  给第二个 RapidIO 控制器，而第一个控制器则被赋值目标 ID=-1。

  (b) 如果 RapidIO 子系统核心被构建为可加载模块，除了上面所示的方法之外，
  宿主目标 ID 还可以使用传统的、在加载时传递模块参数 "hdid=" 的方法来指定：

  - 从命令行："modprobe rapidio hdid=-1,7"，或
  - 从 modprobe 配置文件使用配置命令 "options"，如本例：
    "options rapidio hdid=-1,7"。modprobe 配置文件的一个例子在下文中提供。

注意：
  (i) 如果省略了 "hdid=" 参数，所有可用的 mport 都将被赋值目标 ID = -1；

  (ii) 在具有多个 mport 的系统中，"hdid=" 参数的目标 ID 赋值可以从列表末尾
  省略（默认 = -1）。

如果某个特定主端口的宿主设备 ID 被设为 -1，则将为它执行发现过程。

枚举和发现例程使用 RapidIO 维护事务（maintenance transaction）来访问设备的
配置空间。

注意：如果 RapidIO 交换机特有的设备驱动被构建为可加载模块，它们必须在
枚举/发现过程开始之前加载。这一要求的原因在于：枚举/发现方法会在早期阶段
调用厂商特有的回调。

### 4.2 枚举与发现的自动启动

自动枚举/发现启动方法仅适用于内建（built-in）的枚举/发现 RapidIO 配置选择。
要启用由现有基础枚举器方法进行的自动枚举/发现启动，请使用启动命令行参数
"rio-scan.scan=1"。

该配置要求组成将被枚举/发现的网络的所有 RapidIO 端点同步启动。发现端点必须
在枚举开始之前启动，以确保所有的 RapidIO 控制器都已被初始化并准备好被
发现。配置参数 CONFIG_RAPIDIO_DISC_TIMEOUT 定义了发现端点将等待枚举完成
的时间（以秒为单位）。

当选择自动枚举/发现启动时，基础方法的初始化例程会调用 rio_init_mports() 来
为所有已知的 mport 设备执行枚举或发现。

由于要求所有端点同步启动，根据 RapidIO 网络的规模和配置，这种自动枚举/发现
启动方法可能难以使用。

### 4.3 枚举与发现的用户空间启动

枚举和发现的用户空间启动可用于内建和模块化两种构建配置。对于由用户空间
控制的启动，RapidIO 子系统会创建 sysfs 只写属性文件 '/sys/bus/rapidio/scan'。
要在特定的 mport 设备上发起枚举或发现过程，用户需要把 mport_ID（不是 RapidIO
目标 ID）写入该文件。mport_ID 是在 mport 设备注册期间分配的连续编号
（0 ... RIO_MAX_MPORTS）。例如，对于只有单个 RapidIO 控制器的机器，该控制器的
mport_ID 永远是 0。

要在所有可用的 mport 上发起 RapidIO 枚举/发现，用户可以把 '-1'（或
RIO_MPORT_ANY）写入 scan 属性文件。

### 4.4 基础枚举方法

这是自 RapidIO 子系统代码首次发布起就可用的原始枚举/发现方法。枚举过程根据
RapidIO 互连规范：附录 I [^1^] 中所概述的枚举算法来实现。

该方法可以被配置为内建链接或可加载模块。该方法的单一参数 "scan" 允许从
模块初始化例程触发枚举/发现过程。

这种枚举/发现方法只能启动一次，如果被构建为模块，则不支持卸载。

枚举过程使用递归的深度优先算法遍历网络。当发现一个新设备时，枚举器通过写入
Host Device ID Lock CSR 来获得该设备的所有权。它这样做是为了确保枚举器拥有
枚举该设备的独占权利。如果成功获取了设备的所有权，枚举器就分配一个新的
rio_dev 结构，并根据设备能力对其进行初始化。

如果设备是一个端点，就会给它分配一个唯一的设备 ID，并把其值写入设备的
Base Device ID CSR。

如果设备是一个交换机，枚举器会分配一个额外的 rio_switch 结构来存储交换机
特有的信息。然后，交换机的厂商 ID 和设备 ID 会对照一张已知 RapidIO 交换机的
表进行查询。每个交换机表项都包含一个指向交换机特有初始化例程的指针，该例程
会初始化到其余交换机特有操作的指针，并在必要时执行硬件初始化。RapidIO 交换机
没有唯一的设备 ID；如果需要访问其配置寄存器，它依赖跳数（hopcount）和所挂接
端点的路由来取得设备 ID。如果一个交换机（或一串交换机）没有挂接任何端点
（枚举器除外），就会分配一个伪设备 ID 来配置到该交换机的路由。在没有端点的
一串交换机的情况下，使用一个伪设备 ID 来配置穿越整串交换机的路由，而各个
交换机则通过它们的 hopcount 值来区分。

对于端点和交换机，枚举器都会把一个唯一的组件标签（component tag）写入设备的
Component Tag CSR。这个唯一的值被错误管理通知机制用来识别正在报告错误管理
事件的设备。

交换机之外的枚举通过遍历该交换机的每个活动输出端口来完成。对于每个活动链路，
一个到默认设备 ID（8 位系统为 0xFF，16 位系统为 0xFFFF）的路由会被临时写入
路由表。算法通过以 hopcount + 1 和默认设备 ID 调用自身来递归，以便访问活动
端口上的设备。

在宿主完成整个网络的枚举之后，它通过清除设备 ID 锁（调用 rio_clear_locks()）
来释放设备。对于系统中的每个端点，它会在 Port General Control CSR 中设置
Discovered 位，以指示枚举已完成，并允许代理执行对网络的被动发现。

发现过程由代理执行，并与上面描述的枚举过程类似。然而，发现过程是在不改变
现有路由的情况下执行的，因为代理只收集关于 RapidIO 网络结构的信息，并构建
已发现设备的内部映射。这样，RapidIO 子系统的每个基于 Linux 的组件都对网络有
一个完整的视图。发现过程可以由多个代理同时执行。在初始化其 RapidIO 主端口
之后，每个代理都会在配置好的等待时间段内等待宿主完成枚举。如果在该等待时间段
到期之前枚举仍未完成，代理就跳过 RapidIO 发现，并继续剩余的（剩余的）内核
初始化。

### 4.5 添加新的枚举/发现方法

RapidIO 子系统代码的组织方式允许把新的枚举/发现方法作为新的配置选项添加进来，
而对核心 RapidIO 代码的影响很小。

在枚举/发现过程可以启动之前，必须把新的枚举/发现方法挂接到一个或多个 mport
设备上。通常，该方法的模块初始化例程会调用 rio_register_scan() 来把枚举器
挂接到指定的 mport 设备（或多个设备）。基础的枚举器实现演示了这个过程。

### 4.6 使用可加载的 RapidIO 交换机驱动

在 RapidIO 交换机驱动被构建为可加载模块的情况下，用户必须确保它们在枚举/
发现开始之前已加载。如以下示例所示，可以通过在 RapidIO 特有的 modprobe 配置文件
中指定前置或后置依赖来把这个进程自动化。

```

  # Configure RapidIO subsystem modules

  # Set enumerator host destination ID (overrides kernel command line option)
  options rapidio hdid=-1,2

  # Load RapidIO switch drivers immediately after rapidio core module was loaded
  softdep rapidio post: idt_gen2 idtcps tsi57x

  # OR :

  # Load RapidIO switch drivers just before rio-scan enumerator module is loaded
  softdep rio-scan pre: idt_gen2 idtcps tsi57x

  --------------------------

```
注意：
  在上面例子中，必须删除或注释掉其中一条 "softdep" 命令，以保持所需的模块
  加载顺序。

## 5. 参考资料

[^1^] RapidIO Trade Association. RapidIO Interconnect Specifications.
    http://www.rapidio.org.

[^2^] Rapidio TA. Technology Comparisons.
    http://www.rapidio.org/education/technology_comparisons/

[^3^] RapidIO support for Linux.
    https://lwn.net/Articles/139118/

[^4^] Matt Porter. RapidIO for Linux. Ottawa Linux Symposium, 2005
    https://www.kernel.org/doc/ols/2005/ols2005v2-pages-43-56.pdf
