
## 使用 RCU hlist_nulls 保护链表和对

本节介绍如何使用 hlist_nulls 来保护以读为主（read-mostly）的链表以及使用 SLAB_TYPESAFE_BY_RCU 分配的对象
请先阅读 listRCU.rst 中的基础知识
## 使用 'nulls'


使用特殊的标记（称为 'nulls'）是解决下述问题的一种便捷方法
在没'nulls' 的情况下，一个管理通过 SLAB_TYPESAFE_BY_RCU kmem_cache 分配对象的典RCU 链表可以使用以下算法。以下示例假'obj' 是指向此类对象的指针，其类型如下
```

  struct object {
    struct hlist_node obj_node;
    atomic_t refcnt;
    unsigned int key;
  };

```
### 1) 查找算法


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
请注意，lockless_lookup(key) 不能使用传统hlist_for_each_entry_rcu()，而要使用带有额外内存屏障（smp_rmb()）的版本
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
### 2) 插入算法


我们需要确保读者不能同时读到新'obj->obj_node.next' 值和 'obj->key' 的旧值。否则，一个项可能从一条链中被删除，并被插入到另一条链中。如果移动前新链为空next' 指针NULL，无锁读者就无法察觉其错过了原链中后续的项
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
### 3) 删除算法


这里没什么特殊的，我们可以使用标准的 RCU hlist 删除。但由于 SLAB_TYPESAFE_BY_RCU，请注意被删除的对象可能非常快就被复用（RCU 宽限期结束之前）
```

  if (put_last_reference_on(obj) {
    lock_chain(); // typically a spin_lock()
    hlist_del_init_rcu(&obj->obj_node);
    unlock_chain(); // typically a spin_unlock()
    kmem_cache_free(cachep, obj);
  }



```
--------------------------------------------------------------------------

## 避免额外smp_rmb()


借助 hlist_nulls，我们可以避lockless_lookup() 中额外的 smp_rmb()
例如，如果我们选择将槽位编号存储为哈希表每个槽位的 'nulls' 链表结束标记，我们就可以通过检查最终的 'nulls' 值来检测竞争（某个写者执行了对象的删除和/或将对象移动到另一条链），前提是查找遇到了链表的末尾。如果最终的 'nulls' 值不是槽位编号，那么我们必须从头重新开始查找。如果对象被移动到同一条链，则读者无所谓：它可能会偶尔无害地再次扫描该链表
请注意，使用 hlist_nulls 意味着 'struct object' 'obj_node' 字段的类型变'struct hlist_nulls_node'

### 1) 查找算法


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
### 2) 插入算法


与上面的相同，但使用 hlist_nulls_add_head_rcu() 代替 hlist_add_head_rcu()
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
