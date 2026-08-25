
:Author: Deepak Gupta <debug@rivosinc.com>
:Date:   12 January 2024

## Shadow protect 函数 returns RISC-V Linux


document briefly describes the 接口 provided userspace Linux
启用 shadow stacks 用于 用户 模式 applications RISC-V.

### 1. 特Overview


内存 corruption issues 通常 result crashes.  然 the
hands 一creative adversary, 这些 issues result 一variety 
安全 problems.

一那些 安全 issues code re-use attacks programs
何处 一adversary 使用 corrupt return 地址 present the
鏍? chaining them together 鍒?perform return oriented programming
(ROP) 从compromising the control flow integrity (CFI) the
program.

Return 地址 实时 the read-write 内存.  因此
它们susceptible corruption, allows 一adversary 
control the program counter. 鍦?RISC-V, the `zicfiss` extension
提供 一alternate (the "shadow ) return
地址 safely placed the prologue the 函数 
retrieved 鍦?the epilogue.  The `zicfiss` extension makes the
以下 changes:

- PTE encodings 用于 shadow 虚拟 内存
  一更早 reserved encoding 第一 stage translation i.e.
  PTE.R=0, PTE.W=1, PTE.X=0  becomes the PTE encoding 用于 shadow 

- The `sspush x1/x5` instruction pushes (stores) `x1/x5` 鍒?shadow 鏍。

- The `sspopchk x1/x5` instruction pops (loads) 来自 shadow compares
  `x1/x5` equal, the CPU raises 一`software check exception`
  涓?`*tval = 3`

The compiler toolchain ensures 函数 prologues 具有 ``sspush
x1/x5`` save the return 地址 shadow 此外 the
regular   Similarly, 函数 epilogues 具有 ``ld x5,
偏移(x2)` followed by `sspopchk x5`` ensure 一popped 
来自 the regular matches the popped 来自 the shadow
鏍。

### 2. Shadow protections linux 内存 manager


作为 mentioned 更早, shadow stacks get encodings 
具有 一特殊 properties assigned them, along instructions
璇?operate 鍦?the shadow stacks:

- Regular stores shadow 内存 raise store access faults. 
  protects shadow 内存 来自 stray writes.

- Regular loads 来自 shadow 内存 allowed. allows
  trace utilities backtrace 函数 读取 the true call
  ensure 具有 已经 tampered 

- 浠?shadow 鏍?instructions 鍙?generate shadow 鏍?loads 鎴。
  shadow 鏍?stores.

- Shadow loads stores read-only 内存 raise AMO/store
  faults. 从两`sspush x1/x5` `sspopchk x1/x5` 
  raise AMO/store fault. simplies COW handling 内核
  鏈熼棿 fork(). The 鍐呮牳 鍙?convert shadow 鏍，椤，杩涘叆
  read-only 内存 (作为 执行 用于 regular read-write 内存).  作为
  soon 作为 后续 `sspush` `sspopchk` instructions 
  userspace encountered, the 内核 perform COW.

- Shadow 鏍?loads 鍜?stores 鍦?read-write 鎴?read-write-execute
  内存 raise 一access fault. 这是 一fatal condition 因为
  shadow loads stores 应当 从不 operating 
  read-write read-write-execute 内存.

### 3. ELF 鍜?psABI


The toolchain sets up `GNU_PROPERTY_RISCV_FEATURE_1_BCFI` 用于
property `GNU_PROPERTY_RISCV_FEATURE_1_AND` 鍦?the notes
section the 对象 文件.

### 4. Linux enabling


用户空间 programs 具有 多个 shared objects loaded 它们
地址 space.  s 一difficult task 确保 全部 the
dependencies 具有 已经 compiled shadow 支持.  从
s left the 动loader 启用 shadow stacks 用于 the
program.

### 5. prctl() enabling


`PR_SET_SHADOW_STACK_STATUS` / `PR_GET_SHADOW_STACK_STATUS` /
`PR_LOCK_SHADOW_STACK_STATUS` 鏄?three prctls added 鍒?manage shadow
enabling 用于 tasks.  这些 prctls architecture-agnostic return
-EINVAL 鑻，涓?implemented.

- prctl(PR_SET_SHADOW_栈_状 unsigned long arg)

鑻?arg = `PR_SHADOW_STACK_ENABLE` 鍜，鑻?CPU supports
`zicfiss` 然后 the 内核 启用 shadow stacks 用于 the task.
The 动loader issue `prctl` 一具有
determined 全部 the objects loaded 地址 space 具有 支持
用于 shadow stacks.  Additionally, 存在 一`dlopen` 
一对象 wasn't compiled `zicfiss`, the 动loader
鍙?issue 姝?prctl 涓?arg set 鍒?0 (i.e.
`PR_SHADOW_STACK_ENABLE` 正在 clear)

- prctl(PR_GET_SHADOW_栈_状 unsigned long * arg)

Returns the 电流 状indirect branch tracking. 已启
瀹?ll return `PR_SHADOW_STACK_ENABLE`.

- prctl(PR_锁_SHADOW_栈_状 unsigned long arg)

the 电流 状shadow enabling the
task. Userspace 希望 运行 一strict 安全 posture 
wouldn't 希望 loading objects `zicfiss` 支持.  
case userspace 使用 prctl disallow disabling shadow
stacks the 电流 task.

### 5. violations related returns shadow 已启


Pertaining shadow stacks, the CPU raises 一``软件 check
异常` upon executing `sspopchk x1/x5` if `x1/x5`` doesn't
match the top shadow   一mismatch happens, 然后 the CPU
sets `*tval = 3` raises the 异常.

The Linux 内核 treat 作为 一`SIGSEGV` code =
`SEGV_CPERR` follow the 正常 course 信号 delivery.

### 6. Shadow 鏍?tokens


Regular stores shadow stacks allowed 从t 
tampered 通过 arbitrary stray writes.  然 one 方法 
pivoting / switching 一shadow simply writing the CSR
`CSR_SSP`.  change the active shadow 用于 the
program.  Writes `CSR_SSP` the program 应当 mostly
limited 上下switches, unwinds, longjmp similar
mechanisms (类似 上下switching Green 线程) languages 类似
Go Rust. CSR_SSP writes problematic 因为 一attacker 
使用 内存 corruption bugs leverage 上下switching routines 
pivot 任何 shadow  Shadow tokens help mitigate 
problem 鐢?making sure 璇。

- 软件 switching away 来自 一shadow  the shadow
  指针 应当 saved the shadow itself (这是
  called the `shadow stack token`).

- 软件 switching 一shadow  应当 读取 the
  `shadow stack token` 来自 the shadow 指针 verify 
  the `shadow stack token` itself 一指针 the shadow 
  itself.

- 一the token verification 已完 软件 perform the 写入
  鍒?`CSR_SSP` 鍒?switch shadow stacks.

此处 "软件" 可以 参the 用户 模式 task runtime itself,
managing 各种 contexts 作为 part 一单个 线程.  "软件"
可以 参the 内核, the 内核 具有 deliver 一信号 
一用户 task 必须 save the shadow 指针.  The 内核 
perform similar procedure itself saving 一token the 用户 模式
task's shadow 鏍?  姝?way, whenever `sigreturn` happens,
the 内核 读取 verify the token 然后 switch the shadow
 使用 mechanism, the 内核 helps the 用户 task 因此 
任何 corruption issue the 用户 task exploited adversaries
arbitrarily 使用 `sigreturn`. Adversaries 具有 make
sure 存在 一valid `shadow stack token` 此外 
invoking `sigreturn`.

### 7. 淇″彿 shadow 鏍。

```

    struct __sc_riscv_cfi_state {
        unsigned long ss_ptr;
    };

```
作为 part 信号 delivery, the shadow token saved the
电流 shadow itself.  The updated 指针 saved away the
`ss_ptr` 字段 `__sc_riscv_cfi_state` 在…下
`sigcontext`. The existing shadow 分配 使用
用于 信号 delivery.  期间 `sigreturn`, 内核 obtain
`ss_ptr` 来自 `sigcontext`, verify the saved
token 鍦?the shadow 鏍? 鍜?switch the shadow 鏍。
