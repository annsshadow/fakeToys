## ARM Linux 上的内核初始化参

本文档描述了内核初始化参数结构，也称‘struct param_struct’，它用于大多数 ARM Linux 架构
该结构用于从内核加载器向 Linux 内核本身传递初始化参数，并可能仅在内核初始化过程中短暂存在作为一般规则，不应arch/arm/kernel/setup.c:setup_arch() 之外引用它
其中列出了许多参数，下面进行描述
 page_size
   此参数必须设置为机器的页大小，并将由内核检查
 nr_pages
   这是系统中内存的总页数。如果内存是bank 的，则它应包含系统中总的页数
   如果系统包含独立VRAM，此值不应包含该信息
 ramdisk_size
   这现已过时，不应使用
 flags
   各种内核标志，包括：

    =====   ========================
    bit 0   1 = 以只读方式挂载根文件系统
    bit 1   未使    bit 2   0 = 加载 ramdisk
    bit 3   0 = 提示加载 ramdisk
    =====   ========================

 rootdev
   要挂载为根文件系统的设备major/minor 号对
 video_num_cols / video_num_rows
   这两个一起描述虚拟控制台VGA 控制台的字符大小。它们不应用于任何其他目的
   通常最好将它们设置为标VGA，或你的 fbcon 显示的等效字符大小。这样所有启动消息都   正确显示
 video_x / video_y
   这描VGA 控制台上光标的字符位置，此外未使用。（不应用于其他控制台类型，也不应用   其他目的）
 memc_control_reg
   用于基于 Acorn Archimedes Acorn A5000 机器MEMC 芯片控制寄存器。不同的架构可能   不同方式使用它
 sounddefault
   Acorn 机器上的默认声音设置。不同的架构可能以不同方式使用它
 adfsdrives
   ADFS/MFM 磁盘的数量。不同的架构可能以不同方式使用它
 bytes_per_char_h / bytes_per_char_v
   这些现已过时，不应使用
 pages_in_bank[^4^]
   系统内存每个 bank 中的页数（用RiscPC）。这适用于物理内存在处理器看来是非连续性的系统
 pages_in_vram
   VRAM 中的页数（用Acorn RiscPC）。如果无法从硬件获取视频 RAM 的大小，加载器也可以
   使用此值
 initrd_start / initrd_size
   这描述初ramdisk 的内核虚拟起始地址和大小
 rd_start
   软盘ramdisk 映像的起始地址（以扇区为单位）
 system_rev
   系统修订号
 system_serial_low / system_serial_high
   系统 64 位序列号

 mem_fclk_21285
   连接21285（footbridge）的外部振荡器的速度，它控制内存总线、定时器和串口的速度   根据 CPU 的速度，其值可以在 0-66 MHz 之间。如果没有传递参数或传递了零值，则在 21285
   架构上默认值为 50 MHz
 paths[^8^][^128^]
   这些现已过时，不应使用
 commandline
   内核命令行参数。详细信息见别处