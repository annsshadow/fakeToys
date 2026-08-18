# -*- coding: utf-8 -*-
import re, os
src = "bpf/classic_vs_extended.md"
tmp = src + ".tmp"
t = open(src, encoding="utf-8").read()

reps = []
# Block 1
old1 = ("  The old format had two registers A and X, and a hidden frame pointer. The\n"
"  new layout extends this to be 10 internal registers and a read-only frame\n"
"  pointer. Since 64-bit CPUs are passing arguments to functions via registers\n"
"  the number of args from eBPF program to in-kernel function is restricted\n"
"  to 5 and one register is used to accept return value from an in-kernel\n"
"  function. Natively, x86_64 passes first 6 arguments in registers, aarch64/\n"
"  sparcv9/mips64 have 7 - 8 registers for arguments; x86_64 has 6 callee saved\n"
"  registers, and aarch64/sparcv9/mips64 have 11 or more callee saved registers.\n"
"\n"
"  Thus, all eBPF registers map one to one to HW registers on x86_64, aarch64,\n"
"  etc, and eBPF calling convention maps directly to ABIs used by the kernel on\n"
"  64-bit architectures.\n"
"\n"
"  On 32-bit architectures JIT may map programs that use only 32-bit arithmetic\n"
"  and may let more complex programs to be interpreted.")
new1 = ("  旧格式有两个寄存器 A 和 X，以及一个隐藏的帧指针。新布局将其扩展为 10 个内部寄存器和一个只读帧指针。"
"由于 64 位 CPU 通过寄存器向函数传递参数，因此 eBPF 程序向内核函数传递的参数个数被限制为 5 个，"
"并另有一个寄存器用于接收内核函数的返回值。在原生调用约定中，x86_64 通过寄存器传递前 6 个参数，"
"aarch64/sparcv9/mips64 有 7~8 个寄存器用于传参；x86_64 有 6 个被调用者保存（callee saved）寄存器，"
"而 aarch64/sparcv9/mips64 有 11 个或更多被调用者保存寄存器。\n"
"\n"
"  因此，在 x86_64、aarch64 等架构上，所有 eBPF 寄存器都与硬件寄存器一一对应，eBPF 调用约定也直接映射到 64 位架构上内核所使用的 ABI。\n"
"\n"
"  在 32 位架构上，JIT 可以对仅使用 32 位算术运算的程序进行映射，而更复杂的程序则可能被交由解释器执行。")
reps.append((old1, new1))

# Block 3
old3 = ("  While the original design has constructs such as ``if (cond) jump_true;\n"
"  else jump_false;``, they are being replaced into alternative constructs like\n"
"  `if (cond) jump_true; /** else fall-through **/`.")
new3 = ("  尽管原始设计中有诸如 ``if (cond) jump_true; else jump_false;`` 这样的结构，"
"但它们正被替换为类似 `if (cond) jump_true; /** else fall-through **/` 的替代结构。")
reps.append((old3, new3))

# Block 4a (fused heading)
old4a = "- 引入了 bpf_call 指令以及零开销的寄存器传参约定\n  calls from/to other kernel functions:"
new4a = "- 引入了 bpf_call 指令以及零开销的寄存器传参约定，用于与其他内核函数之间的调用："
reps.append((old4a, new4a))

# Block 4b
old4b = ("  Before an in-kernel function call, the eBPF program needs to\n"
"  place function arguments into R1 to R5 registers to satisfy calling\n"
"  convention, then the interpreter will take them from registers and pass\n"
"  to in-kernel function. If R1 - R5 registers are mapped to CPU registers\n"
"  that are used for argument passing on given architecture, the JIT compiler\n"
"  doesn't need to emit extra moves. Function arguments will be in the correct\n"
"  registers and BPF_CALL instruction will be JITed as single 'call' HW\n"
"  instruction. This calling convention was picked to cover common call\n"
"  situations without performance penalty.\n"
"\n"
"  After an in-kernel function call, R1 - R5 are reset to unreadable and R0 has\n"
"  a return value of the function. Since R6 - R9 are callee saved, their state\n"
"  is preserved across the call.")
new4b = ("  在进行内核函数调用之前，eBPF 程序需要按照调用约定将函数参数放入 R1 至 R5 寄存器，"
"随后解释器会从这些寄存器中取出参数并传递给内核函数。如果 R1~R5 寄存器被映射到给定架构上用于传参的 CPU 寄存器，"
"则 JIT 编译器无需额外发出数据移动指令。函数参数将位于正确的寄存器中，BPF_CALL 指令也会被 JIT 编译为单条 'call' 硬件指令。"
"选择这种调用约定是为了在不损失性能的前提下覆盖常见的调用场景。\n"
"\n"
"  在内核函数调用之后，R1~R5 会被重置为不可读状态，而 R0 中保存着函数的返回值。由于 R6~R9 是被调用者保存（callee saved）寄存器，其状态会在调用过程中得以保留。")
reps.append((old4b, new4b))

for o, n in reps:
    c = t.count(o)
    if c != 1:
        print("WARN exact count=", c)
    t = t.replace(o, n)

# Block 2 via regex (tolerant)
new2 = ("  尽管如此，原始 32 位 ALU 操作的语义通过 32 位子寄存器得以保留。所有 eBPF 寄存器均为 64 位，"
"其低 32 位子寄存器在被写入时会零扩展为 64 位。该行为与 x86_64 和 arm64 的子寄存器定义直接对应，但也使得其他架构的 JIT 实现更加困难。\n"
"\n"
"  32 位架构通过解释器来运行 64 位 eBPF 程序。其 JIT 可以将仅使用 32 位子寄存器的 BPF 程序转换为原生指令集，其余部分则交由解释器执行。\n"
"\n"
"  运算采用 64 位，是因为在 64 位架构上指针同样为 64 位宽，并且我们需要在内核函数之间传入/传出 64 位值；"
"否则 32 位 eBPF 寄存器就需要定义寄存器对（register-pair）ABI，从而无法使用 eBPF 寄存器到硬件寄存器的直接映射，"
"JIT 还需要在每次函数调用进出时为每个寄存器进行合并/拆分/移动操作，这既复杂又容易引入缺陷且速度缓慢。另一个原因是要使用原子 64 位计数器。")
pat2 = re.compile(r"  Still, the semantics of the original 32-bit ALU operations are preserved.*?atomic 64-bit counters\.", re.DOTALL)
m = pat2.search(t)
if not m:
    print("WARN block2 regex no match")
else:
    t = pat2.sub(new2, t, count=1)
    print("block2 replaced len", len(m.group(0)))

open(tmp, "w", encoding="utf-8").write(t)
print("wrote", tmp, "size", len(t))
