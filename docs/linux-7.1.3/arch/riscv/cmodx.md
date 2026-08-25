
## RISC-V Linux 的指令并发修改与执行（CMODX

CMODX 是一种编程技术，程序执行由程序自身修改过的指令。在 RISC-V 硬件上，指令存储
和指令缓存（icache）并不保证同步。因此，程序必须借助非特权指fence.i 来强制进自身的同步
### 内核空间中的 CMODX


### 动ftrace


本质上，动ftrace 通过在每个可打补丁的函数入口插入一个函数调用来引导控制流，
并在运行时动态地给其打补丁以启用或禁用重定向。在 RISC-V 的情况下，需2 条指（AUIPC + JALR）来组成一个函数调用。然而，不可能在2 条指令补丁的同时期望并发读端无竞争条件地执行它们。这个系列使得在 RISC-V ftrace 中进行原子代码补丁成为可能内核抢占使情况更糟，因为它允许旧状态在打补丁的过程中（通过 stop_machine()）持续存在
为了摆脱 stop_machine() 并在完全内核抢占下运行动ftrace，我们在启动时部分初始化
每个可打补丁的函数入口，将第一条指令设AUIPC，第二条设为 NOP。现在原子打补丁成为
可能，因为内核只需更新一条指令。根Ziccif，只要指令是自然对齐的，ISA 就保证原更新
通过固定第一条指AUIPC，由RISC-V 中立即数编码空间不足，ftrace 跳板的寻址范围
被限制在距离预定目标 ftrace_caller +-2K 之内。为了解决这个问题，我们引入CALL_OPS，在每个可打补丁的函数前面添加一8 字节自然对齐的元数据。该元数据在第一跳板处被解析，然后执行可以被引导到另一个自定义跳板
### 用户空间中的 CMODX


尽管 fence.i 是非特权指令，但默认Linux ABI 禁止在用户空间应用程序中使用 fence.i调度器随时可能将任务迁移到一个新hart 上。如果迁移发生在用户空间fence.i 同步icache 和指令存储之后，hart 上的 icache 将不再干净。这是因fence.i 的行为只影响
调用它的那个 hart。因此，任务被迁移到hart 可能尚未同步指令存储icache
有两种方法可以解决这个问题：使用 riscv_flush_icache() 系统调用，或者使`PR_RISCV_SET_ICACHE_FLUSH_CTX` prctl() 并在用户空间发出 fence.i。系统调用执行一次性的
icache 刷新操作。prctl 改变 Linux ABI 以允许用户空间发icache 刷新操作
顺便一提，在内核中有时可能会触发“延迟”的 icache 刷新。在撰写本文时，这仅发生riscv_flush_icache() 系统调用期间，以及内核使copy_to_user_page() 时。这些延迟刷只在hart 正在使用的内存映射发生变化时发生。如prctl() 上下文已经导致了一icache
刷新，则该延icache 刷新将被跳过，因为它是冗余的。因此，prctl() 上下文内部使riscv_flush_icache() 系统调用时不会发生额外的刷新
### prctl() 接口


`PR_RISCV_SET_ICACHE_FLUSH_CTX` 作为第一个参数调prctl()。其余参数将委托给下详述riscv_set_icache_flush_ctx 函数
	:identifiers: riscv_set_icache_flush_ctx

使用示例
以下文件旨在相互编译并链接在一起。modify_instruction() 函数将一个加 0 的加法替换为
一个加 1 的加法，get_value() 中的指令序列从返回零变为返回一
```

	#include <stdio.h>
	#include <sys/prctl.h>

	extern int get_value();
	extern void modify_instruction();

	int main()
	{
		int value = get_value();
		printf("Value before cmodx: %d\n", value);

		// Call prctl before first fence.i is called inside modify_instruction
		prctl(PR_RISCV_SET_ICACHE_FLUSH_CTX, PR_RISCV_CTX_SW_FENCEI_ON, PR_RISCV_SCOPE_PER_PROCESS);
		modify_instruction();
		// Call prctl after final fence.i is called in process
		prctl(PR_RISCV_SET_ICACHE_FLUSH_CTX, PR_RISCV_CTX_SW_FENCEI_OFF, PR_RISCV_SCOPE_PER_PROCESS);

		value = get_value();
		printf("Value after cmodx: %d\n", value);
		return 0;
	}

```
```

	.option norvc

	.text
	.global modify_instruction
	modify_instruction:
	lw a0, new_insn
	lui a5,%hi(old_insn)
	sw  a0,%lo(old_insn)(a5)
	fence.i
	ret

	.section modifiable, "awx"
	.global get_value
	get_value:
	li a0, 0
	old_insn:
	addi a0, a0, 0
	ret

	.data
	new_insn:
	addi a0, a0, 1

```
