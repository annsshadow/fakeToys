
## 鍐呭瓨绠＄悊


## 閲囩敤 4 绾ч〉琛ㄧ殑瀹屾暣铏氭嫙鍐呭瓨鏄犲皠


 - 鍍?"-23 TB" 杩欐牱鐨勮礋鍦板潃鏄粷瀵瑰湴鍧€锛堜互瀛楄妭璁★級锛屼粠 64 浣嶅湴鍧€绌洪棿鐨勯《閮ㄥ悜涓嬭鏁般€傚悓鏃朵互缁濆鍦板潃鍜岃窛椤堕儴鐨勮窛绂昏繖涓ょ鏂瑰紡鏉ョ湅甯冨眬锛屼細鏇村鏄撶悊瑙ｃ€?
   渚嬪 0xffffe90000000000 == -23 TB锛屽畠姣?64 浣嶅湴鍧€绌洪棿鐨勯《閮紙ffffffffffffffff锛変綆 23 TB銆?
   娉ㄦ剰锛屽綋鎴戜滑瓒婇潬杩戝湴鍧€绌洪棿椤堕儴鏃讹紝璁℃暟鍗曚綅浼氫粠 TB 鍙樹负 GB锛屽啀鍒?MB/KB銆?
 - "16M TB" 涔嶄竴鐪嬪彲鑳芥湁浜涘鎬紝浣嗗畠姣?"16 EB" 鏇寸洿瑙傚湴琛ㄧず澶у皬鈥斺€斿緢灏戜細鏈変汉涓€鐪煎氨璁ゅ嚭 16 EB 鏄?16 鑹惧瓧鑺傦紙exabytes锛夈€傚畠涔熷緢濂藉湴灞曠ず浜?64 浣嶅湴鍧€绌洪棿鍒板簳鏈夊涔堝法澶с€?
```

  ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
  ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 00007fffffffefff | ~128 TB | user-space virtual memory, different per mm
   00007ffffffff000 | ~128    TB | 00007fffffffffff |    4 kB | ... guard hole
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0000800000000000 | +128    TB | 7fffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -8 EB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM relaxes canonicallity check allowing to create aliases
                    |            |                  |         | for userspace memory here.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   8000000000000000 |   -8    EB | ffff7fffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -128 TB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM_SUP relaxes canonicallity check allowing to create
                    |            |                  |         | aliases for kernel memory here.
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ffff800000000000 | -128    TB | ffff87ffffffffff |    8 TB | ... guard hole, also reserved for hypervisor
   ffff880000000000 | -120    TB | ffff887fffffffff |  0.5 TB | LDT remap for PTI
   ffff888000000000 | -119.5  TB | ffffc87fffffffff |   64 TB | direct mapping of all physical memory (page_offset_base)
   ffffc88000000000 |  -55.5  TB | ffffc8ffffffffff |  0.5 TB | ... unused hole
   ffffc90000000000 |  -55    TB | ffffe8ffffffffff |   32 TB | vmalloc/ioremap space (vmalloc_base)
   ffffe90000000000 |  -23    TB | ffffe9ffffffffff |    1 TB | ... unused hole
   ffffea0000000000 |  -22    TB | ffffeaffffffffff |    1 TB | virtual memory map (vmemmap_base)
   ffffeb0000000000 |  -21    TB | ffffebffffffffff |    1 TB | ... unused hole
   ffffec0000000000 |  -20    TB | fffffbffffffffff |   16 TB | KASAN shadow memory
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 56-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   fffffc0000000000 |   -4    TB | fffffdffffffffff |    2 TB | ... unused hole
                    |            |                  |         | vaddr_end for KASLR
   fffffe0000000000 |   -2    TB | fffffe7fffffffff |  0.5 TB | cpu_entry_area mapping
   fffffe8000000000 |   -1.5  TB | fffffeffffffffff |  0.5 TB | ... unused hole
   ffffff0000000000 |   -1    TB | ffffff7fffffffff |  0.5 TB | %esp fixup stacks
   ffffff8000000000 | -512    GB | ffffffeeffffffff |  444 GB | ... unused hole
   ffffffef00000000 |  -68    GB | fffffffeffffffff |   64 GB | EFI region mapping space
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | ... unused hole
   ffffffff80000000 |   -2    GB | ffffffff9fffffff |  512 MB | kernel text mapping, mapped to physical address 0
   ffffffff80000000 |-2048    MB |                  |         |
   ffffffffa0000000 |-1536    MB | fffffffffeffffff | 1520 MB | module mapping space
   ffffffffff000000 |  -16    MB |                  |         |
      FIXADDR_START | ~-11    MB | ffffffffff5fffff | ~0.5 MB | kernel-internal fixmap range, variable size and offset
   ffffffffff600000 |  -10    MB | ffffffffff600fff |    4 kB | legacy vsyscall ABI
   ffffffffffe00000 |   -2    MB | ffffffffffffffff |    2 MB | ... unused hole
  __________________|____________|__________________|_________|___________________________________________________________


```
## 閲囩敤 5 绾ч〉琛ㄧ殑瀹屾暣铏氭嫙鍐呭瓨鏄犲皠


 - 瀵逛簬 56 浣嶅湴鍧€锛岀敤鎴风┖闂村唴瀛樻墿澶т簡 512 鍊嶏紝浠?0.125 PB 澧炲姞鍒?64 PB銆傛墍鏈夌殑鍐呮牳鏄犲皠閮戒笅绉诲埌浜?-64 PB 鐨勮捣濮嬪亸绉诲锛屽苟涓旇澶氬尯鍩熶篃杩涜浜嗘墿灞曪紝浠ユ敮鎸佸ぇ寰楀鐨勭墿鐞嗗唴瀛樸€?
```

  ========================================================================================================================
      Start addr    |   Offset   |     End addr     |  Size   | VM area description
  ========================================================================================================================
                    |            |                  |         |
   0000000000000000 |    0       | 00fffffffffff000 |  ~64 PB | user-space virtual memory, different per mm
   00fffffffffff000 |  ~64    PB | 00ffffffffffffff |    4 kB | ... guard hole
  __________________|____________|__________________|_________|___________________________________________________________
                    |            |                  |         |
   0100000000000000 |  +64    PB | 7fffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -8EB TB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM relaxes canonicallity check allowing to create aliases
                    |            |                  |         | for userspace memory here.
  __________________|____________|__________________|_________|___________________________________________________________
                                                              |
                                                              | Kernel-space virtual memory, shared between all processes:
  ____________________________________________________________|___________________________________________________________
   8000000000000000 |   -8    EB | feffffffffffffff |   ~8 EB | ... huge, almost 63 bits wide hole of non-canonical
                    |            |                  |         |     virtual memory addresses up to the -64 PB
                    |            |                  |         |     starting offset of kernel mappings.
                    |            |                  |         |
                    |            |                  |         | LAM_SUP relaxes canonicallity check allowing to create
                    |            |                  |         | aliases for kernel memory here.
  ____________________________________________________________|___________________________________________________________
                    |            |                  |         |
   ff00000000000000 |  -64    PB | ff0fffffffffffff |    4 PB | ... guard hole, also reserved for hypervisor
   ff10000000000000 |  -60    PB | ff10ffffffffffff | 0.25 PB | LDT remap for PTI
   ff11000000000000 |  -59.75 PB | ff90ffffffffffff |   32 PB | direct mapping of all physical memory (page_offset_base)
   ff91000000000000 |  -27.75 PB | ff9fffffffffffff | 3.75 PB | ... unused hole
   ffa0000000000000 |  -24    PB | ffd1ffffffffffff | 12.5 PB | vmalloc/ioremap space (vmalloc_base)
   ffd2000000000000 |  -11.5  PB | ffd3ffffffffffff |  0.5 PB | ... unused hole
   ffd4000000000000 |  -11    PB | ffd5ffffffffffff |  0.5 PB | virtual memory map (vmemmap_base)
   ffd6000000000000 |  -10.5  PB | ffdeffffffffffff | 2.25 PB | ... unused hole
   ffdf000000000000 |   -8.25 PB | fffffbffffffffff |   ~8 PB | KASAN shadow memory
  __________________|____________|__________________|_________|____________________________________________________________
                                                              |
                                                              | Identical layout to the 47-bit one from here on:
  ____________________________________________________________|____________________________________________________________
                    |            |                  |         |
   fffffc0000000000 |   -4    TB | fffffdffffffffff |    2 TB | ... unused hole
                    |            |                  |         | vaddr_end for KASLR
   fffffe0000000000 |   -2    TB | fffffe7fffffffff |  0.5 TB | cpu_entry_area mapping
   fffffe8000000000 |   -1.5  TB | fffffeffffffffff |  0.5 TB | ... unused hole
   ffffff0000000000 |   -1    TB | ffffff7fffffffff |  0.5 TB | %esp fixup stacks
   ffffff8000000000 | -512    GB | ffffffeeffffffff |  444 GB | ... unused hole
   ffffffef00000000 |  -68    GB | fffffffeffffffff |   64 GB | EFI region mapping space
   ffffffff00000000 |   -4    GB | ffffffff7fffffff |    2 GB | ... unused hole
   ffffffff80000000 |   -2    GB | ffffffff9fffffff |  512 MB | kernel text mapping, mapped to physical address 0
   ffffffff80000000 |-2048    MB |                  |         |
   ffffffffa0000000 |-1536    MB | fffffffffeffffff | 1520 MB | module mapping space
   ffffffffff000000 |  -16    MB |                  |         |
      FIXADDR_START | ~-11    MB | ffffffffff5fffff | ~0.5 MB | kernel-internal fixmap range, variable size and offset
   ffffffffff600000 |  -10    MB | ffffffffff600fff |    4 kB | legacy vsyscall ABI
   ffffffffffe00000 |   -2    MB | ffffffffffffffff |    2 MB | ... unused hole
  __________________|____________|__________________|_________|___________________________________________________________


```
璇ユ灦鏋勫畾涔変簡涓€涓?64 浣嶈櫄鎷熷湴鍧€銆傚疄鐜板彲浠ユ敮鎸佹洿灏戙€傜洰鍓嶆敮鎸佺殑鏄?48 浣嶅拰 57 浣嶈櫄鎷熷湴鍧€銆備粠绗?63 浣嶅埌鏈€楂樻湁鏁堝疄鐜颁綅涔嬮棿鐨勪綅杩涜绗﹀彿鎵╁睍銆傚鏋滀綘灏嗗畠浠В閲婁负鏃犵鍙锋暟锛岃繖灏卞湪鐢ㄦ埛绌洪棿鍜屽唴鏍稿湴鍧€涔嬮棿閫犳垚浜嗕竴涓┖娲炪€?
鐩存帴鏄犲皠瑕嗙洊浜嗙郴缁熶腑鐩村埌鏈€楂樺唴瀛樺湴鍧€鐨勬墍鏈夊唴瀛橈紙杩欐剰鍛崇潃鍦ㄦ煇浜涙儏鍐典笅瀹冧篃鍙兘鍖呭惈 PCI 鍐呭瓨绌烘礊锛夈€?
鎴戜滑灏?EFI 杩愯鏃舵湇鍔℃槧灏勫湪 'efi_pgd' PGD 涓紝浣嶄簬涓€涓?64GB 澶у皬鐨勮櫄鎷熷唴瀛樼獥鍙ｄ腑锛堣繖涓ぇ灏忔槸浠绘剰鐨勶紝濡傛灉灏嗘潵闇€瑕佸彲浠ヨ皟澶э級銆傝繖浜涙槧灏勪笉灞炰簬浠讳綍鍏跺畠鍐呮牳 PGD 鐨勪竴閮ㄥ垎锛屽苟涓斾粎鍦?EFI 杩愯鏃惰皟鐢ㄦ湡闂村彲鐢ㄣ€?
娉ㄦ剰锛屽鏋滃惎鐢ㄤ簡 CONFIG_RANDOMIZE_MEMORY锛屽垯鎵€鏈夌墿鐞嗗唴瀛樼殑鐩存帴鏄犲皠銆乿malloc/ioremap 绌洪棿浠ュ強铏氭嫙鍐呭瓨鏄犲皠閮戒細琚殢鏈哄寲銆傚畠浠殑椤哄簭寰椾互淇濈暀锛屼絾瀹冧滑鐨勫熀鍧€浼氬湪鍚姩鏃╂湡琚亸绉汇€?
鍦ㄤ慨鏀硅繖閲岀殑浠讳綍鍐呭鏃讹紝瑕侀潪甯稿皬蹇?KASLR銆侹ASLR 鍦板潃鑼冨洿涓嶅緱涓庨櫎 KASAN 褰卞瓙鍖轰互澶栫殑浠讳綍涓滆タ閲嶅彔锛岃繖鏄纭殑锛屽洜涓?KASAN 浼氱鐢?KASLR銆?
瀵逛簬 4 绾у拰 5 绾т袱绉嶅竷灞€锛屾渶鍚庝竴涓?2MB 绌烘礊澶勭殑 KSTACK_ERASE_POISON 鍊间负锛歠fffffffff4111
