
## ARM 固件伪寄存器接口


KVM 按照客户机的请求处理 hypercall 服务。ARM 规范或 KVM（作为供应商服务）会定期提供新的 hypercall 服务，只要它们从虚拟化的角度来看是有意义的。

这意味着，在两种不同版本的 KVM 上启动的客户机可能观察到两种不同的“固件”修订版本。如果某个客户机绑定到特定版本的 hypercall 服务，或者一次迁移突然向毫无防备的客户机暴露了不同的版本，这可能会导致问题。

为了补救这种情况，KVM 暴露了一组可以使用 GET/SET_ONE_REG 接口操纵的“固件伪寄存器”。这些寄存器可以由用户空间保存/恢复，并根据需要设置为方便的值。

定义了以下寄存器：

- KVM_REG_ARM_PSCI_VERSION:

  KVM 实现了 PSCI（Power State Coordination Interface，电源状态协调接口）规范，以向客户机提供 CPU 开关机、复位和断电等服务。

  - 仅当 vcpu 设置了 KVM_ARM_VCPU_PSCI_0_2 特性（并且因此已经初始化）时才有效
  - 在 GET_ONE_REG 时返回当前 PSCI 版本（默认为 KVM 实现的最高且与 v0.2 兼容的 PSCI 版本）
  - 允许使用 SET_ONE_REG 设置任何 KVM 实现且与 v0.2 兼容的 PSCI 版本
  - 影响整个 VM（即使寄存器视图是按 vcpu 的）

- KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1:
    保存固件支持的状态，用于缓解 CVE-2017-5715，正如 KVM 通过 HVC 调用向客户机提供的那样。该缓解方法在 [^1^] 的 SMCCC_ARCH_WORKAROUND_1 下描述。

  可接受的值为：

    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_AVAIL:
      KVM 不提供
      该缓解方法的固件支持。对客户机的缓解状态未知。
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_AVAIL:
      该缓解方法 HVC 调用对
      客户机可用，且是缓解所必需的。
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_1_NOT_REQUIRED:
      该缓解方法 HVC 调用对
      客户机可用，但在此 VCPU 上不需要。

- KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2:
    保存固件支持的状态，用于缓解 CVE-2018-3639，正如 KVM 通过 HVC 调用向客户机提供的那样。该缓解方法在 [^1^]_ 的 SMCCC_ARCH_WORKAROUND_2 下描述。

  可接受的值为：

    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_AVAIL:
      缓解方法不
      可用。KVM 不提供该缓解方法的固件支持。
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_UNKNOWN:
      缓解方法状态
      未知。KVM 不提供该缓解方法的固件支持。
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_AVAIL:
      缓解方法可用，
      并且可以被 vCPU 禁用。如果设置了
      KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_ENABLED，则它对该 vCPU 处于活动状态。
    KVM_REG_ARM_SMCCC_ARCH_WORKAROUND_2_NOT_REQUIRED:
      该缓解方法始终在该 vCPU 上处于活动状态，或者不需要它。

### 位图特性固件寄存器


与上述寄存器相反，以下寄存器以特性位图的形式向用户空间暴露 hypercall 服务。该位图被转换为对客户机可用的服务。为每个服务调用所有者定义了一个寄存器，并可通过 GET/SET_ONE_REG 接口访问。

默认情况下，这些寄存器被设置为所支持特性的上限。这样用户空间就可以通过 GET_ONE_REG 发现所有可用的 hypercall 服务。用户空间可以通过 SET_ONE_REG 将期望的位图写回。未被触及的寄存器（可能是因为用户空间不知道它们）的特性将按原样暴露给客户机。

请注意，一旦任何 vCPU 至少运行过一次，KVM 将不再允许用户空间配置这些寄存器。相反，它会返回 -EBUSY。

伪固件位图寄存器如下：

- KVM_REG_ARM_STD_BMAP:
    控制 ARM 标准安全服务调用的位图。

  接受以下位：

    Bit-0: KVM_REG_ARM_STD_BIT_TRNG_V1_0:
      该位代表 ARM True Random Number Generator（TRNG，真随机数生成器）规范 v1.0（ARM DEN0098）下提供的服务。

- KVM_REG_ARM_STD_HYP_BMAP:
    控制 ARM 标准 Hypervisor 服务调用的位图。

  接受以下位：

    Bit-0: KVM_REG_ARM_STD_HYP_BIT_PV_TIME:
      该位代表由 ARM DEN0057A 表示的半虚拟化时间（Paravirtualized Time）服务。

- KVM_REG_ARM_VENDOR_HYP_BMAP:
    控制供应商特定的 Hypervisor 服务调用 [0-63] 的位图。

  接受以下位：

    Bit-0: KVM_REG_ARM_VENDOR_HYP_BIT_FUNC_FEAT
      该位代表 ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID
      和 ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID 函数 id。

    Bit-1: KVM_REG_ARM_VENDOR_HYP_BIT_PTP:
      该位代表精确时间协议（Precision Time Protocol）KVM 服务。

- KVM_REG_ARM_VENDOR_HYP_BMAP_2:
    控制供应商特定的 Hypervisor 服务调用 [64-127] 的位图。

  接受以下位：

    Bit-0: KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_VER
      这代表 ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_VER_FUNC_ID
      函数 id。此位被复位为 0。

    Bit-1: KVM_REG_ARM_VENDOR_HYP_BIT_DISCOVER_IMPL_CPUS
      这代表 ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_CPUS_FUNC_ID
      函数 id。此位被复位为 0。

错误：

    =======  =============================================================
    -ENOENT   访问了未知寄存器。
    -EBUSY    在 VM 启动后尝试对寄存器进行“写”操作。
    -EINVAL   写入了无效的位图到寄存器。
    =======  =============================================================
