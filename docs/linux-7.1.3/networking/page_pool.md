## 椤垫睜锛圥age Pool锛堿PI


   :doc: page_pool allocator

## 鏋舵瀯姒傝


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

## 鐩戞帶


绯荤粺涓婂叧浜庨〉姹犵殑淇℃伅鍙互閫氳繃 netdev generic netlink 绯诲垪璁块棶锛堣
Documentation/netlink/specs/netdev.yaml锛夈€?
## API 鎺ュ彛


鍒涘缓鐨勬睜鏁伴噺**蹇呴』**涓庣‖浠堕槦鍒楁暟閲忕浉鍖归厤锛岄櫎闈炵‖浠堕檺鍒朵娇寰楄繖涓嶅彲鑳姐€傚惁鍒欏氨杩濊儗浜嗛〉姹?鐨勫垵琛凤紝鍗充粠缂撳瓨涓棤閿佸湴蹇€熷垎閰嶉〉闈€傝繖绉嶆棤閿佷繚璇佽嚜鐒舵潵鑷簬鍦?NAPI softirq 涓嬭繍琛屻€?杩欑淇濇姢骞朵笉涓ユ牸蹇呴』鏄?NAPI锛屼换浣曗€滃垎閰嶉〉闈笉浼氬鑷寸珵鎬佹潯浠垛€濈殑淇濊瘉閮借冻澶熴€?
   :identifiers: page_pool_create

   :identifiers: struct page_pool_params

   :identifiers: page_pool_put_page page_pool_put_full_page
		 page_pool_recycle_direct page_pool_free_va
		 page_pool_dev_alloc_pages page_pool_dev_alloc_frag
		 page_pool_dev_alloc page_pool_dev_alloc_va
		 page_pool_get_dma_addr page_pool_get_dma_dir

   :identifiers: page_pool_put_page_bulk page_pool_get_stats

### DMA 鍚屾


椹卞姩濮嬬粓璐熻矗涓?CPU 鍚屾椤甸潰銆?椹卞姩涔熷彲浠ラ€夋嫨璐熻矗涓鸿澶囧悓姝ワ紝鎴栬€呰缃?`PP_FLAG_DMA_SYNC_DEV` 鏍囧織鏉ヨ姹備粠椤垫睜鍒嗛厤鐨?椤甸潰宸茬粡涓鸿澶囧悓姝ュソ銆?
濡傛灉璁剧疆浜?`PP_FLAG_DMA_SYNC_DEV`锛岄┍鍔ㄥ繀椤诲憡鐭ユ牳蹇冮渶瑕佸悓姝ョ紦鍐插尯鐨勫摢涓€閮ㄥ垎銆傝繖浣垮緱
鏍稿績鑳藉閬垮厤鍦ㄩ┍鍔ㄧ煡閬撹澶囧彧璁块棶浜嗛〉闈竴閮ㄥ垎鏃讹紝鍚屾鏁翠釜椤甸潰銆?
澶у鏁伴┍鍔ㄤ細鍦ㄥ抚鍓嶉潰淇濈暀 headroom銆傝澶囩殑 DMA 涓嶄細瑙﹀強杩欓儴鍒嗙紦鍐插尯锛屽洜姝や负浜嗛伩鍏嶅悓姝?瀹冿紝椹卞姩鍙互鐩稿簲鍦拌缃?struct page_pool_params 涓殑 `offset` 瀛楁銆?
瀵逛簬鍦?XDP 鍙戦€侊紙xmit锛変笌 skb 璺緞涓婂洖鏀剁殑椤甸潰锛岄〉姹犲皢浣跨敤 struct page_pool_params 鐨?`max_len` 鎴愬憳鏉ュ喅瀹氶渶瑕佸悓姝ュ灏戦〉闈紙浠?`offset` 寮€濮嬶級銆?褰撳湪椹卞姩涓洿鎺ラ噴鏀鹃〉闈㈡椂锛坧age_pool_put_page()锛夛紝`dma_sync_size` 鍙傛暟鎸囧畾闇€瑕佸悓姝ュ灏?缂撳啿鍖恒€?
鑻ヤ笉纭畾锛岃灏?`offset` 璁句负 0銆乣max_len` 璁句负 `PAGE_SIZE`锛屽苟浼犲叆 -1 浣滀负 `dma_sync_size`銆?杩欑粍鍙傛暟缁勫悎姘歌繙鏄纭殑銆?
娉ㄦ剰锛屽悓姝ュ弬鏁版槸閽堝鏁翠釜椤甸潰鐨勩€傚湪浣跨敤鐗囨锛坄PP_FLAG_PAGE_FRAG`锛夋椂杩欎竴鐐瑰緢閲嶈锛屽洜涓?鍒嗛厤鐨勭紦鍐插尯鍙兘灏忎簬涓€涓畬鏁撮〉闈€傞櫎闈為┍鍔ㄥ紑鍙戣€呯湡姝ｇ悊瑙ｉ〉姹犲唴閮ㄦ満鍒讹紝鍚﹀垯寤鸿瀵瑰垎鐗囨
椤垫睜濮嬬粓浣跨敤 `offset = 0`銆乣max_len = PAGE_SIZE`銆?
### 缁熻 API 涓庣粨鏋勪綋


濡傛灉鍐呮牳閰嶇疆浜?`CONFIG_PAGE_POOL_STATS=y`锛屽垯涓嬮潰鎻忚堪鐨?API page_pool_get_stats() 涓?缁撴瀯浣撳彲鐢ㄣ€傚畠鎺ュ彈涓€涓寚鍚?`struct page_pool` 鐨勬寚閽堬紝浠ュ強涓€涓敱璋冪敤鑰呭垎閰嶇殑
struct page_pool_stats 鎸囬拡銆?
杈冩棫鐨勯┍鍔ㄩ€氳繃 ethtool 鎴?debugfs 鏆撮湶椤垫睜缁熻淇℃伅銆?鐩稿悓鐨勭粺璁′俊鎭彲浠ラ€氳繃 netlink netdev 绯诲垪浠ヤ笌椹卞姩鏃犲叧鐨勬柟寮忚闂€?
   :identifiers: struct page_pool_recycle_stats
		 struct page_pool_alloc_stats
		 struct page_pool_stats

## 缂栫爜绀轰緥


### 娉ㄥ唽


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
