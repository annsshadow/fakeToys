
:Author: Deepak Gupta <debug@rivosinc.com>
:Date:   12 January 2024

## Tracking indirect control transfers on RISC-V Linux


本文档简要描述了 Linux 提供给用户空间的接口，用于在 RISC-V 上为用户态应用程序启用间接分支（indirect branch）跟踪。

### 1. Feature Overview


内存损坏问题通常会导致崩溃。然而，在富有创造力的攻击者手中，它们可能导致各种各样的安全问题。

其中一些安全问题属于代码复用攻击（code re-use attack），攻击者可以利用被损坏的函数指针，将它们串联起来执行面向跳转编程（JOP）或面向调用编程（COP），从而破坏程序的控制流完整性（CFI）。

函数指针位于可读写内存中，因此容易受到损坏。这允许攻击者控制程序计数器（PC）的值。在 RISC-V 上，zicfilp 扩展对这类间接控制转移施加了限制：

- 间接控制转移必须落在着陆垫（landing pad）指令 `lpad` 上。此规则有两个例外：

  - rs1 = x1 或 rs1 = x5，即从函数返回，而返回通过影子栈（shadow stack）保护（参见 zicfiss.rst）

  - rs1 = x7。在 RISC-V 上，编译器通常会执行以下操作以达到一个

```

      auipc x7, <imm>
      jalr (x7)

    这种形式的间接控制转移是不可变的，且不依赖内存。因此 rs1=x7 被豁免于跟踪，
    这些被称为软件保护跳转（software guarded jumps）。

```
`lpad` 指令是 `auipc rd, <imm_20bit>`（其中 `rd=x0`）的伪操作。这是一个 HINT 操作。`lpad` 指令必须按 4 字节边界对齐。它将 20 位立即数与 x7 进行比较。如果 `imm_20bit` == 0，CPU 不与 x7 执行任何比较。如果 `imm_20bit` != 0，则 `imm_20bit` 必须与 `x7` 匹配，否则 CPU 将引发 `software check exception`（`cause=18`），且 `*tval = 2`。

编译器可以在函数签名上生成哈希，并在调用点将其（截断为 20 位）设置到 x7 中。函数序言可以包含用相同函数哈希编码的 `lpad` 指令。这进一步减少了调用点所能到达的有效程序计数器地址数量。

### 2. ELF and psABI


工具链会在目标文件的 notes 段中为属性 `GNU_PROPERTY_RISCV_FEATURE_1_AND` 设置 `GNU_PROPERTY_RISCV_FEATURE_1_FCFI`。

### 3. Linux enabling


用户空间程序可以在其地址空间中加载多个共享对象。要确保所有的依赖项都已经使用间接分支支持编译，是一项困难的任务。因此，为程序启用间接分支跟踪的工作留给了动态加载器（dynamic loader）。

### 4. prctl() enabling


每个任务的间接分支跟踪状态可以通过 `PR_GET_CFI` 和 `PR_SET_CFI` 这两个 `prctl()` 参数（分别对应）进行监控和控制，方法是提供 `PR_CFI_BRANCH_LANDING_PADS` 作为第二个参数。这些是与架构无关的，如果底层功能不受支持，将返回 -EINVAL。

- prctl(`PR_SET_CFI`, `PR_CFI_BRANCH_LANDING_PADS`, unsigned long arg)

arg 是一个位掩码。

如果 arg 中设置了 `PR_CFI_ENABLE`，且 CPU 支持 `zicfilp`，则内核将为该任务启用间接分支跟踪。动态加载器一旦确定地址空间中加载的所有对象都支持间接分支跟踪，就可以发出此 `prctl()`。

间接分支跟踪状态在启用后也可以被锁定。这会阻止该任务随后将其禁用。这是通过设置 arg 中的 `PR_CFI_LOCK` 位来完成的。要么该任务的间接分支跟踪必须已经启用，要么 arg 中也必须设置了 `PR_CFI_ENABLE` 位。这适用于希望以严格安全态势运行、且不希望加载不支持 `zicfilp` 的对象的场景。

也可以为该任务禁用间接分支跟踪（假设它此前尚未被启用并锁定）。如果有一个未使用 `zicfilp` 编译的对象被 `dlopen()`，动态加载器可以发出此 `prctl()`，并将 arg 设置为 `PR_CFI_DISABLE`。如果该任务的间接分支跟踪此前已被启用并锁定，则无法禁用它。


- prctl(`PR_GET_CFI`, `PR_CFI_BRANCH_LANDING_PADS`, unsigned long * arg)

将间接分支跟踪的当前状态返回到一个位掩码中，该位掩码存储在 arg 所指向的内存位置。如果当前已为该任务启用间接分支跟踪，则该位掩码会设置 `PR_CFI_ENABLE` 位；如果它被锁定，还会额外设置 `PR_CFI_LOCK` 位。如果当前已为该任务禁用间接分支跟踪，则会设置 `PR_CFI_DISABLE` 位。


### 5. violations related to indirect branch tracking


关于间接分支跟踪，CPU 在以下情况下会引发 software check exception：

- 间接调用 / jmp 之后缺少 `lpad`
- `lpad` 未位于 4 字节边界上
- 嵌入 `lpad` 指令中的 `imm_20bit` 与 `x7` 不匹配

在这 3 种情况下，都会捕获 `*tval = 2` 并引发 software check exception（`cause=18`）。

内核会将其视为 `SIGSEGV`，代码为 `SEGV_CPERR`，并按照正常的信号投递流程处理。
