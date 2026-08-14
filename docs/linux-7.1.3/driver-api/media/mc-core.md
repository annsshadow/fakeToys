
### 濯掍綋鎺у埗鍣ㄨ澶?
#### 濯掍綋鎺у埗鍣?
濯掍綋鎺у埗鍣ㄧ敤鎴风┖闂?API 璁板綍鍦?Media Controller uAPI book <media_controller> 涓€傛湰鏂囨。鍏虫敞濯掍綋妗嗘灦鐨勫唴鏍镐晶瀹炵幇銆?
##### 鎶借薄濯掍綋璁惧妯″瀷

鍙戠幇璁惧鍐呴儴鎷撴墤骞惰繍琛屾椂閰嶇疆瀹冿紝鏄獟浣撴鏋剁殑鐩爣涔嬩竴銆備负瀹炵幇杩欎竴鐐癸紝纭欢璁惧琚缓妯′负涓€涓敱绉颁负瀹炰綋锛坋ntities锛夌殑鏋勫缓鍧楅€氳繃 pad 杩炴帴鑰屾垚鐨勬湁鍚戝浘銆?
瀹炰綋锛坋ntity锛夋槸涓€涓熀鏈殑濯掍綋纭欢鏋勫缓鍧椼€傚畠鍙互瀵瑰簲浜庡悇绉嶅悇鏍风殑閫昏緫鍧楋紝渚嬪鐗╃悊纭欢璁惧锛堟瘮濡?CMOS 浼犳劅鍣級銆侀€昏緫纭欢璁惧锛圫oC 鍥惧儚澶勭悊娴佹按绾夸腑鐨勪竴涓瀯寤哄潡锛夈€丏MA 閫氶亾鎴栫墿鐞嗚繛鎺ュ櫒銆?
Pad 鏄竴涓繛鎺ョ鐐癸紝瀹炰綋閫氳繃瀹冧笌鍏朵粬瀹炰綋浜や簰銆傜敱瀹炰綋浜х敓鐨勬暟鎹紙涓嶉檺浜庤棰戯級浠庡疄浣撶殑杈撳嚭娴佸悜涓€涓垨澶氫釜瀹炰綋杈撳叆銆侾ad 涓嶅簲涓庤姱鐗囪竟鐣屼笂鐨勭墿鐞嗗紩鑴氭贩娣嗐€?
Link锛堥摼鎺ワ級鏄袱涓?pad 涔嬮棿鐨勭偣瀵圭偣鏈夊悜杩炴帴锛屽彲浠ュ湪鍚屼竴瀹炰綋涓婏紝涔熷彲浠ュ湪涓嶅悓瀹炰綋涓娿€傛暟鎹粠婧?pad 娴佸悜瀹匡紙sink锛塸ad銆?
##### 濯掍綋璁惧

濯掍綋璁惧鐢变竴涓?`struct media_device` 瀹炰緥琛ㄧず锛屽畾涔変簬 `include/media/media-device.h`銆傝缁撴瀯鐨勫垎閰嶇敱濯掍綋璁惧椹卞姩澶勭悊锛岄€氬父鏄皢 `media_device` 瀹炰緥鍐呭祵鍦ㄤ竴涓洿澶х殑椹卞姩鐗瑰畾缁撴瀯涓€?
椹卞姩閫氳繃璋冪敤 `media_device_init()` 鍒濆鍖栧獟浣撹澶囧疄渚嬨€傚垵濮嬪寲濯掍綋璁惧瀹炰緥鍚庯紝閫氳繃瀹?`media_device_register()` 璋冪敤 `__media_device_register()` 鏉ユ敞鍐屽畠锛屽苟閫氳繃璋冪敤 `media_device_unregister()` 鏉ユ敞閿€瀹冦€備竴涓凡鍒濆鍖栫殑濯掍綋璁惧鏈€缁堝繀椤婚€氳繃璋冪敤 `media_device_cleanup()` 杩涜娓呯悊銆?
娉ㄦ剰锛屼笉鍏佽娉ㄩ攢涓€涓厛鍓嶆湭娉ㄥ唽鐨勫獟浣撹澶囧疄渚嬶紝鎴栨竻鐞嗕竴涓厛鍓嶆湭鍒濆鍖栫殑濯掍綋璁惧瀹炰緥銆?
##### 瀹炰綋

瀹炰綋鐢?`struct media_entity` 瀹炰緥琛ㄧず锛屽畾涔変簬 `include/media/media-entity.h`銆傝缁撴瀯閫氬父鍐呭祵鍒版洿楂樺眰鐨勭粨鏋勪腑锛屼緥濡?`v4l2_subdev` 鎴?`video_device` 瀹炰緥锛屽敖绠￠┍鍔ㄤ篃鍙互鐩存帴鍒嗛厤瀹炰綋銆?
椹卞姩閫氳繃璋冪敤 `media_entity_pads_init()` 鍒濆鍖栧疄浣撶殑 pad銆?
椹卞姩閫氳繃璋冪敤 `media_device_register_entity()` 灏嗗疄浣撴敞鍐屽埌濯掍綋璁惧锛屽苟閫氳繃璋冪敤 `media_device_unregister_entity()` 娉ㄩ攢銆?
##### 鎺ュ彛

鎺ュ彛鐢?`struct media_interface` 瀹炰緥琛ㄧず锛屽畾涔変簬 `include/media/media-entity.h`銆傜洰鍓嶅彧瀹氫箟浜嗕竴绉嶆帴鍙ｇ被鍨嬶細璁惧鑺傜偣锛坉evice node锛夈€傛绫绘帴鍙ｇ敱 `struct media_intf_devnode` 琛ㄧず銆?
椹卞姩閫氳繃璋冪敤 `media_devnode_create()` 鍒濆鍖栧苟鍒涘缓璁惧鑺傜偣鎺ュ彛锛屽苟閫氳繃璋冪敤 `media_devnode_remove()` 绉婚櫎瀹冧滑銆?
##### Pads

Pad 鐢?`struct media_pad` 瀹炰緥琛ㄧず锛屽畾涔変簬 `include/media/media-entity.h`銆傛瘡涓疄浣撳皢鍏?pad 瀛樺偍鍦ㄧ敱瀹炰綋椹卞姩绠＄悊鐨?pad 鏁扮粍涓€傞┍鍔ㄩ€氬父灏嗚鏁扮粍鍐呭祵鍦ㄩ┍鍔ㄧ壒瀹氱殑缁撴瀯涓€?
Pad 閫氳繃鍏舵墍灞炵殑瀹炰綋浠ュ強瀹冧滑鍦?pad 鏁扮粍涓殑浠?0 寮€濮嬬殑绱㈠紩鏉ユ爣璇嗐€?
杩欎袱绫讳俊鎭兘瀛樺偍鍦?`struct media_pad` 涓紝浣垮緱 `struct media_pad` 鎸囬拡鎴愪负瀛樺偍鍜屼紶閫?link 寮曠敤鐨勮鑼冩柟寮忋€?
Pad 鍏锋湁鎻忚堪鍏惰兘鍔涗笌鐘舵€佺殑鏍囧織銆?
`MEDIA_PAD_FL_SINK` 琛ㄧず璇?pad 鏀寔鎺ユ敹锛坰inking锛夋暟鎹€?`MEDIA_PAD_FL_SOURCE` 琛ㄧず璇?pad 鏀寔浜х敓锛坰ourcing锛夋暟鎹€?
  姣忎釜 pad 蹇呴』涓斾粎蹇呴』璁剧疆 `MEDIA_PAD_FL_SINK` 鎴?`MEDIA_PAD_FL_SOURCE` 涔嬩竴銆?
##### 閾炬帴

閾炬帴鐢?`struct media_link` 瀹炰緥琛ㄧず锛屽畾涔変簬 `include/media/media-entity.h`銆傛湁涓ょ绫诲瀷鐨勯摼鎺ワ細

**1. pad 鍒?pad 閾炬帴**锛?
閫氳繃 PAD 鍏宠仈涓や釜瀹炰綋銆傛瘡涓疄浣撻兘鏈変竴涓垪琛紝鎸囧悜鎵€鏈夋簮鑷垨鎸囧悜鍏朵换涓€ pad 鐨勯摼鎺ャ€傚洜姝わ紝缁欏畾鐨勯摼鎺ヨ瀛樺偍涓ゆ锛屼竴娆″湪婧愬疄浣撲腑锛屼竴娆″湪鐩爣瀹炰綋涓€?
椹卞姩閫氳繃璋冪敤 `media_create_pad_link()` 鍒涘缓 pad 鍒?pad 閾炬帴锛屽苟閫氳繃 `media_entity_remove_links()` 绉婚櫎銆?
**2. interface 鍒?entity 閾炬帴**锛?
灏嗕竴涓帴鍙ｅ叧鑱斿埌涓€涓摼鎺ャ€?
椹卞姩閫氳繃璋冪敤 `media_create_intf_link()` 鍒涘缓 interface 鍒?entity 閾炬帴锛屽苟閫氳繃 `media_remove_intf_links()` 绉婚櫎銆?
   閾炬帴鍙兘鍦ㄤ袱绔兘宸插垱寤轰箣鍚庡垱寤恒€?
閾炬帴鍏锋湁鎻忚堪鍏惰兘鍔涗笌鐘舵€佺殑鏍囧織銆傛湁鏁堝€煎湪 `media_create_pad_link()` 鍜?`media_create_intf_link()` 涓弿杩般€?
##### 鍥鹃亶鍘?
濯掍綋妗嗘灦鎻愪緵浜嗛亶鍘嗗獟浣撳浘銆佸畾浣嶇浉杩炲疄浣撳拰閾炬帴鐨?API銆?
瑕侀亶鍘嗗睘浜庢煇涓獟浣撹澶囩殑鎵€鏈夊疄浣擄紝椹卞姩鍙互浣跨敤 `media_device_for_each_entity` 瀹忥紝瀹氫箟浜?`include/media/media-device.h`銆?

    struct media_entity *entity;

    media_device_for_each_entity(entity, mdev) {
    // entity 灏嗕緷娆℃寚鍚戞瘡涓疄浣?    ...
    }

杈呭姪鍑芥暟鍙敤浜庢煡鎵句袱涓粰瀹?pad 涔嬮棿鐨勯摼鎺ワ紝鎴栭€氳繃宸插惎鐢ㄩ摼鎺ヨ繛鎺ュ埌鍙︿竴涓?pad 鐨?pad
锛坄media_entity_find_link()`銆乣media_pad_remote_pad_first()`銆?`media_entity_remote_source_pad_unique()` 鍜?`media_pad_remote_pad_unique()`锛夈€?
##### 浣跨敤璁℃暟涓庣數婧愬鐞?
鐢变簬椹卞姩鍦ㄧ數婧愮鐞嗛渶姹傛柟闈㈠樊寮傚緢澶э紝濯掍綋鎺у埗鍣ㄤ笉瀹炵幇鐢垫簮绠＄悊銆備笉杩囷紝`struct media_entity` 鍖呭惈涓€涓?`use_count` 瀛楁锛屽獟浣撻┍鍔ㄥ彲浠ヤ娇鐢ㄥ畠鏉ヨ窡韪瘡涓疄浣撶殑鐢ㄦ埛鏁伴噺浠ユ弧瓒崇數婧愮鐞嗛渶姹傘€?
`media_entity`.\ `use_count` 瀛楁褰掑獟浣撻┍鍔ㄦ墍鏈夛紝瀹炰綋椹卞姩涓嶅緱瑙︾銆傚璇ュ瓧娈电殑璁块棶蹇呴』鐢?`media_device`.\ `graph_mutex` 閿佷繚鎶ゃ€?
##### 閾炬帴璁剧疆

閾炬帴灞炴€у彲浠ラ€氳繃璋冪敤 `media_entity_setup_link()` 鍦ㄨ繍琛屾椂淇敼銆?
##### 娴佹按绾夸笌濯掍綋娴?
濯掍綋娴侊紙media stream锛夋槸婧愯嚜涓€涓垨澶氫釜婧愯澶囷紙渚嬪浼犳劅鍣級骞舵祦缁忓獟浣撳疄浣?pad 鍒拌揪鏈€缁堝鐨勫儚绱犳垨鍏冩暟鎹祦銆傝娴佸彲浠ュ湪璺緞涓婅璁惧淇敼锛堜緥濡傜缉鏀炬垨鍍忕礌鏍煎紡杞崲锛夛紝涔熷彲浠ヨ鎷嗗垎涓哄涓垎鏀紝鎴栬€呭涓垎鏀彲浠ヨ鍚堝苟銆?
濯掍綋娴佹按绾匡紙media pipeline锛夋槸涓€缁勭浉浜掍緷璧栫殑濯掍綋娴併€傝繖绉嶇浉浜掍緷璧栧彲鑳芥槸鐢辩‖浠跺紩璧风殑锛堜緥濡傦紝濡傛灉绗竴鏉℃祦宸蹭娇鑳斤紝鍒欑浜屾潯娴侀厤缃棤娉曟洿鏀癸級锛屾垨鐢遍┍鍔ㄧ敱浜庤蒋浠惰璁″紩璧枫€傛渶甯歌鐨勬儏鍐垫槸锛屽獟浣撴祦姘寸嚎鐢变竴鏉′笉鍒嗗弶鐨勫崟涓祦缁勬垚銆?
寮€濮嬫祦寮忎紶杈撴椂锛岄┍鍔ㄥ繀椤婚€氱煡娴佹按绾夸腑鐨勬墍鏈夊疄浣擄紝浠ラ槻姝㈠湪娴佸紡浼犺緭鏈熼棿閾炬帴鐘舵€佽淇敼锛屾柟娉曟槸璋冪敤 `media_pipeline_start()`銆?
璇ュ嚱鏁颁細灏嗘祦姘寸嚎涓墍鏈変綔涓烘祦姘寸嚎涓€閮ㄥ垎鐨?pad 鏍囪涓烘鍦ㄦ祦寮忎紶杈撱€?
`pipe` 鍙傛暟鎸囧悜鐨?`struct media_pipeline` 瀹炰緥灏嗗瓨鍌ㄥ湪娴佹按绾夸腑鐨勬瘡涓?pad 涓€傞┍鍔ㄥ簲灏?`struct media_pipeline` 鍐呭祵鍒版洿楂樺眰鐨勬祦姘寸嚎缁撴瀯涓紝鐒跺悗鍙互閫氳繃 `struct media_pad` 鐨?pipe 瀛楁璁块棶璇ユ祦姘寸嚎銆?
瀵?`media_pipeline_start()` 鐨勮皟鐢ㄥ彲浠ュ祵濂椼€傛墍鏈夊祵濂楄皟鐢ㄨ鍑芥暟鏃讹紝娴佹按绾挎寚閽堝繀椤荤浉鍚屻€?
`media_pipeline_start()` 鍙兘杩斿洖閿欒銆傚湪杩欑鎯呭喌涓嬶紝瀹冧細鑷娓呯悊瀹冩墍鍋氱殑浠讳綍鏇存敼銆?
鍋滄娴佹椂锛岄┍鍔ㄥ繀椤婚€氳繃 `media_pipeline_stop()` 閫氱煡瀹炰綋銆?
濡傛灉澶氭璋冪敤 `media_pipeline_start()`锛屽垯闇€瑕佺浉鍚屾鏁扮殑 `media_pipeline_stop()` 璋冪敤鎵嶈兘鍋滄娴佸紡浼犺緭銆傚湪鏈€鍚庝竴涓祵濂?stop 璋冪敤鏃讹紝`media_entity`.\ `pipe` 瀛楁琚噸缃负 `NULL`銆?
濡傛灉閾炬帴鐨勪换涓€绔槸姝ｅ湪娴佸紡浼犺緭鐨勫疄浣擄紝榛樿鎯呭喌涓嬮摼鎺ラ厤缃皢澶辫触骞惰繑鍥?`-EBUSY`銆傚湪娴佸紡浼犺緭鏈熼棿鍙互淇敼鐨勯摼鎺ュ繀椤绘爣璁颁负 `MEDIA_LNK_FL_DYNAMIC` 鏍囧織銆?
濡傛灉鍏朵粬鎿嶄綔闇€瑕佽绂佹鍦ㄦ祦寮忎紶杈撶殑瀹炰綋涓婏紙渚嬪鏇存敼瀹炰綋閰嶇疆鍙傛暟锛夛紝椹卞姩鍙互鏄惧紡妫€鏌?media_entity 鐨?stream_count 瀛楁浠ユ煡鏄庢煇涓疄浣撴槸鍚︽鍦ㄦ祦寮忎紶杈撱€傛鎿嶄綔蹇呴』鍦ㄦ寔鏈?media_device graph_mutex 鐨勬儏鍐典笅杩涜銆?
##### 閾炬帴楠岃瘉

`media_pipeline_start()` 浼氬娴佹按绾夸腑浠讳綍鍏锋湁瀹?pad 鐨勫疄浣撴墽琛岄摼鎺ラ獙璇併€備负姝や娇鐢?`media_entity`.\ `link_validate()` 鍥炶皟銆傚湪 `link_validate()` 鍥炶皟涓紝瀹炰綋椹卞姩搴旀鏌ョ浉杩炲疄浣撶殑婧?pad 鐨勫睘鎬т笌鍏惰嚜韬殑瀹?pad 鏄惁鍖归厤銆傚疄闄呭尮閰嶇殑鍚箟鍙栧喅浜庡疄浣撶殑绫诲瀷锛堟渶缁堝彇鍐充簬纭欢鐨勫睘鎬э級銆?
瀛愮郴缁熷簲褰撻€氳繃鎻愪緵瀛愮郴缁熺壒瀹氱殑杈呭姪鍑芥暟鏉ヤ究浜庨摼鎺ラ獙璇侊紝浠ヤ究杞绘澗璁块棶閫氬父闇€瑕佺殑淇℃伅锛屽苟鏈€缁堟彁渚涗竴绉嶄娇鐢ㄩ┍鍔ㄧ壒瀹氬洖璋冪殑鏂瑰紡銆?
##### 娴佹按绾块亶鍘?
涓€鏃︿娇鐢?`media_pipeline_start()` 鏋勫缓濂芥祦姘寸嚎锛岄┍鍔ㄥ氨鍙互浣跨敤 `:c:macro:麓media_pipeline_for_each_entity` 鍜?`:c:macro:麓media_pipeline_for_each_pad` 瀹忛亶鍘嗘祦姘寸嚎涓殑瀹炰綋鎴?pad銆傞亶鍘?pad 鏄洿鎺ョ殑锛?

   media_pipeline_pad_iter iter;
   struct media_pad *pad;

   media_pipeline_for_each_pad(pipe, &iter, pad) {
       /** 'pad' 灏嗕緷娆℃寚鍚戞瘡涓?pad **/
       ...
   }

瑕侀亶鍘嗗疄浣擄紝浣滀负棰濆姝ラ锛岃凯浠ｅ櫒闇€瑕佽鍒濆鍖栧拰娓呯悊锛?

   media_pipeline_entity_iter iter;
   struct media_entity *entity;
   int ret;

   ret = media_pipeline_entity_iter_init(pipe, &iter);
   if (ret)
       ...;

   media_pipeline_for_each_entity(pipe, &iter, entity) {
       /** 'entity' 灏嗕緷娆℃寚鍚戞瘡涓疄浣?**/
       ...
   }

   media_pipeline_entity_iter_cleanup(&iter);

##### 濯掍綋鎺у埗鍣ㄨ澶囧垎閰嶅櫒 API

褰撳獟浣撹澶囧睘浜庡涓┍鍔ㄦ椂锛屽叡浜殑濯掍綋璁惧浣跨敤鍏变韩鐨?struct device 浣滀负鏌ユ壘鐨勯敭鏉ュ垎閰嶃€?
鍏变韩濯掍綋璁惧搴斾竴鐩翠繚鎸佹敞鍐岀姸鎬侊紝鐩村埌鏈€鍚庝竴涓┍鍔ㄦ敞閿€瀹冦€傛澶栵紝褰撴墍鏈夊紩鐢ㄩ兘琚噴鏀炬椂锛屽獟浣撹澶囨墠搴旇閲婃斁銆傛瘡涓┍鍔ㄥ湪鎺㈡祴锛坧robe锛夋湡闂村垎閰嶅獟浣撹澶囨椂鑾峰緱瀵瑰獟浣撹澶囩殑涓€涓紩鐢ㄣ€傚鏋滃獟浣撹澶囧凡琚垎閰嶏紝鍒嗛厤 API 浼氬鍔犲紩鐢ㄨ鏁板苟杩斿洖鐜版湁鐨勫獟浣撹澶囥€傞┍鍔ㄥ湪鍏舵柇寮€杩炴帴锛坉isconnect锛変緥绋嬩腑璋冪敤 `media_device_delete()` 鏃跺皢璇ュ紩鐢ㄦ斁鍥炪€?
濯掍綋璁惧浠?kref put 澶勭悊绋嬪簭杩涜娉ㄩ攢鍜屾竻鐞嗭紝浠ョ‘淇濆獟浣撹澶囦繚鎸佹敞鍐岀姸鎬侊紝鐩村埌鏈€鍚庝竴涓┍鍔ㄦ敞閿€濯掍綋璁惧銆?
**椹卞姩鐢ㄦ硶**

椹卞姩搴斾娇鐢ㄩ€傚綋鐨?media-core 渚嬬▼鏉ョ鐞嗗叡浜獟浣撹澶囩殑鐢熷懡鍛ㄦ湡锛屽鐞嗕袱绉嶇姸鎬侊細
1. allocate -> register -> delete
2. 鑾峰彇瀵瑰凡娉ㄥ唽璁惧鐨勫紩鐢?-> delete

璋冪敤 `media_device_delete()` 渚嬬▼浠ョ‘淇濆叡浜獟浣撹澶囩殑鍒犻櫎琚纭鐞嗐€?
**椹卞姩鎺㈡祴锛坧robe锛夛細**
璋冪敤 `media_device_usb_allocate()` 鏉ュ垎閰嶆垨鑾峰彇寮曠敤
濡傛灉濯掍綋 devnode 灏氭湭娉ㄥ唽锛屽垯璋冪敤 `media_device_register()`

**椹卞姩鏂紑杩炴帴锛坉isconnect锛夛細**
璋冪敤 `media_device_delete()` 閲婃斁 media_device銆傞噴鏀剧敱 kref put 澶勭悊绋嬪簭澶勭悊銆?
##### API 瀹氫箟

















