## 静态键



   已弃用的 API

   直接使用 'struct static_key' 现已弃用。此
```

	struct static_key false = STATIC_KEY_INIT_FALSE;
	struct static_key true = STATIC_KEY_INIT_TRUE;
	static_key_true()
	static_key_false()

   The updated API replacements are::

	DEFINE_STATIC_KEY_TRUE(key);
	DEFINE_STATIC_KEY_FALSE(key);
	DEFINE_STATIC_KEY_ARRAY_TRUE(keys, count);
	DEFINE_STATIC_KEY_ARRAY_FALSE(keys, count);
	static_branch_likely()
	static_branch_unlikely()

```
## 摘要


静态键允许通过 GCC 特性和一段代码，将很少使用的特性包含在性能敏感的快速路径内核代码中
```

	DEFINE_STATIC_KEY_FALSE(key);

	...

        if (static_branch_unlikely(&key))
                do unlikely code
        else
                do likely code

	...
	static_branch_enable(&key);
	...
	static_branch_disable(&key);
	...

```
static_branch_unlikely() 分支会被生成为代码，对可能执行的代码路径影响尽可能小


## 动机



目前，跟踪点是通过条件分支实现的。该条件检查需要对每个跟踪点检查一个全局变量。尽管此检查的开销很小，但当内存缓存承受压力时会增大（这些全局变量的内存缓存行可能与其他内存访问共享）。随着内核中跟踪点数量的增加，这一开销可能会成为更大的问题。此外，跟踪点通常是休眠的（被禁用）且不提供直接的内核功能。因此，尽可能降低它们的影响是非常可取的。尽管跟踪点是此项工作最初的动机，其他内核代码路径也应当能够利用静态键机制


## 解决方案



gcc（v4.5）新增了一'asm goto' 语句，允许跳转到标签

https://gcc.gnu.org/ml/gcc-patches/2009-07/msg01556.html

借助 'asm goto'，我们可以创建默认要么执行、要么不执行的跳转分支，而无需检查内存。随后，在运行时我们可以修补分支点来改变分支方向

```

	if (static_branch_unlikely(&key))
		printk("I am the true branch\n");

```
因此，默认情况下不会发出 'printk'。生成的代码将由直线代码路径中的单条原子 'no-op' 指令（x86 上为 5 字节）组成。当分支被“翻转”时，我们会用跳转到离线条件真分支的 'jump' 指令来修补直线代码路径中'no-op'。因此，改变分支方向代价高昂，但分支选择基本上是“免费”的。这就是该优化的基本权衡

这一底层修补机制被称'jump label patching'（跳转标签修补），它构成了静态键机制的基础

## 静态键标签 API、用法与示例



```

	DEFINE_STATIC_KEY_TRUE(key);

```
```

	DEFINE_STATIC_KEY_FALSE(key);


```
该键必须是全局的，也就是说，它不能在栈上分配，也不能在运行时动态分配

```

        if (static_branch_unlikely(&key))
                do unlikely code
        else
                do likely code

```
```

        if (static_branch_likely(&key))
                do likely code
        else
                do unlikely code

```
通过 DEFINE_STATIC_KEY_TRUE() DEFINE_STATIC_KEY_FALSE 定义的键，可用于 static_branch_likely() static_branch_unlikely() 语句中

```

	static_branch_enable(&key);

```
```

	static_branch_disable(&key);

```
```

	static_branch_inc(&key);
	...
	static_branch_dec(&key);

```
'static_branch_inc()' 表示“使分支为真”，'static_branch_dec()' 表示“使分支为假”，并带有相应的引用计数。例如，如果键初始化为真，则 static_branch_dec() 会将分支切换为假；随后的 static_branch_inc() 会将分支重新改为真。类似地，如果键初始化为假，'static_branch_inc()' 会将分支改为真；然后 'static_branch_dec()' 会再次使分支为假

可以使用 'static_key_enabled()' 'static_key_count()' 获取状态和引用计数。一般来说，如果使用这些函数，应当用enable/disable increment/decrement 函数周围相同的互斥锁加以保护

注意，切换分支会导致获取一些锁，特别是 CPU 热插拔锁（以避免在修补内核时 CPU 被接入内核而产生竞争）。因此，在热插拔通知器中调用静态键 API 注定会导致死锁。为了仍然允许使用该功能，提供了以下函数

	static_key_enable_cpuslocked()
	static_key_disable_cpuslocked()
	static_branch_enable_cpuslocked()
	static_branch_disable_cpuslocked()

这些函数**并非**通用目的，必须且仅当在确实处于上述上下文、且没有其它上下文时使用

```

	DEFINE_STATIC_KEY_ARRAY_TRUE(keys, count);

```
```

	DEFINE_STATIC_KEY_ARRAY_FALSE(keys, count);

```
4) 架构级代码修补接口，'jump labels'（跳转标签）


为了利用这一优化，架构必须实现若干函数和宏。如果没有架构支持，我们会简单地回退到传统的“加载、测试、跳转”序列。此外，struct jump_entry 表必须至4 字节对齐，因static_key->entry 字段使用了最低两位

- `select HAVE_ARCH_JUMP_LABEL`，参见：arch/x86/Kconfig

- `#define JUMP_LABEL_NOP_SIZE`，参见：arch/x86/include/asm/jump_label.h

- `__always_inline bool arch_static_branch(struct static_key *key, bool branch)`，参见：arch/x86/include/asm/jump_label.h

- `__always_inline bool arch_static_branch_jump(struct static_key *key, bool branch)`，参见：arch/x86/include/asm/jump_label.h

- `void arch_jump_label_transform(struct jump_entry *entry, enum jump_label_type type)`，参见：arch/x86/kernel/jump_label.c

- `struct jump_entry`，参见：arch/x86/include/asm/jump_label.h


5) 静态键 / 跳转标签分析，结果（x86_64）：


作为示例，我们在 'getppid()' 中添加如下分支，使得
```

  SYSCALL_DEFINE0(getppid)
  {
        int pid;

  +     if (static_branch_unlikely(&key))
  +             printk("I am the true branch\n");

        rcu_read_lock();
        pid = task_tgid_vnr(rcu_dereference(current->real_parent));
        rcu_read_unlock();

        return pid;
  }

```
```

  ffffffff81044290 <sys_getppid>:
  ffffffff81044290:       55                      push   %rbp
  ffffffff81044291:       48 89 e5                mov    %rsp,%rbp
  ffffffff81044294:       e9 00 00 00 00          jmpq   ffffffff81044299 <sys_getppid+0x9>
  ffffffff81044299:       65 48 8b 04 25 c0 b6    mov    %gs:0xb6c0,%rax
  ffffffff810442a0:       00 00
  ffffffff810442a2:       48 8b 80 80 02 00 00    mov    0x280(%rax),%rax
  ffffffff810442a9:       48 8b 80 b0 02 00 00    mov    0x2b0(%rax),%rax
  ffffffff810442b0:       48 8b b8 e8 02 00 00    mov    0x2e8(%rax),%rdi
  ffffffff810442b7:       e8 f4 d9 00 00          callq  ffffffff81051cb0 <pid_vnr>
  ffffffff810442bc:       5d                      pop    %rbp
  ffffffff810442bd:       48 98                   cltq
  ffffffff810442bf:       c3                      retq
  ffffffff810442c0:       48 c7 c7 e3 54 98 81    mov    $0xffffffff819854e3,%rdi
  ffffffff810442c7:       31 c0                   xor    %eax,%eax
  ffffffff810442c9:       e8 71 13 6d 00          callq  ffffffff8171563f <printk>
  ffffffff810442ce:       eb c9                   jmp    ffffffff81044299 <sys_getppid+0x9>

```
```

  ffffffff810441f0 <sys_getppid>:
  ffffffff810441f0:       8b 05 8a 52 d8 00       mov    0xd8528a(%rip),%eax        # ffffffff81dc9480 <key>
  ffffffff810441f6:       55                      push   %rbp
  ffffffff810441f7:       48 89 e5                mov    %rsp,%rbp
  ffffffff810441fa:       85 c0                   test   %eax,%eax
  ffffffff810441fc:       75 27                   jne    ffffffff81044225 <sys_getppid+0x35>
  ffffffff810441fe:       65 48 8b 04 25 c0 b6    mov    %gs:0xb6c0,%rax
  ffffffff81044205:       00 00
  ffffffff81044207:       48 8b 80 80 02 00 00    mov    0x280(%rax),%rax
  ffffffff8104420e:       48 8b 80 b0 02 00 00    mov    0x2b0(%rax),%rax
  ffffffff81044215:       48 8b b8 e8 02 00 00    mov    0x2e8(%rax),%rdi
  ffffffff8104421c:       e8 2f da 00 00          callq  ffffffff81051c50 <pid_vnr>
  ffffffff81044221:       5d                      pop    %rbp
  ffffffff81044222:       48 98                   cltq
  ffffffff81044224:       c3                      retq
  ffffffff81044225:       48 c7 c7 13 53 98 81    mov    $0xffffffff81985313,%rdi
  ffffffff8104422c:       31 c0                   xor    %eax,%eax
  ffffffff8104422e:       e8 60 0f 6d 00          callq  ffffffff81715193 <printk>
  ffffffff81044233:       eb c9                   jmp    ffffffff810441fe <sys_getppid+0xe>
  ffffffff81044235:       66 66 2e 0f 1f 84 00    data32 nopw %cs:0x0(%rax,%rax,1)
  ffffffff8104423c:       00 00 00 00

```
因此，禁用跳转标签的情况会增加一'mov'test' 'jne' 指令，而跳转标签情况只有一'no-op' 'jmp 0'。（jmp 0 在启动时被修补为 5 字节的原no-op 指令。）因此，被禁用的跳
```

  6 (mov) + 2 (test) + 2 (jne) = 10 - 5 (5 byte jump 0) = 5 addition bytes.

```
如果我们再计入填充字节，跳转标签代码为这个小函数节省了总计 16 字节的指令内存。在本例中，非跳转标签函数长 80 字节。因此，我们节省20% 的指令占用。事实上我们还能进一步改进，因为 5 字节 no-op 实际上可以是 2 字节 no-op，因为我们可以用 2 字节 jmp 到达分支。不过，我们尚未实现最优的 no-op 大小（目前是硬编码的）

由于调度器路径中有多处使用静态键 API，可以使'pipe-test'（也称为 'perf bench sched pipe'）来展示性能提升。在 3.3.0-rc2 上完成的测试

```

 Performance counter stats for 'bash -c /tmp/pipe-test' (50 runs):

        855.700314 task-clock                #    0.534 CPUs utilized            ( +-  0.11% )
           200,003 context-switches          #    0.234 M/sec                    ( +-  0.00% )
                 0 CPU-migrations            #    0.000 M/sec                    ( +- 39.58% )
               487 page-faults               #    0.001 M/sec                    ( +-  0.02% )
     1,474,374,262 cycles                    #    1.723 GHz                      ( +-  0.17% )
   <not supported> stalled-cycles-frontend
   <not supported> stalled-cycles-backend
     1,178,049,567 instructions              #    0.80  insns per cycle          ( +-  0.06% )
       208,368,926 branches                  #  243.507 M/sec                    ( +-  0.06% )
         5,569,188 branch-misses             #    2.67% of all branches          ( +-  0.54% )

       1.601607384 seconds time elapsed                                          ( +-  0.07% )

```
```

 Performance counter stats for 'bash -c /tmp/pipe-test' (50 runs):

        841.043185 task-clock                #    0.533 CPUs utilized            ( +-  0.12% )
           200,004 context-switches          #    0.238 M/sec                    ( +-  0.00% )
                 0 CPU-migrations            #    0.000 M/sec                    ( +- 40.87% )
               487 page-faults               #    0.001 M/sec                    ( +-  0.05% )
     1,432,559,428 cycles                    #    1.703 GHz                      ( +-  0.18% )
   <not supported> stalled-cycles-frontend
   <not supported> stalled-cycles-backend
     1,175,363,994 instructions              #    0.82  insns per cycle          ( +-  0.04% )
       206,859,359 branches                  #  245.956 M/sec                    ( +-  0.04% )
         4,884,119 branch-misses             #    2.36% of all branches          ( +-  0.85% )

       1.579384366 seconds time elapsed

```
节省的分支百分比0.7%，并且在 'branch-misses'（分支预测失败）上节省了 12%。这正是我们期望获得最多节省的地方，因为该优化旨在减少分支数量。此外，我们在指令上节省0.2%，在周期上节省了 2.8%，在耗时上节省了 1.4%
