## 为内核对象添加引用计数器（kref

:Author: Corey Minyard <minyard@acm.org>
:Author: Thomas Hellstr枚m <thomas.hellstrom@linux.intel.com>

本文大量内容取自 Greg Kroah-Hartman 2004 OLS 大会上发表的关于 kref 的论文与演讲，可在以下地址找到
  - http://www.kroah.com/linux/talks/ols_2004_kref_paper/Reprint-Kroah-Hartman-OLS2004.pdf
  - http://www.kroah.com/linux/talks/ols_2004_kref_talk/

## 简

kref 让你能够为对象添加引用计数器。如果你的对象在多处被使用并传来传去，而你没有引用计数，那么你的代码几乎肯定是有问题的如果你想要引用计数，kref 就是正确的选择
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

kref 可以出现在数据结构的任何位置
## 初始

你必须在分配 kref 之后对其进行初始化。为此，调用

```
     struct my_data *data;

     data = kmalloc(sizeof(*data), GFP_KERNEL);
     if (!data)
            return -ENOMEM;
     kref_init(&data->refcount);
```

这会kref 中的引用计数设为 1
## kref 规则


一旦你有了一个初始化好的 kref，就必须遵循以下规则
1) 如果你创建了某个指针的非临时副本，尤其是它可能传给另一个执行线程时，你必须

```
       kref_get(&data->refcount);
```

   如果你已经拥有一个指kref 结构体的有效指针（引用计数不可能变为零），则可以无需加锁地执行此操作
```
       kref_put(&data->refcount, data_release);
```

   如果这是对指针的最后一个引用，则会调用释放例程。如果代码从不试图在没有已经持有有效指针的情况下去获取指kref 结构体的有效指针，那么无需加锁即可安全地执行此操作
3) 如果代码试图在没有已经持有有效指针的情况下去获取指向 kref 结构体的引用，它必须串行化访问，使得kref_get() 期间不能发生 kref_put()，且结构体在 kref_get() 期间必须保持有效
例如，如果你分配了一些数据并将其传给另一
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

这样，无论两个线程以何种顺序处理数据，kref_put() 都会负责判断数据何时不再被引用并释放它。kref_get() 不需要加锁，
因为我们已经拥有一个持有其引用计数的有效指针。put 也不需要加锁，因为没有东西会在未持有指针的情况下去获取该数据
在上例中，无论是在成功路径还是错误路径中，kref_put() 都会被调2 次。这是必要的，因为引用计数被 kref_init() kref_get() 各增加了 1 次
注意规则 1 中的“之前”非常关键。你绝不应该

```
	task = kthread_run(more_data_handling, data, "more_data_handling");
	if (task == ERR_PTR(-ENOMEM)) {
		rv = -ENOMEM;
		goto out;
	} else
		/* BAD BAD BAD - get is after the handoff */
		kref_get(&data->refcount);
```

不要自以为了解自己在做什么就使用上述写法。首先，你可能并不清楚自己在做什么。其次，你可能确实清楚自己在做什么（在某些情况下涉及加锁，上述写法可能是合法的）但其他不清楚情况的人可能会修改或复制这段代码。这是糟糕的风格。不要这样做
在某些情况下你可以优get put。例如，如果你已经用完一个对象并将其入队交给其他东西或传递给其他东西，就没有理由

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

最后一条规则（规则 3）是最难处理的。举例来说，假设你有一个由各自kref 的项组成的列表，而你希望获取第一个。你不能直接把第一项从列表中取出并 kref_get()那违反了规则 3，因为你并没有已经持有一个有效指针。你必须添加一个互斥体（或其他锁）
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

kref_put() 的返回值在你不希望在整段释放操作期间持有锁时很有用。假设在上例中你不想在持有锁的情况下调用 kfree()
（因为释放操作有
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

这在你必须调用其他作为释放一部分、可能耗时较长或可能申请同一把锁的例程时更为有用。注意，在释放例程中完成所有工作仍是首选，
因为它更整洁一些
上面的例子也可以使用 kref_get_unless_zero() 来优化，具体
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

这可用于移除 put_entry() kref_put() 周围的互斥锁，但重要的是 kref_get_unless_zero 必须被包裹在与在查找表中找到该项相同的临界区内，
否则 kref_get_unless_zero 可能引用已被释放的内存。注意，未经检查返回值就使用 kref_get_unless_zero 是非法的如果你确定（因为已经持有有效指针）kref_get_unless_zero() 会返true，那么请改用 kref_get()
## Kref 涓?RCU


函数 kref_get_unless_zero 还使得可以将 rcu 用于

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

但请注意，struct kref 成员需要在调用 release_entry_rcu 之后保持有效内存达一RCU 宽限期。这可以通过如上使用 kfree_rcu(entry, rhead) 实现或在使用 kfree 之前调用 synchronize_rcu()，但请注synchronize_rcu() 可能会睡眠相当长的一段时间
## 函数与结构体


