## I915 Small BAR RFC 绔犺妭

浠?DG2 寮€濮嬶紝鎴戜滑灏嗕负璁惧鏈湴鍐呭瓨锛堝嵆 I915_MEMORY_CLASS_DEVICE锛夋彁渚涘彲璋冩暣澶у皬鐨?BAR 鏀寔锛屼絾鍦ㄦ煇浜涙儏鍐典笅锛屾渶缁堢殑 BAR 澶у皬鍙兘浠嶇劧灏忎簬鎬荤殑 probed_size銆傚湪杩欑鎯呭喌涓嬶紝鍙湁 I915_MEMORY_CLASS_DEVICE 鐨勪竴閮ㄥ垎鍙 CPU 璁块棶锛堜緥濡傚墠 256M锛夛紝鍏朵綑閮ㄥ垎鍙兘閫氳繃 GPU 璁块棶銆?
### I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS 鏍囧織

鏂扮殑 gem_create_ext 鏍囧織锛岀敤浜庡憡璇夊唴鏍告煇涓?BO 灏嗛渶瑕?CPU 璁块棶銆傚綋灏嗗璞℃斁缃湪 I915_MEMORY_CLASS_DEVICE 涓椂杩欎竴鐐瑰緢閲嶈锛屽洜涓哄簳灞傝澶囩殑 BAR 杈冨皬锛屾剰鍛崇潃鍏朵腑鍙湁涓€閮ㄥ垎鍙 CPU 璁块棶銆傚鏋滄病鏈夎鏍囧織锛屽唴鏍镐細鍋囧畾涓嶉渶瑕?CPU 璁块棶锛屽苟浼樺厛浣跨敤 I915_MEMORY_CLASS_DEVICE 涓笉鍙 CPU 鐪嬪埌鐨勯儴鍒嗐€?
   :functions: __drm_i915_gem_create_ext

### probed_cpu_visible_size 灞炴€?
鏂扮殑 struct __drm_i915_memory_region 灞炴€э紝杩斿洖鐗瑰畾鍖哄煙涓彲琚?CPU 璁块棶閮ㄥ垎鐨勬€诲ぇ灏忋€傝繖搴斾粎閫傜敤浜?I915_MEMORY_CLASS_DEVICE銆傛垜浠悓鏃舵姤鍛?unallocated_cpu_visible_size 鍜?unallocated_size銆?
Vulkan 闇€瑕佹灞炴€э紝浣滀负鍒涘缓甯︽湁 VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT 鏍囧織鐨勭嫭绔?VkMemoryHeap 鐨勪竴閮ㄥ垎锛屼互琛ㄧず CPU 鍙鐨勯儴鍒嗭紝鍏朵腑闇€瑕佺煡閬撳爢鐨勬€诲ぇ灏忋€傚畠杩橀渶瑕佽兘澶熷ぇ鑷翠及璁″唴瀛樺彲鑳借鍒嗛厤鐨勬儏鍐点€?
   :functions: __drm_i915_memory_region_info

### 閿欒鎹曡幏闄愬埗

閫氳繃閿欒鎹曡幏鎴戜滑鏈変袱涓柊鐨勯檺鍒讹細

    1) 鍦?small BAR 绯荤粺涓婇敊璇崟鑾锋槸灏藉姏鑰屼负鐨勶紱濡傛灉鍦ㄦ崟鑾锋椂椤甸潰涓嶅彲琚?CPU 璁块棶锛岄偅涔堝唴鏍稿彲浠ヨ烦杩囧皾璇曟崟鑾峰畠浠€?
    2) 鍦ㄧ嫭绔嬪紡浠ュ強杈冩柊鐨勯泦鎴愬钩鍙颁笂锛屾垜浠幇鍦ㄦ嫆缁濆湪鍙仮澶嶄笂涓嬫枃涓婅繘琛岄敊璇崟鑾枫€傛湭鏉ュ唴鏍稿彲鑳藉笇鏈涘湪閿欒鎹曡幏鏈熼棿杩涜 blit 鎿嶄綔锛屼緥濡傚綋鏌愪釜瀵硅薄褰撳墠涓嶅彲琚?CPU 璁块棶鏃躲€?