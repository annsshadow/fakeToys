## 防死锁的缠绕/等待（Wound/Wait）互斥锁设计


请先阅读 mutex-design.rst，因为它同样适用于等待/缠绕（wait/wound）互斥锁。

### WW-Mutex 的动机

GPU 执行的操作通常涉及许多缓冲区。这些缓冲区可以在不同的上下文/进程之间共享，存在于不同的内存域（例如 VRAM 与系统内存）中，等等。借助 PRIME / dmabuf，它们甚至可以在设备之间共享。因此存在若干情况，驱动需要等待缓冲区就绪。如果你从等待一个缓冲区互斥锁变为可用这个角度来考虑，这就带来了一个问题：因为无法保证缓冲区在所有上下文中以相同的顺序出现在 execbuf/batch 中。这完全由用户空间控制，是应用程序所发出的 GL 调用序列的结果。这将导致潜在的死锁。当你考虑到内核可能需要在 GPU 操作缓冲区之前，将缓冲区迁移（migrate）到 VRAM，而这反过来又可能需要驱逐（evict）其他一些缓冲区（而你不想驱逐那些已经排队等待 GPU 的其他缓冲区）时，问题变得更加复杂；不过为了对问题有简化的理解，你可以忽略这一点。

TTM 图形子系统为处理这个问题而提出的算法相当简单。对于每一组需要加锁的缓冲区（execbuf），调用方会从一个全局计数器获得一个唯一的保留 ID/票据（reservation id/ticket）。如果在锁定与某个 execbuf 关联的所有缓冲区时发生死锁，则保留票据最小（即最旧的任务）的那个获胜，而保留 ID 较大（即较年轻的任务）的那个，会解锁它已经锁定的所有缓冲区，然后重试。

在 RDBMS 文献中，保留票据与一个事务（transaction）相关联。而死锁处理方法被称为 Wait-Die（等待-死亡）。该名称基于一个加锁线程在遇到已被锁定的互斥锁时所采取的行动。如果持有锁的事务更年轻，则加锁事务等待（waits）。如果持有锁的事务更年长，则加锁事务退让（backs off）并消亡（dies）。因此称为 Wait-Die。还有另一种算法称为 Wound-Wait（缠绕-等待）：如果持有锁的事务更年轻，则加锁事务缠绕（wounds）持有锁的事务，请求其消亡。如果持有锁的事务更年长，则它等待另一个事务。因此称为 Wound-Wait。这两种算法都是公平的，因为事务最终都会成功。然而，通常认为 Wound-Wait 算法相比 Wait-Die 产生的退让次数更少，但另一方面，在从退让中恢复时，它伴随着比 Wait-Die 更多的工作。Wound-Wait 也是一种抢占式（preemptive）算法，因为事务会被其他事务缠绕，这需要一个可靠的方式来获取被缠绕的条件并抢占正在运行的事务。注意，这与进程抢占（process preemption）不是一回事。Wound-Wait 事务在它因被缠绕而消亡（返回 -EDEADLK）时，被视为被抢占。

### 概念


相比于普通互斥锁，w/w 互斥锁的锁接口中出现了两个额外的概念/对象：

获取上下文（Acquire context）：为了确保最终能够向前推进，尝试获取锁的任务不要去获取一个新的保留 ID 是很重要的，而是保留它在开始获取锁时所获得的那个。该票据存储在获取上下文中。此外，获取上下文还跟踪调试状态，以捕获对 w/w 互斥锁接口的滥用。一个获取上下文表示一个事务。

w/w 类（w/w class）：与普通互斥锁不同，锁类对于 w/w 互斥锁必须是显式的，因为初始化获取上下文需要用到它。锁类还指定了要使用哪种算法——Wound-Wait 还是 Wait-Die。

此外还有三类不同的 w/w 锁获取函数：

- 使用上下文进行正常锁获取，使用 ww_mutex_lock。

- 在争用的锁上进行慢速路径（slowpath）锁获取，由刚刚杀掉其事务、并已丢弃所有已获取锁的任务使用。这些函数带有 _slow 后缀。

  从简单的语义角度来看，_slow 函数并非严格必需，因为在争用的锁上简单地调用正常的 ww_mutex_lock 函数（在丢弃所有其他已获取的锁之后）也能正确工作。毕竟，如果尚未获取任何其他 w/w 互斥锁，就不存在死锁的可能，因此 ww_mutex_lock 调用会阻塞，而不会提前返回 -EDEADLK。_slow 函数的优势在于接口安全性：

  - ww_mutex_lock 具有 __must_check int 返回类型，而 ww_mutex_lock_slow 具有 void 返回类型。注意，由于 w/w 互斥锁代码无论如何都需要循环/重试，__must_check 不会导致虚假的警告，即使第一次锁操作绝不会失败。
  - 当启用完整调试时，ww_mutex_lock_slow 会检查所有已获取的 w/w 互斥锁都已被释放（以防止死锁），并确保我们阻塞在争用的锁上（以防止在争用的锁可被获取之前，通过 -EDEADLK 慢速路径自旋）。

- 只获取单个 w/w 互斥锁的函数，其语义与普通互斥锁完全相同。这是通过以 NULL 上下文调用 ww_mutex_lock 来实现的。

  同样，这也不是严格必需的。但通常你只想获取单个锁，这种情况下建立获取上下文没有意义（因此也最好避免获取一个死锁避免票据）。

当然，所有用于处理因信号而唤醒的常用变体也同样提供。

### 用法


算法（Wait-Die 与 Wound-Wait）是通过使用 DEFINE_WW_CLASS()（Wound-Wait）或 DEFINE_WD_CLASS()（Wait-Die）来选择的。作为粗略的经验法则，如果你预期同时竞争的（competing）事务数量通常较小，并且希望减少回滚（rollback）次数，则使用 Wound-Wait。

在同一个 w/w 类中获取锁有三种不同的方式。常见的
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
方法 1，使用 execbuf->buffers 中一个不允许重排序的列表。如果你已经在某处跟踪了所需对象的列表，这会很有用。此外，锁辅助函数可以利用 -EALREADY 返回码向调用方传播一个信号：某个对象在列表上出现了两次。如果从用户空间输入构建列表，并且 ABI 要求用户空间
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
方法 2，使用 execbuf->buffers 中一个可以重排序的列表。与方法 1 一样，使用 -EALREADY 进行重复条目检测的语义相同。但是
```

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
方法 3 在对象列表是临时（ad-hoc）构建而非预先构建时很有用，例如当调整一个图中的边时，其中每个节点都有它自己的 ww_mutex 锁，并且边只有在持有所有相关节点的锁时才能更改。w/w 互斥锁天然适合这种情况，原因有二：

- 它们能以任意顺序处理锁获取，这使我们能够从一个起点开始遍历图，然后迭代地发现新的边，并锁定这些边所连接的节点。
- 由于 -EALREADY 返回码表示某个给定对象已被持有，因此无需额外的簿记（book-keeping）来打破图中的环，也无需跟踪哪些锁已被持有（当使用多个节点作为起点时）。

注意，这种方法与上述方法在两个重要方面有所不同：

- 由于对象列表是动态构建的（并且在因碰到 -EDEADLK 消亡条件而重试时很可能不同），当某个对象未被锁定时，没有必要将其保留在持久列表中。因此我们可以将 list_head 移入对象自身中。
- 另一方面，动态对象列表构建也意味着 -EALREADY 返回码无法被传播。

还要注意，方法 #1 和方法 #2 以及方法 #3 可以组合使用，例如首先使用上述某一种方法锁定一组起始节点（从用户空间传入）。然后使用下面的方法 #3 锁定受操作影响的任何其他对象。回退/重试过程会稍微复杂一些，因为当动态锁定步骤碰到 -EDEADLK 时，我们还需要解锁用固定列表获取的所有对象。但 w/w 互斥锁的调试检查会捕获这些情况下的任何接口误用。

此外，方法 3 不会使锁获取步骤失败，因为它不返回 -EALREADY。当然，当使用 _interruptible 时会有所不同
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
方法 4：只锁一个对象。在这种情况下，死锁检测与预防显然有些过度，因为只获取一个锁时，不可能在同一个类内产生死锁。为了简化这种情况，w/w 互斥锁 API 可以与 NULL 上下文一起使用。

### 实现细节


##### 设计：


  ww_mutex 目前封装了一个 struct mutex，这意味着对普通互斥锁加锁没有额外开销，而普通加锁要常见得多。因此如果不使用等待/缠绕互斥锁，代码大小只会有很小的增加。

  我们为等待列表（wait list）维护以下不变量：

  (1) 带有获取上下文的等待者按 stamp 顺序排序；不带获取上下文的等待者按 FIFO 顺序穿插其中。
  (2) 对于 Wait-Die，在带有上下文的等待者中，只有第一个可以已经获取了其他锁（ctx->acquired > 0）。注意，这个等待者可能在列表中排在其他不带上下文的等待者之后。

  Wound-Wait 的抢占是通过一种惰性抢占（lazy-preemption）方案实现的：仅在出现对新锁的竞争、因此存在真正的死锁可能时，才会检查事务的被缠绕（wounded）状态。在那种情况下，如果事务被缠绕，它就会退让，清除被缠绕状态并重试。以这种方式实现抢占的一大好处是，被缠绕的事务可以在重启事务之前，识别出一个要等待的争用锁。盲目地重启事务很可能会使事务最终又陷入需要再次退让的境地。

  一般来说，预期竞争不会太多。这些锁通常用于序列化对设备资源的访问，因此优化重点应放在无竞争（uncontended）的情况上。

##### Lockdep：


  我们特别小心地尽可能多地警告 API 滥用的情况。一些常见的 API 滥用会被 CONFIG_DEBUG_MUTEXES 捕获，但推荐使用 CONFIG_PROVE_LOCKING。

  会被警告的一些错误：
   - 忘记调用 ww_acquire_fini 或 ww_acquire_init。
   - 试图在 ww_acquire_done 之后锁定更多互斥锁。
   - 试图在 -EDEADLK 之后、并在解锁所有互斥锁之前锁定错误的互斥锁。
   - 试图在 -EDEADLK 之后、并在解锁所有互斥锁之前锁定正确的互斥锁。

   - 在返回 -EDEADLK 之前调用 ww_mutex_lock_slow。

   - 用错误的解锁函数解锁互斥锁。
   - 在同一个上下文上两次调用某个 ww_acquire_* 函数。
   - 对互斥锁使用了与 ww_acquire_ctx 不同的 ww_class。
   - 可能导致死锁的普通 lockdep 错误。

  可能导致死锁的一些 lockdep 错误：
   - 在对第一个 ww_acquire_ctx 调用 ww_acquire_fini 之前，调用 ww_acquire_init 来初始化第二个 ww_acquire_ctx。
   - 可能发生的"普通"死锁。

FIXME:
  一旦我们实现了 TASK_DEADLOCK 任务状态标志的魔法，就更新本节。
