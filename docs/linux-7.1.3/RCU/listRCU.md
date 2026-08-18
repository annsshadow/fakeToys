
## 浣跨敤 RCU 淇濇姢浠ヨ涓轰富鐨勯摼琛?
RCU 鏈€甯歌鐨勭敤閫斾箣涓€锛屾槸淇濇姢浠ヨ涓轰富鐨勯摼琛?锛坄list.h` 涓殑 `struct list_head`锛夈€傝繖绉嶆柟娉曠殑涓€澶т紭鍔垮湪浜庯紝
鎵€鏈夊繀闇€鐨勫唴瀛樻帓搴忛兘鐢遍摼琛ㄥ畯鏉ユ彁渚涖€傛湰鏂囨。鎻忚堪浜嗚嫢骞插熀浜庨摼琛ㄧ殑 RCU 鐢ㄤ緥銆?
鍦ㄦ寔鏈?rcu_read_lock() 閬嶅巻閾捐〃鐨勫悓鏃讹紝鍐欒€呭彲浠ヤ慨鏀硅閾捐〃銆傝鑰呬繚璇佽兘鐪嬪埌
浠栦滑鍦ㄨ幏鍙?rcu_read_lock() 涔嬪墠灏辫鍔犲叆閾捐〃銆佸苟涓斿湪閲婃斁 rcu_read_unlock()
鏃朵粛鐒剁暀鍦ㄩ摼琛ㄤ笂鐨勬墍鏈夊厓绱犮€傝鍔犲叆鎴栫Щ鍑洪摼琛ㄧ殑鍏冪礌鍙兘浼氳鐪嬪埌锛屼篃鍙兘涓嶄細琚湅鍒般€?濡傛灉鍐欒€呰皟鐢?list_replace_rcu()锛岃鑰呭彲鑳界湅鍒版棫鍏冪礌锛屼篃鍙兘鐪嬪埌鏂板厓绱狅紱
浣嗘棦涓嶄細鍚屾椂鐪嬪埌涓よ€咃紝涔熶笉浼氫袱鑰呴兘鐪嬩笉鍒般€?

### 绀轰緥 1锛氫互璇讳负涓荤殑閾捐〃锛氬欢杩熼攢姣?
鍐呮牳涓?RCU 閾捐〃涓€涓箍娉涗娇鐢ㄧ殑鍦烘櫙锛屾槸瀵圭郴缁熶腑**鎵€鏈夎繘绋?*杩涜鏃犻攣閬嶅巻銆?`task_struct` 鐨?`tasks` 瀛楁琛ㄧず閾炬帴鎵€鏈夎繘绋嬬殑閾捐〃鑺傜偣銆傝閾捐〃鍙互涓庝换浣曢摼琛?鐨勬坊鍔犳垨鍒犻櫎鎿嶄綔骞惰鍦拌繘琛岄亶鍘嗐€?
閾捐〃鐨勯亶鍘嗛€氳繃 `for_each_process()` 瀹屾垚锛屽叾瀹氫箟濡備笅锛?```

	#define next_task(p) \
		list_entry_rcu((p)->tasks.next, struct task_struct, tasks)

	#define for_each_process(p) \
		for (p = &init_task ; (p = next_task(p)) != &init_task ; )

```
```

	rcu_read_lock();
	for_each_process(p) {
		/* Do something with p */
	}
	rcu_read_unlock();

```
浠庨摼琛ㄤ腑鍒犻櫎杩涚▼鐨勭畝鍖栦笖楂樺害鍐呰仈鐨勪唬鐮佸涓嬶細
```

	void release_task(struct task_struct *p)
	{
		write_lock(&tasklist_lock);
		list_del_rcu(&p->tasks);
		write_unlock(&tasklist_lock);
		call_rcu(&p->rcu, delayed_put_task_struct);
	}

```
褰撹繘绋嬮€€鍑烘椂锛宍release_task()` 浼氬湪 `tasklist_lock` 鍐欒€呴攣鐨勪繚鎶や笅锛?閫氳繃 __exit_signal() 鍜?__unhash_process() 璋冪敤 `list_del_rcu(&p->tasks)`銆?list_del_rcu() 璋冪敤灏嗕换鍔′粠鎵€鏈変换鍔＄殑閾捐〃涓Щ闄ゃ€俙tasklist_lock`
闃叉骞跺彂鐨勯摼琛ㄦ坊鍔?鍒犻櫎鐮村潖閾捐〃銆備娇鐢?`for_each_process()` 鐨勮鑰呭苟涓嶅彈
`tasklist_lock` 淇濇姢銆備负浜嗛槻姝㈣鑰呭療瑙夊埌閾捐〃鎸囬拡鐨勫彉鍖栵紝`task_struct`
瀵硅薄鍙湁鍦ㄧ粡杩囦竴涓垨澶氫釜瀹介檺鏈熶箣鍚庢墠浼氳閲婃斁锛岃繖鏄€熷姪 call_rcu() 瀹炵幇鐨勶紝
鑰?call_rcu() 閫氳繃 put_task_struct_rcu_user() 璋冪敤銆傝繖绉嶉攢姣佺殑寤惰繜淇濊瘉浜?浠讳綍姝ｅ湪閬嶅巻閾捐〃鐨勮鑰呴兘鑳界湅鍒版湁鏁堢殑 `p->tasks.next` 鎸囬拡锛?骞朵笖鍒犻櫎/閲婃斁鍙互涓庨摼琛ㄩ亶鍘嗗苟琛岃繘琛屻€傝繖绉嶆ā寮忎篃琚О涓?*瀛樺湪閿侊紙existence lock锛?*锛?鍥犱负 RCU 浼氫竴鐩存帹杩熻皟鐢?delayed_put_task_struct() 鍥炶皟鍑芥暟锛岀洿鍒版墍鏈夌幇瀛樼殑
璇昏€呴兘瀹屾垚锛屼粠鑰屼繚璇佺浉鍏崇殑 `task_struct` 瀵硅薄浼氫竴鐩村瓨鍦紝鐩村埌鎵€鏈夊彲鑳芥寔鏈?璇ュ璞″紩鐢ㄧ殑 RCU 璇昏€呴兘鎵ц瀹屾瘯銆?

### 绀轰緥 2锛氬湪閿佷箣澶栨墽琛岃渚ф搷浣滐細鏃犲師鍦版洿鏂?
鏌愪簺璇诲啓閿佺敤渚嬪湪鎸佹湁璇讳晶閿佹椂璁＄畻涓€涓€硷紝浣嗗湪閲婃斁璇ラ攣涔嬪悗浠嶇户缁娇鐢ㄨ繖涓€笺€?杩欑被鐢ㄤ緥閫氬父寰堥€傚悎杞崲涓?RCU銆備竴涓吀鍨嬬殑渚嬪瓙鏄綉缁滄暟鎹寘璺敱銆?鐢变簬鏁版嵁鍖呰矾鐢辨暟鎹拷韪殑鏄绠楁満涔嬪鐨勮澶囩姸鎬侊紝瀹冩湁鏃朵細鍖呭惈杩囨湡鏁版嵁銆?鍥犳锛屼竴鏃﹁矾鐢辫绠楀畬姣曪紝鍦ㄦ暟鎹寘浼犺緭鏈熼棿灏辨病鏈夊繀瑕佷繚鎸佽矾鐢辫〃闈欐銆?姣曠珶锛屼綘鍙互闅忓績鎵€娆插湴璁╄矾鐢辫〃闈欐锛屼絾閭ｅ苟涓嶈兘闃绘澶栭儴浜掕仈缃戝彂鐢熷彉鍖栵紝
鑰岀湡姝ｉ噸瑕佺殑鏄閮ㄤ簰鑱旂綉鐨勭姸鎬併€傛澶栵紝璺敱椤归€氬父鏄娣诲姞鎴栧垹闄わ紝
鑰屼笉鏄師鍦颁慨鏀广€傝繖鏄竴涓綍瑙佺殑渚嬪瓙锛屽厜閫熺殑鏈夐檺鎬у拰鍘熷瓙鐨勯潪闆跺昂瀵稿疄闄呬笂
甯姪闄嶄綆浜嗗悓姝ョ殑寮€閿€銆?
杩欑被 RCU 鐢ㄤ緥鐨勪竴涓畝鍗曚緥瀛愬彲浠ュ湪绯荤粺璋冪敤瀹¤鏀寔涓壘鍒般€備緥濡傦紝涓€涓?璇诲啓閿佷繚鎶ょ殑浠ｇ爜濡備笅锛?```

	static enum audit_state audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		read_lock(&auditsc_lock);
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				read_unlock(&auditsc_lock);
				return state;
			}
		}
		read_unlock(&auditsc_lock);
		return AUDIT_BUILD_CONTEXT;
	}

```
杩欓噷閾捐〃鍦ㄩ攣鐨勪繚鎶や笅杩涜鎼滅储锛屼絾鍦ㄨ繑鍥炲搴旂殑鍊间箣鍓嶅氨閲婃斁浜嗛攣銆?绛夊埌杩欎釜鍊艰浣跨敤鏃讹紝閾捐〃寰堝彲鑳藉凡缁忚淇敼銆傝繖鏄悎鐞嗙殑锛屽洜涓哄鏋滀綘姝ｅ湪鍏抽棴瀹¤锛?澶氬璁″嚑涓郴缁熻皟鐢ㄤ篃娌″叧绯汇€?
```

	static enum audit_state audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		rcu_read_lock();
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry_rcu(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				rcu_read_unlock();
				return state;
			}
		}
		rcu_read_unlock();
		return AUDIT_BUILD_CONTEXT;
	}

```
read_lock() 鍜?read_unlock() 璋冪敤鍒嗗埆鍙樻垚浜?rcu_read_lock()
鍜?rcu_read_unlock()锛岃€?list_for_each_entry() 鍙樻垚浜?list_for_each_entry_rcu()銆?*_rcu()** 閾捐〃閬嶅巻鍘熻澧炲姞浜?READ_ONCE()
浠ュ強鐢ㄤ簬妫€娴嬪湪 RCU 璇讳晶涓寸晫鍖轰箣澶栭敊璇娇鐢ㄧ殑璇婃柇妫€鏌ャ€?
鏇存柊渚х殑鏀瑰姩涔熷緢鐩存帴銆傚湪杩欎簺绠€鍖栫殑浠ｇ爜涓紝璇诲啓閿佸彲鑳藉儚涓嬮潰杩欐牱鐢ㄤ簬鍒犻櫎鍜屾彃鍏ワ細
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		write_lock(&auditsc_lock);
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				list_del(&e->list);
				write_unlock(&auditsc_lock);
				return 0;
			}
		}
		write_unlock(&auditsc_lock);
		return -EFAULT;		/* No matching rule */
	}

	static inline int audit_add_rule(struct audit_entry *entry,
					 struct list_head *list)
	{
		write_lock(&auditsc_lock);
		if (entry->rule.flags & AUDIT_PREPEND) {
			entry->rule.flags &= ~AUDIT_PREPEND;
			list_add(&entry->list, list);
		} else {
			list_add_tail(&entry->list, list);
		}
		write_unlock(&auditsc_lock);
		return 0;
	}

```
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		/* No need to use the _rcu iterator here, since this is the only
		 * deletion routine. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				list_del_rcu(&e->list);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

	static inline int audit_add_rule(struct audit_entry *entry,
					 struct list_head *list)
	{
		if (entry->rule.flags & AUDIT_PREPEND) {
			entry->rule.flags &= ~AUDIT_PREPEND;
			list_add_rcu(&entry->list, list);
		} else {
			list_add_tail_rcu(&entry->list, list);
		}
		return 0;
	}

```
閫氬父锛寃rite_lock() 鍜?write_unlock() 浼氳鏇挎崲涓?spin_lock() 鍜?spin_unlock()銆備絾鍦ㄦ湰渚嬩腑锛屾墍鏈夎皟鐢ㄨ€呴兘鎸佹湁 `audit_filter_mutex`锛?鍥犳涓嶉渶瑕侀澶栫殑閿併€備簬鏄?auditsc_lock 鍙互琚Щ闄わ紝鍥犱负浣跨敤 RCU 娑堥櫎浜?鍐欒€呴渶瑕佹帓鏂ヨ鑰呯殑闇€姹傘€?
list_del()銆乴ist_add() 鍜?list_add_tail() 鍘熻琚浛鎹负
list_del_rcu()銆乴ist_add_rcu() 鍜?list_add_tail_rcu()銆?**_rcu()** 閾捐〃鎿嶄綔鍘熻澧炲姞浜嗗湪寮卞唴瀛樺簭 CPU 涓婃墍闇€鐨勫唴瀛樺睆闅溿€?list_del_rcu() 鍘熻鐪佺暐浜嗘寚閽堟瘨鍖栬皟璇曡緟鍔╀唬鐮侊紝鍚﹀垯浼氬鑷村苟鍙戣鑰?褰诲簳澶辫触銆?
鍥犳锛屽綋璇昏€呰兘澶熷蹇嶈繃鏈熸暟鎹€佷笖琛ㄩ」鍙槸琚坊鍔犳垨鍒犻櫎鑰屼笉杩涜鍘熷湴淇敼鏃讹紝
浣跨敤 RCU 灏遍潪甯稿鏄擄紒


### 绀轰緥 3锛氬鐞嗗師鍦版洿鏂?
绯荤粺璋冪敤瀹¤浠ｇ爜骞朵笉鍘熷湴鏇存柊瀹¤瑙勫垯銆備笉杩囷紝濡傛灉瀹冭杩欎箞鍋氾紝鐢ㄤ簬瀹炵幇姝ょ洰鐨勭殑
璇诲啓閿佷唬鐮佸彲鑳藉涓嬫墍绀猴紙鍋囪鍙洿鏂?`field_count`锛屽惁鍒欐柊澧炵殑瀛楁浼氣€︹€︼級锛?```

	static inline int audit_upd_rule(struct audit_rule *rule,
					 struct list_head *list,
					 __u32 newaction,
					 __u32 newfield_count)
	{
		struct audit_entry *e;
		struct audit_entry *ne;

		write_lock(&auditsc_lock);
		/* Note: audit_filter_mutex held by caller. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				e->rule.action = newaction;
				e->rule.field_count = newfield_count;
				write_unlock(&auditsc_lock);
				return 0;
			}
		}
		write_unlock(&auditsc_lock);
		return -EFAULT;		/* No matching rule */
	}

```
RCU 鐗堟湰鍒涘缓涓€涓壇鏈紝鏇存柊璇ュ壇鏈紝鐒跺悗鐢ㄦ柊鏇存柊鐨勮〃椤规浛鎹㈡棫琛ㄩ」銆傝繖涓€杩炰覆鍔ㄤ綔鈥斺€?鍦ㄥ埗浣滃壇鏈互鎵ц鏇存柊鏃跺厑璁稿苟鍙戣鍙栤€斺€旀鏄?RCU锛坮ead-copy update锛岃-澶嶅埗-鏇存柊锛?鍚嶇О鐨勭敱鏉ャ€?
```

	static inline int audit_upd_rule(struct audit_rule *rule,
					 struct list_head *list,
					 __u32 newaction,
					 __u32 newfield_count)
	{
		struct audit_entry *e;
		struct audit_entry *ne;

		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				ne = kmalloc(sizeof(*entry), GFP_ATOMIC);
				if (ne == NULL)
					return -ENOMEM;
				audit_copy_rule(&ne->rule, &e->rule);
				ne->rule.action = newaction;
				ne->rule.field_count = newfield_count;
				list_replace_rcu(&e->list, &ne->list);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

```
鍚屾牱锛岃繖鍋囪璋冪敤鑰呮寔鏈?`audit_filter_mutex`銆傞€氬父锛屽湪杩欑浠ｇ爜涓啓鑰呴攣浼氬彉鎴愯嚜鏃嬮攣銆?
update_lsm_rule() 鍋氱殑浜嬫儏闈炲父绫讳技锛屽鏋滄兂鐪嬬湡姝ｇ殑 Linux 鍐呮牳浠ｇ爜鍙互鍙傝€冨畠銆?
杩欎竴妯″紡鐨勫彟涓€涓敤娉曞彲浠ュ湪 openvswitch 椹卞姩 `ct_limit_set()` 涓殑*杩炴帴璺熻釜琛?
浠ｇ爜閲屾壘鍒般€傝琛ㄤ繚瀛樿繛鎺ヨ窡韪〃椤癸紝骞跺鏈€澶ц〃椤规暟璁炬湁涓婇檺銆傛瘡涓?zone 鏈変竴涓繖鏍风殑
琛紝鍥犳姣忎釜 zone 鏈変竴涓?*闄愬埗锛坙imit锛?*銆倆one 閫氳繃鍝堝笇琛ㄦ槧灏勫埌瀹冧滑鐨勯檺鍒讹紝
鍝堝笇閾句娇鐢?RCU 绠＄悊鐨?hlist銆傚綋璁剧疆鏂扮殑闄愬埗鏃讹紝浼氬垎閰嶄竴涓柊鐨勯檺鍒跺璞★紝
骞惰皟鐢?`ct_limit_set()` 浣跨敤 list_replace_rcu() 鐢ㄦ柊闄愬埗瀵硅薄鏇挎崲鏃х殑闄愬埗瀵硅薄銆?鏃х殑闄愬埗瀵硅薄闅忓悗鍦ㄥ闄愭湡涔嬪悗閫氳繃 kfree_rcu() 閲婃斁銆?

### 绀轰緥 4锛氭秷闄よ繃鏈熸暟鎹?
涓婇潰鐨勫璁′緥瀛愬蹇嶈繃鏈熸暟鎹紝澶у鏁拌拷韪閮ㄧ姸鎬佺殑绠楁硶涔熸槸濡傛銆?姣曠珶锛屼粠澶栭儴鐘舵€佸彉鍖栧埌 Linux 瀵熻鍒拌繖涓€鍙樺寲涔嬮棿瀛樺湪寤惰繜锛屽洜姝ゅ鍓嶆墍杩帮紝
灏戦噺鐨勩€佺敱 RCU 寮曞叆鐨勯澶栬繃鏈熼€氬父涓嶆垚闂銆?
鐒惰€岋紝瀛樺湪璁稿鏃犳硶瀹瑰繊杩囨湡鏁版嵁鐨勪緥瀛愩€侺inux 鍐呮牳涓殑涓€涓緥瀛愭槸 System V IPC
锛堝弬瑙?ipc/shm.c 涓殑 shm_lock() 鍑芥暟锛夈€傝浠ｇ爜鍦ㄦ瘡琛ㄩ」鑷棆閿佷笅妫€鏌ヤ竴涓?**deleted锛堝凡鍒犻櫎锛?*鏍囧織锛屽鏋滆**deleted**鏍囧織琚疆浣嶏紝灏卞亣瑁呰琛ㄩ」涓嶅瓨鍦ㄣ€?涓轰簡璁╄繖璧蜂綔鐢紝鎼滅储鍑芥暟蹇呴』鍦ㄦ寔鏈夋瘡琛ㄩ」鑷棆閿佺殑鎯呭喌涓嬭繑鍥烇紝姝ｅ shm_lock()
瀹為檯鎵€鍋氱殑閭ｆ牱銆?

蹇€熸祴楠岋細
	瑕佽 deleted 鏍囧織鎶€鏈捣浣滅敤锛屼负浠€涔堝繀椤诲湪浠庢悳绱㈠嚱鏁拌繑鍥炴椂鎸佹湁姣忚〃椤圭殑閿侊紵

蹇€熸祴楠岀瓟妗?<quick_quiz_answer>

濡傛灉绯荤粺璋冪敤瀹¤妯″潡灏嗘潵闇€瑕佹嫆缁濊繃鏈熸暟鎹紝涓€绉嶅疄鐜版柟寮忔槸缁欏璁¤〃椤?澧炲姞涓€涓?`deleted` 鏍囧織鍜屼竴涓?`lock` 鑷棆閿侊紝濡備笅浠ｇ爜鎵€绀猴細
```

	static struct audit_entry *audit_filter_task(struct task_struct *tsk, char **key)
	{
		struct audit_entry *e;
		enum audit_state   state;

		rcu_read_lock();
		list_for_each_entry_rcu(e, &audit_tsklist, list) {
			if (audit_filter_rules(tsk, &e->rule, NULL, &state)) {
				spin_lock(&e->lock);
				if (e->deleted) {
					spin_unlock(&e->lock);
					rcu_read_unlock();
					return NULL;
				}
				rcu_read_unlock();
				if (state == AUDIT_STATE_RECORD)
					*key = kstrdup(e->rule.filterkey, GFP_ATOMIC);
				/* As long as e->lock is held, e is valid and
				 * its value is not stale */
				return e;
			}
		}
		rcu_read_unlock();
		return NULL;
	}

```
`audit_del_rule()` 鍑芥暟闇€瑕佸湪姣忚〃椤归攣涓嬭缃?`deleted` 鏍囧織锛屽涓嬩唬鐮佹墍绀猴細
```

	static inline int audit_del_rule(struct audit_rule *rule,
					 struct list_head *list)
	{
		struct audit_entry *e;

		/* No need to use the _rcu iterator here, since this
		 * is the only deletion routine. */
		list_for_each_entry(e, list, list) {
			if (!audit_compare_rule(rule, &e->rule)) {
				spin_lock(&e->lock);
				list_del_rcu(&e->list);
				e->deleted = 1;
				spin_unlock(&e->lock);
				call_rcu(&e->rcu, audit_free_rule);
				return 0;
			}
		}
		return -EFAULT;		/* No matching rule */
	}

```
杩欎篃鍋囪璋冪敤鑰呮寔鏈?`audit_filter_mutex`銆?
娉ㄦ剰锛屾湰渚嬪亣璁捐〃椤瑰彧浼氳娣诲姞鍜屽垹闄ゃ€傝姝ｇ‘澶勭悊 audit_upd_rule() 鎵ц鐨?鍘熷湴鏇存柊锛岃繕闇€瑕侀澶栫殑鏈哄埗銆備竴鏂归潰锛宎udit_upd_rule() 鍦ㄦ墽琛?list_replace_rcu() 鏃堕渶瑕佸悓鏃舵寔鏈夋棫鐨?`audit_entry` 鍙婂叾鏇挎崲椤圭殑閿併€?

### 绀轰緥 5锛氳烦杩囪繃鏈熷璞?
瀵规煇浜涚敤渚嬭€岃█锛屽彲浠ラ€氳繃鍦ㄨ渚ч摼琛ㄩ亶鍘嗘湡闂磋烦杩囪繃鏈熷璞℃潵鎻愬崌璇昏€呮€ц兘锛?杩欓噷鐨勮繃鏈熷璞℃槸鎸囬偅浜涘皢鍦ㄤ竴涓垨澶氫釜瀹介檺鏈熶箣鍚庤绉婚櫎骞堕攢姣佺殑瀵硅薄銆倀imerfd
瀛愮郴缁熶腑鍙互鎵惧埌杩欐牱涓€涓緥瀛愩€傚綋 `CLOCK_REALTIME` 鏃堕挓琚噸鏂扮紪绋嬫椂
锛堜緥濡傜敱浜庤缃簡绯荤粺鏃堕棿锛夛紝鎵€鏈変緷璧栦簬姝ゆ椂閽熺殑宸茬紪绋?`timerfds` 閮戒細琚Е鍙戯紝
绛夊緟瀹冧滑鐨勮繘绋嬩細鍦ㄩ瀹氬埌鏈熸椂闂翠箣鍓嶈鍞ら啋銆備负渚夸簬瀹炵幇锛屾墍鏈夎繖浜涘畾鏃跺櫒鍦ㄩ€氳繃
濡備笅浠ｇ爜寤虹珛鏃堕兘浼氳鍔犲叆涓€涓敱 RCU 绠＄悊鐨?`cancel_list`锛?```

	static void timerfd_setup_cancel(struct timerfd_ctx *ctx, int flags)
	{
		spin_lock(&ctx->cancel_lock);
		if ((ctx->clockid == CLOCK_REALTIME ||
		     ctx->clockid == CLOCK_REALTIME_ALARM) &&
		    (flags & TFD_TIMER_ABSTIME) && (flags & TFD_TIMER_CANCEL_ON_SET)) {
			if (!ctx->might_cancel) {
				ctx->might_cancel = true;
				spin_lock(&cancel_lock);
				list_add_rcu(&ctx->clist, &cancel_list);
				spin_unlock(&cancel_lock);
			}
		} else {
			__timerfd_remove_cancel(ctx);
		}
		spin_unlock(&ctx->cancel_lock);
	}

```
褰撲竴涓?timerfd 琚噴鏀撅紙fd 琚叧闂級鏃讹紝`might_cancel` 鏍囧織琚竻闄わ紝
瀵硅薄浠?`cancel_list` 涓Щ闄ゅ苟閿€姣侊紝濡備笅绠€鍖栦笖鍐呰仈鐨勪唬鐮佹墍绀猴細
```

	int timerfd_release(struct inode *inode, struct file *file)
	{
		struct timerfd_ctx *ctx = file->private_data;

		spin_lock(&ctx->cancel_lock);
		if (ctx->might_cancel) {
			ctx->might_cancel = false;
			spin_lock(&cancel_lock);
			list_del_rcu(&ctx->clist);
			spin_unlock(&cancel_lock);
		}
		spin_unlock(&ctx->cancel_lock);

		if (isalarm(ctx))
			alarm_cancel(&ctx->t.alarm);
		else
			hrtimer_cancel(&ctx->t.tmr);
		kfree_rcu(ctx, rcu);
		return 0;
	}

```
濡傛灉璁剧疆浜?`CLOCK_REALTIME` 鏃堕挓锛堜緥濡傜敱鏃堕棿鏈嶅姟鍣ㄨ缃級锛宧rtimer 妗嗘灦浼氳皟鐢?`timerfd_clock_was_set()`锛屽畠閬嶅巻 `cancel_list` 骞跺敜閱掔瓑寰呭湪璇?timerfd 涓婄殑杩涚▼銆?鍦ㄩ亶鍘?`cancel_list` 鏃讹紝浼氭煡璇?`might_cancel` 鏍囧織浠ヨ烦杩囪繃鏈熺殑瀵硅薄锛?濡備笅浠ｇ爜鎵€绀猴細
```

	void timerfd_clock_was_set(void)
	{
		ktime_t moffs = ktime_mono_to_real(0);
		struct timerfd_ctx *ctx;
		unsigned long flags;

		rcu_read_lock();
		list_for_each_entry_rcu(ctx, &cancel_list, clist) {
			if (!ctx->might_cancel)
				continue;
			spin_lock_irqsave(&ctx->wqh.lock, flags);
			if (ctx->moffs != moffs) {
				ctx->moffs = KTIME_MAX;
				ctx->ticks++;
				wake_up_locked_poll(&ctx->wqh, EPOLLIN);
			}
			spin_unlock_irqrestore(&ctx->wqh.lock, flags);
		}
		rcu_read_unlock();
	}

```
鍏抽敭鍦ㄤ簬锛岀敱浜庡 `cancel_list` 鐨?RCU 淇濇姢閬嶅巻涓庡璞＄殑娣诲姞鍜岀Щ闄ゆ槸骞跺彂杩涜鐨勶紝
鏈夋椂閬嶅巻浼氳闂埌宸茬粡浠庨摼琛ㄤ腑绉婚櫎鐨勫璞°€傚湪鏈緥涓紝浣跨敤涓€涓爣蹇楁潵璺宠繃杩欑被瀵硅薄銆?

### 鎬荤粨

鑳藉瀹瑰繊杩囨湡鏁版嵁鐨勩€佷互璇讳负涓荤殑銆佸熀浜庨摼琛ㄧ殑鏁版嵁缁撴瀯锛屾渶閫傚悎浣跨敤 RCU銆?鏈€绠€鍗曠殑鎯呭喌鏄〃椤硅娣诲姞鎴栧垹闄わ紙鎴栧師鍦板師瀛愪慨鏀癸級锛屼絾闈炲師瀛愮殑鍘熷湴淇敼鍙互
閫氳繃鍒朵綔鍓湰銆佹洿鏂板壇鏈€佺劧鍚庣敤鍓湰鏇挎崲鍘熷璞℃潵澶勭悊銆傚鏋滄棤娉曞蹇嶈繃鏈熸暟鎹紝
鍒欏彲浠ョ粨鍚堟瘡琛ㄩ」鑷棆閿佷娇鐢ㄤ竴涓?**deleted** 鏍囧織锛屼互鍏佽鎼滅储鍑芥暟鎷掔粷
鏂板垹闄ょ殑鏁版嵁銆?

蹇€熸祴楠岀瓟妗堬細
	瑕佽 deleted 鏍囧織鎶€鏈捣浣滅敤锛屼负浠€涔堝繀椤诲湪浠庢悳绱㈠嚱鏁拌繑鍥炴椂鎸佹湁姣忚〃椤圭殑閿侊紵

	濡傛灉鎼滅储鍑芥暟鍦ㄨ繑鍥炰箣鍓嶉噴鏀句簡姣忚〃椤归攣锛岄偅涔堣皟鐢ㄨ€呮棤璁哄浣曢兘鍦ㄥ鐞嗚繃鏈熸暟鎹€?	濡傛灉澶勭悊杩囨湡鏁版嵁纭疄鍙互鎺ュ彈锛岄偅涔堜綘涓嶉渶瑕?**deleted** 鏍囧織銆傚鏋滃鐞嗚繃鏈熸暟鎹?	纭疄鏄釜闂锛岄偅涔堜綘闇€瑕佸湪浣跨敤鎵€杩斿洖鍊肩殑鍏ㄩ儴浠ｇ爜鑼冨洿鍐呮寔鏈夋瘡琛ㄩ」閿併€?
杩斿洖蹇€熸祴楠?<quick_quiz>
