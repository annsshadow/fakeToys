## MMUv3 鍒濆鍖栧簭鍒?

initialize_mmu 瀹忎腑鐨勪唬鐮佽缃?MMUv3 鍐呭瓨鏄犲皠锛屼笌 MMUv2 鍥哄畾鍐呭瓨鏄犲皠瀹屽叏鐩稿悓銆傛牴鎹?CONFIG_INITIALIZE_XTENSA_MMU_INSIDE_VMLINUX 绗﹀彿锛岃繖娈典唬鐮佷綅浜庡畠琚摼鎺ュ埌鐨勫湴鍧€澶勶紙绗﹀彿鏈畾涔夛級锛屾垨涓嶄綅浜庯紙绗﹀彿宸插畾涔夛級锛屽洜姝ゅ畠闇€瑕佹槸浣嶇疆鏃犲叧鐨勶紙position-independent锛夈€?
璇ヤ唬鐮佹湁浠ヤ笅鍋囪锛?
  - 姝や唬鐮佺墖娈典粎鍦?MMU v3 涓婅繍琛屻€?  - TLB 澶勪簬鍏跺浣嶇姸鎬併€?  - ITLBCFG 鍜?DTLBCFG 涓洪浂锛堝浣嶇姸鎬侊級銆?  - RASID 涓?0x04030201锛堝浣嶇姸鎬侊級銆?  - PS.RING 涓洪浂锛堝浣嶇姸鎬侊級銆?  - LITBASE 涓洪浂锛堝浣嶇姸鎬侊紝PC 鐩稿瀛楅潰閲忥級锛涢渶瑕佹槸 PIC銆?
TLB 璁剧疆鎸変互涓嬫楠よ繘琛屻€?
  鍥句緥锛圠egend锛夛細

    - VA = 铏氭嫙鍦板潃锛堝叾涓や釜楂樹綅鍗婂瓧鑺傦級锛?    - PA = 鐗╃悊鍦板潃锛堝叾涓や釜楂樹綅鍗婂瓧鑺傦級锛?    - pc = 鍖呭惈姝や唬鐮佺殑鐗╃悊鑼冨洿锛?
绗?2 姝ヤ箣鍚庯紝鎴戜滑璺宠浆鍒拌寖鍥?0x40000000..0x5fffffff 鎴?0x00000000..0x1fffffff 涓殑铏氭嫙鍦板潃锛屽彇鍐充簬鍐呮牳鏄姞杞藉湪 0x40000000 浠ヤ笅杩樻槸浠ヤ笂銆傝鍦板潃瀵瑰簲浜庢浠ｇ爜涓鎵ц鐨勪笅涓€鏉℃寚浠ゃ€傜 4 姝ヤ箣鍚庯紝鎴戜滑璺宠浆鍒版浠ｇ爜鐨勯鏈燂紙閾炬帴锛夊湴鍧€銆備笅闈㈢殑鏂规鍋囪鍐呮牳鍔犺浇鍦?0x40000000 浠ヤ笅銆?
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

IO 澶栬鐨勯粯璁や綅缃湪 0xf0000000 浠ヤ笂銆傚彲浠ヤ娇鐢ㄨ澶囨爲 simple-bus 鑺傜偣涓殑 鈥渞anges鈥?灞炴€ф潵鏇存敼銆傛湁鍏?simple-bus 鑺傜偣璇硶鍜岃涔夌殑缁嗚妭锛岃鍙傝 Devicetree Specification 绗?4.5 鑺傘€傞€傜敤浠ヤ笅闄愬埗锛?
1. 鍙€冭檻椤跺眰 simple-bus 鑺傜偣

2. 鍙€冭檻涓€涓紙绗竴涓級simple-bus 鑺傜偣

3. 涓嶆敮鎸佺┖鐨?鈥渞anges鈥?灞炴€?
4. 鍙€冭檻 鈥渞anges鈥?灞炴€т腑鐨勭涓€涓笁鍏冪粍

5. parent-bus-address 鍊煎悜涓嬭垗鍏ュ埌鏈€杩戠殑 256MB 杈圭晫

6. IO 鍖哄煙瑕嗙洊 parent-bus-address 鐨勬暣涓?256MB 娈碉紱鈥渞anges鈥?涓夊厓缁勭殑闀垮害瀛楁琚拷鐣?

## MMUv3 鍦板潃绌洪棿甯冨眬


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
