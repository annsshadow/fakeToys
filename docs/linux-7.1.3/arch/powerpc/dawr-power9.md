## POWER9 上的 DAWR 问题


在较老的 POWER9 处理器上，数据地址观察点寄存器（DAWR）如果指cache inhibited（CI内存，可能会导致 checkstop。目Linux 在配DAWR 时无法区CI 内存，因此在受影响的
系统上，DAWR 被禁用
## 受影响的处理器版

此问题仅存在v2.3 之前的处理器上。版本可通过以下方式查看
```

    processor       : 0
    cpu             : POWER9, altivec supported
    clock           : 3800.000000MHz
    revision        : 2.3 (pvr 004e 1203)

```
在存在此问题的系统上，DAWR 会按如下所述被禁用
## 技术细节：


DAWR 6 种不同的设置方式1) ptrace
2) h_set_mode(DAWR)
3) h_set_dabr()
4) kvmppc_set_one_reg()
5) xmon

对于 ptrace，我们现在通过 PPC_PTRACE_GETHWDBGINFO 调用POWER9 上通告零个断点这会导致 GDB 回退到对观察点的软件模拟（速度较慢）
h_set_mode(DAWR) h_set_dabr() 现在会在 POWER9 主机上向客户机返回错误。当前的 Linux
客户机忽略此错误，因此它们会静默地无法获DAWR
kvmppc_set_one_reg() 会将值存储在 vcpu 中，但不会实际在 POWER9 硬件上设置它。这样做是为不破坏从 POWER8 POWER9 的迁移，代价是在迁移时静默丢DAWR
对于 xmonbd' 命令P9 上会返回错误
## 对用户的后果


对于 POWER9 裸机上的 GDB 观察点（'watch' 命令），GDB 会接受该命令。遗憾的是，由于
没有硬件支持该观察点，GDB 将以软件模拟该观察点，使其运行非常缓慢
对于任何POWER9 主机上启动的客户机也是如此。观察点将失败，GDB 会回退到软件模拟
如果POWER8 主机上启动客户机，GDB 会接受观察点并配置硬件以使用 DAWR。由于可以使硬件模拟，这将以全速运行。遗憾的是，如果该客户机被迁移到 POWER9 主机，观察点将在 POWER9
上丢失。对观察点位置的加载与存储将不会GDB 捕获。观察点会被记住，因此如果客户机迁移POWER8 主机，它将重新开始工作
## 强制启用 DAWR


```

  echo Y > /sys/kernel/debug/powerpc/dawr_enable_dangerous

```
这会即便POWER9 上也启用 DAWR
这是一个危险设置，使用风险自负
某些用户可能不在乎有问题的用户弄崩他们的机器（即单用桌面系统），而确实想DAWR这允许他们强制启DAWR
此标志也可用于禁DAWR 访问。一旦清除该标志，所DAWR 访问应立即被清除，你的机再次免于崩溃风险
用户空间可能会因切换此标志而混乱。如果在获取断点数量（通过 PTRACE_GETHWDBGINFO）与
设置断点之间强制启用/禁用 DAWR，用户空间将获得关于可用资源的不一致视图。对于客户机也类似
要在 KVM 客户机中启用 DAWR，需要在主机***客户机中都强制启DAWR。因此，这在 POWERVM
上无法工作，因为它不允许 HCALL 工作。如hypervisor 不支持写DAWR，向 dawr_enable_dangerous
文件写入 'Y' 将失败
要双重确DAWR 是否工作，请运行此内核自测：

  tools/testing/selftests/powerpc/ptrace/ptrace-hwbreak.c

任何错误/失败/跳过都意味着有问题