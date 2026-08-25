

## 基于编译器的上下文分析（Context Analysis


上下文分析（Context Analysis）是一种语言扩展，它通过获取与释放用户可定义的“上下文锁”（context lock）来静态检查所需的上下文是否处于激活（或非激活）状态。一个明显的应用是对内核各种同步原语（每一个都代表一个“上下文锁”）进行锁安全性检查，并检查是否违反了加锁规则

Clang 编译器目前支持完整的上下文分

```

    CONFIG_WARN_CONTEXT_ANALYSIS=y

```
该特性需Clang 22 或更高版本

该分析默认是**选择性启用（opt-in*的，并且需要声明哪些模块以

```

    CONTEXT_ANALYSIS_mymodule.o := y

```
```

    CONTEXT_ANALYSIS := y

```
不过，也可以在整个代码树上启用该分析，这将导

```

    CONFIG_WARN_CONTEXT_ANALYSIS_ALL=y

```
### 编程模型


下面描述围绕使用上下文锁类型的编程模型

   启用上下文分析可以看作是在启用一种带有“上下文系统（Context System）”的 Linux C 方言。一些涉及复杂控制流的有效模式会受到约束（例如在同一函数内进行条件获取以及随后条件释放）

上下文分析是一种将操作的可允许性指定为依赖于是否持有（或未持有）上下文锁的方法。通常，我们的目标是通过要求某个特定上下文处于激活状态来保护临界区中的数据与代码，例如持有某个特定的锁。该分析确保调用者在没有所需上下文处于激活状态的情况下无法执行某项操作

上下文锁与具名的 struct 相关联，同时也与那些操作 struct 实例以获取和释放相应上下文锁的函数相关联

上下文锁既可以被独占持有，也可以被共享持有。这种机制允许在上下文激活时赋予更精确的权限，通常用于区分线程在某个上下文中只能读取（共享）还是也能写入（独占）受保护的数据

在给定的线程中、在程序执行的某个特定时刻实际处于激活状态的上下文集合，是一个运行时概念。静态分析通过计算该集合的一个近似（称为上下文环境，context environment）来工作。上下文环境针对每一个程序点进行计算，并描述在该特定点上静态已知为激活或非激活的上下文集合。这个环境是对线程在运行时实际会激活的完整上下文集合的一个保守近似

更多细节也记录在 `here
<https://clang.llvm.org/docs/ThreadSafetyAnalysis.html>`_銆。

   Clang 的分析明确地不会推断由内联函数获取或释放的上下文锁。它需要显式注解来 (a) 断言当某个上下文锁被释放或获取时这不是一bug，以(b) 保持内联与非内联函数声明之间的一致性

#### 内核支持的同步原


目前支持以下同步原语
`raw_spinlock_t`, `spinlock_t`, `rwlock_t`, `mutex`, `seqlock_t`,
`bit_spinlock`, RCU, SRCU (`srcu_struct`), `rw_semaphore`, `local_lock_t`,
`ww_mutex`銆。

要使用初始化函数（`type_init(&lock)`）初始化受上下文锁保护的变量，建议优先使`guard(type_init)(&lock)` 
`scoped_guard(type_init, &lock) { ... }` 在外层作用域中初始化此类受保护的成员或全局变量。这会初始化上下文锁，并将该上下文视为在初始化作用域内处于激活状态（初始化意味着对底层对象拥有独占访问权）

```

    struct my_data {
            spinlock_t lock;
            int counter __guarded_by(&lock);
    };

    void init_my_data(struct my_data *d)
    {
            ...
            guard(spinlock_init)(&d->lock);
            d->counter = 0;
            ...
    }

```
另外，初始化受保护变量也可以在禁用上下文分析的情况下进行，最好是在尽可能小的作用域内（因为缺少任何其他检查）：既可以使用 `context_unsafe(var = init)` 表达式，也可以通过 `__context_unsafe(init)` 属性来标记小型初始化函数

Lockdep 断言（例`lockdep_assert_held()`）会告知编译器的上下文分析：在断言之后，相关的同步原语已被持有。这可以避免在复杂控制流场景中出现误报，并在静态分析能力有限的地方鼓励使用 Lockdep。例如，当一个函数并*总是**需要持锁时，这就很有用，因为此`__must_hold()` 并不合适

#### 鍏抽敭瀛。


   :identifiers: context_lock_struct
                 token_context_lock token_context_lock_instance
                 __guarded_by __pt_guarded_by
                 __must_hold
                 __must_not_hold
                 __acquires
                 __cond_acquires
                 __releases
                 __must_hold_shared
                 __acquires_shared
                 __cond_acquires_shared
                 __releases_shared
                 __acquire
                 __release
                 __acquire_shared
                 __release_shared
                 __acquire_ret
                 __acquire_shared_ret
                 context_unsafe
                 __context_unsafe
                 disable_context_analysis enable_context_analysis

   `__no_context_analysis` 函数属性保留给上下文锁类型的内部实现使用，在普通代码中应避免使用

### 背景


Clang 最初将这一特性称`Thread Safety Analysis
<https://clang.llvm.org/docs/ThreadSafetyAnalysis.html>`_，部分关键字与文档仍在使用仅针对线程安全的术语。后来这一特性被改动并变得更加灵活，获得了定义自定义“能力（capabilities）”的能力。其基础可以`Capability
Systems <https://www.cs.cornell.edu/talc/papers/capabilities.pdf>`_ 中找到，它用于指定操作的可允许性依赖于某个“能力”被持有（或未持有）

由于该特性不仅能表达与同步原语相关的能力，而“capability”在内核中已有其他含义，内核所选的命名因此偏离Clang 最初“Thread Safety”与“capability”的术语；我们将其称为“Context Analysis”以避免混淆。内部实现仍会在少数地方引用 Clang 的术语，例如 `-Wthread-safety` 仍是同样会出现在诊断信息中的警告选项
