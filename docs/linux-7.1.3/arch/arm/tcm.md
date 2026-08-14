## Linux 中的 ARM TCM（紧耦合内存）处理


Written by Linus Walleij <linus.walleij@stericsson.com>

一些 ARM SoC 具有所谓的 TCM（Tightly-Coupled Memory，紧耦合内存）。
这通常是 ARM 处理器内部仅几（4-64）KiB 的 RAM。

由于内嵌于 CPU 内部，TCM 具有哈佛（Harvard）架构，因此有一个 ITCM（指令 TCM）
和一个 DTCM（数据 TCM）。DTCM 不能包含任何指令，但 ITCM 实际上可以包含数据。
DTCM 或 ITCM 的最小尺寸为 4KiB，因此典型的配置是 4KiB ITCM 和 4KiB DTCM。

ARM CPU 有专门的寄存器来读出 TCM 内存的状态、物理位置和大小。arch/arm/include/asm/cputype.h
定义了一个 CPUID_TCM 寄存器，你可以从系统控制协处理器中读出。ARM 的文档可以在 http://infocenter.arm.com,
找到，搜索 "TCM Status Register" 可查看所有 CPU 的文档。读取该寄存器你可以确定机器中
是否存在 ITCM（位 1-0）和/或 DTCM（位 17-16）。

还有一个 TCM 区域寄存器（在 ARM 站点搜索 "TCM Region Registers"），可以在运行时报告并
修改 TCM 内存的位置和大小。这用于读出和修改 TCM 的位置与大小。注意这不是 MMU 页表：你
实际上是把 TCM 的物理位置移动了。在你放置它的地方，它会屏蔽掉 CPU 底层任何 RAM，因此通常
最好不要让任何物理 RAM 与 TCM 重叠。

然后可以使用 MMU 把 TCM 内存再次重映射到另一个地址，但请注意 TCM 经常用于 MMU 被关闭的
情况。为避免混淆，当前 Linux 实现会把 TCM 从物理内存到虚拟内存按内核指定的位置做 1 对 1
映射。目前 Linux 会把 ITCM 映射到 0xfffe0000 及之后，把 DTCM 映射到 0xfffe8000 及之后，
最多支持 32KiB 的 ITCM 和 32KiB 的 DTCM。

更新版本的区域寄存器还支持把这些 TCM 分成两个独立的 bank，例如一个 8KiB 的 ITCM 被分成
两个 4KiB 的 bank，各有自己的控制寄存器。其思路是能够锁定并隐藏其中一个 bank 供安全世界
（TrustZone）使用。

TCM 用于以下几方面：

- FIQ 以及其它需要确定性时序且不能等待缓存未命中的中断处理程序。

- 所有外部 RAM 都进入自刷新保持模式的空闲循环，因此 CPU 只能访问片上 RAM，然后我们
  挂起在 ITCM 内等待中断。

- 其它意味着关闭或重新配置外部 RAM 控制器的操作。

在 <asm/tcm.h> 中有一个用于 ARM 架构上使用 TCM 的接口。使用该接口可以：

- 定义 ITCM 和 DTCM 的物理地址和大小。

- 标记要被编译进 ITCM 的函数。

- 标记要分配到 DTCM 和 ITCM 的数据和常量。

- 把剩余的 TCM RAM 通过 gen_pool_create() 和 gen_pool_add() 添加到一个特殊的分配池，
  并为此内存提供 tcm_alloc() 和 tcm_free()。这样的堆非常适合在关闭设备电源域时保存
  设备状态之类的事情。

拥有 TCM 内存的机器应当为自己从 arch/arm/Kconfig 中选择 HAVE_TCM。需要使用 TCM 的代码应当
#include <asm/tcm.h>

要进入 itcm 的函数可以这样标记：
int __tcmfunc foo(int bar);

由于这些被标记为 long_calls，而你可能希望 TCM 内部以本地方式调用函数而不浪费空间，因此
还有 __tcmlocalfunc 前缀，它会让调用变为相对调用。

```

  int __tcmdata foo;

```
```

  int __tcmconst foo;

```
```

  .section ".tcm.text" or .section ".tcm.data"

```
respectively.

```

  #include <asm/tcm.h>

  /* Uninitialized data */
  static u32 __tcmdata tcmvar;
  /* Initialized data */
  static u32 __tcmdata tcmassigned = 0x2BADBABEU;
  /* Constant */
  static const u32 __tcmconst tcmconst = 0xCAFEBABEU;

  static void __tcmlocalfunc tcm_to_tcm(void)
  {
	int i;
	for (i = 0; i < 100; i++)
		tcmvar ++;
  }

  static void __tcmfunc hello_tcm(void)
  {
	/* Some abstract code that runs in ITCM */
	int i;
	for (i = 0; i < 100; i++) {
		tcmvar ++;
	}
	tcm_to_tcm();
  }

  static void __init test_tcm(void)
  {
	u32 *tcmem;
	int i;

	hello_tcm();
	printk("Hello TCM executed from ITCM RAM\n");

	printk("TCM variable from testrun: %u @ %p\n", tcmvar, &tcmvar);
	tcmvar = 0xDEADBEEFU;
	printk("TCM variable: 0x%x @ %p\n", tcmvar, &tcmvar);

	printk("TCM assigned variable: 0x%x @ %p\n", tcmassigned, &tcmassigned);

	printk("TCM constant: 0x%x @ %p\n", tcmconst, &tcmconst);

	/* Allocate some TCM memory from the pool */
	tcmem = tcm_alloc(20);
	if (tcmem) {
		printk("TCM Allocated 20 bytes of TCM @ %p\n", tcmem);
		tcmem[0] = 0xDEADBEEFU;
		tcmem[1] = 0x2BADBABEU;
		tcmem[2] = 0xCAFEBABEU;
		tcmem[3] = 0xDEADBEEFU;
		tcmem[4] = 0x2BADBABEU;
		for (i = 0; i < 5; i++)
			printk("TCM tcmem[%d] = %08x\n", i, tcmem[i]);
		tcm_free(tcmem, 20);
	}
  }

```