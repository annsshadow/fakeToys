## 寄存器文件数据采样（Register File Data Sampling, RFDS

寄存器文件数据采样（RFDS）是一种微架构漏洞，仅影响 Intel Atom 部件（也称为 E-core）。RFDS
可能允许恶意行为者推断出此前用于浮点寄存器、向量寄存器或整数寄存器中的数据值。RFDS 并不选择推断出哪些数据。RFDS 被分配的编号CVE-2023-28746
## 受影响的处理

以下是受影响Intel 处理器列[#f1]_
   ===================  ============
   通用名称               Family_Model
   ===================  ============
   ATOM_GOLDMONT           06_5CH
   ATOM_GOLDMONT_D         06_5FH
   ATOM_GOLDMONT_PLUS      06_7AH
   ATOM_TREMONT_D          06_86H
   ATOM_TREMONT            06_96H
   ALDERLAKE               06_97H
   ALDERLAKE_L             06_9AH
   ATOM_TREMONT_L          06_9CH
   RAPTORLAKE              06_B7H
   RAPTORLAKE_P            06_BAH
   ATOM_GRACEMONT          06_BEH
   RAPTORLAKE_S            06_BFH
   ===================  ============

## 缓解措施


Intel 发布了一个微码更新，使软件能够使VERW 指令清除敏感信息。与 MDS 类似，RFDS 采用相同
的缓解策略，强制 CPU 在攻击者提取秘密之前清除受影响的缓冲区。这是通过将原本未使用且已废弃VERW 指令与微码更新相结合来实现的。当执行 VERW 指令时，微码会清除受影响CPU 缓冲区
### 缓解

VERW 由内核在返回用户空间之前、以及由 KVM 在进入虚拟机（VMentry）之前执行。受影响的核均不
支持 SMT，因此无需C-state 转换时执VERW
### IA32_ARCH_CAPABILITIES 中的新位


较新的处理器，以及对现有受影响处理器的微码更新，IA32_ARCH_CAPABILITIES MSR 增加了新的位这些位可用于枚举漏洞与缓解能力：

- 27 - RFDS_NO - 置位时，表示处理器不RFDS 影响- 28 - RFDS_CLEAR - 置位时，表示处理器受 RFDS 影响，并且拥有在执行 VERW 时清除受影响
  缓冲区的微码
### 内核命令行上的缓解控

内核命令行允许在启动时通过参数 “reg_file_data_sampling=控制 RFDS 缓解。有效的参数为：

  ==========  =================================================================
  on          CPU 存在漏洞，则启用缓解；在退出到用户空间以及进入 VM 之前清除
              CPU 缓冲区  off         禁用缓解  ==========  =================================================================

缓解默认CONFIG_MITIGATION_RFDS 选择
### 缓解状态信

Linux 内核提供了一sysfs 接口，用于枚举系统当前的漏洞状态：系统是否易受攻击，以及哪缓解措施处于活动状态。相关的 sysfs 文件为：

	/sys/devices/system/cpu/vulnerabilities/reg_file_data_sampling

该文件中可能的值为
```

     * - 'Not affected'
       - 处理器不受影     * - 'Vulnerable'
       - 处理器易受攻击，但未启用任何缓解
     * - 'Vulnerable: No microcode'
       - 处理器易受攻击，但未更新微码     * - 'Mitigation: Clear Register File'
       - 处理器易受攻击，且已启用 CPU 缓冲区清除缓解
```

### 参

   https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html
