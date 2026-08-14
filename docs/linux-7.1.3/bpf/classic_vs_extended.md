
## 经典 BPF 与 eBPF


eBPF 被设计为以一一对应的方式进行 JIT 编译，这也为
GCC/LLVM 编译器通过
一个 eBPF 后端生成几乎与原生编译代码一样快的优化 eBPF 代码开辟了可能。

eBPF 格式相对于经典 BPF 的一些核心变化：

- 寄存器数量从 2 个增加到 10 个：

  旧格式有两个寄存器 A 和 X，以及一个隐藏的帧指针。新布局将其扩展为 10 个内部寄存器和一个只读帧指针。由于 64 位 CPU 通过寄存器向函数传递参数，因此 eBPF 程序向内核函数传递的参数个数被限制为 5 个，并另有一个寄存器用于接收内核函数的返回值。在原生调用约定中，x86_64 通过寄存器传递前 6 个参数，aarch64/sparcv9/mips64 有 7~8 个寄存器用于传参；x86_64 有 6 个被调用者保存（callee saved）寄存器，而 aarch64/sparcv9/mips64 有 11 个或更多被调用者保存寄存器。

  因此，在 x86_64、aarch64 等架构上，所有 eBPF 寄存器都与硬件寄存器一一对应，eBPF 调用约定也直接映射到 64 位架构上内核所使用的 ABI。

  在 32 位架构上，JIT 可以对仅使用 32 位算术运算的程序进行映射，而更复杂的程序则可能被交由解释器执行。

  R0 - R5 are scratch registers and eBPF program needs spill/fill them if
  necessary across calls. Note that there is only one eBPF program (== one
  eBPF main routine) and it cannot call other eBPF functions, it can only
  call predefined in-kernel functions, though.

- 寄存器位宽从 32 位增加到 64 位：

  Still, the semantics of the original 32-bit ALU operations are preserved
  via 32-bit subregisters. All eBPF registers are 64-bit with 32-bit lower
  subregisters that zero-extend into 64-bit if they are being written to.
  That behavior maps directly to x86_64 and arm64 subregister definition, but
  makes other JITs more difficult.

  32-bit architectures run 64-bit eBPF programs via interpreter.
  Their JITs may convert BPF programs that only use 32-bit subregisters into
  native instruction set and let the rest being interpreted.

  Operation is 64-bit, because on 64-bit architectures, pointers are also
  64-bit wide, and we want to pass 64-bit values in/out of kernel functions,
  so 32-bit eBPF registers would otherwise require to define register-pair
  ABI, thus, there won't be able to use a direct eBPF register to HW register
  mapping and JIT would need to do combine/split/move operations for every
  register in and out of the function, which is complex, bug prone and slow.
  Another reason is the use of atomic 64-bit counters.

- 条件跳转的 jt/jf 目标被替换为 jt/顺序执行（fall-through）：

  尽管原始设计中有诸如 ``if (cond) jump_true; else jump_false;`` 这样的结构，但它们正被替换为类似 `if (cond) jump_true; /** else fall-through **/` 的替代结构。

- 引入了 bpf_call 指令以及零开销的寄存器传参约定，用于与其他内核函数之间的调用：

  在进行内核函数调用之前，eBPF 程序需要按照调用约定将函数参数放入 R1 至 R5 寄存器，随后解释器会从这些寄存器中取出参数并传递给内核函数。如果 R1~R5 寄存器被映射到给定架构上用于传参的 CPU 寄存器，则 JIT 编译器无需额外发出数据移动指令。函数参数将位于正确的寄存器中，BPF_CALL 指令也会被 JIT 编译为单条 'call' 硬件指令。选择这种调用约定是为了在不损失性能的前提下覆盖常见的调用场景。

  在内核函数调用之后，R1~R5 会被重置为不可读状态，而 R0 中保存着函数的返回值。由于 R6~R9 是被调用者保存（callee saved）寄存器，其状态会在调用过程中得以保留。

```

    u64 f1() { return (*_f2)(1); }
    u64 f2(u64 a) { return f3(a + 1, a); }
    u64 f3(u64 a, u64 b) { return a - b; }

  GCC can compile f1, f3 into x86_64::

    f1:
	movl $1, %edi
	movq _f2(%rip), %rax
	jmp  *%rax
    f3:
	movq %rdi, %rax
	subq %rsi, %rax
	ret

  Function f2 in eBPF may look like::

    f2:
	bpf_mov R2, R1
	bpf_add R1, 1
	bpf_call f3
	bpf_exit

  If f2 is JITed and the pointer stored to ``_f2``. The calls f1 -> f2 -> f3 and
  returns will be seamless. Without JIT, __bpf_prog_run() interpreter needs to
  be used to call into f2.

  For practical reasons all eBPF programs have only one argument 'ctx' which is
  already placed into R1 (e.g. on __bpf_prog_run() startup) and the programs
  can call kernel functions with up to 5 arguments. Calls with 6 or more arguments
  are currently not supported, but these restrictions can be lifted if necessary
  in the future.

  On 64-bit architectures all register map to HW registers one to one. For
  example, x86_64 JIT compiler can map them as ...

  ::

    R0 - rax
    R1 - rdi
    R2 - rsi
    R3 - rdx
    R4 - rcx
    R5 - r8
    R6 - rbx
    R7 - r13
    R8 - r14
    R9 - r15
    R10 - rbp

  ... since x86_64 ABI mandates rdi, rsi, rdx, rcx, r8, r9 for argument passing
  and rbx, r12 - r15 are callee saved.

  Then the following eBPF pseudo-program::

    bpf_mov R6, R1 /* save ctx */
    bpf_mov R2, 2
    bpf_mov R3, 3
    bpf_mov R4, 4
    bpf_mov R5, 5
    bpf_call foo
    bpf_mov R7, R0 /* save foo() return value */
    bpf_mov R1, R6 /* restore ctx for next call */
    bpf_mov R2, 6
    bpf_mov R3, 7
    bpf_mov R4, 8
    bpf_mov R5, 9
    bpf_call bar
    bpf_add R0, R7
    bpf_exit

  After JIT to x86_64 may look like::

    push %rbp
    mov %rsp,%rbp
    sub $0x228,%rsp
    mov %rbx,-0x228(%rbp)
    mov %r13,-0x220(%rbp)
    mov %rdi,%rbx
    mov $0x2,%esi
    mov $0x3,%edx
    mov $0x4,%ecx
    mov $0x5,%r8d
    callq foo
    mov %rax,%r13
    mov %rbx,%rdi
    mov $0x6,%esi
    mov $0x7,%edx
    mov $0x8,%ecx
    mov $0x9,%r8d
    callq bar
    add %r13,%rax
    mov -0x228(%rbp),%rbx
    mov -0x220(%rbp),%r13
    leaveq
    retq

  Which is in this example equivalent in C to::

    u64 bpf_filter(u64 ctx)
    {
	return foo(ctx, 2, 3, 4, 5) + bar(ctx, 6, 7, 8, 9);
    }

  In-kernel functions foo() and bar() with prototype: u64 (*)(u64 arg1, u64
  arg2, u64 arg3, u64 arg4, u64 arg5); will receive arguments in proper
  registers and place their return value into ``%rax`` which is R0 in eBPF.
  Prologue and epilogue are emitted by JIT and are implicit in the
  interpreter. R0-R5 are scratch registers, so eBPF program needs to preserve
  them across the calls as defined by calling convention.

  For example the following program is invalid::

    bpf_mov R1, 1
    bpf_call foo
    bpf_mov R0, R1
    bpf_exit

  After the call the registers R1-R5 contain junk values and cannot be read.
  An in-kernel verifier.rst is used to validate eBPF programs.

```
同样在新的设计中，eBPF 被限制为 4096 条指令，这意味着任何
程序都会快速终止，并且只会调用固定数量的内核
函数。原始 BPF 与 eBPF 都是双操作数指令，
这有助于在 JIT 期间实现 eBPF 指令与 x86 指令之间的一一对应映射。

调用解释器函数的输入上下文指针是通用的，
其内容由具体的用例定义。对于 seccomp，寄存器 R1 指向
seccomp_data；对于转换后的 BPF 过滤器，R1 指向 skb。

```

  op:16, jt:8, jf:8, k:32    ==>    op:8, dst_reg:4, src_reg:4, off:16, imm:32

```
到目前为止已实现 87 条 eBPF 指令。8 位 'op' 操作码字段
为新指令留有空间。其中一些可能使用 16/24/32 字节的编码。新
指令必须是 8 字节的整数倍，以保持向后兼容。

eBPF 是一个通用的 RISC 指令集。并非每个寄存器和
每条指令都会在从原始 BPF 到 eBPF 的转换过程中被用到。
例如，socket 过滤器不会使用 `exclusive add` 指令，但
tracing 过滤器可能会使用它来维护事件计数器等。例如，寄存器 R9
也不会被 socket 过滤器使用，但更复杂的过滤器可能会
用尽寄存器，从而不得不借助栈上的溢出/回填（spill/fill）。

eBPF 可用作通用汇编器，用于最后的性能
优化，socket 过滤器和 seccomp 将其用作汇编器。tracing
过滤器可能将其用作汇编器，以便从内核生成代码。在内核中使用时
可能不受安全因素的限制，因为生成的 eBPF 代码
可能只是在优化内部代码路径，而不会暴露给用户空间。
eBPF 的安全性可来自 verifier.rst。在上述这类用例中，
它可以被当作安全的指令集使用。

与原始 BPF 一样，eBPF 运行在受控环境中，
具有确定性，内核可以轻易证明其安全性。程序的安全性
可以通过两步确定：第一步进行深度优先搜索，以禁止
循环和其他 CFG 校验；第二步从第一条指令开始，
遍历所有可能路径。它会模拟每条指令的执行并观察
寄存器与栈的状态变化。

## 操作码编码


eBPF 复用了经典 BPF 的大部分操作码编码，以简化从经典 BPF
到 eBPF 的转换。

对于算术和跳转指令，8 位 'code' 字段被划分为三个
```

  +----------------+--------+--------------------+
  |   4 bits       |  1 bit |   3 bits           |
  | operation code | source | instruction class  |
  +----------------+--------+--------------------+
  (MSB)                                      (LSB)

```
三个最低有效位（LSB）存储指令类别，类别之一是：

  ===================     ===============
  Classic BPF classes     eBPF classes
  ===================     ===============
  BPF_LD    0x00          BPF_LD    0x00
  BPF_LDX   0x01          BPF_LDX   0x01
  BPF_ST    0x02          BPF_ST    0x02
  BPF_STX   0x03          BPF_STX   0x03
  BPF_ALU   0x04          BPF_ALU   0x04
  BPF_JMP   0x05          BPF_JMP   0x05
  BPF_RET   0x06          BPF_JMP32 0x06
  BPF_MISC  0x07          BPF_ALU64 0x07
  ===================     ===============

第 4 位对源操作数进行编码……

```

	BPF_K     0x00
	BPF_X     0x08

 * in classic BPF, this means::

	BPF_SRC(code) == BPF_X - use register X as source operand
	BPF_SRC(code) == BPF_K - use 32-bit immediate as source operand

 * in eBPF, this means::

	BPF_SRC(code) == BPF_X - use 'src_reg' register as source operand
	BPF_SRC(code) == BPF_K - use 32-bit immediate as source operand

```
……而四个最高有效位（MSB）存储操作码。

```

  BPF_ADD   0x00
  BPF_SUB   0x10
  BPF_MUL   0x20
  BPF_DIV   0x30
  BPF_OR    0x40
  BPF_AND   0x50
  BPF_LSH   0x60
  BPF_RSH   0x70
  BPF_NEG   0x80
  BPF_MOD   0x90
  BPF_XOR   0xa0
  BPF_MOV   0xb0  /* eBPF only: mov reg to reg */
  BPF_ARSH  0xc0  /* eBPF only: sign extending shift right */
  BPF_END   0xd0  /* eBPF only: endianness conversion */

```
```

  BPF_JA    0x00  /* BPF_JMP only */
  BPF_JEQ   0x10
  BPF_JGT   0x20
  BPF_JGE   0x30
  BPF_JSET  0x40
  BPF_JNE   0x50  /* eBPF only: jump != */
  BPF_JSGT  0x60  /* eBPF only: signed '>' */
  BPF_JSGE  0x70  /* eBPF only: signed '>=' */
  BPF_CALL  0x80  /* eBPF BPF_JMP only: function call */
  BPF_EXIT  0x90  /* eBPF BPF_JMP only: function return */
  BPF_JLT   0xa0  /* eBPF only: unsigned '<' */
  BPF_JLE   0xb0  /* eBPF only: unsigned '<=' */
  BPF_JSLT  0xc0  /* eBPF only: signed '<' */
  BPF_JSLE  0xd0  /* eBPF only: signed '<=' */

```
因此 BPF_ADD | BPF_X | BPF_ALU 在经典 BPF 和 eBPF 中都表示 32 位加法，
即 A += X。
在 eBPF 中它表示 dst_reg = (u32) dst_reg + (u32) src_reg；类似地，
BPF_XOR | BPF_K | BPF_ALU 在经典 BPF 中表示 A ^= imm32，在 eBPF 中相应地
表示 src_reg = (u32) src_reg ^ (u32) imm32。

经典 BPF 使用 BPF_MISC 类来表示 A = X 和 X = A 的传送。
eBPF 则改用 BPF_MOV | BPF_X | BPF_ALU 代码。由于 eBPF 中没有
BPF_MISC 操作，类别 7 被用作 BPF_ALU64，表示
与 BPF_ALU 完全相同的操作，但操作数为 64 位宽
而非 32 位。因此 BPF_ADD | BPF_X | BPF_ALU64 表示 64 位加法，即：
dst_reg = dst_reg + src_reg

经典 BPF 耗费整个 BPF_RET 类来表示单一的 `ret`
操作。经典 BPF_RET | BPF_K 表示将 imm32 复制到返回寄存器
并执行函数退出。eBPF 的建模与 CPU 相匹配，因此 BPF_JMP | BPF_EXIT
在 eBPF 中仅表示函数退出。eBPF 程序需要先将返回值
存入寄存器 R0，再执行 BPF_EXIT。eBPF 中的类别 6 被用作
BPF_JMP32，表示与 BPF_JMP 完全相同的操作，但比较操作数
为 32 位宽。

```

  +--------+--------+-------------------+
  | 3 bits | 2 bits |   3 bits          |
  |  mode  |  size  | instruction class |
  +--------+--------+-------------------+
  (MSB)                             (LSB)

```
大小修饰符是下列之一……


```

  BPF_W   0x00    /* word */
  BPF_H   0x08    /* half word */
  BPF_B   0x10    /* byte */
  BPF_DW  0x18    /* eBPF only, double word */

```
```

 B  - 1 byte
 H  - 2 byte
 W  - 4 byte
 DW - 8 byte (eBPF only)

```
```

  BPF_IMM     0x00  /* used for 32-bit mov in classic BPF and 64-bit in eBPF */
  BPF_ABS     0x20
  BPF_IND     0x40
  BPF_MEM     0x60
  BPF_LEN     0x80  /* classic BPF only, reserved in eBPF */
  BPF_MSH     0xa0  /* classic BPF only, reserved in eBPF */
  BPF_ATOMIC  0xc0  /* eBPF only, atomic operations */

```