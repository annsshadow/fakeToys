## 闃叉閿佺殑缂犵粫/绛夊緟锛圵ound/Wait锛変簰鏂ラ攣璁捐


璇峰厛闃呰 mutex-design.rst锛屽洜涓哄畠鍚屾牱閫傜敤浜庣瓑寰?缂犵粫锛坵ait/wound锛変簰鏂ラ攣銆?
### WW-Mutex 鐨勫姩鏈?
GPU 鎵ц鐨勬搷浣滈€氬父娑夊強璁稿缂撳啿鍖恒€傝繖浜涚紦鍐插尯鍙互鍦ㄤ笉鍚岀殑涓婁笅鏂?杩涚▼涔嬮棿鍏变韩锛屽瓨鍦ㄤ簬涓嶅悓鐨勫唴瀛樺煙锛堜緥濡?VRAM 涓庣郴缁熷唴瀛橈級涓紝绛夌瓑銆傚€熷姪 PRIME / dmabuf锛屽畠浠敋鑷冲彲浠ュ湪璁惧涔嬮棿鍏变韩銆傚洜姝ゅ瓨鍦ㄨ嫢骞叉儏鍐碉紝椹卞姩闇€瑕佺瓑寰呯紦鍐插尯灏辩华銆傚鏋滀綘浠庣瓑寰呬竴涓紦鍐插尯浜掓枼閿佸彉涓哄彲鐢ㄨ繖涓搴︽潵鑰冭檻锛岃繖灏卞甫鏉ヤ簡涓€涓棶棰橈細鍥犱负鏃犳硶淇濊瘉缂撳啿鍖哄湪鎵€鏈変笂涓嬫枃涓互鐩稿悓鐨勯『搴忓嚭鐜板湪 execbuf/batch 涓€傝繖瀹屽叏鐢辩敤鎴风┖闂存帶鍒讹紝鏄簲鐢ㄧ▼搴忔墍鍙戝嚭鐨?GL 璋冪敤搴忓垪鐨勭粨鏋溿€傝繖灏嗗鑷存綔鍦ㄧ殑姝婚攣銆傚綋浣犺€冭檻鍒板唴鏍稿彲鑳介渶瑕佸湪 GPU 鎿嶄綔缂撳啿鍖轰箣鍓嶏紝灏嗙紦鍐插尯杩佺Щ锛坢igrate锛夊埌 VRAM锛岃€岃繖鍙嶈繃鏉ュ張鍙兘闇€瑕侀┍閫愶紙evict锛夊叾浠栦竴浜涚紦鍐插尯锛堣€屼綘涓嶆兂椹遍€愰偅浜涘凡缁忔帓闃熺瓑寰?GPU 鐨勫叾浠栫紦鍐插尯锛夋椂锛岄棶棰樺彉寰楁洿鍔犲鏉傦紱涓嶈繃涓轰簡瀵归棶棰樻湁绠€鍖栫殑鐞嗚В锛屼綘鍙互蹇界暐杩欎竴鐐广€?
TTM 鍥惧舰瀛愮郴缁熶负澶勭悊杩欎釜闂鑰屾彁鍑虹殑绠楁硶鐩稿綋绠€鍗曘€傚浜庢瘡涓€缁勯渶瑕佸姞閿佺殑缂撳啿鍖猴紙execbuf锛夛紝璋冪敤鏂逛細浠庝竴涓叏灞€璁℃暟鍣ㄨ幏寰椾竴涓敮涓€鐨勪繚鐣?ID/绁ㄦ嵁锛坮eservation id/ticket锛夈€傚鏋滃湪閿佸畾涓庢煇涓?execbuf 鍏宠仈鐨勬墍鏈夌紦鍐插尯鏃跺彂鐢熸閿侊紝鍒欎繚鐣欑エ鎹渶灏忥紙鍗虫渶鏃х殑浠诲姟锛夌殑閭ｄ釜鑾疯儨锛岃€屼繚鐣?ID 杈冨ぇ锛堝嵆杈冨勾杞荤殑浠诲姟锛夌殑閭ｄ釜锛屼細瑙ｉ攣瀹冨凡缁忛攣瀹氱殑鎵€鏈夌紦鍐插尯锛岀劧鍚庨噸璇曘€?
鍦?RDBMS 鏂囩尞涓紝淇濈暀绁ㄦ嵁涓庝竴涓簨鍔★紙transaction锛夌浉鍏宠仈銆傝€屾閿佸鐞嗘柟娉曡绉颁负 Wait-Die锛堢瓑寰?姝讳骸锛夈€傝鍚嶇О鍩轰簬涓€涓姞閿佺嚎绋嬪湪閬囧埌宸茶閿佸畾鐨勪簰鏂ラ攣鏃舵墍閲囧彇鐨勮鍔ㄣ€傚鏋滄寔鏈夐攣鐨勪簨鍔℃洿骞磋交锛屽垯鍔犻攣浜嬪姟绛夊緟锛坵aits锛夈€傚鏋滄寔鏈夐攣鐨勪簨鍔℃洿骞撮暱锛屽垯鍔犻攣浜嬪姟閫€璁╋紙backs off锛夊苟娑堜骸锛坉ies锛夈€傚洜姝ょО涓?Wait-Die銆傝繕鏈夊彟涓€绉嶇畻娉曠О涓?Wound-Wait锛堢紶缁?绛夊緟锛夛細濡傛灉鎸佹湁閿佺殑浜嬪姟鏇村勾杞伙紝鍒欏姞閿佷簨鍔＄紶缁曪紙wounds锛夋寔鏈夐攣鐨勪簨鍔★紝璇锋眰鍏舵秷浜°€傚鏋滄寔鏈夐攣鐨勪簨鍔℃洿骞撮暱锛屽垯瀹冪瓑寰呭彟涓€涓簨鍔°€傚洜姝ょО涓?Wound-Wait銆傝繖涓ょ绠楁硶閮芥槸鍏钩鐨勶紝鍥犱负浜嬪姟鏈€缁堥兘浼氭垚鍔熴€傜劧鑰岋紝閫氬父璁や负 Wound-Wait 绠楁硶鐩告瘮 Wait-Die 浜х敓鐨勯€€璁╂鏁版洿灏戯紝浣嗗彟涓€鏂归潰锛屽湪浠庨€€璁╀腑鎭㈠鏃讹紝瀹冧即闅忕潃姣?Wait-Die 鏇村鐨勫伐浣溿€俉ound-Wait 涔熸槸涓€绉嶆姠鍗犲紡锛坧reemptive锛夌畻娉曪紝鍥犱负浜嬪姟浼氳鍏朵粬浜嬪姟缂犵粫锛岃繖闇€瑕佷竴涓彲闈犵殑鏂瑰紡鏉ヨ幏鍙栬缂犵粫鐨勬潯浠跺苟鎶㈠崰姝ｅ湪杩愯鐨勪簨鍔°€傛敞鎰忥紝杩欎笌杩涚▼鎶㈠崰锛坧rocess preemption锛変笉鏄竴鍥炰簨銆俉ound-Wait 浜嬪姟鍦ㄥ畠鍥犺缂犵粫鑰屾秷浜★紙杩斿洖 -EDEADLK锛夋椂锛岃瑙嗕负琚姠鍗犮€?
### 姒傚康


鐩告瘮浜庢櫘閫氫簰鏂ラ攣锛寃/w 浜掓枼閿佺殑閿佹帴鍙ｄ腑鍑虹幇浜嗕袱涓澶栫殑姒傚康/瀵硅薄锛?
鑾峰彇涓婁笅鏂囷紙Acquire context锛夛細涓轰簡纭繚鏈€缁堣兘澶熷悜鍓嶆帹杩涳紝灏濊瘯鑾峰彇閿佺殑浠诲姟涓嶈鍘昏幏鍙栦竴涓柊鐨勪繚鐣?ID 鏄緢閲嶈鐨勶紝鑰屾槸淇濈暀瀹冨湪寮€濮嬭幏鍙栭攣鏃舵墍鑾峰緱鐨勯偅涓€傝绁ㄦ嵁瀛樺偍鍦ㄨ幏鍙栦笂涓嬫枃涓€傛澶栵紝鑾峰彇涓婁笅鏂囪繕璺熻釜璋冭瘯鐘舵€侊紝浠ユ崟鑾峰 w/w 浜掓枼閿佹帴鍙ｇ殑婊ョ敤銆備竴涓幏鍙栦笂涓嬫枃琛ㄧず涓€涓簨鍔°€?
w/w 绫伙紙w/w class锛夛細涓庢櫘閫氫簰鏂ラ攣涓嶅悓锛岄攣绫诲浜?w/w 浜掓枼閿佸繀椤绘槸鏄惧紡鐨勶紝鍥犱负鍒濆鍖栬幏鍙栦笂涓嬫枃闇€瑕佺敤鍒板畠銆傞攣绫昏繕鎸囧畾浜嗚浣跨敤鍝绠楁硶鈥斺€擶ound-Wait 杩樻槸 Wait-Die銆?
姝ゅ杩樻湁涓夌被涓嶅悓鐨?w/w 閿佽幏鍙栧嚱鏁帮細

- 浣跨敤涓婁笅鏂囪繘琛屾甯搁攣鑾峰彇锛屼娇鐢?ww_mutex_lock銆?
- 鍦ㄤ簤鐢ㄧ殑閿佷笂杩涜鎱㈤€熻矾寰勶紙slowpath锛夐攣鑾峰彇锛岀敱鍒氬垰鏉€鎺夊叾浜嬪姟銆佸苟宸蹭涪寮冩墍鏈夊凡鑾峰彇閿佺殑浠诲姟浣跨敤銆傝繖浜涘嚱鏁板甫鏈?_slow 鍚庣紑銆?
  浠庣畝鍗曠殑璇箟瑙掑害鏉ョ湅锛宊slow 鍑芥暟骞堕潪涓ユ牸蹇呴渶锛屽洜涓哄湪浜夌敤鐨勯攣涓婄畝鍗曞湴璋冪敤姝ｅ父鐨?ww_mutex_lock 鍑芥暟锛堝湪涓㈠純鎵€鏈夊叾浠栧凡鑾峰彇鐨勯攣涔嬪悗锛変篃鑳芥纭伐浣溿€傛瘯绔燂紝濡傛灉灏氭湭鑾峰彇浠讳綍鍏朵粬 w/w 浜掓枼閿侊紝灏变笉瀛樺湪姝婚攣鐨勫彲鑳斤紝鍥犳 ww_mutex_lock 璋冪敤浼氶樆濉烇紝鑰屼笉浼氭彁鍓嶈繑鍥?-EDEADLK銆俖slow 鍑芥暟鐨勪紭鍔垮湪浜庢帴鍙ｅ畨鍏ㄦ€э細

  - ww_mutex_lock 鍏锋湁 __must_check int 杩斿洖绫诲瀷锛岃€?ww_mutex_lock_slow 鍏锋湁 void 杩斿洖绫诲瀷銆傛敞鎰忥紝鐢变簬 w/w 浜掓枼閿佷唬鐮佹棤璁哄浣曢兘闇€瑕佸惊鐜?閲嶈瘯锛宊_must_check 涓嶄細瀵艰嚧铏氬亣鐨勮鍛婏紝鍗充娇绗竴娆￠攣鎿嶄綔缁濅笉浼氬け璐ャ€?  - 褰撳惎鐢ㄥ畬鏁磋皟璇曟椂锛寃w_mutex_lock_slow 浼氭鏌ユ墍鏈夊凡鑾峰彇鐨?w/w 浜掓枼閿侀兘宸茶閲婃斁锛堜互闃叉姝婚攣锛夛紝骞剁‘淇濇垜浠樆濉炲湪浜夌敤鐨勯攣涓婏紙浠ラ槻姝㈠湪浜夌敤鐨勯攣鍙鑾峰彇涔嬪墠锛岄€氳繃 -EDEADLK 鎱㈤€熻矾寰勮嚜鏃嬶級銆?
- 鍙幏鍙栧崟涓?w/w 浜掓枼閿佺殑鍑芥暟锛屽叾璇箟涓庢櫘閫氫簰鏂ラ攣瀹屽叏鐩稿悓銆傝繖鏄€氳繃浠?NULL 涓婁笅鏂囪皟鐢?ww_mutex_lock 鏉ュ疄鐜扮殑銆?
  鍚屾牱锛岃繖涔熶笉鏄弗鏍煎繀闇€鐨勩€備絾閫氬父浣犲彧鎯宠幏鍙栧崟涓攣锛岃繖绉嶆儏鍐典笅寤虹珛鑾峰彇涓婁笅鏂囨病鏈夋剰涔夛紙鍥犳涔熸渶濂介伩鍏嶈幏鍙栦竴涓閿侀伩鍏嶇エ鎹級銆?
褰撶劧锛屾墍鏈夌敤浜庡鐞嗗洜淇″彿鑰屽敜閱掔殑甯哥敤鍙樹綋涔熷悓鏍锋彁渚涖€?
### 鐢ㄦ硶


绠楁硶锛圵ait-Die 涓?Wound-Wait锛夋槸閫氳繃浣跨敤 DEFINE_WW_CLASS()锛圵ound-Wait锛夋垨 DEFINE_WD_CLASS()锛圵ait-Die锛夋潵閫夋嫨鐨勩€備綔涓虹矖鐣ョ殑缁忛獙娉曞垯锛屽鏋滀綘棰勬湡鍚屾椂绔炰簤鐨勶紙competing锛変簨鍔℃暟閲忛€氬父杈冨皬锛屽苟涓斿笇鏈涘噺灏戝洖婊氾紙rollback锛夋鏁帮紝鍒欎娇鐢?Wound-Wait銆?
鍦ㄥ悓涓€涓?w/w 绫讳腑鑾峰彇閿佹湁涓夌涓嶅悓鐨勬柟寮忋€傚父瑙佺殑
```

  static DEFINE_WW_CLASS(ww_class);

  struct obj {
	struct ww_mutex lock;
	/* obj data */
  };

  struct obj_entry {
	struct list_head head;
	struct obj *obj;
  };

```
鏂规硶 1锛屼娇鐢?execbuf->buffers 涓竴涓笉鍏佽閲嶆帓搴忕殑鍒楄〃銆傚鏋滀綘宸茬粡鍦ㄦ煇澶勮窡韪簡鎵€闇€瀵硅薄鐨勫垪琛紝杩欎細寰堟湁鐢ㄣ€傛澶栵紝閿佽緟鍔╁嚱鏁板彲浠ュ埄鐢?-EALREADY 杩斿洖鐮佸悜璋冪敤鏂逛紶鎾竴涓俊鍙凤細鏌愪釜瀵硅薄鍦ㄥ垪琛ㄤ笂鍑虹幇浜嗕袱娆°€傚鏋滀粠鐢ㄦ埛绌洪棿杈撳叆鏋勫缓鍒楄〃锛屽苟涓?ABI 瑕佹眰鐢ㄦ埛绌洪棿
```

  int lock_objs(struct list_head *list, struct ww_acquire_ctx *ctx)
  {
	struct obj *res_obj = NULL;
	struct obj_entry *contended_entry = NULL;
	struct obj_entry *entry;

	ww_acquire_init(ctx, &ww_class);

  retry:
	list_for_each_entry (entry, list, head) {
		if (entry->obj == res_obj) {
			res_obj = NULL;
			continue;
		}
		ret = ww_mutex_lock(&entry->obj->lock, ctx);
		if (ret < 0) {
			contended_entry = entry;
			goto err;
		}
	}

	ww_acquire_done(ctx);
	return 0;

  err:
	list_for_each_entry_continue_reverse (entry, list, head)
		ww_mutex_unlock(&entry->obj->lock);

	if (res_obj)
		ww_mutex_unlock(&res_obj->lock);

	if (ret == -EDEADLK) {
		/* we lost out in a seqno race, lock and retry.. */
		ww_mutex_lock_slow(&contended_entry->obj->lock, ctx);
		res_obj = contended_entry->obj;
		goto retry;
	}
	ww_acquire_fini(ctx);

	return ret;
  }

```
鏂规硶 2锛屼娇鐢?execbuf->buffers 涓竴涓彲浠ラ噸鎺掑簭鐨勫垪琛ㄣ€備笌鏂规硶 1 涓€鏍凤紝浣跨敤 -EALREADY 杩涜閲嶅鏉＄洰妫€娴嬬殑璇箟鐩稿悓銆備絾鏄?```

  int lock_objs(struct list_head *list, struct ww_acquire_ctx *ctx)
  {
	struct obj_entry *entry, *entry2;

	ww_acquire_init(ctx, &ww_class);

	list_for_each_entry (entry, list, head) {
		ret = ww_mutex_lock(&entry->obj->lock, ctx);
		if (ret < 0) {
			entry2 = entry;

			list_for_each_entry_continue_reverse (entry2, list, head)
				ww_mutex_unlock(&entry2->obj->lock);

			if (ret != -EDEADLK) {
				ww_acquire_fini(ctx);
				return ret;
			}

			/* we lost out in a seqno race, lock and retry.. */
			ww_mutex_lock_slow(&entry->obj->lock, ctx);

			/*
			 * Move buf to head of the list, this will point
			 * buf->next to the first unlocked entry,
			 * restarting the for loop.
			 */
			list_del(&entry->head);
			list_add(&entry->head, list);
		}
	}

	ww_acquire_done(ctx);
	return 0;
  }

```
```

  void unlock_objs(struct list_head *list, struct ww_acquire_ctx *ctx)
  {
	struct obj_entry *entry;

	list_for_each_entry (entry, list, head)
		ww_mutex_unlock(&entry->obj->lock);

	ww_acquire_fini(ctx);
  }

```
鏂规硶 3 鍦ㄥ璞″垪琛ㄦ槸涓存椂锛坅d-hoc锛夋瀯寤鸿€岄潪棰勫厛鏋勫缓鏃跺緢鏈夌敤锛屼緥濡傚綋璋冩暣涓€涓浘涓殑杈规椂锛屽叾涓瘡涓妭鐐归兘鏈夊畠鑷繁鐨?ww_mutex 閿侊紝骞朵笖杈瑰彧鏈夊湪鎸佹湁鎵€鏈夌浉鍏宠妭鐐圭殑閿佹椂鎵嶈兘鏇存敼銆倃/w 浜掓枼閿佸ぉ鐒堕€傚悎杩欑鎯呭喌锛屽師鍥犳湁浜岋細

- 瀹冧滑鑳戒互浠绘剰椤哄簭澶勭悊閿佽幏鍙栵紝杩欎娇鎴戜滑鑳藉浠庝竴涓捣鐐瑰紑濮嬮亶鍘嗗浘锛岀劧鍚庤凯浠ｅ湴鍙戠幇鏂扮殑杈癸紝骞堕攣瀹氳繖浜涜竟鎵€杩炴帴鐨勮妭鐐广€?- 鐢变簬 -EALREADY 杩斿洖鐮佽〃绀烘煇涓粰瀹氬璞″凡琚寔鏈夛紝鍥犳鏃犻渶棰濆鐨勭翱璁帮紙book-keeping锛夋潵鎵撶牬鍥句腑鐨勭幆锛屼篃鏃犻渶璺熻釜鍝簺閿佸凡琚寔鏈夛紙褰撲娇鐢ㄥ涓妭鐐逛綔涓鸿捣鐐规椂锛夈€?
娉ㄦ剰锛岃繖绉嶆柟娉曚笌涓婅堪鏂规硶鍦ㄤ袱涓噸瑕佹柟闈㈡湁鎵€涓嶅悓锛?
- 鐢变簬瀵硅薄鍒楄〃鏄姩鎬佹瀯寤虹殑锛堝苟涓斿湪鍥犵鍒?-EDEADLK 娑堜骸鏉′欢鑰岄噸璇曟椂寰堝彲鑳戒笉鍚岋級锛屽綋鏌愪釜瀵硅薄鏈閿佸畾鏃讹紝娌℃湁蹇呰灏嗗叾淇濈暀鍦ㄦ寔涔呭垪琛ㄤ腑銆傚洜姝ゆ垜浠彲浠ュ皢 list_head 绉诲叆瀵硅薄鑷韩涓€?- 鍙︿竴鏂归潰锛屽姩鎬佸璞″垪琛ㄦ瀯寤轰篃鎰忓懗鐫€ -EALREADY 杩斿洖鐮佹棤娉曡浼犳挱銆?
杩樿娉ㄦ剰锛屾柟娉?#1 鍜屾柟娉?#2 浠ュ強鏂规硶 #3 鍙互缁勫悎浣跨敤锛屼緥濡傞鍏堜娇鐢ㄤ笂杩版煇涓€绉嶆柟娉曢攣瀹氫竴缁勮捣濮嬭妭鐐癸紙浠庣敤鎴风┖闂翠紶鍏ワ級銆傜劧鍚庝娇鐢ㄤ笅闈㈢殑鏂规硶 #3 閿佸畾鍙楁搷浣滃奖鍝嶇殑浠讳綍鍏朵粬瀵硅薄銆傚洖閫€/閲嶈瘯杩囩▼浼氱◢寰鏉備竴浜涳紝鍥犱负褰撳姩鎬侀攣瀹氭楠ょ鍒?-EDEADLK 鏃讹紝鎴戜滑杩橀渶瑕佽В閿佺敤鍥哄畾鍒楄〃鑾峰彇鐨勬墍鏈夊璞°€備絾 w/w 浜掓枼閿佺殑璋冭瘯妫€鏌ヤ細鎹曡幏杩欎簺鎯呭喌涓嬬殑浠讳綍鎺ュ彛璇敤銆?
姝ゅ锛屾柟娉?3 涓嶄細浣块攣鑾峰彇姝ラ澶辫触锛屽洜涓哄畠涓嶈繑鍥?-EALREADY銆傚綋鐒讹紝褰撲娇鐢?_interruptible 鏃朵細鏈夋墍涓嶅悓
```

  struct obj {
	struct ww_mutex ww_mutex;
	struct list_head locked_list;
  };

  static DEFINE_WW_CLASS(ww_class);

  void __unlock_objs(struct list_head *list)
  {
	struct obj *entry, *temp;

	list_for_each_entry_safe (entry, temp, list, locked_list) {
		/* need to do that before unlocking, since only the current lock holder is
		allowed to use object */
		list_del(&entry->locked_list);
		ww_mutex_unlock(entry->ww_mutex)
	}
  }

  void lock_objs(struct list_head *list, struct ww_acquire_ctx *ctx)
  {
	struct obj *obj;

	ww_acquire_init(ctx, &ww_class);

  retry:
	/* re-init loop start state */
	loop {
		/* magic code which walks over a graph and decides which objects
		 * to lock */

		ret = ww_mutex_lock(obj->ww_mutex, ctx);
		if (ret == -EALREADY) {
			/* we have that one already, get to the next object */
			continue;
		}
		if (ret == -EDEADLK) {
			__unlock_objs(list);

			ww_mutex_lock_slow(obj, ctx);
			list_add(&entry->locked_list, list);
			goto retry;
		}

		/* locked a new object, add it to the list */
		list_add_tail(&entry->locked_list, list);
	}

	ww_acquire_done(ctx);
	return 0;
  }

  void unlock_objs(struct list_head *list, struct ww_acquire_ctx *ctx)
  {
	__unlock_objs(list);
	ww_acquire_fini(ctx);
  }

```
鏂规硶 4锛氬彧閿佷竴涓璞°€傚湪杩欑鎯呭喌涓嬶紝姝婚攣妫€娴嬩笌棰勯槻鏄剧劧鏈変簺杩囧害锛屽洜涓哄彧鑾峰彇涓€涓攣鏃讹紝涓嶅彲鑳藉湪鍚屼竴涓被鍐呬骇鐢熸閿併€備负浜嗙畝鍖栬繖绉嶆儏鍐碉紝w/w 浜掓枼閿?API 鍙互涓?NULL 涓婁笅鏂囦竴璧蜂娇鐢ㄣ€?
### 瀹炵幇缁嗚妭


##### 璁捐锛?

  ww_mutex 鐩墠灏佽浜嗕竴涓?struct mutex锛岃繖鎰忓懗鐫€瀵规櫘閫氫簰鏂ラ攣鍔犻攣娌℃湁棰濆寮€閿€锛岃€屾櫘閫氬姞閿佽甯歌寰楀銆傚洜姝ゅ鏋滀笉浣跨敤绛夊緟/缂犵粫浜掓枼閿侊紝浠ｇ爜澶у皬鍙細鏈夊緢灏忕殑澧炲姞銆?
  鎴戜滑涓虹瓑寰呭垪琛紙wait list锛夌淮鎶や互涓嬩笉鍙橀噺锛?
  (1) 甯︽湁鑾峰彇涓婁笅鏂囩殑绛夊緟鑰呮寜 stamp 椤哄簭鎺掑簭锛涗笉甯﹁幏鍙栦笂涓嬫枃鐨勭瓑寰呰€呮寜 FIFO 椤哄簭绌挎彃鍏朵腑銆?  (2) 瀵逛簬 Wait-Die锛屽湪甯︽湁涓婁笅鏂囩殑绛夊緟鑰呬腑锛屽彧鏈夌涓€涓彲浠ュ凡缁忚幏鍙栦簡鍏朵粬閿侊紙ctx->acquired > 0锛夈€傛敞鎰忥紝杩欎釜绛夊緟鑰呭彲鑳藉湪鍒楄〃涓帓鍦ㄥ叾浠栦笉甯︿笂涓嬫枃鐨勭瓑寰呰€呬箣鍚庛€?
  Wound-Wait 鐨勬姠鍗犳槸閫氳繃涓€绉嶆儼鎬ф姠鍗狅紙lazy-preemption锛夋柟妗堝疄鐜扮殑锛氫粎鍦ㄥ嚭鐜板鏂伴攣鐨勭珵浜夈€佸洜姝ゅ瓨鍦ㄧ湡姝ｇ殑姝婚攣鍙兘鏃讹紝鎵嶄細妫€鏌ヤ簨鍔＄殑琚紶缁曪紙wounded锛夌姸鎬併€傚湪閭ｇ鎯呭喌涓嬶紝濡傛灉浜嬪姟琚紶缁曪紝瀹冨氨浼氶€€璁╋紝娓呴櫎琚紶缁曠姸鎬佸苟閲嶈瘯銆備互杩欑鏂瑰紡瀹炵幇鎶㈠崰鐨勪竴澶уソ澶勬槸锛岃缂犵粫鐨勪簨鍔″彲浠ュ湪閲嶅惎浜嬪姟涔嬪墠锛岃瘑鍒嚭涓€涓绛夊緟鐨勪簤鐢ㄩ攣銆傜洸鐩湴閲嶅惎浜嬪姟寰堝彲鑳戒細浣夸簨鍔℃渶缁堝張闄峰叆闇€瑕佸啀娆￠€€璁╃殑澧冨湴銆?
  涓€鑸潵璇达紝棰勬湡绔炰簤涓嶄細澶銆傝繖浜涢攣閫氬父鐢ㄤ簬搴忓垪鍖栧璁惧璧勬簮鐨勮闂紝鍥犳浼樺寲閲嶇偣搴旀斁鍦ㄦ棤绔炰簤锛坲ncontended锛夌殑鎯呭喌涓娿€?
##### Lockdep锛?

  鎴戜滑鐗瑰埆灏忓績鍦板敖鍙兘澶氬湴璀﹀憡 API 婊ョ敤鐨勬儏鍐点€備竴浜涘父瑙佺殑 API 婊ョ敤浼氳 CONFIG_DEBUG_MUTEXES 鎹曡幏锛屼絾鎺ㄨ崘浣跨敤 CONFIG_PROVE_LOCKING銆?
  浼氳璀﹀憡鐨勪竴浜涢敊璇細
   - 蹇樿璋冪敤 ww_acquire_fini 鎴?ww_acquire_init銆?   - 璇曞浘鍦?ww_acquire_done 涔嬪悗閿佸畾鏇村浜掓枼閿併€?   - 璇曞浘鍦?-EDEADLK 涔嬪悗銆佸苟鍦ㄨВ閿佹墍鏈変簰鏂ラ攣涔嬪墠閿佸畾閿欒鐨勪簰鏂ラ攣銆?   - 璇曞浘鍦?-EDEADLK 涔嬪悗銆佸苟鍦ㄨВ閿佹墍鏈変簰鏂ラ攣涔嬪墠閿佸畾姝ｇ‘鐨勪簰鏂ラ攣銆?
   - 鍦ㄨ繑鍥?-EDEADLK 涔嬪墠璋冪敤 ww_mutex_lock_slow銆?
   - 鐢ㄩ敊璇殑瑙ｉ攣鍑芥暟瑙ｉ攣浜掓枼閿併€?   - 鍦ㄥ悓涓€涓笂涓嬫枃涓婁袱娆¤皟鐢ㄦ煇涓?ww_acquire_* 鍑芥暟銆?   - 瀵逛簰鏂ラ攣浣跨敤浜嗕笌 ww_acquire_ctx 涓嶅悓鐨?ww_class銆?   - 鍙兘瀵艰嚧姝婚攣鐨勬櫘閫?lockdep 閿欒銆?
  鍙兘瀵艰嚧姝婚攣鐨勪竴浜?lockdep 閿欒锛?   - 鍦ㄥ绗竴涓?ww_acquire_ctx 璋冪敤 ww_acquire_fini 涔嬪墠锛岃皟鐢?ww_acquire_init 鏉ュ垵濮嬪寲绗簩涓?ww_acquire_ctx銆?   - 鍙兘鍙戠敓鐨?鏅€?姝婚攣銆?
FIXME:
  涓€鏃︽垜浠疄鐜颁簡 TASK_DEADLOCK 浠诲姟鐘舵€佹爣蹇楃殑榄旀硶锛屽氨鏇存柊鏈妭銆?