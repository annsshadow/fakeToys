## Ptrace


GDB 打算支持 BookE 处理器的以下硬件调试特性：

4 个硬件断点（IAC）
2 个硬件观察点（读、写和读-写）（DAC）
2 个用于硬件观察点的值条件（DVC）

为此，我们需要扩展 ptrace，以便 GDB 能够查询并设置这些资源。由于我们正在扩展，我们试图创建一个可扩展、并且同时覆盖 BookE 和服务器处理器的接口，这样 GDB 就不必对它们各自做特殊处理。我们添加了以下 3 个新的 ptrace 请求。

## 1. PPC_PTRACE_GETHWDBGINFO


供 GDB 查询以发现硬件调试特性。这里要返回的主要信息是针对硬件观察点的最小对齐。BookE 处理器在此没有限制，但服务器处理器对硬件观察点有 8 字节对齐的限制。我们希望避免在 GDB 中基于它在 AUXV 中看到的内容添加特殊情况。

既然在做这件事，我们还添加了内核可以返回给 GDB 的其他有用信息：该查询将返回硬件断点的数量、硬件观察点的数量，以及它是否支持一段地址范围和一个条件。
```

  struct ppc_debug_info {
       unit32_t version;
       unit32_t num_instruction_bps;
       unit32_t num_data_bps;
       unit32_t num_condition_regs;
       unit32_t data_bp_alignment;
       unit32_t sizeof_condition; /* size of the DVC register */
       uint64_t features; /* bitmask of the individual flags */
  };

```
```

  #define PPC_DEBUG_FEATURE_INSN_BP_RANGE		0x1
  #define PPC_DEBUG_FEATURE_INSN_BP_MASK		0x2
  #define PPC_DEBUG_FEATURE_DATA_BP_RANGE		0x4
  #define PPC_DEBUG_FEATURE_DATA_BP_MASK		0x8
  #define PPC_DEBUG_FEATURE_DATA_BP_DAWR		0x10
  #define PPC_DEBUG_FEATURE_DATA_BP_ARCH_31		0x20

```
2. PPC_PTRACE_SETHWDEBUG

```

  struct ppc_hw_breakpoint {
        uint32_t version;
  #define PPC_BREAKPOINT_TRIGGER_EXECUTE  0x1
  #define PPC_BREAKPOINT_TRIGGER_READ     0x2
 #define PPC_BREAKPOINT_TRIGGER_WRITE    0x4
        uint32_t trigger_type;       /* only some combinations allowed */
  #define PPC_BREAKPOINT_MODE_EXACT               0x0
  #define PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE     0x1
  #define PPC_BREAKPOINT_MODE_RANGE_EXCLUSIVE     0x2
  #define PPC_BREAKPOINT_MODE_MASK                0x3
        uint32_t addr_mode;          /* address match mode */

  #define PPC_BREAKPOINT_CONDITION_MODE   0x3
  #define PPC_BREAKPOINT_CONDITION_NONE   0x0
  #define PPC_BREAKPOINT_CONDITION_AND    0x1
  #define PPC_BREAKPOINT_CONDITION_EXACT  0x1	/* different name for the same thing as above */
  #define PPC_BREAKPOINT_CONDITION_OR     0x2
  #define PPC_BREAKPOINT_CONDITION_AND_OR 0x3
  #define PPC_BREAKPOINT_CONDITION_BE_ALL 0x00ff0000	/* byte enable bits */
  #define PPC_BREAKPOINT_CONDITION_BE(n)  (1<<((n)+16))
        uint32_t condition_mode;     /* break/watchpoint condition flags */

        uint64_t addr;
        uint64_t addr2;
        uint64_t condition_value;
  };

```
一个请求指定一个事件，而不一定只是要设置的一个寄存器。例如，如果请求是一个带条件的观察点，DAC 和 DVC 寄存器都将在同一个请求中被设置。

通过这种方式，GDB 可以请求 BookE 支持的所有类型的硬件断点和观察点。服务器处理器中可用的 COMEFROM 断点不在考虑之列，但这超出了本工作的范围。

ptrace 将返回一个唯一标识刚刚创建的断点或观察点的整数（句柄）。该整数将在 PPC_PTRACE_DELHWDEBUG 请求中用于请求删除它。如果所请求的断点无法在寄存器上分配，则返回 -ENOSPC。

下面是使用该结构的一些示例：

```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_EXECUTE;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = 0;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_READ;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = 0;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_READ;
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_AND | PPC_BREAKPOINT_CONDITION_BE_ALL;
    p.addr            = (uint64_t) address;
    p.addr2           = 0;
    p.condition_value = (uint64_t) condition;

```
```

    p.version         = PPC_DEBUG_CURRENT_VERSION;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_EXECUTE;
    p.addr_mode       = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) begin_range;
    p.addr2           = (uint64_t) end_range;
    p.condition_value = 0;

```
```

    p.version         = 1;
    p.trigger_type    = PPC_BREAKPOINT_TRIGGER_RW;
    p.addr_mode       = PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE;
    or
    p.addr_mode       = PPC_BREAKPOINT_MODE_EXACT;

    p.condition_mode  = PPC_BREAKPOINT_CONDITION_NONE;
    p.addr            = (uint64_t) begin_range;
    /* For PPC_BREAKPOINT_MODE_RANGE_INCLUSIVE addr2 needs to be specified, where
     * addr2 - addr <= 8 Bytes.
     */
    p.addr2           = (uint64_t) end_range;
    p.condition_value = 0;

```
3. PPC_PTRACE_DELHWDEBUG


接受一个标识现有断点或观察点的整数（即 PTRACE_SETHWDEBUG 返回的值），并删除相应的断点或观察点。
