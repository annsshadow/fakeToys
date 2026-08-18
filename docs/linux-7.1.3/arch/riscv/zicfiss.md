
:Author: Deepak Gupta <debug@rivosinc.com>
:Date:   12 January 2024

## Shadow 鏍?鍒?protect 鍑芥暟 returns 鍦?RISC-V Linux


姝?document briefly describes the 鎺ュ彛 provided 鍒?userspace 鐢?Linux
鍒?鍚敤 shadow stacks 鐢ㄤ簬 鐢ㄦ埛 妯″紡 applications 鍦?RISC-V.

### 1. 鐗规€?Overview


鍐呭瓨 corruption issues 閫氬父 result 鍦?crashes.  鐒惰€? 鍦?the
hands 鐨?涓€涓?creative adversary, 杩欎簺 issues 鍙?result 鍦?涓€涓?variety 鐨?
瀹夊叏 problems.

涓€浜?鐨?閭ｄ簺 瀹夊叏 issues 鍙?涓?code re-use attacks 鍦?programs
浣曞 涓€涓?adversary 鍙?浣跨敤 corrupt return 鍦板潃 present 鍦?the
鏍? chaining them together 鍒?perform return oriented programming
(ROP) 鍜?浠庤€?compromising the control flow integrity (CFI) 鐨?the
program.

Return 鍦板潃 瀹炴椂 鍦?the 鏍?鍦?read-write 鍐呭瓨.  鍥犳
瀹冧滑鏄?susceptible 鍒?corruption, 鍏?allows 涓€涓?adversary 鍒?
control the program counter. 鍦?RISC-V, the `zicfiss` extension
鎻愪緵 涓€涓?alternate 鏍?(the "shadow 鏍?) 鍦?鍏?return
鍦板潃 鍙?涓?safely placed 鍦?the prologue 鐨?the 鍑芥暟 鍜?
retrieved 鍦?the epilogue.  The `zicfiss` extension makes the
浠ヤ笅 changes:

- PTE encodings 鐢ㄤ簬 shadow 鏍?铏氭嫙 鍐呭瓨
  涓€涓?鏇存棭 reserved encoding 鍦?绗竴 stage translation i.e.
  PTE.R=0, PTE.W=1, PTE.X=0  becomes the PTE encoding 鐢ㄤ簬 shadow 鏍?椤?

- The `sspush x1/x5` instruction pushes (stores) `x1/x5` 鍒?shadow 鏍?

- The `sspopchk x1/x5` instruction pops (loads) 鏉ヨ嚜 shadow 鏍?鍜?compares
  涓?`x1/x5` 鍜?鑻?涓?equal, the CPU raises 涓€涓?`software check exception`
  涓?`*tval = 3`

The compiler toolchain ensures 璇?鍑芥暟 prologues 鍏锋湁 ``sspush
x1/x5`` 鍒?save the return 鍦板潃 鍦?shadow 鏍?姝ゅ 鍒?the
regular 鏍?  Similarly, 鍑芥暟 epilogues 鍏锋湁 ``ld x5,
鍋忕Щ(x2)` followed by `sspopchk x5`` 鍒?ensure 璇?涓€涓?popped 鍊?
鏉ヨ嚜 the regular 鏍?matches 涓?the popped 鍊?鏉ヨ嚜 the shadow
鏍?

### 2. Shadow 鏍?protections 鍜?linux 鍐呭瓨 manager


浣滀负 mentioned 鏇存棭, shadow stacks get 鏂?椤?琛?encodings 璇?
鍏锋湁 涓€浜?鐗规畩 properties assigned 鍒?them, along 涓?instructions
璇?operate 鍦?the shadow stacks:

- Regular stores 鍒?shadow 鏍?鍐呭瓨 raise store access faults. 姝?
  protects shadow 鏍?鍐呭瓨 鏉ヨ嚜 stray writes.

- Regular loads 鏉ヨ嚜 shadow 鏍?鍐呭瓨 鏄?allowed. 姝?allows
  鏍?trace utilities 鎴?backtrace 鍑芥暟 鍒?璇诲彇 the true call
  鏍?鍜?ensure 璇?瀹?鍏锋湁 涓?宸茬粡 tampered 涓?

- 浠?shadow 鏍?instructions 鍙?generate shadow 鏍?loads 鎴?
  shadow 鏍?stores.

- Shadow 鏍?loads 鍜?stores 鍦?read-only 鍐呭瓨 raise AMO/store
  椤?faults. 浠庤€?涓よ€?`sspush x1/x5` 鍜?`sspopchk x1/x5` 灏?
  raise AMO/store 椤?fault. 姝?simplies COW handling 鍦?鍐呮牳
  鏈熼棿 fork(). The 鍐呮牳 鍙?convert shadow 鏍?椤?杩涘叆
  read-only 鍐呭瓨 (浣滀负 瀹?鎵ц 鐢ㄤ簬 regular read-write 鍐呭瓨).  浣滀负
  soon 浣滀负 鍚庣画 `sspush` 鎴?`sspopchk` instructions 鍦?
  userspace 鏄?encountered, the 鍐呮牳 鍙?perform COW.

- Shadow 鏍?loads 鍜?stores 鍦?read-write 鎴?read-write-execute
  鍐呭瓨 raise 涓€涓?access fault. 杩欐槸 涓€涓?fatal condition 鍥犱负
  shadow 鏍?loads 鍜?stores 搴斿綋 浠庝笉 涓?operating 鍦?
  read-write 鎴?read-write-execute 鍐呭瓨.

### 3. ELF 鍜?psABI


The toolchain sets up `GNU_PROPERTY_RISCV_FEATURE_1_BCFI` 鐢ㄤ簬
property `GNU_PROPERTY_RISCV_FEATURE_1_AND` 鍦?the notes
section 鐨?the 瀵硅薄 鏂囦欢.

### 4. Linux enabling


鐢ㄦ埛绌洪棿 programs 鍙?鍏锋湁 澶氫釜 shared objects loaded 鍦?瀹冧滑鐨?
鍦板潃 space.  瀹?s 涓€涓?difficult task 鍒?纭繚 鍏ㄩ儴 the
dependencies 鍏锋湁 宸茬粡 compiled 涓?shadow 鏍?鏀寔.  浠庤€?
瀹?s left 鍒?the 鍔ㄦ€?loader 鍒?鍚敤 shadow stacks 鐢ㄤ簬 the
program.

### 5. prctl() enabling


`PR_SET_SHADOW_STACK_STATUS` / `PR_GET_SHADOW_STACK_STATUS` /
`PR_LOCK_SHADOW_STACK_STATUS` 鏄?three prctls added 鍒?manage shadow
鏍?enabling 鐢ㄤ簬 tasks.  杩欎簺 prctls 鏄?architecture-agnostic 鍜?return
-EINVAL 鑻?涓?implemented.

- prctl(PR_SET_SHADOW_鏍坃鐘舵€? unsigned long arg)

鑻?arg = `PR_SHADOW_STACK_ENABLE` 鍜?鑻?CPU supports
`zicfiss` 鐒跺悗 the 鍐呮牳 灏?鍚敤 shadow stacks 鐢ㄤ簬 the task.
The 鍔ㄦ€?loader 鍙?issue 姝?`prctl` 涓€鏃?瀹?鍏锋湁
determined 璇?鍏ㄩ儴 the objects loaded 鍦?鍦板潃 space 鍏锋湁 鏀寔
鐢ㄤ簬 shadow stacks.  Additionally, 鑻?瀛樺湪 涓€涓?`dlopen` 鍒?
涓€涓?瀵硅薄 鍏?wasn't compiled 涓?`zicfiss`, the 鍔ㄦ€?loader
鍙?issue 姝?prctl 涓?arg set 鍒?0 (i.e.
`PR_SHADOW_STACK_ENABLE` 姝ｅ湪 clear)

- prctl(PR_GET_SHADOW_鏍坃鐘舵€? unsigned long * arg)

Returns the 鐢垫祦 鐘舵€?鐨?indirect branch tracking. 鑻?宸插惎鐢?
瀹?ll return `PR_SHADOW_STACK_ENABLE`.

- prctl(PR_閿乢SHADOW_鏍坃鐘舵€? unsigned long arg)

閿?the 鐢垫祦 鐘舵€?鐨?shadow 鏍?enabling 鍦?the
task. Userspace 鍙?甯屾湜 鍒?杩愯 涓?涓€涓?strict 瀹夊叏 posture 鍜?
wouldn't 甯屾湜 loading 鐨?objects 鏃?`zicfiss` 鏀寔.  鍦?姝?
case userspace 鍙?浣跨敤 姝?prctl 鍒?disallow disabling 鐨?shadow
stacks 鍦?the 鐢垫祦 task.

### 5. violations related 鍒?returns 涓?shadow 鏍?宸插惎鐢?


Pertaining 鍒?shadow stacks, the CPU raises 涓€涓?``杞欢 check
寮傚父` upon executing `sspopchk x1/x5` if `x1/x5`` doesn't
match the top 鐨?shadow 鏍?  鑻?涓€涓?mismatch happens, 鐒跺悗 the CPU
sets `*tval = 3` 鍜?raises the 寮傚父.

The Linux 鍐呮牳 灏?treat 姝?浣滀负 涓€涓?`SIGSEGV` 涓?code =
`SEGV_CPERR` 鍜?follow the 姝ｅ父 course 鐨?淇″彿 delivery.

### 6. Shadow 鏍?tokens


Regular stores 鍦?shadow stacks 鏄?涓?allowed 鍜?浠庤€?鍙?t 涓?
tampered 涓?閫氳繃 arbitrary stray writes.  鐒惰€? one 鏂规硶 鐨?
pivoting / switching 鍒?涓€涓?shadow 鏍?鏄?simply writing 鍒?the CSR
`CSR_SSP`.  姝?灏?change the active shadow 鏍?鐢ㄤ簬 the
program.  Writes 鍒?`CSR_SSP` 鍦?the program 搴斿綋 涓?mostly
limited 鍒?涓婁笅鏂?switches, 鏍?unwinds, 鎴?longjmp 鎴?similar
mechanisms (绫讳技 涓婁笅鏂?switching 鐨?Green 绾跨▼) 鍦?languages 绫讳技
Go 鍜?Rust. CSR_SSP writes 鍙?涓?problematic 鍥犱负 涓€涓?attacker 鍙?
浣跨敤 鍐呭瓨 corruption bugs 鍜?leverage 涓婁笅鏂?switching routines 鍒?
pivot 鍒?浠讳綍 shadow 鏍? Shadow 鏍?tokens 鍙?help mitigate 姝?
problem 鐢?making sure 璇?

- 褰?杞欢 鏄?switching away 鏉ヨ嚜 涓€涓?shadow 鏍? the shadow
  鏍?鎸囬拡 搴斿綋 涓?saved 鍦?the shadow 鏍?itself (杩欐槸
  called the `shadow stack token`).

- 褰?杞欢 鏄?switching 鍒?涓€涓?shadow 鏍? 瀹?搴斿綋 璇诲彇 the
  `shadow stack token` 鏉ヨ嚜 the shadow 鏍?鎸囬拡 鍜?verify 璇?
  the `shadow stack token` itself 鏄?涓€涓?鎸囬拡 鍒?the shadow 鏍?
  itself.

- 涓€鏃?the token verification 鏄?宸插畬鎴? 杞欢 鍙?perform the 鍐欏叆
  鍒?`CSR_SSP` 鍒?switch shadow stacks.

姝ゅ "杞欢" 鍙互 鍙傝€?鍒?the 鐢ㄦ埛 妯″紡 task runtime itself,
managing 鍚勭 contexts 浣滀负 part 鐨?涓€涓?鍗曚釜 绾跨▼.  鎴?"杞欢"
鍙互 鍙傝€?鍒?the 鍐呮牳, 褰?the 鍐呮牳 鍏锋湁 鍒?deliver 涓€涓?淇″彿 鍒?
涓€涓?鐢ㄦ埛 task 鍜?蹇呴』 save the shadow 鏍?鎸囬拡.  The 鍐呮牳 鍙?
perform similar procedure itself 鐢?saving 涓€涓?token 鍦?the 鐢ㄦ埛 妯″紡
task's shadow 鏍?  姝?way, whenever `sigreturn` happens,
the 鍐呮牳 鍙?璇诲彇 鍜?verify the token 鍜?鐒跺悗 switch 鍒?the shadow
鏍? 浣跨敤 姝?mechanism, the 鍐呮牳 helps the 鐢ㄦ埛 task 鍥犳 璇?
浠讳綍 corruption issue 鍦?the 鐢ㄦ埛 task 鏄?涓?exploited 鐢?adversaries
arbitrarily 浣跨敤 `sigreturn`. Adversaries 灏?鍏锋湁 鍒?make
sure 璇?瀛樺湪 涓€涓?valid `shadow stack token` 姝ゅ 鍒?
invoking `sigreturn`.

### 7. 淇″彿 shadow 鏍?

```

    struct __sc_riscv_cfi_state {
        unsigned long ss_ptr;
    };

```
浣滀负 part 鐨?淇″彿 delivery, the shadow 鏍?token 鏄?saved 鍦?the
鐢垫祦 shadow 鏍?itself.  The updated 鎸囬拡 鏄?saved away 鍦?the
`ss_ptr` 瀛楁 鍦?`__sc_riscv_cfi_state` 鍦ㄢ€︿笅
`sigcontext`. The existing shadow 鏍?鍒嗛厤 鏄?浣跨敤
鐢ㄤ簬 淇″彿 delivery.  鏈熼棿 `sigreturn`, 鍐呮牳 灏?obtain
`ss_ptr` 鏉ヨ嚜 `sigcontext`, verify the saved
token 鍦?the shadow 鏍? 鍜?switch the shadow 鏍?
