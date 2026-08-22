## MDS - 微架构数据采
微架构数据采样（Microarchitectural Data Sampling）是一种硬件漏洞，它允许对 CPU 内部各类缓冲区中可用的数据进行无特权的推测性访问
### 受影响的处理
该漏洞影响范围广泛的 Intel 处理器。以下处理器不受影响
   - 来自 AMD、Centaur 以及其他Intel 厂商的处理器

   - CPU 系列（family 6 的较旧处理器型号

   - 部分 Atom 处理器（Bonnell、Saltwell、Goldmont、GoldmontPlus
   - IA32_ARCH_CAPABILITIES MSR 中设置了 ARCH_CAP_MDS_NO 位的 Intel 处理器
某个处理器是否受影响，可以从 sysfs 中的 MDS 漏洞文件中读出。参mds_sys_info
并非所有处理器都会受到 MDS 所有变体的影响，但对它们的缓解措施都是相同的，因此内核将它们当作单一漏洞来处理
### 相关CVE

以下 CVE 条目MDS 漏洞相关
   ==============  =====  ===================================================
   CVE-2018-12126  MSBDS  Microarchitectural Store Buffer Data Sampling
   CVE-2018-12130  MFBDS  Microarchitectural Fill Buffer Data Sampling
   CVE-2018-12127  MLPDS  Microarchitectural Load Port Data Sampling
   CVE-2019-11091  MDSUM  Microarchitectural Data Sampling Uncacheable Memory
   ==============  =====  ===================================================

### 问题

在执行存储（store）、加载（load）、L1 填充（refill）等操作时，处理器会将数据写入临时的微架构结构（缓冲区）中。作为优化手段，缓冲区中的数据可以被转发给加载操作
在某些条件下（通常是由某个加载操作引起fault/assist），与加载内存地址无关的数据可能会从缓冲区中被推测性地转发。由于加载操作导致了 fault assist，其结果将被丢弃，因此被转发的数据不会导致错误的程序执行或状态改变。但恶意操作可能能够将此推测性数据转发到一个泄露（disclosure）gadget，从而可以通过缓存侧信道（cache side channel）攻击推断出其值
由于缓冲区有可能在超线程（Hyper-Thread）之间共享，因此跨超线程的攻击是可能的
更深入的技术信息可MDS 特定x86 体系结构章节中找到：Documentation/arch/x86/mds.rst <mds>
### 攻击场景

针对 MDS 漏洞的攻击可以由运行在宿主机或客户机上的恶意、无特权的用户空间应用程序发起。恶意的客户机操作系统显然也可以发起攻击
与其他基于推测的漏洞不同，MDS 漏洞不允许攻击者控制内存目标地址。因此，攻击纯粹是基于采样的，但正如 TLBleed 攻击所展示的，样本可以被成功地进行后处理
##### Web 浏览
  目前尚不清楚通过 Web 浏览器发起攻击是否可能。通过 Java-Script 进行利用被认为极不可能，但其他广泛使用的 Web 技术（Webassembly）有可能被滥用
### MDS 系统信息

Linux 内核提供一sysfs 接口，用于枚举系统当前的 MDS 状态：系统是否易受攻击，以及哪些缓解措施处于活动状态。相关的 sysfs 文件是：

/sys/devices/system/cpu/vulnerabilities/mds

该文件中可能的值为
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

```
如果处理器易受攻击，则会在上述信息之后追加以下信息：

    ========================  ============================================
    'SMT vulnerable'          SMT is enabled
    'SMT mitigated'           SMT is enabled and mitigated
    'SMT disabled'            SMT is disabled
    'SMT Host state unknown'  Kernel runs in a VM, Host SMT state unknown
    ========================  ============================================

### 缓解机制

内核会检测受影响 CPU，以及所需微码的存在
如果某个 CPU 受影响且微码可用，则内核默认启用缓解措施。该缓解措施可以在启动时通过内核命令行选项进行控制。参mds_mitigation_control_command_line
##### CPU 缓冲区清
  针对 MDS 的缓解措施会在返回用户空间以及进入客户机时清除受影响CPU 缓冲区
  如果启用SMT，并且该 CPU 只是MSBDS 影响而不受其他任MDS 变体影响，那么它还会在空闲（idle）进入时清除缓冲区，因为其他变体无法防御跨超线程攻击
  对于仅受 MSBDS 影响CPU，用户空间、客户机和空闲切换这几种缓解措施已经足够，SMT 不受影响
##### 虚拟化缓
  宿主到客户机的切换保护取决于 CPU L1TF 漏洞情况
  - CPU L1TF 影响
    如果启用L1D flush 缓解措施，并且可用的微码是最新的，那L1D flush 缓解措施会自动保护客户机切换
    如果禁用L1D flush 缓解措施，则当宿MDS 缓解措施启用时，会显式地调用 MDS 缓解措施
    有关 L1TF 与虚拟化的细节，参见    Documentation/admin-guide/hw-vuln//l1tf.rst <mitigation_control_kvm>
  - CPU 不受 L1TF 影响
    当宿MDS 缓解措施启用时，会在进入客户机之前刷CPU 缓冲区
  宿主到客户机切换所得到MDS 保护矩阵如下
  ============ ===== ============= ============ =================
   L1TF         MDS   VMX-L1FLUSH   Host MDS     MDS-State

   Don't care   No    Don't care    N/A          Not affected

   Yes          Yes   Disabled      Off          Vulnerable

   Yes          Yes   Disabled      Full         Mitigated

   Yes          Yes   Enabled       Don't care   Mitigated

   No           Yes   N/A           Off          Vulnerable

   No           Yes   N/A           Full         Mitigated
  ============ ===== ============= ============ =================

  这仅涵盖宿主到客户机的切换，即防止从宿主泄露到客户机，但并不能保护客户机内部。客户机需要有其自身的保护措施
##### XEON PHI 相关注意事项

  XEON PHI 处理器系列受 MSBDS 影响，在进入空闲状态时可能被跨超线程利用。部XEON PHI 变体允许在用户空间（Ring 3）使MWAIT，这为恶意用户空间打开了一个潜在的攻击向量。该暴露可以通过内核命令行选项 'ring3mwait=disable' 禁用
  XEON PHI 不受其他 MDS 变体影响，并MSBDS 会在 CPU 进入空闲状态之前得到缓解。由XEON PHI 也不L1TF 影响，因此完全保护并不需要禁SMT
##### SMT 控制

  MSBDS 外的所MDS 变体都可能被跨超线程攻击。这意味着在受 MFBDS MLPDS 影响CPU 上，必须禁用 SMT 才能获得完全的保护。这些是大多数受影响CPU；例外是 XEON PHI，参xeon_phi
  禁用 SMT 可能会带来显著的性能影响，但具体影响取决于工作负载的类型
  详见 L1TF 缓解文档中的相关章节：Documentation/admin-guide/hw-vuln/l1tf.rst <smt_control>
### 内核命令行上的缓解控
内核命令行允许在启动时通过 "mds=" 选项控制 MDS 缓解措施。该选项的有效参数为
  ============  =============================================================
  full		If the CPU is vulnerable, enable all available mitigations
		for the MDS vulnerability, CPU buffer clearing on exit to
		userspace and when entering a VM. Idle transitions are
		protected as well if SMT is enabled.

		It does not automatically disable SMT.

  full,nosmt	The same as mds=full, with SMT disabled on vulnerable
		CPUs.  This is the complete mitigation.

  off		Disables MDS mitigations completely.

  ============  =============================================================

未指定该选项等同"mds=full"。对于同时受 TAA（TSX 异步中止，TSX Asynchronous Abort）和 MDS 影响的处理器，仅指定 "mds=off" 而没有同时指"tsx_async_abort=off" 将不起作用，因为这两种漏洞使用的是相同的缓解措施
### 缓解措施选择指南

##### 1. 受信任的用户空间

   如果所有用户空间应用程序都来自受信任的来源，且不执行外部提供的不可信代码，则可以禁用缓解措施
##### 2. 使用受信任客户机的虚拟化

   上述关于受信任用户空间的考量同样适用
##### 3. 使用不可信客户机的虚拟化

   保护取决L1TF 缓解措施的状态。参virt_mechanism
   如果启用MDS 缓解措施并禁用了 SMT，则可以防止客户机到宿主以及客户机到客户机的攻击
### 默认缓解措施

  内核针对受影响处理器的默认缓解措施是
  - 启用 CPU 缓冲区清
  内核默认不强制禁SMT，这导致运行不可信代码时 SMT 系统仍然易受攻击。其理由L1TF 相同。参Documentation/admin-guide/hw-vuln//l1tf.rst <default_mitigations>