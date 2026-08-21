## Memory Balancing


Started Jan 2000 by Kanoj Sarcar <kanoj@sgi.com>

内存平衡（memory balancing）对!__GFP_HIGH !__GFP_KSWAPD_RECLAIM，以及非 __GFP_IO 的分配都是必需的
调用者可能避免回收（reclaim）的第一个原因是，调用者由于持有自旋锁或处于中断上下文而无法睡眠。第二个原因可能是，调用者宁愿让分配失败，也不愿承担页面回收的开销。这可能会发生在具有 order-0 回退选项的机会性高阶分配请求上。在这种情况下，调用者可能也希望避免唤醒 kswapd
发出 __GFP_IO 分配请求是为了防止文件系统死锁
在没有不可睡眠的分配请求的情况下，进行平衡似乎是有害的。页面回收可以惰性触发，即仅在需要时（也就是zone 空闲内存0 时）才进行，而不是将其变成一个主动的过程
话虽如此，内核应当尽量从直接映射（direct mapped）的页池中满足对直接映射页的请求，而不是回退dma 页池，以便为 dma 请求（无论是否原子）保持 dma 页池的填充。类似的论点也适用highmem 与直接映射页。另一方面，如果有大量空闲dma 页，则更可取的做法是dma 页池中分配一个来满足常规内存请求，而不是承担常zone 平衡的开销
2.2 中，只有当空闲页的_总_数低于总内存的 1/64 时，才会触发内存平衡/页面回收。在 dma 内存与常规内存比例合适的情况下，即使 dma zone 完全为空，也很可能根本不会进行平衡.2 已经在各种内存大小的生产机器上运行，并且即便存在这个问题似乎也运行良好。在 2.3 中，由于 HIGHMEM 的存在，这个问题更加严重
2.3 中，zone 平衡可以通过两种方式之一完成：根zone 大小（可能还有更低级zone 的大小），我们可以在初始化时决定在平衡任zone 时应当追求多少空闲页。好处是，在平衡时我们不需要查看更低级zone 的大小；坏处是，由于忽略了更低级zone 中可能更低的用量，我们可能会过于频繁地进行平衡。此外，通过对分配例程稍作改动，可以memclass() 宏简化为一个简单的相等判断
另一种可能的解决方案是，仅当某个 zone _以及_ 其所有更低级zone 的空闲内存低于该 zone 及其更低级别 zone 总内存的 1/64 时，我们才进行平衡。这修复2.2 的平衡问题，并尽可能保持接近 2.2 的行为。此外，平衡算法在各种具有不同数量和类型 zone 的体系架构上都以相同方式工作。如果我们想做得更复杂，将来可以为不zone 中的空闲页分配不同的权重
请注意，如果常规 zone 的大小相dma zone 非常巨大，那么在决定是否平衡常规 zone 时，考虑空闲 dma 页就变得不那么重要了。此时第一种解决方案变得更有吸引力
所附补丁实现了第二种解决方案。它修复"了两个问题：第一，对于不可睡眠的分配，像 2.2 那样在低内存条件下唤kswapd。第二，HIGHMEM zone 也被平衡，以便给 replace_with_highmem() 获得一HIGHMEM 页的机会，同时确HIGHMEM 分配不会回退到常zone。这也确HIGHMEM 页不会被泄漏（例如，HIGHMEM 页位swapcache 中但未被任何人使用的情况下）
kswapd 还需要了解它应该平衡zone。kswapd 主要在无法进行平衡的情况下才被需要，可能是因为所有分配请求都来自中断上下文，而所有进程上下文都在睡眠。对2.3，kswapd 实际上不需要平highmem zone，因为中断上下文不会请求 highmem 页。kswapd 查看 zone 结构中的 zone_wake_kswapd 字段，以决定某个 zone 是否需要平衡
如果窃取某个页能够缓解该页所在节点的任意 zone 中已经低于其水位线（watermark）的内存压力，则可以从进程内存和 shm 中窃取该页
watermark[WMARK_MIN/WMARK_LOW/WMARK_HIGH]/low_on_memory/zone_wake_kswapd：这些是zone 的字段，用于决定一zone 何时需要被平衡。当页数低于 watermark[WMARK_MIN] 时，滞后（hysteretic）字low_on_memory 被设置。它会一直保持设置，直到空闲页数达到 watermark[WMARK_HIGH]。当 low_on_memory 被设置时，页面分配请求将尝试释放zone 中的一些页（前提是请求中设置了 GFP_WAIT）。与此正交的是决定是否去戳一kswapd 以释放一zone 页。该决定并非基于滞后，而是在空闲页数低watermark[WMARK_LOW] 时做出；在这种情况下也会设置 zone_wake_kswapd

(好的) 我听说过的想法：

1. 动态经验应当影响平衡：可以跟踪某个 zone 的失败请求数量，并将其输入平衡方案（jalvo@mbay.net2. 实现一个类replace_with_highmem() replace_with_regular()，以保护 dma 页。（lkd@tantalophile.demon.co.uk