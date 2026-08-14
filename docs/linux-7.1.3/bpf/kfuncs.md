

## BPF 内核函数（kfuncs）


## 1. 简介


BPF 内核函数，更常见的叫法是 kfuncs，是 Linux 内核中暴露给 BPF 程序使用的函数。与普通的 BPF 辅助函数（helper）不同，kfuncs 没有稳定的接口，可能从一个内核版本变到另一个版本。因此，BPF 程序需要随着内核的变化而更新。更多信息请参阅 BPF_kfunc_lifecycle_expectations。

## 2. 定义一个 kfunc


有两种方式将内核函数暴露给 BPF 程序：要么让内核中已有的函数可见，要么为 BPF 新增一个包装函数。在这两种情况下，都必须注意 BPF 程序只能在有效的上下文中调用此类函数。为了强制执行此约束，kfunc 的可见性可以是按程序类型划分的。

如果你不是为已有的内核函数创建 BPF 包装函数，请跳到 BPF_kfunc_nodef。

### 2.1 创建一个包装 kfunc


定义包装 kfunc 时，包装函数应当具有 extern 链接。这可以阻止编译器优化掉死代码，因为这个包装 kfunc 在内核自身中并不会被任何地方调用。为包装 kfunc 在头文件中提供原型并不是必需的。

```

        /* Disables missing prototype warnings */
        __bpf_kfunc_start_defs();

        __bpf_kfunc struct task_struct *bpf_find_get_task_by_vpid(pid_t nr)
        {
                return find_get_task_by_vpid(nr);
        }

        __bpf_kfunc_end_defs();

```
当我们需要对 kfunc 的参数进行注解时，通常就需要一个包装 kfunc。否则，可以直接通过向 BPF 子系统注册来让 kfunc 对 BPF 程序可见。请参阅 BPF_kfunc_nodef。

### 2.2 kfunc 参数


现在所有 kfunc 默认都要求受信任（trusted）的参数。这意味着所有指针参数都必须有效，并且所有指向 BTF 对象的指针都必须以未修改的形式（零偏移，且不是通过遍历另一个指针获得的，下文描述的例外除外）传入。

有两种类型的指向内核对象的指针被认为是“受信任的”：

1. 作为 tracepoint 或 struct_ops 回调参数传入的指针。
2. 从 KF_ACQUIRE kfunc 返回的指针。

指向非 BTF 对象（例如标量指针）的指针也可以传给 kfuncs，并且可以具有非零偏移。

“有效”指针的定义随时可能改变，绝对没有任何 ABI 稳定性保证。

如上所述，从遍历受信任指针得到的嵌套指针不再受信任，只有一个例外。如果一个结构体类型有一个字段，只要其父指针有效，该字段就保证有效（受信任或 rcu，如下文 KF_RCU 描述），可以使用以下宏向验证器（verifier）表达这一点：

- `BTF_TYPE_SAFE_TRUSTED`
- `BTF_TYPE_SAFE_RCU`
- `BTF_TYPE_SAFE_RCU_OR_NULL`

例如，


	BTF_TYPE_SAFE_TRUSTED(struct socket) {
		struct sock *sk;
	};

或者


	BTF_TYPE_SAFE_RCU(struct task_struct) {
		const cpumask_t *cpus_ptr;
		struct css_set __rcu *cgroups;
		struct task_struct __rcu *real_parent;
		struct task_struct *group_leader;
	};

换句话说，你必须：

1. 将有效指针类型包裹在 `BTF_TYPE_SAFE_*` 宏中。

2. 指定有效嵌套字段的类型和名称。该字段必须与原始类型定义中的字段完全一致。

由 `BTF_TYPE_SAFE_*` 宏声明的新类型也需要被发出（emit），以便它出现在 BTF 中。例如，`BTF_TYPE_SAFE_TRUSTED(struct socket)`
在 `type_is_trusted()` 函数中按如下方式发出：


	BTF_TYPE_EMIT(BTF_TYPE_SAFE_TRUSTED(struct socket));

### 2.3 注解 kfunc 参数


与 BPF 辅助函数类似，有时验证器需要额外的上下文来使内核函数的使用更安全、更有用。因此，我们可以通过在 kfunc 的参数名后加上一个 __tag 后缀来注解该参数，其中 tag 可以是受支持的注解之一。

### 2.3.1 __sz 注解


此注解用于指示参数列表中的一个“内存与大小”对。
```

        __bpf_kfunc void bpf_memzero(void *mem, int mem__sz)
        {
        ...
        }

```
这里，验证器会将第一个参数视为 PTR_TO_MEM，将第二个参数视为其大小。默认情况下，在没有 __sz 注解时，使用指针所指向类型的大小。没有 __sz 注解时，kfunc 不能接受 void 指针。

### 2.3.2 __k 注解


此注解仅用于标量参数，表示验证器必须检查该标量参数是一个已知的常量，它不表示大小参数，并且该常量的值与程序的安全性相关。

```

        __bpf_kfunc void *bpf_obj_new(u32 local_type_id__k, ...)
        {
        ...
        }

```
这里，bpf_obj_new 使用 local_type_id 参数来查找该程序 BTF 中该类型 ID 的大小，并返回指向它的大小化指针。每个类型 ID 都有不同的大小，因此在验证器状态裁剪（pruning）检查中，当值不匹配时，必须将每次这样的调用视为不同的调用。

因此，只要 kfunc 接受非常量标量参数且该参数不是大小参数，并且常量的值对程序安全至关重要，就应当使用 __k 后缀。

### 2.3.3 __uninit 注解


此注解用于指示该参数将被视为未初始化。

```

        __bpf_kfunc int bpf_dynptr_from_skb(..., struct bpf_dynptr_kern *ptr__uninit)
        {
        ...
        }

```
这里，该 dynptr 将被视为一个未初始化的 dynptr。没有此注解时，如果传入的 dynptr 未初始化，验证器将拒绝该程序。

### 2.3.4 __nullable 注解


此注解用于指示该指针参数可能为 NULL。验证器将允许为此类参数传入 NULL。

```

        __bpf_kfunc void bpf_task_release(struct task_struct *task__nullable)
        {
        ...
        }

```
这里，该 task 指针可能为 NULL。kfunc 负责在解引用该指针之前检查它是否为 NULL。

__nullable 注解可以与其他注解组合使用。例如，当与用于“内存与大小”对的 __sz 或 __szk 注解一起使用时，验证器会在传入 NULL 指针时跳过大小校验，但在提取常量大小信息时仍会处理大小参数（当
```

        __bpf_kfunc void *bpf_dynptr_slice(..., void *buffer__nullable,
                                           u32 buffer__szk)

```
这里，该 buffer 可能为 NULL。如果 buffer 不为 NULL，它的大小必须至少为 buffer__szk 字节。kfunc 负责在使用 buffer 之前检查它是否为 NULL。

### 2.3.5 __str 注解


此注解用于指示该参数是一个常量字符串。

```

        __bpf_kfunc bpf_get_file_xattr(..., const char *name__str, ...)
        {
        ...
        }

```
```

        bpf_get_file_xattr(..., "xattr_name", ...);

```
```

        const char name[] = "xattr_name";  /* This need to be global */
        int BPF_PROG(...)
        {
                ...
                bpf_get_file_xattr(..., name, ...);
                ...
        }

```

### 2.4 使用已有的内核函数


当内核中已有的函数适合被 BPF 程序使用时，它可以直接向 BPF 子系统注册。但是，仍然必须注意审查该函数在被 BPF 程序调用时的上下文，以及这样做是否安全。

### 2.5 注解 kfuncs


除了 kfunc 的参数外，验证器可能还需要更多关于注册到 BPF 子系统的 kfunc 类型的信息。为此，我们定义
```

        BTF_KFUNCS_START(bpf_task_set)
        BTF_ID_FLAGS(func, bpf_get_task_pid, KF_ACQUIRE | KF_RET_NULL)
        BTF_ID_FLAGS(func, bpf_put_pid, KF_RELEASE)
        BTF_KFUNCS_END(bpf_task_set)

```
这个集合编码了上面列出的每个 kfunc 的 BTF ID，并连同标志一起编码。当然，也允许指定没有标志。

kfunc 的定义也应当始终用 `__bpf_kfunc` 宏进行注解。这可以防止诸如编译器在 kfunc 是静态内核函数时将其内联，或者在 LTO 构建中因为它在内核其余部分未被使用而将其剔除等问题。开发者不应手动为自己的 kfunc 添加注解来防止这些问题。如果为了防止此类问题而需要对你的 kfunc 添加注解，那是一个 bug，应当添加到该宏的定义中，以便其他 kfunc 也得到同样的处理
```

        __bpf_kfunc struct task_struct *bpf_get_task_pid(s32 pid)
        {
        ...
        }

```
### 2.5.1 KF_ACQUIRE 标志


KF_ACQUIRE 标志用于指示该 kfunc 返回一个指向带引用计数（refcounted）对象的指针。验证器随后将确保该对象指针最终被一个 release kfunc 释放，或者通过被引用的 kptr（调用 bpf_kptr_xchg）转移到映射中。否则，验证器会拒绝加载该 BPF 程序，直到程序所有可能的探索状态中都没有遗留的引用。

### 2.5.2 KF_RET_NULL 标志


KF_RET_NULL 标志用于指示该 kfunc 返回的指针可能为 NULL。因此，它强制用户在使用（解引用或传递给另一个辅助函数）该 kfunc 返回的指针之前，对其做一次 NULL 检查。此标志常与 KF_ACQUIRE 标志配对使用，但二者彼此正交。

### 2.5.3 KF_RELEASE 标志


KF_RELEASE 标志用于指示该 kfunc 释放传入它的指针。只能传入一个被引用的指针。调用带有此标志的 kfunc 会导致被释放指针的所有副本都失效。

### 2.5.4 KF_SLEEPABLE 标志


KF_SLEEPABLE 标志用于可能休眠（sleep）的 kfuncs。此类 kfunc 只能由可休眠的 BPF 程序（BPF_F_SLEEPABLE）调用。

### 2.5.5 KF_DESTRUCTIVE 标志


KF_DESTRUCTIVE 标志用于指示调用它会破坏系统。例如，这样的调用可能导致系统重启或崩溃。因此，对此类调用有额外的限制。目前它们只需要 CAP_SYS_BOOT 能力，但以后可能会增加更多。

### 2.5.6 KF_RCU 标志


KF_RCU 标志允许 kfunc 选择退出默认受信任参数的要求，并接受具有较弱保证的 RCU 指针。标记了 KF_RCU 的 kfunc 期望 PTR_TRUSTED 或 MEM_RCU 参数。验证器保证对象是有效的且不存在 use-after-free。指针不为 NULL，但对象的引用计数可能已达到零。kfunc 需要考虑做 refcnt != 0 的检查，尤其是在返回 KF_ACQUIRE 指针时。还要注意的是，一个 KF_RCU 的 KF_ACQUIRE kfunc 非常可能也应当同时是 KF_RET_NULL。

### 2.5.7 KF_RCU_PROTECTED 标志


KF_RCU_PROTECTED 标志用于指示该 kfunc 必须在 RCU 临界区中调用。这对不可休眠的程序默认是成立的，对于可休眠的程序，则必须通过调用 `bpf_rcu_read_lock` 来显式保证。

如果该 kfunc 返回一个指针值，此标志还强制要求返回的指针受到 RCU 保护，并且只能在 RCU 临界区处于活动状态时使用。

该标志不同于 `KF_RCU` 标志，后者只保证其参数是至少受 RCU 保护的指针。这可能会传递性地暗示 RCU 保护得到保证，但对于那些需要 RCU 保护但不接受受 RCU 保护参数的 kfunc 来说并不适用。

### 2.5.8 KF_DEPRECATED 标志


KF_DEPRECATED 标志用于那些计划在后续内核版本中被修改或移除的 kfuncs。被标记了 KF_DEPRECATED 的 kfunc 还应当在其内核文档中记录任何相关信息。此类信息通常包括该 kfunc 预期的剩余寿命、对可替代它的新功能的建议（如果有的话），以及可能关于为何要移除它的理由。

注意，尽管在某些情况下，一个 KF_DEPRECATED 的 kfunc 可能继续得到支持并移除其 KF_DEPRECATED 标志，但添加之后要移除 KF_DEPRECATED 标志，很可能比一开始就阻止添加它要困难得多。如 BPF_kfunc_lifecycle_expectations 所述，依赖特定 kfunc 的用户被鼓励尽早让他人知晓他们的使用场景，并在此类讨论发生 upstream 时参与关于是否保留、修改、弃用或移除这些 kfunc 的讨论。

### 2.5.9 KF_IMPLICIT_ARGS 标志


KF_IMPLICIT_ARGS 标志用于指示该 kfunc 的 BPF 签名与其内核签名不同，隐式参数的值在加载时由验证器提供。

只有特定类型的参数是隐式的。目前只支持 `struct bpf_prog_aux *` 类型。

因此带有 KF_IMPLICIT_ARGS 标志的 kfunc 在 BTF 中有两种类型：一种匹配内核声明（按惯例名称带有 _impl 后缀），另一种匹配预期的 BPF API。

验证器只允许调用 kfunc 的非 _impl 版本，即使用不带隐式参数的签名。

示例声明：


	__bpf_kfunc int bpf_task_work_schedule_signal(struct task_struct **task, struct bpf_task_work **tw,
						      void *map__map, bpf_task_work_callback_t callback,
						      struct bpf_prog_aux *aux) { ... }

BPF 程序中的示例用法：


	/** note that the last argument is omitted **/
        bpf_task_work_schedule_signal(task, &work->tw, &arrmap, task_work_callback);

### 2.6 注册 kfuncs


一旦 kfunc 准备好使用，使其可见的最后一步就是向 BPF 子系统注册它。注册是按 BPF 程序类型进行的
```

        BTF_KFUNCS_START(bpf_task_set)
        BTF_ID_FLAGS(func, bpf_get_task_pid, KF_ACQUIRE | KF_RET_NULL)
        BTF_ID_FLAGS(func, bpf_put_pid, KF_RELEASE)
        BTF_KFUNCS_END(bpf_task_set)

        static const struct btf_kfunc_id_set bpf_task_kfunc_set = {
                .owner = THIS_MODULE,
                .set   = &bpf_task_set,
        };

        static int init_subsystem(void)
        {
                return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &bpf_task_kfunc_set);
        }
        late_initcall(init_subsystem);

```
### 2.7 使用 ___init 指定无类型转换的别名


验证器总是强制要求 BPF 程序传给 kfunc 的指针的 BTF 类型，与 kfunc 定义中指定的指针类型相匹配。不过，验证器允许那些根据 C 标准等价、但 BTF_ID 不同的类型被传给同一个 kfunc 参数。

例如，对于以下类型定义：


	struct bpf_cpumask {
		cpumask_t cpumask;
		refcount_t usage;
	};

验证器会允许将 `struct bpf_cpumask *` 传给一个接受 `cpumask_t **`（它是 `struct cpumask **` 的一个 typedef）的 kfunc。例如，`struct cpumask **` 和 `struct bpf_cpmuask **` 都可以传给 bpf_cpumask_test_cpu()。

在某些情况下，这种类型别名行为是不期望的。``struct
nf_conn___init`` 就是这样一个例子：


	struct nf_conn___init {
		struct nf_conn ct;
	};

C 标准会认为这些类型是等价的，但将这两种类型中的任何一种传给一个受信任的 kfunc 并不总是安全的。``struct
nf_conn___init` 表示一个已分配但**尚未初始化**的 `struct nf_conn`` 对象，因此将一个 ``struct
nf_conn___init *` 传给一个期望已完全初始化的 `struct
nf_conn *` 的 kfunc（例如 `bpf_ct_change_timeout()``）是不安全的。

为了满足此类需求，如果两个类型具有完全相同的名称，且其中一个带有 `___init` 后缀，验证器将强制进行严格的 PTR_TO_BTF_ID 类型匹配。


## 3. kfunc 生命周期预期


kfuncs 提供的是内核 <-> 内核 API，因此不受任何与内核 <-> 用户 UAPI 相关的严格稳定性限制约束。这意味着它们可以被认为类似于 EXPORT_SYMBOL_GPL，因此当它们所在子系统的维护者认为有必要时，可以对其进行修改或移除。

与内核的任何其他变更一样，维护者不会在没有合理理由的情况下更改或移除一个 kfunc。他们是否会选择更改一个 kfunc，最终取决于多种因素，例如该 kfunc 的使用广泛程度、它在内核中存在的时间长短、是否存在替代的 kfunc、相关子系统在稳定性方面的惯例，当然还有继续支持该 kfunc 的技术代价。

这有几个含义：

a) 被广泛使用或在内核中存在已久的 kfunc，维护者更难证明其更改或移除的合理性。换句话说，已知有大量用户并提供显著价值的 kfunc，对维护者投入时间和精力去支持它们提供了更强的激励。因此，在 BPF 程序中使用 kfuncs 的开发者，向他人沟通和解释这些 kfunc 是如何以及为何被使用的，并在它们于 upstream 被讨论时参与其中，是很重要的。

b) 与用 EXPORT_SYMBOL_GPL 标记的普通内核符号不同，调用 kfuncs 的 BPF 程序通常不属于内核代码树。这意味着当 kfunc 发生变化时，重构通常无法就地修改调用者，而像上游驱动在内核符号变化时被就地更新那样的做法则不可行。

   与普通内核符号不同，这对 BPF 符号来说是预期的行为，使用 kfuncs 的树外 BPF 程序应当被视为与修改和移除这些 kfuncs 相关的讨论和决策的相关方。BPF 社区将在必要时积极扮演参与 upstream 讨论的角色，以确保此类用户的观点被纳入考虑。

c) kfunc 永远不会有任何硬性稳定性保证。BPF API 不能也不会纯粹出于稳定性原因而硬性阻止内核中的变更。话虽如此，kfuncs 是用来解决问题并为用户提供价值的功能。是否更改或移除一个 kfunc 是一个多变量的技术决策，需视具体情况而定，并参考如上所述的数据点。预期一个 kfunc 在无警告的情况下被移除或更改不会是常见现象，也不会在没有充分理由的情况下发生，但使用 kfuncs 就必须接受这种可能性。

### 3.1 kfunc 弃用


如上所述，虽然有时维护者可能发现必须立即更改或移除一个 kfunc 以适应其子系统中的某些变更，但通常 kfuncs 能够容纳更长、更审慎的弃用过程。例如，如果出现了一个新的、比现有 kfunc 功能更优的 kfunc，现有的 kfunc 可能会被弃用一段时间，以允许用户将他们的 BPF 程序迁移到新的 kfunc 上。或者，如果一个 kfunc 没有已知用户，可能会决定在某个弃用期之后移除该 kfunc（不提供替代 API），以便为用户提供一个窗口，以便在其实际被使用的情况下通知 kfunc 维护者。

预期常见的情况是 kfuncs 会经历一个弃用期，而不是在无警告的情况下被更改或移除。如 KF_deprecated_flag 所述，kfunc 框架提供了 KF_DEPRECATED 标志，供 kfunc 开发者向用户发出某 kfunc 已被弃用的信号。一旦一个 kfunc 被标记了 KF_DEPRECATED，移除时遵循以下流程：

1. 任何与已弃用 kfunc 相关的信息都记录在该 kfunc 的内核文档中。该文档通常包括该 kfunc 预期的剩余寿命、对可替代已弃用函数用法的新功能的建议（或解释为何不存在这样的替代品）等。

2. 已弃用的 kfunc 会在首次被标记为弃用后，在内核中保留一段时间。这段时间的长短将视具体情况而定，通常取决于该 kfunc 的使用广泛程度、它在内核中存在的时间长短，以及迁移到替代方案的难度。这个弃用期是“尽力而为”的，并且如上所述 <BPF_kfunc_lifecycle_expectations>，有时情况可能要求在完整的预期弃用期结束之前就移除该 kfunc。

3. 弃用期结束后，该 kfunc 将被移除。此时，调用该 kfunc 的 BPF 程序将被验证器拒绝。

## 4. 核心 kfuncs


BPF 子系统提供了一批“核心”kfuncs，它们可能适用于各种各样的不同潜在使用场景和程序。这些 kfuncs 在此处记录。

### 4.1 struct task_struct * kfuncs


有一些 kfuncs 允许将 `struct task_struct *` 对象用作 kptr：

   :identifiers: bpf_task_acquire bpf_task_release

当你想要获取或释放对一个作为例如 tracepoint 参数或 struct_ops 回调参数传入的 `struct task_struct *` 的引用时，这些 kfuncs 很有用。例如：


	/**
  - A trivial example tracepoint program that shows how to
  - acquire and release a struct task_struct * pointer.
	 */
	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_acquire_release_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *acquired;

		acquired = bpf_task_acquire(task);
		if (acquired)
			/*
    - In a typical program you'd do something like store
    - the task in a map, and the map will automatically
    - release it later. Here, we release it manually.
			 */
			bpf_task_release(acquired);
		return 0;
	}


在 `struct task_struct *` 对象上获取的引用是受 RCU 保护的。因此，在 RCU 读区域内，你可以获得指向嵌入在映射值中的 task 的指针，而无需获取引用：


	#define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
	private(TASK) static struct task_struct *global;

	/**
  - A trivial example showing how to access a task stored
  - in a map using RCU.
	 */
	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_rcu_read_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *local_copy;

		bpf_rcu_read_lock();
		local_copy = global;
		if (local_copy)
			/*
    - We could also pass local_copy to kfuncs or helper functions here,
    - as we're guaranteed that local_copy will be valid until we exit
    - the RCU read region below.
			 */
			bpf_printk("Global task %s is valid", local_copy->comm);
		else
			bpf_printk("No global task found");
		bpf_rcu_read_unlock();

		/** At this point we can no longer reference local_copy. **/

		return 0;
	}

----

一个 BPF 程序也可以从 pid 查找一个 task。如果调用者没有可以获取引用的、指向 `struct task_struct *` 对象的受信任指针，这会很有用。

   :identifiers: bpf_task_from_pid

下面是一个使用它的例子：


	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_get_pid_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *lookup;

		lookup = bpf_task_from_pid(task->pid);
		if (!lookup)
			/** A task should always be found, as %task is a tracepoint arg. **/
			return -ENOENT;

		if (lookup->pid != task->pid) {
			/* bpf_task_from_pid() looks up the task via its
    - globally-unique pid from the init_pid_ns. Thus,
    - the pid of the lookup task should always be the
    - same as the input task.
			 */
			bpf_task_release(lookup);
			return -EINVAL;
		}

		/* bpf_task_from_pid() returns an acquired reference,
   - so it must be dropped before returning from the
   - tracepoint handler.
		 */
		bpf_task_release(lookup);
		return 0;
	}

### 4.2 struct cgroup * kfuncs


`struct cgroup *` 对象也有获取和释放函数：

   :identifiers: bpf_cgroup_acquire bpf_cgroup_release

这些 kfuncs 的使用方式与 bpf_task_acquire() 和 bpf_task_release() 完全相同，因此我们不再为它们提供示例。

----

其他可用于与 `struct cgroup *` 对象交互的 kfuncs 有 bpf_cgroup_ancestor() 和 bpf_cgroup_from_id()，分别允许调用者访问一个 cgroup 的祖先以及通过其 ID 查找一个 cgroup。二者都返回一个 cgroup kptr。

   :identifiers: bpf_cgroup_ancestor

   :identifiers: bpf_cgroup_from_id

最终，应当更新 BPF，以允许在程序自身中通过一次普通的内存加载来完成这件事。目前没有验证器方面更多的支持，这是不可能的。bpf_cgroup_ancestor() 的用法如下：


	/**
  - Simple tracepoint example that illustrates how a cgroup's
  - ancestor can be accessed using bpf_cgroup_ancestor().
	 */
	SEC("tp_btf/cgroup_mkdir")
	int BPF_PROG(cgrp_ancestor_example, struct cgroup **cgrp, const char **path)
	{
		struct cgroup *parent;

		/** The parent cgroup resides at the level before the current cgroup's level. **/
		parent = bpf_cgroup_ancestor(cgrp, cgrp->level - 1);
		if (!parent)
			return -ENOENT;

		bpf_printk("Parent id is %d", parent->self.id);

		/** Return the parent cgroup that was acquired above. **/
		bpf_cgroup_release(parent);
		return 0;
	}

### 4.3 struct cpumask * kfuncs


BPF 提供了一组可用于查询、分配、变更和销毁 struct cpumask * 对象的 kfuncs。更多细节请参阅 cpumasks-header-label。
