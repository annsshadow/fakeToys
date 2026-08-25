
## 通用虚拟机（vm）接

虚拟机“设备”同样接ioctl `KVM_SET_DEVICE_ATTR``KVM_GET_DEVICE_ATTR` `KVM_HAS_DEVICE_ATTR`。该接口使用与其他设相同`struct kvm_device_attr`，但针对的是虚拟机全局的设置与控制
每台虚拟机的分组与属性（如果有）是架构相关的
## 1. 分组：KVM_S390_VM_MEM_CTRL


:Architectures: s390

### 1.1. 属性：KVM_S390_VM_MEM_ENABLE_CMMA


:Parameters: none
:Returns: -EBUSY if a vcpu is already defined, otherwise 0

为虚拟机启用协作式内存管理辅助（Collaborative Memory Management
Assist，CMMA）
### 1.2. 属性：KVM_S390_VM_MEM_CLR_CMMA


:Parameters: none
:Returns: -EINVAL if CMMA was not enabled;
	  0 otherwise

清除所有客户机页的 CMMA 状态，使客户机标记为未使用的页重新变为
已使用，从而可能不会被宿主机回收
### 1.3. 属KVM_S390_VM_MEM_LIMIT_SIZE


:Parameters: in attr->addr the address for the new limit of guest memory
:Returns: -EFAULT if the given address is not accessible;
	  -EINVAL if the virtual machine is of type UCONTROL;
	  -E2BIG if the given guest memory is to big for that machine;
	  -EBUSY if a vcpu is already defined;
	  -ENOMEM if not enough memory is available for a new shadow guest mapping;
	  0 otherwise.

允许用户空间查询实际限制，并为客户机内存的最大大小设置一个新的限制该限制将分别向上取整2048 MB096 GB192 TB，因为此限制由页层级数决定。在没有限制的情况下，我们会将限制设`KVM_S390_NO_MEM_LIMIT`（`U64_MAX`）
## 2. 分组：KVM_S390_VM_CPU_MODEL


:Architectures: s390

### 2.1. 属性：KVM_S390_VM_CPU_MACHINE (r/o)


```
  struct kvm_s390_vm_cpu_machine {
       __u64 cpuid;           # CPUID of host
       __u32 ibc;             # IBC level range offered by host
       __u8  pad[4];
       __u64 fac_mask[256];   # set of cpu facilities enabled by KVM
       __u64 fac_list[256];   # set of cpu facilities offered by host
  }
```

:Parameters: address of buffer to store the machine related cpu data
	     of type struct kvm_s390_vm_cpu_machine*
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -ENOMEM if not enough memory is available to process the ioctl;
	    0 in case of success.

## 2.2. 属性：KVM_S390_VM_CPU_PROCESSOR (r/w)


```
  struct kvm_s390_vm_cpu_processor {
       __u64 cpuid;           # CPUID currently (to be) used by this vcpu
       __u16 ibc;             # IBC level currently (to be) used by this vcpu
       __u8  pad[6];
       __u64 fac_list[256];   # set of cpu facilities currently (to be) used
			      # by this vcpu
  }
```

KVM 不以任何形式强制或限cpu 模型数据。请把通过
`KVM_S390_VM_CPU_MACHINE` 获取的信息作为合理配置设置的参考。由额外设置facility 位触发、但 KVM 未处理的指令拦截，需要在 VM 驱动代码中实现
:Parameters: address of buffer to store/set the processor related cpu
	     data of type struct kvm_s390_vm_cpu_processor*.
:Returns:  -EBUSY in case 1 or more vcpus are already activated (only in write case);
	   -EFAULT if the given address is not accessible from kernel space;
	   -ENOMEM if not enough memory is available to process the ioctl;
	   0 in case of success.


### 2.3. 属性：KVM_S390_VM_CPU_MACHINE_FEAT (r/o)


允许用户空间获取可用cpu 特性。若硬件提供且该特性被 kvm 支持，则视为
可用。理论上，cpu 特性甚至可以完全由 kvm 模拟
```
  struct kvm_s390_vm_cpu_feat {
	__u64 feat[16]; # Bitmap (1 = feature available), MSB 0 bit numbering
  };
```

:Parameters: address of a buffer to load the feature list from.
:Returns:  -EFAULT if the given address is not accessible from kernel space;
	   0 in case of success.

### 2.4. 属性：KVM_S390_VM_CPU_PROCESSOR_FEAT (r/w)


允许用户空间获取或更改某VM 所VCPU 已启用的 cpu 特性。不可用的特无法被启用
详见 `KVM_S390_VM_CPU_MACHINE_FEAT` 中对该参数结构体的描述
:Parameters: address of a buffer to store/load the feature list from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if a cpu feature that is not available is to be enabled;
	    -EBUSY if at least one VCPU has already been defined;
	    0 in case of success.


### 2.5. 属性：KVM_S390_VM_CPU_MACHINE_SUBFUNC (r/o)


允许用户空间获取可用cpu 子函数，不做任何由设IBC 带来的过滤。这子函数通过“查询”或“测试位（test bit）”子函数指示给客户机 VCPU，并cpacf 函数、plo ptff 等使用
只有`KVM_S390_VM_CPU_MACHINE` 包含引入相关指令STFL(E) 位时，子函数才有效。若相关指令通过“查询子函数”指示子函数，则响应块包含在返回结构体中；若相关指令通过“测试位”机制指示子函数，则子函数代码以 MSB 0
位编号方式包含在返回的结构体中
```
  struct kvm_s390_vm_cpu_subfunc {
       u8 plo[32];           # always valid (ESA/390 feature)
       u8 ptff[16];          # valid with TOD-clock steering
       u8 kmac[16];          # valid with Message-Security-Assist
       u8 kmc[16];           # valid with Message-Security-Assist
       u8 km[16];            # valid with Message-Security-Assist
       u8 kimd[16];          # valid with Message-Security-Assist
       u8 klmd[16];          # valid with Message-Security-Assist
       u8 pckmo[16];         # valid with Message-Security-Assist-Extension 3
       u8 kmctr[16];         # valid with Message-Security-Assist-Extension 4
       u8 kmf[16];           # valid with Message-Security-Assist-Extension 4
       u8 kmo[16];           # valid with Message-Security-Assist-Extension 4
       u8 pcc[16];           # valid with Message-Security-Assist-Extension 4
       u8 ppno[16];          # valid with Message-Security-Assist-Extension 5
       u8 kma[16];           # valid with Message-Security-Assist-Extension 8
       u8 kdsa[16];          # valid with Message-Security-Assist-Extension 9
       u8 reserved[1792];    # reserved for future instructions
  };
```

:Parameters: address of a buffer to load the subfunction blocks from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    0 in case of success.

### 2.6. 属性：KVM_S390_VM_CPU_PROCESSOR_SUBFUNC (r/w)


允许用户空间获取或更改要指示给某VM 所VCPU cpu 子函数。仅当内核与
硬件支持就绪时，该属性才可用
内核使用配置好的子函数块来向客户机指示。仅当关联的 STFL(E) 位未被用户空禁用时（即被查询的指令对客户机实际可用），该子函数块才会被使用
只要尚未写入任何数据，读取就会失败。此种情况下将使IBC 来决定可用的
子函数，以保证向后兼容性
详见 `KVM_S390_VM_CPU_MACHINE_SUBFUNC` 中对该参数结构体的描述
:Parameters: address of a buffer to store/load the subfunction blocks from.
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL when reading, if there was no write yet;
	    -EBUSY if at least one VCPU has already been defined;
	    0 in case of success.

## 3. 分组：KVM_S390_VM_TOD


:Architectures: s390

### 3.1. 属性：KVM_S390_VM_TOD_HIGH


允许用户空间设置/获取 TOD 时钟扩展（u8）（已被 `KVM_S390_VM_TOD_EXT` 取代）
:Parameters: address of a buffer in user space to store the data (u8) to
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if setting the TOD clock extension to != 0 is not supported
	    -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

### 3.2. 属性：KVM_S390_VM_TOD_LOW


允许用户空间设置/获取 POP 中定义的 TOD 时钟寄存器第 0-63 位（u64）
:Parameters: address of a buffer in user space to store the data (u64) to
:Returns:    -EFAULT if the given address is not accessible from kernel space
	     -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

### 3.3. 属性：KVM_S390_VM_TOD_EXT


允许用户空间设置/获取 POP 中定义的 TOD 时钟寄存器第 0-63 位（u64）。若客户CPU 模型支持 TOD 时钟扩展（u8），它还允许用户空间获取/设置该扩展；若客户机
CPU 模型不支持，则将其存0 且不允许被设!= 0 的值
:Parameters: address of a buffer in user space to store the data
	     (kvm_s390_vm_tod_clock) to
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    -EINVAL if setting the TOD clock extension to != 0 is not supported
	    -EOPNOTSUPP for a PV guest (TOD managed by the ultravisor)

## 4. 分组：KVM_S390_VM_CRYPTO


:Architectures: s390

### 4.1. 属性：KVM_S390_VM_CRYPTO_ENABLE_AES_KW (w/o)


允许用户空间启用 aes 密钥包装（key wrapping），包括生成一个新的包装密钥
:Parameters: none
:Returns:    0

### 4.2. 属性：KVM_S390_VM_CRYPTO_ENABLE_DEA_KW (w/o)


允许用户空间启用 dea 密钥包装，包括生成一个新的包装密钥
:Parameters: none
:Returns:    0

### 4.3. 属性：KVM_S390_VM_CRYPTO_DISABLE_AES_KW (w/o)


允许用户空间禁用 aes 密钥包装，清除包装密钥
:Parameters: none
:Returns:    0

### 4.4. 属性：KVM_S390_VM_CRYPTO_DISABLE_DEA_KW (w/o)


允许用户空间禁用 dea 密钥包装，清除包装密钥
:Parameters: none
:Returns:    0

## 5. 分组：KVM_S390_VM_MIGRATION


:Architectures: s390

### 5.1. 属性：KVM_S390_VM_MIGRATION_STOP (w/o)


允许用户空间停止迁移模式，PGSTE 迁移需要此模式。在迁移模式未激活时设置
该属性不会有任何效果
:Parameters: none
:Returns:    0

### 5.2. 属性：KVM_S390_VM_MIGRATION_START (w/o)


允许用户空间启动迁移模式，PGSTE 迁移需要此模式。在迁移模式已经激活时设置
该属性不会有任何效果
所有内存槽（memslot）上必须启用脏页跟踪，否则返`-EINVAL`。当任一
内存槽上的脏页跟踪被禁用时，迁移模式会自动停止
:Parameters: none
:Returns:   -ENOMEM if there is not enough free memory to start migration mode;
	    -EINVAL if the state of the VM is invalid (e.g. no memory defined);
	    0 in case of success.

### 5.3. 属性：KVM_S390_VM_MIGRATION_STATUS (r/o)


允许用户空间查询迁移模式的状态
:Parameters: address of a buffer in user space to store the data (u64) to;
	     the data itself is either 0 if migration mode is disabled or 1
	     if it is enabled
:Returns:   -EFAULT if the given address is not accessible from kernel space;
	    0 in case of success.

## 6. 分组：KVM_ARM_VM_SMCCC_CTRL


:Architectures: arm64

### 6.1. 属性：KVM_ARM_VM_SMCCC_FILTER (w/o)


:Parameters: Pointer to a `struct kvm_smccc_filter`

:Returns:

        ======  ===========================================
        EEXIST  Range intersects with a previously inserted
                or reserved range
        EBUSY   A vCPU in the VM has already run
        EINVAL  Invalid filter configuration
        ENOMEM  Failed to allocate memory for the in-kernel
                representation of the SMCCC filter
        ======  ===========================================

```
    enum kvm_smccc_filter_action {
            KVM_SMCCC_FILTER_HANDLE = 0,
            KVM_SMCCC_FILTER_DENY,
            KVM_SMCCC_FILTER_FWD_TO_USER,
    };

    struct kvm_smccc_filter {
            __u32 base;
            __u32 nr_functions;
            __u8 action;
            __u8 pad[15];
    };
```

过滤器定义为一组互不重叠的范围。每个范围定义一个要施加于范围内 SMCCC
调用的动作。用户空间可以通过对该属性连续多次调用来向过滤器中插入多个范围
KVM 的默认配置允许所有已实现SMCCC 调用。因此，用户空间可以稀疏地定义
SMCCC 过滤器，仅需描述那些修改默认行为的范围
`struct kvm_smccc_filter` 表达的范围为
[`base`, `base + nr_functions`)。该范围不允许回绕，即用户空间不能依`base + nr_functions` 溢出
SMCCC 过滤器同时适用于客户机发起SMC HVC 调用。SMCCC 过滤器会拦截SMCCC 调用的内核内模拟，因此其作用早于其他SMCCC 调用交互的接（例hypercall 位图寄存器）
动作
 - `KVM_SMCCC_FILTER_HANDLE`：允许该客户SMCCC 调用在内核内被处理。强   建议用户空间 **不要** 显式描述允许SMCCC 调用范围
 - `KVM_SMCCC_FILTER_DENY`：在内核内拒绝该客户SMCCC 调用并返回给客户机
 - `KVM_SMCCC_FILTER_FWD_TO_USER`：该客户SMCCC 调用被转发到用户空间   退出原因为 `KVM_EXIT_HYPERCALL`
`pad` 字段保留供将来使用，必须0。若该字段非零，KVM 可能返回 `-EINVAL`
KVM 保留了“Arm 架构调用”的函数 ID 范围，并将拒绝为这些范围的任何部分定过滤器：

        =========== ===============
        Start       End (inclusive)
        =========== ===============
        0x8000_0000 0x8000_FFFF
        0xC000_0000 0xC000_FFFF
        =========== ===============
