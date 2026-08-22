## MMUv3 初始化序

initialize_mmu 宏中的代码设MMUv3 内存映射，与 MMUv2 固定内存映射完全相同。根CONFIG_INITIALIZE_XTENSA_MMU_INSIDE_VMLINUX 符号，这段代码位于它被链接到的地址处（符号未定义），或不位于（符号已定义），因此它需要是位置无关的（position-independent）
该代码有以下假设
  - 此代码片段仅MMU v3 上运行  - TLB 处于其复位状态  - ITLBCFG DTLBCFG 为零（复位状态）  - RASID 0x04030201（复位状态）  - PS.RING 为零（复位状态）  - LITBASE 为零（复位状态，PC 相对字面量）；需要是 PIC
TLB 设置按以下步骤进行
  图例（Legend）：

    - VA = 虚拟地址（其两个高位半字节）    - PA = 物理地址（其两个高位半字节）    - pc = 包含此代码的物理范围
2 步之后，我们跳转到范0x40000000..0x5fffffff 0x00000000..0x1fffffff 中的虚拟地址，取决于内核是加载在 0x40000000 以下还是以上。该地址对应于此代码中要执行的下一条指令。第 4 步之后，我们跳转到此代码的预期（链接）地址。下面的方案假设内核加载0x40000000 以下
 ====== =====  =====  =====  =====   ====== =====  =====
 - Step0  Step1  Step2  Step3          Step4  Step5

   VA      PA     PA     PA     PA     VA      PA     PA
 ====== =====  =====  =====  =====   ====== =====  =====
 E0..FF -> E0  -> E0  -> E0          F0..FF -> F0  -> F0
 C0..DF -> C0  -> C0  -> C0          E0..EF -> F0  -> F0
 A0..BF -> A0  -> A0  -> A0          D8..DF -> 00  -> 00
 80..9F -> 80  -> 80  -> 80          D0..D7 -> 00  -> 00
 60..7F -> 60  -> 60  -> 60
 40..5F -> 40         -> pc  -> pc   40..5F -> pc
 20..3F -> 20  -> 20  -> 20
 00..1F -> 00  -> 00  -> 00
 ====== =====  =====  =====  =====   ====== =====  =====

IO 外设的默认位置在 0xf0000000 以上。可以使用设备树 simple-bus 节点中的 “ranges属性来更改。有simple-bus 节点语法和语义的细节，请参见 Devicetree Specification 4.5 节。适用以下限制
1. 只考虑顶层 simple-bus 节点

2. 只考虑一个（第一个）simple-bus 节点

3. 不支持空“ranges属
4. 只考虑 “ranges属性中的第一个三元组

5. parent-bus-address 值向下舍入到最近的 256MB 边界

6. IO 区域覆盖 parent-bus-address 的整256MB 段；“ranges三元组的长度字段被忽

## MMUv3 地址空间布局


```

                        Symbol                   VADDR       Size
  +------------------+
  | Userspace        |                           0x00000000  TASK_SIZE
  +------------------+                           0x40000000
  +------------------+
  | Page table       |  XCHAL_PAGE_TABLE_VADDR   0x80000000  XCHAL_PAGE_TABLE_SIZE
  +------------------+
  | KASAN shadow map |  KASAN_SHADOW_START       0x80400000  KASAN_SHADOW_SIZE
  +------------------+                           0x8e400000
  +------------------+
  | VMALLOC area     |  VMALLOC_START            0xc0000000  128MB - 64KB
  +------------------+  VMALLOC_END
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_1           0xc8000000  DCACHE_WAY_SIZE
  | remap area 1     |
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_2                       DCACHE_WAY_SIZE
  | remap area 2     |
  +------------------+
  +------------------+
  | KMAP area        |  PKMAP_BASE                           PTRS_PER_PTE *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  |                  |                                       (4MB * DCACHE_N_COLORS)
  +------------------+
  | Atomic KMAP area |  FIXADDR_START                        KM_TYPE_NR *
  |                  |                                       NR_CPUS *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  +------------------+  FIXADDR_TOP              0xcffff000
  +------------------+
  | Cached KSEG      |  XCHAL_KSEG_CACHED_VADDR  0xd0000000  128MB
  +------------------+
  | Uncached KSEG    |  XCHAL_KSEG_BYPASS_VADDR  0xd8000000  128MB
  +------------------+
  | Cached KIO       |  XCHAL_KIO_CACHED_VADDR   0xe0000000  256MB
  +------------------+
  | Uncached KIO     |  XCHAL_KIO_BYPASS_VADDR   0xf0000000  256MB
  +------------------+


```
```

                        Symbol                   VADDR       Size
  +------------------+
  | Userspace        |                           0x00000000  TASK_SIZE
  +------------------+                           0x40000000
  +------------------+
  | Page table       |  XCHAL_PAGE_TABLE_VADDR   0x80000000  XCHAL_PAGE_TABLE_SIZE
  +------------------+
  | KASAN shadow map |  KASAN_SHADOW_START       0x80400000  KASAN_SHADOW_SIZE
  +------------------+                           0x8e400000
  +------------------+
  | VMALLOC area     |  VMALLOC_START            0xa0000000  128MB - 64KB
  +------------------+  VMALLOC_END
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_1           0xa8000000  DCACHE_WAY_SIZE
  | remap area 1     |
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_2                       DCACHE_WAY_SIZE
  | remap area 2     |
  +------------------+
  +------------------+
  | KMAP area        |  PKMAP_BASE                           PTRS_PER_PTE *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  |                  |                                       (4MB * DCACHE_N_COLORS)
  +------------------+
  | Atomic KMAP area |  FIXADDR_START                        KM_TYPE_NR *
  |                  |                                       NR_CPUS *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  +------------------+  FIXADDR_TOP              0xaffff000
  +------------------+
  | Cached KSEG      |  XCHAL_KSEG_CACHED_VADDR  0xb0000000  256MB
  +------------------+
  | Uncached KSEG    |  XCHAL_KSEG_BYPASS_VADDR  0xc0000000  256MB
  +------------------+
  +------------------+
  | Cached KIO       |  XCHAL_KIO_CACHED_VADDR   0xe0000000  256MB
  +------------------+
  | Uncached KIO     |  XCHAL_KIO_BYPASS_VADDR   0xf0000000  256MB
  +------------------+


```
```

                        Symbol                   VADDR       Size
  +------------------+
  | Userspace        |                           0x00000000  TASK_SIZE
  +------------------+                           0x40000000
  +------------------+
  | Page table       |  XCHAL_PAGE_TABLE_VADDR   0x80000000  XCHAL_PAGE_TABLE_SIZE
  +------------------+
  | KASAN shadow map |  KASAN_SHADOW_START       0x80400000  KASAN_SHADOW_SIZE
  +------------------+                           0x8e400000
  +------------------+
  | VMALLOC area     |  VMALLOC_START            0x90000000  128MB - 64KB
  +------------------+  VMALLOC_END
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_1           0x98000000  DCACHE_WAY_SIZE
  | remap area 1     |
  +------------------+
  | Cache aliasing   |  TLBTEMP_BASE_2                       DCACHE_WAY_SIZE
  | remap area 2     |
  +------------------+
  +------------------+
  | KMAP area        |  PKMAP_BASE                           PTRS_PER_PTE *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  |                  |                                       (4MB * DCACHE_N_COLORS)
  +------------------+
  | Atomic KMAP area |  FIXADDR_START                        KM_TYPE_NR *
  |                  |                                       NR_CPUS *
  |                  |                                       DCACHE_N_COLORS *
  |                  |                                       PAGE_SIZE
  +------------------+  FIXADDR_TOP              0x9ffff000
  +------------------+
  | Cached KSEG      |  XCHAL_KSEG_CACHED_VADDR  0xa0000000  512MB
  +------------------+
  | Uncached KSEG    |  XCHAL_KSEG_BYPASS_VADDR  0xc0000000  512MB
  +------------------+
  | Cached KIO       |  XCHAL_KIO_CACHED_VADDR   0xe0000000  256MB
  +------------------+
  | Uncached KIO     |  XCHAL_KIO_BYPASS_VADDR   0xf0000000  256MB
  +------------------+


```
