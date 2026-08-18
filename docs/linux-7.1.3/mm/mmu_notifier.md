## 浣曟椂搴斿湪鍐呴〉琛ㄩ攣鍐呰繘琛岄€氱煡锛?


鍦ㄦ竻闄や竴涓?pte/pmd 鏃讹紝鎴戜滑鍙互閫夋嫨鍦ㄥ唴椤佃〃閿佷笅閫氳繃璇ヤ簨浠剁殑閫氱煡锛圽*_clear_flush 璋冪敤鐨?notify 鐗堟湰 mmu_notifier_invalidate_range锛夋潵閫氱煡璇ヤ簨浠躲€備絾骞堕潪鍦ㄦ墍鏈夋儏鍐典笅閮介渶瑕佽閫氱煡銆?

瀵逛簬娆＄骇 TLB锛堥潪 CPU TLB锛夛紝渚嬪 IOMMU TLB 鎴栬澶?TLB锛堝綋璁惧浣跨敤绫讳技 ATS/PASID 鐨勬満鍒惰 IOMMU 閬嶅巻 CPU 椤佃〃浠ヨ闂繘绋嬭櫄鎷熷湴鍧€绌洪棿鏃讹級锛屽湪娓呴櫎 pte/pmd 鏃讹紝鍙湁涓ょ鎯呭喌闇€瑕佸湪鎸佹湁椤佃〃閿佺殑鍚屾椂閫氱煡杩欎簺娆＄骇 TLB锛?

  A) 鍦?mmu_notifier_invalidate_range_end() 涔嬪墠閲婃斁椤电殑澶囦唤鍦板潃
  B) 椤佃〃椤硅鏇存柊涓烘寚鍚戜竴涓柊椤碉紙COW銆佸闆堕〉鐨勫啓缂洪〉銆乢_replace_page() 绛夛級

鎯呭喌 A 寰堟槑鏄撅紝浣犱笉浼氭兂鍐掕澶囧啓鍏ユ煇涓幇鍦ㄥ彲鑳藉凡琚畬鍏ㄤ笉鍚屼换鍔′娇鐢ㄧ殑椤电殑椋庨櫓銆?

鎯呭喌 B 鍒欐洿涓哄井濡欍€備负淇濊瘉姝ｇ‘鎬э紝闇€瑕佸彂鐢熶互涓嬪簭鍒楋細

  - 鑾峰彇椤佃〃閿?
  - 娓呴櫎椤佃〃椤瑰苟杩涜閫氱煡锛圼pmd/pte]p_huge_clear_flush_notify()锛?
  - 灏嗛〉琛ㄩ」璁剧疆涓烘寚鍚戞柊椤?

濡傛灉娓呴櫎椤佃〃椤逛箣鍚庛€佽缃柊鐨?pte/pmd 鍊间箣鍓嶆病鏈夌揣闅忎竴娆￠€氱煡锛岄偅涔堜綘灏卞彲鑳界牬鍧忚澶囦晶鐨勫唴瀛樻ā鍨嬶紙濡?C11 鎴?C++11锛夈€?

鑰冭檻浠ヤ笅鍦烘櫙锛堣澶囦娇鐢ㄤ簡绫讳技 ATS/PASID 鐨勭壒鎬э級锛?

涓や釜鍦板潃 addrA 鍜?addrB锛屾弧瓒?\|addrA - addrB\| >= PAGE_SIZE锛屾垜浠亣璁惧畠浠洜 COW 鑰岃鍐欎繚鎶わ紙鎯呭喌 B 鐨勫叾浠栨儏褰㈠悓鏍烽€傜敤锛夈€?

```

 [Time N] --------------------------------------------------------------------
 CPU-thread-0  {try to write to addrA}
 CPU-thread-1  {try to write to addrB}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {read addrA and populate device TLB}
 DEV-thread-2  {read addrB and populate device TLB}
 [Time N+1] ------------------------------------------------------------------
 CPU-thread-0  {COW_step0: {mmu_notifier_invalidate_range_start(addrA)}}
 CPU-thread-1  {COW_step0: {mmu_notifier_invalidate_range_start(addrB)}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+2] ------------------------------------------------------------------
 CPU-thread-0  {COW_step1: {update page table to point to new page for addrA}}
 CPU-thread-1  {COW_step1: {update page table to point to new page for addrB}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+3] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {preempted}
 CPU-thread-2  {write to addrA which is a write to new page}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+3] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {preempted}
 CPU-thread-2  {}
 CPU-thread-3  {write to addrB which is a write to new page}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+4] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {COW_step3: {mmu_notifier_invalidate_range_end(addrB)}}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {}
 DEV-thread-2  {}
 [Time N+5] ------------------------------------------------------------------
 CPU-thread-0  {preempted}
 CPU-thread-1  {}
 CPU-thread-2  {}
 CPU-thread-3  {}
 DEV-thread-0  {read addrA from old page}
 DEV-thread-2  {read addrB from new page}

```
鎵€浠ヨ繖閲屽洜涓哄湪鏃跺埢 N+2锛屾竻闄ら〉琛ㄩ」鐨勬搷浣滄病鏈変笌涓€娆＄敤浜庝娇娆＄骇 TLB 澶辨晥鐨勯€氱煡閰嶅锛岃澶囦細鍦ㄧ湅鍒?addrA 鐨勬柊鍊间箣鍓嶅厛鐪嬪埌 addrB 鐨勬柊鍊笺€傝繖鐮村潖浜嗚澶囦晶鐨勬€讳綋鍐呭瓨椤哄簭銆?

褰撳皢涓€涓?pte 鏀逛负鍐欎繚鎶わ紝鎴栨寚鍚戜竴涓叿鏈夌浉鍚屽唴瀹癸紙KSM锛夌殑鏂板啓淇濇姢椤垫椂锛屽皢 mmu_notifier_invalidate_range 璋冪敤寤惰繜鍒伴〉琛ㄩ攣涔嬪鐨?mmu_notifier_invalidate_range_end() 鏄彲浠ョ殑銆傚嵆浣垮湪鎵ц椤佃〃鏇存柊鐨勭嚎绋嬩簬閲婃斁椤佃〃閿佷箣鍚庛€佽皟鐢?mmu_notifier_invalidate_range_end() 涔嬪墠琚姠鍗狅紝涔熸槸濡傛銆?
