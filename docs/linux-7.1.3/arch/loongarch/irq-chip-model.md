
## LoongArch IRQ 芯片模型（层级结构）


目前，基LoongArch 的处理器（例Loongson-3A5000）只能与 LS7A 芯片组配合使用。LoongArch 计算机中的中断芯片包CPUINTC（CPU 核心中断控制器）、LIOINTC（传I/O 中断控制器）、EIOINTC（扩I/O 中断控制器）、HTVECINTC（Hyper-Transport 向量中断控制器）、PCH-PIC（LS7A 芯片组中的主中断控制器）、PCH-LPC（LS7A 芯片组中LPC 中断控制器）以及 PCH-MSI（MSI 中断控制器）
CPUINTC 是每个核心的控制器（位于 CPU 内），LIOINTC/EIOINTC/HTVECINTC 是每个封装的控制器（位于 CPU 内），PCH-PIC/PCH-LPC/PCH-MSI 是位CPU 之外（即芯片组）的控制器。这些控制器（换言之即 irqchip）以层级方式连接，共有两种层级模型（传统模型和扩展模型）
## 传统 IRQ 模型


在该模型中，IPI（处理器间中断）CPU 本地定时器中断直接送往 CPUINTC，CPU UARTS 中断送往 LIOINTC，而所有其他设备中断则送往 PCH-PIC/PCH-LPC/PCH-MSI，并HTVECINTC 汇总后送往
```
     +-----+     +---------+     +-------+
     | IPI | --> | CPUINTC | <-- | Timer |
     +-----+     +---------+     +-------+
                      ^
                      |
                 +---------+     +-------+
                 | LIOINTC | <-- | UARTs |
                 +---------+     +-------+
                      ^
                      |
                +-----------+
                | HTVECINTC |
                +-----------+
                 ^         ^
                 |         |
           +---------+ +---------+
           | PCH-PIC | | PCH-MSI |
           +---------+ +---------+
             ^     ^           ^
             |     |           |
     +---------+ +---------+ +---------+
     | PCH-LPC | | Devices | | Devices |
     +---------+ +---------+ +---------+
          ^
          |
     +---------+
     | Devices |
     +---------+
```
## 扩展 IRQ 模型


在该模型中，IPI（处理器间中断）CPU 本地定时器中断直接送往 CPUINTC，CPU UARTS 中断送往 LIOINTC，而所有其他设备中断则送往 PCH-PIC/PCH-LPC/PCH-MSI，并EIOINTC 汇总后送往
```
          +-----+     +---------+     +-------+
          | IPI | --> | CPUINTC | <-- | Timer |
          +-----+     +---------+     +-------+
                       ^       ^
                       |       |
                +---------+ +---------+     +-------+
                | EIOINTC | | LIOINTC | <-- | UARTs |
                +---------+ +---------+     +-------+
                 ^       ^
                 |       |
          +---------+ +---------+
          | PCH-PIC | | PCH-MSI |
          +---------+ +---------+
            ^     ^           ^
            |     |           |
    +---------+ +---------+ +---------+
    | PCH-LPC | | Devices | | Devices |
    +---------+ +---------+ +---------+
         ^
         |
    +---------+
    | Devices |
    +---------+
```
## 虚拟扩展 IRQ 模型


在该模型中，IPI（处理器间中断）CPU 本地定时器中断直接送往 CPUINTC，CPU UARTS 中断送往 PCH-PIC，而所有其他设备中断则送往 PCH-PIC/PCH-MSI，并V-EIOINTC（虚```
       +-----+    +-------------------+     +-------+
       | IPI |--> | CPUINTC(0-255vcpu)| <-- | Timer |
       +-----+    +-------------------+     +-------+
                            ^
                            |
                      +-----------+
                      | V-EIOINTC |
                      +-----------+
                       ^         ^
                       |         |
                +---------+ +---------+
                | PCH-PIC | | PCH-MSI |
                +---------+ +---------+
                  ^      ^          ^
                  |      |          |
           +--------+ +---------+ +---------+
           | UARTs  | | Devices | | Devices |
           +--------+ +---------+ +---------+
```
### 说明

V-EIOINTC（虚拟扩I/O 中断控制器）EIOINTC 的扩展，仅在运行KVM hypervisor VM 模式下工作。通过标准 EIOINTC，中断最多可路由4 vCPU，而借助 V-EIOINTC，中断最多可路由256 个虚CPU
在标EIOINTC 中，中断路由设置包含两个部分 位用CPU 选择 位用CPU IP（中断引脚）选择。CPU 选择中包4 位用EIOINTC 节点选择 位用EIOINTC CPU 选择。CPU 选择CPU IP 选择均采用位图方法，因此在一EIOINTC 节点中，中断只能路由CPU0–CPU3 以及 IP0–IP3
借助 V-EIOINTC，可路由到更CPU 以及 CPU IP（中断引脚），V-EIOINTC 新增了两个寄存器
### EXTIOI_VIRT_FEATURES

该寄存器为只读寄存器，指V-EIOINTC 所支持的特性。新增了 EXTIOI_HAS_INT_ENCODE EXTIOI_HAS_CPU_ENCODE 特性
EXTIOI_HAS_INT_ENCODE 属于标准 EIOINTC 的一部分。若其为 1，表CPU 中断引脚选择可采用常规方法而非位图方法，因此中断可路由IP0–IP15
EXTIOI_HAS_CPU_ENCODE V-EIOINTC 的扩展。若其为 1，表CPU 选择可采用常规方法而非位图方法，因此中断可路由CPU0–CPU255
### EXTIOI_VIRT_CONFIG

该寄存器为读写寄存器，为兼容起见，中断路由默认采用与标准 EIOINTC 相同的方法。若将该位设1，则指示硬件使用常规方法而非位图方法
## 高级扩展 IRQ 模型


在该模型中，IPI（处理器间中断）CPU 本地定时器中断直接送往 CPUINTC，CPU UARTS 中断送往 LIOINTC，PCH-MSI 中断送往 AVECINTC，然后直接送往 CPUINTC，而所有其他设备中```
 +-----+     +-----------------------+     +-------+
 | IPI | --> |        CPUINTC        | <-- | Timer |
 +-----+     +-----------------------+     +-------+
              ^          ^          ^
              |          |          |
       +---------+ +----------+ +---------+     +-------+
       | EIOINTC | | AVECINTC | | LIOINTC | <-- | UARTs |
       +---------+ +----------+ +---------+     +-------+
            ^            ^
            |            |
       +---------+  +---------+
       | PCH-PIC |  | PCH-MSI |
       +---------+  +---------+
         ^     ^           ^
         |     |           |
 +---------+ +---------+ +---------+
 | Devices | | PCH-LPC | | Devices |
 +---------+ +---------+ +---------+
                  ^
                  |
             +---------+
             | Devices |
             +---------+
```
## ACPI 相关定义


```
  ACPI_MADT_TYPE_CORE_PIC;
  struct acpi_madt_core_pic;
  enum acpi_madt_core_pic_version;
```
```
  ACPI_MADT_TYPE_LIO_PIC;
  struct acpi_madt_lio_pic;
  enum acpi_madt_lio_pic_version;
```
```
  ACPI_MADT_TYPE_EIO_PIC;
  struct acpi_madt_eio_pic;
  enum acpi_madt_eio_pic_version;
```
```
  ACPI_MADT_TYPE_HT_PIC;
  struct acpi_madt_ht_pic;
  enum acpi_madt_ht_pic_version;
```
```
  ACPI_MADT_TYPE_BIO_PIC;
  struct acpi_madt_bio_pic;
  enum acpi_madt_bio_pic_version;
```
```
  ACPI_MADT_TYPE_MSI_PIC;
  struct acpi_madt_msi_pic;
  enum acpi_madt_msi_pic_version;
```
```
  ACPI_MADT_TYPE_LPC_PIC;
  struct acpi_madt_lpc_pic;
  enum acpi_madt_lpc_pic_version;
```
## 参考资

Loongson-3A5000 文档
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-3A5000-usermanual-1.02-CN.pdf （中文）

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-3A5000-usermanual-1.02-EN.pdf （英文）

Loongson LS7A 芯片组文档：

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-7A1000-usermanual-2.00-CN.pdf （中文）

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-7A1000-usermanual-2.00-EN.pdf （英文）

    - CPUINTC CSR.ECFG/CSR.ESTAT 及其中断控制器，描述于《LoongArch 参考手册第 1 卷》第 7.4 节；
    - LIOINTC 是《Loongson 3A5000 处理器参考手册》第 11.1 节描述的“传I/O 中断”；
    - EIOINTC 是《Loongson 3A5000 处理器参考手册》第 11.2 节描述的“扩I/O 中断”；
    - HTVECINTC 是《Loongson 3A5000 处理器参考手册》第 14.3 节描述的“HyperTransport 中断”；
    - PCH-PIC/PCH-MSI 是《Loongson 7A1000 桥接器用户手册》第 5 节描述的“中断控制器”；
    - PCH-LPC 是《Loongson 7A1000 桥接器用户手册》第 24.3 节描述的“LPC 中断”