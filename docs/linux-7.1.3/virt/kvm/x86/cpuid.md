
## KVM CPUID bits


:Author: Glauber Costa <glommer@gmail.com>

杩愯鍦?KVM 瀹夸富鏈轰笂鐨勫鎴锋満锛屽彲浠ラ€氳繃 cpuid 妫€鏌ュ叾閮ㄥ垎鐗规€с€傝繖骞朵笉鎬绘槸淇濊瘉鐢熸晥锛屽洜涓虹敤鎴风┖闂村彲浠ュ湪鍚姩瀹㈡埛鏈轰箣鍓嶏紝灏嗛儴鍒嗙敋鑷冲叏閮?KVM 鐩稿叧鐨?cpuid 鐗规€у睆钄芥帀銆?
KVM cpuid 鍑芥暟涓猴細

function: KVM_CPUID_SIGNATURE (0x40000000)

```

   eax = 0x40000001
   ebx = 0x4b4d564b
   ecx = 0x564b4d56
   edx = 0x4d

```
娉ㄦ剰锛宔bx銆乪cx 鍜?edx 涓殑杩欎釜鍊煎搴斾簬瀛楃涓?"KVMKVMKVM"銆?eax 涓殑鍊煎搴斾簬鏈?leaf 涓瓨鍦ㄧ殑鏈€澶?cpuid 鍑芥暟锛岃嫢鏈潵澧炲姞浜嗘洿澶氬嚱鏁帮紝璇ュ€间細闅忎箣鏇存柊銆?鍙﹁娉ㄦ剰锛屾棫鐗堝涓绘満灏?eax 鍊艰涓?0x0銆傚簲灏嗗叾瑙ｉ噴涓哄€?0x40000001銆?鏈嚱鏁扮敤浜庢煡璇?KVM cpuid leaf 鏄惁瀛樺湪銆?
function: define KVM_CPUID_FEATURES (0x40000001)

```

          ebx, ecx
          eax = an OR'ed group of (1 << flag)

```
鍏朵腑 `flag` 瀹氫箟濡備笅锛?
================================== =========== ================================
flag                               value       meaning
================================== =========== ================================
KVM_FEATURE_CLOCKSOURCE            0           kvmclock 鍦?msrs 0x11 鍜?0x12
                                               澶勫彲鐢?
KVM_FEATURE_NOP_IO_DELAY           1           鏃犻渶鍦?PIO 鎿嶄綔涓婃墽琛屽欢杩?
KVM_FEATURE_MMU_OP                 2           宸插簾寮?
KVM_FEATURE_CLOCKSOURCE2           3           kvmclock 鍦?msrs 0x4b564d00 鍜?                                               0x4b564d01 澶勫彲鐢?
KVM_FEATURE_ASYNC_PF               4           鍙€氳繃鍐欏叆 msr 0x4b564d02 鍚敤
                                               async pf

KVM_FEATURE_STEAL_TIME             5           鍙€氳繃鍐欏叆 msr 0x4b564d03 鍚敤
                                               steal time

KVM_FEATURE_PV_EOI                 6           鍙€氳繃鍐欏叆 msr 0x4b564d04 鍚敤
                                               鍗婅櫄鎷熷寲 end of interrupt 澶勭悊绋嬪簭

KVM_FEATURE_PV_UNHALT              7           瀹㈡埛鏈哄湪鍚敤鍗婅櫄鎷熷寲鑷棆閿佹敮鎸佸墠
                                               妫€鏌ヨ鐗规€т綅

KVM_FEATURE_PV_TLB_FLUSH           9           瀹㈡埛鏈哄湪鍚敤鍗婅櫄鎷熷寲 tlb flush 鍓?                                               妫€鏌ヨ鐗规€т綅

KVM_FEATURE_ASYNC_PF_VMEXIT        10          鍙€氳繃鍦ㄥ啓鍏?msr 0x4b564d02 鏃?                                               璁剧疆浣?2 鏉ュ惎鐢ㄥ崐铏氭嫙鍖?async PF
                                               VM EXIT

KVM_FEATURE_PV_SEND_IPI            11          瀹㈡埛鏈哄湪鍚敤鍗婅櫄鎷熷寲鍙戦€?IPI 鍓?                                               妫€鏌ヨ鐗规€т綅

KVM_FEATURE_POLL_CONTROL           12          鍙€氳繃鍐欏叆 msr 0x4b564d05 绂佺敤
                                               瀹夸富鏈轰晶瀵?HLT 鐨勮疆璇?
KVM_FEATURE_PV_SCHED_YIELD         13          瀹㈡埛鏈哄湪浣跨敤鍗婅櫄鎷熷寲 sched yield
                                               鍓嶆鏌ヨ鐗规€т綅

KVM_FEATURE_ASYNC_PF_INT           14          瀹㈡埛鏈哄湪浣跨敤绗簩涓?async pf 鎺у埗
                                               msr 0x4b564d06 浠ュ強 async pf 纭
                                               msr 0x4b564d07 鍓嶆鏌ヨ鐗规€т綅

KVM_FEATURE_MSI_EXT_DEST_ID        15          瀹㈡埛鏈哄湪 MSI 鍦板潃浣?11-5 涓娇鐢?                                               鎵╁睍鐩爣 ID 浣嶅墠妫€鏌ヨ鐗规€т綅

KVM_FEATURE_HC_MAP_GPA_RANGE       16          瀹㈡埛鏈哄湪浣跨敤 map gpa range hypercall
                                               閫氱煡椤电姸鎬佸彉鏇村墠妫€鏌ヨ鐗规€т綅

KVM_FEATURE_MIGRATION_CONTROL      17          瀹㈡埛鏈哄湪浣跨敤 MSR_KVM_MIGRATION_CONTROL
                                               鍓嶆鏌ヨ鐗规€т綅

KVM_FEATURE_CLOCKSOURCE_STABLE_BIT 24          鑻ュ鎴锋満渚?kvmclock 涓鏈熶笉浼氬嚭鐜?                                               per-cpu 鍋忓樊锛屽涓绘満灏嗗彂鍑鸿鍛?================================== =========== ================================

```

      edx = an OR'ed group of (1 << flag)

```
杩欓噷鐨?`flag` 瀹氫箟濡備笅锛?
================== ============ =================================
flag               value        meaning
================== ============ =================================
KVM_HINTS_REALTIME 0            瀹㈡埛鏈烘鏌ヨ鐗规€т綅浠ョ‘瀹?vCPU 涓嶄細
                                鍦ㄦ棤闄愰暱鐨勬椂闂村唴琚姠鍗狅紝浠庤€屽厑璁歌繘琛?                                浼樺寲
================== ============ =================================
