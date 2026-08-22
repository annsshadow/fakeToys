## 页池（Page Pool）API


   :doc: page_pool allocator

## 架构概览


    +------------------+
    |       Driver     |
    +------------------+
            ^
            |
            |
            |
            v
    +--------------------------------------------+
    |                request memory              |
    +--------------------------------------------+
        ^                                  ^
        |                                  |
        | Pool empty                       | Pool has entries
        |                                  |
        v                                  v
    +-----------------------+     +------------------------+
    | alloc (and map) pages |     |  get page from cache   |
    +-----------------------+     +------------------------+
                                    ^                    ^
                                    |                    |
                                    | cache available    | No entries, refill
                                    |                    | from ptr-ring
                                    |                    |
                                    v                    v
                          +-----------------+     +------------------+
                          |   Fast cache    |     |  ptr-ring cache  |
                          +-----------------+     +------------------+

## 监控


系统上关于页池的信息可以通过 netdev generic netlink 系列访问（见
Documentation/netlink/specs/netdev.yaml）
## API 接口


创建的池数量**必须**与硬件队列数量相匹配，除非硬件限制使得这不可能。否则就违背了页的初衷，即从缓存中无锁地快速分配页面。这种无锁保证自然来自于NAPI softirq 下运行这种保护并不严格必须NAPI，任何“分配页面不会导致竞态条件”的保证都足够
   :identifiers: page_pool_create

   :identifiers: struct page_pool_params

   :identifiers: page_pool_put_page page_pool_put_full_page
		 page_pool_recycle_direct page_pool_free_va
		 page_pool_dev_alloc_pages page_pool_dev_alloc_frag
		 page_pool_dev_alloc page_pool_dev_alloc_va
		 page_pool_get_dma_addr page_pool_get_dma_dir

   :identifiers: page_pool_put_page_bulk page_pool_get_stats

### DMA 同步


驱动始终负责CPU 同步页面驱动也可以选择负责为设备同步，或者设`PP_FLAG_DMA_SYNC_DEV` 标志来请求从页池分配页面已经为设备同步好
如果设置`PP_FLAG_DMA_SYNC_DEV`，驱动必须告知核心需要同步缓冲区的哪一部分。这使得
核心能够避免在驱动知道设备只访问了页面一部分时，同步整个页面
大多数驱动会在帧前面保留 headroom。设备的 DMA 不会触及这部分缓冲区，因此为了避免同它，驱动可以相应地设struct page_pool_params 中的 `offset` 字段
对于XDP 发送（xmit）与 skb 路径上回收的页面，页池将使用 struct page_pool_params `max_len` 成员来决定需要同步多少页面（`offset` 开始）当在驱动中直接释放页面时（page_pool_put_page()），`dma_sync_size` 参数指定需要同步多缓冲区
若不确定，请`offset` 设为 0、`max_len` 设为 `PAGE_SIZE`，并传入 -1 作为 `dma_sync_size`这组参数组合永远是正确的
注意，同步参数是针对整个页面的。在使用片段（`PP_FLAG_PAGE_FRAG`）时这一点很重要，因分配的缓冲区可能小于一个完整页面。除非驱动开发者真正理解页池内部机制，否则建议对分片段
页池始终使用 `offset = 0`、`max_len = PAGE_SIZE`
### 统计 API 与结构体


如果内核配置`CONFIG_PAGE_POOL_STATS=y`，则下面描述API page_pool_get_stats() 结构体可用。它接受一个指`struct page_pool` 的指针，以及一个由调用者分配的
struct page_pool_stats 指针
较旧的驱动通过 ethtool debugfs 暴露页池统计信息相同的统计信息可以通过 netlink netdev 系列以与驱动无关的方式访问
   :identifiers: struct page_pool_recycle_stats
		 struct page_pool_alloc_stats
		 struct page_pool_stats

## 编码示例


### 注册


    /** Page pool registration **/
    struct page_pool_params pp_params = { 0 };
    struct xdp_rxq_info xdp_rxq;
    int err;

    pp_params.order = 0;
    /** internal DMA mapping in page_pool **/
    pp_params.flags = PP_FLAG_DMA_MAP;
    pp_params.pool_size = DESC_NUM;
    pp_params.nid = NUMA_NO_NODE;
    pp_params.dev = priv->dev;
    pp_params.napi = napi; /** only if locking is tied to NAPI **/
    pp_params.dma_dir = xdp_prog ? DMA_BIDIRECTIONAL : DMA_FROM_DEVICE;
    page_pool = page_pool_create(&pp_params);

    err = xdp_rxq_info_reg(&xdp_rxq, ndev, 0);
    if (err)
        goto err_out;

    err = xdp_rxq_info_reg_mem_model(&xdp_rxq, MEM_TYPE_PAGE_POOL, page_pool);
    if (err)
        goto err_out;

### NAPI poller


    /** NAPI Rx poller **/
    enum dma_data_direction dma_dir;

    dma_dir = page_pool_get_dma_dir(dring->page_pool);
    while (done < budget) {
        if (some error)
            page_pool_recycle_direct(page_pool, page);
        if (packet_is_xdp) {
            if XDP_DROP:
                page_pool_recycle_direct(page_pool, page);
        } else (packet_is_skb) {
            skb_mark_for_recycle(skb);
            new_page = page_pool_dev_alloc_pages(page_pool);
        }
    }

### Stats


    #ifdef CONFIG_PAGE_POOL_STATS
    /** retrieve stats **/
    struct page_pool_stats stats = { 0 };
    if (page_pool_get_stats(page_pool, &stats)) {
        /** perhaps the driver reports statistics with ethool **/
        ethtool_print_allocation_stats(&stats.alloc_stats);
        ethtool_print_recycle_stats(&stats.recycle_stats);
    }
    #endif

### Driver unload


    /** Driver unload **/
    page_pool_put_full_page(page_pool, page, false);
    xdp_rxq_info_unreg(&xdp_rxq);
