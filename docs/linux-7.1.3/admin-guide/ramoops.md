## Ramoops oops/panic 记录

Sergiu Iordache <sergiu@chromium.org>

Updated: 10 Feb 2021

### 简

Ramoops 是一oops/panic 记录器，在系统崩溃前将其日志写入 RAM。它通过在环缓冲区中记录 oops panic 来工作。Ramoops 需要系统具备持RAM，以便该区域内容在重启后能够保留
### Ramoops 概念


Ramoops 使用预定义的内存区域来存储转储。该内存区域的起始、大小和类型通过以下
三个变量设置
  - `mem_address` 表示起始地址
  - `mem_size` 表示大小。内存大小会向下取整2 的幂  - `mem_type` 用于指定内存类型（默认是 pgprot_writecombine）  - `mem_name` 用于指定`reserve_mem` 命令行参数定义的内存区域
通常应使`mem_type=0` 的默认值，因为这会pstore 映射设置pgprot_writecombine。设`mem_type=1` 会尝试使`pgprot_noncached`，这仅在
某些平台上有效。这是因pstore 依赖于原子操作。至少在 ARM 上，pgprot_noncached
会使内存被映射为强有序（strongly ordered），而对强有序内存的原子操作是依实现
而定的，并且在许ARM（如 omaps）上无法工作。设`mem_type=2` 会尝试将该内区域当作普通内存处理，从而启用其上的完整缓存。这可以提升性能
该内存区域被划分`record_size` 大小的块（同样向下取整为 2 的幂），每次 kmesg
转储会写入一`record_size` 大小的信息块
可以通过 `max_reason` 值来限制存储哪些类型kmsg 转储，该值定义于
include/linux/kmsg_dump.h `enum kmsg_dump_reason`。例如，要同时存Oops Panic，`max_reason` 应设2（KMSG_DUMP_OOPS）；要只存储 Panic，`max_reason`
应设1（KMSG_DUMP_PANIC）。将其设0（KMSG_DUMP_UNDEF）时，原因过滤将`printk.always_kmsg_dump` 启动参数控制：若未设置，则为 KMSG_DUMP_OOPS，否KMSG_DUMP_MAX
该模块使用一个计数器来记录多次转储，但计数器会在重启时重置（即重启后的新转储
会覆盖旧的）
Ramoops 还支持对持久内存区域的软ECC 保护。当使用硬件复位使机器恢复（例如
看门狗触发）时，这可能很有用。在这种情况下，RAM 可能略有损坏，但通常可以恢复
### 设置参数


设置 ramoops 参数有几种不同的方式
```
 A. 使用模块参数（其名称即前述变量名）。为了快速调试，您也可以在启动期间保 部分内存，然后将保留的内存用ramoops。例如，假设一台内存大128 MB 的机器，
 以下内核命令行将告诉内核只使用前 128 MB 内存，并ECC 保护
	mem=128M ramoops.mem_address=0x8000000 ramoops.ecc=1

 B. 使用设备树绑定，``Documentation/devicetree/bindings/reserved-memory/ramoops.yaml``
 所述。例如：

	reserved-memory {
		#address-cells = <2>;
		#size-cells = <2>;
		ranges;

		ramoops@8f000000 {
			compatible = "ramoops";
			reg = <0 0x8f000000 0 0x100000>;
			record-size = <0x4000>;
			console-size = <0x4000>;
		};
	};

 C. 使用平台设备并设置平台数据。然后可以通过该平台数据设置参数。示例如下：

 .. code-block:: c

  #include <linux/pstore_ram.h>
  [...]

  static struct ramoops_platform_data ramoops_data = {
        .mem_size               = <...>,
        .mem_address            = <...>,
        .mem_type               = <...>,
        .record_size            = <...>,
        .max_reason             = <...>,
        .ecc                    = <...>,
  };

  static struct platform_device ramoops_dev = {
        .name = "ramoops",
        .dev = {
                .platform_data = &ramoops_data,
        },
  };

  [... inside a function ...]
  int ret;

  ret = platform_device_register(&ramoops_dev);
  if (ret) {
	printk(KERN_ERR "unable to register platform device\n");
	return ret;
  }

 D. 使用通过 ``reserve_mem`` 命令行参数保留的内存区域。地址和大小由 ``reserve_mem``
 参数定义。请注意，``reserve_mem`` 不一定总是在同一位置分配内存，因此不可依赖 需要进行测试，并且它可能并非在每台机器或每个内核上都有效。请将此视为"尽力而为"
 的方式。``reserve_mem`` 选项接受大小、对齐和名称作为参数。该名称用于将内存映 到一个标签，ramoops 可据此检索
	reserve_mem=2M:4096:oops  ramoops.mem_name=oops
```
您可以指RAM 或外设的内存。但是，当指RAM 时，请务必通过发出 memblock_reserve()

```
	#include <linux/memblock.h>

	memblock_reserve(ramoops_data.mem_address, ramoops_data.mem_size);

```
### 转储格式


数据转储以一个头部开始，当前定义`====`，后跟时间戳和换行符。随后是实际数据
### 读取数据


转储数据可以pstore 文件系统读取。这些文件的格式`dmesg-ramoops-N`，其N 是内存中的记录号。要RAM 中删除已存储的记录，只需取消链接相应pstore 文件
### 持久函数跟踪


持久函数跟踪可能有助于调试与软件或硬件相关的挂起。函数调用链日志存储`ftrace-ramoops`

```
 # mount -t debugfs debugfs /sys/kernel/debug/
 # echo 1 > /sys/kernel/debug/pstore/record_ftrace
 # reboot -f
 [...]
 # mount -t pstore pstore /mnt/
 # tail /mnt/ftrace-ramoops
 0 ffffffff8101ea64  ffffffff8101bcda  native_apic_mem_read <- disconnect_bsp_APIC+0x6a/0xc0
 0 ffffffff8101ea44  ffffffff8101bcf6  native_apic_mem_write <- disconnect_bsp_APIC+0x86/0xc0
 0 ffffffff81020084  ffffffff8101a4b5  hpet_disable <- native_machine_shutdown+0x75/0x90
 0 ffffffff81005f94  ffffffff8101a4bb  iommu_shutdown_noop <- native_machine_shutdown+0x7b/0x90
 0 ffffffff8101a6a1  ffffffff8101a437  native_machine_emergency_restart <- native_machine_restart+0x37/0x40
 0 ffffffff811f9876  ffffffff8101a73a  acpi_reboot <- native_machine_emergency_restart+0xaa/0x1e0
 0 ffffffff8101a514  ffffffff8101a772  mach_reboot_fixups <- native_machine_emergency_restart+0xe2/0x1e0
 0 ffffffff811d9c54  ffffffff8101a7a0  __const_udelay <- native_machine_emergency_restart+0x110/0x1e0
 0 ffffffff811d9c34  ffffffff811d9c80  __delay <- __const_udelay+0x30/0x40
 0 ffffffff811d9d14  ffffffff811d9c3f  delay_tsc <- __delay+0xf/0x20

```
