
## ARM Virtual Generic Interrupt Controller v2 (VGIC)


鏀寔鐨勮澶囩被鍨嬶細

  - KVM_DEV_TYPE_ARM_VGIC_V2     ARM Generic Interrupt Controller v2.0

鍙兘閫氳繃姝?API 鎴栨棫鐨?KVM_CREATE_IRQCHIP API 瀹炰緥鍖栦竴涓?VGIC 瀹炰緥銆傚垱寤虹殑 VGIC
灏嗗厖褰?VM 鐨勪腑鏂帶鍒跺櫒锛岃姹傝妯℃嫙鐨勭敤鎴风┖闂磋澶囧皢涓柇娉ㄥ叆鍒?VGIC锛岃€屼笉鏄洿鎺ユ敞鍏?
鍒?CPU銆?

甯︽湁纭欢鍏煎鎬ф敮鎸佺殑 GICv3 瀹炵幇鍏佽閫氳繃姝ゆ帴鍙ｅ垱寤轰竴涓鎴锋満 GICv2銆傚叧浜庡垱寤哄鎴锋満
GICv3 璁惧鍜屽鎴锋満 ITS 璁惧鐨勪俊鎭紝璇峰弬闃?arm-vgic-v3.txt銆備笉鍙兘鍦ㄥ悓涓€ VM 涓?
鍚屾椂鍒涘缓 GICv3 鍜?GICv2 璁惧銆?


缁勶細
  KVM_DEV_ARM_VGIC_GRP_ADDR
   灞炴€э細

    KVM_VGIC_V2_ADDR_TYPE_DIST (rw, 64-bit)
      瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿涓?GIC 鍒嗗彂鍣紙distributor锛夊瘎瀛樺櫒鏄犲皠鐨勫熀鍦板潃銆備粎瀵?
      KVM_DEV_TYPE_ARM_VGIC_V2 鏈夋晥銆傝鍦板潃闇€瑕?4K 瀵归綈锛屽尯鍩熻鐩?4 KByte銆?

    KVM_VGIC_V2_ADDR_TYPE_CPU (rw, 64-bit)
      瀹㈡埛鏈虹墿鐞嗗湴鍧€绌洪棿涓?GIC 铏氭嫙 cpu 鎺ュ彛瀵勫瓨鍣ㄦ槧灏勭殑鍩哄湴鍧€銆備粎瀵?
      KVM_DEV_TYPE_ARM_VGIC_V2 鏈夋晥銆傝鍦板潃闇€瑕?4K 瀵归綈锛屽尯鍩熻鐩?8 KByte銆?

  閿欒锛?

    =======  =============================================================
    -E2BIG   Address outside of addressable IPA range
    -EINVAL  Incorrectly aligned address
    -EEXIST  Address already configured
    -ENXIO   The group or attribute is unknown/unsupported for this device
             or hardware support is missing.
    -EFAULT  Invalid user pointer for attr->addr.
    =======  =============================================================

  KVM_DEV_ARM_VGIC_GRP_DIST_REGS
   灞炴€э細
```

      bits:     | 63   ....  40 | 39 ..  32  |  31   ....    0 |
      values:   |    reserved   | vcpu_index |      offset     |

    All distributor regs are (rw, 32-bit)

    The offset is relative to the "Distributor base address" as defined in the
    GICv2 specs.  Getting or setting such a register has the same effect as
    reading or writing the register on the actual hardware from the cpu whose
    index is specified with the vcpu_index field.  Note that most distributor
    fields are not banked, but return the same value regardless of the
    vcpu_index used to access the register.

    GICD_IIDR.Revision is updated when the KVM implementation of an emulated
    GICv2 is changed in a way directly observable by the guest or userspace.
    Userspace should read GICD_IIDR from KVM and write back the read value to
    confirm its expected behavior is aligned with the KVM implementation.
    Userspace should set GICD_IIDR before setting any other registers (both
    KVM_DEV_ARM_VGIC_GRP_DIST_REGS and KVM_DEV_ARM_VGIC_GRP_CPU_REGS) to ensure
    the expected behavior. Unless GICD_IIDR has been set from userspace, writes
    to the interrupt group registers (GICD_IGROUPR) are ignored.

  Errors:

    =======  =====================================================
    -ENXIO   Getting or setting this register is not yet supported
    -EBUSY   One or more VCPUs are running
    -EINVAL  Invalid vcpu_index supplied
    =======  =====================================================

  KVM_DEV_ARM_VGIC_GRP_CPU_REGS
   Attributes:

    The attr field of kvm_device_attr encodes two values::

      bits:     | 63   ....  40 | 39 ..  32  |  31   ....    0 |
      values:   |    reserved   | vcpu_index |      offset     |

    All CPU interface regs are (rw, 32-bit)

    The offset specifies the offset from the "CPU interface base address" as
    defined in the GICv2 specs.  Getting or setting such a register has the
    same effect as reading or writing the register on the actual hardware.

    The Active Priorities Registers APRn are implementation defined, so we set a
    fixed format for our implementation that fits with the model of a "GICv2
    implementation without the security extensions" which we present to the
    guest.  This interface always exposes four register APR[0-3] describing the
    maximum possible 128 preemption levels.  The semantics of the register
    indicate if any interrupts in a given preemption level are in the active
    state by setting the corresponding bit.

    Thus, preemption level X has one or more active interrupts if and only if:

      APRn[X mod 32] == 0b1,  where n = X / 32

    Bits for undefined preemption levels are RAZ/WI.

    Note that this differs from a CPU's view of the APRs on hardware in which
    a GIC without the security extensions expose group 0 and group 1 active
    priorities in separate register groups, whereas we show a combined view
    similar to GICv2's GICH_APR.

    For historical reasons and to provide ABI compatibility with userspace we
    export the GICC_PMR register in the format of the GICH_VMCR.VMPriMask
    field in the lower 5 bits of a word, meaning that userspace must always
    use the lower 5 bits to communicate with the KVM device and must shift the
    value left by 3 places to obtain the actual priority mask level.

  Errors:

    =======  =====================================================
    -ENXIO   Getting or setting this register is not yet supported
    -EBUSY   One or more VCPUs are running
    -EINVAL  Invalid vcpu_index supplied
    =======  =====================================================

  KVM_DEV_ARM_VGIC_GRP_NR_IRQS
   Attributes:

    A value describing the number of interrupts (SGI, PPI and SPI) for
    this GIC instance, ranging from 64 to 1024, in increments of 32.

  Errors:

    =======  =============================================================
    -EINVAL  Value set is out of the expected range
    -EBUSY   Value has already be set, or GIC has already been initialized
             with default values.
    =======  =============================================================

  KVM_DEV_ARM_VGIC_GRP_CTRL
   Attributes:

    KVM_DEV_ARM_VGIC_CTRL_INIT
      request the initialization of the VGIC or ITS, no additional parameter
      in kvm_device_attr.addr.

  Errors:

    =======  =========================================================
    -ENXIO   VGIC not properly configured as required prior to calling
             this attribute
    -ENODEV  no online VCPU
    -ENOMEM  memory shortage when allocating vgic internal data
    =======  =========================================================

```
