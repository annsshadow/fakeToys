
## 浣跨敤 RCU hlist_nulls 淇濇姢閾捐〃鍜屽璞?

鏈妭浠嬬粛濡備綍浣跨敤 hlist_nulls 鏉ヤ繚鎶や互璇讳负涓伙紙read-mostly锛夌殑閾捐〃浠ュ強浣跨敤 SLAB_TYPESAFE_BY_RCU 鍒嗛厤鐨勫璞°€?
璇峰厛闃呰 listRCU.rst 涓殑鍩虹鐭ヨ瘑銆?
## 浣跨敤 'nulls'


浣跨敤鐗规畩鐨勬爣璁帮紙绉颁负 'nulls'锛夋槸瑙ｅ喅涓嬭堪闂鐨勪竴绉嶄究鎹锋柟娉曘€?
鍦ㄦ病鏈?'nulls' 鐨勬儏鍐典笅锛屼竴涓鐞嗛€氳繃 SLAB_TYPESAFE_BY_RCU kmem_cache 鍒嗛厤瀵硅薄鐨勫吀鍨?RCU 閾捐〃鍙互浣跨敤浠ヤ笅绠楁硶銆備互涓嬬ず渚嬪亣瀹?'obj' 鏄寚鍚戞绫诲璞＄殑鎸囬拡锛屽叾绫诲瀷濡備笅銆?
```

  struct object {
    struct hlist_node obj_node;
    atomic_t refcnt;
    unsigned int key;
  };

```
### 1) 鏌ユ壘绠楁硶


```

  begin:
  rcu_read_lock();
  obj = lockless_lookup(key);
  if (obj) {
    if (!try_get_ref(obj)) { // might fail for free objects
      rcu_read_unlock();
      goto begin;
    }
    /*
    * Because a writer could delete object, and a writer could
    * reuse these object before the RCU grace period, we
    * must check key after getting the reference on object
    */
    if (obj->key != key) { // not the object we expected
      put_ref(obj);
      rcu_read_unlock();
      goto begin;
    }
  }
  rcu_read_unlock();

```
璇锋敞鎰忥紝lockless_lookup(key) 涓嶈兘浣跨敤浼犵粺鐨?hlist_for_each_entry_rcu()锛岃€岃浣跨敤甯︽湁棰濆鍐呭瓨灞忛殰锛坰mp_rmb()锛夌殑鐗堟湰銆?
```

  lockless_lookup(key)
  {
    struct hlist_node *node, *next;
    for (pos = rcu_dereference((head)->first);
         pos && ({ next = pos->next; smp_rmb(); prefetch(next); 1; }) &&
         ({ obj = hlist_entry(pos, typeof(*obj), obj_node); 1; });
         pos = rcu_dereference(next))
      if (obj->key == key)
        return obj;
    return NULL;
  }

```
```

  struct hlist_node *node;
  for (pos = rcu_dereference((head)->first);
       pos && ({ prefetch(pos->next); 1; }) &&
       ({ obj = hlist_entry(pos, typeof(*obj), obj_node); 1; });
       pos = rcu_dereference(pos->next))
    if (obj->key == key)
      return obj;
  return NULL;

```
```

  "If the object is moved from one list to another list in-between the
  time the hash is calculated and the next field is accessed, and the
  object has moved to the end of a new list, the traversal will not
  complete properly on the list it should have, since the object will
  be on the end of the new list and there's not a way to tell it's on a
  new list and restart the list traversal. I think that this can be
  solved by pre-fetching the "next" field (with proper barriers) before
  checking the key."

```
### 2) 鎻掑叆绠楁硶


鎴戜滑闇€瑕佺‘淇濊鑰呬笉鑳藉悓鏃惰鍒版柊鐨?'obj->obj_node.next' 鍊煎拰 'obj->key' 鐨勬棫鍊笺€傚惁鍒欙紝涓€涓」鍙兘浠庝竴鏉￠摼涓鍒犻櫎锛屽苟琚彃鍏ュ埌鍙︿竴鏉￠摼涓€傚鏋滅Щ鍔ㄥ墠鏂伴摼涓虹┖锛?next' 鎸囬拡涓?NULL锛屾棤閿佽鑰呭氨鏃犳硶瀵熻鍏堕敊杩囦簡鍘熼摼涓悗缁殑椤广€?
```

  /*
   * Please note that new inserts are done at the head of list,
   * not in the middle or end.
   */
  obj = kmem_cache_alloc(...);
  lock_chain(); // typically a spin_lock()
  obj->key = key;
  atomic_set_release(&obj->refcnt, 1); // key before refcnt
  hlist_add_head_rcu(&obj->obj_node, list);
  unlock_chain(); // typically a spin_unlock()


```
### 3) 鍒犻櫎绠楁硶


杩欓噷娌′粈涔堢壒娈婄殑锛屾垜浠彲浠ヤ娇鐢ㄦ爣鍑嗙殑 RCU hlist 鍒犻櫎銆備絾鐢变簬 SLAB_TYPESAFE_BY_RCU锛岃娉ㄦ剰琚垹闄ょ殑瀵硅薄鍙兘闈炲父蹇氨琚鐢紙鍦?RCU 瀹介檺鏈熺粨鏉熶箣鍓嶏級銆?
```

  if (put_last_reference_on(obj) {
    lock_chain(); // typically a spin_lock()
    hlist_del_init_rcu(&obj->obj_node);
    unlock_chain(); // typically a spin_unlock()
    kmem_cache_free(cachep, obj);
  }



```
--------------------------------------------------------------------------

## 閬垮厤棰濆鐨?smp_rmb()


鍊熷姪 hlist_nulls锛屾垜浠彲浠ラ伩鍏?lockless_lookup() 涓澶栫殑 smp_rmb()銆?
渚嬪锛屽鏋滄垜浠€夋嫨灏嗘Ы浣嶇紪鍙峰瓨鍌ㄤ负鍝堝笇琛ㄦ瘡涓Ы浣嶇殑 'nulls' 閾捐〃缁撴潫鏍囪锛屾垜浠氨鍙互閫氳繃妫€鏌ユ渶缁堢殑 'nulls' 鍊兼潵妫€娴嬬珵浜夛紙鏌愪釜鍐欒€呮墽琛屼簡瀵硅薄鐨勫垹闄ゅ拰/鎴栧皢瀵硅薄绉诲姩鍒板彟涓€鏉￠摼锛夛紝鍓嶆彁鏄煡鎵鹃亣鍒颁簡閾捐〃鐨勬湯灏俱€傚鏋滄渶缁堢殑 'nulls' 鍊间笉鏄Ы浣嶇紪鍙凤紝閭ｄ箞鎴戜滑蹇呴』浠庡ご閲嶆柊寮€濮嬫煡鎵俱€傚鏋滃璞¤绉诲姩鍒板悓涓€鏉￠摼锛屽垯璇昏€呮棤鎵€璋擄細瀹冨彲鑳戒細鍋跺皵鏃犲鍦板啀娆℃壂鎻忚閾捐〃銆?
璇锋敞鎰忥紝浣跨敤 hlist_nulls 鎰忓懗鐫€ 'struct object' 鐨?'obj_node' 瀛楁鐨勭被鍨嬪彉涓?'struct hlist_nulls_node'銆?

### 1) 鏌ユ壘绠楁硶


```

  head = &table[slot];
  begin:
  rcu_read_lock();
  hlist_nulls_for_each_entry_rcu(obj, node, head, obj_node) {
    if (obj->key == key) {
      if (!try_get_ref(obj)) { // might fail for free objects
	rcu_read_unlock();
        goto begin;
      }
      if (obj->key != key) { // not the object we expected
        put_ref(obj);
	rcu_read_unlock();
        goto begin;
      }
      goto out;
    }
  }

  // If the nulls value we got at the end of this lookup is
  // not the expected one, we must restart lookup.
  // We probably met an item that was moved to another chain.
  if (get_nulls_value(node) != slot) {
    put_ref(obj);
    rcu_read_unlock();
    goto begin;
  }
  obj = NULL;

  out:
  rcu_read_unlock();

```
### 2) 鎻掑叆绠楁硶


涓庝笂闈㈢殑鐩稿悓锛屼絾浣跨敤 hlist_nulls_add_head_rcu() 浠ｆ浛 hlist_add_head_rcu()銆?
```

  /*
   * Please note that new inserts are done at the head of list,
   * not in the middle or end.
   */
  obj = kmem_cache_alloc(cachep);
  lock_chain(); // typically a spin_lock()
  obj->key = key;
  atomic_set_release(&obj->refcnt, 1); // key before refcnt
  /*
   * insert obj in RCU way (readers might be traversing chain)
   */
  hlist_nulls_add_head_rcu(&obj->obj_node, list);
  unlock_chain(); // typically a spin_unlock()

```
