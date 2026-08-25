
## 内核HYP 之间的内ABI


本文件记录了 Linux 内核hypervisor 层在Linux 作为 hypervisor 运行（例KVM）时的交互。它不涵盖内核作为客户机（在 Xen、KVM 或任何其hypervisor 之下）运行时hypervisor 的交互，也不涵盖内核作为宿主时任hypervisor 特有的交互
注意：KVM/arm 已从内核中移除。但此处描述API 仍然有效，因为它允许内核在以 HYP 模式启动时进kexec。如有必要，KVM hypervisor 也可以使用它
arm arm64（无 VHE）上，内核并不运行在 hypervisor 模式下，但仍需要与之交互，以便安装或拆除内置的 hypervisor
为了实现这一点，内核必须HYP（arm）或 EL2（arm64）下启动，从而能够在切入 SVC/EL1 之前安装一组桩函数（stub）。这些桩函数可通过 `hvc #0` 指令访问，并且仅作用于单CPU
除非另有说明，任何内hypervisor 都必须实现以下函数（参见 arch/arm{,64}/include/asm/virt.h）：

```

    r0/x0 = HVC_SET_VECTORS
    r1/x1 = vectors

  Set HVBAR/VBAR_EL2 to 'vectors' to enable a hypervisor. 'vectors'
  must be a physical address, and respect the alignment requirements
  of the architecture. Only implemented by the initial stubs, not by
  Linux hypervisors.

```
```

    r0/x0 = HVC_RESET_VECTORS

  Turn HYP/EL2 MMU off, and reset HVBAR/VBAR_EL2 to the initials
  stubs' exception vector value. This effectively disables an existing
  hypervisor.

```
```

    r0/x0 = HVC_SOFT_RESTART
    r1/x1 = restart address
    x2 = x0's value when entering the next payload (arm64)
    x3 = x1's value when entering the next payload (arm64)
    x4 = x2's value when entering the next payload (arm64)

  Mask all exceptions, disable the MMU, clear I+D bits, move the arguments
  into place (arm64 only), and jump to the restart address while at HYP/EL2.
  This hypercall is not expected to return to its caller.

```
```

    x0 = HVC_FINALISE_EL2 (arm64 only)

  Finish configuring EL2 depending on the command-line options,
  including an attempt to upgrade the kernel's exception level from
  EL1 to EL2 by enabling the VHE mode. This is conditioned by the CPU
  supporting VHE, the EL2 MMU being off, and VHE not being disabled by
  any other means (command line option, for example).

```
r0/x0 的任何其他取值会触发 hypervisor 特有的处理，此处不予记录
桩函hypercall 的返回值由 r0/x0 保存，成功时0，出错时HVC_STUB_ERR。桩函数 hypercall 允许破坏任何调用者保存的寄存器（arm64 上为 x0-x18，arm 上为 r0-r3 ip）。因此建议使用函数调用来执行hypercall