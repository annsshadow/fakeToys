
## KVM 閿佹杩?

### 1. 鍔犻攣椤哄簭锛圓cquisition Orders锛?

浜掓枼浣擄紙mutex锛夌殑鍔犻攣椤哄簭濡備笅锛?
- cpus_read_lock() 鍦?kvm_lock 涔嬪鑾峰彇

- kvm_usage_lock 鍦?cpus_read_lock() 涔嬪鑾峰彇

- kvm->lock 鍦?vcpu->mutex 涔嬪鑾峰彇

- kvm->lock 鍦?kvm->slots_lock 鍜?kvm->irq_lock 涔嬪鑾峰彇

- vcpu->mutex 鍦?kvm->slots_lock 鍜?kvm->slots_arch_lock 涔嬪鑾峰彇

- kvm->slots_lock 鍦?kvm->irq_lock 涔嬪鑾峰彇锛屽敖绠″悓鏃惰幏鍙栧畠浠殑鎯呭喌鐩稿綋缃曡銆?
- kvm->mn_active_invalidate_count 纭繚鎴愬鐨?invalidate_range_start() 鍜?  invalidate_range_end() 鍥炶皟浣跨敤鍚屼竴涓?memslots 鏁扮粍銆傚湪淇敼 memslots 鏃讹紝绛夊緟渚т細鑾峰彇
  kvm->slots_lock 鍜?kvm->slots_arch_lock锛屽洜姝?MMU 閫氱煡鍣紙notifier锛夌粷涓嶈兘鑾峰彇
  kvm->slots_lock 鎴?kvm->slots_arch_lock銆?
cpus_read_lock() 涓?kvm_lock 鐨勫叧绯伙細

- 灏界瀹樻柟椤哄簭瑙勫畾鍦?kvm_lock 涔嬪鑾峰彇 cpus_read_lock()锛屼絾杩欐牱鍋氭槸鏈夐棶棰樼殑锛屽洜涓哄緢瀹规槗鍦?  鎸佹湁 kvm_lock 鏃朵笉鐭ヤ笉瑙夊湴瑙﹀彂 cpus_read_lock()銆傞亶鍘?vm_list 鏃惰璋ㄦ厧锛屼緥濡傚敖鍙兘閬垮厤
  澶嶆潅鎿嶄綔銆?
瀵逛簬 SRCU锛?
- `synchronize_srcu(&kvm->srcu)` 鍦?kvm->lock銆乿cpu->mutex 鍜?kvm->slots_lock 鐨勪复鐣屽尯鍐呴儴
  璋冪敤銆傝繖浜涢攣**涓嶈兘**鍦?kvm->srcu 璇讳晶涓寸晫鍖哄唴閮ㄨ幏鍙栵紱涔熷氨鏄锛?```

      srcu_read_lock(&kvm->srcu);
      mutex_lock(&kvm->slots_lock);

```

- kvm->slots_arch_lock 鍙嶈€屽湪璋冪敤 `synchronize_srcu()` 涔嬪墠琚噴鏀俱€傚洜姝ゅ畠**鍙互**鍦?  kvm->srcu 璇讳晶涓寸晫鍖哄唴閮ㄨ幏鍙栵紝渚嬪鍦ㄥ鐞?vmexit 鏃躲€?
鍦?x86 涓婏細

- vcpu->mutex 鍦?kvm->arch.hyperv.hv_lock 鍜?kvm->arch.xen.xen_lock 涔嬪鑾峰彇

- kvm->arch.mmu_lock 鏄竴涓?rwlock锛沰vm->arch.tdp_mmu_pages_lock 鍜?  kvm->arch.mmu_unsync_pages_lock 鐨勪复鐣屽尯涔熷繀椤昏幏鍙?kvm->arch.mmu_lock

鍏朵粬涓€鍒囬兘鏄彾閿侊紙leaf锛夛細涓寸晫鍖哄唴涓嶈幏鍙栧叾浠栭攣銆?
### 2. 渚嬪锛圗xception锛?

蹇€熼〉閿欒锛團ast page fault锛夛細

蹇€熼〉閿欒鏄湪 x86 涓婁簬 mmu-lock 涔嬪淇瀹㈡埛鏈洪〉閿欒鐨勫揩閫熻矾寰勩€傜洰鍓嶏紝鍦ㄤ互涓嬩袱绉嶆儏鍐典笅
椤甸敊璇彲浠ユ槸蹇€熺殑锛?
1. 璁块棶杩借釜锛圓ccess Tracking锛夛細SPTE 涓嶅瓨鍦紝浣嗚鏍囪涓鸿闂拷韪€傝繖鎰忓懗鐫€鎴戜滑闇€瑕佹仮澶嶄繚瀛樼殑
   R/X 浣嶃€傝繖鍦ㄤ笅鏂囦腑浼氭洿璇︾粏鍦版弿杩般€?
2. 鍐欎繚鎶わ紙Write-Protection锛夛細SPTE 瀛樺湪涓旈敊璇敱鍐欎繚鎶ゅ紩璧枫€傝繖鎰忓懗鐫€鎴戜滑鍙渶鏀瑰彉 spte 鐨?   W 浣嶃€?
鎴戜滑鐢ㄦ潵閬垮厤鎵€鏈夌珵浜夌殑鏄?spte 涓婄殑 Host-writable 浣嶅拰 MMU-writable 浣嶏細

- Host-writable 琛ㄧず gfn 鍦ㄤ富鏈哄唴鏍搁〉琛ㄥ強鍏?KVM memslot 涓彲鍐欍€?- MMU-writable 琛ㄧず gfn 鍦ㄥ鎴锋満 mmu 涓彲鍐欙紝涓旀湭琚奖瀛愰〉鍐欎繚鎶ゃ€?
鍦ㄥ揩閫熼〉閿欒璺緞涓婏紝濡傛灉 spte.HOST_WRITEABLE = 1 涓?spte.WRITE_PROTECT = 1锛屾垜浠皢浣跨敤
cmpxchg 鍘熷瓙鍦拌缃?spte 鐨?W 浣嶏紝浠ユ仮澶嶄繚瀛樼殑 R/X 浣嶏紙瀵逛簬璁块棶杩借釜鐨?spte锛夛紝鎴栦袱鑰呴兘璁剧疆銆?杩欐槸瀹夊叏鐨勶紝鍥犱负瀵硅繖浜涗綅鐨勪换浣曟敼鍙橀兘鑳借 cmpxchg 妫€娴嬪埌銆?
浣嗘垜浠渶瑕佷粩缁嗘鏌ヤ互涓嬫儏鍐碉細

1) 浠?gfn 鍒?pfn 鐨勬槧灏?
浠?gfn 鍒?pfn 鐨勬槧灏勫彲鑳戒細鏀瑰彉锛屽洜涓烘垜浠彧鑳界‘淇濆湪 cmpxchg 鏈熼棿 pfn 涓嶈鏀瑰彉銆傝繖鏄竴涓?ABA 闂锛屼緥濡備笅闈㈢殑鎯呭喌浼氬彂鐢燂細

+------------------------------------------------------------------------+
**| 寮€濮嬫椂**
: |
|                                                                        |
|	gpte = gfn1                                                      |
|	gfn1 鍦ㄤ富鏈轰笂鏄犲皠鍒?pfn1                                         |
|	spte 鏄笌 gpte 瀵瑰簲鐨勫奖瀛愰〉琛ㄩ」锛屼笖                            |
|	spte = pfn1                                                      |
+------------------------------------------------------------------------+
| 鍦ㄥ揩閫熼〉閿欒璺緞涓婏細                                                   |
+------------------------------------+-----------------------------------+
| CPU 0:                             | CPU 1:                            |
+------------------------------------+-----------------------------------+
**| **
: |                                   |
|                                    |                                   |
|   old_spte = *spte;                |                                   |
+------------------------------------+-----------------------------------+
**|                                    | pfn1 琚崲鍑?*
: |
|                                    |                                   |
|                                    |    spte = 0;                      |
|                                    |                                   |
|                                    | pfn1 琚噸鏂板垎閰嶇粰 gfn2銆?         |
|                                    |                                   |
|                                    | gpte 琚鎴锋満鏀逛负鎸囧悜             |
**|                                    | gfn2**
: |
|                                    |                                   |
|                                    |    spte = pfn1;                   |
+------------------------------------+-----------------------------------+
**| **
: |
|                                                                        |
|   if (cmpxchg(spte, old_spte, old_spte+W)                              |
|	mark_page_dirty(vcpu->kvm, gfn1)                                 |
|            OOPS!!!                                                     |
+------------------------------------------------------------------------+

鎴戜滑瀵?gfn1 鍋氫簡鑴忔棩蹇楋紙dirty-log锛夛紝杩欐剰鍛崇潃 gfn2 鍦ㄨ剰浣嶅浘锛坉irty-bitmap锛変腑涓㈠け浜嗐€?
瀵逛簬鐩存帴 sp锛坉irect sp锛夛紝鎴戜滑鍙互杞绘槗閬垮厤瀹冿紝鍥犱负鐩存帴 sp 鐨?spte 鍥哄畾缁戝畾鍒?gfn銆傚浜庨棿鎺?sp锛坕ndirect sp锛夛紝涓轰簡绠€鍗曡捣瑙佹垜浠鐢ㄤ簡蹇€熼〉閿欒銆?
閽堝闂存帴 sp 鐨勪竴涓В鍐冲姙娉曟槸鍦?cmpxchg 涔嬪墠鍥哄畾锛坧in锛塯fn銆傚浐瀹氫箣鍚庯細

- 鎴戜滑鎸佹湁浜?pfn 鐨勫紩鐢ㄨ鏁帮紙refcount锛夛紱杩欐剰鍛崇潃 pfn 涓嶈兘琚噴鏀惧苟琚彟涓€涓?gfn 閲嶇敤銆?- 璇?pfn 鏄彲鍐欑殑锛屽洜姝ゅ畠涓嶈兘琚?KSM 鍦ㄤ笉鍚?gfn 涔嬮棿鍏变韩銆?
杩欐牱锛屾垜浠氨鍙互纭繚涓?gfn 姝ｇ‘璁剧疆浜嗚剰浣嶅浘銆?
2) 鑴忎綅锛圖irty bit锛夎拷韪?
鍦ㄥ師濮嬩唬鐮佷腑锛屽鏋?spte 鏄彧璇荤殑涓?Accessed 浣嶅凡琚缃紝鍒?spte 鍙互琚揩閫熸洿鏂帮紙闈炲師瀛愬湴锛夛紝
鍥犱负 Accessed 浣嶅拰 Dirty 浣嶄笉浼氫涪澶便€?
浣嗗湪蹇€熼〉閿欒涔嬪悗杩欏氨涓嶆垚绔嬩簡锛屽洜涓哄湪璇诲彇 spte 鍜屾洿鏂?spte 涔嬮棿锛宻pte 鍙兘鍙樻垚鍙啓銆傚涓嬮潰
鐨勬儏鍐碉細

+-------------------------------------------------------------------------+
**| 寮€濮嬫椂**
: |
|                                                                         |
|  spte.W = 0                                                             |
|  spte.Accessed = 1                                                      |
+-------------------------------------+-----------------------------------+
| CPU 0:                              | CPU 1:                            |
+-------------------------------------+-----------------------------------+
**| 鍦?mmu_spte_update() 涓?*
: |                                   |
|                                     |                                   |
|  old_spte = *spte;                  |                                   |
|                                     |                                   |
|                                     |                                   |
|  /** 'if' 鏉′欢琚弧瓒炽€?**/            |                                   |
|  if (old_spte.Accessed == 1 &&      |                                   |
|       old_spte.W == 0)              |                                   |
|     spte = new_spte;                |                                   |
+-------------------------------------+-----------------------------------+
**|                                     | 鍦ㄥ揩閫熼〉閿欒璺緞涓?*
: |
|                                     |                                   |
|                                     |    spte.W = 1                     |
|                                     |                                   |
**|                                     | 瀵?spte 鐨勫唴瀛樺啓鍏?*
: |
|                                     |                                   |
|                                     |    spte.Dirty = 1                 |
+-------------------------------------+-----------------------------------+
**|  **
: |                                   |
|                                     |                                   |
|   else                              |                                   |
|     old_spte = xchg(spte, new_spte);|                                   |
|   if (old_spte.Accessed &&          |                                   |
|       !new_spte.Accessed)           |                                   |
|     flush = true;                   |                                   |
|   if (old_spte.Dirty &&             |                                   |
|       !new_spte.Dirty)              |                                   |
|     flush = true;                   |                                   |
|     OOPS!!!                         |                                   |
+-------------------------------------+-----------------------------------+

鍦ㄨ繖绉嶆儏鍐典笅 Dirty 浣嶄涪澶变簡銆?
涓轰簡閬垮厤杩欑被闂锛屽鏋?spte 鍙互鍦?mmu-lock 涔嬪鏇存柊锛屾垜浠€绘槸灏嗗叾瑙嗕负鈥渧olatile鈥濓紙鏄撳彉锛?[瑙?spte_needs_atomic_update()]锛涜繖鎰忓懗鐫€鍦ㄨ繖绉嶆儏鍐典笅 spte 鎬绘槸琚師瀛愬湴鏇存柊銆?
3) 鍥?spte 鏇存柊鑰屽埛鏂?tlb

濡傛灉 spte 浠庡彲鍐欐洿鏂颁负鍙锛屾垜浠簲璇ュ埛鏂版墍鏈?TLB锛屽惁鍒?rmap_write_protect 浼氭壘鍒颁竴涓彧璇荤殑
spte锛屽嵆浣胯鍙啓鐨?spte 鍙兘浠嶇紦瀛樺湪鏌愪釜 CPU 鐨?TLB 涓€?
濡傚墠鎵€杩帮紝spte 鍦ㄥ揩閫熼〉閿欒璺緞涓婂彲浠ュ湪 mmu-lock 涔嬪琚洿鏂颁负鍙啓銆備负浜嗕究浜庡璁¤璺緞锛屾垜浠湪
mmu_spte_update() 涓煡鐪嬫槸鍚﹂渶瑕佸洜璇ュ師鍥犲埛鏂?TLB锛屽洜涓鸿繖鏄竴涓洿鏂?spte锛坧resent -> present锛?鐨勯€氱敤鍑芥暟銆?
鐢变簬 spte 鍦ㄥ彲浠ュ湪 mmu-lock 涔嬪鏇存柊鏃舵槸鈥渧olatile鈥濈殑锛屾垜浠€绘槸鍘熷瓙鍦版洿鏂?spte锛屼粠鑰屽彲浠ラ伩鍏?鐢卞揩閫熼〉閿欒寮曡捣鐨勭珵浜夈€傚弬瑙?spte_needs_atomic_update() 鍜?mmu_spte_update() 涓殑娉ㄩ噴銆?
鏃犻攣璁块棶杩借釜锛圠ockless Access Tracking锛夛細

杩欑敤浜庝娇鐢?EPT 浣嗕笉鏀寔 EPT A/D 浣嶇殑 Intel CPU銆傚湪杩欑鎯呭喌涓嬶紝PTE 琚爣璁颁负 A/D 绂佺敤锛堜娇鐢?蹇界暐浣嶏級锛屽綋 KVM MMU 閫氱煡鍣ㄨ璋冪敤浠ヨ拷韪鏌愪釜椤电殑璁块棶锛堥€氳繃 kvm_mmu_notifier_clear_flush_young锛?鏃讹紝瀹冮€氳繃娓呴櫎 PTE 涓殑 RWX 浣嶅苟灏嗗師濮嬬殑 R & X 浣嶅瓨鍏ユ洿澶氭湭浣跨敤/蹇界暐浣嶏紝鍦ㄧ‖浠朵腑灏?PTE 鏍囪涓?涓嶅瓨鍦ㄣ€傚綋 VM 绋嶅悗灏濊瘯璁块棶璇ラ〉鏃讹紝浼氫骇鐢熶竴涓敊璇紝骞朵娇鐢ㄤ笂杩板揩閫熼〉閿欒鏈哄埗灏?PTE 鍘熷瓙鍦版仮澶嶄负
Present 鐘舵€併€傚綋 PTE 琚爣璁颁负璁块棶杩借釜鏃讹紝W 浣嶄笉浼氳淇濆瓨锛涘湪鎭㈠鍒?Present 鐘舵€佹椂锛學 浣嶆牴鎹槸鍚?鏄竴娆″啓璁块棶鏉ヨ缃€傚鏋滀笉鏄紝鍒?W 浣嶅皢淇濇寔娓呴浂锛岀洿鍒板彂鐢熶竴娆″啓璁块棶锛屽眾鏃跺畠灏嗕娇鐢ㄤ笂杩拌剰浣嶈拷韪?鏈哄埗琚缃€?
### 3. 鍙傝€冿紙Reference锛?

##### ``kvm_lock``


:Type:		mutex
:Arch:		any
:Protects:	- vm_list

##### ``kvm_usage_lock``


:Type:		mutex
:Arch:		any
:Protects:	- kvm_usage_count
  - 纭欢铏氭嫙鍖栫殑鍚敤/绂佺敤
:Comment:	瀛樺湪璇ラ攣鏄负浜嗗厑璁稿湪 kvm_usage_count 鍙椾繚鎶ゆ椂鑾峰彇 cpus_read_lock()锛?		杩欑畝鍖栦簡铏氭嫙鍖栧惎鐢ㄩ€昏緫銆?
##### ``kvm->mn_invalidate_lock``


:Type:          spinlock_t
:Arch:          any
:Protects:      mn_active_invalidate_count, mn_memslots_update_rcuwait

##### ``kvm_arch::tsc_write_lock``


:Type:		raw_spinlock_t
:Arch:		x86
**:Protects:	- kvm_arch**
: {last_tsc_write,last_tsc_nsec,last_tsc_offset}
  - vmcb 涓殑 tsc 鍋忕Щ
:Comment:	'raw' 鏄洜涓烘洿鏂?tsc 鍋忕Щ鏃朵笉鍙鎶㈠崰銆?
##### ``kvm->mmu_lock``


:Type:		spinlock_t 鎴?rwlock_t
:Arch:		any
:Protects:	- 褰卞瓙椤?褰卞瓙 tlb 椤?:Comment:	杩欐槸涓€涓嚜鏃嬮攣锛屽洜涓哄畠鐢ㄤ簬 mmu 閫氱煡鍣ㄤ腑銆?
##### ``kvm->srcu``


:Type:		srcu 閿?:Arch:		any
:Protects:	- kvm->memslots
  - kvm->buses
:Comment:		璁块棶 memslots锛堜緥濡備娇鐢?gfn_to_* 鍑芥暟锛変互鍙婅闂唴鏍告€?MMIO/PIO
		鍦板潃鍒拌澶囩粨鏋勭殑鏄犲皠锛坘vm->buses锛夋椂锛屽繀椤绘寔鏈?srcu 璇婚攣銆?		濡傛灉澶氫釜鍑芥暟闇€瑕侊紝srcu 绱㈠紩鍙互瀛樺偍鍦ㄦ瘡 vcpu 鐨?kvm_vcpu->srcu_idx 涓€?
##### ``kvm->slots_arch_lock``


:Type:          mutex
:Arch:          any锛堝敖绠′粎鍦?x86 涓婇渶瑕侊級
:Protects:      蹇呴』鍦?`kvm->srcu` 璇讳晶涓寸晫鍖轰腑淇敼鐨?memslots 鐨勪换浣曟灦鏋勭壒瀹氬瓧娈点€?:Comment:       鍦ㄨ鍙栨寚鍚戝綋鍓?memslots 鐨勬寚閽堜箣鍓嶅繀椤绘寔鏈夛紝鐩村埌瀵?memslots 鐨勬墍鏈?                淇敼瀹屾垚涔嬪悗銆?
##### ``wakeup_vcpus_on_cpu_lock``


:Type:		spinlock_t
:Arch:		x86
:Protects:	wakeup_vcpus_on_cpu
:Comment:	杩欐槸涓€涓瘡 CPU 閿侊紝鐢ㄤ簬 VT-d 鎶曢€掍腑鏂紙posted-interrupts锛夈€傚綋鏀寔 VT-d
		鎶曢€掍腑鏂笖 VM 鍒嗛厤浜嗚澶囨椂锛屾垜浠皢琚樆濉炵殑 vCPU 鏀惧叆鐢?blocked_vcpu_on_cpu_lock
		淇濇姢鐨?blocked_vcpu_on_cpu 鍒楄〃涓€傚綋鏉ヨ嚜鍒嗛厤璁惧鐨勫閮ㄤ腑鏂鑷?VT-d 纭欢鍙戝嚭
		鍞ら啋閫氱煡浜嬩欢鏃讹紝鎴戜滑浼氬湪璇ュ垪琛ㄤ腑鎵惧埌 vCPU 骞跺皢鍏跺敜閱掋€?
##### ``vendor_module_lock``


:Type:		mutex
:Arch:		x86
:Protects:	鍔犺浇涓€涓巶鍟嗘ā鍧楋紙kvm_amd 鎴?kvm_intel锛?:Comment:	瀛樺湪璇ラ攣鏄洜涓轰娇鐢?kvm_lock 浼氬鑷存閿併€俴vm_lock 鍦ㄩ€氱煡鍣ㄤ腑琚寔鏈夛紝渚嬪
    __kvmclock_cpufreq_notifier()锛岃€岃閫氱煡鍣ㄥ彲鑳藉湪鎸佹湁 cpu_hotplug_lock锛堜緥濡傛潵鑷?    cpufreq_boost_trigger_state()锛夋椂琚皟鐢紱骞朵笖璁稿鎿嶄綔鍦ㄥ姞杞藉巶鍟嗘ā鍧楁椂闇€瑕佽幏鍙?    cpu_hotplug_lock锛屼緥濡傛洿鏂伴潤鎬佽皟鐢紙static call锛夈€?