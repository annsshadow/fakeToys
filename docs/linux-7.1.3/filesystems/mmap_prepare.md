
## mmap_prepare 鍥炶皟浣跨敤鎸囧崡


## 绠€浠?


`struct file->f_op->mmap()` 鍥炶皟宸茶搴熷純锛屽洜涓哄畠鏃㈠瓨鍦ㄧǔ瀹氭€ч棶棰橈紝涔熷瓨鍦ㄥ畨鍏ㄩ闄╋紝骞朵笖涓嶆€绘槸鍏佽鐩搁偦鏄犲皠鐨勫悎骞讹紝浠庤€屽鑷翠笉蹇呰鐨勫唴瀛樼鐗囥€?

瀹冨凡琚?`file->f_op->mmap_prepare()` 鍥炶皟鍙栦唬锛岃鍥炶皟瑙ｅ喅浜嗚繖浜涢棶棰樸€?

姝ら挬瀛愬湪鍑芥暟鏄犲皠寤虹珛鐨勮捣濮嬮樁娈佃璋冪敤锛岄噸瑕佺殑鏄畠鍦ㄤ换浣曠浉閭绘槧灏勫悎骞跺彂鐢?*涔嬪墠**琚皟鐢ㄣ€?

鑻ュ湪鏄犲皠鏃朵骇鐢熼敊璇紝閿欒鍙兘鍦ㄦ鍥炶皟琚皟鐢ㄤ箣鍚庢墠鍑虹幇锛屽洜姝ゅ簲灏嗗叾瑙嗕负瀹炶川涓婃棤鐘舵€佺殑銆?

涔熷氨鏄鈥斺€斾笉搴斿垎閰嶄换浣曡祫婧愶紝涔熶笉搴旀洿鏂颁换浣曠姸鎬佹潵鍙嶆槧鏄犲皠宸茬粡寤虹珛锛屽洜涓烘槧灏勫彲鑳藉湪鍥炶皟瀹屾垚鍚庤鍚堝苟锛屾垨鑰呮槧灏勫け璐ャ€?

### 宸叉槧灏勫洖璋?


濡傛灉闇€瑕佷负姣忎釜鏄犲皠鍒嗛厤璧勬簮锛屾垨鑰呴渶瑕佹搷浣滆濡傚紩鐢ㄨ鏁颁箣绫荤殑鐘舵€侊紝搴斿綋閫氳繃 `vm_ops->mapped` 閽╁瓙鏉ュ畬鎴愶紝璇ラ挬瀛愭湰韬簲鐢?mmap_prepare 閽╁瓙璁剧疆銆?

浠呭綋涓€涓柊鐨勬槧灏勫缓绔嬩笖鏈笌鍏朵粬鏄犲皠鍚堝苟鏃讹紝鎵嶄細璋冪敤姝ゅ洖璋冿紱骞朵笖瀹冧細鍦ㄦ槧灏勫缓绔嬩箣鍓嶄笉鍙兘鍙戠敓閿欒鐨勬椂鍒昏璋冪敤銆?

浣犲彲浠ュ悜璇ュ洖璋冩湰韬繑鍥為敊璇紝杩欏皢瀵艰嚧鏄犲皠琚彇娑堟槧灏勶紝骞跺悜 mmap() 璋冪敤鑰呰繑鍥為敊璇€傝繖鍦ㄩ渶瑕佸垎閰嶈祫婧愩€佽€屽垎閰嶅彲鑳藉け璐ョ殑鎯呭喌涓嬪緢鏈夌敤銆?

## 濡備綍浣跨敤


鍦ㄤ綘鐨勯┍鍔ㄧ殑 struct file_operations 缁撴瀯浣撲腑锛屾寚瀹氫竴涓?`mmap_prepare` 鍥炶皟锛岃€屼笉鏄?`mmap` 鍥炶皟锛屼緥濡傚浜?ext4锛?


    const struct file_operations ext4_file_operations = {
        ...
        .mmap_prepare    = ext4_file_mmap_prepare,
    };

鍏剁鍚嶄负 `int (**mmap_prepare)(struct vm_area_desc **)`銆?

瑙傚療 struct vm_area_desc 绫诲瀷锛?


    struct vm_area_desc {
        /** Immutable state. **/
        const struct mm_struct *const mm;
        struct file **const file; /** May vary from vm_file in stacked callers. */
        unsigned long start;
        unsigned long end;

        /** Mutable fields. Populated with initial state. **/
        pgoff_t pgoff;
        struct file *vm_file;
        vma_flags_t vma_flags;
        pgprot_t page_prot;

        /** Write-only fields. **/
        const struct vm_operations_struct *vm_ops;
        void *private_data;

        /** Take further action? **/
        struct mmap_action action;
    };

杩欏緢鐩存帴鈥斺€斾綘鎷ユ湁璁剧疆鏄犲皠鎵€闇€鐨勬墍鏈夊瓧娈碉紝骞朵笖鍙互鏇存柊鍙彉鐨勪笌鍙啓鐨勫瓧娈碉紝渚嬪锛?


    static int ext4_file_mmap_prepare(struct vm_area_desc *desc)
    {
        int ret;
        struct file *file = desc->file;
        struct inode *inode = file->f_mapping->host;

        ...

        file_accessed(file);
        if (IS_DAX(file_inode(file))) {
            desc->vm_ops = &ext4_dax_vm_ops;
            vma_desc_set_flags(desc, VMA_HUGEPAGE_BIT);
        } else {
            desc->vm_ops = &ext4_file_vm_ops;
        }
        return 0;
    }

閲嶈鐨勬槸锛屾洿鏂拌繖浜涘瓧娈垫椂浣犱笉鍐嶉渶瑕佸湪寮曠敤璁℃暟鎴栭攣涓婄粫鏉ョ粫鍘烩€斺€?*浣犲彲浠ョ洿鎺ュ幓淇敼瀹冧滑**銆?

涓€鍒囬兘鐢辨槧灏勪唬鐮佽礋璐ｅ鐞嗐€?

### VMA 鏍囧織


闅忕潃 `mmap_prepare`锛孷MA 鏍囧織涔熺粡鍘嗕簡涓€娆″ぇ淇€備互鍓嶄綘浼氳皟鐢?vm_flags_init()銆乿m_flags_reset()銆乿m_flags_set()銆乿m_flags_clear() 鍜?vm_flags_mod() 涓殑涓€涓潵淇敼鏍囧織锛堝苟璁╅攣琚纭墽琛岋級锛岀幇鍦ㄨ繖宸蹭笉鍐嶅繀瑕併€?

姝ゅ锛岄€氳繃 `VM_READ`銆乣VM_WRITE` 绛夋寚瀹?VMA 鏍囧織鐨勪紶缁熸柟寮忊€斺€斿嵆浣跨敤 `-VM_xxx` 瀹忊€斺€斾篃鍙戠敓浜嗗彉鍖栥€?

鍦ㄥ疄鐜?mmap_prepare() 鏃讹紝閫氳繃浣嶅彿鏉ュ紩鐢ㄦ爣蹇楋紝瀹氫箟涓?`VMA_xxx_BIT` 瀹忥紝渚嬪 `VMA_READ_BIT`銆乣VMA_WRITE_BIT` 绛夛紝骞朵娇鐢ㄤ笅鍒楀嚱鏁颁箣涓€锛堝叾涓?`desc` 鏄寚鍚?struct vm_area_desc 鐨勬寚閽堬級锛?

- `vma_desc_test_any(desc, ...)` - 鎸囧畾涓€涓互閫楀彿鍒嗛殧鐨勬爣蹇楀垪琛ㄦ潵娴嬭瘯锛堜换鎰忔爣蹇楁槸鍚﹁璁剧疆锛夛紝渚嬪鈥斺€擿`vma_desc_test_any(desc, VMA_WRITE_BIT, VMA_MAYWRITE_BIT)`` 濡傛灉浠讳竴鏍囧織琚缃垯杩斿洖 `true`锛屽惁鍒欒繑鍥?`false`銆?
- `vma_desc_set_flags(desc, ...)` - 鏇存柊 VMA 鎻忚堪绗︽爣蹇椾互璁剧疆鐢遍€楀彿鍒嗛殧鐨勫垪琛ㄦ墍鎸囧畾鐨勯檮鍔犳爣蹇楋紝渚嬪鈥斺€擿vma_desc_set_flags(desc, VMA_PFNMAP_BIT, VMA_IO_BIT)`銆?
- `vma_desc_clear_flags(desc, ...)` - 鏇存柊 VMA 鎻忚堪绗︽爣蹇椾互娓呴櫎鐢遍€楀彿鍒嗛殧鐨勫垪琛ㄦ墍鎸囧畾鐨勬爣蹇楋紝渚嬪鈥斺€擿`vma_desc_clear_flags(desc, VMA_WRITE_BIT, VMA_MAYWRITE_BIT)``銆?

## 鎿嶄綔


鐜板湪浣犲彲浠ラ潪甯稿鏄撳湴閫氳繃瀵?struct vm_area_desc 鎸囬拡璋冪敤绠€鍗曠殑杈呭姪鍑芥暟锛屽湪鏄犲皠寤虹珛鍚庡鍏舵墽琛屾搷浣溿€傝繖浜涜緟鍔╁嚱鏁板寘鎷細

- mmap_action_remap() - 瀵圭敱鐗瑰畾澶у皬鐨勪竴缁?PFN 缁勬垚銆佽捣濮嬩簬鏌愪釜铏氭嫙鍦板潃鍜?PFN 缂栧彿鐨勮寖鍥磋繘琛岄噸鏄犲皠銆?

- mmap_action_remap_full() - 涓?mmap_action_remap() 鐩稿悓锛屽彧鏄粠 `start_pfn` 寮€濮嬮噸鏄犲皠鏁翠釜鏄犲皠銆?

- mmap_action_ioremap() - 涓?mmap_action_remap() 鐩稿悓锛屽彧鏄墽琛屼竴娆?I/O 閲嶆槧灏勩€?

- mmap_action_ioremap_full() - 涓?mmap_action_ioremap() 鐩稿悓锛屽彧鏄粠 `start_pfn` 寮€濮嬮噸鏄犲皠鏁翠釜鏄犲皠銆?

- mmap_action_simple_ioremap() - 浠庢寚瀹氱殑鐗╃悊鍦板潃寮€濮嬨€佽鐩栨寚瀹氶暱搴︼紝寤虹珛涓€涓?I/O 閲嶆槧灏勩€?

- mmap_action_map_kernel_pages() - 鍦?VMA 涓粠鐗瑰畾鍋忕Щ澶勬槧灏勪竴缁勬寚瀹氱殑 `struct page` 鎸囬拡銆?

- mmap_action_map_kernel_pages_full() - 鍦ㄦ暣娈?VMA 涓婃槧灏勪竴缁勬寚瀹氱殑 `struct page` 鎸囬拡銆傝皟鐢ㄨ€呭繀椤荤‘淇濋〉鏁扮粍涓湁瓒冲鐨勬潯鐩潵瑕嗙洊鎵€鎻忚堪鐨?VMA 鐨勬暣涓寖鍥淬€?

**娉ㄦ剰锛?* `action` 瀛楁閫氬父缁濅笉搴旇鐩存帴鎿嶄綔锛岃€屽簲褰撲娇鐢ㄨ繖浜涜緟鍔╁嚱鏁颁箣涓€銆?
