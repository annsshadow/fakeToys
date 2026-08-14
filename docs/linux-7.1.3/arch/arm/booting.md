## Booting ARM Linux


Author:	Russell King

Date  : 18 可 2002

The 以下 documentation 是 relevant 到 2.4.18-rmk6 和 beyond.

为了 boot ARM Linux, 您 需要 一个 boot loader, 其 是 一个 small
program 该 runs 之前 the 主要 内核.  The boot loader 是 expected
到 initialise 各种 设备, 和 eventually call the Linux 内核,
passing information 到 the 内核.

Essentially, the boot loader 应当 提供 (作为 一个 最小) the
以下:

1. Setup 和 initialise the RAM.
2. Initialise one 串行 端口.
3. Detect the machine 类型.
4. Setup the 内核 tagged 列出.
5. 加载 initramfs.
6. Call the 内核 image.


### 1. Setup 和 initialise RAM


Existing boot loaders:
	MANDATORY
新 boot loaders:
	MANDATORY

The boot loader 是 expected 到 find 和 initialise 全部 RAM 该 the
内核 将 使用 用于 volatile 数据 storage 在 the 系统.  它 performs
此 在 一个 machine dependent manner.  (它 可 使用 内部 algorithms
到 automatically locate 和 大小 全部 RAM, 或 它 可 使用 knowledge 的
the RAM 在 the machine, 或 任何 其他 方法 the boot loader designer
sees fit.)


### 2. Initialise one 串行 端口


Existing boot loaders:
	可选, RECOMMENDED
新 boot loaders:
	可选, RECOMMENDED

The boot loader 应当 initialise 和 启用 one 串行 端口 在 the
target.  此 allows the 内核 串行 驱动 到 automatically detect
其 串行 端口 它 应当 使用 用于 the 内核 console (generally
使用 用于 debugging purposes, 或 communication 与 the target.)

作为 一个 alternative, the boot loader 可 pass the relevant 'console='
选项 到 the 内核 通过 the tagged 列表 specifying the 端口, 和
串行 格式 选项 作为 描述 在

       Documentation/admin-guide/kernel-parameters.rst.


### 3. Detect the machine 类型


Existing boot loaders:
	可选
新 boot loaders:
	MANDATORY except 用于 DT-仅 platforms

The boot loader 应当 detect the machine 类型 其 运行中 在 由 一些
方法.  是否 这是 一个 hard coded 值 或 一些 algorithm 该
looks 在 the connected 硬件 是 beyond the scope 的 此 document.
The boot loader 必须 ultimately 为 able 到 提供 一个 MACH_类型_xxx
值 到 the 内核. (参见 linux/arch/arm/tools/mach-types).  此
应当 为 passed 到 the 内核 在 注册 r1.

用于 DT-仅 platforms, the machine 类型 将 为 determined 由 设备
tree.  set the machine 类型 到 全部 ones (~0).  这是 不 strictly
必要, 但 assures 该 它 将 不 match 任何 existing types.

### 4. Setup boot 数据


Existing boot loaders:
	可选, HIGHLY RECOMMENDED
新 boot loaders:
	MANDATORY

The boot loader 必须 提供 任一个 一个 tagged 列出 或 一个 dtb image 用于
passing 配置 数据 到 the 内核.  The 物理 地址 的 the
boot 数据 是 passed 到 the 内核 在 注册 r2.

### 4一个. Setup the 内核 tagged 列出


The boot loader 必须 创建 和 initialise the 内核 tagged 列出.
一个 valid tagged 列出 starts 与 ATAG_核心 和 ends 与 ATAG_NONE.
The ATAG_核心 tag 可 或 可 不 为 empty.  一个 empty ATAG_核心 tag
具有 the 大小 字段 set 到 '2' (0x00000002).  The ATAG_NONE 必须 set
the 大小 字段 到 zero.

任何 数字 的 tags 可 为 placed 在 the 列出.  它是 undefined
是否 一个 repeated tag appends 到 the information carried 由 the
前一个 tag, 或 是否 它 replaces the information 在 其
entirety; 一些 tags behave 作为 the former, others the latter.

The boot loader 必须 pass 在 一个 最小 the 大小 和 location 的
the 系统 内存, 和 root 文件系统 location.  因此, the
```

		+-----------+
  base ->	| ATAG_CORE |  |
		+-----------+  |
		| ATAG_MEM  |  | increasing address
		+-----------+  |
		| ATAG_NONE |  |
		+-----------+  v

```
The tagged 列出 应当 为 stored 在 系统 RAM.

The tagged 列出 必须 为 placed 在 一个 region 的 内存 何处 两者都不
the 内核 decompressor nor initrd 'bootp' program 将 overwrite
它.  The recommended placement 是 在 the 第一 16KiB 的 RAM.

### 4b. Setup the 设备树


The boot loader 必须 加载 一个 设备树 image (dtb) 进入 系统 ram
在 一个 64位 aligned 地址 和 initialize 它 与 the boot 数据.  The
dtb 格式 是 documented 在 https://www.devicetree.org/specifications/.
The 内核 将 look 用于 the dtb magic 值 的 0xd00dfeed 在 the dtb
物理 地址 到 determine 若 一个 dtb 具有 已经 passed 而非 一个
tagged 列出.

The boot loader 必须 pass 在 一个 最小 the 大小 和 location 的 the
系统 内存, 和 the root 文件系统 location.  The dtb 必须 为
placed 在 一个 region 的 内存 何处 the 内核 decompressor 将 不
overwrite 它, 同时 remaining 之内 the region 其 将 为 covered
由 the 内核's low-memory 映射.

一个 safe location 是 just 上文 the 128MiB boundary 来自 启动 的 RAM.

### 5. 加载 initramfs.


Existing boot loaders:
	可选
新 boot loaders:
	可选

若 一个 initramfs 是 在 使用 然后, 作为 与 the dtb, 它 必须 为 placed 在
一个 region 的 内存 何处 the 内核 decompressor 将 不 overwrite 它
同时 也 与 the region 其 将 为 covered 由 the 内核's
low-memory 映射.

一个 safe location 是 just 上文 the 设备树 blob 其 itself 将
为 loaded just 上文 the 128MiB boundary 来自 the 启动 的 RAM 作为
recommended 上文.

### 6. Calling the 内核 image


Existing boot loaders:
	MANDATORY
新 boot loaders:
	MANDATORY

存在 two 选项 用于 calling the 内核 zImage.  若 the zImage
是 stored 在 flash, 和 是 linked correctly 到 为 运行 来自 flash,
然后 它是 legal 用于 the boot loader 到 call the zImage 在 flash
directly.

The zImage 可 也 为 placed 在 系统 RAM 和 called 那里.  The
内核 应当 为 placed 在 the 第一 128MiB 的 RAM.  它是 recommended
该 它是 loaded 上文 32MiB 为了 avoid the 需要 到 relocate
prior 到 decompression, 其 将 make the boot 进程 slightly
faster.

当 booting 一个 raw (non-zImage) 内核 the constraints 是 tighter.
在 此 case the 内核 必须 为 loaded 在 一个 偏移 进入 系统 equal
到 TEXT_偏移 - 页_偏移.

在 任何 case, the 以下 conditions 必须 为 met:

- Quiesce 全部 DMA capable 设备 因此 该 内存 执行 不 get
  corrupted 由 bogus 网络 packets 或 disk 数据. 此 将 save
  您 许多 hours 的 debug.

- CPU 注册 设置

  - r0 = 0,
  - r1 = machine 类型 数字 discovered 在 (3) 上文.
  - r2 = 物理 地址 的 tagged 列出 在 系统 RAM, 或
    物理 地址 的 设备树 块 (dtb) 在 系统 RAM

- CPU 模式

  全部 forms 的 中断 必须 为 已禁用 (IRQs 和 FIQs)

  用于 CPUs 其 执行 不 包含 the ARM virtualization extensions, the
  CPU 必须 为 在 SVC 模式.  (一个 特殊 异常 exists 用于 Angel)

  CPUs 其 包含 支持 用于 the virtualization extensions 可 为
  entered 在 HYP 模式 为了 启用 the 内核 到 make full 使用 的
  这些 extensions.  这是 the recommended boot 方法 用于 此类 CPUs,
  除非 the virtualisations 是 已经 在 使用 由 一个 pre-installed
  hypervisor.

  若 the 内核 是 不 entered 在 HYP 模式 用于 任何 reason, 它 必须 为
  entered 在 SVC 模式.

- Caches, MMUs

  The MMU 必须 为 off.

  Instruction 缓存 可 为 在 或 off.

  数据 缓存 必须 为 off.

  若 the 内核 是 entered 在 HYP 模式, the 上文 requirements apply 到
  the HYP 模式 配置 此外 到 the ordinary PL1 (privileged
  内核 modes) 配置.  此外, 全部 traps 进入 the
  hypervisor 必须 为 已禁用, 和 PL1 access 必须 为 granted 用于 全部
  peripherals 和 CPU resources 用于 其 这是 architecturally
  可能.  Except 用于 entering 在 HYP 模式, the 系统 配置
  应当 为 此类 该 一个 内核 其 执行 不 包含 支持 用于 the
  virtualization extensions 可 boot correctly 无 extra help.

- The boot loader 是 expected 到 call the 内核 image 由 jumping
  directly 到 the 第一 instruction 的 the 内核 image.

  在 CPUs supporting the ARM instruction set, the 条目 必须 为
  made 在 ARM 状态, even 用于 一个 Thumb-2 内核.

  在 CPUs supporting 仅 the Thumb instruction set 例如
  Cortex-M 类 CPUs, the 条目 必须 为 made 在 Thumb 状态.
