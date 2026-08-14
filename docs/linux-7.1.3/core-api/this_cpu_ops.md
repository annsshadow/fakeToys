## this_cpu 操作


:Author: Christoph Lameter, August 4th, 2014
:Author: Pranith Kumar, Aug 2nd, 2014

this_cpu 操作是一种优化对与**当前**执行处理器相关联的每 CPU 变量
访问的方式。这是通过使用段寄存器（或 cpu 永久存储特定处理器每 CPU
区域起始地址的专用寄存器）来实现的。

this_cpu 操作将每 CPU 变量的偏移加到处理器特定的每 CPU 基址上，并将
该操作编码到操作该每 CPU 变量的指令中。

这意味着在计算偏移和对数据进行操作之间没有原子性问题。因此，不需要
禁用抢占或中断来确保处理器不会在计算地址和操作数据之间被改变。

读-修改-写操作尤其值得关注。处理器通常具有特殊的低延迟指令，可以在
没有典型同步开销的情况下运行，同时仍提供一些宽松的原子性保证。例如，
x86 可以在不使用 lock 前缀及相关延迟代价的情况下执行 RMW（读-修改-写）
指令，如 inc/dec/cmpxchg。

没有 lock 前缀的变量访问并未同步，但由于我们处理的是与当前执行处理器
相关的每 CPU 数据，因此同步并非必要。只有当前处理器应当访问该变量，
因此不存在与其他处理器之间的并发问题。

请注意，远程处理器对每 CPU 区域的访问属于异常情况，可能会通过
this_cpu_* 影响本地 RMW 操作的性能和/或正确性（远程写操作）。

this_cpu 操作的主要用途是优化计数器操作。

定义了以下隐含抢占保护的 this_cpu() 操作。这些操作可以放心使用，无需
担心
```

	this_cpu_read(pcp)
	this_cpu_write(pcp, val)
	this_cpu_add(pcp, val)
	this_cpu_and(pcp, val)
	this_cpu_or(pcp, val)
	this_cpu_add_return(pcp, val)
	this_cpu_xchg(pcp, nval)
	this_cpu_cmpxchg(pcp, oval, nval)
	this_cpu_sub(pcp, val)
	this_cpu_inc(pcp)
	this_cpu_dec(pcp)
	this_cpu_sub_return(pcp, val)
	this_cpu_inc_return(pcp)
	this_cpu_dec_return(pcp)


```
### this_cpu 操作的内部工作机制


在 x86 上，fs: 或 gs: 段寄存器包含每 CPU 区域的基址。因此只需使用段
覆盖前缀，即可将每 CPU 相对地址重定位到该处理器对应的每 CPU 区域。
也就是说，到每 CPU 基址的重定位是通过段寄存器前缀编码在指令中的。

```

	DEFINE_PER_CPU(int, x);
	int z;

	z = this_cpu_read(x);

```
```

	mov ax, gs:[x]

```
而非先计算出地址再从该地址取值（这正是每 CPU 操作的做法）。在
this_cpu_ops 出现之前，此类序列还需要 preempt disable/enable，以防止
内核在计算过程中将线程迁移到不同的处理器。

```

	this_cpu_inc(x)

```
```

	inc gs:[x]

```
而非在没有段寄存器时必须执行的下列操作
```

	int *y;
	int cpu;

	cpu = get_cpu();
	y = per_cpu_ptr(&x, cpu);
	(*y)++;
	put_cpu();

```
请注意，这些操作只能用于为特定处理器保留的每 CPU 数据。如果不禁用
周围代码的抢占，this_cpu_inc() 仅保证某个每 CPU 计数器被正确地递增。
然而，无法保证 OS 不会在 this_cpu 指令执行之前或之后直接将进程迁移。
通常这意味着每个处理器各自计数器的值是没有意义的。所有每 CPU 计数器
的总和才是唯一有意义的值。

出于性能原因使用每 CPU 变量。如果多个处理器并发地经过相同的代码路径，
就可以避免缓存行弹跳（bouncing）。由于每个处理器都有自己的每 CPU 变量，
不会发生并发的缓存行更新。为此优化所付出的代价是，在需要某个计数器
的值时必须将各个每 CPU 计数器累加求和。


### 特殊操作


```

	y = this_cpu_ptr(&x)

```
取一个每 CPU 变量的偏移（&x !）并返回属于当前执行处理器的该每 CPU
变量的地址。this_cpu_ptr 避免了常见的 get_cpu/put_cpu 序列所需的多个
步骤。这里没有可用的处理器编号。相反，只是将本地每 CPU 区域的偏移
简单地加到每 CPU 偏移上。

请注意，该操作只能用于可以使用 smp_processor_id() 的代码段中，例如
已禁用抢占的地方。随后该指针用于在临界区中访问本地每 CPU 数据。当
重新启用抢占时，该指针通常不再有用，因为它可能不再指向当前处理器的
每 CPU 数据。

在可抢占代码中有意义地获取每 CPU 指针的特殊情况由 raw_cpu_ptr()
处理，但此类用例需要处理两个不同 CPU 访问同一个每 CPU 变量的情况，
而这个变量很可能是第三个 CPU 的。这些用例通常是性能优化。例如，SRCU
将一对计数器实现为一对每 CPU 变量，而 rcu_read_lock_nmisafe() 使用
raw_cpu_ptr() 获取指向某个 CPU 计数器的指针，并使用 atomic_inc_long()
来处理 raw_cpu_ptr() 与 atomic_inc_long() 之间的迁移。

### 每 CPU 变量与偏移


每 CPU 变量相对于每 CPU 区域起始位置具有**偏移**。虽然它们在代码中
看起来像地址，但实际上并没有地址。偏移不能直接解引用。必须将偏移
加到某个处理器的每 CPU 区域基址指针上，才能构成一个有效的地址。

因此，在每 CPU 操作的上下文之外使用 x 或 &x 是无效的，通常会被当作
NULL 指针解引用来处理。

```

	DEFINE_PER_CPU(int, x);

```
在每 CPU 操作的上下文中，上述代码意味着 x 是一个每 CPU 变量。大多数
this_cpu 操作接受一个 CPU 变量。

```

	int __percpu *p = &x;

```
&x 以及因此 p 是一个每 CPU 变量的**偏移**。this_cpu_ptr() 接受一个
每 CPU 变量的偏移，这使得它看起来有点奇怪。


### 对每 CPU 结构体字段的操作


```

	struct s {
		int n,m;
	};

	DEFINE_PER_CPU(struct s, p);


```
```

	this_cpu_inc(p.m)

	z = this_cpu_cmpxchg(p.m, 0, 1);


```
```

	struct s __percpu *ps = &p;

	this_cpu_dec(ps->m);

	z = this_cpu_inc_return(ps->n);


```
指针的计算可能需要使用 this_cpu_ptr()
```

	struct s *pp;

	pp = this_cpu_ptr(&p);

	pp->m--;

	z = pp->n++;


```
### this_cpu 操作的变体


this_cpu 操作是中断安全的。某些架构不支持这些每 CPU 本地操作。在这种
情况下，必须用禁用中断、然后执行保证原子的操作、再重新启用中断的代码
来替代。这样做代价高昂。如果没有其他原因导致调度器不能改变我们所在的
处理器，就没有理由禁用中断。为此目的，提供了下列 __this_cpu 操作。

这些操作不保证能防御并发中断或抢占。如果每个 CPU 变量不在中断上下文中
使用，且调度器不能抢占，那么它们是安全的。如果在某个操作进行期间仍发生
了中断，并且该中断也修改了该变量，那么 RMW 动作无法保证是
```

	__this_cpu_read(pcp)
	__this_cpu_write(pcp, val)
	__this_cpu_add(pcp, val)
	__this_cpu_and(pcp, val)
	__this_cpu_or(pcp, val)
	__this_cpu_add_return(pcp, val)
	__this_cpu_xchg(pcp, nval)
	__this_cpu_cmpxchg(pcp, oval, nval)
	__this_cpu_sub(pcp, val)
	__this_cpu_inc(pcp)
	__this_cpu_dec(pcp)
	__this_cpu_sub_return(pcp, val)
	__this_cpu_inc_return(pcp)
	__this_cpu_dec_return(pcp)


```
会递增 x，并且在无法通过地址重定位与同一条指令中的读-修改-写操作实现
原子性的平台上，不会回退到禁用中断的代码。


### &this_cpu_ptr(pp)->n 与 this_cpu_ptr(&pp->n) 的区别


第一种操作取偏移并形成一个地址，然后加上 n 字段的偏移。这可能会
导致编译器发出两条 add 指令。

第二种操作先加上两个偏移，再进行重定位。依我看，第二种形式看起来
更清晰，并且在处理 () 时更轻松。第二种形式也与 this_cpu_read()
及其同类用法保持一致。


### 远程访问每 CPU 数据


每 CPU 数据结构设计为由一个 CPU 独占使用。如果按预期使用这些变量，
this_cpu_ops() 保证是"原子的"，因为没有其他 CPU 能访问这些数据结构。

有些特殊情况可能需要远程访问每 CPU 数据结构。远程读访问通常是安全的，
并且经常用于汇总计数器。远程写访问则可能成问题，因为 this_cpu 操作
没有锁语义。远程写可能会干扰 this_cpu 的 RMW 操作。

除非绝对必要，否则强烈不建议远程写入 percpu 数据结构。请考虑使用 IPI
来唤醒远程 CPU 并对其每 CPU 区域执行更新。

要远程访问每 CPU 数据结构，通常使用 per_cpu_ptr()
```

	DEFINE_PER_CPU(struct data, datap);

	struct data *p = per_cpu_ptr(&datap, cpu);

```
这明确表明我们正准备远程访问一个 percpu 区域。

```

	struct data *p = this_cpu_ptr(&datap);

```
但是，将通过 this_cpu_ptr 计算出的指针传递给其他 CPU 是不寻常的，
应当避免。

远程访问通常只用于读取另一个 CPU 的每 CPU 数据的状态。由于 this_cpu
操作的同步要求宽松，写访问可能导致独特的问题。

一个说明写操作某些隐患的例子是下面这个场景：由于两个每 CPU 变量共享
同一个缓存行，但宽松的同步只应用于更新该缓存行的一个进程。

```

	struct test {
		atomic_t a;
		int b;
	};

	DEFINE_PER_CPU(struct test, onecacheline);

```
这里存在一个顾虑：如果字段 'a' 被一个处理器远程更新，而本地处理器
使用 this_cpu 操作来更新字段 b，应当避免这种对同一缓存行内数据的
并发访问。同时可能还需要代价高昂的同步。在此类场景中，通常建议使用
IPI 而非远程写入另一个处理器的每 CPU 区域。

即使在远程写很少发生的情况下，也请记住，远程写会将缓存行从最可能
访问它的处理器上逐出。如果该处理器被唤醒后发现其每 CPU 区域缺失
本地缓存行，其性能以及唤醒时间都会受到影响。

