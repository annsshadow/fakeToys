
## BPF_PROG_TYPE_FLOW_DISSECTOR


## 概述


流解析器（flow dissector）是一个从数据包中解析元数据的例程。它被用于网络子系统的多个
地方（RFS、流哈希等）。

BPF 流解析器试图用 BPF 重新实现基于 C 的流解析器逻辑，以获得 BPF 验证器（verifier）的全部
好处（即指令数量和尾调用的限制）。

## API


BPF 流解析器程序运行在 `flow_keys` 上。但是，只允许一组受限的字段：`flow_keys`、`struct bpf_flow_keys` 和 `flow_keys`。
`flow_keys` 是 `struct bpf_flow_keys`，包含流解析器的输入和输出参数。

输入如下：
  - `flags` - 网络头部的初始偏移
  - `flags` - 传输层头部的初始偏移，初始化为 nhoff
  - `flags` - L3 协议类型，从 L2 头部解析得出
  - `flags` - 可选标志

BPF 流解析器程序应当填写其余的 ` fields. Input arguments `struct bpf_flow_keys` fields. Input arguments ` 的 nhoff/thoff/n_proto``
也应相应调整。

BPF 程序的返回码是 BPF_OK（表示解析成功）或 BPF_DROP（表示解析错误）。

## __sk_buff->data


在无 VLAN 的情况下，BPF 流解析器的初始状态如下：
```

  +------+------+------------+-----------+
  | DMAC | SMAC | ETHER_TYPE | L3_HEADER |
  +------+------+------------+-----------+
                              ^
                              |
                              +-- flow dissector starts here


```

  skb->data + flow_keys->nhoff 指向 L3_HEADER 的第一个字节
  flow_keys->thoff = nhoff
  flow_keys->n_proto = ETHER_TYPE

在 VLAN 的情况下，流解析器可能以两种不同的状态被调用。

```

  +------+------+------+-----+-----------+-----------+
  | DMAC | SMAC | TPID | TCI |ETHER_TYPE | L3_HEADER |
  +------+------+------+-----+-----------+-----------+
                        ^
                        |
                        +-- flow dissector starts here

```

  skb->data + flow_keys->nhoff 指向 TCI 的第一个字节
  flow_keys->thoff = nhoff
  flow_keys->n_proto = TPID

请注意 TPID 可以是 802.1AD，因此 BPF 程序对于双标签（double tagged）数据包需要解析 VLAN
信息两次。


```

  +------+------+------+-----+-----------+-----------+
  | DMAC | SMAC | TPID | TCI |ETHER_TYPE | L3_HEADER |
  +------+------+------+-----+-----------+-----------+
                                          ^
                                          |
                                          +-- flow dissector starts here

```

  skb->data + flow_keys->nhoff 指向 L3_HEADER 的第一个字节
  flow_keys->thoff = nhoff
  flow_keys->n_proto = ETHER_TYPE

在这种情况下，VLAN 信息在流解析器之前已经被处理，BPF 流解析器不需要处理它。


这里的要点如下：BPF 流解析器程序可能带有可选的 VLAN 头部被调用，并且应当优雅地处理两种
情况：存在单 VLAN 或双 VLAN 以及不存在 VLAN 的情况。同一个程序可能在两种情况下都被调用，
因此必须仔细编写以处理两种情况。


## 标志


`flow_keys->flags` 可能包含可选的输入标志，其作用如下：

- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 告诉 BPF 流解析器继续解析第一个分片；默认预期行为是流解析器一旦发现数据包被
  分片就立即返回；由 `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` 用于为 GRO 估算所有头部的长度。
- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 告诉 BPF 流解析器在到达 IPv6 流标签时停止解析；由 `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` 用于获取流哈希。
- `BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP` - 告诉 BPF 流解析器在到达封装头部时停止解析；由路由基础设施使用。


## 参考实现


参见 `tools/testing/selftests/bpf/flow_dissector_load.[hc]` 获取参考实现，以及 `tools/testing/selftests/bpf/flow_dissector_load.[hc]` 获取加载器。bpftool 也可用于加载 BPF 流解析器程序。

参考实现的组织方式如下：
  - `bpf_tail_call` 映射，包含每个受支持 L3 协议的子程序
  - `bpf_tail_call` 例程 - 入口点；它进行输入 `n_proto` 解析，并借助 `bpf_tail_call` 分发到相应的 L3 处理程序

由于 BPF 目前不支持循环（或任何回跳），改用 jmp_table 来处理多级封装（以及 IPv6 选项）。


## 当前限制

BPF 流解析器不支持导出内核内基于 C 的实现所能导出的全部元数据。一个显著的例子是单 VLAN
（802.1Q）和双 VLAN（802.1AD）标签。请参考 `struct bpf_flow_keys` 获取当前可从 BPF 上下文导出的信息集合。

当 BPF 流解析器被附加到根网络命名空间（machine-wide 策略）时，用户无法在其子网络命名空间中
覆盖它。
