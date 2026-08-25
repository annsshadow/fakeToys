
## APEI 错误注入（Error INJection

EINJ 提供了一种硬件错误注入机制。它对于调试和测APEI 以及 RAS 功能
总体而言非常有用
你需要先检查你BIOS 是否支持 EINJ。为此，查找
```
  ACPI: EINJ 0x000000007370A000 000150 (v01 INTEL           00000001 INTL 00000001)
```
这表BIOS 暴露了一EINJ —它正是执行注入所经由的机制
另一种方法是/sys/firmware/acpi/tables 中查找一"EINJ" 文件，它同一事物的另一种表示形式
上述两者都不存在，并不一定意味着 EINJ 不被支持：在放弃之前，请进入 BIOS
设置，看BIOS 是否有一个用于启用错误注入的选项。查找名WHEA 或类似的
东西。通常，你需要先启用一ACPI5 支持选项，才能看BIOS 菜单所支持暴露APEI、EINJ……功能
要使EINJ，请确保你的内核中启用了以下选项
```
  CONFIG_DEBUG_FS
  CONFIG_ACPI_APEI
  CONFIG_ACPI_APEI_EINJ
```
```
  CONFIG_ACPI_APEI_EINJ_CXL
```
EINJ 的用户接口位<debugfs 挂载/apei/einj
属于它的文件如下
- available_error_type

  该文件显示了支持哪些错误类型
  ================  ===================================
  Error Type Value	错误描述
  ================  ===================================
  0x00000001        处理器可纠正错误
  0x00000002        处理器不可纠正非致命错误
  0x00000004        处理器不可纠正致命错  0x00000008        内存可纠正错  0x00000010        内存不可纠正非致命错  0x00000020        内存不可纠正致命错误
  0x00000040        PCI Express 可纠正错  0x00000080        PCI Express 不可纠正非致命错  0x00000100        PCI Express 不可纠正致命错误
  0x00000200        平台可纠正错  0x00000400        平台不可纠正非致命错  0x00000800        平台不可纠正致命错误
  V2_0x00000001     EINJV2 处理器错  V2_0x00000002     EINJV2 内存错误
  V2_0x00000004     EINJV2 PCI Express 错误
  ================  ===================================

  文件内容的格式如上所示，只是其中只出现可用的错误类型
- error_type

  设置正在注入的错误类型的值。可能的错误类型定义在上方的
  available_error_type 文件中
- error_inject

  向该文件写入任意整数以触发错误注入。请确保你已经指定了所有必要的错误
  参数，即这次写入应当是注入错误时的最后一步
- flags

  在内核版3.13 及以上存在。用于指param{1..4} 中哪些有效、并应在注入
  期间被固件使用。其值是一个位掩码，定义于 ACPI5.0 规范  SET_ERROR_TYPE_WITH_ADDRESS 数据结构
    Bit 0
      处理APIC 字段有效（见下方 param3）    Bit 1
      内存地址和掩码有效（param1 param2）    Bit 2
      PCIe（seg、bus、dev、fn）有效（见下param4）    Bit 3
      EINJv2 扩展结构有效

  如果置为零，则模拟传统行为，此时注入类型只指定一个置位的位，param1
  被多路复用
- param1

  该文件用于设置第一个错误参数值。其作用取决error_type 中指定的错误类型  例如，如果错误类型是内存相关类型，则 param1 应当是一个有效的物理内存地址  [除非设置"flag" —见上]

- param2

  用途同上方param1。例如，如果错误类型是内存相关类型，param2 应当是一  物理内存地址掩码。Linux 要求页粒度或更细，例0xfffffffffffff000
- param3

  "flags" 中的 0x1 位置位时使用，用于指APIC id

- param4
  "flags" 中的 0x4 位置位时使用，用于指定目PCIe 设备

- notrigger

  错误注入机制是一个两步过程。先注入错误，再执行一些操作来触发它。将
  "notrigger" 设为 1 会跳过触发阶段，*可能**允许用户通过对作为错误注  目标CPU、内存位置或设备进行简单访问，而在其他某个上下文中引发错误  这实际是否有效，取决BIOS 在触发阶段实际包含了哪些操作
- component_id0 .. component_idN, component_syndrome0 .. component_syndromeN

  这些文件用于设置 EINJv2 扩展结构"Component Array"（组件数组）字段  每个文件保存一128 位的十六进制值。向这些文件中的任意一个只写入一  换行符，会将其设置为无效（全 1）值
CXL 错误类型ACPI 6.5 起得到支持（前提是存CXL 端口）。用CXL 错误
类型EINJ 用户接口位于 <debugfs 挂载/cxl。属于它的文件如下：

- einj_types:

  提供与上available_error_types 相同的功能，但针CXL 错误类型

- $dport_dev/einj_inject:

  将一CXL 错误类型注入到由 $dport_dev 表示CXL 端口，其$dport_dev
  CXL 端口的名称（通常是一PCIe 设备名）。针CXL 2.0+ 端口的错误注  可以使用位于 <debugfs 挂载/apei/einj 下的传统接口，CXL 1.1/1.0 端口
  的注入必须使用这个文件

基于 ACPI 4.0 规范BIOS 版本在控制错误注入位置方面的选项有限。你BIOS
可能支持一个扩展（通过 param_extension=1 模块参数，或启动命令einj.param_extension=1 启用）。这允许内存注入的地址和掩码由 apei/einj 中的
param1 param2 文件指定
基于 ACPI 5.0 规范BIOS 版本对注入目标有更强的控制能力。对于处理器相关错误（类0x1x2 0x4），你可以将 flags 设为 0x3（bit 0 对应 param3bit 1 对应 param1 param2），以便向错误添加更多信```
	memory_address = param1;
	memory_address_range = param2;
	apicid = param3;
	pcie_sbdf = param4;
```
对于内存错误（类0x8x10 0x20），地址param1 设置，掩码在 param2 x0 等价于全 1）。对PCI Express 错误（类0x40x80 0x100），段总线、设备和
```
         31     24 23    16 15    11 10      8  7        0
	+-------------------------------------------------+
	| segment |   bus  | device | function | reserved |
	+-------------------------------------------------+
```
总之，你明白这个意思就够了，如果有疑问，看一drivers/acpi/apei/einj.c
中的代码
基于 ACPI 5.0 BIOS 也可能允许注入厂商特定的错误。在这种情况下，一个名vendor 的文件会包含来自 BIOS 的标识信息，希望能让想要使用该厂商特定扩展的
应用程序判断自己是否运行在支持它BIOS 上。所有厂商扩展在 error_type 都有 0x80000000 位置位。一个名vendor_flags 的文件控param1 param2
的解释（1 = PROCESSOR = MEMORY = PCI）。详情请参阅你的 BIOS 厂商文档
（并且如果厂商在使用此功能上的创意超出我们预期，这个 API 还会有变动）

```
  # cd /sys/kernel/debug/apei/einj
  # cat available_error_type		# See which errors can be injected
  0x00000002	Processor Uncorrectable non-fatal
  0x00000008	Memory Correctable
  0x00000010	Memory Uncorrectable non-fatal
  # echo 0x12345000 > param1		# Set memory address for injection
  # echo 0xfffffffffffff000 > param2		# Mask - anywhere in this page
  # echo 0x8 > error_type			# Choose correctable memory error
  # echo 1 > error_inject			# Inject now
```
```
  # cd /sys/kernel/debug/apei/einj
  # cat available_error_type			# See which errors can be injected
  0x00000002	Processor Uncorrectable non-fatal
  0x00000008	Memory Correctable
  0x00000010	Memory Uncorrectable non-fatal
  V2_0x00000001	EINJV2 Processor Error
  V2_0x00000002	EINJV2 Memory Error

  # echo 0x12345000 > param1			# Set memory address for injection
  # echo 0xfffffffffffff000 > param2		# Range - anywhere in this page
  # echo 0x1 > component_id0			# First device ID
  # echo 0x4 > component_syndrome0		# First error syndrome
  # echo 0x2 > component_id1			# Second device ID
  # echo 0x4 > component_syndrome1		# Second error syndrome
  # echo '' > component_id2			# Mark id2 invalid to terminate list
  # echo V2_0x2 > error_type			# Choose EINJv2 memory error
  # echo 0xa > flags				# set flags to indicate EINJv2
  # echo 1 > error_inject			# Inject now
```
```
  [22715.830801] EDAC sbridge MC3: HANDLING MCE MEMORY ERROR
  [22715.834759] EDAC sbridge MC3: CPU 0: Machine Check Event: 0 Bank 7: 8c00004000010090
  [22715.834759] EDAC sbridge MC3: TSC 0
  [22715.834759] EDAC sbridge MC3: ADDR 12345000 EDAC sbridge MC3: MISC 144780c86
  [22715.834759] EDAC sbridge MC3: PROCESSOR 0:306e7 TIME 1422553404 SOCKET 0 APIC 0
  [22716.616173] EDAC MC3: 1 CE memory read error on CPU_SrcID#0_Channel#0_DIMM#0 (channel:0 slot:0 page:0x12345 offset:0x0 grain:32 syndrome:0x0 -  area:DRAM err_code:0001:0090 socket:0 channel_mask:1 rank:0)
```
```
    # cd /sys/kernel/debug/cxl/
    # ls
    0000:e0:01.1 0000:0c:00.0
    # cat einj_types                # See which errors can be injected
	0x00008000  CXL.mem Protocol Correctable
	0x00010000  CXL.mem Protocol Uncorrectable non-fatal
	0x00020000  CXL.mem Protocol Uncorrectable fatal
    # cd 0000:e0:01.1               # Navigate to dport to inject into
    # echo 0x8000 > einj_inject     # Inject error
```
针对 SGX enclave 注入的特殊说明：

可能会有一个单独的 BIOS 设置选项用于启用 SGX 注入
注入过程包括设置某个特殊的内存控制器触发器，它会在下一次对目标地址的写入时
注入错误。但硬件阻止 SGX enclave 之外的任何软件（甚至 BIOS SMM 模式）访enclave 页
可以使用以下顺序  1) 确定 enclave 页的物理地址
  2) 使用 "notrigger=1" 模式进行注入（这会设置注入地址，但并不会实际注入）
  3) 进入 enclave
  4) 向与1 步物理地址匹配的虚拟地址写入数据
  5) 对该虚拟地址执行 CLFLUSH
  6) 自旋延迟 250ms
  7) 从该虚拟地址读取。这会触发错
关于 EINJ 的更多信息，请参ACPI 规范 4.0 版第 17.5 节和 ACPI 5.0 18.6 节