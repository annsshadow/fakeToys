
## 鍙?RCU 淇濇姢鐨勫垪琛?鏁扮粍鍏冪礌鐨勫紩鐢ㄨ鏁拌璁?



璇锋敞鎰忥紝濡傛灉浣犻渶瑕佸皢寮曠敤璁℃暟涓?RCU 缁撳悎锛宲ercpu-ref 鐗规€у緢鍙兘鏄綘鐨勯閫夈€?
璇峰弬瑙?include/linux/percpu-refcount.h 浜嗚В鏇村淇℃伅銆傜劧鑰岋紝鍦?percpu-ref 浼?
娑堣€楄繃澶氬唴瀛樼殑缃曡鎯呭喌涓嬶紝璇风户缁線涓嬭銆?

------------------------------------------------------------------------

瀵逛簬鍙椾紶缁熻鑰?鍐欒€呰嚜鏃嬮攣鎴栦俊鍙烽噺淇濇姢鐨勫垪琛ㄥ厓绱犺繘琛屽紩鐢ㄨ鏁版槸鐩存埅浜嗗綋鐨勶細

```

    1.					    2.
    add()				    search_and_reference()
    {					    {
	alloc_object				read_lock(&list_lock);
	...					search_for_element
	atomic_set(&el->rc, 1);			atomic_inc(&el->rc);
	write_lock(&list_lock);			 ...
	add_element				read_unlock(&list_lock);
	...					...
	write_unlock(&list_lock);	   }
    }

    3.					    4.
    release_referenced()		    delete()
    {					    {
	...					write_lock(&list_lock);
	if(atomic_dec_and_test(&el->rc))	...
	    kfree(el);
	...					remove_element
    }						write_unlock(&list_lock);
						...
						if (atomic_dec_and_test(&el->rc))
						    kfree(el);
						...
					    }

```
濡傛灉鍍忎笅闈㈣繖鏍风敤 RCU 鎶婅鍒楄〃/鏁扮粍鍙樻垚鏃犻攣锛氬湪 add() 鍜?delete() 涓妸
write_lock() 鏀逛负 spin_lock()锛屽苟鎶?search_and_reference() 涓殑 read_lock()
鏀逛负 rcu_read_lock()锛岄偅涔?search_and_reference() 涓殑 atomic_inc() 鏈夊彲鑳芥寔鏈?
涓€涓凡缁忎粠鍒楄〃/鏁扮粍涓垹闄ょ殑鍏冪礌鐨勫紩鐢ㄣ€傚湪杩欑鎯呭喌涓嬭浣跨敤 atomic_inc_not_zero()锛?
濡備笅鎵€绀猴細

```

    1.					    2.
    add()				    search_and_reference()
    {					    {
	alloc_object				rcu_read_lock();
	...					search_for_element
	atomic_set(&el->rc, 1);			if (!atomic_inc_not_zero(&el->rc)) {
	spin_lock(&list_lock);			    rcu_read_unlock();
						    return FAIL;
	add_element				}
	...					...
	spin_unlock(&list_lock);		rcu_read_unlock();
    }					    }
    3.					    4.
    release_referenced()		    delete()
    {					    {
	...					spin_lock(&list_lock);
	if (atomic_dec_and_test(&el->rc))	...
	    call_rcu(&el->head, el_free);	remove_element
	...					spin_unlock(&list_lock);
    }						...
						if (atomic_dec_and_test(&el->rc))
						    call_rcu(&el->head, el_free);
						...
					    }

```
鏈夋椂闇€瑕佸湪鏇存柊锛堝啓锛夎矾寰勪腑鑾峰彇鍏冪礌鐨勫紩鐢ㄣ€傚湪杩欑鎯呭喌涓嬶紝atomic_inc_not_zero()
鍙兘鏈変簺杩囧害锛屽洜涓烘垜浠寔鏈夋洿鏂颁晶鐨勮嚜鏃嬮攣銆傛鏃跺彲浠ユ敼鐢?atomic_inc()銆?

鍦?search_and_reference() 浠ｇ爜璺緞涓鐞?"FAIL" 骞朵笉鎬绘槸鏂逛究銆傚湪杩欑鎯呭喌涓嬶紝
鍙互鎶?atomic_dec_and_test() 浠?delete() 绉诲埌 el_free() 涓紝濡備笅鎵€绀猴細

```

    1.					    2.
    add()				    search_and_reference()
    {					    {
	alloc_object				rcu_read_lock();
	...					search_for_element
	atomic_set(&el->rc, 1);			atomic_inc(&el->rc);
	spin_lock(&list_lock);			...

	add_element				rcu_read_unlock();
	...				    }
	spin_unlock(&list_lock);	    4.
    }					    delete()
    3.					    {
    release_referenced()			spin_lock(&list_lock);
    {						...
	...					remove_element
	if (atomic_dec_and_test(&el->rc))	spin_unlock(&list_lock);
	    kfree(el);				...
	...					call_rcu(&el->head, el_free);
    }						...
    5.					    }
    void el_free(struct rcu_head *rhp)
    {
	release_referenced();
    }

```
鍏抽敭鐐瑰湪浜庯紝add() 娣诲姞鐨勫垵鍊煎紩鐢紝瑕佺瓑鍒扮Щ闄や箣鍚庣殑涓€涓闄愭湡杩囧幓涔嬪悗鎵嶄細琚?
绉婚櫎銆傝繖鎰忓懗鐫€ search_and_reference() 鎵句笉鍒拌鍏冪礌锛屼篃灏辨槸璇?el->rc 鐨勫€兼棤娉?
澧炲姞銆傚洜姝わ紝涓€鏃﹀畠闄嶅埌闆讹紝灏变笉瀛樺湪浠讳綍鑳藉銆佹垨灏嗚兘澶熷紩鐢ㄨ鍏冪礌鐨勮鑰呫€傝鍏冪礌
鍥犳鍙互琚畨鍏ㄥ湴閲婃斁銆傝繖鍙嶈繃鏉ヤ繚璇佷簡锛氬鏋滀换浣曡鑰呮壘鍒颁簡璇ュ厓绱狅紝璇ヨ鑰呭彲浠?
鍦ㄤ笉妫€鏌ュ紩鐢ㄨ鏁板€肩殑鎯呭喌涓嬪畨鍏ㄥ湴鑾峰彇涓€涓紩鐢ㄣ€?

鐩告瘮浜庢竻鍗?B 涓殑妯″紡锛屾竻鍗?C 涓熀浜?RCU 鐨勬ā寮忔湁涓€涓槑鏄句紭鍔匡細浠讳綍瀹氫綅鍒版煇涓?
缁欏畾瀵硅薄鐨?search_and_reference() 璋冪敤锛屽嵆渚胯鍚屼竴瀵硅薄鐨?delete() 姝ｅ湪骞跺彂璋冪敤锛?
涔熼兘鑳芥垚鍔熻幏鍙栬瀵硅薄鐨勫紩鐢ㄣ€傜被浼煎湴锛屾竻鍗?B 鍜?C 鐩告瘮浜庢竻鍗?A 鐨勪竴涓槑鏄句紭鍔挎槸锛?
鍗充究鏈変换鎰忓ぇ閲忕殑 search_and_reference() 璋冪敤鍦ㄦ煡鎵?delete() 鎵€浣滅敤鐨勫悓涓€瀵硅薄锛?
delete() 鐨勮皟鐢ㄤ篃涓嶄細琚欢杩熴€傜浉鍙嶏紝琚欢杩熺殑浠呬粎鏄?kfree() 鐨勬渶缁堣皟鐢紝鑰屽湪鐜颁唬
璁＄畻鏈虹郴缁燂紙鍗充究鏄皬鍨嬬殑锛変笂杩欓€氬父涓嶆槸闂銆?

鍦?delete() 鍙互浼戠湢鐨勬儏鍐典笅锛宻ynchronize_rcu() 鍙互浠?
```

    4.
    delete()
    {
	spin_lock(&list_lock);
	...
	remove_element
	spin_unlock(&list_lock);
	...
	synchronize_rcu();
	if (atomic_dec_and_test(&el->rc))
	    kfree(el);
	...
    }

```
浣滀负鍐呮牳涓殑鏇村渚嬪瓙锛屾竻鍗?C 涓殑妯″紡鐢ㄤ簬 struct pid 鐨勫紩鐢ㄨ鏁帮紝鑰屾竻鍗?B 涓殑
妯″紡鐢ㄤ簬 struct posix_acl銆?
