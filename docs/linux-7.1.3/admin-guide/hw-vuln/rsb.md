
## 与 RSB 相关的缓解措施


   请保持本文档为最新状态，否则您将被“自愿”负责更新它，并将其转换为
   bugs.c 中一个非常长的注释！

自 2018 年以来，已经出现了许多与返回栈缓冲区（Return Stack Buffer，RSB）
自 2018 年以来，已经出现了许多与返回栈
缓冲区（RSB，Return Stack Buffer，有时在 AMD 上
称为返回地址栈（RAS）或返回地址预测器（RAP））相关的 Spectre CVE。
关于这些 CVE 以及如何缓解它们的信息分散在大量特定于微架构的文档中。
关于这些 CVE 及其缓解方法的信息分散在
大量与微体系结构相关的文档中。
它力求尽可能简洁，只关注当前内核的缓解措施：RSB 相关的攻击向量是什么，以及
本文档试图将所有相关信息汇总到
一处，并阐明当前与 RSB 相关的
缓解措施背后的缘由。本文力求尽可能简洁，仅聚焦于
当前内核的缓解措施：有哪些与 RSB 相关的攻击途径，
以及目前分别是如何缓解的？
相反，这基本上是一个被美化的注释，但又太长而无法真正成为注释。因此，当下一个
本文**并非**意在描述 RSB 机制如何运作或漏洞利用
如何工作。有关这两者的更多细节可在下文
的参考文献中找到。
对于每个攻击向量（以及适用的微架构），必须分别考虑它们。
相反，这本质上是一段被美化的注释，但篇幅过长以至于
无法作为真正的注释存在。因此，当下一个 CVE 出现时，内核开发者可以
快速参考本文，以回顾我们实际在做什么
以及为什么这样做。

总体而言，RSB 攻击有两类：RSB 投毒
（Intel 和 AMD）以及 RSB 下溢（仅 Intel）。它们必须分别针对
每种攻击途径（以及适用的微体系结构）
进行考虑。
RSB 投毒是 SpectreRSB [#spectre-rsb]_ 使用的一种技术，攻击者通过它投毒一个 RSB 条目，
----
不平衡的 CALL/RET 时，就可能发生这种情况。
## RSB 投毒（Intel 和 AMD）
- 所有攻击向量都可以潜在地通过在不信任域和信任域之间转换时，使用 RSB 填充序列
  [#intel-rsb-filling]_ [#amd-rsb-filling]_ 冲刷掉任何被投毒的 RSB 条目来缓解。
#### SpectreRSB

```
RSB 投毒是 SpectreRSB [#spectre-rsb]_ 使用的一种技术，攻击者
通过污染 RSB 表项，使受害者的返回指令
推测执行到攻击者控制的地址。当
上下文切换或 VMEXIT 之后存在不平衡的 CALL/RET 时，就可能发生。
```
- 所有攻击途径都可以通过刷新
  时，确保 RSB 被填充或清除：

  - AMD:
	在 Zen 4 及以上，IBPB（或使用的 SBPB [#amd-sbpb]_）会清除 RSB。
	这由 CPUID 中的 IBPB_RET 指示 [#amd-ibpb-rsb]_。

	在 Zen < 4 上，除了 IBPB [#amd-ibpb-no-rsb]_ 之外，还必须
	始终执行 RSB 填充序列 [#amd-rsb-filling]_。这由 X86_BUG_IBPB_NO_RET 指示。

  - Intel:
	IBPB 总是清除 RSB：

- 在上下文切换时，user->user 缓解需要确保
	执行的间接分支的预测目标。此上下文中的间接分支一词包括 near return 指令，
	因此这些预测目标可能来自 RSB。” [#intel-ibpb-rsb]_

  - AMD：
  地址。由于 TASK_SIZE_MAX 保留的用户规范地址空间末尾的页面间隙，连非规范地址也
  无法插入。指令获取时的 SMEP #PF 会阻止内核推测执行用户空间。

  - AMD:
	“最后，被预测为 ‘ret’ 指令的分支，其预测目标来自返回地址预测器（RAP）。
	AMD 建议软件使用 RAP 填充序列（[2] 中的缓解措施 V2-3）和/或管理模式执行保护
	（SMEP），以确保 RAP 中的地址对推测执行是安全的。我们将这些缓解措施统称为
  - Intel：

  - Intel:
	“在带有增强型 IBRS 的处理器上，RSB 覆盖序列可能不足以阻止 near return 的
	预测目标使用在特权较低预测模式下创建的 RSB 条目。软件可以通过启用 SMEP
	（用于从用户模式到管理模式转换）并在 VM 退出期间设置 IA32_SPEC_CTRL.IBRS
	来防止这种情况。” [#intel-smep-rsb]_

- 在 VMEXIT 时，guest->host 攻击由 eIBRS（以及必要时 PBRSB 缓解）缓解：
- 在上下文切换时，user->kernel 攻击由 SMEP 阻止。用户
  - AMD:
	“当启用 Automatic IBRS 时，用于返回地址预测的返回地址栈会在 VMEXIT 时
	被清除。” [#amd-eibrs-vmexit]_

  - Intel:
	“在带有增强型 IBRS 的处理器上，RSB 覆盖序列可能不足以阻止 near return 的
  - AMD：
	（用于从用户模式到管理模式转换）并在 VM 退出期间设置 IA32_SPEC_CTRL.IBRS
	来防止这种情况。带有增强型 IBRS 的处理器仍然支持只在 OS/VMM 中设置 IBRS 的
	使用模式，用于启用了 SMEP 的操作系统。为此，此类处理器将确保一旦设置了 IBRS，
	即使在 VM 退出时未设置 IBRS，guest 行为也无法在 VM 退出后控制 RSB。”
	[#intel-eibrs-vmexit]_

    注意，某些 Intel CPU 容易受到后屏障返回栈缓冲区预测（Post-barrier Return
    Stack Buffer Predictions，PBRSB）[#intel-pbrsb]_ 的影响，其中来自 guest 的最后
  - Intel：
    PBRSB 缓解。

#### AMD RETBleed / SRSO / Branch Type Confusion


在 AMD 上，被投毒的 RSB 条目也可能由 AMD RETBleed 变体 [#retbleed-paper]_ [#amd-btc]_
或推测性返回栈溢出（Speculative Return Stack Overflow，SRSO）[#amd-srso]_
- 在 VMEXIT 时，guest->host 攻击由 eIBRS（以及 PBRSB
RET 的分支来保护自己。

  - AMD：

## RSB 下溢（仅 Intel）


  - Intel：


某些 Intel Skylake 代 CPU 容易受到 RETBleed 的 Intel 变体 [#retbleed-paper]_
（返回栈缓冲区下溢，RSBU [#intel-rsbu]_）的影响。如果在 RSB 缓冲区由于 CALL/RET
不匹配或从深层调用栈返回而为空时执行 RET，分支预测器可能会回退到使用分支目标
缓冲区（Branch Target Buffer，BTB）。如果用户强制发生 BTB 冲突，那么 RET 就可能
推测性地分支到用户控制的地址。

- 注意，RSB 填充并不能完全缓解此问题。如果存在足够多的不平衡 RET，RSB 仍可能
  下溢并回退到使用被投毒的 BTB 条目。

- 在上下文切换时，user->user 下溢攻击由上下文切换时的条件 IBPB [#cond-ibpb]_ 缓解，
  它有效地清除了 BTB：

  - “间接分支预测屏障（IBPB）是一种间接分支控制机制，它建立一个屏障，阻止在屏障
    之前运行的软件控制同一逻辑处理器上在屏障之后执行的间接分支的预测目标。”
    [#intel-ibpb-btb]_
#### AMD RETBleed / SRSO / 分支类型混淆
- 在上下文切换和 VMEXIT 时，user->kernel 和 guest->host 的 RSB 下溢由 IBRS 或
  eIBRS 缓解：
在 AMD 上，被污染的 RSB 表项也可能由 AMD RETBleed
变体 [#retbleed-paper]_ [#amd-btc]_ 或推测性返回栈
溢出 [#amd-srso]_（Inception [#inception-paper]_）造成。内核
通过将内核中每一条 RET 替换为跳转到单个
安全 RET 来保护自身。
  但是，请注意 eIBRS 和 IBRS 并不能缓解同模式（intra-mode）攻击。与下文的 RRSBA
----

## RSB 下溢（仅 Intel）
  结合）来跟踪内核返回，并在 RSB 接近为空时填充它。

#### RSBA Alternate（RSBA，“Intel Retbleed”）


某些 Intel Skylake 代 CPU 容易受到 Intel 变体的
RETBleed [#retbleed-paper]_（返回栈缓冲区下溢
[#intel-rsbu]_）影响。如果在 RSB 缓冲区因
CALL/RET 不匹配或从深层调用栈返回而为空时执行 RET，
分支预测器会回退到使用分支目标缓冲区（BTB）。如果用户
强制制造 BTB 冲突，则 RET 可能会推测性地分支到
用户控制的地址。

- 请注意，填充 RSB 并不能完全缓解此问题。如果
[#intel-bhi]_ 影响时，RSB 下溢可用于同模式 BTI 攻击。这通过在进入内核时清除 BHB
来缓解。

- 在上下文切换时，user->user 下溢攻击由

- “当软件将 retpoline 用作针对 BHI 或同模式 BTI 的缓解措施，且处理器既枚举了
  RRSBA 又枚举了 RRSBA_DIS 控制位时，它应禁用此行为。” [#intel-retpoline-rrsba]_
  - “间接分支预测器屏障（IBPB）是一种间接分支
----

## 参考文献
