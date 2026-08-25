
## ARM64 上的 HugeTLBpage


Hugepage 依赖于高效利TLB 来提升地址转换的性能。其收益取决于两者—
  - hugepage 的大  - TLB 支持的条目大
ARM64 移植支持两种 hugepage
### 1) pud/pmd 级别的块映射


这些是常hugepage，其pmd pud 页表项指向一块内存。无TLB 条目支持的大小如何，块映射都减少了转hugepage 地址所需的页表遍历深度
### 2) 使用连续位（Contiguous bit

该架构在转换表项（D4.5.3，ARM DDI 0487C.a）中提供了一个连续位，向
MMU 提示它是一组可缓存到单TLB 条目中的连续条目之一
Linux 中使用连续位来增pmd pte（最后一级）级别的映射大小。支持的
连续条目数量因页大小与页表级别而异

支持以下 hugepage 大小—
  ====== ========   ====    ========    ===
  - CONT PTE    PMD    CONT PMD    PUD
  ====== ========   ====    ========    ===
  4K:         64K     2M         32M     1G
  16K:         2M    32M          1G
  64K:         2M   512M         16G
  ====== ========   ====    ========    ===
