
## SRBDS - 特殊寄存器缓冲区数据采样


SRBDS 是一种硬件漏洞，它允许 MDS
Documentation/admin-guide/hw-vuln/mds.rst 技术来推断从特殊寄存器访问返回的值。特殊寄存器
访问是对核外（off core）寄存器的访问。根据 Intel 的评估，具有隐私安全性预期的那些特殊
寄存器读取是 RDRAND、RDSEED 与 SGX EGETKEY。

当使用 RDRAND、RDSEED 与 EGETKEY 指令时，数据通过易受 MDS 攻击的特殊寄存器机制移动到
核心。

### 受影响的处理器


实现了 RDRAND 和/或 RDSEED 的核心型号（桌面、移动、Xeon-E3）可能会受到影响。

如果处理器的 Family_Model 与 stepping 在以下列表中，则受 SRBDS 影响，但以下例外：列出的
处理器在 Intel TSX 可用却未启用时导出 MDS_NO。后一类处理器仅当软件使用 TSX_CTRL_MSR
启用 Intel TSX 时才受影响，否则不受影响。

  =============  ============  ========
  common name    Family_Model  Stepping
  =============  ============  ========
  IvyBridge      06_3AH        All

  Haswell        06_3CH        All
  Haswell_L      06_45H        All
  Haswell_G      06_46H        All

  Broadwell_G    06_47H        All
  Broadwell      06_3DH        All

  Skylake_L      06_4EH        All
  Skylake        06_5EH        All

  Kabylake_L     06_8EH        <= 0xC
  Kabylake       06_9EH        <= 0xD
  =============  ============  ========

### 相关 CVE


以下 CVE 条目与 SRBDS 问题相关：

    ==============  =====  =====================================
    CVE-2020-0543   SRBDS  特殊寄存器缓冲区数据采样
    ==============  =====  =====================================

### 攻击场景


非特权用户可以使用 MDS 技术，提取在另一个核心或兄弟线程上执行的 RDRAND 与 RDSEED 所返回的
值。


### 缓解机制


Intel 将发布微码更新，修改 RDRAND、RDSEED 与 EGETKEY 指令，在秘密特殊寄存器数据被另一个
逻辑处理器访问之前，覆盖共享暂存缓冲区中的秘密特殊寄存器数据。

在执行 RDRAND、RDSEED 或 EGETKEY 指令期间，来自其他逻辑处理器的核外访问将被延迟，直到
特殊寄存器读取完成，并且共享暂存缓冲区中的秘密数据被覆盖。

这对性能有三个影响：

#. RDRAND、RDSEED 或 EGETKEY 指令具有更高的延迟。

#. 在多个逻辑处理器上同时执行 RDRAND 将被串行化，导致 RDRAND 的最大带宽整体下降。

#. 执行 RDRAND、RDSEED 或 EGETKEY 会延迟来自其他逻辑处理器、未命中其核心缓存的内存访问，
   其影响类似于传统的锁定缓存行拆分（locked cache-line-split）访问。

微码更新提供了一种退出机制（RNGDS_MITG_DIS），用于在 Intel Software Guard Extensions
（Intel SGX）外的 enclave 中执行 RDRAND 与 RDSEED 指令时禁用缓解。在使用此退出机制禁用
缓解的逻辑处理器上，RDRAND 与 RDSEED 执行不会花费更长时间，也不会影响兄弟逻辑处理器的
内存访问性能。该退出机制不影响 Intel SGX enclave（包括在 enclave 内执行 RDRAND 或 RDSEED，
以及 EGETKEY 的执行）。

### IA32_MCU_OPT_CTRL MSR 定义


除了针对此问题的缓解措施外，Intel 还新增了一个线程作用域的 IA32_MCU_OPT_CTRL MSR
（地址 0x123）。该 MSR 以及 RNGDS_MITG_DIS（位 0）的存在由
CPUID.(EAX=07H,ECX=0).EDX[SRBDS_CTRL = 9]==1 枚举。该 MSR 通过微码更新引入。

将某个逻辑处理器的 IA32_MCU_OPT_CTRL[^0^]（RNGDS_MITG_DIS）设为 1，会禁用该逻辑处理器上
在 Intel SGX enclave 外执行的 RDRAND 与 RDSEED 的缓解。为某个特定逻辑处理器退出缓解，不会
影响其他逻辑处理器的 RDRAND 与 RDSEED 缓解。

注意，在 Intel SGX enclave 内部，无论 RNGDS_MITG_DS 的值如何，都会应用缓解。

### 内核命令行上的缓解控制


内核命令行允许在引导时通过 "srbds=" 选项控制 SRBDS 缓解。该选项为：

  ============= =============================================================
  off           此选项在受影响的平台上禁用 RDRAND 与 RDSEED 的 SRBDS 缓解。
  ============= =============================================================

### SRBDS 系统信息


Linux 内核通过 sysfs 提供漏洞状态信息。对于 SRBDS，可通过以下 sysfs 文件访问：
/sys/devices/system/cpu/vulnerabilities/srbds

该文件可能包含的值为：

 ============================== =============================================
 Not affected                   处理器不存在漏洞
 Vulnerable                     处理器存在漏洞且缓解已禁用
 Vulnerable: No microcode       处理器存在漏洞且缺少缓解微码
 Mitigation: Microcode          处理器存在漏洞且缓解已生效
 Mitigation: TSX disabled       处理器仅当 TSX 启用时存在漏洞，而本系统启动时
                                以 TSX 禁用方式引导
 Unknown: Dependent on
 hypervisor status              运行在受影响但无法得知宿主机处理器是否已缓解或
                                存在漏洞的虚拟客户机处理器上
 ============================== =============================================

### SRBDS 默认缓解


这一新的微码在执行 RDRAND、RDSEED 时串行化处理器访问，确保共享缓冲区在被释放复用之前
被覆盖。使用 "srbds=off" 内核命令行来禁用 RDRAND 与 RDSEED 的缓解。
