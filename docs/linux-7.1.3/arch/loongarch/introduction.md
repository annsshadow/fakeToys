
## LoongArch 简

LoongArch 是一种新RISC 指令集架构（ISA），有点类似MIPS RISC-V。目前有 3 种变体：精简 32 位版本（LA32R）、标32 位版本（LA32S）和 64 位版本（LA64）。LoongArch 中定义了 4 个特权级（PLV）：PLV0~PLV3，从高到低排列。内核运行在 PLV0，而应用程序运行在 PLV3。本文档介绍 LoongArch 的寄存器、基本指令集、虚拟内存以及其他一些主题
## 瀵勫瓨鍣。

LoongArch 的寄存器包括通用寄存器（GPR）、浮点寄存器（FPR）、向量寄存器（VR），以及用于特权模式（PLV0）的控制状态寄存器（CSR）
### 通用寄存器（GPR

LoongArch 32 个通用寄存器（GPR）（`$r0` ~ `$r31`）；LA32 中每个为 32 位宽，在 LA64 中为 64 位宽。`$r0` 被硬连线为零，其他寄存器在架构上没有特殊之处。（`$r1` 除外，它被硬连线BL 指令的链接寄存器。）

内核使用LoongArch 寄存器约定的一个变体，如参考资<loongarch-references> 中的 LoongArch ELF psABI 规范所述：

================= =============== =================== ============
名称              别名            用               跨调用保================= =============== =================== ============
`$r0`           `$zero`       常量            未使`$r1`           `$ra`         返回地址           `$r2`           `$tp`         TLS/线程指针       未使`$r3`           `$sp`         栈指            `$r4`-`$r11`  `$a0`-`$a7` 参数寄存        `$r4`-`$r5`   `$v0`-`$v1` 返回            `$r12`-`$r20` `$t0`-`$t8` 临时寄存        `$r21`          `$u0`         CPU 基址        未使`$r22`          `$fp`         帧指            `$r23`-`$r31` `$s0`-`$s8` 静态寄存器         ================= =============== =================== ============

    寄存`$r21` ELF psABI 中保留，但被 Linux 内核用于存储CPU 基址。它通常没有 ABI 名称，在内核中称`$u0`。你也可能在一些旧代码中看`$v0` `$v1`，但它们分别`$a0` `$a1` 已废弃的别名
### 浮点寄存器（FPR

当存FPU 时，LoongArch 32 个浮点寄存器（FPR）（`$f0` ~ `$f31`）。在 LA64 核心上每个为 64 位宽
浮点寄存器约定与 LoongArch ELF psABI 规范中所述相同：

================= ================== =================== ============
名称              别名              用               跨调用保================= ================== =================== ============
`$f0`-`$f7`   `$fa0`-`$fa7`  参数寄存         `$f0`-`$f1`   `$fv0`-`$fv1`  返回             `$f8`-`$f23`  `$ft0`-`$ft15` 临时寄存         `$f24`-`$f31` `$fs0`-`$fs7`  静态寄存器          ================= ================== =================== ============

    你可能会在一些旧代码中看`$fv0` `$fv1`，但它们分别`$fa0` `$fa1` 已废弃的别名
### 向量寄存器（VR

目前 LoongArch 2 种向量扩展：

- LSX（龙SIMD 扩展，Loongson SIMD eXtension），向量128 位，
- LASX（龙芯高SIMD 扩展，Loongson Advanced SIMD eXtension），向量256 位
LSX 提供 `$v0` ~ `$v31`，LASX 提供 `$x0` ~ `$x31` 作为向量寄存器
VR FPR 重叠：例如，在实LSX LASX 的核心上，`$x0` 的低 128 位与 `$v0` 共享，`$v0` 的低 64 位与 `$f0` 共享；其他所VR 也同理
### 控制状态寄存器（CSR

CSR 只能从特权模式（PLV0）访问：

================= ===================================== ==============
地址              全称                                  缩写================= ===================================== ==============
0x0               当前模式信息                           CRMD
0x1               异常前模式信                        PRMD
0x2               扩展单元使能                           EUEN
0x3               杂项控制                               MISC
0x4               异常配置                               ECFG
0x5               异常状                              ESTAT
0x6               异常返回地址                           ERA
0x7               错误（故障）虚拟地址                   BADV
0x8               错误（故障）指令                    BADI
0xC               异常入口地址                           EENTRY
0x10              TLB 索引                              TLBIDX
0x11              TLB 表项高位                           TLBEHI
0x12              TLB 表项低位 0                         TLBELO0
0x13              TLB 表项低位 1                         TLBELO1
0x18              地址空间标识                        ASID
0x19              下半地址空间的页全局目录地址           PGDL
0x1A              上半地址空间的页全局目录地址           PGDH
0x1B              页全局目录地址                         PGD
0x1C              下半地址空间的页游走控制               PWCL
0x1D              上半地址空间的页游走控制               PWCH
0x1E              STLB 椤靛ぇ灏?                          STLBPS
0x1F              缩减虚拟地址配置                       RVACFG
0x20              CPU 鏍囪瘑绗?                           CPUID
0x21              特权资源配置 1                         PRCFG1
0x22              特权资源配置 2                         PRCFG2
0x23              特权资源配置 3                         PRCFG3
0x30+n (0≤n5)   保存数据寄存                        SAVEn
0x40              定时器标识符                           TID
0x41              瀹氭椂鍣ㄩ厤缃?                            TCFG
0x42              定时器                              TVAL
0x43              定时器计数补                        CNTC
0x44              定时器中断清                        TICLR
0x60              LLBit 控制                            LLBCTL
0x80              实现相关控制 1                         IMPCTL1
0x81              实现相关控制 2                         IMPCTL2
0x88              TLB 重填异常入口地址                   TLBRENTRY
0x89              TLB 重填异常错误（故障）虚拟地址       TLBRBADV
0x8A              TLB 重填异常返回地址                   TLBRERA
0x8B              TLB 重填异常保存数据寄存            TLBRSAVE
0x8C              TLB 重填异常入口低位 0                 TLBRELO0
0x8D              TLB 重填异常入口低位 1                 TLBRELO1
0x8E              TLB 重填异常入口高位                   TLBEHI
0x8F              TLB 重填异常异常前模式信            TLBRPRMD
0x90              机器错误控制                           MERRCTL
0x91              机器错误信息 1                         MERRINFO1
0x92              机器错误信息 2                         MERRINFO2
0x93              机器错误异常入口地址                   MERRENTRY
0x94              机器错误异常返回地址                   MERRERA
0x95              机器错误异常保存数据寄存            MERRSAVE
0x98              缓存 TAG                               CTAG
0x180+n (0≤n)   直接映射配置窗口 n                     DMWn
0x200+2n (0≤n1) 性能监视器配n                       PMCFGn
0x201+2n (0≤n1) 性能监视器总计数器 n                   PMCNTn
0x300             内存加载/存储观察点总体控制            MWPC
0x301             内存加载/存储观察点总体状           MWPS
0x310+8n (0≤n)  内存加载/存储观察n 配置 1           MWPnCFG1
0x311+8n (0≤n)  内存加载/存储观察n 配置 2           MWPnCFG2
0x312+8n (0≤n)  内存加载/存储观察n 配置 3           MWPnCFG3
0x313+8n (0≤n)  内存加载/存储观察n 配置 4           MWPnCFG4
0x380             指令获取观察点总体控制                 FWPC
0x381             指令获取观察点总体状                FWPS
0x390+8n (0≤n)  指令获取观察n 配置 1                FWPnCFG1
0x391+8n (0≤n)  指令获取观察n 配置 2                FWPnCFG2
0x392+8n (0≤n)  指令获取观察n 配置 3                FWPnCFG3
0x393+8n (0≤n)  指令获取观察n 配置 4                FWPnCFG4
0x500             璋冭瘯瀵勫瓨鍣?                            DBG
0x501             调试异常返回地址                       DERA
0x502             调试异常保存数据寄存                DSAVE
================= ===================================== ==============

ERA、TLBRERA、MERRERA DERA 有时也分别称EPC、TLBREPC、MERREPC DEPC
## 基本指令

### 指令格式


LoongArch 指令32 位宽，属9 种基本指令格式（及其变体）：

=========== ==========================
格式     组成
=========== ==========================
2R          Opcode + Rj + Rd
3R          Opcode + Rk + Rj + Rd
4R          Opcode + Ra + Rk + Rj + Rd
2RI8        Opcode + I8 + Rj + Rd
2RI12       Opcode + I12 + Rj + Rd
2RI14       Opcode + I14 + Rj + Rd
2RI16       Opcode + I16 + Rj + Rd
1RI21       Opcode + I21L + Rj + I21H
I26         Opcode + I26L + I26H
=========== ==========================

Rd 是目标寄存器操作数，Rj、Rk Raa" 表示 "additional"，额外的）是源寄存器操作数。I8/I12/I14/I16/I21/I26 是相应宽度的立即数操作数。较长的 I21 I26 在指令字中分别存储在高位和低位部分，"L" "H" 后缀表示
### 指令列表


为简洁起见，此处仅列出指令名称（助记符）；详情请参阅参考资<loongarch-references>
```
    ADD.W SUB.W ADDI.W ADD.D SUB.D ADDI.D
    SLT SLTU SLTI SLTUI
    AND OR NOR XOR ANDN ORN ANDI ORI XORI
    MUL.W MULH.W MULH.WU DIV.W DIV.WU MOD.W MOD.WU
    MUL.D MULH.D MULH.DU DIV.D DIV.DU MOD.D MOD.DU
    PCADDI PCADDU12I PCADDU18I
    LU12I.W LU32I.D LU52I.D ADDU16I.D
```

```
    SLL.W SRL.W SRA.W ROTR.W SLLI.W SRLI.W SRAI.W ROTRI.W
    SLL.D SRL.D SRA.D ROTR.D SLLI.D SRLI.D SRAI.D ROTRI.D
```

```
    EXT.W.B EXT.W.H CLO.W CLO.D SLZ.W CLZ.D CTO.W CTO.D CTZ.W CTZ.D
    BYTEPICK.W BYTEPICK.D BSTRINS.W BSTRINS.D BSTRPICK.W BSTRPICK.D
    REVB.2H REVB.4H REVB.2W REVB.D REVH.2W REVH.D BITREV.4B BITREV.8B BITREV.W BITREV.D
    MASKEQZ MASKNEZ
```

```
    BEQ BNE BLT BGE BLTU BGEU BEQZ BNEZ B BL JIRL
```

```
    LD.B LD.BU LD.H LD.HU LD.W LD.WU LD.D ST.B ST.H ST.W ST.D
    LDX.B LDX.BU LDX.H LDX.HU LDX.W LDX.WU LDX.D STX.B STX.H STX.W STX.D
    LDPTR.W LDPTR.D STPTR.W STPTR.D
    PRELD PRELDX
```

```
    LL.W SC.W LL.D SC.D
    AMSWAP.W AMSWAP.D AMADD.W AMADD.D AMAND.W AMAND.D AMOR.W AMOR.D AMXOR.W AMXOR.D
    AMMAX.W AMMAX.D AMMIN.W AMMIN.D
```

```
    IBAR DBAR
```

```
    SYSCALL BREAK CPUCFG NOP IDLE ERTN(ERET) DBCL(DBGCALL) RDTIMEL.W RDTIMEH.W RDTIME.D
    ASRTLE.D ASRTGT.D
```

```
    CSRRD CSRWR CSRXCHG
    IOCSRRD.B IOCSRRD.H IOCSRRD.W IOCSRRD.D IOCSRWR.B IOCSRWR.H IOCSRWR.W IOCSRWR.D
    CACOP TLBP(TLBSRCH) TLBRD TLBWR TLBFILL TLBCLR TLBFLUSH INVTLB LDDIR LDPTE
```

## 虚拟内存


LoongArch 支持直接映射的虚拟内存和页映射的虚拟内存
直接映射的虚拟内存由 CSR.DMWn（n=0~3）配置，它有一个简单的关系

```
 VA = PA + FixedOffset
```

页映射的虚拟内存VA PA 之间是任意关系，记录TLB 和页表中。LoongArch TLB 包含一个全相联MTLB（多页大TLB）和一个组相联STLB（单页大TLB）
默认情况下，LA32 的整个虚拟地址空间配置如下
============ =========================== =============================
名称         地址范围                    属============ =========================== =============================
`UVRANGE`  `0x00000000 - 0x7FFFFFFF` 页映射，缓存，PLV0~3
`KPRANGE0` `0x80000000 - 0x9FFFFFFF` 直接映射，非缓存，PLV0
`KPRANGE1` `0xA0000000 - 0xBFFFFFFF` 直接映射，缓存，PLV0
`KVRANGE`  `0xC0000000 - 0xFFFFFFFF` 页映射，缓存，PLV0
============ =========================== =============================

用户模式（PLV3）只能访UVRANGE。对于直接映射的 KPRANGE0 KPRANGE1，PA 等于清除bit30~31 VA。例如，0x00001000 的非缓存直接映射 VA 0x80001000，0x00001000 的缓存直接映VA 0xA0001000
默认情况下，LA64 的整个虚拟地址空间配置如下
============ ====================== ======================================
名称         地址范围                属============ ====================== ======================================
`XUVRANGE` ``0x0000000000000000 - 页映射，缓存，PLV0~3
             0x3FFFFFFFFFFFFFFF``
`XSPRANGE` ``0x4000000000000000 - 直接映射，缓非缓存，PLV0
             0x7FFFFFFFFFFFFFFF``
`XKPRANGE` ``0x8000000000000000 - 直接映射，缓非缓存，PLV0
             0xBFFFFFFFFFFFFFFF``
`XKVRANGE` ``0xC000000000000000 - 页映射，缓存，PLV0
             0xFFFFFFFFFFFFFFFF``
============ ====================== ======================================

用户模式（PLV3）只能访XUVRANGE。对于直接映射的 XSPRANGE XKPRANGE，PA 等于清除bit 60~63 VA，缓存属性由 VA 中的 bit 60~61 配置 表示强序非缓存，1 表示一致性缓存，2 表示弱序非缓存
目前我们仅使XKPRANGE 进行直接映射，XSPRANGE 保留
举例说明x00000000_00001000 的强序非缓存直接映射 VA（位XKPRANGE）为 0x80000000_00001000，其一致性缓存直接映VA（位XKPRANGE）为 0x90000000_00001000，其弱序非缓存直接映VA（位XKPRANGE）为 0xA0000000_00001000
## Loongson LoongArch 的关

LoongArch 是一种不同于任何其他现有架构RISC ISA，Loongson 是一个处理器系列。Loongson 包含 3 个系列：Loongson-1 32 位处理器系列，Loongson-2 是低64 位处理器系列，Loongson-3 是高64 位处理器系列。旧Loongson 基于 MIPS，而新Loongson 基于 LoongArch。以 Loongson-3 为例：Loongson-3A1000/3B1500/3A2000/3A3000/3A4000 兼容 MIPS，Loongson-3A5000（及后续修订版）全部基于 LoongArch
## 参考资

龙芯中科官方网站
  http://www.loongson.cn/

龙芯LoongArch 开发者网站（软件与文档）
  http://www.loongnix.cn/

  https://github.com/loongson/

  https://loongson.github.io/LoongArch-Documentation/

LoongArch ISA 文档
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-Vol1-v1.10-CN.pdf （中文）

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-Vol1-v1.10-EN.pdf （英文）

LoongArch ELF psABI 文档
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-ELF-ABI-v2.01-CN.pdf （中文）

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-ELF-ABI-v2.01-EN.pdf （英文）

龙芯LoongArch Linux 内核仓库
  https://git.kernel.org/pub/scm/linux/kernel/git/chenhuacai/linux-loongson.git
