## 原子操作控制（ATOMCTL）寄存器


我们有原子操作控制（ATOMCTL）寄存器该寄存器决定了在使用 S32C1I 指令时，与以下各种组合搭配所产生的效果：

     1. 是否带有能够在内存内部执行原子事务（Atomic Transactions）的一致性缓存控制器（Coherent Cache Controller）
     2. 是否带有能够自行执行原子事务的智能内存控制器（Intelligent Memory Controller）
```

      0x28: (WB: Internal, WT: Internal, BY:Exception)

```
FPGA 卡上，我们通常模拟一个能够执RCW 事务的智能内存控制器。对于带有外部内存控制器FPGA 卡，我们在执行缓存（WB）事务时让其在内部完成原子操作，并在非缓存操作中使用内存 RCW
对于没有一致性缓存控制器的系统（MX），我们始终使用内存控制器的 RCW，尽管非 MX 控制器很可能支持内部操作
CUSTOMER-WARNING（客户警告）   几乎所有制程客户都从不支持原子 RCW 内存事务的供应商处购买内存控制器，因此他们很可能希望将该寄存器配置为不使RCW
开发人员可能会发现，在缓存被旁路（bypass）的测试中（例如研究缓存别名问题时），使RCW 的旁路（Bypass）模式会比较方便
```

                             WB     WT      BY
                           5   4 | 3   2 | 1   0

```
=========    ==================      ==================      ===============
  2 Bit
  Field
  Values     WB - 回写(Write Back)    WT - 直写(Write Thru)    BY - 旁路(Bypass)
=========    ==================      ==================      ===============
    0        Exception               Exception               Exception
    1        RCW Transaction         RCW Transaction         RCW Transaction
    2        Internal Operation      Internal Operation      Reserved
    3        Reserved                Reserved                Reserved
=========    ==================      ==================      ===============
