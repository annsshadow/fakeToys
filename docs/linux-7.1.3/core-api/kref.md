## 涓哄唴鏍稿璞℃坊鍔犲紩鐢ㄨ鏁板櫒锛坘ref锛?

:Author: Corey Minyard <minyard@acm.org>
:Author: Thomas Hellstr枚m <thomas.hellstrom@linux.intel.com>

鏈枃澶ч噺鍐呭鍙栬嚜 Greg Kroah-Hartman 鍦?2004 骞?OLS 澶т細涓婂彂琛ㄧ殑鍏充簬 kref 鐨勮鏂囦笌婕旇锛屽彲鍦ㄤ互涓嬪湴鍧€鎵惧埌锛?
  - http://www.kroah.com/linux/talks/ols_2004_kref_paper/Reprint-Kroah-Hartman-OLS2004.pdf
  - http://www.kroah.com/linux/talks/ols_2004_kref_talk/

## 绠€浠?

kref 璁╀綘鑳藉涓哄璞℃坊鍔犲紩鐢ㄨ鏁板櫒銆傚鏋滀綘鐨勫璞″湪澶氬琚娇鐢ㄥ苟浼犳潵浼犲幓锛岃€屼綘娌℃湁寮曠敤璁℃暟锛岄偅涔堜綘鐨勪唬鐮佸嚑涔庤偗瀹氭槸鏈夐棶棰樼殑銆?濡傛灉浣犳兂瑕佸紩鐢ㄨ鏁帮紝kref 灏辨槸姝ｇ‘鐨勯€夋嫨銆?
```
    struct my_data
    {
	.
	.
	struct kref refcount;
	.
	.
    };
```

kref 鍙互鍑虹幇鍦ㄦ暟鎹粨鏋勭殑浠讳綍浣嶇疆銆?
## 鍒濆鍖?

浣犲繀椤诲湪鍒嗛厤 kref 涔嬪悗瀵瑰叾杩涜鍒濆鍖栥€備负姝わ紝璋冪敤

```
     struct my_data *data;

     data = kmalloc(sizeof(*data), GFP_KERNEL);
     if (!data)
            return -ENOMEM;
     kref_init(&data->refcount);
```

杩欎細灏?kref 涓殑寮曠敤璁℃暟璁句负 1銆?
## kref 瑙勫垯


涓€鏃︿綘鏈変簡涓€涓垵濮嬪寲濂界殑 kref锛屽氨蹇呴』閬靛惊浠ヤ笅瑙勫垯锛?
1) 濡傛灉浣犲垱寤轰簡鏌愪釜鎸囬拡鐨勯潪涓存椂鍓湰锛屽挨鍏舵槸瀹冨彲鑳戒紶缁欏彟涓€涓墽琛岀嚎绋嬫椂锛屼綘蹇呴』

```
       kref_get(&data->refcount);
```

   濡傛灉浣犲凡缁忔嫢鏈変竴涓寚鍚?kref 缁撴瀯浣撶殑鏈夋晥鎸囬拡锛堝紩鐢ㄨ鏁颁笉鍙兘鍙樹负闆讹級锛屽垯鍙互鏃犻渶鍔犻攣鍦版墽琛屾鎿嶄綔銆?
```
       kref_put(&data->refcount, data_release);
```

   濡傛灉杩欐槸瀵规寚閽堢殑鏈€鍚庝竴涓紩鐢紝鍒欎細璋冪敤閲婃斁渚嬬▼銆傚鏋滀唬鐮佷粠涓嶈瘯鍥惧湪娌℃湁宸茬粡鎸佹湁鏈夋晥鎸囬拡鐨勬儏鍐典笅鍘昏幏鍙栨寚鍚?kref 缁撴瀯浣撶殑鏈夋晥鎸囬拡锛岄偅涔堟棤闇€鍔犻攣鍗冲彲瀹夊叏鍦版墽琛屾鎿嶄綔銆?
3) 濡傛灉浠ｇ爜璇曞浘鍦ㄦ病鏈夊凡缁忔寔鏈夋湁鏁堟寚閽堢殑鎯呭喌涓嬪幓鑾峰彇鎸囧悜 kref 缁撴瀯浣撶殑寮曠敤锛屽畠蹇呴』涓茶鍖栬闂紝浣垮緱鍦?kref_get() 鏈熼棿涓嶈兘鍙戠敓 kref_put()锛屼笖缁撴瀯浣撳湪 kref_get() 鏈熼棿蹇呴』淇濇寔鏈夋晥銆?
渚嬪锛屽鏋滀綘鍒嗛厤浜嗕竴浜涙暟鎹苟灏嗗叾浼犵粰鍙︿竴涓?
```
    void data_release(struct kref *ref)
    {
	struct my_data *data = container_of(ref, struct my_data, refcount);
	kfree(data);
    }

    void more_data_handling(void *cb_data)
    {
	struct my_data *data = cb_data;
	.
	. do stuff with data here
	.
	kref_put(&data->refcount, data_release);
    }

    int my_data_handler(void)
    {
	int rv = 0;
	struct my_data *data;
	struct task_struct *task;
	data = kmalloc(sizeof(*data), GFP_KERNEL);
	if (!data)
		return -ENOMEM;
	kref_init(&data->refcount);

	kref_get(&data->refcount);
	task = kthread_run(more_data_handling, data, "more_data_handling");
	if (task == ERR_PTR(-ENOMEM)) {
		rv = -ENOMEM;
	        kref_put(&data->refcount, data_release);
		goto out;
	}

	.
	. do stuff with data here
	.
    out:
	kref_put(&data->refcount, data_release);
	return rv;
    }
```

杩欐牱锛屾棤璁轰袱涓嚎绋嬩互浣曠椤哄簭澶勭悊鏁版嵁锛宬ref_put() 閮戒細璐熻矗鍒ゆ柇鏁版嵁浣曟椂涓嶅啀琚紩鐢ㄥ苟閲婃斁瀹冦€俴ref_get() 涓嶉渶瑕佸姞閿侊紝
鍥犱负鎴戜滑宸茬粡鎷ユ湁涓€涓寔鏈夊叾寮曠敤璁℃暟鐨勬湁鏁堟寚閽堛€俻ut 涔熶笉闇€瑕佸姞閿侊紝鍥犱负娌℃湁涓滆タ浼氬湪鏈寔鏈夋寚閽堢殑鎯呭喌涓嬪幓鑾峰彇璇ユ暟鎹€?
鍦ㄤ笂渚嬩腑锛屾棤璁烘槸鍦ㄦ垚鍔熻矾寰勮繕鏄敊璇矾寰勪腑锛宬ref_put() 閮戒細琚皟鐢?2 娆°€傝繖鏄繀瑕佺殑锛屽洜涓哄紩鐢ㄨ鏁拌 kref_init() 鍜?kref_get() 鍚勫鍔犱簡 1 娆°€?
娉ㄦ剰瑙勫垯 1 涓殑鈥滀箣鍓嶁€濋潪甯稿叧閿€備綘缁濅笉搴旇

```
	task = kthread_run(more_data_handling, data, "more_data_handling");
	if (task == ERR_PTR(-ENOMEM)) {
		rv = -ENOMEM;
		goto out;
	} else
		/* BAD BAD BAD - get is after the handoff */
		kref_get(&data->refcount);
```

涓嶈鑷互涓轰簡瑙ｈ嚜宸卞湪鍋氫粈涔堝氨浣跨敤涓婅堪鍐欐硶銆傞鍏堬紝浣犲彲鑳藉苟涓嶆竻妤氳嚜宸卞湪鍋氫粈涔堛€傚叾娆★紝浣犲彲鑳界‘瀹炴竻妤氳嚜宸卞湪鍋氫粈涔堬紙鍦ㄦ煇浜涙儏鍐典笅娑夊強鍔犻攣锛屼笂杩板啓娉曞彲鑳芥槸鍚堟硶鐨勶級锛?浣嗗叾浠栦笉娓呮鎯呭喌鐨勪汉鍙兘浼氫慨鏀规垨澶嶅埗杩欐浠ｇ爜銆傝繖鏄碂绯曠殑椋庢牸銆備笉瑕佽繖鏍峰仛銆?
鍦ㄦ煇浜涙儏鍐典笅浣犲彲浠ヤ紭鍖?get 鍜?put銆備緥濡傦紝濡傛灉浣犲凡缁忕敤瀹屼竴涓璞″苟灏嗗叾鍏ラ槦浜ょ粰鍏朵粬涓滆タ鎴栦紶閫掔粰鍏朵粬涓滆タ锛屽氨娌℃湁鐞嗙敱

```
	/* Silly extra get and put */
	kref_get(&obj->ref);
	enqueue(obj);
	kref_put(&obj->ref, obj_cleanup);
```

```
	enqueue(obj);
	/* We are done with obj, so we pass our refcount off
	   to the queue.  DON'T TOUCH obj AFTER HERE! */
```

鏈€鍚庝竴鏉¤鍒欙紙瑙勫垯 3锛夋槸鏈€闅惧鐞嗙殑銆備妇渚嬫潵璇达紝鍋囪浣犳湁涓€涓敱鍚勮嚜甯?kref 鐨勯」缁勬垚鐨勫垪琛紝鑰屼綘甯屾湜鑾峰彇绗竴涓€備綘涓嶈兘鐩存帴鎶婄涓€椤逛粠鍒楄〃涓彇鍑哄苟 kref_get()銆?閭ｈ繚鍙嶄簡瑙勫垯 3锛屽洜涓轰綘骞舵病鏈夊凡缁忔寔鏈変竴涓湁鏁堟寚閽堛€備綘蹇呴』娣诲姞涓€涓簰鏂ヤ綋锛堟垨鍏朵粬閿侊級銆?
```
	static DEFINE_MUTEX(mutex);
	static LIST_HEAD(q);
	struct my_data
	{
		struct kref      refcount;
		struct list_head link;
	};

	static struct my_data *get_entry()
	{
		struct my_data *entry = NULL;
		mutex_lock(&mutex);
		if (!list_empty(&q)) {
			entry = container_of(q.next, struct my_data, link);
			kref_get(&entry->refcount);
		}
		mutex_unlock(&mutex);
		return entry;
	}

	static void release_entry(struct kref *ref)
	{
		struct my_data *entry = container_of(ref, struct my_data, refcount);

		list_del(&entry->link);
		kfree(entry);
	}

	static void put_entry(struct my_data *entry)
	{
		mutex_lock(&mutex);
		kref_put(&entry->refcount, release_entry);
		mutex_unlock(&mutex);
	}
```

kref_put() 鐨勮繑鍥炲€煎湪浣犱笉甯屾湜鍦ㄦ暣娈甸噴鏀炬搷浣滄湡闂存寔鏈夐攣鏃跺緢鏈夌敤銆傚亣璁惧湪涓婁緥涓綘涓嶆兂鍦ㄦ寔鏈夐攣鐨勬儏鍐典笅璋冪敤 kfree()
锛堝洜涓洪噴鏀炬搷浣滄湁鐐?
```
	static void release_entry(struct kref *ref)
	{
		/* All work is done after the return from kref_put(). */
	}

	static void put_entry(struct my_data *entry)
	{
		mutex_lock(&mutex);
		if (kref_put(&entry->refcount, release_entry)) {
			list_del(&entry->link);
			mutex_unlock(&mutex);
			kfree(entry);
		} else
			mutex_unlock(&mutex);
	}
```

杩欏湪浣犲繀椤昏皟鐢ㄥ叾浠栦綔涓洪噴鏀句竴閮ㄥ垎銆佸彲鑳借€楁椂杈冮暱鎴栧彲鑳界敵璇峰悓涓€鎶婇攣鐨勪緥绋嬫椂鏇翠负鏈夌敤銆傛敞鎰忥紝鍦ㄩ噴鏀句緥绋嬩腑瀹屾垚鎵€鏈夊伐浣滀粛鏄閫夛紝
鍥犱负瀹冩洿鏁存磥涓€浜涖€?
涓婇潰鐨勪緥瀛愪篃鍙互浣跨敤 kref_get_unless_zero() 鏉ヤ紭鍖栵紝鍏蜂綋瑙?
```
	static struct my_data *get_entry()
	{
		struct my_data *entry = NULL;
		mutex_lock(&mutex);
		if (!list_empty(&q)) {
			entry = container_of(q.next, struct my_data, link);
			if (!kref_get_unless_zero(&entry->refcount))
				entry = NULL;
		}
		mutex_unlock(&mutex);
		return entry;
	}

	static void release_entry(struct kref *ref)
	{
		struct my_data *entry = container_of(ref, struct my_data, refcount);

		mutex_lock(&mutex);
		list_del(&entry->link);
		mutex_unlock(&mutex);
		kfree(entry);
	}

	static void put_entry(struct my_data *entry)
	{
		kref_put(&entry->refcount, release_entry);
	}
```

杩欏彲鐢ㄤ簬绉婚櫎 put_entry() 涓?kref_put() 鍛ㄥ洿鐨勪簰鏂ラ攣锛屼絾閲嶈鐨勬槸 kref_get_unless_zero 蹇呴』琚寘瑁瑰湪涓庡湪鏌ユ壘琛ㄤ腑鎵惧埌璇ラ」鐩稿悓鐨勪复鐣屽尯鍐咃紝
鍚﹀垯 kref_get_unless_zero 鍙兘寮曠敤宸茶閲婃斁鐨勫唴瀛樸€傛敞鎰忥紝鏈粡妫€鏌ヨ繑鍥炲€煎氨浣跨敤 kref_get_unless_zero 鏄潪娉曠殑銆?濡傛灉浣犵‘瀹氾紙鍥犱负宸茬粡鎸佹湁鏈夋晥鎸囬拡锛塳ref_get_unless_zero() 浼氳繑鍥?true锛岄偅涔堣鏀圭敤 kref_get()銆?
## Kref 涓?RCU


鍑芥暟 kref_get_unless_zero 杩樹娇寰楀彲浠ュ皢 rcu 鐢ㄤ簬

```
	struct my_data
	{
		struct rcu_head rhead;
		.
		struct kref refcount;
		.
		.
	};

	static struct my_data *get_entry_rcu()
	{
		struct my_data *entry = NULL;
		rcu_read_lock();
		if (!list_empty(&q)) {
			entry = container_of(q.next, struct my_data, link);
			if (!kref_get_unless_zero(&entry->refcount))
				entry = NULL;
		}
		rcu_read_unlock();
		return entry;
	}

	static void release_entry_rcu(struct kref *ref)
	{
		struct my_data *entry = container_of(ref, struct my_data, refcount);

		mutex_lock(&mutex);
		list_del_rcu(&entry->link);
		mutex_unlock(&mutex);
		kfree_rcu(entry, rhead);
	}

	static void put_entry(struct my_data *entry)
	{
		kref_put(&entry->refcount, release_entry_rcu);
	}
```

浣嗚娉ㄦ剰锛宻truct kref 鎴愬憳闇€瑕佸湪璋冪敤 release_entry_rcu 涔嬪悗淇濇寔鏈夋晥鍐呭瓨杈句竴涓?RCU 瀹介檺鏈熴€傝繖鍙互閫氳繃濡備笂浣跨敤 kfree_rcu(entry, rhead) 瀹炵幇锛?鎴栧湪浣跨敤 kfree 涔嬪墠璋冪敤 synchronize_rcu()锛屼絾璇锋敞鎰?synchronize_rcu() 鍙兘浼氱潯鐪犵浉褰撻暱鐨勪竴娈垫椂闂淬€?
## 鍑芥暟涓庣粨鏋勪綋


