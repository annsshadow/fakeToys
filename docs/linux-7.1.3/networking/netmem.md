
## 网络驱动Netmem 支持


本文档概述了网络驱动支持 netmem 的要求，netmem 是一种抽象的内存类型，能够支持诸如设备内TCP 特性。通过支持 netmem，驱动可以以很少甚至无需修改的方式，配合各种底层内存类型工作
Netmem 的好处：

- 灵活性：Netmem 可以由不同的内存类型（例struct page、DMA-buf）作为后端，使驱动能够支持各  用例，例如设备内TCP- 面向未来：带netmem 支持的驱动已为依赖它的后续特性做好准备- 简化开发：无论底层内存实现如何，驱动都通过一致的 API 进行交互
## 驱动 RX 要求


1. 驱动必须支持 page_pool
2. 驱动必须支持 tcp-data-split ethtool 选项
3. 驱动必须为有效载荷内存使page_pool netmem API。netmem API 当前page API 一一对应   转换netmem 应当可以通过page API 切换netmem API，并在驱动中通过 netmem_refs 而非
   struct page * 来跟踪内存来实现
   - page_pool_alloc -> page_pool_alloc_netmem
   - page_pool_get_dma_addr -> page_pool_get_dma_addr_netmem
   - page_pool_put_page -> page_pool_put_netmem

   目前并非所page API 都有对应netmem 版本。如果你的驱动依赖某个缺失的 netmem API，欢   自行添加并提交到 netdev@，或者联系维护者和/almasrymina@google.com 以寻求帮助添加该 netmem API
4. 驱动必须使用以下 PP_FLAGS
   - PP_FLAG_DMA_MAP：netmem 不能被驱动进dma 映射。驱动必须将 dma 映射委托page_pool，它知道
     何时（或不）适合进行 dma 映射   - PP_FLAG_DMA_SYNC_DEV：netmem dma 地址不一定能被驱动进dma 同步。驱动必须将 dma 同步委托     page_pool，它知道何时（或不）适合进行 dma 同步   - PP_FLAG_ALLOW_UNREADABLE_NETMEM。仅当启用了 tcp-data-split 时，驱动才必须指定此标志
5. 驱动不得假定 netmem 是可读的或由页作为后端。page_pool 返回netmem 可能是不可读的，此时
   netmem_address() 将返NULL。驱动必须正确处理不可读netmem，即netmem_address() NULL 时，
   不要尝试处理其内容
   理想情况下，驱动不必通过netmem_is_net_iov() 这样的辅助函数检查底netmem 类型，也不必通过
   netmem_to_page() netmem_to_net_iov() netmem 转换为它的任何底层类型。在大多数情况下，提供了
   抽象了这种复杂性的 netmem page_pool 辅助函数（而且还可以添加更多）
6. 驱动必须使用 page_pool_dma_sync_netmem_for_cpu() 来代dma_sync_single_range_for_cpu()。对于某   内存提供方，面向 CPU dma 同步将由 page_pool 完成；对于其他提供方（特别是 dmabuf 内存提供方）   面向 CPU dma 同步由使dmabuf API 的用户空间负责。驱动必须将整个 dma 同步操作委托page_pool   它会正确地完成
7. 避免基于 page_pool 实现驱动特定的回收。驱动不能持有一struct page 来做自己的回收，因为 netmem
   可能不是struct page 作为后端的。不过，你可以为此目的通过 page_pool_fragment_netmem()    page_pool_ref_netmem() 持有一page_pool 引用，但要注意某netmem 类型可能有更长的流转时间   例如在零拷贝场景中用户空间持有引用时
## 驱动 TX 要求


1. 驱动不得直接netmem dma_addr 传递给任何 dma-mapping API。这是因netmem dma_addr 可能来自
   dma-buf 这样dma-mapping API 不兼容的源
   应使用像 netmem_dma_unmap_page_attrs() netmem_dma_unmap_addr_set() 这样的辅助函数，来代   dma_unmap_page[_attrs]()、dma_unmap_addr_set()。无论来源如何，netmem 变体都会正确处理 netmem    dma_addr，并在适当时委托给 dma-mapping API
   目前并非所dma-mapping API 都有对应netmem 版本。如果你的驱动依赖某个缺失的 netmem API，欢   自行添加并提交到 netdev@，或者联系维护者和/almasrymina@google.com 以寻求帮助添加该 netmem API
2. 驱动应通过设置 `netdev->netmem_tx = true` 来声明支持