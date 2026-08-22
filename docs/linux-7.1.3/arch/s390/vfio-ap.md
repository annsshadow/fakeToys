## Adjunct Processor (AP) facility

## 辅助处理器（AP）设


## Introduction

## 简

The Adjunct Processor (AP) facility is an IBM Z cryptographic facility comprised
of three AP instructions and from 1 up to 256 PCIe cryptographic adapter cards.
The AP devices provide cryptographic functions to all CPUs assigned to a
linux system running in an IBM Z system LPAR.

辅助处理器（AP）设施是 IBM Z 的密码学设施，由三条 AP 指令以及 1 256 
PCIe 密码适配器卡组成。AP 设备为分配给运行IBM Z 系统 LPAR 中的 Linux 系统
所CPU 提供密码学功能

The AP adapter cards are exposed via the AP bus. The motivation for vfio-ap
is to make AP cards available to KVM guests using the VFIO mediated device
framework. This implementation relies considerably on the s390 virtualization
facilities which do most of the hard work of providing direct access to AP
devices.

AP 适配器卡通过 AP 总线暴露。vfio-ap 的动机是使用 VFIO 中介（mediated）设
框架AP 卡对 KVM 客户机可用。该实现在很大程度上依赖 s390 的虚拟化设施，后
完成了提AP 设备直接访问的大部分艰难工作

## AP Architectural Overview

## AP 架构概述

To facilitate the comprehension of the design, let's start with some
definitions:

为了便于理解该设计，让我们从一些定义开始：

- AP adapter

- AP 閫傞厤鍣。

  An AP adapter is an IBM Z adapter card that can perform cryptographic
  functions. There can be from 0 to 256 adapters assigned to an LPAR. Adapters
  assigned to the LPAR in which a linux host is running will be available to
  the linux host. Each adapter is identified by a number from 0 to 255; however,
  the maximum adapter number is determined by machine model and/or adapter type.
  When installed, an AP adapter is accessed by AP instructions executed by any
  CPU.

  一AP 适配器是一块能够执行密码学功能IBM Z 适配器卡。可以分配给一LPAR
  的适配器数量从 0 256 不等。分配给运行 Linux 主机LPAR 的适配器将
  Linux 主机可用。每个适配器由一0 255 之间的数字标识；不过，最大适配器号
  由机型（machine model）和/或适配器类型决定。安装后，AP 适配器由任何 CPU 执行
  AP 指令访问

  The AP adapter cards are assigned to a given LPAR via the system's Activation
  Profile which can be edited via the HMC. When the linux host system is IPL'd
  in the LPAR, the AP bus detects the AP adapter cards assigned to the LPAR and
  creates a sysfs device for each assigned adapter. For example, if AP adapters
  4 and 10 (0x0a) are assigned to the LPAR, the AP bus will create the following

  AP 适配器卡通过系统的激活概要（Activation Profile）分配给给定LPAR，该概要
  可通过 HMC 编辑。当 Linux 主机系统在该 LPAR IPL 后，AP 总线会检测分配给
  LPAR AP 适配器卡，并为每个被分配的适配器创建一sysfs 设备。例如，如果
  AP 适配4 10x0a）被分配给该 LPAR，AP 总线将创建以

```

    /sys/devices/ap/card04
    /sys/devices/ap/card0a

  Symbolic links to these devices will also be created in the AP bus devices
  sub-directory::

    /sys/bus/ap/devices/[card04]
    /sys/bus/ap/devices/[card04]

```

- AP domain

- AP 鍩。

  An adapter is partitioned into domains. An adapter can hold up to 256 domains
  depending upon the adapter type and hardware configuration. A domain is
  identified by a number from 0 to 255; however, the maximum domain number is
  determined by machine model and/or adapter type.. A domain can be thought of
  as a set of hardware registers and memory used for processing AP commands. A
  domain can be configured with a secure private key used for clear key
  encryption. A domain is classified in one of two ways depending upon how it
  may be accessed:

  一个适配器被划分为多个域。根据适配器类型和硬件配置，一个适配器最多可容纳 256 
  域。一个域由一0 255 之间的数字标识；不过，最大域名由机型（machine model
  或适配器类型决定。一个域可以被看作是一组用于处AP 命令的硬件寄存器和内存
  一个域可以配置一个用于明文密钥（clear key）加密的安全私钥。根据域的访问方式，
  域被分为两类

    - Usage domains are domains that are targeted by an AP instruction to
      process an AP command.

    - 使用域（Usage domain）是指被 AP 指令作为目标以处AP 命令的域

    - Control domains are domains that are changed by an AP command sent to a
      usage domain; for example, to set the secure private key for the control
      domain.

    - 控制域（Control domain）是指由发往使用域的 AP 命令更改的域；例如，为控制域
      设置安全私钥

  The AP usage and control domains are assigned to a given LPAR via the system's
  Activation Profile which can be edited via the HMC. When a linux host system
  is IPL'd in the LPAR, the AP bus module detects the AP usage and control
  domains assigned to the LPAR. The domain number of each usage domain and
  adapter number of each AP adapter are combined to create AP queue devices
  (see AP Queue section below). The domain number of each control domain will be
  represented in a bitmask and stored in a sysfs file
  /sys/bus/ap/ap_control_domain_mask. The bits in the mask, from most to least
  significant bit, correspond to domains 0-255.

  AP 使用域和控制域通过系统的激活概要分配给给定LPAR，该概要可通过 HMC 编辑
  Linux 主机系统在该 LPAR IPL 后，AP 总线模块会检测分配给LPAR AP 使用
  和控制域。每个使用域的域名和每个 AP 适配器的适配器号被组合起来创AP 队列设备
  （见下文"AP 队列"一节）。每个控制域的域名将用一个位掩码表示，并存储sysfs 文件
  /sys/bus/ap/ap_control_domain_mask 中。掩码中的位，从最高有效位到最低有效位
  分别对应0-255

- AP Queue

- AP 队列

  An AP queue is the means by which an AP command is sent to a usage domain
  inside a specific adapter. An AP queue is identified by a tuple
  comprised of an AP adapter ID (APID) and an AP queue index (APQI). The
  APQI corresponds to a given usage domain number within the adapter. This tuple
  forms an AP Queue Number (APQN) uniquely identifying an AP queue. AP
  instructions include a field containing the APQN to identify the AP queue to
  which the AP command is to be sent for processing.

  AP 队列是将 AP 命令发送到特定适配器内部使用域的手段。一AP 队列由一个元组标识，
  该元组由 AP 适配ID（APID）和 AP 队列索引（APQI）组成。APQI 对应于适配器内
  给定的使用域号。这个元组构成一AP 队列号（APQN），唯一地标识一AP 队列。AP
  指令包含一个存APQN 的字段，用于标识要将 AP 命令发往哪个 AP 队列进行处理

  The AP bus will create a sysfs device for each APQN that can be derived from
  the cross product of the AP adapter and usage domain numbers detected when the
  AP bus module is loaded. For example, if adapters 4 and 10 (0x0a) and usage
  domains 6 and 71 (0x47) are assigned to the LPAR, the AP bus will create the

  AP 总线会为可从 AP 总线模块加载时检测到AP 适配器号和用法域名叉积推导出的每
  APQN 创建一sysfs 设备。例如，如果适配4 10x0a）以及使用域 6 71
  x47）被分配给该 LPAR，AP 总线将创建以

```

    /sys/devices/ap/card04/04.0006
    /sys/devices/ap/card04/04.0047
    /sys/devices/ap/card0a/0a.0006
    /sys/devices/ap/card0a/0a.0047

  The following symbolic links to these devices will be created in the AP bus
  devices subdirectory::

    /sys/bus/ap/devices/[04.0006]
    /sys/bus/ap/devices/[04.0047]
    /sys/bus/ap/devices/[0a.0006]
    /sys/bus/ap/devices/[0a.0047]

```

- AP Instructions:

- AP 指令

  There are three AP instructions:

  有三AP 指令

  - NQAP: to enqueue an AP command-request message to a queue
  - DQAP: to dequeue an AP command-reply message from a queue
  - PQAP: to administer the queues

  - NQAP：将一AP 命令请求消息入队到某个队
  - DQAP：从一个队列中将一AP 命令应答消息出队
  - PQAP：管理这些队

  AP instructions identify the domain that is targeted to process the AP
  command; this must be one of the usage domains. An AP command may modify a
  domain that is not one of the usage domains, but the modified domain
  must be one of the control domains.

  AP 指令标识作为目标处理 AP 命令的域；这必须是使用域之一。一AP 命令可以修改一
  并非使用域的域，但被修改的域必须是控制域之一

## AP and SIE

## AP 涓?SIE

Let's now take a look at how AP instructions executed on a guest are interpreted
by the hardware.

现在让我们看看在客户机上执行AP 指令是如何被硬件解释的

A satellite control block called the Crypto Control Block (CRYCB) is attached to
our main hardware virtualization control block. The CRYCB contains an AP Control
Block (APCB) that has three fields to identify the adapters, usage domains and
control domains assigned to the KVM guest:

一个称为密码控制块（Crypto Control Block，CRYCB）的辅助控制块被附加到我们的主硬
虚拟化控制块上。CRYCB 包含一AP 控制块（APCB），它有三个字段来标识分配给 KVM
客户机的适配器、使用域和控制域

- The AP Mask (APM) field is a bit mask that identifies the AP adapters assigned
  to the KVM guest. Each bit in the mask, from left to right, corresponds to
  an APID from 0-255. If a bit is set, the corresponding adapter is valid for
  use by the KVM guest.

- AP 掩码（APM）字段是一个位掩码，标识分配给 KVM 客户机的 AP 适配器。掩码中的位
  从左到右，分别对应一个从 0-255 APID。如果某位被置位，则相应适配器可KVM
  客户机使用

- The AP Queue Mask (AQM) field is a bit mask identifying the AP usage domains
  assigned to the KVM guest. Each bit in the mask, from left to right,
  corresponds to an AP queue index (APQI) from 0-255. If a bit is set, the
  corresponding queue is valid for use by the KVM guest.

- AP 队列掩码（AQM）字段是一个位掩码，标识分配给 KVM 客户机的 AP 使用域。掩码中
  位，从左到右，分别对应一个从 0-255 AP 队列索引（APQI）。如果某位被置位，则
  相应队列可供 KVM 客户机使用

- The AP Domain Mask field is a bit mask that identifies the AP control domains
  assigned to the KVM guest. The ADM bit mask controls which domains can be
  changed by an AP command-request message sent to a usage domain from the
  guest. Each bit in the mask, from left to right, corresponds to a domain from
  0-255. If a bit is set, the corresponding domain can be modified by an AP
  command-request message sent to a usage domain.

- AP 域掩码字段是一个位掩码，标识分配给 KVM 客户机的 AP 控制域。ADM 位掩码控制哪
  域可以被从客户机发往使用域的 AP 命令请求消息更改。掩码中的位，从左到右，分别对应
  一个从 0-255 的域。如果某位被置位，则相应域可以被从客户机发往使用域的 AP 命令请求
  消息修改

If you recall from the description of an AP Queue, AP instructions include
an APQN to identify the AP queue to which an AP command-request message is to be
sent (NQAP and PQAP instructions), or from which a command-reply message is to
be received (DQAP instruction). The validity of an APQN is defined by the matrix
calculated from the APM and AQM; it is the Cartesian product of all assigned
adapter numbers (APM) with all assigned queue indexes (AQM). For example, if
adapters 1 and 2 and usage domains 5 and 6 are assigned to a guest, the APQNs
(1,5), (1,6), (2,5) and (2,6) will be valid for the guest.

如果你还记得 AP 队列的描述，AP 指令包含一APQN 以标识要AP 命令请求消息发往
（NQAP PQAP 指令）或从哪个队列接收命令应答消息（DQAP 指令）的 AP 队列。一
APQN 的有效性由APM AQM 计算出的矩阵定义；它是所有被分配的适配器号（APM
与所有被分配的队列索引（AQM）的笛卡尔积。例如，如果适配1 2 以及使用5 6
被分配到一个客户机，那APQN (1,5)1,6)2,5) (2,6) 对该客户机有效

The APQNs can provide secure key functionality - i.e., a private key is stored
on the adapter card for each of its domains - so each APQN must be assigned to
```

   Example 1: Valid configuration:
   ------------------------------
   Guest1: adapters 1,2  domains 5,6
   Guest2: adapter  1,2  domain 7

   This is valid because both guests have a unique set of APQNs:
      Guest1 has APQNs (1,5), (1,6), (2,5), (2,6);
      Guest2 has APQNs (1,7), (2,7)

   Example 2: Valid configuration:
   ------------------------------
   Guest1: adapters 1,2 domains 5,6
   Guest2: adapters 3,4 domains 5,6

   This is also valid because both guests have a unique set of APQNs:
      Guest1 has APQNs (1,5), (1,6), (2,5), (2,6);
      Guest2 has APQNs (3,5), (3,6), (4,5), (4,6)

   Example 3: Invalid configuration:
   --------------------------------
   Guest1: adapters 1,2  domains 5,6
   Guest2: adapter  1    domains 6,7

   This is an invalid configuration because both guests have access to
   APQN (1,6).

```

## The Design

## 设计

The design introduces three new objects:

该设计引入了三个新对象：

1. AP matrix device
2. VFIO AP device driver (vfio_ap.ko)
3. VFIO AP mediated pass-through device

1. AP 矩阵设备
2. VFIO AP 设备驱动（vfio_ap.ko
3. VFIO AP 中介直通（pass-through）设

### The VFIO AP device driver

### VFIO AP 设备驱动

The VFIO AP (vfio_ap) device driver serves the following purposes:

VFIO AP（vfio_ap）设备驱动用于以下目的：

1. Provides the interfaces to secure APQNs for exclusive use of KVM guests.

1. 提供接口以预APQN KVM 客户机独占使用

2. Sets up the VFIO mediated device interfaces to manage a vfio_ap mediated
   device and creates the sysfs interfaces for assigning adapters, usage
   domains, and control domains comprising the matrix for a KVM guest.

2. 建立 VFIO 中介设备接口以管理一vfio_ap 中介设备，并创建用于分配构成 KVM
   客户机矩阵的适配器、使用域和控制域sysfs 接口

3. Configures the APM, AQM and ADM in the APCB contained in the CRYCB referenced
   by a KVM guest's SIE state description to grant the guest access to a matrix
   of AP devices

3. 配置位于 KVM 客户SIE 状态描述所引用CRYCB 中的 APCB 内的 APM、AQM ADM
   以授予客户机对一AP 设备的访问权

### Reserve APQNs for exclusive use of KVM guests

### KVM 客户机独占使用而预APQN

The following block diagram illustrates the mechanism by which APQNs are
```

				+------------------+
		 7 remove       |                  |
	   +--------------------> cex4queue driver |
	   |                    |                  |
	   |                    +------------------+
	   |
	   |
	   |                    +------------------+          +----------------+
	   |  5 register driver |                  | 3 create |                |
	   |   +---------------->   Device core    +---------->  matrix device |
	   |   |                |                  |          |                |
	   |   |                +--------^---------+          +----------------+
	   |   |                         |
	   |   |                         +-------------------+
	   |   | +-----------------------------------+       |
	   |   | |      4 register AP driver         |       | 2 register device
	   |   | |                                   |       |
  +--------+---+-v---+                      +--------+-------+-+
  |                  |                      |                  |
  |      ap_bus      +--------------------- >  vfio_ap driver  |
  |                  |       8 probe        |                  |
  +--------^---------+                      +--^--^------------+
  6 edit   |                                   |  |
    apmask |     +-----------------------------+  | 11 mdev create
    aqmask |     |           1 modprobe           |
  +--------+-----+---+           +----------------+-+         +----------------+
  |                  |           |                  |10 create|     mediated   |
  |      admin       |           | VFIO device core |--------->     matrix     |
  |                  +           |                  |         |     device     |
  +------+-+---------+           +--------^---------+         +--------^-------+
	 | |                              |                            |
	 | | 9 create vfio_ap-passthrough |                            |
	 | +------------------------------+                            |
	 +-------------------------------------------------------------+
		     12  assign adapter/domain/control domain

```

The process for reserving an AP queue for use by a KVM guest is:

KVM 客户机预留一AP 队列的过程是

1. The administrator loads the vfio_ap device driver
2. The vfio-ap driver during its initialization will register a single 'matrix'
   device with the device core. This will serve as the parent device for
   all vfio_ap mediated devices used to configure an AP matrix for a guest.
3. The /sys/devices/vfio_ap/matrix device is created by the device core
4. The vfio_ap device driver will register with the AP bus for AP queue devices
   of type 10 and higher (CEX4 and newer). The driver will provide the vfio_ap
   driver's probe and remove callback interfaces. Devices older than CEX4 queues
   are not supported to simplify the implementation by not needlessly
   complicating the design by supporting older devices that will go out of
   service in the relatively near future, and for which there are few older
   systems around on which to test.
5. The AP bus registers the vfio_ap device driver with the device core
6. The administrator edits the AP adapter and queue masks to reserve AP queues
   for use by the vfio_ap device driver.
7. The AP bus removes the AP queues reserved for the vfio_ap driver from the
   default zcrypt cex4queue driver.
8. The AP bus probes the vfio_ap device driver to bind the queues reserved for
   it.
9. The administrator creates a passthrough type vfio_ap mediated device to be
   used by a guest
10. The administrator assigns the adapters, usage domains and control domains
    to be exclusively used by a guest.

1. 管理员加vfio_ap 设备驱动
2. vfio-ap 驱动在其初始化期间将向设备核心（device core）注册一个单一的“矩阵
   （matrix）设备。它将作为用于为客户机配AP 矩阵的所vfio_ap 中介设备的父设备
3. /sys/devices/vfio_ap/matrix 设备由设备核心创
4. vfio_ap 设备驱动将向 AP 总线注册以处理类型为 10 及更高（CEX4 及更新）AP 队列
   设备。该驱动将提vfio_ap 驱动probe remove 回调接口。不支持早于 CEX4 队列
   设备，这是为了通过不为在相对不久的将来会停止服务、且可用于测试的旧系统很少的旧设
   提供支持，从而避免不必要地使设计复杂化，以简化实现
5. AP 总线vfio_ap 设备驱动注册到设备核
6. 管理员编AP 适配器和队列掩码以预留供 vfio_ap 设备驱动使用AP 队列
7. AP 总线将从默认zcrypt cex4queue 驱动中移除为 vfio_ap 驱动预留AP 队列
8. AP 总线探测 vfio_ap 设备驱动以绑定为其预留的队列
9. 管理员创建一个直通类型的 vfio_ap 中介设备供客户机使用
10. 管理员分配供客户机独占使用的适配器、使用域和控制域

### Set up the VFIO mediated device interfaces

### 建立 VFIO 中介设备接口

The VFIO AP device driver utilizes the common interfaces of the VFIO mediated
device core driver to:

VFIO AP 设备驱动利用 VFIO 中介设备核心驱动的通用接口来：

- Register an AP mediated bus driver to add a vfio_ap mediated device to and
  remove it from a VFIO group.
- Create and destroy a vfio_ap mediated device
- Add a vfio_ap mediated device to and remove it from the AP mediated bus driver
- Add a vfio_ap mediated device to and remove it from an IOMMU group

- 注册一AP 中介总线驱动，以vfio_ap 中介设备加入或移VFIO 组
- 创建并销毁一vfio_ap 中介设备
- vfio_ap 中介设备加入或移AP 中介总线驱动
- vfio_ap 中介设备加入或移IOMMU 

The following high-level block diagram shows the main components and interfaces
```

   +-------------+
   |             |
   | +---------+ | mdev_register_driver() +--------------+
   | |  Mdev   | +<-----------------------+              |
   | |  bus    | |                        | vfio_mdev.ko |
   | | driver  | +----------------------->+              |<-> VFIO user
   | +---------+ |    probe()/remove()    +--------------+    APIs
   |             |
   |  MDEV CORE  |
   |   MODULE    |
   |   mdev.ko   |
   | +---------+ | mdev_register_parent() +--------------+
   | |Physical | +<-----------------------+              |
   | | device  | |                        |  vfio_ap.ko  |<-> matrix
   | |interface| +----------------------->+              |    device
   | +---------+ |       callback         +--------------+
   +-------------+

```

During initialization of the vfio_ap module, the matrix device is registered
with an 'mdev_parent_ops' structure that provides the sysfs attribute
structures, mdev functions and callback interfaces for managing the mediated
matrix device.

vfio_ap 模块的初始化期间，矩阵设备会用一'mdev_parent_ops' 结构注册，该结构
提供用于管理中介矩阵设备sysfs 属性结构、mdev 函数和回调接口

- sysfs attribute structures:

- sysfs 属性结构：

  supported_type_groups
    The VFIO mediated device framework supports creation of user-defined
    mediated device types. These mediated device types are specified
    via the 'supported_type_groups' structure when a device is registered
    with the mediated device framework. The registration process creates the
    sysfs structures for each mediated device type specified in the
    'mdev_supported_types' sub-directory of the device being registered. Along
    with the device type, the sysfs attributes of the mediated device type are
    provided.

  supported_type_groups
    VFIO 中介设备框架支持创建用户定义的中介设备类型。这些中介设备类型在设备向中
    设备框架注册时通过 'supported_type_groups' 结构指定。注册过程会为被注册设备
    'mdev_supported_types' 子目录中指定的每种中介设备类型创sysfs 结构。连同设
    类型，还会提供该中介设备类型sysfs 属性

    The VFIO AP device driver will register one mediated device type for
    passthrough devices:

    VFIO AP 设备驱动将为直通设备注册一种中介设备类型：

      /sys/devices/vfio_ap/matrix/mdev_supported_types/vfio_ap-passthrough

    Only the read-only attributes required by the VFIO mdev framework will
```

	... name
	... device_api
	... available_instances
	... device_api

    Where:

    其中

	* name:
	    specifies the name of the mediated device type
	* device_api:
	    the mediated device type's API
	* available_instances:
	    the number of vfio_ap mediated passthrough devices
	    that can be created
	* device_api:
	    specifies the VFIO API
  mdev_attr_groups
    This attribute group identifies the user-defined sysfs attributes of the
    mediated device. When a device is registered with the VFIO mediated device
    framework, the sysfs attribute files identified in the 'mdev_attr_groups'
    structure will be created in the vfio_ap mediated device's directory. The
    sysfs attributes for a vfio_ap mediated device are:

	* name:
	    指定中介设备类型的名
	* device_api:
	    中介设备类型API
	* available_instances:
	    可创建的 vfio_ap 中介直通设备数
	* device_api:
	    指定 VFIO API
  mdev_attr_groups
    该属性组标识中介设备的用户定sysfs 属性。当设备VFIO 中介设备框架注册时，
    'mdev_attr_groups' 结构中标识的 sysfs 属性文件将创建vfio_ap 中介设备
    目录中。vfio_ap 中介设备sysfs 属性为

    assign_adapter / unassign_adapter:
      Write-only attributes for assigning/unassigning an AP adapter to/from the
      vfio_ap mediated device. To assign/unassign an adapter, the APID of the
      adapter is echoed into the respective attribute file.
    assign_domain / unassign_domain:
      Write-only attributes for assigning/unassigning an AP usage domain to/from
      the vfio_ap mediated device. To assign/unassign a domain, the domain
      number of the usage domain is echoed into the respective attribute
      file.
    matrix:
      A read-only file for displaying the APQNs derived from the Cartesian
      product of the adapter and domain numbers assigned to the vfio_ap mediated
      device.
    guest_matrix:
      A read-only file for displaying the APQNs derived from the Cartesian
      product of the adapter and domain numbers assigned to the APM and AQM
      fields respectively of the KVM guest's CRYCB. This may differ from the
      the APQNs assigned to the vfio_ap mediated device if any APQN does not
      reference a queue device bound to the vfio_ap device driver (i.e., the
      queue is not in the host's AP configuration).
    assign_control_domain / unassign_control_domain:
      Write-only attributes for assigning/unassigning an AP control domain
      to/from the vfio_ap mediated device. To assign/unassign a control domain,
      the ID of the domain to be assigned/unassigned is echoed into the
      respective attribute file.
    control_domains:
      A read-only file for displaying the control domain numbers assigned to the
      vfio_ap mediated device.
    ap_config:
      A read/write file that, when written to, allows all three of the
      vfio_ap mediated device's ap matrix masks to be replaced in one shot.
      Three masks are given, one for adapters, one for domains, and one for
      control domains. If the given state cannot be set then no changes are
      made to the vfio-ap mediated device.

    assign_adapter / unassign_adapter锛?
      用于AP 适配器分解除分配vfio_ap 中介设备的只写属性。要分配/解除分配
      一个适配器，请将该适配器的 APID 回显（echo）到相应的属性文件
    assign_domain / unassign_domain锛?
      用于AP 使用域分解除分配vfio_ap 中介设备的只写属性。要分配/解除分配
      一个域，请将该使用域的域名回显到相应的属性文件
    matrix锛?
      一个只读文件，用于显示从分配给 vfio_ap 中介设备的适配器和域名笛卡尔积推导出的
      APQN銆?
    guest_matrix锛?
      一个只读文件，用于显示从分别分配给 KVM 客户CRYCB APM AQM 字段的适配
      和域名笛卡尔积推导出APQN。如果任APQN 未引用绑定到 vfio_ap 设备驱动（即
      队列不在主机AP 配置中）的队列设备，则它可能与分配给 vfio_ap 中介设备
      APQN 不同
    assign_control_domain / unassign_control_domain锛?
      用于AP 控制域分解除分配vfio_ap 中介设备的只写属性。要分配/解除分配
      一个控制域，请将要分配/解除分配的域 ID 回显到相应的属性文件
    control_domains锛?
      一个只读文件，用于显示分配vfio_ap 中介设备的控制域名
    ap_config锛?
      一个读/写文件，写入时允许一次性替vfio_ap 中介设备的全部三AP 矩阵掩码
      提供三个掩码，分别用于适配器、域和控制域。如果给定状态无法设置，则不会对
      vfio-ap 中介设备做任何更改

      The format of the data written to ap_config is as follows:
      {amask},{dmask},{cmask}\n

      \n is a newline character.

      amask, dmask, and cmask are masks identifying which adapters, domains,
      and control domains should be assigned to the mediated device.

      The format of a mask is as follows:
      0xNN..NN

      Where NN..NN is 64 hexadecimal characters representing a 256-bit value.
      The leftmost (highest order) bit represents adapter/domain 0.

      For an example set of masks that represent your mdev's current
      configuration, simply cat ap_config.

      Setting an adapter or domain number greater than the maximum allowed for
      the system will result in an error.

      This attribute is intended to be used by automation. End users would be
      better served using the respective assign/unassign attributes for
      adapters, domains, and control domains.

      写入 ap_config 的数据格式如下：
      {amask},{dmask},{cmask}\n

      \n 是一个换行符

      amask、dmask cmask 是掩码，标识应将哪些适配器、域和控制域分配给该中介设备

      掩码的格式如下：
      0xNN..NN

      其中 NN..NN 64 个十六进制字符，表示一256 位值。最左边（最高位）的
      表示适配0

      要获取表示你mdev 当前配置的一组掩码示例，只需 cat ap_config

      设置一个大于系统所允许最大值的适配器或域名将导致错误

      该属性旨在供自动化使用。最终用户最好使用各自的 assign/unassign 属性来操作
      适配器、域和控制域

```

- functions:

- 函数

  create:
    allocates the ap_matrix_mdev structure used by the vfio_ap driver to:

    - Store the reference to the KVM structure for the guest using the mdev
    - Store the AP matrix configuration for the adapters, domains, and control
      domains assigned via the corresponding sysfs attributes files
    - Store the AP matrix configuration for the adapters, domains and control
      domains available to a guest. A guest may not be provided access to APQNs
      referencing queue devices that do not exist, or are not bound to the
      vfio_ap device driver.

  create锛。
    分配 vfio_ap 驱动用于以下用途的 ap_matrix_mdev 结构

    - 存储使用 mdev 的客户机KVM 结构引用
    - 存储通过相应 sysfs 属性文件分配的适配器、域和控制域AP 矩阵配置
    - 存储客户机可用的适配器、域和控制域AP 矩阵配置。不得向客户机提供对引用不存
      或未绑定vfio_ap 设备驱动的队列设备的 APQN 的访问

  remove:
    deallocates the vfio_ap mediated device's ap_matrix_mdev structure.
    This will be allowed only if a running guest is not using the mdev.

  remove锛。
    释放 vfio_ap 中介设备ap_matrix_mdev 结构
    仅当没有正在运行的客户机使用mdev 时才允许

- callback interfaces

- 回调接口

  open_device:
    the open_device callback is invoked by userspace to connect the
    VFIO iommu group for the matrix mdev device to the MDEV bus.  The
    callback retrieves the KVM structure used to configure the KVM guest
    and configures the guest's access to the AP matrix defined via the
    vfio_ap mediated device's sysfs attribute files.

  open_device锛。
    open_device 回调由用户空间调用，以将矩阵 mdev 设备VFIO iommu 组连接到 MDEV
    总线。该回调检索用于配KVM 客户机的 KVM 结构，并配置客户机对通过 vfio_ap 中介
    设备 sysfs 属性文件定义的 AP 矩阵的访问

  close_device:
    this callback deconfigures the guest's AP matrix.

  close_device锛。
    该回调取消配置客户机AP 矩阵

  ioctl:
    this callback handles the VFIO_DEVICE_GET_INFO and VFIO_DEVICE_RESET ioctls
    defined by the vfio framework.

  ioctl锛。
    该回调处vfio 框架定义VFIO_DEVICE_GET_INFO VFIO_DEVICE_RESET ioctls

### Configure the guest's AP resources

### 配置客户机的 AP 资源

Configuring the AP resources for a KVM guest will be performed at the
time of `open_device` and `close_device`. The guest's AP resources are
configured via its APCB by:

KVM 客户机配AP 资源将在 `open_device` `close_device` 时进行。客户机
AP 资源通过APCB 配置如下

- Setting the bits in the APM corresponding to the APIDs assigned to the
  vfio_ap mediated device via its 'assign_adapter' interface.
- Setting the bits in the AQM corresponding to the domains assigned to the
  vfio_ap mediated device via its 'assign_domain' interface.
- Setting the bits in the ADM corresponding to the domain dIDs assigned to the
  vfio_ap mediated device via its 'assign_control_domains' interface.

- 设置 APM 中对应于通过 'assign_adapter' 接口分配vfio_ap 中介设备APID 的位
- 设置 AQM 中对应于通过 'assign_domain' 接口分配vfio_ap 中介设备的域的位
- 设置 ADM 中对应于通过 'assign_control_domains' 接口分配vfio_ap 中介设备
  dID 的位

The linux device model precludes passing a device through to a KVM guest that
is not bound to the device driver facilitating its pass-through. Consequently,
an APQN that does not reference a queue device bound to the vfio_ap device
driver will not be assigned to a KVM guest's matrix. The AP architecture,
however, does not provide a means to filter individual APQNs from the guest's
matrix, so the adapters, domains and control domains assigned to vfio_ap
mediated device via its sysfs 'assign_adapter', 'assign_domain' and
'assign_control_domain' interfaces will be filtered before providing the AP
configuration to a guest:

Linux 设备模型不允许将一个未绑定到促成其直通的驱动的设备直通给 KVM 客户机。因此，
不引用绑定到 vfio_ap 设备驱动的队列设备的 APQN 不会被分配给 KVM 客户机的矩阵。然而，
AP 架构没有提供从客户机矩阵中过滤单APQN 的手段，因此在向客户机提AP 配置之前
会通过sysfs 'assign_adapter'assign_domain' 'assign_control_domain' 接口分配
vfio_ap 中介设备的适配器、域和控制域将被过滤

- The APIDs of the adapters, the APQIs of the domains and the domain numbers of
  the control domains assigned to the matrix mdev that are not also assigned to
  the host's AP configuration will be filtered.

- 分配给矩mdev 的适配APID、域 APQI 和控制域名中，那些未同时分配给主AP 配置
  部分将被过滤

- Each APQN derived from the Cartesian product of the APIDs and APQIs assigned
  to the vfio_ap mdev is examined and if any one of them does not reference a
  queue device bound to the vfio_ap device driver, the adapter will not be
  plugged into the guest (i.e., the bit corresponding to its APID will not be
  set in the APM of the guest's APCB).

- 检查从分配vfio_ap mdev APID APQI 的笛卡尔积推导出的每APQN，如果其
  任意一个不引用绑定vfio_ap 设备驱动的队列设备，则该适配器将不会被插入客户机
  （即，其 APID 对应的位不会被设置在客户APCB APM 中）

### The CPU model features for AP

### 用于 AP CPU 模型特

The AP stack relies on the presence of the AP instructions as well as three
facilities: The AP Facilities Test (APFT) facility; the AP Query
Configuration Information (QCI) facility; and the AP Queue Interruption Control
facility. These features/facilities are made available to a KVM guest via the
following CPU model features:

AP 协议栈依AP 指令的存在以及三个设施：AP Facilities Test（APFT）设施；AP Query
Configuration Information（QCI）设施；以及 AP Queue Interruption Control 设施。这
特设施通过以下 CPU 模型特性提供给 KVM 客户机：

1. ap: Indicates whether the AP instructions are installed on the guest. This
   feature will be enabled by KVM only if the AP instructions are installed
   on the host.

1. ap：指示客户机上是否安装了 AP 指令。仅当主机上安装AP 指令时，KVM 才会启用
   特性

2. apft: Indicates the APFT facility is available on the guest. This facility
   can be made available to the guest only if it is available on the host (i.e.,
   facility bit 15 is set).

2. apft：指APFT 设施在客户机上可用。仅当主机上可用该设施时（即设施15 被置位）
   才能将其提供给客户机

3. apqci: Indicates the AP QCI facility is available on the guest. This facility
   can be made available to the guest only if it is available on the host (i.e.,
   facility bit 12 is set).

3. apqci：指AP QCI 设施在客户机上可用。仅当主机上可用该设施时（即设施12 被置位）
   才能将其提供给客户机

4. apqi: Indicates AP Queue Interruption Control faclity is available on the
   guest. This facility can be made available to the guest only if it is
   available on the host (i.e., facility bit 65 is set).

4. apqi：指AP Queue Interruption Control 设施在客户机上可用。仅当主机上可用该设施时
   （即设施65 被置位），才能将其提供给客户机

Note: If the user chooses to specify a CPU model different than the 'host'
model to QEMU, the CPU model features and facilities need to be turned on
```

     /usr/bin/qemu-system-s390x ... -cpu z13,ap=on,apqci=on,apft=on,apqi=on

```

A guest can be precluded from using AP features/facilities by turning them off
```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=off,apqci=off,apft=off,apqi=off

```

Note: If the APFT facility is turned off (apft=off) for the guest, the guest
will not see any AP devices. The zcrypt device drivers on the guest that
register for type 10 and newer AP devices - i.e., the cex4card and cex4queue
device drivers - need the APFT facility to ascertain the facilities installed on
a given AP device. If the APFT facility is not installed on the guest, then no
adapter or domain devices will get created by the AP bus running on the
guest because only type 10 and newer devices can be configured for guest use.

注意：如果为客户机关闭了 APFT 设施（apft=off），客户机将看不到任AP 设备。客户机
上注册用于类10 及更AP 设备（即 cex4card cex4queue 设备驱动）的 zcrypt 设备
驱动需APFT 设施来确定给AP 设备上安装的设施。如果客户机上未安装 APFT 设施，那
在客户机上运行的 AP 总线将不会创建任何适配器或域设备，因为只能为类10 及更新的设备
配置供客户机使用

## Example

## 示例

Let's now provide an example to illustrate how KVM guests may be given
access to AP facilities. For this example, we will show how to configure
three guests such that executing the lszcrypt command on the guests would
look like this:

现在让我们提供一个示例，说明如何授予 KVM 客户机对 AP 设施的访问权限。在本示例中，我
将展示如何配置三个客户机，使得在客户机上执行 lszcrypt 命令时显示如下内容：

### Guest1

### 瀹㈡埛鏈。

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
05          CEX5C CCA-Coproc
05.0004     CEX5C CCA-Coproc
05.00ab     CEX5C CCA-Coproc
06          CEX5A Accelerator
06.0004     CEX5A Accelerator
06.00ab     CEX5A Accelerator
=========== ===== ============

### Guest2

### 瀹㈡埛鏈。

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
05          CEX5C CCA-Coproc
05.0047     CEX5C CCA-Coproc
05.00ff     CEX5C CCA-Coproc
=========== ===== ============

### Guest3

### 瀹㈡埛鏈。

=========== ===== ============
CARD.DOMAIN TYPE  MODE
=========== ===== ============
06          CEX5A Accelerator
06.0047     CEX5A Accelerator
06.00ff     CEX5A Accelerator
=========== ===== ============

These are the steps:

步骤如下

1. Install the vfio_ap module on the linux host. The dependency chain for the
   vfio_ap module is:
   - iommu
   - s390
   - zcrypt
   - vfio
   - vfio_mdev
   - vfio_mdev_device
   - KVM

   To build the vfio_ap module, the kernel build must be configured with the
   following Kconfig elements selected:
   - IOMMU_SUPPORT
   - S390
   - AP
   - VFIO
   - KVM

1. Linux 主机上安vfio_ap 模块。vfio_ap 模块的依赖链为：
   - iommu
   - s390
   - zcrypt
   - vfio
   - vfio_mdev
   - vfio_mdev_device
   - KVM

   要构vfio_ap 模块，内核构建必须配置为选中以下 Kconfig 选项
   - IOMMU_SUPPORT
   - S390
   - AP
   - VFIO
   - KVM

```

     -> Device Drivers
	-> IOMMU Hardware Support
	   select S390 AP IOMMU Support
	-> VFIO Non-Privileged userspace driver framework
	   -> Mediated device driver frramework
	      -> VFIO driver for Mediated devices
     -> I/O subsystem
	-> VFIO support for AP devices

```

2. Secure the AP queues to be used by the three guests so that the host can not
   access them. To secure them, there are two sysfs files that specify
   bitmasks marking a subset of the APQN range as usable only by the default AP
   queue device drivers. All remaining APQNs are available for use by
   any other device driver. The vfio_ap device driver is currently the only
   non-default device driver. The location of the sysfs files containing the
```

     /sys/bus/ap/apmask
     /sys/bus/ap/aqmask

   The 'apmask' is a 256-bit mask that identifies a set of AP adapter IDs
   (APID). Each bit in the mask, from left to right, corresponds to an APID from
   0-255. If a bit is set, the APID belongs to the subset of APQNs marked as
   available only to the default AP queue device drivers.

   The 'aqmask' is a 256-bit mask that identifies a set of AP queue indexes
   (APQI). Each bit in the mask, from left to right, corresponds to an APQI from
   0-255. If a bit is set, the APQI belongs to the subset of APQNs marked as
   available only to the default AP queue device drivers.

   The Cartesian product of the APIDs corresponding to the bits set in the
   apmask and the APQIs corresponding to the bits set in the aqmask comprise
   the subset of APQNs that can be used only by the host default device drivers.
   All other APQNs are available to the non-default device drivers such as the
   vfio_ap driver.

   Take, for example, the following masks::

      apmask:
      0x7d00000000000000000000000000000000000000000000000000000000000000

      aqmask:
      0x8000000000000000000000000000000000000000000000000000000000000000

   The masks indicate:

   * Adapters 1, 2, 3, 4, 5, and 7 are available for use by the host default
     device drivers.

   * Domain 0 is available for use by the host default device drivers

   * The subset of APQNs available for use only by the default host device
     drivers are:

     (1,0), (2,0), (3,0), (4.0), (5,0) and (7,0)

   * All other APQNs are available for use by the non-default device drivers.

   The APQN of each AP queue device assigned to the linux host is checked by the
   AP bus against the set of APQNs derived from the Cartesian product of APIDs
   and APQIs marked as available to the default AP queue device drivers. If a
   match is detected,  only the default AP queue device drivers will be probed;
   otherwise, the vfio_ap device driver will be probed.

   By default, the two masks are set to reserve all APQNs for use by the default
   AP queue device drivers. There are two ways the default masks can be changed:

   1. The sysfs mask files can be edited by echoing a string into the
      respective sysfs mask file in one of two formats:

      * An absolute hex string starting with 0x - like "0x12345678" - sets
	the mask. If the given string is shorter than the mask, it is padded
	with 0s on the right; for example, specifying a mask value of 0x41 is
	the same as specifying::

	   0x4100000000000000000000000000000000000000000000000000000000000000

	Keep in mind that the mask reads from left to right, so the mask
	above identifies device numbers 1 and 7 (01000001).

	If the string is longer than the mask, the operation is terminated with
	an error (EINVAL).

      * Individual bits in the mask can be switched on and off by specifying
	each bit number to be switched in a comma separated list. Each bit
	number string must be prepended with a ('+') or minus ('-') to indicate
	the corresponding bit is to be switched on ('+') or off ('-'). Some
	valid values are:

	   - "+0"    switches bit 0 on
	   - "-13"   switches bit 13 off
	   - "+0x41" switches bit 65 on
	   - "-0xff" switches bit 255 off

	The following example:

	      +0,-6,+0x47,-0xf0

	Switches bits 0 and 71 (0x47) on

	Switches bits 6 and 240 (0xf0) off

	Note that the bits not specified in the list remain as they were before
	the operation.

   2. The masks can also be changed at boot time via parameters on the kernel
      command line like this:

	 ap.apmask=0xffff ap.aqmask=0x40

	 This would create the following masks::

	    apmask:
	    0xffff000000000000000000000000000000000000000000000000000000000000

	    aqmask:
	    0x4000000000000000000000000000000000000000000000000000000000000000

	 Resulting in these two pools::

	    default drivers pool:    adapter 0-15, domain 1
	    alternate drivers pool:  adapter 16-255, domains 0, 2-255

   **Note:**
   Changing a mask such that one or more APQNs will be taken from a vfio_ap
   mediated device (see below) will fail with an error (EBUSY). A message
   is logged to the kernel ring buffer which can be viewed with the 'dmesg'
   command. The output identifies each APQN flagged as 'in use' and identifies
   the vfio_ap mediated device to which it is assigned; for example:

   Userspace may not re-assign queue 05.0054 already assigned to 62177883-f1bb-47f0-914d-32a22e3a8804
   Userspace may not re-assign queue 04.0054 already assigned to cef03c3c-903d-4ecc-9a83-40694cb8aee4

```

### Securing the APQNs for our example

### 为我们的示例预留 APQN

   To secure the AP queues 05.0004, 05.0047, 05.00ab, 05.00ff, 06.0004, 06.0047,
   06.00ab, and 06.00ff for use by the vfio_ap device driver, the corresponding
   APQNs can be removed from the default masks using either of the following
```

      echo -5,-6 > /sys/bus/ap/apmask

      echo -4,-0x47,-0xab,-0xff > /sys/bus/ap/aqmask

   Or the masks can be set as follows::

      echo 0xf9ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff \
      > apmask

      echo 0xf7fffffffffffffffeffffffffffffffffffffffffeffffffffffffffffffffe \
      > aqmask

   This will result in AP queues 05.0004, 05.0047, 05.00ab, 05.00ff, 06.0004,
   06.0047, 06.00ab, and 06.00ff getting bound to the vfio_ap device driver. The
   sysfs directory for the vfio_ap device driver will now contain symbolic links
   to the AP queue devices bound to it::

     /sys/bus/ap
     ... [drivers]
     ...... [vfio_ap]
     ......... [05.0004]
     ......... [05.0047]
     ......... [05.00ab]
     ......... [05.00ff]
     ......... [06.0004]
     ......... [06.0047]
     ......... [06.00ab]
     ......... [06.00ff]

   Keep in mind that only type 10 and newer adapters (i.e., CEX4 and later)
   can be bound to the vfio_ap device driver. The reason for this is to
   simplify the implementation by not needlessly complicating the design by
   supporting older devices that will go out of service in the relatively near
   future and for which there are few older systems on which to test.

   The administrator, therefore, must take care to secure only AP queues that
   can be bound to the vfio_ap device driver. The device type for a given AP
   queue device can be read from the parent card's sysfs directory. For example,
   to see the hardware type of the queue 05.0004:

     cat /sys/bus/ap/devices/card05/hwtype

   The hwtype must be 10 or higher (CEX4 or newer) in order to be bound to the
   vfio_ap device driver.

```

3. Create the mediated devices needed to configure the AP matrixes for the
   three guests and to provide an interface to the vfio_ap driver for
```

     /sys/devices/vfio_ap/matrix/
     --- [mdev_supported_types]
     ------ [vfio_ap-passthrough] (passthrough vfio_ap mediated device type)
     --------- create
     --------- [devices]

   To create the mediated devices for the three guests::

	uuidgen > create
	uuidgen > create
	uuidgen > create

	or

	echo $uuid1 > create
	echo $uuid2 > create
	echo $uuid3 > create

   This will create three mediated devices in the [devices] subdirectory named
   after the UUID written to the create attribute file. We call them $uuid1,
   $uuid2 and $uuid3 and this is the sysfs directory structure after creation::

     /sys/devices/vfio_ap/matrix/
     --- [mdev_supported_types]
     ------ [vfio_ap-passthrough]
     --------- [devices]
     ------------ [$uuid1]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     --------------- unassign_control_domain
     --------------- unassign_domain

     ------------ [$uuid2]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     ----------------unassign_control_domain
     ----------------unassign_domain

     ------------ [$uuid3]
     --------------- assign_adapter
     --------------- assign_control_domain
     --------------- assign_domain
     --------------- matrix
     --------------- unassign_adapter
     ----------------unassign_control_domain
     ----------------unassign_domain

   Note *****: The vfio_ap mdevs do not persist across reboots unless the
               mdevctl tool is used to create and persist them.

```

4. The administrator now needs to configure the matrixes for the mediated
   devices $uuid1 (for Guest1), $uuid2 (for Guest2) and $uuid3 (for Guest3).

4. 管理员现在需要为中介设备 $uuid1（用于客户机1）uuid2（用于客户机2）和 $uuid3
   （用于客户机3）配置矩阵

```

      echo 5 > assign_adapter
      echo 6 > assign_adapter
      echo 4 > assign_domain
      echo 0xab > assign_domain

   Control domains can similarly be assigned using the assign_control_domain
   sysfs file.

   If a mistake is made configuring an adapter, domain or control domain,
   you can use the unassign_xxx files to unassign the adapter, domain or
   control domain.

   To display the matrix configuration for Guest1::

	 cat matrix

   To display the matrix that is or will be assigned to Guest1::

	 cat guest_matrix

   This is how the matrix is configured for Guest2::

      echo 5 > assign_adapter
      echo 0x47 > assign_domain
      echo 0xff > assign_domain

   This is how the matrix is configured for Guest3::

      echo 6 > assign_adapter
      echo 0x47 > assign_domain
      echo 0xff > assign_domain

   In order to successfully assign an adapter:

   * The adapter number specified must represent a value from 0 up to the
     maximum adapter number configured for the system. If an adapter number
     higher than the maximum is specified, the operation will terminate with
     an error (ENODEV).

     Note: The maximum adapter number can be obtained via the sysfs
	   /sys/bus/ap/ap_max_adapter_id attribute file.

   * Each APQN derived from the Cartesian product of the APID of the adapter
     being assigned and the APQIs of the domains previously assigned:

     - Must only be available to the vfio_ap device driver as specified in the
       sysfs /sys/bus/ap/apmask and /sys/bus/ap/aqmask attribute files. If even
       one APQN is reserved for use by the host device driver, the operation
       will terminate with an error (EADDRNOTAVAIL).

     - Must NOT be assigned to another vfio_ap mediated device. If even one APQN
       is assigned to another vfio_ap mediated device, the operation will
       terminate with an error (EBUSY).

     - Must NOT be assigned while the sysfs /sys/bus/ap/apmask and
       /sys/bus/ap/aqmask attribute files are being edited or the operation may
       terminate with an error (EBUSY).

   In order to successfully assign a domain:

   * The domain number specified must represent a value from 0 up to the
     maximum domain number configured for the system. If a domain number
     higher than the maximum is specified, the operation will terminate with
     an error (ENODEV).

     Note: The maximum domain number can be obtained via the sysfs
	   /sys/bus/ap/ap_max_domain_id attribute file.

    * Each APQN derived from the Cartesian product of the APQI of the domain
      being assigned and the APIDs of the adapters previously assigned:

     - Must only be available to the vfio_ap device driver as specified in the
       sysfs /sys/bus/ap/apmask and /sys/bus/ap/aqmask attribute files. If even
       one APQN is reserved for use by the host device driver, the operation
       will terminate with an error (EADDRNOTAVAIL).

     - Must NOT be assigned to another vfio_ap mediated device. If even one APQN
       is assigned to another vfio_ap mediated device, the operation will
       terminate with an error (EBUSY).

     - Must NOT be assigned while the sysfs /sys/bus/ap/apmask and
       /sys/bus/ap/aqmask attribute files are being edited or the operation may
       terminate with an error (EBUSY).

   In order to successfully assign a control domain:

   * The domain number specified must represent a value from 0 up to the maximum
     domain number configured for the system. If a control domain number higher
     than the maximum is specified, the operation will terminate with an
     error (ENODEV).

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid1 ...

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid2 ...

```

```

     /usr/bin/qemu-system-s390x ... -cpu host,ap=on,apqci=on,apft=on,apqi=on \
	-device vfio-ap,sysfsdev=/sys/devices/vfio_ap/matrix/$uuid3 ...

```

When the guest is shut down, the vfio_ap mediated devices may be removed.

当客户机关机时，vfio_ap 中介设备可被移除

```

   /sys/devices/vfio_ap/matrix/
      --- [mdev_supported_types]
      ------ [vfio_ap-passthrough]
      --------- [devices]
      ------------ [$uuid1]
      --------------- remove

```

```

   echo 1 > remove

```

This will remove all of the matrix mdev device's sysfs structures including
the mdev device itself. To recreate and reconfigure the matrix mdev device,
all of the steps starting with step 3 will have to be performed again. Note
that the remove will fail if a guest using the vfio_ap mdev is still running.

这将移除矩阵 mdev 设备的所sysfs 结构，包mdev 设备本身。要重新创建并重新配
矩阵 mdev 设备，必须重新执行从步骤 3 开始的所有步骤。注意，如果正在使用 vfio_ap mdev
的客户机仍在运行，remove 将失败

It is not necessary to remove a vfio_ap mdev, but one may want to
remove it if no guest will use it during the remaining lifetime of the linux
host. If the vfio_ap mdev is removed, one may want to also reconfigure
the pool of adapters and queues reserved for use by the default drivers.

并非必须移除一vfio_ap mdev，但如果在其余的 Linux 主机生命周期内没有客户机会使
它，可能会想要移除它。如果移除了 vfio_ap mdev，可能还想要重新配置为默认驱动预留的
适配器和队列池

## Hot plug/unplug support:

## 热插拔支持：

An adapter, domain or control domain may be hot plugged into a running KVM
guest by assigning it to the vfio_ap mediated device being used by the guest if
the following conditions are met:

在满足以下条件时，可以通过将适配器、域或控制域分配给客户机正在使用vfio_ap 中介
设备，将其热插入到一个正在运行的 KVM 客户机：

- The adapter, domain or control domain must also be assigned to the host's
  AP configuration.

- 该适配器、域或控制域还必须已分配给主机的 AP 配置

- Each APQN derived from the Cartesian product comprised of the APID of the
  adapter being assigned and the APQIs of the domains assigned must reference a
  queue device bound to the vfio_ap device driver.

- 由被分配适配器的 APID 与已分配域的 APQI 组成的笛卡尔积推导出的每APQN 必须引用
  一个绑定到 vfio_ap 设备驱动的队列设备

- To hot plug a domain, each APQN derived from the Cartesian product
  comprised of the APQI of the domain being assigned and the APIDs of the
  adapters assigned must reference a queue device bound to the vfio_ap device
  driver.

- 要热插拔一个域，由被分配域APQI 与已分配适配器的 APID 组成的笛卡尔积推导出的每
  APQN 必须引用一个绑定到 vfio_ap 设备驱动的队列设备

An adapter, domain or control domain may be hot unplugged from a running KVM
guest by unassigning it from the vfio_ap mediated device being used by the
guest.

可以通过将适配器、域或控制域从客户机正在使用vfio_ap 中介设备解除分配，将其从正在
运行KVM 客户机热拔出

## Over-provisioning of AP queues for a KVM guest:

## KVM 客户机过度配AP 队列

Over-provisioning is defined herein as the assignment of adapters or domains to
a vfio_ap mediated device that do not reference AP devices in the host's AP
configuration. The idea here is that when the adapter or domain becomes
available, it will be automatically hot-plugged into the KVM guest using
the vfio_ap mediated device to which it is assigned as long as each new APQN
resulting from plugging it in references a queue device bound to the vfio_ap
device driver.

此处将过度配置（Over-provisioning）定义为将不引用主机 AP 配置AP 设备的适配器或
分配vfio_ap 中介设备。这里的想法是，当适配器或域变得可用时，只要插入它所产生的每
APQN 都引用一个绑定到 vfio_ap 设备驱动的队列设备，它就会被自动热插入到分配给它
vfio_ap 中介设备所在的 KVM 客户机

## Driver Features

## 驱动特

The vfio_ap driver exposes a sysfs file containing supported features.
This exists so third party tools (like Libvirt and mdevctl) can query the
availability of specific features.

vfio_ap 驱动暴露一个包含所支持特性的 sysfs 文件。它的存在是为了让第三方工具（如
Libvirt mdevctl）能够查询特定特性的可用性

The features list can be found here: /sys/bus/matrix/devices/matrix/features

特性列表可在此处找到：/sys/bus/matrix/devices/matrix/features

Entries are space delimited. Each entry consists of a combination of
alphanumeric and underscore characters.

各项以空格分隔。每一项由字母数字和下划线字符的组合构成

Example:
cat /sys/bus/matrix/devices/matrix/features
guest_matrix dyn ap_config

示例
cat /sys/bus/matrix/devices/matrix/features
guest_matrix dyn ap_config

the following features are advertised:

将通告以下特性：

---------------+---------------------------------------------------------------+
| Flag         | Description                                                   |
+==============+===============================================================+
| guest_matrix | guest_matrix attribute exists. It reports the matrix of       |
|              | adapters and domains that are or will be passed through to a  |
|              | guest when the mdev is attached to it.                        |
+--------------+---------------------------------------------------------------+
| dyn          | Indicates hot plug/unplug of AP adapters, domains and control |
|              | domains for a guest to which the mdev is attached.            |
+------------+-----------------------------------------------------------------+
| ap_config    | ap_config interface for one-shot modifications to mdev config |
+--------------+---------------------------------------------------------------+

---------------+---------------------------------------------------------------+
| 标志         | 描述                                                          |
+==============+===============================================================+
| guest_matrix | guest_matrix 属性存在。它报告mdev 附加到客户机时，正在|
|              | 将被直通给该客户机的适配器和域的矩阵                      |
+--------------+---------------------------------------------------------------+
| dyn          | 指示热插拔附加了 mdev 的客户机AP 适配器、域和控制域  |
+------------+-----------------------------------------------------------------+
| ap_config    | 用于一次性修mdev 配置ap_config 接口                 |
+--------------+---------------------------------------------------------------+

## Limitations

## 限制

Live guest migration is not supported for guests using AP devices without
intervention by a system administrator. Before a KVM guest can be migrated,
the vfio_ap mediated device must be removed. Unfortunately, it can not be
removed manually (i.e., echo 1 > /sys/devices/vfio_ap/matrix/$UUID/remove) while
the mdev is in use by a KVM guest. If the guest is being emulated by QEMU,
its mdev can be hot unplugged from the guest in one of two ways:

对于使用 AP 设备的客户机，不支持在系统管理员不干预的情况下进行实时客户机迁移（live
guest migration）。在 KVM 客户机能够被迁移之前，必须移vfio_ap 中介设备。遗憾的是，
mdev 正被 KVM 客户机使用时，无法手动移除它（即 echo 1 >
/sys/devices/vfio_ap/matrix/$UUID/remove）。如果客户机QEMU 模拟，则mdev 可以通过以下
两种方式之一从客户机热拔出：

1. If the KVM guest was started with libvirt, you can hot unplug the mdev via
   the following commands:

1. 如果 KVM 客户机是libvirt 启动的，可以通过以下命令热拔mdev

      virsh detach-device <guestname> <path-to-device-xml>

      For example, to hot unplug mdev 62177883-f1bb-47f0-914d-32a22e3a8804 from
      the guest named 'my-guest':

      例如，要从名'my-guest' 的客户机热拔mdev 62177883-f1bb-47f0-914d-32a22e3a8804

         virsh detach-device my-guest ~/config/my-guest-hostdev.xml

            The contents of my-guest-hostdev.xml:


            <hostdev mode='subsystem' type='mdev' managed='no' model='vfio-ap'>
              <source>
                <address uuid='62177883-f1bb-47f0-914d-32a22e3a8804'/>
              </source>
            </hostdev>


      virsh qemu-monitor-command <guest-name> --hmp "device-del <device-id>"

      For example, to hot unplug the vfio_ap mediated device identified on the
      qemu command line with 'id=hostdev0' from the guest named 'my-guest':


         virsh qemu-monitor-command my-guest --hmp "device_del hostdev0"

2. A vfio_ap mediated device can be hot unplugged by attaching the qemu monitor
   to the guest and using the following qemu monitor command:

2. 可以通过qemu monitor 连接到客户机并使用以qemu monitor 命令来热拔出 vfio_ap
   中介设备

      (QEMU) device-del id=<device-id>

      For example, to hot unplug the vfio_ap mediated device that was specified
      on the qemu command line with 'id=hostdev0' when the guest was started:


         (QEMU) device-del id=hostdev0

After live migration of the KVM guest completes, an AP configuration can be
restored to the KVM guest by hot plugging a vfio_ap mediated device on the target
system into the guest in one of two ways:

KVM 客户机实时迁移完成后，可以通过在目标系统上vfio_ap 中介设备热插入到客户
来恢复其 AP 配置，有两种方式

1. If the KVM guest was started with libvirt, you can hot plug a matrix mediated
   device into the guest via the following virsh commands:

1. 如果 KVM 客户机是libvirt 启动的，可以通过以下 virsh 命令将矩阵中介设备热插入
   客户机：

   virsh attach-device <guestname> <path-to-device-xml>

      For example, to hot plug mdev 62177883-f1bb-47f0-914d-32a22e3a8804 into
      the guest named 'my-guest':

      例如，要mdev 62177883-f1bb-47f0-914d-32a22e3a8804 热插入名'my-guest' 
      客户机：

         virsh attach-device my-guest ~/config/my-guest-hostdev.xml

            The contents of my-guest-hostdev.xml:


            <hostdev mode='subsystem' type='mdev' managed='no' model='vfio-ap'>
              <source>
                <address uuid='62177883-f1bb-47f0-914d-32a22e3a8804'/>
              </source>
            </hostdev>


   virsh qemu-monitor-command <guest-name> --hmp \
   "device_add vfio-ap,sysfsdev=<path-to-mdev>,id=<device-id>"

      For example, to hot plug the vfio_ap mediated device
      62177883-f1bb-47f0-914d-32a22e3a8804 into the guest named 'my-guest' with
      device-id hostdev0:

      virsh qemu-monitor-command my-guest --hmp \
      "device_add vfio-ap,\
      sysfsdev=/sys/devices/vfio_ap/matrix/62177883-f1bb-47f0-914d-32a22e3a8804,\
      id=hostdev0"

2. A vfio_ap mediated device can be hot plugged by attaching the qemu monitor
   to the guest and using the following qemu monitor command:

2. 可以通过qemu monitor 连接到客户机并使用以qemu monitor 命令来热插入 vfio_ap
   中介设备

      (qemu) device_add "vfio-ap,sysfsdev=<path-to-mdev>,id=<device-id>"

      For example, to plug the vfio_ap mediated device
      62177883-f1bb-47f0-914d-32a22e3a8804 into the guest with the device-id
      hostdev0:


         (QEMU) device-add "vfio-ap,\
         sysfsdev=/sys/devices/vfio_ap/matrix/62177883-f1bb-47f0-914d-32a22e3a8804,\
         id=hostdev0"