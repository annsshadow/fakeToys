## 汇编器注解


Copyright (c) 2017-2019 Jiri Slaby

本文档描述了用于在汇编中标注数据和代码的新宏。特别是，它包含有关 `SYM_FUNC_START`、`SYM_FUNC_END`、`SYM_CODE_START` 等宏的信息。

### 动机

某些代码（如入口、跳板或启动代码）需要用汇编编写。与 C 语言一样，这类代码被分组为函数并附带数据。标准汇编器并不强制用户精确地将那些片段标记为代码、数据，甚至不要求指定其长度。然而，汇编器会向开发者提供此类注解，以辅助整个汇编过程中的调试器。此外，开发者还希望将某些函数标记为 **global（全局）**，以便在翻译单元之外可见。

随着时间的推移，Linux 内核采纳了来自多个项目（如 `binutils`）的宏来简化此类注解。因此出于历史原因，开发者一直在汇编中使用 `ENTRY`、`END`、`ENDPROC` 等注解。由于缺乏相关文档，这些宏在某些地方被用在了相当错误的上下文中。显然，`ENTRY` 旨在表示全局符号（无论是数据还是代码）的开头。`END` 用于标记数据的结束或具有 **non-standard（非标准）** 调用约定的特殊函数的结束。相比之下，`ENDPROC` 只应注解 **standard（标准）** 函数的结尾。

当这些宏被正确使用时，它们能帮助汇编器生成大小和类型都被正确设置的理想目标文件。例如，以下命令的输出结果：

```
   Num:    Value          Size Type    Bind   Vis      Ndx Name
    25: 0000000000000000    33 FUNC    GLOBAL DEFAULT    1 __put_user_1
    29: 0000000000000030    37 FUNC    GLOBAL DEFAULT    1 __put_user_2
    32: 0000000000000060    36 FUNC    GLOBAL DEFAULT    1 __put_user_4
    35: 0000000000000090    37 FUNC    GLOBAL DEFAULT    1 __put_user_8

```

这不仅对调试很重要。当存在像这样被正确注解的目标文件时，可以在其上运行工具来生成更有用的信息。特别是，在正确注解的目标文件上，可以运行 `objtool` 来检查并在需要时修复目标文件。目前，`objtool` 可以报告函数中缺失的帧指针建立/销毁。它还可以为大多数代码自动生成 ORC unwinder（Documentation/arch/x86/orc-unwinder.rst）所需的注解。这两者对于支持可靠的栈回溯尤其重要，而可靠的栈回溯又是内核实时补丁（Documentation/livepatch/livepatch.rst）所必需的。

### 注意事项与讨论

正如有人可能已经意识到的，以前只有三个宏。这确实不足以覆盖所有情况组合：

- 标准/非标准函数
- 代码/数据
- 全局/局部符号

曾经有过一次 discussion_，并且没有扩展当前的 `ENTRY/END*`

```
    So how about using macro names that actually show the purpose, instead
    of importing all the crappy, historic, essentially randomly chosen
    debug symbol macro names from the binutils and older kernels?

```

### 宏说明


新宏以 `SYM_` 前缀开头，可分为三个主要类别：

1. `SYM_FUNC_*` —— 用于标注类 C 函数。即采用标准 C 调用约定的函数。例如，在 x86 上，这意味着栈在预定位置包含一个返回地址，并且函数的返回可以以标准方式进行。当启用帧指针时，帧指针的保存/恢复也应当分别在函数的开头/结尾进行。

   诸如 `objtool` 之类的检查工具应确保这些被标注的函数符合这些规则。这些工具还可以轻易地用调试信息（如 **ORC data**）自动注解这些函数。

2. `SYM_CODE_*` —— 使用特殊栈调用的特殊函数。可以是带有特殊栈内容的中断处理程序、跳板或启动函数。

   检查工具大多忽略对这些函数的检查。但仍可自动生成部分调试信息。为了获得正确的调试数据，这段代码需要开发者提供诸如 `UNWIND_HINT_REGS` 之类的提示。

3. `SYM_DATA*` —— 显然是属于 `.data` 段而非 `.text` 段的数据。数据不包含指令，因此工具必须对其进行特殊处理：既不能将这些字节当作指令，也不能为其分配任何调试信息。

#### 指令宏

本节涵盖上文列举的 `SYM_FUNC_**` 和 `SYM_CODE_**`。

`objtool` 要求所有代码都必须包含在一个 ELF 符号中。带有 `.L` 前缀的符号名不会生成符号表项。带有 `.L` 前缀的符号可以在代码区域内使用，但应避免用于通过 `SYM_*_START/END` 注解来表示一段代码范围。

- `SYM_FUNC_START` 与 `SYM_FUNC_START_LOCAL` 应当是 **最常用** 的标记。它们用于具有标准调用约定的函数——全局和局部函数。与 C 语言类似，二者都会将函数按架构特定的 `__ALIGN` 字节对齐。也存在 `_NOALIGN` 变体，用于开发者不希望进行这种隐式对齐的特殊情况。

  `SYM_FUNC_START_WEAK` 与 `SYM_FUNC_START_WEAK_NOALIGN` 标记也作为 C 语言中已知的 **weak** 属性的汇编对应物提供。

  所有这些标记 **都** 应与 `SYM_FUNC_END` 配对使用。首先，它将指令序列标记为一个函数并计算其大小写入生成的目标文件。其次，这也简化了此类目标文件的检查与处理，因为工具可以轻松找到确切的函数边界。

  因此，在大多数情况下，开发者应该编写类似以下的内容：

```
    SYM_FUNC_START(memset)
        ... asm insns ...
    SYM_FUNC_END(memset)

  In fact, this kind of annotation corresponds to the now deprecated ``ENTRY``
  and ``ENDPROC`` macros.

```
- `SYM_FUNC_ALIAS`、`SYM_FUNC_ALIAS_LOCAL` 以及 `SYM_FUNC_ALIAS_WEAK` 可以

```
    SYM_FUNC_START(__memset)
        ... asm insns ...
    SYN_FUNC_END(__memset)
    SYM_FUNC_ALIAS(memset, __memset)

  In this example, one can call ``__memset`` or ``memset`` with the same
  result, except the debug information for the instructions is generated to
  the object file only once -- for the non-``ALIAS`` case.

```
- `SYM_CODE_START` 与 `SYM_CODE_START_LOCAL` 应仅用于特殊情况——即你清楚自己在做什么。它专用于中断处理程序及调用约定非 C 约定的类似场景。也存在 `_NOALIGN` 变体。其用法与 `FUNC`

```
    SYM_CODE_START_LOCAL(bad_put_user)
        ... asm insns ...
    SYM_CODE_END(bad_put_user)

  Again, every ``SYM_CODE_START*`` **shall** be coupled by ``SYM_CODE_END``.

  To some extent, this category corresponds to deprecated ``ENTRY`` and
  ``END``. Except ``END`` had several other meanings too.

```
- `SYM_INNER_LABEL*` 用于表示某些 `SYM_{CODE,FUNC}_START` 与 `SYM_{CODE,FUNC}_END` 内部的标签。它们非常类似于

```
    SYM_CODE_START(ftrace_caller)
        /* save_mcount_regs fills in first two parameters */
        ...

    SYM_INNER_LABEL(ftrace_caller_op_ptr, SYM_L_GLOBAL)
        /* Load the ftrace_ops into the 3rd parameter */
        ...

    SYM_INNER_LABEL(ftrace_call, SYM_L_GLOBAL)
        call ftrace_stub
        ...
        retq
    SYM_CODE_END(ftrace_caller)

```
#### 数据宏

与指令类似，也有几个宏用于描述汇编中的数据。

- `SYM_DATA_START` 与 `SYM_DATA_START_LOCAL` 标记某些数据的起始，并应与 `SYM_DATA_END` 或 `SYM_DATA_END_LABEL` 配合使用。后者还会在结尾添加一个标签，以便人们可以使用 `lstack` 以及（局部的）`lstack_end`，如下所示：

```
    SYM_DATA_START_LOCAL(lstack)
        .skip 4096
    SYM_DATA_END_LABEL(lstack, SYM_L_LOCAL, lstack_end)

```
- `SYM_DATA` 与 `SYM_DATA_LOCAL` 是用于简单、大多为一行的

```
    SYM_DATA(HEAP,     .long rm_heap)
    SYM_DATA(heap_end, .long rm_stack)

  In the end, they expand to ``SYM_DATA_START`` with ``SYM_DATA_END``
  internally.

```
#### 辅助宏

上述所有宏最终都会归结为对 `SYM_START`、`SYM_END` 或 `SYM_ENTRY` 的某种调用。通常，开发者应避免使用这些。此外，在上述示例中，可以看到 `SYM_L_LOCAL`。还有 `SYM_L_GLOBAL` 与 `SYM_L_WEAK`。它们都用于表示被其标记的符号的链接属性。它们既用于前述宏的 `_LABEL` 变体中，也用于 `SYM_START`。


#### 覆盖宏

架构也可以在自己的 `asm/linkage.h` 中覆盖任意宏，包括指定符号类型的宏（`SYM_T_FUNC`、`SYM_T_OBJECT` 以及 `SYM_T_NONE`）。由于本文档中描述的每个宏都被 `#ifdef` + `#endif` 包围，只需在上述架构相关的头文件中以不同方式定义这些宏即可。
