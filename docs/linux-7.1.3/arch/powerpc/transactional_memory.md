## 事务内存支持

POWER 内核目前对该特性的支持仅限于支持用户程序使用它。内核本身目前并未使用它
本文档旨在总结 Linux 如何支持该特性，以及你可以从自己的用户程序中期待怎样的行为
## 基本概述

硬件事务内存（Hardware Transactional Memory）在 POWER8 处理器上受支持，是一种支持不同形原子内存访问的特性。提供了若干新指令来界定事务；事务保证要么以原子方式完成，要么回滚并撤销
任何部分更改
```
  begin_move_money:
    tbegin
    beq   abort_handler

    ld    r4, SAVINGS_ACCT(r3)
    ld    r5, CURRENT_ACCT(r3)
    subi  r5, r5, 1
    addi  r4, r4, 1
    std   r4, SAVINGS_ACCT(r3)
    std   r5, CURRENT_ACCT(r3)

    tend

    b     continue

  abort_handler:
    ... test for odd failures ...

    /* Retry the transaction if it failed because it conflicted with
     * someone else: */
    b     begin_move_money

```
'tbegin' 指令表示起点tend' 表示终点。在这两个点之间，处理器处于“事务”（Transactional）状态；
如果没有与系统中其他事务性或非事务性访问的冲突，任何内存引用都会一次性完成。在此例中，如果
没有其他处理器触碰过 SAVINGS_ACCT(r3) CURRENT_ACCT(r3)，事务就会像普通的顺序代码一样完成；
这样就执行了一次从当前账户到储蓄账户的原子转账。即使使用的是普通的 ld/std 指令（注意没lwarx/stwcx），要么 SAVINGS_ACCT(r3) CURRENT_ACCT(r3) ***被更新，要么**被更新
如果在此期间发生了与事务访问位置的冲突，事务将被 CPU 中止。寄存器和内存状态会回滚'tbegin'
时的状态，控制流将'tbegin+4' 继续。第二次会跳转到 abort_handler；abort handler 可以检查失原因并重试
被检查点化的寄存器包括所GPR、FPR、VR/VSR、LR、CCR/CR、CTR、FPCSR 以及一些其他状标志寄存器；
详见 ISA
## 事务中止的原
- 与其他处理器使用的缓存行冲突
- 信号
- 上下文切- 关于会中止事务的所有内容的完整文档，请参见 ISA
## 系统调用

在活跃事务内部发起的系统调用不会被执行，事务将被内核以失败码 TM_CAUSE_SYSCALL |
TM_CAUSE_PERSISTENT 判定为失败（doomed）
在挂起（suspended）的事务内部发起的系统调用会像正常一样被执行，内核不会显式将其判定为失败然而，内核为执行该系统调用所做的事情可能导致事务被硬件判定为失败。系统调用在挂起模式下执行，
因此任何副作用都是持久的，与事务的成功或失败无关。内核不保证哪些系统调用会影响事务的成功
如果系统调用是通过库发起的，在依赖系统调用在活跃事务期间中止时必须小心。库可能会缓存值（可能让人误以为成功），或者在进入内核之前执行导致事务失败的操作（这可能产生不同的失败码）例如 glibc getpid() 和惰性符号解析
## 信号

在事务期间递送信号（同步和异步）会提供第二个线程状态（ucontext/mcontext）来表示第二个事务寄存器状态。信号递送通过 'treclaim' 来捕获两种寄存器状态，因此信号会中止事务。传给信号处理程的常ucontext_t 表示被检查点原始的寄存器状态；该信号看起来像是'tbegin+4' 处发生的
如果 sighandler ucontext 设置uc_link，则已经递送了第二ucontext。为了未来的兼容性，检MSR.TS 字段以确定事务状态——如果是，则 uc->uc_link 中的第二ucontext 表示信号发生活跃的事务寄存器
对于 64 位进程，uc->uc_mcontext.regs->msr 是一个完整的 64 MSR，其 TS 字段显示了事务模式
对于 32 位进程，mcontext MSR 寄存器只32 位；32 位存储在第二ucontext MSR 中，uc->uc_link->uc_mcontext.regs->msr。高字包含事务状TS
然而，基本的信号处理程序不需要感知事务，简单地从处理程序返回就能正确处理：

感知事务的信号处理程序可以从第二ucontext 读取事务性寄存器状态。这对于崩溃处理程序确定例如
导致 SIGSEGV 的指令地址是必要的
```
    void crash_handler(int sig, siginfo_t *si, void *uc)
    {
      ucontext_t *ucp = uc;
      ucontext_t *transactional_ucp = ucp->uc_link;

      if (ucp.link) {
        u64 msr = ucp->uc_mcontext.regs->msr;
        /* May have transactional ucontext! */
  #ifndef __powerpc64__
        msr |= ((u64)transactional_ucp->uc_mcontext.regs->msr) << 32;
  #endif
        if (MSR_TM_ACTIVE(msr)) {
           /* Yes, we crashed during a transaction.  Oops. */
   fprintf(stderr, "Transaction to be restarted at 0x%llx, but "
                           "crashy instruction was at 0x%llx\n",
                           ucp->uc_mcontext.regs->nip,
                           transactional_ucp->uc_mcontext.regs->nip);
        }
      }

      fix_the_problem(ucp->dar);
    }

```
当处于活跃事务中并收到信号时，我们需要小心处理栈。有可能tbegin 之后栈已经向上回退了。这明显的情况是 tbegin 在一个函数内部被调用，并tend 之前返回。在这种情况下，栈是被检查点事务内存状态的一部分。如果我们以非事务方式或在挂起状态下写覆盖它，就会有麻烦，因为如果我遇到 tm abort，程序计数器和栈指针会回tbegin 处，但我们在内存中的栈将不再有效
为了避免这一点，当在活跃事务中接收信号时，我们需要使用来自检查点化状态的栈指针，而不是推状态。这确保信号上下文（tm 挂起方式写入）会被写在回滚所需栈的下方。由treclaim 会中事务，因此在 tbegin 和信号之间写入的任何内存无论如何都会被回滚
对于在非 TM 或挂起模式下接收的信号，我们使用正常/非检查点化的栈指针
sighandler 内部发起并在sighandler 返回到内核时挂起的事务，将被回收并丢弃
## 内核使用的失败原因码

这些<asm/reg.h> 中定义，用于区分内核中止事务的不同原因：

 ====================== ================================
 TM_CAUSE_RESCHED       线程被重新调度 TM_CAUSE_TLBI          软件 TLB 失效 TM_CAUSE_FAC_UNAV      FP/VEC/VSX 不可用陷阱 TM_CAUSE_SYSCALL       来自活跃事务的系统调用 TM_CAUSE_SIGNAL        已递送信号 TM_CAUSE_MISC          当前未使用 TM_CAUSE_ALIGNMENT     对齐错误 TM_CAUSE_EMULATE       触及内存的模拟 ====================== ================================

这些可以由用户程序的 abort handler 作为 TEXASR[0:7] 来检查。如果第 7 位置位，表示错误被视持久的。例TM_CAUSE_ALIGNMENT 是持久的，TM_CAUSE_RESCHED 不是
## GDB

GDB ptrace 目前不能感知 TM。如果在一个事务期间停下来，看起来就像事务刚刚开始（呈现的是
被检查点化的状态）。然后事务无法继续，并会走失败处理程序路线。此外，事务性的第二个寄存器状将不可访问。GDB 目前可以用于使用 TM 的程序，但在事务内部的部分则不行
## POWER9

POWER9 上的 TM 在存储完整寄存器状态方面存在问题。此

```
    commit 4bb3c7a0208fc13ca70598efd109901a7cd45ae7
    Author: Paul Mackerras <paulus@ozlabs.org>
    Date:   Wed Mar 21 21:32:01 2018 +1100
    KVM: PPC: Book3S HV: Work around transactional memory bugs in POWER9

```
为了应对这一点，不同POWER9 芯片以不同方式启TM
POWER9N DD2.01 及更低版本上，TM 被禁用。即 HWCAP2[PPC_FEATURE2_HTM] 未设置
POWER9N DD2.1 上，TM 由固件配置为tm 挂起发生时总是中止事务。因tsuspend 会导致事被中止并回滚。内核异常也会导致事务被中止并回滚，并且异常不会发生。如果用户空间构造了一个启TM 挂起sigcontext，该 sigcontext 将被内核拒绝。此模式通过向用户设HWCAP2[PPC_FEATURE2_HTM_NO_SUSPEND] 来通告。在此模式下 HWCAP2[PPC_FEATURE2_HTM] 未设置
POWER9N DD2.2 及更高版本上，KVM POWERVM 为客户机模拟 TM（如提交 4bb3c7a0208f 所述），因TM 为客户机启用，即 HWCAP2[PPC_FEATURE2_HTM] 为客户机用户空间设置。大量使TM 挂起（tsuspend
或内核挂起）的客户机会导致陷入管理程序（hypervisor），因此会遭受性能下降。主机用户空间的 TM
被禁用，HWCAP2[PPC_FEATURE2_HTM] 未设置。（尽管如果将来我们将模拟带入主机用户空间上下文
切换，我们可能会在某个时候启用它）
POWER9C DD1.2 及更高版本仅通过 POWERVM 提供，因Linux 只作为客户机运行。在这些系统TM POWER9N DD2.2 一样被模拟
POWER8 POWER9 的客户机迁移POWER9N DD2.2 POWER9C DD1.2 上可以工作。由于较早的
POWER9 处理器不支持 TM 模拟，那里不支持POWER8 POWER9 的迁移
## 内核实现

### h/rfid mtmsrd 怪癖

ISA 中所定义，rfid 有一个在早期异常处理中有用的怪癖。当处于用户空间事务中并通过某个异常
进入内核时，MSR 最终会TM=0 TS=01（即 TM 关闭TM 挂起）。通常内核会希望改MSR 中的
位，并会执行一rfid 来做到这一点。在这种情况下，rfid 可能SRR0 TM=0 TS=00（即 TM
关闭且非事务），而结MSR 将保留之前的 TM=0 TS=01（即保持挂起）。这是架构中的一个怪癖，因这通常是从 TS=01 TS=00（即挂起 -> 非事务）的转移，而这是一次非法转移
该怪癖在架构中 rfid 的定义里用以下行描述
  if (MSR 29:31 卢 = 0b010 | SRR1 29:31 卢 = 0b000) then
     MSR 29:31 <- SRR1 29:31

hrfid mtmsrd 有相同的怪癖
Linux 内核在其早期异常处理中使用了这个怪癖