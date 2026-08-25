## 推测返回栈溢出（Speculative Return Stack Overflow，SRSO

这是针对AMD 处理器上发现的推测返回栈溢出（SRSO）漏洞的缓解措施。其机制如今是众所周知的场景：毒化（poisoning）CPU 功能单元 —在这种情况下是分支目标缓冲区（BTB）和返回地址预测器（RAP—然后诱骗提升的特权域（内核）泄漏敏感数据
AMD CPU 使用返回地址预测器（又称返回地址返回栈缓冲区，Return Address Stack/Return Stack Buffer）来预测 RET 指令。在某些情况下，一个非架构（non-architectural）的 CALL 指令（即被预测为 CALL 但实际并CALL 的指令）可以RAP 中创建一个条目，该条目可能被用来预测后续 RET 指令的目标
导致这一点的具体情况因微架构而异，但令人担忧的是，攻击者可以错误地训练（mis-train）CPU BTB 来预测内核空间中的非架构 CALL 指令，并利用它来控制后续内核 RET 的推测目标，从而可能通过推测侧信道（speculative side-channel）导致信息泄露
该问题在 CVE-2023-20569 下被跟踪
### 受影响的处理

AMD Zen，第 1-4 代。即所family 0x17 0x19。较旧的处理器尚未被研究
### 系统信息与选项


首先，要使缓解措施有效，必须加载最新的微码（microcode）
显示 SRSO 缓解状态的 sysfs 文件是：

  /sys/devices/system/cpu/vulnerabilities/spec_rstack_overflow

此文件中可能的值为
 - 'Not affected'（不受影响）
   处理器不易受攻击
- 'Vulnerable'（易受攻击）
   处理器易受攻击且未应用任何缓解措施
 - 'Vulnerable: No microcode'（易受攻击：无微码）
   处理器易受攻击，未应用扩IBPB 功能以解决该漏洞的微码
 - 'Vulnerable: Safe RET, no microcode'（易受攻击：Safe RET，无微码）：

   已应“Safe RET缓解措施（见下文）以保护内核，但未应用扩IBPB 的微码。用户空间任务可能仍然易受攻击
 - 'Vulnerable: Microcode, no safe RET'（易受攻击：微码，无 Safe RET）：

   已应用扩IBPB 功能微码补丁。它不解User->Kernel Guest->Host 转换保护，但它解决了 User->User VM->VM 攻击向量
   注意，User->User 缓解Spectre v2 缓解IBPB 方面的选择方式控制
     - conditional IBPB（条IBPB）：

       每个进程可以选择是否需要在其周围发IBPB，通过 PR_SPEC_DISABLE/_ENABLE 等，参见 [spectre](spectre)

     - strict（严格）
       即始终开—通过在内核命令行上提spectre_v2_user=on

   (spec_rstack_overflow=microcode)

 - 'Mitigation: Safe RET'（缓解：Safe RET）：

   微码/软件组合缓解。它通过解决 User->Kernel Guest->Host 转换保护来补充扩IBPB 微码补丁功能
   默认选择或经spec_rstack_overflow=safe-ret 选择
 - 'Mitigation: IBPB'（缓解：IBPB）：

   与上面的 “safe RET类似的保护，但在特权域交叉（User->Kernel，Guest->Host）时采用 IBPB 屏障
  (spec_rstack_overflow=ibpb)

 - 'Mitigation: IBPB on VMEXIT'（缓解：VMEXIT 上的 IBPB）：

   解决云提供商场景的缓—Guest->Host 转换
   (spec_rstack_overflow=ibpb-vmexit)

 - 'Mitigation: Reduced Speculation'（缓解：减少的推测）
   当选择了上面的 “IBPB on VMEXIT并且 CPU 支持 BpSpecReduce 位时，此缓解会自动启用
   它在具有 SRSO_USER_KERNEL_NO=1 CPUID 位的机器上自动启用。在这种情况下，代码逻辑切换到上面的 =ibpb-vmexit 缓解，因为用内核边界不再受影响，因此不再需“safe RET”
   在启IBPB on VMEXIT 缓解选项后，会检测到 BpSpecReduce 位（所有此类机器上都存在该功能），这实际上会覆IBPB on VMEXIT，因为它的性能影响小得多，并且也处理了 guest->host 攻击向量
要利用该漏洞，攻击者需要：

 - 在机器上获得本地访问权限

 - 突破 kASLR

 - 在运行的内核中找到可用于漏洞利用gadget

 - 根据微架构，可能需要在兄弟线程上创建并固定一个额外的工作负载（在 fam 0x19 上不必要
 - 运行漏洞利用

考虑到每种缓解类型的性能影响，默认的'Mitigation: safe RET'，它应处理大多数攻击向量，包括本地的 User->Kernel 向量
一如既往，建议用户通过定期应用软件更新来保持其系统处于最新状态
默认设置将在需要时重新评估，特别是当出现新的攻击向量时
正如可以推测的，'Mitigation: safe RET' 确实会以一定的性能为代价，具体取决于工作负载。如果你信任你的用户空间并且不想承受性能影响，你总是可以使用 spec_rstack_overflow=off 禁用该缓解措施
类似地，'Mitigation: IBPB' 是另一种完整的缓解类型，在应用了系统所需的微码补丁后使用间接分支预测屏障。此缓解也会带来性能成本
### 缓解：Safe RET


该缓解通过确保所RET 指令都推测到一个受控的位置来工作，类似于在 retpoline 序列中控制推测的方式。为此，__x86_return_thunk 强制 CPU 使用 “safe return序列来误预测每个函数返回
为了确保此缓解的安全性，内核必须确保 safe return 序列本身不受攻击者干扰。在 Zen3 Zen4 中，这是通过在去训练（untraining）函srso_alias_untrain_ret() safe return 函数 srso_alias_safe_ret() 之间创建 BTB 别名来实现的，这会驱逐可能中毒的 BTB 条目，并将该安全的条目用于所有函数返回
在较旧的 Zen1 Zen2 中，这是通过使用类似Retbleed 的重解释（reinterpretation）技术实现的：srso_untrain_ret() srso_safe_ret()
### 检Safe RET 缓解确实有效


如果有人想验SRSO safe RET 缓解在内核上是否工作，可以使用两个性能计数器：

- PMC_0xc8 - 退役的 RET/RET lw 计数
- PMC_0xc9 - 退役的 RET/RET lw 误预测计
并比较在内核模式下正确退役的 RET 数与误预测退役的 RET 数。另一种指定这些事件的方式

```
        # perf list ex_ret_near_ret

        List of pre-defined events (to be used in -e or -M):

        core:
          ex_ret_near_ret
               [Retired Near Returns]
          ex_ret_near_ret_mispred
               [Retired Near Returns Mispredicted]
```
```
        # perf stat -e ex_ret_near_ret:k -e ex_ret_near_ret_mispred:k sleep 10s
```
```
        # perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s
```
应该给出相同的数量。即，每个退役的 RET 
```
        [root@brent: ~/kernel/linux/tools/perf> ./perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s

         Performance counter stats for 'sleep 10s':

                   137,167      cpu/event=0xc8,umask=0/k
                   137,173      cpu/event=0xc9,umask=0/k

              10.004110303 seconds time elapsed

               0.000000000 seconds user
               0.004462000 seconds sys
```
相对于缓解被禁用（spec_rstack_overflow=off）或运作不正常的情况，后者通常显示误预测退RET 的数量远小于退RET 的总数，在

```
       [root@brent: ~/kernel/linux/tools/perf> ./perf stat -e cpu/event=0xc8,umask=0/k -e cpu/event=0xc9,umask=0/k sleep 10s

        Performance counter stats for 'sleep 10s':

                  201,627      cpu/event=0xc8,umask=0/k
                    4,074      cpu/event=0xc9,umask=0/k

             10.003267252 seconds time elapsed

              0.002729000 seconds user
              0.000000000 seconds sys
```
另外，还有一个执行上述操作的 selftest，前往

```
        make srso
        ./srso
```
