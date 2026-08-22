
## RCU 保护的列数组元素的引用计数设



请注意，如果你需要将引用计数RCU 结合，percpu-ref 特性很可能是你的首选
请参include/linux/percpu-refcount.h 了解更多信息。然而，percpu-ref 
消耗过多内存的罕见情况下，请继续往下读

------------------------------------------------------------------------

对于受传统读写者自旋锁或信号量保护的列表元素进行引用计数是直截了当的：

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
如果像下面这样用 RCU 把该列表/数组变成无锁：在 add() delete() 中把
write_lock() 改为 spin_lock()，并search_and_reference() 中的 read_lock()
改为 rcu_read_lock()，那search_and_reference() 中的 atomic_inc() 有可能持
一个已经从列表/数组中删除的元素的引用。在这种情况下请使用 atomic_inc_not_zero()
如下所示：

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
有时需要在更新（写）路径中获取元素的引用。在这种情况下，atomic_inc_not_zero()
可能有些过度，因为我们持有更新侧的自旋锁。此时可以改atomic_inc()

search_and_reference() 代码路径中处"FAIL" 并不总是方便。在这种情况下，
可以atomic_dec_and_test() delete() 移到 el_free() 中，如下所示：

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
关键点在于，add() 添加的初值引用，要等到移除之后的一个宽限期过去之后才会
移除。这意味着 search_and_reference() 找不到该元素，也就是el->rc 的值无
增加。因此，一旦它降到零，就不存在任何能够、或将能够引用该元素的读者。该元素
因此可以被安全地释放。这反过来保证了：如果任何读者找到了该元素，该读者可
在不检查引用计数值的情况下安全地获取一个引用

相比于清B 中的模式，清C 中基RCU 的模式有一个明显优势：任何定位到某
给定对象search_and_reference() 调用，即便该同一对象delete() 正在并发调用
也都能成功获取该对象的引用。类似地，清B C 相比于清A 的一个明显优势是
即便有任意大量的 search_and_reference() 调用在查delete() 所作用的同一对象
delete() 的调用也不会被延迟。相反，被延迟的仅仅kfree() 的最终调用，而在现代
计算机系统（即便是小型的）上这通常不是问题

delete() 可以休眠的情况下，synchronize_rcu() 可以
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
作为内核中的更多例子，清C 中的模式用于 struct pid 的引用计数，而清B 中的
模式用于 struct posix_acl
