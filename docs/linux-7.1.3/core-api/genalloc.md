## The genalloc/genpool subsystem


鍐呮牳涓湁璁稿鍐呭瓨鍒嗛厤瀛愮郴缁燂紝姣忎釜閮介拡瀵圭壒瀹氱殑闇€姹傘€傜劧鑰岋紝鏈夋椂鍐呮牳寮€鍙戣€呴渶瑕佷负鐗瑰畾鑼冨洿鐨勪笓鐢ㄥ唴瀛樺疄鐜颁竴涓柊鐨勫垎閰嶅櫒锛涜繖浜涘唴瀛橀€氬父浣嶄簬鏌愬鐨勮澶囦笂銆傝璁惧鐨勯┍鍔ㄤ綔鑰呭綋鐒跺彲浠ュ啓涓€涓皬鍒嗛厤鍣ㄦ潵瀹屾垚宸ヤ綔锛屼絾閭ｆ鏄敤鍑犲崄涓祴璇曚笉浣崇殑鍒嗛厤鍣ㄥ婊″唴鏍哥殑閫斿緞銆傛棭鍦?2005 骞达紝Jes Sorensen 浠?sym53c8xx_2 椹卞姩涓彁鍙栦簡鍏朵腑涓€涓垎閰嶅櫒锛屽苟灏嗗叾浣滀负涓€涓敤浜庡垱寤轰复鏃讹紙ad hoc锛夊唴瀛樺垎閰嶅櫒鐨勯€氱敤妯″潡鍙戝竷_銆傝繖娈典唬鐮佸湪 2.6.13 鐗堟湰涓鍚堝苟锛涗粠閭ｄ互鍚庡凡缁忓仛浜嗗ぇ閲忎慨鏀广€?

浣跨敤姝ゅ垎閰嶅櫒鐨勪唬鐮佸簲鍖呭惈 <linux/genalloc.h>銆備竴鍒囧浜庝娇鐢ㄤ互涓嬩箣涓€鍒涘缓涓€涓睜锛?
   :functions: gen_pool_create		

   :functions: devm_gen_pool_create

璋冪敤 gen_pool_create() 灏嗗垱寤轰竴涓睜銆傚垎閰嶇殑绮掑害鐢?min_alloc_order 璁剧疆锛涘畠鏄竴涓互 2 涓哄簳鐨勫鏁帮紝绫讳技浜庨〉鍒嗛厤鍣ㄤ娇鐢ㄧ殑閭ｄ簺锛屼絾瀹冩寚鐨勬槸瀛楄妭鑰屼笉鏄〉銆傚洜姝わ紝濡傛灉 min_alloc_order 浼犲叆 3锛岄偅涔堟墍鏈夊垎閰嶉兘灏嗘槸 8 瀛楄妭鐨勫€嶆暟銆傚澶?min_alloc_order 浼氬噺灏戣窡韪睜涓唴瀛樻墍闇€鐨勫唴瀛樸€俷id 鍙傛暟鎸囧畾搴斾娇鐢ㄥ摢涓?NUMA 鑺傜偣鏉ュ垎閰嶅唴閮ㄧ翱璁扮粨鏋勶紱濡傛灉璋冪敤鑰呬笉鍏冲績锛屽彲浠ヤ紶鍏?-1銆?
"鎵樼"鎺ュ彛 devm_gen_pool_create() 灏嗘睜缁戝畾鍒扮壒瀹氳澶囥€傞櫎姝や箣澶栵紝瀹冧細鍦ㄧ粰瀹氳澶囪閿€姣佹椂鑷姩娓呯悊璇ユ睜銆?
浣跨敤浠ヤ笅鏂瑰紡鍏抽棴涓€涓睜锛?
   :functions: gen_pool_destroy

鍊煎緱娉ㄦ剰鐨勬槸锛屽鏋滅粰瀹氭睜涓粛鏈夋湭閲婃斁鐨勫垎閰嶏紝姝ゅ嚱鏁颁細閲囧彇鐩稿綋鏋佺鐨勬楠も€斺€旇皟鐢?BUG()锛屼娇鏁翠釜绯荤粺宕╂簝銆備綘宸茬粡琚鍛婁簡銆?
鏂板垱寤虹殑姹犳病鏈夊彲鍒嗛厤鐨勫唴瀛樸€傚湪杩欑鐘舵€佷笅瀹冪浉褰撴棤鐢紝鍥犳棣栧厛瑕佸仛鐨勯€氬父灏辨槸鍚戞睜涓坊鍔犲唴瀛樸€傝繖鍙互閫氳繃浠ヤ笅涔嬩竴瀹屾垚锛?
   :functions: gen_pool_add

   :functions: gen_pool_add_owner

璋冪敤 gen_pool_add() 浼氬皢浠?addr锛堝湪鍐呮牳铏氭嫙鍦板潃绌洪棿涓級寮€濮嬬殑 size 瀛楄妭鍐呭瓨鏀惧叆缁欏畾姹狅紝鍐嶆浣跨敤 nid 浣滀负杈呭姪鍐呭瓨鍒嗛厤鐨勮妭鐐?ID銆俫en_pool_add_virt() 鍙樹綋灏嗘樉寮忕殑鐗╃悊鍦板潃涓庤鍐呭瓨鍏宠仈锛涗粎褰撹姹犲皢鐢ㄤ簬 DMA 鍒嗛厤鏃舵墠闇€瑕佽繖鏍峰仛銆?
鐢ㄤ簬浠庢睜涓垎閰嶅唴瀛橈紙浠ュ強灏嗗叾褰掕繕锛夌殑鍑芥暟鏄細

   :functions: gen_pool_alloc

   :functions: gen_pool_dma_alloc

   :functions: gen_pool_free_owner

姝ｅ浜轰滑鎵€鏈熸湜鐨勶紝gen_pool_alloc() 灏嗕粠缁欏畾姹犱腑鍒嗛厤 size< 瀛楄妭銆俫en_pool_dma_alloc() 鍙樹綋鍒嗛厤鐢ㄤ簬 DMA 鎿嶄綔鐨勫唴瀛橈紝骞跺湪 dma 鎵€鎸囧悜鐨勭┖闂翠腑杩斿洖鍏宠仈鐨勭墿鐞嗗湴鍧€銆傝繖鍙湁鍦ㄥ唴瀛樻槸閫氳繃 gen_pool_add_virt() 娣诲姞鏃舵墠鏈夋晥銆傝娉ㄦ剰锛屾鍑芥暟鍋忕浜嗛€氬父浣跨敤 unsigned long 鍊艰〃绀哄唴鏍稿湴鍧€鐨?genpool 妯″紡锛涘畠鍙嶈€岃繑鍥炰竴涓?void *銆?
杩欎簺閮界湅璧锋潵鐩稿绠€鍗曪紱浜嬪疄涓婏紝涓€浜涘紑鍙戣€呮樉鐒惰寰楀畠澶畝鍗曚簡銆傛瘯绔燂紝涓婇潰鐨勬帴鍙ｆ棤娉曟帶鍒跺垎閰嶅嚱鏁板浣曢€夋嫨瑕佽繑鍥炵殑鍝竴鍧楃壒瀹氬唴瀛樸€傚鏋滈渶瑕佽繖绫绘帶鍒讹紝浠ヤ笅鍑芥暟浼氬紩璧蜂綘鐨勫叴瓒ｏ細

   :functions: gen_pool_alloc_algo_owner

   :functions: gen_pool_set_algo

浣跨敤 gen_pool_alloc_algo() 鐨勫垎閰嶄細鎸囧畾涓€涓敤浜庨€夋嫨寰呭垎閰嶅唴瀛樼殑绠楁硶锛涢粯璁ょ畻娉曞彲浠ラ€氳繃 gen_pool_set_algo() 璁剧疆銆俤ata 鍊间細浼犻€掔粰绠楁硶锛涘ぇ澶氭暟浼氬拷鐣ュ畠锛屼絾鍋跺皵浼氶渶瑕併€傝嚜鐒讹紝鍙互缂栧啓涓€涓笓鐢ㄧ畻娉曪紝浣嗗凡缁忔湁涓€濂楃浉褰撲赴瀵岀殑鍙敤绠楁硶锛?
- gen_pool_first_fit 鏄竴涓畝鍗曠殑棣栨閫傞厤锛坒irst-fit锛夊垎閰嶅櫒锛涘鏋滄湭鎸囧畾鍏朵粬绠楁硶锛岃繖鏄粯璁ょ畻娉曘€?
- gen_pool_first_fit_align 寮哄埗鍒嗛厤鍏锋湁鐗瑰畾鐨勫榻愶紙閫氳繃 genpool_data_align 缁撴瀯涓殑 data 浼犲叆锛夈€?
- gen_pool_first_fit_order_align 灏嗗垎閰嶅榻愬埌澶у皬鐨勬鏁帮紙order锛夈€備緥濡傦紝涓€涓?60 瀛楄妭鐨勫垎閰嶅皢鍥犳涓?64 瀛楄妭瀵归綈銆?
- gen_pool_best_fit锛屾濡備汉浠墍鏈熸湜鐨勶紝鏄竴涓畝鍗曠殑鏈€浣抽€傞厤锛坆est-fit锛夊垎閰嶅櫒銆?
- gen_pool_fixed_alloc 鍦ㄦ睜鍐呯壒瀹氱殑鍋忕Щ锛堥€氳繃 data 鍙傛暟鍦?genpool_data_fixed 缁撴瀯涓紶鍏ワ級澶勫垎閰嶃€傚鏋滄寚绀虹殑鍐呭瓨涓嶅彲鐢紝鍒欏垎閰嶅け璐ャ€?
杩樻湁灏戞暟鍏朵粬鍑芥暟锛屼富瑕佺敤浜庢煡璇㈡睜涓彲鐢ㄧ┖闂存垨閬嶅巻鍐呭瓨鍧楃瓑鐩殑銆傜劧鑰岋紝澶у鏁扮敤鎴峰簲璇ヤ笉闇€瑕佽秴鍑轰笂杩版弿杩扮殑鍐呭銆傚垢杩愮殑璇濓紝璁╂洿澶氫汉浜嗚В杩欎釜妯″潡灏嗘湁鍔╀簬闃叉灏嗘潵缂栧啓涓撶敤鍐呭瓨鍒嗛厤鍣ㄣ€?
   :functions: gen_pool_virt_to_phys

   :functions: gen_pool_for_each_chunk

   :functions: gen_pool_has_addr

   :functions: gen_pool_avail

   :functions: gen_pool_size

   :functions: gen_pool_get

   :functions: of_gen_pool_get
