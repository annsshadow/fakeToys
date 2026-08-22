
## Netlink 协议规范（以 YAML 描述

Netlink 协议规范是以 YAML 编写的、对 Netlink 协议完整且机器可读的描述。规范的
目标是将 Netlink 解析与用户空间逻辑分离，并最小化每个family、命令、属性所需手写 Netlink 代码量。Netlink 规范应当是完整的，且不依赖于任何其它规范C 头文件，
从而便于在无法直接包含内核头的语言中使用
内核内部使用 YAML 规范来生成：

 - C uAPI 头文 - ReST 文件形式呈现的协议文- 参见 Documentation/netlink/specs/index.rst <specs>
 - 用于输入属性验证的策略 - 操作
YAML 规范可在 `Documentation/netlink/specs/` 下找到
本文档描schema 的细节。参[intro-specs](intro-specs) 获取实用的入门指南
所有规范必须采`((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)`
许可，以便能在用户空间代码中轻松采用
## 兼容性级

Netlink 规范有四schema 级别，从family 使用的最简单级别，到覆盖所有旧 family
怪异行为的复杂级别。每个后续级别继承前一个级别的属性，这意味着能够处理更复`genetlink` schema 的实现，也与更简单的级别兼容。这些级别是
 - `genetlink` - 最精简，所有新 family 都应使用 - `genetlink-c` - `genetlink` 的超集，带有额外的属性，允许自定define    enum 类型的名称及取值名；对于所有不直接C uAPI 头文件交互的实现，该 schema
   应当`genetlink` 等价
 - `genetlink-legacy` - 支持所有旧 genetlink family 怪异行为、奇怪属性格式、二进制
   结构等的通用 Netlink 兜底 schema
 - `netlink-raw` - 支持 Generic Netlink 之前协议的兜schema，例`NETLINK_ROUTE`

schema 的定义（`jsonschema` 描述）可`Documentation/netlink/` 下找到
## Schema 结构


YAML schema 具有以下几个概念性部分：

 - globals
 - definitions
 - attributes
 - operations
 - multicast groups

schema 中的大多数属性接受（事实上要求）一`doc` 子属性来记录所定义的对象
以下各节描述最现代`genetlink` schema 的属性。有C 名称如何name 属性派生的
信息，请参见 [genetlink-c <c-code-gen>](genetlink-c <c-code-gen>)
另请参见 Documentation/core-api/netlink.rst <kernel_netlink>，了解仅与内核空间相关不属于用户空API Netlink 规范属性
## genetlink


### Globals


直接列在规范文件根级别的属性
#### name


family 的名称。由Family ID 是动态分配的，名称以唯一的方式标识该 family
#### protocol


schema 级别，默认是 `genetlink`，这也是`genetlink` family 唯一允许的取值
### definitions


类型与常量定义的数组
#### name


类型 / 常量的名称
#### type


以下类型之一
 - const - 单个独立的常 - enum - 定义一个整数枚举，每个条目的值递增 1（例0 - flags - 定义一个整数枚举，每个条目的值占据一个位，从0 开始（例如 1
#### value


`const` 的取值
#### value-start


`enum` `flags` 的起始值，允许覆盖默认的起始`0`（对 `enum`）和起始位（`flags`）。对 `flags`，`value-start` 选择起始位，而非移位后的值
不支持稀疏枚举
#### entries


`enum` `flags` 的条目名称数组
#### header


对于 C 兼容的语言，是已经定义了该值的头文件。如果该定义被多family 共享（例`IFNAMSIZ`），C 兼容语言的代码生成器可能倾向于添加一个适当include，而不是重生成一个定义
### attribute-sets


该属性包含该 family netlink 属性信息。所family 至少有一个属性集，大多数多个。`attribute-sets` 是一个数组，每个条目描述一个集合
注意，该规范是“扁平化”的，并非用来在视觉resemble netlink 消息的格式（不同内核注释中看到的某些特设文档格式）。在规范中，从属属性集不是作为嵌套内联定义的，
而是在一个单独的、由容器`nested-attributes` 属性引用的属性集中定义
规范也可能包含分数集（fractional sets）—带有 `subset-of` 属性的集合。这类集描述一个完整集合的某一部分，允许收窄巢（nest）中允许出现的属性，或细化验证标准分数集只能在巢中使用。它们不会以任何形式渲染uAPI
#### name


唯一标识该属性集，操作和嵌套属性通过 `name` 引用这些集合
#### subset-of


重新定义另一个集合的一部分（一个分数集）。允许根据其所处的巢，收窄字段、改变验标准甚至属性类型。分数集中每个属性的 `value` 隐式地与主集中相同
#### attributes


该集合中的属性列表

### Attribute 属

#### name


标识该属性，在集合内唯一
#### type


Netlink 属性类型，参见 attr_types
#### value


数值属ID，用于序列化Netlink 消息中。可以跳`value` 属性，这种情况下属ID
将是前一个属性的值加一（递归地），而属性集中的第一个属性为 `1`
属性（和操作）使用 `1` 作为第一个条目的默认值（不同definitions 中从 `0` 开始的
枚举），因为条目 `0` 几乎总是保留为“未定义”。如果需要，规范可以显式value 设为
`0`銆。
注意，属性的 `value` 仅在其主集中定义（不在子集中）
#### enum


对整数类型，指定该属性中的值属`definitions` 节中的某`enum` `flags`
#### enum-as-flags


无论其在 `definitions` 中的类型如何，都`enum` 视为 `flags`。当同时需`enum` `flags` 两种形式时，`definitions` 应包含一`enum`，而需`flags` 形式的属性应使用
此属性
#### nested-attributes


标识给定属性内嵌套属性的属性空间。仅对可能拥有子属性的复杂属性有效
#### multi-attr（数组）


布尔属性，表示此属性可能出现多次。允许属性重复是实现数组（无需额外嵌套）的推荐方式
#### byte-order


对整数类型，指定属性的字节- `little-endian` `big-endian`
#### checks


内核使用的输入验证约束。用户空间应通过 Generic Netlink 自省来查询运行中的内核的
策略，而不是依赖规范文件中的描述
内核中的验证策略由类型定义（`type` `nested-attributes`）与 `checks` 组合而成
#### sub-type


family 有表达数组的特殊方式。`sub-type` 可用于定义数组成员的类型，前提是数组成员
并未作为属性（在一个真实的属性空间中）被完整定义。例如，一u32 值的 C 数组可以
`type: binary` `sub-type: u32` 来描述。二进制类型和旧式数组格式在
[genetlink-legacy](genetlink-legacy) 中有更详细的描述
#### display-hint


可选格式指示器，仅用于在显示此类型的值时选择正确的格式化机制。目前支持的提示`hex`、`mac`、`fddi`、`ipv4`、`ipv6` `uuid`
### operations


本节描述内核与用户空间之间传递的消息。本节中有三种类型的条目 - 操作（operations）通知（notifications）和事件（events）
操作描述最常见的请- 响应通信。用户发送请求，内核回复。每个操作可以包netlink
用户熟悉的两种模式的任意组合 - `do` `dump`。`do` `dump` 又各自包`request`
`response` 属性的组合。如果在某个方向上没有传递显式的带属性消息（例如一个不接受
过滤`dump`，或者一个内核仅netlink 错误码响应的 SET 操作），则可以跳`request` `response` 节。`request` `response` 节列出消息中允许的属性。该列表
只包含由 `attribute-set` 属性所引用集合中的属性名称
通知和事件都指向内核发送给某个多播组成员的异步消息。两者的区别在于，通知与其内容
与某GET 操作共享（GET 操作的名称在 `notify` 属性中指定）。这种安排通常用于携带
完整对象定义的通知
事件则更聚焦，只携带一部分信息而非完整的对象状态（一个虚构的例子是仅包含接口名和
新链路状态的链路状态变化事件）。事件包`event` 属性。事件被认为不太符合 netlink
的习惯用法，应优先使用通知
#### list


`genetlink` `operations` 的唯一属性，持有操作、通知等的列表
### Operation 属

#### name


标识该操作
#### value


数值消ID，用于序列化Netlink 消息中。应用与属性值相同的枚举规则<assign_val>
#### attribute-set


指定消息中包含的属性集
#### do


`doit` 请求的规范。应包含 `request`、`reply` 或这两者，每个持有一attr_list
#### dump


`dumpit` 请求的规范。应包含 `request`、`reply` 或这两者，每个持有一attr_list
#### notify


将该消息指定为通知。包含与该通知共享内容的操作名称（可能与持有此属性的操作相同（`do`）
#### event


事件中属性的规范，持有一attr_list。`event` 属性与 `notify` 互斥
#### mcgrp


`event` `notify` 一起使用，指定该消息属于哪个多播组

### Message 属性列

`request`、`reply` `event` 属性有一个单一`attributes` 属性，持有属性名称列表
消息还可以定`pre` `post` 属性，它们将被渲染为内核中`pre_doit` `post_doit` 调用（这些属性应被用户空间忽略）
### mcast-groups


本节列出family 的多播组
#### list


`genetlink` `mcast-groups` 的唯一属性，持有组的列表
### 澶氭挱缁勫睘鎬。

#### name


family 中唯一标识该多播组。与 Family ID 类似，多播组 ID 需要在运行时根据名解析

## 属性类

本节描述 `genetlink` 兼容性级别支持的属性类型。关于额外的属性类型，请参考不同级的文档
### 通用整数类型


`sint` `uint` 表示有符号和无符号的 64 位整数。如果值能放入 32 位，netlink
消息中只携带 32 位，否则携带完整64 位。注意，负载只对齐到 4 字节，因此完整的 64
位值可能是未对齐的
在大多数情况下，应优先使用通用整数类型而非定宽类型
### 定宽整数类型


定宽整数类型包括：`u8`、`u16`、`u32`、`u64`、`s8`、`s16`、`s32`、`s64`
注意，应尽量避免小于 32 位的类型，因为使用它们并不会Netlink 消息中节省任何内（由于对齐）。关64 位属性的填充，请参见 pad_type
除非 `byte-order` 另有指定，否则属性的负载是以主机字节序表示的整数
64 位值通常由内核对齐，但建议用户空间能够处理未对齐的值

### pad


特殊属性类型，用于为需要大netlink 标准 4 字节对齐（例64 位整数）的属性进填充。任何属性集中只能有一`pad` 类型的属性，并且在需要时它应被自动用于填充
### flag


没有负载的属性，其存在本身就是全部信息
### binary


原始二进制数据属性，内容对通用代码是不透明的
### string


字符串。除`checks` 中的 `unterminated-ok` 设为 `true`，否则该字符串必须是null
结尾的。`checks` 中的 `max-len` 表示可能的最长字符串，若未给出则字符串长度无界
注意，`max-len` 不计入结尾字符
### nest


包含其它（嵌套）属性的属性。`nested-attributes` 指定内部使用哪个属性集