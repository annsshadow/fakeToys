## 内核同页合并（Kernel Samepage Merging

KSM 是一项节省内存的去重特性，CONFIG_KSM=y 启用，在 2.6.32 加入 Linux 内核。其实现请参`mm/ksm.c`，另http://lwn.net/Articles/306704/ https://lwn.net/Articles/330589/
KSM 的用户空间接口在 Documentation/admin-guide/mm/ksm.rst 中描述
## 设计


### 概述


   :DOC: Overview

### 反向映射

KSM 在稳定树中为 KSM 页面维护反向映射信息
如果一KSM 页面在少`max_page_sharing` VMA 之间共享，那代表KSM 页面的稳定树节点会指向一struct ksm_rmap_item 列表并且 KSM 页面`page->mapping` 指向该稳定树节点
当共享数量超过此阈值时，KSM 为稳定树增加第二维。树节点变为一“链（chain）”，链接一个或多个“副本（dup）”。每个“副本”都保存一KSM 页面的反向映射信息，`page->mapping` 指向该“副本”
每个“链”以及链接到某个“链”中的所有“副本”都强制保证：它们代相同的写保护内存内容，即使每个“副本”会被该内容的不KSM 页面副本
所指向
这样，与无限长度的反向映射列表相比，稳定树查找的计算复杂度不受影响仍然强制保证稳定树自身中不会存在 KSM 页面内容重复
`max_page_sharing` 强制的去重上限是必需的，以避免虚拟内rmap
列表增长过大。rmap 遍历具有 O(N) 复杂度，其中 N 是正在共享该页面rmap_items（即虚拟映射）数量，而该数量又受`max_page_sharing` 限制。因此这有效地将来自 rmap 遍历上下文的线O(N) 计算复杂度分到不同的 KSM 页面上。ksmd 对稳定节点“链”的遍历同样O(N)，但此处N 是稳定节点“副本”的数量，而非 rmap_items 的数量，因此它对 ksmd
性能没有显著影响。在实践中，最佳的稳定节点“副本”候选会被保留并位于
“副本”列表的头部
较高`max_page_sharing` 值会带来更快的内存合并（因为排入
stable_node chain->hlist 以待剪枝的稳定节点副本更少），以及更高的去重
因子，代价是在交换、内存紧凑、NUMA 均衡和页面迁移期间，任意 KSM 页面
rmap 遍历最坏情况变慢
`stable_node_dups/stable_node_chains` 比例也受 `max_page_sharing`
可调参数影响，较高的比例可能表明稳定节点副本中存在碎片，这可以通过
ksmd 中引入碎片整理算法来解决——该算法会将 rmap_items 从一个稳节点副本重新归档到另一个稳定节点副本，以释放其rmap_items 较少稳定节点“副本”，但这可能会增ksmd CPU 占用，并可能拖慢对应KSM 页面的只读计算
链接在稳定节点“链”中的整个稳定节点“副本”列表会被周期性扫描，剪枝过期的稳定节点。此类扫描的频率`stable_node_chains_prune_millisecs`
sysfs 可调参数定义
### 参

   :functions: mm_slot ksm_scan stable_node rmap_item

--
Izik Eidus,
Hugh Dickins, 17 Nov 2009
