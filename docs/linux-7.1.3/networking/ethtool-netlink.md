## ethtool Netlink 接口


## 基本信息


ethtool netlink 接口使用名为 `ethtool` generic netlink 系列
（用户态应用程序应使用`<linux/ethtool_netlink.h>` uapi 头文件中定义的宏
`ETHTOOL_GENL_NAME` `ETHTOOL_GENL_VERSION`）。该系列不使用特定的头部，请求与
回复中的所有信息均通过 netlink 属性传递

ethtool netlink 接口使用扩展 ACK（extended ACK）来上报错误与警告，建议用户态应
程序开发者以合适的方式将这些消息呈现给用户

请求可分为三类：“get”（获取信息）、“set”（设置参数）与“action”（执行某个动作）

所有“set”与“action”类型的请求都需要管理员权限（命名空间内
`CAP_NET_ADMIN`）。大多数“get”类型的请求允许任何人调用，但也有例外（当回复中包含
敏感信息时）。在某些情况下，请求本身对任何人都是允许的，但非特权用户会被省略
包含敏感信息的属性（例如唤醒局域网密码）


## 约定


表示布尔值的属性通常沿用 NLA_U8 类型，以便区分三种状态：“on”（开）、“off”（关）
与“not present”（不存在，即“get”请求中信息不可用，或“set”请求中无需改变该值）
对于这些属性，“true”（真）值应以数1 传递，但接收方应将任何非零值都理解为“true”
在下方的表中，“bool”表示以此方式解释的 NLA_U8 属性

在下面的消息结构描述中，若某属性名带有”后缀，则表示其父嵌套中可以包含多个相
类型的属性。这实现了一个条目数组

需要由设备驱动填充、并依据其是否有效来dump到用户空间的属性，不应使用零作为有效值
这样可以避免在设备驱API 中显式标识该属性的有效性


## 请求


每个请求或回复消息都包含一个带有公共头部的嵌套属性。该头部的结构如下：

  ==============================  ======  =============================
  `ETHTOOL_A_HEADER_DEV_INDEX`  u32     device ifindex
  `ETHTOOL_A_HEADER_DEV_NAME`   string  device name
  `ETHTOOL_A_HEADER_FLAGS`      u32     flags common for all requests
  `ETHTOOL_A_HEADER_PHY_INDEX`  u32     phy device index
  ==============================  ======  =============================

`ETHTOOL_A_HEADER_DEV_INDEX` `ETHTOOL_A_HEADER_DEV_NAME` 用于标识消息所关联
设备。在请求中提供其中一个即可；若两者都提供，则它们必须指向同一设备。某些请
（例如全局字符串集）不需要设备标识。大多数 `GET` 请求也允许不带设备标识的 dump
请求，以查询提供该信息的全部设备（每个设备单独一条消息）

`ETHTOOL_A_HEADER_FLAGS` 是一个对所有请求类型通用的请求标志位图。这些标志的解释
对所有请求类型都相同，但某些标志可能不适用于特定请求。已识别的标志如下：

  =================================  ===================================
  `ETHTOOL_FLAG_COMPACT_BITSETS`   回复中使用紧凑格式位
  `ETHTOOL_FLAG_OMIT_REPLY`        省略可选回复（_SET _ACT
  `ETHTOOL_FLAG_STATS`             包含可选的设备统计信息
  =================================  ===================================

新的请求标志应遵循一个总体原则：若未设置该标志，则行为保持向后兼容，即来自不了
该标志的旧客户端发出的请求，应按客户端期望的方式解释。客户端不得设置它所不了解的
标志

`ETHTOOL_A_HEADER_PHY_INDEX` 标识消息所关联的以太网 PHY。由于有大量命令PHY 配置
相关，且链路上可能存在多PHY，对于需要它的命令，可以在请求中传入 PHY 索引。但
这并非强制要求；如果针对 PHY 的命令未传入该值，则会使用 net_device.phydev 指针

## 位集


对于长度（相对）固定的短位图，使用标准的 `NLA_BITFIELD32` 类型。对于任意长度的位图
ethtool netlink 使用一种嵌套属性，其内容采用两种形式之一：紧凑形式（两个二进制位图，
分别表示位值与受影响位的掩码）与逐位形式（由索引或名称标识的位列表）

详细（逐位）位集允许将位的符号名与其值一同发送，从而省去一次往返（当位集在请求中传递时
或至少省去一次额外请求（当位集在回复中时）。这对于传统 ethtool 命令这类一次性应用很
用处。另一方面，像 ethtool monitor（显示通知）或网络管理守护进程这类长期运行的应用，
可能更倾向于仅获取一次名称，并使用紧凑形式以减小消息体积。ethtool netlink 接口发出
通知始终对位集使用紧凑形式

一个位集既可以表示一个掩码对（`ETHTOOL_A_BITSET_NOMASK` 未设置），也可以表示单个
位图（`ETHTOOL_A_BITSET_NOMASK` 已设置）。在修改位图的请求中，前者将掩码中位对应的位
修改为值中设置的位，其余保持不变；后者则将位图中设置的位设为 1，其余清 0

紧凑形式：嵌套（位集）属性的内容

  ============================  ======  ============================
  `ETHTOOL_A_BITSET_NOMASK`   flag    no mask, only a list
  `ETHTOOL_A_BITSET_SIZE`     u32     number of significant bits
  `ETHTOOL_A_BITSET_VALUE`    binary  bitmap of bit values
  `ETHTOOL_A_BITSET_MASK`     binary  bitmap of valid bits
  ============================  ======  ============================

值和掩码的长度必须至少为 `ETHTOOL_A_BITSET_SIZE` 个位，并向上取整32 位的整数倍
它们由以主机字节序存放的 32 位字组成，字从最低有效到最高有效排序（即与 ioctl 接口
传递位图的方式相同）

对于紧凑形式，`ETHTOOL_A_BITSET_SIZE` `ETHTOOL_A_BITSET_VALUE` 是必填的
`ETHTOOL_A_BITSET_NOMASK` 未设置时（位集表示一个掩码对），`ETHTOOL_A_BITSET_MASK`
属性为必填；若 `ETHTOOL_A_BITSET_NOMASK` 未设置，`ETHTOOL_A_BITSET_MASK` 则不允许出现
（位集表示一个单独位图）

如果较旧的应用程序运行在较新的内核上，或反之，内核的位集长度可能与用户空间的长度不同
若用户空间的位图更长，仅当请求实际试图设置某些内核无法识别的位的值时，才会返回错误

逐位形式：嵌套（位集）属性的内容

 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_NOMASK`        | flag   | no mask, only a list        |
 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_SIZE`          | u32    | number of significant bits  |
 +------------------------------------+--------+-----------------------------+
 | `ETHTOOL_A_BITSET_BITS`          | nested | array of bits               |
 +-+----------------------------------+--------+-----------------------------+
 | | `ETHTOOL_A_BITSET_BITS_BIT+`   | nested | one bit                     |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_INDEX` | u32    | bit index (0 for LSB)       |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_NAME`  | string | bit name                    |
 +-+-+--------------------------------+--------+-----------------------------+
 | | | `ETHTOOL_A_BITSET_BIT_VALUE` | flag   | present if bit is set       |
 +-+-+--------------------------------+--------+-----------------------------+

对于逐位形式，`ETHTOOL_A_BITSET_SIZE` 是可选的，`ETHTOOL_A_BITSET_BITS` 为必填
`ETHTOOL_A_BITSET_BITS` 嵌套中只能包`ETHTOOL_A_BITSET_BITS_BIT` 属性，但其数量
可以任意。一个位可以通过其索引或名称来标识。在请求中使用时，所列出的位会根
`ETHTOOL_A_BITSET_BIT_VALUE` 被设0 1，其余保持不变

如果索引超出了内核的位长度，或者名称无法识别，请求将失败。若名称和索引同时设置，
它们指向不同的位，请求也会失败

`ETHTOOL_A_BITSET_NOMASK` 标志存在时，位集被解释为一个简单位图。这种情况下不使
`ETHTOOL_A_BITSET_BIT_VALUE` 属性。此类位集表示一个位图，其中所列出的位被置位，其余
涓?0銆。

在请求中，应用程序可以使用任意一种形式。内核在回复中使用的形式由请求头 flags 字段中的
`ETHTOOL_FLAG_COMPACT_BITSETS` 标志决定。值与掩码的语义取决于具体属性


## 消息类型列表


所有标识消息类型的常量都使`ETHTOOL_CMD_` 前缀，并根据消息用途使用相应的后缀

  ==============    ======================================
  `_GET`          用户空间用于获取数据的请
  `_SET`          用户空间用于设置数据的请
  `_ACT`          用户空间用于执行某个动作的请
  `_GET_REPLY`    内核`GET` 请求的回
  `_SET_REPLY`    内核`SET` 请求的回
  `_ACT_REPLY`    内核`ACT` 请求的回
  `_NTF`          内核通知
  ==============    ======================================

用户空间到内核：

  ===================================== =================================
  `ETHTOOL_MSG_STRSET_GET`            获取字符串集
  `ETHTOOL_MSG_LINKINFO_GET`          获取链路设置
  `ETHTOOL_MSG_LINKINFO_SET`          设置链路设置
  `ETHTOOL_MSG_LINKMODES_GET`         获取链路模式信息
  `ETHTOOL_MSG_LINKMODES_SET`         设置链路模式信息
  `ETHTOOL_MSG_LINKSTATE_GET`         获取链路状
  `ETHTOOL_MSG_DEBUG_GET`             获取调试设置
  `ETHTOOL_MSG_DEBUG_SET`             设置调试设置
  `ETHTOOL_MSG_WOL_GET`               获取唤醒局域网设置
  `ETHTOOL_MSG_WOL_SET`               设置唤醒局域网设置
  `ETHTOOL_MSG_FEATURES_GET`          获取设备特
  `ETHTOOL_MSG_FEATURES_SET`          设置设备特
  `ETHTOOL_MSG_PRIVFLAGS_GET`         获取私有标志
  `ETHTOOL_MSG_PRIVFLAGS_SET`         设置私有标志
  `ETHTOOL_MSG_RINGS_GET`             获取环形队列大小
  `ETHTOOL_MSG_RINGS_SET`             设置环形队列大小
  `ETHTOOL_MSG_CHANNELS_GET`          获取通道数量
  `ETHTOOL_MSG_CHANNELS_SET`          设置通道数量
  `ETHTOOL_MSG_COALESCE_GET`          获取中断聚合参数
  `ETHTOOL_MSG_COALESCE_SET`          设置中断聚合参数
  `ETHTOOL_MSG_PAUSE_GET`             获取暂停参数
  `ETHTOOL_MSG_PAUSE_SET`             设置暂停参数
  `ETHTOOL_MSG_EEE_GET`               获取 EEE 设置
  `ETHTOOL_MSG_EEE_SET`               设置 EEE 设置
  `ETHTOOL_MSG_TSINFO_GET`		获取时间戳信
  `ETHTOOL_MSG_CABLE_TEST_ACT`        动作：启动线缆测
  `ETHTOOL_MSG_CABLE_TEST_TDR_ACT`    动作：启动原TDR 线缆测试
  `ETHTOOL_MSG_TUNNEL_INFO_GET`       获取隧道卸载信息
  `ETHTOOL_MSG_FEC_GET`               获取 FEC 设置
  `ETHTOOL_MSG_FEC_SET`               设置 FEC 设置
  `ETHTOOL_MSG_MODULE_EEPROM_GET`     读取 SFP 模块 EEPROM
  `ETHTOOL_MSG_STATS_GET`             获取标准统计信息
  `ETHTOOL_MSG_PHC_VCLOCKS_GET`       获取 PHC 虚拟时钟信息
  `ETHTOOL_MSG_MODULE_SET`            设置收发器模块参
  `ETHTOOL_MSG_MODULE_GET`            获取收发器模块参
  `ETHTOOL_MSG_PSE_SET`               设置 PSE 参数
  `ETHTOOL_MSG_PSE_GET`               获取 PSE 参数
  `ETHTOOL_MSG_RSS_GET`               获取 RSS 设置
  `ETHTOOL_MSG_PLCA_GET_CFG`          获取 PLCA RS 参数
  `ETHTOOL_MSG_PLCA_SET_CFG`          设置 PLCA RS 参数
  `ETHTOOL_MSG_PLCA_GET_STATUS`       获取 PLCA RS 状
  `ETHTOOL_MSG_MM_GET`                获取 MAC 合并层状
  `ETHTOOL_MSG_MM_SET`                设置 MAC 合并层参
  `ETHTOOL_MSG_MODULE_FW_FLASH_ACT`   烧录收发器模块固
  `ETHTOOL_MSG_PHY_GET`               获取以太PHY 信息
  `ETHTOOL_MSG_TSCONFIG_GET`          获取硬件时间戳配
  `ETHTOOL_MSG_TSCONFIG_SET`          设置硬件时间戳配
  `ETHTOOL_MSG_RSS_SET`               设置 RSS 设置
  `ETHTOOL_MSG_RSS_CREATE_ACT`        创建额外RSS 上下
  `ETHTOOL_MSG_RSS_DELETE_ACT`        删除额外RSS 上下
  `ETHTOOL_MSG_MSE_GET`               获取 MSE 诊断数据
  ===================================== =================================

内核到用户空间：

  ======================================== =================================
  `ETHTOOL_MSG_STRSET_GET_REPLY`         字符串集内容
  `ETHTOOL_MSG_LINKINFO_GET_REPLY`       链路设置
  `ETHTOOL_MSG_LINKINFO_NTF`             链路设置通知
  `ETHTOOL_MSG_LINKMODES_GET_REPLY`      链路模式信息
  `ETHTOOL_MSG_LINKMODES_NTF`            链路模式通知
  `ETHTOOL_MSG_LINKSTATE_GET_REPLY`      链路状态信
  `ETHTOOL_MSG_DEBUG_GET_REPLY`          调试设置
  `ETHTOOL_MSG_DEBUG_NTF`                调试设置通知
  `ETHTOOL_MSG_WOL_GET_REPLY`            唤醒局域网设置
  `ETHTOOL_MSG_WOL_NTF`                  唤醒局域网设置通知
  `ETHTOOL_MSG_FEATURES_GET_REPLY`       设备特
  `ETHTOOL_MSG_FEATURES_SET_REPLY`       针对 FEATURES_SET 的可选回
  `ETHTOOL_MSG_FEATURES_NTF`             网络设备特性通知
  `ETHTOOL_MSG_PRIVFLAGS_GET_REPLY`      私有标志
  `ETHTOOL_MSG_PRIVFLAGS_NTF`            私有标志
  `ETHTOOL_MSG_RINGS_GET_REPLY`          环形队列大小
  `ETHTOOL_MSG_RINGS_NTF`                环形队列大小
  `ETHTOOL_MSG_CHANNELS_GET_REPLY`       通道数量
  `ETHTOOL_MSG_CHANNELS_NTF`             通道数量
  `ETHTOOL_MSG_COALESCE_GET_REPLY`       中断聚合参数
  `ETHTOOL_MSG_COALESCE_NTF`             中断聚合参数
  `ETHTOOL_MSG_PAUSE_GET_REPLY`          暂停参数
  `ETHTOOL_MSG_PAUSE_NTF`                暂停参数
  `ETHTOOL_MSG_EEE_GET_REPLY`            EEE 设置
  `ETHTOOL_MSG_EEE_NTF`                  EEE 设置
  `ETHTOOL_MSG_TSINFO_GET_REPLY`         时间戳信
  `ETHTOOL_MSG_CABLE_TEST_NTF`           线缆测试结果
  `ETHTOOL_MSG_CABLE_TEST_TDR_NTF`       线缆测试 TDR 结果
  `ETHTOOL_MSG_TUNNEL_INFO_GET_REPLY`    隧道卸载信息
  `ETHTOOL_MSG_FEC_GET_REPLY`            FEC 设置
  `ETHTOOL_MSG_FEC_NTF`                  FEC 设置
  `ETHTOOL_MSG_MODULE_EEPROM_GET_REPLY`  读取 SFP 模块 EEPROM
  `ETHTOOL_MSG_STATS_GET_REPLY`          标准统计信息
  `ETHTOOL_MSG_PHC_VCLOCKS_GET_REPLY`     PHC 虚拟时钟信息
  `ETHTOOL_MSG_MODULE_GET_REPLY`         收发器模块参
  `ETHTOOL_MSG_PSE_GET_REPLY`            PSE 参数
  `ETHTOOL_MSG_RSS_GET_REPLY`            RSS 设置
  `ETHTOOL_MSG_RSS_NTF`                  RSS 设置
  `ETHTOOL_MSG_PLCA_GET_CFG_REPLY`       PLCA RS 参数
  `ETHTOOL_MSG_PLCA_GET_STATUS_REPLY`    PLCA RS 状
  `ETHTOOL_MSG_PLCA_NTF`                 PLCA RS 参数
  `ETHTOOL_MSG_MM_GET_REPLY`             MAC 合并层状
  `ETHTOOL_MSG_MODULE_FW_FLASH_NTF`      收发器模块固件更
  `ETHTOOL_MSG_PHY_GET_REPLY`            以太PHY 信息
  `ETHTOOL_MSG_PHY_NTF`                  以太PHY 信息变更
  `ETHTOOL_MSG_TSCONFIG_GET_REPLY`       硬件时间戳配
  `ETHTOOL_MSG_TSCONFIG_SET_REPLY`       新的硬件时间戳配
  `ETHTOOL_MSG_PSE_NTF`                  PSE 事件通知
  `ETHTOOL_MSG_RSS_NTF`                  RSS 设置通知
  `ETHTOOL_MSG_RSS_CREATE_ACT_REPLY`     创建额外RSS 上下
  `ETHTOOL_MSG_RSS_CREATE_NTF`           已创建额外的 RSS 上下
  `ETHTOOL_MSG_RSS_DELETE_NTF`           已删除额外的 RSS 上下
  `ETHTOOL_MSG_MSE_GET_REPLY`            MSE 诊断数据
  ======================================== =================================

`GET` 请求由用户空间应用程序发出，用于获取设备信息。它们通常不包含任何消息特定的
属性。内核通过相应的“GET_REPLY”消息回复。对于大多数类型，不带设备标识、并设置
`NLM_F_DUMP` `GET` 请求可用于查询所有支持该请求的设备的对应信息

如果数据也可以被修改，则使用具有相同布局（与相应 `GET_REPLY` 一致）`SET` 消息
请求更改。此类请求中仅包含请求了更改的属性（当然，也并非所有属性都可被更改）。对大多
`SET` 请求的回复仅包含错误码与 extack；若内核提供额外数据，则会以相应 `SET_REPLY`
消息的形式发送，可通过在请求头中设`ETHTOOL_FLAG_OMIT_REPLY` 标志来抑制该回复

数据修改还会触发发送一条包含通知`NTF` 消息。这些消息通常只携带受该更改影响的属
子集。如果使用其他方式（主要ioctl ethtool 接口）修改了数据，也会发出相同的通知
与仅在数据实际发生变化时才发送的 ethtool netlink 代码通知不同，由 ioctl 接口触发
通知即使请求实际上没有改变任何数据也可能被发送

`ACT` 消息请求内核（驱动）执行某个特定动作。如果内核上报了某些信息（可通过在请求头
设置 `ETHTOOL_FLAG_OMIT_REPLY` 标志来抑制），则该回复以 `ACT_REPLY` 消息的形式呈现
执行动作还会触发一条通知（`NTF` 消息）

后续章节将描述这些消息的格式与语义


## STRSET_GET


请求 ioctl 命令 `ETHTOOL_GSSET_INFO` `ETHTOOL_GSTRINGS` 所提供的字符串集内容
字符串集不可由用户写入，因此相应`STRSET_SET` 消息仅在内核回复中使用。字符串集分
两类：全局的（与设备无关，例如设备特性名称）与设备特定的（例如设备私有标志）

请求内容

 +---------------------------------------+--------+------------------------+
 | `ETHTOOL_A_STRSET_HEADER`           | nested | request header         |
 +---------------------------------------+--------+------------------------+
 | `ETHTOOL_A_STRSET_STRINGSETS`       | nested | string set to request  |
 +-+-------------------------------------+--------+------------------------+
 | | `ETHTOOL_A_STRINGSETS_STRINGSET+` | nested | one string set         |
 +-+-+-----------------------------------+--------+------------------------+
 | | | `ETHTOOL_A_STRINGSET_ID`        | u32    | set id                 |
 +-+-+-----------------------------------+--------+------------------------+

内核响应内容

 +---------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_HEADER`           | nested | reply header          |
 +---------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_STRINGSETS`       | nested | array of string sets  |
 +-+-------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_STRINGSETS_STRINGSET+` | nested | one string set        |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_ID`        | u32    | set id                |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_COUNT`     | u32    | number of strings     |
 +-+-+-----------------------------------+--------+-----------------------+
 | | | `ETHTOOL_A_STRINGSET_STRINGS`   | nested | array of strings      |
 +-+-+-+---------------------------------+--------+-----------------------+
 | | | | `ETHTOOL_A_STRINGS_STRING+`   | nested | one string            |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | | | | | `ETHTOOL_A_STRING_INDEX`    | u32    | string index          |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | | | | | `ETHTOOL_A_STRING_VALUE`    | string | string value          |
 +-+-+-+-+-------------------------------+--------+-----------------------+
 | `ETHTOOL_A_STRSET_COUNTS_ONLY`      | flag   | return only counts    |
 +---------------------------------------+--------+-----------------------+

请求头中的设备标识是可选的。根据其是否存在以及 `NLM_F_DUMP` 标志，存在三种类型的
`STRSET_GET` 请求

 - `NLM_F_DUMP,` 无设备：获取“全局”字符串
 - `NLM_F_DUMP`，带设备：获取与该设备相关的字符串集
 - `NLM_F_DUMP`，无设备：获取所有设备的设备相关字符串集

如果没有 `ETHTOOL_A_STRSET_STRINGSETS` 数组，则返回所有请求类型的字符串集，否则仅
返回请求中指定的那些。`ETHTOOL_A_STRSET_COUNTS_ONLY` 标志告知内核只返回字符串集的
计数，而非实际的字符串


## LINKINFO_GET


请求链路设置，与 `ETHTOOL_GLINKSETTINGS` 提供的内容相同，但不包括链路模式与自协商
相关的信息。该请求不使用任何属性

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  reply header
  `ETHTOOL_A_LINKINFO_PORT`           u8      physical port
  `ETHTOOL_A_LINKINFO_PHYADDR`        u8      phy MDIO address
  `ETHTOOL_A_LINKINFO_TP_MDIX`        u8      MDI(-X) status
  `ETHTOOL_A_LINKINFO_TP_MDIX_CTRL`   u8      MDI(-X) control
  `ETHTOOL_A_LINKINFO_TRANSCEIVER`    u8      transceiver
  ====================================  ======  ==========================

各属性及其取值与相应 ioctl 结构体中对应的成员含义相同

`LINKINFO_GET` 允许 dump 请求（内核为所有支持该请求的设备返回回复消息）


## LINKINFO_SET


`LINKINFO_SET` 请求允许设置 `LINKINFO_GET` 所上报的部分属性

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKINFO_HEADER`         nested  request header
  `ETHTOOL_A_LINKINFO_PORT`           u8      physical port
  `ETHTOOL_A_LINKINFO_PHYADDR`        u8      phy MDIO address
  `ETHTOOL_A_LINKINFO_TP_MDIX_CTRL`   u8      MDI(-X) control
  ====================================  ======  ==========================

MDI(-X) 状态与收发器不可设置，携带相应属性的请求将被拒绝


## LINKMODES_GET


请求链路模式（支持的、通告的以及对端通告的）以及相关信息（自协商状态、链路速率与双工）
`ETHTOOL_GLINKSETTINGS` 提供的内容相同。该请求不使用任何属性

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`        nested  request header
  ====================================  ======  ==========================

内核响应内容

  ==========================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`              nested  reply header
  `ETHTOOL_A_LINKMODES_AUTONEG`             u8      autonegotiation status
  `ETHTOOL_A_LINKMODES_OURS`                bitset  advertised link modes
  `ETHTOOL_A_LINKMODES_PEER`                bitset  partner link modes
  `ETHTOOL_A_LINKMODES_SPEED`               u32     link speed (Mb/s)
  `ETHTOOL_A_LINKMODES_DUPLEX`              u8      duplex mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG`    u8      Master/slave port mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_STATE`  u8      Master/slave port state
  `ETHTOOL_A_LINKMODES_RATE_MATCHING`       u8      PHY rate matching
  ==========================================  ======  ==========================

对于 `ETHTOOL_A_LINKMODES_OURS`，值表示通告的模式，掩码表示支持的模式。回复中
`ETHTOOL_A_LINKMODES_PEER` 是一个位列表

`LINKMODES_GET` 允许 dump 请求（内核为所有支持该请求的设备返回回复消息）


## LINKMODES_SET


请求内容

  ==========================================  ======  ==========================
  `ETHTOOL_A_LINKMODES_HEADER`              nested  request header
  `ETHTOOL_A_LINKMODES_AUTONEG`             u8      autonegotiation status
  `ETHTOOL_A_LINKMODES_OURS`                bitset  advertised link modes
  `ETHTOOL_A_LINKMODES_PEER`                bitset  partner link modes
  `ETHTOOL_A_LINKMODES_SPEED`               u32     link speed (Mb/s)
  `ETHTOOL_A_LINKMODES_DUPLEX`              u8      duplex mode
  `ETHTOOL_A_LINKMODES_MASTER_SLAVE_CFG`    u8      Master/slave port mode
  `ETHTOOL_A_LINKMODES_RATE_MATCHING`       u8      PHY rate matching
  `ETHTOOL_A_LINKMODES_LANES`               u32     lanes
  ==========================================  ======  ==========================

`ETHTOOL_A_LINKMODES_OURS` 位集允许设置通告的链路模式。如果自协商处于开启状态（无论
本次设置还是沿用之前的设置），且通告模式未改变（`ETHTOOL_A_LINKMODES_OURS` 属性）
并且至少指定了速率、双工与通道（lanes）中的一项，内核会将通告模式调整为所有匹配所指定
的速率、双工、通道（或全部，视指定情况而定）的支持模式。这种自动选择是在使用 ioctl 接口
时由 ethtool 一侧完成的；netlink 接口则旨在允许在不知道内核具体支持什么的情况下请求更改


## LINKSTATE_GET


请求链路状态信息。提供了链路 up/down 标志（由 `ETHTOOL_GLINK` ioctl 命令提供）
可选地，也可能提供扩展状态。总体上，扩展状态描述了端口为何处于 down 状态，或为何以
某种非显而易见的方式运行。该请求没有任何属性

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_LINKSTATE_HEADER`        nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ============================
  `ETHTOOL_A_LINKSTATE_HEADER`        nested  reply header
  `ETHTOOL_A_LINKSTATE_LINK`          bool    link state (up/down)
  `ETHTOOL_A_LINKSTATE_SQI`           u32     Current Signal Quality Index
  `ETHTOOL_A_LINKSTATE_SQI_MAX`       u32     Max support SQI value
  `ETHTOOL_A_LINKSTATE_EXT_STATE`     u8      link extended state
  `ETHTOOL_A_LINKSTATE_EXT_SUBSTATE`  u8      link extended substate
  `ETHTOOL_A_LINKSTATE_EXT_DOWN_CNT`  u32     count of link down events
  ====================================  ======  ============================

对于大多NIC 驱动，`ETHTOOL_A_LINKSTATE_LINK` 的值返回由 `netif_carrier_ok()`
提供的载波标志，但也存在自行定义处理函数的驱动

`ETHTOOL_A_LINKSTATE_EXT_STATE` `ETHTOOL_A_LINKSTATE_EXT_SUBSTATE` 为可选值
ethtool 核心可以既提`ETHTOOL_A_LINKSTATE_EXT_STATE` 又提
`ETHTOOL_A_LINKSTATE_EXT_SUBSTATE`，或只提`ETHTOOL_A_LINKSTATE_EXT_STATE`，或
两者都不提供

`LINKSTATE_GET` 允许 dump 请求（内核为所有支持该请求的设备返回回复消息）


链路扩展状态：

  ================================================      ============================================
  `ETHTOOL_LINK_EXT_STATE_AUTONEG`                    与自协商或其中存在的问题相关的状

  `ETHTOOL_LINK_EXT_STATE_LINK_TRAINING_FAILURE`      链路训练期间失败

  `ETHTOOL_LINK_EXT_STATE_LINK_LOGICAL_MISMATCH`      物理编码子层或前向纠错子层中的逻辑不匹

  `ETHTOOL_LINK_EXT_STATE_BAD_SIGNAL_INTEGRITY`       信号完整性问

  `ETHTOOL_LINK_EXT_STATE_NO_CABLE`                   未连接线

  `ETHTOOL_LINK_EXT_STATE_CABLE_ISSUE`                故障与线缆相关，例如不支持的线缆

  `ETHTOOL_LINK_EXT_STATE_EEPROM_ISSUE`               故障EEPROM 相关，例如在读取或解析数据时失败

  `ETHTOOL_LINK_EXT_STATE_CALIBRATION_FAILURE`        校准算法期间失败

  `ETHTOOL_LINK_EXT_STATE_POWER_BUDGET_EXCEEDED`      硬件无法提供线缆或模块所需的功

  `ETHTOOL_LINK_EXT_STATE_OVERHEAT`                   模块过热

  `ETHTOOL_LINK_EXT_STATE_MODULE`                     收发器模块问
  ================================================      ============================================

链路扩展子状态：

  自协商子状态：

  ===============================================================   ================================
  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_PARTNER_DETECTED`              对端处于 down 状

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_ACK_NOT_RECEIVED`                 未收到对端的 Ack

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NEXT_PAGE_EXCHANGE_FAILED`        下一页交换失

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_PARTNER_DETECTED_FORCE_MODE`   在强制模式期间对端处down 状态，或速率未达成一

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_FEC_MISMATCH_DURING_OVERRIDE`     双方的前向纠错模式不匹配

  `ETHTOOL_LINK_EXT_SUBSTATE_AN_NO_HCD`                           无最高公分母（Highest Common Denominator
  ===============================================================   ================================

  链路训练子状态：

  ===========================================================================   ====================
  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_FRAME_LOCK_NOT_ACQUIRED`                    帧未被识别，锁定失败

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_LINK_INHIBIT_TIMEOUT`                       在超时前未完成锁

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_KR_LINK_PARTNER_DID_NOT_SET_RECEIVER_READY`    训练过程后对端未发出就绪信号

  `ETHTOOL_LINK_EXT_SUBSTATE_LT_REMOTE_FAULT`                                  远端尚未就绪
  ===========================================================================   ====================

  链路逻辑不匹配子状态：

  ================================================================   ===============================
  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_ACQUIRE_BLOCK_LOCK`  物理编码子层在第一阶段未锁定——块

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_ACQUIRE_AM_LOCK`     物理编码子层在第二阶段未锁定——对齐标记锁

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_PCS_DID_NOT_GET_ALIGN_STATUS`    物理编码子层未获得对齐状

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_FC_FEC_IS_NOT_LOCKED`            FC 前向纠错未锁

  `ETHTOOL_LINK_EXT_SUBSTATE_LLM_RS_FEC_IS_NOT_LOCKED`            RS 前向纠错未锁
  ================================================================   ===============================

  信号完整性差子状态：

  =================================================================    =============================
  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_LARGE_NUMBER_OF_PHYSICAL_ERRORS`    大量物理错误

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_UNSUPPORTED_RATE`                   系统尝试以不被正式支持的速率运行线缆，导致信号完整性问

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_SERDES_REFERENCE_CLOCK_LOST`        SerDes 的外部时钟信号过弱或不可

  `ETHTOOL_LINK_EXT_SUBSTATE_BSI_SERDES_ALOS`                        SerDes 的接收信号因模拟信号丢失而过
  =================================================================    =============================

  线缆问题子状态：

  ===================================================   ============================================
  `ETHTOOL_LINK_EXT_SUBSTATE_CI_UNSUPPORTED_CABLE`    不支持的线缆

  `ETHTOOL_LINK_EXT_SUBSTATE_CI_CABLE_TEST_FAILURE`   线缆测试失败
  ===================================================   ============================================

  收发器模块问题子状态：

  ===================================================   ============================================
  `ETHTOOL_LINK_EXT_SUBSTATE_MODULE_CMIS_NOT_READY`   CMIS 模块状态机未到ModuleReady 状态，例如模块停留ModuleFault 状
  ===================================================   ============================================

## DEBUG_GET


请求设备的调试设置。目前仅提供消息掩码

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  reply header
  `ETHTOOL_A_DEBUG_MSGMASK`           bitset  message mask
  ====================================  ======  ==========================

消息掩码（`ETHTOOL_A_DEBUG_MSGMASK`）等同于 ioctl 接口中由 `ETHTOOL_GMSGLVL` 提供
并由 `ETHTOOL_SMSGLVL` 设置的消息级别。虽然出于历史原因在那里被称为消息级别，但大多数
驱动以及几乎所有较新的驱动都将其用作启用消息类别的掩码（由 `NETIF_MSG_*` 常量表示）；
因此 netlink 接口遵循其实际用法

`DEBUG_GET` 允许 dump 请求（内核为所有支持该请求的设备返回回复消息）


## DEBUG_SET


设置或更新设备的调试设置。目前仅支持消息掩码

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_DEBUG_HEADER`            nested  request header
  `ETHTOOL_A_DEBUG_MSGMASK`           bitset  message mask
  ====================================  ======  ==========================

`ETHTOOL_A_DEBUG_MSGMASK` 位集允许设置或修改设备已启用的调试消息类型的掩码


## WOL_GET


查询设备的唤醒局域网（wake-on-lan）设置。与大多数“GET”类型的请求不同
`ETHTOOL_MSG_WOL_GET` 需要（netns 的）`CAP_NET_ADMIN` 权限，因为它（可能）会提
保密SecureOn(tm) 密码

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  reply header
  `ETHTOOL_A_WOL_MODES`               bitset  mask of enabled WoL modes
  `ETHTOOL_A_WOL_SOPASS`              binary  SecureOn(tm) password
  ====================================  ======  ==========================

在回复中，`ETHTOOL_A_WOL_MODES` 掩码由设备支持的模式，以及其中已启用的模式值组成
仅当支持 `WAKE_MAGICSECURE` 模式时，`ETHTOOL_A_WOL_SOPASS` 才会包含在回复中


## WOL_SET


设置或更新唤醒局域网（wake-on-lan）设置

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_WOL_HEADER`              nested  request header
  `ETHTOOL_A_WOL_MODES`               bitset  enabled WoL modes
  `ETHTOOL_A_WOL_SOPASS`              binary  SecureOn(tm) password
  ====================================  ======  ==========================

`ETHTOOL_A_WOL_SOPASS` 仅允许用于支`WAKE_MAGICSECURE` 模式的设备


## FEATURES_GET


获取网络设备特性，类似`ETHTOOL_GFEATURES` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  reply header
  `ETHTOOL_A_FEATURES_HW`             bitset  dev->hw_features
  `ETHTOOL_A_FEATURES_WANTED`         bitset  dev->wanted_features
  `ETHTOOL_A_FEATURES_ACTIVE`         bitset  dev->features
  `ETHTOOL_A_FEATURES_NOCHANGE`       bitset  NETIF_F_NEVER_CHANGE
  ====================================  ======  ==========================


内核响应中的位图ioctl 接口中使用的位图含义相同，但属性名称不同（它们基于
struct net_device 的对应成员）。旧式的“flags”不会被提供；如果用户空间需要它
（很可能只有 ethtool 为了向后兼容），它可以根据相关的特性位自行计算其值
ETHA_FEATURES_HW 使用的掩码由内核识别的所有特性组成（以便在使用详细位图格式时提供
全部名称），其余三个则不使用掩码（仅作为简单的位列表）


## FEATURES_SET


请求设置网络设备特性，类似`ETHTOOL_SFEATURES` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  request header
  `ETHTOOL_A_FEATURES_WANTED`         bitset  requested features
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_FEATURES_HEADER`         nested  reply header
  `ETHTOOL_A_FEATURES_WANTED`         bitset  diff wanted vs. result
  `ETHTOOL_A_FEATURES_ACTIVE`         bitset  diff old vs. new active
  ====================================  ======  ==========================

请求中只包含一个位集，它可以是掩码对（请求更改特定的特性位而保留其余）或仅一个
（请求将所有特性设置为指定的集合）

由于请求要接netdev_change_features() 的合理性检查，可选的内核回复（可通过请求头中
`ETHTOOL_FLAG_OMIT_REPLY` 标志抑制）会告知客户端实际结果。`ETHTOOL_A_FEATURES_WANTED`
报告客户端请求与实际结果之间的差异：掩码由请求的特性与结果（操作后 dev->features）之
不同的位组成，值由这些位在请求中的取值组成（即来自结果特性的取反值）
`ETHTOOL_A_FEATURES_ACTIVE` 报告新旧 dev->features 之间的差异：掩码由发生变化的位组成，
值为这些位在新的 dev->features（操作后）中的取值

`ETHTOOL_MSG_FEATURES_NTF` 通知不仅在通过 `ETHTOOL_MSG_FEATURES_SET` 请求或修
ethtool ioctl 请求来修改设备特性时发送，也会在每次通过 netdev_update_features() 
netdev_change_features() 修改特性时发送


## PRIVFLAGS_GET


获取私有标志，类似于 `ETHTOOL_GPFLAGS` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  reply header
  `ETHTOOL_A_PRIVFLAGS_FLAGS`         bitset  private flags
  ====================================  ======  ==========================

`ETHTOOL_A_PRIVFLAGS_FLAGS` 是一个带有设备私有标志值的位集。这些标志由驱动定义，其数量
与名称（以及含义）取决于具体设备。对于紧凑位集格式，名称可通过 `ETH_SS_PRIV_FLAGS`
字符串集获取。如果请求了详细位集格式，响应会使用设备支持的全部私有标志作为掩码，从
客户端无需再去获取带有名称的字符串集即可获得完整信息


## PRIVFLAGS_SET


设置或修改设备私有标志的值，类似`ETHTOOL_SPFLAGS` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PRIVFLAGS_HEADER`        nested  request header
  `ETHTOOL_A_PRIVFLAGS_FLAGS`         bitset  private flags
  ====================================  ======  ==========================

`ETHTOOL_A_PRIVFLAGS_FLAGS` 既可以设置整个私有标志集合，也可以只修改其中部分标志的值


## RINGS_GET


获取环形队列大小，类似于 `ETHTOOL_GRINGPARAM` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_RINGS_HEADER`            nested  request header
  ====================================  ======  ==========================

内核响应内容

  =======================================   ======  ===========================
  `ETHTOOL_A_RINGS_HEADER`                nested  reply header
  `ETHTOOL_A_RINGS_RX_MAX`                u32     接收（RX）环形队列最大大
  `ETHTOOL_A_RINGS_RX_MINI_MAX`           u32     RX mini 环形队列最大大
  `ETHTOOL_A_RINGS_RX_JUMBO_MAX`          u32     RX jumbo 环形队列最大大
  `ETHTOOL_A_RINGS_TX_MAX`                u32     发送（TX）环形队列最大大
  `ETHTOOL_A_RINGS_RX`                    u32     RX 环形队列大小
  `ETHTOOL_A_RINGS_RX_MINI`               u32     RX mini 环形队列大小
  `ETHTOOL_A_RINGS_RX_JUMBO`              u32     RX jumbo 环形队列大小
  `ETHTOOL_A_RINGS_TX`                    u32     TX 环形队列大小
  `ETHTOOL_A_RINGS_RX_BUF_LEN`            u32     环形队列上缓冲区的大
  `ETHTOOL_A_RINGS_TCP_DATA_SPLIT`        u8      TCP / 数据分离
  `ETHTOOL_A_RINGS_CQE_SIZE`              u32     TX/RX CQE 的大
  `ETHTOOL_A_RINGS_TX_PUSH`               u8      TX Push 模式标志
  `ETHTOOL_A_RINGS_RX_PUSH`               u8      RX Push 模式标志
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN`       u32     TX push 缓冲区大
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN_MAX`   u32     TX push 缓冲区最大大
  `ETHTOOL_A_RINGS_HDS_THRESH`            u32     / 数据分离阈
  `ETHTOOL_A_RINGS_HDS_THRESH_MAX`        u32     / 数据分离最大阈
  =======================================   ======  ===========================

`ETHTOOL_A_RINGS_TCP_DATA_SPLIT` 指示该设备是否可与页翻转（page-flipping）的 TCP 零拷
接收（`getsockopt(TCP_ZEROCOPY_RECEIVE)`）配合使用。若启用，设备被配置为将帧头与数
放入独立的缓冲区。设备配置必须能够接收完整的内存页数据，例如因为 MTU 足够大或通过
HW-GRO銆。

`ETHTOOL_A_RINGS_[RX|TX]_PUSH` 标志用于启用描述符快速路径来发送或接收数据包。在普通路
中，驱动DRAM 中填充描述符并通知 NIC 硬件。在快速路径中，驱动通过 MMIO 写操作将描述
推送到设备，从而降低延迟。然而，启用该特性可能增CPU 开销。驱动可能会施加额外的逐包
资格检查（例如依据包大小）

`ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN` 指定驱动可以直接推送到底层设备（‘push’模式）的发送包
的最大字节数。将部分载荷字节推送到设备具有减少小包延迟（避DMA 映射，与
`ETHTOOL_A_RINGS_TX_PUSH` 参数相同）以及允许底层设备在处理其载荷之前先处理包头的优势
这可以帮助设备基于包头快速采取行动。这与“tx-copybreak”参数类似，后者将包复制到预分配的
DMA 内存区域而非映射新内存。然而，tx-push-buff 参数将包直接复制到设备，以让设备能对
采取更快的动作

## RINGS_SET


设置环形队列大小，类似于 `ETHTOOL_SRINGPARAM` ioctl 请求

请求内容

  ====================================  ======  ===========================
  `ETHTOOL_A_RINGS_HEADER`            nested  reply header
  `ETHTOOL_A_RINGS_RX`                u32     RX 环形队列大小
  `ETHTOOL_A_RINGS_RX_MINI`           u32     RX mini 环形队列大小
  `ETHTOOL_A_RINGS_RX_JUMBO`          u32     RX jumbo 环形队列大小
  `ETHTOOL_A_RINGS_TX`                u32     TX 环形队列大小
  `ETHTOOL_A_RINGS_RX_BUF_LEN`        u32     环形队列上缓冲区的大
  `ETHTOOL_A_RINGS_TCP_DATA_SPLIT`    u8      TCP / 数据分离
  `ETHTOOL_A_RINGS_CQE_SIZE`          u32     TX/RX CQE 的大
  `ETHTOOL_A_RINGS_TX_PUSH`           u8      TX Push 模式标志
  `ETHTOOL_A_RINGS_RX_PUSH`           u8      RX Push 模式标志
  `ETHTOOL_A_RINGS_TX_PUSH_BUF_LEN`   u32     TX push 缓冲区大
  `ETHTOOL_A_RINGS_HDS_THRESH`        u32     / 数据分离阈
  ====================================  ======  ===========================

内核会检查请求的环形队列大小不超过驱动上报的限制。驱动可能施加额外的约束，也可能不支
所有属性


`ETHTOOL_A_RINGS_CQE_SIZE` 指定完成队列事件（Completion Queue Event）的大小。完成队
事件（CQE）是 NIC 发出的、用于指示包发送（如发送成功或出错）或接收（如包片段指针）完成
状态的事件。CQE 大小参数可在 NIC 支持时修改默认的 CQE 大小。更大的 CQE 可以携带更多的接
缓冲区指针，进NIC 可从线路上传输更大的帧。基NIC 硬件，若修改CQE 大小，整体完
队列大小可在驱动中调整

`ETHTOOL_A_RINGS_HDS_THRESH` 指定/ 数据分离特性的阈值。若接收到的包大小大于该阈值，
则头与数据将被分离

## CHANNELS_GET


获取通道数量，类似于 `ETHTOOL_GCHANNELS` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`         nested  request header
  ====================================  ======  ==========================

内核响应内容

  =====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`          nested  reply header
  `ETHTOOL_A_CHANNELS_RX_MAX`          u32     max receive channels
  `ETHTOOL_A_CHANNELS_TX_MAX`          u32     max transmit channels
  `ETHTOOL_A_CHANNELS_OTHER_MAX`       u32     max other channels
  `ETHTOOL_A_CHANNELS_COMBINED_MAX`    u32     max combined channels
  `ETHTOOL_A_CHANNELS_RX_COUNT`        u32     receive channel count
  `ETHTOOL_A_CHANNELS_TX_COUNT`        u32     transmit channel count
  `ETHTOOL_A_CHANNELS_OTHER_COUNT`     u32     other channel count
  `ETHTOOL_A_CHANNELS_COMBINED_COUNT`  u32     combined channel count
  =====================================  ======  ==========================


## CHANNELS_SET


设置通道数量，类似于 `ETHTOOL_SCHANNELS` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_CHANNELS_HEADER`          nested  request header
  `ETHTOOL_A_CHANNELS_RX_COUNT`        u32     receive channel count
  `ETHTOOL_A_CHANNELS_TX_COUNT`        u32     transmit channel count
  `ETHTOOL_A_CHANNELS_OTHER_COUNT`     u32     other channel count
  `ETHTOOL_A_CHANNELS_COMBINED_COUNT`  u32     combined channel count
  =====================================  ======  ==========================

内核会检查请求的通道数量不超过驱动上报的限制。驱动可能施加额外的约束，也可能不支持所
属性


## COALESCE_GET


获取中断聚合参数，类似于 `ETHTOOL_GCOALESCE` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_COALESCE_HEADER`         nested  request header
  ====================================  ======  ==========================

内核响应内容

  ===========================================  ======  =======================
  `ETHTOOL_A_COALESCE_HEADER`                nested  reply header
  `ETHTOOL_A_COALESCE_RX_USECS`              u32     延迟（微秒），普Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES`         u32     最大包数，普Rx
  `ETHTOOL_A_COALESCE_RX_USECS_IRQ`          u32     延迟（微秒），IRQ 中的 Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ`     u32     最大包数，IRQ 中的 Rx
  `ETHTOOL_A_COALESCE_TX_USECS`              u32     延迟（微秒），普Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES`         u32     最大包数，普Tx
  `ETHTOOL_A_COALESCE_TX_USECS_IRQ`          u32     延迟（微秒），IRQ 中的 Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ`     u32     IRQ 中的包数，Tx
  `ETHTOOL_A_COALESCE_STATS_BLOCK_USECS`     u32     统计更新延迟
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX`       bool    自适应 Rx 聚合
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX`       bool    自适应 Tx 聚合
  `ETHTOOL_A_COALESCE_PKT_RATE_LOW`          u32     低速率阈
  `ETHTOOL_A_COALESCE_RX_USECS_LOW`          u32     延迟（微秒），低Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW`     u32     最大包数，低Rx
  `ETHTOOL_A_COALESCE_TX_USECS_LOW`          u32     延迟（微秒），低Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW`     u32     最大包数，低Tx
  `ETHTOOL_A_COALESCE_PKT_RATE_HIGH`         u32     高速率阈
  `ETHTOOL_A_COALESCE_RX_USECS_HIGH`         u32     延迟（微秒），高Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH`    u32     最大包数，高Rx
  `ETHTOOL_A_COALESCE_TX_USECS_HIGH`         u32     延迟（微秒），高Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH`    u32     最大包数，高Tx
  `ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL`  u32     速率采样间隔
  `ETHTOOL_A_COALESCE_USE_CQE_TX`            bool    定时器重置模式，Tx
  `ETHTOOL_A_COALESCE_USE_CQE_RX`            bool    定时器重置模式，Rx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES`     u32     最大聚合大小，Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES`    u32     最大聚合包数，Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS`    u32     时间（微秒），聚合，Tx
  `ETHTOOL_A_COALESCE_RX_PROFILE`            nested  DIM 配置文件，Rx
  `ETHTOOL_A_COALESCE_TX_PROFILE`            nested  DIM 配置文件，Tx
  `ETHTOOL_A_COALESCE_RX_CQE_FRAMES`         u32     最大包数，Rx CQE
  `ETHTOOL_A_COALESCE_RX_CQE_NSECS`          u32     延迟（纳秒），Rx CQE
  ===========================================  ======  =======================

仅当属性的值不为零，或 **对应位在 ``ethtool_ops`` ``supported_coalesce_params`` *
被设置（即被驱动声明为支持）时，该属性才会包含在回复中

定时器重置模式（`ETHTOOL_A_COALESCE_USE_CQE_TX` `ETHTOOL_A_COALESCE_USE_CQE_RX`
控制包到达与各个基于时间的延迟参数之间的交互。默认情况下，定时器应限制任意包到达/离开
与相应中断之间的最大延迟。在此模式下，定时器应由包到达（有时是上一次中断的投递）启动
并在中断投递时重置。将相应属性设置为 1 将启`CQE` 模式，其中每个包事件都会重置定时器
在此模式下，定时器用于防止队列空闲时强制产生中断，而繁忙的队列则依赖包上限来触发中断

Tx 聚合是指将帧复制到连续的缓冲区中，以便作为一个单独的 IO 操作提交
`ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES` 描述提交缓冲区的最大字节数
`ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES` 描述可聚合到单个缓冲区中的最大帧数
`ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS` 描述自聚合块中第一个包到达起算的时间（微秒），
超过该时间后应发送该块。此特性主要对某些不能很好处理频繁小尺URB 传输的特USB 设备
有意义

`ETHTOOL_A_COALESCE_RX_PROFILE` `ETHTOOL_A_COALESCE_TX_PROFILE` 引用 DIM 参数
参见 `Generic Network Dynamic Interrupt Moderation (Net DIM)
<https://www.kernel.org/doc/Documentation/networking/net_dim.rst>`_銆。

Rx CQE 聚合允许多个接收到的包被聚合到单个完成队列条目（CQE）或描述符回写中
`ETHTOOL_A_COALESCE_RX_CQE_FRAMES` 描述可聚合到 CQE 或回写中的最大帧数
`ETHTOOL_A_COALESCE_RX_CQE_NSECS` 描述聚合CQE 或回写自第一个包到达起、被发送前
最大时间（纳秒）

## COALESCE_SET


设置中断聚合参数，类似于 `ETHTOOL_SCOALESCE` ioctl 请求

请求内容

  ===========================================  ======  =======================
  `ETHTOOL_A_COALESCE_HEADER`                nested  request header
  `ETHTOOL_A_COALESCE_RX_USECS`              u32     延迟（微秒），普Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES`         u32     最大包数，普Rx
  `ETHTOOL_A_COALESCE_RX_USECS_IRQ`          u32     延迟（微秒），IRQ 中的 Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_IRQ`     u32     最大包数，IRQ 中的 Rx
  `ETHTOOL_A_COALESCE_TX_USECS`              u32     延迟（微秒），普Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES`         u32     最大包数，普Tx
  `ETHTOOL_A_COALESCE_TX_USECS_IRQ`          u32     延迟（微秒），IRQ 中的 Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_IRQ`     u32     IRQ 中的包数，Tx
  `ETHTOOL_A_COALESCE_STATS_BLOCK_USECS`     u32     统计更新延迟
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_RX`       bool    自适应 Rx 聚合
  `ETHTOOL_A_COALESCE_USE_ADAPTIVE_TX`       bool    自适应 Tx 聚合
  `ETHTOOL_A_COALESCE_PKT_RATE_LOW`          u32     低速率阈
  `ETHTOOL_A_COALESCE_RX_USECS_LOW`          u32     延迟（微秒），低Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_LOW`     u32     最大包数，低Rx
  `ETHTOOL_A_COALESCE_TX_USECS_LOW`          u32     延迟（微秒），低Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_LOW`     u32     最大包数，低Tx
  `ETHTOOL_A_COALESCE_PKT_RATE_HIGH`         u32     高速率阈
  `ETHTOOL_A_COALESCE_RX_USECS_HIGH`         u32     延迟（微秒），高Rx
  `ETHTOOL_A_COALESCE_RX_MAX_FRAMES_HIGH`    u32     最大包数，高Rx
  `ETHTOOL_A_COALESCE_TX_USECS_HIGH`         u32     延迟（微秒），高Tx
  `ETHTOOL_A_COALESCE_TX_MAX_FRAMES_HIGH`    u32     最大包数，高Tx
  `ETHTOOL_A_COALESCE_RATE_SAMPLE_INTERVAL`  u32     速率采样间隔
  `ETHTOOL_A_COALESCE_USE_CQE_TX`            bool    定时器重置模式，Tx
  `ETHTOOL_A_COALESCE_USE_CQE_RX`            bool    定时器重置模式，Rx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_BYTES`     u32     最大聚合大小，Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_MAX_FRAMES`    u32     最大聚合包数，Tx
  `ETHTOOL_A_COALESCE_TX_AGGR_TIME_USECS`    u32     时间（微秒），聚合，Tx
  `ETHTOOL_A_COALESCE_RX_PROFILE`            nested  DIM 配置文件，Rx
  `ETHTOOL_A_COALESCE_TX_PROFILE`            nested  DIM 配置文件，Tx
  `ETHTOOL_A_COALESCE_RX_CQE_FRAMES`         u32     最大包数，Rx CQE
  `ETHTOOL_A_COALESCE_RX_CQE_NSECS`          u32     延迟（纳秒），Rx CQE
  ===========================================  ======  =======================

如果请求包含被驱动声明为不支持的属性（**相应位在 ``ethtool_ops`` 
``supported_coalesce_params`` 中未设置**），则无论其值如何请求都会被拒绝。驱动可能对
聚合参数及其取值施加额外的约束

与通过 `ioctl()` 发出的请求相比，该请求的 netlink 版本会更努力地确保用户指定的值已
应用，并可能调用驱动两次


## PAUSE_GET


获取暂停帧设置，类似`ETHTOOL_GPAUSEPARAM` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_STATS_SRC`          u32     source of statistics
  =====================================  ======  ==========================

`ETHTOOL_A_PAUSE_STATS_SRC` 是可选的。它取值自

    :identifiers: ethtool_mac_stats_src

若请求中缺省，则响应中会带有一个等`ETHTOOL_MAC_STATS_SRC_AGGREGATE` 
`ETHTOOL_A_PAUSE_STATS_SRC` 属性来提供统计信息

内核响应内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_AUTONEG`            bool    pause autonegotiation
  `ETHTOOL_A_PAUSE_RX`                 bool    receive pause frames
  `ETHTOOL_A_PAUSE_TX`                 bool    transmit pause frames
  `ETHTOOL_A_PAUSE_STATS`              nested  pause statistics
  =====================================  ======  ==========================

`ETHTOOL_A_HEADER_FLAGS` 中设置了 `ETHTOOL_FLAG_STATS`，则会报
`ETHTOOL_A_PAUSE_STATS`。如果驱动未报告任何统计信息，它将是空的。驱动在以下结构中填
统计信息

    :identifiers: ethtool_pause_stats

每个成员都有对应的已定义属性


## PAUSE_SET


设置暂停参数，类似于 `ETHTOOL_GPAUSEPARAM` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PAUSE_HEADER`             nested  request header
  `ETHTOOL_A_PAUSE_AUTONEG`            bool    pause autonegotiation
  `ETHTOOL_A_PAUSE_RX`                 bool    receive pause frames
  `ETHTOOL_A_PAUSE_TX`                 bool    transmit pause frames
  =====================================  ======  ==========================


## EEE_GET


获取高效以太网（Energy Efficient Ethernet）设置，类似`ETHTOOL_GEEE` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  =====================================  ======  ==========================

内核响应内容

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  `ETHTOOL_A_EEE_MODES_OURS`           bool    supported/advertised modes
  `ETHTOOL_A_EEE_MODES_PEER`           bool    peer advertised link modes
  `ETHTOOL_A_EEE_ACTIVE`               bool    EEE is actively used
  `ETHTOOL_A_EEE_ENABLED`              bool    EEE is enabled
  `ETHTOOL_A_EEE_TX_LPI_ENABLED`       bool    Tx lpi enabled
  `ETHTOOL_A_EEE_TX_LPI_TIMER`         u32     Tx lpi timeout (in us)
  =====================================  ======  ==========================

`ETHTOOL_A_EEE_MODES_OURS` 中，掩码由启EEE 的链路模式组成，值为通告EEE 的链
模式。对端通告EEE 的链路模式列`ETHTOOL_A_EEE_MODES_PEER` 中（无掩码）。netlink
接口允许报告所有链路模式的 EEE 状态，但目前只有前 32 个由 `ethtool_ops` 回调提供


## EEE_SET


设置高效以太网参数，类似`ETHTOOL_SEEE` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_EEE_HEADER`               nested  request header
  `ETHTOOL_A_EEE_MODES_OURS`           bool    advertised modes
  `ETHTOOL_A_EEE_ENABLED`              bool    EEE is enabled
  `ETHTOOL_A_EEE_TX_LPI_ENABLED`       bool    Tx lpi enabled
  `ETHTOOL_A_EEE_TX_LPI_TIMER`         u32     Tx lpi timeout (in us)
  =====================================  ======  ==========================

`ETHTOOL_A_EEE_MODES_OURS` 用于列出要通告 EEE 的链路模式（若无掩码），或指定对该列表的
更改（若有掩码）。netlink 接口允许报告所有链路模式的 EEE 状态，但目前只能设置前 32 个，
因为 `ethtool_ops` 回调仅支持这么多


## TSINFO_GET


获取时间戳信息，类似`ETHTOOL_GET_TS_INFO` ioctl 请求

请求内容

  ========================================  ======  ============================
  `ETHTOOL_A_TSINFO_HEADER`               nested  request header
  `ETHTOOL_A_TSINFO_HWTSTAMP_PROVIDER`    nested  PTP hw clock provider
  ========================================  ======  ============================

内核响应内容

  =====================================  ======  ==========================
  `ETHTOOL_A_TSINFO_HEADER`            nested  request header
  `ETHTOOL_A_TSINFO_TIMESTAMPING`      bitset  SO_TIMESTAMPING flags
  `ETHTOOL_A_TSINFO_TX_TYPES`          bitset  supported Tx types
  `ETHTOOL_A_TSINFO_RX_FILTERS`        bitset  supported Rx filters
  `ETHTOOL_A_TSINFO_PHC_INDEX`         u32     PTP hw clock index
  `ETHTOOL_A_TSINFO_STATS`             nested  HW timestamping statistics
  =====================================  ======  ==========================

若无关联PHC，则 `ETHTOOL_A_TSINFO_PHC_INDEX` 缺省（此情况无特殊取值）。若位集属性将
为空（无任何位被设置），则会被省略

额外的硬件时间戳统计响应内容

  ==================================================  ======  =====================
  `ETHTOOL_A_TS_STAT_TX_PKTS`                       uint    Tx 硬件时间戳的
  `ETHTOOL_A_TS_STAT_TX_LOST`                       uint    未到达的 Tx 硬件时间戳计
  `ETHTOOL_A_TS_STAT_TX_ERR`                        uint    硬件错误请求Tx 时间戳计
  `ETHTOOL_A_TS_STAT_TX_ONESTEP_PKTS_UNCONFIRMED`   uint    带一步（one-step）硬Tx 时间戳、投递未确认的包
  ==================================================  ======  =====================

## CABLE_TEST


启动线缆测试

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_CABLE_TEST_HEADER`       nested  request header
  ====================================  ======  ==========================

通知内容

一条以太网线缆通常包含 1 4 对线。只有在某对线存在故障从而发生反射时，才能测量该
线的长度。具体硬件可能不提供故障信息。因此通知消息的内容大多是可选的。这些属性可以以
任意次数、任意顺序，针对任意数量的线对重复出现

示例展示了对 T2 线缆（即两对线）完成测试时发送的通知。其中一对正常，因此没有长度信息
第二对存在故障，因此带有长度信息

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_HEADER`             | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_STATUS`             | u8     | completed           |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_CABLE_TEST_NTF_NEST`           | nested | all the results     |
 +-+-------------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_RESULT`           | nested | cable test result   |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_CODE`        | u8     | result code         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_RESULT`           | nested | cable test results  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_CODE`        | u8     | result code         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_RESULT_SRC`          | u32    | information source  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_CABLE_NEST_FAULT_LENGTH`     | nested | cable length        |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_PAIR`   | u8     | pair number         |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_CM`     | u32    | length in cm        |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_CABLE_FAULT_LENGTH_SRC`    | u32    | information source  |
 +-+-+-----------------------------------------+--------+---------------------+

## CABLE_TEST TDR


启动线缆测试并上报原TDR 数据

请求内容

 +--------------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_HEADER`        | nested | reply header          |
 +--------------------------------------------+--------+-----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_CFG`           | nested | test configuration    |
 +-+------------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE`  | u32    | first data distance   |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_LAST_DISTANCE`   | u32    | last data distance    |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_STEP_STEP_DISTANCE`   | u32    | distance of each step |
 +-+-+----------------------------------------+--------+-----------------------+
 | | `ETHTOOL_A_CABLE_TEST_TDR_CFG_PAIR`    | u8     | pair to test          |
 +-+-+----------------------------------------+--------+-----------------------+

ETHTOOL_A_CABLE_TEST_TDR_CFG 及其嵌套中的全部成员均为可选。所有距离都以厘米表示。PHY 
这些距离作为参考，并取整到它实际支持的最近距离。如果传入某对线，则只测试该对线；否则测
所有对线

通知内容

原始 TDR 数据通过向线缆发送脉冲并记录给定距离的反射脉冲幅度来采集

如果1 米间隔探测完整的 100 米，采集 TDR 数据可能需要若干秒。测试启动时会发送一
仅包ETHTOOL_A_CABLE_TEST_TDR_STATUS、且值为
ETHTOOL_A_CABLE_TEST_NTF_STATUS_STARTED 的通知

测试完成时会发送第二条通知，包ETHTOOL_A_CABLE_TEST_TDR_STATUS（值为
ETHTOOL_A_CABLE_TEST_NTF_STATUS_COMPLETED）以TDR 数据

消息可能可选地包含沿线缆发送的脉冲幅度。它mV 计量。反射不应大于发送的脉冲

在原TDR 数据之前应有一ETHTOOL_A_CABLE_TDR_NEST_STEP 嵌套，其中包含关于首次读取
末次读取以及每次读取之间步进距离的信息。距离以厘米计量。这些应PHY 使用的精确值。如
原生测量分辨率大1 cm，这些值可能与用户请求的不同

对于线缆上的每一步，使用一ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE 来报告给定对线上的反
幅度

 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_HEADER`         | nested | reply header         |
 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_STATUS`         | u8     | completed            |
 +---------------------------------------------+--------+----------------------+
 | `ETHTOOL_A_CABLE_TEST_TDR_NTF_NEST`       | nested | all the results      |
 +-+-------------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_PULSE`        | nested | TX Pulse amplitude   |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_PULSE_mV`            | s16    | Pulse amplitude      |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_NEST_STEP`             | nested | TDR step info        |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_FIRST_DISTANCE` | u32    | First data distance  |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_LAST_DISTANCE`  | u32    | Last data distance   |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_STEP_STEP_DISTANCE`  | u32    | distance of each step|
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | `ETHTOOL_A_CABLE_TDR_NEST_AMPLITUDE`    | nested | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_RESULTS_PAIR`        | u8     | pair number          |
 +-+-+-----------------------------------------+--------+----------------------+
 | | | `ETHTOOL_A_CABLE_AMPLITUDE_mV`        | s16    | Reflection amplitude |
 +-+-+-----------------------------------------+--------+----------------------+

## TUNNEL_INFO


获取 NIC 所感知的隧道状态信息

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_TUNNEL_INFO_HEADER`       nested  request header
  =====================================  ======  ==========================

内核响应内容

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_TUNNEL_INFO_HEADER`            | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_TUNNEL_INFO_UDP_PORTS`         | nested | all UDP port tables |
 +-+-------------------------------------------+--------+---------------------+
 | | `ETHTOOL_A_TUNNEL_UDP_TABLE`            | nested | one UDP port table  |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_SIZE`     | u32    | max size of the     |
 | | |                                         |        | table               |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES`    | bitset | tunnel types which  |
 | | |                                         |        | table can hold      |
 +-+-+-----------------------------------------+--------+---------------------+
 | | | `ETHTOOL_A_TUNNEL_UDP_TABLE_ENTRY`    | nested | offloaded UDP port  |
 +-+-+-+---------------------------------------+--------+---------------------+
 | | | | `ETHTOOL_A_TUNNEL_UDP_ENTRY_PORT`   | be16   | UDP port            |
 +-+-+-+---------------------------------------+--------+---------------------+
 | | | | `ETHTOOL_A_TUNNEL_UDP_ENTRY_TYPE`   | u32    | tunnel type         |
 +-+-+-+---------------------------------------+--------+---------------------+

对于 UDP 隧道表，空的 `ETHTOOL_A_TUNNEL_UDP_TABLE_TYPES` 表示该表包含NIC 硬编码的
静态条目

## FEC_GET


获取 FEC 配置与状态，类似`ETHTOOL_GFECPARAM` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  =====================================  ======  ==========================

内核响应内容

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  `ETHTOOL_A_FEC_MODES`                bitset  configured modes
  `ETHTOOL_A_FEC_AUTO`                 bool    FEC mode auto selection
  `ETHTOOL_A_FEC_ACTIVE`               u32     index of active FEC mode
  `ETHTOOL_A_FEC_STATS`                nested  FEC statistics
  =====================================  ======  ==========================

`ETHTOOL_A_FEC_ACTIVE` 是当前在接口上处于活动状态的 FEC 链路模式的位索引。若设备不支
FEC，该属性可能不存在

`ETHTOOL_A_FEC_MODES` `ETHTOOL_A_FEC_AUTO` 仅在禁用自协商时才有意义。若
`ETHTOOL_A_FEC_AUTO` 非零，驱动将根据 SFP 模块的参数自动选择 FEC 模式。这等价ioctl
接口`ETHTOOL_FEC_AUTO` 位。`ETHTOOL_A_FEC_MODES` 使用链路模式位（而非旧的
`ETHTOOL_FEC_*` 位）携带当前FEC 配置

`ETHTOOL_A_HEADER_FLAGS` 中设置了 `ETHTOOL_FLAG_STATS`，则会报
`ETHTOOL_A_FEC_STATS`。每个属性携带一个由 64 位统计组成的数组。数组的第一个条目包含端
上的事件总数，后续条目则对应通道/PCS 实例的计数器。数组中的条目数将为

+--------------+---------------------------------------------+
| `0`          | 设备不支FEC 统计                          |
+--------------+---------------------------------------------+
| `1`          | 设备不支持按通道细分                          |
+--------------+---------------------------------------------+
| `1 + #lanes` | 设备完全支持 FEC 统计                          |
+--------------+---------------------------------------------+

驱动在以下结构中填写统计信息

    :identifiers: ethtool_fec_stats

统计可能带有 FEC 分箱直方图属`ETHTOOL_A_FEC_STAT_HIST`，其定义IEEE 802.3ck-2022
802.3df-2024。嵌套属性将包含该分箱内 FEC 错误的范围（含边界）以及该分箱内的错误事
数量

## FEC_SET


设置 FEC 参数，类似于 `ETHTOOL_SFECPARAM` ioctl 请求

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_FEC_HEADER`               nested  request header
  `ETHTOOL_A_FEC_MODES`                bitset  configured modes
  `ETHTOOL_A_FEC_AUTO`                 bool    FEC mode auto selection
  =====================================  ======  ==========================

`FEC_SET` 仅在禁用自协商时有意义。否FEC 模式将作为自协商的一部分被选择


`ETHTOOL_A_FEC_MODES` 选择应使用哪FEC 模式。建议只设置一位；若设置了多位，驱动可
以具体实现相关的方式在其中选择

`ETHTOOL_A_FEC_AUTO` 请求驱动根据 SFP 模块参数选择 FEC 模式。这并不代表自协商

## MODULE_EEPROM_GET


获取模块 EEPROM 数据转储。此接口设计为每次最多允许转1/2 页。这意味着只允许转
128（或更少）字节，且不得跨越位于偏128 处的半页边界。对0 之外的其它页，只有高
128 字节可访问

请求内容

  =======================================  ======  ==========================
  `ETHTOOL_A_MODULE_EEPROM_HEADER`       nested  request header
  `ETHTOOL_A_MODULE_EEPROM_OFFSET`       u32     offset within a page
  `ETHTOOL_A_MODULE_EEPROM_LENGTH`       u32     amount of bytes to read
  `ETHTOOL_A_MODULE_EEPROM_PAGE`         u8      page number
  `ETHTOOL_A_MODULE_EEPROM_BANK`         u8      bank number
  `ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS`  u8      page I2C address
  =======================================  ======  ==========================

若未指定 `ETHTOOL_A_MODULE_EEPROM_BANK`，则假定bank 0

内核响应内容

 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_MODULE_EEPROM_HEADER`          | nested | reply header        |
 +---------------------------------------------+--------+---------------------+
 | `ETHTOOL_A_MODULE_EEPROM_DATA`            | binary | array of bytes from |
 |                                             |        | module EEPROM       |
 +---------------------------------------------+--------+---------------------+

`ETHTOOL_A_MODULE_EEPROM_DATA` 的属性长度等于驱动实际读取的字节数

## STATS_GET


获取接口的标准统计信息。注意，这不是对暴露驱动定义统计`ETHTOOL_GSTATS` 的重新实现

请求内容

  =======================================  ======  ==========================
  `ETHTOOL_A_STATS_HEADER`               nested  request header
  `ETHTOOL_A_STATS_SRC`                  u32     source of statistics
  `ETHTOOL_A_STATS_GROUPS`               bitset  requested groups of stats
  =======================================  ======  ==========================

内核响应内容

 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_HEADER`        | nested | reply header                   |
 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_SRC`           | u32    | source of statistics           |
 +-----------------------------------+--------+--------------------------------+
 | `ETHTOOL_A_STATS_GRP`           | nested | one or more group of stats     |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_ID`      | u32    | group ID - `ETHTOOL_STATS_*` |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_SS_ID`   | u32    | string set ID for names        |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_STAT`    | nested | nest containing a statistic    |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_HIST_RX` | nested | histogram statistic (Rx)       |
 +-+---------------------------------+--------+--------------------------------+
 | | `ETHTOOL_A_STATS_GRP_HIST_TX` | nested | histogram statistic (Tx)       |
 +-+---------------------------------+--------+--------------------------------+

用户通过 `ETHTOOL_A_STATS_GROUPS` 位集指定他们请求的统计分组。当前已定义的值如下：

 ====================== ======== ===============================================
 ETHTOOL_STATS_ETH_MAC  eth-mac  Basic IEEE 802.3 MAC statistics (30.3.1.1.*)
 ETHTOOL_STATS_ETH_PHY  eth-phy  Basic IEEE 802.3 PHY statistics (30.3.2.1.*)
 ETHTOOL_STATS_ETH_CTRL eth-ctrl Basic IEEE 802.3 MAC Ctrl statistics (30.3.3.*)
 ETHTOOL_STATS_RMON     rmon     RMON (RFC 2819) statistics
 ETHTOOL_STATS_PHY      phy      Additional PHY statistics, not defined by IEEE
 ====================== ======== ===============================================

每个分组应在回复中有一个对应的 `ETHTOOL_A_STATS_GRP`。`ETHTOOL_A_STATS_GRP_ID` 标识
分组的统计嵌套包含了什么。`ETHTOOL_A_STATS_GRP_SS_ID` 标识分组内统计名称的字符串集 ID
（若可用）

统计被添加到 `ETHTOOL_A_STATS_GRP` 嵌套下的 `ETHTOOL_A_STATS_GRP_STAT`
`ETHTOOL_A_STATS_GRP_STAT` 内部应包含一8 字节（u64）属性——该属性的类型即为统计 ID
值为该统计的值。每个分组对统计 ID 有自己的解释。属ID 对应于由 `ETHTOOL_A_STATS_GRP_SS_ID`
标识的字符串集中的字符串。复杂统计（例如 RMON 直方图条目）也列`ETHTOOL_A_STATS_GRP`
内，且未在字符串集中定义字符串

RMON “直方图”计数器统计给定大小范围内的包数量。由RFC 未规定超出标1518 MTU 的范围，
各设备对桶的定义有所不同。因此包范围的定义交由各驱动决定

`ETHTOOL_A_STATS_GRP_HIST_RX` `ETHTOOL_A_STATS_GRP_HIST_TX` 嵌套包含以下属性：

 ================================= ====== ===================================
 ETHTOOL_A_STATS_RMON_HIST_BKT_LOW u32    low bound of the packet size bucket
 ETHTOOL_A_STATS_RMON_HIST_BKT_HI  u32    high bound of the bucket
 ETHTOOL_A_STATS_RMON_HIST_VAL     u64    packet counter
 ================================= ====== ===================================

下界与上界均为含边界，例如：

 ============================= ==== ====
 RFC statistic                 low  high
 ============================= ==== ====
 etherStatsPkts64Octets          0    64
 etherStatsPkts512to1023Octets 512  1023
 ============================= ==== ====

`ETHTOOL_A_STATS_SRC` 是可选的。与 `PAUSE_GET` 类似，它取值自 `enum ethtool_mac_stats_src`
若请求中缺省，则响应中会带有一个等`ETHTOOL_MAC_STATS_SRC_AGGREGATE` 
`ETHTOOL_A_STATS_SRC` 属性来提供统计信息

## PHC_VCLOCKS_GET


查询设备 PHC 虚拟时钟信息

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PHC_VCLOCKS_HEADER`      nested  request header
  ====================================  ======  ==========================

内核响应内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PHC_VCLOCKS_HEADER`      nested  reply header
  `ETHTOOL_A_PHC_VCLOCKS_NUM`         u32     PHC virtual clocks number
  `ETHTOOL_A_PHC_VCLOCKS_INDEX`       s32     PHC index array
  ====================================  ======  ==========================

## MODULE_GET


获取收发器模块参数

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`            nested  request header
  =====================================  ======  ==========================

内核响应内容

  ======================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`             nested  reply header
  `ETHTOOL_A_MODULE_POWER_MODE_POLICY`  u8      power mode policy
  `ETHTOOL_A_MODULE_POWER_MODE`         u8      operational power mode
  ======================================  ======  ==========================

可选的 `ETHTOOL_A_MODULE_POWER_MODE_POLICY` 属性编码了由主机强制执行的收发器模块电源模
策略。默认策略取决于驱动，但“auto”是推荐的默认值，新驱动以及不要求兼容旧行为的驱动都应
实现它

可选的 `ETHTOOL_A_MODULE_POWER_MODE` 属性编码了收发器模块的操作电源模式策略。它仅在插入
模块时才被上报。可能的取值为

    :identifiers: ethtool_module_power_mode

## MODULE_SET


设置收发器模块参数

请求内容

  ======================================  ======  ==========================
  `ETHTOOL_A_MODULE_HEADER`             nested  request header
  `ETHTOOL_A_MODULE_POWER_MODE_POLICY`  u8      power mode policy
  ======================================  ======  ==========================

设置时，可选的 `ETHTOOL_A_MODULE_POWER_MODE_POLICY` 属性用于设置由主机强制执行的收发器
模块电源策略。可能的取值为

    :identifiers: ethtool_module_power_mode_policy

对于 SFF-8636 模块，低功耗模式由主机根据规范 2.10a 修订版的6-10 强制设置

对于 CMIS 模块，低功耗模式由主机根据规范 5.0 修订版的6-12 强制设置

## PSE_GET


获取 PSE 属性

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PSE_HEADER`               nested  request header
  =====================================  ======  ==========================

内核响应内容

  ==========================================  ======  =============================
  `ETHTOOL_A_PSE_HEADER`                    nested  reply header
  `ETHTOOL_A_PODL_PSE_ADMIN_STATE`             u32  PoDL PSE 功能的操作状
  `ETHTOOL_A_PODL_PSE_PW_D_STATUS`             u32  PoDL PSE 的供电检测状
  `ETHTOOL_A_C33_PSE_ADMIN_STATE`              u32  PoE PSE 功能的操作状
  `ETHTOOL_A_C33_PSE_PW_D_STATUS`              u32  PoE PSE 的供电检测状
  `ETHTOOL_A_C33_PSE_PW_CLASS`                 u32  PoE PSE 的功率等
  `ETHTOOL_A_C33_PSE_ACTUAL_PW`                u32  PoE PSE 上实际消耗的功率
  `ETHTOOL_A_C33_PSE_EXT_STATE`                u32  PoE PSE 的扩展错误状
  `ETHTOOL_A_C33_PSE_EXT_SUBSTATE`             u32  PoE PSE 的扩展错误子状
  `ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT`           u32  PoE PSE 当前配置的功率限
  `ETHTOOL_A_C33_PSE_PW_LIMIT_RANGES`       nested  支持的功率限制配置范
  `ETHTOOL_A_PSE_PW_D_ID`                      u32  PSE 供电域的索引
  `ETHTOOL_A_PSE_PRIO_MAX`                     u32  PoE PSE 上可配置的最大优先级
  `ETHTOOL_A_PSE_PRIO`                         u32  PoE PSE 当前配置的优先级
  ==========================================  ======  =============================

设置时，可选的 `ETHTOOL_A_PODL_PSE_ADMIN_STATE` 属性标PoDL PSE 功能的操作状态。PSE
功能的操作状态可使用 `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL` 动作更改。该属性对应于
`IEEE 802.3-2018` 30.15.1.1.2 aPoDLPSEAdminState。可能的取值为

    :identifiers: ethtool_podl_pse_admin_state

`ETHTOOL_A_C33_PSE_ADMIN_STATE` 同理，实现了 `IEEE 802.3-2022` 30.9.1.1.2
aPSEAdminState銆。

    :identifiers: ethtool_c33_pse_admin_state

设置时，可选的 `ETHTOOL_A_PODL_PSE_PW_D_STATUS` 属性标PoDL PSE 的供电检测状态。该状
取决于内PSE 状态机与自PD 分类支持情况。该属性对应于 `IEEE 802.3-2018`
30.15.1.1.3 aPoDLPSEPowerDetectionStatus。可能的取值为

    :identifiers: ethtool_podl_pse_pw_d_status

`ETHTOOL_A_C33_PSE_ADMIN_PW_D_STATUS` 同理，实现了 `IEEE 802.3-2022` 30.9.1.1.5
aPSEPowerDetectionStatus銆。

    :identifiers: ethtool_c33_pse_pw_d_status

设置时，可选的 `ETHTOOL_A_C33_PSE_PW_CLASS` 属性标C33 PSE 的功率等级。它取决PSE 
PD 之间协商得到的等级。该属性对应于 `IEEE 802.3-2022` 30.9.1.1.8 aPSEPowerClassification

设置时，可选的 `ETHTOOL_A_C33_PSE_ACTUAL_PW` 属性标C33 PSE 实际消耗的功率。该属性对应于
`IEEE 802.3-2022` 30.9.1.1.23 aPSEActualPower。实际功率以 mW 报告

设置时，可选的 `ETHTOOL_A_C33_PSE_EXT_STATE` 属性标C33 PSE 的扩展错误状态。可能的取值为

    :identifiers: ethtool_c33_pse_ext_state

设置时，可选的 `ETHTOOL_A_C33_PSE_EXT_SUBSTATE` 属性标C33 PSE 的扩展错误子状态。可能的
取值为

    :identifiers: ethtool_c33_pse_ext_substate_class_num_events
		  ethtool_c33_pse_ext_substate_error_condition
		  ethtool_c33_pse_ext_substate_mr_pse_enable
		  ethtool_c33_pse_ext_substate_option_detect_ted
		  ethtool_c33_pse_ext_substate_option_vport_lim
		  ethtool_c33_pse_ext_substate_ovld_detected
		  ethtool_c33_pse_ext_substate_pd_dll_power_type
		  ethtool_c33_pse_ext_substate_power_not_available
		  ethtool_c33_pse_ext_substate_short_detected

设置时，可选的 `ETHTOOL_A_C33_PSE_AVAIL_PW_LIMIT` 属性标识以 mW 为单位的 C33 PSE 功率限制

设置时，可选的 `ETHTOOL_A_C33_PSE_PW_LIMIT_RANGES` 嵌套属性通过
`ETHTOOL_A_C33_PSE_PWR_VAL_LIMIT_RANGE_MIN` 涓?`ETHTOOL_A_C33_PSE_PWR_VAL_LIMIT_RANGE_MAX`
标识 C33 PSE 功率限制范围。若控制器以固定等级工作，最小值与最大值将相等

`ETHTOOL_A_PSE_PW_D_ID` 属性标PSE 供电域的索引

设置时，可选的 `ETHTOOL_A_PSE_PRIO_MAX` 属性标PSE 最大优先级值。设置时，可选的
`ETHTOOL_A_PSE_PRIO` 属性用于标识当前配置的 PSE 优先级。有PSE 优先级属性的说明，参
`PSE_SET`銆。

## PSE_SET


设置 PSE 参数

请求内容

  ======================================  ======  =============================
  `ETHTOOL_A_PSE_HEADER`                nested  request header
  `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL`       u32  Control PoDL PSE Admin state
  `ETHTOOL_A_C33_PSE_ADMIN_CONTROL`        u32  Control PSE Admin state
  `ETHTOOL_A_C33_PSE_AVAIL_PWR_LIMIT`      u32  Control PoE PSE available
                                                  power limit
  `ETHTOOL_A_PSE_PRIO`                     u32  Control priority of the
                                                  PoE PSE
  ======================================  ======  =============================

设置时，可选的 `ETHTOOL_A_PODL_PSE_ADMIN_CONTROL` 属性用于控PoDL PSE 管理功能。该选项
实现`IEEE 802.3-2018` 30.15.1.2.1 acPoDLPSEAdminControl。支持的取值参
`ETHTOOL_A_PODL_PSE_ADMIN_STATE`銆。

`ETHTOOL_A_C33_PSE_ADMIN_CONTROL` 同理，实现了 `IEEE 802.3-2022` 30.9.1.2.1
acPSEAdminControl銆。

设置时，可选的 `ETHTOOL_A_C33_PSE_AVAIL_PWR_LIMIT` 属性用于控C33 PSE 以毫瓦为单位的可
功率值限制。该属性对应于 `IEEE 802.3-2022` 33.2.4.4 变量145.2.5.4 变量中描述的
`pse_available_power` 变量`pse_avail_pwr` 变量，二者以功率等级描述

决定在本接口中使用毫瓦，是为了与其它同样使用毫瓦的功率监控接口统一，并与各类以瓦（而非
等级）记录功耗的现有产品保持一致。如果确实需要基于等级的功率限制配置，可以在用户空间进行
转换，例如通过 ethtool

设置时，可选的 `ETHTOOL_A_PSE_PRIO` 属性用于控PSE 优先级。允许的优先级取值介0 
`ETHTOOL_A_PSE_PRIO_MAX` 属性值之间

较小的值表示更高的优先级，即优先级值为 0 对应最高端口优先级。端口优先级有两个作用：

 - 上电顺序：复位后，端口按其优先级从高到低依次上电。优先级更高（值更小）的端口先上电
 - 关闭顺序：当功率预算超限时，优先级更低（值更大）的端口先被关闭

## PSE_NTF


通知 PSE 事件

通知内容

  ===============================  ======  ========================
  `ETHTOOL_A_PSE_HEADER`         nested  request header
  `ETHTOOL_A_PSE_EVENTS`         bitset  PSE events
  ===============================  ======  ========================

设置时，可选的 `ETHTOOL_A_PSE_EVENTS` 属性标PSE 事件

    :identifiers: ethtool_pse_event

## RSS_GET


获取与接口某RSS 上下文相关的间接表、哈希密钥与哈希函数信息，类似于 `ETHTOOL_GRSSH`
ioctl 请求

请求内容

=====================================  ======  ============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_START_CONTEXT`      u32     start context number (dumps)
=====================================  ======  ============================

`ETHTOOL_A_RSS_CONTEXT` 指定要查询的 RSS 上下文编号；若未设置，则查询上下0（主上下文）
dump 可以按设备过滤（只列出给netdev 的上下文）。不支持过滤单个上下文编号，但可以使
`ETHTOOL_A_RSS_START_CONTEXT` 从该编号开dump 上下文（主要用于忽略上下0、只 dump
额外的上下文）

内核响应内容

=====================================  ======  ===============================
  `ETHTOOL_A_RSS_HEADER`             nested  reply header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
  `ETHTOOL_A_RSS_FLOW_HASH`          nested  Header fields included in hash
=====================================  ======  ===============================

ETHTOOL_A_RSS_HFUNC 属性是指示所用哈希函数的位图。当前支持的选项toeplitz、xor crc32
ETHTOOL_A_RSS_INDIR 属性返RSS 间接表，其中每个字节表示一个队列编号
ETHTOOL_A_RSS_INPUT_XFRM 属性是一个位图，指示在送给 RSS hfunc 之前对输入协议字段所应用
转换类型。当前支持的选项symmetric-xor symmetric-or-xor
ETHTOOL_A_RSS_FLOW_HASH 携带每个流类型的位掩码，指示哪些头字段被包含在哈希计算中

## RSS_SET


请求内容

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
  `ETHTOOL_A_RSS_FLOW_HASH`          nested  Header fields included in hash
=====================================  ======  ==============================

`ETHTOOL_A_RSS_INDIR` 是用户期望的最RSS 表。若其小于设备支持的最小表大小，内核与设备
驱动可能会复制该表。例如，若用户请`[0, 1]`，但设备至少需8 个条目，则实际使用的表将
变为 `[0, 1, 0, 1, 0, 1, 0, 1]`。大多数设备要求表大小为 2 的幂，因此大小不2 的幂的表
很可能被拒绝。使用大小为 0 的表会将间接表重置为默认值

## RSS_CREATE_ACT


请求内容

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
  `ETHTOOL_A_RSS_HFUNC`              u32     RSS hash func
  `ETHTOOL_A_RSS_INDIR`              binary  Indir table bytes
  `ETHTOOL_A_RSS_HKEY`               binary  Hash key bytes
  `ETHTOOL_A_RSS_INPUT_XFRM`         u32     RSS input data transformation
=====================================  ======  ==============================

内核响应内容

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
=====================================  ======  ==============================

创建一个额外的 RSS 上下文；若未指定 `ETHTOOL_A_RSS_CONTEXT`，内核将自动分配一个

## RSS_DELETE_ACT


请求内容

=====================================  ======  ==============================
  `ETHTOOL_A_RSS_HEADER`             nested  request header
  `ETHTOOL_A_RSS_CONTEXT`            u32     context number
=====================================  ======  ==============================

删除一个额外的 RSS 上下文

## PLCA_GET_CFG


获取 IEEE 802.3cg-2019 148 条物理层冲突避免（PLCA）协调子层（RS）属性

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PLCA_HEADER`              nested  request header
  =====================================  ======  ==========================

内核响应内容

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  reply header
  `ETHTOOL_A_PLCA_VERSION`              u16     Supported PLCA management
                                                  interface standard/version
  `ETHTOOL_A_PLCA_ENABLED`              u8      PLCA Admin State
  `ETHTOOL_A_PLCA_NODE_ID`              u32     PLCA unique local node ID
  `ETHTOOL_A_PLCA_NODE_CNT`             u32     Number of PLCA nodes on the
                                                  network, including the
                                                  coordinator
  `ETHTOOL_A_PLCA_TO_TMR`               u32     Transmit Opportunity Timer
                                                  value in bit-times (BT)
  `ETHTOOL_A_PLCA_BURST_CNT`            u32     Number of additional packets
                                                  the node is allowed to send
                                                  within a single TO
  `ETHTOOL_A_PLCA_BURST_TMR`            u32     Time to wait for the MAC to
                                                  transmit a new frame before
                                                  terminating the burst
  ======================================  ======  =============================

设置时，可选的 `ETHTOOL_A_PLCA_VERSION` 属性指PLCA 管理接口所符合的标准与版本。若未设置，
该接口为供应商特定的，并（可能）由驱动提供。OPEN Alliance SIG 为内PLCA 协调子层
10BASE-T1S PHY 规定了标准寄存器映射。参https://www.opensig.org/about/specifications/ 上的
鈥?0BASE-T1S PLCA Management Registers鈥濄€?

设置时，可选的 `ETHTOOL_A_PLCA_ENABLED` 属性指PLCA RS 的管理状态。若未设置，节点运行
“plainCSMA/CD 模式下。该选项对应`IEEE 802.3cg-2019` 30.16.1.1.1
aPLCAAdminState / 30.16.1.2.1 acPLCAAdminControl銆。

设置时，可选的 `ETHTOOL_A_PLCA_NODE_ID` 属性指PHY 配置好的本地节点 ID。该 ID 决定了为
节点预留用于发送的发送机会（TO）。该选项对应`IEEE 802.3cg-2019` 30.16.1.1.4
aPLCALocalNodeID。该属性的有效范围[0 .. 255]，其255 表示“未配置”

设置时，可选的 `ETHTOOL_A_PLCA_NODE_CNT` 属性指示混合段上配置的 PLCA 节点最大数量。该数字
决定了在一PLCA 周期内生成的发送机会总数。该属性仅PLCA 协调器（aPLCALocalNodeID
设为 0 的节点）相关，跟随节点忽略此设置。该选项对应`IEEE 802.3cg-2019` 30.16.1.1.3
aPLCANodeCount。该属性的有效范围[1 .. 255]

设置时，可选的 `ETHTOOL_A_PLCA_TO_TMR` 属性指示以位时间（bit-times）为单位的发送机会定时器
配置值。为了让 PLCA 正常工作，共享介质的所有节点此值必须设为相等。该选项对应
`IEEE 802.3cg-2019` 30.16.1.1.5 aPLCATransmitOpportunityTimer。该属性的有效范围[0 .. 255]

设置时，可选的 `ETHTOOL_A_PLCA_BURST_CNT` 属性指示节点在单个发送机会内允许发送的额外包数量
默认情况下该属性为 0，表示节点每TO 只能发送单个帧。当大于 0 时，PLCA RS 会在任意发送后
保持TO，等MAC 在最aPLCABurstTimer 个位时间内发送新帧。在一PLCA 周期内这种情
最多发生本参数所指定次数，之后突发结束，正常TO 计数恢复。该选项对应`IEEE 802.3cg-2019`
30.16.1.1.6 aPLCAMaxBurstCount。该属性的有效范围[0 .. 255]

设置时，可选的 `ETHTOOL_A_PLCA_BURST_TMR` 属性指示当 aPLCAMaxBurstCount 大于 0 时，PLCA RS
等待 MAC 发起新发送的位时间数。若 MAC 在此时间内未能发送新帧，突发结束，TO 计数恢复。否则，
新帧作为当前突发的一部分被发送。该选项对应`IEEE 802.3cg-2019` 30.16.1.1.7
aPLCABurstTimer。该属性的有效范围[0 .. 255]。不过，为了PLCA 突发模式按预期工作，该
应设置为大于 MAC 的帧间间隔（IFG）时间（并留有一定余量）

## PLCA_SET_CFG


设置 PLCA RS 参数

请求内容

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  request header
  `ETHTOOL_A_PLCA_ENABLED`              u8      PLCA Admin State
  `ETHTOOL_A_PLCA_NODE_ID`              u8      PLCA unique local node ID
  `ETHTOOL_A_PLCA_NODE_CNT`             u8      Number of PLCA nodes on the
                                                  network, including the
                                                  coordinator
  `ETHTOOL_A_PLCA_TO_TMR`               u8      Transmit Opportunity Timer
                                                  value in bit-times (BT)
  `ETHTOOL_A_PLCA_BURST_CNT`            u8      Number of additional packets
                                                  the node is allowed to send
                                                  within a single TO
  `ETHTOOL_A_PLCA_BURST_TMR`            u8      Time to wait for the MAC to
                                                  transmit a new frame before
                                                  terminating the burst
  ======================================  ======  =============================

各属性的说明参见 `PLCA_GET_CFG`

## PLCA_GET_STATUS


获取 PLCA RS 状态信息

请求内容

  =====================================  ======  ==========================
  `ETHTOOL_A_PLCA_HEADER`              nested  request header
  =====================================  ======  ==========================

内核响应内容

  ======================================  ======  =============================
  `ETHTOOL_A_PLCA_HEADER`               nested  reply header
  `ETHTOOL_A_PLCA_STATUS`               u8      PLCA RS operational status
  ======================================  ======  =============================

设置时，`ETHTOOL_A_PLCA_STATUS` 属性指示节点是否检测到网络BEACON 的存在。该标志对应
`IEEE 802.3cg-2019` 30.16.1.1.2 aPLCAStatus銆。

## MM_GET


获取 802.3 MAC 合并（MAC Merge）参数

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_MM_HEADER`               nested  request header
  ====================================  ======  ==========================

内核响应内容

  =================================  ======  ===================================
  `ETHTOOL_A_MM_HEADER`            nested  request header
  `ETHTOOL_A_MM_PMAC_ENABLED`      bool    若启用可抢占帧与 SMD-V 帧的接收则置
  `ETHTOOL_A_MM_TX_ENABLED`        bool    若管理上启用了可抢占帧的发送则置位
                                            （若验证失败可能未激活）
  `ETHTOOL_A_MM_TX_ACTIVE`         bool    若操作上启用了可抢占帧的发送则置位
  `ETHTOOL_A_MM_TX_MIN_FRAG_SIZE`  u32     发送的非末尾片段的最小大小，以八位组
  `ETHTOOL_A_MM_RX_MIN_FRAG_SIZE`  u32     接收的非末尾片段的最小大小，以八位组
  `ETHTOOL_A_MM_VERIFY_ENABLED`    bool    若管理上启用SMD-V 帧的发送则置位
  `ETHTOOL_A_MM_VERIFY_STATUS`     u8      verification 功能的状
  `ETHTOOL_A_MM_VERIFY_TIME`       u32     两次验证尝试之间的延
  `ETHTOOL_A_MM_MAX_VERIFY_TIME``  u32     maximum verification interval
                                             supported by device
  `ETHTOOL_A_MM_STATS`             nested  IEEE 802.3-2018 子条30.14.1
                                             oMACMergeEntity 统计计数
  =================================  ======  ===================================

这些属性由设备驱动通过以下结构填充

    :identifiers: ethtool_mm_state

`ETHTOOL_A_MM_VERIFY_STATUS` 将报告来自以下取值之一

    :identifiers: ethtool_mm_verify_status

若在 `MM_SET` 命令`ETHTOOL_A_MM_VERIFY_ENABLED` false 传入，则
`ETHTOOL_A_MM_VERIFY_STATUS` 将报`ETHTOOL_MM_VERIFY_STATUS_INITIAL` 
`ETHTOOL_MM_VERIFY_STATUS_DISABLED`，否则应报告其它某个状态

建议驱动pMAC 禁用状态启动，并在用户空间请求时启用它。同时建议用户空间不要依
`ETHTOOL_MSG_MM_GET` 请求的默认值

`ETHTOOL_A_HEADER_FLAGS` 中设置了 `ETHTOOL_FLAG_STATS`，则会报`ETHTOOL_A_MM_STATS`
如果驱动未报告任何统计信息，该属性将为空。驱动在以下结构中填写统计信息：

    :identifiers: ethtool_mm_stats

## MM_SET


修改 802.3 MAC 合并层的配置

请求内容

  =================================  ======  ==========================
  `ETHTOOL_A_MM_VERIFY_TIME`       u32     see MM_GET description
  `ETHTOOL_A_MM_VERIFY_ENABLED`    bool    see MM_GET description
  `ETHTOOL_A_MM_TX_ENABLED`        bool    see MM_GET description
  `ETHTOOL_A_MM_PMAC_ENABLED`      bool    see MM_GET description
  `ETHTOOL_A_MM_TX_MIN_FRAG_SIZE`  u32     see MM_GET description
  =================================  ======  ==========================

这些属性通过以下结构传播给驱动：

    :identifiers: ethtool_mm_cfg

## MODULE_FW_FLASH_ACT


烧录收发器模块固件

请求内容

  =======================================  ======  ===========================
  `ETHTOOL_A_MODULE_FW_FLASH_HEADER`     nested  request header
  `ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME`  string  firmware image file name
  `ETHTOOL_A_MODULE_FW_FLASH_PASSWORD`   u32     transceiver module password
  =======================================  ===========================

固件更新过程由三个逻辑步骤组成

1. 将固件映像下载到收发器模块并校验它
2. 运行固件映像
3. 提交固件映像，使其在复位后运行

给定烧录命令后，这三个步骤按顺序执行

该消息仅调度更新过程并立即返回，不会阻塞。随后该过程异步运行。由于完成可能需要数分钟
在更新过程中内核会向用户空间发出通知，更新其状态与进度

`ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME` 属性编码固件映像文件名。固件映像被下载到收发器模块
校验、运行并提交

可选的 `ETHTOOL_A_MODULE_FW_FLASH_PASSWORD` 属性编码一个密码，该密码可能作为收发器模块
固件更新过程的一部分被需要

固件更新过程可能需要数分钟才能完成。因此，在更新过程中内核会向用户空间发出通知，更新其
状态与进度


通知内容

 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_HEADER`              | nested | reply header   |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_STATUS`              | u32    | status         |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_STATUS_MSG`          | string | status message |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_DONE`                | uint   | progress       |
 +---------------------------------------------------+--------+----------------+
 | `ETHTOOL_A_MODULE_FW_FLASH_TOTAL`               | uint   | total          |
 +---------------------------------------------------+--------+----------------+

`ETHTOOL_A_MODULE_FW_FLASH_STATUS` 属性编码固件更新过程的当前状态。可能的取值为

    :identifiers: ethtool_module_fw_flash_status

`ETHTOOL_A_MODULE_FW_FLASH_STATUS_MSG` 属性编码状态消息字符串

`ETHTOOL_A_MODULE_FW_FLASH_DONE` `ETHTOOL_A_MODULE_FW_FLASH_TOTAL` 属性分别编码已完成
总的工作量

## PHY_GET


获取链路上给定以太网 PHY 的信息。DO 操作返回关于 dev->phydev 的所有可用信息。用户也可以
指定 PHY_INDEX，此DO 请求返回关于该特PHY 的信息

由于可能存在多于一PHY，可以使DUMP 操作，通过dump 请求中传入接口索引或名称，来
列出给定接口上存在的 PHY

更多信息参见 phy_link_topology

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_PHY_HEADER`              nested  request header
  ====================================  ======  ==========================

内核响应内容

  ===================================== ======  ===============================
  `ETHTOOL_A_PHY_HEADER`              nested  request header
  `ETHTOOL_A_PHY_INDEX`               u32     phy 的唯一索引，可用于针对phy 的请
  `ETHTOOL_A_PHY_DRVNAME`             string  phy 的驱动名
  `ETHTOOL_A_PHY_NAME`                string  phy 的设备名
  `ETHTOOL_A_PHY_UPSTREAM_TYPE`       u32     phy 所连接设备的类
  `ETHTOOL_A_PHY_UPSTREAM_INDEX`      u32     上游 PHY PHY 索引
  `ETHTOOL_A_PHY_UPSTREAM_SFP_NAME`   string  若该 PHY 通过 SFP 总线连接到其PHY，该 sfp 总线的名
  `ETHTOOL_A_PHY_DOWNSTREAM_SFP_NAME` string  若该 phy 控制一sfp 总线，该 sfp 总线的名
  ===================================== ======  ===============================

`ETHTOOL_A_PHY_UPSTREAM_TYPE` PHY_UPSTREAM_PHY 时，PHY 的父级是另一PHY

## TSCONFIG_GET


获取当前硬件时间戳源与配置的相关信息

它类似于已废弃的 `SIOCGHWTSTAMP` ioctl 请求

请求内容

  ====================================  ======  ==========================
  `ETHTOOL_A_TSCONFIG_HEADER`         nested  request header
  ====================================  ======  ==========================

内核响应内容

  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

设置时，`ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` 属性标识硬件时间戳提供者的来源。它由描PTP
设备索引`ETHTOOL_A_TS_HWTSTAMP_PROVIDER_INDEX` 属性，以及描述时间戳限定符
`ETHTOOL_A_TS_HWTSTAMP_PROVIDER_QUALIFIER` 属性组成

设置时，`ETHTOOL_A_TSCONFIG_TX_TYPES`、`ETHTOOL_A_TSCONFIG_RX_FILTERS` 
`ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS` 属性标识当前硬件时间戳提供者所配置Tx 类型、Rx 过滤
与标志。这些属性通过以下结构传播给驱动：

    :identifiers: kernel_hwtstamp_config

## TSCONFIG_SET


设置当前硬件时间戳源与配置的相关信息

它类似于已废弃的 `SIOCSHWTSTAMP` ioctl 请求

请求内容


  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

内核响应内容

  ======================================== ======  ============================
  `ETHTOOL_A_TSCONFIG_HEADER`            nested  request header
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_PROVIDER` nested  PTP hw clock provider
  `ETHTOOL_A_TSCONFIG_TX_TYPES`          bitset  hwtstamp Tx type
  `ETHTOOL_A_TSCONFIG_RX_FILTERS`        bitset  hwtstamp Rx filter
  `ETHTOOL_A_TSCONFIG_HWTSTAMP_FLAGS`	   u32     hwtstamp flags
  ======================================== ======  ============================

各属性的说明参见 `TSCONFIG_GET`

## MSE_GET


PHY 获取详细的平均平方误差（Mean Square Error，MSE）诊断信息

请求内容

  ====================================  ======  ============================
  `ETHTOOL_A_MSE_HEADER`              nested  request header
  ====================================  ======  ============================

内核响应内容

  ====================================  ======  ================================
  `ETHTOOL_A_MSE_HEADER`              nested  reply header
  `ETHTOOL_A_MSE_CAPABILITIES`        nested  MSE 测量的能比例信息
  `ETHTOOL_A_MSE_CHANNEL_A`           nested  Channel A 的快
  `ETHTOOL_A_MSE_CHANNEL_B`           nested  Channel B 的快
  `ETHTOOL_A_MSE_CHANNEL_C`           nested  Channel C 的快
  `ETHTOOL_A_MSE_CHANNEL_D`           nested  Channel D 的快
  `ETHTOOL_A_MSE_WORST_CHANNEL`       nested  最差通道的快
  `ETHTOOL_A_MSE_LINK`                nested  链路级聚合的快照
  ====================================  ======  ================================

### MSE 能力


这个嵌套属性报告用于解释快照值的能力 / 缩放属性

  ============================================== ======  =========================
  `ETHTOOL_A_MSE_CAPABILITIES_MAX_AVERAGE_MSE` uint    最avg_mse 比例
  `ETHTOOL_A_MSE_CAPABILITIES_MAX_PEAK_MSE`    uint    最peak_mse 比例
  `ETHTOOL_A_MSE_CAPABILITIES_REFRESH_RATE_PS` uint    采样率（皮秒
  `ETHTOOL_A_MSE_CAPABILITIES_NUM_SYMBOLS`     uint    每个硬件采样的符号数
  ============================================== ======  =========================

max-average/peak 字段仅在 PHY 支持相应指标时才包含。它们的缺失表示该指标不可用

参见 `include/linux/phy.h` `struct phy_mse_capability` 的内核文档

### MSE 快照


每个每通道嵌套包含该选择器（通道 A/B/C/D、最差通道或链路）MSE 值的原子快照

  ==========================================  ======  ===================
  `ETHTOOL_A_MSE_SNAPSHOT_AVERAGE_MSE`      uint    骞冲潎 MSE 鍊。
  `ETHTOOL_A_MSE_SNAPSHOT_PEAK_MSE`         uint    当前峰MSE
  `ETHTOOL_A_MSE_SNAPSHOT_WORST_PEAK_MSE`   uint    最坏情况峰MSE
  ==========================================  ======  ===================

在每个通道嵌套中，仅会出现 PHY 所支持的指标

参见 `include/linux/phy.h` `struct phy_mse_snapshot` 的内核文档

## 请求翻译


下表ioctl 命令映射到提供其功能netlink 命令。右列为“n/a”的条目是尚netlink 替代
的命令。左列为“n/a”的条目则仅存在netlink

  =================================== =====================================
  ioctl command                       netlink command
  =================================== =====================================
  `ETHTOOL_GSET`                    `ETHTOOL_MSG_LINKINFO_GET`
                                      `ETHTOOL_MSG_LINKMODES_GET`
  `ETHTOOL_SSET`                    `ETHTOOL_MSG_LINKINFO_SET`
                                      `ETHTOOL_MSG_LINKMODES_SET`
  `ETHTOOL_GDRVINFO`                n/a
  `ETHTOOL_GREGS`                   n/a
  `ETHTOOL_GWOL`                    `ETHTOOL_MSG_WOL_GET`
  `ETHTOOL_SWOL`                    `ETHTOOL_MSG_WOL_SET`
  `ETHTOOL_GMSGLVL`                 `ETHTOOL_MSG_DEBUG_GET`
  `ETHTOOL_SMSGLVL`                 `ETHTOOL_MSG_DEBUG_SET`
  `ETHTOOL_NWAY_RST`                n/a
  `ETHTOOL_GLINK`                   `ETHTOOL_MSG_LINKSTATE_GET`
  `ETHTOOL_GEEPROM`                 n/a
  `ETHTOOL_SEEPROM`                 n/a
  `ETHTOOL_GCOALESCE`               `ETHTOOL_MSG_COALESCE_GET`
  `ETHTOOL_SCOALESCE`               `ETHTOOL_MSG_COALESCE_SET`
  `ETHTOOL_GRINGPARAM`              `ETHTOOL_MSG_RINGS_GET`
  `ETHTOOL_SRINGPARAM`              `ETHTOOL_MSG_RINGS_SET`
  `ETHTOOL_GPAUSEPARAM`             `ETHTOOL_MSG_PAUSE_GET`
  `ETHTOOL_SPAUSEPARAM`             `ETHTOOL_MSG_PAUSE_SET`
  `ETHTOOL_GRXCSUM`                 `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SRXCSUM`                 `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GTXCSUM`                 `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_STXCSUM`                 `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GSG`                     `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SSG`                     `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_TEST`                    n/a
  `ETHTOOL_GSTRINGS`                `ETHTOOL_MSG_STRSET_GET`
  `ETHTOOL_PHYS_ID`                 n/a
  `ETHTOOL_GSTATS`                  n/a
  `ETHTOOL_GTSO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_STSO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GPERMADDR`               rtnetlink `RTM_GETLINK`
  `ETHTOOL_GUFO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SUFO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GGSO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SGSO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GFLAGS`                  `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SFLAGS`                  `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GPFLAGS`                 `ETHTOOL_MSG_PRIVFLAGS_GET`
  `ETHTOOL_SPFLAGS`                 `ETHTOOL_MSG_PRIVFLAGS_SET`
  `ETHTOOL_GRXFH`                   `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRXFH`                   `ETHTOOL_MSG_RSS_SET`
  `ETHTOOL_GGRO`                    `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SGRO`                    `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GRXRINGS`                n/a
  `ETHTOOL_GRXCLSRLCNT`             n/a
  `ETHTOOL_GRXCLSRULE`              n/a
  `ETHTOOL_GRXCLSRLALL`             n/a
  `ETHTOOL_SRXCLSRLDEL`             n/a
  `ETHTOOL_SRXCLSRLINS`             n/a
  `ETHTOOL_FLASHDEV`                n/a
  `ETHTOOL_RESET`                   n/a
  `ETHTOOL_SRXNTUPLE`               n/a
  `ETHTOOL_GRXNTUPLE`               n/a
  `ETHTOOL_GSSET_INFO`              `ETHTOOL_MSG_STRSET_GET`
  `ETHTOOL_GRXFHINDIR`              `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRXFHINDIR`              `ETHTOOL_MSG_RSS_SET`
  `ETHTOOL_GFEATURES`               `ETHTOOL_MSG_FEATURES_GET`
  `ETHTOOL_SFEATURES`               `ETHTOOL_MSG_FEATURES_SET`
  `ETHTOOL_GCHANNELS`               `ETHTOOL_MSG_CHANNELS_GET`
  `ETHTOOL_SCHANNELS`               `ETHTOOL_MSG_CHANNELS_SET`
  `ETHTOOL_SET_DUMP`                n/a
  `ETHTOOL_GET_DUMP_FLAG`           n/a
  `ETHTOOL_GET_DUMP_DATA`           n/a
  `ETHTOOL_GET_TS_INFO`             `ETHTOOL_MSG_TSINFO_GET`
  `ETHTOOL_GMODULEINFO`             `ETHTOOL_MSG_MODULE_EEPROM_GET`
  `ETHTOOL_GMODULEEEPROM`           `ETHTOOL_MSG_MODULE_EEPROM_GET`
  `ETHTOOL_GEEE`                    `ETHTOOL_MSG_EEE_GET`
  `ETHTOOL_SEEE`                    `ETHTOOL_MSG_EEE_SET`
  `ETHTOOL_GRSSH`                   `ETHTOOL_MSG_RSS_GET`
  `ETHTOOL_SRSSH`                   n/a
  `ETHTOOL_GTUNABLE`                n/a
  `ETHTOOL_STUNABLE`                n/a
  `ETHTOOL_GPHYSTATS`               n/a
  `ETHTOOL_PERQUEUE`                n/a
  `ETHTOOL_GLINKSETTINGS`           `ETHTOOL_MSG_LINKINFO_GET`
                                      `ETHTOOL_MSG_LINKMODES_GET`
  `ETHTOOL_SLINKSETTINGS`           `ETHTOOL_MSG_LINKINFO_SET`
                                      `ETHTOOL_MSG_LINKMODES_SET`
  `ETHTOOL_PHY_GTUNABLE`            n/a
  `ETHTOOL_PHY_STUNABLE`            n/a
  `ETHTOOL_GFECPARAM`               `ETHTOOL_MSG_FEC_GET`
  `ETHTOOL_SFECPARAM`               `ETHTOOL_MSG_FEC_SET`
  n/a                                 `ETHTOOL_MSG_CABLE_TEST_ACT`
  n/a                                 `ETHTOOL_MSG_CABLE_TEST_TDR_ACT`
  n/a                                 `ETHTOOL_MSG_TUNNEL_INFO_GET`
  n/a                                 `ETHTOOL_MSG_PHC_VCLOCKS_GET`
  n/a                                 `ETHTOOL_MSG_MODULE_GET`
  n/a                                 `ETHTOOL_MSG_MODULE_SET`
  n/a                                 `ETHTOOL_MSG_PLCA_GET_CFG`
  n/a                                 `ETHTOOL_MSG_PLCA_SET_CFG`
  n/a                                 `ETHTOOL_MSG_PLCA_GET_STATUS`
  n/a                                 `ETHTOOL_MSG_MM_GET`
  n/a                                 `ETHTOOL_MSG_MM_SET`
  n/a                                 `ETHTOOL_MSG_MODULE_FW_FLASH_ACT`
  n/a                                 `ETHTOOL_MSG_PHY_GET`
  `SIOCGHWTSTAMP`                   `ETHTOOL_MSG_TSCONFIG_GET`
  `SIOCSHWTSTAMP`                   `ETHTOOL_MSG_TSCONFIG_SET`
  =================================== =====================================
