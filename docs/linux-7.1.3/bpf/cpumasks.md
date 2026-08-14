
## BPF cpumask kfuncs

## 1. 引言

`struct cpumask` 是内核中的一个位图数据结构，其索引反映系统上的 CPU。通常，cpumask 用于跟踪一个任务被亲和到哪些 CPU，但它们也可以用于例如跟踪哪些核心与某个调度域相关联、机器上哪些核心是空闲的，等等。

BPF 为程序提供了一组 kfuncs，可用于分配、变更、查询和释放 cpumask。

## 2. BPF cpumask 对象

BPF 程序可以使用两种不同类型的 cpumask。

### 2.1 ``struct bpf_cpumask *``

`struct bpf_cpumask *` 是由 BPF 代表某个 BPF 程序分配的 cpumask，其生命周期完全由 BPF 控制。这些 cpumask 受 RCU 保护，可以被变更，可以用作 kptr，并且可以安全地转换为 `struct cpumask *`。

### 2.1.1 ``struct bpf_cpumask *`` 生命周期

`struct bpf_cpumask *` 使用以下函数进行分配、获取和释放：

  :identifiers: bpf_cpumask_create

  :identifiers: bpf_cpumask_acquire

  :identifiers: bpf_cpumask_release

例如：


        struct cpumask_map_value {
                struct bpf_cpumask __kptr * cpumask;
        };

        struct array_map {
                __uint(type, BPF_MAP_TYPE_ARRAY);
                __type(key, int);
                __type(value, struct cpumask_map_value);
                __uint(max_entries, 65536);
        } cpumask_map SEC(".maps");

        static int cpumask_map_insert(struct bpf_cpumask *mask, u32 pid)
        {
                struct cpumask_map_value local, *v;
                long status;
                struct bpf_cpumask *old;
                u32 key = pid;

                local.cpumask = NULL;
                status = bpf_map_update_elem(&cpumask_map, &key, &local, 0);
                if (status) {
                        bpf_cpumask_release(mask);
                        return status;
                }

                v = bpf_map_lookup_elem(&cpumask_map, &key);
                if (!v) {
                        bpf_cpumask_release(mask);
                        return -ENOENT;
                }

                old = bpf_kptr_xchg(&v->cpumask, mask);
                if (old)
                        bpf_cpumask_release(old);

                return 0;
        }

        /**
         - 一个示例 tracepoint，展示如何查询任务的 cpumask 并
         - 将其记录为 kptr。
         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(record_task_cpumask, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask *cpumask;
                int ret;

                cpumask = bpf_cpumask_create();
                if (!cpumask)
                        return -ENOMEM;

                if (!bpf_cpumask_full(task->cpus_ptr))
                        bpf_printk("task %s has CPU affinity", task->comm);

                bpf_cpumask_copy(cpumask, task->cpus_ptr);
                return cpumask_map_insert(cpumask, task->pid);
        }

----

### 2.1.1 ``struct bpf_cpumask *`` 作为 kptr

如上所述并举例说明，这些 `struct bpf_cpumask *` 对象也可以存储在映射中并用作 kptr。如果一个 `struct bpf_cpumask *` 在映射中，该引用可以使用 bpf_kptr_xchg() 从映射中移除，或者使用 RCU 机会性地获取：


	/** 包含存储在映射中的 struct bpf_cpumask kptr 的结构体。 **/
	struct cpumasks_kfunc_map_value {
		struct bpf_cpumask __kptr * bpf_cpumask;
	};

	/** 包含 struct cpumasks_kfunc_map_value 表项的映射。 **/
	struct {
		__uint(type, BPF_MAP_TYPE_ARRAY);
		__type(key, int);
		__type(value, struct cpumasks_kfunc_map_value);
		__uint(max_entries, 1);
	} cpumasks_kfunc_map SEC(".maps");

	/** ... **/

	/**
  - 一个简单的示例 tracepoint 程序，展示存储在映射中的
  - struct bpf_cpumask * kptr 如何
  - 可以在 RCU 保护下传递给 kfuncs。
	 */
	SEC("tp_btf/cgroup_mkdir")
	int BPF_PROG(cgrp_ancestor_example, struct cgroup **cgrp, const char **path)
	{
		struct bpf_cpumask *kptr;
		struct cpumasks_kfunc_map_value *v;
		u32 key = 0;

		/** 假设之前已在映射中存储了一个 bpf_cpumask ** kptr。 */
		v = bpf_map_lookup_elem(&cpumasks_kfunc_map, &key);
		if (!v)
			return -ENOENT;

		bpf_rcu_read_lock();
		/** 获取对已存储在映射中的 bpf_cpumask ** kptr 的引用。 */
		kptr = v->cpumask;
		if (!kptr) {
			/* 如果映射中没有 bpf_cpumask，那是因为
    - 我们与另一个在 bpf_map_lookup_elem()
    - 之后、以及我们从映射加载指针之前
    - 用 bpf_kptr_xchg() 移除它的 CPU 发生了竞争。
			 */
			bpf_rcu_read_unlock();
			return -EBUSY;
		}

		bpf_cpumask_setall(kptr);
		bpf_rcu_read_unlock();

		return 0;
	}

----

### 2.2 ``struct cpumask``

`struct cpumask` 是实际包含被查询、变更等的 cpumask 位图的对象。一个 `struct bpf_cpumask` 包装了一个 ``struct cpumask``，这就是为什么将其如此转换是安全的（但请注意，将 `struct cpumask **` 转换为 `struct bpf_cpumask **` 是**不**安全的，验证器会拒绝任何尝试这样做的程序）。

正如我们将在下面看到的，任何变更其 cpumask 参数的 kfunc 都会将 `struct bpf_cpumask *` 作为该参数。任何只是查询 cpumask 的参数则会取一个 `struct cpumask *`。

## 3. cpumask kfuncs

上面我们描述了可用于分配、获取、释放等 `struct bpf_cpumask *` 的 kfunc。本文档的这一节将描述用于变更和查询 cpumask 的 kfunc。

### 3.1 变更 cpumask

一些 cpumask kfunc 是“只读”的，因为它们不变更任何参数，而另一些则变更至少一个参数（这意味着该参数必须是 `struct bpf_cpumask *`，如上所述）。

本节将描述所有变更至少一个参数的 cpumask kfunc。下面 cpumasks-querying-label 描述只读 kfunc。

### 3.1.1 设置和清除 CPU

bpf_cpumask_set_cpu() 和 bpf_cpumask_clear_cpu() 可分别用于在 `struct bpf_cpumask` 中设置和清除一个 CPU：

   :identifiers: bpf_cpumask_set_cpu bpf_cpumask_clear_cpu

这些 kfunc 相当直接，例如可以按如下方式使用：


        /**
         - 一个示例 tracepoint，展示如何查询 cpumask。
         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(test_set_clear_cpu, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask *cpumask;

                cpumask = bpf_cpumask_create();
                if (!cpumask)
                        return -ENOMEM;

                bpf_cpumask_set_cpu(0, cpumask);
                if (!bpf_cpumask_test_cpu(0, cast(cpumask)))
                        /** 不应发生。 **/
                        goto release_exit;

                bpf_cpumask_clear_cpu(0, cpumask);
                if (bpf_cpumask_test_cpu(0, cast(cpumask)))
                        /** 不应发生。 **/
                        goto release_exit;

                /** 像 task->cpus_ptr 这样的 struct cpumask ** 指针也可以被查询。 */
                if (bpf_cpumask_test_cpu(0, task->cpus_ptr))
                        bpf_printk("task %s can use CPU %d", task->comm, 0);

        release_exit:
                bpf_cpumask_release(cpumask);
                return 0;
        }

----

bpf_cpumask_test_and_set_cpu() 和 bpf_cpumask_test_and_clear_cpu() 是互补的 kfunc，允许调用者原子地测试和设置（或清除）CPU：

   :identifiers: bpf_cpumask_test_and_set_cpu bpf_cpumask_test_and_clear_cpu

----

我们也可以使用 bpf_cpumask_setall() 和 bpf_cpumask_clear() 在一次操作中设置和清除整个 `struct bpf_cpumask *` 对象：

   :identifiers: bpf_cpumask_setall bpf_cpumask_clear

### 3.1.2 cpumask 之间的操作

除了在单个 cpumask 中设置和清除单个 CPU 之外，调用者还可以使用 bpf_cpumask_and()、bpf_cpumask_or() 和 bpf_cpumask_xor() 在多个 cpumask 之间执行按位操作：

   :identifiers: bpf_cpumask_and bpf_cpumask_or bpf_cpumask_xor

以下是它们如何使用的示例。请注意，此示例中显示的一些 kfunc 将在下文中更详细地介绍。


        /**
         - 一个示例 tracepoint，展示如何使用
           按位运算符变更（并查询）cpumask。
         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(test_and_or_xor, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask **mask1, **mask2, **dst1, **dst2;

                mask1 = bpf_cpumask_create();
                if (!mask1)
                        return -ENOMEM;

                mask2 = bpf_cpumask_create();
                if (!mask2) {
                        bpf_cpumask_release(mask1);
                        return -ENOMEM;
                }

                // ...安全地创建另外两个 mask... */

                bpf_cpumask_set_cpu(0, mask1);
                bpf_cpumask_set_cpu(1, mask2);
                bpf_cpumask_and(dst1, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_empty((const struct cpumask *)dst1))
                        /** 不应发生。 **/
                        goto release_exit;

                bpf_cpumask_or(dst1, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_test_cpu(0, (const struct cpumask *)dst1))
                        /** 不应发生。 **/
                        goto release_exit;

                if (!bpf_cpumask_test_cpu(1, (const struct cpumask *)dst1))
                        /** 不应发生。 **/
                        goto release_exit;

                bpf_cpumask_xor(dst2, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_equal((const struct cpumask *)dst1,
                                       (const struct cpumask *)dst2))
                        /** 不应发生。 **/
                        goto release_exit;

         release_exit:
                bpf_cpumask_release(mask1);
                bpf_cpumask_release(mask2);
                bpf_cpumask_release(dst1);
                bpf_cpumask_release(dst2);
                return 0;
        }

----

可以使用 bpf_cpumask_copy() 将整个 cpumask 的内容复制到另一个：

   :identifiers: bpf_cpumask_copy

----


### 3.2 查询 cpumask

除了上述 kfunc 之外，还有一组只读 kfunc 可用于查询 cpumask 的内容。

   :identifiers: bpf_cpumask_first bpf_cpumask_first_zero bpf_cpumask_first_and
                 bpf_cpumask_test_cpu bpf_cpumask_weight

   :identifiers: bpf_cpumask_equal bpf_cpumask_intersects bpf_cpumask_subset
                 bpf_cpumask_empty bpf_cpumask_full

   :identifiers: bpf_cpumask_any_distribute bpf_cpumask_any_and_distribute

----

上面已经展示了这些查询 kfunc 的一些示例用法。我们不会在此重复那些示例。但是，请注意，所有上述 kfunc 都在 `tools/testing/selftests/bpf/progs/cpumask_success.c`_ 中进行了测试，所以如果你在寻找更多如何使用它们的示例，请看看那里。

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/progs/cpumask_success.c


## 4. 添加 BPF cpumask kfunc

受支持的 BPF cpumask kfunc 集合（目前）与 include/linux/cpumask.h 中的 cpumask 操作并不是 1 对 1 的匹配。那些 cpumask 操作中的任何一个都可以在需要时轻松地封装到一个新的 kfunc 中。如果你想支持一个新的 cpumask 操作，请随时提交补丁。如果你添加了一个新的 cpumask kfunc，请在此处记录它，并将任何相关的自测试用例添加到 cpumask 自测试套件中。
