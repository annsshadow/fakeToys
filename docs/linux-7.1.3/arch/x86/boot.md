
## Linux/x86 引导协议


x86 平台上，Linux 内核采用了一套相当复杂的引导约定。这一约定部分源于历史原因，也源自早期希望内核本身成为可引导映像的想法、复杂的 PC 内存模型，以及随着实模DOS 作为主流操作系统逐渐退出舞台，PC 行业期望发生变化所致

目前，Linux/x86 引导协议存在以下版本

=============	============================================================
Old kernels	仅支zImage/Image。一些非常早期的内核甚至可能不支持命令行

Protocol 2.00	（内1.3.73）新bzImage initrd 支持，以及引导加载程
		与内核之间正式的通信方式。setup.S 变为可重定位，但传统
		setup 区域仍假定可写

Protocol 2.01	（内1.3.76）新增堆溢出警告

Protocol 2.02	（内2.4.0-test3-pre3）新的命令行协议。降低常规内存上限
		不再覆盖传统setup 区域，从而让那些通过 SMM 32 BIOS
		入口点使EBDA 的系统引导更加安全。zImage 已弃用但仍受支持

Protocol 2.03	（内2.4.18-pre1）显式地向引导加载程序提供可能的最
		initrd 地址

Protocol 2.04	（内2.6.14）将 syssize 字段扩展到四个字节

Protocol 2.05	（内2.6.20）使保护模式内核可重定位。引relocatable_kernel
		kernel_alignment 字段

Protocol 2.06	（内2.6.22）新增一个字段，用于保存引导命令行的大小

Protocol 2.07	（内2.6.24）新增半虚拟化的引导协议。引hardware_subarch
		hardware_subarch_data 以及 load_flags 中的 KEEP_SEGMENTS 标志

Protocol 2.08	（内2.6.26）新crc32 校验和与 ELF 格式的有效载荷
		引入 payload_offset payload_length 字段以辅助定位有效载荷

Protocol 2.09	（内2.6.26）新增一64 位物理指针字段，指向
		struct setup_data 的单向链表

Protocol 2.10	（内2.6.31）在已有 kernel_alignment 的基础上新增宽松对
		协议，新init_size pref_address 字段。新增扩展的引导
		加载程序 ID

Protocol 2.11	（内3.6）新增一个字段，用于保存 EFI 交接协议入口点的偏移量

Protocol 2.12	（内3.8）新xloadflags 字段struct boot_params 的扩
		字段，用于在 64 位环境下bzImage ramdisk 加载4G 以上

Protocol 2.13	（内3.14）支持在 xloadflags 中设32 位与 64 位标志，
		以支持从 32 EFI 引导 64 位内

Protocol 2.14	因错误提交而作
                ae7e1238e68f2a472a125673ab506d49158c1889
		锛?x86/boot: Add ACPI RSDP address to setup_header"锛。
		不要使用！！！视作与 2.13 相同

Protocol 2.15	（内5.5）新kernel_info kernel_info.setup_type_max
=============	============================================================

     只有setup header 发生变更时才应更改协议版本号。如boot_params 
     kernel_info 发生变更，则无需更新版本号。此外，建议使用 xloadflags（这
     情况下协议版本号也不应更新）kernel_info 来向引导加载程序传达 Linux
     内核所支持的特性。由于原setup header 中的可用空间非常有限，对其的任何
     更新都应极为谨慎。从协议 2.15 开始，向引导加载程序传达信息的主要方式
     kernel_info銆。


## 内存布局


用于 Image 或以下内核加载器的传统内存映射：

```
		|  			 |
  0A0000	+------------------------+
		|  Reserved for BIOS	 |	Do not use.  Reserved for BIOS EBDA.
  09A000	+------------------------+
		|  Command line		 |
		|  Stack/heap		 |	For use by the kernel real-mode code.
  098000	+------------------------+
		|  Kernel setup		 |	The kernel real-mode code.
  090200	+------------------------+
		|  Kernel boot sector	 |	The kernel legacy boot sector.
  090000	+------------------------+
		|  Protected-mode kernel |	The bulk of the kernel image.
  010000	+------------------------+
		|  Boot loader		 |	<- Boot sector entry point 0000:7C00
  001000	+------------------------+
		|  Reserved for MBR/BIOS |
  000800	+------------------------+
		|  Typically used by MBR |
  000600	+------------------------+
		|  BIOS use only	 |
  000000	+------------------------+

```

当使bzImage 时，保护模式内核被重定位0x100000高端内存"），而内核实模式块（引导扇区、setup 以及堆）被设置为可重定位0x10000 到低内存末尾之间的任意地址。遗憾的是，2.00 2.01 协议中，0x90000 以上内存范围仍被内核内部使用.02 协议解决了这一问题

最好将"内存上限"（即引导加载程序触及的低内存最高位置）保持得尽可能低，因为一些较新的 BIOS 已经开始在低内存顶部附近分配大量被称为扩展 BIOS 数据区（Extended BIOS Data Area）的内存。引导加载程序应使用 "INT 12h" BIOS 调用来确认有多少低内存可用

遗憾的是，如INT 12h 报告可用内存过低，引导加载程序通常无能为力，只能向用户报告错误。因此，引导加载程序的设计应尽可能少占用低内存空间。对于需要将数据写入 0x90000 段的 zImage 或旧bzImage 内核，引导加载程序应确保不使0x9A000 以上的内存；有太BIOS 在该位置以上会出错

对于引导协议版本 >= 2.02 的现bzImage 内核，内存布局如下

```
		~  			 ~
		|  Protected-mode kernel |
  100000	+------------------------+
		|  I/O memory hole	 |
  0A0000	+------------------------+
		|  Reserved for BIOS	 |	Leave as much as possible unused
		~  			 ~
		|  Command line		 |	(Can also be below the X+10000 mark)
  X+10000	+------------------------+
		|  Stack/heap		 |	For use by the kernel real-mode code.
  X+08000	+------------------------+
		|  Kernel setup		 |	The kernel real-mode code.
		|  Kernel boot sector	 |	The kernel legacy boot sector.
  X		+------------------------+
		|  Boot loader		 |	<- Boot sector entry point 0000:7C00
  001000	+------------------------+
		|  Reserved for MBR/BIOS |
  000800	+------------------------+
		|  Typically used by MBR |
  000600	+------------------------+
		|  BIOS use only	 |
  000000	+------------------------+

  ... 其中地址 X 取引导加载程序设计所允许的最低值


```

## 实模式内核头


在下面的文字以及内核引导过程的任何地方，"扇区"指的512 字节。它与底层介质的实际扇区大小无关

加载 Linux 内核的第一步应该是加载实模式代码（引导扇区setup 代码），然后检查位于偏0x01f1 处的以下头。实模式代码最多可32K，不过引导加载程序可以选择只加载前两个扇区K），然后检查引导扇区大小

该头的结构如下：

===========	========	=====================	============================================
Offset/Size	Proto		Name			Meaning
===========	========	=====================	============================================
01F1/1		ALL(1)		setup_sects		setup 的大小（以扇区为单位
01F2/2		ALL		root_flags		若设置，则根文件系统以只读方式挂
01F4/4		2.04+(2)	syssize			32 位代码的大小，以 16 字节段落为单
01F8/2		ALL		ram_size		勿用 - 仅供 bootsect.S 使用
01FA/2		ALL		vid_mode		视频模式控制
01FC/2		ALL		root_dev		默认根设备号
01FE/2		ALL		boot_flag		魔数 0xAA55
0200/2		2.00+		jump			跳转指令
0202/4		2.00+		header			魔数签名 "HdrS"
0206/2		2.00+		version			所支持的引导协议版
0208/4		2.00+		realmode_swtch		引导加载程序钩子（见下文
020C/2		2.00+		start_sys_seg		低位加载段（0x1000）（已废弃）
020E/2		2.00+		kernel_version		指向内核版本字符串的指针
0210/1		2.00+		type_of_loader		引导加载程序标识
0211/1		2.00+		loadflags		引导协议选项标志
0212/2		2.00+		setup_move_size		移动到高端内存的大小（与钩子配合使用
0214/4		2.00+		code32_start		引导加载程序钩子（见下文
0218/4		2.00+		ramdisk_image		initrd 加载地址（由引导加载程序设置
021C/4		2.00+		ramdisk_size		initrd 大小（由引导加载程序设置
0220/4		2.00+		bootsect_kludge		勿用 - 仅供 bootsect.S 使用
0224/2		2.01+		heap_end_ptr		setup 结束后的空闲内存
0226/1		2.02+(3)	ext_loader_ver		扩展的引导加载程序版
0227/1		2.02+(3)	ext_loader_type		扩展的引导加载程ID
0228/4		2.02+		cmd_line_ptr		指向内核命令行的 32 位指
022C/4		2.03+		initrd_addr_max		合法的最initrd 地址
0230/4		2.05+		kernel_alignment	内核所需的物理地址对齐
0234/1		2.05+		relocatable_kernel	内核是否可重定位
0235/1		2.10+		min_alignment		最小对齐，2 的幂表示
0236/2		2.12+		xloadflags		引导协议选项标志
0238/4		2.06+		cmdline_size		内核命令行的最大大
023C/4		2.07+		hardware_subarch	硬件子架
0240/8		2.07+		hardware_subarch_data	特定于子架构的数
0248/4		2.08+		payload_offset		内核有效载荷的偏移量
024C/4		2.08+		payload_length		内核有效载荷的长
0250/8		2.09+		setup_data		指向 struct setup_data 链表64 位物理指
0258/8		2.10+		pref_address		偏好的加载地址
0260/4		2.10+		init_size		初始化期间所需的线性内
0264/4		2.11+		handover_offset		交接入口点的偏移
0268/4		2.15+		kernel_info_offset	kernel_info 的偏移量
===========	========	=====================	============================================

     ）为保持向后兼容，如setup_sects 字段0，则真实值为 4

     ）对2.04 之前的引导协议，syssize 字段的高两个字节不可用，这意味着
         无法确定 bzImage 内核的大小

     ）对2.02-2.09 引导协议，该字段被忽略，但设置它是安全的

如果在偏0x202 处没有找"HdrS"x53726448）魔数，则引导协议版本为 "old"（旧版）。加载旧内核时，情况如下

```
  Image type = zImage
  initrd not supported
  Real-mode kernel must be located at 0x90000.

```

否则version" 字段包含协议版本，例如协议版2.01 在该字段中将包含 0x0201。在设置头中的字段时，你必须确保只设置当前所用协议版本所支持的字段


## 头字段详


对于每个字段，有些是内核提供给引导加载程序的信息read"/读），有些需要由引导加载程序填写write"/写），还有些需要由引导加载程序读取并修改（"modify"/修改）。所有通用引导加载程序都应写入标记为（obligatory/必填）的字段。希望将内核加载到非标准地址的引导加载程序应填写标记为（reloc/可重定位）的字段；其他引导加载程序可以忽略这些字段

所有字段的字节序均为小端（毕竟这是 x86）

============	===========
Field name:	setup_sects
Type:		read
Offset/size:	0x1f1/1
Protocol:	ALL
============	===========

  setup 代码的大小，512 字节扇区为单位。如果该字段0，则真实值为 4。实模式代码由引导扇区（始终为一512 字节扇区）加setup 代码组成

============	=================
Field name:	root_flags
Type:		modify (optional)
Offset/size:	0x1f2/2
Protocol:	ALL
============	=================

  如果该字段非零，则根文件系统默认为只读。该字段的使用已被弃用；请改用命令行上的 "ro" "rw" 选项

============	===============================================
Field name:	syssize
Type:		read
Offset/size:	0x1f4/4 (protocol 2.04+) 0x1f4/2 (protocol ALL)
Protocol:	2.04+
============	===============================================

  保护模式代码的大小，16 字节段落为单位。对2.04 之前的协议版本，该字段只有两个字节宽，因此在设置LOAD_HIGH 标志时，不能据此确定内核的大小

============	===============
Field name:	ram_size
Type:		kernel internal
Offset/size:	0x1f8/2
Protocol:	ALL
============	===============

  该字段已废弃

============	===================
Field name:	vid_mode
Type:		modify (obligatory)
Offset/size:	0x1fa/2
============	===================

  请参特殊命令行选项"小节

============	=================
Field name:	root_dev
Type:		modify (optional)
Offset/size:	0x1fc/2
Protocol:	ALL
============	=================

  默认根设备号。该字段的使用已被弃用，请改用命令行上的 "root=" 选项

============	=========
Field name:	boot_flag
Type:		read
Offset/size:	0x1fe/2
Protocol:	ALL
============	=========

  包含 0xAA55。这是旧Linux 内核最接近魔数的东西

============	=======
Field name:	jump
Type:		read
Offset/size:	0x200/2
Protocol:	2.00+
============	=======

  包含一x86 跳转指令，即 0xEB 后跟一个相对于字节 0x202 的有符号偏移量。这可用于确定头的大小

============	=======
Field name:	header
Type:		read
Offset/size:	0x202/4
Protocol:	2.00+
============	=======

  包含魔数 "HdrS"x53726448）

============	=======
Field name:	version
Type:		read
Offset/size:	0x206/2
Protocol:	2.00+
============	=======

  包含引导协议版本，格式为 (major << 8) + minor，例如版2.04 0x0204，假设的版本 10.17 0x0a11

============	=================
Field name:	realmode_swtch
Type:		modify (optional)
Offset/size:	0x208/4
Protocol:	2.00+
============	=================

  引导加载程序钩子（见下文"高级引导加载程序钩子"）

============	=============
Field name:	start_sys_seg
Type:		read
Offset/size:	0x20c/2
Protocol:	2.00+
============	=============

  低位加载段（0x1000）。已废弃

============	==============
Field name:	kernel_version
Type:		read
Offset/size:	0x20e/2
Protocol:	2.00+
============	==============

  若设置为非零值，则包含一个指向以 NUL 结尾、人类可读的内核版本号字符串的指针，减去 0x200。这可用于向用户显示内核版本。该值应小于 (0x200 * setup_sects)。例如，如果该值设0x1c00，则内核版本号字符串可在内核文件偏移 0x1e00 处找到。当且仅"setup_sects" 字段

```
  0x1c00  < 15 * 0x200 (= 0x1e00) but
  0x1c00 >= 14 * 0x200 (= 0x1c00)

  0x1c00 >> 9 = 14, So the minimum value for setup_secs is 15.

```

============	==================
Field name:	type_of_loader
Type:		write (obligatory)
Offset/size:	0x210/1
Protocol:	2.00+
============	==================

  如果你的引导加载程序有一个已分配ID（见下表），则在此填0xTV，其T 是引导加载程序的标识符，V 是版本号。否则，在此填入 0xFF。对T = 0xD 以上的引导加载程ID，将 T = 0xE 写入该字段，并将扩展 ID 减去 0x10 后写ext_loader_type 字段。类似地，ext_loader_ver 字段可用于为引导加载程序版本提供超过 4 位的信息

```
   type_of_loader  <- 0xE4
   ext_loader_type <- 0x05
   ext_loader_ver  <- 0x23

  Assigned boot loader IDs:

	==== =======================================
	0x0  LILO
	     (0x00 reserved for pre-2.00 bootloader)
	0x1  Loadlin
	0x2  bootsect-loader
	     (0x20, all other values reserved)
	0x3  Syslinux
	0x4  Etherboot/gPXE/iPXE
	0x5  ELILO
	0x7  GRUB
	0x8  U-Boot
	0x9  Xen
	0xA  Gujin
	0xB  Qemu
	0xC  Arcturus Networks uCbootloader
	0xD  kexec-tools
	0xE  Extended (see ext_loader_type)
	0xF  Special (0xFF = undefined)
	0x10 Reserved
	0x11 Minimal Linux Bootloader
	     <http://sebastian-plotz.blogspot.de>
	0x12 OVMF UEFI virtualization stack
	0x13 barebox
	==== =======================================

  Please contact <hpa@zytor.com> if you need a bootloader ID value assigned.

```

============	===================
Field name:	loadflags
Type:		modify (obligatory)
Offset/size:	0x211/1
Protocol:	2.00+
============	===================

  该字段是一个位掩码

  Bit 0 (read):	LOADED_HIGH

 - 如果0，保护模式代码加载在 0x10000
 - 如果1，保护模式代码加载在 0x100000

  Bit 1 (kernel internal): KASLR_FLAG

 - 被压缩内核内部使用，用于向真正的内核传达 KASLR 状态

     - 如果1，则启用 KASLR
     - 如果0，则禁用 KASLR

  Bit 5 (write): QUIET_FLAG

 - 如果该位0，则打印早期消息
 - 如果1，则抑制早期消息

		这向内核（解压程序和早期内核）请求不要写入需要直接访问显示硬件的早期消息

  Bit 6 (obsolete): KEEP_SEGMENTS

	Protocol: 2.07+

        - 该标志已废弃

  Bit 7 (write): CAN_USE_HEAP

	将该位设1 表示 heap_end_ptr 中填入的值是有效的。如果该位清零，部分 setup 代码功能将被禁用


============	===================
Field name:	setup_move_size
Type:		modify (obligatory)
Offset/size:	0x212/2
Protocol:	2.00-2.01
============	===================

  当使2.00 2.01 协议时，如果实模式内核未加载0x90000，则会在加载过程的后续步骤被移动到那里。如果你希望除实模式内核本身之外还移动其他数据（例如内核命令行），则填写该字段

  单位是以引导扇区起始处算起的字节数

  当协议为 2.02 或更高，或实模式代码加载0x90000 时，可以忽略该字段

============	========================
Field name:	code32_start
Type:		modify (optional, reloc)
Offset/size:	0x214/4
Protocol:	2.00+
============	========================

  保护模式下跳转到的地址。默认值为内核的加载地址，引导加载程序可用它来确定正确的加载地址

  该字段可出于两个目的被修改：

    1. 作为引导加载程序钩子（见下文"高级引导加载程序钩子"）

    2. 如果一个不安装钩子的引导加载程序将可重定位内核加载到非标准地址，则必须修改该字段以指向加载地址

============	==================
Field name:	ramdisk_image
Type:		write (obligatory)
Offset/size:	0x218/4
Protocol:	2.00+
============	==================

  初始 ramdisk ramfs 32 位线性地址。如果没有初ramdisk/ramfs，则保持为零

============	==================
Field name:	ramdisk_size
Type:		write (obligatory)
Offset/size:	0x21c/4
Protocol:	2.00+
============	==================

  初始 ramdisk ramfs 的大小。如果没有初ramdisk/ramfs，则保持为零

============	===============
Field name:	bootsect_kludge
Type:		kernel internal
Offset/size:	0x220/4
Protocol:	2.00+
============	===============

  该字段已废弃

============	==================
Field name:	heap_end_ptr
Type:		write (obligatory)
Offset/size:	0x224/2
Protocol:	2.01+
============	==================

  将该字段设为 setup 堆末尾（从实模式代码起始处算起）的偏移量，减0x0200

============	================
Field name:	ext_loader_ver
Type:		write (optional)
Offset/size:	0x226/1
Protocol:	2.02+
============	================

  该字段用type_of_loader 字段中版本号的扩展。总版本号视为 (type_of_loader & 0x0f) + (ext_loader_ver << 4)

  该字段的使用取决于引导加载程序。如果未写入，则为零

  2.6.31 之前的内核不识别该字段，但对2.02 或更高版本的协议，写入它是安全的

============	=====================================================
Field name:	ext_loader_type
Type:		write (obligatory if (type_of_loader & 0xf0) == 0xe0)
Offset/size:	0x227/1
Protocol:	2.02+
============	=====================================================

  该字段用type_of_loader 字段中类型号的扩展。如type_of_loader 中的类型0xE，则实际类型(ext_loader_type + 0x10)

  如果 type_of_loader 中的类型不是 0xE，则忽略该字段

  2.6.31 之前的内核不识别该字段，但对2.02 或更高版本的协议，写入它是安全的

============	==================
Field name:	cmd_line_ptr
Type:		write (obligatory)
Offset/size:	0x228/4
Protocol:	2.02+
============	==================

  将该字段设为内核命令行的线性地址。内核命令行可以位于 setup 堆末尾到 0xA0000 之间的任意位置；它不必与实模式代码本身位于同一64K 段中。即使你的引导加载程序不支持命令行，也要填写该字段，此时可以指向一个空字符串（或者更好，指向字符"auto"。）如果该字段保持为零，内核将假定你的引导加载程序不支持 2.02+ 协议

============	===============
Field name:	initrd_addr_max
Type:		read
Offset/size:	0x22c/4
Protocol:	2.03+
============	===============

  初始 ramdisk/ramfs 内容可能占用的最大地址。对2.02 或更早的引导协议，不存在该字段，最大地址0x37FFFFFF。（该地址被定义为最高安全字节的地址，因此如果你ramdisk 恰好131072 字节长，且该字段0x37FFFFFF，则可以0x37FE0000 开始你ramdisk。）

============	============================
Field name:	kernel_alignment
Type:		read/modify (reloc)
Offset/size:	0x230/4
Protocol:	2.05+ (read), 2.10+ (modify)
============	============================

  内核所需的对齐单位（如果 relocatable_kernel 为真）。以与该字段值不兼容的对齐方式加载的可重定位内核，会在内核初始化期间被重新对齐

  从协议版2.10 开始，这反映了内核为获得最佳性能所偏好的对齐；加载器可以修改该字段以允许较小的对齐。请参见下文min_alignment pref_address 字段

============	==================
Field name:	relocatable_kernel
Type:		read (reloc)
Offset/size:	0x234/1
Protocol:	2.05+
============	==================

  如果该字段非零，则内核的保护模式部分可以加载到任何满kernel_alignment 字段的地址。加载后，引导加载程序必须将 code32_start 字段设置为指向已加载的代码，或指向一个引导加载程序钩子

============	=============
Field name:	min_alignment
Type:		read (reloc)
Offset/size:	0x235/1
Protocol:	2.10+
============	=============

  如果该字段非零，则以 2 的幂表示内核启动所需的（与偏好相对的）最小对齐。如果引导加载程序使用了该字段，则应更新如下

```
   kernel_alignment = 1 << min_alignment;

  过度未对齐的内核可能会带来相当大的性能代价。因此，加载器通常应尝试从 kernel_alignment 到该对齐之间的每一2 的幂对齐

```

============	==========
Field name:	xloadflags
Type:		read
Offset/size:	0x236/2
Protocol:	2.12+
============	==========

  该字段是一个位掩码

  Bit 0 (read):	XLF_KERNEL_64

 - 如果1，则内核0x200 处具有传统的 64 位入口点

  Bit 1 (read): XLF_CAN_BE_LOADED_ABOVE_4G

        - 如果1，则 kernel/boot_params/cmdline/ramdisk 可以位于 4G 以上

  Bit 2 (read):	XLF_EFI_HANDOVER_32

 - 如果1，则内核支持位于 handover_offset 32 EFI 切换入口点

  Bit 3 (read): XLF_EFI_HANDOVER_64

 - 如果1，则内核支持位于 handover_offset + 0x200 64 EFI 切换入口点

  Bit 4 (read): XLF_EFI_KEXEC

 - 如果1，则内核支持带有 EFI 运行时支持的 kexec EFI 引导


============	============
Field name:	cmdline_size
Type:		read
Offset/size:	0x238/4
Protocol:	2.06+
============	============

  命令行的最大大小（不含结尾的零）。这意味着命令行最多可包含 cmdline_size 个字符。对2.05 及更早的协议版本，最大大小为 255

============	====================================
Field name:	hardware_subarch
Type:		write (optional, defaults to x86/PC)
Offset/size:	0x23c/4
Protocol:	2.07+
============	====================================

  在半虚拟化环境中，中断处理、页表处理以及访问进程控制寄存器等底层硬件架构部分需要以不同方式完成。该字段允许引导加载程序告知内核我们正处于这些环境之一中

  ==========	==============================
  0x00000000	默认x86/PC 环境
  0x00000001	lguest
  0x00000002	Xen
  0x00000003	Intel MID (Moorestown, CloverTrail, Merrifield, Moorefield)
  0x00000004	CE4100 TV Platform
  ==========	==============================

============	=========================
Field name:	hardware_subarch_data
Type:		write (subarch-dependent)
Offset/size:	0x240/8
Protocol:	2.07+
============	=========================

  指向特定于硬件子架构的数据的指针。该字段在默认的 x86/PC 环境中目前未使用，请勿修改

============	==============
Field name:	payload_offset
Type:		read
Offset/size:	0x248/4
Protocol:	2.08+
============	==============

  如果非零，则该字段包含从保护模式代码起始处到有效载荷的偏移量

  有效载荷可能被压缩。压缩与未压缩数据的格式都应使用标准魔数来确定。当前支持的压缩格式gzip（魔1F 8B 1F 9E）、bzip2（魔42 5A）、LZMA（魔5D 00）、XZ（魔FD 37）、LZ4（魔02 21）和 ZSTD（魔28 B5）。未压缩的有效载荷目前始终是 ELF（魔7F 45 4C 46）

============	==============
Field name:	payload_length
Type:		read
Offset/size:	0x24c/4
Protocol:	2.08+
============	==============

  有效载荷的长度

============	===============
Field name:	setup_data
Type:		write (special)
Offset/size:	0x250/8
Protocol:	2.09+
============	===============

  指向NULL 结尾struct setup_data 单向链表64 位物理指针。这用于定义更具扩展性的引导参数传递机制。struct setup_data 的定义为

```
   struct setup_data {
	__u64 next;
	__u32 type;
	__u32 len;
	__u8 data[];
   }

```

  其中，next 是指向链表下一个节点的 64 位物理指针，最后一个节点的 next 字段0；type 用于标识 data 的内容；len data 字段的长度；data 保存真正的有效载荷

  该链表可能在引导过程的多个环节被修改。因此，修改该链表时，应始终考虑链表已经包含节点的情形

  setup_data 用于极大数据对象时有些不便，这既是因setup_data 头部必须与数据对象相邻，也是因为它只有一32 位长度字段。然而，引导过程的中间阶段需要有办法识别哪些内存块被内核数据占用，这一点很重要

  因此，协2.15 引入setup_indirect 结构体和 SETUP_INDIRECT 类型

```
   struct setup_indirect {
	__u32 type;
	__u32 reserved;		/* Reserved, must be set to zero. */
	__u64 len;
	__u64 addr;
   };

```

  type 成员SETUP_INDIRECT | SETUP_* 类型。但它不能是 SETUP_INDIRECT 自身，因为将 setup_indirect 做成树形结构可能会在需要解析它的地方消耗大量栈空间，而在引导上下文中栈空间可能有限

  下面举例说明如何使用 setup_indirect 指向 SETUP_E820_EXT 数据。此setup_data setup_indirect 将如下所示：

```
   struct setup_data {
	.next = 0,	/* or <addr_of_next_setup_data_struct> */
	.type = SETUP_INDIRECT,
	.len = sizeof(setup_indirect),
	.data[sizeof(setup_indirect)] = (struct setup_indirect) {
		.type = SETUP_INDIRECT | SETUP_E820_EXT,
		.reserved = 0,
		.len = <len_of_SETUP_E820_EXT_data>,
		.addr = <addr_of_SETUP_E820_EXT_data>,
	},
   }

```

     SETUP_INDIRECT | SETUP_NONE 对象无法SETUP_INDIRECT 本身明确区分。因此，引导加载程序不能提供此类对象

============	============
Field name:	pref_address
Type:		read (reloc)
Offset/size:	0x258/8
Protocol:	2.10+
============	============

  如果该字段非零，则表示内核偏好的加载地址。可重定位的引导加载程序应尽可能尝试在该地址加载

  不可重定位的内核将无条件地移动自身并在该地址运行。可重定位内核如果加载在该地址以下，则会将自身移动到该地址

============	=======
Field name:	init_size
Type:		read
Offset/size:	0x260/4
============	=======

  该字段指示从内核运行时起始地址开始、内核在能够检查其内存映射之前所需的线性连续内存大小。这与内核启动所需的总内存不是同一回事，但可重定位的引导加载程序可用它来帮助为内核选择一个安全的加载地址

```
   if (relocatable_kernel) {
	if (load_address < pref_address)
		load_address = pref_address;
	runtime_start = align_up(load_address, kernel_alignment);
   } else {
	runtime_start = pref_address;
   }

```

因此，所需内存窗口的位置和大小可通过以下方式估算

```
   memory_window_start = runtime_start;
   memory_window_size = init_size;

```

============	===============
Field name:	handover_offset
Type:		read
Offset/size:	0x264/4
============	===============

  该字段是从内核映像起始处EFI 交接协议入口点的偏移量。使EFI 交接协议引导内核的引导加载程序应跳转到该偏移量

  详见下文"EFI 交接协议"

============	==================
Field name:	kernel_info_offset
Type:		read
Offset/size:	0x268/4
Protocol:	2.15+
============	==================

  该字段是从内核映像起始处kernel_info 的偏移量。kernel_info 结构嵌入Linux 映像的未压缩保护模式区域中


## kernel_info


各头部之间的关系类似于各种数

```
  setup_header = .data
  boot_params/setup_data = .bss

```

```
  kernel_info = .rodata

```

长期以来，由于缺乏替代方案——尤其是在早期——以及惯性，我们一直在（滥用）.data 存放本可放入 .rodata .bss 的内容。此外，BIOS stub 负责创建 boot_params，因此它对于基于 BIOS 的加载器并不可用（不setup_data 可用）

setup_header 2 字节跳转字段的寻址范围（它同时充当结构的长度字段）以及 struct boot_params 中必须由保护模式加载器或 BIOS stub 将其复制进去空洞"大小，被永久限制144 字节。它目前119 字节，留给我们的只有非常宝贵25 个字节。若不完全修订引导协议、破坏向后兼容，这是无法修复的

boot_params 本身限于 4096 字节，但可以通过添加 setup_data 条目任意扩展。它不能用于传达内核映像的属性，因为它是 .bss 且没有映像提供的内容

kernel_info 通过为内核映像信息提供一个可扩展的位置来解决这一问题。它是只读的，因为内核不能依赖引导加载程序将其内容复制到任何地方，但没关系；如果确有必要，它仍然可以包含那些启用的引导加载程序应复制setup_data 块中的数据项

所kernel_info 数据都应是该结构的一部分。定长数据必须放kernel_info_var_len_data 标签之前。变长数据必须放kernel_info_var_len_data 标签之后。每个变长数据块都必

```
  kernel_info:
	.ascii  "LToP"		/* Header, Linux top (structure). */
	.long   kernel_info_var_len_data - kernel_info
	.long   kernel_info_end - kernel_info
	.long   0x01234567	/* Some fixed size data for the bootloaders. */
  kernel_info_var_len_data:
  example_struct:		/* Some variable size data for the bootloaders. */
	.ascii  "0123"		/* Header/Magic. */
	.long   example_struct_end - example_struct
	.ascii  "Struct"
	.long   0x89012345
  example_struct_end:
  example_strings:		/* Some variable size data for the bootloaders. */
	.ascii  "ABCD"		/* Header/Magic. */
	.long   example_strings_end - example_strings
	.asciz  "String_0"
	.asciz  "String_1"
  example_strings_end:
  kernel_info_end:

```

这样，kernel_info 就是一个自包含blob

     每个变长数据头部/魔数可以是任4 字符字符串（字符串末尾不\0），且不得与现有的变长数据头魔数冲突


## kernel_info 字段详解


============	========
Field name:	header
Offset/size:	0x0000/4
============	========

  包含魔数 "LToP"x506f544c）

============	========
Field name:	size
Offset/size:	0x0004/4
============	========

  该字段包kernel_info 的大小（kernel_info.header）。它不计kernel_info.kernel_info_var_len_data 的大小。引导加载程序应使用该字段来检kernel_info 中受支持的定长字段以kernel_info.kernel_info_var_len_data 的起始位置

============	========
Field name:	size_total
Offset/size:	0x0008/4
============	========

  该字段包kernel_info 的大小（kernel_info.header kernel_info.kernel_info_var_len_data）

============	==============
Field name:	setup_type_max
Offset/size:	0x000c/4
============	==============

  该字段包setup_data setup_indirect 结构体所允许的最大类型


## 内核命令


内核命令行已成为引导加载程序与内核通信的重要方式。其中一些选项也与引导加载程序本身相关，详见下特殊命令行选项"。内核命令行是一个以 NUL 结尾的字符串。最大长度可cmdline_size 字段获取。在 2.06 协议版本之前，最大长度为 255 个字符。过长的字符串会被内核自动截断

如果引导协议版本2.02 或更高，则内核命令行的地址由头字段 cmd_line_ptr 给出（见上文）。该地址可以位于 setup 堆末尾到 0xA0000 之间的任意位置

如果协议版本**不是** 2.02 或更高，则使用以下协议输入内核命令行

  - 在偏0x0020（字）处"cmd_line_magic" 中，填入魔数 0xA33F

  - 在偏0x0022（字）处"cmd_line_offset" 中，填入内核命令行的偏移量（相对于实模式内核起始处）

  - 内核命令*必须**位于 setup_move_size 所覆盖的内存区域内，因此你可能需要调整该字段


## 实模式代码的内存布局


实模式代码需要设置栈/堆，并分配用于内核命令行的内存。这需要在低兆字节中实模式可访问的内存中完成

需要注意的是，现代机器通常有一个相当大的扩BIOS 数据区（EBDA）。因此，建议尽可能少地使用低兆字节内存

遗憾的是，在以下情况下必须使0x90000 内存段：

 - 加载 zImage 内核时（(loadflags & 0x01) == 0）
 - 加载 2.01 或更早引导协议的内核时

     对于 2.00 2.01 引导协议，实模式代码可以加载到另一个地址，但会在内部重定位到 0x90000。对"old"（旧版）协议，实模式代码必须加载0x90000

0x90000 加载时，避免使用 0x9a000 以上的内存

对于 2.02 或更高版本的引导协议，命令行不必与实模式 setup 代码位于同一64K 段中；因此可以将整个 64K 段都给栈/堆，并将命令行放在它上面

内核命令行不应位于实模式代码下方，也不应位于高端内存中


## 引导配置示例


作为示例配置，假设实模式段具有以下布局

    当加载在 0x90000 以下时，使用整个段：

        =============	===================
	0x0000-0x7fff	Real mode kernel
	0x8000-0xdfff	Stack and heap
	0xe000-0xffff	Kernel command line
	=============	===================

    当加载在 0x90000 或协议版本为 2.01 或更早时

	=============	===================
	0x0000-0x7fff	Real mode kernel
	0x8000-0x97ff	Stack and heap
	0x9800-0x9fff	Kernel command line
	=============	===================

```
  unsigned long base_ptr;	/* base address for real-mode segment */

  if (setup_sects == 0)
	setup_sects = 4;

  if (protocol >= 0x0200) {
	type_of_loader = <type code>;
	if (loading_initrd) {
		ramdisk_image = <initrd_address>;
		ramdisk_size = <initrd_size>;
	}

	if (protocol >= 0x0202 && loadflags & 0x01)
		heap_end = 0xe000;
	else
		heap_end = 0x9800;

	if (protocol >= 0x0201) {
		heap_end_ptr = heap_end - 0x200;
		loadflags |= 0x80;		/* CAN_USE_HEAP */
	}

	if (protocol >= 0x0202) {
		cmd_line_ptr = base_ptr + heap_end;
		strcpy(cmd_line_ptr, cmdline);
	} else {
		cmd_line_magic	= 0xA33F;
		cmd_line_offset = heap_end;
		setup_move_size = heap_end + strlen(cmdline) + 1;
		strcpy(base_ptr + cmd_line_offset, cmdline);
	}
  } else {
	/* Very old kernel */

	heap_end = 0x9800;

	cmd_line_magic	= 0xA33F;
	cmd_line_offset = heap_end;

	/* A very old kernel MUST have its real-mode code loaded at 0x90000 */
	if (base_ptr != 0x90000) {
		/* Copy the real-mode kernel */
		memcpy(0x90000, base_ptr, (setup_sects + 1) * 512);
		base_ptr = 0x90000;		 /* Relocated */
	}

	strcpy(0x90000 + cmd_line_offset, cmdline);

	/* It is recommended to clear memory up to the 32K mark */
	memset(0x90000 + (setup_sects + 1) * 512, 0, (64 - (setup_sects + 1)) * 512);
  }


```

## 加载内核的其余部


32 位（非实模式）内核从内核文件中偏(setup_sects + 1) * 512 处开始（再次强调，如setup_sects == 0，真实值为 4）。对Image/zImage 内核，它应加载在地址 0x10000；对bzImage 内核，应加载0x100000。如果协>= 2.00 0x01

```
  is_bzImage = (protocol >= 0x0200) && (loadflags & 0x01);
  load_address = is_bzImage ? 0x100000 : 0x10000;

```

Image/zImage 内核最大可512K，因此会使用整个 0x10000-0x90000 内存范围。这意味着这些内核几乎必须将实模式部分加载0x90000。bzImage 内核则允许更大的灵活性

## 特殊命令行选项


如果引导加载程序提供的命令行由用户输入，用户可能期望以下命令行选项能正常工作。即使并非所有选项对内核都真正有意义，通常也不应从内核命令行中删除它们。需要为引导加载程序本身添加额外命令行选项的引导加载程序作者，应在 Documentation/admin-guide/kernel-parameters.rst 中注册它们，以确保它们现在或将来都不会与实际内核选项冲突

  vga=<mode>
	<mode> 可以是整数（采用 C 表示法，可为十进制、八进制或十六进制），也可以是字符串 "normal"（即 0xFFFF）ext"（即 0xFFFE）或 "ask"（即 0xFFFD）之一。该值应填入 vid_mode 字段，因为内核在解析命令行之前就会使用它

  mem=<size>
	<size> 是采C 表示法、后面可选择性地跟（大小写不敏感）K、M、G、T、P E（分别表<< 10< 20< 30< 40< 50 << 60）的整数。这向内核指定内存的末端。这会影initrd 可能的放置位置，因为 initrd 应放在内存末端附近。注意，这同时是内核***引导加载程序的一个选项

  initrd=<file>
	应加载一initrdfile> 的含义显然取决于引导加载程序，并且某些引导加载程序（例如 LILO）没有这样的命令

此外，一些引导加载程序会向用户指定的命令行添加以下选项

  BOOT_IMAGE=<file>
	被加载的引导映像。同样，<file> 的含义显然取决于引导加载程序

  auto
	内核在没有用户明确干预的情况下启动

如果这些选项由引导加载程序添加，强烈建议将它们放*最前面**，位于用户指定或配置指定的命令行之前。否则，"init=/bin/sh" 会被 "auto" 选项干扰


## 运行内核


内核通过跳转到内核入口点来启动，该入口点位于距实模式内核起始处的***偏移 0x20。这意味着如果你将实模式内核代码加载在 0x90000，内核入口点就是 9020:0000。进入时，ds = es = ss 应指向实模式内核代码的起始处（如果代码加载在 0x90000，则0x9000），sp 应正确设置，通常指向堆顶，并且中断应被禁用。此外，为防止内核中bug，建议引导加载程序设fs = gs = ds = es = ss

```
  /*
   * Note: in the case of the "old" kernel protocol, base_ptr must
   * be == 0x90000 at this point; see the previous sample code.
   */
  seg = base_ptr >> 4;

  cli();			/* Enter with interrupts disabled! */

  /* Set up the real-mode kernel stack */
  _SS = seg;
  _SP = heap_end;

  _DS = _ES = _FS = _GS = seg;
  jmp_far(seg + 0x20, 0);	/* Run the kernel */

```

如果你的引导扇区访问软盘驱动器，建议在内核运行之前关闭软盘马达，因为内核引导会使中断保持关闭，从而马达不会被关闭，特别是当被加载的内核将软盘驱动作为按需加载模块时！


## 高级引导加载程序钩子


如果引导加载程序运行在特别恶劣的环境中（例如运行DOS 下的 LOADLIN），可能无法遵循标准的内存位置要求。这样的引导加载程序可以使用以下钩子，它们如果被设置，会在适当时机由内核调用。使用这些钩子大概应被视为绝对的最后手段！重要：所有钩子在被调用时都必须保%espebpesi %edi

  realmode_swtch:
	A 16-bit real mode far subroutine invoked immediately before entering protected mode. The default routine disables NMI, so your routine should probably do so, too.

  code32_start:
	A 32-bit flat-mode routine **jumped** to immediately after the transition to protected mode, but before the kernel is uncompressed. No segments, except CS, are guaranteed to be set up (current kernels do, but older ones do not); you should set them up to BOOT_DS (0x18) yourself. After completing your hook, you should jump to the address that was in this field before your boot loader overwrote it (relocated, if appropriate.)


## 32 位引导协


对于使用某些非传BIOS 的新BIOS（如 EFI、LinuxBIOS 等）的机器以kexec，基于传BIOS 16 位实模式 setup 代码无法使用，因此需要定义一32 位引导协议

32 位引导协议中，加Linux 内核的第一步应该是设置引导参数（struct boot_params，传统上称为 "zero page"/零页）。struct boot_params 的内存应被分配并初始化为零。然后，应将内核映像中从偏移 0x01f1 开始的 setup 头加载到 struct boot_params 中并检查。setup 头的末尾可按以下方式计算

```
  0x0202 + byte value at offset 0x0201

```

除了16 位引导协议那样对 struct boot_params setup 头进行读/修改/写之外，引导加载程序还应按照 Documentation/arch/x86/zero-page.rst 一章的描述填写 struct boot_params 的附加字段。设置好 struct boot_params 后，引导加载程序可以16 位引导协议那样加32/64 位内核。在 32 位引导协议中，内核通过跳转32 位内核入口点来启动，该入口点就是已加载的 32/64 位内核的起始地址。进入时，CPU 必须处于禁用分页32 位保护模式；必须加载一GDT，其中包含选择__BOOT_CS(0x10) __BOOT_DS(0x18) 的描述符；两个描述符都必须是 4G 平面段；__BOOT_CS 必须具有执行/读权限，__BOOT_DS 必须具有写权限；CS 必须__BOOT_CS，DS、ES、SS 必须__BOOT_DS；中断必须被禁用esi 必须保存 struct boot_params 的基地址ebpedi %ebx 必须为零

## 64 位引导协


对于配备 64 CPU 64 位内核的机器，我们可以使64 位引导加载程序，并且需要一64 位引导协议

64 位引导协议中，加Linux 内核的第一步应该是设置引导参数（struct boot_params，传统上称为 "zero page"/零页）。struct boot_params 的内存可以分配在任意位置（甚4G 以上）并初始化为零。然后，应将内核映像中偏0x01f1 处的 setup 头加载到 struct boot_params 中并检查。setup 头的末尾

```
  0x0202 + byte value at offset 0x0201

```

除了16 位引导协议那样对 struct boot_params setup 头进行读/修改/写之外，引导加载程序还应按照 Documentation/arch/x86/zero-page.rst 一章的描述填写 struct boot_params 的附加字段。设置好 struct boot_params 后，引导加载程序可以16 位引导协议那样加64 位内核，但内核可以加载到 4G 以上。在 64 位引导协议中，内核通过跳转64 位内核入口点来启动，该入口点是已加载64 位内核的起始地址加上 0x200。进入时，CPU 必须处于启用分页64 位模式。从已加载内核的起始地址起、大小为 setup_header.init_size 的范围，以及零页和命令行缓冲区，都会获得一致性映射（ident mapping）；必须加载一GDT，其中包含选择__BOOT_CS(0x10) __BOOT_DS(0x18) 的描述符；两个描述符都必须是 4G 平面段；__BOOT_CS 必须具有执行/读权限，__BOOT_DS 必须具有写权限；CS 必须__BOOT_CS，DS、ES、SS 必须__BOOT_DS；中断必须被禁用rsi 必须保存 struct boot_params 的基地址

## EFI 交接协议（已弃用


该协议允许引导加载程序将初始化推迟到 EFI boot stub。引导加载程序需要从引导介质加载内核/initrd，并跳转EFI 交接协议入口点，该入口点距离 startup_{32,64} 起始hdr->handover_offset 字节。引导加载程序在处理节对齐、可执行映像超出文件本身大小的内存占用，以及可能影响该映像在 EFI 固件提供的执行上下文中作PE/COFF 二进制正确运行的 PE/COFF 头任何其他方面时，必须遵守内核的 PE/COFF 元数据

```
  void efi_stub_entry(void *handle, efi_system_table_t *table, struct boot_params *bp);

```

'handle' EFI 固件传递给引导加载程序EFI 映像句柄table' EFI 系统表——它们是 UEFI 规范2.3 节所描述交接状的前两个参数bp' 是引导加载程序分配的 boot params

```
  - hdr.cmd_line_ptr
  - hdr.ramdisk_image (if applicable)
  - hdr.ramdisk_size  (if applicable)

```

所有其他字段应为零。EFI 交接协议已弃用，取而代之的是下文描述的普PE/COFF 入口点


## PE/COFF 入口


当使`CONFIG_EFI_STUB=y` 编译时，内核可以作为普通的 PE/COFF 二进制执行。实现细节请参阅 Documentation/admin-guide/efi-stub.rst。stub 加载器可以通过 UEFI 协议请求 initrd。要使此功能工作，固件或引导加载程序需要注册一个句柄，该句柄携`EFI_LOAD_FILE2` 协议的实现，以及暴露 `LINUX_EFI_INITRD_MEDIA_GUID` 厂商媒体设备路径的设备路径协议。在这种情况下，通过 EFI stub 引导的内核将在已注册的协议上调用 **``LoadFile2``: LoadFile()** 方法，指示固件将 initrd 加载到内EFI stub 选择的内存位置。这种方式使EFI 引导加载程序无需了解 boot_params 的内部表示，也无需了解命令行与 ramdisk 在内存中放置位置、或内核映像本身放置位置的任何要限制。有关示例实现，请参`the original u-boot implementation`_ `the OVMF implementation`_