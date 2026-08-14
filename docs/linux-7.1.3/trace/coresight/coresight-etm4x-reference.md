## ETMv4 sysfs Linux 驱动编程参考。


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     October 11th, 2019

作为现有 ETMv4 驱动文档的补充。

### Sysfs 文件与目录


Root: `/sys/bus/coresight/devices/etm<N>`


以下段落说明了 sysfs 文件与它们所影响的 ETMv4 寄存器之间的关联。注意寄存器名称以省略 ‘TRC’ 前缀的形式给出。

----

:File:            `mode` (rw)
:Trace Registers: {CONFIGR + others}
:Notes:
    位选择跟踪特性。参见下文的 ‘mode’ 小节。设置其中的位将导致对跟踪配置寄存器
    及其他寄存器进行等价的编程，以启用所请求的特性。

:Syntax & eg:
    `echo bitfield > mode`

    bitfield 最多 32 位，用于设置跟踪特性。

:Example:
    `$> echo 0x012 > mode`

----

:File:            `reset` (wo)
:Trace Registers: All
:Notes:
    将所有编程复位为不产生任何跟踪 / 未编程任何逻辑。

:Syntax:
    `echo 1 > reset`

----

:File:            `enable_source` (wo)
:Trace Registers: PRGCTLR, All hardware regs.
:Notes:
    - > 0 : 使用驱动中保存的当前值对硬件进行编程并启用跟踪。

    - = 0 : 禁用跟踪硬件。

:Syntax:
    `echo 1 > enable_source`

----

:File:            `cpu` (ro)
:Trace Registers: None.
:Notes:
    此 ETM 所连接的 CPU ID。

:Example:
    `$> cat cpu`

    `$> 0`

----

:File:            `ts_source` (ro)
:Trace Registers: None.
:Notes:
    当实现了 FEAT_TRF 时，为跟踪会话所用 TRFCR_ELx.TS 的值。否则 -1
    表示未知的时间源。检查 trcidr0.tssize 以查看是否存在全局时间戳。

:Example:
    `$> cat ts_source`

    `$> 1`

----

:File:            `addr_idx` (rw)
:Trace Registers: None.
:Notes:
    用于索引地址比较器和范围特性的虚拟寄存器。为范围中的一对比较器
    设置第一个的索引。

:Syntax:
    `echo idx > addr_idx`

    其中 idx < nr_addr_cmp x 2

----

:File:            `addr_range` (rw)
:Trace Registers: ACVR[idx, idx+1], VIIECTLR
:Notes:
    由 addr_idx 选择的某个范围对应的地址对。根据可选参数进行包含 / 排除；
    若省略则使用当前 ‘mode’ 设置。在控制寄存器中选择比较器范围。
    索引为奇数时报错。

:Depends: `mode, addr_idx`
:Syntax:
   `echo addr1 addr2 [exclude] > addr_range`

   其中 addr1 与 addr2 界定该范围，且 addr1 < addr2。

   Optional exclude value:-

   - 0 for include
   - 1 for exclude.
:Example:
   `$> echo 0x0000 0x2000 0 > addr_range`

----

:File:            `addr_single` (rw)
:Trace Registers: ACVR[idx]
:Notes:
    根据 addr_idx 设置一个独立的地址比较器。当该地址比较器用作事件
    生成逻辑等的一部分时使用。

:Depends: `addr_idx`
:Syntax:
   `echo addr1 > addr_single`

----

:File:           `addr_start` (rw)
:Trace Registers: ACVR[idx], VISSCTLR
:Notes:
    根据 addr_idx 设置跟踪起始地址比较器。在控制寄存器中选择比较器。

:Depends: `addr_idx`
:Syntax:
    `echo addr1 > addr_start`

----

:File:            `addr_stop` (rw)
:Trace Registers: ACVR[idx], VISSCTLR
:Notes:
    根据 addr_idx 设置跟踪停止地址比较器。在控制寄存器中选择比较器。

:Depends: `addr_idx`
:Syntax:
    `echo addr1 > addr_stop`

----

:File:            `addr_context` (rw)
:Trace Registers: ACATR[idx,{6:4}]
:Notes:
    将上下文 ID 比较器链接到地址比较器 addr_idx

:Depends: `addr_idx`
:Syntax:
    `echo ctxt_idx > addr_context`

    其中 ctxt_idx 为所链接的上下文 id / vmid 比较器的索引。

----

:File:            `addr_ctxtype` (rw)
:Trace Registers: ACATR[idx,{3:2}]
:Notes:
    输入值字符串。为所链接的上下文 ID 比较器设置类型

:Depends: `addr_idx`
:Syntax:
    `echo type > addr_ctxtype`

    类型为 {all, vmid, ctxid, none} 之一
:Example:
    `$> echo ctxid > addr_ctxtype`

----

:File:            `addr_exlevel_s_ns` (rw)
:Trace Registers: ACATR[idx,{14:8}]
:Notes:
    为所选地址比较器设置 ELx 安全与非安全匹配位

:Depends: `addr_idx`
:Syntax:
    `echo val > addr_exlevel_s_ns`

    val 为用于排除的异常级别对应的 7 位值。输入值在寄存器中被移位到正确的位。
:Example:
    `$> echo 0x4F > addr_exlevel_s_ns`

----

:File:            `addr_instdatatype` (rw)
:Trace Registers: ACATR[idx,{1:0}]
:Notes:
    设置用于匹配的地址比较器类型。驱动仅支持设置为指令地址类型。

:Depends: `addr_idx`

----

:File:            `addr_cmp_view` (ro)
:Trace Registers: ACVR[idx, idx+1], ACATR[idx], VIIECTLR
:Notes:
    读取当前选中的地址比较器。如果属于某个地址范围，则显示两个地址。

:Depends: `addr_idx`
:Syntax:
    `cat addr_cmp_view`
:Example:
    `$> cat addr_cmp_view`

   `addr_cmp[^0^] range 0x0 0xffffffffffffffff include ctrl(0x4b00)`

----

:File:            `nr_addr_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    地址比较器对的数量

----

:File:            `sshot_idx` (rw)
:Trace Registers: None
:Notes:
    选择单次触发（single shot）寄存器组。

----

:File:            `sshot_ctrl` (rw)
:Trace Registers: SSCCR[idx]
:Notes:
    访问单次触发比较器控制寄存器。

:Depends: `sshot_idx`
:Syntax:
    `echo val > sshot_ctrl`

    将 val 写入所选控制寄存器。

----

:File:            `sshot_status` (ro)
:Trace Registers: SSCSR[idx]
:Notes:
    读取单次触发比较器状态寄存器

:Depends: `sshot_idx`
:Syntax:
    `cat sshot_status`

    读取状态。
:Example:
    `$> cat sshot_status`

    `0x1`

----

:File:            `sshot_pe_ctrl` (rw)
:Trace Registers: SSPCICR[idx]
:Notes:
    访问单次触发 PE 比较器输入控制寄存器。

:Depends: `sshot_idx`
:Syntax:
    `echo val > sshot_pe_ctrl`

    将 val 写入所选控制寄存器。

----

:File:            `ns_exlevel_vinst` (rw)
:Trace Registers: VICTLR{23:20}
:Notes:
    对安全异常级别过滤器进行编程。设置 / 清除 NS
    异常过滤器位。设置 ‘1’ 将排除该异常级别的跟踪。

:Syntax:
    `echo bitfield > ns_exlevel_viinst`

    其中 bitfield 包含用于设置 / 清除 EL0 到 EL2 的位
:Example:
    `%> echo 0x4 > ns_exlevel_viinst`

    Excludes EL2 NS trace.

----

:File:            `vinst_pe_cmp_start_stop` (rw)
:Trace Registers: VIPCSSCTLR
:Notes:
    访问 PE 启停比较器输入控制寄存器

----

:File:            `bb_ctrl` (rw)
:Trace Registers: BBCTLR
:Notes:
    定义分支广播（Branch Broadcast）所作用的范围。
    默认值 (0x0) 为全部地址。

:Depends: BB enabled.

----

:File:            `cyc_threshold` (rw)
:Trace Registers: CCCTLR
:Notes:
    设置将发出的周期计数阈值。若尝试设置为低于 IDR3 中定义的最小值则报错，
    并按有效位宽度进行掩码。

:Depends: CC enabled.

----

:File:            `syncfreq` (rw)
:Trace Registers: SYNCPR
:Notes:
    设置跟踪同步周期。值为 2 的幂，可为 0（关闭）或 8-20。驱动默认值为 12（每 4096 字节）。

----

:File:            `cntr_idx` (rw)
:Trace Registers: none
:Notes:
    选择要访问的计数器

:Syntax:
    `echo idx > cntr_idx`

    其中 idx < nr_cntr

----

:File:            `cntr_ctrl` (rw)
:Trace Registers: CNTCTLR[idx]
:Notes:
    设置计数器控制值。

:Depends: `cntr_idx`
:Syntax:
    `echo val > cntr_ctrl`

    其中 val 依据 ETMv4 规范。

----

:File:            `cntrldvr` (rw)
:Trace Registers: CNTRLDVR[idx]
:Notes:
    设置计数器重装载值。

:Depends: `cntr_idx`
:Syntax:
    `echo val > cntrldvr`

    其中 val 依据 ETMv4 规范。

----

:File:            `nr_cntr` (ro)
:Trace Registers: From IDR5

:Notes:
    已实现的计数器数量。

----

:File:            `ctxid_idx` (rw)
:Trace Registers: None
:Notes:
    选择要访问的上下文 ID 比较器

:Syntax:
    `echo idx > ctxid_idx`

    其中 idx < numcidc

----

:File:            `ctxid_pid` (rw)
:Trace Registers: CIDCVR[idx]
:Notes:
   设置上下文 ID 比较器值

:Depends: `ctxid_idx`

----

:File: `ctxid_masks` (rw)
:Trace Registers: CIDCCTLR0, CIDCCTLR1, CIDCVR<0-7>
:Notes:
    用于设置 1-8 个上下文 ID 比较器字节掩码的值对。会在 CID
    值寄存器中自动将掩码字节清零。

:Syntax:
    `echo m3m2m1m0 [m7m6m5m4] > ctxid_masks`

    32 位值由掩码字节组成，其中 mN 表示上下文 ID 比较器 N 的
    字节掩码值。

    在上下文 ID 比较器少于 4 个的系统上不需要第二个值

----

:File:            `numcidc` (ro)
:Trace Registers: From IDR4
:Notes:
    上下文 ID 比较器的数量

----

:File:            `vmid_idx` (rw)
:Trace Registers: None
:Notes:
    选择要访问的 VM ID 比较器。

:Syntax:
    `echo idx > vmid_idx`

    其中 idx < numvmidc

----

:File:            `vmid_val` (rw)
:Trace Registers: VMIDCVR[idx]
:Notes:
    设置 VM ID 比较器值

:Depends: `vmid_idx`

----

:File:            `vmid_masks` (rw)
:Trace Registers: VMIDCCTLR0, VMIDCCTLR1, VMIDCVR<0-7>
:Notes:
    用于设置 1-8 个 VM ID 比较器字节掩码的值对。会在 VMID 值寄存器中
    自动将掩码字节清零。

:Syntax:
    `echo m3m2m1m0 [m7m6m5m4] > vmid_masks`

    其中 mN 表示 VMID 比较器 N 的字节掩码值。在 VMID 比较器少于 4 个的系统上不需要第二个值。

----

:File:            `numvmidc` (ro)
:Trace Registers: From IDR4
:Notes:
    VMID 比较器的数量

----

:File:            `res_idx` (rw)
:Trace Registers: None.
:Notes:
    选择要访问的资源选择器控制。必须为 2 或更高，因为选择器 0 和 1 是硬连线的。

:Syntax:
    `echo idx > res_idx`

    其中 2 <= idx < nr_resource x 2

----

:File:            `res_ctrl` (rw)
:Trace Registers: RSCTLR[idx]
:Notes:
    设置资源选择器控制值。取值遵循 ETMv4 规范。

:Depends: `res_idx`
:Syntax:
    `echo val > res_cntr`

    其中 val 依据 ETMv4 规范。

----

:File:            `nr_resource` (ro)
:Trace Registers: From IDR4
:Notes:
    资源选择器对的数量

----

:File:            `event` (rw)
:Trace Registers: EVENTCTRL0R
:Notes:
    设置最多 4 个已实现的事件字段。

:Syntax:
    `echo ev3ev2ev1ev0 > event`

    其中 evN 为一个 8 位事件字段。最多 4 个事件字段组成 32 位输入值。有效字段的数量取决于具体实现，由 IDR0 定义。

----

:File: `event_instren` (rw)
:Trace Registers: EVENTCTRL1R
:Notes:
    选择将事件包插入跟踪流的事件。

:Depends: EVENTCTRL0R
:Syntax:
    `echo bitfield > event_instren`

    其中 bitfield 根据事件字段的数量最多为 4 位。

----

:File:            `event_ts` (rw)
:Trace Registers: TSCTLR
:Notes:
    设置将生成时间戳请求的事件。

:Depends: `TS activated`
:Syntax:
    `echo evfield > event_ts`

    其中 evfield 为一个 8 位事件选择器。

----

:File:            `seq_idx` (rw)
:Trace Registers: None
:Notes:
    序列器事件寄存器选择 - 0 到 2

----

:File:            `seq_state` (rw)
:Trace Registers: SEQSTR
:Notes:
    序列器当前状态 - 0 到 3。

----

:File:            `seq_event` (rw)
:Trace Registers: SEQEVR[idx]
:Notes:
    状态转移事件寄存器

:Depends: `seq_idx`
:Syntax:
    `echo evBevF > seq_event`

    其中 evBevF 是一个由两位事件选择器组成的 16 位值：

    - evB : 向后（back）
    - evF : 向前（forwards）

----

:File:            `seq_reset_event` (rw)
:Trace Registers: SEQRSTEVR
:Notes:
    序列器复位事件

:Syntax:
    `echo evfield > seq_reset_event`

    其中 evfield 为一个 8 位事件选择器。

----

:File:            `nrseqstate` (ro)
:Trace Registers: From IDR5
:Notes:
    序列器状态数量（0 或 4）

----

:File:            `nr_pe_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    PE 比较器输入的数量

----

:File:            `nr_ext_inp` (ro)
:Trace Registers: From IDR5
:Notes:
    外部输入的数量

----

:File:            `nr_ss_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    单次触发控制寄存器的数量

----

**注意：** 在对任意地址比较器进行编程时，驱动会为该比较器打上使用类型的标记 —— 即 RANGE、SINGLE、START、STOP。一旦设置了该标记，则只能使用对其进行编程的同一个 sysfs 文件 / 类型来修改其值。

```

  % echo 0 > addr_idx		; select address comparator 0
  % echo 0x1000 0x5000 0 > addr_range ; set address range on comparators 0, 1.
  % echo 0x2000 > addr_start    ; error as comparator 0 is a range comparator
  % echo 2 > addr_idx		; select address comparator 2
  % echo 0x2000 > addr_start	; this is OK as comparator 2 is unused.
  % echo 0x3000 > addr_stop	; error as comparator 2 set as start address.
  % echo 2 > addr_idx		; select address comparator 3
  % echo 0x3000 > addr_stop	; this is OK

```
要清除所有比较器（以及所有其他硬件）上的编程，使用
```

  % echo 1 > reset



```

### ‘mode’ sysfs 参数。


这是一个位字段选择参数，用于设置 ETM 的总体跟踪模式。下表使用驱动源文件中的宏定义来描述各个位，并给出其所代表特性的说明。许多特性是可选的，因此依赖于硬件的实现。

位分配如下：-

----

**bit (0):**
    ETM_MODE_EXCLUDE

**description:**
    这是设置地址范围时包含 / 排除函数的默认值。置 1 表示排除范围。设置 mode
    参数时，该值会应用到当前索引的地址范围。


**bit (4):**
    ETM_MODE_BB

**description:**
    若硬件支持 [IDR0] 则设置以启用分支广播。该功能的主要用途是在代码于运行时被动态打补丁、仅使用条件分支可能无法重建完整程序流程的情况下。

    目前 Perf 不支持向解码器提供修改后的二进制文件，因此该功能仅用于调试目的或配合第三方工具使用。

    选择此选项将导致生成的跟踪量显著增加——可能存在溢出风险，或覆盖的指令更少。注意，此选项还会覆盖 ETM_MODE_RETURNSTACK <coresight-return-stack> 的任何设置，因此在分支广播范围与返回栈范围重叠的情况下，该范围内将不可用返回栈。


**bit (5):**
    ETMv4_MODE_CYCACC

**description:**
    若支持 [IDR0] 则设置以启用周期精确跟踪。


**bit (6):**
    ETMv4_MODE_CTXID

**description:**
    若硬件支持 [IDR2] 则设置以启用上下文 ID 跟踪。


**bit (7):**
    ETM_MODE_VMID

**description:**
    若支持 [IDR2] 则设置以启用虚拟机 ID 跟踪。


**bit (11):**
    ETMv4_MODE_TIMESTAMP

**description:**
    若支持 [IDR0] 则设置以启用时间戳生成。


**bit (12):**
    ETM_MODE_RETURNSTACK
**description:**
    若支持 [IDR0] 则设置以启用跟踪返回栈。


**bit (13-14):**
    ETM_MODE_QELEM(val)

**description:**
    ‘val’ 决定所启用的 Q 元素支持级别（若由 ETM [IDR0] 实现）。


**bit (19):**
    ETM_MODE_ATB_TRIGGER

**description:**
    若支持 [IDR5] 则设置以在事件控制寄存器 [EVENTCTLR1] 中启用 ATBTRIGGER 位。


**bit (20):**
    ETM_MODE_LPOVERRIDE

**description:**
    若支持 [IDR5] 则设置以在事件控制寄存器 [EVENTCTLR1] 中启用 LPOVERRIDE 位。


**bit (21):**
    ETM_MODE_ISTALL_EN

**description:**
    设置以在停顿控制寄存器 [STALLCTLR] 中启用 ISTALL 位。


**bit (23):**
    ETM_MODE_INSTPRIO

**description:**
    若支持 [IDR0] 则设置以在停顿控制寄存器 [STALLCTLR] 中启用 INSTPRIORITY 位。


**bit (24):**
    ETM_MODE_NOOVERFLOW

**description:**
    若支持 [IDR3] 则设置以在停顿控制寄存器 [STALLCTLR] 中启用 NOOVERFLOW 位。


**bit (25):**
    ETM_MODE_TRACE_RESET

**description:**
    若支持 [IDR3] 则设置以在视图指令控制寄存器 [VICTLR] 中启用 TRCRESET 位。


**bit (26):**
    ETM_MODE_TRACE_ERR

**description:**
    设置以在视图指令控制寄存器 [VICTLR] 中启用 TRCCTRL 位。


**bit (27):**
    ETM_MODE_VIEWINST_STARTSTOP

**description:**
    设置视图指令控制寄存器 [VICTLR] 中 ViewInst 启停逻辑的初始状态值。


**bit (30):**
    ETM_MODE_EXCL_KERN

**description:**
    设置默认跟踪配置以排除内核模式跟踪（参见注 a）。


**bit (31):**
    ETM_MODE_EXCL_USER

**description:**
    设置默认跟踪配置以排除用户空间跟踪（参见注 a）。

----

**注 a)** 启动时，ETM 被编程为使用地址范围比较器 0 跟踪整个地址空间。‘mode’ 位 30 / 31 会修改此设置，在地址范围比较器中为 NS 状态设置用户空间（EL0）或内核空间（EL1）的 EL 排除位。（默认设置排除所有安全 EL 和 NS EL2）

一旦使用了 reset 参数，和/或实现了自定义编程——使用这些位将以相同方式设置地址比较器 0 的 EL 位。

**注 b)** 位 2-3、8-10、15-16、18、22 控制仅与数据跟踪协同工作的特性。由于 ETMv4 在架构上禁止 A-profile 数据跟踪，此处将其省略。可能的用途是内核作为异构系统的一部分支持对 R 或 M profile 基础设施进行控制的情况。

位 17、28-29 未使用。
