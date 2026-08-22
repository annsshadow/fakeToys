## 处理MMIO 陈旧数据漏洞（Processor MMIO Stale Data Vulnerabilities
处理MMIO 陈旧数据漏洞（Processor MMIO Stale Data Vulnerabilities）是一类可能暴露数据的
内存映射 I/O（MMIO）漏洞。暴露数据的操作序列从简单到非常复杂不等。由于大多数漏洞都要攻击者能够访MMIO，许多环境并不会受到影响。在使用虚拟化、并向不受信任的 guest 提供 MMIO
访问的系统环境中，可能需要缓解措施。这些漏洞并非瞬态执行（transient execution）攻击。不过，
这些漏洞可能会把陈旧数据传播到核心填充缓冲区（core fill buffer）中，之后可能被未缓解的
瞬态执行攻击推断出来。针对这些漏洞的缓解措施视平台和用法的不同，包含微码（microcode）更与软件改动的组合。其中部分缓解措施与用于缓解微架构数据采样（MDS）或专用寄存器缓冲区数据采样
（SRBDS）的措施类似
## 数据传播者（Data Propagators
传播者（Propagator）是指会把陈旧数据从一个微架构缓冲区或寄存器复制或移动到另一个的操作处理MMIO 陈旧数据漏洞是指可能把陈旧数据直接读取到架构化的、软件可见的状态中，或从缓冲区
或寄存器中采样到的操作
### 填充缓冲区陈旧数据传播者（FBSDP，Fill Buffer Stale Data Propagator
在某些非一致写（non-coherent write）操作中，陈旧数据可能会从填充缓冲区（FB）传播到 uncore
的非一致部分。填充缓冲区传播本身并不会让陈旧数据在架构上可见。陈旧数据必须被传播到一个会读取或采样的位置
### 边带陈旧数据传播者（SSDP，Sideband Stale Data Propagator
边带陈旧数据传播者（SSDP）仅限于客户端（包括 Intel Xeon E3 服务器）uncore 实现。边响应缓冲区由所有客户端核心共享。对于发往边带目标的非一致读，uncore 逻辑会从事务缓冲区和边带
响应缓冲区返64 字节数据给核心，既包括请求的数据，也包括未被请求的陈旧数据。结果，来自
边带响应和事务缓冲区的陈旧数据现在可能驻留在核心填充缓冲区中
### 主陈旧数据传播者（PSDP，Primary Stale Data Propagator
主陈旧数据传播者（PSDP）仅限于客户端（包括 Intel Xeon E3 服务器）uncore 实现。与边带
响应缓冲区类似，主响应缓冲区由所有客户端核心共享。对于某些处理器，MMIO 主读会返64 字节
数据给核心填充缓冲区，既包括请求的数据，也包括未被请求的陈旧数据。这与边带陈旧数据传播类似
## 漏洞（Vulnerabilities
### 设备寄存器部分写（DRPW，Device Register Partial Write）（CVE-2022-21166
某些端点 MMIO 寄存器对小于寄存器大小的写处理不当。它不会中止写操作，也不会只复制正确字节子集（例如，2 字节写就只复2 字节），而是可能写入比写事务所指定的更多字节到寄存器中在受 FBSDP 影响的处理器上，这可能会暴露出创建该写事务的那个核心的填充缓冲区中的陈旧数据
### 共享缓冲区数据采样（SBDS，Shared Buffers Data Sampling）（CVE-2022-21125
在传播者可能已经把数据uncore 中搬移、并把陈旧数据复制到客户端核心填充缓冲区之后，受 MFBDS
影响的处理器可以从填充缓冲区泄漏数据。该漏洞仅限于客户端（包Intel Xeon E3 服务器）uncore 实现
### 共享缓冲区数据读（SBDR，Shared Buffers Data Read）（CVE-2022-21123
它与共享缓冲区数据采样（SBDS）类似，区别在于数据是直接从架构上软件可见的状态中读取的。该
漏洞仅限于客户端（包Intel Xeon E3 服务器）uncore 实现
## 受影响的处理器（Affected Processors
并非所CPU 都会受到所有变体的影响。例如，大多数面向服务器市场的处理器（不包括 Intel Xeon
E3 处理器）只受设备寄存器部分写（DRPW）影响
以下是受影响Intel 处理器列[#f1]_
   ===================  ============  =========
   Common name          Family_Model  Steppings
   ===================  ============  =========
   HASWELL_X            06_3FH        2,4
   SKYLAKE_L            06_4EH        3
   BROADWELL_X          06_4FH        All
   SKYLAKE_X            06_55H        3,4,6,7,11
   BROADWELL_D          06_56H        3,4,5
   SKYLAKE              06_5EH        3
   ICELAKE_X            06_6AH        4,5,6
   ICELAKE_D            06_6CH        1
   ICELAKE_L            06_7EH        5
   ATOM_TREMONT_D       06_86H        All
   LAKEFIELD            06_8AH        1
   KABYLAKE_L           06_8EH        9 to 12
   ATOM_TREMONT         06_96H        1
   ATOM_TREMONT_L       06_9CH        0
   KABYLAKE             06_9EH        9 to 13
   COMETLAKE            06_A5H        2,3,5
   COMETLAKE_L          06_A6H        0,1
   ROCKETLAKE           06_A7H        1
   ===================  ============  =========

如果某个 CPU 在受影响处理器列表中，但没有受到某个变体的影响，则通过 MSR IA32_ARCH_CAPABILITIES
中的新位来表示。如后面小节所述，对于所有变体，缓解措施大体相同，即通过 VERW 指令来清CPU
填充缓冲区
## MSR 中的新位（New bits in MSRs
较新的处理器以及对现有受影响处理器进行的微码更新，向 IA32_ARCH_CAPABILITIES MSR 添加了新的位这些位可用于枚举处理MMIO 陈旧数据漏洞的特定变体，以及缓解能力
### MSR IA32_ARCH_CAPABILITIES

Bit 13 - SBDR_SSDP_NO - 置位时，处理器不受共享缓冲区数据读（SBDR）漏洞，也不受边带陈	数据传播者（SSDP）的影响Bit 14 - FBSDP_NO - 置位时，处理器不受填充缓冲区陈旧数据传播者（FBSDP）的影响Bit 15 - PSDP_NO - 置位时，处理器不受主陈旧数据传播者（PSDP）的影响Bit 17 - FB_CLEAR - 置位时，VERW 指令将作MD_CLEAR 操作的一部分覆盖 CPU 填充缓冲区的
	值。未枚举 MDS_NO（即MDS 影响）但同时枚举了对 L1D_FLUSH MD_CLEAR 支持的处理器	会隐式地FB_CLEAR 作为MD_CLEAR 支持的一部分进行枚举Bit 18 - FB_CLEAR_CTRL - 处理器支持对 MSR IA32_MCU_OPT_CTRL[FB_CLEAR_DIS] 的读写。在此类
	处理器上，可以设FB_CLEAR_DIS 位，VERW 指令不执FB_CLEAR 动作。并非所有支	FB_CLEAR 的处理器都支FB_CLEAR_CTRL
### MSR IA32_MCU_OPT_CTRL

Bit 3 - FB_CLEAR_DIS - 置位时，VERW 指令不执FB_CLEAR 动作。在系统软件认为有必要时（例如，
当性能更为关键，或不受信任的软件没MMIO 访问权限时），这可用于降FB_CLEAR 带来的性能
影响。注意，FB_CLEAR_DIS 对枚举没有影响（例如，它不会改变 FB_CLEAR MD_CLEAR 的枚举）并且它可能不被所有枚举了 FB_CLEAR 的处理器所支持
## 缓解措施（Mitigation
MDS 类似，处理器 MMIO 陈旧数据漏洞的所有变体都采用相同的缓解策略：在攻击者能够提取机之前，强CPU 清空受影响的缓冲区
这是通过结合使用原本未使用且已废弃的 VERW 指令与微码更新来实现的。当执行 VERW 指令时，
微码会清空受影响CPU 缓冲区
内核通过 x86_clear_cpu_buffers() 执行缓冲区清空
在受 MDS 影响的处理器上，内核已经在内用户空间、虚拟机监控guest 以及 C-state（空闲）
切换时调用了 CPU 缓冲区清空。这类处理器上无需额外的缓解措施
对于不受 MDS TAA 影响的处理器，只有在具有 MMIO 能力的攻击者情况下才需要缓解。因此，
内核/用户空间不需VERW。对于虚拟化场景，VERW 仅需在进入具MMIO 能力guest 时（VMENTER执行
### 缓解点（Mitigation points
##### 返回用户空间（Return to user space
在受 MDS/TAA 影响时，缓解措施MDS 相同；否则不需要缓解
##### C-State 切换（C-State transition
CPU C-state 切换期间的控制寄存器写操作可能把数据从填充缓冲区传播uncore 缓冲区。在
C-state 切换之前执行 VERW，以清空 CPU 填充缓冲区
##### Guest 进入点（Guest entry point
在处理器同时也受 MDS/TAA 影响时，缓解措施MDS 相同；否则，仅对具有 MMIO 能力guest VMENTER 时执VERW。在不被 MDS/TAA 影响的处理器上，没有 MMIO 访问能力guest 无法利用
处理MMIO 陈旧数据漏洞提取机密，因此对此类 guest 没有必要执行 VERW
### 内核命令行上的缓解控制（Mitigation control on the kernel command line
内核命令行允许在启动时通过 "mmio_stale_data=" 选项控制处理MMIO 陈旧数据漏洞的缓解该选项的有效参数为
  ==========  =================================================================
  full        If the CPU is vulnerable, enable mitigation; CPU buffer clearing
              on exit to userspace and when entering a VM. Idle transitions are
              protected as well. It does not automatically disable SMT.
  full,nosmt  Same as full, with SMT disabled on vulnerable CPUs. This is the
              complete mitigation.
  off         Disables mitigation completely.
  ==========  =================================================================

如果 CPU 受影响，且内核命令行没有提供 mmio_stale_data=off，那么内核会选择适当的缓解措施
### 缓解状态信息（Mitigation status information
Linux 内核提供了一sysfs 接口，用于枚举系统当前的漏洞状态：系统是否易受攻击，以及哪缓解措施处于激活状态。相关的 sysfs 文件是：

	/sys/devices/system/cpu/vulnerabilities/mmio_stale_data

该文件可能的取值为
```

     * - 'Not affected'
       - The processor is not vulnerable
     * - 'Vulnerable'
       - The processor is vulnerable, but no mitigation enabled
     * - 'Vulnerable: Clear CPU buffers attempted, no microcode'
       - The processor is vulnerable but microcode is not updated. The
         mitigation is enabled on a best effort basis.

         If the processor is vulnerable but the availability of the microcode
         based mitigation mechanism is not advertised via CPUID, the kernel
         selects a best effort mitigation mode. This mode invokes the mitigation
         instructions without a guarantee that they clear the CPU buffers.

         This is done to address virtualization scenarios where the host has the
         microcode update applied, but the hypervisor is not yet updated to
         expose the CPUID to the guest. If the host has updated microcode the
         protection takes effect; otherwise a few CPU cycles are wasted
         pointlessly.
     * - 'Mitigation: Clear CPU buffers'
       - The processor is vulnerable and the CPU buffer clearing mitigation is
         enabled.
     * - 'Unknown: No mitigations'
       - The processor vulnerability status is unknown because it is
	 out of Servicing period. Mitigation is not attempted.

```
### 定义（Definitions）：

Servicing period（服务期）：利用 Intel 平台更新（IPU）流程或其它类似机制，向 Intel 处理器或
平台提供功能和安全更新的过程
End of Servicing Updates（ESU，服务更新终止）：ESU Intel 不再提供服务（例如通过 IPU 或其类似更新流程）的日期。ESU 日期通常会与季度末对齐
如果处理器易受攻击，则会在上述信息之后附加以下信息：

  ========================  ===========================================
  'SMT vulnerable'          SMT is enabled
  'SMT disabled'            SMT is disabled
  'SMT Host state unknown'  Kernel runs in a VM, Host SMT state unknown
  ========================  ===========================================

### 参考资料（References
   https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html
