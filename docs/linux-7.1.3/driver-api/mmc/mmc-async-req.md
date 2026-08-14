## MMC 异步请求


## 动机


缓存维护开销有多大？

视情况而定。快速的 eMMC 与带有投机性缓存预取的多级缓存，使得缓存开销相对显著。如果
下一个请求的 DMA 准备工作与当前传输并行进行，DMA 准备开销就不会影响 MMC 性能。

非阻塞（异步）MMC 请求的意图是最小化一个 MMC 请求结束与另一个 MMC 请求开始之间的时间。

使用 mmc_wait_for_req() 时，在 dma_map_sg 与 dma_unmap_sg 处理期间 MMC 控制器处于空闲。
使用非阻塞 MMC 请求则可以在活跃的 MMC 请求进行的同时为下一个任务准备缓存。

## MMC 块设备驱动


MMC 块设备驱动中的 mmc_blk_issue_rw_rq() 被改为非阻塞。

吞吐量的提升与准备一个请求（准备的主要部分是 dma_map_sg() 与 dma_unmap_sg()）以及
内存速度所需的时间成正比。MMC/SD 越快，准备请求的时间就越显著。在一个 L2 缓存平台上，
大致预期的性能提升为：大块写入约 5%，大块读取约 10%。在省电模式下，当时钟以较低频率运行
时，DMA 准备可能耗费更多时间。只要这些较慢的准备工作与传输并行进行，性能就不会受影响。

## 来自 IOZone 与 mmc_test 的测量细节


https://wiki.linaro.org/WorkingGroups/Kernel/Specs/StoragePerfMMC-async-req

## MMC 核心 API 扩展


新增了一个公共函数 mmc_start_req()。

它为主机启动一个新的 MMC 命令请求。该函数并非真正非阻塞。如果存在正在进行的异步请求，
它会等待该请求完成，启动新请求并返回。它不会等待新请求完成。如果没有正在进行的请求，
它会启动新请求并立即返回。

## MMC 主机扩展


在 mmc_host_ops 中有两个可选成员——pre_req() 与 post_req()——主机驱动可以实现它们，
以便将工作移动到实际 mmc_host_ops.request() 函数调用之前和之后。

在 DMA 情况下，pre_req() 可执行 dma_map_sg() 并准备 DMA 描述符，post_req() 运行
dma_unmap_sg()。

## 优化首个请求


在一系列请求中的第一个请求无法与先前的传输并行准备，因为没有先前的请求。

pre_req() 中的参数 is_first_req 表示没有先前的请求。主机驱动可以针对此场景进行优化，
以最小化性能损失。一种优化方式是将当前请求拆分为两块，准备第一块并启动请求，最后准备
第二块并启动传输。

```

  if (is_first_req && req->size > threshold)
     /* 以完整传输大小启动 MMC 传输 */
     mmc_start_command(MMC_CMD_TRANSFER_FULL_SIZE);

     /*
      * 在 MMC 处理 cmd 的同时开始准备 DMA。
      * 请求的第一块准备耗时应与 "MMC 处理命令时间" 相同。
      * 如果准备时间超过 MMC cmd 时间，传输会被延迟，估计最大以 4k 作为第一块大小。
      */
      prepare_1st_chunk_for_dma(req);
      /* 将待处理描述符刷入 DMAC (dmaengine.h) */
      dma_issue_pending(req->dma_desc);

      prepare_2nd_chunk_for_dma(req);
      /*
       * 第二次 issue_pending 应在 MMC 用尽第一块之前调用。
       * 如果 MMC 在此次调用前用尽第一块数据，传输会被延迟。
       */
      dma_issue_pending(req->dma_desc);

```
