
### BMIPS 设备树启

  某些引导加载程序仅支持内核映像起始处的单个入口点。其他引导加载程序会
  跳转ELF 起始地址。两种方案都受支持；CONFIG_BOOT_RAW=y   CONFIG_NO_EXCEPT_FILL=y，因此第一条指令会立即跳转kernel_entry()
  arch/arm 的情(b) 类似，支DT 的引导加载程序应当设置以下寄存器
         a0 : 0

         a1 : 0xffffffff

         a2 : RAM 中设备树块（在第二章中定义）的物理指针。设备树可以位于
         物理地址空间x00000000 - 0x1fffffff）前 512MB 内的任意位置         并按 64 位边界对齐
  传统引导加载程序不使用此约定，也不会传入 DT 块。在这种情况下，Linux   查找通过 CONFIG_DT_* 选择的内DTB
  此约定仅针对 32 位系统定义，因为目前没有任何 64 位的 BMIPS 实现