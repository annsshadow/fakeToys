## 内核同页合并（Kernel Samepage Merging）


KSM 是一项节省内存的去重特性，由 CONFIG_KSM=y 启用，在 2.6.32 中
加入 Linux 内核。其实现请参见 `mm/ksm.c`，另见
http://lwn.net/Articles/306704/ 和 https://lwn.net/Articles/330589/。

KSM 的用户空间接口在 Documentation/admin-guide/mm/ksm.rst 中描述。

## 设计


### 概述


   :DOC: Overview

### 反向映射

KSM 在稳定树中为 KSM 页面维护反向映射信息。

如果一个 KSM 页面在少于 `max_page_sharing` 个 VMA 之间共享，那么
代表该 KSM 页面的稳定树节点会指向一个 struct ksm_rmap_item 列表，
并且 KSM 页面的 `page->mapping` 指向该稳定树节点。

当共享数量超过此阈值时，KSM 为稳定树增加第二维。树节点变为一个
“链（chain）”，链接一个或多个“副本（dup）”。每个“副本”都保存一个
KSM 页面的反向映射信息，其 `page->mapping` 指向该“副本”。

每个“链”以及链接到某个“链”中的所有“副本”都强制保证：它们代表
相同的写保护内存内容，即使每个“副本”会被该内容的不同 KSM 页面副本
所指向。

这样，与无限长度的反向映射列表相比，稳定树查找的计算复杂度不受影响。
仍然强制保证稳定树自身中不会存在 KSM 页面内容重复。

由 `max_page_sharing` 强制的去重上限是必需的，以避免虚拟内存 rmap
列表增长过大。rmap 遍历具有 O(N) 复杂度，其中 N 是正在共享该页面的
rmap_items（即虚拟映射）数量，而该数量又受到 `max_page_sharing` 的
限制。因此这有效地将来自 rmap 遍历上下文的线性 O(N) 计算复杂度分散
到不同的 KSM 页面上。ksmd 对稳定节点“链”的遍历同样是 O(N)，但此处的
N 是稳定节点“副本”的数量，而非 rmap_items 的数量，因此它对 ksmd
性能没有显著影响。在实践中，最佳的稳定节点“副本”候选会被保留并位于
“副本”列表的头部。

较高的 `max_page_sharing` 值会带来更快的内存合并（因为排入
stable_node chain->hlist 以待剪枝的稳定节点副本更少），以及更高的去重
因子，代价是在交换、内存紧凑、NUMA 均衡和页面迁移期间，任意 KSM 页面
的 rmap 遍历最坏情况变慢。

`stable_node_dups/stable_node_chains` 比例也受 `max_page_sharing`
可调参数影响，较高的比例可能表明稳定节点副本中存在碎片，这可以通过
在 ksmd 中引入碎片整理算法来解决——该算法会将 rmap_items 从一个稳定
节点副本重新归档到另一个稳定节点副本，以释放其中 rmap_items 较少的
稳定节点“副本”，但这可能会增加 ksmd 的 CPU 占用，并可能拖慢对应用
KSM 页面的只读计算。

链接在稳定节点“链”中的整个稳定节点“副本”列表会被周期性扫描，以
剪枝过期的稳定节点。此类扫描的频率由 `stable_node_chains_prune_millisecs`
sysfs 可调参数定义。

### 参考


   :functions: mm_slot ksm_scan stable_node rmap_item

--
Izik Eidus,
Hugh Dickins, 17 Nov 2009
