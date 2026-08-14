
## Devlink DPIPE


## 背景


在执行硬件卸载过程时，许多硬件相关的细节无法被呈现。这些细节对调试很有用，而
`devlink-dpipe` 提供了一种标准化的方式来提供对卸载过程的可见性。

例如，Linux 内核使用的最长前缀匹配（LPM）路由算法可能与硬件实现不同。流水线调试
API（DPIPE）旨在以通用的方式让用户能够看到 ASIC 流水线的内部。

硬件卸载过程应当被实现为：用户应当无法区分硬件实现与软件实现。在这个过程中，硬件
细节被忽略了。实际上这些细节可能很有意义，并应以某种标准方式暴露出来。

当想要将整个网络栈的控制路径卸载到交换 ASIC 时，这个问题变得更加复杂。由于硬件和
软件模型存在差异，某些过程无法被正确表示。

一个例子是内核的 LPM 算法，它在许多情况下与硬件实现大不相同。配置 API 是相同的，但
不能指望转发信息库（FIB）在硬件中看起来像 Level Path Compression trie（LPC-trie）。

在许多情况下，仅基于内核的转储来分析系统故障可能不够。通过将此数据与底层硬件的
补充信息相结合，可以使调试更容易；此外，这些信息在调试性能问题时也很有用。

## 概述


`devlink-dpipe` 接口弥补了这一差距。硬件的流水线被建模为匹配/动作表的图。每个表
代表一个特定的硬件块。这种模型并不新鲜，最早由 P4 语言使用。

传统上它被用作硬件配置的替代模型，但 `devlink-dpipe` 接口将其用作可见性目的的
标准化补充工具。来自 `devlink-dpipe` 的系统视图应当根据标准配置工具所做的更改而
变化。

例如，使用三态内容可寻址存储器（TCAM）来实现访问控制列表（ACL）是很常见的。TCAM
存储器可以被划分为 TCAM 区域。复杂的 TC 过滤器可以有多个具有不同优先级和不同查找键
的规则。另一方面，硬件 TCAM 区域有预定义的查找键。使用 TCAM 引擎卸载 TC 过滤规则
可能导致多个 TCAM 区域以链式相连（这可能会影响数据路径延迟）。作为对新 TC 过滤器的
响应，应当创建描述这些区域的新表。

## 模型


`DPIPE` 模型引入了几个对象：

  - 头（headers）
  - 表（tables）
  - 条目（entries）

`header` 描述数据包格式，并为包内的字段提供名称。`table` 描述硬件块。`entry` 描述
特定表的实际内容。

硬件流水线不是端口特定的，而是描述整个 ASIC。因此它被绑定到 `devlink` 基础设施的
顶层。

驱动可以在运行时注册和注销表，以支持动态行为。这种动态行为对于描述像 TCAM 区域这样
可以动态分配和释放的硬件块是必须的。

`devlink-dpipe` 一般无意用于配置。例外是对特定表进行硬件计数。

以下命令用于从用户空间获取 `dpipe` 对象：

  - `table_get`：接收表的描述。
  - `headers_get`：接收设备支持的头。
  - `entries_get`：接收表当前的条目。
  - `counters_set`：启用或禁用表上的计数器。

### 表（Table）


驱动应当为每个表实现以下操作：

  - `matches_dump`：转储支持的匹配。
  - `actions_dump`：转储支持的动作。
  - `entries_dump`：转储表的实际内容。
  - `counters_set_update`：将计数器启用或禁用状态与硬件同步。

### 头/字段（Header/Field）


以类似于 P4 的方式，头和字段被用来描述表的行为。标准协议头与特定 ASIC 元数据之间
略有差异。协议头应当在 `devlink` 核心 API 中声明。另一方面，ASIC 元数据是驱动特定的，
应当在驱动中定义。此外，每个驱动特定的 devlink 文档文件都应当记录其实现的驱动特定
`dpipe` 头。头和字段通过枚举来识别。

为了提供进一步的可见性，某些 ASIC 元数据字段可以映射到内核对象。例如，内部路由器
接口索引可以直接映射到网络设备的 ifindex。不同虚拟路由与转发（VRF）表使用的 FIB
表索引可以映射到内部路由表索引。

### 匹配（Match）


匹配保持为原始的、接近硬件操作的形式。像 LPM 这样的匹配类型不被支持，因为这正是我们
希望完整描述的那个过程。匹配的示例：

  - `field_exact`：对特定字段的精确匹配。
  - `field_exact_mask`：对屏蔽后的特定字段的精确匹配。
  - `field_range`：对特定范围的匹配。

应当指定头和字段的 id，以识别特定的字段。此外，应当指定头索引，以区分包中同类型的
多个头（隧道场景）。

### 动作（Action）


与匹配类似，动作也保持为原始的、接近硬件操作的形式。例如：

  - `field_modify`：修改字段值。
  - `field_inc`：递增字段值。
  - `push_header`：添加一个头。
  - `pop_header`：移除一个头。

### 条目（Entry）


特定表的条目可以按需转储。每个条目由一个索引标识，其属性由一组匹配/动作值以及特定
计数器描述。通过转储表的内容，可以解析表之间的交互关系。

## 抽象示例


以下是 Mellanox Spectrum ASIC 的 L3 部分抽象模型示例。这些块按它们在流水线中出现的
顺序描述。以下示例中的表大小并非真实硬件大小，仅用于演示目的。

### LPM


LPM 算法可以实现为哈希表列表。每个哈希表包含相同前缀长度的路由。列表的根是 /32，在
未命中（miss）的情况下，硬件将继续到下一个哈希表。搜索的深度会影响数据路径延迟。

在命中（hit）的情况下，条目包含关于流水线下一阶段的信息，后者解析 MAC 地址。下一阶段
可以是直连路由的本地主机表，或者是下一跳的邻接表。`meta.lpm_prefix` 字段用于连接两个
LPM 表。


    table lpm_prefix_16 {
      size: 4096,
      counters_enabled: true,
      match: { meta.vr_id: exact,
               ipv4.dst_addr: exact_mask,
               ipv6.dst_addr: exact_mask,
               meta.lpm_prefix: exact },
      action: { meta.adj_index: set,
                meta.adj_group_size: set,
                meta.rif_port: set,
                meta.lpm_prefix: set },
    }

### 本地主机（Local Host）


在本地路由的情况下，LPM 查找已经解析了出口路由器接口（RIF），但确切的 MAC 地址尚不知晓。
本地主机表是一个哈希表，将输出接口 id 与目的 IP 地址组合作为键。结果是 MAC 地址。


    table local_host {
      size: 4096,
      counters_enabled: true,
      match: { meta.rif_port: exact,
               ipv4.dst_addr: exact},
      action: { ethernet.daddr: set }
    }

### 邻接（Adjacency）


在远程路由的情况下，该表执行 ECMP。LPM 查找得到 ECMP 组大小和索引，后者作为进入该表的
全局偏移。同时会生成数据包的哈希。基于 ECMP 组大小和数据包的哈希，生成一个本地偏移。
多个 LPM 条目可以指向同一个邻接组。


    table adjacency {
      size: 4096,
      counters_enabled: true,
      match: { meta.adj_index: exact,
               meta.adj_group_size: exact,
               meta.packet_hash_index: exact },
      action: { ethernet.daddr: set,
                meta.erif: set }
    }

### ERIF


如果出口 RIF 和目的 MAC 已由前面的表解析，该表会执行多个操作，例如 TTL 递减和 MTU 检查。
然后做出转发/丢弃的决定，并根据数据包的类型（广播、单播、组播）更新端口 L3 统计。


    table erif {
      size: 800,
      counters_enabled: true,
      match: { meta.rif_port: exact,
               meta.is_l3_unicast: exact,
               meta.is_l3_broadcast: exact,
               meta.is_l3_multicast, exact },
      action: { meta.l3_drop: set,
                meta.l3_forward: set }
    }
