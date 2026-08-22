## 内核提供的用户空间辅助函

这些是内核提供的一段用户态代码，可从用户空间在内核内存的固定地址处访问它用于向用户空间提供一些需要内核协助的操作，因为许ARM CPU 缺少原生特或指令。其设想是让这段代码直接在用户模式下执行以获得最佳效率，但它内核对应部分过于紧密耦合，因而不能交由用户库来处理。实际上，这段代码甚可能CPU 而异，取决于可用的指令集，或是否SMP 系统。换言之，内核保留
在不预先警告的情况下根据需要更改这段代码的权利。只有这里所记录的入口点及其
结果被保证是稳定的
这与（但也不排除）一个完整的 VDSO 实现不同，然VDSO 会妨碍一些使用常的汇编技巧，而这些技巧能够实现向这些代码段的高效跳转。而且由于这些代码在返回用户代码之前只使用少量周期，VDSO 间接远调用的开销会给这类极简操作
增加可测量的开销
当用户空间针对足够新的、具备必要原生支持的处理器进行优化时，应当绕过这辅助函数并直接内联实现这些功能（要么由编译器直接发出的代码中，要么作为库
函数实现的一部分），但前提是生成的二进制文件由于在其他地方使用了类似原生指令而已经与更早ARM 处理器不兼容。换言之，如果你的编译代码不会其他目的而使用新的指令，不要仅仅为了不使用这些内核辅助函数就让二进制文件
无法在更早的处理器上运行
随着时间的推移可能会增加新的辅助函数，因此较旧的内核可能缺少较新内核中存在的
某些辅助函数。出于这个原因，程序在假定调用某个特定辅助函数是安全的之前，
必须检__kuser_helper_version 的值（见下文）。该检查理想情况下应只在进启动时执行一次，如果进程所运行的内核版本未提供所需的辅助函数，则应尽早中止
执行
### kuser_helper_version


位置0xffff0ffc

```

  extern int32_t __kuser_helper_version;

```
定义
  该字段包含当前运行内核所实现的辅助函数数量。用户空间可读取它以判断某个
  特定辅助函数的可用性
```

  #define __kuser_helper_version (*(int32_t *)0xffff0ffc)

  void check_kuser_version(void)
  {
	if (__kuser_helper_version < 2) {
		fprintf(stderr, "can't do atomic operations, kernel too old\n");
		abort();
	}
  }

```
注意
  用户空间可假定该字段的值在任何单个进程的生存期内都不会改变。这意味着
  该字段可以在库的初始化阶段或程序启动阶段被读取一次
### kuser_get_tls


位置0xffff0fe0

```

  void * __kuser_get_tls(void);

```
输入
  lr = 返回地址

输出
  r0 = TLS 鍊。
被改写的寄存器：

  none

定义
  获取此前__ARM_NR_set_tls 系统调用设置TLS 值
```

  typedef void * (__kuser_get_tls_t)(void);
  #define __kuser_get_tls (*(__kuser_get_tls_t *)0xffff0fe0)

  void foo()
  {
	void *tls = __kuser_get_tls();
	printf("TLS = %p\n", tls);
  }

```
注意
  - 仅在 __kuser_helper_version >= 1 时有效（自内核版2.6.12 起）
### kuser_cmpxchg


位置0xffff0fc0

```

  int __kuser_cmpxchg(int32_t oldval, int32_t newval, volatile int32_t *ptr);

```
输入
  r0 = oldval
  r1 = newval
  r2 = ptr
  lr = 返回地址

输出
  r0 = 成功码（零或非零  C 标志 = r0 == 0 则置位，r0 != 0 则清
被改写的寄存器：

  r3, ip, flags

定义
  仅当 `**ptr` 等于 oldval 时，才将 newval 原子地存`**ptr`。若 `*ptr`   更改则返回零，否则返回非零。C 标志也会`*ptr` 被更改时置位，以便调用方
  代码进行汇编优化
```

  typedef int (__kuser_cmpxchg_t)(int oldval, int newval, volatile int *ptr);
  #define __kuser_cmpxchg (*(__kuser_cmpxchg_t *)0xffff0fc0)

  int atomic_add(volatile int *ptr, int val)
  {
	int old, new;

	do {
		old = *ptr;
		new = old + val;
	} while(__kuser_cmpxchg(old, new, ptr));

	return new;
  }

```
注意
  - 该例程已根据需要包含了内存屏障
  - 仅在 __kuser_helper_version >= 2 时有效（自内核版2.6.12 起）
### kuser_memory_barrier


位置0xffff0fa0

```

  void __kuser_memory_barrier(void);

```
输入
  lr = 返回地址

输出
  none

被改写的寄存器：

  none

定义
  施加任何所需的内存屏障，以与手动修改的数据及 __kuser_cmpxchg 的使用保  一致性
```

  typedef void (__kuser_dmb_t)(void);
  #define __kuser_dmb (*(__kuser_dmb_t *)0xffff0fa0)

```
注意
  - 仅在 __kuser_helper_version >= 3 时有效（自内核版2.6.15 起）
### kuser_cmpxchg64


位置0xffff0f60

```

  int __kuser_cmpxchg64(const int64_t *oldval,
                        const int64_t *newval,
                        volatile int64_t *ptr);

```
输入
  r0 = 指向 oldval 的指  r1 = 指向 newval 的指  r2 = 指向目标值的指针
  lr = 返回地址

输出
  r0 = 成功码（零或非零  C 标志 = r0 == 0 则置位，r0 != 0 则清
被改写的寄存器：

  r3, lr, flags

定义
  仅当 `*ptr` 等于 `**oldval` 所指向64 位值时，才`**newval` 所指向64   值原子地存入 `**ptr`。若 `**ptr` 被更改则返回零，否则返回非零
  C 标志也会`*ptr` 被更改时置位，以便调用方代码进行汇编优化
```

  typedef int (__kuser_cmpxchg64_t)(const int64_t *oldval,
                                    const int64_t *newval,
                                    volatile int64_t *ptr);
  #define __kuser_cmpxchg64 (*(__kuser_cmpxchg64_t *)0xffff0f60)

  int64_t atomic_add64(volatile int64_t *ptr, int64_t val)
  {
	int64_t old, new;

	do {
		old = *ptr;
		new = old + val;
	} while(__kuser_cmpxchg64(&old, &new, ptr));

	return new;
  }

```
注意
  - 该例程已根据需要包含了内存屏障
  - 由于该序列较长，它跨2 个常规的 kuser “槽（slot）”，因此 0xffff0f80
    不作为有效入口点使用
  - 仅在 __kuser_helper_version >= 5 时有效（自内核版3.1 起）