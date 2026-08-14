
## 寰呭姙浜嬮」鍒楄〃


鏈妭鍖呭惈鍐呮牳 DRM 鍥惧舰瀛愮郴缁熶腑涓€绯诲垪杈冨皬鐨勨€滄棩甯告竻鐞嗏€濅换鍔★紝閫傚悎浣滀负鏂版墜椤圭洰锛屾垨鍦ㄦ病鏈夌揣鎬ュ伐浣滅殑闂叉殗鏃ュ瓙鏉ュ仛銆?
### 闅惧害


涓轰簡鏂逛究鎸戦€変换鍔★紝灏嗕换鍔″垝鍒嗕负涓嶅悓鐨勯毦搴︾骇鍒細

Starter锛堝叆闂級锛氶€傚悎寮€濮嬫帴瑙?DRM 瀛愮郴缁熺殑濂戒换鍔°€?
Intermediate锛堜腑绾э級锛氶渶瑕佷竴浜涘湪 DRM 瀛愮郴缁熶腑宸ヤ綔鐨勭粡楠岋紝鎴栨煇浜涚壒瀹氱殑 GPU/鏄剧ず鍥惧舰鐭ヨ瘑銆傝皟璇曢棶棰樻椂锛屾渶濂芥墜澶存湁鐩稿叧纭欢锛堟垨宸查厤缃ソ鐨勮櫄鎷熼┍鍔級鍙緵娴嬭瘯銆?
Advanced锛堥珮绾э級锛氭鎵嬬殑浠诲姟锛岄渶瑕佸 DRM 瀛愮郴缁熷拰鍥惧舰涓婚鏈夎緝濂界殑鐞嗚В銆傞€氬父闇€瑕佹湁鐩稿叧纭欢鐢ㄤ簬寮€鍙戜笌娴嬭瘯銆?
Expert锛堜笓瀹讹級锛氬彧鏈夊綋浣犲凡缁忔垚鍔熷畬鎴愯繃涓€浜涙鎵嬬殑閲嶆瀯宸ヤ綔銆佸苟涓旀槸璇ョ壒瀹氶鍩熺殑涓撳鏃讹紝鎵嶅皾璇曡繖浜涗换鍔°€?
## 瀛愮郴缁熺骇閲嶆瀯


### 鍐呰仈灞曞紑 drm_simple_encoder_init()

杈呭姪鍑芥暟 `drm_simple_encoder_init()` 鍘熸湰鏄负浜嗙畝鍖?encoder 鐨勫垵濮嬪寲銆備絾瀹冨疄闄呬笂鍙槸鍦?atomic modesetting 涓?DRM 椹卞姩涔嬮棿澧炲姞浜嗕竴灞備腑闂村眰銆?
杩欓噷鐨勪换鍔″氨鏄Щ闄?`drm_simple_encoder_init()`銆傛壘鍒颁竴涓皟鐢?`drm_simple_encoder_init()` 鐨勯┍鍔紝灏嗚杈呭姪鍑芥暟鍐呰仈灞曞紑銆傝椹卞姩杩橀渶瑕佽嚜宸辩殑 `drm_encoder_funcs` 瀹炰緥銆?
鑱旂郴浜猴細Thomas Zimmermann锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欵asy

### 鐢ㄥ父瑙?atomic helper 鏇挎崲 struct drm_simple_display_pipe


鏁版嵁绫诲瀷 `struct drm_simple_display_pipe` 鍙婂叾杈呭姪鍑芥暟鍘熸湰鏄负浜嗙畝鍖栭┍鍔ㄥ紑鍙戙€備絾瀹冧滑瀹為檯涓婂彧鏄湪 atomic modesetting 涓?DRM 椹卞姩涔嬮棿澧炲姞浜嗕竴灞備腑闂村眰銆?
浠嶆湁涓€浜涢┍鍔ㄥ湪浣跨敤 `drm_simple_display_pipe`銆傝繖閲岀殑浠诲姟鏄皢瀹冧滑杞崲涓轰娇鐢ㄥ父瑙勭殑 atomic helper銆傛壘鍒颁竴涓皟鐢?`drm_simple_display_pipe_init()` 鐨勯┍鍔紝灏?`drm_simple_kms_helper.c` 涓殑鎵€鏈夎緟鍔╁嚱鏁板唴鑱斿埌璇ラ┍鍔ㄤ腑锛屼粠鑰屼笉鍐嶉渶瑕?simple-KMS 鎺ュ彛銆傚悓鏃惰鎸夌収椹卞姩绾﹀畾閲嶅懡鍚嶆墍鏈夊唴鑱旂殑鍑芥暟銆?
鑱旂郴浜猴細Thomas Zimmermann锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欵asy

### 绉婚櫎鑷畾涔夌殑 dumb_map_offset 瀹炵幇


鎵€鏈夊熀浜?GEM 鐨勯┍鍔ㄩ兘搴旀敼涓轰娇鐢?`drm_gem_create_mmap_offset()`銆傞€愪釜瀹℃煡鍚勪釜椹卞姩锛岀‘淇濆畠浠笌閫氱敤瀹炵幇鍏煎锛堝悇绉嶅疄鐜颁腑閬楃暀浜嗗ぇ閲忚繃鏃剁殑鍔犻攣浠ｇ爜锛夛紝鐒跺悗绉婚櫎鑷畾涔夊疄鐜般€?
鑱旂郴浜猴細Simona Vetter锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欼ntermediate

### 灏嗙幇鏈?KMS 椹卞姩杞崲涓?atomic modesetting


3.19 宸茬粡鎻愪緵浜?atomic modeset 鎺ュ彛涓庤緟鍔╁嚱鏁帮紝鍥犳鐜板湪鍙互杞崲椹卞姩浜嗐€傚儚 Wayland 鎴?Android 涓婄殑 Surfaceflinger 杩欐牱鐨勭幇浠ｅ悎鎴愬櫒闈炲父闇€瑕?atomic modeset 鎺ュ彛锛屾墍浠ヨ繖涓€鍒囬兘鍏充箮缇庡ソ鐨勬湭鏉ャ€?
鍏充簬 atomic 杞崲锛屾湁涓€浠借浆鎹㈡寚鍗?[^1^]_锛屼綘鎵€闇€瑕佺殑鍙槸涓€涓皻鏈浆鎹㈢殑椹卞姩瀵瑰簲鐨?GPU銆侺WN.net 涓婄殑鈥淎tomic mode setting design overview鈥濈郴鍒楁枃绔?[^2^]_ [^3^]_ 涔熷緢鏈夊府鍔┿€?
浣滀负杞崲鐨勪竴閮ㄥ垎锛岄┍鍔ㄨ繕闇€瑕佽浆鎹负 universal plane锛堝嵆灏?primary 涓?cursor 浣滀负姝ｈ鐨?plane 瀵硅薄鏆撮湶鍑烘潵锛夈€備笉杩囪繖閫氳繃鐩存帴浣跨敤鏂扮殑 atomic helper 椹卞姩鐨勫洖璋冩潵鍋氳瀹规槗寰楀銆?
  .. [^1^] https://blog.ffwll.ch/2014/11/atomic-modeset-support-for-kms-drivers.html
  .. [^2^] https://lwn.net/Articles/653071/
  .. [^3^] https://lwn.net/Articles/653466/

鑱旂郴浜猴細Simona Vetter锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欰dvanced

### 娓呯悊鍥寸粫 plane 鐨勮鍓潗鏍囨贩涔?

鎴戜滑鏈変竴涓緟鍔╁嚱鏁?`drm_plane_helper_check_update()` 鍙互姝ｇ‘澶勭悊杩欎釜闂锛屼絾瀹冩病鏈夎涓€鑷村湴浣跨敤銆傝繖涓棶棰樺簲褰撹淇锛屾渶濂藉湪 atomic helper 涓慨澶嶏紙鐒跺悗椹卞姩鍐嶅垏鎹㈠埌瑁佸壀鍚庣殑鍧愭爣锛夈€傚彲鑳借繕搴旇鎶婅繖涓緟鍔╁嚱鏁颁粠 `drm_plane_helper.c` 绉诲埌 atomic helper 涓紝浠ラ伩鍏嶆贩娣嗏€斺€旈偅涓枃浠朵腑鐨勫叾浠栬緟鍔╁嚱鏁伴兘鏄凡杩囨椂鐨勯仐鐣?helper銆?
鑱旂郴浜猴細Ville Syrj盲l盲锛孲imona Vetter锛岄┍鍔ㄧ淮鎶よ€?
闅惧害锛欰dvanced

### 鏀硅繘 plane 鐨?atomic_check helper


闄や簡涓婇潰鎻愬埌鐨勮鍓潗鏍囧锛屽綋鍓嶇殑 helper 杩樻湁涓€浜涗笉澶熺悊鎯崇殑鍦版柟锛?
- `drm_plane_helper_funcs->atomic_check` 浼氬鍚敤鎴栫鐢ㄧ殑 plane 閮借璋冪敤銆傚線濂戒簡璇磋繖浼氳椹卞姩鎰熷埌鍥版儜锛屽線鍧忎簡璇磋繖鎰忓懗鐫€褰?plane 鍦ㄦ病鏈?CRTC 鐨勬儏鍐典笅琚鐢ㄦ椂锛岄┍鍔ㄤ細宕╂簝銆傚敮涓€鐨勭壒娈婂鐞嗘槸鍦?plane state 缁撴瀯浣撲腑閲嶇疆鏁板€硷紝鑰岃繖浜涢噸缃簲褰撶Щ鍏?`drm_plane_funcs->atomic_duplicate_state` 鍑芥暟涓€?
- 涓€鏃﹀畬鎴愪笂杩板伐浣滐紝helper 灏卞彲浠ュ仠姝㈠绂佺敤鐨?plane 璋冪敤 `->atomic_check`銆?
- 鐒跺悗鎴戜滑鍙互閬嶅巻鎵€鏈夐┍鍔紝绉婚櫎閭ｄ簺澶氬皯鏈変簺浠や汉鍥版儜鐨?`plane_state->fb` 涓?`plane_state->crtc` 妫€鏌ャ€?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欰dvanced

### 灏嗘棭鏈熺殑 atomic 椹卞姩杞崲涓?async commit helper


鍦ㄥご涓€骞达紝atomic modeset helper 涓嶆敮鎸佸紓姝?闈為樆濉炴彁浜わ紙nonblocking commit锛夛紝姣忎釜椹卞姩閮戒笉寰椾笉鑷繁鎵嬪啓銆傜幇鍦ㄨ繖涓棶棰樺凡缁忎慨澶嶏紝浣嗕粛鏈夊ぇ閲忕幇鏈夐┍鍔ㄥ彲浠ヨ交鏉捐浆鎹㈠埌鏂扮殑鍩虹璁炬柦涓娿€?
杩欎簺 helper 鐨勪竴涓棶棰樻槸锛屽畠浠姹傞┍鍔ㄦ纭鐞?atomic commit 鐨勫畬鎴愪簨浠躲€備絾淇杩欎簺 bug 鏃犺濡備綍閮芥槸濂戒簨銆?
涓庢澶氬皯鐩稿叧鐨勬槸 `legacy_cursor_update` 杩欎釜 hack锛屽湪浣跨敤璇ユ爣蹇楃殑椹卞姩涓紝搴旇鐢?helper 涓柊鐨?`atomic_async_check`/`commit` 鍔熻兘鏉ユ浛鎹㈠畠銆?
鑱旂郴浜猴細Simona Vetter锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欰dvanced

### 閲嶅懡鍚?drm_atomic_state


KMS 妗嗘灦瀵?`state` 杩欎釜姒傚康浣跨敤浜嗕袱绉嶇暐鏈変笉鍚岀殑瀹氫箟銆傚浜庢煇涓粰瀹氬璞★紙plane銆丆RTC銆乪ncoder 绛夛紝鍗?`drm_$OBJECT_state`锛夛紝state 鏄瀵硅薄鐨勫畬鏁寸姸鎬併€傜劧鑰岋紝鍦ㄨ澶囩骇鍒紝`drm_atomic_state` 鎸囩殑鏄鏈夐檺鏁伴噺瀵硅薄鐨勪竴娆＄姸鎬佹洿鏂般€?
杩欎釜 state 骞朵笉鏄暣涓澶囩殑鐘舵€侊紝鑰屽彧鏄璁惧涓煇浜涘璞＄殑瀹屾暣鐘舵€併€傝繖浼氳 newcomers 鎰熷埌鍥版儜锛屽洜姝?`drm_atomic_state` 搴斿綋琚噸鍛藉悕涓烘洿娓呮櫚鐨勫悕瀛楋紝渚嬪 `drm_atomic_commit`銆?
闄や簡閲嶅懡鍚嶇粨鏋勪綋鏈韩涔嬪锛岃繖涔熸剰鍛崇潃瑕侀噸鍛藉悕涓€浜涚浉鍏冲嚱鏁帮紙`drm_atomic_state_alloc`銆乣drm_atomic_state_get`銆乣drm_atomic_state_put`銆乣drm_atomic_state_init`銆乣__drm_atomic_state_free` 绛夛級銆?
鑱旂郴浜猴細Maxime Ripard <mripard@kernel.org>

闅惧害锛欰dvanced

### atomic KMS 鐨勫悗缁伐浣?

`drm_atomic_helper.c` 鎻愪緵浜嗕竴鎵瑰湪鏂?atomic 椹卞姩鎺ュ彛涔嬩笂瀹炵幇閬楃暀 IOCTL 鐨勫嚱鏁般€傝繖瀵逛簬椹卞姩鐨勯€愭杞崲闈炲父鏈夌敤锛屼絾閬楁喚鐨勬槸浜岃€呯殑璇箟涓嶅尮閰嶈繃浜庝弗閲嶃€傚洜姝ら渶瑕佷竴浜涘悗缁伐浣滄潵璋冩暣鍑芥暟鎺ュ彛浠ヤ慨澶嶈繖浜涢棶棰橈細

- atomic 闇€瑕侀攣鑾峰彇涓婁笅鏂囥€傜洰鍓嶈繖鏄敤涓€浜涚碂绯曠殑 hack 闅愬紡浼犻€掔殑锛屽苟涓斿湪骞曞悗杩樼敤 `GFP_NOFAIL` 鍒嗛厤銆傛墍鏈夐仐鐣欒矾寰勯兘闇€瑕佹樉寮忓湴鍦ㄦ爤涓婂垎閰?acquire context锛岀劧鍚庢樉寮忓湴灏嗗叾浼犲叆椹卞姩锛屼互渚垮熀浜?atomic 鐨勯仐鐣欏嚱鏁板彲浠ヤ娇鐢ㄥ畠浠€?
  闄や簡涓€浜涢┍鍔ㄤ唬鐮佸锛岃繖椤瑰伐浣滃凡缁忓畬鎴愩€傚簲璇ラ€氳繃鍦?`drm_modeset_lock_all()` 涓姞鍏?`WARN_ON(!drm_drv_uses_atomic_modeset)` 鏉ュ畬鎴愯繖涓换鍔°€?
- 涓€澶ф壒 vtable hook 鐜板湪鏀惧湪浜嗛敊璇殑浣嶇疆锛欴RM 鍦ㄦ牳蹇?vfunc 琛紙鍛藉悕涓?`drm_foo_funcs`锛岀敤浜庡疄鐜扮敤鎴风┖闂?ABI锛変笌 helper 搴撶殑鍙€?hook锛堝懡鍚嶄负 `drm_foo_helper_funcs`锛屼粎渚涘唴閮ㄤ娇鐢級涔嬮棿鍋氫簡鍒掑垎銆傚叾涓竴浜?hook 搴旇浠?`_funcs` 绉诲埌 `_helper_funcs`锛屽洜涓哄畠浠笉灞炰簬鏍稿績 ABI銆傚浜庢瘡绉嶈繖鏍风殑鎯呭喌锛宍drm_crtc.h` 鐨?kerneldoc 涓兘鏈変竴鏉?`FIXME` 娉ㄩ噴銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

### 灏?Buffer Object 鍔犻攣杩佺Щ鍒?dma_resv_lock()


璁稿椹卞姩鏈夎嚜宸辩殑鎸夊璞″姞閿佹柟妗堬紝閫氬父浣跨敤 `mutex_lock()`銆傝繖缁?buffer 鍏变韩甯︽潵浜嗗悇绉嶅悇鏍风殑楹荤儲锛屽洜涓烘牴鎹┍鍔ㄦ槸 exporter 杩樻槸 importer锛屽姞閿佸眰绾т細棰犲€掕繃鏉ャ€?
涓轰簡瑙ｅ喅杩欎釜闂锛屾垜浠渶瑕佷竴涓爣鍑嗙殑鎸夊璞″姞閿佹満鍒讹紝鍗?`dma_resv_lock()`銆傝繖涓攣闇€瑕佷綔涓烘渶澶栧眰閿佹潵璋冪敤锛屽悓鏃剁Щ闄ゆ墍鏈夊叾浠栭┍鍔ㄧ壒瀹氱殑鎸夊璞￠攣銆傞棶棰樺湪浜庯紝鐢变簬 struct `dma_buf` 鐨?buffer 鍏变韩锛屽疄闄呮帹琛屽姞閿佺害瀹氱殑鍙樻洿浼氭槸涓€涓€渇lag day鈥濓紙闇€瑕佷竴娆℃€у垏鎹級銆?
闅惧害锛欵xpert

### 灏嗘棩蹇楄緭鍑鸿浆鎹负甯?drm_device 鍙傛暟鐨?drm_* 鍑芥暟


瀵逛簬鍙兘瀛樺湪澶氫釜瀹炰緥鐨勯┍鍔紝闇€瑕佸湪鏃ュ織涓尯鍒嗗摢涓槸鍝釜銆傜敱浜?`DRM_INFO`/`WARN`/`ERROR` 鍋氫笉鍒拌繖涓€鐐癸紝椹卞姩浣跨敤 `dev_info`/`warn`/`err` 鏉ュ仛杩欑鍖哄垎銆傜幇鍦ㄦ垜浠湁浜?drm 鎵撳嵃鍑芥暟鐨?`drm_*` 鍙樹綋锛屽洜姝ゅ彲浠ヨ杩欎簺椹卞姩閲嶆柊鏀瑰洖浣跨敤 drm 鏍煎紡鐨勭壒瀹氭棩蹇楁秷鎭€?
鍦ㄥ紑濮嬭繖绉嶈浆鎹箣鍓嶏紝璇疯仈绯荤浉鍏崇淮鎶よ€咃紝浠ョ‘淇濅綘鐨勫伐浣滀細琚悎鍏モ€斺€斿苟涓嶆槸鎵€鏈変汉閮借鍚?DRM dmesg 瀹忔洿濂姐€?
鑱旂郴浜猴細Sean Paul锛屼綘璁″垝杞崲鐨勯┍鍔ㄧ殑缁存姢鑰?
闅惧害锛歋tarter

### 灏嗛┍鍔ㄨ浆鎹负浣跨敤绠€鍗曠殑 modeset suspend/resume


澶у鏁伴┍鍔紙i915 鍜?nouveau 闄ゅ锛夊鏋滀娇鐢?`drm_atomic_helper_suspend`/`resume()`锛屽彲鑳藉彲浠ヨ浆鎹负浣跨敤 `drm_mode_config_helper_suspend`/`resume()`銆傛澶栵紝鏃╂湡鐨?atomic modeset 椹卞姩涓粛鐒跺瓨鍦ㄦ墜宸ョ紪鍐欑殑 atomic suspend/resume 浠ｇ爜銆?
鑱旂郴浜猴細浣犺鍒掕浆鎹㈢殑椹卞姩鐨勭淮鎶よ€?
闅惧害锛欼ntermediate

### 鍦ㄤ笉渚濊禆 fbdev 鐨勬儏鍐典笅閲嶆柊瀹炵幇 drm_fbdev_fb_ops 涓殑鍑芥暟


`drm_fbdev_fb_ops` 涓殑璁稿鍥炶皟鍑芥暟鍙互浠庝笉渚濊禆 fbdev 妯″潡鐨勮搴﹂噸鍐欎腑鍙楃泭銆傚叾涓竴浜?helper 杩樺彲浠ラ€氳繃浣跨敤 `struct iosys_map` 鑰岄潪瑁告寚閽堟潵杩涗竴姝ュ彈鐩娿€?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>锛孲imona Vetter

闅惧害锛欰dvanced

### 瀵?blitting 涓庢牸寮忚浆鎹㈠嚱鏁拌繘琛屽熀鍑嗘祴璇曚笌浼樺寲


蹇€熺粯鍒跺埌鏄剧ず鍐呭瓨瀵逛簬璁稿搴旂敤绋嬪簭鐨勬€ц兘鑷冲叧閲嶈銆?
鑷冲皯鍦?x86-64 涓婏紝`sys_imageblit()` 鏄庢樉姣?`cfb_imageblit()` 鎱紝灏界浜岃€呬娇鐢ㄧ浉鍚岀殑 blitting 绠楁硶锛岃€屼笖鍚庤€呮槸涓?I/O 鍐呭瓨缂栧啓鐨勩€傜粨鏋滃彂鐜?`cfb_imageblit()` 浣跨敤浜?`movl` 鎸囦护锛岃€?`sys_imageblit` 鏄剧劧娌℃湁銆傝繖浼间箮鏄?gcc 浼樺寲鍣ㄧ殑涓€涓棶棰樸€侱RM 鐨勬牸寮忚浆鎹?helper 涔熷彲鑳藉瓨鍦ㄧ被浼奸棶棰樸€?
瀵?fbdev 鐨?`sys_()` helper 涓?DRM 鐨勬牸寮忚浆鎹?helper 杩涜鍩哄噯娴嬭瘯骞朵紭鍖栥€傚湪鍙互杩涗竴姝ヤ紭鍖栫殑鍦版柟锛屼篃璁稿彲浠ュ疄鐜颁竴绉嶄笉鍚岀殑绠楁硶銆傚浜庡井浼樺寲锛屾樉寮忎娇鐢?`movl`/`movq` 鎸囦护銆傝繖鍙兘闇€瑕佹灦鏋勭壒瀹氱殑 helper锛堜緥濡?`storel()`銆乣storeq()`锛夈€?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛欼ntermediate

### drm_framebuffer_funcs 涓?drm_mode_config_funcs.fb_create 娓呯悊


杩樻湁鏇村椹卞姩鍙互鍒囨崲鍒?`drm_gem_framebuffer` helper銆傚瓨鍦ㄥ悇绉嶉樆纰嶅洜绱狅細

- 闇€瑕佸厛鍒囨崲鍒颁娇鐢ㄩ€氱敤鑴忚窡韪唬鐮侊紝鍗?`drm_atomic_helper_dirtyfb`锛堜緥濡?qxl锛夈€?
- 闇€瑕佸垏鎹㈠埌 `drm_fbdev_generic_setup()`锛屽惁鍒欏ぇ閲忚嚜瀹氫箟鐨?fb 璁剧疆浠ｇ爜鏃犳硶鍒犻櫎銆?
- 闇€瑕佸垏鎹㈠埌 `drm_gem_fb_create()`锛屽洜涓虹幇鍦?`drm_gem_fb_create()` 浼氫负 atomic 椹卞姩妫€鏌ユ湁鏁堢殑鏍煎紡銆?
- 璁稿椹卞姩瀵?`drm_framebuffer` 鍋氫簡瀛愮被鍖栵紝鎴戜滑闇€瑕佷竴涓笌涔嬪吋瀹圭殑宓屽叆锛坋mbedding锛夌増鏈殑鍚勭被 `drm_gem_fb_create` 鍑芥暟銆備篃璁告牴鎹渶瑕佸懡鍚嶄负 `drm_gem_fb_create`/`_with_dirty`/`_with_funcs`銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

### 閫氱敤鐨?fbdev defio 鏀寔


fbdev 鏍稿績涓殑 defio 鏀寔浠ｇ爜鏈変竴浜涢潪甯稿叿浣撶殑瑕佹眰锛岃繖鎰忓懗鐫€椹卞姩闇€瑕佷竴涓壒娈婄殑 framebuffer 鐢ㄤ簬 fbdev銆備富瑕侀棶棰樺湪浜庡畠浣跨敤浜?`struct page` 鑷韩涓殑涓€浜涘瓧娈碉紝杩欎細鐮村潖 shmem gem 瀵硅薄锛堜互鍙婂叾浠栦笢瑗匡級銆備负浜嗘敮鎸?defio锛屽彈褰卞搷鐨勯┍鍔ㄩ渶瑕佷娇鐢ㄤ竴涓?shadow buffer锛岃繖鍙兘浼氬鍔?CPU 涓庡唴瀛樺紑閿€銆?
鍙兘鐨勮В鍐虫柟妗堟槸鍦?DRM 鐨?fbdev 妯℃嫙涓紪鍐欐垜浠嚜宸辩殑 defio mmap 浠ｇ爜銆傚畠闇€瑕佸畬鍏ㄥ寘瑁圭幇鏈夌殑 mmap 鎿嶄綔锛屽湪瀹屾垚浜嗗啓淇濇姢/mkwrite 鐨勬妧宸т箣鍚庡啀杞彂涓€鍒囷細

- 鍦?`drm_fbdev_fb_mmap` helper 涓紝濡傛灉鎴戜滑闇€瑕?defio锛屽垯淇敼

```
      vma->vm_page_prot = pgprot_wrprotect(vma->vm_page_prot);

```

- 鐢ㄤ笌鏍稿績 fbdev defio 浠ｇ爜绫讳技鐨勫疄鐜版潵璁剧疆 mkwrite 涓?fsync 鍥炶皟銆傝繖浜涢兘搴斿綋宸ヤ綔鍦ㄦ櫘閫氱殑 pte 涓婏紝瀹冧滑瀹為檯涓婂苟涓嶉渶瑕?`struct page`銆倁ff. 杩欎簺閮藉簲褰撳伐浣滃湪鏅€氱殑 pte 涓婏紝瀹冧滑瀹為檯涓婂苟涓嶉渶瑕?`struct page`銆?
- 鍦ㄤ竴涓嫭绔嬬殑缁撴瀯浣擄紙姣忎釜椤典竴涓?bit 鐨勪綅鍩熷簲璇ュ彲琛岋級涓窡韪剰椤碉紝浠ラ伩鍏嶇牬鍧?`struct page`銆?
鏈€濂戒篃涓鸿繖涓噯澶囦竴浜?igt 娴嬭瘯鐢ㄤ緥銆?
鑱旂郴浜猴細Simona Vetter锛孨oralf Tronnes

闅惧害锛欰dvanced

### connector 娉ㄥ唽/娉ㄩ攢淇


- 瀵逛簬澶у鏁?connector锛岀洿鎺ヤ粠椹卞姩浠ｇ爜涓皟鐢?`drm_connector_register`/`unregister` 鏄┖鎿嶄綔锛屽洜涓?`drm_dev_register`/`unregister` 宸茬粡澶勭悊浜嗚繖浠朵簨銆傛垜浠彲浠ョЩ闄ゆ墍鏈夎繖浜涜皟鐢ㄣ€?
- 瀵逛簬 DP 椹卞姩锛屾儏鍐佃娣蜂贡涓€浜涳紝鍥犱负鍦ㄨ皟鐢?`drm_dp_aux_register` 鏃舵垜浠渶瑕?connector 宸茬粡娉ㄥ唽銆傚彲浠ラ€氳繃鏀逛负璋冪敤 `drm_dp_aux_init`锛屽苟灏嗗疄闄呯殑娉ㄥ唽鍔ㄤ綔绉诲叆 `late_register` 鍥炶皟锛堝 kerneldoc 涓墍寤鸿锛夋潵淇銆?
闅惧害锛欼ntermediate

### 绉婚櫎 load/unload 鍥炶皟


`struct &drm_driver` 涓殑 load/unload 鍥炶皟寰堝ぇ绋嬪害涓婃槸涓棿灞傦紙midlayer锛夛紝鑰屼笖鐢变簬鍘嗗彶鍘熷洜锛屽畠浠湪璁剧疆 `&drm_driver` 缁撴瀯浣撲笌璋冪敤 `drm_dev_register()` 涔嬮棿鐨勯『搴忔槸閿欒鐨勶紙鑰屼笖鎴戜滑鏃犳硶淇杩欎竴鐐癸級銆?
- 閲嶆柊鏀归€犻┍鍔紝浣垮叾涓嶅啀浣跨敤 load/unload 鍥炶皟锛岃€屾槸灏?load/unload 娴佺▼鐩存帴缂栫爜鍒伴┍鍔ㄧ殑 probe 鍑芥暟涓€?
- 涓€鏃︽墍鏈夐┍鍔ㄩ兘杞崲瀹屾垚锛岀Щ闄?load/unload 鍥炶皟銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

### 鐢?drm_display_info.is_hdmi 鏇挎崲 drm_detect_hdmi_monitor()


涓€鏃?EDID 琚В鏋愶紝鏄剧ず鍣ㄧ殑 HDMI 鏀寔淇℃伅灏卞彲浠ラ€氳繃 `drm_display_info.is_hdmi` 鑾峰彇銆傝澶氶┍鍔ㄤ粛鐒惰皟鐢?`drm_detect_hdmi_monitor()` 鏉ヨ幏鍙栫浉鍚岀殑淇℃伅锛屾晥鐜囪緝浣庛€?
閫愪釜瀹℃煡璋冪敤 `drm_detect_hdmi_monitor()` 鐨勫悇涓┍鍔紝濡傛灉閫傜敤鍒欏垏鎹㈠埌 `drm_display_info.is_hdmi`銆?
鑱旂郴浜猴細Laurent Pinchart锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欼ntermediate

### 鏁村悎鍚勯┍鍔ㄨ嚜瀹氫箟鐨?modeset 灞炴€?

鍦?atomic modeset 鍑虹幇涔嬪墠锛岃澶氶┍鍔ㄩ兘鍒涘缓浜嗚嚜宸辩殑灞炴€с€傞櫎姝や箣澶栵紝atomic 杩樺甫鏉ヤ簡涓€涓姹傦細涓嶅簲浣跨敤鑷畾涔夈€侀┍鍔ㄧ壒瀹氱殑灞炴€с€?
瀵逛簬杩欎釜浠诲姟锛屾垜浠殑鐩爣鏄紩鍏ユ牳蹇?helper锛屾垨鑰呭湪鍙敤鏃跺鐢ㄥ凡鏈夌殑 helper锛?
涓€浠藉揩閫熴€佹湭缁忕‘璁ょ殑渚嬪瓙鍒楄〃銆?
寮曞叆鏍稿績 helper锛?- audio锛坅mdgpu銆乮ntel銆乬ma500銆乺adeon锛?- brightness銆乧ontrast 绛夛紙armada銆乶ouveau锛夆€斺€斾粎 overlay锛堬紵锛?- broadcast rgb锛坓ma500銆乮ntel锛?- colorkey锛坅rmada銆乶ouveau銆乺car锛夆€斺€斾粎 overlay锛堬紵锛?- dither锛坅mdgpu銆乶ouveau銆乺adeon锛夆€斺€斿悇椹卞姩涔嬮棿涓嶅悓
- underscan 绯诲垪锛坅mdgpu銆乺adeon銆乶ouveau锛?
宸插湪鏍稿績涓細
- colorspace锛坰ti锛?- tv format 鍚嶇О銆佸寮猴紙gma500銆乮ntel锛?- tv overscan銆乵argins 绛夛紙gma500銆乮ntel锛?- zorder锛坥mapdrm锛夆€斺€斾笌 zpos 鐩稿悓锛堬紵锛?
鑱旂郴浜猴細Emil Velikov锛岀浉鍏抽┍鍔ㄧ淮鎶よ€?
闅惧害锛欼ntermediate

### 鍦ㄦ暣涓唬鐮佸簱涓娇鐢?struct iosys_map


鎸囧悜鍏变韩璁惧鍐呭瓨鐨勬寚閽堝瓨鍌ㄥ湪 `struct iosys_map` 涓€傛瘡涓疄渚嬮兘鐭ラ亾瀹冩寚鍚戠殑鏄郴缁熷唴瀛樿繕鏄?I/O 鍐呭瓨銆傚ぇ澶氭暟 DRM 鑼冨洿鍐呯殑鎺ュ彛宸茬粡杞崲涓轰娇鐢?`struct iosys_map`锛屼絾瀹炵幇閫氬父浠嶄娇鐢ㄨ８鎸囬拡銆?
浠诲姟鏄紝鍦ㄦ湁鎰忎箟鐨勫湴鏂逛娇鐢?`struct iosys_map`銆?
- 鍐呭瓨绠＄悊鍣ㄥ簲瀵?dma-buf 瀵煎叆鐨勭紦鍐插尯浣跨敤 `struct iosys_map`銆?- TTM 鍙兘鍦ㄥ唴閮ㄤ娇鐢?`struct iosys_map` 浼氬彈鐩娿€?- Framebuffer 澶嶅埗涓?blitting helper 搴斿熀浜?`struct iosys_map` 鎿嶄綔銆?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>锛孋hristian K枚nig锛孲imona Vetter

闅惧害锛欼ntermediate

### 瀹℃煡鎵€鏈夐┍鍔ㄦ槸鍚︽纭缃?struct drm_mode_config.{max_width,max_height}


`struct drm_mode_config.{max_width,max_height}` 涓殑鍊兼弿杩颁簡鎵€鏀寔鐨勬渶澶?framebuffer 灏哄銆傚畠鏄櫄鎷熷睆骞曞ぇ灏忥紝浣嗚澶氶┍鍔ㄦ妸瀹冨綋浣滅墿鐞嗗垎杈ㄧ巼鐨勯檺鍒躲€?
鏈€澶у搴﹀彇鍐充簬纭欢鐨勬渶澶ф壂鎻忚 pitch銆傛渶澶ч珮搴﹀彇鍐充簬鍙鍧€鏄惧瓨鐨勫閲忋€傚鏌ユ墍鏈夐┍鍔紝灏嗚繖涓や釜瀛楁鍒濆鍖栦负姝ｇ‘鐨勫€笺€?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛欼ntermediate

### 鍦ㄦ墍鏈?fbdev 椹卞姩涓敵璇峰唴瀛樺尯鍩?

鑰佹棫/鍙よ€佺殑 fbdev 椹卞姩娌℃湁姝ｇ‘鍦扮敵璇峰畠浠殑鍐呭瓨銆傞亶鍘嗚繖浜涢┍鍔紝娣诲姞浠ｇ爜浠ョ敵璇烽┍鍔ㄦ墍浣跨敤鐨勫唴瀛樺尯鍩熴€傝繖闇€瑕佹坊鍔犲 `request_mem_region()`銆乣pci_request_region()` 鎴栫被浼煎嚱鏁扮殑璋冪敤銆傚敖鍙兘浣跨敤甯︽墭绠＄殑锛坢anaged锛夋竻鐞?helper銆傚瓨鍦ㄩ棶棰樼殑鍖哄煙鍖呮嫭鍍?VGA 杩欐牱鎷ユ湁鐙崰鑼冨洿鐨勭‖浠躲€俈GA16fb 娌℃湁鍍忛鏈熺殑閭ｆ牱鐢宠璇ヨ寖鍥淬€傞┍鍔ㄥ湪鍋氳繖浠朵簨涓婄浉褰撶碂绯曪紝鑰屼笖 DRM 涓?fbdev 椹卞姩涔嬮棿鏇剧粡鏈夎繃鍐茬獊銆備笉杩囷紝杩欐牱鍋氭槸姝ｇ‘鐨勩€?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛歋tarter

### 绉婚櫎椹卞姩瀵?FB_DEVICE 鐨勪緷璧?

璁稿 fbdev 椹卞姩閫氳繃 sysfs 鎻愪緵灞炴€э紝鍥犳渚濊禆浜庨€変腑 `CONFIG_FB_DEVICE`銆傚鏌ユ瘡涓┍鍔紝骞跺皾璇曚娇浠讳綍瀵?`CONFIG_FB_DEVICE` 鐨勪緷璧栧彉涓哄彲閫夈€傝嚦灏戯紝椹卞姩涓搴旂殑浠ｇ爜鍙互閫氳繃 `ifdef CONFIG_FB_DEVICE` 杩涜鏉′欢缂栬瘧銆傚苟闈炴墍鏈夐┍鍔ㄩ兘鑳藉幓鎺?`CONFIG_FB_DEVICE`銆?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛歋tarter

### 绉婚櫎 panel-simple 涓?panel-edp 鍦?remove/shutdown 涓殑 disable/unprepare


鏍规嵁鎻愪氦 d2aacaf07395锛堚€渄rm/panel: Check for already prepared/enabled in drm_panel鈥濓級锛屾垜浠湪 `drm_panel` 鏍稿績涓鍔犱簡涓€涓鏌ワ紝浠ョ‘淇濅笉浼氭湁浜洪噸澶嶈皟鐢?prepare/enable/disable/unprepare銆傛渶缁堣繖鍙兘搴旇鍙樻垚涓€涓?`WARN_ON()`锛屾垨鑰呬互鏌愮鏂瑰紡璁╂彁绀烘洿鏄庢樉銆?
鐩墠锛屾垜浠璁″湪浣跨敤 panel-simple 涓?panel-edp 鏃朵粛鍙兘鍦?`drm_panel` 鏍稿績涓亣鍒拌繖浜涜鍛娿€傜敱浜庤繖浜?panel 椹卞姩涓庤澶氫笉鍚岀殑 DRM modeset 椹卞姩涓€璧蜂娇鐢紝瀹冧滑浠嶄細棰濆鍦?shutdown 鏃惰嚜琛?disable/unprepare 璇?panel銆傚叿浣撴潵璇达紝濡傛灉 panel 椹卞姩鍦?DRM modeset 椹卞姩 *涔嬪墠* 琚?`shutdown()`锛岃€?DRM modeset 椹卞姩鍦ㄥ叾鑷韩鐨?`shutdown()` 鍥炶皟涓纭湴璋冪敤浜?`drm_atomic_helper_shutdown()`锛屾垜浠粛鍙兘閬囧埌杩欎簺璀﹀憡銆傚湪杩欑鎯呭喌涓嬶紝鍙互閫氳繃浣跨敤绫讳技 device link 鐨勬満鍒舵潵纭繚 panel 鍦?DRM modeset 椹卞姩涔嬪悗琚?`shutdown()`锛屼粠鑰岄伩鍏嶈鍛娿€?
涓€鏃﹀凡鐭ユ墍鏈?DRM modeset 椹卞姩閮借兘姝ｇ‘ shutdown锛屽氨搴旇绉婚櫎 panel-simple 涓?panel-edp 鍦?remove/shutdown 涓 disable/unprepare 鐨勯澶栬皟鐢紝骞跺皢鏈?TODO 椤规爣璁颁负瀹屾垚銆?
鑱旂郴浜猴細Douglas Anderson <dianders@chromium.org>

闅惧害锛欼ntermediate

### 鎽嗚劚宸插純鐢ㄧ殑 MIPI DSI 鍑芥暟


`drm_mipi_dsi.c` 涓畾涔変簡璁稿宸茶寮冪敤鐨勫嚱鏁般€傛瘡涓寮冪敤鐨勫嚱鏁伴兘鏄负浜嗚浣嶄簬鍏?`multi` 鍙樹綋锛堜緥濡?`mipi_dsi_generic_write()` 涓?`mipi_dsi_generic_write_multi()`锛夈€傚嚱鏁扮殑 `multi` 鍙樹綋鍖呭惈浜嗘敼杩涚殑閿欒澶勭悊閫昏緫锛屽苟浣胯繛缁繘琛屽娆¤皟鐢ㄦ洿鍔犳柟渚匡紝灏卞儚澶у鏁?MIPI 椹卞姩鎵€鍋氱殑閭ｆ牱銆?
椹卞姩搴斿綋鏇存柊涓轰娇鐢ㄦ湭寮冪敤鐨勫嚱鏁般€備竴鏃︽墍鏈夊宸插純鐢?MIPI DSI 鍑芥暟鐨勪娇鐢ㄩ兘琚Щ闄わ紝瀹冧滑鐨勫畾涔夊氨鍙互浠?`drm_mipi_dsi.c` 涓垹闄ゃ€?
鑱旂郴浜猴細Douglas Anderson <dianders@chromium.org>

闅惧害锛歋tarter

### 绉婚櫎 devm_drm_put_bridge()


鐢变簬 panel bridge 澶勭悊 `drm_bridge` 瀵硅薄鐢熷懡鍛ㄦ湡鐨勬柟寮忥紝鍦ㄧЩ闄?`panel_bridge` 鏃跺繀椤荤壒鍒皬蹇冨湴閲婃斁 `drm_bridge` 瀵硅薄銆傜洰鍓嶈繖閫氳繃 `devm_drm_put_bridge()` 鏉ョ鐞嗭紝浣嗛偅鏄竴涓笉瀹夊叏銆佷复鏃剁殑鏉冨疁涔嬭銆傝淇杩欎釜闂锛岄渶瑕侀噸鏂拌璁?DRM panel 鐨勭敓鍛藉懆鏈熴€傞噸鏂拌璁″畬鎴愪箣鍚庯紝绉婚櫎 `devm_drm_put_bridge()` 浠ュ強 `drm_panel_bridge_remove()` 涓殑 TODO銆?
鑱旂郴浜猴細Maxime Ripard <mripard@kernel.org>锛?         Luca Ceresoli <luca.ceresoli@bootlin.com>

闅惧害锛欼ntermediate

### 灏?of_drm_find_bridge() 鐨勪娇鐢ㄨ€呰浆鎹负 of_drm_find_and_get_bridge()


鑾峰彇涓€涓?`struct drm_bridge` 鎸囬拡闇€瑕佸彇寰椾竴涓紩鐢紝骞跺湪閲婃斁璇ユ寚閽堝悗褰掕繕瀹冦€傚ぇ澶氭暟杩斿洖 `struct drm_bridge` 鎸囬拡鐨勫嚱鏁板凡缁忚皟鐢?`drm_bridge_get()` 鏉ュ鍔犲紩鐢ㄨ鏁帮紝骞朵笖瀹冧滑鐨勪娇鐢ㄨ€呭凡缁忔洿鏂颁负鍦ㄩ€傚綋鐨勬椂鍊欒皟鐢?`drm_bridge_put()`銆俙of_drm_find_bridge()` 涓嶄細鍙栧緱寮曠敤锛屽畠宸茶寮冪敤锛岀敱浼氬彇寰楀紩鐢ㄧ殑 `of_drm_find_and_get_bridge()` 鍙栦唬锛屼絾涓€浜涗娇鐢ㄨ€呬粛闇€瑕佽杞崲銆?
鑱旂郴浜猴細Maxime Ripard <mripard@kernel.org>锛?         Luca Ceresoli <luca.ceresoli@bootlin.com>

闅惧害锛欼ntermediate

## 鏍稿績閲嶆瀯


### 璁?panic 澶勭悊姝ｅ父宸ヤ綔


杩欐槸涓€椤瑰唴瀹归潪甯稿鏍风殑浠诲姟锛屽寘鍚澶氶浂纰庣殑灏忓伐浣滐細

- panic 璺緞鐩墠鏃犳硶琚祴璇曪紝瀵艰嚧瀹冧笉鏂嚭闂銆傝繖閲岀殑涓昏闂鏄?panic 鍙互浠?hardirq 涓婁笅鏂囦腑瑙﹀彂锛屽洜姝ゆ墍鏈変笌 panic 鐩稿叧鐨勫洖璋冮兘鍙兘鍦?hardirq 涓婁笅鏂囦腑杩愯銆傚鏋滆嚦灏戣兘閫氳繃渚嬪 drm debugfs 鏂囦欢瑙﹀彂璋冪敤鏉ユ祴璇?fbdev helper 浠ｇ爜涓庨┍鍔ㄤ唬鐮侊紝閭ｄ細寰堟銆俬ardirq 涓婁笅鏂囧彲浠ラ€氳繃鍚戞湰鍦板鐞嗗櫒鍙戦€佷竴涓?IPI 鏉ュ疄鐜般€?
- 鍚勭 panic handler 涔嬮棿瀛樺湪宸ㄥぇ鐨勬贩涔便€侱RM fbdev 妯℃嫙 helper 鏇剧粡鏈夎嚜宸辩殑锛堟棭宸茬Щ闄わ級锛屼絾闄ゆ涔嬪 fbcon 浠ｇ爜鏈韩涔熸湁涓€涓€傛垜浠渶瑕佺‘淇濆畠浠笉鍐嶄簰鐩镐簤鎶€傜洰鍓嶇殑鏉冨疁涔嬭鏄湪杩涘叆 DRM fbdev 妯℃嫙 helper 鐨勫悇涓叆鍙ｇ偣妫€鏌?`oops_in_progress`銆傝繖閲屾洿骞插噣鐨勫仛娉曟槸灏?fbcon 鍒囨崲鍒?`threaded printk support <https://lwn.net/Articles/800946/>`_銆?
- `drm_can_sleep()` 鏄竴鍥贡楹汇€傚畠鎺╃洊浜嗘甯告搷浣滀腑鐨勭湡姝?bug锛屽苟涓斿浜?panic 璺緞鏉ヨ涔熶笉鏄竴涓畬鏁寸殑瑙ｅ喅鏂规銆傛垜浠渶瑕佺‘淇濆畠浠呭湪鐪熸鍙戠敓 panic 鏃舵墠杩斿洖 true锛屽苟淇鎵€鏈夊洜姝や骇鐢熺殑闂銆?
- panic handler 缁濅笉鑳戒紤鐪狅紝杩欎篃鎰忓懗鐫€瀹冧笉鑳借皟鐢?`mutex_lock()`銆傚畠涔熶笉鑳芥棤鏉′欢鍦拌幏鍙栦换浣曞叾浠栭攣锛岀敋鑷冲寘鎷嚜鏃嬮攣锛堝洜涓?NMI 涓?hardirq 涔熷彲鑳藉彂鐢?panic锛夈€傛垜浠渶瑕佺‘淇濊涔堜笉璋冪敤杩欐牱鐨勮矾寰勶紝瑕佷箞瀵规墍鏈夊湴鏂归兘浣跨敤 trylock銆傝繖鐪熺殑寰堟鎵嬨€?
- 涓€涓共鍑€鐨勮В鍐虫柟妗堟槸鍦?KMS 涓彁渚涗竴涓畬鍏ㄧ嫭绔嬬殑 panic 杈撳嚭鏀寔锛岀粫杩囧綋鍓嶇殑 fbcon 鏀寔銆傚弬瑙?`[PATCH v2 0/3] drm: Add panic handling <https://lore.kernel.org/dri-devel/20190311174218.51899-1-noralf@tronnes.org/>`_銆?
- 灏嗗疄闄呯殑 oops 浠ュ強涔嬪墠鐨?dmesg 缂栫爜鎴愪簩缁寸爜锛圦R锛夛紝鍙兘鏈夊姪浜庤В鍐斥€滈噸瑕佸唴瀹硅婊氳蛋鈥濊繖涓护浜哄ご鐤肩殑闂銆傚弬瑙?`[RFC][PATCH] Oops messages transfer using QR codes <https://lore.kernel.org/lkml/1446217392-11981-1-git-send-email-alexandru.murtaza@intel.com/>`_ 涓竴浜涘彲浠ュ鐢ㄧ殑绀轰緥浠ｇ爜銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欰dvanced

### 娓呯悊 debugfs 鏀寔


瀹冨瓨鍦ㄤ竴鍫嗛棶棰橈細

- 灏嗛┍鍔ㄨ浆鎹负鏀寔 `drm_debugfs_add_files()` 鍑芥暟锛岃€屼笉鏄?`drm_debugfs_create_files()` 鍑芥暟銆?
- 閫氳繃涓?connector 涓?crtc 涔熸帹琛屽悓鏍风殑 debugfs 棰勬敞鍐屽熀纭€璁炬柦锛屾敼杩?late-register debugfs銆傝繖鏍凤紝椹卞姩灏辨棤闇€鍐嶅皢瀹冧滑鐨勮缃唬鐮佹媶鍒嗕负 init 涓?register 涓ら儴鍒嗐€?
- 鎴戜滑鍙兘甯屾湜鍦ㄦ牳蹇冧腑涓?crtc/connector 浠ュ強涔熻鍏朵粬 KMS 瀵硅薄鐩存帴鎻愪緵涓€浜?debugfs 鏂囦欢鏀寔銆傝繖浜涘璞＄殑 funcs 涓敋鑷虫湁 `drm_print` 鏀寔鏉ヨ浆鍌?KMS 鐘舵€侊紝鎵€浠ヤ竴鍒囬兘宸茬粡灏变綅銆傜劧鍚?`->show()` 鍑芥暟鏄剧劧搴旇缁欎綘涓€涓寚鍚戞纭璞＄殑鎸囬拡銆?
- 鎴戜滑鐜版湁鐨?`drm_driver->debugfs_init` hook 鍙槸鏃х殑銆佷腑闂村眰鍖栫殑 load 娴佺▼鐨勪竴涓仐鐣欑墿銆侱RM debugfs 搴旇鏇村儚 sysfs锛屼綘鍙互鍦ㄤ换浣曟兂瑕佺殑鏃跺€欎负鏌愪釜瀵硅薄鍒涘缓灞炴€?鏂囦欢锛岀敱鏍稿績璐熻矗鍦?register/unregister 鏃跺彂甯?鍙栨秷鍙戝竷鎵€鏈夎繖浜涙枃浠躲€傞┍鍔ㄤ笉搴旇闇€瑕佹搷蹇冭繖浜涙妧鏈粏鑺傦紝淇杩欎釜闂锛堣繛鍚?`drm_minor->drm_device` 鐨勮縼绉伙級灏嗕娇鎴戜滑鑳藉绉婚櫎 `debugfs_init`銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

### 瀵硅薄鐢熷懡鍛ㄦ湡淇


杩欓噷鏈変袱涓浉鍏崇殑闂锛?
- 娓呯悊鍚勭鍚勬牱鐨?`->destroy` 鍥炶皟锛岃繖浜涘洖璋冮€氬父閮芥槸鐩稿悓鐨勪竴娈电畝鍗曚唬鐮併€?
- 澶ч噺椹卞姩閿欒鍦颁娇鐢?`devm_kzalloc` 鍒嗛厤 DRM modeset 瀵硅薄锛岃繖浼氬湪椹卞姩鍗歌浇鏃跺鑷?use-after-free 闂銆傚嵆渚垮浜庣‖浠堕泦鎴愬湪 SoC 涓婄殑椹卞姩锛岀敱浜?`EPROBE_DEFERRED` 鍥為€€锛岃繖涔熷彲鑳藉甫鏉ヤ弗閲嶉夯鐑︺€?
杩欎袱涓棶棰橀兘鍙互閫氳繃鍒囨崲鍒?`drmm_kzalloc()` 浠ュ強鎻愪緵鐨勫悇绉嶄究鍒╁寘瑁呭櫒鏉ヨВ鍐筹紝渚嬪 `drmm_crtc_alloc_with_planes()`銆乣drmm_universal_plane_alloc()` 绛夌瓑銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

### 浠?dma-buf 瀵煎叆涓Щ闄よ嚜鍔ㄩ〉鏄犲皠


鍦ㄥ鍏?dma-buf 鏃讹紝dma-buf 涓?PRIME 妗嗘灦浼氳嚜鍔ㄥ皢瀵煎叆鐨勯〉鏄犲皠鍒?importer 鐨?DMA 鍖哄煙銆俙drm_gem_prime_fd_to_handle()` 涓?`drm_gem_prime_handle_to_fd()` 瑕佹眰 importer 璋冪敤 `dma_buf_attach()`锛屽嵆渚垮畠浠粠涓嶈繘琛岀湡姝ｇ殑璁惧 DMA锛岃€屽彧閫氳繃 `dma_buf_vmap()` 杩涜 CPU 璁块棶銆傝繖瀵逛笉鏀寔 DMA 鎿嶄綔鐨?USB 璁惧鏉ヨ鏄釜闂銆?
涓轰簡淇杩欎釜闂锛屽簲褰撲粠 buffer 鍏变韩浠ｇ爜涓Щ闄よ嚜鍔ㄩ〉鏄犲皠銆備慨澶嶈捣鏉ョ◢寰鏉備竴浜涳紝鍥犱负 import/export 缂撳瓨杩樹笌 `&drm_gem_object.import_attach` 缁戝畾鍦ㄤ竴璧枫€備笌姝ゅ悓鏃讹紝鎴戜滑閫氳繃鍦ㄦ敮鎸?DMA 鐨勬儏鍐典笅鎵惧嚭 USB 涓绘満鎺у埗鍣ㄨ澶囷紝鏉ヤ负 USB 璁惧鎺╃洊杩欎釜闂銆傚惁鍒欏鍏ヤ粛鐒跺彲鑳戒笉蹇呰鍦板け璐ャ€?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>锛孲imona Vetter

闅惧害锛欰dvanced

### 瀹炵幇鏂扮殑 DUMB_CREATE2 ioctl


褰撳墠鐨?`DUMB_CREATE` ioctl 瀹氫箟寰椾笉澶熷ソ銆傚畠涓嶆帴鍙楀儚绱犱笌 framebuffer 鏍煎紡锛屽彧鎺ュ彈涓€涓涔夋ā绯婄殑棰滆壊妯″紡銆傚亣璁炬槸绾挎€?framebuffer锛岄鑹叉ā寮忕粰鍑轰簡鎵€鏀寔鍍忕礌鏍煎紡鐨勬蹇点€備絾鐢ㄦ埛绌洪棿瀹為檯涓婁笉寰椾笉鍘荤寽娴嬫纭殑鍊笺€傚畠鐪熸鍙潬鐨勫彧鏈?XRGB8888 鐨?framebuffer銆傜敤鎴风┖闂村凡缁忓紑濮嬮€氳繃璁＄畻浠绘剰鏍煎紡鐨勭紦鍐插尯澶у皬骞朵互 XRGB8888 鍍忕礌涓哄崟浣嶈绠楀叾澶у皬锛屾潵缁曡繃杩欎簺闄愬埗銆?
涓€涓彲鑳界殑瑙ｅ喅鏂规鏄柊鐨?ioctl `DUMB_CREATE2`銆傚畠搴旇鎺ュ彈涓€涓?DRM 鏍煎紡涓庝竴涓牸寮忎慨楗扮锛坒ormat modifier锛夛紝浠ユ秷闄ら鑹叉ā寮忕殑姝т箟銆傜敱浜?framebuffer 鍙互鏄骞抽潰鐨勶紝鏂?ioctl 蹇呴』杩斿洖姣忎釜鐙珛棰滆壊骞抽潰鐨勭紦鍐插尯澶у皬銆乸itch 涓?GEM handle銆?
绗竴姝ワ紝鏂?ioctl 鍙互闄愬畾涓虹幇鏈?`DUMB_CREATE` 鐨勫綋鍓嶅姛鑳姐€傜劧鍚庡悇涓┍鍔ㄥ彲浠ユ墿灞曚互鏀寔澶氬钩闈㈡牸寮忋€俁ockchip 鍙兘闇€瑕佽繖涓紝浼氭槸涓€涓ソ鐨勫€欓€夎€呫€?
鍚戠敤鎴风┖闂存彁渚涘叧浜庢綔鍦ㄧ紦鍐插尯锛堝鏋滃垎閰嶇殑璇濓級澶у皬鐨勪俊鎭篃鍙兘鏈夊府鍔┿€傜敤鎴风┖闂翠細鎻愪緵鍑犱綍褰㈢姸涓庢牸寮忥紱鍐呮牳浼氳繑鍥炴渶灏忕殑鍒嗛厤澶у皬涓庢壂鎻忚 pitch銆備汉浠湁鍏磋叮浠庡彟涓€涓澶囧垎閰嶈鍐呭瓨骞舵彁渚涚粰 DRM 椹卞姩锛堜緥濡傞€氳繃 dma-buf锛夈€?
鍙︿竴涓璇锋眰鐨勭壒鎬ф槸鑳藉鎸夊ぇ灏忥紙鑰屼笉鎸囧畾鏍煎紡锛夊垎閰嶇紦鍐插尯銆傚姞閫熷櫒锛圓ccelator锛夊湪瀹冧滑鐨勭紦鍐插尯鍒嗛厤涓娇鐢ㄤ簡杩欎竴鐐癸紝骞朵笖寰堝彲鑳藉彲浠ユ硾鍖栥€?
闄や簡鍐呮牳瀹炵幇涔嬪锛岃繕蹇呴』鏈夌敤鎴风┖闂村鏂?ioctl 鐨勬敮鎸併€侻esa 涓湁涓€浜涗唬鐮佷篃璁歌兘澶熶娇鐢ㄨ繖涓柊璋冪敤銆?
鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛欰dvanced

## 鏇村ソ鐨勬祴璇?

### 浣跨敤鍐呮牳鍗曞厓娴嬭瘯锛圞Unit锛夋鏋舵坊鍔犲崟鍏冩祴璇?

`KUnit <https://www.kernel.org/doc/html/latest/dev-tools/kunit/index.html>`_ 涓?Linux 鍐呮牳涓殑鍗曞厓娴嬭瘯鎻愪緵浜嗕竴涓€氱敤妗嗘灦銆傛嫢鏈変竴涓祴璇曞浠跺彲浠ユ洿鏃╁湴鍙戠幇鍥炲綊銆?
绗竴鎵瑰崟鍏冩祴璇曠殑涓€涓ソ鍊欓€夎€呮槸 `drm_format_helper.c` 涓殑鏍煎紡杞崲 helper銆?
鑱旂郴浜猴細Javier Martinez Canillas <javierm@redhat.com>

闅惧害锛欼ntermediate

### 娓呯悊骞舵枃妗ｅ寲浠ュ墠鐨?selftests 濂椾欢


涓€浜?KUnit 娴嬭瘯濂椾欢锛坉rm_buddy銆乨rm_cmdline_parser銆乨rm_damage_helper銆乨rm_format銆乨rm_framebuffer銆乨rm_dp_mst_helper銆乨rm_mm銆乨rm_plane_helper 涓?drm_rect锛夋槸浠ュ墠鐨?selftests 濂椾欢锛屽湪 KUnit 鏈€鍒濆紩鍏ユ椂琚浆鎹簡杩囨潵銆?
杩欎簺濂椾欢褰撴椂鍑犱箮娌℃湁鏂囨。锛岃€屼笖鐩爣涓庡崟鍏冩祴璇曟墍鑳藉仛鐨勬湁鎵€涓嶅悓銆傚皾璇曡瘑鍒繖浜涘浠朵腑姣忎釜娴嬭瘯瀹為檯娴嬭瘯鐨勬槸浠€涔堬紝瀵逛簬鍗曞厓娴嬭瘯鏄惁鏈夋剰涔夛紝濡傛灉涓嶅悎鐞嗗垯绉婚櫎瀹冿紝鍚堢悊鍒欎负鍏剁紪鍐欐枃妗ｏ紝灏嗕細澶ф湁甯姪銆?
鑱旂郴浜猴細Maxime Ripard <mripard@kernel.org>

闅惧害锛欼ntermediate

### 涓?DRM 鍚敤 trinity


骞朵慨澶嶇敱姝や骇鐢熺殑闂銆傚簲璇ヤ細鐪熺殑寰堟湁瓒ｂ€︹€?
闅惧害锛欰dvanced

### 璁?i-g-t 涓殑 KMS 娴嬭瘯鎴愪负閫氱敤娴嬭瘯


i915 椹卞姩鍥㈤槦缁存姢鐫€涓€濂楀箍娉涚殑 i915 DRM 椹卞姩娴嬭瘯濂椾欢锛屽叾涓寘鍚ぇ閲忛拡瀵?modesetting API 涓竟鐣屾儏鍐电殑娴嬭瘯鐢ㄤ緥銆傚鏋滈偅浜涙祴璇曪紙鑷冲皯鏄偅浜涗笉渚濊禆 Intel 鐗瑰畾 GEM 鐗规€х殑娴嬭瘯锛夎兘澶熻繍琛屽湪浠讳綍 KMS 椹卞姩涓婏紝閭ｅ氨澶浜嗐€?
鍦ㄩ潪 i915 涓婅繍琛?i-g-t 娴嬭瘯鐨勫熀纭€宸ヤ綔宸茬粡瀹屾垚锛岀幇鍦ㄧ己鐨勬槸灏嗗畠浠ぇ瑙勬ā杞崲杩囨潵銆傚浜?modeset 娴嬭瘯锛屾垜浠鍏堣繕闇€瑕佷竴鐐瑰熀纭€璁炬柦鏉ヤ娇鐢?dumb buffer 浣滀负 untiled buffer锛屼互渚胯兘澶熻繍琛屾墍鏈夐潪 i915 鐗瑰畾鐨?modeset 娴嬭瘯銆?
闅惧害锛欰dvanced

### 鎵╁睍铏氭嫙娴嬭瘯椹卞姩锛圴KMS锛?

鍙傝 VKMS <vkms> 鐨勬枃妗ｄ簡瑙ｆ洿澶氱粏鑺傘€傝繖鏄竴涓悊鎯崇殑瀹炰範浠诲姟锛屽洜涓哄畠鍙渶瑕佷竴鍙拌櫄鎷熸満锛屽苟涓斿彲浠ユ牴鎹彲鐢ㄦ椂闂磋皟鏁磋妯°€?
闅惧害锛歋ee details

### Backlight 閲嶆瀯


Backlight 椹卞姩鏈変笁閲?enable/disable 鐘舵€侊紝杩欐湁鐐硅繃搴︿簡銆備慨澶嶈鍒掞細

1. 鍦ㄦ墍鏈夊湴鏂规帹琛?`backlight_enable()` 涓?`backlight_disable()` helper銆傝繖宸茬粡寮€濮嬩簡銆?2. 鎬讳綋涓婏紝鍙湅涓婅堪 helper 璁剧疆鐨勪笁涓姸鎬佷綅涓殑涓€涓€?3. 绉婚櫎鍙﹀涓や釜鐘舵€佷綅銆?
鑱旂郴浜猴細Simona Vetter

闅惧害锛欼ntermediate

## 椹卞姩鐗瑰畾


### AMD DC 鏄剧ず椹卞姩


AMD DC 鏄?AMD 璁惧锛堜粠 Vega 寮€濮嬶級鐨勬樉绀洪┍鍔ㄣ€傚湪娓呯悊瀹冩柟闈㈠凡缁忓彇寰椾簡涓€浜涜繘灞曪紝浣嗕粛鏈夊ぇ閲忓伐浣滆鍋氥€?
鍙傝 drivers/gpu/drm/amd/display/TODO 涓殑浠诲姟銆?
鑱旂郴浜猴細Harry Wentland锛孉lex Deucher

## 鍚姩鐢婚潰锛圔ootsplash锛?

鐜板湪宸茬粡鏈夊缂栧啓鍐呴儴 DRM 瀹㈡埛绔殑鏀寔锛岃繖浣垮緱鍙互鎷捐捣閭ｄ釜鍥犱负涓?fbdev 缂栧啓鑰岃鎷掔粷鐨勫惎鍔ㄧ敾闈㈠伐浣溿€?
- [v6,8/8] drm/client: Hack: Add bootsplash example
  https://patchwork.freedesktop.org/patch/306579/

- [RFC PATCH v2 00/13] Kernel based bootsplash
  https://lore.kernel.org/r/20171213194755.3409-1-mstaudt@suse.de

鑱旂郴浜猴細Sam Ravnborg

闅惧害锛欰dvanced

## 鍏锋湁澶氫釜鍐呴儴闈㈡澘鐨勮澶囦笂鐨勪寒搴﹀鐞?

鍦?x86/ACPI 璁惧涓婏紝鍙兘瀛樺湪澶氫釜 backlight 鍥轰欢鎺ュ彛锛氾紙ACPI锛塿ideo銆佸巶鍟嗙壒瀹氱殑浠ュ強鍏朵粬鎺ュ彛銆傝繕鏈?KMS 椹卞姩瀵圭洿鎺?鍘熺敓锛圥WM锛夊瘎瀛樺櫒鐨勭紪绋嬨€?
涓轰簡澶勭悊杩欎釜闂锛岀敤浜?x86/ACPI 鐨?backlight 椹卞姩璋冪敤 `acpi_video_get_backlight_type()`锛屽畠浣跨敤鍚彂寮忥紙鍔?quirk锛夋潵閫夋嫨浣跨敤鍝釜 backlight 鎺ュ彛锛涜€屼笉鍖归厤鎵€杩斿洖绫诲瀷鐨?backlight 椹卞姩灏嗕笉浼氭敞鍐岃嚜宸憋紝浠庤€屽彧鏈変竴涓?backlight 璁惧琚敞鍐岋紙鍦ㄥ崟 GPU 璁剧疆涓嬶紝瑙佷笅鏂囷級銆?
鐩墠杩欏湪寰堝ぇ绋嬪害涓婂亣璁句竴涓郴缁熶笂鍙細鏈変竴涓紙鍐呴儴锛夐潰鏉裤€?
鍦ㄦ湁涓や釜闈㈡澘鐨勭郴缁熶笂锛岃繖鍙兘鏄竴涓棶棰橈紝鍙栧喅浜?`acpi_video_get_backlight_type()` 閫夋嫨浜嗕粈涔堟帴鍙ｏ細

1. native锛氳繖绉嶆儏鍐典笅锛孠MS 椹卞姩搴斿綋鐭ラ亾鍝釜 backlight 璁惧灞炰簬鍝釜杈撳嚭锛屽洜姝や竴鍒囧簲璇ラ兘鑳芥甯稿伐浣溿€?2. video锛氳繖纭疄鏀寔鎺у埗澶氫釜 backlight锛屼絾闇€瑕佸仛浜涘伐浣滄潵鑾峰緱 output 涓?backlight 璁惧涔嬮棿鐨勬槧灏勩€?
涓婇潰鍋囪涓や釜闈㈡澘闇€瑕佺浉鍚岀殑 backlight 鎺ュ彛绫诲瀷銆傚綋涓や釜闈㈡澘闇€瑕佷笉鍚岀被鍨嬫帶鍒舵椂锛屼簨鎯呬細鍑洪棶棰樸€備緥濡傦紝涓€涓潰鏉块渶瑕?ACPI video backlight 鎺у埗锛岃€屽彟涓€涓娇鐢ㄥ師鐢?backlight 鎺у埗銆傜洰鍓嶅湪杩欑鎯呭喌涓嬶紝鏍规嵁 `acpi_video_get_backlight_type()` 鐨勮繑鍥炲€硷紝涓や釜鎵€闇€ backlight 璁惧涓彧鏈変竴涓細琚敞鍐屻€?
濡傛灉杩欑锛堢悊璁轰笂鐨勶級鎯呭喌鐪熺殑鍑虹幇锛岄偅涔堟敮鎸佸畠灏嗛渶瑕佷竴浜涘伐浣溿€傝繖閲屼竴涓彲鑳界殑瑙ｅ喅鏂规鏄悜 `acpi_video_get_backlight_type()` 浼犲叆涓€涓?device 涓?connector-name锛屼互渚垮畠鑳藉鐞嗚繖绉嶆儏鍐点€?
娉ㄦ剰锛屽湪鏌愮鎰忎箟涓婏紝鎴戜滑宸茬粡鏈変簡鐢ㄦ埛鍦ㄧ敤鎴风┖闂寸湅鍒颁袱涓潰鏉跨殑鎯呭喌锛屽嵆鍦ㄥ甫鏈?mux 鐨勫弻 GPU 绗旇鏈缃腑銆傚湪杩欎簺绯荤粺涓婏紝鎴戜滑鍙兘鐪嬪埌涓や釜鍘熺敓 backlight 璁惧锛涙垨鑰呬袱涓師鐢?backlight 璁惧銆?
鐢ㄦ埛绌洪棿宸茬粡鏈変唬鐮侀€氳繃妫€娴嬬浉鍏抽潰鏉挎槸鍚﹀浜庢椿鍔ㄧ姸鎬侊紙鍗?GPU 涓庨潰鏉夸箣闂寸殑 mux 鎸囧悜鍝竴杈癸級鏉ュ鐞嗚繖涓棶棰橈紝鐒跺悗浣跨敤閭ｄ釜 backlight 璁惧銆備笉杩囩敤鎴风┖闂村湪杩欓噷闈炲父鍋囪鍙湁涓€涓潰鏉裤€傚畠鍙湪涓や釜 backlight 璁惧涓€夋嫨涓€涓紝鐒跺悗鍙娇鐢ㄩ偅涓€涓€?
璇锋敞鎰忥紝鎵€鏈夛紙鎴戞墍鐭ラ亾鐨勶級鐢ㄦ埛绌洪棿浠ｇ爜鐩墠閮芥槸纭紪鐮佸亣璁惧崟涓潰鏉跨殑銆?
鍦ㄦ渶杩戠殑鍙樻洿涔嬪墠锛堜笉涓哄崟涓潰鏉匡紙鍦ㄥ崟 GPU 绗旇鏈笂锛夋敞鍐屽涓紙渚嬪 video + native锛塦/sys/class/backlight` 璁惧锛夛紝鐢ㄦ埛绌洪棿浼氱湅鍒板涓?backlight 璁惧锛屽叏閮ㄦ帶鍒跺悓涓€涓?backlight銆?
涓轰簡澶勭悊杩欎釜闂锛岀敤鎴风┖闂存€绘槸閫夊彇 `/sys/class/backlight` 涓嬩竴涓亸濂界殑璁惧锛屽苟蹇界暐鍏朵粬鐨勩€傚洜姝わ紝瑕佹敮鎸佸涓潰鏉夸笂鐨勪寒搴︽帶鍒讹紝鐢ㄦ埛绌洪棿涔熼渶瑕佽鏇存柊銆?
鏈夎鍒掗€氳繃鍚?KMS API 娣诲姞涓€涓柊鐨勨€渄isplay brightness鈥濆睘鎬у埌 `drm_connector` 瀵硅薄锛堢敤浜庨潰鏉匡級鏉ュ厑璁搁€氳繃 KMS API 杩涜浜害鎺у埗銆傝繖瑙ｅ喅浜?`/sys/class/backlight` API 鐨勪竴浜涢棶棰橈紝鍖呮嫭鏃犳硶灏?sysfs backlight 璁惧鏄犲皠鍒扮壒瀹?connector銆備换浣曚负鏀寔澶氫釜闈㈡澘鐨勮澶囨坊鍔犱寒搴︽帶鍒惰€屽仛鐨勭敤鎴风┖闂村彉鏇达紝閮界‘瀹炲簲璇ユ瀯寤哄湪杩欎釜鏂扮殑 KMS 灞炴€т箣涓娿€?
鑱旂郴浜猴細Hans de Goede

闅惧害锛欰dvanced

## 鐢ㄤ簬缂撳啿鍖烘崯鍧忕殑缂撳啿鍖哄勾榫勬垨鍏朵粬鎹熷潖绱Н绠楁硶


杩涜鎸夌紦鍐插尯涓婁紶鐨勯┍鍔ㄩ渶瑕佷竴绉嶇紦鍐插尯鎹熷潖澶勭悊锛堣€屼笉鏄儚鎸?plane 鎴栨寜 CRTC 涓婁紶鐨勯┍鍔ㄩ偅鏍风殑甯ф崯鍧忥級锛屼絾鐩墠娌℃湁鏀寔鏉ヨ幏鍙栫紦鍐插尯骞撮緞鎴栦换浣曞叾浠栨崯鍧忕疮绉畻娉曘€?
鍥犳锛屾崯鍧?helper 鍦ㄩ檮鍔犲埌 plane 鐨?framebuffer 鑷笂娆?page-flip 浠ユ潵鍙戠敓鍙樺寲鏃讹紝鍙細鍥為€€鍒板畬鏁寸殑 plane 鏇存柊銆傞┍鍔ㄥ皢 `&drm_plane_state.ignore_damage_clips` 璁剧疆涓?true锛屼綔涓虹粰 `drm_atomic_helper_damage_iter_init()` 涓?`drm_atomic_helper_damage_iter_next()` helper 鐨勬寚绀猴紝琛ㄧず搴旇蹇界暐鎹熷潖 clips銆?
杩欏簲褰撹鏀硅繘锛屼互浣挎寜缂撳啿鍖轰笂浼犵殑椹卞姩涓婃崯鍧忚窡韪兘姝ｅ父宸ヤ綔銆?
鍏充簬鎹熷潖璺熻釜鐨勬洿澶氫俊鎭互鍙婂涔犺祫鏂欑殑鍙傝€冿紝鍙互鍦?damage_tracking_properties 涓壘鍒般€?
鑱旂郴浜猴細Javier Martinez Canillas <javierm@redhat.com>

闅惧害锛欰dvanced

## 浠?drm_syncobj 鏌ヨ閿欒


`drm_syncobj` 瀹瑰櫒鍙互琚笌椹卞姩鏃犲叧鐨勪唬鐮佺敤鏉ュ彂鍑烘彁浜ゅ畬鎴愮殑淇″彿銆?
浠嶇劧缂哄皯鐨勪竴涓皬鐗规€ф槸涓€涓€氱敤鐨?DRM IOCTL锛岀敤浜庢煡璇簩杩涘埗涓庢椂闂寸嚎 `drm_syncobj` 鐨勯敊璇姸鎬併€?
杩欏簲璇ラ€氳繃瀹炵幇蹇呰鐨勫唴鏍告帴鍙ｅ苟鍦ㄧ敤鎴风┖闂存爤涓坊鍔犲璇ユ帴鍙ｇ殑鏀寔鏉ユ敼杩涖€?
鑱旂郴浜猴細Christian K枚nig

闅惧害锛歋tarter

## DRM GPU 璋冨害鍣紙Scheduler锛?

### 涓?drm_sched_resubmit_jobs() 鎻愪緵涓€涓€氱敤鐨勬浛浠ｈ€?

`drm_sched_resubmit_jobs()` 宸茶寮冪敤銆備富瑕佸師鍥犳槸瀹冧細瀵艰嚧閲嶆柊鍒濆鍖?`dma_fence`銆傝瑙佽鍑芥暟鐨勬枃妗ｃ€傚浜?amdgpu 涓?Xe 鏈夋晥鐨勯噸鏂版彁浜わ紝鏇村ソ鐨勬柟娉曪紙鏄剧劧鏄級鏄紕娓呮鍝釜 job锛堜互鍙婇€氳繃鍏宠仈锛氬摢涓?entity锛夊鑷翠簡鎸傝捣銆傜劧鍚庯紝璇?job 鐨勭紦鍐插尯鏁版嵁锛岃繛鍚屽綋鍓嶅湪鍚屼竴涓‖浠?ring 涓婄殑鎵€鏈夊叾浠?job 鐨勭紦鍐插尯鏁版嵁锛屽繀椤昏缃负鏃犳晥銆備緥濡傚彲浠ラ€氳繃瑕嗙洊瀹冩潵瀹炵幇銆俛mdgpu 鐩墠閫氳繃淇濈暀 job 鐨勫壇鏈潵纭畾鍝簺 job 鍦?ring 涓渶瑕佽瑕嗙洊銆俋e 閫氳繃鐩存帴璁块棶 `drm_sched` 鐨?`pending_list` 鏉ヨ幏鍙栬淇℃伅銆?
浠诲姟锛?
1. 瀹炵幇璋冨害鍣ㄥ姛鑳斤紝浣块┍鍔ㄨ兘澶熻幏鍙栧綋鍓嶅湪纭欢 ring 涓殑鍝簺**鎹熷潖鐨?* job 鐨勪俊鎭€?2. 杩欐牱鐨勫熀纭€璁炬柦闅忓悗閫氬父浼氳鐢ㄥ湪 `drm_sched_backend_ops.timedout_job()` 涓€傚姝ゅ姞浠ユ枃妗ｈ鏄庛€?3. 绉绘涓€涓┍鍔ㄤ綔涓虹涓€涓娇鐢ㄨ€呫€?4. 鍦ㄥ凡寮冪敤鐨?`drm_sched_resubmit_jobs()` 鐨勬枃妗ｄ腑璁板綍杩欎釜鏂扮殑鏇夸唬鏂规銆?
鑱旂郴浜猴細Christian K枚nig <christian.koenig@amd.com>
         Philipp Stanner <phasta@kernel.org>

闅惧害锛欰dvanced

### 涓?runqueue 娣诲姞鍔犻攣


鍦?`include/drm/gpu_scheduler.h` 涓湁涓€鏉?Sima 鐣欎笅鐨勬棫 `FIXME`銆傚畠璇︾粏璇存槑 `struct drm_sched_rq` 鍦ㄨ澶氬湴鏂硅璇诲彇鑰屾病鏈変换浣曢攣锛岀敋鑷虫病鏈?`READ_ONCE`銆傚湪 XDC 2025 涓婏紝娌℃湁浜鸿兘鐪熸璇存竻涓轰粈涔堟槸杩欑鎯呭喌銆佹槸鍚﹂渶瑕侀攣浠ュ強鏄惁鍙互娣诲姞閿併€傦紙浣嗚鐪熺殑锛岄偅澶ф搴旇鍔犻攣锛侊級妫€鏌ユ槸鍚﹀彲鑳藉湪鎵€鏈夊湴鏂规坊鍔犻攣锛屽鏋滃彲浠ュ氨杩欐牱鍋氥€?
鑱旂郴浜猴細Philipp Stanner <phasta@kernel.org>

闅惧害锛欼ntermediate

## DRM 涔嬪


### 灏?fbdev 椹卞姩杞崲涓?DRM


鏈夊ぇ閲忕敤浜庤緝鏃х‖浠剁殑 fbdev 椹卞姩銆傛湁浜涚‖浠跺凡缁忚繃鏃讹紝浣嗘湁浜涗粛鐒舵彁渚涳紙瓒冲濂界殑锛塮ramebuffer銆備粛鐒舵湁鐢ㄧ殑椹卞姩搴斿綋琚浆鎹负 DRM锛岀劧鍚庡湪 fbdev 涓Щ闄ゃ€?
闈炲父绠€鍗曠殑 fbdev 椹卞姩鏈€濂戒粠鍒涘缓涓€涓柊鐨?DRM 椹卞姩寮€濮嬭浆鎹€係imple KMS helper 涓?SHMEM 搴斿綋鑳藉澶勭悊浠讳綍鐜版湁纭欢銆傛柊椹卞姩鐨勫洖璋冨嚱鏁扮敱鐜版湁 fbdev 浠ｇ爜濉厖銆?
鏇村鏉傜殑 fbdev 椹卞姩鍙互鍦?DRM fbconv helper [^4^]_ 鐨勫府鍔╀笅閫愭閲嶆瀯涓轰竴涓?DRM 椹卞姩銆傝繖浜?helper 鎻愪緵浜?DRM 鏍稿績鍩虹璁炬柦涓?fbdev 椹卞姩鎺ュ彛涔嬮棿鐨勮繃娓″眰銆傚湪 fbconv helper 涔嬩笂鍒涘缓涓€涓柊鐨?DRM 椹卞姩锛屽鍒?fbdev 椹卞姩锛屽苟灏嗗叾鎸傛帴鍒?DRM 浠ｇ爜涓€俆homas Zimmermann 鐨?fbconv 鏍?[^4^]_ 涓彁渚涗簡鍑犱釜 fbdev 椹卞姩鐨勪緥瀛愶紝浠ュ強涓€涓杩囩▼鐨勬暀绋?[^5^]_銆傜粨鏋滄槸涓€涓彲浠ヨ繍琛?X11 涓?Weston 鐨勫師濮?DRM 椹卞姩銆?
 .. [^4^] https://gitlab.freedesktop.org/tzimmermann/linux/tree/fbconv
 .. [^5^] https://gitlab.freedesktop.org/tzimmermann/linux/blob/fbconv/drivers/gpu/drm/drm_fbconv_helper.c

鑱旂郴浜猴細Thomas Zimmermann <tzimmermann@suse.de>

闅惧害锛欰dvanced
