
## 杩涚▼鍦板潃


鐢ㄦ埛鎬佸唴瀛樺尯闂寸敱鍐呮牳閫氳繃铏氭嫙鍐呭瓨鍖哄煙锛圴irtual Memory Areas锛岀畝绉?'VMA'锛夋潵璺熻釜锛屽叾绫诲瀷涓?:c`!struct vm_area_struct`銆?

姣忎釜 VMA 鎻忚堪涓€娈佃櫄鎷熻繛缁殑銆佸叿鏈夊畬鍏ㄧ浉鍚屽睘鎬х殑鍐呭瓨鍖洪棿锛岀敱涓€涓釜 :c`!struct vm_area_struct` 瀵硅薄鏉ユ弿杩般€傚湪 VMA 涔嬪鐨勭敤鎴锋€佽闂槸鏃犳晥鐨勶紝闄ら潪鐩搁偦鐨勬爤 VMA 鍙互琚墿灞曚互鍖呭惈琚闂殑鍦板潃銆?

鎵€鏈?VMA 閮藉寘鍚湪涓€涓笖鍞竴涓€涓櫄鎷熷湴鍧€绌洪棿涓紝璇ュ湴鍧€绌洪棿鐢变竴涓?:c`!struct mm_struct` 瀵硅薄鎻忚堪锛屾墍鏈夊叡浜铏氭嫙鍦板潃绌洪棿鐨勪换鍔★紙鍗崇嚎绋嬶級閮藉紩鐢ㄥ畠銆傛垜浠О涔嬩负 :c`!mm`銆?

姣忎釜 mm 瀵硅薄鍖呭惈涓€涓?maple tree 鏁版嵁缁撴瀯锛岀敤浜庢弿杩拌櫄鎷熷湴鍧€绌洪棿鍐呯殑鎵€鏈?VMA銆?

          浣跨敤 :c`!vsyscall` 鐨勬灦鏋勶紝骞朵笖鏄竴涓叏灞€闈欐€佸璞★紝涓嶅睘浜庝换浣曠壒瀹氱殑 mm銆?

### 閿佹満鍒?


鍐呮牳鍦ㄨ璁′笂瀵?VMA **鍏冩暟鎹紙metadata锛?* 鐨勫苟鍙戣鎿嶄綔鍏锋湁楂樺害鍙墿灞曟€э紝鍥犳闇€瑕佷竴濂楀鏉傜殑閿佹潵纭繚涓嶄細鍙戠敓鍐呭瓨鎹熷潖銆?

          瀹冧滑鎵€鎻忚堪鐨勶紝涔熶笉鍖呮嫭鏄犲皠瀹冧滑鐨勯〉琛ㄣ€?

### 鏈


- **mmap 閿?* - 姣忎釜 MM 閮芥湁涓€涓鍐欎俊鍙烽噺 :c`!mmap_lock`锛屽畠浠ヨ繘绋嬪湴鍧€绌洪棿涓虹矑搴﹀姞閿侊紝鍙€氳繃 `!mmap_read_lock`銆乣!mmap_write_lock` 鍙婂叾鍙樹綋鑾峰彇銆?
- **VMA 閿?* - VMA 閿佺殑绮掑害涓?VMA锛堢悊鎵€褰撶劧锛夛紝鍦ㄥ疄璺典腑琛ㄧ幇涓轰竴涓鍐欎俊鍙烽噺銆俈MA 璇婚攣閫氳繃 `!lock_vma_under_rcu` 鑾峰彇锛堝苟閫氳繃 `!vma_end_read` 閲婃斁锛夛紝鍐欓攣閫氳繃 vma_start_write() 鎴?vma_start_write_killable() 鑾峰彇锛堟墍鏈?VMA 鍐欓攣鍦?mmap 鍐欓攣閲婃斁鏃朵細鑷姩閲婃斁锛夈€傝鑾峰彇 VMA 鍐欓攣锛屼綘**蹇呴』**宸茬粡鎸佹湁浜?`!mmap_write_lock`銆?
- **rmap 閿?* - 褰撳皾璇曢€氳繃鍙嶅悜鏄犲皠锛坮everse mapping锛夌粡鐢变竴涓?:c`!struct address_space` 鎴?:c`!struct anon_vma` 瀵硅薄锛堝彲鐢变竴涓?folio 閫氳繃 :c`!folio->mapping` 鍒拌揪锛夋潵璁块棶 VMA 鏃躲€俈MA 蹇呴』閫氳繃 `!anon_vma_[try]lock_read` 鎴?`!anon_vma_[try]lock_write`锛堢敤浜庡尶鍚嶅唴瀛橈級浠ュ強 `!i_mmap_[try]lock_read` 鎴?`!i_mmap_[try]lock_write`锛堢敤浜庢枃浠跺悗澶囧唴瀛橈級鏉ヤ繚鎸佺ǔ瀹氥€傛垜浠О杩欎簺閿佷负鍙嶅悜鏄犲皠閿侊紝鎴栫畝绉?'rmap 閿?銆?

鎴戜滑灏嗗湪涓嬮潰涓撻棬鐨勫皬鑺備腑鍗曠嫭璁ㄨ椤佃〃閿併€?

杩欎簺閿?*浠讳綍**涓€涓鍏堣瀹炵幇鐨勭洰鏍囨槸璁?VMA 鍦?MM 鏍戜腑**绋冲畾**涓嬫潵銆備篃灏辨槸璇达紝淇濊瘉 VMA 瀵硅薄涓嶄細琚湪浣犵湅涓嶈鐨勬儏鍐典笅鍒犻櫎鎴栦慨鏀癸紙涓嬮潰鎻忚堪鐨勪竴浜涚壒瀹氬瓧娈甸櫎澶栵級銆?

绋冲畾涓€涓?VMA 鍚屾椂涔熶繚鎸佷簡瀹冩墍鎻忚堪鐨勫湴鍧€绌洪棿鐨勫瓨鍦ㄣ€?

### 閿佺殑浣跨敤


濡傛灉浣犳兂瑕?*璇诲彇** VMA 鍏冩暟鎹瓧娈碉紝鎴栬€呭彧鏄兂璁?VMA 淇濇寔绋冲畾锛屼綘蹇呴』鍋氫互涓嬩箣涓€锛?

- 閫氳繃 `!mmap_read_lock`锛堟垨鍚堥€傜殑鍙樹綋锛夊湪 MM 绮掑害涓婅幏鍙栦竴涓?mmap 璇婚攣锛屽湪浣犱娇鐢ㄥ畬 VMA 鍚庨€氳繃鐩稿簲鐨?`!mmap_read_unlock` 閲婃斁瀹冿紝**鎴栬€?*
- 灏濊瘯閫氳繃 `!lock_vma_under_rcu` 鑾峰彇涓€涓?VMA 璇婚攣銆傚畠浼氬皾璇曞師瀛愬湴鑾峰彇璇ラ攣锛屽洜姝ゅ彲鑳藉け璐ワ紝姝ゆ椂闇€瑕佸洖閫€閫昏緫锛屽湪鍏惰繑鍥?`!NULL` 鐨勬儏鍐典笅鏀逛负鑾峰彇涓€涓?mmap 璇婚攣锛?*鎴栬€?*
- 鍦ㄩ亶鍘嗚閿佸畾鐨勫尯闂存爲锛堟棤璁烘槸鍖垮悕鐨勮繕鏄枃浠跺悗澶囩殑锛変互鑾峰彇鎵€闇€鐨?VMA 涔嬪墠锛屽厛鑾峰彇涓€涓?rmap 閿併€?

濡傛灉浣犳兂瑕?*鍐欏叆** VMA 鍏冩暟鎹瓧娈碉紝鍒欐儏鍐靛洜瀛楁鑰屽紓锛堟垜浠皢鍦ㄤ笅闈㈣缁嗘帰璁ㄦ瘡涓?VMA 瀛楁锛夈€傚浜庡ぇ澶氭暟瀛楁锛屼綘蹇呴』锛?

- 閫氳繃 `!mmap_write_lock`锛堟垨鍚堥€傜殑鍙樹綋锛夊湪 MM 绮掑害涓婅幏鍙栦竴涓?mmap 鍐欓攣锛屽湪浣犱娇鐢ㄥ畬 VMA 鍚庨€氳繃鐩稿簲鐨?`!mmap_write_unlock` 閲婃斁瀹冿紝**骞朵笖**
- 閫氳繃 `!vma_start_write` 涓轰綘鎯宠淇敼鐨勬瘡涓?VMA 鑾峰彇涓€涓?VMA 鍐欓攣锛屽畠浼氬湪 `!mmap_write_unlock` 琚皟鐢ㄦ椂鑷姩閲婃斁銆?
- 濡傛灉浣犳兂瑕佽兘澶熷啓鍏?*浠讳綍**瀛楁锛屼綘杩樺繀椤婚€氳繃鑾峰彇涓€涓?**rmap 鍐欓攣** 灏?VMA 浠庡弽鍚戞槧灏勪腑闅愯棌璧锋潵銆?

VMA 閿佺殑鐗规畩涔嬪鍦ㄤ簬锛屼綘蹇呴』**鍏?*鑾峰彇涓€涓?mmap **鍐?*閿侊紝鎵嶈兘鑾峰彇涓€涓?VMA **鍐?*閿併€備笉杩囷紝VMA **璇?*閿佸彲浠ュ湪娌℃湁浠讳綍鍏朵粬閿佺殑鎯呭喌涓嬭幏鍙栵紙`!lock_vma_under_rcu` 浼氳幏鍙栧啀閲婃斁涓€涓?RCU 閿佹潵涓轰綘鏌ユ壘 VMA锛夈€?

杩欓檺鍒朵簡鍐欒€呭璇昏€呯殑褰卞搷锛屽洜涓哄啓鑰呭彲浠ヤ笌涓€涓?VMA 浜や簰锛岃€岃鑰呭彲浠ュ悓鏃朵笌鍙︿竴涓?VMA 浜や簰銆?

          鎰忓懗鐫€鍦ㄦ病鏈?VMA 鍐欓攣鐨勬儏鍐典笅锛岀己椤靛紓甯革紙page fault锛夊皢涓庝綘鎵€鍋氱殑浠讳綍鎿嶄綔骞跺彂杩愯銆?

鑰冨療鎵€鏈夋湁鏁堢殑閿佺姸鎬侊細

   ========= ======== ========= ======= ===== =========== ==========
   mmap lock VMA lock rmap lock Stable? Read? Write most? Write all?
   ========= ======== ========= ======= ===== =========== ==========
   \-        \-       \-        N       N     N           N
   \-        R        \-        Y       Y     N           N
   \-        \-       R/W       Y       Y     N           N
   R/W       \-/R     \-/R/W    Y       Y     N           N
   W         W        \-/R      Y       Y     Y           N
   W         W        W         Y       Y     Y           Y
   ========= ======== ========= ======= ===== =========== ==========

            灏濊瘯鍙嶅悜鎿嶄綔鏄棤鏁堢殑锛屽洜涓鸿繖鍙兘瀵艰嚧姝婚攣鈥斺€斿鏋?
            鍙︿竴涓换鍔″凡缁忔寔鏈?mmap 鍐欓攣骞跺皾璇曡幏鍙栦竴涓?VMA
            鍐欓攣锛屽畠灏嗗湪 VMA 璇婚攣涓婂彂鐢熸閿併€?

鎵€鏈夎繖浜涢攣鍦ㄥ疄璺典腑閮借〃鐜颁负璇诲啓淇″彿閲忥紝鍥犳浣犲彲浠ヤ负瀹冧滑涓殑姣忎竴涓幏鍙栬閿佹垨鍐欓攣銆?

          鍏佽澶氫釜骞跺彂璇昏€呫€備絾鍐欓攣鍙湁鍦ㄦ墍鏈夎鑰呴兘宸茬寮€涓寸晫鍖猴紙骞朵笖寰呭鐞嗙殑璇昏€呰缃负绛夊緟锛夋椂鎵嶈兘鑾峰彇銆?

          杩欎娇寰楄鍐欎俊鍙烽噺涓婄殑璇婚攣涓庡叾浠栬鑰呭苟鍙戯紝鑰屽啓閿佸垯鎺掍粬浜庢墍鏈夋寔鏈夎淇″彿閲忕殑鍏朵粬鏂广€?

##### VMA 瀛楁


鎴戜滑鍙互鏍规嵁 :c`!struct vm_area_struct` 瀛楁鐨勭敤閫斿鍏惰繘琛岀粏鍒嗭紝杩欎娇寰楁帰绱㈠畠浠殑閿佺壒鎬ф洿涓哄鏄擄細

          瀹為檯涓婃槸涓€涓唴閮ㄥ疄鐜扮粏鑺傘€?


   ===================== ======================================== ===========
   Field                 Description                              Write lock
   ===================== ======================================== ===========
   :c`!vm_start` Inclusive start virtual address of range mmap write,
                         VMA describes.                           VMA write,
                                                                  rmap write.
   :c`!vm_end`   Exclusive end virtual address of range   mmap write,
                         VMA describes.                           VMA write,
                                                                  rmap write.
   :c`!vm_pgoff` Describes the page offset into the file, mmap write,
                         the original page offset within the      VMA write,
                         virtual address space (prior to any      rmap write.
                         `!mremap`), or PFN if a PFN map
                         and the architecture does not support
                         `!CONFIG_ARCH_HAS_PTE_SPECIAL`.
   ===================== ======================================== ===========

杩欎簺瀛楁鎻忚堪浜?VMA 鐨勫ぇ灏忋€佽捣濮嬪拰缁撴潫鍦板潃锛屽洜姝ゆ棤娉曞湪涓嶅厛灏嗗叾浠庡弽鍚戞槧灏勪腑闅愯棌鐨勬儏鍐典笅琚慨鏀癸紝鍥犱负杩欎簺瀛楁鐢ㄤ簬鍦ㄥ弽鍚戞槧灏勫尯闂存爲涓畾浣?VMA銆?


   ============================ ======================================== =========================
   Field                        Description                              Write lock
   ============================ ======================================== =========================
   :c`!vm_mm`           Containing mm_struct.                    None - written once on
                                                                         initial map.
   :c`!vm_page_prot`    Architecture-specific page table         mmap write, VMA write.
                                protection bits determined from VMA
                                flags.
   :c`!vm_flags`        Read-only access to VMA flags describing N/A
                                attributes of the VMA, in union with
                                private writable
                                :c`!__vm_flags`.
   :c`!__vm_flags`      Private, writable access to VMA flags    mmap write, VMA write.
                                field, updated by
                                `!vm_flags_*` functions.
   :c`!vm_file`         If the VMA is file-backed, points to a   None - written once on
                                struct file object describing the        initial map.
                                underlying file, if anonymous then
                                `!NULL`.
   :c`!vm_ops`          If the VMA is file-backed, then either   None - Written once on
                                the driver or file-system provides a     initial map by
                                :c`!struct vm_operations_struct` `!f_ops->mmap()`.
                                object describing callbacks to be
                                invoked on VMA lifetime events.
   :c`!vm_private_data` A :c`!void *` field for          Handled by driver.
                                driver-specific metadata.
   ============================ ======================================== =========================

杩欎簺鏄敤浜庢弿杩?VMA 鎵€灞炵殑 MM 鍙婂叾灞炴€х殑鏍稿績瀛楁銆?


   ================================= ===================== ======================================== ===============
   Field                             Configuration option  Description                              Write lock
   ================================= ===================== ======================================== ===============
   :c`!anon_name`            CONFIG_ANON_VMA_NAME  A field for storing a                    mmap write,
                                                           :c`!struct anon_vma_name`        VMA write.
                                                           object providing a name for anonymous
                                                           mappings, or `!NULL` if none
                                                           is set or the VMA is file-backed. The
							   underlying object is reference counted
							   and can be shared across multiple VMAs
							   for scalability.
   :c`!swap_readahead_info`  CONFIG_SWAP           Metadata used by the swap mechanism      mmap read,
                                                           to perform readahead. This field is      swap-specific
                                                           accessed atomically.                     lock.
   :c`!vm_policy`            CONFIG_NUMA           `!mempolicy` object which        mmap write,
                                                           describes the NUMA behaviour of the      VMA write.
                                                           VMA. The underlying object is reference
							   counted.
   :c`!numab_state`          CONFIG_NUMA_BALANCING `!vma_numab_state` object which  mmap read,
                                                           describes the current state of           numab-specific
                                                           NUMA balancing in relation to this VMA.  lock.
                                                           Updated under mmap read lock by
                                                           `!task_numa_work`.
   :c`!vm_userfaultfd_ctx`   CONFIG_USERFAULTFD    Userfaultfd context wrapper object of    mmap write,
                                                           type `!vm_userfaultfd_ctx`,      VMA write.
                                                           either of zero size if userfaultfd is
                                                           disabled, or containing a pointer
                                                           to an underlying
                                                           `!userfaultfd_ctx` object which
                                                           describes userfaultfd metadata.
   ================================= ===================== ======================================== ===============

杩欎簺瀛楁鏄惁瀛樺湪锛屽彇鍐充簬鐩稿叧鐨勫唴鏍搁厤缃€夐」鏄惁琚缃€?


   =================================== ========================================= ============================
   Field                               Description                               Write lock
   =================================== ========================================= ============================
   :c`!shared.rb`              A red/black tree node used, if the        mmap write, VMA write,
                                       mapping is file-backed, to place the VMA  i_mmap write.
                                       in the
                                       :c`!struct address_space->i_mmap`
                                       red/black interval tree.
   :c`!shared.rb_subtree_last` Metadata used for management of the       mmap write, VMA write,
                                       interval tree if the VMA is file-backed.  i_mmap write.
   :c`!anon_vma_chain`         List of pointers to both forked/CoW鈥檇     mmap read, anon_vma write.
                                       `!anon_vma` objects and
                                       :c`!vma->anon_vma` if it is
                                       non-`!NULL`.
   :c`!anon_vma`               `!anon_vma` object used by        When `NULL` and
                                       anonymous folios mapped exclusively to    setting non-`NULL`:
                                       this VMA. Initially set by                mmap read, page_table_lock.
                                       `!anon_vma_prepare` serialised
                                       by the `!page_table_lock`. This  When non-`NULL` and
                                       is set as soon as any page is faulted in. setting `NULL`:
                                                                                 mmap write, VMA write,
                                                                                 anon_vma write.
   =================================== ========================================= ============================

杩欎簺瀛楁鏃㈢敤浜庡皢 VMA 鏀剧疆鍒板弽鍚戞槧灏勪腑锛屼篃鐢ㄤ簬鍦ㄥ尶鍚嶆槧灏勬椂璁块棶鐩稿叧鐨?:c`!struct anon_vma` 瀵硅薄锛屼互鍙婇偅浜涜鐙崰鏄犲皠鍒版湰 VMA 鐨?folio 搴斿綋鎵€鍦ㄧ殑 :c`!struct anon_vma`銆?

          閭ｄ箞瀹冨彲鑳藉悓鏃跺浜?`!anon_vma` 鍜?`!i_mmap`
          鏍戜腑锛屽洜姝ゆ墍鏈夎繖浜涘瓧娈靛彲鑳藉悓鏃惰浣跨敤銆?

### 椤佃〃


鎴戜滑涓嶄細璇﹀敖鍦拌璁鸿繖涓富棰橈紝浣嗗ぇ浣撹€岃█锛岄〉琛ㄩ€氳繃涓€绯诲垪椤佃〃灏嗚櫄鎷熷湴鍧€鏄犲皠鍒扮墿鐞嗗湴鍧€锛屽叾涓瘡涓〉琛ㄩ兘鍖呭惈鎸囧悜涓嬩竴绾ч〉琛ㄧ墿鐞嗗湴鍧€鐨勬潯鐩紙浠ュ強鏍囧織浣嶏級锛屽湪鍙跺瓙绾у埆鍒欐槸搴曞眰鐗╃悊鏁版嵁椤电殑鐗╃悊鍦板潃锛屾垨鑰呬氦鎹㈡潯鐩€佽縼绉绘潯鐩垨鍏朵粬鐗规畩鏍囪銆傝繖浜涢〉鍐呯殑鍋忕Щ閲忕敱铏氭嫙鍦板潃鏈韩鎻愪緵銆?

鍦?Linux 涓紝杩欎簺琚垝鍒嗕负浜斾釜绾у埆鈥斺€擯GD銆丳4D銆丳UD銆丳MD 鍜?PTE銆傚ぇ椤碉紙Huge pages锛夊彲鑳戒細娑堥櫎鍏朵腑鐨勪竴涓垨涓や釜绾у埆锛屼絾鍦ㄨ繖绉嶆儏鍐典笅锛屾垜浠€氬父浠嶅皢鍙跺瓙绾у埆绉颁负 PTE 绾у埆銆?

	  鍦ㄥ唴鏍稿緢宸у鍦板椤佃〃绾у埆杩涜鈥滄姌鍙犫€濓紝鍗虫妸涓庤璺宠繃绾у埆鐩稿叧鐨勫嚱鏁板瓨鏍瑰寲锛坰tubbing out锛夈€傝繖璁╂垜浠湪姒傚康涓婂彲浠ュ儚濮嬬粓鏈変簲涓骇鍒竴鏍锋搷浣滐紝鍗充娇缂栬瘧鍣ㄥ湪瀹炶返涓彲鑳戒細娑堥櫎涓庣己澶辩骇鍒浉鍏崇殑浠讳綍浠ｇ爜銆?

椤佃〃涓婇€氬父鏈夊洓涓叧閿搷浣滐細

1. **閬嶅巻锛圱raversing锛?* 椤佃〃 - 浠呬粎鏄鍙栭〉琛ㄤ互渚块亶鍘嗗畠浠€傝繖鍙姹?VMA 淇濇寔绋冲畾锛屽洜姝よ冻浠ュ缓绔嬭繖绉嶇ǔ瀹氭€х殑閿佸嵆鍙敤浜庨亶鍘嗭紙涔熸湁鏃犻攣鍙樹綋锛岃繛杩欎竴瑕佹眰涔熸秷闄や簡锛屼緥濡?`!gup_fast`锛夈€傚浜庨潪 VMA 鍖哄煙鐨勯〉琛ㄩ亶鍘嗘湁涓€涓壒渚嬶紝鎴戜滑鍦ㄤ笅闈㈠崟鐙€冭檻銆?
2. **瀹夎锛圛nstalling锛?* 椤佃〃鏄犲皠 - 鏃犺鏄垱寤烘柊鏄犲皠杩樻槸浠ユ敼鍙樺叾鏍囪瘑鐨勬柟寮忎慨鏀圭幇鏈夋槧灏勩€傝繖瑕佹眰 VMA 閫氳繃 mmap 鎴?VMA 閿侊紙鏄庣‘涓嶆槸 rmap 閿侊級淇濇寔绋冲畾銆?
3. **娓呯┖/瑙ｉ櫎鏄犲皠锛圸apping/unmapping锛?* 椤佃〃鏉＄洰 - 杩欐槸鍐呮牳瀵逛粎鍦ㄥ彾瀛愮骇鍒竻闄ら〉琛ㄦ槧灏勭殑绉板懠锛屽悓鏃朵繚鐣欐墍鏈夐〉琛ㄤ笉鍙樸€傝繖鏄唴鏍镐腑鍦ㄦ枃浠舵埅鏂€乣!MADV_DONTNEED` 鎿嶄綔锛堢粡鐢?`!madvise`锛夌瓑鍦烘櫙涓嬫墽琛岀殑闈炲父甯歌鐨勬搷浣溿€傚畠鐢卞寘鎷?`!unmap_mapping_range` 鍜?`!unmap_mapping_pages` 鍦ㄥ唴鐨勮嫢骞插嚱鏁版墽琛屻€傛鎿嶄綔鍙渶瑕?VMA 淇濇寔绋冲畾銆?
4. **閲婃斁锛團reeing锛?* 椤佃〃 - 褰撳唴鏍告渶缁堜粠鐢ㄦ埛鎬佽繘绋嬬Щ闄ら〉琛ㄦ椂锛堥€氬父閫氳繃 `!free_pgtables`锛夛紝蹇呴』鏋佸害灏忓績浠ョ‘淇濆畨鍏ㄥ湴瀹屾垚锛屽洜涓鸿閫昏緫鏈€缁堜細閲婃斁鎸囧畾鑼冨洿鍐呯殑鎵€鏈夐〉琛紝蹇界暐鐜版湁鐨勫彾瀛愭潯鐩紙瀹冨亣璁捐皟鐢ㄨ€呮棦宸叉竻绌鸿鑼冨洿锛屽張闃绘浜嗗叾涓换浣曡繘涓€姝ョ殑缂洪〉鎴栦慨鏀癸級銆?

          閿侊紝鍥犱负涓庢竻绌轰竴鏍凤紝瀹冩牴鏈笂涓嶄細淇敼琚槧灏勫璞＄殑鏍囪瘑銆?

**閬嶅巻** 鍜?**娓呯┖** 鑼冨洿鍙互鎸佹湁涓婅堪鏈灏忚妭涓弿杩扮殑浠讳竴閿佹潵鎵ц鈥斺€斿嵆 mmap 閿併€乂MA 閿佹垨浠讳竴涓弽鍚戞槧灏勯攣銆?

涔熷氨鏄鈥斺€斿彧瑕佷綘璁╃浉鍏崇殑 VMA 淇濇寔**绋冲畾**鈥斺€斾綘灏卞彲浠ユ斁鎵嬪杩欎簺椤佃〃鎵ц杩欎簺鎿嶄綔锛堜笉杩囧湪鍐呴儴锛屾墽琛屽啓鍏ョ殑鍐呮牳鎿嶄綔涔熶細鑾峰彇鍐呴儴椤佃〃閿佷互杩涜涓茶鍖栤€斺€旇瑙侀〉琛ㄥ疄鐜扮粏鑺傚皬鑺傦級銆?

          鏀瑰彉涓婅堪鍏充簬娓呯┖鐨勯攣瑕佹眰銆?

褰?*瀹夎** 椤佃〃鏉＄洰鏃讹紝蹇呴』鎸佹湁 mmap 鎴?VMA 閿佷互淇濇寔 VMA 绋冲畾銆傛垜浠細鍦ㄤ笅闈㈢殑椤佃〃閿佺粏鑺傚皬鑺備腑鎺㈣鍏跺師鍥犮€?

**閲婃斁** 椤佃〃鏄竴椤瑰畬鍏ㄥ唴閮ㄧ殑鍐呭瓨绠＄悊鎿嶄綔锛屽叿鏈夌壒娈婄殑瑕佹眰锛堣瑙佷笅闈㈢殑椤甸噴鏀惧皬鑺傦級銆?

            鍖呭惈杩欎簺椤佃〃鎵€鏄犲皠鑼冨洿鐨?VMA 鍙€氳繃鍙嶅悜鏄犲皠璁块棶銆?

            `!free_pgtables` 鍑芥暟灏嗚繖浜?VMA 浠庡弽鍚戞槧灏勪腑绉婚櫎锛?
            浣嗕笉寰楀厑璁镐换浣曞叾浠?VMA 鍙闂苟璺ㄨ秺鎸囧畾鑼冨洿銆?

### 閬嶅巻闈?VMA 椤佃〃


鎴戜滑涓婇潰鍏虫敞鐨勬槸灞炰簬 VMA 鐨勯〉琛ㄧ殑閬嶅巻銆備篃鏈夊彲鑳介亶鍘嗕笉鐢?VMA 琛ㄧず鐨勯〉琛ㄣ€?

鍐呮牳椤佃〃鏄犲皠鏈韩閫氬父鐢卞缓绔嬪畠浠殑鍐呮牳鐨勭浉搴旈儴鍒嗙鐞嗭紝鍓嶈堪閿佸畾瑙勫垯涓嶉€傜敤鈥斺€斾緥濡?vmalloc 鏈夎嚜宸辩殑涓€缁勯攣锛岀敤浜庡缓绔嬪拰鎷嗛櫎鍏堕〉琛ㄣ€?

涓嶈繃锛屼负鏂逛究璧疯锛屾垜浠彁渚涗簡 `!walk_kernel_page_table_range` 鍑芥暟锛屽畠閫氳繃 `!init_mm` 杩欎釜 :c`!struct mm_struct` 鍏冩暟鎹璞＄殑鍐呮牳瀹炰緥涓婄殑 mmap 閿佽繘琛屽悓姝ャ€?

濡傛灉闇€瑕佺嫭鍗犺闂紝鍒欎娇鐢ㄥ啓閿侊紝鍚﹀垯璇婚攣灏辫冻澶熶簡鈥斺€旀垜浠彧鏂█鑷冲皯宸茶幏鍙栦簡涓€涓閿併€?

鐢变簬闄?vmalloc 鍜屽唴瀛樼儹鎻掓嫈澶栵紝鍐呮牳椤佃〃骞朵笉缁忓父琚媶闄も€斺€旇繖閫氬父灏辫冻澶熶簡锛屼絾鏄鍔熻兘鐨勪换浣曡皟鐢ㄨ€呴兘蹇呴』纭繚鎻愬墠鑾峰彇浠讳綍棰濆闇€瑕佺殑閿併€?

鎴戜滑杩樺厑璁镐竴涓湡姝ｄ笉瀵诲父鐨勬儏鍐碉紝鍗冲湪**鐢ㄦ埛鎬?*鑼冨洿鍐呴亶鍘嗛潪 VMA 椤佃〃锛岃繖鐢?`!walk_page_range_debug` 鎻愪緵銆?

瀹冨彧鏈変竴涓娇鐢ㄨ€呪€斺€旈€氱敤鐨勯〉琛ㄨ浆鍌ㄩ€昏緫锛堝疄鐜颁簬 `!mm/ptdump.c`锛夆€斺€斿叾鐩殑鏄毚闇叉墍鏈夋槧灏勪互渚涜皟璇曪紝鍗充娇瀹冧滑闈炲父涓嶅甯革紙鍙兘鏄灦鏋勭浉鍏崇殑锛変笖涓嶇敱 VMA 鍚庡銆?

鍦ㄨ繖绉嶆儏鍐典笅鎴戜滑蹇呴』鏍煎灏忓績锛屽洜涓?`!munmap` 瀹炵幇浼氬湪闄嶇骇涓?mmap 璇婚攣鐨勬儏鍐典笅锛屼簬 mmap 鍐欓攣涓嬪厛鍒嗙 VMA锛屽啀鎷嗛櫎椤佃〃銆?

杩欐剰鍛崇潃姝ょ被鎿嶄綔鍙兘涓庝箣绔炰簤锛屽洜姝ら渶瑕?mmap **鍐?*閿併€?

### 閿侀『搴?


鐢变簬鍐呮牳涓湁澶氫釜閿佸彲鑳戒細鎴栦笉鍙兘浼氫笌鏄惧紡鐨?mm 鎴?VMA 閿佸悓鏃惰幏鍙栵紝鎴戜滑蹇呴』璀︽儠閿佸弽杞紙lock inversion锛夛紝骞朵笖閿佽幏鍙栦笌閲婃斁鐨?*椤哄簭**鍙樺緱闈炲父閲嶈銆?

            浣嗚繖鏍峰仛浼氭棤鎰忎腑瀵艰嚧鐩镐簰姝婚攣銆?

            渚嬪锛岃€冭檻鎸佹湁閿?A 骞跺皾璇曡幏鍙栭攣 B 鐨勭嚎绋?1锛岃€屾寔鏈夐攣 B 骞跺皾璇曡幏鍙栭攣 A 鐨勭嚎绋?2銆?

            鐜板湪涓や釜绾跨▼鐩镐簰姝婚攣銆傜劧鑰岋紝濡傛灉瀹冧滑灏濊瘯浠ョ浉鍚岄『搴忚幏鍙栭攣锛屽叾涓竴涓細绛夊緟鍙︿竴涓畬鎴愬叾宸ヤ綔锛屽氨涓嶄細鍙戠敓姝婚攣銆?

`!mm/rmap.c` 寮€澶寸殑娉ㄩ噴璇︾粏鎻忚堪浜嗗唴瀛樼鐞嗕唬鐮佸唴閮ㄦ墍闇€鐨勯攣椤哄簭锛?

  inode->i_rwsem        (while writing or truncating, not reading or faulting)
    mm->mmap_lock
      mapping->invalidate_lock (in filemap_fault)
        folio_lock
          hugetlbfs_i_mmap_rwsem_key (in huge_pmd_share, see hugetlbfs below)
            vma_start_write
              mapping->i_mmap_rwsem
                anon_vma->rwsem
                  mm->page_table_lock or pte_lock
                    swap_lock (in swap_duplicate, swap_info_get)
                      mmlist_lock (in mmput, drain_mmlist and others)
                      mapping->private_lock (in block_dirty_folio)
                          i_pages lock (widely used)
                            lruvec->lru_lock (in folio_lruvec_lock_irq)
                      inode->i_lock (in set_page_dirty's __mark_inode_dirty)
                      bdi.wb->list_lock (in set_page_dirty's __mark_inode_dirty)
                        sb_lock (within inode_lock in fs/fs-writeback.c)
                        i_pages lock (widely used, in set_page_dirty,
                                  in arch-dependent flush_dcache_mmap_lock,
                                  within bdi.wb->list_lock in __sync_single_inode)

鍦?`!mm/filemap.c` 椤堕儴杩樻湁涓€涓枃浠剁郴缁熺壒瀹氱殑閿侀『搴忔敞閲婏細

  ->i_mmap_rwsem                        (truncate_pagecache)
    ->private_lock                      (__free_pte->block_dirty_folio)
      ->swap_lock                       (exclusive_swap_page, others)
        ->i_pages lock

  ->i_rwsem
    ->invalidate_lock                   (acquired by fs in truncate path)
      ->i_mmap_rwsem                    (truncate->unmap_mapping_range)

  ->mmap_lock
    ->i_mmap_rwsem
      ->page_table_lock or pte_lock     (various, mainly in memory.c)
        ->i_pages lock                  (arch-dependent flush_dcache_mmap_lock)

  ->mmap_lock
    ->invalidate_lock                   (filemap_fault)
      ->lock_page                       (filemap_fault, access_process_vm)

  ->i_rwsem                             (generic_perform_write)
    ->mmap_lock                         (fault_in_readable->do_page_fault)

  bdi->wb.list_lock
    sb_lock                             (fs/fs-writeback.c)
    ->i_pages lock                      (__sync_single_inode)

  ->i_mmap_rwsem
    ->anon_vma.lock                     (vma_merge)

  ->anon_vma.lock
    ->page_table_lock or pte_lock       (anon_vma_prepare and various)

  ->page_table_lock or pte_lock
    ->swap_lock                         (try_to_unmap_one)
    ->private_lock                      (try_to_unmap_one)
    ->i_pages lock                      (try_to_unmap_one)
    ->lruvec->lru_lock                  (follow_page_mask->mark_page_accessed)
    ->lruvec->lru_lock                  (check_pte_range->folio_isolate_lru)
    ->private_lock                      (folio_remove_rmap_pte->set_page_dirty)
    ->i_pages lock                      (folio_remove_rmap_pte->set_page_dirty)
    bdi.wb->list_lock                   (folio_remove_rmap_pte->set_page_dirty)
    ->inode->i_lock                     (folio_remove_rmap_pte->set_page_dirty)
    bdi.wb->list_lock                   (zap_pte_range->set_page_dirty)
    ->inode->i_lock                     (zap_pte_range->set_page_dirty)
    ->private_lock                      (zap_pte_range->block_dirty_folio)

璇锋鏌ヨ繖浜涙敞閲婄殑褰撳墠鐘舵€侊紝瀹冧滑鑷湰鏂囨。鎾板啓涔嬫椂璧峰彲鑳藉凡鍙戠敓鍙樺寲銆?

### 閿佸疄鐜扮粏鑺?


             鍏朵粬绾у埆椤佃〃鐨勯攣瑙勫垯銆?

### 椤佃〃閿佺粏鑺?


          琚竴涓?VMA 鎵€鍖呭惈銆傛湁鍏虫垜浠浣曞鐞嗚鎯呭喌鐨勭粏鑺傦紝璇峰弬瑙佷笂闈㈠叧浜庨潪 VMA 椤佃〃閬嶅巻鐨勫皬鑺傘€?

闄や簡涓婅堪鏈灏忚妭涓弿杩扮殑閿佷箣澶栵紝鎴戜滑杩樻湁涓撶敤浜庨〉琛ㄧ殑棰濆閿侊細

- **鏇撮珮绾у埆鐨勯〉琛ㄩ攣** - 鏇撮珮绾у埆鐨勯〉琛紝鍗?PGD銆丳4D 鍜?PUD锛屽湪淇敼鏃跺悇鑷娇鐢ㄤ互杩涚▼鍦板潃绌洪棿涓虹矑搴︾殑 :c`!mm->page_table_lock` 閿併€?
- **缁嗙矑搴﹂〉琛ㄩ攣** - PMD 鍜?PTE 鍚勮嚜鎷ユ湁缁嗙矑搴﹂攣锛岃繖浜涢攣瑕佷箞淇濆瓨鍦ㄦ弿杩伴〉琛ㄧ殑 folio 涓紝瑕佷箞鍦ㄨ缃?`!ALLOC_SPLIT_PTLOCKS` 鏃跺崟鐙垎閰嶅苟鐢?folio 鎸囧悜銆侾MD 鑷棆閿侀€氳繃 `!pmd_lock` 鑾峰彇锛岃€?PTE 琚槧灏勫埌楂樼鍐呭瓨锛堝鏋滄槸 32 浣嶇郴缁燂級骞堕€氳繃 `!pte_offset_map_lock` 灏忓績鍦板姞閿併€?

杩欎簺閿佷唬琛ㄤ簡涓庢瘡涓〉琛ㄧ骇鍒氦浜掓墍闇€鐨勬渶浣庤姹傦紝浣嗚繕鏈夎繘涓€姝ョ殑瑕佹眰銆?

閲嶈鐨勬槸锛屾敞鎰忓湪椤佃〃**閬嶅巻**鏃讹紝鏈夋椂涓嶄細鑾峰彇姝ょ被閿併€傜劧鑰岋紝鍦?PTE 绾у埆锛岃嚦灏戝繀椤婚槻姝㈠苟鍙戠殑椤佃〃鍒犻櫎锛堜娇鐢?RCU锛夛紝骞朵笖椤佃〃蹇呴』琚槧灏勫埌楂樼鍐呭瓨锛岃涓嬫枃銆?

鏄惁灏忓績鍦拌鍙栭〉琛ㄦ潯鐩彇鍐充簬鏋舵瀯锛岃瑙佷笅闈㈢殑鍘熷瓙鎬у皬鑺傘€?

##### 閿佽鍒?


鎴戜滑鍦ㄤ笌椤佃〃浜や簰鏃跺缓绔嬪熀鏈殑閿佽鍒欙細

- 褰撲慨鏀逛竴涓〉琛ㄦ潯鐩椂锛?*蹇呴』**鎸佹湁璇ラ〉琛ㄧ殑椤佃〃閿侊紝闄ら潪浣犺兘瀹夊叏鍦板亣璁炬病鏈変汉鍙互骞跺彂璁块棶杩欎簺椤佃〃锛堜緥濡傚湪璋冪敤 `!free_pgtables` 鏃讹級銆?
- 瀵归〉琛ㄦ潯鐩殑璇诲彇鍜屽啓鍏ュ繀椤绘槸**鎭板綋**鍘熷瓙鐨勩€傝瑙佷笅闈㈢殑鍘熷瓙鎬у皬鑺傘€?
- 濉厖鍏堝墠涓虹┖鐨勬潯鐩姹傛寔鏈?mmap 鎴?VMA 閿侊紙璇绘垨鍐欙級锛屼粎浣跨敤 rmap 閿佹潵杩欐牱鍋氭槸鍗遍櫓鐨勶紙瑙佷笅闈㈢殑璀﹀憡锛夈€?
- 濡傚墠鎵€杩帮紝娓呯┖鍙互鍦ㄤ粎浠呬繚鎸?VMA 绋冲畾鐨勬儏鍐典笅鎵ц锛屽嵆鎸佹湁 mmap銆乂MA 鎴?rmap 閿佷腑鐨勪换鎰忎竴涓€?

             `!vms_clear_ptes` 鍦ㄦ竻绌猴紙缁忕敱 `!unmap_vmas`锛夊拰閲婃斁椤佃〃锛堢粡鐢?`!free_pgtables`锛変箣闂存湁涓€涓椂闂寸獥鍙ｏ紝姝ゆ椂 VMA 鍦?rmap 鏍戜腑浠嶇劧鍙銆俙!free_pgtables` 鍋囪娓呯┖宸茬粡鎵ц锛屽苟鏃犳潯浠跺湴绉婚櫎 PTE锛堣繛鍚岄噴鏀捐寖鍥村唴鎵€鏈夊叾浠栭〉琛級锛屽洜姝ゅ湪姝ゆ椂瀹夎鏂扮殑 PTE 鏉＄洰鍙兘娉勬紡鍐呭瓨锛屽苟瀵艰嚧鍏朵粬鎰忓涓斿嵄闄╃殑琛屼负銆?

绉诲姩椤佃〃鏃惰繕鏈夐澶栫殑閫傜敤瑙勫垯锛屾垜浠湪涓嬮潰鍏充簬璇ヤ富棰樼殑灏忚妭涓璁恒€?

PTE 绾у埆鐨勯〉琛ㄤ笉鍚屼簬鍏朵粬绾у埆鐨勯〉琛紝璁块棶瀹冧滑鏈夐澶栬姹傦細

- 鍦?32 浣嶆灦鏋勪笂锛屽畠浠彲鑳戒綅浜庨珮绔唴瀛橈紙鎰忓懗鐫€闇€瑕佽鏄犲皠鍒板唴鏍稿唴瀛樻墠鑳借闂級銆?
- 褰撲负绌烘椂锛屽畠浠彲浠ュ湪鎸佹湁 mmap 閿佹垨 rmap 閿佽繘琛岃鍙栥€佸苟涓?PTE 鍜?PMD 椤佃〃閿佺粨鍚堢殑鎯呭喌涓嬭瑙ｉ櫎閾炬帴骞?RCU 閲婃斁銆傜壒鍒槸锛岃繖鍦ㄥ鐞?`!MADV_COLLAPSE` 鏃剁殑 `!retract_page_tables` 涓彂鐢熴€傚洜姝よ闂?PTE 绾у埆椤佃〃鑷冲皯瑕佹眰鎸佹湁涓€涓?RCU 璇婚攣锛涗絾杩欏彧閫傜敤浜庤兘澶熷蹇嶄笌骞跺彂椤佃〃鏇存柊绔炰簤鐨勮鑰咃紝鍗宠瀵熷埌涓€涓┖鐨?PTE锛堝湪瀹為檯涓婂凡琚垎绂诲苟鏍囪涓?RCU 閲婃斁鐨勯〉琛ㄤ腑锛夛紝鑰屽彟涓€涓柊鐨勯〉琛ㄥ凡瀹夎鍦ㄧ浉鍚屼綅缃苟濉叆浜嗘潯鐩€傚啓鑰呴€氬父闇€瑕佽幏鍙?PTE 閿侊紝骞堕噸鏂伴獙璇?PMD 鏉＄洰浠嶇劧鎸囧悜鍚屼竴涓?PTE 绾у埆椤佃〃銆傚鏋滃啓鑰呬笉鍏冲績鏄惁涓哄悓涓€涓?PTE 绾у埆椤佃〃锛屽畠鍙互鑾峰彇 PMD 閿佸苟閲嶆柊楠岃瘉 pmd 鏉＄洰鐨勫唴瀹逛粛鐒舵弧瓒宠姹傘€傜壒鍒槸锛岃繖鍦ㄥ鐞?`!MADV_COLLAPSE` 鏃剁殑 `!retract_page_tables` 涓篃浼氬彂鐢熴€?

瑕佽闂?PTE 绾у埆椤佃〃锛屽彲浠ユ牴鎹ǔ瀹氭€ц姹備娇鐢?`!pte_offset_map_lock` 鎴?`!pte_offset_map` 涔嬬被鐨勮緟鍔╁嚱鏁般€傝繖浜涘嚱鏁颁細鍦ㄩ渶瑕佹椂鎶婇〉琛ㄦ槧灏勫埌鍐呮牳鍐呭瓨锛岃幏鍙?RCU 閿侊紝骞舵牴鎹彉浣撳彲鑳借繕浼氭煡鎵炬垨鑾峰彇 PTE 閿併€傚弬瑙?`!pte_offset_map_lock` 涓婄殑娉ㄩ噴銆?

##### 鍘熷瓙鎬?


鏃犺椤佃〃閿佸浣曪紝MMU 纭欢閮戒細骞跺彂鍦版洿鏂拌闂綅鍜岃剰浣嶏紙鍙兘鏇村锛屽彇鍐充簬鏋舵瀯锛夈€傛澶栵紝骞惰鐨勯〉琛ㄩ亶鍘嗘搷浣滐紙灏界淇濇寔浜?VMA 绋冲畾锛変互鍙婂儚 GUP-fast 杩欐牱鐨勫姛鑳戒細鏃犻攣鍦伴亶鍘嗭紙鍗宠鍙栵級椤佃〃锛岀敋鑷冲畬鍏ㄤ笉淇濇寔 VMA 绋冲畾銆?

褰撴墽琛岄〉琛ㄩ亶鍘嗗苟淇濇寔 VMA 绋冲畾鏃讹紝璇诲彇鏄惁蹇呴』鍙繘琛屼竴娆′笖浠呬竴娆★紝鍙栧喅浜庢灦鏋勶紙渚嬪 x86-64 涓嶉渶瑕佷换浣曠壒娈婇闃叉帾鏂斤級銆?

濡傛灉姝ｅ湪鎵ц鍐欏叆锛屾垨鑰呬竴娆¤鍙栧喅瀹氫簡鏄惁鍙戠敓鍐欏叆锛堜緥濡傚湪瀹夎椤佃〃鏉＄洰鏃讹紝渚嬪 `!__pud_install`锛夛紝鍒欏繀椤诲缁堢壒鍒皬蹇冦€傚湪杩欎簺鎯呭喌涓嬶紝鎴戜滑姘歌繙涓嶈兘鍋囪椤佃〃閿佺粰浜嗘垜浠畬鍏ㄧ嫭鍗犵殑璁块棶锛屽苟涓斿繀椤诲彧鑾峰彇涓€娆￠〉琛ㄦ潯鐩€?

濡傛灉鎴戜滑姝ｅ湪璇诲彇椤佃〃鏉＄洰锛岄偅涔堟垜浠彧闇€纭繚缂栬瘧鍣ㄤ笉浼氶噸鎺掓垜浠殑鍔犺浇銆傝繖閫氳繃 `!pXXp_get` 鍑芥暟瀹炵幇鈥斺€擿!pgdp_get`銆乣!p4dp_get`銆乣!pudp_get`銆乣!pmdp_get` 鍜?`!ptep_get`銆?

瀹冧滑涓殑姣忎竴涓兘浣跨敤 `!READ_ONCE` 鏉ヤ繚璇佺紪璇戝櫒鍙鍙栭〉琛ㄦ潯鐩竴娆°€?

鐒惰€岋紝濡傛灉鎴戜滑鎯宠鎿嶄綔涓€涓幇鏈夌殑椤佃〃鏉＄洰骞跺叧蹇冨厛鍓嶅瓨鍌ㄧ殑鏁版嵁锛屾垜浠繀椤绘洿杩涗竴姝ワ紝浣跨敤纭欢鍘熷瓙鎿嶄綔锛屼緥濡傚湪 `!ptep_get_and_clear` 涓€?

鍚屾牱锛屼笉渚濊禆浜庢寔鏈夌ǔ瀹?VMA 鐨勬搷浣滐紝渚嬪 GUP-fast锛堝弬瑙?`!gup_fast` 鍙婂叾鍚勭椤佃〃绾у埆澶勭悊绋嬪簭濡?`!gup_fast_pte_range`锛夛紝蹇呴』闈炲父灏忓績鍦颁笌椤佃〃鏉＄洰浜や簰锛屼娇鐢?`!ptep_get_lockless` 浠ュ強鏇撮珮绾у埆椤佃〃瀵瑰簲鐨勭瓑浠峰嚱鏁般€?

瀵归〉琛ㄦ潯鐩殑鍐欏叆涔熷繀椤绘槸鎭板綋鍘熷瓙鐨勶紝杩欑敱 `!set_pXX` 鍑芥暟纭珛鈥斺€擿!set_pgd`銆乣!set_p4d`銆乣!set_pud`銆乣!set_pmd` 鍜?`!set_pte`銆?

鍚屾牱锛屾竻闄ら〉琛ㄦ潯鐩殑鍑芥暟涔熷繀椤绘槸鎭板綋鍘熷瓙鐨勶紝濡?`!pXX_clear` 鍑芥暟鈥斺€擿!pgd_clear`銆乣!p4d_clear`銆乣!pud_clear`銆乣!pmd_clear` 鍜?`!pte_clear`銆?

##### 椤佃〃瀹夎


椤佃〃瀹夎鏄€氳繃 mmap 鎴?VMA 閿佷互璇绘垨鍐欐ā寮忔樉寮忎繚鎸?VMA 绋冲畾鏉ユ墽琛岀殑锛堟湁鍏冲師鍥犵殑缁嗚妭锛岃鍙傝閿佽鍒欏皬鑺備腑鐨勮鍛婏級銆?

褰撳垎閰嶄竴涓?P4D銆丳UD 鎴?PMD 骞跺湪涓婅堪 PGD銆丳4D 鎴?PUD 涓缃浉鍏虫潯鐩椂锛屽繀椤绘寔鏈?:c`!mm->page_table_lock`銆傝繖鍒嗗埆鍦?`!__p4d_alloc`銆乣!__pud_alloc` 鍜?`!__pmd_alloc` 涓幏鍙栥€?

   `!pud_lockptr` 鍒欎害鐒讹紝涓嶈繃鍦ㄦ挵鍐欐湰鏂囨椂瀹冩渶缁堝紩鐢ㄧ殑鏄?:c`!mm->page_table_lock`銆?

鍒嗛厤涓€涓?PTE 灏嗕娇鐢?:c`!mm->page_table_lock`锛屾垨鑰咃紝濡傛灉瀹氫箟浜?`!USE_SPLIT_PMD_PTLOCKS`锛屽垯浣跨敤宓屽叆鍦?PMD 鐗╃悊椤靛厓鏁版嵁涓殑涓€涓?:c`!struct ptdesc` 褰㈠紡鐨勯攣锛岀敱浠?`!pmd_lock` 璋冪敤鐨?`!pmd_ptdesc` 鑾峰彇锛屽苟鏈€缁堢敱 `!__pte_alloc` 鑾峰彇銆?

鏈€鍚庯紝淇敼 PTE 鐨勫唴瀹归渶瑕佺壒娈婂鐞嗭紝鍥犱负 PTE 椤佃〃閿佸繀椤诲湪鎴戜滑鎯宠瀵?PTE 涓寘鍚殑鏉＄洰杩涜绋冲畾涓旂嫭鍗犵殑璁块棶鏃惰幏鍙栵紝灏ゅ叾鏄綋鎴戜滑鎯宠淇敼瀹冧滑鏃躲€?

杩欓€氳繃 `!pte_offset_map_lock` 鎵ц锛屽畠浼氬皬蹇冨湴妫€鏌ヤ互纭繚 PTE 娌℃湁鍦ㄦ垜浠涔嬩笅鍙戠敓鍙樺寲锛屾渶缁堣皟鐢?`!pte_lockptr` 鏉ヨ幏鍙栧寘鍚湪鍏宠仈浜庣墿鐞?PTE 椤电殑 :c`!struct ptdesc` 涓殑銆佷互 PTE 涓虹矑搴︾殑鑷棆閿併€傝閿佸繀椤婚€氳繃 `!pte_unmap_unlock` 閲婃斁銆?

   `!pte_offset_map_rw_nolock`锛屽綋鎴戜滑鐭ラ亾鎴戜滑淇濇寔浜?PTE 绋冲畾鏃垛€斺€斾絾涓轰簡绠€娲佹垜浠笉鎺㈣瀹冦€傛湁鍏虫洿澶氱粏鑺傦紝璇峰弬瑙?`!pte_offset_map_lock` 鐨勬敞閲娿€?

褰撲慨鏀硅寖鍥村唴鐨勬暟鎹椂锛屾垜浠€氬父鍙笇鏈涙寜闇€鍒嗛厤鏇撮珮绾у埆鐨勯〉琛紝浣跨敤杩欎簺閿佹潵閬垮厤绔炰簤鎴栬鐩栦换浣曞唴瀹癸紝骞舵寜闇€鍦?PTE 绾у埆璁剧疆/娓呴櫎鏁版嵁锛堜緥濡傚湪缂洪〉鎴栨竻绌烘椂锛夈€?

閬嶅巻椤佃〃鏉＄洰浠ュ畨瑁呮柊鏄犲皠鏃堕噰鍙栫殑鍏稿瀷妯″紡鏄紝涔愯鍦扮‘瀹氫笂涓€绾ч〉琛ㄤ腑鐨勯〉琛ㄦ潯鐩槸鍚︿负绌猴紝濡傛灉鏄紝鍒欎粎姝ゆ椂鑾峰彇椤佃〃閿佸苟鍐嶆妫€鏌ュ畠鏄惁鍦ㄦ垜浠箣涓嬭鍒嗛厤銆?

杩欎娇寰楅〉琛ㄩ攣鍙湪瀹為檯闇€瑕佹椂鎵嶈鑾峰彇銆傝繖鏂归潰鐨勪竴涓緥瀛愭槸 `!__pud_alloc`銆?

鍦ㄥ彾瀛愰〉琛紝鍗?PTE锛屾垜浠笉鑳藉畬鍏ㄤ緷璧栬繖绉嶆ā寮忥紝鍥犱负鎴戜滑鏈夌嫭绔嬬殑 PMD 鍜?PTE 閿侊紝鑰?THP 鎶樺彔锛坈ollapse锛変緥濡傚彲鑳藉凡缁忎粠鎴戜滑涔嬩笅娑堥櫎浜?PMD 鏉＄洰浠ュ強 PTE銆?

杩欏氨鏄负浠€涔?`!pte_offset_map_lock` 鏃犻攣鍦版绱?PTE 鐨?PMD 鏉＄洰锛屽皬蹇冨湴妫€鏌ュ畠鏄惁绗﹀悎棰勬湡锛岀劧鍚庡啀鑾峰彇 PTE 鐗瑰畾鐨勯攣锛岀劧鍚?*鍐嶆**妫€鏌?PMD 鏉＄洰鏄惁绗﹀悎棰勬湡銆?

濡傛灉鍙戠敓 THP 鎶樺彔锛堟垨绫讳技鎯呭喌锛夛紝鍒欎袱涓〉涓婄殑閿侀兘灏嗚鑾峰彇锛屽洜姝ゆ垜浠彲浠ュ湪鎸佹湁 PTE 閿佺殑鍚屾椂闃叉杩欑鎯呭喌鍙戠敓銆?

浠ヨ繖绉嶆柟寮忓畨瑁呮潯鐩‘淇濅簡鍐欏叆鐨勪簰鏂ャ€?

##### 椤佃〃閲婃斁


鎷嗛櫎椤佃〃鏈韩鏄竴浠堕渶瑕佹瀬澶у皬蹇冪殑浜嬫儏銆傜粷涓嶈兘鏈夊姙娉曡琚寚瀹氱Щ闄ょ殑椤佃〃琚苟鍙戜换鍔￠亶鍘嗘垨寮曠敤銆?

浠呮寔鏈?mmap 鍐欓攣鍜?VMA 閿侊紙杩欏皢闃绘绔炰簤鐨勭己椤靛拰 rmap 鎿嶄綔锛夋槸涓嶅鐨勶紝鍥犱负鏂囦欢鍚庡鏄犲皠鍙互鍦ㄤ粎 :c`!struct address_space->i_mmap_rwsem` 涔嬩笅琚埅鏂€?

鍥犳锛屼换浣曞彲閫氳繃鍙嶅悜鏄犲皠璁块棶鐨?VMA锛堟棤璁烘槸閫氳繃 :c`!struct anon_vma->rb_root` 杩樻槸 :c:member:`!struct address_space->i_mmap` 鍖洪棿鏍戯級閮戒笉鑳芥媶闄ゅ叾椤佃〃銆?

璇ユ搷浣滈€氬父閫氳繃 `!free_pgtables` 鎵ц锛屽畠鍋囪瑕佷箞宸茬粡鑾峰彇浜?mmap 鍐欓攣锛堢敱鍏?:c`!mm_wr_locked` 鍙傛暟鎸囧畾锛夛紝瑕佷箞 VMA 宸茬粡涓嶅彲杈俱€?

瀹冨皬蹇冨湴浠庢墍鏈夊弽鍚戞槧灏勪腑绉婚櫎 VMA锛屼絾鏄噸瑕佺殑鏄紝涓嶈兘鏈変换浣曟柊鐨勫弽鍚戞槧灏勪笌鏈?VMA 閲嶅彔锛屼篃涓嶈兘淇濈暀浠讳綍鍏佽璁块棶姝ｅ湪鎷嗛櫎椤佃〃鐨勮寖鍥村唴鐨勫湴鍧€鐨勯€斿緞銆?

姝ゅ锛屽畠鍋囪宸茬粡鎵ц浜嗕竴娆℃竻绌猴紝骞朵笖宸茬粡閲囧彇浜嗘帾鏂界‘淇濆湪娓呯┖涓庤皟鐢?`!free_pgtables` 涔嬮棿涓嶈兘鍐嶅畨瑁呬换浣曢〉琛ㄦ潯鐩€?

鐢变簬鍋囪鎵€鏈夋绫绘楠ら兘宸叉墽琛岋紝椤佃〃鏉＄洰鍦ㄦ病鏈夐〉琛ㄩ攣鐨勬儏鍐典笅琚竻闄わ紙鍦?`!pgd_clear`銆乣!p4d_clear`銆乣!pud_clear` 鍜?`!pmd_clear` 鍑芥暟涓級銆?

          涓婇潰鐨勯〉琛紝姝ｅ `!retract_page_tables` 鎵€鍋氱殑閭ｆ牱锛屽畠鍦?i_mmap 璇婚攣銆丳MD 鍜?PTE 椤佃〃閿佷笅鎵ц锛屾病鏈夎繖绉嶇骇鍒殑璋ㄦ厧銆?

##### 椤佃〃绉诲姩


涓€浜涘嚱鏁版搷浣?PMD 浠ヤ笂鐨勯〉琛ㄧ骇鍒紙鍗?PUD銆丳4D 鍜?PGD 椤佃〃锛夈€傚叾涓渶鍊煎緱娉ㄦ剰鐨勬槸 `!mremap`锛屽畠鑳藉绉诲姩鏇撮珮绾у埆鐨勯〉琛ㄣ€?

鍦ㄨ繖浜涙儏鍐典笅锛岃姹傝幏鍙?*鎵€鏈?*閿侊紝鍗?mmap 閿併€乂MA 閿佸拰鐩稿叧鐨?rmap 閿併€?

浣犲彲浠ュ湪 `!mremap` 瀹炵幇涓殑 `!take_rmap_locks` 鍜?`!drop_rmap_locks` 鍑芥暟涓瀵熷埌杩欎竴鐐癸紝瀹冧滑鎵ц閿佽幏鍙栫殑 rmap 涓€渚э紝鏈€缁堢敱 `!move_page_tables` 璋冪敤銆?

### VMA 閿佸唴閮ㄦ満鍒?


##### 姒傝堪


VMA 璇婚攣瀹屽叏鏄箰瑙傜殑鈥斺€斿鏋滈攣瀛樺湪绔炰簤锛屾垨鑰呬竴涓珵浜夌殑鍐欏叆宸茬粡寮€濮嬶紝閭ｄ箞鎴戜滑涓嶄細鑾峰彇璇婚攣銆?

VMA **璇?*閿侀€氳繃 `!lock_vma_under_rcu` 鑾峰彇锛屽畠棣栧厛璋冪敤 `!rcu_read_lock` 浠ョ‘淇濆湪 RCU 涓寸晫鍖轰腑鏌ユ壘 VMA锛岀劧鍚庡皾璇曢€氳繃 `!vma_start_read` 瀵?VMA 鍔犻攣锛屾渶鍚庨€氳繃 `!rcu_read_unlock` 閲婃斁 RCU 閿併€?

鍦ㄧ敤鎴峰凡缁忔寔鏈?mmap 璇婚攣鐨勬儏鍐典笅锛屽彲浠ヤ娇鐢?`!vma_start_read_locked` 鍜?`!vma_start_read_locked_nested`銆傝繖浜涘嚱鏁颁笉浼氬洜閿佺珵浜夎€屽け璐ワ紝浣嗚皟鐢ㄨ€呬粛搴旀鏌ュ畠浠殑杩斿洖鍊硷紝浠ラ槻瀹冧滑鍥犲叾浠栧師鍥犲け璐ャ€?

VMA 璇婚攣鍦ㄥ叾鎸佺画鏈熼棿閫掑 :c`!vma.vm_refcnt` 寮曠敤璁℃暟鍣紝鑰?`!lock_vma_under_rcu` 鐨勮皟鐢ㄨ€呭繀椤婚€氳繃 `!vma_end_read` 灏嗗叾閫掑噺銆?

VMA **鍐?*閿侀€氳繃 `!vma_start_write` 鍦?VMA 鍗冲皢琚慨鏀圭殑瀹炰緥涓幏鍙栵紝涓?`!vma_start_read` 涓嶅悓锛岃閿佹€绘槸琚幏鍙栥€俶map 鍐欓攣鐨勬寔缁椂闂村繀椤昏鐩?VMA 鍐欓攣锛岄噴鏀炬垨闄嶇骇 mmap 鍐欓攣涔熶細閲婃斁 VMA 鍐欓攣锛屽洜姝ゆ病鏈?`!vma_end_write` 鍑芥暟銆?

娉ㄦ剰锛屽綋瀵?VMA 閿佽繘琛?
鍐欓攣鏃讹紝:c`!vma.vm_refcnt` 琚复鏃朵慨鏀癸紝浠ヤ究璇昏€呰兘澶熸娴嬪埌鍐欒€呯殑瀛樺湪銆備竴鏃︾敤浜庝覆琛屽寲鐨?vma 搴忓垪鍙疯鏇存柊锛岃寮曠敤璁℃暟鍣ㄥ氨浼氳鎭㈠銆?

杩欑‘淇濅簡鎴戜滑鎵€闇€鐨勮涔夆€斺€擵MA 鍐欓攣鎻愪緵瀵?VMA 鐨勭嫭鍗犲啓璁块棶銆?

##### 瀹炵幇缁嗚妭


VMA 閿佹満鍒舵棬鍦ㄦ垚涓轰竴绉嶈交閲忕骇鐨勬墜娈碉紝浠ラ伩鍏嶄娇鐢ㄧ珵浜夋縺鐑堢殑 mmap 閿併€傚畠閫氳繃缁勫悎浣跨敤灞炰簬鍖呭惈鐨?:c`!struct mm_struct` 鍜?VMA 鐨勫紩鐢ㄨ鏁板櫒鍜屽簭鍒楀彿鏉ュ疄鐜般€?

璇婚攣閫氳繃 `!vma_start_read` 鑾峰彇锛岃繖鏄竴涓箰瑙傛搷浣滐紝鍗冲畠灏濊瘯鑾峰彇璇婚攣锛屼絾濡傛灉鏃犳硶鑾峰彇鍒欒繑鍥?false銆傚湪璇绘搷浣滅粨鏉熸椂锛岃皟鐢?`!vma_end_read` 鏉ラ噴鏀?VMA 璇婚攣銆?

璋冪敤 `!vma_start_read` 瑕佹眰鍏堣皟鐢?`!rcu_read_lock`锛屽缓绔嬪湪鑾峰彇 VMA 璇婚攣鏃舵垜浠浜?RCU 涓寸晫鍖轰箣涓€備竴鏃﹁幏鍙栵紝RCU 閿佸氨鍙互琚噴鏀撅紝鍥犱负瀹冧粎鐢ㄤ簬鏌ユ壘銆傝繖鐢?`!lock_vma_under_rcu` 鎶借薄锛屽畠鏄敤鎴峰簲褰撲娇鐢ㄧ殑鎺ュ彛銆?

鍐欏叆瑕佹眰 mmap 琚啓閿侀攣瀹氾紝骞朵笖 VMA 閿侀€氳繃 `!vma_start_write` 鑾峰彇锛屼絾鍐欓攣鐢?mmap 鍐欓攣鐨勭粓姝㈡垨闄嶇骇鏉ラ噴鏀撅紝鍥犳涓嶉渶瑕?`!vma_end_write`銆?

鎵€鏈夎繖涓€鍒囬兘鏄€氳繃姣?mm 鍜屾瘡-VMA 鐨勫簭鍒楄鏁板疄鐜扮殑锛岀敤浜庨檷浣庡鏉傛€э紝灏ゅ叾鏄浜庨偅浜涗竴娆℃€у啓閿佸涓?VMA 鐨勬搷浣溿€?

濡傛灉 mm 搴忓垪璁℃暟 :c`!mm->mm_lock_seq` 绛変簬 VMA 搴忓垪璁℃暟 :c`!vma->vm_lock_seq`锛屽垯 VMA 琚啓閿侀攣瀹氥€傚鏋滃畠浠笉鍚岋紝鍒欎笉鏄€?

姣忔鍦?`!mmap_write_unlock` 鎴?`!mmap_write_downgrade` 涓噴鏀?mmap 鍐欓攣鏃讹紝閮戒細璋冪敤 `!vma_end_write_all`锛屽畠杩樹細閫氳繃 `!mm_lock_seqcount_end` 閫掑 :c`!mm->mm_lock_seq`銆?

杩欐牱锛屾垜浠‘淇濇棤璁?VMA 鐨勫簭鍒楀彿濡備綍锛岄兘涓嶄細閿欒鍦版寚绀哄啓閿侊紝骞朵笖褰撴垜浠噴鏀?mmap 鍐欓攣鏃讹紝鎴戜滑楂樻晥鍦板悓鏃堕噴鏀句簡 mmap 鍐呭寘鍚殑**鎵€鏈?* VMA 鍐欓攣銆?

鐢变簬 mmap 鍐欓攣鎺掍粬浜庢寔鏈夊畠鐨勫叾浠栨柟锛屽湪鍏堕噴鏀炬椂鑷姩閲婃斁浠讳綍 VMA 閿佹槸鏈夋剰涔夌殑锛屽洜涓轰綘缁濅笉浼氭兂瑕佸湪瀹屽叏鐙珛鐨勫啓鎿嶄綔涔嬮棿淇濇寔 VMA 琚攣銆?

瀹冭繕鏈夊姪浜庣淮鎸佹纭殑閿侀『搴忋€?

姣忔鑾峰彇涓€涓?VMA 璇婚攣锛屾垜浠€掑 :c`!vma.vm_refcnt` 寮曠敤璁℃暟鍣紝骞舵鏌?VMA 鐨勫簭鍒楄鏁版槸鍚︿笌 mm 鐨勪笉鍖归厤銆?

濡傛灉涓嶅尮閰嶏紝璇婚攣澶辫触骞堕€掑噺 :c`!vma.vm_refcnt`銆傚鏋滃尮閰嶏紝鎴戜滑淇濇寔寮曠敤璁℃暟鍣ㄥ崌楂橈紝鎺掗櫎鍐欒€咃紝浣嗗厑璁稿叾浠栬鑰咃紝瀹冧滑涔熷彲浠ュ湪 RCU 涓嬭幏鍙栨閿併€?

閲嶈鐨勬槸锛宍!lock_vma_under_rcu` 涓墽琛岀殑 maple tree 鎿嶄綔涔熸槸 RCU 瀹夊叏鐨勶紝鍥犳鏁翠釜璇婚攣鎿嶄綔淇濊瘉姝ｇ‘杩愯銆?

鍦ㄥ啓鐨勪竴渚э紝鎴戜滑鍦?:c`!vma.vm_refcnt` 涓缃竴涓鑰呮棤娉曚慨鏀圭殑浣嶏紝骞剁瓑寰呮墍鏈夎鑰呴€掑噺瀹冧滑鐨勫紩鐢ㄨ鏁般€備竴鏃︽病鏈夎鑰咃紝VMA 鐨勫簭鍒楀彿琚缃负涓?mm 鐨勭浉鍖归厤銆傚湪鏁翠釜鎿嶄綔鏈熼棿鎸佹湁 mmap 鍐欓攣銆?

杩欐牱锛屽鏋滄湁浠讳綍璇婚攣鐢熸晥锛宍!vma_start_write` 灏嗕紤鐪犵洿鍒板畠浠畬鎴愶紝浠庤€屽疄鐜颁簰鏂ャ€?

鍦ㄨ缃?VMA 鐨勫簭鍒楀彿涔嬪悗锛屾寚绀哄啓鑰呯殑 :c`!vma.vm_refcnt` 涓殑浣嶈娓呴櫎銆備粠姝ゆ椂璧凤紝VMA 鐨勫簭鍒楀彿灏嗘寚绀?VMA 鐨勫啓閿佺姸鎬侊紝鐩村埌 mmap 鍐欓攣琚涪寮冩垨闄嶇骇銆?

杩欑寮曠敤璁℃暟鍣ㄥ拰搴忓垪鍙风殑宸у缁勫悎锛屼娇寰楀熀浜?RCU 鐨勫揩閫熸瘡-VMA 閿佽幏鍙栵紙灏ゅ叾鍦ㄧ己椤垫椂锛屽敖绠′篃鍦ㄥ叾浠栧湴鏂逛娇鐢級鑳藉浠ユ渶灏忕殑閿侀『搴忓鏉傚害瀹炵幇銆?

### mmap 鍐欓攣闄嶇骇


褰撴寔鏈変竴涓?mmap 鍐欓攣鏃讹紝浣犳嫢鏈夊 mmap 鍐呰祫婧愮殑鐙崰璁块棶锛堥€氬父闇€瑕佹敞鎰忚姹?VMA 鍐欓攣浠ラ伩鍏嶄笌鎸佹湁 VMA 璇婚攣鐨勪换鍔＄珵浜夛級銆?

鐒跺悗鍙互閫氳繃 `!mmap_write_downgrade` 灏嗗啓閿?*闄嶇骇**涓鸿閿侊紝瀹冧笌 `!mmap_write_unlock` 绫讳技锛岄€氳繃 `!vma_end_write_all` 闅愬紡缁堟鎵€鏈?VMA 鍐欓攣锛屼絾閲嶈鐨勬槸鍦ㄩ檷绾ф椂骞朵笉鏀惧純 mmap 閿侊紝鍥犳淇濇寔琚攣瀹氱殑铏氭嫙鍦板潃绌洪棿绋冲畾銆?

鐢辨浜х敓涓€涓湁瓒ｇ殑缁撴灉锛氶檷绾у悗鐨勯攣鎺掍粬浜庝换浣曞叾浠栨寔鏈夐檷绾ч攣鐨勪换鍔★紙鍥犱负涓€涓珵浜夌殑浠诲姟蹇呴』鍏堣幏鍙栧啓閿佹墠鑳藉皢鍏堕檷绾э紝鑰岄檷绾ч攣浼氶樆姝㈠湪鍘熷閿佽閲婃斁涔嬪墠鑾峰彇鏂扮殑鍐欓攣锛夈€?

涓烘竻鏅拌捣瑙侊紝鎴戜滑灏嗚锛圧锛?闄嶇骇鍐欙紙D锛?鍐欙紙W锛夐攣鐩镐簰瀵圭収锛屾樉绀哄摢浜涢攣鎺掓枼鍏朵粬閿侊細

   :widths: 5 5 5 5
   :header-rows: 1
   :stub-columns: 1

   - -
     - R
     - D
     - W
   - - R
     - N
     - N
     - Y
   - - D
     - N
     - Y
     - Y
   - - W
     - Y
     - Y
     - Y

杩欓噷 Y 琛ㄧず鍖归厤琛?鍒楃殑閿佹槸浜掓枼鐨勶紝N 琛ㄧず瀹冧滑涓嶆槸銆?

### 鏍堟墿灞?


鏍堟墿灞曞甫鏉ヤ簡棰濆鐨勫鏉傛€э紝鍥犱负鎴戜滑涓嶅厑璁稿瓨鍦ㄧ珵浜夌殑缂洪〉锛屽洜姝ゆ垜浠湪 `!expand_downwards` 鎴?`!expand_upwards` 涓皟鐢?`!vma_start_write` 鏉ラ槻姝㈣繖绉嶆儏鍐点€?

### 鍑芥暟涓庣粨鏋勪綋
