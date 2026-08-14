## Intel(R) TXT 概述


Intel 用于更安全计算的技术 —— Intel(R) 可信执行技术（Intel(R) TXT），定义了一些
平台级的增强，这些增强为构建可信平台提供了基础构件。

Intel TXT 以前以代号 LaGrande Technology（LT）为人所知。

Intel TXT 简介：

- 提供动态可信度量根（DRTM，dynamic root of trust for measurement）
- 在非正常关机情况下的数据保护
- 对启动环境进行度量与验证

Intel TXT 是 vPro(TM) 品牌的一部分，在一些非 vPro 系统上也可用。目前它在基于
Q35、X38、Q45 和 Q43 Express 芯片组的桌面系统（例如 Dell Optiplex 755、HP dc7800
等）以及基于 GM45、PM45 和 GS45 Express 芯片组的移动系统上可用。

更多信息请参阅 http://www.intel.com/technology/security/。该站点还提供了
Intel TXT MLE 开发者手册的链接，该手册已针对新发布的平台进行了更新。

在过去的几年里，Intel TXT 曾在多个活动上被介绍过，其中一些是：

      - LinuxTAG 2008:
          http://www.linuxtag.org/2008/en/conf/events/vp-donnerstag.html

      - TRUST2008:
          http://www.trust-conference.eu/downloads/Keynote-Speakers/
          3_David-Grawrock_The-Front-Door-of-Trusted-Computing.pdf

      - IDF, 上海:
          http://www.prcidf.com.cn/index_en.html

      - IDF 2006、2007
	  （我不确定它们是否/在何处可在线获取）

## 可信启动（Trusted Boot）项目概述


Trusted Boot（tboot）是一个开源的、内核/VMM 之前的模块，它使用 Intel TXT 来
对一个 OS 内核/VMM 进行度量且经过验证的启动。

它托管在 SourceForge 的 http://sourceforge.net/projects/tboot。mercurial 源码
仓库可在 http://www.bughost.org/repos.hg/tboot.hg 获取。

Tboot 目前支持启动 Xen（自 v3.2 起支持 TXT 的开源 VMM/虚拟机监控器），以及现在的
Linux 内核。


## 对 Linux 的价值主张，或“你为什么要关心？”


虽然有许多产品和技术尝试度量或保护运行内核的完整性，但它们都假设内核从一开始就是
“好”的。完整性度量架构（IMA，Integrity Measurement Architecture）和 Linux 完整性
模块（Linux Integrity Module）接口就是这类方案的例子。

要在不使用 Intel TXT 的情况下获得对初始内核的信任，必须使用静态可信根（static root
of trust）。这把信任建立在系统复位时的 BIOS 之上，并要求对从系统复位到内核启动完成
之间所执行的所有代码以及这些代码所使用的数据对象进行度量。对于 Linux 内核而言，这
意味着所有的 BIOS、任何选项 ROM、引导加载程序以及引导配置。在实践中，这是大量的
代码/数据，其中很多会因每次启动而变化（例如更换网卡可能会改变选项 ROM）。在没有
参考哈希的情况下，这些度量的变化很难评估或确认其为良性。这个过程也不提供 DMA 保护、
内存配置/别名检查与锁定、崩溃保护或策略支持。

通过使用 Intel TXT 提供的基于硬件的可信根，许多这类问题可以得到缓解。具体而言：许多
启动前（pre-launch）组件可以从信任链中移除，为所有被启动的组件提供 DMA 保护，执行
大量平台配置检查并锁定取值，为任何数据在非正常关机情况下提供保护，并且支持基于策略
的执行/验证。这提供了比原本可能做到的更稳定的度量，以及对系统配置和初始状态更高的
保证。由于 tboot 项目是开源的，信任链中几乎所有部分的源代码都是可用的（SMM 和 Intel
提供的固件除外）。


## 它是如何工作的？


- Tboot 是一个由引导加载程序作为“内核”（即引导加载程序所执行的二进制文件）启动的
  可执行文件。
- 它执行所有必要的工作，以确定平台是否支持 Intel TXT；如果支持，则执行
  GETSEC[SENTER] 处理器指令，发起动态可信根。

   - 如果 tboot 判定系统不支持 Intel TXT，或者配置不正确（例如 SINIT AC Module
     不正确），它会直接启动内核，而不对任何状态做出改动。
   - Tboot 会把关于其进度的各种信息输出到终端、串口和/或内存日志；输出位置可以用
     一个命令行开关配置。

- GETSEC[SENTER] 指令会把控制权交还给 tboot，然后 tboot 会验证环境的某些方面（例如
  TPM NV 锁、e820 表没有无效条目等）。
- 它会把 APs（应用处理器）从 GETSEC[SENTER] 指令让它们进入的特殊睡眠状态唤醒，并使
  它们进入等待 SIPI（wait-for-SIPI）状态。

   - 由于在 TXT 环境下处理器不会响应 INIT 或 SIPI，因此有必要为 APs 创建一个小的
     VT-x 客户机。当它们在客户机中运行时，只会等待 INIT-SIPI-SIPI 序列，这会引发
     VMEXITs，然后禁用 VT 并跳转到 SIPI 向量。这种方法似乎比在内核的 MP 唤醒序列中
     插入特殊代码更好。

- Tboot 随后应用一个（可选的）用户定义的启动策略来验证内核和 initrd。

   - 该策略根植于 TPM NV，并在 tboot 项目中有描述。tboot 项目还包含用于创建和配置
     该策略的工具代码。
   - 策略完全由用户控制，如果不存在，则会启动任何内核。
   - 策略动作是灵活的，可以包括在失败时停机，或者只是记录它们并继续。

- Tboot 调整引导加载程序提供的 e820 表，以保留自身在内存中的位置，以及保留某些其他
  与 TXT 相关的区域。
- 作为其启动的一部分，tboot 使用 VT-d PMR 对全部 RAM 进行 DMA 保护。因此，内核必须
  以 'intel_iommu=on' 启动，以解除这种全面保护，改用 VT-d 的页级保护。
- Tboot 会用一个关于自身的数据填充一个共享页，并在交出控制权时把它传递给 Linux 内核。

   - 共享页的位置通过 boot_params 结构体以物理地址的形式传递。

- 内核会查找 tboot 共享页地址，如果存在则映射它。
- 作为 TXT 提供的检查/保护之一，它把 VT-d DMAR 复制到一个 DMA 保护的内存区域，并
  校验其正确性。VT-d 代码会检测内核是否由 tboot 启动，并使用这份副本而非 ACPI 表中
  的那份。
- 此时，在关机（S<n>）之前，tboot 和 TXT 都不再参与。
- 为了在 TXT 启动后将系统置入任一睡眠状态，必须先退出 TXT。这是为了防止试图让系统
  崩溃以在重启时获取控制权、并窃取残留在内存中的数据的攻击。

   - 内核会执行所有的睡眠准备工作，并用把平台置入所需睡眠状态所需的 ACPI 数据填充
     共享页。
   - 然后内核通过共享页中指定的向量跳入 tboot。
   - Tboot 会清理环境并禁用 TXT，然后使用内核提供的 ACPI 信息真正把平台置入所需的
     睡眠状态。
   - 对于 S3 的情况，tboot 还会把自己注册为恢复（resume）向量。这是必要的，因为它
     必须在恢复时重新建立被度量的环境。一旦 TXT 环境被恢复，它会恢复 TPM PCR，然后
     把控制权交还给内核的 S3 恢复向量。为了保持跨 S3 的系统完整性，内核向 tboot 提供
     一组内存范围（e820 表中的 RAM 和 RESERVED_KERN，但不包括 BIOS 可能在 S3 转换期间
     改动过的任何内存），tboot 会计算这些范围的 MAC（消息认证码）并用 TPM 进行密封。
     在恢复时，一旦被度量的环境被重新建立，tboot 会重新计算 MAC 并对照密封值进行验证。
     tboot 的策略决定了验证失败时发生什么。注意：带有新 MAC 代码的 tboot c/s 194 支持
     这一点。

这就是 TXT 支持的全部内容。


## 配置系统


此代码适用于 32 位、32 位 PAE 和 64 位（x86_64）内核。

在 BIOS 中，用户必须启用：TPM、TXT、VT-x、VT-d。并非所有 BIOS 都允许单独启用/禁用
这些选项，而且找到它们的界面因 BIOS 而异。

```
        title Linux 2.6.29-tip w/ tboot
          root (hd0,0)
                kernel /tboot.gz logging=serial,vga,memory
                module /vmlinuz-2.6.29-tip intel_iommu=on ro
                       root=LABEL=/ rhgb console=ttyS0,115200 3
                module /initrd-2.6.29-tip.img
                module /Q35_SINIT_17.BIN
```
用于启用 Intel TXT 支持的内核选项位于 Security 顶级菜单下，名为“Enable Intel(R)
Trusted Execution Technology (TXT)”。它被视为试验性（EXPERIMENTAL），并依赖于通用的
x86 支持（以允许内核构建选项的最大灵活性），因为 tboot 代码会检测平台是否真正支持
Intel TXT，从而决定是否执行任何内核代码。

Q35_SINIT_17.BIN 文件就是 Intel TXT 所称的认证代码模块（Authenticated Code Module，
ACM）。它特定于系统中的芯片组，也可以在 Trusted Boot 站点找到。它是一个由 Intel 签名
的（未加密）模块，作为 DRTM 过程的一部分用于验证和配置系统。它之所以被签名，是因为
它在系统中的特权级别高于任何其他微码，其正确运行对于建立 DRTM 至关重要。为系统确定
正确的 SINIT ACM 的过程记录在 SINIT-guide.txt 文件中，该文件位于 tboot 的 SourceForge
站点下的 SINIT ACM 下载处。
