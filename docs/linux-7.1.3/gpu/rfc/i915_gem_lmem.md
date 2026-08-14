## I915 DG1/LMEM RFC 章节


## 上游计划

针对上游，把所有 DG1 相关代码落地并最终启用、同时包含全部 uAPI 部分的总体计划如下：

- 合并 DG1 的基础硬件支持（仍不带 pciid）
- 在特殊的 CONFIG_BROKEN（或类似）标志之后合并 uAPI 部分
        - 此时我们仍可做改动，但重要的是这让我们
          能在 CI 中运行可利用本地内存（local-memory）的 IGTs
- 迁移到 TTM，确保一切持续可用。部分工作内容：
        - 面向独立显卡的 TTM shrinker
        - 完整 dma_resv_lock 的 dma_resv_lockitem，即不仅仅是 trylock
        - 使用 TTM CPU 缺页处理程序（pagefault handler）
        - 将 shmem 后端路由到独立显卡的 TTM SYSTEM
        - TTM 可回收对象（purgeable object）支持
        - 将 i915 buddy 分配器迁移到 TTM
- 发送 RFC（抄送 mesa-dev）以获得 uAPI 的最终签署确认
- 为 DG1 添加 pciid 并真正启用 uAPI
