
## 在用户空间应用程序中使用 FS 与 GS 段


x86 架构支持分段。访问内存的指令可以使用基于段寄存器的寻址模式。下面使用如下记法来表示段内某个字节的地址：

  Segment-register:Byte-address

段基址会被加到 Byte-address 上，以计算出实际被访问的虚拟地址。这样就可以用相同的 Byte-address（即相同的代码）访问多份数据实例。具体访问哪个实例，完全取决于段寄存器中的基址。

在 32 位模式下，CPU 提供 6 个段，并且支持段限长。段限长可用于实施地址空间保护。

在 64 位模式下，CS/SS/DS/ES 段被忽略，其基址始终为 0，从而提供完整的 64 位地址空间。而 FS 与 GS 段在 64 位模式下仍然可用。

### FS 与 GS 的常见用法


FS 段通常用于访问线程本地存储（TLS）。FS 一般由运行时代码或线程库管理。使用 `__thread` 存储类说明符声明的变量会按线程分别实例化，编译器在访问这些变量时会生成 FS: 地址前缀。每个线程都有自己的 FS 基址，因此可以使用通用代码，而无需进行复杂的地址偏移计算来访问各线程的实例。当应用程序使用了管理每线程 FS 的运行时或线程库时，不应将 FS 用于其他目的。

GS 段没有通用用途，应用程序可自由使用。GCC 与 Clang 通过地址空间标识符支持基于 GS 的寻址。

### 读取与写入 FS/GS 基址


读取和写入 FS/GS 基址有两种机制：

 - arch_prctl() 系统调用

 - FSGSBASE 指令族

### 使用 arch_prctl() 访问 FS/GS 基址


 基于 arch_prctl(2) 的机制在所有 64 位 CPU 和所有内核版本上都可用。

 读取基址：

   arch_prctl(ARCH_GET_FS, &fsbase);
   arch_prctl(ARCH_GET_GS, &gsbase);

 写入基址：

   arch_prctl(ARCH_SET_FS, fsbase);
   arch_prctl(ARCH_SET_GS, gsbase);

 ARCH_SET_GS 这个 prctl 可能会根据内核配置和安全设置被禁用。

### 使用 FSGSBASE 指令访问 FS/GS 基址


 在 Ivy Bridge 这一代 CPU 上，Intel 引入了一组新指令，可直接从用户空间访问 FS 与 GS 基址寄存器。这些指令在 AMD Family 17H CPU 上也受支持。可用的指令如下：

  =============== ===========================
  RDFSBASE %reg   读取 FS 基址寄存器
  RDGSBASE %reg   读取 GS 基址寄存器
  WRFSBASE %reg   写入 FS 基址寄存器
  WRGSBASE %reg   写入 GS 基址寄存器
  =============== ===========================

 这些指令避免了 arch_prctl() 系统调用的开销，并允许在用户空间应用程序中更灵活地运用 FS/GS 寻址模式。但这并不能防止使用 FS 的线程库、运行时与希望将其用于自身目的的应用程序之间发生冲突。

##### FSGSBASE 指令的启用


 这些指令在 CPUID 叶 7 的 EBX 位 0 中枚举。若可用，/proc/cpuinfo 会在 CPU 的 flag 项中显示 'fsgsbase'。

 指令可用并不代表它们会自动被启用。内核必须在 CR4 中显式启用它们。原因是较旧的内核对 GS 寄存器中的值做了一些假设，并在通过 arch_prctl() 设置 GS 基址时强制执行这些假设。允许用户空间向 GS 基址写入任意值会违背这些假设并导致运行异常。

 在未启用 FSGSBASE 的内核上，执行 FSGSBASE 指令会因 #UD 异常而报错。

 内核在 ELF AUX 向量中提供关于启用状态的可靠信息。如果 AUX 向量中设置了 HWCAP2_FSGSBASE 位，说明内核已启用 FSGSBASE 指令，应用程序即可使用它们。
```
   #include <sys/auxv.h>
   #include <elf.h>

   /* 最终会进入 asm/hwcap.h */
   #ifndef HWCAP2_FSGSBASE
   #define HWCAP2_FSGSBASE        (1 << 1)
   #endif

   ....

   unsigned val = getauxval(AT_HWCAP2);

   if (val & HWCAP2_FSGSBASE)
        printf("FSGSBASE enabled\n");

```
##### FSGSBASE 指令的编译器支持


GCC 4.6.4 及更新版本为 FSGSBASE 指令提供了内建函数（intrinsics）。Clang 5 也支持它们。

  =================== ===========================
  _readfsbase_u64()   读取 FS 基址寄存器
  _readgsbase_u64()   读取 GS 基址寄存器
  _writefsbase_u64()  写入 FS 基址寄存器
  _writegsbase_u64()  写入 GS 基址寄存器
  =================== ===========================

要使用这些内建函数，必须在源代码中包含 <immintrin.h>，并添加编译器选项 -mfsgsbase。

### 编译器对基于 FS/GS 寻址的支持


GCC 6 及更新版本通过命名地址空间（Named Address Spaces）提供对基于 FS/GS 寻址的支持。GCC 为 x86 实现了以下地址空间标识符：

  ========= ====================================
  __seg_fs  变量相对于 FS 进行寻址
  __seg_gs  变量相对于 GS 进行寻址
  ========= ====================================

当支持这些地址空间时，会定义预处理宏 __SEG_FS 与 __SEG_GS。实现回退模式的代码应
```
  #ifdef __SEG_GS

  long data0 = 0;
  long data1 = 1;

  long __seg_gs *ptr;

  /* 检查内核是否启用了 FSGSBASE（HWCAP2_FSGSBASE） */
  ....

  /* 将 GS 基址指向 data0 */
  _writegsbase_u64(&data0);

  /* 访问 GS 的偏移 0 */
  ptr = 0;
  printf("data0 = %ld\n", *ptr);

  /* 将 GS 基址指向 data1 */
  _writegsbase_u64(&data1);
  /* ptr 仍然寻址偏移 0！ */
  printf("data1 = %ld\n", *ptr);


```
Clang 不提供 GCC 的地址空间标识符，但它在 Clang 2.6 及更新版本中通过基于属性的机制提供地址空间：

 ==================================== =====================================
  __attribute__((address_space(256))  变量相对于 GS 进行寻址
  __attribute__((address_space(257))  变量相对于 FS 进行寻址
 ==================================== =====================================

### 使用内联汇编进行基于 FS/GS 的寻址


如果编译器不支持地址空间，可以使用内联汇编
```
	mov %fs:offset, %reg
	mov %gs:offset, %reg

	mov %reg, %fs:offset
	mov %reg, %gs:offset

```
