## BPF 杩唬鍣?

### 姒傝堪


BPF 鏀寔涓や釜鍚堢О涓恒€孊PF iterators銆嶏紙BPF 杩唬鍣級鐨勭嫭绔嬪疄浣擄細BPF 杩唬鍣?*绋嬪簭绫诲瀷**锛坧rogram type锛変笌**寮€鏀剧紪鐮?*锛坥pen-coded锛夌殑 BPF 杩唬鍣ㄣ€傚墠鑰呮槸涓€绉嶇嫭绔嬬殑 BPF 绋嬪簭绫诲瀷锛屽綋鐢ㄦ埛灏嗗叾闄勫姞骞舵縺娲诲悗锛屼細瀵规瘡涓杩唬鐨勫疄浣擄紙task_struct銆乧group 绛夛級璋冪敤涓€娆°€傚悗鑰呮槸涓€缁勫疄鐜拌凯浠ｅ櫒鍔熻兘鐨?BPF 绔?API锛屽彲鍦ㄥ绉?BPF 绋嬪簭绫诲瀷涓娇鐢ㄣ€傚紑鏀剧紪鐮佽凯浠ｅ櫒鎻愪緵涓?BPF 杩唬鍣ㄧ▼搴忕被浼肩殑鍔熻兘锛屼絾璧嬩簣鎵€鏈夊叾浠?BPF 绋嬪簭绫诲瀷鏇村ぇ鐨勭伒娲绘€т笌鎺у埗鍔涖€傚彟涓€鏂归潰锛孊PF 杩唬鍣ㄧ▼搴忓彲鐢ㄤ簬瀹炵幇鍖垮悕鎴栨寕杞戒簬 BPF FS 鐨勭壒娈婃枃浠讹紝鍏跺唴瀹圭敱闄勫姞鐨?BPF 杩唬鍣ㄧ▼搴忕敓鎴愶紝骞朵互 seq_file 鍔熻兘涓烘敮鎾戙€備袱鑰呰鍏蜂綋闇€姹傞兘寰堟湁鐢ㄣ€?
鍦ㄦ柊澧炰竴涓?BPF 杩唬鍣ㄧ▼搴忔椂锛屾湡鏈涘悓鏃朵互寮€鏀剧紪鐮佽凯浠ｅ櫒鐨勫舰寮忔坊鍔犵被浼煎姛鑳戒互鑾峰緱鏈€澶х殑鐏垫椿鎬с€傚悓鏃朵篃鏈熸湜杩唬閫昏緫涓庝唬鐮佸湪涓ょ杩唬鍣?API 鎺ュ彛涔嬮棿寰椾互鏈€澶х▼搴﹀湴鍏变韩涓庡鐢ㄣ€?
### 寮€鏀剧紪鐮佺殑 BPF 杩唬鍣?

寮€鏀剧紪鐮?BPF 杩唬鍣ㄥ疄鐜颁负绱у瘑鑰﹀悎鐨?kfunc 涓夊厓缁勶紙鏋勯€犲嚱鏁般€佷笅涓€涓厓绱犺幏鍙栥€佹瀽鏋勫嚱鏁帮級浠ュ強鎻忚堪鏍堜笂杩唬鍣ㄧ姸鎬佺殑杩唬鍣ㄧ壒瀹氱被鍨嬶紝BPF 楠岃瘉鍣ㄤ繚璇佽鐘舵€佷笉浼氬湪鐩稿簲鐨?constructor/destructor/next API 涔嬪琚鏀广€?
姣忕寮€鏀剧紪鐮?BPF 杩唬鍣ㄩ兘鏈夊叾鍏宠仈鐨?struct bpf_iter_<type>锛屽叾涓?<type> 琛ㄧず鐗瑰畾鐨勮凯浠ｅ櫒绫诲瀷銆俠pf_iter_<type> 鐘舵€侀渶瑕佷綅浜?BPF 绋嬪簭鏍堜笂锛屽洜姝よ纭繚瀹冭冻澶熷皬浠ラ€傞厤 BPF 鏍堛€傚嚭浜庢€ц兘鑰冭檻锛屾渶濂介伩鍏嶄负杩唬鍣ㄧ姸鎬佽繘琛屽姩鎬佸唴瀛樺垎閰嶏紝骞跺皢鐘舵€佺粨鏋勭殑澶у皬璁惧緱瓒充互瀹圭撼涓€鍒囧繀瑕佸唴瀹广€備絾濡傛湁蹇呰锛屽姩鎬佸唴瀛樺垎閰嶆槸缁曡繃 BPF 鏍堥檺鍒剁殑涓€绉嶆柟寮忋€傛敞鎰忥紝鐘舵€佺粨鏋勭殑澶у皬灞炰簬杩唬鍣ㄧ敤鎴峰彲瑙?API 鐨勪竴閮ㄥ垎锛屽洜姝ゆ洿鏀瑰畠浼氱牬鍧忓悜鍚庡吋瀹规€э紝鍦ㄨ璁℃椂鍔″繀鎱庨噸銆?
鎵€鏈?kfunc锛堟瀯閫犲嚱鏁般€乶ext銆佹瀽鏋勫嚱鏁帮級蹇呴』涓€鑷村湴鍒嗗埆鍛藉悕涓?bpf_iter_<type>_{new,next,destroy}()銆?type> 琛ㄧず杩唬鍣ㄧ被鍨嬶紝杩唬鍣ㄧ姸鎬佸簲琛ㄧず涓哄尮閰嶇殑 `struct bpf_iter_<type>` 鐘舵€佺被鍨嬨€傛澶栵紝鎵€鏈?iter kfunc 閮藉簲灏嗘寚鍚戣 `struct bpf_iter_<type>` 鐨勬寚閽堜綔涓虹涓€涓弬鏁般€?
姝ゅ锛?  - 鏋勯€犲嚱鏁帮紝鍗?`bpf_iter_<type>_new()`锛屽彲浠ユ湁浠绘剰鏁伴噺鐨勯澶栧弬鏁般€傝繑鍥炵被鍨嬩篃涓嶄綔寮哄埗瑕佹眰銆?  - next 鏂规硶锛屽嵆 `bpf_iter_<type>_next()`锛屽繀椤昏繑鍥炴寚閽堢被鍨嬶紝涓斿簲鎭板ソ鏈変竴涓弬鏁帮細`struct bpf_iter_<type> *`锛坈onst/volatile/restrict 涓?typedef 琚拷鐣ワ級銆?  - 鏋愭瀯鍑芥暟锛屽嵆 `bpf_iter_<type>_destroy()`锛屽簲杩斿洖 void锛屼笖搴旀伆濂芥湁涓€涓弬鏁帮紝涓?next 鏂规硶绫讳技銆?  - `struct bpf_iter_<type>` 鐨勫ぇ灏忚寮哄埗瑕佹眰涓烘鍊间笖涓?8 瀛楄妭鐨勫€嶆暟锛堜互姝ｇ‘閫傞厤鏍堟Ы锛夈€?
杩欑涓ユ牸鎬т笌涓€鑷存€т娇寰楀彲浠ユ瀯寤洪€氱敤杈呭姪鍑芥暟锛屽皢閲嶈浣嗘牱鏉垮寲鐨勭粏鑺傛娊璞″嚭鏉ワ紝浠庤€岃兘澶熼珮鏁堜笖椤烘墜鍦颁娇鐢ㄥ紑鏀剧紪鐮佽凯浠ｅ櫒锛堝弬瑙?libbpf 鐨?bpf_for_each() 瀹忥級銆傝繖涓€鐐圭敱鍐呮牳鍦?kfunc 娉ㄥ唽鐐瑰己鍒舵墽琛屻€?
鏋勯€犲嚱鏁?next/鏋愭瀯鍑芥暟鐨勫疄鐜板绾﹀涓嬶細
  - 鏋勯€犲嚱鏁?`bpf_iter_<type>_new()` 鎬绘槸鍦ㄦ爤涓婂垵濮嬪寲杩唬鍣ㄧ姸鎬併€傚鏋滀换浣曡緭鍏ュ弬鏁版棤鏁堬紝鏋勯€犲嚱鏁颁粛搴旂‘淇濆畬鎴愬垵濮嬪寲锛屼互浣垮悗缁殑 next() 璋冪敤杩斿洖 NULL銆傚嵆锛屽嚭閿欐椂**杩斿洖閿欒骞舵瀯閫犵┖杩唬鍣?*銆傛瀯閫犲嚱鏁?kfunc 琚爣璁?KF_ITER_NEW 鏍囧織銆?  - next 鏂规硶 `bpf_iter_<type>_next()` 鎺ュ彈鎸囧悜杩唬鍣ㄧ姸鎬佺殑鎸囬拡骞朵骇鍑轰竴涓厓绱犮€俷ext 鏂规硶搴斿缁堣繑鍥炰竴涓寚閽堛€備笌 BPF 楠岃瘉鍣ㄧ殑濂戠害鏄細next 鏂规硶**淇濊瘉**鍦ㄥ厓绱犺€楀敖鏃舵渶缁堣繑鍥?NULL銆備竴鏃﹁繑鍥?NULL锛屽悗缁?next 璋冪敤**搴旀寔缁繑鍥?NULL**銆俷ext 鏂规硶琚爣璁?KF_ITER_NEXT锛堝綋鐒讹紝瀹冭繕搴斿叿鏈?KF_RET_NULL 浠ヨ〃绀鸿繑鍥?NULL 鐨?kfunc锛夈€?  - 鏋愭瀯鍑芥暟 `bpf_iter_<type>_destroy()` 鎬绘槸琚皟鐢ㄤ竴娆°€傚嵆浣挎瀯閫犲嚱鏁板け璐ユ垨 next 娌℃湁杩斿洖浠讳綍鍐呭銆傛瀽鏋勫嚱鏁伴噴鏀炬墍鏈夎祫婧愶紝骞跺皢 `struct bpf_iter_<type>` 浣跨敤鐨勬爤绌洪棿鏍囪涓哄彲鐢ㄤ簬鍏朵粬鐢ㄩ€斻€傛瀽鏋勫嚱鏁拌鏍囪 KF_ITER_DESTROY 鏍囧織銆?
浠讳綍寮€鏀剧紪鐮?BPF 杩唬鍣ㄥ疄鐜伴兘蹇呴』鑷冲皯瀹炵幇杩欎笁涓柟娉曘€傚唴鏍稿己鍒惰姹傦細瀵逛簬浠绘剰缁欏畾鐨勮凯浠ｅ櫒绫诲瀷锛屽彧鏈夐€傜敤鐨?constructor/destructor/next 鍙璋冪敤銆傚嵆锛岄獙璇佸櫒纭繚浣犱笉鑳藉皢锛堜緥濡傦級number 杩唬鍣ㄧ姸鎬佷紶鍏?cgroup 杩唬鍣ㄧ殑 next 鏂规硶銆?
浠庡畯瑙傜殑 BPF 楠岃瘉瑙嗚鏉ョ湅锛宯ext 鏂规硶鏄垎鍙夐獙璇佺姸鎬佺殑鐐癸紝鍦ㄦ蹇典笂绫讳技浜庨獙璇佸櫒鍦ㄦ牎楠屾潯浠惰烦杞椂鎵€鍋氱殑鎿嶄綔銆傞獙璇佸櫒瀵?`call bpf_iter_<type>_next` 鎸囦护杩涜鍒嗗弶锛屽苟妯℃嫙涓ょ缁撴灉锛歂ULL锛堣凯浠ｅ畬鎴愶級涓庨潪 NULL锛堣繑鍥炴柊鍏冪礌锛夈€傞鍏堟ā鎷?NULL锛屽苟涓斿簲褰撳湪涓嶈繘鍏ュ惊鐜殑鎯呭喌涓嬪埌杈鹃€€鍑恒€備箣鍚庨獙璇侀潪 NULL 鐨勬儏鍐碉紝瀹冭涔堝埌杈鹃€€鍑猴紙瀵逛簬娌℃湁鐪熸寰幆鐨勭畝鍗曠ず渚嬶級锛岃涔堝埌杈惧彟涓€鏉?`call bpf_iter_<type>_next` 鎸囦护锛屽叾鐘舵€佷笌宸茬粡锛堥儴鍒嗭級楠岃瘉杩囩殑鐘舵€佺瓑浠枫€傛鏃剁殑鐘舵€佺瓑浠锋剰鍛崇潃锛屼粠鎶€鏈笂璁叉垜浠皢姘歌繙寰幆锛岃€屾棤娉曘€岃烦鍑恒€嶅凡寤虹珛鐨勩€岀姸鎬佸寘缁溿€嶏紙鍗筹紝鍚庣画杩唬涓嶄細鍚戦獙璇佸櫒鐘舵€佹坊鍔犱换浣曟柊鐭ヨ瘑鎴栫害鏉燂紝鍥犳杩愯 1 娆°€? 娆°€?0 娆℃垨涓€鐧句竾娆￠兘鏃犲叧绱ц锛夈€備絾鑰冭檻鍒板绾﹁瀹氳凯浠ｅ櫒 next 鏂规硶**蹇呴』**鏈€缁堣繑鍥?NULL锛屾垜浠彲浠ュ緱鍑虹粨璁猴細寰幆浣撴槸瀹夊叏鐨勶紝涓旀渶缁堜細缁堟銆傞壌浜庢垜浠凡缁忛獙璇佷簡寰幆涔嬪鐨勯€昏緫锛圢ULL 鎯呭喌锛夛紝骞跺緱鍑哄惊鐜綋瀹夊叏锛堝敖绠″彲鑳藉惊鐜娆★級鐨勭粨璁猴紝楠岃瘉鍣ㄥ彲浠ュ垽瀹氭暣涓▼搴忛€昏緫鐨勫畨鍏ㄦ€с€?
### BPF 杩唬鍣ㄧ殑鍔ㄦ満


鐜版湁鍑犵灏嗗唴鏍告暟鎹浆鍌ㄥ埌鐢ㄦ埛绌洪棿鐨勬柟寮忋€傛渶娴佽鐨勬槸 `/proc` 绯荤粺銆備緥濡傦紝`cat /proc/net/tcp6` 杞偍绯荤粺涓墍鏈夌殑 tcp6 濂楁帴瀛楋紝`cat /proc/net/netlink` 杞偍绯荤粺涓墍鏈夌殑 netlink 濂楁帴瀛椼€傜劧鑰岋紝瀹冧滑鐨勮緭鍑烘牸寮忓線寰€鍥哄畾锛屽鏋滅敤鎴锋兂瑕佹洿澶氬叧浜庤繖浜涘鎺ュ瓧鐨勪俊鎭紝灏卞繀椤荤粰鍐呮牳鎵撹ˉ涓侊紝鑰岃繖閫氬父闇€瑕佸緢闀挎椂闂存墠鑳藉悎鍏ヤ笂娓稿苟鍙戝竷銆傚浜?`ss <https://man7.org/linux/man-pages/man8/ss.8.html>`_ 绛夋祦琛屽伐鍏蜂篃鏄姝わ紝浠讳綍棰濆淇℃伅閮介渶瑕佸唴鏍歌ˉ涓併€?
涓鸿В鍐宠繖涓棶棰橈紝甯稿父浣跨敤 `drgn <https://www.kernel.org/doc/html/latest/bpf/drgn.html>`_ 宸ュ叿鍦ㄤ笉淇敼鍐呮牳鐨勬儏鍐典笅鎸栨帢鍐呮牳鏁版嵁銆傜劧鑰岋紝drgn 鐨勪富瑕佺己鐐瑰湪浜庢€ц兘锛屽洜涓哄畠鏃犳硶鍦ㄥ唴鏍稿唴閮ㄨ繘琛屾寚閽堣拷韪€傛澶栵紝drgn 鏃犳硶楠岃瘉鎸囬拡鍊硷紝濡傛灉鎸囬拡鍦ㄥ唴鏍镐腑鍙樹负鏃犳晥锛屽彲鑳戒細璇诲彇鍒版棤鏁堟暟鎹€?
BPF 杩唬鍣ㄩ€氳繃鎻愪緵鐏垫椿鎬цВ鍐充簡涓婅堪闂鈥斺€斿畠閫氳繃瀵规瘡涓唴鏍告暟鎹璞¤皟鐢?BPF 绋嬪簭锛屾潵鐏垫椿鍦版敹闆嗗摢浜涙暟鎹紙渚嬪 tasks銆乥pf_maps 绛夛級銆?
### BPF 杩唬鍣ㄧ殑宸ヤ綔鍘熺悊


BPF 杩唬鍣ㄦ槸涓€绉嶅厑璁哥敤鎴烽亶鍘嗙壒瀹氱被鍨嬪唴鏍稿璞＄殑 BPF 绋嬪簭銆備笌鍏佽鐢ㄦ埛瀹氫箟鍦ㄥ唴鏍镐腑鐗瑰畾鎵ц鐐硅璋冪敤鐨勫洖璋冪殑浼犵粺 BPF 璺熻釜绋嬪簭涓嶅悓锛孊PF 杩唬鍣ㄥ厑璁哥敤鎴峰畾涔夊簲瀵瑰绉嶅唴鏍告暟鎹粨鏋勪腑鐨勬瘡涓€椤规墽琛岀殑鍥炶皟銆?
渚嬪锛岀敤鎴峰彲浠ュ畾涔変竴涓亶鍘嗙郴缁熶笂姣忎釜 task 骞惰浆鍌ㄥ畠浠綋鍓嶅悇鑷娇鐢ㄧ殑 CPU 杩愯鏃堕暱鎬婚噺鐨?BPF 杩唬鍣ㄣ€傚彟涓€涓?BPF task 杩唬鍣ㄥ垯鍙互杞偍姣忎釜 task 鐨?cgroup 淇℃伅銆傝繖绉嶇伒娲绘€ф鏄?BPF 杩唬鍣ㄧ殑鏍稿績浠峰€笺€?
BPF 绋嬪簭鎬绘槸鐢辩敤鎴风┖闂磋繘绋嬪湪闇€瑕佹椂鍔犺浇鍒板唴鏍镐腑銆傜敤鎴风┖闂磋繘绋嬮€氳繃鎸夎姹傛墦寮€骞跺垵濮嬪寲绋嬪簭楠ㄦ灦锛坰keleton锛夛紝鐒跺悗璋冪敤绯荤粺璋冪敤锛屼娇 BPF 绋嬪簭鐢卞唴鏍搁獙璇佸苟鍔犺浇銆?
鍦ㄤ紶缁熻窡韪▼搴忎腑锛岀▼搴忛€氳繃鐢ㄦ埛绌洪棿鐢?`bpf_program__attach()` 鑾峰彇绋嬪簭鐨?`bpf_link` 鑰岃婵€娲汇€備竴鏃︽縺娲伙紝姣忓綋涓诲唴鏍镐腑瑙﹀彂 tracepoint 鏃讹紝绋嬪簭鍥炶皟灏变細琚皟鐢ㄣ€傚浜?BPF 杩唬鍣ㄧ▼搴忥紝绋嬪簭鐨?`bpf_link` 閫氳繃 `bpf_link_create()` 鑾峰彇锛岀▼搴忓洖璋冨垯鐢辩敤鎴风┖闂村彂鍑虹郴缁熻皟鐢ㄨ€岃Е鍙戙€?
鎺ヤ笅鏉ワ紝璁╂垜浠湅鐪嬪浣曚娇鐢ㄨ凯浠ｅ櫒閬嶅巻鍐呮牳瀵硅薄骞惰鍙栨暟鎹€?
### 濡備綍浣跨敤 BPF 杩唬鍣?

BPF selftests锛堣嚜娴嬶級鏄鏄庡浣曚娇鐢ㄨ凯浠ｅ櫒鐨勬瀬浣宠祫婧愩€傚湪鏈妭涓紝鎴戜滑灏嗚蛋鏌ヤ竴涓睍绀哄浣曞姞杞藉拰浣跨敤 BPF 杩唬鍣ㄧ▼搴忕殑 BPF 鑷祴銆傞鍏堬紝鎴戜滑鏉ョ湅 `bpf_iter.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/prog_tests/bpf_iter.c>`_锛屽畠灞曠ず浜嗗浣曞湪鐢ㄦ埛绌洪棿渚у姞杞藉苟瑙﹀彂 BPF 杩唬鍣ㄣ€備箣鍚庯紝鎴戜滑灏嗙湅涓€涓繍琛屽湪鍐呮牳绌洪棿鐨?BPF 绋嬪簭銆?
浠庣敤鎴风┖闂村湪鍐呮牳涓姞杞?BPF 杩唬鍣ㄩ€氬父娑夊強浠ヤ笅姝ラ锛?
- 閫氳繃 `libbpf` 灏?BPF 绋嬪簭鍔犺浇鍒板唴鏍镐腑銆備竴鏃﹀唴鏍搁獙璇佸苟鍔犺浇浜嗚绋嬪簭锛屽畠浼氬悜鐢ㄦ埛绌洪棿杩斿洖涓€涓枃浠舵弿杩扮锛坒d锛夈€?- 閫氳繃璋冪敤 `bpf_link_create()` 骞舵寚瀹氫粠鍐呮牳鏀跺埌鐨?BPF 绋嬪簭鏂囦欢鎻忚堪绗︼紝鑾峰彇璇?BPF 绋嬪簭鐨?`link_fd`銆?- 鎺ヤ笅鏉ワ紝閫氳繃璋冪敤浠ョ 2 姝ユ敹鍒扮殑 `bpf_link` 涓哄弬鏁扮殑 `bpf_iter_create()`锛岃幏鍙?BPF 杩唬鍣ㄦ枃浠舵弿杩扮锛坄bpf_iter_fd`锛夈€?- 閫氳繃璋冪敤 `read(bpf_iter_fd)` 瑙﹀彂杩唬锛岀洿鍒版病鏈夋暟鎹彲鐢ㄣ€?- 浣跨敤 `close(bpf_iter_fd)` 鍏抽棴杩唬鍣?fd銆?- 濡傛灉闇€瑕侀噸鏂拌鍙栨暟鎹紝鑾峰彇涓€涓柊鐨?`bpf_iter_fd` 骞跺啀娆¤鍙栥€?
浠ヤ笅鏄嚑涓嚜娴?BPF 杩唬鍣ㄧ▼搴忕殑绀轰緥锛?
- `bpf_iter_tcp4.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_tcp4.c>`_
- `bpf_iter_task_vmas.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_task_vmas.c>`_
- `bpf_iter_task_file.c <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/tools/testing/selftests/bpf/progs/bpf_iter_task_file.c>`_

璁╂垜浠潵鐪嬭繍琛屽湪鍐呮牳绌洪棿鐨?`bpf_iter_task_file.c`锛?
浠ヤ笅鏄?`vmlinux.h <https://facebookmicrosites.github.io/bpf/blog/2020/02/19/bpf-portability-and-co-re.html#btf>`_ 涓?`bpf_iter__task_file` 鐨勫畾涔夈€傚湪 `vmlinux.h` 涓紝浠讳綍鏍煎紡涓?`bpf_iter__<iter_name>` 鐨勭粨鏋勪綋鍚嶇О閮借〃绀轰竴涓?BPF 杩唬鍣ㄣ€傚悗缂€ `<iter_name>` 琛ㄧず杩唬鍣ㄧ殑绫诲瀷銆?
```

    struct bpf_iter__task_file {
            union {
                struct bpf_iter_meta *meta;
            };
            union {
                struct task_struct *task;
            };
            u32 fd;
            union {
                struct file *file;
            };
    };

```
鍦ㄤ笂杩颁唬鐮佷腑锛屽瓧娈?'meta' 鍖呭惈鍏冩暟鎹紝杩欏鎵€鏈?BPF 杩唬鍣ㄧ▼搴忛兘鏄浉鍚岀殑銆傚叾浣欏瓧娈靛垯鐗瑰畾浜庝笉鍚岀殑杩唬鍣ㄣ€備緥濡傦紝瀵逛簬 task_file 杩唬鍣紝鍐呮牳灞傛彁渚?'task'銆?fd' 涓?'file' 瀛楁鍊笺€?task' 涓?'file' 鏄痐寮曠敤璁℃暟 <https://facebookmicrosites.github.io/bpf/blog/2018/08/31/object-lifetime.html#file-descriptors-and-reference-counters>`_ 鐨勶紝鍥犳鍦?BPF 绋嬪簭杩愯鏃跺畠浠笉浼氭秷澶便€?
浠ヤ笅鏄?`bpf_iter_task_file.c` 鏂囦欢鐨勭墖娈碉細

```

  SEC("iter/task_file")
  int dump_task_file(struct bpf_iter__task_file *ctx)
  {
    struct seq_file *seq = ctx->meta->seq;
    struct task_struct *task = ctx->task;
    struct file *file = ctx->file;
    __u32 fd = ctx->fd;

    if (task == NULL || file == NULL)
      return 0;

    if (ctx->meta->seq_num == 0) {
      count = 0;
      BPF_SEQ_PRINTF(seq, "    tgid      gid       fd      file\n");
    }

    if (tgid == task->tgid && task->tgid != task->pid)
      count++;

    if (last_tgid != task->tgid) {
      last_tgid = task->tgid;
      unique_tgid_count++;
    }

    BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
            (long)file->f_op);
    return 0;
  }

```
鍦ㄤ笂杩扮ず渚嬩腑锛屾鍚?`SEC(iter/task_file)` 琛ㄦ槑璇ョ▼搴忔槸涓€涓敤浜庨亶鍘嗘墍鏈?task 鐨勫叏閮ㄦ枃浠剁殑 BPF 杩唬鍣ㄧ▼搴忋€傝绋嬪簭鐨勪笂涓嬫枃鏄?`bpf_iter__task_file` 缁撴瀯浣撱€?
鐢ㄦ埛绌洪棿绋嬪簭閫氳繃鍙戝嚭 `read()` 绯荤粺璋冪敤鏉ヨ皟鐢ㄨ繍琛屽湪鍐呮牳涓殑 BPF 杩唬鍣ㄧ▼搴忋€備竴鏃﹁璋冪敤锛孊PF 绋嬪簭灏卞彲浠ヤ娇鐢ㄥ悇绉?BPF 杈呭姪鍑芥暟灏嗘暟鎹鍑哄埌鐢ㄦ埛绌洪棿銆傛牴鎹綘鏄惁闇€瑕佹牸寮忓寲杈撳嚭鎴栦粎浠呮槸浜岃繘鍒舵暟鎹紝鍙互鍒嗗埆浣跨敤 `bpf_seq_printf()`锛堜互鍙?BPF_SEQ_PRINTF 杈呭姪瀹忥級鎴?`bpf_seq_write()` 鍑芥暟銆傚浜庝簩杩涘埗缂栫爜鐨勬暟鎹紝鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互鎸夐渶澶勭悊鏉ヨ嚜 `bpf_seq_write()` 鐨勬暟鎹€傚浜庢牸寮忓寲鏁版嵁锛屽湪灏?BPF 杩唬鍣ㄥ浐瀹氾紙pin锛夊埌 bpffs 鎸傝浇鐐瑰悗锛屽彲浠ヤ娇鐢?`cat <path>` 鎵撳嵃缁撴灉锛岀被浼间簬 ``cat /proc/net/netlink``銆備箣鍚庝娇鐢?`rm -f <path>` 绉婚櫎琚浐瀹氱殑杩唬鍣ㄣ€?
渚嬪锛屼綘鍙互浣跨敤浠ヤ笅鍛戒护浠?`bpf_iter_ipv6_route.o` 鐩爣鏂囦欢鍒涘缓涓€涓?BPF 杩唬鍣紝骞跺皢鍏跺浐瀹氬埌 `/sys/fs/bpf/my_route` 璺緞锛?
```

  $ bpftool iter pin ./bpf_iter_ipv6_route.o  /sys/fs/bpf/my_route

```
鐒跺悗浣跨敤浠ヤ笅鍛戒护鎵撳嵃缁撴灉锛?
```

  $ cat /sys/fs/bpf/my_route


```
### 涓?BPF 杩唬鍣ㄧ▼搴忕被鍨嬪疄鐜板唴鏍告敮鎸?

瑕佸湪鍐呮牳涓疄鐜?BPF 杩唬鍣紝寮€鍙戣€呭繀椤诲 `bpf.h <https://git.kernel.org/pub/scm/linux/kernel/git/bpf/bpf-next.git/tree/include/linux/bpf.h>`_ 鏂囦欢涓畾涔夌殑涓嬪垪鍏抽敭鏁版嵁缁撴瀯鍋氫竴娆℃€х殑淇敼銆?
```

  struct bpf_iter_reg {
            const char *target;
            bpf_iter_attach_target_t attach_target;
            bpf_iter_detach_target_t detach_target;
            bpf_iter_show_fdinfo_t show_fdinfo;
            bpf_iter_fill_link_info_t fill_link_info;
            bpf_iter_get_func_proto_t get_func_proto;
            u32 ctx_arg_info_size;
            u32 feature;
            struct bpf_ctx_arg_aux ctx_arg_info[BPF_ITER_CTX_ARG_MAX];
            const struct bpf_iter_seq_info *seq_info;
  };

```
濉啓瀹屾暟鎹粨鏋勫瓧娈靛悗锛岃皟鐢?`bpf_iter_reg_target()` 灏嗚杩唬鍣ㄦ敞鍐屽埌涓?BPF 杩唬鍣ㄥ瓙绯荤粺銆?
浠ヤ笅鏄 struct `bpf_iter_reg` 鍚勫瓧娈电殑璇存槑銆?
   :widths: 25 50
   :header-rows: 1

   - - Fields
     - Description
   - - target
     - 鎸囧畾 BPF 杩唬鍣ㄧ殑鍚嶇О銆備緥濡傦細`bpf_map`銆乣bpf_map_elem`銆傝鍚嶇О搴斾笉鍚屼簬鍐呮牳涓叾浠?`bpf_iter` 鐩爣鍚嶇О銆?   - - attach_target and detach_target
     - 鍏佽鐩爣鐗瑰畾鐨?`link_create` 鎿嶄綔锛屽洜涓烘煇浜涚洰鏍囧彲鑳介渶瑕佺壒娈婂鐞嗐€傚湪鐢ㄦ埛绌洪棿 link_create 闃舵琚皟鐢ㄣ€?   - - show_fdinfo and fill_link_info
     - 褰撶敤鎴疯瘯鍥捐幏鍙栦笌杩唬鍣ㄥ叧鑱旂殑 link 淇℃伅鏃讹紝琚皟鐢ㄤ互濉厖鐩爣鐗瑰畾淇℃伅銆?   - - get_func_proto
     - 鍏佽 BPF 杩唬鍣ㄨ闂壒瀹氫簬璇ヨ凯浠ｅ櫒鐨?BPF 杈呭姪鍑芥暟銆?   - - ctx_arg_info_size and ctx_arg_info
     - 鎸囧畾涓?bpf 杩唬鍣ㄥ叧鑱旂殑 BPF 绋嬪簭鍙傛暟鐨勯獙璇佸櫒鐘舵€併€?   - - feature
     - 鎸囧畾鍐呮牳 BPF 杩唬鍣ㄥ熀纭€璁炬柦涓殑鏌愪簺鎿嶄綔璇锋眰銆傜洰鍓嶄粎鏀寔 BPF_ITER_RESCHED銆傝繖鎰忓懗鐫€浼氳皟鐢ㄥ唴鏍稿嚱鏁?cond_resched() 浠ラ伩鍏嶅叾浠栧唴鏍稿瓙绯荤粺锛堜緥濡?rcu锛夊嚭鐜板紓甯歌涓恒€?   - - seq_info
     - 鎸囧畾鐢ㄤ簬 BPF 杩唬鍣ㄧ殑 seq 鎿嶄綔闆嗗悎锛屼互鍙婄敤浜庡垵濮嬪寲/閲婃斁鐩稿簲 `seq_file` 绉佹湁鏁版嵁鐨勮緟鍔╁嚱鏁般€?
`鐐瑰嚮姝ゅ <https://lore.kernel.org/bpf/20210212183107.50963-2-songliubraving@fb.com/>`_ 鏌ョ湅鍐呮牳涓?`task_vma` BPF 杩唬鍣ㄧ殑瀹炵幇銆?
### 涓?BPF Task 杩唬鍣ㄦ坊鍔犲弬鏁?

榛樿鎯呭喌涓嬶紝BPF 杩唬鍣ㄩ亶鍘嗘暣涓郴缁熶腑鎵€鏈夋寚瀹氱被鍨嬶紙杩涚▼銆乧group銆乵ap 绛夛級鐨勫璞★紝浠ヨ鍙栫浉鍏崇殑鍐呮牳鏁版嵁銆備絾甯稿父鍙叧蹇冨彲杩唬鍐呮牳瀵硅薄涓緢灏忕殑涓€涓瓙闆嗭紝渚嬪浠呴亶鍘嗘煇涓壒瀹氳繘绋嬪唴鐨?task銆傚洜姝わ紝BPF 杩唬鍣ㄧ▼搴忔敮鎸佸湪闄勫姞鏃剁敱鐢ㄦ埛绌洪棿瀵硅凯浠ｅ櫒绋嬪簭杩涜閰嶇疆锛屼粠鑰屽皢瀵硅薄浠庤凯浠ｄ腑杩囨护鎺夈€?
### BPF Task 杩唬鍣ㄧ▼搴?

浠ヤ笅浠ｇ爜鏄竴涓€氳繃杩唬鍣ㄧ殑 `seq_file` 鎵撳嵃鏂囦欢涓?task 淇℃伅鐨?BPF 杩唬鍣ㄧ▼搴忋€傚畠鏄竴涓爣鍑嗙殑 BPF 杩唬鍣ㄧ▼搴忥紝浼氳闂凯浠ｅ櫒鐨勬瘡涓枃浠躲€傛垜浠◢鍚庡皢鍦ㄧず渚嬩腑浣跨敤杩欎釜 BPF 绋嬪簭銆?
```

  #include <vmlinux.h>
  #include <bpf/bpf_helpers.h>

  char _license[] SEC("license") = "GPL";

  SEC("iter/task_file")
  int dump_task_file(struct bpf_iter__task_file *ctx)
  {
        struct seq_file *seq = ctx->meta->seq;
        struct task_struct *task = ctx->task;
        struct file *file = ctx->file;
        __u32 fd = ctx->fd;
        if (task == NULL || file == NULL)
                return 0;
        if (ctx->meta->seq_num == 0) {
                BPF_SEQ_PRINTF(seq, "    tgid      pid       fd      file\n");
        }
        BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
                        (long)file->f_op);
        return 0;
  }

```
### 鍒涘缓甯﹀弬鏁扮殑鏂囦欢杩唬鍣?

鐜板湪锛岃鎴戜滑鐪嬬湅濡備綍鍒涘缓涓€涓粎鍖呭惈鏌愪釜杩涚▼鏂囦欢鐨勮凯浠ｅ櫒銆?
棣栧厛锛屾寜濡備笅鎵€绀哄～鍐?`bpf_iter_attach_opts` 缁撴瀯浣擄細

```

  LIBBPF_OPTS(bpf_iter_attach_opts, opts);
  union bpf_iter_link_info linfo;
  memset(&linfo, 0, sizeof(linfo));
  linfo.task.pid = getpid();
  opts.link_info = &linfo;
  opts.link_info_len = sizeof(linfo);

```
`linfo.task.pid` 鑻ラ潪闆讹紝鍒欐寚绀哄唴鏍稿垱寤轰竴涓粎鍖呭惈鍏锋湁鎸囧畾 `pid` 鐨勮繘绋嬫墍鎵撳紑鏂囦欢鐨勮凯浠ｅ櫒銆傚湪鏈緥涓紝鎴戜滑灏嗗彧閬嶅巻鑷繁杩涚▼鐨勬枃浠躲€傚鏋?`linfo.task.pid` 涓洪浂锛岃凯浠ｅ櫒灏嗚闂瘡涓繘绋嬬殑姣忎釜宸叉墦寮€鏂囦欢銆傜被浼煎湴锛宍linfo.task.tid` 鎸囩ず鍐呮牳鍒涘缓涓€涓闂煇涓壒瀹氱嚎绋嬶紙鑰岄潪杩涚▼锛夊凡鎵撳紑鏂囦欢鐨勮凯浠ｅ櫒銆傛湰渚嬩腑锛宍linfo.task.tid` 浠呭湪鏌愪釜绾跨▼鎷ユ湁鐙珛鐨勬枃浠舵弿杩扮琛ㄦ椂鎵嶄笌 `linfo.task.pid` 涓嶅悓銆傚湪澶у鏁版儏鍐典笅锛岃繘绋嬬殑鎵€鏈夌嚎绋嬪叡浜悓涓€涓枃浠舵弿杩扮琛ㄣ€?
鐜板湪锛屽湪鐢ㄦ埛绌洪棿绋嬪簭涓紝灏嗚缁撴瀯浣撶殑鎸囬拡浼犵粰 `bpf_program__attach_iter()`銆?
```

  link = bpf_program__attach_iter(prog, &opts);
  iter_fd = bpf_iter_create(bpf_link__fd(link));

```
濡傛灉 **tid** 涓?**pid** 閮戒负闆讹紝鍒欎粠璇?`bpf_iter_attach_opts` 缁撴瀯浣撳垱寤虹殑杩唬鍣ㄥ皢鍖呭惈绯荤粺涓紙瀹為檯涓婃槸鍛藉悕绌洪棿鍐咃級姣忎釜 task 鐨勬瘡涓凡鎵撳紑鏂囦欢銆傝繖绛夊悓浜庡悜 `bpf_program__attach_iter()` 浼犲叆 NULL 浣滀负绗簩涓弬鏁般€?
鏁翠釜绋嬪簭濡備笅鎵€绀猴細

```

  #include <stdio.h>
  #include <unistd.h>
  #include <bpf/bpf.h>
  #include <bpf/libbpf.h>
  #include "bpf_iter_task_ex.skel.h"

  static int do_read_opts(struct bpf_program *prog, struct bpf_iter_attach_opts *opts)
  {
        struct bpf_link *link;
        char buf[16] = {};
        int iter_fd = -1, len;
        int ret = 0;

        link = bpf_program__attach_iter(prog, opts);
        if (!link) {
                fprintf(stderr, "bpf_program__attach_iter() fails\n");
                return -1;
        }
        iter_fd = bpf_iter_create(bpf_link__fd(link));
        if (iter_fd < 0) {
                fprintf(stderr, "bpf_iter_create() fails\n");
                ret = -1;
                goto free_link;
        }
        /* not check contents, but ensure read() ends without error */
        while ((len = read(iter_fd, buf, sizeof(buf) - 1)) > 0) {
                buf[len] = 0;
                printf("%s", buf);
        }
        printf("\n");
  free_link:
        if (iter_fd >= 0)
                close(iter_fd);
        bpf_link__destroy(link);
        return 0;
  }

  static void test_task_file(void)
  {
        LIBBPF_OPTS(bpf_iter_attach_opts, opts);
        struct bpf_iter_task_ex *skel;
        union bpf_iter_link_info linfo;
        skel = bpf_iter_task_ex__open_and_load();
        if (skel == NULL)
                return;
        memset(&linfo, 0, sizeof(linfo));
        linfo.task.pid = getpid();
        opts.link_info = &linfo;
        opts.link_info_len = sizeof(linfo);
        printf("PID %d\n", getpid());
        do_read_opts(skel->progs.dump_task_file, &opts);
        bpf_iter_task_ex__destroy(skel);
  }

  int main(int argc, const char * const * argv)
  {
        test_task_file();
        return 0;
  }

```
浠ヤ笅鏄绋嬪簭鐨勮緭鍑恒€?
```

  PID 1859

     tgid      pid       fd      file
     1859     1859        0 ffffffff82270aa0
     1859     1859        1 ffffffff82270aa0
     1859     1859        2 ffffffff82270aa0
     1859     1859        3 ffffffff82272980
     1859     1859        4 ffffffff8225e120
     1859     1859        5 ffffffff82255120
     1859     1859        6 ffffffff82254f00
     1859     1859        7 ffffffff82254d80
     1859     1859        8 ffffffff8225abe0

```
### 涓嶅甫鍙傛暟


璁╂垜浠湅鐪嬩笉甯﹀弬鏁扮殑 BPF 杩唬鍣ㄥ浣曡烦杩囩郴缁熶腑鍏朵粬杩涚▼鐨勬枃浠躲€傚湪杩欑鎯呭喌涓嬶紝BPF 绋嬪簭蹇呴』妫€鏌?task 鐨?pid 鎴?tid锛屽惁鍒欏畠灏嗘帴鏀跺埌绯荤粺涓紙瀹為檯涓婃槸褰撳墠 **pid** 鍛藉悕绌洪棿鍐咃級姣忎釜宸叉墦寮€鐨勬枃浠躲€傚洜姝わ紝鎴戜滑閫氬父浼氬湪 BPF 绋嬪簭涓坊鍔犱竴涓叏灞€鍙橀噺锛屽皢 **pid** 浼犻€掔粰 BPF 绋嬪簭銆?
BPF 绋嬪簭濡備笅鎵€绀恒€?
```

    ......
    int target_pid = 0;

    SEC("iter/task_file")
    int dump_task_file(struct bpf_iter__task_file *ctx)
    {
          ......
          if (task->tgid != target_pid) /* Check task->pid instead to check thread IDs */
                  return 0;
          BPF_SEQ_PRINTF(seq, "%8d %8d %8d %lx\n", task->tgid, task->pid, fd,
                          (long)file->f_op);
          return 0;
    }

```
鐢ㄦ埛绌洪棿绋嬪簭濡備笅鎵€绀猴細

```

    ......
    static void test_task_file(void)
    {
          ......
          skel = bpf_iter_task_ex__open_and_load();
          if (skel == NULL)
                  return;
          skel->bss->target_pid = getpid(); /* process ID.  For thread id, use gettid() */
          memset(&linfo, 0, sizeof(linfo));
          linfo.task.pid = getpid();
          opts.link_info = &linfo;
          opts.link_info_len = sizeof(linfo);
          ......
    }

```
`target_pid` 鏄?BPF 绋嬪簭涓殑鍏ㄥ眬鍙橀噺銆傜敤鎴风┖闂寸▼搴忓簲灏嗚鍙橀噺鍒濆鍖栦负涓€涓繘绋?ID锛屼互璺宠繃 BPF 绋嬪簭涓叾浠栬繘绋嬬殑宸叉墦寮€鏂囦欢銆傚綋浣犱负 BPF 杩唬鍣ㄦ坊鍔犲弬鏁版椂锛岃凯浠ｅ櫒璋冪敤 BPF 绋嬪簭鐨勬鏁颁細鍑忓皯锛屼粠鑰屽彲浠ヨ妭鐪佸ぇ閲忚祫婧愩€?
### 涓?VMA 杩唬鍣ㄦ坊鍔犲弬鏁?

榛樿鎯呭喌涓嬶紝BPF VMA 杩唬鍣ㄥ寘鍚瘡涓繘绋嬬殑姣忎釜 VMA銆備笉杩囷紝浣犱粛鐒跺彲浠ユ寚瀹氫竴涓繘绋嬫垨绾跨▼锛屼互浠呭寘鍚叾 VMA銆備笌鏂囦欢涓嶅悓锛岀嚎绋嬩笉鑳芥嫢鏈夌嫭绔嬬殑鍦板潃绌洪棿锛堣嚜 Linux 2.6.0-test6 璧凤級銆傚湪杩欓噷锛屼娇鐢?**tid** 涓庝娇鐢?**pid** 娌℃湁鍖哄埆銆?
### 涓?Task 杩唬鍣ㄦ坊鍔犲弬鏁?

甯?**pid** 鐨?BPF task 杩唬鍣ㄥ寘鍚煇涓繘绋嬬殑鎵€鏈?task锛堢嚎绋嬶級銆侭PF 绋嬪簭浼氶€愪釜鎺ユ敹杩欎簺 task銆備綘鍙互鎸囧畾甯?**tid** 鍙傛暟鐨?BPF task 杩唬鍣紝浠ヤ粎鍖呭惈涓庣粰瀹?**tid** 鍖归厤鐨?task銆?