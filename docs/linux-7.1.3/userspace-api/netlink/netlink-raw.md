
## Netlink specification support for raw Netlink families


本文档描述了诸如 `NETLINK_ROUTE` 这类使用 `netlink-raw` 协议规范的原始（raw）Netlink 族所需的额外属性。

## Specification


netlink-raw schema 通过原始 netlink 族所需的协议号和组播 ID 等属性，扩展了 [genetlink-legacy <genetlink-legacy>](genetlink-legacy <genetlink-legacy>) schema。更多信息请参阅 classic_netlink。原始 netlink 族也使用特定类型的子消息（sub-message）。

### Globals


#### protonum


`protonum` 属性用于指定打开 netlink 套接字时要使用的协议号。


  # SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)

  name: rt-addr
  protocol: netlink-raw
  protonum: 0             # NETLINK_ROUTE 协议的一部分

### Multicast group properties


#### value


`value` 属性用于指定组播组注册要使用的组 ID。


  mcast-groups:
    list:
      -
        name: rtnlgrp-ipv4-ifaddr
        value: 5
      -
        name: rtnlgrp-ipv6-ifaddr
        value: 9
      -
        name: rtnlgrp-mctp-ifaddr
        value: 34

### Sub-messages


几个原始 netlink 族，如
rt-link<netlink-rt-link> 和
tc<netlink-tc> 使用属性嵌套（attribute nesting）作为一种抽象来携带模块特定信息。

```

    [OUTER NEST OR MESSAGE LEVEL]
      [GENERIC ATTR 1]
      [GENERIC ATTR 2]
      [GENERIC ATTR 3]
      [GENERIC ATTR - wrapper]
        [MODULE SPECIFIC ATTR 1]
        [MODULE SPECIFIC ATTR 2]

```
外层级别的 `GENERIC ATTRs` 定义在核心（或 rt_link 或核心 TC）中，而特定的驱动、TC 分类器、qdisc 等可以携带它们自己的、包裹在 `GENERIC ATTR - wrapper` 中的信息。尽管上面的例子显示了属性嵌套在 wrapper 内部，但模块通常拥有定义嵌套格式的完全自由。实际上，wrapper 属性的负载与 netlink 消息具有非常相似的特征。它可能包含固定头部/结构、netlink 属性，或两者皆有。由于这些共同特征，我们将 wrapper 属性的负载称为子消息（sub-message）。

子消息属性使用另一个属性的值作为选择键（selector key）来选择正确的子消息格式。例如，如果已经解码了以下属性：


  { "kind": "gre" }

并且我们遇到以下属性规范：


  -
    name: data
    type: sub-message
    sub-message: linkinfo-data-msg
    selector: kind

那么我们会查找名为 `linkinfo-data-msg` 的子消息定义，并使用 `kind` 属性的值（即 `gre`）作为键来选择该子消息的正确格式：


  sub-messages:
    name: linkinfo-data-msg
    formats:
      -
        value: bridge
        attribute-set: linkinfo-bridge-attrs
      -
        value: gre
        attribute-set: linkinfo-gre-attrs
      -
        value: geneve
        attribute-set: linkinfo-geneve-attrs

这会将该属性值解码为以名为 `linkinfo-gre-attrs` 的 attribute-set 作为属性空间的子消息。

子消息可以有一个可选的 `fixed-header`，后跟来自 `attribute-set` 的零个或多个属性。例如，以下 `tc-options-msg` 子消息定义了混合使用 `fixed-header`、`attribute-set` 或两者兼有的消息格式：


  sub-messages:
    -
      name: tc-options-msg
      formats:
        -
          value: bfifo
          fixed-header: tc-fifo-qopt
        -
          value: cake
          attribute-set: tc-cake-attrs
        -
          value: netem
          fixed-header: tc-netem-qopt
          attribute-set: tc-netem-attrs

请注意，selector 属性必须出现在任何依赖于它的子消息属性之前，出现在 netlink 消息中。

如果像 `kind` 这样的属性定义在多个嵌套级别上，那么子消息选择器将使用"最接近"选择器的那个值来解析。例如，如果同一个属性名定义在一个嵌套的 `attribute-set` 中（与子消息选择器一起）以及顶层的 `attribute-set` 中，那么选择器将使用"最接近"选择器的那个值来解析。如果该值没有出现在与规范定义相同级别的消息中，则这是一个错误。

### Nested struct definitions


许多原始 netlink 族，如 tc<netlink-tc>，使用嵌套结构体定义。`netlink-raw` schema 使得可以使用 `struct` 属性将结构体嵌入到结构体定义中。例如，以下结构体定义将 `tc-ratespec` 结构体定义嵌入到 `struct tc-tbf-qopt` 的 `rate` 和 `peakrate` 成员中。


  -
    name: tc-tbf-qopt
    type: struct
    members:
      -
        name: rate
        type: binary
        struct: tc-ratespec
      -
        name: peakrate
        type: binary
        struct: tc-ratespec
      -
        name: limit
        type: u32
      -
        name: buffer
        type: u32
      -
        name: mtu
        type: u32
