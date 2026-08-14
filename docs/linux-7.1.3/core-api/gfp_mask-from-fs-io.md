
## 鍦?FS/IO 涓婁笅鏂囦腑浣跨敤鐨?GFP 鎺╃爜


:Date: May, 2018
:Author: Michal Hocko <mhocko@kernel.org>

## 绠€浠?

鏂囦欢绯荤粺锛坒ilesystem锛夊拰 IO 鏍堜腑鐨勪唬鐮佽矾寰勫湪鍒嗛厤鍐呭瓨鏃跺繀椤诲皬蹇冿紝浠ラ槻姝㈢敱鐩存帴鍐呭瓨鍥炴敹锛坉irect memory reclaim锛夊洖璋冨埌 FS 鎴?IO 璺緞銆佸苟闃诲鍦ㄥ凡鎸佹湁璧勬簮锛堜緥濡傞攣鈥斺€旀渶甯歌鐨勬槸鐢ㄤ簬浜嬪姟涓婁笅鏂囩殑閭ｄ簺閿侊級涓婃墍寮曡捣鐨勯€掑綊姝婚攣銆?
浼犵粺鐨勯伩鍏嶆姝婚攣闂鐨勬柟寮忔槸锛屽湪璋冪敤鍒嗛厤鍣ㄦ椂娓呴櫎 gfp 鎺╃爜涓殑 __GFP_FS 鎴栫浉搴斿湴 __GFP_IO锛堟敞鎰忓悗鑰呬篃闅愬惈浜嗘竻闄ゅ墠鑰咃級銆侴FP_NOFS 鎴栫浉搴斿湴 GFP_NOIO 鍙敤浣滃揩鎹锋柟寮忋€備絾浜嬪疄璇佹槑锛屼笂杩版柟娉曞凡琚互鐢細鍙楅檺鐨?gfp 鎺╃爜琚?浠ラ槻涓囦竴"鍦颁娇鐢紝鑰屾病鏈夋洿娣卞叆鐨勮€冮噺锛岃繖浼氬鑷撮棶棰橈紝鍥犱负杩囧害浣跨敤 GFP_NOFS/GFP_NOIO 鍙兘瀵艰嚧鍐呭瓨杩囧害鍥炴敹锛坥ver-reclaim锛夋垨鍏朵粬鍐呭瓨鍥炴敹闂銆?
## 鏂?API


鑷?4.12 璧凤紝鎴戜滑鏈変簡鐢ㄤ簬 NOFS 鍜?NOIO 涓婁笅鏂囩殑閫氱敤浣滅敤鍩燂紙scope锛堿PI锛歚memalloc_nofs_save`銆乣memalloc_nofs_restore` 浠ュ強鐩稿簲鐨?`memalloc_noio_save`銆乣memalloc_noio_restore`锛屽畠浠厑璁稿皢鏌愪釜浣滅敤鍩熸爣璁颁负浠庢枃浠剁郴缁熸垨 I/O 瑙掑害鐪嬬殑涓寸晫鍖恒€傝浣滅敤鍩熷唴鐨勪换浣曞垎閰嶉兘浼氳嚜鍔ㄤ粠缁欏畾鐨勬帺鐮佷腑鍘绘帀 __GFP_FS 鎴栫浉搴旂殑 __GFP_IO锛屽洜姝ゆ病鏈変换浣曞唴瀛樺垎閰嶈兘澶熼€掑綊鍥炲埌 FS/IO 涓€?
   :functions: memalloc_nofs_save memalloc_nofs_restore
   :functions: memalloc_noio_save memalloc_noio_restore

FS/IO 浠ｇ爜闅忓悗鍙渶鍦ㄥ紑鍚换浣曠浉瀵逛簬鍥炴敹鑰岃█鐨勪复鐣屽尯涔嬪墠鈥斺€斾緥濡備笌鍥炴敹涓婁笅鏂囧叡浜殑閿侊紝鎴栬€呭彲鑳介€氳繃鍥炴敹鍙戠敓浜嬪姟涓婁笅鏂囧祵濂楁椂鈥斺€旇皟鐢ㄧ浉搴旂殑 save 鍑芥暟銆傚綋涓寸晫鍖虹粨鏉熸椂搴斿綋璋冪敤 restore 鍑芥暟銆傜悊鎯虫儏鍐典笅锛屾墍鏈夎繖浜涢兘搴旈檮甯︿竴娈佃В閲婏紝璇存槑鍥炴敹涓婁笅鏂囨槸浠€涔堬紝浠ヤ究缁存姢銆?
璇锋敞鎰忥紝save/restore 鍑芥暟鐨勬纭厤瀵瑰厑璁稿祵濂楋紝鍥犳浠庡凡鏈夌殑 NOIO 鎴?NOFS 浣滅敤鍩熶腑璋冪敤 `memalloc_noio_save` 鎴栫浉搴旂殑 `memalloc_noio_restore` 鏄畨鍏ㄧ殑銆?
## __vmalloc(GFP_NOFS) 鍛?

鑷?v5.17 璧凤紝鐗瑰埆鏄湪鎻愪氦 451769ebb7e79锛?mm/vmalloc: alloc GFP_NO{FS,IO} for vmalloc"锛変箣鍚庯紝GFP_NOFS/GFP_NOIO 鐜板湪宸查€氳繃闅愬紡浣跨敤浣滅敤鍩?API 鍦?`[k]vmalloc` 涓緱鍒版敮鎸併€?
鍦ㄦ棭鏈熷唴鏍镐腑锛宍vmalloc` 涓嶆敮鎸?GFP_NOFS 璇箟锛屽洜涓哄垎閰嶅櫒鍐呴儴娣卞鏈夌‖缂栫爜鐨?GFP_KERNEL 鍒嗛厤銆傝繖鎰忓懗鐫€浣跨敤 GFP_NOFS/GFP_NOIO 璋冪敤 `vmalloc` 鍑犱箮鎬绘槸涓€涓?bug銆?
鍦ㄧ悊鎯虫儏鍐典笅锛屼笂灞傚簲璇ュ凡缁忔爣璁颁簡鍗遍櫓涓婁笅鏂囷紝鍥犳鏃犻渶鐗瑰埆灏忓績锛宍vmalloc` 搴斿彲姣棤闂鍦拌璋冪敤銆傛湁鏃讹紝濡傛灉涓婁笅鏂囧苟涓嶆竻鏅版垨瀛樺湪鍒嗗眰杩濊锛岄偅涔堬紙鍦?v5.17 涔嬪墠鐨勫唴鏍镐笂锛夋帹鑽愮殑鍙橀€氭柟娉曟槸鐢变綔鐢ㄥ煙 API 鍖呰９ `vmalloc`锛屽苟闄勪笂娉ㄩ噴璇存槑闂銆?