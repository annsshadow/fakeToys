
:Author: Deepak Gupta <debug@rivosinc.com>
:Date:   12 January 2024

## Shadow 栈 到 protect 函数 returns 在 RISC-V Linux


此 document briefly describes the 接口 provided 到 userspace 由 Linux
到 启用 shadow stacks 用于 用户 模式 applications 在 RISC-V.

### 1. 特性 Overview


内存 corruption issues 通常 result 在 crashes.  然而, 在 the
hands 的 一个 creative adversary, 这些 issues 可 result 在 一个 variety 的
安全 problems.

一些 的 那些 安全 issues 可 为 code re-use attacks 在 programs
何处 一个 adversary 可 使用 corrupt return 地址 present 在 the
栈. chaining them together 到 perform return oriented programming
(ROP) 和 从而 compromising the control flow integrity (CFI) 的 the
program.

Return 地址 实时 在 the 栈 在 read-write 内存.  因此
它们是 susceptible 到 corruption, 其 allows 一个 adversary 到
control the program counter. 在 RISC-V, the `zicfiss` extension
提供 一个 alternate 栈 (the "shadow 栈") 在 其 return
地址 可 为 safely placed 在 the prologue 的 the 函数 和
retrieved 在 the epilogue.  The `zicfiss` extension makes the
以下 changes:

- PTE encodings 用于 shadow 栈 虚拟 内存
  一个 更早 reserved encoding 在 第一 stage translation i.e.
  PTE.R=0, PTE.W=1, PTE.X=0  becomes the PTE encoding 用于 shadow 栈 页.

- The `sspush x1/x5` instruction pushes (stores) `x1/x5` 到 shadow 栈.

- The `sspopchk x1/x5` instruction pops (loads) 来自 shadow 栈 和 compares
  与 `x1/x5` 和 若 不 equal, the CPU raises 一个 `software check exception`
  与 `*tval = 3`

The compiler toolchain ensures 该 函数 prologues 具有 ``sspush
x1/x5`` 到 save the return 地址 在 shadow 栈 此外 到 the
regular 栈.  Similarly, 函数 epilogues 具有 ``ld x5,
偏移(x2)` followed by `sspopchk x5`` 到 ensure 该 一个 popped 值
来自 the regular 栈 matches 与 the popped 值 来自 the shadow
栈.

### 2. Shadow 栈 protections 和 linux 内存 manager


作为 mentioned 更早, shadow stacks get 新 页 表 encodings 该
具有 一些 特殊 properties assigned 到 them, along 与 instructions
该 operate 在 the shadow stacks:

- Regular stores 到 shadow 栈 内存 raise store access faults. 此
  protects shadow 栈 内存 来自 stray writes.

- Regular loads 来自 shadow 栈 内存 是 allowed. 此 allows
  栈 trace utilities 或 backtrace 函数 到 读取 the true call
  栈 和 ensure 该 它 具有 不 已经 tampered 与.

- 仅 shadow 栈 instructions 可 generate shadow 栈 loads 或
  shadow 栈 stores.

- Shadow 栈 loads 和 stores 在 read-only 内存 raise AMO/store
  页 faults. 从而 两者 `sspush x1/x5` 和 `sspopchk x1/x5` 将
  raise AMO/store 页 fault. 此 simplies COW handling 在 内核
  期间 fork(). The 内核 可 convert shadow 栈 页 进入
  read-only 内存 (作为 它 执行 用于 regular read-write 内存).  作为
  soon 作为 后续 `sspush` 或 `sspopchk` instructions 在
  userspace 是 encountered, the 内核 可 perform COW.

- Shadow 栈 loads 和 stores 在 read-write 或 read-write-execute
  内存 raise 一个 access fault. 这是 一个 fatal condition 因为
  shadow 栈 loads 和 stores 应当 从不 为 operating 在
  read-write 或 read-write-execute 内存.

### 3. ELF 和 psABI


The toolchain sets up `GNU_PROPERTY_RISCV_FEATURE_1_BCFI` 用于
property `GNU_PROPERTY_RISCV_FEATURE_1_AND` 在 the notes
section 的 the 对象 文件.

### 4. Linux enabling


用户空间 programs 可 具有 多个 shared objects loaded 在 它们的
地址 space.  它's 一个 difficult task 到 确保 全部 the
dependencies 具有 已经 compiled 与 shadow 栈 支持.  从而
它's left 到 the 动态 loader 到 启用 shadow stacks 用于 the
program.

### 5. prctl() enabling


`PR_SET_SHADOW_STACK_STATUS` / `PR_GET_SHADOW_STACK_STATUS` /
`PR_LOCK_SHADOW_STACK_STATUS` 是 three prctls added 到 manage shadow
栈 enabling 用于 tasks.  这些 prctls 是 architecture-agnostic 和 return
-EINVAL 若 不 implemented.

- prctl(PR_SET_SHADOW_栈_状态, unsigned long arg)

若 arg = `PR_SHADOW_STACK_ENABLE` 和 若 CPU supports
`zicfiss` 然后 the 内核 将 启用 shadow stacks 用于 the task.
The 动态 loader 可 issue 此 `prctl` 一旦 它 具有
determined 该 全部 the objects loaded 在 地址 space 具有 支持
用于 shadow stacks.  Additionally, 若 存在 一个 `dlopen` 到
一个 对象 其 wasn't compiled 与 `zicfiss`, the 动态 loader
可 issue 此 prctl 与 arg set 到 0 (i.e.
`PR_SHADOW_STACK_ENABLE` 正在 clear)

- prctl(PR_GET_SHADOW_栈_状态, unsigned long * arg)

Returns the 电流 状态 的 indirect branch tracking. 若 已启用
它'll return `PR_SHADOW_STACK_ENABLE`.

- prctl(PR_锁_SHADOW_栈_状态, unsigned long arg)

锁 the 电流 状态 的 shadow 栈 enabling 在 the
task. Userspace 可 希望 到 运行 与 一个 strict 安全 posture 和
wouldn't 希望 loading 的 objects 无 `zicfiss` 支持.  在 此
case userspace 可 使用 此 prctl 到 disallow disabling 的 shadow
stacks 在 the 电流 task.

### 5. violations related 到 returns 与 shadow 栈 已启用


Pertaining 到 shadow stacks, the CPU raises 一个 ``软件 check
异常` upon executing `sspopchk x1/x5` if `x1/x5`` doesn't
match the top 的 shadow 栈.  若 一个 mismatch happens, 然后 the CPU
sets `*tval = 3` 和 raises the 异常.

The Linux 内核 将 treat 此 作为 一个 `SIGSEGV` 与 code =
`SEGV_CPERR` 和 follow the 正常 course 的 信号 delivery.

### 6. Shadow 栈 tokens


Regular stores 在 shadow stacks 是 不 allowed 和 从而 可't 为
tampered 与 通过 arbitrary stray writes.  然而, one 方法 的
pivoting / switching 到 一个 shadow 栈 是 simply writing 到 the CSR
`CSR_SSP`.  此 将 change the active shadow 栈 用于 the
program.  Writes 到 `CSR_SSP` 在 the program 应当 为 mostly
limited 到 上下文 switches, 栈 unwinds, 或 longjmp 或 similar
mechanisms (类似 上下文 switching 的 Green 线程) 在 languages 类似
Go 和 Rust. CSR_SSP writes 可 为 problematic 因为 一个 attacker 可
使用 内存 corruption bugs 和 leverage 上下文 switching routines 到
pivot 到 任何 shadow 栈. Shadow 栈 tokens 可 help mitigate 此
problem 由 making sure 该:

- 当 软件 是 switching away 来自 一个 shadow 栈, the shadow
  栈 指针 应当 为 saved 在 the shadow 栈 itself (这是
  called the `shadow stack token`).

- 当 软件 是 switching 到 一个 shadow 栈, 它 应当 读取 the
  `shadow stack token` 来自 the shadow 栈 指针 和 verify 该
  the `shadow stack token` itself 是 一个 指针 到 the shadow 栈
  itself.

- 一旦 the token verification 是 已完成, 软件 可 perform the 写入
  到 `CSR_SSP` 到 switch shadow stacks.

此处 "软件" 可以 参考 到 the 用户 模式 task runtime itself,
managing 各种 contexts 作为 part 的 一个 单个 线程.  或 "软件"
可以 参考 到 the 内核, 当 the 内核 具有 到 deliver 一个 信号 到
一个 用户 task 和 必须 save the shadow 栈 指针.  The 内核 可
perform similar procedure itself 由 saving 一个 token 在 the 用户 模式
task's shadow 栈.  此 way, whenever `sigreturn` happens,
the 内核 可 读取 和 verify the token 和 然后 switch 到 the shadow
栈. 使用 此 mechanism, the 内核 helps the 用户 task 因此 该
任何 corruption issue 在 the 用户 task 是 不 exploited 由 adversaries
arbitrarily 使用 `sigreturn`. Adversaries 将 具有 到 make
sure 该 存在 一个 valid `shadow stack token` 此外 到
invoking `sigreturn`.

### 7. 信号 shadow 栈

```

    struct __sc_riscv_cfi_state {
        unsigned long ss_ptr;
    };

```
作为 part 的 信号 delivery, the shadow 栈 token 是 saved 在 the
电流 shadow 栈 itself.  The updated 指针 是 saved away 在 the
`ss_ptr` 字段 在 `__sc_riscv_cfi_state` 在…下
`sigcontext`. The existing shadow 栈 分配 是 使用
用于 信号 delivery.  期间 `sigreturn`, 内核 将 obtain
`ss_ptr` 来自 `sigcontext`, verify the saved
token 在 the shadow 栈, 和 switch the shadow 栈.
