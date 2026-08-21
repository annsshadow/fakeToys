## Booting ARM Linux


Author:	Russell King

Date  : 18 鍙?2002

The 以下 documentation relevant 2.4.18-rmk6 beyond.

为了 boot ARM Linux, 需一boot loader, 一small
program runs 之前 the 主要 内核.  The boot loader expected
initialise 各种 设备, eventually call the Linux 内核,
passing information the 内核.

Essentially, the boot loader 应当 提供 (作为 一最 the
以下:

1. Setup 鍜?initialise the RAM.
2. Initialise one 串行 端口.
3. Detect the machine 类型.
4. Setup the 内核 tagged 列出.
5. 加载 initramfs.
6. Call the 内核 image.


### 1. Setup 鍜?initialise RAM


Existing boot loaders:
	MANDATORY
鏂?boot loaders:
	MANDATORY

The boot loader expected find initialise 全部 RAM the
内核 使用 用于 volatile 数据 storage the 系统.  performs
一machine dependent manner.  (使用 内部 algorithms
automatically locate 大小 全部 RAM, 使用 knowledge 
the RAM the machine, 任何 其他 方法 the boot loader designer
sees fit.)


### 2. Initialise one 串行 端口


Existing boot loaders:
	可 RECOMMENDED
鏂?boot loaders:
	可 RECOMMENDED

The boot loader 应当 initialise 启用 one 串行 端口 the
target.  allows the 内核 串行 驱动 automatically detect
串行 端口 应当 使用 用于 the 内核 console (generally
使用 用于 debugging purposes, communication the target.)

作为 一alternative, the boot loader pass the relevant 'console='
选项 the 内核 通过 the tagged 列表 specifying the 端口, 
串行 格式 选项 作为 描述 

       Documentation/admin-guide/kernel-parameters.rst.


### 3. Detect the machine 类型


Existing boot loaders:
	可
鏂?boot loaders:
	MANDATORY except 用于 DT-platforms

The boot loader 应当 detect the machine 类型 运行一
方法.  是否 这是 一hard coded 一algorithm 
looks the connected 硬件 beyond the scope document.
The boot loader 必须 ultimately able 提供 一MACH_类型_xxx
the 内核. (参见 linux/arch/arm/tools/mach-types).  
应当 passed the 内核 注册 r1.

用于 DT-platforms, the machine 类型 determined 设备
tree.  set the machine 类型 全部 ones (~0).  这是 strictly
必要, assures match 任何 existing types.

### 4. Setup boot 数据


Existing boot loaders:
	可 HIGHLY RECOMMENDED
鏂?boot loaders:
	MANDATORY

The boot loader 必须 提供 任一一tagged 列出 一dtb image 用于
passing 配置 数据 the 内核.  The 物理 地址 the
boot 数据 passed the 内核 注册 r2.

### 4一 Setup the 内核 tagged 列出


The boot loader 必须 创建 initialise the 内核 tagged 列出.
一valid tagged 列出 starts ATAG_核心 ends ATAG_NONE.
The ATAG_核心 tag empty.  一empty ATAG_核心 tag
具有 the 大小 字段 set '2' (0x00000002).  The ATAG_NONE 必须 set
the 大小 字段 zero.

任何 数字 tags placed the 列出.  它是 undefined
是否 一repeated tag appends the information carried the
前一tag, 是否 replaces the information 
entirety; 一tags behave 作为 the former, others the latter.

The boot loader 必须 pass 一最the 大小 location 
the 系统 内存, root 文件系统 location.  因此, the
```

		+-----------+
  base ->	| ATAG_CORE |  |
		+-----------+  |
		| ATAG_MEM  |  | increasing address
		+-----------+  |
		| ATAG_NONE |  |
		+-----------+  v

```
The tagged 列出 应当 stored 系统 RAM.

The tagged 列出 必须 placed 一region 内存 何处 两者都
the 鍐呮牳 decompressor nor initrd 'bootp' program 灏?overwrite
  The recommended placement the 第一 16KiB RAM.

### 4b. Setup the 设备


The boot loader 必须 加载 一设备image (dtb) 进入 系统 ram
一64aligned 地址 initialize the boot 数据.  The
dtb 格式 documented https://www.devicetree.org/specifications/.
The 内核 look 用于 the dtb magic 0xd00dfeed the dtb
物理 地址 determine 一dtb 具有 已经 passed 而非 一
tagged 列出.

The boot loader 必须 pass 一最the 大小 location the
系统 内存, the root 文件系统 location.  The dtb 必须 
placed 一region 内存 何处 the 内核 decompressor 
overwrite  同时 remaining 之内 the region covered
the 内核's low-memory 映射.

一safe location just 上文 the 128MiB boundary 来自 启动 RAM.

### 5. 加载 initramfs.


Existing boot loaders:
	可
鏂?boot loaders:
	可

一initramfs 使用 然后, 作为 the dtb, 必须 placed 
一region 内存 何处 the 内核 decompressor overwrite 
同时 the region covered the 内核's
low-memory 映射.

一safe location just 上文 the 设备blob itself 
loaded just 上文 the 128MiB boundary 来自 the 启动 RAM 作为
recommended 上文.

### 6. Calling the 内核 image


Existing boot loaders:
	MANDATORY
鏂?boot loaders:
	MANDATORY

存在 two 选项 用于 calling the 内核 zImage.  the zImage
stored flash, linked correctly 运行 来自 flash,
然后 它是 legal 用于 the boot loader call the zImage flash
directly.

The zImage placed 系统 RAM called 那里.  The
内核 应当 placed the 第一 128MiB RAM.  它是 recommended
它是 loaded 上文 32MiB 为了 avoid the 需relocate
prior decompression, make the boot 进程 slightly
faster.

booting 一raw (non-zImage) 内核 the constraints tighter.
case the 内核 必须 loaded 一偏移 进入 系统 equal
TEXT_偏移 - 页_偏移.

任何 case, the 以下 conditions 必须 met:

- Quiesce 全部 DMA capable 设备 因此 内存 执行 get
  corrupted bogus 网络 packets disk 数据. save
  许多 hours debug.

- CPU 注册 设置

  - r0 = 0,
  - r1 = machine 类型 数字 discovered (3) 上文.
  - r2 = 物理 地址 tagged 列出 系统 RAM, 
    物理 地址 设备(dtb) 系统 RAM

- CPU 模式

  全部 forms 中断 必须 已禁(IRQs FIQs)

  用于 CPUs 执行 包含 the ARM virtualization extensions, the
  CPU 必须 SVC 模式.  (一特殊 异常 exists 用于 Angel)

  CPUs 包含 支持 用于 the virtualization extensions 
  entered HYP 模式 为了 启用 the 内核 make full 使用 
  这些 extensions.  这是 the recommended boot 方法 用于 此类 CPUs,
  除非 the virtualisations 已经 使用 一pre-installed
  hypervisor.

  the 内核 entered HYP 模式 用于 任何 reason, 必须 
  entered SVC 模式.

- Caches, MMUs

  The MMU 必须 off.

  Instruction 缓存 off.

  数据 缓存 必须 off.

  the 内核 entered HYP 模式, the 上文 requirements apply 
  the HYP 模式 配置 此外 the ordinary PL1 (privileged
  内核 modes) 配置.  此外, 全部 traps 进入 the
  hypervisor 必须 已禁 PL1 access 必须 granted 用于 全部
  peripherals CPU resources 用于 这是 architecturally
  可能.  Except 用于 entering HYP 模式, the 系统 配置
  应当 此类 一内核 执行 包含 支持 用于 the
  virtualization extensions 鍙?boot correctly 鏃?extra help.

- The boot loader expected call the 内核 image jumping
  directly the 第一 instruction the 内核 image.

  CPUs supporting the ARM instruction set, the 条目 必须 
  made ARM 状 even 用于 一Thumb-2 内核.

  CPUs supporting the Thumb instruction set 例如
  Cortex-M CPUs, the 条目 必须 made Thumb 状
