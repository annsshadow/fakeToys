
## GDS - Gather Data Sampling（数据聚集采样）


Gather Data Sampling 是一种硬件漏洞，允许对先前存储在向量寄存器中的数据进行
未经特权（unprivileged）的推测性访问。

### 问题


当一条 gather 指令执行从内存的加载时，不同的数据元素被合并到目标向量寄存器
中。然而，当一条被瞬态执行（transiently executed）的 gather 指令遇到故障时，
来自架构或内部向量寄存器的陈旧数据可能会转而瞬态转发到目标向量寄存器。
这将允许恶意攻击者使用典型的侧信道技术（如缓存时序攻击）推断出陈旧数据。
GDS 是一种纯粹基于采样的攻击。

攻击者使用 gather 指令来推断陈旧的向量寄存器数据。受害者不需要做任何特殊
的事情，只需使用向量寄存器即可。受害者不需要使用 gather 指令就会受到
影响。

由于缓冲区在超线程（Hyper-Thread）之间共享，跨超线程的攻击是可能的。

### 攻击场景


在没有缓解措施的情况下，GDS 可以跨几乎所有可能的权限边界推断陈旧数据：

	非 enclave 可以推断 SGX enclave 数据
	用户空间可以推断内核数据
	客户机可以推断来自主机的数据
	客户机可以推断来自其他客户机的数据
	用户可以推断来自其他用户的数据

因此，确保在较低权限的上下文（如客户机）以及运行在 SGX enclave 之外时
缓解措施保持启用是很重要的。

硬件对 SGX 强制实施缓解措施。同样，VMM 应确保不允许客户机禁用 GDS 缓解措施。
如果主机出错并允许了这一点，客户机理论上可以禁用 GDS 缓解措施、发起攻击，
然后再重新启用它。

### 缓解机制


该问题在微码（microcode）中得到缓解。微码定义了以下新的位：

 ================================   ===   ============================
 IA32_ARCH_CAPABILITIES[GDS_CTRL]   R/O   枚举 GDS 漏洞与缓解支持。
 IA32_ARCH_CAPABILITIES[GDS_NO]     R/O   处理器不受影响。
 IA32_MCU_OPT_CTRL[GDS_MITG_DIS]    R/W   禁用缓解措施
                                          默认 0。
 IA32_MCU_OPT_CTRL[GDS_MITG_LOCK]   R/W   锁定 GDS_MITG_DIS=0。对
                                          GDS_MITG_DIS 的写入被忽略。
                                          一旦设置无法清除。
 ================================   ===   ============================

在没有更新微码的系统上，也可以通过禁用 AVX 来缓解 GDS。这可以通过在内核
命令行上设置 gather_data_sampling="force" 或 "clearcpuid=avx" 来完成。

如果使用这些选项，将通过关闭 XSAVE YMM 支持来禁用 AVX 使用。但是，处理器
仍会枚举 AVX 支持。不遵循正确的 AVX 枚举（同时检查 AVX **和** XSAVE YMM
支持）的用户空间将会出错。

### 内核命令行上的缓解控制


可以通过在内核命令行上设置 "gather_data_sampling=off" 或 "mitigations=off"
来禁用缓解措施。两者都不指定则默认启用缓解措施。指定
"gather_data_sampling=force" 将在可用时使用微码缓解，或在受影响但微码尚未
更新以包含缓解措施的系统上禁用 AVX。

### GDS 系统信息


内核通过 sysfs 提供漏洞状态信息。对于 GDS，可以通过下列 sysfs 文件访问：

/sys/devices/system/cpu/vulnerabilities/gather_data_sampling

该文件可能包含的值有：

 ============================== =============================================
 Not affected                   处理器不受影响。
 Vulnerable                     处理器受影响且缓解措施已禁用。
 Vulnerable: No microcode       处理器受影响且微码缺少缓解措施。
 Mitigation: AVX disabled,
 no microcode                   处理器受影响且微码缺少缓解措施。
                                AVX 已作为缓解措施禁用。
 Mitigation: Microcode          处理器受影响且缓解措施已生效。
 Mitigation: Microcode (locked) 处理器受影响且缓解措施已生效且无法禁用。
 Unknown: Dependent on
 hypervisor status              运行在受影响但没有办法知道主机处理器是否
                                已缓解或受影响的虚拟客户机处理器上。
 ============================== =============================================

### GDS 默认缓解


更新的微码将默认启用缓解措施。内核的默认行为是让缓解措施保持启用。
