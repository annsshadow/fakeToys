## RISC-V 内核启动要求与约束


:Author: Alexandre Ghiti <alexghiti@rivosinc.com>
:Date: 23 May 2023

本文档描述了 RISC-V 内核对引导加载程序和固件的要求，以及任何开发者在改动
早期启动流程时必须牢记的约束。就本文档而言，“早期启动流程（early boot process）”
指任何在最终虚拟映射建立之前运行的代码。

## 内核启动前的要求与约束


RISC-V 内核对引导加载程序和平台固件有以下要求：

### 寄存器状态


RISC-V 内核要求：

  - `$a0` 包含当前核心的 hartid。
  - `$a1` 包含内存中设备树（devicetree）的地址。

### CSR 状态


RISC-V 内核要求：

  - `$satp = 0`：MMU（若存在）必须处于禁用状态。

### 常驻固件的保留内存


RISC-V 内核不得在内核直接映射中映射任何常驻内存，或通过 PMP 保护的内存，因此
固件必须依据设备树规范和/或 UEFI 规范，正确地将这些区域标记出来。

### 内核位置


RISC-V 内核要求被放置在 PMD 边界上（rv64 按 2MB 对齐，rv32 按 4MB 对齐）。
注意，若非如此，EFI stub 会对内核进行物理重定位。

### 硬件描述


固件可以向 RISC-V 内核传递设备树或 ACPI 表。

设备树既可以由上一阶段直接使用 `$a1` 寄存器传递给内核，也可以在使用 UEFI
启动时，通过 EFI 配置表传递。

ACPI 表通过 EFI 配置表传递给内核。在这种情况下，EFI stub 仍会创建一个极简的
设备树。有关此设备树的详细信息，请参阅下文的“EFI stub 与设备树”一节。

### 内核入口


在 SMP 系统上，进入内核有两种方法：

- `RISCV_BOOT_SPINWAIT`：固件将内核中的所有 hart 全部释放，其中一个 hart
  通过抽签胜出并执行早期启动代码，而其他 hart 则被停放，等待初始化完成。
  该方法主要用于支持没有 SBI HSM 扩展以及 M 模式 RISC-V 内核的旧固件。
- `Ordered booting`（有序启动）：固件只释放一个 hart，由它执行初始化阶段，
  然后再利用 SBI HSM 扩展启动所有其他 hart。有序启动是启动 RISC-V 内核的
  首选方式，因为它支持 CPU 热插拔和 kexec。

### UEFI


#### UEFI 内存映射


使用 UEFI 启动时，RISC-V 内核将仅使用 EFI 内存映射来填充系统内存。

UEFI 固件必须解析设备树 `/reserved-memory` 节点的子节点，并遵循设备树规范，
将这些子节点的属性（`no-map` 和 `reusable`）转换为正确的 EFI 等价形式
（参见设备树规范 v0.4-rc1 的“3.5.4 /reserved-memory 与 UEFI”一节）。

#### RISCV_EFI_BOOT_PROTOCOL


使用 UEFI 启动时，EFI stub 需要 boot hartid，以便通过 `$a1` 将其传递给
RISC-V 内核。EFI stub 通过以下方法之一获取 boot hartid：

- `RISCV_EFI_BOOT_PROTOCOL`（**首选**）。
- `boot-hartid` 设备树子节点（**已弃用**）。

任何新固件都必须实现 `RISCV_EFI_BOOT_PROTOCOL`，因为基于设备树的方式现已
弃用。

## 早期启动的要求与约束


RISC-V 内核的早期启动流程须遵循以下约束：

### EFI stub 与设备树


使用 UEFI 启动时，设备树由 EFI stub 补充（或创建），其参数与 arm64 相同，
相关描述见 Documentation/arch/arm/uefi.rst 中“UEFI kernel support on ARM”
（ARM 上的 UEFI 内核支持）一节。

### 虚拟映射的建立


RISC-V 内核分两个步骤建立虚拟映射：

1. `setup_vm()` 在 `early_pg_dir` 中安装一个临时内核映射，以便发现系统内存。
   此时仅映射内核文本/数据。建立该映射时无法分配内存（因为系统内存尚未可知），
   因此 `early_pg_dir` 页表是静态分配的（每个级别仅使用一张表）。

2. `setup_vm_final()` 在 `swapper_pg_dir` 中创建最终的内核映射，并利用已发现的
   系统内存来建立线性映射。建立该映射时，内核可以分配内存，但无法直接访问
   （因为直接映射尚不存在），因此它在 fixmap 区域中使用临时映射来访问新分配的
   页表层级。

为了让 `virt_to_phys()` 和 `phys_to_virt()` 能够正确将直接映射地址转换为物理
地址，它们需要知道 DRAM 的起始地址。这发生在第 1 步之后、第 2 步建立直接映射
之前（参见 arch/riscv/mm/init.c 中的 `setup_bootmem()` 函数）。在最终虚拟映射
建立之前使用这些宏，必须仔细审视。

### 通过 fixmap 映射设备树


由于 `reserved_mem` 数组使用 `setup_vm()` 建立的虚拟地址进行初始化，并配合
`setup_vm_final()` 建立的映射使用，RISC-V 内核利用 fixmap 区域来映射设备树。
这确保了设备树对两种虚拟映射都保持可访问。

### MMU 之前的执行


有少量代码需要在第一个虚拟映射建立之前运行。这包括第一个虚拟映射本身的安装、
早期 alternatives 的补丁应用，以及内核命令行参数的早期解析。这些代码在编译时
必须非常小心：

- `-fno-pie`：对于使用 `-fPIE` 的可重定位内核，这是必需的，否则任何对全局
  符号的访问都会经由 GOT，而 GOT 仅在虚拟重定位后才生效。
- `-mcmodel=medany`：任何对全局符号的访问都必须是 PC 相对的，以避免在 MMU
  建立之前发生重定位。
- **所有**插桩（instrumentation）也必须被禁用（包括 KASAN、ftrace 等）。

由于从一个不同的编译单元使用符号，要求该单元也使用这些标志编译，因此我们建议
尽可能不要使用外部符号。