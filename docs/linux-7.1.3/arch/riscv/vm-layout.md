
## RISC-V Linux 涓婄殑铏氭嫙鍐呭瓨甯冨眬


:Author: Alexandre Ghiti <alex@ghiti.fr>
:Date: 2021 骞?2 鏈?12 鏃?
鏈枃妗ｆ弿杩?RISC-V Linux 鍐呮牳鎵€浣跨敤鐨勮櫄鎷熷唴瀛樺竷灞€銆?
## RISC-V Linux 鍐呮牳 32 浣?

### RISC-V Linux 鍐呮牳 SV32


寰呭姙

## RISC-V Linux 鍐呮牳 64 浣?

RISC-V 鐗规潈鏋舵瀯鏂囨。鎸囧嚭锛?4 浣嶅湴鍧€鈥滃繀椤诲皢绗?63鈥?8 浣嶅叏閮ㄧ瓑浜庣 47 浣嶏紝鍚﹀垯浼氬彂鐢熼〉閿欒寮傚父銆傗€濓細
杩欏氨鎶婅櫄鎷熷湴鍧€绌洪棿鍒嗗壊涓鸿涓€涓瀬澶х┖娲為殧寮€鐨勪袱鍗婏紝涓嬪崐閮ㄥ垎鏄敤鎴风┖闂存墍鍦ㄧ殑浣嶇疆锛屼笂鍗婇儴鍒嗘槸 RISC-V Linux 鍐呮牳鎵€鍦ㄧ殑浣嶇疆銆?
### RISC-V Linux 鍐呮牳 SV39


```
  ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
  ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 0000003fffffffff |  256 GB | user-space virtual memory, different per mm
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0000004000000000 | +256    GB | ffffffbfffffffff | ~16M TB | ... huge, almost 64 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -256 GB
                    |            |                  |         |     starting offset of kernel mappings.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ffffffc4fea00000 | -236    GB | ffffffc4feffffff |    6 MB | fixmap
   ffffffc4ff000000 | -236    GB | ffffffc4ffffffff |   16 MB | PCI io
   ffffffc500000000 | -236    GB | ffffffc5ffffffff |    4 GB | vmemmap
   ffffffc600000000 | -232    GB | ffffffd5ffffffff |   64 GB | vmalloc/ioremap space
   ffffffd600000000 | -168    GB | fffffff5ffffffff |  128 GB | direct mapping of all physical memory
                    |            |                  |         |
   fffffff700000000 |  -36    GB | fffffffeffffffff |   32 GB | kasan
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              |
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | modules, BPF
   ffffffff80000000 |   -2    GB | ffffffffffffffff |    2 GB | kernel
  __________________|____________|__________________|_________|____________________________________________________________
```
### RISC-V Linux 鍐呮牳 SV48


```
 ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
 ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 00007fffffffffff |  128 TB | user-space virtual memory, different per mm
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0000800000000000 | +128    TB | ffff7fffffffffff | ~16M TB | ... huge, almost 64 bits wide hole of non-canonical
                    |            |                  |         | virtual memory addresses up to the -128 TB
                    |            |                  |         | starting offset of kernel mappings.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ffff8d7ffea00000 |  -114.5 TB | ffff8d7ffeffffff |    6 MB | fixmap
   ffff8d7fff000000 |  -114.5 TB | ffff8d7fffffffff |   16 MB | PCI io
   ffff8d8000000000 |  -114.5 TB | ffff8f7fffffffff |    2 TB | vmemmap
   ffff8f8000000000 |  -112.5 TB | ffffaf7fffffffff |   32 TB | vmalloc/ioremap space
   ffffaf8000000000 |  -80.5  TB | ffffef7fffffffff |   64 TB | direct mapping of all physical memory
   ffffef8000000000 |  -16.5  TB | fffffffeffffffff | 16.5 TB | kasan
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 39-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | modules, BPF
   ffffffff80000000 |   -2    GB | ffffffffffffffff |    2 GB | kernel
  __________________|____________|__________________|_________|____________________________________________________________
```
### RISC-V Linux 鍐呮牳 SV57


```
 ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
 ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |   0        | 00ffffffffffffff |   64 PB | user-space virtual memory, different per mm
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0100000000000000 | +64     PB | feffffffffffffff | ~16K PB | ... huge, almost 64 bits wide hole of non-canonical
                    |            |                  |         | virtual memory addresses up to the -64 PB
                    |            |                  |         | starting offset of kernel mappings.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ff1bfffffea00000 | -57     PB | ff1bfffffeffffff |    6 MB | fixmap
   ff1bffffff000000 | -57     PB | ff1bffffffffffff |   16 MB | PCI io
   ff1c000000000000 | -57     PB | ff1fffffffffffff |    1 PB | vmemmap
   ff20000000000000 | -56     PB | ff5fffffffffffff |   16 PB | vmalloc/ioremap space
   ff60000000000000 | -40     PB | ffdeffffffffffff |   32 PB | direct mapping of all physical memory
   ffdf000000000000 |  -8     PB | fffffffeffffffff |    8 PB | kasan
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 39-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   ffffffff00000000 |  -4     GB | ffffffff7fffffff |    2 GB | modules, BPF
   ffffffff80000000 |  -2     GB | ffffffffffffffff |    2 GB | kernel
  __________________|____________|__________________|_________|____________________________________________________________
```
